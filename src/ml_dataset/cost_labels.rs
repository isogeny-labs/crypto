//! Fail-closed retired-instruction labels for complete ECDLP solves.
//!
//! The current checked collector is a calling-thread phase diagnostic. Its
//! records cannot be promoted into whole-worker training costs.
use serde::Serialize;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Scope {
    WholeWorker,
    Phase,
}

/// A producer-consistency projection; `trusted_replay` is supplied only by an
/// independent caller, never parsed from the producer's JSON.
#[derive(Clone, Copy, Debug)]
struct CounterProof {
    scope: Scope,
    requested_hardware_instructions: bool,
    opened_event_id: u64,
    read_event_id: u64,
    raw: u64,
    enabled_ns: u64,
    running_ns: u64,
    complete_execution: bool,
    whole_worker_coverage: bool,
    descendant_lifetimes_accounted: bool,
    escaped_descendant: bool,
    readback_complete: bool,
    independently_checked_hardware: bool,
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
    if p.scope != Scope::WholeWorker {
        return reject("phase_only_counter");
    }
    if !p.requested_hardware_instructions || !p.independently_checked_hardware {
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
    if !p.whole_worker_coverage || !p.descendant_lifetimes_accounted || p.escaped_descendant {
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
    phase_records: usize,
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
        let phase_records = manifest
            .artifacts
            .iter()
            .filter(|a| a.path.starts_with("telemetry/") && a.path.ends_with(".json"))
            .count();
        let d = admit(
            None,
            false,
            manifest.outcome["solver_completed"] == true,
            manifest.outcome["scalar_replay_verified"] == true
                && manifest.outcome["scalar_recovered_without_witness"] == true,
        );
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
            phase_records,
        });
    }
    rows.sort_by_key(|r| r.cell_index);
    let completed_cells = rows.len();
    Ok(json!({
        "schema":"crypto.ml-dataset.cost-label-coverage/v1",
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
            scope: Scope::WholeWorker,
            requested_hardware_instructions: true,
            opened_event_id: 12,
            read_event_id: 12,
            raw: 1234,
            enabled_ns: 80,
            running_ns: 80,
            complete_execution: true,
            whole_worker_coverage: true,
            descendant_lifetimes_accounted: true,
            escaped_descendant: false,
            readback_complete: true,
            independently_checked_hardware: true,
        }
    }

    #[test]
    fn instruction_label_requires_all_independent_evidence() {
        let p = valid();
        assert_eq!(
            admit(Some(&p), true, true, true).instruction_to_solve,
            Some(1234)
        );
        assert_eq!(
            admit(Some(&p), false, true, true).instruction_to_solve,
            None
        );
        assert_eq!(
            admit(Some(&p), true, true, false).reason,
            "independent_scalar_replay_missing"
        );
        assert_eq!(
            admit(None, true, true, true).reason,
            "whole_worker_counter_missing"
        );
    }

    #[test]
    fn forged_id_multiplexing_and_escaped_descendant_are_rejected() {
        let mut p = valid();
        p.read_event_id = 13;
        assert_eq!(
            admit(Some(&p), true, true, true).reason,
            "counter_event_id_mismatch"
        );
        p = valid();
        p.running_ns = 79;
        assert_eq!(
            admit(Some(&p), true, true, true).reason,
            "counter_multiplexed"
        );
        p = valid();
        p.escaped_descendant = true;
        assert_eq!(
            admit(Some(&p), true, true, true).reason,
            "worker_or_descendant_coverage_missing"
        );
    }

    #[test]
    fn phase_counter_and_incomplete_readback_are_rejected() {
        let mut p = valid();
        p.scope = Scope::Phase;
        assert_eq!(
            admit(Some(&p), true, true, true).reason,
            "phase_only_counter"
        );
        p = valid();
        p.readback_complete = false;
        assert_eq!(
            admit(Some(&p), true, true, true).reason,
            "execution_or_readback_incomplete"
        );
    }
}
