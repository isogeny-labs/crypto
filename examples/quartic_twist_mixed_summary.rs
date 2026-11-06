//! Native derivation of paired mixed-line metrics from immutable raw NDJSON.
use std::collections::BTreeMap;
use std::fs;

use serde_json::{json, Value};

fn u(v: &Value) -> u64 {
    v.as_u64().expect("u64")
}
fn median(mut a: Vec<f64>) -> f64 {
    a.sort_by(|x, y| x.total_cmp(y));
    a[a.len() / 2]
}
fn cost_m(r: &Value) -> u64 {
    // Counted Fp multiplications on the twist24 path. The producer's
    // other-arm verification and row arithmetic are separate and this
    // number is not a complete GAE or whole-process cost.
    [
        "model_fp_muls",
        "input_fp_muls",
        "twist_geometry_fp_muls",
        "cubic_eigen_fp_muls",
        "frob_eigen_fp_muls",
        "base_fp_muls",
        "twist_map_fp_muls",
        "table_fp_muls",
    ]
    .iter()
    .map(|&k| u(&r[k]))
    .sum::<u64>()
        + u(&r["stream_fp_muls_at_solve"][2])
        + u(&r["verification_fp_muls"][2])
}
fn main() {
    let raw = fs::read_to_string(std::env::args().nth(1).expect("NDJSON path")).unwrap();
    let records: Vec<Value> = raw
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| serde_json::from_str(x).unwrap())
        .collect();
    assert_eq!(records.len(), 42);
    let mut pairs = BTreeMap::<(u64, u64), Vec<&Value>>::new();
    for r in &records {
        pairs
            .entry((u(&r["p"]), u(&r["seed"])))
            .or_default()
            .push(r);
    }
    assert_eq!(pairs.len(), 21);
    let mut per_curve = BTreeMap::<u64, Vec<Value>>::new();
    for ((p, seed), items) in pairs {
        assert_eq!(items.len(), 2);
        let (single, mixed) = if items[0]["line_powers"] == json!([0]) {
            (items[0], items[1])
        } else {
            (items[1], items[0])
        };
        assert_eq!(single["line_powers"], json!([0]));
        assert_eq!(mixed["line_powers"], json!([0, 2]));
        assert_eq!(single["target_x"], mixed["target_x"]);
        assert_eq!(single["target_y"], mixed["target_y"]);
        assert!(
            single["twist24"]["correct"].as_bool().unwrap()
                && mixed["twist24"]["correct"].as_bool().unwrap()
        );
        let (a, b) = (
            single["query_trace"].as_array().unwrap(),
            mixed["query_trace"].as_array().unwrap(),
        );
        let common = a.len().min(b.len());
        let (mut ah, mut bh) = (0u64, 0u64);
        for i in 0..common {
            assert_eq!((&a[i][0], &a[i][1]), (&b[i][0], &b[i][1]));
            let x = u(&a[i][2]) > 0;
            let y = u(&b[i][2]) > 0;
            assert!(!x || y);
            ah += u64::from(x);
            bh += u64::from(y);
        }
        let (sm, mm) = (cost_m(single), cost_m(mixed));
        let x = json!({
            "p":p,"seed":seed,"r":u(&single["n"]),
            "base_single":u(&single["base_points"]),"base_mixed":u(&mixed["base_points"]),
            "columns_single":u(&single["twist24"]["columns"]),"columns_mixed":u(&mixed["twist24"]["columns"]),
            "shared_targets":common,"shared_hits_single":ah,"shared_hits_mixed":bh,
            "shared_yield_ratio":if ah>0 {Some(bh as f64/ah as f64)} else {None},
            "twist_residuals_single":u(&single["twist24"]["residuals_at_solve"]),
            "twist_residuals_mixed":u(&mixed["twist24"]["residuals_at_solve"]),
            "query_ratio":u(&single["twist24"]["residuals_at_solve"]) as f64/u(&mixed["twist24"]["residuals_at_solve"]) as f64,
            "twist_la_ops_single":u(&single["twist24"]["la_ops"]),
            "twist_la_ops_mixed":u(&mixed["twist24"]["la_ops"]),
            "la_ratio_single_over_mixed":u(&single["twist24"]["la_ops"]) as f64/u(&mixed["twist24"]["la_ops"]) as f64,
            "twist_fp_muls_single":sm,"twist_fp_muls_mixed":mm,
            "m_ratio_single_over_mixed":sm as f64/mm as f64,
            "table_fp_muls_single":u(&single["table_fp_muls"]),
            "table_fp_muls_mixed":u(&mixed["table_fp_muls"]),
            "decompositions_single":u(&single["decompositions"]),
            "decompositions_mixed":u(&mixed["decompositions"]),
        });
        per_curve.entry(p).or_default().push(x);
    }
    let mut curves = Vec::new();
    for (p, rows) in &per_curve {
        assert_eq!(rows.len(), 3);
        let get = |key: &str| {
            median(
                rows.iter()
                    .map(|r| r[key].as_f64().expect("ratio"))
                    .collect(),
            )
        };
        curves.push(json!({
            "p":p,"r":u(&rows[0]["r"]),
            "base_single":u(&rows[0]["base_single"]),"base_mixed":u(&rows[0]["base_mixed"]),
            "columns_single":u(&rows[0]["columns_single"]),"columns_mixed":u(&rows[0]["columns_mixed"]),
            "shared_yield_ratio_median":get("shared_yield_ratio"),
            "query_ratio_median":get("query_ratio"),
            "la_ratio_median":get("la_ratio_single_over_mixed"),
            "m_ratio_median":get("m_ratio_single_over_mixed"),
            "m_ratio_min":rows.iter().map(|r|r["m_ratio_single_over_mixed"].as_f64().unwrap()).fold(f64::INFINITY,f64::min),
            "m_ratio_max":rows.iter().map(|r|r["m_ratio_single_over_mixed"].as_f64().unwrap()).fold(f64::NEG_INFINITY,f64::max),
            "runs":rows,
        }));
    }
    println!(
        "{}",
        json!({"status":"derived","runs":records.len(),"pairs":21,"curves":curves})
    );
}
