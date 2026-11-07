//! Compare frozen input identity and the observed prefix of the deterministic relation stream.
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};

fn rows(path: &str) -> BTreeMap<String, Value> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .skip(1)
        .map(|line| {
            let row: Value = serde_json::from_str(&line.unwrap()).unwrap();
            (row["workload_id"].as_str().unwrap().to_owned(), row)
        })
        .collect()
}
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    assert_eq!(
        args.len(),
        3,
        "audit-stream BASE.jsonl CANDIDATE.jsonl OUTPUT.json"
    );
    let baseline = rows(&args[0]);
    let candidate = rows(&args[1]);
    assert!(candidate.keys().all(|key| baseline.contains_key(key)));
    let mut records = Vec::new();
    for (workload, row) in &candidate {
        let control = &baseline[workload];
        for key in [
            "p",
            "r",
            "bits",
            "workload_id",
            "workload_sha256",
            "target_index",
            "algorithm_seed",
            "cap",
            "planted",
        ] {
            assert_eq!(row[key], control[key], "fixed input {key} {workload}");
        }
        assert_eq!(row["rho"]["recovered"], control["rho"]["recovered"]);
        assert_eq!(row["rho"]["counts"], control["rho"]["counts"]);
        assert_eq!(row["rho"]["native"], control["rho"]["native"]);
        let same_observed_prefix_counts = [
            "walks",
            "candidate_lambdas",
            "verified_relations",
            "rejected_candidates",
        ]
        .iter()
        .all(|&key| row[key] == control[key]);
        records.push(json!({
            "workload_id":workload,
            "same_input_and_rho_counters":true,
            "same_observed_prefix_counts":same_observed_prefix_counts,
            "control_walks":control["walks"],"candidate_walks":row["walks"],
            "control_candidate_lambdas":control["candidate_lambdas"],
            "candidate_candidate_lambdas":row["candidate_lambdas"],
            "control_verified_relations":control["verified_relations"],
            "candidate_verified_relations":row["verified_relations"],
            "control_solution_key":control["solution_key"],
            "candidate_solution_key":row["solution_key"]
        }));
    }
    let matching = records
        .iter()
        .filter(|record| record["same_observed_prefix_counts"] == true)
        .count();
    let result = json!({
        "schema":"pencil-markowitz-stream-audit/v1",
        "baseline_raw":args[0],"candidate_raw":args[1],
        "matched_frozen_workloads":records.len(),
        "matching_observed_prefix_counts":matching,
        "note":"Equal aggregate prefix counts support the unchanged collector path, but are not a bytewise relation trace. The shared source keeps walk and relation generation outside the solver branch.",
        "records":records
    });
    fs::write(&args[2], serde_json::to_vec_pretty(&result).unwrap()).unwrap();
}
