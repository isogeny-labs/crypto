//! Reuse the repository's native reservation and restoration implementation.
use super::{native_isolation as iso, write, R};
use serde_json::json;
use std::path::Path;
use std::time::{Duration, Instant};

pub fn lock_wait_seconds(value: Option<&serde_json::Value>) -> R<u64> {
    let seconds = match value {
        None => 0,
        Some(value) => value.as_u64().ok_or("lock_wait_seconds must be unsigned")?,
    };
    super::require(seconds <= 600, "lock wait exceeds 600 seconds")?;
    Ok(seconds)
}

fn acquire_lock(path: &str, budget: Duration) -> R<iso::BenchLock> {
    let started = Instant::now();
    let mut first_attempt = true;
    loop {
        if !first_attempt && started.elapsed() >= budget {
            return Err(format!("lock queue deadline expired for {path}").into());
        }
        first_attempt = false;
        if let Some(signal) = super::native_signals::interrupted() {
            return Err(format!("interrupted by signal {signal} while queuing for {path}").into());
        }
        match iso::BenchLock::acquire(path, false) {
            Ok(lock) => return Ok(lock),
            Err(error) => {
                let busy =
                    format!("another benchmark holds {path}; pass --wait to queue behind it");
                if error != busy || started.elapsed() >= budget {
                    return Err(error.into());
                }
                std::thread::sleep(
                    Duration::from_millis(200).min(budget.saturating_sub(started.elapsed())),
                );
            }
        }
    }
}

fn cpu_request(value: Option<&serde_json::Value>) -> R<iso::CpuRequest> {
    match value {
        None => Ok(iso::CpuRequest::Auto),
        Some(value) => {
            let cpus: Vec<u32> = serde_json::from_value(value.clone())?;
            super::require(
                !cpus.is_empty() && cpus.windows(2).all(|w| w[0] < w[1]),
                "cpu_reservation must be a nonempty ascending list without duplicates",
            )?;
            // plan_cpus also checks online/cgroup membership and all SMT siblings.
            Ok(iso::CpuRequest::Explicit(cpus))
        }
    }
}

pub struct Reservation {
    pub plan: iso::CpuPlan,
    _lock: iso::BenchLock,
    eviction: iso::Eviction,
    original: Vec<u32>,
}
impl Reservation {
    pub fn acquire(dir: &Path, cpus: Option<&serde_json::Value>, wait_seconds: u64) -> R<Self> {
        let started = Instant::now();
        let acquired = acquire_lock(iso::DEFAULT_LOCK, Duration::from_secs(wait_seconds));
        write(
            dir.join("lock-wait.json"),
            &json!({
                "schema":"n83.lock-wait/v1", "lock_path":iso::DEFAULT_LOCK,
                "requested_seconds":wait_seconds,"elapsed_seconds":started.elapsed().as_secs_f64(),
                "acquired":acquired.is_ok(),"error":acquired.as_ref().err().map(|e|e.to_string()),
                "scope":"before CPU planning, eviction, container creation or worker timing"
            }),
        )?;
        let lock = acquired?;
        let plan = iso::plan_cpus(
            &cpu_request(cpus)?,
            &iso::topology(),
            iso::allowed_cpus().as_deref(),
        )?
        .ok_or("no CPU plan")?;
        let original = iso::own_affinity().ok_or("no affinity")?;
        let eviction = iso::evict(&plan.reserved);
        iso::leave_reserved(&plan.reserved)?;
        let guard = Self {
            plan,
            _lock: lock,
            eviction,
            original,
        };
        // Preflight samples are readiness observations, never worker retries.
        // Hold the reservation while transient kernel activity settles, with
        // a strict bound; preserve every sample, including failed ones.
        let settle_started = std::time::Instant::now();
        let mut preflights = Vec::new();
        let preflight = loop {
            let sample = iso::preflight(&guard.plan, &iso::Thresholds::default());
            let ready = sample.quiet;
            preflights.push(serde_json::to_value(&sample)?);
            if ready
                || guard.eviction.summary.failed_user_threads != 0
                || settle_started.elapsed().as_secs() >= 30
            {
                break sample;
            }
        };
        write(
            dir.join("isolation-before.json"),
            &json!({
                "plan":guard.plan,"eviction":guard.eviction.summary,"preflight":preflight,"preflight_samples":preflights,
                "settings":iso::cpu_settings(guard.plan.run_cpu),
                "uid":unsafe { libc::geteuid() },
                "grade":"recorded reservation; timing eligibility assessed from observations, no automatic L2 claim"
            }),
        )?;
        super::require(
            preflight.quiet && guard.eviction.summary.failed_user_threads == 0,
            "CPU reservation preflight failed; observations retained, no timed worker launched",
        )?;
        Ok(guard)
    }
    pub fn finish(&self, dir: &Path) -> R<()> {
        write(
            dir.join("isolation-after.json"),
            &json!({"conditions":iso::conditions(&self.plan.observed_cpus())}),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lock_queue_is_explicit_and_bounded() {
        assert_eq!(lock_wait_seconds(None).unwrap(), 0);
        assert_eq!(lock_wait_seconds(Some(&json!(600))).unwrap(), 600);
        for value in [
            json!(-1),
            json!(601),
            json!(true),
            json!(null),
            json!("600"),
        ] {
            assert!(lock_wait_seconds(Some(&value)).is_err());
        }
    }

    #[test]
    fn contended_lock_times_out_then_acquires_after_release() {
        let path = std::env::temp_dir().join(format!("n83-lock-queue-{}", std::process::id()));
        let path = path.to_str().unwrap();
        let held = iso::BenchLock::acquire(path, false).unwrap();
        let started = Instant::now();
        assert!(acquire_lock(path, Duration::from_millis(30)).is_err());
        assert!(started.elapsed() >= Duration::from_millis(30));
        let releaser = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(40));
            drop(held);
        });
        let acquired = acquire_lock(path, Duration::from_secs(2)).unwrap();
        releaser.join().unwrap();
        drop(acquired);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn lock_open_errors_are_not_queued() {
        let path =
            std::env::temp_dir().join(format!("n83-lock-no-parent-{}/lock", std::process::id()));
        let error = acquire_lock(path.to_str().unwrap(), Duration::from_secs(600))
            .err()
            .unwrap();
        assert!(error.to_string().starts_with("cannot open lock"));
    }

    #[test]
    fn explicit_reservation_preserves_strict_requests() {
        assert_eq!(cpu_request(None).unwrap(), iso::CpuRequest::Auto);
        assert_eq!(
            cpu_request(Some(&json!([14, 30]))).unwrap(),
            iso::CpuRequest::Explicit(vec![14, 30])
        );
        for value in [
            json!([]),
            json!([14, 14]),
            json!([30, 14]),
            json!([-1]),
            json!("none"),
            json!(null),
        ] {
            assert!(cpu_request(Some(&value)).is_err());
        }
    }
}
impl Drop for Reservation {
    fn drop(&mut self) {
        self.eviction.restore();
        unsafe {
            let mut mask: libc::cpu_set_t = std::mem::zeroed();
            libc::CPU_ZERO(&mut mask);
            for &cpu in &self.original {
                libc::CPU_SET(cpu as usize, &mut mask);
            }
            libc::sched_setaffinity(0, std::mem::size_of_val(&mask), &mask);
        }
    }
}
