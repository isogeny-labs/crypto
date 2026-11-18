//! Exact F2 group-algebra parity experiment on the unchanged ordinary Prime31
//! curve; generic 4+4 MITM DIAGNOSTIC, not a nonenumerative ECDLP extractor.
//! Uses repository original Arithmetic, Point and PRIME31; no target log in oracle.
//! Protocol: PRIME31_WEIGHTED_SLOT_PARITY_PROTOCOL_20261118.md
//! [pcc:unverified]
//! cargo +1.100.0 test --release --locked --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_weighted_slot_parity
//! cargo +1.100.0 run --release --locked --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_weighted_slot_parity
use prime31_algebraic_ic::{check_curve, Arithmetic, Counts, Point, PRIME31};
use serde_json::json;
use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::time::Instant;

const WEIGHTS: [u64; 8] = [1, 3, 5, 7, 11, 13, 17, 19];
const SEED: u64 = 0x5753_4c4f_5450_3131;
type Word4 = [u8; 4];

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn first_rational_points(ar: &mut Arithmetic, b: usize) -> Vec<Point> {
    let mut points = Vec::with_capacity(b);
    for x in 0..PRIME31.p {
        let rhs = ar.rhs(PRIME31, x);
        if let Some(y) = ar.sqrt(rhs, PRIME31.p) {
            assert_ne!(y, 0, "odd-order curve cannot contain nonzero 2-torsion");
            points.push(Some((x, y.min(PRIME31.p - y))));
            if points.len() == b {
                return points;
            }
        }
    }
    panic!("fewer rational points than requested");
}

fn slots(ar: &mut Arithmetic, base: &[Point], weights: &[u64; 8]) -> [Vec<Point>; 8] {
    std::array::from_fn(|i| {
        base.iter()
            .map(|&p| ar.scalar(PRIME31, weights[i], p))
            .collect()
    })
}

fn eval(ar: &mut Arithmetic, slots: &[Vec<Point>; 8], word: &[u8; 8]) -> Point {
    let mut sum = None;
    for i in 0..8 {
        sum = ar.add(PRIME31, sum, slots[i][word[i] as usize]);
    }
    sum
}

#[derive(Debug)]
struct Half {
    // A key is present iff the number of ordered four-slot words is odd.
    odd: HashMap<Point, Word4>,
    raw_words: u64,
    ns: u128,
    group_additions: u64,
}

fn half(ar: &mut Arithmetic, slots: &[Vec<Point>; 8], start: usize) -> Half {
    let before = ar.counts.group_additions;
    let start_ns = Instant::now();
    let b = slots[0].len();
    assert!(b <= u8::MAX as usize && b > 0);
    let mut left_pairs = Vec::with_capacity(b * b);
    let mut right_pairs = Vec::with_capacity(b * b);
    for i in 0..b {
        for j in 0..b {
            left_pairs.push((ar.add(PRIME31, slots[start][i], slots[start + 1][j]), [i as u8, j as u8]));
            right_pairs.push((ar.add(PRIME31, slots[start + 2][i], slots[start + 3][j]), [i as u8, j as u8]));
        }
    }
    let mut odd = HashMap::<Point, Word4>::with_capacity(b.pow(4));
    for &(lp, lword) in &left_pairs {
        for &(rp, rword) in &right_pairs {
            let point = ar.add(PRIME31, lp, rp);
            match odd.entry(point) {
                Entry::Vacant(e) => { e.insert([lword[0], lword[1], rword[0], rword[1]]); }
                Entry::Occupied(e) => { e.remove(); }
            }
        }
    }
    Half {
        odd,
        raw_words: (b as u64).pow(4),
        ns: start_ns.elapsed().as_nanos(),
        group_additions: ar.counts.group_additions - before,
    }
}

fn parity_query(ar: &mut Arithmetic, slots: &[Vec<Point>; 8],
                l: &Half, r: &Half, target: Point) -> (bool, Option<[u8; 8]>, u64) {
    let before = ar.counts.group_additions;
    let mut parity = false;
    let mut witness: Option<[u8; 8]> = None;
    for (&pt, left) in &l.odd {
        let need = ar.add(PRIME31, target, ar.neg(PRIME31, pt));
        if let Some(right) = r.odd.get(&need) {
            parity = !parity;
            witness = Some([
                left[0], left[1], left[2], left[3],
                right[0], right[1], right[2], right[3],
            ]);
        }
    }
    // A surviving odd match entails at least one true relation, and
    // witness labels are recovered without target scalar information.
    if parity {
        assert_eq!(eval(ar, slots, &witness.expect("odd parity needs a witness")), target);
    }
    (parity, if parity { witness } else { None }, ar.counts.group_additions - before)
}

fn brute_support(ar: &mut Arithmetic, slots: &[Vec<Point>; 8]) -> HashSet<Point> {
    assert_eq!(slots[0].len(), 2);
    let mut odd = HashSet::<Point>::new();
    for mask in 0..256u16 {
        let mut word = [0u8; 8];
        for (j, w) in word.iter_mut().enumerate() {
            *w = ((mask >> j) & 1) as u8;
        }
        let sum = eval(ar, slots, &word);
        if !odd.insert(sum) {
            odd.remove(&sum);
        }
    }
    odd
}

fn cross_support(ar: &mut Arithmetic, l: &Half, r: &Half) -> HashSet<Point> {
    let mut out = HashSet::<Point>::new();
    for &p in l.odd.keys() {
        for &q in r.odd.keys() {
            let sum = ar.add(PRIME31, p, q);
            if !out.insert(sum) {
                out.remove(&sum);
            }
        }
    }
    out
}

fn validate_exhaustive_b2(ar: &mut Arithmetic, base: &[Point]) {
    let two = &base[..2];
    for weights in [[1u64; 8], WEIGHTS] {
        let s = slots(ar, two, &weights);
        let left = half(ar, &s, 0);
        let right = half(ar, &s, 4);
        assert_eq!(cross_support(ar, &left, &right), brute_support(ar, &s));
    }

    // Different per-slot public translations cannot break uniform-base
    // characteristic-two cancellation: product factors as [sum T_i] A^8.
    let uniform = slots(ar, two, &[1u64; 8]);
    let mut translated = uniform.clone();
    let mut offset_sum = None;
    for (i, slot) in translated.iter_mut().enumerate() {
        let shift = ar.scalar(PRIME31, (i + 1) as u64, Some(PRIME31.g));
        offset_sum = ar.add(PRIME31, offset_sum, shift);
        for p in slot.iter_mut() {
            *p = ar.add(PRIME31, *p, shift);
        }
    }
    let left = half(ar, &translated, 0);
    let right = half(ar, &translated, 4);
    let actual = cross_support(ar, &left, &right);
    let expected: HashSet<_> = two.iter()
        .map(|&p| ar.add(PRIME31, ar.scalar(PRIME31, 8, p), offset_sum))
        .collect();
    assert_eq!(actual, expected);
}

fn counts_delta(now: Counts, old: Counts) -> serde_json::Value {
    json!({
        "group_additions": now.group_additions - old.group_additions,
        "fp_multiplications": now.fp_multiplications - old.fp_multiplications,
        "fp_inversions": now.fp_inversions - old.fp_inversions,
    })
}

fn panel(ar: &mut Arithmetic, base: &[Point], residuals: &[Point]) -> serde_json::Value {
    let b = base.len();
    let old = ar.counts;
    let setup_time = Instant::now();
    let uniform: HashSet<Point> = base.iter()
        .map(|&p| ar.scalar(PRIME31, 8, p))
        .collect();
    assert_eq!(uniform.len(), b, "[8] must be injective over odd-order E");
    let s = slots(ar, base, &WEIGHTS);
    let left = half(ar, &s, 0);
    let right = half(ar, &s, 4);
    let setup_ns = setup_time.elapsed().as_nanos();
    let setup_counts = counts_delta(ar.counts, old);
    let weighted_plant = eval(ar, &s, &[0; 8]);
    let uniform_plant = ar.scalar(PRIME31, 8, base[0]);

    let mut target_rows = Vec::new();
    let mut num_odd = 0;
    let mut query_ns_total = 0u128;
    let mut query_additions = 0u64;
    for (case, &q) in residuals.iter().enumerate() {
        let tic = Instant::now();
        let (odd, witness, operations) = parity_query(ar, &s, &left, &right, q);
        let ns = tic.elapsed().as_nanos();
        query_ns_total += ns;
        query_additions += operations;
        num_odd += usize::from(odd);
        target_rows.push(json!({
            "case":case, "public_target":q,
            "uniform_slot_odd_parity":uniform.contains(&q),
            "weighted_slot_odd_parity":odd,
            "weighted_witness_index_per_slot_if_odd":witness,
            "query_additions_including_witness_replay":operations,
            "query_ns_diagnostic":ns,
        }));
    }
    let checks: Vec<_> = [uniform_plant, weighted_plant].iter().enumerate()
        .map(|(j, &q)| {
            let (odd, word, ops) = parity_query(ar, &s, &left, &right, q);
            json!({"check":j,"public_target":q,"uniform_slot_odd_parity":uniform.contains(&q),
                "weighted_slot_odd_parity":odd,"weighted_word_if_odd":word,
                "extra_group_additions":ops})
        }).collect();
    json!({
        "b": b, "raw_words_per_four_slot_half":left.raw_words,
        "unweighted_theorem_odd_support_size": uniform.len(),
        "uniform_probability_bound_per_uniform_R": format!("{}/{}",b,PRIME31.n),
        "left_parity_support":left.odd.len(), "right_parity_support":right.odd.len(),
        "left_build_ns_diagnostic":left.ns, "right_build_ns_diagnostic":right.ns,
        "left_build_additions":left.group_additions,
        "right_build_additions":right.group_additions,
        "charged_candidate_setup_ns_diagnostic":setup_ns,
        "charged_candidate_setup_operations":setup_counts,
        "public_residuals":residuals.len(),"odd_residuals":num_odd,
        "public_query_ns_total_diagnostic":query_ns_total,
        "public_query_group_additions":query_additions,
        "controls":checks, "targets":target_rows,
        "nonenumerative_algebraic_extractor":false,
        "complete_ecdpl_unknown_logs_solved":0,
    })
}

fn main() {
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("source curve certificate");
    let base = first_rational_points(&mut ar, 31);
    assert!(base.iter().all(|&q| ar.on_curve(PRIME31, q)));
    validate_exhaustive_b2(&mut ar, &base);
    let mut state = SEED;
    let mut residuals = Vec::new();
    for _ in 0..12 {
        let k = 1 + splitmix64(&mut state) % (PRIME31.n - 1);
        residuals.push(ar.scalar(PRIME31, k, Some(PRIME31.g)));
    }
    let rows: Vec<_> = [8usize,16,31].into_iter()
        .map(|b| panel(&mut ar, &base[..b], &residuals))
        .collect();
    println!("{}", serde_json::to_string_pretty(&json!({
        "source_curve_p":PRIME31.p,"source_curve_a":PRIME31.a,"source_curve_b":PRIME31.b,
        "source_group_order":PRIME31.n,"generator":PRIME31.g,
        "weights":WEIGHTS,"fixed_seed":SEED,
        "curve_and_generator_checked":true,"exhaustive_b2_original_curve_parity_checked":true,
        "uniform_slot_parity_proved_by_freshman_dream":true,
        "experimental_panels":rows,
        "new_nongeneric_relations":0,"new_complete_ecdpl_logs":0,
        "performance_claim":"None; original Rust generic 4+4 cost diagnostic only",
        "pcc":"unverified"
    })).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_uniform_weighted_and_translated_parity_b2() {
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).unwrap();
        let base = first_rational_points(&mut ar, 2);
        validate_exhaustive_b2(&mut ar, &base);
    }
}
