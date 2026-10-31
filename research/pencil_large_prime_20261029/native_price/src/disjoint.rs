//! Fail-closed admission of the reviewed p739 disjoint enrollment packet.
//!
//! This module does not generate plans or run records.  It invokes the separately
//! compiled, hash-pinned native preparation verifier so that its `preserve_order`
//! serde feature cannot change the historical native-price serialization contract.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

pub const SCOPE_KIND: &str = "full1260-disjoint-p739/v1";
pub const REQUEST_SHA256: &str = "6b99db317edac83a401126ec1d427739b2e786591e4fbaffd0003ce882c846f2";
pub const CERTIFICATE_SHA256: &str =
    "de0003271c4f49898dd38f261c279338bfe756779d405d0753d0401d58b83e72";
pub const VERIFIER_SHA256: &str =
    "8ce29108cbb6471bee85aa4e1b6087f6ef9340c7d25e8161079cb28e62806820";
pub const VERIFIER_SOURCE_SHA256: &str =
    "583377eed49f1f6d409376d8f57040379be95671e9274613f579319225ff19ae";
pub const PRIOR_GATE_PROTOCOL_SHA256: &str =
    "cf8f76eadc42ff487644ea13b3392af1fd31c94e4629eecd6ad7b9a18820badb";
pub const GATE_PLAN_SHA256: &str =
    "bc57819e43862f5d9c9aee0f13b983fec186b2dd14dbc8b921d8999acc4b6776";
pub const GATE_RECORDS_SHA256: &str =
    "9ccc07c9b55b9180af36c3b910891803dde7f4249742881a7322f748bfdb73f7";
pub const GATE_SESSION_ID: &str = "ECBS1h4c7220d5a9f6";

const ENROLLED: [(&str, &str, &str); 8] = [
    ("gate-p739.json", GATE_PLAN_SHA256, "prior_gate"),
    (
        "cohort-16-remainder-plan.json",
        "39b73d2a2ffbab3deec9dbdf9857af15a66607ca8843b8f29c64cd94c7d2d9c9",
        "prospective_remainder",
    ),
    (
        "cohort-20.json",
        "51a99e83c38b4d5d6b19f82337918b2f64f34b4934a5361298e84fc2f8bd432f",
        "prospective_unchanged_cohort",
    ),
    (
        "cohort-24.json",
        "7d34d0ce87c5a167ee1e94fecb08bcac34c9b4339570deb6377effe75efc28c4",
        "prospective_unchanged_cohort",
    ),
    (
        "cohort-28.json",
        "87760bc68b7598250c22976f15e4dbb36b5f2d738647de501222de1a8b27b001",
        "prospective_unchanged_cohort",
    ),
    (
        "cohort-32.json",
        "a9aebbc3120b61380a6936408635ac0f0e60715b4b4918dc826fbab4b578ae96",
        "prospective_unchanged_cohort",
    ),
    (
        "cohort-36.json",
        "6bb162d846bfbfe68b1bf7ba330abbc1e11c403b003215c960e7e71d81be3aa2",
        "prospective_unchanged_cohort",
    ),
    (
        "cohort-40.json",
        "8f2ec73a139f84b660573bd67c54ebfc567626ce7068a135dcb52c61891b22d5",
        "prospective_unchanged_cohort",
    ),
];

fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn protocol_hash(value: &Value) -> Result<String, String> {
    let mut body = value.clone();
    body.as_object_mut()
        .ok_or("protocol must be an object")?
        .remove("freeze");
    Ok(sha(&serde_json::to_vec(&body).map_err(|e| e.to_string())?))
}
fn text<'a>(v: &'a Value, p: &str) -> Result<&'a str, String> {
    v.pointer(p)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing string {p}"))
}
fn array<'a>(v: &'a Value, p: &str) -> Result<&'a Vec<Value>, String> {
    v.pointer(p)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("missing array {p}"))
}
fn exact_keys(v: &Value, expected: &[&str], what: &str) -> Result<(), String> {
    let object = v
        .as_object()
        .ok_or_else(|| format!("{what} must be object"))?;
    let got: BTreeSet<_> = object.keys().map(String::as_str).collect();
    let want: BTreeSet<_> = expected.iter().copied().collect();
    if got != want {
        return Err(format!("{what} keys differ"));
    }
    Ok(())
}
fn canonical_hex(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}
fn read_binding(v: &Value, what: &str) -> Result<(PathBuf, Vec<u8>), String> {
    exact_keys(v, &["path", "sha256"], what)?;
    let digest = text(v, "/sha256")?;
    if !canonical_hex(digest) {
        return Err(format!("{what} digest is not canonical SHA256"));
    }
    let path = PathBuf::from(text(v, "/path")?);
    let bytes = fs::read(&path).map_err(|e| format!("{what}: {e}"))?;
    if sha(&bytes) != digest {
        return Err(format!("{what} digest mismatch"));
    }
    Ok((path, bytes))
}
fn read_json_binding(v: &Value, what: &str) -> Result<(PathBuf, Vec<u8>, Value), String> {
    let (path, bytes) = read_binding(v, what)?;
    let parsed = serde_json::from_slice(&bytes).map_err(|e| format!("{what}: {e}"))?;
    Ok((path, bytes, parsed))
}

pub fn full1260_disjoint_p739_scope() -> Value {
    json!({
        "kind": SCOPE_KIND,
        "expected_workloads": 1260,
        "expected_plans": 8,
        "prepared_enrollment_schema": "pencil-disjoint-enrollment/v1",
        "enrollment_request_sha256": REQUEST_SHA256,
        "enrollment_certificate_sha256": CERTIFICATE_SHA256,
        "native_preparation_verifier_sha256": VERIFIER_SHA256,
        "native_preparation_verifier_source_sha256": VERIFIER_SOURCE_SHA256,
    })
}

fn invoke_verifier(path: &Path, request: &Path, output: &Path) -> Result<(), String> {
    let result = Command::new(path)
        .arg("check-disjoint-enrollment")
        .arg(request)
        .arg(output)
        .output()
        .map_err(|e| format!("native preparation verifier failed to start: {e}"))?;
    if !result.status.success() {
        return Err(format!(
            "native preparation verifier rejected enrollment: status={:?}; stderr={}",
            result.status.code(),
            String::from_utf8_lossy(&result.stderr)
        ));
    }
    Ok(())
}

fn validate_reuse_admission(
    binding: &Value,
    observation_receipt_sha256: &str,
    gate_records_sha256: &str,
    gate_session_id: &str,
) -> Result<Value, String> {
    let (_, _, admission) = read_json_binding(binding, "scientific reuse admission")?;
    exact_keys(
        &admission,
        &[
            "admitted",
            "gate_data_protocol_sha256",
            "gate_observation_receipt_sha256",
            "gate_plan_sha256",
            "gate_records_sha256",
            "gate_session_id",
            "independent_review",
            "reviewer",
            "schema",
            "scope",
        ],
        "scientific reuse admission",
    )?;
    if admission["schema"] != "pencil-disjoint-scientific-reuse-admission/v1"
        || admission["admitted"] != true
        || admission["scope"] != "exact_prior_p739_gate_records_only"
        || admission["gate_plan_sha256"] != GATE_PLAN_SHA256
        || admission["gate_data_protocol_sha256"] != PRIOR_GATE_PROTOCOL_SHA256
        || admission["gate_observation_receipt_sha256"] != observation_receipt_sha256
        || admission["gate_records_sha256"] != gate_records_sha256
        || admission["gate_session_id"] != gate_session_id
        || text(&admission, "/reviewer")?.is_empty()
    {
        return Err("scientific reuse admission identity/scope mismatch".into());
    }
    let (_, _, review) =
        read_json_binding(&admission["independent_review"], "independent reuse review")?;
    exact_keys(
        &review,
        &[
            "decision",
            "findings",
            "gate_data_protocol_sha256",
            "gate_observation_receipt_sha256",
            "gate_plan_sha256",
            "gate_records_sha256",
            "gate_session_id",
            "producer",
            "reviewer",
            "schema",
            "status",
        ],
        "independent reuse review",
    )?;
    if review["schema"] != "pencil-disjoint-reuse-review/v1"
        || review["status"] != "reviewed"
        || review["decision"] != "admit_exact_prior_p739_gate_records_only"
        || review["gate_plan_sha256"] != GATE_PLAN_SHA256
        || review["gate_data_protocol_sha256"] != PRIOR_GATE_PROTOCOL_SHA256
        || review["gate_observation_receipt_sha256"] != observation_receipt_sha256
        || review["gate_records_sha256"] != GATE_RECORDS_SHA256
        || review["gate_session_id"] != GATE_SESSION_ID
        || !review["findings"].as_array().is_some_and(Vec::is_empty)
        || text(&review, "/reviewer")?.is_empty()
        || text(&review, "/producer")?.is_empty()
        || review["reviewer"] == review["producer"]
        || admission["reviewer"] != review["reviewer"]
    {
        return Err("independent scientific reuse review mismatch".into());
    }
    Ok(admission)
}

fn validate_lineage(
    lineage: &Value,
    manifest_plans: &[Value],
    certificate: &Value,
    certificate_binding: &Value,
) -> Result<Value, String> {
    exact_keys(
        lineage,
        &[
            "continuation_protocol",
            "known_data_declaration",
            "plans",
            "schema",
        ],
        "disjoint protocol lineage",
    )?;
    if lineage["schema"] != "pencil-disjoint-protocol-lineage/v1" {
        return Err("unsupported disjoint protocol lineage schema".into());
    }
    exact_keys(
        &lineage["continuation_protocol"],
        &["file", "protocol_sha256"],
        "continuation protocol",
    )?;
    let (_, _, continuation_protocol) = read_json_binding(
        &lineage["continuation_protocol"]["file"],
        "continuation protocol file",
    )?;
    let continuation_sha = text(&lineage["continuation_protocol"], "/protocol_sha256")?;
    if !canonical_hex(continuation_sha) {
        return Err("continuation protocol identity is not canonical SHA256".into());
    }
    if continuation_protocol["schema"] != "experiment.protocol/v1"
        || continuation_protocol
            .pointer("/analysis_plan/frozen_before_data")
            .and_then(Value::as_bool)
            != Some(false)
    {
        return Err("continuation protocol is not the declared prospective amendment".into());
    }
    if continuation_protocol.pointer("/freeze/protocol_sha256") != Some(&json!(continuation_sha))
        || protocol_hash(&continuation_protocol)? != continuation_sha
    {
        return Err("continuation protocol canonical identity mismatch".into());
    }

    let known = &lineage["known_data_declaration"];
    exact_keys(
        known,
        &[
            "existing_gate_data_known_at_design",
            "gate_observation_receipt",
            "scientific_reuse_admission",
        ],
        "known-data declaration",
    )?;
    if known["existing_gate_data_known_at_design"] != true {
        return Err("prior gate data must be declared known at continuation design".into());
    }
    if continuation_protocol.pointer("/freeze/status") != Some(&json!("frozen"))
        || continuation_protocol.pointer("/sample_size/instances") != Some(&json!(1260))
        || continuation_protocol.pointer("/extensions/analysis_scope")
            != Some(&full1260_disjoint_p739_scope())
    {
        return Err("continuation protocol freeze/scope mismatch".into());
    }
    let declaration = continuation_protocol
        .pointer("/extensions/disjoint_enrollment")
        .ok_or("continuation disjoint declaration missing")?;
    if declaration["known_prior_gate_data"] != true
        || declaration["freeze_scope"] != "prospective_continuation_only"
        || declaration["certificate"] != *certificate_binding
        || declaration["gate_observation_receipt"] != known["gate_observation_receipt"]
        || declaration["scientific_reuse_admission"] != known["scientific_reuse_admission"]
    {
        return Err("frozen continuation does not bind exact known-data evidence".into());
    }
    let amendment_id = text(declaration, "/known_data_amendment_id")?;
    let amendments = array(&continuation_protocol, "/amendments")?;
    let mut found = false;
    for amendment in amendments {
        if amendment["id"] == amendment_id {
            if found || amendment["filed_before_affected_data"] != false {
                return Err("known-data amendment timing/identity mismatch".into());
            }
            found = true;
        } else if amendment["filed_before_affected_data"] != true {
            return Err("unrelated post-data amendment in continuation protocol".into());
        }
    }
    if !found {
        return Err("known-data amendment absent from continuation protocol".into());
    }
    let (_, observation_receipt, _) = read_json_binding(
        &known["gate_observation_receipt"],
        "gate observation receipt",
    )?;
    let certificate_plans = array(certificate, "/enrolled_plans")?;
    let lineage_plans = array(lineage, "/plans")?;
    if manifest_plans.len() != ENROLLED.len()
        || lineage_plans.len() != ENROLLED.len()
        || certificate_plans.len() != ENROLLED.len()
    {
        return Err("disjoint plan inventory cardinality mismatch".into());
    }
    let cert_by_name: BTreeMap<_, _> = certificate_plans
        .iter()
        .map(|v| Ok((text(v, "/name")?, v)))
        .collect::<Result<_, String>>()?;
    let manifest_by_name: BTreeMap<_, _> = manifest_plans
        .iter()
        .map(|v| Ok((text(v, "/name")?, v)))
        .collect::<Result<_, String>>()?;
    if cert_by_name.len() != ENROLLED.len() || manifest_by_name.len() != ENROLLED.len() {
        return Err("duplicate disjoint plan name".into());
    }
    let prior_protocol = array(certificate, "/protocol_lineage")?
        .iter()
        .find(|binding| binding["name"] == "prior-v5-protocol-file")
        .ok_or("certificate prior protocol binding missing")?;
    let prior_protocol_binding = json!({
        "path": prior_protocol["path"],
        "sha256": prior_protocol["sha256"],
    });
    let mut seen = BTreeSet::new();
    let mut seen_sessions = BTreeSet::new();
    let mut validated = vec![];
    let mut gate_records_sha256 = None;
    let mut gate_session_id = None;
    for plan in lineage_plans {
        exact_keys(
            plan,
            &[
                "coverage_review",
                "data_protocol",
                "execution_history",
                "name",
                "origin",
                "plan_sha256",
                "records",
                "session_id",
            ],
            "lineage plan",
        )?;
        let name = text(plan, "/name")?;
        if !seen.insert(name) {
            return Err("duplicate lineage plan name".into());
        }
        let (_, want_sha, classification) = ENROLLED
            .iter()
            .find(|(n, _, _)| *n == name)
            .ok_or("unknown lineage plan")?;
        if text(plan, "/plan_sha256")? != *want_sha {
            return Err("lineage plan digest mismatch".into());
        }
        let cert = cert_by_name.get(name).ok_or("certificate plan missing")?;
        if text(cert, "/sha256")? != *want_sha || text(cert, "/classification")? != *classification
        {
            return Err("certificate enrollment classification mismatch".into());
        }
        let manifest = manifest_by_name.get(name).ok_or("manifest plan missing")?;
        exact_keys(
            manifest,
            &[
                "coverage_review",
                "execution_history",
                "name",
                "path",
                "records",
                "session_id",
                "sha256",
            ],
            "disjoint manifest plan",
        )?;
        if text(manifest, "/sha256")? != *want_sha
            || manifest["coverage_review"] != plan["coverage_review"]
            || manifest["execution_history"] != plan["execution_history"]
            || manifest["records"] != plan["records"]
            || manifest["session_id"] != plan["session_id"]
        {
            return Err("manifest plan differs from protocol lineage".into());
        }
        read_binding(
            &json!({"path":manifest["path"],"sha256":manifest["sha256"]}),
            "enrolled plan",
        )?;
        exact_keys(
            &plan["data_protocol"],
            &["file", "protocol_sha256"],
            "plan data protocol",
        )?;
        let (_, _, data_protocol) =
            read_json_binding(&plan["data_protocol"]["file"], "plan data protocol file")?;
        if data_protocol["schema"] != "experiment.protocol/v1" {
            return Err("plan data protocol schema mismatch".into());
        }
        let data_protocol_sha = text(&plan["data_protocol"], "/protocol_sha256")?;
        if data_protocol.pointer("/freeze/protocol_sha256") != Some(&json!(data_protocol_sha))
            || protocol_hash(&data_protocol)? != data_protocol_sha
        {
            return Err("plan data protocol canonical identity mismatch".into());
        }
        let prior = name == "gate-p739.json";
        if prior {
            if plan["origin"] != "prior_gate"
                || data_protocol_sha != PRIOR_GATE_PROTOCOL_SHA256
                || plan["data_protocol"]["file"] != prior_protocol_binding
                || data_protocol.pointer("/freeze/protocol_sha256")
                    != Some(&json!(PRIOR_GATE_PROTOCOL_SHA256))
                || plan["session_id"].as_str().is_none_or(str::is_empty)
                || plan["session_id"] != GATE_SESSION_ID
                || plan["records"]["sha256"] != GATE_RECORDS_SHA256
                || plan["records"].is_null()
                || plan["execution_history"].is_null()
            {
                return Err("prior gate lineage is incomplete or misclassified".into());
            }
            read_binding(&plan["records"], "gate records")?;
            read_binding(&plan["execution_history"], "gate execution history")?;
            gate_records_sha256 = Some(text(&plan["records"], "/sha256")?.to_owned());
            gate_session_id = Some(text(plan, "/session_id")?.to_owned());
        } else {
            if plan["origin"] != "prospective_continuation"
                || data_protocol_sha != continuation_sha
                || plan["data_protocol"]["file"] != lineage["continuation_protocol"]["file"]
            {
                return Err("prospective plan protocol/origin mismatch".into());
            }
            let absent = plan["session_id"].is_null()
                && plan["records"].is_null()
                && plan["execution_history"].is_null();
            let present = plan["session_id"].as_str().is_some_and(|s| !s.is_empty())
                && plan["records"].is_object()
                && plan["execution_history"].is_object();
            if !absent && !present {
                return Err(
                    "prospective session, records and history must be all null or all present"
                        .into(),
                );
            }
            if present {
                read_binding(&plan["records"], "prospective records")?;
                read_binding(&plan["execution_history"], "prospective execution history")?;
            }
        }
        if let Some(session) = plan["session_id"].as_str() {
            if !seen_sessions.insert(session) {
                return Err("session reused across disjoint plans".into());
            }
        }
        if plan["coverage_review"].is_null() || !plan["coverage_review"].is_object() {
            return Err("per-plan coverage review object required".into());
        }
        validated.push(json!({
            "name": name,
            "plan_sha256": want_sha,
            "origin": plan["origin"],
            "data_protocol": plan["data_protocol"],
            "data_protocol_sha256": data_protocol_sha,
            "coverage_review": plan["coverage_review"],
            "execution_history": plan["execution_history"],
            "session_id": plan["session_id"],
            "records": plan["records"],
        }));
    }
    let reuse = validate_reuse_admission(
        &known["scientific_reuse_admission"],
        &sha(&observation_receipt),
        gate_records_sha256
            .as_deref()
            .ok_or("gate records missing from lineage")?,
        gate_session_id
            .as_deref()
            .ok_or("gate session missing from lineage")?,
    )?;
    Ok(json!({
        "schema": "pencil-disjoint-protocol-lineage-validated/v1",
        "continuation_protocol": lineage["continuation_protocol"],
        "known_data_declaration": {
            "existing_gate_data_known_at_design": true,
            "gate_observation_receipt": known["gate_observation_receipt"],
            "scientific_reuse_admission": known["scientific_reuse_admission"],
            "validated_admission": reuse,
        },
        "plans": validated,
    }))
}

fn validate_with(
    manifest: &Value,
    run_verifier: impl FnOnce(&Path, &Path, &Path) -> Result<(), String>,
) -> Result<Value, String> {
    if manifest["analysis_scope"] != full1260_disjoint_p739_scope() {
        return Err("exact full1260 disjoint p739 scope required".into());
    }
    let enrollment = manifest
        .get("enrollment")
        .ok_or("disjoint enrollment block missing")?;
    exact_keys(
        enrollment,
        &[
            "certificate",
            "output_directory",
            "protocol_lineage",
            "request",
            "verifier",
        ],
        "disjoint enrollment block",
    )?;
    exact_keys(
        &enrollment["verifier"],
        &["binary", "source"],
        "native preparation verifier",
    )?;
    let (verifier_path, verifier_bytes) =
        read_binding(&enrollment["verifier"]["binary"], "verifier binary")?;
    if sha(&verifier_bytes) != VERIFIER_SHA256 {
        return Err("unreviewed native preparation verifier".into());
    }
    let (_, verifier_source) = read_binding(&enrollment["verifier"]["source"], "verifier source")?;
    if sha(&verifier_source) != VERIFIER_SOURCE_SHA256 {
        return Err("unreviewed native preparation verifier source".into());
    }
    let (request_path, request_bytes) = read_binding(&enrollment["request"], "request")?;
    if sha(&request_bytes) != REQUEST_SHA256 {
        return Err("unreviewed disjoint enrollment request".into());
    }
    let output_directory = PathBuf::from(text(enrollment, "/output_directory")?);
    run_verifier(&verifier_path, &request_path, &output_directory)?;
    let (certificate_path, certificate_bytes, certificate) =
        read_json_binding(&enrollment["certificate"], "enrollment certificate")?;
    if sha(&certificate_bytes) != CERTIFICATE_SHA256
        || certificate_path != output_directory.join("disjoint-enrollment.json")
        || certificate["schema"] != "pencil-disjoint-enrollment/v1"
        || certificate["passed"] != true
        || certificate["request_sha256"] != REQUEST_SHA256
        || certificate.pointer("/mapping/workloads") != Some(&json!(1260))
        || certificate.pointer("/mapping/slots") != Some(&json!(22680))
        || certificate.pointer("/claims/scientific_admission") != Some(&json!(false))
        || certificate.pointer("/claims/timing_qualification") != Some(&json!(false))
        || certificate.pointer("/claims/launch_authorization") != Some(&json!(false))
        || certificate.pointer("/claims/empirical_solver_execution") != Some(&json!(false))
    {
        return Err("reviewed enrollment certificate identity/content mismatch".into());
    }
    let receipt_path = output_directory.join("prepared-plan-validation.json");
    let receipt_bytes = fs::read(&receipt_path).map_err(|e| e.to_string())?;
    let receipt: Value = serde_json::from_slice(&receipt_bytes).map_err(|e| e.to_string())?;
    if receipt["schema"] != "pencil-native-preparation-validation/v1"
        || receipt["passed"] != true
        || receipt["solver_executions_performed"] != 0
        || receipt["plan_only_invocations"] != 1
        || receipt["request_sha256"] != REQUEST_SHA256
        || receipt["cohort_targets"] != 1260
        || receipt["cohort_planned_executions"] != 22680
        || receipt["cohort_measured_executions"] != 18900
        || receipt["cohort_warmup_executions"] != 3780
        || receipt["enrolled_plans"] != certificate["enrolled_plans"]
    {
        return Err("native preparation receipt mismatch".into());
    }
    let (_, _, lineage) = read_json_binding(&enrollment["protocol_lineage"], "protocol lineage")?;
    let validated_lineage = validate_lineage(
        &lineage,
        array(manifest, "/plans")?,
        &certificate,
        &enrollment["certificate"],
    )?;
    let continuation_sha = text(&validated_lineage, "/continuation_protocol/protocol_sha256")?;
    if text(manifest, "/protocol_sha256")? != continuation_sha {
        return Err("manifest continuation protocol identity mismatch".into());
    }
    Ok(json!({
        "schema": "pencil-disjoint-enrollment-validation/v1",
        "scope": full1260_disjoint_p739_scope(),
        "request": enrollment["request"],
        "certificate": enrollment["certificate"],
        "verifier": enrollment["verifier"],
        "prepared_plan_validation": {"path":receipt_path,"sha256":sha(&receipt_bytes)},
        "normalized_prepared_plan_validation": {
            "schema": "pencil-prepared-plan-validation/v1",
            "passed": true,
            "targets": 1260,
            "planned_executions": 22680,
            "measured_executions": 18900,
            "warmup_executions": 3780,
            "solver_executions_performed": 0,
            "plan_only_invocations": 1,
            "binary_sha256": receipt["binary_sha256"],
            "files": certificate["enrolled_plans"].as_array().ok_or("certificate plans missing")?.iter().map(|p| {
                (p["name"].as_str().unwrap().to_owned(), p["sha256"].clone())
            }).collect::<serde_json::Map<String, Value>>(),
        },
        "certificate_mapping": certificate["mapping"],
        "lineage": validated_lineage,
        "claims": {
            "scientific_admission": false,
            "timing_qualification": false,
            "launch_authorization": false,
            "empirical_solver_execution": false,
        },
    }))
}

/// Recompute the reviewed enrollment and validate the per-plan protocol/data lineage.
/// This function never generates a plan or executes a solver.
pub fn validate_disjoint_manifest(manifest: &Value) -> Result<Value, String> {
    validate_with(manifest, invoke_verifier)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp() -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "pencil-disjoint-price-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&path).unwrap();
        path
    }
    fn binding(path: &Path) -> Value {
        let bytes = fs::read(path).unwrap();
        json!({"path":path,"sha256":sha(&bytes)})
    }

    #[test]
    fn scope_pins_request_certificate_and_separate_verifier_source() {
        let scope = full1260_disjoint_p739_scope();
        assert_eq!(scope["expected_workloads"], 1260);
        assert_eq!(scope["expected_plans"], 8);
        assert_eq!(scope["enrollment_request_sha256"], REQUEST_SHA256);
        assert_eq!(scope["enrollment_certificate_sha256"], CERTIFICATE_SHA256);
        assert_eq!(scope["native_preparation_verifier_sha256"], VERIFIER_SHA256);
        assert_eq!(
            scope["native_preparation_verifier_source_sha256"],
            VERIFIER_SOURCE_SHA256
        );
    }

    #[test]
    fn reuse_admission_binds_records_session_receipt_and_prior_protocol() {
        let dir = temp();
        let admission_path = dir.join("admission.json");
        let receipt_hash = "1".repeat(64);
        let review_path = dir.join("review.json");
        let review = json!({
            "schema":"pencil-disjoint-reuse-review/v1",
            "status":"reviewed",
            "decision":"admit_exact_prior_p739_gate_records_only",
            "gate_plan_sha256":GATE_PLAN_SHA256,
            "gate_data_protocol_sha256":PRIOR_GATE_PROTOCOL_SHA256,
            "gate_observation_receipt_sha256":receipt_hash,
            "gate_records_sha256":GATE_RECORDS_SHA256,
            "gate_session_id":GATE_SESSION_ID,
            "producer":"software-fixture-producer",
            "reviewer":"independent-software-reviewer",
            "findings":[]
        });
        fs::write(&review_path, serde_json::to_vec(&review).unwrap()).unwrap();
        let admission = json!({
            "schema":"pencil-disjoint-scientific-reuse-admission/v1",
            "admitted":true,
            "scope":"exact_prior_p739_gate_records_only",
            "gate_plan_sha256":GATE_PLAN_SHA256,
            "gate_data_protocol_sha256":PRIOR_GATE_PROTOCOL_SHA256,
            "gate_observation_receipt_sha256":receipt_hash,
            "gate_records_sha256":GATE_RECORDS_SHA256,
            "gate_session_id":GATE_SESSION_ID,
            "reviewer":"independent-software-reviewer",
            "independent_review":binding(&review_path)
        });
        fs::write(&admission_path, serde_json::to_vec(&admission).unwrap()).unwrap();
        let b = binding(&admission_path);
        assert!(validate_reuse_admission(
            &b,
            &"1".repeat(64),
            GATE_RECORDS_SHA256,
            GATE_SESSION_ID
        )
        .is_ok());
        assert!(
            validate_reuse_admission(&b, &"1".repeat(64), &"3".repeat(64), GATE_SESSION_ID)
                .is_err()
        );
        assert!(validate_reuse_admission(
            &b,
            &"1".repeat(64),
            GATE_RECORDS_SHA256,
            "other-session"
        )
        .is_err());
        let mut changed = admission;
        changed["gate_data_protocol_sha256"] = json!("4".repeat(64));
        fs::write(&admission_path, serde_json::to_vec(&changed).unwrap()).unwrap();
        let changed_binding = binding(&admission_path);
        assert!(validate_reuse_admission(
            &changed_binding,
            &"1".repeat(64),
            GATE_RECORDS_SHA256,
            GATE_SESSION_ID
        )
        .is_err());
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn binding_rejects_unknown_fields_and_changed_bytes() {
        let dir = temp();
        let path = dir.join("input.json");
        fs::write(&path, b"{}\n").unwrap();
        let mut b = binding(&path);
        b["extra"] = json!(true);
        assert!(read_binding(&b, "test binding").is_err());
        b.as_object_mut().unwrap().remove("extra");
        fs::write(&path, b"{\"changed\":true}\n").unwrap();
        assert!(read_binding(&b, "test binding").is_err());
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn canonical_protocol_identity_excludes_freeze_and_binds_body() {
        let mut protocol = json!({
            "schema":"experiment.protocol/v1",
            "analysis_plan":{"frozen_before_data":false},
            "freeze":{"protocol_sha256":null}
        });
        let digest = protocol_hash(&protocol).unwrap();
        protocol["freeze"]["protocol_sha256"] = json!(digest);
        assert_eq!(protocol_hash(&protocol).unwrap(), digest);
        protocol["analysis_plan"]["frozen_before_data"] = json!(true);
        assert_ne!(protocol_hash(&protocol).unwrap(), digest);
    }
}
