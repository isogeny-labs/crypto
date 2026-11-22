//! Universal weighted-slot formal signature quotient on the original Prime31 curve.
//!
//! Counts *integer coefficient vectors*, which may still collide after mapping
//! into E(Fp); this is an upper bound on actual EC support, not an extractor.
//! Exact precommitted protocol: PRIME31_M6_FORMAL_COLLISION_PROTOCOL_20261118.md
//!
//! cargo test --locked --release --manifest-path
//!   research/prime31_ic_20261115/native/Cargo.toml --example prime31_m6_formal_quotient
//! cargo run --locked --release --manifest-path
//!   research/prime31_ic_20261115/native/Cargo.toml --example prime31_m6_formal_quotient
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::HashSet;

const M: usize = 6;
const HISTORICAL: [u64; M] = [1, 3, 5, 7, 11, 13];
const BINARY: [u64; M] = [1, 2, 4, 8, 16, 32];
const ODD_SUPERINCREASING: [u64; M] = [1, 3, 7, 15, 31, 63];
const N: u128 = 2_147_446_831;

#[derive(Debug)]
struct Patterns {
    signatures: Vec<HashSet<Vec<u64>>>,
    partitions: [u64; M + 1],
}

fn visit_partitions(
    depth: usize,
    blocks: usize,
    labels: &mut [usize; M],
    weights: &[u64; M],
    patterns: &mut Patterns,
) {
    if depth == M {
        let mut block_sums = vec![0u64; blocks];
        for slot in 0..M {
            block_sums[labels[slot]] += weights[slot];
        }
        block_sums.sort_unstable();
        patterns.partitions[blocks] += 1;
        patterns.signatures[blocks].insert(block_sums);
        return;
    }
    // Restricted growth strings enumerate each set partition exactly once.
    // labels[0] = 0. Later labels may join an old block or start one new block.
    for label in 0..=blocks {
        labels[depth] = label;
        visit_partitions(
            depth + 1,
            blocks.max(label + 1),
            labels,
            weights,
            patterns,
        );
    }
}

fn patterns(weights: &[u64; M]) -> Patterns {
    let mut out = Patterns {
        signatures: (0..=M).map(|_| HashSet::new()).collect(),
        partitions: [0; M + 1],
    };
    visit_partitions(0, 0, &mut [0; M], weights, &mut out);
    assert_eq!(out.partitions.iter().sum::<u64>(), 203);
    out
}

fn factorial(n: usize) -> u128 {
    (1..=n).fold(1u128, |a, b| a * b as u128)
}

fn signature_count(b: usize, patterns: &Patterns) -> (u128, Vec<u128>) {
    let mut by_support_size = vec![0u128; M + 1];
    for k in 1..=M.min(b) {
        let falling = (0..k).fold(1u128, |acc, j| acc * (b - j) as u128);
        for pattern in &patterns.signatures[k] {
            let mut denom = 1u128;
            let mut i = 0;
            while i < k {
                let mut j = i + 1;
                while j < k && pattern[j] == pattern[i] {
                    j += 1;
                }
                denom *= factorial(j - i);
                i = j;
            }
            assert_eq!(falling % denom, 0);
            by_support_size[k] += falling / denom;
        }
    }
    (by_support_size.iter().sum(), by_support_size)
}

fn subset_sums_distinct(weights: &[u64; M]) -> bool {
    let mut sums = HashSet::new();
    for mask in 0..(1usize << M) {
        let sum: u64 = (0..M)
            .filter(|&i| mask & (1 << i) != 0)
            .map(|i| weights[i])
            .sum();
        if !sums.insert(sum) {
            return false;
        }
    }
    true
}

fn brute_coefficient_vectors(b: usize, weights: &[u64; M]) -> usize {
    let total = b.pow(M as u32);
    let mut seen = HashSet::with_capacity(total);
    // Deliberately enumerates only B<=8 as an independent exact combinatorial
    // test; this must never be repurposed as an ECDLP relation oracle.
    assert!((2..=8).contains(&b));
    for mut word in 0..total {
        let mut signature = vec![0u64; b];
        for &weight in weights {
            let index = word % b;
            word /= b;
            signature[index] += weight;
        }
        seen.insert(signature);
    }
    seen.len()
}

fn first_rational_points(ar: &mut Arithmetic) -> Vec<Point> {
    let mut base = Vec::new();
    for x in 0..PRIME31.p {
        let rhs = ar.rhs(PRIME31, x);
        if let Some(y) = ar.sqrt(rhs, PRIME31.p) {
            assert_ne!(y, 0);
            base.push(Some((x, y.min(PRIME31.p - y))));
            if base.len() == 31 {
                break;
            }
        }
    }
    assert_eq!(base.len(), 31);
    assert_eq!(base[0].unwrap().0, 0);
    assert_eq!(base[30].unwrap().0, 66);
    base
}

fn sum_word(ar: &mut Arithmetic, base: &[Point], indices: [usize; M]) -> Point {
    let mut out = None;
    for (slot, &index) in indices.iter().enumerate() {
        let term = ar.scalar(PRIME31, HISTORICAL[slot], base[index]);
        out = ar.add(PRIME31, out, term);
    }
    out
}

fn study() -> Value {
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("original ordinary cofactor-one Prime31 certificate");
    assert_eq!(PRIME31.p, 2_147_483_647);
    assert_eq!(PRIME31.n as u128, N);
    let base = first_rational_points(&mut ar);

    // A universal collision, before any special algebraic structure of E:
    // 1+7 = 3+5 = 8. Exchange the two basis points between these equal-sum
    // groups while leaving slots 11 and 13 at the same point.
    let word_a = [0, 1, 1, 0, 2, 3];
    let word_b = [1, 0, 0, 1, 2, 3];
    let pa = sum_word(&mut ar, &base, word_a);
    let pb = sum_word(&mut ar, &base, word_b);
    assert_eq!(pa, pb);
    assert!(pa.is_some());

    let mut results = Vec::new();
    for (name, weights) in [
        ("historical", HISTORICAL),
        ("binary_unique_subset_sums", BINARY),
        ("odd_unique_subset_sums", ODD_SUPERINCREASING),
    ] {
        let pats = patterns(&weights);
        let mut tests = Vec::new();
        for b in [2usize, 3, 4, 8, 16, 31] {
            let (exact, by_size) = signature_count(b, &pats);
            assert!(exact <= (b as u128).pow(M as u32));
            if b <= 8 {
                assert_eq!(exact, brute_coefficient_vectors(b, &weights) as u128);
            }
            tests.push(json!({"B": b, "formal_signatures": exact,
                              "ordered_words": (b as u128).pow(M as u32),
                              "by_support_size_k_0_through_6":by_size}));
            if b == 31 {
                if name == "historical" {
                    assert_eq!(exact, 884_350_051);
                } else {
                    assert_eq!(exact, 887_503_681);
                }
            }
        }
        let (b31, _) = signature_count(31, &pats);
        let distinct: Vec<_> = pats.signatures.iter().map(HashSet::len).collect();
        results.push(json!({
            "name":name,
            "weights":weights,
            "raw_partitions_by_k_0_through_6":pats.partitions,
            "distinct_block_sum_patterns_by_k_0_through_6":distinct,
            "subset_sums_distinct":subset_sums_distinct(&weights),
            "formal_B31_support_upper":b31,
            "uniform_point_hit_upper_numerator":b31,
            "uniform_point_hit_upper_denominator":N,
            "B_controls":tests
        }));
    }
    assert!(!subset_sums_distinct(&HISTORICAL));
    assert!(subset_sums_distinct(&BINARY));
    assert!(subset_sums_distinct(&ODD_SUPERINCREASING));
    json!({
        "scope":"exact integer formal-signature quotient, not actual EC support or ECDLP timing",
        "curve":{"p":PRIME31.p,"a":PRIME31.a,"b":PRIME31.b,"n":PRIME31.n},
        "factor_base":"original first 31 liftable x, canonical y",
        "planted_weight_equality":"1+7=3+5",
        "planted_words":[word_a,word_b],
        "planted_native_original_curve_sum":{"x":pa.unwrap().0,"y":pa.unwrap().1},
        "results":results,
        "notes":[
            "Actual elliptic-curve sums can collide even if formal vectors are different.",
            "The unsigned 31-bit ceiling does not measure a complete solver or a nonenumerative oracle.",
            "Changing only weights cannot increase the six-slot uniform hit bound above 31^6/n."
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
    fn b31_weighted_collision_quotient_and_curve_replay() {
        let v = study();
        assert_eq!(v["results"][0]["formal_B31_support_upper"], 884_350_051_u64);
        assert_eq!(v["results"][1]["formal_B31_support_upper"], 887_503_681_u64);
        assert_eq!(v["results"][2]["formal_B31_support_upper"], 887_503_681_u64);
    }
    #[test]
    fn independent_small_base_word_enumeration() {
        for w in [HISTORICAL, BINARY, ODD_SUPERINCREASING] {
            let ps = patterns(&w);
            for b in [2, 3, 4, 8] {
                assert_eq!(signature_count(b, &ps).0, brute_coefficient_vectors(b, &w) as u128);
            }
        }
    }
}
