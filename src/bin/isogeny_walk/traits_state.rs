//! Checked local and S3 custody for completed trait-shard curve records.

use std::fs::{self, File, OpenOptions};
use std::io::Write;
#[cfg(unix)]
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::Instant;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

const CHECKPOINT: &str = "traits.ckpt";

#[derive(Serialize, Deserialize)]
struct Snapshot {
    schema_version: u32,
    fingerprint: String,
    lines: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Envelope {
    checksum: String,
    payload: String,
}

pub(super) struct TraitsState {
    dir: PathBuf,
    remote: Option<String>,
    _lock: File,
    fingerprint: String,
    lines: Vec<String>,
    sync_every: u64,
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

fn remote_has_objects(uri: &str) -> Result<bool, String> {
    let (bucket, prefix) = uri
        .strip_prefix("s3://")
        .and_then(|s| s.split_once('/'))
        .ok_or("--state-s3-uri must be s3://BUCKET/PREFIX")?;
    let output = aws(&[
        "s3api",
        "list-objects-v2",
        "--bucket",
        bucket,
        "--prefix",
        &format!("{prefix}/"),
        "--max-keys",
        "1",
        "--output",
        "json",
    ])?;
    let listing: Value =
        serde_json::from_slice(&output.stdout).map_err(|e| format!("invalid S3 listing: {e}"))?;
    Ok(listing["KeyCount"].as_u64().unwrap_or(0) > 0)
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), String> {
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
        if entry.file_name() != ".traits.lock" {
            return Ok(true);
        }
    }
    Ok(false)
}

fn decode(bytes: &[u8], fingerprint: &str) -> Result<Vec<String>, String> {
    let envelope: Envelope =
        serde_json::from_slice(bytes).map_err(|e| format!("invalid trait checkpoint: {e}"))?;
    if envelope.checksum != blake3::hash(envelope.payload.as_bytes()).to_hex().as_str() {
        return Err("trait checkpoint checksum mismatch".into());
    }
    let snapshot: Snapshot = serde_json::from_str(&envelope.payload)
        .map_err(|e| format!("invalid trait checkpoint payload: {e}"))?;
    if snapshot.schema_version != 1 || snapshot.fingerprint != fingerprint {
        return Err("trait checkpoint parameters or executable differ".into());
    }
    Ok(snapshot.lines)
}

impl TraitsState {
    pub(super) fn prepare(
        dir: PathBuf,
        resume: bool,
        remote: Option<String>,
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
                return Err(
                    "--state-s3-uri needs a nonempty bucket and prefix without a trailing slash"
                        .into(),
                );
            }
        }
        fs::create_dir_all(&dir).map_err(|e| format!("creating {}: {e}", dir.display()))?;
        let lock = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(dir.join(".traits.lock"))
            .map_err(|e| format!("opening trait lock: {e}"))?;
        #[cfg(unix)]
        if unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
            return Err(format!("{} is already in use", dir.display()));
        }
        #[cfg(not(unix))]
        return Err("trait state directory locking is unavailable on this platform".into());

        let manifest_path = dir.join("manifest.json");
        if resume {
            if !manifest_path.exists() {
                if has_payload(&dir)? {
                    return Err("trait state has no manifest but is not empty".into());
                }
                let uri = remote.as_ref().ok_or("no local trait manifest to resume")?;
                if verbose {
                    eprintln!("isogeny_walk: restoring trait state from S3");
                }
                aws(&[
                    "s3",
                    "sync",
                    &format!("{uri}/"),
                    &dir.to_string_lossy(),
                    "--exclude",
                    ".traits.lock",
                    "--only-show-errors",
                ])?;
            } else if let Some(uri) = &remote {
                if remote_has_objects(uri)? {
                    let output = aws(&[
                        "s3",
                        "cp",
                        &format!("{uri}/manifest.json"),
                        "-",
                        "--only-show-errors",
                    ])?;
                    let found: Value = serde_json::from_slice(&output.stdout)
                        .map_err(|e| format!("invalid S3 trait manifest: {e}"))?;
                    if found != manifest {
                        return Err("S3 trait manifest differs from the local run".into());
                    }
                }
            }
            let found: Value = serde_json::from_slice(
                &fs::read(&manifest_path).map_err(|e| format!("reading trait manifest: {e}"))?,
            )
            .map_err(|e| format!("invalid trait manifest: {e}"))?;
            if found != manifest {
                return Err("trait manifest differs from the requested run".into());
            }
        } else {
            if has_payload(&dir)? {
                return Err(format!("{} is not empty; use --resume", dir.display()));
            }
            if let Some(uri) = &remote {
                if remote_has_objects(uri)? {
                    return Err(
                        "S3 trait prefix is not empty; use a unique prefix or --resume".into(),
                    );
                }
            }
            atomic_write(
                &manifest_path,
                &serde_json::to_vec(&manifest).map_err(|e| e.to_string())?,
            )?;
        }
        let fingerprint = blake3::hash(&serde_json::to_vec(&manifest).map_err(|e| e.to_string())?)
            .to_hex()
            .to_string();
        let lines = if resume {
            let local_bytes = fs::read(dir.join(CHECKPOINT))
                .map_err(|e| format!("reading trait checkpoint: {e}"))?;
            let local = decode(&local_bytes, &fingerprint)?;
            if let Some(uri) = &remote {
                if remote_has_objects(uri)? {
                    let remote_bytes = aws(&[
                        "s3",
                        "cp",
                        &format!("{uri}/{CHECKPOINT}"),
                        "-",
                        "--only-show-errors",
                    ])?
                    .stdout;
                    let saved = decode(&remote_bytes, &fingerprint)?;
                    if saved.len() > local.len() {
                        return Err("S3 trait checkpoint is ahead of local state; restore into a fresh directory".into());
                    }
                    if saved.len() == local.len() && remote_bytes != local_bytes {
                        return Err(
                            "S3 and local trait checkpoints diverge at the same curve".into()
                        );
                    }
                }
            }
            local
        } else {
            Vec::new()
        };
        let mut state = Self {
            dir,
            remote,
            _lock: lock,
            fingerprint,
            lines,
            sync_every,
            last_sync: Instant::now(),
            verbose,
        };
        if !resume {
            state.save()?;
            state.sync(false)?;
        }
        Ok(state)
    }

    pub(super) fn lines(&self) -> &[String] {
        &self.lines
    }

    fn save(&self) -> Result<(), String> {
        let payload = serde_json::to_string(&Snapshot {
            schema_version: 1,
            fingerprint: self.fingerprint.clone(),
            lines: self.lines.clone(),
        })
        .map_err(|e| format!("serializing trait checkpoint: {e}"))?;
        let envelope = Envelope {
            checksum: blake3::hash(payload.as_bytes()).to_hex().to_string(),
            payload,
        };
        atomic_write(
            &self.dir.join(CHECKPOINT),
            &serde_json::to_vec(&envelope).map_err(|e| e.to_string())?,
        )?;
        atomic_write(
            &self.dir.join("progress.json"),
            &serde_json::to_vec(&json!({
                "stage":"traits", "completed_curves":self.lines.len(),
            }))
            .map_err(|e| e.to_string())?,
        )
    }

    pub(super) fn record(&mut self, line: &str) -> Result<(), String> {
        self.lines.push(line.to_string());
        self.save()?;
        if self.remote.is_some() && self.last_sync.elapsed().as_secs() >= self.sync_every {
            self.sync(false)?;
        }
        Ok(())
    }

    fn sync(&mut self, finished: bool) -> Result<(), String> {
        let Some(uri) = &self.remote else {
            return Ok(());
        };
        if self.verbose {
            eprintln!("isogeny_walk: trait S3 upload started");
        }
        for name in ["manifest.json", CHECKPOINT, "progress.json"] {
            self.push(uri, name)?;
        }
        if finished {
            for name in [
                "traits.jsonl",
                "class_audits.json",
                "STORE.json",
                "metrics.json",
                "result.json",
            ] {
                self.push(uri, name)?;
            }
        }
        self.last_sync = Instant::now();
        if self.verbose {
            eprintln!("isogeny_walk: trait S3 upload complete");
        }
        Ok(())
    }

    fn push(&self, uri: &str, name: &str) -> Result<(), String> {
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
    }

    pub(super) fn finish(&mut self) -> Result<(), String> {
        let metrics = fs::read(self.dir.join("metrics.json"))
            .map_err(|e| format!("reading completed trait metrics: {e}"))?;
        atomic_write(
            &self.dir.join("result.json"),
            &serde_json::to_vec(&json!({
                "complete": true, "completed_curves": self.lines.len(),
                "metrics_blake3": blake3::hash(&metrics).to_hex().to_string(),
            }))
            .map_err(|e| e.to_string())?,
        )?;
        self.sync(true)
    }
}
