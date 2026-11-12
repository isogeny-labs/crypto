//! # Word-size hyperelliptic Jacobian for larger-prime tower points
//! (genus <= 3, `p < 2^31`).
//!
//! [`crate::cryptanalysis::hyperelliptic_word::HCurve`] holds a reduced Mumford
//! divisor in `u8` slots and tables for `p < 256`.  This module is the same
//! group on `u32` slots with `u64` arithmetic, so the cover tower of
//! [`super::tower`] can be carried to primes in the thousands.  Products of
//! two residues below `2^31` fit a `u64`; the linear-algebra routine of
//! [`ic`] switches to `u128` products only for a modulus past `2^32`.
//!
//! ## Model
//!
//! The model is the one `HCurve` and `tower.rs` use, nothing new: the
//! tower's even-degree curve `C : y² = F(x²)` (degree `2g + 2 = 8` or `6` in
//! `x`) has the rational Weierstrass point `(a, 0)`, and `t = 1/(x − a)`,
//! `Yc = y·t^{g+1}` sends it to infinity, giving the **imaginary** (odd)
//! model `Yc² = g(t)`, `deg g = 2g + 1`.  [`H64Curve::new`] takes that `g`,
//! exactly like `HCurve::new`, so a pulled-up instance transfers unchanged
//! (the tests feed it `HCurve::f_coeffs()` of the `p < 256` towers and
//! compare divisor for divisor).  [`Cover64`] and [`build64`] are the
//! `Cover` and `build` of `tower.rs` with `H64Curve` in place of `HCurve`
//! (copied, with one added knob: a floor on the prime `r`, and a cap on cover
//! attempts at large `p`).
//!
//! - **Representation.**  [`H64Div`]: `D = (u, v)`, `u` monic of degree
//!   `deg ≤ g` (non-leading coefficients stored), `deg v < deg u`; identity
//!   is `deg = 0`.  Reduced representatives are unique.  Polynomials are
//!   fixed arrays of 16 `u64` ([`Poly`]); the largest Cantor intermediate at
//!   `g = 3` has 10 coefficients.
//! - **Arithmetic.**  Cantor composition by one or two extended gcds, then
//!   the reduction loop `u' = (f − v²)/u`, `v' = −v mod u'`; the same
//!   algorithm as `HCurve::cantor`.  Doubling is composition with itself.
//! - **Charge.**  [`CountedGroup`] counts one addition, or one doubling when
//!   the operands are equal, per call, whatever the field work behind it.
//! - **Key.**  `Σ_{e<deg} p^{2e}` plus the base-`p` index of `(v, u)`,
//!   computed with wrapping `u64` arithmetic.  For `p < 256`, `g ≤ 3` it
//!   equals `HCurve::key_of` and is injective; past that it wraps and is a
//!   64-bit fingerprint, not an injection (only `rho` on the Jacobian reads
//!   it).
//! - **Group order.**  [`H64Curve::jacobian_order`]: the first `g − 1`
//!   coefficients of `L(T)` from `N_1 … N_{g−1}` (point counts over `F_p`
//!   and `F_{p²}` by enumeration, the latter `O(p²)` and split over
//!   threads), then the multiples of the prime `r` in the Hasse–Weil window
//!   for the last coefficient that kill 32 random divisors; exactly one
//!   must survive.  Same method as `HCurve::jacobian_order`.
//! - **Index calculus.**  [`ic::solve`] is `ecbench::hyper_ic::solve` ported
//!   to this group, line for line (same random stream, same walk, same
//!   counters), so that on `p < 256` the two give identical counts.
//!
//! Every figure produced with it is `[pcc:unverified]` until replayed.

use std::collections::BTreeMap;

use super::ec::{Curve, Pt};
use super::field::Fp;
use crate::cryptanalysis::ic_boundary::{CountedGroup, GroupOps};
use crate::cryptanalysis::residual_walk::mix64;

/// Largest genus the fixed arrays hold.
pub const MAX_GENUS: usize = 3;
/// Polynomial capacity (coefficients).
const CAP: usize = 16;

/// A reduced Mumford divisor `(u, v)` on a curve of genus ≤ 3 over `F_p`,
/// `p < 2^31`.  `u = x^deg + u[deg−1] x^{deg−1} + … + u[0]`, `v = Σ v[i] x^i`
/// (`i < deg`); unused slots are zero.  It does not record its curve.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct H64Div {
    pub deg: u8,
    pub u: [u32; MAX_GENUS],
    pub v: [u32; MAX_GENUS],
}

impl H64Div {
    /// The identity class `(1, 0)`.
    pub const IDENTITY: H64Div = H64Div {
        deg: 0,
        u: [0; MAX_GENUS],
        v: [0; MAX_GENUS],
    };

    pub fn is_identity(&self) -> bool {
        self.deg == 0
    }

    /// `u`, lowest degree first, with its leading 1 (length `deg + 1`).
    pub fn u_coeffs(&self) -> Vec<u64> {
        let d = self.deg as usize;
        let mut out: Vec<u64> = self.u[..d].iter().map(|&c| u64::from(c)).collect();
        out.push(1);
        out
    }

    /// `v`, lowest degree first, without trailing zeros.
    pub fn v_coeffs(&self) -> Vec<u64> {
        let mut out: Vec<u64> = self.v[..self.deg as usize]
            .iter()
            .map(|&c| u64::from(c))
            .collect();
        while out.last() == Some(&0) {
            out.pop();
        }
        out
    }
}

/// A polynomial over `F_p` on the stack: `c[..n]`, `c[n − 1] ≠ 0`, the rest
/// zero.  `n = 0` is the zero polynomial.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Poly {
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

/// `a⁻¹ mod p` by the extended Euclidean algorithm, `a ≠ 0`.
#[inline]
fn inv_mod_p(p: u64, a: u64) -> u64 {
    debug_assert!(a != 0 && a < p);
    let (mut r0, mut r1) = (p as i64, a as i64);
    let (mut t0, mut t1) = (0i64, 1i64);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    debug_assert_eq!(r0, 1);
    t0.rem_euclid(p as i64) as u64
}

fn padd(k: &Fp, a: &Poly, b: &Poly) -> Poly {
    let mut q = Poly::ZERO;
    let n = a.n.max(b.n);
    for i in 0..n {
        q.c[i] = k.add(a.c[i], b.c[i]);
    }
    q.n = n;
    q.trim();
    q
}

fn psub(k: &Fp, a: &Poly, b: &Poly) -> Poly {
    let mut q = Poly::ZERO;
    let n = a.n.max(b.n);
    for i in 0..n {
        q.c[i] = k.sub(a.c[i], b.c[i]);
    }
    q.n = n;
    q.trim();
    q
}

fn pscale(k: &Fp, a: &Poly, s: u64) -> Poly {
    let mut q = Poly::ZERO;
    for i in 0..a.n {
        q.c[i] = k.mul(a.c[i], s);
    }
    q.n = a.n;
    q.trim();
    q
}

fn pmul(k: &Fp, a: &Poly, b: &Poly) -> Poly {
    if a.is_zero() || b.is_zero() {
        return Poly::ZERO;
    }
    let n = a.n + b.n - 1;
    assert!(n <= CAP, "polynomial product past capacity");
    let mut q = Poly::ZERO;
    for i in 0..a.n {
        if a.c[i] == 0 {
            continue;
        }
        for j in 0..b.n {
            // a.c[i]·b.c[j] < 2^62 and q.c[·] < 2^31: no overflow.
            q.c[i + j] = (q.c[i + j] + a.c[i] * b.c[j]) % k.p;
        }
    }
    q.n = n;
    q.trim();
    q
}

/// `(a div b, a mod b)`, `b ≠ 0`.
fn pdivrem(k: &Fp, a: &Poly, b: &Poly) -> (Poly, Poly) {
    assert!(!b.is_zero(), "division by the zero polynomial");
    let mut r = *a;
    let mut q = Poly::ZERO;
    if a.n < b.n {
        return (q, r);
    }
    let li = if b.lead() == 1 {
        1
    } else {
        inv_mod_p(k.p, b.lead())
    };
    let db = b.deg();
    while r.n >= b.n && !r.is_zero() {
        let shift = r.deg() - db;
        let coef = if li == 1 {
            r.lead()
        } else {
            k.mul(r.lead(), li)
        };
        q.c[shift] = coef;
        for j in 0..b.n {
            let t = k.mul(coef, b.c[j]);
            r.c[shift + j] = k.sub(r.c[shift + j], t);
        }
        r.trim();
    }
    q.n = a.n - b.n + 1;
    q.trim();
    (q, r)
}

fn prem(k: &Fp, a: &Poly, b: &Poly) -> Poly {
    pdivrem(k, a, b).1
}

fn pmonic(k: &Fp, a: &Poly) -> Poly {
    if a.is_zero() || a.lead() == 1 {
        return *a;
    }
    pscale(k, a, inv_mod_p(k.p, a.lead()))
}

/// `(g, s, t)` with `s·a + t·b = g`, `g` monic (zero only for `a = b = 0`).
fn pxgcd(k: &Fp, a: &Poly, b: &Poly) -> (Poly, Poly, Poly) {
    let (mut r0, mut r1) = (*a, *b);
    let (mut s0, mut s1) = (Poly::constant(1), Poly::ZERO);
    let (mut t0, mut t1) = (Poly::ZERO, Poly::constant(1));
    while !r1.is_zero() {
        let (q, r) = pdivrem(k, &r0, &r1);
        r0 = r1;
        r1 = r;
        let s = psub(k, &s0, &pmul(k, &q, &s1));
        s0 = s1;
        s1 = s;
        let t = psub(k, &t0, &pmul(k, &q, &t1));
        t0 = t1;
        t1 = t;
    }
    if r0.is_zero() {
        return (r0, s0, t0);
    }
    let li = inv_mod_p(k.p, r0.lead());
    (pscale(k, &r0, li), pscale(k, &s0, li), pscale(k, &t0, li))
}

fn peval(k: &Fp, a: &Poly, x: u64) -> u64 {
    let mut acc = 0;
    for i in (0..a.n).rev() {
        acc = (acc * x + a.c[i]) % k.p;
    }
    acc
}

/// A curve `y² = f(x)` of genus `g ≤ 3` over `F_p`, `p < 2^31`, `deg f =
/// 2g + 1`, `f` squarefree, with Cantor's arithmetic on its Jacobian.
#[derive(Clone)]
pub struct H64Curve {
    k: Fp,
    genus: u8,
    f: Poly,
    /// `Σ_{e<d} p^{2e}` (wrapping): where the degree-`d` keys start.
    offsets: [u64; MAX_GENUS + 1],
}

impl std::fmt::Debug for H64Curve {
    fn fmt(&self, fm: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fm.debug_struct("H64Curve")
            .field("p", &self.k.p)
            .field("genus", &self.genus)
            .field("f", &self.f_coeffs())
            .finish()
    }
}

impl H64Curve {
    /// The curve `y² = f[0] + f[1] x + … + f[2g+1] x^{2g+1}` over `F_p`.
    /// Requires an odd prime `p < 2^31`, `1 ≤ g ≤ 3`, coefficients below
    /// `p`, `deg f = 2g + 1` exactly and `f` squarefree (the checks of
    /// `HCurve::new`).
    pub fn new(p: u64, genus: u32, f: &[u64]) -> Result<H64Curve, String> {
        let k = Fp::new(p).ok_or_else(|| format!("p must be an odd prime below 2^31, not {p}"))?;
        if !(1..=MAX_GENUS as u32).contains(&genus) {
            return Err(format!("genus must be 1 to {MAX_GENUS}, not {genus}"));
        }
        if let Some(&c) = f.iter().find(|&&c| c >= p) {
            return Err(format!("coefficient {c} is not reduced mod {p}"));
        }
        if f.len() > CAP {
            return Err("too many coefficients".into());
        }
        let fp = Poly::from_slice(f);
        if fp.is_zero() || fp.deg() != 2 * genus as usize + 1 {
            return Err(format!(
                "deg f must be 2g + 1 = {} for the imaginary model",
                2 * genus + 1
            ));
        }
        // Squarefree: gcd(f, f') = 1.
        let mut d = Poly::ZERO;
        for i in 1..fp.n {
            d.c[i - 1] = k.mul(fp.c[i], i as u64 % p);
        }
        d.n = fp.n - 1;
        d.trim();
        if d.is_zero() || pxgcd(&k, &fp, &d).0.n != 1 {
            return Err("f is not squarefree".into());
        }
        let mut offsets = [0u64; MAX_GENUS + 1];
        let p2 = p.wrapping_mul(p);
        let mut step = 1u64;
        for e in 1..=MAX_GENUS {
            offsets[e] = offsets[e - 1].wrapping_add(step);
            step = step.wrapping_mul(p2);
        }
        Ok(H64Curve {
            k,
            genus: genus as u8,
            f: fp,
            offsets,
        })
    }

    pub fn p(&self) -> u64 {
        self.k.p
    }

    pub fn genus(&self) -> u32 {
        u32::from(self.genus)
    }

    /// `f`, lowest degree first, length `2g + 2`.
    pub fn f_coeffs(&self) -> Vec<u64> {
        self.f.c[..self.f.n].to_vec()
    }

    fn polys(&self, d: &H64Div) -> (Poly, Poly) {
        let n = d.deg as usize;
        let mut u = Poly::ZERO;
        for i in 0..n {
            u.c[i] = u64::from(d.u[i]);
        }
        u.c[n] = 1;
        u.n = n + 1;
        let mut v = Poly::ZERO;
        for i in 0..n {
            v.c[i] = u64::from(d.v[i]);
        }
        v.n = n;
        v.trim();
        (u, v)
    }

    fn pack(&self, u: &Poly, v: &Poly) -> H64Div {
        debug_assert!(u.lead() == 1 && u.deg() <= self.genus as usize);
        debug_assert!(v.is_zero() || v.deg() < u.deg());
        let mut d = H64Div {
            deg: u.deg() as u8,
            ..H64Div::IDENTITY
        };
        for i in 0..u.deg() {
            d.u[i] = u.c[i] as u32;
        }
        for i in 0..v.n {
            d.v[i] = v.c[i] as u32;
        }
        d
    }

    /// `D` is a reduced divisor on this curve: `deg ≤ g`, coefficients below
    /// `p`, unused slots zero, and `u | f − v²`.
    pub fn is_valid(&self, d: &H64Div) -> bool {
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
        let t = psub(&self.k, &self.f, &pmul(&self.k, &v, &v));
        prem(&self.k, &t, &u).is_zero()
    }

    /// `[P] − [∞]` for an affine point `P = (x, y)` on the curve.
    pub fn point(&self, x: u64, y: u64) -> Option<H64Div> {
        let p = self.k.p;
        if x >= p || y >= p {
            return None;
        }
        if self.k.mul(y, y) != peval(&self.k, &self.f, x) {
            return None;
        }
        let mut d = H64Div {
            deg: 1,
            ..H64Div::IDENTITY
        };
        d.u[0] = ((p - x) % p) as u32;
        d.v[0] = y as u32;
        Some(d)
    }

    /// A random divisor: the sum of `g` random affine points (either
    /// ordinate), from a SplitMix64 stream `state`.
    pub fn random_divisor(&self, state: &mut u64) -> H64Div {
        let p = self.k.p;
        let mut d = H64Div::IDENTITY;
        let mut placed = 0;
        while placed < self.genus {
            *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let h = mix64(*state);
            let x = h % p;
            let Some(s) = self.k.sqrt(peval(&self.k, &self.f, x)) else {
                continue;
            };
            let y = if (h >> 63) & 1 == 1 { self.k.neg(s) } else { s };
            let pt = self.point(x, y).expect("a lifted point lies on the curve");
            d = self.cantor(d, pt);
            placed += 1;
        }
        d
    }

    /// Cantor's composition and reduction, uncounted.
    pub fn cantor(&self, a: H64Div, b: H64Div) -> H64Div {
        if a.is_identity() {
            return b;
        }
        if b.is_identity() {
            return a;
        }
        let k = &self.k;
        let (u1, v1) = self.polys(&a);
        let (u2, v2) = self.polys(&b);
        let (d1, e1, e2) = pxgcd(k, &u1, &u2);
        let (u, v) = if d1.n == 1 {
            // gcd(u₁, u₂) = 1: v ≡ vᵢ (mod uᵢ) by CRT, e₁u₁ ≡ 1 (mod u₂).
            let u = pmul(k, &u1, &u2);
            let t = pmul(k, &pmul(k, &e1, &u1), &psub(k, &v2, &v1));
            (u, prem(k, &padd(k, &v1, &t), &u))
        } else {
            let (d, c1, c2) = pxgcd(k, &d1, &padd(k, &v1, &v2));
            let s1 = pmul(k, &c1, &e1);
            let s2 = pmul(k, &c1, &e2);
            let (u, ur) = pdivrem(k, &pmul(k, &u1, &u2), &pmul(k, &d, &d));
            debug_assert!(ur.is_zero(), "u₁u₂/d² is exact");
            let t1 = pmul(k, &pmul(k, &s1, &u1), &v2);
            let t2 = pmul(k, &pmul(k, &s2, &u2), &v1);
            let t3 = pmul(k, &c2, &padd(k, &pmul(k, &v1, &v2), &self.f));
            let (vq, vr) = pdivrem(k, &padd(k, &padd(k, &t1, &t2), &t3), &d);
            debug_assert!(vr.is_zero(), "the v numerator divides by d");
            (u, prem(k, &vq, &u))
        };
        self.reduce(u, v)
    }

    fn reduce(&self, mut u: Poly, mut v: Poly) -> H64Div {
        let k = &self.k;
        let g = self.genus as usize;
        while u.deg() > g {
            let t = psub(k, &self.f, &pmul(k, &v, &v));
            let (q, r) = pdivrem(k, &t, &u);
            debug_assert!(r.is_zero(), "(f − v²)/u is exact");
            let un = pmonic(k, &q);
            let mut nv = Poly::ZERO;
            for i in 0..v.n {
                nv.c[i] = k.neg(v.c[i]);
            }
            nv.n = v.n;
            v = prem(k, &nv, &un);
            u = un;
        }
        let u = pmonic(k, &u);
        self.pack(&u, &v)
    }

    /// `−D = (u, −v)`.
    pub fn negate(&self, d: H64Div) -> H64Div {
        let mut n = d;
        for i in 0..d.deg as usize {
            n.v[i] = self.k.neg(u64::from(d.v[i])) as u32;
        }
        n
    }

    /// `[k]D`, uncounted (the counted double-and-add of [`CountedGroup::mul`]).
    pub fn mul_uncounted(&self, d: H64Div, k: u64) -> H64Div {
        let mut ops = GroupOps::default();
        CountedGroup::mul(self, &mut ops, d, k)
    }

    /// `[k]D` by right-to-left binary expansion: a second, independent path
    /// for certificates (`[k]G == T`), not the one the solvers use.
    pub fn mul_rtl(&self, d: H64Div, mut k: u64) -> H64Div {
        let mut acc = H64Div::IDENTITY;
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

    /// The key (see the module notes).
    pub fn key_of(&self, d: &H64Div) -> u64 {
        let p = self.k.p;
        let n = d.deg as usize;
        let mut idx = 0u64;
        for i in (0..n).rev() {
            idx = idx.wrapping_mul(p).wrapping_add(u64::from(d.v[i]));
        }
        for i in (0..n).rev() {
            idx = idx.wrapping_mul(p).wrapping_add(u64::from(d.u[i]));
        }
        self.offsets[n].wrapping_add(idx)
    }

    /// The divisor of `u` (with its leading 1, lowest degree first) and `v`
    /// (lowest first, `deg v < deg u`), checked to be reduced on this curve.
    pub fn from_coeffs(&self, u: &[u64], v: &[u64]) -> Option<H64Div> {
        let deg = u.len().checked_sub(1)?;
        if deg > self.genus as usize || u[deg] != 1 {
            return None;
        }
        let mut v = v.to_vec();
        while v.last() == Some(&0) {
            v.pop();
        }
        if v.len() > deg {
            return None;
        }
        let p = self.k.p;
        if u.iter().chain(v.iter()).any(|&c| c >= p) {
            return None;
        }
        let mut d = H64Div {
            deg: deg as u8,
            ..H64Div::IDENTITY
        };
        for i in 0..deg {
            d.u[i] = u[i] as u32;
        }
        for (i, &c) in v.iter().enumerate() {
            d.v[i] = c as u32;
        }
        self.is_valid(&d).then_some(d)
    }

    // ── Point counts and the group order ────────────────────────────

    /// `#C(F_{p^k})` (the point at infinity included), `k ∈ {1, 2}`, by
    /// enumeration: `O(p^k)` evaluations of `f`; `k = 2` is split over
    /// threads and uses `x ↔ x̄` (the conjugate abscissa has the conjugate
    /// ordinate, same point count).
    pub fn count_points(&self, k: u32) -> u64 {
        assert!((1..=2).contains(&k), "point counts over F_p^k for k ≤ 2");
        let p = self.k.p;
        let mut sq = vec![false; p as usize];
        for y in 0..p {
            sq[self.k.mul(y, y) as usize] = true;
        }
        if k == 1 {
            let mut n = 1u64;
            for x in 0..p {
                let y2 = peval(&self.k, &self.f, x);
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
        // F_{p²} = F_p[t]/(t² − ν), ν a non-residue.
        let nu = self.k.nr;
        let f = &self.f;
        let kk = &self.k;
        let sq = &sq;
        let count_a = |a: u64| -> u64 {
            let mut n = 0u64;
            for b in 0..=(p - 1) / 2 {
                // Horner in F_p[t]/(t² − ν).
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
                    let norm = kk.sub(kk.mul(ra, ra), kk.mul(nu, kk.mul(rb, rb)));
                    if sq[norm as usize] {
                        n += 2 * w;
                    }
                }
            }
            n
        };
        let threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
            .min(8) as u64;
        let total = std::sync::atomic::AtomicU64::new(1);
        let next = std::sync::atomic::AtomicU64::new(0);
        std::thread::scope(|s| {
            for _ in 0..threads {
                s.spawn(|| {
                    let mut local = 0u64;
                    loop {
                        let a = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if a >= p {
                            break;
                        }
                        local += count_a(a);
                    }
                    total.fetch_add(local, std::sync::atomic::Ordering::Relaxed);
                });
            }
        });
        total.into_inner()
    }

    /// `#J(F_p)`, given a prime `r` known to divide it: `L(T)`'s first
    /// `g − 1` coefficients from `N_1 … N_{g−1}`, then the multiples of `r`
    /// in the Hasse–Weil window for the last one that kill 32 random
    /// divisors; exactly one must survive.  `seed` drives the test divisors.
    pub fn jacobian_order(&self, r: u64, seed: u64) -> Result<u64, String> {
        let g = self.genus as usize;
        let p = i128::from(self.k.p);
        if r < 2 || !crate::cryptanalysis::residual_walk::is_prime_u64(r) {
            return Err(format!("r = {r} is not prime"));
        }
        let s: Vec<i128> = (1..g as u32)
            .map(|k| self.count_points(k) as i128 - p.pow(k) - 1)
            .collect();
        let mut c = vec![0i128; g];
        c[0] = 1;
        for k in 1..g {
            let num: i128 = (1..=k).map(|i| s[i - 1] * c[k - i]).sum();
            if num % k as i128 != 0 {
                return Err("point counts violate Newton integrality".into());
            }
            c[k] = num / k as i128;
        }
        let base: i128 =
            c.iter().sum::<i128>() + (1..=g).map(|j| p.pow(j as u32) * c[g - j]).sum::<i128>();
        let binom: i128 = (0..g as i128).fold(1, |acc, i| acc * (2 * g as i128 - i) / (i + 1));
        let bound = (binom as f64 * (p as f64).powf(g as f64 / 2.0)).ceil() as i128 + 1;
        let sp = (p as f64).sqrt();
        let lo = ((sp - 1.0).powi(2 * g as i32) * (1.0 - 1e-9))
            .floor()
            .max(1.0) as i128;
        let hi = ((sp + 1.0).powi(2 * g as i32) * (1.0 + 1e-9)).ceil() as i128;
        let from = (base - bound).max(lo).max(1);
        let to = (base + bound).min(hi).min(i128::from(u64::MAX));
        let r = i128::from(r);
        let mut state = seed;
        let tests: Vec<H64Div> = (0..32).map(|_| self.random_divisor(&mut state)).collect();
        let mut survivors = Vec::new();
        let mut n = (from + r - 1) / r * r;
        while n <= to {
            if tests
                .iter()
                .all(|d| self.mul_uncounted(*d, n as u64).is_identity())
            {
                survivors.push(n as u64);
            }
            n += r;
        }
        match survivors.as_slice() {
            [one] => Ok(*one),
            [] => Err("no multiple of r in the L-polynomial window kills the test divisors".into()),
            many => Err(format!(
                "#J is ambiguous: {} candidates ({:?}…) kill all 32 test divisors",
                many.len(),
                &many[..many.len().min(4)]
            )),
        }
    }
}

impl CountedGroup for H64Curve {
    type Elt = H64Div;
    fn identity(&self) -> H64Div {
        H64Div::IDENTITY
    }
    fn is_identity(&self, d: &H64Div) -> bool {
        d.is_identity()
    }
    fn add(&self, ops: &mut GroupOps, a: H64Div, b: H64Div) -> H64Div {
        if a == b {
            ops.doubles += 1;
        } else {
            ops.adds += 1;
        }
        self.cantor(a, b)
    }
    fn double(&self, ops: &mut GroupOps, a: H64Div) -> H64Div {
        ops.doubles += 1;
        self.cantor(a, a)
    }
    fn neg(&self, a: H64Div) -> H64Div {
        self.negate(a)
    }
    fn key(&self, d: &H64Div) -> u64 {
        self.key_of(d)
    }
}

// ── The cover tower on this group (copied from `tower.rs`, `HCurve` → `H64Curve`) ──

fn tpadd(k: &Fp, a: &[u64], b: &[u64]) -> Vec<u64> {
    let n = a.len().max(b.len());
    (0..n)
        .map(|i| k.add(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0)))
        .collect()
}

fn tpmul(k: &Fp, a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut c = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            c[i + j] = k.add(c[i + j], k.mul(x, y));
        }
    }
    c
}

fn tptrim(mut a: Vec<u64>) -> Vec<u64> {
    while a.len() > 1 && *a.last().unwrap() == 0 {
        a.pop();
    }
    a
}

/// A degree-2 cover `C → E` of genus 2 or 3 (see `tower.rs`) on [`H64Curve`].
#[derive(Clone)]
pub struct Cover64 {
    pub p: u64,
    pub genus: u32,
    pub a: u64,
    /// Model cubic `c(s)`, lowest degree first.
    pub c: [u64; 4],
    pub curve: H64Curve,
    k: Fp,
}

impl Cover64 {
    /// The cover for model `c` and parameter `a`; `Err` when the curve is
    /// singular or the genus is not 2 or 3 (as `Cover::new`).
    pub fn new(p: u64, c: [u64; 4], a: u64) -> Result<Cover64, String> {
        let k = Fp::new(p).ok_or("p is not an odd prime below 2^31")?;
        let c = c.map(|v| v % p);
        let a = a % p;
        if a == 0 {
            return Err("a must be nonzero".into());
        }
        let genus = if c[0] != 0 {
            3
        } else if c[1] != 0 {
            2
        } else {
            return Err("c0 = c1 = 0: not a cover of genus 2 or 3".into());
        };
        let e = [0u64, c[3], c[2], c[1], c[0]];
        let ypoly = vec![k.neg(k.mul(a, a)), 0, 1];
        let mut f = vec![0u64];
        let mut ypow = vec![1u64];
        for &em in e.iter().skip(1) {
            ypow = tpmul(&k, &ypow, &ypoly);
            f = tpadd(&k, &f, &tpmul(&k, &ypow, &[em]));
        }
        let f = tptrim(f);
        let n = 2 * genus as usize + 2;
        if f.len() != n + 1 {
            return Err(format!("deg f = {} is not 2g + 2 = {n}", f.len() - 1));
        }
        // Odd model: g(t) = t^n f(a + 1/t) = Σ_j f_j (a t + 1)^j t^{n−j}.
        let mut g = vec![0u64; n + 1];
        let lin = [1u64, a];
        let mut powj = vec![1u64];
        for (j, &fj) in f.iter().enumerate() {
            if j > 0 {
                powj = tpmul(&k, &powj, &lin);
            }
            for (i, &v) in powj.iter().enumerate() {
                let d = i + n - j;
                g[d] = k.add(g[d], k.mul(fj, v));
            }
        }
        if g[n] != 0 {
            return Err("(a, 0) is not on C".into());
        }
        let curve = H64Curve::new(p, genus, &g[..n])?;
        Ok(Cover64 {
            p,
            genus,
            a,
            c,
            curve,
            k,
        })
    }

    /// `[P − ∞]` for an affine point `(x, y)` of `C` (original coordinates).
    fn place(&self, x: u64, y: u64) -> Option<H64Div> {
        let k = &self.k;
        let dx = k.sub(x % self.p, self.a);
        if dx == 0 {
            return None;
        }
        let t = k.inv(dx);
        let yc = k.mul(y % self.p, k.pow(t, u64::from(self.genus) + 1));
        self.curve.point(t, yc)
    }

    /// `φ^*([Q] − [O])` for a model point `Q = (s, Y)`, when both preimages
    /// are `F_p`-rational and distinct; `None` otherwise.  The two Cantor
    /// additions it performs are charged to `ops`.
    pub fn pull_counted(&self, s: u64, yq: u64, ops: &mut GroupOps) -> Option<H64Div> {
        let k = &self.k;
        if s.is_multiple_of(self.p) {
            return None;
        }
        let x2 = k.add(k.mul(self.a, self.a), k.inv(s));
        if x2 == 0 {
            return None;
        }
        let r = k.sqrt(x2)?;
        let y = k.mul(yq, k.inv(k.mul(s, s)));
        let p1 = self.place(r, y)?;
        let p2 = self.place(k.neg(r), y)?;
        let w = self.place(k.neg(self.a), 0)?;
        let d = CountedGroup::add(&self.curve, ops, p1, p2);
        Some(CountedGroup::add(&self.curve, ops, d, w))
    }

    pub fn pull(&self, s: u64, yq: u64) -> Option<H64Div> {
        self.pull_counted(s, yq, &mut GroupOps::default())
    }
}

/// One rung: a cover with `D = φ^*(G)`, `T = [x]D` and `#J`.
pub struct Rung64 {
    pub cover: Cover64,
    pub sigma: u64,
    pub d: H64Div,
    pub t: H64Div,
    pub jac_order: u64,
    pub hom_checks: u32,
    /// Cantor operations of pulling `G` and `T` up (the one-time transfer).
    pub pull_ops: u64,
    /// Whether `φ^*(T)` is `F_p`-rational and equals `[x]D` (`None`: not rational).
    pub target_pull_matches: Option<bool>,
}

/// One target curve with its genus-2 and genus-3 covers over `F_p`.
pub struct Tower64 {
    pub p: u64,
    pub e: Curve,
    pub r: u64,
    pub x: u64,
    pub gen: Pt,
    pub target: Pt,
    pub rungs: Vec<Rung64>,
    /// Wall seconds to build the tower and pull the instance up (one-time).
    pub transfer_wall_s: f64,
}

fn largest_cyclic_odd_prime(mut n: u64) -> u64 {
    let mut best = 1;
    let mut d = 2;
    while d * d <= n {
        if n.is_multiple_of(d) {
            let mut e = 0;
            while n.is_multiple_of(d) {
                n /= d;
                e += 1;
            }
            if e == 1 && d > 2 {
                best = best.max(d);
            }
        }
        d += 1;
    }
    if n > 2 {
        best = best.max(n);
    }
    best
}

fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn pull_pt(rg: &Cover64, sigma: u64, pt: &Pt, ops: &mut GroupOps) -> Option<H64Div> {
    let (x, y) = (*pt)?;
    rg.pull_counted(rg.k.sub(x.a, sigma), y.a, ops)
}

/// Build a tower over `F_p` from a seed: `tower::build` with `H64Curve`.
/// `min_r` is a floor on the prime `r` (11 reproduces `tower::build`);
/// `cover_attempts` caps the `a` values tried per rung (`p − 1` reproduces
/// it).  `None` when no instance is found.
pub fn build64(p: u64, seed: u64, min_r: u64, cover_attempts: u64) -> Option<Tower64> {
    let t0 = std::time::Instant::now();
    let k = Fp::new(p)?;
    for attempt in 0..4000u64 {
        let h = mix(seed.wrapping_mul(1_000_003).wrapping_add(attempt));
        let (a2, a4, a6) = (h % p, (h >> 16) % p, (h >> 32) % p);
        let Some(e) = Curve::new(k, a2, a4, a6) else {
            continue;
        };
        let roots = e.rational_two_torsion();
        if roots.is_empty() {
            continue;
        }
        let n = e.order();
        let r = largest_cyclic_odd_prime(n);
        if r < min_r.max(11) {
            continue;
        }
        let c = [a6 % p, a4 % p, a2 % p, 1];
        let rho = roots[0].unwrap().0.a;
        let sigma3 = (0..p).find(|&s| super::tower::translate(&k, c, s)[0] != 0)?;
        let mut rungs = vec![];
        for (sigma, want) in [(rho, 2u32), (sigma3, 3u32)] {
            let model = super::tower::translate(&k, c, sigma);
            let mut found = None;
            for ai in 1..p.min(cover_attempts.saturating_add(1)) {
                let a = (ai + (h >> 48)) % p;
                let Ok(cov) = Cover64::new(p, model, a) else {
                    continue;
                };
                if cov.genus != want {
                    continue;
                }
                let Ok(jo) = cov.curve.jacobian_order(r, seed ^ ai) else {
                    continue;
                };
                if !jo.is_multiple_of(r) || (jo / r).is_multiple_of(r) {
                    continue;
                }
                found = Some((cov, jo));
                break;
            }
            let (cover, jac_order) = found?;
            rungs.push(Rung64 {
                cover,
                sigma,
                d: H64Div::IDENTITY,
                t: H64Div::IDENTITY,
                jac_order,
                hom_checks: 0,
                pull_ops: 0,
                target_pull_matches: None,
            });
        }
        if rungs.len() != 2 {
            continue;
        }
        // A generator of E[r] whose image pulls back rationally on both rungs.
        let cof = (n / r) as u128;
        let mut chosen = None;
        for s in 1..3000u64 {
            let g = e.mul(&e.point_from_seed(mix(seed ^ s)), cof);
            if g.is_none() || e.mul(&g, r as u128).is_some() {
                continue;
            }
            let mut scratch = GroupOps::default();
            let ds: Vec<Option<H64Div>> = rungs
                .iter()
                .map(|rg| pull_pt(&rg.cover, rg.sigma, &g, &mut scratch))
                .collect();
            if ds.iter().all(|d| d.is_some()) {
                chosen = Some((g, ds));
                break;
            }
        }
        let (gen, ds) = chosen?;
        let x = 1 + mix(seed ^ 0x5151) % (r - 1);
        let target = e.mul(&gen, x as u128);
        for (rg, d) in rungs.iter_mut().zip(ds) {
            let d = d.unwrap();
            let cur = &rg.cover.curve;
            if d.is_identity() || !cur.mul_uncounted(d, r).is_identity() {
                return None;
            }
            // Homomorphism: φ^*([k]P) = [k]φ^*(P) wherever the left side is defined.
            let mut checks = 0;
            for kk in 2..r.min(80) {
                let q = e.mul(&gen, kk as u128);
                if let Some(dq) = pull_pt(&rg.cover, rg.sigma, &q, &mut GroupOps::default()) {
                    assert_eq!(dq, cur.mul_uncounted(d, kk), "φ^* is a homomorphism");
                    checks += 1;
                }
            }
            rg.d = d;
            rg.t = cur.mul_uncounted(d, x);
            rg.hom_checks = checks;
            // The one-time pull-up of G and T, charged separately.
            let mut ops = GroupOps::default();
            let _ = pull_pt(&rg.cover, rg.sigma, &gen, &mut ops);
            let pt = pull_pt(&rg.cover, rg.sigma, &target, &mut ops);
            rg.pull_ops = ops.adds + ops.doubles;
            rg.target_pull_matches = pt.map(|t| t == rg.t);
        }
        return Some(Tower64 {
            p,
            e,
            r,
            x,
            gen,
            target,
            rungs,
            transfer_wall_s: t0.elapsed().as_secs_f64(),
        });
    }
    None
}

/// Index calculus on [`H64Curve`]: `ecbench::hyper_ic::solve` ported line for
/// line (copied from `src/cryptanalysis/ecbench/hyper_ic.rs`, `H64Curve` →
/// `H64Curve`, `H64Div` → `H64Div`; the only changes are the types, a `u64`
/// product in `mulmod` when the modulus is below `2^32`, and the wall-clock
/// bookkeeping dropped from the outcome).  Same random stream, same walk, same
/// counters: on `p < 256` the counts equal `hyper_ic::solve`'s.
pub mod ic {
    use std::collections::BTreeMap;

    use rand::rngs::StdRng;
    use rand::{RngCore, SeedableRng};

    use super::{H64Curve, H64Div};
    use crate::cryptanalysis::ecbench::hyper_ic::{HyperIcParams, ROUNDS};
    use crate::cryptanalysis::ic_boundary::{CountedGroup, GroupOps};

    /// What a run did, by phase (counts only).
    #[derive(Clone, Debug, Default)]
    pub struct Ic64Outcome {
        pub recovered: Option<u64>,
        pub exhausted: bool,
        /// `factor_base`, `walk_setup`, `relations`, `linear_algebra`,
        /// `verification`: ledger and native counters of each.
        pub phases: Vec<(&'static str, GroupOps, BTreeMap<String, u64>)>,
        /// Shape descriptors and every `*_uncharged` counter.
        pub counters: BTreeMap<String, u64>,
    }

    impl Ic64Outcome {
        /// Cantor additions plus doublings over every phase (the `IC` column).
        pub fn group_ops(&self) -> u64 {
            self.phases.iter().map(|(_, g, _)| g.adds + g.doubles).sum()
        }
        /// The `linalg_mulmods*` counters.
        pub fn la_mulmods(&self) -> u64 {
            self.counters
                .iter()
                .filter(|(k, _)| k.starts_with("linalg_mulmods"))
                .map(|(_, v)| *v)
                .sum()
        }
        pub fn counter(&self, name: &str) -> u64 {
            self.counters.get(name).copied().unwrap_or(0)
        }
    }

    // ── F_p, counted ───────────────────────────────────────────────────────

    /// `F_p` for `p < 256`, every multiplication counted into the tally the
    /// caller passes.
    struct Fp {
        p: u64,
        /// `p − 1 = q·2^s`, `q` odd.
        q: u64,
        s: u32,
        /// A quadratic non-residue, for Tonelli–Shanks.
        z: u64,
    }

    impl Fp {
        fn new(p: u64, c: &mut u64) -> Fp {
            let mut q = p - 1;
            let mut s = 0;
            while q.is_multiple_of(2) {
                q /= 2;
                s += 1;
            }
            let mut f = Fp { p, q, s, z: 0 };
            let mut z = 2;
            while f.pow(z, (p - 1) / 2, c) != p - 1 {
                z += 1;
            }
            f.z = z;
            f
        }

        fn mul(&self, a: u64, b: u64, c: &mut u64) -> u64 {
            *c += 1;
            a * b % self.p
        }

        fn add(&self, a: u64, b: u64) -> u64 {
            (a + b) % self.p
        }

        fn sub(&self, a: u64, b: u64) -> u64 {
            (a + self.p - b) % self.p
        }

        fn neg(&self, a: u64) -> u64 {
            (self.p - a) % self.p
        }

        /// `a^e` by right-to-left square-and-multiply.
        fn pow(&self, a: u64, mut e: u64, c: &mut u64) -> u64 {
            let mut result = 1;
            let mut base = a % self.p;
            while e > 0 {
                if e & 1 == 1 {
                    result = self.mul(result, base, c);
                }
                e >>= 1;
                if e > 0 {
                    base = self.mul(base, base, c);
                }
            }
            result
        }

        /// `a⁻¹` by Fermat, `a ≠ 0`.
        fn inv(&self, a: u64, c: &mut u64) -> u64 {
            debug_assert!(!a.is_multiple_of(self.p));
            self.pow(a, self.p - 2, c)
        }

        /// A square root of `a`, or `None` for a non-residue: Euler's
        /// criterion, then Tonelli–Shanks.
        fn sqrt(&self, a: u64, c: &mut u64) -> Option<u64> {
            let a = a % self.p;
            if a == 0 {
                return Some(0);
            }
            if self.pow(a, (self.p - 1) / 2, c) != 1 {
                return None;
            }
            let mut m = self.s;
            let mut cc = self.pow(self.z, self.q, c);
            let mut t = self.pow(a, self.q, c);
            let mut r = self.pow(a, self.q.div_ceil(2), c);
            while t != 1 {
                let mut i = 0;
                let mut t2 = t;
                while t2 != 1 {
                    t2 = self.mul(t2, t2, c);
                    i += 1;
                }
                let mut b = cc;
                for _ in 0..m - i - 1 {
                    b = self.mul(b, b, c);
                }
                m = i;
                cc = self.mul(b, b, c);
                t = self.mul(t, cc, c);
                r = self.mul(r, b, c);
            }
            Some(r)
        }

        // ── Polynomials: lowest degree first, no trailing zeros ────────────

        fn trim(a: &mut Vec<u64>) {
            while a.last() == Some(&0) {
                a.pop();
            }
        }

        fn deg(a: &[u64]) -> usize {
            a.len().saturating_sub(1)
        }

        fn pmul(&self, a: &[u64], b: &[u64], c: &mut u64) -> Vec<u64> {
            if a.is_empty() || b.is_empty() {
                return Vec::new();
            }
            let mut out = vec![0u64; a.len() + b.len() - 1];
            for (i, &x) in a.iter().enumerate() {
                for (j, &y) in b.iter().enumerate() {
                    out[i + j] = self.add(out[i + j], self.mul(x, y, c));
                }
            }
            Self::trim(&mut out);
            out
        }

        /// `(a div b, a mod b)`, `b ≠ 0`.  A monic divisor needs no inversion
        /// and no multiplication for the quotient digit.
        fn pdivrem(&self, a: &[u64], b: &[u64], c: &mut u64) -> (Vec<u64>, Vec<u64>) {
            debug_assert!(!b.is_empty());
            let mut r = a.to_vec();
            Self::trim(&mut r);
            if r.len() < b.len() {
                return (Vec::new(), r);
            }
            let db = Self::deg(b);
            let lead = b[db];
            let inv_lead = if lead == 1 { 1 } else { self.inv(lead, c) };
            let mut q = vec![0u64; r.len() - db];
            while !r.is_empty() && Self::deg(&r) >= db {
                let dr = Self::deg(&r);
                let top = r[dr];
                let digit = if inv_lead == 1 {
                    top
                } else {
                    self.mul(top, inv_lead, c)
                };
                q[dr - db] = digit;
                for i in 0..db {
                    let t = self.mul(digit, b[i], c);
                    r[dr - db + i] = self.sub(r[dr - db + i], t);
                }
                r[dr] = 0;
                Self::trim(&mut r);
            }
            Self::trim(&mut q);
            (q, r)
        }

        fn prem(&self, a: &[u64], b: &[u64], c: &mut u64) -> Vec<u64> {
            self.pdivrem(a, b, c).1
        }

        fn monic(&self, mut a: Vec<u64>, c: &mut u64) -> Vec<u64> {
            if let Some(&lead) = a.last() {
                if lead != 1 {
                    let inv = self.inv(lead, c);
                    let n = a.len();
                    for x in a.iter_mut().take(n - 1) {
                        *x = self.mul(*x, inv, c);
                    }
                    a[n - 1] = 1;
                }
            }
            a
        }

        /// Monic `gcd(a, b)`.
        fn pgcd(&self, a: &[u64], b: &[u64], c: &mut u64) -> Vec<u64> {
            let (mut a, mut b) = (a.to_vec(), b.to_vec());
            Self::trim(&mut a);
            Self::trim(&mut b);
            while !b.is_empty() {
                let r = self.prem(&a, &b, c);
                a = b;
                b = r;
            }
            self.monic(a, c)
        }

        /// `base^e mod m`, `m` monic of degree ≥ 1.
        fn ppowmod(&self, base: &[u64], mut e: u64, m: &[u64], c: &mut u64) -> Vec<u64> {
            let mut result = self.prem(&[1], m, c);
            let mut acc = self.prem(base, m, c);
            while e > 0 {
                if e & 1 == 1 {
                    result = self.prem(&self.pmul(&result, &acc, c), m, c);
                }
                e >>= 1;
                if e > 0 {
                    acc = self.prem(&self.pmul(&acc, &acc, c), m, c);
                }
            }
            result
        }

        /// Horner evaluation.
        fn eval(&self, a: &[u64], x: u64, c: &mut u64) -> u64 {
            let mut acc = 0;
            for (i, &coef) in a.iter().enumerate().rev() {
                acc = if i + 1 == a.len() {
                    coef
                } else {
                    self.add(self.mul(acc, x, c), coef)
                };
            }
            acc
        }

        /// Roots of a monic quadratic `x² + a₁x + a₀`, sorted, with
        /// multiplicity; `None` when it is irreducible.
        fn quadratic_roots(&self, u: &[u64], c: &mut u64) -> Option<Vec<(u64, usize)>> {
            let (a0, a1) = (u[0], u[1]);
            let four_a0 = self.add(self.add(a0, a0), self.add(a0, a0));
            let disc = self.sub(self.mul(a1, a1, c), four_a0);
            let inv2 = self.p.div_ceil(2);
            let neg_a1 = self.neg(a1);
            if disc == 0 {
                return Some(vec![(self.mul(neg_a1, inv2, c), 2)]);
            }
            let s = self.sqrt(disc, c)?;
            let r1 = self.mul(self.add(neg_a1, s), inv2, c);
            let r2 = self.mul(self.sub(neg_a1, s), inv2, c);
            Some(if r1 < r2 {
                vec![(r1, 1), (r2, 1)]
            } else {
                vec![(r2, 1), (r1, 1)]
            })
        }

        /// Roots of a monic, squarefree, completely split `l`, sorted.
        fn split_roots(&self, l: &[u64], c: &mut u64) -> Vec<u64> {
            let mut out = match Self::deg(l) {
                0 => Vec::new(),
                1 => vec![self.neg(l[0])],
                2 => self
                    .quadratic_roots(l, c)
                    .expect("a split quadratic has roots")
                    .into_iter()
                    .map(|(x, _)| x)
                    .collect(),
                _ => {
                    // Cantor–Zassenhaus with the shifts b = 0, 1, 2, …: the
                    // gcd with (x + b)^((p−1)/2) − 1 collects the roots whose
                    // shift is a non-zero square.
                    let half = (self.p - 1) / 2;
                    let mut found = None;
                    for b in 0..self.p {
                        let mut h = self.ppowmod(&[b, 1], half, l, c);
                        if h.is_empty() {
                            h.push(0);
                        }
                        h[0] = self.sub(h[0], 1);
                        Self::trim(&mut h);
                        if h.is_empty() {
                            continue;
                        }
                        let g = self.pgcd(l, &h, c);
                        let dg = Self::deg(&g);
                        if dg > 0 && dg < Self::deg(l) {
                            found = Some(g);
                            break;
                        }
                    }
                    match found {
                        Some(g) => {
                            let (other, _) = self.pdivrem(l, &g, c);
                            let mut roots = self.split_roots(&g, c);
                            roots.extend(self.split_roots(&other, c));
                            roots
                        }
                        // Not reached for p ≥ 5; scan rather than fail.
                        None => (0..self.p).filter(|&x| self.eval(l, x, c) == 0).collect(),
                    }
                }
            };
            out.sort_unstable();
            out
        }

        /// Split the monic `u` (`1 ≤ deg u`) into linear factors over `F_p`:
        /// `(root, multiplicity)` sorted by root, or `None` when some
        /// irreducible factor has degree ≥ 2.
        fn split(&self, u: &[u64], c: &mut u64) -> Option<Vec<(u64, usize)>> {
            match Self::deg(u) {
                0 => return Some(Vec::new()),
                1 => return Some(vec![(self.neg(u[0]), 1)]),
                2 => return self.quadratic_roots(u, c),
                _ => {}
            }
            let xp = self.ppowmod(&[0, 1], self.p, u, c);
            let mut xp_minus_x = xp;
            xp_minus_x.resize(xp_minus_x.len().max(2), 0);
            xp_minus_x[1] = self.sub(xp_minus_x[1], 1);
            Self::trim(&mut xp_minus_x);
            // L: the product of u's distinct linear factors.
            let l = self.pgcd(u, &xp_minus_x, c);
            if Self::deg(&l) == 0 {
                return None;
            }
            if Self::deg(&l) < Self::deg(u) {
                // Smooth iff every factor of u / L divides a power of L.
                let (mut q, _) = self.pdivrem(u, &l, c);
                loop {
                    let h = self.pgcd(&q, &l, c);
                    if Self::deg(&h) == 0 {
                        break;
                    }
                    q = self.pdivrem(&q, &h, c).0;
                }
                if Self::deg(&q) > 0 {
                    return None;
                }
            }
            let roots = self.split_roots(&l, c);
            // Multiplicities by synthetic division.
            let mut out = Vec::with_capacity(roots.len());
            let mut rest = u.to_vec();
            for x in roots {
                let lin = [self.neg(x), 1];
                let mut mult = 0;
                loop {
                    let (q, r) = self.pdivrem(&rest, &lin, c);
                    if !r.is_empty() {
                        break;
                    }
                    rest = q;
                    mult += 1;
                }
                out.push((x, mult));
            }
            debug_assert_eq!(rest, vec![1]);
            Some(out)
        }
    }

    // ── Relations ──────────────────────────────────────────────────────────

    /// `a·D₁ + b·D₂ = Σ cⱼ Fⱼ` (the walk's steps already subtracted).
    #[derive(Clone, Debug)]
    struct Relation {
        a: u64,
        b: u64,
        entries: Vec<(usize, i64)>,
    }

    enum Decomposition {
        Smooth(Vec<(usize, i64)>),
        OneLargePrime {
            entries: Vec<(usize, i64)>,
            x: u64,
            coef: i64,
        },
        SmoothOffBase,
        NotSmooth,
    }

    /// The exact work of the relation phase, by counter.
    #[derive(Default)]
    struct Tally {
        oracle_fp_muls: u64,
        factor_base_lookups: u64,
        canonicalisations: u64,
        step_draws: u64,
        large_prime_mulmods: u64,
        large_prime_lookups: u64,
        linalg_mulmods: u64,
        linalg_inversions: u64,
    }

    fn mulmod(a: u64, b: u64, n: u64) -> u64 {
        if n < 1 << 32 && a < 1 << 32 && b < 1 << 32 {
            a * b % n
        } else {
            (u128::from(a) * u128::from(b) % u128::from(n)) as u64
        }
    }

    fn signed_mod(v: i64, n: u64) -> u64 {
        let m = v.unsigned_abs() % n;
        if v >= 0 {
            m
        } else {
            (n - m) % n
        }
    }

    /// `a⁻¹ mod n` by the extended Euclidean algorithm, `n` prime.
    fn inv_mod(a: u64, n: u64) -> Option<u64> {
        let (mut r0, mut r1) = (i128::from(n), i128::from(a % n));
        let (mut t0, mut t1) = (0i128, 1i128);
        while r1 != 0 {
            let q = r0 / r1;
            (r0, r1) = (r1, r0 - q * r1);
            (t0, t1) = (t1, t0 - q * t1);
        }
        (r0 == 1).then(|| t0.rem_euclid(i128::from(n)) as u64)
    }

    /// The thread's `combine_partials`: `e₂·r₁ − e₁·r₂` eliminates the shared
    /// off-base place.
    fn combine(
        r1: &Relation,
        e1: i64,
        r2: &Relation,
        e2: i64,
        n: u64,
        t: &mut Tally,
    ) -> Option<Relation> {
        let mut scale = |v: u64, k: i64| {
            t.large_prime_mulmods += 1;
            let s = mulmod(v, k.unsigned_abs() % n, n);
            if k < 0 {
                (n - s) % n
            } else {
                s
            }
        };
        let a = (scale(r1.a, e2) + scale(r2.a, -e1)) % n;
        let b = (scale(r1.b, e2) + scale(r2.b, -e1)) % n;
        let mut acc: BTreeMap<usize, i64> = BTreeMap::new();
        for &(j, c) in &r1.entries {
            *acc.entry(j).or_insert(0) += e2 * c;
        }
        for &(j, c) in &r2.entries {
            *acc.entry(j).or_insert(0) -= e1 * c;
        }
        let entries: Vec<(usize, i64)> = acc.into_iter().filter(|&(_, c)| c != 0).collect();
        if entries.is_empty() && a == 0 && b == 0 {
            return None;
        }
        Some(Relation { a, b, entries })
    }

    /// The thread's Markowitz sparse elimination mod `n`, on words: eliminate
    /// every factor-base column and read `k` (column `m`) off a surviving row.
    fn sparse_solve_for_k(
        mut rows: Vec<Vec<(usize, u64)>>,
        mut rhs: Vec<u64>,
        m: usize,
        n: u64,
        t: &mut Tally,
    ) -> Option<u64> {
        let mut col_count = vec![0usize; m + 1];
        let mut holders: Vec<Vec<usize>> = vec![Vec::new(); m + 1];
        for (i, row) in rows.iter().enumerate() {
            for &(j, _) in row {
                col_count[j] += 1;
                holders[j].push(i);
            }
        }
        let mut eliminated = vec![false; rows.len()];
        let has = |row: &[(usize, u64)], col: usize| row.binary_search_by_key(&col, |&(j, _)| j);

        while let Some(col) = (0..m)
            .filter(|&j| col_count[j] > 0)
            .min_by_key(|&j| col_count[j])
        {
            let mut live = std::mem::take(&mut holders[col]);
            live.retain(|&i| !eliminated[i] && has(&rows[i], col).is_ok());
            live.sort_unstable();
            live.dedup();
            debug_assert_eq!(live.len(), col_count[col]);
            let pivot = *live.iter().min_by_key(|&&i| rows[i].len())?;

            let at = has(&rows[pivot], col).ok()?;
            t.linalg_inversions += 1;
            let inv = inv_mod(rows[pivot][at].1, n)?;
            for (_, v) in rows[pivot].iter_mut() {
                *v = mulmod(*v, inv, n);
                t.linalg_mulmods += 1;
            }
            rhs[pivot] = mulmod(rhs[pivot], inv, n);
            t.linalg_mulmods += 1;
            eliminated[pivot] = true;
            let pivot_row = std::mem::take(&mut rows[pivot]);
            let pivot_rhs = rhs[pivot];
            for &(j, _) in &pivot_row {
                col_count[j] -= 1;
            }

            for i in live {
                if i == pivot {
                    continue;
                }
                let old = std::mem::take(&mut rows[i]);
                let factor = old[has(&old, col).ok()?].1;
                let mut merged: Vec<(usize, u64)> = Vec::with_capacity(old.len() + pivot_row.len());
                let (mut a, mut b) = (0, 0);
                while a < old.len() || b < pivot_row.len() {
                    let ja = old.get(a).map_or(usize::MAX, |e| e.0);
                    let jb = pivot_row.get(b).map_or(usize::MAX, |e| e.0);
                    if ja < jb {
                        merged.push(old[a]);
                        a += 1;
                        continue;
                    }
                    let term = mulmod(factor, pivot_row[b].1, n);
                    t.linalg_mulmods += 1;
                    let v = if ja == jb {
                        let v = (old[a].1 + n - term) % n;
                        a += 1;
                        v
                    } else {
                        (n - term) % n
                    };
                    if v != 0 {
                        if ja != jb {
                            holders[jb].push(i);
                        }
                        merged.push((jb, v));
                    }
                    b += 1;
                }
                for &(j, _) in &old {
                    col_count[j] -= 1;
                }
                for &(j, _) in &merged {
                    col_count[j] += 1;
                }
                rhs[i] = (rhs[i] + n - mulmod(factor, pivot_rhs, n)) % n;
                t.linalg_mulmods += 1;
                rows[i] = merged;
            }
        }

        for (i, row) in rows.iter().enumerate() {
            if eliminated[i] {
                continue;
            }
            if row.len() == 1 && row[0].0 == m {
                t.linalg_inversions += 1;
                let inv = inv_mod(row[0].1, n)?;
                t.linalg_mulmods += 1;
                return Some(mulmod(rhs[i], inv, n));
            }
        }
        None
    }

    /// Rows `Σ cᵢ yᵢ − b·k ≡ a (mod n)` over the unknowns `(y₀ … y_{m−1}, k)`.
    fn solve_relations(relations: &[Relation], m: usize, n: u64, t: &mut Tally) -> Option<u64> {
        let mut rows = Vec::with_capacity(relations.len());
        let mut rhs = Vec::with_capacity(relations.len());
        for rel in relations {
            let mut row: BTreeMap<usize, u64> = BTreeMap::new();
            for &(j, c) in &rel.entries {
                let e = row.entry(j).or_insert(0);
                *e = (*e + signed_mod(c, n)) % n;
            }
            row.insert(m, (n - rel.b % n) % n);
            rows.push(row.into_iter().filter(|&(_, v)| v != 0).collect());
            rhs.push(rel.a % n);
        }
        sparse_solve_for_k(rows, rhs, m, n, t)
    }
    // ── The run ────────────────────────────────────────────────────────────

    /// Solve `target = k·gen` in the subgroup of prime order `r`, under the
    /// algorithm seed `seed`.  `group_order` is `#J`; the run refuses
    /// `r² | #J` (see the module notes).
    pub fn solve(
        curve: &H64Curve,
        gen: H64Div,
        target: H64Div,
        r: u64,
        group_order: u64,
        seed: u64,
        params: &HyperIcParams,
    ) -> Result<Ic64Outcome, String> {
        if !group_order.is_multiple_of(r) || (group_order / r).is_multiple_of(r) {
            return Err(format!(
            "ic.hyperelliptic_thread needs r ∥ #J (a cyclic r-part); r = {r}, #J = {group_order}"
        ));
        }
        if params.branches == 0 || params.d_step_in < 2 {
            return Err("ic.hyperelliptic_thread needs branches ≥ 1 and d_step_in ≥ 2".into());
        }
        let p = curve.p();
        let f = curve.f_coeffs();
        let half = (p - 1) / 2;

        // ── factor_base ──
        let mut fb_muls = 0u64;
        let fp = Fp::new(p, &mut fb_muls);
        let mut columns: Vec<H64Div> = Vec::new();
        let mut col_ys: Vec<u64> = Vec::new();
        let mut col_of_x: Vec<Option<usize>> = vec![None; p as usize];
        for x in 0..p {
            if columns.len() >= params.fb_size {
                break;
            }
            let fx = fp.eval(&f, x, &mut fb_muls);
            let Some(y) = fp.sqrt(fx, &mut fb_muls) else {
                continue;
            };
            let y = if y > half { p - y } else { y };
            let d = curve
                .point(x, y)
                .ok_or("a factor-base place is not on the curve")?;
            col_of_x[x as usize] = Some(columns.len());
            columns.push(d);
            col_ys.push(y);
        }
        let m = columns.len();
        if m == 0 {
            return Err("the factor base is empty".into());
        }

        // ── walk_setup ──
        let mut setup_ops = GroupOps::default();
        let mut tally = Tally::default();
        let mut setup_draws = 0u64;
        let mut rng = StdRng::seed_from_u64(seed);
        let branches = params.branches.min(m).max(1);
        let mut chosen: Vec<usize> = (0..m).collect();
        for i in (1..chosen.len()).rev() {
            setup_draws += 1;
            let j = (rng.next_u64() as usize) % (i + 1);
            chosen.swap(i, j);
        }
        let fb_steps: Vec<usize> = chosen.into_iter().take(branches).collect();

        // ── relations (the start D₁ + D₂ is charged to walk_setup) ──
        let mut rel_ops = GroupOps::default();
        let mut relations: Vec<Relation> = Vec::new();
        let mut partials: BTreeMap<u64, (Relation, i64)> = BTreeMap::new();
        let mut taken = vec![0i64; m];
        let mut touched: Vec<usize> = Vec::new();
        let mut position: Option<(u64, u64, H64Div)> = None;
        let (mut trials, mut smooth, mut partial_count, mut combined, mut off_base) =
            (0u64, 0u64, 0u64, 0u64, 0u64);
        let mut rounds = 0u64;
        let mut extra = params.extra_relations;
        let mut recovered = None;

        let subtract_taken = |entries: Vec<(usize, i64)>, taken: &[i64], touched: &[usize]| {
            let mut acc: BTreeMap<usize, i64> = entries.into_iter().collect();
            for &idx in touched {
                *acc.entry(idx).or_insert(0) -= taken[idx];
            }
            acc.into_iter().filter(|&(_, c)| c != 0).collect::<Vec<_>>()
        };

        for _round in 0..ROUNDS {
            let wanted = m + 1 + extra;
            while relations.len() < wanted && trials < params.max_trials {
                trials += 1;
                let (a, b, d) = match position.take() {
                    None => {
                        let d = curve.add(&mut setup_ops, gen, target);
                        (1 % r, 1 % r, d)
                    }
                    Some((a, b, d)) => {
                        tally.step_draws += 1;
                        let roll = rng.next_u64() % params.d_step_in;
                        if roll == 0 {
                            ((a + 1) % r, b, curve.add(&mut rel_ops, d, gen))
                        } else if roll == 1 {
                            (a, (b + 1) % r, curve.add(&mut rel_ops, d, target))
                        } else {
                            tally.step_draws += 1;
                            let idx = fb_steps[(rng.next_u64() as usize) % fb_steps.len()];
                            if let Err(at) = touched.binary_search(&idx) {
                                touched.insert(at, idx);
                            }
                            taken[idx] += 1;
                            (a, b, curve.add(&mut rel_ops, d, columns[idx]))
                        }
                    }
                };
                position = Some((a, b, d));

                if d.is_identity() {
                    smooth += 1;
                    relations.push(Relation {
                        a,
                        b,
                        entries: subtract_taken(Vec::new(), &taken, &touched),
                    });
                    continue;
                }
                match classify(&fp, &d, &col_of_x, &col_ys, half, &mut tally) {
                    Decomposition::Smooth(entries) => {
                        smooth += 1;
                        relations.push(Relation {
                            a,
                            b,
                            entries: subtract_taken(entries, &taken, &touched),
                        });
                    }
                    Decomposition::OneLargePrime { entries, x, coef } if params.large_primes => {
                        smooth += 1;
                        partial_count += 1;
                        let rel = Relation {
                            a,
                            b,
                            entries: subtract_taken(entries, &taken, &touched),
                        };
                        tally.large_prime_lookups += 1;
                        match partials.get(&x) {
                            None => {
                                partials.insert(x, (rel, coef));
                            }
                            Some((first, first_coef)) => {
                                if let Some(full) =
                                    combine(first, *first_coef, &rel, coef, r, &mut tally)
                                {
                                    combined += 1;
                                    relations.push(full);
                                }
                            }
                        }
                    }
                    Decomposition::OneLargePrime { .. } | Decomposition::SmoothOffBase => {
                        smooth += 1;
                        off_base += 1;
                    }
                    Decomposition::NotSmooth => {}
                }
            }
            if relations.len() < m + 1 {
                break;
            }
            rounds += 1;
            let k = solve_relations(&relations, m, r, &mut tally);
            if let Some(k) = k {
                recovered = Some(k);
                break;
            }
            if trials >= params.max_trials {
                break;
            }
            extra += m / 4 + 8;
        }

        // ── verification: [k]D₁ = D₂, one counted scalar multiplication ──
        let mut ver_ops = GroupOps::default();
        if let Some(k) = recovered {
            let _ = curve.mul(&mut ver_ops, gen, k);
        }

        let named = |pairs: &[(&str, u64)]| -> BTreeMap<String, u64> {
            pairs.iter().map(|&(k, v)| (k.to_string(), v)).collect()
        };
        let fb_native = named(&[("factor_base_fp_muls_uncharged", fb_muls)]);
        let setup_native = named(&[("step_draws_uncharged", setup_draws)]);
        let mut rel_native = named(&[
            ("oracle_fp_muls_uncharged", tally.oracle_fp_muls),
            ("factor_base_lookups_uncharged", tally.factor_base_lookups),
            ("canonicalisations_uncharged", tally.canonicalisations),
            ("step_draws_uncharged", tally.step_draws),
        ]);
        if params.large_primes {
            rel_native.insert(
                "large_prime_mulmods_uncharged".into(),
                tally.large_prime_mulmods,
            );
            rel_native.insert(
                "large_prime_lookups_uncharged".into(),
                tally.large_prime_lookups,
            );
        }
        let la_native = named(&[
            ("linalg_mulmods_uncharged", tally.linalg_mulmods),
            ("linalg_inversions_uncharged", tally.linalg_inversions),
        ]);

        let mut counters: BTreeMap<String, u64> = BTreeMap::new();
        for native in [&fb_native, &setup_native, &rel_native, &la_native] {
            for (k, v) in native {
                *counters.entry(k.clone()).or_insert(0) += v;
            }
        }
        for (k, v) in [
            ("factor_base_columns", m as u64),
            ("walk_branches", branches as u64),
            ("trials", trials),
            ("smooth_trials", smooth),
            ("relations", relations.len() as u64),
            ("partial_relations", partial_count),
            ("combined_relations", combined),
            ("discarded_off_base", off_base),
            ("solve_rounds", rounds),
        ] {
            counters.insert(k.into(), v);
        }

        Ok(Ic64Outcome {
            recovered,
            // No candidate: the trial cap, or ROUNDS solves that left `k` free.
            exhausted: recovered.is_none(),
            phases: vec![
                ("factor_base", GroupOps::default(), fb_native),
                ("walk_setup", setup_ops, setup_native),
                ("relations", rel_ops, rel_native),
                ("linear_algebra", GroupOps::default(), la_native),
                ("verification", ver_ops, BTreeMap::new()),
            ],
            counters,
        })
    }

    /// The thread's `classify_counted_with` on a word-size divisor.
    fn classify(
        fp: &Fp,
        d: &H64Div,
        col_of_x: &[Option<usize>],
        col_ys: &[u64],
        half: u64,
        t: &mut Tally,
    ) -> Decomposition {
        let u = d.u_coeffs();
        let v: Vec<u64> = d.v_coeffs();
        let Some(roots) = fp.split(&u, &mut t.oracle_fp_muls) else {
            return Decomposition::NotSmooth;
        };
        let mut acc: BTreeMap<usize, i64> = BTreeMap::new();
        let mut off: Option<(u64, i64)> = None;
        for (x, mult) in roots {
            let y = fp.eval(&v, x, &mut t.oracle_fp_muls);
            t.canonicalisations += 1;
            let sign: i64 = if y == 0 || y <= half { 1 } else { -1 };
            t.factor_base_lookups += 1;
            match col_of_x[x as usize] {
                Some(idx) => {
                    debug_assert_eq!(col_ys[idx], if sign > 0 { y } else { fp.p - y });
                    *acc.entry(idx).or_insert(0) += sign * mult as i64;
                }
                None => {
                    if off.is_some() {
                        return Decomposition::SmoothOffBase;
                    }
                    off = Some((x, sign * mult as i64));
                }
            }
        }
        let entries: Vec<(usize, i64)> = acc.into_iter().filter(|&(_, c)| c != 0).collect();
        match off {
            None => Decomposition::Smooth(entries),
            Some((x, coef)) => Decomposition::OneLargePrime { entries, x, coef },
        }
    }
}

// ── The cross-check against `HCurve` (the `p < 256` gate) ──

/// Cross-check of this group against `HCurve` on the same tower instances.
pub mod crosscheck {
    use super::*;
    use crate::cryptanalysis::chained_transfer::measure::EcGroup;
    use crate::cryptanalysis::chained_transfer::tower::build;
    use crate::cryptanalysis::ecbench::hyper_ic::{solve as old_solve, HyperIcParams};
    use crate::cryptanalysis::hyperelliptic_word::{HCurve, HDiv};
    use crate::cryptanalysis::ic_boundary::rho_reference;

    /// The index-calculus parameters `tower_bench` and `tower_ladder` use.
    pub fn ic_params() -> HyperIcParams {
        HyperIcParams {
            fb_size: usize::MAX,
            large_primes: false,
            extra_relations: 8,
            branches: 64,
            d_step_in: 4,
            max_trials: 5_000_000,
        }
    }

    /// What one cross-check compared.
    #[derive(Clone, Debug, Default, serde::Serialize)]
    pub struct CrossReport {
        pub p: u64,
        pub seed: u64,
        pub built: bool,
        pub r: u64,
        pub jac_orders: [u64; 2],
        pub field_checks: u64,
        pub divisors_compared: u64,
        pub ic_solves_compared: u64,
        pub rho_runs_compared: u64,
    }

    fn to_new(c: &H64Curve, d: &HDiv) -> Result<H64Div, String> {
        c.from_coeffs(&d.u_coeffs(), &d.v_coeffs())
            .ok_or_else(|| format!("divisor {d:?} is not valid on the new curve"))
    }

    fn to_old(c: &HCurve, d: &H64Div) -> Result<HDiv, String> {
        c.from_coeffs(&d.u_coeffs(), &d.v_coeffs())
            .ok_or_else(|| format!("divisor {d:?} is not valid on the old curve"))
    }

    fn same(old: &HDiv, new: &H64Div) -> bool {
        old.u_coeffs() == new.u_coeffs() && old.v_coeffs() == new.v_coeffs()
    }

    fn ops_eq(a: &GroupOps, b: &GroupOps) -> bool {
        (a.adds, a.doubles, a.scalar_mults) == (b.adds, b.doubles, b.scalar_mults)
    }

    /// Build the instance with `tower::build` and with [`build64`] under one
    /// seed and require: identical `E`, `r`, `x`, `G`, `T`; identical covers
    /// (the odd-model polynomial), `#J`, pulled-up divisors and homomorphism
    /// counts; identical divisors after `n_ops` random operations on each
    /// rung (random divisors drawn on each side and imported to the other);
    /// identical index-calculus runs and rho runs on `ic_seeds` seeds.
    /// `Ok` with `built = false` when neither builds this seed.
    pub fn run(p: u64, seed: u64, n_ops: usize, ic_seeds: u64) -> Result<CrossReport, String> {
        let mut rep = CrossReport {
            p,
            seed,
            ..Default::default()
        };
        let (old, new) = (build(p, seed), build64(p, seed, 11, p.saturating_sub(1)));
        let (old, new) = match (old, new) {
            (None, None) => return Ok(rep),
            (Some(o), Some(n)) => (o, n),
            (o, n) => {
                return Err(format!(
                    "p = {p} seed {seed}: build old = {}, build new = {}",
                    o.is_some(),
                    n.is_some()
                ))
            }
        };
        rep.built = true;
        let mut chk = |ok: bool, what: &str| -> Result<(), String> {
            rep.field_checks += 1;
            if ok {
                Ok(())
            } else {
                Err(format!("p = {p} seed {seed}: mismatch in {what}"))
            }
        };
        chk(
            old.p == new.p && old.r == new.r && old.x == new.x,
            "p, r, x",
        )?;
        chk(old.gen == new.gen && old.target == new.target, "G, T on E")?;
        chk(old.e.order() == new.e.order() && old.e == new.e, "E and #E")?;
        for (i, (ro, rn)) in old.rungs.iter().zip(new.rungs.iter()).enumerate() {
            let (co, cn) = (&ro.cover.curve, &rn.cover.curve);
            chk(ro.cover.genus == rn.cover.genus, "genus")?;
            chk(co.f_coeffs() == cn.f_coeffs(), "odd-model polynomial")?;
            chk(ro.sigma == rn.sigma && ro.cover.a == rn.cover.a, "sigma, a")?;
            chk(ro.jac_order == rn.jac_order, "#J")?;
            chk(ro.hom_checks == rn.hom_checks, "homomorphism checks")?;
            chk(same(&ro.d, &rn.d) && same(&ro.t, &rn.t), "D, T on the rung")?;
            // #J again from a different seed on both sides.
            let jo = co.jacobian_order(old.r, 99).map_err(|e| e.to_string())?;
            let jn = cn.jacobian_order(new.r, 99).map_err(|e| e.to_string())?;
            chk(jo == jn && jo == ro.jac_order, "#J recomputed")?;
            chk(
                co.count_points(1) == cn.count_points(1),
                "point count over F_p",
            )?;
            if ro.cover.genus == 3 {
                chk(
                    co.count_points(2) == cn.count_points(2),
                    "point count over F_p^2",
                )?;
            }
            rep.jac_orders[i] = ro.jac_order;

            // Random operation sequences, divisors drawn on both sides.
            let mut st = mix(seed ^ (p << 8) ^ i as u64);
            let mut pool_o: Vec<HDiv> = vec![];
            let mut pool_n: Vec<H64Div> = vec![];
            for j in 0..8 {
                if j % 2 == 0 {
                    let d = co.random_divisor(&mut st);
                    pool_n.push(to_new(cn, &d)?);
                    pool_o.push(d);
                } else {
                    let d = cn.random_divisor(&mut st);
                    pool_o.push(to_old(co, &d)?);
                    pool_n.push(d);
                }
            }
            pool_o.push(ro.d);
            pool_n.push(rn.d);
            for _ in 0..n_ops {
                st = mix(st);
                let (a, b) = (
                    (st >> 8) as usize % pool_o.len(),
                    (st >> 24) as usize % pool_o.len(),
                );
                let dst = (st >> 40) as usize % pool_o.len();
                let kk = (st >> 48) % 100_000 + 1;
                let (ro_, rn_) = match st % 6 {
                    0 | 1 => (
                        co.cantor(pool_o[a], pool_o[b]),
                        cn.cantor(pool_n[a], pool_n[b]),
                    ),
                    2 => (
                        co.cantor(pool_o[a], pool_o[a]),
                        cn.cantor(pool_n[a], pool_n[a]),
                    ),
                    3 => (co.negate(pool_o[a]), cn.negate(pool_n[a])),
                    4 => (
                        co.mul_uncounted(pool_o[a], kk),
                        cn.mul_uncounted(pool_n[a], kk),
                    ),
                    _ => (
                        co.cantor(pool_o[a], co.negate(pool_o[b])),
                        cn.cantor(pool_n[a], cn.negate(pool_n[b])),
                    ),
                };
                rep.divisors_compared += 1;
                chk(same(&ro_, &rn_), "divisor after an operation")?;
                chk(cn.is_valid(&rn_) && co.is_valid(&ro_), "validity")?;
                chk(co.key_of(&ro_) == cn.key_of(&rn_), "key")?;
                pool_o[dst] = ro_;
                pool_n[dst] = rn_;
            }

            // Index calculus and rho on the rung: identical counts.
            for s in 0..ic_seeds {
                let po = old_solve(co, ro.d, ro.t, old.r, ro.jac_order, 300 + s, &ic_params())?;
                let pn = ic::solve(cn, rn.d, rn.t, new.r, rn.jac_order, 300 + s, &ic_params())?;
                chk(
                    po.recovered == pn.recovered && po.recovered == Some(old.x),
                    "IC recovered",
                )?;
                chk(po.phases.len() == pn.phases.len(), "IC phases")?;
                for (a, b) in po.phases.iter().zip(pn.phases.iter()) {
                    chk(
                        a.0 == b.0 && ops_eq(&a.1, &b.1) && a.2 == b.2,
                        "IC phase ledger",
                    )?;
                }
                chk(po.counters == pn.counters, "IC counters")?;
                rep.ic_solves_compared += 1;
                let rho_o = rho_reference(co, ro.d, ro.t, old.r, 200 + s, 1 << 24);
                let rho_n = rho_reference(cn, rn.d, rn.t, new.r, 200 + s, 1 << 24);
                chk(
                    ops_eq(&rho_o.group_ops, &rho_n.group_ops)
                        && rho_o.steps == rho_n.steps
                        && rho_o.walks == rho_n.walks
                        && rho_o.recovered == rho_n.recovered
                        && rho_n.recovered == Some(old.x),
                    "rho on the rung",
                )?;
                rep.rho_runs_compared += 1;
            }
        }
        // Rho on E: the same code on the same instance, run from both builds.
        for s in 0..ic_seeds {
            let a = rho_reference(
                &EcGroup(old.e),
                old.gen,
                old.target,
                old.r,
                100 + s,
                1 << 24,
            );
            let b = rho_reference(
                &EcGroup(new.e),
                new.gen,
                new.target,
                new.r,
                100 + s,
                1 << 24,
            );
            chk(
                ops_eq(&a.group_ops, &b.group_ops)
                    && a.recovered == b.recovered
                    && b.recovered == Some(new.x),
                "rho on E",
            )?;
            rep.rho_runs_compared += 1;
        }
        Ok(rep)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rungs(p: u64, seed: u64) -> Option<Tower64> {
        build64(p, seed, 11, p - 1)
    }

    #[test]
    fn group_law_on_small_primes() {
        let mut built = 0;
        for (i, p) in [53u64, 101, 151, 199].into_iter().enumerate() {
            let Some(t) = rungs(p, 7 + i as u64) else {
                continue;
            };
            built += 1;
            for rg in &t.rungs {
                let c = &rg.cover.curve;
                let mut st = 0xC0FFEE ^ p;
                for _ in 0..40 {
                    let (a, b, d) = (
                        c.random_divisor(&mut st),
                        c.random_divisor(&mut st),
                        c.random_divisor(&mut st),
                    );
                    assert!(c.is_valid(&a) && c.is_valid(&b) && c.is_valid(&d));
                    // Associativity, commutativity.
                    assert_eq!(
                        c.cantor(c.cantor(a, b), d),
                        c.cantor(a, c.cantor(b, d)),
                        "associativity"
                    );
                    assert_eq!(c.cantor(a, b), c.cantor(b, a), "commutativity");
                    // Identity and inverse.
                    assert_eq!(c.cantor(a, H64Div::IDENTITY), a);
                    assert_eq!(c.cantor(H64Div::IDENTITY, a), a);
                    assert!(c.cantor(a, c.negate(a)).is_identity(), "inverse");
                    assert_eq!(c.negate(c.negate(a)), a);
                    // [#J] D = 0, both multiplication paths agree.
                    assert!(c.mul_uncounted(a, rg.jac_order).is_identity(), "[#J]D = 0");
                    assert_eq!(c.mul_rtl(a, 977), c.mul_uncounted(a, 977));
                    // Doubling is composition with itself, and counted as a doubling.
                    let mut ops = GroupOps::default();
                    assert_eq!(c.add(&mut ops, a, a), c.double(&mut ops, a));
                    assert_eq!((ops.adds, ops.doubles), (0, 2));
                }
                assert!(c.mul_uncounted(rg.d, t.r).is_identity());
                assert_eq!(c.mul_rtl(rg.d, t.x), rg.t);
            }
        }
        assert!(built >= 3, "built {built} towers");
    }

    #[test]
    fn agrees_with_hcurve_on_the_p_below_256_towers() {
        let mut built = 0;
        for (i, p) in [53u64, 101, 151, 199, 251].into_iter().enumerate() {
            let rep = crosscheck::run(p, 11 + i as u64, 150, 2).unwrap_or_else(|e| panic!("{e}"));
            if rep.built {
                built += 1;
                assert!(rep.divisors_compared >= 300 && rep.ic_solves_compared >= 4);
            }
        }
        assert!(built >= 4, "built {built}");
    }

    #[test]
    fn odd_primes_past_255_build_and_pass_the_group_law() {
        // p = 1009: first prime where HCurve cannot go.
        let t = build64(1009, 5, 1009 / 8, 64).expect("a tower at p = 1009");
        for rg in &t.rungs {
            let c = &rg.cover.curve;
            assert!(rg.jac_order.is_multiple_of(t.r));
            assert!(c.mul_uncounted(rg.d, t.r).is_identity());
            assert_eq!(c.mul_rtl(rg.d, t.x), rg.t);
            let mut st = 99;
            let (a, b, d) = (
                c.random_divisor(&mut st),
                c.random_divisor(&mut st),
                c.random_divisor(&mut st),
            );
            assert_eq!(c.cantor(c.cantor(a, b), d), c.cantor(a, c.cantor(b, d)));
            assert!(c.mul_uncounted(a, rg.jac_order).is_identity());
        }
    }
}

// Silence an unused-import warning when `BTreeMap` is only used by `ic`.
#[allow(dead_code)]
type _Unused = BTreeMap<u8, u8>;
