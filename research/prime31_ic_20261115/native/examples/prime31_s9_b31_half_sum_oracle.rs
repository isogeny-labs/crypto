//! Exact signed eight-leaf membership oracle on ordinary prime31 via 4A+4A.
//! This is GENERIC point-sum meet-in-the-middle, a nonenumerative-S9 extractor
//! validation oracle, NOT a non-generic index-calculus breakthrough.
//!
//! Reuses prime31_algebraic_ic::{PRIME31, Arithmetic, check_curve, Point}.
//! Frozen study: S9_B31_FOUR_PLUS_FOUR_ORACLE_PROTOCOL_20261116.md.
//!
//! cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s9_b31_half_sum_oracle
//! cargo +1.100.0 run  --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s9_b31_half_sum_oracle
#![recursion_limit = "512"] // JSON transcript has 1024 case records
use prime31_algebraic_ic::{check_curve, Arithmetic, Base, Point, PRIME31};
use serde_json::json;
use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::time::Instant;

type Word4 = [u8; 4];
type Word8 = [u8; 8];
type Signature = [i8; 31];

const B31: usize = 31;
const NATURAL_SEED: u64 = 0x5339_4233_314e_4154;
const PLANTED_SEED: u64 = 0x5339_4233_3150_4c54;

// Every even eight-leaf net signature can be padded by opposite-sign pairs.
// These signed four-leaf counts are purely combinatorial upper bounds:
// the actual group image may have additional point collisions.
fn signature_count(base: usize, arity: usize) -> u128 {
    let mut row = vec![0_u128; arity + 1];
    row[0] = 1;
    for _ in 0..base {
        let mut next = vec![0_u128; arity + 1];
        for (d, &v) in row.iter().enumerate() {
            next[d] += v;
            for k in 1..=(arity - d) {
                next[d + k] += 2 * v;
            }
        }
        row = next;
    }
    row.iter()
        .enumerate()
        .filter(|(k, _)| k % 2 == arity % 2)
        .map(|(_, &n)| n)
        .sum()
}

fn choose(n: usize, k: usize) -> u128 {
    let k = k.min(n - k);
    (1..=k).fold(1_u128, |a, i| a * (n - k + i) as u128 / i as u128)
}

fn first_rational_points(ar: &mut Arithmetic, base: usize) -> Vec<(u64, u64)> {
    let mut out = Vec::new();
    for x in 0..PRIME31.p {
        let rhs = ar.rhs(PRIME31, x);
        if let Some(y) = ar.sqrt(rhs, PRIME31.p) {
            assert_ne!(y, 0, "cofactor-one odd-order curve has no order-two point");
            out.push((x, y.min(PRIME31.p - y)));
            if out.len() == base {
                return out;
            }
        }
    }
    panic!("not enough rational x values");
}

// Token 2i denotes P_i; token 2i+1 denotes -P_i.
fn tokens(ar: &Arithmetic, xs: &[(u64, u64)]) -> Vec<Point> {
    let mut out = Vec::with_capacity(2 * xs.len());
    for &p in xs {
        out.push(Some(p));
        out.push(ar.neg(PRIME31, Some(p)));
    }
    out
}

fn word_signature(word: Word4) -> Signature {
    let mut out = [0_i8; B31];
    for token in word {
        let i = (token / 2) as usize;
        out[i] += if token % 2 == 0 { 1 } else { -1 };
    }
    out
}

fn evaluate(ar: &mut Arithmetic, pts: &[Point], word: &Word8) -> Point {
    let mut out = None;
    for &t in word {
        out = ar.add(PRIME31, out, pts[t as usize]);
    }
    out
}

struct FourSet {
    index: HashMap<Point, Word4>,
    ordered: Vec<Point>,
    words: u64,
    integer_signatures: usize,
    build_additions: u64,
    build_ns: u128,
}

impl FourSet {
    fn build(ar: &mut Arithmetic, pts: &[Point]) -> Self {
        let start = Instant::now();
        let before = ar.counts.group_additions;
        let n = pts.len();
        assert!(n > 0 && n <= 2 * B31 && n <= u8::MAX as usize);
        let mut pairs = vec![None; n * n];
        for i in 0..n {
            for j in i..n {
                let v = ar.add(PRIME31, pts[i], pts[j]);
                pairs[i * n + j] = v;
                pairs[j * n + i] = v;
            }
        }
        let mut index = HashMap::<Point, Word4>::with_capacity(signature_count(n / 2, 4) as usize);
        let mut ordered = Vec::with_capacity(signature_count(n / 2, 4) as usize);
        let mut net = HashSet::<Signature>::with_capacity(signature_count(n / 2, 4) as usize);
        let mut words = 0_u64;
        for i in 0..n {
            for j in i..n {
                let left = pairs[i * n + j];
                for k in j..n {
                    for l in k..n {
                        let word = [i as u8, j as u8, k as u8, l as u8];
                        let q = ar.add(PRIME31, left, pairs[k * n + l]);
                        words += 1;
                        net.insert(word_signature(word));
                        if let Entry::Vacant(e) = index.entry(q) {
                            e.insert(word);
                            ordered.push(q);
                        }
                    }
                }
            }
        }
        assert_eq!(words as u128, choose(n + 3, 4));
        assert_eq!(net.len() as u128, signature_count(n / 2, 4));
        assert_eq!(index.len(), ordered.len());
        let build_ns = start.elapsed().as_nanos();
        Self {
            index,
            ordered,
            words,
            integer_signatures: net.len(),
            build_additions: ar.counts.group_additions - before,
            build_ns,
        }
    }

    // Exactly decides Q in 8A. No expected-hit assumption or capped scan.
    // A negative is certified by an exhaustive sweep over distinct 4A.
    fn recover(&self, ar: &mut Arithmetic, pts: &[Point], q: Point) -> (Option<Word8>, usize) {
        for (i, &left) in self.ordered.iter().enumerate() {
            let opposite_left = ar.neg(PRIME31, left);
            let missing = ar.add(PRIME31, q, opposite_left);
            if let Some(&right) = self.index.get(&missing) {
                let w = self.index.get(&left).expect("index/list invariant");
                let witness = [
                    w[0], w[1], w[2], w[3],
                    right[0], right[1], right[2], right[3],
                ];
                assert_eq!(evaluate(ar, pts, &witness), q, "original curve replay");
                return (Some(witness), i + 1);
            }
        }
        (None, self.ordered.len())
    }
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn describe(ar: &mut Arithmetic, half: &FourSet, pts: &[Point], q: Point,
            label: &str, case: usize, caller_scalar: Option<u64>)
    -> serde_json::Value
{
    let start = Instant::now();
    let before = ar.counts.group_additions;
    let (witness, probes) = half.recover(ar, pts, q);
    let lookup_ns = start.elapsed().as_nanos();
    let group_additions = ar.counts.group_additions - before;
    json!({
        "type":label, "case":case, "target_point":q,
        "caller_only_scalar_not_supplied_to_oracle":caller_scalar,
        "found":witness.is_some(), "eight_signed_token_witness":witness,
        "four_set_probes":probes, "lookup_group_additions_including_replay":group_additions,
        "lookup_ns_diagnostic_only":lookup_ns,
        "fully_exhausted_on_absence":witness.is_none() && probes == half.ordered.len(),
        "full_original_curve_sum_replay":witness.is_some(),
    })
}

fn run_panel(ar: &mut Arithmetic, base: Vec<(u64, u64)>, base_kind: &str) -> serde_json::Value {
    assert_eq!(base.len(), B31, "exact 31-point x factor base");
    let pts = tokens(ar, &base);
    let half = FourSet::build(ar, &pts);
    assert_eq!(half.integer_signatures, 618_885);
    let mut rows = Vec::new();
    let mut natural_seed = NATURAL_SEED;
    let mut seen = HashSet::new();
    let mut natural_hits = 0_u64;
    for case in 0..24 {
        let k = loop {
            let candidate = 1 + splitmix64(&mut natural_seed) % (PRIME31.n - 1);
            if seen.insert(candidate) { break candidate; }
        };
        let q = ar.scalar(PRIME31, k, Some(PRIME31.g));
        let record = describe(ar, &half, &pts, q, "natural_unknown_k_for_oracle", case, Some(k));
        natural_hits += u64::from(record["found"].as_bool().unwrap());
        rows.push(record);
    }
    let mut planted_seed = PLANTED_SEED;
    for case in 0..16 {
        let mut word = [0_u8; 8];
        for t in &mut word { *t = (splitmix64(&mut planted_seed) % pts.len() as u64) as u8; }
        let q = evaluate(ar, &pts, &word);
        let record = describe(ar, &half, &pts, q, "planted_blinded_eight_sum", case, None);
        assert_eq!(record["found"], true, "planted positive missed");
        rows.push(record);
    }
    let identity_word = [0_u8, 1, 0, 1, 0, 1, 0, 1];
    assert_eq!(evaluate(ar, &pts, &identity_word), None);
    let identity = describe(ar, &half, &pts, None, "identity_cancellation", 0, None);
    assert_eq!(identity["found"], true);
    rows.push(identity);
    json!({
        "base_kind":base_kind,
        "first_rational_x_base_or_special_fibre":base,
        "base_size":B31,
        "signed_four_word_multisets":half.words,
        "signed_four_net_integer_signatures":half.integer_signatures,
        "distinct_four_leaf_group_points":half.index.len(),
        "extra_collisions_of_distinct_net_signatures":half.integer_signatures-half.index.len(),
        "four_word_duplicate_representations":half.words-half.integer_signatures as u64,
        "four_set_build_ns_diagnostic_only":half.build_ns.to_string(),
        "four_set_build_group_additions":half.build_additions,
        "natural_seed":format!("{NATURAL_SEED:#x}"),
        "planted_seed":format!("{PLANTED_SEED:#x}"),
        "natural_cases":24, "natural_eight_leaf_hits":natural_hits,
        "planted_cases":16, "planted_hits":16,
        "identity_control_hits":1,
        "per_target":rows
    })
}

fn main() {
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("valid ordinary prime31 group");
    let first_base = first_rational_points(&mut ar, B31);
    let first_x = run_panel(&mut ar, first_base, "first31_rational_x_control");

    // The actual K961/S5 bridge uses the 31 liftable points in Base depth six.
    // Do not confuse coverage on an arbitrary first-x base with this special
    // algebraic factor base.
    let dickson = Base::build(PRIME31, 6, &mut ar).expect("special depth-six factor base");
    assert_eq!(dickson.points.len(), B31);
    let special = run_panel(&mut ar, dickson.points, "actual_dickson_depth6_31_algebraic_base");

    println!("{}", json!({
        "schema":"prime31.s9.b31_four_plus_four_oracle.v2",
        "curve":PRIME31,
        "algorithm":"generic group-law signed 4A+4A MITM; exact 8A membership oracle, not S9 polynomial elimination or ECDLP solver",
        "natural_and_planted_public_Q_seeds_match_across_bases":true,
        "panels":[first_x,special],
        "cold_timing_comparable_to_APS_or_rho":false,
        "independent_signed_pcc":false,
        "status":"development producer [pcc:unverified]"
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_integer_signature_and_word_counts() {
        assert_eq!(signature_count(4, 4), 225);
        assert_eq!(signature_count(8, 4), 2_945);
        assert_eq!(signature_count(31, 4), 618_885);
        assert_eq!(signature_count(31, 8), 5_654_193_417);
        assert_eq!(choose(65, 4), 677_040);
        assert_eq!(choose(38, 8), 48_903_492);
    }

    #[test]
    fn b4_b8_exact_four_leaf_images_and_recovered_witnesses() {
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).unwrap();
        for (b, expected) in [(4, 225), (8, 2_945)] {
            let base = first_rational_points(&mut ar, b);
            let pts = tokens(&ar, &base);
            let half = FourSet::build(&mut ar, &pts);
            assert_eq!(half.index.len(), expected);
            let repeated = [0_u8; 8];
            let q = evaluate(&mut ar, &pts, &repeated);
            assert!(half.recover(&mut ar, &pts, q).0.is_some());
            assert!(half.recover(&mut ar, &pts, None).0.is_some());
            let cancel = [0_u8, 1, 0, 1, 0, 1, 0, 1];
            assert_eq!(evaluate(&mut ar, &pts, &cancel), None);
        }
    }
}


// Prospective public-residual holdout gate. This is a GENERIC 4A+4A
// membership control, not an algebraic/non-generic extractor.
#[cfg(test)]
mod adaptive_b31_tests {
    use super::*;

    const HOLDOUT_SEED: u64 = 0x5339_4248_4f4c_4431;
    const N_HOLDOUT: usize = 96;
    const MAX_SHIFT: u64 = 8;

    fn probe_adaptive(
        ar: &mut Arithmetic,
        half: &FourSet,
        pts: &[Point],
        q: Point,
        case: usize,
        panel: &str,
        caller_only_k: u64,
    ) -> serde_json::Value {
        let mut attempts = Vec::new();
        let mut first_successful_shift: Option<u64> = None;
        for a in 0..=MAX_SHIFT {
            let before_additions = ar.counts.group_additions;
            let before_fp_mults = ar.counts.fp_multiplications;
            let before_fp_inversions = ar.counts.fp_inversions;
            let start = Instant::now();
            // Only Q and the public [a]G are input to membership.
            let delta = ar.scalar(PRIME31, a, Some(PRIME31.g));
            let r = ar.add(PRIME31, q, delta);
            let (witness, probes) = half.recover(ar, pts, r);
            let elapsed_ns = start.elapsed().as_nanos();
            assert!(probes > 0 && probes <= half.ordered.len());
            if witness.is_none() {
                assert_eq!(probes, half.ordered.len(), "negative must exhaust 4A");
            }
            // recover already replays all eight original-curve signed leaves.
            attempts.push(json!({
                "public_shift_a": a,
                "residual": r,
                "membership_found": witness.is_some(),
                "signed_eight_token_witness": witness,
                "four_set_probes": probes,
                "exhaustive_negative": witness.is_none() && probes == half.ordered.len(),
                "native_group_additions": ar.counts.group_additions - before_additions,
                "native_fp_multiplications": ar.counts.fp_multiplications - before_fp_mults,
                "native_fp_inversions": ar.counts.fp_inversions - before_fp_inversions,
                "elapsed_ns_diagnostic": elapsed_ns.to_string()
            }));
            if witness.is_some() {
                first_successful_shift = Some(a);
                break;
            }
        }
        json!({
            "panel": panel,
            "case": case,
            "caller_only_known_k_not_passed_to_oracle": caller_only_k,
            "public_target_q": q,
            "first_successful_shift": first_successful_shift,
            "attempt_count": attempts.len(),
            "attempts": attempts
        })
    }

    #[test]
    fn adaptive_dickson_b31_blinded_holdout() {
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).expect("original ordinary cofactor-one prime31");
        let full_study_start = Instant::now();
        let special = Base::build(PRIME31, 6, &mut ar).expect("Dickson depth-six base");
        assert_eq!(special.points.len(), B31);
        let pts = tokens(&ar, &special.points);
        let half = FourSet::build(&mut ar, &pts);
        assert_eq!(half.index.len(), 618_885);
        assert_eq!(half.integer_signatures, 618_885);

        let mut state = HOLDOUT_SEED;
        let mut distinct_k = HashSet::<u64>::new();
        let mut holdout = Vec::new();
        for case in 0..N_HOLDOUT {
            let k = loop {
                let candidate = 1 + splitmix64(&mut state) % (PRIME31.n - 1);
                if distinct_k.insert(candidate) {
                    break candidate;
                }
            };
            let q = ar.scalar(PRIME31, k, Some(PRIME31.g));
            assert!(ar.on_curve(PRIME31, q));
            holdout.push(probe_adaptive(
                &mut ar, &half, &pts, q, case, "fresh_blinded_public_target", k,
            ));
        }
        assert_eq!(distinct_k.len(), N_HOLDOUT);

        let mut legacy_misses = Vec::new();
        for (case, k) in [1_610_011_808_u64, 1_264_623_843_u64]
            .into_iter()
            .enumerate()
        {
            let q = ar.scalar(PRIME31, k, Some(PRIME31.g));
            let record = probe_adaptive(
                &mut ar, &half, &pts, q, case, "legacy_exact_a0_nonmembership", k,
            );
            assert_eq!(record["attempts"][0]["public_shift_a"], 0);
            assert_eq!(record["attempts"][0]["membership_found"], false);
            assert_eq!(record["attempts"][0]["exhaustive_negative"], true);
            legacy_misses.push(record);
        }
        let fresh_a0 = holdout.iter()
            .filter(|v| v["first_successful_shift"].as_u64() == Some(0))
            .count();
        let fresh_shifted = holdout.iter()
            .filter(|v| v["first_successful_shift"].as_u64().is_some_and(|a| a > 0))
            .count();
        let fresh_unresolved = holdout.iter()
            .filter(|v| v["first_successful_shift"].is_null())
            .count();
        let total_fresh_attempts: usize = holdout.iter()
            .map(|v| v["attempt_count"].as_u64().unwrap() as usize)
            .sum();
        assert_eq!(fresh_a0 + fresh_shifted + fresh_unresolved, N_HOLDOUT);
        assert!(total_fresh_attempts >= N_HOLDOUT);
        assert!(total_fresh_attempts <= N_HOLDOUT * (MAX_SHIFT as usize + 1));

        // Full raw trial transcript for later sign-free S9 comparison, not
        // a solved unknown-k ECDLP result.
        println!(
            "DICKSON31_ADAPTIVE_HOLDOUT_JSON={}",
            json!({
                "schema": "prime31.dickson31.adaptive_public_8a_control.v1",
                "curve_p": PRIME31.p,
                "curve_n": PRIME31.n,
                "factor_base": "actual Dickson depth-six B31",
                "signed_four_group_points": half.index.len(),
                "four_set_build_ns_diagnostic": half.build_ns.to_string(),
                "four_set_build_group_additions": half.build_additions,
                "four_set_index_capacity_entries": half.index.capacity(),
                "four_set_ordered_capacity_entries": half.ordered.capacity(),
                "full_test_wall_ns_diagnostic": full_study_start.elapsed().as_nanos().to_string(),
                "holdout_seed": format!("{HOLDOUT_SEED:#018x}"),
                "fresh_target_count": N_HOLDOUT,
                "public_shift_max_inclusive": MAX_SHIFT,
                "fresh_a0_hits": fresh_a0,
                "fresh_shifted_hits": fresh_shifted,
                "fresh_unresolved_at_cap": fresh_unresolved,
                "fresh_total_membership_attempts": total_fresh_attempts,
                "legacy_known_a0_misses_count": legacy_misses.len(),
                "fresh_cases": holdout,
                "legacy_cases": legacy_misses,
                "generic_group_sum_mitm_only": true,
                "sign_free_algebraic_extraction": false,
                "complete_unknown_ecdlp_logs": 0,
                "full_cost_ecdlp_benchmark": false,
                "pcc": "unverified"
            })
        );
    }
}


// Matched arithmetic-control follow-up to the parent Dickson31 98-Q cohort.
// This changes ONLY how the original generic 4A complement search computes R-L.
// Montgomery simultaneous inversion is established prior art, NOT a new
// non-generic decomposition or ECDLP method.
#[cfg(test)]
mod batch_complement_tests {
    use super::*;

    const WINDOW: usize = 512;
    const HOLDOUT_SEED: u64 = 0x5339_4248_4f4c_4431;

    #[derive(Default, Clone, Copy, Debug)]
    pub(super) struct BatchStats {
        windows: u64,
        processed: u64,
        regular: u64,
        special: u64,
        identity: u64,
        inversions: u64,
        multiplications: u64,
        replayed: u64,
    }

    impl BatchStats {
        fn add(&mut self, rhs: Self) {
            self.windows += rhs.windows;
            self.processed += rhs.processed;
            self.regular += rhs.regular;
            self.special += rhs.special;
            self.identity += rhs.identity;
            self.inversions += rhs.inversions;
            self.multiplications += rhs.multiplications;
            self.replayed += rhs.replayed;
        }
    }

    #[inline]
    fn mul(a: u64, b: u64, stats: &mut BatchStats) -> u64 {
        stats.multiplications += 1;
        ((a as u128 * b as u128) % PRIME31.p as u128) as u64
    }

    fn pow(mut base: u64, mut exp: u64, stats: &mut BatchStats) -> u64 {
        let mut out = 1_u64;
        while exp > 0 {
            if exp & 1 != 0 {
                out = mul(out, base, stats);
            }
            base = mul(base, base, stats);
            exp >>= 1;
        }
        out
    }

    #[inline]
    fn sub(a: u64, b: u64) -> u64 {
        (a + PRIME31.p - b) % PRIME31.p
    }

    // One inversion for all nonzero x-differences in this window. The fallback
    // is the ORIGINAL native arithmetic for equal-x, so tangent, inverse,
    // 2-torsion and identity branches are never silently discarded.
    fn complement_window(
        ar: &mut Arithmetic,
        residual: Point,
        left: &[Point],
        stats: &mut BatchStats,
    ) -> Vec<Point> {
        stats.windows += 1;
        stats.processed += left.len() as u64;
        let mut answers: Vec<Point> = vec![None; left.len()];
        let mut positions = Vec::with_capacity(left.len());
        let mut denoms = Vec::with_capacity(left.len());
        let mut prefixes = Vec::with_capacity(left.len());
        let mut running = 1_u64;

        for (i, &point) in left.iter().enumerate() {
            match (residual, point) {
                (Some((rx, _ry)), Some((lx, _ly))) if rx != lx => {
                    let denominator = sub(rx, lx);
                    assert_ne!(denominator, 0);
                    positions.push(i);
                    denoms.push(denominator);
                    prefixes.push(running);
                    running = mul(running, denominator, stats);
                    stats.regular += 1;
                }
                (Some(_), Some(_)) => {
                    let inverse_point = ar.neg(PRIME31, point);
                    answers[i] = ar.add(PRIME31, residual, inverse_point);
                    stats.special += 1;
                }
                _ => {
                    // R-O = R; O-L = -L, both with zero field inversions.
                    answers[i] = if residual.is_none() {
                        ar.neg(PRIME31, point)
                    } else {
                        residual
                    };
                    stats.identity += 1;
                }
            }
        }

        if !positions.is_empty() {
            stats.inversions += 1;
            let mut inverse_product = pow(running, PRIME31.p - 2, stats);
            // Reverse prefix products recover every inverse exactly:
            // inv(d_i) = prefix_i * inv(product_all), then divide product by d_i.
            for j in (0..positions.len()).rev() {
                let reciprocal = mul(prefixes[j], inverse_product, stats);
                inverse_product = mul(inverse_product, denoms[j], stats);
                let i = positions[j];
                let (rx, ry) = residual.expect("regular finite target");
                let (lx, ly) = left[i].expect("regular finite left half");
                // R + (-L): lambda = (ry+ly)/(rx-lx).
                let numerator = (ry + ly) % PRIME31.p;
                let slope = mul(numerator, reciprocal, stats);
                let sx = mul(slope, slope, stats);
                let x = sub(sub(sx, rx), lx);
                let y = sub(mul(slope, sub(rx, x), stats), ry);
                answers[i] = Some((x, y));
            }
        }
        answers
    }

    pub(super) fn recover_batched(
        ar: &mut Arithmetic,
        half: &FourSet,
        pts: &[Point],
        residual: Point,
        stats: &mut BatchStats,
    ) -> (Option<Word8>, usize) {
        for (chunk_i, chunk) in half.ordered.chunks(WINDOW).enumerate() {
            let answers = complement_window(ar, residual, chunk, stats);
            for (offset, needed) in answers.into_iter().enumerate() {
                if let Some(&right) = half.index.get(&needed) {
                    let probe = chunk_i * WINDOW + offset;
                    let left = half.index.get(&half.ordered[probe]).expect("stable left index");
                    let witness = [
                        left[0], left[1], left[2], left[3],
                        right[0], right[1], right[2], right[3],
                    ];
                    assert_eq!(evaluate(ar, pts, &witness), residual, "original-curve replay");
                    stats.replayed += 1;
                    return (Some(witness), probe + 1);
                }
            }
        }
        (None, half.ordered.len())
    }

    #[test]
    fn actual_dickson31_matched_batched_complements() {
        let entire_setup_start = Instant::now();
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).expect("original cofactor-one prime31 curve");
        let base = Base::build(PRIME31, 6, &mut ar).expect("unchanged Dickson31");
        assert_eq!(base.points.len(), B31);
        let pts = tokens(&ar, &base.points);
        let half = FourSet::build(&mut ar, &pts);
        assert_eq!(half.index.len(), 618_885);
        let entire_setup_ns = entire_setup_start.elapsed().as_nanos();

        // Explicit same-x and identity tests at the arithmetic operation level,
        // not a claim that a naturally successful query visited those branches.
        let same_x_left = half.ordered.iter().copied().find(|x| x.is_some())
            .expect("4A contains nonidentity");
        let mut corner_stats = BatchStats::default();
        let same = complement_window(&mut ar, same_x_left, &[same_x_left], &mut corner_stats);
        let minus_same = ar.neg(PRIME31, same_x_left);
        assert_eq!(same[0], ar.add(PRIME31, same_x_left, minus_same));
        assert_eq!(corner_stats.special, 1);
        assert_eq!(corner_stats.inversions, 0);
        let none = complement_window(&mut ar, None, &[same_x_left, None], &mut corner_stats);
        assert_eq!(none[0], minus_same);
        assert_eq!(none[1], None);
        let target_identity_half = complement_window(&mut ar, same_x_left, &[None], &mut corner_stats);
        assert_eq!(target_identity_half[0], same_x_left);

        let mut state = HOLDOUT_SEED;
        let mut seen = HashSet::<u64>::new();
        let mut public_targets = Vec::new();
        for _ in 0..96 {
            let k = loop {
                let k = 1 + splitmix64(&mut state) % (PRIME31.n - 1);
                if seen.insert(k) { break k; }
            };
            public_targets.push(ar.scalar(PRIME31, k, Some(PRIME31.g)));
        }
        for k in [1_610_011_808_u64, 1_264_623_843_u64] {
            public_targets.push(ar.scalar(PRIME31, k, Some(PRIME31.g)));
        }

        let mut old_total_ns = 0_u128;
        let mut new_total_ns = 0_u128;
        let mut shared_residual_generation_ns = 0_u128;
        let mut old_fp_inversions = 0_u64;
        let mut old_fp_multiplications = 0_u64;
        let mut old_group_additions = 0_u64;
        let mut optimized_native_fp_inversions = 0_u64;
        let mut optimized_native_fp_multiplications = 0_u64;
        let mut new_stats = BatchStats::default();
        let mut first_shift_histogram = [0_usize; 10];
        let mut total_attempts = 0_usize;
        let mut total_positive = 0_usize;
        let mut total_negative = 0_usize;
        let mut old_probes = 0_u64;
        let mut old_full_eight_replay = 0_usize;

        for (case, q) in public_targets.iter().copied().enumerate() {
            let mut found = false;
            for shift in 0..=8_u64 {
                let common_start = Instant::now();
                let delta = ar.scalar(PRIME31, shift, Some(PRIME31.g));
                let residual = ar.add(PRIME31, q, delta);
                shared_residual_generation_ns += common_start.elapsed().as_nanos();
                total_attempts += 1;

                let old_before = ar.counts;
                let start = Instant::now();
                let (old_witness, old_count) = half.recover(&mut ar, &pts, residual);
                old_total_ns += start.elapsed().as_nanos();
                old_fp_inversions += ar.counts.fp_inversions - old_before.fp_inversions;
                old_fp_multiplications += ar.counts.fp_multiplications - old_before.fp_multiplications;
                old_group_additions += ar.counts.group_additions - old_before.group_additions;
                old_probes += old_count as u64;

                let new_before = ar.counts;
                let start = Instant::now();
                let (new_witness, new_count) =
                    recover_batched(&mut ar, &half, &pts, residual, &mut new_stats);
                new_total_ns += start.elapsed().as_nanos();
                optimized_native_fp_inversions +=
                    ar.counts.fp_inversions - new_before.fp_inversions;
                optimized_native_fp_multiplications +=
                    ar.counts.fp_multiplications - new_before.fp_multiplications;

                assert_eq!(new_count, old_count, "identical lookup order case={case}, shift={shift}");
                assert_eq!(new_witness, old_witness, "exact same first witness case={case}, shift={shift}");
                if old_witness.is_some() {
                    total_positive += 1;
                    old_full_eight_replay += 1;
                    first_shift_histogram[shift as usize] += 1;
                    found = true;
                    break;
                }
                total_negative += 1;
                assert_eq!(old_count, half.ordered.len(), "exhaustive negative");
            }
            assert!(found, "heldout target absent within frozen shift cap, case={case}");
        }
        assert_eq!(&first_shift_histogram[0..3], &[88, 9, 1]);
        assert_eq!(first_shift_histogram[3..9], [0; 6]);
        assert_eq!(total_attempts, 109);
        assert_eq!(total_positive, 98);
        assert_eq!(total_negative, 11);
        assert_eq!(new_stats.replayed as usize, old_full_eight_replay);
        assert_eq!(old_probes, 18_049_427);

        println!("DICKSON31_BATCH_COMPLEMENTS_JSON={}", json!({
            "schema":"prime31.dickson31.batch_complement.v1",
            "field_p":PRIME31.p, "group_n":PRIME31.n,
            "original_dickson31_base_size":B31,
            "signed_four_group_points":half.index.len(),
            "four_set_setup_ns_diagnostic":half.build_ns.to_string(),
            "curve_validation_and_full_factor_base_and_four_set_setup_ns":entire_setup_ns.to_string(),
            "shared_residual_generation_ns_excluded_from_both_arms":shared_residual_generation_ns.to_string(),
            "legacy_setup_plus_query_plus_shared_residual_ns":(entire_setup_ns + old_total_ns + shared_residual_generation_ns).to_string(),
            "batch_setup_plus_query_plus_shared_residual_ns":(entire_setup_ns + new_total_ns + shared_residual_generation_ns).to_string(),
            "four_set_setup_group_additions":half.build_additions,
            "hashmap_capacity_entries_not_bytes":half.index.capacity(),
            "public_target_count":public_targets.len(),
            "public_shift_max":8,
            "first_successful_shift_histogram":first_shift_histogram,
            "total_attempts":total_attempts,
            "positive_witnesses":total_positive,
            "exhaustive_negative_attempts":total_negative,
            "legacy_probes_before_return":old_probes,
            "legacy_attempt_wall_ns_diagnostic":old_total_ns.to_string(),
            "batch_attempt_wall_ns_diagnostic":new_total_ns.to_string(),
            "legacy_fp_inversions":old_fp_inversions,
            "legacy_fp_multiplications":old_fp_multiplications,
            "legacy_group_additions":old_group_additions,
            "batch_regular_lanes":new_stats.regular,
            "batch_all_processed_lanes_including_final_window":new_stats.processed,
            "batch_special_equal_x_lanes":new_stats.special,
            "batch_identity_lanes":new_stats.identity,
            "batch_windows":new_stats.windows,
            "batch_field_inversions_in_custom_arithmetic":new_stats.inversions,
            "batch_field_multiplications_in_custom_arithmetic":new_stats.multiplications,
            "batch_fallback_and_replay_native_fp_inversions":optimized_native_fp_inversions,
            "batch_fallback_and_replay_native_fp_multiplications":optimized_native_fp_multiplications,
            "montgomery_window":WINDOW,
            "first_witness_equal_in_all_positive_cases":true,
            "generic_equivalent_mitm_control":true,
            "sign_independent_algebraic_extractor":false,
            "complete_unknown_ecdlp_logs":0,
            "pcc":"unverified"
        }));
    }
}


// End-to-end *index-calculus-style* correctness/cost control, not a new
// algebraic/non-generic relation extractor. Reuses the signed 4A index and
// Montgomery-batched complement implementation above, not a new group law.
// Protocol: DICKSON31_COMPLETE_RANK_1024_PROTOCOL_20261117.md (PR #4623).
#[cfg(test)]
mod complete_b31_1024_tests {
    use super::*;
    use super::batch_complement_tests::{recover_batched, BatchStats};
    use super::b31_negation_quotient_tests::{prepare, recover_paired, Rep, PairStats};

    const RELATION_SEED: u64 = 0x4241_5345_3331_5f31;
    const CORPUS_SEED: u64 = 0x4d49_5834_5033_3131;
    const MAX_SOURCES: usize = 128;
    const TARGETS: usize = 1024;
    const MAX_PUBLIC_SHIFT: u64 = 8;
    const DIM: usize = B31;
    type Row = [u64; DIM + 1];

    #[inline]
    fn nmul(x: u64, y: u64) -> u64 {
        ((x as u128 * y as u128) % PRIME31.n as u128) as u64
    }
    #[inline]
    fn nadd(x: u64, y: u64) -> u64 {
        let t = x + y;
        if t >= PRIME31.n { t - PRIME31.n } else { t }
    }
    #[inline]
    fn nsub(x: u64, y: u64) -> u64 {
        if x >= y { x - y } else { x + PRIME31.n - y }
    }
    fn npow(mut x: u64, mut e: u64) -> u64 {
        let mut z = 1u64;
        while e > 0 {
            if e & 1 != 0 { z = nmul(z, x); }
            x = nmul(x, x);
            e >>= 1;
        }
        z
    }

    // No secret scalar, root oracle or support list reaches relation search.
    // Signed token 2i/2i+1 maps to +/-P_i; all eight terms are charged.
    fn signed_row(word: Word8, rhs: u64) -> Row {
        let mut row = [0u64; DIM + 1];
        for token in word {
            let i = (token / 2) as usize;
            assert!(i < DIM);
            row[i] = if token & 1 == 0 {
                nadd(row[i], 1)
            } else {
                nsub(row[i], 1)
            };
        }
        row[DIM] = rhs % PRIME31.n;
        row
    }

    #[derive(Default)]
    struct ModRank {
        pivots: [Option<Row>; DIM],
        count: usize,
        dependent: usize,
    }
    impl ModRank {
        fn insert(&mut self, mut row: Row) {
            for i in 0..DIM {
                if row[i] == 0 { continue; }
                if let Some(pivot) = self.pivots[i] {
                    let scale = row[i];
                    for j in i..=DIM { row[j] = nsub(row[j], nmul(scale, pivot[j])); }
                } else {
                    let recip = npow(row[i], PRIME31.n - 2);
                    for v in &mut row[i..] { *v = nmul(*v, recip); }
                    assert_eq!(row[i], 1);
                    self.pivots[i] = Some(row);
                    self.count += 1;
                    return;
                }
            }
            assert_eq!(row[DIM], 0, "inconsistent authentic curve relation");
            self.dependent += 1;
        }
        fn solve(&self) -> Option<[u64; DIM]> {
            if self.count != DIM { return None; }
            let mut logs = [0u64; DIM];
            for i in (0..DIM).rev() {
                let row = self.pivots[i]?;
                let mut rhs = row[DIM];
                for j in i + 1..DIM { rhs = nsub(rhs, nmul(row[j], logs[j])); }
                logs[i] = rhs;
            }
            Some(logs)
        }
    }

    fn evaluate_row(row: &Row, logs: &[u64; DIM]) -> u64 {
        let mut sum = 0;
        for i in 0..DIM { sum = nadd(sum, nmul(row[i], logs[i])); }
        sum
    }

    // Guaranteed full-group fallback for any unresolved target. It is generic,
    // charged and separately labelled, never counted as index calculus.
    fn fallback_bsgs(ar: &mut Arithmetic, q: Point) -> Option<u64> {
        const M: u64 = 46_341; // ceil sqrt(2147446831)
        let mut table = HashMap::<Point, u64>::with_capacity(M as usize);
        let mut baby = None;
        for j in 0..M {
            table.entry(baby).or_insert(j);
            baby = ar.add(PRIME31, baby, Some(PRIME31.g));
        }
        let mg = ar.scalar(PRIME31, M, Some(PRIME31.g));
        let minus_mg = ar.neg(PRIME31, mg);
        let mut giant = q;
        for i in 0..=(PRIME31.n / M + 1) {
            if let Some(&j) = table.get(&giant) {
                let k = (i * M + j) % PRIME31.n;
                if ar.scalar(PRIME31, k, Some(PRIME31.g)) == q { return Some(k); }
            }
            giant = ar.add(PRIME31, giant, minus_mg);
        }
        None
    }

    #[test]
    fn cold_bsgs_fallback_complete_corner_cases() {
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).expect("actual prime-order source curve");
        for k in [0u64, 1, PRIME31.n - 1, PRIME31.n / 2, 123_456_789] {
            let q = ar.scalar(PRIME31, k, Some(PRIME31.g));
            let recovered = fallback_bsgs(&mut ar, q).expect("generic subgroup fallback failed");
            assert_eq!(recovered, k, "failed complete fallback for k={k}");
            assert_eq!(ar.scalar(PRIME31, recovered, Some(PRIME31.g)), q);
        }
    }

    fn public_relation(
        ar: &mut Arithmetic, half: &FourSet, pts: &[Point],
        q: Point, batch: &mut BatchStats,
        attempts: &mut usize, negatives: &mut usize
    ) -> Option<(Word8, u64)> {
        for shift in 0..=MAX_PUBLIC_SHIFT {
            let moved = ar.scalar(PRIME31, shift, Some(PRIME31.g));
            let residual = ar.add(PRIME31, q, moved);
            *attempts += 1;
            let (maybe, probes) = recover_batched(ar, half, pts, residual, batch);
            if let Some(word) = maybe { return Some((word, shift)); }
            *negatives += 1;
            assert_eq!(probes, half.ordered.len(), "negative exhaustive lookup");
        }
        None
    }


    // Selects the previously verified 512-way complement or the new exact
    // negation-paired variant; all public shifts and failures are charged.
    fn public_relation_choice(
        ar: &mut Arithmetic, half: &FourSet, pts: &[Point], q: Point,
        batch: &mut BatchStats, reps: Option<&[Rep]>, pair_batch: &mut PairStats,
        attempts: &mut usize, negatives: &mut usize
    ) -> Option<(Word8,u64)> {
        if let Some(reps) = reps {
            for shift in 0..=MAX_PUBLIC_SHIFT {
                let moved = ar.scalar(PRIME31, shift, Some(PRIME31.g));
                let residual = ar.add(PRIME31, q, moved);
                *attempts += 1;
                let (witness, probes) =
                    recover_paired(ar, half, reps, pts, residual, pair_batch);
                if let Some(word) = witness { return Some((word, shift)); }
                *negatives += 1;
                assert_eq!(probes, reps.len(), "paired negative must be exhaustive");
            }
            None
        } else {
            public_relation(ar, half, pts, q, batch, attempts, negatives)
        }
    }

    #[test]
    fn complete_prime31_dickson31_rank_1024q() {
        let complete_start = Instant::now();
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).expect("valid original cofactor-one group");
        let base = Base::build(PRIME31, 6, &mut ar).expect("original Dickson depth-six");
        assert_eq!(base.points.len(), 31);
        let pts = tokens(&ar, &base.points);
        let half = FourSet::build(&mut ar, &pts);
        assert_eq!(half.index.len(), 618_885);
        let common_build_ns = complete_start.elapsed().as_nanos();

        let mut stats = BatchStats::default();
        let mut attempts = 0usize;
        let mut negatives = 0usize;
        let mut relation_seed = RELATION_SEED;
        let mut rank = ModRank::default();
        let mut collected = Vec::<Row>::new();
        let basis_start = Instant::now();
        let mut used_sources = 0usize;
        let mut setup_unresolved = 0usize;
        for _ in 0..MAX_SOURCES {
            if rank.count == DIM { break; }
            used_sources += 1;
            // The source scalar a is public and used only for the RHS.
            let a = 1 + splitmix64(&mut relation_seed) % (PRIME31.n - 1);
            let q = ar.scalar(PRIME31, a, Some(PRIME31.g));
            if let Some((w, shift)) = public_relation(
                &mut ar, &half, &pts, q, &mut stats, &mut attempts, &mut negatives
            ) {
                let row = signed_row(w, nadd(a, shift));
                rank.insert(row);
                collected.push(row);
            } else {
                setup_unresolved += 1;
            }
        }
        assert_eq!(rank.count, DIM, "128 public source controls did not span all 31 columns");
        let logs = rank.solve().expect("non-invertible rank-31 basis");
        for (i, &ell) in logs.iter().enumerate() {
            assert_eq!(
                ar.scalar(PRIME31, ell, Some(PRIME31.g)),
                Some(base.points[i]),
                "learned factor-base log failed original curve replay"
            );
        }
        for row in &collected {
            assert_eq!(evaluate_row(row, &logs), row[DIM], "setup equation replay");
        }
        let rank_learning_ns = basis_start.elapsed().as_nanos();

        let controls_start = Instant::now();
        let mut extra_verified = 0usize;
        for _ in 0..16 {
            let a = 1 + splitmix64(&mut relation_seed) % (PRIME31.n - 1);
            let q = ar.scalar(PRIME31, a, Some(PRIME31.g));
            if let Some((word, shift)) = public_relation(
                &mut ar, &half, &pts, q, &mut stats, &mut attempts, &mut negatives
            ) {
                let row = signed_row(word, nadd(a, shift));
                assert_eq!(evaluate_row(&row, &logs), row[DIM]);
                extra_verified += 1;
            } else {
                panic!("independent known-source control unrepresented within cap");
            }
        }
        let verification_controls_ns = controls_start.elapsed().as_nanos();

        // Caller-only corpus: no k supplied to public_relation or rank learning.
        let corpus_start = Instant::now();
        let mut corpus_seed = CORPUS_SEED;
        let mut seen = HashSet::new();
        let mut targets = Vec::with_capacity(TARGETS);
        for _ in 0..TARGETS {
            let k = loop {
                let v = 1 + splitmix64(&mut corpus_seed) % (PRIME31.n - 1);
                if seen.insert(v) { break v; }
            };
            targets.push((ar.scalar(PRIME31, k, Some(PRIME31.g)), k));
        }
        let harness_generation_ns_excluded = corpus_start.elapsed().as_nanos();
        assert_eq!(seen.len(), TARGETS);

        let target_start = Instant::now();
        let mut b31_solves = 0usize;
        let mut bsgs_fallbacks = 0usize;
        let mut first_shift = [0usize; 10];
        let mut target_attempts = 0usize;
        let mut target_negatives = 0usize;
        let mut exact_point_checks = 0usize;
        let mut returned = Vec::with_capacity(TARGETS);
        let mut target_transcript = Vec::with_capacity(TARGETS);
        for &(q, caller_k) in &targets {
            let before_attempts = target_attempts;
            let before_negatives = target_negatives;
            let mut signed_witness: Option<Word8> = None;
            let mut successful_shift: Option<u64> = None;
            let recovered = if let Some((word, shift)) = public_relation(
                &mut ar, &half, &pts, q, &mut stats, &mut target_attempts, &mut target_negatives
            ) {
                signed_witness = Some(word);
                successful_shift = Some(shift);
                let mut rlog = 0u64;
                for token in word {
                    let i = (token / 2) as usize;
                    rlog = if token & 1 == 0 {
                        nadd(rlog, logs[i])
                    } else {
                        nsub(rlog, logs[i])
                    };
                }
                b31_solves += 1;
                first_shift[shift as usize] += 1;
                nsub(rlog, shift)
            } else {
                bsgs_fallbacks += 1;
                fallback_bsgs(&mut ar, q).expect("complete subgroup fallback failed")
            };
            assert_eq!(ar.scalar(PRIME31, recovered, Some(PRIME31.g)), q);
            exact_point_checks += 1;
            // Caller knows this, candidate (including fallback) did not receive it.
            assert_eq!(recovered, caller_k);
            target_transcript.push(json!({
                "public_target_q":q,
                "recovered_log":recovered,
                "shift":successful_shift,
                "signed_eight_token_witness":signed_witness,
                "membership_attempts":target_attempts-before_attempts,
                "exhaustive_negatives":target_negatives-before_negatives,
                "used_generic_bsgs_fallback":signed_witness.is_none()
            }));
            returned.push(recovered);
        }
        let target_solve_verify_ns = target_start.elapsed().as_nanos();
        assert_eq!(returned.len(), TARGETS);
        assert_eq!(b31_solves + bsgs_fallbacks, TARGETS);

        println!("DICKSON31_COMPLETE_1024_JSON={}", json!({
            "schema":"prime31.dickson31.complete_rank1024.v1",
            "validation":"producer CI [pcc:unverified]",
            "field_p": PRIME31.p, "order_n": PRIME31.n,
            "curve_a": PRIME31.a, "curve_b": PRIME31.b,
            "base_size":DIM, "half_set":half.index.len(),
            "base_points":base.points,
            "basis_rows_raw":collected,
            "recovered_factor_base_logs":logs,
            "target_replay_transcript":target_transcript,
            "basis_rows":collected.len(), "basis_rank":rank.count,
            "basis_dependent_rows":rank.dependent,
            "basis_source_points":used_sources, "basis_unresolved_shift_cap":setup_unresolved,
            "additional_independent_source_equations_verified":extra_verified,
            "known_source_relation_attempts_including_16_controls":attempts,
            "known_source_exhaustive_negative_attempts":negatives,
            "target_corpus_splitmix_seed":format!("{CORPUS_SEED:#018x}"),
            "distinct_public_targets":seen.len(),
            "complete_unknown_logs":returned.len(), "b31_index_calculus_style_logs":b31_solves,
            "generic_bsgs_fallback_logs":bsgs_fallbacks,
            "target_relation_attempts":target_attempts, "target_exhaustive_negatives":target_negatives,
            "target_first_hit_shift_histogram":first_shift,
            "original_curve_scalar_replays":exact_point_checks+DIM,
            "common_original_curve_base_and_half_table_setup_ns":common_build_ns.to_string(),
            "public_basis_learning_and_replay_ns":rank_learning_ns.to_string(),
            "extra_known_source_validation_ns":verification_controls_ns.to_string(),
            "heldout_caller_corpus_generation_ns_excluded_from_solver":harness_generation_ns_excluded.to_string(),
            "unknown_target_solve_and_verify_ns":target_solve_verify_ns.to_string(),
            "charged_total_setup_basis_controls_queries_and_verification_ns":(
                common_build_ns + rank_learning_ns + verification_controls_ns + target_solve_verify_ns
            ).to_string(),
            "fully_charged_batch_complement_diagnostic":format!("{stats:?}"),
            "four_set_index_capacity_entries":half.index.capacity(),
            "four_set_ordered_capacity_entries":half.ordered.capacity(),
            "uses_generic_signed_four_plus_four_lookup":true,
            "genuinely_sign_independent_extractor":false,
            "beats_shared_precompute_bsgs_claim":false,
            "independent_signed_pcc":false
        }));
    }

    // Frozen protocol B27_B29_B31_FULL_COST_PROTOCOL_20261117.md.
    // Sensitivity of a GENERIC 4A+4A complete pipeline, not an algebraic
    // sign-independent prime-field index-calculus breakthrough.
    #[test]
    fn actual_prime31_b27_b29_b31_256q_complete_cost() {
        const QCOUNT: usize = 256;
        let mut panels = Vec::new();

        for &active in &[27usize, 29, 31] {
            let setup_start = Instant::now();
            let mut ar = Arithmetic::default();
            check_curve(PRIME31, &mut ar).expect("actual ordinary prime-order curve");
            let entire_dickson = Base::build(PRIME31, 6, &mut ar)
                .expect("unchanged depth-six Dickson base");
            assert_eq!(entire_dickson.points.len(), 31);
            let base = entire_dickson.points[..active].to_vec();
            let mut unique_x = HashSet::new();
            for &(x,y) in &base {
                assert!(unique_x.insert(x));
                assert_eq!(ar.rhs(PRIME31,x), ((y as u128*y as u128)%PRIME31.p as u128) as u64);
            }
            let pts = tokens(&ar, &base);
            let half = FourSet::build(&mut ar, &pts);
            let setup_ns = setup_start.elapsed().as_nanos();
            let n4 = signature_count(active, 4);
            let n8 = signature_count(active, 8);
            assert_eq!(half.integer_signatures as u128, n4);
            assert!(half.index.len() <= n4 as usize);
            if active == 31 {
                assert_eq!(half.index.len(), 618_885);
                assert_eq!(n8, 5_654_193_417);
            }

            // Generate exactly the same 256 public Q as the 1024-Q control
            // outside candidate phase timings. Harness-only k never reaches
            // public_relation, rank learning or the factor-base oracle.
            let corpus_start = Instant::now();
            let mut seed = CORPUS_SEED;
            let mut seen = HashSet::<u64>::new();
            let mut targets = Vec::with_capacity(QCOUNT);
            while targets.len() < QCOUNT {
                let k = 1 + splitmix64(&mut seed) % (PRIME31.n - 1);
                if !seen.insert(k) { continue; }
                targets.push((ar.scalar(PRIME31,k,Some(PRIME31.g)), k));
            }
            assert_eq!(targets[0].0, Some((872_133_043,510_239_525)));
            let corpus_generation_ns_excluded = corpus_start.elapsed().as_nanos();

            let mut batch = BatchStats::default();
            let mut rank = ModRank::default();
            let mut rows = Vec::<Row>::new();
            let mut relation_seed = RELATION_SEED;
            let mut source_attempts = 0usize;
            let mut source_negatives = 0usize;
            let mut source_count = 0usize;
            let mut source_unresolved = 0usize;
            let before_rank = ar.counts;
            let rank_start = Instant::now();
            for _ in 0..MAX_SOURCES {
                if rank.count == active { break; }
                source_count += 1;
                let a = 1 + splitmix64(&mut relation_seed) % (PRIME31.n - 1);
                let q = ar.scalar(PRIME31,a,Some(PRIME31.g));
                if let Some((w,shift)) = public_relation(
                    &mut ar,&half,&pts,q,&mut batch,&mut source_attempts,&mut source_negatives
                ) {
                    let row = signed_row(w,nadd(a,shift));
                    for &coeff in &row[active..DIM] {
                        assert_eq!(coeff,0,"inactive factor-base column must be zero");
                    }
                    rank.insert(row);
                    rows.push(row);
                } else { source_unresolved += 1; }
            }
            assert_eq!(rank.count,active,"128-public-source rank cap insufficient for B={active}");

            // The existing 31-column solver is restricted to its first
            // active columns without weakening the modular consistency gate.
            let mut logs = [0u64; DIM];
            for i in (0..active).rev() {
                let row = rank.pivots[i].expect("rank-active pivot absent");
                let mut rhs = row[DIM];
                for j in (i+1)..active {
                    rhs = nsub(rhs, nmul(row[j],logs[j]));
                }
                logs[i] = rhs;
            }
            for i in active..DIM { assert!(rank.pivots[i].is_none()); }
            for (i,&v) in logs.iter().enumerate().take(active) {
                assert_eq!(
                    ar.scalar(PRIME31,v,Some(PRIME31.g)),
                    Some(base[i]),
                    "recovered prefix factor-base log does not replay"
                );
            }
            for row in &rows {
                assert_eq!(evaluate_row(row,&logs),row[DIM]);
            }
            let rank_ns = rank_start.elapsed().as_nanos();
            let rank_fp_mults = ar.counts.fp_multiplications-before_rank.fp_multiplications;

            // Same 16 separate known-public-source equations as #4623.
            let controls_start = Instant::now();
            let mut controls_verified = 0usize;
            let mut controls_unresolved = 0usize;
            for _ in 0..16 {
                let a = 1 + splitmix64(&mut relation_seed) % (PRIME31.n - 1);
                let q = ar.scalar(PRIME31,a,Some(PRIME31.g));
                if let Some((w,shift)) = public_relation(
                    &mut ar,&half,&pts,q,&mut batch,&mut source_attempts,&mut source_negatives
                ) {
                    let row = signed_row(w,nadd(a,shift));
                    assert_eq!(evaluate_row(&row,&logs),row[DIM]);
                    controls_verified += 1;
                } else { controls_unresolved += 1; }
            }
            assert_eq!(controls_verified+controls_unresolved,16);
            let controls_ns = controls_start.elapsed().as_nanos();

            let before_targets = ar.counts;
            let target_start = Instant::now();
            let mut target_attempts = 0usize;
            let mut target_negatives = 0usize;
            let mut first_shift = [0usize;10];
            let mut generic_bsgs_fallbacks = 0usize;
            let mut transcript = Vec::with_capacity(QCOUNT);
            for (case,&(q,caller_only_k)) in targets.iter().enumerate() {
                let old_attempts = target_attempts;
                let old_negatives = target_negatives;
                let (recovered,witness,shift) =
                    if let Some((w,shift))=public_relation(
                        &mut ar,&half,&pts,q,&mut batch,&mut target_attempts,&mut target_negatives
                    ) {
                        let mut rlog = 0u64;
                        for token in w {
                            let i=(token/2) as usize;
                            assert!(i<active);
                            rlog=if token&1==0 {
                                nadd(rlog,logs[i])
                            } else { nsub(rlog,logs[i]) };
                        }
                        first_shift[shift as usize]+=1;
                        (nsub(rlog,shift),Some(w),Some(shift))
                    } else {
                        generic_bsgs_fallbacks+=1;
                        (
                            fallback_bsgs(&mut ar,q)
                                .expect("charged total group BSGS fallback failed"),
                            None,None
                        )
                    };
                assert_eq!(ar.scalar(PRIME31,recovered,Some(PRIME31.g)),q);
                assert_eq!(recovered,caller_only_k,"hidden known answer only in harness");
                transcript.push(json!({
                    "index":case,
                    "public_Q":q,
                    "recovered_k":recovered,
                    "first_successful_shift":shift,
                    "signed_eight_witness":witness,
                    "exact_attempts":target_attempts-old_attempts,
                    "exhaustive_negative_attempts":target_negatives-old_negatives,
                    "charged_generic_BSGS_fallback":witness.is_none()
                }));
            }
            let target_ns = target_start.elapsed().as_nanos();
            assert_eq!(transcript.len(),QCOUNT);
            if active == 31 {
                assert_eq!(first_shift[0],237,
                    "same first-256 frozen corpus as previous B31 coverage");
            }
            let target_fp_mults =
                ar.counts.fp_multiplications-before_targets.fp_multiplications;
            let target_native_adds =
                ar.counts.group_additions-before_targets.group_additions;
            let charged_ns=setup_ns+rank_ns+controls_ns+target_ns;

            panels.push(json!({
                "base_size":active,
                "factor_base_prefix_points":base,
                "formal_signed_four_nets":n4.to_string(),
                "formal_signed_eight_nets":n8.to_string(),
                "distinct_four_group_points":half.index.len(),
                "four_signature_collisions":n4 as usize-half.index.len(),
                "four_sum_hashmap_capacity":half.index.capacity(),
                "four_sum_ordered_capacity":half.ordered.capacity(),
                "rank":rank.count,
                "rank_target":active,
                "public_known_source_count_to_full_rank":source_count,
                "public_known_source_unresolved_after_9_shifts":source_unresolved,
                "factor_base_logs":logs[..active].to_vec(),
                "signed_modular_relation_rows":rows,
                "extra_known_source_equations_verified":controls_verified,
                "extra_known_source_equations_unresolved":controls_unresolved,
                "known_source_membership_attempts_including_controls":source_attempts,
                "known_source_exhaustive_negative_attempts":source_negatives,
                "unknown_log_results":transcript,
                "unknown_logs_recovered":QCOUNT,
                "target_membership_attempts":target_attempts,
                "target_exhaustive_negative_attempts":target_negatives,
                "first_successful_public_shift_histogram":first_shift,
                "generic_BSGS_fallback_count":generic_bsgs_fallbacks,
                "charged_common_curve_base_and_four_index_setup_ns":setup_ns.to_string(),
                "charged_public_basis_learning_and_original_curve_replay_ns":rank_ns.to_string(),
                "charged_16_public_source_controls_ns":controls_ns.to_string(),
                "charged_256_blinded_target_solve_and_original_curve_verification_ns":target_ns.to_string(),
                "charged_total_ns":charged_ns.to_string(),
                "target_generation_harness_ns_excluded":corpus_generation_ns_excluded.to_string(),
                "rank_learning_native_fp_mults":rank_fp_mults,
                "target_native_fp_mults":target_fp_mults,
                "target_native_group_additions":target_native_adds,
                "full_panel_native_fp_mults":ar.counts.fp_multiplications,
                "full_panel_native_fp_inversions":ar.counts.fp_inversions,
                "full_panel_native_group_additions":ar.counts.group_additions,
                "batched_complement_operation_counters":format!("{batch:?}")
            }));
        }

        println!("PRIME31_B27_B29_B31_COMPLETE_JSON={}", json!({
            "schema":"prime31.actual_bsize_256q_complete.v1",
            "validation":"source-specific producer Rust [pcc:unverified]",
            "field_p":PRIME31.p,"subgroup_n":PRIME31.n,
            "same_public_256_target_corpus_seed":format!("{CORPUS_SEED:#018x}"),
            "same_known_source_seed":format!("{RELATION_SEED:#018x}"),
            "same_public_shift_cap":MAX_PUBLIC_SHIFT,
            "same_public_target_count":QCOUNT,
            "original_rust_curve_and_batched_four_sum_oracle":true,
            "all_attempts_and_generic_fallback_charged":true,
            "true_sign_independent_algebraic_extraction":false,
            "external_matched_IC_frontier_measured_here":false,
            "independent_signed_pcc":false,
            "panels":panels
        }));
    }


    // Same-curve complete unknown-log baseline: first-rational-x versus Dickson.
    // Preregistered B31_DICKSON_FIRSTX_FULL_COST_PROTOCOL_20261117.md.
    // Both arms remain generic-equivalent signed 4A+4A membership methods.
    #[test]
    fn actual_prime31_b31_firstx_vs_dickson_256q_complete_cost() {
        const QCOUNT: usize = 256;
        let mut panels = Vec::new();

        let paired_compare = std::env::var("PRIME31_B31_NEGATION_COMPARE")
            .ok().as_deref() == Some("1");
        let reversed = std::env::var("PRIME31_B31_BASECHOICE_REVERSE")
            .ok().as_deref() == Some("1");
        let arms: [&str;2] = if paired_compare {
            if reversed { ["first_x31_paired", "first_x31"] }
            else { ["first_x31", "first_x31_paired"] }
        } else if reversed {
            ["first_x31", "dickson31"]
        } else {
            ["dickson31", "first_x31"]
        };
        for &base_kind in &arms {
            let active = 31usize;
            let setup_start = Instant::now();
            let mut ar = Arithmetic::default();
            check_curve(PRIME31, &mut ar).expect("actual ordinary prime-order curve");
            let base = if base_kind == "dickson31" {
                Base::build(PRIME31, 6, &mut ar)
                    .expect("unchanged depth-six Dickson base").points
            } else {
                first_rational_points(&mut ar, active)
            };
            assert_eq!(base.len(), active);
            let mut unique_x = HashSet::new();
            for &(x,y) in &base {
                assert!(unique_x.insert(x));
                assert_eq!(ar.rhs(PRIME31,x), ((y as u128*y as u128)%PRIME31.p as u128) as u64);
            }
            let pts = tokens(&ar, &base);
            let half = FourSet::build(&mut ar, &pts);
            let representatives = if base_kind == "first_x31_paired" {
                Some(prepare(&half))
            } else { None };
            let setup_ns = setup_start.elapsed().as_nanos();
            let n4 = signature_count(active, 4);
            let n8 = signature_count(active, 8);
            assert_eq!(half.integer_signatures as u128, n4);
            assert!(half.index.len() <= n4 as usize);
            if base_kind == "dickson31" {
                assert_eq!(half.index.len(), 618_885);
                assert_eq!(n8, 5_654_193_417);
            }

            if base_kind == "first_x31_paired" {
                assert_eq!(representatives.as_ref().unwrap().len(),309_443);
            }

            // Generate exactly the same 256 public Q as the 1024-Q control
            // outside candidate phase timings. Harness-only k never reaches
            // public_relation, rank learning or the factor-base oracle.
            let corpus_start = Instant::now();
            let mut seed = CORPUS_SEED;
            let mut seen = HashSet::<u64>::new();
            let mut targets = Vec::with_capacity(QCOUNT);
            while targets.len() < QCOUNT {
                let k = 1 + splitmix64(&mut seed) % (PRIME31.n - 1);
                if !seen.insert(k) { continue; }
                targets.push((ar.scalar(PRIME31,k,Some(PRIME31.g)), k));
            }
            assert_eq!(targets[0].0, Some((872_133_043,510_239_525)));
            let corpus_generation_ns_excluded = corpus_start.elapsed().as_nanos();

            let mut batch = BatchStats::default();
            let mut paired_batch = PairStats::default();
            let mut rank = ModRank::default();
            let mut rows = Vec::<Row>::new();
            let mut relation_seed = RELATION_SEED;
            let mut source_attempts = 0usize;
            let mut source_negatives = 0usize;
            let mut source_count = 0usize;
            let mut source_unresolved = 0usize;
            let before_rank = ar.counts;
            let rank_start = Instant::now();
            for _ in 0..MAX_SOURCES {
                if rank.count == active { break; }
                source_count += 1;
                let a = 1 + splitmix64(&mut relation_seed) % (PRIME31.n - 1);
                let q = ar.scalar(PRIME31,a,Some(PRIME31.g));
                if let Some((w,shift)) = public_relation_choice(
                    &mut ar,&half,&pts,q,&mut batch,representatives.as_deref(),&mut paired_batch,&mut source_attempts,&mut source_negatives
                ) {
                    let row = signed_row(w,nadd(a,shift));
                    for &coeff in &row[active..DIM] {
                        assert_eq!(coeff,0,"inactive factor-base column must be zero");
                    }
                    rank.insert(row);
                    rows.push(row);
                } else { source_unresolved += 1; }
            }
            assert_eq!(rank.count,active,"128-public-source rank cap insufficient for B={active}");

            // Recover all 31 original-coordinate base logarithms without
            // weakening the modular consistency gate.
            let mut logs = [0u64; DIM];
            for i in (0..active).rev() {
                let row = rank.pivots[i].expect("rank-active pivot absent");
                let mut rhs = row[DIM];
                for j in (i+1)..active {
                    rhs = nsub(rhs, nmul(row[j],logs[j]));
                }
                logs[i] = rhs;
            }
            for i in active..DIM { assert!(rank.pivots[i].is_none()); }
            for (i,&v) in logs.iter().enumerate().take(active) {
                assert_eq!(
                    ar.scalar(PRIME31,v,Some(PRIME31.g)),
                    Some(base[i]),
                    "recovered prefix factor-base log does not replay"
                );
            }
            for row in &rows {
                assert_eq!(evaluate_row(row,&logs),row[DIM]);
            }
            let rank_ns = rank_start.elapsed().as_nanos();
            let rank_fp_mults = ar.counts.fp_multiplications-before_rank.fp_multiplications;

            // Same 16 separate known-public-source equations as #4623.
            let controls_start = Instant::now();
            let mut controls_verified = 0usize;
            let mut controls_unresolved = 0usize;
            for _ in 0..16 {
                let a = 1 + splitmix64(&mut relation_seed) % (PRIME31.n - 1);
                let q = ar.scalar(PRIME31,a,Some(PRIME31.g));
                if let Some((w,shift)) = public_relation_choice(
                    &mut ar,&half,&pts,q,&mut batch,representatives.as_deref(),&mut paired_batch,&mut source_attempts,&mut source_negatives
                ) {
                    let row = signed_row(w,nadd(a,shift));
                    assert_eq!(evaluate_row(&row,&logs),row[DIM]);
                    controls_verified += 1;
                } else { controls_unresolved += 1; }
            }
            assert_eq!(controls_verified+controls_unresolved,16);
            let controls_ns = controls_start.elapsed().as_nanos();

            let before_targets = ar.counts;
            let target_start = Instant::now();
            let mut target_attempts = 0usize;
            let mut target_negatives = 0usize;
            let mut first_shift = [0usize;10];
            let mut generic_bsgs_fallbacks = 0usize;
            let mut transcript = Vec::with_capacity(QCOUNT);
            for (case,&(q,caller_only_k)) in targets.iter().enumerate() {
                let old_attempts = target_attempts;
                let old_negatives = target_negatives;
                let (recovered,witness,shift) =
                    if let Some((w,shift))=public_relation_choice(
                        &mut ar,&half,&pts,q,&mut batch,representatives.as_deref(),&mut paired_batch,&mut target_attempts,&mut target_negatives
                    ) {
                        let mut rlog = 0u64;
                        for token in w {
                            let i=(token/2) as usize;
                            assert!(i<active);
                            rlog=if token&1==0 {
                                nadd(rlog,logs[i])
                            } else { nsub(rlog,logs[i]) };
                        }
                        first_shift[shift as usize]+=1;
                        (nsub(rlog,shift),Some(w),Some(shift))
                    } else {
                        generic_bsgs_fallbacks+=1;
                        (
                            fallback_bsgs(&mut ar,q)
                                .expect("charged total group BSGS fallback failed"),
                            None,None
                        )
                    };
                assert_eq!(ar.scalar(PRIME31,recovered,Some(PRIME31.g)),q);
                assert_eq!(recovered,caller_only_k,"hidden known answer only in harness");
                transcript.push(json!({
                    "index":case,
                    "public_Q":q,
                    "recovered_k":recovered,
                    "first_successful_shift":shift,
                    "signed_eight_witness":witness,
                    "exact_attempts":target_attempts-old_attempts,
                    "exhaustive_negative_attempts":target_negatives-old_negatives,
                    "charged_generic_BSGS_fallback":witness.is_none()
                }));
            }
            let target_ns = target_start.elapsed().as_nanos();
            assert_eq!(transcript.len(),QCOUNT);
            assert_eq!(first_shift[0], if base_kind == "dickson31" {237} else {244},
                "same first-256 frozen corpus as matched B31 coverage control #4626");
            let target_fp_mults =
                ar.counts.fp_multiplications-before_targets.fp_multiplications;
            let target_native_adds =
                ar.counts.group_additions-before_targets.group_additions;
            let charged_ns=setup_ns+rank_ns+controls_ns+target_ns;

            panels.push(json!({
                "base_kind":base_kind,
                "base_size":active,
                "factor_base_prefix_points":base,
                "formal_signed_four_nets":n4.to_string(),
                "formal_signed_eight_nets":n8.to_string(),
                "distinct_four_group_points":half.index.len(),
                "four_signature_collisions":n4 as usize-half.index.len(),
                "four_sum_hashmap_capacity":half.index.capacity(),
                "four_sum_ordered_capacity":half.ordered.capacity(),
                "rank":rank.count,
                "rank_target":active,
                "public_known_source_count_to_full_rank":source_count,
                "public_known_source_unresolved_after_9_shifts":source_unresolved,
                "factor_base_logs":logs[..active].to_vec(),
                "signed_modular_relation_rows":rows,
                "extra_known_source_equations_verified":controls_verified,
                "extra_known_source_equations_unresolved":controls_unresolved,
                "known_source_membership_attempts_including_controls":source_attempts,
                "known_source_exhaustive_negative_attempts":source_negatives,
                "unknown_log_results":transcript,
                "unknown_logs_recovered":QCOUNT,
                "target_membership_attempts":target_attempts,
                "target_exhaustive_negative_attempts":target_negatives,
                "first_successful_public_shift_histogram":first_shift,
                "generic_BSGS_fallback_count":generic_bsgs_fallbacks,
                "charged_common_curve_base_and_four_index_setup_ns":setup_ns.to_string(),
                "charged_public_basis_learning_and_original_curve_replay_ns":rank_ns.to_string(),
                "charged_16_public_source_controls_ns":controls_ns.to_string(),
                "charged_256_blinded_target_solve_and_original_curve_verification_ns":target_ns.to_string(),
                "charged_total_ns":charged_ns.to_string(),
                "target_generation_harness_ns_excluded":corpus_generation_ns_excluded.to_string(),
                "rank_learning_native_fp_mults":rank_fp_mults,
                "target_native_fp_mults":target_fp_mults,
                "target_native_group_additions":target_native_adds,
                "full_panel_native_fp_mults":ar.counts.fp_multiplications,
                "full_panel_native_fp_inversions":ar.counts.fp_inversions,
                "full_panel_native_group_additions":ar.counts.group_additions,
                "batched_complement_operation_counters":format!("{batch:?}"),
                "negation_quotient_representatives":representatives.as_ref().map(|r|r.len()),
                "negation_paired_batch_counters":format!("{paired_batch:?}"),
                "paired_candidate":base_kind=="first_x31_paired"
            }));
        }

        let prefix = if paired_compare { "PRIME31_B31_NEGATION_COMPLETE_JSON" }
                     else { "PRIME31_B31_FIRSTX_DICKSON_COMPLETE_JSON" };
        println!("{prefix}={}", json!({
            "schema":if paired_compare { "prime31.b31.negation_complete_256q.v1" }
                     else { "prime31.actual_b31_basechoice_256q_complete.v1" },
            "panel_order": arms,
            "validation":"source-specific producer Rust [pcc:unverified]",
            "field_p":PRIME31.p,"subgroup_n":PRIME31.n,
            "same_public_256_target_corpus_seed":format!("{CORPUS_SEED:#018x}"),
            "same_known_source_seed":format!("{RELATION_SEED:#018x}"),
            "same_public_shift_cap":MAX_PUBLIC_SHIFT,
            "same_public_target_count":QCOUNT,
            "original_rust_curve_and_batched_four_sum_oracle":true,
            "all_attempts_and_generic_fallback_charged":true,
            "true_sign_independent_algebraic_extraction":false,
            "external_matched_IC_frontier_measured_here":false,
            "independent_signed_pcc":false,
            "panels":panels
        }));
    }
}


// Original-curve matched-coverage *null control* for signed eight-leaf sums.
// Public-Q candidate reuses the existing generic FourSet and 512-lane batch
// complement engine. It is NOT sign-independent S9/RR9 extraction or ECDLP.
// Preregistered: B31_MATCHED_COVERAGE_NULL_PROTOCOL_20261117.md (PR #4626).
#[cfg(test)]
mod b31_coverage_null_tests {
    use super::*;
    use super::batch_complement_tests::{recover_batched, BatchStats};

    const TARGETS: usize = 256;
    const CORPUS_SEED: u64 = 0x4d49_5834_5033_3131;
    const RANDOM_SEEDS: [u64; 2] = [
        0x5241_4e44_4233_3101,
        0x5241_4e44_4233_3102,
    ];

    // In the abstract null, z_i are independent uniform scalars modulo n.
    // The conditioned fixed-x bases below are empirical controls, not an
    // independently hashed family of all N signed-net signatures.
    const NET8: u128 = 5_654_193_417;
    const NET4: u128 = 618_885;

    fn random_distinct_x_base(ar: &mut Arithmetic, mut seed: u64) -> Vec<(u64, u64)> {
        let mut seen_x = HashSet::new();
        let mut points = Vec::with_capacity(B31);
        while points.len() < B31 {
            let k = 1 + splitmix64(&mut seed) % (PRIME31.n - 1);
            let (x, y) = ar.scalar(PRIME31, k, Some(PRIME31.g))
                .expect("nonzero generator scalar gives a finite point");
            if seen_x.insert(x) {
                points.push((x, y.min(PRIME31.p - y)));
            }
        }
        assert_eq!(seen_x.len(), B31);
        points
    }

    fn corpus(ar: &mut Arithmetic) -> Vec<Point> {
        let mut seed = CORPUS_SEED;
        let mut seen = HashSet::new();
        let mut targets = Vec::with_capacity(TARGETS);
        while targets.len() < TARGETS {
            let scalar = 1 + splitmix64(&mut seed) % (PRIME31.n - 1);
            if seen.insert(scalar) {
                targets.push(ar.scalar(PRIME31, scalar, Some(PRIME31.g)));
            }
        }
        assert_eq!(targets[0], Some((872_133_043, 510_239_525)),
            "must reuse original frozen 1024-Q Rust corpus");
        targets
    }

    #[test]
    fn actual_prime31_matched_b31_256q_unshifted_coverage() {
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).expect("original ordinary cofactor-one curve");
        assert_eq!(signature_count(B31, 4), NET4);
        assert_eq!(signature_count(B31, 8), NET8);

        let targets = corpus(&mut ar);
        let dickson = Base::build(PRIME31, 6, &mut ar)
            .expect("original 31-point Dickson depth-six factor base");
        assert_eq!(dickson.points.len(), B31);
        let arms: Vec<(&str, Vec<(u64, u64)>)> = vec![
            ("actual_dickson31", dickson.points),
            ("first31_rational_x", first_rational_points(&mut ar, B31)),
            ("seeded_random31_a", random_distinct_x_base(&mut ar, RANDOM_SEEDS[0])),
            ("seeded_random31_b", random_distinct_x_base(&mut ar, RANDOM_SEEDS[1])),
        ];

        let mut all_hits: Vec<Vec<bool>> = Vec::with_capacity(arms.len());
        let mut panels = Vec::with_capacity(arms.len());
        for (name, base) in arms {
            assert_eq!(base.len(), B31);
            let mut unique_x = HashSet::new();
            for &(x, y) in &base {
                assert!(unique_x.insert(x), "every point has a distinct x");
                assert_eq!(y, y.min(PRIME31.p - y), "canonical y sign");
                assert_eq!(ar.rhs(PRIME31, x), ((y as u128*y as u128)%PRIME31.p as u128) as u64);
            }
            let setup_start = Instant::now();
            let pts = tokens(&ar, &base);
            let half = FourSet::build(&mut ar, &pts);
            let setup_ns = setup_start.elapsed().as_nanos();
            assert!(half.index.len() <= NET4 as usize);
            let mut stats = BatchStats::default();
            let before = ar.counts;
            let query_start = Instant::now();
            let mut mask = Vec::with_capacity(TARGETS);
            let mut probes = 0u64;
            let mut positives = 0usize;
            let mut negatives = 0usize;
            for &q in &targets {
                let (witness, visited) = recover_batched(&mut ar, &half, &pts, q, &mut stats);
                probes += visited as u64;
                if witness.is_some() {
                    positives += 1;
                } else {
                    negatives += 1;
                    assert_eq!(visited, half.ordered.len(),
                        "negative query must exhaust every distinct 4A sum");
                }
                mask.push(witness.is_some());
            }
            let query_ns = query_start.elapsed().as_nanos();
            assert_eq!(mask.len(), TARGETS);
            assert_eq!(positives + negatives, TARGETS);
            if name == "actual_dickson31" {
                // Recheck against the frozen first-256 JSON from #4623.
                assert_eq!(positives, 237, "Dickson coverage changed vs exact 1024-Q Rust transcript");
                assert_eq!(half.index.len() as u128, NET4, "Dickson 4A cardinality changed");
            }
            panels.push(json!({
                "base_name":name,
                "public_base_points":base,
                "four_net_signatures":NET4.to_string(),
                "distinct_four_group_points":half.index.len(),
                "four_signature_collisions":NET4 as usize-half.index.len(),
                "four_set_hash_capacity_entries":half.index.capacity(),
                "four_set_ordered_capacity_entries":half.ordered.capacity(),
                "charged_four_set_build_and_tokens_ns":setup_ns.to_string(),
                "charged_all_256_public_queries_ns":query_ns.to_string(),
                "original_arithmetic_native_fp_mults":ar.counts.fp_multiplications-before.fp_multiplications,
                "original_arithmetic_native_fp_inversions":ar.counts.fp_inversions-before.fp_inversions,
                "native_group_additions":ar.counts.group_additions-before.group_additions,
                "custom_batch_operation_counters":format!("{stats:?}"),
                "first_shift_zero_hits":positives,
                "first_shift_zero_exhaustive_negatives":negatives,
                "total_exact_four_set_complement_probes":probes,
                "unshifted_public_target_hit_mask":mask,
                "original_curve_signed_witness_replay_for_each_hit":true
            }));
            all_hits.push(mask);
        }

        let mut paired = Vec::new();
        for j in 1..all_hits.len() {
            let mut both = 0usize;
            let mut dickson_only = 0usize;
            let mut comparator_only = 0usize;
            let mut neither = 0usize;
            for i in 0..TARGETS {
                match (all_hits[0][i], all_hits[j][i]) {
                    (true, true) => both += 1,
                    (true, false) => dickson_only += 1,
                    (false, true) => comparator_only += 1,
                    (false, false) => neither += 1,
                }
            }
            assert_eq!(both+dickson_only+comparator_only+neither, TARGETS);
            paired.push(json!({
                "comparator_index":j,
                "both_hit":both,
                "dickson_only":dickson_only,
                "comparator_only":comparator_only,
                "neither_hit":neither
            }));
        }

        let n = PRIME31.n as u128;
        // Pairwise-collision formula is rigorous only for independent uniform
        // abstract z_i. Bin/Poisson coverage below is heuristic because every
        // signed-net map reuses the same 31 scalars, inducing correlations.
        let expected_energy_numerator = NET8 * (n + NET8 - 1);
        let mean_occupancy = NET8 as f64 / PRIME31.n as f64;
        let heuristic_hit = 1.0 - (-mean_occupancy).exp();
        println!("B31_MATCHED_COVERAGE_JSON={}", json!({
            "schema":"prime31.b31.matched_256q_coverage_null.v1",
            "validation":"producer exact-source Rust [pcc:unverified]",
            "field_p":PRIME31.p,
            "order_n":PRIME31.n,
            "curve_a":PRIME31.a,
            "curve_b":PRIME31.b,
            "common_frozen_corpus_seed":format!("{CORPUS_SEED:#018x}"),
            "random_factor_base_seeds":[format!("{:#018x}",RANDOM_SEEDS[0]),format!("{:#018x}",RANDOM_SEEDS[1])],
            "same_public_target_count":TARGETS,
            "public_shift":0,
            "formal_signed_four_net_count":NET4.to_string(),
            "formal_signed_eight_net_count":NET8.to_string(),
            "uniform_scalar_null_expected_collision_energy_numerator":expected_energy_numerator.to_string(),
            "uniform_scalar_null_expected_collision_energy_denominator":PRIME31.n,
            "uniform_scalar_null_support_lower_bound_fraction":NET8 as f64/(n+NET8-1) as f64,
            "independent_bin_occupancy_heuristic_mean":mean_occupancy,
            "independent_bin_occupancy_heuristic_hit_fraction":heuristic_hit,
            "independent_bin_occupancy_is_not_a_theorem":true,
            "matched_panels":panels,
            "paired_outcomes_against_dickson":paired,
            "reused_original_generic_four_plus_four_oracle":true,
            "sign_independent_extraction":false,
            "new_unknown_logarithms_solved":0,
            "full_cost_ecdlp_benchmark":false,
            "signed_pcc":false
        }));
    }
}


// Paired R±P complements quotient the existing signed FourSet by negation.
// This is still a generic-equivalent group-sum method, NOT RR9 elimination.
// Frozen protocol: B31_NEGATION_QUOTIENT_PROTOCOL_20261118.md, PR #4647.
#[cfg(test)]
mod b31_negation_quotient_tests {
    use super::*;
    use super::batch_complement_tests::{recover_batched, BatchStats};

    const WINDOW: usize = 512;
    const QCOUNT: usize = 256;
    const CORPUS_SEED: u64 = 0x4d49_5834_5033_3131;

    #[derive(Clone, Copy)]
    pub(super) struct Rep {
        pub(super) point: Point,
        pub(super) plus_word: Word4,
        pub(super) minus_word: Word4,
    }

    #[derive(Default, Clone, Copy, Debug)]
    pub(super) struct PairStats {
        pub(super) windows: u64,
        pub(super) inversions: u64,
        pub(super) multiplications: u64,
        pub(super) canonical_probes: u64,
        pub(super) exceptional: u64,
        pub(super) witness_replays: u64,
    }

    #[inline]
    fn sub(a: u64, b: u64) -> u64 {
        if a >= b { a - b } else { a + PRIME31.p - b }
    }
    #[inline]
    fn add(a: u64, b: u64) -> u64 {
        let z = a + b;
        if z >= PRIME31.p { z - PRIME31.p } else { z }
    }
    #[inline]
    fn mul(a: u64, b: u64, stats: &mut PairStats) -> u64 {
        stats.multiplications += 1;
        ((a as u128 * b as u128) % PRIME31.p as u128) as u64
    }
    fn pow(mut x: u64, mut e: u64, stats: &mut PairStats) -> u64 {
        let mut out = 1u64;
        while e > 0 {
            if (e & 1) != 0 { out = mul(out, x, stats); }
            x = mul(x, x, stats);
            e >>= 1;
        }
        out
    }

    pub(super) fn prepare(half: &FourSet) -> Vec<Rep> {
        assert_eq!(half.index.len(), half.ordered.len());
        assert_eq!(half.index.len() % 2, 1, "odd-order negation quotient");
        let mut reps = Vec::with_capacity((half.index.len() + 1) / 2);
        for &p in &half.ordered {
            match p {
                None => {
                    let &word = half.index.get(&None).expect("identity in 4A");
                    reps.push(Rep { point: None, plus_word: word, minus_word: word });
                }
                Some((x, y)) => {
                    assert_ne!(y, 0, "original odd-order subgroup has no 2-torsion");
                    if y < PRIME31.p - y {
                        let neg = Some((x, PRIME31.p - y));
                        let &plus_word = half.index.get(&p).expect("positive half in 4A");
                        let &minus_word = half.index.get(&neg).expect("negated half in 4A");
                        reps.push(Rep { point: p, plus_word, minus_word });
                    }
                }
            }
        }
        assert_eq!(reps.len(), (half.index.len() + 1) / 2,
            "every nonzero signed four-sum must have its inverse");
        assert_eq!(reps.iter().filter(|r| r.point.is_none()).count(), 1,
            "identity fixed point must be included exactly once");
        reps
    }

    // For every canonical P compute R-P and R+P using ONE inversion of
    // rx-px. Equal-x and infinity paths defer to original Rust group addition.
    fn paired_window(
        ar: &mut Arithmetic, r: Point, reps: &[Rep],
        stats: &mut PairStats
    ) -> Vec<(Point, Point)> {
        stats.windows += 1;
        let mut out = vec![(None, None); reps.len()];
        let mut ids = Vec::with_capacity(reps.len());
        let mut denoms = Vec::with_capacity(reps.len());
        let mut prefixes = Vec::with_capacity(reps.len());
        let mut product = 1u64;
        for (i, rep) in reps.iter().enumerate() {
            match (r, rep.point) {
                (Some((rx, _)), Some((px, _))) if rx != px => {
                    let d = sub(rx, px);
                    ids.push(i);
                    denoms.push(d);
                    prefixes.push(product);
                    product = mul(product, d, stats);
                }
                _ => {
                    let minus_p = ar.neg(PRIME31, rep.point);
                    out[i] = (
                        ar.add(PRIME31, r, minus_p),
                        ar.add(PRIME31, r, rep.point),
                    );
                    stats.exceptional += 1;
                }
            }
        }
        if !ids.is_empty() {
            stats.inversions += 1;
            let mut inv_product = pow(product, PRIME31.p - 2, stats);
            for j in (0..ids.len()).rev() {
                let inv = mul(prefixes[j], inv_product, stats);
                inv_product = mul(inv_product, denoms[j], stats);
                let i = ids[j];
                let (rx, ry) = r.expect("finite regular residual");
                let (px, py) = reps[i].point.expect("finite regular representative");
                let slope_minus = mul(add(ry, py), inv, stats);
                let x_minus = sub(sub(mul(slope_minus, slope_minus, stats), rx), px);
                let y_minus = sub(mul(slope_minus, sub(rx, x_minus), stats), ry);
                let slope_plus = mul(sub(ry, py), inv, stats);
                let x_plus = sub(sub(mul(slope_plus, slope_plus, stats), rx), px);
                let y_plus = sub(mul(slope_plus, sub(rx, x_plus), stats), ry);
                out[i] = (Some((x_minus, y_minus)), Some((x_plus, y_plus)));
            }
        }
        out
    }

    pub(super) fn recover_paired(
        ar: &mut Arithmetic, half: &FourSet, reps: &[Rep],
        pts: &[Point], r: Point, stats: &mut PairStats
    ) -> (Option<Word8>, usize) {
        let mut seen = 0;
        for chunk in reps.chunks(WINDOW) {
            let answers = paired_window(ar, r, chunk, stats);
            for (rep, (needed_minus, needed_plus)) in chunk.iter().zip(answers.into_iter()) {
                seen += 1;
                stats.canonical_probes += 1;
                for (needed, left) in [(needed_minus, rep.plus_word), (needed_plus, rep.minus_word)] {
                    if let Some(&right) = half.index.get(&needed) {
                        let word = [left[0], left[1], left[2], left[3],
                                    right[0], right[1], right[2], right[3]];
                        assert_eq!(evaluate(ar, pts, &word), r,
                            "exact signed witness on original curve");
                        stats.witness_replays += 1;
                        return (Some(word), seen);
                    }
                }
            }
        }
        (None, seen)
    }

    #[test]
    fn actual_prime31_firstx_b31_paired_256q_component() {
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).expect("original prime-order curve");
        let base = first_rational_points(&mut ar, B31);
        let pts = tokens(&ar, &base);
        let half = FourSet::build(&mut ar, &pts);
        assert_eq!(half.index.len(), 618_885);
        let prep_start = Instant::now();
        let reps = prepare(&half);
        let additional_setup_ns = prep_start.elapsed().as_nanos();
        assert_eq!(reps.len(), 309_443);

        let mut state = CORPUS_SEED;
        let mut seen = HashSet::new();
        let mut targets = Vec::with_capacity(QCOUNT);
        while targets.len() < QCOUNT {
            let k = 1 + splitmix64(&mut state) % (PRIME31.n - 1);
            if seen.insert(k) {
                targets.push(ar.scalar(PRIME31, k, Some(PRIME31.g)));
            }
        }
        assert_eq!(targets[0], Some((872_133_043, 510_239_525)));

        let mut old = BatchStats::default();
        let mut paired = PairStats::default();
        let mut old_ns = 0u128;
        let mut paired_ns = 0u128;
        let mut hit_count = 0usize;
        let mut exhaustive_negatives = 0usize;
        for (i, &r) in targets.iter().enumerate() {
            // AB/BA order alternated on identical public residuals; no pre-tuning.
            let (a, pa, b, pb) = if i % 2 == 0 {
                let t = Instant::now();
                let (a,pa) = recover_batched(&mut ar, &half, &pts, r, &mut old);
                old_ns += t.elapsed().as_nanos();
                let t = Instant::now();
                let (b,pb) = recover_paired(&mut ar, &half, &reps, &pts, r, &mut paired);
                paired_ns += t.elapsed().as_nanos();
                (a,pa,b,pb)
            } else {
                let t = Instant::now();
                let (b,pb) = recover_paired(&mut ar, &half, &reps, &pts, r, &mut paired);
                paired_ns += t.elapsed().as_nanos();
                let t = Instant::now();
                let (a,pa) = recover_batched(&mut ar, &half, &pts, r, &mut old);
                old_ns += t.elapsed().as_nanos();
                (a,pa,b,pb)
            };
            assert_eq!(a.is_some(), b.is_some(), "membership mask changed at target {i}");
            if b.is_some() { hit_count += 1; } else {
                exhaustive_negatives += 1;
                assert_eq!(pa, half.ordered.len(), "baseline negative not exhaustive");
                assert_eq!(pb, reps.len(), "paired negative not exhaustive");
            }
        }
        assert_eq!(hit_count, 244, "frozen first-x 256-Q coverage mismatch");
        assert_eq!(exhaustive_negatives, 12);

        // Identity and same-x/tangent/add-inverse branches must use original EC.
        let rp = reps.iter().map(|r| r.point).find(|p| p.is_some()).expect("nonzero 4A point");
        let cases = [None, rp, ar.neg(PRIME31, rp), Some(PRIME31.g)];
        let mut corner_verified = 0;
        for &r in &cases {
            let (a, _) = recover_batched(&mut ar, &half, &pts, r, &mut old);
            let (b, _) = recover_paired(&mut ar, &half, &reps, &pts, r, &mut paired);
            assert_eq!(a.is_some(), b.is_some());
            corner_verified += 1;
        }
        assert!(paired.exceptional > 0, "exceptional arithmetic was not exercised");
        println!("B31_NEGATION_QUOTIENT_COMPONENT_JSON={}", json!({
            "schema":"prime31.b31.negation_quotient.component.v1",
            "validation":"producer native Rust [pcc:unverified]",
            "field_p":PRIME31.p,"subgroup_n":PRIME31.n,
            "base":"first31_rational_x","public_corpus_seed":format!("{CORPUS_SEED:#018x}"),
            "original_four_sum_points":half.index.len(),
            "canonical_representatives_including_identity":reps.len(),
            "additional_candidate_setup_ns":additional_setup_ns.to_string(),
            "matched_256q_baseline_query_ns":old_ns.to_string(),
            "matched_256q_paired_query_ns":paired_ns.to_string(),
            "paired_query_plus_extra_setup_ns":(paired_ns+additional_setup_ns).to_string(),
            "first_shift_hits":hit_count,"exhaustive_negatives":exhaustive_negatives,
            "corner_controls_verified":corner_verified,
            "baseline_batch_stats":format!("{old:?}"),
            "paired_stats":format!("{paired:?}"),
            "signed_witness_verified_for_each_success":true,
            "novel_non_generic_ecdlp":false,
            "complete_solver_measured_here":false,
        }));
    }
}
