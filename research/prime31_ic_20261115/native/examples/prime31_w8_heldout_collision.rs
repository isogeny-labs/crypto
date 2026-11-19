//! Original Prime31 signed-slot weight comparison.
//! This is an out-of-band generic 4+4 GRADER and exact signed-integer
//! collision census, NOT a public-only RR9/F4 extractor or ECDLP solver.
//! Protocol: research/prime31_ic_20261115/rr9/W8_WEIGHT_SIGNATURE_AUDIT_PROTOCOL_20261119.md
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

const P: u64 = 2_147_483_647;
const N: u64 = 2_147_446_831;
const G: Point = Some((625_037_715, 2_100_627_251));
const OLD: [u64; 8] = [1, 3, 5, 7, 11, 13, 17, 19];
const POW2: [u64; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
const WINDOWS: [[usize; 8]; 4] = [
    [0, 1, 2, 3, 4, 5, 6, 7],
    [8, 9, 10, 11, 12, 13, 14, 15],
    [16, 17, 18, 19, 20, 21, 22, 23],
    [23, 24, 25, 26, 27, 28, 29, 30],
];
const SEED: u64 = 0x4845_4c44_4f55_5431;
const TARGETS_PER_WINDOW: usize = 256;
const WORDS_PER_HALF: usize = 65_536;

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
            let k = raw % N;
            if k != 0 { return k; }
        }
    }
}
fn first_x31(ar: &mut Arithmetic) -> Vec<Point> {
    let mut out = Vec::with_capacity(31);
    for x in 0..10_000u64 {
        let rhs = ar.rhs(PRIME31, x);
        if let Some(y) = ar.sqrt(rhs, P) {
            if y != 0 {
                let q = Some((x, y.min(P - y)));
                assert!(ar.on_curve(PRIME31, q));
                out.push(q);
                if out.len() == 31 { break; }
            }
        }
    }
    assert_eq!(out.len(), 31);
    assert_eq!(out.iter().take(8).map(|q| q.unwrap().0).collect::<Vec<_>>(),
        vec![0, 1, 2, 3, 5, 6, 8, 9]);
    out
}
#[derive(Debug)]
struct SignatureStats {
    unique: usize,
    unordered_collision_pairs: u64,
    max_fiber: usize,
}
// Exact signed-INTEGER vectors; no EC arithmetic or accidental group collisions.
fn formal_half(weights: &[u64; 8], begin: usize) -> SignatureStats {
    let mut fibers: HashMap<[i16; 8], usize> = HashMap::new();
    for mut word in 0..WORDS_PER_HALF {
        let mut v = [0i16; 8];
        for j in begin..begin+4 {
            let leaf = word & 15;
            word >>= 4;
            let sign: i16 = if leaf < 8 { 1 } else { -1 };
            v[leaf % 8] += sign * (weights[j] as i16);
        }
        *fibers.entry(v).or_insert(0) += 1;
    }
    SignatureStats {
        unique: fibers.len(),
        unordered_collision_pairs: fibers.values()
            .map(|&m| ((m as u64) * (m as u64 - 1)) / 2).sum(),
        max_fiber: fibers.values().copied().max().unwrap(),
    }
}
fn signature_json(weights: &[u64; 8]) -> Value {
    let l = formal_half(weights, 0);
    let r = formal_half(weights, 4);
    json!({
        "left": {"unique": l.unique, "collision_pairs": l.unordered_collision_pairs,
                 "max_fiber": l.max_fiber},
        "right":{"unique": r.unique, "collision_pairs": r.unordered_collision_pairs,
                 "max_fiber": r.max_fiber},
    })
}
struct Grader {
    left: Vec<Point>,
    right: HashSet<Point>,
    left_unique: usize,
    distinct_unsigned_x: usize,
}
impl Grader {
    fn build(ar: &mut Arithmetic, base: &[Point], window: [usize; 8], w: &[u64; 8]) -> Self {
        let mut slots = Vec::with_capacity(8);
        let mut abscissae = HashSet::with_capacity(64);
        for &weight in w {
            let positive: Vec<Point> = window.iter().map(|&i| {
                let pt = ar.scalar(PRIME31, weight, base[i]);
                assert!(pt.is_some());
                abscissae.insert(pt.unwrap().0);
                pt
            }).collect();
            let mut signed = positive.clone();
            signed.extend(positive.iter().map(|&q| ar.neg(PRIME31, q)));
            assert_eq!(signed.len(), 16);
            slots.push(signed);
        }
        let mut left = Vec::with_capacity(WORDS_PER_HALF);
        let mut right = HashSet::with_capacity(WORDS_PER_HALF);
        for begin in [0, 4] {
            for mut word in 0..WORDS_PER_HALF {
                let mut sum = None;
                for j in begin..begin+4 {
                    let leaf = word & 15;
                    word >>= 4;
                    sum = ar.add(PRIME31, sum, slots[j][leaf]);
                }
                if begin == 0 { left.push(sum); } else { right.insert(sum); }
            }
        }
        let left_unique = left.iter().copied().collect::<HashSet<_>>().len();
        Self {left, right, left_unique, distinct_unsigned_x:abscissae.len()}
    }
    fn contains(&self, ar: &mut Arithmetic, target: Point) -> bool {
        // Complete generic MITM grading membership, NEVER candidate extraction.
        for &s in &self.left {
            let complement = ar.add(PRIME31, target, ar.neg(PRIME31, s));
            if self.right.contains(&complement) { return true; }
        }
        false
    }
}
struct Grade {
    left_unique: usize,
    right_unique: usize,
    distinct_unsigned_x: usize,
    hits: Vec<bool>,
    build_ns: u128,
    probe_ns: u128,
    ec_additions: u64,
}
fn grade_window(base: &[Point], window: [usize; 8], w: &[u64; 8], targets: &[Point]) -> Grade {
    let mut ar = Arithmetic::default();
    let t = Instant::now();
    let grader = Grader::build(&mut ar, base, window, w);
    let build_ns = t.elapsed().as_nanos();
    let u = Instant::now();
    let hits = targets.iter().map(|&q| grader.contains(&mut ar, q)).collect();
    let probe_ns = u.elapsed().as_nanos();
    Grade {
        left_unique: grader.left_unique,
        right_unique: grader.right.len(),
        distinct_unsigned_x: grader.distinct_unsigned_x,
        hits, build_ns, probe_ns,
        ec_additions: ar.counts.group_additions,
    }
}

fn w1_accidental_collision_certificates(base: &[Point]) -> Value {
    // Out-of-band finite-group GRADER and explicit certificates, not an
    // RR9/F4 public-only unknown-index extractor. The formal coefficient
    // map is injective for POW2, so every duplicate EC sum is non-formal.
    let mut ar = Arithmetic::default();
    let mut slots: Vec<Vec<Point>> = Vec::with_capacity(8);
    for &weight in &POW2 {
        let pos: Vec<Point> = WINDOWS[1].iter()
            .map(|&i| ar.scalar(PRIME31, weight, base[i])).collect();
        assert!(pos.iter().all(|q| q.is_some()));
        let mut signed = pos.clone();
        signed.extend(pos.iter().map(|&q| ar.neg(PRIME31, q)));
        slots.push(signed);
    }
    let mut halves = Vec::new();
    for begin in [0usize, 4usize] {
        let mut seen: HashMap<Point, (usize, [i16; 8])> =
            HashMap::with_capacity(WORDS_PER_HALF);
        let mut formal_duplicates = 0usize;
        let mut accidental_duplicates = 0usize;
        let mut examples = Vec::new();
        for word in 0..WORDS_PER_HALF {
            let mut remaining = word;
            let mut sum: Point = None;
            let mut coeff = [0i16; 8];
            for j in begin..(begin + 4) {
                let digit = remaining & 15;
                remaining >>= 4;
                let leaf = digit & 7;
                let sign: i16 = if digit < 8 { 1 } else { -1 };
                coeff[leaf] += sign * POW2[j] as i16;
                sum = ar.add(PRIME31, sum, slots[j][digit]);
            }
            if let Some((first_word, first_coeff)) = seen.get(&sum) {
                if *first_coeff == coeff {
                    formal_duplicates += 1;
                } else {
                    accidental_duplicates += 1;
                    if examples.len() < 40 {
                        examples.push(json!({
                            "half": begin/4,
                            "first_word":first_word,
                            "colliding_word":word,
                            "first_coeff":first_coeff,
                            "colliding_coeff":coeff,
                            "sum":sum
                        }));
                    }
                }
            } else {
                seen.insert(sum, (word, coeff));
            }
        }
        halves.push(json!({
            "half":begin/4,
            "unique_sums":seen.len(),
            "formal_duplicate_words":formal_duplicates,
            "accidental_duplicate_words":accidental_duplicates,
            "all_certificates":examples
        }));
    }
    json!({"window":1,"weights":POW2,"halves":halves,
        "method":"generic EC half-sum witness enumeration; NOT candidate extraction"})
}
fn study(targets_per_window: usize) -> Value {
    assert!((1..=256).contains(&targets_per_window));
    assert_eq!(PRIME31.p, P);
    assert_eq!(PRIME31.n, N);
    let mut cert = Arithmetic::default();
    check_curve(PRIME31, &mut cert).expect("unchanged ordinary cofactor-one curve");
    let base = first_x31(&mut cert);
    let mut seed = SEED;
    // Fixture generation is separate from each grader, which receives Points only.
    // No scalar from this generation is given to Grader::contains.
    let public_targets: Vec<Point> = (0..4*targets_per_window)
        .map(|_| cert.scalar(PRIME31, nonzero_scalar(&mut seed), G)).collect();

    let old_formal = signature_json(&OLD);
    let new_formal = signature_json(&POW2);
    let mut window_rows = Vec::new();
    let (mut old_hits, mut new_hits, mut both, mut old_only, mut new_only, mut neither)
        = (0usize, 0usize, 0usize, 0usize, 0usize, 0usize);
    for (i, window) in WINDOWS.iter().copied().enumerate() {
        let qs = &public_targets[i*targets_per_window..(i+1)*targets_per_window];
        // Alternate arm execution order by window to mitigate order bias.
        let (old,new) = if i % 2 == 0 {
            (grade_window(&base, window, &OLD, qs),
             grade_window(&base, window, &POW2, qs))
        } else {
            let candidate=grade_window(&base, window, &POW2, qs);
            let incumbent=grade_window(&base, window, &OLD, qs);
            (incumbent,candidate)
        };
        let mut overlap = [0usize; 4]; // both, old-only, new-only, neither
        for (&a, &b) in old.hits.iter().zip(&new.hits) {
            let bin = match (a,b) {
                (true,true) => 0, (true,false) => 1,
                (false,true) => 2, (false,false) => 3,
            };
            overlap[bin] += 1;
        }
        both += overlap[0]; old_only += overlap[1];
        new_only += overlap[2]; neither += overlap[3];
        old_hits += old.hits.iter().filter(|&&b|b).count();
        new_hits += new.hits.iter().filter(|&&b|b).count();
        window_rows.push(json!({
            "window": i, "indices": window,
            "old": {"left_ec_unique": old.left_unique,
                    "right_ec_unique": old.right_unique,
                    "distinct_unsigned_x": old.distinct_unsigned_x,
                    "hits": old.hits.iter().filter(|&&b|b).count(),
                    "build_ns":old.build_ns.to_string(),
                    "probe_ns":old.probe_ns.to_string(),
                    "ec_additions":old.ec_additions},
            "powers_two": {"left_ec_unique":new.left_unique,
                    "right_ec_unique":new.right_unique,
                    "distinct_unsigned_x":new.distinct_unsigned_x,
                    "hits":new.hits.iter().filter(|&&b|b).count(),
                    "build_ns":new.build_ns.to_string(),
                    "probe_ns":new.probe_ns.to_string(),
                    "ec_additions":new.ec_additions},
            "paired": {"both":overlap[0],"old_only":overlap[1],
                       "new_only":overlap[2],"neither":overlap[3]}
        }));
    }
    json!({
        "schema":"prime31.w8.heldout-collision.v1",
        "date":"2026-11-19",
        "curve":{"p":P,"a":PRIME31.a,"b":PRIME31.b,"n":N,
                 "cofactor":1,"generator":G},
        "old_weights":OLD,"candidate_weights":POW2,
        "base_x": base.iter().map(|q|q.unwrap().0).collect::<Vec<_>>(),
        "formal_old":old_formal, "formal_powers_two":new_formal,
        "targets_seed":format!("{SEED:016x}"),
        "arm_order_by_window":["old-first","powers-two-first","old-first","powers-two-first"],
        "w1_power2_collision_certificates":w1_accidental_collision_certificates(&base),
        "target_count":public_targets.len(),
        "per_window":window_rows,
        "paired_total":{"old_hits":old_hits,"powers_two_hits":new_hits,
                        "both":both,"old_only":old_only,"new_only":new_only,
                        "neither":neither},
        "generic_four_plus_four_grader_only":true,
        "public_only_algebraic_extractor_implemented":false,
        "complete_ecdlp_solved":false,
        "whole_solver_benchmark":false,
        "pcc_verified":false
    })
}
fn main() {
    println!("{}",serde_json::to_string_pretty(&study(TARGETS_PER_WINDOW)).unwrap());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn formal_weight_collision_counts() {
        for (w, expected_unique, expected_pairs, expected_max) in [
            (OLD, 64_737, 904, 16),
            (POW2, 65_536, 0, 1),
        ] {
            for begin in [0usize,4usize] {
                let s=formal_half(&w,begin);
                assert_eq!(s.unique,expected_unique);
                assert_eq!(s.unordered_collision_pairs,expected_pairs);
                assert_eq!(s.max_fiber,expected_max);
            }
        }
    }
    #[test]
    fn w1_power_two_nonformal_witnesses() {
        let mut ar=Arithmetic::default();
        let base=first_x31(&mut ar);
        let witnesses=w1_accidental_collision_certificates(&base);
        assert_eq!(witnesses["halves"][0]["unique_sums"],65_496);
        assert_eq!(witnesses["halves"][1]["unique_sums"],65_496);
        for h in witnesses["halves"].as_array().unwrap() {
            assert_eq!(h["formal_duplicate_words"],0);
            assert_eq!(h["accidental_duplicate_words"],40);
            assert!(h["all_certificates"].as_array().unwrap().len()>=8);
        }
    }
    #[test]
    fn prime31_original_curve_two_arms_same_public_queries() {
        let j = study(2);
        assert_eq!(j["target_count"],8);
        assert_eq!(j["formal_old"]["left"]["unique"],64_737);
        assert_eq!(j["formal_powers_two"]["right"]["unique"],65_536);
        for x in j["per_window"].as_array().unwrap() {
            assert_eq!(x["old"]["distinct_unsigned_x"],64);
            assert_eq!(x["powers_two"]["distinct_unsigned_x"],64);
            // Formal signatures upper bound EC support. Extra collisions
            // between *different* vectors are possible in this finite group
            // and must be measured, not asserted away as in a random model.
            for (arm, formal) in [("old",64_737u64),("powers_two",65_536u64)] {
                for half in ["left_ec_unique","right_ec_unique"] {
                    let observed=x[arm][half].as_u64().unwrap();
                    assert!(observed <= formal);
                    assert!(observed >= formal - 64);
                }
            }
        }
    }
}
