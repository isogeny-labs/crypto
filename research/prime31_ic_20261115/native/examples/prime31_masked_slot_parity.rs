//! Original-curve independent binary slot masks: exact parity, *generic* diagnostic.
//! Frozen protocol: PRIME31_MASKED_SLOT_PARITY_PROTOCOL_20261118.md
//! Reuses the pinned native Arithmetic and PRIME31. [pcc:unverified]
use prime31_algebraic_ic::{check_curve, Arithmetic, Counts, Point, PRIME31};
use serde_json::json;
use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::time::Instant;

const MASK_SEED: u64 = 0x4d41_534b_5033_3131;
const RESIDUAL_SEED: u64 = 0x5753_4c4f_5450_3131;
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
            assert_ne!(y, 0, "odd-order group has no nonzero 2-torsion");
            points.push(Some((x, y.min(PRIME31.p - y))));
            if points.len() == b {
                return points;
            }
        }
    }
    panic!("insufficient rational points");
}

fn masked_slots(base: &[Point]) -> ([Vec<Point>; 8], [Vec<usize>; 8]) {
    let keep = base.len().div_ceil(2);
    let indices: [Vec<usize>; 8] = std::array::from_fn(|slot| {
        let mut ranked: Vec<(u64, usize)> = (0..base.len())
            .map(|j| {
                let mut state = MASK_SEED ^ (((slot + 1) as u64) << 32) ^ ((j + 1) as u64);
                (splitmix64(&mut state), j)
            })
            .collect();
        ranked.sort_unstable();
        let mut result: Vec<usize> = ranked[..keep].iter().map(|&(_, j)| j).collect();
        result.sort_unstable();
        result
    });
    let slots: [Vec<Point>; 8] = std::array::from_fn(|i| {
        indices[i].iter().map(|&j| base[j]).collect()
    });
    (slots, indices)
}

fn eval(ar: &mut Arithmetic, slots: &[Vec<Point>; 8], word: &[u8; 8]) -> Point {
    let mut sum = None;
    for i in 0..8 {
        sum = ar.add(PRIME31, sum, slots[i][word[i] as usize]);
    }
    sum
}

struct Half {
    odd: HashMap<Point, Word4>,
    raw_words: u64,
    group_additions: u64,
    ns: u128,
}

fn half(ar: &mut Arithmetic, slots: &[Vec<Point>; 8], start: usize) -> Half {
    let before = ar.counts.group_additions;
    let tic = Instant::now();
    let k = slots[0].len();
    assert!(k > 0 && k <= u8::MAX as usize);
    assert!(slots.iter().all(|slot| slot.len() == k));
    let mut first = Vec::with_capacity(k * k);
    let mut second = Vec::with_capacity(k * k);
    for i in 0..k {
        for j in 0..k {
            first.push((ar.add(PRIME31, slots[start][i], slots[start + 1][j]), [i as u8, j as u8]));
            second.push((ar.add(PRIME31, slots[start + 2][i], slots[start + 3][j]), [i as u8, j as u8]));
        }
    }
    let mut odd = HashMap::<Point, Word4>::with_capacity(k.pow(4));
    for &(p, a) in &first {
        for &(q, b) in &second {
            let sum = ar.add(PRIME31, p, q);
            match odd.entry(sum) {
                Entry::Vacant(e) => { e.insert([a[0], a[1], b[0], b[1]]); }
                Entry::Occupied(e) => { e.remove(); }
            }
        }
    }
    Half {
        odd,
        raw_words: (k as u64).pow(4),
        group_additions: ar.counts.group_additions - before,
        ns: tic.elapsed().as_nanos(),
    }
}

fn parity_query(
    ar: &mut Arithmetic, slots: &[Vec<Point>; 8],
    left: &Half, right: &Half, target: Point,
) -> (bool, Option<[u8; 8]>, u64) {
    let before = ar.counts.group_additions;
    let mut parity = false;
    let mut witness: Option<[u8; 8]> = None;
    for (&p, a) in &left.odd {
        let need = ar.add(PRIME31, target, ar.neg(PRIME31, p));
        if let Some(b) = right.odd.get(&need) {
            parity = !parity;
            witness = Some([a[0], a[1], a[2], a[3], b[0], b[1], b[2], b[3]]);
        }
    }
    if parity {
        assert_eq!(eval(ar, slots, &witness.expect("odd support needs witness")), target);
    }
    (parity, if parity { witness } else { None }, ar.counts.group_additions - before)
}

fn exhaustive_b4(ar: &mut Arithmetic, base: &[Point]) {
    let (s, _) = masked_slots(&base[..4]);
    assert!(s.iter().all(|v| v.len() == 2));
    let left = half(ar, &s, 0);
    let right = half(ar, &s, 4);
    let mut brute = HashSet::new();
    for mask in 0..256u16 {
        let mut word = [0u8; 8];
        for (j, w) in word.iter_mut().enumerate() {
            *w = ((mask >> j) & 1) as u8;
        }
        let p = eval(ar, &s, &word);
        if !brute.insert(p) { brute.remove(&p); }
    }
    let mut actual = HashSet::new();
    for &p in left.odd.keys() {
        for &q in right.odd.keys() {
            let sum = ar.add(PRIME31, p, q);
            if !actual.insert(sum) { actual.remove(&sum); }
        }
    }
    assert_eq!(actual, brute, "B4 exact ordered parity");
}

fn counts_delta(now: Counts, old: Counts) -> serde_json::Value {
    json!({
        "group_additions":now.group_additions-old.group_additions,
        "fp_multiplications":now.fp_multiplications-old.fp_multiplications,
        "fp_inversions":now.fp_inversions-old.fp_inversions,
    })
}

fn panel(ar: &mut Arithmetic, base: &[Point], residuals: &[Point]) -> serde_json::Value {
    let before = ar.counts;
    let tic = Instant::now();
    let (s, selected) = masked_slots(base);
    let same_slot: HashSet<Point> = s[0].iter().map(|&p| ar.scalar(PRIME31, 8, p)).collect();
    assert_eq!(same_slot.len(), s[0].len(), "[8] injective on odd-order group");
    let left = half(ar, &s, 0);
    let right = half(ar, &s, 4);
    let setup_ns = tic.elapsed().as_nanos();
    let setup_counts = counts_delta(ar.counts, before);
    let planted = eval(ar, &s, &[0;8]);
    let mut rows = Vec::new();
    let mut odd_total = 0usize;
    let mut query_additions = 0u64;
    let mut query_ns_total = 0u128;
    for (case, &target) in residuals.iter().enumerate() {
        let qtic = Instant::now();
        let (odd, witness, additions) = parity_query(ar, &s, &left, &right, target);
        let ns = qtic.elapsed().as_nanos();
        odd_total += usize::from(odd);
        query_additions += additions;
        query_ns_total += ns;
        rows.push(json!({
            "case":case,
            "public_target":target,
            "masked_odd_parity":odd,
            "masked_witness_slot_local_indices_if_odd":witness,
            "query_additions_including_witness_replay":additions,
            "query_ns_diagnostic":ns,
        }));
    }
    let (plant_odd, plant_witness, _) = parity_query(ar, &s, &left, &right, planted);
    json!({
        "b":base.len(), "per_slot_size":s[0].len(),
        "selected_base_indices":selected,
        "same_slot_odd_support_theorem":same_slot.len(),
        "raw_words_per_four_slot_half":left.raw_words,
        "left_parity_support":left.odd.len(),
        "right_parity_support":right.odd.len(),
        "left_build_additions":left.group_additions,
        "right_build_additions":right.group_additions,
        "left_build_ns_diagnostic":left.ns,
        "right_build_ns_diagnostic":right.ns,
        "candidate_setup_operations":setup_counts,
        "candidate_setup_ns_diagnostic":setup_ns,
        "public_residuals":residuals.len(),
        "odd_residuals":odd_total,
        "public_query_group_additions":query_additions,
        "public_query_ns_total_diagnostic":query_ns_total,
        "planted_known_relation_target":planted,
        "planted_target_odd_parity":plant_odd,
        "planted_target_witness_if_odd":plant_witness,
        "targets":rows,
        "nonenumerative_algebraic_extractor":false,
        "new_nongeneric_relations":0,
        "new_complete_ecdpl_unknown_logs":0,
    })
}

fn main() {
    let mut ar = Arithmetic::default();
    let old = ar.counts;
    check_curve(PRIME31, &mut ar).expect("original curve certificate");
    let base = first_rational_points(&mut ar, 31);
    assert!(base.iter().all(|&q| ar.on_curve(PRIME31, q)));
    let original_curve_and_base_cost = counts_delta(ar.counts, old);
    exhaustive_b4(&mut ar, &base);
    let mut state = RESIDUAL_SEED;
    let prior = ar.counts;
    let mut residuals = Vec::new();
    for _ in 0..12 {
        let k = 1 + splitmix64(&mut state) % (PRIME31.n - 1);
        residuals.push(ar.scalar(PRIME31, k, Some(PRIME31.g)));
    }
    let public_target_fixture_cost = counts_delta(ar.counts, prior);
    let panels: Vec<_> = [8usize,16,31].into_iter()
        .map(|b| panel(&mut ar, &base[..b], &residuals))
        .collect();
    println!("{}", serde_json::to_string_pretty(&json!({
        "source_curve_p":PRIME31.p,"source_curve_a":PRIME31.a,"source_curve_b":PRIME31.b,
        "source_group_order":PRIME31.n,"generator":PRIME31.g,
        "mask_seed_hex":format!("{MASK_SEED:#018x}"),
        "residual_seed_hex":format!("{RESIDUAL_SEED:#018x}"),
        "original_curve_and_base_cost":original_curve_and_base_cost,
        "public_target_fixture_cost":public_target_fixture_cost,
        "exhaustive_b4_original_curve_parity_checked":true,
        "experimental_panels":panels,
        "new_complete_ecdpl_logs":0,
        "performance_claim":"None: generic 4+4 parity cost control only; no complete ECDLP",
        "pcc":"unverified"
    })).expect("serialize"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn masked_b4_exact_parity_and_curve() {
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).unwrap();
        let base = first_rational_points(&mut ar, 4);
        exhaustive_b4(&mut ar, &base);
    }
}
