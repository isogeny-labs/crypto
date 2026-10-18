//! Explicit Smith isogeny `psi: J_H -> J_X` on divisor classes, with the plane
//! quartic `C ~ X` it lands on, and a self-contained linear-equivalence test on
//! `C` used to verify it.
//!
//! Source: B. Smith, "Isogenies and the discrete logarithm problem in
//! Jacobians of genus 3 hyperelliptic curves", J. Cryptology 22 (2009),
//! arXiv:0806.2995: §4 (trigonal construction, Donagi-Livne), §5 (trigonal
//! map `g = N/D`), §6 (the curve `X`, Prop. 3, the correspondence `R`,
//! Prop. 4).  Companion of [`super::smith_prop24`], which tests Smith's
//! Propositions 2 and 4; this module builds the map those propositions
//! license.
//!
//! # What `psi` is
//!
//! `H: y^2 = F(x)` (`deg F` 7 or 8, odd `q`), `S` a rational tractable
//! subgroup passing Propositions 2 and 4, `g = N/D` a rational trigonal map
//! for `S` in Smith's normal form, `G(t, x) = N(x) - t D(x)`.  A point of the
//! codomain `X` over `t_0` is a pair `{(G(t_0, x), y -+ b(x))}` of Mumford
//! ideals with `b^2 = F mod G(t_0, .)`.  Smith's correspondence is
//!
//! ```text
//! R = { (P, Q) : P in H, Q in X, g(x(P)) = f(Q), P in the "R-triple" of Q },
//! R-triple of Q = the triple {y = b(x)} whose leading coefficient b_2 is
//!                 rho(Q) = (d4 w^2 + d2 w + d0) / (8 sigma(t)),  w = b_2^2,
//! ```
//!
//! with `sigma in F_q[t]` the polynomial square root of `s(t)` fixed by
//! `lc(sigma) = sqrt_mod(lc(s))` (the representative in `[0, (q-1)/2]`).
//! `psi = (pi_X)_* (pi_H)^*`, so for an affine point `P` of `H`
//! `psi(P) = Q_1 + Q_2`, the two points of `X` over `g(x(P))` whose R-triple
//! contains `P`.  Reversing the sign of `sigma` replaces `R` by `R'` and `psi`
//! by `-psi`.  There is **no** extra factor 2 and no translation: on classes
//! of degree 0, `psi` is exactly Smith's isogeny, of kernel `S` (degree 8).
//!
//! # The plane quartic
//!
//! `(pi_X)_* (pi_H)^*` also maps `H^0(Omega_H) -> H^0(Omega_X)`
//! (dual correspondence), sending `x^i dx / y` to
//! `eta_i dt`, `eta_i(Q) = sum_{x_k in fibre} x_k^i D(x_k) / (y_k G'(x_k))`.
//! `Q -> (eta_0 : eta_1 : eta_2)` is therefore the canonical map of `X`; for
//! non-hyperelliptic `X` its image `C` is a smooth plane quartic.  The
//! coefficients of `C` are found by linear algebra on images of points of `H`
//! over `F_{q^j}`, `j <= 3`.
//!
//! # Output
//!
//! [`PsiImage`] holds two effective `F_q`-rational divisors `plus`, `minus` on
//! `C` (Galois-stable lists of closed points, each stored as one
//! representative over `F_{q^d}` in the field of the module's `L12`) with
//! `psi([D]) = [plus] - [minus]`.  When the reduced Mumford form of
//! `[D] + [c_base]` has only generic points, `minus = E_base` is the fixed
//! base divisor `Psi(A_base)` (degree 6, constant per `(H, S, seed)`) and
//! `plus` is an effective divisor of degree 6, so `psi([D]) = [plus] - [E_base]`.
//! Otherwise `plus`, `minus` are two effective divisors of equal degree.
//!
//! Scope: synthetic toy fields only.  No deployed-curve or sub-rho claim.

use super::smith_prop24::{
    analyze_subgroup, f_mod_g, factor_squarefree, g_coeffs, smith_deltas, smith_s,
    tractable_subgroups, El, Ext, MapOutcome, Poly, SplitMix64, Weierstrass,
};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::cell::Cell;
use std::collections::{BTreeMap, HashMap, HashSet};

// ---------------------------------------------------------------------------
// F_p helpers (u64, p < 2^31).

#[inline]
fn md(a: u64, b: u64, p: u64) -> u64 {
    a * b % p
}
#[inline]
fn ad(a: u64, b: u64, p: u64) -> u64 {
    let s = a + b;
    if s >= p {
        s - p
    } else {
        s
    }
}
#[inline]
fn sb(a: u64, b: u64, p: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + p - b
    }
}
#[inline]
fn ng(a: u64, p: u64) -> u64 {
    if a == 0 {
        0
    } else {
        p - a
    }
}
fn pw(b: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1 % p;
    let mut b = b % p;
    while e > 0 {
        if e & 1 == 1 {
            r = md(r, b, p);
        }
        b = md(b, b, p);
        e >>= 1;
    }
    r
}
fn inv(a: u64, p: u64) -> u64 {
    assert!(a % p != 0, "inverse of zero");
    pw(a, p - 2, p)
}
fn chi_p(a: u64, p: u64) -> i8 {
    let a = a % p;
    if a == 0 {
        0
    } else if pw(a, (p - 1) / 2, p) == 1 {
        1
    } else {
        -1
    }
}

// Polynomials over F_p, little-endian, trimmed.
fn ptrim(a: &mut Poly) {
    while a.last() == Some(&0) {
        a.pop();
    }
}
fn pdeg(a: &[u64]) -> isize {
    a.iter().rposition(|&c| c != 0).map_or(-1, |i| i as isize)
}
fn padd(a: &[u64], b: &[u64], p: u64) -> Poly {
    let n = a.len().max(b.len());
    let mut r: Poly = (0..n)
        .map(|i| ad(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0), p))
        .collect();
    ptrim(&mut r);
    r
}
fn psub(a: &[u64], b: &[u64], p: u64) -> Poly {
    let n = a.len().max(b.len());
    let mut r: Poly = (0..n)
        .map(|i| sb(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0), p))
        .collect();
    ptrim(&mut r);
    r
}
fn pscale(a: &[u64], c: u64, p: u64) -> Poly {
    let mut r: Poly = a.iter().map(|&x| md(x, c, p)).collect();
    ptrim(&mut r);
    r
}
fn pmul(a: &[u64], b: &[u64], p: u64) -> Poly {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut r = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        if x == 0 {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            r[i + j] = ad(r[i + j], md(x, y, p), p);
        }
    }
    ptrim(&mut r);
    r
}
fn pdivrem(a: &[u64], b: &[u64], p: u64) -> (Poly, Poly) {
    let mut b = b.to_vec();
    ptrim(&mut b);
    assert!(!b.is_empty(), "division by the zero polynomial");
    let mut r = a.to_vec();
    ptrim(&mut r);
    if r.len() < b.len() {
        return (Vec::new(), r);
    }
    let li = inv(*b.last().unwrap(), p);
    let mut q = vec![0u64; r.len() - b.len() + 1];
    while r.len() >= b.len() && !r.is_empty() {
        let s = r.len() - b.len();
        let c = md(*r.last().unwrap(), li, p);
        q[s] = c;
        for (i, &bi) in b.iter().enumerate() {
            r[s + i] = sb(r[s + i], md(c, bi, p), p);
        }
        ptrim(&mut r);
    }
    ptrim(&mut q);
    (q, r)
}
fn prem(a: &[u64], b: &[u64], p: u64) -> Poly {
    pdivrem(a, b, p).1
}
fn pmonic(a: &[u64], p: u64) -> Poly {
    let mut a = a.to_vec();
    ptrim(&mut a);
    if let Some(&l) = a.last() {
        let li = inv(l, p);
        a = pscale(&a, li, p);
    }
    a
}
fn pgcd(a: &[u64], b: &[u64], p: u64) -> Poly {
    let (mut a, mut b) = (a.to_vec(), b.to_vec());
    ptrim(&mut a);
    ptrim(&mut b);
    while !b.is_empty() {
        let r = prem(&a, &b, p);
        a = b;
        b = r;
    }
    pmonic(&a, p)
}
/// `(g, s, t)` with `s a + t b = g` monic.
fn pxgcd(a: &[u64], b: &[u64], p: u64) -> (Poly, Poly, Poly) {
    let (mut r0, mut r1) = (a.to_vec(), b.to_vec());
    ptrim(&mut r0);
    ptrim(&mut r1);
    let (mut s0, mut s1): (Poly, Poly) = (vec![1], Vec::new());
    let (mut t0, mut t1): (Poly, Poly) = (Vec::new(), vec![1]);
    while !r1.is_empty() {
        let (q, r) = pdivrem(&r0, &r1, p);
        let s2 = psub(&s0, &pmul(&q, &s1, p), p);
        let t2 = psub(&t0, &pmul(&q, &t1, p), p);
        r0 = r1;
        r1 = r;
        s0 = s1;
        s1 = s2;
        t0 = t1;
        t1 = t2;
    }
    if let Some(&l) = r0.last() {
        let li = inv(l, p);
        (pscale(&r0, li, p), pscale(&s0, li, p), pscale(&t0, li, p))
    } else {
        (r0, s0, t0)
    }
}
fn peval(a: &[u64], x: u64, p: u64) -> u64 {
    a.iter().rev().fold(0, |r, &c| ad(md(r, x, p), c, p))
}
fn pderiv(a: &[u64], p: u64) -> Poly {
    let mut r: Poly = (1..a.len()).map(|i| md(i as u64 % p, a[i], p)).collect();
    ptrim(&mut r);
    r
}
fn ppowmod(a: &[u64], mut e: u64, m: &[u64], p: u64) -> Poly {
    let mut r: Poly = vec![1];
    let mut b = prem(a, m, p);
    while e > 0 {
        if e & 1 == 1 {
            r = prem(&pmul(&r, &b, p), m, p);
        }
        b = prem(&pmul(&b, &b, p), m, p);
        e >>= 1;
    }
    r
}

/// `h` monic with `h^2 = sm` (monic, even degree), if one exists.
fn poly_sqrt_monic(sm: &[u64], p: u64) -> Option<Poly> {
    let n = pdeg(sm);
    if n < 0 || n % 2 == 1 {
        return None;
    }
    let m = (n / 2) as usize;
    let mut h = vec![0u64; m + 1];
    h[m] = 1;
    let inv2 = inv(2, p);
    for k in 1..=m {
        let mut acc = 0;
        for i in (m - k + 1)..=m {
            let j = 2 * m - k - i;
            if j > m - k && j <= m {
                acc = ad(acc, md(h[i], h[j], p), p);
            }
        }
        h[m - k] = md(sb(sm[2 * m - k], acc, p), inv2, p);
    }
    (pmul(&h, &h, p) == sm).then_some(h)
}

/// A monic irreducible polynomial of degree `d` over `F_p`.
pub fn irreducible_poly(p: u64, d: usize, rng: &mut SplitMix64) -> Poly {
    if d == 1 {
        return vec![0, 1];
    }
    let x: Poly = vec![0, 1];
    let primes: Vec<usize> = (2..=d).filter(|r| d % r == 0 && (2..*r).all(|s| r % s != 0)).collect();
    loop {
        let mut f: Poly = (0..d).map(|_| rng.below(p)).collect();
        f.push(1);
        if f[0] == 0 {
            continue;
        }
        // x^{p^d} = x mod f and gcd(x^{p^{d/r}} - x, f) = 1.
        let mut h = x.clone();
        let mut pows: Vec<Poly> = vec![h.clone()];
        for _ in 0..d {
            h = ppowmod(&h, p, &f, p);
            pows.push(h.clone());
        }
        if pows[d] != x {
            continue;
        }
        let ok = primes.iter().all(|&r| {
            let g = pgcd(&psub(&pows[d / r], &x, p), &f, p);
            pdeg(&g) == 0
        });
        if ok {
            return f;
        }
    }
}

// ---------------------------------------------------------------------------
// Field F_{p^d} with cost counters.

/// `F_{p^d} = F_p[z]/(m)` (wrapping [`Ext`], `d <= 8`) with counters of
/// `F_p` multiplications (`d^2` for a product plus `d(d-1)` for reduction; an
/// inversion is counted separately and priced by calibration).
#[derive(Clone)]
pub struct Fld {
    pub e: Ext,
    pub p: u64,
    pub d: usize,
    qm1: BigUint,
    mul_units: Cell<u64>,
    inv_calls: Cell<u64>,
}

impl Fld {
    pub fn with_modulus(p: u64, m: &[u64]) -> Fld {
        let e = Ext::new(m, p);
        let d = e.d;
        let q = BigUint::from(p).pow(d as u32);
        Fld {
            e,
            p,
            d,
            qm1: q - 1u32,
            mul_units: Cell::new(0),
            inv_calls: Cell::new(0),
        }
    }
    /// A field of degree `d` with a modulus found by a seeded search.
    pub fn new(p: u64, d: usize, seed: u64) -> Fld {
        let mut rng = SplitMix64(seed ^ (d as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ p);
        let m = irreducible_poly(p, d, &mut rng);
        Fld::with_modulus(p, &m)
    }
    pub fn zero(&self) -> El {
        [0; 8]
    }
    pub fn one(&self) -> El {
        self.e.one()
    }
    pub fn c(&self, x: u64) -> El {
        self.e.c(x)
    }
    pub fn z(&self) -> El {
        self.e.z()
    }
    pub fn is_zero(&self, a: &El) -> bool {
        self.e.is_zero(a)
    }
    pub fn eq(&self, a: &El, b: &El) -> bool {
        a[..self.d] == b[..self.d]
    }
    pub fn add(&self, a: &El, b: &El) -> El {
        self.e.add(a, b)
    }
    pub fn sub(&self, a: &El, b: &El) -> El {
        self.e.sub(a, b)
    }
    pub fn neg(&self, a: &El) -> El {
        self.e.neg(a)
    }
    pub fn mul(&self, a: &El, b: &El) -> El {
        let d = self.d as u64;
        self.mul_units.set(self.mul_units.get() + d * d + d * (d - 1));
        self.e.mul(a, b)
    }
    pub fn inv(&self, a: &El) -> El {
        self.inv_calls.set(self.inv_calls.get() + 1);
        self.e.inv(a)
    }
    pub fn scale(&self, a: &El, c: u64) -> El {
        self.mul_units.set(self.mul_units.get() + self.d as u64);
        let mut r = [0u64; 8];
        for i in 0..self.d {
            r[i] = md(a[i], c % self.p, self.p);
        }
        r
    }
    pub fn pow_big(&self, a: &El, e: &BigUint) -> El {
        let mut r = self.one();
        for i in (0..e.bits()).rev() {
            r = self.mul(&r, &r);
            if e.bit(i) {
                r = self.mul(&r, a);
            }
        }
        r
    }
    pub fn pow_u64(&self, a: &El, mut e: u64) -> El {
        let mut r = self.one();
        let mut b = *a;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(&r, &b);
            }
            b = self.mul(&b, &b);
            e >>= 1;
        }
        r
    }
    pub fn frob(&self, a: &El) -> El {
        self.pow_u64(a, self.p)
    }
    pub fn frob_k(&self, a: &El, k: usize) -> El {
        let mut r = *a;
        for _ in 0..k {
            r = self.frob(&r);
        }
        r
    }
    pub fn is_square(&self, a: &El) -> bool {
        if self.is_zero(a) {
            return true;
        }
        let e = &self.qm1 >> 1u32;
        self.eq(&self.pow_big(a, &e), &self.one())
    }
    /// A square root, Tonelli-Shanks with big exponents.
    pub fn sqrt(&self, a: &El) -> Option<El> {
        if self.is_zero(a) {
            return Some(self.zero());
        }
        if !self.is_square(a) {
            return None;
        }
        let mut r = self.qm1.clone();
        let mut s = 0u32;
        while !r.bit(0) {
            r >>= 1u32;
            s += 1;
        }
        // non-residue
        let mut zc = self.z();
        if self.d == 1 {
            zc = self.c(2);
        }
        let mut cnt = 0u64;
        let mut nr = zc;
        while self.is_square(&nr) {
            cnt += 1;
            nr = self.add(&self.mul(&zc, &zc), &self.c(cnt + 1));
            nr = self.add(&nr, &self.scale(&zc, cnt));
        }
        let mut m = s;
        let mut c = self.pow_big(&nr, &r);
        let mut t = self.pow_big(a, &r);
        let mut rr = self.pow_big(a, &((&r + 1u32) >> 1u32));
        let one = self.one();
        while !self.eq(&t, &one) {
            let mut i = 0u32;
            let mut tt = t;
            while !self.eq(&tt, &one) {
                tt = self.mul(&tt, &tt);
                i += 1;
            }
            let mut b = c;
            for _ in 0..(m - i - 1) {
                b = self.mul(&b, &b);
            }
            m = i;
            c = self.mul(&b, &b);
            t = self.mul(&t, &c);
            rr = self.mul(&rr, &b);
        }
        Some(rr)
    }
    pub fn reset_counters(&self) {
        self.mul_units.set(0);
        self.inv_calls.set(0);
    }
    /// `(F_p multiplications in products, number of inversions)`.
    pub fn counters(&self) -> (u64, u64) {
        (self.mul_units.get(), self.inv_calls.get())
    }
    /// Evaluate a polynomial with `F_p` coefficients at an element.
    pub fn eval(&self, f: &[u64], x: &El) -> El {
        let mut r = self.zero();
        for &c in f.iter().rev() {
            r = self.add(&self.mul(&r, x), &self.c(c));
        }
        r
    }
    pub fn from_u64s(&self, cs: &[u64]) -> El {
        let mut r = [0u64; 8];
        for (i, &c) in cs.iter().take(self.d).enumerate() {
            r[i] = c % self.p;
        }
        r
    }
}

// Polynomials over Fld, only for root finding.
type KP = Vec<El>;
fn ktrim(k: &Fld, a: &mut KP) {
    while a.last().is_some_and(|x| k.is_zero(x)) {
        a.pop();
    }
}
fn kdivrem(k: &Fld, a: &[El], b: &[El]) -> (KP, KP) {
    let mut r = a.to_vec();
    ktrim(k, &mut r);
    let mut b = b.to_vec();
    ktrim(k, &mut b);
    let db = b.len() - 1;
    if r.len() <= db {
        return (Vec::new(), r);
    }
    let li = k.inv(&b[db]);
    let mut q = vec![k.zero(); r.len() - db];
    while r.len() > db {
        let s = r.len() - 1 - db;
        let c = k.mul(&r[r.len() - 1], &li);
        q[s] = c;
        for (i, bi) in b.iter().enumerate() {
            r[s + i] = k.sub(&r[s + i], &k.mul(&c, bi));
        }
        ktrim(k, &mut r);
    }
    (q, r)
}
fn kmulmod(k: &Fld, a: &[El], b: &[El], m: &[El]) -> KP {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut r = vec![k.zero(); a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            r[i + j] = k.add(&r[i + j], &k.mul(x, y));
        }
    }
    kdivrem(k, &r, m).1
}
fn kmonic(k: &Fld, a: &[El]) -> KP {
    let mut a = a.to_vec();
    ktrim(k, &mut a);
    if let Some(l) = a.last().copied() {
        let li = k.inv(&l);
        for x in a.iter_mut() {
            *x = k.mul(x, &li);
        }
    }
    a
}
fn kgcd(k: &Fld, a: &[El], b: &[El]) -> KP {
    let (mut a, mut b) = (a.to_vec(), b.to_vec());
    ktrim(k, &mut a);
    ktrim(k, &mut b);
    while !b.is_empty() {
        let r = kdivrem(k, &a, &b).1;
        a = b;
        b = r;
    }
    kmonic(k, &a)
}
/// One root in `k` of a squarefree polynomial over `F_p` all of whose roots
/// lie in `k`.
pub fn one_root(k: &Fld, f_fp: &[u64], rng: &mut SplitMix64) -> El {
    let mut f: KP = kmonic(k, &f_fp.iter().map(|&c| k.c(c)).collect::<Vec<_>>());
    let e = &k.qm1 >> 1u32;
    loop {
        let n = f.len() - 1;
        if n == 1 {
            return k.neg(&f[0]);
        }
        let mut a: KP = (0..n)
            .map(|_| {
                let mut x = [0; 8];
                for c in x.iter_mut().take(k.d) {
                    *c = rng.below(k.p);
                }
                x
            })
            .collect();
        ktrim(k, &mut a);
        if a.len() < 2 {
            continue;
        }
        let mut r: KP = vec![k.one()];
        let mut b = kdivrem(k, &a, &f).1;
        for i in 0..e.bits() {
            if e.bit(i) {
                r = kmulmod(k, &r, &b, &f);
            }
            b = kmulmod(k, &b, &b, &f);
        }
        if r.is_empty() {
            r.push(k.zero());
        }
        r[0] = k.sub(&r[0], &k.one());
        let h = kgcd(k, &r, &f);
        let dh = h.len() - 1;
        if dh > 0 && dh < n {
            let q = kmonic(k, &kdivrem(k, &f, &h).0);
            f = if dh < q.len() { h } else { q };
        }
    }
}

// ---------------------------------------------------------------------------
// Linear algebra over F_p.

fn rref(mat: &mut Vec<Vec<u64>>, p: u64) -> Vec<usize> {
    let rows = mat.len();
    if rows == 0 {
        return Vec::new();
    }
    let cols = mat[0].len();
    let mut piv = Vec::new();
    let mut r = 0;
    for c in 0..cols {
        if r == rows {
            break;
        }
        let Some(k) = (r..rows).find(|&i| mat[i][c] != 0) else {
            continue;
        };
        mat.swap(r, k);
        let iv = inv(mat[r][c], p);
        for x in mat[r].iter_mut() {
            *x = md(*x, iv, p);
        }
        let pivot = mat[r].clone();
        for (i, row) in mat.iter_mut().enumerate() {
            if i == r || row[c] == 0 {
                continue;
            }
            let f = row[c];
            for j in 0..cols {
                row[j] = sb(row[j], md(f, pivot[j], p), p);
            }
        }
        piv.push(c);
        r += 1;
    }
    mat.truncate(r);
    piv
}

/// A basis of `{x : A x = 0}`.
fn nullspace(rows: &[Vec<u64>], cols: usize, p: u64) -> Vec<Vec<u64>> {
    let mut m = rows.to_vec();
    let piv = rref(&mut m, p);
    let mut basis = Vec::new();
    for f in (0..cols).filter(|c| !piv.contains(c)) {
        let mut x = vec![0u64; cols];
        x[f] = 1;
        for (ri, &pc) in piv.iter().enumerate() {
            x[pc] = ng(m[ri][f], p);
        }
        basis.push(x);
    }
    basis
}

// ---------------------------------------------------------------------------
// Smith's data for one (H, S, g).

/// Everything Smith's construction needs for one tractable subgroup `S`
/// with a rational trigonal map `g = N/D` in normal form.
#[derive(Clone, Debug)]
pub struct SmithData {
    pub p: u64,
    /// `F(x)`, degree 7 or 8.
    pub f: Poly,
    /// `N = x^3 + n1 x + n0`, `D = x^2 + d1 x + d0`.
    pub n1: u64,
    pub n0: u64,
    pub d1: u64,
    pub d0: u64,
    pub fs: [Poly; 3],
    pub gs: [Poly; 3],
    pub s: Poly,
    /// `sigma^2 = s`, `lc(sigma) = sqrt_mod(lc(s))`; `None` if `lc(s)` is a
    /// non-square (the isogeny is then not `F_p`-rational: Prop. 4 fails).
    pub sigma: Option<Poly>,
    /// `delta_0, delta_2, delta_4` of Prop. 3.
    pub deltas: [Poly; 3],
    pub label: String,
    /// Square class of `lc(s)`.
    pub alpha_chi: i8,
}

impl SmithData {
    pub fn from_map(f: &[u64], p: u64, label: &str, m: &MapOutcome) -> Option<SmithData> {
        if !m.valid() {
            return None;
        }
        let (n1, n0, d1, d0) = m.map.smith_coeffs()?;
        let mut f = f.to_vec();
        ptrim(&mut f);
        let fs = f_mod_g(&f, n1, n0, d1, d0, p);
        let gs = g_coeffs(n1, n0, d1, d0, p);
        let s = smith_s(&fs, &gs, p);
        let lc = *s.last()?;
        let chi = chi_p(lc, p);
        let sigma = if chi == 1 {
            let h = poly_sqrt_monic(&pmonic(&s, p), p)?;
            let r = crate::cryptanalysis::smith_prop24::sqrt_mod(lc, p)?;
            Some(pscale(&h, r, p))
        } else {
            None
        };
        let deltas = smith_deltas(&fs, &gs, p);
        Some(SmithData {
            p,
            f,
            n1,
            n0,
            d1,
            d0,
            fs,
            gs,
            s,
            sigma,
            deltas,
            label: label.to_string(),
            alpha_chi: chi,
        })
    }

    pub fn deg_f(&self) -> usize {
        pdeg(&self.f) as usize
    }

    pub fn rational(&self) -> bool {
        self.sigma.is_some()
    }

    /// `N(x)`, `D(x)` as polynomials.
    pub fn n_poly(&self) -> Poly {
        vec![self.n0, self.n1, 0, 1]
    }
    pub fn d_poly(&self) -> Poly {
        vec![self.d0, self.d1, 1]
    }
}

/// All `(S, g)` pairs of `H: y^2 = f` with a valid normal-form trigonal map,
/// with Smith's data; Proposition 2 holds for all of them, Proposition 4 for
/// those with `rational()`.
pub fn smith_data_for_curve(f: &[u64], p: u64, rng: &mut SplitMix64) -> Vec<SmithData> {
    let Some(w) = Weierstrass::new(f, p, rng) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for sg in tractable_subgroups(&w, rng) {
        let so = analyze_subgroup(&w.f, &sg, p);
        if !so.trig_rational() {
            continue;
        }
        for m in &so.maps {
            if let Some(sd) = SmithData::from_map(&w.f, p, &so.label, m) {
                out.push(sd);
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------
// psi on a point.

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Degenerate {
    /// `D(x_0) = 0`: the point lies over `t = infinity`.
    OverInfinity,
    /// `G(g(x_0), .)` has a repeated root.
    RepeatedFibre,
    /// A Weierstrass point in the fibre (`y = 0`).
    WeierstrassInFibre,
    /// `F(x_2)` not a square in the working field (`S` or `g` not rational).
    NoSqrt,
    /// The 4 candidates did not satisfy Prop. 3's quartic, or the R-selection
    /// did not pick exactly 2 triples (a bug if it ever fires).
    Inconsistent(String),
}

/// A point `Q` of `X` over `t_0`, with the R-triple `b` and its canonical image.
#[derive(Clone, Debug)]
pub struct PsiPoint {
    pub t: El,
    /// `b(x) = b[0] + b[1] x + b[2] x^2`, `b[2] = rho`.
    pub b: [El; 3],
    /// `(eta_0 : eta_1 : eta_2)`, first nonzero coordinate 1.
    pub kappa: [El; 3],
}

fn normalize_proj(k: &Fld, v: &[El; 3]) -> Option<[El; 3]> {
    let i = (0..3).find(|&i| !k.is_zero(&v[i]))?;
    let li = k.inv(&v[i]);
    Some([
        k.mul(&v[0], &li),
        k.mul(&v[1], &li),
        k.mul(&v[2], &li),
    ])
}

impl SmithData {
    /// `psi(P) = Q_1 + Q_2` for the affine point `P = (x0, y0)` of `H` with
    /// coordinates in `k`.  `k` must contain `F_{p^{2j}}` for the degree `j`
    /// of the field of definition of `P`.
    pub fn psi_point(&self, k: &Fld, x0: &El, y0: &El) -> Result<Vec<PsiPoint>, Degenerate> {
        let sigma = self.sigma.as_ref().expect("rational isogeny needs lc(s) square");
        let p = self.p;
        let dx = k.add(
            &k.add(&k.mul(x0, x0), &k.scale(x0, self.d1)),
            &k.c(self.d0),
        );
        if k.is_zero(&dx) {
            return Err(Degenerate::OverInfinity);
        }
        let nx = k.add(
            &k.add(&k.mul(&k.mul(x0, x0), x0), &k.scale(x0, self.n1)),
            &k.c(self.n0),
        );
        let t0 = k.mul(&nx, &k.inv(&dx));
        // G = (x - x0)(x^2 + e x + f)
        let g1 = k.sub(&k.c(self.n1), &k.scale(&t0, self.d1));
        let g0 = k.sub(&k.c(self.n0), &k.scale(&t0, self.d0));
        let e = k.sub(x0, &t0);
        let fc = k.add(&g1, &k.mul(x0, &e));
        debug_assert!(k.is_zero(&k.add(&g0, &k.mul(x0, &fc))));
        let disc = k.sub(&k.mul(&e, &e), &k.scale(&fc, 4));
        let Some(r) = k.sqrt(&disc) else {
            return Err(Degenerate::NoSqrt);
        };
        if k.is_zero(&r) {
            return Err(Degenerate::RepeatedFibre);
        }
        let i2 = inv(2, p);
        let x2 = k.scale(&k.sub(&r, &e), i2);
        let x3 = k.scale(&k.neg(&k.add(&r, &e)), i2);
        if k.eq(&x2, x0) || k.eq(&x3, x0) {
            return Err(Degenerate::RepeatedFibre);
        }
        let f0 = k.eval(&self.f, x0);
        let f2 = k.eval(&self.f, &x2);
        let f3 = k.eval(&self.f, &x3);
        if k.is_zero(&f0) || k.is_zero(&f2) || k.is_zero(&f3) {
            return Err(Degenerate::WeierstrassInFibre);
        }
        // y0 is given; check it.
        debug_assert!(k.eq(&k.mul(y0, y0), &f0));
        let (Some(y2), Some(y3)) = (k.sqrt(&f2), k.sqrt(&f3)) else {
            return Err(Degenerate::NoSqrt);
        };
        let xs = [*x0, x2, x3];
        let dens: Vec<El> = (0..3)
            .map(|i| {
                let (j, l) = ((i + 1) % 3, (i + 2) % 3);
                k.mul(&k.sub(&xs[i], &xs[j]), &k.sub(&xs[i], &xs[l]))
            })
            .collect();
        let dinv: Vec<El> = dens.iter().map(|d| k.inv(d)).collect();
        let sig = k.eval(sigma, &t0);
        if k.is_zero(&sig) {
            return Err(Degenerate::Inconsistent("sigma(t0) = 0".into()));
        }
        let d0t = k.eval(&self.deltas[0], &t0);
        let d2t = k.eval(&self.deltas[1], &t0);
        let d4t = k.eval(&self.deltas[2], &t0);
        let st = k.eval(&self.s, &t0);
        let eight_sig_inv = k.inv(&k.scale(&sig, 8));
        let mut out = Vec::new();
        for (e2, e3) in [(false, false), (false, true), (true, false), (true, true)] {
            let ys = [
                *y0,
                if e2 { k.neg(&y2) } else { y2 },
                if e3 { k.neg(&y3) } else { y3 },
            ];
            // Lagrange interpolation of y = b(x) through (x_i, y_i).
            let mut b = [k.zero(), k.zero(), k.zero()];
            for i in 0..3 {
                let (j, l) = ((i + 1) % 3, (i + 2) % 3);
                let w = k.mul(&ys[i], &dinv[i]);
                b[2] = k.add(&b[2], &w);
                b[1] = k.sub(&b[1], &k.mul(&w, &k.add(&xs[j], &xs[l])));
                b[0] = k.add(&b[0], &k.mul(&w, &k.mul(&xs[j], &xs[l])));
            }
            let w = k.mul(&b[2], &b[2]);
            let inner = k.add(
                &k.add(&k.mul(&d4t, &k.mul(&w, &w)), &k.mul(&d2t, &w)),
                &d0t,
            );
            // Prop. 3: (d4 w^2 + d2 w + d0)^2 = 64 s w
            let lhs = k.mul(&inner, &inner);
            let rhs = k.scale(&k.mul(&st, &w), 64);
            if !k.eq(&lhs, &rhs) {
                return Err(Degenerate::Inconsistent("Prop. 3 quartic fails".into()));
            }
            let rho = k.mul(&inner, &eight_sig_inv);
            if !k.eq(&rho, &b[2]) {
                continue;
            }
            // canonical image: eta_i = sum_k x_k^i D(x_k) / (y_k G'(x_k))
            let mut eta = [k.zero(), k.zero(), k.zero()];
            for i in 0..3 {
                let xk = xs[i];
                let dk = k.add(&k.add(&k.mul(&xk, &xk), &k.scale(&xk, self.d1)), &k.c(self.d0));
                let c = k.mul(&k.mul(&dk, &dinv[i]), &k.inv(&ys[i]));
                let mut pw_ = k.one();
                for slot in eta.iter_mut() {
                    *slot = k.add(slot, &k.mul(&c, &pw_));
                    pw_ = k.mul(&pw_, &xk);
                }
            }
            let Some(kappa) = normalize_proj(k, &eta) else {
                return Err(Degenerate::Inconsistent("eta = 0".into()));
            };
            out.push(PsiPoint { t: t0, b, kappa });
        }
        if out.len() != 2 {
            return Err(Degenerate::Inconsistent(format!(
                "R-selection picked {} triples",
                out.len()
            )));
        }
        Ok(out)
    }
}

// ---------------------------------------------------------------------------
// Mumford divisors on the imaginary model `y^2 = F(x)`, `deg F = 7`.

/// Reduced divisor `(u, v)`: `u` monic, `deg u <= 3`, `deg v < deg u`,
/// `v^2 = F mod u`; it stands for the class `[D - deg(u) infinity]`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mum {
    pub u: Poly,
    pub v: Poly,
}

/// The curve `H: y^2 = F(x)` with `deg F = 7` (genus 3, `infinity` a
/// rational Weierstrass point) and Cantor's arithmetic on `J_H(F_p)`.
#[derive(Clone, Debug)]
pub struct Hyp {
    pub p: u64,
    pub f: Poly,
}

impl Hyp {
    pub fn new(f: &[u64], p: u64) -> Hyp {
        let mut f = f.to_vec();
        ptrim(&mut f);
        assert_eq!(pdeg(&f), 7, "Hyp is the imaginary model deg F = 7");
        Hyp { p, f }
    }
    pub fn zero(&self) -> Mum {
        Mum {
            u: vec![1],
            v: Vec::new(),
        }
    }
    pub fn is_valid(&self, d: &Mum) -> bool {
        let p = self.p;
        pdeg(&d.u) <= 3
            && d.u.last() == Some(&1)
            && pdeg(&d.v) < pdeg(&d.u)
            && prem(&psub(&pmul(&d.v, &d.v, p), &self.f, p), &d.u, p).is_empty()
    }
    pub fn neg(&self, d: &Mum) -> Mum {
        Mum {
            u: d.u.clone(),
            v: pscale(&d.v, self.p - 1, self.p),
        }
    }
    /// Cantor addition (composition and reduction).
    pub fn add(&self, a: &Mum, b: &Mum) -> Mum {
        let p = self.p;
        let (d1, e1, e2) = pxgcd(&a.u, &b.u, p);
        let (d, c1, c2) = pxgcd(&d1, &padd(&a.v, &b.v, p), p);
        let s1 = pmul(&c1, &e1, p);
        let s2 = pmul(&c1, &e2, p);
        let s3 = c2;
        let dd = pmul(&d, &d, p);
        let u = pdivrem(&pmul(&a.u, &b.u, p), &dd, p).0;
        let t1 = pmul(&pmul(&s1, &a.u, p), &b.v, p);
        let t2 = pmul(&pmul(&s2, &b.u, p), &a.v, p);
        let t3 = pmul(&s3, &padd(&pmul(&a.v, &b.v, p), &self.f, p), p);
        let vv = pdivrem(&padd(&padd(&t1, &t2, p), &t3, p), &d, p).0;
        let mut u = pmonic(&u, p);
        let mut v = prem(&vv, &u, p);
        while pdeg(&u) > 3 {
            let num = psub(&self.f, &pmul(&v, &v, p), p);
            let un = pmonic(&pdivrem(&num, &u, p).0, p);
            let vn = prem(&pscale(&v, p - 1, p), &un, p);
            u = un;
            v = vn;
        }
        Mum { u, v }
    }
    pub fn sub(&self, a: &Mum, b: &Mum) -> Mum {
        self.add(a, &self.neg(b))
    }
    pub fn mul(&self, mut n: u64, a: &Mum) -> Mum {
        let mut r = self.zero();
        let mut b = a.clone();
        while n > 0 {
            if n & 1 == 1 {
                r = self.add(&r, &b);
            }
            b = self.add(&b, &b);
            n >>= 1;
        }
        r
    }
    /// A random reduced divisor from `n` random points of `H` over `F_{p^j}`
    /// (shape: the degrees `j` of the closed points, `sum j <= 3`).
    pub fn random_shape(&self, shape: &[usize], rng: &mut SplitMix64) -> Option<Mum> {
        let p = self.p;
        let mut u: Poly = vec![1];
        let mut v: Poly = Vec::new();
        for &j in shape {
            let mut r;
            let mut tries = 0;
            loop {
                tries += 1;
                if tries > 500 {
                    return None;
                }
                let mut c: Poly = (0..j).map(|_| rng.below(p)).collect();
                c.push(1);
                if j == 1 {
                    r = c;
                    break;
                }
                if let Some(fs) = factor_squarefree(&c, p, rng) {
                    if fs.len() == 1 {
                        r = c;
                        break;
                    }
                }
            }
            if pdeg(&pgcd(&r, &u, p)) > 0 {
                return None;
            }
            let k = Fld::with_modulus(p, &if j == 1 { vec![0, 1] } else { r.clone() });
            // x0 = z (j >= 2) or the rational root (j == 1)
            let (x0, vr_of): (El, Box<dyn Fn(&El) -> Poly>) = if j == 1 {
                (k.c(ng(r[0], p)), Box::new(|y: &El| vec![y[0]]))
            } else {
                (k.z(), Box::new(move |y: &El| y[..j].to_vec()))
            };
            let fx = k.eval(&self.f, &x0);
            let y0 = k.sqrt(&fx)?;
            if k.is_zero(&y0) {
                return None;
            }
            let mut vr = vr_of(&y0);
            ptrim(&mut vr);
            // CRT: v = v (mod u), v = vr (mod r)
            let (_, s, t) = pxgcd(&u, &r, p);
            // s u + t r = 1
            let num = padd(
                &pmul(&pmul(&s, &u, p), &vr, p),
                &pmul(&pmul(&t, &r, p), &v, p),
                p,
            );
            let un = pmul(&u, &r, p);
            v = prem(&num, &un, p);
            u = un;
        }
        let d = Mum { u, v };
        debug_assert!(self.is_valid(&d), "random_shape built an invalid divisor");
        Some(d)
    }
    /// A random degree-3 divisor of a random closed-point shape.
    pub fn random_generic3(&self, rng: &mut SplitMix64) -> Mum {
        loop {
            let shape: &[usize] = match rng.below(6) {
                0 | 1 => &[3],
                2 | 3 => &[2, 1],
                _ => &[1, 1, 1],
            };
            if let Some(d) = self.random_shape(shape, rng) {
                return d;
            }
        }
    }
    /// A random element of the Jacobian (sum of two random generic divisors).
    pub fn random_class(&self, rng: &mut SplitMix64) -> Mum {
        let a = self.random_generic3(rng);
        let b = self.random_generic3(rng);
        self.sub(&a, &b)
    }
}

// ---------------------------------------------------------------------------
// Closed points of P^2 and Galois-stable effective divisors.

/// A closed point of `P^2` over `F_p` of degree `m`, stored without any
/// choice of field: the projective coordinates (first nonzero one equal to 1)
/// are polynomials of degree `< m` in a generator `c` of the residue field
/// whose minimal polynomial is `mu`.  `c = kappa_1 + lambda kappa_2` with the
/// least `lambda` that generates; the data is the same for every Frobenius
/// conjugate, so equality of keys is equality of closed points.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClosedPt {
    pub mu: Poly,
    pub co: [Poly; 3],
}

impl ClosedPt {
    pub fn degree(&self) -> usize {
        self.mu.len() - 1
    }
}

/// An effective `F_p`-rational divisor: closed point -> multiplicity.
pub type Div = BTreeMap<ClosedPt, u32>;

pub fn div_degree(d: &Div) -> usize {
    d.iter().map(|(c, &m)| c.degree() * m as usize).sum()
}

pub fn div_add(a: &Div, b: &Div) -> Div {
    let mut r = a.clone();
    for (c, &m) in b {
        *r.entry(c.clone()).or_insert(0) += m;
    }
    r
}

pub fn div_is_reduced(d: &Div) -> bool {
    d.values().all(|&m| m == 1)
}

fn norm_pt3(k: &Fld, kap: &[El; 3]) -> Option<[El; 3]> {
    normalize_proj(k, kap)
}

fn frob_pt(k: &Fld, kap: &[El; 3]) -> [El; 3] {
    [k.frob(&kap[0]), k.frob(&kap[1]), k.frob(&kap[2])]
}

fn pt_eq(k: &Fld, a: &[El; 3], b: &[El; 3]) -> bool {
    (0..3).all(|i| k.eq(&a[i], &b[i]))
}

/// Orbit length of a normalised point under the `p`-Frobenius.
fn orbit_len(k: &Fld, kap: &[El; 3]) -> usize {
    let mut cur = frob_pt(k, kap);
    let mut m = 1;
    while !pt_eq(k, &cur, kap) {
        cur = frob_pt(k, &cur);
        m += 1;
        assert!(m <= 8, "Frobenius orbit longer than the field");
    }
    m
}

/// Coordinates of `target` in the `F_p`-basis `1, c, .., c^{m-1}`.
fn solve_in_powers(k: &Fld, c: &El, m: usize, target: &El) -> Poly {
    let p = k.p;
    let d = k.d;
    let mut cols: Vec<El> = Vec::with_capacity(m);
    let mut pw_ = k.one();
    for _ in 0..m {
        cols.push(pw_);
        pw_ = k.mul(&pw_, c);
    }
    let mut mat: Vec<Vec<u64>> = (0..d)
        .map(|r| {
            let mut row: Vec<u64> = cols.iter().map(|cc| cc[r]).collect();
            row.push(target[r]);
            row
        })
        .collect();
    let piv = rref(&mut mat, p);
    assert!(
        !piv.contains(&m) && piv.len() == m,
        "coordinate not in the F_p-span of the powers of c"
    );
    let mut out = vec![0u64; m];
    for (ri, &pc) in piv.iter().enumerate() {
        out[pc] = mat[ri][m];
    }
    ptrim(&mut out);
    out
}

/// The closed point of a point `kappa` of `P^2(k)`.
pub fn closed_pt_from(k: &Fld, kap: &[El; 3]) -> ClosedPt {
    let p = k.p;
    let kap = norm_pt3(k, kap).expect("zero point");
    let m = orbit_len(k, &kap);
    if m == 1 {
        let co = [0, 1, 2].map(|a| {
            let mut v = vec![kap[a][0]];
            ptrim(&mut v);
            v
        });
        return ClosedPt { mu: vec![0, 1], co };
    }
    // generator c = kappa_1 + lambda kappa_2 (kappa_0 = 1 or 0 as normalised)
    let (a1, a2) = if k.is_zero(&kap[0]) && k.is_zero(&kap[1]) {
        (2usize, 2usize)
    } else {
        (1usize, 2usize)
    };
    let mut c = k.zero();
    let mut found = false;
    for lam in 0..p {
        let cand = k.add(&kap[a1], &k.scale(&kap[a2], lam));
        // order of cand under Frobenius
        let mut cur = k.frob(&cand);
        let mut n = 1;
        while !k.eq(&cur, &cand) && n <= 8 {
            cur = k.frob(&cur);
            n += 1;
        }
        if n == m {
            c = cand;
            found = true;
            break;
        }
    }
    assert!(found, "no generator of the residue field among kappa_1 + lambda kappa_2");
    // mu = prod (Z - c^{p^i})
    let mut mu: Vec<El> = vec![k.one()];
    let mut ci = c;
    for _ in 0..m {
        let mut nm = vec![k.zero(); mu.len() + 1];
        for (i, co) in mu.iter().enumerate() {
            nm[i + 1] = k.add(&nm[i + 1], co);
            nm[i] = k.sub(&nm[i], &k.mul(co, &ci));
        }
        mu = nm;
        ci = k.frob(&ci);
    }
    let mu_fp: Poly = mu
        .iter()
        .map(|e| {
            assert!(
                (1..k.d).all(|t| e[t] == 0),
                "minimal polynomial not over F_p"
            );
            e[0]
        })
        .collect();
    let co = [0, 1, 2].map(|a| solve_in_powers(k, &c, m, &kap[a]));
    ClosedPt { mu: mu_fp, co }
}

/// Group a Frobenius-stable multiset of points of `P^2(k)` into closed points.
fn group_orbits(k: &Fld, pts: &[[El; 3]]) -> Result<Div, String> {
    let mut left: Vec<([El; 3], u32)> = Vec::new();
    for pt in pts {
        let Some(n) = norm_pt3(k, pt) else {
            return Err("zero point".into());
        };
        if let Some(e) = left.iter_mut().find(|e| pt_eq(k, &e.0, &n)) {
            e.1 += 1;
        } else {
            left.push((n, 1));
        }
    }
    let mut out = Div::new();
    while let Some((pt, cnt)) = left.pop() {
        let mut cur = frob_pt(k, &pt);
        let mut m = 1u32;
        while !pt_eq(k, &cur, &pt) {
            let Some(i) = left.iter().position(|e| pt_eq(k, &e.0, &cur)) else {
                return Err("multiset is not Frobenius stable".into());
            };
            if left[i].1 != cnt {
                return Err("orbit multiplicities differ".into());
            }
            left.swap_remove(i);
            cur = frob_pt(k, &cur);
            m += 1;
        }
        let _ = m;
        *out.entry(closed_pt_from(k, &pt)).or_insert(0) += cnt;
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// psi on effective divisors and on classes.

/// The image of a class: `psi(x) = [plus] - [minus]` in `Pic^0(C)`, both
/// effective of degree 6.
#[derive(Clone, Debug)]
pub struct PsiImage {
    pub plus: Div,
    pub minus: Div,
}

/// Smith's map on `H: y^2 = F(x)` (`deg F = 7`) for one `(S, g)`.
pub struct Smith {
    pub sd: SmithData,
    pub h: Hyp,
    fields: std::cell::RefCell<HashMap<usize, Fld>>,
    seed: u64,
}

impl Smith {
    pub fn new(sd: SmithData) -> Smith {
        assert!(sd.rational(), "Prop. 4 fails: no F_p-rational isogeny");
        let h = Hyp::new(&sd.f, sd.p);
        Smith {
            sd,
            h,
            fields: Default::default(),
            seed: 0x5EED_1018,
        }
    }
    fn with_field<R>(&self, d: usize, f: impl FnOnce(&Fld) -> R) -> R {
        let mut m = self.fields.borrow_mut();
        let k = m
            .entry(d)
            .or_insert_with(|| Fld::new(self.sd.p, d, self.seed));
        f(k)
    }
    /// `(F_p multiplications, inversions)` spent so far in all fields.
    pub fn counters(&self) -> (u64, u64) {
        let m = self.fields.borrow();
        m.values().fold((0, 0), |a, k| {
            let c = k.counters();
            (a.0 + c.0, a.1 + c.1)
        })
    }
    pub fn reset_counters(&self) {
        for k in self.fields.borrow().values() {
            k.reset_counters();
        }
    }
    /// `psi(D)` for an effective squarefree affine divisor `(u, v)`.
    pub fn psi_effective(&self, d: &Mum, rng: &mut SplitMix64) -> Result<Div, Degenerate> {
        let p = self.sd.p;
        let Some(factors) = factor_squarefree(&d.u, p, rng) else {
            return Err(Degenerate::Inconsistent("u not squarefree".into()));
        };
        let mut out = Div::new();
        for r in factors {
            let j = r.len() - 1;
            let res: Result<Div, Degenerate> = self.with_field(2 * j, |k| {
                let x0 = one_root(k, &r, rng);
                let y0 = k.eval(&d.v, &x0);
                let pts = self.sd.psi_point(k, &x0, &y0)?;
                let mut ms: Vec<[El; 3]> = Vec::new();
                for q in &pts {
                    let mut cur = q.kappa;
                    for _ in 0..j {
                        ms.push(cur);
                        cur = frob_pt(k, &cur);
                    }
                }
                group_orbits(k, &ms).map_err(Degenerate::Inconsistent)
            });
            out = div_add(&out, &res?);
        }
        Ok(out)
    }
    /// `Psi_A(x) = [psi(D')] - [psi(A)]` with `D' ~ x + A` of degree 3.
    pub fn psi_class(
        &self,
        x: &Mum,
        base: &Mum,
        rng: &mut SplitMix64,
    ) -> Result<PsiImage, Degenerate> {
        let dp = self.h.add(x, base);
        if pdeg(&dp.u) != 3 {
            return Err(Degenerate::Inconsistent("D' has degree < 3".into()));
        }
        Ok(PsiImage {
            plus: self.psi_effective(&dp, rng)?,
            minus: self.psi_effective(base, rng)?,
        })
    }
    /// As [`Self::psi_class`] with random generic bases until it works;
    /// returns the base used.
    pub fn psi_class_auto(
        &self,
        x: &Mum,
        rng: &mut SplitMix64,
    ) -> (PsiImage, Mum) {
        for _ in 0..2000 {
            let a = self.h.random_generic3(rng);
            if let Ok(im) = self.psi_class(x, &a, rng) {
                return (im, a);
            }
        }
        panic!("no generic base divisor found");
    }
}

// ---------------------------------------------------------------------------
// Plane forms, frames, resultants.

/// A homogeneous form in `X, Y, Z` over `F_p`, as a monomial -> coefficient map.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Form {
    pub d: usize,
    pub c: BTreeMap<[u8; 3], u64>,
}

/// Monomials `X^i Y^j Z^k` of degree `d`, in a fixed order.
pub fn monomials(d: usize) -> Vec<[u8; 3]> {
    let mut v = Vec::new();
    for i in (0..=d).rev() {
        for j in (0..=d - i).rev() {
            v.push([i as u8, j as u8, (d - i - j) as u8]);
        }
    }
    v
}

impl Form {
    pub fn from_coeffs(d: usize, coef: &[u64]) -> Form {
        let mons = monomials(d);
        assert_eq!(mons.len(), coef.len());
        let c = mons
            .iter()
            .zip(coef)
            .filter(|(_, &c)| c != 0)
            .map(|(m, &c)| (*m, c))
            .collect();
        Form { d, c }
    }
    pub fn coeffs(&self) -> Vec<u64> {
        monomials(self.d)
            .iter()
            .map(|m| *self.c.get(m).unwrap_or(&0))
            .collect()
    }
    pub fn mul(&self, o: &Form, p: u64) -> Form {
        let mut c: BTreeMap<[u8; 3], u64> = BTreeMap::new();
        for (m1, &a) in &self.c {
            for (m2, &b) in &o.c {
                let m = [m1[0] + m2[0], m1[1] + m2[1], m1[2] + m2[2]];
                let e = c.entry(m).or_insert(0);
                *e = ad(*e, md(a, b, p), p);
            }
        }
        c.retain(|_, v| *v != 0);
        Form {
            d: self.d + o.d,
            c,
        }
    }
    /// `f(M (x, y, w)^T)`: the form in the new coordinates.
    pub fn subst(&self, m: &[[u64; 3]; 3], p: u64) -> Form {
        let lin: Vec<Form> = (0..3)
            .map(|a| Form {
                d: 1,
                c: [[1u8, 0, 0], [0, 1, 0], [0, 0, 1]]
                    .iter()
                    .enumerate()
                    .filter(|(b, _)| m[a][*b] != 0)
                    .map(|(b, mm)| (*mm, m[a][b]))
                    .collect(),
            })
            .collect();
        let mut res = Form {
            d: self.d,
            c: BTreeMap::new(),
        };
        for (mon, &coef) in &self.c {
            let mut t = Form {
                d: 0,
                c: [([0u8, 0, 0], coef)].into_iter().collect(),
            };
            for a in 0..3 {
                for _ in 0..mon[a] {
                    t = t.mul(&lin[a], p);
                }
            }
            for (mm, v) in t.c {
                let e = res.c.entry(mm).or_insert(0);
                *e = ad(*e, v, p);
            }
        }
        res.c.retain(|_, v| *v != 0);
        res
    }
    /// Partial derivative in variable `v` (0 = X, 1 = Y, 2 = Z).
    pub fn deriv(&self, v: usize, p: u64) -> Form {
        let mut c = BTreeMap::new();
        for (m, &a) in &self.c {
            if m[v] > 0 {
                let mut mm = *m;
                mm[v] -= 1;
                let cf = md(a, m[v] as u64 % p, p);
                if cf != 0 {
                    c.insert(mm, cf);
                }
            }
        }
        Form {
            d: self.d.saturating_sub(1),
            c,
        }
    }
    /// `w = 1`: coefficients by power of `y`, each a polynomial in `x`.
    pub fn to_biv(&self) -> Vec<Poly> {
        let mut out: Vec<Poly> = vec![Vec::new(); self.d + 1];
        for (m, &a) in &self.c {
            let (i, j) = (m[0] as usize, m[1] as usize);
            if out[j].len() <= i {
                out[j].resize(i + 1, 0);
            }
            out[j][i] = a;
        }
        for q in out.iter_mut() {
            ptrim(q);
        }
        out
    }
    pub fn eval_prime(&self, pt: [u64; 3], p: u64) -> u64 {
        let mut s = 0;
        for (m, &a) in &self.c {
            let mut t = a;
            for v in 0..3 {
                t = md(t, pw(pt[v], m[v] as u64, p), p);
            }
            s = ad(s, t, p);
        }
        s
    }
}

fn mat3_inv(m: &[[u64; 3]; 3], p: u64) -> Option<[[u64; 3]; 3]> {
    let det = |a: &[[u64; 3]; 3]| -> u64 {
        let t1 = md(a[0][0], sb(md(a[1][1], a[2][2], p), md(a[1][2], a[2][1], p), p), p);
        let t2 = md(a[0][1], sb(md(a[1][0], a[2][2], p), md(a[1][2], a[2][0], p), p), p);
        let t3 = md(a[0][2], sb(md(a[1][0], a[2][1], p), md(a[1][1], a[2][0], p), p), p);
        ad(sb(t1, t2, p), t3, p)
    };
    let d = det(m);
    if d == 0 {
        return None;
    }
    let di = inv(d, p);
    let mut r = [[0u64; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            // cofactor (j, i)
            let (r0, r1) = ((j + 1) % 3, (j + 2) % 3);
            let (c0, c1) = ((i + 1) % 3, (i + 2) % 3);
            let cof = sb(md(m[r0][c0], m[r1][c1], p), md(m[r0][c1], m[r1][c0], p), p);
            r[i][j] = md(cof, di, p);
        }
    }
    Some(r)
}

fn mat3_vec(m: &[[u64; 3]; 3], v: &[u64; 3], p: u64) -> [u64; 3] {
    let mut r = [0u64; 3];
    for i in 0..3 {
        for j in 0..3 {
            r[i] = ad(r[i], md(m[i][j], v[j], p), p);
        }
    }
    r
}

/// Resultant in `y` of two polynomials in `y` with coefficients in `F_p[x]`
/// (Bareiss on the Sylvester matrix, formal degrees = `len - 1`).
pub fn res_y(a: &[Poly], b: &[Poly], p: u64) -> Poly {
    let (da, db) = (a.len() - 1, b.len() - 1);
    let n = da + db;
    if n == 0 {
        return vec![1];
    }
    let mut mat: Vec<Vec<Poly>> = vec![vec![Vec::new(); n]; n];
    for i in 0..db {
        for (j, c) in a.iter().rev().enumerate() {
            mat[i][i + j] = c.clone();
        }
    }
    for i in 0..da {
        for (j, c) in b.iter().rev().enumerate() {
            mat[db + i][i + j] = c.clone();
        }
    }
    let mut prev: Poly = vec![1];
    let mut sign = 1u64;
    for k in 0..n - 1 {
        if mat[k][k].is_empty() {
            let Some(i) = (k + 1..n).find(|&i| !mat[i][k].is_empty()) else {
                return Vec::new();
            };
            mat.swap(k, i);
            sign = p - sign;
        }
        let pivot = mat[k][k].clone();
        for i in k + 1..n {
            for j in k + 1..n {
                let t = psub(
                    &pmul(&mat[i][j], &pivot, p),
                    &pmul(&mat[i][k], &mat[k][j], p),
                    p,
                );
                let (q, r) = pdivrem(&t, &prev, p);
                debug_assert!(r.is_empty(), "Bareiss division not exact");
                mat[i][j] = q;
            }
        }
        prev = pivot;
    }
    let det = mat[n - 1][n - 1].clone();
    pscale(&det, sign % p, p)
}

// ---------------------------------------------------------------------------
// The plane quartic: fitting, smoothness, point counts, L-polynomial.

/// Rows of the linear conditions "a form with monomials `mons` vanishes at the
/// closed point": `m` rows over `F_p` (coefficients of the reduction mod `mu`).
pub fn cond_rows(cp: &ClosedPt, mons: &[[u8; 3]], p: u64) -> Vec<Vec<u64>> {
    let m = cp.degree();
    let mut pws: [Vec<Poly>; 3] = [vec![vec![1]], vec![vec![1]], vec![vec![1]]];
    let maxd = mons.iter().map(|x| x.iter().map(|&e| e as usize).sum::<usize>()).max().unwrap_or(0);
    for a in 0..3 {
        for e in 1..=maxd {
            let nx = prem(&pmul(&pws[a][e - 1], &cp.co[a], p), &cp.mu, p);
            pws[a].push(nx);
        }
    }
    let vals: Vec<Poly> = mons
        .iter()
        .map(|mo| {
            let t = pmul(&pws[0][mo[0] as usize], &pws[1][mo[1] as usize], p);
            let t = pmul(&t, &pws[2][mo[2] as usize], p);
            prem(&t, &cp.mu, p)
        })
        .collect();
    (0..m)
        .map(|t| vals.iter().map(|v| *v.get(t).unwrap_or(&0)).collect())
        .collect()
}

/// Rows for a whole divisor (reduced).
pub fn div_rows(dv: &Div, mons: &[[u8; 3]], p: u64) -> Vec<Vec<u64>> {
    let mut rows = Vec::new();
    for (cp, &mult) in dv {
        assert_eq!(mult, 1, "div_rows needs a multiplicity-free divisor");
        rows.extend(cond_rows(cp, mons, p));
    }
    rows
}

/// A smooth (to be checked) plane quartic over `F_p`.
#[derive(Clone, Debug)]
pub struct Quartic {
    pub p: u64,
    pub f: Form,
}

#[derive(Clone, Debug, Default)]
pub struct FitStats {
    pub divisors_used: usize,
    pub holdout_points: usize,
    pub holdout_failures: usize,
    pub nullity: usize,
    pub conic_through: bool,
    pub degenerate: usize,
}

impl Smith {
    /// Fit the plane quartic through the images `psi(D)` of random divisors.
    /// `Err` carries a reason (nullity > 1: `X` hyperelliptic or too few points).
    pub fn fit_quartic(
        &self,
        rng: &mut SplitMix64,
        n_holdout: usize,
    ) -> Result<(Quartic, FitStats), (String, FitStats)> {
        let p = self.sd.p;
        let mons4 = monomials(4);
        let mons2 = monomials(2);
        let mut st = FitStats::default();
        let mut rows: Vec<Vec<u64>> = Vec::new();
        let mut rows2: Vec<Vec<u64>> = Vec::new();
        let mut seen: std::collections::BTreeSet<ClosedPt> = Default::default();
        let mut mat = Vec::new();
        for it in 0..400 {
            let d = self.h.random_generic3(rng);
            let Ok(img) = self.psi_effective(&d, rng) else {
                st.degenerate += 1;
                continue;
            };
            for cp in img.keys() {
                if seen.insert(cp.clone()) {
                    rows.extend(cond_rows(cp, &mons4, p));
                    rows2.extend(cond_rows(cp, &mons2, p));
                }
            }
            st.divisors_used = it + 1;
            if it >= 8 && it % 4 == 0 && rows.len() >= 14 {
                mat = rows.clone();
                let ns = nullspace(&mat, 15, p);
                st.nullity = ns.len();
                if ns.len() == 1 {
                    break;
                }
            }
        }
        let ns = nullspace(&rows, 15, p);
        st.nullity = ns.len();
        st.conic_through = !nullspace(&rows2, 6, p).is_empty();
        if ns.len() != 1 {
            return Err((
                if st.conic_through {
                    "images lie on a conic: X hyperelliptic".into()
                } else {
                    format!("quartic nullspace has dimension {}", ns.len())
                },
                st,
            ));
        }
        let mut c = ns[0].clone();
        let lead = c.iter().position(|&x| x != 0).unwrap();
        let li = inv(c[lead], p);
        for x in c.iter_mut() {
            *x = md(*x, li, p);
        }
        let q = Quartic {
            p,
            f: Form::from_coeffs(4, &c),
        };
        // hold-out: images of fresh divisors must lie on the quartic
        let mut got = 0;
        let mut tries = 0;
        while got < n_holdout && tries < 20 * n_holdout + 50 {
            tries += 1;
            let d = self.h.random_generic3(rng);
            let Ok(img) = self.psi_effective(&d, rng) else {
                continue;
            };
            for cp in img.keys() {
                if seen.contains(cp) {
                    continue;
                }
                st.holdout_points += 1;
                let r = cond_rows(cp, &mons4, p);
                let ok = r.iter().all(|row| {
                    row.iter()
                        .zip(&c)
                        .fold(0, |a, (&x, &y)| ad(a, md(x, y, p), p))
                        == 0
                });
                if !ok {
                    st.holdout_failures += 1;
                }
            }
            got += 1;
        }
        Ok((q, st))
    }
}

impl Quartic {
    /// Certify smoothness.  In a random frame the affine singular points are
    /// common roots of `Res_y(f, f_y)` and `Res_y(f, f_x)`; a frame with
    /// `gcd = 1` has no affine singular point.  Two certifying frames leave only
    /// the intersection point of their lines at infinity to check directly.
    /// Returns the number of frames tried.
    pub fn certify_smooth(&self, rng: &mut SplitMix64, max_frames: usize) -> Option<usize> {
        let p = self.p;
        let mut good: Vec<[[u64; 3]; 3]> = Vec::new();
        for t in 1..=max_frames {
            let m = random_frame(p, rng);
            let g = self.f.subst(&m, p);
            let trim_tail = |mut v: Vec<Poly>| {
                while v.last().is_some_and(|c| c.is_empty()) {
                    v.pop();
                }
                v
            };
            let bg = g.to_biv();
            let bfy = trim_tail(g.deriv(1, p).to_biv());
            let bfx = trim_tail(g.deriv(0, p).to_biv());
            if bg.len() != 5 || bg[4].len() != 1 || bfy.len() != 4 || bfx.is_empty() {
                continue;
            }
            let r1 = res_y(&bg, &bfy, p);
            let r2 = res_y(&bg, &bfx, p);
            if r1.is_empty() || r2.is_empty() {
                continue;
            }
            if pdeg(&pgcd(&r1, &r2, p)) == 0 {
                good.push(m);
                if good.len() == 2 {
                    let (i1, i2) = (mat3_inv(&good[0], p)?, mat3_inv(&good[1], p)?);
                    let s0 = cross(&i1[2], &i2[2], p);
                    if s0 == [0, 0, 0] {
                        good.pop();
                        continue;
                    }
                    let sing = self.f.eval_prime(s0, p) == 0
                        && (0..3).all(|v| self.f.deriv(v, p).eval_prime(s0, p) == 0);
                    return if sing { None } else { Some(t) };
                }
            }
        }
        None
    }

    /// `#C(F_{p^k})`, projective points, by enumeration.
    pub fn count_points(&self, k: usize) -> u64 {
        let p = self.p;
        let fld = Fld::new(p, k, 0xC0DE);
        let size = p.pow(k as u32);
        let bf = self.f.to_biv(); // z = 1 chart via symmetric roles below
        let _ = bf;
        // chart w = 1: poly in y for each x
        let mons = monomials(4);
        let coef = self.f.coeffs();
        let eval_poly_y = |x: &El, z: &El| -> Vec<El> {
            // coefficients in y of f(x, y, z)
            let mut out = vec![fld.zero(); 5];
            for (mo, &c) in mons.iter().zip(&coef) {
                if c == 0 {
                    continue;
                }
                let t = fld.mul(
                    &fld.pow_u64(x, mo[0] as u64),
                    &fld.pow_u64(z, mo[2] as u64),
                );
                out[mo[1] as usize] = fld.add(&out[mo[1] as usize], &fld.scale(&t, c));
            }
            out
        };
        let nroots = |poly: &[El]| -> u64 {
            let mut g: KP = poly.to_vec();
            ktrim(&fld, &mut g);
            if g.is_empty() {
                return u64::MAX; // identically zero (should not happen)
            }
            if g.len() == 1 {
                return 0;
            }
            // y^size mod g
            let mut r: KP = vec![fld.one()];
            let mut b: KP = kdivrem(&fld, &[fld.zero(), fld.one()], &g).1;
            let mut e = size;
            while e > 0 {
                if e & 1 == 1 {
                    r = kmulmod(&fld, &r, &b, &g);
                }
                b = kmulmod(&fld, &b, &b, &g);
                e >>= 1;
            }
            // r - y
            let mut ry = r;
            if ry.len() < 2 {
                ry.resize(2, fld.zero());
            }
            ry[1] = fld.sub(&ry[1], &fld.one());
            ktrim(&fld, &mut ry);
            if ry.is_empty() {
                return (g.len() - 1) as u64;
            }
            let gg = kgcd(&fld, &g, &ry);
            (gg.len() - 1) as u64
        };
        let mut total = 0u64;
        let one = fld.one();
        // enumerate field elements
        let elems: Vec<El> = (0..size)
            .map(|mut i| {
                let mut cs = vec![0u64; k];
                for c in cs.iter_mut() {
                    *c = i % p;
                    i /= p;
                }
                fld.from_u64s(&cs)
            })
            .collect();
        for x in &elems {
            total += nroots(&eval_poly_y(x, &one));
        }
        // line w = 0: (1 : y : 0) and (0 : 1 : 0)
        total += nroots(&eval_poly_y(&one, &fld.zero()));
        if self.f.eval_prime([0, 1, 0], p) == 0 {
            total += 1;
        }
        total
    }

    /// All `F_p`-rational points.
    pub fn rational_points(&self) -> Vec<[u64; 3]> {
        let p = self.p;
        let mut v = Vec::new();
        for x in 0..p {
            for y in 0..p {
                if self.f.eval_prime([x, y, 1], p) == 0 {
                    v.push([x, y, 1]);
                }
            }
        }
        for y in 0..p {
            if self.f.eval_prime([1, y, 0], p) == 0 {
                v.push([1, y, 0]);
            }
        }
        if self.f.eval_prime([0, 1, 0], p) == 0 {
            v.push([0, 1, 0]);
        }
        v
    }
}

pub fn random_frame(p: u64, rng: &mut SplitMix64) -> [[u64; 3]; 3] {
    loop {
        let mut m = [[0u64; 3]; 3];
        for r in m.iter_mut() {
            for c in r.iter_mut() {
                *c = rng.below(p);
            }
        }
        if mat3_inv(&m, p).is_some() {
            return m;
        }
    }
}

/// `#H(F_{p^k})` for `y^2 = F(x)`, `deg F` 7 or 8.
pub fn h_count_ext(f: &[u64], p: u64, k: usize) -> u64 {
    let fld = Fld::new(p, k, 0xC0DE);
    let size = p.pow(k as u32);
    let mut total = 0i64;
    for mut i in 0..size {
        let mut cs = vec![0u64; k];
        for c in cs.iter_mut() {
            *c = i % p;
            i /= p;
        }
        let x = fld.from_u64s(&cs);
        let fx = fld.eval(f, &x);
        total += if fld.is_zero(&fx) {
            1
        } else if fld.is_square(&fx) {
            2
        } else {
            0
        };
    }
    let d = pdeg(f);
    total += if d == 8 {
        let lc = fld.c(*f.last().unwrap());
        if fld.is_square(&lc) {
            2
        } else {
            0
        }
    } else {
        1
    };
    total as u64
}

/// `L(T) = 1 + c1 T + ... + p^3 T^6` coefficients `[c0..c6]` from `N1, N2, N3`
/// of a genus-3 curve over `F_p`; and `#J(F_p) = L(1)`.
pub fn lpoly_from_counts(p: u64, n: [u64; 3]) -> ([i128; 7], i128) {
    let pp = p as i128;
    let s: [i128; 3] = [
        pp + 1 - n[0] as i128,
        pp * pp + 1 - n[1] as i128,
        pp * pp * pp + 1 - n[2] as i128,
    ];
    let e1 = s[0];
    let e2 = (s[0] * s[0] - s[1]) / 2;
    let e3 = (s[0] * s[0] * s[0] - 3 * s[0] * s[1] + 2 * s[2]) / 6;
    // prod (1 - a_i T) = 1 - e1 T + e2 T^2 - e3 T^3 + p e2 T^4 - p^2 e1 T^5 + p^3 T^6
    let c = [1, -e1, e2, -e3, pp * e2, -pp * pp * e1, pp * pp * pp];
    let order: i128 = c.iter().sum();
    (c, order)
}

// ---------------------------------------------------------------------------
// Linear equivalence and group law on Pic^0(C)(F_p) by adjoint (cubic)
// reduction.  A class `x` is represented by the effective degree-3 divisor
// `a` with `a ~ x + Omega`, `Omega = H - P0` (the residual of a rational line
// through a rational point `P0`); `a` is unique unless `h^0(a) = 2`.

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PicErr {
    /// No frame with all intersection points affine and separated by `x`.
    NoFrame,
    /// `h^0 = 2`: the degree-3 representative is not unique.
    Exceptional,
    /// A residual divisor has a repeated point (not supported).
    NonReduced,
    /// Supports overlap in a way the method cannot handle.
    Overlap,
    NoPoint,
    Other(String),
}

pub struct Pic {
    pub q: Quartic,
    pub p0: ClosedPt,
    pub omega: Div,
    tangent: [u64; 3],
    rng: std::cell::RefCell<SplitMix64>,
    pub n_intersections: std::cell::Cell<u64>,
    pub n_frames: std::cell::Cell<u64>,
}

fn ring_minpoly(mu: &[u64], a: &[u64], p: u64) -> Poly {
    // smallest dependency among 1, a, a^2, ... modulo mu
    let m = mu.len() - 1;
    let mut vecs: Vec<Vec<u64>> = Vec::new();
    let mut cur: Poly = vec![1];
    for t in 0..=m {
        let mut v = cur.clone();
        v.resize(m, 0);
        // solve v = sum_{s<t} lam_s vecs[s]
        let cols = t;
        let mut mat: Vec<Vec<u64>> = (0..m)
            .map(|r| {
                let mut row: Vec<u64> = (0..cols).map(|s| vecs[s][r]).collect();
                row.push(v[r]);
                row
            })
            .collect();
        let piv = rref(&mut mat, p);
        if !piv.contains(&cols) {
            // v in span: a^t = sum lam_s a^s
            let mut lam = vec![0u64; t];
            for (ri, &pc) in piv.iter().enumerate() {
                lam[pc] = mat[ri][cols];
            }
            let mut mp: Poly = lam.iter().map(|&l| ng(l, p)).collect();
            mp.push(1);
            return mp;
        }
        vecs.push(v);
        cur = prem(&pmul(&cur, a, p), mu, p);
    }
    unreachable!("minimal polynomial must have degree <= m")
}

fn poly_inv_mod(a: &[u64], mu: &[u64], p: u64) -> Option<Poly> {
    let (g, s, _) = pxgcd(a, mu, p);
    (pdeg(&g) == 0).then(|| prem(&s, mu, p))
}

fn cross(a: &[u64; 3], b: &[u64; 3], p: u64) -> [u64; 3] {
    [
        sb(md(a[1], b[2], p), md(a[2], b[1], p), p),
        sb(md(a[2], b[0], p), md(a[0], b[2], p), p),
        sb(md(a[0], b[1], p), md(a[1], b[0], p), p),
    ]
}

fn ring_mul(a: &[u64], b: &[u64], mu: &[u64], p: u64) -> Poly {
    prem(&pmul(a, b, p), mu, p)
}

/// Value of a form at a point with coordinates in the ring `F_p[c]/mu`.
fn eval_form_ring(f: &Form, co: &[Poly; 3], mu: &[u64], p: u64) -> Poly {
    let mut acc: Poly = Vec::new();
    for (m, &a) in &f.c {
        let mut t: Poly = vec![a];
        for v in 0..3 {
            for _ in 0..m[v] {
                t = ring_mul(&t, &co[v], mu, p);
            }
        }
        acc = padd(&acc, &t, p);
    }
    prem(&acc, mu, p)
}

/// Squarefree decomposition `f = prod g_e^e` (Yun; needs `p > deg f`).
fn sqfree_decomp(f: &[u64], p: u64) -> Vec<(Poly, u32)> {
    let f = pmonic(f, p);
    let mut out = Vec::new();
    if pdeg(&f) <= 0 {
        return out;
    }
    let fd = pderiv(&f, p);
    let c = pgcd(&f, &fd, p);
    let mut w = pdivrem(&f, &c, p).0;
    let mut y = pdivrem(&fd, &c, p).0;
    let mut z = psub(&y, &pderiv(&w, p), p);
    let mut i = 1u32;
    while pdeg(&w) > 0 {
        let g = pgcd(&w, &z, p);
        if pdeg(&g) > 0 {
            out.push((g.clone(), i));
        }
        w = pdivrem(&w, &g, p).0;
        y = pdivrem(&z, &g, p).0;
        z = psub(&y, &pderiv(&w, p), p);
        i += 1;
    }
    out
}

pub fn div_max_mult(d: &Div) -> u32 {
    d.values().copied().max().unwrap_or(0)
}


impl Pic {
    pub fn new(q: Quartic, seed: u64) -> Result<Pic, PicErr> {
        let p = q.p;
        let pts = q.rational_points();
        let Some(&pt) = pts.first() else {
            return Err(PicErr::NoPoint);
        };
        let co = [0, 1, 2].map(|a| {
            let mut v = vec![pt[a]];
            ptrim(&mut v);
            v
        });
        // normalise the first nonzero coordinate to 1
        let lead = pt.iter().position(|&x| x != 0).unwrap();
        let li = inv(pt[lead], p);
        let ptn = [md(pt[0], li, p), md(pt[1], li, p), md(pt[2], li, p)];
        let _ = co;
        let p0 = ClosedPt {
            mu: vec![0, 1],
            co: [0, 1, 2].map(|a| {
                let mut v = vec![ptn[a]];
                ptrim(&mut v);
                v
            }),
        };
        // tangent direction: n = grad f(P0); t in ker n, not proportional to P0
        let n: [u64; 3] = [0, 1, 2].map(|v| q.f.deriv(v, p).eval_prime(ptn, p));
        if n == [0, 0, 0] {
            return Err(PicErr::Other("P0 singular".into()));
        }
        let mut tangent = None;
        for cand in [[1u64, 0, 0], [0, 1, 0], [0, 0, 1]] {
            let t = cross(&n, &cand, p);
            if t != [0, 0, 0] && cross(&t, &ptn, p) != [0, 0, 0] {
                tangent = Some(t);
                break;
            }
        }
        let tangent = tangent.ok_or(PicErr::Other("no tangent direction".into()))?;
        let pic = Pic {
            q,
            p0,
            omega: Div::new(),
            tangent,
            rng: std::cell::RefCell::new(SplitMix64(seed)),
            n_intersections: Default::default(),
            n_frames: Default::default(),
        };
        // Omega: residual of a random rational line through P0
        let mut pic = pic;
        for _ in 0..400 {
            let qq = [
                pic.rnd(p),
                pic.rnd(p),
                pic.rnd(p),
            ];
            let l = cross(&ptn, &qq, p);
            if l == [0, 0, 0] {
                continue;
            }
            let form = Form::from_coeffs(1, &l);
            let Ok(r) = pic.intersect_residual(&form, &[(&pic.p0, 1)]) else {
                continue;
            };
            if div_degree(&r) == 3 && div_is_reduced(&r) && !r.contains_key(&pic.p0) {
                pic.omega = r;
                return Ok(pic);
            }
        }
        Err(PicErr::Other("no suitable line for Omega".into()))
    }

    fn x_min_poly(&self, cp: &ClosedPt, minv: &[[u64; 3]; 3]) -> Option<Poly> {
        let p = self.q.p;
        let mu = &cp.mu;
        let comp = |r: usize| -> Poly {
            let mut acc: Poly = Vec::new();
            for c in 0..3 {
                acc = padd(&acc, &pscale(&cp.co[c], minv[r][c], p), p);
            }
            prem(&acc, mu, p)
        };
        let (xx, ww) = (comp(0), comp(2));
        let wi = poly_inv_mod(&ww, mu, p)?;
        let x = prem(&pmul(&xx, &wi, p), mu, p);
        let mp = ring_minpoly(mu, &x, p);
        (mp.len() == mu.len()).then_some(mp)
    }

    /// Conditions "g vanishes to order >= 2 along C at the closed point".
    fn tangent_rows(&self, cp: &ClosedPt, mons: &[[u8; 3]]) -> Vec<Vec<u64>> {
        let p = self.q.p;
        let mu = &cp.mu;
        let m = cp.degree();
        let n: Vec<Poly> = (0..3)
            .map(|v| eval_form_ring(&self.q.f.deriv(v, p), &cp.co, mu, p))
            .collect();
        let crs = |a: &[Poly], b: &[Poly]| -> Vec<Poly> {
            (0..3)
                .map(|i| {
                    let (j, k) = ((i + 1) % 3, (i + 2) % 3);
                    psub(
                        &ring_mul(&a[j], &b[k], mu, p),
                        &ring_mul(&a[k], &b[j], mu, p),
                        p,
                    )
                })
                .collect()
        };
        let mut tvec: Option<Vec<Poly>> = None;
        for e in 0..3 {
            let mut ev: Vec<Poly> = vec![Vec::new(); 3];
            ev[e] = vec![1];
            let t = crs(&n, &ev);
            if t.iter().any(|x| !x.is_empty()) {
                let c = crs(&t, &cp.co);
                if c.iter().any(|x| !x.is_empty()) {
                    tvec = Some(t);
                    break;
                }
            }
        }
        let t = tvec.expect("no tangent direction at a smooth point");
        // value of D_t mon = sum_v t_v * d mon/dX_v at P
        let vals: Vec<Poly> = mons
            .iter()
            .map(|mo| {
                let mut acc: Poly = Vec::new();
                for v in 0..3 {
                    if mo[v] == 0 {
                        continue;
                    }
                    let mut term: Poly = vec![mo[v] as u64 % p];
                    for w in 0..3 {
                        let e = if w == v { mo[w] - 1 } else { mo[w] };
                        for _ in 0..e {
                            term = ring_mul(&term, &cp.co[w], mu, p);
                        }
                    }
                    term = ring_mul(&term, &t[v], mu, p);
                    acc = padd(&acc, &term, p);
                }
                prem(&acc, mu, p)
            })
            .collect();
        (0..m)
            .map(|t_| vals.iter().map(|v| *v.get(t_).unwrap_or(&0)).collect())
            .collect()
    }

    /// Vanishing conditions of a divisor with multiplicities up to 2.
    pub fn rows_for(&self, dv: &Div, mons: &[[u8; 3]]) -> Result<Vec<Vec<u64>>, PicErr> {
        let p = self.q.p;
        let mut rows = Vec::new();
        for (cp, &mult) in dv {
            rows.extend(cond_rows(cp, mons, p));
            match mult {
                1 => {}
                2 => rows.extend(self.tangent_rows(cp, mons)),
                _ => return Err(PicErr::NonReduced),
            }
        }
        Ok(rows)
    }

    /// The residual of `V(g) cap C` after removing `known` (with multiplicities).
    pub fn intersect_residual(
        &self,
        g: &Form,
        known: &[(&ClosedPt, u32)],
    ) -> Result<Div, PicErr> {
        let p = self.q.p;
        self.n_intersections.set(self.n_intersections.get() + 1);
        let d = g.d;
        let mut nonsf = 0;
        'frames: for _ in 0..400 {
            self.n_frames.set(self.n_frames.get() + 1);
            let m = random_frame(p, &mut self.rng.borrow_mut());
            let minv = mat3_inv(&m, p).unwrap();
            let f2 = self.q.f.subst(&m, p);
            let g2 = g.subst(&m, p);
            let (bf, bg) = (f2.to_biv(), g2.to_biv());
            if bf[4].len() != 1 || bg.len() != d + 1 || bg[d].len() != 1 {
                continue;
            }
            let r = res_y(&bf, &bg, p);
            if pdeg(&r) != 4 * d as isize {
                continue;
            }
            let mut rem = r;
            for (cp, mult) in known {
                let Some(xp) = self.x_min_poly(cp, &minv) else {
                    continue 'frames;
                };
                for _ in 0..*mult {
                    let (qq, rr) = pdivrem(&rem, &xp, p);
                    if !rr.is_empty() {
                        return Err(PicErr::Other("known point not on the intersection".into()));
                    }
                    rem = qq;
                }
            }
            if pdeg(&rem) <= 0 {
                return Ok(Div::new());
            }
            let mut rng = SplitMix64(0xFAC7 ^ rem.len() as u64);
            let mut factors: Vec<(Poly, u32)> = Vec::new();
            for (sq, e) in sqfree_decomp(&rem, p) {
                if e > 2 {
                    return Err(PicErr::NonReduced);
                }
                let Some(fs) = factor_squarefree(&sq, p, &mut rng) else {
                    nonsf += 1;
                    continue 'frames;
                };
                for f in fs {
                    factors.push((f, e));
                }
            }
            let mut out = Div::new();
            for (fr, mult) in &factors {
                let mr = fr.len() - 1;
                if mr > 8 {
                    continue 'frames;
                }
                let k = Fld::with_modulus(p, fr);
                let x0 = if mr == 1 { k.c(ng(fr[0], p)) } else { k.z() };
                let fy: KP = bf.iter().map(|c| k.eval(c, &x0)).collect();
                let gy: KP = bg.iter().map(|c| k.eval(c, &x0)).collect();
                let gc = kgcd(&k, &fy, &gy);
                if gc.len() != 2 {
                    continue 'frames;
                }
                let y0 = k.neg(&gc[0]);
                let v = [x0, y0, k.one()];
                let mut kap = [k.zero(), k.zero(), k.zero()];
                for a in 0..3 {
                    for b in 0..3 {
                        kap[a] = k.add(&kap[a], &k.scale(&v[b], m[a][b]));
                    }
                }
                let _ = minv;
                *out.entry(closed_pt_from(&k, &kap)).or_insert(0) += *mult;
            }
            return Ok(out);
        }
        if nonsf > 100 {
            Err(PicErr::NonReduced)
        } else {
            Err(PicErr::NoFrame)
        }
    }

    fn rnd(&self, p: u64) -> u64 {
        self.rng.borrow_mut().below(p)
    }

    fn random_combo(&self, basis: &[Vec<u64>]) -> Vec<u64> {
        let p = self.q.p;
        let n = basis[0].len();
        let mut r = vec![0u64; n];
        loop {
            for b in basis {
                let c = self.rnd(p);
                for i in 0..n {
                    r[i] = ad(r[i], md(c, b[i], p), p);
                }
            }
            if r.iter().any(|&x| x != 0) {
                return r;
            }
        }
    }

    /// Elements are `Div`s: a reduced degree-3 divisor `a` (not collinear) for
    /// the class `[a - Omega]`, or a rational point `Q` (degree 1) for the
    /// exceptional class `[P0 - Q] = [(H - Q) - Omega]`, whose degree-3
    /// representatives form a pencil.  `zero` is `{P0}`.
    pub fn zero(&self) -> Div {
        [(self.p0.clone(), 1)].into_iter().collect()
    }
    pub fn is_zero(&self, a: &Div) -> bool {
        *a == self.zero()
    }

    /// Turn an effective degree-3 divisor into an element: a collinear
    /// divisor `a = ell.C - Q` becomes `Q`.
    pub fn finish(&self, a: Div) -> Result<Div, PicErr> {
        let p = self.q.p;
        if div_degree(&a) != 3 || div_max_mult(&a) > 2 {
            return Err(PicErr::Other("finish: need a divisor of degree 3, multiplicities <= 2".into()));
        }
        let mons1 = monomials(1);
        let ns = nullspace(&self.rows_for(&a, &mons1)?, 3, p);
        if ns.is_empty() {
            return Ok(a);
        }
        let ell = Form::from_coeffs(1, &ns[0]);
        let kn: Vec<(&ClosedPt, u32)> = a.iter().map(|(c, &m)| (c, m)).collect();
        let q = self.intersect_residual(&ell, &kn)?;
        if div_degree(&q) != 1 {
            return Err(PicErr::Other("finish: residual of a line is not a point".into()));
        }
        Ok(q)
    }

    /// A degree-3 representative of the class (`x + Omega`), avoiding `avoid`.
    fn to_rep(&self, x: &Div, avoid: &Div) -> Result<Div, PicErr> {
        let p = self.q.p;
        if div_degree(x) == 3 {
            return Ok(x.clone());
        }
        let q = x.keys().next().unwrap();
        let qc: [u64; 3] = [0, 1, 2].map(|i| *q.co[i].first().unwrap_or(&0));
        for _ in 0..200 {
            let r = [self.rnd(p), self.rnd(p), self.rnd(p)];
            let l = cross(&qc, &r, p);
            if l == [0, 0, 0] {
                continue;
            }
            let Ok(res) = self.intersect_residual(&Form::from_coeffs(1, &l), &[(q, 1)]) else {
                continue;
            };
            if div_degree(&res) == 3
                && div_is_reduced(&res)
                && !res.keys().any(|c| avoid.contains_key(c))
            {
                return Ok(res);
            }
        }
        Err(PicErr::Other("to_rep failed".into()))
    }

    /// Element of the class of an effective, reduced divisor `E` of degree 6:
    /// the unique `A` (degree 3) with `E ~ A + Omega`, or the exceptional `Q`.
    pub fn canon6(&self, e: &Div) -> Result<Div, PicErr> {
        let p = self.q.p;
        if div_degree(e) != 6 || div_max_mult(e) > 2 {
            return Err(PicErr::Other("canon6 needs a divisor of degree 6, multiplicities <= 2".into()));
        }
        let mons3 = monomials(3);
        let rows = self.rows_for(e, &mons3)?;
        let ns = nullspace(&rows, 10, p);
        if ns.len() != 4 {
            return Err(PicErr::Other(format!("cubics through E: dim {}", ns.len())));
        }
        let mut last = PicErr::NoFrame;
        for _ in 0..40 {
            let g = Form::from_coeffs(3, &self.random_combo(&ns));
            let kn: Vec<(&ClosedPt, u32)> = e.iter().map(|(c, &m)| (c, m)).collect();
            let r = match self.intersect_residual(&g, &kn) {
                Ok(r) => r,
                Err(er) => {
                    last = er;
                    continue;
                }
            };
            if div_degree(&r) != 6 {
                last = PicErr::Other("canon6: residual of wrong degree".into());
                continue;
            }
            let ro = div_add(&r, &self.omega);
            if div_max_mult(&ro) > 2 {
                last = PicErr::NonReduced;
                continue;
            }
            let rows2 = match self.rows_for(&ro, &mons3) {
                Ok(r) => r,
                Err(er) => {
                    last = er;
                    continue;
                }
            };
            let ns2 = nullspace(&rows2, 10, p);
            if ns2.is_empty() || ns2.len() > 2 {
                return Err(PicErr::Other(format!("cubics through R + Omega: dim {}", ns2.len())));
            }
            let h = Form::from_coeffs(3, &self.random_combo(&ns2));
            let kn2: Vec<(&ClosedPt, u32)> = ro.iter().map(|(c, &m)| (c, m)).collect();
            match self.intersect_residual(&h, &kn2) {
                Ok(a) => {
                    if div_degree(&a) != 3 {
                        return Err(PicErr::Other("canon6: residual of wrong degree".into()));
                    }
                    return self.finish(a);
                }
                Err(er) => {
                    last = er;
                }
            }
        }
        Err(last)
    }

    /// `-x`: the residual of the conic through `a + 2 P0` for a degree-3
    /// representative `a` of `x + Omega`.
    pub fn neg(&self, x: &Div) -> Result<Div, PicErr> {
        self.neg_depth(x, 0)
    }
    fn neg_depth(&self, x: &Div, depth: usize) -> Result<Div, PicErr> {
        let p = self.q.p;
        if self.is_zero(x) {
            return Ok(x.clone());
        }
        let mut avoid = Div::new();
        avoid.insert(self.p0.clone(), 1);
        let a = self.to_rep(x, &avoid)?;
        if a.contains_key(&self.p0) {
            // -x = z - (x + z)
            if depth > 4 {
                return Err(PicErr::Overlap);
            }
            let z = self.random_elem()?;
            let w = self.add(x, &z)?;
            let nw = self.neg_depth(&w, depth + 1)?;
            return self.add(&nw, &z);
        }
        let mons2 = monomials(2);
        let mut rows = self.rows_for(&a, &mons2)?;
        rows.extend(cond_rows(&self.p0, &mons2, p));
        let p0c: [u64; 3] = [0, 1, 2].map(|i| *self.p0.co[i].first().unwrap_or(&0));
        let trow: Vec<u64> = mons2
            .iter()
            .map(|mo| {
                let mut s = 0;
                for v in 0..3 {
                    if mo[v] == 0 {
                        continue;
                    }
                    let mut t = md(mo[v] as u64 % p, self.tangent[v], p);
                    for w in 0..3 {
                        let e = if w == v { mo[w] - 1 } else { mo[w] };
                        t = md(t, pw(p0c[w], e as u64, p), p);
                    }
                    s = ad(s, t, p);
                }
                s
            })
            .collect();
        rows.push(trow);
        let ns = nullspace(&rows, 6, p);
        if ns.len() != 1 {
            return Err(PicErr::Other(format!("conic through a + 2P0: dim {}", ns.len())));
        }
        let qf = Form::from_coeffs(2, &ns[0]);
        let mut kn: Vec<(&ClosedPt, u32)> = a.iter().map(|(c, &m)| (c, m)).collect();
        kn.push((&self.p0, 2));
        let r = self.intersect_residual(&qf, &kn)?;
        if div_degree(&r) != 3 {
            return Err(PicErr::Other("neg: residual of wrong degree".into()));
        }
        self.finish(r)
    }

    /// `x + y`.  Overlapping supports are separated with a random translate:
    /// `x + y = (x + z) + (y - z)`.
    pub fn add(&self, a: &Div, b: &Div) -> Result<Div, PicErr> {
        self.add_depth(a, b, 0)
    }
    fn add_depth(&self, a: &Div, b: &Div, depth: usize) -> Result<Div, PicErr> {
        if self.is_zero(a) {
            return Ok(b.clone());
        }
        if self.is_zero(b) {
            return Ok(a.clone());
        }
        let ra = self.to_rep(a, &Div::new())?;
        let rb = self.to_rep(b, &ra)?;
        let overlap = ra.keys().any(|c| rb.contains_key(c));
        if !overlap {
            return self.canon6(&div_add(&ra, &rb));
        }
        if depth > 6 {
            return Err(PicErr::Overlap);
        }
        let z = self.random_elem()?;
        let nz = self.neg(&z)?;
        let az = self.add_depth(a, &z, depth + 1)?;
        let bz = self.add_depth(b, &nz, depth + 1)?;
        self.add_depth(&az, &bz, depth + 1)
    }
    pub fn sub(&self, a: &Div, b: &Div) -> Result<Div, PicErr> {
        let nb = self.neg(b)?;
        self.add(a, &nb)
    }
    pub fn mul(&self, mut n: u64, a: &Div) -> Result<Div, PicErr> {
        let mut r = self.zero();
        let mut b = a.clone();
        while n > 0 {
            if n & 1 == 1 {
                r = self.add(&r, &b)?;
            }
            n >>= 1;
            if n > 0 {
                b = self.add(&b, &b)?;
            }
        }
        Ok(r)
    }
    /// The class `[E1 - E2]` for effective reduced degree-6 divisors.
    pub fn class_of_diff(&self, plus: &Div, minus: &Div) -> Result<Div, PicErr> {
        let ap = self.canon6(plus)?;
        let am = self.canon6(minus)?;
        self.sub(&ap, &am)
    }
    /// A random element: a degree-3 sub-divisor of a random cubic section.
    pub fn random_elem(&self) -> Result<Div, PicErr> {
        let p = self.q.p;
        for _ in 0..200 {
            let coef: Vec<u64> = (0..10).map(|_| self.rnd(p)).collect();
            if coef.iter().all(|&c| c == 0) {
                continue;
            }
            let Ok(sec) = self.intersect_residual(&Form::from_coeffs(3, &coef), &[]) else {
                continue;
            };
            let cps: Vec<(&ClosedPt, u32)> = sec.iter().map(|(c, &m)| (c, m)).collect();
            let mut chosen = Vec::new();
            'subset: for mask in 1u32..(1 << cps.len()) {
                let mut deg = 0;
                chosen.clear();
                for (i, (c, m)) in cps.iter().enumerate() {
                    if mask >> i & 1 == 1 {
                        if *m != 1 {
                            continue 'subset;
                        }
                        deg += c.degree();
                        chosen.push((*c).clone());
                    }
                }
                if deg == 3 {
                    break;
                }
                chosen.clear();
            }
            if chosen.is_empty() {
                continue;
            }
            let a: Div = chosen.into_iter().map(|c| (c, 1)).collect();
            if a.keys().any(|c| self.omega.contains_key(c)) {
                continue;
            }
            // class of a: [a - Omega]; normalise through canon6(a + Omega)
            if let Ok(r) = self.canon6(&div_add(&a, &self.omega)) {
                return Ok(r);
            }
        }
        Err(PicErr::Other("random_elem failed".into()))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    pub fn example1() -> (Poly, u64) {
        (vec![2, 29, 12, 33, 20, 15, 28, 1], 37)
    }

    fn find_example_map(sds: &[SmithData]) -> &SmithData {
        sds.iter()
            .find(|s| (s.n1, s.n0, s.d1, s.d0) == (16, 22, 32, 18))
            .expect("Smith's Example 1 map")
    }

    fn example_smith() -> Smith {
        let (f, p) = example1();
        let mut rng = SplitMix64(1);
        let sds = smith_data_for_curve(&f, p, &mut rng);
        Smith::new(find_example_map(&sds).clone())
    }

    #[test]
    fn example1_quartic_and_counts() {
        let sm = example_smith();
        let mut rng = SplitMix64(7);
        let (q, st) = sm.fit_quartic(&mut rng, 20).expect("fit");
        println!("fit stats {:?}", st);
        println!("quartic {:?}", q.f.coeffs());
        assert_eq!(st.holdout_failures, 0);
        assert!(st.holdout_points > 20);
        let fr = q.certify_smooth(&mut rng, 50);
        println!("smooth certified in {:?} frames", fr);
        assert!(fr.is_some());
        let p = sm.sd.p;
        let nh = [1, 2, 3].map(|k| h_count_ext(&sm.sd.f, p, k));
        let nc = [1, 2, 3].map(|k| q.count_points(k));
        println!("H counts {:?}  C counts {:?}", nh, nc);
        assert_eq!(nh, nc);
        let (lc, ord) = lpoly_from_counts(p, nc);
        println!("L = {:?}, #J = {}", lc, ord);
        // Smith's Example 1: Z numerator 37^3 T^6 + 4*37^2 T^5 - 6*37 T^4 - 240 T^3 - 6 T^2 + 4 T + 1
        assert_eq!(lc, [1, 4, -6, -240, -222, 5476, 50653]);
    }

    fn example_pic() -> (Smith, Quartic, Pic) {
        let sm = example_smith();
        let mut rng = SplitMix64(7);
        let (q, _) = sm.fit_quartic(&mut rng, 5).expect("fit");
        let pic = Pic::new(q.clone(), 99).expect("pic");
        (sm, q, pic)
    }

    #[test]
    fn example1_pic_group_law() {
        let (_sm, _q, pic) = example_pic();
        println!("P0 = {:?}\nOmega = {:?}", pic.p0, pic.omega);
        let mut errs: BTreeMap<String, u32> = BTreeMap::new();
        let mut ok = 0;
        let mut note = |e: PicErr| *errs.entry(format!("{:?}", e)).or_insert(0) += 1;
        for _ in 0..6 {
            let (Ok(a), Ok(b), Ok(c)) = (pic.random_elem(), pic.random_elem(), pic.random_elem()) else {
                continue;
            };
            let ab = pic.add(&a, &b);
            let ba = pic.add(&b, &a);
            match (&ab, &ba) {
                (Ok(x), Ok(y)) => assert_eq!(x, y, "commutativity"),
                (Err(e), _) | (_, Err(e)) => {
                    note(e.clone());
                    continue;
                }
            }
            let na = match pic.neg(&a) {
                Ok(x) => x,
                Err(e) => {
                    note(e);
                    continue;
                }
            };
            match pic.add(&a, &na) {
                Ok(z) => assert!(pic.is_zero(&z), "a + (-a) = 0"),
                Err(e) => note(e),
            }
            if let Ok(nn) = pic.neg(&na) {
                assert_eq!(nn, a, "-(-a) = a");
            }
            let l = pic.add(&ab.clone().unwrap(), &c);
            let bc = pic.add(&b, &c);
            let r = bc.clone().and_then(|bc| pic.add(&a, &bc));
            match (l, r) {
                (Ok(l), Ok(r)) => {
                    if l != r {
                        println!("ASSOC FAIL a={:?}\n b={:?}\n c={:?}", a, b, c);
                    }
                    assert_eq!(l, r, "associativity");
                    ok += 1;
                }
                (Err(e), _) | (_, Err(e)) => note(e),
            }
            match pic.mul(55666, &a) {
                Ok(z) => assert!(pic.is_zero(&z), "#J * a = 0"),
                Err(e) => note(e),
            }
        }
        println!("assoc ok {ok}, errors {:?}", errs);
        println!("intersections {} frames {}", pic.n_intersections.get(), pic.n_frames.get());
        assert!(ok >= 2);
    }

    #[test]
    fn debug_canon() {
        let (_sm, _q, pic) = example_pic();
        let mut n = 0;
        for t in 0..60u64 {
            let l = [1 + t, 2 * t + 3, 5 + t * t];
            let Ok(sec) = pic.intersect_residual(&Form::from_coeffs(1, &l), &[]) else { continue };
            let cps: Vec<&ClosedPt> = sec.keys().collect();
            let degs: Vec<usize> = cps.iter().map(|c| c.degree()).collect();
            // find subset of degree 3
            let mut chosen = Div::new();
            for mask in 1u32..(1 << cps.len()) {
                let d: usize = (0..cps.len()).filter(|i| mask >> i & 1 == 1).map(|i| degs[i]).sum();
                if d == 3 {
                    for i in 0..cps.len() { if mask >> i & 1 == 1 { chosen.insert(cps[i].clone(), 1); } }
                    break;
                }
            }
            if chosen.is_empty() { continue; }
            n += 1;
            let e = div_add(&chosen, &pic.omega);
            let r = pic.canon6(&e);
            println!("line {:?} degs {:?}: canon6 -> {:?}", l, degs, r.as_ref().map(|d| div_degree(d)));
            if let Ok(a) = &r {
                // a should equal chosen's class: canon6(a + omega) == canon6(chosen + omega)
                println!("   same as chosen? {}", a == &chosen);
            }
            if n >= 6 { break; }
        }
    }
}
