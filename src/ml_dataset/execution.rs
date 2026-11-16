//! Attempt-bound original worker limits. Historical verification never reads a clock.
use super::capture::AttemptBinding;
use super::types::Artifact;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;
use std::time::{Duration, Instant};

pub(super) const PATH: &str = "execution-context.json";
pub(super) const ROLE: &str = "restricted-original-worker-execution-context/v1";
const SCHEMA: &str = "crypto.ml-dataset.execution-context/v1";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct LaunchContext {
    pub schema: String,
    pub binding: AttemptBinding,
    pub boot_id: String,
    pub start_monotonic_ns: u64,
    pub deadline_monotonic_ns: u64,
    pub seconds_per_case: u64,
    pub memory_bytes: u64,
    pub trace_bytes: u64,
}
#[derive(Clone, Debug)]
pub(super) struct SolverExecutionContext {
    pub launch: LaunchContext,
    pub deadline: Instant,
}
fn positive(run: &Value, key: &str) -> Result<u64, String> {
    run[key]
        .as_u64()
        .filter(|n| *n > 0)
        .ok_or_else(|| format!("missing/zero original {key}"))
}
fn bytes(run: &Value, key: &str) -> Result<u64, String> {
    positive(run, key)?
        .checked_mul(1024 * 1024)
        .ok_or_else(|| format!("original {key} overflow"))
}
impl LaunchContext {
    fn new_at(
        binding: AttemptBinding,
        run: &Value,
        boot_id: String,
        start: u64,
    ) -> Result<Self, String> {
        let seconds = positive(run, "seconds_per_case")?;
        let duration = seconds
            .checked_mul(1_000_000_000)
            .ok_or("worker duration overflow")?;
        let context = Self {
            schema: SCHEMA.into(),
            binding,
            boot_id,
            start_monotonic_ns: start,
            deadline_monotonic_ns: start
                .checked_add(duration)
                .ok_or("worker deadline overflow")?,
            seconds_per_case: seconds,
            memory_bytes: bytes(run, "memory_mib")?,
            trace_bytes: bytes(run, "trace_mib")?,
        };
        context.validate(run, &context.binding)?;
        Ok(context)
    }
    /// Static replay: no dependence on current clock, boot, or elapsed lifetime.
    pub fn validate(&self, run: &Value, binding: &AttemptBinding) -> Result<(), String> {
        let interval = self
            .deadline_monotonic_ns
            .checked_sub(self.start_monotonic_ns)
            .ok_or("reversed worker deadline")?;
        let duration = self
            .seconds_per_case
            .checked_mul(1_000_000_000)
            .ok_or("worker duration overflow")?;
        if self.schema != SCHEMA
            || &self.binding != binding
            || self.boot_id.len() != 36
            || !self.boot_id.bytes().enumerate().all(|(i, b)| {
                if [8, 13, 18, 23].contains(&i) {
                    b == b'-'
                } else {
                    b.is_ascii_hexdigit()
                }
            })
            || self.seconds_per_case != positive(run, "seconds_per_case")?
            || self.memory_bytes != bytes(run, "memory_mib")?
            || self.trace_bytes != bytes(run, "trace_mib")?
            || interval != duration
        {
            return Err("execution context differs from original run/attempt".into());
        }
        Ok(())
    }
    fn admit_at(
        &self,
        boot: &str,
        now: u64,
        instant: Instant,
    ) -> Result<SolverExecutionContext, String> {
        if boot != self.boot_id || now < self.start_monotonic_ns {
            return Err("execution context clock/boot mismatch".into());
        }
        let remaining = self
            .deadline_monotonic_ns
            .checked_sub(now)
            .filter(|n| *n > 0)
            .ok_or("original worker deadline expired")?;
        Ok(SolverExecutionContext {
            launch: self.clone(),
            deadline: instant
                .checked_add(Duration::from_nanos(remaining))
                .ok_or("Instant deadline overflow")?,
        })
    }
}
#[cfg(target_os = "linux")]
pub(super) fn monotonic_ns() -> Result<u64, String> {
    let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    // SAFETY: ts is writable and CLOCK_MONOTONIC is a system clock.
    if unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts) } != 0 {
        return Err(std::io::Error::last_os_error().to_string());
    }
    let seconds = u64::try_from(ts.tv_sec).map_err(|_| "negative monotonic seconds")?;
    let nanos = u64::try_from(ts.tv_nsec).map_err(|_| "negative monotonic nanoseconds")?;
    if nanos >= 1_000_000_000 {
        return Err("invalid monotonic nanoseconds".into());
    }
    seconds
        .checked_mul(1_000_000_000)
        .and_then(|s| s.checked_add(nanos))
        .ok_or_else(|| "monotonic clock overflow".into())
}
#[cfg(not(target_os = "linux"))]
pub(super) fn monotonic_ns() -> Result<u64, String> {
    Err("supervised execution context currently requires Linux CLOCK_MONOTONIC".into())
}
fn boot_id() -> Result<String, String> {
    std::fs::read_to_string("/proc/sys/kernel/random/boot_id")
        .map(|s| s.trim().to_owned())
        .map_err(|e| e.to_string())
}
/// Apply the supervisor's terminal decision. Retained worker result.json is
/// separate evidence; a censored manifest never inherits its success labels.
pub(super) fn terminal_outcome(
    worker: Value,
    stop: Option<&str>,
    exit_success: bool,
    exit_code: Option<i32>,
) -> Value {
    if let Some(reason) = stop {
        return serde_json::json!({"status":reason,"solver_completed":false});
    }
    if !exit_success {
        return serde_json::json!({"status":"worker_failed","exit_code":exit_code,"solver_completed":false});
    }
    worker
}

pub(super) fn deadline_expired(deadline: u64, observed: u64) -> bool {
    observed >= deadline
}

pub(super) fn create(dir: &Path) -> Result<LaunchContext, String> {
    let run: Value = super::read_json(&dir.join("run.json"))?;
    let context = LaunchContext::new_at(
        AttemptBinding::from_cell_files(dir)?,
        &run,
        boot_id()?,
        monotonic_ns()?,
    )?;
    super::storage::write_json(dir, PATH, ROLE, &context)?;
    Ok(context)
}
pub(super) fn load(dir: &Path) -> Result<Option<LaunchContext>, String> {
    match std::fs::symlink_metadata(dir.join(PATH)) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            if dir.join("run.json").exists() {
                let run: Value = super::read_json(&dir.join("run.json"))?;
                if run["execution_context_required"] == true {
                    return Err("required original execution context missing".into());
                }
            }
            return Ok(None);
        }
        Err(e) => return Err(e.to_string()),
        Ok(m) if !m.is_file() || m.file_type().is_symlink() => {
            return Err("execution context is not a regular file".into())
        }
        Ok(_) => {}
    }
    let context: LaunchContext = super::read_json(&dir.join(PATH))?;
    context.validate(
        &super::read_json(&dir.join("run.json"))?,
        &AttemptBinding::from_cell_files(dir)?,
    )?;
    Ok(Some(context))
}
pub(super) fn admit(dir: &Path) -> Result<Option<SolverExecutionContext>, String> {
    load(dir)?
        .map(|c| {
            // Sample Instant before the system clock, never extend the remaining interval.
            let instant = Instant::now();
            let now = monotonic_ns()?;
            c.admit_at(&boot_id()?, now, instant)
        })
        .transpose()
}
pub(super) fn verify(dir: &Path, artifacts: &[Artifact]) -> Result<(), String> {
    let context = load(dir)?;
    let records: Vec<_> = artifacts
        .iter()
        .filter(|a| a.path == PATH || a.role == ROLE)
        .collect();
    if context.is_some() {
        if records.len() != 1
            || records[0].path != PATH
            || records[0].role != ROLE
            || records[0].encoding != "json"
        {
            return Err("execution context manifest identity mismatch".into());
        }
    } else if !records.is_empty() {
        return Err("execution context descriptor without original record".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (LaunchContext, Value) {
        let binding = AttemptBinding {
            schema: "test".into(),
            run_sha256: "r".into(),
            plan_sha256: "p".into(),
            protocol_sha256: "t".into(),
            config_sha256: "c".into(),
            source_commit: "s".into(),
            binary_sha256: "b".into(),
            cell_index: 0,
            attempt_sha256: "a".into(),
        };
        let run = serde_json::json!({"seconds_per_case":2,"memory_mib":512,"trace_mib":16});
        (
            LaunchContext::new_at(
                binding,
                &run,
                "00000000-0000-0000-0000-000000000000".into(),
                10,
            )
            .unwrap(),
            run,
        )
    }
    #[test]
    fn replay_rejects_simultaneous_duration_overflow_and_reversed_deadline() {
        let (mut c, mut run) = fixture();
        c.seconds_per_case = u64::MAX;
        run["seconds_per_case"] = serde_json::json!(u64::MAX);
        c.start_monotonic_ns = 10;
        c.deadline_monotonic_ns = 9;
        assert!(c.validate(&run, &c.binding).is_err());
    }
    #[test]
    fn successful_exit_observation_at_deadline_is_censored() {
        // Same decision and terminal-outcome reducer as generate's observed
        // successful child exit. No sleeps/process scheduling assumptions.
        let worker = serde_json::json!({"status":"verified","solver_completed":true,"pdp_witness_verified":true,"exhaustive_no_decomposition":true});
        for observed in [99, 100, 101] {
            let stop = deadline_expired(100, observed).then_some("timeout");
            let result = terminal_outcome(worker.clone(), stop, true, Some(0));
            if observed < 100 {
                assert_eq!(result, worker);
            } else {
                assert_eq!(
                    result,
                    serde_json::json!({"status":"timeout","solver_completed":false})
                );
                assert!(result.get("pdp_witness_verified").is_none());
            }
        }
        assert_eq!(
            terminal_outcome(
                worker.clone(),
                Some("trace_budget_exhausted"),
                true,
                Some(0)
            )["solver_completed"],
            false
        );
        assert_eq!(
            terminal_outcome(worker, None, false, Some(137))["status"],
            "worker_failed"
        );
    }
    #[test]
    fn original_deadline_survives_setup_and_historical_replay() {
        let (c, run) = fixture();
        let instant = Instant::now();
        let context = c.admit_at(&c.boot_id, 1_000_000_010, instant).unwrap();
        assert_eq!(
            context.deadline.duration_since(instant),
            Duration::from_secs(1)
        );
        assert!(c
            .admit_at(&c.boot_id, c.deadline_monotonic_ns, instant)
            .is_err());
        assert!(c.admit_at("another-boot", 10, instant).is_err());
        assert!(c.validate(&run, &c.binding).is_ok());
    }
    #[test]
    fn context_rejects_rehashed_limits_deadlines_and_unknown_fields() {
        let (c, run) = fixture();
        for field in [
            "memory_bytes",
            "trace_bytes",
            "seconds_per_case",
            "deadline_monotonic_ns",
            "start_monotonic_ns",
        ] {
            let mut value = serde_json::to_value(&c).unwrap();
            value[field] = serde_json::json!(value[field].as_u64().unwrap() + 1);
            let bad: LaunchContext = serde_json::from_value(value).unwrap();
            assert!(bad.validate(&run, &c.binding).is_err());
        }
        let mut value = serde_json::to_value(&c).unwrap();
        value["unknown"] = serde_json::json!(1);
        assert!(serde_json::from_value::<LaunchContext>(value).is_err());
        for field in ["seconds_per_case", "memory_mib", "trace_mib"] {
            for n in [0, u64::MAX] {
                let mut bad = run.clone();
                bad[field] = serde_json::json!(n);
                assert!(
                    LaunchContext::new_at(c.binding.clone(), &bad, c.boot_id.clone(), 10).is_err()
                );
            }
        }
    }
}
