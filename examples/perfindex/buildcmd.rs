//! `build`: a perfbench binary for a revision, with this tree's harness
//! overlaid, in a sparse worktree.  The directories the sparse checkout needs
//! come from the revision's own manifests, not from a list that goes stale
//! every time a workspace crate is added.

use crate::compare::{repo_root, run_capture, tool_provenance};
use crate::host::host_manifest;
use crate::util::{sha256_file, sha256_hex, write_json};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(clap::Args, Clone, Debug)]
pub struct BuildArgs {
    /// Build this revision (with this tree's harness overlaid).
    #[arg(long, conflicts_with = "current")]
    pub reference: Option<String>,
    /// Build the working tree.
    #[arg(long)]
    pub current: bool,
    #[arg(long)]
    pub out: PathBuf,
    /// Keep the worktree and target directory.
    #[arg(long)]
    pub keep: bool,
    /// Extra arguments for `cargo build`.
    #[arg(last = true)]
    pub cargo_args: Vec<String>,
}

/// The Cargo crate holding the harness: the root (crypto) or `suite/` (cryptanalysis).
pub fn crate_dir(root: &Path) -> Result<&'static str, String> {
    if root.join("Cargo.toml").exists() {
        Ok(".")
    } else if root.join("suite/Cargo.toml").exists() {
        Ok("suite")
    } else {
        Err("no Cargo crate at the repository root or in suite/".into())
    }
}

/// Directories a sparse build worktree checks out (cone mode keeps root
/// files); empty for a full checkout.  crypto's research tree is gigabytes
/// and its build needs none of it; the suite includes files from across its
/// small repository, so it takes all.
pub fn sparse_dirs(krate: &str) -> Vec<String> {
    if krate == "." {
        ["src", "examples", "benches", "tests", "scripts", "docs"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    } else {
        Vec::new()
    }
}

/// Whether a `[section]` header names a table of dependencies, and if the
/// header is itself one dependency (`[dependencies.foo]`) rather than the list.
fn dependency_table(header: &str) -> Option<bool> {
    let h = header.trim();
    const KINDS: [&str; 3] = ["dependencies", "dev-dependencies", "build-dependencies"];
    let h = h.strip_prefix("target.").map_or(h, |rest| {
        // target.<cfg or triple>.<kind>...: the cfg may be quoted and contain dots
        KINDS
            .iter()
            .filter_map(|k| rest.rfind(&format!(".{k}")).map(|i| &rest[i + 1..]))
            .next()
            .unwrap_or(h)
    });
    let h = h.strip_prefix("workspace.").unwrap_or(h);
    for k in KINDS {
        if h == k {
            return Some(false);
        }
        if h.strip_prefix(k).is_some_and(|r| r.starts_with('.')) {
            return Some(true);
        }
    }
    None
}

/// Every `path = "…"` of a dependency in a manifest: inline tables
/// (`foo = { path = "x" }`) and dependency subtables (`[dependencies.foo]`),
/// in `[dependencies]`, `[dev-dependencies]`, `[build-dependencies]`,
/// `[target.*.…]` and `[workspace.dependencies]`.  Not `[[bin]]` or
/// `[[example]]` source paths: those are not needed to build another target.
pub fn manifest_path_deps(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut table: Option<bool> = None; // Some(is_subtable) inside a dependency table
    for raw in text.lines() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        if let Some(h) = line.strip_prefix('[') {
            table = if h.starts_with('[') {
                None
            } else {
                dependency_table(h.trim_end_matches(']'))
            };
            continue;
        }
        let Some(is_sub) = table else { continue };
        let rest = if is_sub {
            line.strip_prefix("path")
                .map(str::trim_start)
                .filter(|r| r.starts_with('='))
        } else {
            // `name = { …, path = "x", … }`
            line.split_once('=').and_then(|(_, v)| {
                let v = v.trim();
                v.starts_with('{').then(|| find_inline_path(v)).flatten()
            })
        };
        if is_sub {
            if let Some(r) = rest {
                if let Some(p) = quoted(r.trim_start_matches('=').trim()) {
                    out.push(p);
                }
            }
        } else if let Some(p) = rest.and_then(|r| quoted(r.trim())) {
            out.push(p);
        }
    }
    out
}

/// In an inline table, the text after `path =`.
fn find_inline_path(v: &str) -> Option<&str> {
    let mut from = 0;
    while let Some(i) = v[from..].find("path") {
        let at = from + i;
        let before = v[..at].chars().last();
        let after = v[at + 4..].trim_start();
        if matches!(before, Some('{' | ',' | ' ' | '\t')) && after.starts_with('=') {
            return Some(after[1..].trim_start());
        }
        from = at + 4;
    }
    None
}

fn quoted(s: &str) -> Option<String> {
    let q = s.chars().next().filter(|c| *c == '"' || *c == '\'')?;
    let end = s[1..].find(q)?;
    Some(s[1..1 + end].to_string())
}

fn manifest_text(root: &Path, rev: Option<&str>, rel: &str) -> Option<String> {
    match rev {
        None => std::fs::read_to_string(root.join(rel)).ok(),
        Some(r) => {
            let o = Command::new("git")
                .arg("-C")
                .arg(root)
                .args(["show", &format!("{r}:{rel}")])
                .output()
                .ok()?;
            o.status
                .success()
                .then(|| String::from_utf8_lossy(&o.stdout).into_owned())
        }
    }
}

/// `a/b/../c` as `a/c`; `None` if it climbs out of the root.
fn normalise(path: &str) -> Option<String> {
    let mut parts: Vec<&str> = Vec::new();
    for p in path.split('/') {
        match p {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            x => parts.push(x),
        }
    }
    Some(parts.join("/"))
}

/// Directories (repo-relative) of the crate's path dependencies, followed
/// recursively, as of `rev` (the working tree when `None`).
pub fn path_dependency_dirs(root: &Path, rev: Option<&str>, krate: &str) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut todo =
        vec![normalise(&format!("{krate}/Cargo.toml")).unwrap_or_else(|| "Cargo.toml".into())];
    while let Some(rel) = todo.pop() {
        if !seen.insert(rel.clone()) {
            continue;
        }
        let Some(text) = manifest_text(root, rev, &rel) else {
            continue;
        };
        let parent = rel.rsplit_once('/').map_or("", |(p, _)| p);
        for p in manifest_path_deps(&text) {
            let joined = if parent.is_empty() {
                p
            } else {
                format!("{parent}/{p}")
            };
            let Some(d) = normalise(&joined) else {
                continue;
            };
            if d.is_empty() || found.contains(&d) {
                continue;
            }
            todo.push(format!("{d}/Cargo.toml"));
            found.push(d);
        }
    }
    found
}

/// SHA-256 over the relative paths and contents of every file under `dir`, in
/// path order: the identity of a harness source tree.
pub fn sha256_tree(dir: &Path) -> Result<String, String> {
    fn walk(base: &Path, dir: &Path, out: &mut Vec<Vec<String>>) -> std::io::Result<()> {
        for e in std::fs::read_dir(dir)? {
            let p = e?.path();
            if p.is_dir() {
                walk(base, &p, out)?;
            } else if p.is_file() {
                out.push(
                    p.strip_prefix(base)
                        .unwrap()
                        .components()
                        .map(|c| c.as_os_str().to_string_lossy().into_owned())
                        .collect(),
                );
            }
        }
        Ok(())
    }
    let mut files = Vec::new();
    walk(dir, dir, &mut files).map_err(|e| format!("{}: {e}", dir.display()))?;
    files.sort();
    let mut buf: Vec<u8> = Vec::new();
    for parts in files {
        let rel = parts.join("/");
        buf.extend_from_slice(rel.as_bytes());
        buf.push(0);
        buf.extend(std::fs::read(dir.join(&rel)).map_err(|e| e.to_string())?);
        buf.push(0);
    }
    Ok(sha256_hex(&buf))
}

fn git(root: &Path, args: &[&str]) -> Result<String, String> {
    let mut c = Command::new("git");
    c.arg("-C").arg(root).args(args);
    let (out, err, ok) = run_capture(c, None)?;
    if ok {
        Ok(out.trim().to_string())
    } else {
        Err(format!("git {args:?} failed: {err}"))
    }
}

pub fn run_build(a: &BuildArgs) -> Result<i32, String> {
    let root = repo_root().ok_or("not in a git repository")?;
    let out = std::path::absolute(&a.out).map_err(|e| e.to_string())?;
    if out.exists() {
        return Err(format!("{} exists", out.display()));
    }
    std::fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    let krate = crate_dir(&root)?;
    let harness_rel = Path::new(krate).join("examples/perfbench");
    let harness_src = root.join(&harness_rel);
    let target = out.join("target");
    let mut meta = json!({
        "harness_sha256": sha256_tree(&harness_src)?,
        "host": host_manifest(true),
        "tool": tool_provenance(),
        "cargo_args": a.cargo_args,
    });
    let src: PathBuf;
    if a.current {
        src = root.clone();
        meta["rev"] = json!(git(&root, &["rev-parse", "HEAD"])?);
        meta["dirty"] =
            json!(!git(&root, &["status", "--porcelain", "--", "src", "Cargo.toml"])?.is_empty());
    } else {
        let reference = a
            .reference
            .as_deref()
            .ok_or("give --reference REV or --current")?;
        src = out.join("src");
        git(
            &root,
            &[
                "worktree",
                "add",
                "--no-checkout",
                "--detach",
                &src.display().to_string(),
                reference,
            ],
        )?;
        let dirs = sparse_dirs(krate);
        if dirs.is_empty() {
            git(&src, &["sparse-checkout", "disable"])?;
        } else {
            let mut all = dirs.clone();
            for d in path_dependency_dirs(&root, Some(reference), krate) {
                if !all.contains(&d) {
                    all.push(d);
                }
            }
            let mut args = vec!["sparse-checkout", "set"];
            args.extend(all.iter().map(String::as_str));
            git(&src, &args)?;
        }
        git(&src, &["checkout", "--detach", reference])?;
        meta["rev"] = json!(git(&src, &["rev-parse", "HEAD"])?);
        meta["dirty"] = json!(false);
        // Overlay this tree's harness so both sides run identical kernels.
        let dst = src.join(&harness_rel);
        if dst.exists() {
            std::fs::remove_dir_all(&dst).map_err(|e| e.to_string())?;
        }
        copy_dir(&harness_src, &dst)?;
    }
    let lock = src.join(krate).join("Cargo.lock");
    if lock.exists() {
        meta["cargo_lock_sha256"] = json!(sha256_file(&lock).map_err(|e| e.to_string())?);
    }
    let t0 = std::time::Instant::now();
    let status = Command::new("cargo")
        .args(["build", "--release", "--example", "perfbench"])
        .args(&a.cargo_args)
        .current_dir(src.join(krate))
        .env("CARGO_TARGET_DIR", &target)
        .status()
        .map_err(|e| e.to_string())?;
    if !status.success() {
        return Err(format!("cargo build failed ({status})"));
    }
    meta["build_seconds"] = json!((t0.elapsed().as_secs_f64() * 10.0).round() / 10.0);
    let bin = out.join("perfbench");
    std::fs::copy(target.join("release/examples/perfbench"), &bin).map_err(|e| e.to_string())?;
    meta["binary_sha256"] = json!(sha256_file(&bin).map_err(|e| e.to_string())?);
    write_json(&out.join("build.json"), &meta)?;
    if !a.keep {
        let _ = std::fs::remove_dir_all(&target);
        if !a.current {
            let _ = git(
                &root,
                &["worktree", "remove", "--force", &src.display().to_string()],
            );
        }
    }
    println!("{}", bin.display());
    let _ = Value::Null;
    Ok(0)
}

fn copy_dir(from: &Path, to: &Path) -> Result<(), String> {
    std::fs::create_dir_all(to).map_err(|e| e.to_string())?;
    for e in std::fs::read_dir(from).map_err(|e| e.to_string())? {
        let e = e.map_err(|e| e.to_string())?;
        let (p, q) = (e.path(), to.join(e.file_name()));
        if p.is_dir() {
            copy_dir(&p, &q)?;
        } else {
            std::fs::copy(&p, &q).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const MANIFEST: &str = r#"
[package]
name = "root"

[dependencies]
cli = { path = "tools/cli" }           # inline table
serde = "1"
rand = { version = "0.8", features = ["small_rng"] }
iso = { version = "0.1", path = "isogeny_algos", optional = true }

[dependencies.sub]
version = "1"
path = "crates/sub"

[dev-dependencies]
dev = { path = "dev/dev" }

[target.'cfg(unix)'.dependencies]
unix = { path = "unix_only" }

[target.x86_64-pc-windows-msvc.build-dependencies]
win = { path = "win" }

[workspace.dependencies]
shared = { path = "shared" }

[[bin]]
name = "x"
path = "experiments/x.rs"

[[example]]
name = "y"
path = "experiments/y.rs"

[features]
default = []
"#;

    #[test]
    fn finds_dependency_paths_only() {
        let mut got = manifest_path_deps(MANIFEST);
        got.sort();
        assert_eq!(
            got,
            vec![
                "crates/sub",
                "dev/dev",
                "isogeny_algos",
                "shared",
                "tools/cli",
                "unix_only",
                "win"
            ],
            "bin and example source paths must not be dependencies"
        );
    }

    #[test]
    fn a_key_named_like_path_is_not_a_path() {
        assert!(manifest_path_deps(
            "[dependencies]\nfoo = { version = \"1\", features = [\"mypath\"] }\nbar = \"path\"\n"
        )
        .is_empty());
    }

    #[test]
    fn dependency_tables_are_recognised() {
        assert_eq!(dependency_table("dependencies"), Some(false));
        assert_eq!(dependency_table("dependencies.foo"), Some(true));
        assert_eq!(
            dependency_table("target.'cfg(unix)'.dev-dependencies"),
            Some(false)
        );
        assert_eq!(
            dependency_table("target.'cfg(any(unix, windows))'.dependencies.foo"),
            Some(true)
        );
        assert_eq!(dependency_table("workspace.dependencies"), Some(false));
        assert_eq!(dependency_table("features"), None);
        assert_eq!(dependency_table("package"), None);
        assert_eq!(dependency_table("profile.release"), None);
    }

    #[test]
    fn normalise_resolves_dots_and_refuses_escape() {
        assert_eq!(normalise("a/b/../c").as_deref(), Some("a/c"));
        assert_eq!(normalise("./a//b").as_deref(), Some("a/b"));
        assert_eq!(normalise("../x"), None);
        assert_eq!(normalise("a/../../x"), None);
    }

    fn init_repo(tag: &str) -> PathBuf {
        let dir = crate::util::tests_support::tempdir(tag);
        let run = |args: &[&str]| {
            let st = Command::new("git")
                .arg("-C")
                .arg(&dir)
                .args(args)
                .env("GIT_AUTHOR_NAME", "t")
                .env("GIT_AUTHOR_EMAIL", "t@t")
                .env("GIT_COMMITTER_NAME", "t")
                .env("GIT_COMMITTER_EMAIL", "t@t")
                .output()
                .unwrap();
            assert!(
                st.status.success(),
                "{args:?}: {}",
                String::from_utf8_lossy(&st.stderr)
            );
        };
        run(&["init", "-q"]);
        let w = |rel: &str, text: &str| {
            let p = dir.join(rel);
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(p, text).unwrap();
        };
        w("Cargo.toml", "[package]\nname=\"root\"\n[dependencies]\ncli = { path = \"tools/cli\" }\niso = { path = \"isogeny\" }\n[[bin]]\nname=\"x\"\npath=\"experiments/x.rs\"\n");
        w("tools/cli/Cargo.toml", "[package]\nname=\"cli\"\n");
        w("isogeny/Cargo.toml", "[package]\nname=\"iso\"\n[dependencies]\ninner = { path = \"inner\" }\nescape = { path = \"../../outside\" }\nsibling = { path = \"../tools/cli\" }\n");
        w("isogeny/inner/Cargo.toml", "[package]\nname=\"inner\"\n");
        run(&["add", "."]);
        run(&["commit", "-q", "-m", "one"]);
        w("newcrate/Cargo.toml", "[package]\nname=\"n\"\n");
        w(
            "Cargo.toml",
            &(std::fs::read_to_string(dir.join("Cargo.toml")).unwrap()
                + "[build-dependencies]\nn = { path = \"newcrate\" }\n"),
        );
        run(&["add", "."]);
        run(&["commit", "-q", "-m", "two"]);
        dir
    }

    #[test]
    fn follows_recursively_ignores_escapes_and_respects_the_revision() {
        let repo = init_repo("deps");
        let mut now = path_dependency_dirs(&repo, Some("HEAD"), ".");
        now.sort();
        assert_eq!(
            now,
            vec!["isogeny", "isogeny/inner", "newcrate", "tools/cli"]
        ); // sibling resolves, the escape does not
        let mut before = path_dependency_dirs(&repo, Some("HEAD~1"), ".");
        before.sort();
        assert_eq!(before, vec!["isogeny", "isogeny/inner", "tools/cli"]); // a later crate is not seen by an older revision
        let mut tree = path_dependency_dirs(&repo, None, ".");
        tree.sort();
        assert_eq!(tree, now);
    }

    #[test]
    fn the_real_manifest_of_this_repository() {
        if let Some(root) = repo_root() {
            if crate_dir(&root) == Ok(".") && root.join("tools/volcano-cli").exists() {
                assert!(path_dependency_dirs(&root, None, ".")
                    .contains(&"tools/volcano-cli".to_string()));
            }
        }
    }

    #[test]
    fn harness_hash_depends_on_names_and_contents() {
        let d = crate::util::tests_support::tempdir("tree");
        std::fs::write(d.join("a.rs"), "x").unwrap();
        let h1 = sha256_tree(&d).unwrap();
        std::fs::write(d.join("a.rs"), "y").unwrap();
        assert_ne!(h1, sha256_tree(&d).unwrap());
        std::fs::write(d.join("a.rs"), "x").unwrap();
        assert_eq!(h1, sha256_tree(&d).unwrap());
        std::fs::rename(d.join("a.rs"), d.join("b.rs")).unwrap();
        assert_ne!(h1, sha256_tree(&d).unwrap());
    }
}
