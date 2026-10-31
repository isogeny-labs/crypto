//! Local and S3 custody for breadth-first walk checkpoints.

use std::fs::{self, File, OpenOptions};
use std::io::Write;
#[cfg(unix)]
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::Instant;

use crypto_lib::cryptanalysis::isogeny_walk::{checkpoint, walk::Walk};
use serde_json::{json, Value};

pub(super) struct WalkState {
    dir: PathBuf,
    remote: Option<String>,
    _lock: File,
    fingerprint: String,
    checkpoint_every: u64,
    sync_every: u64,
    last_checkpoint: Instant,
    last_sync: Instant,
    verbose: bool,
}

fn aws(args: &[&str]) -> Result<Output, String> {
    let output = Command::new("aws")
        .args(args)
        .output()
        .map_err(|e| format!("starting AWS CLI: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "AWS CLI failed ({}): {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(output)
}

fn has_remote_objects(uri: &str) -> Result<bool, String> {
    let (bucket, prefix) = uri
        .strip_prefix("s3://")
        .and_then(|s| s.split_once('/'))
        .ok_or("--state-s3-uri must be s3://BUCKET/PREFIX")?;
    let prefix = format!("{prefix}/");
    let output = aws(&[
        "s3api",
        "list-objects-v2",
        "--bucket",
        bucket,
        "--prefix",
        &prefix,
        "--max-keys",
        "1",
        "--output",
        "json",
    ])?;
    let listing: Value =
        serde_json::from_slice(&output.stdout).map_err(|e| format!("invalid S3 listing: {e}"))?;
    Ok(listing["KeyCount"].as_u64().unwrap_or(0) > 0)
}

fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let temp = path.with_extension("tmp");
    if temp.exists() || temp.is_symlink() {
        fs::remove_file(&temp).map_err(|e| format!("removing {}: {e}", temp.display()))?;
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp)
        .map_err(|e| format!("creating {}: {e}", temp.display()))?;
    file.write_all(bytes)
        .and_then(|_| file.sync_all())
        .map_err(|e| format!("writing {}: {e}", temp.display()))?;
    fs::rename(&temp, path).map_err(|e| format!("publishing {}: {e}", path.display()))
}

fn has_payload(dir: &Path) -> Result<bool, String> {
    for entry in fs::read_dir(dir).map_err(|e| format!("reading {}: {e}", dir.display()))? {
        let entry = entry.map_err(|e| format!("reading {}: {e}", dir.display()))?;
        if entry.file_name() != ".walk.lock" {
            return Ok(true);
        }
    }
    Ok(false)
}

impl WalkState {
    pub(super) fn prepare(
        dir: PathBuf,
        resume: bool,
        remote: Option<String>,
        checkpoint_every: u64,
        sync_every: u64,
        verbose: bool,
        manifest: Value,
    ) -> Result<Self, String> {
        if let Some(uri) = &remote {
            let Some((bucket, prefix)) = uri.strip_prefix("s3://").and_then(|s| s.split_once('/'))
            else {
                return Err("--state-s3-uri must be s3://BUCKET/PREFIX".into());
            };
            if bucket.is_empty() || prefix.trim_matches('/').is_empty() || uri.ends_with('/') {
                return Err("--state-s3-uri must name a nonempty bucket and prefix without a trailing slash".into());
            }
        }
        fs::create_dir_all(&dir).map_err(|e| format!("creating {}: {e}", dir.display()))?;
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(dir.join(".walk.lock"))
            .map_err(|e| format!("opening walk state lock: {e}"))?;
        #[cfg(unix)]
        if unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
            return Err(format!("{} is already in use", dir.display()));
        }
        #[cfg(not(unix))]
        return Err("walk state directory locking is unavailable on this platform".into());
        let manifest_path = dir.join("manifest.json");
        if resume {
            if !manifest_path.exists() {
                if has_payload(&dir)? {
                    return Err("walk state directory has no manifest but is not empty".into());
                }
                let uri = remote.as_ref().ok_or("no local walk manifest to resume")?;
                if verbose {
                    eprintln!("isogeny_walk: restoring S3 state");
                }
                aws(&[
                    "s3",
                    "sync",
                    &format!("{uri}/"),
                    &dir.to_string_lossy(),
                    "--exclude",
                    ".walk.lock",
                    "--only-show-errors",
                ])?;
            } else if let Some(uri) = &remote {
                if has_remote_objects(uri)? {
                    let output = aws(&[
                        "s3",
                        "cp",
                        &format!("{uri}/manifest.json"),
                        "-",
                        "--only-show-errors",
                    ])?;
                    let found: Value = serde_json::from_slice(&output.stdout)
                        .map_err(|e| format!("invalid S3 walk manifest: {e}"))?;
                    if found != manifest {
                        return Err("S3 walk manifest differs from the local run".into());
                    }
                }
            }
            let found: Value = serde_json::from_slice(
                &fs::read(&manifest_path).map_err(|e| format!("reading walk manifest: {e}"))?,
            )
            .map_err(|e| format!("invalid walk manifest: {e}"))?;
            if found != manifest {
                return Err("walk manifest differs from the requested run".into());
            }
            if !dir.join("walk.ckpt").is_file() {
                return Err("walk resume needs walk.ckpt".into());
            }
        } else {
            if has_payload(&dir)? {
                return Err(format!("{} is not empty; use --resume", dir.display()));
            }
            if let Some(uri) = &remote {
                if has_remote_objects(uri)? {
                    return Err("S3 prefix is not empty; use a unique prefix or --resume".into());
                }
            }
            write_atomic(
                &manifest_path,
                &serde_json::to_vec(&manifest).map_err(|e| e.to_string())?,
            )?;
        }
        let fingerprint = blake3::hash(&serde_json::to_vec(&manifest).map_err(|e| e.to_string())?)
            .to_hex()
            .to_string();
        Ok(Self {
            dir,
            remote,
            _lock: lock,
            fingerprint,
            checkpoint_every: checkpoint_every.min(sync_every),
            sync_every,
            last_checkpoint: Instant::now(),
            last_sync: Instant::now(),
            verbose,
        })
    }

    pub(super) fn restore(&self, walk: &mut Walk) -> Result<(), String> {
        checkpoint::load(&self.dir.join("walk.ckpt"), &self.fingerprint, walk)
    }

    pub(super) fn progress(&mut self, walk: &Walk, final_batch: bool) -> Result<(), String> {
        let expanded = walk.nodes.iter().filter(|n| n.expanded).count();
        if self.verbose {
            eprintln!("isogeny_walk: {} curves, {expanded} expanded, {} verified edges, {} failures; {} ms",
                walk.nodes.len(), walk.edges.len(), walk.failures.len(), walk.stats.walk_ms);
        }
        let status = json!({"stage":"walk", "curves":walk.nodes.len(), "expanded":expanded,
            "edges":walk.edges.len(), "failures":walk.failures.len(), "walk_ms":walk.stats.walk_ms});
        write_atomic(
            &self.dir.join("progress.json"),
            &serde_json::to_vec(&status).map_err(|e| e.to_string())?,
        )?;
        if final_batch
            || self.checkpoint_every == 0
            || self.last_checkpoint.elapsed().as_secs() >= self.checkpoint_every
        {
            checkpoint::save(&self.dir.join("walk.ckpt"), &self.fingerprint, walk)?;
            self.last_checkpoint = Instant::now();
        }
        if self.remote.is_some() && self.last_sync.elapsed().as_secs() >= self.sync_every {
            self.sync(false)?;
        }
        Ok(())
    }

    fn sync(&mut self, final_upload: bool) -> Result<(), String> {
        let Some(uri) = &self.remote else {
            return Ok(());
        };
        if self.verbose {
            eprintln!("isogeny_walk: S3 checkpoint upload started");
        }
        let push = |name: &str| -> Result<(), String> {
            let path = self.dir.join(name);
            if path.is_file() {
                aws(&[
                    "s3",
                    "cp",
                    &path.to_string_lossy(),
                    &format!("{uri}/{name}"),
                    "--only-show-errors",
                ])?;
            }
            Ok(())
        };
        for name in ["manifest.json", "walk.ckpt", "progress.json"] {
            push(name)?;
        }
        if final_upload {
            for name in [
                "curves.yaml",
                "isogeny_routes.json",
                "STORE.json",
                "walk.json",
            ] {
                push(name)?;
            }
        }
        self.last_sync = Instant::now();
        if self.verbose {
            eprintln!("isogeny_walk: S3 upload complete");
        }
        Ok(())
    }

    pub(super) fn finish(&mut self) -> Result<(), String> {
        self.sync(true)
    }
}
