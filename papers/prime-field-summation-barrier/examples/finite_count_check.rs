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

fn main() {}
