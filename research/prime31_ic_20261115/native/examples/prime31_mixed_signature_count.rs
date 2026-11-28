//! Exact free-lattice count for mixed signed 4+4 nets with one common base
//! point; NOT curve-image coverage and NOT an ECDLP speed measurement.
//! Rust 1.100.0: cargo test --release --example prime31_mixed_signature_count
use serde_json::json;
use std::collections::HashSet;

const ARITY: usize = 4;
const B31: usize = 31;
const MIXED_EXPECTED: u128 = 381_434_284_809;
const PAIR_EXPECTED: u128 = 383_018_643_225;
const SAME_BASE_EXPECTED: u128 = 5_654_193_417;

fn choose(n: usize, k: usize) -> u128 {
    if k > n { return 0; }
    let k = k.min(n - k);
    (1..=k).fold(1_u128, |a, i| a * (n - k + i) as u128 / i as u128)
}

fn l1_exact(m: usize, k: usize) -> u128 {
    if k == 0 { return 1; }
    (1..=k.min(m))
        .map(|j| choose(m, j) * (1_u128 << j) * choose(k - 1, j - 1))
        .sum()
}

fn d4_size(m: usize) -> u128 {
    (0..=ARITY).filter(|k| k % 2 == 0).map(|k| l1_exact(m, k)).sum()
}

// Two bases have exactly one identified coordinate; all other coordinates
// are distinct, so alpha/beta label the exact non-shared L1 norms.
fn mixed_one_overlap(first_unique: usize, second_unique: usize) -> u128 {
    let mut count = 0_u128;
    for a in 0..=ARITY {
        for b in 0..=ARITY {
            count += l1_exact(first_unique, a) * l1_exact(second_unique, b)
                * (2 * ARITY + 1 - a - b) as u128;
        }
    }
    count
}

fn half_nets(coordinates: &[usize], total_dimensions: usize) -> HashSet<Vec<i8>> {
    let tokens = 2 * coordinates.len();
    let mut out = HashSet::new();
    for i in 0..tokens {
        for j in i..tokens {
            for k in j..tokens {
                for l in k..tokens {
                    let mut net = vec![0_i8; total_dimensions];
                    for token in [i, j, k, l] {
                        net[coordinates[token / 2]] += if token % 2 == 0 { 1 } else { -1 };
                    }
                    out.insert(net);
                }
            }
        }
    }
    out
}

fn brute_mixed(first: &[usize], second: &[usize], total_dimensions: usize) -> usize {
    let a = half_nets(first, total_dimensions);
    let b = half_nets(second, total_dimensions);
    let mut merged = HashSet::new();
    for x in &a {
        for y in &b {
            let v = x.iter().zip(y).map(|(x, y)| *x + *y).collect::<Vec<_>>();
            merged.insert(v);
        }
    }
    merged.len()
}

fn main() {
    let n30: Vec<u128> = (0..=ARITY).map(|k| l1_exact(30, k)).collect();
    let mixed = mixed_one_overlap(30, 30);
    let half = d4_size(B31);
    assert_eq!(n30, vec![1, 60, 1_800, 36_020, 541_200]);
    assert_eq!(half, 618_885);
    assert_eq!(half * half, PAIR_EXPECTED);
    assert_eq!(mixed, MIXED_EXPECTED);
    println!("{}", json!({
        "schema": "prime31.mixed_4p4_integer_signature_theorem.v1",
        "status": "exact_free_lattice_combinatorics_not_group_coverage",
        "unique_components_each_side": 30,
        "common_basis_points": 1,
        "union_dimension": 61,
        "l1_shell_counts_30": n30,
        "half_nets_per_base": half.to_string(),
        "mixed_distinct_formal_nets": mixed.to_string(),
        "homogeneous_distinct_formal_nets": SAME_BASE_EXPECTED.to_string(),
        "ordered_formal_pairs": PAIR_EXPECTED.to_string(),
        "mixed_formal_to_homogeneous_ratio": mixed as f64 / SAME_BASE_EXPECTED as f64,
        "mixed_formal_nets_per_prime31_group_element": mixed as f64 / 2_147_446_831_f64,
        "actual_group_coverage_or_ECDLP_speedup_verified": false,
        "pcc": "unverified"
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_prime31_31_plus_31_one_overlap() {
        assert_eq!((0..=4).map(|k| l1_exact(30, k)).collect::<Vec<_>>(),
            vec![1, 60, 1800, 36020, 541200]);
        assert_eq!(d4_size(31), 618_885);
        assert_eq!(d4_size(31) * d4_size(31), PAIR_EXPECTED);
        assert_eq!(mixed_one_overlap(30, 30), MIXED_EXPECTED);
        assert!(MIXED_EXPECTED < PAIR_EXPECTED);
        assert!(MIXED_EXPECTED > SAME_BASE_EXPECTED);
    }

    #[test]
    fn exact_brute_one_common_point_b2_b3_b4() {
        for b in 2..=4 {
            let a: Vec<usize> = (0..b).collect();
            let d: Vec<usize> = (b-1..2*b-1).collect();
            assert_eq!(brute_mixed(&a, &d, 2*b-1) as u128,
                mixed_one_overlap(b-1, b-1), "B{b} overlap-one");
        }
    }

    #[test]
    fn exact_brute_disjoint_b2_b3() {
        for b in 2..=3 {
            let a: Vec<usize> = (0..b).collect();
            let d: Vec<usize> = (b..2*b).collect();
            assert_eq!(brute_mixed(&a, &d, 2*b) as u128,
                d4_size(b) * d4_size(b), "B{b} disjoint");
        }
    }

    #[test]
    fn shared_coefficient_interval_count() {
        for a in 0..=4 {
            for b in 0..=4 {
                let first = (0..=4-a).map(|i| -(4_i32 - a as i32) + 2*i as i32);
                let second = (0..=4-b).map(|i| -(4_i32 - b as i32) + 2*i as i32)
                    .collect::<Vec<_>>();
                let sums = first.flat_map(|x| second.iter().map(move |y| x + y))
                    .collect::<HashSet<_>>();
                assert_eq!(sums.len(), 9-a-b);
            }
        }
    }
}
