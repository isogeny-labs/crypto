//! **`bitop/v1` primitive counters for the Joux–Vitse cover route on
//! `E(F_{p⁶})` and its rho reference** (`jv_cover`, `jv_sieve`,
//! `jv_isogeny_walk`, `f4_fp`, `jv_rho_strong`).
//!
//! The route's committed outputs count "`F_p` multiplication equivalents":
//! an `F_{p²}` product is 5, an inversion a flat 21, additions are free, the
//! F4 counter sees only products, and the Wiedemann operations are a flat 16
//! each.  This module counts the same runs in the primitives of
//! `docs/units/BITOPS.md` ([`crate::cryptanalysis::bitops::Prim`]) without
//! touching those outputs: the legacy counters keep counting as before and
//! every committed experiment still reproduces.
//!
//! ## How the counts are taken
//!
//! The arithmetic of the whole route bottoms out in a handful of free
//! functions, `gaudry_cubic::{mm, am, sm}` and
//! `residual_walk::{inv_mod, pow_mod}`, which call the hooks of this module
//! ([`hook_mul`], [`hook_add`], [`hook_inv`], [`hook_pow`]).  The hooks cost
//! one relaxed atomic load while counting is off ([`enable`]), so every other
//! user of those functions is unaffected.  While it is on, a hook charges
//! the operation to this thread's tally by its **modulus**: the field prime
//! `p` ([`set_moduli`]) gives `field_*`, the subgroup order `ℓ` gives
//! `modr_*`, any other modulus is tallied apart ([`snapshot_other`]) so a
//! reader can check that nothing went unclassified.  `F4`'s own Barrett
//! arithmetic charges through [`charge`] at its operation counter, and the
//! few inline operations (`(p − a) % p`) and the table probes and scans of
//! the route charge explicitly where they run.
//!
//! The tally is **thread-local**, and it is complete only when every counted
//! path runs on the thread that reads it: the study runs the index-calculus
//! arm inside a one-thread rayon pool, where every nested parallel iterator
//! runs on the calling thread.  A hook that fires on a thread that did not
//! [`claim`] the tally is counted in a process-wide [`foreign`] counter;
//! a run whose `foreign()` is not zero lost work and is not admitted.
//!
//! ## Counting rules (one line each)
//!
//! * `field_mul` — one product reduced mod `p` (`mm`), including products by
//!   a small constant; a square is a product (`bitop/v1` prices them alike).
//! * `field_add` — one addition, subtraction or negation mod `p` (`am`,
//!   `sm`, the inline `(p − a) % p`).
//! * `field_inv` — one inversion mod `p` (`inv_mod`), whatever algorithm.
//! * `pow_mod(x, e, p)` — square-and-multiply as executed:
//!   `bits(e)` squarings and `popcount(e)` products.
//! * `modr_*` — the same operations modulo `ℓ` (Wiedemann, Berlekamp–Massey,
//!   the collision solve).
//! * one F4 row operation (the engine's `count_ops` unit) — a
//!   multiply-subtract: one `field_mul` and one `field_add`.
//! * `hash_probe` — one insert into or lookup in a hash table; the key's
//!   words absorbed by the hash are `word_op`s beside it.
//! * `word_op` — one 64-bit compare, counter update or scan step that the
//!   counted code executes on its data (sieve counter updates, the
//!   factor-base test of each sieve step, singleton-filter column counts,
//!   F4's column scans), and one 64-bit RNG draw.

use std::cell::Cell;
use std::sync::atomic::{AtomicBool, Ordering};

use serde::Serialize;

use super::bitops::{Field, NoPrice, Prim, Widths};
use crate::prime_hyperelliptic::ops::KINDS;
pub use crate::prime_hyperelliptic::ops::{Kind, OpTally};

static ENABLED: AtomicBool = AtomicBool::new(false);
/// Set once any hook fires on a thread that did not [`claim`] the tally.
/// A flag written only on its first change, so threads that are not
/// counted (other tests, other pools) never contend on a shared line.
static FOREIGN_SEEN: AtomicBool = AtomicBool::new(false);

/// Number of phases.
pub const PHASES: usize = 15;

/// Where a counted operation is attributed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[repr(usize)]
pub enum Phase {
    /// Building the instance (the weak curve, the moves off the weak locus,
    /// the planted logarithm): the instance's work, never the attacker's.
    Instance = 0,
    /// The harness's own bookkeeping (unit prices, the paired walk-free
    /// route): reported, never charged to an arm.
    Harness = 1,
    /// Index calculus: the isogeny walk from the challenge to a weak curve.
    Walk = 2,
    /// Index calculus: `G`, `Q` carried along the walk's path.
    Transport = 3,
    /// Index calculus: the model change, the cover's construction and the
    /// transfer of `G`, `Q` to the Jacobian (cover set-up).
    Model = 4,
    /// Index calculus: the factor base and its index.
    FactorBase = 5,
    /// Index calculus: the sieve (tables, line enumeration, sieving,
    /// relation extraction and verification, duplicate removal).
    Relations = 6,
    /// Index calculus: the residual stream and the six-point descent.
    Descent = 7,
    /// Index calculus: the core filter and Wiedemann modulo `ℓ`.
    LinearAlgebra = 8,
    /// Index calculus: `[d]G = Q` checks (on the transported instance and
    /// on the challenge curve).
    Verification = 9,
    /// Rho: the walk's multipliers, start points and restarts.
    RhoPrecompute = 10,
    /// Rho: the walk steps (group law, canonicalisation, cycle handling).
    RhoWalk = 11,
    /// Rho: collision detection (step hash, distinguished-point test,
    /// table probes) and the collision solve.
    RhoCollision = 12,
    /// Rho: `[d]G = Q` on the challenge curve.
    RhoVerification = 13,
    /// Anything outside a phase.
    Unattributed = 14,
}

impl Phase {
    pub const ALL: [Phase; PHASES] = [
        Phase::Instance,
        Phase::Harness,
        Phase::Walk,
        Phase::Transport,
        Phase::Model,
        Phase::FactorBase,
        Phase::Relations,
        Phase::Descent,
        Phase::LinearAlgebra,
        Phase::Verification,
        Phase::RhoPrecompute,
        Phase::RhoWalk,
        Phase::RhoCollision,
        Phase::RhoVerification,
        Phase::Unattributed,
    ];

    /// The index-calculus attack's phases (the arm's total is their sum).
    /// Work inside the attack that no phase claims is in `Unattributed`, and
    /// is charged to the arm.
    pub const IC: [Phase; 9] = [
        Phase::Unattributed,
        Phase::Walk,
        Phase::Transport,
        Phase::Model,
        Phase::FactorBase,
        Phase::Relations,
        Phase::Descent,
        Phase::LinearAlgebra,
        Phase::Verification,
    ];

    /// The rho attack's phases.
    pub const RHO: [Phase; 4] = [
        Phase::RhoPrecompute,
        Phase::RhoWalk,
        Phase::RhoCollision,
        Phase::RhoVerification,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Phase::Instance => "instance",
            Phase::Harness => "harness",
            Phase::Walk => "walk",
            Phase::Transport => "transport",
            Phase::Model => "model",
            Phase::FactorBase => "factor_base",
            Phase::Relations => "relations",
            Phase::Descent => "descent",
            Phase::LinearAlgebra => "linear_algebra",
            Phase::Verification => "verification",
            Phase::RhoPrecompute => "precompute",
            Phase::RhoWalk => "walk",
            Phase::RhoCollision => "collision",
            Phase::RhoVerification => "verification",
            Phase::Unattributed => "unattributed",
        }
    }
}

/// A tally per phase.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct JvTally {
    pub phases: [OpTally; PHASES],
}

impl JvTally {
    pub fn phase(&self, phase: Phase) -> &OpTally {
        &self.phases[phase as usize]
    }

    pub fn phase_mut(&mut self, phase: Phase) -> &mut OpTally {
        &mut self.phases[phase as usize]
    }

    pub fn since(&self, before: &JvTally) -> JvTally {
        let mut out = JvTally::default();
        for i in 0..PHASES {
            out.phases[i] = self.phases[i].since(&before.phases[i]);
        }
        out
    }

    pub fn add(&mut self, other: &JvTally) {
        for i in 0..PHASES {
            self.phases[i].add(&other.phases[i]);
        }
    }

    /// Sum over the given phases.
    pub fn sum(&self, phases: &[Phase]) -> OpTally {
        let mut out = OpTally::default();
        for &p in phases {
            out.add(self.phase(p));
        }
        out
    }

    /// Sum over every phase.
    pub fn total(&self) -> OpTally {
        self.sum(&Phase::ALL)
    }
}

/// The widths of the route: `F_p` primitives at `bits(p)`, mod-`ℓ` ones at
/// `bits(ℓ)`.
pub fn widths(p: u64, l: u64) -> Widths {
    Widths {
        field: Field::Prime {
            bits: 64 - p.leading_zeros(),
        },
        r_bits: 64 - l.leading_zeros(),
    }
}

/// `Σ count · gates` of a tally.
pub fn price(t: &OpTally, w: &Widths) -> Result<u128, NoPrice> {
    t.bitops(w)
}

/// Counts and their price, by primitive name (for JSON).
pub fn prims_json(t: &OpTally) -> serde_json::Value {
    let mut m = serde_json::Map::new();
    for k in Kind::ALL {
        m.insert(
            k.prim().name().to_string(),
            serde_json::Value::from(t.get(k)),
        );
    }
    m.insert("rng_words".into(), serde_json::Value::from(t.rng_words));
    serde_json::Value::Object(m)
}

type Cells = [[Cell<u64>; KINDS + 1]; PHASES];

thread_local! {
    static TALLY: Cells = const { [const { [const { Cell::new(0) }; KINDS + 1] }; PHASES] };
    static PHASE: Cell<usize> = const { Cell::new(Phase::Unattributed as usize) };
    static FIELD_P: Cell<u64> = const { Cell::new(0) };
    static MODR_N: Cell<u64> = const { Cell::new(0) };
    static OWNER: Cell<bool> = const { Cell::new(false) };
    static OTHER: [Cell<u64>; PHASES] = const { [const { Cell::new(0) }; PHASES] };
}

/// Turn the hooks on or off for the whole process.
pub fn enable(on: bool) {
    ENABLED.store(on, Ordering::SeqCst);
}

/// Is counting on?
#[inline(always)]
pub fn enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

/// This thread reads the tally: hooks firing here are counted; hooks firing
/// on any other thread while counting is on go to [`foreign`].
pub fn claim() {
    OWNER.with(|o| o.set(true));
}

/// `1` once any hook has fired on a thread that did not [`claim`] the
/// tally (work the reader cannot see), else `0`.
pub fn foreign() -> u64 {
    FOREIGN_SEEN.load(Ordering::SeqCst) as u64
}

/// Operations on a modulus that is neither `p` nor `ℓ` (this thread), by
/// phase (the instance's prime searches, say).
pub fn snapshot_other() -> [u64; PHASES] {
    OTHER.with(|o| core::array::from_fn(|i| o[i].get()))
}

/// The field prime and the subgroup order the hooks classify by.
pub fn set_moduli(p: u64, l: u64) {
    FIELD_P.with(|c| c.set(p));
    MODR_N.with(|c| c.set(l));
}

#[cold]
fn mark_foreign() {
    if !FOREIGN_SEEN.load(Ordering::Relaxed) {
        FOREIGN_SEEN.store(true, Ordering::Relaxed);
    }
}

#[inline]
fn bump(kind: Kind, n: u64) {
    if n == 0 {
        return;
    }
    if !OWNER.with(Cell::get) {
        mark_foreign();
        return;
    }
    let phase = PHASE.with(Cell::get);
    TALLY.with(|t| {
        let c = &t[phase][kind as usize];
        c.set(c.get() + n);
    });
}

/// Charge `n` operations of `kind` to this thread's current phase (only
/// while counting is on).
#[inline]
pub fn charge(kind: Kind, n: u64) {
    if enabled() {
        bump(kind, n);
    }
}

/// One 64-bit RNG draw per `n`: a `word_op` each, also kept in `rng_words`.
pub fn rng_words(n: u64) {
    if !enabled() || n == 0 {
        return;
    }
    bump(Kind::WordOp, n);
    if OWNER.with(Cell::get) {
        let phase = PHASE.with(Cell::get);
        TALLY.with(|t| {
            let c = &t[phase][KINDS];
            c.set(c.get() + n);
        });
    }
}

/// `n` F4 row operations (multiply-subtracts): `n` products and `n`
/// additions mod `p`.
#[inline]
pub fn row_ops(n: u64) {
    if enabled() {
        bump(Kind::FieldMul, n);
        bump(Kind::FieldAdd, n);
    }
}

/// `n` 64-bit word operations.
#[inline]
pub fn words(n: u64) {
    if enabled() {
        bump(Kind::WordOp, n);
    }
}

/// `n` hash-table probes, each key absorbing `key_words` words.
#[inline]
pub fn probes(n: u64, key_words: u64) {
    if enabled() {
        bump(Kind::HashProbe, n);
        bump(Kind::WordOp, n * key_words);
    }
}

/// One hash-table probe whose key absorbs `words` 64-bit words.
#[inline]
pub fn probe(words: u64) {
    if enabled() {
        bump(Kind::HashProbe, 1);
        bump(Kind::WordOp, words);
    }
}

#[derive(Clone, Copy)]
enum Class {
    Field,
    Modr,
    Other,
}

#[inline]
fn class(m: u64) -> Class {
    if FIELD_P.with(Cell::get) == m {
        Class::Field
    } else if MODR_N.with(Cell::get) == m {
        Class::Modr
    } else {
        Class::Other
    }
}

#[inline(never)]
fn slow(m: u64, field: Kind, modr: Kind, n: u64) {
    match class(m) {
        Class::Field => bump(field, n),
        Class::Modr => bump(modr, n),
        Class::Other => {
            if OWNER.with(Cell::get) {
                let phase = PHASE.with(Cell::get);
                OTHER.with(|c| c[phase].set(c[phase].get() + n));
            } else {
                mark_foreign();
            }
        }
    }
}

/// A product mod `m` (called by `gaudry_cubic::mm`).
#[inline(always)]
pub fn hook_mul(m: u64) {
    if enabled() {
        slow(m, Kind::FieldMul, Kind::ModrMul, 1);
    }
}

/// An addition, subtraction or negation mod `m`.
#[inline(always)]
pub fn hook_add(m: u64) {
    if enabled() {
        slow(m, Kind::FieldAdd, Kind::ModrAdd, 1);
    }
}

/// `n` additions mod `m` (inline negations).
#[inline(always)]
pub fn hook_adds(m: u64, n: u64) {
    if enabled() {
        slow(m, Kind::FieldAdd, Kind::ModrAdd, n);
    }
}

/// An inversion mod `m` (called by `residual_walk::inv_mod`).
#[inline(always)]
pub fn hook_inv(m: u64) {
    if enabled() {
        slow(m, Kind::FieldInv, Kind::ModrInv, 1);
    }
}

/// `x^e mod m` by square-and-multiply as `residual_walk::pow_mod` runs it:
/// one squaring per bit of `e`, one product per set bit.
#[inline(always)]
pub fn hook_pow(m: u64, e: u64) {
    if enabled() {
        let bits = 64 - e.leading_zeros() as u64;
        slow(
            m,
            Kind::FieldMul,
            Kind::ModrMul,
            bits + e.count_ones() as u64,
        );
    }
}

/// Everything this thread has tallied, by phase.
pub fn snapshot() -> JvTally {
    TALLY.with(|cells| {
        let mut out = JvTally::default();
        for (p, row) in cells.iter().enumerate() {
            for k in 0..KINDS {
                out.phases[p].counts[k] = row[k].get();
            }
            out.phases[p].rng_words = row[KINDS].get();
        }
        out
    })
}

/// The current phase.
pub fn current_phase() -> Phase {
    Phase::ALL[PHASE.with(Cell::get)]
}

/// Restores the previous phase when dropped.
#[must_use = "the phase reverts when the guard drops"]
pub struct PhaseGuard {
    previous: usize,
}

impl Drop for PhaseGuard {
    fn drop(&mut self) {
        PHASE.with(|p| p.set(self.previous));
    }
}

/// Attribute counted work to `phase` until the guard drops.
pub fn enter(phase: Phase) -> PhaseGuard {
    let previous = PHASE.with(|p| p.replace(phase as usize));
    PhaseGuard { previous }
}

/// Comparisons of a sort of `k` keys: `k·⌈log₂ k⌉` word ops.
pub fn sort_cost(k: usize) -> u64 {
    crate::prime_hyperelliptic::ops::sort_cost(k)
}

/// The `bitop/v1` name of a kind.
pub fn kind_name(k: Kind) -> &'static str {
    let p: Prim = k.prim();
    p.name()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::gaudry_cubic::{am, mm, sm};
    use crate::cryptanalysis::residual_walk::{inv_mod, pow_mod};

    /// Run `f` with counting on, on this thread, and return its tally.
    /// (Tests run in parallel threads; the global switch only adds hook
    /// calls elsewhere, which go to `foreign` and are not asserted on.)
    fn counted<R>(p: u64, l: u64, f: impl FnOnce() -> R) -> (R, JvTally) {
        enable(true);
        claim();
        set_moduli(p, l);
        let before = snapshot();
        let r = {
            let _g = enter(Phase::Relations);
            f()
        };
        (r, snapshot().since(&before))
    }

    #[test]
    fn hooks_classify_by_modulus() {
        let (_, t) = counted(101, 1_000_003, || {
            let a = mm(3, 5, 101);
            let b = am(a, 7, 101);
            let _ = sm(b, 9, 101);
            let _ = mm(3, 5, 1_000_003);
            let _ = am(3, 5, 1_000_003);
            let _ = inv_mod(7, 101);
            let _ = inv_mod(7, 1_000_003);
            let _ = pow_mod(2, 0b1011, 101);
        });
        let r = t.phase(Phase::Relations);
        assert_eq!(r.get(Kind::FieldMul), 1 + 4 + 3);
        assert_eq!(r.get(Kind::FieldAdd), 2);
        assert_eq!(r.get(Kind::FieldInv), 1);
        assert_eq!(r.get(Kind::ModrMul), 1);
        assert_eq!(r.get(Kind::ModrAdd), 1);
        assert_eq!(r.get(Kind::ModrInv), 1);
    }

    #[test]
    fn widths_follow_p_and_l() {
        let w = widths(1511, 2_975_272_819_875_491_089);
        assert_eq!(w.field, Field::Prime { bits: 11 });
        assert_eq!(w.r_bits, 62);
    }
}
