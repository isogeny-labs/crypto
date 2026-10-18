//! The `n = 2` routes end to end: rho on `E(F_{p^2})`, genus-2 index calculus
//! in `Jac(C)(F_p)` after a Weil-restriction transfer, and Gaudry's
//! decomposition in `E(F_{p^2})` with the Weil-descended summation polynomial.
//!
//! Round 3, items 1 and 3 (`research/weil_descent_angles_20261018/PROTOCOL.md`,
//! Addendum A, Parts O/J; Amendment C).
//!
//! Construction (Scholten type). `C : y^2 = f(x)` over `F_p` with
//! `x^6 f(κ/x) = κ^3 f(x)`, `κ` a non-square, `f = (x − r)(x − κ/r)·g`. With
//! `s^2 = κ`, `u = x + κ/x` and `v = y(x + s)/x^2` give
//! `φ : C → E : v^2 = h(u)(u + 2s)` over `F_{p^2}`, `f(x) = x^3 h(u)`, and
//! `Jac(C) ~ Res_{F_{p^2}/F_p} E`. The transfer is `τ(P) = φ^*P + (φ^*P)^σ` in
//! the quintic model of `C` obtained by sending the rational Weierstrass point
//! `x = r` to infinity.
//!
//! Costs are counted in `F_p` multiplications (`M`) and inversions (`I`),
//! with `F_{p^2}` multiply = 3 M, square = 2 M, inverse = I + 4 M; native
//! operations (hashing, table lookups, linear algebra mod `ℓ`) are counted
//! separately and priced afterwards at 0, 0.1 and 1 M.
//!
//! std-only; `rustc --edition 2021 -O examples/n2_routes.rs`.
//! Usage: `n2_routes <out-dir> [run|check|control|summarize|hsweep|memsweep|hsummarize|gluecheck|glue|gluesummarize]`; env `N2_THREADS`, `N2_SIZES`
//! (comma-separated `log2 #E` targets, default `20,24,28,32,36,40`).
#![allow(dead_code)]
#![allow(clippy::needless_range_loop)]

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

// ---------------------------------------------------------------------------
// Thread-local field context and operation counters.
// ---------------------------------------------------------------------------

thread_local! {
    static P: Cell<u64> = const { Cell::new(0) };
    static KAPPA: Cell<u64> = const { Cell::new(0) };
    static CM: Cell<u64> = const { Cell::new(0) };
    static CI: Cell<u64> = const { Cell::new(0) };
    static CN: Cell<u64> = const { Cell::new(0) };
    static CJ: Cell<u64> = const { Cell::new(0) };
    /// Addendum H1: exact cofactor wanted by the instance generator (0: h <= 8)
    static WANT_H: Cell<u64> = const { Cell::new(0) };
    /// sqrt table: SQ[a] = some b with b^2 = a, or u32::MAX for a non-residue
    static SQ: RefCell<Vec<u32>> = const { RefCell::new(Vec::new()) };
}

#[inline]
fn pp() -> u64 {
    P.with(|c| c.get())
}
#[inline]
fn kap() -> u64 {
    KAPPA.with(|c| c.get())
}
#[inline]
fn cnt_m(k: u64) {
    CM.with(|c| c.set(c.get() + k));
}
#[inline]
fn cnt_i(k: u64) {
    CI.with(|c| c.set(c.get() + k));
}
#[inline]
fn cnt_n(k: u64) {
    CN.with(|c| c.set(c.get() + k));
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Counts {
    m: u64,
    i: u64,
    n: u64,
    /// Jacobian additions/doublings (Cantor calls)
    j: u64,
}

impl Counts {
    fn now() -> Counts {
        Counts {
            m: CM.with(|c| c.get()),
            i: CI.with(|c| c.get()),
            n: CN.with(|c| c.get()),
            j: CJ.with(|c| c.get()),
        }
    }
    fn since(self) -> Counts {
        let t = Counts::now();
        Counts {
            m: t.m - self.m,
            i: t.i - self.i,
            n: t.n - self.n,
            j: t.j - self.j,
        }
    }
    fn add(self, o: Counts) -> Counts {
        Counts {
            m: self.m + o.m,
            i: self.i + o.i,
            n: self.n + o.n,
            j: self.j + o.j,
        }
    }
    fn minus(self, o: Counts) -> Counts {
        Counts {
            m: self.m - o.m,
            i: self.i - o.i,
            n: self.n - o.n,
            j: self.j - o.j,
        }
    }
}

fn set_field(p: u64, kappa: u64) {
    P.with(|c| c.set(p));
    KAPPA.with(|c| c.set(kappa));
    let mut t = vec![u32::MAX; p as usize];
    for b in 0..p {
        let a = (b * b % p) as usize;
        if t[a] == u32::MAX {
            t[a] = b as u32;
        }
    }
    SQ.with(|s| *s.borrow_mut() = t);
}

/// sqrt in F_p by table (one native lookup); None for a non-residue.
fn sqrt_p(a: u64) -> Option<u64> {
    cnt_n(1);
    let v = SQ.with(|s| s.borrow()[a as usize]);
    if v == u32::MAX {
        None
    } else {
        Some(v as u64)
    }
}

fn xorshift(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *s = x;
    x
}

/// splitmix64: independent per-job seeds (successive xorshift outputs would
/// give streams that are shifts of one another).
fn splitmix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

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
    for q in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % q == 0 {
            return n == q;
        }
    }
    let (mut d, mut s) = (n - 1, 0);
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'w: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = (x as u128 * x as u128 % n as u128) as u64;
            if x == n - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}

#[inline]
fn mm(a: u64, b: u64, m: u64) -> u64 {
    (a as u128 * b as u128 % m as u128) as u64
}

fn inv_mod(a: u64, m: u64) -> u64 {
    let (mut t, mut nt) = (0i128, 1i128);
    let (mut r, mut nr) = (m as i128, (a % m) as i128);
    while nr != 0 {
        let q = r / nr;
        (t, nt) = (nt, t - q * nt);
        (r, nr) = (nr, r - q * nr);
    }
    assert_eq!(r, 1, "not invertible");
    t.rem_euclid(m as i128) as u64
}

// ---------------------------------------------------------------------------
// Fields.
// ---------------------------------------------------------------------------

trait Fld: Copy + PartialEq + Debug {
    fn zero() -> Self;
    fn one() -> Self;
    fn from_u64(v: u64) -> Self;
    fn add(self, o: Self) -> Self;
    fn sub(self, o: Self) -> Self;
    fn neg(self) -> Self;
    fn mul(self, o: Self) -> Self;
    fn sqr(self) -> Self;
    fn inv(self) -> Self;
    fn is_zero(self) -> bool;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
struct Fp(u64);

impl Fld for Fp {
    fn zero() -> Self {
        Fp(0)
    }
    fn one() -> Self {
        Fp(1)
    }
    fn from_u64(v: u64) -> Self {
        Fp(v % pp())
    }
    #[inline]
    fn add(self, o: Self) -> Self {
        let p = pp();
        let s = self.0 + o.0;
        Fp(if s >= p { s - p } else { s })
    }
    #[inline]
    fn sub(self, o: Self) -> Self {
        let p = pp();
        Fp(if self.0 >= o.0 {
            self.0 - o.0
        } else {
            self.0 + p - o.0
        })
    }
    fn neg(self) -> Self {
        Fp(if self.0 == 0 { 0 } else { pp() - self.0 })
    }
    #[inline]
    fn mul(self, o: Self) -> Self {
        cnt_m(1);
        Fp(self.0 * o.0 % pp())
    }
    fn sqr(self) -> Self {
        self.mul(self)
    }
    fn inv(self) -> Self {
        cnt_i(1);
        Fp(inv_mod(self.0, pp()))
    }
    fn is_zero(self) -> bool {
        self.0 == 0
    }
}

/// a0 + a1 s with s^2 = κ.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Fp2(u64, u64);

impl Fp2 {
    fn conj(self) -> Self {
        Fp2(self.0, if self.1 == 0 { 0 } else { pp() - self.1 })
    }
    fn s() -> Self {
        Fp2(0, 1)
    }
    fn from_fp(a: Fp) -> Self {
        Fp2(a.0, 0)
    }
    fn mul_fp(self, a: Fp) -> Self {
        cnt_m(2);
        let p = pp();
        Fp2(self.0 * a.0 % p, self.1 * a.0 % p)
    }
    /// canonical order key for the negation map
    fn key(self) -> (u64, u64) {
        (self.1, self.0)
    }
    /// square root in F_{p^2} by the norm method (tables for F_p roots).
    fn sqrt(self) -> Option<Fp2> {
        let p = pp();
        let k = kap();
        if self.1 == 0 {
            if let Some(r) = sqrt_p(self.0) {
                return Some(Fp2(r, 0));
            }
            // a0 = κ c^2  =>  sqrt = c s
            cnt_i(1);
            let c = sqrt_p(self.0 * inv_mod(k, p) % p)?;
            return Some(Fp2(0, c));
        }
        cnt_m(3);
        let n = (self.0 * self.0 % p + p - k * (self.1 * self.1 % p) % p) % p;
        let t = sqrt_p(n)?;
        let half = (p + 1) / 2;
        for tt in [t, (p - t) % p] {
            let c0sq = (self.0 + tt) % p * half % p;
            if let Some(c0) = sqrt_p(c0sq) {
                if c0 == 0 {
                    continue;
                }
                cnt_i(1);
                cnt_m(2);
                let c1 = self.1 * inv_mod(2 * c0 % p, p) % p;
                let r = Fp2(c0, c1);
                debug_assert_eq!(r.sqr_raw(), self);
                return Some(r);
            }
        }
        None
    }
    fn sqr_raw(self) -> Fp2 {
        let p = pp();
        let k = kap();
        Fp2(
            (self.0 * self.0 % p + k * (self.1 * self.1 % p)) % p,
            2 * self.0 * self.1 % p,
        )
    }
}

impl Fld for Fp2 {
    fn zero() -> Self {
        Fp2(0, 0)
    }
    fn one() -> Self {
        Fp2(1, 0)
    }
    fn from_u64(v: u64) -> Self {
        Fp2(v % pp(), 0)
    }
    #[inline]
    fn add(self, o: Self) -> Self {
        let p = pp();
        Fp2((self.0 + o.0) % p, (self.1 + o.1) % p)
    }
    #[inline]
    fn sub(self, o: Self) -> Self {
        let p = pp();
        Fp2((self.0 + p - o.0) % p, (self.1 + p - o.1) % p)
    }
    fn neg(self) -> Self {
        Fp2(0, 0).sub(self)
    }
    #[inline]
    fn mul(self, o: Self) -> Self {
        cnt_m(3);
        let p = pp();
        let k = kap();
        Fp2(
            (self.0 * o.0 % p + k * (self.1 * o.1 % p)) % p,
            (self.0 * o.1 + self.1 * o.0) % p,
        )
    }
    fn sqr(self) -> Self {
        cnt_m(2);
        self.sqr_raw()
    }
    fn inv(self) -> Self {
        cnt_i(1);
        cnt_m(4);
        let p = pp();
        let k = kap();
        let n = (self.0 * self.0 % p + p - k * (self.1 * self.1 % p) % p) % p;
        let ni = inv_mod(n, p);
        Fp2(self.0 * ni % p, (p - self.1) % p * ni % p)
    }
    fn is_zero(self) -> bool {
        self.0 == 0 && self.1 == 0
    }
}

// ---------------------------------------------------------------------------
// Polynomials (lowest degree first) over a field.
// ---------------------------------------------------------------------------

fn trim<F: Fld>(mut a: Vec<F>) -> Vec<F> {
    while a.last().is_some_and(|c| c.is_zero()) {
        a.pop();
    }
    a
}

fn deg<F: Fld>(a: &[F]) -> isize {
    a.len() as isize - 1
}

fn padd<F: Fld>(a: &[F], b: &[F]) -> Vec<F> {
    let n = a.len().max(b.len());
    let mut r = Vec::with_capacity(n);
    for i in 0..n {
        let x = a.get(i).copied().unwrap_or(F::zero());
        let y = b.get(i).copied().unwrap_or(F::zero());
        r.push(x.add(y));
    }
    trim(r)
}

fn psub<F: Fld>(a: &[F], b: &[F]) -> Vec<F> {
    let n = a.len().max(b.len());
    let mut r = Vec::with_capacity(n);
    for i in 0..n {
        let x = a.get(i).copied().unwrap_or(F::zero());
        let y = b.get(i).copied().unwrap_or(F::zero());
        r.push(x.sub(y));
    }
    trim(r)
}

fn pneg<F: Fld>(a: &[F]) -> Vec<F> {
    a.iter().map(|c| c.neg()).collect()
}

fn pmul<F: Fld>(a: &[F], b: &[F]) -> Vec<F> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut r = vec![F::zero(); a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        if x.is_zero() {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            if y.is_zero() {
                continue;
            }
            r[i + j] = r[i + j].add(x.mul(y));
        }
    }
    trim(r)
}

fn pscale<F: Fld>(a: &[F], c: F) -> Vec<F> {
    trim(a.iter().map(|&x| x.mul(c)).collect())
}

/// (q, r) with a = q b + r.
fn pdivrem<F: Fld>(a: &[F], b: &[F]) -> (Vec<F>, Vec<F>) {
    let db = deg(b);
    assert!(db >= 0, "division by zero polynomial");
    let mut r = a.to_vec();
    if deg(&r) < db {
        return (Vec::new(), trim(r));
    }
    let lc = *b.last().unwrap();
    let li = if lc == F::one() { F::one() } else { lc.inv() };
    let mut q = vec![F::zero(); (deg(&r) - db + 1) as usize];
    while deg(&r) >= db {
        let d = (deg(&r) - db) as usize;
        let mut c = *r.last().unwrap();
        if li != F::one() {
            c = c.mul(li);
        }
        q[d] = c;
        for (j, &y) in b.iter().enumerate() {
            if !y.is_zero() {
                r[d + j] = r[d + j].sub(c.mul(y));
            }
        }
        r.pop();
        r = trim(r);
    }
    (trim(q), r)
}

fn prem<F: Fld>(a: &[F], b: &[F]) -> Vec<F> {
    pdivrem(a, b).1
}

fn pmonic<F: Fld>(a: &[F]) -> Vec<F> {
    let lc = *a.last().unwrap();
    if lc == F::one() {
        a.to_vec()
    } else {
        pscale(a, lc.inv())
    }
}

/// (d, s, t) with s a + t b = d, d monic (or zero if a = b = 0).
fn pxgcd<F: Fld>(a: &[F], b: &[F]) -> (Vec<F>, Vec<F>, Vec<F>) {
    let (mut r0, mut r1) = (a.to_vec(), b.to_vec());
    let (mut s0, mut s1) = (vec![F::one()], Vec::new());
    let (mut t0, mut t1) = (Vec::new(), vec![F::one()]);
    while !r1.is_empty() {
        let (q, r) = pdivrem(&r0, &r1);
        let s2 = psub(&s0, &pmul(&q, &s1));
        let t2 = psub(&t0, &pmul(&q, &t1));
        r0 = std::mem::replace(&mut r1, r);
        s0 = std::mem::replace(&mut s1, s2);
        t0 = std::mem::replace(&mut t1, t2);
    }
    if r0.is_empty() {
        return (r0, s0, t0);
    }
    let lc = *r0.last().unwrap();
    if lc != F::one() {
        let li = lc.inv();
        (pscale(&r0, li), pscale(&s0, li), pscale(&t0, li))
    } else {
        (r0, s0, t0)
    }
}

fn peval<F: Fld>(a: &[F], x: F) -> F {
    let mut r = F::zero();
    for &c in a.iter().rev() {
        r = r.mul(x).add(c);
    }
    r
}

fn pderiv<F: Fld>(a: &[F]) -> Vec<F> {
    trim(
        a.iter()
            .enumerate()
            .skip(1)
            .map(|(i, &c)| c.mul(F::from_u64(i as u64)))
            .collect(),
    )
}

// ---------------------------------------------------------------------------
// Genus-2 Jacobian (imaginary model y^2 = F(x), F monic quintic), Cantor.
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
struct Div<F: Fld> {
    u: Vec<F>,
    v: Vec<F>,
}

impl<F: Fld> Div<F> {
    fn zero() -> Self {
        Div {
            u: vec![F::one()],
            v: Vec::new(),
        }
    }
    fn is_zero(&self) -> bool {
        self.u.len() == 1
    }
    fn neg(&self) -> Self {
        Div {
            u: self.u.clone(),
            v: pneg(&self.v),
        }
    }
}

fn jadd<F: Fld>(f: &[F], a: &Div<F>, b: &Div<F>) -> Div<F> {
    CJ.with(|c| c.set(c.get() + 1));
    if a.is_zero() {
        return b.clone();
    }
    if b.is_zero() {
        return a.clone();
    }
    let (d1, e1, e2) = pxgcd(&a.u, &b.u);
    let (d, s1, s2, s3) = if d1.len() == 1 {
        (vec![F::one()], e1, e2, Vec::new())
    } else {
        let vs = padd(&a.v, &b.v);
        let (d, c1, c2) = pxgcd(&d1, &vs);
        (d, pmul(&c1, &e1), pmul(&c1, &e2), c2)
    };
    let mut u = pmul(&a.u, &b.u);
    let mut num = padd(&pmul(&pmul(&s1, &a.u), &b.v), &pmul(&pmul(&s2, &b.u), &a.v));
    if !s3.is_empty() {
        num = padd(&num, &pmul(&s3, &padd(&pmul(&a.v, &b.v), f)));
    }
    if d.len() > 1 {
        u = pdivrem(&u, &pmul(&d, &d)).0;
        num = pdivrem(&num, &d).0;
    }
    let mut v = prem(&num, &u);
    // reduction
    while deg(&u) > 2 {
        let u2 = pdivrem(&psub(f, &pmul(&v, &v)), &u).0;
        u = pmonic(&u2);
        v = prem(&pneg(&v), &u);
    }
    Div { u, v }
}

/// Reduced formulas for the generic genus-2 addition and doubling (both
/// divisors of weight 2, coprime `u`'s, or doubling with `gcd(u, 2v) = 1`,
/// and a leading `s1 != 0`); every other case falls back to Cantor. Verified
/// against `jadd` in the tests. `f` is the monic quintic.
fn jadd_fast<F: Fld>(f: &[F], a: &Div<F>, b: &Div<F>) -> Div<F> {
    if a.u.len() != 3 || b.u.len() != 3 {
        return jadd(f, a, b);
    }
    let g = |v: &Vec<F>, i: usize| v.get(i).copied().unwrap_or(F::zero());
    let (u11, u10, v11, v10) = (a.u[1], a.u[0], g(&a.v, 1), g(&a.v, 0));
    let dbl = a == b;
    // modulus m = x^2 + m1 x + m0 for the inverse, and the numerator w
    let (m1, m0, al, be, w1, w0, big3, big2);
    if dbl {
        // k = (f - v^2)/u mod u, then s = k / (2v) mod u
        let k2 = f[4].sub(u11);
        let k1 = f[3].sub(u10).sub(u11.mul(k2));
        let k0 = f[2].sub(v11.sqr()).sub(u10.mul(k2)).sub(u11.mul(k1));
        let u11s = u11.sqr();
        w1 = u11s.sub(u10).sub(k2.mul(u11)).add(k1);
        w0 = u11.mul(u10).sub(k2.mul(u10)).add(k0);
        m1 = u11;
        m0 = u10;
        al = v11.add(v11);
        be = v10.add(v10);
        big3 = u11.add(u11);
        big2 = u11s.add(u10).add(u10);
    } else {
        let (u21, u20, v21, v20) = (b.u[1], b.u[0], g(&b.v, 1), g(&b.v, 0));
        m1 = u21;
        m0 = u20;
        al = u11.sub(u21);
        be = u10.sub(u20);
        w1 = v21.sub(v11);
        w0 = v20.sub(v10);
        big3 = u11.add(u21);
        big2 = u10.add(u20).add(u11.mul(u21));
    }
    // r = Res(al x + be, m); inverse numerator (ga x + de)
    let alm1 = al.mul(m1);
    let r = be.sqr().sub(be.mul(alm1)).add(al.sqr().mul(m0));
    if r.is_zero() {
        return jadd(f, a, b);
    }
    let ga = al.neg();
    let de = be.sub(alm1);
    let w1ga = w1.mul(ga);
    let st1 = w1.mul(de).add(w0.mul(ga)).sub(w1ga.mul(m1));
    let st0 = w0.mul(de).sub(w1ga.mul(m0));
    if st1.is_zero() {
        return jadd(f, a, b);
    }
    CJ.with(|c| c.set(c.get() + 1));
    let st1s = st1.sqr();
    let iall = r.mul(st1s).inv();
    let rinv = st1s.mul(iall);
    let s1 = st1.mul(rinv);
    let s0 = st0.mul(rinv);
    let r2 = r.sqr();
    let q2inv = r2.mul(r).mul(iall).neg();
    // v = v1 + u1 s = c3 x^3 + c2 x^2 + c1 x + c0
    let c3 = s1;
    let c2 = s0.add(u11.mul(s1));
    let c1 = v11.add(u11.mul(s0)).add(u10.mul(s1));
    let c0 = v10.add(u10.mul(s0));
    // q = (f - v^2) / (u1 u2), top three coefficients
    let q2 = c3.sqr().neg();
    let n5 = F::one().sub(c3.mul(c2).add(c3.mul(c2)));
    let n4 = f[4].sub(c2.sqr()).sub(c3.mul(c1).add(c3.mul(c1)));
    let q1 = n5.sub(q2.mul(big3));
    let q0 = n4.sub(q2.mul(big2)).sub(q1.mul(big3));
    let up1 = q1.mul(q2inv);
    let up0 = q0.mul(q2inv);
    // v' = -(v mod u')
    let up1s = up1.sqr();
    let x1 = c3.mul(up1s.sub(up0)).sub(c2.mul(up1)).add(c1);
    let x0 = c3.mul(up1).mul(up0).sub(c2.mul(up0)).add(c0);
    Div {
        u: vec![up0, up1, F::one()],
        v: trim(vec![x0.neg(), x1.neg()]),
    }
}

/// Real-model (monic sextic `f`) version of `jadd_fast`: same composition; the
/// reduction keeps the `f6`, `f5` terms. Falls back to `radd` in every other case.
fn radd_fast<F: Fld>(f: &[F], a: &Div<F>, b: &Div<F>) -> Option<Div<F>> {
    if a.u.len() != 3 || b.u.len() != 3 {
        return radd(f, a, b);
    }
    let g = |v: &Vec<F>, i: usize| v.get(i).copied().unwrap_or(F::zero());
    let (u11, u10, v11, v10) = (a.u[1], a.u[0], g(&a.v, 1), g(&a.v, 0));
    let dbl = a == b;
    // modulus m = x^2 + m1 x + m0 for the inverse, and the numerator w
    let (m1, m0, al, be, w1, w0, big3, big2);
    if dbl {
        // k = (f - v^2)/u mod u, then s = k / (2v) mod u
        // k = (f − v²)/u for the monic sextic, then k mod u
        let k3 = f[5].sub(u11);
        let k2 = f[4].sub(u10).sub(u11.mul(k3));
        let k1 = f[3].sub(u10.mul(k3)).sub(u11.mul(k2));
        let k0 = f[2].sub(v11.sqr()).sub(u10.mul(k2)).sub(u11.mul(k1));
        // reduce x⁴ + k3 x³ + k2 x² + k1 x + k0 mod x² + u11 x + u10
        let c3r = k3.sub(u11);
        let c2r = k2.sub(u10).sub(c3r.mul(u11));
        let c1r = k1.sub(c3r.mul(u10)).sub(c2r.mul(u11));
        let c0r = k0.sub(c2r.mul(u10));
        let u11s = u11.sqr();
        w1 = c1r;
        w0 = c0r;
        m1 = u11;
        m0 = u10;
        al = v11.add(v11);
        be = v10.add(v10);
        big3 = u11.add(u11);
        big2 = u11s.add(u10).add(u10);
    } else {
        let (u21, u20, v21, v20) = (b.u[1], b.u[0], g(&b.v, 1), g(&b.v, 0));
        m1 = u21;
        m0 = u20;
        al = u11.sub(u21);
        be = u10.sub(u20);
        w1 = v21.sub(v11);
        w0 = v20.sub(v10);
        big3 = u11.add(u21);
        big2 = u10.add(u20).add(u11.mul(u21));
    }
    // r = Res(al x + be, m); inverse numerator (ga x + de)
    let alm1 = al.mul(m1);
    let r = be.sqr().sub(be.mul(alm1)).add(al.sqr().mul(m0));
    if r.is_zero() {
        return radd(f, a, b);
    }
    let ga = al.neg();
    let de = be.sub(alm1);
    let w1ga = w1.mul(ga);
    let st1 = w1.mul(de).add(w0.mul(ga)).sub(w1ga.mul(m1));
    let st0 = w0.mul(de).sub(w1ga.mul(m0));
    if st1.is_zero() {
        return radd(f, a, b);
    }
    // q2 = f6 − s1² = (r² − s̃1²)/r²: one inversion for 1/r and 1/q2
    let st1s = st1.sqr();
    let r2 = r.sqr();
    let dq = r2.sub(st1s);
    if dq.is_zero() {
        return radd(f, a, b);
    }
    CJ.with(|c| c.set(c.get() + 1));
    let iall = r.mul(dq).inv();
    let rinv = dq.mul(iall);
    let s1 = st1.mul(rinv);
    let s0 = st0.mul(rinv);
    let q2inv = r2.mul(r).mul(iall);
    // v = v1 + u1 s = c3 x^3 + c2 x^2 + c1 x + c0
    let c3 = s1;
    let c2 = s0.add(u11.mul(s1));
    let c1 = v11.add(u11.mul(s0)).add(u10.mul(s1));
    let c0 = v10.add(u10.mul(s0));
    // q = (f - v^2) / (u1 u2), top three coefficients
    let q2 = F::one().sub(c3.sqr());
    let n5 = f[5].sub(c3.mul(c2).add(c3.mul(c2)));
    let n4 = f[4].sub(c2.sqr()).sub(c3.mul(c1).add(c3.mul(c1)));
    let q1 = n5.sub(q2.mul(big3));
    let q0 = n4.sub(q2.mul(big2)).sub(q1.mul(big3));
    let up1 = q1.mul(q2inv);
    let up0 = q0.mul(q2inv);
    // v' = -(v mod u')
    let up1s = up1.sqr();
    let x1 = c3.mul(up1s.sub(up0)).sub(c2.mul(up1)).add(c1);
    let x0 = c3.mul(up1).mul(up0).sub(c2.mul(up0)).add(c0);
    Some(Div {
        u: vec![up0, up1, F::one()],
        v: trim(vec![x0.neg(), x1.neg()]),
    })
}

fn jmul_fast<F: Fld>(f: &[F], a: &Div<F>, mut k: u64) -> Div<F> {
    let mut r = Div::zero();
    let mut b = a.clone();
    while k > 0 {
        if k & 1 == 1 {
            r = jadd_fast(f, &r, &b);
        }
        k >>= 1;
        if k > 0 {
            b = jadd_fast(f, &b, &b);
        }
    }
    r
}

fn jmul<F: Fld>(f: &[F], a: &Div<F>, mut k: u64) -> Div<F> {
    let mut r = Div::zero();
    let mut b = a.clone();
    while k > 0 {
        if k & 1 == 1 {
            r = jadd(f, &r, &b);
        }
        k >>= 1;
        if k > 0 {
            b = jadd(f, &b, &b);
        }
    }
    r
}

fn jvalid<F: Fld>(f: &[F], d: &Div<F>) -> bool {
    prem(&psub(&pmul(&d.v, &d.v), f), &d.u).is_empty()
}

// ---------------------------------------------------------------------------
// Elliptic curve y^2 = x^3 + A x + B over F_{p^2} (affine).
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum EP {
    O,
    A(Fp2, Fp2),
}

#[derive(Clone, Copy, Debug)]
struct Ell {
    a: Fp2,
    b: Fp2,
}

impl Ell {
    fn rhs(&self, x: Fp2) -> Fp2 {
        x.sqr().mul(x).add(self.a.mul(x)).add(self.b)
    }
    fn add(&self, p: EP, q: EP) -> EP {
        match (p, q) {
            (EP::O, r) | (r, EP::O) => r,
            (EP::A(x1, y1), EP::A(x2, y2)) => {
                let lam = if x1 == x2 {
                    if y1.add(y2).is_zero() {
                        return EP::O;
                    }
                    let num = x1.sqr().mul(Fp2::from_u64(3)).add(self.a);
                    num.mul(y1.add(y1).inv())
                } else {
                    y2.sub(y1).mul(x2.sub(x1).inv())
                };
                let x3 = lam.sqr().sub(x1).sub(x2);
                let y3 = lam.mul(x1.sub(x3)).sub(y1);
                EP::A(x3, y3)
            }
        }
    }
    fn neg(&self, p: EP) -> EP {
        match p {
            EP::O => EP::O,
            EP::A(x, y) => EP::A(x, y.neg()),
        }
    }
    fn mul(&self, p: EP, mut k: u64) -> EP {
        let mut r = EP::O;
        let mut b = p;
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(r, b);
            }
            k >>= 1;
            if k > 0 {
                b = self.add(b, b);
            }
        }
        r
    }
    fn random_point(&self, rng: &mut u64) -> EP {
        let p = pp();
        loop {
            let x = Fp2(xorshift(rng) % p, xorshift(rng) % p);
            if let Some(y) = self.rhs(x).sqrt() {
                let y = if xorshift(rng) & 1 == 1 { y.neg() } else { y };
                return EP::A(x, y);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Instance: C, E, the transfer, the order.
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct Instance {
    p: u64,
    kappa: u64,
    f: Vec<Fp>, // sextic of C
    r: Fp,
    big_l: Fp,
    fq: Vec<Fp>, // monic quintic model of C
    // E long form Y^2 = X^3 + c2 X^2 + c4 X + c6; short form x = X + c2/3
    u0: Fp2,
    h0: Fp2,
    c2: Fp2,
    e: Ell,
    n: u64,
    h: u64,
    ell: u64,
}

/// Taylor shift: coefficients of f(x + r).
fn taylor<F: Fld>(f: &[F], r: F) -> Vec<F> {
    let mut a = f.to_vec();
    let n = a.len();
    for i in 0..n {
        for j in (i..n - 1).rev() {
            let t = a[j + 1].mul(r);
            a[j] = a[j].add(t);
        }
    }
    a
}

fn lift(a: &[Fp]) -> Vec<Fp2> {
    a.iter().map(|&c| Fp2::from_fp(c)).collect()
}

fn small_factors(mut n: u64) -> Vec<u64> {
    let mut v = Vec::new();
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            v.push(d);
            while n % d == 0 {
                n /= d;
            }
        }
        d += 1;
    }
    if n > 1 {
        v.push(n);
    }
    v
}

/// Order of E(F_{p^2}) by BSGS on random points (Hasse interval width 4p).
fn ec_order(e: &Ell, p: u64, rng: &mut u64) -> Option<u64> {
    let lo = p * p + 1 - 2 * p;
    let hi = p * p + 1 + 2 * p;
    let mut lcm = 1u64;
    for _ in 0..8 {
        let pt = e.random_point(rng);
        // BSGS for m in [lo, hi] with m pt = O
        let w = hi - lo + 1;
        let bs = ((w as f64).sqrt() as u64).max(1) + 1;
        let mut baby: HashMap<EP, u64> = HashMap::new();
        let mut q = EP::O;
        for j in 0..bs {
            baby.entry(q).or_insert(j);
            q = e.add(q, pt);
        }
        // giant: lo pt + i bs pt ; want lo + i bs + ... find m with m pt = O:
        // (lo + i bs) pt = -j pt  =>  m = lo + i bs + j
        let step = e.neg(e.mul(pt, bs));
        let mut g = e.neg(e.mul(pt, lo));
        // g = -(lo + i bs) pt ; look for j pt = g
        let mut found = None;
        let mut i = 0u64;
        while lo + i * bs <= hi + bs {
            if let Some(&j) = baby.get(&g) {
                let m = lo + i * bs + j;
                if m >= lo && m <= hi {
                    found = Some(m);
                    break;
                }
            }
            g = e.add(g, step);
            i += 1;
        }
        let m = found?;
        // order of pt divides m: strip primes
        let mut ord = m;
        for q in small_factors(m) {
            while ord % q == 0 && e.mul(pt, ord / q) == EP::O {
                ord /= q;
            }
        }
        lcm = lcm / gcd(lcm, ord) * ord;
        let cands: Vec<u64> = (lo..=hi).filter(|m| m % lcm == 0).collect();
        if cands.len() == 1 {
            return Some(cands[0]);
        }
    }
    None
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl Instance {
    /// Build an instance with #E(F_{p^2}) = h·ℓ, h ≤ 8, ℓ prime, p near 2^half.
    fn new(bits: u32, rng: &mut u64) -> Instance {
        loop {
            let base = 1u64 << (bits / 2);
            let p = loop {
                let c = base + xorshift(rng) % (base / 8).max(8);
                if c > 5 && is_prime(c) {
                    break c;
                }
            };
            let kappa = (2..p)
                .find(|&k| powmod(k, (p - 1) / 2, p) == p - 1)
                .unwrap();
            set_field(p, kappa);
            for _ in 0..400 {
                if let Some(inst) = Instance::try_new(p, kappa, rng) {
                    return inst;
                }
            }
        }
    }

    /// An instance with exact cofactor `h` (`#E = h·ℓ`, `ℓ` prime).
    fn with_h(bits: u32, h: u64, rng: &mut u64) -> Instance {
        WANT_H.with(|c| c.set(h));
        let inst = Instance::new(bits, rng);
        WANT_H.with(|c| c.set(0));
        inst
    }

    fn try_new(p: u64, kappa: u64, rng: &mut u64) -> Option<Instance> {
        let k = Fp(kappa);
        let r = Fp(1 + xorshift(rng) % (p - 1));
        let (g2, g3, g4) = (
            Fp(xorshift(rng) % p),
            Fp(xorshift(rng) % p),
            Fp(1 + xorshift(rng) % (p - 1)),
        );
        let kr = k.mul(r.inv());
        let g = vec![k.sqr().mul(g4), k.mul(g3), g2, g3, g4];
        let lin = vec![r.mul(kr), r.add(kr).neg(), Fp(1)];
        let f = pmul(&lin, &g);
        if f.len() != 7 {
            return None;
        }
        let (gd, _, _) = pxgcd(&f, &pderiv(&f));
        if gd.len() != 1 {
            return None;
        }
        // h(u) = f6 u^3 + f5 u^2 + (f4 - 3κ f6) u + (f3 - 2κ f5)
        let hh = [
            f[3].sub(k.mul(f[5]).mul(Fp(2))),
            f[4].sub(k.mul(f[6]).mul(Fp(3))),
            f[5],
            f[6],
        ];
        let s = Fp2::s();
        let u0 = s.add(s).neg();
        let hl = lift(&hh);
        let h0 = peval(&hl, u0);
        let h1 = peval(&pderiv(&hl), u0);
        let h2 = Fp2::from_fp(f[6])
            .mul(u0)
            .mul(Fp2::from_u64(3))
            .add(Fp2::from_fp(f[5]));
        if h0.is_zero() {
            return None;
        }
        let c2 = h1;
        let c4 = h0.mul(h2);
        let c6 = h0.sqr().mul(Fp2::from_fp(f[6]));
        let i3 = Fp2::from_u64(3).inv();
        let a = c4.sub(c2.sqr().mul(i3));
        let b = c2
            .sqr()
            .mul(c2)
            .mul(Fp2::from_u64(2))
            .mul(Fp2::from_u64(27).inv())
            .sub(c2.mul(c4).mul(i3))
            .add(c6);
        let disc = a
            .sqr()
            .mul(a)
            .mul(Fp2::from_u64(4))
            .add(b.sqr().mul(Fp2::from_u64(27)));
        if disc.is_zero() {
            return None;
        }
        let e = Ell { a, b };
        let n = ec_order(&e, p, rng)?;
        let mut h = 0;
        let want = WANT_H.with(|c| c.get());
        if want > 0 {
            if n % want == 0 && n / want > want && is_prime(n / want) {
                h = want;
            }
        } else {
            for hh in 1..=8u64 {
                if n % hh == 0 && is_prime(n / hh) && n / hh > 8 {
                    h = hh;
                    break;
                }
            }
        }
        if h == 0 {
            return None;
        }
        // quintic model: x = r + L/Ξ, y = w' L / Ξ^3... (see module docs)
        let t = taylor(&f, r); // t[k] = f^(k)(r)/k!
        let big_l = t[1];
        if big_l.is_zero() {
            return None;
        }
        let l2 = big_l.sqr();
        let fq = vec![
            t[6].mul(l2.sqr()),
            t[5].mul(l2.mul(big_l)),
            t[4].mul(l2),
            t[3].mul(big_l),
            t[2],
            Fp(1),
        ];
        Some(Instance {
            p,
            kappa,
            f,
            r,
            big_l,
            fq,
            u0,
            h0,
            c2,
            e,
            n,
            h,
            ell: n / h,
        })
    }

    /// #C(F_p) on the sextic model.
    fn count_c(&self) -> u64 {
        let p = self.p;
        let mut c = 0u64;
        for x in 0..p {
            let v = peval(&self.f, Fp(x)).0;
            c += if v == 0 {
                1
            } else if sqrt_p(v).is_some() {
                2
            } else {
                0
            };
        }
        c + if sqrt_p(self.f[6].0).is_some() { 2 } else { 0 }
    }

    /// φ^* of a point of E (short model) as a sextic-model Mumford pair over F_{p^2}.
    fn pullback(&self, pt: EP) -> Option<(Vec<Fp2>, Vec<Fp2>)> {
        let EP::A(x, y) = pt else { return None };
        let xl = x.sub(self.c2.mul(Fp2::from_u64(3).inv()));
        if xl.is_zero() {
            return None;
        }
        let xi = xl.inv();
        let u = self.u0.add(self.h0.mul(xi));
        let v = y.mul(self.h0).mul(xi.sqr());
        let kk = Fp2::from_u64(self.kappa);
        let big_u = vec![kk, u.neg(), Fp2::one()];
        let w = vec![kk.neg(), u]; // x^2 mod U
        let (d, sinv, _) = pxgcd(&[Fp2::s(), Fp2::one()], &big_u);
        if d.len() != 1 {
            return None;
        }
        let big_v = pscale(&prem(&pmul(&w, &sinv), &big_u), v);
        Some((big_u, big_v))
    }

    /// Sextic-model Mumford pair (degree 2) to the quintic model.
    fn to_quintic(&self, u: &[Fp2], v: &[Fp2]) -> Option<Div<Fp2>> {
        let r = Fp2::from_fp(self.r);
        let l = Fp2::from_fp(self.big_l);
        let u1 = u.get(1).copied().unwrap_or(Fp2::zero());
        let d = peval(u, r);
        if d.is_zero() {
            return None;
        }
        let di = d.inv();
        let uq = vec![l.sqr().mul(di), l.mul(u1.add(r).add(r)).mul(di), Fp2::one()];
        let v0 = v.first().copied().unwrap_or(Fp2::zero());
        let v1 = v.get(1).copied().unwrap_or(Fp2::zero());
        let li = l.inv();
        let num = vec![
            Fp2::zero(),
            Fp2::zero(),
            v1.mul(l).mul(li),
            v1.mul(r).add(v0).mul(li),
        ];
        let vq = prem(&trim(num), &uq);
        Some(Div { u: uq, v: vq })
    }

    /// τ(P) = φ^*P + (φ^*P)^σ in Jac(C)(F_p), quintic model.
    fn transfer(&self, pt: EP) -> Option<Div<Fp>> {
        let (u, v) = self.pullback(pt)?;
        let d = self.to_quintic(&u, &v)?;
        let dc = Div {
            u: d.u.iter().map(|c| c.conj()).collect(),
            v: d.v.iter().map(|c| c.conj()).collect(),
        };
        let fq2 = lift(&self.fq);
        let t = jadd(&fq2, &d, &dc);
        let mut out = Div {
            u: Vec::new(),
            v: Vec::new(),
        };
        for c in &t.u {
            if c.1 != 0 {
                return None;
            }
            out.u.push(Fp(c.0));
        }
        for c in &t.v {
            if c.1 != 0 {
                return None;
            }
            out.v.push(Fp(c.0));
        }
        Some(out)
    }
}

// ---------------------------------------------------------------------------
// Relation linear algebra mod ℓ (shared by Arms J and D).
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Default)]
struct Row {
    bcols: Vec<(u32, u64)>, // sorted by column
    lp: Vec<(u64, u64)>,    // sorted by key
    a: u64,
    b: u64,
}

fn merge<K: Ord + Copy>(x: &[(K, u64)], y: &[(K, u64)], c: u64, ell: u64) -> Vec<(K, u64)> {
    // x - c y mod ell
    let mut out = Vec::with_capacity(x.len() + y.len());
    let (mut i, mut j) = (0, 0);
    while i < x.len() || j < y.len() {
        if j >= y.len() || (i < x.len() && x[i].0 < y[j].0) {
            out.push(x[i]);
            i += 1;
        } else if i >= x.len() || y[j].0 < x[i].0 {
            cnt_n(1);
            out.push((y[j].0, (ell - mm(c, y[j].1, ell)) % ell));
            j += 1;
        } else {
            cnt_n(1);
            let v = (x[i].1 + ell - mm(c, y[j].1, ell)) % ell;
            if v != 0 {
                out.push((x[i].0, v));
            }
            i += 1;
            j += 1;
        }
    }
    out
}

struct Lin {
    ell: u64,
    nb: usize,
    pivots: HashMap<u64, Row>,
    brels: Vec<Row>,
    direct: Vec<(u64, u64)>,
}

impl Lin {
    fn new(ell: u64, nb: usize) -> Lin {
        Lin {
            ell,
            nb,
            pivots: HashMap::new(),
            brels: Vec::new(),
            direct: Vec::new(),
        }
    }
    fn sub_row(&self, row: &Row, piv: &Row, c: u64) -> Row {
        let ell = self.ell;
        cnt_n(2);
        Row {
            bcols: merge(&row.bcols, &piv.bcols, c, ell),
            lp: merge(&row.lp, &piv.lp, c, ell),
            a: (row.a + ell - mm(c, piv.a, ell)) % ell,
            b: (row.b + ell - mm(c, piv.b, ell)) % ell,
        }
    }
    /// Eliminate the row's large-prime columns; returns a candidate log when
    /// the row reduces to a direct relation `a + b k = 0` with `b != 0`.
    fn push(&mut self, mut row: Row) -> Option<u64> {
        loop {
            cnt_n(row.lp.len() as u64);
            let hit = row
                .lp
                .iter()
                .find(|(k, _)| self.pivots.contains_key(k))
                .copied();
            match hit {
                Some((k, c)) => {
                    let piv = self.pivots.get(&k).unwrap();
                    row = self.sub_row(&row, piv, c);
                }
                None => break,
            }
        }
        if let Some(&(k, c)) = row.lp.first() {
            let ci = inv_mod(c, self.ell);
            cnt_n(1 + row.lp.len() as u64 + row.bcols.len() as u64);
            let ell = self.ell;
            let norm = |v: &[(u64, u64)]| v.iter().map(|&(x, y)| (x, mm(y, ci, ell))).collect();
            let nr = Row {
                bcols: row
                    .bcols
                    .iter()
                    .map(|&(x, y)| (x, mm(y, ci, ell)))
                    .collect(),
                lp: norm(&row.lp),
                a: mm(row.a, ci, ell),
                b: mm(row.b, ci, ell),
            };
            self.pivots.insert(k, nr);
        } else if row.bcols.is_empty() {
            self.direct.push((row.a, row.b));
            if row.b != 0 {
                let ell = self.ell;
                return Some(mm((ell - row.a) % ell, inv_mod(row.b, ell), ell));
            }
        } else {
            self.brels.push(row);
        }
        None
    }
    /// Candidate logs from dependencies of the B-relations (dense elimination
    /// on the factor-base columns, skipping zero entries).
    fn solve(&self) -> Vec<u64> {
        let ell = self.ell;
        let mut out = Vec::new();
        for &(a, b) in &self.direct {
            if b != 0 {
                out.push(mm((ell - a) % ell, inv_mod(b, ell), ell));
            }
        }
        let nb = self.nb;
        let mut piv: Vec<Option<(Vec<u64>, Vec<u32>, u64, u64)>> = vec![None; nb];
        for r in &self.brels {
            let mut v = vec![0u64; nb];
            for &(c, x) in &r.bcols {
                v[c as usize] = x;
            }
            let (mut a, mut b) = (r.a, r.b);
            let mut col = 0usize;
            let mut placed = false;
            while col < nb {
                if v[col] == 0 {
                    col += 1;
                    continue;
                }
                match &piv[col] {
                    Some((pv, nz, pa, pb)) => {
                        let c = v[col];
                        cnt_n(nz.len() as u64 + 2);
                        for &j in nz {
                            let j = j as usize;
                            v[j] = (v[j] + ell - mm(c, pv[j], ell)) % ell;
                        }
                        a = (a + ell - mm(c, *pa, ell)) % ell;
                        b = (b + ell - mm(c, *pb, ell)) % ell;
                        col += 1;
                    }
                    None => {
                        let ci = inv_mod(v[col], ell);
                        let mut nz = Vec::new();
                        for j in 0..nb {
                            if v[j] != 0 {
                                v[j] = mm(v[j], ci, ell);
                                nz.push(j as u32);
                            }
                        }
                        cnt_n(nb as u64);
                        piv[col] = Some((v.clone(), nz, mm(a, ci, ell), mm(b, ci, ell)));
                        placed = true;
                        break;
                    }
                }
            }
            if !placed && b != 0 {
                out.push(mm((ell - a) % ell, inv_mod(b, ell), ell));
            }
        }
        out
    }
}

// ---------------------------------------------------------------------------
// Arms.
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Default)]
struct ArmResult {
    arm: String,
    c: u32,
    nb: usize,
    steps: u64,
    rels: u64,
    brels: u64,
    jac_ops: u64,
    walk: Counts,
    jac: Counts,
    la: Counts,
    k: u64,
    ok: bool,
    secs: f64,
    /// Amendment F control: divisors that recurred among relation rows, and
    /// whether the solving row's divisor was such a repeat
    dup_events: u64,
    solved_by_dup: bool,
    /// Addendum H4: stored rows (J: pivots + B-relations; R: distinguished points)
    mem: u64,
}

fn canon_e(pt: EP) -> (EP, bool) {
    match pt {
        EP::O => (EP::O, false),
        EP::A(x, y) => {
            let ny = y.neg();
            if ny.key() < y.key() {
                (EP::A(x, ny), true)
            } else {
                (pt, false)
            }
        }
    }
}

fn hash_fp2(x: Fp2) -> u64 {
    cnt_n(1);
    let mut h = x.0.wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ x.1.wrapping_mul(0xc2b2_ae3d_27d4_eb4f);
    h ^= h >> 29;
    h.wrapping_mul(0x1656_67b1_9e37_79f9) >> 20
}

/// Arm R: rho on E(F_{p^2}) with the negation map, r = 64, distinguished points.
fn arm_rho(inst: &Instance, pt: EP, q: EP, rng: &mut u64) -> ArmResult {
    let t0 = Instant::now();
    let c0 = Counts::now();
    let e = &inst.e;
    let ell = inst.ell;
    let r = 64usize;
    let mut mults = Vec::new();
    for _ in 0..r {
        let (a, b) = (xorshift(rng) % ell, xorshift(rng) % ell);
        mults.push((e.add(e.mul(pt, a), e.mul(q, b)), a, b));
    }
    let lb = 64 - ell.leading_zeros();
    let dbits = (lb / 4).saturating_sub(1).max(1);
    let dmask = (1u64 << dbits) - 1;
    let maxlen = 40u64 << dbits;
    let mut table: HashMap<(u64, u64), (u64, u64)> = HashMap::new();
    let mut steps = 0u64;
    let mut res = ArmResult {
        arm: "R".into(),
        ..Default::default()
    };
    // one canonical step of the walk (negation map)
    let step = |w: EP, a: u64, b: u64, steps: &mut u64| -> (EP, u64, u64, u64) {
        let EP::A(x, _) = w else { return (w, a, b, 0) };
        let hx = hash_fp2(x);
        let (m, ma, mb) = mults[((hx >> dbits) as usize) % r];
        let (nw, fl) = canon_e(e.add(w, m));
        *steps += 1;
        let (mut na, mut nb) = ((a + ma) % ell, (b + mb) % ell);
        if fl {
            na = (ell - na) % ell;
            nb = (ell - nb) % ell;
        }
        (nw, na, nb, hx)
    };
    let double = |w: EP, a: u64, b: u64, steps: &mut u64| -> (EP, u64, u64) {
        let (dw, fl) = canon_e(e.add(w, w));
        *steps += 1;
        let (mut da, mut db) = (2 * a % ell, 2 * b % ell);
        if fl {
            da = (ell - da) % ell;
            db = (ell - db) % ell;
        }
        (dw, da, db)
    };
    'outer: loop {
        let (mut a, mut b) = (xorshift(rng) % ell, xorshift(rng) % ell);
        let (mut w, fl) = canon_e(e.add(e.mul(pt, a), e.mul(q, b)));
        if fl {
            a = (ell - a) % ell;
            b = (ell - b) % ell;
        }
        let mut len = 0u64;
        // fruitless-cycle detection (Bos–Kleinjung–Lenstra style): every 32
        // steps remember the point; meeting it again within 32 steps means a
        // cycle of length <= 32, left by doubling its smallest point
        let mut chk = EP::O;
        let mut chk_len = 0u64;
        let mut prev: (EP, u64, u64) = (EP::O, 0, 0);
        loop {
            let EP::A(x, _) = w else { continue 'outer };
            let hx = hash_fp2(x);
            if hx & dmask == 0 {
                cnt_n(1);
                let key = (x.0, x.1);
                if let Some(&(a2, b2)) = table.get(&key) {
                    if b2 != b {
                        let k = mm(
                            (a + ell - a2) % ell,
                            inv_mod((b2 + ell - b) % ell, ell),
                            ell,
                        );
                        if e.mul(pt, k) == q {
                            res.k = k;
                            res.ok = true;
                            break 'outer;
                        }
                    }
                    // own trail revisited: restart from a fresh point
                    continue 'outer;
                }
                // store and keep walking (van Oorschot–Wiener trails continue)
                table.insert(key, (a, b));
            }
            let (nw, na, nb, _) = step(w, a, b, &mut steps);
            len += 1;
            if nw == prev.0 {
                // fruitless 2-cycle: double the smaller of the two points
                let sel = if key_ep(w) < key_ep(nw) {
                    (w, a, b)
                } else {
                    (nw, na, nb)
                };
                (w, a, b) = double(sel.0, sel.1, sel.2, &mut steps);
                prev = (EP::O, 0, 0);
                continue;
            }
            prev = (w, a, b);
            (w, a, b) = (nw, na, nb);
            if w == chk && len - chk_len <= 32 {
                // walk the cycle once more to find its smallest point
                let (mut mw, mut ma, mut mb) = (w, a, b);
                let (mut cw, mut ca, mut cb) = (w, a, b);
                for _ in 0..(len - chk_len) {
                    let (nw, na, nb, _) = step(cw, ca, cb, &mut steps);
                    (cw, ca, cb) = (nw, na, nb);
                    if key_ep(cw) < key_ep(mw) {
                        (mw, ma, mb) = (cw, ca, cb);
                    }
                }
                (w, a, b) = double(mw, ma, mb, &mut steps);
                chk = EP::O;
                prev = (EP::O, 0, 0);
                continue;
            }
            if len % 32 == 0 {
                chk = w;
                chk_len = len;
            }
            if len > maxlen {
                continue 'outer;
            }
        }
    }
    res.steps = steps;
    res.mem = table.len() as u64;
    res.walk = c0.since();
    res.secs = t0.elapsed().as_secs_f64();
    res
}

fn key_ep(p: EP) -> (u64, u64) {
    match p {
        EP::O => (0, 0),
        EP::A(x, _) => (x.0, x.1),
    }
}

const IC_R: usize = 32;
/// targets per instance in `run` (Amendment G: 10; Addendum A had 3)
const TARGETS: usize = 10;
const IC_RESTART: u64 = 4096;

/// Arm J: genus-2 index calculus in Jac(C)(F_p) on τ(P), τ(Q).
fn arm_jac(inst: &Instance, pt: EP, q: EP, c: u32, rng: &mut u64) -> ArmResult {
    arm_jac_impl(inst, pt, q, c, rng, false, false, 1.0)
}

/// `track`: record repeated divisors (Amendment F); `indep`: every trial is a
/// fresh `a τP + b τQ` (no walk), so no walk can collide with itself.
fn arm_jac_impl(
    inst: &Instance,
    pt: EP,
    q: EP,
    c: u32,
    rng: &mut u64,
    track: bool,
    indep: bool,
    theta: f64,
) -> ArmResult {
    // Addendum H4: keep a relation only if every point is distinguished
    let dist_cut = (theta * (1u64 << 32) as f64) as u64;
    let distinguished = |xi: u64| (splitmix(xi ^ 0xd157) >> 32) < dist_cut;
    let mut seen: HashMap<Vec<u64>, ()> = HashMap::new();
    let t0 = Instant::now();
    let c0 = Counts::now();
    let p = inst.p;
    let ell = inst.ell;
    let f = &inst.fq;
    let mut res = ArmResult {
        arm: "J".into(),
        c,
        ..Default::default()
    };
    let mut jac = Counts::default();
    macro_rules! jop {
        ($e:expr) => {{
            let s = Counts::now();
            let v = $e;
            jac = jac.add(s.since());
            v
        }};
    }
    // transfer (counted)
    let s = Counts::now();
    let tp = inst.transfer(pt).expect("transfer P");
    let tq = inst.transfer(q).expect("transfer Q");
    jac = jac.add(s.since());
    // factor base
    let nb = ((c as f64) * (p as f64).sqrt()).ceil() as usize;
    let mut bidx = vec![u32::MAX; p as usize];
    let mut order: Vec<u64> = (0..p).collect();
    for i in (1..order.len()).rev() {
        let j = (xorshift(rng) % (i as u64 + 1)) as usize;
        order.swap(i, j);
    }
    let mut cnt = 0usize;
    for &x in &order {
        if cnt == nb {
            break;
        }
        let v = peval(f, Fp(x)).0;
        if v != 0 && sqrt_p(v).is_some() {
            bidx[x as usize] = cnt as u32;
            cnt += 1;
        }
    }
    let nb = cnt;
    res.nb = nb;
    let mut lin = Lin::new(ell, nb);
    let mut mults = Vec::new();
    for _ in 0..IC_R {
        let (a, b) = (xorshift(rng) % ell, xorshift(rng) % ell);
        let d = jop!(jadd_fast(f, &jmul_fast(f, &tp, a), &jmul_fast(f, &tq, b)));
        mults.push((d, a, b));
    }
    let half = Fp((p + 1) / 2);
    let mut steps = 0u64;
    let mut target = nb + 10;
    loop {
        let (mut a, mut b) = (xorshift(rng) % ell, xorshift(rng) % ell);
        let mut d = jop!(jadd_fast(f, &jmul_fast(f, &tp, a), &jmul_fast(f, &tq, b)));
        for _ in 0..IC_RESTART {
            // split test
            let mut pts: Vec<(u64, u64)> = Vec::new(); // (xi, w)
            if d.u.len() == 3 {
                let (u0, u1) = (d.u[0], d.u[1]);
                let disc = u1.sqr().sub(u0.mul(Fp(4)));
                if let Some(sq) = sqrt_p(disc.0) {
                    let v0 = d.v.first().copied().unwrap_or(Fp(0));
                    let v1 = d.v.get(1).copied().unwrap_or(Fp(0));
                    for sgn in [Fp(sq), Fp(sq).neg()] {
                        let xi = u1.neg().add(sgn).mul(half);
                        let w = v1.mul(xi).add(v0);
                        pts.push((xi.0, w.0));
                    }
                    if sq == 0 {
                        pts.pop();
                        pts[0].1 = u64::MAX; // marker: double point
                    }
                }
            } else if d.u.len() == 2 {
                let v0 = d.v.first().copied().unwrap_or(Fp(0));
                pts.push((d.u[0].neg().0, v0.0));
            }
            if theta < 1.0 && pts.iter().any(|&(xi, _)| !distinguished(xi)) {
                pts.clear();
            }
            if !pts.is_empty() {
                let mut row = Row {
                    a,
                    b,
                    ..Default::default()
                };
                let double = pts[0].1 == u64::MAX;
                if double {
                    let v0 = d.v.first().copied().unwrap_or(Fp(0));
                    let v1 = d.v.get(1).copied().unwrap_or(Fp(0));
                    pts[0].1 = v1.mul(Fp(pts[0].0)).add(v0).0;
                }
                let mut entries: Vec<(bool, u64, u64)> = Vec::new();
                for &(xi, w) in &pts {
                    cnt_n(1);
                    let sign = if w <= p - w || w == 0 { 1 } else { ell - 1 };
                    let coef = if double { 2 * sign % ell } else { sign };
                    let bi = bidx[xi as usize];
                    entries.push((
                        bi != u32::MAX,
                        if bi != u32::MAX { bi as u64 } else { xi },
                        coef,
                    ));
                }
                for (isb, key, coef) in entries {
                    if isb {
                        add_entry(&mut row.bcols, key as u32, coef, ell);
                    } else {
                        add_entry(&mut row.lp, key, coef, ell);
                    }
                }
                res.rels += 1;
                let mut is_dup = false;
                if track {
                    let key: Vec<u64> = d.u.iter().chain(d.v.iter()).map(|c| c.0).collect();
                    is_dup = seen.insert(key, ()).is_some();
                    res.dup_events += u64::from(is_dup);
                }
                let s = Counts::now();
                let cand = lin.push(row);
                res.la = res.la.add(s.since());
                if let Some(k) = cand.filter(|&k| inst.e.mul(pt, k) == q) {
                    res.solved_by_dup = is_dup;
                    res.k = k;
                    res.ok = true;
                    res.steps = steps;
                    res.brels = lin.brels.len() as u64;
                    res.jac = jac;
                    res.jac_ops = jac.j;
                    res.mem = (lin.pivots.len() + lin.brels.len()) as u64;
                    res.walk = c0.since().minus(res.la);
                    res.secs = t0.elapsed().as_secs_f64();
                    return res;
                }
                if lin.brels.len() >= target {
                    let s = Counts::now();
                    let ks = lin.solve();
                    res.la = res.la.add(s.since());
                    if let Some(&k) = ks.iter().find(|&&k| inst.e.mul(pt, k) == q) {
                        res.k = k;
                        res.ok = true;
                        res.steps = steps;
                        res.brels = lin.brels.len() as u64;
                        res.jac = jac;
                        res.jac_ops = jac.j;
                        res.mem = (lin.pivots.len() + lin.brels.len()) as u64;
                        res.walk = c0.since().minus(res.la);
                        res.secs = t0.elapsed().as_secs_f64();
                        return res;
                    }
                    target += 10;
                    if target > nb + 200 {
                        return finish_fail(res, t0);
                    }
                }
            }
            cnt_n(1);
            let hx =
                d.u.iter()
                    .chain(d.v.iter())
                    .fold(0u64, |h, c| (h ^ c.0).wrapping_mul(0x9e37_79b9_7f4a_7c15))
                    >> 40;
            if indep {
                a = xorshift(rng) % ell;
                b = xorshift(rng) % ell;
                d = jop!(jadd_fast(f, &jmul_fast(f, &tp, a), &jmul_fast(f, &tq, b)));
            } else {
                let (m, ma, mb) = &mults[(hx as usize) % IC_R];
                d = jop!(jadd_fast(f, &d, m));
                a = (a + ma) % ell;
                b = (b + mb) % ell;
            }
            steps += 1;
        }
    }
}

fn finish_fail(mut res: ArmResult, t0: Instant) -> ArmResult {
    res.ok = false;
    res.secs = t0.elapsed().as_secs_f64();
    res
}

fn add_entry<K: Ord + Copy>(v: &mut Vec<(K, u64)>, k: K, c: u64, ell: u64) {
    match v.binary_search_by(|e| e.0.cmp(&k)) {
        Ok(i) => {
            let nv = (v[i].1 + c) % ell;
            if nv == 0 {
                v.remove(i);
            } else {
                v[i].1 = nv;
            }
        }
        Err(i) => v.insert(i, (k, c)),
    }
}

// ---------------------------------------------------------------------------
// Arm D: decomposition oracle (Weil-descended S_3) and the IC walk on E.
// ---------------------------------------------------------------------------

/// Roots in F_p of a polynomial (gcd with x^p − x, then Cantor–Zassenhaus).
fn roots_fp(a: &[Fp], rng: &mut u64) -> Vec<Fp> {
    let a = trim(a.to_vec());
    if deg(&a) <= 0 {
        return Vec::new();
    }
    let a = pmonic(&a);
    let p = pp();
    // x^p mod a
    let mut r = vec![Fp(1)];
    let mut base = prem(&[Fp(0), Fp(1)], &a);
    let mut e = p;
    while e > 0 {
        if e & 1 == 1 {
            r = prem(&pmul(&r, &base), &a);
        }
        e >>= 1;
        if e > 0 {
            base = prem(&pmul(&base, &base), &a);
        }
    }
    let xp_x = psub(&r, &[Fp(0), Fp(1)]);
    let g = if xp_x.is_empty() {
        a.clone()
    } else {
        pxgcd(&a, &xp_x).0
    };
    let mut out = Vec::new();
    split_roots(&g, rng, &mut out);
    out
}

fn split_roots(g: &[Fp], rng: &mut u64, out: &mut Vec<Fp>) {
    let p = pp();
    match deg(g) {
        d if d <= 0 => {}
        1 => out.push(g[0].neg().mul(g[1].inv())),
        2 => {
            let g = pmonic(g);
            let disc = g[1].sqr().sub(g[0].mul(Fp(4)));
            if let Some(s) = sqrt_p(disc.0) {
                let half = Fp((p + 1) / 2);
                out.push(g[1].neg().add(Fp(s)).mul(half));
                if s != 0 {
                    out.push(g[1].neg().sub(Fp(s)).mul(half));
                }
            }
        }
        _ => loop {
            let delta = Fp(xorshift(rng) % p);
            let mut r = vec![Fp(1)];
            let mut base = prem(&[delta, Fp(1)], g);
            let mut e = (p - 1) / 2;
            while e > 0 {
                if e & 1 == 1 {
                    r = prem(&pmul(&r, &base), g);
                }
                e >>= 1;
                if e > 0 {
                    base = prem(&pmul(&base, &base), g);
                }
            }
            let h = psub(&r, &[Fp(1)]);
            if h.is_empty() {
                continue;
            }
            let d = pxgcd(g, &h).0;
            if deg(&d) > 0 && deg(&d) < deg(g) {
                split_roots(&d, rng, out);
                split_roots(&pdivrem(g, &d).0, rng, out);
                return;
            }
        },
    }
}

/// All (x1, x2) ∈ F_p^2 (x1 ≤ x2 as residues) with S_3(x1, x2, x3) = 0 on
/// y^2 = x^3 + A x + B, by Weil descent of S_3 to two equations over F_p.
fn decompose_x(e: &Ell, x3: Fp2, rng: &mut u64) -> Vec<(Fp, Fp)> {
    // S3 = (e1^2 - 4 e2) x3^2 - 2 (e1 (e2 + A) + 2B) x3 + (e2 - A)^2 - 4 B e1
    //    = e2^2 + e2 (-4 x3^2 - 2 e1 x3 - 2A) + e1^2 x3^2 - 2 A x3 e1 - 4 B x3 + A^2 - 4 B e1
    let (a, b) = (e.a, e.b);
    let x3s = x3.sqr();
    // coefficients as polynomials in e1 over F_{p^2}:
    // lin(e1) = -4 x3^2 - 2A - 2 x3 e1        (coefficient of e2)
    // con(e1) = x3^2 e1^2 + (-2 A x3 - 4B) e1 + (A^2 - 4 B x3)
    let two = Fp2::from_u64(2);
    let four = Fp2::from_u64(4);
    let lin = [x3s.mul(four).add(a.mul(two)).neg(), x3.mul(two).neg()];
    let con = [
        a.sqr().sub(b.mul(x3).mul(four)),
        a.mul(x3).mul(two).add(b.mul(four)).neg(),
        x3s,
    ];
    // components: G1 = s-part: lin1(e1) e2 + con1(e1) = 0 (no e2^2 term)
    let m1: Vec<Fp> = trim(lin.iter().map(|c| Fp(c.1)).collect());
    let n1: Vec<Fp> = trim(con.iter().map(|c| Fp(c.1)).collect());
    let m0: Vec<Fp> = trim(lin.iter().map(|c| Fp(c.0)).collect());
    let n0: Vec<Fp> = trim(con.iter().map(|c| Fp(c.0)).collect());
    // e2 = -n1/m1 ; G0 = e2^2 + m0 e2 + n0 = 0  => n1^2 - m0 n1 m1 + n0 m1^2 = 0
    let quart = padd(
        &psub(&pmul(&n1, &n1), &pmul(&pmul(&m0, &n1), &m1)),
        &pmul(&n0, &pmul(&m1, &m1)),
    );
    let mut out = Vec::new();
    let mut e1s = roots_fp(&quart, rng);
    // m1(e1) = 0 roots need separate handling (e2 free in G1): rare, check them too
    if !m1.is_empty() && deg(&m1) == 1 {
        let r = m1[0].neg().mul(m1[1].inv());
        if !e1s.contains(&r) {
            e1s.push(r);
        }
    }
    let half = Fp((pp() + 1) / 2);
    for e1 in e1s {
        let m1v = peval(&m1, e1);
        let mut e2s = Vec::new();
        if m1v.is_zero() {
            if !peval(&n1, e1).is_zero() {
                continue;
            }
            // G0 quadratic in e2
            let m0v = peval(&m0, e1);
            let n0v = peval(&n0, e1);
            let disc = m0v.sqr().sub(n0v.mul(Fp(4)));
            if let Some(s) = sqrt_p(disc.0) {
                e2s.push(m0v.neg().add(Fp(s)).mul(half));
                e2s.push(m0v.neg().sub(Fp(s)).mul(half));
            }
        } else {
            let e2 = peval(&n1, e1).neg().mul(m1v.inv());
            // must satisfy G0
            let g0 = e2.sqr().add(peval(&m0, e1).mul(e2)).add(peval(&n0, e1));
            if !g0.is_zero() {
                continue;
            }
            e2s.push(e2);
        }
        for e2 in e2s {
            let disc = e1.sqr().sub(e2.mul(Fp(4)));
            if let Some(s) = sqrt_p(disc.0) {
                let x1 = e1.add(Fp(s)).mul(half);
                let x2 = e1.sub(Fp(s)).mul(half);
                let pr = if x1.0 <= x2.0 { (x1, x2) } else { (x2, x1) };
                if !out.contains(&pr) {
                    out.push(pr);
                }
            }
        }
    }
    out
}

/// Arm D: decomposition index calculus in E(F_{p^2}), factor base x ∈ F_p.
fn arm_dec(inst: &Instance, pt: EP, q: EP, nb_target: usize, c: u32, rng: &mut u64) -> ArmResult {
    let t0 = Instant::now();
    let c0 = Counts::now();
    let p = inst.p;
    let ell = inst.ell;
    let e = &inst.e;
    let mut res = ArmResult {
        arm: "D".into(),
        c,
        ..Default::default()
    };
    let mut bidx = vec![u32::MAX; p as usize];
    let mut order: Vec<u64> = (0..p).collect();
    for i in (1..order.len()).rev() {
        let j = (xorshift(rng) % (i as u64 + 1)) as usize;
        order.swap(i, j);
    }
    let mut cnt = 0usize;
    for &x in &order {
        if cnt == nb_target {
            break;
        }
        if e.rhs(Fp2(x, 0)).sqrt().is_some() {
            bidx[x as usize] = cnt as u32;
            cnt += 1;
        }
    }
    let nb = cnt;
    res.nb = nb;
    let mut lin = Lin::new(ell, nb);
    let mut mults = Vec::new();
    for _ in 0..IC_R {
        let (a, b) = (xorshift(rng) % ell, xorshift(rng) % ell);
        mults.push((e.add(e.mul(pt, a), e.mul(q, b)), a, b));
    }
    let mut target = nb + 10;
    let mut steps = 0u64;
    loop {
        let (mut a, mut b) = (xorshift(rng) % ell, xorshift(rng) % ell);
        let mut r = e.add(e.mul(pt, a), e.mul(q, b));
        for _ in 0..IC_RESTART {
            if let EP::A(x3, _) = r {
                let decs = decompose_x(e, x3, rng);
                for (x1, x2) in decs {
                    let (Some(y1), Some(y2)) = (
                        e.rhs(Fp2::from_fp(x1)).sqrt(),
                        e.rhs(Fp2::from_fp(x2)).sqrt(),
                    ) else {
                        continue;
                    };
                    let p1 = EP::A(Fp2::from_fp(x1), y1);
                    let p2 = EP::A(Fp2::from_fp(x2), y2);
                    // signs: r = e1 p1 + e2 p2
                    let mut found = None;
                    let s = e.add(p1, p2);
                    if s == r {
                        found = Some((1i32, 1i32));
                    } else if s == e.neg(r) {
                        found = Some((-1, -1));
                    } else {
                        let dd = e.add(p1, e.neg(p2));
                        if dd == r {
                            found = Some((1, -1));
                        } else if dd == e.neg(r) {
                            found = Some((-1, 1));
                        }
                    }
                    let Some((s1, s2)) = found else { continue };
                    let mut row = Row {
                        a,
                        b,
                        ..Default::default()
                    };
                    for (x, y, sg) in [(x1, y1, s1), (x2, y2, s2)] {
                        cnt_n(1);
                        let canon_pos = y.key() <= y.neg().key();
                        let sgn = if canon_pos == (sg > 0) { 1 } else { ell - 1 };
                        let bi = bidx[x.0 as usize];
                        if bi != u32::MAX {
                            add_entry(&mut row.bcols, bi, sgn, ell);
                        } else {
                            add_entry(&mut row.lp, x.0, sgn, ell);
                        }
                    }
                    res.rels += 1;
                    let s = Counts::now();
                    let cand = lin.push(row);
                    res.la = res.la.add(s.since());
                    if let Some(k) = cand.filter(|&k| e.mul(pt, k) == q) {
                        res.k = k;
                        res.ok = true;
                        res.steps = steps;
                        res.brels = lin.brels.len() as u64;
                        res.walk = c0.since().minus(res.la);
                        res.secs = t0.elapsed().as_secs_f64();
                        return res;
                    }
                    if lin.brels.len() >= target {
                        let s = Counts::now();
                        let ks = lin.solve();
                        res.la = res.la.add(s.since());
                        if let Some(&k) = ks.iter().find(|&&k| e.mul(pt, k) == q) {
                            res.k = k;
                            res.ok = true;
                            res.steps = steps;
                            res.brels = lin.brels.len() as u64;
                            res.walk = c0.since().minus(res.la);
                            res.secs = t0.elapsed().as_secs_f64();
                            return res;
                        }
                        target += 10;
                        if target > nb + 200 {
                            return finish_fail(res, t0);
                        }
                    }
                    break; // one relation per point
                }
            }
            let hx = match r {
                EP::A(x, _) => hash_fp2(x),
                EP::O => 0,
            };
            let (m, ma, mb) = mults[(hx as usize) % IC_R];
            r = e.add(r, m);
            a = (a + ma) % ell;
            b = (b + mb) % ell;
            steps += 1;
        }
    }
}

// ---------------------------------------------------------------------------
// Checks (K) and the recorded run.
// ---------------------------------------------------------------------------

/// Brute-force decomposition sets for every x3 (p small): x3 -> {(x1, x2)}.
fn brute_decomp(e: &Ell, p: u64) -> HashMap<(u64, u64), Vec<(Fp, Fp)>> {
    let mut pts = Vec::new();
    for x in 0..p {
        if let Some(y) = e.rhs(Fp2(x, 0)).sqrt() {
            pts.push((Fp(x), EP::A(Fp2(x, 0), y)));
        }
    }
    let mut map: HashMap<(u64, u64), Vec<(Fp, Fp)>> = HashMap::new();
    for i in 0..pts.len() {
        for j in i..pts.len() {
            let (x1, p1) = pts[i];
            let (x2, p2) = pts[j];
            for s in [e.add(p1, p2), e.add(p1, e.neg(p2))] {
                if let EP::A(x3, _) = s {
                    let pr = if x1.0 <= x2.0 { (x1, x2) } else { (x2, x1) };
                    let v = map.entry((x3.0, x3.1)).or_default();
                    if !v.contains(&pr) {
                        v.push(pr);
                    }
                }
            }
        }
    }
    map
}

fn run_checks(out: &Path) {
    let mut rng = 0x6e32_c4ec_u64;
    let mut log = String::new();
    // K1: oracle vs brute force for small p
    let mut oracle_viol = 0u64;
    let mut oracle_points = 0u64;
    let mut insts = 0;
    for &bits in &[10u32, 11, 12, 13] {
        let inst = Instance::new(bits, &mut rng);
        let p = inst.p;
        if p > 101 {
            continue;
        }
        insts += 1;
        let br = brute_decomp(&inst.e, p);
        for x0 in 0..p {
            for x1 in 0..p {
                let x3 = Fp2(x0, x1);
                if inst.e.rhs(x3).sqrt().is_none() {
                    continue;
                }
                oracle_points += 1;
                let mut got = decompose_x(&inst.e, x3, &mut rng);
                // keep only pairs whose points exist
                got.retain(|(a, b)| {
                    inst.e.rhs(Fp2::from_fp(*a)).sqrt().is_some()
                        && inst.e.rhs(Fp2::from_fp(*b)).sqrt().is_some()
                });
                got.sort_by_key(|(a, b)| (a.0, b.0));
                let mut want = br.get(&(x0, x1)).cloned().unwrap_or_default();
                want.sort_by_key(|(a, b)| (a.0, b.0));
                if got != want {
                    oracle_viol += 1;
                }
            }
        }
    }
    log.push_str(&format!(
        "K1 decomposition oracle vs brute force (p <= 101): instances {insts}, points {oracle_points}, mismatches {oracle_viol}\n"
    ));
    // K2..K4 on instances at each size
    for &bits in &[16u32, 20, 24] {
        for _ in 0..2 {
            let inst = Instance::new(bits, &mut rng);
            let nc = inst.count_c();
            let pt0 = inst.e.random_point(&mut rng);
            let pt = inst.e.mul(pt0, inst.h);
            let pt2 = inst.e.mul(inst.e.random_point(&mut rng), inst.h);
            let ok_ell = inst.e.mul(pt, inst.ell) == EP::O;
            let tp = inst.transfer(pt);
            let tp2 = inst.transfer(pt2);
            let tsum = inst.transfer(inst.e.add(pt, pt2));
            let fq = &inst.fq;
            let (hom, nz, ordj, valid) = match (&tp, &tp2, &tsum) {
                (Some(a), Some(b), Some(s)) => (
                    jadd(fq, a, b) == *s,
                    !a.is_zero(),
                    jmul(fq, a, inst.ell).is_zero(),
                    jvalid(fq, a) && jvalid(fq, b),
                ),
                _ => (false, false, false, false),
            };
            // N·D = 0 for a random Jacobian divisor (sum of two random points)
            let mut rd = Div::zero();
            for _ in 0..2 {
                loop {
                    let x = Fp(xorshift(&mut rng) % inst.p);
                    if let Some(w) = sqrt_p(peval(fq, x).0) {
                        rd = jadd(
                            fq,
                            &rd,
                            &Div {
                                u: vec![x.neg(), Fp(1)],
                                v: trim(vec![Fp(w)]),
                            },
                        );
                        break;
                    }
                }
            }
            let nd = jmul(fq, &rd, inst.n).is_zero();
            log.push_str(&format!(
                "K2-K4 p={} N={} h={} ell={}: #C(F_p)={} (p+1={}) | ell·P=O {} | tau valid {} hom {} nonzero {} ell·tau(P)=0 {} | N·D=0 {}\n",
                inst.p, inst.n, inst.h, inst.ell, nc, inst.p + 1, ok_ell, valid, hom, nz, ordj, nd
            ));
        }
    }
    // K5: measured F_p cost per genus-2 operation (fast path vs Cantor), and
    // per E(F_{p^2}) affine addition, at one instance per size
    for &bits in &[20u32, 24, 28, 32] {
        let inst = Instance::new(bits, &mut rng);
        let f = &inst.fq;
        let pts: Vec<Div<Fp>> = (0..401)
            .map(|_| {
                let pt = inst.e.random_point(&mut rng);
                inst.transfer(pt).unwrap_or(Div::zero())
            })
            .collect();
        let mut agree = true;
        let (mut cf, mut cc, mut df, mut dc) = (
            Counts::default(),
            Counts::default(),
            Counts::default(),
            Counts::default(),
        );
        for w in pts.windows(2) {
            let s0 = Counts::now();
            let x = jadd_fast(f, &w[0], &w[1]);
            cf = cf.add(s0.since());
            let s0 = Counts::now();
            let y = jadd(f, &w[0], &w[1]);
            cc = cc.add(s0.since());
            let s0 = Counts::now();
            let xd = jadd_fast(f, &w[0], &w[0]);
            df = df.add(s0.since());
            let s0 = Counts::now();
            let yd = jadd(f, &w[0], &w[0]);
            dc = dc.add(s0.since());
            agree &= x == y && xd == yd;
        }
        let (pa, pb) = (inst.e.random_point(&mut rng), inst.e.random_point(&mut rng));
        let s0 = Counts::now();
        let _ = inst.e.add(pa, pb);
        let ea = s0.since();
        let k = 400.0;
        log.push_str(&format!(
            "K5 p={}: genus-2 add fast {:.1}M+{:.2}I cantor {:.1}M+{:.2}I | double fast {:.1}M+{:.2}I cantor {:.1}M+{:.2}I | agree {} | E(F_p^2) add {}M+{}I\n",
            inst.p,
            cf.m as f64 / k, cf.i as f64 / k, cc.m as f64 / k, cc.i as f64 / k,
            df.m as f64 / k, df.i as f64 / k, dc.m as f64 / k, dc.i as f64 / k,
            agree, ea.m, ea.i
        ));
    }
    std::fs::write(out.join("checks.txt"), &log).unwrap();
    eprint!("{log}");
}

#[derive(Clone, Debug)]
struct Job {
    bits: u32,
    inst_id: usize,
    seed: u64,
}

fn run_job(job: &Job) -> Vec<String> {
    let mut rng = job.seed | 1;
    let inst = Instance::new(job.bits, &mut rng);
    let mut rows = Vec::new();
    // measured cost of one affine E(F_{p^2}) addition (generic case)
    let (pa, pb) = (inst.e.random_point(&mut rng), inst.e.random_point(&mut rng));
    let s = Counts::now();
    let _ = inst.e.add(pa, pb);
    let eadd = s.since();
    let pt = loop {
        let v = inst.e.mul(inst.e.random_point(&mut rng), inst.h);
        if v != EP::O {
            break v;
        }
    };
    for tgt in 0..TARGETS {
        let k = 1 + xorshift(&mut rng) % (inst.ell - 1);
        let q = inst.e.mul(pt, k);
        let mut results = vec![arm_rho(&inst, pt, q, &mut rng)];
        let cs: Vec<u32> = std::env::var("N2_CS")
            .unwrap_or_else(|_| "1,2,4".into())
            .split(',')
            .map(|x| x.trim().parse().unwrap())
            .collect();
        for &c in &cs {
            let rj = arm_jac(&inst, pt, q, c, &mut rng);
            let nb = rj.nb;
            results.push(rj);
            results.push(arm_dec(&inst, pt, q, nb, c, &mut rng));
        }
        for r in results {
            let ok = r.ok && inst.e.mul(pt, r.k) == q && r.k == k;
            rows.push(format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{:.3}",
                job.bits,
                job.inst_id,
                inst.p,
                inst.n,
                inst.h,
                inst.ell,
                tgt,
                r.arm,
                r.c,
                r.nb,
                r.steps,
                r.rels,
                r.brels,
                r.jac_ops,
                r.walk.m,
                r.walk.i,
                r.walk.n,
                r.jac.m,
                r.jac.i,
                r.la.m,
                r.la.i,
                r.la.n,
                eadd.m,
                eadd.i,
                u8::from(ok),
                r.secs
            ));
        }
        eprintln!("bits {} inst {} target {} done", job.bits, job.inst_id, tgt);
    }
    rows
}

/// Pricing band (Amendment C): cost of a row in F_p multiplications for an
/// inversion price `ipr`, Jacobian pricing (`measured` or one E(F_{p^2})
/// addition per Jacobian operation) and native weight `w`.
fn row_cost(r: &HashMap<&str, f64>, ipr: f64, jac_as_eadd: bool, w: f64) -> f64 {
    let m = r["walk_M"] + r["la_M"];
    let i = r["walk_I"] + r["la_I"];
    let n = r["walk_native"] + r["la_native"];
    let mut c = m + ipr * i + w * n;
    if jac_as_eadd {
        c -= r["jac_M"] + ipr * r["jac_I"];
        c += r["jac_ops"] * (r["eadd_M"] + ipr * r["eadd_I"]);
    }
    c
}

/// (routes_summary.csv, ranking.csv, summary text)
fn summarize(csv: &str) -> (String, String, String) {
    let mut lines = csv.lines();
    let hdr: Vec<&str> = lines.next().unwrap().split(',').collect();
    let mut rows: Vec<(HashMap<&str, f64>, String)> = Vec::new();
    for l in lines {
        let v: Vec<&str> = l.split(',').collect();
        let mut m = HashMap::new();
        let mut arm = String::new();
        for (k, x) in hdr.iter().zip(v.iter()) {
            if *k == "arm" {
                arm = x.to_string();
            } else {
                m.insert(*k, x.parse::<f64>().unwrap_or(f64::NAN));
            }
        }
        rows.push((m, arm));
    }
    let mut bits: Vec<u64> = rows.iter().map(|(m, _)| m["bits"] as u64).collect();
    bits.sort_unstable();
    bits.dedup();
    let arms = [
        ("J", 1.0),
        ("J", 2.0),
        ("J", 4.0),
        ("D", 1.0),
        ("D", 2.0),
        ("D", 4.0),
    ];
    let mut out = String::from(
        "bits,arm,c,ipr,jac_pricing,native_w,runs,verified,mean_p,mean_cost,mean_rho_cost,ratio_to_rho,mean_steps_over_p,rho_steps_over_p\n",
    );
    let mut rank = String::from(
        "bits,arm,c,cells,ic_cheaper_cells,rho_cheaper_cells,min_ratio,max_ratio,verdict\n",
    );
    let mut txt = String::new();
    let mut verified_all = true;
    for &b in &bits {
        let sel = |arm: &str, c: f64| -> Vec<&HashMap<&str, f64>> {
            rows.iter()
                .filter(|(m, a)| m["bits"] as u64 == b && a == arm && (arm == "R" || m["c"] == c))
                .map(|(m, _)| m)
                .collect()
        };
        let rho = sel("R", 0.0);
        let mean_p = rho.iter().map(|m| m["p"]).sum::<f64>() / rho.len() as f64;
        let rho_steps = rho.iter().map(|m| m["steps"] / m["p"]).sum::<f64>() / rho.len() as f64;
        for &(arm, c) in &arms {
            let ic = sel(arm, c);
            if ic.is_empty() {
                continue;
            }
            let ver = ic.iter().chain(rho.iter()).all(|m| m["verified"] == 1.0);
            verified_all &= ver;
            let steps = ic.iter().map(|m| m["steps"] / m["p"]).sum::<f64>() / ic.len() as f64;
            let (mut cells, mut icw, mut rw) = (0, 0, 0);
            let (mut lo, mut hi) = (f64::MAX, 0f64);
            for ipr_kind in 0..2 {
                for jp in [false, true] {
                    if arm == "D" && jp {
                        continue;
                    }
                    for w in [0.0, 0.1, 1.0] {
                        // per-run inversion price uses that run's p
                        let ipr_of = |m: &HashMap<&str, f64>| {
                            if ipr_kind == 0 {
                                3.0
                            } else {
                                2.0 * m["p"].log2()
                            }
                        };
                        let ic_c = ic
                            .iter()
                            .map(|m| row_cost(m, ipr_of(m), jp, w))
                            .sum::<f64>()
                            / ic.len() as f64;
                        let rho_c = rho
                            .iter()
                            .map(|m| row_cost(m, ipr_of(m), false, w))
                            .sum::<f64>()
                            / rho.len() as f64;
                        let ratio = ic_c / rho_c;
                        lo = lo.min(ratio);
                        hi = hi.max(ratio);
                        cells += 1;
                        if ratio < 1.0 {
                            icw += 1;
                        } else {
                            rw += 1;
                        }
                        out.push_str(&format!(
                            "{b},{arm},{c},{},{},{w},{},{},{mean_p:.0},{ic_c:.1},{rho_c:.1},{ratio:.4},{steps:.4},{rho_steps:.4}\n",
                            if ipr_kind == 0 { "3M" } else { "2log2p_M" },
                            if jp { "eadd" } else { "measured" },
                            ic.len(),
                            u8::from(ver)
                        ));
                    }
                }
            }
            let verdict = if icw == cells {
                "IC cheaper in every cell"
            } else if rw == cells {
                "rho cheaper in every cell"
            } else {
                "unresolved"
            };
            rank.push_str(&format!(
                "{b},{arm},{c},{cells},{icw},{rw},{lo:.4},{hi:.4},{verdict}\n"
            ));
            txt.push_str(&format!(
                "O/J bits {b} arm {arm} c {c}: IC/rho ratio in [{lo:.3}, {hi:.3}] over {cells} cells -> {verdict}; steps/p {steps:.3} (rho {rho_steps:.3})\n"
            ));
        }
    }
    // scaling exponents: least-squares slope of log(mean cost) on log(mean p),
    // at the measured Jacobian pricing, I = 3M, w = 0.1
    let mut series: Vec<(&str, f64)> = vec![("R", 0.0)];
    series.extend(arms.iter().copied());
    for (arm, c) in series {
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        for &b in &bits {
            let sel: Vec<&HashMap<&str, f64>> = rows
                .iter()
                .filter(|(m, a)| m["bits"] as u64 == b && a == arm && (arm == "R" || m["c"] == c))
                .map(|(m, _)| m)
                .collect();
            if sel.is_empty() {
                continue;
            }
            let mp = sel.iter().map(|m| m["p"]).sum::<f64>() / sel.len() as f64;
            let mc = sel
                .iter()
                .map(|m| row_cost(m, 3.0, false, 0.1))
                .sum::<f64>()
                / sel.len() as f64;
            xs.push(mp.ln());
            ys.push(mc.ln());
        }
        if xs.len() >= 2 {
            let n = xs.len() as f64;
            let mx = xs.iter().sum::<f64>() / n;
            let my = ys.iter().sum::<f64>() / n;
            let sxy: f64 = xs.iter().zip(&ys).map(|(x, y)| (x - mx) * (y - my)).sum();
            let sxx: f64 = xs.iter().map(|x| (x - mx).powi(2)).sum();
            txt.push_str(&format!(
                "O/J scaling arm {arm} c {c}: cost ~ p^{:.3}\n",
                sxy / sxx
            ));
        }
    }
    // K6: rho positive control, mean steps/p against 0.886/sqrt(h) per size
    for &b in &bits {
        let rho: Vec<&HashMap<&str, f64>> = rows
            .iter()
            .filter(|(m, a)| m["bits"] as u64 == b && a == "R")
            .map(|(m, _)| m)
            .collect();
        let n = rho.len() as f64;
        let obs: Vec<f64> = rho.iter().map(|m| m["steps"] / m["p"]).collect();
        let th: Vec<f64> = rho
            .iter()
            .map(|m| 0.886 * (m["ell"] / (m["p"] * m["p"])).sqrt())
            .collect();
        let mo = obs.iter().sum::<f64>() / n;
        let mt = th.iter().sum::<f64>() / n;
        let sd = (obs.iter().map(|x| (x - mo).powi(2)).sum::<f64>() / (n - 1.0).max(1.0)).sqrt();
        txt.push_str(&format!(
            "K6 bits {b}: rho steps/p mean {mo:.4} (s.e. {:.4}, {n} runs) vs 0.886*sqrt(ell)/p mean {mt:.4}\n",
            sd / n.sqrt()
        ));
    }
    txt.push_str(&format!("O/J all answers verified: {verified_all}\n"));
    (out, rank, txt)
}

/// Amendment F control: Arm J with its walk (repeat tracking) and Arm J with
/// independent random trials, c = 2, same instance generator as the run.
fn control_job(job: &Job) -> Vec<String> {
    let mut rng = job.seed | 1;
    let inst = Instance::new(job.bits, &mut rng);
    let pt = loop {
        let v = inst.e.mul(inst.e.random_point(&mut rng), inst.h);
        if v != EP::O {
            break v;
        }
    };
    let mut rows = Vec::new();
    for tgt in 0..3 {
        let k = 1 + xorshift(&mut rng) % (inst.ell - 1);
        let q = inst.e.mul(pt, k);
        for (name, indep) in [("Jwalk", false), ("Jind", true)] {
            let r = arm_jac_impl(&inst, pt, q, 2, &mut rng, true, indep, 1.0);
            let ok = r.ok && r.k == k && inst.e.mul(pt, r.k) == q;
            rows.push(format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}",
                job.bits,
                job.inst_id,
                inst.p,
                inst.h,
                inst.ell,
                tgt,
                name,
                r.steps,
                r.rels,
                r.dup_events,
                u8::from(r.solved_by_dup),
                u8::from(ok)
            ));
        }
        eprintln!(
            "control bits {} inst {} target {} done",
            job.bits, job.inst_id, tgt
        );
    }
    rows
}

fn summarize_control(csv: &str) -> String {
    let mut by: std::collections::BTreeMap<(u64, String), Vec<(f64, f64, f64, f64)>> =
        Default::default();
    let mut byh: std::collections::BTreeMap<(String, u64), Vec<f64>> = Default::default();
    let mut all_ok = true;
    for l in csv.lines().skip(1) {
        let v: Vec<&str> = l.split(',').collect();
        let bits: u64 = v[0].parse().unwrap();
        let p: f64 = v[2].parse().unwrap();
        let h: u64 = v[3].parse().unwrap();
        let steps: f64 = v[7].parse().unwrap();
        let dups: f64 = v[9].parse().unwrap();
        let sbd: f64 = v[10].parse().unwrap();
        all_ok &= v[11] == "1";
        by.entry((bits, v[6].to_string()))
            .or_default()
            .push((steps / p, dups, sbd, 1.0));
        byh.entry((v[6].to_string(), h))
            .or_default()
            .push(steps / p);
    }
    let mut t = String::new();
    for ((bits, arm), v) in &by {
        let n = v.len() as f64;
        let m = v.iter().map(|x| x.0).sum::<f64>() / n;
        let sd = (v.iter().map(|x| (x.0 - m).powi(2)).sum::<f64>() / (n - 1.0)).sqrt();
        let dup = v.iter().map(|x| x.1).sum::<f64>() / n;
        let sbd = v.iter().map(|x| x.2).sum::<f64>();
        t.push_str(&format!(
            "F bits {bits} {arm}: runs {n} steps/p mean {m:.4} sd {sd:.4} | repeated divisors per run {dup:.2} | solved by a repeat {sbd}/{n}\n"
        ));
    }
    for ((arm, h), v) in &byh {
        let n = v.len() as f64;
        let m = v.iter().sum::<f64>() / n;
        let sd = (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (n - 1.0).max(1.0)).sqrt();
        t.push_str(&format!(
            "F by cofactor {arm} h={h}: runs {n} steps/p mean {m:.4} sd {sd:.4} (s.e. {:.4})\n",
            sd / n.sqrt()
        ));
    }
    t.push_str(&format!("F all answers verified: {all_ok}\n"));
    t
}

// ---------------------------------------------------------------------------
// Addendum H1 (cofactor law), H4 (memory against time).
// ---------------------------------------------------------------------------

/// Structure of the cofactor part: the largest order of `ℓ·Q` over 24 random
/// points Q of E (equal to h iff that part is cyclic).
fn cofactor_exponent(inst: &Instance, rng: &mut u64) -> u64 {
    let mut best = 1u64;
    for _ in 0..24 {
        let q = inst.e.mul(inst.e.random_point(rng), inst.ell);
        let mut ord = inst.h;
        for f in small_factors(inst.h) {
            while ord % f == 0 && inst.e.mul(q, ord / f) == EP::O {
                ord /= f;
            }
        }
        best = best.max(ord);
    }
    best
}

/// Model M-H1: first useful cycle in the uniform admissible-pair graph on the
/// instance's actual degree-1 points and their cofactor classes. Useful means
/// a dependency mod ℓ: a cycle whose sign product closes consistently, or any
/// cycle in (or edge between) components already fixed by an inconsistent one.
/// Returns the mean number of edges over `reps` simulations, and n (vertices).
fn model_first_cycle(inst: &Instance, reps: usize, rng: &mut u64) -> (f64, usize) {
    let f = &inst.fq;
    let p = inst.p;
    // vertices: ξ with F(ξ) a nonzero square; class = ℓ·(ξ, w)
    let mut class_of: Vec<usize> = Vec::new();
    let mut self_ok: Vec<bool> = Vec::new();
    let mut keys: HashMap<Vec<u64>, usize> = HashMap::new();
    for x in 0..p {
        let v = peval(f, Fp(x)).0;
        let Some(w) = (if v == 0 { None } else { sqrt_p(v) }) else {
            continue;
        };
        let d = Div {
            u: vec![Fp(x).neg(), Fp(1)],
            v: trim(vec![Fp(w)]),
        };
        let c = jmul_fast(f, &d, inst.ell);
        let key = |d: &Div<Fp>| -> Vec<u64> { d.u.iter().chain(d.v.iter()).map(|c| c.0).collect() };
        let k1 = key(&c);
        let k2 = key(&c.neg());
        let k = if k1 <= k2 { k1.clone() } else { k2 };
        let n = keys.len();
        let id = *keys.entry(k).or_insert(n);
        class_of.push(id);
        // R = 2P is in the ℓ-part iff 2c = 0, i.e. c = -c
        self_ok.push(k1 == key(&c.neg()));
    }
    let n = class_of.len();
    let mut groups: Vec<Vec<usize>> = vec![Vec::new(); keys.len()];
    for (v, &g) in class_of.iter().enumerate() {
        groups[g].push(v);
    }
    // choose a group with probability ∝ |G|^2, then two uniform members
    let mut cum = Vec::with_capacity(groups.len());
    let mut acc = 0f64;
    for g in &groups {
        acc += (g.len() * g.len()) as f64;
        cum.push(acc);
    }
    let mut total = 0f64;
    for _ in 0..reps {
        let mut parent: Vec<usize> = (0..n).collect();
        let mut par: Vec<u8> = vec![0; n]; // parity to parent
        let mut rigid: Vec<bool> = vec![false; n];
        fn find(parent: &mut [usize], par: &mut [u8], x: usize) -> (usize, u8) {
            let mut path = Vec::new();
            let mut y = x;
            while parent[y] != y {
                path.push(y);
                y = parent[y];
            }
            let root = y;
            // compress: parity of each node to root
            let mut acc = 0u8;
            for &z in path.iter().rev() {
                acc ^= par[z];
                par[z] = acc;
                parent[z] = root;
            }
            (root, if path.is_empty() { 0 } else { par[x] })
        }
        let mut edges = 0u64;
        loop {
            let r = (xorshift(rng) as f64 / u64::MAX as f64) * acc;
            let gi = cum.partition_point(|&c| c < r).min(groups.len() - 1);
            let g = &groups[gi];
            let u = g[(xorshift(rng) % g.len() as u64) as usize];
            let v = g[(xorshift(rng) % g.len() as u64) as usize];
            if u == v && !self_ok[u] {
                continue;
            }
            edges += 1;
            // edge sign: potential ratio -ε_u ε_v, random
            let sgn = (xorshift(rng) & 1) as u8;
            let (ru, pu) = find(&mut parent, &mut par, u);
            let (rv, pv) = find(&mut parent, &mut par, v);
            if u == v {
                // 2 x_u = r: fixes the component
                if rigid[ru] {
                    break;
                }
                rigid[ru] = true;
                continue;
            }
            if ru == rv {
                if rigid[ru] || (pu ^ pv ^ sgn) == 0 {
                    break;
                }
                rigid[ru] = true;
            } else {
                if rigid[ru] && rigid[rv] {
                    break;
                }
                parent[ru] = rv;
                par[ru] = pu ^ pv ^ sgn;
                rigid[rv] = rigid[rv] || rigid[ru];
            }
        }
        total += edges as f64;
    }
    (total / reps as f64, n)
}

#[derive(Clone, Debug)]
struct HJob {
    bits: u32,
    h: u64,
    inst_id: usize,
    seed: u64,
    theta: f64,
}

fn hsweep_job(job: &HJob) -> Vec<String> {
    let mut rng = job.seed | 1;
    let inst = Instance::with_h(job.bits, job.h, &mut rng);
    let cexp = cofactor_exponent(&inst, &mut rng);
    let pt = loop {
        let v = inst.e.mul(inst.e.random_point(&mut rng), inst.h);
        if v != EP::O {
            break v;
        }
    };
    let (model_edges, nverts) = if job.theta == 1.0 && job.bits <= 40 {
        model_first_cycle(&inst, 200, &mut rng)
    } else {
        (f64::NAN, 0)
    };
    let mut rows = Vec::new();
    for tgt in 0..TARGETS {
        let k = 1 + xorshift(&mut rng) % (inst.ell - 1);
        let q = inst.e.mul(pt, k);
        let mut arms: Vec<(&str, ArmResult)> = Vec::new();
        if job.theta == 1.0 {
            arms.push(("R", arm_rho(&inst, pt, q, &mut rng)));
            arms.push((
                "J",
                arm_jac_impl(&inst, pt, q, 2, &mut rng, false, false, 1.0),
            ));
            if job.bits <= 32 {
                arms.push((
                    "Jind",
                    arm_jac_impl(&inst, pt, q, 2, &mut rng, false, true, 1.0),
                ));
            }
        } else {
            arms.push((
                "Jtheta",
                arm_jac_impl(&inst, pt, q, 2, &mut rng, false, false, job.theta),
            ));
        }
        for (name, r) in arms {
            let ok = r.ok && r.k == k && inst.e.mul(pt, r.k) == q;
            rows.push(format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{:.3},{},{:.6}",
                job.bits,
                job.h,
                job.inst_id,
                inst.p,
                inst.ell,
                cexp,
                tgt,
                name,
                job.theta,
                r.steps,
                r.rels,
                r.mem,
                u8::from(ok),
                nverts,
                model_edges,
                r.walk.m + r.la.m,
                r.secs
            ));
        }
    }
    eprintln!(
        "h-sweep bits {} h {} inst {} theta {} done",
        job.bits, job.h, job.inst_id, job.theta
    );
    rows
}

/// Per-cell means and the α fit (bootstrap over instances) for H1; the
/// memory/time table for H4.
fn summarize_hsweep(csv: &str) -> String {
    use std::collections::BTreeMap;
    // per (bits, h, inst, arm): mean trials/p, split fraction, model trials/p
    let mut per: BTreeMap<(u32, u64, usize, String), Vec<(f64, f64, f64, f64, f64)>> =
        BTreeMap::new();
    let mut all_ok = true;
    let mut theta_rows: BTreeMap<(u32, String), Vec<(f64, f64)>> = BTreeMap::new();
    for l in csv.lines().skip(1) {
        let v: Vec<&str> = l.split(',').collect();
        let bits: u32 = v[0].parse().unwrap();
        let h: u64 = v[1].parse().unwrap();
        let inst: usize = v[2].parse().unwrap();
        let p: f64 = v[3].parse().unwrap();
        let arm = v[7].to_string();
        let theta: f64 = v[8].parse().unwrap();
        let steps: f64 = v[9].parse().unwrap();
        let rels: f64 = v[10].parse().unwrap();
        let mem: f64 = v[11].parse().unwrap();
        all_ok &= v[12] == "1";
        let model: f64 = v[14].parse().unwrap_or(f64::NAN);
        if arm == "Jtheta" || (arm == "J" && theta == 1.0 && h == 4) {
            theta_rows
                .entry((bits, format!("{theta}")))
                .or_default()
                .push((steps / p, mem / p));
        }
        per.entry((bits, h, inst, arm)).or_default().push((
            steps / p,
            rels / steps.max(1.0),
            model / p,
            mem / p,
            theta,
        ));
    }
    let mut out = String::new();
    // cell means
    let mut cells: BTreeMap<(u32, u64, String), Vec<f64>> = BTreeMap::new();
    let mut model_cells: BTreeMap<(u32, u64), Vec<f64>> = BTreeMap::new();
    for ((bits, h, _inst, arm), v) in &per {
        if v[0].4 != 1.0 {
            continue;
        }
        let m = v.iter().map(|x| x.0).sum::<f64>() / v.len() as f64;
        cells.entry((*bits, *h, arm.clone())).or_default().push(m);
        if arm == "J" {
            let split = v.iter().map(|x| x.1).sum::<f64>() / v.len() as f64;
            let me = v[0].2;
            if me.is_finite() && split > 0.0 {
                // model trials/p = model edges/p / split fraction
                model_cells.entry((*bits, *h)).or_default().push(me / split);
            }
        }
    }
    for ((bits, h, arm), v) in &cells {
        let n = v.len() as f64;
        let m = v.iter().sum::<f64>() / n;
        let sd = (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (n - 1.0).max(1.0)).sqrt();
        let model = model_cells
            .get(&(*bits, *h))
            .filter(|_| arm == "J")
            .map(|w| format!(" | model {:.4}", w.iter().sum::<f64>() / w.len() as f64))
            .unwrap_or_default();
        out.push_str(&format!(
            "H1 bits {bits} h {h} {arm}: trials/p {m:.4} (s.e. over {} instances {:.4}){model} | rho theory 0.886/sqrt(h) {:.4}\n",
            v.len(),
            sd / n.sqrt(),
            0.886 / (*h as f64).sqrt()
        ));
    }
    // α fit per arm: log(trials/p) = c - α log h, cell means over instances, bootstrap
    for arm in ["J", "R", "Jind"] {
        let mut pts: Vec<(f64, Vec<f64>)> = Vec::new();
        for ((_bits, h, a), v) in &cells {
            if a == arm {
                pts.push(((*h as f64).ln(), v.clone()));
            }
        }
        if pts.len() < 3 {
            continue;
        }
        let fit = |pts: &[(f64, f64)]| -> f64 {
            let n = pts.len() as f64;
            let mx = pts.iter().map(|x| x.0).sum::<f64>() / n;
            let my = pts.iter().map(|x| x.1).sum::<f64>() / n;
            let sxy: f64 = pts.iter().map(|x| (x.0 - mx) * (x.1 - my)).sum();
            let sxx: f64 = pts.iter().map(|x| (x.0 - mx).powi(2)).sum();
            -sxy / sxx
        };
        let base: Vec<(f64, f64)> = pts
            .iter()
            .map(|(lh, v)| (*lh, (v.iter().sum::<f64>() / v.len() as f64).ln()))
            .collect();
        let alpha = fit(&base);
        let mut rng = 0xa1fa_u64;
        let mut boots = Vec::new();
        for _ in 0..2000 {
            let bp: Vec<(f64, f64)> = pts
                .iter()
                .map(|(lh, v)| {
                    let m = (0..v.len())
                        .map(|_| v[(xorshift(&mut rng) % v.len() as u64) as usize])
                        .sum::<f64>()
                        / v.len() as f64;
                    (*lh, m.ln())
                })
                .collect();
            boots.push(fit(&bp));
        }
        boots.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let lo = boots[(0.025 * boots.len() as f64) as usize];
        let hi = boots[(0.975 * boots.len() as f64) as usize];
        let verdict = if arm != "J" {
            ""
        } else if hi < 0.5 {
            " -> alpha < 1/2: Arm J never beats rho on count (stop)"
        } else if lo > 0.5 {
            " -> alpha > 1/2: report the crossover h*"
        } else {
            " -> unresolved"
        };
        out.push_str(&format!("H1 alpha fit {arm}: alpha {alpha:.3} (bootstrap 95% [{lo:.3}, {hi:.3}], {} cells){verdict}\n", pts.len()));
    }
    for ((bits, th), v) in &theta_rows {
        let n = v.len() as f64;
        let t = v.iter().map(|x| x.0).sum::<f64>() / n;
        let m = v.iter().map(|x| x.1).sum::<f64>() / n;
        out.push_str(&format!("H4 bits {bits} theta {th}: trials/p {t:.4} stored/p {m:.5} time*memory/p^2 {:.5} ({} runs)\n", t * m, v.len()));
    }
    out.push_str(&format!("H1/H4 all answers verified: {all_ok}\n"));
    out
}

// ---------------------------------------------------------------------------
// Addendum H2: the general gluing E ~ E^σ along Frobenius on E[2], for any
// E/F_{p²} with j ∉ F_p, and genus-2 arithmetic on a sextic (real) model.
// ---------------------------------------------------------------------------

/// Class [D − D∞] on y² = f(x), deg f = 6, with D effective of degree 2 and
/// finite (u monic quadratic), or the zero class (u = 1). `None` marks a
/// degenerate case (a point at infinity in the unique effective
/// representative), which happens with probability O(1/p) per operation.
fn radd<F: Fld>(f: &[F], a: &Div<F>, b: &Div<F>) -> Option<Div<F>> {
    CJ.with(|c| c.set(c.get() + 1));
    if a.is_zero() {
        return Some(b.clone());
    }
    if b.is_zero() {
        return Some(a.clone());
    }
    if a.u.len() != 3 || b.u.len() != 3 {
        return None;
    }
    let (d1, e1, e2) = pxgcd(&a.u, &b.u);
    let (d, s1, s2, s3) = if d1.len() == 1 {
        (vec![F::one()], e1, e2, Vec::new())
    } else {
        let vs = padd(&a.v, &b.v);
        let (d, c1, c2) = pxgcd(&d1, &vs);
        (d, pmul(&c1, &e1), pmul(&c1, &e2), c2)
    };
    let mut u = pmul(&a.u, &b.u);
    let mut num = padd(&pmul(&pmul(&s1, &a.u), &b.v), &pmul(&pmul(&s2, &b.u), &a.v));
    if !s3.is_empty() {
        num = padd(&num, &pmul(&s3, &padd(&pmul(&a.v, &b.v), f)));
    }
    if d.len() > 1 {
        u = pdivrem(&u, &pmul(&d, &d)).0;
        num = pdivrem(&num, &d).0;
    }
    match deg(&u) {
        0 => Some(Div::zero()),
        2 => {
            let v = prem(&num, &u);
            Some(Div { u, v })
        }
        4 => {
            // y − v vanishes on D1 + D2; its other two zeros give ι(D3)
            let v = prem(&num, &u);
            let w = psub(f, &pmul(&v, &v));
            if deg(&w) != 6 {
                return None;
            }
            let u2 = pmonic(&pdivrem(&w, &u).0);
            let v2 = prem(&pneg(&v), &u2);
            Some(Div { u: u2, v: v2 })
        }
        _ => None,
    }
}

fn rmul<F: Fld>(f: &[F], a: &Div<F>, mut k: u64) -> Option<Div<F>> {
    let mut r = Div::zero();
    let mut b = a.clone();
    while k > 0 {
        if k & 1 == 1 {
            r = radd(f, &r, &b)?;
        }
        k >>= 1;
        if k > 0 {
            b = radd(f, &b, &b)?;
        }
    }
    Some(r)
}

fn rmul_fast<F: Fld>(f: &[F], a: &Div<F>, mut k: u64) -> Option<Div<F>> {
    let mut r = Div::zero();
    let mut b = a.clone();
    while k > 0 {
        if k & 1 == 1 {
            r = radd_fast(f, &r, &b)?;
        }
        k >>= 1;
        if k > 0 {
            b = radd_fast(f, &b, &b)?;
        }
    }
    Some(r)
}

fn conj_div(d: &Div<Fp2>) -> Div<Fp2> {
    Div {
        u: d.u.iter().map(|c| c.conj()).collect(),
        v: d.v.iter().map(|c| c.conj()).collect(),
    }
}

fn to_fp_div(d: &Div<Fp2>) -> Option<Div<Fp>> {
    let mut u = Vec::new();
    for c in &d.u {
        if c.1 != 0 {
            return None;
        }
        u.push(Fp(c.0));
    }
    let mut v = Vec::new();
    for c in &d.v {
        if c.1 != 0 {
            return None;
        }
        v.push(Fp(c.0));
    }
    Some(Div { u, v })
}

/// Solve a 3×3 linear system over F_{p²} (Gauss); None if singular.
fn solve3(mut m: [[Fp2; 4]; 3]) -> Option<[Fp2; 3]> {
    for col in 0..3 {
        let piv = (col..3).find(|&r| !m[r][col].is_zero())?;
        m.swap(col, piv);
        let inv = m[col][col].inv();
        for j in col..4 {
            m[col][j] = m[col][j].mul(inv);
        }
        for r in 0..3 {
            if r != col && !m[r][col].is_zero() {
                let c = m[r][col];
                for j in col..4 {
                    m[r][j] = m[r][j].sub(c.mul(m[col][j]));
                }
            }
        }
    }
    Some([m[0][3], m[1][3], m[2][3]])
}

#[derive(Clone, Debug)]
struct Glue {
    p: u64,
    e: Ell,
    n: u64,
    h: u64,
    ell: u64,
    s1: Fp2,
    s2: Fp2,
    /// Möbius x ↦ W = (a x + b)/(c x + d) from the bielliptic model to the F_p model
    mob: [Fp2; 4],
    /// y-scaling: y_F = y · (−cW + a)³ · yscale
    yscale: Fp2,
    /// the F_p model y² = hf(W), deg 6
    hf: Vec<Fp>,
    /// diagnostics: t2 = σ(s2), λ² ∈ F_p, λ ∈ F_p, the model is F_p-rational, twist used
    diag: [bool; 4],
    twist: bool,
}

impl Glue {
    /// One attempt at a glued instance with exact cofactor `want_h` over a field already set.
    fn try_new(p: u64, want_h: u64, rng: &mut u64, twist: bool) -> Option<Glue> {
        let a = Fp2(xorshift(rng) % p, xorshift(rng) % p);
        let b = Fp2(xorshift(rng) % p, xorshift(rng) % p);
        if a.is_zero() || b.is_zero() {
            return None;
        }
        // j ∉ F_p  <=>  a³/b² ∉ F_p
        let jr = a.sqr().mul(a).mul(b.sqr().inv());
        if jr.1 == 0 {
            return None;
        }
        let disc = a
            .sqr()
            .mul(a)
            .mul(Fp2::from_u64(4))
            .add(b.sqr().mul(Fp2::from_u64(27)));
        if disc.is_zero() {
            return None;
        }
        let e = Ell { a, b };
        let n = ec_order(&e, p, rng)?;
        if n % want_h != 0 || !is_prime(n / want_h) || n / want_h <= 8 {
            return None;
        }
        let g = vec![b, a, Fp2::zero(), Fp2::one()];
        // w = z^{p³} in F_{p²}[z]/(g): the gluing ψ(a) = a^{p³} satisfies the
        // descent cocycle ψ^σ ∘ ψ = id on the roots of an irreducible or split g
        // (a^{p⁶} = a); with exactly one rational root no Galois-equivariant ψ does
        let mut w = vec![Fp2::zero(), Fp2::one()];
        for _ in 0..3 {
            let mut acc = vec![Fp2::one()];
            let mut base = w.clone();
            let mut ex = p;
            while ex > 0 {
                if ex & 1 == 1 {
                    acc = prem(&pmul(&acc, &base), &g);
                }
                ex >>= 1;
                if ex > 0 {
                    base = prem(&pmul(&base, &base), &g);
                }
            }
            w = acc;
        }
        let zw = prem(&pmul(&[Fp2::zero(), Fp2::one()], &w), &g);
        let co = |v: &Vec<Fp2>, i: usize| v.get(i).copied().unwrap_or(Fp2::zero());
        let z = vec![Fp2::zero(), Fp2::one()];
        let mut m = [[Fp2::zero(); 4]; 3];
        for j in 0..3 {
            m[j] = [
                co(&zw, j),
                co(&z, j),
                co(&w, j),
                if j == 0 { Fp2::one() } else { Fp2::zero() },
            ];
        }
        let [r, s, t] = solve3(m)?;
        if r.is_zero() {
            return None;
        }
        let ri = r.inv();
        let s2 = t.mul(ri).neg();
        let t2 = s.mul(ri).neg();
        let k = s2.mul(t2).add(ri);
        let gs2 = s2.sqr().mul(s2).add(a.mul(s2)).add(b);
        let s1 = k.mul(gs2);
        if s1.is_zero() {
            return None;
        }
        // bielliptic sextic h(x) = g(s1 x² + s2)/s1²
        let q = vec![s2, Fp2::zero(), s1];
        let gq = padd(&padd(&pmul(&pmul(&q, &q), &q), &pscale(&q, a)), &[b]);
        let hb = pscale(&gq, s1.sqr().inv());
        // descent datum x ↦ λ/x^σ with λ² = k/N(s1)
        let ns1 = s1.mul(s1.conj());
        let lam2 = k.mul(ns1.inv());
        let d_t2 = t2 == s2.conj();
        let d_lam2 = lam2.1 == 0;
        if !d_lam2 {
            return None;
        }
        let lam = Fp(lam2.0);
        let lsq = sqrt_p(lam.0);
        let d_lam = lsq.is_some();
        let lam = Fp(lsq?);
        // three points on the Baer subline N(x) = λ
        let mut cs: Vec<Fp2> = Vec::new();
        while cs.len() < 3 {
            let u = Fp2(xorshift(rng) % p, xorshift(rng) % p);
            if u.is_zero() {
                continue;
            }
            let nu = u.mul(u.conj()).0;
            if let Some(r) = sqrt_p(lam.0 * inv_mod(nu, p) % p) {
                let c = u.mul_fp(Fp(r));
                if !cs.contains(&c) {
                    cs.push(c);
                }
            }
        }
        let (c1, c2, c3) = (cs[0], cs[1], cs[2]);
        let (ma, mb, mc, md) = (
            c2.sub(c3),
            c1.mul(c2.sub(c3)).neg(),
            c2.sub(c1),
            c3.mul(c2.sub(c1)).neg(),
        );
        // pick w0 ∈ F_p whose point lies over a square of h, then W = 1/(M(x) − w0)
        for _ in 0..64 {
            let w0 = Fp2(xorshift(rng) % p, 0);
            let (na, nb, nc, nd) = (mc, md, ma.sub(w0.mul(mc)), mb.sub(w0.mul(md)));
            // x = (d W − b)/(−c W + a)
            let num = vec![nb.neg(), nd];
            let den = vec![na, nc.neg()];
            let mut hraw: Vec<Fp2> = Vec::new();
            let mut npow = vec![Fp2::one()];
            for j in 0..7 {
                let mut term = pscale(&npow, co(&hb, j));
                for _ in j..6 {
                    term = pmul(&term, &den);
                }
                hraw = padd(&hraw, &term);
                npow = pmul(&npow, &num);
            }
            if hraw.len() != 7 {
                continue;
            }
            let mu = hraw[6];
            let Some(smu) = mu.sqrt() else { continue };
            let hf2 = pscale(&hraw, mu.inv());
            if hf2.iter().any(|c| c.1 != 0) {
                return None; // the x-map did not descend: recorded as a failed G0
            }
            let mut hf: Vec<Fp> = hf2.iter().map(|c| Fp(c.0)).collect();
            let mut yscale = smu.inv();
            if twist {
                // quadratic twist over F_p: y_tw = y·sqrt(ν), ν = κ
                let nu = Fp(kap());
                hf = pscale(&hf, nu);
                yscale = yscale.mul(Fp2::s());
            }
            return Some(Glue {
                p,
                e,
                n,
                h: want_h,
                ell: n / want_h,
                s1,
                s2,
                mob: [na, nb, nc, nd],
                yscale,
                hf,
                diag: [d_t2, d_lam2, d_lam, true],
                twist,
            });
        }
        None
    }

    fn new(bits: u32, want_h: u64, rng: &mut u64) -> Glue {
        loop {
            let base = 1u64 << (bits / 2);
            let p = loop {
                let c = base + xorshift(rng) % (base / 8).max(8);
                if c > 5 && is_prime(c) {
                    break c;
                }
            };
            let kappa = (2..p)
                .find(|&k| powmod(k, (p - 1) / 2, p) == p - 1)
                .unwrap();
            set_field(p, kappa);
            for _ in 0..4000 {
                if let Some(g0) = Glue::try_new(p, want_h, rng, false) {
                    // choose the F_p form on which the trace is nonzero on the ℓ-part
                    let pt = loop {
                        let v = g0.e.mul(g0.e.random_point(rng), g0.h);
                        if v != EP::O {
                            break v;
                        }
                    };
                    for tw in [false, true] {
                        let mut g1 = g0.clone();
                        if tw {
                            g1.hf = pscale(&g1.hf, Fp(kap()));
                            g1.yscale = g1.yscale.mul(Fp2::s());
                            g1.twist = true;
                        }
                        if let Some(t) = g1.transfer(pt) {
                            if !t.is_zero() && rmul(&g1.hf, &t, g1.ell).is_some_and(|z| z.is_zero())
                            {
                                return g1;
                            }
                        }
                    }
                }
            }
        }
    }

    /// φ₁^*P as a degree-2 divisor on the F_p model, over F_{p²}.
    fn pullback(&self, pt: EP) -> Option<Div<Fp2>> {
        let EP::A(x, y) = pt else { return None };
        let [a, b, c, d] = self.mob;
        let r0 = x.sub(self.s2).mul(self.s1.inv());
        let num = vec![b.neg(), d];
        let den = vec![a, c.neg()];
        let u = psub(&pmul(&num, &num), &pscale(&pmul(&den, &den), r0));
        if u.len() != 3 {
            return None;
        }
        let u = pmonic(&u);
        let yv = y.mul(self.s1.inv()).mul(self.yscale);
        let v = prem(&pscale(&pmul(&pmul(&den, &den), &den), yv), &u);
        Some(Div { u, v })
    }

    /// τ(P) = [φ₁^*P − D∞] + its conjugate, in Jac(C_F)(F_p).
    fn transfer(&self, pt: EP) -> Option<Div<Fp>> {
        let d = self.pullback(pt)?;
        let f2 = lift(&self.hf);
        if !jvalid(&f2, &d) {
            return None;
        }
        let t = radd(&f2, &d, &conj_div(&d))?;
        to_fp_div(&t)
    }

    fn as_instance(&self) -> Instance {
        Instance {
            p: self.p,
            kappa: kap(),
            f: Vec::new(),
            r: Fp(0),
            big_l: Fp(0),
            fq: Vec::new(),
            u0: Fp2::zero(),
            h0: Fp2::zero(),
            c2: Fp2::zero(),
            e: self.e,
            n: self.n,
            h: self.h,
            ell: self.ell,
        }
    }

    /// #C_F(F_p) on the sextic model (points at infinity: 2 if lc is a square)
    fn count_c(&self) -> u64 {
        let mut c = 0u64;
        for x in 0..self.p {
            let v = peval(&self.hf, Fp(x)).0;
            c += if v == 0 {
                1
            } else if sqrt_p(v).is_some() {
                2
            } else {
                0
            };
        }
        c + if sqrt_p(self.hf[6].0).is_some() { 2 } else { 0 }
    }
}

/// Arm J on the glued sextic model: first-cycle large-prime IC with columns
/// x_P = [2P − D∞] (so [P + Q − D∞] = (x_P + x_Q)/2 and x_{ιP} = −x_P).
fn arm_glue(g: &Glue, pt: EP, q: EP, rng: &mut u64) -> ArmResult {
    let t0 = Instant::now();
    let c0 = Counts::now();
    let p = g.p;
    let ell = g.ell;
    let f = &g.hf;
    let inv2 = (ell + 1) / 2;
    let mut res = ArmResult {
        arm: "Jglue".into(),
        c: 0,
        ..Default::default()
    };
    let mut jac = Counts::default();
    let s = Counts::now();
    let (tp, tq) = loop {
        if let (Some(a), Some(b)) = (g.transfer(pt), g.transfer(q)) {
            break (a, b);
        }
    };
    jac = jac.add(s.since());
    let mut lin = Lin::new(ell, 0);
    let mut mults = Vec::new();
    while mults.len() < IC_R {
        let (a, b) = (xorshift(rng) % ell, xorshift(rng) % ell);
        let s = Counts::now();
        let d = rmul_fast(f, &tp, a)
            .and_then(|x| rmul_fast(f, &tq, b).and_then(|y| radd_fast(f, &x, &y)));
        jac = jac.add(s.since());
        if let Some(d) = d {
            mults.push((d, a, b));
        }
    }
    let half = Fp((p + 1) / 2);
    let mut steps = 0u64;
    'walk: loop {
        let (mut a, mut b) = (xorshift(rng) % ell, xorshift(rng) % ell);
        let s = Counts::now();
        let d0 = rmul_fast(f, &tp, a)
            .and_then(|x| rmul_fast(f, &tq, b).and_then(|y| radd_fast(f, &x, &y)));
        jac = jac.add(s.since());
        let Some(mut d) = d0 else { continue };
        for _ in 0..IC_RESTART {
            if d.u.len() == 3 {
                let (u0, u1) = (d.u[0], d.u[1]);
                let disc = u1.sqr().sub(u0.mul(Fp(4)));
                if let Some(sq) = sqrt_p(disc.0) {
                    let v0 = d.v.first().copied().unwrap_or(Fp(0));
                    let v1 = d.v.get(1).copied().unwrap_or(Fp(0));
                    let mut row = Row {
                        a,
                        b,
                        ..Default::default()
                    };
                    let roots = if sq == 0 {
                        vec![Fp(sq)]
                    } else {
                        vec![Fp(sq), Fp(sq).neg()]
                    };
                    let mult = if sq == 0 { 2 } else { 1 };
                    for sgn in roots {
                        cnt_n(1);
                        let xi = u1.neg().add(sgn).mul(half);
                        let w = v1.mul(xi).add(v0).0;
                        let pos = w <= p - w;
                        let coef = mm(inv2, mult, ell);
                        add_entry(
                            &mut row.lp,
                            xi.0,
                            if pos { coef } else { (ell - coef) % ell },
                            ell,
                        );
                    }
                    res.rels += 1;
                    let s = Counts::now();
                    let cand = lin.push(row);
                    res.la = res.la.add(s.since());
                    if let Some(k) = cand.filter(|&k| g.e.mul(pt, k) == q) {
                        res.k = k;
                        res.ok = true;
                        res.steps = steps;
                        res.jac = jac;
                        res.jac_ops = jac.j;
                        res.mem = lin.pivots.len() as u64;
                        res.walk = c0.since().minus(res.la);
                        res.secs = t0.elapsed().as_secs_f64();
                        return res;
                    }
                }
            }
            cnt_n(1);
            let hx =
                d.u.iter()
                    .chain(d.v.iter())
                    .fold(0u64, |h, c| (h ^ c.0).wrapping_mul(0x9e37_79b9_7f4a_7c15))
                    >> 40;
            let (m, ma, mb) = &mults[(hx as usize) % IC_R];
            let s = Counts::now();
            let nd = radd_fast(f, &d, m);
            jac = jac.add(s.since());
            steps += 1;
            match nd {
                Some(x) => d = x,
                None => continue 'walk,
            }
            a = (a + ma) % ell;
            b = (b + mb) % ell;
        }
    }
}

fn glue_checks(out: &Path) {
    let mut rng = 0x6c75_6500_u64;
    let mut log = String::new();
    for &bits in &[14u32, 16, 20, 24] {
        for &h in &[1u64, 4] {
            for _ in 0..2 {
                let g = Glue::new(bits, h, &mut rng);
                let f = &g.hf;
                let t = g.p * g.p + 1 - g.n;
                let nc = g.count_c();
                // #C(F_{p²}) on the F_p model, brute force for small p
                let nc2 = if g.p < 400 {
                    let f2 = lift(f);
                    let mut c = 0u64;
                    for x0 in 0..g.p {
                        for x1 in 0..g.p {
                            let v = peval(&f2, Fp2(x0, x1));
                            c += if v.is_zero() {
                                1
                            } else if v.sqrt().is_some() {
                                2
                            } else {
                                0
                            };
                        }
                    }
                    let lc = Fp2::from_fp(f[6]);
                    c += if lc.sqrt().is_some() { 2 } else { 0 };
                    Some(c)
                } else {
                    None
                };
                // G2/G3 on transfers of random ℓ-part points
                let mut pts = Vec::new();
                while pts.len() < 3 {
                    let v = g.e.mul(g.e.random_point(&mut rng), g.h);
                    if v != EP::O {
                        pts.push(v);
                    }
                }
                let ts: Vec<Option<Div<Fp>>> = pts.iter().map(|&x| g.transfer(x)).collect();
                let tsum = g.transfer(g.e.add(pts[0], pts[1]));
                // tri-state: true / false / "degenerate" (a point at infinity met on the way)
                let tri = |o: Option<bool>| match o {
                    Some(true) => "true",
                    Some(false) => "FALSE",
                    None => "degenerate",
                };
                let (hom, assoc, inv, ordl, nz, ordn) = match (&ts[0], &ts[1], &ts[2], &tsum) {
                    (Some(a), Some(b), Some(c), Some(sab)) => {
                        let ab = radd(f, a, b);
                        let hom = ab.as_ref().map(|x| x == sab);
                        let l = ab.clone().and_then(|ab| radd(f, &ab, c));
                        let r = radd(f, b, c).and_then(|bc| radd(f, a, &bc));
                        let assoc = l.and_then(|l| r.map(|r| l == r));
                        let inv = radd(f, a, &a.neg()).map(|z| z.is_zero());
                        let ordl = rmul(f, a, g.ell).map(|z| z.is_zero());
                        let ordn = rmul(f, c, g.n).map(|z| z.is_zero());
                        (
                            tri(hom),
                            tri(assoc),
                            tri(inv),
                            tri(ordl),
                            tri(Some(!a.is_zero())),
                            tri(ordn),
                        )
                    }
                    _ => (
                        "degenerate",
                        "degenerate",
                        "degenerate",
                        "degenerate",
                        "degenerate",
                        "degenerate",
                    ),
                };
                log.push_str(&format!(
                    "G p={} N={} h={} ell={} t={}: t2=sigma(s2) {} lambda^2 in F_p {} lambda in F_p {} | twist {} | G1 #C(F_p)={} (p+1={}) #C(F_p2)={} (p^2+1-2t={}) | G2 assoc {} inverse {} N*A=0 {} | G3 hom {} nonzero {} ell*tau=0 {}\n",
                    g.p, g.n, g.h, g.ell, t as i64 - 0, g.diag[0], g.diag[1], g.diag[2], g.twist, nc, g.p + 1,
                    nc2.map(|c| c.to_string()).unwrap_or("-".into()),
                    (g.p * g.p + 1) as i64 - 2 * (t as i64), assoc, inv, ordn, hom, nz, ordl
                ));
            }
        }
    }
    // G5: reduced real-model formulas against the generic real-model law, and per-op cost
    for &bits in &[20u32, 28, 36] {
        let g = Glue::new(bits, 1, &mut rng);
        let f = &g.hf;
        let mut ds: Vec<Div<Fp>> = Vec::new();
        while ds.len() < 401 {
            let v = g.e.mul(g.e.random_point(&mut rng), g.h);
            if let Some(t) = g.transfer(v) {
                ds.push(t);
            }
        }
        let (mut agree, mut cmp, mut degen) = (0u64, 0u64, 0u64);
        let (mut cf, mut cg, mut df, mut dg) = (
            Counts::default(),
            Counts::default(),
            Counts::default(),
            Counts::default(),
        );
        for w in ds.windows(2) {
            for (x, y, is_dbl) in [(&w[0], &w[1], false), (&w[0], &w[0], true)] {
                let s0 = Counts::now();
                let a = radd_fast(f, x, y);
                let c1 = s0.since();
                let s0 = Counts::now();
                let b = radd(f, x, y);
                let c2 = s0.since();
                match (a, b) {
                    (Some(a), Some(b)) => {
                        cmp += 1;
                        agree += u64::from(a == b);
                    }
                    _ => degen += 1,
                }
                if is_dbl {
                    df = df.add(c1);
                    dg = dg.add(c2);
                } else {
                    cf = cf.add(c1);
                    cg = cg.add(c2);
                }
            }
        }
        let k = 400.0;
        log.push_str(&format!(
            "G5 p={}: fast = generic on {agree}/{cmp} pairs ({degen} degenerate) | add fast {:.1}M+{:.2}I generic {:.1}M+{:.2}I | double fast {:.1}M+{:.2}I generic {:.1}M+{:.2}I\n",
            g.p, cf.m as f64 / k, cf.i as f64 / k, cg.m as f64 / k, cg.i as f64 / k,
            df.m as f64 / k, df.i as f64 / k, dg.m as f64 / k, dg.i as f64 / k
        ));
    }
    std::fs::write(out.join("glue_checks.txt"), &log).unwrap();
    eprint!("{log}");
}

fn glue_job(job: &HJob) -> Vec<String> {
    let mut rng = job.seed | 1;
    let g = Glue::new(job.bits, job.h, &mut rng);
    let inst = g.as_instance();
    let pt = loop {
        let v = g.e.mul(g.e.random_point(&mut rng), g.h);
        if v != EP::O {
            break v;
        }
    };
    let (pa, pb) = (g.e.random_point(&mut rng), g.e.random_point(&mut rng));
    let s = Counts::now();
    let _ = g.e.add(pa, pb);
    let eadd = s.since();
    let mut rows = Vec::new();
    for tgt in 0..TARGETS {
        let k = 1 + xorshift(&mut rng) % (g.ell - 1);
        let q = g.e.mul(pt, k);
        for (name, r) in [
            ("R", arm_rho(&inst, pt, q, &mut rng)),
            ("Jglue", arm_glue(&g, pt, q, &mut rng)),
        ] {
            let ok = r.ok && r.k == k && g.e.mul(pt, r.k) == q;
            rows.push(format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{:.3}",
                job.bits,
                g.h,
                job.inst_id,
                g.p,
                g.n,
                g.ell,
                u8::from(g.twist),
                tgt,
                name,
                r.steps,
                r.rels,
                r.jac_ops,
                r.walk.m,
                r.walk.i,
                r.walk.n,
                r.jac.m,
                r.jac.i,
                r.la.n,
                eadd.m,
                eadd.i,
                u8::from(ok),
                r.secs
            ));
        }
    }
    eprintln!(
        "glue bits {} h {} inst {} done",
        job.bits, job.h, job.inst_id
    );
    rows
}

fn summarize_glue(csv: &str) -> String {
    use std::collections::BTreeMap;
    let mut cells: BTreeMap<(u32, u64, String), Vec<(f64, f64)>> = BTreeMap::new();
    let mut all_ok = true;
    let mut per_op: BTreeMap<String, (f64, f64, f64)> = BTreeMap::new();
    for l in csv.lines().skip(1) {
        let v: Vec<&str> = l.split(',').collect();
        let bits: u32 = v[0].parse().unwrap();
        let h: u64 = v[1].parse().unwrap();
        let p: f64 = v[3].parse().unwrap();
        let arm = v[8].to_string();
        let steps: f64 = v[9].parse().unwrap();
        let jops: f64 = v[11].parse().unwrap();
        let (wm, wi): (f64, f64) = (v[12].parse().unwrap(), v[13].parse().unwrap());
        let (jm, ji): (f64, f64) = (v[15].parse().unwrap(), v[16].parse().unwrap());
        all_ok &= v[20] == "1";
        // cost at I = 3M, native 0: total and per step
        let cost = wm + 3.0 * wi;
        cells
            .entry((bits, h, arm.clone()))
            .or_default()
            .push((steps / p, cost));
        if arm == "Jglue" && jops > 0.0 {
            let e = per_op.entry(arm).or_insert((0.0, 0.0, 0.0));
            e.0 += jm;
            e.1 += ji;
            e.2 += jops;
        }
    }
    let mut out = String::new();
    for ((bits, h, arm), v) in &cells {
        let n = v.len() as f64;
        let m = v.iter().map(|x| x.0).sum::<f64>() / n;
        let sd = (v.iter().map(|x| (x.0 - m).powi(2)).sum::<f64>() / (n - 1.0).max(1.0)).sqrt();
        let c = v.iter().map(|x| x.1).sum::<f64>() / n;
        out.push_str(&format!(
            "H2 bits {bits} h {h} {arm}: steps/p {m:.4} (s.e. {:.4}, {} runs) | mean cost (I=3M, w=0) {c:.4e} | rho theory {:.4}\n",
            sd / n.sqrt(),
            v.len(),
            0.886 / (*h as f64).sqrt()
        ));
    }
    for ((bits, h, arm), v) in &cells {
        if arm != "Jglue" {
            continue;
        }
        if let Some(r) = cells.get(&(*bits, *h, "R".to_string())) {
            let c = v.iter().map(|x| x.1).sum::<f64>() / v.len() as f64;
            let cr = r.iter().map(|x| x.1).sum::<f64>() / r.len() as f64;
            out.push_str(&format!(
                "H2 bits {bits} h {h}: Jglue/rho cost ratio (measured, I=3M, w=0) {:.3}\n",
                c / cr
            ));
        }
    }
    for (arm, (m, i, j)) in &per_op {
        out.push_str(&format!(
            "H2 {arm}: measured per Jacobian op {:.1}M + {:.2}I\n",
            m / j,
            i / j
        ));
    }
    // Full pricing band (Amendment C, with H3's honest split test: one
    // Legendre symbol per Jglue step priced at one inversion).
    let mut rows: BTreeMap<(u32, u64, String), Vec<Vec<f64>>> = BTreeMap::new();
    for l in csv.lines().skip(1) {
        let v: Vec<&str> = l.split(',').collect();
        let f: Vec<f64> = v
            .iter()
            .map(|x| x.parse::<f64>().unwrap_or(f64::NAN))
            .collect();
        rows.entry((
            v[0].parse().unwrap(),
            v[1].parse().unwrap(),
            v[8].to_string(),
        ))
        .or_default()
        .push(f);
    }
    // columns: 3 p, 9 steps, 11 jac_ops, 12 walk_M, 13 walk_I, 14 walk_native,
    // 15 jac_M, 16 jac_I, 17 la_native, 18 eadd_M, 19 eadd_I
    let cost = |r: &Vec<f64>, ipr: f64, eadd_price: bool, w: f64, legendre: bool| -> f64 {
        let mut c = r[12] + ipr * r[13] + w * (r[14] + r[17]);
        if eadd_price {
            c -= r[15] + ipr * r[16];
            c += r[11] * (r[18] + ipr * r[19]);
        }
        if legendre {
            c += r[9] * ipr;
        }
        c
    };
    for ((bits, h, arm), v) in &rows {
        if arm != "Jglue" {
            continue;
        }
        let Some(rr) = rows.get(&(*bits, *h, "R".to_string())) else {
            continue;
        };
        let (mut lo, mut hi, mut icw, mut cells) = (f64::MAX, 0f64, 0, 0);
        for ik in 0..2 {
            for jp in [false, true] {
                for w in [0.0, 0.1, 1.0] {
                    let ipr = |r: &Vec<f64>| if ik == 0 { 3.0 } else { 2.0 * r[3].log2() };
                    let ci = v.iter().map(|r| cost(r, ipr(r), jp, w, true)).sum::<f64>()
                        / v.len() as f64;
                    let cr = rr
                        .iter()
                        .map(|r| cost(r, ipr(r), false, w, false))
                        .sum::<f64>()
                        / rr.len() as f64;
                    let ratio = ci / cr;
                    lo = lo.min(ratio);
                    hi = hi.max(ratio);
                    cells += 1;
                    icw += usize::from(ratio < 1.0);
                }
            }
        }
        let verdict = if icw == cells {
            "IC cheaper in every cell"
        } else if icw == 0 {
            "rho cheaper in every cell"
        } else {
            "unresolved"
        };
        out.push_str(&format!(
            "H2 band bits {bits} h {h}: Jglue/rho in [{lo:.3}, {hi:.3}] over {cells} cells (IC cheaper in {icw}) -> {verdict}\n"
        ));
    }
    out.push_str(&format!("H2 all answers verified: {all_ok}\n"));
    out
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out = Path::new(args.get(1).expect("usage: n2_routes <out-dir> [run|check]"));
    std::fs::create_dir_all(out).unwrap();
    let mode = args.get(2).map(String::as_str).unwrap_or("run");
    if mode == "check" {
        run_checks(out);
        return;
    }
    if mode == "summarize" {
        // re-derive the summary files from a recorded routes.csv
        let summary = summarize(&std::fs::read_to_string(out.join("routes.csv")).unwrap());
        std::fs::write(out.join("routes_summary.csv"), &summary.0).unwrap();
        std::fs::write(out.join("ranking.csv"), &summary.1).unwrap();
        std::fs::write(out.join("summary.txt"), &summary.2).unwrap();
        eprint!("{}", summary.2);
        return;
    }
    if mode == "gluesummarize" {
        let t = summarize_glue(&std::fs::read_to_string(out.join("glue.csv")).unwrap());
        std::fs::write(out.join("glue_summary.txt"), &t).unwrap();
        eprint!("{t}");
        return;
    }
    if mode == "gluecheck" {
        glue_checks(out);
        return;
    }
    if mode == "glue" {
        let threads: usize = std::env::var("N2_THREADS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(4);
        let env_list = |k: &str, d: &str| -> Vec<u64> {
            std::env::var(k)
                .unwrap_or_else(|_| d.into())
                .split(',')
                .map(|x| x.trim().parse().unwrap())
                .collect()
        };
        let mut jobs = Vec::new();
        for &bits in &env_list("N2_SIZES", "28,32,36,40") {
            for &h in &env_list("N2_HS", "1,4") {
                for inst_id in 0..4 {
                    jobs.push(HJob {
                        bits: bits as u32,
                        h,
                        inst_id,
                        seed: splitmix(0x6c75_0000 ^ (bits << 40) ^ (h << 8) ^ inst_id as u64),
                        theta: 1.0,
                    });
                }
            }
        }
        jobs.sort_by_key(|j| (j.bits, j.h));
        let t0 = Instant::now();
        let queue = std::sync::Mutex::new(jobs);
        let results = std::sync::Mutex::new(Vec::<String>::new());
        std::thread::scope(|s| {
            for _ in 0..threads {
                s.spawn(|| loop {
                    let j = queue.lock().unwrap().pop();
                    let Some(job) = j else { break };
                    let r = glue_job(&job);
                    results.lock().unwrap().extend(r);
                });
            }
        });
        let mut rows = results.into_inner().unwrap();
        rows.sort();
        let mut csv = String::from("bits,h,instance,p,N,ell,twist,target,arm,steps,relations,jac_ops,walk_M,walk_I,walk_native,jac_M,jac_I,la_native,eadd_M,eadd_I,verified,seconds\n");
        for r in &rows {
            csv.push_str(r);
            csv.push('\n');
        }
        std::fs::write(out.join("glue.csv"), &csv).unwrap();
        let t = summarize_glue(&csv);
        std::fs::write(out.join("glue_summary.txt"), &t).unwrap();
        eprint!("{t}");
        eprintln!("done in {:.1}s", t0.elapsed().as_secs_f64());
        return;
    }
    if mode == "hsweep" || mode == "memsweep" || mode == "hsummarize" {
        if mode == "hsummarize" {
            let t = summarize_hsweep(&std::fs::read_to_string(out.join("hsweep.csv")).unwrap());
            std::fs::write(out.join("hsweep_summary.txt"), &t).unwrap();
            eprint!("{t}");
            return;
        }
        let threads: usize = std::env::var("N2_THREADS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(4);
        let env_list = |k: &str, d: &str| -> Vec<u64> {
            std::env::var(k)
                .unwrap_or_else(|_| d.into())
                .split(',')
                .map(|x| x.trim().parse().unwrap())
                .collect()
        };
        let mut jobs = Vec::new();
        if mode == "hsweep" {
            let sizes = env_list("N2_SIZES", "32,36,40");
            let hs = env_list("N2_HS", "4,8,12,16,24,32,48,64,96,128");
            for &bits in &sizes {
                for &h in &hs {
                    for inst_id in 0..4 {
                        jobs.push(HJob {
                            bits: bits as u32,
                            h,
                            inst_id,
                            seed: splitmix(0x4831_0000 ^ (bits << 40) ^ (h << 8) ^ inst_id as u64),
                            theta: 1.0,
                        });
                    }
                }
            }
        } else {
            let sizes = env_list("N2_SIZES", "32,36");
            for &bits in &sizes {
                for theta in [0.5f64, 0.25, 0.125] {
                    for inst_id in 0..4 {
                        jobs.push(HJob {
                            bits: bits as u32,
                            h: 4,
                            inst_id,
                            // same instances as the h = 4 cells of the h-sweep
                            seed: splitmix(0x4831_0000 ^ (bits << 40) ^ (4 << 8) ^ inst_id as u64),
                            theta,
                        });
                    }
                }
            }
        }
        let t0 = Instant::now();
        // big jobs first
        jobs.sort_by_key(|j| (j.bits, j.h));
        let queue = std::sync::Mutex::new(jobs);
        let results = std::sync::Mutex::new(Vec::<String>::new());
        std::thread::scope(|s| {
            for _ in 0..threads {
                s.spawn(|| loop {
                    let j = queue.lock().unwrap().pop();
                    let Some(job) = j else { break };
                    let r = hsweep_job(&job);
                    results.lock().unwrap().extend(r);
                });
            }
        });
        let mut rows = results.into_inner().unwrap();
        rows.sort();
        let mut csv = String::from(
            "bits,h,instance,p,ell,cofactor_exponent,target,arm,theta,steps,relations,stored_rows,verified,model_vertices,model_first_cycle_edges,M_total,seconds\n",
        );
        for r in &rows {
            csv.push_str(r);
            csv.push('\n');
        }
        let name = if mode == "hsweep" {
            "hsweep"
        } else {
            "memsweep"
        };
        std::fs::write(out.join(format!("{name}.csv")), &csv).unwrap();
        let t = summarize_hsweep(&csv);
        std::fs::write(out.join(format!("{name}_summary.txt")), &t).unwrap();
        eprint!("{t}");
        eprintln!("done in {:.1}s", t0.elapsed().as_secs_f64());
        return;
    }
    let control = mode == "control";
    let threads: usize = std::env::var("N2_THREADS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(4);
    let sizes: Vec<u32> = std::env::var("N2_SIZES")
        .unwrap_or_else(|_| "20,24,28,32,36,40".into())
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    let mut jobs = Vec::new();
    for &bits in &sizes {
        for inst_id in 0..3 {
            jobs.push(Job {
                bits,
                inst_id,
                seed: splitmix(0x6e32_5eed ^ ((bits as u64) << 32) ^ inst_id as u64),
            });
        }
    }
    let t0 = Instant::now();
    let njobs = jobs.len();
    let queue = std::sync::Mutex::new(jobs.into_iter().enumerate().collect::<Vec<_>>());
    let results = std::sync::Mutex::new(Vec::<(usize, Vec<String>)>::new());
    std::thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| loop {
                let j = queue.lock().unwrap().pop();
                let Some((i, job)) = j else { break };
                let r = if control {
                    control_job(&job)
                } else {
                    run_job(&job)
                };
                results.lock().unwrap().push((i, r));
            });
        }
    });
    let mut res = results.into_inner().unwrap();
    res.sort_by_key(|(i, _)| *i);
    if control {
        let mut csv = String::from("bits,instance,p,h,ell,target,arm,steps,relations,repeat_events,solved_by_repeat,verified\n");
        for (_, rows) in res {
            for r in rows {
                csv.push_str(&r);
                csv.push('\n');
            }
        }
        std::fs::write(out.join("control.csv"), &csv).unwrap();
        let t = summarize_control(&csv);
        std::fs::write(out.join("control_summary.txt"), &t).unwrap();
        eprint!("{t}");
        eprintln!("done in {:.1}s", t0.elapsed().as_secs_f64());
        return;
    }
    let mut f = File::create(out.join("routes.csv")).unwrap();
    writeln!(
        f,
        "bits,instance,p,N,h,ell,target,arm,c,factor_base,steps,relations,b_relations,jac_ops,walk_M,walk_I,walk_native,jac_M,jac_I,la_M,la_I,la_native,eadd_M,eadd_I,verified,seconds"
    )
    .unwrap();
    for (_, rows) in res {
        for r in rows {
            writeln!(f, "{r}").unwrap();
        }
    }
    let summary = summarize(&std::fs::read_to_string(out.join("routes.csv")).unwrap());
    std::fs::write(out.join("routes_summary.csv"), &summary.0).unwrap();
    std::fs::write(out.join("ranking.csv"), &summary.1).unwrap();
    std::fs::write(out.join("summary.txt"), &summary.2).unwrap();
    eprint!("{}", summary.2);
    let mut log = File::create(out.join("tool_log.txt")).unwrap();
    writeln!(log, "jobs {njobs} total {:.1}s", t0.elapsed().as_secs_f64()).unwrap();
    eprintln!("done in {:.1}s", t0.elapsed().as_secs_f64());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fp2_and_sqrt() {
        set_field(103, 5);
        let mut rng = 5u64;
        for _ in 0..200 {
            let a = Fp2(xorshift(&mut rng) % 103, xorshift(&mut rng) % 103);
            if let Some(r) = a.sqrt() {
                assert_eq!(r.sqr_raw(), a);
            }
            if !a.is_zero() {
                assert_eq!(a.mul(a.inv()), Fp2::one());
            }
        }
    }

    #[test]
    fn instance_transfer_and_orders() {
        let mut rng = 77u64;
        let inst = Instance::new(16, &mut rng);
        assert_eq!(inst.count_c(), inst.p + 1);
        let pt = inst.e.mul(inst.e.random_point(&mut rng), inst.h);
        let pt2 = inst.e.mul(inst.e.random_point(&mut rng), inst.h);
        assert_eq!(inst.e.mul(pt, inst.ell), EP::O);
        let a = inst.transfer(pt).unwrap();
        let b = inst.transfer(pt2).unwrap();
        let s = inst.transfer(inst.e.add(pt, pt2)).unwrap();
        assert!(jvalid(&inst.fq, &a));
        assert_eq!(jadd(&inst.fq, &a, &b), s);
        assert!(jmul(&inst.fq, &a, inst.ell).is_zero());
        assert!(!a.is_zero());
    }

    #[test]
    fn fast_formulas_match_cantor() {
        let mut rng = 4242u64;
        let inst = Instance::new(20, &mut rng);
        let f = &inst.fq;
        let rand_div = |rng: &mut u64| {
            let mut d = Div::zero();
            for _ in 0..2 {
                loop {
                    let x = Fp(xorshift(rng) % inst.p);
                    if let Some(w) = sqrt_p(peval(f, x).0) {
                        d = jadd(
                            f,
                            &d,
                            &Div {
                                u: vec![x.neg(), Fp(1)],
                                v: trim(vec![Fp(w)]),
                            },
                        );
                        break;
                    }
                }
            }
            d
        };
        let mut fast_hits = 0;
        for _ in 0..3000 {
            let a = rand_div(&mut rng);
            let b = rand_div(&mut rng);
            let s0 = Counts::now();
            let x = jadd_fast(f, &a, &b);
            if s0.since().m < 60 {
                fast_hits += 1;
            }
            assert_eq!(x, jadd(f, &a, &b));
            assert_eq!(jadd_fast(f, &a, &a), jadd(f, &a, &a));
        }
        assert!(fast_hits > 2000);
        let a = rand_div(&mut rng);
        assert_eq!(jmul_fast(f, &a, 123457), jmul(f, &a, 123457));
    }

    #[test]
    fn oracle_matches_brute_force() {
        let mut rng = 91u64;
        let inst = loop {
            let i = Instance::new(12, &mut rng);
            if i.p <= 101 {
                break i;
            }
        };
        let br = brute_decomp(&inst.e, inst.p);
        let p = inst.p;
        for x0 in 0..p {
            for x1 in 0..p {
                let x3 = Fp2(x0, x1);
                if inst.e.rhs(x3).sqrt().is_none() {
                    continue;
                }
                let mut got = decompose_x(&inst.e, x3, &mut rng);
                got.retain(|(a, b)| {
                    inst.e.rhs(Fp2::from_fp(*a)).sqrt().is_some()
                        && inst.e.rhs(Fp2::from_fp(*b)).sqrt().is_some()
                });
                got.sort_by_key(|(a, b)| (a.0, b.0));
                let mut want = br.get(&(x0, x1)).cloned().unwrap_or_default();
                want.sort_by_key(|(a, b)| (a.0, b.0));
                assert_eq!(got, want, "x3 = {x3:?}");
            }
        }
    }

    #[test]
    fn small_end_to_end() {
        let mut rng = 1234u64;
        let inst = Instance::new(16, &mut rng);
        let pt = loop {
            let v = inst.e.mul(inst.e.random_point(&mut rng), inst.h);
            if v != EP::O {
                break v;
            }
        };
        let k = 1 + xorshift(&mut rng) % (inst.ell - 1);
        let q = inst.e.mul(pt, k);
        let r = arm_rho(&inst, pt, q, &mut rng);
        assert!(r.ok && r.k == k);
        let j = arm_jac(&inst, pt, q, 2, &mut rng);
        assert!(j.ok && j.k == k, "{j:?}");
        let d = arm_dec(&inst, pt, q, j.nb, 2, &mut rng);
        assert!(d.ok && d.k == k, "{d:?}");
    }
}
