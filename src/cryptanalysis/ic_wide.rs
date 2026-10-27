//! # Index calculus and rho past one word: `k = 4` curves over `F_{(2^m)^4}`, `m ≥ 16`.
//!
//! `#E(F_{q⁴}) ≈ 2^{4m}`, so from `m = 16` a curve with a small cofactor
//! has a subgroup order `r ≥ 2^64`, and the one-word pipeline
//! (`ic_framework::run_pipeline`, `ic_boundary::rho_walk_with`) cannot
//! hold its scalars.  This module is those two loops written once over a
//! scalar `S` ([`RhoScalar`]: `u64`, or `u128` for `r < 2^127`), with
//! everything else unchanged:
//!
//! - [`rho_negation`] is `rho_walk_with(NegationClasses, RhoWalk::negation())`:
//!   the same draws from the seed, the same jump table and distinguished
//!   points, look-ahead, cycle escape and stride starts, the same counters;
//! - [`run_f2m4`] is `run_pipeline` with `bin-t-line`, `gaudry-d4-bin`, the
//!   `walk` target source (lazy restart pool, guarded) and the `wiedemann`
//!   matrix: the same walk, the same oracle calls, the same rows, pricing
//!   and report.
//!
//! At `S = u64` both are the one-word code step for step: the tests run
//! them beside the originals on curves with `m ≤ 15` and require every
//! count, every counter and the answer to agree.  That is what lets a
//! `u128` run stand next to the committed one-word sessions.
//!
//! Point keys past `4m + 1 = 64` bits are a 64-bit mix of the exact key
//! (`gaudry_quartic_bin::wide_key`); every use of a key is followed by an
//! exact check, so a shared key can only waste a step.

use std::collections::{BTreeMap, HashMap};
use std::time::Instant;

use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::cryptanalysis::gaudry_quartic_bin::{
    t_line_coef, BinCurve, BinD4Oracle, BinRelation, CurveB, Eb, PtB, TLineBase,
};
use crate::cryptanalysis::ic_boundary::{
    generic_floor_ops, price_phase, Calibration, CountedGroup, GroupOps, OracleCounters, PhaseCost,
    RhoResult,
};
use crate::cryptanalysis::ic_framework::linalg::Wiedemann;
use crate::cryptanalysis::ic_framework::stages::{
    DecompositionOracle, FactorBaseBuilder, InstanceCtx, Targets,
};
use crate::cryptanalysis::ic_framework::{
    DecompositionReport, FactorBaseReport, LinearAlgebraReport, PipelineSpec, RunReport,
    SolverReport,
};
use crate::cryptanalysis::koblitz_strong_rho::RhoScalar;

fn s_of<S: RhoScalar>(v: u128) -> S {
    S::from_biguint(&BigUint::from(v)).expect("fits the scalar")
}

/// `rng.gen_range(lo..hi)`, drawn as a `u64` while `hi` fits one, so a
/// `u128` run on a one-word order draws what the one-word code draws.
fn gen_below<S: RhoScalar>(rng: &mut StdRng, lo: S, hi: S) -> S {
    match (u64::try_from(lo.to_u128()), u64::try_from(hi.to_u128())) {
        (Ok(l), Ok(h)) => s_of(u128::from(rng.gen_range(l..h))),
        _ => rng.gen_range(lo..hi),
    }
}

/// `ic_boundary::mix`: the walk's hash.
fn mix(v: u64) -> u64 {
    let mut h = v ^ 0x2545_F491_4F6C_DD1D;
    h = h.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    h ^= h >> 31;
    h = h.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    h ^= h >> 29;
    h
}

/// `[k]p` by `CountedGroup::mul`'s double-and-add, for a scalar of any width.
pub fn mul_s<G: CountedGroup, S: RhoScalar>(g: &G, ops: &mut GroupOps, p: G::Elt, k: S) -> G::Elt {
    ops.scalar_mults += 1;
    if k == S::zero() || g.is_identity(&p) {
        return g.identity();
    }
    let bits = k.bit_len();
    let mut acc = p;
    for i in (0..bits - 1).rev() {
        acc = g.double(ops, acc);
        if k.bit(i) {
            acc = g.add(ops, acc, p);
        }
    }
    acc
}

/// `ic_boundary::joint_mul`: `[a]p + [b]q` by one joint double-and-add,
/// `p + q` supplied; one scalar multiplication.
fn joint_mul_s<G: CountedGroup, S: RhoScalar>(
    g: &G,
    ops: &mut GroupOps,
    p: G::Elt,
    q: G::Elt,
    pq: G::Elt,
    a: S,
    b: S,
) -> G::Elt {
    ops.scalar_mults += 1;
    let top = a.bit_len().max(b.bit_len());
    if top == 0 {
        return g.identity();
    }
    let pick = |i: u32| match (a.bit(i), b.bit(i)) {
        (true, false) => Some(p),
        (false, true) => Some(q),
        (true, true) => Some(pq),
        _ => None,
    };
    let mut acc = pick(top - 1).expect("the top bit of a | b is set");
    for i in (0..top - 1).rev() {
        acc = g.double(ops, acc);
        if let Some(t) = pick(i) {
            acc = g.add(ops, acc, t);
        }
    }
    acc
}

// ── Rho with the negation map ───────────────────────────────────────────

#[derive(Clone, Copy, Debug, Default)]
struct Tally {
    steps: u64,
    look_ahead_adds: u64,
    escapes: u64,
    cycles_2: u64,
    cycles_4: u64,
    cycles_other: u64,
    abandoned: u64,
    capped: u64,
    canonicalisations: u64,
    // Native work the unbatched walk does besides its group operations,
    // read by `rho_negation_tallied` (nothing else depends on it).
    key_evals: u64,
    mix_evals: u64,
    neg_evals: u64,
    flips: u64,
    scalar_mod_ops: u64,
    window_scans: u64,
    window_scan_elems: u64,
    window_pushes: u64,
    table_queries: u64,
    table_inserts: u64,
    scalar_mulmods: u64,
    scalar_invmods: u64,
    rng_draws: u64,
}

/// What [`rho_negation`] did besides its group operations: the native
/// work the full-pricing methods charge.
#[derive(Clone, Copy, Debug, Default)]
pub struct NegationNative {
    pub canonicalisations: u64,
    pub key_evals: u64,
    pub mix_evals: u64,
    pub neg_evals: u64,
    pub canonical_flips: u64,
    pub scalar_mod_ops: u64,
    pub window_scans: u64,
    pub window_scan_elems: u64,
    pub window_pushes: u64,
    pub table_queries: u64,
    pub table_inserts: u64,
    pub scalar_mulmods: u64,
    pub scalar_invmods: u64,
    pub rng_draws: u64,
    pub starts: u64,
}

impl Tally {
    fn walk_ops(&self) -> u64 {
        self.steps + self.look_ahead_adds + self.escapes
    }
}

/// `{P, −P}`'s representative: the smaller key, the coefficients negated
/// with it (`rho_canon` with `NegationClasses`).
fn canon<G: CountedGroup, S: RhoScalar>(
    g: &G,
    p: G::Elt,
    a: S,
    b: S,
    r: S,
    tally: &mut Tally,
) -> (G::Elt, S, S, bool) {
    tally.canonicalisations += 1;
    tally.neg_evals += 1;
    tally.key_evals += 2;
    let n = g.neg(p);
    if g.key(&n) < g.key(&p) {
        tally.flips += 1;
        tally.scalar_mod_ops += 2;
        (
            n,
            S::sub_mod(S::zero(), a, r),
            S::sub_mod(S::zero(), b, r),
            true,
        )
    } else {
        (p, a, b, false)
    }
}

/// `ic_boundary::rho_walk_with(g, &NegationClasses { r }, …,
/// RhoWalk::negation())` at any scalar width: the result, and the
/// logarithm (`RhoResult::recovered` holds it too while it fits a word).
pub fn rho_negation<G: CountedGroup, S: RhoScalar>(
    g: &G,
    generator: G::Elt,
    target: G::Elt,
    r: S,
    seed: u64,
    max_steps: u64,
) -> (RhoResult, Option<S>) {
    let (res, d, _) = rho_negation_tallied(g, generator, target, r, seed, max_steps);
    (res, d)
}

/// [`rho_negation`], and the native work it did besides its group
/// operations (the walk, its draws and counts are identical).
pub fn rho_negation_tallied<G: CountedGroup, S: RhoScalar>(
    g: &G,
    generator: G::Elt,
    target: G::Elt,
    r: S,
    seed: u64,
    max_steps: u64,
) -> (RhoResult, Option<S>, NegationNative) {
    let start = Instant::now();
    let mut ops = GroupOps::default();
    let mut rng = StdRng::seed_from_u64(seed);
    // WalkShape::of(r, RhoWalk::negation())
    let bits = r.bit_len();
    let jump_count = match bits {
        0..=15 => 4usize,
        16..=22 => 8,
        _ => 16,
    };
    let dp_bits = (bits / 4).min(40);
    let dp_mask = if dp_bits == 0 {
        0
    } else {
        (1u64 << dp_bits) - 1
    };
    let log = usize::BITS - 1 - jump_count.max(2).leading_zeros();
    let coefficient_bits = (3 * log).clamp(6, 12);
    let walk_cap = (1u64 << dp_bits) * 20 + 64;
    let window = 16usize;

    let cap_s: S = s_of(1u128 << coefficient_bits);
    let two: S = s_of(2);
    let short = (if r < cap_s { r } else { cap_s }).max(two);
    let r2 = r.max(two);
    let mut tally = Tally {
        // Two draws per jump, two for the stride, and the scalar
        // reductions of the jump coefficients and the start.
        rng_draws: 2 * jump_count as u64 + 2,
        scalar_mod_ops: 2 * jump_count as u64 + 2,
        ..Tally::default()
    };
    let gq = g.add(&mut ops, generator, target);
    let jumps: Vec<(G::Elt, S, S)> = (0..jump_count)
        .map(|_| {
            let a = gen_below(&mut rng, S::one(), short);
            let b = gen_below(&mut rng, S::one(), short);
            (
                joint_mul_s(g, &mut ops, generator, target, gq, a, b),
                S::add_mod(a, S::zero(), r),
                S::add_mod(b, S::zero(), r),
            )
        })
        .collect();
    let (mut ta, mut tb) = (
        gen_below(&mut rng, S::one(), r2),
        gen_below(&mut rng, S::one(), r2),
    );
    let mut stride = joint_mul_s(g, &mut ops, generator, target, gq, ta, tb);
    while g.is_identity(&stride) {
        ta = gen_below(&mut rng, S::one(), r2);
        tb = gen_below(&mut rng, S::one(), r2);
        tally.rng_draws += 2;
        stride = joint_mul_s(g, &mut ops, generator, target, gq, ta, tb);
    }
    let setup = ops;

    let index = |t: &mut Tally, x: &G::Elt| {
        t.key_evals += 1;
        t.mix_evals += 1;
        ((mix(g.key(x)) >> 24) % jump_count as u64) as usize
    };
    let mut table: HashMap<u64, (S, S)> = HashMap::with_capacity(1024);
    let expected = generic_floor_ops(r.to_f64(), 2.0);
    let (mut walks, mut dps, mut start_adds) = (0u64, 0u64, 0u64);
    let (mut useless, mut verification_ops) = (0u64, 0u64);
    let mut recovered: Option<S> = None;
    let (mut sx, mut sa, mut sb) = (
        stride,
        S::add_mod(ta, S::zero(), r),
        S::add_mod(tb, S::zero(), r),
    );
    let mut recent: std::collections::VecDeque<(u64, G::Elt, S, S)> =
        std::collections::VecDeque::with_capacity(window + 1);

    while tally.walk_ops() < max_steps && walks < max_steps {
        walks += 1;
        if walks > 1 {
            sx = g.add(&mut ops, sx, stride);
            sa = S::add_mod(sa, ta, r);
            sb = S::add_mod(sb, tb, r);
            tally.scalar_mod_ops += 2;
            start_adds += 1;
        }
        if g.is_identity(&sx) {
            continue;
        }
        // TunedWalker::walk
        let end: Option<(G::Elt, S, S)> = 'walk: {
            let (mut x, mut a, mut b, _) = canon(g, sx, sa, sb, r, &mut tally);
            recent.clear();
            let mut last_escape: Option<u64> = None;
            let mut len = 0u64;
            loop {
                let j0 = index(&mut tally, &x);
                let mut tried = 0usize;
                let (y, ya, yb) = loop {
                    let j = (j0 + tried) % jump_count;
                    let (m, aj, bj) = jumps[j];
                    let sum = g.add(&mut ops, x, m);
                    tally.scalar_mod_ops += 2;
                    let (y, ya, yb, flipped) = canon(
                        g,
                        sum,
                        S::add_mod(a, aj, r),
                        S::add_mod(b, bj, r),
                        r,
                        &mut tally,
                    );
                    tried += 1;
                    if flipped
                        && tried < jump_count
                        && !g.is_identity(&y)
                        && index(&mut tally, &y) == j
                    {
                        tally.look_ahead_adds += 1;
                        continue;
                    }
                    break (y, ya, yb);
                };
                x = y;
                a = ya;
                b = yb;
                tally.steps += 1;
                len += 1;
                if g.is_identity(&x) {
                    break 'walk None;
                }
                let key = g.key(&x);
                tally.key_evals += 1;
                tally.window_scans += 1;
                let found = recent.iter().position(|e| e.0 == key);
                tally.window_scan_elems += found.map_or(recent.len(), |p| p + 1) as u64;
                if let Some(pos) = found {
                    let c = recent.len() - pos;
                    tally.window_scan_elems += c as u64;
                    match c {
                        2 => tally.cycles_2 += 1,
                        4 => tally.cycles_4 += 1,
                        _ => tally.cycles_other += 1,
                    }
                    let (min_key, mp, ma, mb) = recent
                        .iter()
                        .skip(pos)
                        .copied()
                        .min_by_key(|e| e.0)
                        .expect("a cycle has members");
                    if last_escape == Some(min_key) {
                        tally.abandoned += 1;
                        break 'walk None;
                    }
                    last_escape = Some(min_key);
                    let d2 = g.double(&mut ops, mp);
                    tally.escapes += 1;
                    tally.scalar_mod_ops += 2;
                    let (e, ea, eb, _) = canon(
                        g,
                        d2,
                        S::add_mod(ma, ma, r),
                        S::add_mod(mb, mb, r),
                        r,
                        &mut tally,
                    );
                    x = e;
                    a = ea;
                    b = eb;
                    recent.clear();
                    if g.is_identity(&x) {
                        break 'walk None;
                    }
                }
                recent.push_back((g.key(&x), x, a, b));
                tally.key_evals += 2;
                tally.mix_evals += 1;
                tally.window_pushes += 1;
                if recent.len() > window {
                    recent.pop_front();
                }
                if mix(g.key(&x)) & dp_mask == 0 {
                    break 'walk Some((x, a, b));
                }
                if len > walk_cap {
                    tally.capped += 1;
                    break 'walk None;
                }
                if tally.walk_ops() >= max_steps {
                    break 'walk None;
                }
            }
        };
        let Some((x, a, b)) = end else {
            continue;
        };
        dps += 1;
        let key = g.key(&x);
        tally.key_evals += 1;
        tally.table_queries += 1;
        match table.get(&key) {
            Some(&(a2, b2)) => {
                if b2 != b {
                    let num = S::sub_mod(a, a2, r);
                    let den = S::sub_mod(b2, b, r);
                    tally.scalar_mod_ops += 2;
                    tally.scalar_invmods += 1;
                    if let Some(inv) = S::inverse_mod(den, r) {
                        tally.scalar_mulmods += 1;
                        let d = S::mul_mod(num, inv, r);
                        let before = ops.gae();
                        let check = mul_s(g, &mut ops, generator, d);
                        verification_ops += (ops.gae() - before) as u64;
                        if check == target {
                            recovered = Some(d);
                            break;
                        }
                    }
                }
                useless += 1;
            }
            None => {
                tally.table_inserts += 1;
                table.insert(key, (a, b));
            }
        }
    }
    let wall_ns = start.elapsed().as_nanos() as u64;
    let gae = ops.gae();
    let walked = tally.walk_ops();
    let rf = r.to_f64();
    let native = NegationNative {
        canonicalisations: tally.canonicalisations,
        key_evals: tally.key_evals,
        mix_evals: tally.mix_evals,
        neg_evals: tally.neg_evals,
        canonical_flips: tally.flips,
        scalar_mod_ops: tally.scalar_mod_ops,
        window_scans: tally.window_scans,
        window_scan_elems: tally.window_scan_elems,
        window_pushes: tally.window_pushes,
        table_queries: tally.table_queries,
        table_inserts: tally.table_inserts,
        scalar_mulmods: tally.scalar_mulmods,
        scalar_invmods: tally.scalar_invmods,
        rng_draws: tally.rng_draws,
        starts: walks,
    };
    let mut counters = BTreeMap::new();
    for (k, v) in [
        ("jumps", jump_count as u64),
        ("jump_coefficient_bits", coefficient_bits as u64),
        ("distinguished_point_bits", dp_bits as u64),
        ("setup_additions", setup.adds),
        ("setup_doublings", setup.doubles),
        ("start_additions", start_adds),
        ("walk_operations", walked),
        ("verification_operations", verification_ops),
        ("look_ahead_additions", tally.look_ahead_adds),
        ("cycle_escape_doublings", tally.escapes),
        ("cycles_length_2", tally.cycles_2),
        ("cycles_length_4", tally.cycles_4),
        ("cycles_other_length", tally.cycles_other),
        ("walks_abandoned_in_a_cycle", tally.abandoned),
        ("walks_capped", tally.capped),
        ("useless_collisions", useless),
        ("canonicalisations_uncharged", tally.canonicalisations),
    ] {
        counters.insert(k.to_string(), v);
    }
    let res = RhoResult {
        method: "negation-map r-adding walk: look-ahead, short-cycle escape by doubling, distinguished points, stride starts".into(),
        automorphisms: 2,
        group_ops: ops,
        steps: tally.steps,
        walks,
        distinguished_points: dps,
        gae,
        s: gae / rf.sqrt(),
        s_walk: walked as f64 / rf.sqrt(),
        expected_steps: expected,
        steps_over_expected: walked as f64 / expected,
        wall_ns,
        verified: recovered.is_some(),
        recovered: recovered.and_then(|d| u64::try_from(d.to_u128()).ok()),
        counters,
    };
    (res, recovered, native)
}

// ── Rho with the negation map, many walks in lockstep, batched inversion ─

/// Parameters of [`rho_negation_batched`].
#[derive(Clone, Copy, Debug)]
pub struct BatchedRhoParams {
    /// Parallel walks advanced one step per round; one field inversion
    /// per round.
    pub walks: usize,
    /// Distinguished-point bits; 0 takes the unbatched walk's `bits(r)/4`.
    pub dp_bits: u32,
}

impl Default for BatchedRhoParams {
    fn default() -> Self {
        BatchedRhoParams {
            walks: 256,
            dp_bits: 0,
        }
    }
}

/// Everything [`rho_negation_batched`] counted.  `F_q` operations are the
/// `Gf2q` counter's (a multiplication, a squaring and an inversion are one
/// each, exactly as `gaudry-d4-bin` and `fq_muls_per_add` count them); the
/// `*_uncharged` fields are native work with no `F_q` multiplication in it.
#[derive(Clone, Debug, Default)]
pub struct BatchedRho {
    pub rounds: u64,
    pub walks_started: u64,
    pub distinguished_points: u64,
    pub useless_collisions: u64,
    /// Accepted steps (one affine addition each).
    pub steps: u64,
    pub look_ahead_adds: u64,
    pub escapes: u64,
    pub cycles_2: u64,
    pub cycles_4: u64,
    pub cycles_other: u64,
    pub abandoned: u64,
    pub capped: u64,
    /// Affine additions inside the lockstep batches.
    pub batched_adds: u64,
    /// Additions and doublings done outside a batch, each with its own
    /// inversion (starts, escapes, `x1 = x2` cases, set-up).
    pub single_ops: u64,
    pub batch_inversions: u64,
    /// `F_{q^4}` multiplications spent by Montgomery's trick: `3(n−1)` per batch.
    pub batch_fq4_muls: u64,
    // F_q operations, by region; `fq_total` is their sum.
    pub fq_setup: u64,
    pub fq_batch_inversion: u64,
    pub fq_batched_formula: u64,
    pub fq_single_ops: u64,
    pub fq_verification: u64,
    pub fq_total: u64,
    // The same regions in the reference pricing (`Fq4::ref_muls`).
    pub ref_fq_setup: u64,
    pub ref_fq_batch_inversion: u64,
    pub ref_fq_batched_formula: u64,
    pub ref_fq_single_ops: u64,
    pub ref_fq_verification: u64,
    pub ref_fq_total: u64,
    /// The same total under `RefModel::PolyFreeConst` (`Fq4::ref_muls_poly`).
    pub poly_fq_total: u64,
    pub setup_ops: GroupOps,
    pub verification_ops: u64,
    // Native work, not F_q multiplications.
    pub canonicalisations: u64,
    pub canonical_flips: u64,
    /// `F_q` additions (XOR) inside canonicalisation, 4 per call.
    pub canon_fq_additions: u64,
    pub hash_evals: u64,
    pub table_queries: u64,
    pub table_inserts: u64,
    /// Additions, subtractions and negations of the `(a, b)` coefficients mod `r`.
    pub scalar_mod_ops: u64,
    /// Scans of a walk's recent-point window (one per accepted step that
    /// is not the identity), the entries they compared, the entries a
    /// cycle's minimum was taken over, and the points pushed.
    pub window_scans: u64,
    pub window_scan_elems: u64,
    pub window_pushes: u64,
    /// Modular multiplications and inversions of the collision solve.
    pub scalar_mulmods: u64,
    pub scalar_invmods: u64,
    /// Random draws of the set-up.
    pub rng_draws: u64,
    pub jumps: u64,
    pub dp_bits: u32,
    pub expected_steps: f64,
}

struct BWalk<S> {
    x: PtB,
    a: S,
    b: S,
    tried: usize,
    j0: usize,
    len: u64,
    last_escape: Option<u128>,
    recent: std::collections::VecDeque<(u128, PtB, S, S)>,
}

/// Negation-map r-adding walk with distinguished points, `walks` walks in
/// lockstep, one Montgomery batch inversion per round.  The walk's rules
/// are those of [`rho_negation`] (jump table, look-ahead, 2- and 4-cycle
/// escape by doubling, stride starts), with the jump index and the
/// distinguished-point test read from a hash of the abscissa alone and
/// the representative of `{P, −P}` the one with the smaller packed
/// ordinate.  A new method: `rho_negation` itself is unchanged.
#[allow(clippy::too_many_arguments)]
pub fn rho_negation_batched<S: RhoScalar>(
    c: &CurveB,
    generator: PtB,
    target: PtB,
    r: S,
    seed: u64,
    max_steps: u64,
    par: BatchedRhoParams,
) -> (BatchedRho, Option<S>) {
    let f = &c.f;
    let mut recovered: Option<S> = None;
    let fq = || f.f.muls();
    let rq = || f.ref_muls();
    let start_count = fq();
    let rstart = rq();
    let pstart = f.ref_muls_poly();
    let mut out = BatchedRho::default();
    let mut ops = GroupOps::default();
    let mut rng = StdRng::seed_from_u64(seed);
    let bits = r.bit_len();
    let jump_count = match bits {
        0..=15 => 4usize,
        16..=22 => 8,
        _ => 16,
    };
    let dp_bits = if par.dp_bits == 0 {
        (bits / 4).min(40)
    } else {
        par.dp_bits.min(40)
    };
    let dp_mask = if dp_bits == 0 {
        0
    } else {
        (1u64 << dp_bits) - 1
    };
    let log = usize::BITS - 1 - jump_count.max(2).leading_zeros();
    let coefficient_bits = (3 * log).clamp(6, 12);
    let walk_cap = (1u64 << dp_bits) * 20 + 64;
    let window = 16usize;
    let n_walks = par.walks.max(1);

    let cap_s: S = s_of(1u128 << coefficient_bits);
    let two: S = s_of(2);
    let short = (if r < cap_s { r } else { cap_s }).max(two);
    let r2 = r.max(two);
    out.rng_draws = 2 * jump_count as u64 + 2;
    out.scalar_mod_ops += 2 * jump_count as u64 + 2;
    let gq = c.add(&generator, &target);
    ops.adds += 1;
    let jumps: Vec<(PtB, S, S)> = (0..jump_count)
        .map(|_| {
            let a = gen_below(&mut rng, S::one(), short);
            let b = gen_below(&mut rng, S::one(), short);
            (
                joint_mul_s(c, &mut ops, generator, target, gq, a, b),
                S::add_mod(a, S::zero(), r),
                S::add_mod(b, S::zero(), r),
            )
        })
        .collect();
    let (mut ta, mut tb) = (
        gen_below(&mut rng, S::one(), r2),
        gen_below(&mut rng, S::one(), r2),
    );
    let mut stride = joint_mul_s(c, &mut ops, generator, target, gq, ta, tb);
    while c.is_identity(&stride) {
        ta = gen_below(&mut rng, S::one(), r2);
        tb = gen_below(&mut rng, S::one(), r2);
        out.rng_draws += 2;
        stride = joint_mul_s(c, &mut ops, generator, target, gq, ta, tb);
    }
    out.setup_ops = ops;
    out.fq_setup = fq() - start_count;
    out.ref_fq_setup = rq().wrapping_sub(rstart);
    out.jumps = jump_count as u64;
    out.dp_bits = dp_bits;
    out.expected_steps = generic_floor_ops(r.to_f64(), 2.0);

    // The class representative: the smaller packed ordinate of y and x + y.
    let canon_b = |out: &mut BatchedRho, p: PtB, a: S, b: S| -> (PtB, S, S, bool) {
        out.canonicalisations += 1;
        out.canon_fq_additions += 4;
        if p.inf {
            return (p, a, b, false);
        }
        let ny = p.x.add(&p.y);
        if f.pack(&ny) < f.pack(&p.y) {
            out.canonical_flips += 1;
            out.scalar_mod_ops += 2;
            (
                PtB {
                    x: p.x,
                    y: ny,
                    inf: false,
                },
                S::sub_mod(S::zero(), a, r),
                S::sub_mod(S::zero(), b, r),
                true,
            )
        } else {
            (p, a, b, false)
        }
    };
    // 64-bit hash of the abscissa: the jump index and the DP test.
    let hash_x = |out: &mut BatchedRho, p: &PtB| -> u64 {
        out.hash_evals += 1;
        mix(crate::cryptanalysis::gaudry_quartic_bin::wide_key(
            f.pack(&p.x),
        ))
    };
    let index_of = |h: u64| ((h >> 24) % jump_count as u64) as usize;

    let mut table: HashMap<u128, (S, S)> = HashMap::with_capacity(4096);
    let (mut sx, mut sa, mut sb) = (
        stride,
        S::add_mod(ta, S::zero(), r),
        S::add_mod(tb, S::zero(), r),
    );
    let mut first_start = true;

    // A fresh walk from the next stride start, entered; None if it is the identity.
    let mut next_start = |out: &mut BatchedRho| -> Option<(BWalk<S>, u64)> {
        loop {
            out.walks_started += 1;
            if !first_start {
                let before = fq();
                let rbefore = rq();
                sx = c.add(&sx, &stride);
                out.fq_single_ops += fq() - before;
                out.ref_fq_single_ops += rq().wrapping_sub(rbefore);
                out.single_ops += 1;
                sa = S::add_mod(sa, ta, r);
                sb = S::add_mod(sb, tb, r);
                out.scalar_mod_ops += 2;
            }
            first_start = false;
            if sx.inf {
                if out.walks_started > max_steps {
                    return None;
                }
                continue;
            }
            let (x, a, b, _) = canon_b(out, sx, sa, sb);
            let h = hash_x(out, &x);
            return Some((
                BWalk {
                    x,
                    a,
                    b,
                    tried: 0,
                    j0: index_of(h),
                    len: 0,
                    last_escape: None,
                    recent: std::collections::VecDeque::with_capacity(window + 1),
                },
                h,
            ));
        }
    };

    let mut pool: Vec<Option<BWalk<S>>> = Vec::with_capacity(n_walks);
    for _ in 0..n_walks {
        pool.push(next_start(&mut out).map(|w| w.0));
    }
    let walk_ops = |o: &BatchedRho| o.steps + o.look_ahead_adds + o.escapes;

    'rounds: while walk_ops(&out) < max_steps {
        out.rounds += 1;
        // 1. The additions of this round.
        struct Job {
            slot: usize,
            jump: usize,
            denom: Option<Eb>,
            sum: Option<PtB>,
        }
        let mut jobs: Vec<Job> = Vec::with_capacity(n_walks);
        for (slot, w) in pool.iter().enumerate() {
            let Some(w) = w else { continue };
            let j = (w.j0 + w.tried) % jump_count;
            let m = &jumps[j].0;
            let d = w.x.x.add(&m.x);
            if m.inf || d.is_zero() {
                jobs.push(Job {
                    slot,
                    jump: j,
                    denom: None,
                    sum: None,
                });
            } else {
                jobs.push(Job {
                    slot,
                    jump: j,
                    denom: Some(d),
                    sum: None,
                });
            }
        }
        if jobs.is_empty() {
            break;
        }
        // 2. Singles (x1 = x2, or the identity as jump): their own inversion.
        for job in jobs.iter_mut().filter(|j| j.denom.is_none()) {
            let w = pool[job.slot].as_ref().unwrap();
            let before = fq();
            let rbefore = rq();
            job.sum = Some(c.add(&w.x, &jumps[job.jump].0));
            out.fq_single_ops += fq() - before;
            out.ref_fq_single_ops += rq().wrapping_sub(rbefore);
            out.single_ops += 1;
        }
        // 3. Montgomery's trick over the rest: one inversion, 3(n−1) multiplications.
        let idx: Vec<usize> = (0..jobs.len())
            .filter(|&i| jobs[i].denom.is_some())
            .collect();
        if !idx.is_empty() {
            let before = fq();
            let rbefore = rq();
            let n = idx.len();
            let mut pref: Vec<Eb> = Vec::with_capacity(n);
            for (k, &i) in idx.iter().enumerate() {
                let d = jobs[i].denom.unwrap();
                pref.push(if k == 0 { d } else { f.mul(&pref[k - 1], &d) });
            }
            let mut acc = f.inv(&pref[n - 1]);
            let mut invs = vec![Eb::ZERO; n];
            for k in (1..n).rev() {
                invs[k] = f.mul(&acc, &pref[k - 1]);
                acc = f.mul(&acc, &jobs[idx[k]].denom.unwrap());
            }
            invs[0] = acc;
            out.fq_batch_inversion += fq() - before;
            out.ref_fq_batch_inversion += rq().wrapping_sub(rbefore);
            out.batch_inversions += 1;
            out.batch_fq4_muls += 3 * (n as u64 - 1);
            // 4. The addition formula with the inverse supplied.
            let before = fq();
            let rbefore = rq();
            for (k, &i) in idx.iter().enumerate() {
                let w = pool[jobs[i].slot].as_ref().unwrap();
                let (p, q) = (&w.x, &jumps[jobs[i].jump].0);
                let l = f.mul(&p.y.add(&q.y), &invs[k]);
                let x3 = f.sq(&l).add(&l).add(&p.x).add(&q.x).add(&c.a);
                let y3 = f.mul(&l, &p.x.add(&x3)).add(&x3).add(&p.y);
                jobs[i].sum = Some(PtB {
                    x: x3,
                    y: y3,
                    inf: false,
                });
            }
            out.fq_batched_formula += fq() - before;
            out.ref_fq_batched_formula += rq().wrapping_sub(rbefore);
            out.batched_adds += n as u64;
        }
        // 5. Each walk takes its sum.
        for job in jobs {
            let slot = job.slot;
            let sum = job.sum.unwrap();
            let mut w = pool[slot].take().unwrap();
            let (aj, bj) = (jumps[job.jump].1, jumps[job.jump].2);
            out.scalar_mod_ops += 2;
            let (y, ya, yb, flipped) = canon_b(
                &mut out,
                sum,
                S::add_mod(w.a, aj, r),
                S::add_mod(w.b, bj, r),
            );
            let hy = if y.inf { 0 } else { hash_x(&mut out, &y) };
            if flipped && w.tried + 1 < jump_count && !y.inf && index_of(hy) == job.jump {
                // Wiener–Zuccherato look-ahead: try the next jump from the same point.
                out.look_ahead_adds += 1;
                w.tried += 1;
                pool[slot] = Some(w);
                continue;
            }
            w.x = y;
            w.a = ya;
            w.b = yb;
            w.tried = 0;
            out.steps += 1;
            w.len += 1;
            let mut h = hy;
            let mut end: Option<Option<(PtB, S, S)>> = None; // Some(None) = dead walk, Some(Some) = DP
            if w.x.inf {
                end = Some(None);
            } else {
                let kx = f.pack(&w.x.x);
                out.window_scans += 1;
                let found = w.recent.iter().position(|e| e.0 == kx);
                out.window_scan_elems += found.map_or(w.recent.len(), |p| p + 1) as u64;
                if let Some(pos) = found {
                    let cyc = w.recent.len() - pos;
                    out.window_scan_elems += cyc as u64;
                    match cyc {
                        2 => out.cycles_2 += 1,
                        4 => out.cycles_4 += 1,
                        _ => out.cycles_other += 1,
                    }
                    let (min_key, mp, ma, mb) = w
                        .recent
                        .iter()
                        .skip(pos)
                        .copied()
                        .min_by_key(|e| e.0)
                        .expect("a cycle has members");
                    if w.last_escape == Some(min_key) {
                        out.abandoned += 1;
                        end = Some(None);
                    } else {
                        w.last_escape = Some(min_key);
                        let before = fq();
                        let rbefore = rq();
                        let d2 = c.add(&mp, &mp);
                        out.fq_single_ops += fq() - before;
                        out.ref_fq_single_ops += rq().wrapping_sub(rbefore);
                        out.single_ops += 1;
                        out.escapes += 1;
                        out.scalar_mod_ops += 2;
                        let (e, ea, eb, _) =
                            canon_b(&mut out, d2, S::add_mod(ma, ma, r), S::add_mod(mb, mb, r));
                        w.x = e;
                        w.a = ea;
                        w.b = eb;
                        w.recent.clear();
                        if w.x.inf {
                            end = Some(None);
                        } else {
                            h = hash_x(&mut out, &w.x);
                        }
                    }
                }
                if end.is_none() {
                    out.window_pushes += 1;
                    w.recent.push_back((f.pack(&w.x.x), w.x, w.a, w.b));
                    if w.recent.len() > window {
                        w.recent.pop_front();
                    }
                    if h & dp_mask == 0 {
                        end = Some(Some((w.x, w.a, w.b)));
                    } else if w.len > walk_cap {
                        out.capped += 1;
                        end = Some(None);
                    } else {
                        w.j0 = index_of(h);
                    }
                }
            }
            match end {
                None => {
                    pool[slot] = Some(w);
                }
                Some(res) => {
                    if let Some((x, a, b)) = res {
                        out.distinguished_points += 1;
                        let key = f.pack(&x.x);
                        out.table_queries += 1;
                        match table.get(&key) {
                            Some(&(a2, b2)) => {
                                if b2 != b {
                                    let num = S::sub_mod(a, a2, r);
                                    let den = S::sub_mod(b2, b, r);
                                    out.scalar_mod_ops += 2;
                                    out.scalar_invmods += 1;
                                    if let Some(inv) = S::inverse_mod(den, r) {
                                        out.scalar_mulmods += 1;
                                        let d = S::mul_mod(num, inv, r);
                                        let before = fq();
                                        let rbefore = rq();
                                        let mut vops = GroupOps::default();
                                        let check = mul_s(c, &mut vops, generator, d);
                                        out.fq_verification += fq() - before;
                                        out.ref_fq_verification += rq().wrapping_sub(rbefore);
                                        out.verification_ops += vops.adds + vops.doubles;
                                        if check == target {
                                            recovered = Some(d);
                                            break 'rounds;
                                        }
                                    }
                                }
                                out.useless_collisions += 1;
                            }
                            None => {
                                out.table_inserts += 1;
                                table.insert(key, (a, b));
                            }
                        }
                    }
                    if walk_ops(&out) >= max_steps {
                        break 'rounds;
                    }
                    pool[slot] = next_start(&mut out).map(|s| s.0);
                }
            }
        }
    }
    out.fq_total = fq() - start_count;
    out.ref_fq_total = rq().wrapping_sub(rstart);
    out.poly_fq_total = f.ref_muls_poly().wrapping_sub(pstart);
    (out, recovered)
}

// ── The k = 4 pipeline ──────────────────────────────────────────────────

/// What [`run_f2m4`] returns: the report (its one-word `r`,
/// `group_order` and `recovered` fields saturate past `2^64`; the
/// logarithm is here) and the logarithm.
pub struct WideRun<S> {
    pub report: RunReport,
    pub recovered: Option<S>,
}

/// `run_pipeline(ctx, spec, TLineBase, BinD4Oracle, …)` with the walk
/// target source and the `wiedemann` matrix, at any scalar width.
/// `ctx` carries the curve; its one-word fields are not read (the base
/// and the oracle ignore them), and `r`, `h` come as `S`.
#[allow(clippy::too_many_arguments)]
pub fn run_f2m4<S: RhoScalar>(
    ctx: &InstanceCtx<CurveB>,
    inst: &BinCurve,
    spec: &PipelineSpec,
    oracle: &mut BinD4Oracle,
    r: S,
    h: S,
    calib: &Calibration,
) -> Result<WideRun<S>, String> {
    let started = Instant::now();
    if spec.targets != Targets::Walk {
        return Err("the wide pipeline runs the `walk` target source only".into());
    }
    if spec.linalg != "wiedemann" {
        return Err("the wide pipeline runs the `wiedemann` matrix only".into());
    }
    let g: &CurveB = &inst.curve;
    let (generator, target) = (ctx.generator, ctx.target);
    let base = TLineBase { curve: inst };

    // Stage 1: the factor base.
    let mut fb_ops = GroupOps::default();
    let fb_started = Instant::now();
    let fb = base.build(ctx, &spec.factor_base_params, &mut fb_ops)?;
    let mut fb_cost = fb.cost.clone();
    fb_cost.group_ops.merge(fb_ops);
    fb_cost.wall_ns = fb_cost.wall_ns.max(fb_started.elapsed().as_nanos() as u64);

    // Stage 2: the oracle's set-up.
    let mut prep_ops = GroupOps::default();
    let prep_started = Instant::now();
    oracle.prepare(ctx, &fb, &spec.oracle_params, &mut prep_ops)?;
    let mut prep_cost = PhaseCost::default();
    prep_cost.group_ops.merge(prep_ops);
    prep_cost.native = oracle.setup_native();
    prep_cost.wall_ns = prep_started.elapsed().as_nanos() as u64;

    // Stages 3 and 4: `collect_and_solve_relations` with
    // `TargetSource::Walk`, `RestartPool::Lazy`, `CompletionRule::TargetPinned`.
    let r128 = r.to_u128();
    let mut matrix: Wiedemann<S> = Wiedemann::new(fb.columns + 1, r);
    let seed = spec.seed;
    let max_trials = spec.max_trials;
    let mut rng = StdRng::seed_from_u64(seed ^ 0x5245_4C41_5449_4F4E);
    let mut rel = PhaseCost::default();
    let mut la = PhaseCost::default();
    let mut ver = PhaseCost::default();
    let mut ctr = OracleCounters::default();
    let d_col = fb.columns;
    let (mut trials, mut found) = (0u64, 0u64);
    let mut recovered: Option<S> = None;
    let mut verified = false;
    let rel_start = Instant::now();
    let mut la_ns = 0u64;
    let h_mod = S::add_mod(h, S::zero(), r);
    let mut direct_skipped = 0u64;
    let (mut single_column_rows, mut two_column_rows) = (0u64, 0u64);
    let mut column_parts: HashMap<u64, ()> = HashMap::with_capacity(64);
    let (mut repeated_column_rows, mut pinned_by_repeated_row) = (0u64, false);
    let jump_count = 16usize;
    let pool_count = 16usize;
    let draw = |rng: &mut StdRng, ops: &mut GroupOps| -> (PtB, S, S) {
        let aj = gen_below(rng, S::one(), r);
        let bj = gen_below(rng, S::one(), r);
        let ag = mul_s(g, ops, generator, aj);
        let bq = mul_s(g, ops, target, bj);
        (CountedGroup::add(g, ops, ag, bq), aj, bj)
    };
    let mut pool: Vec<(PtB, S, S)> = Vec::new();
    let mut pool_rng = StdRng::seed_from_u64(seed ^ 0x504F_4F4C_5F4F_4646);
    let mut pool_ops = GroupOps::default();
    let mut pool_taken = 0u64;
    let mut jump_perm: Vec<usize> = (0..jump_count).collect();
    let mut walk_point = g.identity();
    let (mut wa, mut wb) = (S::zero(), S::zero());
    let mut walk_fresh = true;
    let segment_cap = (r.to_f64().sqrt() * 4.0) as u64 + 65_536;
    let mut segment_steps = 0u64;
    let mut seen: HashMap<u64, ()> = HashMap::with_capacity(1 << 12);
    let (mut walk_steps, mut walk_restarts, mut repeats_skipped, mut guard_probes) =
        (0u64, 0u64, 0u64, 0u64);
    let jumps: Vec<(PtB, S, S)> = (0..jump_count)
        .map(|_| draw(&mut rng, &mut rel.group_ops))
        .collect();
    let walk_jumps = jump_count as u64;

    while trials < max_trials {
        if walk_fresh || segment_steps >= segment_cap {
            if walk_restarts == 0 {
                wa = gen_below(&mut rng, S::one(), r);
                wb = gen_below(&mut rng, S::one(), r);
                let ag = mul_s(g, &mut rel.group_ops, generator, wa);
                let bq = mul_s(g, &mut rel.group_ops, target, wb);
                walk_point = CountedGroup::add(g, &mut rel.group_ops, ag, bq);
            } else {
                let idx = (pool_taken as usize) % pool_count;
                pool_taken += 1;
                if idx == pool.len() {
                    let mut local = GroupOps::default();
                    let off = draw(&mut pool_rng, &mut local);
                    rel.group_ops.merge(local);
                    pool_ops.merge(local);
                    pool.push(off);
                }
                let (op, oa, ob) = pool[idx];
                walk_point = CountedGroup::add(g, &mut rel.group_ops, walk_point, op);
                wa = S::add_mod(wa, oa, r);
                wb = S::add_mod(wb, ob, r);
                for i in (1..jump_count).rev() {
                    jump_perm.swap(i, rng.gen_range(0..i + 1));
                }
            }
            walk_fresh = false;
            walk_restarts += 1;
            segment_steps = 0;
        } else {
            let hsh = mix(g.key(&walk_point));
            let j = jump_perm[((hsh >> 24) as usize) % jump_count];
            let (jp, aj, bj) = jumps[j];
            walk_point = CountedGroup::add(g, &mut rel.group_ops, walk_point, jp);
            wa = S::add_mod(wa, aj, r);
            wb = S::add_mod(wb, bj, r);
            walk_steps += 1;
            segment_steps += 1;
        }
        if g.is_identity(&walk_point) {
            walk_fresh = true;
            direct_skipped += 1;
            continue;
        }
        let (point, a, b) = (walk_point, wa, wb);
        trials += 1;
        guard_probes += 1;
        if seen.insert(g.key(&point), ()).is_some() {
            repeats_skipped += 1;
            walk_fresh = true;
            continue;
        }
        let relations = oracle.relations_for(
            ctx,
            &fb,
            &mut rel.group_ops,
            &mut ctr,
            point,
            a.to_u128(),
            b.to_u128(),
            |i| t_line_coef(i, r128),
        );
        if relations.is_empty() {
            continue;
        }
        let la_start = Instant::now();
        let mut stop = false;
        let mut la_timed = false;
        for relation in relations {
            found += 1;
            let mut row: BTreeMap<usize, S> = BTreeMap::new();
            let (ra, rb) = match relation {
                BinRelation::Summands(summands) => {
                    for &i in &summands {
                        let c = fb.col_of[i];
                        let e = row.entry(c).or_insert(S::zero());
                        *e = S::add_mod(*e, s_of(t_line_coef(i, r128)), r);
                    }
                    (a, b)
                }
                BinRelation::Row(f) => {
                    for &(c, coef) in &f.columns {
                        let e = row.entry(c).or_insert(S::zero());
                        *e = S::add_mod(*e, s_of(coef % r128), r);
                    }
                    (s_of(f.a), s_of(f.b))
                }
            };
            row.retain(|_, v| *v != S::zero());
            let support = row.len();
            match support {
                1 => single_column_rows += 1,
                2 => two_column_rows += 1,
                _ => {}
            }
            // ic_boundary::column_part_key over the non-zero columns.
            let mut acc = 0x9E37_79B9_7F4A_7C15u64;
            for (&c, &v) in &row {
                let v = v.to_u128();
                let v64 = (v as u64) ^ ((v >> 64) as u64).rotate_left(29);
                acc = mix(acc ^ mix((c as u64) << 32 ^ v64));
            }
            let repeated = column_parts.insert(acc, ()).is_some();
            if repeated {
                repeated_column_rows += 1;
            }
            let dv = S::sub_mod(S::zero(), S::mul_mod(h_mod, rb, r), r);
            let rhs = S::mul_mod(h_mod, ra, r);
            let mut sparse: Vec<(usize, S)> = row.into_iter().collect();
            if dv != S::zero() {
                sparse.push((d_col, dv));
            }
            matrix.add_sparse(sparse, rhs);
            if let Some(d) = matrix.pinned_value(d_col) {
                pinned_by_repeated_row = repeated;
                la_ns += la_start.elapsed().as_nanos() as u64;
                la_timed = true;
                let v_start = Instant::now();
                let check = mul_s(g, &mut ver.group_ops, generator, d);
                ver.wall_ns = v_start.elapsed().as_nanos() as u64;
                if check == target {
                    recovered = Some(d);
                    verified = true;
                } else {
                    ver.count("verification_failures", 1);
                }
                stop = true;
                break;
            }
        }
        if !la_timed {
            la_ns += la_start.elapsed().as_nanos() as u64;
        }
        if stop {
            break;
        }
    }
    rel.wall_ns = rel_start.elapsed().as_nanos() as u64 - la_ns - ver.wall_ns;
    rel.count("trials", trials);
    rel.count("relations", found);
    rel.count("lookups", ctr.lookups);
    rel.count("sqrt_solves", ctr.sqrt_solves);
    rel.count("inversions", ctr.inversions);
    rel.count("s4_pairs", ctr.s4_pairs);
    rel.count("s4_hits", ctr.s4_hits);
    rel.count("lift_failures", ctr.lift_failures);
    rel.count("unliftable_systems", ctr.unliftable_systems);
    rel.count("canonicalisations", ctr.canonicalisations);
    rel.count("frobfold_mismatches", ctr.frobfold_mismatches);
    rel.count("direct_relations_skipped", direct_skipped);
    rel.count("repeated_targets_skipped", repeats_skipped);
    rel.count("target_guard_probes", guard_probes);
    rel.count("walk_jumps", walk_jumps + pool.len() as u64);
    rel.count("walk_pool_offsets", pool.len() as u64);
    rel.count("walk_pool_ops", pool_ops.adds + pool_ops.doubles);
    rel.count("walk_pool_restarts", pool_taken);
    rel.count("walk_steps", walk_steps);
    rel.count("walk_restarts", walk_restarts);
    la.wall_ns = la_ns;
    let work = matrix.row_ops();
    la.count("row_ops", work);
    la.count("rows", found);
    la.count("single_column_rows", single_column_rows);
    la.count("two_column_rows", two_column_rows);
    la.count("repeated_column_rows", repeated_column_rows);
    la.count("pinned_by_repeated_row", u64::from(pinned_by_repeated_row));
    la.count("columns", (fb.columns + 1) as u64);
    la.count("rank", matrix.solved_rank() as u64);
    for (name, value) in oracle.relation_counters() {
        rel.count(&name, value);
    }

    // The report: run_pipeline's pricing.
    let mut fb_phase = fb_cost;
    for phase in [&mut fb_phase, &mut prep_cost, &mut rel, &mut la, &mut ver] {
        price_phase(phase, calib);
    }
    let gae_before_solver = rel.gae;
    let solver_report = oracle.solver_totals().map(|t| {
        let wall_per_op = (t.ops > 0).then(|| t.wall_ns as f64 / t.ops as f64);
        let (gae, priced_by, ns_per_op) = match t.extra.get("fp_muls_per_add") {
            Some(&v) if v > 0 && t.op_unit == "F_q multiplications" => {
                (t.ops as f64 / v as f64, "counted", None)
            }
            _ => (0.0, "unpriced", wall_per_op),
        };
        rel.count("solver_calls", t.calls);
        rel.count("solver_ops", t.ops);
        rel.count("solver_budget_exceeded", t.budget_exceeded);
        rel.gae += gae;
        SolverReport {
            name: t.solver.clone(),
            calls: t.calls,
            ops: t.ops,
            op_unit: t.op_unit.clone(),
            ns_per_op,
            wall_ns: t.wall_ns,
            peak_bytes: t.peak_bytes,
            budget_exceeded: t.budget_exceeded,
            gae,
            priced_by: priced_by.into(),
            solving_degree_mean: t.solving_degree_mean(),
            solving_degree_max: t.solving_degree_max,
            semi_regular_degree: t.semi_regular_degree(),
            degree_over_bound: t.degree_over_bound(),
            extra: t.extra.clone(),
        }
    });
    let total_gae = fb_phase.gae + prep_cost.gae + rel.gae + la.gae + ver.gae;
    let rf = r.to_f64();
    let s = total_gae / rf.sqrt();
    let report = RunReport {
        instance: ctx.name.clone(),
        log2_r: rf.log2(),
        r: u64::try_from(r128).unwrap_or(u64::MAX),
        group_order: u64::try_from(inst.group_order).unwrap_or(u64::MAX),
        spec: spec.clone(),
        label: spec.label(),
        factor_base: FactorBaseReport {
            name: base.name().to_string(),
            description: base.describe(&spec.factor_base_params),
            signed_points: fb.points.len(),
            abscissae: fb.abscissae,
            columns: fb.columns,
            points_per_column: fb.points.len() as f64 / fb.columns.max(1) as f64,
            cost: fb_phase,
        },
        decomposition: DecompositionReport {
            name: oracle.name().to_string(),
            summands: oracle.summands(),
            description: oracle.describe(&spec.oracle_params),
            targets_tried: trials,
            relations_found: found,
            hit_rate: found as f64 / trials.max(1) as f64,
            setup: prep_cost,
            cost: rel,
            gae_before_solver,
            system: oracle.last_system(),
            solver: solver_report,
        },
        linear_algebra: LinearAlgebraReport {
            name: "wiedemann".into(),
            rows: found,
            rank: matrix.solved_rank() as u64,
            dependent: 0,
            work,
            work_unit: "row_ops".into(),
            cost: la,
        },
        verify: ver,
        recovered: recovered.and_then(|d| u64::try_from(d.to_u128()).ok()),
        verified,
        total_gae,
        s,
        rho_s: None,
        s_over_rho: None,
        wall_seconds: started.elapsed().as_secs_f64(),
        exhausted: recovered.is_none(),
    };
    Ok(WideRun { report, recovered })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::gaudry_quartic_bin::{find_bin_curve, BinOptions};
    use crate::cryptanalysis::ic_boundary::{rho_walk_with, NegationClasses, RhoWalk};
    use crate::cryptanalysis::ic_framework::run_pipeline;
    use crate::cryptanalysis::ic_framework::stages::Params;

    fn target_of(inst: &BinCurve, d: u64) -> PtB {
        inst.curve.mul(&inst.generator, u128::from(d))
    }

    /// At `S = u64` the port is `rho_walk_with(NegationClasses)` step for step.
    #[test]
    fn rho_negation_is_the_one_word_walk() {
        for (m, seed) in [(6u32, 1u64), (7, 2), (8, 1)] {
            let inst = find_bin_curve(m, seed, 16).unwrap();
            let r = inst.r as u64;
            let c = &inst.curve;
            for s in 0..3u64 {
                let d = 1 + (0x1234_5678u64.wrapping_mul(s + 7)) % (r - 1);
                let q = target_of(&inst, d);
                let a = rho_walk_with(
                    c,
                    &NegationClasses { r },
                    inst.generator,
                    q,
                    r,
                    s,
                    1 << 40,
                    RhoWalk::negation(),
                );
                let (b, d128) = rho_negation::<CurveB, u64>(c, inst.generator, q, r, s, 1 << 40);
                assert_eq!(a.recovered, Some(d));
                assert_eq!(d128, Some(d));
                assert_eq!(a.group_ops, b.group_ops, "m {m} seed {s}");
                assert_eq!(a.steps, b.steps);
                assert_eq!(a.walks, b.walks);
                assert_eq!(a.counters, b.counters);
                // The u128 instantiation walks the same values too.
                let (w, dw) =
                    rho_negation::<CurveB, u128>(c, inst.generator, q, u128::from(r), s, 1 << 40);
                assert_eq!(dw, Some(u128::from(d)));
                assert_eq!(
                    w.counters.get("walk_operations"),
                    b.counters.get("walk_operations")
                );
            }
        }
    }

    fn batched(inst: &BinCurve, d: u64, seed: u64, walks: usize) -> (BatchedRho, Option<u64>) {
        let q = target_of(inst, d);
        let par = BatchedRhoParams { walks, dp_bits: 0 };
        rho_negation_batched::<u64>(
            &inst.curve,
            inst.generator,
            q,
            inst.r as u64,
            seed,
            1 << 40,
            par,
        )
    }

    /// The batched walk recovers the logarithm on small `f2m4` curves,
    /// m = 6..=10, at both scalar widths, and checks out against the group.
    #[test]
    fn rho_negation_batched_recovers_the_logarithm() {
        for m in 6u32..=10 {
            let inst = find_bin_curve(m, 1, 16).unwrap();
            let r = inst.r as u64;
            for s in 0..3u64 {
                let d = 1 + (0x9e37_79b9u64.wrapping_mul(s + 3 + m as u64)) % (r - 1);
                let (res, got) = batched(&inst, d, s, 32);
                assert_eq!(got, Some(d), "m {m} seed {s}");
                assert_eq!(
                    res.fq_total,
                    res.fq_setup
                        + res.fq_batch_inversion
                        + res.fq_batched_formula
                        + res.fq_single_ops
                        + res.fq_verification
                );
                let q = target_of(&inst, d);
                let par = BatchedRhoParams {
                    walks: 32,
                    dp_bits: 0,
                };
                let (res2, got2) = rho_negation_batched::<u128>(
                    &inst.curve,
                    inst.generator,
                    q,
                    u128::from(r),
                    s,
                    1 << 40,
                    par,
                );
                assert_eq!(got2, Some(u128::from(d)));
                assert_eq!(res.fq_total, res2.fq_total, "u128 walks the same values");
                assert_eq!(res.steps, res2.steps);
            }
        }
    }

    /// Same seed, same counts; a different seed, different counts.
    #[test]
    fn rho_negation_batched_is_deterministic() {
        let inst = find_bin_curve(9, 2, 16).unwrap();
        let d = 1234 % (inst.r as u64);
        let (a, da) = batched(&inst, d, 5, 64);
        let (b, db) = batched(&inst, d, 5, 64);
        assert_eq!(da, db);
        assert_eq!(format!("{a:?}"), format!("{b:?}"));
        let (c2, _) = batched(&inst, d, 6, 64);
        assert_ne!(a.fq_total, c2.fq_total);
    }

    /// F_q multiplications per walk step: the batched walk is cheaper than
    /// the unbatched one by about the inversion it amortises, never more
    /// than the one extra batch of `3(n - 1)` F_{q^4} products per round.
    #[test]
    fn rho_negation_batched_cost_per_step_is_in_band() {
        let inst = find_bin_curve(10, 1, 16).unwrap();
        let c = &inst.curve;
        let r = inst.r as u64;
        let per_add = crate::cryptanalysis::gaudry_quartic_bin::fq_muls_per_add(c) as f64;
        let (mut ub, mut ub_steps, mut b, mut b_steps) = (0f64, 0f64, 0f64, 0f64);
        for s in 0..4u64 {
            let d = 1 + (0x1234_5678u64.wrapping_mul(s + 7)) % (r - 1);
            let q = target_of(&inst, d);
            let before = c.f.f.muls();
            let (u, _) = rho_negation::<CurveB, u64>(c, inst.generator, q, r, s, 1 << 40);
            ub += (c.f.f.muls() - before) as f64;
            ub_steps += (u.counters["walk_operations"]) as f64;
            let (res, got) = batched(&inst, d, s, 64);
            assert_eq!(got, Some(d));
            b += (res.fq_batch_inversion + res.fq_batched_formula + res.fq_single_ops) as f64;
            b_steps += (res.steps + res.look_ahead_adds + res.escapes) as f64;
        }
        let (u_per, b_per) = (ub / ub_steps, b / b_steps);
        eprintln!(
            "per_add {per_add} unbatched/step {u_per:.2} batched/step {b_per:.2} ratio {:.3}",
            b_per / u_per
        );
        // The unbatched walk pays about one affine addition per step.
        assert!(
            u_per > 0.9 * per_add && u_per < 1.1 * per_add,
            "{u_per} vs {per_add}"
        );
        let ratio = b_per / u_per;
        assert!((0.70..0.98).contains(&ratio), "ratio {ratio}");
    }

    fn spec(oracle: &str) -> PipelineSpec {
        let (fb, fbp) = Params::parse_spec("bin-t-line").unwrap();
        let (or, orp) = Params::parse_spec(oracle).unwrap();
        PipelineSpec {
            factor_base: fb,
            factor_base_params: fbp,
            oracle: or,
            oracle_params: orp,
            solver: None,
            solver_params: Params::default(),
            targets: Targets::Walk,
            linalg: "wiedemann".into(),
            max_trials: 2_000_000,
            seed: 7,
            solver_budget_seconds: 0,
        }
    }

    /// At `S = u64` the wide pipeline reproduces `run_pipeline` with
    /// `wiedemann`: the same counts, the same answer.
    #[test]
    fn run_f2m4_is_the_one_word_pipeline() {
        for (oracle, lp, permille) in [
            (
                "gaudry-d4-bin:macaulay=11,frobenius=1,sparsest=1,replay=1",
                0u8,
                1000u64,
            ),
            (
                "gaudry-d4-bin:macaulay=11,frobenius=1,sparsest=1,replay=1",
                2,
                500,
            ),
        ] {
            let inst = find_bin_curve(6, 1, 16).unwrap();
            let c = &inst.curve;
            let d = 12_345 % (inst.r as u64);
            let q = target_of(&inst, d);
            let ctx = InstanceCtx {
                group: c,
                generator: inst.generator,
                target: q,
                r: inst.r as u64,
                cofactor: inst.cofactor as u64,
                group_order: inst.group_order as u64,
                name: "test".into(),
                field_degree: None,
            };
            let calib = Calibration {
                ns_per_add: 1.0,
                ..Default::default()
            };
            let opts = BinOptions {
                weighted: true,
                frobenius: true,
                sparsest: true,
                replay: true,
            };
            let sp = spec(oracle);
            let mut o1 = BinD4Oracle::new(&inst, 11, sp.seed, opts);
            o1.lp.max_large = lp;
            o1.lp.small_permille = permille;
            let base = TLineBase { curve: &inst };
            let one = run_pipeline(&ctx, &sp, &base, &mut o1, d, &calib, None).unwrap();
            let mut o2 = BinD4Oracle::new(&inst, 11, sp.seed, opts);
            o2.lp.max_large = lp;
            o2.lp.small_permille = permille;
            let wide = run_f2m4::<u64>(
                &ctx,
                &inst,
                &sp,
                &mut o2,
                inst.r as u64,
                inst.cofactor as u64,
                &calib,
            )
            .unwrap();
            assert!(one.verified, "the one-word run solves");
            assert_eq!(wide.recovered, Some(d));
            assert_eq!(one.total_gae, wide.report.total_gae, "lp {lp}");
            assert_eq!(
                one.decomposition.targets_tried,
                wide.report.decomposition.targets_tried
            );
            assert_eq!(one.linear_algebra.work, wide.report.linear_algebra.work);
            assert_eq!(
                one.decomposition.cost.native,
                wide.report.decomposition.cost.native
            );
            assert_eq!(
                one.linear_algebra.cost.native,
                wide.report.linear_algebra.cost.native
            );
            // And at u128, the same counts.
            let mut o3 = BinD4Oracle::new(&inst, 11, sp.seed, opts);
            o3.lp.max_large = lp;
            o3.lp.small_permille = permille;
            let w128 =
                run_f2m4::<u128>(&ctx, &inst, &sp, &mut o3, inst.r, inst.cofactor, &calib).unwrap();
            assert_eq!(w128.recovered, Some(u128::from(d)));
            assert_eq!(w128.report.total_gae, one.total_gae);
        }
    }
}
