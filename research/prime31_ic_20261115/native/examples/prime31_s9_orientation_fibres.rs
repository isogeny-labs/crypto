//! Prime31: exact S9 orientation-fibre arithmetic and exceptional infinity controls.
//! Protocol: S9_ORIENTATION_FIBRES_PROTOCOL_20261116.md (frozen before this file).
//! Pure combinatorics + existing public prime31 Rust curve arithmetic.
//! This is not f-from-Q extraction, a complete log solver, or a measured speedup.
//!
//! cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s9_orientation_fibres
//! cargo +1.100.0 run  --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s9_orientation_fibres

use prime31_algebraic_ic::{check_curve, Arithmetic, Base, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::HashSet;

const ARITY: usize = 8;

fn choose(n: usize, k: usize) -> u128 {
    if k > n { return 0; }
    let k = k.min(n - k);
    (1..=k).fold(1_u128, |v, j| v * (n - k + j) as u128 / j as u128)
}

// Coefficient of z^k in ((1+z)/(1-z))^B counts integer net
// vectors with exactly k non-cancelled signed point occurrences.
fn net_norm_counts(b: usize) -> [u128; ARITY + 1] {
    let mut coeff = [0_u128; ARITY + 1];
    coeff[0] = 1;
    for _ in 0..b {
        let mut next = [0_u128; ARITY + 1];
        for (degree, &value) in coeff.iter().enumerate() {
            next[degree] += value;
            for k in 1..=(ARITY - degree) {
                next[degree + k] += 2 * value;
            }
        }
        coeff = next;
    }
    coeff
}

// Independently count oriented x-multisets by the generating function
// prod_i(sum_{m>=0} (m+1) z^m) = (1-z)^(-2B).
fn orientation_word_count(b: usize) -> u128 {
    let mut coeff = [0_u128; ARITY + 1];
    coeff[0] = 1;
    for _ in 0..b {
        let mut next = [0_u128; ARITY + 1];
        for (degree, &value) in coeff.iter().enumerate() {
            for m in 0..=(ARITY - degree) {
                next[degree + m] += value * (m + 1) as u128;
            }
        }
        coeff = next;
    }
    coeff[ARITY]
}

fn combinatorial_row(b: usize) -> Value {
    let norms = net_norm_counts(b);
    let mut weighted_by_net = 0_u128;
    let mut net_count = 0_u128;
    let mut norm_rows = Vec::new();
    for k in (0..=ARITY).step_by(2) {
        let t = (ARITY - k) / 2;
        let f_per_net = choose(b + t - 1, t);
        weighted_by_net += norms[k] * f_per_net;
        net_count += norms[k];
        norm_rows.push(json!({
            "net_l1_norm": k,
            "net_signatures": norms[k],
            "different_x_polynomials_per_fixed_net_signature": f_per_net
        }));
    }
    let orientations_from_polynomial = orientation_word_count(b);
    let expected_orientations = choose(2 * b + ARITY - 1, ARITY);
    assert_eq!(weighted_by_net, expected_orientations, "net-side orientation double count");
    assert_eq!(orientations_from_polynomial, expected_orientations, "polynomial-side count");
    let polynomials = choose(b + ARITY - 1, ARITY);
    let expected_net = match b {
        4 => 2_241,
        8 => 187_137,
        31 => 5_654_193_417,
        _ => panic!("unspecified benchmark base"),
    };
    let expected_polys = match b {
        4 => 165,
        8 => 6_435,
        31 => 48_903_492,
        _ => unreachable!(),
    };
    let expected_oriented = match b {
        4 => 6_435,
        8 => 490_314,
        31 => 8_361_453_672,
        _ => unreachable!(),
    };
    assert_eq!(net_count, expected_net);
    assert_eq!(polynomials, expected_polys);
    assert_eq!(expected_orientations, expected_oriented);
    json!({
        "base_size": b,
        "net_integer_signatures": net_count,
        "x_only_degree_eight_polynomials": polynomials,
        "oriented_signed_token_multisets": expected_orientations,
        "orientation_count_two_independent_generating_functions_agree": true,
        "average_orientation_vectors_per_x_polynomial": expected_orientations as f64 / polynomials as f64,
        "l1_norm_fibres": norm_rows
    })
}

fn signed_point_sum(ar: &mut Arithmetic, points: &[(u64, u64)], mask: u16) -> Point {
    let mut q = None;
    for (i, &point) in points.iter().enumerate() {
        let leaf = if (mask >> i) & 1 == 0 {
            Some(point)
        } else {
            ar.neg(PRIME31, Some(point))
        };
        q = ar.add(PRIME31, q, leaf);
    }
    q
}

fn curve_orientation_controls() -> Value {
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("prime31 curve must validate");
    // Match the actual K961/S5 algebraic factor base, NOT unrelated first-x31.
    let base = Base::build(PRIME31, 6, &mut ar).expect("depth-six Dickson fibre");
    assert_eq!(base.points.len(), 31);
    let points: Vec<(u64, u64)> = base.points.iter().take(8).copied().collect();
    assert_eq!(points.len(), 8);
    assert_eq!(points.iter().map(|p| p.0).collect::<HashSet<_>>().len(), 8);
    let before = ar.counts.group_additions;
    let mut group_images = HashSet::<Point>::new();
    let mut first_target = None;
    let mut flipped_target = None;
    let mut squarefree_identity_sign_orientations = 0_usize;
    for mask in 0_u16..256 {
        let q = signed_point_sum(&mut ar, &points, mask);
        assert!(ar.on_curve(PRIME31, q));
        squarefree_identity_sign_orientations += usize::from(q.is_none());
        group_images.insert(q);
        if mask == 0 { first_target = q; }
        if mask == 1 { flipped_target = q; }
    }
    // D4-injectivity on this fixed Dickson31 base rules out any nonzero
    // eight-net returning the identity: all squarefree sign masks have norm 8.
    assert_eq!(squarefree_identity_sign_orientations, 0,
        "squarefree eight-leaf polynomial would contradict D4 injectivity");
    // The x polynomial is IDENTICAL in these two cases: eight distinct x_i.
    // The group targets are distinct since P_0 != -P_0 in a prime odd group.
    assert_ne!(first_target, flipped_target, "sign forgotten by x-only f");
    let squarefree_sign_adds = ar.counts.group_additions - before;
    let repeated_before = ar.counts.group_additions;
    let mut repeated_targets = HashSet::<Point>::new();
    let mut identity_count = 0;
    for plus in 0..=8 {
        let mut q = None;
        let negative = ar.neg(PRIME31, Some(points[0]));
        for _ in 0..plus {
            q = ar.add(PRIME31, q, Some(points[0]));
        }
        for _ in plus..8 {
            q = ar.add(PRIME31, q, negative);
        }
        assert!(ar.on_curve(PRIME31, q));
        if plus == 4 {
            assert_eq!(q, None, "four cancelling pairs sum to infinity");
        }
        if q.is_none() { identity_count += 1; }
        repeated_targets.insert(q);
    }
    assert_eq!(identity_count, 1);
    assert_eq!(repeated_targets.len(), 9, "distinct 2a-8 multiples of non-torsion point");
    json!({
        "base": "actual_dickson_depth6_31_rational_points",
        "selected_eight_x": points.iter().map(|p| p.0).collect::<Vec<_>>(),
        "one_fixed_squarefree_x_polynomial": "product_i(T-x_i), eight distinct selected x",
        "oriented_masks_evaluated": 256,
        "squarefree_identity_sign_orientations": squarefree_identity_sign_orientations,
        "distinct_group_targets_for_fixed_x_polynomial": group_images.len(),
        "first_two_sign_orientations_have_distinct_Q": first_target != flipped_target,
        "squarefree_orientation_group_additions": squarefree_sign_adds,
        "one_fixed_repeated_x_polynomial": "(T-x_0)^8",
        "repeated_x_orientation_net_choices": 9,
        "distinct_group_targets_for_repeated_x": repeated_targets.len(),
        "repeated_x_identity_cases": identity_count,
        "repeated_x_group_additions": ar.counts.group_additions - repeated_before,
        "all_public_targets_checked_on_original_prime31_curve": true
    })
}

fn main() {
    let rows: Vec<Value> = [4, 8, 31].into_iter().map(combinatorial_row).collect();
    let curve = curve_orientation_controls();
    println!("{}", json!({
        "schema": "prime31.s9.orientation_fibres.v1",
        "curve": PRIME31,
        "classification": "signed S9 root multiplicity and orientation diagnostic, NOT an f-from-Q solver",
        "combinatorics": rows,
        "prime31_curve_controls": curve,
        "complete_unknown_target_ecdlp_recoveries": 0,
        "measured_speedup": false,
        "pcc": "unverified"
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn b4_b8_b31_exact_double_count_and_golden_constants() {
        for b in [4, 8, 31] {
            let row = combinatorial_row(b);
            assert_eq!(row["orientation_count_two_independent_generating_functions_agree"], true);
            assert_eq!(row["l1_norm_fibres"].as_array().unwrap().len(), 5);
        }
        assert_eq!(choose(34, 4), 46_376);
        assert_eq!(choose(11, 4), 330);
        assert_eq!(choose(31, 1), 31);
    }

    #[test]
    fn real_curve_sign_and_repeated_root_infinity_controls() {
        let row = curve_orientation_controls();
        assert_eq!(row["oriented_masks_evaluated"], 256);
        assert_eq!(row["squarefree_identity_sign_orientations"], 0);
        assert_eq!(row["first_two_sign_orientations_have_distinct_Q"], true);
        assert_eq!(row["repeated_x_identity_cases"], 1);
        assert_eq!(row["distinct_group_targets_for_repeated_x"], 9);
    }
}
