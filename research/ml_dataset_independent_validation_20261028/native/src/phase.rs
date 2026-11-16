//! Independent bounded validation of sealed, recorded checked-phase evidence.
mod binding;
mod budget;
mod context;
mod io;
mod native;
mod protocol;
mod schema;
#[cfg(test)]
mod tests;

use anyhow::{ensure, Context, Result};
pub(crate) use budget::Budget as PhaseBudget;
use budget::{Budget, LIMIT};
pub(crate) use context::{audit_admitted, reconstruct_admitted, ContextArtifact, ContextReader};
pub(crate) use schema::Binding;
use schema::*;
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs, path::Path};

const POLICY_PATH: &str = "measurement-policy.json";
const POLICY_HASH: &str = "45c9f43fa4df83c6c478094a84478690dd791e27fccaa19af5b87db33f6b4a50";
const POLICY_BYTES: &[u8] =
    include_bytes!("../../../ml_dataset_consumer_checked_phase_20261029/measurement-policy.json");
const USAGE: &str =
    "cumulative worker process; peak RSS is a high-water mark, not an additive phase cost";

pub(crate) struct CheckedPhaseAudit {
    pub measurements: Value,
    pub summary: Value,
}

/// Compatibility extension only for protocol numbers the legacy Value reader
/// cannot represent. Preserve every previously accepted legacy path. The root
/// admission reservation is carried into each checked-cell ledger; it is not a
/// fresh unreported allowance ahead of binding verification.
pub(crate) fn verify_large_protocol(raw: &[u8]) -> Result<usize> {
    let mut budget = Budget::new(LIMIT);
    budget.charge(131072, "protocol preflight fixed envelope")?;
    protocol::verify(raw, &mut budget)?;
    Ok(budget.used())
}
fn role(path: &str) -> Option<&'static str> {
    if path == POLICY_PATH {
        return Some("restricted-measurement-policy");
    }
    for p in Phase::ALL {
        if path == p.path(true) {
            return Some("phase-start");
        }
        if path == p.path(false) {
            return Some("hardware-and-resource-telemetry");
        }
    }
    None
}
fn descriptor(a: &Value) -> Result<()> {
    let fields = ["path", "role", "encoding", "bytes", "sha256"];
    let o = a.as_object().context("phase descriptor object")?;
    ensure!(
        o.len() == 5 && fields.iter().all(|k| o.contains_key(*k)),
        "phase descriptor field set"
    );
    ensure!(
        a["path"].is_string()
            && a["role"].is_string()
            && a["encoding"] == "json"
            && a["bytes"].as_u64().is_some()
            && a["sha256"]
                .as_str()
                .is_some_and(|s| crate::common::valid_hex(s, 64)),
        "phase descriptor type/encoding"
    );
    Ok(())
}
fn relevant(role: &str) -> bool {
    matches!(
        role,
        "restricted-measurement-policy" | "phase-start" | "hardware-and-resource-telemetry"
    )
}

pub(crate) fn audit_sealed(
    root: &Path,
    members: &BTreeMap<String, Value>,
    index: u64,
    plan_sha: &str,
    protocol_preflight_reserved: usize,
) -> Result<Option<CheckedPhaseAudit>> {
    let mut budget = Budget::new(LIMIT);
    budget.charge(
        protocol_preflight_reserved,
        "root protocol preflight carried reservation",
    )?;
    budget.charge(131072, "finite bookkeeping")?;
    let present = match fs::symlink_metadata(root.join(POLICY_PATH)) {
        Ok(_) => true,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => false,
        Err(e) => return Err(e.into()),
    };
    ensure!(
        !present || members.contains_key(POLICY_PATH),
        "phase policy is not manifest-admitted"
    );
    let checked = present || members.contains_key(POLICY_PATH);
    let mut descriptors = 0;
    for (path, a) in members {
        let declared = a["role"].as_str().context("phase artifact role")?;
        if !checked {
            ensure!(
                declared != "restricted-measurement-policy",
                "phase policy role at unexpected path"
            );
            if path.starts_with("telemetry/") && path.ends_with(".json") {
                // Legacy probing has no retained output. Its per-file fixed
                // scratch is released, independent of the checked-cell ledger.
                ensure!(
                    !io::legacy_has_v2(root, a, &mut Budget::new(LIMIT))?,
                    "phase v2 missing policy"
                );
            }
            continue;
        }
        if relevant(declared)
            || (path.starts_with("telemetry/") && path.ends_with(".json"))
            || path == POLICY_PATH
        {
            ensure!(role(path) == Some(declared), "phase unexpected path/role");
            descriptor(a)?;
            ensure!(descriptors < 9, "phase artifact count");
            descriptors += 1;
        }
    }
    if !checked {
        return Ok(None);
    }
    let policy = members.get(POLICY_PATH).context("phase policy absent")?;
    let raw = io::bytes(root, policy, 16384, &mut budget)?;
    ensure!(
        raw == POLICY_BYTES && crate::common::digest(&raw) == POLICY_HASH,
        "phase fixed policy bytes"
    );
    let p: FixedPolicy = io::typed(&raw, &mut budget)?;
    // Byte identity fixes every field; typed parsing additionally enforces inventory.
    ensure!(
        p.schema == "crypto.ml-dataset.measurement-policy/v1"
            && p.policy_id == "checked-calling-thread-diagnostic-v1"
            && p.collector_version == "checked-native-pmu/v1"
            && p.task_scope == "calling_thread"
            && p.cycles == "optional_with_standalone_fallback"
            && p.counter_interval == "successful enable through disable including wrapper work"
            && p.usage_scope == "cumulative_worker_process"
            && p.estimate_rule == "floor_u128_raw_times_enabled_over_running_checked_u64"
            && p.max_policy_bytes == 16384
            && p.max_start_bytes == 65536
            && p.max_metrics_bytes == 524288
            && p.max_error_preview_bytes == 4096
            && p.max_attempts == 2
            && p.max_events_per_attempt == 2
            && p.max_calls_per_event == 12
            && p.max_context_checks == 2
            && !p.training_cost_eligible
            && !p.whole_worker_coverage_verified
            && !p.hardware_independently_verified
            && !p.platform_admitted
            && !p.clock_valid_for_comparison,
        "phase policy fields"
    );
    let bound = binding::reconstruct(root, members, index, plan_sha, &mut budget)?;
    let mut records = Vec::with_capacity(4);
    let mut identities = Vec::with_capacity(9);
    identities.push(policy.clone());
    for phase in Phase::ALL {
        let s = members.get(&phase.path(true));
        let e = members.get(&phase.path(false));
        ensure!(s.is_some() || e.is_none(), "phase end without start");
        if let Some(a) = s {
            let raw = io::bytes(root, a, 65536, &mut budget)?;
            let start = io::typed::<Start>(&raw, &mut budget)?;
            let start_hash = crate::common::digest(&raw);
            let start_value = crate::common::read_json_bytes(&raw)?;
            identities.push(a.clone());
            let end = if let Some(a) = e {
                let raw = io::bytes(root, a, 524288, &mut budget)?;
                let metrics = io::typed::<Metrics>(&raw, &mut budget)?;
                let value = crate::common::read_json_bytes(&raw)?;
                identities.push(a.clone());
                Some((metrics, value, crate::common::digest(&raw)))
            } else {
                None
            };
            records.push((phase, start, start_value, start_hash, end));
        }
    }
    Ok(Some(
        validate_context(bound, records, identities, &budget)?.sealed(),
    ))
}

type Recorded = (
    Phase,
    Start,
    Value,
    String,
    Option<(Metrics, Value, String)>,
);
fn usage(u: &Option<Usage>) -> Result<()> {
    ensure!(u.as_ref().is_none_or(|u|u.scope==USAGE && matches!(u.peak_rss_unit.as_str(),"bytes"|"KiB")),"phase usage scope/unit");
    Ok(())
}
fn error_summary(e: &Option<ErrorSummary>) -> Result<&'static str> {
    let Some(e) = e else {
        return Ok("not_supplied");
    };
    ensure!(
        e.message_preview.len() <= 4096
            && crate::common::valid_hex(&e.message_sha256, 64)
            && e.message_bytes >= e.message_preview.len() as u64
            && e.truncated == (e.message_bytes > e.message_preview.len() as u64),
        "phase error summary bounds"
    );
    if e.truncated {
        Ok("truncated_suffix_unverified")
    } else {
        ensure!(
            crate::common::digest(e.message_preview.as_bytes()) == e.message_sha256,
            "phase error summary digest"
        );
        Ok("full_preview_checked")
    }
}
fn validate_context(
    bound: Binding,
    records: Vec<Recorded>,
    identities: Vec<Value>,
    budget: &Budget,
) -> Result<ValidatedContext> {
    let mut allow_next = true;
    let mut phases = Vec::with_capacity(4);
    let mut measurements = Vec::with_capacity(4);
    let mut ends = 0;
    let mut state = "policy_only";
    for (position, (phase, start, start_value, start_hash, end)) in records.into_iter().enumerate()
    {
        ensure!(
            allow_next && position == usize::from(phase.ordinal()),
            "phase dependency prefix gap or continuation after failure"
        );
        ensure!(
            start.schema == "crypto.ml-dataset.phase-start/v2"
                && start.phase == phase
                && start.ordinal == phase.ordinal()
                && start.binding == bound
                && start.policy_sha256 == POLICY_HASH
                && start.recording_pid > 0
                && start.counter_boundary == "durable_start_before_open",
            "phase start binding/identity"
        );
        usage(&start.usage_before)?;
        let mut measure = json!({"phase":phase.name(),"start_sha256":start_hash,"start":start_value,"metrics_sha256":null,
            "producer_reported":null,"measurement_state":"interrupted_or_unfinished","independently_verified":false});
        let mut summary = json!({"phase":phase.name(),"ordinal":phase.ordinal(),"start_sha256":start_hash,"end_sha256":null,
            "state":"interrupted_or_unfinished","start_binding_checked":true,"lifecycle_checked":false,"projection_checked":false,"error_message_check":"not_supplied"});
        if let Some((m, value, sha)) = end {
            ensure!(
                m.schema == "crypto.ml-dataset.phase-metrics/v2"
                    && m.phase == phase
                    && m.ordinal == phase.ordinal()
                    && m.binding == bound
                    && m.policy_sha256 == POLICY_HASH
                    && m.start_sha256 == start_hash
                    && m.returned_ok == m.returned_error.is_none(),
                "phase end binding/operation state"
            );
            ensure!(
                !m.training_cost_eligible
                    && m.whole_worker_retired_instructions.is_none()
                    && !m.whole_worker_coverage_verified
                    && !m.independently_verified
                    && !m.platform_admitted
                    && !m.clock_valid_for_comparison
                    && !m.interrupted_phase_readback
                    && !m.consumer_v2_admitted,
                "phase prohibited admission claim"
            );
            let elapsed = m
                .elapsed_ns_diagnostic
                .parse::<u128>()
                .context("phase elapsed diagnostic")?;
            ensure!(
                elapsed.to_string() == m.elapsed_ns_diagnostic,
                "phase noncanonical elapsed diagnostic"
            );
            usage(&m.usage_after)?;
            let error_check = error_summary(&m.returned_error)?;
            let (started, observations) = native::validate(&m.collection, start.recording_pid)?;
            ensure!(
                started == m.counter_start_succeeded && observations == m.observations,
                "phase projection/start disagreement"
            );
            allow_next = m.returned_ok;
            state = if allow_next {
                "completed_recorded_prefix"
            } else {
                "operation_error"
            };
            ends += 1;
            measure["metrics_sha256"] = json!(sha);
            measure["producer_reported"] = value;
            measure["measurement_state"] = json!("reported_phase_readback");
            summary["end_sha256"] = json!(sha);
            summary["state"] = json!("recorded_end_checked");
            summary["lifecycle_checked"] = json!(true);
            summary["projection_checked"] = json!(true);
            summary["error_message_check"] = json!(error_check);
        } else {
            allow_next = false;
            state = "interrupted_or_unfinished";
        }
        phases.push(summary);
        measurements.push(measure);
    }
    Ok(ValidatedContext {
        bound,
        identities,
        phases,
        measurements,
        ends,
        state,
        reserved: budget.used(),
    })
}

/// Semantic result has no sealed/partial transport claim. Presentations own it.
struct ValidatedContext {
    bound: Binding,
    identities: Vec<Value>,
    phases: Vec<Value>,
    measurements: Vec<Value>,
    ends: usize,
    state: &'static str,
    reserved: usize,
}
impl ValidatedContext {
    fn sealed(self) -> CheckedPhaseAudit {
        let summary = json!({"schema":"ml-dataset.checked-phase-contract/v1","validator":"independent-sealed-phase-v2/v1",
            "policy_sha256":POLICY_HASH,"binding":self.bound,"input_artifacts":self.identities,"supplied_starts":self.phases.len(),"supplied_ends":self.ends,
            "prefix_state":self.state,"phases":self.phases,"authoritative_binding_checked":true,"modeled_storage_limit":LIMIT,"modeled_storage_reserved":self.reserved,
            "independently_verified":false,"hardware_independently_verified":false,"platform_admitted":false,"training_cost_eligible":false,
            "complete_execution_verified":false,"clock_valid_for_comparison":false,"whole_worker_instruction_count":null,"partial_snapshot_chain_verified":false});
        CheckedPhaseAudit {
            measurements: Value::Array(self.measurements),
            summary,
        }
    }
    fn partial(self) -> CheckedPhaseAudit {
        let mut projection = self.sealed();
        projection.summary["validator"] = json!("independent-partial-phase-v2/v1");
        projection
            .summary
            .as_object_mut()
            .expect("summary object")
            .remove("partial_snapshot_chain_verified");
        projection
    }
}
