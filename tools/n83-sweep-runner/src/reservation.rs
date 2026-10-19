//! Reuse the repository's native reservation and restoration implementation.
use super::{native_isolation as iso, write, R};
use serde_json::json;
use std::path::Path;

pub struct Reservation {
    pub plan: iso::CpuPlan,
    _lock: iso::BenchLock,
    eviction: iso::Eviction,
    original: Vec<u32>,
}
impl Reservation {
    pub fn acquire(dir: &Path) -> R<Self> {
        let lock = iso::BenchLock::acquire("/tmp/crypto-bench.lock", false)?;
        let plan = iso::plan_cpus(
            &iso::CpuRequest::Auto,
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
        let preflight = iso::preflight(&guard.plan, &iso::Thresholds::default());
        write(
            dir.join("isolation-before.json"),
            &json!({
                "plan":guard.plan,"eviction":guard.eviction.summary,"preflight":preflight,
                "settings":iso::cpu_settings(guard.plan.run_cpu),
                "uid":unsafe { libc::geteuid() },
                "grade":"recorded reservation; timing eligibility assessed from observations, no automatic L2 claim"
            }),
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
