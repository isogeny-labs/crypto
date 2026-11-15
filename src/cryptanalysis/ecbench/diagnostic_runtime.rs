//! Explicit, single-thread diagnostic lifecycle. Never accepted mathematics or cost.
//! Prefix counters live in a separate directory and are not interpreted here.
use super::{
    canonical::{reject_duplicate_keys, sha256_hex},
    diagnostics::{Context, Identity},
};
use serde::{Deserialize, Serialize};
use std::{
    cell::Cell,
    fs,
    io::{self, Read, Write},
    marker::PhantomData,
    path::{Path, PathBuf},
    rc::Rc,
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};

pub const MAX_BYTES: usize = 16_384;
const SCHEMA: &str = "ecbench.diagnostic_lifecycle/v1";
thread_local! {
    static OWNER: Cell<u64> = const { Cell::new(0) };
    static BUSY: Cell<bool> = const { Cell::new(false) };
}
static NEXT: AtomicU64 = AtomicU64::new(1);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinishStatus {
    Returned,
    Error,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IoTotals {
    pub attempted: u64,
    pub published: u64,
    pub committed: u64,
    pub published_bytes: u64,
    pub committed_bytes: u64,
    /// Completed writer interval only, not whole diagnostic overhead.
    pub writer_ns: u128,
    pub failure: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Record {
    schema: String,
    identity: Identity,
    stage: String,
    status: Option<FinishStatus>,
    diagnostic_only: bool,
    accepted_result: Option<u64>,
    accepted_cost: Option<u64>,
    prior_io: IoTotals,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LifecycleSummary {
    pub schema: String,
    pub identity: Identity,
    pub requested_finish: FinishStatus,
    pub io: IoTotals,
    pub lifecycle_durable: bool,
    pub diagnostic_only: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entry {
    pub name: String,
    pub sha256: String,
    pub bytes: usize,
    pub temporary: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LifecycleRead {
    pub identity: Identity,
    pub entries: Vec<Entry>,
    pub entered_present: bool,
    pub finished_present: bool,
    pub finish_status: Option<FinishStatus>,
    pub diagnostic_only: bool,
    pub durability_confirmed: Option<bool>,
}
struct Bounded(Vec<u8>);
impl Write for Bounded {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        if b.len() > MAX_BYTES - self.0.len() {
            return Err(io::Error::other("lifecycle byte cap"));
        }
        self.0.extend_from_slice(b);
        Ok(b.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
fn validate_identity(identity: &Identity) -> Result<(), String> {
    Context {
        schema: "ecbench.diagnostic_context/v1".into(),
        relative_leaf: "lifecycle".into(),
        identity: identity.clone(),
    }
    .validate()
}
fn private_directory(path: &Path) -> Result<(), String> {
    let parent = path.parent().ok_or("lifecycle parent absent")?;
    if !fs::symlink_metadata(parent)
        .map_err(|e| e.to_string())?
        .file_type()
        .is_dir()
    {
        return Err("lifecycle parent not directory".into());
    }
    let mut builder = fs::DirBuilder::new();
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder.create(path).map_err(|e| e.to_string())
}
/// Parent must be stable and runner-owned. No ambient variable enables this scope.
/// Non-Send: install, callbacks, finish and Drop belong to the installing thread.
pub struct DiagnosticScope {
    token: u64,
    directory: PathBuf,
    identity: Identity,
    io: IoTotals,
    _thread: PhantomData<Rc<()>>,
    #[cfg(test)]
    fault: Option<&'static str>,
}
impl DiagnosticScope {
    pub fn install(directory: &Path, identity: Identity) -> Result<Self, String> {
        validate_identity(&identity)?;
        if OWNER.with(|o| o.get() != 0) || BUSY.with(Cell::get) {
            return Err("diagnostic scope already installed".into());
        }
        private_directory(directory)?;
        let token = NEXT
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |x| x.checked_add(1))
            .map_err(|_| "diagnostic ownership exhausted")?;
        OWNER.with(|o| o.set(token));
        let mut scope = Self {
            token,
            directory: directory.into(),
            identity,
            io: IoTotals::default(),
            _thread: PhantomData,
            #[cfg(test)]
            fault: None,
        };
        scope.emit("entered", None);
        Ok(scope)
    }
    /// No mutable runtime borrow is exposed, and recursive callbacks fail cleanly.
    pub fn with_identity<T>(&self, f: impl FnOnce(&Identity) -> T) -> Result<T, String> {
        if OWNER.with(Cell::get) != self.token {
            return Err("diagnostic ownership mismatch".into());
        }
        if BUSY.with(|b| b.replace(true)) {
            return Err("reentrant diagnostic callback".into());
        }
        struct Clear;
        impl Drop for Clear {
            fn drop(&mut self) {
                BUSY.with(|b| b.set(false));
            }
        }
        let _clear = Clear;
        Ok(f(&self.identity))
    }
    fn write(&mut self, record: &Record) -> Result<usize, String> {
        let mut buffer = Bounded(Vec::new());
        serde_json::to_writer(&mut buffer, record).map_err(|e| e.to_string())?;
        let tmp = self.directory.join(format!("{}.tmp", record.stage));
        let dst = self.directory.join(format!("{}.json", record.stage));
        let mut options = fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options.open(tmp.clone()).map_err(|e| e.to_string())?;
        file.write_all(&buffer.0).map_err(|e| e.to_string())?;
        file.sync_all().map_err(|e| e.to_string())?;
        drop(file);
        fs::hard_link(&tmp, &dst).map_err(|e| e.to_string())?;
        self.io.published += 1;
        self.io.published_bytes += buffer.0.len() as u64;
        #[cfg(test)]
        if self.fault == Some("unlink") {
            return Err("injected unlink failure".into());
        }
        fs::remove_file(tmp).map_err(|e| e.to_string())?;
        #[cfg(test)]
        if self.fault == Some("sync") {
            return Err("injected sync failure".into());
        }
        fs::File::open(&self.directory)
            .and_then(|f| f.sync_all())
            .map_err(|e| e.to_string())?;
        Ok(buffer.0.len())
    }
    fn emit(&mut self, stage: &str, status: Option<FinishStatus>) {
        if self.io.failure.is_some() || self.io.attempted >= 2 {
            return;
        }
        let record = Record {
            schema: SCHEMA.into(),
            identity: self.identity.clone(),
            stage: stage.into(),
            status,
            diagnostic_only: true,
            accepted_result: None,
            accepted_cost: None,
            prior_io: self.io.clone(),
        };
        self.io.attempted += 1;
        let start = Instant::now();
        let result = self.write(&record);
        self.io.writer_ns += start.elapsed().as_nanos();
        match result {
            Ok(bytes) => {
                self.io.committed += 1;
                self.io.committed_bytes += bytes as u64
            }
            Err(e) => self.io.failure = Some(e),
        }
    }
    pub fn finish(mut self, status: FinishStatus) -> Result<LifecycleSummary, String> {
        if OWNER.with(Cell::get) != self.token || BUSY.with(Cell::get) {
            return Err("diagnostic scope busy or ownership mismatch".into());
        }
        self.emit("finished", Some(status));
        Ok(LifecycleSummary {
            schema: "ecbench.diagnostic_lifecycle_summary/v1".into(),
            identity: self.identity.clone(),
            requested_finish: status,
            lifecycle_durable: self.io.committed == 2 && self.io.failure.is_none(),
            io: self.io.clone(),
            diagnostic_only: true,
        })
    }
}
impl Drop for DiagnosticScope {
    fn drop(&mut self) {
        OWNER.with(|o| {
            if o.get() == self.token {
                o.set(0)
            }
        });
    }
}
fn read_file(path: &Path) -> Result<Vec<u8>, String> {
    if !fs::symlink_metadata(path)
        .map_err(|e| e.to_string())?
        .file_type()
        .is_file()
    {
        return Err("nonregular lifecycle file".into());
    }
    let mut options = fs::OpenOptions::new();
    options.read(true);
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW);
    }
    let file = options.open(path).map_err(|e| e.to_string())?;
    if !file.metadata().map_err(|e| e.to_string())?.is_file() {
        return Err("nonregular opened lifecycle".into());
    }
    let mut bytes = Vec::new();
    file.take((MAX_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() > MAX_BYTES {
        return Err("lifecycle read byte cap".into());
    }
    Ok(bytes)
}
/// Inspect only after the child is quiescent. Presence is not durability or success.
pub fn read_lifecycle(directory: &Path, identity: &Identity) -> Result<LifecycleRead, String> {
    validate_identity(identity)?;
    if !fs::symlink_metadata(directory)
        .map_err(|e| e.to_string())?
        .file_type()
        .is_dir()
    {
        return Err("lifecycle leaf not directory".into());
    }
    let mut files = std::collections::BTreeMap::new();
    for item in fs::read_dir(directory).map_err(|e| e.to_string())? {
        if files.len() >= 4 {
            return Err("lifecycle entry cap".into());
        }
        let item = item.map_err(|e| e.to_string())?;
        let name = item
            .file_name()
            .into_string()
            .map_err(|_| "invalid lifecycle filename")?;
        if !matches!(
            name.as_str(),
            "entered.json" | "entered.tmp" | "finished.json" | "finished.tmp"
        ) {
            return Err("unexpected lifecycle entry".into());
        }
        files.insert(name, read_file(&item.path())?);
    }
    if files.contains_key("entered.tmp")
        && (files.contains_key("finished.json") || files.contains_key("finished.tmp"))
    {
        return Err("lifecycle continued after incomplete entry".into());
    }
    if (files.contains_key("finished.json") || files.contains_key("finished.tmp"))
        && !files.contains_key("entered.json")
    {
        return Err("lifecycle finish without entry".into());
    }
    let mut status = None;
    for (stage, index) in [("entered", 0), ("finished", 1)] {
        let name = format!("{stage}.json");
        if let Some(bytes) = files.get(&name) {
            let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
            reject_duplicate_keys(text)?;
            let record: Record = serde_json::from_str(text).map_err(|e| e.to_string())?;
            if record.schema != SCHEMA
                || record.identity != *identity
                || record.stage != stage
                || !record.diagnostic_only
                || record.accepted_result.is_some()
                || record.accepted_cost.is_some()
                || record.status.is_some() != (index == 1)
            {
                return Err("invalid lifecycle record".into());
            }
            let expected_bytes = if index == 0 {
                0
            } else {
                files["entered.json"].len() as u64
            };
            let io = &record.prior_io;
            if io.attempted != index
                || io.published != index
                || io.committed != index
                || io.published_bytes != expected_bytes
                || io.committed_bytes != expected_bytes
                || io.failure.is_some()
                || (index == 0 && io.writer_ns != 0)
            {
                return Err("invalid lifecycle IO history".into());
            }
            status = record.status.or(status);
            if let Some(tmp) = files.get(&format!("{stage}.tmp")) {
                if tmp != bytes {
                    return Err("lifecycle linked temporary mismatch".into());
                }
            }
        }
    }
    Ok(LifecycleRead {
        identity: identity.clone(),
        entered_present: files.contains_key("entered.json"),
        finished_present: files.contains_key("finished.json"),
        finish_status: status,
        entries: files
            .into_iter()
            .map(|(name, bytes)| Entry {
                temporary: name.ends_with(".tmp"),
                name,
                sha256: sha256_hex(&bytes),
                bytes: bytes.len(),
            })
            .collect(),
        diagnostic_only: true,
        durability_confirmed: None,
    })
}

#[cfg(test)]
mod lifecycle_fixture_tests {
    use super::*;
    fn identity() -> Identity {
        Identity {
            session: "fixture".into(),
            execution: 7,
            workload_sha256: "a".repeat(64),
            binary_sha256: "b".repeat(64),
            protocol_sha256: "c".repeat(64),
            seed: u64::MAX,
            attempt: 0,
            source_sha256: "d".repeat(64),
            method_config_sha256: "e".repeat(64),
        }
    }
    fn path(tag: &str) -> PathBuf {
        std::env::temp_dir().join(format!("lifecycle-{}-{tag}", std::process::id()))
    }
    #[test]
    fn common_lifecycle_and_ownership() {
        let p = path("common");
        let scope = DiagnosticScope::install(&p, identity()).unwrap();
        assert!(DiagnosticScope::install(&path("nested"), identity()).is_err());
        scope
            .with_identity(|id| {
                assert_eq!(id.seed, u64::MAX);
                assert!(scope.with_identity(|_| ()).is_err());
            })
            .unwrap();
        let summary = scope.finish(FinishStatus::Returned).unwrap();
        assert_eq!(summary.io.committed, 2);
        let read = read_lifecycle(&p, &identity()).unwrap();
        assert!(read.finished_present);
        assert_eq!(read.finish_status, Some(FinishStatus::Returned));
        fs::remove_dir_all(p).unwrap();
    }
    #[test]
    fn panic_clears_without_finish() {
        let p = path("panic");
        let result = std::panic::catch_unwind(|| {
            let scope = DiagnosticScope::install(&p, identity()).unwrap();
            scope.with_identity(|_| panic!("fixture")).unwrap();
        });
        assert!(result.is_err());
        assert!(!read_lifecycle(&p, &identity()).unwrap().finished_present);
        let q = path("after-panic");
        DiagnosticScope::install(&q, identity())
            .unwrap()
            .finish(FinishStatus::Error)
            .unwrap();
        fs::remove_dir_all(p).unwrap();
        fs::remove_dir_all(q).unwrap();
    }
    #[test]
    fn failure_retains_published_and_no_clobber() {
        for fault in ["unlink", "sync"] {
            let p = path(fault);
            let mut scope = DiagnosticScope::install(&p, identity()).unwrap();
            scope.fault = Some(fault);
            let summary = scope.finish(FinishStatus::Error).unwrap();
            assert_eq!(summary.io.published, 2);
            assert_eq!(summary.io.committed, 1);
            assert!(summary.io.failure.is_some());
            assert!(read_lifecycle(&p, &identity()).unwrap().finished_present);
            assert!(DiagnosticScope::install(&p, identity()).is_err());
            fs::remove_dir_all(p).unwrap();
        }
    }
    #[test]
    fn reader_rejects_tamper_and_symlink() {
        let p = path("tamper");
        DiagnosticScope::install(&p, identity())
            .unwrap()
            .finish(FinishStatus::Returned)
            .unwrap();
        let wrong = Identity {
            seed: 0,
            ..identity()
        };
        assert!(read_lifecycle(&p, &wrong).is_err());
        fs::write(p.join("finished.json"), b"{\"stage\":1,\"stage\":2}").unwrap();
        assert!(read_lifecycle(&p, &identity()).is_err());
        #[cfg(unix)]
        {
            fs::remove_file(p.join("finished.json")).unwrap();
            std::os::unix::fs::symlink("/dev/null", p.join("finished.json")).unwrap();
            assert!(read_lifecycle(&p, &identity()).is_err());
        }
        fs::remove_dir_all(p).unwrap();
    }
    #[cfg(unix)]
    #[test]
    fn private_permissions() {
        use std::os::unix::fs::PermissionsExt;
        let p = path("modes");
        let scope = DiagnosticScope::install(&p, identity()).unwrap();
        assert_eq!(fs::metadata(&p).unwrap().permissions().mode() & 0o077, 0);
        assert_eq!(
            fs::metadata(p.join("entered.json"))
                .unwrap()
                .permissions()
                .mode()
                & 0o077,
            0
        );
        drop(scope);
        fs::remove_dir_all(p).unwrap();
    }
}
