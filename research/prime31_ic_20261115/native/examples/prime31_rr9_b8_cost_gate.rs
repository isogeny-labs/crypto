//! Exact, bounded B8 signed RR9 random-residual coverage/cost gate on original prime31.
//! This is a feasibility audit, not a relation extractor or a complete ECDLP solver.
//! Protocol: RR9_B8_COST_GATE_PROTOCOL_20261117.md.
//! Reuses immutable prime31_algebraic_ic native Arithmetic and curve parameters.
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::HashSet;

const B8_SIGNED: u128 = 256;
const HISTORICAL_BSGS_1024Q_MS: f64 = 170.113;

fn first_eight(ar: &mut Arithmetic) -> [(u64, u64); 8] {
    let mut pts = Vec::new();
    for x in 0..PRIME31.p {
        let rhs = ar.rhs(PRIME31, x);
        if let Some(y) = ar.sqrt(rhs, PRIME31.p) {
            assert_ne!(y, 0, "odd prime-order curve cannot have rational 2-torsion");
            pts.push((x, y.min(PRIME31.p - y)));
            if pts.len() == 8 { break; }
        }
    }
    let pts: [(u64, u64); 8] = pts.try_into().expect("eight rational abscissae");
    assert_eq!(pts.map(|p| p.0), [0, 1, 2, 3, 5, 6, 8, 9]);
    pts
}

fn exact_support(ar: &mut Arithmetic, base: &[(u64, u64); 8]) -> HashSet<Point> {
    let mut sums = HashSet::new();
    for mask in 0..256_u16 {
        let mut sum: Point = None;
        for (i, &p) in base.iter().enumerate() {
            let mut leaf: Point = Some(p);
            if (mask >> i) & 1 != 0 {
                leaf = ar.neg(PRIME31, leaf);
            }
            sum = ar.add(PRIME31, sum, leaf);
        }
        assert!(sums.insert(sum), "unexpected collision of B8 signed sums");
    }
    assert_eq!(sums.len(), B8_SIGNED as usize);
    sums
}

fn report() -> Value {
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("original ordinary cofactor-one curve");
    assert_eq!(PRIME31.p, 2_147_483_647);
    assert_eq!(PRIME31.n, 2_147_446_831);
    let base = first_eight(&mut ar);
    let points = exact_support(&mut ar, &base);
    let n = PRIME31.n as u128;
    let one_draws = n; // expected draws = numerator / 256
    let eight_draws = 8 * n;
    // ms -> ns: 1 ms = 1,000,000 ns.
    // This is a hypothetical budget ceiling, NOT observed algorithm performance.
    let budget_ns = HISTORICAL_BSGS_1024Q_MS * 1_000_000.0;
    json!({
        "schema": "prime31.rr9.b8-cost-gate.v1",
        "curve_p": PRIME31.p,
        "curve_n": PRIME31.n,
        "cofactor": 1,
        "first_eight_base_x": base.map(|p| p.0),
        "b8_sign_masks_enumerated": 256,
        "b8_distinct_group_sums": points.len(),
        "probability_uniform_residual_numerator": B8_SIGNED,
        "probability_uniform_residual_denominator": n,
        "single_success_expected_draws_numerator": one_draws,
        "eight_success_expected_draws_numerator": eight_draws,
        "expected_draws_denominator": B8_SIGNED,
        "uniformity_assumption": "independent uniform b in Z/nZ; R=Q+[b]G; G generates E(Fp)",
        "budget_source": "historical producer #4532 1024-Q fully charged shared BSGS",
        "budget_ms_historical_not_remeasured": HISTORICAL_BSGS_1024Q_MS,
        "hypothetical_single_success_max_ns_per_attempt": budget_ns * B8_SIGNED as f64 / n as f64,
        "hypothetical_eight_success_max_ns_per_attempt": budget_ns * B8_SIGNED as f64 / eight_draws as f64,
        "nonuniform_adaptive_or_changeable_factor_base_covered": false,
        "is_non_generic_extraction": false,
        "complete_unknown_prime31_logs": 0,
        "is_complete_ecdlp_benchmark": false,
        "pcc": "unverified"
    })
}

fn main() {
    println!("{}", serde_json::to_string_pretty(&report()).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_original_curve_b8_coverage_and_budget() {
        let r = report();
        assert_eq!(r["schema"], "prime31.rr9.b8-cost-gate.v1");
        assert_eq!(r["curve_p"], 2_147_483_647u64);
        assert_eq!(r["curve_n"], 2_147_446_831u64);
        assert_eq!(r["first_eight_base_x"], json!([0,1,2,3,5,6,8,9]));
        assert_eq!(r["b8_distinct_group_sums"], 256);
        assert_eq!(r["probability_uniform_residual_numerator"], json!(256u128));
        assert_eq!(r["probability_uniform_residual_denominator"], json!(2_147_446_831u128));
        assert_eq!(r["eight_success_expected_draws_numerator"], json!(17_179_574_648u128));
        let ns = r["hypothetical_eight_success_max_ns_per_attempt"].as_f64().unwrap();
        assert!(ns > 2.53 && ns < 2.54);
        assert_eq!(r["complete_unknown_prime31_logs"], 0);
        assert_eq!(r["is_complete_ecdlp_benchmark"], false);
    }
}
