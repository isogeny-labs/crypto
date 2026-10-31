//! Independent headline recomputation from the committed in-process raw rows.
use serde_json::{json, Value};
use std::fs;

const ROOT: &str = "research/four_followups_20261021/results/matched";
const ARMS: [(u64, u64); 5] = [(1, 16), (2, 16), (4, 16), (1, 8), (1, 32)];
const BITS: [u64; 7] = [16, 20, 24, 28, 32, 36, 40];

fn logical(row: &Value) -> f64 {
    row.as_object()
        .unwrap()
        .iter()
        .filter_map(|(key, value)| {
            key.ends_with("_uncharged")
                .then(|| value.as_u64().unwrap() as f64)
        })
        .sum()
}
fn gae(row: &Value) -> f64 {
    let c = &row["counts"];
    let u = &row["source_add"];
    let arithmetic = c["m"].as_u64().unwrap() as f64
        + 0.5 * c["s"].as_u64().unwrap() as f64
        + 10.0 * c["i"].as_u64().unwrap() as f64;
    let denominator = u["m"].as_u64().unwrap() as f64
        + 0.5 * u["s"].as_u64().unwrap() as f64
        + 10.0 * u["i"].as_u64().unwrap() as f64;
    assert!(denominator > 0.0);
    arithmetic / denominator
        + 0.1
            * (c["field_adds"].as_u64().unwrap() as f64
                + c["bit_ops"].as_u64().unwrap() as f64
                + logical(&row["native"]))
}
fn near(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-11 * a.abs().max(b.abs()).max(1.0)
}
fn main() {
    let summary: Value =
        serde_json::from_str(&fs::read_to_string(format!("{ROOT}/summary.json")).unwrap()).unwrap();
    assert_eq!(
        summary["schema"],
        "four-followups-factor-base-matched-analysis/v1"
    );
    let mut raw_data =
        vec![json!({"path":"../../research/four_followups_20261021/results/matched/summary.json"})];
    let mut headline = Vec::new();
    let mut arm_reports = Vec::new();
    for (d, j) in ARMS {
        let arm = format!("fb{d}-j{j}");
        let mut candidate = 0.0;
        let mut rho = 0.0;
        let mut processed = 0u64;
        let mut count = 0;
        for bits in BITS {
            let relative = format!("{ROOT}/{arm}-bits{bits}.jsonl");
            let input = fs::read_to_string(&relative).unwrap();
            raw_data.push(json!({"path":format!("../../{relative}")}));
            let mut size_count = 0;
            for line in input.lines() {
                let row: Value = serde_json::from_str(line).unwrap();
                assert_eq!(row["bits"], bits);
                assert_eq!(row["verified"], true);
                assert_eq!(row["rho"]["verified"], true);
                assert_eq!(row["exhausted"], false);
                assert_eq!(row["rho"]["exhausted"], false);
                assert_eq!(row["recovered"], row["planted"]);
                assert_eq!(row["rho"]["recovered"], row["planted"]);
                candidate += gae(&row);
                rho += gae(&row["rho"]);
                processed += row["native"]["processed_divisors"].as_u64().unwrap();
                count += 1;
                size_count += 1;
            }
            assert_eq!(size_count, 180);
        }
        assert_eq!(count, 1260);
        let claimed = summary["summary"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["arm"] == arm && r["slice"] == "all")
            .unwrap();
        let claimed_candidate = claimed["representative"]["candidate_GAE"].as_f64().unwrap();
        let claimed_rho = claimed["representative"]["rho_GAE"].as_f64().unwrap();
        let claimed_ratio = claimed["representative"]["candidate_over_rho"]
            .as_f64()
            .unwrap();
        assert!(near(candidate, claimed_candidate));
        assert!(near(rho, claimed_rho));
        assert!(near(candidate / rho, claimed_ratio));
        assert_eq!(processed, claimed["processed_divisors"].as_u64().unwrap());
        for (name, claim, recomputed) in [
            (format!("{arm}_candidate_GAE"), claimed_candidate, candidate),
            (format!("{arm}_rho_GAE"), claimed_rho, rho),
            (
                format!("{arm}_candidate_over_rho"),
                claimed_ratio,
                candidate / rho,
            ),
        ] {
            headline
                .push(json!({"name":name,"claimed":claim,"recomputed":recomputed,"rel_tol":1e-11}));
        }
        arm_reports.push(json!({"arm":arm,"targets":count,"source_verified":count,
            "candidate_GAE":candidate,"rho_GAE":rho,"candidate_over_rho":candidate/rho,
            "processed_divisors":processed}));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schema":"four-followups-independent-validator/v1","status":"pass",
            "independent":true,"method":"separate native binary; direct raw-row recomputation",
            "raw_data":raw_data,"headline":headline,"arms":arm_reports
        }))
        .unwrap()
    );
}
