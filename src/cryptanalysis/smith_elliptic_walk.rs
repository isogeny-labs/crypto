//! **The Smith walk on the elliptic side** (workstream W3a of
//! `research/plane_quartic_program_20261018/PROGRAM.md`; protocol
//! `research/smith_elliptic_walk_20261018/PROTOCOL.md`).
//!
//! Setting: `K = F_{p^3}`, `sigma = Frob_p`. A *weak model* is
//! `C: y^2 = h(x) (x - alpha)(x - sigma alpha)` with `h in F_p[x]` of degree 1
//! or 2 and `alpha in K \ F_p` (Joux-Vitse Eurocrypt 2012 §4.1 eq. (3) and App.
//! A.1; Momose-Chao ECC 2007 sl. 24). Its Jacobian `E/K` descends to the genus-3
//! hyperelliptic `H: y^2 = 4 N^2 h(F/(4N))` over `F_p` with
//! `L_H(T) = 1 - t T^3 + p^3 T^6` for `t` the trace of `E/K`. Smith's construction (`smith_prop24::analyze_curve`) then either
//! passes (`full_success`) or not on `H`.
//!
//! This module provides
//!
//! * a counted `F_{p^3}` (every `F_p` multiplication is tallied, inversions
//!   apart) with norm, Frobenius and Tonelli-Shanks square roots;
//! * the weak-model enumeration (reduced by `PGL_2(F_p)`, which acts simply
//!   transitively on `P^1(K) \ P^1(F_p)`), the descent, the point-count check of
//!   `L_H`, and the Smith outcome of every `H`;
//! * elliptic curves over `K` as `K`-isomorphism classes, rational 2- and
//!   3-isogenies (Velu), and the component / distance analysis of the graph
//!   restricted to the weak curves (exhaustive, small `p`);
//! * the *JV-weak* test (full rational 2-torsion and a cross-ratio of norm one),
//!   the explicit weak models it gives (Hilbert 90), and the priced random
//!   2-isogeny walk (sampled `p`).
//!
//! Synthetic curves only. No deployed-curve claim.

use super::smith_prop24 as sp;
use serde::Serialize;
use std::cell::Cell;
use std::collections::{HashMap, HashSet, VecDeque};

/// An element `a0 + a1 z + a2 z^2` of `K`.
pub type El = [u64; 3];
pub const ZERO: El = [0, 0, 0];
pub const ONE: El = [1, 0, 0];

// ---------------------------------------------------------------------------
// F_p helpers (free functions, uncounted; the counted ones live in `Fp3`).

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

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn fnv(f: &[u64]) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for &c in f {
        h ^= c;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

// ---------------------------------------------------------------------------
// The counted field K = F_p[z] / (z^3 - m1 z - m0).

pub struct Fp3 {
    pub p: u64,
    pub m0: u64,
    pub m1: u64,
    /// Least quadratic non-residue of `F_p` (a non-square in `K` too).
    pub d: u64,
    zp: El,
    zp2: El,
    /// `F_p` multiplications performed (inversions not included).
    pub mults: Cell<u64>,
    /// `F_p` inversions performed.
    pub invs: Cell<u64>,
}

impl Fp3 {
    pub fn new(p: u64) -> Fp3 {
        assert!(p % 2 == 1 && (5..(1 << 30)).contains(&p), "odd prime 5 <= p < 2^30");
        let (m0, m1) = Self::find_modulus(p);
        let mut k = Fp3 {
            p,
            m0,
            m1,
            d: sp::least_nonsquare(p),
            zp: ZERO,
            zp2: ZERO,
            mults: Cell::new(0),
            invs: Cell::new(0),
        };
        let z = [0, 1, 0];
        k.zp = k.pow(&z, p as u128);
        k.zp2 = k.mul(&k.zp, &k.zp);
        k.reset();
        k
    }

    fn find_modulus(p: u64) -> (u64, u64) {
        for m1 in 0..p {
            for m0 in 1..=16u64.min(p - 1) {
                let root = (0..p).any(|z| {
                    let z3 = md(md(z, z, p), z, p);
                    sb(sb(z3, md(m1, z, p), p), m0, p) == 0
                });
                if !root {
                    return (m0, m1);
                }
            }
        }
        unreachable!("an irreducible z^3 - m1 z - m0 exists")
    }

    pub fn reset(&self) {
        self.mults.set(0);
        self.invs.set(0);
    }

    pub fn size(&self) -> u64 {
        self.p * self.p * self.p
    }

    #[inline]
    fn cm(&self, n: u64) {
        self.mults.set(self.mults.get() + n);
    }

    pub fn add(&self, a: &El, b: &El) -> El {
        let p = self.p;
        [ad(a[0], b[0], p), ad(a[1], b[1], p), ad(a[2], b[2], p)]
    }

    pub fn sub(&self, a: &El, b: &El) -> El {
        let p = self.p;
        [sb(a[0], b[0], p), sb(a[1], b[1], p), sb(a[2], b[2], p)]
    }

    pub fn neg(&self, a: &El) -> El {
        let p = self.p;
        [ng(a[0], p), ng(a[1], p), ng(a[2], p)]
    }

    pub fn int(&self, v: i64) -> El {
        let p = self.p as i64;
        [(((v % p) + p) % p) as u64, 0, 0]
    }

    /// Multiplication by an element of `F_p` (3 multiplications).
    pub fn mulf(&self, a: &El, s: u64) -> El {
        self.cm(3);
        let p = self.p;
        [md(a[0], s, p), md(a[1], s, p), md(a[2], s, p)]
    }

    /// Schoolbook product and reduction by `z^3 = m0 + m1 z`: 13 multiplications.
    pub fn mul(&self, a: &El, b: &El) -> El {
        self.cm(13);
        let p = self.p;
        let c0 = md(a[0], b[0], p);
        let c1 = (a[0] * b[1] + a[1] * b[0]) % p;
        let c2 = (a[0] * b[2] + a[1] * b[1] + a[2] * b[0]) % p;
        let c3 = (a[1] * b[2] + a[2] * b[1]) % p;
        let c4 = md(a[2], b[2], p);
        [
            (c0 + md(self.m0, c3, p)) % p,
            (c1 + md(self.m1, c3, p) + md(self.m0, c4, p)) % p,
            (c2 + md(self.m1, c4, p)) % p,
        ]
    }

    pub fn pow(&self, a: &El, mut e: u128) -> El {
        let mut r = ONE;
        let mut b = *a;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(&r, &b);
            }
            e >>= 1;
            if e > 0 {
                b = self.mul(&b, &b);
            }
        }
        r
    }

    /// `x -> x^p`, `F_p`-linear (6 multiplications).
    pub fn frob(&self, a: &El) -> El {
        self.cm(6);
        let p = self.p;
        [
            (a[0] + a[1] * self.zp[0] + a[2] * self.zp2[0]) % p,
            (a[1] * self.zp[1] + a[2] * self.zp2[1]) % p,
            (a[1] * self.zp[2] + a[2] * self.zp2[2]) % p,
        ]
    }

    /// `Tr_{K/F_p}`.
    pub fn trace(&self, a: &El) -> u64 {
        (3 * a[0] + 2 * md(self.m1, a[2], self.p)) % self.p
    }

    /// `N_{K/F_p}`.
    pub fn norm(&self, a: &El) -> u64 {
        let s1 = self.frob(a);
        let s2 = self.frob(&s1);
        let t = self.mul(a, &s1);
        let t = self.mul(&t, &s2);
        debug_assert!(t[1] == 0 && t[2] == 0);
        t[0]
    }

    fn fp_pow(&self, mut b: u64, mut e: u64) -> u64 {
        let p = self.p;
        let mut r = 1 % p;
        b %= p;
        while e > 0 {
            if e & 1 == 1 {
                r = md(r, b, p);
                self.cm(1);
            }
            e >>= 1;
            if e > 0 {
                b = md(b, b, p);
                self.cm(1);
            }
        }
        r
    }

    /// Quadratic character of `F_p` by Euler's criterion (counted).
    pub fn chi_fp(&self, a: u64) -> i8 {
        let a = a % self.p;
        if a == 0 {
            0
        } else if self.fp_pow(a, (self.p - 1) / 2) == 1 {
            1
        } else {
            -1
        }
    }

    /// Quadratic character of `K`: `chi_p(N(a))`.
    pub fn chi(&self, a: &El) -> i8 {
        self.chi_fp(self.norm(a))
    }

    pub fn inv_fp(&self, a: u64) -> u64 {
        self.invs.set(self.invs.get() + 1);
        sp::inv_mod(a, self.p)
    }

    /// `a^{-1} = sigma(a) sigma^2(a) / N(a)`.
    pub fn inv(&self, a: &El) -> El {
        assert!(*a != ZERO, "inverse of zero in K");
        let s1 = self.frob(a);
        let s2 = self.frob(&s1);
        let w = self.mul(&s1, &s2);
        let n = self.mul(a, &w)[0];
        let ni = self.inv_fp(n);
        self.mulf(&w, ni)
    }

    /// Tonelli-Shanks square root in `K`.
    pub fn sqrt(&self, a: &El) -> Option<El> {
        if *a == ZERO {
            return Some(ZERO);
        }
        if self.chi(a) != 1 {
            return None;
        }
        let qm1 = self.size() as u128 - 1;
        let s = qm1.trailing_zeros();
        let t = qm1 >> s;
        let z = [self.d, 0, 0];
        let mut c = self.pow(&z, t);
        let mut r = self.pow(a, t.div_ceil(2));
        let mut tt = self.pow(a, t);
        let mut m = s;
        while tt != ONE {
            let mut i = 0;
            let mut x = tt;
            while x != ONE {
                x = self.mul(&x, &x);
                i += 1;
            }
            let b = self.pow(&c, 1u128 << (m - i - 1));
            m = i;
            c = self.mul(&b, &b);
            tt = self.mul(&tt, &c);
            r = self.mul(&r, &b);
        }
        Some(r)
    }

    pub fn idx(&self, a: &El) -> usize {
        (a[0] + self.p * (a[1] + self.p * a[2])) as usize
    }

    pub fn el(&self, i: usize) -> El {
        let i = i as u64;
        [i % self.p, (i / self.p) % self.p, i / (self.p * self.p)]
    }

    pub fn in_fp(&self, a: &El) -> bool {
        a[1] == 0 && a[2] == 0
    }
}

// Polynomials over K, little-endian, untrimmed helpers for the descent.

fn ptrim(a: &mut Vec<El>) {
    while a.last() == Some(&ZERO) {
        a.pop();
    }
}

fn pmul(k: &Fp3, a: &[El], b: &[El]) -> Vec<El> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![ZERO; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            let t = k.mul(x, y);
            r[i + j] = k.add(&r[i + j], &t);
        }
    }
    ptrim(&mut r);
    r
}

fn padd(k: &Fp3, a: &[El], b: &[El]) -> Vec<El> {
    let n = a.len().max(b.len());
    let mut r: Vec<El> = (0..n)
        .map(|i| k.add(a.get(i).unwrap_or(&ZERO), b.get(i).unwrap_or(&ZERO)))
        .collect();
    ptrim(&mut r);
    r
}

fn pscale(k: &Fp3, c: &El, a: &[El]) -> Vec<El> {
    let mut r: Vec<El> = a.iter().map(|x| k.mul(c, x)).collect();
    ptrim(&mut r);
    r
}

// Polynomials over F_p (counted through `k`).

fn fp_pmul(k: &Fp3, a: &[u64], b: &[u64]) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let p = k.p;
    k.cm((a.len() * b.len()) as u64);
    let mut r = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] = ad(r[i + j], md(x, y, p), p);
        }
    }
    r
}

fn fp_padd(p: u64, a: &[u64], b: &[u64]) -> Vec<u64> {
    let n = a.len().max(b.len());
    (0..n)
        .map(|i| ad(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0), p))
        .collect()
}

fn fp_pscale(k: &Fp3, c: u64, a: &[u64]) -> Vec<u64> {
    k.cm(a.len() as u64);
    a.iter().map(|&x| md(c, x, k.p)).collect()
}

fn fp_trim(a: &mut Vec<u64>) {
    while a.last() == Some(&0) {
        a.pop();
    }
}

// ---------------------------------------------------------------------------
// The descent H(h, alpha) and its Smith outcome.

/// `4 N^2 h(F/(4N))` for `h = h0 + h1 x + h2 x^2` over `F_p` and `alpha in
/// K \ F_p`; `None` if `alpha in F_p`. The result has degree 7 or 8 (else the
/// caller must not call Smith).
pub fn descend(k: &Fp3, h: &[u64; 3], alpha: &El) -> Option<Vec<u64>> {
    let a0 = *alpha;
    let a1 = k.frob(&a0);
    if a0 == a1 {
        return None;
    }
    let a2 = k.frob(&a1);
    let lin = |c: &El| -> Vec<El> { vec![k.neg(c), ONE] };
    let x: Vec<El> = vec![ZERO, ONE];
    let n = pmul(k, &pmul(k, &lin(&a0), &lin(&a1)), &lin(&a2));
    let d0 = k.mul(&k.sub(&a0, &a2), &k.sub(&a1, &a2));
    let d1 = k.frob(&d0);
    let d2 = k.frob(&d1);
    let tr = k.add(&k.add(&a0, &a1), &a2);
    let mut f = padd(k, &pmul(k, &x, &n), &pscale(k, &tr, &n));
    f = padd(k, &f, &pscale(k, &d0, &pmul(k, &lin(&a0), &lin(&a1))));
    f = padd(k, &f, &pscale(k, &d1, &pmul(k, &lin(&a1), &lin(&a2))));
    f = padd(k, &f, &pscale(k, &d2, &pmul(k, &lin(&a0), &lin(&a2))));
    let mut nf = Vec::new();
    let mut ff = Vec::new();
    for c in &n {
        if !k.in_fp(c) {
            return None;
        }
        nf.push(c[0]);
    }
    for c in &f {
        if !k.in_fp(c) {
            return None;
        }
        ff.push(c[0]);
    }
    let p = k.p;
    let n2 = fp_pmul(k, &nf, &nf);
    let fnn = fp_pmul(k, &ff, &nf);
    let ff2 = fp_pmul(k, &ff, &ff);
    let inv4 = k.inv_fp(4);
    let t0 = fp_pscale(k, md(4, h[0], p), &n2);
    let t1 = fp_pscale(k, h[1], &fnn);
    let t2 = fp_pscale(k, md(inv4, h[2], p), &ff2);
    let mut out = fp_padd(p, &fp_padd(p, &t0, &t1), &t2);
    fp_trim(&mut out);
    Some(out)
}

/// Smith's outcome on one `H`.
#[derive(Clone, Debug, Default, Serialize, PartialEq, Eq)]
pub struct HOut {
    /// `F` is squarefree of degree 7 or 8 and Smith's machinery ran.
    pub ok: bool,
    pub degree: usize,
    pub cycle_type: Vec<u32>,
    /// `#S(H)`.
    pub n_tractable: usize,
    /// Subgroups passing Proposition 2.
    pub n_trig: usize,
    /// Subgroups passing Propositions 2 and 4.
    pub n_iso: usize,
    /// ... on the quadratic twist of `H` (not a pass).
    pub n_iso_twist: usize,
    pub full_success: bool,
}

impl HOut {
    /// The signature used to compare outcome sets between enumerations.
    pub fn signature(&self) -> String {
        format!(
            "{}|{:?}|{}|{}|{}|{}",
            self.ok, self.cycle_type, self.n_tractable, self.n_trig, self.n_iso, self.n_iso_twist
        )
    }
}

pub fn smith_outcome(f: &[u64], p: u64) -> HOut {
    let d = f.len().wrapping_sub(1);
    if !(d == 7 || d == 8) {
        return HOut::default();
    }
    let mut rng = sp::SplitMix64(fnv(f) ^ 0x5eed_5a17);
    match sp::analyze_curve(f, p, &mut rng) {
        None => HOut {
            degree: d,
            ..Default::default()
        },
        Some(o) => HOut {
            ok: true,
            degree: d,
            cycle_type: o.cycle_type.clone(),
            n_tractable: o.n_tractable(),
            n_trig: o.n_trig(),
            n_iso: o.n_iso(false),
            n_iso_twist: o.n_iso(true),
            full_success: o.full_success(false),
        },
    }
}

// ---------------------------------------------------------------------------
// Tables for exhaustive work (small p).

pub struct Tables {
    /// `chi_K` by element index.
    pub sq: Vec<i8>,
    /// Discrete logarithm by element index (`u32::MAX` for zero).
    pub dlog: Vec<u32>,
    pub gen: El,
}

impl Tables {
    pub fn new(k: &Fp3) -> Tables {
        let q = k.size() as usize;
        let mut sq = vec![-1i8; q];
        sq[0] = 0;
        for i in 1..q {
            let e = k.el(i);
            sq[k.idx(&k.mul(&e, &e))] = 1;
        }
        // primitive root
        let qm1 = (q - 1) as u64;
        let mut fs = vec![];
        let mut m = qm1;
        let mut f = 2;
        while f * f <= m {
            if m % f == 0 {
                fs.push(f);
                while m % f == 0 {
                    m /= f;
                }
            }
            f += 1;
        }
        if m > 1 {
            fs.push(m);
        }
        let mut gen = ZERO;
        for i in 2..q {
            let e = k.el(i);
            if fs.iter().all(|&f| k.pow(&e, (qm1 / f) as u128) != ONE) {
                gen = e;
                break;
            }
        }
        let mut dlog = vec![u32::MAX; q];
        let mut cur = ONE;
        for e in 0..qm1 {
            dlog[k.idx(&cur)] = e as u32;
            cur = k.mul(&cur, &gen);
        }
        Tables { sq, dlog, gen }
    }
}

// ---------------------------------------------------------------------------
// Elliptic curves y^2 = x^3 + A x + B over K.

/// `y^2 = a4 x^4 + ... + a0` (coefficients low to high) to its Jacobian
/// `Y^2 = X^3 - 27 I X - 27 J`.
pub fn quartic_to_weierstrass(k: &Fp3, c: &[El; 5]) -> (El, El) {
    let (e, d, cc, b, a) = (c[0], c[1], c[2], c[3], c[4]);
    let m = |x: &El, y: &El| k.mul(x, y);
    // I = 12 a e - 3 b d + c^2
    let ae = m(&a, &e);
    let bd = m(&b, &d);
    let c2 = m(&cc, &cc);
    let i_ = k.add(&k.sub(&k.mulf(&ae, 12), &k.mulf(&bd, 3)), &c2);
    // J = 72 a c e + 9 b c d - 27 a d^2 - 27 e b^2 - 2 c^3
    let ace = m(&ae, &cc);
    let bcd = m(&bd, &cc);
    let ad2 = m(&a, &m(&d, &d));
    let eb2 = m(&e, &m(&b, &b));
    let c3 = m(&c2, &cc);
    let mut j = k.mulf(&ace, 72);
    j = k.add(&j, &k.mulf(&bcd, 9));
    j = k.sub(&j, &k.mulf(&ad2, 27));
    j = k.sub(&j, &k.mulf(&eb2, 27));
    j = k.sub(&j, &k.mulf(&c3, 2));
    let p = k.p;
    (k.mulf(&i_, ng(27, p)), k.mulf(&j, ng(27, p)))
}

/// The quartic `h(x) (x - alpha)(x - sigma alpha)`, coefficients low to high.
pub fn model_quartic(k: &Fp3, h: &[u64; 3], alpha: &El) -> [El; 5] {
    let a1 = k.frob(alpha);
    let s = k.add(alpha, &a1);
    let t = k.mul(alpha, &a1);
    let hh = |i: usize| -> El { [h[i], 0, 0] };
    let (h0, h1, h2) = (hh(0), hh(1), hh(2));
    let m = |x: &El, y: &El| k.mul(x, y);
    let a0 = m(&t, &h0);
    let a1c = k.sub(&m(&t, &h1), &m(&s, &h0));
    let a2 = k.add(&k.sub(&h0, &m(&s, &h1)), &m(&t, &h2));
    let a3 = k.sub(&h1, &m(&s, &h2));
    [a0, a1c, a2, a3, h2]
}

/// Number of `K`-points of `y^2 = f(x)` for the quartic `f` (infinity included).
pub fn count_quartic(k: &Fp3, tab: &Tables, c: &[El; 5]) -> u64 {
    let mut n: i64 = 0;
    for i in 0..k.size() as usize {
        let x = k.el(i);
        let mut v = c[4];
        for j in (0..4).rev() {
            v = k.add(&k.mul(&v, &x), &c[j]);
        }
        n += 1 + tab.sq[k.idx(&v)] as i64;
    }
    // points at infinity: 1 + chi(a4) if a4 != 0, else 1 (a4 = 0: a single point)
    n += if c[4] == ZERO {
        1
    } else {
        1 + tab.sq[k.idx(&c[4])] as i64
    };
    n as u64
}

pub fn count_ec(k: &Fp3, tab: &Tables, a: &El, b: &El) -> u64 {
    let mut n: i64 = 1;
    for i in 0..k.size() as usize {
        let x = k.el(i);
        let v = k.add(&k.add(&k.mul(&k.mul(&x, &x), &x), &k.mul(a, &x)), b);
        n += 1 + tab.sq[k.idx(&v)] as i64;
    }
    n as u64
}

/// `(A, B)` with `A^3 B0^2 = A0^3 B^2` (same `j`): are the curves
/// `K`-isomorphic? `None` if the `j`-invariants differ.
pub fn iso_test(k: &Fp3, a: &El, b: &El, a0: &El, b0: &El) -> Option<bool> {
    let a3 = k.mul(&k.mul(a, a), a);
    let a03 = k.mul(&k.mul(a0, a0), a0);
    if k.mul(&a3, &k.mul(b0, b0)) != k.mul(&a03, &k.mul(b, b)) {
        return None;
    }
    if *a == ZERO || *b == ZERO {
        return None; // special j: not handled here
    }
    let pr = k.mul(&k.mul(a, b), &k.mul(a0, b0));
    Some(k.chi(&pr) == 1)
}

/// Canonical key of the `K`-isomorphism class of `y^2 = x^3 + a x + b`.
pub type Key = (u8, u64, i8);

pub fn class_key(k: &Fp3, tab: &Tables, a: &El, b: &El) -> Option<Key> {
    let a3 = k.mul(&k.mul(a, a), a);
    let b2 = k.mul(b, b);
    let disc = k.add(&k.mulf(&a3, 4), &k.mulf(&b2, 27));
    if disc == ZERO {
        return None;
    }
    let q1 = k.size() - 1;
    if *a == ZERO {
        let g = gcd(6, q1);
        return Some((1, tab.dlog[k.idx(b)] as u64 % g, 0));
    }
    if *b == ZERO {
        let g = gcd(4, q1);
        return Some((2, tab.dlog[k.idx(a)] as u64 % g, 0));
    }
    let j = k.mul(&k.mulf(&a3, 1728 * 4 % k.p), &k.inv(&disc));
    let e = k.sub(&k.int(1728), &j);
    let a0 = k.mulf(&k.mul(&j, &e), 3);
    let b0 = k.mulf(&k.mul(&j, &k.mul(&e, &e)), 2);
    let pr = k.mul(&k.mul(a, b), &k.mul(&a0, &b0));
    Some((0, k.idx(&j) as u64, k.chi(&pr)))
}

/// Velu 2-isogeny with kernel `(r, 0)`.
pub fn velu2(k: &Fp3, a: &El, b: &El, r: &El) -> (El, El) {
    let r2 = k.mul(r, r);
    let t = k.add(&k.mulf(&r2, 3), a);
    let w = k.mul(r, &t);
    (k.sub(a, &k.mulf(&t, 5)), k.sub(b, &k.mulf(&w, 7)))
}

/// Velu 3-isogeny with kernel `{O, (x0, +-y0)}`.
pub fn velu3(k: &Fp3, a: &El, b: &El, x0: &El) -> (El, El) {
    let x2 = k.mul(x0, x0);
    let t = k.add(&k.mulf(&x2, 6), &k.mulf(a, 2));
    let y2 = k.add(&k.add(&k.mul(&x2, x0), &k.mul(a, x0)), b);
    let u = k.mulf(&y2, 4);
    let w = k.add(&u, &k.mul(x0, &t));
    (k.sub(a, &k.mulf(&t, 5)), k.sub(b, &k.mulf(&w, 7)))
}

/// Roots of `x^3 + a x + b` in `K` by scanning `K` (exhaustive work only).
pub fn roots_cubic_scan(k: &Fp3, a: &El, b: &El) -> Vec<El> {
    (0..k.size() as usize)
        .map(|i| k.el(i))
        .filter(|x| {
            k.add(&k.add(&k.mul(&k.mul(x, x), x), &k.mul(a, x)), b) == ZERO
        })
        .collect()
}

/// Roots of the 3-division polynomial `3x^4 + 6 a x^2 + 12 b x - a^2` in `K`.
pub fn roots_div3_scan(k: &Fp3, a: &El, b: &El) -> Vec<El> {
    let a2 = k.mul(a, a);
    (0..k.size() as usize)
        .map(|i| k.el(i))
        .filter(|x| {
            let x2 = k.mul(x, x);
            let v = k.add(
                &k.sub(&k.mulf(&k.mul(&x2, &x2), 3), &a2),
                &k.add(&k.mulf(&k.mul(a, &x2), 6), &k.mulf(&k.mul(b, x), 12)),
            );
            v == ZERO
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Weak models: enumeration.

#[derive(Clone, Debug, Serialize)]
pub struct ModelSpec {
    /// `h0, h1, h2` (the scalar `c` is included).
    pub h: [u64; 3],
    /// 0: `deg h = 1`, 1: split quadratic, 2: conjugate quadratic.
    pub kind: u8,
    pub alpha: El,
}

fn is_square_fp(p: u64, a: u64) -> bool {
    sp::chi(a, p) >= 0
}

/// Reduced weak models with `alpha = z`: `2 p^2` of them.
pub fn reduced_models(k: &Fp3) -> Vec<ModelSpec> {
    let p = k.p;
    let alpha = [0, 1, 0];
    let mut hs: Vec<([u64; 3], u8)> = vec![];
    for b in 0..p {
        hs.push(([ng(b, p), 1, 0], 0)); // x - b
    }
    for b1 in 0..p {
        for b2 in (b1 + 1)..p {
            hs.push(([md(b1, b2, p), ng(ad(b1, b2, p), p), 1], 1));
        }
    }
    for u in 0..p {
        for v in 0..p {
            // x^2 + u x + v, irreducible: disc u^2 - 4v non-square
            let disc = sb(md(u, u, p), md(4, v, p), p);
            if sp::chi(disc, p) == -1 {
                hs.push(([v, u, 1], 2));
            }
        }
    }
    let mut out = vec![];
    for c in [1, k.d] {
        for (h, kind) in &hs {
            out.push(ModelSpec {
                h: [md(c, h[0], p), md(c, h[1], p), md(c, h[2], p)],
                kind: *kind,
                alpha,
            });
        }
    }
    out
}

/// Unreduced control set: every `c in F_p^*`, every monic squarefree `h` of
/// degree 1 or 2, every `alpha in K \ F_p`.
pub fn unreduced_models(k: &Fp3) -> Vec<ModelSpec> {
    let p = k.p;
    let base = reduced_models(k);
    // monic h list = the first half of `base` (c = 1)
    let hs: Vec<&ModelSpec> = base.iter().take(base.len() / 2).collect();
    let mut out = vec![];
    for i in 0..k.size() as usize {
        let alpha = k.el(i);
        if k.in_fp(&alpha) {
            continue;
        }
        for c in 1..p {
            for m in &hs {
                out.push(ModelSpec {
                    h: [md(c, m.h[0], p), md(c, m.h[1], p), md(c, m.h[2], p)],
                    kind: m.kind,
                    alpha,
                });
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------
// The JV-weak test and its explicit models.

/// The three norms `u = N(e1-e2)`, `v = N(e2-e3)`, `w = N(e3-e1)`; the curve is
/// JV-weak iff two of them sum to zero.
pub fn jv_norms(k: &Fp3, e: &[El; 3]) -> [u64; 3] {
    [
        k.norm(&k.sub(&e[0], &e[1])),
        k.norm(&k.sub(&e[1], &e[2])),
        k.norm(&k.sub(&e[2], &e[0])),
    ]
}

pub fn jv_weak_from_norms(p: u64, n: &[u64; 3]) -> bool {
    (ad(n[0], n[1], p) == 0) || (ad(n[1], n[2], p) == 0) || (ad(n[0], n[2], p) == 0)
}

/// One explicit weak model of a JV-weak curve: `y^2 = c x (x - alpha)(x - sigma alpha)`.
#[derive(Clone, Debug)]
pub struct JvModel {
    pub c: u64,
    pub alpha: El,
    pub partition: usize,
    pub pair_is_first: bool,
    pub order: usize,
}

/// All explicit weak models of `y^2 = x^3 + a x + b` with roots `e` (the
/// partitions with `N(r) = 1`, both choices of the `F_p`-rational pair, both
/// orders). Returns the models in a fixed order.
pub fn jv_models(
    k: &Fp3,
    a: &El,
    b: &El,
    e: &[El; 3],
    rng: &mut sp::SplitMix64,
    tab: Option<&Tables>,
) -> Vec<JvModel> {
    let p = k.p;
    let pts: [Option<El>; 4] = [Some(e[0]), Some(e[1]), Some(e[2]), None];
    let mut out = vec![];
    for part in 0..3 {
        // partition {inf, e_part} | {others}
        let others: Vec<usize> = (0..3).filter(|&i| i != part).collect();
        let pair_a = [3usize, part];
        let pair_b = [others[0], others[1]];
        for (first, (bp, cp)) in [(true, (pair_a, pair_b)), (false, (pair_b, pair_a))] {
            // bp -> {0, inf}; cp -> {P, sigma P}
            let (bi, bj) = (pts[bp[0]], pts[bp[1]]);
            let w = |z: Option<El>| -> El {
                match (bi, bj, z) {
                    (Some(i), Some(j), Some(z)) => k.mul(&k.sub(&z, &i), &k.inv(&k.sub(&z, &j))),
                    (Some(_), Some(_), None) => ONE,
                    (Some(i), None, Some(z)) => k.sub(&z, &i),
                    (None, Some(j), Some(z)) => k.inv(&k.sub(&z, &j)),
                    _ => unreachable!(),
                }
            };
            for order in 0..2 {
                let (kk, ll) = if order == 0 {
                    (cp[0], cp[1])
                } else {
                    (cp[1], cp[0])
                };
                let (wk, wl) = (w(pts[kk]), w(pts[ll]));
                if wk == ZERO || wl == ZERO {
                    continue;
                }
                if k.norm(&wk) != k.norm(&wl) {
                    continue;
                }
                let r = k.mul(&wl, &k.inv(&wk));
                let rinv = k.inv(&r);
                let s1 = k.frob(&r);
                // u0 = beta + sigma(beta)/r + sigma^2(beta)/(r sigma(r))
                let g1 = rinv;
                let g2 = k.inv(&k.mul(&r, &s1));
                let mut u0 = ZERO;
                for _ in 0..8 {
                    let beta = [rng.below(p), rng.below(p), rng.below(p)];
                    let b1 = k.frob(&beta);
                    let b2 = k.frob(&b1);
                    u0 = k.add(&k.add(&beta, &k.mul(&b1, &g1)), &k.mul(&b2, &g2));
                    if u0 != ZERO && !k.in_fp(&u0) {
                        break;
                    }
                    u0 = ZERO;
                }
                if u0 == ZERO {
                    continue;
                }
                // c: which twist of y^2 = c x (x-P)(x-sigma P) is E?
                let mut chosen = None;
                for c in [1u64, k.d] {
                    let q = model_quartic(k, &[0, c, 0], &u0);
                    let (aa, bb) = quartic_to_weierstrass(k, &q);
                    let same = match tab {
                        Some(t) => {
                            let ka = class_key(k, t, &aa, &bb);
                            ka.is_some() && ka == class_key(k, t, a, b)
                        }
                        None => iso_test(k, &aa, &bb, a, b) == Some(true),
                    };
                    if same {
                        chosen = Some(c);
                        break;
                    }
                }
                if let Some(c) = chosen {
                    out.push(JvModel {
                        c,
                        alpha: u0,
                        partition: part,
                        pair_is_first: first,
                        order,
                    });
                }
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Exhaustive study.

#[derive(Clone, Debug, Serialize)]
pub struct ModelRec {
    pub spec: ModelSpec,
    pub node: usize,
    pub out: HOut,
    /// `N1 = p+1`, `N2 = p^2+1`, `N3 = p^3+1-3t` hold (`None` if not checked).
    pub l_ok: Option<bool>,
}

#[derive(Clone, Debug, Serialize)]
pub struct NodeRec {
    pub a: El,
    pub b: El,
    pub key: Key,
    pub t: i64,
    /// Some weak model has this Jacobian.
    pub weak: bool,
    pub jv_weak: bool,
    pub models: Vec<usize>,
    pub pass: bool,
    pub adj2: Vec<usize>,
    pub adj3: Vec<usize>,
    pub comp2: usize,
    pub comp23: usize,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Controls {
    pub l_checked: u64,
    pub l_fail: u64,
    pub velu_trace_checked: u64,
    pub velu_trace_fail: u64,
    pub dual_checked: u64,
    pub dual_missing: u64,
    pub jv_models_checked: u64,
    pub jv_not_weak: u64,
    pub jv_no_models: u64,
    pub jv_signature_missing: u64,
    pub smith_none: u64,
    pub roots_checked: u64,
    pub roots_mismatch: u64,
}

pub struct Exhaustive {
    pub p: u64,
    pub models: Vec<ModelRec>,
    pub nodes: Vec<NodeRec>,
    pub controls: Controls,
}

fn weak_key(k: &Fp3, tab: &Tables, spec: &ModelSpec) -> Option<(El, El, Key)> {
    let q = model_quartic(k, &spec.h, &spec.alpha);
    let (a, b) = quartic_to_weierstrass(k, &q);
    let key = class_key(k, tab, &a, &b)?;
    Some((a, b, key))
}

/// L-polynomial check by point counts on `F_p`, `F_{p^2}`, `F_{p^3}`.
pub fn l_check(k: &Fp3, tab: &Tables, f: &[u64], t: i64) -> bool {
    let p = k.p;
    let d = f.len() - 1;
    let pi = p as i64;
    let n1 = sp::h_point_count(f, p) as i64;
    // F_{p^2} = F_p[s]/(s^2 - dd), dd the least non-residue.
    let dd = k.d;
    let mut n2: i64 = 0;
    for a in 0..p {
        for b in 0..p {
            // Horner in F_{p^2}
            let (mut va, mut vb) = (0u64, 0u64);
            for &c in f.iter().rev() {
                let na = (va * a + vb * b % p * dd + c) % p;
                let nb = (va * b + vb * a) % p;
                va = na;
                vb = nb;
            }
            let nrm = sb(md(va, va, p), md(dd, md(vb, vb, p), p), p);
            n2 += 1 + sp::chi(nrm, p) as i64;
        }
    }
    n2 += if d == 8 {
        1 + sp::chi(f[8], p) as i64
    } else {
        1
    };
    let mut n3: i64 = 0;
    for i in 0..k.size() as usize {
        let x = k.el(i);
        let mut v = ZERO;
        for &c in f.iter().rev() {
            v = k.add(&k.mul(&v, &x), &[c, 0, 0]);
        }
        n3 += 1 + tab.sq[k.idx(&v)] as i64;
    }
    n3 += if d == 8 {
        1 + sp::chi(f[8], p) as i64
    } else {
        1
    };
    let q3 = pi * pi * pi;
    n1 == pi + 1 && n2 == pi * pi + 1 && n3 == q3 + 1 - 3 * t
}

struct Dsu(Vec<usize>);
impl Dsu {
    fn find(&mut self, x: usize) -> usize {
        if self.0[x] != x {
            let r = self.find(self.0[x]);
            self.0[x] = r;
        }
        self.0[x]
    }
    fn union(&mut self, a: usize, b: usize) {
        let (a, b) = (self.find(a), self.find(b));
        self.0[a] = b;
    }
}

/// Run the exhaustive study at `p`. `with_l`: run the `L`-polynomial check on
/// every model. `graph3`: also close the explored set under rational
/// 3-isogenies. `unreduced`: use the unreduced model set (control).
pub fn exhaustive(p: u64, with_l: bool, graph3: bool, unreduced: bool) -> Exhaustive {
    let k = Fp3::new(p);
    let tab = Tables::new(&k);
    let specs = if unreduced {
        unreduced_models(&k)
    } else {
        reduced_models(&k)
    };
    let mut ctl = Controls::default();
    let mut nodes: Vec<NodeRec> = vec![];
    let mut keys: HashMap<Key, usize> = HashMap::new();
    let mut models: Vec<ModelRec> = vec![];
    let mut memo: HashMap<Vec<u64>, HOut> = HashMap::new();
    let mut tcache: HashMap<Key, i64> = HashMap::new();

    let mut node_of = |a: &El, b: &El, key: Key, nodes: &mut Vec<NodeRec>, tcache: &mut HashMap<Key, i64>| -> usize {
        if let Some(&i) = keys.get(&key) {
            return i;
        }
        let t = *tcache.entry(key).or_insert_with(|| {
            (k.size() as i64) + 1 - count_ec(&k, &tab, a, b) as i64
        });
        nodes.push(NodeRec {
            a: *a,
            b: *b,
            key,
            t,
            weak: false,
            jv_weak: false,
            models: vec![],
            pass: false,
            adj2: vec![],
            adj3: vec![],
            comp2: 0,
            comp23: 0,
        });
        keys.insert(key, nodes.len() - 1);
        nodes.len() - 1
    };

    for spec in &specs {
        let Some((a, b, key)) = weak_key(&k, &tab, spec) else {
            continue;
        };
        let id = node_of(&a, &b, key, &mut nodes, &mut tcache);
        let Some(f) = descend(&k, &spec.h, &spec.alpha) else {
            continue;
        };
        let out = memo
            .entry(f.clone())
            .or_insert_with(|| smith_outcome(&f, p))
            .clone();
        if !out.ok {
            ctl.smith_none += 1;
        }
        let l_ok = if with_l && (f.len() == 8 || f.len() == 9) {
            ctl.l_checked += 1;
            let ok = l_check(&k, &tab, &f, nodes[id].t);
            if !ok {
                ctl.l_fail += 1;
            }
            Some(ok)
        } else {
            None
        };
        nodes[id].weak = true;
        nodes[id].models.push(models.len());
        models.push(ModelRec {
            spec: spec.clone(),
            node: id,
            out,
            l_ok,
        });
    }
    for n in nodes.iter_mut() {
        n.pass = n.models.iter().any(|&m| models[m].out.full_success);
    }

    // JV-weak flag and the constructive-model control.
    let weak_ids: Vec<usize> = (0..nodes.len()).collect();
    for &id in &weak_ids {
        let (a, b) = (nodes[id].a, nodes[id].b);
        let roots = roots_cubic_scan(&k, &a, &b);
        if roots.len() == 3 {
            let e = [roots[0], roots[1], roots[2]];
            let nrm = jv_norms(&k, &e);
            let jv = jv_weak_from_norms(p, &nrm);
            nodes[id].jv_weak = jv;
            if jv {
                let mut rng = sp::SplitMix64(0x1234 + id as u64);
                let ms = jv_models(&k, &a, &b, &e, &mut rng, Some(&tab));
                if ms.is_empty() {
                    ctl.jv_no_models += 1;
                }
                if !ms.is_empty() && !nodes[id].weak {
                    ctl.jv_not_weak += 1;
                }
                let have: HashSet<String> = nodes[id]
                    .models
                    .iter()
                    .map(|&m| models[m].out.signature())
                    .collect();
                for m in &ms {
                    ctl.jv_models_checked += 1;
                    let Some(f) = descend(&k, &[0, m.c, 0], &m.alpha) else {
                        continue;
                    };
                    let out = memo
                        .entry(f.clone())
                        .or_insert_with(|| smith_outcome(&f, p))
                        .clone();
                    if nodes[id].weak && !have.contains(&out.signature()) {
                        ctl.jv_signature_missing += 1;
                    }
                }
            }
        }
    }

    // Close under isogenies.
    let mut queue: VecDeque<usize> = (0..nodes.len()).collect();
    let mut expanded = vec![false; nodes.len()];
    while let Some(id) = queue.pop_front() {
        while expanded.len() < nodes.len() {
            expanded.push(false);
        }
        if expanded[id] {
            continue;
        }
        expanded[id] = true;
        let (a, b, t) = (nodes[id].a, nodes[id].b, nodes[id].t);
        let mut adj2 = vec![];
        let mut adj3 = vec![];
        let roots = roots_cubic_scan(&k, &a, &b);
        for r in &roots {
            let (a2, b2) = velu2(&k, &a, &b, r);
            let Some(key) = class_key(&k, &tab, &a2, &b2) else {
                continue;
            };
            let n_before = nodes.len();
            let tgt = node_of(&a2, &b2, key, &mut nodes, &mut tcache);
            if nodes.len() > n_before {
                queue.push_back(tgt);
            }
            ctl.velu_trace_checked += 1;
            if nodes[tgt].t != t {
                ctl.velu_trace_fail += 1;
            }
            adj2.push(tgt);
        }
        if graph3 {
            for x0 in roots_div3_scan(&k, &a, &b) {
                let (a2, b2) = velu3(&k, &a, &b, &x0);
                let Some(key) = class_key(&k, &tab, &a2, &b2) else {
                    continue;
                };
                let n_before = nodes.len();
                let tgt = node_of(&a2, &b2, key, &mut nodes, &mut tcache);
                if nodes.len() > n_before {
                    queue.push_back(tgt);
                }
                ctl.velu_trace_checked += 1;
                if nodes[tgt].t != t {
                    ctl.velu_trace_fail += 1;
                }
                adj3.push(tgt);
            }
        }
        adj2.sort_unstable();
        adj2.dedup();
        adj3.sort_unstable();
        adj3.dedup();
        nodes[id].adj2 = adj2;
        nodes[id].adj3 = adj3;
        ctl.roots_checked += 1;
    }
    // Dual control: every edge has its reverse.
    for id in 0..nodes.len() {
        for &j in nodes[id].adj2.clone().iter() {
            ctl.dual_checked += 1;
            if !nodes[j].adj2.contains(&id) {
                ctl.dual_missing += 1;
            }
        }
        for &j in nodes[id].adj3.clone().iter() {
            ctl.dual_checked += 1;
            if !nodes[j].adj3.contains(&id) {
                ctl.dual_missing += 1;
            }
        }
    }
    // Components.
    let n = nodes.len();
    let mut d2 = Dsu((0..n).collect());
    let mut d23 = Dsu((0..n).collect());
    for id in 0..n {
        for &j in &nodes[id].adj2 {
            d2.union(id, j);
            d23.union(id, j);
        }
        for &j in &nodes[id].adj3 {
            d23.union(id, j);
        }
    }
    for id in 0..n {
        nodes[id].comp2 = d2.find(id);
        nodes[id].comp23 = d23.find(id);
    }
    Exhaustive {
        p,
        models,
        nodes,
        controls: ctl,
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Dist {
    /// `hist[d]` = failing weak `E` at distance `d`; the last entry is
    /// `unreachable` (no passing `E` in the component).
    pub hist: Vec<u64>,
    pub unreachable: u64,
    pub mean_reachable: f64,
    pub max: usize,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct CompStats {
    pub components_with_weak: u64,
    pub components_ge2_weak: u64,
    pub mono_ge2: u64,
    pub mixed_ge2: u64,
    pub weak_in_mixed: u64,
    pub max_size: u64,
    pub share_weak_with_pass_in_comp: f64,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Summary {
    pub p: u64,
    pub models: usize,
    pub weak_nodes: usize,
    pub explored_nodes: usize,
    pub models_per_node_mean: f64,
    pub kinds: [u64; 3],
    pub jv_weak_nodes: usize,
    pub share_jv_weak: f64,
    pub models_ok: u64,
    pub models_smith_none: u64,
    pub share_h_tractable: f64,
    pub share_h_prop2: f64,
    pub share_h_full: f64,
    pub share_prop4_given_prop2: f64,
    pub cycle_types: Vec<(String, u64)>,
    pub share_e_pass: f64,
    pub share_e_models_disagree: f64,
    pub share_e_jv_pass: f64,
    pub dist2: Dist,
    pub dist23: Dist,
    pub comp2: CompStats,
    pub comp23: CompStats,
    pub share_class_t_pass: f64,
    pub classes_t: u64,
    pub t_values_by_pass: (u64, u64),
    pub r_comp2: f64,
    pub r_comp23: f64,
    pub ceiling_ref: (f64, f64),
}

fn bfs_dist(nodes: &[NodeRec], use3: bool, sources: &[usize]) -> Vec<i32> {
    let mut dist = vec![-1i32; nodes.len()];
    let mut q = VecDeque::new();
    for &s in sources {
        dist[s] = 0;
        q.push_back(s);
    }
    while let Some(u) = q.pop_front() {
        let e3: &[usize] = if use3 { &nodes[u].adj3 } else { &[] };
        for &v in nodes[u].adj2.iter().chain(e3.iter()) {
            if dist[v] < 0 {
                dist[v] = dist[u] + 1;
                q.push_back(v);
            }
        }
    }
    dist
}

fn dist_of(nodes: &[NodeRec], use3: bool) -> Dist {
    let passing: Vec<usize> = (0..nodes.len())
        .filter(|&i| nodes[i].weak && nodes[i].pass)
        .collect();
    let dist = bfs_dist(nodes, use3, &passing);
    let mut d = Dist::default();
    let mut sum = 0f64;
    let mut cnt = 0u64;
    for i in 0..nodes.len() {
        if nodes[i].weak && !nodes[i].pass {
            if dist[i] < 0 {
                d.unreachable += 1;
            } else {
                let x = dist[i] as usize;
                if d.hist.len() <= x {
                    d.hist.resize(x + 1, 0);
                }
                d.hist[x] += 1;
                sum += x as f64;
                cnt += 1;
                d.max = d.max.max(x);
            }
        }
    }
    d.mean_reachable = if cnt > 0 { sum / cnt as f64 } else { 0.0 };
    d
}

fn comp_stats(nodes: &[NodeRec], use3: bool) -> CompStats {
    let mut by: HashMap<usize, (u64, u64, u64)> = HashMap::new(); // weak, pass, size
    for n in nodes {
        let c = if use3 { n.comp23 } else { n.comp2 };
        let e = by.entry(c).or_insert((0, 0, 0));
        e.2 += 1;
        if n.weak {
            e.0 += 1;
            if n.pass {
                e.1 += 1;
            }
        }
    }
    let mut s = CompStats::default();
    let mut weak_total = 0u64;
    let mut weak_with_pass = 0u64;
    for (_, &(w, pa, sz)) in by.iter() {
        if w == 0 {
            continue;
        }
        s.components_with_weak += 1;
        s.max_size = s.max_size.max(sz);
        weak_total += w;
        if pa > 0 {
            weak_with_pass += w;
        }
        if w >= 2 {
            s.components_ge2_weak += 1;
            if pa == 0 || pa == w {
                s.mono_ge2 += 1;
            } else {
                s.mixed_ge2 += 1;
                s.weak_in_mixed += w;
            }
        }
    }
    s.share_weak_with_pass_in_comp = if weak_total > 0 {
        weak_with_pass as f64 / weak_total as f64
    } else {
        0.0
    };
    s
}

pub fn summarize(x: &Exhaustive) -> Summary {
    let weak: Vec<&NodeRec> = x.nodes.iter().filter(|n| n.weak).collect();
    let nw = weak.len().max(1) as f64;
    let nm = x.models.len().max(1) as f64;
    let mut s = Summary {
        p: x.p,
        models: x.models.len(),
        weak_nodes: weak.len(),
        explored_nodes: x.nodes.len(),
        models_per_node_mean: x.models.len() as f64 / nw,
        ..Default::default()
    };
    for m in &x.models {
        s.kinds[m.spec.kind as usize] += 1;
    }
    let mut ct: HashMap<String, u64> = HashMap::new();
    let (mut tr, mut p2, mut full, mut ok, mut p2n, mut p4_given) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    for m in &x.models {
        if !m.out.ok {
            s.models_smith_none += 1;
            continue;
        }
        ok += 1;
        *ct.entry(sp::ct_string(&m.out.cycle_type)).or_default() += 1;
        if m.out.n_tractable > 0 {
            tr += 1;
        }
        if m.out.n_trig > 0 {
            p2 += 1;
            p2n += 1;
            if m.out.n_iso > 0 {
                p4_given += 1;
            }
        }
        if m.out.full_success {
            full += 1;
        }
    }
    s.models_ok = ok;
    let okf = ok.max(1) as f64;
    s.share_h_tractable = tr as f64 / okf;
    s.share_h_prop2 = p2 as f64 / okf;
    s.share_h_full = full as f64 / okf;
    s.share_prop4_given_prop2 = if p2n > 0 { p4_given as f64 / p2n as f64 } else { 0.0 };
    let mut cts: Vec<(String, u64)> = ct.into_iter().collect();
    cts.sort();
    s.cycle_types = cts;
    s.jv_weak_nodes = weak.iter().filter(|n| n.jv_weak).count();
    s.share_jv_weak = s.jv_weak_nodes as f64 / nw;
    s.share_e_pass = weak.iter().filter(|n| n.pass).count() as f64 / nw;
    s.share_e_jv_pass = {
        let jv: Vec<&&NodeRec> = weak.iter().filter(|n| n.jv_weak).collect();
        if jv.is_empty() {
            0.0
        } else {
            jv.iter().filter(|n| n.pass).count() as f64 / jv.len() as f64
        }
    };
    s.share_e_models_disagree = weak
        .iter()
        .filter(|n| {
            let v: Vec<bool> = n.models.iter().map(|&m| x.models[m].out.full_success).collect();
            v.iter().any(|&b| b) && v.iter().any(|&b| !b)
        })
        .count() as f64
        / nw;
    let _ = nm;
    s.dist2 = dist_of(&x.nodes, false);
    s.dist23 = dist_of(&x.nodes, true);
    s.comp2 = comp_stats(&x.nodes, false);
    s.comp23 = comp_stats(&x.nodes, true);
    // K-isogeny classes: same trace t among weak curves.
    let mut by_t: HashMap<i64, (u64, u64)> = HashMap::new();
    for n in &weak {
        let e = by_t.entry(n.t).or_default();
        e.0 += 1;
        if n.pass {
            e.1 += 1;
        }
    }
    s.classes_t = by_t.len() as u64;
    let wp: u64 = by_t.values().filter(|v| v.1 > 0).map(|v| v.0).sum();
    s.share_class_t_pass = wp as f64 / nw;
    s.t_values_by_pass = (
        by_t.values().filter(|v| v.1 > 0).count() as u64,
        by_t.values().filter(|v| v.1 == 0).count() as u64,
    );
    let d = s.share_e_pass;
    let u = s.share_class_t_pass;
    let r = |c: f64| if u > d { (c - d) / (u - d) } else { f64::NAN };
    s.r_comp2 = r(s.comp2.share_weak_with_pass_in_comp);
    s.r_comp23 = r(s.comp23.share_weak_with_pass_in_comp);
    s.ceiling_ref = (0.1857, 0.4943);
    s
}

// ---------------------------------------------------------------------------
// The priced random walk (sampled p).

#[derive(Clone, Debug)]
pub struct WalkNode {
    pub a: El,
    pub b: El,
    pub roots: [El; 3],
    /// Index of the root that is the kernel of the dual (the way back).
    pub back: Option<usize>,
}

/// Build the node of `y^2 = (x - r0)(x - r1)(x - r2)` (any three distinct roots)
/// shifted to trace zero.
pub fn node_from_roots(k: &Fp3, r: &[El; 3]) -> WalkNode {
    let inv3 = k.inv_fp(3);
    let m = k.mulf(&k.add(&k.add(&r[0], &r[1]), &r[2]), inv3);
    let e = [k.sub(&r[0], &m), k.sub(&r[1], &m), k.sub(&r[2], &m)];
    let a = k.add(
        &k.add(&k.mul(&e[0], &e[1]), &k.mul(&e[0], &e[2])),
        &k.mul(&e[1], &e[2]),
    );
    let b = k.neg(&k.mul(&k.mul(&e[0], &e[1]), &e[2]));
    WalkNode {
        a,
        b,
        roots: e,
        back: None,
    }
}

/// One non-backtracking step to a neighbour with full rational 2-torsion.
/// `None` if no such neighbour exists (a dead end: reflect).
pub fn step(k: &Fp3, node: &WalkNode, rng: &mut sp::SplitMix64) -> Option<(WalkNode, usize)> {
    let cands: Vec<usize> = (0..3).filter(|&i| Some(i) != node.back).collect();
    let mut adm: Vec<(usize, El)> = vec![];
    for &i in &cands {
        let (j, l) = ((i + 1) % 3, (i + 2) % 3);
        let fp = k.mul(
            &k.sub(&node.roots[i], &node.roots[j]),
            &k.sub(&node.roots[i], &node.roots[l]),
        );
        if k.chi(&fp) == 1 {
            adm.push((i, fp));
        }
    }
    if adm.is_empty() {
        return None;
    }
    let (i, fp) = adm[rng.below(adm.len() as u64) as usize];
    let r = node.roots[i];
    let s = k.sqrt(&fp).expect("square checked");
    let (a2, b2) = velu2(k, &node.a, &node.b, &r);
    let two_s = k.add(&s, &s);
    let roots = [k.neg(&k.add(&r, &r)), k.add(&r, &two_s), k.sub(&r, &two_s)];
    Some((
        WalkNode {
            a: a2,
            b: b2,
            roots,
            back: Some(0),
        },
        i,
    ))
}

/// Cost record accumulated in `F_p` multiplications, inversions, and ns.
#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct Cost {
    pub mults: u64,
    pub invs: u64,
    pub ns: u128,
}

impl Cost {
    pub fn add(&mut self, o: &Cost) {
        self.mults += o.mults;
        self.invs += o.invs;
        self.ns += o.ns;
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct WalkResult {
    pub steps: u64,
    pub reflects: u64,
    pub nodes_tested: u64,
    pub jv_weak_hits: u64,
    pub models_tried: u64,
    pub smith_calls: u64,
    pub smith_ns: u128,
    pub passed: bool,
    pub censored: bool,
    pub cost_step: Cost,
    pub cost_test: Cost,
    pub cost_descent: Cost,
    /// Distinct Smith signatures seen on the JV-weak hits (per hit, max).
    pub max_distinct_sigs: u64,
    pub hits_all_fail: u64,
}

/// Test a JV-weak node: try its explicit models until one passes Smith.
/// Returns `(passed, models_tried, smith_calls, smith_ns, distinct signatures)`.
pub fn test_models(
    k: &Fp3,
    node: &WalkNode,
    rng: &mut sp::SplitMix64,
    stop_at_pass: bool,
    cost_desc: &mut Cost,
) -> (bool, u64, u64, u128, u64) {
    let t0 = std::time::Instant::now();
    let m0 = k.mults.get();
    let i0 = k.invs.get();
    let ms = jv_models(k, &node.a, &node.b, &node.roots, rng, None);
    let mut passed = false;
    let (mut tried, mut calls, mut sm_ns) = (0u64, 0u64, 0u128);
    let mut sigs: Vec<String> = vec![];
    let mut seen: HashMap<Vec<u64>, bool> = HashMap::new();
    let mut desc_ns = t0.elapsed().as_nanos();
    for m in &ms {
        let t1 = std::time::Instant::now();
        let f = descend(k, &[0, m.c, 0], &m.alpha);
        desc_ns += t1.elapsed().as_nanos();
        let Some(f) = f else { continue };
        tried += 1;
        if let Some(&pass) = seen.get(&f) {
            if pass {
                passed = true;
            }
            continue;
        }
        let t2 = std::time::Instant::now();
        let o = smith_outcome(&f, k.p);
        sm_ns += t2.elapsed().as_nanos();
        calls += 1;
        let sig = o.signature();
        if !sigs.contains(&sig) {
            sigs.push(sig);
        }
        seen.insert(f, o.full_success);
        if o.full_success {
            passed = true;
            if stop_at_pass {
                break;
            }
        }
    }
    cost_desc.mults += k.mults.get() - m0;
    cost_desc.invs += k.invs.get() - i0;
    cost_desc.ns += desc_ns;
    (passed, tried, calls, sm_ns, sigs.len() as u64)
}

/// Walk from `start` until a node has a JV-weak model passing Smith
/// (`stop_on_pass`) or merely a JV-weak model (`!stop_on_pass`, arm B).
/// `test_start`: test the start node too (arm A passes `false`: the start is
/// already known to fail).
pub fn walk(
    k: &Fp3,
    start: WalkNode,
    rng: &mut sp::SplitMix64,
    max_nodes: u64,
    max_ns: u128,
    stop_on_pass: bool,
    test_start: bool,
) -> WalkResult {
    let wall = std::time::Instant::now();
    let p = k.p;
    let mut res = WalkResult::default();
    let mut cur = start;
    let mut skip_test = !test_start;
    loop {
        if !skip_test {
            let t0 = std::time::Instant::now();
            let m0 = k.mults.get();
            let i0 = k.invs.get();
            let nrm = jv_norms(k, &cur.roots);
            let hit = jv_weak_from_norms(p, &nrm);
            res.cost_test.mults += k.mults.get() - m0;
            res.cost_test.invs += k.invs.get() - i0;
            res.cost_test.ns += t0.elapsed().as_nanos();
            res.nodes_tested += 1;
            if hit {
                let (passed, tried, calls, sm_ns, dsig) =
                    test_models(k, &cur, rng, stop_on_pass, &mut res.cost_descent);
                res.jv_weak_hits += 1;
                res.models_tried += tried;
                res.smith_calls += calls;
                res.smith_ns += sm_ns;
                res.max_distinct_sigs = res.max_distinct_sigs.max(dsig);
                if stop_on_pass {
                    if passed {
                        res.passed = true;
                        break;
                    }
                    res.hits_all_fail += 1;
                } else {
                    res.passed = true;
                    break;
                }
            }
        }
        skip_test = false;
        if res.nodes_tested >= max_nodes || wall.elapsed().as_nanos() >= max_ns {
            res.censored = true;
            break;
        }
        let t0 = std::time::Instant::now();
        let m0 = k.mults.get();
        let i0 = k.invs.get();
        match step(k, &cur, rng) {
            Some((nx, _)) => {
                cur = nx;
                res.steps += 1;
            }
            None => {
                // Dead end: both forward neighbours lack full 2-torsion. Take
                // the dual edge (the only admissible one) to the predecessor,
                // which has been tested already.
                cur.back = None;
                let (nx, _) = step(k, &cur, rng).expect("the dual neighbour has full 2-torsion");
                cur = nx;
                res.steps += 1;
                res.reflects += 1;
                skip_test = true;
            }
        }
        res.cost_step.mults += k.mults.get() - m0;
        res.cost_step.invs += k.invs.get() - i0;
        res.cost_step.ns += t0.elapsed().as_nanos();
    }
    res
}

/// A uniformly random JV-split weak start: `y^2 = c x (x - alpha)(x - sigma
/// alpha)`, as a walk node with roots `0, c alpha, c sigma alpha` (shifted).
pub fn random_weak_start(k: &Fp3, rng: &mut sp::SplitMix64) -> WalkNode {
    let p = k.p;
    loop {
        let alpha = [rng.below(p), rng.below(p), rng.below(p)];
        if k.in_fp(&alpha) {
            continue;
        }
        let c = if rng.below(2) == 0 { 1 } else { k.d };
        let a1 = k.frob(&alpha);
        let r = [ZERO, k.mulf(&alpha, c), k.mulf(&a1, c)];
        return node_from_roots(k, &r);
    }
}

/// Three uniform distinct roots.
pub fn random_full2_start(k: &Fp3, rng: &mut sp::SplitMix64) -> WalkNode {
    let p = k.p;
    loop {
        let r: [El; 3] = [0, 1, 2].map(|_| [rng.below(p), rng.below(p), rng.below(p)]);
        if r[0] != r[1] && r[1] != r[2] && r[0] != r[2] {
            return node_from_roots(k, &r);
        }
    }
}

/// Microbenchmark: seconds per `F_p` multiplication and the inversion /
/// multiplication ratio on this host.
pub fn calibrate(p: u64) -> (f64, f64, f64) {
    use std::hint::black_box;
    let n = 20_000_000u64;
    let t0 = std::time::Instant::now();
    let mut x = 12345u64 % p;
    let y = 6789u64 % p;
    for _ in 0..n {
        x = black_box(md(x, y, p)) + 1;
        if x >= p {
            x -= p;
        }
    }
    black_box(x);
    let t_mul = t0.elapsed().as_secs_f64() / n as f64;
    let m = 200_000u64;
    let t1 = std::time::Instant::now();
    let mut acc = 0u64;
    for i in 1..=m {
        acc = acc.wrapping_add(sp::inv_mod(black_box(i % (p - 1) + 1), p));
    }
    black_box(acc);
    let t_inv = t1.elapsed().as_secs_f64() / m as f64;
    // K multiplication
    let k = Fp3::new(p);
    let a = [1, 2, 3];
    let mut z = [4, 5, 6];
    let t2 = std::time::Instant::now();
    for _ in 0..2_000_000 {
        z = black_box(k.mul(&a, &z));
    }
    black_box(z);
    let t_k = t2.elapsed().as_secs_f64() / 2_000_000.0;
    (t_mul, t_inv / t_mul, t_k / t_mul)
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;

    fn rnd_el(k: &Fp3, rng: &mut sp::SplitMix64) -> El {
        [rng.below(k.p), rng.below(k.p), rng.below(k.p)]
    }

    #[test]
    fn field_axioms_and_frobenius() {
        for &p in &[7u64, 11, 13, 101] {
            let k = Fp3::new(p);
            let mut rng = sp::SplitMix64(1);
            for _ in 0..200 {
                let a = rnd_el(&k, &mut rng);
                let b = rnd_el(&k, &mut rng);
                let c = rnd_el(&k, &mut rng);
                assert_eq!(k.mul(&a, &b), k.mul(&b, &a));
                assert_eq!(
                    k.mul(&a, &k.add(&b, &c)),
                    k.add(&k.mul(&a, &b), &k.mul(&a, &c))
                );
                // Frobenius is the p-th power and a ring map
                assert_eq!(k.frob(&a), k.pow(&a, p as u128));
                assert_eq!(k.frob(&k.mul(&a, &b)), k.mul(&k.frob(&a), &k.frob(&b)));
                assert_eq!(k.frob(&k.frob(&k.frob(&a))), a);
                if a != ZERO {
                    assert_eq!(k.mul(&a, &k.inv(&a)), ONE);
                    assert_eq!(k.norm(&a), k.pow(&a, ((p * p * p - 1) / (p - 1)) as u128)[0]);
                }
                // trace is the sum of conjugates
                let s = k.add(&k.add(&a, &k.frob(&a)), &k.frob(&k.frob(&a)));
                assert_eq!(s, [k.trace(&a), 0, 0]);
            }
        }
    }

    #[test]
    fn sqrt_roundtrip_and_chi() {
        for &p in &[7u64, 13, 31, 1009] {
            let k = Fp3::new(p);
            let mut rng = sp::SplitMix64(2);
            for _ in 0..100 {
                let a = rnd_el(&k, &mut rng);
                let sq = k.mul(&a, &a);
                let r = k.sqrt(&sq).unwrap();
                assert_eq!(k.mul(&r, &r), sq);
                if a != ZERO {
                    let want = if k.pow(&a, (k.size() as u128 - 1) / 2) == ONE {
                        1
                    } else {
                        -1
                    };
                    assert_eq!(k.chi(&a) as i32, want);
                }
            }
            // the F_p non-residue stays a non-residue
            assert_eq!(k.chi(&[k.d, 0, 0]), -1);
        }
    }

    #[test]
    fn velu_preserves_point_count() {
        for &p in &[7u64, 11] {
            let k = Fp3::new(p);
            let tab = Tables::new(&k);
            let mut rng = sp::SplitMix64(3);
            let (mut n2, mut n3) = (0, 0);
            for _ in 0..60 {
                let a = rnd_el(&k, &mut rng);
                let b = rnd_el(&k, &mut rng);
                if class_key(&k, &tab, &a, &b).is_none() {
                    continue;
                }
                let n0 = count_ec(&k, &tab, &a, &b);
                for r in roots_cubic_scan(&k, &a, &b) {
                    let (a2, b2) = velu2(&k, &a, &b, &r);
                    assert_eq!(count_ec(&k, &tab, &a2, &b2), n0);
                    n2 += 1;
                }
                for x0 in roots_div3_scan(&k, &a, &b) {
                    let (a2, b2) = velu3(&k, &a, &b, &x0);
                    assert_eq!(count_ec(&k, &tab, &a2, &b2), n0);
                    n3 += 1;
                }
            }
            assert!(n2 > 0 && n3 > 0, "test must exercise both isogeny degrees");
        }
    }

    #[test]
    fn quartic_jacobian_has_same_count() {
        let p = 7;
        let k = Fp3::new(p);
        let tab = Tables::new(&k);
        let mut rng = sp::SplitMix64(4);
        let mut checked = 0;
        for _ in 0..40 {
            let mut c: [El; 5] = [ZERO; 5];
            for x in c.iter_mut() {
                *x = rnd_el(&k, &mut rng);
            }
            if rng.below(3) == 0 {
                c[4] = ZERO; // cubic
            }
            let (a, b) = quartic_to_weierstrass(&k, &c);
            if class_key(&k, &tab, &a, &b).is_none() {
                continue;
            }
            assert_eq!(count_quartic(&k, &tab, &c), count_ec(&k, &tab, &a, &b));
            checked += 1;
        }
        assert!(checked > 10);
    }

    #[test]
    fn jv_descent_has_the_expected_l_polynomial() {
        for &p in &[7u64, 11] {
            let k = Fp3::new(p);
            let tab = Tables::new(&k);
            let models = reduced_models(&k);
            assert_eq!(models.len() as u64, 2 * p * p);
            let mut rng = sp::SplitMix64(5);
            let mut kinds = [0; 3];
            for _ in 0..24 {
                let m = &models[rng.below(models.len() as u64) as usize];
                let (a, b, _) = weak_key(&k, &tab, m).unwrap();
                let t = k.size() as i64 + 1 - count_ec(&k, &tab, &a, &b) as i64;
                let f = descend(&k, &m.h, &m.alpha).unwrap();
                assert!(f.len() == 8 || f.len() == 9, "degree 7 or 8");
                assert!(l_check(&k, &tab, &f, t), "p={p} {:?}", m);
                kinds[m.kind as usize] += 1;
            }
            assert!(kinds.iter().all(|&c| c > 0));
        }
    }

    #[test]
    fn descended_cycle_types_match_the_record() {
        // (3,3,1,1) for h with a rational root structure, (6,2) for irreducible h.
        let p = 7;
        let k = Fp3::new(p);
        let mut seen = std::collections::HashSet::new();
        for m in reduced_models(&k).iter().take(400) {
            let f = descend(&k, &m.h, &m.alpha).unwrap();
            let o = smith_outcome(&f, p);
            if o.ok {
                seen.insert((m.kind, sp::ct_string(&o.cycle_type)));
            }
        }
        assert!(seen.contains(&(0, "1+1+3+3".to_string())), "{seen:?}");
        assert!(seen.contains(&(1, "1+1+3+3".to_string())), "{seen:?}");
        assert!(seen.contains(&(2, "2+6".to_string())), "{seen:?}");
    }

    #[test]
    fn step_matches_scan_and_keeps_trace() {
        let p = 11;
        let k = Fp3::new(p);
        let tab = Tables::new(&k);
        let mut rng = sp::SplitMix64(6);
        let mut moved = 0;
        for _ in 0..30 {
            let node = random_full2_start(&k, &mut rng);
            let n0 = count_ec(&k, &tab, &node.a, &node.b);
            if let Some((nx, _)) = step(&k, &node, &mut rng) {
                // the claimed roots are roots, and distinct
                for r in &nx.roots {
                    let v = k.add(
                        &k.add(&k.mul(&k.mul(r, r), r), &k.mul(&nx.a, r)),
                        &nx.b,
                    );
                    assert_eq!(v, ZERO);
                }
                assert_ne!(nx.roots[0], nx.roots[1]);
                assert_eq!(count_ec(&k, &tab, &nx.a, &nx.b), n0);
                let mut sc = roots_cubic_scan(&k, &nx.a, &nx.b);
                let mut ro = nx.roots.to_vec();
                sc.sort();
                ro.sort();
                assert_eq!(sc, ro);
                moved += 1;
            }
        }
        assert!(moved > 5);
    }

    #[test]
    fn jv_models_match_the_node_class_and_descend() {
        let p = 7;
        let k = Fp3::new(p);
        let tab = Tables::new(&k);
        let mut rng = sp::SplitMix64(7);
        let mut tested = 0;
        for _ in 0..400 {
            let node = random_weak_start(&k, &mut rng);
            let nrm = jv_norms(&k, &node.roots);
            assert!(jv_weak_from_norms(p, &nrm), "constructed start is JV-weak");
            let ms = jv_models(&k, &node.a, &node.b, &node.roots, &mut rng, None);
            assert!(!ms.is_empty());
            let t = k.size() as i64 + 1 - count_ec(&k, &tab, &node.a, &node.b) as i64;
            for m in &ms {
                let q = model_quartic(&k, &[0, m.c, 0], &m.alpha);
                let (a, b) = quartic_to_weierstrass(&k, &q);
                assert_eq!(iso_test(&k, &a, &b, &node.a, &node.b), Some(true));
                let f = descend(&k, &[0, m.c, 0], &m.alpha).unwrap();
                assert!(l_check(&k, &tab, &f, t));
                tested += 1;
            }
            if tested > 60 {
                break;
            }
        }
        assert!(tested > 20);
    }

    #[test]
    fn exhaustive_p7_controls() {
        let x = exhaustive(7, true, true, false);
        let c = &x.controls;
        assert_eq!(c.l_fail, 0);
        assert_eq!(c.velu_trace_fail, 0);
        assert_eq!(c.dual_missing, 0);
        assert_eq!(c.jv_not_weak, 0);
        assert_eq!(c.jv_signature_missing, 0);
        assert_eq!(c.smith_none, 0);
        assert_eq!(x.models.len() as u64, 2 * 49);
        let s = summarize(&x);
        assert!(s.weak_nodes > 20 && s.weak_nodes <= 98);
    }

    #[test]
    fn unreduced_enumeration_agrees_with_reduced_at_p7() {
        let red = exhaustive(7, false, false, false);
        let full = exhaustive(7, false, false, true);
        // class key -> set of Smith signatures over all models of that curve
        let sigs = |x: &Exhaustive| -> std::collections::BTreeMap<Key, Vec<String>> {
            x.nodes
                .iter()
                .filter(|n| n.weak)
                .map(|n| {
                    let mut s: Vec<String> = n
                        .models
                        .iter()
                        .map(|&m| x.models[m].out.signature())
                        .collect();
                    s.sort();
                    s.dedup();
                    (n.key, s)
                })
                .collect()
        };
        assert_eq!(sigs(&red), sigs(&full));
        let pass = |x: &Exhaustive| -> std::collections::BTreeMap<Key, bool> {
            x.nodes.iter().filter(|n| n.weak).map(|n| (n.key, n.pass)).collect()
        };
        assert_eq!(pass(&red), pass(&full));
    }

    #[test]
    fn walk_reaches_a_weak_curve_at_small_p() {
        let p = 61;
        let k = Fp3::new(p);
        let mut rng = sp::SplitMix64(8);
        let start = random_full2_start(&k, &mut rng);
        let r = walk(&k, start, &mut rng, 200_000, u128::MAX, false, true);
        assert!(r.passed && !r.censored, "{r:?}");
        assert!(r.jv_weak_hits == 1);
    }
}
