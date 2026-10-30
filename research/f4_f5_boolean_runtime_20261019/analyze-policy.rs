//! Fresh crossover and typed-entry confirmation. All ratios are within one host.
#[path = "analyze.rs"]
#[allow(dead_code)]
mod common;
use common::{comparison, median, select, spread, stable};
use serde_json::json;
use std::{fs, path::Path};

fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("analyze-policy OUTPUT_DIRECTORY");
    let root = Path::new(&arg);
    assert_eq!(
        fs::read_to_string(root.join("policy-phase.status"))
            .unwrap()
            .trim(),
        "complete"
    );
    let mut rejected = Vec::new();
    let mut controls = Vec::new();
    let mut changes = Vec::new();
    for n in [17, 23] {
        for (seed, random) in [(230, 0), (230, 1), (231, 0), (232, 0)] {
            for arm in ["generalized-native", "decomp-generalized"] {
                let case = format!("{arm}-n{n}-s{seed}-r{random}");
                let mut aa = Vec::new();
                for round in 0..5 {
                    let a = select(
                        root,
                        &format!("policy-controls/{case}-{round}-a"),
                        &mut rejected,
                    );
                    let b = select(
                        root,
                        &format!("policy-controls/{case}-{round}-b"),
                        &mut rejected,
                    );
                    if let (Some(a), Some(b)) = (a, b) {
                        assert_eq!(stable(&a.row), stable(&b.row));
                        aa.push(a.cold / b.cold);
                    }
                }
                let noise = if aa.len() == 5 {
                    median(&aa.iter().map(|r| (r - 1.0).abs()).collect::<Vec<_>>())
                } else {
                    f64::INFINITY
                };
                controls.push(json!({"case":case,"ratios":aa,"median_absolute_noise":if noise.is_finite(){Some(noise)}else{None}}));
                let (mut before, mut after, mut wide, mut typed) =
                    (Vec::new(), Vec::new(), Vec::new(), Vec::new());
                let (mut policy_ratios, mut wide_ratios, mut typed_ratios, mut combined_ratios) =
                    (Vec::new(), Vec::new(), Vec::new(), Vec::new());
                let mut sources = Vec::new();
                let mut methods = Vec::new();
                for round in 0..5 {
                    let prefix = format!("policy/{case}-{round}");
                    let a = select(root, &format!("{prefix}-a"), &mut rejected);
                    let b = select(root, &format!("{prefix}-b"), &mut rejected);
                    let w = select(root, &format!("{prefix}-wide"), &mut rejected);
                    let t = select(root, &format!("{prefix}-typed"), &mut rejected);
                    if let (Some(a), Some(b), Some(w), Some(t)) = (a, b, w, t) {
                        for other in [&b, &w, &t] {
                            assert_eq!(
                                stable(&a.row),
                                stable(&other.row),
                                "{} vs {}",
                                a.name,
                                other.name
                            );
                        }
                        assert_eq!(b.row["explicit_wide"], false);
                        assert_eq!(w.row["explicit_wide"], true);
                        assert_eq!(b.row["seed"], seed);
                        before.push(a.cold);
                        after.push(b.cold);
                        wide.push(w.cold);
                        typed.push(t.cold);
                        policy_ratios.push(a.cold / b.cold);
                        wide_ratios.push(w.cold / b.cold);
                        typed_ratios.push(b.cold / t.cold);
                        combined_ratios.push(a.cold / t.cold);
                        methods.push(json!({"auto":b.row["profile"],"typed":t.row["profile"]}));
                        sources.push(json!([a.name, b.name, w.name, t.name]));
                    }
                }
                changes.push(json!({"arm":arm,"n":n,"seed":seed,"random":random,
                    "before":spread(&before),"after":spread(&after),"wide":spread(&wide),"typed":spread(&typed),
                    "policy_change":comparison(&policy_ratios,noise),"explicit_wide_over_auto":comparison(&wide_ratios,noise),
                    "json_over_typed":comparison(&typed_ratios,noise),"before_over_typed":comparison(&combined_ratios,noise),
                    "sources":sources,"methods":methods}));
            }
        }
    }
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"boolean-native-policy/v1","baseline":"frozen-iteration1 (before cutoff and typed entry)",
        "changes":changes,"controls":controls,"rejected_attempts":rejected,"scope":"Fresh same-host five-pair controls, planted seed230 and generator plus planted holdouts231/232; exact original roots or group-verified witness"})).unwrap());
}
