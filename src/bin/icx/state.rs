//! Completed-repeat custody for `icx run`.

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
#[cfg(unix)]
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crypto_lib::cryptanalysis::ic_framework::RunReport;
use crypto_lib::cryptanalysis::ic_run::RhoRef;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

const MAGIC: &str = "ICXCKPT1 ";

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct Snapshot {
    fingerprint: String,
    pub reports: Vec<Value>,
    pub rho: Option<RhoRef>,
    pub config_label: String,
    pub origin_host: String,
}

pub(super) struct RunState {
    dir: PathBuf,
    remote: Option<String>,
    _lock: File,
    pub snapshot: Snapshot,
    verbose: bool,
}

pub(super) fn host_name() -> String {
    Command::new("hostname")
        .output()
        .ok()
        .filter(|out| out.status.success())
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "unknown".into())
}

pub(super) fn binary_digest() -> Result<String, String> {
    let path = std::env::current_exe().map_err(|e| format!("locating ICX executable: {e}"))?;
    let mut file = File::open(&path).map_err(|e| format!("reading {}: {e}", path.display()))?;
    let mut hash = blake3::Hasher::new();
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|e| format!("reading {}: {e}", path.display()))?;
        if count == 0 {
            break;
        }
        hash.update(&buffer[..count]);
    }
    Ok(hash.finalize().to_hex().to_string())
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
        .ok_or("--s3-uri must be s3://BUCKET/PREFIX")?;
    let listing = aws(&[
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
    let value: Value =
        serde_json::from_slice(&listing.stdout).map_err(|e| format!("invalid S3 listing: {e}"))?;
    Ok(value["KeyCount"].as_u64().unwrap_or(0) > 0)
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
    fs::rename(&temp, path).map_err(|e| format!("publishing {}: {e}", path.display()))?;
    #[cfg(unix)]
    File::open(path.parent().ok_or("ICX state path has no parent")?)
        .and_then(|dir| dir.sync_all())
        .map_err(|e| format!("syncing ICX state directory: {e}"))?;
    Ok(())
}

fn has_payload(dir: &Path) -> Result<bool, String> {
    for entry in fs::read_dir(dir).map_err(|e| format!("reading {}: {e}", dir.display()))? {
        let entry = entry.map_err(|e| format!("reading {}: {e}", dir.display()))?;
        if entry.file_name() != ".lock" {
            return Ok(true);
        }
    }
    Ok(false)
}

fn save_checkpoint(path: &Path, snapshot: &Snapshot) -> Result<(), String> {
    let payload =
        serde_json::to_vec(snapshot).map_err(|e| format!("serializing ICX checkpoint: {e}"))?;
    let bytes = [
        format!("{MAGIC}{}\n", blake3::hash(&payload).to_hex()).into_bytes(),
        payload,
    ]
    .concat();
    atomic_write(path, &bytes)
}

fn load_checkpoint(path: &Path, fingerprint: &str) -> Result<Snapshot, String> {
    let bytes = fs::read(path).map_err(|e| format!("reading {}: {e}", path.display()))?;
    let newline = bytes
        .iter()
        .position(|&b| b == b'\n')
        .ok_or_else(|| format!("{}: missing ICX checkpoint header", path.display()))?;
    let (header, body) = bytes.split_at(newline);
    let expected = std::str::from_utf8(header)
        .ok()
        .and_then(|s| s.strip_prefix(MAGIC))
        .ok_or_else(|| format!("{}: invalid ICX checkpoint version", path.display()))?;
    if expected != blake3::hash(&body[1..]).to_hex().as_str() {
        return Err(format!(
            "{}: ICX checkpoint checksum mismatch",
            path.display()
        ));
    }
    let snapshot: Snapshot = serde_json::from_slice(&body[1..])
        .map_err(|e| format!("{}: invalid ICX checkpoint: {e}", path.display()))?;
    if snapshot.fingerprint != fingerprint {
        return Err(format!(
            "{}: ICX checkpoint parameters differ",
            path.display()
        ));
    }
    Ok(snapshot)
}

impl RunState {
    pub(super) fn prepare(
        dir: PathBuf,
        resume: bool,
        remote: Option<String>,
        verbose: bool,
        manifest: Value,
    ) -> Result<Self, String> {
        if let Some(uri) = &remote {
            let Some((bucket, prefix)) = uri.strip_prefix("s3://").and_then(|s| s.split_once('/'))
            else {
                return Err("--s3-uri must be s3://BUCKET/PREFIX".into());
            };
            if bucket.is_empty() || prefix.trim_matches('/').is_empty() || uri.ends_with('/') {
                return Err(
                    "--s3-uri must name a nonempty bucket and prefix without a trailing slash"
                        .into(),
                );
            }
        }
        fs::create_dir_all(&dir).map_err(|e| format!("creating {}: {e}", dir.display()))?;
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(dir.join(".lock"))
            .map_err(|e| format!("opening ICX state lock: {e}"))?;
        #[cfg(unix)]
        if unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
            return Err(format!("{} is already in use", dir.display()));
        }
        #[cfg(not(unix))]
        return Err("ICX state directory locking is unavailable on this platform".into());
        let manifest_path = dir.join("manifest.json");
        if resume {
            if !manifest_path.exists() {
                if has_payload(&dir)? {
                    return Err("ICX state directory has no manifest but is not empty".into());
                }
                let uri = remote.as_ref().ok_or("no local ICX manifest to resume")?;
                if verbose {
                    eprintln!("icx: restoring S3 state");
                }
                aws(&[
                    "s3",
                    "sync",
                    &format!("{uri}/"),
                    &dir.to_string_lossy(),
                    "--exclude",
                    ".lock",
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
                        .map_err(|e| format!("invalid S3 ICX manifest: {e}"))?;
                    if found != manifest {
                        return Err("S3 ICX manifest differs from the local run".into());
                    }
                }
            }
            let found: Value = serde_json::from_slice(
                &fs::read(&manifest_path).map_err(|e| format!("reading ICX manifest: {e}"))?,
            )
            .map_err(|e| format!("invalid ICX manifest: {e}"))?;
            if found != manifest {
                return Err("ICX manifest differs from the requested run".into());
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
            atomic_write(
                &manifest_path,
                &serde_json::to_vec(&manifest).map_err(|e| e.to_string())?,
            )?;
        }
        let fingerprint = blake3::hash(&serde_json::to_vec(&manifest).map_err(|e| e.to_string())?)
            .to_hex()
            .to_string();
        let snapshot = if resume {
            load_checkpoint(&dir.join("icx.ckpt"), &fingerprint)?
        } else {
            Snapshot {
                fingerprint,
                reports: Vec::new(),
                rho: None,
                config_label: String::new(),
                origin_host: host_name(),
            }
        };
        let mut state = Self {
            dir,
            remote,
            _lock: lock,
            snapshot,
            verbose,
        };
        if !resume {
            save_checkpoint(&state.dir.join("icx.ckpt"), &state.snapshot)?;
            state.sync(false)?;
        }
        Ok(state)
    }

    pub(super) fn completed(&self) -> usize {
        self.snapshot.reports.len()
    }

    pub(super) fn repeat(
        &mut self,
        rep: usize,
        report: &RunReport,
        rho: Option<&RhoRef>,
        label: &str,
    ) -> Result<(), String> {
        if rep != self.snapshot.reports.len() {
            return Err("ICX checkpoint repeat index differs from the run".into());
        }
        self.snapshot
            .reports
            .push(serde_json::to_value(report).map_err(|e| e.to_string())?);
        self.snapshot.rho = rho.cloned();
        self.snapshot.config_label = label.into();
        save_checkpoint(&self.dir.join("icx.ckpt"), &self.snapshot)?;
        let progress = json!({"stage":"complete_repeat", "completed":self.completed()});
        atomic_write(
            &self.dir.join("progress.json"),
            &serde_json::to_vec(&progress).map_err(|e| e.to_string())?,
        )?;
        if self.verbose {
            eprintln!("icx: saved repeat {}", self.completed());
        }
        self.sync(false)
    }

    fn sync(&mut self, final_upload: bool) -> Result<(), String> {
        let Some(uri) = &self.remote else {
            return Ok(());
        };
        if self.verbose {
            eprintln!("icx: S3 state upload started");
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
        for name in ["manifest.json", "icx.ckpt", "progress.json"] {
            push(name)?;
        }
        if final_upload {
            push("result.json")?;
        }
        if self.verbose {
            eprintln!("icx: S3 state upload complete");
        }
        Ok(())
    }

    pub(super) fn finish(&mut self, result: &Value) -> Result<(), String> {
        atomic_write(
            &self.dir.join("result.json"),
            &serde_json::to_vec(result).map_err(|e| e.to_string())?,
        )?;
        self.sync(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkpoint_rejects_changed_inputs_and_corruption() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("icx-state-{}-{unique}", std::process::id()));
        let manifest = json!({"curve":"p192", "bits":7, "repeats":2});
        let first = RunState::prepare(dir.clone(), false, None, false, manifest.clone()).unwrap();
        assert_eq!(first.completed(), 0);
        assert!(RunState::prepare(dir.clone(), true, None, false, manifest.clone()).is_err());
        drop(first);
        let restored = RunState::prepare(dir.clone(), true, None, false, manifest.clone()).unwrap();
        assert_eq!(restored.completed(), 0);
        drop(restored);
        assert!(
            RunState::prepare(dir.clone(), true, None, false, json!({"curve":"p224"})).is_err()
        );
        let checkpoint = dir.join("icx.ckpt");
        let mut bytes = fs::read(&checkpoint).unwrap();
        *bytes.last_mut().unwrap() ^= 1;
        fs::write(&checkpoint, bytes).unwrap();
        let error = RunState::prepare(dir.clone(), true, None, false, manifest)
            .err()
            .expect("corrupt checkpoint rejected");
        assert!(error.contains("checksum mismatch"));
        fs::remove_dir_all(dir).unwrap();
    }
}
