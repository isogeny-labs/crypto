//! Exact W8 ordered-witness multiplicity on the original prime31 fixture.
//! Counts all signed 8-slot representations using generic 4+4 EC sum tables.
//! NEVER passes generator scalars to graders. NOT an algebraic extractor or ECDLP solver.
//! Frozen before data: rr9/W8_FULL_FIBRE_PROTOCOL_20261119.md
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::Instant;

const N: u64 = 2_147_446_831;
const G: Point = Some((625_037_715, 2_100_627_251));
const SEED: u64 = 0x5748_385f_4845_5432;
const WORDS: usize = 1 << 16;
const WINS: [[usize; 8]; 4] = [
    [0,1,2,3,4,5,6,7], [8,9,10,11,12,13,14,15],
    [16,17,18,19,20,21,22,23], [23,24,25,26,27,28,29,30],
];
const WEIGHTS: [[u64; 8]; 4] = [
    [1,3,5,7,11,13,17,19],
    [1,2,4,8,16,32,64,128],
    [1,2,4,8,16,32,64,127],
    [1,3,9,27,81,243,729,2187],
];
const LABELS: [&str; 4] = ["old", "pow2", "near127", "ternary"];

fn next(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *seed;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}
fn nonzero_scalar(seed: &mut u64) -> u64 {
    let threshold = N.wrapping_neg() % N;
    loop {
        let raw = next(seed);
        if raw >= threshold {
            let k = raw % N;
            if k != 0 { return k; }
        }
    }
}
fn base31(ar: &mut Arithmetic) -> Vec<Point> {
    let mut points = Vec::new();
    for x in 0..10_000u64 {
        let rhs = ar.rhs(PRIME31, x);
        if let Some(y) = ar.sqrt(rhs, PRIME31.p) {
            if y != 0 {
                points.push(Some((x, y.min(PRIME31.p - y))));
                if points.len() == 31 { break; }
            }
        }
    }
    assert_eq!(points.len(), 31);
    assert_eq!(points.iter().take(8).map(|p| p.unwrap().0).collect::<Vec<_>>(),
               vec![0,1,2,3,5,6,8,9]);
    points
}
fn slots(ar: &mut Arithmetic, base: &[Point], indices: [usize; 8],
         weights: [u64; 8]) -> Vec<Vec<Point>> {
    weights.iter().map(|&w| {
        let plus: Vec<Point> = indices.iter()
            .map(|&i| ar.scalar(PRIME31, w, base[i])).collect();
        assert!(plus.iter().all(Option::is_some));
        let mut signed = plus.clone();
        signed.extend(plus.iter().map(|&p| ar.neg(PRIME31, p)));
        signed
    }).collect()
}
fn partial_sum(ar: &mut Arithmetic, slots: &[Vec<Point>],
               begin: usize, mut word: usize) -> Point {
    let mut sum: Point = None;
    for slot in slots.iter().skip(begin).take(4) {
        sum = ar.add(PRIME31, sum, slot[word & 15]);
        word >>= 4;
    }
    sum
}
fn replay(ar: &mut Arithmetic, slots: &[Vec<Point>], left: u16, right: u16) -> Point {
    let u = partial_sum(ar, slots, 0, left as usize);
    let v = partial_sum(ar, slots, 4, right as usize);
    ar.add(PRIME31, u, v)
}
struct MultiSet {
    /// Sorted unique sum, count, lexicographically first word.
    left: Vec<(Point, (u64, u16))>,
    right: HashMap<Point, (u64, u16)>,
    slots: Vec<Vec<Point>>,
    left_words: u64,
    right_words: u64,
}
fn build(ar: &mut Arithmetic, base: &[Point],
         win: [usize; 8], weights: [u64; 8]) -> MultiSet {
    let slots = slots(ar, base, win, weights);
    let mut left: HashMap<Point, (u64, u16)> = HashMap::with_capacity(WORDS);
    let mut right: HashMap<Point, (u64, u16)> = HashMap::with_capacity(WORDS);
    for word in 0..WORDS {
        let u = partial_sum(ar, &slots, 0, word);
        let v = partial_sum(ar, &slots, 4, word);
        left.entry(u).and_modify(|x| x.0 += 1).or_insert((1, word as u16));
        right.entry(v).and_modify(|x| x.0 += 1).or_insert((1, word as u16));
    }
    let mut ordered_left: Vec<_> = left.into_iter().collect();
    ordered_left.sort_by_key(|(p, _)| *p);
    let left_words = ordered_left.iter().map(|(_, x)| x.0).sum();
    let right_words = right.values().map(|x| x.0).sum();
    assert_eq!(left_words, WORDS as u64);
    assert_eq!(right_words, WORDS as u64);
    MultiSet { left:ordered_left, right, slots, left_words, right_words }
}
fn multiplicity(ar: &mut Arithmetic, h: &MultiSet, r: Point) -> (u64, Option<(u16,u16)>) {
    let mut count = 0u64;
    let mut witness = None;
    for &(u, (left_count, left_word)) in &h.left {
        let complement = ar.add(PRIME31, r, ar.neg(PRIME31, u));
        if let Some(&(right_count, right_word)) = h.right.get(&complement) {
            count += left_count * right_count;
            if witness.is_none() { witness = Some((left_word, right_word)); }
        }
    }
    if let Some((wl, wr)) = witness {
        assert_eq!(replay(ar, &h.slots, wl, wr), r, "witness EC replay failed");
    } else {
        assert_eq!(count, 0);
    }
    assert_eq!(count > 0, witness.is_some());
    (count, witness)
}
fn stats(values: &[u64]) -> Value {
    let mut hist = [0u64; 7];
    let mut sum = 0u64;
    let mut sum_squares = 0u128;
    let mut max = 0u64;
    for &m in values {
        sum += m;
        sum_squares += (m as u128) * (m as u128);
        max = max.max(m);
        let bin = match m { 0=>0, 1=>1, 2=>2, 3=>3, 4..=7=>4, 8..=15=>5, _=>6 };
        hist[bin] += 1;
    }
    json!({"zero":hist[0],"positive":values.len() as u64-hist[0],
           "hist_0_1_2_3_4to7_8to15_16plus":hist,
           "sum_ordered_witnesses":sum,"sum_witness_squares":sum_squares.to_string(),
           "max":max})
}
fn study(per_window: usize) -> Value {
    assert!((1..=256).contains(&per_window));
    let mut rng = SEED;
    let mut setup = Arithmetic::default();
    check_curve(PRIME31, &mut setup).expect("unchanged ordinary prime-order curve");
    let base = base31(&mut setup);
    // Public target construction happens before any grader, scalar never passed to grader.
    let qs: Vec<Point> = (0..4*per_window)
        .map(|_| setup.scalar(PRIME31, nonzero_scalar(&mut rng), G)).collect();
    let mut windows = Vec::new();
    let mut pooled: [Vec<u64>;4] = std::array::from_fn(|_| Vec::new());
    let mut certificates = Vec::new();
    let mut pairwise = [[0u64;4];4];
    for (widx, win) in WINS.iter().copied().enumerate() {
        let window_qs = &qs[widx*per_window..(widx+1)*per_window];
        let mut per_arm: [Option<Value>;4] = std::array::from_fn(|_| None);
        let mut m: [Vec<u64>;4] = std::array::from_fn(|_| Vec::new());
        for cycle in 0..4 {
            let arm = (widx + cycle) % 4;
            let mut ar = Arithmetic::default();
            let start = Instant::now();
            let h = build(&mut ar, &base, win, WEIGHTS[arm]);
            let build_ns = start.elapsed().as_nanos().to_string();
            let build_ops = ar.counts.group_additions;
            let probe_start = Instant::now();
            for (qidx, &q) in window_qs.iter().enumerate() {
                let (count, witness) = multiplicity(&mut ar, &h, q);
                if qidx == 0 || (qidx > 0 && certificates.iter()
                    .filter(|x:&&Value|x["window"].as_u64() == Some(widx as u64) && x["arm"].as_str() == Some(LABELS[arm])).count() == 0 && count > 0) {
                    if let Some((left,right)) = witness {
                        certificates.push(json!({"window":widx,"arm":LABELS[arm],
                            "q_index":qidx,"target":q,"left_word":left,"right_word":right,
                            "weights":WEIGHTS[arm],"window_indices":win}));
                    }
                }
                m[arm].push(count);
            }
            let probe_ns = probe_start.elapsed().as_nanos().to_string();
            per_arm[arm] = Some(json!({
                "name":LABELS[arm],"weights":WEIGHTS[arm],
                "left_unique":h.left.len(),"right_unique":h.right.len(),
                "left_words":h.left_words,"right_words":h.right_words,
                "build_ns":build_ns,"full_probe_ns":probe_ns,
                "build_group_additions":build_ops,
                "probe_group_additions_including_replay":ar.counts.group_additions-build_ops,
                "fp_multiplications_total":ar.counts.fp_multiplications,
                "fp_inversions_total":ar.counts.fp_inversions,
                "stats":stats(&m[arm])
            }));
            pooled[arm].extend_from_slice(&m[arm]);
        }
        for a in 0..4 {
            for b in 0..4 {
                for (&ca,&cb) in m[a].iter().zip(&m[b]) {
                    if ca == 0 && cb > 0 { pairwise[a][b] += 1; }
                }
            }
        }
        windows.push(json!({
            "window":widx, "indices":win,
            "arm_cycle":(0..4).map(|i|LABELS[(widx+i)%4]).collect::<Vec<_>>(),
            "arms":per_arm,"multiplicity_per_target":m
        }));
    }
    json!({
       "schema":"prime31.w8.full-fibre.v1",
       "curve":{"p":PRIME31.p,"a":PRIME31.a,"b":PRIME31.b,
                "n":PRIME31.n,"cofactor":1,"generator":G},
       "seed":format!("{SEED:016x}"),"samples_per_window":per_window,
       "base":base,"labels":LABELS,"weights":WEIGHTS,
       "per_window":windows,
       "pooled":(0..4).map(|i|json!({"arm":LABELS[i],"stats":stats(&pooled[i])})).collect::<Vec<_>>(),
       "pairwise_a_miss_b_hit":pairwise,
       "certificates":certificates,
       "generic_four_plus_four_grading_only":true,
       "algebraic_extraction_implemented":false,
       "complete_ecdlp_solved":false,
       "pcc_verified":false
    })
}
fn main() {
    println!("{}", serde_json::to_string_pretty(&study(256)).unwrap());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_curve_and_weight_schedule() {
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).unwrap();
        assert_eq!(WEIGHTS[2][7], 127);
        assert_eq!(base31(&mut ar).len(), 31);
    }
    #[test]
    fn planted_public_residual_full_fibre_count() {
        let mut ar = Arithmetic::default();
        let b = base31(&mut ar);
        let h = build(&mut ar, &b, WINS[0], WEIGHTS[2]);
        let planted = replay(&mut ar, &h.slots, 4132, 32871);
        let (m, example) = multiplicity(&mut ar, &h, planted);
        assert!(m >= 1);
        let (l,r)=example.unwrap();
        assert_eq!(replay(&mut ar,&h.slots,l,r),planted);
        assert_eq!(h.left_words,65_536);
        assert_eq!(h.right_words,65_536);
    }
    #[test]
    fn public_only_small_same_window_controls() {
        let result=study(2);
        assert_eq!(result["schema"],"prime31.w8.full-fibre.v1");
        assert_eq!(result["pooled"].as_array().unwrap().len(), 4);
        for window in result["per_window"].as_array().unwrap() {
            for arm in window["arms"].as_array().unwrap() {
                let rows=arm["stats"]["hist_0_1_2_3_4to7_8to15_16plus"].as_array().unwrap();
                assert_eq!(rows.iter().map(|x|x.as_u64().unwrap()).sum::<u64>(),2);
            }
        }
    }
}
