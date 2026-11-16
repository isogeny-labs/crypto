//! Local work/correctness gate; never admits local elapsed time.
#[path = "../f4_f5_boolean_runtime_20261019/analyze.rs"]
#[allow(dead_code)]
mod common;
use serde_json::{json, Value};
use std::{fs, path::Path};
fn read(path: &Path) -> Value {
    serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap()
}
fn records(path: &Path) -> Vec<Value> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect()
}
fn main() {
    let study = std::env::args().nth(1).expect("analyze-screen STUDY");
    let study = Path::new(&study);
    let label = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "adaptive-screen-retry1".into());
    let root = study.join("runs").join(label);
    assert_eq!(
        fs::read_to_string(root.join("screen-phase.status"))
            .unwrap()
            .trim(),
        "complete"
    );
    let mut rows = Vec::new();
    for n in [17, 23] {
        let audit = read(&root.join(format!("audit-n{n}.json")));
        assert_eq!(audit["ok"], true);
        let replays = audit["replays"].as_array().unwrap();
        assert_eq!(replays.len(), 4);
        assert!(replays.iter().all(|r| r["reproduced"] == true));
        let current = records(&root.join(format!("session-n{n}/records.jsonl")));
        assert_eq!(current.len(), 8);
        assert!(current.iter().all(|r| r["outcome"]["status"] == "verified"));
        let old = records(&study.join(format!(
            "runs/iteration1-ecbench/out/session-n{n}/records.jsonl"
        )));
        for engine in ["f4", "f5"] {
            let get = |name: String| {
                current
                    .iter()
                    .find(|r| r["arm"] == name && r["warmup"] == false)
                    .unwrap()
            };
            let baseline = get(format!("{engine}-baseline"));
            let candidate = get(format!("{engine}-tail"));
            let failed = old
                .iter()
                .find(|r| {
                    r["arm"] == format!("{engine}-tail")
                        && r["round"] == candidate["round"]
                        && r["workload"]["workload_id"] == candidate["workload"]["workload_id"]
                })
                .unwrap();
            for reference in [baseline, failed] {
                assert_eq!(candidate["outcome"], reference["outcome"]);
                assert_eq!(candidate["factor_base"], reference["factor_base"]);
                assert_eq!(candidate["algorithm_seed"], reference["algorithm_seed"]);
                for key in ["targets_tried", "relations_found", "matrix_rank"] {
                    assert_eq!(candidate["counters"][key], reference["counters"][key]);
                }
            }
            assert_eq!(candidate["solver"]["budget_exceeded"], 0);
            let ops = candidate["solver"]["ops"].as_u64().unwrap();
            let previous = failed["solver"]["ops"].as_u64().unwrap();
            assert!(ops < previous);
            rows.push(json!({"n":n,"engine":engine,"baseline_word_xors":baseline["solver"]["ops"],"failed_policy_word_xors":previous,"adaptive_word_xors":ops,"solver_calls":candidate["solver"]["calls"],"query_counts_and_answer_equal":true}));
        }
    }
    let mut wide = Vec::new();
    for n in [17, 23] {
        for arm in ["solve-f4", "solve-f5"] {
            let a = read(&root.join(format!("{arm}-n{n}-before.jsonl")));
            let b = read(&root.join(format!("{arm}-n{n}-after.jsonl")));
            assert_eq!(a["verified"], true);
            assert_eq!(b["verified"], true);
            assert_eq!(common::stable(&a), common::stable(&b));
            assert_eq!(a["stats"], b["stats"]);
            for key in [
                "calls",
                "rows",
                "cols",
                "word_ops",
                "rows_pruned",
                "criterion_word_ops",
                "specialise_word_ops",
            ] {
                assert_eq!(a["profile"][key], b["profile"][key]);
            }
            wide.push(json!({"n":n,"arm":arm,"original_roots_and_counters_equal":true}));
        }
    }
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"boolean-adaptive-screen/v1","gate_passed":true,"local_pipeline_records_verified":16,"local_warmup_records":8,"local_measured_replays_identical":8,"work_rows":rows,"wide_controls":wide,"native_timing_claim":false,"scope":"L0 work/correctness only; fresh full native and ecbench confirmation remains mandatory"})).unwrap());
}
