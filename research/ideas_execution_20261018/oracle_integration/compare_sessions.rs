//! Exact correctness/count comparison; excludes runtime and source labels.
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;

fn rows(path: &str) -> BTreeMap<String, Value> {
    fs::read_to_string(format!("{path}/records.jsonl")).unwrap().lines().map(|line|{
        let row:Value=serde_json::from_str(line).unwrap();
        let w=&row["workload"];
        (row["run_id"].as_str().unwrap().into(),json!({
            "arm":row["arm"],"method":row["method"],"algorithm_seed":row["algorithm_seed"],
            "curve_spec":w["curve_spec"],"r":w["curve"]["r"],"generator":w["curve"]["generator"],
            "target":w["target"],"planted":w["planted"],"outcome_status":row["outcome"]["status"],
            "recovered":row["outcome"]["recovered"],"matches_target":row["outcome"]["matches_target"],
            "matches_planted":row["outcome"]["matches_planted"],"cost":row["cost"],
            "counters":row["counters"],"detail":row["detail"]}))
    }).collect()
}

fn main() {
    let a: Vec<_> = std::env::args().collect();
    let left = rows(&a[1]);
    let right = rows(&a[2]);
    let missing: Vec<_> = left
        .keys()
        .filter(|k| !right.contains_key(*k))
        .cloned()
        .collect();
    let extra: Vec<_> = right
        .keys()
        .filter(|k| !left.contains_key(*k))
        .cloned()
        .collect();
    let mismatch: Vec<_> = left
        .iter()
        .filter(|(k, v)| right.get(*k).is_some_and(|r| r != *v))
        .map(|(k, _)| k.clone())
        .collect();
    let passed = left.len() == 224
        && right.len() == 224
        && missing.is_empty()
        && extra.is_empty()
        && mismatch.is_empty();
    let result = json!({"left":a[1],"right":a[2],"left_records":left.len(),"right_records":right.len(),
        "matching_records":left.len()-missing.len()-mismatch.len(),"missing":missing,"extra":extra,
        "mismatches":mismatch,"passed":passed,"scope":"Exact deterministic target, outcome, cost/counter/detail payloads; timings/source labels excluded. Includes72 exhausted runs."});
    let mut output = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&a[3])
        .unwrap();
    output
        .write_all(&serde_json::to_vec_pretty(&result).unwrap())
        .unwrap();
    if !passed {
        std::process::exit(1);
    }
}
