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

    #[test]
    fn spike_example1() {
        let (f, p) = example1();
        let mut rng = SplitMix64(1);
        let sds = smith_data_for_curve(&f, p, &mut rng);
        println!("maps: {}", sds.len());
        for s in &sds {
            println!("{:?} rational={}", (s.n1, s.n0, s.d1, s.d0), s.rational());
        }
        let sd = find_example_map(&sds);
        println!("sigma = {:?}", sd.sigma);
        println!("deltas = {:?}", sd.deltas);
        let l = Fld::new(p, 2, 7);
        let mut cnt = 0;
        for x in 0..p {
            let fx = peval(&f, x, p);
            let Some(y) = crate::cryptanalysis::smith_prop24::sqrt_mod(fx, p) else { continue };
            if y == 0 { continue; }
            let r = sd.psi_point(&l, &l.c(x), &l.c(y));
            println!("P=({x},{y}) -> {:?}", r.as_ref().map(|v| v.len()).map_err(|e| e.clone()));
            if let Ok(v) = r { for q in v { println!("   kappa = {:?}", &q.kappa.iter().map(|c| c[..2].to_vec()).collect::<Vec<_>>()); } }
            cnt += 1;
        }
        assert!(cnt > 0);
    }
}
