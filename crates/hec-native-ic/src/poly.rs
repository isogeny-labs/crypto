//! Dense polynomials over `F_p` in a fixed-size array (no allocation).
//!
//! `CAP` bounds the number of coefficients.  Genus 4 is the worst case: a
//! Cantor composition forms products of degree up to `4g − 1 = 15` before it
//! divides, so 24 slots leave margin.  Coefficients at index `>= n` are
//! always zero, so derived equality and hashing are on the polynomial.

use crate::fp::Fp;

pub const CAP: usize = 24;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Poly {
    pub c: [u64; CAP],
    /// Number of coefficients: `deg + 1`, or 0 for the zero polynomial.
    pub n: usize,
}

impl Poly {
    #[inline]
    pub const fn zero() -> Self {
        Poly { c: [0; CAP], n: 0 }
    }

    #[inline]
    pub fn constant(a: u64) -> Self {
        let mut p = Poly::zero();
        if a != 0 {
            p.c[0] = a;
            p.n = 1;
        }
        p
    }

    #[inline]
    pub fn one() -> Self {
        Poly::constant(1)
    }

    /// `x`.
    #[inline]
    pub fn x() -> Self {
        let mut p = Poly::zero();
        p.c[1] = 1;
        p.n = 2;
        p
    }

    /// `x − a`.
    #[inline]
    pub fn x_minus(a: u64, fp: &Fp) -> Self {
        let mut p = Poly::zero();
        p.c[0] = fp.neg(a % fp.p);
        p.c[1] = 1;
        p.n = 2;
        p
    }

    /// From coefficients, low degree first (already reduced mod p).
    pub fn from_coeffs(cs: &[u64]) -> Self {
        assert!(cs.len() <= CAP);
        let mut p = Poly::zero();
        p.c[..cs.len()].copy_from_slice(cs);
        p.n = cs.len();
        p.trim();
        p
    }

    #[inline]
    fn trim(&mut self) {
        while self.n > 0 && self.c[self.n - 1] == 0 {
            self.n -= 1;
        }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.n == 0
    }

    /// Degree, `−1` for zero.
    #[inline]
    pub fn deg(&self) -> isize {
        self.n as isize - 1
    }

    #[inline]
    pub fn lc(&self) -> u64 {
        if self.n == 0 {
            0
        } else {
            self.c[self.n - 1]
        }
    }

    pub fn add(&self, o: &Poly, fp: &Fp) -> Poly {
        let mut r = Poly::zero();
        let n = self.n.max(o.n);
        for i in 0..n {
            r.c[i] = fp.add(self.c[i], o.c[i]);
        }
        r.n = n;
        r.trim();
        r
    }

    pub fn sub(&self, o: &Poly, fp: &Fp) -> Poly {
        let mut r = Poly::zero();
        let n = self.n.max(o.n);
        for i in 0..n {
            r.c[i] = fp.sub(self.c[i], o.c[i]);
        }
        r.n = n;
        r.trim();
        r
    }

    pub fn neg(&self, fp: &Fp) -> Poly {
        let mut r = *self;
        for i in 0..r.n {
            r.c[i] = fp.neg(r.c[i]);
        }
        r
    }

    pub fn scale(&self, k: u64, fp: &Fp) -> Poly {
        if k == 0 {
            return Poly::zero();
        }
        let mut r = *self;
        for i in 0..r.n {
            r.c[i] = fp.mul(r.c[i], k);
        }
        r
    }

    pub fn mul(&self, o: &Poly, fp: &Fp) -> Poly {
        if self.n == 0 || o.n == 0 {
            return Poly::zero();
        }
        let n = self.n + o.n - 1;
        assert!(n <= CAP, "Poly::mul: degree overflow ({n} > {CAP})");
        // Accumulate in u128 and reduce every 4 terms: 4 products are
        // < 4p² < p·2^64 (p < 2^62), the bound `reduce128` needs.
        let mut r = Poly::zero();
        for k in 0..n {
            let lo = k.saturating_sub(o.n - 1);
            let hi = k.min(self.n - 1);
            let mut acc: u128 = 0;
            let mut out = 0u64;
            let mut cnt = 0;
            for i in lo..=hi {
                acc += self.c[i] as u128 * o.c[k - i] as u128;
                cnt += 1;
                if cnt == 4 {
                    out = fp.add(out, fp.reduce128(acc));
                    acc = 0;
                    cnt = 0;
                }
            }
            r.c[k] = fp.add(out, fp.reduce128(acc));
        }
        r.n = n;
        r.trim();
        r
    }

    /// Quotient and remainder.  Panics on division by zero.
    pub fn divrem(&self, d: &Poly, fp: &Fp) -> (Poly, Poly) {
        assert!(!d.is_zero(), "Poly::divrem by zero");
        if self.n < d.n {
            return (Poly::zero(), *self);
        }
        let inv = fp.inv(d.lc());
        let mut r = *self;
        let mut q = Poly::zero();
        let dn = d.n;
        q.n = self.n - dn + 1;
        for k in (0..q.n).rev() {
            let coef = fp.mul(r.c[k + dn - 1], inv);
            q.c[k] = coef;
            if coef != 0 {
                for j in 0..dn {
                    r.c[k + j] = fp.sub(r.c[k + j], fp.mul(coef, d.c[j]));
                }
            }
        }
        r.n = dn - 1;
        r.trim();
        // Clear anything above the remainder (it is zero by construction,
        // but keep the invariant explicit).
        for i in r.n..CAP {
            r.c[i] = 0;
        }
        q.trim();
        (q, r)
    }

    #[inline]
    pub fn rem(&self, d: &Poly, fp: &Fp) -> Poly {
        self.divrem(d, fp).1
    }

    /// Exact division; debug-asserts a zero remainder.
    pub fn div_exact(&self, d: &Poly, fp: &Fp) -> Poly {
        let (q, r) = self.divrem(d, fp);
        debug_assert!(r.is_zero(), "Poly::div_exact: non-zero remainder");
        q
    }

    pub fn monic(&self, fp: &Fp) -> Poly {
        if self.is_zero() || self.lc() == 1 {
            return *self;
        }
        self.scale(fp.inv(self.lc()), fp)
    }

    pub fn eval(&self, x: u64, fp: &Fp) -> u64 {
        let mut acc = 0u64;
        for i in (0..self.n).rev() {
            acc = fp.add(fp.mul(acc, x), self.c[i]);
        }
        acc
    }

    pub fn deriv(&self, fp: &Fp) -> Poly {
        if self.n <= 1 {
            return Poly::zero();
        }
        let mut r = Poly::zero();
        for i in 1..self.n {
            r.c[i - 1] = fp.mul(self.c[i], (i as u64) % fp.p);
        }
        r.n = self.n - 1;
        r.trim();
        r
    }

    /// Monic gcd (zero if both are zero).
    pub fn gcd(a: &Poly, b: &Poly, fp: &Fp) -> Poly {
        let (mut x, mut y) = (*a, *b);
        while !y.is_zero() {
            let r = x.rem(&y, fp);
            x = y;
            y = r;
        }
        x.monic(fp)
    }

    /// `(g, s, t)` with `g = s·a + t·b` monic (all zero if `a = b = 0`).
    pub fn xgcd(a: &Poly, b: &Poly, fp: &Fp) -> (Poly, Poly, Poly) {
        let (mut r0, mut r1) = (*a, *b);
        let (mut s0, mut s1) = (Poly::one(), Poly::zero());
        let (mut t0, mut t1) = (Poly::zero(), Poly::one());
        while !r1.is_zero() {
            let (q, r) = r0.divrem(&r1, fp);
            r0 = r1;
            r1 = r;
            let s2 = s0.sub(&q.mul(&s1, fp), fp);
            s0 = s1;
            s1 = s2;
            let t2 = t0.sub(&q.mul(&t1, fp), fp);
            t0 = t1;
            t1 = t2;
        }
        if r0.is_zero() {
            return (Poly::zero(), Poly::zero(), Poly::zero());
        }
        let inv = fp.inv(r0.lc());
        (r0.scale(inv, fp), s0.scale(inv, fp), t0.scale(inv, fp))
    }

    #[inline]
    pub fn mulmod(&self, o: &Poly, m: &Poly, fp: &Fp) -> Poly {
        self.mul(o, fp).rem(m, fp)
    }

    /// `self^e mod m`.
    pub fn powmod(&self, mut e: u64, m: &Poly, fp: &Fp) -> Poly {
        let mut base = self.rem(m, fp);
        let mut r = Poly::one().rem(m, fp);
        while e > 0 {
            if e & 1 == 1 {
                r = r.mulmod(&base, m, fp);
            }
            e >>= 1;
            if e > 0 {
                base = base.mulmod(&base, m, fp);
            }
        }
        r
    }

    /// Resultant over a field (Euclidean recursion).
    pub fn resultant(a: &Poly, b: &Poly, fp: &Fp) -> u64 {
        if a.is_zero() || b.is_zero() {
            return 0;
        }
        let (mut a, mut b) = (*a, *b);
        let mut acc = 1u64;
        loop {
            let (da, db) = (a.deg(), b.deg());
            if db == 0 {
                return fp.mul(acc, fp.pow(b.lc(), da as u64));
            }
            if da < db {
                // Res(a, b) = (−1)^{da·db} Res(b, a)
                if (da * db) % 2 == 1 {
                    acc = fp.neg(acc);
                }
                std::mem::swap(&mut a, &mut b);
                continue;
            }
            let r = a.rem(&b, fp);
            if r.is_zero() {
                return 0;
            }
            // Res(a, b) = (−1)^{da·db} lc(b)^{da − dr} Res(b, r)
            let dr = r.deg();
            if (da * db) % 2 == 1 {
                acc = fp.neg(acc);
            }
            acc = fp.mul(acc, fp.pow(b.lc(), (da - dr) as u64));
            a = b;
            b = r;
        }
    }

    /// Discriminant of a monic polynomial: `(−1)^{n(n−1)/2} Res(u, u')`.
    pub fn disc_monic(&self, fp: &Fp) -> u64 {
        let n = self.deg();
        let r = Poly::resultant(self, &self.deriv(fp), fp);
        if ((n * (n - 1)) / 2) % 2 == 1 {
            fp.neg(r)
        } else {
            r
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    fn rand_poly(rng: &mut Rng, deg: usize, fp: &Fp) -> Poly {
        let mut cs = vec![0u64; deg + 1];
        for c in cs.iter_mut() {
            *c = rng.below(fp.p);
        }
        cs[deg] = 1 + rng.below(fp.p - 1);
        Poly::from_coeffs(&cs)
    }

    #[test]
    fn divrem_xgcd_roundtrip() {
        let fp = Fp::new(1_000_003);
        let mut rng = Rng::new(7);
        for _ in 0..500 {
            let (da, db) = (1 + rng.below(9) as usize, 1 + rng.below(6) as usize);
            let a = rand_poly(&mut rng, da, &fp);
            let b = rand_poly(&mut rng, db, &fp);
            let (q, r) = a.divrem(&b, &fp);
            assert_eq!(q.mul(&b, &fp).add(&r, &fp), a);
            assert!(r.deg() < b.deg());
            let (g, s, t) = Poly::xgcd(&a, &b, &fp);
            assert_eq!(s.mul(&a, &fp).add(&t.mul(&b, &fp), &fp), g);
            assert_eq!(g.lc(), 1);
            assert!(a.rem(&g, &fp).is_zero() && b.rem(&g, &fp).is_zero());
        }
    }

    #[test]
    fn discriminant_matches_closed_form_cubic() {
        let fp = Fp::new(101);
        let mut rng = Rng::new(3);
        for _ in 0..300 {
            let (b, c, d) = (rng.below(101), rng.below(101), rng.below(101));
            let u = Poly::from_coeffs(&[d, c, b, 1]);
            // b²c² − 4c³ − 4b³d − 27d² + 18bcd
            let i = |v: u64| v as i128;
            let closed = i(b) * i(b) * i(c) * i(c) - 4 * i(c).pow(3) - 4 * i(b).pow(3) * i(d)
                - 27 * i(d) * i(d)
                + 18 * i(b) * i(c) * i(d);
            assert_eq!(u.disc_monic(&fp) as i128, closed.rem_euclid(101));
        }
    }
}
