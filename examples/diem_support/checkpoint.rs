//! Durable instance receipts. A record is reusable only with the same source
//! identity, task grid, payload checksum and key. Arithmetic acceptance remains
//! the responsibility of the producer and its recorded controls.
#![allow(dead_code)] // Producer-only write APIs are exercised by this module's tests.
use crypto_lib::hash::sha256::sha256;
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static SERIAL: AtomicU64 = AtomicU64::new(0);

pub fn digest(bytes: &[u8]) -> String {
    sha256(bytes).iter().map(|b| format!("{b:02x}")).collect()
}

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

fn atomic_write(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let parent = path.parent().ok_or_else(|| invalid("missing parent"))?;
    let tmp = parent.join(format!(
        ".{}.tmp-{}-{}-{}",
        path.file_name().unwrap().to_string_lossy(),
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos(),
        SERIAL.fetch_add(1, Ordering::Relaxed)
    ));
    let mut file = OpenOptions::new().create_new(true).write(true).open(&tmp)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    drop(file);
    fs::rename(&tmp, path)?;
    File::open(parent)?.sync_all()?;
    Ok(())
}

pub struct Store {
    root: PathBuf,
    records: PathBuf,
    identity: String,
    tasks: BTreeSet<(u32, u32)>,
    // flock is released by the kernel even after SIGKILL or a host reboot.
    // Never unlink the lock file: another process may already hold its inode.
    _lock: File,
    save_lock: std::sync::Mutex<()>,
}

impl Store {
    pub fn open(root: &Path, identity: &str, tasks: &[(u32, u32)]) -> io::Result<Self> {
        let grid: BTreeSet<_> = tasks.iter().copied().collect();
        if grid.len() != tasks.len() || grid.is_empty() {
            return Err(invalid("empty or duplicate task grid"));
        }
        fs::create_dir_all(root)?;
        if let Some(parent) = root.parent().filter(|p| !p.as_os_str().is_empty()) {
            File::open(parent)?.sync_all()?;
        }
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(root.join(".generator.lock"))?;
        lock.try_lock().map_err(|e| match e {
            std::fs::TryLockError::Error(e) => e,
            std::fs::TryLockError::WouldBlock => io::Error::from(io::ErrorKind::WouldBlock),
        })?;
        let records = root.join("instance-checkpoints-v1");
        fs::create_dir_all(&records)?;
        File::open(root)?.sync_all()?;
        let manifest = records.join("manifest.json");
        let expected = json!({"schema": 1, "source_sha256": identity, "tasks": grid});
        match fs::read(&manifest) {
            Ok(bytes) => {
                let existing: Value =
                    serde_json::from_slice(&bytes).map_err(|e| invalid(e.to_string()))?;
                if existing != expected {
                    return Err(invalid("checkpoint source identity or task grid differs; preserve this run and use a separate results directory"));
                }
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                let has_records = fs::read_dir(&records)?.try_fold(false, |found, entry| {
                    Ok::<_, io::Error>(
                        found || entry?.path().extension().is_some_and(|e| e == "json"),
                    )
                })?;
                if has_records || root.join("instances.jsonl").exists() {
                    return Err(invalid("unbound existing results require provenance review; refusing to replace or silently import them"));
                }
                atomic_write(&manifest, &serde_json::to_vec_pretty(&expected)?)?;
            }
            Err(e) => return Err(e),
        }
        let store = Self {
            root: root.to_owned(),
            records,
            identity: identity.to_owned(),
            tasks: grid,
            _lock: lock,
            save_lock: std::sync::Mutex::new(()),
        };
        for entry in fs::read_dir(&store.records)? {
            let path = entry?.path();
            if path.extension().is_some_and(|e| e == "json") && path != manifest {
                let allowed = store
                    .tasks
                    .iter()
                    .any(|&(k, i)| store.record_path(k, i) == path);
                if !allowed {
                    return Err(invalid(format!("unexpected checkpoint {}", path.display())));
                }
            }
        }
        // Validate all existing records before any new computation begins.
        for &(k, i) in &store.tasks {
            store.read(k, i)?;
        }
        match fs::read(store.root.join("instances.jsonl")) {
            Ok(bytes) if bytes != store.expected_body()?.as_bytes() => {
                return Err(invalid("existing frozen list differs from durable records"))
            }
            Ok(_) => (),
            Err(e) if e.kind() == io::ErrorKind::NotFound => (),
            Err(e) => return Err(e),
        }
        Ok(store)
    }

    fn record_path(&self, k: u32, i: u32) -> PathBuf {
        self.records.join(format!("k{k:02}-i{i:02}.json"))
    }

    fn validate_payload(&self, k: u32, i: u32, payload: &str) -> io::Result<()> {
        if !self.tasks.contains(&(k, i)) {
            return Err(invalid("key outside frozen grid"));
        }
        let value: Value = serde_json::from_str(payload).map_err(|e| invalid(e.to_string()))?;
        if value["k"].as_u64() != Some(k.into()) || value["idx"].as_u64() != Some(i.into()) {
            return Err(invalid("payload key differs from checkpoint key"));
        }
        if payload.contains('\n') || payload.contains('\r') {
            return Err(invalid("payload must occupy one JSONL record"));
        }
        Ok(())
    }

    pub fn read(&self, k: u32, i: u32) -> io::Result<Option<String>> {
        let bytes = match fs::read(self.record_path(k, i)) {
            Ok(v) => v,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(e),
        };
        let value: Value = serde_json::from_slice(&bytes).map_err(|e| invalid(e.to_string()))?;
        let payload = value["payload"]
            .as_str()
            .ok_or_else(|| invalid("missing payload"))?;
        if value["schema"] != 1
            || value["source_sha256"] != self.identity
            || value["payload_sha256"] != digest(payload.as_bytes())
        {
            return Err(invalid(
                "checkpoint schema, source identity or payload checksum mismatch",
            ));
        }
        self.validate_payload(k, i, payload)?;
        Ok(Some(payload.to_owned()))
    }

    pub fn save(&self, k: u32, i: u32, payload: &str) -> io::Result<()> {
        let _guard = self
            .save_lock
            .lock()
            .map_err(|_| invalid("checkpoint writer lock poisoned"))?;
        self.validate_payload(k, i, payload)?;
        if let Some(old) = self.read(k, i)? {
            return if old == payload {
                Ok(())
            } else {
                Err(invalid("refusing to overwrite completed instance"))
            };
        }
        let record = json!({"schema": 1, "source_sha256": self.identity,
            "payload_sha256": digest(payload.as_bytes()), "payload": payload});
        atomic_write(
            &self.record_path(k, i),
            &serde_json::to_vec_pretty(&record)?,
        )
    }

    pub fn missing(&self) -> io::Result<Vec<(u32, u32)>> {
        let mut out = vec![];
        for &(k, i) in &self.tasks {
            if self.read(k, i)?.is_none() {
                out.push((k, i));
            }
        }
        Ok(out)
    }

    fn expected_body(&self) -> io::Result<String> {
        let mut body = String::new();
        for &(k, i) in &self.tasks {
            let line = self
                .read(k, i)?
                .ok_or_else(|| invalid(format!("missing instance ({k},{i})")))?;
            body.push_str(&line);
            body.push('\n');
        }
        Ok(body)
    }

    pub fn publish(&self) -> io::Result<()> {
        let body = self.expected_body()?;
        let final_path = self.root.join("instances.jsonl");
        match fs::read(&final_path) {
            Ok(old) if old != body.as_bytes() => {
                return Err(invalid(
                    "existing frozen list differs; refusing to overwrite",
                ))
            }
            Ok(_) => (),
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                atomic_write(&final_path, body.as_bytes())?
            }
            Err(e) => return Err(e),
        }
        let frozen = format!("{}  instances.jsonl\n", digest(body.as_bytes()));
        atomic_write(&self.root.join("FROZEN.sha256"), frozen.as_bytes())
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::process::{Command, Stdio};

    struct Dir(PathBuf);
    impl Dir {
        fn new() -> Self {
            let p = std::env::temp_dir().join(format!(
                "diem-checkpoint-test-{}-{}",
                std::process::id(),
                SERIAL.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&p).unwrap();
            Self(p)
        }
    }
    impl Drop for Dir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    const GRID: [(u32, u32); 2] = [(8, 0), (8, 1)];
    const ROW: &str = "{\"k\":8,\"idx\":0,\"gen_wall_s\":1.25}";

    #[test]
    fn restart_reuses_exact_record_and_publishes_only_complete_grid() {
        let d = Dir::new();
        {
            let s = Store::open(&d.0, "source-a", &GRID).unwrap();
            s.save(8, 0, ROW).unwrap();
            assert!(s.publish().is_err());
        }
        let s = Store::open(&d.0, "source-a", &GRID).unwrap();
        assert_eq!(s.missing().unwrap(), vec![(8, 1)]);
        assert_eq!(s.read(8, 0).unwrap().as_deref(), Some(ROW));
        let next = "{\"k\":8,\"idx\":1}";
        s.save(8, 1, next).unwrap();
        s.publish().unwrap();
        let bytes = fs::read(d.0.join("instances.jsonl")).unwrap();
        assert_eq!(bytes, format!("{ROW}\n{next}\n").as_bytes());
        s.publish().unwrap();
        assert_eq!(
            fs::read_to_string(d.0.join("FROZEN.sha256")).unwrap(),
            format!("{}  instances.jsonl\n", digest(&bytes))
        );
    }
    #[test]
    fn concurrent_writer_is_rejected_and_lock_is_reusable() {
        let d = Dir::new();
        let s = Store::open(&d.0, "a", &GRID).unwrap();
        assert!(Store::open(&d.0, "a", &GRID).is_err());
        drop(s);
        assert!(Store::open(&d.0, "a", &GRID).is_ok());
    }
    #[test]
    fn changed_source_or_grid_is_rejected() {
        let d = Dir::new();
        drop(Store::open(&d.0, "a", &GRID).unwrap());
        assert!(Store::open(&d.0, "b", &GRID).is_err());
        assert!(Store::open(&d.0, "a", &[(8, 0)]).is_err());
    }
    #[test]
    fn corruption_and_wrong_key_are_rejected() {
        let d = Dir::new();
        let s = Store::open(&d.0, "a", &GRID).unwrap();
        assert!(s.save(8, 1, ROW).is_err());
        s.save(8, 0, ROW).unwrap();
        let p = s.record_path(8, 0);
        let mut v: Value = serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
        v["payload"] = Value::String(ROW.replace("1.25", "2.25"));
        fs::write(p, serde_json::to_vec(&v).unwrap()).unwrap();
        assert!(s.read(8, 0).is_err());
        drop(s);
        assert!(Store::open(&d.0, "a", &GRID).is_err());
    }
    #[test]
    fn partial_temporary_file_is_never_a_completed_record() {
        let d = Dir::new();
        {
            let s = Store::open(&d.0, "a", &GRID).unwrap();
            fs::write(s.records.join(".k08-i00.json.tmp-killed"), b"{partial").unwrap();
        }
        let s = Store::open(&d.0, "a", &GRID).unwrap();
        assert_eq!(s.missing().unwrap(), GRID);
    }
    #[test]
    fn completed_and_unbound_results_are_never_replaced() {
        let d = Dir::new();
        {
            let s = Store::open(&d.0, "a", &GRID).unwrap();
            s.save(8, 0, ROW).unwrap();
            assert!(s.save(8, 0, &ROW.replace("1.25", "2.25")).is_err());
        }
        let d2 = Dir::new();
        fs::write(d2.0.join("instances.jsonl"), ROW).unwrap();
        assert!(Store::open(&d2.0, "a", &GRID).is_err());
    }
    #[test]
    fn changed_frozen_file_is_rejected_before_work_starts() {
        let d = Dir::new();
        {
            let s = Store::open(&d.0, "a", &GRID).unwrap();
            s.save(8, 0, ROW).unwrap();
            s.save(8, 1, "{\"k\":8,\"idx\":1}").unwrap();
            s.publish().unwrap();
        }
        fs::write(d.0.join("instances.jsonl"), "altered").unwrap();
        assert!(Store::open(&d.0, "a", &GRID).is_err());
    }
    #[test]
    fn checkpoint_child() {
        if let Ok(root) = std::env::var("DIEM_CHECKPOINT_CHILD_DIR") {
            let s = Store::open(Path::new(&root), "a", &GRID).unwrap();
            s.save(8, 0, ROW).unwrap();
            fs::write(Path::new(&root).join("child-ready"), b"ready").unwrap();
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    }
    #[test]
    fn killed_writer_leaves_durable_record_and_releases_lock() {
        let d = Dir::new();
        let mut child = Command::new(std::env::current_exe().unwrap())
            .args(["checkpoint_child", "--test-threads=1"])
            .env("DIEM_CHECKPOINT_CHILD_DIR", &d.0)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let started = std::time::Instant::now();
        while !d.0.join("child-ready").exists() && started.elapsed().as_secs() < 10 {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        let ready = d.0.join("child-ready").exists();
        child.kill().unwrap();
        child.wait().unwrap();
        assert!(ready);
        let s = Store::open(&d.0, "a", &GRID).unwrap();
        assert_eq!(s.read(8, 0).unwrap().as_deref(), Some(ROW));
        assert_eq!(s.missing().unwrap(), vec![(8, 1)]);
    }
}
