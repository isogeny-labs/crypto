//! Solver-free integration against the reviewed preparation packet.
//! Run explicitly with the four `PENCIL_DISJOINT_*` paths; the default suite
//! remains self-contained and does not rely on `/tmp` or a prior build.
use pencil_accounting_price_draft::{aggregate, disjoint};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn protocol_hash(value: &Value) -> String {
    let mut body = value.clone();
    body.as_object_mut().unwrap().remove("freeze");
    sha(&serde_json::to_vec(&body).unwrap())
}
fn binding(path: &Path) -> Value {
    let bytes = fs::read(path).unwrap();
    json!({"path":path,"sha256":sha(&bytes)})
}
fn write(path: &Path, value: &Value) -> Value {
    fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
    binding(path)
}

#[test]
#[ignore = "requires hydrated reviewed enrollment packet; invokes verifier only, no plan or solver"]
fn reviewed_packet_recomputes_and_preserves_all_missing_future_slots() {
    let request = std::path::PathBuf::from(std::env::var("PENCIL_DISJOINT_REQUEST").unwrap());
    let output = std::path::PathBuf::from(std::env::var("PENCIL_DISJOINT_OUTPUT").unwrap());
    let verifier = std::path::PathBuf::from(std::env::var("PENCIL_DISJOINT_VERIFIER").unwrap());
    let verifier_source =
        std::path::PathBuf::from(std::env::var("PENCIL_DISJOINT_VERIFIER_SOURCE").unwrap());
    let certificate_path = output.join("disjoint-enrollment.json");
    let certificate: Value = serde_json::from_slice(&fs::read(&certificate_path).unwrap()).unwrap();
    let dir = std::env::temp_dir().join(format!(
        "pencil-disjoint-actual-packet-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir(&dir).unwrap();

    let prior_protocol_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../protocol_draft/first_gate_p739_infrastructure_retry_20261030/protocol.json")
        .canonicalize()
        .unwrap();
    let records_path =
        Path::new("/home/ec2-user/pencil-actual-results-20261030-a1/fresh/session/records.jsonl");
    let history_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../validation/canonical_gate_export_20261030/execution-history.json")
        .canonicalize()
        .unwrap();
    let observation_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../validation/canonical_gate_export_20261030/receipt.json")
        .canonicalize()
        .unwrap();
    assert_eq!(
        binding(records_path)["sha256"],
        disjoint::GATE_RECORDS_SHA256
    );
    let history_binding = binding(&history_path);
    let observation_binding = binding(&observation_path);
    let review_path = dir.join("reuse-independent-review.json");
    let review_binding = write(
        &review_path,
        &json!({
            "schema":"pencil-disjoint-reuse-review/v1",
            "status":"reviewed",
            "decision":"admit_exact_prior_p739_gate_records_only",
            "gate_plan_sha256":disjoint::GATE_PLAN_SHA256,
            "gate_data_protocol_sha256":disjoint::PRIOR_GATE_PROTOCOL_SHA256,
            "gate_observation_receipt_sha256":observation_binding["sha256"],
            "gate_records_sha256":disjoint::GATE_RECORDS_SHA256,
            "gate_session_id":disjoint::GATE_SESSION_ID,
            "producer":"software-integration-fixture-producer",
            "reviewer":"software-integration-fixture-reviewer",
            "findings":[]
        }),
    );
    let admission_path = dir.join("reuse-admission.json");
    let admission_binding = write(
        &admission_path,
        &json!({
            "schema":"pencil-disjoint-scientific-reuse-admission/v1",
            "admitted":true,
            "scope":"exact_prior_p739_gate_records_only",
            "gate_plan_sha256":disjoint::GATE_PLAN_SHA256,
            "gate_data_protocol_sha256":disjoint::PRIOR_GATE_PROTOCOL_SHA256,
            "gate_observation_receipt_sha256":observation_binding["sha256"],
            "gate_records_sha256":disjoint::GATE_RECORDS_SHA256,
            "gate_session_id":disjoint::GATE_SESSION_ID,
            "reviewer":"software-integration-fixture-reviewer",
            "independent_review":review_binding
        }),
    );
    let certificate_binding = binding(&certificate_path);
    let continuation_protocol_path = dir.join("continuation-protocol.json");
    let known_amendment = "A-known-prior-p739-software-fixture";
    let mut continuation_protocol = json!({
        "schema":"experiment.protocol/v1",
        "analysis_plan":{"frozen_before_data":false},
        "sample_size":{"instances":1260},
        "extensions":{
            "analysis_scope":disjoint::full1260_disjoint_p739_scope(),
            "disjoint_enrollment":{
                "known_prior_gate_data":true,
                "freeze_scope":"prospective_continuation_only",
                "known_data_amendment_id":known_amendment,
                "certificate":certificate_binding,
                "gate_observation_receipt":observation_binding,
                "scientific_reuse_admission":admission_binding
            }
        },
        "amendments":[{"id":known_amendment,"filed_before_affected_data":false}],
        "status":"software-only prospective lineage fixture; no launch authorization",
        "freeze":{"status":"frozen","protocol_sha256":null}
    });
    let continuation_identity = protocol_hash(&continuation_protocol);
    continuation_protocol["freeze"]["protocol_sha256"] = json!(continuation_identity);
    let continuation_binding = write(&continuation_protocol_path, &continuation_protocol);

    let mut plans = vec![];
    let mut lineage_plans = vec![];
    for enrolled in certificate["enrolled_plans"].as_array().unwrap() {
        let name = enrolled["name"].as_str().unwrap();
        let prior = name == "gate-p739.json";
        let path = if name == "cohort-16-remainder-plan.json" {
            output.join(name)
        } else {
            std::path::PathBuf::from(enrolled["path"].as_str().unwrap())
        };
        let (session, records, history, origin, protocol_file, protocol_identity) = if prior {
            (
                json!(disjoint::GATE_SESSION_ID),
                binding(records_path),
                history_binding.clone(),
                json!("prior_gate"),
                binding(&prior_protocol_path),
                json!(disjoint::PRIOR_GATE_PROTOCOL_SHA256),
            )
        } else {
            (
                Value::Null,
                Value::Null,
                Value::Null,
                json!("prospective_continuation"),
                continuation_binding.clone(),
                json!(continuation_identity),
            )
        };
        let coverage = json!({"software_fixture":true,"qualified":false});
        plans.push(json!({
            "name":name,
            "path":path,
            "sha256":enrolled["sha256"],
            "session_id":session,
            "records":records,
            "execution_history":history,
            "coverage_review":coverage,
        }));
        lineage_plans.push(json!({
            "name":name,
            "plan_sha256":enrolled["sha256"],
            "origin":origin,
            "data_protocol":{"file":protocol_file,"protocol_sha256":protocol_identity},
            "coverage_review":coverage,
            "execution_history":history,
            "session_id":session,
            "records":records,
        }));
    }
    let lineage_path = dir.join("lineage.json");
    let lineage_binding = write(
        &lineage_path,
        &json!({
            "schema":"pencil-disjoint-protocol-lineage/v1",
            "continuation_protocol":{"file":continuation_binding,"protocol_sha256":continuation_identity},
            "known_data_declaration":{
                "existing_gate_data_known_at_design":true,
                "gate_observation_receipt":observation_binding,
                "scientific_reuse_admission":admission_binding,
            },
            "plans":lineage_plans,
        }),
    );
    let manifest = json!({
        "schema":"pencil-price-cohort-input/v1",
        "analysis_scope":disjoint::full1260_disjoint_p739_scope(),
        "enrollment":{
            "verifier":{"binary":binding(&verifier),"source":binding(&verifier_source)},
            "request":binding(&request),
            "certificate":binding(&certificate_path),
            "output_directory":output,
            "protocol_lineage":lineage_binding,
        },
        "prepared_plan_validation":null,
        "binary_sha256":certificate["instrument"]["binary"]["sha256"],
        "protocol_sha256":continuation_identity,
        "coverage_review":null,
        "separate_resources":null,
        "plans":plans,
    });
    let lineage = aggregate::validate_disjoint_manifest(&manifest).unwrap();
    assert_eq!(lineage["plans"].as_array().unwrap().len(), 8);
    let normalized = aggregate::normalized_disjoint_receipt(&manifest).unwrap();
    assert_eq!(normalized["targets"], 1260);
    assert_eq!(normalized["files"].as_object().unwrap().len(), 8);
    let output_value = aggregate::aggregate_disjoint_p739(&manifest).unwrap();
    assert_eq!(output_value["expected_workloads"], 1260);
    assert_eq!(output_value["expected_measured_pairs"], 6300);
    assert_eq!(output_value["expected_executions"], 22680);
    assert_eq!(output_value["observed_executions"], 1080);
    assert_eq!(
        output_value["missing_executions"].as_array().unwrap().len(),
        21600
    );
    assert!(output_value["aggregate_cells"]
        .as_array()
        .unwrap()
        .iter()
        .all(|cell| cell["algorithm_ratio_of_sums"].is_null()));
    assert!(output_value["missing_executions"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|row| row["plan"] != "gate-p739.json")
        .all(|row| row["session_id"].is_null() && row["expected"]["session"].is_null()));

    let mut mixed = manifest.clone();
    mixed["plans"][1]["session_id"] = json!("fabricated-future-session");
    assert!(aggregate::validate_disjoint_manifest(&mixed).is_err());
    let mut changed_plan = manifest.clone();
    changed_plan["plans"][1]["sha256"] = json!("4".repeat(64));
    assert!(aggregate::validate_disjoint_manifest(&changed_plan).is_err());

    // The same contract accepts future observations only when the complete
    // session/records/history triple is supplied and hash-bound.
    let future_records = dir.join("future-records.jsonl");
    fs::write(&future_records, b"").unwrap();
    let future_history = dir.join("future-history.json");
    let future_history_binding = write(
        &future_history,
        &json!({"schema":"software-only-future-history-binding/v1","records":[]}),
    );
    let mut observed = manifest.clone();
    let mut observed_lineage: Value =
        serde_json::from_slice(&fs::read(&lineage_path).unwrap()).unwrap();
    for value in [&mut observed["plans"][1], &mut observed_lineage["plans"][1]] {
        value["session_id"] = json!("software-future-session");
        value["records"] = binding(&future_records);
        value["execution_history"] = future_history_binding.clone();
    }
    let observed_lineage_path = dir.join("lineage-with-future-observations.json");
    observed["enrollment"]["protocol_lineage"] = write(&observed_lineage_path, &observed_lineage);
    assert!(aggregate::validate_disjoint_manifest(&observed).is_ok());
    let mut duplicate_session = observed.clone();
    let mut duplicate_lineage: Value =
        serde_json::from_slice(&fs::read(&observed_lineage_path).unwrap()).unwrap();
    for value in [
        &mut duplicate_session["plans"][2],
        &mut duplicate_lineage["plans"][2],
    ] {
        value["session_id"] = json!("software-future-session");
        value["records"] = binding(&future_records);
        value["execution_history"] = future_history_binding.clone();
    }
    let duplicate_lineage_path = dir.join("lineage-duplicate-session.json");
    duplicate_session["enrollment"]["protocol_lineage"] =
        write(&duplicate_lineage_path, &duplicate_lineage);
    assert!(aggregate::validate_disjoint_manifest(&duplicate_session).is_err());
    fs::remove_dir_all(dir).unwrap();
}
