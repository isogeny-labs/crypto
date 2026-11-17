//! Post-run grid/decision auditor; not part of the frozen analysis instrument.
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
};
fn main() {
    let a: Vec<_> = env::args().skip(1).collect();
    let rows: Vec<Value> = fs::read_to_string(&a[0])
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let summary: Value = serde_json::from_slice(&fs::read(&a[1]).unwrap()).unwrap();
    let mut seen = BTreeSet::new();
    let mut references = BTreeMap::<String, Value>::new();
    let mut calls = 0;
    let cases: BTreeSet<_> = [83, 131]
        .iter()
        .flat_map(|n| {
            ["distinct-planted-0", "uniform", "cancellation"]
                .iter()
                .map(move |k| format!("scale-n{n}-d12-{k}"))
        })
        .collect();
    for r in &rows {
        let id = r["instance"].as_str().unwrap();
        let arm = r["arm"].as_str().unwrap();
        let rep = r["replicate"].as_u64().unwrap();
        assert!(cases.contains(id));
        assert!(["baseline", "baseline-aa", "candidate"].contains(&arm));
        assert!(rep < 5);
        assert!(seen.insert((id.to_string(), arm.to_string(), rep)));
        let z = &r["result"];
        assert_eq!(z["valid"], true);
        assert_ne!(z["outcome"], "Inconclusive");
        if id.ends_with("uniform") {
            assert_eq!(z["outcome"], "Refuted");
        } else {
            assert_eq!(z["outcome"], "Found");
            assert_eq!(z["replay"], true);
        }
        if let Some(old) = references.get(id) {
            for key in [
                "outcome",
                "indices",
                "points",
                "models",
                "patterns_attempted",
                "patterns_refuted",
                "reason",
                "diagnostics",
            ] {
                assert_eq!(old[key], z[key], "{id}/{arm}/{rep}/{key}");
            }
        } else {
            references.insert(id.to_string(), z.clone());
        }
        calls += 2;
    }
    assert_eq!(rows.len(), 90);
    assert_eq!(seen.len(), 90);
    assert_eq!(references.len(), 6);
    let mut verdicts = Vec::new();
    let mut passes = 0;
    for r in summary["comparisons"].as_array().unwrap() {
        let delta = r["instructions_removed_fraction"].as_f64().unwrap();
        let aa = r["aa_relative_median_difference"].as_f64().unwrap();
        let pass = delta >= 0.05 && aa <= 0.01;
        if pass {
            passes += 1;
        }
        verdicts.push(json!({"instance":r["instance"],"instruction_reduction_criterion":pass,"verdict":if pass{"descriptive_difference"}else{"inconclusive"},"removed_fraction":delta,"aa_relative_difference":aa}));
    }
    assert_eq!(verdicts.len(), 6);
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"trace-zero-fast-grid-audit/v1","planned":90,"attempted":90,"completed":90,"invalid":0,"unique_grid":true,"same_decision_and_exact_witness_order":true,"fresh_child_processes":calls,"passing_cells":passes,"cell_verdicts":verdicts,"standing":"producer-written post-run check; no independently signed verdict"})).unwrap());
}
