//! Restricted artifact-prefix report. Never a training row or generation receipt.
use super::{
    budget::{Budget, REPORT_MAX},
    capture::CaptureState,
    chain::{Preflight, Snapshot},
    schema::Anchor,
};
use anyhow::{ensure, Result};
use serde_json::{json, Value};
use std::io::Write;

pub(crate) struct Outcome {
    pub capture: Option<CaptureState>,
    pub phase: Option<crate::phase::CheckedPhaseAudit>,
    pub restored: bool,
    pub actual_remote: bool,
}
pub(crate) fn build(
    anchor: &Anchor,
    expected: &str,
    chain: &[Snapshot],
    admitted: &Preflight<'_>,
    outcome: Outcome,
    budget: &mut Budget,
) -> Result<Value> {
    let Outcome {
        capture,
        phase,
        restored,
        actual_remote,
    } = outcome;
    budget.charge(
        32768 + chain.len() * 12288 + anchor.remote.len() * 8,
        "restricted report nodes and metadata identities",
    )?;
    let metadata: Vec<_> = chain
        .iter()
        .map(|s| {
            json!({"ordinal":s.anchor.ordinal,
        "catalog":s.anchor.catalog,"receipt":s.anchor.receipt,"manifest":s.receipt.manifest})
        })
        .collect();
    let capture_state = match capture.as_ref() {
        None => "not_present",
        Some(s) if s.cursor.event_count == 0 => "verified_empty_prefix",
        Some(_) => "verified_committed_prefix",
    };
    let checked = phase.is_some();
    let phase_summary = phase.map(|p| p.summary);
    let last = chain.last().expect("nonempty admitted chain");
    let mut output = json!({"schema":"ml-dataset.independent-partial-audit/v1","validator":"independent-partial-lineage/v1",
        "anchor_sha256":expected,"declared_remote":anchor.remote,"terminal_profile":admitted.profile,
        "terminal_ordinal":anchor.ordinal,"binding":last.manifest.binding,
        "snapshot_count":chain.len(),"context_count":admitted.context_count,"artifact_count":admitted.members.len(),
        "artifact_bytes":admitted.artifact_bytes,"logical_object_count":budget.logical_objects,"logical_object_bytes":budget.logical_bytes,
        "actual_reader_bytes":budget.read_bytes,"written_restore_bytes_before_report":budget.written_bytes,"destination_bytes_reserved":budget.disk_reserved,
        "modeled_allocation_limit":super::budget::LIMIT,"modeled_allocation_reserved":budget.used(),
        "phase_modeled_allocation_limit":32*1024*1024,"phase_modeled_allocation_used":budget.phase.used(),
        "allocation_model":"cumulative admitted allocation volume, not RSS; child ceiling reserved once"});
    let verification = json!({
        "metadata_chain_verified":true,"artifact_bytes_verified":true,"authoritative_context_verified":true,
        "committed_event_chain_verified":capture.is_some(),"capture_validation_state":capture_state,
        "cursor":capture.as_ref().map(|s| &s.cursor),"reported_stop":capture.as_ref().and_then(|s| s.stop),
        "reported_observation":last.manifest.observation,"input_metadata":metadata,
        "checked_phase_contract_verified":checked,"checked_phase_validation_state":if checked {"verified_recorded_prefix"} else {"not_applicable_legacy_profile"},
        "checked_phase_contract":phase_summary,"partial_snapshot_chain_verified":true,"restored_locally":restored,
        "fresh_remote_readback_verified":actual_remote,"public_rows":0});
    let boundary = json!({
        "cell_complete":false,"solver_state_restorable":false,"solver_resumed":false,"scientific_result_verified":false,
        "complete_execution_verified":false,"hardware_independently_verified":false,"platform_admitted":false,
        "training_cost_eligible":false,"clock_valid_for_comparison":false,"whole_worker_coverage_verified":false,
        "whole_worker_instruction_count":null,"worker_publication_verified":false,
        "boundary":"caller-pinned structural and byte recovery; no authenticity, event truth, solver resume, hardware truth or worker publication"});
    let limits = json!({"anchor_bytes":super::budget::ANCHOR_MAX,"metadata_bytes":super::budget::METADATA_MAX,
            "artifact_bytes":super::budget::ARTIFACT_MAX,"segment_bytes":super::budget::SEGMENT_MAX,
            "event_line_bytes":super::budget::LINE_MAX,"chain_length":super::budget::CHAIN_MAX,
            "logical_objects":super::budget::OBJECTS_MAX,"logical_bytes":super::budget::LOGICAL_BYTES_MAX,
            "snapshot_artifacts":super::budget::SNAPSHOT_ARTIFACTS,"capture_items":super::budget::MAX_ITEMS,
            "capture_stream_bytes":super::budget::MAX_STREAM,"capture_payload_bytes":super::budget::MAX_STREAM,
            "checked_context_bytes":super::budget::CONTEXT_BYTES_MAX,"report_bytes":REPORT_MAX});
    let Value::Object(verification) = verification else {
        unreachable!("verification object")
    };
    let Value::Object(boundary) = boundary else {
        unreachable!("boundary object")
    };
    output
        .as_object_mut()
        .expect("report object")
        .extend(verification);
    output
        .as_object_mut()
        .expect("report object")
        .extend(boundary);
    output["limits"] = limits;
    Ok(output)
}

pub(crate) fn serialized(report: &Value) -> Result<Vec<u8>> {
    struct Bounded(Vec<u8>);
    impl Write for Bounded {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            if self
                .0
                .len()
                .checked_add(bytes.len())
                .is_none_or(|n| n > REPORT_MAX)
            {
                return Err(std::io::Error::other(
                    "consumer_resource_limit: partial report serialization",
                ));
            }
            self.0.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut raw = Vec::new();
    raw.try_reserve_exact(REPORT_MAX)?;
    let mut out = Bounded(raw);
    serde_json::to_writer_pretty(&mut out, report)?;
    out.write_all(b"\n")?;
    ensure!(
        out.0.len() <= REPORT_MAX,
        "consumer_resource_limit: partial report"
    );
    Ok(out.0)
}
