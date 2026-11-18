//! Exact M6 uniform-residual membership upper bound on original Prime31.
//! This is a fixed-instance analytical / generic support diagnostic; not an
//! algebraic extractor or a complete cheaper ECDLP solver.
//!
//! cargo test --locked --release --manifest-path
//!   research/prime31_ic_20261115/native/Cargo.toml --example prime31_m6_yield_ceiling
//! cargo run --locked --release --manifest-path
//!   research/prime31_ic_20261115/native/Cargo.toml --example prime31_m6_yield_ceiling
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::HashSet;

const B: usize = 31;
const WEIGHTS: [u64; 6] = [1, 3, 5, 7, 11, 13];
const TRAIN_CALLS: u128 = 87;
const MIN_TRAIN_INDEPENDENT_ROWS: u128 = 31;
const ONLINE_CALLS: u128 = 3_143;
const ONLINE_SOLVES: u128 = 1_024;

fn first_rational_points(ar: &mut Arithmetic) -> Vec<Point> {
    let mut out = Vec::with_capacity(B);
    for x in 0..PRIME31.p {
        let rhs = ar.rhs(PRIME31, x);
        if let Some(y) = ar.sqrt(rhs, PRIME31.p) {
            assert_ne!(y, 0, "an odd-prime-order curve cannot have 2-torsion");
            out.push(Some((x, y.min(PRIME31.p - y))));
            if out.len() == B {
                break;
            }
        }
    }
    assert_eq!(out.len(), B);
    assert_eq!(out[0].unwrap().0, 0);
    assert_eq!(out[B - 1].unwrap().0, 66);
    out
}

// Diagnostic support census only; not part of a production relation oracle.
// Each of the 31^3 ordered half-words is evaluated on the existing Rust curve.
fn three_slot_support(ar: &mut Arithmetic, slots: &[Vec<Point>]) -> usize {
    assert_eq!(slots.len(), 3);
    assert!(slots.iter().all(|s| s.len() == B));
    let mut out = HashSet::with_capacity(B.pow(3));
    for &a in &slots[0] {
        for &b in &slots[1] {
            let ab = ar.add(PRIME31, a, b);
            for &c in &slots[2] {
                out.insert(ar.add(PRIME31, ab, c));
            }
        }
    }
    out.len()
}

fn ceil_ratio(num: u128, den: u128) -> u128 {
    assert!(den != 0);
    num / den + u128::from(num % den != 0)
}

// For R uniform in E(Fp), sum_R X(R) = B^6 exactly, including multiplicities.
// Therefore E[X(R)] = B^6/n, and Pr[X(R)>0] <= B^6/n.
// No independence among the six tuple images is assumed.
fn study() -> Value {
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("original ordinary Prime31 certificate");
    assert_eq!(PRIME31.p, 2_147_483_647);
    assert_eq!(PRIME31.n, 2_147_446_831);
    let base = first_rational_points(&mut ar);
    let slots: Vec<Vec<Point>> = WEIGHTS
        .iter()
        .map(|&w| {
            base.iter()
                .map(|&p| ar.scalar(PRIME31, w, p))
                .collect()
        })
        .collect();
    let before = ar.counts.group_additions;
    let left = three_slot_support(&mut ar, &slots[0..3]);
    let right = three_slot_support(&mut ar, &slots[3..6]);
    let half_support_group_additions = ar.counts.group_additions - before;

    let words = (B as u128).pow(6);
    let n = PRIME31.n as u128;
    assert_eq!(words, 887_503_681);
    assert_eq!(left, B.pow(3));
    assert_eq!(right, B.pow(3));
    assert!(words < n);
    // Even with a perfect witness oracle, positive uniform residuals
    // cannot occur with greater marginal probability than words/n.
    assert!(MIN_TRAIN_INDEPENDENT_ROWS * n <= TRAIN_CALLS * words);
    assert!(ONLINE_SOLVES * n <= ONLINE_CALLS * words);
    let training_min_calls_for_expected_rows =
        ceil_ratio(MIN_TRAIN_INDEPENDENT_ROWS * n, words);
    let online_min_calls_for_expected_hits = ceil_ratio(ONLINE_SOLVES * n, words);
    assert_eq!(training_min_calls_for_expected_rows, 76);
    assert_eq!(online_min_calls_for_expected_hits, 2_478);

    json!({
        "scope": "exact uniform-residual M6 fixed-base upper bound; no solver timing",
        "curve": {"p":PRIME31.p,"n":PRIME31.n,"a":PRIME31.a,"b":PRIME31.b},
        "weights": WEIGHTS,
        "factor_base_size": B,
        "ordered_six_slot_words": words,
        "maximum_uniform_positive_probability": {
            "numerator": words, "denominator": n
        },
        "three_slot_half_support": {"left_distinct":left,"right_distinct":right,
            "expected_words_each":B.pow(3),
            "generic_diagnostic_group_additions":half_support_group_additions},
        "archived_training": {"calls":TRAIN_CALLS,
            "minimum_independent_rows":MIN_TRAIN_INDEPENDENT_ROWS,
            "ideal_uniform_calls_for_expected_31_hits_ceil": training_min_calls_for_expected_rows,
            "max_yield_only_gain_ratio_numerator":TRAIN_CALLS * words,
            "max_yield_only_gain_ratio_denominator":MIN_TRAIN_INDEPENDENT_ROWS * n},
        "archived_online": {"calls":ONLINE_CALLS,"solved":ONLINE_SOLVES,
            "ideal_uniform_calls_for_expected_1024_hits_ceil":online_min_calls_for_expected_hits,
            "max_yield_only_gain_ratio_numerator":ONLINE_CALLS * words,
            "max_yield_only_gain_ratio_denominator":ONLINE_SOLVES * n},
        "caveats": [
            "Uniform independent query model is required for expected waiting-time lower bounds.",
            "A fixed 87 or 3143 pseudorandom-seed corpus is not constrained by the expectation in every execution.",
            "The rank-31 numerator is a lower bound on observed positive training trials, not a count of every positive.",
            "Adaptive target-conditioned queries, altered factor bases, additional slots, and genuinely algebraic extractors are outside this ceiling.",
            "The 3+3 support census is generic arithmetic used only for verification."
        ]
    })
}

fn main() {
    println!("{}", serde_json::to_string_pretty(&study()).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_prime31_m6_exact_uniform_throughput_ceiling() {
        let v = study();
        assert_eq!(v["ordered_six_slot_words"], 887_503_681_u64);
        assert_eq!(v["three_slot_half_support"]["left_distinct"], 29_791);
        assert_eq!(v["three_slot_half_support"]["right_distinct"], 29_791);
        assert_eq!(v["archived_training"]["ideal_uniform_calls_for_expected_31_hits_ceil"], 76);
        assert_eq!(v["archived_online"]["ideal_uniform_calls_for_expected_1024_hits_ceil"], 2478);
    }
    #[test]
    fn small_exact_count_markov_identity() {
        // The counting identity is purely finite-group combinatorics:
        // every ordered tuple has exactly one image in the group.
        for b in 1u128..=31 {
            let n = PRIME31.n as u128;
            let words = b.pow(6);
            let cardinality_ceiling = words.min(n);
            assert!(cardinality_ceiling <= words);
            assert!(words <= 887_503_681);
        }
    }
}
