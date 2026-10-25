//! Durable progress and S3 snapshots for the SQLite class walker.

use super::{counts, err, hash, Options, Plan, Result};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
#[cfg(unix)]
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

const SNAPSHOT_SCHEMA: u32 = 1;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SnapshotMarker {
    schema_version: u32,
    plan_sha256: String,
    sqlite_sha256: String,
    sqlite_bytes: u64,
    expanded: u64,
    key: String,
}

fn sidecar(path: &Path, suffix: &str) -> PathBuf {
    PathBuf::from(format!("{}.{suffix}", path.display()))
}

fn sync_parent(path: &Path) -> Result<()> {
    #[cfg(unix)]
    File::open(
        path.parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new(".")),
    )
    .and_then(|parent| parent.sync_all())
    .map_err(err)?;
    Ok(())
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    let temp = sidecar(path, "write.tmp");
    if temp.exists() || temp.is_symlink() {
        fs::remove_file(&temp).map_err(err)?;
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp)
        .map_err(err)?;
    file.write_all(bytes)
        .and_then(|_| file.sync_all())
        .map_err(err)?;
    fs::rename(&temp, path).map_err(err)?;
    sync_parent(path)
}

fn atomic_json(path: &Path, value: &impl Serialize) -> Result<()> {
    atomic_write(path, &serde_json::to_vec(value).map_err(err)?)
}

fn digest_file(path: &Path) -> Result<(String, u64)> {
    let mut file = File::open(path).map_err(err)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    let mut size = 0u64;
    loop {
        let count = file.read(&mut buffer).map_err(err)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
        size += count as u64;
    }
    Ok((format!("{:x}", hasher.finalize()), size))
}

fn aws(args: &[&str]) -> Result<Output> {
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

fn validate_uri(uri: &str) -> Result<()> {
    let Some((bucket, prefix)) = uri.strip_prefix("s3://").and_then(|s| s.split_once('/')) else {
        return Err("--state-s3-uri must be s3://BUCKET/PREFIX".into());
    };
    if bucket.is_empty() || prefix.trim_matches('/').is_empty() || uri.ends_with('/') {
        return Err(
            "--state-s3-uri needs a nonempty bucket and prefix without a trailing slash".into(),
        );
    }
    Ok(())
}

fn remote_has_objects(uri: &str) -> Result<bool> {
    let (bucket, prefix) = uri
        .strip_prefix("s3://")
        .and_then(|s| s.split_once('/'))
        .ok_or("invalid S3 state prefix")?;
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
    let value: Value = serde_json::from_slice(&listing.stdout).map_err(err)?;
    Ok(value["KeyCount"].as_u64().unwrap_or(0) > 0)
}

fn remote_marker(uri: &str) -> Result<Option<SnapshotMarker>> {
    if !remote_has_objects(uri)? {
        return Ok(None);
    }
    let result = aws(&[
        "s3",
        "cp",
        &format!("{uri}/LATEST.json"),
        "-",
        "--only-show-errors",
    ])?;
    let marker: SnapshotMarker = serde_json::from_slice(&result.stdout)
        .map_err(|e| format!("invalid S3 class snapshot marker: {e}"))?;
    if marker.schema_version != SNAPSHOT_SCHEMA
        || marker.sqlite_bytes == 0
        || marker.key
            != format!(
                "snapshots/{:012}-{}.sqlite",
                marker.expanded, marker.sqlite_sha256
            )
        || marker.sqlite_sha256.len() != 64
        || !marker.sqlite_sha256.bytes().all(|b| b.is_ascii_hexdigit())
        || marker.plan_sha256.len() != 64
        || !marker.plan_sha256.bytes().all(|b| b.is_ascii_hexdigit())
    {
        return Err("unsupported or invalid S3 class snapshot marker".into());
    }
    Ok(Some(marker))
}

fn lock(path: &Path) -> Result<File> {
    let lock_path = sidecar(path, "lock");
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&lock_path)
        .map_err(err)?;
    #[cfg(unix)]
    if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
        return Err(format!("{} is already in use", path.display()));
    }
    #[cfg(not(unix))]
    return Err("class state locking is unavailable on this platform".into());
    Ok(file)
}

struct Heartbeat {
    counts: Arc<Mutex<(u64, u64)>>,
    stop: mpsc::Sender<()>,
    worker: Option<JoinHandle<()>>,
}

impl Heartbeat {
    fn start(seconds: u64, target: u64) -> Self {
        let counts = Arc::new(Mutex::new((0, target)));
        let shared = Arc::clone(&counts);
        let (stop, receiver) = mpsc::channel();
        let worker = std::thread::spawn(move || {
            let start = Instant::now();
            while receiver
                .recv_timeout(Duration::from_secs(seconds))
                .is_err_and(|error| error == mpsc::RecvTimeoutError::Timeout)
            {
                let (done, target) = *shared.lock().expect("class progress lock");
                if target == 0 {
                    eprintln!(
                        "[isogeny-class] {:.1}s restoring/opening state",
                        start.elapsed().as_secs_f64()
                    );
                } else {
                    eprintln!(
                        "[isogeny-class] {:.1}s expanded {done}/{target}",
                        start.elapsed().as_secs_f64()
                    );
                }
            }
        });
        Self {
            counts,
            stop,
            worker: Some(worker),
        }
    }

    fn update(&self, done: u64, target: u64) {
        *self.counts.lock().expect("class progress lock") = (done, target);
    }
}

impl Drop for Heartbeat {
    fn drop(&mut self) {
        let _ = self.stop.send(());
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

pub(super) struct Custody {
    path: PathBuf,
    remote: Option<String>,
    marker: Option<SnapshotMarker>,
    restored: bool,
    _lock: File,
    heartbeat: Option<Heartbeat>,
    verbose: bool,
    sync_every: Duration,
    last_sync: Instant,
}

impl Custody {
    fn new(path: &Path, options: &Options, target: u64) -> Result<Self> {
        let remote = options.get("state-s3-uri").map(str::to_owned);
        if let Some(uri) = &remote {
            validate_uri(uri)?;
        }
        if options.get("sync-every").is_some() && remote.is_none() {
            return Err("--sync-every requires --state-s3-uri".into());
        }
        let every = |name: &str, default: u64| -> Result<u64> {
            let value = options
                .get(name)
                .map_or(Ok(default), |v| v.parse::<u64>().map_err(err))?;
            if value == 0 {
                return Err(format!("--{name} must be positive"));
            }
            Ok(value)
        };
        let verbose = options.get("verbose").is_some() || options.get("progress-every").is_some();
        let heartbeat = if verbose {
            Some(Heartbeat::start(every("progress-every", 5)?, target))
        } else {
            None
        };
        let sync_every = Duration::from_secs(every("sync-every", 60)?);
        let lock = lock(path)?;
        Ok(Self {
            path: path.to_owned(),
            remote,
            marker: None,
            restored: false,
            _lock: lock,
            heartbeat,
            verbose,
            sync_every,
            last_sync: Instant::now(),
        })
    }

    pub(super) fn build(path: &Path, plan: &Plan, options: &Options) -> Result<Self> {
        let state = Self::new(path, options, plan.expected_vertices)?;
        if path.exists()
            || ["progress.json", "result.json", "snapshot.json"]
                .iter()
                .any(|suffix| sidecar(path, suffix).exists())
        {
            return Err(format!(
                "{} already contains class state; use resume",
                path.display()
            ));
        }
        if let Some(uri) = &state.remote {
            if remote_has_objects(uri)? {
                return Err("S3 class prefix is not empty; use a unique prefix or resume".into());
            }
        }
        Ok(state)
    }

    pub(super) fn resume(path: &Path, options: &Options) -> Result<Self> {
        let mut state = Self::new(path, options, 0)?;
        if let Some(uri) = &state.remote {
            state.marker = remote_marker(uri)?;
        }
        if !path.is_file() {
            let uri = state
                .remote
                .as_ref()
                .ok_or("no local class database or S3 snapshot to resume")?;
            if ["progress.json", "result.json", "snapshot.json"]
                .iter()
                .any(|suffix| sidecar(path, suffix).exists())
            {
                return Err(
                    "class state has sidecars but no database; restore into a fresh path".into(),
                );
            }
            let marker = state
                .marker
                .as_ref()
                .ok_or("no local class database or S3 snapshot to resume")?;
            if state.verbose {
                eprintln!("[isogeny-class] restoring S3 snapshot");
            }
            let temp = sidecar(path, "restore.tmp");
            if temp.exists() || temp.is_symlink() {
                fs::remove_file(&temp).map_err(err)?;
            }
            aws(&[
                "s3",
                "cp",
                &format!("{uri}/{}", marker.key),
                &temp.to_string_lossy(),
                "--only-show-errors",
            ])?;
            let (digest, size) = digest_file(&temp)?;
            if digest != marker.sqlite_sha256 || size != marker.sqlite_bytes {
                return Err("S3 class snapshot checksum or size mismatch".into());
            }
            fs::rename(&temp, path).map_err(err)?;
            sync_parent(path)?;
            state.restored = true;
        }
        Ok(state)
    }

    pub(super) fn validate_plan(&self, db: &Connection, plan: &Plan) -> Result<()> {
        let Some(marker) = &self.marker else {
            return Ok(());
        };
        let integrity: String = db
            .query_row("PRAGMA integrity_check", [], |row| row.get(0))
            .map_err(err)?;
        if integrity != "ok" {
            return Err(format!("class SQLite integrity_check: {integrity}"));
        }
        let mut foreign_keys = db.prepare("PRAGMA foreign_key_check").map_err(err)?;
        if foreign_keys
            .query([])
            .map_err(err)?
            .next()
            .map_err(err)?
            .is_some()
        {
            return Err("class snapshot foreign key integrity failed".into());
        }
        let fingerprint = hash(&serde_json::to_vec(plan).map_err(err)?);
        if marker.plan_sha256 != fingerprint {
            return Err("S3 class snapshot plan differs from the local database".into());
        }
        let (_, expanded, _) = counts(db)?;
        if self.restored && marker.expanded != expanded {
            return Err("S3 class snapshot expansion count differs from SQLite".into());
        }
        if marker.expanded > expanded {
            return Err(
                "S3 class snapshot is ahead of local state; restore into a fresh path".into(),
            );
        }
        Ok(())
    }

    pub(super) fn progress(&mut self, db: &Connection, plan: &Plan) -> Result<()> {
        let (vertices, expanded, edges) = counts(db)?;
        if let Some(heartbeat) = &self.heartbeat {
            heartbeat.update(expanded, plan.expected_vertices);
        }
        if self.verbose {
            eprintln!(
                "[isogeny-class] expanded {expanded}/{} vertices={vertices} edges={edges}",
                plan.expected_vertices
            );
        }
        atomic_json(
            &sidecar(&self.path, "progress.json"),
            &json!({
                "schema_version": 1, "expanded": expanded, "expected": plan.expected_vertices,
                "vertices": vertices, "edges": edges,
            }),
        )?;
        if self.remote.is_some() && self.last_sync.elapsed() >= self.sync_every {
            if let Err(error) = self.sync(db, plan) {
                eprintln!("[isogeny-class] S3 snapshot failed: {error}");
                self.last_sync = Instant::now();
            }
        }
        Ok(())
    }

    pub(super) fn start(&mut self, db: &Connection, plan: &Plan) -> Result<()> {
        self.progress(db, plan)?;
        if self.remote.is_some() && self.marker.is_none() {
            self.sync(db, plan)?;
        }
        Ok(())
    }

    fn sync(&mut self, db: &Connection, plan: &Plan) -> Result<()> {
        let Some(uri) = &self.remote else {
            return Ok(());
        };
        if self.verbose {
            eprintln!("[isogeny-class] S3 snapshot started");
        }
        let started = Instant::now();
        let temp = sidecar(&self.path, "snapshot.tmp.sqlite");
        if temp.exists() || temp.is_symlink() {
            fs::remove_file(&temp).map_err(err)?;
        }
        db.execute("VACUUM INTO ?1", [temp.to_string_lossy().as_ref()])
            .map_err(err)?;
        File::open(&temp)
            .and_then(|file| file.sync_all())
            .map_err(err)?;
        let (digest, size) = digest_file(&temp)?;
        let (_, expanded, _) = counts(db)?;
        let marker = SnapshotMarker {
            schema_version: SNAPSHOT_SCHEMA,
            plan_sha256: hash(&serde_json::to_vec(plan).map_err(err)?),
            sqlite_sha256: digest.clone(),
            sqlite_bytes: size,
            expanded,
            key: format!("snapshots/{expanded:012}-{digest}.sqlite"),
        };
        aws(&[
            "s3",
            "cp",
            &temp.to_string_lossy(),
            &format!("{uri}/{}", marker.key),
            "--only-show-errors",
        ])?;
        let marker_path = sidecar(&self.path, "snapshot.json");
        atomic_json(&marker_path, &marker)?;
        aws(&[
            "s3",
            "cp",
            &marker_path.to_string_lossy(),
            &format!("{uri}/LATEST.json"),
            "--only-show-errors",
        ])?;
        fs::remove_file(&temp).map_err(err)?;
        self.marker = Some(marker);
        self.last_sync = Instant::now();
        if self.verbose {
            eprintln!(
                "[isogeny-class] S3 snapshot complete in {:.1}s",
                started.elapsed().as_secs_f64()
            );
        }
        Ok(())
    }

    pub(super) fn finish(&mut self, db: &Connection, plan: &Plan, result: &Value) -> Result<()> {
        let result_path = sidecar(&self.path, "result.json");
        atomic_json(&result_path, result)?;
        if let Some(uri) = self.remote.clone() {
            self.sync(db, plan)?;
            aws(&[
                "s3",
                "cp",
                &result_path.to_string_lossy(),
                &format!("{uri}/result.json"),
                "--only-show-errors",
            ])?;
        }
        Ok(())
    }
}
