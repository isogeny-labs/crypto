//! Weil-restriction isogeny census.
//!
//! Second round of the Teske-variant search
//! (`research/weil_restriction_isogeny_census_20261018/PROTOCOL.md`): an exact
//! census of the `F_q`-rational `(ℓ, …, ℓ)`-isogeny kernels in the Weil
//! restriction `W = Res_{F_{q^n}/F_q} E` of an elliptic curve (Tian's
//! association, J. Cryptology 2023), by Frobenius class (Part G) and for every
//! curve over `F_{q^3}` at `q ≤ 13` (Part H).
//!
//! Part G works over `F_ℓ` only: `W[ℓ] = V_0 ⊕ … ⊕ V_{n−1}`, `V_i ≅ F_ℓ^2`,
//! the `F_q`-Frobenius `π(v_0, …, v_{n−1}) = (φ v_{n−1}, v_0, …, v_{n−2})` with
//! `φ = π_{q^n}|E[ℓ]`, and the Weil pairing `ω_V = ⊕ q̄^i ω`. A kernel is an
//! `F_ℓ[π]`-submodule of dimension `n` on which `ω_V` vanishes; a Tian kernel
//! is a kernel meeting no `V_i`.
//!
//! Part H computes, for every curve over `F_{q^3}`, the class of `φ` from the
//! distinct-degree factorisation of the `ℓ`-division polynomial and joins
//! it with Part G.
//!
//! std-only; `rustc --edition 2021 -O examples/weil_restriction_isogeny_census.rs`.
//! Usage: `weil_restriction_isogeny_census <out-dir> [G|H ...]`.
#![allow(dead_code)]

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

// ---------------------------------------------------------------------------
// Helpers reused from examples/teske_variants_field_types.rs (same semantics).
// ---------------------------------------------------------------------------

fn powmod(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u64 % m;
    b %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = (r as u128 * b as u128 % m as u128) as u64;
        }
        b = (b as u128 * b as u128 % m as u128) as u64;
        e >>= 1;
    }
    r
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2u64;
    while d * d <= n {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 1;
    }
    true
}

fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut out = Vec::new();
    let mut d = 2u64;
    while d * d <= n {
        if n.is_multiple_of(d) {
            out.push(d);
            while n.is_multiple_of(d) {
                n /= d;
            }
        }
        d += 1;
    }
    if n > 1 {
        out.push(n);
    }
    out
}

fn inv_mod_p(a: u32, p: u32) -> u32 {
    powmod(a as u64, p as u64 - 2, p as u64) as u32
}

fn poly_trim(a: &mut Vec<u32>) {
    while a.last() == Some(&0) {
        a.pop();
    }
}

fn poly_mulmod(a: &[u32], b: &[u32], f: &[u32], p: u32) -> Vec<u32> {
    let l = f.len() - 1;
    let mut prod = vec![0u64; a.len() + b.len()];
    for (i, &x) in a.iter().enumerate() {
        if x == 0 {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            prod[i + j] = (prod[i + j] + x as u64 * y as u64) % p as u64;
        }
    }
    // reduce modulo monic f of degree l
    for k in (l..prod.len()).rev() {
        let c = prod[k];
        if c == 0 {
            continue;
        }
        prod[k] = 0;
        for i in 0..l {
            prod[k - l + i] = (prod[k - l + i] + (p as u64 - f[i] as u64) * c) % p as u64;
        }
    }
    let mut out: Vec<u32> = prod.iter().take(l).map(|&v| v as u32).collect();
    poly_trim(&mut out);
    out
}

fn poly_powmod(a: &[u32], mut e: u64, f: &[u32], p: u32) -> Vec<u32> {
    let mut r = vec![1u32];
    let mut b = a.to_vec();
    while e > 0 {
        if e & 1 == 1 {
            r = poly_mulmod(&r, &b, f, p);
        }
        b = poly_mulmod(&b, &b, f, p);
        e >>= 1;
    }
    r
}

fn poly_rem(a: &[u32], b: &[u32], p: u32) -> Vec<u32> {
    let mut r = a.to_vec();
    poly_trim(&mut r);
    let db = b.len() - 1;
    let inv = inv_mod_p(b[db], p);
    while r.len() > db {
        let k = r.len() - 1;
        let c = r[k] * inv % p;
        if c != 0 {
            for i in 0..=db {
                r[k - db + i] = (r[k - db + i] + p - (c * b[i]) % p) % p;
            }
        }
        r.pop();
        poly_trim(&mut r);
    }
    r
}

fn poly_gcd(a: &[u32], b: &[u32], p: u32) -> Vec<u32> {
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    poly_trim(&mut a);
    poly_trim(&mut b);
    while !b.is_empty() {
        let r = poly_rem(&a, &b, p);
        a = b;
        b = r;
    }
    if let Some(&lc) = a.last() {
        let inv = inv_mod_p(lc, p);
        for c in a.iter_mut() {
            *c = *c * inv % p;
        }
    }
    a
}

fn poly_sub(a: &[u32], b: &[u32], p: u32) -> Vec<u32> {
    let n = a.len().max(b.len());
    let mut out = vec![0u32; n];
    for (i, o) in out.iter_mut().enumerate() {
        let x = a.get(i).copied().unwrap_or(0);
        let y = b.get(i).copied().unwrap_or(0);
        *o = (x + p - y) % p;
    }
    poly_trim(&mut out);
    out
}

fn poly_irreducible(f: &[u32], p: u32) -> bool {
    let l = f.len() - 1;
    let x = vec![0u32, 1];
    let mut h = x.clone();
    for _ in 1..=l / 2 {
        h = poly_powmod(&h, p as u64, f, p);
        let d = poly_sub(&h, &x, p);
        let g = poly_gcd(f, &d, p);
        if g.len() != 1 {
            return false;
        }
    }
    true
}

fn poly_derivative(f: &[u32], p: u32) -> Vec<u32> {
    let mut d: Vec<u32> = f
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, &c)| (c * (i as u32 % p)) % p)
        .collect();
    poly_trim(&mut d);
    d
}

fn poly_div_exact(a: &[u32], b: &[u32], p: u32) -> Vec<u32> {
    let mut r = a.to_vec();
    let db = b.len() - 1;
    let mut qout = vec![0u32; r.len() - db];
    let inv = inv_mod_p(b[db], p);
    for k in (db..r.len()).rev() {
        let c = r[k] * inv % p;
        qout[k - db] = c;
        if c != 0 {
            for i in 0..=db {
                r[k - db + i] = (r[k - db + i] + p - c * b[i] % p) % p;
            }
        }
    }
    poly_trim(&mut qout);
    qout
}

const INF: u32 = u32::MAX;

struct Gf {
    p: u32,
    #[cfg_attr(not(test), allow(dead_code))]
    l: u32,
    nn: u32,
    #[cfg_attr(not(test), allow(dead_code))]
    exp: Vec<u32>,
    log: Vec<u32>,
    zech: Vec<u32>,
    half: u32,
}

impl Gf {
    fn new(p: u32, l: u32) -> Gf {
        assert!(p >= 3 && is_prime(p as u64));
        let size = (p as u64).pow(l) as u32;
        let nn = size - 1;
        let factors = prime_factors(nn as u64);
        let mut exp = vec![0u32; nn as usize];
        if l == 1 {
            let mut g = 2u32;
            loop {
                if factors
                    .iter()
                    .all(|&r| powmod(g as u64, nn as u64 / r, p as u64) != 1)
                {
                    break;
                }
                g += 1;
            }
            let mut cur = 1u64;
            for e in 0..nn {
                exp[e as usize] = cur as u32;
                cur = cur * g as u64 % p as u64;
            }
        } else {
            // irreducible monic polynomial of degree l
            let mut f = Vec::new();
            'search: for c in 0..size {
                let mut coeffs = vec![0u32; l as usize + 1];
                let mut v = c;
                for coeff in coeffs.iter_mut().take(l as usize) {
                    *coeff = v % p;
                    v /= p;
                }
                coeffs[l as usize] = 1;
                if coeffs[0] == 0 {
                    continue;
                }
                if poly_irreducible(&coeffs, p) {
                    f = coeffs;
                    break 'search;
                }
            }
            assert!(!f.is_empty());
            // primitive element: smallest non-constant digit index of full order
            let to_poly = |idx: u32| -> Vec<u32> {
                let mut v = idx;
                let mut c = Vec::new();
                for _ in 0..l {
                    c.push(v % p);
                    v /= p;
                }
                poly_trim(&mut c);
                c
            };
            let mut g = Vec::new();
            for idx in p..size {
                let cand = to_poly(idx);
                let ok = factors.iter().all(|&r| {
                    let pw = poly_powmod(&cand, nn as u64 / r, &f, p);
                    pw != vec![1u32]
                });
                if ok {
                    g = cand;
                    break;
                }
            }
            assert!(!g.is_empty());
            let mut cur = vec![1u32];
            for e in 0..nn {
                let mut idx = 0u32;
                let mut mulp = 1u32;
                for i in 0..l as usize {
                    idx += cur.get(i).copied().unwrap_or(0) * mulp;
                    mulp *= p;
                }
                exp[e as usize] = idx;
                cur = poly_mulmod(&cur, &g, &f, p);
            }
        }
        let mut log = vec![nn; size as usize];
        for e in 0..nn {
            log[exp[e as usize] as usize] = e;
        }
        // Zech: log(1 + g^d)
        let mut zech = vec![nn; nn as usize];
        for d in 0..nn {
            let idx = exp[d as usize];
            // add 1 to digit 0
            let d0 = idx % p;
            let idx1 = idx - d0 + (d0 + 1) % p;
            zech[d as usize] = log[idx1 as usize];
        }
        Gf {
            p,
            l,
            nn,
            exp,
            log,
            zech,
            half: nn / 2,
        }
    }

    #[inline]
    fn zero(&self) -> u32 {
        self.nn
    }
    #[inline]
    fn is_zero(&self, a: u32) -> bool {
        a == self.nn
    }
    #[inline]
    fn mul(&self, a: u32, b: u32) -> u32 {
        if a == self.nn || b == self.nn {
            return self.nn;
        }
        let s = a + b;
        if s >= self.nn {
            s - self.nn
        } else {
            s
        }
    }
    #[inline]
    fn inv(&self, a: u32) -> u32 {
        debug_assert!(a != self.nn);
        if a == 0 {
            0
        } else {
            self.nn - a
        }
    }
    #[inline]
    fn neg(&self, a: u32) -> u32 {
        if a == self.nn {
            return a;
        }
        let s = a + self.half;
        if s >= self.nn {
            s - self.nn
        } else {
            s
        }
    }
    #[inline]
    fn add(&self, a: u32, b: u32) -> u32 {
        if a == self.nn {
            return b;
        }
        if b == self.nn {
            return a;
        }
        let d = if b >= a { b - a } else { b + self.nn - a };
        let z = self.zech[d as usize];
        if z == self.nn {
            return self.nn;
        }
        let s = a + z;
        if s >= self.nn {
            s - self.nn
        } else {
            s
        }
    }
    #[inline]
    fn sub(&self, a: u32, b: u32) -> u32 {
        self.add(a, self.neg(b))
    }
    /// Constant `c mod p` in log form.
    fn const_int(&self, c: i64) -> u32 {
        let c = c.rem_euclid(self.p as i64) as u32;
        self.log[c as usize]
    }
    /// `nn / (p^d - 1)`: the exponent step of the subfield `F_{p^d}`.
    fn step(&self, d: u32) -> u32 {
        self.nn / ((self.p as u64).pow(d) as u32 - 1)
    }
    fn in_sub(&self, a: u32, d: u32) -> bool {
        a == self.nn || a.is_multiple_of(self.step(d))
    }
    /// Quadratic character of `a` in `F_{p^d}` (`a` must lie there).
    fn chi_sub(&self, a: u32, step: u32) -> i32 {
        if a == self.nn {
            0
        } else if (a / step).is_multiple_of(2) {
            1
        } else {
            -1
        }
    }
    /// Frobenius `a -> a^{p^k}` on log-form elements; `INF` is fixed.
    fn frob(&self, a: u32, pk_mod: u64) -> u32 {
        if a == INF || a == self.nn {
            a
        } else {
            ((a as u64 * pk_mod) % self.nn as u64) as u32
        }
    }
    fn pk_mod(&self, k: u32) -> u64 {
        powmod(self.p as u64, k as u64, self.nn as u64)
    }
    /// Digit vector (ascending) of a log-form element (used by the tests).
    #[cfg_attr(not(test), allow(dead_code))]
    fn digits(&self, a: u32) -> Vec<u32> {
        let mut idx = if a == self.nn {
            0
        } else {
            self.exp[a as usize]
        };
        let mut out = Vec::new();
        for _ in 0..self.l {
            out.push(idx % self.p);
            idx /= self.p;
        }
        out
    }
}

fn gfp_trim(g: &Gf, a: &mut Vec<u32>) {
    while a.last().is_some_and(|&c| g.is_zero(c)) {
        a.pop();
    }
}

fn gfp_rem(g: &Gf, a: &[u32], m: &[u32]) -> Vec<u32> {
    let mut r = a.to_vec();
    gfp_trim(g, &mut r);
    let dm = m.len() - 1;
    let inv = g.inv(m[dm]);
    while r.len() > dm {
        let k = r.len() - 1;
        let c = g.mul(r[k], inv);
        if !g.is_zero(c) {
            for i in 0..=dm {
                r[k - dm + i] = g.sub(r[k - dm + i], g.mul(c, m[i]));
            }
        }
        r.pop();
        gfp_trim(g, &mut r);
    }
    r
}

fn gfp_mulmod(g: &Gf, a: &[u32], b: &[u32], m: &[u32]) -> Vec<u32> {
    let mut prod = vec![g.zero(); a.len() + b.len()];
    for (i, &x) in a.iter().enumerate() {
        if g.is_zero(x) {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            prod[i + j] = g.add(prod[i + j], g.mul(x, y));
        }
    }
    gfp_rem(g, &prod, m)
}

fn gfp_powmod(g: &Gf, a: &[u32], mut e: u64, m: &[u32]) -> Vec<u32> {
    let mut r = vec![0u32]; // one
    let mut b = gfp_rem(g, a, m);
    while e > 0 {
        if e & 1 == 1 {
            r = gfp_mulmod(g, &r, &b, m);
        }
        b = gfp_mulmod(g, &b, &b, m);
        e >>= 1;
    }
    r
}

fn gfp_gcd(g: &Gf, a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    gfp_trim(g, &mut a);
    gfp_trim(g, &mut b);
    while !b.is_empty() {
        let r = gfp_rem(g, &a, &b);
        a = b;
        b = r;
    }
    if let Some(&lc) = a.last() {
        let inv = g.inv(lc);
        for c in a.iter_mut() {
            *c = g.mul(*c, inv);
        }
    }
    a
}

fn gfp_div_exact(g: &Gf, a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut r = a.to_vec();
    gfp_trim(g, &mut r);
    let db = b.len() - 1;
    let mut q = vec![g.zero(); r.len() - db];
    let inv = g.inv(b[db]);
    for k in (db..r.len()).rev() {
        let c = g.mul(r[k], inv);
        q[k - db] = c;
        if !g.is_zero(c) {
            for i in 0..=db {
                r[k - db + i] = g.sub(r[k - db + i], g.mul(c, b[i]));
            }
        }
    }
    gfp_trim(g, &mut q);
    q
}

// ---------------------------------------------------------------------------
// Part G: kernels over F_ℓ.
// ---------------------------------------------------------------------------

/// Largest `2n` handled (`n ≤ 4`).
const DMAX: usize = 8;
type V8 = [u8; DMAX];

#[inline]
fn fl_inv(l: u8, a: u8) -> u8 {
    debug_assert!(!a.is_multiple_of(l));
    (1..l)
        .find(|&b| (a as u16 * b as u16) % l as u16 == 1)
        .expect("inverse mod a prime")
}

#[inline]
fn fl_sub(l: u8, a: u8, b: u8) -> u8 {
    ((a as u16 + l as u16 - b as u16) % l as u16) as u8
}

#[inline]
fn fl_mul(l: u8, a: u8, b: u8) -> u8 {
    ((a as u16 * b as u16) % l as u16) as u8
}

#[inline]
fn fl_add(l: u8, a: u8, b: u8) -> u8 {
    ((a as u16 + b as u16) % l as u16) as u8
}

/// A `2 × 2` matrix over `F_ℓ`, `a[row][col]`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct M2 {
    a: [[u8; 2]; 2],
}

impl M2 {
    fn identity() -> M2 {
        M2 {
            a: [[1, 0], [0, 1]],
        }
    }
    fn mul(self, o: M2, l: u8) -> M2 {
        let mut r = [[0u8; 2]; 2];
        for (i, row) in r.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                let mut s = 0u16;
                for k in 0..2 {
                    s += self.a[i][k] as u16 * o.a[k][j] as u16;
                }
                *cell = (s % l as u16) as u8;
            }
        }
        M2 { a: r }
    }
    fn det(self, l: u8) -> u8 {
        fl_sub(
            l,
            fl_mul(l, self.a[0][0], self.a[1][1]),
            fl_mul(l, self.a[0][1], self.a[1][0]),
        )
    }
    fn tr(self, l: u8) -> u8 {
        fl_add(l, self.a[0][0], self.a[1][1])
    }
    fn apply(self, v: [u8; 2], l: u8) -> [u8; 2] {
        [
            fl_add(
                l,
                fl_mul(l, self.a[0][0], v[0]),
                fl_mul(l, self.a[0][1], v[1]),
            ),
            fl_add(
                l,
                fl_mul(l, self.a[1][0], v[0]),
                fl_mul(l, self.a[1][1], v[1]),
            ),
        ]
    }
    fn transpose(self) -> M2 {
        M2 {
            a: [[self.a[0][0], self.a[1][0]], [self.a[0][1], self.a[1][1]]],
        }
    }
    fn inverse(self, l: u8) -> M2 {
        let d = fl_inv(l, self.det(l));
        M2 {
            a: [
                [
                    fl_mul(l, self.a[1][1], d),
                    fl_mul(l, fl_sub(l, 0, self.a[0][1]), d),
                ],
                [
                    fl_mul(l, fl_sub(l, 0, self.a[1][0]), d),
                    fl_mul(l, self.a[0][0], d),
                ],
            ],
        }
    }
    fn is_identity(self) -> bool {
        self.a == [[1, 0], [0, 1]]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum ClassType {
    Scalar,
    Jordan,
    Split,
    Nonsplit,
}

impl ClassType {
    fn name(self) -> &'static str {
        match self {
            ClassType::Scalar => "scalar",
            ClassType::Jordan => "jordan",
            ClassType::Split => "split",
            ClassType::Nonsplit => "nonsplit",
        }
    }
}

/// A conjugacy class of `GL_2(F_ℓ)`: type, trace, determinant, a representative
/// and alternative representatives (for check 4).
#[derive(Clone, Debug)]
struct Class {
    typ: ClassType,
    tr: u8,
    det: u8,
    rep: M2,
    alts: Vec<M2>,
}

impl Class {
    /// Whether 1 is an eigenvalue (`ℓ | #E` for the curve's Frobenius).
    fn has_eigenvalue_one(&self, l: u8) -> bool {
        (1 + l as u16 + self.det as u16 - self.tr as u16).is_multiple_of(l as u16)
    }
}

/// Every conjugacy class with determinant `d`.
fn classes_with_det(l: u8, d: u8) -> Vec<Class> {
    let g = M2 {
        a: [[1, 1], [0, 1]],
    };
    let ginv = g.inverse(l);
    let conj = |m: M2| g.mul(m, l).mul(ginv, l);
    let mut out = Vec::new();
    for tr in 0..l {
        let roots: Vec<u8> = (0..l)
            .filter(|&x| {
                (x as u16 * x as u16 + d as u16 + l as u16 * l as u16 - tr as u16 * x as u16)
                    .is_multiple_of(l as u16)
            })
            .collect();
        match roots.len() {
            2 => {
                let (lam, mu) = (roots[0], roots[1]);
                let rep = M2 {
                    a: [[lam, 0], [0, mu]],
                };
                out.push(Class {
                    typ: ClassType::Split,
                    tr,
                    det: d,
                    rep,
                    alts: vec![
                        M2 {
                            a: [[mu, 0], [0, lam]],
                        },
                        conj(rep),
                    ],
                });
            }
            1 => {
                let lam = roots[0];
                out.push(Class {
                    typ: ClassType::Scalar,
                    tr,
                    det: d,
                    rep: M2 {
                        a: [[lam, 0], [0, lam]],
                    },
                    alts: vec![],
                });
                let rep = M2 {
                    a: [[lam, 1], [0, lam]],
                };
                out.push(Class {
                    typ: ClassType::Jordan,
                    tr,
                    det: d,
                    rep,
                    alts: vec![
                        M2 {
                            a: [[lam, 0], [1, lam]],
                        },
                        conj(rep),
                    ],
                });
            }
            _ => {
                let rep = M2 {
                    a: [[0, fl_sub(l, 0, d)], [1, tr]],
                };
                out.push(Class {
                    typ: ClassType::Nonsplit,
                    tr,
                    det: d,
                    rep,
                    alts: vec![conj(rep), rep.transpose()],
                });
            }
        }
    }
    out
}

/// `π(v_0, …, v_{n−1}) = (φ v_{n−1}, v_0, …, v_{n−2})`.
fn apply_pi(l: u8, n: usize, phi: M2, v: &V8) -> V8 {
    let mut out = [0u8; DMAX];
    let last = phi.apply([v[2 * (n - 1)], v[2 * (n - 1) + 1]], l);
    out[0] = last[0];
    out[1] = last[1];
    for i in 1..n {
        out[2 * i] = v[2 * (i - 1)];
        out[2 * i + 1] = v[2 * (i - 1) + 1];
    }
    out
}

/// Reduced row echelon form over `F_ℓ` on the first `ncols` coordinates; zero
/// rows are removed. Returns the rank.
fn rref(l: u8, ncols: usize, rows: &mut Vec<V8>) -> usize {
    let mut r = 0usize;
    for c in 0..ncols {
        let Some(piv) = (r..rows.len()).find(|&i| !rows[i][c].is_multiple_of(l)) else {
            continue;
        };
        rows.swap(r, piv);
        let inv = fl_inv(l, rows[r][c]);
        for x in rows[r].iter_mut() {
            *x = fl_mul(l, *x, inv);
        }
        for i in 0..rows.len() {
            if i != r && rows[i][c] != 0 {
                let f = rows[i][c];
                let prow = rows[r];
                for (x, &pv) in rows[i][..ncols].iter_mut().zip(prow[..ncols].iter()) {
                    *x = fl_sub(l, *x, fl_mul(l, f, pv));
                }
            }
        }
        r += 1;
        if r == rows.len() {
            break;
        }
    }
    rows.retain(|row| row[..ncols].iter().any(|&x| x != 0));
    rows.len()
}

#[derive(Clone, Debug)]
struct Sub {
    rows: Vec<V8>,
    dim: usize,
}

impl Sub {
    fn from_rows(l: u8, ncols: usize, mut rows: Vec<V8>) -> Sub {
        let dim = rref(l, ncols, &mut rows);
        Sub { rows, dim }
    }
    fn key(&self, ncols: usize) -> Vec<u8> {
        let mut k = Vec::with_capacity(1 + self.dim * ncols);
        k.push(self.dim as u8);
        for r in &self.rows {
            k.extend_from_slice(&r[..ncols]);
        }
        k
    }
}

fn cyclic_sub(l: u8, n: usize, phi: M2, v: &V8) -> Sub {
    let ncols = 2 * n;
    let mut rows = Vec::with_capacity(ncols);
    let mut w = *v;
    for _ in 0..ncols {
        rows.push(w);
        w = apply_pi(l, n, phi, &w);
    }
    Sub::from_rows(l, ncols, rows)
}

fn sum_sub(l: u8, n: usize, a: &Sub, b: &Sub) -> Sub {
    let mut rows = a.rows.clone();
    rows.extend_from_slice(&b.rows);
    Sub::from_rows(l, 2 * n, rows)
}

/// `ω_V(v, w) = Σ_i q̄^i (v_{2i} w_{2i+1} − v_{2i+1} w_{2i})`.
fn omega(l: u8, n: usize, qbar: u8, v: &V8, w: &V8) -> u8 {
    let mut s = 0u8;
    let mut qi = 1u8;
    for i in 0..n {
        let a = fl_sub(
            l,
            fl_mul(l, v[2 * i], w[2 * i + 1]),
            fl_mul(l, v[2 * i + 1], w[2 * i]),
        );
        s = fl_add(l, s, fl_mul(l, qi, a));
        qi = fl_mul(l, qi, qbar);
    }
    s
}

fn is_isotropic(l: u8, n: usize, qbar: u8, s: &Sub) -> bool {
    for i in 0..s.rows.len() {
        for j in i + 1..s.rows.len() {
            if omega(l, n, qbar, &s.rows[i], &s.rows[j]) != 0 {
                return false;
            }
        }
    }
    true
}

/// Rank of the rows restricted to the coordinates `from..2n`.
fn rank_from(l: u8, n: usize, s: &Sub, from: usize) -> usize {
    let ncols = 2 * n - from;
    let mut rows: Vec<V8> = s
        .rows
        .iter()
        .map(|r| {
            let mut v = [0u8; DMAX];
            v[..ncols].copy_from_slice(&r[from..2 * n]);
            v
        })
        .collect();
    rref(l, ncols, &mut rows)
}

/// `dim Λ ∩ V_0` and `dim Λ ∩ (V_0 ⊕ V_1)`.
fn meet_dims(l: u8, n: usize, s: &Sub) -> (usize, usize) {
    let d0 = s.dim - rank_from(l, n, s, 2);
    let d01 = if n >= 3 {
        s.dim - rank_from(l, n, s, 4)
    } else {
        s.dim
    };
    (d0, d01)
}

/// For `n = 2`, the graph matrix `ψ` of a Tian kernel `{(v, ψ v)}` (rows are
/// in reduced echelon form with pivots in columns 0 and 1).
fn graph_psi(s: &Sub) -> Option<M2> {
    if s.dim != 2
        || s.rows[0][0] != 1
        || s.rows[1][1] != 1
        || s.rows[0][1] != 0
        || s.rows[1][0] != 0
    {
        return None;
    }
    // ψ e_0 = (rows[0][2], rows[0][3]), ψ e_1 = (rows[1][2], rows[1][3]).
    Some(M2 {
        a: [[s.rows[0][2], s.rows[1][2]], [s.rows[0][3], s.rows[1][3]]],
    })
}

#[derive(Clone, Debug)]
struct GRow {
    n: usize,
    l: u8,
    qbar: u8,
    typ: ClassType,
    tr: u8,
    det: u8,
    eig1: bool,
    n_sub: usize,
    n_ker: usize,
    n_tian: usize,
    types: BTreeMap<usize, usize>,
    /// `n = 2`: every Tian kernel is a graph with `det ψ = −q̄^{-1}` and `φ ψ^2 = I`.
    graph_ok: Option<bool>,
    secs: f64,
}

/// The census for one `(n, ℓ, q̄, φ)`: `(n_sub, n_ker, n_tian, types, graph_ok)`.
fn kernel_census(
    l: u8,
    n: usize,
    qbar: u8,
    phi: M2,
) -> (usize, usize, usize, BTreeMap<usize, usize>, Option<bool>) {
    let ncols = 2 * n;
    // Every cyclic submodule of dimension ≤ n.
    let mut seen: HashSet<Vec<u8>> = HashSet::new();
    let mut cyc: Vec<Sub> = Vec::new();
    let mut v = [0u8; DMAX];
    loop {
        // next vector (odometer over F_ℓ^{2n})
        let mut i = 0;
        loop {
            if i == ncols {
                break;
            }
            v[i] += 1;
            if v[i] == l {
                v[i] = 0;
                i += 1;
            } else {
                break;
            }
        }
        if i == ncols {
            break;
        }
        let s = cyclic_sub(l, n, phi, &v);
        if s.dim <= n && seen.insert(s.key(ncols)) {
            cyc.push(s);
        }
    }
    // Closure under sums, with the dimension bound.
    let mut all: Vec<Sub> = cyc.clone();
    let mut queue: Vec<usize> = (0..all.len()).collect();
    while let Some(ai) = queue.pop() {
        if all[ai].dim == n {
            continue;
        }
        for c in &cyc {
            if all[ai].dim + c.dim > 2 * n {
                continue;
            }
            let s = sum_sub(l, n, &all[ai], c);
            if s.dim <= n && seen.insert(s.key(ncols)) {
                all.push(s);
                queue.push(all.len() - 1);
            }
        }
    }
    let mut n_sub = 0;
    let mut n_ker = 0;
    let mut n_tian = 0;
    let mut types = BTreeMap::new();
    let mut graph_ok = if n == 2 { Some(true) } else { None };
    for s in all.iter().filter(|s| s.dim == n) {
        n_sub += 1;
        if !is_isotropic(l, n, qbar, s) {
            continue;
        }
        n_ker += 1;
        let (d0, d01) = meet_dims(l, n, s);
        if d0 != 0 {
            continue;
        }
        n_tian += 1;
        *types.entry(d01).or_insert(0) += 1;
        if n == 2 {
            let ok = match graph_psi(s) {
                Some(psi) => {
                    let want = fl_sub(l, 0, fl_inv(l, qbar));
                    psi.det(l) == want && phi.mul(psi.mul(psi, l), l).is_identity()
                }
                None => false,
            };
            if !ok {
                graph_ok = Some(false);
            }
        }
    }
    (n_sub, n_ker, n_tian, types, graph_ok)
}

/// `(n, ℓ)` cells of the frozen range.
fn part_g_cells() -> Vec<(usize, u8)> {
    vec![
        (2, 2),
        (2, 3),
        (2, 5),
        (2, 7),
        (3, 2),
        (3, 3),
        (3, 5),
        (3, 7),
        (4, 2),
        (4, 3),
        (4, 5),
    ]
}

type GKey = (usize, u8, u8, ClassType, u8, u8);

fn part_g(out: &Path, log: &mut File, budget_secs: f64) -> HashMap<GKey, (usize, usize)> {
    let t0 = Instant::now();
    let mut rows: Vec<GRow> = Vec::new();
    let mut checks = String::new();
    let mut check4_fail = 0usize;
    let mut not_run = Vec::new();
    for (n, l) in part_g_cells() {
        if t0.elapsed().as_secs_f64() > budget_secs {
            not_run.push((n, l));
            continue;
        }
        let tc = Instant::now();
        let qbars: Vec<u8> = (1..l).collect();
        for qbar in qbars {
            let d = {
                let mut x = 1u8;
                for _ in 0..n {
                    x = fl_mul(l, x, qbar);
                }
                x
            };
            for cls in classes_with_det(l, d) {
                let ts = Instant::now();
                let (n_sub, n_ker, n_tian, types, graph_ok) = kernel_census(l, n, qbar, cls.rep);
                for alt in &cls.alts {
                    let (_, k2, t2, _, _) = kernel_census(l, n, qbar, *alt);
                    if (k2, t2) != (n_ker, n_tian) {
                        check4_fail += 1;
                        let _ = writeln!(
                            log,
                            "CHECK4 MISMATCH n={n} l={l} qbar={qbar} {} tr={} det={}: rep ({n_ker},{n_tian}) alt ({k2},{t2})",
                            cls.typ.name(),
                            cls.tr,
                            cls.det
                        );
                    }
                }
                rows.push(GRow {
                    n,
                    l,
                    qbar,
                    typ: cls.typ,
                    tr: cls.tr,
                    det: cls.det,
                    eig1: cls.has_eigenvalue_one(l),
                    n_sub,
                    n_ker,
                    n_tian,
                    types,
                    graph_ok,
                    secs: ts.elapsed().as_secs_f64(),
                });
            }
        }
        let _ = writeln!(
            log,
            "part G cell n={n} l={l}: {:.2}s",
            tc.elapsed().as_secs_f64()
        );
    }
    // kernels.csv
    let mut f = File::create(out.join("kernels.csv")).expect("kernels.csv");
    writeln!(
        f,
        "n,ell,qbar,class,tr,det,eigenvalue_one,submodules_dim_n,kernels,tian_kernels,types_d01,graph_ok,seconds"
    )
    .unwrap();
    for r in &rows {
        let types: Vec<String> = r.types.iter().map(|(k, v)| format!("{k}:{v}")).collect();
        writeln!(
            f,
            "{},{},{},{},{},{},{},{},{},{},{},{},{:.3}",
            r.n,
            r.l,
            r.qbar,
            r.typ.name(),
            r.tr,
            r.det,
            r.eig1,
            r.n_sub,
            r.n_ker,
            r.n_tian,
            if types.is_empty() {
                "-".to_string()
            } else {
                types.join(";")
            },
            match r.graph_ok {
                Some(true) => "true",
                Some(false) => "false",
                None => "na",
            },
            r.secs
        )
        .unwrap();
    }
    // Checks 1–4 and the P2 control.
    let c1_bad: Vec<&GRow> = rows
        .iter()
        .filter(|r| r.n == 3 && r.l == 2 && r.n_tian > 0 && r.typ != ClassType::Scalar)
        .collect();
    checks.push_str(&format!(
        "check1 (n=3, l=2: Tian kernels only for phi = I): violations={}\n",
        c1_bad.len()
    ));
    let c2_bad = rows
        .iter()
        .filter(|r| r.n == 2 && r.graph_ok == Some(false))
        .count();
    checks.push_str(&format!(
        "check2 (n=2: every Tian kernel is a graph with det psi = -qbar^-1 and phi psi^2 = I): violations={c2_bad}\n"
    ));
    let p2 = rows
        .iter()
        .filter(|r| r.n == 3 && r.l == 3 && r.qbar == 1 && !r.eig1 && r.n_tian > 0)
        .count();
    checks.push_str(&format!(
        "check3 (P2: at (n,l,qbar)=(3,3,1) a class without eigenvalue 1 has a Tian kernel): classes={p2}\n"
    ));
    checks.push_str(&format!(
        "check4 (representative independence): mismatches={check4_fail}\n"
    ));
    for (n, l) in &not_run {
        checks.push_str(&format!("not run (budget): n={n} l={l}\n"));
    }
    std::fs::write(out.join("checks_g.txt"), &checks).unwrap();
    let _ = write!(log, "{checks}");
    rows.iter()
        .map(|r| ((r.n, r.l, r.qbar, r.typ, r.tr, r.det), (r.n_ker, r.n_tian)))
        .collect()
}

// ---------------------------------------------------------------------------
// Part H: every curve over F_{q^3}.
// ---------------------------------------------------------------------------

fn gfp_add(g: &Gf, a: &[u32], b: &[u32]) -> Vec<u32> {
    let n = a.len().max(b.len());
    let mut out = vec![g.zero(); n];
    for (i, o) in out.iter_mut().enumerate() {
        let x = a.get(i).copied().unwrap_or(g.zero());
        let y = b.get(i).copied().unwrap_or(g.zero());
        *o = g.add(x, y);
    }
    gfp_trim(g, &mut out);
    out
}

fn gfp_sub_poly(g: &Gf, a: &[u32], b: &[u32]) -> Vec<u32> {
    let nb: Vec<u32> = b.iter().map(|&c| g.neg(c)).collect();
    gfp_add(g, a, &nb)
}

fn gfp_mul(g: &Gf, a: &[u32], b: &[u32]) -> Vec<u32> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut prod = vec![g.zero(); a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        if g.is_zero(x) {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            prod[i + j] = g.add(prod[i + j], g.mul(x, y));
        }
    }
    gfp_trim(g, &mut prod);
    prod
}

fn gfp_scale(g: &Gf, a: &[u32], c: u32) -> Vec<u32> {
    let mut out: Vec<u32> = a.iter().map(|&x| g.mul(x, c)).collect();
    gfp_trim(g, &mut out);
    out
}

fn gfp_monic(g: &Gf, a: &[u32]) -> Vec<u32> {
    let mut f = a.to_vec();
    gfp_trim(g, &mut f);
    if let Some(&lc) = f.last() {
        let inv = g.inv(lc);
        for c in f.iter_mut() {
            *c = g.mul(*c, inv);
        }
    }
    f
}

fn gfp_derivative(g: &Gf, f: &[u32]) -> Vec<u32> {
    let mut out = Vec::with_capacity(f.len().saturating_sub(1));
    for (i, &c) in f.iter().enumerate().skip(1) {
        out.push(g.mul(c, g.const_int(i as i64)));
    }
    gfp_trim(g, &mut out);
    out
}

/// Distinct-degree factorisation pattern (sorted degrees of the irreducible
/// factors) of a squarefree `f` over the table field; also returns whether
/// `f` was squarefree.
fn ddf_pattern(g: &Gf, f: &[u32]) -> (Vec<usize>, bool) {
    let mut f = gfp_monic(g, f);
    let q = g.nn as u64 + 1;
    let x = vec![g.zero(), 0u32];
    let squarefree = {
        let d = gfp_derivative(g, &f);
        let h = gfp_gcd(g, &f, &d);
        h.len() == 1
    };
    let mut pattern = Vec::new();
    let mut xp = x.clone();
    let mut d = 1usize;
    while f.len() > 2 * d {
        xp = gfp_powmod(g, &xp, q, &f);
        let h = gfp_sub_poly(g, &xp, &x);
        let h = if h.is_empty() { vec![g.zero()] } else { h };
        let gc = gfp_gcd(g, &f, &h);
        if gc.len() > 1 {
            let k = (gc.len() - 1) / d;
            for _ in 0..k {
                pattern.push(d);
            }
            f = gfp_div_exact(g, &f, &gc);
            if f.len() > 1 {
                xp = gfp_rem(g, &xp, &f);
            }
        }
        d += 1;
    }
    if f.len() > 1 {
        pattern.push(f.len() - 1);
    }
    pattern.sort_unstable();
    (pattern, squarefree)
}

/// A curve `y^2 = x^3 + a_2 x^2 + a_4 x + a_6` over the table field.
#[derive(Clone, Debug)]
struct Curve {
    label: String,
    j: u32,
    twist: u32,
    a2: u32,
    a4: u32,
    a6: u32,
}

fn curves_over(g: &Gf) -> (Vec<Curve>, usize) {
    let mut out = Vec::new();
    let zero = g.zero();
    let one = 0u32;
    let nu = 1u32; // the generator: a non-square
    let big_q = g.nn as u64 + 1;
    let mut j0_orbits = 0usize;
    if g.p >= 5 {
        let c1728 = g.const_int(1728);
        let c3 = g.const_int(3);
        let c2 = g.const_int(2);
        let gcd4 = gcd_u64(4, big_q - 1) as u32;
        let gcd6 = gcd_u64(6, big_q - 1) as u32;
        // j = 0: y^2 = x^3 + b, b over F^x/(F^x)^6.
        for k in 0..gcd6 {
            out.push(Curve {
                label: "0".into(),
                j: zero,
                twist: k,
                a2: zero,
                a4: zero,
                a6: k,
            });
        }
        // j = 1728: y^2 = x^3 + a x, a over F^x/(F^x)^4.
        for k in 0..gcd4 {
            out.push(Curve {
                label: "1728".into(),
                j: c1728,
                twist: k,
                a2: zero,
                a4: k,
                a6: zero,
            });
        }
        for j in 0..g.nn {
            if j == c1728 {
                continue;
            }
            let k = g.mul(j, g.inv(g.sub(c1728, j)));
            let a = g.mul(c3, k);
            let b = g.mul(c2, k);
            out.push(Curve {
                label: format!("{j}"),
                j,
                twist: 0,
                a2: zero,
                a4: a,
                a6: b,
            });
            out.push(Curve {
                label: format!("{j}"),
                j,
                twist: 1,
                a2: zero,
                a4: g.mul(a, g.mul(nu, nu)),
                a6: g.mul(b, g.mul(nu, g.mul(nu, nu))),
            });
        }
    } else {
        // p = 3: j ≠ 0 as y^2 = x^3 + x^2 − 1/j and its twist; j = 0 as orbits.
        for j in 0..g.nn {
            let a6 = g.neg(g.inv(j));
            out.push(Curve {
                label: format!("{j}"),
                j,
                twist: 0,
                a2: one,
                a4: zero,
                a6,
            });
            let nu3 = g.mul(nu, g.mul(nu, nu));
            out.push(Curve {
                label: format!("{j}"),
                j,
                twist: 1,
                a2: nu,
                a4: zero,
                a6: g.mul(a6, nu3),
            });
        }
        // Orbits of (a4 ≠ 0, a6) under (u, r): (a4 u^{-4}, (r^3 + a4 r + a6) u^{-6}).
        let elems: Vec<u32> = (0..=g.nn).collect(); // log forms, nn = zero
        let idx = |a4: u32, a6: u32| -> usize { (a4 as usize) * elems.len() + a6 as usize };
        let total = g.nn as usize * elems.len();
        let mut parent: Vec<usize> = (0..total).collect();
        fn find(p: &mut [usize], mut x: usize) -> usize {
            while p[x] != x {
                p[x] = p[p[x]];
                x = p[x];
            }
            x
        }
        for a4 in 0..g.nn {
            for &a6 in &elems {
                let i = idx(a4, a6);
                for u in 0..g.nn {
                    let u2 = g.mul(u, u);
                    let u4 = g.mul(u2, u2);
                    let u6 = g.mul(u4, u2);
                    let na4 = g.mul(a4, g.inv(u4));
                    for &r in &elems {
                        let r3 = g.mul(r, g.mul(r, r));
                        let na6 = g.mul(g.add(g.add(r3, g.mul(a4, r)), a6), g.inv(u6));
                        let jx = idx(na4, na6);
                        let (ri, rj) = (find(&mut parent, i), find(&mut parent, jx));
                        if ri != rj {
                            parent[ri] = rj;
                        }
                    }
                }
            }
        }
        let mut reps: BTreeMap<usize, (u32, u32)> = BTreeMap::new();
        for a4 in 0..g.nn {
            for &a6 in &elems {
                let r = find(&mut parent, idx(a4, a6));
                reps.entry(r).or_insert((a4, a6));
            }
        }
        for (k, (a4, a6)) in reps.values().enumerate() {
            out.push(Curve {
                label: "0ss".into(),
                j: zero,
                twist: k as u32,
                a2: zero,
                a4: *a4,
                a6: *a6,
            });
        }
        j0_orbits = reps.len();
    }
    (out, j0_orbits)
}

fn gcd_u64(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd_u64(b, a % b)
    }
}

fn count_points(g: &Gf, c: &Curve) -> u64 {
    let mut n: i64 = g.nn as i64 + 2; // q + 1
    for x in 0..=g.nn {
        let x2 = g.mul(x, x);
        let x3 = g.mul(x2, x);
        let v = g.add(g.add(x3, g.mul(c.a2, x2)), g.add(g.mul(c.a4, x), c.a6));
        if g.is_zero(v) {
            continue;
        }
        n += if v.is_multiple_of(2) { 1 } else { -1 };
    }
    n as u64
}

/// The polynomial whose roots are the `x`-coordinates of `E[ℓ] ∖ 0`:
/// the cubic for `ℓ = 2`, the division polynomial `ψ_ℓ` (in `b`-invariants)
/// for `ℓ = 3, 5, 7`.
fn torsion_x_poly(g: &Gf, c: &Curve, l: u8) -> Vec<u32> {
    let k = |v: i64| g.const_int(v);
    let one = 0u32;
    if l == 2 {
        return vec![c.a6, c.a4, c.a2, one];
    }
    let b2 = g.mul(k(4), c.a2);
    let b4 = g.mul(k(2), c.a4);
    let b6 = g.mul(k(4), c.a6);
    let b8 = g.sub(g.mul(k(4), g.mul(c.a2, c.a6)), g.mul(c.a4, c.a4));
    let f2 = vec![b6, g.mul(k(2), b4), b2, k(4)];
    let psi3 = vec![b8, g.mul(k(3), b6), g.mul(k(3), b4), b2, k(3)];
    if l == 3 {
        return psi3;
    }
    let f4 = vec![
        g.sub(g.mul(b4, b8), g.mul(b6, b6)),
        g.sub(g.mul(b2, b8), g.mul(b4, b6)),
        g.mul(k(10), b8),
        g.mul(k(10), b6),
        g.mul(k(5), b4),
        b2,
        k(2),
    ];
    let f2sq = gfp_mul(g, &f2, &f2);
    let psi3cu = gfp_mul(g, &psi3, &gfp_mul(g, &psi3, &psi3));
    let psi5 = gfp_sub_poly(g, &gfp_mul(g, &f4, &f2sq), &psi3cu);
    if l == 5 {
        return psi5;
    }
    let f4cu = gfp_mul(g, &f4, &gfp_mul(g, &f4, &f4));
    let psi7 = gfp_sub_poly(g, &gfp_mul(g, &psi5, &psi3cu), &gfp_mul(g, &f2sq, &f4cu));
    assert_eq!(l, 7);
    psi7
}

/// Orbit sizes of `φ` on `(F_ℓ^2 ∖ 0)/±1`, sorted.
fn orbit_pattern(l: u8, phi: M2) -> Vec<usize> {
    let mut seen = vec![false; (l as usize) * (l as usize)];
    let idx = |v: [u8; 2]| v[0] as usize * l as usize + v[1] as usize;
    let neg = |v: [u8; 2]| [fl_sub(l, 0, v[0]), fl_sub(l, 0, v[1])];
    let mut pattern = Vec::new();
    for a in 0..l {
        for b in 0..l {
            let v = [a, b];
            if v == [0, 0] || seen[idx(v)] {
                continue;
            }
            let mut w = v;
            let mut len = 0usize;
            loop {
                seen[idx(w)] = true;
                seen[idx(neg(w))] = true;
                w = phi.apply(w, l);
                len += 1;
                if w == v || w == neg(v) {
                    break;
                }
            }
            pattern.push(len);
        }
    }
    pattern.sort_unstable();
    pattern
}

/// The class of `φ` from `(tr, det)` and the factorisation pattern.
fn class_from_pattern(l: u8, tr: u8, det: u8, pattern: &[usize]) -> Option<Class> {
    let cands: Vec<Class> = classes_with_det(l, det)
        .into_iter()
        .filter(|c| c.tr == tr && orbit_pattern(l, c.rep) == pattern)
        .collect();
    if cands.len() == 1 {
        cands.into_iter().next()
    } else {
        None
    }
}

type ByClassKey = (u32, u8, u8, String, u8, u8);
type ByClassVal = (usize, usize, usize, bool);

#[derive(Default, Clone)]
struct QSummary {
    classes: usize,
    prime: usize,
    cof2: usize,
    full2: usize,
    tian: BTreeMap<u8, (usize, usize, usize)>, // l -> (all, prime, cof2)
    prime_any_odd: usize,
    all_any_odd: usize,
}

fn part_h(out: &Path, log: &mut File, gmap: &HashMap<GKey, (usize, usize)>, qs: &[u32]) {
    let ells: [u8; 4] = [2, 3, 5, 7];
    let mut curves_f = File::create(out.join("curves.csv")).expect("curves.csv");
    writeln!(
        curves_f,
        "q,j,twist,a2,a4,a6,t,numE,order_class,ell,qbar,pattern,class,tr,det,eigenvalue_one,tian"
    )
    .unwrap();
    let mut summary_f = File::create(out.join("summary.csv")).expect("summary.csv");
    writeln!(
        summary_f,
        "q,classes,prime_order,cofactor_2,full_2_torsion,ell,with_tian,prime_with_tian,cof2_with_tian,prime_any_odd_ell,all_any_odd_ell,seconds"
    )
    .unwrap();
    let mut byclass: BTreeMap<ByClassKey, ByClassVal> = BTreeMap::new();
    let mut checks = String::new();
    for &q in qs {
        let tq = Instant::now();
        let g = Gf::new(q, 3);
        let big_q = g.nn as u64 + 1;
        let (curves, j0_orbits) = curves_over(&g);
        let expected = if q >= 5 {
            2 * (big_q as usize - 2)
                + gcd_u64(4, big_q - 1) as usize
                + gcd_u64(6, big_q - 1) as usize
        } else {
            2 * (big_q as usize - 1) + j0_orbits
        };
        checks.push_str(&format!(
            "q={q} check5 classes={} expected={} {}\n",
            curves.len(),
            expected,
            if curves.len() == expected {
                "ok"
            } else {
                "MISMATCH"
            }
        ));
        let mut s = QSummary::default();
        let mut c6_bad = 0usize;
        let mut c7_bad = 0usize;
        let mut c8_bad = 0usize;
        let mut hasse_bad = 0usize;
        let mut class_fail = 0usize;
        for c in &curves {
            let num = count_points(&g, c);
            let t = big_q as i64 + 1 - num as i64;
            if (t as f64).abs() > 2.0 * (big_q as f64).sqrt() {
                hasse_bad += 1;
            }
            let order_class = if is_prime(num) {
                "prime"
            } else if num.is_multiple_of(2) && is_prime(num / 2) {
                "cof2"
            } else {
                "other"
            };
            s.classes += 1;
            if order_class == "prime" {
                s.prime += 1;
            }
            if order_class == "cof2" {
                s.cof2 += 1;
            }
            let mut any_odd = false;
            for &l in &ells {
                if l as u32 == q {
                    continue;
                }
                let poly = torsion_x_poly(&g, c, l);
                let (pattern, squarefree) = ddf_pattern(&g, &poly);
                let deg = poly.len() - 1;
                // The cubic for ℓ = 2 (degree ℓ^2 − 1, since x identifies P and −P = P);
                // ψ_ℓ of degree (ℓ^2 − 1)/2 for odd ℓ.
                let want_deg = if l == 2 {
                    3
                } else {
                    ((l as usize) * (l as usize) - 1) / 2
                };
                if deg != want_deg || !squarefree {
                    c6_bad += 1;
                }
                let tr = (t.rem_euclid(l as i64)) as u8;
                let det = (big_q % l as u64) as u8;
                let qbar = (q % l as u32) as u8;
                let Some(cls) = class_from_pattern(l, tr, det, &pattern) else {
                    class_fail += 1;
                    let _ = writeln!(
                        log,
                        "CLASS NOT FOUND q={q} curve={} twist={} l={l} tr={tr} det={det} pattern={:?}",
                        c.label, c.twist, pattern
                    );
                    continue;
                };
                let eig1 = cls.has_eigenvalue_one(l);
                if l == 2 {
                    let full = cls.typ == ClassType::Scalar;
                    let splits = pattern == [1, 1, 1];
                    if full != splits || (full && !num.is_multiple_of(4)) {
                        c7_bad += 1;
                    }
                    if full {
                        s.full2 += 1;
                    }
                }
                let tian = gmap
                    .get(&(3usize, l, qbar, cls.typ, cls.tr, cls.det))
                    .map(|&(_, nt)| nt > 0);
                let tian_s = match tian {
                    Some(true) => "1",
                    Some(false) => "0",
                    None => "na",
                };
                if tian == Some(true) {
                    let e = s.tian.entry(l).or_insert((0, 0, 0));
                    e.0 += 1;
                    if order_class == "prime" {
                        e.1 += 1;
                    }
                    if order_class == "cof2" {
                        e.2 += 1;
                    }
                    if l != 2 {
                        any_odd = true;
                    }
                    if l == 2 && cls.typ != ClassType::Scalar {
                        c7_bad += 1;
                    }
                }
                if order_class == "prime" && (eig1 || (l == 2 && tian == Some(true))) {
                    c8_bad += 1;
                }
                let pat: Vec<String> = pattern.iter().map(|d| d.to_string()).collect();
                writeln!(
                    curves_f,
                    "{q},{},{},{},{},{},{t},{num},{order_class},{l},{qbar},{},{},{},{},{},{}",
                    c.label,
                    c.twist,
                    c.a2,
                    c.a4,
                    c.a6,
                    pat.join("+"),
                    cls.typ.name(),
                    cls.tr,
                    cls.det,
                    eig1,
                    tian_s
                )
                .unwrap();
                let e = byclass
                    .entry((q, l, qbar, cls.typ.name().to_string(), cls.tr, cls.det))
                    .or_insert((0, 0, 0, tian == Some(true)));
                e.0 += 1;
                if order_class == "prime" {
                    e.1 += 1;
                }
                if order_class == "cof2" {
                    e.2 += 1;
                }
            }
            if any_odd {
                s.all_any_odd += 1;
                if order_class == "prime" {
                    s.prime_any_odd += 1;
                }
            }
        }
        let secs = tq.elapsed().as_secs_f64();
        for &l in &ells {
            if l as u32 == q {
                continue;
            }
            let (a, p, c2) = s.tian.get(&l).copied().unwrap_or((0, 0, 0));
            writeln!(
                summary_f,
                "{q},{},{},{},{},{l},{a},{p},{c2},{},{},{:.1}",
                s.classes, s.prime, s.cof2, s.full2, s.prime_any_odd, s.all_any_odd, secs
            )
            .unwrap();
        }
        checks.push_str(&format!(
            "q={q} hasse_violations={hasse_bad} class_not_found={class_fail} check6_violations={c6_bad} check7_violations={c7_bad} check8_violations={c8_bad}\n"
        ));
        checks.push_str(&format!(
            "q={q} check9 prime_order={} prime_with_tian_some_odd_ell={} all={} all_with_tian_some_odd_ell={}\n",
            s.prime, s.prime_any_odd, s.classes, s.all_any_odd
        ));
        let _ = writeln!(log, "part H q={q}: {} curves, {:.1}s", curves.len(), secs);
    }
    let mut bc = File::create(out.join("by_class.csv")).expect("by_class.csv");
    writeln!(
        bc,
        "q,ell,qbar,class,tr,det,curves,prime_order_curves,cofactor_2_curves,tian"
    )
    .unwrap();
    for ((q, l, qbar, typ, tr, det), (n, np, nc, tian)) in &byclass {
        writeln!(
            bc,
            "{q},{l},{qbar},{typ},{tr},{det},{n},{np},{nc},{}",
            *tian as u8
        )
        .unwrap();
    }
    std::fs::write(out.join("checks_h.txt"), &checks).unwrap();
    let _ = write!(log, "{checks}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} <out-dir> [G|H ...]", args[0]);
        std::process::exit(2);
    }
    let out = Path::new(&args[1]);
    std::fs::create_dir_all(out).expect("out dir");
    let parts: Vec<String> = if args.len() > 2 {
        args[2..].to_vec()
    } else {
        vec!["G".into(), "H".into()]
    };
    let mut log = File::create(out.join("tool_log.txt")).expect("tool_log.txt");
    let t0 = Instant::now();
    let budget_g: f64 = std::env::var("WRIC_BUDGET_G")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3600.0);
    let qs: Vec<u32> = std::env::var("WRIC_QS")
        .ok()
        .map(|s| s.split(',').filter_map(|x| x.parse().ok()).collect())
        .unwrap_or_else(|| vec![3, 5, 7, 11, 13]);
    let mut gmap = HashMap::new();
    if parts.iter().any(|p| p == "G") {
        gmap = part_g(out, &mut log, budget_g);
    }
    if parts.iter().any(|p| p == "H") {
        if gmap.is_empty() {
            // Part H alone still needs the n = 3 rows of Part G.
            let tmp = out.join("g_for_h");
            std::fs::create_dir_all(&tmp).unwrap();
            gmap = part_g(&tmp, &mut log, budget_g);
        }
        part_h(out, &mut log, &gmap, &qs);
    }
    let _ = writeln!(log, "total {:.1}s", t0.elapsed().as_secs_f64());
    eprintln!("done in {:.1}s", t0.elapsed().as_secs_f64());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rref_rank() {
        let mut rows = vec![
            [1, 2, 0, 0, 0, 0, 0, 0],
            [2, 4, 0, 0, 0, 0, 0, 0],
            [0, 1, 1, 0, 0, 0, 0, 0],
        ];
        assert_eq!(rref(5, 4, &mut rows), 2);
    }

    /// `n = 2`: the census agrees with the closed form of Amendment 1 (a Tian
    /// kernel exists iff `φ^{-1}` has a square root `ψ` with `det ψ = −q̄^{-1}`),
    /// checked against a brute-force search over `GL_2(F_ℓ)` at `ℓ = 3, 5`.
    #[test]
    fn n2_closed_form() {
        for l in [3u8, 5] {
            for qbar in 1..l {
                let d = fl_mul(l, qbar, qbar);
                for cls in classes_with_det(l, d) {
                    let (_, _, n_tian, _, graph_ok) = kernel_census(l, 2, qbar, cls.rep);
                    let phi_inv = cls.rep.inverse(l);
                    let want = fl_sub(l, 0, fl_inv(l, qbar));
                    let mut exists = 0usize;
                    for a in 0..l {
                        for b in 0..l {
                            for c in 0..l {
                                for e in 0..l {
                                    let psi = M2 {
                                        a: [[a, b], [c, e]],
                                    };
                                    if psi.det(l) == want && psi.mul(psi, l) == phi_inv {
                                        exists += 1;
                                    }
                                }
                            }
                        }
                    }
                    assert_eq!(n_tian, exists, "l={l} qbar={qbar} {:?}", cls);
                    assert_ne!(graph_ok, Some(false));
                }
            }
        }
    }

    #[test]
    fn orbit_patterns() {
        assert_eq!(orbit_pattern(3, M2::identity()), vec![1, 1, 1, 1]);
        assert_eq!(
            orbit_pattern(
                3,
                M2 {
                    a: [[1, 1], [0, 1]]
                }
            ),
            vec![1, 3]
        );
        assert_eq!(
            orbit_pattern(
                2,
                M2 {
                    a: [[0, 1], [1, 1]]
                }
            ),
            vec![3]
        );
        // 2^2 = 3^2 = −1 mod 5: every class returns to ±itself after two steps.
        assert_eq!(
            orbit_pattern(
                5,
                M2 {
                    a: [[2, 0], [0, 3]]
                }
            ),
            vec![2; 6]
        );
        // classes are vectors modulo ±1, not points of P^1: (a,0) and (0,b) are fixed.
        assert_eq!(
            orbit_pattern(
                5,
                M2 {
                    a: [[1, 0], [0, 4]]
                }
            ),
            vec![1, 1, 1, 1, 2, 2, 2, 2]
        );
    }

    /// Division polynomials: degree `(ℓ^2 − 1)/2`, squarefree, and the number
    /// of rational `x`-roots of `ψ_3` is `(#E[3](F) − 1)/2` for a curve over
    /// `F_{11^3}` whose 3-torsion is read off independently from `#E mod 3`.
    #[test]
    fn division_polynomials() {
        let g = Gf::new(11, 3);
        let (curves, _) = curves_over(&g);
        for c in curves.iter().step_by(97).take(20) {
            for l in [3u8, 5, 7] {
                let f = torsion_x_poly(&g, c, l);
                assert_eq!(f.len() - 1, ((l as usize) * (l as usize) - 1) / 2);
                let (pattern, sqf) = ddf_pattern(&g, &f);
                assert!(sqf);
                assert_eq!(pattern.iter().sum::<usize>(), f.len() - 1);
            }
            let num = count_points(&g, c);
            let (p3, _) = ddf_pattern(&g, &torsion_x_poly(&g, c, 3));
            let rational = p3.iter().filter(|&&d| d == 1).count();
            // Rational x-roots come in whole subgroups only if the points are
            // rational; a rational subgroup contributes 1 root. #E[3](F) ∈ {1,3,9}.
            if !num.is_multiple_of(3) {
                assert_eq!(rational, 0);
            }
            if rational == 4 {
                assert_eq!(num % 9, 0);
            }
        }
    }

    #[test]
    fn two_torsion_and_order() {
        let g = Gf::new(7, 3);
        let (curves, _) = curves_over(&g);
        for c in &curves {
            let num = count_points(&g, c);
            let (p2, _) = ddf_pattern(&g, &torsion_x_poly(&g, c, 2));
            let roots = p2.iter().filter(|&&d| d == 1).count();
            assert_eq!(num.is_multiple_of(2), roots >= 1);
            if roots == 3 {
                assert_eq!(num % 4, 0);
            }
        }
    }

    #[test]
    fn class_count_p5() {
        let g = Gf::new(5, 3);
        let (curves, _) = curves_over(&g);
        let q3 = 125u64;
        assert_eq!(
            curves.len(),
            2 * (q3 as usize - 2) + gcd_u64(4, q3 - 1) as usize + gcd_u64(6, q3 - 1) as usize
        );
    }
}

#[cfg(test)]
mod module_theory_tests {
    use super::*;

    /// `n = 3`, `ℓ = 7`, `φ = J(1)`: `V ≅ F_7[T]/((T^3 − 1)^2)` is cyclic and
    /// `T^3 − 1` splits, so the submodules of dimension 3 are the seven
    /// degree-3 divisors; three of them are isotropic and two meet no factor.
    #[test]
    fn jordan_seven_divisors() {
        let phi = M2 {
            a: [[1, 1], [0, 1]],
        };
        let (n_sub, n_ker, n_tian, _, _) = kernel_census(7, 3, 1, phi);
        assert_eq!((n_sub, n_ker, n_tian), (7, 3, 2));
    }

    /// Nonsplit `φ`: `χ_φ(T^3)` has no factor of odd degree, so no submodule
    /// of dimension 3 exists at all (`n = 3`, every `ℓ ≤ 7`).
    #[test]
    fn nonsplit_has_no_dim3_submodule() {
        for l in [3u8, 5, 7] {
            for qbar in 1..l {
                let d = fl_mul(l, fl_mul(l, qbar, qbar), qbar);
                for cls in classes_with_det(l, d) {
                    if cls.typ == ClassType::Nonsplit {
                        let (n_sub, _, _, _, _) = kernel_census(l, 3, qbar, cls.rep);
                        assert_eq!(n_sub, 0, "l={l} qbar={qbar} tr={}", cls.tr);
                    }
                }
            }
        }
    }
}
