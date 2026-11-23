//! HUMBERT_SETUP_FAST=1: faster arithmetic and algorithms for the instance setup
//! (Setup::new, FactorBase::build) that compute exactly the same deterministic
//! values and consume exactly the same RNG draws as the default path.
//!
//! * `FK`: K4 = F_p[t]/(m) arithmetic with u64 accumulators and Barrett
//!   reduction (p < 2^17, so every product sum is < 7 p^2 < 2^37 and fits in u64). Canonical
//!   representatives in [0, p), so every value equals the K4 value.
//! * `group_order_fast`: replays curve::group_order. That routine draws random
//!   points P_1, P_2, ... and keeps the multiples-of-ord(P) sets intersected over
//!   the Hasse interval until one candidate is left. For ord(P) > m (its baby-step
//!   count) the BSGS set of round k is exactly {n in [lo, hi] : ord(P_k) | n}
//!   (see the proof in the comment of `group_order_fast`). The fast path draws
//!   the same points (same x draws, same square test, same sign draw), finds one
//!   multiple of ord(P_1) in the interval with a 4|N, +-x BSGS, derives ord(P_1)
//!   by factoring, and tests the remaining candidates directly. Any unusual case
//!   (small order, many candidates, no hit) restores the RNG and runs the
//!   original routine.
//! * `factor_fast`: Miller-Rabin + Pollard-Brent rho; same output as curve::factor.
use crate::curve::{Curve, Pt, Rng};
use crate::field::{legendre, K4, E4};
use std::collections::HashMap;

pub fn setup_fast() -> bool {
    std::env::var("HUMBERT_SETUP_FAST").map(|v| v == "1").unwrap_or(false)
}

#[derive(Clone)]
pub struct FK {
    pub p: u64,
    red: [E4; 3],
    frob_t: [E4; 4],
    /// floor(2^64 / p)
    bm: u64,
}

impl FK {
    pub fn from_k4(k: &K4) -> FK {
        FK { p: k.p, red: k.red, frob_t: k.frob_t, bm: (u64::MAX / k.p) }
    }
    #[inline(always)]
    fn rd(&self, a: u64) -> u64 {
        // Barrett: q_est <= a/p, error at most 2
        let q = ((a as u128 * self.bm as u128) >> 64) as u64;
        let mut r = a - q * self.p;
        while r >= self.p {
            r -= self.p;
        }
        r
    }
    #[inline(always)]
    pub fn add(&self, a: &E4, b: &E4) -> E4 {
        let p = self.p;
        let f = |x: u64, y: u64| {
            let c = x + y;
            if c >= p {
                c - p
            } else {
                c
            }
        };
        [f(a[0], b[0]), f(a[1], b[1]), f(a[2], b[2]), f(a[3], b[3])]
    }
    #[inline(always)]
    pub fn sub(&self, a: &E4, b: &E4) -> E4 {
        let p = self.p;
        let f = |x: u64, y: u64| if x >= y { x - y } else { x + p - y };
        [f(a[0], b[0]), f(a[1], b[1]), f(a[2], b[2]), f(a[3], b[3])]
    }
    #[inline(always)]
    pub fn neg(&self, a: &E4) -> E4 {
        self.sub(&[0; 4], a)
    }
    #[inline(always)]
    pub fn mul(&self, a: &E4, b: &E4) -> E4 {
        // c_i <= 4 (p-1)^2 < 2^36 (p < 2^17)
        let mut c = [0u64; 7];
        for i in 0..4 {
            for j in 0..4 {
                c[i + j] += a[i] * b[j];
            }
        }
        let mut r = [c[0], c[1], c[2], c[3]];
        for h in 0..3 {
            let ch = self.rd(c[4 + h]);
            for i in 0..4 {
                r[i] += ch * self.red[h][i]; // < 2^36 + 3 * 2^34
            }
        }
        [self.rd(r[0]), self.rd(r[1]), self.rd(r[2]), self.rd(r[3])]
    }
    #[inline(always)]
    fn mulm(&self, a: u64, b: u64) -> u64 {
        self.rd(a * b)
    }
    pub fn frob(&self, a: &E4) -> E4 {
        let mut r = [0u64; 4];
        for j in 0..4 {
            for i in 0..4 {
                r[i] += a[j] * self.frob_t[j][i];
            }
        }
        [self.rd(r[0]), self.rd(r[1]), self.rd(r[2]), self.rd(r[3])]
    }
    pub fn norm(&self, a: &E4) -> u64 {
        let f1 = self.frob(a);
        let f2 = self.frob(&f1);
        let f3 = self.frob(&f2);
        self.mul(&self.mul(a, &f1), &self.mul(&f2, &f3))[0]
    }
    fn powp(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1 % self.p;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mulm(r, a);
            }
            a = self.mulm(a, a);
            e >>= 1;
        }
        r
    }
    pub fn legendre(&self, a: u64) -> i32 {
        let a = a % self.p;
        if a == 0 {
            return 0;
        }
        if self.powp(a, (self.p - 1) / 2) == 1 {
            1
        } else {
            -1
        }
    }
    pub fn chi(&self, a: &E4) -> i32 {
        self.legendre(self.norm(a))
    }
    pub fn inv(&self, a: &E4) -> E4 {
        assert!(*a != [0; 4], "K4 inverse of zero");
        let f1 = self.frob(a);
        let f2 = self.frob(&f1);
        let f3 = self.frob(&f2);
        let c = self.mul(&self.mul(&f1, &f2), &f3);
        let n = self.mul(a, &c)[0];
        let ni = self.powp(n, self.p - 2);
        [self.mulm(ni, c[0]), self.mulm(ni, c[1]), self.mulm(ni, c[2]), self.mulm(ni, c[3])]
    }
    pub fn pow(&self, a: &E4, mut e: u128) -> E4 {
        let mut r = [1, 0, 0, 0];
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
}

/// Tonelli-Shanks square root inside K4 (one of the two roots), given a
/// non-square `nu` of K4. None when `a` is a non-square.
pub struct Sqrt4 {
    s: u32,
    t: u128,
    z: E4, // nu^t
}
impl Sqrt4 {
    pub fn new(f: &FK, nu: &E4) -> Sqrt4 {
        let ord = (f.p as u128).pow(4) - 1;
        let s = ord.trailing_zeros();
        let t = ord >> s;
        Sqrt4 { s, t, z: f.pow(nu, t) }
    }
    pub fn sqrt(&self, f: &FK, a: &E4) -> Option<E4> {
        if *a == [0; 4] {
            return Some([0; 4]);
        }
        if f.chi(a) != 1 {
            return None;
        }
        let one = [1, 0, 0, 0];
        let mut m = self.s;
        let mut c = self.z;
        let mut t = f.pow(a, self.t);
        let mut r = f.pow(a, (self.t + 1) / 2);
        while t != one {
            let mut i = 0;
            let mut tt = t;
            while tt != one {
                tt = f.mul(&tt, &tt);
                i += 1;
                assert!(i < m);
            }
            let mut b = c;
            for _ in 0..(m - i - 1) {
                b = f.mul(&b, &b);
            }
            m = i;
            c = f.mul(&b, &b);
            t = f.mul(&t, &c);
            r = f.mul(&r, &b);
        }
        debug_assert_eq!(f.mul(&r, &r), *a);
        Some(r)
    }
}

/// Affine long-Weierstrass curve over FK (same formulas as curve::Curve).
pub struct FCurve<'a> {
    pub f: &'a FK,
    pub a2: E4,
    pub a4: E4,
    pub a6: E4,
}
impl<'a> FCurve<'a> {
    pub fn rhs(&self, x: &E4) -> E4 {
        let f = self.f;
        let x2 = f.mul(x, x);
        let x3 = f.mul(&x2, x);
        f.add(&f.add(&x3, &f.mul(&self.a2, &x2)), &f.add(&f.mul(&self.a4, x), &self.a6))
    }
    pub fn add(&self, p: &Pt<E4>, q: &Pt<E4>) -> Pt<E4> {
        let f = self.f;
        let (x1, y1) = match p {
            None => return *q,
            Some(v) => *v,
        };
        let (x2, y2) = match q {
            None => return *p,
            Some(v) => *v,
        };
        let lam = if x1 == x2 {
            if f.add(&y1, &y2) == [0; 4] {
                return None;
            }
            let x1s = f.mul(&x1, &x1);
            let num = f.add(&f.add(&f.add(&x1s, &f.add(&x1s, &x1s)), &f.mul(&f.add(&self.a2, &self.a2), &x1)), &self.a4);
            f.mul(&num, &f.inv(&f.add(&y1, &y1)))
        } else {
            f.mul(&f.sub(&y2, &y1), &f.inv(&f.sub(&x2, &x1)))
        };
        let x3 = f.sub(&f.sub(&f.sub(&f.mul(&lam, &lam), &self.a2), &x1), &x2);
        let y3 = f.sub(&f.mul(&lam, &f.sub(&x1, &x3)), &y1);
        Some((x3, y3))
    }
    pub fn mul(&self, p: &Pt<E4>, k: u128) -> Pt<E4> {
        let mut r: Pt<E4> = None;
        let mut b = *p;
        let mut k = k;
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(&r, &b);
            }
            k >>= 1;
            if k > 0 {
                b = self.add(&b, &b);
            }
        }
        r
    }
    /// Same RNG draws as curve::random_point (4 x-coordinates per try, one sign
    /// draw on success); the returned point is the original one up to sign.
    pub fn random_point(&self, sq: &Sqrt4, rng: &mut Rng) -> Pt<E4> {
        let p = self.f.p;
        loop {
            let x = [rng.below(p), rng.below(p), rng.below(p), rng.below(p)];
            let r = self.rhs(&x);
            if let Some(y) = sq.sqrt(self.f, &r) {
                let _ = rng.next();
                return Some((x, y));
            }
        }
    }
}

/// Order of P from a known multiple n (nP = O, n > 0).
pub fn order_from_multiple(c: &FCurve, p: &Pt<E4>, n: u128) -> u128 {
    let mut ord = n;
    for (l, _) in factor_fast(n) {
        while ord % l == 0 && c.mul(p, ord / l).is_none() {
            ord /= l;
        }
    }
    ord
}

/// One n in [lo, hi] with 4 | n and nP = O, by baby-step giant-step on Q = 4P
/// with the +-x trick. None if not found (only possible for small ord(Q)).
fn bsgs_one(c: &FCurve, pt: &Pt<E4>, lo: u128, hi: u128) -> Option<u128> {
    let q = c.mul(pt, 4);
    q?;
    let klo = (lo + 3) / 4;
    let khi = hi / 4;
    if khi < klo {
        return None;
    }
    let wk = khi - klo;
    let mm = ((wk as f64 / 2.0).sqrt() as u128) + 1;
    // baby steps: x(jQ), j = 1..=mm
    let mut baby: HashMap<E4, u128> = HashMap::with_capacity(mm as usize + 1);
    let mut acc = q;
    for j in 1..=mm {
        match acc {
            None => break,
            Some((x, _)) => {
                baby.entry(x).or_insert(j);
            }
        }
        acc = c.add(&acc, &q);
    }
    let step = c.mul(&q, 2 * mm + 1);
    let mut center = klo + mm;
    let mut g = c.mul(&q, center);
    let check = |k: u128| -> Option<u128> {
        if k < klo || k > khi {
            return None;
        }
        let n = 4 * k;
        if n >= lo && n <= hi && c.mul(pt, n).is_none() {
            Some(n)
        } else {
            None
        }
    };
    loop {
        match g {
            None => {
                if let Some(n) = check(center) {
                    return Some(n);
                }
            }
            Some((gx, _)) => {
                if let Some(&j) = baby.get(&gx) {
                    if let Some(n) = check(center + j) {
                        return Some(n);
                    }
                    if center >= j {
                        if let Some(n) = check(center - j) {
                            return Some(n);
                        }
                    }
                }
            }
        }
        if center > khi + mm {
            return None;
        }
        center += 2 * mm + 1;
        g = c.add(&g, &step);
    }
}

/// Exact replay of curve::group_order (same value, same RNG draws), or None
/// after restoring the RNG when the case is outside what the fast path proves.
///
/// Proof sketch of the replay: for a point P of order o > m (m the baby-step
/// count of the original), the original round's `found` set is exactly
/// {n in [lo, lo+width] : o | n}: any such n = lo + i m + j with 0 <= j < m is
/// hit either through G_i = O (j = 0) or through the baby entry jP = -G_i
/// (jP != O as j < o), and nothing else is pushed. The candidate list is then
/// the intersection over rounds, i.e. the elements of the previous list that
/// P_k kills, and the loop stops at the first round with one candidate left.
pub fn group_order_fast(c: &FCurve, sq: &Sqrt4, rng: &mut Rng) -> Option<u128> {
    let saved = rng.0;
    let p = c.f.p as u128;
    let q = p.pow(4);
    let lo = q + 1 - 2 * p * p;
    let width = 4 * p * p;
    let hi = lo + width;
    let m = ((width as f64).sqrt() as u128) + 1;
    let guard = 2 * m + 2;
    let mut cands: Option<Vec<u128>> = None;
    for _round in 0..40 {
        let pt = c.random_point(sq, rng);
        let next: Option<Vec<u128>> = match &cands {
            None => {
                let ord = bsgs_one(c, &pt, lo, hi).map(|n| order_from_multiple(c, &pt, n));
                match ord {
                    Some(o) if o > guard && width / o <= 4096 => {
                        let first = (lo + o - 1) / o * o;
                        let mut v = Vec::new();
                        let mut n = first;
                        while n <= hi {
                            v.push(n);
                            n += o;
                        }
                        Some(v)
                    }
                    _ => None,
                }
            }
            Some(prev) => {
                let kept: Vec<u128> = prev.iter().cloned().filter(|&n| c.mul(&pt, n).is_none()).collect();
                // the true order is in prev and kills P, so kept is non-empty
                match kept.first() {
                    Some(&n0) if order_from_multiple(c, &pt, n0) > guard => Some(kept),
                    _ => None,
                }
            }
        };
        match next {
            None => {
                rng.0 = saved;
                return None;
            }
            Some(v) => {
                if v.len() == 1 {
                    return Some(v[0]);
                }
                cands = Some(v);
            }
        }
    }
    rng.0 = saved;
    None
}

/// group_order through the fast replay, falling back to the original routine.
pub fn group_order(k: &K4, cv: &Curve<K4>, sqrt_k4: &dyn Fn(&E4) -> Option<E4>, nu: &E4, rng: &mut Rng) -> u128 {
    let f = FK::from_k4(k);
    let sq = Sqrt4::new(&f, nu);
    let fc = FCurve { f: &f, a2: cv.a2, a4: cv.a4, a6: cv.a6 };
    match group_order_fast(&fc, &sq, rng) {
        Some(n) => n,
        None => {
            if crate::cover::setup_prof() {
                eprintln!("[setup-prof] group_order fast replay fell back to the original routine");
            }
            crate::curve::group_order(cv, sqrt_k4, rng)
        }
    }
}

// ---- factoring ---------------------------------------------------------------

fn mulmod(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}
fn powmod(mut a: u64, mut e: u64, n: u64) -> u64 {
    let mut r = 1 % n;
    a %= n;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, a, n);
        }
        a = mulmod(a, a, n);
        e >>= 1;
    }
    r
}
/// Deterministic Miller-Rabin for n < 2^64.
pub fn is_prime_u64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for &sp in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % sp == 0 {
            return n == sp;
        }
    }
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    'outer: for &a in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulmod(x, x, n);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}
/// A non-trivial factor of composite odd n (Pollard-Brent rho, deterministic seeds).
fn rho_factor(n: u64) -> u64 {
    let mut c = 1u64;
    loop {
        let f = |x: u64| ((mulmod(x, x, n) as u128 + c as u128) % n as u128) as u64;
        let (mut x, mut y, mut g, mut r, mut q) = (2u64, 2u64, 1u64, 1u64, 1u64);
        let mut ys = 2u64;
        let mm = 128;
        while g == 1 {
            x = y;
            for _ in 0..r {
                y = f(y);
            }
            let mut k = 0;
            while k < r && g == 1 {
                ys = y;
                for _ in 0..mm.min(r - k) {
                    y = f(y);
                    q = mulmod(q, x.abs_diff(y), n);
                }
                g = gcd(q, n);
                k += mm;
            }
            r *= 2;
        }
        if g == n {
            loop {
                ys = f(ys);
                g = gcd(x.abs_diff(ys), n);
                if g > 1 {
                    break;
                }
            }
        }
        if g != n {
            return g;
        }
        c += 1;
    }
}
fn split(n: u64, out: &mut Vec<u64>) {
    if n == 1 {
        return;
    }
    if is_prime_u64(n) {
        out.push(n);
        return;
    }
    let d = rho_factor(n);
    split(d, out);
    split(n / d, out);
}
// ---- u128 factoring for 2^64 <= n < 2^78 (group orders for p >= 2^16) ----------
//
// Bound for the u128 path: n < 2^78. Then every residue is < 2^78, `mulmod128`
// shifts a residue by 32 bits (< 2^110) and adds a residue times a 32-bit limb
// (< 2^110), so nothing overflows u128; and the twelve prime bases 2..37 make
// Miller-Rabin deterministic below 3.18 * 10^23 > 2^78 (Sorenson-Webster, psi_12).
// Group orders for p < 2^17 are below p^4 + 2 p^2 + 1 < 2^68.

/// Largest n (exclusive) for the u128 factoring path.
pub const FACTOR128_MAX: u128 = 1u128 << 78;

/// a b mod n for n < 2^78 and a, b < n (b in three 32-bit limbs, Horner).
fn mulmod128(a: u128, b: u128, n: u128) -> u128 {
    debug_assert!(n < FACTOR128_MAX && a < n && b < n);
    let mut r = 0u128;
    for sh in [64u32, 32, 0] {
        r = (r << 32) % n;
        r = (r + a * ((b >> sh) & 0xFFFF_FFFF)) % n;
    }
    r
}
fn powmod128(mut a: u128, mut e: u128, n: u128) -> u128 {
    let mut r = 1 % n;
    a %= n;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod128(r, a, n);
        }
        a = mulmod128(a, a, n);
        e >>= 1;
    }
    r
}
/// Deterministic Miller-Rabin for n < 2^78 (the u64 test below 2^64).
pub fn is_prime_u128(n: u128) -> bool {
    assert!(n < FACTOR128_MAX);
    if n < (1u128 << 64) {
        return is_prime_u64(n as u64);
    }
    for &sp in &[2u128, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % sp == 0 {
            return false;
        }
    }
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    'outer: for &a in &[2u128, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod128(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulmod128(x, x, n);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}
fn gcd128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}
/// A non-trivial factor of composite odd n, 2^64 <= n < 2^78 (Pollard-Brent rho).
fn rho_factor128(n: u128) -> u128 {
    let mut c = 1u128;
    loop {
        let f = |x: u128| (mulmod128(x, x, n) + c) % n;
        let (mut x, mut y, mut g, mut r, mut q) = (2u128, 2u128, 1u128, 1u64, 1u128);
        let mut ys = 2u128;
        let mm = 128u64;
        while g == 1 {
            x = y;
            for _ in 0..r {
                y = f(y);
            }
            let mut k = 0;
            while k < r && g == 1 {
                ys = y;
                for _ in 0..mm.min(r - k) {
                    y = f(y);
                    q = mulmod128(q, x.abs_diff(y), n);
                }
                g = gcd128(q, n);
                k += mm;
            }
            r *= 2;
        }
        if g == n {
            loop {
                ys = f(ys);
                g = gcd128(x.abs_diff(ys), n);
                if g > 1 {
                    break;
                }
            }
        }
        if g != n {
            return g;
        }
        c += 1;
    }
}
fn split128(n: u128, out: &mut Vec<u128>) {
    if n < (1u128 << 64) {
        let mut v = Vec::new();
        split(n as u64, &mut v);
        out.extend(v.into_iter().map(|x| x as u128));
        return;
    }
    if is_prime_u128(n) {
        out.push(n);
        return;
    }
    let d = rho_factor128(n);
    split128(d, out);
    split128(n / d, out);
}

/// Same output as curve::factor (ascending primes with exponents). n < 2^64: the
/// u64 path (unchanged); 2^64 <= n < 2^78: the u128 path; larger n: trial division.
pub fn factor_fast(n: u128) -> Vec<(u128, u32)> {
    if n >= FACTOR128_MAX {
        return crate::curve::factor(n);
    }
    if n >= (1u128 << 64) {
        let mut n = n;
        let mut ps: Vec<u128> = Vec::new();
        for d in [2u128, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
            while n % d == 0 {
                ps.push(d);
                n /= d;
            }
        }
        split128(n, &mut ps);
        ps.sort();
        let mut out: Vec<(u128, u32)> = Vec::new();
        for x in ps {
            match out.last_mut() {
                Some((q, e)) if *q == x => *e += 1,
                _ => out.push((x, 1)),
            }
        }
        return out;
    }
    let mut n = n as u64;
    let mut ps: Vec<u64> = Vec::new();
    for d in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        while n % d == 0 {
            ps.push(d);
            n /= d;
        }
    }
    split(n, &mut ps);
    ps.sort();
    let mut out: Vec<(u128, u32)> = Vec::new();
    for x in ps {
        match out.last_mut() {
            Some((q, e)) if *q == x as u128 => *e += 1,
            _ => out.push((x as u128, 1)),
        }
    }
    out
}

/// #D(F_p), same value as Setup::count_d.
pub fn count_d(f: &FK, eta: &E4, lam0: &E4) -> u64 {
    let p = f.p;
    let ne = f.norm(eta);
    let mut n: i64 = 1 + legendre(ne, p) as i64;
    for x in 0..p {
        let mx = f.norm(&f.sub(&[x, 0, 0, 0], lam0));
        n += 1 + f.legendre(f.mulm(ne, mx)) as i64;
    }
    n as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cover::{FactorBase, Setup};
    use crate::field::FieldOps;

    #[test]
    fn factor_fast_matches_trial_division() {
        let mut rng = Rng(12345);
        for i in 0..400u64 {
            let n: u128 = match i % 4 {
                0 => (rng.next() >> 20) as u128 + 2,
                1 => (rng.next() >> 40) as u128 + 2,
                2 => {
                    // semiprime of two ~2^20 numbers (made odd)
                    let a = (rng.next() >> 44) | 1;
                    let b = (rng.next() >> 44) | 1;
                    a as u128 * b as u128 + 0
                }
                _ => {
                    let a = (rng.next() >> 50) | 1;
                    (a as u128).pow(2) * 4 * 27
                }
            };
            if n < 2 {
                continue;
            }
            assert_eq!(factor_fast(n), crate::curve::factor(n), "n={n}");
        }
        for n in [2u128, 3, 4, 8, 9, 49, 1 << 40, 65552385173699392, 1048969271773005892] {
            let v = factor_fast(n);
            assert_eq!(v.iter().map(|&(q, e)| q.pow(e)).product::<u128>(), n);
            assert!(v.iter().all(|&(q, _)| is_prime_u64(q as u64)));
        }
        assert_eq!(factor_fast(65552385173699392), crate::curve::factor(65552385173699392));
    }

    #[test]
    fn fk_matches_k4() {
        let mut rng = Rng(7);
        for &p in &[17u64, 101, 16001, 32003, 32771, 49999, 65521, 65537, 115001, 131071] {
            let k = loop {
                let m = vec![rng.below(p), rng.below(p), rng.below(p), rng.below(p), 1];
                if crate::poly::is_irreducible_quartic(&m, p) {
                    break K4::new(p, [m[0], m[1], m[2], m[3]]);
                }
            };
            let f = FK::from_k4(&k);
            // products of p - 1 terms (the largest accumulators: c_3 = 4 (p-1)^2)
            let full = [p - 1; 4];
            assert_eq!(f.mul(&full, &full), k.mul(&full, &full), "p={p}");
            assert_eq!(f.frob(&full), k.frob(&full), "p={p}");
            assert_eq!(f.norm(&full), k.norm(&full), "p={p}");
            assert_eq!(f.inv(&full), k.inv(&full), "p={p}");
            for _ in 0..2000 {
                let a = [rng.below(p), rng.below(p), rng.below(p), rng.below(p)];
                let b = [rng.below(p), rng.below(p), rng.below(p), rng.below(p)];
                assert_eq!(f.mul(&a, &b), k.mul(&a, &b));
                assert_eq!(f.frob(&a), k.frob(&a));
                assert_eq!(f.norm(&a), k.norm(&a));
                assert_eq!(f.chi(&a), k.chi(&a));
                if a != [0; 4] {
                    assert_eq!(f.inv(&a), k.inv(&a));
                }
            }
        }
    }

    /// Fast replay vs the original group_order on random curves over K4 (both
    /// the full-2-torsion curves of the cover and generic curves, where the
    /// 4 | N shortcut fails and the fallback runs): same order, same RNG state.
    #[test]
    fn group_order_fast_matches_original() {
        let mut rng = Rng(99);
        let mut fast_hits = 0;
        for trial in 0..60u64 {
            let p = [17u64, 31, 101, 257, 1009][trial as usize % 5];
            let k = loop {
                let m = vec![rng.below(p), rng.below(p), rng.below(p), rng.below(p), 1];
                if crate::poly::is_irreducible_quartic(&m, p) {
                    break K4::new(p, [m[0], m[1], m[2], m[3]]);
                }
            };
            let nu = loop {
                let c = [rng.below(p), rng.below(p), rng.below(p), rng.below(p)];
                if k.chi(&c) == -1 {
                    break c;
                }
            };
            let l = crate::field::L::new(k.clone(), nu);
            let (a2, a4, a6) = if trial % 2 == 0 {
                let r: Vec<E4> = (0..3).map(|_| [rng.below(p), rng.below(p), rng.below(p), rng.below(p)]).collect();
                let a2 = k.neg(&k.add(&k.add(&r[0], &r[1]), &r[2]));
                let a4 = k.add(&k.add(&k.mul(&r[0], &r[1]), &k.mul(&r[0], &r[2])), &k.mul(&r[1], &r[2]));
                let a6 = k.neg(&k.mul(&k.mul(&r[0], &r[1]), &r[2]));
                (a2, a4, a6)
            } else {
                ([rng.below(p), 0, 0, 0], [rng.below(p), rng.below(p), 0, 1], [rng.below(p), 1, rng.below(p), rng.below(p)])
            };
            // skip singular curves (the original would panic on them)
            let disc_ok = {
                let c = Curve { f: &k, a2, a4, a6 };
                let mut ok = true;
                // a curve with a repeated root has a singular point (x0, 0); test x in Fp only as a cheap filter
                for x in 0..p.min(64) {
                    let xv = [x, 0, 0, 0];
                    if c.rhs(&xv) == [0; 4] {
                        let d = k.add(&k.add(&k.mul(&k.from_u64(3), &k.mul(&xv, &xv)), &k.mul(&k.from_u64(2), &k.mul(&a2, &xv))), &a4);
                        if d == [0; 4] {
                            ok = false;
                        }
                    }
                }
                ok
            };
            if !disc_ok {
                continue;
            }
            let c = Curve { f: &k, a2, a4, a6 };
            let sq = |a: &E4| {
                let s = l.sqrt(&l.emb(a))?;
                if crate::field::L::in_k4(&s) {
                    Some(s.0)
                } else {
                    None
                }
            };
            let seed = rng.next();
            let mut r1 = Rng(seed);
            let mut r2 = Rng(seed);
            let n_slow = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| crate::curve::group_order(&c, &sq, &mut r1)));
            let n_fast = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| group_order(&k, &c, &sq, &nu, &mut r2)));
            match (n_slow, n_fast) {
                (Ok(a), Ok(b)) => {
                    assert_eq!(a, b, "trial {trial} p={p}");
                    assert_eq!(r1.0, r2.0, "rng state trial {trial}");
                }
                (Err(_), Err(_)) => {}
                _ => panic!("one path panicked, the other did not (trial {trial})"),
            }
            // count how often the replay itself (no fallback) decides
            let f = FK::from_k4(&k);
            let s4 = Sqrt4::new(&f, &nu);
            let fc = FCurve { f: &f, a2, a4, a6 };
            let mut r3 = Rng(seed);
            if let Some(n) = group_order_fast(&fc, &s4, &mut r3) {
                fast_hits += 1;
                assert_eq!(r3.0, r1.0);
                let mut r4 = Rng(seed);
                assert_eq!(n, crate::curve::group_order(&c, &sq, &mut r4));
            }
        }
        assert!(fast_hits >= 10, "fast replay decided only {fast_hits} times");
    }

    /// p < 2^16 is accepted (65521, the largest prime below 2^16, with the fast
    /// setup), and the instance is consistent: the fast factor base equals the
    /// reference one, count_d agrees, r is prime with r^2 not dividing #E and
    /// the group order lies in the Hasse interval.
    #[test]
    fn setup_accepts_p_65521() {
        let p = 65521u64;
        let mut rng = Rng(2);
        let s = Setup::new_with(p, -1, &mut rng, true);
        let pp = p as u128;
        assert!(s.order >= pp.pow(4) + 1 - 2 * pp * pp && s.order <= pp.pow(4) + 1 + 2 * pp * pp);
        assert!(is_prime_u64(s.r) && s.order % s.r as u128 == 0 && (s.order / s.r as u128) % s.r as u128 != 0);
        assert!((s.r as u128) * 128 >= pp.pow(4) && s.r < (1u64 << 62));
        assert_eq!(factor_fast(s.order), crate::curve::factor(s.order / s.r as u128).into_iter().chain([(s.r as u128, 1)]).collect::<Vec<_>>());
        let f = FK::from_k4(s.k());
        assert_eq!(count_d(&f, &s.eta, &s.lams[0]), s.count_d());
        assert_eq!(s.d_count, s.count_d());
        let fs = FactorBase::build_slow(&s);
        let ff = FactorBase::build_fast(&s);
        assert_eq!((fs.xs, fs.w0, fs.xidx, fs.rows), (ff.xs, ff.w0, ff.xidx, ff.rows));
        let c = s.curve();
        let sq = |a: &E4| s.sqrt_k4(a);
        let pt = crate::curve::random_point(&c, &sq, &mut rng);
        assert!(c.mul(&pt, s.order).is_none());
        let gen = c.mul(&pt, s.order / s.r as u128);
        assert!(c.mul(&gen, s.r as u128).is_none());
    }

    #[test]
    #[should_panic]
    fn setup_rejects_p_131101() {
        // the smallest prime above 2^17
        let mut rng = Rng(1);
        Setup::new_with(131101, 1, &mut rng, true);
    }

    /// 2^64 <= n < 2^78 (group orders for p >= 2^16): the u128 path equals trial
    /// division where trial division is affordable (second-largest prime factor
    /// and the square root of the largest below ~2^25), and equals the known
    /// factorization of products of large primes (each prime certified by the u64
    /// Miller-Rabin of the old path) where it is not. Boundaries 2^64 - 1 .. 2^64 + 1
    /// and the smallest prime above 2^64.
    #[test]
    fn factor_fast_u128_matches_trial_division_and_known_factors() {
        let mut rng = Rng(0x78);
        let next_prime = |lo: u64| -> u64 {
            let mut x = lo | 1;
            while !is_prime_u64(x) {
                x += 2;
            }
            x
        };
        // affordable for curve::factor
        let mut cheap: Vec<u128> = vec![(1u128 << 64) - 1, 1u128 << 64, (1u128 << 64) + 1, 1u128 << 77, 3u128.pow(48)];
        for _ in 0..12 {
            let a = next_prime((rng.next() >> 41) | (1 << 22)) as u128; // 2^22..2^23
            let b = next_prime((rng.next() >> 41) | (1 << 22)) as u128;
            let c = next_prime((rng.next() >> 39) | (1 << 24)) as u128; // 2^24..2^25: n in [2^68, 2^78)
            let h = [1u128, 4, 12, 112, 128][rng.below(5) as usize];
            let n = a * b * c * h;
            assert!(n >= 1u128 << 64 && n < FACTOR128_MAX, "n={n}");
            cheap.push(n);
        }
        for n in cheap {
            assert!(n < FACTOR128_MAX);
            assert_eq!(factor_fast(n), crate::curve::factor(n), "n={n}");
        }
        // products of large primes (trial division would take minutes)
        for i in 0..40 {
            let mut fs: Vec<u128> = match i % 4 {
                // cofactor h * prime ~2^61 (the shape of a group order near p = 2^17)
                0 => vec![[64u128, 112, 120, 126][rng.below(4) as usize], next_prime((rng.next() >> 3) | (1 << 60)) as u128],
                // two ~2^33 primes and a ~2^5 cofactor (balanced split for rho)
                1 => vec![next_prime((rng.next() >> 31) | (1 << 32)) as u128, next_prime((rng.next() >> 31) | (1 << 32)) as u128, 41],
                // a prime square times a large prime
                2 => {
                    let a = next_prime((rng.next() >> 44) | (1 << 19)) as u128;
                    vec![a, a, next_prime((rng.next() >> 31) | (1 << 32)) as u128, 6]
                }
                // three ~2^22 primes and 2^3
                _ => vec![next_prime((rng.next() >> 42) | (1 << 21)) as u128, next_prime((rng.next() >> 42) | (1 << 21)) as u128, next_prime((rng.next() >> 42) | (1 << 21)) as u128, 2u128.pow(3)],
            };
            let n: u128 = fs.iter().product();
            if n < (1u128 << 64) || n >= FACTOR128_MAX {
                continue;
            }
            // expected list: the large primes as built, the small cofactors by trial division
            let mut ps: Vec<u128> = Vec::new();
            for f in fs.drain(..) {
                if f < (1 << 40) {
                    for (q, e) in crate::curve::factor(f) {
                        ps.extend(std::iter::repeat(q).take(e as usize));
                    }
                } else {
                    ps.push(f);
                }
            }
            ps.sort();
            let mut want: Vec<(u128, u32)> = Vec::new();
            for x in ps {
                match want.last_mut() {
                    Some((q, e)) if *q == x => *e += 1,
                    _ => want.push((x, 1)),
                }
            }
            let got = factor_fast(n);
            assert_eq!(got, want, "n={n}");
            assert!(got.iter().all(|&(q, _)| is_prime_u128(q)));
        }
        // primality at the 2^64 boundary: 2^64 + 13 is the smallest prime above 2^64
        assert!(is_prime_u128((1u128 << 64) + 13));
        assert!((1..13).all(|d| !is_prime_u128((1u128 << 64) + d)));
        assert_eq!(factor_fast((1u128 << 64) + 13), vec![((1u128 << 64) + 13, 1)]);
        // mulmod128 on extreme residues: (n - 1)^2 = 1 and (n - 1)(n - 2) = 2 mod n for
        // the largest n of the path; Fermat for the prime 2^64 + 13
        let n = FACTOR128_MAX - 1;
        assert_eq!(mulmod128(n - 1, n - 1, n), 1);
        assert_eq!(mulmod128(n - 1, n - 2, n), 2);
        let q = (1u128 << 64) + 13;
        assert_eq!(powmod128(3, q - 1, q), 1);
    }

    /// p = 131071 (the largest prime below 2^17): Setup accepts it (fast setup), and
    /// the instance is consistent as at 65521; r < 2^62 (the LA bound), so the
    /// cofactor is at least p^4 / 2^62 > 64.
    #[test]
    fn setup_accepts_p_131071() {
        let p = 131071u64;
        let mut rng = Rng(1);
        let s = Setup::new_with(p, 1, &mut rng, true);
        let pp = p as u128;
        assert!(s.order >= pp.pow(4) + 1 - 2 * pp * pp && s.order <= pp.pow(4) + 1 + 2 * pp * pp);
        assert!(s.order >= 1u128 << 64, "the u128 factoring path is the one used");
        assert!(is_prime_u64(s.r) && s.order % s.r as u128 == 0 && (s.order / s.r as u128) % s.r as u128 != 0);
        assert!((s.r as u128) * 128 >= pp.pow(4) && s.r < (1u64 << 62));
        assert!(s.order / s.r as u128 > 64 && s.order / s.r as u128 <= 128);
        assert_eq!(factor_fast(s.order), crate::curve::factor(s.order / s.r as u128).into_iter().chain([(s.r as u128, 1)]).collect::<Vec<_>>());
        let f = FK::from_k4(s.k());
        assert_eq!(count_d(&f, &s.eta, &s.lams[0]), s.count_d());
        assert_eq!(s.d_count, s.count_d());
        let fs = FactorBase::build_slow(&s);
        let ff = FactorBase::build_fast(&s);
        assert_eq!((fs.xs, fs.w0, fs.xidx, fs.rows), (ff.xs, ff.w0, ff.xidx, ff.rows));
        let c = s.curve();
        let sq = |a: &E4| s.sqrt_k4(a);
        let pt = crate::curve::random_point(&c, &sq, &mut rng);
        assert!(c.mul(&pt, s.order).is_none());
        let gen = c.mul(&pt, s.order / s.r as u128);
        assert!(c.mul(&gen, s.r as u128).is_none());
        // the fast BSGS replay at this size agrees with the order the setup found
        let fk = FK::from_k4(s.k());
        let s4 = Sqrt4::new(&fk, &s.l.nu);
        let fc = FCurve { f: &fk, a2: s.a2, a4: s.a4, a6: s.a6 };
        let pt2 = fc.random_point(&s4, &mut rng);
        assert!(fc.mul(&pt2, s.order).is_none());
    }

    /// Setup at the first prime above 2^16, both setup paths: the same instance.
    #[test]
    fn setup_fast_identical_at_p_65537() {
        let p = 65537u64;
        let (mut r1, mut r2) = (Rng(3), Rng(3));
        let a = Setup::new_with(p, 1, &mut r1, false);
        let b = Setup::new_with(p, 1, &mut r2, true);
        assert_eq!(r1.0, r2.0);
        assert_eq!((a.order, a.r, a.d_count), (b.order, b.r, b.d_count));
        assert_eq!((a.eta, a.lams, a.mu, a.a2, a.a4, a.a6, a.gamma), (b.eta, b.lams, b.mu, b.a2, b.a4, b.a6, b.gamma));
        assert!(a.r < (1u64 << 62));
    }

    /// Setup::new fast vs default: same instance, same RNG state afterwards;
    /// FactorBase::build_fast vs build_slow and count_d agree.
    #[test]
    fn setup_fast_identical_instance_and_rng() {
        let (mut built, mut panicked) = (0, 0);
        for (p, seeds) in [(17u64, 0..40u64), (101, 0..20), (1009, 0..8), (4001, 0..4)] {
            for seed in seeds {
                let tw = if seed % 2 == 1 { 1 } else { -1 };
                let mut r1 = Rng(seed);
                let mut r2 = Rng(seed);
                // the original panics when the group order stays ambiguous (tiny p); the fast path must too
                let a = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| Setup::new_with(p, tw, &mut r1, false)));
                let b = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| Setup::new_with(p, tw, &mut r2, true)));
                let (a, b) = match (a, b) {
                    (Ok(a), Ok(b)) => (a, b),
                    (Err(_), Err(_)) => {
                        panicked += 1;
                        continue;
                    }
                    _ => panic!("one path panicked, the other did not (p={p} seed={seed})"),
                };
                built += 1;
                assert_eq!(r1.0, r2.0, "p={p} seed={seed}");
                assert_eq!((a.order, a.r, a.d_count), (b.order, b.r, b.d_count));
                assert_eq!((a.eta, a.lams, a.mu, a.a2, a.a4, a.a6, a.gamma), (b.eta, b.lams, b.mu, b.a2, b.a4, b.a6, b.gamma));
                let f = FK::from_k4(a.k());
                assert_eq!(count_d(&f, &a.eta, &a.lams[0]), a.count_d());
                let fs = FactorBase::build_slow(&a);
                let ff = FactorBase::build_fast(&b);
                assert_eq!(fs.xs, ff.xs);
                assert_eq!(fs.w0, ff.w0);
                assert_eq!(fs.xidx, ff.xidx);
                assert_eq!(fs.rows, ff.rows);
                assert_eq!(fs.pick, ff.pick);
            }
        }
        eprintln!("setup_fast test: {built} instances identical, {panicked} panicked on both paths");
        assert!(built >= 60);
    }
}
