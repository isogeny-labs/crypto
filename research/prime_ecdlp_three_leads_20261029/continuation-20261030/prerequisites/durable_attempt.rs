//! Local attempt-record primitive for a future native supervisor.
//!
//! This module launches nothing and authorizes no experiment. Its callers must
//! enforce checkpoint/protocol/runtime gates. Records are producer assertions,
//! not authenticated execution proofs. Source/request identities are supplied
//! labels; this module does not compute or attest them.
//!
//! On a filesystem supporting directory fsync and hard links, successful writes
//! have requested file/directory synchronization. A crash leaves either an
//! inspectable terminal record or an unfinished/invalid record, never inferred
//! zero work. Errors can occur after a terminal link becomes visible; inspection
//! reports observed bytes, not whether the writer received a durability ack.
//! No remote checkpoint, power-loss test, hostile-directory or resume guarantee
//! is provided. The fixed records deliberately say work counts are unavailable.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

const MAX_RECORD_BYTES: u64 = 512;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Binding {
    request_sha256: String,
    source_revision: String,
}

impl Binding {
    pub fn new(request_sha256: &str, source_revision: &str) -> io::Result<Self> {
        let canonical = |s: &str, len| {
            s.len() == len
                && s.bytes()
                    .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
        };
        if !canonical(request_sha256, 64) || !canonical(source_revision, 40) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "noncanonical binding",
            ));
        }
        Ok(Self {
            request_sha256: request_sha256.into(),
            source_revision: source_revision.into(),
        })
    }

    fn record(&self, schema: &str, status: &str) -> String {
        format!(
            "{schema}\nrequest_sha256={}\nsource_revision={}\nstatus={status}\nwork_counts=unavailable\n",
            self.request_sha256, self.source_revision
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Completed,
    Refused,
    Failed,
    Capped,
}

impl Outcome {
    fn label(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::Refused => "refused",
            Self::Failed => "failed",
            Self::Capped => "capped",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Observed {
    Unfinished,
    Terminal(Outcome),
}

pub struct Attempt {
    directory: PathBuf,
    binding: Binding,
}

fn sync_directory(path: &Path) -> io::Result<()> {
    File::open(path)?.sync_all()
}

fn write_new_synced(path: &Path, record: &str) -> io::Result<()> {
    if record.len() as u64 > MAX_RECORD_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "oversize record",
        ));
    }
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(record.as_bytes())?;
    file.sync_all()
}

fn read_record(path: &Path) -> io::Result<String> {
    if !fs::symlink_metadata(path)?.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "not a regular record",
        ));
    }
    let mut bytes = Vec::new();
    File::open(path)?
        .take(MAX_RECORD_BYTES + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > MAX_RECORD_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "oversize record",
        ));
    }
    String::from_utf8(bytes)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "non-UTF8 record"))
}

impl Attempt {
    /// Requires an existing parent directory. An existing attempt is never
    /// overwritten. Partial creation errors leave their evidence in place.
    pub fn begin(directory: &Path, binding: Binding) -> io::Result<Self> {
        fs::create_dir(directory)?;
        let parent = directory
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        sync_directory(parent)?;
        write_new_synced(
            &directory.join("attempt"),
            &binding.record("PKM_ATTEMPT_V1", "started"),
        )?;
        sync_directory(directory)?;
        Ok(Self {
            directory: directory.into(),
            binding,
        })
    }

    /// The pending file is synced before atomically publishing a nonoverwriting
    /// hard link. A pending-only record remains unfinished. Retrying a finalizer
    /// never silently overwrites either pending or terminal evidence.
    pub fn finish(&self, outcome: Outcome) -> io::Result<()> {
        let pending = self.directory.join("outcome.pending");
        write_new_synced(
            &pending,
            &self.binding.record("PKM_OUTCOME_V1", outcome.label()),
        )?;
        fs::hard_link(&pending, self.directory.join("outcome"))?;
        sync_directory(&self.directory)
    }
}

/// Fail closed on a malformed/mismatched record. An absent terminal file has no
/// cost estimate and no inferred completion; a pending file is not terminal.
pub fn inspect(directory: &Path, expected: &Binding) -> io::Result<Observed> {
    let start = read_record(&directory.join("attempt"))?;
    if start != expected.record("PKM_ATTEMPT_V1", "started") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "attempt binding mismatch",
        ));
    }
    let terminal = match read_record(&directory.join("outcome")) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Observed::Unfinished),
        other => other?,
    };
    for outcome in [
        Outcome::Completed,
        Outcome::Refused,
        Outcome::Failed,
        Outcome::Capped,
    ] {
        if terminal == expected.record("PKM_OUTCOME_V1", outcome.label()) {
            return Ok(Observed::Terminal(outcome));
        }
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "invalid outcome or binding",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    struct Scratch(PathBuf);
    impl Scratch {
        fn new() -> Self {
            static NEXT: AtomicU64 = AtomicU64::new(0);
            for _ in 0..1000 {
                let path = std::env::temp_dir().join(format!(
                    "pkm-attempt-{}-{}",
                    std::process::id(),
                    NEXT.fetch_add(1, Ordering::Relaxed)
                ));
                match fs::create_dir(&path) {
                    Ok(()) => return Self(path),
                    Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
                    Err(e) => panic!("scratch {path:?}: {e}"),
                }
            }
            panic!("scratch namespace exhausted")
        }
    }
    impl Drop for Scratch {
        fn drop(&mut self) {
            if !std::thread::panicking() {
                let _ = fs::remove_dir_all(&self.0);
            }
        }
    }
    fn binding() -> Binding {
        Binding::new(&"a".repeat(64), &"b".repeat(40)).unwrap()
    }

    #[test]
    fn abandoned_or_pending_only_attempt_is_unfinished_with_unknown_work() {
        let scratch = Scratch::new();
        let path = scratch.0.join("one");
        let b = binding();
        let attempt = Attempt::begin(&path, b.clone()).unwrap();
        drop(attempt);
        assert_eq!(inspect(&path, &b).unwrap(), Observed::Unfinished);
        // Even a complete pending payload is not a published terminal record.
        write_new_synced(
            &path.join("outcome.pending"),
            &b.record("PKM_OUTCOME_V1", "completed"),
        )
        .unwrap();
        assert_eq!(inspect(&path, &b).unwrap(), Observed::Unfinished);
        assert!(read_record(&path.join("attempt"))
            .unwrap()
            .contains("work_counts=unavailable"));
    }

    #[test]
    fn all_terminal_kinds_survive_close_and_reopen() {
        for outcome in [
            Outcome::Completed,
            Outcome::Refused,
            Outcome::Failed,
            Outcome::Capped,
        ] {
            let scratch = Scratch::new();
            let path = scratch.0.join("one");
            let b = binding();
            {
                let attempt = Attempt::begin(&path, b.clone()).unwrap();
                attempt.finish(outcome).unwrap();
            }
            assert_eq!(inspect(&path, &b).unwrap(), Observed::Terminal(outcome));
        }
    }

    #[test]
    fn duplicate_attempt_or_terminal_never_overwrites_evidence() {
        let scratch = Scratch::new();
        let path = scratch.0.join("one");
        let b = binding();
        let attempt = Attempt::begin(&path, b.clone()).unwrap();
        let original = fs::read(path.join("attempt")).unwrap();
        assert!(
            matches!(Attempt::begin(&path, b.clone()), Err(e) if e.kind()==io::ErrorKind::AlreadyExists)
        );
        attempt.finish(Outcome::Failed).unwrap();
        let terminal = fs::read(path.join("outcome")).unwrap();
        assert_eq!(
            attempt.finish(Outcome::Completed).unwrap_err().kind(),
            io::ErrorKind::AlreadyExists
        );
        assert_eq!(fs::read(path.join("attempt")).unwrap(), original);
        assert_eq!(fs::read(path.join("outcome")).unwrap(), terminal);
        assert_eq!(
            inspect(&path, &b).unwrap(),
            Observed::Terminal(Outcome::Failed)
        );

        // Exercise publication collision itself, rather than only the pending
        // create_new guard: an existing terminal must not be replaced.
        let other = scratch.0.join("two");
        let attempt = Attempt::begin(&other, b.clone()).unwrap();
        let existing = b.record("PKM_OUTCOME_V1", "refused");
        fs::write(other.join("outcome"), &existing).unwrap();
        assert_eq!(
            attempt.finish(Outcome::Completed).unwrap_err().kind(),
            io::ErrorKind::AlreadyExists
        );
        assert_eq!(fs::read_to_string(other.join("outcome")).unwrap(), existing);
        assert_eq!(
            inspect(&other, &b).unwrap(),
            Observed::Terminal(Outcome::Refused)
        );
    }

    #[test]
    fn malformed_torn_or_transplanted_records_are_rejected() {
        let scratch = Scratch::new();
        let path = scratch.0.join("one");
        let b = binding();
        let attempt = Attempt::begin(&path, b.clone()).unwrap();
        attempt.finish(Outcome::Completed).unwrap();
        let wrong = Binding::new(&"c".repeat(64), &"b".repeat(40)).unwrap();
        assert_eq!(
            inspect(&path, &wrong).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        fs::write(path.join("outcome"), b"PKM_OUTCOME_V1\ntruncated").unwrap();
        assert_eq!(
            inspect(&path, &b).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        fs::write(path.join("outcome"), vec![b'x'; 513]).unwrap();
        assert_eq!(
            inspect(&path, &b).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        fs::write(path.join("attempt"), b"truncated").unwrap();
        assert_eq!(
            inspect(&path, &b).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn bad_bindings_and_io_failures_are_errors_not_terminal_success() {
        assert!(Binding::new(&"A".repeat(64), &"b".repeat(40)).is_err());
        assert!(Binding::new(&"a".repeat(63), &"b".repeat(40)).is_err());
        let scratch = Scratch::new();
        let blocked = scratch.0.join("not-directory");
        fs::write(&blocked, b"public test fixture").unwrap();
        assert!(Attempt::begin(&blocked.join("one"), binding()).is_err());
        let path = scratch.0.join("two");
        let attempt = Attempt::begin(&path, binding()).unwrap();
        fs::create_dir(path.join("outcome.pending")).unwrap();
        assert!(attempt.finish(Outcome::Completed).is_err());
        assert_eq!(inspect(&path, &binding()).unwrap(), Observed::Unfinished);
    }
}
