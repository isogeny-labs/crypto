//! Prime31 W8 paired signed-weight IC support A/B test, generic 4+4 GRADER ONLY.
//! NOT a non-generic RR9 unknown-index extractor. See preregistered
//! research/prime31_ic_20261115/rr9/W8_WINDOWED_B8_RANK_PROTOCOL_20261118.md
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

const P: u64 = 2_147_483_647;
const N: u64 = 2_147_446_831;
const OLD_W: [u64; 8] = [1, 3, 5, 7, 11, 13, 17, 19];
const TERNARY_W: [u64; 8] = [1, 3, 9, 27, 81, 243, 729, 2187];
// For each slot j, weight 3^j exceeds twice the sum of all earlier weights.
// Thus distinct signed, B-labelled slot words have distinct INTEGER
// coefficient vectors (not necessarily distinct EC point sums mod subgroup order).
fn signed_formal_injective(w: &[u64; 8]) -> bool {
    let mut prior = 0u64;
    for &x in w {
        if x <= 2*prior { return false; }
        prior += x;
    }
    true
}
const G: Point = Some((625_037_715, 2_100_627_251));
const SEED: u64 = 0x7738_4234_4238_3231;
const TRAIN_XOR: u64 = 0x5749_4e44_4f57_5338;
const TARGET_XOR: u64 = 0x5441_5247_4554_5338;
const WINDOWS: [[usize; 8]; 4] = [
    [0, 1, 2, 3, 4, 5, 6, 7],
    [8, 9, 10, 11, 12, 13, 14, 15],
    [16, 17, 18, 19, 20, 21, 22, 23],
    [23, 24, 25, 26, 27, 28, 29, 30],
];
type Word4 = [u8; 4];
type Word8 = [u8; 8];
type Row = [u64; 32];

fn addn(a: u64, b: u64) -> u64 { (a + b) % N }
fn subn(a: u64, b: u64) -> u64 { (a + N - b) % N }
fn muln(a: u64, b: u64) -> u64 { ((a as u128 * b as u128) % N as u128) as u64 }
fn pown(mut x: u64, mut e: u64) -> u64 {
    let mut out = 1;
    while e > 0 {
        if e & 1 != 0 { out = muln(out, x); }
        x = muln(x, x);
        e >>= 1;
    }
    out
}
fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}
fn nonzero_scalar(state: &mut u64) -> u64 {
    let threshold = N.wrapping_neg() % N;
    loop {
        let x = splitmix64(state);
        if x >= threshold {
            let k = x % N;
            if k != 0 { return k; }
        }
    }
}
fn canonical_base(ar: &mut Arithmetic) -> Vec<Point> {
    let mut out = Vec::with_capacity(31);
    for x in 0..10_000 {
        let rhs = ar.rhs(PRIME31, x);
        if let Some(y) = ar.sqrt(rhs, P) {
            if y != 0 {
                let q = Some((x, y.min(P-y)));
                assert!(ar.on_curve(PRIME31, q));
                out.push(q);
                if out.len() == 31 { break; }
            }
        }
    }
    assert_eq!(out.len(), 31);
    assert_eq!(out.iter().take(8).map(|q| q.unwrap().0).collect::<Vec<_>>(),
               vec![0,1,2,3,5,6,8,9]);
    out
}
struct Oracle {
    window: [usize; 8],
    weights: [u64; 8],
    slots: Vec<Vec<Point>>,
    left: Vec<(Point, Word4)>,
    right: HashMap<Point, Word4>,
}
impl Oracle {
    fn new(ar: &mut Arithmetic, base: &[Point], window: [usize; 8], weights: [u64; 8]) -> Self {
        let mut slots = Vec::new();
        let mut abscissae = HashSet::new();
        for (j, &weight) in weights.iter().enumerate() {
            let pos: Vec<Point> = window.iter().map(|&i| {
                let q = ar.scalar(PRIME31, weight, base[i]);
                let (x, _) = q.expect("nonidentity weighted support point");
                assert!(abscissae.insert(x), "overlapping weighted abscissae");
                q
            }).collect();
            let mut signed = pos.clone();
            signed.extend(pos.iter().map(|&q| ar.neg(PRIME31, q)));
            assert_eq!(signed.len(), 16, "eight signed points per slot");
            assert_eq!(j+1, slots.len()+1);
            slots.push(signed);
        }
        assert_eq!(abscissae.len(), 64);
        let mut left = Vec::with_capacity(65_536);
        let mut right = HashMap::with_capacity(65_536);
        for begin in [0usize, 4usize] {
            for i in 0..16 {
                for j in 0..16 {
                    let s2 = ar.add(PRIME31, slots[begin][i], slots[begin+1][j]);
                    for k in 0..16 {
                        let s3 = ar.add(PRIME31, s2, slots[begin+2][k]);
                        for l in 0..16 {
                            let q = ar.add(PRIME31, s3, slots[begin+3][l]);
                            let word = [i as u8, j as u8, k as u8, l as u8];
                            if begin == 0 {
                                left.push((q, word));
                            } else {
                                right.entry(q).or_insert(word);
                            }
                        }
                    }
                }
            }
        }
        assert_eq!(left.len(), 65_536);
        // The prior B8 fixture had 64,737 distinct right sums, but EC-only
        // collisions across these NEW windows must be measured, not assumed
        // absent. Hash first-witness semantics remain correct with collisions.
        assert!((64_000..=65_536).contains(&right.len()),
                "unexpectedly low four-slot support; inspect exact collisions");
        if weights == TERNARY_W {
            // This is an exact, instrumented EC observation on the fixed
            // Prime31 curve, not just a formal-vector uniqueness claim.
            assert_eq!(right.len(),65_536,"unexpected ternary 4-sum collision");
        }
        Self {window, weights, slots, left, right}
    }
    fn replay(&self, ar: &mut Arithmetic, word: Word8) -> Point {
        let mut sum = None;
        for j in 0..8 {
            sum = ar.add(PRIME31, sum, self.slots[j][word[j] as usize]);
        }
        sum
    }
    fn find(&self, ar: &mut Arithmetic, target: Point) -> Option<Word8> {
        // Generic MITM belongs to the GRADER ONLY, never the proposed RR9 extractor.
        // Caller passes Point, not its scalar, a chart or a planted word.
        for &(left_sum, left_word) in &self.left {
            let minus_left = ar.neg(PRIME31, left_sum);
            let complement = ar.add(PRIME31, target, minus_left);
            if let Some(&right_word) = self.right.get(&complement) {
                let word = [
                    left_word[0], left_word[1], left_word[2], left_word[3],
                    right_word[0], right_word[1], right_word[2], right_word[3],
                ];
                assert_eq!(self.replay(ar,word), target);
                return Some(word);
            }
        }
        None
    }
    fn row(&self, word: Word8, rhs: u64) -> Row {
        let mut out = [0u64; 32];
        for j in 0..8 {
            let leaf = word[j] as usize;
            assert!(leaf < 16);
            let c = if leaf < 8 {self.weights[j]} else {N - self.weights[j]};
            let idx = self.window[leaf % 8];
            out[idx] = addn(out[idx], c);
        }
        out[31] = rhs;
        out
    }
}
#[derive(Default)]
struct Rref { rows: Vec<(usize, Row)> }
impl Rref {
    fn rank(&self) -> usize { self.rows.len() }
    fn insert(&mut self, mut row: Row) -> bool {
        // Incremental Gauss-Jordan over the prime subgroup order. Every
        // identity is independently checked against original curve points.
        for &(pivot, ref b) in &self.rows {
            let factor = row[pivot];
            if factor != 0 {
                for j in 0..32 { row[j] = subn(row[j], muln(factor, b[j])); }
            }
        }
        let Some(pivot) = (0..31).find(|&j| row[j]!=0) else {
            assert_eq!(row[31], 0, "inconsistent natural relation");
            return false;
        };
        let inverse = pown(row[pivot], N-2);
        for v in &mut row { *v = muln(*v, inverse); }
        for (_, b) in &mut self.rows {
            let factor = b[pivot];
            if factor != 0 {
                for j in 0..32 { b[j] = subn(b[j], muln(factor, row[j])); }
            }
        }
        self.rows.push((pivot,row));
        self.rows.sort_unstable_by_key(|(p,_)|*p);
        true
    }
    fn logs(&self) -> [u64;31] {
        assert_eq!(self.rank(),31);
        let mut logs = [0u64;31];
        for (i, &(pivot, ref row)) in self.rows.iter().enumerate() {
            assert_eq!(pivot,i);
            for j in 0..31 { assert_eq!(row[j],u64::from(j==pivot)); }
            logs[pivot] = row[31];
        }
        logs
    }
}
fn study(num_targets: usize, weights: [u64; 8]) -> Value {
    assert!(num_targets > 0 && num_targets <= 1024);
    assert_eq!(PRIME31.n,N);
    let start = Instant::now();
    let mut ar = Arithmetic::default();
    check_curve(PRIME31,&mut ar).expect("unchanged original prime-order curve");
    let base = canonical_base(&mut ar);
    let oracles: Vec<Oracle> = WINDOWS.iter().copied()
        .map(|w| Oracle::new(&mut ar,&base,w,weights)).collect();
    let setup_ns=start.elapsed().as_nanos();
    let setup_add=ar.counts.group_additions;
    let train_start=Instant::now();
    let mut train_state=SEED ^ TRAIN_XOR;
    let mut elimination=Rref::default();
    let mut accepted=Vec::new();
    let mut window_stats=Vec::new();
    for (wi, oracle) in oracles.iter().enumerate() {
        let desired=[8,16,24,31][wi];
        let mut trials=0usize;
        let mut hits=0usize;
        let mut independent=0usize;
        while trials < 128 && elimination.rank() < desired {
            let k=nonzero_scalar(&mut train_state);
            let q=ar.scalar(PRIME31,k,G);
            trials+=1;
            if let Some(word)=oracle.find(&mut ar,q) {
                hits+=1;
                let row=oracle.row(word,k);
                if elimination.insert(row) {
                    independent+=1;
                    accepted.push(json!({"window":wi,"public_r":q,
                        "known_offset_scalar":k,"word":word,"rank":elimination.rank()}));
                }
            }
        }
        window_stats.push(json!({"window":wi, "trials":trials, "hits":hits,
            "independent":independent, "cumulative_rank":elimination.rank()}));
        assert_eq!(elimination.rank(),desired,
            "window rank gate not met within 128 natural training trials");
    }
    let logs=elimination.logs();
    for (i,&q) in base.iter().enumerate() {
        assert_eq!(ar.scalar(PRIME31,logs[i],G),q,
                   "every learned log must independently replay");
    }
    let train_ns=train_start.elapsed().as_nanos();
    let train_add=ar.counts.group_additions-setup_add;
    let online_start=Instant::now();
    let mut successes=Vec::with_capacity(num_targets);
    let mut online_queries=0usize;
    let mut failed_queries=0usize;
    for t in 0..num_targets {
        // Independent per-target secret stream: the two arms solve IDENTICAL
        // public Q values regardless of different relation-hit/attempt counts.
        let mut secret_state=SEED ^ TARGET_XOR
            ^ (t as u64).wrapping_mul(0xa076_1d64_78bd_642f)
            ^ 0x7365_6372_6574_0001;
        let secret=nonzero_scalar(&mut secret_state);
        let q=ar.scalar(PRIME31,secret,G);
        let mut obtained=None;
        for attempt in 0..32usize {
            let mut offset_state=SEED ^ TARGET_XOR
                ^ (t as u64).wrapping_mul(0xe703_7ed1_a0b4_28db)
                ^ (attempt as u64).wrapping_mul(0x8ebc_6af0_9c88_c6e3)
                ^ 0x6f66_6673_6574_0002;
            let offset=nonzero_scalar(&mut offset_state);
            let wi=(t+attempt)%4;
            let shift_point=ar.scalar(PRIME31,offset,G);
            let translated=ar.add(PRIME31,q,shift_point);
            online_queries+=1;
            if let Some(word)=oracles[wi].find(&mut ar,translated) {
                let row=oracles[wi].row(word,0);
                let mut sum=0u64;
                for i in 0..31 {
                    sum=addn(sum,muln(row[i],logs[i]));
                }
                let recovered=subn(sum,offset);
                assert_eq!(ar.scalar(PRIME31,recovered,G),q);
                assert_eq!(recovered,secret,"fixture-only final check");
                obtained=Some(json!({"target_index":t,"public_q":q,
                    "test_secret":secret,"log":recovered,"offset":offset,
                    "window":wi,"word":word,"failed_prior_queries":attempt}));
                break;
            }
            failed_queries+=1;
        }
        successes.push(obtained.expect("32-offset public relation cap exceeded"));
    }
    let online_ns=online_start.elapsed().as_nanos();
    let online_add=ar.counts.group_additions-setup_add-train_add;
    json!({
        "schema":"prime31.w8.windowed-generic-oracle-full-log.v1",
        "date":"2026-11-18",
        "status":"producer-side [pcc:unverified]",
        "generic_four_plus_four_grader_used":true,
        "genuine_nongeneric_rr9_extractors":0,
        "claim_of_cheaper_nongeneric_solver":false,
        "curve":{"p":P,"a":PRIME31.a,"b":PRIME31.b,"n":N,
                 "cofactor":1,"g":G},
        "weights":weights,"windows":WINDOWS,
        "base_points":base,
        "ordered_words_per_window":16u64.pow(8),
        "mean_ordered_witnesses_over_uniform_group":{
          "numerator":16u64.pow(8),"denominator":N
        },
        "window_training":window_stats,
        "rank":elimination.rank(),
        "accepted_natural_training_relations":accepted,
        "factor_base_logs":logs,
        "target_count":num_targets,
        "online_queries":online_queries,
        "failed_online_queries":failed_queries,
        "recovered_targets":successes,
        "verified_target_count":num_targets,
        "times_ns":{"setup":setup_ns.to_string(),
                    "training_and_log_verification":train_ns.to_string(),
                    "target_recovery_and_verification":online_ns.to_string()},
        "counts":{"setup_ec_additions":setup_add,
                  "training_ec_additions":train_add,
                  "online_ec_additions":online_add,
                  "total_ec_additions":ar.counts.group_additions,
                  "total_fp_multiplies":ar.counts.fp_multiplications,
                  "total_fp_inversions":ar.counts.fp_inversions},
    })
}
fn main() {
    let arm=std::env::var("PRIME31_WEIGHT_ARM").expect("set PRIME31_WEIGHT_ARM=old or ternary");
    let w=match arm.as_str() {
        "old" => OLD_W,
        "ternary" => TERNARY_W,
        _ => panic!("invalid PRIME31_WEIGHT_ARM, expected old or ternary"),
    };
    println!("{}",serde_json::to_string_pretty(&study(1024,w)).unwrap());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_curve_window_rank_and_public_only_oracle_32_targets() {
        let v=study(32,OLD_W);
        assert_eq!(v["rank"],31);
        assert_eq!(v["verified_target_count"],32);
        assert_eq!(v["genuine_nongeneric_rr9_extractors"],0);
        assert_eq!(v["window_training"].as_array().unwrap().len(),4);
    }
    #[test]
    fn ternary_full_rank_and_recovery_32_targets() {
        assert!(signed_formal_injective(&TERNARY_W));
        assert!(!signed_formal_injective(&OLD_W));
        let v=study(32,TERNARY_W);
        assert_eq!(v["rank"],31);
        assert_eq!(v["verified_target_count"],32);
        assert_eq!(v["genuine_nongeneric_rr9_extractors"],0);
    }
    #[test]
    fn exact_first_moment_and_31_dimensional_window_coverage() {
        // Exact formal signed word count: 16^8 distinct vectors for ternary W.
        let avg=(16u128.pow(8) as f64)/(N as f64);
        assert!(avg>2.0 && avg<2.0001);
        let mut union=HashSet::new();
        for w in WINDOWS {
            for idx in w {union.insert(idx);}
        }
        assert_eq!(union.len(),31);
        assert_eq!(WINDOWS[2][7],WINDOWS[3][0]);
    }
}
