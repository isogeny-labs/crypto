//! Reuse the repository's native reservation and restoration implementation.
use super::{native_isolation as iso, write, R};
use serde_json::json;
use std::path::Path;

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
    pub fn acquire(dir: &Path, cpus: Option<&serde_json::Value>) -> R<Self> {
        let lock = iso::BenchLock::acquire("/tmp/crypto-bench.lock", false)?;
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
