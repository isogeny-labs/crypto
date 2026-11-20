//! Native-width prime-field ECDLP engine — the speed-up prototype for
//! the prime regime of the index-calculus ladder.
//!
//! Every prime-field rho and index-calculus path in this crate runs on
//! heap-allocated `BigUint` through [`crate::ecc::field::FieldElement`],
//! whose inverse is a Fermat exponentiation on a fixed-iteration ladder
//! (about `2·log₂p` big-integer multiplications per inverse) and whose
//! affine point addition pays one such inverse per step.  This module
//! replaces that stack for `p < 2^62` with:
//!
//! 1. **Montgomery arithmetic on `u64`** ([`Fp64`]): one `u128` product
//!    and one reduction per multiplication, no allocation, and a
//!    binary extended-Euclid inverse.
//! 2. **Simultaneous inversion** (Montgomery's trick, [`Fp64::batch_inv`]):
//!    `W` affine additions share one inverse at the price of `3(W−1)`
//!    multiplications.
//! 3. **Parallel distinguished-point Pollard rho** ([`rho_parallel`]):
//!    `r`-adding walk with a 1024-entry table, the negation map with a
//!    deterministic fruitless-2-cycle escape, lock-step walkers per
//!    thread so every step is batched, and a shared DP table.
//! 4. **Index-calculus relation collection without summation
//!    polynomials** ([`collect_relations`]): the Semaev `S₃` root
//!    `X` of `S₃(x_R, x_i, X) = 0` *is* `x(R ± F_i)`, so a
//!    2-decomposition sweep is one batched subtraction per factor-base
//!    element plus an array lookup — no square root, no inversion per
//!    element.  The `S₄` quartic roots are likewise `x(R ∓ F_i)` matched
//!    against a precomputed table of `x(F_j ± F_k)` ([`PairTable`]),
//!    turning the `B²/2` root-findings per target into `2B` lookups.
//! 5. **Linear algebra on `u64`** mod the group order (Montgomery form,
//!    Gauss–Jordan) and a **Hasse-interval BSGS point counter**
//!    ([`point_order_hasse`]) so the `a = −3` bench ladder can be
//!    extended past the `u64` brute-force counter's reach.
//!
//! The relation sets produced by (4) are *identical* to the ones the
//! `S₃` / `S₄` sweeps in [`crate::cryptanalysis::ec_index_calculus`]
//! produce for the same factor base (the summation polynomial vanishes
//! exactly when the x-coordinates come from points summing to zero), so
//! the staged timers remain comparable; only the cost per target
//! changes.  Everything is public-synthetic known-answer only, and every
//! recovered logarithm is re-checked in the group before it is returned.
//!
//! The benchmark driver is `examples/prime_ecdlp_fast_bench.rs`.

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::cryptanalysis::residual_walk::is_prime_u64;
use crate::ecc::curve::CurveParams;
use crate::ecc::field::FieldElement;
use crate::ecc::point::Point;

/// Largest field size the Montgomery layer supports (`p < 2^62` keeps
/// `a + b` and `t + m·p` inside `u64` / `u128`).
pub const MAX_BITS: u32 = 62;

// ── Montgomery F_p on u64 ─────────────────────────────────────────────

/// Montgomery field context for an odd prime `p < 2^62`, `R = 2^64`.
/// Values are held as `a·R mod p`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fp64 {
    pub p: u64,
    /// `−p⁻¹ mod 2^64`.
    n_inv: u64,
    /// `R mod p` — the Montgomery form of 1.
    pub one: u64,
    /// `R² mod p`.
    r2: u64,
    /// `R³ mod p` (turns a canonical inverse of a Montgomery value back
    /// into Montgomery form in one multiplication).
    r3: u64,
    /// `(p − 1) / 2`, the sign boundary for the negation map.
    pub half: u64,
}

impl Fp64 {
    pub fn new(p: u64) -> Self {
        assert!(p & 1 == 1 && p > 3, "Fp64 needs an odd prime > 3");
        assert!(p < (1u64 << MAX_BITS), "Fp64 supports p < 2^{MAX_BITS}");
        // Newton iteration for p⁻¹ mod 2^64 (each step doubles the
        // number of correct bits; 6 steps from 1 correct bit is enough).
        let mut inv: u64 = 1;
        for _ in 0..6 {
            inv = inv.wrapping_mul(2u64.wrapping_sub(p.wrapping_mul(inv)));
        }
        let n_inv = inv.wrapping_neg();
        let r = ((1u128 << 64) % p as u128) as u64;
        let r2 = ((r as u128 * r as u128) % p as u128) as u64;
        let mut f = Fp64 {
            p,
            n_inv,
            one: r,
            r2,
            r3: 0,
            half: (p - 1) / 2,
        };
        f.r3 = f.mul(r2, r2);
        f
    }

    /// `u mod p` for `u < 2p`.  A select, not a branch: on field data the
    /// comparison is a coin flip a branch predictor cannot learn (see
    /// [`crate::cryptanalysis::fp_backend::Fp64`] and
    /// `research/hwcost_round2_20261016`, addendum 1).
    #[inline(always)]
    fn csub(&self, u: u64) -> u64 {
        let (d, borrow) = u.overflowing_sub(self.p);
        std::hint::select_unpredictable(borrow, u, d)
    }

    /// Montgomery reduction of `t < p·2^64`: returns `t·R⁻¹ mod p`.
    #[inline(always)]
    pub fn reduce(&self, t: u128) -> u64 {
        let m = (t as u64).wrapping_mul(self.n_inv);
        let u = ((t + (m as u128) * (self.p as u128)) >> 64) as u64;
        self.csub(u)
    }

    #[inline(always)]
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        self.reduce(a as u128 * b as u128)
    }

    #[inline(always)]
    pub fn sqr(&self, a: u64) -> u64 {
        self.mul(a, a)
    }

    #[inline(always)]
    pub fn add(&self, a: u64, b: u64) -> u64 {
        self.csub(a + b)
    }

    #[inline(always)]
    pub fn sub(&self, a: u64, b: u64) -> u64 {
        let (d, borrow) = a.overflowing_sub(b);
        std::hint::select_unpredictable(borrow, d.wrapping_add(self.p), d)
    }

    #[inline(always)]
    pub fn neg(&self, a: u64) -> u64 {
        // a == 0 is rare on data, so this branch predicts: kept.
        if a == 0 {
            0
        } else {
            self.p - a
        }
    }

    /// Canonical → Montgomery.
    #[inline]
    pub fn to_mont(&self, a: u64) -> u64 {
        self.mul(a % self.p, self.r2)
    }

    /// Montgomery → canonical.
    #[inline(always)]
    pub fn from_mont(&self, a: u64) -> u64 {
        self.reduce(a as u128)
    }

    /// Inverse of a non-zero Montgomery value, in Montgomery form.
    #[inline]
    pub fn inv(&self, a: u64) -> u64 {
        // inv_mod gives (a·R)⁻¹ = a⁻¹·R⁻¹; times R³ through one
        // Montgomery multiplication (which divides by R) is a⁻¹·R.
        let c = inv_mod_u64(a, self.p);
        self.mul(c, self.r3)
    }

    /// `a^e` for a Montgomery value `a` (result in Montgomery form).
    pub fn pow(&self, mut a: u64, mut e: u64) -> u64 {
        let mut acc = self.one;
        while e > 0 {
            if e & 1 == 1 {
                acc = self.mul(acc, a);
            }
            a = self.sqr(a);
            e >>= 1;
        }
        acc
    }

    /// Montgomery's trick: invert every (non-zero) element of `xs` in
    /// place with a single field inversion and `3(k − 1)` multiplications.
    pub fn batch_inv(&self, xs: &mut [u64], scratch: &mut Vec<u64>) {
        let k = xs.len();
        if k == 0 {
            return;
        }
        static INTERLEAVE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        if k >= 32 && *INTERLEAVE.get_or_init(|| std::env::var_os("NOINTERLEAVE").is_none()) {
            return self.batch_inv_x4(xs, scratch);
        }
        scratch.clear();
        scratch.reserve(k);
        let mut acc = self.one;
        for &x in xs.iter() {
            scratch.push(acc);
            acc = self.mul(acc, x);
        }
        let mut inv_acc = self.inv(acc);
        for i in (0..k).rev() {
            let xi = xs[i];
            xs[i] = self.mul(inv_acc, scratch[i]);
            inv_acc = self.mul(inv_acc, xi);
        }
    }

    /// Montgomery's trick over four interleaved chains: the prefix-product
    /// and back-substitution chains are latency-bound, so four independent
    /// chains keep the multiplier busy. One inversion, same exact results.
    fn batch_inv_x4(&self, xs: &mut [u64], scratch: &mut Vec<u64>) {
        let k = xs.len();
        scratch.clear();
        scratch.resize(k, 0);
        let base = k / 4;
        let rem = k % 4;
        let mut start = [0usize; 4];
        let mut s = 0usize;
        for (c, st) in start.iter_mut().enumerate() {
            *st = s;
            s += base + usize::from(c < rem);
        }
        let mut acc = [self.one; 4];
        for t in 0..base {
            for c in 0..4 {
                let i = start[c] + t;
                scratch[i] = acc[c];
                acc[c] = self.mul(acc[c], xs[i]);
            }
        }
        for c in 0..rem {
            let i = start[c] + base;
            scratch[i] = acc[c];
            acc[c] = self.mul(acc[c], xs[i]);
        }
        let p01 = self.mul(acc[0], acc[1]);
        let p012 = self.mul(p01, acc[2]);
        let p0123 = self.mul(p012, acc[3]);
        let mut inv = self.inv(p0123);
        let i3 = self.mul(inv, p012);
        inv = self.mul(inv, acc[3]);
        let i2 = self.mul(inv, p01);
        inv = self.mul(inv, acc[2]);
        let i1 = self.mul(inv, acc[0]);
        let i0 = self.mul(inv, acc[1]);
        let mut ia = [i0, i1, i2, i3];
        for c in 0..rem {
            let i = start[c] + base;
            let xi = xs[i];
            xs[i] = self.mul(ia[c], scratch[i]);
            ia[c] = self.mul(ia[c], xi);
        }
        for t in (0..base).rev() {
            for c in 0..4 {
                let i = start[c] + t;
                let xi = xs[i];
                xs[i] = self.mul(ia[c], scratch[i]);
                ia[c] = self.mul(ia[c], xi);
            }
        }
    }

    /// Square root of a Montgomery value (Tonelli–Shanks), in Montgomery
    /// form.  `None` if `a` is a non-residue.
    pub fn sqrt(&self, a: u64) -> Option<u64> {
        if a == 0 {
            return Some(0);
        }
        let p = self.p;
        if self.pow(a, (p - 1) / 2) != self.one {
            return None;
        }
        if p % 4 == 3 {
            return Some(self.pow(a, (p + 1) / 4));
        }
        let mut q = p - 1;
        let mut s = 0u32;
        while q & 1 == 0 {
            q >>= 1;
            s += 1;
        }
        let minus_one = self.neg(self.one);
        let mut z = 2u64;
        while self.pow(self.to_mont(z), (p - 1) / 2) != minus_one {
            z += 1;
        }
        let mut m = s;
        let mut c = self.pow(self.to_mont(z), q);
        let mut t = self.pow(a, q);
        let mut r = self.pow(a, q.div_ceil(2));
        loop {
            if t == self.one {
                return Some(r);
            }
            let mut i = 0u32;
            let mut tt = t;
            while tt != self.one {
                tt = self.sqr(tt);
                i += 1;
                if i == m {
                    return None;
                }
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
    }
}

/// `a⁻¹ mod p` for odd `p < 2^63` and `0 < a < p`, by the binary
/// extended Euclidean algorithm (HAC 14.61): no divisions.
pub fn inv_mod_u64(a: u64, p: u64) -> u64 {
    debug_assert!(a != 0 && a < p && p & 1 == 1);
    let (mut u, mut v) = (a, p);
    // Invariants: x1·a ≡ u, x2·a ≡ v (mod p).
    let (mut x1, mut x2) = (1u64, 0u64);
    #[inline(always)]
    fn halve(x: u64, p: u64) -> u64 {
        if x & 1 == 0 {
            x >> 1
        } else {
            ((x as u128 + p as u128) >> 1) as u64
        }
    }
    while u != 1 && v != 1 {
        while u & 1 == 0 {
            u >>= 1;
            x1 = halve(x1, p);
        }
        while v & 1 == 0 {
            v >>= 1;
            x2 = halve(x2, p);
        }
        if u >= v {
            u -= v;
            x1 = if x1 >= x2 { x1 - x2 } else { x1 + p - x2 };
        } else {
            v -= u;
            x2 = if x2 >= x1 { x2 - x1 } else { x2 + p - x1 };
        }
    }
    if u == 1 {
        x1
    } else {
        x2
    }
}

/// `a·b mod m` for a fixed modulus, with the value of
/// `((a as u128 * b as u128) % m as u128) as u64` for every input.
///
/// For an odd `m < 2^63` and reduced operands the product goes through two
/// Montgomery reductions (`a·b·R⁻¹`, then `·R²·R⁻¹`), which return the
/// canonical remainder in `[0, m)` — the same value the 128-bit `%`
/// (`__umodti3`, a library call) returns, without a division.  Anything
/// else (even modulus, `m ≥ 2^63`, unreduced operand) takes the 128-bit
/// path unchanged, so no input changes its result.  Replay class WALL:
/// callers that swap their `u128 %` for this keep every value and every
/// counted unit (assessment 2026-11-10, R1/R2).
#[derive(Clone, Copy, Debug)]
pub struct MulModN {
    pub m: u64,
    /// `−m⁻¹ mod 2^64` (0 when the Montgomery path is off).
    n_inv: u64,
    /// `2^128 mod m`.
    r2: u64,
    mont: bool,
}

impl MulModN {
    pub fn new(m: u64) -> Self {
        let mont = m & 1 == 1 && m > 1 && m < (1u64 << 63);
        if !mont {
            return MulModN {
                m,
                n_inv: 0,
                r2: 0,
                mont,
            };
        }
        let mut inv: u64 = 1;
        for _ in 0..6 {
            inv = inv.wrapping_mul(2u64.wrapping_sub(m.wrapping_mul(inv)));
        }
        let r = ((1u128 << 64) % m as u128) as u64;
        let r2 = ((r as u128 * r as u128) % m as u128) as u64;
        MulModN {
            m,
            n_inv: inv.wrapping_neg(),
            r2,
            mont,
        }
    }

    /// Montgomery reduction of `t < m·2^64`: `t·2^-64 mod m` in `[0, m)`.
    #[inline(always)]
    fn redc(&self, t: u128) -> u64 {
        let q = (t as u64).wrapping_mul(self.n_inv);
        // t + q·m < 2m·2^64 ≤ 2^128 for m < 2^63, and the quotient by 2^64
        // is below 2m, so one conditional subtraction makes it canonical.
        let u = ((t + q as u128 * self.m as u128) >> 64) as u64;
        // A select, not a branch: on field data the comparison is a coin
        // flip a branch predictor cannot learn.
        let (d, borrow) = u.overflowing_sub(self.m);
        std::hint::select_unpredictable(borrow, u, d)
    }

    #[inline(always)]
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        if self.mont && a < self.m && b < self.m {
            self.redc(self.redc(a as u128 * b as u128) as u128 * self.r2 as u128)
        } else {
            ((a as u128 * b as u128) % self.m as u128) as u64
        }
    }
}

#[inline(always)]
fn add_mod(a: u64, b: u64, n: u64) -> u64 {
    let s = a + b;
    if s >= n {
        s - n
    } else {
        s
    }
}

#[inline(always)]
fn neg_mod(a: u64, n: u64) -> u64 {
    if a == 0 {
        0
    } else {
        n - a
    }
}

// ── Curve and points ──────────────────────────────────────────────────

/// Finite affine point with Montgomery-form coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Pt {
    pub x: u64,
    pub y: u64,
}

/// A Jacobian point `(X : Y : Z)`, `x = X/Z²`, `y = Y/Z³`, Montgomery
/// coordinates; `Z = 0` is `O`.  Only the scalar multiplications use it.
#[derive(Clone, Copy, Debug)]
struct Jac {
    x: u64,
    y: u64,
    z: u64,
}

const JAC_INFINITY: Jac = Jac { x: 0, y: 0, z: 0 };

/// Short-Weierstrass curve `y² = x³ + ax + b` over `F_p`, `p < 2^62`,
/// with a prime-order generator subgroup of order `n`.
#[derive(Clone, Debug)]
pub struct FastCurve {
    pub f: Fp64,
    /// `a`, `b` in Montgomery form.
    pub a: u64,
    pub b: u64,
    /// Subgroup order (prime, `< 2^62`).
    pub n: u64,
    pub name: String,
}

/// Reused storage for simultaneous additions from independent walk lanes.
/// Each active ordinary lane contributes one denominator; exceptional lanes
/// use the ordinary group law after the shared inversion.
#[derive(Default)]
pub struct BatchAddScratch {
    denominators: Vec<u64>,
    prefix: Vec<u64>,
    ordinary: Vec<Option<usize>>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BatchAddStats {
    pub ordinary: usize,
    pub exceptional: usize,
    pub inversions: usize,
}

impl FastCurve {
    pub fn new(name: &str, p: u64, a: u64, b: u64, n: u64) -> Self {
        let f = Fp64::new(p);
        FastCurve {
            f,
            a: f.to_mont(a),
            b: f.to_mont(b),
            n,
            name: name.to_string(),
        }
    }

    /// Adopt a crate [`CurveParams`] whose `p` and `n` fit in 62 bits.
    pub fn from_params(c: &CurveParams) -> Option<Self> {
        let p = biguint_to_u64(&c.p)?;
        let n = biguint_to_u64(&c.n)?;
        if p >= (1u64 << MAX_BITS) || n >= (1u64 << MAX_BITS) {
            return None;
        }
        Some(Self::new(
            c.name,
            p,
            biguint_to_u64(&c.a)?,
            biguint_to_u64(&c.b)?,
            n,
        ))
    }

    /// Group-order field context (Montgomery mod `n`) for the linear
    /// algebra.
    pub fn scalar_field(&self) -> Fp64 {
        Fp64::new(self.n)
    }

    pub fn point(&self, x: u64, y: u64) -> Pt {
        Pt {
            x: self.f.to_mont(x),
            y: self.f.to_mont(y),
        }
    }

    pub fn canonical(&self, p: Pt) -> (u64, u64) {
        (self.f.from_mont(p.x), self.f.from_mont(p.y))
    }

    pub fn to_textbook(&self, p: Option<Pt>) -> Point {
        match p {
            None => Point::Infinity,
            Some(pt) => {
                let (x, y) = self.canonical(pt);
                let m = BigUint::from(self.f.p);
                Point::Affine {
                    x: FieldElement::new(BigUint::from(x), m.clone()),
                    y: FieldElement::new(BigUint::from(y), m),
                }
            }
        }
    }

    pub fn from_textbook(&self, p: &Point) -> Option<Pt> {
        match p {
            Point::Infinity => None,
            Point::Affine { x, y } => Some(self.point(
                biguint_to_u64(&x.value).expect("coordinate fits u64"),
                biguint_to_u64(&y.value).expect("coordinate fits u64"),
            )),
        }
    }

    pub fn is_on_curve(&self, p: Pt) -> bool {
        let f = &self.f;
        let lhs = f.sqr(p.y);
        let rhs = f.add(f.add(f.mul(f.sqr(p.x), p.x), f.mul(self.a, p.x)), self.b);
        lhs == rhs
    }

    /// `p1 + p2` for `x1 ≠ x2`, given `inv_dx = (x2 − x1)⁻¹`.
    #[inline(always)]
    pub fn add_with_inv(&self, p1: Pt, p2: Pt, inv_dx: u64) -> Pt {
        let f = &self.f;
        let lam = f.mul(f.sub(p2.y, p1.y), inv_dx);
        let x3 = f.sub(f.sub(f.sqr(lam), p1.x), p2.x);
        let y3 = f.sub(f.mul(lam, f.sub(p1.x, x3)), p1.y);
        Pt { x: x3, y: y3 }
    }

    /// Montgomery x of `p1 + p2` for `x1 != x2` given `inv_dx = 1 / (x2 - x1)`; skips the y coordinate.
    #[inline]
    pub fn add_x_with_inv(&self, p1: Pt, p2: Pt, inv_dx: u64) -> u64 {
        let f = &self.f;
        let lam = f.mul(f.sub(p2.y, p1.y), inv_dx);
        f.sub(f.sub(f.sqr(lam), p1.x), p2.x)
    }

    #[inline]
    pub fn neg(&self, p: Pt) -> Pt {
        Pt {
            x: p.x,
            y: self.f.neg(p.y),
        }
    }

    pub fn double(&self, p: Pt) -> Option<Pt> {
        if p.y == 0 {
            return None;
        }
        let f = &self.f;
        let x2 = f.sqr(p.x);
        let num = f.add(f.add(f.add(x2, x2), x2), self.a);
        let den = f.inv(f.add(p.y, p.y));
        let lam = f.mul(num, den);
        let x3 = f.sub(f.sqr(lam), f.add(p.x, p.x));
        let y3 = f.sub(f.mul(lam, f.sub(p.x, x3)), p.y);
        Some(Pt { x: x3, y: y3 })
    }

    /// General addition with the point at infinity as `None`.
    pub fn add(&self, p1: Option<Pt>, p2: Option<Pt>) -> Option<Pt> {
        match (p1, p2) {
            (None, q) | (q, None) => q,
            (Some(a), Some(b)) => {
                if a.x == b.x {
                    if a.y == b.y {
                        self.double(a)
                    } else {
                        None
                    }
                } else {
                    let inv = self.f.inv(self.f.sub(b.x, a.x));
                    Some(self.add_with_inv(a, b, inv))
                }
            }
        }
    }

    /// Advance independent affine walk lanes with one Montgomery inversion
    /// across their ordinary additions.  Lane order and each output agree
    /// exactly with calling `add` separately.  The caller owns lane fill,
    /// drain, jump selection, coefficients and target verification.
    pub fn add_many_batched(
        &self,
        left: &mut [Option<Pt>],
        right: &[Option<Pt>],
        scratch: &mut BatchAddScratch,
    ) -> BatchAddStats {
        assert_eq!(left.len(), right.len());
        let n = left.len();
        scratch.denominators.clear();
        scratch.ordinary.clear();
        scratch.denominators.reserve(n);
        scratch.ordinary.reserve(n);
        let mut stats = BatchAddStats::default();
        for (&a, &b) in left.iter().zip(right) {
            if let (Some(p), Some(q)) = (a, b) {
                if p.x != q.x {
                    let index = scratch.denominators.len();
                    scratch.denominators.push(self.f.sub(q.x, p.x));
                    scratch.ordinary.push(Some(index));
                    stats.ordinary += 1;
                    continue;
                }
            }
            scratch.ordinary.push(None);
            stats.exceptional += 1;
        }
        if stats.ordinary > 0 {
            self.f
                .batch_inv(&mut scratch.denominators, &mut scratch.prefix);
            stats.inversions = 1;
        }
        for i in 0..n {
            left[i] = if let Some(index) = scratch.ordinary[i] {
                Some(self.add_with_inv(
                    left[i].expect("ordinary lane"),
                    right[i].expect("ordinary lane"),
                    scratch.denominators[index],
                ))
            } else {
                self.add(left[i], right[i])
            };
        }
        stats
    }

    /// `[k]P` by double-and-add (variable time; everything here is public).
    ///
    /// The running sum is kept in Jacobian coordinates, so the whole
    /// multiplication pays one inversion instead of one per group
    /// operation (about 93 at 62 bits).  The result is the same affine
    /// point the affine ladder gives, for any point and any `k`.
    pub fn mul(&self, p: Option<Pt>, k: u64) -> Option<Pt> {
        let p = p?;
        if k == 0 {
            return None;
        }
        let mut acc = self.jac(p);
        for i in (0..63 - k.leading_zeros()).rev() {
            acc = self.jac_double(acc);
            if (k >> i) & 1 == 1 {
                acc = self.jac_add_affine(acc, p);
            }
        }
        self.jac_to_affine(acc)
    }

    /// `[u]G + [v]Q` by Straus–Shamir: one doubling chain over both
    /// scalars, adding `G`, `Q` or the precomputed `G + Q` per bit, in
    /// Jacobian coordinates (two inversions in all).  Rho calls this for
    /// every walk start and every `r`-adding table entry.
    pub fn combine(&self, g: Pt, q: Pt, u: u64, v: u64) -> Option<Pt> {
        let gq = self.add(Some(g), Some(q));
        let mut acc = JAC_INFINITY;
        for i in (0..64 - (u | v).leading_zeros()).rev() {
            acc = self.jac_double(acc);
            match ((u >> i) & 1 == 1, (v >> i) & 1 == 1) {
                (true, false) => acc = self.jac_add_affine(acc, g),
                (false, true) => acc = self.jac_add_affine(acc, q),
                (true, true) => {
                    // G + Q = O adds nothing.
                    if let Some(s) = gq {
                        acc = self.jac_add_affine(acc, s);
                    }
                }
                (false, false) => {}
            }
        }
        self.jac_to_affine(acc)
    }

    #[inline]
    fn jac(&self, p: Pt) -> Jac {
        Jac {
            x: p.x,
            y: p.y,
            z: self.f.one,
        }
    }

    /// `2P` (dbl-2007-bl, general `a`).  `O` and the points with `y = 0`
    /// come out with `Z = 0` without a branch.
    #[inline]
    fn jac_double(&self, p: Jac) -> Jac {
        let f = &self.f;
        let xx = f.sqr(p.x);
        let yy = f.sqr(p.y);
        let yyyy = f.sqr(yy);
        let zz = f.sqr(p.z);
        let s = f.sub(f.sub(f.sqr(f.add(p.x, yy)), xx), yyyy);
        let s = f.add(s, s);
        let m = f.add(f.add(f.add(xx, xx), xx), f.mul(self.a, f.sqr(zz)));
        let t = f.sub(f.sqr(m), f.add(s, s));
        let y8 = f.add(yyyy, yyyy);
        let y8 = f.add(y8, y8);
        let y8 = f.add(y8, y8);
        Jac {
            x: t,
            y: f.sub(f.mul(m, f.sub(s, t)), y8),
            z: f.sub(f.sub(f.sqr(f.add(p.y, p.z)), yy), zz),
        }
    }

    /// `P + A` with `A` affine (madd-2007-bl), including `P = O`,
    /// `P = A` (a doubling) and `P = −A` (giving `O`).
    #[inline]
    fn jac_add_affine(&self, p: Jac, a: Pt) -> Jac {
        let f = &self.f;
        if p.z == 0 {
            return self.jac(a);
        }
        let z1z1 = f.sqr(p.z);
        let u2 = f.mul(a.x, z1z1);
        let s2 = f.mul(a.y, f.mul(p.z, z1z1));
        let h = f.sub(u2, p.x);
        let r = f.sub(s2, p.y);
        if h == 0 {
            return if r == 0 {
                self.jac_double(p)
            } else {
                JAC_INFINITY
            };
        }
        let r = f.add(r, r);
        let hh = f.sqr(h);
        let i = f.add(hh, hh);
        let i = f.add(i, i);
        let j = f.mul(h, i);
        let v = f.mul(p.x, i);
        let x3 = f.sub(f.sub(f.sqr(r), j), f.add(v, v));
        let y1j = f.mul(p.y, j);
        Jac {
            x: x3,
            y: f.sub(f.mul(r, f.sub(v, x3)), f.add(y1j, y1j)),
            z: f.sub(f.sub(f.sqr(f.add(p.z, h)), z1z1), hh),
        }
    }

    fn jac_to_affine(&self, p: Jac) -> Option<Pt> {
        if p.z == 0 {
            return None;
        }
        let f = &self.f;
        let zi = f.inv(p.z);
        let zi2 = f.sqr(zi);
        Some(Pt {
            x: f.mul(p.x, zi2),
            y: f.mul(p.y, f.mul(zi2, zi)),
        })
    }

    /// Lift a canonical x-coordinate to the curve point whose
    /// Montgomery-form `y ≤ (p−1)/2` (the sign convention every table
    /// in this module uses).  `None` if `x³ + ax + b` is a non-residue.
    pub fn lift_x(&self, x_canonical: u64) -> Option<Pt> {
        let f = &self.f;
        let x = f.to_mont(x_canonical);
        let rhs = f.add(f.add(f.mul(f.sqr(x), x), f.mul(self.a, x)), self.b);
        let y = f.sqrt(rhs)?;
        let y = if y > f.half { f.neg(y) } else { y };
        Some(Pt { x, y })
    }

    /// Negation-map representative: the one of `±P` with `y ≤ (p−1)/2`
    /// (in Montgomery form; any fixed choice works, this one is free).
    #[inline(always)]
    pub fn canonical_sign(&self, p: Pt) -> (Pt, bool) {
        if p.y > self.f.half {
            (self.neg(p), true)
        } else {
            (p, false)
        }
    }
}

fn biguint_to_u64(v: &BigUint) -> Option<u64> {
    let digits = v.to_u64_digits();
    match digits.len() {
        0 => Some(0),
        1 => Some(digits[0]),
        _ => None,
    }
}

// ── Parallel distinguished-point rho ──────────────────────────────────

#[derive(Clone, Debug)]
pub struct RhoConfig {
    pub threads: usize,
    /// Lock-step walkers per thread (the Montgomery-trick batch width).
    pub walkers: usize,
    /// `log₂` of the r-adding table size.
    pub table_bits: u32,
    /// A point is distinguished when this many hashed bits of `x` vanish.
    pub dp_bits: u32,
    pub negation: bool,
    pub seed: u64,
    /// Give up after this many total steps (0 = unlimited).
    pub max_steps: u64,
    /// Wiener–Zuccherato look-ahead under the negation map: a flipped step
    /// that would take the same jump again is rejected (one counted
    /// addition) and retried with the next jump.  Off: legacy.
    pub look_ahead: bool,
    /// Short-cycle detection over the last `cycle_window` points of every
    /// lane, escaping by doubling the smallest-x member and abandoning the
    /// trail on a repeat.  0: legacy (2-cycle check against the previous
    /// point only).
    pub cycle_window: usize,
    /// Table coefficients are drawn below `2^coeff_bits` (cheap table
    /// build); 0: full range `[1, n)`.
    pub coeff_bits: u32,
    /// Walk starts are `T, 2T, 3T, …` for one random `T` per worker (one
    /// addition each) instead of an independent `[a]G + [b]Q` (a joint
    /// multiplication each).  Off: legacy.
    pub stride_starts: bool,
}

impl RhoConfig {
    /// Heuristic sizing for a prime-order group of `bits` bits: enough
    /// walkers to batch well without the DP start-up overhead
    /// (`walkers · 2^dp_bits`) exceeding a few percent of the expected
    /// `√(πn/4)` steps.
    pub fn for_bits(bits: u32, negation: bool) -> Self {
        let avail = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);
        let expected = (std::f64::consts::PI * 2f64.powi(bits as i32)
            / if negation { 4.0 } else { 2.0 })
        .sqrt();
        let walkers_total = (expected / 64.0).clamp(32.0, (avail * 1024) as f64) as usize;
        let threads = (walkers_total / 64).clamp(1, avail);
        let walkers = (walkers_total / threads).max(16);
        let dp_bits = (expected / (8.0 * (walkers * threads) as f64))
            .log2()
            .floor()
            .clamp(0.0, 24.0) as u32;
        RhoConfig {
            threads,
            walkers,
            table_bits: 10,
            dp_bits,
            negation,
            seed: 1,
            max_steps: 0,
            look_ahead: false,
            cycle_window: 0,
            coeff_bits: 0,
            stride_starts: false,
        }
    }

    /// The step-efficient single-thread configuration for a prime-order
    /// group of order `r`: negation map with look-ahead and windowed cycle
    /// escape, a 4..16-entry table of short-coefficient jumps, stride
    /// starts, and `walkers · 2^dp_bits` (the lock-step detection delay)
    /// held to about 2% of the expected `√(πr/4)` steps.
    pub fn tuned(r: u64) -> Self {
        let bits = 64 - r.leading_zeros();
        let expected = (std::f64::consts::PI * r as f64 / 4.0).sqrt();
        let table_bits = match bits {
            0..=15 => 2,
            16..=22 => 3,
            _ => 4,
        };
        let walkers = if bits <= 30 { 32 } else { 64 };
        let dp_bits = (0.02 * expected / walkers as f64)
            .log2()
            .floor()
            .clamp(0.0, 24.0) as u32;
        RhoConfig {
            threads: 1,
            walkers,
            table_bits,
            dp_bits,
            negation: true,
            seed: 1,
            max_steps: 0,
            look_ahead: true,
            cycle_window: 16,
            coeff_bits: (3 * table_bits).clamp(6, 12),
            stride_starts: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RhoResult {
    pub log: u64,
    pub total_steps: u64,
    pub distinguished_points: u64,
    pub wall_secs: f64,
    pub steps_per_sec: f64,
    pub threads: usize,
    pub walkers_per_thread: usize,
    pub dp_bits: u32,
    /// Group operations behind the walk, counted without touching it.
    pub ops: RhoOpCounts,
}

/// Group operations of a [`rho_parallel`] run outside the walk steps
/// (which are `total_steps` additions): the shared table, every walk
/// start, the cycle-escape doublings and the collision checks.
#[derive(Clone, Copy, Debug, Default)]
pub struct RhoOpCounts {
    /// Additions/doublings of the table build (the `G + Q` sum included).
    pub table_adds: u64,
    pub table_doubles: u64,
    /// The same for the first start of every walker.
    pub initial_start_adds: u64,
    pub initial_start_doubles: u64,
    /// The same for every later restart (reset, cap, escape failure).
    pub restart_adds: u64,
    pub restart_doubles: u64,
    /// Doublings of the 2-cycle escape.
    pub escape_doubles: u64,
    /// Candidate collisions checked by a scalar multiplication.
    pub collision_checks: u64,
    // Diagnostics, all inside `total_steps` or the counts above:
    /// Look-ahead's rejected additions (part of `total_steps`).
    pub look_ahead_adds: u64,
    pub cycles_2: u64,
    pub cycles_4: u64,
    pub cycles_other: u64,
    /// Trails abandoned on a repeated escape, trails capped, restarts.
    pub abandoned: u64,
    pub capped: u64,
    pub restarts: u64,
    /// Lane steps on trails that reached a distinguished point.
    pub dp_trail_steps: u64,
    /// Lane steps on trails that were then restarted.
    pub lost_steps: u64,
    /// Lane steps on trails still running when the solve ended.
    pub in_flight_steps: u64,
}

/// Additions and doublings [`FastCurve::combine`] performs for `[u]G + [v]Q`:
/// the `G + Q` sum, then one doubling per bit below the top and one
/// addition per nonzero bit pair below the top (the top pair is loaded).
pub fn combine_ops(u: u64, v: u64) -> (u64, u64) {
    let top = 64 - (u | v).leading_zeros();
    if top == 0 {
        return (0, 1);
    }
    let low = (u | v) & ((1u64 << (top - 1)) - 1);
    (low.count_ones() as u64 + 1, (top - 1) as u64)
}

#[derive(Clone, Copy)]
struct TableEntry {
    pt: Pt,
    u: u64,
    v: u64,
}

struct Shared {
    dps: Mutex<HashMap<u64, (u64, u64, u64)>>, // x → (y, a, b)
    done: AtomicBool,
    answer: Mutex<Option<u64>>,
    total_steps: AtomicU64,
    dp_count: AtomicU64,
    ops: Mutex<RhoOpCounts>,
}

#[inline(always)]
fn branch_index(x: u64, table_bits: u32) -> usize {
    (x.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> (64 - table_bits)) as usize
}

#[inline(always)]
fn is_distinguished(x: u64, dp_bits: u32) -> bool {
    if dp_bits == 0 {
        return true;
    }
    (x.rotate_left(23).wrapping_mul(0xD6E8_FEB8_6659_FD93) >> (64 - dp_bits)) == 0
}

/// Solve `aG + bQ` collisions: `a1 + b1·k ≡ ±(a2 + b2·k)`.
fn solve_collision(
    curve: &FastCurve,
    g: Pt,
    q: Pt,
    (y1, a1, b1): (u64, u64, u64),
    (y2, a2, b2): (u64, u64, u64),
) -> Option<u64> {
    let fn_ = curve.scalar_field();
    let n = curve.n;
    let candidates: [(u64, u64); 1] = if y1 == y2 {
        // a1 + b1 k = a2 + b2 k  ⟹  k = (a1 − a2) / (b2 − b1)
        [(fn_.sub(a1, a2), fn_.sub(b2, b1))]
    } else {
        // a1 + b1 k = −(a2 + b2 k)  ⟹  k = −(a1 + a2) / (b1 + b2)
        [(neg_mod(add_mod(a1, a2, n), n), add_mod(b1, b2, n))]
    };
    for (num, den) in candidates {
        if den == 0 {
            continue;
        }
        let k = ((num as u128 * inv_mod_u64(den, n) as u128) % n as u128) as u64;
        if curve.mul(Some(g), k) == Some(q) {
            return Some(k);
        }
    }
    None
}

/// Parallel distinguished-point Pollard rho for `Q = [k]G` in a
/// prime-order subgroup of order `curve.n`.
pub fn rho_parallel(curve: &FastCurve, g: Pt, q: Pt, cfg: &RhoConfig) -> Option<RhoResult> {
    let started = Instant::now();
    let n = curve.n;
    // Shared r-adding table T_j = u_j G + v_j Q.
    let mut rng = StdRng::seed_from_u64(cfg.seed ^ 0xA5A5_5A5A_1234_5678);
    let table_len = 1usize << cfg.table_bits;
    let mut table = Vec::with_capacity(table_len);
    let mut table_ops = RhoOpCounts::default();
    while table.len() < table_len {
        let short = if cfg.coeff_bits == 0 {
            n
        } else {
            n.min(1u64 << cfg.coeff_bits).max(2)
        };
        let u = rng.gen_range(1..short);
        let v = rng.gen_range(1..short);
        let (a, d) = combine_ops(u, v);
        table_ops.table_adds += a;
        table_ops.table_doubles += d;
        if let Some(pt) = curve.combine(g, q, u, v) {
            table.push(TableEntry { pt, u, v });
        }
    }
    let shared = Shared {
        dps: Mutex::new(HashMap::new()),
        done: AtomicBool::new(false),
        answer: Mutex::new(None),
        total_steps: AtomicU64::new(0),
        dp_count: AtomicU64::new(0),
        ops: Mutex::new(RhoOpCounts::default()),
    };
    std::thread::scope(|s| {
        for tid in 0..cfg.threads {
            let table = &table;
            let shared = &shared;
            s.spawn(move || rho_worker(curve, g, q, table, cfg, tid, shared));
        }
    });
    let log = (*shared.answer.lock().unwrap())?;
    let mut ops = *shared.ops.lock().unwrap();
    ops.table_adds = table_ops.table_adds;
    ops.table_doubles = table_ops.table_doubles;
    let wall = started.elapsed().as_secs_f64();
    let total_steps = shared.total_steps.load(Ordering::Relaxed);
    Some(RhoResult {
        log,
        total_steps,
        distinguished_points: shared.dp_count.load(Ordering::Relaxed),
        wall_secs: wall,
        steps_per_sec: total_steps as f64 / wall.max(1e-9),
        threads: cfg.threads,
        walkers_per_thread: cfg.walkers,
        dp_bits: cfg.dp_bits,
        ops,
    })
}

/// Per-lane state of one worker's lock-step walkers.
struct Lanes {
    xs: Vec<u64>,
    ys: Vec<u64>,
    as_: Vec<u64>,
    bs: Vec<u64>,
    prev: Vec<u64>,
    /// Steps since the lane's last distinguished point or restart.
    steps: Vec<u32>,
    /// Look-ahead: jumps already tried (and rejected) from this point.
    tried: Vec<u32>,
    /// Look-ahead: the jump the next round must use instead of the hash's.
    forced: Vec<usize>,
    /// Cycle window: ring of the last `window` points per lane.
    wx: Vec<u64>,
    wy: Vec<u64>,
    wa: Vec<u64>,
    wb: Vec<u64>,
    wpos: Vec<usize>,
    wlen: Vec<usize>,
    last_escape: Vec<u64>,
}

const NO_JUMP: usize = usize::MAX;

impl Lanes {
    fn new(w: usize, window: usize) -> Self {
        let ring = w * window;
        Lanes {
            xs: vec![0; w],
            ys: vec![0; w],
            as_: vec![0; w],
            bs: vec![0; w],
            prev: vec![u64::MAX; w],
            steps: vec![0; w],
            tried: vec![0; w],
            forced: vec![NO_JUMP; w],
            wx: vec![u64::MAX; ring],
            wy: vec![0; ring],
            wa: vec![0; ring],
            wb: vec![0; ring],
            wpos: vec![0; w],
            wlen: vec![0; w],
            last_escape: vec![u64::MAX; w],
        }
    }

    /// Put lane `i` at `[a]G + [b]Q = pt` (sign-canonical when `negation`).
    fn place(&mut self, i: usize, curve: &FastCurve, pt: Pt, a: u64, b: u64, negation: bool) {
        let n = curve.n;
        let (pt, flipped) = if negation {
            curve.canonical_sign(pt)
        } else {
            (pt, false)
        };
        self.xs[i] = pt.x;
        self.ys[i] = pt.y;
        self.as_[i] = if flipped { neg_mod(a, n) } else { a };
        self.bs[i] = if flipped { neg_mod(b, n) } else { b };
        self.prev[i] = u64::MAX;
        self.steps[i] = 0;
        self.tried[i] = 0;
        self.forced[i] = NO_JUMP;
        self.wpos[i] = 0;
        self.wlen[i] = 0;
        let ring = self.wx.len() / self.xs.len();
        self.wx[i * ring..(i + 1) * ring].fill(u64::MAX);
        self.last_escape[i] = u64::MAX;
    }
}

/// The start-point stream of a worker: random `[a]G + [b]Q` (legacy) or
/// `T, 2T, 3T, …` for one random `T = [ta]G + [tb]Q` (one addition per
/// start after the first).
struct Starts {
    stride: Option<(Pt, u64, u64)>,
    cur: Option<Pt>,
    ca: u64,
    cb: u64,
}

fn rho_worker(
    curve: &FastCurve,
    g: Pt,
    q: Pt,
    table: &[TableEntry],
    cfg: &RhoConfig,
    tid: usize,
    shared: &Shared,
) {
    let f = curve.f;
    let n = curve.n;
    let w = cfg.walkers;
    // The ring is a power of two, so the index arithmetic is a mask.
    let window = if cfg.cycle_window == 0 {
        0
    } else {
        cfg.cycle_window.next_power_of_two()
    };
    let mask = window.wrapping_sub(1);
    let jumps = table.len();
    // The tuned walk also counts the final partial round exactly; the
    // legacy walk's `total_steps` stays as it was.
    let tuned = cfg.look_ahead || cfg.cycle_window > 0 || cfg.stride_starts;
    let mut rng =
        StdRng::seed_from_u64(cfg.seed ^ (tid as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    let step_cap: u32 = (40u64 << cfg.dp_bits).max(256).min(u32::MAX as u64) as u32;

    let mut lanes = Lanes::new(w, window.max(1));
    let mut d = vec![0u64; w];
    let mut js = vec![0usize; w];
    let mut reset = vec![false; w];
    let mut scratch = Vec::with_capacity(w);
    let counts = std::cell::Cell::new(RhoOpCounts::default());
    let initial = std::cell::Cell::new(true);
    let starts = std::cell::RefCell::new(Starts {
        stride: None,
        cur: None,
        ca: 0,
        cb: 0,
    });
    let mut escape_doubles = 0u64;
    let mut collision_checks = 0u64;
    // Diagnostics (never charged).
    let (mut la_adds, mut cycles_2, mut cycles_4, mut cycles_other) = (0u64, 0u64, 0u64, 0u64);
    let (mut abandoned, mut capped, mut restarts) = (0u64, 0u64, 0u64);
    let mut dp_trail_steps = 0u64;

    let restart = |i: usize, rng: &mut StdRng, lanes: &mut Lanes| {
        let mut c0 = counts.get();
        c0.lost_steps += lanes.steps[i] as u64;
        counts.set(c0);
        loop {
            let mut c = counts.get();
            let (pt, a, b) = if cfg.stride_starts {
                let mut st = starts.borrow_mut();
                if st.stride.is_none() {
                    // The stride and the first start: one full-width combine.
                    let (ta, tb) = (rng.gen_range(1..n), rng.gen_range(1..n));
                    let (da, dd) = combine_ops(ta, tb);
                    c.initial_start_adds += da;
                    c.initial_start_doubles += dd;
                    counts.set(c);
                    match curve.combine(g, q, ta, tb) {
                        Some(t) => {
                            st.stride = Some((t, ta, tb));
                            st.cur = Some(t);
                            st.ca = ta;
                            st.cb = tb;
                            (t, ta, tb)
                        }
                        None => continue,
                    }
                } else {
                    let (t, ta, tb) = st.stride.unwrap();
                    if initial.get() {
                        c.initial_start_adds += 1;
                    } else {
                        c.restart_adds += 1;
                    }
                    counts.set(c);
                    st.ca = add_mod(st.ca, ta, n);
                    st.cb = add_mod(st.cb, tb, n);
                    st.cur = curve.add(st.cur, Some(t));
                    match st.cur {
                        Some(p) => (p, st.ca, st.cb),
                        None => continue,
                    }
                }
            } else {
                let a = rng.gen_range(1..n);
                let b = rng.gen_range(1..n);
                let (da, dd) = combine_ops(a, b);
                if initial.get() {
                    c.initial_start_adds += da;
                    c.initial_start_doubles += dd;
                } else {
                    c.restart_adds += da;
                    c.restart_doubles += dd;
                }
                counts.set(c);
                match curve.combine(g, q, a, b) {
                    Some(pt) => (pt, a, b),
                    None => continue,
                }
            };
            lanes.place(i, curve, pt, a, b, cfg.negation);
            return;
        }
    };
    for i in 0..w {
        restart(i, &mut rng, &mut lanes);
    }
    initial.set(false);
    let mut local_steps = 0u64;
    let mut local_dps = 0u64;
    'outer: loop {
        if shared.done.load(Ordering::Relaxed) {
            break;
        }
        if cfg.max_steps > 0 && shared.total_steps.load(Ordering::Relaxed) > cfg.max_steps {
            break;
        }
        for i in 0..w {
            let j = if lanes.forced[i] != NO_JUMP {
                lanes.forced[i]
            } else {
                branch_index(lanes.xs[i], cfg.table_bits)
            };
            js[i] = j;
            let dd = f.sub(table[j].pt.x, lanes.xs[i]);
            if dd == 0 {
                reset[i] = true;
                d[i] = f.one;
            } else {
                reset[i] = false;
                d[i] = dd;
            }
        }
        f.batch_inv(&mut d, &mut scratch);
        for i in 0..w {
            if reset[i] {
                restarts += 1;
                restart(i, &mut rng, &mut lanes);
                continue;
            }
            let t = &table[js[i]];
            let lam = f.mul(f.sub(t.pt.y, lanes.ys[i]), d[i]);
            let x3 = f.sub(f.sub(f.sqr(lam), lanes.xs[i]), t.pt.x);
            let mut y3 = f.sub(f.mul(lam, f.sub(lanes.xs[i], x3)), lanes.ys[i]);
            let mut a = add_mod(lanes.as_[i], t.u, n);
            let mut b = add_mod(lanes.bs[i], t.v, n);
            let mut flipped = false;
            if cfg.negation && y3 > f.half {
                y3 = f.p - y3;
                a = neg_mod(a, n);
                b = neg_mod(b, n);
                flipped = true;
            }
            // Look-ahead: a flipped step that would take the same jump
            // again is rejected (one wasted addition) and the next round
            // retries this lane from the same point with the next jump, so
            // the retry rides the next batch's inversion.
            if cfg.look_ahead
                && flipped
                && ((lanes.tried[i] + 1) as usize) < jumps
                && branch_index(x3, cfg.table_bits) == js[i]
            {
                la_adds += 1;
                lanes.tried[i] += 1;
                lanes.forced[i] = (js[i] + 1) % jumps;
                continue;
            }
            lanes.tried[i] = 0;
            lanes.forced[i] = NO_JUMP;
            if window > 0 {
                // Short cycle: x3 is one of the last `window` points.
                let base = i * window;
                let len = lanes.wlen[i];
                let pos = lanes.wpos[i];
                let mut hit = 0usize;
                // Unused slots hold `u64::MAX`, never an x: the common
                // no-hit case is one branch-free pass over the ring.
                if lanes.wx[base..base + window].contains(&x3) {
                    for k in 1..=len {
                        let idx = base + ((pos + window - k) & mask);
                        if lanes.wx[idx] == x3 {
                            hit = k;
                            break;
                        }
                    }
                }
                if hit > 0 {
                    match hit {
                        2 => cycles_2 += 1,
                        4 => cycles_4 += 1,
                        _ => cycles_other += 1,
                    }
                    // Leave from the member with the smallest x.
                    let mut best = base + ((pos + window - 1) & mask);
                    for k in 2..=hit {
                        let idx = base + ((pos + window - k) & mask);
                        if lanes.wx[idx] < lanes.wx[best] {
                            best = idx;
                        }
                    }
                    let (cx, cy, ca, cb) = (
                        lanes.wx[best],
                        lanes.wy[best],
                        lanes.wa[best],
                        lanes.wb[best],
                    );
                    if lanes.last_escape[i] == cx {
                        abandoned += 1;
                        restarts += 1;
                        restart(i, &mut rng, &mut lanes);
                        continue;
                    }
                    escape_doubles += 1;
                    match curve.double(Pt { x: cx, y: cy }) {
                        Some(pt) => {
                            let kept = lanes.steps[i];
                            lanes.place(
                                i,
                                curve,
                                pt,
                                add_mod(ca, ca, n),
                                add_mod(cb, cb, n),
                                cfg.negation,
                            );
                            lanes.last_escape[i] = cx;
                            lanes.steps[i] = kept;
                        }
                        None => {
                            restarts += 1;
                            restart(i, &mut rng, &mut lanes);
                        }
                    }
                    continue;
                }
                let slot = base + pos;
                lanes.wx[slot] = x3;
                lanes.wy[slot] = y3;
                lanes.wa[slot] = a;
                lanes.wb[slot] = b;
                lanes.wpos[i] = (pos + 1) & mask;
                lanes.wlen[i] = (len + 1).min(window);
                lanes.prev[i] = lanes.xs[i];
                lanes.xs[i] = x3;
                lanes.ys[i] = y3;
                lanes.as_[i] = a;
                lanes.bs[i] = b;
            } else if x3 == lanes.prev[i] {
                // Fruitless 2-cycle {prev, cur}: both members are known
                // (prev == new).  Escape deterministically by doubling the
                // member with the smaller x, so merged walks stay merged.
                let (cx, cy, ca, cb) = if lanes.xs[i] < x3 {
                    (lanes.xs[i], lanes.ys[i], lanes.as_[i], lanes.bs[i])
                } else {
                    (x3, y3, a, b)
                };
                escape_doubles += 1;
                match curve.double(Pt { x: cx, y: cy }) {
                    Some(pt) => {
                        let (pt, flipped) = if cfg.negation {
                            curve.canonical_sign(pt)
                        } else {
                            (pt, false)
                        };
                        let a2 = add_mod(ca, ca, n);
                        let b2 = add_mod(cb, cb, n);
                        lanes.xs[i] = pt.x;
                        lanes.ys[i] = pt.y;
                        lanes.as_[i] = if flipped { neg_mod(a2, n) } else { a2 };
                        lanes.bs[i] = if flipped { neg_mod(b2, n) } else { b2 };
                        lanes.prev[i] = u64::MAX;
                    }
                    None => {
                        restart(i, &mut rng, &mut lanes);
                        continue;
                    }
                }
            } else {
                lanes.prev[i] = lanes.xs[i];
                lanes.xs[i] = x3;
                lanes.ys[i] = y3;
                lanes.as_[i] = a;
                lanes.bs[i] = b;
            }
            lanes.steps[i] += 1;
            let (xi, yi, ai, bi) = (lanes.xs[i], lanes.ys[i], lanes.as_[i], lanes.bs[i]);
            if is_distinguished(xi, cfg.dp_bits) {
                local_dps += 1;
                dp_trail_steps += lanes.steps[i] as u64;
                let mut dps = shared.dps.lock().unwrap();
                match dps.get(&xi) {
                    Some(&other) => {
                        if (other.1, other.2) != (ai, bi) {
                            collision_checks += 1;
                            if let Some(k) = solve_collision(curve, g, q, (yi, ai, bi), other) {
                                *shared.answer.lock().unwrap() = Some(k);
                                shared.done.store(true, Ordering::Relaxed);
                                drop(dps);
                                if tuned {
                                    // The additions of the final partial round.
                                    local_steps += i as u64 + 1;
                                }
                                break 'outer;
                            }
                        }
                    }
                    None => {
                        dps.insert(xi, (yi, ai, bi));
                    }
                }
                drop(dps);
                lanes.steps[i] = 0;
            } else if lanes.steps[i] > step_cap {
                // Probable fruitless cycle of length ≥ 4: abandon the trail.
                capped += 1;
                restarts += 1;
                restart(i, &mut rng, &mut lanes);
            }
        }
        local_steps += w as u64;
        if local_steps >= 1 << 16 {
            shared.total_steps.fetch_add(local_steps, Ordering::Relaxed);
            shared.dp_count.fetch_add(local_dps, Ordering::Relaxed);
            local_steps = 0;
            local_dps = 0;
        }
    }
    shared.total_steps.fetch_add(local_steps, Ordering::Relaxed);
    shared.dp_count.fetch_add(local_dps, Ordering::Relaxed);
    let in_flight: u64 = lanes.steps.iter().map(|&s| s as u64).sum();
    let mut tot = shared.ops.lock().unwrap();
    let c = counts.get();
    tot.initial_start_adds += c.initial_start_adds;
    tot.initial_start_doubles += c.initial_start_doubles;
    tot.restart_adds += c.restart_adds;
    tot.restart_doubles += c.restart_doubles;
    tot.escape_doubles += escape_doubles;
    tot.collision_checks += collision_checks;
    tot.look_ahead_adds += la_adds;
    tot.cycles_2 += cycles_2;
    tot.cycles_4 += cycles_4;
    tot.cycles_other += cycles_other;
    tot.abandoned += abandoned;
    tot.capped += capped;
    tot.restarts += restarts;
    tot.dp_trail_steps += dp_trail_steps;
    tot.lost_steps += c.lost_steps;
    tot.in_flight_steps += in_flight;
}

// ── Index calculus: factor base, pair table, relations, linear algebra ─

/// Small-x factor base: the points with the smallest x-coordinates
/// `x = 1, 2, 3, …` that lie on the curve, with the `y ≤ (p−1)/2` sign.
/// Same policy as [`crate::cryptanalysis::ec_index_calculus::build_factor_base`].
#[derive(Clone, Debug)]
pub struct FastFactorBase {
    pub pts: Vec<Pt>,
    /// Canonical x of each entry.
    pub xs: Vec<u64>,
    pub max_x: u64,
    /// `index[x] = i` for `x = xs[i]`, else `u32::MAX`.
    index: Vec<u32>,
}

impl FastFactorBase {
    pub fn build(curve: &FastCurve, size: usize) -> Self {
        let mut pts = Vec::with_capacity(size);
        let mut xs = Vec::with_capacity(size);
        let mut x = 1u64;
        while pts.len() < size && x < curve.f.p {
            if let Some(pt) = curve.lift_x(x) {
                pts.push(pt);
                xs.push(x);
            }
            x += 1;
        }
        let max_x = xs.last().copied().unwrap_or(0);
        let mut index = vec![u32::MAX; max_x as usize + 1];
        for (i, &x) in xs.iter().enumerate() {
            index[x as usize] = i as u32;
        }
        FastFactorBase {
            pts,
            xs,
            max_x,
            index,
        }
    }

    pub fn len(&self) -> usize {
        self.pts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pts.is_empty()
    }

    #[inline(always)]
    pub fn lookup(&self, x_canonical: u64) -> Option<usize> {
        if x_canonical > self.max_x {
            return None;
        }
        let i = self.index[x_canonical as usize];
        if i == u32::MAX {
            None
        } else {
            Some(i as usize)
        }
    }
}

/// Meet-in-the-middle table for 3-decompositions: every
/// `x(F_j + F_k)` (`j ≤ k`) and `x(F_j − F_k)` (`j < k`), bucketed by the
/// high bits of the canonical x (entries inside a bucket are unordered).  `B²` entries of 16 bytes.
#[derive(Clone, Debug)]
pub struct PairTable {
    /// `(x_canonical, j, k_with_sign)`; the top bit of the third field
    /// is set for a difference `F_j − F_k`.
    entries: Vec<(u64, u32, u32)>,
    /// Bucket index over the high bits of x: bucket `b` holds `entries[starts[b]..starts[b + 1]]`,
    /// the buckets are monotone in x; entries inside a bucket are in construction order.
    starts: Vec<u32>,
    shift: u32,
    /// Largest canonical x in the table (0 when empty).
    max_x: u64,
    /// Legacy layout (one bucket, globally sorted, binary-search lookup); `IC_PAIR_SORTED=1`.
    sorted: bool,
}

const PAIR_DIFF_BIT: u32 = 1 << 31;

impl PairTable {
    pub fn build(curve: &FastCurve, fb: &FastFactorBase) -> Self {
        let f = curve.f;
        let b = fb.len();
        let t_build = Instant::now();
        let mut entries = Vec::with_capacity(b * b);
        let mut d = Vec::with_capacity(b);
        let mut scratch = Vec::with_capacity(b);
        for j in 0..b {
            let fj = fb.pts[j];
            if let Some(dbl) = curve.double(fj) {
                entries.push((f.from_mont(dbl.x), j as u32, j as u32));
            }
            // One inverse per (j, k) pair serves both F_j + F_k and F_j − F_k.
            d.clear();
            for k in (j + 1)..b {
                d.push(f.sub(fb.pts[k].x, fj.x));
            }
            f.batch_inv(&mut d, &mut scratch);
            for (idx, k) in ((j + 1)..b).enumerate() {
                let fk = fb.pts[k];
                let inv = d[idx];
                let sum_x = curve.add_x_with_inv(fj, fk, inv);
                let diff_x = curve.add_x_with_inv(fj, curve.neg(fk), inv);
                entries.push((f.from_mont(sum_x), j as u32, k as u32));
                entries.push((f.from_mont(diff_x), j as u32, k as u32 | PAIR_DIFF_BIT));
            }
        }
        let add_ms = t_build.elapsed().as_secs_f64() * 1e3;
        let t_sort = Instant::now();
        if std::env::var_os("IC_PAIR_SORTED").is_some() {
            entries.sort_unstable();
            let max_x = entries.last().map(|e| e.0).unwrap_or(0);
            let len = entries.len() as u32;
            return PairTable {
                entries,
                starts: vec![0, len],
                shift: 63,
                max_x,
                sorted: true,
            };
        }
        // Counting sort over the high bits of x (monotone buckets), then a tiny sort inside each bucket.
        let bits_p = 64 - (f.p.max(2) - 1).leading_zeros();
        let target_bits = (usize::BITS - 1 - (entries.len().max(32) / 2).leading_zeros()).max(4);
        let shift = bits_p.saturating_sub(target_bits);
        let nb = (((f.p.max(2) - 1) >> shift) + 1) as usize;
        let mut starts = vec![0u32; nb + 1];
        for e in &entries {
            starts[((e.0 >> shift) as usize) + 1] += 1;
        }
        for i in 0..nb {
            starts[i + 1] += starts[i];
        }
        let mut cursor: Vec<u32> = starts[..nb].to_vec();
        let mut sorted: Vec<(u64, u32, u32)> = vec![(0, 0, 0); entries.len()];
        for e in &entries {
            let b = (e.0 >> shift) as usize;
            sorted[cursor[b] as usize] = *e;
            cursor[b] += 1;
        }
        let max_x = entries.iter().map(|e| e.0).max().unwrap_or(0);
        let entries = sorted;
        if std::env::var_os("IC_MULTI_TIMING").is_some() {
            eprintln!(
                "pair table: {} entries, adds {:.1} ms, sort {:.1} ms",
                entries.len(),
                add_ms,
                t_sort.elapsed().as_secs_f64() * 1e3
            );
        }
        PairTable {
            entries,
            starts,
            shift,
            max_x,
            sorted: false,
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// All `(j, k, sigma)` with `x(F_j + sigma·F_k) == x`.
    #[inline]
    pub fn lookup(&self, x: u64) -> impl Iterator<Item = &(u64, u32, u32)> {
        let b = (x >> self.shift) as usize;
        let mut bucket: &[(u64, u32, u32)] = if b + 1 >= self.starts.len() {
            &[]
        } else {
            &self.entries[self.starts[b] as usize..self.starts[b + 1] as usize]
        };
        if self.sorted {
            let lo = bucket.partition_point(|e| e.0 < x);
            let mut hi = lo;
            while hi < bucket.len() && bucket[hi].0 == x {
                hi += 1;
            }
            bucket = &bucket[lo..hi];
        }
        bucket.iter().filter(move |e| e.0 == x)
    }

    /// Largest canonical x in the table (0 when empty).
    pub fn max_x(&self) -> u64 {
        self.max_x
    }
}

/// One relation `a·G + b·Q = Σ entries[i].1 · F_{entries[i].0}`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FastRelation {
    pub a: u64,
    pub b: u64,
    pub entries: Vec<(u32, i8)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct SmallEntries {
    len: u8,
    items: [(u32, i8); 3],
}

impl SmallEntries {
    #[inline(always)]
    fn one(i0: u32, s0: i8) -> Self {
        if s0 == 0 {
            return Self {
                len: 0,
                items: [(0, 0); 3],
            };
        }
        Self {
            len: 1,
            items: [(i0, s0), (0, 0), (0, 0)],
        }
    }

    #[inline(always)]
    fn two(i0: u32, s0: i8, i1: u32, s1: i8) -> Self {
        Self::from_raw(&[(i0, s0), (i1, s1)])
    }

    #[inline(always)]
    fn three(i0: u32, s0: i8, i1: u32, s1: i8, i2: u32, s2: i8) -> Self {
        Self::from_raw(&[(i0, s0), (i1, s1), (i2, s2)])
    }

    #[inline]
    fn from_raw(raw: &[(u32, i8)]) -> Self {
        let mut buf = [(0u32, 0i8); 3];
        let n = raw.len().min(3);
        buf[..n].copy_from_slice(&raw[..n]);
        if n >= 2 && buf[0].0 > buf[1].0 {
            buf.swap(0, 1);
        }
        if n == 3 {
            if buf[1].0 > buf[2].0 {
                buf.swap(1, 2);
            }
            if buf[0].0 > buf[1].0 {
                buf.swap(0, 1);
            }
        }
        let mut out = [(0u32, 0i8); 3];
        let mut m = 0usize;
        for &(idx, s) in &buf[..n] {
            if s == 0 {
                continue;
            }
            if m > 0 && out[m - 1].0 == idx {
                out[m - 1].1 += s;
                if out[m - 1].1 == 0 {
                    out[m - 1] = (0, 0);
                    m -= 1;
                }
            } else {
                out[m] = (idx, s);
                m += 1;
            }
        }
        Self {
            len: m as u8,
            items: out,
        }
    }

    #[inline(always)]
    fn as_slice(&self) -> &[(u32, i8)] {
        &self.items[..self.len as usize]
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    fn to_vec(self) -> Vec<(u32, i8)> {
        self.as_slice().to_vec()
    }
}

/// Fail-closed check: re-add the decomposition in the group.
fn relation_holds(curve: &FastCurve, fb: &FastFactorBase, r: Pt, entries: &[(u32, i8)]) -> bool {
    let mut acc: Option<Pt> = None;
    for &(idx, s) in entries {
        let pt = fb.pts[idx as usize];
        let pt = if s < 0 { curve.neg(pt) } else { pt };
        for _ in 0..s.unsigned_abs() {
            acc = curve.add(acc, Some(pt));
        }
    }
    acc == Some(r)
}

#[derive(Clone, Debug, Default)]
pub struct CollectStats {
    pub targets: u64,
    pub relations: usize,
    pub rows_one: usize,
    pub rows_two: usize,
    pub rows_three: usize,
    pub rejected: usize,
}

/// Collect `needed` relations by walking targets `R = aG + bQ`
/// (fresh random `(a, b)` after every productive target, `R ← R + G`
/// otherwise) and decomposing each by direct subtraction:
///
/// - 2-decomposition: `x(R ∓ F_i) ∈ FB` — one batched inverse per
///   target, one array lookup per `(i, sign)`;
/// - 3-decomposition (when `pairs` is given): `x(R ∓ F_i)` matched
///   against the pair table.
pub fn collect_relations(
    curve: &FastCurve,
    g: Pt,
    q: Pt,
    fb: &FastFactorBase,
    pairs: Option<&PairTable>,
    needed: usize,
    max_targets: u64,
    seed: u64,
) -> (Vec<FastRelation>, CollectStats) {
    let f = curve.f;
    let n = curve.n;
    let b_len = fb.len();
    let mut rng = StdRng::seed_from_u64(seed);
    let mut rels: Vec<FastRelation> = Vec::with_capacity(needed);
    let mut stats = CollectStats::default();
    let mut d = vec![0u64; b_len];
    let mut degenerate = vec![false; b_len];
    let mut scratch = Vec::with_capacity(b_len);
    let mut seen: HashSet<SmallEntries> = HashSet::with_capacity(8);
    let mut found: Vec<SmallEntries> = Vec::with_capacity(8);

    let fresh = |rng: &mut StdRng| -> (u64, u64, Pt) {
        loop {
            let a = rng.gen_range(1..n);
            let b = rng.gen_range(1..n);
            if let Some(r) = curve.combine(g, q, a, b) {
                return (a, b, r);
            }
        }
    };
    let (mut a, mut b, mut r) = fresh(&mut rng);

    while rels.len() < needed && stats.targets < max_targets {
        stats.targets += 1;
        for i in 0..b_len {
            let dd = f.sub(r.x, fb.pts[i].x);
            degenerate[i] = dd == 0;
            d[i] = if dd == 0 { f.one } else { dd };
        }
        f.batch_inv(&mut d, &mut scratch);
        seen.clear();
        found.clear();
        for i in 0..b_len {
            let fi = fb.pts[i];
            if degenerate[i] {
                // R = ±F_i outright.
                let s = if r.y == fi.y { 1i8 } else { -1 };
                found.push(SmallEntries::one(i as u32, s));
                continue;
            }
            let inv = d[i];
            // T = R − ε·F_i for ε = +1 (lam uses y_R + y_i) and ε = −1
            // (lam uses y_R − y_i); both share the inverse of x_R − x_i.
            let lam_minus = f.mul(f.add(r.y, fi.y), inv);
            let lam_plus = f.mul(f.sub(r.y, fi.y), inv);
            for (eps, lam) in [(1i8, lam_minus), (-1i8, lam_plus)] {
                let x3 = f.sub(f.sub(f.sqr(lam), r.x), fi.x);
                let xc = f.from_mont(x3);
                let mut y3: Option<u64> = None;
                if let Some(j) = fb.lookup(xc) {
                    if j >= i {
                        let y = f.sub(f.mul(lam, f.sub(r.x, x3)), r.y);
                        y3 = Some(y);
                        let tau = if y == fb.pts[j].y { 1i8 } else { -1 };
                        found.push(SmallEntries::two(i as u32, eps, j as u32, tau));
                    }
                }
                if let Some(pairs) = pairs {
                    if xc <= pairs.max_x() {
                        for &(_, j, k_raw) in pairs.lookup(xc) {
                            if (j as usize) < i {
                                continue;
                            }
                            let sigma = if k_raw & PAIR_DIFF_BIT != 0 { -1i8 } else { 1 };
                            let k = (k_raw & !PAIR_DIFF_BIT) as usize;
                            let fk = fb.pts[k];
                            let fk = if sigma < 0 { curve.neg(fk) } else { fk };
                            let Some(pjk) = curve.add(Some(fb.pts[j as usize]), Some(fk)) else {
                                continue;
                            };
                            let y =
                                *y3.get_or_insert_with(|| f.sub(f.mul(lam, f.sub(r.x, x3)), r.y));
                            let tau = if y == pjk.y { 1i8 } else { -1 };
                            found.push(SmallEntries::three(
                                i as u32,
                                eps,
                                j,
                                tau,
                                k as u32,
                                tau * sigma,
                            ));
                        }
                    }
                }
            }
        }
        let mut productive = false;
        for &merged in &found {
            if merged.is_empty() || !seen.insert(merged) {
                continue;
            }
            if !relation_holds(curve, fb, r, merged.as_slice()) {
                stats.rejected += 1;
                continue;
            }
            match merged.len {
                1 => stats.rows_one += 1,
                2 => stats.rows_two += 1,
                _ => stats.rows_three += 1,
            }
            rels.push(FastRelation {
                a,
                b,
                entries: merged.to_vec(),
            });
            productive = true;
            if rels.len() >= needed {
                break;
            }
        }
        if productive {
            let (na, nb, nr) = fresh(&mut rng);
            a = na;
            b = nb;
            r = nr;
        } else {
            match curve.add(Some(r), Some(g)) {
                Some(nr) => {
                    r = nr;
                    a = add_mod(a, 1, n);
                }
                None => {
                    let (na, nb, nr) = fresh(&mut rng);
                    a = na;
                    b = nb;
                    r = nr;
                }
            }
        }
    }
    stats.relations = rels.len();
    (rels, stats)
}

/// Table-lookup 2-decomposition: like [`collect_relations`] but, instead of subtracting every
/// factor-base element from `R` (O(fb) work per target), looks `x(R)` up in the pair table of
/// `x(F_j ± F_k)` (plus the factor base itself for 1-term rows), O(1) per target.  The decomposable
/// set is the same as the 2-decomposition sweep's (`R = ±F_j ± F_k`), found in the same walk
/// (`R ← R + G`, fresh `(a, b)` after a productive target); relations are verified in the group.
/// Experimental (`ic_solve_table`); the existing paths are unchanged.
pub fn collect_relations_table(
    curve: &FastCurve,
    g: Pt,
    q: Pt,
    fb: &FastFactorBase,
    pairs: &PairTable,
    needed: usize,
    max_targets: u64,
    seed: u64,
) -> (Vec<FastRelation>, CollectStats) {
    let f = curve.f;
    let n = curve.n;
    let mut rng = StdRng::seed_from_u64(seed);
    let mut rels: Vec<FastRelation> = Vec::with_capacity(needed);
    let mut stats = CollectStats::default();
    let max_pair_x = pairs.max_x();
    let mut found: Vec<SmallEntries> = Vec::with_capacity(8);
    let mut seen: HashSet<SmallEntries> = HashSet::with_capacity(8);

    let fresh = |rng: &mut StdRng| -> (u64, u64, Pt) {
        loop {
            let a = rng.gen_range(1..n);
            let b = rng.gen_range(1..n);
            if let Some(r) = curve.combine(g, q, a, b) {
                return (a, b, r);
            }
        }
    };
    let (mut a, mut b, mut r) = fresh(&mut rng);

    while rels.len() < needed && stats.targets < max_targets {
        stats.targets += 1;
        let xc = f.from_mont(r.x);
        found.clear();
        if let Some(j) = fb.lookup(xc) {
            let s = if r.y == fb.pts[j].y { 1i8 } else { -1 };
            found.push(SmallEntries::one(j as u32, s));
        }
        if xc <= max_pair_x {
            for &(_, j, k_raw) in pairs.lookup(xc) {
                let sigma = if k_raw & PAIR_DIFF_BIT != 0 { -1i8 } else { 1 };
                let k = (k_raw & !PAIR_DIFF_BIT) as usize;
                let fk = fb.pts[k];
                let fk = if sigma < 0 { curve.neg(fk) } else { fk };
                let Some(pjk) = curve.add(Some(fb.pts[j as usize]), Some(fk)) else {
                    continue;
                };
                let tau = if r.y == pjk.y { 1i8 } else { -1 };
                found.push(SmallEntries::two(j, tau, k as u32, tau * sigma));
            }
        }
        let mut productive = false;
        seen.clear();
        for &merged in &found {
            if merged.is_empty() || !seen.insert(merged) {
                continue;
            }
            if !relation_holds(curve, fb, r, merged.as_slice()) {
                stats.rejected += 1;
                continue;
            }
            match merged.len {
                1 => stats.rows_one += 1,
                2 => stats.rows_two += 1,
                _ => stats.rows_three += 1,
            }
            rels.push(FastRelation {
                a,
                b,
                entries: merged.to_vec(),
            });
            productive = true;
            if rels.len() >= needed {
                break;
            }
        }
        if productive {
            let (na, nb, nr) = fresh(&mut rng);
            a = na;
            b = nb;
            r = nr;
        } else {
            match curve.add(Some(r), Some(g)) {
                Some(nr) => {
                    r = nr;
                    a = add_mod(a, 1, n);
                }
                None => {
                    let (na, nb, nr) = fresh(&mut rng);
                    a = na;
                    b = nb;
                    r = nr;
                }
            }
        }
    }
    stats.relations = rels.len();
    (rels, stats)
}

/// Solve the relation system for `x = log_G Q` by Gauss–Jordan
/// elimination mod `n` on `u64` (Montgomery form).  Unknowns are
/// `(y_0 … y_{m−1}, x)`; row `r` reads
/// `Σ e_j y_j − b·x ≡ a (mod n)`.  Returns `None` unless `x` is pinned.
pub fn solve_relations(curve: &FastCurve, m: usize, rels: &[FastRelation]) -> Option<u64> {
    let fnum = curve.scalar_field();
    let n = curve.n;
    let cols = m + 1;
    let mut rows: Vec<Vec<u64>> = Vec::with_capacity(rels.len());
    let mut rhs: Vec<u64> = Vec::with_capacity(rels.len());
    for rel in rels {
        let mut row = vec![0u64; cols];
        for &(j, s) in &rel.entries {
            let v = if s < 0 {
                neg_mod(s.unsigned_abs() as u64 % n, n)
            } else {
                s as u64 % n
            };
            row[j as usize] = fnum.to_mont(v);
        }
        row[m] = fnum.to_mont(neg_mod(rel.b % n, n));
        rows.push(row);
        rhs.push(fnum.to_mont(rel.a % n));
    }
    let nrows = rows.len();
    let mut pivot_row_of_col = vec![usize::MAX; cols];
    let mut row = 0usize;
    for col in 0..cols {
        if row >= nrows {
            break;
        }
        let Some(piv) = (row..nrows).find(|&r| rows[r][col] != 0) else {
            continue;
        };
        rows.swap(row, piv);
        rhs.swap(row, piv);
        let inv = fnum.inv(rows[row][col]);
        for c in 0..cols {
            rows[row][c] = fnum.mul(rows[row][c], inv);
        }
        rhs[row] = fnum.mul(rhs[row], inv);
        let (pivot_vals, pivot_rhs) = (rows[row].clone(), rhs[row]);
        for r in 0..nrows {
            if r == row || rows[r][col] == 0 {
                continue;
            }
            let factor = rows[r][col];
            for c in col..cols {
                if pivot_vals[c] != 0 {
                    rows[r][c] = fnum.sub(rows[r][c], fnum.mul(factor, pivot_vals[c]));
                }
            }
            rhs[r] = fnum.sub(rhs[r], fnum.mul(factor, pivot_rhs));
        }
        pivot_row_of_col[col] = row;
        row += 1;
    }
    // Consistency: zero rows must have zero rhs.
    for r in row..nrows {
        if rhs[r] != 0 && rows[r].iter().all(|&v| v == 0) {
            return None;
        }
    }
    let pr = pivot_row_of_col[m];
    if pr == usize::MAX {
        return None;
    }
    Some(fnum.from_mont(rhs[pr]))
}

#[derive(Clone, Debug, Default)]
pub struct IcReport {
    pub factor_base_ms: f64,
    pub pair_table_ms: f64,
    pub relations_ms: f64,
    pub linear_algebra_ms: f64,
    pub verify_ms: f64,
    pub total_ms: f64,
    pub factor_base_size: usize,
    pub pair_table_entries: usize,
    pub relations: usize,
    pub targets: u64,
    pub rows_one: usize,
    pub rows_two: usize,
    pub rows_three: usize,
}

/// End-to-end index calculus: factor base → (pair table) → relations →
/// linear algebra → group verification.  `summands` is 2 or 3.
pub fn ic_solve(
    curve: &FastCurve,
    g: Pt,
    q: Pt,
    fb_size: usize,
    extra_relations: usize,
    summands: u32,
    max_targets: u64,
    seed: u64,
) -> Option<(u64, IcReport)> {
    let t0 = Instant::now();
    let fb = FastFactorBase::build(curve, fb_size);
    let factor_base_ms = t0.elapsed().as_secs_f64() * 1e3;
    if fb.is_empty() {
        return None;
    }
    let t1 = Instant::now();
    let pairs = if summands >= 3 {
        Some(PairTable::build(curve, &fb))
    } else {
        None
    };
    let pair_table_ms = t1.elapsed().as_secs_f64() * 1e3;
    let needed = fb.len() + extra_relations;
    let t2 = Instant::now();
    let (rels, stats) =
        collect_relations(curve, g, q, &fb, pairs.as_ref(), needed, max_targets, seed);
    let relations_ms = t2.elapsed().as_secs_f64() * 1e3;
    if rels.len() < needed {
        return None;
    }
    let t3 = Instant::now();
    let x = solve_relations(curve, fb.len(), &rels)?;
    let linear_algebra_ms = t3.elapsed().as_secs_f64() * 1e3;
    let t4 = Instant::now();
    if curve.mul(Some(g), x) != Some(q) {
        return None;
    }
    let verify_ms = t4.elapsed().as_secs_f64() * 1e3;
    Some((
        x,
        IcReport {
            factor_base_ms,
            pair_table_ms,
            relations_ms,
            linear_algebra_ms,
            verify_ms,
            total_ms: t0.elapsed().as_secs_f64() * 1e3,
            factor_base_size: fb.len(),
            pair_table_entries: pairs.map(|p| p.len()).unwrap_or(0),
            relations: rels.len(),
            targets: stats.targets,
            rows_one: stats.rows_one,
            rows_two: stats.rows_two,
            rows_three: stats.rows_three,
        },
    ))
}

/// Solve for the discrete logs of the factor-base points to base `G` from relations `sum s_j x_j = a + b`
/// (every relation collected against `Q = G`, so `R = (a + b) G`).  Returns `Some(x_j)` only for columns
/// whose value is determined (pivot column with no free column in its reduced row).
pub fn solve_fb_logs(curve: &FastCurve, m: usize, rels: &[FastRelation]) -> Vec<Option<u64>> {
    let fnum = curve.scalar_field();
    let n = curve.n;
    let mut rows: Vec<Vec<u64>> = Vec::with_capacity(rels.len());
    let mut rhs: Vec<u64> = Vec::with_capacity(rels.len());
    for rel in rels {
        let mut row = vec![0u64; m];
        for &(j, sg) in &rel.entries {
            let v = if sg < 0 {
                neg_mod(sg.unsigned_abs() as u64 % n, n)
            } else {
                sg as u64 % n
            };
            row[j as usize] = fnum.to_mont(v);
        }
        rows.push(row);
        rhs.push(fnum.to_mont(add_mod(rel.a % n, rel.b % n, n)));
    }
    let nrows = rows.len();
    let mut pivot_of_col = vec![usize::MAX; m];
    let mut row = 0usize;
    for col in 0..m {
        if row >= nrows {
            break;
        }
        let Some(piv) = (row..nrows).find(|&r| rows[r][col] != 0) else {
            continue;
        };
        rows.swap(row, piv);
        rhs.swap(row, piv);
        let inv = fnum.inv(rows[row][col]);
        for c in 0..m {
            rows[row][c] = fnum.mul(rows[row][c], inv);
        }
        rhs[row] = fnum.mul(rhs[row], inv);
        let (pv, pr) = (rows[row].clone(), rhs[row]);
        for r in 0..nrows {
            if r == row || rows[r][col] == 0 {
                continue;
            }
            let factor = rows[r][col];
            for c in col..m {
                if pv[c] != 0 {
                    rows[r][c] = fnum.sub(rows[r][c], fnum.mul(factor, pv[c]));
                }
            }
            rhs[r] = fnum.sub(rhs[r], fnum.mul(factor, pr));
        }
        pivot_of_col[col] = row;
        row += 1;
    }
    (0..m)
        .map(|c| {
            let r = pivot_of_col[c];
            if r == usize::MAX {
                return None;
            }
            let free = (0..m).any(|c2| c2 != c && rows[r][c2] != 0);
            if free {
                None
            } else {
                Some(fnum.from_mont(rhs[r]))
            }
        })
        .collect()
}

#[derive(Clone, Debug, Default)]
pub struct MultiReport {
    pub setup_ms: f64,
    pub per_target_ms: Vec<f64>,
    pub total_ms: f64,
    pub factor_base_size: usize,
    pub determined_logs: usize,
    pub targets_walked: Vec<u64>,
}

/// Relation collection against `Q = G` for the shared factor-base solve, with `lanes` walkers advanced in
/// lock-step (one batched inversion per round) and the pair-table lookup of [`collect_relations_table`].
/// Relations are returned as `a = a_eff, b = 0` (so `R = a G`).
fn collect_setup_batched(
    curve: &FastCurve,
    g: Pt,
    fb: &FastFactorBase,
    pairs: &PairTable,
    needed: usize,
    lanes: usize,
    seed: u64,
) -> Vec<FastRelation> {
    let n = curve.n;
    let mut rng = StdRng::seed_from_u64(seed ^ 0x5E70_BA7C);
    let max_pair_x = pairs.max_x();
    let mut rels: Vec<FastRelation> = Vec::with_capacity(needed);
    let mut seen: HashSet<SmallEntries> = HashSet::with_capacity(needed * 2);
    let mut cands: Vec<SmallEntries> = Vec::with_capacity(8);
    let fresh = |rng: &mut StdRng| -> (u64, Pt) {
        loop {
            let a = rng.gen_range(1..n);
            if let Some(r) = curve.mul(Some(g), a) {
                return (a, r);
            }
        }
    };
    let mut a_l: Vec<u64> = Vec::with_capacity(lanes);
    let mut cur: Vec<Option<Pt>> = Vec::with_capacity(lanes);
    for _ in 0..lanes {
        let (a, r) = fresh(&mut rng);
        a_l.push(a);
        cur.push(Some(r));
    }
    let gv: Vec<Option<Pt>> = vec![Some(g); lanes];
    let mut scratch = BatchAddScratch::default();
    let mut guard = 0u64;
    while rels.len() < needed && guard < 50_000_000 {
        guard += 1;
        for l in 0..lanes {
            let Some(r) = cur[l] else {
                let (a, nr) = fresh(&mut rng);
                a_l[l] = add_mod(a, 1, n);
                cur[l] = Some(nr);
                continue;
            };
            let xc = curve.f.from_mont(r.x);
            cands.clear();
            if let Some(j) = fb.lookup(xc) {
                let sg = if r.y == fb.pts[j].y { 1i8 } else { -1 };
                cands.push(SmallEntries::one(j as u32, sg));
            }
            if xc <= max_pair_x {
                for &(_, j, k_raw) in pairs.lookup(xc) {
                    let sigma = if k_raw & PAIR_DIFF_BIT != 0 { -1i8 } else { 1 };
                    let k = (k_raw & !PAIR_DIFF_BIT) as usize;
                    let fk = fb.pts[k];
                    let fk = if sigma < 0 { curve.neg(fk) } else { fk };
                    let Some(pjk) = curve.add(Some(fb.pts[j as usize]), Some(fk)) else {
                        continue;
                    };
                    let tau = if r.y == pjk.y { 1i8 } else { -1 };
                    cands.push(SmallEntries::two(j, tau, k as u32, tau * sigma));
                }
            }
            let mut productive = false;
            for &merged in &cands {
                if merged.is_empty() || !seen.insert(merged) {
                    continue;
                }
                if !relation_holds(curve, fb, r, merged.as_slice()) {
                    continue;
                }
                rels.push(FastRelation {
                    a: a_l[l],
                    b: 0,
                    entries: merged.to_vec(),
                });
                productive = true;
                if rels.len() >= needed {
                    break;
                }
            }
            if productive {
                // restart this lane at a fresh point, pre-shifted by -G so the batched step below lands on it
                let (a, nr) = fresh(&mut rng);
                a_l[l] = a;
                cur[l] = Some(nr);
            }
            a_l[l] = add_mod(a_l[l], 1, n);
        }
        curve.add_many_batched(&mut cur, &gv, &mut scratch);
    }
    rels
}

/// Same as [`ic_multi_solve_ex`] without the batched per-target phase.
pub fn ic_multi_solve(
    curve: &FastCurve,
    g: Pt,
    qs: &[Pt],
    fb_size: usize,
    extra_relations: usize,
    max_walk: u64,
    seed: u64,
    table: bool,
) -> Option<(Vec<Option<u64>>, MultiReport)> {
    ic_multi_solve_ex(
        curve,
        g,
        qs,
        fb_size,
        extra_relations,
        max_walk,
        seed,
        table,
        false,
    )
}

/// Report of [`ic_solve_fast`]: the batched pair-table path's phases for a single target.
#[derive(Clone, Debug, Default)]
pub struct IcFastReport {
    /// Factor-base size actually used (after auto-tuning / clamping).
    pub factor_base_size: usize,
    /// Shared setup: factor base + pair table + batched relation collection + log solve (ms).
    pub setup_ms: f64,
    /// Per-target lock-step walk until a decomposition over determined logs (ms).
    pub target_walk_ms: f64,
    pub total_ms: f64,
    /// Points visited by the target walk.
    pub target_points_walked: u64,
    /// Factor-base logs determined by the linear solve.
    pub determined_logs: usize,
}

/// Auto-tuned factor-base size for [`ic_solve_fast`]: `round(p^(1/3))` clamped to `[32, 16384]`
/// (the measured optimum of the batched table path is B ~ p^(1/3); see
/// `research/prime_ic_collection_frontier_20261117`).
pub fn ic_fast_auto_fb(p: u64) -> usize {
    ((p as f64).cbrt().round() as usize).clamp(32, 16_384)
}

/// Fast single-target prime-field index calculus: the batched lock-step pair-table path
/// ([`ic_multi_solve_ex`] with `table = batched = true`) run on `[q]`.  Additive: [`ic_solve`] is
/// unchanged.  `fb_size = None` auto-tunes via [`ic_fast_auto_fb`].  The returned log is verified
/// `x G == q` inside the multi solver (fail-closed `None` otherwise).
pub fn ic_solve_fast(
    curve: &FastCurve,
    g: Pt,
    q: Pt,
    fb_size: Option<usize>,
    extra_relations: usize,
    max_walk: u64,
    seed: u64,
) -> Option<(u64, IcFastReport)> {
    let b = fb_size.unwrap_or_else(|| ic_fast_auto_fb(curve.f.p));
    let (logs, rep) = ic_multi_solve_ex(
        curve,
        g,
        &[q],
        b,
        extra_relations,
        max_walk,
        seed,
        true,
        true,
    )?;
    let x = logs.first().copied().flatten()?;
    Some((
        x,
        IcFastReport {
            factor_base_size: rep.factor_base_size,
            setup_ms: rep.setup_ms,
            target_walk_ms: rep.per_target_ms.first().copied().unwrap_or(0.0),
            total_ms: rep.total_ms,
            target_points_walked: rep.targets_walked.first().copied().unwrap_or(0),
            determined_logs: rep.determined_logs,
        },
    ))
}

/// Multi-target index calculus: one shared factor-base log solve (relations against `G`), then per target
/// one relation `aG + bQ_t = sum s_j F_j` (3-term decompositions) gives `log Q_t = (sum s_j x_j - a)/b`.
/// Every returned log is verified `k G == Q_t`; `None` entries failed (no relation within `max_walk`
/// points, an undetermined factor-base log, or b = 0).
pub fn ic_multi_solve_ex(
    curve: &FastCurve,
    g: Pt,
    qs: &[Pt],
    fb_size: usize,
    extra_relations: usize,
    max_walk: u64,
    seed: u64,
    table: bool,
    batched: bool,
) -> Option<(Vec<Option<u64>>, MultiReport)> {
    let t0 = Instant::now();
    let fb = FastFactorBase::build(curve, fb_size);
    if fb.is_empty() {
        return None;
    }
    let t_pt = Instant::now();
    let pairs = PairTable::build(curve, &fb);
    let pair_ms = t_pt.elapsed().as_secs_f64() * 1e3;
    let needed = fb.len() + extra_relations;
    let collect = |q: Pt, need: usize, maxw: u64, sd: u64| {
        if table {
            collect_relations_table(curve, g, q, &fb, &pairs, need, maxw, sd)
        } else {
            collect_relations(curve, g, q, &fb, Some(&pairs), need, maxw, sd)
        }
    };
    let t_col = Instant::now();
    let (rels, _) = if table && batched {
        (
            collect_setup_batched(curve, g, &fb, &pairs, needed, 256, seed),
            CollectStats::default(),
        )
    } else {
        collect(g, needed, u64::MAX, seed)
    };
    let col_ms = t_col.elapsed().as_secs_f64() * 1e3;
    if rels.len() < needed {
        return None;
    }
    let t_sol = Instant::now();
    let logs = solve_fb_logs(curve, fb.len(), &rels);
    if std::env::var_os("IC_MULTI_TIMING").is_some() {
        eprintln!(
            "multi setup: fb {} pair_table {:.1} ms collect {:.1} ms solve {:.1} ms",
            fb.len(),
            pair_ms,
            col_ms,
            t_sol.elapsed().as_secs_f64() * 1e3
        );
    }
    let determined = logs.iter().filter(|l| l.is_some()).count();
    let setup_ms = t0.elapsed().as_secs_f64() * 1e3;
    let fnum = curve.scalar_field();
    let n = curve.n;
    if table && batched {
        // All targets walk in lock-step: one batched inversion per round advances every active R_t <- R_t + G;
        // each point is looked up in the pair table (O(1)); a relation using only determined logs retires the lane.
        let m = qs.len();
        let mut rng = StdRng::seed_from_u64(seed ^ 0xBA7C_4ED0);
        let max_pair_x = pairs.max_x();
        let mut ab: Vec<(u64, u64)> = vec![(0, 0); m];
        let mut cur: Vec<Option<Pt>> = Vec::with_capacity(m);
        let fresh = |rng: &mut StdRng, q: Pt| -> (u64, u64, Pt) {
            loop {
                let a = rng.gen_range(1..n);
                let b = rng.gen_range(1..n);
                if let Some(r) = curve.combine(g, q, a, b) {
                    return (a, b, r);
                }
            }
        };
        for (t, &q) in qs.iter().enumerate() {
            let (a, b, r) = fresh(&mut rng, q);
            ab[t] = (a, b);
            cur.push(Some(r));
        }
        let mut res: Vec<Option<u64>> = vec![None; m];
        let mut act: Vec<usize> = (0..m).collect();
        let mut act2: Vec<usize> = Vec::with_capacity(m);
        let mut cur2: Vec<Option<Pt>> = Vec::with_capacity(m);
        let gv: Vec<Option<Pt>> = vec![Some(g); m];
        let mut cands: Vec<SmallEntries> = Vec::with_capacity(8);
        let mut scratch = BatchAddScratch::default();
        let mut walked = vec![0u64; m];
        let mut round = 0u64;
        while !act.is_empty() && round < max_walk {
            round += 1;
            act2.clear();
            cur2.clear();
            for (pos, &t) in act.iter().enumerate() {
                walked[t] += 1;
                let (a, b) = ab[t];
                let Some(r) = cur[pos] else {
                    let (na, nb, nr) = fresh(&mut rng, qs[t]);
                    ab[t] = (add_mod(na, 1, n), nb);
                    act2.push(t);
                    cur2.push(Some(nr));
                    continue;
                };
                let xc = curve.f.from_mont(r.x);
                cands.clear();
                if let Some(j) = fb.lookup(xc) {
                    let sg = if r.y == fb.pts[j].y { 1i8 } else { -1 };
                    cands.push(SmallEntries::one(j as u32, sg));
                }
                if xc <= max_pair_x {
                    for &(_, j, k_raw) in pairs.lookup(xc) {
                        let sigma = if k_raw & PAIR_DIFF_BIT != 0 { -1i8 } else { 1 };
                        let k = (k_raw & !PAIR_DIFF_BIT) as usize;
                        let fk = fb.pts[k];
                        let fk = if sigma < 0 { curve.neg(fk) } else { fk };
                        let Some(pjk) = curve.add(Some(fb.pts[j as usize]), Some(fk)) else {
                            continue;
                        };
                        let tau = if r.y == pjk.y { 1i8 } else { -1 };
                        cands.push(SmallEntries::two(j, tau, k as u32, tau * sigma));
                    }
                }
                let mut done = false;
                let mut restart = false;
                for &merged in &cands {
                    let slice = merged.as_slice();
                    if slice.is_empty() || !relation_holds(curve, &fb, r, slice) {
                        continue;
                    }
                    let usable =
                        b % n != 0 && slice.iter().all(|&(j, _)| logs[j as usize].is_some());
                    if !usable {
                        restart = true;
                        continue;
                    }
                    let mut sum = 0u64;
                    for &(j, sg) in slice {
                        let x = logs[j as usize].unwrap() % n;
                        let term = if sg < 0 { neg_mod(x, n) } else { x };
                        let term = fnum.from_mont(fnum.mul(
                            fnum.to_mont(term),
                            fnum.to_mont((sg.unsigned_abs() as u64) % n),
                        ));
                        sum = add_mod(sum, term, n);
                    }
                    let num = add_mod(sum, neg_mod(a % n, n), n);
                    let k = ((num as u128 * inv_mod_u64(b % n, n) as u128) % n as u128) as u64;
                    if curve.mul(Some(g), k) == Some(qs[t]) {
                        res[t] = Some(k);
                        done = true;
                        break;
                    }
                    restart = true;
                }
                if done {
                    continue;
                }
                if restart {
                    let (na, nb, nr) = fresh(&mut rng, qs[t]);
                    ab[t] = (add_mod(na, 1, n), nb);
                    act2.push(t);
                    cur2.push(Some(nr));
                } else {
                    ab[t].0 = add_mod(a, 1, n);
                    act2.push(t);
                    cur2.push(Some(r));
                }
            }
            // every lane in act2 now holds R = a*G + b*Q - G... i.e. a has been incremented to match R + G below
            let cur2_len = cur2.len();
            curve.add_many_batched(&mut cur2, &gv[..cur2_len], &mut scratch);
            std::mem::swap(&mut act, &mut act2);
            std::mem::swap(&mut cur, &mut cur2);
        }
        let total_ms = t0.elapsed().as_secs_f64() * 1e3;
        let m_f = m.max(1) as f64;
        return Some((
            res,
            MultiReport {
                setup_ms,
                per_target_ms: vec![(total_ms - setup_ms) / m_f; m],
                total_ms,
                factor_base_size: fb.len(),
                determined_logs: determined,
                targets_walked: walked,
            },
        ));
    }
    let mut out = Vec::with_capacity(qs.len());
    let mut per = Vec::with_capacity(qs.len());
    let mut walked = Vec::with_capacity(qs.len());
    for (t, &q) in qs.iter().enumerate() {
        let tt = Instant::now();
        let mut rs: Vec<FastRelation> = Vec::new();
        let mut st = CollectStats::default();
        for attempt in 0..32u64 {
            let (r1, s1) = collect(
                q,
                1,
                max_walk,
                seed ^ (0x9E37_79B9 + t as u64 * 7919 + attempt * 104_729),
            );
            st.targets += s1.targets;
            let usable = r1.first().is_some_and(|r| {
                r.b % n != 0 && r.entries.iter().all(|&(j, _)| logs[j as usize].is_some())
            });
            if usable {
                rs = r1;
                break;
            }
        }
        walked.push(st.targets);
        let mut res = None;
        if let Some(rel) = rs.first() {
            let mut acc: Option<u64> = Some(0);
            for &(j, sg) in &rel.entries {
                match (acc, logs[j as usize]) {
                    (Some(a0), Some(x)) => {
                        let term = if sg < 0 {
                            neg_mod(x.wrapping_mul(1) % n, n)
                        } else {
                            x % n
                        };
                        let term = fnum.from_mont(fnum.mul(
                            fnum.to_mont(term),
                            fnum.to_mont((sg.unsigned_abs() as u64) % n),
                        ));
                        acc = Some(add_mod(a0, term, n));
                    }
                    _ => acc = None,
                }
            }
            if let (Some(sum), true) = (acc, rel.b % n != 0) {
                let num = add_mod(sum, neg_mod(rel.a % n, n), n);
                let k = ((num as u128 * inv_mod_u64(rel.b % n, n) as u128) % n as u128) as u64;
                if curve.mul(Some(g), k) == Some(q) {
                    res = Some(k);
                }
            }
        }
        per.push(tt.elapsed().as_secs_f64() * 1e3);
        out.push(res);
    }
    Some((
        out,
        MultiReport {
            setup_ms,
            per_target_ms: per,
            total_ms: t0.elapsed().as_secs_f64() * 1e3,
            factor_base_size: fb.len(),
            determined_logs: determined,
            targets_walked: walked,
        },
    ))
}

/// End-to-end index calculus with the table-lookup collector ([`collect_relations_table`]): factor
/// base → pair table → relations → linear algebra → group verification.  Same report and the same
/// fail-closed verification as [`ic_solve`].  Experimental.
pub fn ic_solve_table(
    curve: &FastCurve,
    g: Pt,
    q: Pt,
    fb_size: usize,
    extra_relations: usize,
    max_targets: u64,
    seed: u64,
) -> Option<(u64, IcReport)> {
    let t0 = Instant::now();
    let fb = FastFactorBase::build(curve, fb_size);
    let factor_base_ms = t0.elapsed().as_secs_f64() * 1e3;
    if fb.is_empty() {
        return None;
    }
    let t1 = Instant::now();
    let pairs = PairTable::build(curve, &fb);
    let pair_table_ms = t1.elapsed().as_secs_f64() * 1e3;
    let needed = fb.len() + extra_relations;
    let t2 = Instant::now();
    let (rels, stats) =
        collect_relations_table(curve, g, q, &fb, &pairs, needed, max_targets, seed);
    let relations_ms = t2.elapsed().as_secs_f64() * 1e3;
    if rels.len() < needed {
        return None;
    }
    let t3 = Instant::now();
    let x = solve_relations(curve, fb.len(), &rels)?;
    let linear_algebra_ms = t3.elapsed().as_secs_f64() * 1e3;
    let t4 = Instant::now();
    if curve.mul(Some(g), x) != Some(q) {
        return None;
    }
    let verify_ms = t4.elapsed().as_secs_f64() * 1e3;
    Some((
        x,
        IcReport {
            factor_base_ms,
            pair_table_ms,
            relations_ms,
            linear_algebra_ms,
            verify_ms,
            total_ms: t0.elapsed().as_secs_f64() * 1e3,
            factor_base_size: fb.len(),
            pair_table_entries: pairs.len(),
            relations: rels.len(),
            targets: stats.targets,
            rows_one: stats.rows_one,
            rows_two: stats.rows_two,
            rows_three: stats.rows_three,
        },
    ))
}

// ── Shared factor-base subproduct tree & lock-step multi-walker IC ────

/// Default crossover factor-base size `B*` at/above which top-down
/// remainder-tree multipoint evaluation on `P_V(X)` replaces direct `Fp64`
/// Horner evaluation over `V`.
pub const DEFAULT_MULTIPOINT_CROSSOVER_B: usize = 64;

/// Default crossover factor-base size `B*` at/above which batched candidate
/// product-tree + `P_V(X)` remainder-tree evaluation replaces direct small-`B`
/// `Fp64` Horner evaluation over `V`.
pub const DEFAULT_BATCH_SUBPRODUCT_CROSSOVER_B: usize = 96;

/// Leaf subtree width at/below which remainder-tree multipoint evaluation
/// switches to direct Horner evaluation in cache.
const MULTIPOINT_LEAF_WIDTH: usize = 16;

#[inline]
fn poly_trim_mont(p: &mut Vec<u64>) {
    while p.last() == Some(&0) {
        p.pop();
    }
}

#[inline]
fn max_lazy_terms(p: u64) -> usize {
    (((1u128 << 64) / (p as u128)) as usize).max(1)
}

/// Lazy-`u128` schoolbook polynomial multiplication in Montgomery form.
/// Every output coefficient performs at most `ceil(min_deg / max_lazy_terms(p))`
/// Montgomery reductions (`1` reduction per output term whenever `p < 2^52`
/// and `min_len <= 4096`).
fn poly_mul_schoolbook_mont(f: &Fp64, a: &[u64], b: &[u64], out: &mut Vec<u64>) {
    if a.is_empty() || b.is_empty() {
        out.clear();
        return;
    }
    let na = a.len();
    let nb = b.len();
    let out_len = na + nb - 1;
    out.clear();
    out.resize(out_len, 0);
    let max_lazy = max_lazy_terms(f.p);
    let one_u128 = f.one as u128;
    for k in 0..out_len {
        let i_lo = k.saturating_sub(nb - 1);
        let i_hi = k.min(na - 1);
        let mut acc: u128 = 0;
        let mut cnt = 0usize;
        for i in i_lo..=i_hi {
            acc += (a[i] as u128) * (b[k - i] as u128);
            cnt += 1;
            if cnt == max_lazy {
                let red = f.reduce(acc);
                acc = (red as u128) * one_u128;
                cnt = 1;
            }
        }
        out[k] = f.reduce(acc);
    }
    poly_trim_mont(out);
}

/// Polynomial multiplication in Montgomery form: lazy-`u128` schoolbook below
/// 48 coefficients, Karatsuba above.
pub fn poly_mul_mont(f: &Fp64, a: &[u64], b: &[u64], out: &mut Vec<u64>) {
    let mut aa = a;
    while aa.last() == Some(&0) {
        aa = &aa[..aa.len() - 1];
    }
    let mut bb = b;
    while bb.last() == Some(&0) {
        bb = &bb[..bb.len() - 1];
    }
    if aa.is_empty() || bb.is_empty() {
        out.clear();
        return;
    }
    if aa.len().min(bb.len()) < 48 {
        poly_mul_schoolbook_mont(f, aa, bb, out);
        return;
    }
    let split = aa.len().max(bb.len()) / 2;
    let (a0, a1) = if split < aa.len() {
        aa.split_at(split)
    } else {
        (aa, &[][..])
    };
    let (b0, b1) = if split < bb.len() {
        bb.split_at(split)
    } else {
        (bb, &[][..])
    };
    let mut z0 = Vec::new();
    let mut z2 = Vec::new();
    poly_mul_mont(f, a0, b0, &mut z0);
    poly_mul_mont(f, a1, b1, &mut z2);

    let sa_len = a0.len().max(a1.len());
    let mut sa = Vec::with_capacity(sa_len);
    for i in 0..sa_len {
        let x = a0.get(i).copied().unwrap_or(0);
        let y = a1.get(i).copied().unwrap_or(0);
        sa.push(f.add(x, y));
    }
    let sb_len = b0.len().max(b1.len());
    let mut sb = Vec::with_capacity(sb_len);
    for i in 0..sb_len {
        let x = b0.get(i).copied().unwrap_or(0);
        let y = b1.get(i).copied().unwrap_or(0);
        sb.push(f.add(x, y));
    }
    let mut z1 = Vec::new();
    poly_mul_mont(f, &sa, &sb, &mut z1);
    let z1_need = z1.len().max(z0.len()).max(z2.len());
    if z1.len() < z1_need {
        z1.resize(z1_need, 0);
    }
    for (i, &v) in z0.iter().enumerate() {
        z1[i] = f.sub(z1[i], v);
    }
    for (i, &v) in z2.iter().enumerate() {
        z1[i] = f.sub(z1[i], v);
    }
    poly_trim_mont(&mut z1);

    let out_len = aa.len() + bb.len() - 1;
    out.clear();
    out.resize(out_len, 0);
    for (i, &v) in z0.iter().enumerate() {
        out[i] = v;
    }
    for (i, &v) in z1.iter().enumerate() {
        if i + split < out_len {
            out[i + split] = f.add(out[i + split], v);
        }
    }
    for (i, &v) in z2.iter().enumerate() {
        if i + 2 * split < out_len {
            out[i + 2 * split] = f.add(out[i + 2 * split], v);
        }
    }
    poly_trim_mont(out);
}

/// Remainder `a(X) mod d(X)` for monic `d(X)` (`*d.last() == Some(&f.one)`) in
/// Montgomery form using lazy-`u128` Knuth division (zero field inversions, one
/// Montgomery reduction per quotient term and per remainder term).
pub fn poly_rem_monic_mont(
    f: &Fp64,
    a: &[u64],
    d: &[u64],
    rem_u128: &mut Vec<u128>,
    out: &mut Vec<u64>,
) {
    debug_assert!(!d.is_empty() && *d.last().unwrap() == f.one);
    let m = d.len() - 1;
    if m == 0 {
        out.clear();
        return;
    }
    let mut aa = a;
    while aa.last() == Some(&0) {
        aa = &aa[..aa.len() - 1];
    }
    if aa.len() <= m {
        out.clear();
        out.extend_from_slice(aa);
        return;
    }
    let n = aa.len() - 1;
    let one_u128 = f.one as u128;
    let max_lazy = max_lazy_terms(f.p);
    rem_u128.clear();
    rem_u128.reserve(n + 1);
    for &c in aa {
        rem_u128.push((c as u128) * one_u128);
    }
    let mut ops_in_window = 1usize;
    let d_low = &d[..m];
    for k in (m..=n).rev() {
        let lead = f.reduce(rem_u128[k]);
        if lead != 0 {
            if ops_in_window == max_lazy {
                for slot in &mut rem_u128[..k] {
                    *slot = (f.reduce(*slot) as u128) * one_u128;
                }
                ops_in_window = 1;
            }
            let neg_lead = (f.p - lead) as u128;
            let base = k - m;
            for j in 0..m {
                rem_u128[base + j] += neg_lead * (d_low[j] as u128);
            }
            ops_in_window += 1;
        }
    }
    out.clear();
    out.reserve(m);
    for &slot in &rem_u128[..m] {
        out.push(f.reduce(slot));
    }
    poly_trim_mont(out);
}

#[inline]
fn poly_eval_horner_mont(f: &Fp64, coeffs_mont: &[u64], x_mont: u64) -> u64 {
    let mut acc = 0u64;
    for &c in coeffs_mont.iter().rev() {
        acc = f.add(f.mul(acc, x_mont), c);
    }
    acc
}

fn poly_make_monic_mont(f: &Fp64, p: &mut Vec<u64>) {
    poly_trim_mont(p);
    let Some(&lead) = p.last() else {
        return;
    };
    if lead == f.one {
        return;
    }
    let inv = f.inv(lead);
    let last = p.len() - 1;
    for c in &mut p[..last] {
        *c = f.mul(*c, inv);
    }
    p[last] = f.one;
}

fn poly_gcd_mont(f: &Fp64, a_in: &[u64], b_in: &[u64], rem_u128: &mut Vec<u128>) -> Vec<u64> {
    let mut a = a_in.to_vec();
    let mut b = b_in.to_vec();
    poly_make_monic_mont(f, &mut a);
    poly_make_monic_mont(f, &mut b);
    let mut next = Vec::new();
    while !b.is_empty() {
        poly_rem_monic_mont(f, &a, &b, rem_u128, &mut next);
        poly_make_monic_mont(f, &mut next);
        a = std::mem::take(&mut b);
        b = std::mem::take(&mut next);
    }
    a
}

fn poly_pow_mod_mont(
    f: &Fp64,
    base_in: &[u64],
    mut exp: u64,
    modulus_monic: &[u64],
    rem_u128: &mut Vec<u128>,
) -> Vec<u64> {
    if modulus_monic.len() <= 1 {
        return Vec::new();
    }
    let mut acc = vec![f.one];
    let mut base = Vec::new();
    poly_rem_monic_mont(f, base_in, modulus_monic, rem_u128, &mut base);
    let mut prod = Vec::new();
    let mut rem = Vec::new();
    while exp > 0 {
        if exp & 1 == 1 {
            poly_mul_schoolbook_mont(f, &acc, &base, &mut prod);
            poly_rem_monic_mont(f, &prod, modulus_monic, rem_u128, &mut rem);
            std::mem::swap(&mut acc, &mut rem);
        }
        exp >>= 1;
        if exp > 0 {
            poly_mul_schoolbook_mont(f, &base, &base, &mut prod);
            poly_rem_monic_mont(f, &prod, modulus_monic, rem_u128, &mut rem);
            std::mem::swap(&mut base, &mut rem);
        }
    }
    acc
}

/// Split a monic squarefree polynomial `g_monic` (in Montgomery form) whose
/// irreducible factors over `F_p` are all linear, returning canonical sorted
/// roots in `[0, p)`.
fn split_totally_linear_monic_mont(
    f: &Fp64,
    g_monic: &[u64],
    inv_two_mont: u64,
    rem_u128: &mut Vec<u128>,
    rng_state: &mut u64,
    out_canonical: &mut Vec<u64>,
) {
    if g_monic.len() <= 1 {
        return;
    }
    let mut queue: Vec<Vec<u64>> = vec![g_monic.to_vec()];
    let half_p = (f.p - 1) / 2;
    let mut next_rand = || -> u64 {
        let mut s = *rng_state;
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        *rng_state = s;
        f.to_mont(s % f.p)
    };
    while let Some(h) = queue.pop() {
        let deg = h.len() - 1;
        if deg == 0 {
            continue;
        }
        if deg == 1 {
            out_canonical.push(f.from_mont(f.neg(h[0])));
            continue;
        }
        if deg == 2 {
            // h(X) = X^2 + c1 X + c0 splits over F_p.
            let half_c1 = f.mul(h[1], inv_two_mont);
            let disc = f.sub(f.sqr(half_c1), h[0]);
            if let Some(s) = f.sqrt(disc) {
                let r0 = f.from_mont(f.sub(s, half_c1));
                let r1 = f.from_mont(f.neg(f.add(s, half_c1)));
                out_canonical.push(r0);
                if r1 != r0 {
                    out_canonical.push(r1);
                }
            }
            continue;
        }
        loop {
            let delta = next_rand();
            // (X + delta)^((p-1)/2) - 1 mod h(X)
            let lin = [delta, f.one];
            let mut pow = poly_pow_mod_mont(f, &lin, half_p, &h, rem_u128);
            if pow.is_empty() {
                pow.push(f.neg(f.one));
            } else {
                pow[0] = f.sub(pow[0], f.one);
                poly_trim_mont(&mut pow);
            }
            if pow.is_empty() {
                continue;
            }
            let factor = poly_gcd_mont(f, &h, &pow, rem_u128);
            let f_deg = factor.len().saturating_sub(1);
            if f_deg > 0 && f_deg < deg {
                // Divide h by monic `factor` via exact monic quotient.
                let q = poly_div_exact_monic_mont(f, &h, &factor);
                queue.push(factor);
                queue.push(q);
                break;
            }
        }
    }
    out_canonical.sort_unstable();
    out_canonical.dedup();
}

fn poly_div_exact_monic_mont(f: &Fp64, a: &[u64], d_monic: &[u64]) -> Vec<u64> {
    let m = d_monic.len() - 1;
    let n = a.len() - 1;
    if n < m {
        return Vec::new();
    }
    let mut rem = a.to_vec();
    let q_len = n - m + 1;
    let mut q = vec![0u64; q_len];
    for k in (m..=n).rev() {
        let lead = rem[k];
        q[k - m] = lead;
        if lead != 0 {
            for j in 0..m {
                rem[k - m + j] = f.sub(rem[k - m + j], f.mul(lead, d_monic[j]));
            }
        }
    }
    poly_trim_mont(&mut q);
    q
}

/// Native Montgomery `Fp64` Cantor–Zassenhaus root finder over `F_p` (`p < 2^62`).
/// Returns all distinct roots in `F_p` sorted in canonical `[0, p)` order,
/// matching [`crate::cryptanalysis::ec_index_calculus::find_roots_fp_fast`]
/// (`Vec::new()` on the zero or nonzero constant polynomial).
pub fn find_roots_fp64_cz(f: &Fp64, coeffs_canonical: &[u64]) -> Vec<u64> {
    let mut poly: Vec<u64> = coeffs_canonical
        .iter()
        .map(|&c| f.to_mont(c % f.p))
        .collect();
    poly_make_monic_mont(f, &mut poly);
    if poly.len() <= 1 {
        return Vec::new();
    }
    let inv_two = f.inv(f.add(f.one, f.one));
    if poly.len() == 2 {
        return vec![f.from_mont(f.neg(poly[0]))];
    }
    if poly.len() == 3 {
        let half_c1 = f.mul(poly[1], inv_two);
        let disc = f.sub(f.sqr(half_c1), poly[0]);
        let Some(s) = f.sqrt(disc) else {
            return Vec::new();
        };
        let r0 = f.from_mont(f.sub(s, half_c1));
        let r1 = f.from_mont(f.neg(f.add(s, half_c1)));
        let mut out = if r0 == r1 { vec![r0] } else { vec![r0, r1] };
        out.sort_unstable();
        return out;
    }
    let mut rem_u128 = Vec::with_capacity(poly.len() * 2);
    // X^p - X mod poly(X)
    let x_poly = [0u64, f.one];
    let mut xp = poly_pow_mod_mont(f, &x_poly, f.p, &poly, &mut rem_u128);
    if xp.len() < 2 {
        xp.resize(2, 0);
    }
    xp[1] = f.sub(xp[1], f.one);
    poly_trim_mont(&mut xp);
    let g = if xp.is_empty() {
        poly
    } else {
        poly_gcd_mont(f, &poly, &xp, &mut rem_u128)
    };
    if g.len() <= 1 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(g.len() - 1);
    let mut seed = f.p ^ 0xA55A_9E37_79B9_7F4A;
    split_totally_linear_monic_mont(f, &g, inv_two, &mut rem_u128, &mut seed, &mut out);
    out
}

/// Reusable scratch buffers for [`FactorBaseSubproductTree::roots_in_base_batch`].
#[derive(Default, Clone, Debug)]
pub struct SubproductBatchScratch {
    rem_u128: Vec<u128>,
    cand_levels: Vec<Vec<Vec<u64>>>,
    rem_curr: Vec<Vec<u64>>,
    rem_next: Vec<Vec<u64>>,
    gcd_a: Vec<Vec<u64>>,
    denoms: Vec<u64>,
    denom_idx: Vec<usize>,
    prefix: Vec<u64>,
}

/// Precomputed factor-base subproduct tree `P_V(X) = prod_{x in V} (X - x)`
/// over `Fp64`, supporting:
/// 1. Fast top-down remainder-tree multipoint evaluation (`eval_all`), and
/// 2. Crossover-gated factor-base root extraction for single polynomials
///    (`roots_in_base`) and multi-polynomial candidate batches
///    (`roots_in_base_batch`), eliminating the `O(d^2 log p)` per-candidate
///    `gcd(X^p - X, g(X))` squaring ladder on non-`V`-splitting polynomials.
#[derive(Clone, Debug)]
pub struct FactorBaseSubproductTree {
    pub f: Fp64,
    /// Sorted deduplicated canonical abscissae `x in V`.
    pub xs: Vec<u64>,
    /// Montgomery-form abscissae aligned with `xs`.
    xs_mont: Vec<u64>,
    /// Binary product tree levels in Montgomery form (`levels[0]` are leaves
    /// `[ -x_i, 1 ]` or leaf blocks, `levels.last()[0]` is `P_V(X)`).
    levels: Vec<Vec<Vec<u64>>>,
    /// `(2)^(-1)` in Montgomery form.
    inv_two_mont: u64,
    /// Fast membership table when `max_x <= 262_144`, else `HashSet`.
    dense_membership: Vec<u64>,
    max_x: u64,
    sparse_membership: HashSet<u64>,
    pub multipoint_crossover_b: usize,
    pub batch_crossover_b: usize,
}

impl FastFactorBase {
    /// Build the shared factor-base subproduct tree `P_V(X) = prod_{x in FB} (X - x)`
    /// once during preparation.
    pub fn prepare_subproduct_tree(&self, curve: &FastCurve) -> FactorBaseSubproductTree {
        FactorBaseSubproductTree::prepare(curve.f, &self.xs)
    }
}

impl FactorBaseSubproductTree {
    /// Precompute `P_V(X) = prod_{x in V} (X - x)` once per factor base.
    pub fn prepare(f: Fp64, base_xs: &[u64]) -> Self {
        Self::prepare_with_crossovers(
            f,
            base_xs,
            DEFAULT_MULTIPOINT_CROSSOVER_B,
            DEFAULT_BATCH_SUBPRODUCT_CROSSOVER_B,
        )
    }

    pub fn prepare_with_crossovers(
        f: Fp64,
        base_xs: &[u64],
        multipoint_crossover_b: usize,
        batch_crossover_b: usize,
    ) -> Self {
        let mut xs: Vec<u64> = base_xs.iter().map(|&x| x % f.p).collect();
        xs.sort_unstable();
        xs.dedup();
        let xs_mont: Vec<u64> = xs.iter().map(|&x| f.to_mont(x)).collect();
        let max_x = xs.last().copied().unwrap_or(0);
        let (dense_membership, sparse_membership) = if !xs.is_empty() && max_x <= 262_144 {
            let words = (max_x as usize / 64) + 1;
            let mut bits = vec![0u64; words];
            for &x in &xs {
                bits[x as usize >> 6] |= 1u64 << (x & 63);
            }
            (bits, HashSet::new())
        } else {
            (Vec::new(), xs.iter().copied().collect())
        };

        let mut levels: Vec<Vec<Vec<u64>>> = Vec::new();
        if !xs_mont.is_empty() {
            let lvl0: Vec<Vec<u64>> = xs_mont.iter().map(|&xm| vec![f.neg(xm), f.one]).collect();
            levels.push(lvl0);
            while levels.last().unwrap().len() > 1 {
                let prev = levels.last().unwrap();
                let mut next = Vec::with_capacity(prev.len().div_ceil(2));
                for pair in prev.chunks(2) {
                    if pair.len() == 2 {
                        let mut prod = Vec::new();
                        poly_mul_mont(&f, &pair[0], &pair[1], &mut prod);
                        next.push(prod);
                    } else {
                        next.push(pair[0].clone());
                    }
                }
                levels.push(next);
            }
        }
        let inv_two_mont = f.inv(f.add(f.one, f.one));
        Self {
            f,
            xs,
            xs_mont,
            levels,
            inv_two_mont,
            dense_membership,
            max_x,
            sparse_membership,
            multipoint_crossover_b,
            batch_crossover_b,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.xs.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.xs.is_empty()
    }

    /// Monic factor-base vanishing polynomial `P_V(X)` in Montgomery form.
    pub fn root_poly_mont(&self) -> &[u64] {
        self.levels
            .last()
            .and_then(|lvl| lvl.first())
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    #[inline]
    pub fn contains_x(&self, x_canonical: u64) -> bool {
        if self.xs.is_empty() || x_canonical > self.max_x {
            return false;
        }
        if !self.dense_membership.is_empty() {
            (self.dense_membership[x_canonical as usize >> 6] >> (x_canonical & 63)) & 1 == 1
        } else {
            self.sparse_membership.contains(&x_canonical)
        }
    }

    /// Evaluate `poly_canonical` at every `x in V`, writing canonical values
    /// aligned with `self.xs` into `out_canonical`.
    pub fn eval_all(&self, poly_canonical: &[u64], out_canonical: &mut Vec<u64>) {
        out_canonical.clear();
        let b = self.xs.len();
        if b == 0 {
            return;
        }
        let mut poly_mont: Vec<u64> = poly_canonical
            .iter()
            .map(|&c| self.f.to_mont(c % self.f.p))
            .collect();
        poly_trim_mont(&mut poly_mont);
        if poly_mont.is_empty() {
            out_canonical.resize(b, 0);
            return;
        }
        let deg = poly_mont.len() - 1;
        out_canonical.resize(b, 0);
        if b < self.multipoint_crossover_b || deg <= MULTIPOINT_LEAF_WIDTH {
            for (i, &xm) in self.xs_mont.iter().enumerate() {
                let val = poly_eval_horner_mont(&self.f, &poly_mont, xm);
                out_canonical[i] = self.f.from_mont(val);
            }
            return;
        }
        let mut rem_u128 = Vec::with_capacity(poly_mont.len().max(b) + 2);
        let top_level = self.levels.len() - 1;
        let mut root_rem = Vec::new();
        poly_rem_monic_mont(
            &self.f,
            &poly_mont,
            &self.levels[top_level][0],
            &mut rem_u128,
            &mut root_rem,
        );
        self.eval_subtree_mont(top_level, 0, 0, b, &root_rem, &mut rem_u128, out_canonical);
    }

    #[allow(clippy::too_many_arguments)]
    fn eval_subtree_mont(
        &self,
        level: usize,
        node_idx: usize,
        leaf_lo: usize,
        leaf_hi: usize,
        poly_mont: &[u64],
        rem_u128: &mut Vec<u128>,
        out_canonical: &mut [u64],
    ) {
        let span = leaf_hi - leaf_lo;
        if span == 0 {
            return;
        }
        if poly_mont.is_empty() {
            out_canonical[leaf_lo..leaf_hi].fill(0);
            return;
        }
        if span <= MULTIPOINT_LEAF_WIDTH
            || poly_mont.len() <= MULTIPOINT_LEAF_WIDTH + 1
            || level == 0
        {
            for idx in leaf_lo..leaf_hi {
                let v = poly_eval_horner_mont(&self.f, poly_mont, self.xs_mont[idx]);
                out_canonical[idx] = self.f.from_mont(v);
            }
            return;
        }
        let child_level = level - 1;
        let left_node = 2 * node_idx;
        let right_node = left_node + 1;
        let left_span = 1usize << child_level;
        let mid = (leaf_lo + left_span).min(leaf_hi);

        let mut left_rem = Vec::new();
        poly_rem_monic_mont(
            &self.f,
            poly_mont,
            &self.levels[child_level][left_node],
            rem_u128,
            &mut left_rem,
        );
        self.eval_subtree_mont(
            child_level,
            left_node,
            leaf_lo,
            mid,
            &left_rem,
            rem_u128,
            out_canonical,
        );

        if mid < leaf_hi {
            let mut right_rem = Vec::new();
            poly_rem_monic_mont(
                &self.f,
                poly_mont,
                &self.levels[child_level][right_node],
                rem_u128,
                &mut right_rem,
            );
            self.eval_subtree_mont(
                child_level,
                right_node,
                mid,
                leaf_hi,
                &right_rem,
                rem_u128,
                out_canonical,
            );
        }
    }

    /// Find all sorted distinct roots of `poly_canonical` inside `V`, gated so
    /// small-`B` and low-degree single queries never regress.
    pub fn roots_in_base(&self, poly_canonical: &[u64]) -> Vec<u64> {
        let b = self.xs.len();
        if b == 0 {
            return Vec::new();
        }
        let mut poly_mont: Vec<u64> = poly_canonical
            .iter()
            .map(|&c| self.f.to_mont(c % self.f.p))
            .collect();
        poly_trim_mont(&mut poly_mont);
        if poly_mont.len() <= 1 {
            return Vec::new();
        }
        let deg = poly_mont.len() - 1;
        if deg == 1 {
            let inv = self.f.inv(poly_mont[1]);
            let r = self.f.from_mont(self.f.neg(self.f.mul(poly_mont[0], inv)));
            return if self.contains_x(r) {
                vec![r]
            } else {
                Vec::new()
            };
        }
        let p_bits = (64 - self.f.p.leading_zeros()) as usize;
        let cz_est_ops = deg * deg * p_bits * 2;
        let horner_ops = b * deg;
        // For high-degree single polynomials over large V, use the remainder tree.
        if b >= self.multipoint_crossover_b && deg > MULTIPOINT_LEAF_WIDTH && cz_est_ops > b * 12 {
            let mut vals = Vec::with_capacity(b);
            self.eval_all(poly_canonical, &mut vals);
            let mut roots = Vec::new();
            for (i, &v) in vals.iter().enumerate() {
                if v == 0 {
                    roots.push(self.xs[i]);
                }
            }
            return roots;
        }
        // Small B: direct cache-hot Horner scan over V beats O(d^2 log p) squarings.
        if horner_ops <= cz_est_ops {
            let mut roots = Vec::new();
            for (i, &xm) in self.xs_mont.iter().enumerate() {
                if poly_eval_horner_mont(&self.f, &poly_mont, xm) == 0 {
                    roots.push(self.xs[i]);
                }
            }
            return roots;
        }
        // Single low-degree query on large B: native Fp64 CZ + O(1) V-filter.
        let mut roots = find_roots_fp64_cz(&self.f, poly_canonical);
        roots.retain(|&r| self.contains_x(r));
        roots
    }

    /// Batched factor-base root extraction across candidate polynomials
    /// `polys_canonical` (`coeffs[i]` = coeff of `X^i`).
    ///
    /// - When `B < self.batch_crossover_b` (small factor base), uses direct
    ///   `Fp64` Horner evaluation over `V` (with zero tree overhead) so small-`B`
    ///   cells never regress.
    /// - When `B >= self.batch_crossover_b`, normalizes candidate polynomials in
    ///   one shared `Fp64::batch_inv`, builds a candidate product tree `Q(X)`
    ///   chunked so `deg(Q) >= B + 1` (making top-level `P_V mod Q = P_V` free),
    ///   propagates `P_V(X)` down `Q`'s remainder tree to obtain
    ///   `r_k(X) = P_V(X) mod g_k(X)`, computes `h_k = gcd(g_k, r_k)` in
    ///   lock-step via `Fp64::batch_inv`, and extracts the exact roots in `V`.
    pub fn roots_in_base_batch(
        &self,
        polys_canonical: &[Vec<u64>],
        scratch: &mut SubproductBatchScratch,
    ) -> Vec<Vec<u64>> {
        let mut out = vec![Vec::new(); polys_canonical.len()];
        let b = self.xs.len();
        if b == 0 || polys_canonical.is_empty() {
            return out;
        }
        if b < self.batch_crossover_b {
            for (idx, poly) in polys_canonical.iter().enumerate() {
                out[idx] = self.roots_in_base(poly);
            }
            return out;
        }

        // Convert candidates to Montgomery form and batch-invert leading coefficients.
        scratch.gcd_a.clear();
        scratch.gcd_a.reserve(polys_canonical.len());
        scratch.denoms.clear();
        scratch.denom_idx.clear();
        let mut active_orig_idx: Vec<usize> = Vec::with_capacity(polys_canonical.len());

        for (orig_i, poly_c) in polys_canonical.iter().enumerate() {
            let mut pm: Vec<u64> = poly_c
                .iter()
                .map(|&c| self.f.to_mont(c % self.f.p))
                .collect();
            poly_trim_mont(&mut pm);
            if pm.len() <= 1 {
                continue;
            }
            let lead = *pm.last().unwrap();
            let d_idx = if lead == self.f.one {
                usize::MAX
            } else {
                let k = scratch.denoms.len();
                scratch.denoms.push(lead);
                k
            };
            scratch.denom_idx.push(d_idx);
            scratch.gcd_a.push(pm);
            active_orig_idx.push(orig_i);
        }
        if active_orig_idx.is_empty() {
            return out;
        }
        if !scratch.denoms.is_empty() {
            self.f.batch_inv(&mut scratch.denoms, &mut scratch.prefix);
        }
        for (k, pm) in scratch.gcd_a.iter_mut().enumerate() {
            let d_idx = scratch.denom_idx[k];
            if d_idx != usize::MAX {
                let inv = scratch.denoms[d_idx];
                let last = pm.len() - 1;
                for c in &mut pm[..last] {
                    *c = self.f.mul(*c, inv);
                }
                pm[last] = self.f.one;
            }
        }

        // Process active monic polynomials in chunks whose total degree exceeds B
        // (so top-level `P_V mod Q_root = P_V` costs zero divisions) while keeping
        // the candidate tree inside L2 cache.
        let pv_mont = self.root_poly_mont();
        let mut rng_state = self.f.p ^ 0x517C_C1B7_2722_0A95;
        let mut cursor = 0usize;
        while cursor < active_orig_idx.len() {
            let start = cursor;
            let mut sum_deg = 0usize;
            let target_deg = (b + 1).max(128);
            while cursor < active_orig_idx.len() && (sum_deg < target_deg || cursor == start) {
                sum_deg += scratch.gcd_a[cursor].len() - 1;
                cursor += 1;
            }
            let count = cursor - start;
            if count <= 2 {
                for pos in start..cursor {
                    let orig_i = active_orig_idx[pos];
                    out[orig_i] = self.roots_in_base(&polys_canonical[orig_i]);
                }
                continue;
            }

            // Build candidate binary product tree `cand_levels`.
            scratch.cand_levels.clear();
            let lvl0: Vec<Vec<u64>> = scratch.gcd_a[start..cursor].to_vec();
            scratch.cand_levels.push(lvl0);
            while scratch.cand_levels.last().unwrap().len() > 1 {
                let prev = scratch.cand_levels.last().unwrap();
                let mut next = Vec::with_capacity(prev.len().div_ceil(2));
                for pair in prev.chunks(2) {
                    if pair.len() == 2 {
                        let mut prod = Vec::new();
                        poly_mul_mont(&self.f, &pair[0], &pair[1], &mut prod);
                        next.push(prod);
                    } else {
                        next.push(pair[0].clone());
                    }
                }
                scratch.cand_levels.push(next);
            }

            // Propagate `P_V(X)` top-down through `cand_levels`.
            let top = scratch.cand_levels.len() - 1;
            let mut root_rem = Vec::new();
            poly_rem_monic_mont(
                &self.f,
                pv_mont,
                &scratch.cand_levels[top][0],
                &mut scratch.rem_u128,
                &mut root_rem,
            );
            scratch.rem_curr.clear();
            scratch.rem_curr.push(root_rem);
            for lvl in (0..top).rev() {
                let nodes = &scratch.cand_levels[lvl];
                scratch.rem_next.clear();
                scratch.rem_next.reserve(nodes.len());
                for (parent_i, parent_rem) in scratch.rem_curr.iter().enumerate() {
                    let left_i = 2 * parent_i;
                    let mut lrem = Vec::new();
                    poly_rem_monic_mont(
                        &self.f,
                        parent_rem,
                        &nodes[left_i],
                        &mut scratch.rem_u128,
                        &mut lrem,
                    );
                    scratch.rem_next.push(lrem);
                    let right_i = left_i + 1;
                    if right_i < nodes.len() {
                        let mut rrem = Vec::new();
                        poly_rem_monic_mont(
                            &self.f,
                            parent_rem,
                            &nodes[right_i],
                            &mut scratch.rem_u128,
                            &mut rrem,
                        );
                        scratch.rem_next.push(rrem);
                    }
                }
                std::mem::swap(&mut scratch.rem_curr, &mut scratch.rem_next);
            }

            // Lock-step batched Euclidean GCD across all `count` leaves:
            // `h_k = gcd(g_k, r_k)` with shared `Fp64::batch_inv` per GCD step!
            let mut a_lanes: Vec<Vec<u64>> = scratch.cand_levels[0].clone();
            let mut b_lanes: Vec<Vec<u64>> = std::mem::take(&mut scratch.rem_curr);
            loop {
                scratch.denoms.clear();
                scratch.denom_idx.clear();
                let mut any_active = false;
                for bl in &mut b_lanes {
                    poly_trim_mont(bl);
                    if let Some(&lead) = bl.last() {
                        any_active = true;
                        if lead == self.f.one {
                            scratch.denom_idx.push(usize::MAX);
                        } else {
                            let idx = scratch.denoms.len();
                            scratch.denoms.push(lead);
                            scratch.denom_idx.push(idx);
                        }
                    } else {
                        scratch.denom_idx.push(usize::MAX - 1);
                    }
                }
                if !any_active {
                    break;
                }
                if !scratch.denoms.is_empty() {
                    self.f.batch_inv(&mut scratch.denoms, &mut scratch.prefix);
                }
                for lane in 0..count {
                    let tag = scratch.denom_idx[lane];
                    if tag == usize::MAX - 1 {
                        continue;
                    }
                    if tag != usize::MAX {
                        let inv = scratch.denoms[tag];
                        let bl = &mut b_lanes[lane];
                        let last = bl.len() - 1;
                        for c in &mut bl[..last] {
                            *c = self.f.mul(*c, inv);
                        }
                        bl[last] = self.f.one;
                    }
                    let mut next_rem = Vec::new();
                    poly_rem_monic_mont(
                        &self.f,
                        &a_lanes[lane],
                        &b_lanes[lane],
                        &mut scratch.rem_u128,
                        &mut next_rem,
                    );
                    a_lanes[lane] = std::mem::take(&mut b_lanes[lane]);
                    b_lanes[lane] = next_rem;
                }
            }

            // Extract roots from each monic `h_k = a_lanes[lane]` (every factor of
            // `h_k` divides `P_V(X)` and therefore splits completely in `V`).
            for (lane, hk) in a_lanes.iter().enumerate() {
                let orig_i = active_orig_idx[start + lane];
                let deg_h = hk.len().saturating_sub(1);
                if deg_h == 0 {
                    continue;
                }
                let dst = &mut out[orig_i];
                if deg_h == 1 {
                    dst.push(self.f.from_mont(self.f.neg(hk[0])));
                } else if deg_h == 2 {
                    let half_c1 = self.f.mul(hk[1], self.inv_two_mont);
                    let disc = self.f.sub(self.f.sqr(half_c1), hk[0]);
                    if let Some(s) = self.f.sqrt(disc) {
                        let r0 = self.f.from_mont(self.f.sub(s, half_c1));
                        let r1 = self.f.from_mont(self.f.neg(self.f.add(s, half_c1)));
                        dst.push(r0);
                        if r1 != r0 {
                            dst.push(r1);
                        }
                        dst.sort_unstable();
                    }
                } else {
                    split_totally_linear_monic_mont(
                        &self.f,
                        hk,
                        self.inv_two_mont,
                        &mut scratch.rem_u128,
                        &mut rng_state,
                        dst,
                    );
                }
            }
        }
        out
    }
}

/// Decomposition strategy for [`collect_relations_lockstep`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LockStepMode {
    /// `O(1)` lookup per walker in `fb` (1-term) and `PairTable` (2-term),
    /// sharing one `Fp64::batch_inv` across all `W` walkers' `R_w <- R_w + G`
    /// steps each round.
    PairTable2,
    /// Direct subtraction sweep (`R_w ∓ F_i` across all `W` walkers and `B`
    /// factor-base points, optionally matching `PairTable` for 3-term rows)
    /// sharing `Fp64::batch_inv` over both subtraction denominators and
    /// walker step denominators.
    DirectSweep,
}

/// Reusable SoA buffers for [`collect_relations_lockstep`]: zero heap
/// allocations occur inside the target-collection loop once sized for `W`.
#[derive(Default, Clone, Debug)]
pub struct LockStepCollectScratch {
    rx: Vec<u64>,
    ry: Vec<u64>,
    ra: Vec<u64>,
    rb: Vec<u64>,
    denoms: Vec<u64>,
    prefix: Vec<u64>,
    degenerate: Vec<bool>,
    found: Vec<SmallEntries>,
    lane_seen: HashSet<SmallEntries>,
    global_seen: HashSet<(u64, u64, SmallEntries)>,
}

/// Lock-step multi-walker relation collector (`W = 64..256` walkers):
/// - Initializes and restarts walkers via cheap stride additions `T_0 + k*S`
///   (falling back to `combine` only on identity/zero-`b` hits);
/// - Reuses [`LockStepCollectScratch`] across rounds and calls with zero
///   per-target heap allocations (`SmallEntries` stack representation);
/// - Batches all `W` walker advances `R_w <- R_w + G` in a single
///   [`Fp64::batch_inv`] per round (and batches `W * B` subtraction
///   denominators in [`LockStepMode::DirectSweep`]).
#[allow(clippy::too_many_arguments)]
pub fn collect_relations_lockstep(
    curve: &FastCurve,
    g: Pt,
    q: Pt,
    fb: &FastFactorBase,
    pairs: Option<&PairTable>,
    needed: usize,
    max_targets: u64,
    walkers: usize,
    mode: LockStepMode,
    seed: u64,
    scratch: &mut LockStepCollectScratch,
) -> (Vec<FastRelation>, CollectStats) {
    let f = curve.f;
    let n = curve.n;
    let w_len = walkers.max(1);
    let b_len = fb.len();
    let mut rng = StdRng::seed_from_u64(seed ^ 0x10C4_57E9_2026_1119);
    let mut rels: Vec<FastRelation> = Vec::with_capacity(needed);
    let mut stats = CollectStats::default();
    if b_len == 0 || needed == 0 || max_targets == 0 {
        return (rels, stats);
    }

    // Stride-based walker start generator: two scalar combines initially,
    // then 1 affine addition per new start/restart!
    let fresh_combine = |rng: &mut StdRng| -> (u64, u64, Pt) {
        loop {
            let a = rng.gen_range(1..n);
            let b = rng.gen_range(1..n);
            if let Some(pt) = curve.combine(g, q, a, b) {
                return (a, b, pt);
            }
        }
    };
    let (sa, sb, stride_pt) = fresh_combine(&mut rng);
    let (mut cur_a, mut cur_b, mut cur_pt) = fresh_combine(&mut rng);
    let mut next_start = |rng: &mut StdRng| -> (u64, u64, Pt) {
        loop {
            cur_a = add_mod(cur_a, sa, n);
            cur_b = add_mod(cur_b, sb, n);
            if cur_b != 0 {
                if let Some(pt) = curve.add(Some(cur_pt), Some(stride_pt)) {
                    cur_pt = pt;
                    return (cur_a, cur_b, pt);
                }
            }
            let (na, nb, npt) = fresh_combine(rng);
            cur_a = na;
            cur_b = nb;
            cur_pt = npt;
            return (na, nb, npt);
        }
    };

    scratch.rx.clear();
    scratch.ry.clear();
    scratch.ra.clear();
    scratch.rb.clear();
    scratch.rx.reserve(w_len);
    scratch.ry.reserve(w_len);
    scratch.ra.reserve(w_len);
    scratch.rb.reserve(w_len);
    for _ in 0..w_len {
        let (a, b, pt) = next_start(&mut rng);
        scratch.rx.push(pt.x);
        scratch.ry.push(pt.y);
        scratch.ra.push(a);
        scratch.rb.push(b);
    }
    scratch.global_seen.clear();
    scratch.global_seen.reserve(needed * 2);

    let max_pair_x = pairs.map(PairTable::max_x).unwrap_or(0);

    'rounds: while rels.len() < needed && stats.targets < max_targets {
        let rem_budget = (max_targets - stats.targets) as usize;
        let active_w = w_len.min(rem_budget);

        match mode {
            LockStepMode::PairTable2 => {
                let pairs_ref = pairs.expect("LockStepMode::PairTable2 requires PairTable");
                for lane in 0..active_w {
                    stats.targets += 1;
                    let r = Pt {
                        x: scratch.rx[lane],
                        y: scratch.ry[lane],
                    };
                    let xc = f.from_mont(r.x);
                    scratch.found.clear();
                    if let Some(j) = fb.lookup(xc) {
                        let s = if r.y == fb.pts[j].y { 1i8 } else { -1 };
                        scratch.found.push(SmallEntries::one(j as u32, s));
                    }
                    if xc <= max_pair_x {
                        for &(_, j, k_raw) in pairs_ref.lookup(xc) {
                            let sigma = if k_raw & PAIR_DIFF_BIT != 0 { -1i8 } else { 1 };
                            let k = (k_raw & !PAIR_DIFF_BIT) as usize;
                            let fk = fb.pts[k];
                            let fk = if sigma < 0 { curve.neg(fk) } else { fk };
                            let Some(pjk) = curve.add(Some(fb.pts[j as usize]), Some(fk)) else {
                                continue;
                            };
                            let tau = if r.y == pjk.y { 1i8 } else { -1 };
                            scratch
                                .found
                                .push(SmallEntries::two(j, tau, k as u32, tau * sigma));
                        }
                    }
                    if !scratch.found.is_empty() {
                        scratch.lane_seen.clear();
                        let mut productive = false;
                        for idx in 0..scratch.found.len() {
                            let merged = scratch.found[idx];
                            if merged.is_empty()
                                || !scratch.lane_seen.insert(merged)
                                || !scratch.global_seen.insert((
                                    scratch.ra[lane],
                                    scratch.rb[lane],
                                    merged,
                                ))
                            {
                                continue;
                            }
                            if !relation_holds(curve, fb, r, merged.as_slice()) {
                                stats.rejected += 1;
                                continue;
                            }
                            match merged.len {
                                1 => stats.rows_one += 1,
                                2 => stats.rows_two += 1,
                                _ => stats.rows_three += 1,
                            }
                            rels.push(FastRelation {
                                a: scratch.ra[lane],
                                b: scratch.rb[lane],
                                entries: merged.to_vec(),
                            });
                            productive = true;
                            break;
                        }
                        if productive {
                            if rels.len() >= needed {
                                break 'rounds;
                            }
                            let (na, nb, npt) = next_start(&mut rng);
                            scratch.rx[lane] = npt.x;
                            scratch.ry[lane] = npt.y;
                            scratch.ra[lane] = na;
                            scratch.rb[lane] = nb;
                        }
                    }
                }
            }
            LockStepMode::DirectSweep => {
                // Batch all `active_w * b_len` subtraction denominators `(R_w.x - F_i.x)`
                // in cache-hot chunks of up to 1024 entries!
                let lanes_per_chunk = (1024 / b_len.max(1)).clamp(1, active_w);
                let mut lane_base = 0usize;
                while lane_base < active_w {
                    let chunk_w = (active_w - lane_base).min(lanes_per_chunk);
                    let total_den = chunk_w * b_len;
                    scratch.denoms.clear();
                    scratch.degenerate.clear();
                    scratch.denoms.reserve(total_den);
                    scratch.degenerate.reserve(total_den);
                    for off in 0..chunk_w {
                        let rx = scratch.rx[lane_base + off];
                        for i in 0..b_len {
                            let dd = f.sub(rx, fb.pts[i].x);
                            let deg = dd == 0;
                            scratch.degenerate.push(deg);
                            scratch.denoms.push(if deg { f.one } else { dd });
                        }
                    }
                    f.batch_inv(&mut scratch.denoms, &mut scratch.prefix);

                    for off in 0..chunk_w {
                        let lane = lane_base + off;
                        stats.targets += 1;
                        let r = Pt {
                            x: scratch.rx[lane],
                            y: scratch.ry[lane],
                        };
                        let row_off = off * b_len;
                        scratch.found.clear();
                        for i in 0..b_len {
                            let fi = fb.pts[i];
                            if scratch.degenerate[row_off + i] {
                                let s = if r.y == fi.y { 1i8 } else { -1 };
                                scratch.found.push(SmallEntries::one(i as u32, s));
                                continue;
                            }
                            let inv = scratch.denoms[row_off + i];
                            let lam_minus = f.mul(f.add(r.y, fi.y), inv);
                            let lam_plus = f.mul(f.sub(r.y, fi.y), inv);
                            for (eps, lam) in [(1i8, lam_minus), (-1i8, lam_plus)] {
                                let x3 = f.sub(f.sub(f.sqr(lam), r.x), fi.x);
                                let xc = f.from_mont(x3);
                                let mut y3: Option<u64> = None;
                                if let Some(j) = fb.lookup(xc) {
                                    if j >= i {
                                        let y = f.sub(f.mul(lam, f.sub(r.x, x3)), r.y);
                                        y3 = Some(y);
                                        let tau = if y == fb.pts[j].y { 1i8 } else { -1 };
                                        scratch
                                            .found
                                            .push(SmallEntries::two(i as u32, eps, j as u32, tau));
                                    }
                                }
                                if let Some(pairs_ref) = pairs {
                                    if xc <= max_pair_x {
                                        for &(_, j, k_raw) in pairs_ref.lookup(xc) {
                                            if (j as usize) < i {
                                                continue;
                                            }
                                            let sigma =
                                                if k_raw & PAIR_DIFF_BIT != 0 { -1i8 } else { 1 };
                                            let k = (k_raw & !PAIR_DIFF_BIT) as usize;
                                            let fk = fb.pts[k];
                                            let fk = if sigma < 0 { curve.neg(fk) } else { fk };
                                            let Some(pjk) =
                                                curve.add(Some(fb.pts[j as usize]), Some(fk))
                                            else {
                                                continue;
                                            };
                                            let y = *y3.get_or_insert_with(|| {
                                                f.sub(f.mul(lam, f.sub(r.x, x3)), r.y)
                                            });
                                            let tau = if y == pjk.y { 1i8 } else { -1 };
                                            scratch.found.push(SmallEntries::three(
                                                i as u32,
                                                eps,
                                                j,
                                                tau,
                                                k as u32,
                                                tau * sigma,
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                        if !scratch.found.is_empty() {
                            scratch.lane_seen.clear();
                            let mut productive = false;
                            for idx in 0..scratch.found.len() {
                                let merged = scratch.found[idx];
                                if merged.is_empty()
                                    || !scratch.lane_seen.insert(merged)
                                    || !scratch.global_seen.insert((
                                        scratch.ra[lane],
                                        scratch.rb[lane],
                                        merged,
                                    ))
                                {
                                    continue;
                                }
                                if !relation_holds(curve, fb, r, merged.as_slice()) {
                                    stats.rejected += 1;
                                    continue;
                                }
                                match merged.len {
                                    1 => stats.rows_one += 1,
                                    2 => stats.rows_two += 1,
                                    _ => stats.rows_three += 1,
                                }
                                rels.push(FastRelation {
                                    a: scratch.ra[lane],
                                    b: scratch.rb[lane],
                                    entries: merged.to_vec(),
                                });
                                productive = true;
                                break;
                            }
                            if productive {
                                if rels.len() >= needed {
                                    break 'rounds;
                                }
                                let (na, nb, npt) = next_start(&mut rng);
                                scratch.rx[lane] = npt.x;
                                scratch.ry[lane] = npt.y;
                                scratch.ra[lane] = na;
                                scratch.rb[lane] = nb;
                            }
                        }
                    }
                    lane_base += chunk_w;
                }
            }
        }

        if stats.targets >= max_targets {
            break;
        }

        // Advance all `w_len` walkers simultaneously: `R_w <- R_w + G` with ONE
        // shared `Fp64::batch_inv` and zero heap allocations!
        scratch.denoms.clear();
        scratch.degenerate.clear();
        scratch.denoms.reserve(w_len);
        scratch.degenerate.reserve(w_len);
        for lane in 0..w_len {
            let dd = f.sub(g.x, scratch.rx[lane]);
            let deg = dd == 0;
            scratch.degenerate.push(deg);
            scratch.denoms.push(if deg { f.one } else { dd });
        }
        f.batch_inv(&mut scratch.denoms, &mut scratch.prefix);
        for lane in 0..w_len {
            if scratch.degenerate[lane] {
                let r = Pt {
                    x: scratch.rx[lane],
                    y: scratch.ry[lane],
                };
                match curve.add(Some(r), Some(g)) {
                    Some(nr) => {
                        scratch.rx[lane] = nr.x;
                        scratch.ry[lane] = nr.y;
                        scratch.ra[lane] = add_mod(scratch.ra[lane], 1, n);
                    }
                    None => {
                        let (na, nb, npt) = next_start(&mut rng);
                        scratch.rx[lane] = npt.x;
                        scratch.ry[lane] = npt.y;
                        scratch.ra[lane] = na;
                        scratch.rb[lane] = nb;
                    }
                }
            } else {
                let rx = scratch.rx[lane];
                let ry = scratch.ry[lane];
                let lam = f.mul(f.sub(g.y, ry), scratch.denoms[lane]);
                let x3 = f.sub(f.sub(f.sqr(lam), rx), g.x);
                let y3 = f.sub(f.mul(lam, f.sub(rx, x3)), ry);
                scratch.rx[lane] = x3;
                scratch.ry[lane] = y3;
                scratch.ra[lane] = add_mod(scratch.ra[lane], 1, n);
            }
        }
    }

    stats.relations = rels.len();
    (rels, stats)
}

/// End-to-end single-target discrete-log solver using [`collect_relations_lockstep`]
/// (`W = 64..256` walkers, zero-allocation scratch reuse) and fail-closed
/// `[d]G == Q` verification.
#[allow(clippy::too_many_arguments)]
pub fn ic_solve_lockstep(
    curve: &FastCurve,
    g: Pt,
    q: Pt,
    fb_size: usize,
    extra_relations: usize,
    walkers: usize,
    mode: LockStepMode,
    max_targets: u64,
    seed: u64,
) -> Option<(u64, IcReport)> {
    let t0 = Instant::now();
    let fb = FastFactorBase::build(curve, fb_size);
    let factor_base_ms = t0.elapsed().as_secs_f64() * 1e3;
    if fb.is_empty() {
        return None;
    }
    let t1 = Instant::now();
    let pairs = match mode {
        LockStepMode::PairTable2 => Some(PairTable::build(curve, &fb)),
        LockStepMode::DirectSweep => None,
    };
    let pair_table_ms = t1.elapsed().as_secs_f64() * 1e3;
    let needed = fb.len() + extra_relations;
    let t2 = Instant::now();
    let mut scratch = LockStepCollectScratch::default();
    let (mut rels, mut stats) = collect_relations_lockstep(
        curve,
        g,
        q,
        &fb,
        pairs.as_ref(),
        needed,
        max_targets,
        walkers,
        mode,
        seed,
        &mut scratch,
    );
    let mut relations_ms = t2.elapsed().as_secs_f64() * 1e3;
    if rels.len() < needed {
        return None;
    }
    let mut linear_algebra_ms = 0.0;
    let mut topup_epoch = 1u64;
    let topup_step = (fb.len() / 4).clamp(16, 64);
    let d = loop {
        let t3 = Instant::now();
        let sol = solve_relations(curve, fb.len(), &rels);
        linear_algebra_ms += t3.elapsed().as_secs_f64() * 1e3;
        if let Some(d_found) = sol {
            break d_found;
        }
        if stats.targets >= max_targets || topup_epoch > 8 {
            return None;
        }
        let rem_budget = max_targets - stats.targets;
        let t_more = Instant::now();
        let (more_rels, more_stats) = collect_relations_lockstep(
            curve,
            g,
            q,
            &fb,
            pairs.as_ref(),
            topup_step,
            rem_budget,
            walkers,
            mode,
            seed.wrapping_add(topup_epoch.wrapping_mul(0x9E37_79B9_7F4A_7C15)),
            &mut scratch,
        );
        relations_ms += t_more.elapsed().as_secs_f64() * 1e3;
        stats.targets += more_stats.targets;
        stats.rows_one += more_stats.rows_one;
        stats.rows_two += more_stats.rows_two;
        stats.rows_three += more_stats.rows_three;
        stats.rejected += more_stats.rejected;
        if more_rels.is_empty() {
            return None;
        }
        rels.extend(more_rels);
        stats.relations = rels.len();
        topup_epoch += 1;
    };
    let t4 = Instant::now();
    if curve.mul(Some(g), d) != Some(q) {
        return None;
    }
    let verify_ms = t4.elapsed().as_secs_f64() * 1e3;
    Some((
        d,
        IcReport {
            factor_base_ms,
            pair_table_ms,
            relations_ms,
            linear_algebra_ms,
            verify_ms,
            total_ms: t0.elapsed().as_secs_f64() * 1e3,
            factor_base_size: fb.len(),
            pair_table_entries: pairs.as_ref().map(PairTable::len).unwrap_or(0),
            relations: rels.len(),
            targets: stats.targets,
            rows_one: stats.rows_one,
            rows_two: stats.rows_two,
            rows_three: stats.rows_three,
        },
    ))
}

// ── Point counting and bench-curve generation ─────────────────────────

/// Order of `g` found as the unique `N` in the Hasse interval with
/// `[N]g = O`, by baby-step giant-step on `t = p + 1 − N`.  Returns
/// `None` when the interval holds several candidates (small-order `g`)
/// or the match is not unique — callers only accept prime results.
pub fn point_order_hasse(curve: &FastCurve, g: Pt) -> Option<u64> {
    let p = curve.f.p;
    let s = (p as f64).sqrt().ceil() as u64 + 1;
    let lo: i128 = -(2 * s as i128);
    let hi: i128 = 2 * s as i128;
    let width = (hi - lo + 1) as u64;
    let m = ((width as f64).sqrt().ceil() as u64).max(2);
    // Baby steps: x([j]G) → j, with y for sign resolution.
    let mut baby: HashMap<u64, (u64, u64)> = HashMap::with_capacity(m as usize);
    let mut cur: Option<Pt> = None;
    for j in 0..m {
        if let Some(pt) = cur {
            baby.entry(pt.x).or_insert((j, pt.y));
        }
        cur = curve.add(cur, Some(g));
    }
    let q0 = curve.mul(Some(g), p + 1); // [p+1]G = [t]G
    let m_g = curve.mul(Some(g), m);
    let neg_m_g = m_g.map(|pt| curve.neg(pt));
    // Y_i = Q0 − [lo + i·m]G, i = 0 ..= width/m.
    let lo_g = {
        let mag = lo.unsigned_abs() as u64;
        curve.mul(Some(g), mag).map(|pt| curve.neg(pt)) // [lo]G with lo < 0
    };
    let mut y = curve.add(q0, lo_g.map(|pt| curve.neg(pt))); // Q0 − [lo]G
    let mut candidates: Vec<i128> = Vec::new();
    let iters = width / m + 1;
    for i in 0..=iters {
        match y {
            None => candidates.push(lo + (i as i128) * m as i128),
            Some(pt) => {
                if let Some(&(j, by)) = baby.get(&pt.x) {
                    let t = if by == pt.y {
                        lo + (i as i128) * m as i128 + j as i128
                    } else {
                        lo + (i as i128) * m as i128 - j as i128
                    };
                    candidates.push(t);
                }
            }
        }
        y = curve.add(y, neg_m_g);
    }
    candidates.sort_unstable();
    candidates.dedup();
    candidates.retain(|&t| {
        t >= lo && t <= hi && curve.mul(Some(g), (p as i128 + 1 - t) as u64).is_none()
    });
    if candidates.len() != 1 {
        return None;
    }
    Some((p as i128 + 1 - candidates[0]) as u64)
}

/// Deterministic `a = −3` prime-order bench curve at `bits` bits in the
/// requested residue class of `p mod 4`: `p` the largest such prime
/// below `2^bits`, `b` the smallest positive value giving prime order,
/// `G` the smallest-x point. Reproduces the curve and the generator's
/// negation class in [`crate::cryptanalysis::research_bench::bench_curves_a_minus_3`].
pub fn find_a3_curve(bits: u32, residue_mod_4: u64) -> Option<(FastCurve, Pt)> {
    assert!((8..=MAX_BITS).contains(&bits));
    let mut p = (1u64 << bits) - 1;
    while !(p % 4 == residue_mod_4 && is_prime_u64(p)) {
        p -= 1;
    }
    let a = p - 3;
    for b in 1u64..1_000_000 {
        // Singular if 4a³ + 27b² ≡ 0.
        let f = Fp64::new(p);
        let am = f.to_mont(a);
        let bm = f.to_mont(b);
        let disc = f.add(
            f.mul(f.to_mont(4), f.mul(f.sqr(am), am)),
            f.mul(f.to_mont(27), f.sqr(bm)),
        );
        if disc == 0 {
            continue;
        }
        let probe = FastCurve::new("probe", p, a, b, 1);
        let Some(g) = (1u64..10_000).find_map(|x| probe.lift_x(x)) else {
            continue;
        };
        let Some(order) = point_order_hasse(&probe, g) else {
            continue;
        };
        if !is_prime_u64(order) {
            continue;
        }
        let name = format!(
            "a3-fast-{bits}bit-{}",
            if residue_mod_4 == 3 {
                "p256class"
            } else {
                "cryptoproclass"
            }
        );
        let curve = FastCurve::new(&name, p, a, b, order);
        let g = curve.lift_x(probe.canonical(g).0)?;
        if curve.mul(Some(g), order).is_some() {
            continue;
        }
        return Some((curve, g));
    }
    None
}

#[cfg(test)]
mod tests {

    #[test]
    fn add_x_with_inv_matches_the_full_addition() {
        let (curve, g) = find_a3_curve(24, 3).expect("curve");
        let f = curve.f;
        let mut pts = vec![g];
        for k in 2..40u64 {
            pts.push(curve.mul(Some(g), k).expect("multiple"));
        }
        for a in &pts {
            for b in &pts {
                if a.x == b.x {
                    continue;
                }
                let inv = f.inv(f.sub(b.x, a.x));
                assert_eq!(
                    curve.add_x_with_inv(*a, *b, inv),
                    curve.add_with_inv(*a, *b, inv).x
                );
                let nb = curve.neg(*b);
                let inv2 = f.inv(f.sub(nb.x, a.x));
                assert_eq!(
                    curve.add_x_with_inv(*a, nb, inv2),
                    curve.add_with_inv(*a, nb, inv2).x
                );
            }
        }
    }
    use super::*;
    use crate::cryptanalysis::research_bench::bench_curves_a_minus_3;

    /// The branch-free selections of [`Fp64`] (`reduce`, `add`, `sub`)
    /// against the branchy formulas they replaced, bit for bit.
    #[test]
    fn fp64_selects_match_branchy_formulas() {
        for p in [
            5u64,
            7,
            13,
            1_000_000_007,
            2_147_483_647,
            (1 << 61) - 1,
            (1 << 62) - 57,
        ] {
            assert!(is_prime_u64(p), "{p}");
            let f = Fp64::new(p);
            let reduce = |t: u128| {
                let m = (t as u64).wrapping_mul(f.n_inv);
                let u = ((t + (m as u128) * (p as u128)) >> 64) as u64;
                if u >= p {
                    u - p
                } else {
                    u
                }
            };
            let add = |a: u64, b: u64| if a + b >= p { a + b - p } else { a + b };
            let sub = |a: u64, b: u64| if a >= b { a - b } else { a + p - b };
            let check = |a: u64, b: u64| {
                let t = a as u128 * b as u128;
                assert_eq!(f.reduce(t), reduce(t), "reduce p={p} a={a} b={b}");
                assert_eq!(f.mul(a, b), reduce(t), "mul p={p} a={a} b={b}");
                assert_eq!(f.sqr(a), reduce(a as u128 * a as u128), "sqr p={p}");
                assert_eq!(f.add(a, b), add(a, b), "add p={p} a={a} b={b}");
                assert_eq!(f.sub(a, b), sub(a, b), "sub p={p} a={a} b={b}");
            };
            let edges = [0, 1, 2, p - 2, p - 1, p / 2, p / 2 + 1];
            for &a in &edges {
                for &b in &edges {
                    check(a, b);
                }
            }
            let mut rng = StdRng::seed_from_u64(p);
            for _ in 0..20_000 {
                check(rng.gen_range(0..p), rng.gen_range(0..p));
            }
            // reduce's whole input range, t < p·2^64, not only products.
            for _ in 0..20_000 {
                let t = rng.gen_range(0..(p as u128) << 64);
                assert_eq!(f.reduce(t), reduce(t), "reduce p={p} t={t}");
            }
        }
    }

    fn ladder(bits: u32) -> (FastCurve, Pt) {
        let (_, c) = bench_curves_a_minus_3()
            .into_iter()
            .find(|(b, c)| *b == bits && c.name.contains("p256class"))
            .expect("ladder rung");
        let fc = FastCurve::from_params(&c).unwrap();
        let g = fc.from_textbook(&c.generator()).unwrap();
        (fc, g)
    }

    #[test]
    fn montgomery_matches_biguint() {
        let mut rng = StdRng::seed_from_u64(7);
        for &p in &[
            65_519u64,
            1_048_571,
            16_777_199,
            (1u64 << 61) - 1,
            4_611_686_018_427_387_847,
        ] {
            let f = Fp64::new(p);
            let pb = BigUint::from(p);
            for _ in 0..200 {
                let a = rng.gen_range(0..p);
                let b = rng.gen_range(0..p);
                let (am, bm) = (f.to_mont(a), f.to_mont(b));
                assert_eq!(f.from_mont(am), a);
                let prod = ((BigUint::from(a) * BigUint::from(b)) % &pb).to_u64_digits();
                assert_eq!(
                    f.from_mont(f.mul(am, bm)),
                    prod.first().copied().unwrap_or(0)
                );
                assert_eq!(
                    f.from_mont(f.add(am, bm)),
                    ((a as u128 + b as u128) % p as u128) as u64
                );
                assert_eq!(
                    f.from_mont(f.sub(am, bm)),
                    ((a as u128 + p as u128 - b as u128) % p as u128) as u64
                );
                if a != 0 {
                    let inv = f.inv(am);
                    assert_eq!(f.mul(inv, am), f.one);
                    let e = BigUint::from(a).modpow(&(&pb - BigUint::from(2u32)), &pb);
                    assert_eq!(f.from_mont(inv), e.to_u64_digits()[0]);
                }
                if let Some(r) = f.sqrt(am) {
                    assert_eq!(f.sqr(r), am);
                }
            }
            // Batch inversion agrees with pointwise.
            let xs: Vec<u64> = (0..37).map(|_| f.to_mont(rng.gen_range(1..p))).collect();
            let mut batched = xs.clone();
            f.batch_inv(&mut batched, &mut Vec::new());
            for (x, b) in xs.iter().zip(batched.iter()) {
                assert_eq!(f.inv(*x), *b);
            }
        }
    }

    #[test]
    fn fast_arithmetic_matches_textbook_points() {
        for bits in [16u32, 20, 24] {
            let (fc, g) = ladder(bits);
            let (_, c) = bench_curves_a_minus_3()
                .into_iter()
                .find(|(b, c)| *b == bits && c.name.contains("p256class"))
                .unwrap();
            let a_fe = c.a_fe();
            let tg = c.generator();
            assert!(fc.is_on_curve(g));
            let mut rng = StdRng::seed_from_u64(bits as u64);
            for _ in 0..20 {
                let k = rng.gen_range(1..fc.n);
                let l = rng.gen_range(1..fc.n);
                let kg = fc.mul(Some(g), k);
                let lg = fc.mul(Some(g), l);
                let sum = fc.add(kg, lg);
                let textbook = tg
                    .scalar_mul(&BigUint::from(k), &a_fe)
                    .add(&tg.scalar_mul(&BigUint::from(l), &a_fe), &a_fe);
                assert_eq!(fc.to_textbook(sum), textbook);
            }
            assert_eq!(fc.mul(Some(g), fc.n), None);
        }
    }

    /// The affine ladder `mul` used before it moved to Jacobian
    /// coordinates, kept as the reference the new one must reproduce.
    fn mul_affine_reference(fc: &FastCurve, p: Option<Pt>, mut k: u64) -> Option<Pt> {
        let mut acc: Option<Pt> = None;
        let mut base = p;
        while k > 0 {
            if k & 1 == 1 {
                acc = fc.add(acc, base);
            }
            base = fc.add(base, base);
            k >>= 1;
        }
        acc
    }

    /// Jacobian `mul` and Straus–Shamir `combine` give exactly the affine
    /// ladder's points on every point of small curves (2-torsion and
    /// points of every order included), for scalars below, at and past
    /// the group order, and on the rho ladder curves at random scalars.
    #[test]
    fn jacobian_mul_and_combine_match_the_affine_ladder() {
        for (p, a, b) in [
            (1009u64, 3u64, 7u64),
            (1013, 0, 5),
            (1019, 1018, 0),
            (2003, 5, 0),
        ] {
            let fc = FastCurve::new("tiny", p, a, b, 3);
            let mut pts = Vec::new();
            for x in 0..p {
                let rhs = (x * x % p * x + a * x + b) % p;
                for y in 0..p {
                    if y * y % p == rhs {
                        pts.push(fc.point(x, y));
                    }
                }
            }
            let order = pts.len() as u64 + 1;
            assert!(pts.iter().all(|&q| fc.is_on_curve(q)));
            assert!(
                pts.iter().any(|q| q.y == 0) || b != 0,
                "y = 0 cases present"
            );
            for (i, &q) in pts.iter().enumerate().step_by(7) {
                for k in (0..2 * order + 3).chain([u64::MAX >> 2, (1 << 40) + 17]) {
                    assert_eq!(
                        fc.mul(Some(q), k),
                        mul_affine_reference(&fc, Some(q), k),
                        "p={p} point={i} k={k}"
                    );
                }
                for &r in pts.iter().step_by(37).chain([q, fc.neg(q)].iter()) {
                    for (u, v) in [
                        (0u64, 0u64),
                        (1, 0),
                        (0, 1),
                        (1, 1),
                        (5, 9),
                        (order, 3),
                        (123_457, 98_765),
                    ] {
                        let want = fc.add(
                            mul_affine_reference(&fc, Some(q), u),
                            mul_affine_reference(&fc, Some(r), v),
                        );
                        assert_eq!(fc.combine(q, r, u, v), want, "p={p} u={u} v={v}");
                    }
                }
            }
            assert_eq!(fc.mul(None, 5), None);
        }
        for bits in [24u32, 40, 56] {
            let (fc, g) = if bits <= 28 {
                ladder(bits)
            } else {
                find_a3_curve(bits, 3).expect("ladder curve")
            };
            let mut rng = StdRng::seed_from_u64(0x5eed + bits as u64);
            let q = fc.mul(Some(g), rng.gen_range(1..fc.n)).unwrap();
            for _ in 0..50 {
                let (u, v) = (rng.gen_range(0..fc.n), rng.gen_range(0..fc.n));
                assert_eq!(fc.mul(Some(g), u), mul_affine_reference(&fc, Some(g), u));
                let want = fc.add(
                    mul_affine_reference(&fc, Some(g), u),
                    mul_affine_reference(&fc, Some(q), v),
                );
                assert_eq!(fc.combine(g, q, u, v), want, "bits={bits}");
            }
            assert_eq!(fc.mul(Some(g), fc.n), None);
        }
    }

    #[test]
    fn rho_recovers_known_logs() {
        for bits in [16u32, 20, 24] {
            let (fc, g) = ladder(bits);
            let mut rng = StdRng::seed_from_u64(99 + bits as u64);
            for negation in [true, false] {
                let k = rng.gen_range(1..fc.n);
                let q = fc.mul(Some(g), k).unwrap();
                let mut cfg = RhoConfig::for_bits(bits, negation);
                cfg.threads = 2;
                cfg.seed = 5;
                let res = rho_parallel(&fc, g, q, &cfg).expect("rho solves");
                assert_eq!(res.log, k, "bits={bits} negation={negation}");
            }
        }
    }

    #[test]
    fn batched_additions_preserve_independent_lane_streams() {
        let (curve, g) = ladder(20);
        let mut rng = StdRng::seed_from_u64(20261018);
        for width in [1usize, 2, 4, 8, 16] {
            let starts: Vec<Option<Pt>> = (0..width)
                .map(|_| curve.mul(Some(g), rng.gen_range(1..curve.n)))
                .collect();
            let jumps: Vec<Option<Pt>> = (0..width)
                .map(|_| curve.mul(Some(g), rng.gen_range(1..curve.n)))
                .collect();
            let mut scalar = starts.clone();
            let mut batch = starts;
            let mut scratch = BatchAddScratch::default();
            let mut inversions = 0usize;
            for step in 0..1024 {
                for (left, right) in scalar.iter_mut().zip(&jumps) {
                    *left = curve.add(*left, *right);
                }
                let stats = curve.add_many_batched(&mut batch, &jumps, &mut scratch);
                inversions += stats.inversions;
                assert_eq!(batch, scalar, "width {width}, step {step}");
            }
            assert!(inversions <= 1024);
        }
    }

    /// Kernel throughput diagnostic only: this excludes factor-base setup,
    /// target probing, lane fill/drain and the matrix.
    #[test]
    #[ignore]
    fn profile_native_walk_lane_batch() {
        let (curve, g) = ladder(20);
        let mut rng = StdRng::seed_from_u64(20261018);
        for width in [1usize, 4, 8, 16] {
            let starts: Vec<Option<Pt>> = (0..width)
                .map(|_| curve.mul(Some(g), rng.gen_range(1..curve.n)))
                .collect();
            let jumps: Vec<Option<Pt>> = (0..width)
                .map(|_| curve.mul(Some(g), rng.gen_range(1..curve.n)))
                .collect();
            let mut scalar = starts.clone();
            let mut batch = starts;
            let steps = 4096usize;
            let started = Instant::now();
            for _ in 0..steps {
                for (left, right) in scalar.iter_mut().zip(&jumps) {
                    *left = curve.add(*left, *right);
                }
            }
            let scalar_ns = started.elapsed().as_nanos();
            let started = Instant::now();
            let mut scratch = BatchAddScratch::default();
            let mut inversions = 0usize;
            for _ in 0..steps {
                inversions += curve
                    .add_many_batched(&mut batch, &jumps, &mut scratch)
                    .inversions;
            }
            let batch_ns = started.elapsed().as_nanos();
            assert_eq!(scalar, batch);
            println!(
                "lanes={width} steps={steps} scalar_ns={scalar_ns} batch_ns={batch_ns} \
                 scalar_inversions_at_most={} batch_inversions={inversions}",
                steps * width
            );
        }
    }

    #[test]
    fn ic_two_and_three_decomposition_recover_known_logs() {
        for bits in [16u32, 20] {
            let (fc, g) = ladder(bits);
            let mut rng = StdRng::seed_from_u64(3 + bits as u64);
            let k = rng.gen_range(1..fc.n);
            let q = fc.mul(Some(g), k).unwrap();
            for summands in [2u32, 3] {
                let (x, rep) = ic_solve(&fc, g, q, 60, 6, summands, 50_000_000, 11)
                    .unwrap_or_else(|| panic!("ic bits={bits} summands={summands}"));
                assert_eq!(x, k);
                assert_eq!(rep.relations, rep.factor_base_size + 6);
                if summands == 3 {
                    assert!(rep.rows_three > 0);
                }
            }
        }
    }

    #[test]
    fn ic_solve_fast_recovers_same_log_as_ic_solve() {
        for bits in [16u32, 20, 24] {
            let (fc, g) = ladder(bits);
            let mut rng = StdRng::seed_from_u64(555 + bits as u64);
            let k = rng.gen_range(1..fc.n);
            let q = fc.mul(Some(g), k).unwrap();
            let (x_ref, _) = ic_solve(&fc, g, q, 120, 16, 2, 50_000_000, 7)
                .unwrap_or_else(|| panic!("ic_solve bits={bits}"));
            assert_eq!(x_ref, k);
            let (x_auto, rep) = ic_solve_fast(&fc, g, q, None, 16, 50_000_000, 7)
                .unwrap_or_else(|| panic!("ic_solve_fast auto bits={bits}"));
            assert_eq!(x_auto, k, "fast(auto) wrong log bits={bits}");
            assert!(rep.factor_base_size <= ic_fast_auto_fb(fc.f.p));
            let (x_b, _) = ic_solve_fast(&fc, g, q, Some(120), 16, 50_000_000, 9)
                .unwrap_or_else(|| panic!("ic_solve_fast B=120 bits={bits}"));
            assert_eq!(x_b, x_ref, "fast/sweep logs differ bits={bits}");
        }
    }

    #[test]
    fn ic_solve_table_agrees_with_sweep() {
        // The pair-table 2-decomposition solver recovers the same log as the
        // sweep `ic_solve(summands = 2)` on the same instance (both decide the
        // identical decomposable set R = ±F_i ± F_j), and collects the same
        // number of relations.  Pins the correctness measured in
        // research/prime_ic_collection_frontier_20261117/.
        for bits in [16u32, 20, 24] {
            let (fc, g) = ladder(bits);
            let mut rng = StdRng::seed_from_u64(101 + bits as u64);
            let k = rng.gen_range(1..fc.n);
            let q = fc.mul(Some(g), k).unwrap();
            let (x_sweep, rep_sweep) = ic_solve(&fc, g, q, 120, 16, 2, 50_000_000, 7)
                .unwrap_or_else(|| panic!("sweep ic bits={bits}"));
            let (x_table, rep_table) = ic_solve_table(&fc, g, q, 120, 16, 50_000_000, 7)
                .unwrap_or_else(|| panic!("table ic bits={bits}"));
            assert_eq!(x_sweep, k, "sweep wrong log bits={bits}");
            assert_eq!(x_table, k, "table wrong log bits={bits}");
            assert_eq!(x_sweep, x_table, "sweep/table logs differ bits={bits}");
            assert_eq!(rep_sweep.relations, rep_table.relations);
            // Same seed, same walk rule, same decomposable set: identical target stream.
            assert_eq!(
                rep_sweep.targets, rep_table.targets,
                "sweep/table walked different target counts bits={bits}"
            );
        }
    }

    #[test]
    fn fast_relations_match_the_s3_sweep() {
        // Every 2-decomposition the direct subtraction finds satisfies
        // S₃(x_R, x_i, x_j) = 0, and the staged BigUint driver's relation
        // for the same target is among them.
        use crate::cryptanalysis::ec_index_calculus::{build_factor_base, semaev_s3};
        let (fc, g) = ladder(16);
        let (_, c) = bench_curves_a_minus_3()
            .into_iter()
            .find(|(b, c)| *b == 16 && c.name.contains("p256class"))
            .unwrap();
        let fb = FastFactorBase::build(&fc, 40);
        let fb_text = build_factor_base(&c, 40);
        assert_eq!(fb.len(), fb_text.len());
        let q = fc.mul(Some(g), 12345).unwrap();
        let (rels, _) = collect_relations(&fc, g, q, &fb, None, 5, 1_000_000, 1);
        assert_eq!(rels.len(), 5);
        let a_fe = c.a_fe();
        let b_fe = c.fe(c.b.clone());
        for rel in &rels {
            if rel.entries.len() != 2 {
                continue;
            }
            let r = fc.combine(g, q, rel.a, rel.b).unwrap();
            let (xr, _) = fc.canonical(r);
            let xi = fb.xs[rel.entries[0].0 as usize];
            let xj = fb.xs[rel.entries[1].0 as usize];
            let s3 = semaev_s3(
                &c.fe(BigUint::from(xr)),
                &c.fe(BigUint::from(xi)),
                &c.fe(BigUint::from(xj)),
                &a_fe,
                &b_fe,
            );
            assert!(s3.is_zero(), "S3 must vanish on a found relation");
        }
    }

    #[test]
    fn hasse_bsgs_reproduces_the_ladder_models_and_orders() {
        // The committed 16/20/24-bit a=−3 rungs, and the 28-bit P-256-class
        // rung found independently by `examples/find_a3_bench_curves`
        // (p = 268435399, b = 3, G = (1, 1), n = 268407199).
        for bits in [16u32, 20, 24] {
            let (fc, g) = ladder(bits);
            assert_eq!(point_order_hasse(&fc, g), Some(fc.n), "bits={bits}");
            let (gen, gg) = find_a3_curve(bits, 3).unwrap();
            // Preserve the frozen generator's identity. Square-root implementations
            // can select its negative, a different representation of the same model.
            assert_eq!(
                (
                    gen.f.p,
                    gen.n,
                    gen.f.from_mont(gen.a),
                    gen.f.from_mont(gen.b)
                ),
                (fc.f.p, fc.n, fc.f.from_mont(fc.a), fc.f.from_mont(fc.b))
            );
            let (x, y) = gen.canonical(gg);
            let (expected_x, expected_y) = fc.canonical(g);
            assert_eq!(x, expected_x);
            assert_eq!(
                y.min(gen.f.p - y),
                expected_y.min(fc.f.p - expected_y),
                "bits={bits}"
            );
            assert_eq!(gen.mul(Some(gg), gen.n), None);
        }
        let (gen, gg) = find_a3_curve(28, 3).unwrap();
        assert_eq!(gen.f.p, 268_435_399);
        assert_eq!(gen.f.from_mont(gen.b), 3);
        let found = gen.canonical(gg);
        assert_eq!(found.0, 1);
        assert!(found.1 == 1 || found.1 == gen.f.p - 1);
        assert_eq!(gen.n, 268_407_199);
    }

    #[test]
    fn generated_curves_have_prime_order_and_solve() {
        let (fc, g) = find_a3_curve(36, 3).unwrap();
        assert!(is_prime_u64(fc.n));
        assert_eq!(fc.mul(Some(g), fc.n), None);
        let k = 0xDEAD_BEEFu64 % fc.n;
        let q = fc.mul(Some(g), k).unwrap();
        let mut cfg = RhoConfig::for_bits(36, true);
        cfg.threads = cfg.threads.min(4);
        let res = rho_parallel(&fc, g, q, &cfg).unwrap();
        assert_eq!(res.log, k);
    }
    /// The tuned walk (look-ahead, windowed cycle escape, stride starts,
    /// short table) recovers the log, exercises its look-ahead and cycle
    /// paths, and leaves the legacy configuration's output untouched.
    #[test]
    fn tuned_walk_exercises_look_ahead_and_cycles_and_solves() {
        let fc = FastCurve::new("t", 53411, 4589, 186, 53117);
        let g = fc.point(43982, 22687);
        let (mut la, mut cyc, mut esc) = (0u64, 0u64, 0u64);
        let mut steps = 0u64;
        for k in (1u64..53117).step_by(997) {
            let q = fc.mul(Some(g), k).unwrap();
            for seed in 1..4u64 {
                let mut cfg = RhoConfig::tuned(53117);
                cfg.seed = seed;
                let res = rho_parallel(&fc, g, q, &cfg).unwrap();
                assert_eq!(res.log, k, "k={k} seed={seed}");
                la += res.ops.look_ahead_adds;
                cyc += res.ops.cycles_2 + res.ops.cycles_4 + res.ops.cycles_other;
                esc += res.ops.escape_doubles;
                steps += res.total_steps;
                assert!(res.ops.look_ahead_adds <= res.total_steps);
            }
        }
        assert!(la > 0, "look-ahead never fired");
        assert!(cyc > 0 && esc > 0, "cycle escape never fired");
        assert!(steps > 0);
        // Legacy defaults: no tuned counters move.
        let q = fc.mul(Some(g), 12345).unwrap();
        let mut cfg = RhoConfig::for_bits(16, true);
        cfg.threads = 1;
        let res = rho_parallel(&fc, g, q, &cfg).unwrap();
        assert_eq!(res.log, 12345);
        assert_eq!(res.ops.look_ahead_adds, 0);
        assert_eq!(
            res.ops.cycles_2 + res.ops.cycles_4 + res.ops.cycles_other,
            0
        );
    }

    #[test]
    fn subproduct_tree_matches_find_roots_fp_fast_zero_mismatches() {
        use crate::cryptanalysis::ec_index_calculus::find_roots_fp_fast;
        for &p in &[65_537u64, 1_048_573, 2_147_483_647] {
            let f = Fp64::new(p);
            let p_big = BigUint::from(p);
            let mut rng = StdRng::seed_from_u64(p ^ 0xCAFE_BABE);
            for &b_size in &[16usize, 64, 160] {
                let mut base_xs = Vec::with_capacity(b_size);
                let mut x = 1u64;
                while base_xs.len() < b_size && x < p {
                    base_xs.push(x);
                    x += 1 + (rng.gen_range(0..3));
                }
                let tree = FactorBaseSubproductTree::prepare(f, &base_xs);

                // Mix of random non-split, partially-in-V, fully-in-V, repeated-root,
                // quadratic, quartic, and octic polynomials.
                let mut batch: Vec<Vec<u64>> = Vec::new();
                batch.push(vec![]);
                batch.push(vec![7]);
                for case in 0..80usize {
                    let deg = match case % 5 {
                        0 => 1,
                        1 => 2,
                        2 => 4,
                        3 => 4,
                        _ => 6,
                    };
                    let poly = if case % 3 == 0 {
                        // Construct from linear factors (some inside V, some outside V, plus repeated).
                        let mut cur = vec![f.one];
                        for t in 0..deg {
                            let r = if t % 2 == 0 {
                                tree.xs[rng.gen_range(0..tree.len())]
                            } else {
                                rng.gen_range(0..p)
                            };
                            let lin = [f.neg(f.to_mont(r)), f.one];
                            let mut next = Vec::new();
                            poly_mul_schoolbook_mont(&f, &cur, &lin, &mut next);
                            cur = next;
                        }
                        let scale = f.to_mont(rng.gen_range(1..p));
                        cur.into_iter()
                            .map(|c| f.from_mont(f.mul(c, scale)))
                            .collect()
                    } else {
                        let mut coeffs: Vec<u64> = (0..deg).map(|_| rng.gen_range(0..p)).collect();
                        coeffs.push(rng.gen_range(1..p));
                        coeffs
                    };
                    batch.push(poly);
                }

                let mut scratch = SubproductBatchScratch::default();
                let batch_roots = tree.roots_in_base_batch(&batch, &mut scratch);

                for (idx, poly) in batch.iter().enumerate() {
                    let fe_coeffs: Vec<FieldElement> = poly
                        .iter()
                        .map(|&c| FieldElement::new(BigUint::from(c), p_big.clone()))
                        .collect();
                    let mut want: Vec<u64> = find_roots_fp_fast(&fe_coeffs, &p_big)
                        .into_iter()
                        .map(|fe| biguint_to_u64(&fe.value).unwrap_or(0))
                        .filter(|&r| tree.contains_x(r))
                        .collect();
                    want.sort_unstable();
                    want.dedup();

                    let single = tree.roots_in_base(poly);
                    assert_eq!(single, want, "single mismatch p={p} B={b_size} idx={idx}");
                    assert_eq!(
                        batch_roots[idx], want,
                        "batch mismatch p={p} B={b_size} idx={idx}"
                    );
                }
            }
        }
    }

    #[test]
    fn lockstep_collectors_and_solver_recover_exact_dlog_against_rho() {
        for bits in [16u32, 20, 24] {
            let (fc, g) = ladder(bits);
            let mut rng = StdRng::seed_from_u64(0x10C4 + bits as u64);
            let k = rng.gen_range(1..fc.n);
            let q = fc.mul(Some(g), k).expect("target");
            let rho_cfg = RhoConfig::tuned(fc.n);
            let rho_res = rho_parallel(&fc, g, q, &rho_cfg).expect("rho solves");
            assert_eq!(rho_res.log, k);

            for walkers in [64usize, 128, 256] {
                let (d_tbl, _) = ic_solve_lockstep(
                    &fc,
                    g,
                    q,
                    96,
                    16,
                    walkers,
                    LockStepMode::PairTable2,
                    50_000_000,
                    77 + walkers as u64,
                )
                .unwrap_or_else(|| panic!("lockstep table failed bits={bits} W={walkers}"));
                assert_eq!(d_tbl, rho_res.log, "bits={bits} W={walkers}");
                assert_eq!(fc.mul(Some(g), d_tbl), Some(q));
            }
            let (d_swp, _) = ic_solve_lockstep(
                &fc,
                g,
                q,
                64,
                12,
                64,
                LockStepMode::DirectSweep,
                50_000_000,
                99,
            )
            .unwrap_or_else(|| panic!("lockstep sweep failed bits={bits}"));
            assert_eq!(d_swp, k);
        }
    }
}

#[cfg(test)]
mod mulmod_n_tests {
    use super::MulModN;

    /// `MulModN::mul` against the 128-bit remainder it replaces, on edge
    /// and pseudo-random inputs for odd, even, small and large moduli.
    #[test]
    fn mulmod_n_matches_the_128_bit_remainder() {
        let mut s = 0x9e37_79b9_7f4a_7c15u64;
        let mut next = || {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            s
        };
        let moduli = [
            1u64,
            2,
            3,
            5,
            15,
            53_411,
            65_521,
            4_294_967_291,
            4_294_967_296,
            (1 << 61) - 1,
            (1 << 62) - 57,
            (1 << 63) - 25,
            (1 << 63) + 29,
            u64::MAX,
        ];
        for &m in &moduli {
            let f = MulModN::new(m);
            let mut cases = vec![(0, 0), (0, m - 1), (m - 1, m - 1), (1, m - 1)];
            for _ in 0..2000 {
                cases.push((next() % m, next() % m));
                cases.push((next(), next() % m)); // unreduced operand
            }
            for (a, b) in cases {
                assert_eq!(
                    f.mul(a, b),
                    ((a as u128 * b as u128) % m as u128) as u64,
                    "mul {a} {b} mod {m}"
                );
            }
        }
    }
}

#[cfg(test)]
mod batch_inv_x4_tests {
    use super::*;

    #[test]
    fn interleaved_batch_inverse_matches_scalar() {
        let f = Fp64::new(4_294_967_291); // largest prime below 2^32
        for k in [31usize, 32, 33, 34, 35, 36, 100, 257] {
            let xs: Vec<u64> = (0..k).map(|i| f.to_mont(3 + 7 * i as u64)).collect();
            let mut a = xs.clone();
            let mut sc = Vec::new();
            f.batch_inv(&mut a, &mut sc);
            for (x, y) in xs.iter().zip(&a) {
                assert_eq!(f.mul(*x, *y), f.one, "k={k}");
            }
        }
    }
}
