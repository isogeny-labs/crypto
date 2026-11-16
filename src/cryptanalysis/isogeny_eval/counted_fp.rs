//! `F_p` for an odd prime `5 ≤ p < 2^63`, with optional operation counters.
//!
//! [`PrimeField`] is generic over an [`OpCounter`].  The default,
//! [`NoCount`], is a zero-sized type whose hooks are empty and inline away,
//! so ordinary callers pay nothing.  [`PrimeField::counted`] builds a field
//! that records every multiplication or squaring (`M`) and every inversion
//! (`I`) in an [`OpCounts`]; read it with [`PrimeField::tally`].
//!
//! The unit is the one of the round-4 measurements
//! (`research/isogeny_fibers_20261010/round4/16_3_vertical_margin`): `M`
//! counts every product in `F_p`, including each `a_i·b_j` inside polynomial
//! arithmetic (see [`super::poly`]); additions and subtractions are free unless
//! asked for (below);
//! `I` is one extended-Euclid inversion.  Products done by the NTT
//! ([`super::ntt`]) are counted as *word multiplications* (`W`, one
//! Montgomery product modulo a 62-bit NTT prime) plus the `M` of their CRT
//! reconstruction; [`OpTally::mul_equivalents`] converts `W` at
//! [`super::ntt::NTT_WORD_COST_IN_M`]` = 1` `M` each.
//!
//! **Additions (opt-in, round 7).**  A field built by
//! [`PrimeField::counted_with_adds`] additionally tallies additions and
//! subtractions in an [`AddTally`] (read with [`PrimeField::add_tally`] or
//! [`PrimeField::complete_tally`]); [`OpTally`] itself is unchanged.  One *addition*
//! is one modular addition, subtraction or negation of canonical residues
//! (the add/sub plus its conditional correction counts once), whether done
//! by [`PrimeField::add`], [`PrimeField::sub`], [`PrimeField::neg`] or fused
//! inside a polynomial product.  `add` is the count in `F_p` (including the
//! `n − 1` accumulations per schoolbook output coefficient, and the two
//! `u128` accumulations per output coefficient of the NTT's CRT
//! reconstruction); `word_add` is the count of additions and subtractions
//! modulo the NTT primes (butterflies, Garner differences, conditional
//! input reductions).  A plain [`PrimeField::counted`] field leaves both at
//! zero and its `M`, `W`, `I` are exactly those of the frozen round-4 to
//! round-6 measurements; [`OpTally::mul_equivalents`] never includes
//! additions.  [`CompleteTally::weighted`] prices them at `a·M` each.
//!
//! Which polynomial multiplication runs is a property of the field
//! ([`PolyMul`], default [`PolyMul::Auto`]); [`PrimeField::with_poly_mul`]
//! selects [`PolyMul::Karatsuba`] to reproduce the round-4 counts.
//!
//! Elements are plain `u64` values in canonical form `0 ≤ a < p`.  Every
//! method assumes canonical inputs (checked by `debug_assert!` only) and
//! returns canonical outputs.  Use [`PrimeField::elem`] or
//! [`PrimeField::from_i64`] to reduce arbitrary integers.

use std::cell::Cell;

/// Hooks called by [`PrimeField`] for every counted operation.
pub trait OpCounter {
    /// `n` multiplications or squarings in `F_p` happened.
    fn count_mul(&self, n: u64);
    /// One inversion in `F_p` happened.
    fn count_inv(&self);
    /// `n` word multiplications modulo an NTT prime happened (inside
    /// [`super::ntt::mul_ntt`]).  The default records them as `n` `M`
    /// (the stated conversion, [`super::ntt::NTT_WORD_COST_IN_M`]).
    #[inline]
    fn count_word_mul(&self, n: u64) {
        self.count_mul(n.saturating_mul(super::ntt::NTT_WORD_COST_IN_M));
    }
    /// `n` additions, subtractions or negations in `F_p` happened.  The
    /// default records nothing (additions are free unless a counter asks).
    #[inline]
    fn count_add(&self, _n: u64) {}
    /// `n` additions or subtractions modulo an NTT prime happened (inside
    /// [`super::ntt::mul_ntt`]).  The default records nothing.
    #[inline]
    fn count_word_add(&self, _n: u64) {}
}

/// How [`super::poly::mul`] multiplies polynomials over a [`PrimeField`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PolyMul {
    /// Karatsuba with schoolbook below
    /// [`super::poly::KARATSUBA_THRESHOLD`] (the round-4 arithmetic; its
    /// counts equal those of round-4 lead 16.3).
    Karatsuba,
    /// The three-prime NTT ([`super::ntt::mul_ntt`]) when the shorter
    /// operand has at least [`super::poly::NTT_THRESHOLD`] coefficients,
    /// Karatsuba otherwise.
    #[default]
    Auto,
    /// The NTT for every product (for tests and measurements).
    Ntt,
}

/// The counter that counts nothing (zero-sized, free).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NoCount;

impl OpCounter for NoCount {
    #[inline(always)]
    fn count_mul(&self, _n: u64) {}
    #[inline(always)]
    fn count_inv(&self) {}
    #[inline(always)]
    fn count_word_mul(&self, _n: u64) {}
    #[inline(always)]
    fn count_add(&self, _n: u64) {}
    #[inline(always)]
    fn count_word_add(&self, _n: u64) {}
}

/// Running multiplication and inversion counts (single-threaded, via
/// [`Cell`]), and, if asked for with [`OpCounts::with_adds`], additions.
#[derive(Clone, Debug, Default)]
pub struct OpCounts {
    mul: Cell<u64>,
    inv: Cell<u64>,
    word: Cell<u64>,
    add: Cell<u64>,
    word_add: Cell<u64>,
    /// Whether additions are tallied (off by default).
    adds_on: bool,
}

impl OpCounter for OpCounts {
    #[inline]
    fn count_mul(&self, n: u64) {
        self.mul.set(self.mul.get().wrapping_add(n));
    }
    #[inline]
    fn count_inv(&self) {
        self.inv.set(self.inv.get().wrapping_add(1));
    }
    #[inline]
    fn count_word_mul(&self, n: u64) {
        self.word.set(self.word.get().wrapping_add(n));
    }
    #[inline]
    fn count_add(&self, n: u64) {
        if self.adds_on {
            self.add.set(self.add.get().wrapping_add(n));
        }
    }
    #[inline]
    fn count_word_add(&self, n: u64) {
        if self.adds_on {
            self.word_add.set(self.word_add.get().wrapping_add(n));
        }
    }
}

impl OpCounts {
    /// Counters that also tally additions (see the module documentation).
    pub fn with_adds() -> Self {
        OpCounts {
            adds_on: true,
            ..Default::default()
        }
    }

    /// Whether additions are being tallied.
    pub fn counts_adds(&self) -> bool {
        self.adds_on
    }

    /// The counts so far.
    pub fn snapshot(&self) -> OpTally {
        OpTally {
            mul: self.mul.get(),
            inv: self.inv.get(),
            word: self.word.get(),
        }
    }

    /// The addition counts so far (zero unless built by
    /// [`OpCounts::with_adds`]).
    pub fn add_snapshot(&self) -> AddTally {
        AddTally {
            add: self.add.get(),
            word_add: self.word_add.get(),
        }
    }

    /// Set all counts to zero.
    pub fn reset(&self) {
        self.mul.set(0);
        self.inv.set(0);
        self.word.set(0);
        self.add.set(0);
        self.word_add.set(0);
    }
}

/// Frozen counts: `mul` multiplications or squarings and `inv` inversions
/// in `F_p`, `word` NTT word multiplications, and, only for a field built
/// with [`PrimeField::counted_with_adds`], `add` additions in `F_p` and
/// `word_add` additions modulo the NTT primes (zero otherwise).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OpTally {
    /// Multiplications and squarings in `F_p` (`M`).
    pub mul: u64,
    /// Inversions (`I`).
    pub inv: u64,
    /// Word multiplications modulo the NTT primes (`W`, zero unless the NTT
    /// ran).
    pub word: u64,
}

/// Frozen addition counts (see the module documentation): `add` additions,
/// subtractions and negations in `F_p`, `word_add` additions and
/// subtractions modulo the NTT primes.  Both are zero unless the field was
/// built with [`PrimeField::counted_with_adds`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AddTally {
    /// `A`: additions, subtractions and negations in `F_p`.
    pub add: u64,
    /// `WA`: additions and subtractions modulo the NTT primes.
    pub word_add: u64,
}

impl AddTally {
    /// The additions done since `earlier` (saturating at zero).
    pub fn since(self, earlier: AddTally) -> AddTally {
        AddTally {
            add: self.add.saturating_sub(earlier.add),
            word_add: self.word_add.saturating_sub(earlier.word_add),
        }
    }

    /// `A + WA`: every counted addition, each of the same weight.
    pub fn total(&self) -> u64 {
        self.add.saturating_add(self.word_add)
    }
}

/// [`OpTally`] and [`AddTally`] together.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompleteTally {
    /// `M`, `W`, `I` (unchanged by whether additions are counted).
    pub ops: OpTally,
    /// `A`, `WA`.
    pub adds: AddTally,
}

impl CompleteTally {
    /// The operations done since `earlier`.
    pub fn since(self, earlier: CompleteTally) -> CompleteTally {
        CompleteTally {
            ops: self.ops.since(earlier.ops),
            adds: self.adds.since(earlier.adds),
        }
    }

    /// `Meq + a·(A + WA)`: the multiplication equivalents with every
    /// addition priced at `a` `M` (`a = 0` gives
    /// [`OpTally::mul_equivalents`]).
    pub fn weighted(&self, inv_cost: u64, a: f64) -> f64 {
        self.ops.mul_equivalents(inv_cost) as f64 + a * self.adds.total() as f64
    }
}

impl OpTally {
    /// The operations done since `earlier` (saturating at zero).
    pub fn since(self, earlier: OpTally) -> OpTally {
        OpTally {
            mul: self.mul.saturating_sub(earlier.mul),
            inv: self.inv.saturating_sub(earlier.inv),
            word: self.word.saturating_sub(earlier.word),
        }
    }

    /// `M + W·NTT_WORD_COST_IN_M + inv_cost·I`: one number in
    /// multiplication equivalents (`W` at one `M` each, see
    /// [`super::ntt`]).  The round-4 reports used `inv_cost = 91` (the
    /// Fermat cost at 61 bits) and had `W = 0`.
    pub fn mul_equivalents(&self, inv_cost: u64) -> u64 {
        self.mul
            .saturating_add(self.word.saturating_mul(super::ntt::NTT_WORD_COST_IN_M))
            .saturating_add(inv_cost.saturating_mul(self.inv))
    }
}

/// The prime field `F_p`, `5 ≤ p < 2^63`, with counter `C`.
///
/// Cloning a counted field clones its current counts; the clone then counts
/// separately.
#[derive(Clone, Debug)]
pub struct PrimeField<C = NoCount> {
    p: u64,
    /// How many products `< (p−1)²` a `u128` accumulator holding a value
    /// `< p` can absorb before it must be reduced (at least 4 for `p < 2^63`).
    fold: usize,
    counter: C,
    poly_mul: PolyMul,
}

/// Deterministic Miller–Rabin for every `u64` (the first twelve primes as
/// bases).
pub fn is_prime_u64(n: u64) -> bool {
    const BASES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    if n < 2 {
        return false;
    }
    for sp in BASES {
        if n.is_multiple_of(sp) {
            return n == sp;
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d.is_multiple_of(2) {
        d /= 2;
        s += 1;
    }
    let mulm = |a: u64, b: u64| ((a as u128 * b as u128) % n as u128) as u64;
    let powm = |mut a: u64, mut e: u64| {
        let mut r = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                r = mulm(r, a);
            }
            a = mulm(a, a);
            e >>= 1;
        }
        r
    };
    'witness: for a in BASES {
        let mut x = powm(a, d);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulm(x, x);
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}

impl PrimeField<NoCount> {
    /// `F_p` without counting.  `None` unless `p` is a prime with
    /// `5 ≤ p < 2^63`.
    pub fn new(p: u64) -> Option<Self> {
        Self::with_counter(p, NoCount)
    }
}

impl PrimeField<OpCounts> {
    /// `F_p` that counts multiplications and inversions, starting from zero.
    /// `None` unless `p` is a prime with `5 ≤ p < 2^63`.
    pub fn counted(p: u64) -> Option<Self> {
        Self::with_counter(p, OpCounts::default())
    }

    /// Like [`PrimeField::counted`], and additionally tallies additions
    /// ([`AddTally`]).  `M`, `W` and `I` are the
    /// same as with [`PrimeField::counted`] for the same computation.
    pub fn counted_with_adds(p: u64) -> Option<Self> {
        Self::with_counter(p, OpCounts::with_adds())
    }

    /// The counts so far.
    pub fn tally(&self) -> OpTally {
        self.counter.snapshot()
    }

    /// The addition counts so far (all zero unless the field was built by
    /// [`PrimeField::counted_with_adds`]).
    pub fn add_tally(&self) -> AddTally {
        self.counter.add_snapshot()
    }

    /// [`PrimeField::tally`] and [`PrimeField::add_tally`] together.
    pub fn complete_tally(&self) -> CompleteTally {
        CompleteTally {
            ops: self.tally(),
            adds: self.add_tally(),
        }
    }

    /// Reset the counts (including the addition counts) to zero.
    pub fn reset_counts(&self) {
        self.counter.reset();
    }
}

impl<C: OpCounter> PrimeField<C> {
    /// `F_p` with an explicit counter.  `None` unless `p` is a prime with
    /// `5 ≤ p < 2^63`.
    pub fn with_counter(p: u64, counter: C) -> Option<Self> {
        if !(5..1u64 << 63).contains(&p) || !is_prime_u64(p) {
            return None;
        }
        let pm1 = (p - 1) as u128;
        let fold = ((u128::MAX - p as u128) / (pm1 * pm1)).min(1 << 20) as usize;
        Some(PrimeField {
            p,
            fold,
            counter,
            poly_mul: PolyMul::default(),
        })
    }

    /// The same field with polynomial multiplication `m` (see [`PolyMul`]).
    pub fn with_poly_mul(mut self, m: PolyMul) -> Self {
        self.poly_mul = m;
        self
    }

    /// The polynomial multiplication [`super::poly::mul`] uses.
    #[inline]
    pub fn poly_mul(&self) -> PolyMul {
        self.poly_mul
    }

    /// The modulus `p`.
    #[inline]
    pub fn p(&self) -> u64 {
        self.p
    }

    /// The counter.
    pub fn counter(&self) -> &C {
        &self.counter
    }

    /// Record `n` multiplications done outside [`PrimeField::mul`] (for
    /// example the fused products of schoolbook polynomial multiplication).
    #[inline]
    pub fn count_muls(&self, n: u64) {
        self.counter.count_mul(n);
    }

    /// Record `n` additions done outside [`PrimeField::add`] and
    /// [`PrimeField::sub`] (for example the accumulations fused into
    /// schoolbook polynomial multiplication).  Free unless additions are
    /// counted.
    #[inline]
    pub fn count_adds(&self, n: u64) {
        self.counter.count_add(n);
    }

    /// Products a `u128` accumulator can absorb between reductions.
    #[inline]
    pub(crate) fn fold(&self) -> usize {
        self.fold
    }

    /// `v mod p`.
    #[inline]
    pub fn elem(&self, v: u64) -> u64 {
        v % self.p
    }

    /// `v mod p` for a signed `v`.
    #[inline]
    pub fn from_i64(&self, v: i64) -> u64 {
        (v as i128).rem_euclid(self.p as i128) as u64
    }

    /// `v mod p` for a `u128` (uncounted reduction).
    #[inline]
    pub fn reduce_u128(&self, v: u128) -> u64 {
        (v % self.p as u128) as u64
    }

    /// `a + b`.
    #[inline]
    pub fn add(&self, a: u64, b: u64) -> u64 {
        debug_assert!(a < self.p && b < self.p);
        self.counter.count_add(1);
        let s = a + b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }

    /// `a − b`.
    #[inline]
    pub fn sub(&self, a: u64, b: u64) -> u64 {
        debug_assert!(a < self.p && b < self.p);
        self.counter.count_add(1);
        if a >= b {
            a - b
        } else {
            a + (self.p - b)
        }
    }

    /// `−a`.
    #[inline]
    pub fn neg(&self, a: u64) -> u64 {
        debug_assert!(a < self.p);
        self.counter.count_add(1);
        if a == 0 {
            0
        } else {
            self.p - a
        }
    }

    /// `a·b` (one `M`).
    #[inline]
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        debug_assert!(a < self.p && b < self.p);
        self.counter.count_mul(1);
        ((a as u128 * b as u128) % self.p as u128) as u64
    }

    /// `a²` (one `M`).
    #[inline]
    pub fn sqr(&self, a: u64) -> u64 {
        self.mul(a, a)
    }

    /// `c·a` for a small integer constant `c` (counted as one `M`, as the
    /// round-4 code did).
    #[inline]
    pub fn mul_small(&self, c: u64, a: u64) -> u64 {
        self.mul(self.elem(c), a)
    }

    /// `a^{−1}` (one `I`), or `None` for `a = 0`.
    pub fn inv(&self, a: u64) -> Option<u64> {
        debug_assert!(a < self.p);
        if a == 0 {
            return None;
        }
        self.counter.count_inv();
        let (mut r0, mut r1) = (self.p as i128, a as i128);
        let (mut s0, mut s1) = (0i128, 1i128);
        while r1 != 0 {
            let q = r0 / r1;
            (r0, r1) = (r1, r0 - q * r1);
            (s0, s1) = (s1, s0 - q * s1);
        }
        Some(s0.rem_euclid(self.p as i128) as u64)
    }

    /// `a / b`, or `None` for `b = 0`.
    pub fn div(&self, a: u64, b: u64) -> Option<u64> {
        Some(self.mul(a, self.inv(b)?))
    }

    /// `a^e` by right-to-left square-and-multiply (`0^0 = 1`); like the
    /// round-4 code it squares once per exponent bit, including the last.
    pub fn pow(&self, a: u64, mut e: u128) -> u64 {
        let mut r = 1u64;
        let mut b = a;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, b);
            }
            b = self.sqr(b);
            e >>= 1;
        }
        r
    }

    /// The Legendre symbol `(a/p) ∈ {−1, 0, 1}`.
    pub fn legendre(&self, a: u64) -> i32 {
        if a == 0 {
            return 0;
        }
        if self.pow(a, ((self.p - 1) / 2) as u128) == 1 {
            1
        } else {
            -1
        }
    }

    /// A square root of `a` (Tonelli–Shanks), or `None` if `a` is a
    /// non-residue.  Which of the two roots is returned is deterministic.
    pub fn sqrt(&self, a: u64) -> Option<u64> {
        if a == 0 {
            return Some(0);
        }
        if self.legendre(a) != 1 {
            return None;
        }
        let mut q = self.p - 1;
        let mut s = 0u32;
        while q.is_multiple_of(2) {
            q /= 2;
            s += 1;
        }
        let mut z = 2u64;
        while self.legendre(z) != -1 {
            z += 1;
        }
        let mut m = s;
        let mut c = self.pow(z, q as u128);
        let mut t = self.pow(a, q as u128);
        let mut r = self.pow(a, q.div_ceil(2) as u128);
        while t != 1 {
            let mut i = 0u32;
            let mut tt = t;
            while tt != 1 {
                tt = self.sqr(tt);
                i += 1;
            }
            let mut b = c;
            for _ in 0..(m - i - 1) {
                b = self.sqr(b);
            }
            m = i;
            c = self.sqr(b);
            t = self.mul(t, c);
            r = self.mul(r, b);
        }
        Some(r)
    }
}
