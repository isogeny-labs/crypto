//! Native record/provenance cross-check. No collector, BSGS or timing claim.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::path::Path;

const PUBLIC: [&str; 13] = [
    "p",
    "seed",
    "n",
    "b0",
    "generator_x",
    "generator_y",
    "target_x",
    "target_y",
    "omega",
    "lambda_cubic",
    "lambda_frob",
    "base_coordinates",
    "query_trace",
];
fn read(path: &Path) -> (String, Value) {
    let bytes = std::fs::read(path).unwrap();
    (
        hex::encode(Sha256::digest(&bytes)),
        serde_json::from_slice(&bytes).unwrap(),
    )
}
fn public_projection(raw: &Value) -> Value {
    let mut projected: serde_json::Map<String, Value> = PUBLIC
        .iter()
        .map(|key| ((*key).into(), raw[*key].clone()))
        .collect();
    // The frozen exporter excludes per-query oracle hit counts. It keeps
    // every query in order, with exactly the two public coefficients.
    projected.insert(
        "query_trace".into(),
        json!(raw["query_trace"]
            .as_array()
            .unwrap()
            .iter()
            .map(|q| vec![q[0].clone(), q[1].clone()])
            .collect::<Vec<_>>()),
    );
    Value::Object(projected)
}
fn compatible(raw: &Value, public: &Value, old: &Value, solution: &Value, audit: &Value) -> bool {
    public_projection(raw) == *public
        && public.as_object().unwrap().len() == 13
        && ["p", "seed", "n", "b0"]
            .iter()
            .all(|key| old[*key] == public[*key])
        && old["target_scalar"] == raw["known_scalar"]
        && old["rows"][0]["a"] == public["query_trace"][2][0]
        && old["rows"][0]["b"] == public["query_trace"][2][1]
        && solution["p"] == public["p"]
        && solution["seed"] == public["seed"]
        && solution["query_index"] == 2
        && solution["arm"] == "l0-peel"
        && solution["status"] == "complete"
        && solution["roots_complete"] == true
        && audit["status"] == "pass"
        && audit["candidate_roots_complete"] == true
        && ["p", "seed", "query_index", "arm", "rank", "columns"]
            .iter()
            .all(|key| audit[*key] == solution[*key])
        && audit["reference_sha256"]
            == "3a14f14c525661a45ddf2ba5db2c016f9ff2647e23836a567e1541fda4389d85"
        && audit["missing"] == json!([])
        && audit["extra"] == json!([])
        && old["rows"][0]["reference_signed_relations"] == solution["signed_relations"]
        && audit["expected_signed_relations"] == audit["actual_signed_relations"]
        && audit["actual_signed_relations"]
            == json!(solution["signed_relations"].as_array().unwrap().len())
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 3);
    let root = Path::new(&args[1]);
    let out = Path::new(&args[2]);
    assert!(!out.exists());
    let study = root.join("research/twist_index_calculus_20261020");
    let joint = study.join("joint_image_elimination");
    let tape = study.join("quartic_twist_mixed_v1.ndjson");
    let tape_bytes = std::fs::read(&tape).unwrap();
    let tape_sha = hex::encode(Sha256::digest(&tape_bytes));
    assert_eq!(
        tape_sha,
        "d50c500961344a1d5813a02dbba9019aef3309cb74180e99715ca693f5b8094c"
    );
    let raw = std::str::from_utf8(&tape_bytes)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .find(|v| v["p"] == 37 && v["seed"] == 0 && v["line_powers"] == json!([0, 2]))
        .unwrap();
    let (public_sha, public) = read(&joint.join("runs/algebra-v1/prepared/public/p37-seed0.json"));
    assert_eq!(
        public_sha,
        "5a55715c3d8f532634722ff6f58562e8f7e9f7d339f529cda724677da0f9b393"
    );
    let (old_sha, old) = read(&study.join("quartic_twist_tagged_s5_k0_q2_d30_p37_v1.json"));
    let (solution_sha, solution) =
        read(&joint.join("runs/regression-v2/solves/l0-peel-orbit-union/summary.json"));
    let (audit_sha, audit) =
        read(&joint.join("results/source-block-regression/l0-peel-audit.json"));
    assert_eq!(old["status"], "partial");
    assert_eq!(old["k"], 0);
    assert_eq!(old["rows"][0]["polynomial_signed_relations"], json!([]));
    assert_eq!(solution["public_input_sha256"], public_sha);
    std::fs::create_dir_all(out).unwrap();
    let diagnostics = json!({"public_parameters":PUBLIC.iter().filter(|key| **key != "query_trace")
        .map(|key|((*key).to_string(),json!(raw[*key]==public[*key]))).collect::<serde_json::Map<String,Value>>(),
        "whole_public_projection_identical":public_projection(&raw)==public,
        "historical_query_a":old["rows"][0]["a"]==public["query_trace"][2][0],
        "historical_query_b":old["rows"][0]["b"]==public["query_trace"][2][1],
        "physical_set_identical":old["rows"][0]["reference_signed_relations"]==solution["signed_relations"],
        "audit_matches_solution_fields":["p","seed","query_index","arm","rank","columns"].iter()
            .map(|key|((*key).to_string(),json!(audit[*key]==solution[*key]))).collect::<serde_json::Map<String,Value>>()});
    std::fs::write(
        out.join("diagnostics.json"),
        serde_json::to_vec_pretty(&diagnostics).unwrap(),
    )
    .unwrap();
    assert!(compatible(&raw, &public, &old, &solution, &audit));
    let mut bad = public.clone();
    bad["target_x"][0] = json!(public["target_x"][0].as_u64().unwrap() + 1);
    assert!(!compatible(&raw, &bad, &old, &solution, &audit));
    let mut bad = old.clone();
    bad["rows"][0]["a"] = json!(old["rows"][0]["a"].as_u64().unwrap() + 1);
    assert!(!compatible(&raw, &public, &bad, &solution, &audit));
    let mut bad = solution.clone();
    bad["signed_relations"] = json!([]);
    assert!(!compatible(&raw, &public, &old, &bad, &audit));
    let mut bad = public.clone();
    bad["query_trace"].as_array_mut().unwrap().pop();
    assert!(!compatible(&raw, &bad, &old, &solution, &audit));
    let result = json!({"schema":"historical-s5-fixture-crosscheck/v1","status":"source-and-record-match",
        "p":37,"seed":0,"query_index":2,"k":0,"tape_sha256":tape_sha,"public_sha256":public_sha,
        "historical_report_sha256":old_sha,"solution_sha256":solution_sha,"independent_audit_sha256":audit_sha,
        "all_public_fields_match_frozen_export_projection":true,"query_identical":true,
        "projection":"All 13 public keys; raw query oracle-count column removed, every a/b pair and its order retained",
        "signed_physical_tuple_identical":true,"new_roots_complete":true,"independent_signed_set_audit_pass":true,
        "changed_coordinate_rejected":true,"changed_query_rejected":true,"omitted_tuple_rejected":true,"truncated_trace_rejected":true,
        "scope":"Source-and-record binding of the one historical p37/seed0/query2 pure S5 miss to the completed source-block solution. The historical report itself does not carry a full runtime input SHA; no retroactive runtime seal, whole-61-query completion, generic ideal proof or cost improvement is claimed. Field representation compatibility is a separate source audit of the two first-nonsquare quartic constructors."});
    std::fs::write(
        out.join("summary.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
}
