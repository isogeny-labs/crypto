//! Price the complete published-class quartic line-sieve DLP, including
//! line enumeration, relation elimination, descent, and verification.
//! Source-to-Jacobian transfer is absent: this is one destination fixture.
use serde_json::{json, Value};
use std::fs;

const BASE: &str = "research/five_leads_e2e_20261019";
fn u(v: &Value, key: &str) -> u64 {
    v[key].as_u64().unwrap_or_else(|| panic!("missing {key}"))
}
fn counts(v: &Value) -> [u64; 4] {
    [u(v, "m"), u(v, "s"), u(v, "i"), u(v, "field_adds")]
}
fn add(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    std::array::from_fn(|j| a[j].checked_add(b[j]).unwrap())
}
fn price(c: [u64; 4], s: f64, i: f64, a: f64) -> f64 {
    c[0] as f64 + s * c[1] as f64 + i * c[2] as f64 + a * c[3] as f64
}
fn main() {
    let run: Value = serde_json::from_slice(
        &fs::read(format!(
            "{BASE}/correctness/tian-quartic-line-sieve-dlp-v1.json"
        ))
        .unwrap(),
    )
    .unwrap();
    assert_eq!(run["schema"], "tian-quartic-line-sieve-dlp/v1");
    assert_eq!(run["p"], 523);
    assert_eq!(run["r"], 143069629);
    assert_eq!(run["affine_rational_points"], 522);
    assert_eq!(run["fully_split_lines"], 11116);
    assert_eq!(run["relation_rank"], 521);
    assert_eq!(run["nullity"], 1);
    assert_eq!(run["descent"]["recovered"], 1987);
    assert_eq!(run["descent"]["source_verified"], true);
    assert_eq!(run["descent"]["destination_verified"], true);
    assert_eq!(run["descent"]["source_map_executed"], false);
    assert_eq!(run["descent"]["index_calculus_executed"], true);
    let mut curve = add(
        counts(&run["curve_field_counts"]),
        counts(&run["descent"]["after_verification_curve_field_counts"]),
    );
    curve = add(curve, counts(&run["descent"]["source_verification_counts"]));
    let relation = &run["scalar_field_counts"];
    let scalar = [
        u(relation, "multiplications") + u(&run["descent"], "scalar_kernel_multiplications"),
        0,
        u(relation, "inversions") + u(&run["descent"], "scalar_recovery_inversions"),
        u(relation, "additions") + u(&run["descent"], "scalar_kernel_additions"),
    ];
    let full = add(curve, scalar);
    let rho: Vec<Value> = fs::read_to_string(format!("{BASE}/correctness/tian-cubic-rho-v2.jsonl"))
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(rho.len(), 5);
    assert!(rho
        .iter()
        .all(|r| r["verified"] == true && r["recovered"] == run["descent"]["recovered"]));
    let rho_counts: Vec<_> = rho.iter().map(|r| counts(&r["counts"])).collect();
    let mut csv=String::from("S_per_M,I_per_M,add_per_M,cold_candidate_GAE,mean_source_rho_GAE,cold_candidate_over_mean_rho\n");
    let mut rows = Vec::new();
    for s in [0.0, 0.5, 1.0] {
        for i in [1.0, 10.0, 100.0, 1000.0] {
            for a in [0.0, 0.1, 1.0] {
                let candidate = price(full, s, i, a);
                let baseline = rho_counts.iter().map(|&c| price(c, s, i, a)).sum::<f64>()
                    / rho_counts.len() as f64;
                let ratio = candidate / baseline;
                csv.push_str(&format!("{s},{i},{a},{candidate},{baseline},{ratio}\n"));
                rows.push(json!({"S_per_M":s,"I_per_M":i,"add_per_M":a,"cold_candidate_GAE":candidate,"mean_source_rho_GAE":baseline,"cold_candidate_over_mean_rho":ratio}));
            }
        }
    }
    let representative = rows
        .iter()
        .find(|r| r["S_per_M"] == 0.5 && r["I_per_M"] == 10.0 && r["add_per_M"] == 0.1)
        .unwrap();
    let min = rows
        .iter()
        .map(|r| r["cold_candidate_over_mean_rho"].as_f64().unwrap())
        .fold(f64::INFINITY, f64::min);
    let max = rows
        .iter()
        .map(|r| r["cold_candidate_over_mean_rho"].as_f64().unwrap())
        .fold(f64::NEG_INFINITY, f64::max);
    let summary = json!({"schema":"tian-quartic-line-sieve-analysis/v1","published_fixture_only":true,
        "source_map_executed":false,"index_calculus_executed":true,"solver_received_planted_scalar":false,
        "source_and_destination_verified":true,"recovered":1987,"rho_runs":rho.len(),
        "affine_factor_base":522,"fully_split_lines":11116,"relation_rank":521,
        "descent_attempts":run["descent"]["attempts"].as_array().unwrap().len(),
        "counts":{"curve_and_source_field":curve,"scalar_relation_and_recovery_field":scalar,"cold_total":full},
        "representative":representative,"cold_ratio_band":[min,max],"candidate_wins":rows.iter().filter(|r|r["cold_candidate_over_mean_rho"].as_f64().unwrap()<1.0).count(),
        "candidate_losses":rows.iter().filter(|r|r["cold_candidate_over_mean_rho"].as_f64().unwrap()>1.0).count(),
        "unpriced":"memory allocation, comparisons, hash and control flow; wall speed not established"});
    fs::write(
        format!("{BASE}/results/tian-quartic-line-sieve-price-grid.csv"),
        csv,
    )
    .unwrap();
    fs::write(
        format!("{BASE}/results/tian-quartic-line-sieve-summary.json"),
        serde_json::to_vec_pretty(&summary).unwrap(),
    )
    .unwrap();
    println!("{}", serde_json::to_string_pretty(&summary).unwrap());
}
