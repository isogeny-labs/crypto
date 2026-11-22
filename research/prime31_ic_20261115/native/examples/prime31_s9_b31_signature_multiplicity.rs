//! Exact B31 eight-leaf *net-signature* multiplicity, not just generic 4+4 hits.
//! Reuses PRIME31 Arithmetic and both bases from the existing Rust research crate.
//! Signed generic meet-in-the-middle is a verification oracle, not an ECDLP attack.
//! Protocol: S9_B31_MULTIPLICITY_PROTOCOL_20261116.md [pcc:unverified].
//!
//! cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s9_b31_signature_multiplicity
//! cargo +1.100.0 run  --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s9_b31_signature_multiplicity

use prime31_algebraic_ic::{check_curve, Arithmetic, Base, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::time::Instant;

type Sig = [i8; 31];
const B: usize = 31;
const NATURAL_SEED: u64 = 0x5339_4233_3152_4f4f;
const PLANTED_SEED: u64 = 0x5339_4233_3150_524f;
const NATURAL_CASES: usize = 32;
const PLANTED_CASES: usize = 12;

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn signed_tokens(ar: &Arithmetic, base: &[(u64, u64)]) -> Vec<Point> {
    let mut out = Vec::with_capacity(2 * base.len());
    for &p in base {
        out.push(Some(p));
        out.push(ar.neg(PRIME31, Some(p)));
    }
    out
}

fn first_rational_points(ar: &mut Arithmetic) -> Vec<(u64, u64)> {
    let mut out = Vec::with_capacity(B);
    for x in 0..PRIME31.p {
        let rhs = ar.rhs(PRIME31, x);
        if let Some(y) = ar.sqrt(rhs, PRIME31.p) {
            assert_ne!(y, 0);
            out.push((x, y.min(PRIME31.p - y)));
            if out.len() == B { return out; }
        }
    }
    panic!("insufficient rational x");
}

fn word_signature(word: &[usize]) -> Sig {
    let mut d = [0_i8; B];
    for &t in word {
        d[t / 2] += if t % 2 == 0 { 1 } else { -1 };
    }
    d
}

fn sum_signatures(a: &Sig, b: &Sig) -> Sig {
    let mut d = [0_i8; B];
    for i in 0..B { d[i] = a[i] + b[i]; }
    d
}

fn net_replay(ar: &mut Arithmetic, pts: &[Point], d: &Sig) -> Point {
    let weight: usize = d.iter().map(|x| x.unsigned_abs() as usize).sum();
    assert!(weight <= 8 && weight % 2 == 0, "not an eight-leaf net signature");
    let mut q = None;
    for (i, &v) in d.iter().enumerate() {
        // Sig is always B31-wide, including small B4/B8 test bases. Skip
        // zero coefficients BEFORE indexing the shorter signed-point slice.
        if v == 0 { continue; }
        let leaf = pts[2 * i + usize::from(v < 0)];
        for _ in 0..v.unsigned_abs() {
            q = ar.add(PRIME31, q, leaf);
        }
    }
    q
}

fn binomial(n: usize, k: usize) -> u128 {
    if k > n { return 0; }
    let k = k.min(n - k);
    (1..=k).fold(1_u128, |v, i| v * (n - k + i) as u128 / i as u128)
}

fn integer_signature_count(b: usize, arity: usize) -> u128 {
    let mut poly = vec![0_u128; arity + 1];
    poly[0] = 1;
    for _ in 0..b {
        let mut next = vec![0_u128; arity + 1];
        for (i, &v) in poly.iter().enumerate() {
            next[i] += v;
            for k in 1..=(arity - i) { next[i + k] += 2 * v; }
        }
        poly = next;
    }
    poly.iter().enumerate().filter(|(i, _)| i % 2 == arity % 2).map(|(_, v)| v).sum()
}

struct Half {
    // A group point can have MULTIPLE distinct integer net signatures.
    // This cannot be replaced by a one-witness map when counting roots.
    by_point: HashMap<Point, Vec<Sig>>,
    ordered_points: Vec<Point>,
    integer_half_signatures: usize,
    word_multisets: u64,
    extra_group_collisions: usize,
    additions: u64,
    setup_ns: u128,
}

impl Half {
    fn build(ar: &mut Arithmetic, pts: &[Point]) -> Self {
        let clock = Instant::now();
        let before = ar.counts.group_additions;
        let n = pts.len();
        assert!(n <= 2 * B && n % 2 == 0);
        let mut pairs = vec![None; n * n];
        for i in 0..n {
            for j in i..n {
                let q = ar.add(PRIME31, pts[i], pts[j]);
                pairs[i * n + j] = q;
                pairs[j * n + i] = q;
            }
        }
        let mut seen = HashSet::<Sig>::with_capacity(integer_signature_count(n / 2, 4) as usize);
        let mut by_point: HashMap<Point, Vec<Sig>> = HashMap::new();
        let mut ordered_points = Vec::new();
        let mut words = 0_u64;
        for i in 0..n {
            for j in i..n {
                for k in j..n {
                    for l in k..n {
                        words += 1;
                        let d = word_signature(&[i, j, k, l]);
                        let q = ar.add(PRIME31, pairs[i * n + j], pairs[k * n + l]);
                        if seen.insert(d) {
                            match by_point.entry(q) {
                                Entry::Vacant(v) => {
                                    v.insert(vec![d]);
                                    ordered_points.push(q);
                                }
                                Entry::Occupied(mut o) => { o.get_mut().push(d); }
                            }
                        }
                    }
                }
            }
        }
        let integer_half_signatures = seen.len();
        assert_eq!(integer_half_signatures as u128, integer_signature_count(n / 2, 4));
        assert_eq!(words as u128, (n as u128 * (n + 1) as u128 * (n + 2) as u128 * (n + 3) as u128) / 24);
        let extra_group_collisions = integer_half_signatures - ordered_points.len();
        Self {
            by_point, ordered_points, integer_half_signatures, word_multisets: words,
            extra_group_collisions,
            additions: ar.counts.group_additions - before,
            setup_ns: clock.elapsed().as_nanos(),
        }
    }

    // Exhaustive. Each distinct d is replayed independently in E(Fp).
    // This receives public Q, not the generator scalar or planted witness.
    fn all_net_roots(&self, ar: &mut Arithmetic, pts: &[Point], q: Point)
        -> (Vec<Sig>, u64, u128, u64)
    {
        let before = ar.counts.group_additions;
        let t0 = Instant::now();
        let mut net_roots = HashSet::<Sig>::new();
        let mut ordered_pairs = 0_u64;
        for &left in &self.ordered_points {
            let missing = ar.add(PRIME31, q, ar.neg(PRIME31, left));
            if let Some(right_vectors) = self.by_point.get(&missing) {
                let left_vectors = self.by_point.get(&left).expect("index/list mismatch");
                for u in left_vectors {
                    for v in right_vectors {
                        ordered_pairs += 1;
                        net_roots.insert(sum_signatures(u, v));
                    }
                }
            }
        }
        let mut roots: Vec<Sig> = net_roots.into_iter().collect();
        roots.sort_unstable();
        for d in &roots {
            assert_eq!(net_replay(ar, pts, d), q, "original-curve net-signature replay");
        }
        let elapsed_ns = t0.elapsed().as_nanos();
        let additions = ar.counts.group_additions - before;
        (roots, ordered_pairs, elapsed_ns, additions)
    }
}

fn measure(ar: &mut Arithmetic, h: &Half, pts: &[Point], q: Point,
           category: &str, case: usize, verifier_k: Option<u64>) -> Value {
    let (net_roots, ordered_pairs, elapsed_ns, additions) = h.all_net_roots(ar, pts, q);
    json!({
        "type":category, "case":case, "public_point":q,
        "verifier_only_scalar_never_passed_to_oracle":verifier_k,
        "distinct_eight_net_signatures":net_roots.len(),
        "ordered_four_plus_four_pairs":ordered_pairs,
        "all_net_signatures":net_roots,
        "positive": !net_roots.is_empty(),
        "exhausted_four_point_image":h.ordered_points.len(),
        "original_curve_replay_for_every_net_signature":true,
        "query_ns_diagnostic_only":elapsed_ns.to_string(),
        "query_group_additions_including_replay":additions
    })
}

fn panel(ar: &mut Arithmetic, base: Vec<(u64, u64)>, label: &str,
         base_setup_ns: u128, base_setup_additions: u64) -> Value {
    assert_eq!(base.len(), B);
    let pts = signed_tokens(ar, &base);
    let h = Half::build(ar, &pts);
    let mut rows = Vec::new();
    let mut seed = NATURAL_SEED;
    let mut seen = HashSet::new();
    let mut natural_hits = 0_usize;
    for case in 0..NATURAL_CASES {
        let k = loop {
            let k = 1 + splitmix64(&mut seed) % (PRIME31.n - 1);
            if seen.insert(k) { break k; }
        };
        let q = ar.scalar(PRIME31, k, Some(PRIME31.g));
        let row = measure(ar, &h, &pts, q, "natural_verifier_only_k", case, Some(k));
        natural_hits += usize::from(row["positive"].as_bool().unwrap());
        rows.push(row);
    }
    let mut planted_seed = PLANTED_SEED;
    for case in 0..PLANTED_CASES {
        let mut q = None;
        for _ in 0..8 {
            let i = (splitmix64(&mut planted_seed) % pts.len() as u64) as usize;
            q = ar.add(PRIME31, q, pts[i]);
        }
        let row = measure(ar, &h, &pts, q, "blinded_constructed_positive", case, None);
        assert!(row["positive"].as_bool().unwrap(), "planted eight-leaf positive missed");
        rows.push(row);
    }
    let zero = measure(ar, &h, &pts, None, "identity_cancellation", 0, None);
    assert!(zero["positive"].as_bool().unwrap());
    // The exact group-theoretic zero-fibre theorem: D4 injectivity forces
    // every D8 sum to identity to be a cancellation net d=0. There are
    // C(B+3,4) distinct x-only cancellation polynomials for that ONE d.
    let four_sum_injective = h.extra_group_collisions == 0;
    let identity_unique_net_zero = zero["distinct_eight_net_signatures"].as_u64() == Some(1)
        && zero["all_net_signatures"].as_array()
            .is_some_and(|roots| roots.len() == 1 && roots[0] == json!(vec![0_i8; B]));
    if four_sum_injective {
        assert!(identity_unique_net_zero,
            "D4 group-point injectivity must imply only net zero at eight leaves");
    }
    let identity_x_polynomial_cancellation_count = binomial(B + 3, 4);
    assert_eq!(identity_x_polynomial_cancellation_count, 46_376);
    rows.push(zero);
    // Retrospective controls from independently recorded earlier two-base CI.
    // These two Q are NOT part of the prospective 32-target sample.
    for (case, q) in [
        Some((854_640_086_u64, 1_374_782_344_u64)),
        Some((1_146_958_692_u64, 202_234_633_u64)),
    ].into_iter().enumerate() {
        let row = measure(ar, &h, &pts, q, "retrospective_cross_base_regression", case, None);
        if label.starts_with("dickson") {
            assert_eq!(row["distinct_eight_net_signatures"].as_u64(), Some(0),
                "prior Dickson nonrepresentation falsified by new exhaustive oracle");
        } else {
            assert!(row["positive"].as_bool().unwrap(),
                "prior first-x representation falsified by new exhaustive oracle");
        }
        rows.push(row);
    }
    json!({
        "base_kind":label,
        "factor_base":base,
        "factor_base_build_ns_diagnostic_only":base_setup_ns.to_string(),
        "factor_base_build_group_additions":base_setup_additions,
        "four_sum_integer_signatures":h.integer_half_signatures,
        "four_sum_distinct_group_points":h.ordered_points.len(),
        "four_sum_group_collisions":h.extra_group_collisions,
        "identity_four_sum_injective":four_sum_injective,
        "identity_unique_net_zero":identity_unique_net_zero,
        "identity_x_polynomial_cancellation_count":identity_x_polynomial_cancellation_count,
        "unordered_four_token_words":h.word_multisets,
        "four_sum_setup_ns_diagnostic_only":h.setup_ns.to_string(),
        "four_sum_setup_group_additions":h.additions,
        "natural_cases":NATURAL_CASES,
        "natural_eight_leaf_reachable":natural_hits,
        "planted_cases":PLANTED_CASES,
        "planted_reachable":PLANTED_CASES,
        "cases":rows
    })
}

fn main() {
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("prime31 certificate");
    let t0 = Instant::now();
    let before = ar.counts.group_additions;
    let first_base = first_rational_points(&mut ar);
    let first_ns = t0.elapsed().as_nanos();
    let first_adds = ar.counts.group_additions - before;
    let first = panel(&mut ar, first_base, "first31_rational_x", first_ns, first_adds);

    let t1 = Instant::now();
    let before = ar.counts.group_additions;
    let special = Base::build(PRIME31, 6, &mut ar).expect("Dickson depth-six fibre");
    let dickson_ns = t1.elapsed().as_nanos();
    let dickson_adds = ar.counts.group_additions - before;
    assert_eq!(special.points.len(), B);
    let dickson = panel(&mut ar, special.points, "dickson_depth6_31_rational_points", dickson_ns, dickson_adds);
    let m = integer_signature_count(B, 4);
    let s = integer_signature_count(B, 8);
    println!("{}", json!({
        "schema":"prime31.s9.b31_net_signature_multiplicity.v1",
        "curve":PRIME31,
        "algorithm":"generic four-sum point table; diagnostic exact enumeration of eight-net signatures, not non-generic S9 elimination",
        "natural_seed":format!("{NATURAL_SEED:#x}"),
        "planted_seed":format!("{PLANTED_SEED:#x}"),
        "exact_integer_four_signature_count":m,
        "exact_integer_eight_signature_count":s,
        "exact_ordered_integer_half_pair_count":m * m,
        "exact_mean_net_signatures_per_group_target":s as f64 / PRIME31.n as f64,
        "exact_mean_ordered_integer_half_pairs_per_group_target":(m * m) as f64 / PRIME31.n as f64,
        "exact_mean_ordered_pairs_per_net_signature":(m * m) as f64 / s as f64,
        "panels":[first,dickson],
        "full_unknown_target_ecdlp_solves":0,
        "matched_optimized_rho":false,
        "original_ideal_certified_nongeneric_S9_extractor":false,
        "pcc":"unverified"
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn every_word(arity: usize, n_tokens: usize, start: usize, word: &mut Vec<usize>, out: &mut HashSet<Sig>) {
        if word.len() == arity {
            out.insert(word_signature(word));
            return;
        }
        for i in start..n_tokens {
            word.push(i);
            every_word(arity, n_tokens, i, word, out);
            word.pop();
        }
    }

    #[test]
    fn exact_four_eight_counts_and_b4_partition_equivalence() {
        assert_eq!(integer_signature_count(31, 4), 618_885);
        assert_eq!(integer_signature_count(31, 8), 5_654_193_417);
        let mut four = HashSet::new();
        let mut eight = HashSet::new();
        every_word(4, 8, 0, &mut Vec::new(), &mut four);
        every_word(8, 8, 0, &mut Vec::new(), &mut eight);
        assert_eq!(four.len(), 225);
        assert_eq!(eight.len(), 2241);
        let mut from_halves = HashSet::new();
        for u in &four { for v in &four {
            from_halves.insert(sum_signatures(u, v));
        }}
        assert_eq!(from_halves, eight);
    }

    #[test]
    fn b4_curve_roots_replay_and_zero_control() {
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).unwrap();
        let all = first_rational_points(&mut ar);
        let pts = signed_tokens(&ar, &all[..4]);
        let h = Half::build(&mut ar, &pts);
        assert_eq!(h.integer_half_signatures, 225);
        assert_eq!(h.extra_group_collisions, 0);
        let q = ar.add(PRIME31, pts[0], pts[1]);
        assert!(!h.all_net_roots(&mut ar, &pts, q).0.is_empty());
        let zero = h.all_net_roots(&mut ar, &pts, None);
        assert_eq!(zero.0.len(), 1, "injective four-leaf image gives only the zero eight-net");
        assert_eq!(zero.0[0], [0_i8; 31]);
        assert_eq!(binomial(4 + 3, 4), 35,
            "B4 has exactly 35 x-only cancellation polynomials");
        assert_eq!(binomial(31 + 3, 4), 46_376);
    }
}
