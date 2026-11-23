//! Block-scale k=4..8 controlled W8 generic-grader experiment on the original Prime31 curve.
//! This is NOT an algebraic unknown-index extractor or a faster complete ECDLP solver.
//! Reuses the pinned original prime31-algebraic-ic Rust arithmetic.
//! Protocol: ../rr9/W8_BLOCK_SCALE_PROTOCOL_20261119.md
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::Instant;

const N: u64 = 2_147_446_831;
const G: Point = Some((625_037_715, 2_100_627_251));
const SEED: u64 = 0x424c_4f43_4b53_4341;
const WORDS: usize = 65_536;
const WINDOWS: [[usize; 8]; 4] = [
    [0,1,2,3,4,5,6,7],
    [8,9,10,11,12,13,14,15],
    [16,17,18,19,20,21,22,23],
    [23,24,25,26,27,28,29,30],
];
const K: [usize; 5] = [4, 5, 6, 7, 8];

fn splitmix64(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *seed;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}
fn nonzero_scalar(seed: &mut u64) -> u64 {
    let threshold = N.wrapping_neg() % N;
    loop {
        let raw = splitmix64(seed);
        if raw >= threshold {
            let scalar = raw % N;
            if scalar != 0 { return scalar; }
        }
    }
}
fn base31(ar: &mut Arithmetic) -> Vec<Point> {
    let mut base = Vec::with_capacity(31);
    for x in 0..10_000u64 {
        let rhs = ar_rhs(ar, x);
        if let Some(y) = ar.sqrt(rhs, PRIME31.p) {
            if y != 0 {
                let p = Some((x, y.min(PRIME31.p - y)));
                assert!(ar.on_curve(PRIME31, p));
                base.push(p);
                if base.len() == 31 { break; }
            }
        }
    }
    assert_eq!(base.len(), 31);
    assert_eq!(
        base.iter().take(8).map(|p| p.unwrap().0).collect::<Vec<_>>(),
        vec![0,1,2,3,5,6,8,9]
    );
    base
}
fn ar_rhs(ar: &mut Arithmetic, x: u64) -> u64 {
    ar.rhs(PRIME31, x)
}

type Fibre = (u64, u16);
type Summands = Vec<(Point, Fibre)>;

fn signed_slots(ar: &mut Arithmetic, base: &[Point], indices: [usize; 8]) -> Vec<Vec<Point>> {
    [1u64, 2, 4, 8].iter().map(|&w| {
        let positive: Vec<Point> = indices.iter()
            .map(|&i| ar.scalar(PRIME31, w, base[i])).collect();
        assert!(positive.iter().all(Option::is_some));
        let mut signed = positive.clone();
        signed.extend(positive.iter().map(|&p| ar.neg(PRIME31, p)));
        assert_eq!(signed.len(), 16);
        signed
    }).collect()
}
fn sum_word(ar: &mut Arithmetic, slots: &[Vec<Point>], mut word: usize) -> Point {
    let mut out: Point = None;
    for slot in slots {
        out = ar.add(PRIME31, out, slot[word & 15]);
        word >>= 4;
    }
    assert_eq!(word, 0);
    out
}
fn build_s(ar: &mut Arithmetic, slots: &[Vec<Point>]) -> Summands {
    assert_eq!(slots.len(), 4);
    let mut map: HashMap<Point, Fibre> = HashMap::with_capacity(WORDS);
    for word in 0..WORDS {
        let p = sum_word(ar, slots, word);
        map.entry(p)
            .and_modify(|entry| entry.0 += 1)
            .or_insert((1, word as u16));
    }
    let mut rows: Summands = map.into_iter().collect();
    rows.sort_by_key(|(p, _)| *p);
    assert_eq!(rows.iter().map(|(_, (count, _))| *count).sum::<u64>(), WORDS as u64);
    rows
}
fn double_scaled(ar: &mut Arithmetic, scaled: &mut Summands) {
    for (point, _) in scaled.iter_mut() {
        *point = ar.add(PRIME31, *point, *point);
    }
}
fn right_map(scaled: &Summands, original: &Summands) -> HashMap<Point, Fibre> {
    let right: HashMap<Point, Fibre> = scaled.iter().copied().collect();
    // This is guaranteed mathematically by gcd(2,n)=1, and checked in the
    // original actual EC group rather than silently assuming no EC collisions.
    assert_eq!(right.len(), original.len(), "doubling map lost points");
    assert_eq!(
        right.values().map(|f| f.0).sum::<u64>(),
        WORDS as u64
    );
    right
}
fn witness_replay(
    ar: &mut Arithmetic, slots: &[Vec<Point>], left_word: u16,
    right_word: u16, k: usize,
) -> Point {
    let u = sum_word(ar, slots, left_word as usize);
    let mut v = sum_word(ar, slots, right_word as usize);
    for _ in 0..k { v = ar.add(PRIME31, v, v); }
    ar.add(PRIME31, u, v)
}
fn count_all(
    ar: &mut Arithmetic, left: &Summands, right: &HashMap<Point, Fibre>,
    target: Point,
) -> (u64, Option<(u16, u16)>) {
    let mut count = 0u64;
    let mut witness = None;
    for &(u, (lc, lw)) in left {
        let minus_u = ar.neg(PRIME31, u);
        let complement = ar.add(PRIME31, target, minus_u);
        if let Some(&(rc, rw)) = right.get(&complement) {
            count += lc * rc;
            if witness.is_none() { witness = Some((lw, rw)); }
        }
    }
    assert_eq!(count > 0, witness.is_some());
    (count, witness)
}
fn stats(v: &[u64]) -> Value {
    let mut hist = [0u64; 7];
    let mut total = 0u64;
    let mut squared = 0u128;
    let mut max = 0u64;
    for &x in v {
        total += x;
        squared += (x as u128) * (x as u128);
        max = max.max(x);
        let h = match x {0=>0, 1=>1, 2=>2, 3=>3, 4..=7=>4, 8..=15=>5, _=>6};
        hist[h] += 1;
    }
    json!({
        "hits":v.len() as u64-hist[0], "misses":hist[0],
        "total_witnesses":total, "witness_square_sum":squared.to_string(),
        "max":max, "hist_0_1_2_3_4to7_8to15_16plus":hist
    })
}
fn study(per_window: usize) -> Value {
    assert!((1..=128).contains(&per_window));
    let mut setup = Arithmetic::default();
    check_curve(PRIME31, &mut setup).expect("original cofactor-one curve certificate");
    assert_eq!(PRIME31.n, N);
    let base = base31(&mut setup);
    let mut seed = SEED;
    // All known generator scalars are consumed before any relation grader exists.
    // The only inputs to each grader are point coordinates; no discrete logs.
    let qs: Vec<Point> = (0..4*per_window)
        .map(|_| {
            let d = nonzero_scalar(&mut seed);
            setup.scalar(PRIME31, d, G)
        }).collect();
    let mut windows = Vec::new();
    let mut pooled: [Vec<u64>; 5] = std::array::from_fn(|_| Vec::new());
    let mut pairwise = [[0usize; 5]; 5];
    let mut certificates = Vec::new();
    for (wi, indices) in WINDOWS.iter().copied().enumerate() {
        let targets = &qs[wi*per_window..(wi+1)*per_window];
        let mut ar = Arithmetic::default();
        let begin = Instant::now();
        let slots = signed_slots(&mut ar, &base, indices);
        let s = build_s(&mut ar, &slots);
        let build_s_ns = begin.elapsed().as_nanos().to_string();
        let build_s_group_additions = ar.counts.group_additions;
        let mut scaled = s.clone();
        let mut arms = Vec::new();
        let mut window_counts: [Vec<u64>; 5] = std::array::from_fn(|_| Vec::new());
        for (ai, &k) in K.iter().enumerate() {
            let t_scale = Instant::now();
            let count_before_scale = ar.counts.group_additions;
            let steps = if ai == 0 {4} else {1};
            for _ in 0..steps { double_scaled(&mut ar, &mut scaled); }
            let right = right_map(&scaled, &s);
            let scale_ns = t_scale.elapsed().as_nanos().to_string();
            let scale_group_additions = ar.counts.group_additions-count_before_scale;
            let t_probe = Instant::now();
            let additions_before_probe = ar.counts.group_additions;
            let mut cert: Option<Value> = None;
            for (qi, &q) in targets.iter().enumerate() {
                let (multiplicity, witness) = count_all(&mut ar, &s, &right, q);
                if cert.is_none() {
                    if let Some((l, r)) = witness {
                        assert_eq!(witness_replay(&mut ar, &slots, l, r, k), q);
                        cert=Some(json!({
                            "window":wi, "k":k, "q_index":qi, "target":q,
                            "left_word":l, "right_word":r, "window_indices":indices
                        }));
                    }
                }
                window_counts[ai].push(multiplicity);
            }
            let probe_ns = t_probe.elapsed().as_nanos().to_string();
            let probe_group_additions = ar.counts.group_additions-additions_before_probe;
            assert!(cert.is_some(), "no positive point to independently replay");
            certificates.push(cert.unwrap());
            arms.push(json!({
                "k":k,
                "weights":[1u64,2,4,8,1u64<<k,1u64<<(k+1),1u64<<(k+2),1u64<<(k+3)],
                "four_slot_unique":s.len(),"right_unique":right.len(),
                "build_scaled_table_ns":scale_ns,"build_scaled_group_additions":scale_group_additions,
                "full_probe_ns":probe_ns,"full_probe_group_additions":probe_group_additions,
                "stats":stats(&window_counts[ai])
            }));
            pooled[ai].extend_from_slice(&window_counts[ai]);
        }
        for a in 0..5 {
            for b in 0..5 {
                for (&u, &v) in window_counts[a].iter().zip(&window_counts[b]) {
                    if u == 0 && v > 0 { pairwise[a][b] += 1; }
                }
            }
        }
        windows.push(json!({
            "window":wi,"indices":indices,"targets":targets,
            "shared_four_slot_unique":s.len(),"shared_four_slot_words":WORDS,
            "shared_four_slot_build_ns":build_s_ns,
            "shared_four_slot_group_additions":build_s_group_additions,
            "arms":arms,"multiplicity_per_target":window_counts
        }));
    }
    json!({
        "schema":"prime31.w8.blockscale.v1",
        "curve":{"p":PRIME31.p,"a":PRIME31.a,"b":PRIME31.b,
                "n":PRIME31.n,"cofactor":1,"generator":G},
        "seed":format!("{SEED:016x}"),"samples_per_window":per_window,
        "k_values":K,"base":base,"per_window":windows,
        "pooled":(0..5).map(|ai|json!({"k":K[ai],"stats":stats(&pooled[ai])})).collect::<Vec<_>>(),
        "pairwise_a_miss_b_hit":pairwise,"certificates":certificates,
        "all_65536_words_in_each_half":true,
        "generic_four_plus_four_grading_only":true,
        "nonenumerative_index_extraction":false,
        "complete_ecdlp_solved":false,
        "pcc_verified":false
    })
}
fn main() {
    println!("{}", serde_json::to_string_pretty(&study(128)).unwrap());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_curve_and_target_base() {
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).unwrap();
        assert_eq!(base31(&mut ar).len(),31);
        assert_eq!(K,[4,5,6,7,8]);
    }
    #[test]
    fn four_slot_bijective_doubling_invariant() {
        let mut ar = Arithmetic::default();
        let base = base31(&mut ar);
        let slots = signed_slots(&mut ar, &base, WINDOWS[1]);
        let s = build_s(&mut ar, &slots);
        let original = s.iter().map(|(_,f)|f.0).sum::<u64>();
        let mut scaled = s.clone();
        for _k in 1..=8 {
            double_scaled(&mut ar, &mut scaled);
            let right = right_map(&scaled, &s);
            assert_eq!(right.len(),s.len());
            assert_eq!(right.values().map(|f|f.0).sum::<u64>(), original);
        }
    }
    #[test]
    fn planted_indexed_relation_is_counted() {
        let mut ar=Arithmetic::default();
        let base=base31(&mut ar);
        let slots=signed_slots(&mut ar, &base, WINDOWS[0]);
        let s=build_s(&mut ar, &slots);
        let mut scaled=s.clone();
        for _ in 0..5 {double_scaled(&mut ar, &mut scaled);}
        let right=right_map(&scaled,&s);
        let q=witness_replay(&mut ar,&slots,4132,32871,5);
        let (count,wit)=count_all(&mut ar,&s,&right,q);
        assert!(count>=1);
        let (l,r)=wit.unwrap();
        assert_eq!(witness_replay(&mut ar,&slots,l,r,5),q);
    }
    #[test]
    fn small_public_only_deterministic_study() {
        let r=study(8);
        assert_eq!(r["schema"],"prime31.w8.blockscale.v1");
        assert_eq!(r["certificates"].as_array().unwrap().len(),20);
        for row in r["per_window"].as_array().unwrap() {
            for arm in row["arms"].as_array().unwrap() {
                assert_eq!(arm["four_slot_unique"],arm["right_unique"]);
                let st=&arm["stats"];
                assert_eq!(st["hits"].as_u64().unwrap()+st["misses"].as_u64().unwrap(),8);
            }
        }
    }
}
