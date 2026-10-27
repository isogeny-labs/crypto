//! **A calibrated battery of cheap "weak class recognisable from `b`" tests.**
//!
//! Hypothesis 1 of `research/p224_hypothesis_ranking_20261008/` says a
//! designer grinding SHA-1 seeds could select `b` from a rare class that is
//! testable from `(p, a, b)` without counting points.  The only test so far
//! (`b_seed_profile`) sees classes that bias `b mod q`.  This module adds
//! seventeen further statistics, every one computable from `(p, a = −3, b)`
//! (and the X9.62 integer `c` where stated) without point counting:
//!
//! | family | statistics |
//! |:--|:--|
//! | A. Jacobi pattern over the first 32 odd primes | `jac_b`, `jac_c`, `jac_j`, `jac_disc` |
//! | B. power-residue depth in `F_p^*` | `depth_b`, `depth_disc` |
//! | C. bit structure | `hw_b`, `run_b`, `naf_b`, `hw_c`, `run_c` |
//! | D. smoothness (primes below 2^16) | `smooth_b`, `smooth_c`, `smooth_j` |
//! | E. bit structure of `j` | `hw_j`, `run_j` |
//! | F. 3-isogeny pattern | `d3` |
//!
//! Every statistic is oriented so that **large is suspicious**; a curve's
//! empirical p-value is `(1 + #{null ≥ x}) / (N + 1)`.
//!
//! # Null and conditioning
//!
//! The null population is generated the way the NIST curves were: a random
//! 160-bit seed gives `c` ([`x962_c_from_seed`]), and `b` is a square root of
//! `−27/c` when one exists (sign drawn from the seed hash).  The NIST curves
//! also have prime order, so none has a rational 2-torsion or 3-torsion
//! point.  Those facts need no point count to test, so the null is
//! **conditioned** on them: the cubic `x³ − 3x + b` has no root in `F_p`,
//! and no root of the 3-division polynomial has a square `y²`.  The torsion
//! statistics are then not tested (they are constant on the real curves);
//! `d3`, the number of roots of `ψ₃` in `F_p`, counts the rational 3-isogeny
//! kernels whose points are defined only over the quadratic extension.
//!
//! # Positive controls
//!
//! [`plant`] builds instances with the property each statistic targets, by
//! construction (Chinese remaindering for the Jacobi patterns, products of
//! small primes for smoothness, `r^g` for power-residue depth, and so on).
//! They go through the same conditioning and the same statistic code.
//!
//! Protocol and report: `research/b_weak_class_battery_20261022/`.

use crate::cryptanalysis::p224_hypotheses::x962_c_from_seed;
use crate::hash::sha1::sha1;
use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Number of statistics in the battery.
pub const NSTAT: usize = 17;

/// Statistic names, in column order.
pub const STAT_NAMES: [&str; NSTAT] = [
    "jac_b",
    "jac_c",
    "jac_j",
    "jac_disc",
    "depth_b",
    "depth_disc",
    "hw_b",
    "run_b",
    "naf_b",
    "hw_c",
    "run_c",
    "smooth_b",
    "smooth_c",
    "smooth_j",
    "hw_j",
    "run_j",
    "d3",
];

/// Family letter of each statistic (see the table above).
pub const STAT_FAMILY: [&str; NSTAT] = [
    "A", "A", "A", "A", "B", "B", "C", "C", "C", "C", "C", "D", "D", "D", "E", "E", "F",
];

/// Number of odd primes in the Jacobi pattern.
pub const JACOBI_PRIMES: usize = 32;
/// Smoothness bound: primes below `2^16`.
pub const SMOOTH_BOUND: u64 = 1 << 16;

// ── small number theory ─────────────────────────────────────────────────────

fn low_u64(n: &BigUint) -> u64 {
    n.iter_u64_digits().next().unwrap_or(0)
}

/// Jacobi symbol `(a | n)` for odd `n > 0`.
pub fn jacobi(a: &BigUint, n: &BigUint) -> i32 {
    let mut a = a % n;
    let mut n = n.clone();
    let mut t = 1i32;
    while !a.is_zero() {
        while !a.bit(0) {
            a >>= 1;
            let r = low_u64(&n) & 7;
            if r == 3 || r == 5 {
                t = -t;
            }
        }
        std::mem::swap(&mut a, &mut n);
        if low_u64(&a) & 3 == 3 && low_u64(&n) & 3 == 3 {
            t = -t;
        }
        a %= &n;
    }
    if n.is_one() {
        t
    } else {
        0
    }
}

/// The primes below `bound`.
pub fn sieve(bound: u64) -> Vec<u64> {
    let n = bound as usize;
    let mut s = vec![true; n.max(2)];
    s[0] = false;
    s[1] = false;
    let mut i = 2;
    while i * i < n {
        if s[i] {
            let mut j = i * i;
            while j < n {
                s[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    (0..n).filter(|&i| s[i]).map(|i| i as u64).collect()
}

/// The field `F_p` for an odd prime `p` and the curve `y² = x³ − 3x + b`.
pub struct Ctx {
    pub p: BigUint,
    /// Bit length of `p`.
    pub t: u64,
    small_primes: Vec<u64>,
    jac_primes: Vec<u64>,
    /// `qr[i][r] = (r | jac_primes[i])` for `r` below the prime.
    qr: Vec<Vec<i8>>,
    /// Divisor `G` of `p − 1` used by the depth statistics, with its
    /// prime-power structure `(ℓ, exponent)`.
    depth_g: Vec<(u64, u32)>,
}

/// Caps on the exponent of each small prime in `G`.
const DEPTH_PRIMES: [(u64, u32); 7] = [(2, 24), (3, 8), (5, 4), (7, 3), (11, 2), (13, 2), (17, 2)];

impl Ctx {
    pub fn new(p: &BigUint) -> Ctx {
        let small_primes = sieve(SMOOTH_BOUND);
        let jac_primes: Vec<u64> = small_primes
            .iter()
            .copied()
            .filter(|&q| q > 2)
            .take(JACOBI_PRIMES)
            .collect();
        let qr = jac_primes
            .iter()
            .map(|&q| {
                (0..q)
                    .map(|r| {
                        let mut e = 1u64;
                        let mut base = r % q;
                        let mut k = (q - 1) / 2;
                        while k > 0 {
                            if k & 1 == 1 {
                                e = e * base % q;
                            }
                            base = base * base % q;
                            k >>= 1;
                        }
                        if r == 0 {
                            0
                        } else if e == 1 {
                            1
                        } else {
                            -1
                        }
                    })
                    .collect()
            })
            .collect();
        let pm1 = p - 1u8;
        let depth_g = DEPTH_PRIMES
            .iter()
            .filter_map(|&(l, cap)| {
                let mut e = 0u32;
                let mut m = pm1.clone();
                let lb = BigUint::from(l);
                while e < cap && (&m % &lb).is_zero() {
                    m /= &lb;
                    e += 1;
                }
                (e > 0).then_some((l, e))
            })
            .collect();
        Ctx {
            p: p.clone(),
            t: p.bits(),
            small_primes,
            jac_primes,
            qr,
            depth_g,
        }
    }

    pub fn mul(&self, a: &BigUint, b: &BigUint) -> BigUint {
        a * b % &self.p
    }

    pub fn neg(&self, a: &BigUint) -> BigUint {
        if a.is_zero() {
            BigUint::zero()
        } else {
            &self.p - a
        }
    }

    pub fn inv(&self, a: &BigUint) -> BigUint {
        a.modpow(&(&self.p - 2u8), &self.p)
    }

    pub fn jac(&self, a: &BigUint) -> i32 {
        jacobi(a, &self.p)
    }

    /// A square root of `a` (Tonelli–Shanks when `p ≡ 1 mod 4`).
    pub fn sqrt(&self, a: &BigUint) -> Option<BigUint> {
        let p = &self.p;
        let a = a % p;
        if a.is_zero() {
            return Some(a);
        }
        if self.jac(&a) != 1 {
            return None;
        }
        if low_u64(p) & 3 == 3 {
            let r = a.modpow(&((p + 1u8) >> 2), p);
            return (self.mul(&r, &r) == a).then_some(r);
        }
        let mut q = p - 1u8;
        let mut s = 0u32;
        while !q.bit(0) {
            q >>= 1;
            s += 1;
        }
        let mut z = BigUint::from(2u8);
        while self.jac(&z) != -1 {
            z += 1u8;
        }
        let mut m = s;
        let mut c = z.modpow(&q, p);
        let mut t = a.modpow(&q, p);
        let mut r = a.modpow(&((&q + 1u8) >> 1), p);
        while !t.is_one() {
            let mut i = 0u32;
            let mut tt = t.clone();
            while !tt.is_one() {
                tt = self.mul(&tt, &tt);
                i += 1;
                if i >= m {
                    return None;
                }
            }
            let mut b = c.clone();
            for _ in 0..(m - i - 1) {
                b = self.mul(&b, &b);
            }
            r = self.mul(&r, &b);
            c = self.mul(&b, &b);
            t = self.mul(&t, &c);
            m = i;
        }
        (self.mul(&r, &r) == a).then_some(r)
    }

    /// `u = −27/c`, the value `b²` must take.
    pub fn b_squared_from_c(&self, c: &BigUint) -> BigUint {
        self.mul(&(&self.p - 27u8), &self.inv(c))
    }

    /// `c = −27/b² mod p`.
    pub fn c_from_b(&self, b: &BigUint) -> BigUint {
        self.mul(&(&self.p - 27u8), &self.inv(&self.mul(b, b)))
    }

    /// `b² − 4 mod p` (zero iff the curve is singular).
    pub fn b2_minus_4(&self, b: &BigUint) -> BigUint {
        (self.mul(b, b) + &self.p - 4u8) % &self.p
    }

    /// `j = −6912 / (b² − 4)`.
    pub fn j_invariant(&self, b: &BigUint) -> Option<BigUint> {
        let d = self.b2_minus_4(b);
        if d.is_zero() {
            return None;
        }
        Some(self.mul(&(&self.p - 6912u32), &self.inv(&d)))
    }

    /// `4a³ + 27b² = 27 (b² − 4)` for `a = −3`.
    pub fn disc_value(&self, b: &BigUint) -> BigUint {
        self.mul(&BigUint::from(27u8), &self.b2_minus_4(b))
    }
}

// ── polynomials over F_p ────────────────────────────────────────────────────

type Poly = Vec<BigUint>;

fn ptrim(a: &mut Poly) {
    while a.last().is_some_and(|x| x.is_zero()) {
        a.pop();
    }
}

fn pmul(a: &Poly, b: &Poly, p: &BigUint) -> Poly {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut out = Vec::with_capacity(a.len() + b.len() - 1);
    for k in 0..a.len() + b.len() - 1 {
        let lo = k.saturating_sub(b.len() - 1);
        let hi = k.min(a.len() - 1);
        let mut acc = BigUint::zero();
        for i in lo..=hi {
            acc += &a[i] * &b[k - i];
        }
        out.push(acc % p);
    }
    ptrim(&mut out);
    out
}

fn psub(a: &Poly, b: &Poly, p: &BigUint) -> Poly {
    let n = a.len().max(b.len());
    let zero = BigUint::zero();
    let mut out: Poly = (0..n)
        .map(|i| {
            let x = a.get(i).unwrap_or(&zero);
            let y = b.get(i).unwrap_or(&zero);
            (x + p - y) % p
        })
        .collect();
    ptrim(&mut out);
    out
}

fn pscale(a: &Poly, k: &BigUint, p: &BigUint) -> Poly {
    let mut out: Poly = a.iter().map(|x| x * k % p).collect();
    ptrim(&mut out);
    out
}

fn pmonic(a: &Poly, ctx: &Ctx) -> Poly {
    match a.last() {
        None => vec![],
        Some(l) if l.is_one() => a.clone(),
        Some(l) => pscale(a, &ctx.inv(l), &ctx.p),
    }
}

/// Quotient and remainder of `a` by a monic `m` of degree at least 1.
fn pdivrem(a: &Poly, m: &Poly, p: &BigUint) -> (Poly, Poly) {
    let d = m.len() - 1;
    let mut r = a.clone();
    let mut q: Poly = if a.len() > d {
        vec![BigUint::zero(); a.len() - d]
    } else {
        vec![]
    };
    while r.len() > d {
        let lead = r.pop().expect("nonempty");
        if lead.is_zero() {
            continue;
        }
        let shift = r.len() - d;
        let neg = p - &lead;
        for j in 0..d {
            r[shift + j] = (&r[shift + j] + &neg * &m[j]) % p;
        }
        q[shift] = lead;
    }
    ptrim(&mut r);
    ptrim(&mut q);
    (q, r)
}

fn pgcd(a: &Poly, b: &Poly, ctx: &Ctx) -> Poly {
    let mut a = a.clone();
    let mut b = b.clone();
    ptrim(&mut a);
    ptrim(&mut b);
    while !b.is_empty() {
        let bm = pmonic(&b, ctx);
        let r = if bm.len() == 1 {
            vec![]
        } else {
            pdivrem(&a, &bm, &ctx.p).1
        };
        a = bm;
        b = r;
    }
    pmonic(&a, ctx)
}

/// `base^e mod m` for a monic `m`.
fn ppow(base: &Poly, e: &BigUint, m: &Poly, p: &BigUint) -> Poly {
    let base = pdivrem(base, m, p).1;
    let mut result = base.clone();
    for i in (0..e.bits().saturating_sub(1)).rev() {
        result = pdivrem(&pmul(&result, &result, p), m, p).1;
        if e.bit(i) {
            result = pdivrem(&pmul(&result, &base, p), m, p).1;
        }
    }
    result
}

/// The distinct roots in `F_p` of a monic product of distinct linear factors.
fn split_roots(g: &Poly, ctx: &Ctx, out: &mut Vec<BigUint>) {
    let deg = g.len().saturating_sub(1);
    if deg == 0 {
        return;
    }
    if deg == 1 {
        out.push(ctx.neg(&g[0]));
        return;
    }
    let half = (&ctx.p - 1u8) >> 1;
    let one: Poly = vec![BigUint::one()];
    for r in 1u32.. {
        let h = ppow(&vec![BigUint::from(r), BigUint::one()], &half, g, &ctx.p);
        let d = pgcd(g, &psub(&h, &one, &ctx.p), ctx);
        let dd = d.len() - 1;
        if dd > 0 && dd < deg {
            split_roots(&d, ctx, out);
            let (q, _) = pdivrem(g, &d, &ctx.p);
            split_roots(&pmonic(&q, ctx), ctx, out);
            return;
        }
    }
}

/// The roots in `F_p` of a squarefree polynomial.
pub fn poly_roots(f: &Poly, ctx: &Ctx) -> Vec<BigUint> {
    let f = pmonic(f, ctx);
    let x: Poly = vec![BigUint::zero(), BigUint::one()];
    let xp = ppow(&x, &ctx.p, &f, &ctx.p);
    let g = pgcd(&f, &psub(&xp, &x, &ctx.p), ctx);
    let mut roots = Vec::new();
    split_roots(&g, ctx, &mut roots);
    roots
}

/// `x³ − 3x + b`.
fn cubic(ctx: &Ctx, b: &BigUint) -> Poly {
    vec![b.clone(), &ctx.p - 3u8, BigUint::zero(), BigUint::one()]
}

/// The 3-division polynomial `3x⁴ − 18x² + 12bx − 9` of `y² = x³ − 3x + b`.
fn psi3(ctx: &Ctx, b: &BigUint) -> Poly {
    let p = &ctx.p;
    vec![
        p - 9u8,
        b * 12u8 % p,
        p - 18u8,
        BigUint::zero(),
        BigUint::from(3u8),
    ]
}

/// What the torsion checks found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Torsion {
    /// Passes the conditioning; `d3` roots of `ψ₃`, all with non-square `y²`.
    Accepted { d3: u32 },
    /// `b² = 4`: the curve is singular.
    Singular,
    /// `x³ − 3x + b` has a root: even group order.
    RationalTwoTorsion,
    /// A root of `ψ₃` has a square `y²`: group order divisible by 3.
    RationalThreeTorsion,
}

/// Number of roots of `x³ − 3x + b` in `F_p`.
pub fn cubic_root_count(ctx: &Ctx, b: &BigUint) -> usize {
    poly_roots(&cubic(ctx, b), ctx).len()
}

/// Roots in `F_p` of `ψ₃` (x-coordinates of the points of order 3).
pub fn psi3_roots(ctx: &Ctx, b: &BigUint) -> Vec<BigUint> {
    poly_roots(&psi3(ctx, b), ctx)
}

/// The conditioning of the null population, from `b` alone.
pub fn torsion_profile(ctx: &Ctx, b: &BigUint) -> Torsion {
    if ctx.b2_minus_4(b).is_zero() {
        return Torsion::Singular;
    }
    // The cubic's discriminant is −27 (b² − 4).  An irreducible cubic has a
    // square discriminant, so a non-square one means a rational root.
    let d = ctx.mul(&(&ctx.p - 3u8), &ctx.b2_minus_4(b));
    if ctx.jac(&d) != 1 {
        return Torsion::RationalTwoTorsion;
    }
    if cubic_root_count(ctx, b) != 0 {
        return Torsion::RationalTwoTorsion;
    }
    let roots = psi3_roots(ctx, b);
    for x0 in &roots {
        let rhs = (ctx.mul(&ctx.mul(x0, x0), x0) + &ctx.p * 3u8 - ctx.mul(&BigUint::from(3u8), x0)
            + b)
            % &ctx.p;
        if ctx.jac(&rhs) != -1 {
            return Torsion::RationalThreeTorsion;
        }
    }
    Torsion::Accepted {
        d3: roots.len() as u32,
    }
}

// ── statistics ──────────────────────────────────────────────────────────────

fn hamming(x: &BigUint) -> u64 {
    x.iter_u64_digits().map(|d| d.count_ones() as u64).sum()
}

/// Weight of the non-adjacent form: `popcount((3x ⊕ x) >> 1)`.
pub fn naf_weight(x: &BigUint) -> u64 {
    let x3 = x * 3u8;
    hamming(&((&x3 ^ x) >> 1))
}

/// Longest run of equal bits in the `nbits`-bit big-endian representation.
pub fn longest_run(x: &BigUint, nbits: u64) -> u64 {
    let mut best = 0u64;
    let mut cur = 0u64;
    let mut prev = None;
    for i in (0..nbits).rev() {
        let bit = x.bit(i);
        if prev == Some(bit) {
            cur += 1;
        } else {
            cur = 1;
            prev = Some(bit);
        }
        best = best.max(cur);
    }
    best
}

/// `|HW − n/2| / (√n / 2)`.
fn hw_z(x: &BigUint, nbits: u64) -> f64 {
    let n = nbits as f64;
    ((hamming(x) as f64 - n / 2.0) / (n.sqrt() / 2.0)).abs()
}

/// `|NAF − n/3| / √(2n/27)`.
fn naf_z(x: &BigUint, nbits: u64) -> f64 {
    let n = nbits as f64;
    ((naf_weight(x) as f64 - n / 3.0) / (2.0 * n / 27.0).sqrt()).abs()
}

fn round6(v: f64) -> f64 {
    (v * 1e6).round() / 1e6
}

impl Ctx {
    /// `|Σ (x | ℓ)| / √32` over the first 32 odd primes.
    pub fn jacobi_z(&self, x: &BigUint) -> f64 {
        let s: i64 = self
            .jac_primes
            .iter()
            .zip(&self.qr)
            .map(|(&q, table)| {
                let r = x
                    .iter_u64_digits()
                    .rev()
                    .fold(0u128, |r, d| ((r << 64) | d as u128) % q as u128);
                table[r as usize] as i64
            })
            .sum();
        (s as f64).abs() / (self.jac_primes.len() as f64).sqrt()
    }

    /// `log2` of the part of `x` made of primes below `2^16`, with multiplicity.
    pub fn smooth_bits(&self, x: &BigUint) -> f64 {
        let mut digits: Vec<u64> = x.iter_u64_digits().rev().collect();
        let mut y = x.clone();
        let mut bits = 0.0f64;
        for &q in &self.small_primes {
            let r = digits
                .iter()
                .fold(0u128, |r, &d| ((r << 64) | d as u128) % q as u128);
            if r == 0 {
                let qb = BigUint::from(q);
                while (&y % &qb).is_zero() {
                    y /= &qb;
                    bits += (q as f64).log2();
                }
                digits = y.iter_u64_digits().rev().collect();
                if y.is_one() {
                    break;
                }
            }
        }
        bits
    }

    /// Power-residue depth of `x ≠ 0` in bits: `log2` of the largest divisor
    /// `g` of `G` for which `x` is a `g`-th power, `G` the `{2,3,5,7,11,13,17}`-
    /// part of `p − 1` (the exponent of 2 capped at 24).
    pub fn depth_bits(&self, x: &BigUint) -> f64 {
        if x.is_zero() || self.depth_g.is_empty() {
            return 0.0;
        }
        let g: BigUint = self
            .depth_g
            .iter()
            .map(|&(l, e)| BigUint::from(l).pow(e))
            .product();
        let y = x.modpow(&((&self.p - 1u8) / &g), &self.p);
        let mut depth = 0.0;
        for &(l, e) in &self.depth_g {
            let lb = BigUint::from(l);
            // z has order dividing l^e
            let mut z = y.modpow(&(&g / lb.pow(e)), &self.p);
            let mut k = 0u32;
            while !z.is_one() {
                z = z.modpow(&lb, &self.p);
                k += 1;
            }
            depth += (e - k) as f64 * (l as f64).log2();
        }
        depth
    }

    /// The full statistic vector of `(b, c)`; `None` for a singular curve.
    pub fn stats(&self, b: &BigUint, c: &BigUint, d3: u32) -> Option<[f64; NSTAT]> {
        let j = self.j_invariant(b)?;
        let disc = self.disc_value(b);
        let u = self.b2_minus_4(b);
        let t = self.t;
        let tc = t - 1; // the X9.62 c has t − 1 bits
        let v = [
            self.jacobi_z(b),
            self.jacobi_z(c),
            self.jacobi_z(&j),
            self.jacobi_z(&disc),
            self.depth_bits(b),
            self.depth_bits(&u),
            hw_z(b, t),
            longest_run(b, t) as f64,
            naf_z(b, t),
            hw_z(c, tc),
            longest_run(c, tc) as f64,
            self.smooth_bits(b),
            self.smooth_bits(c),
            self.smooth_bits(&j),
            hw_z(&j, t),
            longest_run(&j, t) as f64,
            d3 as f64,
        ];
        Some(v.map(round6))
    }
}

/// An accepted curve with its statistics.
#[derive(Clone, Debug)]
pub struct Instance {
    pub b: BigUint,
    pub c: BigUint,
    pub stats: [f64; NSTAT],
}

/// Statistics of a curve already known to satisfy the conditioning.
pub fn evaluate(ctx: &Ctx, b: &BigUint, c: &BigUint) -> Result<Instance, Torsion> {
    match torsion_profile(ctx, b) {
        Torsion::Accepted { d3 } => ctx
            .stats(b, c, d3)
            .map(|stats| Instance {
                b: b.clone(),
                c: c.clone(),
                stats,
            })
            .ok_or(Torsion::Singular),
        other => Err(other),
    }
}

// ── the null population ─────────────────────────────────────────────────────

/// Seed `k` of null draw `i`: `SHA-1(label ‖ curve ‖ i ‖ k)`.
pub fn null_seed(label: &str, curve: &str, i: u32, k: u32) -> [u8; 20] {
    let mut msg = label.as_bytes().to_vec();
    msg.push(b'/');
    msg.extend_from_slice(curve.as_bytes());
    msg.extend_from_slice(&i.to_be_bytes());
    msg.extend_from_slice(&k.to_be_bytes());
    sha1(&msg)
}

/// Null draw `i`: the first seed `k` whose `c` yields a `b` that passes the
/// conditioning.  Returns the instance and the number of seeds tried.
pub fn null_instance(ctx: &Ctx, label: &str, curve: &str, i: u32) -> (Instance, u32) {
    for k in 0u32.. {
        let seed = null_seed(label, curve, i, k);
        let c = x962_c_from_seed(&seed, ctx.t);
        if c.is_zero() {
            continue;
        }
        // b² = −27/c exists iff (−3c | p) = 1
        if ctx.jac(&ctx.mul(&(&ctx.p - 3u8), &c)) != 1 {
            continue;
        }
        let u = ctx.b_squared_from_c(&c);
        // cheap pre-filter of the 2-torsion condition: (−3 (u − 4) | p) = 1
        let d = ctx.mul(&(&ctx.p - 3u8), &((&u + &ctx.p - 4u8) % &ctx.p));
        if ctx.jac(&d) != 1 {
            continue;
        }
        let Some(mut b) = ctx.sqrt(&u) else { continue };
        if sha1(&[&seed[..], b"sign"].concat())[0] & 1 == 1 {
            b = ctx.neg(&b);
        }
        if let Ok(inst) = evaluate(ctx, &b, &c) {
            return (inst, k + 1);
        }
    }
    unreachable!()
}

// ── p-values ────────────────────────────────────────────────────────────────

/// A sorted null column.
pub struct NullColumn {
    sorted: Vec<f64>,
}

impl NullColumn {
    pub fn new(values: &[f64]) -> NullColumn {
        let mut sorted = values.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).expect("no NaN"));
        NullColumn { sorted }
    }

    pub fn n(&self) -> usize {
        self.sorted.len()
    }

    /// `(1 + #{null ≥ x}) / (N + 1)`.
    pub fn p_upper(&self, x: f64) -> f64 {
        let below = self.sorted.partition_point(|&v| v < x);
        (1 + self.sorted.len() - below) as f64 / (self.sorted.len() + 1) as f64
    }

    /// The empirical `q`-quantile.
    pub fn quantile(&self, q: f64) -> f64 {
        let i = ((self.sorted.len() as f64 - 1.0) * q).round() as usize;
        self.sorted[i]
    }

    pub fn max(&self) -> f64 {
        *self.sorted.last().expect("nonempty")
    }
}

// ── positive controls: planted instances ────────────────────────────────────

/// A deterministic byte stream: `SHA-1(label ‖ counter)`.
pub struct Stream {
    label: Vec<u8>,
    ctr: u64,
}

impl Stream {
    pub fn new(label: &str) -> Stream {
        Stream {
            label: label.as_bytes().to_vec(),
            ctr: 0,
        }
    }

    pub fn bytes(&mut self, n: usize) -> Vec<u8> {
        let mut out = Vec::with_capacity(n + 20);
        while out.len() < n {
            let mut msg = self.label.clone();
            msg.extend_from_slice(&self.ctr.to_be_bytes());
            self.ctr += 1;
            out.extend_from_slice(&sha1(&msg));
        }
        out.truncate(n);
        out
    }

    /// A uniform integer below `2^nbits`.
    pub fn bits(&mut self, nbits: u64) -> BigUint {
        let n = nbits.div_ceil(8) as usize;
        BigUint::from_bytes_be(&self.bytes(n)) & ((BigUint::one() << nbits) - 1u8)
    }

    /// An integer below `bound`, with a negligible bias (64 extra bits).
    pub fn below(&mut self, bound: &BigUint) -> BigUint {
        self.bits(bound.bits() + 64) % bound
    }

    pub fn u64_below(&mut self, n: u64) -> u64 {
        low_u64(&self.below(&BigUint::from(n)))
    }

    pub fn coin(&mut self) -> bool {
        self.bytes(1)[0] & 1 == 1
    }
}

/// An integer below `limit` that is a quadratic residue modulo each of the
/// first 32 odd primes, by Chinese remaindering.
fn crt_all_residues(ctx: &Ctx, limit_bits: u64, s: &mut Stream) -> Option<BigUint> {
    let mut x = BigUint::zero();
    let mut m = BigUint::one();
    for (i, &q) in ctx.jac_primes.iter().enumerate() {
        let r = loop {
            let r = s.u64_below(q - 1) + 1;
            if ctx.qr[i][r as usize] == 1 {
                break r;
            }
        };
        let xm = low_u64(&(&x % q));
        let minv = (1..q).find(|k| (low_u64(&(&m % q)) * k) % q == 1)?;
        let tq = ((r + q - xm) % q) * minv % q;
        x += &m * tq;
        m *= q;
    }
    if m.bits() >= limit_bits {
        return None;
    }
    let room = (BigUint::one() << limit_bits) / &m;
    Some(x + &m * s.below(&room))
}

/// A product of random primes below `2^16` of about `bits` bits.
fn smooth_number(ctx: &Ctx, bits: u64, s: &mut Stream) -> BigUint {
    let mut v = BigUint::one();
    while v.bits() < bits {
        let q = ctx.small_primes[s.u64_below(ctx.small_primes.len() as u64) as usize];
        v *= q;
    }
    v
}

/// `r^g mod p` for the largest `g` the depth statistics use.
fn depth_power(ctx: &Ctx, s: &mut Stream) -> BigUint {
    let g: BigUint = ctx
        .depth_g
        .iter()
        .map(|&(l, e)| BigUint::from(l).pow(e.min(if l == 2 { 12 } else { e })))
        .product();
    let r = s.below(&(&ctx.p - 1u8)) + 1u8;
    r.modpow(&g, &ctx.p)
}

fn sparse_naf(ctx: &Ctx, s: &mut Stream) -> BigUint {
    // 24 non-adjacent signed digits in positions 0..t−1, positive total
    let t = ctx.t;
    let mut v = num_bigint::BigInt::from(0);
    let mut pos = 2 * s.u64_below(4);
    let step = (t - 8) / 24;
    for _ in 0..24 {
        pos += 2 + s.u64_below(step.saturating_sub(2).max(1));
        if pos >= t - 1 {
            break;
        }
        let term = num_bigint::BigInt::from(BigUint::one() << pos);
        if s.coin() {
            v += term;
        } else {
            v -= term;
        }
    }
    v.magnitude().clone()
}

/// A random integer with each bit set with probability 3/4, of `nbits` bits.
fn heavy_bits(nbits: u64, s: &mut Stream) -> BigUint {
    s.bits(nbits) | s.bits(nbits)
}

/// A random `nbits`-bit integer with a run of 40 ones at a random position.
fn run_bits(nbits: u64, s: &mut Stream) -> BigUint {
    let pos = s.u64_below(nbits - 40);
    s.bits(nbits) | (((BigUint::one() << 40) - 1u8) << pos)
}

/// Which positive-control construction fed the statistic.
pub fn plant_description(stat: usize) -> &'static str {
    match STAT_NAMES[stat] {
        "jac_b" | "jac_c" | "jac_j" | "jac_disc" => {
            "target quantity is a quadratic residue modulo each of the first 32 odd primes (Chinese remaindering)"
        }
        "depth_b" => "b = r^g, with g the product of 2^12 and the available 3,5,7,11,13,17 parts of G",
        "depth_disc" => "b² − 4 = v^g, same g; b a square root of v^g + 4",
        "hw_b" | "hw_c" | "hw_j" => "each bit of the target set with probability 3/4",
        "run_b" | "run_c" | "run_j" => "a run of 40 one-bits at a random position",
        "naf_b" => "b with 24 non-adjacent signed digits",
        "smooth_b" | "smooth_c" | "smooth_j" => "target quantity a product of primes below 2^16",
        "d3" => "rejection sampling of null draws with the maximum d3 seen in the null panel",
        _ => "",
    }
}

/// A planted instance for statistic `stat` that passes the conditioning, or
/// `None` after `tries` attempts.  For `d3`, `d3_target` is the value to
/// reach (planted by rejection from the null stream, see [`plant_d3`]).
pub fn plant(ctx: &Ctx, stat: usize, s: &mut Stream, tries: u32) -> Option<Instance> {
    let p = &ctx.p;
    let t = ctx.t;
    let name = STAT_NAMES[stat];
    for _ in 0..tries {
        // the target quantity, then (b, c)
        let cand: Option<(BigUint, Option<BigUint>)> = match name {
            "jac_b" => crt_all_residues(ctx, t, s).map(|b| (b, None)),
            "jac_c" => crt_all_residues(ctx, t - 1, s).and_then(|c| {
                let b = ctx.sqrt(&ctx.b_squared_from_c(&c))?;
                Some((b, Some(c)))
            }),
            "jac_j" => crt_all_residues(ctx, t, s).and_then(|j| {
                let b2 =
                    (BigUint::from(4u8) + p - ctx.mul(&BigUint::from(6912u32), &ctx.inv(&j))) % p;
                Some((ctx.sqrt(&b2)?, None))
            }),
            "jac_disc" => crt_all_residues(ctx, t, s).and_then(|d| {
                let b2 = (ctx.mul(&d, &ctx.inv(&BigUint::from(27u8))) + 4u8) % p;
                Some((ctx.sqrt(&b2)?, None))
            }),
            "depth_b" => Some((depth_power(ctx, s), None)),
            "depth_disc" => {
                let u = depth_power(ctx, s);
                let b2 = (u + 4u8) % p;
                ctx.sqrt(&b2).map(|b| (b, None))
            }
            "hw_b" => Some((heavy_bits(t, s), None)),
            "run_b" => Some((run_bits(t, s), None)),
            "naf_b" => Some((sparse_naf(ctx, s), None)),
            "hw_c" | "run_c" => {
                let c = if name == "hw_c" {
                    heavy_bits(t - 1, s)
                } else {
                    run_bits(t - 1, s)
                };
                if c.bits() < t - 1 {
                    // keep the (t − 1)-bit length the real c has
                    None
                } else {
                    ctx.sqrt(&ctx.b_squared_from_c(&c)).map(|b| (b, Some(c)))
                }
            }
            "smooth_b" => Some((smooth_number(ctx, t - 2, s), None)),
            "smooth_c" => {
                let c = smooth_number(ctx, t - 2, s);
                if c.bits() != t - 1 {
                    None
                } else {
                    ctx.sqrt(&ctx.b_squared_from_c(&c)).map(|b| (b, Some(c)))
                }
            }
            "smooth_j" => {
                let j = smooth_number(ctx, t - 2, s);
                if &j >= p {
                    None
                } else {
                    let b2 = (BigUint::from(4u8) + p
                        - ctx.mul(&BigUint::from(6912u32), &ctx.inv(&j)))
                        % p;
                    ctx.sqrt(&b2).map(|b| (b, None))
                }
            }
            "hw_j" | "run_j" => {
                let j = if name == "hw_j" {
                    heavy_bits(t, s)
                } else {
                    run_bits(t, s)
                };
                if &j >= p || j.is_zero() {
                    None
                } else {
                    let b2 = (BigUint::from(4u8) + p
                        - ctx.mul(&BigUint::from(6912u32), &ctx.inv(&j)))
                        % p;
                    ctx.sqrt(&b2).map(|b| (b, None))
                }
            }
            _ => None,
        };
        let Some((mut b, c)) = cand else { continue };
        if &b >= p || b.is_zero() {
            continue;
        }
        // a planted b keeps its construction only when the sign is not
        // flipped; roots derived through a square root get a random sign
        if matches!(
            name,
            "jac_c"
                | "jac_j"
                | "jac_disc"
                | "depth_disc"
                | "hw_c"
                | "run_c"
                | "smooth_c"
                | "smooth_j"
                | "hw_j"
                | "run_j"
        ) && s.coin()
        {
            b = ctx.neg(&b);
        }
        let c = c.unwrap_or_else(|| ctx.c_from_b(&b));
        if let Ok(inst) = evaluate(ctx, &b, &c) {
            return Some(inst);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::p224_hypotheses::nist_seeded_curves;

    fn small_ctx(p: u64) -> Ctx {
        Ctx::new(&BigUint::from(p))
    }

    fn euler(a: u64, p: u64) -> i32 {
        let r = BigUint::from(a).modpow(&BigUint::from((p - 1) / 2), &BigUint::from(p));
        if r.is_zero() {
            0
        } else if r.is_one() {
            1
        } else {
            -1
        }
    }

    #[test]
    fn jacobi_matches_euler_criterion() {
        for &p in &[10007u64, 10009, 1_000_003] {
            for a in 0..300u64 {
                assert_eq!(
                    jacobi(&BigUint::from(a * 37 + 5), &BigUint::from(p)),
                    euler((a * 37 + 5) % p, p),
                    "p={p} a={a}"
                );
            }
        }
    }

    #[test]
    fn sqrt_works_for_both_residue_classes_of_p_mod_4() {
        // 10007 ≡ 3, 10009 ≡ 1 (mod 4); also P-224 (v2 = 96)
        for &p in &[10007u64, 10009] {
            let ctx = small_ctx(p);
            for a in 1..200u64 {
                let a = BigUint::from(a);
                if let Some(r) = ctx.sqrt(&a) {
                    assert_eq!(ctx.mul(&r, &r), a);
                } else {
                    assert_eq!(ctx.jac(&a), -1);
                }
            }
        }
        let curve = &nist_seeded_curves()[1];
        let ctx = Ctx::new(&curve.params.p);
        let r = ctx
            .sqrt(&ctx.mul(&curve.params.b, &curve.params.b))
            .unwrap();
        assert!(r == curve.params.b || r == ctx.neg(&curve.params.b));
    }

    /// Brute-force group order of `y² = x³ − 3x + b` over a small prime.
    fn brute_order(p: u64, b: u64) -> u64 {
        let mut n = 1u64;
        for x in 0..p {
            let rhs = (x * x % p * x % p + 3 * p - 3 * x + b) % p;
            n += match euler(rhs, p) {
                0 => 1,
                1 => 2,
                _ => 0,
            };
        }
        n
    }

    #[test]
    fn torsion_flags_agree_with_point_counts() {
        for &p in &[10007u64, 10009] {
            let ctx = small_ctx(p);
            let (mut even_seen, mut three_seen, mut acc_seen) = (0, 0, 0);
            for b in 1..120u64 {
                let bb = BigUint::from(b);
                if ctx.b2_minus_4(&bb).is_zero() {
                    continue;
                }
                let n = brute_order(p, b);
                match torsion_profile(&ctx, &bb) {
                    Torsion::RationalTwoTorsion => {
                        assert_eq!(n % 2, 0, "p={p} b={b}");
                        even_seen += 1;
                    }
                    Torsion::RationalThreeTorsion => {
                        assert_eq!(n % 2, 1, "p={p} b={b}");
                        assert_eq!(n % 3, 0, "p={p} b={b}");
                        three_seen += 1;
                    }
                    Torsion::Accepted { .. } => {
                        assert_eq!(n % 2, 1, "p={p} b={b}");
                        assert_ne!(n % 3, 0, "p={p} b={b}");
                        acc_seen += 1;
                    }
                    Torsion::Singular => unreachable!(),
                }
            }
            assert!(even_seen > 10 && three_seen > 3 && acc_seen > 5);
        }
    }

    #[test]
    fn psi3_root_count_matches_brute_force() {
        let p = 10007u64;
        let ctx = small_ctx(p);
        for b in 1..80u64 {
            let bb = BigUint::from(b);
            if ctx.b2_minus_4(&bb).is_zero() {
                continue;
            }
            let brute = (0..p)
                .filter(|&x| {
                    let x2 = x * x % p;
                    let v =
                        (3 * x2 % p * x2 % p + 12 * b % p * x % p + 2 * p * p - 18 * x2 - 9) % p;
                    v == 0
                })
                .count();
            assert_eq!(psi3_roots(&ctx, &bb).len(), brute, "b={b}");
        }
    }

    #[test]
    fn cubic_roots_match_brute_force_for_p_1_mod_4() {
        let p = 10009u64;
        let ctx = small_ctx(p);
        for b in 1..80u64 {
            let bb = BigUint::from(b);
            if ctx.b2_minus_4(&bb).is_zero() {
                continue;
            }
            let brute = (0..p)
                .filter(|&x| (x * x % p * x % p + 3 * p - 3 * x + b).is_multiple_of(p))
                .count();
            assert_eq!(cubic_root_count(&ctx, &bb), brute, "b={b}");
        }
    }

    #[test]
    fn naf_weight_matches_direct_recoding() {
        let mut s = Stream::new("naf-test");
        for _ in 0..200 {
            let v = low_u64(&s.bits(48));
            let (mut k, mut w) = (v as i128, 0u64);
            while k != 0 {
                if k & 1 == 1 {
                    let d = 2 - k.rem_euclid(4);
                    k -= d;
                    w += 1;
                }
                k >>= 1;
            }
            assert_eq!(naf_weight(&BigUint::from(v)), w, "v={v}");
        }
    }

    #[test]
    fn run_and_hamming_basics() {
        assert_eq!(longest_run(&BigUint::from(0b0011_1100u32), 8), 4);
        assert_eq!(longest_run(&BigUint::from(0u32), 8), 8);
        assert_eq!(hamming(&BigUint::from(0xF0F0u32)), 8);
    }

    #[test]
    fn smooth_bits_counts_multiplicity() {
        let ctx = small_ctx(10007);
        let x = BigUint::from(2u32 * 2 * 3 * 65521) * BigUint::from(1_000_003u64);
        let want = 2.0 + 3f64.log2() + 65521f64.log2();
        assert!((ctx.smooth_bits(&x) - want).abs() < 1e-9);
    }

    #[test]
    fn depth_counts_power_residue_order() {
        let curve = &nist_seeded_curves()[1]; // P-224, v2(p-1) = 96
        let ctx = Ctx::new(&curve.params.p);
        let r = BigUint::from(123456789u64);
        let x = r.modpow(&BigUint::from(1u32 << 12), &ctx.p);
        assert!(ctx.depth_bits(&x) >= 12.0);
        let y = r.modpow(&BigUint::from(3u32), &ctx.p);
        assert!(ctx.depth_bits(&y) >= 1.58);
    }

    #[test]
    fn real_nist_curves_pass_the_conditioning_and_their_seeds() {
        for curve in nist_seeded_curves() {
            let ctx = Ctx::new(&curve.params.p);
            let seed = hex::decode(curve.seed_hex).unwrap();
            let c = x962_c_from_seed(&seed, ctx.t);
            assert_eq!(ctx.c_from_b(&curve.params.b), ctx.mul(&c, &BigUint::one()));
            let inst = evaluate(&ctx, &curve.params.b, &c)
                .unwrap_or_else(|t| panic!("{}: {t:?}", curve.params.name));
            assert!(inst.stats.iter().all(|v| v.is_finite()));
        }
    }

    #[test]
    fn null_draw_is_deterministic_and_conditioned() {
        let curve = &nist_seeded_curves()[0];
        let ctx = Ctx::new(&curve.params.p);
        let (a, ka) = null_instance(&ctx, "test", "P-192", 3);
        let (b, kb) = null_instance(&ctx, "test", "P-192", 3);
        assert_eq!(a.b, b.b);
        assert_eq!(ka, kb);
        assert!(matches!(
            torsion_profile(&ctx, &a.b),
            Torsion::Accepted { .. }
        ));
        assert_eq!(ctx.mul(&ctx.mul(&a.b, &a.b), &a.c), &ctx.p - 27u8);
    }

    #[test]
    fn plants_exceed_the_null_on_their_own_statistic() {
        let curve = &nist_seeded_curves()[1];
        let ctx = Ctx::new(&curve.params.p);
        let nulls: Vec<_> = (0..40)
            .map(|i| null_instance(&ctx, "plant-test", "P-224", i).0)
            .collect();
        let mut s = Stream::new("plant-test-stream");
        for stat in 0..NSTAT - 1 {
            let col = NullColumn::new(&nulls.iter().map(|n| n.stats[stat]).collect::<Vec<_>>());
            let inst = plant(&ctx, stat, &mut s, 400)
                .unwrap_or_else(|| panic!("no plant for {}", STAT_NAMES[stat]));
            assert!(
                inst.stats[stat] > col.max(),
                "{}: planted {} vs null max {}",
                STAT_NAMES[stat],
                inst.stats[stat],
                col.max()
            );
        }
    }

    #[test]
    fn p_upper_is_add_one_and_counts_ties() {
        let col = NullColumn::new(&[1.0, 2.0, 2.0, 3.0]);
        assert_eq!(col.p_upper(2.0), 4.0 / 5.0);
        assert_eq!(col.p_upper(10.0), 1.0 / 5.0);
        assert_eq!(col.p_upper(0.0), 1.0);
    }
}
