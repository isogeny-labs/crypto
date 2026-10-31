//! Phase-scoped native PMU and process counters. Missing counters remain missing.
pub(crate) mod checked;
use serde_json::{json, Value};
use std::path::Path;
use std::time::Instant;

use super::{storage, types::Artifact};
use crate::cryptanalysis::ecbench::isolation::HwCounters;

fn usage() -> Option<Value> {
    #[cfg(unix)]
    {
        // SAFETY: getrusage writes the allocated, correctly sized rusage value.
        let mut ru: libc::rusage = unsafe { std::mem::zeroed() };
        if unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut ru) } != 0 {
            return None;
        }
        let micros = |t: libc::timeval| t.tv_sec as u64 * 1_000_000 + t.tv_usec as u64;
        Some(
            json!({"user_cpu_us":micros(ru.ru_utime),"system_cpu_us":micros(ru.ru_stime),
            "peak_rss_native":ru.ru_maxrss,"peak_rss_unit":if cfg!(target_os="macos") {"bytes"} else {"KiB"},
            "minor_faults":ru.ru_minflt,"major_faults":ru.ru_majflt,
            "voluntary_context_switches":ru.ru_nvcsw,"involuntary_context_switches":ru.ru_nivcsw,
            "scope":"cumulative worker process; peak RSS is a high-water mark, not an additive phase cost"}),
        )
    }
    #[cfg(not(unix))]
    None
}

/// Keep phase boundaries durable before work begins and record metrics even when
/// a backend returns an error. SIGKILL/OOM cannot run the final read: the start
/// record survives and no completed count is fabricated for that phase.
pub fn measured<T>(
    out: &Path,
    registry: &mut Vec<Artifact>,
    phase: &str,
    op: impl FnOnce() -> Result<T, String>,
) -> Result<T, String> {
    if checked::policy_present(out)? {
        return checked::measured(out, registry, phase, op);
    }
    let before = usage();
    registry.push(storage::write_json(out, &format!("telemetry/{phase}-start.json"),
        "phase-start", &json!({"schema":"crypto.ml-dataset.phase-start/v1","phase":phase,
            "pid":std::process::id(),"usage_before":before,"counter_boundary":"after this durable record"}))?);
    let counters = HwCounters::open();
    let started = Instant::now();
    counters.start();
    let result = op();
    let counts = counters.stop();
    let elapsed_ns = started.elapsed().as_nanos().to_string();
    let usable =
        counts.instructions.is_some() && counts.instructions_time_running_ns.is_some_and(|n| n > 0);
    let exact = usable && counts.multiplexed == Some(false);
    let status = if !usable {
        "unavailable"
    } else if exact {
        "measured_unmultiplexed"
    } else {
        "measured_multiplexed_estimate"
    };
    registry.push(storage::write_json(out, &format!("telemetry/{phase}.json"),
        "hardware-and-resource-telemetry", &json!({
            "schema":"crypto.ml-dataset.phase-metrics/v1","phase":phase,
            "returned_ok":result.is_ok(),"returned_error":result.as_ref().err(),
            "instructions_status":status,"retired_instructions_raw":if usable {counts.instructions} else {None},
            "retired_instructions_exact_in_scope":if exact {counts.instructions} else {None},
            "hardware":counts,"usage_after":usage(),"elapsed_ns_diagnostic":elapsed_ns,
            "clock_valid_for_comparison":false,"hardware_events":["PERF_COUNT_HW_INSTRUCTIONS","PERF_COUNT_HW_CPU_CYCLES"],
            "counter_scope":"calling thread user space and newly created threads/children only when hardware.inherit=true; excludes kernel/hypervisor",
            "excludes":["process startup","artifact serialization outside phase","upload","preexisting worker threads"],
            "interrupted_phase_readback":false,
            "scaling":"raw counts preserved; hardware.*_scaled estimates multiplexed counts using time_enabled/time_running"
        }))?);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml_dataset::{curves, solvers, types::CaseConfig};

    #[test]
    fn reference_and_candidate_phase_telemetry_preserves_errors_and_boundaries() {
        let root =
            std::env::temp_dir().join(format!("ml-dataset-phase-test-{}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        let config = CaseConfig {
            family: "koblitz".into(),
            n: 23,
            m: 2,
            twist: false,
            curve_seed: 0,
            generator_index: 0,
            key_index: 0,
            seed: 20261027,
            factor_base_size: 4,
            degree_bound: 3,
            work_limit: 20_000,
            solver: "bsgs".into(),
        };
        let case = curves::generate(&config).unwrap();
        let mut artifacts = Vec::new();
        for method in ["bsgs", "enumerate"] {
            let mut cfg = config.clone();
            cfg.solver = method.into();
            let outcome = measured(&root, &mut artifacts, method, || {
                solvers::solve(&cfg, &case)
            })
            .unwrap();
            if method == "bsgs" {
                assert_eq!(outcome.outcome["scalar_recovered_without_witness"], true);
            }
            let metrics: Value = serde_json::from_slice(
                &std::fs::read(root.join(format!("telemetry/{method}.json"))).unwrap(),
            )
            .unwrap();
            assert_eq!(metrics["returned_ok"], true);
            assert_eq!(metrics["clock_valid_for_comparison"], false);
            assert!(root.join(format!("telemetry/{method}-start.json")).exists());
            println!(
                "software_preflight_only={}",
                json!({"method":method,"n":23,"m":2,"factor_base_size":4,"work_limit":20000,"metrics":metrics})
            );
        }
        let failure = measured::<()>(&root, &mut artifacts, "returned-error", || {
            Err("deliberate test failure".into())
        });
        assert_eq!(failure.unwrap_err(), "deliberate test failure");
        let metrics: Value = serde_json::from_slice(
            &std::fs::read(root.join("telemetry/returned-error.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(metrics["returned_ok"], false);
        assert_eq!(metrics["returned_error"], "deliberate test failure");
        storage::verify_manifest(&root, &artifacts).unwrap();
        std::fs::remove_dir_all(root).unwrap();
    }
}
