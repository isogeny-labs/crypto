//! Exact L-polynomials of the Jacobians of the cover sources (the genus-2 and
//! genus-3 curves H of `curve_morphism` rows of kind `hyperelliptic_pullback`).
//!
//! Each source gets an `isogeny_class_fq` row (label `g.q.a1_..._ag`, built as
//! for every other class), `abelian_variety.isogeny_class_id` of its Jacobian,
//! and a `curve_zeta` row with #H(F_{q^k}), k = 1..g, the same shape as the
//! genus-1 rows of the registry (one count per degree of the class). Nothing
//! is ever deleted; a source already linked and with a zeta row is left alone.
//!
//! # Mathematics
//!
//! **`prime_quadratic_pullback_v1`, genus 2, q = p^r, p > 3.** H: v^2 = G(u^2)
//! with G(x) = x^3 + g2 x^2 + g1 x + g0 (the sextic is F(u^2 + c), expanded;
//! g0 = F(c) != 0). Jac(H) is isogenous over F_q to E x E', where
//!
//! * E: v^2 = G(x), the target of the cover (G is F translated by c), and
//! * E': Y^2 = x G(x), the other quotient of H by an involution; with x = 1/z
//!   it is the smooth cubic V^2 = Z^3 + g1 Z^2 + g0 g2 Z + g0^2 (Z = g0 z).
//!
//! So L(T) = (1 - tT + qT^2)(1 - t'T + qT^2) = 1 + a1 T + a2 T^2 + q a1 T^3 +
//! q^2 T^4 with a1 = -(t + t') and a2 = 2q + t t'. Both traces come from group
//! orders found by baby-step giant-step in the Hasse interval: for random
//! points P the candidates N with N P = O are intersected until one is left,
//! and since the true order is always among them, a unique survivor is the
//! order (no guess). When p < 2^22 the count #H(F_p) = p + 2 + sum_u chi(f(u))
//! is also taken exhaustively and must give the same a1; when p < 2^12,
//! #H(F_{p^2}) is counted exhaustively too (over F_{p^2} through the norm to
//! F_p) and must give the same a2, a2 = (#H(F_{p^2}) - p^2 - 1 + a1^2)/2.
//! In that range the exhaustive counts are the stored result (method
//! "exhaustive count"); above it, a2 is derived from the traces (method
//! "derived"). The trace of the target recorded in ecdb is the cross-check:
//! T^2 - tT + q must divide T^4 + a1 T^3 + a2 T^2 + q a1 T + q^2.
//!
//! **`binary_cubic_pullback_v1`, genus 3, q = 2^m.** H: y^2 + u^2 y = u^7 +
//! a u^4 + d u (one point at infinity: deg f = 2g + 1, deg h <= g). With
//! s = y / u^2 it is s^2 + s = u^3 + a + d u^-3 for u != 0, so
//! #H(F_Q) = Q + 1 + sum_{u != 0} (-1)^Tr(u^3 + a + d u^-3). The counts over
//! F_{2^m}, F_{2^2m} and F_{2^3m} need Q <= 2^24 elements, i.e. m <= 8 for
//! all three and m <= 12 for the first two; F_{2^km} is built from an
//! irreducible polynomial of degree km, with a, d embedded through a root of
//! the m-bit modulus. With the three counts, L follows from Newton's
//! identities (method "exhaustive count"). With two, L = L_E L_B, where E is
//! the elliptic quotient (y^2 + xy = x^3 + a x^2 + d^2, counted exhaustively)
//! and B is the Prym surface, whose two unknown coefficients the two counts
//! determine (method "derived"). A larger m is skipped, never guessed.
//!
//! # Report
//!
//! `cover_lpoly_computed`, `cover_lpoly_skipped` (with `cover_lpoly_skipped_*`
//! per reason), `cover_lpoly_crosscheck_failures` (each also a problem and
//! never inserted), `cover_lpoly_classes`, `cover_lpoly_jacobians_linked`,
//! `cover_lpoly_zeta`.

use super::covers::fq_from_parts;
use super::{bulk, bulk1, Ctx, Report};
use crate::ec::is_probable_prime;
use crate::field::{Elt, Encoding, Fq};
use crate::math::weil_label;
use anyhow::Result;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{ToPrimitive, Zero};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

/// Largest prime field (bits of p) point-counted by baby-step giant-step.
pub const MAX_FP_BITS: u64 = 62;
/// Largest extension field (bits of q) counted by baby-step giant-step.
pub const MAX_EXT_BITS: u64 = 48;
/// Exhaustive #H(F_p) cross-check below this p.
pub const DIRECT_FP_MAX: u64 = 1 << 22;
/// Exhaustive #H(F_{p^2}) below this p.
pub const DIRECT_FP2_MAX: u64 = 1 << 12;
/// Largest binary field extension (bits of Q) counted exhaustively.
pub const MAX_BINARY_COUNT_BITS: u32 = 24;
/// Random points tried before baby-step giant-step gives up.
const MAX_POINTS: usize = 48;

pub const PRIME_CONSTRUCTION: &str = "prime_quadratic_pullback_v1";
pub const BINARY_CONSTRUCTION: &str = "binary_cubic_pullback_v1";

// ------------------------------------------------------------------ fields

/// The few field operations the group-order search needs.
pub trait Gf {
    type E: Clone + Eq + Hash;
    /// q = |F|.
    fn order(&self) -> u128;
    fn zero(&self) -> Self::E;
    fn one(&self) -> Self::E;
    /// The element with base-p digits n (every element once for n < q).
    fn elt_of_index(&self, n: u128) -> Self::E;
    /// The image of an integer.
    fn small(&self, n: i64) -> Self::E;
    fn add(&self, a: &Self::E, b: &Self::E) -> Self::E;
    fn sub(&self, a: &Self::E, b: &Self::E) -> Self::E;
    fn mul(&self, a: &Self::E, b: &Self::E) -> Self::E;
    /// The inverse; zero for zero (callers never divide by zero).
    fn inv(&self, a: &Self::E) -> Self::E;
    fn is_zero(&self, a: &Self::E) -> bool;
}

/// F_p, p < 2^62, on machine words.
pub struct Fp64 {
    pub p: u64,
}

impl Gf for Fp64 {
    type E = u64;
    fn order(&self) -> u128 {
        self.p as u128
    }
    fn zero(&self) -> u64 {
        0
    }
    fn one(&self) -> u64 {
        1
    }
    fn elt_of_index(&self, n: u128) -> u64 {
        (n % self.p as u128) as u64
    }
    fn small(&self, n: i64) -> u64 {
        n.rem_euclid(self.p as i64) as u64
    }
    fn add(&self, a: &u64, b: &u64) -> u64 {
        let s = a + b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    fn sub(&self, a: &u64, b: &u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.p - b
        }
    }
    fn mul(&self, a: &u64, b: &u64) -> u64 {
        if self.p < 1 << 32 {
            a * b % self.p
        } else {
            (*a as u128 * *b as u128 % self.p as u128) as u64
        }
    }
    fn inv(&self, a: &u64) -> u64 {
        let (mut r0, mut r1) = (self.p as i128, *a as i128);
        let (mut t0, mut t1) = (0i128, 1i128);
        while r1 != 0 {
            let q = r0 / r1;
            (r0, r1) = (r1, r0 - q * r1);
            (t0, t1) = (t1, t0 - q * t1);
        }
        if r0 != 1 {
            return 0;
        }
        t0.rem_euclid(self.p as i128) as u64
    }
    fn is_zero(&self, a: &u64) -> bool {
        *a == 0
    }
}

/// GF(p^k) on the catalogue's exact arithmetic (a few hundred operations per
/// curve, so its speed does not matter for q < 2^48).
pub struct ExtField<'a> {
    pub fq: &'a Fq,
    q: u128,
    p: u128,
}

impl<'a> ExtField<'a> {
    /// `None` when q does not fit 48 bits.
    pub fn new(fq: &'a Fq) -> Option<ExtField<'a>> {
        let p = fq.p.to_u128()?;
        let q = num_traits::pow(fq.p.clone(), fq.degree()).to_u128()?;
        (q < 1u128 << MAX_EXT_BITS).then_some(ExtField { fq, q, p })
    }
}

impl Gf for ExtField<'_> {
    type E = Elt;
    fn order(&self) -> u128 {
        self.q
    }
    fn zero(&self) -> Elt {
        self.fq.zero()
    }
    fn one(&self) -> Elt {
        self.fq.from_int(1)
    }
    fn elt_of_index(&self, mut n: u128) -> Elt {
        (0..self.fq.degree())
            .map(|_| {
                let d = n % self.p;
                n /= self.p;
                BigInt::from(d)
            })
            .collect()
    }
    fn small(&self, n: i64) -> Elt {
        self.fq.from_int(n)
    }
    fn add(&self, a: &Elt, b: &Elt) -> Elt {
        self.fq.add(a, b)
    }
    fn sub(&self, a: &Elt, b: &Elt) -> Elt {
        self.fq.sub(a, b)
    }
    fn mul(&self, a: &Elt, b: &Elt) -> Elt {
        self.fq.mul(a, b)
    }
    fn inv(&self, a: &Elt) -> Elt {
        self.fq.inv(a).unwrap_or_else(|_| self.fq.zero())
    }
    fn is_zero(&self, a: &Elt) -> bool {
        self.fq.is_zero(a)
    }
}

fn gpow<F: Gf>(f: &F, a: &F::E, mut e: u128) -> F::E {
    let mut r = f.one();
    let mut b = a.clone();
    while e > 0 {
        if e & 1 == 1 {
            r = f.mul(&r, &b);
        }
        b = f.mul(&b, &b);
        e >>= 1;
    }
    r
}

/// A square root in F_q (q odd) by Tonelli-Shanks; `None` for a non-square.
fn gsqrt<F: Gf>(f: &F, a: &F::E) -> Option<F::E> {
    if f.is_zero(a) {
        return Some(f.zero());
    }
    let q = f.order();
    let one = f.one();
    if gpow(f, a, (q - 1) / 2) != one {
        return None;
    }
    let (mut e, mut s) = (0u32, q - 1);
    while s % 2 == 0 {
        s /= 2;
        e += 1;
    }
    let mut z_index = 2u128;
    let z = loop {
        let z = f.elt_of_index(z_index);
        if !f.is_zero(&z) && gpow(f, &z, (q - 1) / 2) != one {
            break z;
        }
        z_index += 1;
    };
    let mut c = gpow(f, &z, s);
    let mut x = gpow(f, a, s.div_ceil(2));
    let mut t = gpow(f, a, s);
    let mut m = e;
    while t != one {
        let mut i = 0u32;
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

fn isqrt(n: u128) -> u128 {
    if n < 2 {
        return n;
    }
    let mut x = (n as f64).sqrt() as u128;
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

// ------------------------------------------------- group order by BSGS

type Pt<E> = Option<(E, E)>;

/// y^2 = x^3 + a2 x^2 + a4 x + a6 over F_q, q odd.
struct Cubic<'a, F: Gf> {
    f: &'a F,
    a2: F::E,
    a4: F::E,
    a6: F::E,
    two: F::E,
    three: F::E,
}

impl<'a, F: Gf> Cubic<'a, F> {
    fn new(f: &'a F, a2: F::E, a4: F::E, a6: F::E) -> Self {
        Cubic { f, a2, a4, a6, two: f.small(2), three: f.small(3) }
    }

    fn rhs(&self, x: &F::E) -> F::E {
        let f = self.f;
        let x2 = f.mul(x, x);
        let t = f.add(&f.mul(&x2, x), &f.mul(&self.a2, &x2));
        f.add(&t, &f.add(&f.mul(&self.a4, x), &self.a6))
    }

    fn neg(&self, p: &Pt<F::E>) -> Pt<F::E> {
        p.as_ref().map(|(x, y)| (x.clone(), self.f.sub(&self.f.zero(), y)))
    }

    fn add(&self, p: &Pt<F::E>, q: &Pt<F::E>) -> Pt<F::E> {
        let f = self.f;
        let (Some((x1, y1)), Some((x2, y2))) = (p, q) else {
            return if p.is_none() { q.clone() } else { p.clone() };
        };
        let lambda = if x1 == x2 {
            if f.is_zero(&f.add(y1, y2)) {
                return None;
            }
            let x1sq = f.mul(x1, x1);
            let num = f.add(&f.add(&f.mul(&self.three, &x1sq), &f.mul(&self.two, &f.mul(&self.a2, x1))), &self.a4);
            f.mul(&num, &f.inv(&f.mul(&self.two, y1)))
        } else {
            f.mul(&f.sub(y2, y1), &f.inv(&f.sub(x2, x1)))
        };
        let x3 = f.sub(&f.sub(&f.sub(&f.mul(&lambda, &lambda), &self.a2), x1), x2);
        let y3 = f.sub(&f.mul(&lambda, &f.sub(x1, &x3)), y1);
        Some((x3, y3))
    }

    fn mul(&self, mut n: u128, p: &Pt<F::E>) -> Pt<F::E> {
        let mut acc: Pt<F::E> = None;
        let mut b = p.clone();
        while n > 0 {
            if n & 1 == 1 {
                acc = self.add(&acc, &b);
            }
            b = self.add(&b, &b);
            n >>= 1;
        }
        acc
    }

    /// Nonzero discriminant of x^3 + a2 x^2 + a4 x + a6.
    fn is_smooth(&self) -> bool {
        let f = self.f;
        let (a, b, c) = (&self.a2, &self.a4, &self.a6);
        let m = |x: &F::E, y: &F::E| f.mul(x, y);
        let n = |k: i64, x: &F::E| f.mul(&f.small(k), x);
        let (a2, b2, c2) = (m(a, a), m(b, b), m(c, c));
        let (b3, a3) = (m(&b2, b), m(&a2, a));
        let mut d = m(&a2, &b2);
        d = f.sub(&d, &n(4, &b3));
        d = f.sub(&d, &n(4, &m(&a3, c)));
        d = f.sub(&d, &n(27, &c2));
        d = f.add(&d, &n(18, &m(&m(a, b), c)));
        !f.is_zero(&d)
    }
}

/// Why a group order was not established.
#[derive(Debug, PartialEq, Eq)]
pub enum Bsgs {
    /// Several orders survive every point tried (small exponent group).
    Ambiguous(usize),
    /// No order fits: the curve is singular or the arithmetic is wrong.
    Inconsistent(String),
}

/// The N in [lo, hi] with N P = O, for one point P (`None` when P has too
/// small an order to narrow the interval).
fn candidates<F: Gf>(c: &Cubic<'_, F>, p: &Pt<F::E>, lo: u128, hi: u128, m: u128) -> Option<Vec<u128>> {
    let span = hi - lo;
    let multiples = |o: u128| -> Option<Vec<u128>> {
        let first = lo.div_ceil(o) * o;
        let count = if first > hi { 0 } else { (hi - first) / o + 1 };
        (count <= 4096).then(|| (0..count).map(|i| first + i * o).collect())
    };
    let mut baby: HashMap<(F::E, F::E), u128> = HashMap::new();
    let mut cur = p.clone();
    for j in 1..m {
        match &cur {
            None => return multiples(j),
            Some(pt) => {
                baby.insert(pt.clone(), j);
            }
        }
        cur = c.add(&cur, p);
    }
    // cur = m P.
    if cur.is_none() {
        return multiples(m);
    }
    let step = c.neg(&cur);
    let mut r = c.neg(&c.mul(lo, p));
    let mut out = vec![];
    let mut i = 0u128;
    while i * m <= span {
        let base = i * m;
        match &r {
            None => out.push(lo + base),
            Some(pt) => {
                if let Some(&j) = baby.get(pt) {
                    if base + j <= span {
                        out.push(lo + base + j);
                    }
                }
            }
        }
        r = c.add(&r, &step);
        i += 1;
    }
    out.sort_unstable();
    Some(out)
}

/// #E(F_q) of y^2 = x^3 + a2 x^2 + a4 x + a6 (smooth), q odd.
fn group_order<F: Gf>(f: &F, a2: F::E, a4: F::E, a6: F::E) -> Result<u128, Bsgs> {
    let c = Cubic::new(f, a2, a4, a6);
    if !c.is_smooth() {
        return Err(Bsgs::Inconsistent("singular cubic".into()));
    }
    let q = f.order();
    let w = isqrt(4 * q) + 1;
    let (lo, hi) = (q + 1 - w.min(q), q + 1 + w);
    let m = isqrt(hi - lo) + 1;
    let mut cands: Option<Vec<u128>> = None;
    let (mut tried, mut index) = (0usize, 0u128);
    while tried < MAX_POINTS && index < q {
        let x = f.elt_of_index(index);
        index += 1;
        let Some(y) = gsqrt(f, &c.rhs(&x)) else { continue };
        if f.is_zero(&y) {
            continue;
        }
        tried += 1;
        let Some(s) = candidates(&c, &Some((x, y)), lo, hi, m) else { continue };
        let merged: Vec<u128> = match cands {
            None => s,
            Some(prev) => prev.into_iter().filter(|n| s.binary_search(n).is_ok()).collect(),
        };
        match merged.len() {
            0 => return Err(Bsgs::Inconsistent("no order in the Hasse interval annihilates a point".into())),
            1 => return Ok(merged[0]),
            _ => cands = Some(merged),
        }
    }
    Err(Bsgs::Ambiguous(cands.map_or(0, |c| c.len())))
}

/// Traces (t, t') of E: v^2 = G(x) and E': Y^2 = x G(x) for the even sextic
/// f(u) = G(u^2) = u^6 + g2 u^4 + g1 u^2 + g0 (ascending `c`, length 7).
pub fn quotient_traces<F: Gf>(f: &F, c: &[F::E]) -> Result<(i128, i128), Bsgs> {
    let q = f.order() as i128;
    let (g0, g1, g2) = (c[0].clone(), c[2].clone(), c[4].clone());
    let n = group_order(f, g2.clone(), g1.clone(), g0.clone())?;
    let d2 = f.mul(&g0, &g0);
    let n2 = group_order(f, g1, f.mul(&g0, &g2), d2)?;
    Ok((q + 1 - n as i128, q + 1 - n2 as i128))
}

/// Is `c` (ascending, length 7) an even monic sextic u^6 + g2 u^4 + g1 u^2 + g0
/// with g0 != 0?
fn is_even_monic_sextic<F: Gf>(f: &F, c: &[F::E]) -> bool {
    c.len() == 7 && [1, 3, 5].iter().all(|&i| f.is_zero(&c[i])) && c[6] == f.one() && !f.is_zero(&c[0])
}

// -------------------------------------------- exhaustive counts, odd p

/// chi(x) in {-1, 0, 1} for x in 0..p.
pub fn chi_table(p: u64) -> Vec<i8> {
    let mut chi = vec![-1i8; p as usize];
    chi[0] = 0;
    for x in 1..=p / 2 {
        chi[(x * x % p) as usize] = 1;
    }
    chi
}

/// #C(F_p) for the smooth projective model of y^2 = f(x), p odd, genus g
/// (f of degree 2g + 1 or 2g + 2, ascending coefficients below p): the
/// affine points plus the points at infinity, 1 + chi(f_{2g+2}).
pub fn count_hyperelliptic_fp(p: u64, f: &[u64], genus: usize) -> u64 {
    let chi = chi_table(p);
    let mut sum = 0i64;
    for u in 0..p {
        let v = f.iter().rev().fold(0u64, |acc, &c| (acc * u + c) % p);
        sum += chi[v as usize] as i64;
    }
    let top = f.get(2 * genus + 2).copied().unwrap_or(0);
    let inf = if top == 0 { 1 } else { 1 + chi[top as usize] as i64 };
    (p as i64 + sum + inf) as u64
}

/// #C(F_{p^2}) for the same model, over F_p[s]/(s^2 - nr) with nr the least
/// non-residue; the character of F_{p^2} is chi of the norm to F_p. A
/// coefficient of F_p is a square of F_{p^2}, so the infinity count is 1 or 2.
pub fn count_hyperelliptic_fp2(p: u64, f: &[u64], genus: usize) -> u64 {
    let chi = chi_table(p);
    let nr = (2..p).find(|&x| chi[x as usize] == -1).unwrap_or(1);
    let mut sum = 0i64;
    for x in 0..p {
        for y in 0..p {
            let (mut a, mut b) = (0u64, 0u64);
            for &c in f.iter().rev() {
                // (a + b s)(x + y s) + c
                let na = (a * x + nr * (b * y % p)) % p;
                let nb = (a * y + b * x) % p;
                a = (na + c) % p;
                b = nb;
            }
            let norm = (a * a + (p - nr) * (b * b % p)) % p;
            sum += chi[norm as usize] as i64;
        }
    }
    let top = f.get(2 * genus + 2).copied().unwrap_or(0);
    let inf = if top == 0 { 1 } else { 2 };
    ((p * p) as i64 + sum + inf) as u64
}

// ------------------------------------------------------ binary fields

/// GF(2^n) in the polynomial basis of `modulus` (bit n set), n <= 32.
#[derive(Clone, Copy, Debug)]
pub struct Gf2n {
    pub n: u32,
    pub modulus: u64,
}

impl Gf2n {
    pub fn order(&self) -> u64 {
        1u64 << self.n
    }

    pub fn mul(&self, mut a: u64, mut b: u64) -> u64 {
        let mut r = 0;
        while b != 0 {
            if b & 1 == 1 {
                r ^= a;
            }
            b >>= 1;
            a <<= 1;
            if a >> self.n & 1 == 1 {
                a ^= self.modulus;
            }
        }
        r
    }

    pub fn pow(&self, a: u64, mut e: u64) -> u64 {
        let (mut r, mut b) = (1, a);
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, b);
            }
            b = self.mul(b, b);
            e >>= 1;
        }
        r
    }

    pub fn inv(&self, a: u64) -> u64 {
        self.pow(a, self.order() - 2)
    }

    /// Bit i is Tr(x^i); then Tr(z) = parity(z & mask).
    pub fn trace_mask(&self) -> u64 {
        (0..self.n)
            .map(|i| {
                let xi = self.pow(2, i as u64);
                let (mut t, mut s) = (0, xi);
                for _ in 0..self.n {
                    t ^= s;
                    s = self.mul(s, s);
                }
                debug_assert!(t <= 1, "the trace lies in F_2");
                t << i
            })
            .fold(0, |m, b| m | b)
    }

    /// A generator of the multiplicative group.
    pub fn primitive_element(&self) -> u64 {
        let n1 = self.order() - 1;
        let mut primes = vec![];
        let mut m = n1;
        let mut d = 2;
        while d * d <= m {
            if m.is_multiple_of(d) {
                primes.push(d);
                while m.is_multiple_of(d) {
                    m /= d;
                }
            }
            d += 1;
        }
        if m > 1 {
            primes.push(m);
        }
        (2..).find(|&c| c < self.order() && primes.iter().all(|&r| self.pow(c, n1 / r) != 1)).unwrap_or(2)
    }
}

fn gf2_deg(a: u64) -> u32 {
    63 - a.leading_zeros()
}

fn gf2_poly_mod(mut a: u64, b: u64) -> u64 {
    let db = gf2_deg(b);
    while a != 0 && gf2_deg(a) >= db {
        a ^= b << (gf2_deg(a) - db);
    }
    a
}

fn gf2_poly_gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, gf2_poly_mod(a, b));
    }
    a
}

/// Rabin's test for a polynomial of degree n over F_2.
pub fn gf2_is_irreducible(modulus: u64) -> bool {
    let n = gf2_deg(modulus);
    let f = Gf2n { n, modulus };
    let frob = |k: u32| (0..k).fold(2u64, |x, _| f.mul(x, x));
    if frob(n) != 2 {
        return false;
    }
    let mut m = n;
    let mut d = 2;
    let mut primes = vec![];
    while d * d <= m {
        if m.is_multiple_of(d) {
            primes.push(d);
            while m.is_multiple_of(d) {
                m /= d;
            }
        }
        d += 1;
    }
    if m > 1 {
        primes.push(m);
    }
    primes.iter().all(|&l| gf2_poly_gcd(frob(n / l) ^ 2, modulus) == 1)
}

/// The least (as an integer) irreducible polynomial of degree n over F_2.
pub fn gf2_irreducible(n: u32) -> u64 {
    (0u64..).map(|low| (1u64 << n) | (low << 1) | 1).find(|&m| gf2_is_irreducible(m)).unwrap_or(1u64 << n)
}

/// F_{2^km} as an extension of F_{2^m} (modulus `base`): the field, and the
/// ring map F_{2^m} -> F_{2^km} sending t to a root of `base`.
pub fn binary_extension(base: Gf2n, k: u32) -> Option<(Gf2n, u64)> {
    if k == 1 {
        return Some((base, 2));
    }
    let ext = Gf2n { n: base.n * k, modulus: gf2_irreducible(base.n * k) };
    let e = (ext.order() - 1) / (base.order() - 1);
    let g = |z: u64| (0..=base.n).rev().fold(0u64, |acc, i| ext.mul(acc, z) ^ (base.modulus >> i & 1));
    for c in 2..ext.order().min(200) {
        let w = ext.pow(c, e);
        let mut z = 1;
        for _ in 0..base.order() - 1 {
            z = ext.mul(z, w);
            if g(z) == 0 {
                return Some((ext, z));
            }
        }
    }
    None
}

/// a(theta) for a polynomial a in the basis of F_{2^m} (bit i = t^i).
pub fn embed(ext: &Gf2n, theta: u64, a: u64) -> u64 {
    let (mut r, mut pw) = (0, 1);
    for i in 0..64 {
        if a >> i & 1 == 1 {
            r ^= pw;
        }
        pw = ext.mul(pw, theta);
    }
    r
}

/// #H(F_Q) for H: y^2 + u^2 y = u^7 + a u^4 + d u over F_Q = `f` (a, d in
/// F_Q): the affine points are Q + sum_{u != 0} (-1)^Tr(u^3 + a + d u^-3) and
/// u = 0 gives (0, 0); one point at infinity. A generator walk avoids
/// inversions.
pub fn count_binary_pullback(f: &Gf2n, a: u64, d: u64) -> u64 {
    let mask = f.trace_mask();
    let tr = |x: u64| ((x & mask).count_ones() & 1) as i64;
    let g = f.primitive_element();
    let g3 = f.mul(f.mul(g, g), g);
    let g3i = f.inv(g3);
    let (mut u3, mut w) = (1u64, d);
    let mut s = 0i64;
    for _ in 0..f.order() - 1 {
        s += 1 - 2 * tr(u3 ^ a ^ w);
        u3 = f.mul(u3, g3);
        w = f.mul(w, g3i);
    }
    (f.order() as i64 + s + 1) as u64
}

fn gf2_eval(f: &Gf2n, poly: &[u64], u: u64) -> u64 {
    poly.iter().rev().fold(0, |acc, &c| f.mul(acc, u) ^ c)
}

/// #C(F_Q) for the smooth projective model of y^2 + h y = f over GF(2^n),
/// genus g, by evaluating every u (slow; the reference for the fast counts).
/// Points at infinity: 2 or 0 by Tr(f_{2g+2} / h_{g+1}^2) when h_{g+1} != 0,
/// else 1.
pub fn count_binary_generic(fld: &Gf2n, f: &[u64], h: &[u64], genus: usize) -> u64 {
    let mask = fld.trace_mask();
    let tr = |x: u64| ((x & mask).count_ones() & 1) as u64;
    let mut n = 0;
    for u in 0..fld.order() {
        let (hv, fv) = (gf2_eval(fld, h, u), gf2_eval(fld, f, u));
        if hv == 0 {
            n += 1;
        } else {
            let r = fld.mul(fv, fld.inv(fld.mul(hv, hv)));
            n += 2 * (1 - tr(r));
        }
    }
    let htop = h.get(genus + 1).copied().unwrap_or(0);
    if htop != 0 {
        let ftop = f.get(2 * genus + 2).copied().unwrap_or(0);
        n += 2 * (1 - tr(fld.mul(ftop, fld.inv(fld.mul(htop, htop)))));
    } else {
        n += 1;
    }
    n
}

/// #E(F_q) of y^2 + x y = x^3 + a x^2 + b (b != 0), q = 2^m.
pub fn count_binary_ec(f: &Gf2n, a: u64, b: u64) -> u64 {
    let mask = f.trace_mask();
    let tr = |x: u64| ((x & mask).count_ones() & 1) as u64;
    // infinity, and x = 0: y^2 = b
    let mut n = 2;
    for x in 1..f.order() {
        let xi = f.inv(x);
        let r = x ^ a ^ f.mul(b, f.mul(xi, xi));
        n += 2 * (1 - tr(r));
    }
    n
}

// ----------------------------------------------------- L-polynomials

/// 1 - tT + qT^2 times 1 - t'T + qT^2, ascending.
pub fn genus2_l(q: &BigInt, t: &BigInt, tp: &BigInt) -> Vec<BigInt> {
    let a1 = -(t + tp);
    let a2 = BigInt::from(2) * q + t * tp;
    vec![BigInt::from(1), a1.clone(), a2.clone(), q * &a1, q * q]
}

/// L(T) of genus g from the power sums s_k = q^k + 1 - #C(F_{q^k}), k = 1..g,
/// by Newton's identities (L = prod (1 - alpha_i T), a_i = (-1)^i e_i).
pub fn l_from_power_sums(q: &BigInt, s: &[BigInt]) -> Option<Vec<BigInt>> {
    let g = s.len();
    let mut e: Vec<BigInt> = vec![BigInt::from(1)];
    for k in 1..=g {
        let mut acc = BigInt::zero();
        for i in 1..=k {
            let term = &e[k - i] * &s[i - 1];
            if i % 2 == 1 {
                acc += term;
            } else {
                acc -= term;
            }
        }
        let (quot, rem) = acc.div_rem(&BigInt::from(k));
        if !rem.is_zero() {
            return None;
        }
        e.push(quot);
    }
    let a: Vec<BigInt> = (0..=g).map(|i| if i % 2 == 0 { e[i].clone() } else { -&e[i] }).collect();
    let mut l = a.clone();
    for i in (0..g).rev() {
        l.push(a[i].clone() * num_traits::pow(q.clone(), g - i));
    }
    Some(l)
}

/// Does T^2 - tau T + q divide T^{2g} L(1/T) (the characteristic polynomial
/// of Frobenius, `l` read from the top)?
pub fn divisible_by_trace_poly(l: &[BigInt], tau: &BigInt, q: &BigInt) -> bool {
    let mut r: Vec<BigInt> = l.to_vec();
    let n = r.len();
    for i in 0..n - 2 {
        let c = r[i].clone();
        r[i + 1] += tau * &c;
        r[i + 2] -= q * &c;
    }
    r[n - 2].is_zero() && r[n - 1].is_zero()
}

/// The power sums s_k = sum alpha_i^k, k = 1..g, of the roots of L (Newton).
pub fn power_sums(l: &[BigInt]) -> Vec<BigInt> {
    let g = (l.len() - 1) / 2;
    let e: Vec<BigInt> = (0..=g).map(|i| if i % 2 == 0 { l[i].clone() } else { -&l[i] }).collect();
    let mut p: Vec<BigInt> = vec![];
    for k in 1..=g {
        let mut v = BigInt::zero();
        for i in 1..k {
            let term = &e[i] * &p[k - i - 1];
            if i % 2 == 1 {
                v += term;
            } else {
                v -= term;
            }
        }
        let last = BigInt::from(k as u64) * &e[k];
        if k % 2 == 1 {
            v += last;
        } else {
            v -= last;
        }
        p.push(v);
    }
    p
}

/// #C(F_{q^k}) = q^k + 1 - s_k, k = 1..g, from L (g counts).
pub fn curve_counts(l: &[BigInt], q: &BigInt) -> Vec<BigInt> {
    power_sums(l).iter().enumerate().map(|(k, sk)| num_traits::pow(q.clone(), k + 1) + 1 - sk).collect()
}

// ------------------------------------------------------------ outcomes

/// How an L-polynomial was established (the provenance method text).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Method {
    G2Count,
    G2Derived,
    G3Count,
    G3Derived,
}

impl Method {
    pub fn text(self) -> &'static str {
        match self {
            Method::G2Count => {
                "exhaustive count of #H(F_q) and #H(F_q^2) of the cover source (q < 2^12), L(T) = 1 + a1 T + a2 T^2 + q a1 T^3 + q^2 T^4 \
                 with a1 = #H(F_q) - q - 1 and a2 = (#H(F_q^2) - q^2 - 1 + a1^2)/2; equal to L_E L_E' from the group orders of the \
                 two quotient curves (baby-step giant-step); T^2 - tT + q of the target divides the reversed polynomial"
            }
            Method::G2Derived => {
                "derived: Jac(H) ~ E x E' with E: v^2 = G(x) (the target) and E': Y^2 = x G(x), L(T) = (1 - tT + qT^2)(1 - t'T + qT^2), \
                 a1 = -(t + t'), a2 = 2q + t t', t and t' from group orders found by baby-step giant-step in the Hasse interval \
                 (unique candidate); #H(F_p) counted exhaustively and equal when p < 2^22; T^2 - tT + q of the target divides the \
                 reversed polynomial"
            }
            Method::G3Count => {
                "exhaustive count of #H over F_q, F_q^2 and F_q^3 (s^2 + s = u^3 + a + d u^-3), L(T) from Newton's identities; \
                 T^2 - tT + q divides the reversed polynomial for the exhaustively counted trace t of the target and for the trace in ecdb"
            }
            Method::G3Derived => {
                "derived: exhaustive count of #H over F_q and F_q^2 and of the elliptic quotient E (trace t), L = L_E L_B with the \
                 Prym surface B fixed by the two counts; the target trace in ecdb equals t"
            }
        }
    }
}

/// A computed, cross-checked L-polynomial.
#[derive(Debug)]
pub struct Computed {
    pub g: usize,
    pub q: BigInt,
    pub p: BigInt,
    pub r: usize,
    /// Ascending, length 2g + 1.
    pub l: Vec<BigInt>,
    pub method: Method,
}

#[derive(Debug)]
pub enum Outcome {
    Done(Box<Computed>),
    /// Not computed, with the reason (never a guess).
    Skip(&'static str),
    /// A cross-check failed: a problem, never inserted.
    Fail(String),
}

fn big(n: i128) -> BigInt {
    BigInt::from(n)
}

/// The final checks every result passes: the shape, #J > 0, Weil-type bounds
/// of the elliptic factors, and the divisibility by the target's trace
/// polynomial for each recorded target trace.
fn finish(c: Computed, targets: &[BigInt]) -> Outcome {
    let g = c.g;
    let one = BigInt::from(1);
    let l = &c.l;
    let shape = l.len() == 2 * g + 1
        && l[0] == one
        && l[2 * g] == num_traits::pow(c.q.clone(), g)
        && (1..=g).all(|i| l[2 * g - i] == &l[i] * num_traits::pow(c.q.clone(), g - i));
    if !shape {
        return Outcome::Fail(format!("L(T) {l:?} lacks the functional-equation shape over F_{}", c.q));
    }
    let order: BigInt = l.iter().sum();
    if order <= BigInt::zero() {
        return Outcome::Fail(format!("#Jac = L(1) = {order} is not positive over F_{}", c.q));
    }
    if targets.is_empty() {
        return Outcome::Skip("no_target_trace");
    }
    for tau in targets {
        if !divisible_by_trace_poly(l, tau, &c.q) {
            return Outcome::Fail(format!(
                "T^2 - ({tau})T + {} does not divide the Frobenius polynomial of L = {l:?}",
                c.q
            ));
        }
    }
    Outcome::Done(Box::new(c))
}

fn bsgs_skip(e: &Bsgs) -> Outcome {
    match e {
        Bsgs::Ambiguous(_) => Outcome::Skip("bsgs_inconclusive"),
        Bsgs::Inconsistent(m) => Outcome::Fail(format!("group order search inconsistent: {m}")),
    }
}

/// Genus 2 over F_p, p > 3 word-sized: `f` the even monic sextic (ascending,
/// residues mod p), `targets` the traces of the covered curves in ecdb.
pub fn genus2_prime(p: u64, f: &[u64], targets: &[BigInt]) -> Outcome {
    let fp = Fp64 { p };
    if !is_even_monic_sextic(&fp, f) {
        return Outcome::Skip("unsupported_shape");
    }
    let bs = quotient_traces(&fp, f);
    let direct1 = (p < DIRECT_FP_MAX).then(|| count_hyperelliptic_fp(p, f, 2));
    let direct2 = (p < DIRECT_FP2_MAX).then(|| count_hyperelliptic_fp2(p, f, 2));
    let (q, pp) = (BigInt::from(p), BigInt::from(p));
    let from_counts = match (direct1, direct2) {
        (Some(n1), Some(n2)) => {
            let a1 = big(n1 as i128 - p as i128 - 1);
            let num: BigInt = big(n2 as i128) - &q * &q - 1 + &a1 * &a1;
            if num.is_odd() {
                return Outcome::Fail(format!("#H(F_p^2) = {n2} gives a non-integral a2 over F_{p}"));
            }
            let a2: BigInt = num / 2;
            Some(vec![BigInt::from(1), a1.clone(), a2.clone(), &q * &a1, &q * &q])
        }
        _ => None,
    };
    let (l, method) = match (&bs, from_counts) {
        (Ok((t, tp)), counted) => {
            let derived = genus2_l(&q, &big(*t), &big(*tp));
            if let Some(n1) = direct1 {
                if big(n1 as i128 - p as i128 - 1) != derived[1] {
                    return Outcome::Fail(format!(
                        "#H(F_p) = {n1} disagrees with a1 = {} from the quotient curves over F_{p}",
                        derived[1]
                    ));
                }
            }
            match counted {
                Some(c) if c != derived => {
                    return Outcome::Fail(format!(
                        "exhaustive counts give L = {c:?}, the quotient curves give {derived:?} over F_{p}"
                    ))
                }
                Some(c) => (c, Method::G2Count),
                None => (derived, Method::G2Derived),
            }
        }
        (Err(Bsgs::Ambiguous(_)), Some(c)) => (c, Method::G2Count),
        (Err(e), _) => return bsgs_skip(e),
    };
    finish(Computed { g: 2, q, p: pp, r: 1, l, method }, targets)
}

/// Genus 2 over GF(p^k) (k >= 2, q < 2^48): derived from the quotient curves.
pub fn genus2_extension(fq: &Fq, f: &[Elt], targets: &[BigInt]) -> Outcome {
    let Some(ext) = ExtField::new(fq) else { return Outcome::Skip("field_too_large") };
    if !is_even_monic_sextic(&ext, f) {
        return Outcome::Skip("unsupported_shape");
    }
    match quotient_traces(&ext, f) {
        Ok((t, tp)) => {
            let q = BigInt::from(ext.q);
            let l = genus2_l(&q, &big(t), &big(tp));
            finish(Computed { g: 2, q, p: fq.p.clone(), r: fq.degree(), l, method: Method::G2Derived }, targets)
        }
        Err(e) => bsgs_skip(&e),
    }
}

/// Genus 3 over GF(2^m) for H: y^2 + u^2 y = u^7 + a u^4 + d u, with `base`
/// the field (modulus bits) and a, d its elements as bit masks.
pub fn genus3_binary(base: Gf2n, a: u64, d: u64, targets: &[BigInt]) -> Outcome {
    let m = base.n;
    if 2 * m > MAX_BINARY_COUNT_BITS {
        return Outcome::Skip("field_too_large");
    }
    let q = BigInt::from(base.order());
    let three = 3 * m <= MAX_BINARY_COUNT_BITS;
    let mut counts = vec![];
    for k in 1..=if three { 3 } else { 2 } {
        let Some((ext, theta)) = binary_extension(base, k) else { return Outcome::Skip("binary_embedding_failed") };
        counts.push(BigInt::from(count_binary_pullback(&ext, embed(&ext, theta, a), embed(&ext, theta, d))));
    }
    // The elliptic quotient E: y^2 + x y = x^3 + a x^2 + d^2, counted exhaustively.
    let b = base.mul(d, d);
    if b == 0 {
        return Outcome::Skip("unsupported_shape");
    }
    let t = &q + 1 - BigInt::from(count_binary_ec(&base, a, b));
    match genus3_l(&q, &counts, &t) {
        Ok((l, method)) => {
            finish(Computed { g: 3, q: q.clone(), p: BigInt::from(2), r: m as usize, l, method }, targets)
        }
        Err(e) => Outcome::Fail(e),
    }
}

/// L(T) of the genus-3 source over F_q from #H(F_{q^k}) (`counts`, k = 1..3 or
/// k = 1..2) and the trace t of the elliptic quotient E. With three counts L
/// is the Newton polynomial and E must divide it; with two, L = L_E L_B where
/// the two coefficients of the Prym surface B are fixed by the counts.
pub fn genus3_l(q: &BigInt, counts: &[BigInt], t: &BigInt) -> Result<(Vec<BigInt>, Method), String> {
    let s: Vec<BigInt> = counts.iter().enumerate().map(|(k, n)| num_traits::pow(q.clone(), k + 1) + 1 - n).collect();
    if s.len() == 3 {
        let Some(l) = l_from_power_sums(q, &s) else {
            return Err(format!("counts over F_{q}, F_{q}^2, F_{q}^3 give a non-integral L(T)"));
        };
        if !divisible_by_trace_poly(&l, t, q) {
            return Err(format!("the elliptic quotient (trace {t}) does not divide the genus-3 L(T) over F_{q}"));
        }
        return Ok((l, Method::G3Count));
    }
    // Power sums of B are those of H minus those of E.
    let sb1 = &s[0] - t;
    let sb2 = &s[1] - (t * t - BigInt::from(2) * q);
    let num: BigInt = &sb1 * &sb1 - &sb2;
    if num.is_odd() {
        return Err(format!("non-integral Prym L-polynomial over F_{q}"));
    }
    let b1 = -&sb1;
    let b2: BigInt = num / 2;
    let lb = [BigInt::from(1), b1.clone(), b2, q * &b1, q * q];
    let le = [BigInt::from(1), -t, q.clone()];
    let mut l = vec![BigInt::zero(); 7];
    for (i, x) in le.iter().enumerate() {
        for (j, y) in lb.iter().enumerate() {
            l[i + j] += x * y;
        }
    }
    Ok((l, Method::G3Derived))
}

// -------------------------------------------------------- the importer

/// One cover source to compute.
struct Job {
    curve_id: i64,
    genus: i32,
    constructions: Vec<String>,
    f: Vec<String>,
    h: Vec<String>,
    fq: Fq,
    /// The characteristic is proved prime and the extension modulus proved irreducible.
    field_proved: bool,
    targets: Vec<BigInt>,
}

fn residue(e: &Elt) -> Option<u64> {
    e.first()?.to_u64()
}

fn bits_mask(e: &Elt) -> u64 {
    e.iter().enumerate().fold(0, |m, (i, c)| if c.is_zero() { m } else { m | 1 << i })
}

fn run_job(job: &Job) -> Outcome {
    let fq = &job.fq;
    let parse = |v: &[String]| -> Option<Vec<Elt>> { v.iter().map(|s| fq.parse(s).ok()).collect() };
    let (Some(f), Some(h)) = (parse(&job.f), parse(&job.h)) else { return Outcome::Skip("unparsed_model") };
    let has = |c: &str| job.constructions.iter().any(|x| x == c);
    match fq.encoding {
        Encoding::Decimal if has(PRIME_CONSTRUCTION) && job.genus == 2 && h.iter().all(|c| fq.is_zero(c)) => {
            if fq.p.bits() > MAX_FP_BITS {
                return Outcome::Skip("field_too_large");
            }
            let Some(p) = fq.p.to_u64() else { return Outcome::Skip("field_too_large") };
            if p < 5 {
                return Outcome::Skip("unsupported_shape");
            }
            let Some(fu) = f.iter().map(residue).collect::<Option<Vec<u64>>>() else {
                return Outcome::Skip("unparsed_model");
            };
            genus2_prime(p, &fu, &job.targets)
        }
        Encoding::DecimalVector if has(PRIME_CONSTRUCTION) && job.genus == 2 && h.iter().all(|c| fq.is_zero(c)) => {
            genus2_extension(fq, &f, &job.targets)
        }
        Encoding::HexBitset if has(BINARY_CONSTRUCTION) && job.genus == 3 => {
            let m = fq.degree() as u32;
            if 2 * m > MAX_BINARY_COUNT_BITS {
                return Outcome::Skip("field_too_large");
            }
            let modulus =
                fq.f.iter().enumerate().fold(0u64, |acc, (i, c)| if c.is_zero() { acc } else { acc | 1 << i });
            let base = Gf2n { n: m, modulus };
            let (zero, shape) = (0u64, [0u64, 0, 0, 0, 0, 0, 0, 1]);
            let fb: Vec<u64> = f.iter().map(bits_mask).collect();
            let hb: Vec<u64> = h.iter().map(bits_mask).collect();
            let ok = fb.len() == 8
                && hb == [0, 0, 1]
                && fb[0] == zero
                && [2, 3, 5, 6].iter().all(|&i| fb[i] == shape[i])
                && fb[7] == 1;
            if !ok {
                return Outcome::Skip("unsupported_shape");
            }
            genus3_binary(base, fb[4], fb[1], &job.targets)
        }
        _ => Outcome::Skip("unsupported_construction"),
    }
}

/// Run every job on the available cores.
fn run_jobs(jobs: &[Job]) -> Vec<Outcome> {
    let next = std::sync::atomic::AtomicUsize::new(0);
    let threads = std::thread::available_parallelism().map_or(2, |n| n.get()).min(16);
    let results = std::sync::Mutex::new(Vec::with_capacity(jobs.len()));
    std::thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| loop {
                let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let Some(job) = jobs.get(i) else { break };
                let out = run_job(job);
                if let Ok(mut r) = results.lock() {
                    r.push((i, out));
                }
            });
        }
    });
    let mut r = results.into_inner().unwrap_or_default();
    r.sort_by_key(|(i, _)| *i);
    r.into_iter().map(|(_, o)| o).collect()
}

/// The `isogeny_class_fq` row of a computed L-polynomial, shaped like the
/// registry's (`registry::class_row`) and the generated genus-2 rows.
pub fn class_row(c: &Computed, status: &str) -> Result<(String, Value)> {
    let label = weil_label(c.g, &c.q, &c.l)?;
    let order: BigInt = c.l.iter().sum();
    let counts = curve_counts(&c.l, &c.q);
    let top = &c.l[c.g];
    let ordinary = !(top.mod_floor(&c.p)).is_zero();
    Ok((
        label.clone(),
        json!({
            "q": c.q.to_string(), "p": c.p.to_string(), "r": c.r, "g": c.g,
            "l_polynomial": c.l.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
            "label": label, "order_value": order.to_string(),
            "curve_counts": counts.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
            "ordinary": ordinary, "status": status,
        }),
    ))
}

const JOBS_SQL: &str = "
SELECT h.id, h.genus, hm.f, hm.h,
       fl.kind::text, fl.characteristic::text, fl.defining_polynomial::text[], fl.modulus_hex,
       fl.irreducibility_status::text,
       ARRAY(SELECT DISTINCT m.construction FROM ec.curve_morphism m
              WHERE m.source_curve_id = h.id AND m.kind = 'hyperelliptic_pullback' AND m.construction IS NOT NULL),
       ARRAY(SELECT DISTINCT ic.trace::text FROM ec.curve_morphism m
               JOIN ec.abelian_variety tav ON tav.curve_id = m.target_curve_id
               JOIN ec.isogeny_class_fq ic ON ic.id = tav.isogeny_class_id AND ic.g = 1
              WHERE m.source_curve_id = h.id AND m.kind = 'hyperelliptic_pullback')
FROM ec.curve h
JOIN ec.hyperelliptic_model hm ON hm.curve_id = h.id
JOIN ec.field fl ON fl.id = h.field_id
JOIN ec.abelian_variety av ON av.curve_id = h.id
LEFT JOIN ec.curve_zeta z ON z.curve_id = h.id
WHERE h.genus IN (2, 3) AND h.family = 'cover'
  AND EXISTS (SELECT 1 FROM ec.curve_morphism m
               WHERE m.source_curve_id = h.id AND m.kind = 'hyperelliptic_pullback'
                 AND m.construction IN ('prime_quadratic_pullback_v1', 'binary_cubic_pullback_v1'))
  AND (av.isogeny_class_id IS NULL OR z.curve_id IS NULL)
ORDER BY h.id";

const DONE_SQL: &str = "
SELECT count(*) FROM ec.curve h
JOIN ec.abelian_variety av ON av.curve_id = h.id
JOIN ec.curve_zeta z ON z.curve_id = h.id
WHERE h.genus IN (2, 3) AND h.family = 'cover' AND av.isogeny_class_id IS NOT NULL
  AND EXISTS (SELECT 1 FROM ec.curve_morphism m
               WHERE m.source_curve_id = h.id AND m.kind = 'hyperelliptic_pullback'
                 AND m.construction IN ('prime_quadratic_pullback_v1', 'binary_cubic_pullback_v1'))";

const CLASS_SQL: &str = "
INSERT INTO ec.isogeny_class_fq (q, p, r, g, l_polynomial, label, order_value, order_bits, point_counts, curve_counts,
       is_ordinary, has_jacobian, status, provenance_id)
SELECT r.q, r.p, r.r, r.g, r.l_polynomial, r.label, r.order_value, ec.bits(r.order_value), ARRAY[r.order_value],
       r.curve_counts, r.ordinary, true, r.status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(q numeric, p numeric, r int, g int, l_polynomial numeric[], label text,
     order_value numeric, curve_counts numeric[], ordinary boolean, status text)
ON CONFLICT DO NOTHING";

const LINK_SQL: &str = "
UPDATE ec.abelian_variety av SET isogeny_class_id = ic.id
FROM jsonb_to_recordset($1::jsonb) AS r(curve_id bigint, label text)
JOIN ec.isogeny_class_fq ic ON ic.label = r.label
WHERE av.curve_id = r.curve_id AND av.isogeny_class_id IS DISTINCT FROM ic.id";

const ZETA_SQL: &str = "
INSERT INTO ec.curve_zeta (curve_id, isogeny_class_id, point_counts, order_check, order_method, provenance_id)
SELECT r.curve_id, ic.id, r.point_counts, r.status::ec.fact_status, r.method, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(curve_id bigint, label text, point_counts numeric[], status text, method text)
JOIN ec.isogeny_class_fq ic ON ic.label = r.label
ON CONFLICT (curve_id) DO UPDATE SET isogeny_class_id = EXCLUDED.isogeny_class_id,
       point_counts = EXCLUDED.point_counts, order_check = EXCLUDED.order_check,
       order_method = EXCLUDED.order_method, provenance_id = EXCLUDED.provenance_id
WHERE (ec.curve_zeta.isogeny_class_id, ec.curve_zeta.point_counts, ec.curve_zeta.order_check, ec.curve_zeta.order_method)
      IS DISTINCT FROM (EXCLUDED.isogeny_class_id, EXCLUDED.point_counts, EXCLUDED.order_check, EXCLUDED.order_method)";

/// Compute and store the L-polynomial of every cover source that lacks one.
pub async fn import(ctx: &Ctx<'_>, report: &mut Report) -> Result<()> {
    let client = ctx.client;
    let rows = client.query(JOBS_SQL, &[]).await?;
    let done: i64 = client.query_one(DONE_SQL, &[]).await?.get(0);
    report.count("cover_lpoly_already_complete", done as u64);
    let mut jobs = Vec::with_capacity(rows.len());
    for r in rows {
        let curve_id: i64 = r.get(0);
        let kind: String = r.get(4);
        let p: String = r.get(5);
        let poly: Option<Vec<String>> = r.get(6);
        let modulus: Option<String> = r.get(7);
        let irreducible: String = r.get(8);
        let Some(fq) = fq_from_parts(&kind, &p, poly, modulus)? else {
            report.count("cover_lpoly_skipped", 1);
            report.count("cover_lpoly_skipped_unsupported_field", 1);
            continue;
        };
        let jf = |v: Value| -> Vec<String> {
            v.as_array().into_iter().flatten().filter_map(|x| x.as_str().map(str::to_string)).collect()
        };
        let prime_proved = is_probable_prime(&fq.p).1;
        let traces: Vec<String> = r.get(10);
        jobs.push(Job {
            curve_id,
            genus: r.get(1),
            constructions: r.get(9),
            f: jf(r.get(2)),
            h: jf(r.get(3)),
            field_proved: prime_proved && (kind == "prime" || kind == "binary" || irreducible == "proved"),
            fq,
            targets: traces.iter().filter_map(|t| t.parse().ok()).collect(),
        });
    }
    if jobs.is_empty() {
        return Ok(());
    }
    let jobs = std::sync::Arc::new(jobs);
    let outcomes = {
        let jobs = jobs.clone();
        tokio::task::spawn_blocking(move || run_jobs(&jobs)).await?
    };

    // Group by method: one provenance row per method text.
    let mut by_method: BTreeMap<Method, (Vec<Value>, Vec<Value>)> = BTreeMap::new();
    let mut classes: BTreeMap<Method, BTreeMap<String, Value>> = BTreeMap::new();
    for (job, out) in jobs.iter().zip(outcomes) {
        match out {
            Outcome::Done(c) => {
                let status = if job.field_proved { "proved" } else { "probable" };
                let (label, class) = class_row(&c, status)?;
                let counts: Vec<String> = curve_counts(&c.l, &c.q).iter().map(|x| x.to_string()).collect();
                classes.entry(c.method).or_default().entry(label.clone()).or_insert(class);
                let e = by_method.entry(c.method).or_default();
                e.0.push(json!({ "curve_id": job.curve_id, "label": label }));
                e.1.push(json!({
                    "curve_id": job.curve_id, "label": label, "point_counts": counts, "status": status,
                    "method": c.method.text(),
                }));
                report.count("cover_lpoly_computed", 1);
                report.count(&format!("cover_lpoly_genus{}", c.g), 1);
            }
            Outcome::Skip(reason) => {
                report.count("cover_lpoly_skipped", 1);
                report.count(&format!("cover_lpoly_skipped_{reason}"), 1);
            }
            Outcome::Fail(msg) => {
                report.count("cover_lpoly_crosscheck_failures", 1);
                report.problem(format!(
                    "cover source curve id {}: L-polynomial cross-check failed, nothing stored: {msg}",
                    job.curve_id
                ));
            }
        }
    }
    let src = client
        .query_one(
            "WITH ins AS (INSERT INTO ec.source (kind, title, provenance_class)
                          VALUES ('computation', 'ecdb cover L-polynomials', 'internal') ON CONFLICT DO NOTHING RETURNING id)
             SELECT id FROM ins UNION ALL
             SELECT id FROM ec.source WHERE kind = 'computation' AND title = 'ecdb cover L-polynomials' AND repository IS NULL LIMIT 1",
            &[],
        )
        .await?
        .get::<_, i64>(0);
    for (method, (links, zetas)) in by_method {
        let prov = ctx.provenance(src, method.text(), None).await?;
        let class_rows: Vec<Value> = classes.remove(&method).map(|m| m.into_values().collect()).unwrap_or_default();
        report.count("cover_lpoly_classes", bulk(client, CLASS_SQL, &class_rows, prov).await?);
        report.count("cover_lpoly_jacobians_linked", bulk1(client, LINK_SQL, &links).await?);
        report.count("cover_lpoly_zeta", bulk(client, ZETA_SQL, &zetas, prov).await?);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::import::registry::{class_row as genus1_class_row, prime_field};

    /// G(x) = F(x + c) for the first c in 0..4 with F(c) != 0, F = x^3 + a x + b,
    /// as the even sextic G(u^2) of `prime_quadratic_pullback_v1` (ascending).
    fn cover_sextic(p: u64, a: u64, b: u64) -> Vec<u64> {
        let c = (0..4u64).find(|&c| !(c * c * c + a * c + b).is_multiple_of(p)).unwrap();
        let g0 = (c * c * c + a * c + b) % p;
        let g1 = (3 * c * c + a) % p;
        let g2 = 3 * c % p;
        vec![g0, 0, g1, 0, g2, 0, 1]
    }

    /// #E(F_p) of y^2 = x^3 + a x + b by listing the pairs.
    fn brute_ec(p: u64, a: u64, b: u64) -> u64 {
        let mut n = 1;
        for x in 0..p {
            for y in 0..p {
                if (y * y + p - (x * x % p * x + a * x + b) % p).is_multiple_of(p) {
                    n += 1;
                }
            }
        }
        n
    }

    /// #H(F_p) of v^2 = f(u), f a monic sextic, by listing the pairs (+ 2 at infinity).
    fn brute_h(p: u64, f: &[u64]) -> u64 {
        let ev = |u: u64| f.iter().rev().fold(0, |acc, &c| (acc * u + c) % p);
        let mut n = 2;
        for u in 0..p {
            let fu = ev(u);
            n += (0..p).filter(|&v| v * v % p == fu).count() as u64;
        }
        n
    }

    /// #H(F_{p^2}) by listing the pairs (u, v) of F_p[s]/(s^2 - nr).
    fn brute_h_fp2(p: u64, f: &[u64]) -> u64 {
        let nr = (2..p).find(|&x| (1..p).all(|y| y * y % p != x)).unwrap();
        let mul = |a: (u64, u64), b: (u64, u64)| ((a.0 * b.0 + nr * (a.1 * b.1 % p)) % p, (a.0 * b.1 + a.1 * b.0) % p);
        let ev = |u: (u64, u64)| {
            f.iter().rev().fold((0, 0), |acc, &c| {
                let m = mul(acc, u);
                ((m.0 + c) % p, m.1)
            })
        };
        let mut n = 2;
        for ux in 0..p {
            for uy in 0..p {
                let fu = ev((ux, uy));
                for vx in 0..p {
                    for vy in 0..p {
                        if mul((vx, vy), (vx, vy)) == fu {
                            n += 1;
                        }
                    }
                }
            }
        }
        n
    }

    fn done(o: Outcome) -> Computed {
        match o {
            Outcome::Done(c) => *c,
            other => panic!("expected a result, got {other:?}"),
        }
    }

    fn ints(v: &[i128]) -> Vec<BigInt> {
        v.iter().map(|&x| BigInt::from(x)).collect()
    }

    /// Test 1: p = 101 against listing every pair, and the divisibility by the
    /// target's trace polynomial.
    #[test]
    fn small_prime_counts_match_brute_force_and_target_trace() {
        let (p, a, b) = (101u64, 3, 7);
        let f = cover_sextic(p, a, b);
        let nh = brute_h(p, &f);
        assert_eq!(count_hyperelliptic_fp(p, &f, 2), nh);
        let t = p as i128 + 1 - brute_ec(p, a, b) as i128;

        // F_{p^2} is checked against listing the pairs at p = 13.
        let f13 = cover_sextic(13, 2, 5);
        assert_eq!(count_hyperelliptic_fp2(13, &f13, 2), brute_h_fp2(13, &f13));

        let c = done(genus2_prime(p, &f, &ints(&[t])));
        assert_eq!(c.method, Method::G2Count);
        let a1 = nh as i128 - p as i128 - 1;
        let n2 = count_hyperelliptic_fp2(p, &f, 2) as i128;
        let a2 = (n2 - (p * p) as i128 - 1 + a1 * a1) / 2;
        let q = p as i128;
        assert_eq!(c.l, ints(&[1, a1, a2, q * a1, q * q]));
        // T^2 - tT + p divides T^4 + a1 T^3 + a2 T^2 + p a1 T + p^2.
        assert!(divisible_by_trace_poly(&c.l, &BigInt::from(t), &BigInt::from(p)));
        // The two quotient curves give the same polynomial: t' = -a1 - t and a2 = 2p + t t'.
        let tp = -a1 - t;
        assert_eq!(a2, 2 * q + t * tp);
        assert_eq!(quotient_traces(&Fp64 { p }, &f), Ok((t, tp)));
        // A wrong target trace is a cross-check failure, never a result.
        assert!(matches!(genus2_prime(p, &f, &ints(&[t + 1])), Outcome::Fail(_)));
        assert!(matches!(genus2_prime(p, &f, &[]), Outcome::Skip("no_target_trace")));
    }

    /// Test 2: rows of the live data. (p, a, b, t) are the registry's; the
    /// expected (a1, a2) come from the Python reference (`count_H_fp`,
    /// `count_H_fp2`; for the 24-bit prime only a1 is counted and a2 = 2p + t t').
    #[test]
    fn live_cover_rows_have_the_independent_l_polynomial() {
        // icv1-fp10-t15-87f382ed: GF(571), a = 508, b = 137, t = 15, c = 0.
        let (p, f) = (571u64, vec![137, 0, 508, 0, 0, 0, 1]);
        assert_eq!(cover_sextic(p, 508, 137), f);
        let c = done(genus2_prime(p, &f, &ints(&[15])));
        assert_eq!(c.method, Method::G2Count);
        assert_eq!(c.l, ints(&[1, 22, 587, 571 * 22, 571 * 571]));
        assert_eq!(curve_counts(&c.l, &BigInt::from(p)), ints(&[594, 326732]));
        // icv1-fp24-t553-4d349218: GF(8416129), a = 1040051, b = 1713668, t = 553.
        let p = 8416129u64;
        let f = vec![1713668, 0, 1040051, 0, 0, 0, 1];
        assert_eq!(cover_sextic(p, 1040051, 1713668), f);
        let c = done(genus2_prime(p, &f, &ints(&[553])));
        assert_eq!(c.method, Method::G2Derived);
        let q = p as i128;
        assert_eq!(c.l, ints(&[1, -2156, 17718717, q * -2156, q * q]));
        // t' = -a1 - t = 1603 and #J = N N' for the two quotient curves.
        let order: BigInt = c.l.iter().sum();
        assert_eq!(order, BigInt::from((q + 1 - 553) * (q + 1 - 1603)));
        assert!(matches!(genus2_prime(p, &f, &ints(&[554])), Outcome::Fail(_)));
    }

    /// The group-order search over an extension field agrees with the
    /// Frobenius relation t_{q^2} = t^2 - 2q from the subfield.
    #[test]
    fn extension_field_traces_follow_the_subfield() {
        let p = 13u64;
        let (a, b) = (2u64, 3u64);
        let f = cover_sextic(p, a, b);
        let (t, tp) = quotient_traces(&Fp64 { p }, &f).unwrap();
        // GF(169) = F_13[t] / (t^2 - 2): 2 is a non-residue mod 13.
        let fq = Fq::extension(BigInt::from(p), &[BigInt::from(11), BigInt::from(0)]);
        let lift: Vec<Elt> = f.iter().map(|&c| vec![BigInt::from(c), BigInt::zero()]).collect();
        let ext = ExtField::new(&fq).unwrap();
        let (te, tpe) = quotient_traces(&ext, &lift).unwrap();
        assert_eq!((te, tpe), (t * t - 26, tp * tp - 26));
        let target = BigInt::from(t * t - 26);
        let c = done(genus2_extension(&fq, &lift, &[target]));
        assert_eq!(c.q, BigInt::from(169));
        assert_eq!(c.l, genus2_l(&BigInt::from(169), &BigInt::from(te), &BigInt::from(tpe)));
        assert_eq!(c.r, 2);
    }

    #[test]
    fn character_sums_and_infinity_for_degree_five_and_six() {
        let p = 31u64;
        // y^2 = x^5 + x + 3: one point at infinity (degree 2g + 1).
        let f5 = vec![3, 1, 0, 0, 0, 1];
        let aff: u64 = (0..p).map(|u| (0..p).filter(|&v| v * v % p == (u.pow(5) + u + 3) % p).count() as u64).sum();
        assert_eq!(count_hyperelliptic_fp(p, &f5, 2), aff + 1);
        // y^2 = n x^6 + x + 3 with n a non-residue: no point at infinity; with n a residue: two.
        let nr = (2..p).find(|&x| (1..p).all(|y| y * y % p != x)).unwrap();
        for (lead, inf) in [(nr, 0), (4, 2)] {
            let f6 = vec![3, 1, 0, 0, 0, 0, lead];
            let aff: u64 =
                (0..p).map(|u| (0..p).filter(|&v| v * v % p == (lead * u.pow(6) + u + 3) % p).count() as u64).sum();
            assert_eq!(count_hyperelliptic_fp(p, &f6, 2), aff + inf);
        }
    }

    #[test]
    fn binary_field_arithmetic_and_irreducibility() {
        assert!(gf2_is_irreducible(0b111));
        assert!(!gf2_is_irreducible(0b101));
        assert!(gf2_is_irreducible(0b1011));
        for n in [2u32, 6, 9, 12, 18, 24] {
            let m = gf2_irreducible(n);
            assert_eq!(gf2_deg(m), n);
            assert!(gf2_is_irreducible(m));
            let f = Gf2n { n, modulus: m };
            let g = f.primitive_element();
            assert_eq!(f.pow(g, f.order() - 1), 1);
            for x in [1u64, 2, 3, f.order() - 1] {
                assert_eq!(f.mul(x, f.inv(x)), 1);
            }
        }
    }

    /// Test 3: genus 3 over GF(2^m) at tiny m, against listing the pairs
    /// (y, u) with y^2 + u^2 y = u^7 + a u^4 + d u.
    #[test]
    fn binary_genus3_counts_and_l_polynomial() {
        for (m, modulus, a, d) in
            [(3u32, 0b1011u64, 0u64, 1u64), (3, 0b1011, 1, 5), (4, 0b10011, 1, 3), (5, 0b100101, 0, 7)]
        {
            let base = Gf2n { n: m, modulus };
            let q = base.order();
            let hpoly = [0, 0, 1];
            let mut counts = vec![];
            for k in 1..=3u32 {
                let (ext, theta) = binary_extension(base, k).unwrap();
                // embed is a ring homomorphism F_{2^m} -> F_{2^km}
                for x in 0..q.min(8) {
                    for y in 0..q.min(8) {
                        assert_eq!(
                            embed(&ext, theta, base.mul(x, y)),
                            ext.mul(embed(&ext, theta, x), embed(&ext, theta, y))
                        );
                        assert_eq!(embed(&ext, theta, x ^ y), embed(&ext, theta, x) ^ embed(&ext, theta, y));
                    }
                }
                let (ea, ed) = (embed(&ext, theta, a), embed(&ext, theta, d));
                let fast = count_binary_pullback(&ext, ea, ed);
                let ef = [0, ed, 0, 0, ea, 0, 0, 1];
                assert_eq!(fast, count_binary_generic(&ext, &ef, &hpoly, 3), "m = {m}, k = {k}");
                if ext.order() <= 512 {
                    // listing the pairs, without traces: y^2 + h(u) y = f(u)
                    let mut n = 1;
                    for u in 0..ext.order() {
                        for y in 0..ext.order() {
                            if ext.mul(y, y) ^ ext.mul(gf2_eval(&ext, &hpoly, u), y) == gf2_eval(&ext, &ef, u) {
                                n += 1;
                            }
                        }
                    }
                    assert_eq!(fast, n, "m = {m}, k = {k}");
                }
                counts.push(BigInt::from(fast));
            }
            // The elliptic quotient, counted by listing the pairs of y^2 + x y = x^3 + a x^2 + d^2.
            let b = base.mul(d, d);
            let mut ne = 1;
            for x in 0..q {
                for y in 0..q {
                    let lhs = base.mul(y, y) ^ base.mul(x, y);
                    let rhs = base.mul(base.mul(x, x), x) ^ base.mul(a, base.mul(x, x)) ^ b;
                    if lhs == rhs {
                        ne += 1;
                    }
                }
            }
            assert_eq!(count_binary_ec(&base, a, b), ne);
            let bq = BigInt::from(q);
            let t = &bq + 1 - BigInt::from(ne);
            let (l3, m3) = genus3_l(&bq, &counts, &t).unwrap();
            assert_eq!(m3, Method::G3Count);
            // Jac(H) ~ E x B: two counts and the elliptic trace give the same L.
            let (l2, m2) = genus3_l(&bq, &counts[..2], &t).unwrap();
            assert_eq!(m2, Method::G3Derived);
            assert_eq!(l2, l3, "m = {m}");
            assert_eq!(curve_counts(&l3, &bq), counts);
            let c = done(genus3_binary(base, a, d, std::slice::from_ref(&t)));
            assert_eq!((c.g, c.r, c.p.clone()), (3, m as usize, BigInt::from(2)));
            assert_eq!(c.l, l3);
            assert!(matches!(genus3_binary(base, a, d, &[&t + 1]), Outcome::Fail(_)));
        }
    }

    #[test]
    fn newton_identities_roundtrip() {
        let q = BigInt::from(571);
        let l = genus2_l(&q, &BigInt::from(15), &BigInt::from(-37));
        let s = power_sums(&l);
        assert_eq!(s[0], BigInt::from(-22));
        assert_eq!(l_from_power_sums(&q, &s).unwrap(), l);
    }

    /// Test 4: the label and the row follow the conventions of `registry.rs`
    /// (`class_row`, `weil_label`) and the `isogeny_class_fq` CHECKs.
    #[test]
    fn class_row_follows_the_registry_conventions() {
        let c = Computed {
            g: 2,
            q: BigInt::from(571),
            p: BigInt::from(571),
            r: 1,
            l: ints(&[1, 22, 587, 571 * 22, 571 * 571]),
            method: Method::G2Count,
        };
        let (label, row) = class_row(&c, "proved").unwrap();
        // 22 = 'w', 587 = 22 * 26 + 15 = 'wp' in the LMFDB base-26 letters.
        assert_eq!(label, "2.571.w_wp");
        assert_eq!(row["label"], json!("2.571.w_wp"));
        // The genus-1 label of the registry has the same g.q.letters shape.
        let spec = prime_field("fp-571", &BigInt::from(571)).unwrap();
        let (l1, row1) = genus1_class_row(&spec, &BigInt::from(15)).unwrap();
        assert_eq!(l1, weil_label(1, &BigInt::from(571), &genus1_l(15, 571)).unwrap());
        assert!(l1.starts_with("1.571."));
        // Same column conventions: decimal strings, ascending L, q = p^r, order = L(1).
        assert_eq!(row1["q"], json!("571"));
        assert_eq!(row1["g"], json!(1));
        assert_eq!(row["q"], json!("571"));
        assert_eq!(row["l_polynomial"], json!(["1", "22", "587", "12562", "326041"]));
        assert_eq!(row["order_value"], json!((1 + 22 + 587 + 12562 + 326041).to_string()));
        assert_eq!(row["curve_counts"], json!(["594", "326732"]));
        assert_eq!(row["ordinary"], json!(true));
        assert_eq!(row["status"], json!("proved"));
        // CHECK (cardinality = 2g + 1, l[1] = 1, l[2g + 1] = q^g, q = p^r)
        assert_eq!(c.l.len(), 2 * c.g + 1);
        assert_eq!(c.l[4], num_traits::pow(c.q.clone(), 2));
        // A genus-3 label over GF(2^6): three letters.
        let c3 = Computed { g: 3, q: BigInt::from(64), p: BigInt::from(2), r: 6, l: vec![], method: Method::G3Count };
        let l3 = ints(&[1, -5, 9, -20, 9 * 64, -5 * 64 * 64, 64 * 64 * 64]);
        let (label3, row3) = class_row(&Computed { l: l3, ..c3 }, "probable").unwrap();
        assert_eq!(label3, "3.64.af_j_au");
        assert_eq!(row3["r"], json!(6));
        assert_eq!(row3["status"], json!("probable"));
    }

    fn genus1_l(t: i128, q: i128) -> Vec<BigInt> {
        ints(&[1, -t, q])
    }
}
