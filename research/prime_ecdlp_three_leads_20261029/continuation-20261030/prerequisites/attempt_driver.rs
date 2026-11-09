//! Local callback bookkeeping, not a scientific launcher or runtime gate.
//!
//! Creates the attempt before the supplied gate, and invokes work only after
//! that gate returns Permit. Supplied callbacks/identity labels are untrusted;
//! Permit is not checkpoint/protocol/runtime qualification. This module starts
//! no process, implements no deadline or remote persistence, and prices no work.
//! A panic/abort/interruption leaves the attempt unfinished. A parent supervisor
//! must still observe/reap children and qualify interruption/resume separately.

#[path = "durable_attempt.rs"]
pub mod durable_attempt;

use durable_attempt::{Attempt, Binding, Observed, Outcome};
use std::io;
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub enum Gate {
    Permit,
    Refuse,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Stage {
    Begin,
    Gate,
    Work,
    Publish,
}

/// Retains both the callback failure and any failure to publish its outcome.
/// A visible terminal file need not imply the writer received a sync ack.
#[derive(Debug)]
pub struct DriverError {
    pub stage: Stage,
    pub primary: io::Error,
    pub publication: Option<io::Error>,
}

fn failure(attempt: &Attempt, stage: Stage, primary: io::Error) -> DriverError {
    DriverError {
        stage,
        primary,
        publication: attempt.finish(Outcome::Failed).err(),
    }
}

/// A single local attempt with no automatic retry and no inferred counts.
/// If begin fails neither callback runs. A gate refusal never invokes work.
/// Callback errors are retained even if recording Failed also fails.
/// Outcome::Completed is only a callback label, not checked curve correctness.
pub fn run_attempt(
    directory: &Path,
    binding: Binding,
    gate: impl FnOnce() -> io::Result<Gate>,
    work: impl FnOnce() -> io::Result<Outcome>,
) -> Result<Outcome, DriverError> {
    let attempt = Attempt::begin(directory, binding).map_err(|primary| DriverError {
        stage: Stage::Begin,
        primary,
        publication: None,
    })?;
    let outcome = match gate() {
        Ok(Gate::Refuse) => Outcome::Refused,
        Err(error) => return Err(failure(&attempt, Stage::Gate, error)),
        Ok(Gate::Permit) => match work() {
            Ok(outcome) => outcome,
            Err(error) => return Err(failure(&attempt, Stage::Work, error)),
        },
    };
    attempt.finish(outcome).map_err(|primary| DriverError {
        stage: Stage::Publish,
        primary,
        publication: None,
    })?;
    Ok(outcome)
}

/// Read-only reconciliation of explicitly named attempts in one local root.
/// It never suppresses unfinished records, mutates an old attempt, resumes work,
/// returns an uninterrupted clock, or treats missing counts as zero.
#[derive(Debug, PartialEq, Eq)]
pub struct Inventory {
    pub attempts: Vec<(String, Observed)>,
    pub work_counts: Option<u64>,
    pub uninterrupted_wall_ns: Option<u64>,
}

pub fn inventory(root: &Path, ids: &[&str], binding: &Binding) -> io::Result<Inventory> {
    let mut attempts = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for id in ids {
        if id.is_empty()
            || id.len() > 64
            || !id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
            || !seen.insert(*id)
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid or duplicate attempt id",
            ));
        }
        attempts.push((
            (*id).into(),
            durable_attempt::inspect(&root.join(id), binding)?,
        ));
    }
    Ok(Inventory {
        attempts,
        work_counts: None,
        uninterrupted_wall_ns: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    struct Scratch(PathBuf);
    impl Scratch {
        fn new() -> Self {
            static NEXT: AtomicU64 = AtomicU64::new(0);
            for _ in 0..1000 {
                let p = std::env::temp_dir().join(format!(
                    "pkm-driver-{}-{}",
                    std::process::id(),
                    NEXT.fetch_add(1, Ordering::Relaxed)
                ));
                match fs::create_dir(&p) {
                    Ok(()) => return Self(p),
                    Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
                    Err(e) => panic!("scratch: {e}"),
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
    fn refusal_work() -> io::Result<Outcome> {
        panic!("work must not run")
    }

    #[test]
    fn refusal_sees_existing_record_and_never_runs_work() {
        let root = Scratch::new();
        let path = root.0.join("one");
        assert_eq!(
            run_attempt(
                &path,
                binding(),
                || {
                    assert_eq!(
                        durable_attempt::inspect(&path, &binding())?,
                        Observed::Unfinished
                    );
                    Ok(Gate::Refuse)
                },
                refusal_work,
            )
            .unwrap(),
            Outcome::Refused
        );
        assert_eq!(
            durable_attempt::inspect(&path, &binding()).unwrap(),
            Observed::Terminal(Outcome::Refused)
        );
    }

    #[test]
    fn failed_begin_does_not_invoke_either_callback() {
        let root = Scratch::new();
        let path = root.0.join("one");
        let _attempt = Attempt::begin(&path, binding()).unwrap();
        let before = fs::read(path.join("attempt")).unwrap();
        let e = run_attempt(
            &path,
            binding(),
            || panic!("gate must not run"),
            refusal_work,
        )
        .unwrap_err();
        assert_eq!(e.stage, Stage::Begin);
        assert_eq!(fs::read(path.join("attempt")).unwrap(), before);
        assert!(!path.join("outcome").exists());
    }

    #[test]
    fn gate_and_work_errors_record_failure_without_losing_primary_error() {
        for fail_gate in [true, false] {
            let root = Scratch::new();
            let path = root.0.join("one");
            let e = run_attempt(
                &path,
                binding(),
                || {
                    if fail_gate {
                        Err(io::Error::other("gate unavailable"))
                    } else {
                        Ok(Gate::Permit)
                    }
                },
                || Err(io::Error::other("work failure")),
            )
            .unwrap_err();
            assert_eq!(e.stage, if fail_gate { Stage::Gate } else { Stage::Work });
            assert_eq!(
                e.primary.to_string(),
                if fail_gate {
                    "gate unavailable"
                } else {
                    "work failure"
                }
            );
            assert!(e.publication.is_none());
            assert_eq!(
                durable_attempt::inspect(&path, &binding()).unwrap(),
                Observed::Terminal(Outcome::Failed)
            );
        }
    }

    #[test]
    fn both_work_and_publication_errors_survive_collision() {
        let root = Scratch::new();
        let path = root.0.join("one");
        let e = run_attempt(
            &path,
            binding(),
            || Ok(Gate::Permit),
            || {
                fs::write(path.join("outcome.pending"), b"retained prior pending")?;
                Err(io::Error::other("work failed"))
            },
        )
        .unwrap_err();
        assert_eq!(e.stage, Stage::Work);
        assert_eq!(e.primary.to_string(), "work failed");
        assert_eq!(e.publication.unwrap().kind(), io::ErrorKind::AlreadyExists);
        assert_eq!(
            fs::read(path.join("outcome.pending")).unwrap(),
            b"retained prior pending"
        );
        assert_eq!(
            durable_attempt::inspect(&path, &binding()).unwrap(),
            Observed::Unfinished
        );
    }

    #[test]
    fn permission_precedes_work_and_all_terminal_labels_are_retained() {
        for outcome in [
            Outcome::Completed,
            Outcome::Failed,
            Outcome::Capped,
            Outcome::Refused,
        ] {
            let root = Scratch::new();
            let path = root.0.join("one");
            let permitted = Cell::new(false);
            assert_eq!(
                run_attempt(
                    &path,
                    binding(),
                    || {
                        permitted.set(true);
                        Ok(Gate::Permit)
                    },
                    || {
                        assert!(permitted.get());
                        Ok(outcome)
                    }
                )
                .unwrap(),
                outcome
            );
            assert_eq!(
                durable_attempt::inspect(&path, &binding()).unwrap(),
                Observed::Terminal(outcome)
            );
        }
    }

    #[test]
    fn interrupted_work_survives_inventory_and_fresh_retry_has_no_continuous_clock() {
        let root = Scratch::new();
        let path = root.0.join("old");
        assert!(std::panic::catch_unwind(|| {
            let _ = run_attempt(
                &path,
                binding(),
                || Ok(Gate::Permit),
                || panic!("simulated unwind"),
            );
        })
        .is_err());
        let before = fs::read(path.join("attempt")).unwrap();
        run_attempt(
            &root.0.join("new"),
            binding(),
            || Ok(Gate::Permit),
            || Ok(Outcome::Completed),
        )
        .unwrap();
        let items = inventory(&root.0, &["old", "new"], &binding()).unwrap();
        assert_eq!(
            items.attempts,
            vec![
                ("old".into(), Observed::Unfinished),
                ("new".into(), Observed::Terminal(Outcome::Completed))
            ]
        );
        assert_eq!(items.work_counts, None);
        assert_eq!(items.uninterrupted_wall_ns, None);
        assert_eq!(fs::read(path.join("attempt")).unwrap(), before);
        for ids in [vec!["old", "old"], vec!["../old"], vec!["missing"]] {
            assert!(inventory(&root.0, &ids, &binding()).is_err());
        }
        let wrong = Binding::new(&"c".repeat(64), &"b".repeat(40)).unwrap();
        assert!(inventory(&root.0, &["old"], &wrong).is_err());
    }
}
