//! Exact signed eight-slot formal-support count on the existing Prime31 curve.
//! Protocol: PRIME31_W8_SIGNED_FORMAL_PROTOCOL_20261118.md
//! This does NOT enumerate EC sums or extract ECDLP relations.
//!
//! cargo test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_w8_signed_formal
//! cargo run  --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_w8_signed_formal
use prime31_algebraic_ic::{check_curve, Arithmetic, PRIME31};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

const M: usize = 8;
const W: [i64; M] = [1, 3, 5, 7, 11, 13, 17, 19];
const N: u128 = 2_147_446_831;

type SignatureMinBlocks = HashMap<Vec<i64>, usize>;

fn visit(
    i: usize,
    block_sums: &mut Vec<i64>,
    weights: &[i64; M],
    signatures: &mut SignatureMinBlocks,
    leaves: &mut u64,
) {
    if i == M {
        *leaves += 1;
        let mut sig: Vec<i64> = block_sums.iter().copied().filter(|&v| v != 0).collect();
        sig.sort_unstable();
        let h = block_sums.len();
        signatures.entry(sig).and_modify(|min_h| {
            *min_h = (*min_h).min(h);
        }).or_insert(h);
        return;
    }
    for j in 0..block_sums.len() {
        block_sums[j] += weights[i];
        visit(i + 1, block_sums, weights, signatures, leaves);
        block_sums[j] -= weights[i];
    }
    block_sums.push(weights[i]);
    visit(i + 1, block_sums, weights, signatures, leaves);
    block_sums.pop();
}

fn patterns(signed: bool) -> (SignatureMinBlocks, u64) {
    let mut result = SignatureMinBlocks::new();
    let mut leaves = 0;
    let cases = if signed { 1usize << M } else { 1 };
    for mask in 0..cases {
        let w: [i64; M] = std::array::from_fn(|i| {
            if mask & (1 << i) != 0 { -W[i] } else { W[i] }
        });
        visit(0, &mut Vec::new(), &w, &mut result, &mut leaves);
    }
    assert_eq!(leaves, 4140 * cases as u64);
    (result, leaves)
}

fn factorial(n: usize) -> u128 {
    (1..=n).fold(1u128, |a, v| a * v as u128)
}

fn count(b: usize, patterns: &SignatureMinBlocks) -> (u128, Vec<u128>) {
    let mut by_k = vec![0u128; M + 1];
    for (sig, &min_h) in patterns {
        if min_h > b || sig.len() > b {
            continue;
        }
        let k = sig.len();
        let falling = (0..k).fold(1u128, |a, j| a * (b - j) as u128);
        let mut denom = 1u128;
        let mut i = 0;
        while i < k {
            let mut j = i + 1;
            while j < k && sig[j] == sig[i] { j += 1; }
            denom *= factorial(j - i);
            i = j;
        }
        assert_eq!(falling % denom, 0);
        by_k[k] += falling / denom;
    }
    (by_k.iter().sum(), by_k)
}

fn brute_signed(b: usize) -> usize {
    assert!((1..=2).contains(&b));
    let mut seen = HashSet::<Vec<i64>>::new();
    for mut word in 0..b.pow(M as u32) {
        let mut ix = [0usize; M];
        for slot in 0..M {
            ix[slot] = word % b;
            word /= b;
        }
        for sign in 0..(1usize << M) {
            let mut coefficients = vec![0i64; b];
            for slot in 0..M {
                let weight = if sign & (1 << slot) == 0 { W[slot] } else { -W[slot] };
                coefficients[ix[slot]] += weight;
            }
            seen.insert(coefficients);
        }
    }
    seen.len()
}

fn original_curve_control() -> Vec<u64> {
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("unchanged Prime31 original curve");
    assert_eq!(PRIME31.n as u128, N);
    assert_eq!(PRIME31.p, 2_147_483_647);
    let mut pts = Vec::new();
    for x in 0..100u64 {
        if let Some(y) = ar.sqrt(ar.rhs(PRIME31, x), PRIME31.p) {
            assert_ne!(y, 0);
            pts.push((x, y.min(PRIME31.p - y)));
            if pts.len() == 31 { break; }
        }
    }
    assert_eq!(pts.len(), 31);
    assert_eq!(pts[0].0, 0);
    assert_eq!(pts[30].0, 66);
    // Distinct ordered words can have identical *signed* formal sums:
    // 1+7 = 3+5, with the remaining weights held at fixed public points.
    let words = [
        [0usize, 1, 1, 0, 2, 3, 4, 5],
        [1usize, 0, 0, 1, 2, 3, 4, 5]
    ];
    let mut sums = Vec::new();
    for word in words {
        let mut total = None;
        for j in 0..M {
            let pt = ar.scalar(PRIME31, W[j] as u64, Some(pts[word[j]]));
            total = ar.add(PRIME31, total, pt);
        }
        sums.push(total.expect("nonidentity planted group sum"));
    }
    assert_eq!(sums[0], sums[1]);
    vec![sums[0].0, sums[0].1]
}

fn study() -> Value {
    let point = original_curve_control();
    let (signed, signed_partitions) = patterns(true);
    let (positive, positive_partitions) = patterns(false);
    assert_eq!(signed_partitions, 1_059_840);
    assert_eq!(positive_partitions, 4_140);
    assert_eq!(signed.len(), 528_104);

    let mut b_controls = Vec::new();
    let mut previous_signed = 0u128;
    for b in [1usize, 2, 3, 4, 8, 12, 16, 31] {
        let (s, signed_by_k) = count(b, &signed);
        let (u, positive_by_k) = count(b, &positive);
        let signed_words = (1u128 << M) * (b as u128).pow(M as u32);
        let positive_words = (b as u128).pow(M as u32);
        assert!(s <= signed_words && u <= positive_words);
        assert!(s >= previous_signed);
        previous_signed = s;
        if b <= 2 { assert_eq!(s as usize, brute_signed(b)); }
        if b == 4 { assert_eq!(s, 5_491_889); assert_eq!(u, 35_428); }
        if b == 8 { assert_eq!(s, 3_240_125_537); assert_eq!(u, 14_233_920); }
        if b == 31 { assert_eq!(s, 214_117_016_830_955); assert_eq!(u, 843_295_566_676); }
        b_controls.push(json!({
            "B": b,
            "signed_formal": s,
            "positive_only_formal": u,
            "signed_words": signed_words,
            "positive_only_words": positive_words,
            "signed_by_k_0_through_8": signed_by_k,
            "positive_only_by_k_0_through_8": positive_by_k,
            "uniform_hit_ceiling_numerator": s.min(N),
            "uniform_hit_ceiling_denominator": N
        }));
    }
    json!({
        "scope": "exact signed integer coefficient-vector cardinalities, not ECDLP solver speed or actual elliptic-curve support",
        "curve": {"p": PRIME31.p, "a": PRIME31.a, "b": PRIME31.b, "n": PRIME31.n},
        "signed_weights": W,
        "original_curve_planted_sum": {"x": point[0], "y": point[1]},
        "signed_partition_leaves": signed_partitions,
        "positive_partition_leaves": positive_partitions,
        "signed_distinct_patterns": signed.len(),
        "positive_distinct_patterns": positive.len(),
        "b_controls": b_controls,
        "warning": "Signed RR9 has independent orientation choices; positive-only support is NOT a bound for signed extraction."
    })
}

fn main() {
    println!("{}", serde_json::to_string_pretty(&study()).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn signed_w8_exact_original_curve() {
        let r = study();
        assert_eq!(r["b_controls"][3]["signed_formal"], 5_491_889u64);
        assert_eq!(r["b_controls"][4]["signed_formal"], 3_240_125_537u64);
    }
    #[test]
    fn two_base_bruteforce_and_zero_cancellation() {
        let (signed, _) = patterns(true);
        assert_eq!(count(1, &signed).0 as usize, brute_signed(1));
        assert_eq!(count(2, &signed).0 as usize, brute_signed(2));
    }
}
