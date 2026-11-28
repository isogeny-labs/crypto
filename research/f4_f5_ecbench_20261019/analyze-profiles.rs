//! Auxiliary complete-call instruction counts; never native elapsed time.
#[path = "../f4_f5_boolean_runtime_20261019/analyze.rs"]
#[allow(dead_code)]
mod common;
use common::{median, stable};
use serde_json::{json, Value};
use std::{fs, path::Path};
pub(crate) fn sample(path: &Path) -> Result<(Value, u64), String> {
    let exit = fs::read_to_string(path.join("exit.txt")).unwrap_or_else(|_| "missing".into());
    if exit.trim() != "0" {
        return Err(format!("exit {}", exit.trim()));
    }
    let text = fs::read_to_string(path.join("run.jsonl")).map_err(|e| e.to_string())?;
    let row: Value = serde_json::from_str(text.trim()).map_err(|e| e.to_string())?;
    assert_eq!(row["verified"], true);
    assert_eq!(row["stats"]["exhausted"], false);
    assert_eq!(row["stats"]["unsupported"], false);
    let mut found = None;
    for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
        let file = entry.unwrap().path();
        if !file
            .file_name()
            .unwrap()
            .to_string_lossy()
            .contains(".callgrind")
        {
            continue;
        }
        let text = fs::read_to_string(file).unwrap();
        let Some(label) = text
            .lines()
            .find_map(|l| l.strip_prefix("desc: Trigger: Client Request: "))
        else {
            continue;
        };
        let label: Value = serde_json::from_str(label).unwrap();
        assert_eq!(label[0], row["arm"]);
        assert!(!text.lines().any(|l| l.starts_with("thread:")));
        let events: Vec<_> = text
            .lines()
            .find_map(|l| l.strip_prefix("events: "))
            .unwrap()
            .split_whitespace()
            .collect();
        let totals: Vec<u64> = text
            .lines()
            .find_map(|l| l.strip_prefix("totals: "))
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let ir = totals[events.iter().position(|s| *s == "Ir").unwrap()];
        assert!(ir > 0);
        assert!(found.replace(ir).is_none());
    }
    Ok((
        row,
        found.ok_or("missing client-request instruction region")?,
    ))
}
fn values(values: &[f64]) -> Value {
    if values.is_empty() {
        return json!({"n":0});
    }
    json!({"n":values.len(),"median_ir":median(values),"min_ir":values.iter().copied().fold(f64::INFINITY,f64::min),"max_ir":values.iter().copied().fold(0.0,f64::max),"samples_ir":values})
}
fn main() {
    let root = std::env::args().nth(1).expect("analyze-profiles PROFILES");
    let root = Path::new(&root);
    let mut rows = Vec::new();
    let mut failures = Vec::new();
    for arm in ["solve-f4", "solve-f5"] {
        let mut before = Vec::new();
        let mut prepared = Vec::new();
        let mut tail = Vec::new();
        let mut ratios = Vec::new();
        for round in 0..5 {
            let mut batch = Vec::new();
            for policy in ["baseline", "prepared", "quadratic-tail"] {
                let name = format!("{arm}-{round}-{policy}");
                match sample(&root.join(&name)) {
                    Ok(result) => batch.push(result),
                    Err(reason) => failures.push(json!({"name":name,"reason":reason})),
                }
            }
            if batch.len() != 3 {
                continue;
            }
            assert_eq!(stable(&batch[0].0), stable(&batch[1].0));
            assert_eq!(stable(&batch[0].0), stable(&batch[2].0));
            before.push(batch[0].1 as f64);
            prepared.push(batch[1].1 as f64);
            tail.push(batch[2].1 as f64);
            ratios.push(batch[0].1 as f64 / batch[2].1 as f64);
        }
        rows.push(json!({"arm":arm,"n":23,"seed":230,"before":values(&before),"prepared":values(&prepared),"candidate":values(&tail),"paired_ir_ratio":(!ratios.is_empty()).then(||median(&ratios)),"complete":ratios.len()==5}));
    }
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"boolean-recursive-instructions/v1","rows":rows,"failures":failures,"scope":"Local Valgrind merged-thread Ir, simulated AVX2, one worker, complete-root calls; no native speed, cycles or IPC claim"})).unwrap());
}
