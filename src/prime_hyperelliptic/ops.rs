//! **Deterministic `bitop/v1` primitive counters** for the prime-field
//! hyperelliptic code and the index-calculus / rho comparison built on it.
//!
//! Every coefficient-level `F_p` operation the polynomial and Mumford
//! arithmetic executes, every operation modulo the subgroup order `N`, and
//! the table probes and word operations of the walks are tallied here, in
//! the primitives of `docs/units/BITOPS.md`
//! ([`crate::cryptanalysis::bitops::Prim`]), so that both arms of the
//! hyperelliptic head-to-head are priced by one instrument instead of by a
//! wall-clock conversion.
//!
//! The tallies are **thread-local** and are only ever read on the thread
//! that did the work: a reader takes a [`snapshot`] before and after and
//! subtracts.  None of the counted paths runs inside rayon workers, so
//! every count is a deterministic function of the inputs and seeds.
//!
//! Each tally is split by [`Phase`].  The caller sets the current phase
//! ([`enter`], restored when the guard drops); work outside any phase lands
//! in [`Phase::Unattributed`].  Work done inside the *checked arithmetic's
//! validation* (`HyperellipticCurveP::validate`, `MumfordDivisorP::validate`,
//! which every checked group operation runs on its inputs and output) is
//! counted in the phase like everything else **and** separately in the
//! validation subset ([`snapshot_validation`]), so a reader can see how much
//! of a group operation is the implementation re-checking its own domain.
//!
//! ## Counting rules (one line each)
//!
//! * `field_add` — one `F_p` addition, subtraction or negation that the code
//!   executes (`(a + b) % p`, `(a + p − b) % p`, `(p − a) % p`), including
//!   the full-length coefficient loops of `FpPoly::add`/`sub` (positions past
//!   the shorter operand included, because the loop computes them).
//! * `field_mul` — one `F_p` product `(a·b) % p`, including products by a
//!   small integer constant (derivatives, `4·a₀`); products a loop skips
//!   because a coefficient is zero are not charged.
//! * `field_sqr` — a product of an element with itself where the code can
//!   tell (the diagonal terms of `FpPoly::mul(x, x)`, `y·y`).
//! * `field_inv` — one inversion mod `p` (`fp_inv`, `mod_inverse(·, p)`).
//! * `field_sqrt` — one Tonelli–Shanks square root that found a root
//!   (`sqrt_mod_p` charges its Euler test as a `legendre` first).
//! * `legendre` — one quadratic-residue test (a Jacobi symbol, the gcd-type
//!   loop `bitop/v1` prices it as; Euler's criterion only off the word size).
//! * `modpow` with a public exponent `e` other than the above — charged as
//!   left-to-right square-and-multiply: `bits(e) − 1` squarings and
//!   `popcount(e) − 1` products ([`charge_pow`]).
//! * reduction of a freshly drawn wide random integer mod `p` or mod `N` —
//!   one `field_mul` / `modr_mul` (a Barrett reduction).
//! * canonicalising reductions of operands that are already reduced
//!   (`c % p` with `c < p`) — not charged: they are the identity.
//! * `modr_add` / `modr_mul` / `modr_inv` — one addition, subtraction,
//!   negation, product or inversion modulo `N`.
//! * `hash_probe` — one insert into or lookup in a hash table keyed by a
//!   field element, an `x`-coordinate or a walk hash.
//! * `word_op` — one 64-bit compare or index step that decides control flow
//!   (branch selection, distinguished-point test, a field-element compare
//!   with `(p−1)/2`, a coefficient compare of an equality test, pivot-column
//!   scan entry, one comparison of a sort of `k` keys charged
//!   `k·⌈log₂ k⌉`, one merge step of two sorted sparse rows, one
//!   column-count update), and one 64-bit word absorbed into a divisor hash,
//!   and one 64-bit draw from the RNG (the generator's own internals —
//!   ChaCha12 in `StdRng` — are not counted; see `rng_words`).

use std::cell::Cell;

use crate::cryptanalysis::bitops::{NoPrice, Prim, Widths};

/// Number of counted primitive kinds.
pub const KINDS: usize = 11;

/// The primitive kinds, in tally order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Kind {
    FieldAdd = 0,
    FieldMul = 1,
    FieldSqr = 2,
    FieldInv = 3,
    FieldSqrt = 4,
    Legendre = 5,
    ModrAdd = 6,
    ModrMul = 7,
    ModrInv = 8,
    WordOp = 9,
    HashProbe = 10,
}

impl Kind {
    pub const ALL: [Kind; KINDS] = [
        Kind::FieldAdd,
        Kind::FieldMul,
        Kind::FieldSqr,
        Kind::FieldInv,
        Kind::FieldSqrt,
        Kind::Legendre,
        Kind::ModrAdd,
        Kind::ModrMul,
        Kind::ModrInv,
        Kind::WordOp,
        Kind::HashProbe,
    ];

    /// The `bitop/v1` primitive this kind is priced as.
    pub fn prim(self) -> Prim {
        match self {
            Kind::FieldAdd => Prim::FieldAdd,
            Kind::FieldMul => Prim::FieldMul,
            Kind::FieldSqr => Prim::FieldSqr,
            Kind::FieldInv => Prim::FieldInv,
            Kind::FieldSqrt => Prim::FieldSqrt,
            Kind::Legendre => Prim::Legendre,
            Kind::ModrAdd => Prim::ModrAdd,
            Kind::ModrMul => Prim::ModrMul,
            Kind::ModrInv => Prim::ModrInv,
            Kind::WordOp => Prim::WordOp,
            Kind::HashProbe => Prim::HashProbe,
        }
    }
}

/// Number of phases.
pub const PHASES: usize = 6;

/// Where a counted operation is attributed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Phase {
    /// Anything outside a measured run (instance selection, calibration).
    Unattributed = 0,
    /// Factor-base build, walk-step / branch precomputation, start points
    /// and restarts.
    Precompute = 1,
    /// Relation search / rho walk steps and their bookkeeping.
    Walk = 2,
    /// Smoothness oracle (index calculus); collision detection — divisor
    /// hash, distinguished-point test, table probe — (rho).
    Oracle = 3,
    /// Relation system build and solve (index calculus); collision solve
    /// `k = Δa/Δb` (rho).
    LinearAlgebra = 4,
    /// The final `k·D₁ = D₂` check.
    Verification = 5,
}

impl Phase {
    pub const ALL: [Phase; PHASES] = [
        Phase::Unattributed,
        Phase::Precompute,
        Phase::Walk,
        Phase::Oracle,
        Phase::LinearAlgebra,
        Phase::Verification,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Phase::Unattributed => "unattributed",
            Phase::Precompute => "precompute",
            Phase::Walk => "walk",
            Phase::Oracle => "oracle",
            Phase::LinearAlgebra => "linear_algebra",
            Phase::Verification => "verification",
        }
    }
}

/// Counts of each primitive kind.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OpTally {
    pub counts: [u64; KINDS],
    /// 64-bit RNG draws (each also charged one `word_op`), reported so the
    /// uncounted generator internals can be sized.
    pub rng_words: u64,
}

impl OpTally {
    pub fn get(&self, kind: Kind) -> u64 {
        self.counts[kind as usize]
    }

    pub fn since(&self, before: &OpTally) -> OpTally {
        let mut out = OpTally::default();
        for i in 0..KINDS {
            out.counts[i] = self.counts[i].wrapping_sub(before.counts[i]);
        }
        out.rng_words = self.rng_words.wrapping_sub(before.rng_words);
        out
    }

    pub fn add(&mut self, other: &OpTally) {
        for i in 0..KINDS {
            self.counts[i] += other.counts[i];
        }
        self.rng_words += other.rng_words;
    }

    pub fn is_zero(&self) -> bool {
        self.counts.iter().all(|&c| c == 0)
    }

    /// `F_p` operations only (add, mul, sqr, inv, sqrt, legendre).
    pub fn field_ops(&self) -> u64 {
        self.counts[..6].iter().sum()
    }

    /// `(primitive, count)` for every kind.
    pub fn prims(&self) -> Vec<(Prim, u64)> {
        Kind::ALL
            .iter()
            .map(|&k| (k.prim(), self.counts[k as usize]))
            .collect()
    }

    /// `Σ count · gates` at these widths: field primitives and hash probes
    /// at `bits(p)`, mod-`N` primitives at `bits(N)`.
    pub fn bitops(&self, widths: &Widths) -> Result<u128, NoPrice> {
        let mut total = 0u128;
        for k in Kind::ALL {
            let c = self.counts[k as usize];
            if c != 0 {
                total += c as u128 * widths.gates(k.prim())?;
            }
        }
        Ok(total)
    }
}

/// A tally per phase.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PhaseTally {
    pub phases: [OpTally; PHASES],
}

impl PhaseTally {
    pub fn phase(&self, phase: Phase) -> &OpTally {
        &self.phases[phase as usize]
    }

    pub fn since(&self, before: &PhaseTally) -> PhaseTally {
        let mut out = PhaseTally::default();
        for i in 0..PHASES {
            out.phases[i] = self.phases[i].since(&before.phases[i]);
        }
        out
    }

    pub fn add(&mut self, other: &PhaseTally) {
        for i in 0..PHASES {
            self.phases[i].add(&other.phases[i]);
        }
    }

    /// Sum over every phase.
    pub fn total(&self) -> OpTally {
        let mut out = OpTally::default();
        for p in &self.phases {
            out.add(p);
        }
        out
    }
}

type Cells = [[Cell<u64>; KINDS + 1]; PHASES];

thread_local! {
    // One cell per (phase, kind); index `KINDS` holds `rng_words`.
    static TALLY: Cells = const { [const { [const { Cell::new(0) }; KINDS + 1] }; PHASES] };
    static VALIDATION: Cells = const { [const { [const { Cell::new(0) }; KINDS + 1] }; PHASES] };
    static PHASE: Cell<usize> = const { Cell::new(0) };
    static VALIDATING: Cell<u32> = const { Cell::new(0) };
}

#[inline]
fn bump_slot(slot: usize, n: u64) {
    if n == 0 {
        return;
    }
    let phase = PHASE.with(Cell::get);
    TALLY.with(|t| {
        let c = &t[phase][slot];
        c.set(c.get() + n);
    });
    if VALIDATING.with(Cell::get) > 0 {
        VALIDATION.with(|t| {
            let c = &t[phase][slot];
            c.set(c.get() + n);
        });
    }
}

#[inline]
fn bump(kind: Kind, n: u64) {
    bump_slot(kind as usize, n);
}

fn read(cells: &Cells) -> PhaseTally {
    let mut out = PhaseTally::default();
    for (p, row) in cells.iter().enumerate() {
        for k in 0..KINDS {
            out.phases[p].counts[k] = row[k].get();
        }
        out.phases[p].rng_words = row[KINDS].get();
    }
    out
}

/// Charge `n` operations of `kind` to the current phase.
#[inline]
pub fn charge(kind: Kind, n: u64) {
    bump(kind, n);
}

#[inline]
pub fn fp_add(n: u64) {
    bump(Kind::FieldAdd, n);
}
#[inline]
pub fn fp_mul(n: u64) {
    bump(Kind::FieldMul, n);
}
#[inline]
pub fn fp_sqr(n: u64) {
    bump(Kind::FieldSqr, n);
}
#[inline]
pub fn fp_inv(n: u64) {
    bump(Kind::FieldInv, n);
}
#[inline]
pub fn fp_sqrt(n: u64) {
    bump(Kind::FieldSqrt, n);
}
#[inline]
pub fn legendre(n: u64) {
    bump(Kind::Legendre, n);
}
#[inline]
pub fn modr_add(n: u64) {
    bump(Kind::ModrAdd, n);
}
#[inline]
pub fn modr_mul(n: u64) {
    bump(Kind::ModrMul, n);
}
#[inline]
pub fn modr_inv(n: u64) {
    bump(Kind::ModrInv, n);
}
#[inline]
pub fn word_op(n: u64) {
    bump(Kind::WordOp, n);
}
#[inline]
pub fn hash_probe(n: u64) {
    bump(Kind::HashProbe, n);
}

/// One 64-bit RNG draw: one `word_op`, and recorded in `rng_words`.
#[inline]
pub fn rng_word(n: u64) {
    bump(Kind::WordOp, n);
    bump_slot(KINDS, n);
}

/// Comparisons of a sort of `k` keys: `k·⌈log₂ k⌉` word ops.
pub fn sort_cost(k: usize) -> u64 {
    if k < 2 {
        return 0;
    }
    let lg = usize::BITS - (k - 1).leading_zeros();
    (k as u64) * lg as u64
}

/// Charge a public-exponent `modpow` by `e` as left-to-right
/// square-and-multiply: `bits(e) − 1` squarings and `popcount(e) − 1`
/// products, in `F_p`.
pub fn charge_pow(e: &num_bigint::BigUint) {
    let bits = e.bits();
    if bits == 0 {
        return;
    }
    fp_sqr(bits - 1);
    fp_mul(e.count_ones().saturating_sub(1));
}

/// Words a hash of a divisor absorbs: one per 64-bit limb of each
/// coefficient, plus one terminator per polynomial.
pub fn divisor_hash_words(d: &super::MumfordDivisorP) -> u64 {
    let limbs = |p: &super::FpPoly| {
        p.coeffs
            .iter()
            .map(|c| c.to_u64_digits().len() as u64)
            .sum::<u64>()
            + 1
    };
    limbs(&d.u) + limbs(&d.v)
}

/// Everything this thread has tallied, by phase.
pub fn snapshot() -> PhaseTally {
    TALLY.with(read)
}

/// The validation subset of [`snapshot`].
pub fn snapshot_validation() -> PhaseTally {
    VALIDATION.with(read)
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

/// Marks the checked arithmetic's validation while alive.
pub struct ValidationGuard;

impl Drop for ValidationGuard {
    fn drop(&mut self) {
        VALIDATING.with(|v| v.set(v.get() - 1));
    }
}

/// Count work as validation (as well as in its phase) until the guard
/// drops.  Nests.
pub fn validating() -> ValidationGuard {
    VALIDATING.with(|v| v.set(v.get() + 1));
    ValidationGuard
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::bitops::Field;

    #[test]
    fn phases_route_and_restore() {
        let before = snapshot();
        {
            let _g = enter(Phase::Oracle);
            fp_mul(3);
            {
                let _h = enter(Phase::LinearAlgebra);
                modr_inv(1);
            }
            fp_add(2);
        }
        word_op(1);
        let d = snapshot().since(&before);
        assert_eq!(d.phase(Phase::Oracle).get(Kind::FieldMul), 3);
        assert_eq!(d.phase(Phase::Oracle).get(Kind::FieldAdd), 2);
        assert_eq!(d.phase(Phase::LinearAlgebra).get(Kind::ModrInv), 1);
        assert_eq!(d.phase(Phase::Unattributed).get(Kind::WordOp), 1);
    }

    #[test]
    fn pricing_matches_bitops() {
        let mut t = OpTally::default();
        t.counts[Kind::FieldMul as usize] = 2;
        t.counts[Kind::ModrAdd as usize] = 1;
        t.counts[Kind::HashProbe as usize] = 1;
        let w = Widths {
            field: Field::Prime { bits: 8 },
            r_bits: 12,
        };
        let want = 2 * w.gates(Prim::FieldMul).unwrap()
            + w.gates(Prim::ModrAdd).unwrap()
            + w.gates(Prim::HashProbe).unwrap();
        assert_eq!(t.bitops(&w).unwrap(), want);
    }

    /// One Cantor addition of two degree-1 genus-2 divisors over `F_41`,
    /// traced by hand through `cantor_compose_checked` and
    /// `reduce_checked` (no reduction step is needed: `deg u' = 2 = g`).
    ///
    /// `C : y² = x⁵ + 3x³ + 2x² + x + 1`, `D₁ = (x − 1, 7)`,
    /// `D₂ = (x − 3, 10)`.  Per step (inv / mul / sqr / add):
    ///
    /// ```text
    /// ext_gcd(u₁, u₂)          3 / 19 / 0 / 27   two divrems, q·r, q·s, q·t, 3 scalings
    /// v₁ + v₂                  0 /  0 / 0 /  1
    /// ext_gcd(1, v₁ + v₂)      2 /  6 / 0 /  7
    /// s₁ = c₁e₁, s₂ = c₁e₂     0 (c₁ = 0: the product returns early)
    /// u₁·u₂                    0 /  4 / 0 /  4
    /// d·d                      0 /  0 / 1 /  1   (d = 1, a squaring)
    /// u₁u₂ / d²                1 /  6 / 0 /  9   three quotient steps
    /// v₁v₂                     0 /  1 / 0 /  1
    /// s₃·(v₁v₂ + f)            0 /  5 / 0 / 11   add over 6 positions; x⁴ term is zero
    /// term₁ + term₂ + term₃    0 /  0 / 0 /  6
    /// v_num / d                1 / 10 / 0 / 21   five quotient steps (x⁴ absent)
    /// v mod u'                 1 / 16 / 0 / 22   four quotient steps
    /// total                    8 / 67 / 1 / 110
    /// ```
    ///
    /// Validation (each operand and the result re-checked, the curve's
    /// squarefreeness included) is counted separately; its total is pinned
    /// as a regression value, not hand-derived.
    #[test]
    fn one_cantor_addition_has_the_hand_counted_tally() {
        use super::super::{FpPoly, HyperellipticCurveP, MumfordDivisorP};
        use num_bigint::BigUint;
        let p = BigUint::from(41u32);
        let b = |v: u32| BigUint::from(v);
        let f = FpPoly::from_coeffs(vec![b(1), b(1), b(2), b(3), b(0), b(1)], p.clone());
        let curve = HyperellipticCurveP::try_new(p.clone(), f, 2).expect("squarefree");
        let d1 = MumfordDivisorP::try_from_point(&curve, &b(1), &b(7)).unwrap();
        let d2 = MumfordDivisorP::try_from_point(&curve, &b(3), &b(10)).unwrap();

        let (t0, v0) = (snapshot(), snapshot_validation());
        let sum = {
            let _g = enter(Phase::Walk);
            d1.add(&d2, &curve)
        };
        let all = snapshot().since(&t0).phase(Phase::Walk).to_owned();
        let valid = snapshot_validation()
            .since(&v0)
            .phase(Phase::Walk)
            .to_owned();
        let cantor = all.since(&valid);
        assert_eq!(sum.u.degree(), Some(2));
        assert_eq!(cantor.get(Kind::FieldInv), 8);
        assert_eq!(cantor.get(Kind::FieldMul), 67);
        assert_eq!(cantor.get(Kind::FieldSqr), 1);
        assert_eq!(cantor.get(Kind::FieldAdd), 110);
        assert_eq!(cantor.get(Kind::ModrAdd) + cantor.get(Kind::HashProbe), 0);
        // Validation of two inputs and the output: pinned.
        assert!(valid.field_ops() > 0);
        let pinned = [
            valid.get(Kind::FieldInv),
            valid.get(Kind::FieldMul),
            valid.get(Kind::FieldSqr),
            valid.get(Kind::FieldAdd),
        ];
        eprintln!("validation tally (inv, mul, sqr, add) = {pinned:?}");
        assert_eq!(pinned, VALIDATION_PIN);
    }

    const VALIDATION_PIN: [u64; 4] = [21, 165, 4, 213];

    #[test]
    fn sort_and_pow_charges() {
        assert_eq!(sort_cost(1), 0);
        assert_eq!(sort_cost(2), 2);
        assert_eq!(sort_cost(5), 15);
        let before = snapshot();
        charge_pow(&num_bigint::BigUint::from(0b1011u32));
        let d = snapshot().since(&before).total();
        assert_eq!(d.get(Kind::FieldSqr), 3);
        assert_eq!(d.get(Kind::FieldMul), 2);
    }
}
