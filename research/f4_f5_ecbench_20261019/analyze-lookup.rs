//! Decide the instruction-only refinement gate frozen before profiles.
#[path = "../f4_f5_boolean_runtime_20261019/analyze.rs"]
#[allow(dead_code)]
mod common;
#[path = "analyze-profiles.rs"]
#[allow(dead_code)]
mod profiles;
use common::{median, stable};
use serde_json::json;
use std::path::Path;
fn main() {
    let root = std::env::args()
        .nth(1)
        .expect("analyze-lookup LOOKUP_PROFILES");
    let root = Path::new(&root);
    let mut rows = Vec::new();
    let mut failures = Vec::new();
    for arm in ["solve-f4", "solve-f5"] {
        let mut before = Vec::new();
        let mut after = Vec::new();
        let mut ratios = Vec::new();
        for round in 0..5 {
            let mut pair = Vec::new();
            for side in ["a", "b"] {
                let name = format!("{arm}-{round}-{side}");
                match profiles::sample(&root.join(&name)) {
                    Ok(sample) => pair.push(sample),
                    Err(reason) => failures.push(json!({"name":name,"reason":reason})),
                }
            }
            if pair.len() != 2 {
                continue;
            }
            assert_eq!(stable(&pair[0].0), stable(&pair[1].0));
            assert_eq!(pair[0].0["stats"], pair[1].0["stats"]);
            for key in [
                "calls",
                "rows",
                "cols",
                "word_ops",
                "rows_pruned",
                "criterion_word_ops",
                "specialise_word_ops",
            ] {
                assert_eq!(pair[0].0["profile"][key], pair[1].0["profile"][key]);
            }
            before.push(pair[0].1 as f64);
            after.push(pair[1].1 as f64);
            ratios.push(pair[0].1 as f64 / pair[1].1 as f64);
        }
        let reduction = (!ratios.is_empty()).then(|| 1.0 - median(&after) / median(&before));
        rows.push(json!({"arm":arm,"pairs":ratios.len(),"before_ir":(!before.is_empty()).then(||median(&before)),"after_ir":(!after.is_empty()).then(||median(&after)),"paired_ir_ratio":(!ratios.is_empty()).then(||median(&ratios)),"ir_reduction_fraction":reduction,"before_samples":before,"after_samples":after,"retain":ratios.len()==5 && reduction.is_some_and(|r|r>0.02)}));
    }
    let retain = rows.iter().all(|r| r["retain"] == true);
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"boolean-column-lookup-instructions/v1","rows":rows,"retain_refinement":retain,"failures":failures,"scope":"Same-host complete-root Ir and identical deterministic counters; native timing confirmation is a separate requirement"})).unwrap());
}
