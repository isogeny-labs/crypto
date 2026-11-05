//! Local root-child observation and attempt publication prerequisite.
//!
//! Wraps an ALREADY spawned child; this library launches nothing. The caller
//! must still create the attempt before its genuine gates/launch, bind source,
//! command and binary identities, qualify its runtime and manage descendants.
//! Here, a stop request, an absent status or an observation error is not a reap.
//! Only an observed root exit permits sidecar access or outcome publication.
//!
//! Exit zero is not scalar/witness verification. A successful kill request may
//! race a natural exit, so Capped means deadline stop requested, not a proven
//! causal exit reason. No exit timestamp, cost, continuous clock, process-tree
//! cleanup, remote durability or interruption/resume guarantee is provided.
//! The owner retains this handle on errors and must arrange eventual cleanup.
//! Like std::process::Child, dropping this wrapper does not kill/reap a child.

#[path = "attempt_driver.rs"]
pub mod attempt_driver;

use attempt_driver::durable_attempt::{Attempt, Outcome};
use std::io;
use std::process::{Child, ExitStatus};
use std::time::Instant;

/// Fields are private: an instance is produced only after std returns an exit.
#[derive(Clone, Debug)]
pub struct ObservedExit {
    status: ExitStatus,
    deadline_stop_requested: bool,
}

impl ObservedExit {
    pub fn code(&self) -> Option<i32> {
        self.status.code()
    }

    pub fn deadline_stop_requested(&self) -> bool {
        self.deadline_stop_requested
    }

    pub fn outcome(&self) -> Outcome {
        if self.deadline_stop_requested {
            Outcome::Capped
        } else if self.status.success() {
            Outcome::Completed
        } else {
            Outcome::Failed
        }
    }
}

pub struct ChildObservation {
    child: Child,
    stop_requested: bool,
    observed: Option<ObservedExit>,
}

impl ChildObservation {
    pub fn new(child: Child) -> Self {
        Self {
            child,
            stop_requested: false,
            observed: None,
        }
    }

    pub fn child_id(&self) -> u32 {
        self.child.id()
    }

    /// Nonblocking; None/errors do not authorize any terminal publication.
    /// try_wait retains stdin, so the caller must avoid pipe backpressure and
    /// close/write its input explicitly when needed. This API drains no pipes.
    pub fn poll(&mut self) -> io::Result<Option<&ObservedExit>> {
        if self.observed.is_none() {
            if let Some(status) = self.child.try_wait()? {
                self.observed = Some(ObservedExit {
                    status,
                    deadline_stop_requested: self.stop_requested,
                });
            }
        }
        Ok(self.observed.as_ref())
    }

    /// Reject a premature request. A successful kill request is retained, but
    /// is not an observed reap; poll/reap_after_stop must still collect status.
    /// On an OS error the owner keeps the child handle for recovery/cleanup.
    pub fn request_deadline_stop(&mut self, deadline: Instant) -> io::Result<()> {
        if self.observed.is_some() {
            return Ok(());
        }
        if Instant::now() < deadline {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "deadline has not elapsed",
            ));
        }
        if self.poll()?.is_some() {
            return Ok(());
        }
        self.child.kill()?;
        self.stop_requested = true;
        Ok(())
    }

    /// Blocking wait is available only after an accepted stop request (or an
    /// already observed exit). This is local root-child reaping, not a process
    /// tree deadline guarantee. wait errors leave the observation unavailable.
    pub fn reap_after_stop(&mut self) -> io::Result<&ObservedExit> {
        if self.observed.is_none() {
            if !self.stop_requested {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "no accepted stop request",
                ));
            }
            let status = self.child.wait()?;
            self.observed = Some(ObservedExit {
                status,
                deadline_stop_requested: true,
            });
        }
        Ok(self
            .observed
            .as_ref()
            .expect("observed after successful wait"))
    }

    /// No read callback runs until an exit has actually been collected.
    /// Observed bytes still need their own binding/content validation.
    pub fn read_after_reap<T>(
        &self,
        read: impl FnOnce() -> io::Result<T>,
    ) -> io::Result<Option<T>> {
        if self.observed.is_some() {
            read().map(Some)
        } else {
            Ok(None)
        }
    }

    /// The Attempt must be the caller's record for this child; this module does
    /// not attest that association. Publication errors propagate, including an
    /// error after a terminal link is visible but before a sync acknowledgment.
    pub fn publish_outcome(&self, attempt: &Attempt) -> io::Result<()> {
        let observed = self
            .observed
            .as_ref()
            .ok_or_else(|| io::Error::new(io::ErrorKind::WouldBlock, "child reap not observed"))?;
        attempt.finish(observed.outcome())
    }

    /// Explicitly transfer cleanup responsibility; no implicit kill or wait.
    pub fn into_child(self) -> Child {
        self.child
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use attempt_driver::durable_attempt::{inspect, Binding, Observed};
    use std::cell::Cell;
    use std::fs;
    use std::path::PathBuf;
    use std::process::{Command, Stdio};
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::Duration;

    struct Scratch(PathBuf);
    impl Scratch {
        fn new() -> Self {
            static NEXT: AtomicU64 = AtomicU64::new(0);
            for _ in 0..1000 {
                let p = std::env::temp_dir().join(format!(
                    "pkm-child-{}-{}",
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
    struct Cleanup(ChildObservation);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            if self.0.observed.is_none() {
                // Test-fixture cleanup, not a production Drop guarantee.
                let _ = self.0.request_deadline_stop(Instant::now());
                let _ = self.0.reap_after_stop();
            }
        }
    }
    fn fixture(mode: &str) -> Cleanup {
        let executable = std::env::var_os("PKM_CHILD_FIXTURE")
            .expect("test recipe must supply compiled native fixture");
        let child = Command::new(executable)
            .arg(mode)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        Cleanup(ChildObservation::new(child))
    }
    fn binding() -> Binding {
        Binding::new(&"a".repeat(64), &"b".repeat(40)).unwrap()
    }
    fn observe_ready(child: &mut ChildObservation) -> ObservedExit {
        let limit = Instant::now() + Duration::from_secs(5);
        loop {
            if let Some(status) = child.poll().unwrap() {
                return status.clone();
            }
            if Instant::now() >= limit {
                child.request_deadline_stop(limit).unwrap();
                child.reap_after_stop().unwrap();
                panic!("native fixture did not exit within test budget");
            }
            std::thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn pending_child_cannot_publish_or_read_sidecars() {
        let root = Scratch::new();
        let path = root.0.join("one");
        let attempt = Attempt::begin(&path, binding()).unwrap();
        let mut child = fixture("hold");
        assert!(child.0.poll().unwrap().is_none());
        assert_eq!(
            child.0.publish_outcome(&attempt).unwrap_err().kind(),
            io::ErrorKind::WouldBlock
        );
        let reads = Cell::new(0);
        assert_eq!(
            child
                .0
                .read_after_reap(|| {
                    reads.set(reads.get() + 1);
                    Ok(42)
                })
                .unwrap(),
            None
        );
        assert_eq!(reads.get(), 0);
        assert_eq!(inspect(&path, &binding()).unwrap(), Observed::Unfinished);
    }

    #[test]
    fn observed_zero_exit_allows_one_publication_and_propagates_read_errors() {
        let root = Scratch::new();
        let path = root.0.join("one");
        let attempt = Attempt::begin(&path, binding()).unwrap();
        let mut child = fixture("ok");
        let status = observe_ready(&mut child.0);
        assert_eq!(status.code(), Some(0));
        assert_eq!(status.outcome(), Outcome::Completed);
        assert!(!status.deadline_stop_requested());
        child.0.publish_outcome(&attempt).unwrap();
        assert_eq!(
            inspect(&path, &binding()).unwrap(),
            Observed::Terminal(Outcome::Completed)
        );
        assert_eq!(child.0.read_after_reap(|| Ok(42)).unwrap(), Some(42));
        assert!(child
            .0
            .read_after_reap(|| fs::read(root.0.join("missing")))
            .is_err());
        let before = fs::read(path.join("outcome")).unwrap();
        assert_eq!(
            child.0.publish_outcome(&attempt).unwrap_err().kind(),
            io::ErrorKind::AlreadyExists
        );
        assert_eq!(fs::read(path.join("outcome")).unwrap(), before);
        assert_eq!(child.0.poll().unwrap().unwrap().code(), Some(0));
    }

    #[test]
    fn observed_nonzero_exit_is_failed_not_completed_or_unknown_zero() {
        let root = Scratch::new();
        let path = root.0.join("one");
        let attempt = Attempt::begin(&path, binding()).unwrap();
        let mut child = fixture("fail");
        let status = observe_ready(&mut child.0);
        assert_eq!(status.code(), Some(7));
        assert_eq!(status.outcome(), Outcome::Failed);
        child.0.publish_outcome(&attempt).unwrap();
        assert_eq!(
            inspect(&path, &binding()).unwrap(),
            Observed::Terminal(Outcome::Failed)
        );
    }

    #[test]
    fn deadline_stop_request_alone_is_not_reap_or_terminal() {
        let root = Scratch::new();
        let path = root.0.join("one");
        let attempt = Attempt::begin(&path, binding()).unwrap();
        let mut child = fixture("hold");
        child.0.request_deadline_stop(Instant::now()).unwrap();
        assert!(child.0.stop_requested);
        assert!(child.0.observed.is_none());
        assert_eq!(
            child.0.publish_outcome(&attempt).unwrap_err().kind(),
            io::ErrorKind::WouldBlock
        );
        assert_eq!(inspect(&path, &binding()).unwrap(), Observed::Unfinished);
        let status = child.0.reap_after_stop().unwrap();
        assert!(status.deadline_stop_requested());
        assert_eq!(status.outcome(), Outcome::Capped);
        #[cfg(unix)]
        assert_eq!(status.code(), None); // signal termination is not exit zero.
        child.0.publish_outcome(&attempt).unwrap();
        assert_eq!(
            inspect(&path, &binding()).unwrap(),
            Observed::Terminal(Outcome::Capped)
        );
    }

    #[test]
    fn premature_stop_or_wait_error_keeps_pending_child_and_unfinished_record() {
        let root = Scratch::new();
        let path = root.0.join("one");
        let _attempt = Attempt::begin(&path, binding()).unwrap();
        let mut child = fixture("hold");
        assert_eq!(
            child
                .0
                .request_deadline_stop(Instant::now() + Duration::from_secs(60))
                .unwrap_err()
                .kind(),
            io::ErrorKind::InvalidInput
        );
        assert_eq!(
            child.0.reap_after_stop().unwrap_err().kind(),
            io::ErrorKind::InvalidInput
        );
        assert!(!child.0.stop_requested);
        assert!(child.0.poll().unwrap().is_none());
        assert_eq!(inspect(&path, &binding()).unwrap(), Observed::Unfinished);
    }

    #[test]
    fn publication_collision_preserves_bytes_after_actual_reap() {
        let root = Scratch::new();
        let path = root.0.join("one");
        let attempt = Attempt::begin(&path, binding()).unwrap();
        let mut child = fixture("ok");
        observe_ready(&mut child.0);
        fs::write(path.join("outcome"), b"retained prior bytes").unwrap();
        assert_eq!(
            child.0.publish_outcome(&attempt).unwrap_err().kind(),
            io::ErrorKind::AlreadyExists
        );
        assert_eq!(
            fs::read(path.join("outcome")).unwrap(),
            b"retained prior bytes"
        );
        assert!(inspect(&path, &binding()).is_err());
        assert_eq!(child.0.poll().unwrap().unwrap().code(), Some(0));
    }

    #[test]
    fn late_stop_does_not_relabel_an_already_observed_success() {
        let mut child = fixture("ok");
        observe_ready(&mut child.0);
        child.0.request_deadline_stop(Instant::now()).unwrap();
        let status = child.0.reap_after_stop().unwrap();
        assert!(!status.deadline_stop_requested());
        assert_eq!(status.outcome(), Outcome::Completed);
    }
}
