//! # Genus ladder: the hyperelliptic DLP at genus 2, 3, 4, 5 over one field.
//!
//! Protocol: `research/genus_ladder_20261114/PROTOCOL.md`.  Every figure this
//! module produces is `[pcc:unverified]` until an independent replay.
//!
//! The genus-3 positive control (`chained_transfer::ic_control`) measures
//! index calculus against rho on one genus.  This module is the same
//! instrument with the genus as a variable: random imaginary hyperelliptic
//! curves `y² = f(x)`, `deg f = 2g + 1`, `g ∈ {2, 3, 4, 5}`, all over the
//! same `F_p`, solved by
//!
//! * [`Arm::Full`]: index calculus over every degree-1 place;
//! * [`Arm::DoubleLp`]: a reduced base with at most two large primes;
//! * [`run_rho`]: the lab's negation-map rho (`rho_reference_walk`).
//!
//! The relation graph, peeling, Wiedemann and the certificate are the code of
//! `ic_control` (`build_rows`, `peel`, `wiedemann`), not a copy.
//!
//! ## What is counted
//!
//! * **Group operations**: every Cantor addition or doubling (`GroupOps`).
//! * **Field operations behind them**: [`JCurve`] tallies the `F_p`
//!   multiplications and inversions of every Cantor step (`fmul`, `finv`).
//!   One group operation costs more at genus 5 than at genus 2, so a
//!   comparison across genera needs this tally; a comparison inside one genus
//!   can use group operations as before.
//! * **Native operations** of the relation phase (factor base, smoothness,
//!   roots) and of the linear algebra, as in `ic_control` ([`Nat`]).
//!
//! ## The parity prefilter
//!
//! A squarefree `u` of degree `d` over `F_p` with `k` irreducible factors has
//! `χ(disc u) = (−1)^{d−k}` (Pellet 1878, Stickelberger 1897).  A `u` that
//! splits completely has `k = d`, so its discriminant is a non-zero square.
//! [`IcParams::prefilter`] computes `χ(disc u)` by a pseudo-remainder chain
//! (no inversion) and one table look-up, and runs the `x^p mod u` test only
//! when it is `+1`.  The filter is exact: it rejects no `u` that the
//! unfiltered test accepts, so the relations, the trial count and the answer
//! are identical and only the native multiplications change.  At degree 2 it
//! decides smoothness outright.
//!
//! ## Scope
//!
//! Odd prime `p < 2^31` with `(√p + 1)^{2g} < 2^63`; `#J = h·r` with `r`
//! prime.  Synthetic random curves only.  No claim about any deployed curve.

use std::cell::Cell;
use std::collections::HashMap;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use super::chained_transfer::ic_control::{
    addm, build_rows, combine, mulmod, pa, peel, pm, ppow, ps, split_order, subm, wiedemann,
    wiedemann_coord, CubicOracle, Dsu, Nat, Rel, Row, REL_FB,
};
use super::ic_boundary::{
    invmod, pivot_inversion_steps, rho_walk_with_policy, CountedGroup, GroupOps, NegationClasses,
    PointClasses, RhoWalk, WalkPolicy,
};
use super::residual_walk::{is_prime_u64, mix64};

/// Largest genus the fixed arrays hold.
pub const MAX_GENUS: usize = 5;
/// Polynomial capacity: the largest Cantor intermediate at `g = 5` is
/// `v²` with `deg v ≤ 2g − 1`, 19 coefficients.
const CAP: usize = 24;

// ── polynomials over F_p on the stack ───────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Poly {
    c: [u64; CAP],
    n: usize,
}

impl Poly {
    const ZERO: Poly = Poly { c: [0; CAP], n: 0 };

    fn constant(a: u64) -> Poly {
        let mut q = Poly::ZERO;
        if a != 0 {
            q.c[0] = a;
            q.n = 1;
        }
        q
    }
    fn from_slice(s: &[u64]) -> Poly {
        assert!(s.len() <= CAP, "polynomial past capacity");
        let mut q = Poly::ZERO;
        q.c[..s.len()].copy_from_slice(s);
        q.n = s.len();
        q.trim();
        q
    }
    fn trim(&mut self) {
        while self.n > 0 && self.c[self.n - 1] == 0 {
            self.n -= 1;
        }
    }
    fn is_zero(&self) -> bool {
        self.n == 0
    }
    fn deg(&self) -> usize {
        self.n.saturating_sub(1)
    }
    fn lead(&self) -> u64 {
        if self.n == 0 {
            0
        } else {
            self.c[self.n - 1]
        }
    }
}

/// `F_p` with a tally of the multiplications and inversions spent in the
/// group law.
#[derive(Clone, Debug)]
struct Fk {
    p: u64,
    mul: Cell<u64>,
    inv: Cell<u64>,
}

impl Fk {
    #[inline]
    fn tick(&self, n: usize) {
        self.mul.set(self.mul.get() + n as u64);
    }
    fn inv(&self, a: u64) -> u64 {
        debug_assert!(a != 0 && a < self.p);
        self.inv.set(self.inv.get() + 1);
        let (mut r0, mut r1) = (self.p as i64, a as i64);
        let (mut t0, mut t1) = (0i64, 1i64);
        while r1 != 0 {
            let q = r0 / r1;
            (r0, r1) = (r1, r0 - q * r1);
            (t0, t1) = (t1, t0 - q * t1);
        }
        t0.rem_euclid(self.p as i64) as u64
    }
    fn add(&self, a: &Poly, b: &Poly) -> Poly {
        let p = self.p;
        let mut q = Poly::ZERO;
        let n = a.n.max(b.n);
        for i in 0..n {
            let s = a.c[i] + b.c[i];
            q.c[i] = if s >= p { s - p } else { s };
        }
        q.n = n;
        q.trim();
        q
    }
    fn sub(&self, a: &Poly, b: &Poly) -> Poly {
        let p = self.p;
        let mut q = Poly::ZERO;
        let n = a.n.max(b.n);
        for i in 0..n {
            q.c[i] = if a.c[i] >= b.c[i] {
                a.c[i] - b.c[i]
            } else {
                a.c[i] + p - b.c[i]
            };
        }
        q.n = n;
        q.trim();
        q
    }
    fn neg(&self, a: &Poly) -> Poly {
        let mut q = *a;
        for i in 0..a.n {
            q.c[i] = (self.p - a.c[i]) % self.p;
        }
        q
    }
    fn scale(&self, a: &Poly, s: u64) -> Poly {
        let mut q = Poly::ZERO;
        for i in 0..a.n {
            q.c[i] = a.c[i] * s % self.p;
        }
        self.tick(a.n);
        q.n = a.n;
        q.trim();
        q
    }
    /// Schoolbook product, `a.n · b.n` multiplications.
    fn mul(&self, a: &Poly, b: &Poly) -> Poly {
        if a.is_zero() || b.is_zero() {
            return Poly::ZERO;
        }
        let n = a.n + b.n - 1;
        assert!(n <= CAP, "polynomial product past capacity");
        let mut acc = [0u128; CAP];
        for i in 0..a.n {
            let ai = a.c[i] as u128;
            for j in 0..b.n {
                acc[i + j] += ai * b.c[j] as u128;
            }
        }
        self.tick(a.n * b.n);
        let mut q = Poly::ZERO;
        let p = self.p as u128;
        for i in 0..n {
            q.c[i] = (acc[i] % p) as u64;
        }
        q.n = n;
        q.trim();
        q
    }
    /// `(a div b, a mod b)`, `b ≠ 0`.
    fn divrem(&self, a: &Poly, b: &Poly) -> (Poly, Poly) {
        assert!(!b.is_zero(), "division by the zero polynomial");
        let p = self.p;
        let mut r = *a;
        let mut q = Poly::ZERO;
        if a.n < b.n {
            return (q, r);
        }
        let monic = b.lead() == 1;
        let li = if monic { 1 } else { self.inv(b.lead()) };
        let db = b.deg();
        while r.n >= b.n && !r.is_zero() {
            let shift = r.deg() - db;
            let coef = if monic {
                r.lead()
            } else {
                self.tick(1);
                r.lead() * li % p
            };
            q.c[shift] = coef;
            for j in 0..db {
                let t = coef * b.c[j] % p;
                let x = r.c[shift + j];
                r.c[shift + j] = if x >= t { x - t } else { x + p - t };
            }
            self.tick(db);
            r.c[shift + db] = 0;
            r.trim();
        }
        q.n = a.n - b.n + 1;
        q.trim();
        (q, r)
    }
    fn rem(&self, a: &Poly, b: &Poly) -> Poly {
        self.divrem(a, b).1
    }
    fn monic(&self, a: &Poly) -> Poly {
        if a.is_zero() || a.lead() == 1 {
            return *a;
        }
        self.scale(a, self.inv(a.lead()))
    }
    /// `(g, s, t)` with `s·a + t·b = g`, `g` monic.
    fn xgcd(&self, a: &Poly, b: &Poly) -> (Poly, Poly, Poly) {
        let (mut r0, mut r1) = (*a, *b);
        let (mut s0, mut s1) = (Poly::constant(1), Poly::ZERO);
        let (mut t0, mut t1) = (Poly::ZERO, Poly::constant(1));
        while !r1.is_zero() {
            let (q, r) = self.divrem(&r0, &r1);
            r0 = r1;
            r1 = r;
            let s = self.sub(&s0, &self.mul(&q, &s1));
            s0 = s1;
            s1 = s;
            let t = self.sub(&t0, &self.mul(&q, &t1));
            t0 = t1;
            t1 = t;
        }
        if r0.is_zero() || r0.lead() == 1 {
            return (r0, s0, t0);
        }
        let li = self.inv(r0.lead());
        (
            self.scale(&r0, li),
            self.scale(&s0, li),
            self.scale(&t0, li),
        )
    }
    fn eval(&self, a: &Poly, x: u64) -> u64 {
        let mut acc = 0;
        for i in (0..a.n).rev() {
            acc = (acc * x + a.c[i]) % self.p;
        }
        acc
    }
}

fn pow_mod(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = r * a % p;
        }
        a = a * a % p;
        e >>= 1;
    }
    r
}

/// A square root of `a` modulo the odd prime `p` (Tonelli–Shanks), uncounted
/// (instance set-up only).
fn sqrt_mod(a: u64, p: u64) -> Option<u64> {
    if a == 0 {
        return Some(0);
    }
    if pow_mod(a, (p - 1) / 2, p) != 1 {
        return None;
    }
    if p % 4 == 3 {
        return Some(pow_mod(a, p.div_ceil(4), p));
    }
    let (mut q, mut s) = (p - 1, 0u32);
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2;
    while pow_mod(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }
    let (mut m, mut c, mut t, mut r) = (
        s,
        pow_mod(z, q, p),
        pow_mod(a, q, p),
        pow_mod(a, q.div_ceil(2), p),
    );
    while t != 1 {
        let mut i = 0;
        let mut tt = t;
        while tt != 1 {
            tt = tt * tt % p;
            i += 1;
        }
        let b = pow_mod(c, 1 << (m - i - 1), p);
        m = i;
        c = b * b % p;
        t = t * c % p;
        r = r * b % p;
    }
    Some(r)
}

// ── the Jacobian ────────────────────────────────────────────────────────

/// A reduced Mumford divisor `(u, v)`, `u` monic of degree `deg ≤ g` (its
/// non-leading coefficients stored), `deg v < deg u`; unused slots are zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct JDiv {
    pub deg: u8,
    pub u: [u32; MAX_GENUS],
    pub v: [u32; MAX_GENUS],
}

impl JDiv {
    pub const IDENTITY: JDiv = JDiv {
        deg: 0,
        u: [0; MAX_GENUS],
        v: [0; MAX_GENUS],
    };
    pub fn is_identity(&self) -> bool {
        self.deg == 0
    }
}

/// `y² = f(x)` of genus `g ≤ 5` over `F_p`, `deg f = 2g + 1`, `f`
/// squarefree, with Cantor's arithmetic and a tally of its field work.
#[derive(Clone, Debug)]
pub struct JCurve {
    k: Fk,
    genus: u8,
    f: Poly,
}

impl JCurve {
    pub fn new(p: u64, genus: u32, f: &[u64]) -> Result<JCurve, String> {
        if !(3..(1 << 31)).contains(&p) || !is_prime_u64(p) {
            return Err(format!("p must be an odd prime below 2^31, not {p}"));
        }
        if !(1..=MAX_GENUS as u32).contains(&genus) {
            return Err(format!("genus must be 1 to {MAX_GENUS}, not {genus}"));
        }
        if ((p as f64).sqrt() + 1.0).powi(2 * genus as i32) >= 9.0e18 {
            return Err(format!("#J does not fit 63 bits at p = {p}, g = {genus}"));
        }
        if f.iter().any(|&c| c >= p) {
            return Err("coefficient not reduced".into());
        }
        if f.len() != 2 * genus as usize + 2 || f[f.len() - 1] == 0 {
            return Err(format!("deg f must be 2g + 1 = {}", 2 * genus + 1));
        }
        let k = Fk {
            p,
            mul: Cell::new(0),
            inv: Cell::new(0),
        };
        let fp = Poly::from_slice(f);
        let mut d = Poly::ZERO;
        for i in 1..fp.n {
            d.c[i - 1] = fp.c[i] * (i as u64 % p) % p;
        }
        d.n = fp.n - 1;
        d.trim();
        if d.is_zero() || k.xgcd(&fp, &d).0.n != 1 {
            return Err("f is not squarefree".into());
        }
        k.mul.set(0);
        k.inv.set(0);
        Ok(JCurve {
            k,
            genus: genus as u8,
            f: fp,
        })
    }

    pub fn p(&self) -> u64 {
        self.k.p
    }
    pub fn genus(&self) -> u32 {
        u32::from(self.genus)
    }
    pub fn f_coeffs(&self) -> Vec<u64> {
        self.f.c[..self.f.n].to_vec()
    }
    /// `F_p` multiplications spent in the group law since construction.
    pub fn fmul(&self) -> u64 {
        self.k.mul.get()
    }
    /// `F_p` inversions spent in the group law since construction.
    pub fn finv(&self) -> u64 {
        self.k.inv.get()
    }

    fn polys(&self, d: &JDiv) -> (Poly, Poly) {
        let n = d.deg as usize;
        let mut u = Poly::ZERO;
        let mut v = Poly::ZERO;
        for i in 0..n {
            u.c[i] = u64::from(d.u[i]);
            v.c[i] = u64::from(d.v[i]);
        }
        u.c[n] = 1;
        u.n = n + 1;
        v.n = n;
        v.trim();
        (u, v)
    }

    fn pack(&self, u: &Poly, v: &Poly) -> JDiv {
        debug_assert!(u.lead() == 1 && u.deg() <= self.genus as usize);
        debug_assert!(v.is_zero() || v.deg() < u.deg());
        let mut d = JDiv {
            deg: u.deg() as u8,
            ..JDiv::IDENTITY
        };
        for i in 0..u.deg() {
            d.u[i] = u.c[i] as u32;
        }
        for i in 0..v.n {
            d.v[i] = v.c[i] as u32;
        }
        d
    }

    /// `u`, lowest degree first, with its leading 1.
    pub fn u_coeffs(&self, d: &JDiv) -> Vec<u64> {
        let mut out: Vec<u64> = d.u[..d.deg as usize]
            .iter()
            .map(|&c| u64::from(c))
            .collect();
        out.push(1);
        out
    }
    /// `v`, lowest degree first, `deg` entries.
    pub fn v_coeffs(&self, d: &JDiv) -> Vec<u64> {
        d.v[..d.deg as usize]
            .iter()
            .map(|&c| u64::from(c))
            .collect()
    }

    /// `D` is a reduced divisor on this curve.
    pub fn is_valid(&self, d: &JDiv) -> bool {
        let n = d.deg as usize;
        if n > self.genus as usize {
            return false;
        }
        let p = self.k.p;
        if d.u.iter().chain(d.v.iter()).any(|&c| u64::from(c) >= p) {
            return false;
        }
        if d.u[n..].iter().chain(d.v[n..].iter()).any(|&c| c != 0) {
            return false;
        }
        let (u, v) = self.polys(d);
        let t = self.k.sub(&self.f, &self.k.mul(&v, &v));
        self.k.rem(&t, &u).is_zero()
    }

    /// `[P] − [∞]` for an affine point.
    pub fn point(&self, x: u64, y: u64) -> Option<JDiv> {
        let p = self.k.p;
        if x >= p || y >= p || y * y % p != self.k.eval(&self.f, x) {
            return None;
        }
        let mut d = JDiv {
            deg: 1,
            ..JDiv::IDENTITY
        };
        d.u[0] = ((p - x) % p) as u32;
        d.v[0] = y as u32;
        Some(d)
    }

    /// The sum of `g` random affine points, from a SplitMix64 stream.
    pub fn random_divisor(&self, state: &mut u64) -> JDiv {
        let p = self.k.p;
        let mut d = JDiv::IDENTITY;
        let mut placed = 0;
        while placed < self.genus {
            *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let h = mix64(*state);
            let x = h % p;
            let Some(s) = sqrt_mod(self.k.eval(&self.f, x), p) else {
                continue;
            };
            let y = if (h >> 63) & 1 == 1 { (p - s) % p } else { s };
            let pt = self.point(x, y).expect("a lifted point lies on the curve");
            d = self.cantor(d, pt);
            placed += 1;
        }
        d
    }

    /// Cantor's composition and reduction.
    pub fn cantor(&self, a: JDiv, b: JDiv) -> JDiv {
        if a.is_identity() {
            return b;
        }
        if b.is_identity() {
            return a;
        }
        let k = &self.k;
        let (u1, v1) = self.polys(&a);
        let (u2, v2) = self.polys(&b);
        let (d1, e1, e2) = k.xgcd(&u1, &u2);
        let (u, v) = if d1.n == 1 {
            let u = k.mul(&u1, &u2);
            let t = k.mul(&k.mul(&e1, &u1), &k.sub(&v2, &v1));
            (u, k.rem(&k.add(&v1, &t), &u))
        } else {
            let (d, c1, c2) = k.xgcd(&d1, &k.add(&v1, &v2));
            let s1 = k.mul(&c1, &e1);
            let s2 = k.mul(&c1, &e2);
            let (u, ur) = k.divrem(&k.mul(&u1, &u2), &k.mul(&d, &d));
            debug_assert!(ur.is_zero(), "u₁u₂/d² is exact");
            // (s₁u₁v₂ + s₂u₂v₁ + c₂(v₁v₂ + f)) / d; degree at most 4g − 3.
            let t1 = k.mul(&k.mul(&s1, &u1), &v2);
            let t2 = k.mul(&k.mul(&s2, &u2), &v1);
            let t3 = k.mul(&c2, &k.add(&k.mul(&v1, &v2), &self.f));
            let (vq, vr) = k.divrem(&k.add(&k.add(&t1, &t2), &t3), &d);
            debug_assert!(vr.is_zero(), "the v numerator divides by d");
            (u, k.rem(&vq, &u))
        };
        self.reduce(u, v)
    }

    fn reduce(&self, mut u: Poly, mut v: Poly) -> JDiv {
        let k = &self.k;
        let g = self.genus as usize;
        while u.deg() > g {
            let t = k.sub(&self.f, &k.mul(&v, &v));
            let (q, r) = k.divrem(&t, &u);
            debug_assert!(r.is_zero(), "(f − v²)/u is exact");
            let un = k.monic(&q);
            v = k.rem(&k.neg(&v), &un);
            u = un;
        }
        let u = k.monic(&u);
        self.pack(&u, &v)
    }

    pub fn negate(&self, d: JDiv) -> JDiv {
        let p = self.k.p;
        let mut n = d;
        for i in 0..d.deg as usize {
            n.v[i] = ((p - u64::from(d.v[i])) % p) as u32;
        }
        n
    }

    /// `[k]D`, left-to-right.
    pub fn mul_ltr(&self, d: JDiv, k: u64) -> JDiv {
        let mut acc = JDiv::IDENTITY;
        for i in (0..64 - k.leading_zeros()).rev() {
            acc = self.cantor(acc, acc);
            if (k >> i) & 1 == 1 {
                acc = self.cantor(acc, d);
            }
        }
        acc
    }

    /// `[k]D`, right-to-left: the independent path of the certificates.
    pub fn mul_rtl(&self, d: JDiv, mut k: u64) -> JDiv {
        let mut acc = JDiv::IDENTITY;
        let mut base = d;
        while k > 0 {
            if k & 1 == 1 {
                acc = self.cantor(acc, base);
            }
            k >>= 1;
            if k > 0 {
                base = self.cantor(base, base);
            }
        }
        acc
    }

    /// A 64-bit fingerprint of the divisor (not an injection).
    pub fn key_of(&self, d: &JDiv) -> u64 {
        let mut h = 0x243F_6A88_85A3_08D3u64 ^ u64::from(d.deg);
        for i in 0..d.deg as usize {
            h = mix64(h ^ (u64::from(d.u[i]) << 32 | u64::from(d.v[i])));
        }
        h
    }

    // ── point counts and the group order ────────────────────────────

    /// `#C(F_{p^k})` with the point at infinity, `k ∈ {1, 2}`, by enumeration.
    pub fn count_points(&self, k: u32) -> u64 {
        assert!((1..=2).contains(&k));
        let p = self.k.p;
        let mut sq = vec![false; p as usize];
        for y in 1..p {
            sq[(y * y % p) as usize] = true;
        }
        if k == 1 {
            let mut n = 1u64;
            for x in 0..p {
                let y2 = self.k.eval(&self.f, x);
                n += if y2 == 0 {
                    1
                } else if sq[y2 as usize] {
                    2
                } else {
                    0
                };
            }
            return n;
        }
        let nu = (2..p).find(|&a| !sq[a as usize]).expect("a non-residue");
        let f = &self.f;
        assert!(p < (1 << 31), "count_points(2) wants p < 2^31");
        // Horner over F_p[√ν] with one Barrett reduction per coordinate and
        // step (the reference is `count_points2_reference` in the tests).
        let m = u64::MAX / p;
        let red = |x: u64| -> u64 {
            let q = ((u128::from(x) * u128::from(m)) >> 64) as u64;
            let r = x - q * p;
            if r >= p {
                r - p
            } else {
                r
            }
        };
        let nub: Vec<u64> = (0..=(p - 1) / 2).map(|b| nu * b % p).collect();
        let mut n = 1u64;
        for a in 0..p {
            for b in 0..=(p - 1) / 2 {
                let (mut ra, mut rb) = (0u64, 0u64);
                let nb_ = nub[b as usize];
                for i in (0..f.n).rev() {
                    let na = red(ra * a + rb * nb_ + f.c[i]);
                    let nb = red(ra * b + rb * a);
                    ra = na;
                    rb = nb;
                }
                let w = if b == 0 { 1 } else { 2 };
                if ra == 0 && rb == 0 {
                    n += w;
                } else {
                    let norm = red(ra * ra + p * p - nu * red(rb * rb));
                    if sq[norm as usize] {
                        n += 2 * w;
                    }
                }
            }
        }
        n
    }

    /// `#J(F_p)` by the enumeration route: [`OrderMethod::Enumerate`].
    pub fn jacobian_order(&self, seed: u64) -> Result<u64, String> {
        self.jacobian_order_with(OrderMethod::Enumerate, seed)
    }

    /// `#J(F_p)`: the first `known` coefficients of `L(T)` from point counts
    /// (`known = min(g − 1, 2)` for `Enumerate` and `Bsgs2`, `min(g − 1, 1)`
    /// for `Bsgs1`), then baby-step giant-step over the window the Weil
    /// bounds leave for the remaining ones; every candidate is checked on 32
    /// random divisors and exactly one must survive.
    pub fn jacobian_order_with(&self, method: OrderMethod, seed: u64) -> Result<u64, String> {
        let g = self.genus as usize;
        let p = i128::from(self.k.p);
        let known = known_coefficients(g, method);
        let s: Vec<i128> = (1..=known as u32)
            .map(|k| self.count_points(k) as i128 - p.pow(k) - 1)
            .collect();
        let mut c = vec![0i128; known + 1];
        c[0] = 1;
        for k in 1..=known {
            let num: i128 = (1..=k).map(|i| s[i - 1] * c[k - i]).sum();
            if num % k as i128 != 0 {
                return Err("Newton integrality".into());
            }
            c[k] = num / k as i128;
        }
        let coef = |i: usize| -> i128 {
            if i < g {
                1 + p.pow((g - i) as u32)
            } else {
                1
            }
        };
        let base: i128 = (0..=known).map(|i| c[i] * coef(i)).sum();
        let (bound, lo, hi) = weil_window(self.k.p, g, known);
        let from = (base - bound).max(lo).max(1);
        let to = (base + bound).min(hi);
        if to < from {
            return Err("empty window".into());
        }
        let width = (to - from) as u64;
        let m = (((width + 1) as f64).sqrt().ceil() as u64).max(1);
        if m > MAX_BSGS_TABLE {
            return Err(format!(
                "order window too wide for baby-step giant-step (m = {m})"
            ));
        }
        let mut st = seed ^ 0xA5A5_5A5A;
        let mut cands = Vec::new();
        let mut complete = false;
        for _ in 0..8 {
            let d0 = self.random_divisor(&mut st);
            // Baby steps j·D0, j = 0..=m.  They are pairwise distinct exactly
            // when ord(D0) > m, and only then does the giant-step scan below
            // find every n in the window with n·D0 = 0.  A probe of smaller
            // order is discarded.
            let mut baby: HashMap<u64, u32> = HashMap::with_capacity(m as usize + 1);
            let mut cur = JDiv::IDENTITY;
            let mut small = false;
            for j in 0..=m {
                if j > 0 && cur.is_identity() {
                    small = true;
                    break;
                }
                if baby.insert(self.key_of(&cur), j as u32).is_some() {
                    small = true; // a fingerprint collision: treat alike
                    break;
                }
                cur = self.cantor(cur, d0);
            }
            if small {
                continue;
            }
            let w = self.negate(self.mul_ltr(d0, from as u64));
            let nmd0 = self.negate(self.mul_ltr(d0, m));
            let mut q = w;
            for i in 0..=(width / m + 1) {
                if let Some(&j) = baby.get(&self.key_of(&q)) {
                    let sidx = i * m + u64::from(j);
                    if sidx <= width {
                        let n = from as u64 + sidx;
                        if self.mul_ltr(d0, n).is_identity() {
                            cands.push(n);
                        }
                    }
                }
                q = self.cantor(q, nmd0);
            }
            complete = true;
            break;
        }
        if !complete {
            return Err("no candidate: every probe divisor had small order".into());
        }
        cands.sort_unstable();
        cands.dedup();
        let tests: Vec<JDiv> = (0..32).map(|_| self.random_divisor(&mut st)).collect();
        let surv: Vec<u64> = cands
            .into_iter()
            .filter(|&n| tests.iter().all(|t| self.mul_ltr(*t, n).is_identity()))
            .collect();
        match surv.as_slice() {
            [one] => Ok(*one),
            [] => Err("no candidate".into()),
            many => Err(format!("ambiguous: {} candidates", many.len())),
        }
    }
}

/// How `#J(F_p)` is obtained: which point counts are enumerated before the
/// baby-step giant-step over the window the Weil bounds leave.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrderMethod {
    /// `N_1`, and `N_2` for `g ≥ 3` (O(p²) enumeration over `F_{p²}`).
    Enumerate,
    /// `N_1` only; the window covers every remaining coefficient.
    Bsgs1,
    /// `N_1` and `N_2` (the `Enumerate` window; the same counts).
    Bsgs2,
}

impl OrderMethod {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "enumerate" => Ok(Self::Enumerate),
            "bsgs1" => Ok(Self::Bsgs1),
            "bsgs2" => Ok(Self::Bsgs2),
            _ => Err(format!(
                "--order-method wants enumerate, bsgs1 or bsgs2, not {s}"
            )),
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Enumerate => "enumerate",
            Self::Bsgs1 => "bsgs1",
            Self::Bsgs2 => "bsgs2",
        }
    }
}

/// The most baby steps held in the table (`2^26` entries).
pub const MAX_BSGS_TABLE: u64 = 1 << 26;

fn known_coefficients(g: usize, method: OrderMethod) -> usize {
    let enumerated = match g {
        1 => 0,
        2 => 1,
        _ => 2,
    };
    match method {
        OrderMethod::Enumerate | OrderMethod::Bsgs2 => enumerated,
        OrderMethod::Bsgs1 => enumerated.min(1),
    }
}

/// `(bound, lo, hi)`: the half-width the Weil bounds leave for `#J` once the
/// first `known` coefficients are fixed, and the absolute range of `#J`.
fn weil_window(p: u64, g: usize, known: usize) -> (i128, i128, i128) {
    let p = i128::from(p);
    let coef = |i: usize| -> i128 {
        if i < g {
            1 + p.pow((g - i) as u32)
        } else {
            1
        }
    };
    let mut bound = 1i128;
    for i in known + 1..=g {
        let binom = (0..i as i128).fold(1i128, |acc, j| acc * (2 * g as i128 - j) / (j + 1));
        let w = (binom as f64 * (p as f64).powf(i as f64 / 2.0)).ceil() as i128 + 1;
        bound += w * coef(i);
    }
    let sp = (p as f64).sqrt();
    let lo = ((sp - 1.0).powi(2 * g as i32) * (1.0 - 1e-9))
        .floor()
        .max(1.0) as i128;
    let hi = ((sp + 1.0).powi(2 * g as i32) * (1.0 + 1e-9)).ceil() as i128;
    (bound, lo, hi)
}

/// The baby-step table size `m` of the widest window (centre ignored, Weil
/// range not binding) for `(p, g, method)`: the memory bound of the route.
pub fn bsgs_table_size(p: u64, g: usize, method: OrderMethod) -> u64 {
    let (bound, lo, hi) = weil_window(p, g, known_coefficients(g, method));
    let width = (2 * bound).min(hi - lo) as f64;
    (width + 1.0).sqrt().ceil() as u64
}

impl CountedGroup for JCurve {
    type Elt = JDiv;
    fn identity(&self) -> JDiv {
        JDiv::IDENTITY
    }
    fn is_identity(&self, d: &JDiv) -> bool {
        d.is_identity()
    }
    fn add(&self, ops: &mut GroupOps, a: JDiv, b: JDiv) -> JDiv {
        if a == b {
            ops.doubles += 1;
        } else {
            ops.adds += 1;
        }
        self.cantor(a, b)
    }
    fn double(&self, ops: &mut GroupOps, a: JDiv) -> JDiv {
        ops.doubles += 1;
        self.cantor(a, a)
    }
    fn neg(&self, a: JDiv) -> JDiv {
        self.negate(a)
    }
    fn key(&self, d: &JDiv) -> u64 {
        self.key_of(d)
    }
}

// ── instances ───────────────────────────────────────────────────────────

/// A Jacobian with `#J = h·r`, `r` prime, a generator `G` of the order-`r`
/// subgroup and a target `T = [t]G`.
#[derive(Clone, Debug)]
pub struct Instance {
    pub curve: JCurve,
    pub p: u64,
    pub genus: u32,
    pub order: u64,
    pub h: u64,
    pub r: u64,
    pub g: JDiv,
    pub t_secret: u64,
    pub target: JDiv,
}

/// Rabin's irreducibility test for a monic `f` over `F_p` (coefficients lowest
/// degree first, `f.len() - 1 = n ≥ 1`): `f` is irreducible iff
/// `x^(p^n) ≡ x (mod f)` and `gcd(x^(p^(n/q)) − x, f) = 1` for every prime `q | n`.
///
/// Used to skip seeds in a prime-order search: for `y² = f(x)` with `deg f`
/// odd, `#J(F_p)[2] = 2^(m−1)` where `m` is the number of irreducible factors
/// of `f` over `F_p`, so a reducible `f` gives an even `#J` and cannot pass
/// `--max-h 1`.  Not counted (search bookkeeping, not part of any measured arm).
pub fn f_irreducible(p: u64, f: &[u64]) -> bool {
    let n = f.len() - 1;
    if n <= 1 {
        return n == 1;
    }
    let mut nat = Nat::default();
    let mulmod = |a: &[u64], b: &[u64], nat: &mut Nat| -> Vec<u64> {
        let mut c = vec![0u64; a.len() + b.len()];
        for (i, &x) in a.iter().enumerate() {
            if x == 0 {
                continue;
            }
            for (j, &y) in b.iter().enumerate() {
                c[i + j] = (c[i + j] + x * y % p) % p;
            }
        }
        vrem(p, &c, f, nat)
    };
    let powp = |a: &[u64], nat: &mut Nat| -> Vec<u64> {
        // a^p mod f by square-and-multiply
        let mut res = vec![1u64];
        let mut base = a.to_vec();
        let mut e = p;
        while e > 0 {
            if e & 1 == 1 {
                res = mulmod(&res, &base, nat);
            }
            base = mulmod(&base, &base, nat);
            e >>= 1;
        }
        res
    };
    let x = vec![0u64, 1];
    let minus_x = |a: &[u64]| -> Vec<u64> {
        let mut d = a.to_vec();
        if d.len() < 2 {
            d.resize(2, 0);
        }
        d[1] = (d[1] + p - 1) % p;
        vtrim(&mut d);
        d
    };
    let primes: Vec<usize> = (2..=n)
        .filter(|&q| n.is_multiple_of(q) && (2..q).all(|d| !q.is_multiple_of(d)))
        .collect();
    // frob[k] = x^(p^k) mod f, k = 0..=n
    let mut frob = vec![x.clone()];
    for k in 1..=n {
        let next = powp(&frob[k - 1], &mut nat);
        frob.push(next);
    }
    let fx = minus_x(&frob[n]);
    if !fx.is_empty() {
        return false;
    }
    for q in primes {
        let d = minus_x(&frob[n / q]);
        if d.is_empty() {
            return false;
        }
        let g = vgcd_monic(p, f, &d, &mut nat);
        if g.len() > 1 {
            return false;
        }
    }
    true
}

/// A random monic `f` of degree `2g + 1` from `(curve_seed, p, g)`.
pub fn random_f(p: u64, genus: u32, curve_seed: u64) -> Vec<u64> {
    let mut st = curve_seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ p ^ (u64::from(genus) << 48);
    let mut f = Vec::with_capacity(2 * genus as usize + 2);
    for _ in 0..2 * genus + 1 {
        st = st.wrapping_add(0x9E37_79B9_7F4A_7C15);
        f.push(mix64(st) % p);
    }
    f.push(1);
    f
}

/// Why a curve seed was not admitted, or the instance.
pub fn make_instance(
    p: u64,
    genus: u32,
    curve_seed: u64,
    target_seed: u64,
    max_h: u64,
) -> Result<Instance, String> {
    make_instance_with(
        p,
        genus,
        curve_seed,
        target_seed,
        max_h,
        OrderMethod::Enumerate,
    )
}

/// [`make_instance`] with the group order found by `method`.
pub fn make_instance_with(
    p: u64,
    genus: u32,
    curve_seed: u64,
    target_seed: u64,
    max_h: u64,
    method: OrderMethod,
) -> Result<Instance, String> {
    let curve = JCurve::new(p, genus, &random_f(p, genus, curve_seed))?;
    let order = curve.jacobian_order_with(method, curve_seed)?;
    let (h, r) = split_order(order, max_h).ok_or_else(|| format!("#J = {order} is not h*r"))?;
    instance_from(curve, order, h, r, target_seed)
}

pub fn instance_from(
    curve: JCurve,
    order: u64,
    h: u64,
    r: u64,
    target_seed: u64,
) -> Result<Instance, String> {
    let mut st = target_seed ^ 0x1234_5678_9ABC_DEF0;
    let mut g = JDiv::IDENTITY;
    for _ in 0..64 {
        let d = curve.random_divisor(&mut st);
        let cand = curve.mul_ltr(d, h);
        if !cand.is_identity() && curve.mul_ltr(cand, r).is_identity() {
            g = cand;
            break;
        }
    }
    if g.is_identity() {
        return Err("no generator of order r".into());
    }
    st = mix64(st);
    let t_secret = 1 + mix64(st) % (r - 1);
    let target = curve.mul_rtl(g, t_secret);
    Ok(Instance {
        p: curve.p(),
        genus: curve.genus(),
        curve,
        order,
        h,
        r,
        g,
        t_secret,
        target,
    })
}

// ── counted arithmetic of the relation phase ────────────────────────────

/// Residues modulo a monic `u` of degree `d ≤ 5`.
struct ModU {
    p: u64,
    d: usize,
    u: [u64; MAX_GENUS],
}

type Res = [u64; MAX_GENUS];

impl ModU {
    fn mul(&self, a: &Res, b: &Res, n: &mut Nat) -> Res {
        let (p, d) = (self.p, self.d);
        let mut t = [0u64; 2 * MAX_GENUS];
        for i in 0..d {
            for j in 0..d {
                t[i + j] = (t[i + j] + a[i] * b[j]) % p;
            }
        }
        n.mul += (d * d) as u64;
        n.add += (d * d) as u64;
        for k in (d..(2 * d).saturating_sub(1)).rev() {
            let c = t[k];
            for j in 0..d {
                t[k - d + j] = (t[k - d + j] + (p - self.u[j]) * c) % p;
            }
            n.mul += d as u64;
            n.add += d as u64;
        }
        let mut out = [0u64; MAX_GENUS];
        out[..d].copy_from_slice(&t[..d]);
        out
    }
    /// `a · (x + aa)`.
    fn mul_xpa(&self, a: &Res, aa: u64, n: &mut Nat) -> Res {
        let (p, d) = (self.p, self.d);
        let top = a[d - 1];
        let mut out = [0u64; MAX_GENUS];
        out[1..d].copy_from_slice(&a[..d - 1]);
        for j in 0..d {
            out[j] = (out[j] + (p - self.u[j]) * top + aa * a[j]) % p;
        }
        let work = if aa == 0 { d } else { 2 * d } as u64;
        n.mul += work;
        n.add += work;
        out
    }
    fn base(&self, aa: u64) -> Res {
        let mut b = [0u64; MAX_GENUS];
        if self.d == 1 {
            b[0] = (aa + self.p - self.u[0]) % self.p;
        } else {
            b[0] = aa;
            b[1] = 1;
        }
        b
    }
    /// `(x + aa)^e mod u`, `e ≥ 1`.
    fn pow_xpa(&self, aa: u64, e: u64, n: &mut Nat) -> Res {
        let mut res = self.base(aa);
        let bits = 64 - e.leading_zeros();
        for i in (0..bits - 1).rev() {
            res = self.mul(&res, &res, n);
            if (e >> i) & 1 == 1 {
                res = self.mul_xpa(&res, aa, n);
            }
        }
        res
    }
}

fn mod_u(p: u64, u: &[u64]) -> ModU {
    let d = u.len() - 1;
    let mut uu = [0u64; MAX_GENUS];
    uu[..d].copy_from_slice(&u[..d]);
    ModU { p, d, u: uu }
}

/// `u | x^p − x` for monic `u` of degree `1..=5`.
pub fn splits_completely(p: u64, u: &[u64], n: &mut Nat) -> bool {
    let ctx = mod_u(p, u);
    let xp = ctx.pow_xpa(0, p, n);
    xp[..ctx.d] == ctx.base(0)[..ctx.d]
}

fn vtrim(a: &mut Vec<u64>) {
    while a.last() == Some(&0) {
        a.pop();
    }
}

fn cinv(p: u64, a: u64, n: &mut Nat) -> u64 {
    // Fermat inverse, counted as its multiplications.
    let e = p - 2;
    n.mul += u64::from(64 - e.leading_zeros()) + u64::from(e.count_ones()) - 2;
    pow_mod(a, e, p)
}

fn vrem(p: u64, a: &[u64], m: &[u64], n: &mut Nat) -> Vec<u64> {
    let mut r = a.to_vec();
    vtrim(&mut r);
    let dm = m.len() - 1;
    let lead = m[dm];
    let li = if lead == 1 { 1 } else { cinv(p, lead, n) };
    while r.len() > dm {
        let top = r.len() - 1;
        let mut c = r[top];
        if li != 1 {
            c = c * li % p;
            n.mul += 1;
        }
        for j in 0..dm {
            let t = c * m[j] % p;
            let x = r[top - dm + j];
            r[top - dm + j] = if x >= t { x - t } else { x + p - t };
        }
        n.mul += dm as u64;
        n.add += dm as u64;
        r.pop();
        vtrim(&mut r);
    }
    r
}

fn vgcd_monic(p: u64, a: &[u64], b: &[u64], n: &mut Nat) -> Vec<u64> {
    let (mut a, mut b) = (a.to_vec(), b.to_vec());
    vtrim(&mut a);
    vtrim(&mut b);
    while !b.is_empty() {
        let r = vrem(p, &a, &b, n);
        a = b;
        b = r;
    }
    if let Some(&lead) = a.last() {
        if lead != 1 {
            let il = cinv(p, lead, n);
            for c in a.iter_mut() {
                *c = *c * il % p;
            }
            n.mul += a.len() as u64;
        }
    }
    a
}

/// `a / m` for monic `m` dividing `a`.
fn vdiv_monic(p: u64, a: &[u64], m: &[u64], n: &mut Nat) -> Vec<u64> {
    let dm = m.len() - 1;
    let mut r = a.to_vec();
    let mut q = vec![0u64; a.len() - dm];
    for top in (dm..a.len()).rev() {
        let c = r[top];
        q[top - dm] = c;
        for j in 0..dm {
            let t = c * m[j] % p;
            let x = r[top - dm + j];
            r[top - dm + j] = if x >= t { x - t } else { x + p - t };
        }
        n.mul += dm as u64;
        n.add += dm as u64;
    }
    q
}

/// The roots of a monic `u` known to split completely (Cantor–Zassenhaus).
pub fn roots_of(p: u64, u: &[u64], rng: &mut StdRng, n: &mut Nat) -> Option<Vec<u64>> {
    let d = u.len() - 1;
    if d == 0 {
        return Some(Vec::new());
    }
    if d == 1 {
        return Some(vec![(p - u[0]) % p]);
    }
    for _ in 0..64 {
        let aa = rng.gen_range(0..p);
        let ctx = mod_u(p, u);
        let h = ctx.pow_xpa(aa, (p - 1) / 2, n);
        let mut hm1: Vec<u64> = h[..d].to_vec();
        hm1[0] = (hm1[0] + p - 1) % p;
        vtrim(&mut hm1);
        if hm1.is_empty() {
            continue;
        }
        let g = vgcd_monic(p, u, &hm1, n);
        let dg = g.len() - 1;
        if dg == 0 || dg == d {
            continue;
        }
        let q = vdiv_monic(p, u, &g, n);
        let mut out = roots_of(p, &g, rng, n)?;
        out.extend(roots_of(p, &q, rng, n)?);
        return Some(out);
    }
    None
}

/// `χ(disc u)` for monic `u` of degree `d ≥ 2`: `0` if `u` has a repeated
/// root, else `(−1)^{d−k}` with `k` the number of irreducible factors.
/// `chi[a]` is the quadratic character of `a`.  A pseudo-remainder chain on
/// `(u, u′)`: no inversion, the scalar factors enter through their characters.
pub fn disc_character(p: u64, u: &[u64], chi: &[i8], n: &mut Nat) -> i8 {
    let d = u.len() - 1;
    debug_assert!(d >= 2 && u[d] == 1);
    let mut a: Vec<u64> = u.to_vec();
    let mut b: Vec<u64> = (1..=d).map(|i| u[i] * (i as u64 % p) % p).collect();
    n.mul += d as u64;
    vtrim(&mut b);
    // disc(u) = (−1)^{d(d−1)/2} Res(u, u′)
    let mut sign: i8 = if (d * (d - 1) / 2) % 2 == 1 {
        chi[(p - 1) as usize]
    } else {
        1
    };
    loop {
        if b.is_empty() {
            return 0;
        }
        let m = a.len() - 1;
        let nn = b.len() - 1;
        if nn == 0 {
            return if m % 2 == 1 {
                sign * chi[b[0] as usize]
            } else {
                sign
            };
        }
        // pseudo-remainder: lc(b)^δ a = q b + r, δ = m − n + 1 steps.
        let lb = b[nn];
        let delta = m - nn + 1;
        let mut r = a.clone();
        for step in 0..delta {
            let top = m - step;
            let c = r[top];
            for x in r.iter_mut().take(top) {
                *x = *x * lb % p;
            }
            for j in 0..nn {
                let t = c * b[j] % p;
                let x = r[top - nn + j];
                r[top - nn + j] = if x >= t { x - t } else { x + p - t };
            }
            n.mul += (top + nn) as u64;
            n.add += nn as u64;
            r.truncate(top);
        }
        vtrim(&mut r);
        if r.is_empty() {
            return 0;
        }
        let dr = r.len() - 1;
        // Res(a, b) = (−1)^{mn} lc(b)^{m − dr − δn} Res(b, r)
        if (m + dr + delta * nn) % 2 == 1 {
            sign *= chi[lb as usize];
        }
        if (m * nn) % 2 == 1 {
            sign *= chi[(p - 1) as usize];
        }
        a = b;
        b = r;
    }
}

// ── the index-calculus arms ─────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    Full,
    DoubleLp,
}

impl Arm {
    pub fn name(self) -> &'static str {
        match self {
            Arm::Full => "A_full",
            Arm::DoubleLp => "C_double_lp",
        }
    }
    pub fn max_lp(self) -> usize {
        match self {
            Arm::Full => 0,
            Arm::DoubleLp => 2,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct IcParams {
    pub arm: Arm,
    /// Factor-base columns (ignored for [`Arm::Full`]).
    pub nb: usize,
    pub seed: u64,
    pub max_trials: u64,
    /// Walk steps between fresh random starts ([`RESTART`] in the frozen
    /// grid).  A walk in a group of order `r` meets itself after about
    /// `sqrt(pi r / 2)` steps and then repeats; [`restart_sqrt`] stays below.
    pub restart: u64,
    /// The discriminant-parity prefilter (module notes).
    pub prefilter: bool,
    /// Check every accepted relation by group arithmetic (tests only).
    pub verify_relations: bool,
    /// Structured Gaussian elimination between the peel and Wiedemann
    /// ([`SgeParams`]); `None` is the frozen instrument, count for count.
    pub sge: Option<SgeParams>,
}

/// Double-large-prime base size `ceil(c · p^{1 − 1/g})` (Gaudry, Thomé,
/// Thériault, Diem), capped at `p`.
pub fn fb_size_2lp(p: u64, genus: u32, c: f64) -> usize {
    let nb = (c * (p as f64).powf(1.0 - 1.0 / f64::from(genus))).ceil() as usize;
    nb.min(p as usize)
}

#[derive(Clone, Debug, Default)]
pub struct IcResult {
    pub arm: &'static str,
    pub prefilter: bool,
    pub p: u64,
    pub genus: u32,
    pub recovered: Option<u64>,
    pub certified: bool,
    pub exhausted: bool,
    pub group: GroupOps,
    pub setup_ops: u64,
    pub walk_ops: u64,
    pub verify_ops: u64,
    /// `F_p` multiplications and inversions inside the group operations.
    pub cantor_mul: u64,
    pub cantor_inv: u64,
    pub rel: Nat,
    pub la: Nat,
    pub la_graph: Nat,
    pub la_peel: Nat,
    pub la_wied: Nat,
    /// The merge stage ([`SgeParams`]); zero without it.  Included in `la`.
    pub la_sge: Nat,
    /// What the merge did to the core of the last attempt (zero without it).
    pub sge: SgeStats,
    pub fb_cols: usize,
    pub trials: u64,
    /// Trials of full degree that reached the smoothness decision.
    pub candidates: u64,
    /// Candidates the prefilter settled without the `x^p` test.
    pub prefilter_rejects: u64,
    pub splits: u64,
    pub relations: u64,
    pub rels_by_lp: [u64; 3],
    pub bad_relations: u64,
    pub cycles: i64,
    pub core_rows: usize,
    pub core_cols: usize,
    pub attempts: u32,
    /// [`IcExt::coord_square`] solves that returned nothing and fell back.
    pub la_fallbacks: u32,
    /// Candidates the Ferrari test left undecided (baseline test run).
    pub ferrari_undecided: u64,
    pub wall_ns: u64,
}

impl IcResult {
    pub fn group_ops(&self) -> u64 {
        self.group.adds + self.group.doubles
    }
    pub fn native_mul(&self) -> u64 {
        self.rel.mul + self.la.mul
    }
}

/// Walk length between fresh random starts.
pub const RESTART: u64 = 4096;

/// The amendment rule: `min(4096, max(16, floor(sqrt(r) / 2)))`.
pub fn restart_sqrt(r: u64) -> u64 {
    (((r as f64).sqrt() / 2.0) as u64).clamp(16, RESTART)
}

/// Ferrari splitting test for a monic quartic `u = x⁴ + c₃x³ + c₂x² + c₁x + c₀`
/// whose discriminant is already known to be a non-zero square (the parity
/// prefilter passed).  For such a squarefree quartic Frobenius is the
/// identity, a (2,2) or a (3,1) permutation of the roots.
///
/// * The resolvent cubic `R(z) = z³ − c₂z² + (c₁c₃ − 4c₀)z − (c₁² + c₀c₃² − 4c₀c₂)`
///   has the roots `x₁x₂ + x₃x₄`, `x₁x₃ + x₂x₄`, `x₁x₄ + x₂x₃`; (3,1) permutes
///   them cyclically, the identity and (2,2) fix them.  `disc R = disc u`, so
///   the Cardano–Lucas cubic test decides (3,1) (not split) without any
///   polynomial exponentiation.
/// * Otherwise one root `θ` of `R` gives `b₁, b₂` (roots of `z² − θz + c₀`) and
///   `a₁ = (c₁ − c₃b₁)/(b₂ − b₁)`, `a₂ = c₃ − a₁` with
///   `u = (x² + a₁x + b₁)(x² + a₂x + b₂)` when the pairing is rational.  For
///   the identity every `θ` gives a square `θ² − 4c₀`; for (2,2) a non-square
///   proves "not split", and a square leaves the two quadratic discriminants
///   to decide.
///
/// Returns `Some(None)` (not split), `Some(Some(roots))` (split, the four
/// roots) or `None` (undecided: the caller runs the baseline test).  Every
/// native operation is counted in `n`.
pub(crate) fn ferrari_split(
    co: &CubicOracle,
    p: u64,
    u: &[u64],
    n: &mut Nat,
) -> Option<Option<Vec<u64>>> {
    if u.len() != 5 || u[4] != 1 {
        return None;
    }
    let (c0, c1, c2, c3) = (u[0], u[1], u[2], u[3]);
    let four_c0 = pm(p, 4, c0, n);
    let r2 = ps(p, 0, c2, n);
    let r1 = ps(p, pm(p, c1, c3, n), four_c0, n);
    let t = pa(
        p,
        pm(p, c1, c1, n),
        ps(p, pm(p, c0, pm(p, c3, c3, n), n), pm(p, four_c0, c2, n), n),
        n,
    );
    let r0 = ps(p, 0, t, n);
    let res: Vec<u64> = vec![r0, r1, r2, 1];
    let (splits, info) = co.split_cubic(&res, n)?;
    if !splits {
        return Some(None); // (3,1)
    }
    let roots = match info {
        Some((tt, pp)) => co.cubic_roots(&res, tt, pp, n),
        None => None,
    }
    .or_else(|| co.split_cubic_roots_det(&res, n))?;
    let theta = roots[0];
    let d = ps(p, pm(p, theta, theta, n), four_c0, n);
    if d == 0 {
        return None;
    }
    if ppow(p, d, (p - 1) / 2, n) != 1 {
        return Some(None); // (2,2): this pairing is not rational
    }
    let s = co.sqrt(d, n);
    let inv2 = p.div_ceil(2);
    let b1 = pm(p, pa(p, theta, s, n), inv2, n);
    let b2 = pm(p, ps(p, theta, s, n), inv2, n);
    if b1 == b2 {
        return None;
    }
    let den = ps(p, b2, b1, n);
    let a1 = pm(p, ps(p, c1, pm(p, c3, b1, n), n), ppow(p, den, p - 2, n), n);
    let a2 = ps(p, c3, a1, n);
    if pa(p, pm(p, a1, a2, n), pa(p, b1, b2, n), n) != c2 {
        return None;
    }
    let mut out = Vec::with_capacity(4);
    for (a, b) in [(a1, b1), (a2, b2)] {
        let dq = ps(p, pm(p, a, a, n), pm(p, 4, b, n), n);
        if dq == 0 {
            return None;
        }
        if ppow(p, dq, (p - 1) / 2, n) != 1 {
            return Some(None); // an irreducible quadratic factor
        }
        let sq = co.sqrt(dq, n);
        let na = ps(p, 0, a, n);
        out.push(pm(p, pa(p, na, sq, n), inv2, n));
        out.push(pm(p, ps(p, na, sq, n), inv2, n));
    }
    out.sort_unstable();
    Some(Some(out))
}

/// Optional speed-ups ported from the genus-3 positive control
/// (`research/ic_positive_control_{la,capped}_20261116`).  The default is
/// the frozen instrument, count for count.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct IcExt {
    /// Coordinate Wiedemann on a random square row subset
    /// (`ic_control::wiedemann_coord`): only the target's coordinate, no
    /// `AᵀDA`, no reconstruction.  A failed certificate falls back, in the
    /// same attempt and counted, to the baseline [`wiedemann`].  Ignored
    /// when [`IcParams::sge`] is set.
    pub coord_square: bool,
    /// Unit-T walk: starts `a G + T`, jumps `α G`; the unknown is
    /// `h · log T` (column −1) and the segment between restarts is capped at
    /// `max(64, ⌊√r / 16⌋)` so a segment never runs into its rho cycle.
    pub unit_t: bool,
    /// Ferrari splitting test for quartics ([`ferrari_split`]) after the
    /// parity prefilter, at genus 4; undecided cases fall back to the
    /// baseline test.  Needs [`IcParams::prefilter`].
    pub ferrari: bool,
    /// Structured Gaussian elimination ([`sge_reduce`], with the
    /// [`SgeParams`] of [`IcParams::sge`]) on the peeled core, then the
    /// coordinate Wiedemann on a random square row subset of the *reduced*
    /// matrix, the target column mapped through the renumbering.  A failed
    /// certificate falls back, in the same attempt and counted, to the
    /// baseline [`wiedemann`] on the same reduced matrix (the merge is not
    /// repeated).  Needs [`IcParams::sge`]; without it this flag does nothing.
    /// Merged rows carry non-unit entries, which [`ic_control`]'s `mat_vec`
    /// charges one multiplication each per product (a `±1` entry is free).
    pub sge_coord: bool,
}

/// Run one index-calculus arm on an instance.  The structure is
/// `ic_control::run_ic_with`, with the genus a variable.
pub fn run_ic(inst: &Instance, prm: IcParams) -> IcResult {
    run_ic_ext(inst, prm, IcExt::default())
}

/// [`run_ic`] with the speed-ups of [`IcExt`].
pub fn run_ic_ext(inst: &Instance, prm: IcParams, ext: IcExt) -> IcResult {
    let start = std::time::Instant::now();
    let curve = &inst.curve;
    let (p, r, h) = (inst.p, inst.r, inst.h);
    let g = inst.genus as usize;
    assert!(g <= REL_FB);
    let (fm0, fi0) = (curve.fmul(), curve.finv());
    let mut rng = StdRng::seed_from_u64(prm.seed);
    let mut res = IcResult {
        arm: prm.arm.name(),
        prefilter: prm.prefilter,
        p,
        genus: inst.genus,
        ..Default::default()
    };
    let mut ops = GroupOps::default();
    let f = curve.f_coeffs();

    // ── factor base: abscissae with f(x) a non-zero square ──
    // The character table costs one squaring per y in 1..=(p−1)/2.
    let mut chi = vec![-1i8; p as usize];
    chi[0] = 0;
    for y in 1..=(p - 1) / 2 {
        chi[(y * y % p) as usize] = 1;
    }
    res.rel.mul += (p - 1) / 2;
    let want = if prm.arm == Arm::Full {
        usize::MAX
    } else {
        prm.nb
    };
    let mut col = vec![u32::MAX; p as usize];
    let mut ncols = 0usize;
    for x in 0..p {
        if ncols >= want {
            break;
        }
        let mut fx = 0u64;
        for &cf in f.iter().rev() {
            fx = (fx * x + cf) % p;
        }
        res.rel.mul += f.len() as u64;
        res.rel.add += f.len() as u64;
        if chi[fx as usize] == 1 {
            col[x as usize] = ncols as u32;
            ncols += 1;
        }
    }
    res.fb_cols = ncols;
    let tcol = ncols as u32;
    let nunk = ncols + 1;

    // ── walk set-up: 16 jumps α G + β T, short coefficients ──
    let short = r.min(1 << 12);
    let mut jumps: Vec<(JDiv, u64, u64)> = Vec::new();
    for _ in 0..16 {
        let a = rng.gen_range(1..short);
        if ext.unit_t {
            jumps.push((curve.mul(&mut ops, inst.g, a), a, 0));
            continue;
        }
        let b = rng.gen_range(1..short);
        let ag = curve.mul(&mut ops, inst.g, a);
        let bt = curve.mul(&mut ops, inst.target, b);
        jumps.push((curve.add(&mut ops, ag, bt), a, b));
    }
    res.setup_ops = ops.adds + ops.doubles;
    // Cardano–Lucas constants for the Ferrari test, charged to the relation phase.
    let cubic = (ext.ferrari && g == 4 && p > 3).then(|| CubicOracle::new(p, &mut res.rel));
    let restart = if ext.unit_t {
        prm.restart.min((((r as f64).sqrt() / 16.0) as u64).max(64))
    } else {
        prm.restart
    };

    let mut rels: Vec<Rel> = Vec::new();
    let mut vid: HashMap<u32, u32> = HashMap::new();
    let mut dsu = Dsu::new();
    let margin = |n: usize| 8 + n / 100;
    let mut cur = JDiv::IDENTITY;
    let (mut ca, mut cb) = (0u64, 0u64);
    let mut since_restart = u64::MAX;
    let mut next_check = nunk as u64;
    let mut solved = false;

    'outer: while !solved {
        while (rels.len() as u64) < next_check {
            if res.trials >= prm.max_trials {
                res.exhausted = true;
                break 'outer;
            }
            if since_restart >= restart.max(1) && ext.unit_t {
                let a = rng.gen_range(1..r);
                let ag = curve.mul(&mut ops, inst.g, a);
                cur = curve.add(&mut ops, ag, inst.target);
                ca = a;
                cb = 1;
                since_restart = 0;
            } else if since_restart >= restart.max(1) {
                let a = rng.gen_range(1..r);
                let b = rng.gen_range(1..r);
                let ag = curve.mul(&mut ops, inst.g, a);
                let bt = curve.mul(&mut ops, inst.target, b);
                cur = curve.add(&mut ops, ag, bt);
                ca = a;
                cb = b;
                since_restart = 0;
            } else {
                let j = (mix64(curve.key_of(&cur)) >> 24) as usize % 16;
                let (m, aj, bj) = jumps[j];
                cur = curve.add(&mut ops, cur, m);
                ca = addm(ca, aj, r);
                cb = addm(cb, bj, r);
                since_restart += 1;
            }
            res.trials += 1;
            if cur.deg as usize != g {
                continue;
            }
            res.candidates += 1;
            let u = curve.u_coeffs(&cur);
            let mut decided = false;
            if prm.prefilter && g >= 2 {
                let ch = disc_character(p, &u, &chi, &mut res.rel);
                if ch != 1 {
                    res.prefilter_rejects += 1;
                    continue;
                }
                // Two factors of a quadratic with square discriminant.
                decided = g == 2;
            }
            let mut fer_roots = None;
            if let (Some(co), true) = (cubic.as_ref(), prm.prefilter && g == 4) {
                match ferrari_split(co, p, &u, &mut res.rel) {
                    Some(None) => continue,
                    Some(Some(r)) => fer_roots = Some(r),
                    None => res.ferrari_undecided += 1,
                }
            }
            if fer_roots.is_none() && !decided && !splits_completely(p, &u, &mut res.rel) {
                continue;
            }
            res.splits += 1;
            let roots = match fer_roots {
                Some(r) => r,
                None => {
                    let Some(r) = roots_of(p, &u, &mut rng, &mut res.rel) else {
                        continue;
                    };
                    r
                }
            };
            let vc = curve.v_coeffs(&cur);
            let mut rel = Rel {
                fb: [(0, 0); REL_FB],
                nfb: 0,
                lp: [(0, 0); 2],
                nlp: 0,
                tcoef: 0,
                rhs: 0,
            };
            let mut lps: Vec<(u32, i8)> = Vec::new();
            let mut places: Vec<(u64, u64)> = Vec::new();
            for &x in &roots {
                let mut y = 0u64;
                for &cf in vc.iter().rev() {
                    y = (y * x + cf) % p;
                }
                res.rel.mul += vc.len() as u64;
                res.rel.add += vc.len() as u64;
                places.push((x, y));
                if y == 0 {
                    continue; // a 2-torsion place: its log is 0 in the r-part
                }
                let eps: i8 = if y > (p - 1) / 2 { -1 } else { 1 };
                if col[x as usize] != u32::MAX {
                    rel.fb[rel.nfb as usize] = (col[x as usize], eps);
                    rel.nfb += 1;
                } else {
                    lps.push((x as u32, eps));
                }
            }
            if lps.len() > prm.arm.max_lp() {
                continue;
            }
            if rel.nfb == 0 && lps.is_empty() {
                continue;
            }
            rel.rhs = mulmod(h % r, ca, r);
            if ext.unit_t {
                debug_assert_eq!(cb, 1);
                res.rel.mul += 1;
                rel.tcoef = r - 1; // unknown h·log T
            } else {
                res.rel.mul += 2;
                rel.tcoef = subm(0, mulmod(h % r, cb, r), r);
            }
            if prm.verify_relations {
                let mut sum = JDiv::IDENTITY;
                for &(x, y) in &places {
                    let pt = curve.point(x, y).expect("a root of u lies on the curve");
                    sum = curve.cantor(sum, pt);
                }
                if sum != cur {
                    res.bad_relations += 1;
                }
            }
            for (x, e) in lps {
                let id = *vid.entry(x).or_insert_with(|| dsu.add_vertex());
                rel.lp[rel.nlp as usize] = (id, e);
                rel.nlp += 1;
            }
            let (a, _, b, _) = rel.endpoints();
            dsu.edge(a, b);
            res.rels_by_lp[rel.nlp as usize] += 1;
            res.relations += 1;
            rels.push(rel);
        }

        res.cycles = dsu.cycles0();
        let nver = dsu.parent.len();
        if prm.arm != Arm::Full {
            let need = (nunk + margin(nunk)) as i64;
            if res.cycles < need {
                next_check = rels.len() as u64 + (need - res.cycles).max(1) as u64;
                continue;
            }
        } else {
            let mut scratch = Nat::default();
            let rows = build_rows(&rels, nver, tcol, r, &mut scratch);
            let (core, cc, _) = peel(&rows, nunk, tcol, &mut scratch);
            if core.len() < cc + margin(cc) {
                next_check = rels.len() as u64 + (nunk as u64 / 32).max(8);
                continue;
            }
        }

        res.attempts += 1;
        let mut g_nat = Nat::default();
        let rows = build_rows(&rels, nver, tcol, r, &mut g_nat);
        let mut p_nat = Nat::default();
        let (core, cc, ct) = peel(&rows, nunk, tcol, &mut p_nat);
        res.la_graph.merge(g_nat);
        res.la_peel.merge(p_nat);
        res.la.merge(g_nat);
        res.la.merge(p_nat);
        res.core_rows = core.len();
        res.core_cols = cc;
        if core.len() < cc + margin(cc) {
            next_check = rels.len() as u64 + (nunk as u64 / 20).max(8);
            continue;
        }
        let mut w_nat = Nat::default();
        let dense = res.attempts > 1;
        // UnitT solves for h·log T: one inversion of h and one product.
        let h_inv = if ext.unit_t {
            w_nat.mul += pivot_inversion_steps(r) + 1;
            invmod(h % r, r).expect("h is a unit modulo the prime r")
        } else {
            1
        };
        let to_log = |t: u64| if ext.unit_t { mulmod(t, h_inv, r) } else { t };
        // The certificate: [t]G = T by the counted multiplication (its group
        // operations are verify_ops) and by an independent one whose field
        // work is taken back out of the tally.
        // (Assigned, not accumulated, exactly as the frozen instrument does.)
        let certify = |t: u64, ops: &mut GroupOps, verify_ops: &mut u64| -> bool {
            let v0 = ops.adds + ops.doubles;
            let chk = curve.mul(ops, inst.g, t);
            *verify_ops = ops.adds + ops.doubles - v0;
            let (cm, ci) = (curve.fmul(), curve.finv());
            let ok = chk == inst.target && curve.mul_rtl(inst.g, t) == inst.target;
            curve.k.mul.set(cm);
            curve.k.inv.set(ci);
            ok
        };
        let mut answer = None;
        // The reduced core of `sge_coord`, kept for the certified fallback.
        let mut reduced: Option<(Vec<Row>, usize, u32)> = None;
        if let (true, Some(sp)) = (ext.sge_coord, prm.sge) {
            let mut s_nat = Nat::default();
            let (red, rcc, rct, st) = sge_reduce(&core, cc, ct, r, sp, &mut s_nat);
            res.la_sge.merge(s_nat);
            res.la.merge(s_nat);
            res.sge = st;
            let t = wiedemann_coord(&red, rcc, rct as usize, r, &mut rng, true, &mut w_nat);
            match t.map(to_log) {
                Some(t) if certify(t, &mut ops, &mut res.verify_ops) => answer = Some(t),
                _ => res.la_fallbacks += 1,
            }
            reduced = Some((red, rcc, rct));
        } else if ext.coord_square && prm.sge.is_none() {
            let t = wiedemann_coord(&core, cc, ct as usize, r, &mut rng, true, &mut w_nat);
            match t.map(to_log) {
                Some(t) if certify(t, &mut ops, &mut res.verify_ops) => answer = Some(t),
                _ => res.la_fallbacks += 1,
            }
        }
        if answer.is_none() {
            let sol = match prm.sge {
                _ if reduced.is_some() => {
                    let (red, rcc, rct) = reduced.as_ref().expect("checked");
                    wiedemann(red, *rcc, r, &mut rng, true, &mut w_nat).map(|x| x[*rct as usize])
                }
                None => wiedemann(
                    &core,
                    cc,
                    r,
                    &mut rng,
                    dense || ext.coord_square,
                    &mut w_nat,
                )
                .map(|x| x[ct as usize]),
                Some(sp) => {
                    let mut s_nat = Nat::default();
                    let (red, rcc, rct, st) = sge_reduce(&core, cc, ct, r, sp, &mut s_nat);
                    res.la_sge.merge(s_nat);
                    res.la.merge(s_nat);
                    res.sge = st;
                    // The merged system is solved with a copy of the stream; the
                    // stream itself advances as the unmerged solve would, so a
                    // failed attempt resumes the relation phase identically.
                    let mut wrng = rng.clone();
                    skip_wiedemann_draws(&mut rng, core.len(), cc, r, dense);
                    wiedemann(&red, rcc, r, &mut wrng, dense, &mut w_nat).map(|x| x[rct as usize])
                }
            };
            if let Some(t) = sol.map(to_log) {
                if certify(t, &mut ops, &mut res.verify_ops) {
                    answer = Some(t);
                }
            }
        }
        res.la_wied.merge(w_nat);
        res.la.merge(w_nat);
        if let Some(t) = answer {
            res.recovered = Some(t);
            res.certified = true;
            solved = true;
            continue;
        }
        if answer.is_none() {
            let sol = match prm.sge {
                None => wiedemann(
                    &core,
                    cc,
                    r,
                    &mut rng,
                    dense || ext.coord_square,
                    &mut w_nat,
                )
                .map(|x| x[ct as usize]),
                Some(sp) => {
                    let mut s_nat = Nat::default();
                    let (red, rcc, rct, st) = sge_reduce(&core, cc, ct, r, sp, &mut s_nat);
                    res.la_sge.merge(s_nat);
                    res.la.merge(s_nat);
                    res.sge = st;
                    // The merged system is solved with a copy of the stream; the
                    // stream itself advances as the unmerged solve would, so a
                    // failed attempt resumes the relation phase identically.
                    let mut wrng = rng.clone();
                    skip_wiedemann_draws(&mut rng, core.len(), cc, r, dense);
                    wiedemann(&red, rcc, r, &mut wrng, dense, &mut w_nat).map(|x| x[rct as usize])
                }
            };
            if let Some(t) = sol.map(to_log) {
                if certify(t, &mut ops, &mut res.verify_ops) {
                    answer = Some(t);
                }
            }
        }
        res.la_wied.merge(w_nat);
        res.la.merge(w_nat);
        if let Some(t) = answer {
            res.recovered = Some(t);
            res.certified = true;
            solved = true;
            continue;
        }
        next_check = rels.len() as u64 + (nunk as u64 / 20).max(16);
    }
    res.la.add += dsu.ops;
    res.group = ops;
    res.walk_ops = (ops.adds + ops.doubles) - res.setup_ops - res.verify_ops;
    res.cantor_mul = curve.fmul() - fm0;
    res.cantor_inv = curve.finv() - fi0;
    if prm.verify_relations {
        // relation re-checks are not solver work either; they only run in tests.
        res.cantor_mul = 0;
        res.cantor_inv = 0;
    }
    res.wall_ns = start.elapsed().as_nanos() as u64;
    res
}

// ── structured Gaussian elimination before Wiedemann ────────────────────

/// Structured Gaussian elimination (filtering, as in NFS linear algebra)
/// between the peel and Wiedemann: columns of weight `2, 3, …, w_max` are
/// merged away while the non-zeros stay within `fill_max` times the peeled
/// core's.  See [`sge_reduce`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SgeParams {
    pub w_max: usize,
    pub fill_max: f64,
}

/// What the merge did to the last core it reduced.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SgeStats {
    pub rows_in: usize,
    pub cols_in: usize,
    pub nnz_in: usize,
    /// Entries that are not `±1` (each costs a multiplication per matrix
    /// product in Wiedemann; a `±1` entry costs an addition).
    pub generic_in: usize,
    pub rows_out: usize,
    pub cols_out: usize,
    pub nnz_out: usize,
    pub generic_out: usize,
    /// Columns eliminated by a merge (one pivot row each).
    pub merged: usize,
    /// Columns of weight one removed with their row (no arithmetic).
    pub peeled: usize,
    /// Rows that became empty, and duplicate rows, removed.
    pub empty: usize,
    pub duplicates: usize,
    /// Merges refused because the fill bound would have been passed.
    pub refused: usize,
}

#[inline]
fn is_unit(v: u64, r: u64) -> bool {
    v == 1 || v == r - 1
}

fn coef_at(t: &[(u32, u64)], c: u32) -> Option<u64> {
    t.binary_search_by_key(&c, |x| x.0).ok().map(|i| t[i].1)
}

/// Column state of [`sge_reduce`]: weights, the rows that may hold each
/// column (stale ids are filtered on use), and the queue by weight.
struct SgeCols {
    w: Vec<usize>,
    rows: Vec<Vec<u32>>,
    heap: std::collections::BinaryHeap<std::cmp::Reverse<(usize, u32)>>,
    tcol: u32,
    w_max: usize,
}

impl SgeCols {
    /// Row `i` changes from `old` to `new`: weights out and in, holders and
    /// queue updated; one addition of bookkeeping per entry visited.
    fn reweigh(&mut self, old: &[(u32, u64)], new: &[(u32, u64)], i: u32, nat: &mut Nat) {
        let (mut a, mut b) = (0, 0);
        while a < old.len() || b < new.len() {
            nat.add += 1;
            let c = if b == new.len() || (a < old.len() && old[a].0 < new[b].0) {
                let c = old[a].0;
                self.w[c as usize] -= 1;
                a += 1;
                c
            } else if a == old.len() || new[b].0 < old[a].0 {
                let c = new[b].0;
                self.w[c as usize] += 1;
                self.rows[c as usize].push(i);
                b += 1;
                c
            } else {
                a += 1;
                b += 1;
                continue;
            };
            if c != self.tcol && self.w[c as usize] <= self.w_max {
                self.heap.push(std::cmp::Reverse((self.w[c as usize], c)));
            }
        }
    }
}

/// Structured Gaussian elimination of `rows` (columns `0..ncols`, the
/// unknown `t` in column `tcol`, never eliminated).
///
/// Repeatedly takes the live column of least weight `w ≤ w_max` (ties: the
/// lower index).  Weight 0: dropped.  Weight 1: the column and its row are
/// removed (a peel; no arithmetic).  Weight `w ≥ 2`: the shortest row holding
/// the column (ties: a `±1` coefficient first) is the pivot; every other row
/// holding the column becomes `row − (a_i / a_p)·pivot`, and the pivot row
/// and the column leave the system.  A merge is refused when its worst-case
/// fill (no cancellation: `(w − 1)(len_p − 2) − len_p` new non-zeros) would
/// take the non-zeros past `fill_max` times the start.  Afterwards empty and
/// duplicate rows are dropped and the columns renumbered densely, `t` last.
///
/// Every eliminated variable is determined by its pivot row from variables
/// that stay or were eliminated later, so a full-column-rank system stays of
/// full column rank and `t` keeps its value; the pivot rows are not needed
/// because the solver wants `t` only (no back substitution).
///
/// Counting, in the convention of `ic_control` (`combine`, `mat_vec`): a
/// product with a `±1` factor is free, any other modular product is one
/// multiplication; an inversion of a pivot coefficient other than `±1` costs
/// `pivot_inversion_steps(r)` multiplications; `combine` charges one
/// addition per entry of both operands; the right-hand side one
/// multiplication (unless the multiplier is `±1`) and one addition per
/// merged row; the index bookkeeping one addition per entry touched, as
/// `peel` does.
pub(crate) fn sge_reduce(
    rows: &[Row],
    ncols: usize,
    tcol: u32,
    r: u64,
    prm: SgeParams,
    nat: &mut Nat,
) -> (Vec<Row>, usize, u32, SgeStats) {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut st = SgeStats {
        rows_in: rows.len(),
        cols_in: ncols,
        ..Default::default()
    };
    let mut rws: Vec<Row> = rows.to_vec();
    let mut alive = vec![true; rws.len()];
    let mut cs = SgeCols {
        w: vec![0usize; ncols],
        rows: vec![Vec::new(); ncols],
        heap: BinaryHeap::new(),
        tcol,
        w_max: prm.w_max,
    };
    let mut nnz = 0usize;
    for (i, (t, _)) in rws.iter().enumerate() {
        for &(c, v) in t {
            cs.rows[c as usize].push(i as u32);
            cs.w[c as usize] += 1;
            st.generic_in += usize::from(!is_unit(v, r));
            nat.add += 1;
        }
        nnz += t.len();
    }
    st.nnz_in = nnz;
    let limit = (prm.fill_max * nnz as f64).floor() as usize;
    let mut dead = vec![false; ncols];
    for c in 0..ncols as u32 {
        if c != tcol && cs.w[c as usize] <= prm.w_max {
            cs.heap.push(Reverse((cs.w[c as usize], c)));
        }
    }

    while let Some(Reverse((wc, c))) = cs.heap.pop() {
        let cu = c as usize;
        if dead[cu] || cs.w[cu] != wc {
            continue;
        }
        if wc == 0 {
            dead[cu] = true;
            continue;
        }
        // live rows holding c
        let mut holders: Vec<u32> = Vec::with_capacity(wc);
        cs.rows[cu].retain(|&i| alive[i as usize] && coef_at(&rws[i as usize].0, c).is_some());
        cs.rows[cu].sort_unstable();
        cs.rows[cu].dedup();
        holders.extend_from_slice(&cs.rows[cu]);
        nat.add += holders.len() as u64;
        debug_assert_eq!(holders.len(), wc, "column weight bookkeeping");
        if wc == 1 {
            let i = holders[0];
            let old = std::mem::take(&mut rws[i as usize].0);
            alive[i as usize] = false;
            nnz -= old.len();
            cs.reweigh(&old, &[], i, nat);
            dead[cu] = true;
            st.peeled += 1;
            continue;
        }
        let &piv = holders
            .iter()
            .min_by_key(|&&i| {
                let t = &rws[i as usize].0;
                let a = coef_at(t, c).expect("holder");
                (t.len(), !is_unit(a, r), i)
            })
            .expect("weight at least 2");
        let lp = rws[piv as usize].0.len();
        let grow = (wc - 1) as isize * (lp as isize - 2) - lp as isize;
        if nnz as isize + grow > limit as isize {
            st.refused += 1;
            continue;
        }
        let (pt, prhs) = {
            let (t, b) = &rws[piv as usize];
            (t.clone(), *b)
        };
        let ap = coef_at(&pt, c).expect("pivot holds c");
        // ip = 1 / a_p
        let ip = if is_unit(ap, r) {
            ap
        } else {
            nat.mul += pivot_inversion_steps(r);
            invmod(ap, r).expect("r is prime")
        };
        for &i in &holders {
            if i == piv {
                continue;
            }
            let (old, rhs) = {
                let (t, b) = &rws[i as usize];
                (t.clone(), *b)
            };
            let ai = coef_at(&old, c).expect("holder");
            // m = a_i / a_p
            let m = if ap == 1 {
                ai
            } else if ap == r - 1 {
                subm(0, ai, r)
            } else if is_unit(ai, r) {
                if ai == 1 {
                    ip
                } else {
                    subm(0, ip, r)
                }
            } else {
                nat.mul += 1;
                mulmod(ai, ip, r)
            };
            let (new, nrhs) = if m == 1 {
                nat.add += 1;
                (combine(&old, false, &pt, true, r, nat), subm(rhs, prhs, r))
            } else if m == r - 1 {
                nat.add += 1;
                (combine(&old, false, &pt, false, r, nat), addm(rhs, prhs, r))
            } else {
                let scaled: Vec<(u32, u64)> = pt
                    .iter()
                    .map(|&(cc, v)| {
                        let s = if v == 1 {
                            m
                        } else if v == r - 1 {
                            subm(0, m, r)
                        } else {
                            nat.mul += 1;
                            mulmod(v, m, r)
                        };
                        (cc, s)
                    })
                    .collect();
                nat.mul += 1;
                nat.add += 1;
                (
                    combine(&old, false, &scaled, true, r, nat),
                    subm(rhs, mulmod(m, prhs, r), r),
                )
            };
            debug_assert!(coef_at(&new, c).is_none(), "the pivot column cancels");
            nnz = nnz + new.len() - old.len();
            cs.reweigh(&old, &new, i, nat);
            if new.is_empty() {
                alive[i as usize] = false;
                st.empty += 1;
            }
            rws[i as usize] = (new, nrhs);
        }
        alive[piv as usize] = false;
        nnz -= lp;
        let old = std::mem::take(&mut rws[piv as usize].0);
        cs.reweigh(&old, &[], piv, nat);
        dead[cu] = true;
        st.merged += 1;
    }

    // duplicates (same entries and right-hand side), first occurrence kept
    let mut seen: std::collections::HashSet<Row> = std::collections::HashSet::new();
    let mut kept: Vec<Row> = Vec::new();
    for (i, row) in rws.into_iter().enumerate() {
        if !alive[i] {
            continue;
        }
        nat.add += row.0.len() as u64;
        if !seen.insert(row.clone()) {
            st.duplicates += 1;
            continue;
        }
        kept.push(row);
    }
    // dense renumbering, t last
    let mut wt = vec![0usize; ncols];
    for (t, _) in &kept {
        for &(c, _) in t {
            wt[c as usize] += 1;
        }
    }
    let mut map = vec![u32::MAX; ncols];
    let mut n = 0u32;
    for c in 0..ncols {
        if c as u32 != tcol && wt[c] > 0 {
            map[c] = n;
            n += 1;
        }
    }
    map[tcol as usize] = n;
    let out: Vec<Row> = kept
        .into_iter()
        .map(|(t, b)| {
            let mut nt: Vec<(u32, u64)> = t.iter().map(|&(c, v)| (map[c as usize], v)).collect();
            nt.sort_unstable_by_key(|x| x.0);
            (nt, b)
        })
        .collect();
    st.rows_out = out.len();
    st.cols_out = n as usize + 1;
    st.nnz_out = out.iter().map(|(t, _)| t.len()).sum();
    st.generic_out = out
        .iter()
        .flat_map(|(t, _)| t.iter())
        .filter(|&&(_, v)| !is_unit(v, r))
        .count();
    (out, n as usize + 1, n, st)
}

/// Advance `rng` exactly as `wiedemann` on an `nr × ncols` system would, so
/// that the relation phase after a failed attempt draws the same stream with
/// and without the merge (the merged system is solved with a copy).
fn skip_wiedemann_draws(rng: &mut StdRng, nr: usize, ncols: usize, r: u64, dense_u: bool) {
    for _ in 0..nr {
        let _: u64 = rng.gen_range(1..r);
    }
    if dense_u || ncols <= 16 {
        for _ in 0..ncols {
            let _: u64 = rng.gen_range(1..r);
        }
    } else {
        for _ in 0..16 {
            let _: usize = rng.gen_range(0..ncols);
            let _: u64 = rng.gen_range(1..r);
        }
    }
}

// ── rho ─────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Default)]
pub struct RhoOut {
    pub p: u64,
    pub genus: u32,
    pub recovered: Option<u64>,
    pub certified: bool,
    pub group: GroupOps,
    pub walk_ops: u64,
    pub cantor_mul: u64,
    pub cantor_inv: u64,
    /// Canonicalisations (the walk's per-step native work).
    pub native: u64,
    /// The walk's other counted native work (`RhoResult::work_native`: the
    /// short-cycle window's and Brent's compares), summed.  Not in `native`.
    pub compares: u64,
    pub steps: u64,
    pub wall_ns: u64,
}

impl RhoOut {
    pub fn group_ops(&self) -> u64 {
        self.group.adds + self.group.doubles
    }
}

/// Negation-map rho on the order-`r` subgroup (`rho_reference_walk`,
/// `RhoWalk::negation()`), certified by an independent `[k]G`.
pub fn run_rho(inst: &Instance, seed: u64) -> RhoOut {
    run_rho_walk(inst, seed, RhoWalk::negation())
}

/// [`run_rho`] with the walk chosen (`RhoWalk::plain()` for the walk on
/// points).  `native` is the walk's canonicalisations, floored at one per
/// step, as in [`run_rho`].
pub fn run_rho_walk(inst: &Instance, seed: u64, walk: RhoWalk) -> RhoOut {
    run_rho_policy(inst, seed, walk, WalkPolicy::legacy())
}

/// [`run_rho_walk`] under a [`WalkPolicy`]. `WalkPolicy::legacy()` is
/// `run_rho_walk` exactly (`rho_reference_walk` is that policy). Used for the
/// leaner references: negation with `cycle_window = 0` and Brent detection
/// (arm B), and the plain walk with `cycle_window = 0` (arm Q).
pub fn run_rho_policy(inst: &Instance, seed: u64, walk: RhoWalk, policy: WalkPolicy) -> RhoOut {
    let (fm0, fi0) = (inst.curve.fmul(), inst.curve.finv());
    let max_steps = (60.0 * (inst.r as f64).sqrt()) as u64 + 10_000;
    let rr = if walk.negation {
        rho_walk_with_policy(
            &inst.curve,
            &NegationClasses { r: inst.r },
            inst.g,
            inst.target,
            inst.r,
            seed,
            max_steps,
            walk,
            policy,
        )
    } else {
        rho_walk_with_policy(
            &inst.curve,
            &PointClasses,
            inst.g,
            inst.target,
            inst.r,
            seed,
            max_steps,
            walk,
            policy,
        )
    };
    let (cm, ci) = (inst.curve.fmul() - fm0, inst.curve.finv() - fi0);
    let certified = rr
        .recovered
        .map(|d| inst.curve.mul_rtl(inst.g, d) == inst.target)
        .unwrap_or(false);
    RhoOut {
        p: inst.p,
        genus: inst.genus,
        recovered: rr.recovered,
        certified,
        group: rr.group_ops,
        walk_ops: rr.counters.get("walk_operations").copied().unwrap_or(0),
        cantor_mul: cm,
        cantor_inv: ci,
        native: rr
            .counters
            .get("canonicalisations_uncharged")
            .copied()
            .unwrap_or(0)
            .max(rr.steps),
        compares: rr.work_native.values().sum(),
        steps: rr.steps,
        wall_ns: rr.wall_ns,
    }
}

#[cfg(test)]
mod tests {

    /// The default extension is the frozen instrument: every count equal.
    #[test]
    fn ext_default_is_run_ic_count_for_count() {
        for (p, g) in [(131u64, 3u32), (131, 4), (257, 3)] {
            let inst = first_instance(p, g);
            let prm = IcParams {
                arm: Arm::DoubleLp,
                nb: fb_size_2lp(p, g, 2.0),
                seed: 5,
                max_trials: 5_000_000,
                restart: restart_sqrt(inst.r),
                prefilter: true,
                verify_relations: false,
                sge: None,
            };
            let a = run_ic(&inst, prm);
            let b = run_ic_ext(&inst, prm, IcExt::default());
            assert_eq!(
                (
                    a.trials,
                    a.relations,
                    a.group_ops(),
                    a.rel,
                    a.la,
                    a.verify_ops,
                    a.recovered
                ),
                (
                    b.trials,
                    b.relations,
                    b.group_ops(),
                    b.rel,
                    b.la,
                    b.verify_ops,
                    b.recovered
                )
            );
        }
    }

    /// Coordinate Wiedemann and the unit-T walk certify at genus 3 and 4, alone
    /// and together, and the coordinate solve spends fewer LA multiplications.
    #[test]
    fn ext_arms_certify_at_genus_3_and_4() {
        for (p, g) in [(131u64, 3u32), (131, 4), (257, 4)] {
            let inst = first_instance(p, g);
            let prm = IcParams {
                arm: Arm::DoubleLp,
                nb: fb_size_2lp(p, g, 2.0),
                seed: 9,
                max_trials: 5_000_000,
                restart: restart_sqrt(inst.r),
                prefilter: true,
                verify_relations: true,
                sge: None,
            };
            let base = run_ic(&inst, prm);
            for ext in [
                IcExt {
                    coord_square: true,
                    unit_t: false,
                    ferrari: false,
                    sge_coord: false,
                },
                IcExt {
                    coord_square: false,
                    unit_t: true,
                    ferrari: false,
                    sge_coord: false,
                },
                IcExt {
                    coord_square: true,
                    unit_t: true,
                    ferrari: false,
                    sge_coord: false,
                },
            ] {
                let x = run_ic_ext(&inst, prm, ext);
                assert!(x.certified, "p={p} g={g} {ext:?}");
                assert_eq!(x.bad_relations, 0);
                assert_eq!(x.recovered.unwrap() % inst.r, inst.t_secret % inst.r);
                if ext
                    == (IcExt {
                        coord_square: true,
                        unit_t: false,
                        ferrari: false,
                        sge_coord: false,
                    })
                {
                    assert_eq!(x.relations, base.relations);
                    if x.la_fallbacks == 0 {
                        assert!(x.la_wied.mul < base.la_wied.mul, "p={p} g={g}");
                    }
                }
            }
        }
    }
    use super::*;
    use crate::cryptanalysis::chained_transfer::hyper64::H64Curve;

    /// For odd-degree `f`, a reducible `f` gives an even `#J` (2-torsion), so a
    /// prime-order search may skip it; an irreducible `f` gives an odd `#J`.
    #[test]
    fn reducible_f_gives_even_jacobian_order() {
        let (mut red, mut irr) = (0, 0);
        for &(p, g) in &[(131u64, 3u32), (131, 4), (257, 4)] {
            for cs in 1..120u64 {
                let f = random_f(p, g, cs);
                let Ok(curve) = JCurve::new(p, g, &f) else {
                    continue;
                };
                let Ok(order) = curve.jacobian_order(cs) else {
                    continue;
                };
                if f_irreducible(p, &f) {
                    irr += 1;
                    assert_eq!(order % 2, 1, "p={p} g={g} cs={cs}: irreducible f, even #J");
                } else {
                    red += 1;
                    assert_eq!(order % 2, 0, "p={p} g={g} cs={cs}: reducible f, odd #J");
                }
            }
        }
        assert!(red > 50 && irr > 10, "red {red} irr {irr}");
    }

    /// Every monic quartic with a non-zero square discriminant over small
    /// primes (both residue classes of p mod 3, with and without a
    /// single-exponent cube root): a decided Ferrari answer equals brute
    /// force, roots included, and most candidates are decided.
    #[test]
    fn ferrari_matches_brute_force_on_every_square_disc_quartic() {
        let mut rates = Vec::new();
        for p in [5u64, 7, 11, 13, 17, 19, 23] {
            let mut chi = vec![-1i8; p as usize];
            chi[0] = 0;
            for y in 1..=(p - 1) / 2 {
                chi[(y * y % p) as usize] = 1;
            }
            let mut n = Nat::default();
            let co = CubicOracle::new(p, &mut n);
            let (mut decided, mut total) = (0u64, 0u64);
            for c0 in 0..p {
                for c1 in 0..p {
                    for c2 in 0..p {
                        for c3 in 0..p {
                            let u = vec![c0, c1, c2, c3, 1];
                            if disc_character(p, &u, &chi, &mut n) != 1 {
                                continue;
                            }
                            total += 1;
                            let truth: Vec<u64> = (0..p)
                                .filter(|&x| {
                                    (((((x + c3) % p * x + c2) % p) * x + c1) % p * x + c0) % p == 0
                                })
                                .collect();
                            match ferrari_split(&co, p, &u, &mut n) {
                                None => {}
                                Some(None) => {
                                    decided += 1;
                                    assert_ne!(truth.len(), 4, "p={p} u={u:?} split but rejected");
                                }
                                Some(Some(r)) => {
                                    decided += 1;
                                    assert_eq!(r, truth, "p={p} u={u:?}");
                                }
                            }
                        }
                    }
                }
            }
            eprintln!("ferrari p={p}: decided {decided} of {total}");
            rates.push((p, decided, total));
        }
        // Correctness is asserted above for every prime; the fast path must
        // decide most candidates once p is not tiny (degenerate quartics, with
        // P' = 0 or b1 = b2, fall back to the exact baseline by design).
        for (p, decided, total) in rates {
            assert!(
                p < 11 || decided * 2 >= total,
                "p={p}: decided only {decided} of {total}"
            );
        }
    }

    /// `sge_coord` (merge, then the coordinate solve on a square subset of the
    /// reduced matrix) certifies at genus 3 and 4, reproduces the relation
    /// phase of arm w, and where the merge shrank the matrix and neither run
    /// fell back it spends fewer LA multiplications than arm w.
    #[test]
    fn sge_coord_certifies_and_cuts_la() {
        let w = IcExt {
            coord_square: true,
            unit_t: true,
            ferrari: false,
            sge_coord: false,
        };
        let s = IcExt {
            sge_coord: true,
            ..w
        };
        let mut shrank = 0;
        for (p, g) in [(131u64, 3u32), (257, 3), (131, 4), (257, 4)] {
            let inst = first_instance(p, g);
            for sp in [
                SgeParams {
                    w_max: 4,
                    fill_max: 1.5,
                },
                SgeParams {
                    w_max: 8,
                    fill_max: 3.0,
                },
            ] {
                let mut prm = prm(p, g, Arm::DoubleLp, true, None);
                prm.verify_relations = true;
                prm.restart = restart_sqrt(inst.r);
                let a = run_ic_ext(&inst, prm, w);
                prm.sge = Some(sp);
                let b = run_ic_ext(&inst, prm, s);
                let tag = format!("p={p} g={g} {sp:?}");
                assert!(b.certified && b.bad_relations == 0, "{tag}");
                assert_eq!(b.recovered.unwrap() % inst.r, inst.t_secret % inst.r);
                assert_eq!(
                    b.la.mul,
                    b.la_graph.mul + b.la_peel.mul + b.la_sge.mul + b.la_wied.mul
                );
                if b.attempts == a.attempts && b.relations == a.relations {
                    assert_eq!((b.sge.rows_in, b.sge.cols_in), (b.core_rows, b.core_cols));
                    eprintln!(
                        "sge_coord {tag}: cols {}->{} nnz {}->{} generic {}->{} fb {}/{} \
                         la_wied mul {} add {} (w: {} {}) la mul {} (w {})",
                        b.sge.cols_in,
                        b.sge.cols_out,
                        b.sge.nnz_in,
                        b.sge.nnz_out,
                        b.sge.generic_in,
                        b.sge.generic_out,
                        b.la_fallbacks,
                        a.la_fallbacks,
                        b.la_wied.mul,
                        b.la_wied.add,
                        a.la_wied.mul,
                        a.la_wied.add,
                        b.la.mul,
                        a.la.mul
                    );
                    if b.sge.cols_out * 10 <= b.sge.cols_in * 8
                        && b.la_fallbacks == 0
                        && a.la_fallbacks == 0
                    {
                        shrank += 1;
                        assert!(b.la_wied.mul < a.la_wied.mul, "{tag}");
                    }
                }
            }
        }
        eprintln!("sge_coord: matrices shrunk by 20% or more in {shrank} runs");
    }

    /// The Ferrari arm certifies at genus 4 and spends fewer relation-phase
    /// multiplications per trial than the same arm without it.
    #[test]
    fn ferrari_arm_certifies_and_cuts_relation_work() {
        for p in [131u64, 257] {
            let inst = first_instance(p, 4);
            let prm = IcParams {
                arm: Arm::DoubleLp,
                nb: fb_size_2lp(p, 4, 2.0),
                seed: 9,
                max_trials: 5_000_000,
                restart: restart_sqrt(inst.r),
                prefilter: true,
                verify_relations: true,
                sge: None,
            };
            let base = IcExt {
                coord_square: true,
                unit_t: true,
                ferrari: false,
                sge_coord: false,
            };
            let fer = IcExt {
                ferrari: true,
                ..base
            };
            let a = run_ic_ext(&inst, prm, base);
            let b = run_ic_ext(&inst, prm, fer);
            assert!(b.certified && b.bad_relations == 0, "p={p}");
            assert_eq!(b.recovered.unwrap() % inst.r, inst.t_secret % inst.r);
            let per = |x: &IcResult| x.rel.mul as f64 / x.trials as f64;
            assert!(per(&b) < per(&a), "p={p}: {} vs {}", per(&b), per(&a));
        }
    }

    fn first_instance(p: u64, g: u32) -> Instance {
        for cs in 1..400u64 {
            if let Ok(i) = make_instance(p, g, cs, 7, 64) {
                return i;
            }
        }
        panic!("no instance at p = {p}, g = {g}");
    }

    /// The group law agrees with `H64Curve` divisor for divisor (g ≤ 3).
    #[test]
    fn cantor_matches_hyper64() {
        for (p, g) in [(131u64, 2u32), (131, 3), (1031, 3)] {
            let f = (1..50)
                .map(|s| random_f(p, g, s))
                .find(|f| JCurve::new(p, g, f).is_ok())
                .unwrap();
            let c = JCurve::new(p, g, &f).unwrap();
            let h = H64Curve::new(p, g, &f).unwrap();
            let mut st = 11u64;
            let mut pair = || {
                let (mut a, mut ha) = (
                    JDiv::IDENTITY,
                    h.cantor(Default::default(), Default::default()),
                );
                let mut placed = 0;
                while placed < g {
                    st = st.wrapping_add(0x9E37_79B9_7F4A_7C15);
                    let x = mix64(st) % p;
                    let Some(y) = sqrt_mod(c.k.eval(&c.f, x), p) else {
                        continue;
                    };
                    a = c.cantor(a, c.point(x, y).unwrap());
                    ha = h.cantor(ha, h.point(x, y).unwrap());
                    placed += 1;
                }
                (a, ha)
            };
            let (mut a, mut ha) = pair();
            for i in 0..200 {
                assert_eq!(c.u_coeffs(&a), ha.u_coeffs(), "u at step {i}");
                let mut hv = ha.v_coeffs();
                hv.resize(a.deg as usize, 0);
                assert_eq!(c.v_coeffs(&a), hv, "v at step {i}");
                assert!(c.is_valid(&a));
                let (b, hb) = pair();
                a = if i % 3 == 0 {
                    c.cantor(a, a)
                } else {
                    c.cantor(a, b)
                };
                ha = if i % 3 == 0 {
                    h.cantor(ha, ha)
                } else {
                    h.cantor(ha, hb)
                };
            }
        }
    }

    /// Group axioms at every genus, including sums with common support.
    #[test]
    fn group_axioms_all_genera() {
        for g in 2..=5u32 {
            let p = 67;
            let f = (1..50)
                .map(|s| random_f(p, g, s))
                .find(|f| JCurve::new(p, g, f).is_ok())
                .unwrap();
            let c = JCurve::new(p, g, &f).unwrap();
            let mut st = 5u64;
            for _ in 0..60 {
                let a = c.random_divisor(&mut st);
                let b = c.random_divisor(&mut st);
                let d = c.random_divisor(&mut st);
                let ab = c.cantor(a, b);
                assert!(c.is_valid(&ab));
                assert_eq!(ab, c.cantor(b, a));
                assert_eq!(c.cantor(ab, d), c.cantor(a, c.cantor(b, d)));
                assert!(c.cantor(a, c.negate(a)).is_identity());
                let a2 = c.cantor(a, a);
                assert_eq!(c.cantor(a2, a), c.mul_ltr(a, 3));
                assert_eq!(c.mul_ltr(a, 1000), c.mul_rtl(a, 1000));
                // common support: (a + b) + (a − b) = 2a
                assert_eq!(c.cantor(ab, c.cantor(a, c.negate(b))), a2);
            }
        }
    }

    // An independent `#J`: `N_1 … N_g` by brute force over `F_{p^k}` and the
    // Newton identities.  Test support only.
    fn ext_mul(a: &[u64], b: &[u64], m: &[u64], p: u64) -> Vec<u64> {
        let k = m.len() - 1;
        let mut t = vec![0u64; 2 * k - 1];
        for i in 0..k {
            for j in 0..k {
                t[i + j] = (t[i + j] + a[i] * b[j]) % p;
            }
        }
        for i in (k..2 * k - 1).rev() {
            let c = t[i];
            for j in 0..k {
                t[i - k + j] = (t[i - k + j] + (p - m[j]) * c) % p;
            }
        }
        t.truncate(k);
        t
    }
    fn ext_pow(a: &[u64], mut e: u64, m: &[u64], p: u64) -> Vec<u64> {
        let k = m.len() - 1;
        let mut r = vec![0u64; k];
        r[0] = 1;
        let mut b = a.to_vec();
        while e > 0 {
            if e & 1 == 1 {
                r = ext_mul(&r, &b, m, p);
            }
            b = ext_mul(&b, &b, m, p);
            e >>= 1;
        }
        r
    }
    fn irreducible(p: u64, k: usize) -> Vec<u64> {
        // Trial division by every monic polynomial of degree 1..=k/2.
        let mut nat = Nat::default();
        let mut st = 99u64;
        loop {
            st = st.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut m: Vec<u64> = (0..k).map(|i| mix64(st ^ (i as u64 * 77)) % p).collect();
            m.push(1);
            let mut reducible = false;
            for dd in 1..=k / 2 {
                for idx in 0..p.pow(dd as u32) {
                    let mut d: Vec<u64> = Vec::new();
                    let mut t = idx;
                    for _ in 0..dd {
                        d.push(t % p);
                        t /= p;
                    }
                    d.push(1);
                    if vrem(p, &m, &d, &mut nat).is_empty() {
                        reducible = true;
                    }
                }
            }
            if !reducible {
                return m;
            }
        }
    }
    fn brute_order(p: u64, g: usize, f: &[u64]) -> i128 {
        let mut s = Vec::new();
        for k in 1..=g {
            let m = irreducible(p, k);
            let q = (0..k).fold(1u64, |a, _| a * p);
            let mut n = 1i128;
            for idx in 0..q {
                let mut x = vec![0u64; k];
                let mut t = idx;
                for c in x.iter_mut() {
                    *c = t % p;
                    t /= p;
                }
                let mut acc = vec![0u64; k];
                for &cf in f.iter().rev() {
                    acc = ext_mul(&acc, &x, &m, p);
                    acc[0] = (acc[0] + cf) % p;
                }
                if acc.iter().all(|&c| c == 0) {
                    n += 1;
                } else {
                    let e = ext_pow(&acc, (q - 1) / 2, &m, p);
                    if e[0] == 1 && e[1..].iter().all(|&c| c == 0) {
                        n += 2;
                    }
                }
            }
            s.push(n - q as i128 - 1);
        }
        let pp = p as i128;
        let mut c = vec![0i128; g + 1];
        c[0] = 1;
        for k in 1..=g {
            let num: i128 = (1..=k).map(|i| s[i - 1] * c[k - i]).sum();
            assert_eq!(num % k as i128, 0);
            c[k] = num / k as i128;
        }
        (0..=g)
            .map(|i| c[i] * if i < g { 1 + pp.pow((g - i) as u32) } else { 1 })
            .sum()
    }

    /// `jacobian_order` against the brute-force zeta function (genus 2, 3,
    /// 4, at primes where the Weil window holds one multiple of the
    /// exponent), and the brute-force order annihilating random divisors at
    /// genus 4 and 5.
    #[test]
    fn order_matches_brute_force_zeta() {
        for (p, g) in [(31u64, 2usize), (37, 2), (17, 3), (19, 3), (29, 4)] {
            let want = if g == 4 { 1 } else { 3 };
            let mut done = 0;
            for cs in 1..400u64 {
                let f = random_f(p, g as u32, cs);
                let Ok(c) = JCurve::new(p, g as u32, &f) else {
                    continue;
                };
                let Ok(n) = c.jacobian_order(cs) else {
                    continue;
                };
                assert_eq!(
                    i128::from(n),
                    brute_order(p, g, &f),
                    "p = {p}, g = {g}, seed {cs}"
                );
                done += 1;
                if done == want {
                    break;
                }
            }
            assert_eq!(done, want, "too few determined orders at p = {p}, g = {g}");
        }
        for (p, g) in [(5u64, 4usize), (7, 4), (5, 5), (7, 5)] {
            let f = (1..200)
                .map(|s| random_f(p, g as u32, s))
                .find(|f| JCurve::new(p, g as u32, f).is_ok())
                .unwrap();
            let c = JCurve::new(p, g as u32, &f).unwrap();
            let n = brute_order(p, g, &f);
            assert!(n > 0);
            let mut st = 3u64;
            for _ in 0..40 {
                let d = c.random_divisor(&mut st);
                assert!(
                    c.mul_ltr(d, n as u64).is_identity(),
                    "p = {p}, g = {g}: #J = {n} does not kill a divisor"
                );
            }
        }
    }

    /// The parity prefilter: `+1` on split polynomials, `−1` with one
    /// irreducible quadratic factor, `0` on a repeated root.
    #[test]
    fn disc_character_parity() {
        let p = 103u64;
        let mut chi = vec![-1i8; p as usize];
        chi[0] = 0;
        for y in 1..p {
            chi[(y * y % p) as usize] = 1;
        }
        let nr = (2..p).find(|&a| chi[a as usize] == -1).unwrap();
        let mulp = |a: &[u64], b: &[u64]| -> Vec<u64> {
            let mut c = vec![0u64; a.len() + b.len() - 1];
            for i in 0..a.len() {
                for j in 0..b.len() {
                    c[i + j] = (c[i + j] + a[i] * b[j]) % p;
                }
            }
            c
        };
        let mut st = 1u64;
        let mut nat = Nat::default();
        for d in 2..=5usize {
            for _ in 0..300 {
                let mut roots: Vec<u64> = Vec::new();
                while roots.len() < d {
                    st = st.wrapping_add(0x9E37_79B9_7F4A_7C15);
                    let x = mix64(st) % p;
                    if !roots.contains(&x) {
                        roots.push(x);
                    }
                }
                let lin = |x: u64| vec![(p - x) % p, 1];
                let split = roots.iter().fold(vec![1u64], |acc, &x| mulp(&acc, &lin(x)));
                assert_eq!(disc_character(p, &split, &chi, &mut nat), 1);
                assert!(splits_completely(p, &split, &mut nat));
                // (x − a)² − nr is irreducible; with d − 2 distinct linear factors.
                let a = roots[0];
                let quad = vec![(a * a + p - nr) % p, (2 * (p - a)) % p, 1];
                let mixed = roots[2..].iter().fold(quad, |acc, &x| mulp(&acc, &lin(x)));
                assert_eq!(disc_character(p, &mixed, &chi, &mut nat), -1);
                // a repeated root
                let rep = roots[1..]
                    .iter()
                    .fold(lin(roots[1]), |acc, &x| mulp(&acc, &lin(x)));
                assert_eq!(disc_character(p, &rep, &chi, &mut nat), 0);
            }
        }
    }

    fn prm(p: u64, g: u32, arm: Arm, prefilter: bool, sge: Option<SgeParams>) -> IcParams {
        IcParams {
            arm,
            nb: fb_size_2lp(p, g, 2.0),
            seed: 5,
            max_trials: 5_000_000,
            restart: RESTART,
            prefilter,
            verify_relations: false,
            sge,
        }
    }

    /// `sge: None` is the frozen instrument: these counts were recorded from
    /// the source before the merge stage existed (2026-11-15) and must not
    /// move.  Fields: trials, relations, rel.mul, rel.add, cantor_mul,
    /// la.mul, la.add, la_wied.mul, core_rows, core_cols, recovered.
    #[test]
    fn sge_none_reproduces_frozen_counts() {
        #[rustfmt::skip]
        let gold: [(u64, u32, Arm, bool, u64, u64, u64, u64, u64, u64, u64, u64, usize, usize, u64); 16] = [
            (131, 2, Arm::Full, false, 145, 75, 17029, 14414, 59514, 19741, 36136, 19741, 72, 28, 2418),
            (131, 2, Arm::Full, true, 145, 75, 11374, 8034, 59514, 19741, 36136, 19741, 72, 28, 2418),
            (131, 2, Arm::DoubleLp, false, 99, 53, 11419, 9628, 55474, 2645, 3352, 2645, 32, 8, 2418),
            (131, 2, Arm::DoubleLp, true, 99, 53, 7558, 5272, 55474, 2645, 3352, 2645, 32, 8, 2418),
            (67, 3, Arm::Full, false, 4272, 39, 419722, 417673, 945740, 11597, 25023, 11597, 36, 27, 5447),
            (67, 3, Arm::Full, true, 4272, 39, 323304, 275525, 945740, 11597, 25023, 11597, 36, 27, 5447),
            (67, 3, Arm::DoubleLp, false, 4272, 39, 419722, 417673, 945740, 11597, 25023, 11597, 36, 27, 5447),
            (67, 3, Arm::DoubleLp, true, 4272, 39, 323304, 275525, 945740, 11597, 25023, 11597, 36, 27, 5447),
            (67, 4, Arm::Full, false, 1945, 39, 357024, 353828, 992336, 15695, 38397, 15695, 38, 30, 380849),
            (67, 4, Arm::Full, true, 1945, 39, 241824, 202296, 992336, 15695, 38397, 15695, 38, 30, 380849),
            (67, 4, Arm::DoubleLp, false, 1945, 39, 357024, 353828, 992336, 15695, 38397, 15695, 38, 30, 380849),
            (67, 4, Arm::DoubleLp, true, 1945, 39, 241824, 202296, 992336, 15695, 38397, 15695, 38, 30, 380849),
            (43, 5, Arm::Full, false, 2597, 35, 632387, 628324, 2205323, 13390, 37575, 13390, 35, 27, 15903666),
            (43, 5, Arm::Full, true, 2597, 35, 448821, 371486, 2205323, 13390, 37575, 13390, 35, 27, 15903666),
            (43, 5, Arm::DoubleLp, false, 2597, 35, 632387, 628324, 2205323, 13390, 37575, 13390, 35, 27, 15903666),
            (43, 5, Arm::DoubleLp, true, 2597, 35, 448821, 371486, 2205323, 13390, 37575, 13390, 35, 27, 15903666),
        ];
        let mut insts: HashMap<(u64, u32), Instance> = HashMap::new();
        for &(p, g, arm, pre, tr, rl, rm, ra, cm, lm, la, lw, cr, cc, t) in &gold {
            let inst = insts.entry((p, g)).or_insert_with(|| first_instance(p, g));
            let o = run_ic(inst, prm(p, g, arm, pre, None));
            let got = (
                o.trials,
                o.relations,
                o.rel.mul,
                o.rel.add,
                o.cantor_mul,
                o.la.mul,
                o.la.add,
                o.la_wied.mul,
                o.core_rows,
                o.core_cols,
                o.recovered,
            );
            assert_eq!(
                got,
                (tr, rl, rm, ra, cm, lm, la, lw, cr, cc, Some(t)),
                "p = {p} g = {g} {arm:?} pre = {pre}"
            );
            assert!(o.certified);
            assert_eq!(o.la_sge, Nat::default());
            assert_eq!(o.sge, SgeStats::default());
        }
    }

    /// With the merge the relation phase is identical, only the linear
    /// algebra changes, and the run certifies with the same `t`.
    #[test]
    fn sge_keeps_relation_phase_and_t() {
        let settings = [
            SgeParams {
                w_max: 2,
                fill_max: 3.0,
            },
            SgeParams {
                w_max: 4,
                fill_max: 3.0,
            },
            SgeParams {
                w_max: 8,
                fill_max: 3.0,
            },
            SgeParams {
                w_max: 8,
                fill_max: 1.0,
            },
        ];
        let mut reduced_some = false;
        for (p, g) in [
            (131u64, 2u32),
            (67, 3),
            (67, 4),
            (43, 5),
            (257, 3),
            (131, 4),
        ] {
            let inst = first_instance(p, g);
            for arm in [Arm::Full, Arm::DoubleLp] {
                let off = run_ic(&inst, prm(p, g, arm, true, None));
                assert!(off.certified);
                for sp in settings {
                    let on = run_ic(&inst, prm(p, g, arm, true, Some(sp)));
                    let tag = format!("p = {p} g = {g} {arm:?} {sp:?}");
                    assert!(on.certified, "{tag}");
                    assert_eq!(on.recovered, off.recovered, "{tag}");
                    assert_eq!(on.recovered, Some(inst.t_secret), "{tag}");
                    assert_eq!(on.trials, off.trials, "{tag}");
                    assert_eq!(on.candidates, off.candidates, "{tag}");
                    assert_eq!(on.splits, off.splits, "{tag}");
                    assert_eq!(on.relations, off.relations, "{tag}");
                    assert_eq!(on.rels_by_lp, off.rels_by_lp, "{tag}");
                    assert_eq!(on.rel, off.rel, "{tag}");
                    assert_eq!(on.group_ops(), off.group_ops(), "{tag}");
                    assert_eq!(on.cantor_mul, off.cantor_mul, "{tag}");
                    assert_eq!(on.cantor_inv, off.cantor_inv, "{tag}");
                    assert_eq!(on.attempts, off.attempts, "{tag}");
                    assert_eq!(on.la_graph, off.la_graph, "{tag}");
                    assert_eq!(on.la_peel, off.la_peel, "{tag}");
                    assert_eq!((on.core_rows, on.core_cols), (off.core_rows, off.core_cols));
                    assert_eq!(on.sge.rows_in, on.core_rows, "{tag}");
                    assert_eq!(on.sge.cols_in, on.core_cols, "{tag}");
                    assert!(on.sge.nnz_out as f64 <= sp.fill_max * on.sge.nnz_in as f64 + 1e-9);
                    assert_eq!(
                        on.la,
                        Nat {
                            mul: on.la_graph.mul + on.la_peel.mul + on.la_sge.mul + on.la_wied.mul,
                            add: on.la_graph.add
                                + on.la_peel.add
                                + on.la_sge.add
                                + on.la_wied.add
                                + (off.la.add
                                    - off.la_graph.add
                                    - off.la_peel.add
                                    - off.la_wied.add),
                        },
                        "{tag}"
                    );
                    if on.sge.cols_out < on.sge.cols_in {
                        reduced_some = true;
                    }
                }
            }
        }
        assert!(reduced_some, "the merge never removed a column");
    }

    /// The stream skip advances the generator exactly as `wiedemann` does.
    #[test]
    fn skip_draws_match_wiedemann() {
        let r = 1_000_003u64;
        for (nrow, ncol, dense) in [(30usize, 20usize, false), (30, 20, true), (12, 10, false)] {
            let mut st = 77u64;
            let rows: Vec<Row> = (0..nrow)
                .map(|_| {
                    let mut t: Vec<(u32, u64)> = (0..3)
                        .map(|_| {
                            st = st.wrapping_add(0x9E37_79B9_7F4A_7C15);
                            (
                                (mix64(st) % ncol as u64) as u32,
                                1 + mix64(st ^ 9) % (r - 1),
                            )
                        })
                        .collect();
                    t.sort_unstable_by_key(|x| x.0);
                    t.dedup_by_key(|x| x.0);
                    (t, 1)
                })
                .collect();
            let mut a = StdRng::seed_from_u64(3);
            let mut b = a.clone();
            let mut nat = Nat::default();
            let _ = wiedemann(&rows, ncol, r, &mut a, dense, &mut nat);
            skip_wiedemann_draws(&mut b, nrow, ncol, r, dense);
            assert_eq!(a.gen::<u64>(), b.gen::<u64>());
        }
    }

    /// On a random sparse system with general coefficients, the merged
    /// system has the same `t`, and its counts are charged.
    #[test]
    fn sge_preserves_t_on_random_systems() {
        let r = 1_000_000_007u64;
        // Sparse enough that columns of weight 2 and 3 exist.
        let (nrow, ncol) = (60usize, 41usize);
        let tcol = (ncol - 1) as u32;
        let mut st = 5u64;
        let mut next = || {
            st = st.wrapping_add(0x9E37_79B9_7F4A_7C15);
            mix64(st)
        };
        for trial in 0..6 {
            let x: Vec<u64> = (0..ncol).map(|_| next() % r).collect();
            let rows: Vec<Row> = (0..nrow)
                .map(|_| {
                    let k = 2 + next() % 2;
                    let mut t: Vec<(u32, u64)> = (0..k)
                        .map(|_| {
                            let c = (next() % (ncol as u64 - 1)) as u32;
                            let v = match next() % 4 {
                                0 => r - 1,
                                1 | 2 => 1,
                                _ => 1 + next() % (r - 1),
                            };
                            (c, v)
                        })
                        .collect();
                    t.push((tcol, 1 + next() % (r - 1)));
                    t.sort_unstable_by_key(|e| e.0);
                    t.dedup_by_key(|e| e.0);
                    let rhs = t
                        .iter()
                        .fold(0, |acc, &(c, v)| addm(acc, mulmod(v, x[c as usize], r), r));
                    (t, rhs)
                })
                .collect();
            for sp in [
                SgeParams {
                    w_max: 3,
                    fill_max: 2.0,
                },
                SgeParams {
                    w_max: 10,
                    fill_max: 4.0,
                },
            ] {
                let mut nat = Nat::default();
                let (red, rc, rt, s) = sge_reduce(&rows, ncol, tcol, r, sp, &mut nat);
                assert!(s.merged > 0 && nat.add > 0, "trial {trial} {s:?}");
                assert_eq!(s.rows_out, red.len());
                assert!(s.cols_out < ncol);
                assert_eq!(
                    s.rows_in - s.rows_out,
                    s.merged + s.peeled + s.empty + s.duplicates
                );
                let mut rng = StdRng::seed_from_u64(trial);
                let mut wn = Nat::default();
                let sol = wiedemann(&red, rc, r, &mut rng, false, &mut wn);
                assert_eq!(
                    sol.map(|v| v[rt as usize]),
                    Some(x[tcol as usize]),
                    "trial {trial} {sp:?}"
                );
            }
        }
    }

    /// Every arm recovers the planted logarithm at every genus; the prefilter
    /// changes the native count and nothing else.
    #[test]
    fn arms_certify_and_prefilter_is_exact() {
        for (p, g) in [(131u64, 2u32), (67, 3), (67, 4), (43, 5)] {
            let inst = first_instance(p, g);
            assert!(inst.curve.mul_ltr(inst.g, inst.r).is_identity());
            for arm in [Arm::Full, Arm::DoubleLp] {
                let mk = |prefilter: bool, verify: bool| IcParams {
                    arm,
                    nb: fb_size_2lp(p, g, 2.0),
                    seed: 5,
                    max_trials: 5_000_000,
                    restart: RESTART,
                    prefilter,
                    verify_relations: verify,
                    sge: None,
                };
                let chk = run_ic(&inst, mk(false, true));
                assert!(chk.certified, "{arm:?} p = {p} g = {g}");
                assert_eq!(chk.bad_relations, 0);
                assert_eq!(chk.recovered, Some(inst.t_secret));
                let off = run_ic(&inst, mk(false, false));
                let on = run_ic(&inst, mk(true, false));
                assert!(off.certified && on.certified);
                assert_eq!(off.recovered, on.recovered);
                assert_eq!(off.trials, on.trials);
                assert_eq!(off.relations, on.relations);
                assert_eq!(off.splits, on.splits);
                assert_eq!(off.group_ops(), on.group_ops());
                assert_eq!(off.cantor_mul, on.cantor_mul);
                assert_eq!(off.la.mul, on.la.mul);
                assert!(
                    on.rel.mul < off.rel.mul,
                    "prefilter saved nothing at g = {g}"
                );
            }
            let rho = run_rho(&inst, 3);
            assert!(rho.certified);
            assert_eq!(rho.recovered, Some(inst.t_secret));
            assert!(rho.cantor_mul > 0);
        }
    }

    // ── order methods (fast order study, 2026-11-21) ────────────────────

    /// The enumeration of `count_points(2)` as it was before the Barrett
    /// reduction: the reference every new count is compared with.
    fn count_points2_reference(c: &JCurve) -> u64 {
        let p = c.k.p;
        let mut sq = vec![false; p as usize];
        for y in 1..p {
            sq[(y * y % p) as usize] = true;
        }
        let nu = (2..p).find(|&a| !sq[a as usize]).expect("a non-residue");
        let f = &c.f;
        let mut n = 1u64;
        for a in 0..p {
            for b in 0..=(p - 1) / 2 {
                let (mut ra, mut rb) = (0u64, 0u64);
                for i in (0..f.n).rev() {
                    let na = (ra * a + nu * (rb * b % p)) % p;
                    let nb = (ra * b + rb * a) % p;
                    ra = (na + f.c[i]) % p;
                    rb = nb;
                }
                let w = if b == 0 { 1 } else { 2 };
                if ra == 0 && rb == 0 {
                    n += w;
                } else {
                    let norm = (ra * ra % p + p - nu * (rb * rb % p) % p) % p;
                    if sq[norm as usize] {
                        n += 2 * w;
                    }
                }
            }
        }
        n
    }

    #[test]
    fn count_points2_matches_reference() {
        for (p, g, cs) in [
            (67u64, 3u32, 1u64),
            (257, 3, 2),
            (1031, 3, 3),
            (1031, 4, 4),
            (2053, 2, 5),
        ] {
            let c = JCurve::new(p, g, &random_f(p, g, cs)).unwrap();
            assert_eq!(
                c.count_points(2),
                count_points2_reference(&c),
                "p={p} g={g}"
            );
        }
    }

    /// `Ok(order)` or the class of the error, for comparing routes.
    fn order_outcome(c: &JCurve, m: OrderMethod, seed: u64) -> String {
        match c.jacobian_order_with(m, seed) {
            Ok(n) => format!("ok {n}"),
            Err(e) => format!("err {}", e.split(':').next().unwrap_or("")),
        }
    }

    fn json_instances(src: &str) -> Vec<(u64, Vec<u64>, u64, u64)> {
        let v: serde_json::Value = serde_json::from_str(src).unwrap();
        v["instances"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|i| i["g"] == 3 && i["family"] == "hyperelliptic")
            .map(|i| {
                (
                    i["p"].as_u64().unwrap(),
                    i["f"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|x| x.as_u64().unwrap())
                        .collect(),
                    i["order"].as_u64().unwrap(),
                    i["curve_seed"].as_u64().unwrap(),
                )
            })
            .collect()
    }

    fn check_recorded(src: &str, p_max: u64, p_min: u64) -> usize {
        let mut n = 0;
        for (p, f, order, cs) in json_instances(src) {
            if p > p_max || p < p_min {
                continue;
            }
            let c = JCurve::new(p, 3, &f).unwrap();
            assert_eq!(
                c.jacobian_order_with(OrderMethod::Bsgs1, cs),
                Ok(order),
                "bsgs1 p={p} cs={cs}"
            );
            assert_eq!(
                c.jacobian_order_with(OrderMethod::Enumerate, cs),
                Ok(order),
                "enum p={p} cs={cs}"
            );
            if p <= 1031 {
                assert_eq!(c.jacobian_order_with(OrderMethod::Bsgs2, cs), Ok(order));
            }
            n += 1;
        }
        n
    }

    #[test]
    fn bsgs1_matches_enumerate_on_recorded_instances() {
        let a = check_recorded(
            include_str!("../../research/genus_ladder_20261114/analysis.json"),
            4099,
            0,
        );
        assert_eq!(a, 28);
    }

    #[test]
    #[ignore = "minutes: enumerates p = 8209, 16411 and 32771"]
    fn bsgs1_matches_enumerate_on_scale_instances() {
        let b = check_recorded(
            include_str!("../../research/genus_ladder_scale_20261115/g3_analysis.json"),
            32771,
            0,
        );
        assert_eq!(b, 9);
    }

    #[test]
    fn bsgs1_matches_enumerate_on_random_curves() {
        let p = 1031u64;
        let (mut ok, mut err, mut tested) = (0, 0, 0);
        for cs in 9000..9050u64 {
            let Ok(c) = JCurve::new(p, 3, &random_f(p, 3, cs)) else {
                continue;
            };
            tested += 1;
            let e = order_outcome(&c, OrderMethod::Enumerate, cs);
            let b = order_outcome(&c, OrderMethod::Bsgs1, cs);
            let b2 = order_outcome(&c, OrderMethod::Bsgs2, cs);
            println!("RANDOM cs={cs} enumerate={e} bsgs1={b} bsgs2={b2}");
            assert_eq!(e, b, "cs={cs}");
            assert_eq!(e, b2, "cs={cs}");
            if e.starts_with("ok") {
                ok += 1;
            } else {
                err += 1;
            }
        }
        println!("RANDOM tested={tested} ok={ok} err={err}");
        assert!(tested >= 45);
    }

    #[test]
    fn bsgs_table_size_is_monotone_and_bounded() {
        assert!(bsgs_table_size(131101, 3, OrderMethod::Bsgs1) < MAX_BSGS_TABLE);
        assert!(
            bsgs_table_size(131101, 3, OrderMethod::Bsgs1)
                < bsgs_table_size(262147, 3, OrderMethod::Bsgs1)
        );
    }

    /// Timing probe: `PROBE_ORDER=p,g,method,cs cargo test --release --lib
    /// order_probe -- --ignored --nocapture`.
    #[test]
    #[ignore = "manual timing probe"]
    fn order_probe() {
        let v = std::env::var("PROBE_ORDER").expect("PROBE_ORDER=p,g,method,cs");
        let a: Vec<&str> = v.split(',').collect();
        let (p, g, cs): (u64, u32, u64) = (
            a[0].parse().unwrap(),
            a[1].parse().unwrap(),
            a[3].parse().unwrap(),
        );
        let c = JCurve::new(p, g, &random_f(p, g, cs)).unwrap();
        if a[2] == "count2" {
            let t = std::time::Instant::now();
            let n = c.count_points(2);
            println!(
                "PROBE p={p} g={g} count_points(2)={n} wall_s={:.3}",
                t.elapsed().as_secs_f64()
            );
            if a.len() > 4 {
                let t = std::time::Instant::now();
                let r = count_points2_reference(&c);
                println!(
                    "PROBE reference={r} wall_s={:.3}",
                    t.elapsed().as_secs_f64()
                );
            }
            return;
        }
        let m = OrderMethod::parse(a[2]).unwrap();
        let t = std::time::Instant::now();
        let r = c.jacobian_order_with(m, cs);
        println!(
            "PROBE p={p} g={g} method={} cs={cs} result={r:?} table_bound={} wall_s={:.3}",
            m.name(),
            bsgs_table_size(p, g as usize, m),
            t.elapsed().as_secs_f64()
        );
    }

    /// The legacy policy is `run_rho_walk` count for count; the window-free
    /// walks (arms B and Q) certify, and the plain walk without its window is
    /// the same walk with zero compares (validator finding B13).
    #[test]
    fn lean_rho_walks_certify_and_drop_window_compares() {
        for (p, g) in [(131u64, 3u32), (131, 4), (257, 3), (1031, 3)] {
            let inst = first_instance(p, g);
            for seed in 1..=3u64 {
                let a = run_rho_walk(&inst, seed, RhoWalk::negation());
                let b = run_rho_policy(&inst, seed, RhoWalk::negation(), WalkPolicy::legacy());
                assert_eq!(
                    (a.recovered, a.steps, a.cantor_mul, a.compares, a.native),
                    (b.recovered, b.steps, b.cantor_mul, b.compares, b.native)
                );
                let mut nw = RhoWalk::negation();
                nw.cycle_window = 0;
                let br = run_rho_policy(&inst, seed, nw, WalkPolicy::v2());
                assert!(br.certified, "B p={p} g={g} seed={seed}");
                assert_eq!(br.recovered, Some(inst.t_secret));
                let pl = run_rho_walk(&inst, seed, RhoWalk::plain());
                let mut pw = RhoWalk::plain();
                pw.cycle_window = 0;
                let q = run_rho_policy(&inst, seed, pw, WalkPolicy::legacy());
                assert!(q.certified);
                assert_eq!(q.compares, 0);
                if pl.compares > 0 {
                    // same walk when the window never fired
                    assert_eq!((pl.recovered, pl.steps), (q.recovered, q.steps));
                }
            }
        }
    }
}
