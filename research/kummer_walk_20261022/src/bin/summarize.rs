use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;

fn rows(path: &str) -> Vec<Value> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

fn main() {
    let dir = "research/kummer_walk_20261022/results";
    let mut targets = 0u64;
    let mut target_cases = 0u64;
    let mut models = 0u64;
    let mut exception_cases = 0u64;
    let mut target_failures = 0u64;
    let mut exception_failures = 0u64;
    let mut compatible = BTreeMap::<u64, u64>::new();
    let mut per_bits = Vec::new();
    let mut workload_ids = BTreeSet::new();
    let mut model_ids = BTreeSet::new();
    for bits in [16, 20, 24, 28, 32, 36, 40] {
        let target_rows = rows(&format!("{dir}/target-gate-{bits}.jsonl"));
        let model_rows = rows(&format!("{dir}/model-exceptions-{bits}.jsonl"));
        assert_eq!(target_rows.len(), 180);
        assert_eq!(model_rows.len(), 3);
        let mut passed = 0;
        for row in target_rows {
            assert_eq!(row["schema"], "kummer-differential-target/v1");
            assert_eq!(row["bits"], bits);
            assert_eq!(row["case_count"], 7);
            let id = row["workload_id"].as_str().unwrap().to_owned();
            assert!(workload_ids.insert(id));
            let cases = row["cases"].as_array().unwrap();
            assert_eq!(cases.len(), 7);
            let mut cases_correct = true;
            for case in cases {
                let result = &case["result"];
                let n = result["compatible_lifts"].as_u64().unwrap();
                *compatible.entry(n).or_default() += 1;
                cases_correct &= result["differential_correct"] == true
                    && result["signed_recovery_correct"] == true
                    && result["label_transfer_correct"] == true
                    && result["differential_output_count"] == 1
                    && result["signed_lift_count"] == 1;
            }
            let recomputed = row["source_scalar_verified"] == true && cases_correct;
            assert_eq!(row["all_correct"], recomputed);
            passed += u64::from(recomputed);
            targets += 1;
            target_cases += cases.len() as u64;
            target_failures += u64::from(!recomputed);
        }
        for row in model_rows {
            assert_eq!(row["schema"], "kummer-differential-model/v1");
            assert_eq!(row["bits"], bits);
            assert_eq!(row["root_count"], 5);
            assert_eq!(row["exception_cases"], 30);
            let p = row["p"].as_u64().unwrap();
            assert!(model_ids.insert(p));
            let cases = row["cases"].as_array().unwrap();
            assert_eq!(cases.len(), 30);
            let recomputed = cases.iter().all(|case| {
                let result = &case["result"];
                result["differential_correct"] == true
                    && result["signed_recovery_correct"] == true
                    && result["differential_output_count"] == 1
                    && result["signed_lift_count"] == 1
            });
            assert_eq!(row["all_correct"], recomputed);
            models += 1;
            exception_cases += cases.len() as u64;
            exception_failures += u64::from(!recomputed);
        }
        per_bits.push(
            json!({"bits":bits,"targets":180,"target_cases":1260,"passed_targets":passed,
            "models":3,"exception_cases":90}),
        );
    }
    assert_eq!(workload_ids.len(), 1260);
    assert_eq!(model_ids.len(), 21);
    assert_eq!(targets, 1260);
    assert_eq!(target_cases, 8820);
    assert_eq!(exception_cases, 630);
    let summary = json!({"schema":"kummer-differential-summary/v1",
        "status":"reference_invariant_gate_only",
        "targets":targets,"target_cases":target_cases,"models":models,
        "exception_cases":exception_cases,"target_failures":target_failures,
        "exception_failures":exception_failures,"compatible_lifts_histogram":compatible,
        "per_bits":per_bits,"full_collector_targets":0,
        "fast_coordinate_formula_verified":false,
        "candidate_over_rho":Value::Null});
    fs::write(
        format!("{dir}/summary.json"),
        serde_json::to_string_pretty(&summary).unwrap() + "\n",
    )
    .unwrap();
    let mut grid = String::from("arm,slice,targets,verified,failures,S_per_M,I_per_M,native_GAE_per_logical,candidate_GAE,rho_GAE,candidate_over_rho,status\n");
    for slice in ["all", "16", "20", "24", "28", "32", "36", "40"] {
        for s in ["0", "0.5", "1"] {
            for i in ["1", "10", "100", "1000"] {
                for native in ["0", "0.1", "1"] {
                    grid.push_str(&format!(
                        "kummer-differential,{slice},0,0,0,{s},{i},{native},,,,not_measured\n"
                    ));
                }
            }
        }
    }
    fs::write(format!("{dir}/price-grid.csv"), grid).unwrap();
    println!("{}", serde_json::to_string(&summary).unwrap());
}
