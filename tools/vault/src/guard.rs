//! `vault guard`: refuse plaintext where a vault blob belongs.
//!
//! A path is protected when its `filter` attribute (as of the commit being
//! checked) is `vault-<realm>`, or when it matches a `protect` glob in
//! `.vault/vault.json`. The second rule exists because the most likely leak is
//! not a broken filter but a missing one: a clone that never ran
//! `vault install`, or a commit that drops a line from `.gitattributes`.
//!
//! Every protected blob must carry the vault header. With `--verify` and the
//! key at hand it must also authenticate and name the path it sits at.

use crate::filter::{try_open, Keyring};
use crate::format;
use anyhow::{bail, Context, Result};
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Command, Stdio};

pub struct Entry {
    pub path: String,
    /// `git cat-file` object spec, e.g. `:path` or `<commit>:path`.
    pub spec: String,
}

pub enum AttrSource {
    Index,
    Commit(String),
}

pub struct Report {
    pub checked: usize,
    pub unverified: usize,
    pub violations: Vec<String>,
}

fn git(args: &[&str]) -> Result<Vec<u8>> {
    let o = Command::new("git")
        .args(args)
        .stderr(Stdio::inherit())
        .output()?;
    if !o.status.success() {
        bail!("vault: git {} failed", args.join(" "));
    }
    Ok(o.stdout)
}

fn split_nul(b: &[u8]) -> Vec<String> {
    b.split(|&c| c == 0)
        .filter(|s| !s.is_empty())
        .map(|s| String::from_utf8_lossy(s).into_owned())
        .collect()
}

pub fn staged_entries() -> Result<Vec<Entry>> {
    let out = git(&[
        "diff",
        "--cached",
        "--name-only",
        "-z",
        "--no-renames",
        "--diff-filter=ACMRT",
    ])?;
    Ok(split_nul(&out)
        .into_iter()
        .map(|p| Entry {
            spec: format!(":{p}"),
            path: p,
        })
        .collect())
}

pub fn commit_entries(commit: &str) -> Result<Vec<Entry>> {
    let out = git(&[
        "diff-tree",
        "-r",
        "-z",
        "--name-only",
        "--no-commit-id",
        "--root",
        "-m",
        "--first-parent",
        "--no-renames",
        "--diff-filter=ACMRT",
        commit,
    ])?;
    let mut paths = split_nul(&out);
    paths.sort();
    paths.dedup();
    Ok(paths
        .into_iter()
        .map(|p| Entry {
            spec: format!("{commit}:{p}"),
            path: p,
        })
        .collect())
}

pub fn rev_list(args: &[&str]) -> Result<Vec<String>> {
    let mut a = vec!["rev-list"];
    a.extend_from_slice(args);
    Ok(String::from_utf8(git(&a)?)?
        .lines()
        .map(str::to_string)
        .collect())
}

/// `filter` attribute of each path, as of `src`.
fn filter_attrs(paths: &[&str], src: &AttrSource) -> Result<Vec<Option<String>>> {
    if paths.is_empty() {
        return Ok(vec![]);
    }
    let mut cmd = Command::new("git");
    cmd.arg("check-attr").arg("-z");
    match src {
        AttrSource::Index => {
            cmd.arg("--cached");
        }
        AttrSource::Commit(c) => {
            cmd.arg(format!("--source={c}"));
        }
    }
    cmd.args(["--stdin", "filter"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped());
    let mut child = cmd.spawn()?;
    let mut input = vec![];
    for p in paths {
        input.extend_from_slice(p.as_bytes());
        input.push(0);
    }
    let mut stdin = child.stdin.take().unwrap();
    let writer = std::thread::spawn(move || stdin.write_all(&input));
    let o = child.wait_with_output()?;
    writer.join().unwrap()?;
    if !o.status.success() {
        bail!("vault: git check-attr failed");
    }
    // path NUL attr NUL value NUL, in input order.
    let f: Vec<&[u8]> = o.stdout.split(|&c| c == 0).collect();
    let mut out = Vec::with_capacity(paths.len());
    for i in 0..paths.len() {
        let v = String::from_utf8_lossy(f.get(3 * i + 2).copied().unwrap_or(b"")).into_owned();
        out.push(match v.as_str() {
            "unspecified" | "unset" | "set" | "" => None,
            _ => Some(v),
        });
    }
    Ok(out)
}

struct Batch {
    child: std::process::Child,
    out: BufReader<std::process::ChildStdout>,
}

impl Batch {
    fn new() -> Result<Self> {
        let mut child = Command::new("git")
            .args(["cat-file", "--batch"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let out = BufReader::new(child.stdout.take().unwrap());
        Ok(Batch { child, out })
    }
    fn get(&mut self, spec: &str) -> Result<Option<Vec<u8>>> {
        let stdin = self.child.stdin.as_mut().unwrap();
        writeln!(stdin, "{spec}")?;
        stdin.flush()?;
        let mut line = String::new();
        self.out.read_line(&mut line)?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            return Ok(None); // "<spec> missing"
        }
        let size: usize = parts[2].parse().context("vault: cat-file size")?;
        let mut buf = vec![0u8; size + 1];
        self.out.read_exact(&mut buf)?;
        buf.truncate(size);
        Ok(if parts[1] == "blob" { Some(buf) } else { None })
    }
}

pub fn protect_set(globs: &[String]) -> Result<GlobSet> {
    let mut b = GlobSetBuilder::new();
    for g in globs {
        b.add(Glob::new(g).with_context(|| format!("vault: bad protect glob {g:?}"))?);
    }
    Ok(b.build()?)
}

pub fn check(
    kr: &mut Keyring,
    protect: &GlobSet,
    entries: &[Entry],
    src: &AttrSource,
    verify: bool,
    report: &mut Report,
) -> Result<()> {
    let paths: Vec<&str> = entries.iter().map(|e| e.path.as_str()).collect();
    let attrs = filter_attrs(&paths, src)?;
    let mut batch = Batch::new()?;
    let at = match src {
        AttrSource::Index => "index".to_string(),
        AttrSource::Commit(c) => c.chars().take(12).collect(),
    };
    for (e, attr) in entries.iter().zip(attrs) {
        let attr_realm = attr
            .as_deref()
            .and_then(|a| a.strip_prefix("vault-"))
            .map(str::to_string);
        if attr_realm.is_none() && !protect.is_match(&e.path) {
            continue;
        }
        if e.spec.contains('\n') {
            report
                .violations
                .push(format!("{at}: {:?}: newline in path", e.path));
            continue;
        }
        let Some(blob) = batch.get(&e.spec)? else {
            continue;
        };
        report.checked += 1;
        if !format::is_ciphertext(&blob) {
            report.violations.push(format!(
                "{at}: {} is PLAINTEXT ({} bytes){}",
                e.path,
                blob.len(),
                if attr_realm.is_none() {
                    " and has no vault filter attribute"
                } else {
                    ""
                }
            ));
            continue;
        }
        let header = match format::parse(&blob) {
            Ok((h, _)) => h,
            Err(err) => {
                report.violations.push(format!("{at}: {}: {err:#}", e.path));
                continue;
            }
        };
        if let Some(r) = &attr_realm {
            if *r != header.realm {
                report.violations.push(format!(
                    "{at}: {} is in realm {} but its attribute says {r} (re-stage it)",
                    e.path, header.realm
                ));
            }
        }
        if verify {
            match try_open(kr, Some(&e.path), &blob) {
                Ok(Some(_)) => {}
                Ok(None) => report.unverified += 1,
                Err(err) => report.violations.push(format!(
                    "{at}: {}: {err:#}; run `git add -- {}`",
                    e.path, e.path
                )),
            }
        }
    }
    let _ = batch.child.kill();
    Ok(())
}

/// Ranges to check for a pre-push hook, from git's stdin protocol.
pub fn pre_push_ranges(input: &str) -> Vec<Vec<String>> {
    let zero = |s: &str| s.bytes().all(|b| b == b'0');
    let mut out = vec![];
    for line in input.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() != 4 || zero(f[1]) {
            continue; // deletion
        }
        if zero(f[3]) {
            out.push(vec![
                f[1].to_string(),
                "--not".to_string(),
                "--remotes".to_string(),
            ]);
        } else {
            out.push(vec![format!("{}..{}", f[3], f[1])]);
        }
    }
    out
}

pub fn read_all_stdin() -> Result<String> {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s)?;
    Ok(s)
}

#[cfg(test)]
mod tests {
    #[test]
    fn pre_push_parsing() {
        let z = "0".repeat(40);
        let a = "a".repeat(40);
        let b = "b".repeat(40);
        let input = format!("refs/heads/x {a} refs/heads/x {b}\nrefs/heads/n {a} refs/heads/n {z}\nrefs/heads/d {z} refs/heads/d {b}\n");
        let r = super::pre_push_ranges(&input);
        assert_eq!(
            r,
            vec![
                vec![format!("{b}..{a}")],
                vec![a.clone(), "--not".into(), "--remotes".into()]
            ]
        );
    }
}
