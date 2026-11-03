//! Price the fixed published destination DLP solved by bounded linear scan.
//! This is not an executable source transfer or genus-3 index calculus.
use serde_json::{json, Value};
use std::fs;

const BASE: &str = "research/five_leads_e2e_20261019";
const FIELDS: [&str; 3] = ["m", "s", "i"];
fn counts(v: &Value) -> [u64; 3] {
    FIELDS.map(|key| v[key].as_u64().expect("field count"))
}
fn delta(a: [u64; 3], b: [u64; 3]) -> [u64; 3] {
    std::array::from_fn(|j| b[j].checked_sub(a[j]).expect("monotone counts"))
}
fn add(a: [u64; 3], b: [u64; 3]) -> [u64; 3] {
    std::array::from_fn(|j| a[j].checked_add(b[j]).expect("cost overflow"))
}
fn price(c: [u64; 3], s: f64, i: f64) -> f64 {
    c[0] as f64 + s * c[1] as f64 + i * c[2] as f64
}
fn main() {
    let run: Value = serde_json::from_slice(
        &fs::read(format!("{BASE}/correctness/tian-km-linear-dlp-v1.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(run["schema"], "tian-km-linear-dlp/v1");
    assert_eq!(run["source_verified"], true);
    assert_eq!(run["source_map_executed"], false);
    assert_eq!(run["index_calculus_executed"], false);
    assert_eq!(run["recovered"], 1987);
    let rho: Vec<Value> = fs::read_to_string(format!("{BASE}/correctness/tian-cubic-rho-v2.jsonl"))
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(rho.len(), 5);
    assert!(rho
        .iter()
        .all(|row| row["verified"] == true && row["recovered"] == run["recovered"]));
    let setup = counts(&run["after_setup_counts"]);
    let search = delta(setup, counts(&run["after_search_counts"]));
    let verification = counts(&run["source_verification_counts"]);
    let full = add(counts(&run["after_search_counts"]), verification);
    let rho_counts: Vec<_> = rho.iter().map(|row| counts(&row["counts"])).collect();
    let mut csv=String::from("S_per_M,I_per_M,cold_candidate_GAE,search_only_GAE,mean_source_rho_GAE,cold_candidate_over_mean_rho,search_only_over_mean_rho\n");
    let mut rows = Vec::new();
    for s in [0.0, 0.5, 1.0] {
        for i in [1.0, 10.0, 100.0, 1000.0] {
            let candidate = price(full, s, i);
            let search_cost = price(search, s, i);
            let mean =
                rho_counts.iter().map(|&c| price(c, s, i)).sum::<f64>() / rho_counts.len() as f64;
            let ratio = candidate / mean;
            let search_ratio = search_cost / mean;
            csv.push_str(&format!(
                "{s},{i},{candidate},{search_cost},{mean},{ratio},{search_ratio}\n"
            ));
            rows.push(json!({"S_per_M":s,"I_per_M":i,"cold_candidate_GAE":candidate,"search_only_GAE":search_cost,
                "mean_source_rho_GAE":mean,"cold_candidate_over_mean_rho":ratio,"search_only_over_mean_rho":search_ratio}));
        }
    }
    let representative = rows
        .iter()
        .find(|row| row["S_per_M"].as_f64() == Some(0.5) && row["I_per_M"].as_f64() == Some(10.0))
        .unwrap();
    let range = |key: &str| -> Value {
        json!([
            rows.iter()
                .map(|row| row[key].as_f64().unwrap())
                .fold(f64::INFINITY, f64::min),
            rows.iter()
                .map(|row| row[key].as_f64().unwrap())
                .fold(f64::NEG_INFINITY, f64::max)
        ])
    };
    let summary = json!({"schema":"tian-km-linear-dlp-analysis/v1","published_fixture_only":true,
        "source_map_executed":false,"index_calculus_executed":false,"solver_received_planted_scalar":false,
        "source_verified":true,"recovered":run["recovered"],"cap":run["cap"],"rho_runs":rho.len(),
        "counts":{"cold_setup":{"m":setup[0],"s":setup[1],"i":setup[2]},
                  "search_only":{"m":search[0],"s":search[1],"i":search[2]},
                  "source_verification":{"m":verification[0],"s":verification[1],"i":verification[2]},
                  "cold_total":{"m":full[0],"s":full[1],"i":full[2]}},
        "representative":representative,"cold_ratio_band":range("cold_candidate_over_mean_rho"),
        "search_ratio_band":range("search_only_over_mean_rho")});
    fs::create_dir_all(format!("{BASE}/results")).unwrap();
    fs::write(
        format!("{BASE}/results/tian-km-linear-dlp-price-grid.csv"),
        csv,
    )
    .unwrap();
    fs::write(
        format!("{BASE}/results/tian-km-linear-dlp-summary.json"),
        serde_json::to_vec_pretty(&summary).unwrap(),
    )
    .unwrap();
    println!("{}", serde_json::to_string_pretty(&summary).unwrap());
}
