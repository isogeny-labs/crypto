//! Instruction-only auxiliary evidence. Profiled elapsed time is never native.
#[path = "analyze.rs"]
#[allow(dead_code)]
mod common;
use common::{median, stable};
use serde_json::{json, Value};
use std::{fs, path::Path};
fn profile(dir: &Path) -> (Value, u64, u64) {
    assert_eq!(
        fs::read_to_string(dir.join("exit.txt")).unwrap().trim(),
        "0"
    );
    let text = fs::read_to_string(dir.join("run.jsonl")).unwrap();
    let lines = text.lines().collect::<Vec<_>>();
    assert_eq!(lines.len(), 1);
    let row: Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(row["verified"], true);
    let (mut setup, mut query) = (None, None);
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if !path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .contains(".callgrind")
        {
            continue;
        }
        let data = fs::read_to_string(path).unwrap();
        let Some(label) = data
            .lines()
            .find_map(|l| l.strip_prefix("desc: Trigger: Client Request: "))
        else {
            continue;
        };
        assert!(
            !data.lines().any(|l| l.starts_with("thread:")),
            "merge all worker threads"
        );
        let label: Value = serde_json::from_str(label).unwrap();
        let events = data
            .lines()
            .find_map(|l| l.strip_prefix("events: "))
            .unwrap()
            .split_whitespace()
            .collect::<Vec<_>>();
        let totals = data
            .lines()
            .find_map(|l| l.strip_prefix("totals: "))
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let ir = totals[events.iter().position(|e| *e == "Ir").unwrap()];
        assert!(ir > 0);
        if label[0] == "setup" {
            assert!(setup.replace(ir).is_none())
        } else {
            assert!(query.replace(ir).is_none())
        }
    }
    let query = query.unwrap();
    let total = if row["arm"] == "rootfind-mitm" {
        query.checked_add(setup.unwrap()).unwrap()
    } else {
        assert!(setup.is_none());
        query
    };
    (row, query, total)
}
fn values(v: &[f64]) -> Value {
    json!({"n":v.len(),"median_ir":median(v),"min_ir":v.iter().copied().fold(f64::INFINITY,f64::min),"max_ir":v.iter().copied().fold(0.0,f64::max),"samples_ir":v})
}
fn main() {
    let root = std::env::args()
        .nth(1)
        .expect("analyze-profiles PROFILES_DIRECTORY");
    let root = Path::new(&root);
    let mut rows = Vec::new();
    for arm in [
        "matrix-f4",
        "matrix-f5",
        "generalized-native",
        "rootfind-mitm",
    ] {
        let (mut before, mut after, mut queries_a, mut queries_b, mut ratios) =
            (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
        for round in 0..5 {
            let (a, qa, ca) = profile(&root.join(format!("{arm}-{round}-a")));
            let (b, qb, cb) = profile(&root.join(format!("{arm}-{round}-b")));
            assert_eq!(stable(&a), stable(&b));
            if arm == "rootfind-mitm" {
                assert_eq!(a["indices"], b["indices"]);
                assert_eq!(a["setup_group_ops"], b["setup_group_ops"]);
                assert_eq!(
                    a["setup_counts"]["pair_table_entries"],
                    b["setup_counts"]["pair_table_entries"]
                );
            }
            before.push(ca as f64);
            after.push(cb as f64);
            queries_a.push(qa as f64);
            queries_b.push(qb as f64);
            ratios.push(ca as f64 / cb as f64);
        }
        rows.push(json!({"arm":arm,"n":23,"seed":230,"target_kind":"planted","before":values(&before),"after":values(&after),
            "before_query":values(&queries_a),"after_query":values(&queries_b),"paired_reference_over_candidate":median(&ratios),
            "median_ir_reduction_percent":100.0*(1.0-median(&after)/median(&before)),"ratios":ratios}));
    }
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"boolean-followup-ir/v1","rows":rows,
        "scope":"Valgrind3.19 merged workers, simulated AVX2, one worker; complete-call Ir; Semaev cold includes setup; no native elapsed time, cycles or IPC"})).unwrap());
}
