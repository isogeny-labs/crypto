//! # Index calculus vs Pollard rho on the same hyperelliptic Jacobian.
//!
//! [`crate::cryptanalysis::hyperelliptic_index_calculus`] implements the
//! Adleman–DeMarrais–Huang / Gaudry attack but claims nothing about its
//! cost.  This module supplies the missing half: the **reference**
//! `AGENTS.md` requires — Pollard rho run on the *same* group, the same
//! `D₁`, `D₂` and the same prime order `N`, with the same operation
//! accounting — and a head-to-head table in one unit.
//!
//! ## Unit
//!
//! ```text
//!   S = total group operations / sqrt(N)
//! ```
//!
//! which holds rho flat at every size: a collision arrives after about
//! `sqrt(π N / 2) ≈ 1.2533·sqrt(N)` distinct points, and Floyd spends
//! three group operations per iteration, so this reference sits near
//! `S ≈ 3` (a distinguished-point rho would sit near `1.25`).  Either
//! way it is a constant, which is what makes "is index calculus better
//! than rho here" one column to read.
//!
//! Every cost each side incurs is inside `S`:
//!
//! - **rho** — the `r` precomputed walk steps `R_j = a_j D₁ + b_j D₂`
//!   plus every tortoise/hare step.
//! - **index calculus** — the relation-stage group operations, *and* the
//!   smoothness oracle's field work, *and* the linear algebra, the last
//!   two converted into group-operation equivalents by a **measured**
//!   factor
//!   ([`calibrate_modmuls_per_group_op`]), not an assumed one.  Leaving
//!   the solve out is exactly the relabelling failure mode: the work
//!   does not disappear, it moves somewhere the headline does not look.
//!
//! ## Boundaries
//!
//! Two, both derived before measuring:
//!
//! - **Floor (index calculus).**  `m + 1` unknowns need `m + 1`
//!   independent relations.  Under heuristic **H1** — a reduced divisor
//!   drawn uniformly from `Jac(C)(F_p)` splits into degree-1 places with
//!   probability `≈ 1/g!` — the expected number of trials is at least
//!   `(m+1)·g!`, and no walk can produce a trial for less than one group
//!   operation.  So
//!
//!   ```text
//!     floor_ops = (m + 1)·(1 + g/c)  +  m²/c
//!   ```
//!
//!   one group operation per relation, plus the field work no
//!   implementation can skip: the smoothness oracle must read each
//!   candidate's `u`, whose `g` coefficients cost `g` multiplications,
//!   and any dense solve must read its own `m × m` matrix once.  Those
//!   mul-mods are divided by the measured mul-mods-per-group-op factor
//!   `c` so every term is in the same unit as the measurement.  It
//!   moves only with `m` and `g`, so it cannot be tuned away; it is a
//!   floor on *this* algorithm, not on the DLP.
//!
//!   The floor does **not** divide the relation count by a smoothness
//!   rate.  An earlier version took `(m+1)·g!` trials from heuristic H1
//!   (yield `≈ 1/g!` for uniformly drawn divisors) and the factor-base
//!   walk measured below it — which a real floor cannot do. The walk
//!   does not draw uniformly, so H1 is a **prediction** reported beside
//!   the floor (`ic_h1_predicted_s`), not a bound inside it.
//! - **Reference.**  Pollard rho, measured, on the same instances.
//!
//! The ratio `S_ic / S_rho` and the ratio `S_ic` to the floor are the
//! progress measures.  This module *measures*; the numbers it produces
//! at toy `p` say nothing about cryptographic sizes, where the relation
//! stage's `O(p)` root finding alone would dominate.

use std::collections::HashMap;
use std::time::Instant;

use num_bigint::BigUint;
use num_traits::{One, Zero};
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};

use crate::prime_hyperelliptic::{HyperellipticCurveP, MumfordDivisorP};
use crate::utils::mod_inverse;

use super::hyperelliptic_index_calculus::{
    build_factor_base, divisor_order_bsgs, hasse_interval, hec_index_calculus_dlp,
    largest_prime_factor, HecIndexCalculusParams,
};
use crate::prime_hyperelliptic::FpPoly;

// ── The thread's curves and instance rule ──────────────────────────────

/// The curve family every round of this thread measures on, as the
/// frozen examples `hyperelliptic_ic_vs_rho` and
/// `hyperelliptic_ic_large_prime` define it (they keep their own copies so
/// their frozen commands stay byte-for-byte as they were):
///
/// - genus 2: `y² = x⁵ + 3x³ + 2x² + x + c`,
/// - genus 3: `y² = x⁷ + x³ + c·x + 1`,
/// - genus 4: `y² = x⁹ + x³ + c·x + 1`.
///
/// Returns `None` for a `c` whose model is not a valid curve (`f` not
/// squarefree): the instance rule skips those, and `HyperellipticCurveP::
/// new` now refuses them by panicking, which the frozen examples predate.
pub fn thread_curve(p: u64, c: u64, genus: u32) -> Option<HyperellipticCurveP> {
    let p = BigUint::from(p);
    let one = BigUint::one;
    let zero = BigUint::zero;
    let cc = BigUint::from(c) % &p;
    let coeffs = match genus {
        2 => vec![
            cc,
            one(),
            BigUint::from(2u32),
            BigUint::from(3u32),
            zero(),
            one(),
        ],
        3 => vec![one(), cc, zero(), one(), zero(), zero(), zero(), one()],
        4 => vec![
            one(),
            cc,
            zero(),
            one(),
            zero(),
            zero(),
            zero(),
            zero(),
            zero(),
            one(),
        ],
        g => panic!("no thread curve for genus {g}"),
    };
    HyperellipticCurveP::try_new(p.clone(), FpPoly::from_coeffs(coeffs, p), genus).ok()
}

fn thread_curve_is_squarefree(curve: &HyperellipticCurveP) -> bool {
    let p = &curve.p;
    let d: Vec<BigUint> = curve
        .f
        .coeffs
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, c)| (c * BigUint::from(i as u64)) % p)
        .collect();
    let deriv = FpPoly::from_coeffs(d, p.clone());
    !deriv.is_zero() && curve.f.gcd(&deriv).degree() == Some(0)
}

/// The thread's instance rule, unchanged since round three: the first `c`
/// whose Jacobian has a prime subgroup above 1000 carrying at least a
/// quarter of the Hasse-Weil lower bound, its generator the first
/// factor-base place (of the first eight) whose order exposes it.
/// Returns `(curve, D₁, N, c)`.
pub fn thread_instance(
    p: u64,
    genus: u32,
) -> Option<(HyperellipticCurveP, MumfordDivisorP, BigUint, u64)> {
    for c in 1..p.min(60) {
        let Some(curve) = thread_curve(p, c, genus) else {
            continue;
        };
        if !thread_curve_is_squarefree(&curve) {
            continue;
        }
        let fb = build_factor_base(&curve, usize::MAX);
        for e in fb.entries.iter().take(8) {
            let order = match divisor_order_bsgs(&curve, &e.divisor, 200_000) {
                Some(o) => o,
                None => continue,
            };
            let l = largest_prime_factor(&order);
            let (lo, _hi) = hasse_interval(&curve)?;
            if l < BigUint::from(1000u32) || &l * BigUint::from(4u32) < BigUint::from(lo) {
                continue;
            }
            let d1 = e.divisor.scalar_mul(&(&order / &l), &curve);
            if divisor_order_bsgs(&curve, &d1, 200_000).as_ref() != Some(&l) {
                continue;
            }
            return Some((curve, d1, l, c));
        }
    }
    None
}

// ── Pollard rho on the Jacobian ────────────────────────────────────────

/// Number of branches in the additive walk.  Teske's analysis puts the
/// cycle length of an `r`-adding walk within a few percent of the
/// random-map ideal once `r ≥ 16`, so a smaller `r` would flatter the
/// index calculus by handicapping its reference.
const RHO_BRANCHES: usize = 16;

/// Bit length of the short rho branch coefficients `a_j, b_j`
/// ([`RhoVariant::DistinguishedPoints`] and the other short-step walks).
///
/// The index-calculus walk builds its steps from short coefficients
/// because precomputation is its largest term, and it looks for smooth
/// points, so nothing is lost.  Rho was given the same trick on the
/// argument that it cost nothing; round twelve measured otherwise.  With
/// short coefficients a walk's displacement `ΔA + ΔB·k` stays on a small
/// 2-D set, and at `N ≈ 1.7·10⁷` the walk needed 1.42× the steps of the
/// full-width walk ([`RhoVariant::DistinguishedPointsFullWidth`], now the
/// default).
const RHO_STEP_BITS: u32 = 8;

/// Which rho to run as the reference.
///
/// The distinction is worth 3× and nothing else: both walks take the
/// same expected number of *steps* to a collision, `≈ sqrt(π N / 2)`,
/// but Floyd runs three group operations per iteration (one tortoise,
/// two hare) while a distinguished-point walk runs one and detects the
/// collision through a table.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum RhoVariant {
    /// Floyd cycle finding.  Constant memory, 3 operations per
    /// iteration.
    Floyd,
    /// Distinguished points: store the walk positions whose hash ends
    /// in `theta_bits` zero bits, and stop when one repeats.  One
    /// operation per step, at the price of a table — this is the rho
    /// anyone attacking a real curve would run.  Its branch
    /// coefficients are `RHO_STEP_BITS` bits, which round twelve showed
    /// slows the collision at large `N`; kept for the frozen rounds, and
    /// no longer the default.
    DistinguishedPoints,
    /// Distinguished points on `±` classes: after every step the
    /// position is replaced by a canonical representative of `{D, −D}`,
    /// with `(a, b)` negated alongside, so the walk needs
    /// `sqrt(πN/4)` steps instead of `sqrt(πN/2)` — the `√2` of the
    /// negation map (Wiener–Zuccherato 1998; Duursma–Gaudry–Morain
    /// 1999).  Index calculus folds `±P` in its factor base, so a
    /// reference without this fold leaves out a cheap automorphism the
    /// method under study already uses.
    ///
    /// Folding makes fruitless two-cycles likely (about `1/(2r)` per
    /// step); a look-ahead takes the next branch when the folded
    /// position would select the same branch again.  Every look-ahead
    /// trial is a counted group operation, reported in
    /// [`RhoResult::fruitless_ops`].
    DistinguishedPointsNegation,
    /// [`Self::DistinguishedPoints`] with the walk hash passed through
    /// the SplitMix64 finaliser before it picks branches and
    /// distinguished points (round ten).  `divisor_hash` alone is a
    /// multiply-accumulate whose low bits depend only on the low bits of
    /// each coefficient.
    DistinguishedPointsMixed,
    /// [`Self::DistinguishedPointsNegation`] with the mixed hash.
    DistinguishedPointsNegationMixed,
    /// [`Self::DistinguishedPoints`] with full-width branch coefficients
    /// `a_j, b_j` uniform in `[0, N)` instead of `RHO_STEP_BITS`-bit ones
    /// (round twelve).  With short coefficients a walk's displacement
    /// `ΔA + ΔB·k` concentrates on a small 2-D set, which can delay its
    /// rho collision at large `N`.  The default reference since round
    /// twelve.
    #[default]
    DistinguishedPointsFullWidth,
    /// [`Self::DistinguishedPointsNegation`] on the full-width walk
    /// (round thirteen).
    DistinguishedPointsNegationFullWidth,
}

/// Outcome of one rho run.
#[derive(Clone, Debug)]
pub struct RhoResult {
    pub k: Option<BigUint>,
    /// Jacobian group operations: walk steps plus the precomputed
    /// branch divisors.
    pub group_ops: usize,
    /// Of those, the branch precomputation — a fixed cost that does not
    /// grow with `sqrt(N)` and so dominates `S` at toy sizes.  Split
    /// out because a reader comparing against the `sqrt(π/2)` ideal
    /// needs the walk alone, and hiding it inside one number would make
    /// rho look worse than it is.
    pub precompute_ops: usize,
    /// Of those, the tortoise/hare steps.
    pub walk_ops: usize,
    pub restarts: usize,
    /// Of `walk_ops`, the work spent on fruitless cycles: look-ahead
    /// trials that were discarded and distinguished-point gap escapes.
    /// Zero for walks that do not fold `±`.
    pub fruitless_ops: usize,
    pub wall_ms: f64,
}

/// Branch selector: a cheap hash of the Mumford representation.
///
/// Any function of the *class* works, as long as equal classes select
/// equal branches — that is what makes the walk deterministic and the
/// collision meaningful.
fn branch_of(d: &MumfordDivisorP) -> usize {
    let mut acc: u64 = 0;
    for poly in [&d.u, &d.v] {
        for limb in poly.coeffs.iter().flat_map(|c| c.to_u64_digits()) {
            acc = acc.wrapping_mul(0x9e37_79b9_7f4a_7c15).wrapping_add(limb);
        }
        acc = acc.wrapping_mul(0x9e37_79b9_7f4a_7c15).wrapping_add(1);
    }
    (acc >> 32) as usize % RHO_BRANCHES
}

fn rand_below(rng: &mut StdRng, n: &BigUint) -> BigUint {
    let bytes = (n.bits() as usize).div_ceil(8) + 8;
    let mut buf = vec![0u8; bytes];
    rng.fill_bytes(&mut buf);
    BigUint::from_bytes_be(&buf) % n
}

/// **Solve `D₂ = k·D₁`** by Pollard rho with an `r`-adding walk and
/// Floyd cycle finding, over the order-`N` subgroup generated by `D₁`.
///
/// `n` must be prime (the final division inverts mod `n`).  A collision
/// with `b₁ ≡ b₂` carries no information, so the walk restarts from a
/// fresh seed; `max_steps` bounds the total.  The returned `k` is
/// verified before it is returned.
pub fn pollard_rho_jacobian(
    curve: &HyperellipticCurveP,
    d1: &MumfordDivisorP,
    d2: &MumfordDivisorP,
    n: &BigUint,
    seed: u64,
    max_steps: usize,
) -> RhoResult {
    pollard_rho_jacobian_with(curve, d1, d2, n, seed, max_steps, &RhoVariant::Floyd)
}

/// Build the `r`-adding walk's branches, charging their construction.
///
/// `full_width` draws `a_j, b_j` uniform in `[0, N)` and charges
/// `2·(bits(N) + 1)` operations per branch, as for a walk's start point;
/// otherwise they are `RHO_STEP_BITS`-bit and nonzero.
fn build_branches(
    curve: &HyperellipticCurveP,
    d1: &MumfordDivisorP,
    d2: &MumfordDivisorP,
    n: &BigUint,
    rng: &mut StdRng,
    ops: &mut usize,
    full_width: bool,
) -> Vec<(BigUint, BigUint, MumfordDivisorP)> {
    let step_bound = BigUint::one() << RHO_STEP_BITS;
    let mut branch = Vec::with_capacity(RHO_BRANCHES);
    for _ in 0..RHO_BRANCHES {
        let (a, b) = if full_width {
            (rand_below(rng, n), rand_below(rng, n))
        } else {
            (
                rand_below(rng, &step_bound) + BigUint::one(),
                rand_below(rng, &step_bound) + BigUint::one(),
            )
        };
        let r = d1
            .scalar_mul(&a, curve)
            .add(&d2.scalar_mul(&b, curve), curve);
        *ops += if full_width {
            2 * (n.bits() as usize + 1)
        } else {
            2 * RHO_STEP_BITS as usize
        };
        branch.push((a, b, r));
    }
    branch
}

/// Full hash of a divisor, used both for branch selection and for the
/// distinguished-point predicate.
fn divisor_hash(d: &MumfordDivisorP) -> u64 {
    let mut acc: u64 = 0;
    for poly in [&d.u, &d.v] {
        for limb in poly.coeffs.iter().flat_map(|c| c.to_u64_digits()) {
            acc = acc.wrapping_mul(0x9e37_79b9_7f4a_7c15).wrapping_add(limb);
        }
        acc = acc.wrapping_mul(0x9e37_79b9_7f4a_7c15).wrapping_add(1);
    }
    acc
}

/// SplitMix64's output finaliser: a bijection on `u64` whose every output
/// bit depends on every input bit.
fn splitmix64_finalise(mut z: u64) -> u64 {
    z ^= z >> 30;
    z = z.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z ^= z >> 27;
    z = z.wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// Recover `k` from a collision `a₁ + b₁k ≡ a₂ + b₂k`, or `None` when
/// the collision is degenerate (`b₁ ≡ b₂`, which carries no
/// information).
fn solve_collision(
    a1: &BigUint,
    b1: &BigUint,
    a2: &BigUint,
    b2: &BigUint,
    n: &BigUint,
) -> Option<BigUint> {
    let db = (b2 + n - b1) % n;
    if db.is_zero() {
        return None;
    }
    let da = (a1 + n - a2) % n;
    let inv = mod_inverse(&db, n)?;
    Some((da * inv) % n)
}

/// **Solve `D₂ = k·D₁`** by Pollard rho, choosing the variant.
pub fn pollard_rho_jacobian_with(
    curve: &HyperellipticCurveP,
    d1: &MumfordDivisorP,
    d2: &MumfordDivisorP,
    n: &BigUint,
    seed: u64,
    max_steps: usize,
    variant: &RhoVariant,
) -> RhoResult {
    match variant {
        RhoVariant::Floyd => rho_floyd(curve, d1, d2, n, seed, max_steps),
        RhoVariant::DistinguishedPoints => {
            rho_dp(curve, d1, d2, n, seed, max_steps, false, false, false)
        }
        RhoVariant::DistinguishedPointsNegation => {
            rho_dp(curve, d1, d2, n, seed, max_steps, true, false, false)
        }
        RhoVariant::DistinguishedPointsMixed => {
            rho_dp(curve, d1, d2, n, seed, max_steps, false, true, false)
        }
        RhoVariant::DistinguishedPointsNegationMixed => {
            rho_dp(curve, d1, d2, n, seed, max_steps, true, true, false)
        }
        RhoVariant::DistinguishedPointsFullWidth => {
            rho_dp(curve, d1, d2, n, seed, max_steps, false, false, true)
        }
        RhoVariant::DistinguishedPointsNegationFullWidth => {
            rho_dp(curve, d1, d2, n, seed, max_steps, true, false, true)
        }
    }
}

/// Distinguished-point rho: one group operation per step.
///
/// `theta_bits` is chosen so that about 32 distinguished points are
/// expected before the collision — few enough that the table is
/// negligible, many enough that the walk is not dominated by the
/// distance to the first one.
/// Canonical representative of `{D, −D}`: the one whose `v` has its
/// leading coefficient in `[0, (p−1)/2]`.  `−(u, v) = (u, −v)` because
/// `deg v < deg u`, so the two `v`s are coefficientwise negatives and
/// exactly one of them qualifies unless `v = 0`, where `D = −D`.
/// Returns the representative and whether it is the negation.
fn fold_negation(d: MumfordDivisorP, curve: &HyperellipticCurveP) -> (MumfordDivisorP, bool) {
    let lead = match d.v.coeffs.iter().rposition(|c| !c.is_zero()) {
        Some(i) => d.v.coeffs[i].clone(),
        None => return (d, false),
    };
    let half = (&curve.p - BigUint::one()) >> 1;
    if lead <= half {
        (d, false)
    } else {
        (d.neg(curve), true)
    }
}

#[allow(clippy::too_many_arguments)]
fn rho_dp(
    curve: &HyperellipticCurveP,
    d1: &MumfordDivisorP,
    d2: &MumfordDivisorP,
    n: &BigUint,
    seed: u64,
    max_steps: usize,
    negation: bool,
    mix: bool,
    full_width: bool,
) -> RhoResult {
    let started = Instant::now();
    // The walk's hash: `divisor_hash`, optionally through SplitMix64's
    // finaliser (round ten).  Used for branch choice, the
    // distinguished-point test and the look-ahead alike.
    let walk_hash = |d: &MumfordDivisorP| {
        let h = divisor_hash(d);
        if mix {
            splitmix64_finalise(h)
        } else {
            h
        }
    };
    let mut fruitless_ops = 0usize;
    // Fold a position and its coefficients together, so that the walk
    // position is always exactly `a·D₁ + b·D₂`.
    let fold = |x: MumfordDivisorP, a: BigUint, b: BigUint| {
        if !negation {
            return (x, a, b, false);
        }
        let (x, flipped) = fold_negation(x, curve);
        if flipped {
            (x, (n - a) % n, (n - b) % n, true)
        } else {
            (x, a, b, false)
        }
    };
    let mut group_ops = 0usize;
    let mut precompute_ops = 0usize;
    let mut restarts = 0usize;
    let mut steps_left = max_steps;
    let mut rng = StdRng::seed_from_u64(seed);

    let mut branch_ops = 0usize;
    let branch = build_branches(curve, d1, d2, n, &mut rng, &mut branch_ops, full_width);
    group_ops += branch_ops;
    precompute_ops += branch_ops;

    // Expected steps to a collision ≈ 1.25·sqrt(N); aim for ~32
    // distinguished points along the way.
    let root_n = sqrt_big(n).max(2.0);
    let theta_bits = ((root_n / 32.0).log2().round() as i64).clamp(0, 24) as u32;
    let mask: u64 = if theta_bits == 0 {
        0
    } else {
        (1u64 << theta_bits) - 1
    };

    // divisor hash → (a, b) that reached it.
    let mut seen: HashMap<u64, (BigUint, BigUint)> = HashMap::new();

    // A deterministic walk can enter a cycle that contains **no**
    // distinguished point, and then runs forever detecting nothing.
    // Bound each attempt at a generous multiple of the expected
    // distance to a collision and restart instead.  Without this the
    // walk silently burns the entire step budget — not a slow run but a
    // hung one, and from outside the two look identical.
    let expected = 1.25 * root_n + (1u64 << theta_bits) as f64;
    let attempt_cap = (64.0 * expected).min(1e9) as usize + 64;

    for _attempt in 0..64u64 {
        let a0 = rand_below(&mut rng, n);
        let b0 = rand_below(&mut rng, n);
        let mut x = d1
            .scalar_mul(&a0, curve)
            .add(&d2.scalar_mul(&b0, curve), curve);
        group_ops += 2 * (n.bits() as usize + 1);
        precompute_ops += 2 * (n.bits() as usize + 1);
        let (x0, a0, b0, _) = fold(x, a0, b0);
        x = x0;
        let (mut a, mut b) = (a0, b0);
        let mut this_attempt = 0usize;
        let mut since_dp = 0usize;
        // A cycle can contain no distinguished point at all, in which
        // case the walk detects nothing however long it runs.  Expected
        // distance to the next one is `2^theta_bits`; well past that,
        // jump rather than keep going.  This is what the outer cap was
        // silently absorbing, at 64x the cost.
        let dp_gap_limit = 32usize << theta_bits;

        while steps_left > 0 {
            if since_dp >= dp_gap_limit {
                restarts += 1;
                since_dp = 0;
                let j = (rng.next_u64() as usize) % branch.len();
                let (aj, bj, rj) = &branch[j];
                let (nx, na, nb, _) = fold(x.add(rj, curve), (&a + aj) % n, (&b + bj) % n);
                (x, a, b) = (nx, na, nb);
                group_ops += 1;
                if negation {
                    fruitless_ops += 1;
                }
                steps_left -= 1;
                this_attempt += 1;
                continue;
            }
            if this_attempt >= attempt_cap {
                // A cycle with no distinguished point in it, or simply
                // an unlucky start: begin somewhere else.
                restarts += 1;
                break;
            }
            this_attempt += 1;
            let h = walk_hash(&x);
            since_dp += 1;
            if (h & mask) == 0 {
                since_dp = 0;
                match seen.get(&h) {
                    Some((pa, pb)) => {
                        if let Some(k) = solve_collision(&a, &b, pa, pb, n) {
                            if &d1.scalar_mul(&k, curve) == d2 {
                                return RhoResult {
                                    k: Some(k),
                                    group_ops,
                                    precompute_ops,
                                    walk_ops: group_ops - precompute_ops,
                                    restarts,
                                    fruitless_ops,
                                    wall_ms: started.elapsed().as_secs_f64() * 1e3,
                                };
                            }
                        }
                        // Degenerate collision: the walk re-entered a
                        // trail with the same coefficients, so it would
                        // loop forever.  Jump by a randomly chosen step
                        // — one group operation — instead of building a
                        // fresh starting point for `2⌈log₂ N⌉`.  At
                        // small `N` these collisions are common enough
                        // that the difference showed up in `S_walk` as
                        // a 3× inflation of the reference, which is the
                        // wrong direction for a reference to be wrong
                        // in.
                        restarts += 1;
                        let j = (rng.next_u64() as usize) % branch.len();
                        let (aj, bj, rj) = &branch[j];
                        let (nx, na, nb, _) = fold(x.add(rj, curve), (&a + aj) % n, (&b + bj) % n);
                        (x, a, b) = (nx, na, nb);
                        group_ops += 1;
                        steps_left -= 1;
                        this_attempt += 1;
                        continue;
                    }
                    None => {
                        seen.insert(h, (a.clone(), b.clone()));
                    }
                }
            }
            let mut j = (h >> 32) as usize % branch.len();
            // Look-ahead (folded walk only): a step that lands on the
            // negation of `x + S_j` and then selects `S_j` again returns
            // to `x` — a fruitless two-cycle.  Take the next branch
            // instead.  Each discarded trial is a group operation spent,
            // and counted.
            let mut tries = 0usize;
            let (nx, na, nb) = loop {
                let (aj, bj, rj) = &branch[j];
                let (cx, ca, cb, flipped) = fold(x.add(rj, curve), (&a + aj) % n, (&b + bj) % n);
                group_ops += 1;
                tries += 1;
                let returns = flipped && (walk_hash(&cx) >> 32) as usize % branch.len() == j;
                if !returns || tries >= branch.len() {
                    break (cx, ca, cb);
                }
                fruitless_ops += 1;
                j = (j + 1) % branch.len();
            };
            (x, a, b) = (nx, na, nb);
            steps_left = steps_left.saturating_sub(tries);
        }
        if steps_left == 0 {
            break;
        }
    }

    RhoResult {
        k: None,
        group_ops,
        precompute_ops,
        walk_ops: group_ops - precompute_ops,
        restarts,
        fruitless_ops,
        wall_ms: started.elapsed().as_secs_f64() * 1e3,
    }
}

/// Round eleven's control: the plain distinguished-point walk run on the
/// logarithms themselves, `Z/N`, instead of on the Jacobian.
///
/// The position is `x = a + b·k mod N`; the branches add
/// `s_j = a_j + b_j·k` with `(a_j, b_j)` drawn from the same RNG stream, in
/// the same order, as [`rho_dp`] draws them; the starts are drawn the same
/// way; `θ`, the gap limit, the attempt cap and the collision handling are
/// the same rules.  Only the hash differs: SplitMix64 of `x`, which is an
/// ideal hash of the group element.  If this walk needs `≈ 1.25·√N` and
/// the Jacobian walk needs much more, the excess is on the Jacobian side.
///
/// `n` must be below `2⁶³`.  `k` is used only to place the walk, exactly
/// as `D₂ = k·D₁` places the Jacobian walk; the returned `k` is
/// recovered from a collision and compared against nothing here.
pub fn rho_dp_integer_model(n: &BigUint, k: &BigUint, seed: u64, max_steps: usize) -> RhoResult {
    rho_dp_integer_model_width(n, k, seed, max_steps, false)
}

/// [`rho_dp_integer_model`] with the branch width chosen: `full_width`
/// mirrors [`RhoVariant::DistinguishedPointsFullWidth`]'s draws and charge
/// (round twelve).
pub fn rho_dp_integer_model_width(
    n: &BigUint,
    k: &BigUint,
    seed: u64,
    max_steps: usize,
    full_width: bool,
) -> RhoResult {
    let started = Instant::now();
    let nn: u64 = n.to_u64_digits().first().copied().unwrap_or(0);
    assert!(n.bits() < 63, "integer model needs N < 2^63");
    let kk: u64 = (k % n).to_u64_digits().first().copied().unwrap_or(0);
    let mulmod = |a: u64, b: u64| ((a as u128 * b as u128) % nn as u128) as u64;
    let to64 = |v: &BigUint| v.to_u64_digits().first().copied().unwrap_or(0);
    let mut rng = StdRng::seed_from_u64(seed);
    let (mut group_ops, mut precompute_ops, mut restarts) = (0usize, 0usize, 0usize);
    let mut steps_left = max_steps;

    // Branches: the same draws as `build_branches`, same order.
    let step_bound = BigUint::one() << RHO_STEP_BITS;
    let mut branch: Vec<(u64, u64, u64)> = Vec::with_capacity(RHO_BRANCHES);
    let branch_cost = if full_width {
        2 * (n.bits() as usize + 1)
    } else {
        2 * RHO_STEP_BITS as usize
    };
    for _ in 0..RHO_BRANCHES {
        let (a, b) = if full_width {
            (
                to64(&rand_below(&mut rng, n)),
                to64(&rand_below(&mut rng, n)),
            )
        } else {
            (
                to64(&(rand_below(&mut rng, &step_bound) + BigUint::one())) % nn,
                to64(&(rand_below(&mut rng, &step_bound) + BigUint::one())) % nn,
            )
        };
        branch.push((a, b, (a + mulmod(b, kk)) % nn));
        group_ops += branch_cost;
        precompute_ops += branch_cost;
    }

    let root_n = sqrt_big(n).max(2.0);
    let theta_bits = ((root_n / 32.0).log2().round() as i64).clamp(0, 24) as u32;
    let mask: u64 = if theta_bits == 0 {
        0
    } else {
        (1u64 << theta_bits) - 1
    };
    let mut seen: HashMap<u64, (u64, u64)> = HashMap::new();
    let expected = 1.25 * root_n + (1u64 << theta_bits) as f64;
    let attempt_cap = (64.0 * expected).min(1e9) as usize + 64;
    let add = |x: (u64, u64, u64), s: (u64, u64, u64)| {
        ((x.0 + s.0) % nn, (x.1 + s.1) % nn, (x.2 + s.2) % nn)
    };

    for _attempt in 0..64u64 {
        let a0 = to64(&rand_below(&mut rng, n));
        let b0 = to64(&rand_below(&mut rng, n));
        let mut pos = (a0, b0, (a0 + mulmod(b0, kk)) % nn); // (a, b, x)
        group_ops += 2 * (n.bits() as usize + 1);
        precompute_ops += 2 * (n.bits() as usize + 1);
        let (mut this_attempt, mut since_dp) = (0usize, 0usize);
        let dp_gap_limit = 32usize << theta_bits;
        while steps_left > 0 {
            if since_dp >= dp_gap_limit {
                restarts += 1;
                since_dp = 0;
                let j = (rng.next_u64() as usize) % branch.len();
                pos = add(pos, branch[j]);
                group_ops += 1;
                steps_left -= 1;
                this_attempt += 1;
                continue;
            }
            if this_attempt >= attempt_cap {
                restarts += 1;
                break;
            }
            this_attempt += 1;
            let h = splitmix64_finalise(pos.2);
            since_dp += 1;
            if (h & mask) == 0 {
                since_dp = 0;
                match seen.get(&h) {
                    Some(&(pa, pb)) => {
                        let got = solve_collision(
                            &BigUint::from(pos.0),
                            &BigUint::from(pos.1),
                            &BigUint::from(pa),
                            &BigUint::from(pb),
                            n,
                        );
                        if let Some(kk_found) = got {
                            return RhoResult {
                                k: Some(kk_found),
                                group_ops,
                                precompute_ops,
                                walk_ops: group_ops - precompute_ops,
                                restarts,
                                fruitless_ops: 0,
                                wall_ms: started.elapsed().as_secs_f64() * 1e3,
                            };
                        }
                        restarts += 1;
                        let j = (rng.next_u64() as usize) % branch.len();
                        pos = add(pos, branch[j]);
                        group_ops += 1;
                        steps_left -= 1;
                        this_attempt += 1;
                        continue;
                    }
                    None => {
                        seen.insert(h, (pos.0, pos.1));
                    }
                }
            }
            let j = (h >> 32) as usize % branch.len();
            pos = add(pos, branch[j]);
            group_ops += 1;
            steps_left -= 1;
        }
        if steps_left == 0 {
            break;
        }
    }
    RhoResult {
        k: None,
        group_ops,
        precompute_ops,
        walk_ops: group_ops - precompute_ops,
        restarts,
        fruitless_ops: 0,
        wall_ms: started.elapsed().as_secs_f64() * 1e3,
    }
}

fn rho_floyd(
    curve: &HyperellipticCurveP,
    d1: &MumfordDivisorP,
    d2: &MumfordDivisorP,
    n: &BigUint,
    seed: u64,
    max_steps: usize,
) -> RhoResult {
    let started = Instant::now();
    let mut group_ops = 0usize;
    let mut precompute_ops = 0usize;
    let mut restarts = 0usize;
    let mut steps_left = max_steps;

    for attempt in 0..64u64 {
        let mut rng = StdRng::seed_from_u64(seed ^ (attempt << 32));

        // Precomputed branches R_j = a_j·D₁ + b_j·D₂.  Charged.
        let mut branch: Vec<(BigUint, BigUint, MumfordDivisorP)> = Vec::with_capacity(RHO_BRANCHES);
        let step_bound = BigUint::one() << RHO_STEP_BITS;
        for _ in 0..RHO_BRANCHES {
            let a = rand_below(&mut rng, &step_bound) + BigUint::one();
            let b = rand_below(&mut rng, &step_bound) + BigUint::one();
            let r = d1
                .scalar_mul(&a, curve)
                .add(&d2.scalar_mul(&b, curve), curve);
            group_ops += 2 * RHO_STEP_BITS as usize;
            precompute_ops += 2 * RHO_STEP_BITS as usize;
            branch.push((a, b, r));
        }

        let step = |a: &BigUint, b: &BigUint, x: &MumfordDivisorP| {
            let (aj, bj, rj) = &branch[branch_of(x)];
            ((a + aj) % n, (b + bj) % n, x.add(rj, curve))
        };

        // Start away from the identity so the first branch is not
        // degenerate.
        let a0 = rand_below(&mut rng, n);
        let b0 = rand_below(&mut rng, n);
        let x0 = d1
            .scalar_mul(&a0, curve)
            .add(&d2.scalar_mul(&b0, curve), curve);
        group_ops += 2 * (n.bits() as usize + 1);
        precompute_ops += 2 * (n.bits() as usize + 1);

        let (mut at, mut bt, mut xt) = (a0.clone(), b0.clone(), x0.clone());
        let (mut ah, mut bh, mut xh) = (a0, b0, x0);

        while steps_left > 0 {
            let (na, nb, nx) = step(&at, &bt, &xt);
            at = na;
            bt = nb;
            xt = nx;
            group_ops += 1;

            let (na, nb, nx) = step(&ah, &bh, &xh);
            let (na, nb, nx) = step(&na, &nb, &nx);
            ah = na;
            bh = nb;
            xh = nx;
            group_ops += 2;
            steps_left = steps_left.saturating_sub(3);

            if xt == xh {
                // a_t + b_t k ≡ a_h + b_h k  ⟹  k ≡ (a_t − a_h)/(b_h − b_t).
                let db = (&bh + n - &bt) % n;
                if db.is_zero() {
                    restarts += 1;
                    break; // useless collision; new walk
                }
                let da = (&at + n - &ah) % n;
                let k = match mod_inverse(&db, n) {
                    Some(inv) => (da * inv) % n,
                    None => {
                        restarts += 1;
                        break;
                    }
                };
                if &d1.scalar_mul(&k, curve) == d2 {
                    return RhoResult {
                        k: Some(k),
                        group_ops,
                        precompute_ops,
                        walk_ops: group_ops - precompute_ops,
                        restarts,
                        fruitless_ops: 0,
                        wall_ms: started.elapsed().as_secs_f64() * 1e3,
                    };
                }
                restarts += 1;
                break;
            }
        }
        if steps_left == 0 {
            break;
        }
    }

    RhoResult {
        k: None,
        group_ops,
        precompute_ops,
        walk_ops: group_ops - precompute_ops,
        restarts,
        fruitless_ops: 0,
        wall_ms: started.elapsed().as_secs_f64() * 1e3,
    }
}

// ── Unit conversion ────────────────────────────────────────────────────

/// Measure how many `BigUint` mul-mod-`n` operations fit in the time of
/// one Jacobian group operation on this curve.
///
/// The linear-algebra stage is counted in mul-mods and the walks in
/// group operations; without this factor the two cannot be added, and
/// adding them anyway is how a cost gets hidden rather than removed.
/// Reported alongside every table so a reader can redo the arithmetic.
pub fn calibrate_modmuls_per_group_op(
    curve: &HyperellipticCurveP,
    d: &MumfordDivisorP,
    n: &BigUint,
    rounds: usize,
) -> f64 {
    calibrate(curve, d, n, rounds).0
}

/// Measure `(mul-mods per group operation, seconds per group operation)`.
///
/// The second number is what lets a wall-clock measurement be converted
/// into the same unit as everything else, which is how the oracle is
/// now charged: its hand-derived multiplication count understated its
/// real cost by about an order of magnitude.
pub fn calibrate(
    curve: &HyperellipticCurveP,
    d: &MumfordDivisorP,
    n: &BigUint,
    rounds: usize,
) -> (f64, f64) {
    let mut acc = d.clone();
    let start = Instant::now();
    for _ in 0..rounds {
        acc = acc.add(d, curve);
    }
    let per_group_op = start.elapsed().as_secs_f64() / rounds as f64;
    std::hint::black_box(&acc);

    let a: BigUint = (n - BigUint::one()) >> 1u32;
    let mut x = a.clone();
    let start = Instant::now();
    for _ in 0..rounds {
        x = (&x * &a) % n;
    }
    let per_modmul = start.elapsed().as_secs_f64() / rounds as f64;
    std::hint::black_box(&x);

    if per_modmul <= 0.0 {
        return (f64::NAN, per_group_op);
    }
    (per_group_op / per_modmul, per_group_op)
}

// ── Head-to-head ───────────────────────────────────────────────────────

/// One row of the comparison table: both algorithms, one instance, one
/// unit.
#[derive(Clone, Debug)]
pub struct HeadToHeadRow {
    pub p: u64,
    pub genus: u32,
    pub jac_order: BigUint,
    /// Prime order of `D₁` — the group both algorithms actually work in.
    pub n: BigUint,
    pub factor_base_size: usize,

    /// Means over `HeadToHeadTrials::ic` runs.
    pub ic_relation_ops: f64,
    /// Of those, walk-step precomputation and restarts.
    pub ic_precompute_ops: f64,
    /// Group operations per trial, precomputation excluded.
    pub ic_ops_per_trial: f64,
    /// Linear algebra, in mul-mods.
    pub ic_la_modmuls: f64,
    /// Smoothness oracle (root finding + decomposition), in mul-mods —
    /// the hand-derived charge, kept for comparison.
    pub ic_oracle_modmuls: f64,
    /// The oracle in group-operation equivalents, from its **measured**
    /// wall time divided by the measured seconds per group operation.
    /// This is the number that enters `S`.
    pub ic_oracle_group_equiv: f64,
    /// What the hand-derived mul-mod charge would have claimed instead.
    /// Reported because the gap between the two is large (roughly 10x)
    /// and was only visible in wall clock.
    pub ic_oracle_group_equiv_charged: f64,
    /// The same, converted to group-operation equivalents.
    pub ic_la_group_equiv: f64,
    /// Large-prime bookkeeping (storing and combining partials), from
    /// its measured wall time converted in situ.  Zero without large
    /// primes.
    pub ic_partial_group_equiv: f64,
    /// Means per run: partials kept, and full relations combined from
    /// them.
    pub ic_partial_relations: f64,
    pub ic_combined_relations: f64,
    pub ic_total_group_ops: f64,
    pub ic_s: f64,
    pub ic_wall_ms: f64,
    pub ic_correct: bool,
    pub ic_smoothness_rate: f64,

    /// Means over `HeadToHeadTrials::rho` runs.
    pub rho_group_ops: f64,
    pub rho_precompute_ops: f64,
    pub rho_s: f64,
    /// `S` counting only the walk — this is the column that should sit
    /// at the `sqrt(π/2) ≈ 1.25` ideal.
    pub rho_walk_s: f64,
    pub rho_wall_ms: f64,
    pub rho_correct: bool,

    /// Unconditional floor, `(m+1)·(1 + g/c) + m²/c` — see the module
    /// docs.
    pub ic_floor_ops: f64,
    pub ic_floor_s: f64,
    /// What heuristic H1 (yield `= 1/g!`) predicts the cost will be.
    /// This is a **prediction, not a bound**: a search whose divisors
    /// are not uniformly distributed can beat it, and the factor-base
    /// walk does.
    pub ic_h1_predicted_ops: f64,
    pub ic_h1_predicted_s: f64,
    pub modmuls_per_group_op: f64,
}

impl HeadToHeadRow {
    /// `S_ic / S_rho`: below 1 means index calculus won this instance.
    pub fn ratio_to_reference(&self) -> f64 {
        self.ic_s / self.rho_s
    }

    /// `S_ic` against the derived floor.  A ratio near 1 means the
    /// implementation is close to what the method can do; it says
    /// nothing about whether the method is any good.
    pub fn ratio_to_floor(&self) -> f64 {
        self.ic_s / self.ic_floor_s
    }
}

fn factorial(g: u32) -> f64 {
    (1..=g as u64).map(|v| v as f64).product::<f64>().max(1.0)
}

fn sqrt_big(n: &BigUint) -> f64 {
    // Toy sizes; f64 is exact enough for a reporting denominator and
    // the alternative (integer sqrt) hides nothing here.
    let as_f = n
        .to_u64_digits()
        .iter()
        .rev()
        .fold(0f64, |acc, &d| acc * 2f64.powi(64) + d as f64);
    as_f.sqrt()
}

/// How many independent runs of each side a row averages over.
///
/// Both algorithms are randomised and a single run of rho has a very
/// wide distribution — its step count is a Rayleigh-like variable, not
/// a constant — so a one-seed row would report sampling noise as a
/// difference between methods.  These are the fixed sample counts of
/// this comparison; changing them changes the protocol.
#[derive(Clone, Debug)]
pub struct HeadToHeadTrials {
    pub ic: usize,
    pub rho: usize,
}

impl Default for HeadToHeadTrials {
    fn default() -> Self {
        // Rho's step count is a wide random variable and the measured
        // gap is now a factor of ~2, not ~10, so the sample has to be
        // large enough that the gap is not the sampling noise.  25 runs
        // put the standard error of the mean near 20% of one run's
        // spread; 9 did not.
        Self { ic: 5, rho: 40 }
    }
}

/// Run both algorithms on one instance and score them in `S`, averaging
/// each side over `trials` independent seeds.
///
/// `d1` must have prime order `n`, and `d2 = k·D₁` for the `k` being
/// recovered.  Every run is verified independently; `*_correct` is true
/// only if **all** runs of that side returned the right `k`, and a row
/// without it is not a result, per `AGENTS.md`.
#[allow(clippy::too_many_arguments)]
pub fn head_to_head(
    curve: &HyperellipticCurveP,
    d1: &MumfordDivisorP,
    d2: &MumfordDivisorP,
    n: &BigUint,
    ic_params: &HecIndexCalculusParams,
    rho_seed: u64,
    rho_max_steps: usize,
    expected_k: &BigUint,
    trials: &HeadToHeadTrials,
    rho_variant: &RhoVariant,
) -> HeadToHeadRow {
    let p_u = curve.p.to_u64_digits().first().copied().unwrap_or(0);
    let (conv, secs_per_group_op) = calibrate(curve, d1, n, 2_000);

    let ic_runs = trials.ic.max(1);
    let mut ic_relation_ops = 0f64;
    let mut ic_la_modmuls = 0f64;
    let mut ic_oracle_modmuls = 0f64;
    let mut ic_oracle_ns = 0f64;
    let mut ic_walk_ns = 0f64;
    let mut ic_solve_ns = 0f64;
    let mut ic_partial_ns = 0f64;
    let mut ic_partials = 0f64;
    let mut ic_combined = 0f64;
    let mut ic_precompute_ops = 0f64;
    let mut ic_ops_per_trial = 0f64;
    let mut ic_wall_ms = 0f64;
    let mut ic_smooth = 0f64;
    let mut ic_fb = 0usize;
    let mut ic_correct = true;
    for t in 0..ic_runs {
        let mut params = ic_params.clone();
        params.seed = ic_params.seed.wrapping_add(t as u64);
        let ic_start = Instant::now();
        let (ic_k, rep) = hec_index_calculus_dlp(curve, d1, d2, n, &params);
        ic_wall_ms += ic_start.elapsed().as_secs_f64() * 1e3;
        ic_relation_ops += rep.jacobian_ops as f64;
        ic_la_modmuls += rep.solve_row_ops as f64;
        ic_oracle_modmuls += rep.smoothness_field_ops as f64;
        ic_oracle_ns += rep.smoothness_wall_ns as f64;
        ic_walk_ns += rep.walk_wall_ns as f64;
        ic_solve_ns += rep.solve_wall_ns as f64;
        ic_partial_ns += rep.partial_wall_ns as f64;
        ic_partials += rep.partial_relations as f64;
        ic_combined += rep.combined_relations as f64;
        ic_precompute_ops += rep.precompute_ops as f64;
        ic_ops_per_trial += rep.ops_per_trial();
        ic_smooth += rep.smoothness_rate();
        ic_fb = rep.factor_base_size;
        ic_correct &= ic_k.as_ref() == Some(expected_k);
    }
    let f = ic_runs as f64;
    let (ic_precompute_ops, ic_ops_per_trial) = (ic_precompute_ops / f, ic_ops_per_trial / f);
    let (ic_oracle_ns, ic_walk_ns, ic_solve_ns) =
        (ic_oracle_ns / f, ic_walk_ns / f, ic_solve_ns / f);
    let (ic_partial_ns, ic_partials, ic_combined) =
        (ic_partial_ns / f, ic_partials / f, ic_combined / f);
    let (ic_relation_ops, ic_la_modmuls, ic_oracle_modmuls, ic_wall_ms, ic_smooth) = (
        ic_relation_ops / f,
        ic_la_modmuls / f,
        ic_oracle_modmuls / f,
        ic_wall_ms / f,
        ic_smooth / f,
    );

    let rho_runs = trials.rho.max(1);
    let mut rho_ops = 0f64;
    let mut rho_pre = 0f64;
    let mut rho_walk = 0f64;
    let mut rho_wall_ms = 0f64;
    let mut rho_correct = true;
    for t in 0..rho_runs {
        let r = pollard_rho_jacobian_with(
            curve,
            d1,
            d2,
            n,
            rho_seed.wrapping_add(t as u64),
            rho_max_steps,
            rho_variant,
        );
        rho_ops += r.group_ops as f64;
        rho_pre += r.precompute_ops as f64;
        rho_walk += r.walk_ops as f64;
        rho_wall_ms += r.wall_ms;
        rho_correct &= r.k.as_ref() == Some(expected_k);
    }
    let g = rho_runs as f64;
    let (rho_ops, rho_pre, rho_walk, rho_wall_ms) =
        (rho_ops / g, rho_pre / g, rho_walk / g, rho_wall_ms / g);

    let root_n = sqrt_big(n);
    // One group operation's cost **in situ**: the relation search's own
    // time divided by the operations it counted.  Converting the oracle
    // and the solve by this is apples-to-apples; converting them by a
    // tight calibration loop is not, and overstated the oracle by the
    // same margin the hand count understated it.  Fall back to the
    // calibrated figure only if the walk was too short to time.
    let in_situ_per_op = if ic_relation_ops > 0.0 && ic_walk_ns > 0.0 {
        ic_walk_ns / 1e9 / ic_relation_ops
    } else {
        secs_per_group_op
    };
    let oracle_group_equiv = ic_oracle_ns / 1e9 / in_situ_per_op.max(f64::MIN_POSITIVE);
    let solve_group_equiv = ic_solve_ns / 1e9 / in_situ_per_op.max(f64::MIN_POSITIVE);
    let oracle_group_equiv_charged = ic_oracle_modmuls / conv.max(f64::MIN_POSITIVE);
    let la_group_equiv = solve_group_equiv;
    let partial_group_equiv = ic_partial_ns / 1e9 / in_situ_per_op.max(f64::MIN_POSITIVE);
    let ic_total = ic_relation_ops + la_group_equiv + oracle_group_equiv + partial_group_equiv;

    let m = ic_fb as f64;
    // Both terms in group-operation equivalents: the solve's `m²`
    // matrix read is `m²` mul-mods, which is `m²/conv` group ops.  A
    // floor stated in a different unit from the measurement is not a
    // floor — it was one of these that first read as "below the
    // floor", which is impossible and meant the unit, not the run.
    // The oracle is not optional either: every trial must be tested for
    // smoothness, and the cheapest test that reads `u` at all touches
    // its `g` coefficients.  Charging zero for it would put the floor
    // below what any implementation can reach.
    // Unconditional floor: `m + 1` independent relations are needed,
    // each costs at least one group operation to produce a candidate and
    // at least one oracle call reading `u`'s `g` coefficients, and any
    // dense solve reads its `m × m` matrix once.
    //
    // It deliberately does **not** divide by a smoothness rate.  An
    // earlier version took the trial count to be `(m+1)·g!` from
    // heuristic H1, and the factor-base walk then measured *below* it —
    // which cannot happen to a real floor, and meant the boundary had an
    // assumption inside it rather than that the run was impossible.  H1
    // says the yield is `≈ 1/g!` for divisors drawn uniformly; a walk
    // stepping through the factor base does not draw uniformly, and the
    // measured genus-3 yield (0.17 – 0.27) sits above `1/3! = 0.167`.
    let relations_floor = m + 1.0;
    let floor_ops = relations_floor
        + (relations_floor * curve.genus as f64 + m * m) / conv.max(f64::MIN_POSITIVE);
    let h1_trials = (m + 1.0) * factorial(curve.genus);
    let h1_ops = h1_trials + (h1_trials * curve.genus as f64 + m * m) / conv.max(f64::MIN_POSITIVE);

    HeadToHeadRow {
        p: p_u,
        genus: curve.genus,
        jac_order: BigUint::zero(), // filled by the caller when known
        n: n.clone(),
        factor_base_size: ic_fb,

        ic_relation_ops,
        ic_precompute_ops,
        ic_ops_per_trial,
        ic_la_modmuls,
        ic_la_group_equiv: la_group_equiv,
        ic_partial_group_equiv: partial_group_equiv,
        ic_partial_relations: ic_partials,
        ic_combined_relations: ic_combined,
        ic_oracle_modmuls,
        ic_oracle_group_equiv: oracle_group_equiv,
        ic_oracle_group_equiv_charged: oracle_group_equiv_charged,
        ic_total_group_ops: ic_total,
        ic_s: ic_total / root_n,
        ic_wall_ms,
        ic_correct,
        ic_smoothness_rate: ic_smooth,

        rho_group_ops: rho_ops,
        rho_precompute_ops: rho_pre,
        rho_s: rho_ops / root_n,
        rho_walk_s: rho_walk / root_n,
        rho_wall_ms,
        rho_correct,

        ic_floor_ops: floor_ops,
        ic_floor_s: floor_ops / root_n,
        ic_h1_predicted_ops: h1_ops,
        ic_h1_predicted_s: h1_ops / root_n,
        modmuls_per_group_op: conv,
    }
}

/// Convenience: the factor-base size for `curve`, which is what the
/// floor and the solve both scale in.
pub fn full_factor_base_size(curve: &HyperellipticCurveP) -> usize {
    build_factor_base(curve, usize::MAX).len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::hyperelliptic_index_calculus::{
        prime_order_of, subgroup_generator, LinearAlgebra, RelationSearch, SmoothnessTest,
    };
    use crate::prime_hyperelliptic::{brute_force_jac_order_via_lpoly, FpPoly};

    fn toy_curve(p: u64) -> HyperellipticCurveP {
        let p = BigUint::from(p);
        let f = FpPoly::from_coeffs(
            vec![
                BigUint::from(1u32),
                BigUint::from(1u32),
                BigUint::from(2u32),
                BigUint::from(3u32),
                BigUint::zero(),
                BigUint::from(1u32),
            ],
            p.clone(),
        );
        HyperellipticCurveP::new(p, f, 2)
    }

    fn instance(
        p: u64,
    ) -> (
        HyperellipticCurveP,
        MumfordDivisorP,
        MumfordDivisorP,
        BigUint,
        BigUint,
    ) {
        let curve = toy_curve(p);
        let jac = brute_force_jac_order_via_lpoly(&curve);
        let fb = build_factor_base(&curve, usize::MAX);
        // Largest prime factor of #Jac.
        let mut l = BigUint::one();
        let mut rest = jac.clone();
        let mut d = BigUint::from(2u32);
        while &d * &d <= rest {
            while (&rest % &d).is_zero() {
                rest /= &d;
                if d > l {
                    l = d.clone();
                }
            }
            d += 1u32;
        }
        if rest > l {
            l = rest;
        }
        let d1 = subgroup_generator(&curve, &fb, &jac, &l).expect("generator");
        assert_eq!(prime_order_of(&curve, &d1, &jac).as_ref(), Some(&l));
        let k = &l / BigUint::from(3u32) + BigUint::from(5u32);
        let d2 = d1.scalar_mul(&k, &curve);
        (curve, d1, d2, l, k)
    }

    #[test]
    fn rho_solves_the_same_instance() {
        let (curve, d1, d2, n, k) = instance(41);
        let r = pollard_rho_jacobian(&curve, &d1, &d2, &n, 1, 1_000_000);
        assert_eq!(r.k, Some(k));
        assert!(r.group_ops > 0);
    }

    #[test]
    fn thread_instance_matches_the_frozen_rows() {
        // `N` for these instances is recorded in round seven's frozen
        // output (experiments/hyperelliptic_sparse_solve_r7/): the
        // library copy of the instance rule must pick the same subgroups
        // the frozen examples did.
        for (genus, p, n) in [
            (2u32, 41u64, 1321u64),
            (3, 23, 6299),
            (3, 31, 7333),
            (4, 31, 239753),
            (4, 61, 16790591),
        ] {
            let (_curve, _d1, got, _c) = thread_instance(p, genus).expect("instance");
            assert_eq!(got, BigUint::from(n), "genus {genus} p {p}");
        }
    }

    #[test]
    fn negation_folded_rho_solves_and_walks_less() {
        // The folded walk must recover `k` on every seed, keep the plain
        // walk's result untouched, and on average take fewer walk
        // operations: ideally `1/√2 ≈ 0.71` of the plain walk's.  The
        // bound here is loose (0.9) because twenty runs of either walk
        // are a wide random variable at this size; the registered
        // measurement is the round-nine sweep, not this test.
        let (curve, d1, d2, n, k) = instance(251);
        let (mut plain, mut folded, mut fruitless) = (0usize, 0usize, 0usize);
        for seed in 0..20u64 {
            let p = pollard_rho_jacobian_with(
                &curve,
                &d1,
                &d2,
                &n,
                seed,
                5_000_000,
                &RhoVariant::DistinguishedPoints,
            );
            let f = pollard_rho_jacobian_with(
                &curve,
                &d1,
                &d2,
                &n,
                seed,
                5_000_000,
                &RhoVariant::DistinguishedPointsNegation,
            );
            assert_eq!(p.k.as_ref(), Some(&k), "plain seed {seed}");
            assert_eq!(f.k.as_ref(), Some(&k), "folded seed {seed}");
            assert_eq!(p.fruitless_ops, 0, "the plain walk folds nothing");
            assert!(f.fruitless_ops <= f.walk_ops);
            plain += p.walk_ops;
            folded += f.walk_ops;
            fruitless += f.fruitless_ops;
        }
        assert!(
            (folded as f64) < 0.9 * plain as f64,
            "folded {folded} vs plain {plain} walk ops"
        );
        assert!(
            (fruitless as f64) < 0.15 * folded as f64,
            "fruitless {fruitless} of {folded}"
        );
    }

    #[test]
    fn mixed_hash_walks_solve_and_mixing_is_a_bijection_on_samples() {
        // The mixed-hash variants change only the walk's hash, so they
        // must still recover `k` on every seed.  The finaliser itself must
        // be injective (it is a bijection on u64); spot-check that no two
        // of a few thousand inputs collide.
        let (curve, d1, d2, n, k) = instance(101);
        for seed in 0..10u64 {
            for v in [
                RhoVariant::DistinguishedPointsMixed,
                RhoVariant::DistinguishedPointsNegationMixed,
            ] {
                let r = pollard_rho_jacobian_with(&curve, &d1, &d2, &n, seed, 5_000_000, &v);
                assert_eq!(r.k.as_ref(), Some(&k), "{v:?} seed {seed}");
            }
        }
        let mut seen = std::collections::HashSet::new();
        for z in 0..4096u64 {
            assert!(seen.insert(splitmix64_finalise(z.wrapping_mul(0x1234_5678_9abc_def1))));
        }
    }

    #[test]
    fn integer_model_recovers_k() {
        for (n, k) in [(1009u64, 777u64), (61667, 20562), (4620611, 1540210)] {
            for seed in 0..8u64 {
                let r =
                    rho_dp_integer_model(&BigUint::from(n), &BigUint::from(k), seed, 50_000_000);
                assert_eq!(r.k, Some(BigUint::from(k)), "N {n} seed {seed}");
            }
        }
    }

    #[test]
    fn full_width_branches_solve_on_both_walks() {
        // Round twelve's arms change only the branch coefficients' width,
        // so both must still recover `k` on every seed, and the branch
        // charge must be the full-width one.
        let (curve, d1, d2, n, k) = instance(101);
        for seed in 0..10u64 {
            let r = pollard_rho_jacobian_with(
                &curve,
                &d1,
                &d2,
                &n,
                seed,
                5_000_000,
                &RhoVariant::DistinguishedPointsFullWidth,
            );
            assert_eq!(r.k.as_ref(), Some(&k), "jacobian seed {seed}");
            assert_eq!(r.precompute_ops % (2 * (n.bits() as usize + 1)), 0);
            let f = pollard_rho_jacobian_with(
                &curve,
                &d1,
                &d2,
                &n,
                seed,
                5_000_000,
                &RhoVariant::DistinguishedPointsNegationFullWidth,
            );
            assert_eq!(f.k.as_ref(), Some(&k), "folded jacobian seed {seed}");
            assert!(f.fruitless_ops <= f.walk_ops);
        }
        for (nn, kk) in [(61667u64, 20562u64), (4620611, 1540210)] {
            for seed in 0..8u64 {
                let r = rho_dp_integer_model_width(
                    &BigUint::from(nn),
                    &BigUint::from(kk),
                    seed,
                    50_000_000,
                    true,
                );
                assert_eq!(r.k, Some(BigUint::from(kk)), "N {nn} seed {seed}");
            }
        }
    }

    #[test]
    fn fold_negation_picks_one_of_each_pair() {
        // `{D, −D}` must map to one representative from either side, and
        // the flag must say which one was taken.
        let (curve, d1, _d2, _n, _k) = instance(61);
        let mut d = d1.clone();
        for _ in 0..200 {
            let (a, fa) = fold_negation(d.clone(), &curve);
            let (b, fb) = fold_negation(d.neg(&curve), &curve);
            assert_eq!(a, b, "both signs fold to one representative");
            assert_eq!(a, if fa { d.neg(&curve) } else { d.clone() });
            if !d.v.is_zero() {
                assert_ne!(fa, fb, "exactly one side is flipped");
            }
            d = d.add(&d1, &curve);
        }
    }

    #[test]
    fn distinguished_points_cost_about_a_third_of_floyd() {
        let (curve, d1, d2, n, k) = instance(251);
        let mut floyd = 0usize;
        let mut dp = 0usize;
        for seed in 0..12u64 {
            let f = pollard_rho_jacobian_with(
                &curve,
                &d1,
                &d2,
                &n,
                seed,
                5_000_000,
                &RhoVariant::Floyd,
            );
            let d = pollard_rho_jacobian_with(
                &curve,
                &d1,
                &d2,
                &n,
                seed,
                5_000_000,
                &RhoVariant::DistinguishedPoints,
            );
            assert_eq!(f.k.as_ref(), Some(&k), "floyd seed {seed}");
            assert_eq!(d.k.as_ref(), Some(&k), "dp seed {seed}");
            floyd += f.walk_ops;
            dp += d.walk_ops;
        }
        // Same expected step count, a third of the operations per step.
        // Averaged over 12 seeds because one run of either is a wide
        // random variable; the claim is the ratio, not any single run.
        assert!(
            (dp as f64) < 0.55 * floyd as f64,
            "dp {dp} vs floyd {floyd} walk ops"
        );
    }

    #[test]
    fn dp_rho_terminates_on_every_seed_it_is_given() {
        // Regression: a deterministic walk can enter a cycle holding no
        // distinguished point, and the first implementation then ran to
        // `max_steps` — 200 million operations for one unlucky seed,
        // which presented as a hung benchmark rather than a slow one.
        // Every seed must finish well inside a budget that a healthy
        // run never approaches.
        let (curve, d1, d2, n, k) = instance(251);
        for seed in 0..40u64 {
            let r = pollard_rho_jacobian_with(
                &curve,
                &d1,
                &d2,
                &n,
                seed,
                200_000,
                &RhoVariant::DistinguishedPoints,
            );
            assert_eq!(r.k.as_ref(), Some(&k), "seed {seed} failed to solve");
            assert!(
                r.group_ops < 100_000,
                "seed {seed} spent {} ops — the walk is not bounded",
                r.group_ops
            );
        }
    }

    #[test]
    #[ignore = "wall-clock cost calibration requires an isolated CPU"]
    fn the_unit_tracks_wall_clock() {
        // The accounting has now been wrong twice in opposite
        // directions: a hand-derived multiplication charge understated
        // the oracle about 20x, and converting its measured time by a
        // tight calibration loop overstated it about as much.  Both
        // times the only signal was that wall clock disagreed with `S`.
        // So make that disagreement a test: the ratio the unit reports
        // and the ratio the clock reports must be within a factor of
        // three of each other.  Anything worse means a cost is being
        // counted in a unit it does not belong to.
        let (curve, d1, d2, n, k) = instance(251);
        let params = HecIndexCalculusParams {
            fb_size: usize::MAX,
            extra_relations: 8,
            max_trials: 200_000,
            seed: 31337,
            search: RelationSearch::factor_base_walk(),
            linear_algebra: LinearAlgebra::Sparse,
            smoothness: SmoothnessTest::Gcd,
            large_primes: false,
        };
        let row = head_to_head(
            &curve,
            &d1,
            &d2,
            &n,
            &params,
            7,
            1_000_000,
            &k,
            &HeadToHeadTrials { ic: 3, rho: 9 },
            &RhoVariant::DistinguishedPoints,
        );
        assert!(row.ic_correct && row.rho_correct, "{row:?}");
        let by_unit = row.ratio_to_reference();
        let by_clock = row.ic_wall_ms / row.rho_wall_ms.max(f64::MIN_POSITIVE);
        let disagreement = (by_unit / by_clock).max(by_clock / by_unit);
        assert!(
            disagreement < 3.0,
            "S says {by_unit:.2} and the clock says {by_clock:.2} \
             (factor {disagreement:.1}); a cost is in the wrong unit"
        );
    }

    #[test]
    fn head_to_head_scores_both_sides() {
        let (curve, d1, d2, n, k) = instance(41);
        let params = HecIndexCalculusParams {
            fb_size: usize::MAX,
            extra_relations: 5,
            max_trials: 50_000,
            seed: 20260916,
            search: RelationSearch::Random,
            linear_algebra: LinearAlgebra::Dense,
            smoothness: SmoothnessTest::Scan,
            large_primes: false,
        };
        let row = head_to_head(
            &curve,
            &d1,
            &d2,
            &n,
            &params,
            1,
            1_000_000,
            &k,
            &HeadToHeadTrials { ic: 2, rho: 3 },
            &RhoVariant::DistinguishedPoints,
        );
        assert!(
            row.ic_correct,
            "index calculus row without a verified answer"
        );
        assert!(row.rho_correct, "rho row without a verified answer");
        assert!(row.ic_s > 0.0 && row.rho_s > 0.0);
        assert!(row.modmuls_per_group_op.is_finite());
        // The floor is a floor: no run may come in under it.
        assert!(
            row.ic_total_group_ops >= row.ic_floor_ops * 0.999,
            "measured {} below the derived floor {}",
            row.ic_total_group_ops,
            row.ic_floor_ops
        );
    }
}
