//! Conservative S3 command/body reservations on a mounted durable authority.
//!
//! The controller allocates disjoint envelopes once. Lost allocations are never
//! refunded. Every worker reservation is committed to the authoritative journal
//! before its local mirror and before a transport command is admitted. Missing
//! authority, a stale worker mirror, or a lost journal requires a new allocation.
//! This assumes the named authority volume itself is durable and is not rolled
//! back together with workers. It is not an account-wide or invoice-cost cap.
use std::collections::BTreeSet;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;
#[cfg(unix)]
use std::time::Instant;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::store::backend::{Operation, Phase, RequestSpec, S3Admission, S3Guard};

const POLICY: &str = "crypto.ml-dataset.budget-policy/v1";
const GRANT: &str = "crypto.ml-dataset.budget-allocation/v1";
const ZERO: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const MAX_RECORD: u64 = 128 * 1024;
const MAX_RECORDS: usize = 1_000_000;
const CONTROLLER_LOCK_TIMEOUT: Duration = Duration::from_secs(30);
#[cfg(unix)]
const LOCK_RETRY_INTERVAL: Duration = Duration::from_millis(10);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limits {
    pub upload_bytes: u64,
    pub download_bytes: u64,
    pub put_attempts: u64,
    pub get_attempts: u64,
    pub head_attempts: u64,
    pub list_attempts: u64,
}
impl Limits {
    fn add(self, other: Self) -> Result<Self, String> {
        let plus = |a: u64, b: u64| {
            a.checked_add(b)
                .ok_or("budget counter overflow".to_string())
        };
        Ok(Self {
            upload_bytes: plus(self.upload_bytes, other.upload_bytes)?,
            download_bytes: plus(self.download_bytes, other.download_bytes)?,
            put_attempts: plus(self.put_attempts, other.put_attempts)?,
            get_attempts: plus(self.get_attempts, other.get_attempts)?,
            head_attempts: plus(self.head_attempts, other.head_attempts)?,
            list_attempts: plus(self.list_attempts, other.list_attempts)?,
        })
    }
    fn fits(self, cap: Self) -> bool {
        self.upload_bytes <= cap.upload_bytes
            && self.download_bytes <= cap.download_bytes
            && self.put_attempts <= cap.put_attempts
            && self.get_attempts <= cap.get_attempts
            && self.head_attempts <= cap.head_attempts
            && self.list_attempts <= cap.list_attempts
    }
    fn minus(self, other: Self) -> Result<Self, String> {
        if !other.fits(self) {
            return Err("finalization reserve exceeds allocation".into());
        }
        Ok(Self {
            upload_bytes: self.upload_bytes - other.upload_bytes,
            download_bytes: self.download_bytes - other.download_bytes,
            put_attempts: self.put_attempts - other.put_attempts,
            get_attempts: self.get_attempts - other.get_attempts,
            head_attempts: self.head_attempts - other.head_attempts,
            list_attempts: self.list_attempts - other.list_attempts,
        })
    }
    fn cost(request: &RequestSpec) -> Result<Self, String> {
        if (request.operation != Operation::Put && request.upload_bytes != 0)
            || (request.operation != Operation::Get && request.download_bytes != 0)
        {
            return Err("request bytes disagree with S3 operation".into());
        }
        let mut cost = Self {
            upload_bytes: request.upload_bytes,
            download_bytes: request.download_bytes,
            ..Self::default()
        };
        match request.operation {
            Operation::Put => cost.put_attempts = 1,
            Operation::Get => cost.get_attempts = 1,
            Operation::Head => cost.head_attempts = 1,
            Operation::List => cost.list_attempts = 1,
        }
        Ok(cost)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Destination {
    pub bucket: String,
    pub prefix: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BudgetPolicy {
    pub schema: String,
    pub campaign_id: String,
    pub authority_dir: PathBuf,
    pub allowed_destinations: Vec<Destination>,
    pub limits: Limits,
    pub command_timeout_secs: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Allocation {
    schema: String,
    policy_sha256: String,
    allocation_id: String,
    sequence: u64,
    previous_sha256: String,
    limits: Limits,
    finalization_reserve: Limits,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Binding {
    schema: String,
    policy_sha256: String,
    allocation_sha256: String,
    journal_path: PathBuf,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Head {
    sequence: u64,
    sha256: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Reservation {
    schema: String,
    sequence: u64,
    previous_sha256: String,
    allocation_sha256: String,
    request: RequestSpec,
    total: Limits,
    bulk: Limits,
}

fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}
fn bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, String> {
    let mut b = serde_json::to_vec(value).map_err(err)?;
    b.push(b'\n');
    Ok(b)
}
fn digest(b: &[u8]) -> String {
    super::digest(b)
}
fn safe_id(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 128
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}
fn safe_key(s: &str) -> bool {
    !s.is_empty()
        && !s.starts_with('/')
        && !s.ends_with('/')
        && s.len() <= 1024
        && !s.bytes().any(|b| b.is_ascii_control() || b == b'\\')
        && s.split('/').all(|p| !p.is_empty() && p != "." && p != "..")
}

/// Reject symlinks in every ancestor; authority paths must be absolute.
fn directory(path: &Path, create: bool) -> Result<PathBuf, String> {
    if !path.is_absolute() {
        return Err("budget paths must be absolute".into());
    }
    let mut current = PathBuf::new();
    for component in path.components() {
        match component {
            Component::RootDir | Component::Normal(_) => current.push(component.as_os_str()),
            _ => return Err("noncanonical budget path".into()),
        }
        match fs::symlink_metadata(&current) {
            Ok(m) if m.is_dir() && !m.file_type().is_symlink() => {}
            Ok(_) => {
                return Err(format!(
                    "budget path is not a plain directory: {}",
                    current.display()
                ))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound && create => {
                fs::create_dir(&current).map_err(err)?;
                sync(current.parent().ok_or("directory parent missing")?)?;
            }
            Err(e) => {
                return Err(format!(
                    "budget authority/path unavailable: {}: {e}",
                    current.display()
                ))
            }
        }
    }
    Ok(current)
}
fn sync(path: &Path) -> Result<(), String> {
    File::open(path).and_then(|f| f.sync_all()).map_err(err)
}
fn read_bytes(path: &Path) -> Result<Vec<u8>, String> {
    let parent = path.parent().ok_or("budget file lacks parent")?;
    directory(parent, false)?;
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC);
    }
    let mut file = options.open(path).map_err(err)?;
    let meta = file.metadata().map_err(err)?;
    if !meta.is_file() || meta.len() > MAX_RECORD {
        return Err("budget record is not a bounded regular file".into());
    }
    let mut data = Vec::new();
    Read::by_ref(&mut file)
        .take(MAX_RECORD + 1)
        .read_to_end(&mut data)
        .map_err(err)?;
    if data.len() as u64 > MAX_RECORD {
        return Err("budget record grew beyond cap".into());
    }
    Ok(data)
}
fn read<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
    serde_json::from_slice(&read_bytes(path)?).map_err(err)
}
fn write_new(path: &Path, data: &[u8]) -> Result<(), String> {
    if data.len() as u64 > MAX_RECORD {
        return Err("budget record exceeds cap".into());
    }
    let parent = directory(path.parent().ok_or("budget file lacks parent")?, false)?;
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options
            .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC)
            .mode(0o600);
    }
    let mut file = options.open(path).map_err(err)?;
    // A partial record poisons this allocation; it is never truncated/reused.
    file.write_all(data).map_err(err)?;
    file.sync_all().map_err(err)?;
    sync(&parent)
}
fn replace_head(path: &Path, head: &Head) -> Result<(), String> {
    // The caller holds the controller or allocation lock. Old journal entries
    // remain immutable; this independent watermark detects a truncated history.
    let parent = path.parent().ok_or("head parent missing")?;
    let suffix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(err)?
        .as_nanos();
    let pending = parent.join(format!("head-pending-{}-{suffix}", std::process::id()));
    write_new(&pending, &bytes(head)?)?;
    let metadata = fs::symlink_metadata(path).map_err(err)?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err("authority head is not regular".into());
    }
    fs::rename(&pending, path).map_err(err)?;
    sync(parent)
}
fn verify_head(path: &Path, sequence: u64, sha256: &str) -> Result<(), String> {
    if read::<Head>(path)?
        != (Head {
            sequence,
            sha256: sha256.into(),
        })
    {
        return Err("authoritative budget high watermark differs; refusing rollback".into());
    }
    Ok(())
}
fn initialization_marker(p: &BudgetPolicy) -> Result<PathBuf, String> {
    let name = p
        .authority_dir
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("invalid authority directory name")?;
    Ok(p.authority_dir
        .parent()
        .ok_or("authority parent missing")?
        .join(format!(".{name}.budget-authority-created")))
}
fn locked(path: &Path) -> Result<File, String> {
    lock_file(path, None)
}
fn controller_locked(path: &Path) -> Result<File, String> {
    lock_file(path, Some(CONTROLLER_LOCK_TIMEOUT))
}
fn lock_file(path: &Path, wait: Option<Duration>) -> Result<File, String> {
    #[cfg(not(unix))]
    {
        let _ = (path, wait);
        Err("durable budget locking requires Unix".into())
    }
    #[cfg(unix)]
    {
        locked_with(path, wait, try_lock)
    }
}
#[cfg(unix)]
fn try_lock(file: &File) -> std::io::Result<()> {
    use std::os::fd::AsRawFd;
    if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}
#[cfg(unix)]
fn lock_metadata(path: &Path) -> Result<fs::Metadata, String> {
    directory(path.parent().ok_or("budget lock lacks parent")?, false)?;
    let metadata = fs::symlink_metadata(path).map_err(err)?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err("budget lock is not regular".into());
    }
    Ok(metadata)
}
#[cfg(unix)]
fn verify_lock_identity(path: &Path, file: &File, initial: &fs::Metadata) -> Result<(), String> {
    use std::os::unix::fs::MetadataExt;
    let named = lock_metadata(path)?;
    let opened = file.metadata().map_err(err)?;
    if !opened.is_file()
        || named.dev() != initial.dev()
        || named.ino() != initial.ino()
        || opened.dev() != initial.dev()
        || opened.ino() != initial.ino()
    {
        return Err("budget lock path identity changed".into());
    }
    Ok(())
}
// flock(2) associates locks with open descriptions: replacing a pathname while
// waiting must not let a stale descriptor authorize a different authority.
// LOCK_NB reports EWOULDBLOCK for contention; EINTR may be retried, other errors
// are failures. See https://man7.org/linux/man-pages/man2/flock.2.html and
// https://doc.rust-lang.org/std/time/struct.Instant.html for the monotonic clock.
// The deadline bounds retries, not filesystem I/O on an unavailable mount.
#[cfg(unix)]
fn locked_with(
    path: &Path,
    wait: Option<Duration>,
    mut attempt: impl FnMut(&File) -> std::io::Result<()>,
) -> Result<File, String> {
    use std::os::unix::fs::OpenOptionsExt;
    // Session contention never waits. Its deadline only bounds repeated EINTR.
    let timeout = wait.unwrap_or(CONTROLLER_LOCK_TIMEOUT);
    let deadline = Instant::now()
        .checked_add(timeout)
        .ok_or("budget lock deadline overflow")?;
    let initial = lock_metadata(path)?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC | libc::O_NONBLOCK)
        .open(path)
        .map_err(err)?;
    let mut retried = false;
    let timed_out = || format!("budget lock acquisition timed out: {}", path.display());
    loop {
        verify_lock_identity(path, &file, &initial)?;
        if retried && Instant::now() >= deadline {
            return Err(timed_out());
        }
        match attempt(&file) {
            Ok(()) => {
                verify_lock_identity(path, &file, &initial)?;
                if retried && Instant::now() >= deadline {
                    return Err(timed_out());
                }
                return Ok(file);
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock && wait.is_none() => {
                return Err("budget session already locked".into());
            }
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::Interrupted
                ) => {}
            Err(e) => return Err(format!("budget lock acquisition failed: {e}")),
        }
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(timed_out());
        }
        std::thread::sleep(LOCK_RETRY_INTERVAL.min(remaining));
        retried = true;
    }
}
fn records(path: &Path) -> Result<Vec<PathBuf>, String> {
    directory(path, false)?;
    let mut result = Vec::new();
    for entry in fs::read_dir(path).map_err(err)? {
        let entry = entry.map_err(err)?;
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| "non-UTF8 budget record")?;
        if name.len() != 25
            || !name.ends_with(".json")
            || !name[..20].bytes().all(|b| b.is_ascii_digit())
        {
            return Err("unexpected file in authoritative budget journal".into());
        }
        result.push(entry.path());
        if result.len() > MAX_RECORDS {
            return Err("budget journal record cap reached".into());
        }
    }
    result.sort();
    for (i, p) in result.iter().enumerate() {
        if p.file_name().and_then(|x| x.to_str()) != Some(format!("{:020}.json", i + 1).as_str()) {
            return Err("budget journal sequence gap".into());
        }
    }
    Ok(result)
}
fn policy(path: &Path) -> Result<(BudgetPolicy, String), String> {
    let data = read_bytes(path)?;
    let p: BudgetPolicy = serde_json::from_slice(&data).map_err(err)?;
    if p.schema != POLICY
        || !safe_id(&p.campaign_id)
        || !(1..=3600).contains(&p.command_timeout_secs)
        || !p.authority_dir.is_absolute()
        || p.allowed_destinations.is_empty()
        || p.allowed_destinations.len() > 32
    {
        return Err("invalid budget policy".into());
    }
    let mut seen = BTreeSet::new();
    for d in &p.allowed_destinations {
        if d.bucket != "setec-astronomy-artifacts"
            || !safe_key(&d.prefix)
            || !seen.insert((d.bucket.clone(), d.prefix.clone()))
        {
            return Err(
                "budget destination must be a unique nonempty canonical-bucket prefix".into(),
            );
        }
    }
    Ok((p, digest(&data)))
}
fn authority(p: &BudgetPolicy, hash: &str) -> Result<(), String> {
    directory(&p.authority_dir, false)?;
    if read_bytes(&initialization_marker(p)?)? != hash.as_bytes() {
        return Err("authority initialization anchor differs or is missing".into());
    }
    let v: Binding = read(&p.authority_dir.join("authority.json"))?;
    if v.schema != "crypto.ml-dataset.budget-authority/v1"
        || v.policy_sha256 != hash
        || v.allocation_sha256 != ZERO
        || v.journal_path != p.authority_dir
    {
        return Err("budget authority binding differs; refusing reset".into());
    }
    Ok(())
}

pub fn initialize(policy_path: &Path) -> Result<Value, String> {
    let (p, hash) = policy(policy_path)?;
    directory(
        p.authority_dir.parent().ok_or("authority parent missing")?,
        false,
    )?;
    // The anchor is outside the worker/authority tree. Re-initializing a lost
    // authority is forbidden; recovery must retain the original durable volume.
    if fs::symlink_metadata(&p.authority_dir).is_ok() {
        return Err("budget authority must be new".into());
    }
    write_new(&initialization_marker(&p)?, hash.as_bytes())?;
    fs::create_dir(&p.authority_dir).map_err(|e| format!("budget authority must be new: {e}"))?;
    sync(p.authority_dir.parent().expect("validated parent"))?;
    write_new(
        &p.authority_dir.join("authority.json"),
        &bytes(&Binding {
            schema: "crypto.ml-dataset.budget-authority/v1".into(),
            policy_sha256: hash.clone(),
            allocation_sha256: ZERO.into(),
            journal_path: p.authority_dir.clone(),
        })?,
    )?;
    write_new(&p.authority_dir.join("controller.lock"), b"")?;
    write_new(
        &p.authority_dir.join("allocation-head.json"),
        &bytes(&Head {
            sequence: 0,
            sha256: ZERO.into(),
        })?,
    )?;
    directory(&p.authority_dir.join("allocations"), true)?;
    directory(&p.authority_dir.join("workers"), true)?;
    Ok(
        json!({"schema":"crypto.ml-dataset.budget-initialized/v1","policy_sha256":hash,"authority_dir":p.authority_dir,"scope":"explicit_s3_commands_and_object_bodies","billing_cap":false}),
    )
}

pub fn allocate(
    policy_path: &Path,
    allocation_id: &str,
    limits: Limits,
    finalization_reserve: Limits,
    out: &Path,
) -> Result<Value, String> {
    let (p, hash) = policy(policy_path)?;
    authority(&p, &hash)?;
    if !safe_id(allocation_id) {
        return Err("invalid allocation ID".into());
    }
    limits.minus(finalization_reserve)?;
    directory(out.parent().ok_or("allocation output lacks parent")?, false)?;
    if fs::symlink_metadata(out).is_ok() {
        return Err("allocation output already exists".into());
    }
    let _lock = controller_locked(&p.authority_dir.join("controller.lock"))?;
    authority(&p, &hash)?;
    let entries = records(&p.authority_dir.join("allocations"))?;
    if entries.len() >= MAX_RECORDS {
        return Err("controller allocation record cap reached".into());
    }
    let mut total = Limits::default();
    let mut previous = ZERO.to_string();
    for (i, path) in entries.iter().enumerate() {
        let data = read_bytes(path)?;
        let grant: Allocation = serde_json::from_slice(&data).map_err(err)?;
        if grant.schema != GRANT
            || grant.policy_sha256 != hash
            || grant.previous_sha256 != previous
            || grant.sequence != i as u64 + 1
        {
            return Err("controller allocation history invalid".into());
        }
        if grant.allocation_id == allocation_id {
            return Err("allocation ID already spent; lost grants are not refunded".into());
        }
        grant.limits.minus(grant.finalization_reserve)?;
        total = total.add(grant.limits)?;
        previous = digest(&data);
    }
    verify_head(
        &p.authority_dir.join("allocation-head.json"),
        entries.len() as u64,
        &previous,
    )?;
    let next = total.add(limits)?;
    if !next.fits(p.limits) {
        return Err("campaign allocation budget exhausted".into());
    }
    let grant = Allocation {
        schema: GRANT.into(),
        policy_sha256: hash,
        allocation_id: allocation_id.into(),
        sequence: entries.len() as u64 + 1,
        previous_sha256: previous,
        limits,
        finalization_reserve,
    };
    let encoded = bytes(&grant)?;
    let grant_hash = digest(&encoded);
    // Charge the entire grant before making it usable or exporting its token.
    write_new(
        &p.authority_dir
            .join("allocations")
            .join(format!("{:020}.json", grant.sequence)),
        &encoded,
    )?;
    replace_head(
        &p.authority_dir.join("allocation-head.json"),
        &Head {
            sequence: grant.sequence,
            sha256: grant_hash.clone(),
        },
    )?;
    let worker = directory(&p.authority_dir.join("workers").join(&grant_hash), true)?;
    write_new(&worker.join("session.lock"), b"")?;
    write_new(
        &worker.join("reservation-head.json"),
        &bytes(&Head {
            sequence: 0,
            sha256: ZERO.into(),
        })?,
    )?;
    directory(&worker.join("reservations"), true)?;
    write_new(out, &encoded)?;
    Ok(
        json!({"schema":"crypto.ml-dataset.budget-allocated/v1","allocation_sha256":grant_hash,"allocation":grant,"campaign_allocated":next,"unused_or_lost_grants_refunded":false,"authority_required":true}),
    )
}

struct State {
    sequence: u64,
    hash: String,
    total: Limits,
    bulk: Limits,
    poisoned: bool,
}
pub struct BudgetSession {
    policy: BudgetPolicy,
    policy_hash: String,
    allocation: Allocation,
    allocation_hash: String,
    journal: PathBuf,
    worker: PathBuf,
    _lock: File,
    state: Mutex<State>,
}
impl BudgetSession {
    pub fn open(
        policy_path: &Path,
        allocation_path: &Path,
        journal_path: &Path,
    ) -> Result<Arc<Self>, String> {
        let (p, hash) = policy(policy_path)?;
        authority(&p, &hash)?;
        let controller = controller_locked(&p.authority_dir.join("controller.lock"))?;
        authority(&p, &hash)?;
        let grant_data = read_bytes(allocation_path)?;
        let grant: Allocation = serde_json::from_slice(&grant_data).map_err(err)?;
        if grant.schema != GRANT
            || grant.policy_sha256 != hash
            || !safe_id(&grant.allocation_id)
            || grant.sequence == 0
        {
            return Err("allocation does not belong to budget policy".into());
        }
        let grant_hash = digest(&grant_data);
        grant.limits.minus(grant.finalization_reserve)?;
        // Verify complete allocation history, including earlier lost grants.
        let allocations = records(&p.authority_dir.join("allocations"))?;
        let mut previous = ZERO.to_string();
        let mut total = Limits::default();
        let mut found = false;
        for (i, path) in allocations.iter().enumerate() {
            let b = read_bytes(path)?;
            let a: Allocation = serde_json::from_slice(&b).map_err(err)?;
            if a.schema != GRANT
                || a.policy_sha256 != hash
                || a.sequence != i as u64 + 1
                || a.previous_sha256 != previous
            {
                return Err("allocation authority history invalid".into());
            }
            a.limits.minus(a.finalization_reserve)?;
            total = total.add(a.limits)?;
            previous = digest(&b);
            if a.sequence == grant.sequence {
                found = b == grant_data;
            }
        }
        verify_head(
            &p.authority_dir.join("allocation-head.json"),
            allocations.len() as u64,
            &previous,
        )?;
        if !found || !total.fits(p.limits) {
            return Err("allocation missing or authority rolled back".into());
        }
        let worker = directory(&p.authority_dir.join("workers").join(&grant_hash), false)?;
        let lock = locked(&worker.join("session.lock"))?;
        // The verified allocation is immutable and this worker is now exclusive.
        // Its binding and reservation replay do not mutate allocation history;
        // unrelated allocations can proceed without waiting for that replay.
        drop(controller);
        let binding = Binding {
            schema: "crypto.ml-dataset.budget-session/v1".into(),
            policy_sha256: hash.clone(),
            allocation_sha256: grant_hash.clone(),
            journal_path: journal_path.into(),
        };
        let authority_binding = worker.join("session.json");
        let journal = if authority_binding.exists() {
            if read::<Binding>(&authority_binding)? != binding {
                return Err(
                    "allocation already bound to another journal; cannot transfer remaining quota"
                        .into(),
                );
            }
            let path = directory(journal_path, false)?;
            if read::<Binding>(&path.join("session.json"))? != binding {
                return Err("worker journal binding missing or rolled back".into());
            }
            path
        } else {
            if !records(&worker.join("reservations"))?.is_empty() {
                return Err("authority session binding lost".into());
            }
            directory(
                journal_path.parent().ok_or("journal parent missing")?,
                false,
            )?;
            fs::create_dir(journal_path)
                .map_err(|e| format!("first worker journal must be new: {e}"))?;
            sync(journal_path.parent().expect("validated parent"))?;
            write_new(&authority_binding, &bytes(&binding)?)?;
            write_new(&journal_path.join("session.json"), &bytes(&binding)?)?;
            directory(&journal_path.join("reservations"), true)?;
            journal_path.into()
        };
        let authoritative = records(&worker.join("reservations"))?;
        let local = records(&journal.join("reservations"))?;
        if authoritative.len() != local.len() {
            return Err("worker journal is missing/rolled back; allocation forfeited".into());
        }
        let mut state = State {
            sequence: 0,
            hash: ZERO.into(),
            total: Limits::default(),
            bulk: Limits::default(),
            poisoned: false,
        };
        for (remote, local) in authoritative.iter().zip(&local) {
            let b = read_bytes(remote)?;
            if read_bytes(local)? != b {
                return Err("worker reservation differs from authority".into());
            }
            let r: Reservation = serde_json::from_slice(&b).map_err(err)?;
            validate_reservation(&p, &grant, &grant_hash, &state, &r)?;
            state = State {
                sequence: r.sequence,
                hash: digest(&b),
                total: r.total,
                bulk: r.bulk,
                poisoned: false,
            };
        }
        verify_head(
            &worker.join("reservation-head.json"),
            state.sequence,
            &state.hash,
        )?;
        Ok(Arc::new(Self {
            policy: p,
            policy_hash: hash,
            allocation: grant,
            allocation_hash: grant_hash,
            journal,
            worker,
            _lock: lock,
            state: Mutex::new(state),
        }))
    }
    pub fn guard(self: &Arc<Self>, phase: Phase) -> S3Guard {
        S3Guard {
            admission: self.clone(),
            phase,
            timeout_secs: self.policy.command_timeout_secs,
        }
    }
    pub fn status(&self) -> Result<Value, String> {
        let state = self.state.lock().map_err(err)?;
        Ok(json!({
        "schema":"crypto.ml-dataset.budget-status/v1","policy_sha256":self.policy_hash,"allocation_sha256":self.allocation_hash,
        "sequence":state.sequence,"reservation_sha256":state.hash,"reserved":state.total,"bulk_reserved":state.bulk,
        "limits":self.allocation.limits,"finalization_reserve":self.allocation.finalization_reserve,"poisoned":state.poisoned,
        "authority_required":true,"billing_cap":false,"scope":"explicit_s3_command_attempts_and_object_body_reservations"}))
    }
}
fn allowed(p: &BudgetPolicy, r: &RequestSpec) -> bool {
    safe_key(if r.operation == Operation::List {
        r.key.trim_end_matches('/')
    } else {
        &r.key
    }) && p.allowed_destinations.iter().any(|d| {
        r.bucket == d.bucket
            && (r.key == d.prefix
                || r.key
                    .strip_prefix(&d.prefix)
                    .is_some_and(|s| s.starts_with('/')))
    })
}
fn validate_reservation(
    p: &BudgetPolicy,
    a: &Allocation,
    hash: &str,
    state: &State,
    r: &Reservation,
) -> Result<(), String> {
    if r.schema != "crypto.ml-dataset.budget-reservation/v1"
        || r.sequence
            != state
                .sequence
                .checked_add(1)
                .ok_or("budget sequence overflow")?
        || r.previous_sha256 != state.hash
        || r.allocation_sha256 != hash
        || !allowed(p, &r.request)
    {
        return Err("invalid budget reservation chain or destination".into());
    }
    let cost = Limits::cost(&r.request)?;
    let total = state.total.add(cost)?;
    let bulk = if r.request.phase == Phase::Finalize {
        state.bulk
    } else {
        state.bulk.add(cost)?
    };
    if r.total != total
        || r.bulk != bulk
        || !total.fits(a.limits)
        || !bulk.fits(a.limits.minus(a.finalization_reserve)?)
    {
        return Err("budget exhausted or reservation totals invalid".into());
    }
    Ok(())
}
impl S3Admission for BudgetSession {
    fn reserve(&self, request: &RequestSpec) -> Result<(), String> {
        let mut state = self.state.lock().map_err(err)?;
        if state.poisoned {
            return Err("budget session poisoned; allocation must be forfeited".into());
        }
        authority(&self.policy, &self.policy_hash)?;
        verify_head(
            &self.worker.join("reservation-head.json"),
            state.sequence,
            &state.hash,
        )?;
        let grant_path = self
            .policy
            .authority_dir
            .join("allocations")
            .join(format!("{:020}.json", self.allocation.sequence));
        if digest(&read_bytes(&grant_path)?) != self.allocation_hash {
            return Err("allocation authority missing or rolled back".into());
        }
        if state.sequence > 0 {
            let name = format!("{:020}.json", state.sequence);
            for root in [&self.worker, &self.journal] {
                if digest(&read_bytes(&root.join("reservations").join(&name))?) != state.hash {
                    return Err("budget high watermark rolled back".into());
                }
            }
        }
        let cost = Limits::cost(request)?;
        let r = Reservation {
            schema: "crypto.ml-dataset.budget-reservation/v1".into(),
            sequence: state
                .sequence
                .checked_add(1)
                .ok_or("budget sequence overflow")?,
            previous_sha256: state.hash.clone(),
            allocation_sha256: self.allocation_hash.clone(),
            request: request.clone(),
            total: state.total.add(cost)?,
            bulk: if request.phase == Phase::Finalize {
                state.bulk
            } else {
                state.bulk.add(cost)?
            },
        };
        validate_reservation(
            &self.policy,
            &self.allocation,
            &self.allocation_hash,
            &state,
            &r,
        )?;
        if r.sequence > MAX_RECORDS as u64 {
            return Err("budget journal record cap reached".into());
        }
        let encoded = bytes(&r)?;
        let name = format!("{:020}.json", r.sequence);
        state.poisoned = true;
        write_new(&self.worker.join("reservations").join(&name), &encoded)?;
        replace_head(
            &self.worker.join("reservation-head.json"),
            &Head {
                sequence: r.sequence,
                sha256: digest(&encoded),
            },
        )?;
        write_new(&self.journal.join("reservations").join(&name), &encoded)?;
        *state = State {
            sequence: r.sequence,
            hash: digest(&encoded),
            total: r.total,
            bulk: r.bulk,
            poisoned: false,
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};
    struct Fixture {
        root: PathBuf,
        policy: PathBuf,
        grant: PathBuf,
        journal: PathBuf,
    }
    impl Fixture {
        fn new(limits: Limits, reserve: Limits) -> Self {
            Self::with_allocation(limits, limits, reserve)
        }
        fn with_allocation(campaign: Limits, limits: Limits, reserve: Limits) -> Self {
            let root = std::env::temp_dir().join(format!(
                "dataset-budget-{}-{}",
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            fs::create_dir(&root).unwrap();
            let policy = root.join("policy.json");
            let grant = root.join("grant.json");
            let journal = root.join("worker-journal");
            let p = BudgetPolicy {
                schema: POLICY.into(),
                campaign_id: "software-fixture".into(),
                authority_dir: root.join("authority"),
                allowed_destinations: vec![Destination {
                    bucket: "setec-astronomy-artifacts".into(),
                    prefix: "research/fixture".into(),
                }],
                limits: campaign,
                command_timeout_secs: 1,
            };
            write_new(&policy, &bytes(&p).unwrap()).unwrap();
            initialize(&policy).unwrap();
            allocate(&policy, "attempt-1", limits, reserve, &grant).unwrap();
            Self {
                root,
                policy,
                grant,
                journal,
            }
        }
        fn open(&self) -> Arc<BudgetSession> {
            BudgetSession::open(&self.policy, &self.grant, &self.journal).unwrap()
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
    fn limits() -> Limits {
        Limits {
            upload_bytes: 100,
            download_bytes: 100,
            put_attempts: 10,
            get_attempts: 10,
            head_attempts: 10,
            list_attempts: 10,
        }
    }
    fn request(operation: Operation, phase: Phase, body: u64) -> RequestSpec {
        RequestSpec {
            operation,
            phase,
            bucket: "setec-astronomy-artifacts".into(),
            key: "research/fixture/object".into(),
            upload_bytes: if operation == Operation::Put { body } else { 0 },
            download_bytes: if operation == Operation::Get { body } else { 0 },
        }
    }
    #[test]
    fn exact_cap_and_failed_attempts_remain_charged() {
        let f = Fixture::new(limits(), Limits::default());
        let s = f.open();
        s.reserve(&request(Operation::Put, Phase::Snapshot, 100))
            .unwrap();
        assert!(s
            .reserve(&request(Operation::Put, Phase::Snapshot, 1))
            .unwrap_err()
            .contains("budget"));
        assert_eq!(s.status().unwrap()["reserved"]["upload_bytes"], 100);
        drop(s);
        let resumed = f.open();
        assert!(resumed
            .reserve(&request(Operation::Put, Phase::Snapshot, 1))
            .is_err());
    }
    #[test]
    fn finalization_reserve_cannot_be_spent_by_snapshot_or_restore() {
        let reserve = Limits {
            upload_bytes: 20,
            put_attempts: 2,
            ..Limits::default()
        };
        let f = Fixture::new(limits(), reserve);
        let s = f.open();
        s.reserve(&request(Operation::Put, Phase::Snapshot, 80))
            .unwrap();
        assert!(s
            .reserve(&request(Operation::Put, Phase::Restore, 1))
            .is_err());
        s.reserve(&request(Operation::Put, Phase::Finalize, 20))
            .unwrap();
        assert!(s
            .reserve(&request(Operation::Put, Phase::Finalize, 1))
            .is_err());
    }
    #[test]
    fn different_phases_share_the_same_ledger() {
        let cap = Limits {
            head_attempts: 1,
            ..limits()
        };
        let f = Fixture::new(cap, Limits::default());
        let s = f.open();
        s.guard(Phase::Snapshot)
            .admission
            .reserve(&request(Operation::Head, Phase::Snapshot, 0))
            .unwrap();
        assert!(s
            .guard(Phase::Restore)
            .admission
            .reserve(&request(Operation::Head, Phase::Restore, 0))
            .is_err());
    }
    #[test]
    fn concurrent_last_credit_is_reserved_once() {
        let f = Fixture::new(
            Limits {
                head_attempts: 1,
                ..limits()
            },
            Limits::default(),
        );
        let s = f.open();
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let s = s.clone();
                std::thread::spawn(move || {
                    s.reserve(&request(Operation::Head, Phase::Snapshot, 0))
                        .is_ok()
                })
            })
            .collect();
        assert_eq!(
            handles
                .into_iter()
                .filter_map(|h| h.join().ok())
                .filter(|x| *x)
                .count(),
            1
        );
    }
    #[test]
    fn controller_charges_unused_lost_grants_in_full() {
        let f = Fixture::new(limits(), Limits::default());
        assert!(allocate(
            &f.policy,
            "attempt-2",
            limits(),
            Limits::default(),
            &f.root.join("second.json")
        )
        .unwrap_err()
        .contains("exhausted"));
        assert!(allocate(
            &f.policy,
            "attempt-1",
            Limits::default(),
            Limits::default(),
            &f.root.join("same.json")
        )
        .unwrap_err()
        .contains("already spent"));
    }
    #[test]
    fn missing_authority_does_not_reinitialize() {
        let f = Fixture::new(limits(), Limits::default());
        fs::rename(f.root.join("authority"), f.root.join("lost-authority")).unwrap();
        assert!(BudgetSession::open(&f.policy, &f.grant, &f.journal).is_err());
        assert!(!f.root.join("authority").exists());
        assert!(initialize(&f.policy).is_err());
    }
    #[test]
    fn initialize_never_resets_existing_authority() {
        let f = Fixture::new(limits(), Limits::default());
        assert!(initialize(&f.policy).is_err());
    }
    #[test]
    fn lost_worker_journal_forfeits_allocation() {
        let f = Fixture::new(limits(), Limits::default());
        let s = f.open();
        s.reserve(&request(Operation::Head, Phase::Snapshot, 0))
            .unwrap();
        drop(s);
        fs::remove_dir_all(&f.journal).unwrap();
        assert!(BudgetSession::open(&f.policy, &f.grant, &f.journal).is_err());
        assert!(!f.journal.exists());
    }
    #[test]
    fn rolled_back_worker_and_authority_records_are_rejected() {
        let f = Fixture::new(limits(), Limits::default());
        let s = f.open();
        s.reserve(&request(Operation::Head, Phase::Snapshot, 0))
            .unwrap();
        fs::remove_file(f.journal.join("reservations/00000000000000000001.json")).unwrap();
        assert!(s
            .reserve(&request(Operation::Head, Phase::Snapshot, 0))
            .is_err());
        drop(s);
        assert!(BudgetSession::open(&f.policy, &f.grant, &f.journal).is_err());
    }
    #[test]
    fn authority_committed_local_failure_poisoned_and_no_retry() {
        let f = Fixture::new(limits(), Limits::default());
        let s = f.open();
        let local = f.journal.join("reservations/00000000000000000001.json");
        write_new(&local, b"torn").unwrap();
        assert!(s
            .reserve(&request(Operation::Head, Phase::Snapshot, 0))
            .is_err());
        assert!(s
            .worker
            .join("reservations/00000000000000000001.json")
            .exists());
        assert_eq!(s.status().unwrap()["poisoned"], true);
        assert!(s
            .reserve(&request(Operation::Head, Phase::Snapshot, 0))
            .unwrap_err()
            .contains("poisoned"));
        drop(s);
        assert!(BudgetSession::open(&f.policy, &f.grant, &f.journal).is_err());
    }
    #[test]
    fn allocation_replay_on_new_worker_path_is_forbidden() {
        let f = Fixture::new(limits(), Limits::default());
        drop(f.open());
        assert!(BudgetSession::open(&f.policy, &f.grant, &f.root.join("new-journal")).is_err());
    }
    #[test]
    fn allocation_authority_rollback_is_rejected() {
        let f = Fixture::new(limits(), Limits::default());
        let s = f.open();
        fs::remove_file(
            f.root
                .join("authority/allocations/00000000000000000001.json"),
        )
        .unwrap();
        assert!(s
            .reserve(&request(Operation::Head, Phase::Snapshot, 0))
            .is_err());
        drop(s);
        assert!(BudgetSession::open(&f.policy, &f.grant, &f.journal).is_err());
        assert!(allocate(
            &f.policy,
            "replacement",
            limits(),
            Limits::default(),
            &f.root.join("replacement.json")
        )
        .is_err());
    }
    #[test]
    fn both_worker_and_reservation_history_rollback_cannot_reset_high_watermark() {
        let f = Fixture::new(limits(), Limits::default());
        let s = f.open();
        s.reserve(&request(Operation::Head, Phase::Snapshot, 0))
            .unwrap();
        let worker = s.worker.clone();
        drop(s);
        fs::remove_file(worker.join("reservations/00000000000000000001.json")).unwrap();
        fs::remove_file(f.journal.join("reservations/00000000000000000001.json")).unwrap();
        assert!(BudgetSession::open(&f.policy, &f.grant, &f.journal).is_err());
    }
    #[test]
    fn destination_boundary_and_operation_bytes_are_checked() {
        let f = Fixture::new(limits(), Limits::default());
        let s = f.open();
        let mut r = request(Operation::Head, Phase::Snapshot, 0);
        r.key = "research/fixture-escape/object".into();
        assert!(s.reserve(&r).is_err());
        r.key = "research/fixture/../other".into();
        assert!(s.reserve(&r).is_err());
        r = request(Operation::Head, Phase::Snapshot, 0);
        r.upload_bytes = 1;
        assert!(s.reserve(&r).is_err());
        assert_eq!(s.status().unwrap()["sequence"], 0);
    }
    #[test]
    fn counters_fail_on_overflow() {
        let a = Limits {
            upload_bytes: u64::MAX,
            ..Limits::default()
        };
        assert!(a
            .add(Limits {
                upload_bytes: 1,
                ..Limits::default()
            })
            .is_err());
    }
    #[test]
    fn concurrent_session_open_is_excluded() {
        let f = Fixture::new(limits(), Limits::default());
        let _s = f.open();
        assert!(BudgetSession::open(&f.policy, &f.grant, &f.journal)
            .err()
            .unwrap()
            .contains("budget session already locked"));
    }
    #[cfg(unix)]
    #[test]
    fn independent_allocations_wait_for_controller_then_both_open() {
        use std::sync::{mpsc, Barrier};
        let f =
            Fixture::with_allocation(limits().add(limits()).unwrap(), limits(), Limits::default());
        let other_grant = f.root.join("other-grant.json");
        let other_journal = f.root.join("other-journal");
        allocate(
            &f.policy,
            "attempt-2",
            limits(),
            Limits::default(),
            &other_grant,
        )
        .unwrap();
        let controller_path = f.root.join("authority/controller.lock");
        let controller = locked(&controller_path).unwrap();
        let barrier = Barrier::new(3);
        let (tx, rx) = mpsc::channel();
        std::thread::scope(|scope| {
            for (grant, journal) in [(&f.grant, &f.journal), (&other_grant, &other_journal)] {
                let tx = tx.clone();
                let policy = &f.policy;
                let barrier = &barrier;
                scope.spawn(move || {
                    barrier.wait();
                    assert!(tx.send(BudgetSession::open(policy, grant, journal)).is_ok());
                });
            }
            barrier.wait();
            // Neither waiter may fail fast or create a journal while the
            // controller is still owned. This is synchronization, not timing evidence.
            let while_locked = rx.recv_timeout(Duration::from_millis(40));
            let no_journals = !f.journal.exists() && !other_journal.exists();
            drop(controller);
            assert!(matches!(while_locked, Err(mpsc::RecvTimeoutError::Timeout)));
            assert!(no_journals);
            let a = rx.recv_timeout(Duration::from_secs(5)).unwrap().unwrap();
            let b = rx.recv_timeout(Duration::from_secs(5)).unwrap().unwrap();
            assert_ne!(a.allocation_hash, b.allocation_hash);
            assert!(f.journal.is_dir());
            assert!(other_journal.is_dir());
            // Keeping both sessions alive must not retain the global lock.
            let _controller = locked(&controller_path).unwrap();
            for session in [a, b] {
                session
                    .reserve(&request(Operation::Head, Phase::Snapshot, 0))
                    .unwrap();
                assert_eq!(session.status().unwrap()["sequence"], 1);
            }
        });
    }
    #[cfg(unix)]
    #[test]
    fn allocation_issuance_waits_for_controller_without_spending_early() {
        use std::sync::{mpsc, Barrier};
        let f =
            Fixture::with_allocation(limits().add(limits()).unwrap(), limits(), Limits::default());
        let controller = locked(&f.root.join("authority/controller.lock")).unwrap();
        let out = f.root.join("second-grant.json");
        let barrier = Barrier::new(2);
        let (tx, rx) = mpsc::channel();
        std::thread::scope(|scope| {
            let barrier = &barrier;
            let policy = &f.policy;
            let out = &out;
            scope.spawn(move || {
                barrier.wait();
                tx.send(allocate(
                    policy,
                    "attempt-2",
                    limits(),
                    Limits::default(),
                    out,
                ))
                .unwrap();
            });
            barrier.wait();
            let while_locked = rx.recv_timeout(Duration::from_millis(40));
            let count = records(&f.root.join("authority/allocations"))
                .unwrap()
                .len();
            let no_export = !out.exists();
            drop(controller);
            assert!(matches!(while_locked, Err(mpsc::RecvTimeoutError::Timeout)));
            assert_eq!(count, 1);
            assert!(no_export);
            let result = rx.recv_timeout(Duration::from_secs(5)).unwrap().unwrap();
            assert_eq!(result["allocation"]["sequence"], 2);
            assert_eq!(
                records(&f.root.join("authority/allocations"))
                    .unwrap()
                    .len(),
                2
            );
        });
    }
    #[cfg(unix)]
    #[test]
    fn session_lock_contention_is_not_retried() {
        let f = Fixture::new(limits(), Limits::default());
        let session = f.open();
        let mut attempts = 0;
        let failure = locked_with(&session.worker.join("session.lock"), None, |file| {
            attempts += 1;
            try_lock(file)
        })
        .unwrap_err();
        assert!(failure.contains("budget session already locked"));
        assert_eq!(attempts, 1);
        assert_eq!(session.status().unwrap()["sequence"], 0);
    }
    #[cfg(unix)]
    #[test]
    fn controller_lock_timeout_preserves_existing_owner() {
        let f = Fixture::new(limits(), Limits::default());
        let path = f.root.join("authority/controller.lock");
        let owner = locked(&path).unwrap();
        let mut attempts = 0;
        let failure = locked_with(&path, Some(Duration::from_millis(30)), |file| {
            attempts += 1;
            try_lock(file)
        })
        .unwrap_err();
        assert!(failure.contains("timed out"));
        assert!(attempts >= 1);
        assert!(locked(&path).is_err());
        drop(owner);
        assert!(locked(&path).is_ok());
        assert_eq!(
            records(&f.root.join("authority/allocations"))
                .unwrap()
                .len(),
            1
        );
    }
    #[cfg(unix)]
    #[test]
    fn controller_retries_interruption_and_contention_but_preserves_other_errors() {
        let f = Fixture::new(limits(), Limits::default());
        let path = f.root.join("authority/controller.lock");
        let mut attempts = 0;
        let lock = locked_with(&path, Some(Duration::from_secs(1)), |file| {
            attempts += 1;
            match attempts {
                1 => Err(std::io::Error::from_raw_os_error(libc::EINTR)),
                2 => Err(std::io::Error::from_raw_os_error(libc::EWOULDBLOCK)),
                _ => try_lock(file),
            }
        })
        .unwrap();
        assert_eq!(attempts, 3);
        assert!(locked(&path).is_err());
        drop(lock);
        for code in [
            libc::EBADF,
            libc::EINVAL,
            libc::ENOLCK,
            libc::EIO,
            libc::EACCES,
        ] {
            let mut attempts = 0;
            let failure = locked_with(&path, Some(Duration::from_secs(1)), |_| {
                attempts += 1;
                Err(std::io::Error::from_raw_os_error(code))
            })
            .unwrap_err();
            assert!(failure.contains("acquisition failed"));
            assert!(failure.contains(&std::io::Error::from_raw_os_error(code).to_string()));
            assert_eq!(attempts, 1);
            assert!(locked(&path).is_ok());
        }
    }
    #[cfg(unix)]
    #[test]
    fn repeated_interruption_obeys_the_same_deadline() {
        let f = Fixture::new(limits(), Limits::default());
        let path = f.root.join("authority/controller.lock");
        let mut attempts = 0;
        let failure = locked_with(&path, Some(Duration::from_millis(20)), |_| {
            attempts += 1;
            Err(std::io::Error::from_raw_os_error(libc::EINTR))
        })
        .unwrap_err();
        assert!(failure.contains("timed out"));
        assert!(attempts >= 1);
        assert!(locked(&path).is_ok());
    }
    #[cfg(unix)]
    #[test]
    fn replaced_lock_path_is_rejected_after_wait_or_success() {
        for replace_after_success in [false, true] {
            let f = Fixture::new(limits(), Limits::default());
            let path = f.root.join("authority/controller.lock");
            let mut attempts = 0;
            let failure = locked_with(&path, Some(Duration::from_secs(1)), |file| {
                attempts += 1;
                fs::rename(&path, f.root.join("old-controller.lock")).unwrap();
                write_new(&path, b"").unwrap();
                if replace_after_success {
                    try_lock(file)
                } else {
                    Err(std::io::Error::from_raw_os_error(libc::EWOULDBLOCK))
                }
            })
            .unwrap_err();
            assert!(failure.contains("identity changed"));
            assert_eq!(attempts, 1);
            assert!(locked(&path).is_ok());
            assert!(locked(&f.root.join("old-controller.lock")).is_ok());
        }
    }
    #[cfg(unix)]
    #[test]
    fn missing_or_nonregular_lock_fails_before_attempt() {
        use std::os::unix::fs::symlink;
        let f = Fixture::new(limits(), Limits::default());
        let absent = f.root.join("absent.lock");
        let linked = f.root.join("linked.lock");
        symlink(f.root.join("authority/controller.lock"), &linked).unwrap();
        for path in [absent, linked, f.root.join("authority")] {
            let mut attempts = 0;
            assert!(locked_with(&path, Some(Duration::from_secs(1)), |_| {
                attempts += 1;
                Ok(())
            })
            .is_err());
            assert_eq!(attempts, 0);
        }
    }
    #[cfg(unix)]
    #[test]
    fn symlinked_authority_is_not_accepted() {
        use std::os::unix::fs::symlink;
        let f = Fixture::new(limits(), Limits::default());
        let original = f.root.join("authority");
        let moved = f.root.join("moved");
        fs::rename(&original, &moved).unwrap();
        symlink(&moved, &original).unwrap();
        assert!(BudgetSession::open(&f.policy, &f.grant, &f.journal).is_err());
    }
}
