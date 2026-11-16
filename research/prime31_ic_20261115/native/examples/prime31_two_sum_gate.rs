//! Deterministic prime31 two-summand yield preflight using the *existing* Rust
//! curve arithmetic and Dickson factor-base builder (not the historical C++).
//!
//! Pure correctness/coverage diagnostic. It is NOT an index-calculus cost
//! benchmark, a solver replacement, or independently signed verification.
//! Reproduce:
//! cargo run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_two_sum_gate
use prime31_algebraic_ic::{check_curve, Arithmetic, Base, Point, PRIME31};
use serde_json::json;
use std::collections::HashSet;

fn distinct_signed_sums(base: &Base, ar: &mut Arithmetic) -> usize {
    let mut seen = HashSet::<Point>::new();
    // Every point P_i-P_i gives the identity.
    seen.insert(None);
    for (i, &u) in base.points.iter().enumerate() {
        let twice = ar.add(PRIME31, Some(u), Some(u));
        seen.insert(twice);
        seen.insert(ar.neg(PRIME31, twice));
        for &v in &base.points[i + 1..] {
            let sum = ar.add(PRIME31, Some(u), Some(v));
            let opposite = ar.neg(PRIME31, Some(v));
            let difference = ar.add(PRIME31, Some(u), opposite);
            seen.insert(sum);
            seen.insert(ar.neg(PRIME31, sum));
            seen.insert(difference);
            seen.insert(ar.neg(PRIME31, difference));
        }
    }
    seen.len()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut certificate_ar = Arithmetic::default();
    check_curve(PRIME31, &mut certificate_ar)?;
    let expected = [
        // depth, Dickson terminal, liftable B, exact unique signed two-sums
        (2_usize, 47_u64, 2_usize, 9_usize),
        (8, 117_813_225, 135, 36_451),
        (9, 375_193_997, 256, 131_067),
        (10, 1_616_683_971, 509, 518_085),
    ];
    let mut rows = Vec::new();
    for (depth, terminal, expected_b, expected_support) in expected {
        let mut ar = Arithmetic::default();
        let base = Base::build(PRIME31, depth, &mut ar)?;
        assert_eq!(base.depth, depth);
        assert_eq!(base.terminal, terminal, "unexpected first selected fibre");
        assert_eq!(base.values.len(), 1_usize << depth);
        assert_eq!(base.points.len(), expected_b, "changed liftable base");
        let setup_counts = ar.counts;
        let s = distinct_signed_sums(&base, &mut ar);
        assert_eq!(s, expected_support, "signed-support replay discrepancy");
        let b = base.points.len() as u128;
        let n = PRIME31.n as u128;
        // Four signed choices for each unordered distinct pair; for a
        // repeated index, ++, --, +- are the three distinct relation rows.
        let relation_templates = 2 * b * b + b;
        assert!((s as u128) <= 2 * b * b + 1);
        rows.push(json!({
            "depth":depth,
            "terminal":terminal,
            "full_x_fibre":base.values.len(),
            "actual_liftable_B":base.points.len(),
            "signed_support_size":s,
            "canonical_relation_templates":relation_templates,
            "exact_nonempty_uniform_target_probability":format!("{s}/{}",PRIME31.n),
            "expected_uniform_calls_to_first_positive":format!("{}/{s}",PRIME31.n),
            "expected_templates_per_uniform_query":format!("{relation_templates}/{n}"),
            "full_B_plus_1_rank_query_floor_only":format!("{}/{relation_templates}",(b+1)*n),
            "base_setup_arithmetic":setup_counts,
            "total_diagnostic_arithmetic":ar.counts,
        }));
    }
    println!("{}", serde_json::to_string_pretty(&json!({
        "standing":"[pcc:unverified]",
        "p":PRIME31.p,
        "curve":[PRIME31.a,PRIME31.b],
        "n":PRIME31.n,
        "status":"deterministic Rust arithmetic preflight, not a cold solver benchmark",
        "note":"full-rank query floor assumes iid uniform R and full B+1 row rank; early scalar recovery from cycles is not excluded",
        "rows":rows
    }))?);
    Ok(())
}
