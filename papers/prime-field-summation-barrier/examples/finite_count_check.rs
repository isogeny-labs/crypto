//! Exact finite sanity checks for the manuscript's counting layer.
//!
//! This is deliberately not a performance benchmark and does not test an
//! asymptotic lower bound.  It exhaustively enumerates small cyclic-group
//! instances and checks the identities used before any algebraic-solving
//! model is introduced.

use std::collections::BTreeSet;

fn binomial(n: u128, k: u128) -> u128 {
    let k = k.min(n - k);
    (0..k).fold(1, |acc, i| acc * (n - i) / (i + 1))
}

fn enumerate_multisets(
    base: &[usize],
    modulus: usize,
    arity: usize,
    start: usize,
    depth: usize,
    sum: usize,
    sums: &mut BTreeSet<usize>,
    tuples: &mut u128,
) {
    if depth == arity {
        *tuples += 1;
        sums.insert(sum % modulus);
        return;
    }

    for index in start..base.len() {
        enumerate_multisets(
            base,
            modulus,
            arity,
            index,
            depth + 1,
            (sum + base[index]) % modulus,
            sums,
            tuples,
        );
    }
}

fn check_instance(modulus: usize, base_size: usize, arity: usize) {
    let base: Vec<_> = (1..=base_size).collect();
    let mut sums = BTreeSet::new();
    let mut tuples = 0;
    enumerate_multisets(
        &base,
        modulus,
        arity,
        0,
        0,
        0,
        &mut sums,
        &mut tuples,
    );

    let expected = binomial((base_size + arity - 1) as u128, arity as u128);
    assert_eq!(tuples, expected);
    assert!(sums.len() as u128 <= tuples);
    assert!(sums.len() <= modulus);

    // Counting with multiplicity is the finite kernel of the static-provenance
    // direct-sum theorem: summing fibre sizes over every target recovers the
    // number of stored candidate leaves exactly.
    let mut fibre_sizes = vec![0_u128; modulus];
    fn count_fibres(
        base: &[usize],
        modulus: usize,
        arity: usize,
        start: usize,
        depth: usize,
        sum: usize,
        fibres: &mut [u128],
    ) {
        if depth == arity {
            fibres[sum % modulus] += 1;
            return;
        }
        for index in start..base.len() {
            count_fibres(
                base,
                modulus,
                arity,
                index,
                depth + 1,
                (sum + base[index]) % modulus,
                fibres,
            );
        }
    }
    count_fibres(&base, modulus, arity, 0, 0, 0, &mut fibre_sizes);
    assert_eq!(fibre_sizes.iter().sum::<u128>(), tuples);

    // Exhausting every target makes the uniform-target identity exact:
    // the number of targets with a decomposition is precisely |im sigma_m|.
    let accepted_targets = (0..modulus).filter(|target| sums.contains(target)).count();
    assert_eq!(accepted_targets, sums.len());
}

#[test]
fn exact_uniform_target_and_multiset_bounds() {
    for modulus in [17, 31, 101] {
        for base_size in 2..=7 {
            for arity in 2..=5 {
                check_instance(modulus, base_size, arity);
            }
        }
    }
}

#[test]
fn audited_fixed_arity_exponents_exceed_one_half() {
    for arity in 2_u128..=32 {
        let numerator = arity;
        let denominator = 2 * arity - 1;
        assert!(2 * numerator > denominator);
    }

    assert_eq!((3_u128, 2 * 3 - 1), (3, 5));
    assert_eq!((4_u128, 2 * 4 - 1), (4, 7));
    assert_eq!((5_u128, 2 * 5 - 1), (5, 9));
}

fn implicit_ds_exponent(m: f64, s: f64, t: f64) -> f64 {
    if s - t >= 1.0 {
        s / (s + m - 1.0 - t)
    } else {
        (1.0 + t) / m
    }
}

#[test]
fn piecewise_implicit_data_structure_balance_matches_grid_search() {
    let cases = [
        (3.0, 1.0, 0.0),
        (3.0, 2.5, 0.5),
        (4.0, 1.0, 1.5),
        (4.0, 3.0, 0.5),
        (5.0, 1.25, 0.75),
        (5.0, 4.0, 1.0),
    ];

    for (m, s, t) in cases {
        assert!(t < m - 1.0);
        let predicted = implicit_ds_exponent(m, s, t);
        let mut observed = f64::INFINITY;
        for step in 1..=100_000 {
            let alpha = (step as f64) / (100_000.0 * m);
            let exponent = (s * alpha).max(1.0 - (m - 1.0 - t) * alpha);
            observed = observed.min(exponent);
        }
        assert!((predicted - observed).abs() < 1e-5);
    }
}

#[test]
fn audited_indexing_family_small_arity_fractions() {
    // (m, delta numerator, delta denominator, expected numerator, denominator)
    let cases = [
        (3_i128, 0_i128, 1_i128, 7_i128, 11_i128),
        (3, 1, 2, 2, 3),
        (3, 1, 1, 5, 7),
        (4, 0, 1, 3, 5),
        (4, 1, 2, 8, 13),
        (4, 1, 1, 7, 11),
        (5, 0, 1, 11, 19),
        (5, 1, 2, 10, 17),
        (5, 1, 1, 3, 5),
    ];

    for (m, delta_num, delta_den, expected_num, expected_den) in cases {
        let numerator = (2 * m + 1) * delta_den - 2 * delta_num;
        let denominator = (4 * m - 1) * delta_den - 4 * delta_num;
        assert_eq!(numerator * expected_den, expected_num * denominator);
        assert!(2 * numerator > denominator);
    }
}

fn main() {}
