//! `vault`: transparent git encryption for research findings.
//! See tools/vault/README.md for the design, threat model and workflows.

mod filter;
mod format;
mod guard;
mod keys;

use anyhow::{bail, ensure, Context, Result};
use clap::{Parser, Subcommand};
use filter::Keyring;
use keys::Repo;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(
    name = "vault",
    version,
    about = "Transparent git encryption: AES-SIV blobs, data keys wrapped by AWS KMS"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Create .vault/vault.json.
    Init {
        #[arg(long)]
        repo: String,
        #[arg(long)]
        kms_key: String,
        #[arg(long, default_value = "us-west-2")]
        region: String,
        /// Glob that must always be committed encrypted (repeatable).
        #[arg(long)]
        protect: Vec<String>,
    },
    /// Generate a realm's next-epoch data key with KMS and commit its wrapped form.
    NewKey {
        #[arg(long)]
        realm: String,
    },
    /// Unwrap realm keys with KMS (or import break-glass key files), install
    /// the filters and hooks, and decrypt the working tree.
    Unlock {
        /// Realms to unlock; default all.
        #[arg(long)]
        realm: Vec<String>,
        /// Import `REALM:EPOCH:PATH` (hex key file) instead of calling KMS.
        #[arg(long)]
        key_file: Vec<String>,
        /// Do not install git hooks.
        #[arg(long)]
        no_hooks: bool,
    },
    /// Forget cached keys and re-encrypt the working tree.
    Lock {
        #[arg(long)]
        force: bool,
    },
    /// Configure the git filters, textconv and (unless --no-hooks) hooks.
    Install {
        #[arg(long)]
        no_hooks: bool,
    },
    /// Show realms, epochs, cached keys and installation state.
    Status,
    /// Single-shot clean filter (stdin -> stdout).
    Clean {
        #[arg(long)]
        realm: String,
        #[arg(long)]
        path: String,
    },
    /// Single-shot smudge filter (stdin -> stdout).
    Smudge {
        #[arg(long)]
        path: String,
    },
    /// Long-running git filter process (gitprotocol-long-running-process v2).
    FilterProcess {
        #[arg(long)]
        realm: String,
    },
    /// Decrypt a file to stdout if possible (diff textconv).
    Cat { file: PathBuf },
    /// Refuse plaintext in protected paths: staged, in a commit range, or pre-push.
    Guard {
        #[arg(long)]
        staged: bool,
        /// rev-list arguments, e.g. `origin/main..HEAD` or `HEAD` (all history).
        #[arg(long)]
        range: Option<String>,
        /// Read git's pre-push stdin.
        #[arg(long)]
        pre_push: bool,
        /// Also authenticate and check the embedded path when the key is cached.
        #[arg(long)]
        verify: bool,
    },
    /// Encrypt a file outside git (e.g. a CI artifact). The label plays the role of the path.
    Seal {
        #[arg(long)]
        realm: String,
        #[arg(long)]
        label: String,
        input: PathBuf,
        output: PathBuf,
    },
    /// Decrypt a sealed file; with --label, require that label.
    Unseal {
        #[arg(long)]
        label: Option<String>,
        input: PathBuf,
        output: PathBuf,
    },
    /// Print a cached key in hex for offline escrow. Treat the output as the key.
    ExportKey {
        #[arg(long)]
        realm: String,
        #[arg(long)]
        epoch: u32,
    },
}

fn main() {
    if let Err(e) = run(Cli::parse()) {
        eprintln!("{e:#}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    match cli.cmd {
        Cmd::Init {
            repo,
            kms_key,
            region,
            protect,
        } => {
            let r = Repo::discover()?;
            ensure!(
                !r.config_path().exists(),
                "vault: {} already exists",
                keys::CONFIG_REL
            );
            ensure!(
                repo.split('/').count() == 2,
                "vault: --repo must be owner/name"
            );
            r.save_config(&keys::Config {
                version: 1,
                repo,
                kms_key_id: kms_key,
                region,
                realms: Default::default(),
                protect,
            })?;
            println!(
                "wrote {}; next: vault new-key --realm <realm>",
                keys::CONFIG_REL
            );
        }
        Cmd::NewKey { realm } => {
            format::validate_realm(&realm)?;
            let r = Repo::discover()?;
            let mut cfg = r.load_config()?;
            let epoch = cfg.realms.get(&realm).map_or(1, |e| e + 1);
            let wp = r.wrapped_path(&realm, epoch);
            ensure!(!wp.exists(), "vault: {} already exists", wp.display());
            let (key, wrapped) = keys::kms_generate(&cfg, &realm, epoch)?;
            keys::write_wrapped(&wp, &wrapped)?;
            r.store_cached_key(&realm, epoch, &key)?;
            cfg.realms.insert(realm.clone(), epoch);
            r.save_config(&cfg)?;
            install(&r, &cfg, true)?;
            println!(
                "realm {realm} epoch {epoch}: wrapped key at {}",
                wp.strip_prefix(&r.root)?.display()
            );
            if epoch > 1 {
                println!("re-encrypt under the new epoch with: git add --renormalize -- .");
            }
        }
        Cmd::Unlock {
            realm,
            key_file,
            no_hooks,
        } => {
            let r = Repo::discover()?;
            let cfg = r.load_config()?;
            for spec in &key_file {
                let mut it = spec.splitn(3, ':');
                let (Some(rl), Some(ep), Some(p)) = (it.next(), it.next(), it.next()) else {
                    bail!("vault: --key-file wants REALM:EPOCH:PATH");
                };
                format::validate_realm(rl)?;
                r.store_cached_key(rl, ep.parse()?, &keys::read_key_file(p.as_ref())?)?;
            }
            if key_file.is_empty() {
                let realms: Vec<String> = if realm.is_empty() {
                    cfg.realms.keys().cloned().collect()
                } else {
                    realm
                };
                for rl in &realms {
                    ensure!(cfg.realms.contains_key(rl), "vault: unknown realm {rl}");
                    for ep in r.wrapped_epochs(rl)? {
                        if r.cached_key(rl, ep).is_some() {
                            continue;
                        }
                        let k = keys::kms_unwrap(
                            &cfg,
                            rl,
                            ep,
                            &keys::read_wrapped(&r.wrapped_path(rl, ep))?,
                        )
                        .with_context(|| format!("vault: unwrapping {rl} epoch {ep}"))?;
                        r.store_cached_key(rl, ep, &k)?;
                        eprintln!("vault: unlocked {rl} epoch {ep}");
                    }
                }
            }
            install(&r, &cfg, !no_hooks)?;
            let n = refresh(&r, format::is_ciphertext)?;
            eprintln!("vault: decrypted {n} file(s) in the working tree");
        }
        Cmd::Lock { force } => {
            let r = Repo::discover()?;
            let paths = vault_paths(&r)?;
            if !force && !paths.is_empty() {
                let mut a = vec!["status", "--porcelain", "-z", "--"];
                a.extend(paths.iter().map(String::as_str));
                let st = keys::git_out(&a)?;
                ensure!(
                    st.is_empty(),
                    "vault: encrypted paths have uncommitted changes; commit them or pass --force"
                );
            }
            if r.cache_dir().exists() {
                std::fs::remove_dir_all(r.cache_dir())?;
            }
            let n = refresh(&r, |data| !format::is_ciphertext(data))?;
            eprintln!("vault: locked; {n} file(s) back to ciphertext");
        }
        Cmd::Install { no_hooks } => {
            let r = Repo::discover()?;
            install(&r, &r.load_config()?, !no_hooks)?;
        }
        Cmd::Status => status()?,
        Cmd::Clean { realm, path } => {
            let mut kr = Keyring::open()?;
            let out = filter::clean(&mut kr, &realm, &path, &filter::read_stdin()?)?;
            std::io::stdout().write_all(&out)?;
        }
        Cmd::Smudge { path } => {
            let mut kr = Keyring::open()?;
            let (filter::Smudged::Plain(v) | filter::Smudged::Locked(v)) =
                filter::smudge(&mut kr, &path, filter::read_stdin()?);
            std::io::stdout().write_all(&v)?;
        }
        Cmd::FilterProcess { realm } => filter::filter_process(&realm)?,
        Cmd::Cat { file } => {
            let data = std::fs::read(&file)?;
            let out = match Keyring::open() {
                Ok(mut kr) => filter::textconv(&mut kr, &data),
                Err(_) => data,
            };
            std::io::stdout().write_all(&out)?;
        }
        Cmd::Guard {
            staged,
            range,
            pre_push,
            verify,
        } => guard_cmd(staged, range, pre_push, verify)?,
        Cmd::Seal {
            realm,
            label,
            input,
            output,
        } => {
            let mut kr = Keyring::open()?;
            let data = std::fs::read(&input)?;
            ensure!(
                !format::is_ciphertext(&data),
                "vault: {} is already sealed",
                input.display()
            );
            let out = filter::clean(&mut kr, &realm, &label, &data)?;
            std::fs::write(&output, out)?;
        }
        Cmd::Unseal {
            label,
            input,
            output,
        } => {
            let mut kr = Keyring::open()?;
            let data = std::fs::read(&input)?;
            let (_, content) = filter::try_open(&mut kr, label.as_deref(), &data)?
                .context("vault: no key for this blob's realm/epoch (run vault unlock)")?;
            std::fs::write(&output, content)?;
        }
        Cmd::ExportKey { realm, epoch } => {
            let r = Repo::discover()?;
            let k = r
                .cached_key(&realm, epoch)
                .context("vault: key not cached (run vault unlock)")?;
            println!("{}", keys::key_hex(&k).as_str());
        }
    }
    Ok(())
}

fn exe() -> Result<String> {
    let p = std::env::current_exe()?.canonicalize()?;
    let s = p.to_str().context("vault: executable path is not UTF-8")?;
    ensure!(!s.contains('\''), "vault: executable path contains a quote");
    Ok(format!("'{s}'"))
}

fn git_config(key: &str, value: &str) -> Result<()> {
    let st = Command::new("git")
        .args(["config", "--local", key, value])
        .status()?;
    ensure!(st.success(), "vault: git config {key} failed");
    Ok(())
}

const HOOK_MARK: &str = "# installed by vault";

fn install(r: &Repo, cfg: &keys::Config, hooks: bool) -> Result<()> {
    let exe = exe()?;
    for realm in cfg.realms.keys() {
        let d = format!("filter.vault-{realm}");
        git_config(
            &format!("{d}.process"),
            &format!("{exe} filter-process --realm {realm}"),
        )?;
        git_config(
            &format!("{d}.clean"),
            &format!("{exe} clean --realm {realm} --path %f"),
        )?;
        git_config(&format!("{d}.smudge"), &format!("{exe} smudge --path %f"))?;
        git_config(&format!("{d}.required"), "true")?;
    }
    git_config("diff.vault.textconv", &format!("{exe} cat"))?;
    if hooks {
        let dir = PathBuf::from(
            keys::git_out(&["rev-parse", "--path-format=absolute", "--git-path", "hooks"])?.trim(),
        );
        std::fs::create_dir_all(&dir)?;
        for (name, args) in [
            ("pre-commit", "guard --staged --verify"),
            ("pre-push", "guard --pre-push --verify"),
        ] {
            let p = dir.join(name);
            if let Ok(existing) = std::fs::read_to_string(&p) {
                if !existing.contains(HOOK_MARK) {
                    eprintln!(
                        "vault: {} exists and is not ours; add `{exe} {args}` to it",
                        p.display()
                    );
                    continue;
                }
            }
            std::fs::write(&p, format!("#!/bin/sh\n{HOOK_MARK}\nexec {exe} {args}\n"))?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o755))?;
            }
        }
    }
    let _ = r;
    Ok(())
}

/// Tracked paths whose filter attribute is a vault realm.
fn vault_paths(r: &Repo) -> Result<Vec<String>> {
    let out = Command::new("git")
        .current_dir(&r.root)
        .args(["ls-files", "-z"])
        .output()?;
    let files: Vec<String> = out
        .stdout
        .split(|&c| c == 0)
        .filter(|s| !s.is_empty())
        .map(|s| String::from_utf8_lossy(s).into_owned())
        .collect();
    if files.is_empty() {
        return Ok(vec![]);
    }
    let mut child = Command::new("git")
        .current_dir(&r.root)
        .args(["check-attr", "-z", "--stdin", "filter"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let mut input = vec![];
    for f in &files {
        input.extend_from_slice(f.as_bytes());
        input.push(0);
    }
    let mut stdin = child.stdin.take().unwrap();
    let w = std::thread::spawn(move || stdin.write_all(&input));
    let o = child.wait_with_output()?;
    w.join().unwrap()?;
    let f: Vec<&[u8]> = o.stdout.split(|&c| c == 0).collect();
    Ok(files
        .into_iter()
        .enumerate()
        .filter(|(i, _)| f.get(3 * i + 2).is_some_and(|v| v.starts_with(b"vault-")))
        .map(|(_, p)| p)
        .collect())
}

/// Re-checks-out every vault path whose working copy satisfies `pick`.
fn refresh(r: &Repo, pick: impl Fn(&[u8]) -> bool) -> Result<usize> {
    let mut chosen = vec![];
    for p in vault_paths(r)? {
        let full = r.root.join(&p);
        if let Ok(data) = std::fs::read(&full) {
            if pick(&data) {
                std::fs::remove_file(&full)?;
                chosen.push(p);
            }
        }
    }
    if chosen.is_empty() {
        return Ok(0);
    }
    let mut child = Command::new("git")
        .current_dir(&r.root)
        .args([
            "checkout",
            "--pathspec-from-file=-",
            "--pathspec-file-nul",
            "--",
        ])
        .stdin(Stdio::piped())
        .spawn()?;
    {
        let mut stdin = child.stdin.take().unwrap();
        for p in &chosen {
            stdin.write_all(p.as_bytes())?;
            stdin.write_all(&[0])?;
        }
    }
    ensure!(child.wait()?.success(), "vault: git checkout failed");
    Ok(chosen.len())
}

fn guard_cmd(staged: bool, range: Option<String>, pre_push: bool, verify: bool) -> Result<()> {
    let r = Repo::discover()?;
    if !r.config_path().exists() {
        return Ok(()); // vault not adopted in this repository
    }
    let mut kr = Keyring::open()?;
    let protect = guard::protect_set(&kr.cfg.protect.clone())?;
    let mut rep = guard::Report {
        checked: 0,
        unverified: 0,
        violations: vec![],
    };
    let mut ranges: Vec<Vec<String>> = vec![];
    if pre_push {
        ranges.extend(guard::pre_push_ranges(&guard::read_all_stdin()?));
    }
    if let Some(rg) = range {
        ranges.push(rg.split_whitespace().map(str::to_string).collect());
    }
    ensure!(
        staged || !ranges.is_empty() || pre_push,
        "vault: guard needs --staged, --range or --pre-push"
    );
    if staged {
        let e = guard::staged_entries()?;
        guard::check(
            &mut kr,
            &protect,
            &e,
            &guard::AttrSource::Index,
            verify,
            &mut rep,
        )?;
    }
    for rg in ranges {
        let args: Vec<&str> = rg.iter().map(String::as_str).collect();
        for c in guard::rev_list(&args)? {
            let e = guard::commit_entries(&c)?;
            guard::check(
                &mut kr,
                &protect,
                &e,
                &guard::AttrSource::Commit(c.clone()),
                verify,
                &mut rep,
            )?;
        }
    }
    if !rep.violations.is_empty() {
        for v in &rep.violations {
            eprintln!("vault guard: {v}");
        }
        bail!(
            "vault guard: {} violation(s) in {} protected blob(s); nothing was pushed or committed",
            rep.violations.len(),
            rep.checked
        );
    }
    eprintln!(
        "vault guard: {} protected blob(s) encrypted{}",
        rep.checked,
        if rep.unverified > 0 {
            format!(", {} not authenticated (key not cached)", rep.unverified)
        } else {
            String::new()
        }
    );
    Ok(())
}

fn status() -> Result<()> {
    let r = Repo::discover()?;
    let cfg = r.load_config()?;
    println!("repo {}  kms {} ({})", cfg.repo, cfg.kms_key_id, cfg.region);
    for (realm, cur) in &cfg.realms {
        let eps = r.wrapped_epochs(realm)?;
        let cached: Vec<String> = eps
            .iter()
            .filter(|e| r.cached_key(realm, **e).is_some())
            .map(u32::to_string)
            .collect();
        let installed = Command::new("git")
            .args(["config", "--get", &format!("filter.vault-{realm}.process")])
            .output()?
            .status
            .success();
        println!(
            "realm {realm}: epoch {cur}; wrapped {:?}; unlocked [{}]; filter {}",
            eps,
            cached.join(","),
            if installed {
                "installed"
            } else {
                "NOT installed"
            }
        );
    }
    println!("protect {:?}", cfg.protect);
    Ok(())
}
