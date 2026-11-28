//! Native process observations, using the production ecbench modules.
//! The C ABI is optional instrumentation, never a mathematical verifier.
use std::io::Write;
use std::os::fd::FromRawFd;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[path = "../../../../../src/cryptanalysis/ecbench/canonical.rs"]
pub mod canonical;
#[path = "../../../../../src/cryptanalysis/ecbench/host.rs"]
pub mod host;
#[path = "../../../../../src/cryptanalysis/ecbench/isolation.rs"]
pub mod isolation;
#[path = "../../../../../src/cryptanalysis/ecbench/measurement.rs"]
pub mod measurement;
#[path = "../../../../../src/hash/sha256.rs"]
pub mod sha256;
#[path = "../../../../../src/cryptanalysis/ecbench/signals.rs"]
pub mod signals;

// Preserve the production modules' import paths; no replacement implementation.
pub mod cryptanalysis {
    pub mod ecbench {
        pub use crate::{canonical, host, isolation, measurement, signals};
    }
}
pub mod hash {
    pub use crate::sha256;
}

pub mod command;

pub const CHILD_SCHEMA: &str = "cubic.native_process_observations/v1";

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChildObservations {
    pub schema: String,
    pub pid: u32,
    /// Inside main, before argument/input parsing through final cleanup.
    /// The parent records the separate fork-to-reap cold interval.
    pub algorithm_wall_ns: u64,
    pub placement: isolation::SelfPlacement,
    pub solve: measurement::SolveObs,
    pub schedstat: Option<measurement::SchedStat>,
    /// Full checked lifecycle, including unsupported/denied PMU outcomes.
    /// Its coverage/platform booleans retain the collector's false defaults.
    pub pmu: Value,
}

struct Resident {
    fd: i32,
    placement: isolation::SelfPlacement,
    probe: measurement::SolveProbe,
    schedstat: Option<measurement::SchedStat>,
    counters: isolation::checked::CheckedCounters,
    start: Instant,
}

impl Resident {
    fn begin(fd: i32) -> Result<Self, String> {
        if fd < 3 || unsafe { libc::fcntl(fd, libc::F_GETFD) } < 0 {
            return Err("invalid observation descriptor".into());
        }
        let placement = isolation::SelfPlacement {
            affinity: isolation::own_affinity(),
            cpu_at_start: isolation::current_cpu(),
            mems_allowed: isolation::mems_allowed(),
            mempolicy: isolation::mempolicy(),
            ..Default::default()
        };
        let mut counters =
            isolation::checked::CheckedCounters::open(isolation::checked::CounterPolicy {
                scope: isolation::checked::TaskScope::CallingThreadAndNewDescendants,
                cycles: isolation::checked::CyclesPolicy::OptionalWithStandaloneFallback,
            });
        let probe = measurement::SolveProbe::read();
        let schedstat = measurement::schedstat();
        // Infrastructure probes enclose the actual algorithm window. Their
        // exact PMU coverage is retained rather than called complete cold work.
        counters.start();
        Ok(Self {
            fd,
            placement,
            probe,
            schedstat,
            counters,
            start: Instant::now(),
        })
    }

    fn finish(mut self) -> Result<(), String> {
        let elapsed = u64::try_from(self.start.elapsed().as_nanos())
            .map_err(|_| "algorithm interval overflow")?;
        let pmu = serde_json::to_value(self.counters.finish()).map_err(|e| e.to_string())?;
        let after = measurement::SolveProbe::read();
        let schedstat = match (measurement::schedstat(), self.schedstat) {
            (Some(b), Some(a)) => Some(measurement::SchedStat {
                cpu_ns: b
                    .cpu_ns
                    .checked_sub(a.cpu_ns)
                    .ok_or("scheduler counter regression")?,
                run_delay_ns: b
                    .run_delay_ns
                    .checked_sub(a.run_delay_ns)
                    .ok_or("scheduler counter regression")?,
                timeslices: b
                    .timeslices
                    .checked_sub(a.timeslices)
                    .ok_or("scheduler counter regression")?,
            }),
            _ => None,
        };
        self.placement.cpu_at_end = isolation::current_cpu();
        self.placement.anon_pages_by_node = isolation::anon_pages_by_node();
        let doc = ChildObservations {
            schema: CHILD_SCHEMA.into(),
            pid: std::process::id(),
            algorithm_wall_ns: elapsed,
            placement: self.placement,
            solve: after.since(&self.probe),
            schedstat,
            pmu,
        };
        // The parent exclusively creates this file and passes the descriptor.
        // The child owns its post-fork descriptor; no path is opened here.
        let mut out = unsafe { std::fs::File::from_raw_fd(self.fd) };
        serde_json::to_writer(&mut out, &doc).map_err(|e| e.to_string())?;
        out.write_all(b"\n").map_err(|e| e.to_string())?;
        out.sync_all().map_err(|e| e.to_string())
    }
}

/// Start observations on the current thread. Null means instrumentation failed.
///
/// # Safety
/// `fd` must be a writable, uniquely owned child observation descriptor. Invoke
/// finish once on the same thread, before the library is unloaded.
#[no_mangle]
pub unsafe extern "C" fn cubic_measure_begin(fd: i32) -> *mut std::ffi::c_void {
    match std::panic::catch_unwind(|| Resident::begin(fd)) {
        Ok(Ok(state)) => Box::into_raw(Box::new(state)).cast(),
        _ => std::ptr::null_mut(),
    }
}

/// Finish and consume an observation handle. Nonzero means no usable receipt.
///
/// # Safety
/// `handle` must be returned by begin, still live and consumed exactly once on
/// its creating thread. Never pass handles through a process fork.
#[no_mangle]
pub unsafe extern "C" fn cubic_measure_finish(handle: *mut std::ffi::c_void) -> i32 {
    if handle.is_null() {
        return 1;
    }
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let state = *Box::from_raw(handle.cast::<Resident>());
        state.finish()
    })) {
        Ok(Ok(())) => 0,
        _ => 1,
    }
}
