use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::fs;

fn main() {
    let dir = "research/kummer_walk_20261022/results";
    let raw = fs::read_to_string(format!("{dir}/rosenhain-models.jsonl")).unwrap();
    let mut primes = BTreeSet::new();
    let mut passed = 0;
    let mut failures = Vec::new();
    let mut square_pairs = BTreeSet::new();
    for line in raw.lines() {
        let row: Value = serde_json::from_str(line).unwrap();
        assert_eq!(row["schema"], "kummer-rosenhain-model/v1");
        let p = row["p"].as_u64().unwrap();
        assert!(primes.insert(p));
        assert_eq!(row["pairs_tried"], 20);
        square_pairs.insert(row["square_pairs"].as_u64().unwrap());
        let selected = &row["selected"];
        let scale = selected["scale"].as_u64().unwrap();
        let sqrt = selected["sqrt_scale"].as_u64().unwrap();
        let roots = selected["lambda_mu_nu"].as_array().unwrap();
        let all_roots_distinct = {
            let mut seen = BTreeSet::from([0u64, 1u64]);
            roots.iter().all(|v| seen.insert(v.as_u64().unwrap()))
        };
        let valid = row["input_factorization_verified"] == true
            && row["fp_rosenhain_map_found"] == true
            && row["all_correct"] == true
            && selected["coefficient_identity"] == true
            && selected["observed_coefficients"] == selected["reconstructed_coefficients"]
            && sqrt * sqrt % p == scale
            && all_roots_distinct;
        if valid {
            passed += 1;
        } else {
            failures.push(p);
        }
    }
    assert_eq!(primes.len(), 21);
    let summary = json!({"schema":"kummer-rosenhain-summary/v1",
        "models":primes.len(),"passed":passed,"failures":failures,
        "square_pairs_values":square_pairs,
        "coordinate_only_kummer_verified":false,
        "full_collector_targets":0,"candidate_over_rho":Value::Null});
    fs::write(
        format!("{dir}/rosenhain-summary.json"),
        serde_json::to_string_pretty(&summary).unwrap() + "\n",
    )
    .unwrap();
    println!("{}", summary);
}
