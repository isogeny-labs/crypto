//! Derive cohort status and the matched rho band from independently validated native records.
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};

const ROOT: &str = "research/pencil_sparse_20261022/results";
const CURVES: [u64; 9] = [739, 1151, 1187, 3371, 3527, 3931, 11731, 11887, 12143];

fn read_json(path: &str) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}
fn lines(path: &str) -> Vec<Value> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| serde_json::from_str(&line.unwrap()).unwrap())
        .collect()
}
fn main() {
    let validation = read_json(&format!("{ROOT}/validation.json"));
    assert_eq!(validation["status"], "pass");
    assert_eq!(validation["independent"], true);
    assert!(validation["missing_replays"].as_array().unwrap().is_empty());
    let mut solved = BTreeSet::new();
    let mut failed = Vec::new();
    let mut arms = Vec::new();
    for file in validation["files"].as_array().unwrap() {
        let path = file["path"].as_str().unwrap();
        if !path.starts_with(ROOT) {
            continue; // historical controls are separately preserved
        }
        let rows = lines(path);
        assert_eq!(rows.len() - 1, file["targets"].as_u64().unwrap() as usize);
        let mut verified = 0u64;
        let mut failures = 0u64;
        for row in rows.iter().skip(1) {
            assert_eq!(row["rho"]["verified"], true);
            if row["verified"] == true {
                assert!(row["recovered"].is_u64());
                solved.insert(row["workload_id"].as_str().unwrap().to_owned());
                verified += 1;
            } else {
                assert!(row["recovered"].is_null());
                failed.push(json!({
                    "raw_path":path.strip_prefix("research/pencil_sparse_20261022/").unwrap(),
                    "workload_id":row["workload_id"],"failure":row["failure"],
                    "candidate_ratio":Value::Null
                }));
                failures += 1;
            }
        }
        assert_eq!(verified, file["verified_candidate_rows"].as_u64().unwrap());
        assert_eq!(failures, file["failed_candidate_rows"].as_u64().unwrap());
        arms.push(json!({
            "raw_path":path.strip_prefix("research/pencil_sparse_20261022/").unwrap(),
            "bits":file["bits"],"p":file["p"],"target_rows":rows.len()-1,
            "verified":verified,"failed":failures
        }));
    }
    arms.sort_by_key(|arm| arm["raw_path"].as_str().unwrap().to_owned());
    failed.sort_by_key(|row| row["raw_path"].as_str().unwrap().to_owned());
    let mut processes = Vec::new();
    for entry in fs::read_dir(ROOT).unwrap() {
        let path = entry.unwrap().path();
        if path.to_string_lossy().ends_with(".status.json") {
            processes.push(read_json(path.to_str().unwrap()));
        }
    }
    processes.sort_by_key(|record| record["p"].as_u64().unwrap());
    let status = json!({
        "schema":"pencil-sparse-cohort-status/v1",
        "original_cohort_targets":1260,
        "source_verified_distinct_targets":solved.len(),
        "distinct_targets_still_open":1260-solved.len(),
        "overall_status":if solved.len()==1260 {"complete"} else {"partial"},
        "validation_status":validation["status"],
        "pending_replay_paths":validation["missing_replays"],
        "new_study_arms":arms,"failed_target_records":failed,
        "process_outcomes":processes,
        "note":"A repeated arm on an existing workload is not a new distinct target; a stopped run has no candidate cost ratio."
    });
    fs::write(
        format!("{ROOT}/status.json"),
        serde_json::to_vec_pretty(&status).unwrap(),
    )
    .unwrap();

    let symmetry = read_json(&format!("{ROOT}/rho-symmetry-check.json"));
    assert_eq!(symmetry["checks"].as_array().unwrap().len(), 15);
    assert!(symmetry["checks"].as_array().unwrap().iter().all(|check| {
        check["frobenius_fixes_j"] == false && check["extra_geometric_automorphism"] == false
    }));
    let mut by_curve = Map::new();
    for p in CURVES {
        let pair = read_json(&format!("{ROOT}/p{p}-peel-vs-online.json"));
        assert_eq!(pair["targets"], 60);
        assert_eq!(pair["pricing_cells"], 36);
        assert_eq!(pair["cells_candidate_below_rho"], 0);
        let csv = fs::read_to_string(format!("{ROOT}/p{p}-peel-vs-online.csv")).unwrap();
        let mut bands = BTreeMap::<String, Value>::new();
        assert_eq!(csv.lines().skip(1).count(), 36);
        for line in csv.lines().skip(1) {
            let cells: Vec<_> = line.split(',').collect();
            assert_eq!(cells.len(), 11);
            if cells[2] == "0.5" && cells[3] == "10" {
                let native = cells[4];
                assert!(["0", "0.1", "1"].contains(&native));
                let ic: f64 = cells[6].parse().unwrap();
                let rho: f64 = cells[7].parse().unwrap();
                assert!(ic > rho);
                assert!(bands
                    .insert(native.to_owned(), json!({"ic":ic,"rho":rho}))
                    .is_none());
            }
        }
        assert_eq!(bands.len(), 3);
        by_curve.insert(p.to_string(), json!(bands));
    }
    let band = json!({
        "kind":"negation",
        "protocol":"same-process source-negation-rho on each frozen target; PROTOCOL.md",
        "unit":"GAE",
        "binary_or_process":"pencil-sparse process runs candidate and source rho per target",
        "matches_protocol":true,"strongest_available":true,
        "same_unit":true,"same_binary_or_process":true,
        "gae_band":by_curve["739"],
        "gae_band_scope":"p=739, 60 targets, S/M=0.5, I/M=10; native paired CSV",
        "gae_band_by_curve":by_curve,
        "order_holds_across_band":true,
        "ordering":"candidate GAE > matched rho GAE on each of the nine curves at all three native prices"
    });
    fs::write(
        format!("{ROOT}/rho-band.json"),
        serde_json::to_vec_pretty(&band).unwrap(),
    )
    .unwrap();
    println!(
        "native summary: {} distinct solves; {} curves with rho bands",
        solved.len(),
        CURVES.len()
    );
}
