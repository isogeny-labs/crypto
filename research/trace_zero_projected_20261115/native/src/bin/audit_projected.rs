//! Post-run grid and frozen-goal auditor; separate from the frozen analysis.
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
    let s: Value = serde_json::from_slice(&fs::read(&a[1]).unwrap()).unwrap();
    let mut seen = BTreeSet::new();
    let mut common = BTreeMap::<String, Value>::new();
    let arms = [
        "original-eager",
        "original-lazy",
        "v1-eager",
        "v1-lazy",
        "projected-eager",
        "projected-lazy",
        "original-eager-aa",
    ];
    let cases: BTreeSet<_> = [83, 131]
        .iter()
        .flat_map(|n| {
            ["distinct-planted-0", "uniform", "cancellation"]
                .iter()
                .map(move |k| format!("scale-n{n}-d12-{k}"))
        })
        .collect();
    let mut projected = BTreeMap::new();
    let mut found = 0;
    for r in &rows {
        let id = r["instance"].as_str().unwrap();
        let arm = r["arm"].as_str().unwrap();
        let rep = r["replicate"].as_u64().unwrap();
        assert!(cases.contains(id) && arms.contains(&arm) && rep < 5);
        assert!(seen.insert((id.to_string(), arm.to_string(), rep)));
        let z = &r["result"];
        assert_eq!(z["valid"], true);
        assert_eq!(
            r["instructions"].as_u64().unwrap(),
            r["setup_instructions"].as_u64().unwrap()
                + r["oracle_instructions"].as_u64().unwrap()
                + r["post_instructions"].as_u64().unwrap()
        );
        if id.ends_with("uniform") {
            assert_eq!(z["outcome"], "Refuted");
        } else {
            assert_eq!(z["outcome"], "Found");
            assert_eq!(z["replay"], true);
            found += 1;
        }
        if let Some(old) = common.get(id) {
            for k in [
                "outcome",
                "indices",
                "points",
                "models",
                "patterns_attempted",
                "patterns_refuted",
                "reason",
                "common",
            ] {
                assert_eq!(old[k], z[k], "{id}/{arm}/{rep}/{k}");
            }
        } else {
            common.insert(id.to_string(), z.clone());
        }
        if arm.starts_with("projected") && !id.ends_with("cancellation") {
            assert_eq!(z["projected"]["columns"], 13);
            assert!(z["projected"]["systems"].as_u64().unwrap() > 0);
            projected.insert(format!("{id}/{arm}"), z["projected"].clone());
        }
    }
    assert_eq!(rows.len(), 210);
    assert_eq!(seen.len(), 210);
    let mut criteria = Vec::new();
    let mut goal = true;
    for n in [83, 131] {
        let id = format!("scale-n{n}-d12-uniform");
        let get = |arm: &str, m: &str| {
            s["cells"][format!("{id}/{arm}")]["metrics"][m]["median"]
                .as_u64()
                .unwrap()
        };
        let oracle = get("v1-eager", "oracle_instructions") as f64
            / get("projected-eager", "oracle_instructions") as f64;
        let whole =
            get("v1-eager", "instructions") as f64 / get("projected-lazy", "instructions") as f64;
        let aa = get("original-eager", "instructions")
            .abs_diff(get("original-eager-aa", "instructions")) as f64
            / get("original-eager", "instructions") as f64;
        let passes = oracle >= 5. && whole >= 2. && aa <= 0.01;
        goal &= passes;
        criteria.push(json!({"instance":id,"v1_over_projected_oracle":oracle,"v1_eager_over_projected_lazy_whole":whole,"aa_relative_median_difference":aa,"passes":passes}));
    }
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"trace-zero-projected-grid-goal-audit/v1","planned":210,"attempted":210,"completed":210,"invalid":0,"unique_grid":true,"exact_common_outputs":true,"phase_sums_match":true,"found_point_tuples":found,"fresh_primary_child_processes":420,"goal_criteria":criteria,"goal_met":goal,"projected_diagnostics":projected,"standing":"producer-written post-run audit; unsigned, no independently signed verdict"})).unwrap());
}
