//! Fail-closed retained-attempt cohort exporter.
//!
//! This module is postrun-only. It opens every supplied pin, reconstructs the
//! exact seven-plan / 1,260-workload / 22,680-slot enrollment, emits explicit missing slots, retains retries
//! without substituting them for primaries, and derives finite prices only by
//! calling the registered nine-category mapper. It does not launch, solve,
//! replay, authenticate an external controller, or invent a missing cost.
use super::{
    canonical::{reject_duplicate_keys, sha256_hex},
    diagnostic_launch::resolved_method_binding,
    diagnostic_resources::{PublicationTail, ResourcePrefix, Status},
    diagnostics,
    pencil_progress_sink,
    record::{check_line_seal, DiagnosticChild, Record},
};
use pencil_accounting_price_draft::analyzer;
use pencil_symbolic_graph::relation_stream as relation;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::{Component, Path},
};

pub const REQUEST_SCHEMA: &str = "ecbench.pencil-attempt-export-request/v3";
pub const OUTPUT_SCHEMA: &str = "ecbench.pencil-attempt-cohort/v3";
pub const PREPARED_RECEIPT_SHA256: &str =
    "1892db0ad5937bba74894c5be9f3b6f1e48dcf20ddb0284f043686a2f822ef1e";
pub const MAX_REQUEST_BYTES: usize = 256 * 1024 * 1024;
const MAX_JSON_BYTES: u64 = 64 * 1024 * 1024;
const MAX_RELATION_BYTES: u64 = 4 * 1024 * 1024 * 1024;
const MAX_BINARY_BYTES: u64 = 4 * 1024 * 1024 * 1024;

const PLAN_INVENTORY: [(&str, &str); 7] = [
    ("plan-16.json", "35e6311a7105a29c2459543ad0b281ed91e43aa63a99f66226f95a1f3c9cc2ba"),
    ("plan-20.json", "51a99e83c38b4d5d6b19f82337918b2f64f34b4934a5361298e84fc2f8bd432f"),
    ("plan-24.json", "7d34d0ce87c5a167ee1e94fecb08bcac34c9b4339570deb6377effe75efc28c4"),
    ("plan-28.json", "87760bc68b7598250c22976f15e4dbb36b5f2d738647de501222de1a8b27b001"),
    ("plan-32.json", "a9aebbc3120b61380a6936408635ac0f0e60715b4b4918dc826fbab4b578ae96"),
    ("plan-36.json", "6bb162d846bfbfe68b1bf7ba330abbc1e11c403b003215c960e7e71d81be3aa2"),
    ("plan-40.json", "8f2ec73a139f84b660573bd67c54ebfc567626ce7068a135dcb52c61891b22d5"),
];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FilePin {
    pub path: String,
    pub sha256: String,
    pub bytes: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanPin {
    pub name: String,
    pub session_id: String,
    pub file: FilePin,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionCustody {
    pub session_id: String,
    /// Exact `ecbench.session/v1` bytes bind the session to its prepared plan
    /// and checked execution binary.
    pub native_session: FilePin,
    /// Existing native owner observation: `isolab.fresh-owner-observation/v1`
    /// or retained recovery owner exit `isolab.recovery.owner-exit/v1`.
    pub process: Option<FilePin>,
    /// Existing controller association: `isolab.fresh-owner-controller-check/v1`.
    pub controller: Option<FilePin>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CohortPins {
    pub prepared_plan_validation: FilePin,
    pub source_manifest: FilePin,
    pub binary: FilePin,
    pub protocol: FilePin,
    pub coverage_review: FilePin,
    pub plans: Vec<PlanPin>,
    pub sessions: Vec<SessionCustody>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AttemptRef {
    pub session_id: String,
    pub sequence: u64,
    pub attempt: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RelationEvidence {
    pub stream: FilePin,
    pub totals: FilePin,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceEvidence {
    pub prefix: FilePin,
    pub tail: FilePin,
}

/// Only routing identity and file locations are caller supplied. Scientific
/// identity, terminal state, eligibility and cost are derived from checked
/// bytes. Unknown `eligible_for_pair` or `raw_costs` fields are rejected by
/// `deny_unknown_fields`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AttemptEvidence {
    pub session_id: String,
    pub sequence: u64,
    pub attempt: u64,
    pub retry_of: Option<AttemptRef>,
    pub context: FilePin,
    pub ordinary_record: Option<FilePin>,
    pub progress_files: Vec<FilePin>,
    pub progress_totals: Option<FilePin>,
    pub diagnostics_attachment: Option<FilePin>,
    pub relation: Option<RelationEvidence>,
    pub resources: Option<ResourceEvidence>,
    /// Per-execution immutable `ecbench.session/v1` snapshot bound by the
    /// resource prefix. This is distinct from the final session file.
    pub session_snapshot: Option<FilePin>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExportRequest {
    pub schema: String,
    pub cohort: CohortPins,
    pub attempts: Vec<AttemptEvidence>,
}

#[derive(Clone, Debug)]
struct Slot {
    plan_name: String,
    plan_sha256: String,
    session_id: String,
    sequence: u64,
    round: u64,
    warmup: bool,
    arm: String,
    role: String,
    algorithm_seed: u64,
    workload: Value,
    method: Value,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SlotKey {
    session_id: String,
    sequence: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct TripletKey {
    plan_name: String,
    session_id: String,
    workload_id: String,
    round: u64,
    seed: u64,
}

#[derive(Clone, Debug)]
struct CheckedAttempt {
    input: AttemptEvidence,
    slot: Slot,
    context: diagnostics::Context,
    record: Option<Value>,
    record_id: Option<String>,
    evidence_complete: bool,
    custody_closed: bool,
    stream_complete: Option<bool>,
    progress_diagnostic_complete: bool,
    diagnostic_prefix_complete: bool,
    reasons: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RelationTotals {
    schema: String,
    path: String,
    sha256: String,
    bytes: u64,
    synced_records: u64,
    write_ns: u128,
    sync_ns: u128,
    complete: bool,
    failure: Option<String>,
}

fn canonical_hex(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn checked_path(path: &str) -> Result<&Path, String> {
    let path = Path::new(path);
    if !path.is_absolute()
        || path.components().any(|component| {
            !matches!(component, Component::RootDir | Component::Normal(_))
        })
    {
        return Err("file pin requires absolute traversal-free path".into());
    }
    Ok(path)
}

fn checked_file(pin: &FilePin, max_bytes: u64) -> Result<Vec<u8>, String> {
    if !canonical_hex(&pin.sha256) || pin.bytes > max_bytes {
        return Err("invalid file pin digest or byte bound".into());
    }
    let path = checked_path(&pin.path)?;
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    let mut file = options.open(path).map_err(|error| error.to_string())?;
    let metadata = file.metadata().map_err(|error| error.to_string())?;
    if !metadata.is_file() || metadata.len() != pin.bytes || metadata.len() > max_bytes {
        return Err(format!("file pin size/type mismatch: {}", pin.path));
    }
    let reserve = usize::try_from(metadata.len())
        .map_err(|_| "file too large for address space")?;
    let mut bytes = Vec::with_capacity(reserve);
    file.read_to_end(&mut bytes).map_err(|error| error.to_string())?;
    if bytes.len() as u64 != pin.bytes || sha256_hex(&bytes) != pin.sha256 {
        return Err(format!("file pin byte/hash mismatch: {}", pin.path));
    }
    Ok(bytes)
}

fn checked_digest_only(pin: &FilePin, max_bytes: u64) -> Result<(), String> {
    if !canonical_hex(&pin.sha256) || pin.bytes > max_bytes {
        return Err("invalid file pin digest or byte bound".into());
    }
    let path = checked_path(&pin.path)?;
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    let mut file = options.open(path).map_err(|error| error.to_string())?;
    let metadata = file.metadata().map_err(|error| error.to_string())?;
    if !metadata.is_file() || metadata.len() != pin.bytes || metadata.len() > max_bytes {
        return Err(format!("file pin size/type mismatch: {}", pin.path));
    }
    let mut digest = Sha256::new();
    let mut total = 0u64;
    let mut buffer = [0u8; 1024 * 1024];
    loop {
        let read = file.read(&mut buffer).map_err(|error| error.to_string())?;
        if read == 0 {
            break;
        }
        total = total.checked_add(read as u64).ok_or("file byte overflow")?;
        if total > max_bytes {
            return Err("file pin byte bound".into());
        }
        digest.update(&buffer[..read]);
    }
    if total != pin.bytes || format!("{:x}", digest.finalize()) != pin.sha256 {
        return Err(format!("file pin byte/hash mismatch: {}", pin.path));
    }
    Ok(())
}

fn checked_json(pin: &FilePin, cap: u64) -> Result<(Vec<u8>, Value), String> {
    let bytes = checked_file(pin, cap)?;
    let text = std::str::from_utf8(&bytes).map_err(|error| error.to_string())?;
    reject_duplicate_keys(text)?;
    let value = serde_json::from_str(text).map_err(|error| error.to_string())?;
    Ok((bytes, value))
}

fn array<'a>(value: &'a Value, pointer: &str) -> Result<&'a Vec<Value>, String> {
    value
        .pointer(pointer)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("missing array {pointer}"))
}

fn text<'a>(value: &'a Value, pointer: &str) -> Result<&'a str, String> {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing string {pointer}"))
}

fn number(value: &Value, pointer: &str) -> Result<u64, String> {
    value
        .pointer(pointer)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("missing u64 {pointer}"))
}

fn bool_field(value: &Value, pointer: &str) -> Result<bool, String> {
    value
        .pointer(pointer)
        .and_then(Value::as_bool)
        .ok_or_else(|| format!("missing bool {pointer}"))
}

fn expected_plan_map() -> BTreeMap<&'static str, &'static str> {
    PLAN_INVENTORY.into_iter().collect()
}

fn validate_prepared_receipt(pin: &FilePin) -> Result<Value, String> {
    if pin.sha256 != PREPARED_RECEIPT_SHA256 {
        return Err("prepared-plan receipt is not the frozen receipt".into());
    }
    let (_, receipt) = checked_json(pin, MAX_JSON_BYTES)?;
    if text(&receipt, "/schema")? != "pencil-prepared-plan-validation/v1"
        || receipt.pointer("/passed") != Some(&Value::Bool(true))
        || number(&receipt, "/targets")? != 1_260
        || number(&receipt, "/planned_executions")? != 22_680
        || number(&receipt, "/measured_executions")? != 18_900
        || number(&receipt, "/warmup_executions")? != 3_780
    {
        return Err("prepared-plan receipt status or geometry mismatch".into());
    }
    let files = receipt
        .pointer("/files")
        .and_then(Value::as_object)
        .ok_or("prepared-plan receipt files missing")?;
    let expected = expected_plan_map();
    if files.len() != expected.len()
        || expected.iter().any(|(name, digest)| {
            files.get(*name).and_then(Value::as_str) != Some(*digest)
        })
    {
        return Err("prepared-plan receipt inventory mismatch".into());
    }
    Ok(receipt)
}

fn workload_id(workload: &Value) -> Result<&str, String> {
    text(workload, "/workload_id")
}

fn workload_sha(workload: &Value) -> Result<&str, String> {
    let digest = text(workload, "/workload_sha256")?;
    if !canonical_hex(digest) {
        return Err("plan workload SHA256 is not canonical".into());
    }
    Ok(digest)
}

fn workload_binding(workload: &Value) -> Result<String, String> {
    Ok(sha256_hex(
        &serde_json::to_vec(workload).map_err(|error| error.to_string())?,
    ))
}

fn build_slots(cohort: &CohortPins) -> Result<(BTreeMap<SlotKey, Slot>, Value), String> {
    let receipt = validate_prepared_receipt(&cohort.prepared_plan_validation)?;
    let expected = expected_plan_map();
    if cohort.plans.len() != expected.len() {
        return Err("exact seven plan pins required".into());
    }
    let mut slots = BTreeMap::new();
    let mut plan_names = BTreeSet::new();
    let mut sessions = BTreeSet::new();
    let mut workload_ids = BTreeSet::new();
    let mut measured = 0usize;
    let mut warmups = 0usize;
    for plan_pin in &cohort.plans {
        let expected_digest = expected
            .get(plan_pin.name.as_str())
            .ok_or("unregistered prepared plan name")?;
        if plan_pin.file.sha256 != *expected_digest
            || plan_pin.session_id.is_empty()
            || !plan_names.insert(plan_pin.name.clone())
            || !sessions.insert(plan_pin.session_id.clone())
        {
            return Err("plan name/hash/session inventory mismatch".into());
        }
        let (_, plan) = checked_json(&plan_pin.file, MAX_JSON_BYTES)?;
        if text(&plan, "/schema")? != "ecbench.plan/v1" {
            return Err("unsupported prepared plan schema".into());
        }
        let arms = array(&plan, "/arms")?;
        if arms.len() != 3
            || arms[0]["name"] != json!("pencil")
            || arms[0]["role"] != json!("candidate")
            || arms[0]["method"]["id"] != json!("ic.three_leads_genus2_pencil_symbolic_v2")
            || arms[1]["name"] != json!("rho")
            || arms[1]["role"] != json!("reference")
            || arms[1]["method"]["id"] != json!("rho.three_leads_negation_v1")
            || arms[2]["name"] != json!("rho_AA")
            || arms[2]["role"] != json!("control")
            || arms[2]["method"] != arms[1]["method"]
        {
            return Err("prepared plan three-arm design mismatch".into());
        }
        let workloads = array(&plan, "/workloads")?;
        let executions = array(&plan, "/executions")?;
        if workloads.len() != 180 || executions.len() != 3_240 {
            return Err("per-plan workload/execution geometry mismatch".into());
        }
        let mut groups: BTreeMap<(usize, bool, u64), BTreeSet<String>> = BTreeMap::new();
        let mut group_seeds: BTreeMap<(usize, bool, u64), u64> = BTreeMap::new();
        let mut per_workload: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
        for workload in workloads {
            let id = workload_id(workload)?;
            workload_sha(workload)?;
            if !workload_ids.insert(id.to_owned()) {
                return Err("workload repeated across prepared plans".into());
            }
        }
        for execution in executions {
            let workload_index = usize::try_from(number(execution, "/workload")?)
                .map_err(|_| "workload index overflow")?;
            let arm_index = usize::try_from(number(execution, "/arm")?)
                .map_err(|_| "arm index overflow")?;
            let workload = workloads
                .get(workload_index)
                .ok_or("plan workload index out of range")?;
            let arm = arms.get(arm_index).ok_or("plan arm index out of range")?;
            let sequence = number(execution, "/seq")?;
            let round = number(execution, "/round")?;
            let warmup = bool_field(execution, "/warmup")?;
            let seed = number(execution, "/algorithm_seed")?;
            let arm_name = text(arm, "/name")?.to_owned();
            let group_key = (workload_index, warmup, round);
            if group_seeds
                .insert(group_key, seed)
                .is_some_and(|old| old != seed)
            {
                return Err("prepared plan seeds differ within workload round".into());
            }
            if !groups
                .entry(group_key)
                .or_default()
                .insert(arm_name.clone())
            {
                return Err("duplicate arm in plan workload round".into());
            }
            let counts = per_workload.entry(workload_index).or_default();
            if warmup {
                warmups += 1;
                counts.1 += 1;
            } else {
                measured += 1;
                counts.0 += 1;
            }
            let key = SlotKey {
                session_id: plan_pin.session_id.clone(),
                sequence,
            };
            let slot = Slot {
                plan_name: plan_pin.name.clone(),
                plan_sha256: plan_pin.file.sha256.clone(),
                session_id: plan_pin.session_id.clone(),
                sequence,
                round,
                warmup,
                arm: arm_name,
                role: text(arm, "/role")?.to_owned(),
                algorithm_seed: seed,
                workload: workload.clone(),
                method: arm["method"].clone(),
            };
            if slots.insert(key, slot).is_some() {
                return Err("duplicate plan session/sequence slot".into());
            }
        }
        let expected_arms: BTreeSet<String> =
            ["pencil", "rho", "rho_AA"].into_iter().map(str::to_owned).collect();
        if groups.len() != workloads.len() * 6
            || groups.values().any(|arms| arms != &expected_arms)
            || per_workload.len() != workloads.len()
            || per_workload.values().any(|(measured, warmup)| *measured != 15 || *warmup != 3)
        {
            return Err("plan lacks five measured and one warmup three-arm rounds per workload".into());
        }
    }
    if slots.len() != 22_680
        || workload_ids.len() != 1_260
        || measured != 18_900
        || warmups != 3_780
    {
        return Err("full prepared cohort geometry mismatch".into());
    }
    Ok((slots, receipt))
}

fn validate_protocol_and_sources(cohort: &CohortPins) -> Result<Value, String> {
    checked_digest_only(&cohort.binary, MAX_BINARY_BYTES)?;
    let (_, source) = checked_json(&cohort.source_manifest, MAX_JSON_BYTES)?;
    if source.pointer("/schema").and_then(Value::as_str).is_none() {
        return Err("source manifest schema missing".into());
    }
    let (_, protocol) = checked_json(&cohort.protocol, MAX_JSON_BYTES)?;
    if text(&protocol, "/schema")? != "experiment.protocol/v1"
        || text(&protocol, "/protocol_id")? != "PENCIL-SYMBOLIC-PAIRED-1260-20261029"
        || number(&protocol, "/sample_size/instances")? != 1_260
        || number(&protocol, "/sample_size/replicates")? != 5
        || number(&protocol, "/sample_size/warmups")? != 1
    {
        return Err("frozen protocol identity or cohort geometry mismatch".into());
    }
    let (_, review) = checked_json(&cohort.coverage_review, MAX_JSON_BYTES)?;
    if text(&review, "/schema")? != "pencil-algorithm-coverage-review/v1"
        || text(&review, "/protocol_sha256")? != cohort.protocol.sha256
        || text(&review, "/protocol_path")? != cohort.protocol.path
    {
        return Err("coverage review does not bind exact protocol pin".into());
    }
    Ok(review)
}

// A killed child can leave diagnostics for the next unwritten slot. Preserve
// that boundary attempt, but never accept an ordinary record outside the
// session's durable written prefix or observations beyond its next slot.
fn validate_observed_written_prefix(session: &str, written: u64, attempts: &[AttemptEvidence]) -> Result<(), String> {
    for attempt in attempts.iter().filter(|attempt| attempt.session_id == session) {
        if attempt.sequence > written
            || (attempt.sequence == written && attempt.ordinary_record.is_some()) {
            return Err("observed attempt exceeds native session written prefix".into());
        }
    }
    Ok(())
}

fn session_custody(cohort: &CohortPins, attempts: &[AttemptEvidence]) -> Result<BTreeMap<String, (bool, String)>, String> {
    let plan_sessions: BTreeMap<String, String> = cohort
        .plans
        .iter()
        .map(|plan| (plan.session_id.clone(), plan.file.sha256.clone()))
        .collect();
    let mut result = BTreeMap::new();
    for session in &cohort.sessions {
        if result.contains_key(&session.session_id) {
            return Err("duplicate session custody identity".into());
        }
        let (_, native_session) = checked_json(&session.native_session, MAX_JSON_BYTES)?;
        let plan_sha256 = text(&native_session, "/plan_sha256")?;
        if !cohort
            .plans
            .iter()
            .any(|plan| plan.file.sha256 == plan_sha256)
            || plan_sessions
                .get(&session.session_id)
                .is_some_and(|primary| primary != plan_sha256)
        {
            return Err("native session plan is not its registered frozen plan".into());
        }
        if text(&native_session, "/schema")? != "ecbench.session/v1"
            || text(&native_session, "/session_id")? != session.session_id
            || text(&native_session, "/binary_sha256")? != cohort.binary.sha256
            || !matches!(text(&native_session, "/status")?, "complete" | "running" | "interrupted")
            || number(&native_session, "/executions_planned")? != 3_240
            || number(&native_session, "/records_written")? > 3_240
        {
            return Err("native session does not bind exact plan/session/binary geometry".into());
        }
        let written = number(&native_session, "/records_written")?;
        validate_observed_written_prefix(&session.session_id, written, attempts)?;
        let expected_last = if written == 0 { Value::Null } else { json!(written - 1) };
        if native_session["last_seq"] != expected_last
            || (native_session["status"] == "complete" && written != 3_240)
        { return Err("native session prefix geometry mismatch".into()); }
        // Missing ownership bytes are retained, never promoted to closure.
        let Some(process_pin) = &session.process else {
            if let Some(pin) = &session.controller {
                let (_, controller) = checked_json(pin, MAX_JSON_BYTES)?;
                if text(&controller,"/schema")? != "isolab.fresh-owner-controller-check/v1" {
                    return Err("unsupported controller custody schema".into());
                }
            }
            result.insert(session.session_id.clone(), (false, plan_sha256.to_owned()));
            continue;
        };
        let (_, process) = checked_json(process_pin, MAX_JSON_BYTES)?;
        let process_schema = text(&process, "/schema")?;
        if !matches!(
            process_schema,
            "isolab.fresh-owner-observation/v1" | "isolab.recovery.owner-exit/v1"
        ) {
            return Err("unsupported native owner process schema".into());
        }
        if process_schema == "isolab.fresh-owner-observation/v1"
            && text(&process, "/binding/binary_sha256")? != cohort.binary.sha256
        {
            return Err("owner process binary differs from checked binary".into());
        }
        let Some(controller_pin) = &session.controller else {
            result.insert(session.session_id.clone(), (false, plan_sha256.to_owned()));
            continue;
        };
        let (_, controller) = checked_json(controller_pin, MAX_JSON_BYTES)?;
        if text(&controller, "/schema")? != "isolab.fresh-owner-controller-check/v1" {
            return Err("unsupported controller custody schema".into());
        }
        let records = array(&controller, "/records")?;
        let process_config = process.pointer("/config_sha256").and_then(Value::as_str);
        if !records.iter().any(|record| {
            record.pointer("/receipt_sha256").and_then(Value::as_str)
                == Some(process_pin.sha256.as_str())
                && process_config.is_none_or(|config| {
                    record.pointer("/config_sha256").and_then(Value::as_str) == Some(config)
                })
        }) {
            return Err("controller custody does not bind owner process bytes".into());
        }
        if let (
            Some(process_request),
            Some(controller_request),
        ) = (
            process.pointer("/binding/request_sha256").and_then(Value::as_str),
            controller.pointer("/request_sha256").and_then(Value::as_str),
        ) {
            if process_request != controller_request {
                return Err("controller request differs from owner process binding".into());
            }
        }
        let reaped = process.pointer("/reap_observed") == Some(&Value::Bool(true));
        let exit_zero = process.pointer("/exit_code").and_then(Value::as_i64) == Some(0);
        let no_findings = process
            .pointer("/findings")
            .and_then(Value::as_array)
            .is_some_and(Vec::is_empty);
        let externally_closed = !controller
            .pointer("/accepted_external_closure")
            .is_none_or(Value::is_null)
            && !process
                .pointer("/accepted_external_closure")
                .is_none_or(Value::is_null);
        result.insert(
            session.session_id.clone(),
            (
                reaped && exit_zero && no_findings && externally_closed
                    && native_session["status"] == "complete",
                plan_sha256.to_owned(),
            ),
        );
    }
    for (session, plan) in plan_sessions {
        result.entry(session).or_insert((false, plan));
    }
    Ok(result)
}

fn one_record(pin: &FilePin) -> Result<(Value, Record), String> {
    let bytes = checked_file(pin, MAX_JSON_BYTES)?;
    let raw = std::str::from_utf8(&bytes).map_err(|error| error.to_string())?;
    if !raw.ends_with('\n') || raw.ends_with("\r\n") {
        return Err("ordinary record must retain the native single LF terminator".into());
    }
    let line = &raw[..raw.len() - 1];
    if line.is_empty() || line.contains('\n') || line.contains('\r') {
        return Err("ordinary record pin must contain exactly one native JSONL record".into());
    }
    reject_duplicate_keys(line)?;
    let value: Value = serde_json::from_str(line).map_err(|error| error.to_string())?;
    let record: Record = serde_json::from_value(value.clone()).map_err(|error| error.to_string())?;
    if record.schema != "ecbench.record/v1" || !check_line_seal(line, &record.record_id) {
        return Err("ordinary record schema or byte seal mismatch".into());
    }
    Ok((value, record))
}

fn expected_value(slot: &Slot, binary_sha256: &str) -> Value {
    json!({
        "session": slot.session_id,
        "binary_sha256": binary_sha256,
        "seq": slot.sequence,
        "round": slot.round,
        "warmup": slot.warmup,
        "arm": slot.arm,
        "role": slot.role,
        "seed": slot.algorithm_seed.to_string(),
        "workload": slot.workload,
        "method": slot.method,
    })
}

fn validate_context(
    attempt: &AttemptEvidence,
    slot: &Slot,
    cohort: &CohortPins,
) -> Result<DiagnosticChild, String> {
    let (context_bytes, value) = checked_json(
        &attempt.context,
        super::record::MAX_DIAGNOSTIC_CHILD_BYTES as u64,
    )?;
    let child: DiagnosticChild =
        serde_json::from_value(value).map_err(|error| error.to_string())?;
    let mut expected_bytes = serde_json::to_vec_pretty(&child).map_err(|error| error.to_string())?;
    expected_bytes.push(b'\n');
    if context_bytes != expected_bytes {
        return Err("diagnostic child file is not the native pretty-JSON bytes".into());
    }
    child.envelope.validate(&child.launch)?;
    let context = &child.envelope.prefix;
    context.validate()?;
    let identity = &context.identity;
    if identity.session != attempt.session_id
        || identity.execution != slot.sequence
        || identity.attempt != attempt.attempt
        || identity.seed != slot.algorithm_seed
        || identity.workload_sha256 != workload_binding(&slot.workload)?
        || identity.binary_sha256 != cohort.binary.sha256
        || identity.protocol_sha256 != cohort.protocol.sha256
        || identity.source_sha256 != cohort.source_manifest.sha256
    {
        return Err("diagnostic context differs from plan/custody identity".into());
    }
    if child.envelope.arm.name != slot.arm
        || serde_json::to_value(&child.envelope.arm.role).map_err(|error| error.to_string())?
            != Value::String(slot.role.clone())
        || child.envelope.arm.method != text(&slot.method, "/id")?
        || child.envelope.arm.method_config_sha256 != identity.method_config_sha256
    {
        return Err("diagnostic child arm differs from exact prepared slot".into());
    }
    Ok(child)
}

fn validate_record(
    pin: &FilePin,
    context: &diagnostics::Context,
    slot: &Slot,
    cohort: &CohortPins,
) -> Result<(Value, Record), String> {
    let (value, record) = one_record(pin)?;
    if record.session_id != context.identity.session
        || record.seq != slot.sequence
        || u64::from(record.round) != slot.round
        || record.warmup != slot.warmup
        || record.arm != slot.arm
        || record.role != slot.role
        || record.algorithm_seed != slot.algorithm_seed.to_string()
        || serde_json::to_value(&record.workload).map_err(|error| error.to_string())?
            != slot.workload
        || serde_json::to_value(&record.method).map_err(|error| error.to_string())?
            != slot.method
        || record.binary_sha256.as_deref() != Some(cohort.binary.sha256.as_str())
    {
        return Err("ordinary record differs from exact prepared slot".into());
    }
    let method_binding = resolved_method_binding(&record.method)?;
    if method_binding != context.identity.method_config_sha256 {
        return Err("record method differs from diagnostic method binding".into());
    }
    Ok((value, record))
}

fn record_is_verified_measured(value: &Value) -> bool {
    value.pointer("/warmup") == Some(&Value::Bool(false))
        && value.pointer("/outcome/status").and_then(Value::as_str) == Some("verified")
        && value.pointer("/outcome/matches_target") == Some(&Value::Bool(true))
        && value.pointer("/outcome/matches_planted") == Some(&Value::Bool(true))
        && value.pointer("/outcome/error") == Some(&Value::Null)
}

fn validate_progress(
    attempt: &AttemptEvidence,
    context: &diagnostics::Context,
) -> Result<bool, String> {
    if attempt.progress_files.len() > pencil_progress_sink::MAX_SNAPSHOTS as usize {
        return Err("progress snapshot count bound".into());
    }
    let identity = pencil_progress_sink::Identity {
        session: context.identity.session.clone(),
        execution: context.identity.execution,
        workload_sha256: context.identity.workload_sha256.clone(),
        binary_sha256: context.identity.binary_sha256.clone(),
        protocol_sha256: context.identity.protocol_sha256.clone(),
        seed: context.identity.seed,
        attempt: context.identity.attempt,
        source_sha256: context.identity.source_sha256.clone(),
        method_config_sha256: context.identity.method_config_sha256.clone(),
    };
    let mut bytes = 0u64;
    for (sequence, pin) in attempt.progress_files.iter().enumerate() {
        let (raw, value) = checked_json(pin, pencil_progress_sink::MAX_BYTES as u64)?;
        let snapshot: pencil_progress_sink::Snapshot =
            serde_json::from_value(value).map_err(|error| error.to_string())?;
        pencil_progress_sink::validate(&snapshot, &identity, sequence as u64)
            .map_err(|error| error.to_string())?;
        bytes = bytes.checked_add(raw.len() as u64).ok_or("progress byte overflow")?;
    }
    match &attempt.progress_totals {
        None if attempt.progress_files.is_empty() => Ok(true),
        None => Err("progress snapshots lack totals bytes".into()),
        Some(pin) => {
            let (_, value) = checked_json(pin, pencil_progress_sink::MAX_BYTES as u64)?;
            let totals: pencil_progress_sink::IoTotals =
                serde_json::from_value(value).map_err(|error| error.to_string())?;
            if totals.committed != attempt.progress_files.len() as u64
                || totals.bytes != bytes
                || totals.published < totals.committed
                || totals.attempted < totals.committed
            {
                return Err("progress totals differ from checked snapshot files".into());
            }
            Ok(totals.failure.is_none())
        }
    }
}

fn validate_diagnostics_attachment(
    attempt: &AttemptEvidence,
    expected_child: &DiagnosticChild,
    context: &diagnostics::Context,
    slot: &Slot,
) -> Result<bool, String> {
    let Some(attachment_pin) = &attempt.diagnostics_attachment else {
        return Ok(false);
    };
    let (attachment_bytes, attachment) = checked_json(attachment_pin, MAX_JSON_BYTES)?;
    let mut expected_attachment_bytes =
        serde_json::to_vec_pretty(&attachment).map_err(|error| error.to_string())?;
    expected_attachment_bytes.push(b'\n');
    if attachment_bytes != expected_attachment_bytes {
        return Err("diagnostic attachment is not the native pretty-JSON bytes".into());
    }
    let Some(record_pin) = &attempt.ordinary_record else {
        return Err("diagnostic attachment binds a missing ordinary record".into());
    };
    if text(&attachment, "/schema")? != "ecbench.diagnostic_attachment/v2"
        || text(&attachment, "/record_sha256")? != record_pin.sha256
        || attachment["accepted_result"] != Value::Null
        || attachment["accepted_cost"] != Value::Null

    {
        return Err("diagnostic attachment schema/record/lifecycle mismatch".into());
    }
    let child: DiagnosticChild = serde_json::from_value(attachment["context"].clone())
        .map_err(|error| error.to_string())?;
    let mut child_bytes = serde_json::to_vec_pretty(&child).map_err(|error| error.to_string())?;
    child_bytes.push(b'\n');
    let launch = child.launch.document()?;
    child.envelope.validate(&child.launch)?;
    if text(&attachment, "/context_sha256")? != sha256_hex(&child_bytes)
        || text(&attachment, "/context_sha256")? != attempt.context.sha256
        || &child != expected_child
        || child.envelope.prefix != *context
        || child.envelope.scientific_run_id != launch.scientific_run_id
        || child.envelope.source_receipt != launch.source_receipt
        || child.envelope.arm.name != slot.arm
        || serde_json::to_value(&child.envelope.arm.role).map_err(|error| error.to_string())?
            != Value::String(slot.role.clone())
        || child.envelope.arm.method != text(&slot.method, "/id")?
        || child.envelope.arm.method_config_sha256 != context.identity.method_config_sha256
    {
        return Err("diagnostic attachment child envelope differs from exact slot/context".into());
    }
    let lifecycle_complete = attachment.pointer("/lifecycle/error") == Some(&Value::Null)
        && !attachment.pointer("/lifecycle/observation").is_none_or(Value::is_null);
    let prefix_applicable = slot.arm == "pencil";
    if attachment.pointer("/prefix_applicable").and_then(Value::as_bool)
        != Some(prefix_applicable)
    {
        return Err("diagnostic attachment prefix applicability differs from arm".into());
    }
    if !prefix_applicable {
        if !attachment["prefix"].is_null() {
            return Err("rho diagnostic attachment carries a pencil prefix".into());
        }
        return Ok(lifecycle_complete);
    }
    if attachment.pointer("/prefix/error") != Some(&Value::Null)
        || attachment
            .pointer("/prefix/observation")
            .is_none_or(Value::is_null)
    {
        return Ok(false);
    }
    let manifest: diagnostics::Manifest = serde_json::from_value(
        attachment
            .pointer("/prefix/observation")
            .cloned()
            .ok_or("pencil diagnostic attachment prefix missing")?,
    )
    .map_err(|error| error.to_string())?;
    if manifest.schema != "ecbench.diagnostic_manifest/v1"
        || manifest.identity != context.identity
        || manifest.context_sha256
            != sha256_hex(&serde_json::to_vec(context).map_err(|error| error.to_string())?)
        || manifest.record_sha256 != record_pin.sha256
        || !manifest.diagnostic_only
        || manifest.counter_semantics_verified
        || manifest.durability_confirmed.is_some()
    {
        return Err("embedded diagnostic manifest identity/bindings/status mismatch".into());
    }
    let expected: BTreeMap<String, (&str, usize)> = attempt
        .progress_files
        .iter()
        .map(|pin| {
            let name = Path::new(&pin.path)
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or("non-UTF8 progress filename")?
                .to_owned();
            Ok((name, (pin.sha256.as_str(), pin.bytes as usize)))
        })
        .collect::<Result<_, String>>()?;
    if manifest.entries.len() != expected.len()
        || manifest.entries.iter().any(|entry| {
            expected.get(&entry.name).is_none_or(|(sha256, bytes)| {
                entry.sha256 != *sha256
                    || entry.bytes != *bytes
                    || entry.classification != "structurally_valid_prefix"
                    || entry.sequence.is_none()
            })
        })
    {
        return Err("embedded diagnostic manifest entries differ from checked progress bytes".into());
    }
    Ok(lifecycle_complete)
}

fn validate_relation(
    evidence: &RelationEvidence,
    context: &diagnostics::Context,
    slot: &Slot,
    source_revision: Option<&str>,
) -> Result<(bool, bool), String> {
    let stream = checked_file(&evidence.stream, MAX_RELATION_BYTES)?;
    let text_stream = std::str::from_utf8(&stream).map_err(|error| error.to_string())?;
    let mut records = Vec::new();
    for (line_number, line) in text_stream.lines().enumerate() {
        if line.is_empty() {
            return Err("empty relation-stream line".into());
        }
        reject_duplicate_keys(line)?;
        let record: relation::Record =
            serde_json::from_str(line).map_err(|error| format!("relation line {}: {error}", line_number + 1))?;
        if record.sequence != line_number as u64 {
            return Err("relation stream sequence is not contiguous".into());
        }
        records.push(record);
    }
    if records.is_empty() {
        let (_, value) = checked_json(&evidence.totals, MAX_JSON_BYTES)?;
        let totals: RelationTotals = serde_json::from_value(value).map_err(|e| e.to_string())?;
        if totals.schema != "ecbench.pencil-relation-file/v1" || totals.path != evidence.stream.path
            || totals.sha256 != evidence.stream.sha256 || totals.bytes != 0
            || totals.synced_records != 0 || totals.complete || totals.failure.is_none() {
            return Err("empty relation prefix lacks matching failure totals".into());
        }
        return Ok((false,false));
    }
    let first = records.first().expect("nonempty checked");
    let relation::Event::Header(header) = &first.event else {
        return Err("relation stream does not start with Header".into());
    };
    let expected_run_id = format!(
        "{}:execution-{}:attempt-{}",
        context.identity.session, context.identity.execution, context.identity.attempt
    );
    if header.schema != "pencil-relation-stream/v1"
        || Some(header.identity.source_revision.as_str()) != source_revision
        || header.identity.source_sha256 != context.identity.source_sha256
        || header.identity.workload_id != context.identity.workload_sha256
        || header.identity.run_id != expected_run_id
        || header.seed != slot.algorithm_seed
        || header.p != number(&slot.workload, "/curve_spec/p")?
        || header.r != number(&slot.workload, "/curve_spec/r")?
    {
        return Err("relation Header differs from exact attempt/plan identity".into());
    }
    let mut terminal = None;
    for (index, record) in records.iter().enumerate().skip(1) {
        if matches!(record.event, relation::Event::Header(_)) {
            return Err("relation stream contains a second Header".into());
        }
        if let relation::Event::Terminal(value) = &record.event {
            if index + 1 != records.len() || terminal.replace(value).is_some() {
                return Err("relation Terminal must be unique and final".into());
            }
        }
    }
    let (_, totals_value) = checked_json(&evidence.totals, MAX_JSON_BYTES)?;
    let totals: RelationTotals =
        serde_json::from_value(totals_value).map_err(|error| error.to_string())?;
    let _relation_io_ns = totals
        .write_ns
        .checked_add(totals.sync_ns)
        .ok_or("relation IO timer overflow")?;
    let complete = terminal.is_some();
    if totals.schema != "ecbench.pencil-relation-file/v1"
        || totals.path != evidence.stream.path
        || totals.sha256 != evidence.stream.sha256
        || totals.bytes != evidence.stream.bytes
        || totals.synced_records != records.len() as u64
        || totals.complete != complete
        || (!complete && totals.failure.is_none())
    {
        return Err("relation totals differ from checked stream bytes/state".into());
    }
    let source_verified = terminal.is_some_and(|terminal| {
        terminal.source_verified && terminal.failure.is_none()
    });
    Ok((complete, source_verified))
}

fn validate_resources(
    evidence: &ResourceEvidence,
    attempt: &AttemptEvidence,
    context: &diagnostics::Context,
    scientific_run_id: &str,
    record: Option<(&FilePin, &Record)>,
    cohort: &CohortPins,
    slot: &Slot,
) -> Result<bool, String> {
    let prefix_bytes = checked_file(&evidence.prefix, super::diagnostic_resources::MAX_PREFIX_BYTES as u64)?;
    let prefix_text = std::str::from_utf8(&prefix_bytes).map_err(|error| error.to_string())?;
    reject_duplicate_keys(prefix_text)?;
    let prefix: ResourcePrefix =
        serde_json::from_str(prefix_text).map_err(|error| error.to_string())?;
    let (_, tail_value) = checked_json(&evidence.tail, MAX_JSON_BYTES)?;
    let tail: PublicationTail =
        serde_json::from_value(tail_value).map_err(|error| error.to_string())?;
    let reconstructed = PublicationTail::observed(
        &prefix,
        &prefix_bytes,
        tail.receipt_write_started_ns,
        tail.receipt_write_durable_ns,
    )?;
    if reconstructed != tail
        || prefix.identity.session_id != attempt.session_id
        || prefix.identity.seq != attempt.sequence
        || prefix.identity.attempt != attempt.attempt
        || prefix.identity.scientific_run_id != scientific_run_id
        || prefix.identity.protocol_sha256 != cohort.protocol.sha256
        || prefix.identity.source_manifest_sha256 != cohort.source_manifest.sha256
        || prefix.identity.binary_sha256 != cohort.binary.sha256
        || prefix.bindings.context_sha256.as_deref() != Some(attempt.context.sha256.as_str())
    {
        return Err("resource prefix/tail differs from checked attempt identity".into());
    }
    if let Some((pin, _record)) = record {
        if prefix.bindings.record_sha256.as_deref() != Some(pin.sha256.as_str())
        {
            return Err("resource record binding differs from ordinary record".into());
        }
    } else if prefix.bindings.record_sha256.is_some() {
        return Err("resource prefix binds an unretained ordinary record".into());
    }
    if prefix.bindings.diagnostics_sha256.as_deref()
        != attempt
            .diagnostics_attachment
            .as_ref()
            .map(|pin| pin.sha256.as_str())
    {
        return Err("resource diagnostics binding differs from retained manifest".into());
    }
    let snapshot_pin = attempt
        .session_snapshot
        .as_ref()
        .ok_or("resource attempt lacks immutable session snapshot")?;
    let (_, snapshot) = checked_json(snapshot_pin, MAX_JSON_BYTES)?;
    if prefix.bindings.session_sha256.as_deref() != Some(snapshot_pin.sha256.as_str()) {
        return Err("resource snapshot bytes differ from bound pin".into());
    }
    validate_session_snapshot(&snapshot, attempt, slot, &cohort.binary.sha256)?;
    Ok(matches!(prefix.status, Status::Returned))
}

fn validate_session_snapshot(snapshot: &Value, attempt: &AttemptEvidence, slot: &Slot, binary: &str) -> Result<(),String> {
    if text(snapshot,"/schema")? != "ecbench.session/v1"
        || text(snapshot,"/session_id")? != attempt.session_id
        || text(snapshot,"/plan_sha256")? != slot.plan_sha256
        || text(snapshot,"/binary_sha256")? != binary
        || text(snapshot,"/status")? != "running"
        || number(snapshot,"/executions_planned")? != 3_240
        || number(snapshot,"/records_written")? != attempt.sequence.checked_add(1).ok_or("sequence overflow")?
        || number(snapshot,"/last_seq")? != attempt.sequence {
        return Err("resource session binding differs from immutable execution snapshot".into());
    }
    Ok(())
}

fn validate_retry_links(
    expected: &BTreeMap<SlotKey, Slot>,
    attempts: &[AttemptEvidence],
) -> Result<(), String> {
    let mut identities = BTreeSet::new();
    let mut primaries = BTreeSet::new();
    for attempt in attempts {
        let key = AttemptRef {
            session_id: attempt.session_id.clone(),
            sequence: attempt.sequence,
            attempt: attempt.attempt,
        };
        if !identities.insert(key.clone()) {
            return Err("duplicate retained attempt identity".into());
        }
        if attempt.attempt == 0 {
            let slot_key = SlotKey {
                session_id: attempt.session_id.clone(),
                sequence: attempt.sequence,
            };
            if !expected.contains_key(&slot_key) {
                return Err("primary retained attempt is absent from frozen plans".into());
            }
            if attempt.retry_of.is_some() || !primaries.insert(slot_key) {
                return Err("primary attempt retry link or uniqueness mismatch".into());
            }
        } else {
            let parent = attempt.retry_of.as_ref().ok_or("retry lacks explicit primary parent")?;
            if parent.sequence != attempt.sequence
                || parent.attempt != 0
            {
                return Err("retry parent crosses slot or is not attempt zero".into());
            }
        }
    }
    let expected_keys: BTreeSet<SlotKey> = expected.keys().cloned().collect();
    if !primaries.is_subset(&expected_keys) {
        return Err("observed primary is outside the frozen enrollment".into());
    }
    for attempt in attempts.iter().filter(|attempt| attempt.attempt > 0) {
        let parent = attempt.retry_of.as_ref().expect("validated retry parent");
        if !expected.contains_key(&SlotKey { session_id: parent.session_id.clone(), sequence: parent.sequence }) {
            return Err("retry parent is outside frozen primary enrollment".into());
        }
    }
    Ok(())
}

fn validate_session_attempt_coverage(
    slots: &BTreeMap<SlotKey, Slot>,
    attempts: &[AttemptEvidence],
    custody: &BTreeMap<String, (bool, String)>,
) -> Result<(), String> {
    let primary_sessions: BTreeSet<&str> = slots
        .keys()
        .map(|key| key.session_id.as_str())
        .collect();
    for (session, (_, plan_sha256)) in custody {
        let expected: BTreeSet<u64> = slots
            .values()
            .filter(|slot| &slot.plan_sha256 == plan_sha256)
            .map(|slot| slot.sequence)
            .collect();
        let rows: Vec<&AttemptEvidence> = attempts
            .iter()
            .filter(|attempt| &attempt.session_id == session)
            .collect();
        let observed: BTreeSet<u64> = rows.iter().map(|attempt| attempt.sequence).collect();
        let attempt_numbers: BTreeSet<u64> = rows.iter().map(|attempt| attempt.attempt).collect();
        if expected.len() != 3_240
            || !observed.is_subset(&expected)
            || rows.len() != observed.len()
            || attempt_numbers.len() > 1
            || (primary_sessions.contains(session.as_str())
                && !rows.is_empty() && attempt_numbers.first() != Some(&0))
            || (!primary_sessions.contains(session.as_str())
                && !rows.is_empty() && attempt_numbers.first().is_none_or(|attempt| *attempt == 0))
        {
            return Err("native session rows are not a unique subset of its exact plan slots".into());
        }
    }
    Ok(())
}

fn checked_attempts(
    request: &ExportRequest,
    slots: &BTreeMap<SlotKey, Slot>,
    custody: &BTreeMap<String, (bool, String)>,
) -> Result<Vec<CheckedAttempt>, String> {
    validate_retry_links(slots, &request.attempts)?;
    validate_session_attempt_coverage(slots, &request.attempts, custody)?;
    let mut checked = Vec::with_capacity(request.attempts.len());
    for input in &request.attempts {
        let route = input.retry_of.as_ref().map_or(
            SlotKey {
                session_id: input.session_id.clone(),
                sequence: input.sequence,
            },
            |parent| SlotKey {
                session_id: parent.session_id.clone(),
                sequence: parent.sequence,
            },
        );
        let slot = slots
            .get(&route)
            .ok_or("attempt slot disappeared")?
            .clone();
        let (custody_closed, custody_plan_sha256) = custody
            .get(&input.session_id)
            .ok_or("attempt lacks session custody")?;
        if custody_plan_sha256 != &slot.plan_sha256 {
            return Err("retry session plan differs from its primary parent plan".into());
        }
        let diagnostic_child = validate_context(input, &slot, &request.cohort)?;
        let context = diagnostic_child.envelope.prefix.clone();
        let progress_ok = validate_progress(input, &context)?;
        let mut reasons = Vec::new();
        if input.attempt > 0 {
            reasons.push("retry_retained_not_substituted_for_primary".into());
        }
        if slot.warmup {
            reasons.push("warmup_not_measured".into());
        }
        let (record_value, record_typed) = match &input.ordinary_record {
            None => {
                reasons.push("ordinary_record_missing".into());
                (None, None)
            }
            Some(pin) => {
                let (value, record) = validate_record(pin, &context, &slot, &request.cohort)?;
                (Some(value), Some(record))
            }
        };
        let diagnostics_ok =
            validate_diagnostics_attachment(input, &diagnostic_child, &context, &slot)?;
        let (stream_complete, stream_verified) = match (&slot.arm[..], &input.relation) {
            ("pencil", None) => {
                reasons.push("pencil_relation_stream_missing".into());
                (None, false)
            }
            ("pencil", Some(evidence)) => {
                let (complete, verified) = validate_relation(
                    evidence,
                    &context,
                    &slot,
                    diagnostic_child
                        .envelope
                        .source_receipt
                        .source_revision
                        .as_deref(),
                )?;
                if !complete {
                    reasons.push("pencil_relation_prefix_incomplete".into());
                } else if !verified {
                    reasons.push("pencil_relation_terminal_not_source_verified".into());
                }
                (Some(complete), verified)
            }
            (_, Some(_)) => return Err("rho arm carries pencil relation stream".into()),
            (_, None) => (None, true),
        };
        let resource_returned = match &input.resources {
            None => {
                if input.session_snapshot.is_some() {
                    return Err("session snapshot supplied without resource prefix/tail".into());
                }
                reasons.push("resource_prefix_or_tail_missing".into());
                false
            }
            Some(evidence) => {
                let record = input
                    .ordinary_record
                    .as_ref()
                    .zip(record_typed.as_ref());
                let returned = validate_resources(
                    evidence,
                    input,
                    &context,
                    &diagnostic_child.envelope.scientific_run_id,
                    record,
                    &request.cohort,
                    &slot,
                )?;
                if !returned {
                    reasons.push("resource_status_not_returned".into());
                }
                returned
            }
        };
        let record_verified = record_value
            .as_ref()
            .is_some_and(record_is_verified_measured);
        if record_typed.is_some() && !record_verified {
            reasons.push("ordinary_record_not_verified_measured".into());
        }
        let evidence_complete = record_typed.is_some()
            && resource_returned
            && stream_verified;
        checked.push(CheckedAttempt {
            input: input.clone(),
            slot,
            context,
            record: record_value,
            record_id: record_typed.as_ref().map(|record| record.record_id.clone()),
            evidence_complete,
            custody_closed: *custody_closed,
            stream_complete,
            progress_diagnostic_complete: progress_ok,
            diagnostic_prefix_complete: diagnostics_ok,
            reasons,
        });
    }
    Ok(checked)
}

fn triplet_key(slot: &Slot) -> Result<TripletKey, String> {
    Ok(TripletKey {
        plan_name: slot.plan_name.clone(),
        session_id: slot.session_id.clone(),
        workload_id: workload_id(&slot.workload)?.to_owned(),
        round: slot.round,
        seed: slot.algorithm_seed,
    })
}

fn pair_id(key: &TripletKey) -> Result<String, String> {
    let bytes = serde_json::to_vec(&json!({
        "plan": key.plan_name,
        "session": key.session_id,
        "workload_id": key.workload_id,
        "round": key.round,
        "seed": key.seed,
    }))
    .map_err(|error| error.to_string())?;
    Ok(format!("PPAIR2h{}", &sha256_hex(&bytes)[..16]))
}

fn mapped_price_is_complete(value: &Value) -> Result<bool, String> {
    let blockers = value
        .pointer("/coverage_blockers")
        .and_then(Value::as_array)
        .ok_or("nine-category mapper omitted coverage_blockers")?;
    let cells = value
        .pointer("/cells")
        .and_then(Value::as_array)
        .ok_or("nine-category mapper omitted cells")?;
    Ok(blockers.is_empty()
        && cells.len() == 36
        && cells.iter().all(|cell| {
            !cell["candidate"].is_null()
                && !cell["reference"].is_null()
                && !cell["ratio"].is_null()
        }))
}

fn price_triplets(
    checked: &mut [CheckedAttempt],
    coverage_review: &Value,
    cohort: &CohortPins,
    slots: &BTreeMap<SlotKey, Slot>,
) -> Result<Vec<Value>, String> {
    let mut primary_index = BTreeMap::<TripletKey, BTreeMap<String, usize>>::new();
    for slot in slots.values().filter(|slot| !slot.warmup) {
        primary_index.entry(triplet_key(slot)?).or_default();
    }
    for (index, attempt) in checked.iter().enumerate() {
        if attempt.input.attempt != 0 || attempt.slot.warmup {
            continue;
        }
        let key = triplet_key(&attempt.slot)?;
        if primary_index
            .entry(key)
            .or_default()
            .insert(attempt.slot.arm.clone(), index)
            .is_some()
        {
            return Err("duplicate primary measured triplet arm".into());
        }
    }
    if primary_index.len() != 6_300 {
        return Err("measured triplet count differs from frozen 6,300".into());
    }
    let mut output = Vec::with_capacity(primary_index.len());
    for (key, arms) in primary_index {
        if arms.len() != 3 {
            for index in arms.values() {
                checked[*index].reasons.push("primary_triplet_incomplete".into());
            }
            output.push(json!({"pair_id":pair_id(&key)?,"plan":key.plan_name,
                "session_id":key.session_id,"workload_id":key.workload_id,"round":key.round,
                "algorithm_seed":key.seed,"observed_arms":arms.keys().collect::<Vec<_>>(),
                "candidate_reference_eligible":false,
                "candidate_reference_ineligibility_reasons":["primary_slot_evidence_missing"],
                "finite_algorithm_cost":Value::Null,"rho_AA_evidence_eligible":false}));
            continue;
        }
        if !["pencil","rho","rho_AA"].into_iter().all(|arm| arms.contains_key(arm)) {
            return Err("unexpected measured arm".into());
        }
        let candidate_index = arms["pencil"];
        let reference_index = arms["rho"];
        let aa_index = arms["rho_AA"];
        let candidate = &checked[candidate_index];
        let reference = &checked[reference_index];
        let aa = &checked[aa_index];
        if candidate.slot.plan_sha256 != reference.slot.plan_sha256
            || candidate.slot.plan_sha256 != aa.slot.plan_sha256
            || candidate.slot.workload != reference.slot.workload
            || candidate.slot.workload != aa.slot.workload
            || reference.slot.method != aa.slot.method
            || candidate.context.identity.source_sha256 != reference.context.identity.source_sha256
            || candidate.context.identity.source_sha256 != aa.context.identity.source_sha256
            || candidate.context.identity.protocol_sha256 != reference.context.identity.protocol_sha256
            || candidate.context.identity.protocol_sha256 != aa.context.identity.protocol_sha256
            || candidate.context.identity.binary_sha256 != reference.context.identity.binary_sha256
            || candidate.context.identity.binary_sha256 != aa.context.identity.binary_sha256
        {
            return Err("measured triplet full identity mismatch".into());
        }
        let id = pair_id(&key)?;
        let base_eligible = candidate.evidence_complete
            && reference.evidence_complete
            && candidate.reasons.is_empty()
            && reference.reasons.is_empty();
        let bundle = json!({
            "schema": "pencil-price-pair-input/v1",
            "p": number(&candidate.slot.workload, "/curve_spec/p")?,
            "r": number(&candidate.slot.workload, "/curve_spec/r")?,
            "protocol_sha256": cohort.protocol.sha256,
            "coverage_review": coverage_review,
            "candidate": {
                "expected": expected_value(&candidate.slot, &cohort.binary.sha256),
                "record": candidate.record,
            },
            "reference": {
                "expected": expected_value(&reference.slot, &cohort.binary.sha256),
                "record": reference.record,
            },
        });
        let (price, price_blockers) = if base_eligible {
            match analyzer::analyze(&bundle) {
                Ok(value) => {
                    let blockers = value
                        .pointer("/coverage_blockers")
                        .and_then(Value::as_array)
                        .cloned()
                        .ok_or("nine-category mapper omitted coverage_blockers")?;
                    if mapped_price_is_complete(&value)? {
                        (
                            Some(json!({
                                "schema": value["schema"],
                                "policy": value["policy"],
                                "mapping_sha256": value["mapping_sha256"],
                                "protocol_sha256": value["protocol_sha256"],
                                "candidate_record_id": value["candidate_record_id"],
                                "reference_record_id": value["reference_record_id"],
                                "coverage_blockers": value["coverage_blockers"],
                                "cells": value["cells"],
                                "resource_scope": value["resource_scope"],
                                "raw_input": Value::Null,
                                "raw_input_status": "retained_by_exact_record_and_evidence_pins_not_duplicated",
                            })),
                            Vec::new(),
                        )
                    } else {
                        (None, blockers.into_iter().map(|value| value.to_string()).collect())
                    }
                }
                Err(error) => (None, vec![format!("nine_category_mapper:{error}")]),
            }
        } else {
            (None, vec!["attempt_evidence_or_custody_ineligible".into()])
        };
        let aa_eligible = aa.evidence_complete
            && reference.evidence_complete
            && aa.reasons.is_empty()
            && reference.reasons.is_empty();
        let price_missing = price.is_none();
        let evidence_blockers = comparison_blockers(candidate, reference);
        let comparison_eligible = price.is_some() && evidence_blockers.is_empty();
        let mut comparison_reasons = price_blockers;
        comparison_reasons.extend(evidence_blockers);
        output.push(json!({
            "pair_id": id,
            "plan": key.plan_name,
            "plan_sha256": candidate.slot.plan_sha256,
            "session_id": key.session_id,
            "workload_id": key.workload_id,
            "round": key.round,
            "algorithm_seed": key.seed,
            "sequence": {
                "pencil": candidate.slot.sequence,
                "rho": reference.slot.sequence,
                "rho_AA": aa.slot.sequence,
            },
            "roles": {
                "pencil": candidate.slot.role,
                "rho": reference.slot.role,
                "rho_AA": aa.slot.role,
            },
            "record_ids": {
                "pencil": candidate.record_id,
                "rho": reference.record_id,
                "rho_AA": aa.record_id,
            },
            "candidate_reference_eligible": comparison_eligible,
            "candidate_reference_ineligibility_reasons": comparison_reasons,
            "finite_algorithm_cost": price,
            "rho_AA_evidence_eligible": aa_eligible && comparison_blockers(aa, reference).is_empty(),
            "owner_closure_structurally_complete": {
                "pencil": candidate.custody_closed,
                "rho": reference.custody_closed,
                "rho_AA": aa.custody_closed,
            },
            "rho_AA_cost": Value::Null,
            "rho_AA_cost_status": "not_a_candidate_reference_nine_category_price",
        }));
        if price_missing {
            checked[candidate_index]
                .reasons
                .push("finite_cost_unknown_pair_ineligible".into());
            checked[reference_index]
                .reasons
                .push("finite_cost_unknown_pair_ineligible".into());
        }
    }
    output.sort_by(|a, b| {
        (a["plan"].as_str(), a["workload_id"].as_str(), a["round"].as_u64())
            .cmp(&(b["plan"].as_str(), b["workload_id"].as_str(), b["round"].as_u64()))
    });
    Ok(output)
}

fn required_evidence_blockers(custody_closed: bool, diagnostic_complete: bool) -> Vec<&'static str> {
    let mut reasons=Vec::new();
    if !custody_closed { reasons.push("owner_custody_not_closed"); }
    if !diagnostic_complete { reasons.push("diagnostic_attachment_incomplete"); }
    reasons
}

fn comparison_blockers(candidate: &CheckedAttempt, reference: &CheckedAttempt) -> Vec<String> {
    let mut blockers = Vec::new();
    for (role, row) in [("candidate",candidate),("reference",reference)] {
        for reason in required_evidence_blockers(row.custody_closed,row.diagnostic_prefix_complete) {
            blockers.push(format!("{role}_{reason}"));
        }
    }
    blockers
}

fn attempt_output(attempt: &CheckedAttempt) -> Result<Value, String> {
    let measured = !attempt.slot.warmup;
    let raw_costs = attempt.record.as_ref().map_or(Value::Null, |record| {
        json!({
            "record_cost_observation": record["cost"],
            "field_ops": record["field_ops"],
            "time": record["time"],
            "complete_native_ledger": attempt.input.ordinary_record,
        })
    });
    let mut eligibility_reasons = attempt.reasons.clone();
    eligibility_reasons.extend(comparison_blockers(attempt, attempt));
    eligibility_reasons.sort(); eligibility_reasons.dedup();
    Ok(json!({
        "session_id": attempt.input.session_id,
        "sequence": attempt.input.sequence,
        "round": attempt.slot.round,
        "warmup": attempt.slot.warmup,
        "role": attempt.slot.role,
        "arm": attempt.slot.arm,
        "attempt": attempt.input.attempt,
        "retry_of": attempt.input.retry_of,
        "retry_substitutes_for_primary": false,
        "plan": attempt.slot.plan_name,
        "plan_sha256": attempt.slot.plan_sha256,
        "workload_id": workload_id(&attempt.slot.workload)?,
        "workload_sha256": workload_sha(&attempt.slot.workload)?,
        "method": attempt.slot.method,
        "algorithm_seed": attempt.slot.algorithm_seed,
        "source_manifest_sha256": attempt.context.identity.source_sha256,
        "binary_sha256": attempt.context.identity.binary_sha256,
        "protocol_sha256": attempt.context.identity.protocol_sha256,
        "context": attempt.input.context,
        "ordinary_record": attempt.input.ordinary_record,
        "record_id": attempt.record_id,
        "progress_files": attempt.input.progress_files,
        "progress_totals": attempt.input.progress_totals,
        "diagnostics_attachment": attempt.input.diagnostics_attachment,
        "relation": attempt.input.relation,
        "relation_stream_complete": attempt.stream_complete,
        "resources": attempt.input.resources,
        "session_snapshot": attempt.input.session_snapshot,
        "progress_diagnostic_complete": attempt.progress_diagnostic_complete,
        "diagnostic_prefix_complete": attempt.diagnostic_prefix_complete,
        "session_custody_closed": attempt.custody_closed,
        "evidence_complete": attempt.evidence_complete,
        "eligible_for_measured_pair": measured
            && attempt.input.attempt == 0
            && attempt.evidence_complete
            && eligibility_reasons.is_empty(),
        "ineligibility_reasons": eligibility_reasons,
        "raw_costs": raw_costs,
        "raw_costs_status": if attempt.record.is_some() {
            "derived_from_checked_ordinary_record_bytes"
        } else {
            "unknown_no_ordinary_record"
        },
    }))
}

pub fn validate_request(request: &ExportRequest) -> Result<Value, String> {
    if request.schema != REQUEST_SCHEMA {
        return Err("unsupported retained-attempt request schema".into());
    }
    let (slots, prepared_receipt) = build_slots(&request.cohort)?;
    let coverage_review = validate_protocol_and_sources(&request.cohort)?;
    let custody = session_custody(&request.cohort, &request.attempts)?;
    let attempt_sessions: BTreeSet<&str> = request
        .attempts
        .iter()
        .map(|attempt| attempt.session_id.as_str())
        .collect();
    let custody_sessions: BTreeSet<&str> = custody.keys().map(String::as_str).collect();
    if !attempt_sessions.is_subset(&custody_sessions) {
        return Err("session custody set differs from retained attempt sessions".into());
    }
    let mut attempts = checked_attempts(request, &slots, &custody)?;
    let price_pairs = price_triplets(&mut attempts, &coverage_review, &request.cohort, &slots)?;
    let mut attempt_rows = attempts
        .iter()
        .map(attempt_output)
        .collect::<Result<Vec<_>, _>>()?;
    let observed_primaries: BTreeSet<SlotKey> = attempts.iter().filter(|a| a.input.attempt==0)
        .map(|a| SlotKey {session_id:a.input.session_id.clone(), sequence:a.input.sequence}).collect();
    let mut missing_primary_slots = 0usize;
    for (key, slot) in &slots {
        if !observed_primaries.contains(key) {
            missing_primary_slots += 1;
            attempt_rows.push(json!({"session_id":slot.session_id,"sequence":slot.sequence,
                "attempt":0,"plan":slot.plan_name,"plan_sha256":slot.plan_sha256,
                "workload_id":workload_id(&slot.workload)?,"method":slot.method,
                "round":slot.round,"warmup":slot.warmup,"arm":slot.arm,"role":slot.role,
                "algorithm_seed":slot.algorithm_seed,"observation_status":"not_observed",
                "eligible_for_measured_pair":false,"ineligibility_reasons":["primary_slot_evidence_missing"],
                "context":Value::Null,"ordinary_record":Value::Null,"raw_costs":Value::Null,
                "retry_substitutes_for_primary":false}));
        }
    }
    let retries = attempts.iter().filter(|attempt| attempt.input.attempt > 0).count();
    let admitted_prices = price_pairs
        .iter()
        .filter(|pair| pair["candidate_reference_eligible"] == Value::Bool(true))
        .count();
    Ok(json!({
        "schema": OUTPUT_SCHEMA,
        "status": "postrun_bytes_and_frozen_enrollment_checked",
        "prepared_plan_validation": request.cohort.prepared_plan_validation,
        "prepared_plan_receipt": prepared_receipt,
        "source_manifest": request.cohort.source_manifest,
        "binary": request.cohort.binary,
        "protocol": request.cohort.protocol,
        "coverage_review": request.cohort.coverage_review,
        "plans": request.cohort.plans,
        "sessions": request.cohort.sessions,
        "primary_execution_slots": 22_680,
        "measured_execution_slots": 18_900,
        "warmup_execution_slots": 3_780,
        "measured_triplets": 6_300,
        "retained_retry_attempts": retries,
        "missing_primary_slots": missing_primary_slots,
        "admitted_finite_price_pairs": admitted_prices,
        "raw_finite_price_pairs": price_pairs.iter().filter(|p| !p["finite_algorithm_cost"].is_null()).count(),
        "missing_or_ineligible_finite_price_pairs": 6_300usize - admitted_prices,
        "nine_category_mapping": analyzer::mapping_contract(),
        "attempts": attempt_rows,
        "measured_pair_rows": price_pairs,
        "owner_closure_structurally_complete": custody.values().all(|(closed, _)| *closed),
        "owner_authentication_verified": false,
        "scientific_execution_admitted": false,
        "cost_rule": "only existing nine-category mapper output with zero blockers is non-null; unknown costs remain null and ineligible",
        "retry_rule": "attempt zero is the sole frozen slot observation; retries are retained and never substitute",
    }))
}

fn checked_output_path(path: &Path) -> Result<(), String> {
    if !path.is_absolute()
        || path.parent().is_none()
        || path.components().any(|component| {
            !matches!(component, Component::RootDir | Component::Normal(_))
        })
    {
        return Err("attempt export requires absolute traversal-free path".into());
    }
    let parent = path.parent().expect("checked parent");
    if fs::canonicalize(parent).map_err(|error| error.to_string())? != parent {
        return Err("attempt export parent must be canonical".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let metadata = fs::symlink_metadata(parent).map_err(|error| error.to_string())?;
        if !metadata.is_dir()
            || metadata.file_type().is_symlink()
            || metadata.uid() != unsafe { libc::geteuid() }
            || metadata.mode() & 0o077 != 0
        {
            return Err("attempt export parent must be caller-owned and owner-only".into());
        }
    }
    Ok(())
}

pub fn validate_and_export(request: &ExportRequest, path: &Path) -> Result<FilePin, String> {
    checked_output_path(path)?;
    let output = validate_request(request)?;
    let bytes = serde_json::to_vec(&output).map_err(|error| error.to_string())?;
    if bytes.len() > MAX_REQUEST_BYTES {
        return Err("attempt export output byte bound".into());
    }
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o400).custom_flags(libc::O_NOFOLLOW);
    }
    let mut file = options.open(path).map_err(|error| error.to_string())?;
    file.write_all(&bytes).map_err(|error| error.to_string())?;
    file.sync_all().map_err(|error| error.to_string())?;
    fs::File::open(path.parent().expect("checked parent"))
        .and_then(|directory| directory.sync_all())
        .map_err(|error| error.to_string())?;
    Ok(FilePin {
        path: path.to_str().ok_or("non-UTF8 output path")?.to_owned(),
        sha256: sha256_hex(&bytes),
        bytes: bytes.len() as u64,
    })
}

#[cfg(test)]
#[path = "retained_attempt_join_v4_controls.rs"]
mod adversarial_controls;
