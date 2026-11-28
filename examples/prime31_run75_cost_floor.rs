//! Run75: reproduce the architecture-specific cold feasibility floor for
//! original-Rust Run72 K=2/3/4 authentic-RHS Prime31 ECDLP controls.
//!
//! Reads frozen source-linked Run72 Rust measurements. It does NOT run a
//! discrete logarithm or claim a new benchmark. No secret DLP input.
//! cargo test --locked --release --example prime31_run75_cost_floor
//! cargo run --locked --release --example prime31_run75_cost_floor

use serde_json::Value;

const DATA: &str =
    include_str!("../research/prime31_f4_admission_20261121/RUN72_RESULTS.json");

#[derive(Clone, Copy, Debug)]
struct Floor {
    width: u64,
    cold_ms: f64,
    matched_bsgs_ms: f64,
    fixed_pre_ms: f64,
    relation_ms: f64,
    // Required pre-stage reduction if all remaining relation work vanished.
    min_pre_cut: f64,
    ratio: f64,
}

fn get_ms(v: &Value, key: &str) -> f64 {
    v.get(key)
        .unwrap_or_else(|| panic!("missing field {key}"))
        .as_f64()
        .unwrap_or_else(|| panic!("non-numeric field {key}"))
}

fn rows() -> Vec<(String, Floor)> {
    let j: Value = serde_json::from_str(DATA).expect("frozen Run72 JSON");
    assert_eq!(j["curve"]["p"].as_u64(), Some(2_147_483_647));
    assert_eq!(j["curve"]["n"].as_u64(), Some(2_147_446_831));
    assert_eq!(j["setup"]["generic_pair_keys"].as_u64(), Some(916_469));
    assert_eq!(j["setup"]["targets_per_cohort"].as_u64(), Some(32));

    let records = j["records"].as_array().expect("run72 records");
    assert_eq!(records.len(), 9, "three widths x three seeds");

    let mut result = Vec::new();
    for r in records {
        let width = r["width"].as_u64().expect("width");
        assert!((2..=4).contains(&width));
        assert_eq!(r["cold"]["verified"].as_u64(), Some(32));
        assert_eq!(r["pass"].as_bool(), Some(true));

        let cold = &r["cold"];
        let bsgs = &r["bsgs"];
        let pre = get_ms(cold, "setup_ms") + get_ms(cold, "witness_ms");
        let shared_bsgs = get_ms(bsgs, "setup_ms") + get_ms(bsgs, "online_32_ms");
        let total = get_ms(cold, "cold_total_ms");
        let relation = get_ms(cold, "cold_relation_ms");
        assert!(pre.is_finite() && shared_bsgs.is_finite() && relation.is_finite());
        assert!(total > pre, "fully charged total must include non-pre work");
        assert!(pre > shared_bsgs,
                "this frozen cohort must have a strictly uncompetitive pre-stage");

        result.push((
            r["seed"].as_str().expect("seed").to_owned(),
            Floor {
                width,
                cold_ms: total,
                matched_bsgs_ms: shared_bsgs,
                fixed_pre_ms: pre,
                relation_ms: relation,
                min_pre_cut: 1.0 - shared_bsgs / pre,
                ratio: total / shared_bsgs,
            },
        ));
    }
    result
}

fn main() {
    println!("RUN75: source-frozen Run72 cost sensitivity, NOT a new ECDLP solve");
    println!("Prime31 ordinary cofactor-one, n=2147446831, B=677, T=32");
    for (seed, v) in rows() {
        println!(
            "seed={seed} K={} cold_ms={:.3} shared_bsgs_ms={:.3} ratio={:.3}              setup_plus_witness_ms={:.3} relation_ms={:.3}              pre_over_bsgs={:.3} minimum_pre_cut_if_free_relation_pct={:.2}",
            v.width,
            v.cold_ms,
            v.matched_bsgs_ms,
            v.ratio,
            v.fixed_pre_ms,
            v.relation_ms,
            v.fixed_pre_ms / v.matched_bsgs_ms,
            100.0 * v.min_pre_cut,
        );
    }
    println!("Fixed Run72 pre-stage alone exceeds matched shared BSGS in all nine cells.");
    println!("No speedup claim. The GENERIC 916469-entry pair dictionary disqualifies algorithmic novelty.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_original_rust_cohort_fails_the_fixed_pre_floor() {
        let data = rows();
        assert_eq!(data.len(), 9);
        for (_, x) in &data {
            assert!(x.fixed_pre_ms > x.matched_bsgs_ms);
            assert!(x.min_pre_cut > 0.4);
            assert!(x.ratio > 4.0);
            assert!(x.relation_ms > 0.0);
        }
        assert_eq!(data.iter().filter(|(_, x)| x.width == 4).count(), 3);
    }
}
