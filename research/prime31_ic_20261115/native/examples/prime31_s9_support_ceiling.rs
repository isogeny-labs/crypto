//! Exact eight-leaf signed-support census on the existing ordinary prime31
//! curve. This is a *coverage/correctness oracle*, NOT an ECDLP solver,
//! nonenumerative S9 elimination, index-calculus speed claim, or rho benchmark.
//!
//! Reuses the native research crate's PRIME31 curve, Arithmetic and verifier.
//! Public factor base: first B rational x-coordinates, canonical smaller y.
//! Full signed-word enumeration is replaced by exact cancellation signatures:
//! d in Z^B with ||d||_1 <= m and ||d||_1 == m (mod 2).
//!
//! cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s9_support_ceiling
//! cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s9_support_ceiling
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::json;
use std::collections::HashSet;
use std::time::Instant;

const ARITY: usize = 8;

// Coefficient of z^k in ((1+z)/(1-z))^B counts signed, cancellation-free
// multiplicity vectors with sum of absolute coefficients equal to k.
// The allowable length-m signatures have k<=m and k==m (mod 2).
fn signature_upper_count(base: usize, arity: usize) -> u128 {
    let mut coefficients = vec![0_u128; arity + 1];
    coefficients[0] = 1;
    for _ in 0..base {
        let mut next = vec![0_u128; arity + 1];
        for (degree, &count) in coefficients.iter().enumerate() {
            next[degree] += count;
            for multiplicity in 1..=arity - degree {
                next[degree + multiplicity] += 2 * count;
            }
        }
        coefficients = next;
    }
    coefficients
        .iter()
        .enumerate()
        .filter(|(degree, _)| degree % 2 == arity % 2)
        .map(|(_, &count)| count)
        .sum()
}

fn choose(n: usize, k: usize) -> u128 {
    let k = k.min(n - k);
    (1..=k).fold(1_u128, |v, j| v * (n - k + j) as u128 / j as u128)
}

fn first_rational_points(ar: &mut Arithmetic, base: usize) -> Vec<(u64, u64)> {
    let mut points = Vec::with_capacity(base);
    for x in 0..PRIME31.p {
        let rhs = ar.rhs(PRIME31, x);
        if let Some(y) = ar.sqrt(rhs, PRIME31.p) {
            // A point of order two cannot occur on this odd prime-order curve.
            assert_ne!(y, 0);
            points.push((x, y.min(PRIME31.p - y)));
            if points.len() == base {
                return points;
            }
        }
    }
    panic!("insufficient rational abscissae on PRIME31");
}

fn scalar_tables(
    ar: &mut Arithmetic,
    points: &[(u64, u64)],
) -> (Vec<Vec<Point>>, Vec<Vec<Point>>) {
    let mut positive = Vec::with_capacity(points.len());
    let mut negative = Vec::with_capacity(points.len());
    for &point in points {
        let mut pos = vec![None; ARITY + 1];
        let mut neg = vec![None; ARITY + 1];
        for k in 1..=ARITY {
            pos[k] = ar.add(PRIME31, pos[k - 1], Some(point));
            neg[k] = ar.neg(PRIME31, pos[k]);
        }
        positive.push(pos);
        negative.push(neg);
    }
    (positive, negative)
}

// Enumerate exactly one group sum for each *net* signed multiplicity vector.
// A positive/negative pair on the same x cancels and merely pads the word.
// This quotient is for oracle census only; it is NOT an S9 root extractor.
fn enumerate(
    i: usize,
    remaining: usize,
    sum: Point,
    pos: &[Vec<Point>],
    neg: &[Vec<Point>],
    ar: &mut Arithmetic,
    support: &mut HashSet<Point>,
    signatures: &mut u128,
) {
    if i == pos.len() {
        if remaining % 2 == 0 {
            *signatures += 1;
            support.insert(sum);
        }
        return;
    }
    enumerate(i + 1, remaining, sum, pos, neg, ar, support, signatures);
    for multiplicity in 1..=remaining {
        let plus = ar.add(PRIME31, sum, pos[i][multiplicity]);
        enumerate(
            i + 1,
            remaining - multiplicity,
            plus,
            pos,
            neg,
            ar,
            support,
            signatures,
        );
        let minus = ar.add(PRIME31, sum, neg[i][multiplicity]);
        enumerate(
            i + 1,
            remaining - multiplicity,
            minus,
            pos,
            neg,
            ar,
            support,
            signatures,
        );
    }
}

fn census(base: usize, ar: &mut Arithmetic) -> serde_json::Value {
    let start = Instant::now();
    let points = first_rational_points(ar, base);
    let (pos, neg) = scalar_tables(ar, &points);
    let mut support = HashSet::<Point>::new();
    let mut signatures = 0_u128;
    let before = ar.counts.group_additions;
    enumerate(0, ARITY, None, &pos, &neg, ar, &mut support, &mut signatures);
    let bound = signature_upper_count(base, ARITY);
    assert_eq!(signatures, bound);
    let has_infinity = support.contains(&None);
    assert!(has_infinity, "even-length signed leaves must represent O");
    let nonidentity = support.len() - 1;
    assert!(nonidentity < PRIME31.n as usize);
    json!({
        "base_size":base,
        "factor_base_affine_points":points,
        "arity":ARITY,
        "unordered_abscissa_multisets":choose(base + ARITY - 1, ARITY) as u64,
        "exact_integer_signature_count":signatures as u64,
        "distinct_prime31_group_targets_including_identity":support.len(),
        "distinct_nonidentity_targets":nonidentity,
        "nonidentity_density_numerator":nonidentity,
        "nonidentity_density_denominator":PRIME31.n-1,
        "group_additions_after_base_setup": ar.counts.group_additions - before,
        "elapsed_ns_diagnostic_only":start.elapsed().as_nanos() as u64,
        "classification":"exact finite-curve signed-support oracle; not target-conditioned S9 elimination or a complete ECDLP solve"
    })
}

fn main() {
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("prime31 curve certificate");
    let rows = [4_usize, 8, 12, 16, 20, 24, 28, 31]
        .into_iter()
        .map(|b| {
            json!({
                "base_size":b,
                "unsigned_degree_8_multisets":choose(b + ARITY - 1, ARITY) as u64,
                "unconditional_group_support_ceiling":signature_upper_count(b, ARITY) as u64,
                "subgroup_order":PRIME31.n
            })
        })
        .collect::<Vec<_>>();
    let b4 = census(4, &mut ar);
    let b8 = census(8, &mut ar);
    println!("{}", json!({
        "schema":"prime31.s9.signed_support.v1",
        "curve":PRIME31,
        "source":"native Rust Arithmetic from prime31-algebraic-ic",
        "support_bound_method":"coefficient of ((1+z)/(1-z))^B summed over k=0,2,4,6,8",
        "ceiling_table":rows,
        "exact_finite_curve_census":[b4,b8],
        "pcc":"unverified",
        "full_ecdlp_solver_benchmark":false,
        "nonenumerative_root_extraction":false
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn brute_words(base: usize, remaining: usize, signature: &mut [i16], seen: &mut HashSet<Vec<i16>>) {
        if remaining == 0 {
            seen.insert(signature.to_vec());
            return;
        }
        for i in 0..base {
            signature[i] += 1;
            brute_words(base, remaining - 1, signature, seen);
            signature[i] -= 2;
            brute_words(base, remaining - 1, signature, seen);
            signature[i] += 1;
        }
    }

    #[test]
    fn coefficients_agree_with_brute_force_signed_words() {
        for base in 1..=3 {
            for arity in [2, 4, 6] {
                let mut seen = HashSet::new();
                brute_words(base, arity, &mut vec![0_i16; base], &mut seen);
                assert_eq!(seen.len() as u128, signature_upper_count(base, arity));
            }
        }
    }

    #[test]
    fn prime31_m8_exact_integer_ceiling_and_multisets() {
        assert_eq!(signature_upper_count(4, 8), 2_241);
        assert_eq!(signature_upper_count(8, 8), 187_137);
        assert_eq!(signature_upper_count(31, 8), 5_654_193_417);
        assert_eq!(choose(11, 8), 165);
        assert_eq!(choose(15, 8), 6_435);
        assert_eq!(choose(38, 8), 48_903_492);
    }

    #[test]
    fn b4_b8_exact_curve_support_matches_independent_reference() {
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).unwrap();
        let points = first_rational_points(&mut ar, 8);
        assert_eq!(points, vec![
            (0, 309_380_359), (1, 749_117_756),
            (2, 51_658_786), (3, 166_645_833),
            (5, 457_995_111), (6, 865_603_774),
            (8, 849_737_830), (9, 548_858_265)
        ]);
        let b4 = census(4, &mut ar);
        let b8 = census(8, &mut ar);
        assert_eq!(b4["distinct_prime31_group_targets_including_identity"].as_u64(), Some(2_241));
        assert_eq!(b8["distinct_prime31_group_targets_including_identity"].as_u64(), Some(187_137));
        // Injective map from net signed signatures to group elements on
        // these fixed B4/B8 sets. This is NOT uniqueness of raw eight-words.
    }
}
