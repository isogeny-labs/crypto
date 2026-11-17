//! Exact signed eight-leaf membership oracle on ordinary prime31 via 4A+4A.
//! This is GENERIC point-sum meet-in-the-middle, a nonenumerative-S9 extractor
//! validation oracle, NOT a non-generic index-calculus breakthrough.
//!
//! Reuses prime31_algebraic_ic::{PRIME31, Arithmetic, check_curve, Point}.
//! Frozen study: S9_B31_FOUR_PLUS_FOUR_ORACLE_PROTOCOL_20261116.md.
//!
//! cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s9_b31_half_sum_oracle
//! cargo +1.100.0 run  --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s9_b31_half_sum_oracle
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
    struct BatchStats {
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

    fn recover_batched(
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
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).expect("original cofactor-one prime31 curve");
        let base = Base::build(PRIME31, 6, &mut ar).expect("unchanged Dickson31");
        assert_eq!(base.points.len(), B31);
        let pts = tokens(&ar, &base.points);
        let half = FourSet::build(&mut ar, &pts);
        assert_eq!(half.index.len(), 618_885);

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
                let delta = ar.scalar(PRIME31, shift, Some(PRIME31.g));
                let residual = ar.add(PRIME31, q, delta);
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
