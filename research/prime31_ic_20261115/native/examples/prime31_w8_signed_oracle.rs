//! Exact generic grading oracle for the proposed NON-GENERIC weighted RR9 chart extractor.
//! The 4+4 group enumeration here is strictly a ground-truth test oracle, NOT a solver advance.
//! Frozen panel and expected checks: W8_B4_B8_SIGNED_ORACLE_PROTOCOL_20261118.md
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

const P: u64 = 2_147_483_647;
const W: [u64; 8] = [1, 3, 5, 7, 11, 13, 17, 19];
const GENERATOR: Point = Some((625_037_715, 2_100_627_251));
const SEED: u64 = 0x7738_4234_4238_3231;
type Word4 = [u8; 4];
type Word8 = [u8; 8];

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}
fn sample_nonzero_uniform_scalar(state: &mut u64) -> u64 {
    let n = PRIME31.n;
    // Reject the low 2^64 mod n residues to avoid reduction bias.
    let threshold = n.wrapping_neg() % n;
    loop {
        let x = splitmix64(state);
        if x >= threshold {
            let k = x % n;
            if k != 0 { return k; }
        }
    }
}
fn canonical_base(ar: &mut Arithmetic, b: usize) -> Vec<Point> {
    let mut result = Vec::with_capacity(b);
    for x in 0..10_000 {
        let rhs = ar.rhs(PRIME31, x);
        if let Some(y) = ar.sqrt(rhs, P) {
            if y != 0 {
                let q = Some((x, y.min(P-y)));
                assert!(ar.on_curve(PRIME31, q));
                result.push(q);
                if result.len() == b { break; }
            }
        }
    }
    assert_eq!(result.len(), b);
    result
}
fn signed_slots(ar: &mut Arithmetic, base: &[Point]) -> Vec<Vec<Point>> {
    W.iter().map(|&weight| {
        let plus: Vec<Point> = base.iter().map(|&q| ar.scalar(PRIME31, weight, q)).collect();
        assert!(plus.iter().all(|q| q.is_some()));
        plus.iter().copied().chain(plus.iter().map(|&q| ar.neg(PRIME31, q))).collect()
    }).collect()
}
struct Half {
    words: Vec<(Point, Word4)>,
    index: HashMap<Point, (u64, Word4)>,
    formal: usize,
}
fn build_half(ar: &mut Arithmetic, slots: &[Vec<Point>], begin: usize, b: usize) -> Half {
    let m = 2*b;
    let mut words = Vec::with_capacity(m.pow(4));
    let mut index: HashMap<Point, (u64, Word4)> = HashMap::with_capacity(m.pow(4));
    let mut formal: HashSet<[i32; 8]> = HashSet::with_capacity(m.pow(4));
    for i in 0..m {
        for j in 0..m {
            let s2 = ar.add(PRIME31, slots[begin][i], slots[begin+1][j]);
            for k in 0..m {
                let s3 = ar.add(PRIME31, s2, slots[begin+2][k]);
                for l in 0..m {
                    let q = ar.add(PRIME31, s3, slots[begin+3][l]);
                    let w: Word4 = [i as u8, j as u8, k as u8, l as u8];
                    let mut coeff = [0i32;8];
                    for c in 0..4 {
                        let t = w[c] as usize;
                        let sg = if t < b {1i32} else {-1i32};
                        coeff[t%b] += sg * W[begin+c] as i32;
                    }
                    formal.insert(coeff);
                    words.push((q,w));
                    index.entry(q).and_modify(|entry| {
                        entry.0 += 1;
                    }).or_insert((1,w));
                }
            }
        }
    }
    assert_eq!(words.len(), m.pow(4));
    // Every formally identical vector maps to the same EC sum. Equality of
    // the two support cardinalities certifies NO additional EC collisions.
    assert_eq!(index.len(), formal.len());
    Half { words, index, formal: formal.len() }
}
fn replay(ar: &mut Arithmetic, slots: &[Vec<Point>], word: Word8) -> Point {
    let mut sum = None;
    for j in 0..8 { sum = ar.add(PRIME31, sum, slots[j][word[j] as usize]); }
    sum
}
fn query(ar: &mut Arithmetic, left: &Half, right: &Half,
         slots: &[Vec<Point>], target: Point) -> (u64, Option<Word8>) {
    let mut count = 0u64;
    let mut witness = None;
    for &(sum_l, word_l) in &left.words {
        let complement = ar.add(PRIME31, target, ar.neg(PRIME31, sum_l));
        if let Some(&(multiplicity,word_r)) = right.index.get(&complement) {
            count = count.checked_add(multiplicity).expect("witness count overflow");
            if witness.is_none() {
                let w = [word_l[0], word_l[1], word_l[2], word_l[3],
                         word_r[0], word_r[1], word_r[2], word_r[3]];
                assert_eq!(replay(ar, slots, w), target, "witness must replay original curve");
                witness = Some(w);
            }
        }
    }
    assert_eq!(count == 0, witness.is_none());
    (count,witness)
}
fn run(b: usize) -> Value {
    assert!(b==4 || b==8);
    let started = Instant::now();
    let mut ar=Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("original prime-order curve certificate");
    assert_eq!(PRIME31.p, P);
    let base=canonical_base(&mut ar,b);
    let slots=signed_slots(&mut ar,&base);
    let left=build_half(&mut ar,&slots,0,b);
    let right=build_half(&mut ar,&slots,4,b);
    let expected=if b==4 {3921} else {64737};
    assert_eq!(left.formal,expected);
    assert_eq!(right.formal,expected);
    let setup_ns=started.elapsed().as_nanos();
    let mut state=SEED ^ b as u64;
    let queries_start=Instant::now();
    let mut uniform=Vec::new();
    let mut hits=0usize;
    let mut multiplicity_total=0u64;
    for _ in 0..64 {
        let k=sample_nonzero_uniform_scalar(&mut state);
        // The private fixture scalar is deliberately not passed into query().
        let r=ar.scalar(PRIME31,k,GENERATOR);
        let (multiplicity,witness)=query(&mut ar,&left,&right,&slots,r);
        hits += usize::from(multiplicity>0);
        multiplicity_total += multiplicity;
        uniform.push(json!({"target":r,"multiplicity":multiplicity,"witness":witness}));
    }
    let mut planted=Vec::new();
    for t in 0..8 {
        let mut planted_word=[0u8;8];
        for j in 0..8 {
            let i=(3*t+7*j+j*j)%b;
            let positive=(t+3*j)%4<2;
            planted_word[j]=(i+if positive {0} else {b}) as u8;
        }
        let r=replay(&mut ar,&slots,planted_word);
        assert!(r.is_some(), "nonidentity planted target required for RR9");
        let (multiplicity,witness)=query(&mut ar,&left,&right,&slots,r);
        assert!(multiplicity>=1, "planted relation must be found");
        planted.push(json!({"target":r,"multiplicity":multiplicity,"witness":witness}));
    }
    let query_ns=queries_start.elapsed().as_nanos();
    let additions=ar.counts.group_additions;
    let inversions=ar.counts.fp_inversions;
    let multiplications=ar.counts.fp_multiplications;
    let nwords=(2*b as u64).pow(8);
    json!({
      "base_size":b,
      "base_x":base.iter().map(|q|q.unwrap().0).collect::<Vec<_>>(),
      "half_words":(2*b).pow(4),
      "left_group_support":left.index.len(),
      "right_group_support":right.index.len(),
      "left_formal_support":left.formal,
      "right_formal_support":right.formal,
      "unavoidable_formal_redundancy_per_half":(2*b).pow(4)-expected,
      "full_eight_word_count":nwords,
      "full_group_first_moment_numerator":nwords,
      "full_group_first_moment_denominator":PRIME31.n,
      "uniform_trials":uniform,
      "uniform_hits":hits,
      "uniform_multiplicity_sum":multiplicity_total,
      "planted_trials":planted,
      "original_curve_ec_additions_total":additions,
      "original_curve_fp_inversions_total":inversions,
      "original_curve_fp_multiplications_total":multiplications,
      "setup_ns":setup_ns.to_string(),
      "query_panel_ns":query_ns.to_string()
    })
}
fn main() {
    let record=json!({
        "schema":"prime31.w8.signed-4plus4-grader.v1",
        "date":"2026-11-18",
        "curve":{"p":PRIME31.p,"a":PRIME31.a,"b":PRIME31.b,
                 "n":PRIME31.n,"cofactor":1},
        "oracle_only":true,
        "generic_four_plus_four_tables":true,
        "unknown_rr9_chart_extractions":0,
        "complete_unknown_ecdlp_solves":0,
        "pcc":"unverified",
        "cases":[run(4),run(8)]
    });
    println!("{}",serde_json::to_string_pretty(&record).unwrap());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn b4_exact_witness_and_formal_support() {
        let r=run(4);
        assert_eq!(r["left_formal_support"],3921);
        assert_eq!(r["uniform_trials"].as_array().unwrap().len(),64);
        assert_eq!(r["planted_trials"].as_array().unwrap().len(),8);
    }
    #[test]
    fn b8_exact_witness_and_formal_support() {
        let r=run(8);
        assert_eq!(r["right_formal_support"],64737);
        assert_eq!(r["uniform_trials"].as_array().unwrap().len(),64);
        assert_eq!(r["planted_trials"].as_array().unwrap().len(),8);
    }
}
