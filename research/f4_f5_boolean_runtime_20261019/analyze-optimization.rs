//! Same-host implementation changes and final cross-method native costs.
#[path = "analyze.rs"]
#[allow(dead_code)]
mod common;
use common::{comparison, median, select, spread, stable, Sample};
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs, path::Path};

fn pair_equal(a: &Sample, b: &Sample) {
    assert_eq!(stable(&a.row), stable(&b.row), "{} vs {}", a.name, b.name);
    for key in [
        "word_ops",
        "rows",
        "cols",
        "calls",
        "criterion_word_ops",
        "specialise_word_ops",
    ] {
        assert_eq!(
            a.row["profile"][key], b.row["profile"][key],
            "counted {key}: {} vs {}",
            a.name, b.name
        );
    }
}
fn aa(root: &Path, case: &str, rejected: &mut Vec<Value>) -> (f64, Value) {
    let mut ratios = Vec::new();
    let mut sources = Vec::new();
    for round in 0..5 {
        let a = select(root, &format!("controls/{case}-{round}-a"), rejected);
        let b = select(root, &format!("controls/{case}-{round}-b"), rejected);
        if let (Some(a), Some(b)) = (a, b) {
            pair_equal(&a, &b);
            ratios.push(a.cold / b.cold);
            sources.push(json!([a.name, b.name]));
        }
    }
    let noise = if ratios.len() == 5 {
        median(&ratios.iter().map(|r| (r - 1.0).abs()).collect::<Vec<_>>())
    } else {
        f64::INFINITY
    };
    (
        noise,
        json!({"case":case,"ratios":ratios,"median_absolute_noise":if noise.is_finite(){Some(noise)}else{None},"sources":sources}),
    )
}
fn run(root: &Path) -> Value {
    let mut rejected = Vec::new();
    let mut changes = Vec::new();
    let mut controls = Vec::new();
    let mut final_costs = Vec::new();
    let mut matrices = Vec::new();
    for n in [17u32, 23] {
        for arm in ["matrix-f4", "matrix-f5"] {
            let case = format!("{arm}-n{n}-r0");
            let (noise, control) = aa(root, &case, &mut rejected);
            controls.push(control);
            let mut a = Vec::new();
            let mut b = Vec::new();
            let mut p = Vec::new();
            let mut legacy = Vec::new();
            let mut combined = Vec::new();
            let mut vector = Vec::new();
            let mut single = Vec::new();
            let mut sources = Vec::new();
            for round in 0..5 {
                let before = select(root, &format!("paired/{case}-{round}-a"), &mut rejected);
                let after = select(root, &format!("paired/{case}-{round}-b"), &mut rejected);
                let portable = select(
                    root,
                    &format!("ablation/{case}-{round}-portable"),
                    &mut rejected,
                );
                let old = select(
                    root,
                    &format!("ablation/{case}-{round}-legacy"),
                    &mut rejected,
                );
                if let (Some(before), Some(after)) = (before, after) {
                    pair_equal(&before, &after);
                    a.push(before.query);
                    b.push(after.query);
                    combined.push(before.query / after.query);
                    assert_eq!(before.row["table_write_once"], true);
                    assert_eq!(after.row["vector_tables"], true);
                    if let Some(portable) = portable {
                        pair_equal(&portable, &after);
                        p.push(portable.query);
                        vector.push(portable.query / after.query);
                    }
                    if let Some(old) = old {
                        pair_equal(&old, &before);
                        assert_eq!(old.row["table_write_once"], false);
                        legacy.push(old.query);
                        single.push(old.query / before.query);
                    }
                    sources.push(json!([before.name, after.name]));
                }
            }
            matrices.push(json!({"arm":arm,"n":n,"seed":230,"before":spread(&a),"after":spread(&b),"portable_candidate":spread(&p),"legacy":spread(&legacy),
                "combined":comparison(&combined,noise),"vector_only":comparison(&vector,noise),"original_single_write":comparison(&single,noise),"sources":sources}));
        }
    }
    for n in [17u32, 23] {
        for random in [0u8, 1] {
            let mut data = BTreeMap::<String, Vec<Option<Sample>>>::new();
            let mut noise_by_boundary = BTreeMap::new();
            for arm in [
                "solve-f4",
                "solve-f5",
                "solve-inherited",
                "generalized-native",
                "decomp-f4",
                "decomp-f5",
                "decomp-f6",
                "decomp-generalized",
            ] {
                let boundary = if arm.starts_with("decomp-") {
                    "decomposition"
                } else {
                    "complete_roots"
                };
                let case = format!("{arm}-n{n}-r{random}");
                let (noise, control) = aa(root, &case, &mut rejected);
                controls.push(control);
                if arm == "solve-f4" || arm == "decomp-f4" {
                    noise_by_boundary.insert(boundary, noise);
                }
                let mut before_values = Vec::new();
                let mut after_values = Vec::new();
                let mut ratios = Vec::new();
                let mut scalar_values = Vec::new();
                let mut scalar_ratios = Vec::new();
                let mut scalar_before = Vec::new();
                let mut sources = Vec::new();
                let mut rows = Vec::new();
                for round in 0..5 {
                    let before = select(root, &format!("paired/{case}-{round}-a"), &mut rejected);
                    let after = select(root, &format!("paired/{case}-{round}-b"), &mut rejected);
                    if let (Some(before), Some(after)) = (&before, &after) {
                        pair_equal(before, after);
                        before_values.push(before.cold);
                        after_values.push(after.cold);
                        ratios.push(before.cold / after.cold);
                        sources.push(json!([before.name, after.name]));
                        if arm.contains("generalized") {
                            if let Some(scalar) = select(
                                root,
                                &format!("ablation/{case}-{round}-scalar"),
                                &mut rejected,
                            ) {
                                pair_equal(&scalar, after);
                                scalar_values.push(scalar.cold);
                                scalar_ratios.push(scalar.cold / after.cold);
                                scalar_before.push(before.cold / scalar.cold);
                            }
                        }
                    }
                    rows.push(after);
                }
                changes.push(json!({"boundary":boundary,"arm":arm,"n":n,"random":random,"before":spread(&before_values),"after":spread(&after_values),
                "comparison":comparison(&ratios,noise),"scalar_after":spread(&scalar_values),"wide_only":comparison(&scalar_ratios,noise),
                "borrowing_evaluation_only":comparison(&scalar_before,noise),"sources":sources}));
                data.insert(arm.to_string(), rows);
            }
            for arm in [
                "roots-gray",
                "roots-wide",
                "roots-moebius",
                "roots-monica",
                "generic-gb",
                "decomp-f6-pairs",
                "rootfind-mitm",
                "rootfind-table",
            ] {
                let mut rows = Vec::new();
                for round in 0..5 {
                    rows.push(select(
                        root,
                        &format!("references/{arm}-n{n}-r{random}-{round}"),
                        &mut rejected,
                    ));
                }
                data.insert(arm.to_string(), rows);
            }
            let mut fingerprints = BTreeMap::new();
            for (arm, rows) in &data {
                let boundary = if arm.starts_with("decomp-") || arm.starts_with("rootfind-") {
                    "decomposition"
                } else {
                    "complete_roots"
                };
                let reference = if boundary == "decomposition" {
                    "decomp-f4"
                } else {
                    "solve-f4"
                };
                let mut cold = Vec::new();
                let mut query = Vec::new();
                let mut ratios = Vec::new();
                let mut f6_ratios = Vec::new();
                let mut sources = Vec::new();
                let mut methods = Vec::new();
                let mut phase_values = BTreeMap::<&str, Vec<f64>>::new();
                for (round, row) in rows.iter().enumerate() {
                    if let Some(s) = row {
                        assert_eq!(s.row["arm"], *arm);
                        assert_eq!(s.row["n"], n);
                        assert_eq!(s.row["random"], random == 1);
                        let fp = stable(&s.row);
                        if let Some(old) = fingerprints.get(boundary) {
                            assert_eq!(*old, fp, "cross-method output: {arm}")
                        } else {
                            fingerprints.insert(boundary, fp);
                        }
                        cold.push(s.cold);
                        query.push(s.query);
                        sources.push(s.name.clone());
                        if let Some(r) = &data[reference][round] {
                            ratios.push(r.cold / s.cold);
                        }
                        if boundary == "decomposition" {
                            if let Some(r) = &data["decomp-f6"][round] {
                                f6_ratios.push(r.cold / s.cold);
                            }
                        }
                        methods.push(json!({"generalized_method":s.row["generalized_method"],"wide_lanes":s.row["wide_lanes"]}));
                        for key in ["build_ns", "reduce_ns", "readback_ns"] {
                            phase_values
                                .entry(key)
                                .or_default()
                                .push(s.row["profile"][key].as_u64().unwrap_or(0) as f64);
                        }
                    }
                }
                let phases = phase_values
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), spread(&v)))
                    .collect::<serde_json::Map<String, Value>>();
                final_costs.push(json!({"boundary":boundary,"arm":arm,"n":n,"random":random,"cold":spread(&cold),"query":spread(&query),
                "reference":reference,"comparison":comparison(&ratios,noise_by_boundary[boundary]),"f6_over_candidate":comparison(&f6_ratios,noise_by_boundary[boundary]),
                "phase_medians":phases,"sources":sources,"methods":methods}));
            }
        }
    }
    let mut holdouts = Vec::new();
    for seed in [231, 232] {
        for arm in ["matrix-f4", "matrix-f5"] {
            let mut before_values = Vec::new();
            let mut after_values = Vec::new();
            let mut ratios = Vec::new();
            let mut sources = Vec::new();
            let control = controls
                .iter()
                .find(|r| r["case"] == format!("{arm}-n23-r0"))
                .unwrap();
            let noise = control["median_absolute_noise"]
                .as_f64()
                .unwrap_or(f64::INFINITY);
            for round in 0..5 {
                let a = select(
                    root,
                    &format!("holdouts/{arm}-n23-s{seed}-{round}-a"),
                    &mut rejected,
                );
                let b = select(
                    root,
                    &format!("holdouts/{arm}-n23-s{seed}-{round}-b"),
                    &mut rejected,
                );
                if let (Some(a), Some(b)) = (a, b) {
                    pair_equal(&a, &b);
                    before_values.push(a.query);
                    after_values.push(b.query);
                    ratios.push(a.query / b.query);
                    sources.push(json!([a.name, b.name]));
                }
            }
            holdouts.push(json!({"arm":arm,"n":23,"seed":seed,"before":spread(&before_values),"after":spread(&after_values),"comparison":comparison(&ratios,noise),"sources":sources}));
        }
    }
    json!({"schema":"boolean-native-optimization/v1","baseline":"1340a6176","matrices":matrices,"changes":changes,"final_costs":final_costs,"holdouts":holdouts,
        "controls":controls,"rejected_attempts":rejected,"scope":"same-host paired builds; one Rayon worker; exact complete roots or checked decomposition verdict; cold oracle setup included",
        "source_sha256":fs::read_to_string(root.join("source.sha256")).ok(),"binary_sha256":fs::read_to_string(root.join("binaries.sha256")).ok()})
}
fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("analyze-optimization OUTPUT_DIRECTORY");
    println!(
        "{}",
        serde_json::to_string_pretty(&run(Path::new(&path))).unwrap()
    );
}
