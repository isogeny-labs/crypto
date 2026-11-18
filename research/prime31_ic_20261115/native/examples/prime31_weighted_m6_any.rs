//! An existence/any-witness M6 3+3 generic-IC control, using chunked inversion.
//! Runs on the original cofactor-one Prime31 elliptic curve, not a weak curve.
//! Reuses the native prime31_algebraic_ic Arithmetic/Point/PRIME31 implementation.
//! Uses GENERIC three-sum enumeration and is NOT a nonenumerative extractor.
//! All benchmark/PCC claims are development-only [pcc:unverified].
//!
//!    cargo +1.100.0 test --release --locked --example prime31_weighted_m6_complete \
//!      --manifest-path research/prime31_ic_20261115/native/Cargo.toml
//!    cargo +1.100.0 run --release --locked --example prime31_weighted_m6_complete \
//!      --manifest-path research/prime31_ic_20261115/native/Cargo.toml
use prime31_algebraic_ic::{check_curve, Arithmetic, Counts, Point, PRIME31};
use serde_json::json;
use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::time::Instant;

const WEIGHTS: [u64; 6] = [1, 3, 5, 7, 11, 13];
const TRAIN_SEED: u64 = 0x4d36_5452_4149_4e31;
const TARGET_SEED: u64 = 0x4d36_5441_5247_4554;
const TRAIN_CAP: usize = 512;
const TARGETS: usize = 1024;
const SHIFT_CAP: usize = 64;
type Word3 = [u8; 3];
type Word6 = [u8; 6];
type Row = [u64; 32];

fn splitmix64(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn first_rational_points(ar: &mut Arithmetic, b: usize) -> Vec<Point> {
    let mut out = Vec::with_capacity(b);
    for x in 0..PRIME31.p {
        let rhs = ar.rhs(PRIME31, x);
        if let Some(y) = ar.sqrt(rhs, PRIME31.p) {
            assert_ne!(y, 0);
            out.push(Some((x, y.min(PRIME31.p - y))));
            if out.len() == b { return out; }
        }
    }
    panic!("insufficient rational points");
}

fn slot_points(ar: &mut Arithmetic, base: &[Point]) -> [Vec<Point>; 6] {
    std::array::from_fn(|i| base.iter().map(|&p| ar.scalar(PRIME31, WEIGHTS[i], p)).collect())
}

fn eval(ar: &mut Arithmetic, slots: &[Vec<Point>; 6], word: &Word6) -> Point {
    let mut acc = None;
    for i in 0..6 {
        acc = ar.add(PRIME31, acc, slots[i][word[i] as usize]);
    }
    acc
}

fn half(ar: &mut Arithmetic, slots: &[Vec<Point>; 6], from: usize) -> Vec<(Point, Word3)> {
    let b = slots[0].len();
    assert!(b > 0 && b < 256);
    let mut pair = Vec::with_capacity(b * b);
    for i in 0..b {
        for j in 0..b {
            pair.push((ar.add(PRIME31, slots[from][i], slots[from + 1][j]), i as u8, j as u8));
        }
    }
    let mut odd: HashMap<Point, Word3> = HashMap::with_capacity(b * b * b);
    for &(sum, i, j) in &pair {
        for k in 0..b {
            let pt = ar.add(PRIME31, sum, slots[from + 2][k]);
            match odd.entry(pt) {
                Entry::Vacant(e) => { e.insert([i, j, k as u8]); }
                Entry::Occupied(e) => { e.remove(); }
            }
        }
    }
    let mut v: Vec<_> = odd.into_iter().collect();
    v.sort_unstable_by_key(|(pt, _)| *pt); // fixed witness ordering, independent of hash randomization
    v
}

fn half_any(ar: &mut Arithmetic, slots: &[Vec<Point>; 6], from: usize) -> Vec<(Point, Word3)> {
    let b = slots[0].len();
    assert!(b > 0 && b < 256);
    let mut pair = Vec::with_capacity(b * b);
    for i in 0..b {
        for j in 0..b {
            pair.push((ar.add(PRIME31, slots[from][i], slots[from + 1][j]), i as u8, j as u8));
        }
    }
    let mut any: HashMap<Point, Word3> = HashMap::with_capacity(b * b * b);
    for &(sum, i, j) in &pair {
        for k in 0..b {
            let pt = ar.add(PRIME31, sum, slots[from + 2][k]);
            match any.entry(pt) {
                Entry::Vacant(e) => { e.insert([i, j, k as u8]); }
                Entry::Occupied(_) => {}
            }
        }
    }
    let mut v: Vec<_> = any.into_iter().collect();
    v.sort_unstable_by_key(|(pt, _)| *pt); // fixed witness ordering, independent of hash randomization
    v
}

fn query(ar: &mut Arithmetic, left: &[(Point, Word3)],
         right: &HashMap<Point, Word3>, target: Point) -> Option<Word6> {
    let mut parity = false;
    let mut witness = None;
    for &(lp, lw) in left {
        let minus_lp = ar.neg(PRIME31, lp);
        let complement = ar.add(PRIME31, target, minus_lp);
        if let Some(&rw) = right.get(&complement) {
            parity = !parity;
            witness = Some([lw[0], lw[1], lw[2], rw[0], rw[1], rw[2]]);
        }
    }
    if parity { witness } else { None }
}

// Classical Montgomery batched inversion over all nonexceptional complements R-L_i.
// Count native field multiplications, field inversions and group additions explicitly.
// Every candidate result is replayed against the original group arithmetic in the caller.
fn counted_mul(ar: &mut Arithmetic, x: u64, y: u64) -> u64 {
    ar.counts.fp_multiplications += 1;
    ((x as u128 * y as u128) % PRIME31.p as u128) as u64
}
fn counted_pow(ar: &mut Arithmetic, mut x: u64, mut k: u64) -> u64 {
    let mut result = 1;
    while k > 0 {
        if k & 1 != 0 { result = counted_mul(ar, result, x); }
        x = counted_mul(ar, x, x);
        k >>= 1;
    }
    result
}
fn query_any_chunk(ar: &mut Arithmetic, left: &[(Point, Word3)],
                 right: &HashMap<Point, Word3>, target: Point) -> Option<Word6> {
    let p = PRIME31.p;
    let mut complements = vec![None; left.len()];
    let mut denoms = Vec::<u64>::with_capacity(left.len());
    let mut offsets = Vec::<(usize, u64, u64, u64, u64)>::with_capacity(left.len());
    let mut prefix = Vec::<u64>::with_capacity(left.len()+1);
    prefix.push(1);
    for (i, &(lp, _)) in left.iter().enumerate() {
        ar.counts.group_additions += 1;
        match (target, lp) {
            (None, _) => complements[i] = ar.neg(PRIME31, lp),
            (_, None) => complements[i] = target,
            (Some((x, y)), Some((z, w))) => {
                let neg_w = (p - w) % p;
                if x == z && (y + neg_w) % p == 0 {
                    complements[i] = None;
                    continue;
                }
                let (numer, denom) = if x == z {
                    ((3 * counted_mul(ar, x, x) + PRIME31.a) % p, (2 * y) % p)
                } else {
                    ((neg_w + p - y) % p, (z + p - x) % p)
                };
                assert_ne!(denom, 0, "original odd-order curve cannot have exceptional zero denominator");
                let pr = *prefix.last().expect("unit prefix");
                prefix.push(counted_mul(ar, pr, denom));
                denoms.push(denom);
                offsets.push((i, x, y, z, numer));
            }
        }
    }
    if !denoms.is_empty() {
        ar.counts.fp_inversions += 1;
        let mut u = counted_pow(ar, *prefix.last().unwrap(), p - 2);
        for j in (0..denoms.len()).rev() {
            let inv_d = counted_mul(ar, u, prefix[j]);
            u = counted_mul(ar, u, denoms[j]);
            let (i, x, y, z, numer) = offsets[j];
            let slope = counted_mul(ar, numer, inv_d);
            let xout = (counted_mul(ar, slope, slope) + 2*p - x - z) % p;
            let yout = (counted_mul(ar, slope, (x + p - xout) % p) + p - y) % p;
            complements[i] = Some((xout, yout));
        }
        debug_assert_eq!(u, 1, "batch inverse returned initial unit");
    }
    for (i, &complement) in complements.iter().enumerate() {
        if let Some(&rw) = right.get(&complement) {
            let lw = left[i].1;
            return Some([lw[0], lw[1], lw[2], rw[0], rw[1], rw[2]]);
        }
    }
    None
}

const ANY_CHUNK_SIZE: usize = 256;
fn query_any(ar: &mut Arithmetic, left: &[(Point, Word3)],
             right: &HashMap<Point, Word3>, target: Point) -> Option<Word6> {
    for batch in left.chunks(ANY_CHUNK_SIZE) {
        if let Some(w) = query_any_chunk(ar, batch, right, target) {
            return Some(w);
        }
    }
    None
}


fn exact_b2_any_control(ar: &mut Arithmetic, base: &[Point]) {
    let slots = slot_points(ar, &base[..2]);
    let left = half_any(ar, &slots, 0);
    let right: HashMap<Point, Word3> = half_any(ar, &slots, 3).into_iter().collect();
    let mut support = HashSet::new();
    for mask in 0..64 {
        let mut word = [0u8; 6];
        for (i, w) in word.iter_mut().enumerate() { *w = ((mask >> i) & 1) as u8; }
        support.insert(eval(ar, &slots, &word));
    }
    for &q in &support {
        let w = query_any(ar, &left, &right, q).expect("every ordered-word sum has a witness");
        assert_eq!(eval(ar, &slots, &w), q);
    }
    let mut negatives = 0usize;
    for k in 1..=128 {
        let q = ar.scalar(PRIME31, k, Some(PRIME31.g));
        if !support.contains(&q) {
            assert!(query_any(ar, &left, &right, q).is_none());
            negatives += 1;
        }
    }
    assert!(negatives >= 64);
}


fn mod_mul(x: u64, y: u64) -> u64 {
    ((x as u128 * y as u128) % PRIME31.n as u128) as u64
}
fn mod_sub(x: u64, y: u64) -> u64 { (x + PRIME31.n - y) % PRIME31.n }
fn mod_pow(mut x: u64, mut e: u64) -> u64 {
    let mut z = 1u64;
    while e > 0 {
        if e & 1 != 0 { z = mod_mul(z, x); }
        x = mod_mul(x, x);
        e >>= 1;
    }
    z
}
fn coeffs(word: Word6) -> Row {
    let mut r = [0u64; 32];
    for i in 0..6 {
        let j = word[i] as usize;
        r[j] = (r[j] + WEIGHTS[i]) % PRIME31.n;
    }
    r
}

#[derive(Default)]
struct Elimination { pivots: [Option<Row>; 31], rank: usize }
impl Elimination {
    fn add(&mut self, mut row: Row) -> bool {
        for i in 0..31 {
            if row[i] == 0 { continue; }
            if let Some(p) = self.pivots[i] {
                let t = row[i];
                for j in i..32 { row[j] = mod_sub(row[j], mod_mul(t, p[j])); }
            } else {
                let inv = mod_pow(row[i], PRIME31.n - 2);
                for v in &mut row[i..] { *v = mod_mul(*v, inv); }
                self.pivots[i] = Some(row);
                self.rank += 1;
                return true;
            }
        }
        assert_eq!(row[31], 0, "inconsistent relation rows");
        false
    }
    fn recover(&self) -> Option<[u64; 31]> {
        if self.rank != 31 { return None; }
        let mut logs = [0u64; 31];
        for i in (0..31).rev() {
            let row = self.pivots[i]?;
            let mut v = row[31];
            for j in (i + 1)..31 { v = mod_sub(v, mod_mul(row[j], logs[j])); }
            logs[i] = v;
        }
        Some(logs)
    }
}
fn log_from_word(word: Word6, logs: &[u64; 31]) -> u64 {
    let mut v = 0u64;
    for i in 0..6 {
        v = (v + mod_mul(WEIGHTS[i], logs[word[i] as usize])) % PRIME31.n;
    }
    v
}
fn delta(now: Counts, old: Counts) -> serde_json::Value {
    json!({"group_additions":now.group_additions-old.group_additions,
           "fp_multiplications":now.fp_multiplications-old.fp_multiplications,
           "fp_inversions":now.fp_inversions-old.fp_inversions})
}

fn exact_b2_parity_control(ar: &mut Arithmetic, base: &[Point]) {
    let slots = slot_points(ar, &base[..2]);
    let left = half(ar, &slots, 0);
    let right: HashMap<Point, Word3> = half(ar, &slots, 3).into_iter().collect();
    let mut expected = HashSet::new();
    for wordmask in 0..64 {
        let mut word = [0u8; 6];
        for (i, w) in word.iter_mut().enumerate() { *w = ((wordmask >> i) & 1) as u8; }
        let pt = eval(ar, &slots, &word);
        if !expected.insert(pt) { expected.remove(&pt); }
    }
    let mut got = HashSet::new();
    for &lp in left.iter().map(|v| &v.0) {
        for &rp in right.keys() {
            let target = ar.add(PRIME31, lp, rp);
            if !got.insert(target) { got.remove(&target); }
        }
    }
    assert_eq!(got, expected);
    let mut all_images = HashSet::new();
    for mask in 0..64u64 {
        let mut word = [0u8;6];
        for (i,v) in word.iter_mut().enumerate() { *v = ((mask >> i) & 1) as u8; }
        all_images.insert(eval(ar, &slots, &word));
    }
    for q in all_images {
        let a = query(ar, &left, &right, q);
        let b = query_batched(ar, &left, &right, q);
        assert_eq!(a.is_some(), b.is_some(), "batched parity disagreed on full B2 image");
        if let Some(w) = b { assert_eq!(eval(ar, &slots, &w), q); }
    }
    for k in 1..=64 {
        let q = ar.scalar(PRIME31, k, Some(PRIME31.g));
        assert_eq!(query(ar, &left, &right, q).is_some(),
                   query_batched(ar, &left, &right, q).is_some(),
                   "batched parity disagreed on frozen public negative control");
    }
    // A repeated identical unsigned six-slot base is not the distinct-weight case.
    for q in expected {
        let w = query(ar, &left, &right, q).expect("odd case must return word");
        let wb = query_batched(ar, &left, &right, q).expect("batched odd case must return word");
        assert_eq!(eval(ar, &slots, &w), q);
        assert_eq!(eval(ar, &slots, &wb), q);
    }
}

// Frozen external source corpus is validated by the *caller*, outside solver timing.
// Only public points are ever passed into query_batched().
fn load_frozen_original_1024(path: &str) -> Result<Vec<(Point, u64)>, String> {
    let bytes = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let d: serde_json::Value = serde_json::from_str(&bytes).map_err(|e| e.to_string())?;
    if d["schema"] != "prime31.mixed_4p4_amortized_complete_ic.v1"
        || d["target_count"] != TARGETS
        || d["completed_unknown_target_logs"] != TARGETS
        || d["setup_rank"] != 61
        || d["base_log_point_replays"] != 61
        || d["is_nongeneric_s9_extraction"] != false
    { return Err("source corpus has wrong schema, rank or success controls".into()); }
    let rows = d["targets"].as_array().ok_or("missing source targets")?;
    if rows.len() != TARGETS { return Err("wrong source target count".into()); }
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar)?;
    let mut seen = HashSet::<(u64, u64)>::with_capacity(TARGETS);
    let mut result = Vec::with_capacity(TARGETS);
    for (i, row) in rows.iter().enumerate() {
        if row["solver"]["case"].as_u64() != Some(i as u64)
            || row["solver"]["status"] != "complete" || row["caller_check"] != true
        { return Err(format!("source target {i} lacks a completed caller replay")); }
        let xy: [u64;2] = serde_json::from_value(row["solver"]["public_q"].clone())
            .map_err(|e| format!("invalid original public point row {i}: {e}"))?;
        let k = row["caller_only_scalar"].as_u64()
            .ok_or_else(|| format!("source known-answer missing row {i}"))?;
        let q = Some((xy[0], xy[1]));
        if k == 0 || k >= PRIME31.n || !seen.insert((xy[0], xy[1]))
            || !ar.on_curve(PRIME31, q)
            || ar.scalar(PRIME31, k, Some(PRIME31.g)) != q
        { return Err(format!("invalid, duplicate or false scalar row {i}")); }
        result.push((q,k));
    }
    Ok(result)
}

fn main() {
    let external = std::env::args().nth(1).map(|p| {
        load_frozen_original_1024(&p).expect("invalid frozen original 1,024-Q corpus")
    });
    let total_t0 = Instant::now();
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("original ordinary cofactor-one curve");
    let base = first_rational_points(&mut ar, 31);
    exact_b2_parity_control(&mut ar, &base);
    exact_b2_any_control(&mut ar, &base);
    let setup_before = ar.counts;
    let setup_t0 = Instant::now();
    let slots = slot_points(&mut ar, &base);
    let left = half_any(&mut ar, &slots, 0);
    let right_map: HashMap<_, _> = half_any(&mut ar, &slots, 3).into_iter().collect();
    let setup_ns = setup_t0.elapsed().as_nanos();
    let setup_cost = delta(ar.counts, setup_before);

    let mut train_rng = TRAIN_SEED;
    let mut ranker = Elimination::default();
    let mut train_rows = Vec::new();
    let mut positive = 0usize;
    let training_t0 = Instant::now();
    let training_before = ar.counts;
    let mut calls = 0usize;
    for t in 0..TRAIN_CAP {
        let known_a = 1 + splitmix64(&mut train_rng) % (PRIME31.n - 1);
        let residual = ar.scalar(PRIME31, known_a, Some(PRIME31.g));
        calls += 1;
        if let Some(word) = query_any(&mut ar, &left, &right_map, residual) {
            positive += 1;
            assert_eq!(eval(&mut ar, &slots, &word), residual);
            let mut row = coeffs(word);
            row[31] = known_a;
            let independent = ranker.add(row);
            train_rows.push(json!({"public_residual":residual, "known_query_coefficient":known_a,
                                   "word":word, "independent":independent}));
            if ranker.rank == 31 { break; }
        }
        assert_eq!(t + 1, calls);
    }
    let training_ns = training_t0.elapsed().as_nanos();
    let training_cost = delta(ar.counts, training_before);
    let mut result = json!({
        "protocol":"prime31-weighted-m6-any-chunk256-v1",
        "source_p":PRIME31.p,"source_a":PRIME31.a,"source_b":PRIME31.b,
        "source_n":PRIME31.n,"generator":PRIME31.g,
        "weights":WEIGHTS,"base":base,"left_any_keys":left.len(),"right_any_keys":right_map.len(),
        "theoretical_ordered_mean_b31": format!("{:.12}",(31f64).powi(6)/PRIME31.n as f64),
        "train_seed":format!("{TRAIN_SEED:#018x}"),
        "target_seed":format!("{TARGET_SEED:#018x}"),
        "target_corpus":if external.is_some() {"original_mixed_1024q"} else {"weighted_m6_seed"},
        "external_corpus_validation_excluded_from_solver_timer":true,
        "setup_ns":setup_ns,"setup_cost":setup_cost,
        "train_calls":calls,"train_any":positive,"train_rank":ranker.rank,
        "train_ns":training_ns,"train_cost":training_cost,"training":train_rows,
        "target_limit":TARGETS,"shift_cap":SHIFT_CAP,
        "nonenumerative_algebraic_extractor":false,
        "generic_three_plus_three_enumeration":true,
        "montgomery_batched_complement":true,
        "existence_not_parity":true,"chunk_size":ANY_CHUNK_SIZE,
        "pcc":"unverified"
    });
    if let Some(logs) = ranker.recover() {
        for (p, &ell) in base.iter().zip(logs.iter()) {
            assert_eq!(ar.scalar(PRIME31, ell, Some(PRIME31.g)), *p);
        }
        let query_before = ar.counts;
        let query_t0 = Instant::now();
        let mut rng = TARGET_SEED;
        let mut completed = 0usize;
        let mut targets = Vec::with_capacity(TARGETS);
        let mut total_attempts = 0usize;
        for idx in 0..TARGETS {
            // The caller's expected scalar is NEVER passed to the query oracle.
            let (q, secret_for_verification_only) = if let Some(ref external_rows) = external {
                external_rows[idx]
            } else {
                let secret = 1 + splitmix64(&mut rng) % (PRIME31.n - 1);
                (ar.scalar(PRIME31, secret, Some(PRIME31.g)), secret)
            };
            let mut r = q;
            let mut solved = None;
            for shift in 0..SHIFT_CAP {
                total_attempts += 1;
                if let Some(word) = query_any(&mut ar, &left, &right_map, r) {
                    assert_eq!(eval(&mut ar, &slots, &word), r);
                    let log = mod_sub(log_from_word(word, &logs), shift as u64);
                    assert_eq!(ar.scalar(PRIME31, log, Some(PRIME31.g)), q);
                    assert_eq!(log, secret_for_verification_only);
                    solved = Some(json!({"index":idx,"public_point":q,"recovered":log,
                        "known_scalar_verifier_only":secret_for_verification_only,
                        "shift":shift,"word":word}));
                    completed += 1;
                    break;
                }
                r = ar.add(PRIME31, r, Some(PRIME31.g));
            }
            if let Some(row) = solved { targets.push(row); }
            else { targets.push(json!({"index":idx,"public_point":q,"censored":true})); }
        }
        result["base_logs"] = json!(logs);
        result["targets"] = json!(targets);
        result["completed"] = json!(completed);
        result["target_attempts"] = json!(total_attempts);
        result["query_ns"] = json!(query_t0.elapsed().as_nanos());
        result["query_cost"] = delta(ar.counts, query_before);
    } else {
        result["completed"] = json!(0);
        result["target_attempts"] = json!(0);
        result["censored_reason"] = json!("rank31 not reached within preregistered training cap");
    }
    result["total_ns_diagnostic"] = json!(total_t0.elapsed().as_nanos());
    result["complete_nongeneric_speedup_claim"] = json!(false);
    println!("{}",serde_json::to_string_pretty(&result).expect("json"));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_original_curve_b2_six_weight_parity() {
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).unwrap();
        let b = first_rational_points(&mut ar, 2);
        exact_b2_parity_control(&mut ar, &b);
    }
    #[test]
    fn exact_original_curve_b2_any_witness_and_negative_cases() {
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).unwrap();
        let b = first_rational_points(&mut ar, 2);
        exact_b2_any_control(&mut ar, &b);
    }
    #[test]
    fn frozen_corpus_fails_closed_on_missing_source() {
        assert!(load_frozen_original_1024("/path/that/must/not/exist/for/source-gate.json").is_err());
    }
    #[test]
    fn modular_elimination_and_weighted_coefficients() {
        let mut ranker = Elimination::default();
        for i in 0..31 {
            let mut row=[0u64;32];row[i]=7;row[31]=(7 * (i as u64 + 1))%PRIME31.n;
            assert!(ranker.add(row));
        }
        assert_eq!(ranker.recover().unwrap(),std::array::from_fn(|i|i as u64+1));
        let c=coeffs([0,0,0,0,0,0]);
        assert_eq!(c[0], WEIGHTS.iter().sum::<u64>());
    }
}
