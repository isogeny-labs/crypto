//! Polynomials over a base field [`Fq`] (coefficient `i` is `x^i`), the
//! extension ring `F_q[y]/(g)`, distinct- and equal-degree factorisation in
//! both characteristics, and Hasse derivatives.

use rand::rngs::SmallRng;

use super::field::{Fq, Ring};

pub type Poly = Vec<u64>;

pub fn trim(mut a: Poly) -> Poly {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}

pub fn degree(a: &Poly) -> Option<usize> {
    if a.is_empty() {
        None
    } else {
        Some(a.len() - 1)
    }
}

pub fn x_poly() -> Poly {
    vec![0, 1]
}

pub fn add<F: Fq>(f: &F, a: &Poly, b: &Poly) -> Poly {
    let n = a.len().max(b.len());
    trim(
        (0..n)
            .map(|i| f.fadd(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0)))
            .collect(),
    )
}

pub fn sub<F: Fq>(f: &F, a: &Poly, b: &Poly) -> Poly {
    let n = a.len().max(b.len());
    trim(
        (0..n)
            .map(|i| f.fsub(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0)))
            .collect(),
    )
}

pub fn scale<F: Fq>(f: &F, a: &Poly, c: u64) -> Poly {
    trim(a.iter().map(|&v| f.fmul(v, c)).collect())
}

pub fn mul<F: Fq>(f: &F, a: &Poly, b: &Poly) -> Poly {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut r = vec![0u64; a.len() + b.len() - 1];
    for (i, &ai) in a.iter().enumerate() {
        if ai == 0 {
            continue;
        }
        for (j, &bj) in b.iter().enumerate() {
            r[i + j] = f.fadd(r[i + j], f.fmul(ai, bj));
        }
    }
    trim(r)
}

/// `(quotient, remainder)` of `a / b`, `b ≠ 0`.
pub fn divrem<F: Fq>(f: &F, a: &Poly, b: &Poly) -> (Poly, Poly) {
    let b = trim(b.clone());
    let db = degree(&b).expect("division by the zero polynomial");
    let mut r = trim(a.clone());
    if r.len() <= db {
        return (Vec::new(), r);
    }
    let inv = f.finv(b[db]);
    let mut q = vec![0u64; r.len() - db];
    while r.len() > db {
        let d = r.len() - 1;
        let c = f.fmul(r[d], inv);
        q[d - db] = c;
        for i in 0..=db {
            r[d - db + i] = f.fsub(r[d - db + i], f.fmul(c, b[i]));
        }
        r = trim(r);
        if r.is_empty() {
            break;
        }
    }
    (trim(q), r)
}

pub fn rem<F: Fq>(f: &F, a: &Poly, b: &Poly) -> Poly {
    divrem(f, a, b).1
}

pub fn mulmod<F: Fq>(f: &F, a: &Poly, b: &Poly, m: &Poly) -> Poly {
    rem(f, &mul(f, a, b), m)
}

pub fn monic<F: Fq>(f: &F, a: &Poly) -> Poly {
    let a = trim(a.clone());
    match a.last() {
        None => a,
        Some(&c) => scale(f, &a, f.finv(c)),
    }
}

pub fn gcd<F: Fq>(f: &F, a: &Poly, b: &Poly) -> Poly {
    let (mut a, mut b) = (trim(a.clone()), trim(b.clone()));
    while !b.is_empty() {
        let r = rem(f, &a, &b);
        a = b;
        b = r;
    }
    monic(f, &a)
}

/// `a^{-1} mod m`, if `gcd(a, m) = 1`.
pub fn invmod<F: Fq>(f: &F, a: &Poly, m: &Poly) -> Option<Poly> {
    let (mut r0, mut r1) = (trim(m.clone()), rem(f, a, m));
    let (mut t0, mut t1): (Poly, Poly) = (Vec::new(), vec![1]);
    while !r1.is_empty() {
        let (q, r) = divrem(f, &r0, &r1);
        let t = sub(f, &t0, &mul(f, &q, &t1));
        r0 = r1;
        r1 = r;
        t0 = t1;
        t1 = t;
    }
    if degree(&r0) != Some(0) {
        return None;
    }
    Some(rem(f, &scale(f, &t0, f.finv(r0[0])), m))
}

/// `base^e mod m` with `e` given as bits, least significant first.
pub fn powmod_bits<F: Fq>(f: &F, base: &Poly, bits: &[bool], m: &Poly) -> Poly {
    let mut r: Poly = vec![1];
    let mut b = rem(f, base, m);
    for &bit in bits {
        if bit {
            r = mulmod(f, &r, &b, m);
        }
        b = mulmod(f, &b, &b, m);
    }
    r
}

/// `a^q mod m`: the Frobenius of `F_q[x]/(m)` applied to `a`.
pub fn frob<F: Fq>(f: &F, a: &Poly, m: &Poly) -> Poly {
    if f.char2() {
        let mut r = rem(f, a, m);
        for _ in 0..f.bits() - 1 {
            r = mulmod(f, &r, &r, m);
        }
        r
    } else {
        powmod_bits(f, a, &f.q_bits(), m)
    }
}

pub fn derivative<F: Fq>(f: &F, a: &Poly) -> Poly {
    trim(
        a.iter()
            .enumerate()
            .skip(1)
            .map(|(i, &c)| f.fmul(f.fsmall(i as u64), c))
            .collect(),
    )
}

/// The `k`-th Hasse derivative `Σ_i C(i, k) a_i x^{i−k}`.
pub fn hasse<F: Fq>(f: &F, a: &Poly, k: usize) -> Poly {
    if a.len() <= k {
        return Vec::new();
    }
    trim(
        (k..a.len())
            .map(|i| f.fmul(f.fsmall(binom_mod(i as u64, k as u64, f.char2())), a[i]))
            .collect(),
    )
}

/// `C(n, k)`, exactly for the small `k` used here (`k ≤ 3`); reduced mod 2
/// in characteristic 2 by Lucas' theorem.
fn binom_mod(n: u64, k: u64, char2: bool) -> u64 {
    if char2 {
        return if (n & k) == k { 1 } else { 0 };
    }
    match k {
        0 => 1,
        1 => n,
        2 => n * (n - 1) / 2,
        3 => n * (n - 1) * (n - 2) / 6,
        _ => unreachable!("hasse: k ≤ 3"),
    }
}

pub fn eval<F: Fq>(f: &F, a: &Poly, x: u64) -> u64 {
    a.iter().rev().fold(0, |acc, &c| f.fadd(f.fmul(acc, x), c))
}

pub fn is_squarefree<F: Fq>(f: &F, a: &Poly) -> bool {
    degree(&gcd(f, a, &derivative(f, a))) == Some(0)
}

/// Distinct-degree factorisation up to degree `max_d`: returns
/// `[(d, product of the irreducible factors of degree d)]` for every `d ≤
/// max_d` with a factor, and the remaining cofactor (degree > `max_d` or 1).
/// `a` must be squarefree.
pub fn ddf<F: Fq>(f: &F, a: &Poly, max_d: usize) -> (Vec<(usize, Poly)>, Poly) {
    let mut a = monic(f, a);
    let mut out = Vec::new();
    let mut xq = x_poly();
    let mut d = 0;
    while degree(&a).unwrap_or(0) > 0 && d < max_d {
        d += 1;
        if degree(&a).unwrap() < 2 * d {
            // what is left is irreducible
            let deg = degree(&a).unwrap();
            if deg <= max_d {
                out.push((deg, a.clone()));
                a = vec![1];
            }
            break;
        }
        xq = frob(f, &xq, &a);
        let g = gcd(f, &a, &sub(f, &xq, &x_poly()));
        if degree(&g).unwrap_or(0) > 0 {
            out.push((d, g.clone()));
            a = divrem(f, &a, &g).0;
            xq = rem(f, &xq, &a);
        }
    }
    (out, a)
}

/// Split a monic squarefree product `g` of irreducibles of degree `d` into
/// them (Cantor–Zassenhaus; the trace map in characteristic 2).
pub fn edf<F: Fq>(f: &F, g: &Poly, d: usize, rng: &mut SmallRng, out: &mut Vec<Poly>) {
    let dg = degree(g).unwrap_or(0);
    if dg == 0 {
        return;
    }
    if dg == d {
        out.push(monic(f, g));
        return;
    }
    loop {
        let r: Poly = trim((0..dg).map(|_| f.frandom(rng)).collect());
        if degree(&r).unwrap_or(0) == 0 {
            continue;
        }
        let h = if f.char2() {
            // T(r) = r + r² + r⁴ + … + r^{2^{md−1}} mod g
            let steps = (f.bits() as usize - 1) * d;
            let mut t = r.clone();
            let mut acc = r.clone();
            for _ in 1..steps {
                t = mulmod(f, &t, &t, g);
                acc = add(f, &acc, &t);
            }
            acc
        } else {
            // r^{(q^d − 1)/2} − 1 mod g
            let qd = big_pow_bits(&f.q_bits(), d);
            let half = big_half_minus_one(&qd);
            sub(f, &powmod_bits(f, &r, &half, g), &vec![1])
        };
        let s = gcd(f, g, &h);
        let ds = degree(&s).unwrap_or(0);
        if ds > 0 && ds < dg {
            let other = monic(f, &divrem(f, g, &s).0);
            edf(f, &s, d, rng, out);
            edf(f, &other, d, rng, out);
            return;
        }
    }
}

/// `(q^d)` as bits from `q`'s bits (schoolbook on bit vectors).
fn big_pow_bits(q: &[bool], d: usize) -> Vec<bool> {
    let mut r = vec![true];
    for _ in 0..d {
        r = bits_mul(&r, q);
    }
    r
}

fn bits_mul(a: &[bool], b: &[bool]) -> Vec<bool> {
    let mut acc = vec![0u8; a.len() + b.len() + 1];
    for (i, &ai) in a.iter().enumerate() {
        if !ai {
            continue;
        }
        let mut carry = 0u8;
        for (j, &bj) in b.iter().enumerate() {
            let s = acc[i + j] + (bj as u8) + carry;
            acc[i + j] = s & 1;
            carry = s >> 1;
        }
        let mut k = i + b.len();
        while carry > 0 {
            let s = acc[k] + carry;
            acc[k] = s & 1;
            carry = s >> 1;
            k += 1;
        }
    }
    let mut v: Vec<bool> = acc.into_iter().map(|x| x == 1).collect();
    while v.len() > 1 && !v.last().unwrap() {
        v.pop();
    }
    v
}

/// `(n − 1)/2` as bits for odd `n` given as bits.
fn big_half_minus_one(n: &[bool]) -> Vec<bool> {
    // n is odd: (n − 1)/2 = n >> 1
    let mut v: Vec<bool> = n[1..].to_vec();
    while v.len() > 1 && !v.last().unwrap() {
        v.pop();
    }
    v
}

/// All roots in `F_q` of `a` (distinct).
pub fn roots<F: Fq>(f: &F, a: &Poly, rng: &mut SmallRng) -> Vec<u64> {
    let a = monic(f, a);
    if degree(&a).unwrap_or(0) == 0 {
        return Vec::new();
    }
    let xq = frob(f, &x_poly(), &a);
    let g = gcd(f, &a, &sub(f, &xq, &x_poly()));
    let mut lin = Vec::new();
    edf(f, &g, 1, rng, &mut lin);
    let mut r: Vec<u64> = lin.iter().map(|l| f.fneg(l[0])).collect();
    r.sort();
    r
}

/// The degree of the smallest irreducible factor of a squarefree `a` (the
/// size of the smallest Frobenius orbit among its roots).
pub fn min_factor_degree<F: Fq>(f: &F, a: &Poly) -> usize {
    let a = monic(f, a);
    let mut xq = x_poly();
    for d in 1..=degree(&a).unwrap_or(0) {
        xq = frob(f, &xq, &a);
        if degree(&gcd(f, &a, &sub(f, &xq, &x_poly()))).unwrap_or(0) > 0 {
            return d;
        }
    }
    degree(&a).unwrap_or(0)
}

// ================================================================== F_q[y]/(g)

/// The extension `F_q[y]/(g)`, `g` monic irreducible of degree `d`.
pub struct Ext<'a, F: Fq> {
    pub f: &'a F,
    pub g: Poly,
}

impl<'a, F: Fq> Ext<'a, F> {
    pub fn new(f: &'a F, g: &Poly) -> Self {
        let g = monic(f, g);
        degree(&g).expect("extension modulus");
        Ext { f, g }
    }
    /// The class of `y`.
    pub fn gen(&self) -> Poly {
        rem(self.f, &x_poly(), &self.g)
    }
    /// The base-field element `a` is, if it lies in `F_q`.
    pub fn to_base(&self, a: &Poly) -> Option<u64> {
        let a = trim(a.clone());
        match a.len() {
            0 => Some(0),
            1 => Some(a[0]),
            _ => None,
        }
    }
}

impl<F: Fq> Ring for Ext<'_, F> {
    type El = Poly;
    fn zero(&self) -> Poly {
        Vec::new()
    }
    fn one(&self) -> Poly {
        vec![1]
    }
    fn small(&self, k: u64) -> Poly {
        trim(vec![self.f.fsmall(k)])
    }
    fn lift(&self, v: u64) -> Poly {
        trim(vec![v])
    }
    fn is_zero(&self, a: &Poly) -> bool {
        trim(a.clone()).is_empty()
    }
    fn add(&self, a: &Poly, b: &Poly) -> Poly {
        add(self.f, a, b)
    }
    fn sub(&self, a: &Poly, b: &Poly) -> Poly {
        sub(self.f, a, b)
    }
    fn mul(&self, a: &Poly, b: &Poly) -> Poly {
        mulmod(self.f, a, b, &self.g)
    }
    fn inv(&self, a: &Poly) -> Option<Poly> {
        invmod(self.f, a, &self.g)
    }
}

/// `∏ (x − r_i)` over ring elements `r_i`, as a polynomial with ring
/// coefficients (outer index = power of `x`).
pub fn product_of_linear<R: Ring>(r: &R, rs: &[R::El]) -> Vec<R::El> {
    let mut p = vec![r.one()];
    for root in rs {
        // p ← p·(x − root)
        let mut np = vec![r.zero(); p.len() + 1];
        for (i, c) in p.iter().enumerate() {
            np[i + 1] = r.add(&np[i + 1], c);
            np[i] = r.sub(&np[i], &r.mul(c, root));
        }
        p = np;
    }
    p
}

// ================================================================== F_2[y] bit-packed

/// Bit-packed `F_2[y]` polynomial helpers for finding an irreducible of
/// large degree (the modulus of an extension `F_{q^d}` when `gcd(d, m) = 1`:
/// a polynomial irreducible over `F_2` stays irreducible over `F_{2^m}`).
mod f2 {
    pub type P = Vec<u64>;
    pub fn deg(a: &P) -> Option<usize> {
        for (i, &w) in a.iter().enumerate().rev() {
            if w != 0 {
                return Some(i * 64 + 63 - w.leading_zeros() as usize);
            }
        }
        None
    }
    pub fn bit(a: &P, i: usize) -> bool {
        a.get(i / 64).is_some_and(|w| (w >> (i % 64)) & 1 == 1)
    }
    pub fn set(a: &mut P, i: usize) {
        if a.len() <= i / 64 {
            a.resize(i / 64 + 1, 0);
        }
        a[i / 64] ^= 1 << (i % 64);
    }
    pub fn shl(a: &P, s: usize) -> P {
        let (w, b) = (s / 64, s % 64);
        let mut r = vec![0u64; a.len() + w + 1];
        for (i, &x) in a.iter().enumerate() {
            r[i + w] |= x << b;
            if b > 0 {
                r[i + w + 1] |= x >> (64 - b);
            }
        }
        while r.last() == Some(&0) {
            r.pop();
        }
        r
    }
    pub fn xor_into(a: &mut P, b: &P) {
        if a.len() < b.len() {
            a.resize(b.len(), 0);
        }
        for (x, y) in a.iter_mut().zip(b) {
            *x ^= y;
        }
        while a.last() == Some(&0) {
            a.pop();
        }
    }
    pub fn rem(a: &P, m: &P) -> P {
        let dm = deg(m).unwrap();
        let mut r = a.clone();
        while let Some(d) = deg(&r) {
            if d < dm {
                break;
            }
            let s = shl(m, d - dm);
            xor_into(&mut r, &s);
        }
        r
    }
    pub fn mul(a: &P, b: &P) -> P {
        let mut r = vec![0u64; a.len() + b.len() + 1];
        if let Some(db) = deg(b) {
            for i in 0..=db {
                if bit(b, i) {
                    let s = shl(a, i);
                    xor_into(&mut r, &s);
                }
            }
        }
        r
    }
    pub fn gcd(a: &P, b: &P) -> P {
        let (mut a, mut b) = (a.clone(), b.clone());
        while deg(&b).is_some() {
            let r = rem(&a, &b);
            a = b;
            b = r;
        }
        a
    }
    pub fn is_one(a: &P) -> bool {
        deg(a) == Some(0)
    }
    /// Ben-Or irreducibility test.
    pub fn irreducible(f: &P) -> bool {
        let d = deg(f).unwrap();
        let mut x: P = vec![2];
        let xx: P = vec![2];
        for _ in 1..=d / 2 {
            x = rem(&mul(&x, &x), f);
            let mut t = x.clone();
            xor_into(&mut t, &xx);
            if !is_one(&gcd(f, &t)) {
                return false;
            }
        }
        true
    }
    /// An irreducible polynomial of degree `d` over `F_2`: a trinomial if
    /// one exists, else a pentanomial.  Returned as `0/1` coefficients.
    pub fn find_irreducible(d: usize) -> Vec<u64> {
        let mut f: P = Vec::new();
        set(&mut f, d);
        set(&mut f, 0);
        for t in 1..d {
            let mut g = f.clone();
            set(&mut g, t);
            if irreducible(&g) {
                return (0..=d).map(|i| bit(&g, i) as u64).collect();
            }
        }
        for t3 in 3..d {
            for t2 in 2..t3 {
                for t1 in 1..t2 {
                    let mut g = f.clone();
                    set(&mut g, t3);
                    set(&mut g, t2);
                    set(&mut g, t1);
                    if irreducible(&g) {
                        return (0..=d).map(|i| bit(&g, i) as u64).collect();
                    }
                }
            }
        }
        panic!("no irreducible polynomial of degree {d} found")
    }
}

pub use f2::find_irreducible as irreducible_f2;

/// `x(kP)` from `x(P)` by the Montgomery ladder in any ring, `k` given as
/// big-endian bits; `None` when a multiple is `O` or two ladder legs
/// coincide.
pub fn x_mul_bits<F: Fq, R: Ring>(
    f: &F,
    r: &R,
    c: &super::field::Curve,
    x1: &R::El,
    bits: &[bool],
) -> Option<R::El> {
    // (x_n, x_{n+1}) starting at n = 1
    let mut a = x1.clone();
    let mut b = f.x_dbl(r, c, x1)?;
    let mut started = false;
    for &bit in bits {
        if !started {
            if bit {
                started = true;
            }
            continue;
        }
        if bit {
            // (n, n+1) → (2n+1, 2n+2)
            let s = f.x_sum(r, c, &a, &b, x1)?;
            let d = f.x_dbl(r, c, &b)?;
            a = s;
            b = d;
        } else {
            // (n, n+1) → (2n, 2n+1)
            let s = f.x_sum(r, c, &a, &b, x1)?;
            let d = f.x_dbl(r, c, &a)?;
            b = s;
            a = d;
        }
    }
    Some(a)
}
