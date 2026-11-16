//! Point counting on elliptic curves over large odd fields: Schoof's algorithm
//! for t mod l, finished by baby-step giant-step on the residue class.
//!
//! [`frobenius_trace`] returns t = q + 1 - #E(F_q) for y^2 = x^3 + a x + b
//! over F_q, q = p^k odd with p > 3, where F_q is [`PrimeField`] (any p) or
//! [`ExtField`] (p < 2^60, k <= 16).
//!
//! # Method
//!
//! * **t mod 2**: #E is even iff x^3 + a x + b has a root in F_q, i.e. iff
//!   gcd(x^q - x, x^3 + a x + b) != 1; and t = q + 1 - #E = #E (mod 2).
//! * **t mod l** for odd primes l != p (Schoof 1985): in R = F_q[x]/(h), h a
//!   factor of the l-division polynomial psi_l, with y^2 = x^3 + a x + b, the
//!   generic l-torsion point P = (x, y) satisfies phi^2(P) + [q mod l] P =
//!   [tau] phi(P), phi the q-power Frobenius, exactly for tau = t mod l. Points
//!   of E(R[y]) are kept as (X, Y) meaning (X, Y y); x^(q^2) and y^(q^2) come
//!   from x^q and y^q by modular composition (Brent-Kung); [k]P from the
//!   division polynomials; [tau] phi(P) in projective coordinates, so the loop
//!   over tau has no inversion. When an inversion in R meets a zero divisor,
//!   the gcd it exposes is a proper factor of h, stable under Frobenius (it is
//!   defined over F_q), and the computation restarts modulo that factor: the
//!   relation holds for every nonzero l-torsion point, so tau is still unique.
//!   A tau that is not found, or a y-coordinate that matches neither +tau nor
//!   -tau, is an inconsistency (an error, never a guess).
//! * **Finishing**: with t = c (mod M), M the product of the primes used, the
//!   order N = q + 1 - t lies in the Hasse interval and in one residue class
//!   modulo M. Baby-step giant-step finds, for a point P, every N of that class
//!   in the interval with N P = O (each candidate is confirmed by a scalar
//!   multiplication). The candidate sets of several points are intersected
//!   until one is left; since the true order is in every set, a unique survivor
//!   is #E (a proof, not a heuristic). No candidate means the congruences are
//!   wrong: an error. Several survivors after every point tried: ambiguous.
//!
//! Primes are taken in increasing order until the residue class leaves at most
//! 2^[`Config::bsgs_bits`] candidates; a prime above [`Config::max_l`] needed
//! for that is [`SchoofError::TooLarge`]. Nothing here depends on time.

use num_bigint::{BigInt, BigUint};
use num_traits::{One, ToPrimitive, Zero};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

// ------------------------------------------------------------------ fields

/// The field operations the point count needs.
pub trait Field: Sync {
    type E: Clone + PartialEq + Eq + Hash + Debug + Send + Sync;
    /// q = |F|.
    fn q(&self) -> &BigUint;
    /// The characteristic.
    fn char_p(&self) -> &BigUint;
    fn zero(&self) -> Self::E;
    fn one(&self) -> Self::E;
    fn elt_u64(&self, n: u64) -> Self::E;
    fn add(&self, a: &Self::E, b: &Self::E) -> Self::E;
    fn sub(&self, a: &Self::E, b: &Self::E) -> Self::E;
    fn mul(&self, a: &Self::E, b: &Self::E) -> Self::E;
    /// `None` for zero.
    fn inv(&self, a: &Self::E) -> Option<Self::E>;
    fn is_zero(&self, a: &Self::E) -> bool;
    /// Distinct elements for distinct i < q (a deterministic point source).
    fn index_elt(&self, i: u64) -> Self::E;
    /// The product of two polynomials (ascending coefficients).
    fn poly_mul(&self, a: &[Self::E], b: &[Self::E]) -> Vec<Self::E>;

    fn neg(&self, a: &Self::E) -> Self::E {
        self.sub(&self.zero(), a)
    }

    /// sum_i c_i v_i, each v_i of length <= d (the result has length d).
    fn lin_comb(&self, c: &[&Self::E], v: &[&[Self::E]], d: usize) -> Vec<Self::E> {
        let mut acc = vec![self.zero(); d];
        for (ci, vi) in c.iter().zip(v) {
            for (t, x) in vi.iter().enumerate() {
                acc[t] = self.add(&acc[t], &self.mul(ci, x));
            }
        }
        acc
    }
}

/// Schoolbook product (the small-size path of every field).
fn schoolbook<F: Field>(f: &F, a: &[F::E], b: &[F::E]) -> Vec<F::E> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![f.zero(); a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        if f.is_zero(x) {
            continue;
        }
        for (j, y) in b.iter().enumerate() {
            r[i + j] = f.add(&r[i + j], &f.mul(x, y));
        }
    }
    r
}

/// Below this length the fields multiply polynomials by schoolbook.
const KRONECKER_MIN: usize = 12;

/// F_p for any odd prime p, on `BigUint` residues; polynomial products by
/// Kronecker substitution (one big-integer product per polynomial product).
pub struct PrimeField {
    p: BigUint,
    pbits: u64,
}

impl PrimeField {
    pub fn new(p: BigUint) -> PrimeField {
        let pbits = p.bits();
        PrimeField { p, pbits }
    }
}

/// Limbs of `x` (u32 digits) written at `out[at..]`.
fn put_digits(out: &mut [u32], at: usize, x: &BigUint) {
    for (i, d) in x.iter_u32_digits().enumerate() {
        out[at + i] = d;
    }
}

impl Field for PrimeField {
    type E = BigUint;
    fn q(&self) -> &BigUint {
        &self.p
    }
    fn char_p(&self) -> &BigUint {
        &self.p
    }
    fn zero(&self) -> BigUint {
        BigUint::zero()
    }
    fn one(&self) -> BigUint {
        BigUint::one()
    }
    fn elt_u64(&self, n: u64) -> BigUint {
        BigUint::from(n) % &self.p
    }
    fn add(&self, a: &BigUint, b: &BigUint) -> BigUint {
        let s = a + b;
        if s >= self.p {
            s - &self.p
        } else {
            s
        }
    }
    fn sub(&self, a: &BigUint, b: &BigUint) -> BigUint {
        if a >= b {
            a - b
        } else {
            a + &self.p - b
        }
    }
    fn mul(&self, a: &BigUint, b: &BigUint) -> BigUint {
        (a * b) % &self.p
    }
    fn inv(&self, a: &BigUint) -> Option<BigUint> {
        if a.is_zero() {
            return None;
        }
        a.modinv(&self.p)
    }
    fn is_zero(&self, a: &BigUint) -> bool {
        a.is_zero()
    }
    fn index_elt(&self, i: u64) -> BigUint {
        BigUint::from(i) % &self.p
    }
    fn lin_comb(&self, c: &[&BigUint], v: &[&[BigUint]], d: usize) -> Vec<BigUint> {
        // Unreduced sums, one reduction per coefficient.
        let mut acc = vec![BigUint::zero(); d];
        for (ci, vi) in c.iter().zip(v) {
            for (t, x) in vi.iter().enumerate() {
                acc[t] += *ci * x;
            }
        }
        acc.into_iter().map(|x| x % &self.p).collect()
    }
    fn poly_mul(&self, a: &[BigUint], b: &[BigUint]) -> Vec<BigUint> {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        if a.len().min(b.len()) < KRONECKER_MIN {
            return schoolbook(self, a, b);
        }
        // Each product coefficient is < min(len) p^2: slots of whole u32 digits.
        let n = a.len().min(b.len()) as u64;
        let bits = 2 * self.pbits + (64 - n.leading_zeros() as u64) + 1;
        let s = bits.div_ceil(32) as usize;
        let pack = |v: &[BigUint]| {
            let mut d = vec![0u32; v.len() * s];
            for (i, c) in v.iter().enumerate() {
                put_digits(&mut d, i * s, c);
            }
            BigUint::new(d)
        };
        let prod = if std::ptr::eq(a, b) {
            let x = pack(a);
            &x * &x
        } else {
            pack(a) * pack(b)
        };
        let digits = prod.to_u32_digits();
        let len = a.len() + b.len() - 1;
        (0..len)
            .map(|i| {
                let lo = (i * s).min(digits.len());
                let hi = ((i + 1) * s).min(digits.len());
                BigUint::from_slice(&digits[lo..hi]) % &self.p
            })
            .collect()
    }
}

/// Largest extension degree of [`ExtField`].
pub const MAX_EXT_DEGREE: usize = 16;

/// An element of [`ExtField`]: coefficients in the basis 1, t, .., t^(k-1),
/// each in 0..p, unused slots zero.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Ek(pub [u64; MAX_EXT_DEGREE]);

/// GF(p^k) = F_p[t]/(m(t)), p < 2^60 odd, 2 <= k <= 16, m monic irreducible.
pub struct ExtField {
    p: u64,
    k: usize,
    /// m(t) = t^k + sum low[i] t^i: `neg_low[i]` = -low[i] mod p.
    neg_low: [u64; MAX_EXT_DEGREE],
    q: BigUint,
    pbig: BigUint,
}

fn mulmod64(a: u64, b: u64, p: u64) -> u64 {
    (a as u128 * b as u128 % p as u128) as u64
}

fn inv64(a: u64, p: u64) -> Option<u64> {
    let (mut r0, mut r1) = (p as i128, a as i128);
    let (mut t0, mut t1) = (0i128, 1i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    (r0 == 1).then(|| t0.rem_euclid(p as i128) as u64)
}

impl ExtField {
    /// `low` = m_0..m_{k-1} of the monic modulus (any integers; reduced mod p).
    /// `None` outside p < 2^60 odd, 2 <= k <= 16.
    pub fn new(p: u64, low: &[i128]) -> Option<ExtField> {
        let k = low.len();
        if !(2..=MAX_EXT_DEGREE).contains(&k) || !(5..1 << 60).contains(&p) || p.is_multiple_of(2) {
            return None;
        }
        let mut neg_low = [0u64; MAX_EXT_DEGREE];
        for (i, &c) in low.iter().enumerate() {
            neg_low[i] = (-c).rem_euclid(p as i128) as u64;
        }
        let pbig = BigUint::from(p);
        let q = num_traits::pow(pbig.clone(), k);
        Some(ExtField { p, k, neg_low, q, pbig })
    }

    pub fn degree(&self) -> usize {
        self.k
    }

    /// The element with coefficients `c` (reduced mod p).
    pub fn elt(&self, c: &[u64]) -> Ek {
        let mut e = [0u64; MAX_EXT_DEGREE];
        for (i, &x) in c.iter().take(self.k).enumerate() {
            e[i] = x % self.p;
        }
        Ek(e)
    }

    /// Reduce 2k - 1 coefficients modulo p and m(t). Each input is below
    /// 2^124 / 2 (at most k products of residues, or one reduced slot); the
    /// reduction adds at most k - 1 products below p^2 to each, so the
    /// accumulators stay below 2k p^2 < 2^125 for p < 2^60, k <= 16.
    fn reduce_wide(&self, w: &mut [u128]) -> Ek {
        let (k, p) = (self.k, self.p as u128);
        for i in (k..w.len()).rev() {
            let lead = w[i] % p;
            if lead == 0 {
                continue;
            }
            for j in 0..k {
                w[i - k + j] += lead * self.neg_low[j] as u128;
            }
        }
        let mut e = [0u64; MAX_EXT_DEGREE];
        for (ei, wi) in e.iter_mut().zip(&w[..k]) {
            *ei = (*wi % p) as u64;
        }
        Ek(e)
    }

    /// Slot width (u32 digits) for Kronecker products with `n` terms per sum.
    fn slot_digits(&self, n: usize) -> usize {
        let pb = 64 - self.p.leading_zeros() as u64;
        let nb = 64 - ((n * self.k) as u64).leading_zeros() as u64;
        (2 * pb + nb + 1).div_ceil(32) as usize
    }
}

impl Field for ExtField {
    type E = Ek;
    fn q(&self) -> &BigUint {
        &self.q
    }
    fn char_p(&self) -> &BigUint {
        &self.pbig
    }
    fn zero(&self) -> Ek {
        Ek([0; MAX_EXT_DEGREE])
    }
    fn one(&self) -> Ek {
        self.elt_u64(1)
    }
    fn elt_u64(&self, n: u64) -> Ek {
        let mut e = [0u64; MAX_EXT_DEGREE];
        e[0] = n % self.p;
        Ek(e)
    }
    fn add(&self, a: &Ek, b: &Ek) -> Ek {
        let mut e = [0u64; MAX_EXT_DEGREE];
        for (ei, (x, y)) in e.iter_mut().zip(a.0.iter().zip(&b.0)).take(self.k) {
            let s = x + y;
            *ei = if s >= self.p { s - self.p } else { s };
        }
        Ek(e)
    }
    fn sub(&self, a: &Ek, b: &Ek) -> Ek {
        let mut e = [0u64; MAX_EXT_DEGREE];
        for (ei, (&x, &y)) in e.iter_mut().zip(a.0.iter().zip(&b.0)).take(self.k) {
            *ei = if x >= y { x - y } else { x + self.p - y };
        }
        Ek(e)
    }
    fn mul(&self, a: &Ek, b: &Ek) -> Ek {
        let k = self.k;
        let mut w = [0u128; 2 * MAX_EXT_DEGREE - 1];
        for i in 0..k {
            if a.0[i] == 0 {
                continue;
            }
            for j in 0..k {
                w[i + j] += a.0[i] as u128 * b.0[j] as u128;
            }
        }
        self.reduce_wide(&mut w[..2 * k - 1])
    }
    fn inv(&self, a: &Ek) -> Option<Ek> {
        // Extended Euclid in F_p[t]: r0 = m, r1 = a; s tracks the a-coefficient.
        let (k, p) = (self.k, self.p);
        if a.0[..k].iter().all(|&c| c == 0) {
            return None;
        }
        let trim = |v: &mut Vec<u64>| {
            while v.last() == Some(&0) {
                v.pop();
            }
        };
        let mut r0: Vec<u64> = (0..k).map(|i| (p - self.neg_low[i]) % p).chain([1]).collect();
        let mut r1: Vec<u64> = a.0[..k].to_vec();
        trim(&mut r1);
        let (mut s0, mut s1): (Vec<u64>, Vec<u64>) = (vec![], vec![1]);
        while !r1.is_empty() {
            // r0 = qt r1 + r
            let inv_lead = inv64(*r1.last()?, p)?;
            let mut qt = vec![0u64; r0.len().saturating_sub(r1.len()) + 1];
            let mut r = r0.clone();
            while r.len() >= r1.len() && !r.is_empty() {
                let shift = r.len() - r1.len();
                let c = mulmod64(*r.last()?, inv_lead, p);
                qt[shift] = c;
                for (j, &x) in r1.iter().enumerate() {
                    r[shift + j] = (r[shift + j] + p - mulmod64(c, x, p)) % p;
                }
                trim(&mut r);
            }
            // s0 - qt s1
            let mut ns = vec![0u64; (qt.len() + s1.len()).max(s0.len())];
            for (i, &x) in s0.iter().enumerate() {
                ns[i] = x;
            }
            for (i, &x) in qt.iter().enumerate() {
                for (j, &y) in s1.iter().enumerate() {
                    ns[i + j] = (ns[i + j] + p - mulmod64(x, y, p)) % p;
                }
            }
            trim(&mut ns);
            (r0, r1) = (r1, r);
            (s0, s1) = (s1, ns);
        }
        if r0.len() != 1 {
            return None;
        }
        let c = inv64(r0[0], p)?;
        let mut e = [0u64; MAX_EXT_DEGREE];
        for (i, &x) in s0.iter().enumerate().take(k) {
            e[i] = mulmod64(x, c, p);
        }
        Some(Ek(e))
    }
    fn is_zero(&self, a: &Ek) -> bool {
        a.0[..self.k].iter().all(|&c| c == 0)
    }
    fn index_elt(&self, mut i: u64) -> Ek {
        let mut e = [0u64; MAX_EXT_DEGREE];
        for c in e.iter_mut().take(self.k) {
            *c = i % self.p;
            i /= self.p;
        }
        Ek(e)
    }
    fn poly_mul(&self, a: &[Ek], b: &[Ek]) -> Vec<Ek> {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let s = self.slot_digits(a.len().min(b.len()));
        if a.len().min(b.len()) < KRONECKER_MIN || s > 4 {
            return schoolbook(self, a, b);
        }
        // x -> 2^(32 s (2k - 1)), t -> 2^(32 s): every product coefficient of
        // t^j x^i lands in its own slot.
        let k = self.k;
        let stride = (2 * k - 1) * s;
        let pack = |v: &[Ek]| {
            let mut d = vec![0u32; v.len() * stride];
            for (i, c) in v.iter().enumerate() {
                for j in 0..k {
                    let x = c.0[j];
                    let at = i * stride + j * s;
                    d[at] = x as u32;
                    if s > 1 {
                        d[at + 1] = (x >> 32) as u32;
                    }
                }
            }
            BigUint::new(d)
        };
        let prod = if std::ptr::eq(a, b) {
            let x = pack(a);
            &x * &x
        } else {
            pack(a) * pack(b)
        };
        let digits = prod.to_u32_digits();
        let len = a.len() + b.len() - 1;
        let mut w = vec![0u128; 2 * k - 1];
        (0..len)
            .map(|i| {
                for (j, wj) in w.iter_mut().enumerate() {
                    let at = i * stride + j * s;
                    let mut v = 0u128;
                    for d in (0..s).rev() {
                        v = (v << 32) | digits.get(at + d).copied().unwrap_or(0) as u128;
                    }
                    *wj = v % self.p as u128;
                }
                self.reduce_wide(&mut w)
            })
            .collect()
    }
}

/// a^e in F.
pub fn fpow<F: Field>(f: &F, a: &F::E, e: &BigUint) -> F::E {
    let mut r = f.one();
    for i in (0..e.bits()).rev() {
        r = f.mul(&r, &r);
        if e.bit(i) {
            r = f.mul(&r, a);
        }
    }
    r
}

/// A deterministic 64-bit mix (SplitMix64).
fn splitmix64(i: u64) -> u64 {
    let mut z = i.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// A square root by Tonelli-Shanks (q odd); `None` for a non-square.
pub fn fsqrt<F: Field>(f: &F, a: &F::E) -> Option<F::E> {
    if f.is_zero(a) {
        return Some(f.zero());
    }
    let q1 = f.q() - 1u32;
    let one = f.one();
    if fpow(f, a, &(&q1 >> 1)) != one {
        return None;
    }
    let e = q1.trailing_zeros().unwrap_or(0);
    let s = &q1 >> e;
    // A non-residue z. Small indices are elements of the prime field, all
    // squares when q = p^(even): draw spread-out indices instead.
    let cap = f.q().to_u64().unwrap_or(u64::MAX);
    let z = (1u64..)
        .map(|i| f.index_elt(splitmix64(i) % cap))
        .take(4096)
        .find(|z| !f.is_zero(z) && fpow(f, z, &(&q1 >> 1)) != one)?;
    let mut c = fpow(f, &z, &s);
    let mut x = fpow(f, a, &((&s + 1u32) >> 1));
    let mut t = fpow(f, a, &s);
    let mut m = e;
    while t != one {
        let mut i = 0u64;
        let mut t2 = t.clone();
        while t2 != one {
            t2 = f.mul(&t2, &t2);
            i += 1;
            if i >= m {
                return None;
            }
        }
        let mut b = c.clone();
        for _ in 0..m - i - 1 {
            b = f.mul(&b, &b);
        }
        x = f.mul(&x, &b);
        c = f.mul(&b, &b);
        t = f.mul(&t, &c);
        m = i;
    }
    Some(x)
}

// ------------------------------------------------------------- polynomials

fn trim<F: Field>(f: &F, mut a: Vec<F::E>) -> Vec<F::E> {
    while a.last().is_some_and(|c| f.is_zero(c)) {
        a.pop();
    }
    a
}

fn padd<F: Field>(f: &F, a: &[F::E], b: &[F::E]) -> Vec<F::E> {
    let n = a.len().max(b.len());
    let z = f.zero();
    trim(f, (0..n).map(|i| f.add(a.get(i).unwrap_or(&z), b.get(i).unwrap_or(&z))).collect())
}

fn psub<F: Field>(f: &F, a: &[F::E], b: &[F::E]) -> Vec<F::E> {
    let n = a.len().max(b.len());
    let z = f.zero();
    trim(f, (0..n).map(|i| f.sub(a.get(i).unwrap_or(&z), b.get(i).unwrap_or(&z))).collect())
}

fn pscale<F: Field>(f: &F, a: &[F::E], c: &F::E) -> Vec<F::E> {
    trim(f, a.iter().map(|x| f.mul(x, c)).collect())
}

fn pmul<F: Field>(f: &F, a: &[F::E], b: &[F::E]) -> Vec<F::E> {
    trim(f, f.poly_mul(a, b))
}

/// (quotient, remainder) of a by b != 0, schoolbook.
fn pdivrem<F: Field>(f: &F, a: &[F::E], b: &[F::E]) -> (Vec<F::E>, Vec<F::E>) {
    let b = trim(f, b.to_vec());
    let mut r = trim(f, a.to_vec());
    if r.len() < b.len() {
        return (vec![], r);
    }
    let inv = f.inv(b.last().expect("nonzero divisor")).expect("nonzero leading coefficient");
    let mut qt = vec![f.zero(); r.len() - b.len() + 1];
    while r.len() >= b.len() && !r.is_empty() {
        let shift = r.len() - b.len();
        let c = f.mul(r.last().expect("nonempty"), &inv);
        for (j, x) in b.iter().enumerate() {
            r[shift + j] = f.sub(&r[shift + j], &f.mul(&c, x));
        }
        qt[shift] = c;
        r = trim(f, r);
    }
    (trim(f, qt), r)
}

fn monic<F: Field>(f: &F, a: Vec<F::E>) -> Vec<F::E> {
    match a.last().and_then(|c| f.inv(c)) {
        Some(i) => pscale(f, &a, &i),
        None => a,
    }
}

/// The monic gcd.
fn pgcd<F: Field>(f: &F, a: &[F::E], b: &[F::E]) -> Vec<F::E> {
    let (mut a, mut b) = (trim(f, a.to_vec()), trim(f, b.to_vec()));
    while !b.is_empty() {
        let r = pdivrem(f, &a, &b).1;
        (a, b) = (b, r);
    }
    monic(f, a)
}

/// Arithmetic in F[x]/(h), h monic of degree d >= 1.
pub struct PolyMod<'a, F: Field> {
    f: &'a F,
    h: Vec<F::E>,
    d: usize,
    /// 1 / rev(h) mod x^(d - 1) (reduction by Newton's method).
    hrev_inv: Vec<F::E>,
}

impl<'a, F: Field> PolyMod<'a, F> {
    pub fn new(f: &'a F, h: Vec<F::E>) -> Self {
        let h = monic(f, trim(f, h));
        let d = h.len() - 1;
        let n = d.saturating_sub(1);
        let hrev: Vec<F::E> = h.iter().rev().cloned().collect();
        let mut g = vec![f.one()];
        let mut prec = 1;
        while prec < n {
            let p2 = (2 * prec).min(n);
            let mut e = f.poly_mul(&hrev[..p2.min(hrev.len())], &g);
            e.truncate(p2);
            // g (2 - e)
            let mut two_minus = e.iter().map(|c| f.neg(c)).collect::<Vec<_>>();
            two_minus.resize(p2, f.zero());
            two_minus[0] = f.add(&two_minus[0], &f.elt_u64(2));
            let mut ng = f.poly_mul(&g, &two_minus);
            ng.truncate(p2);
            g = ng;
            prec = p2;
        }
        g.truncate(n);
        PolyMod { f, h, d, hrev_inv: g }
    }

    pub fn modulus(&self) -> &[F::E] {
        &self.h
    }

    pub fn reduce(&self, a: Vec<F::E>) -> Vec<F::E> {
        let f = self.f;
        let a = trim(f, a);
        if a.len() <= self.d {
            return a;
        }
        if a.len() > 2 * self.d - 1 || self.d < 2 {
            return pdivrem(f, &a, &self.h).1;
        }
        let n = a.len() - 1;
        let m = n - self.d;
        let rev_a: Vec<F::E> = (0..=m).map(|i| a[n - i].clone()).collect();
        let mut qrev = f.poly_mul(&rev_a, &self.hrev_inv[..(m + 1).min(self.hrev_inv.len())]);
        qrev.resize(m + 1, f.zero());
        let qt: Vec<F::E> = qrev.into_iter().rev().collect();
        let qh = f.poly_mul(&qt, &self.h);
        let z = f.zero();
        trim(f, (0..self.d).map(|i| f.sub(&a[i], qh.get(i).unwrap_or(&z))).collect())
    }

    pub fn mul(&self, a: &[F::E], b: &[F::E]) -> Vec<F::E> {
        self.reduce(self.f.poly_mul(a, b))
    }

    pub fn sqr(&self, a: &[F::E]) -> Vec<F::E> {
        self.reduce(self.f.poly_mul(a, a))
    }

    /// a^e mod h, left-to-right with a 4-bit window.
    pub fn pow(&self, a: &[F::E], e: &BigUint) -> Vec<F::E> {
        let a = self.reduce(a.to_vec());
        let mut table = vec![vec![self.f.one()], a.clone()];
        for i in 2..16 {
            let next = self.mul(&table[i - 1], &a);
            table.push(next);
        }
        let mut r = vec![self.f.one()];
        let nb = e.bits();
        let mut i = nb.div_ceil(4);
        while i > 0 {
            i -= 1;
            for _ in 0..4 {
                r = self.sqr(&r);
            }
            let w = (0..4).fold(0usize, |acc, j| acc | (e.bit(4 * i + j) as usize) << j);
            if w != 0 {
                r = self.mul(&r, &table[w]);
            }
        }
        r
    }

    /// The inverse of a, or `Err(g)` with g = gcd(a, h) monic of degree >= 1.
    pub fn inv(&self, a: &[F::E]) -> Result<Vec<F::E>, Vec<F::E>> {
        let f = self.f;
        let (mut r0, mut r1) = (self.h.clone(), self.reduce(a.to_vec()));
        let (mut s0, mut s1) = (vec![], vec![f.one()]);
        while !r1.is_empty() {
            let (qt, r) = pdivrem(f, &r0, &r1);
            let ns = psub(f, &s0, &pmul(f, &qt, &s1));
            (r0, r1) = (r1, r);
            (s0, s1) = (s1, ns);
        }
        if r0.len() == 1 {
            let c = f.inv(&r0[0]).expect("nonzero constant");
            Ok(self.reduce(pscale(f, &s0, &c)))
        } else {
            Err(monic(f, r0))
        }
    }

    /// g(b) mod h for g of any degree (Brent-Kung).
    pub fn compose(&self, g: &[F::E], b: &[F::E]) -> Vec<F::E> {
        let f = self.f;
        let g = trim(f, g.to_vec());
        if g.is_empty() {
            return g;
        }
        let m = ((g.len() as f64).sqrt().ceil() as usize).max(1);
        let mut pw = vec![vec![f.one()], self.reduce(b.to_vec())];
        while pw.len() <= m {
            let next = self.mul(pw.last().expect("nonempty"), &pw[1]);
            pw.push(next);
        }
        let blocks = g.len().div_ceil(m);
        let block = |j: usize| -> Vec<F::E> {
            let (mut cs, mut vs) = (vec![], vec![]);
            for (c, pwi) in g.iter().skip(j * m).take(m).zip(&pw) {
                if !f.is_zero(c) {
                    cs.push(c);
                    vs.push(&pwi[..]);
                }
            }
            trim(f, f.lin_comb(&cs, &vs, self.d))
        };
        let mut r = block(blocks - 1);
        for j in (0..blocks - 1).rev() {
            r = padd(f, &self.mul(&r, &pw[m]), &block(j));
        }
        r
    }
}

// ----------------------------------------------------- division polynomials

/// The division polynomials of y^2 = x^3 + a x + b in the "stand-in" form:
/// psi_n for odd n, psi_n / (2y) for even n (both polynomials in x), optionally
/// reduced modulo h.
struct DivPolys<'a, 'b, F: Field> {
    f: &'a F,
    m: Option<&'b PolyMod<'a, F>>,
    /// (2y)^4 = 16 (x^3 + a x + b)^2.
    ff: Vec<F::E>,
    memo: HashMap<usize, Vec<F::E>>,
}

impl<'a, 'b, F: Field> DivPolys<'a, 'b, F> {
    fn new(f: &'a F, a: &F::E, b: &F::E, m: Option<&'b PolyMod<'a, F>>) -> Self {
        let c = |n: u64| f.elt_u64(n);
        let mut memo = HashMap::new();
        let red = |v: Vec<F::E>| match m {
            Some(m) => m.reduce(v),
            None => trim(f, v),
        };
        let cubic = vec![b.clone(), a.clone(), f.zero(), f.one()];
        let ff = red(pscale(f, &pmul(f, &cubic, &cubic), &c(16)));
        memo.insert(0, vec![]);
        memo.insert(1, vec![f.one()]);
        memo.insert(2, vec![f.one()]);
        // psi_3 = 3x^4 + 6a x^2 + 12 b x - a^2
        let a2 = f.mul(a, a);
        memo.insert(3, red(vec![f.neg(&a2), f.mul(&c(12), b), f.mul(&c(6), a), f.zero(), c(3)]));
        // psi_4 / (2y) = 2(x^6 + 5a x^4 + 20 b x^3 - 5a^2 x^2 - 4ab x - 8b^2 - a^3)
        let p4 = vec![
            f.neg(&f.add(&f.mul(&c(8), &f.mul(b, b)), &f.mul(&a2, a))),
            f.neg(&f.mul(&c(4), &f.mul(a, b))),
            f.neg(&f.mul(&c(5), &a2)),
            f.mul(&c(20), b),
            f.mul(&c(5), a),
            f.zero(),
            f.one(),
        ];
        memo.insert(4, red(pscale(f, &p4, &c(2))));
        DivPolys { f, m, ff, memo }
    }

    fn mul(&self, a: &[F::E], b: &[F::E]) -> Vec<F::E> {
        match self.m {
            Some(m) => m.mul(a, b),
            None => pmul(self.f, a, b),
        }
    }

    fn get(&mut self, n: usize) -> Vec<F::E> {
        if let Some(v) = self.memo.get(&n) {
            return v.clone();
        }
        let m = n / 2;
        let v = if n % 2 == 1 {
            // psi_{2m+1} = psi_{m+2} psi_m^3 - psi_{m-1} psi_{m+1}^3, with
            // (2y)^4 on the term whose factors are even-indexed.
            let (a, b, c, d) = (self.get(m + 2), self.get(m), self.get(m - 1), self.get(m + 1));
            let mut t1 = self.mul(&a, &self.mul(&b, &self.mul(&b, &b)));
            let mut t2 = self.mul(&c, &self.mul(&d, &self.mul(&d, &d)));
            if m.is_multiple_of(2) {
                t1 = self.mul(&t1, &self.ff);
            } else {
                t2 = self.mul(&t2, &self.ff);
            }
            psub(self.f, &t1, &t2)
        } else {
            // psi_{2m} = psi_m (psi_{m+2} psi_{m-1}^2 - psi_{m-2} psi_{m+1}^2)
            let (a, b, c, d, e) = (self.get(m + 2), self.get(m - 1), self.get(m - 2), self.get(m + 1), self.get(m));
            let t = psub(self.f, &self.mul(&a, &self.mul(&b, &b)), &self.mul(&c, &self.mul(&d, &d)));
            self.mul(&e, &t)
        };
        self.memo.insert(n, v.clone());
        v
    }
}

/// The l-division polynomial psi_l (l odd), degree (l^2 - 1)/2.
pub fn division_polynomial<F: Field>(f: &F, a: &F::E, b: &F::E, l: usize) -> Vec<F::E> {
    DivPolys::new(f, a, b, None).get(l)
}

// ------------------------------------------------------ points over R[y]

/// (X, Y) standing for the point (X(x), Y(x) y) of E over R[y], R = F[x]/(h).
type RPt<E> = (Vec<E>, Vec<E>);

enum Step<T, E> {
    Ok(T),
    /// A proper factor of h was found.
    Split(Vec<E>),
}

/// [k]P for the generic point P = (x, y), 1 <= k < l, by the division
/// polynomials, as (X : Y : Z) with x = X/Z, y-coordinate (Y/Z) y; Z is a
/// unit of R (psi_k and 4(x^3 + a x + b) vanish at no nonzero l-torsion point).
fn mult_generic_proj<'a, F: Field>(f: &'a F, m: &PolyMod<'a, F>, a: &F::E, b: &F::E, k: usize) -> PPt<F::E> {
    let x = m.reduce(vec![f.zero(), f.one()]);
    if k == 1 {
        return (x, vec![f.one()], vec![f.one()]);
    }
    let mut dp = DivPolys::new(f, a, b, Some(m));
    let (pm2, pm1, p0, pp1, pp2) = (dp.get(k - 2), dp.get(k - 1), dp.get(k), dp.get(k + 1), dp.get(k + 2));
    let ff = dp.ff.clone();
    // F = 4(x^3 + a x + b) = (2y)^2.
    let cubic = vec![b.clone(), a.clone(), f.zero(), f.one()];
    let f4 = m.reduce(pscale(f, &cubic, &f.elt_u64(4)));
    let den2 = m.sqr(&p0);
    let den3 = m.mul(&den2, &p0);
    // k odd: X = x - F psi'_{k-1} psi'_{k+1} / psi_k^2,
    //        Y = (psi_{k+2} psi'^2_{k-1} - psi_{k-2} psi'^2_{k+1}) / psi_k^3;
    // k even: X = x - psi_{k-1} psi_{k+1} / (F psi'^2_k),
    //        Y = (psi'_{k+2} psi^2_{k-1} - psi'_{k-2} psi^2_{k+1}) / (F^2 psi'^3_k).
    let ynum = psub(f, &m.mul(&pp2, &m.sqr(&pm1)), &m.mul(&pm2, &m.sqr(&pp1)));
    let (xnum, xden, yden) = if k % 2 == 1 {
        (m.mul(&f4, &m.mul(&pm1, &pp1)), den2, den3)
    } else {
        (m.mul(&pm1, &pp1), m.mul(&den2, &f4), m.mul(&den3, &ff))
    };
    // Common denominator Z = xden yden.
    let xx = m.mul(&psub(f, &m.mul(&x, &xden), &xnum), &yden);
    let yy = m.mul(&ynum, &xden);
    let zz = m.mul(&xden, &yden);
    (xx, yy, zz)
}

/// [k]P in affine (X, Y) form (one inversion in R).
fn mult_generic<'a, F: Field>(f: &'a F, m: &PolyMod<'a, F>, a: &F::E, b: &F::E, k: usize) -> Step<RPt<F::E>, F::E> {
    let (xx, yy, zz) = mult_generic_proj(f, m, a, b, k);
    match m.inv(&zz) {
        Ok(i) => Step::Ok((m.mul(&xx, &i), m.mul(&yy, &i))),
        Err(g) => Step::Split(g),
    }
}

/// P1 + P2 in affine (X, Y) coordinates on f Y^2 = X^3 + a X + b; `None` = O.
fn add_generic<'a, F: Field>(
    f: &'a F,
    m: &PolyMod<'a, F>,
    a: &F::E,
    fx: &[F::E],
    p1: &RPt<F::E>,
    p2: &RPt<F::E>,
) -> Step<Option<RPt<F::E>>, F::E> {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let dx = psub(f, x2, x1);
    let lambda = if dx.is_empty() {
        let sy = padd(f, y1, y2);
        if sy.is_empty() {
            return Step::Ok(None);
        }
        let dy = psub(f, y1, y2);
        if !dy.is_empty() {
            // Y1 = Y2 on some components, -Y2 on the others.
            let g = pgcd(f, &dy, m.modulus());
            return Step::Split(g);
        }
        // Doubling: lambda = (3X^2 + a) / (2 f Y).
        let num = padd(f, &pscale(f, &m.sqr(x1), &f.elt_u64(3)), std::slice::from_ref(a));
        let den = pscale(f, &m.mul(fx, y1), &f.elt_u64(2));
        match m.inv(&den) {
            Ok(i) => m.mul(&num, &i),
            Err(g) => return Step::Split(g),
        }
    } else {
        match m.inv(&dx) {
            Ok(i) => m.mul(&psub(f, y2, y1), &i),
            Err(g) => return Step::Split(g),
        }
    };
    let l2f = m.mul(&m.sqr(&lambda), fx);
    let x3 = psub(f, &psub(f, &l2f, x1), x2);
    let y3 = psub(f, &m.mul(&lambda, &psub(f, x1, &x3)), y1);
    Step::Ok(Some((x3, y3)))
}

/// Homogeneous projective point (U : V : Z) on v^2 = u^3 + A u + B over R.
type PPt<E> = (Vec<E>, Vec<E>, Vec<E>);

fn proj_double<'a, F: Field>(f: &'a F, m: &PolyMod<'a, F>, aa: &[F::E], p: &PPt<F::E>) -> PPt<F::E> {
    let (x, y, z) = p;
    let c = |n: u64| f.elt_u64(n);
    let w = padd(f, &m.mul(aa, &m.sqr(z)), &pscale(f, &m.sqr(x), &c(3)));
    let s = m.mul(y, z);
    let bb = m.mul(&m.mul(x, y), &s);
    let h = psub(f, &m.sqr(&w), &pscale(f, &bb, &c(8)));
    let x3 = pscale(f, &m.mul(&h, &s), &c(2));
    let y3 =
        psub(f, &m.mul(&w, &psub(f, &pscale(f, &bb, &c(4)), &h)), &pscale(f, &m.mul(&m.sqr(y), &m.sqr(&s)), &c(8)));
    let z3 = pscale(f, &m.mul(&m.sqr(&s), &s), &c(8));
    (x3, y3, z3)
}

/// P + Q for Q = (u2, v2, 1), P != +-Q on every component.
fn proj_add_affine<'a, F: Field>(f: &'a F, m: &PolyMod<'a, F>, p: &PPt<F::E>, q: &RPt<F::E>) -> PPt<F::E> {
    let (x1, y1, z1) = p;
    let (x2, y2) = q;
    let c2 = f.elt_u64(2);
    let u = psub(f, &m.mul(y2, z1), y1);
    let v = psub(f, &m.mul(x2, z1), x1);
    let v2 = m.sqr(&v);
    let v3 = m.mul(&v2, &v);
    let v2x1 = m.mul(&v2, x1);
    let w = psub(f, &psub(f, &m.mul(&m.sqr(&u), z1), &v3), &pscale(f, &v2x1, &c2));
    let x3 = m.mul(&v, &w);
    let y3 = psub(f, &m.mul(&u, &psub(f, &v2x1, &w)), &m.mul(&v3, y1));
    let z3 = m.mul(&v3, z1);
    (x3, y3, z3)
}

/// Why a trace was not established.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchoofError {
    /// The congruences need a prime above `Config::max_l`.
    TooLarge,
    /// The cubic has a repeated root, or the characteristic is 2 or 3.
    Unsupported(String),
    /// Several orders survive every point tried.
    Ambiguous(usize),
    /// The arithmetic contradicts itself (never a guess).
    Inconsistent(String),
}

/// t mod l for an odd prime l != p.
pub fn trace_mod_l<F: Field>(f: &F, a: &F::E, b: &F::E, l: usize) -> Result<usize, SchoofError> {
    let q = f.q();
    let mut h = division_polynomial(f, a, b, l);
    if h.len() != (l * l - 1) / 2 + 1 {
        return Err(SchoofError::Inconsistent(format!("psi_{l} has degree {} (l | p?)", h.len().saturating_sub(1))));
    }
    let k = (q % BigUint::from(l)).to_usize().expect("small");
    if k == 0 {
        return Err(SchoofError::Unsupported(format!("l = {l} divides q")));
    }
    let cubic = vec![b.clone(), a.clone(), f.zero(), f.one()];
    'restart: loop {
        let m = PolyMod::new(f, h.clone());
        let fx = m.reduce(cubic.clone());
        let x = m.reduce(vec![f.zero(), f.one()]);
        let xq = m.pow(&x, q);
        let yq = m.pow(&fx, &((q - 1u32) >> 1));
        let xq2 = m.compose(&xq, &xq);
        let yq2 = m.mul(&m.compose(&yq, &xq), &yq);
        // To v^2 = u^3 + A u + B with u = f X, v = f^2 Y, A = a f^2.
        let f2 = m.sqr(&fx);
        let aa = pscale(f, &f2, a);
        let to_uv = |p: &RPt<F::E>| (m.mul(&fx, &p.0), m.mul(&f2, &p.1));
        let (kx, ky, kz) = mult_generic_proj(f, &m, a, b, k);
        let kp: PPt<F::E> = (m.mul(&fx, &kx), m.mul(&f2, &ky), kz);
        let phi2 = to_uv(&(xq2.clone(), yq2.clone()));
        // S = phi^2(P) + [k]P. When x(phi^2 P) != x([k]P) on some component,
        // the projective sum is right on every such component and (0 : 0 : 0)
        // on the others (where phi^2 P = [k]P); tau is the same on every
        // component, so matching on the good ones decides it, and the others
        // match trivially. If S = O anywhere, tau = 0 and S = O everywhere,
        // which makes the x-difference vanish identically.
        let (us, vs, zs) = if !psub(f, &m.mul(&phi2.0, &kp.2), &kp.0).is_empty() {
            proj_add_affine(f, &m, &kp, &phi2)
        } else {
            let kp = match mult_generic(f, &m, a, b, k) {
                Step::Ok(p) => p,
                Step::Split(g) => {
                    h = smaller_factor(f, &h, g);
                    continue 'restart;
                }
            };
            let s = match add_generic(f, &m, a, &fx, &(xq2, yq2), &kp) {
                Step::Ok(None) => return Ok(0),
                Step::Ok(Some(s)) => s,
                Step::Split(g) => {
                    h = smaller_factor(f, &h, g);
                    continue 'restart;
                }
            };
            let (u, v) = to_uv(&s);
            (u, v, vec![f.one()])
        };
        let t1 = to_uv(&(xq, yq));
        let one = vec![f.one()];
        let mut cur: PPt<F::E> = (t1.0.clone(), t1.1.clone(), one);
        for tau in 1..=(l - 1) / 2 {
            if tau == 2 {
                cur = proj_double(f, &m, &aa, &(t1.0.clone(), t1.1.clone(), vec![f.one()]));
            } else if tau > 2 {
                cur = proj_add_affine(f, &m, &cur, &t1);
            }
            let (cu, cv, cz) = &cur;
            if psub(f, &m.mul(&us, cz), &m.mul(cu, &zs)).is_empty() {
                let vz = m.mul(&vs, cz);
                let cvz = m.mul(cv, &zs);
                if psub(f, &vz, &cvz).is_empty() {
                    return Ok(tau);
                }
                if padd(f, &vz, &cvz).is_empty() {
                    return Ok(l - tau);
                }
                return Err(SchoofError::Inconsistent(format!(
                    "x matches tau = {tau} mod {l} but y matches neither sign"
                )));
            }
        }
        return Err(SchoofError::Inconsistent(format!("no tau mod {l} satisfies the Frobenius relation")));
    }
}

/// The smaller-degree nontrivial factor among g and h / g.
fn smaller_factor<F: Field>(f: &F, h: &[F::E], g: Vec<F::E>) -> Vec<F::E> {
    let (cof, r) = pdivrem(f, h, &g);
    debug_assert!(r.is_empty());
    if cof.len() > 1 && cof.len() < g.len() {
        monic(f, cof)
    } else {
        g
    }
}

/// t mod 2.
pub fn trace_mod_2<F: Field>(f: &F, a: &F::E, b: &F::E) -> usize {
    let cubic = vec![b.clone(), a.clone(), f.zero(), f.one()];
    let m = PolyMod::new(f, cubic.clone());
    let xq = m.pow(&[f.zero(), f.one()], f.q());
    let d = psub(f, &xq, &[f.zero(), f.one()]);
    let g = pgcd(f, &d, &cubic);
    usize::from(g.len() <= 1)
}

// ------------------------------------------------------- affine points

type APt<E> = Option<(E, E)>;

struct Curve<'a, F: Field> {
    f: &'a F,
    a: F::E,
    b: F::E,
}

impl<F: Field> Curve<'_, F> {
    fn add(&self, p: &APt<F::E>, q: &APt<F::E>) -> APt<F::E> {
        let f = self.f;
        let (Some((x1, y1)), Some((x2, y2))) = (p, q) else {
            return if p.is_none() { q.clone() } else { p.clone() };
        };
        let lambda = if x1 == x2 {
            if f.is_zero(&f.add(y1, y2)) {
                return None;
            }
            let num = f.add(&f.mul(&f.elt_u64(3), &f.mul(x1, x1)), &self.a);
            f.mul(&num, &f.inv(&f.add(y1, y1))?)
        } else {
            f.mul(&f.sub(y2, y1), &f.inv(&f.sub(x2, x1))?)
        };
        let x3 = f.sub(&f.sub(&f.mul(&lambda, &lambda), x1), x2);
        let y3 = f.sub(&f.mul(&lambda, &f.sub(x1, &x3)), y1);
        Some((x3, y3))
    }

    fn neg(&self, p: &APt<F::E>) -> APt<F::E> {
        p.as_ref().map(|(x, y)| (x.clone(), self.f.neg(y)))
    }

    fn mul(&self, n: &BigUint, p: &APt<F::E>) -> APt<F::E> {
        let mut r = None;
        for i in (0..n.bits()).rev() {
            r = self.add(&r, &r);
            if n.bit(i) {
                r = self.add(&r, p);
            }
        }
        r
    }

    fn rhs(&self, x: &F::E) -> F::E {
        let f = self.f;
        f.add(&f.add(&f.mul(&f.mul(x, x), x), &f.mul(&self.a, x)), &self.b)
    }
}

/// Points tried before the residue-class search gives up.
const MAX_POINTS: usize = 24;
/// At most this many candidates per point (a point of small order).
const MAX_CANDIDATES: usize = 64;

/// The N = c (mod M) in [lo, hi] with N P = O, at most `MAX_CANDIDATES` (else
/// `None`); the count of progression terms is j_max + 1.
fn class_candidates<F: Field>(
    e: &Curve<'_, F>,
    p: &APt<F::E>,
    n0: &BigUint,
    modulus: &BigUint,
    j_max: u64,
) -> Option<Vec<BigUint>> {
    let s = e.mul(modulus, p);
    let r = e.mul(n0, p);
    let check = |j: u64| -> bool { e.mul(&(n0 + modulus * j), p).is_none() };
    let mut out = vec![];
    if s.is_none() {
        // M P = O: N0 P = O decides every term at once.
        if r.is_none() {
            if j_max as usize >= MAX_CANDIDATES {
                return None;
            }
            return Some((0..=j_max).map(|j| n0 + modulus * j).collect());
        }
        return Some(out);
    }
    let mb = (((j_max + 1).div_ceil(2) as f64).sqrt().ceil() as u64).max(1);
    // Baby steps: x(i S) -> i, i = 1..mb.
    let mut baby: HashMap<F::E, Vec<u64>> = HashMap::new();
    let mut cur = s.clone();
    for i in 1..=mb {
        match &cur {
            None => {
                // S has order o = i: the solutions are j0 + o Z, j0 < o.
                let o = i;
                let mut t = r.clone();
                for j0 in 0..o {
                    if t.is_none() {
                        let count = if j0 > j_max { 0 } else { (j_max - j0) / o + 1 };
                        if count as usize > MAX_CANDIDATES {
                            return None;
                        }
                        return Some((0..count).map(|c| n0 + modulus * (j0 + c * o)).collect());
                    }
                    t = e.add(&t, &s);
                }
                return Some(out);
            }
            Some((x, _)) => baby.entry(x.clone()).or_default().push(i),
        }
        cur = e.add(&cur, &s);
    }
    if cur.is_none() {
        // (mb + 1) S = O is covered by the giant steps only partially: treat
        // like the loop above.
        let o = mb + 1;
        let mut t = r.clone();
        for j0 in 0..o {
            if t.is_none() {
                let count = if j0 > j_max { 0 } else { (j_max - j0) / o + 1 };
                if count as usize > MAX_CANDIDATES {
                    return None;
                }
                return Some((0..count).map(|c| n0 + modulus * (j0 + c * o)).collect());
            }
            t = e.add(&t, &s);
        }
        return Some(out);
    }
    let w = 2 * mb + 1;
    let giant = e.mul(&BigUint::from(w), &s);
    let mut q = r;
    let mut g = 0u64;
    let mut found: Vec<u64> = vec![];
    loop {
        // q = R + g W S; q = +-i S gives j = g W -+ i.
        let base = g * w;
        match &q {
            None => found.push(base),
            Some((x, _)) => {
                if let Some(is) = baby.get(x) {
                    for &i in is {
                        if base >= i {
                            found.push(base - i);
                        }
                        found.push(base + i);
                    }
                }
            }
        }
        if base > j_max + mb {
            break;
        }
        q = e.add(&q, &giant);
        g += 1;
    }
    found.sort_unstable();
    found.dedup();
    for j in found.into_iter().filter(|&j| j <= j_max) {
        if check(j) {
            out.push(n0 + modulus * j);
            if out.len() > MAX_CANDIDATES {
                return None;
            }
        }
    }
    Some(out)
}

/// The search parameters.
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// The residue class may leave at most 2^bsgs_bits candidates.
    pub bsgs_bits: u32,
    /// The largest Schoof prime allowed.
    pub max_l: usize,
}

/// The import's parameters: classes of at most 2^36 terms, l <= 61.
pub const IMPORT: Config = Config { bsgs_bits: 36, max_l: 61 };

/// The odd primes l != p (with 2 first) Schoof needs for these parameters,
/// or `None` when one above `max_l` would be needed.
pub fn schoof_primes(q: &BigUint, p: &BigUint, cfg: &Config) -> Option<Vec<usize>> {
    let width: BigUint = (q * 16u32).sqrt() + 2u32; // >= 4 sqrt(q) + 1
    let target = (&width >> cfg.bsgs_bits) + 1u32;
    let mut primes = vec![2usize];
    let mut m = BigUint::from(2u32);
    let mut l = 3usize;
    while m < target {
        if (2..l).take_while(|d| d * d <= l).all(|d| !l.is_multiple_of(d)) && BigUint::from(l) != *p {
            if l > cfg.max_l {
                return None;
            }
            primes.push(l);
            m *= l;
        }
        l += 2;
    }
    Some(primes)
}

/// t = q + 1 - #E(F_q) for y^2 = x^3 + a x + b (char > 3, smooth).
pub fn frobenius_trace<F: Field>(f: &F, a: &F::E, b: &F::E, cfg: &Config) -> Result<BigInt, SchoofError> {
    let q = f.q().clone();
    let p = f.char_p();
    if *p <= BigUint::from(3u32) {
        return Err(SchoofError::Unsupported("characteristic 2 or 3".into()));
    }
    // 4a^3 + 27b^2 != 0
    let disc = f.add(&f.mul(&f.elt_u64(4), &f.mul(&f.mul(a, a), a)), &f.mul(&f.elt_u64(27), &f.mul(b, b)));
    if f.is_zero(&disc) {
        return Err(SchoofError::Unsupported("singular curve".into()));
    }
    let primes = schoof_primes(&q, p, cfg).ok_or(SchoofError::TooLarge)?;
    // CRT: t = c (mod M).
    let (mut c, mut modulus) = (BigUint::zero(), BigUint::one());
    for &l in &primes {
        let r = if l == 2 { trace_mod_2(f, a, b) } else { trace_mod_l(f, a, b, l)? };
        let lb = BigUint::from(l);
        // c' = c + M * ((r - c) M^-1 mod l)
        let mi = (&modulus % &lb).modinv(&lb).expect("coprime moduli");
        let diff = (BigUint::from(r) + &lb - (&c % &lb)) % &lb;
        let k = (diff * mi) % &lb;
        c += &modulus * k;
        modulus *= &lb;
    }
    // N = q + 1 - t = n0 (mod M) in [lo, hi].
    let width = (&q * 16u32).sqrt() + 1u32; // floor(4 sqrt q) + 1 >= 2 floor(2 sqrt q)
    let half = &width >> 1u32;
    let qp1 = &q + 1u32;
    let lo = if half >= qp1 { BigUint::zero() } else { &qp1 - &half };
    let hi = &qp1 + &half;
    let n_res = (&qp1 + &modulus - (&c % &modulus)) % &modulus;
    let n0 = &lo + ((&n_res + &modulus - (&lo % &modulus)) % &modulus);
    if n0 > hi {
        return Err(SchoofError::Inconsistent("no order in the Hasse interval has the residue".into()));
    }
    let j_max = ((&hi - &n0) / &modulus).to_u64().ok_or(SchoofError::TooLarge)?;
    let e = Curve { f, a: a.clone(), b: b.clone() };
    let mut cands: Option<Vec<BigUint>> = None;
    let (mut tried, mut index) = (0usize, 1u64);
    while tried < MAX_POINTS {
        let x = f.index_elt(index);
        index += 1;
        if BigUint::from(index) > q {
            break;
        }
        let Some(y) = fsqrt(f, &e.rhs(&x)) else { continue };
        if f.is_zero(&y) {
            continue;
        }
        let pt = Some((x, y));
        debug_assert!(e.neg(&pt) != pt);
        tried += 1;
        let Some(s) = class_candidates(&e, &pt, &n0, &modulus, j_max) else { continue };
        let merged: Vec<BigUint> = match cands {
            None => s,
            Some(prev) => prev.into_iter().filter(|n| s.contains(n)).collect(),
        };
        match merged.len() {
            0 => {
                return Err(SchoofError::Inconsistent(
                    "no order of the residue class annihilates a point (wrong congruences)".into(),
                ))
            }
            1 => return Ok(BigInt::from(qp1) - BigInt::from(merged[0].clone())),
            _ => cands = Some(merged),
        }
    }
    Err(SchoofError::Ambiguous(cands.map_or(0, |c| c.len())))
}

/// (A, B) of y^2 = x^3 + A x + B isomorphic to y^2 = x^3 + a2 x^2 + a4 x + a6
/// (x -> x - a2/3; char > 3).
pub fn short_weierstrass<F: Field>(f: &F, a2: &F::E, a4: &F::E, a6: &F::E) -> (F::E, F::E) {
    let i3 = f.inv(&f.elt_u64(3)).expect("char > 3");
    let i27 = f.inv(&f.elt_u64(27)).expect("char > 3");
    let a2sq = f.mul(a2, a2);
    let aa = f.sub(a4, &f.mul(&a2sq, &i3));
    let bb = f.add(&f.sub(a6, &f.mul(&f.mul(a2, a4), &i3)), &f.mul(&f.mul(&f.elt_u64(2), &f.mul(&a2sq, a2)), &i27));
    (aa, bb)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn brute_fp(p: u64, a: u64, b: u64) -> i64 {
        let mut n = 1i64;
        let sq: Vec<u64> = {
            let mut c = vec![0u64; p as usize];
            for y in 0..p {
                c[(y * y % p) as usize] += 1;
            }
            c
        };
        for x in 0..p {
            n += sq[((x * x % p * x + a * x + b) % p) as usize] as i64;
        }
        p as i64 + 1 - n
    }

    fn is_prime(n: u64) -> bool {
        n >= 2 && (2..).take_while(|d| d * d <= n).all(|d| !n.is_multiple_of(d))
    }

    /// Force Schoof to determine t alone (no BSGS search).
    const ALL_SCHOOF: Config = Config { bsgs_bits: 0, max_l: 200 };

    #[test]
    fn kronecker_matches_schoolbook() {
        let f = PrimeField::new(BigUint::from(1_000_000_007u64));
        let a: Vec<BigUint> = (0..40u64).map(|i| f.elt_u64(i * i * 7919 + 3)).collect();
        let b: Vec<BigUint> = (0..33u64).map(|i| f.elt_u64(i * 104_729 + 11)).collect();
        assert_eq!(f.poly_mul(&a, &b), schoolbook(&f, &a, &b));
        assert_eq!(f.poly_mul(&a, &a), schoolbook(&f, &a, &a));
        let e = ExtField::new(1_000_003, &[2, 0, 5]).unwrap();
        let a: Vec<Ek> = (0..30u64).map(|i| e.elt(&[i * 31 + 1, i * i, 999_999 - i])).collect();
        let b: Vec<Ek> = (0..25u64).map(|i| e.elt(&[i + 7, 3 * i, i * i * i])).collect();
        assert_eq!(e.poly_mul(&a, &b), schoolbook(&e, &a, &b));
    }

    #[test]
    fn polymod_reduce_inverse_and_compose() {
        let f = PrimeField::new(BigUint::from(10_007u64));
        let h: Vec<BigUint> = (0..30u64).map(|i| f.elt_u64(i * i + 5)).collect();
        let m = PolyMod::new(&f, h.clone());
        let a: Vec<BigUint> = (0..55u64).map(|i| f.elt_u64(i * 13 + 1)).collect();
        assert_eq!(m.reduce(a.clone()), pdivrem(&f, &a, &m.h).1);
        let b: Vec<BigUint> = (0..20u64).map(|i| f.elt_u64(3 * i + 2)).collect();
        // compose(g, b) equals Horner evaluation.
        let g: Vec<BigUint> = (0..37u64).map(|i| f.elt_u64(i * 101 + 9)).collect();
        let mut horner = vec![];
        for c in g.iter().rev() {
            horner = padd(&f, &m.mul(&horner, &b), std::slice::from_ref(c));
        }
        assert_eq!(m.compose(&g, &b), horner);
        match m.inv(&b) {
            Ok(i) => assert_eq!(m.mul(&i, &b), vec![f.one()]),
            Err(g) => assert!(g.len() > 1),
        }
        // pow agrees with repeated multiplication.
        let mut r = vec![f.one()];
        for _ in 0..77 {
            r = m.mul(&r, &b);
        }
        assert_eq!(m.pow(&b, &BigUint::from(77u32)), r);
    }

    #[test]
    fn division_polynomial_roots_are_torsion() {
        // Over F_p every x-root of psi_l whose point is rational has order l.
        let p = 1009u64;
        let f = PrimeField::new(BigUint::from(p));
        for (a, b) in [(3u64, 7u64), (1, 1), (0, 5), (11, 0)] {
            let e = Curve { f: &f, a: f.elt_u64(a), b: f.elt_u64(b) };
            for l in [3usize, 5, 7] {
                let psi = division_polynomial(&f, &e.a, &e.b, l);
                assert_eq!(psi.len(), (l * l - 1) / 2 + 1);
                for x in 0..p {
                    let xe = f.elt_u64(x);
                    let v = psi.iter().rev().fold(f.zero(), |acc, c| f.add(&f.mul(&acc, &xe), c));
                    let Some(y) = fsqrt(&f, &e.rhs(&xe)) else { continue };
                    let pt = Some((xe, y));
                    let torsion = e.mul(&BigUint::from(l), &pt).is_none();
                    assert_eq!(f.is_zero(&v), torsion, "a={a} b={b} l={l} x={x}");
                }
            }
        }
    }

    #[test]
    fn generic_multiples_match_affine() {
        // [k](x, y) from the division polynomials, specialised at a rational
        // l-torsion point, equals the affine multiple.
        let p = 1009u64;
        let f = PrimeField::new(BigUint::from(p));
        let (a, b) = (f.elt_u64(3), f.elt_u64(7));
        let e = Curve { f: &f, a: a.clone(), b: b.clone() };
        let mut checked = 0;
        for l in [5usize, 7, 11, 13] {
            let psi = division_polynomial(&f, &a, &b, l);
            let m = PolyMod::new(&f, psi);
            for k in 1..l {
                let Step::Ok((xk, yk)) = mult_generic(&f, &m, &a, &b, k) else { continue };
                for x in 0..p {
                    let xe = f.elt_u64(x);
                    let Some(y) = fsqrt(&f, &e.rhs(&xe)) else { continue };
                    let pt = Some((xe.clone(), y.clone()));
                    if f.is_zero(&y) || e.mul(&BigUint::from(l), &pt).is_some() {
                        continue;
                    }
                    let ev = |v: &[BigUint]| v.iter().rev().fold(f.zero(), |acc, c| f.add(&f.mul(&acc, &xe), c));
                    let want = e.mul(&BigUint::from(k), &pt).unwrap();
                    assert_eq!((ev(&xk), f.mul(&ev(&yk), &y)), want, "l={l} k={k} x={x}");
                    checked += 1;
                }
            }
        }
        assert!(checked > 0);
    }

    #[test]
    fn schoof_alone_matches_brute_force_fp() {
        let mut n = 0;
        for p in [101u64, 211, 1009, 4099, 10_007, 65_537] {
            for (a, b) in [(1u64, 1u64), (2, 3), (0, 7), (5, 0), (p - 3, 11), (1234 % p, 4321 % p)] {
                if (4 * a * a % p * a + 27 * b * b) % p == 0 {
                    continue;
                }
                let f = PrimeField::new(BigUint::from(p));
                let t = frobenius_trace(&f, &f.elt_u64(a), &f.elt_u64(b), &ALL_SCHOOF).unwrap();
                assert_eq!(t, BigInt::from(brute_fp(p, a, b)), "p={p} a={a} b={b}");
                n += 1;
            }
        }
        assert!(n > 30);
    }

    #[test]
    fn residue_class_search_matches_brute_force() {
        for p in (100_000u64..100_400).filter(|&p| is_prime(p)).take(6) {
            let f = PrimeField::new(BigUint::from(p));
            for (a, b) in [(1u64, 1u64), (7, 0), (0, 3)] {
                let cfg = Config { bsgs_bits: 6, max_l: 200 };
                let t = frobenius_trace(&f, &f.elt_u64(a), &f.elt_u64(b), &cfg).unwrap();
                assert_eq!(t, BigInt::from(brute_fp(p, a, b)), "p={p} a={a} b={b}");
            }
        }
    }

    /// #E(GF(p^k)) by listing x and the quadratic character (Euler).
    fn brute_ext(e: &ExtField, a: &Ek, b: &Ek) -> BigInt {
        let q = e.q().to_u64().unwrap();
        let half = (e.q() - 1u32) >> 1;
        let mut n = 1i64;
        for i in 0..q {
            let x = e.index_elt(i);
            let r = e.add(&e.add(&e.mul(&e.mul(&x, &x), &x), &e.mul(a, &x)), b);
            n += if e.is_zero(&r) {
                1
            } else if fpow(e, &r, &half) == e.one() {
                2
            } else {
                0
            };
        }
        BigInt::from(q as i64 + 1 - n)
    }

    #[test]
    fn schoof_matches_brute_force_extension() {
        // Irreducible moduli: t^3 + t^2 + 3, t^4 + t^3 + t^2 + 1, t^2 + t + 2, t^3 + t + 4.
        let fields = [(7u64, vec![3i128, 0, 1]), (5, vec![1, 0, 1, 1]), (13, vec![2, 1]), (11, vec![4, 1, 0])];
        let mut n = 0;
        for (p, low) in fields {
            let e = ExtField::new(p, &low).unwrap();
            // every nonzero element must be invertible (the modulus is irreducible)
            let q = e.q().to_u64().unwrap();
            assert!((1..q).all(|i| e.inv(&e.index_elt(i)).is_some_and(|v| e.mul(&v, &e.index_elt(i)) == e.one())));
            for (ai, bi) in [(1u64, 2u64), (p + 3, 5 * p + 1), (2 * p, 1), (7, p * p + 2)] {
                let (a, b) = (e.index_elt(ai % q), e.index_elt(bi % q));
                let disc =
                    e.add(&e.mul(&e.elt_u64(4), &e.mul(&e.mul(&a, &a), &a)), &e.mul(&e.elt_u64(27), &e.mul(&b, &b)));
                if e.is_zero(&disc) {
                    continue;
                }
                let want = brute_ext(&e, &a, &b);
                assert_eq!(frobenius_trace(&e, &a, &b, &ALL_SCHOOF).unwrap(), want, "p={p} low={low:?}");
                // With a residue-class search: the right trace or no answer, never a wrong one.
                let cfg = Config { bsgs_bits: 4, max_l: 200 };
                match frobenius_trace(&e, &a, &b, &cfg) {
                    Ok(t) => assert_eq!(t, want, "bsgs p={p} low={low:?} {ai} {bi}"),
                    Err(err) => assert!(matches!(err, SchoofError::Ambiguous(_)), "{err:?}"),
                }
                n += 1;
            }
        }
        assert!(n >= 12, "{n}");
    }

    #[test]
    fn secp112r1_order_from_sec2() {
        // SEC 2 v1.0, secp112r1: n = DB7C2ABF62E35E7628DFAC6561C5, h = 1.
        let hex = |s: &str| BigUint::parse_bytes(s.as_bytes(), 16).unwrap();
        let p = hex("DB7C2ABF62E35E668076BEAD208B");
        let f = PrimeField::new(p.clone());
        let a = hex("DB7C2ABF62E35E668076BEAD2088");
        let b = hex("659EF8BA043916EEDE8911702B22");
        let n = hex("DB7C2ABF62E35E7628DFAC6561C5");
        let t = frobenius_trace(&f, &a, &b, &IMPORT).unwrap();
        assert_eq!(t, BigInt::from(p) + 1 - BigInt::from(n));
    }

    #[test]
    fn short_weierstrass_preserves_the_count() {
        let p = 1009u64;
        let f = PrimeField::new(BigUint::from(p));
        let (a2, a4, a6) = (5u64, 17u64, 400u64);
        let (aa, bb) = short_weierstrass(&f, &f.elt_u64(a2), &f.elt_u64(a4), &f.elt_u64(a6));
        let mut n = 1i64;
        for x in 0..p {
            for y in 0..p {
                if (y * y) % p == (x * x % p * x + a2 * x % p * x + a4 * x + a6) % p {
                    n += 1;
                }
            }
        }
        let t = frobenius_trace(&f, &aa, &bb, &ALL_SCHOOF).unwrap();
        assert_eq!(t, BigInt::from(p as i64 + 1 - n));
    }

    #[test]
    fn primes_respect_the_budget() {
        let q = BigUint::one() << 192u32;
        let pr = schoof_primes(&q, &q, &IMPORT).unwrap();
        assert_eq!(pr[0], 2);
        assert!(*pr.last().unwrap() <= IMPORT.max_l);
        let q = BigUint::one() << 400u32;
        assert!(schoof_primes(&q, &q, &IMPORT).is_none());
    }
}

#[cfg(test)]
mod timing {
    use super::*;
    use std::time::Instant;

    fn next_prime(mut n: BigUint) -> BigUint {
        while !crate::ec::is_probable_prime(&BigInt::from(n.clone())).0 {
            n += 1u32;
        }
        n
    }

    /// NIST P-192 (FIPS 186-4 D.1.2.1), order as published; 192 bits is past
    /// the import's limit, so this is a timing and reference run only:
    /// `cargo test --release -- --ignored p192 --nocapture`.
    #[test]
    #[ignore]
    fn p192_order_from_fips186() {
        let hex = |s: &str| BigUint::parse_bytes(s.as_bytes(), 16).unwrap();
        let one = BigUint::one();
        let p = (&one << 192u32) - (&one << 64u32) - 1u32;
        let f = PrimeField::new(p.clone());
        let b = hex("64210519E59C80E70FA7E9AB72243049FEB8DEECC146B9B1");
        let n = hex("FFFFFFFFFFFFFFFFFFFFFFFF99DEF836146BC9B1B4D22831");
        let t0 = Instant::now();
        let t = frobenius_trace(&f, &(&p - 3u32), &b, &IMPORT).unwrap();
        eprintln!("P-192: {:?}", t0.elapsed());
        assert_eq!(t, BigInt::from(p) + 1 - BigInt::from(n));
    }

    /// Rabin's irreducibility test of t^k + low over F_p.
    fn irreducible(p: u64, low: &[i128]) -> bool {
        let f = PrimeField::new(BigUint::from(p));
        let k = low.len();
        let mut m: Vec<BigUint> = low.iter().map(|&c| BigUint::from(c.rem_euclid(p as i128) as u64)).collect();
        m.push(BigUint::one());
        let pm = PolyMod::new(&f, m.clone());
        let x = vec![f.zero(), f.one()];
        let frob = |e: usize| pm.pow(&x, &num_traits::pow(BigUint::from(p), e));
        if !psub(&f, &frob(k), &x).is_empty() {
            return false;
        }
        (2..=k)
            .filter(|r| k.is_multiple_of(*r) && (2..*r).all(|d| !r.is_multiple_of(d)))
            .all(|r| pgcd(&f, &psub(&f, &frob(k / r), &x), &m).len() == 1)
    }

    /// `SCHOOF_SIZES="96 128 p32k4" cargo test --release -- --ignored size_timing --nocapture`
    #[test]
    #[ignore]
    fn size_timing() {
        let spec = std::env::var("SCHOOF_SIZES").unwrap_or_else(|_| "96".into());
        for item in spec.split_whitespace() {
            let t0 = Instant::now();
            let t = if let Some(rest) = item.strip_prefix('p') {
                let (pb, k) = rest.split_once('k').unwrap();
                let (pb, k): (u32, usize) = (pb.parse().unwrap(), k.parse().unwrap());
                let p =
                    next_prime((BigUint::one() << (pb - 1)) + (BigUint::one() << (pb - 3)) + 1u32).to_u64().unwrap();
                let low = (1i128..)
                    .map(|c| {
                        let mut v = vec![0i128; k];
                        v[0] = c;
                        v[1] = 1;
                        v
                    })
                    .find(|v| irreducible(p, v))
                    .unwrap();
                let e = ExtField::new(p, &low).unwrap();
                let t0b = Instant::now();
                let r = frobenius_trace(&e, &e.index_elt(3 * p + 1), &e.index_elt(7 + p * p), &IMPORT);
                eprintln!("{item}: {:?} {r:?}", t0b.elapsed());
                continue;
            } else {
                let bits: u32 = item.parse().unwrap();
                let p = next_prime((BigUint::one() << (bits - 1)) + BigUint::from(987_654_321u64));
                let f = PrimeField::new(p);
                frobenius_trace(&f, &f.elt_u64(3), &f.elt_u64(1234567), &IMPORT)
            };
            eprintln!("{item}: {:?} {t:?}", t0.elapsed());
        }
    }
}
