//! Diagnostic prefixes only. This module supplies no accepted result or price.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    time::Instant,
};
pub const MAX_SNAPSHOTS: u64 = 256;
pub const MAX_BYTES: usize = 65536;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Identity {
    pub session: String,
    pub execution: u64,
    pub workload_sha256: String,
    pub binary_sha256: String,
    pub protocol_sha256: String,
    pub seed: u64,
    pub attempt: u64,
    pub source_sha256: String,
    pub method_config_sha256: String,
}
impl Identity {
    pub fn validate(&self) -> io::Result<()> {
        if self.session.is_empty()
            || self.session.len() > 128
            || [
                &self.workload_sha256,
                &self.binary_sha256,
                &self.protocol_sha256,
                &self.source_sha256,
                &self.method_config_sha256,
            ]
            .iter()
            .any(|x| x.len() != 64 || !x.bytes().all(|b| b.is_ascii_hexdigit()))
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid identity",
            ));
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Snapshot {
    pub schema: String,
    pub identity: Identity,
    pub sequence: u64,
    pub diagnostic_only: bool,
    pub complete: bool,
    pub accepted_result: Option<u64>,
    pub candidate_over_rho: Option<f64>,
    pub boundary: String,
    pub epoch: String,
    pub walk: Option<u64>,
    pub completed_candidates: u64,
    pub counters: Value,
    pub prior_io: IoTotals,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct IoTotals {
    pub attempted: u64,
    /// Files published and parent-directory synchronization confirmed.
    pub committed: u64,
    /// Files atomically made visible, including later durability failures.
    pub published: u64,
    pub published_bytes: u64,
    pub bytes: u64,
    pub completed_write_ns: u128,
    pub failure: Option<String>,
}
pub trait Sink {
    fn enabled(&self) -> bool {
        true
    }
    fn emit(
        &mut self,
        boundary: &str,
        epoch: &str,
        walk: Option<u64>,
        completed: u64,
        counters: Value,
    );
}
pub struct Noop;
impl Sink for Noop {
    fn enabled(&self) -> bool {
        false
    }
    fn emit(&mut self, _: &str, _: &str, _: Option<u64>, _: u64, _: Value) {}
}
pub fn milestone(n: u64) -> bool {
    matches!(n, 1 | 4 | 16 | 64 | 256 | 1024 | 4096 | 16384)
}
struct Bounded {
    bytes: Vec<u8>,
}
impl Write for Bounded {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        if b.len() > MAX_BYTES - self.bytes.len() {
            return Err(io::Error::other("snapshot exceeds byte cap"));
        }
        self.bytes.extend_from_slice(b);
        Ok(b.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
pub fn validate(snapshot: &Snapshot, identity: &Identity, expected: u64) -> io::Result<()> {
    identity.validate()?;
    if snapshot.schema != "pencil-progress/v1"
        || &snapshot.identity != identity
        || snapshot.sequence != expected
        || snapshot.sequence >= MAX_SNAPSHOTS
        || !snapshot.diagnostic_only
        || snapshot.complete
        || snapshot.accepted_result.is_some()
        || snapshot.candidate_over_rho.is_some()
        || !matches!(snapshot.epoch.as_str(), "cold_setup" | "candidate")
    {
        return Err(io::Error::other("invalid diagnostic snapshot"));
    }
    Ok(())
}
#[cfg(test)]
#[derive(Clone, Copy, PartialEq)]
enum Fault {
    Unlink,
    DirectorySync,
}
pub struct AtomicSink {
    dir: PathBuf,
    identity: Identity,
    pub totals: IoTotals,
    #[cfg(test)]
    fault: Option<Fault>,
}
impl AtomicSink {
    /// Creates a new private leaf directory. Parent must be runner-owned and trusted.
    pub fn create(dir: &Path, identity: Identity) -> io::Result<Self> {
        identity.validate()?;
        let mut builder = fs::DirBuilder::new();
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        builder.create(dir)?;
        Ok(Self {
            dir: dir.into(),
            identity,
            totals: IoTotals::default(),
            #[cfg(test)]
            fault: None,
        })
    }
    fn write_snapshot(&mut self, s: &Snapshot) -> io::Result<usize> {
        validate(s, &self.identity, self.totals.committed)?;
        let mut buffer = Bounded { bytes: Vec::new() };
        serde_json::to_writer(&mut buffer, s).map_err(io::Error::other)?;
        let tmp = self.dir.join(format!("{:06}.tmp", s.sequence));
        let dst = self.dir.join(format!("{:06}.json", s.sequence));
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options.open(&tmp)?;
        file.write_all(&buffer.bytes)?;
        file.sync_all()?;
        drop(file);
        // hard_link publishes atomically without clobbering an existing committed file.
        // Both names are siblings; a failure leaves the temporary as diagnostic evidence.
        fs::hard_link(&tmp, &dst)?;
        self.totals.published += 1;
        self.totals.published_bytes += buffer.bytes.len() as u64;
        #[cfg(test)]
        if self.fault == Some(Fault::Unlink) {
            return Err(io::Error::other("injected unlink failure"));
        }
        fs::remove_file(&tmp)?;
        #[cfg(test)]
        if self.fault == Some(Fault::DirectorySync) {
            return Err(io::Error::other("injected directory sync failure"));
        }
        fs::File::open(&self.dir)?.sync_all()?;
        Ok(buffer.bytes.len())
    }
}
impl Sink for AtomicSink {
    fn enabled(&self) -> bool {
        self.totals.failure.is_none() && self.totals.attempted < MAX_SNAPSHOTS
    }
    fn emit(
        &mut self,
        boundary: &str,
        epoch: &str,
        walk: Option<u64>,
        completed: u64,
        counters: Value,
    ) {
        if !self.enabled() {
            return;
        }
        let s = Snapshot {
            schema: "pencil-progress/v1".into(),
            identity: self.identity.clone(),
            sequence: self.totals.committed,
            diagnostic_only: true,
            complete: false,
            accepted_result: None,
            candidate_over_rho: None,
            boundary: boundary.into(),
            epoch: epoch.into(),
            walk,
            completed_candidates: completed,
            counters,
            prior_io: self.totals.clone(),
        };
        self.totals.attempted += 1;
        let start = Instant::now();
        let result = self.write_snapshot(&s);
        self.totals.completed_write_ns += start.elapsed().as_nanos();
        match result {
            Ok(n) => {
                self.totals.committed += 1;
                self.totals.bytes += n as u64
            }
            Err(e) => self.totals.failure = Some(e.to_string()),
        }
    }
}
#[cfg(test)]
mod diagnostic_fixture_tests {
    use super::*;
    fn id() -> Identity {
        Identity {
            session: "fixture".into(),
            execution: 0,
            workload_sha256: "a".repeat(64),
            binary_sha256: "b".repeat(64),
            protocol_sha256: "c".repeat(64),
            seed: 9,
            attempt: 0,
            source_sha256: "d".repeat(64),
            method_config_sha256: "e".repeat(64),
        }
    }
    fn dir(tag: &str) -> PathBuf {
        std::env::temp_dir().join(format!("pencil-progress-{}-{tag}", std::process::id()))
    }
    #[test]
    fn atomic_epochs_identity_and_bounds() {
        let d = dir("epochs");
        let mut s = AtomicSink::create(&d, id()).unwrap();
        for epoch in ["cold_setup", "candidate"] {
            s.emit("done", epoch, None, 0, serde_json::json!({"counter":3}));
        }
        assert_eq!(s.totals.committed, 2);
        let bytes = fs::read(d.join("000001.json")).unwrap();
        let row: Snapshot = serde_json::from_slice(&bytes).unwrap();
        validate(&row, &id(), 1).unwrap();
        assert_eq!(row.epoch, "candidate");
        assert_eq!(row.counters["counter"], 3);
        assert!(validate(&row, &id(), 0).is_err());
        let mut wrong = id();
        wrong.seed += 1;
        assert!(validate(&row, &wrong, 1).is_err());
        s.emit(
            "oversized",
            "candidate",
            None,
            0,
            Value::String("x".repeat(MAX_BYTES)),
        );
        assert!(s.totals.failure.is_some());
        assert_eq!(s.totals.committed, 2);
        s.emit("disabled", "candidate", None, 0, Value::Null);
        assert_eq!(s.totals.attempted, 3);
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn failed_write_and_partial_file() {
        let d = dir("partial");
        let mut s = AtomicSink::create(&d, id()).unwrap();
        fs::write(d.join("000000.tmp"), b"partial").unwrap();
        s.emit("done", "cold_setup", None, 0, Value::Null);
        assert_eq!(s.totals.committed, 0);
        assert!(s.totals.failure.is_some());
        assert!(!d.join("000000.json").exists());
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn milestone_and_cap() {
        assert_eq!((0..20000).filter(|&n| milestone(n)).count(), 8);
        let d = dir("cap");
        let mut s = AtomicSink::create(&d, id()).unwrap();
        s.totals.attempted = MAX_SNAPSHOTS;
        s.emit("ignored", "candidate", None, 0, Value::Null);
        assert_eq!(s.totals.committed, 0);
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn noop_is_disabled() {
        assert!(!Noop.enabled());
    }
    #[test]
    fn existing_destination_is_preserved() {
        let d = dir("existing");
        let mut sink = AtomicSink::create(&d, id()).unwrap();
        fs::write(d.join("000000.json"), b"frozen").unwrap();
        sink.emit("done", "candidate", None, 0, Value::Null);
        assert!(sink.totals.failure.is_some());
        assert_eq!(fs::read(d.join("000000.json")).unwrap(), b"frozen");
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn partial_never_accepts_final_result() {
        let d = dir("acceptance");
        let mut sink = AtomicSink::create(&d, id()).unwrap();
        sink.emit("done", "candidate", None, 0, Value::Null);
        let mut row: Snapshot =
            serde_json::from_slice(&fs::read(d.join("000000.json")).unwrap()).unwrap();
        row.accepted_result = Some(1);
        assert!(validate(&row, &id(), 0).is_err());
        row.accepted_result = None;
        row.candidate_over_rho = Some(0.5);
        assert!(validate(&row, &id(), 0).is_err());
        assert!(serde_json::from_slice::<Snapshot>(b"{\"schema\":").is_err());
        fs::remove_dir_all(d).unwrap();
    }
    #[cfg(unix)]
    #[test]
    fn symlink_temporary_is_not_followed() {
        let d = dir("symlink");
        let mut sink = AtomicSink::create(&d, id()).unwrap();
        let target = d.join("sentinel");
        fs::write(&target, b"unchanged").unwrap();
        std::os::unix::fs::symlink(&target, d.join("000000.tmp")).unwrap();
        sink.emit("done", "candidate", None, 0, Value::Null);
        assert!(sink.totals.failure.is_some());
        assert_eq!(fs::read(&target).unwrap(), b"unchanged");
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn post_publication_failures_distinguish_durability() {
        for (name, fault, temp_remains) in [
            ("unlink", Fault::Unlink, true),
            ("sync", Fault::DirectorySync, false),
        ] {
            let d = dir(name);
            let mut sink = AtomicSink::create(&d, id()).unwrap();
            sink.fault = Some(fault);
            sink.emit("done", "candidate", None, 0, Value::Null);
            assert_eq!(sink.totals.published, 1);
            assert!(sink.totals.published_bytes > 0);
            assert_eq!(sink.totals.committed, 0);
            assert_eq!(sink.totals.bytes, 0);
            assert!(sink.totals.failure.is_some());
            assert!(!sink.enabled());
            let row: Snapshot =
                serde_json::from_slice(&fs::read(d.join("000000.json")).unwrap()).unwrap();
            validate(&row, &id(), 0).unwrap();
            assert_eq!(d.join("000000.tmp").exists(), temp_remains);
            sink.emit("ignored", "candidate", None, 0, Value::Null);
            assert_eq!(sink.totals.attempted, 1);
            fs::remove_dir_all(d).unwrap();
        }
    }
    #[cfg(unix)]
    #[test]
    fn private_unix_modes() {
        use std::os::unix::fs::PermissionsExt;
        let d = dir("modes");
        let mut sink = AtomicSink::create(&d, id()).unwrap();
        assert_eq!(fs::metadata(&d).unwrap().permissions().mode() & 0o077, 0);
        sink.emit("done", "candidate", None, 0, Value::Null);
        assert_eq!(
            fs::metadata(d.join("000000.json"))
                .unwrap()
                .permissions()
                .mode()
                & 0o077,
            0
        );
        assert_eq!(sink.totals.published, 1);
        assert_eq!(sink.totals.committed, 1);
        fs::remove_dir_all(d).unwrap();
    }
}
