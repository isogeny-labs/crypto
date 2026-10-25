//! Native cost accounting for the published Tian arithmetic gate, not DLP.
use serde_json::{json, Value};
use std::fs;

const BASE: &str = "research/five_leads_e2e_20261019";
const FIELDS: [&str; 3] = ["m", "s", "i"];

fn counts(v: &Value) -> [u64; 3] {
    FIELDS.map(|key| v[key].as_u64().expect("field count"))
}
fn delta(a: [u64; 3], b: [u64; 3]) -> [u64; 3] {
    std::array::from_fn(|j| b[j].checked_sub(a[j]).expect("monotone counter"))
}
fn price(c: [u64; 3], s: f64, i: f64) -> f64 {
    c[0] as f64 + s * c[1] as f64 + i * c[2] as f64
}
fn main() {
    let km: Value = serde_json::from_slice(
        &fs::read(format!("{BASE}/correctness/tian-quartic-km-v2.json")).unwrap(),
    )
    .unwrap();
    let rho: Vec<Value> = fs::read_to_string(format!("{BASE}/correctness/tian-cubic-rho-v2.jsonl"))
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(km["schema"], "tian-km-gate/v2");
    assert_eq!(km["t_equals_1987_s"], true);
    assert_eq!(km["r_s_is_zero"], true);
    assert_eq!(km["group_law_controls"], true);
    assert_eq!(rho.len(), 5);
    assert!(rho
        .iter()
        .all(|v| v["verified"] == true && v["recovered"] == 1987));
    let shot = &km["snapshots"];
    let arms = [
        (
            "cold_geometry",
            delta([0; 3], counts(&shot["after_geometry"])),
        ),
        (
            "published_classes",
            delta(
                counts(&shot["after_geometry"]),
                counts(&shot["after_classes"]),
            ),
        ),
        (
            "one_jacobian_double",
            delta(
                counts(&shot["before_double"]),
                counts(&shot["after_double"]),
            ),
        ),
        (
            "known_scalar_1987_multiplication",
            delta(
                counts(&shot["before_scalar"]),
                counts(&shot["after_scalar"]),
            ),
        ),
    ];
    let rho_counts: Vec<_> = rho.iter().map(|v| counts(&v["counts"])).collect();
    let mut csv =
        String::from("S_per_M,I_per_M,mean_source_rho_GAE,min_source_rho_GAE,max_source_rho_GAE");
    for (name, _) in &arms {
        csv.push_str(&format!(",{name}_GAE,{name}_over_mean_rho"));
    }
    csv.push('\n');
    let mut rows = Vec::new();
    for s in [0.0, 0.5, 1.0] {
        for i in [1.0, 10.0, 100.0, 1000.0] {
            let source: Vec<_> = rho_counts.iter().map(|&c| price(c, s, i)).collect();
            let mean = source.iter().sum::<f64>() / source.len() as f64;
            let mut row = json!({"S_per_M":s,"I_per_M":i,"mean_source_rho_GAE":mean,
                "min_source_rho_GAE":source.iter().cloned().fold(f64::INFINITY,f64::min),
                "max_source_rho_GAE":source.iter().cloned().fold(f64::NEG_INFINITY,f64::max)});
            csv.push_str(&format!(
                "{s},{i},{mean},{},{}",
                row["min_source_rho_GAE"], row["max_source_rho_GAE"]
            ));
            for (name, cost) in &arms {
                let priced = price(*cost, s, i);
                let ratio = priced / mean;
                row[format!("{name}_GAE")] = json!(priced);
                row[format!("{name}_over_mean_rho")] = json!(ratio);
                csv.push_str(&format!(",{priced},{ratio}"));
            }
            csv.push('\n');
            rows.push(row);
        }
    }
    let representative = rows
        .iter()
        .find(|v| v["S_per_M"].as_f64() == Some(0.5) && v["I_per_M"].as_f64() == Some(10.0))
        .unwrap();
    let band = |key: &str| -> Value {
        json!([
            rows.iter()
                .map(|v| v[key].as_f64().unwrap())
                .fold(f64::INFINITY, f64::min),
            rows.iter()
                .map(|v| v[key].as_f64().unwrap())
                .fold(f64::NEG_INFINITY, f64::max)
        ])
    };
    let mut arm_counts = json!({});
    for (name, c) in arms {
        arm_counts[name] = json!({"m":c[0],"s":c[1],"i":c[2]});
    }
    let summary = json!({"schema":"tian-km-arithmetic-gate-analysis/v2","published_fixture_only":true,
        "complete_transfer_dlp":false,"rho_runs":rho.len(),"rho_all_source_verified":true,
        "destination_relation_checked_using_published_scalar":true,"counts":arm_counts,"representative":representative,
        "one_double_ratio_band":band("one_jacobian_double_over_mean_rho"),
        "known_scalar_multiplication_ratio_band":band("known_scalar_1987_multiplication_over_mean_rho")});
    fs::create_dir_all(format!("{BASE}/results")).unwrap();
    fs::write(
        format!("{BASE}/results/tian-km-arithmetic-gate-price-grid.csv"),
        csv,
    )
    .unwrap();
    fs::write(
        format!("{BASE}/results/tian-km-arithmetic-gate-summary.json"),
        serde_json::to_vec_pretty(&summary).unwrap(),
    )
    .unwrap();
    println!("{}", serde_json::to_string_pretty(&summary).unwrap());
}
