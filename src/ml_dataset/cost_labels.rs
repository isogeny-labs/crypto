//! Fail-closed retired-instruction labels for complete ECDLP solves.
//!
//! The current checked collector is a calling-thread phase diagnostic. Its
//! records cannot be promoted into whole-worker training costs.
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::Path;

/// A producer-consistency projection; `trusted_replay` is supplied only by an
/// independent caller, never parsed from the producer's JSON.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CounterBinding {
    run_sha256: [u8; 32],
    cell_index: u64,
    worker_id: [u8; 32],
    interval_start_ns: u64,
    interval_end_ns: u64,
}

#[derive(Clone, Copy, Debug)]
struct CounterProof {
    binding: CounterBinding,
    whole_worker_scope: bool,
    requested_hardware_instructions: bool,
    opened_event_id: u64,
    read_event_id: u64,
    raw: u64,
    enabled_ns: u64,
    running_ns: u64,
    complete_execution: bool,
    worker_contained: bool,
    whole_worker_coverage: bool,
    descendant_lifetimes_accounted: bool,
    escaped_descendant: bool,
    readback_complete: bool,
    independently_checked_hardware: bool,
    platform_admitted: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct Decision {
    instruction_to_solve: Option<u64>,
    reason: &'static str,
}

fn reject(reason: &'static str) -> Decision {
    Decision {
        instruction_to_solve: None,
        reason,
    }
}

fn admit(
    proof: Option<&CounterProof>,
    expected_binding: Option<&CounterBinding>,
    trusted_replay: bool,
    solver_completed: bool,
    scalar_replay_verified: bool,
) -> Decision {
    if !solver_completed || !scalar_replay_verified {
        return reject("independent_scalar_replay_missing");
    }
    if !trusted_replay {
        return reject("independent_cost_evidence_replay_missing");
    }
    let Some(p) = proof else {
        return reject("whole_worker_counter_missing");
    };
    let Some(expected) = expected_binding else {
        return reject("counter_binding_missing");
    };
    if p.binding != *expected
        || p.binding.interval_start_ns >= p.binding.interval_end_ns
        || p.binding.run_sha256 == [0; 32]
        || p.binding.worker_id == [0; 32]
    {
        return reject("counter_binding_mismatch");
    }
    if !p.whole_worker_scope {
        return reject("phase_only_counter");
    }
    if !p.requested_hardware_instructions
        || !p.independently_checked_hardware
        || !p.platform_admitted
    {
        return reject("hardware_event_unverified");
    }
    if p.opened_event_id == 0 || p.read_event_id != p.opened_event_id {
        return reject("counter_event_id_mismatch");
    }
    if p.enabled_ns == 0 || p.running_ns == 0 || p.running_ns > p.enabled_ns {
        return reject("counter_time_invalid");
    }
    if p.running_ns != p.enabled_ns {
        return reject("counter_multiplexed");
    }
    if !p.complete_execution || !p.readback_complete {
        return reject("execution_or_readback_incomplete");
    }
    if !p.worker_contained
        || !p.whole_worker_coverage
        || !p.descendant_lifetimes_accounted
        || p.escaped_descendant
    {
        return reject("worker_or_descendant_coverage_missing");
    }
    Decision {
        instruction_to_solve: Some(p.raw),
        reason: "admitted",
    }
}

#[derive(Serialize)]
struct CellCost {
    cell_index: u64,
    family: String,
    n: u32,
    twist: bool,
    solver: String,
    status: String,
    instruction_to_solve: Option<u64>,
    reason: &'static str,
    missing_gates: Vec<&'static str>,
    phase_artifact_count: usize,
    phase_artifacts: Vec<String>,
}

/// Replays the native run and emits a bounded local coverage projection.
/// No present producer artifact can construct a trusted whole-worker proof.
pub(super) fn report_run(root: &Path) -> Result<Value, String> {
    let root = super::checked_directory(root, false)?;
    let _lease = super::Lease::acquire(&root)?;
    super::assert_no_workers(&root)?;
    super::verify_run(&root, true)?;
    let mut rows = Vec::new();
    let mut reasons = BTreeMap::<&'static str, usize>::new();
    for dir in super::completed_cells(&root)? {
        let manifest = super::verify_cell(&dir, true)?;
        let phase_artifacts: Vec<String> = manifest
            .artifacts
            .iter()
            .filter(|a| a.path.starts_with("telemetry/") && a.path.ends_with(".json"))
            .map(|a| format!("{}:{}", a.path, a.sha256))
            .collect();
        let phase_artifact_count = phase_artifacts.len();
        let solver_completed = manifest.outcome["solver_completed"] == true;
        let scalar_replay_verified = manifest.outcome["scalar_replay_verified"] == true
            && (manifest.outcome["scalar_recovered_without_witness"] == true
                || manifest.outcome["pdp_witness_verified"] == true);
        let mut missing_gates = vec![
            "trusted_whole_worker_counter",
            "worker_containment",
            "descendant_lifetime_accounting",
            "complete_nonmultiplexed_pmu_readback",
            "platform_hardware_validation",
            "independent_cost_evidence_replay",
        ];
        if !solver_completed {
            missing_gates.push("complete_solve");
        }
        if !scalar_replay_verified {
            missing_gates.push("independent_scalar_replay");
        }
        let d = admit(None, None, false, solver_completed, scalar_replay_verified);
        *reasons.entry(d.reason).or_default() += 1;
        rows.push(CellCost {
            cell_index: manifest.cell_index,
            family: manifest.config.family,
            n: manifest.config.n,
            twist: manifest.config.twist,
            solver: manifest.config.solver,
            status: manifest.outcome["status"]
                .as_str()
                .unwrap_or("unspecified")
                .into(),
            instruction_to_solve: d.instruction_to_solve,
            reason: d.reason,
            missing_gates,
            phase_artifact_count,
            phase_artifacts,
        });
    }
    rows.sort_by_key(|r| r.cell_index);
    let completed_cells = rows.len();
    Ok(json!({
        "schema":"crypto.ml-dataset.cost-label-coverage/v2",
        "source":root.display().to_string(),
        "cells":rows,
        "counts":{"completed_cells":completed_cells,"admitted_cost_labels":0,"reasons":reasons},
        "method":"native_manifest_and_scalar_replay; no whole-worker PMU collector admitted"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid() -> CounterProof {
        CounterProof {
            binding: CounterBinding {
                run_sha256: [1; 32],
                cell_index: 7,
                worker_id: [2; 32],
                interval_start_ns: 10,
                interval_end_ns: 90,
            },
            whole_worker_scope: true,
            requested_hardware_instructions: true,
            opened_event_id: 12,
            read_event_id: 12,
            raw: 1234,
            enabled_ns: 80,
            running_ns: 80,
            complete_execution: true,
            worker_contained: true,
            whole_worker_coverage: true,
            descendant_lifetimes_accounted: true,
            escaped_descendant: false,
            readback_complete: true,
            independently_checked_hardware: true,
            platform_admitted: true,
        }
    }

    #[test]
    fn instruction_label_requires_all_independent_evidence() {
        let p = valid();
        assert_eq!(
            admit(Some(&p), Some(&p.binding), true, true, true).instruction_to_solve,
            Some(1234)
        );
        assert_eq!(
            admit(Some(&p), Some(&p.binding), false, true, true).instruction_to_solve,
            None
        );
        assert_eq!(
            admit(Some(&p), Some(&p.binding), true, true, false).reason,
            "independent_scalar_replay_missing"
        );
        assert_eq!(
            admit(None, None, true, true, true).reason,
            "whole_worker_counter_missing"
        );
    }

    #[test]
    fn forged_id_multiplexing_and_escaped_descendant_are_rejected() {
        let mut p = valid();
        p.read_event_id = 13;
        assert_eq!(
            admit(Some(&p), Some(&p.binding), true, true, true).reason,
            "counter_event_id_mismatch"
        );
        p = valid();
        p.running_ns = 79;
        assert_eq!(
            admit(Some(&p), Some(&p.binding), true, true, true).reason,
            "counter_multiplexed"
        );
        p = valid();
        p.escaped_descendant = true;
        assert_eq!(
            admit(Some(&p), Some(&p.binding), true, true, true).reason,
            "worker_or_descendant_coverage_missing"
        );
        p = valid();
        p.worker_contained = false;
        assert_eq!(
            admit(Some(&p), Some(&p.binding), true, true, true).reason,
            "worker_or_descendant_coverage_missing"
        );
    }

    #[test]
    fn phase_counter_and_incomplete_readback_are_rejected() {
        let mut p = valid();
        p.whole_worker_scope = false;
        assert_eq!(
            admit(Some(&p), Some(&p.binding), true, true, true).reason,
            "phase_only_counter"
        );
        p = valid();
        p.readback_complete = false;
        assert_eq!(
            admit(Some(&p), Some(&p.binding), true, true, true).reason,
            "execution_or_readback_incomplete"
        );
        p = valid();
        p.platform_admitted = false;
        assert_eq!(
            admit(Some(&p), Some(&p.binding), true, true, true).reason,
            "hardware_event_unverified"
        );
    }

    #[test]
    fn counter_must_bind_exact_run_cell_worker_and_interval() {
        let p = valid();
        assert_eq!(
            admit(Some(&p), None, true, true, true).reason,
            "counter_binding_missing"
        );
        let mut expected = p.binding;
        expected.run_sha256[0] ^= 1;
        assert_eq!(
            admit(Some(&p), Some(&expected), true, true, true).reason,
            "counter_binding_mismatch"
        );
        expected = p.binding;
        expected.cell_index += 1;
        assert_eq!(
            admit(Some(&p), Some(&expected), true, true, true).reason,
            "counter_binding_mismatch"
        );
        expected = p.binding;
        expected.worker_id[0] ^= 1;
        assert_eq!(
            admit(Some(&p), Some(&expected), true, true, true).reason,
            "counter_binding_mismatch"
        );
        expected = p.binding;
        expected.interval_end_ns += 1;
        assert_eq!(
            admit(Some(&p), Some(&expected), true, true, true).reason,
            "counter_binding_mismatch"
        );
        let mut invalid = p;
        invalid.binding.interval_start_ns = invalid.binding.interval_end_ns;
        assert_eq!(
            admit(Some(&invalid), Some(&invalid.binding), true, true, true).reason,
            "counter_binding_mismatch"
        );
    }
}
