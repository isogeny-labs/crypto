use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

pub const SCHEMA: &str = "p739-cross-session-audit/v2";
pub const REQUEST_SCHEMA: &str = "p739-cross-session-audit-request/v2";

// This is deliberately a leaf-level, fail-closed allowlist derived from the two
// hash-bound record streams. A new timing or environment field that differs is
// rejected until a reviewer explicitly changes this source and its tests.
pub const ALLOWED_DIFFERENCE_PATHS: &[&str] = &[
    "/detail/adapter_accounting/adapter_elapsed_ns_before_accounting_attachment",
    "/detail/candidate/candidate_elapsed_ns",
    "/detail/candidate/candidate_phase_overhead_ns",
    "/detail/candidate/cold_setup_elapsed_ns",
    "/detail/candidate/phases/candidate_verification/elapsed_ns",
    "/detail/candidate/phases/elimination/elapsed_ns",
    "/detail/candidate/phases/recovery_check/elapsed_ns",
    "/detail/candidate/phases/sieve/elapsed_ns",
    "/detail/candidate/phases/solver_setup/elapsed_ns",
    "/detail/candidate/phases/target_validation/elapsed_ns",
    "/detail/candidate/phases/walk_generation/elapsed_ns",
    "/detail/cold_setup/elapsed_ns",
    "/env_class_id",
    "/isolation/after/intruders",
    "/isolation/after/loadavg/0",
    "/isolation/after/loadavg/1",
    "/isolation/after/loadavg/2",
    "/isolation/after/psi_cpu_some_avg10",
    "/isolation/after/psi_cpu_some_total_us",
    "/isolation/after/reserved_ticks/4",
    "/isolation/after/unix_ms",
    "/isolation/before/intruders",
    "/isolation/before/loadavg/0",
    "/isolation/before/loadavg/1",
    "/isolation/before/loadavg/2",
    "/isolation/before/psi_cpu_some_avg10",
    "/isolation/before/psi_cpu_some_total_us",
    "/isolation/before/reserved_ticks/4",
    "/isolation/before/unix_ms",
    "/isolation/blockers/#length",
    "/isolation/blockers/0",
    "/isolation/blockers/1",
    "/isolation/blockers/2",
    "/isolation/blockers/3",
    "/isolation/foreign_busy_ticks",
    "/isolation/idle_sibling_busy_ticks",
    "/isolation/level",
    "/isolation/node",
    "/isolation/placement/affinity/#length",
    "/isolation/placement/affinity/0",
    "/isolation/placement/affinity/1",
    "/isolation/placement/affinity/2",
    "/isolation/placement/affinity/3",
    "/isolation/placement/anon_pages_by_node/0",
    "/isolation/placement/cpu_at_end",
    "/isolation/placement/cpu_at_start",
    "/isolation/placement/mempolicy",
    "/isolation/reserved/#length",
    "/isolation/reserved/0",
    "/isolation/run_cpu",
    "/isolation/steal_ticks",
    "/online/phases_ns/recovery_check",
    "/online/phases_ns/rho_solve",
    "/online/phases_ns/target_PDP",
    "/online/phases_ns/target_descent",
    "/online/phases_ns/target_query",
    "/online/phases_ns/target_recovery_check",
    "/online/phases_ns/target_relation_check",
    "/online/wall_ns",
    "/record_id",
    "/session_id",
    "/time/involuntary_switches",
    "/time/max_rss_kib",
    "/time/method_call_wall_ns",
    "/time/minor_faults",
    "/time/parent_audit_verification_ns",
    "/time/process_wall_ns",
    "/time/schedstat_solve/cpu_ns",
    "/time/solve/cpu",
    "/time/solve/irq",
    "/time/solve/minor_faults",
    "/time/solve/schedstat_all/cpu_ns",
    "/time/solve_wall_ns",
    "/time/sys_ns",
    "/time/user_ns",
    "/time/voluntary_switches",
];

pub const PROJECTION_OMISSIONS: &[&str] = &[
    "/record_id",
    "/session_id",
    "/env_class_id",
    "/isolation",
    "/time",
    "/online/wall_ns",
    "/online/phases_ns",
    "/detail/adapter_accounting/adapter_elapsed_ns_before_accounting_attachment",
    "/detail/cold_setup/elapsed_ns",
    "/detail/candidate/candidate_elapsed_ns",
    "/detail/candidate/candidate_phase_overhead_ns",
    "/detail/candidate/cold_setup_elapsed_ns",
    "/detail/candidate/phases/*/elapsed_ns",
];

#[derive(Clone, Debug, Deserialize)]
pub struct BoundFile {
    pub path: PathBuf,
    pub sha256: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SideRequest {
    pub spec: BoundFile,
    pub plan: BoundFile,
    pub records: BoundFile,
    pub session: BoundFile,
    pub host: BoundFile,
    pub audit: BoundFile,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FreshProvenanceRequest {
    pub locators: BoundFile,
    pub object_versions: BoundFile,
    pub audit_head_receipt: BoundFile,
    pub sha256_manifest: BoundFile,
    pub expected_bucket: String,
    pub expected_key: String,
    pub expected_version_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ComputationProvenanceRequest {
    pub archive: BoundFile,
    pub archive_head_receipt: BoundFile,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Request {
    pub schema: String,
    pub expected_records: u64,
    pub expected_replay_candidates: u64,
    pub expected_binary_sha256: String,
    pub expected_git_commit: String,
    pub computation: SideRequest,
    pub fresh: SideRequest,
    pub computation_provenance: ComputationProvenanceRequest,
    pub fresh_provenance: FreshProvenanceRequest,
    pub reference_sources: Vec<BoundFile>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Binding {
    pub path: String,
    pub sha256: String,
    pub bytes: u64,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RecordKey {
    pub arm: String,
    pub role: String,
    pub warmup: bool,
    pub round: u64,
    pub algorithm_seed: u64,
    pub workload_id: String,
    pub method_id: String,
}

#[derive(Clone, Debug)]
struct LoadedSide {
    bindings: BTreeMap<String, Binding>,
    spec: Value,
    plan: Value,
    session: Value,
    host: Value,
    audit: Value,
    records: BTreeMap<RecordKey, Value>,
}

#[derive(Clone, Debug, Serialize)]
pub struct MappingRow {
    pub key: RecordKey,
    pub computation_seq: u64,
    pub fresh_seq: u64,
    pub computation_record_id: String,
    pub fresh_record_id: String,
    pub projection_sha256: String,
    pub deterministic_projection_equal: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct Comparison {
    pub rows: Vec<MappingRow>,
    pub observed_allowed_differences: BTreeMap<String, u64>,
    pub computation_projection_sha256: String,
    pub fresh_projection_sha256: String,
    pub normalized_plan_sha256: String,
    pub normalized_spec_sha256: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct AuditResult {
    pub schema: String,
    pub generated_unix_ms: u64,
    pub exact_deterministic_solver_accounting_equivalence: bool,
    pub all_non_wall_record_content_equal: bool,
    pub projection_scope: String,
    pub records_per_session: u64,
    pub mapped_records: u64,
    pub measured_records_per_session: u64,
    pub warmup_records_per_session: u64,
    pub disallowed_differences: u64,
    pub normalized_plan_sha256: String,
    pub normalized_spec_sha256: String,
    pub deterministic_projection_sha256: String,
    pub projection_bytes: u64,
    pub omission_rules: Vec<String>,
    pub allowed_difference_paths: Vec<String>,
    pub observed_allowed_differences: BTreeMap<String, u64>,
    pub computation_inputs: BTreeMap<String, Binding>,
    pub fresh_inputs: BTreeMap<String, Binding>,
    pub provenance: Value,
    pub reference_sources: Vec<Binding>,
    pub review_limits: Vec<String>,
    pub new_solver_executions: u64,
    pub new_measurements: u64,
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn load_bound(file: &BoundFile) -> Result<(Binding, Vec<u8>), String> {
    let bytes = fs::read(&file.path).map_err(|e| format!("read {}: {e}", file.path.display()))?;
    let actual = sha256(&bytes);
    if actual != file.sha256 {
        return Err(format!(
            "hash mismatch for {}: expected {}, got {}",
            file.path.display(),
            file.sha256,
            actual
        ));
    }
    Ok((
        Binding {
            path: file.path.display().to_string(),
            sha256: actual,
            bytes: u64::try_from(bytes.len()).map_err(|_| "file size overflow")?,
        },
        bytes,
    ))
}

fn parse_json(file: &BoundFile) -> Result<(Binding, Value), String> {
    let (binding, bytes) = load_bound(file)?;
    let value = serde_json::from_slice(&bytes)
        .map_err(|e| format!("parse {}: {e}", file.path.display()))?;
    Ok((binding, value))
}

fn text<'a>(value: &'a Value, pointer: &str) -> Result<&'a str, String> {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing string {pointer}"))
}

fn exact_u64(value: &Value, pointer: &str) -> Result<u64, String> {
    value
        .pointer(pointer)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("missing exact u64 {pointer}"))
}

fn exact_u64_or_decimal_string(value: &Value, pointer: &str) -> Result<u64, String> {
    let field = value
        .pointer(pointer)
        .ok_or_else(|| format!("missing exact u64 or decimal string {pointer}"))?;
    if let Some(number) = field.as_u64() {
        return Ok(number);
    }
    let digits = field
        .as_str()
        .filter(|text| !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit()))
        .ok_or_else(|| format!("invalid exact u64 decimal string {pointer}"))?;
    digits
        .parse::<u64>()
        .map_err(|_| format!("u64 decimal string overflow {pointer}"))
}

fn boolean(value: &Value, pointer: &str) -> Result<bool, String> {
    value
        .pointer(pointer)
        .and_then(Value::as_bool)
        .ok_or_else(|| format!("missing bool {pointer}"))
}

fn record_key(record: &Value) -> Result<RecordKey, String> {
    Ok(RecordKey {
        arm: text(record, "/arm")?.to_owned(),
        role: text(record, "/role")?.to_owned(),
        warmup: boolean(record, "/warmup")?,
        round: exact_u64(record, "/round")?,
        algorithm_seed: exact_u64_or_decimal_string(record, "/algorithm_seed")?,
        workload_id: text(record, "/workload/workload_id")?.to_owned(),
        method_id: text(record, "/method/method_id")?.to_owned(),
    })
}

fn inventory_records(
    values: impl IntoIterator<Item = Value>,
) -> Result<BTreeMap<RecordKey, Value>, String> {
    let mut result = BTreeMap::new();
    for value in values {
        let key = record_key(&value)?;
        if result.insert(key.clone(), value).is_some() {
            return Err(format!("duplicate record key: {key:?}"));
        }
    }
    Ok(result)
}

fn load_records(file: &BoundFile) -> Result<(Binding, BTreeMap<RecordKey, Value>), String> {
    let (binding, bytes) = load_bound(file)?;
    let content = std::str::from_utf8(&bytes).map_err(|e| format!("records UTF-8: {e}"))?;
    let mut values = Vec::new();
    for (index, line) in content.lines().enumerate() {
        if line.is_empty() {
            return Err(format!("empty records line {}", index + 1));
        }
        let value: Value =
            serde_json::from_str(line).map_err(|e| format!("records line {}: {e}", index + 1))?;
        if text(&value, "/schema")? != "ecbench.record/v1" {
            return Err(format!("wrong record schema on line {}", index + 1));
        }
        values.push(value);
    }
    Ok((binding, inventory_records(values)?))
}

fn load_side(request: &SideRequest) -> Result<LoadedSide, String> {
    let (spec_binding, spec) = parse_json(&request.spec)?;
    let (plan_binding, plan) = parse_json(&request.plan)?;
    let (session_binding, session) = parse_json(&request.session)?;
    let (host_binding, host) = parse_json(&request.host)?;
    let (audit_binding, audit) = parse_json(&request.audit)?;
    let (records_binding, records) = load_records(&request.records)?;
    let bindings = BTreeMap::from([
        ("spec".to_owned(), spec_binding),
        ("plan".to_owned(), plan_binding),
        ("session".to_owned(), session_binding),
        ("host".to_owned(), host_binding),
        ("audit".to_owned(), audit_binding),
        ("records".to_owned(), records_binding),
    ]);
    Ok(LoadedSide {
        bindings,
        spec,
        plan,
        session,
        host,
        audit,
        records,
    })
}

fn validate_records_against_plan(side: &LoadedSide, expected_records: u64) -> Result<(), String> {
    let workloads = side
        .plan
        .pointer("/workloads")
        .and_then(Value::as_array)
        .ok_or("plan workloads is not an array")?;
    let arms = side
        .plan
        .pointer("/arms")
        .and_then(Value::as_array)
        .ok_or("plan arms is not an array")?;
    let executions = side
        .plan
        .pointer("/executions")
        .and_then(Value::as_array)
        .ok_or("plan executions is not an array")?;
    if executions.len() as u64 != expected_records {
        return Err("plan execution count differs from expected records".into());
    }
    let mut records_by_seq = BTreeMap::new();
    for record in side.records.values() {
        let seq = exact_u64(record, "/seq")?;
        if records_by_seq.insert(seq, record).is_some() {
            return Err(format!("duplicate record seq {seq}"));
        }
    }
    let mut plan_seqs = BTreeSet::new();
    for execution in executions {
        let seq = exact_u64(execution, "/seq")?;
        if !plan_seqs.insert(seq) {
            return Err(format!("duplicate plan seq {seq}"));
        }
        let record = records_by_seq
            .get(&seq)
            .ok_or_else(|| format!("missing record for plan seq {seq}"))?;
        let workload_index = usize::try_from(exact_u64(execution, "/workload")?)
            .map_err(|_| "workload index overflow")?;
        let arm_index =
            usize::try_from(exact_u64(execution, "/arm")?).map_err(|_| "arm index overflow")?;
        let workload = workloads
            .get(workload_index)
            .ok_or_else(|| format!("invalid workload index at seq {seq}"))?;
        let arm = arms
            .get(arm_index)
            .ok_or_else(|| format!("invalid arm index at seq {seq}"))?;
        if record.pointer("/workload") != Some(workload)
            || text(record, "/arm")? != text(arm, "/name")?
            || text(record, "/role")? != text(arm, "/role")?
            || record.pointer("/method") != arm.pointer("/method")
            || exact_u64(record, "/round")? != exact_u64(execution, "/round")?
            || boolean(record, "/warmup")? != boolean(execution, "/warmup")?
            || exact_u64_or_decimal_string(record, "/algorithm_seed")?
                != exact_u64(execution, "/algorithm_seed")?
        {
            return Err(format!(
                "record does not match resolved plan slot at seq {seq}"
            ));
        }
    }
    if plan_seqs.len() != records_by_seq.len() {
        return Err("plan/record seq inventory is not a bijection".into());
    }
    Ok(())
}

fn validate_p739_scope(plan: &Value) -> Result<(), String> {
    let workloads = plan
        .pointer("/workloads")
        .and_then(Value::as_array)
        .ok_or("plan workloads is not an array")?;
    let arms = plan
        .pointer("/arms")
        .and_then(Value::as_array)
        .ok_or("plan arms is not an array")?;
    if workloads.len() != 60 || arms.len() != 3 {
        return Err("p739 gate plan must contain exactly 60 workloads and 3 arms".into());
    }
    if workloads
        .iter()
        .any(|workload| workload.pointer("/curve_spec/p").and_then(Value::as_u64) != Some(739))
    {
        return Err("p739 gate plan contains a workload from another prime".into());
    }
    Ok(())
}

fn validate_replay_coverage(
    audit: &Value,
    records: &BTreeMap<RecordKey, Value>,
    expected_replays: u64,
) -> Result<(), String> {
    let replays = audit
        .pointer("/replays")
        .and_then(Value::as_array)
        .ok_or("audit replays is not an array")?;
    if replays.len() as u64 != expected_replays {
        return Err("audit replay count differs".into());
    }
    let measured_records: BTreeMap<_, _> = records
        .values()
        .filter(|record| record.pointer("/warmup").and_then(Value::as_bool) == Some(false))
        .map(|record| {
            Ok((
                exact_u64(record, "/seq")?,
                text(record, "/record_id")?.to_owned(),
            ))
        })
        .collect::<Result<_, String>>()?;
    if measured_records.len() as u64 != expected_replays {
        return Err("actual measured record count differs from replay candidates".into());
    }
    let mut replay_seqs = BTreeSet::new();
    let mut replay_record_ids = BTreeSet::new();
    for replay in replays {
        if replay.pointer("/reproduced").and_then(Value::as_bool) != Some(true)
            || replay.pointer("/detail").and_then(Value::as_str) != Some("identical")
        {
            return Err("audit contains a non-identical replay".into());
        }
        let seq = exact_u64(replay, "/seq")?;
        let record_id = text(replay, "/record_id")?;
        if !replay_seqs.insert(seq) || !replay_record_ids.insert(record_id.to_owned()) {
            return Err("audit contains duplicate replay seq or record ID".into());
        }
        if measured_records.get(&seq).map(String::as_str) != Some(record_id) {
            return Err(format!(
                "audit replay does not match measured record at seq {seq}"
            ));
        }
    }
    if replay_seqs != measured_records.keys().copied().collect() {
        return Err("audit replay set is not the exact measured record set".into());
    }
    Ok(())
}

fn validate_side(
    side: &LoadedSide,
    expected_records: u64,
    expected_replays: u64,
    expected_binary: &str,
    expected_commit: &str,
) -> Result<(), String> {
    if text(&side.spec, "/schema")? != "ecbench.spec/v1"
        || text(&side.plan, "/schema")? != "ecbench.plan/v1"
        || text(&side.session, "/schema")? != "ecbench.session/v1"
        || text(&side.host, "/schema")? != "ecbench.host/v1"
        || text(&side.audit, "/schema")? != "ecbench.audit/v1"
    {
        return Err("unexpected ecbench schema".into());
    }
    if side.records.len() as u64 != expected_records {
        return Err(format!(
            "record count {} != {expected_records}",
            side.records.len()
        ));
    }
    if text(&side.session, "/status")? != "complete"
        || exact_u64(&side.session, "/executions_planned")? != expected_records
        || exact_u64(&side.session, "/records_written")? != expected_records
    {
        return Err("session is not the expected complete inventory".into());
    }
    let session_id = text(&side.session, "/session_id")?;
    let env_class_id = text(&side.session, "/env_class_id")?;
    if text(&side.session, "/binary_sha256")? != expected_binary
        || text(&side.session, "/git_commit")? != expected_commit
    {
        return Err("session build identity differs from the request".into());
    }
    if text(&side.session, "/plan_sha256")? != side.bindings["plan"].sha256
        || text(&side.session, "/records_sha256")? != side.bindings["records"].sha256
        || text(&side.session, "/host_sha256")? != side.bindings["host"].sha256
    {
        return Err("session raw plan, records or host binding differs".into());
    }
    // ecbench's spec_sha256 is its canonical spec identity, not the raw file
    // hash. The raw file is independently bound by the audit receipt below.
    let canonical_spec_sha = text(&side.session, "/spec_sha256")?;
    if canonical_spec_sha.len() != 64
        || !canonical_spec_sha
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err("invalid canonical spec SHA-256".into());
    }
    let expected_spec_id = format!("ECS1h{}", &canonical_spec_sha[..12]);
    if text(&side.session, "/spec_id")? != expected_spec_id
        || text(&side.plan, "/spec_id")? != expected_spec_id
    {
        return Err("canonical spec identity differs between session and plan".into());
    }
    if text(&side.host, "/env_class_id")? != env_class_id {
        return Err("host environment identity differs from session".into());
    }
    for record in side.records.values() {
        if text(record, "/session_id")? != session_id
            || text(record, "/binary_sha256")? != expected_binary
        {
            return Err("record session or binary binding differs".into());
        }
    }
    validate_records_against_plan(side, expected_records)?;
    validate_p739_scope(&side.plan)?;
    if !boolean(&side.audit, "/ok")?
        || !boolean(&side.audit, "/records_bound")?
        || !boolean(&side.audit, "/regraded")?
        || exact_u64(&side.audit, "/planned_executions")? != expected_records
        || exact_u64(&side.audit, "/records")? != expected_records
        || exact_u64(&side.audit, "/verified_records")? != expected_records
        || text(&side.audit, "/session_id")? != session_id
        || text(&side.audit, "/session_status")? != "complete"
        || text(&side.audit, "/replay_selection")? != "all"
        || exact_u64(&side.audit, "/replay_candidates")? != expected_replays
        || text(&side.audit, "/auditor_binary_sha256")? != expected_binary
        || text(&side.audit, "/auditor_env_class_id")? != env_class_id
    {
        return Err("audit receipt does not bind the expected complete session".into());
    }
    let problems = side
        .audit
        .pointer("/problems")
        .and_then(Value::as_array)
        .ok_or("audit problems is not an array")?;
    if !problems.is_empty() {
        return Err("audit receipt contains problems".into());
    }
    validate_replay_coverage(&side.audit, &side.records, expected_replays)?;
    for (name, binding_name) in [
        ("spec.json", "spec"),
        ("plan.json", "plan"),
        ("records.jsonl", "records"),
        ("session.json", "session"),
        ("host.json", "host"),
    ] {
        if text(&side.audit, &format!("/files/{name}"))? != side.bindings[binding_name].sha256 {
            return Err(format!("audit file binding mismatch for {name}"));
        }
    }
    Ok(())
}

fn escape_pointer(segment: &str) -> String {
    segment.replace('~', "~0").replace('/', "~1")
}

fn collect_differences(a: &Value, b: &Value, path: &str, out: &mut Vec<String>) {
    match (a, b) {
        (Value::Object(left), Value::Object(right)) => {
            let keys: BTreeSet<_> = left.keys().chain(right.keys()).collect();
            for key in keys {
                let child = format!("{}/{}", path, escape_pointer(key));
                match (left.get(key), right.get(key)) {
                    (Some(x), Some(y)) => collect_differences(x, y, &child, out),
                    _ => out.push(child),
                }
            }
        }
        (Value::Array(left), Value::Array(right)) => {
            if left.len() != right.len() {
                out.push(format!("{path}/#length"));
            }
            for (index, (x, y)) in left.iter().zip(right).enumerate() {
                collect_differences(x, y, &format!("{path}/{index}"), out);
            }
            for index in left.len().min(right.len())..left.len().max(right.len()) {
                out.push(format!("{path}/{index}"));
            }
        }
        _ if a != b => out.push(path.to_owned()),
        _ => {}
    }
}

fn remove_pointer(value: &mut Value, pointer: &str) {
    let Some((parent, leaf)) = pointer.rsplit_once('/') else {
        return;
    };
    let parent = if parent.is_empty() {
        Some(value)
    } else {
        value.pointer_mut(parent)
    };
    if let Some(Value::Object(map)) = parent {
        map.remove(leaf);
    }
}

pub fn deterministic_projection(record: &Value) -> Value {
    let mut result = record.clone();
    for pointer in [
        "/record_id",
        "/session_id",
        "/env_class_id",
        "/isolation",
        "/time",
        "/online/wall_ns",
        "/online/phases_ns",
        "/detail/adapter_accounting/adapter_elapsed_ns_before_accounting_attachment",
        "/detail/cold_setup/elapsed_ns",
        "/detail/candidate/candidate_elapsed_ns",
        "/detail/candidate/candidate_phase_overhead_ns",
        "/detail/candidate/cold_setup_elapsed_ns",
    ] {
        remove_pointer(&mut result, pointer);
    }
    if let Some(Value::Object(phases)) = result.pointer_mut("/detail/candidate/phases") {
        for phase in phases.values_mut() {
            if let Value::Object(fields) = phase {
                fields.remove("elapsed_ns");
            }
        }
    }
    result
}

fn normalized_plan(plan: &Value) -> Value {
    let mut result = plan.clone();
    remove_pointer(&mut result, "/spec_id");
    result
}

fn normalized_spec(spec: &Value) -> Value {
    let mut result = spec.clone();
    remove_pointer(&mut result, "/label");
    remove_pointer(&mut result, "/measurement/isolation_required");
    result
}

fn canonical_bytes(value: &Value) -> Result<Vec<u8>, String> {
    serde_json::to_vec(value).map_err(|e| e.to_string())
}

fn canonical_jsonl<'a>(values: impl IntoIterator<Item = &'a Value>) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    for value in values {
        output.extend(canonical_bytes(value)?);
        output.push(b'\n');
    }
    Ok(output)
}

fn ensure_same_record_keys(
    computation: &BTreeMap<RecordKey, Value>,
    fresh: &BTreeMap<RecordKey, Value>,
) -> Result<(), String> {
    let computation_keys: BTreeSet<_> = computation.keys().collect();
    let fresh_keys: BTreeSet<_> = fresh.keys().collect();
    if computation_keys != fresh_keys {
        let missing = computation_keys.difference(&fresh_keys).count();
        let extra = fresh_keys.difference(&computation_keys).count();
        return Err(format!(
            "record identity mismatch: missing={missing}, extra={extra}"
        ));
    }
    Ok(())
}

fn compare_loaded(computation: &LoadedSide, fresh: &LoadedSide) -> Result<Comparison, String> {
    let comp_plan = normalized_plan(&computation.plan);
    let fresh_plan = normalized_plan(&fresh.plan);
    if comp_plan != fresh_plan {
        return Err("normalized plans differ".into());
    }
    let comp_spec = normalized_spec(&computation.spec);
    let fresh_spec = normalized_spec(&fresh.spec);
    if comp_spec != fresh_spec {
        return Err("normalized specifications differ".into());
    }
    ensure_same_record_keys(&computation.records, &fresh.records)?;
    let allowed: BTreeSet<_> = ALLOWED_DIFFERENCE_PATHS.iter().copied().collect();
    let mut observed = BTreeMap::new();
    let mut rows = Vec::with_capacity(computation.records.len());
    let mut comp_projections = Vec::with_capacity(computation.records.len());
    let mut fresh_projections = Vec::with_capacity(fresh.records.len());
    for key in computation.records.keys() {
        let left = &computation.records[key];
        let right = &fresh.records[key];
        let mut differences = Vec::new();
        collect_differences(left, right, "", &mut differences);
        let unexpected: Vec<_> = differences
            .iter()
            .filter(|path| !allowed.contains(path.as_str()))
            .cloned()
            .collect();
        if !unexpected.is_empty() {
            return Err(format!(
                "unexpected differing paths for {key:?}: {unexpected:?}"
            ));
        }
        for path in differences {
            *observed.entry(path).or_insert(0u64) += 1;
        }
        let left_projection = deterministic_projection(left);
        let right_projection = deterministic_projection(right);
        if left_projection != right_projection {
            return Err(format!("deterministic projection differs for {key:?}"));
        }
        let projection_sha256 = sha256(&canonical_bytes(&left_projection)?);
        rows.push(MappingRow {
            key: key.clone(),
            computation_seq: exact_u64(left, "/seq")?,
            fresh_seq: exact_u64(right, "/seq")?,
            computation_record_id: text(left, "/record_id")?.to_owned(),
            fresh_record_id: text(right, "/record_id")?.to_owned(),
            projection_sha256,
            deterministic_projection_equal: true,
        });
        comp_projections.push(left_projection);
        fresh_projections.push(right_projection);
    }
    let comp_bytes = canonical_jsonl(comp_projections.iter())?;
    let fresh_bytes = canonical_jsonl(fresh_projections.iter())?;
    if comp_bytes != fresh_bytes {
        return Err("whole deterministic projection streams differ".into());
    }
    Ok(Comparison {
        rows,
        observed_allowed_differences: observed,
        computation_projection_sha256: sha256(&comp_bytes),
        fresh_projection_sha256: sha256(&fresh_bytes),
        normalized_plan_sha256: sha256(&canonical_bytes(&comp_plan)?),
        normalized_spec_sha256: sha256(&canonical_bytes(&comp_spec)?),
    })
}

fn verify_archive_members(
    archive_file: &BoundFile,
    computation: &LoadedSide,
) -> Result<Binding, String> {
    let (archive_binding, bytes) = load_bound(archive_file)?;
    let mut archive = tar::Archive::new(bytes.as_slice());
    let expected = BTreeMap::from([
        (
            "compute/session/spec.json",
            computation.bindings["spec"].sha256.as_str(),
        ),
        (
            "compute/session/plan.json",
            computation.bindings["plan"].sha256.as_str(),
        ),
        (
            "compute/session/records.jsonl",
            computation.bindings["records"].sha256.as_str(),
        ),
        (
            "compute/session/session.json",
            computation.bindings["session"].sha256.as_str(),
        ),
        (
            "compute/session/host.json",
            computation.bindings["host"].sha256.as_str(),
        ),
        (
            "compute/audit.json",
            computation.bindings["audit"].sha256.as_str(),
        ),
    ]);
    let mut seen = BTreeSet::new();
    for entry in archive.entries().map_err(|e| e.to_string())? {
        let mut entry = entry.map_err(|e| e.to_string())?;
        let path = entry
            .path()
            .map_err(|e| e.to_string())?
            .to_string_lossy()
            .to_string();
        if let Some(expected_hash) = expected.get(path.as_str()) {
            let mut member = Vec::new();
            entry.read_to_end(&mut member).map_err(|e| e.to_string())?;
            if sha256(&member) != *expected_hash {
                return Err(format!("archive member differs: {path}"));
            }
            if !seen.insert(path) {
                return Err("duplicate bound archive member".into());
            }
        }
    }
    if seen.len() != expected.len() {
        return Err(format!(
            "archive contained {} of {} bound members",
            seen.len(),
            expected.len()
        ));
    }
    Ok(archive_binding)
}

fn line_manifest_hash(manifest: &[u8], relative: &str) -> Option<String> {
    std::str::from_utf8(manifest)
        .ok()?
        .lines()
        .find_map(|line| {
            let (hash, path) = line.split_once("  ")?;
            (path == relative).then(|| hash.to_owned())
        })
}

fn validate_fresh_provenance(
    request: &FreshProvenanceRequest,
    fresh: &LoadedSide,
) -> Result<Value, String> {
    let (locators_binding, locators_bytes) = load_bound(&request.locators)?;
    let locator = std::str::from_utf8(&locators_bytes)
        .map_err(|e| e.to_string())?
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .find(|value| value["relative_path"] == "audit.json")
        .ok_or("fresh audit locator is absent")?;
    if text(&locator, "/bucket")? != request.expected_bucket
        || text(&locator, "/key")? != request.expected_key
        || text(&locator, "/version_id")? != request.expected_version_id
        || text(&locator, "/sha256")? != fresh.bindings["audit"].sha256
        || exact_u64(&locator, "/bytes")? != fresh.bindings["audit"].bytes
    {
        return Err("fresh audit locator does not bind the downloaded receipt".into());
    }
    let (head_binding, head) = parse_json(&request.audit_head_receipt)?;
    if text(&head, "/VersionId")? != request.expected_version_id
        || exact_u64(&head, "/ContentLength")? != fresh.bindings["audit"].bytes
    {
        return Err("fresh audit HEAD receipt differs from locator".into());
    }
    let (versions_binding, versions) = parse_json(&request.object_versions)?;
    let listed = versions
        .pointer("/Versions")
        .and_then(Value::as_array)
        .ok_or("object version list lacks Versions")?;
    let matching_versions: Vec<_> = listed
        .iter()
        .filter(|version| {
            version["Key"] == request.expected_key
                && version["VersionId"] == request.expected_version_id
        })
        .collect();
    if matching_versions.len() != 1 {
        return Err(format!(
            "expected one listed audit version, got {}",
            matching_versions.len()
        ));
    }
    let listed_versions_for_key = listed
        .iter()
        .filter(|version| version["Key"] == request.expected_key)
        .count();
    let (manifest_binding, manifest_bytes) = load_bound(&request.sha256_manifest)?;
    if line_manifest_hash(&manifest_bytes, "audit.json").as_deref()
        != Some(fresh.bindings["audit"].sha256.as_str())
    {
        return Err("fresh SHA256SUMS does not bind audit.json".into());
    }
    let audited_ms = exact_u64(&fresh.audit, "/audited_unix_ms")?;
    let finished_ms = exact_u64(&fresh.session, "/finished_unix_ms")?;
    if audited_ms < finished_ms {
        return Err("fresh audit predates session completion".into());
    }
    Ok(json!({
        "classification": "post-session versioned remote result artifact",
        "audit_sha256": fresh.bindings["audit"].sha256,
        "bucket": request.expected_bucket,
        "key": request.expected_key,
        "version_id": request.expected_version_id,
        "head_last_modified": text(&head, "/LastModified")?,
        "head_etag": text(&head, "/ETag")?,
        "audited_unix_ms": audited_ms,
        "session_finished_unix_ms": finished_ms,
        "audit_after_session_ms": audited_ms - finished_ms,
        "auditor_binary_matches_measurement_binary": true,
        "auditor_environment_matches_session_environment": true,
        "listed_versions_for_key": listed_versions_for_key,
        "matching_named_version_entries": matching_versions.len(),
        "independent_auditor_established": false,
        "origin_limit": "The evidence binds the exact retained bytes to one named object version and records when the receipt audited the session. It does not identify a separate auditor principal/process or prove auditor independence.",
        "evidence_bindings": [locators_binding, head_binding, versions_binding, manifest_binding]
    }))
}

fn generated_unix_ms() -> Result<u64, String> {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?;
    u64::try_from(duration.as_millis()).map_err(|_| "clock overflow".into())
}

pub fn execute(request: &Request) -> Result<(AuditResult, Vec<MappingRow>), String> {
    if request.schema != REQUEST_SCHEMA {
        return Err("wrong request schema".into());
    }
    if request.expected_records != 1080 || request.expected_replay_candidates != 900 {
        return Err(
            "this scoped p739 audit requires exactly 1080 records and 900 measured replays".into(),
        );
    }
    let computation = load_side(&request.computation)?;
    let fresh = load_side(&request.fresh)?;
    validate_side(
        &computation,
        request.expected_records,
        request.expected_replay_candidates,
        &request.expected_binary_sha256,
        &request.expected_git_commit,
    )?;
    validate_side(
        &fresh,
        request.expected_records,
        request.expected_replay_candidates,
        &request.expected_binary_sha256,
        &request.expected_git_commit,
    )?;
    let comparison = compare_loaded(&computation, &fresh)?;
    if comparison.rows.len() as u64 != request.expected_records
        || comparison.computation_projection_sha256 != comparison.fresh_projection_sha256
    {
        return Err("comparison does not cover the full equal projection".into());
    }
    let projection_bytes = canonical_jsonl(
        computation
            .records
            .values()
            .map(deterministic_projection)
            .collect::<Vec<_>>()
            .iter(),
    )?
    .len() as u64;
    let archive_binding =
        verify_archive_members(&request.computation_provenance.archive, &computation)?;
    let (archive_head_binding, archive_head) =
        parse_json(&request.computation_provenance.archive_head_receipt)?;
    if exact_u64(&archive_head, "/ContentLength")? != archive_binding.bytes {
        return Err("computation archive HEAD length differs from retained archive".into());
    }
    let fresh_provenance = validate_fresh_provenance(&request.fresh_provenance, &fresh)?;
    let mut references = Vec::new();
    for source in &request.reference_sources {
        references.push(load_bound(source)?.0);
    }
    let measured = fresh
        .records
        .values()
        .filter(|record| record.pointer("/warmup").and_then(Value::as_bool) == Some(false))
        .count() as u64;
    let computation_measured = computation
        .records
        .values()
        .filter(|record| record.pointer("/warmup").and_then(Value::as_bool) == Some(false))
        .count() as u64;
    if measured != request.expected_replay_candidates || computation_measured != measured {
        return Err("actual measured record inventories differ from the p739 contract".into());
    }
    let warmups = fresh
        .records
        .values()
        .filter(|record| record.pointer("/warmup").and_then(Value::as_bool) == Some(true))
        .count() as u64;
    let computation_warmups = computation
        .records
        .values()
        .filter(|record| record.pointer("/warmup").and_then(Value::as_bool) == Some(true))
        .count() as u64;
    if warmups != 180 || computation_warmups != warmups {
        return Err("actual warmup inventories differ from the p739 contract".into());
    }
    let computation_audited_ms = exact_u64(&computation.audit, "/audited_unix_ms")?;
    let computation_finished_ms = exact_u64(&computation.session, "/finished_unix_ms")?;
    let computation_audit_after_session_ms = computation_audited_ms
        .checked_sub(computation_finished_ms)
        .ok_or("computation audit predates session completion")?;
    let result = AuditResult {
        schema: SCHEMA.to_owned(),
        generated_unix_ms: generated_unix_ms()?,
        exact_deterministic_solver_accounting_equivalence: true,
        all_non_wall_record_content_equal: false,
        projection_scope: "Exact deterministic solver outcome, method, phase and operation-accounting content after excluding timing, process-resource, environment, isolation and session/record identity observations.".to_owned(),
        records_per_session: request.expected_records,
        mapped_records: comparison.rows.len() as u64,
        measured_records_per_session: measured,
        warmup_records_per_session: warmups,
        disallowed_differences: 0,
        normalized_plan_sha256: comparison.normalized_plan_sha256,
        normalized_spec_sha256: comparison.normalized_spec_sha256,
        deterministic_projection_sha256: comparison.computation_projection_sha256,
        projection_bytes,
        omission_rules: PROJECTION_OMISSIONS.iter().map(|s| (*s).to_owned()).collect(),
        allowed_difference_paths: ALLOWED_DIFFERENCE_PATHS
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        observed_allowed_differences: comparison.observed_allowed_differences,
        computation_inputs: computation.bindings.clone(),
        fresh_inputs: fresh.bindings.clone(),
        provenance: json!({
            "computation": {
                "classification": "members byte-matched to the hash-bound retained controller archive",
                "archive": archive_binding,
                "archive_head_receipt": archive_head_binding,
                "archive_version_id": text(&archive_head, "/VersionId")?,
                "archive_last_modified": text(&archive_head, "/LastModified")?,
                "audit_sha256": computation.bindings["audit"].sha256,
                "audited_unix_ms": computation_audited_ms,
                "session_finished_unix_ms": computation_finished_ms,
                "audit_after_session_ms": computation_audit_after_session_ms
            },
            "fresh_audit_receipt": fresh_provenance
        }),
        reference_sources: references,
        review_limits: vec![
            "Both sessions used the same measurement binary; this is not an independent implementation check.".to_owned(),
            "The 900 measured rows were replayed identically by each retained ecbench audit receipt; 180 warmups were statically verified and cross-session compared but were not replay candidates.".to_owned(),
            "Timing, isolation, environment, session IDs and record IDs are excluded; this does not validate wall time, admit ratios or qualify either session for a timing claim.".to_owned(),
            "Excluded process-resource observations include maximum RSS, faults, voluntary and involuntary context switches, scheduler CPU time/CPU identity, IRQ observations, and user/system CPU time. No equality of complete resource costs is claimed.".to_owned(),
            "The fresh receipt is version-bound and post-session, but the preserved evidence does not establish an independent auditor principal or process.".to_owned(),
            "This software audit changes no frozen protocol, record, plan or measurement artifact.".to_owned(),
        ],
        new_solver_executions: 0,
        new_measurements: 0,
    };
    Ok((result, comparison.rows))
}

pub fn read_request(path: &Path) -> Result<Request, String> {
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    serde_json::from_slice(&bytes).map_err(|e| e.to_string())
}

pub fn write_outputs(
    output: &Path,
    result: &AuditResult,
    rows: &[MappingRow],
) -> Result<(), String> {
    fs::create_dir(output).map_err(|e| e.to_string())?;
    fs::write(
        output.join("audit.json"),
        serde_json::to_vec_pretty(result).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let mapping_values = rows
        .iter()
        .map(|row| serde_json::to_value(row).map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    fs::write(
        output.join("row-mapping.json"),
        serde_json::to_vec(&mapping_values).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    fs::write(output.join("equivalence.svg"), render_svg(result)).map_err(|e| e.to_string())?;
    fs::write(output.join("report.tex"), render_tex(result)).map_err(|e| e.to_string())?;
    Ok(())
}

fn render_svg(result: &AuditResult) -> String {
    let bars = [
        (
            "Mapped deterministic projections",
            result.mapped_records,
            "#286f9b",
        ),
        (
            "Measured replay receipts",
            result.measured_records_per_session,
            "#4f8f68",
        ),
        (
            "Warmups: static comparison",
            result.warmup_records_per_session,
            "#9a7b35",
        ),
        (
            "Disallowed differences",
            result.disallowed_differences,
            "#b23a48",
        ),
    ];
    let mut svg = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"900\" height=\"330\" viewBox=\"0 0 900 330\"><rect width=\"900\" height=\"330\" fill=\"white\"/><g font-family=\"sans-serif\" fill=\"#172b4d\"><text x=\"24\" y=\"32\" font-size=\"21\">p739 retained-session equivalence inventory</text><text x=\"24\" y=\"55\" font-size=\"13\">Deterministic solver/outcome/operation accounting only. No solver or new measurement.</text>");
    for (index, (label, count, color)) in bars.iter().enumerate() {
        let y = 88 + index * 49;
        let width = count.saturating_mul(520) / result.records_per_session.max(1);
        svg.push_str(&format!("<text x=\"24\" y=\"{}\" font-size=\"14\">{label}</text><rect x=\"260\" y=\"{y}\" width=\"{width}\" height=\"25\" fill=\"{color}\"/><text x=\"800\" y=\"{}\" font-size=\"14\">{count}</text>", y + 18, y + 18));
    }
    svg.push_str("<line x1=\"260\" y1=\"278\" x2=\"780\" y2=\"278\" stroke=\"#52606d\"/><text x=\"260\" y=\"298\" font-size=\"12\">0</text><text x=\"735\" y=\"298\" font-size=\"12\">1,080 records</text><text x=\"24\" y=\"320\" font-size=\"11\">Timing, process-resource, isolation, environment and identity observations are excluded.</text></g></svg>");
    svg
}

fn tex_escape(value: &str) -> String {
    value
        .replace('\\', "\\textbackslash{}")
        .replace('_', "\\_")
        .replace('%', "\\%")
        .replace('#', "\\#")
        .replace('&', "\\&")
}

fn render_tex(result: &AuditResult) -> String {
    let limits = result
        .review_limits
        .iter()
        .map(|limit| format!("\\item {}\n", tex_escape(limit)))
        .collect::<String>();
    format!(
        r#"\documentclass[10pt]{{article}}
\usepackage[margin=0.72in]{{geometry}}
\usepackage{{booktabs}}
\usepackage{{tikz}}
\usepackage[T1]{{fontenc}}
\begin{{document}}
\section*{{p739 retained cross-session audit}}
\textbf{{Observations only.}} This report records a native, solver-free comparison of two preserved sessions. It makes no timing, ratio, admission, or scientific-priority judgment.

\subsection*{{Recorded comparison}}
\begin{{tabular}}{{lr}}
\toprule
Quantity & Count \\
\midrule
Records in each session & {records} \\
Exact deterministic solver/accounting projections & {mapped} \\
Measured rows replayed identically by each retained audit & {measured} \\
Warmups statically verified and cross-session compared & {warmups} \\
Disallowed differing paths & {disallowed} \\
New solver executions / measurements & 0 / 0 \\
\bottomrule
\end{{tabular}}

\begin{{tikzpicture}}[x=0.0085cm,y=0.62cm]
\node[anchor=east] at (0,0) {{mapped}}; \fill[blue!60] (0,-0.35) rectangle ({mapped},0.35);
\node[anchor=east] at (0,-1) {{replayed}}; \fill[green!55!black] (0,-1.35) rectangle ({measured},-0.65);
\node[anchor=east] at (0,-2) {{warmups}}; \fill[orange!70!black] (0,-2.35) rectangle ({warmups},-1.65);
\node[anchor=east] at (0,-3) {{disallowed}}; \fill[red!65] (0,-3.35) rectangle ({disallowed},-2.65);
\draw (0,-3.65) -- ({records},-3.65); \node[anchor=north] at (0,-3.65) {{0}}; \node[anchor=north] at ({records},-3.65) {{{records}}};
\end{{tikzpicture}}

\subsection*{{Hash-bound result}}
Normalized plan SHA-256: \texttt{{\footnotesize {plan_hash}}}\\
Normalized specification SHA-256: \texttt{{\footnotesize {spec_hash}}}\\
Deterministic projection SHA-256: \texttt{{\footnotesize {projection_hash}}}\\
Projection bytes: {projection_bytes}.

The row mapping and complete input bindings are in \texttt{{row-mapping.json}} and \texttt{{audit.json}}. The comparison pairs exact unsigned 64-bit seeds; it never converts them to floating point. Every observed difference is checked against a fixed leaf-level allowlist before the deterministic solver/accounting projection is formed. An unlisted or newly differing field causes failure. The projection excludes wall and CPU timing, maximum RSS, faults, context switches, scheduler CPU observations, IRQ observations, isolation/environment fields, and session/record identities. It therefore does not establish equality of all non-wall or all resource content.

\subsection*{{Fresh audit-receipt provenance}}
The compared fresh audit is the exact retained bytes of one named, versioned remote result object. Its own timestamp is after session completion, its binary and environment identifiers match the fresh session, and the downloaded SHA manifest binds the same bytes. These records do not establish a separate auditor principal or process.

\subsection*{{Review limits}}
\begin{{itemize}}
{limits}\end{{itemize}}
\end{{document}}
"#,
        records = result.records_per_session,
        mapped = result.mapped_records,
        measured = result.measured_records_per_session,
        warmups = result.warmup_records_per_session,
        disallowed = result.disallowed_differences,
        plan_hash = result.normalized_plan_sha256,
        spec_hash = result.normalized_spec_sha256,
        projection_hash = result.deterministic_projection_sha256,
        projection_bytes = result.projection_bytes,
        limits = limits,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(seed: u64) -> Value {
        json!({
            "schema":"ecbench.record/v1",
            "record_id":"a",
            "session_id":"s1",
            "env_class_id":"e1",
            "arm":"rho",
            "role":"baseline",
            "warmup":false,
            "round":0,
            "algorithm_seed":seed,
            "seq":0,
            "binary_sha256":"b",
            "workload":{"workload_id":"w","target":{"x":"1"}},
            "method":{"method_id":"m"},
            "cost":{"total_gae":17},
            "time":{"solve_wall_ns":10},
            "isolation":{"level":"L0"},
            "online":{"wall_ns":10,"phases_ns":{"rho_solve":9}},
            "detail":{"candidate":{"phases":{"recovery_check":{"elapsed_ns":2,"adds":3}}}}
        })
    }

    fn compare_records(left: Value, right: Value) -> Result<(), String> {
        let key_left = record_key(&left)?;
        let key_right = record_key(&right)?;
        if key_left != key_right {
            return Err("record identity mismatch".into());
        }
        let mut differences = Vec::new();
        collect_differences(&left, &right, "", &mut differences);
        let allowed: BTreeSet<_> = ALLOWED_DIFFERENCE_PATHS.iter().copied().collect();
        let unexpected: Vec<_> = differences
            .iter()
            .filter(|path| !allowed.contains(path.as_str()))
            .collect();
        if !unexpected.is_empty() {
            return Err(format!("unexpected: {unexpected:?}"));
        }
        if deterministic_projection(&left) != deterministic_projection(&right) {
            return Err("projection differs".into());
        }
        Ok(())
    }

    fn side_with_plan(record: Value) -> LoadedSide {
        let plan = json!({
            "workloads":[record["workload"].clone()],
            "arms":[{"name":record["arm"].clone(),"role":record["role"].clone(),"method":record["method"].clone()}],
            "executions":[{"seq":record["seq"].clone(),"round":record["round"].clone(),"warmup":record["warmup"].clone(),"arm":0,"workload":0,"algorithm_seed":exact_u64_or_decimal_string(&record,"/algorithm_seed").unwrap()}]
        });
        LoadedSide {
            bindings: BTreeMap::new(),
            spec: json!({}),
            plan,
            session: json!({}),
            host: json!({}),
            audit: json!({}),
            records: inventory_records([record]).unwrap(),
        }
    }

    #[test]
    fn exact_u64_seed_survives() {
        let value = record(u64::MAX);
        assert_eq!(record_key(&value).unwrap().algorithm_seed, u64::MAX);
        let encoded = canonical_bytes(&value).unwrap();
        assert!(std::str::from_utf8(&encoded)
            .unwrap()
            .contains("18446744073709551615"));
    }

    #[test]
    fn exact_decimal_string_seed_above_f64_precision_survives() {
        let mut value = record(1);
        value["algorithm_seed"] = json!("17823097723983109480");
        assert_eq!(
            record_key(&value).unwrap().algorithm_seed,
            17_823_097_723_983_109_480
        );
    }

    #[test]
    fn malformed_or_overflow_decimal_seed_is_rejected() {
        for invalid in ["", "-1", "1.0", "18446744073709551616"] {
            let mut value = record(1);
            value["algorithm_seed"] = json!(invalid);
            assert!(record_key(&value).is_err(), "accepted {invalid}");
        }
    }

    #[test]
    fn allowed_timing_and_identity_differences_pass() {
        let left = record(9_007_199_254_740_993);
        let mut right = left.clone();
        right["record_id"] = json!("b");
        right["session_id"] = json!("s2");
        right["env_class_id"] = json!("e2");
        right["time"]["solve_wall_ns"] = json!(11);
        right["online"]["wall_ns"] = json!(12);
        right["online"]["phases_ns"]["rho_solve"] = json!(10);
        right["isolation"]["level"] = json!("L2");
        right["detail"]["candidate"]["phases"]["recovery_check"]["elapsed_ns"] = json!(3);
        compare_records(left, right).unwrap();
    }

    #[test]
    fn mutated_cost_is_rejected() {
        let left = record(1);
        let mut right = left.clone();
        right["cost"]["total_gae"] = json!(18);
        assert!(compare_records(left, right)
            .unwrap_err()
            .contains("/cost/total_gae"));
    }

    #[test]
    fn mutated_seed_is_rejected_as_identity_change() {
        let left = record(9_007_199_254_740_993);
        let right = record(9_007_199_254_740_992);
        assert!(compare_records(left, right)
            .unwrap_err()
            .contains("identity"));
    }

    #[test]
    fn mutated_workload_is_rejected() {
        let left = record(1);
        let mut right = left.clone();
        right["workload"]["target"]["x"] = json!("2");
        assert!(compare_records(left, right)
            .unwrap_err()
            .contains("/workload/target/x"));
    }

    #[test]
    fn future_unknown_field_difference_is_rejected() {
        let left = record(1);
        let mut right = left.clone();
        right["future_schema_field"] = json!({"changed":true});
        assert!(compare_records(left, right)
            .unwrap_err()
            .contains("/future_schema_field"));
    }

    #[test]
    fn future_trailing_array_element_is_not_hidden_by_allowed_length_difference() {
        let mut left = record(1);
        let mut right = left.clone();
        left["isolation"]["blockers"] = json!(["a", "b", "c", "d"]);
        right["isolation"]["blockers"] = json!(["a", "b", "c", "d", "future"]);
        assert!(compare_records(left, right)
            .unwrap_err()
            .contains("/isolation/blockers/4"));
    }

    #[test]
    fn record_seed_and_method_must_match_resolved_plan_slot() {
        let original = record(9_007_199_254_740_993);
        let mut seed_mutated = side_with_plan(original.clone());
        seed_mutated.records.values_mut().next().unwrap()["algorithm_seed"] =
            json!("9007199254740992");
        assert!(validate_records_against_plan(&seed_mutated, 1)
            .unwrap_err()
            .contains("resolved plan slot"));

        let mut method_mutated = side_with_plan(original);
        method_mutated.records.values_mut().next().unwrap()["method"]["method_id"] =
            json!("different");
        assert!(validate_records_against_plan(&method_mutated, 1)
            .unwrap_err()
            .contains("resolved plan slot"));
    }

    #[test]
    fn replay_receipt_requires_distinct_exact_measured_record_ids() {
        let first = record(1);
        let mut second = record(2);
        second["seq"] = json!(1);
        second["record_id"] = json!("b");
        let records = inventory_records([first, second]).unwrap();
        let audit = json!({"replays":[
            {"seq":0,"record_id":"a","reproduced":true,"detail":"identical"},
            {"seq":0,"record_id":"a","reproduced":true,"detail":"identical"}
        ]});
        assert!(validate_replay_coverage(&audit, &records, 2)
            .unwrap_err()
            .contains("duplicate replay"));
    }

    #[test]
    fn duplicate_record_is_rejected_by_inventory() {
        let first = record(1);
        assert!(inventory_records([first.clone(), first])
            .unwrap_err()
            .contains("duplicate record key"));
    }

    #[test]
    fn missing_record_is_rejected_by_key_bijection() {
        let first = record(1);
        let second = record(2);
        let complete = inventory_records([first, second]).unwrap();
        let incomplete = inventory_records([record(1)]).unwrap();
        let error = ensure_same_record_keys(&complete, &incomplete).unwrap_err();
        assert!(error.contains("missing=1"));
    }
}
