//! Small-characteristic generic polynomial-basis extension fields for CAS replay.
use crate::toy_field::{inv, prime};
pub type F = Vec<u64>;
pub type Point = Option<(F, F)>;
#[derive(Clone)]
pub struct Field {
    pub p: u64,
    pub modulus: F,
    pub degree: usize,
}
impl Field {
    pub fn new(p: u64, modulus: F) -> Self {
        assert!(prime(p) && p < 32768);
        assert_eq!(modulus.last(), Some(&1));
        assert!(modulus.iter().all(|v| *v < p));
        let degree = modulus.len() - 1;
        assert!((2..=12).contains(&degree));
        Self { p, modulus, degree }
    }
    pub fn trim(&self, mut a: F) -> F {
        while a.last() == Some(&0) {
            a.pop();
        }
        a
    }
    fn ps(&self, a: &[u64], b: &[u64]) -> F {
        self.trim(
            (0..a.len().max(b.len()))
                .map(|i| (*a.get(i).unwrap_or(&0) + self.p - *b.get(i).unwrap_or(&0)) % self.p)
                .collect(),
        )
    }
    fn pm(&self, a: &[u64], b: &[u64]) -> F {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let mut c = vec![0; a.len() + b.len() - 1];
        for i in 0..a.len() {
            for j in 0..b.len() {
                c[i + j] = (c[i + j] + a[i] * b[j]) % self.p;
            }
        }
        self.trim(c)
    }
    fn scale(&self, a: &[u64], n: u64) -> F {
        self.trim(a.iter().map(|v| v * n % self.p).collect())
    }
    fn dr(&self, a: &[u64], b: &[u64]) -> (F, F) {
        assert!(!b.is_empty());
        let mut a = self.trim(a.to_vec());
        let mut q = vec![0; a.len().saturating_sub(b.len()) + 1];
        let d = inv(*b.last().unwrap(), self.p);
        while a.len() >= b.len() {
            let i = a.len() - b.len();
            let c = a.last().unwrap() * d % self.p;
            q[i] = c;
            for j in 0..b.len() {
                a[i + j] = (a[i + j] + self.p - c * b[j] % self.p) % self.p;
            }
            a = self.trim(a);
        }
        (self.trim(q), a)
    }
    fn gcd(&self, mut a: F, mut b: F) -> F {
        while !b.is_empty() {
            let r = self.dr(&a, &b).1;
            a = b;
            b = r;
        }
        let d = inv(*a.last().unwrap(), self.p);
        self.scale(&a, d)
    }
    pub fn c(&self, n: u64) -> F {
        let mut v = vec![0; self.degree];
        v[0] = n % self.p;
        v
    }
    pub fn reduce(&self, a: &[u64]) -> F {
        let mut a = self.dr(a, &self.modulus).1;
        a.resize(self.degree, 0);
        a
    }
    pub fn add(&self, a: &F, b: &F) -> F {
        a.iter().zip(b).map(|(a, b)| (a + b) % self.p).collect()
    }
    pub fn neg(&self, a: &F) -> F {
        a.iter().map(|a| (self.p - a) % self.p).collect()
    }
    pub fn sub(&self, a: &F, b: &F) -> F {
        self.add(a, &self.neg(b))
    }
    pub fn mul(&self, a: &F, b: &F) -> F {
        self.reduce(&self.pm(a, b))
    }
    pub fn inv(&self, a: &F) -> F {
        let (mut r0, mut r1) = (self.modulus.clone(), self.trim(a.clone()));
        let (mut s0, mut s1) = (vec![], vec![1]);
        while !r1.is_empty() {
            let (q, r) = self.dr(&r0, &r1);
            r0 = r1;
            r1 = r;
            let s = self.ps(&s0, &self.pm(&q, &s1));
            s0 = s1;
            s1 = s;
        }
        assert_eq!(r0.len(), 1, "noninvertible field element");
        let v = self.reduce(&self.scale(&s0, inv(r0[0], self.p)));
        assert_eq!(self.mul(a, &v), self.c(1));
        v
    }
    pub fn div(&self, a: &F, b: &F) -> F {
        self.mul(a, &self.inv(b))
    }
    pub fn pow(&self, a: &F, mut n: u64) -> F {
        let mut x = a.clone();
        let mut y = self.c(1);
        while n != 0 {
            if n & 1 != 0 {
                y = self.mul(&y, &x);
            }
            x = self.mul(&x, &x);
            n >>= 1;
        }
        y
    }
    pub fn irreducible(&self) -> bool {
        let mut x = self.c(0);
        x[1] = 1;
        let mut y = x.clone();
        let tests = (2..=self.degree)
            .filter(|d| prime(*d as u64) && self.degree % d == 0)
            .map(|d| self.degree / d)
            .collect::<Vec<_>>();
        for i in 1..=self.degree {
            y = self.pow(&y, self.p);
            if tests.contains(&i)
                && self
                    .gcd(self.modulus.clone(), self.trim(self.sub(&y, &x)))
                    .len()
                    != 1
            {
                return false;
            }
        }
        y == x
    }
    pub fn parse(&self, s: &str) -> F {
        let s = s
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .collect::<String>()
            .replace('-', "+-");
        let mut out = self.c(0);
        for t in s.split('+').filter(|s| !s.is_empty()) {
            let (negative, t) = t.strip_prefix('-').map(|t| (true, t)).unwrap_or((false, t));
            let (c, mon) = if let Some((c, mon)) = t.split_once('*') {
                (c.parse::<u64>().unwrap(), mon)
            } else if t.starts_with('z') {
                (1, t)
            } else {
                (t.parse::<u64>().unwrap(), "")
            };
            let degree = if mon.is_empty() {
                0
            } else if mon == "z" {
                1
            } else {
                mon.strip_prefix("z^")
                    .expect("non-polynomial finite-field coordinate")
                    .parse::<usize>()
                    .unwrap()
            };
            assert!(degree < self.degree);
            let c = c % self.p;
            out[degree] = (out[degree] + if negative { (self.p - c) % self.p } else { c }) % self.p;
        }
        out
    }
    pub fn on(&self, p: &Point, a: u64, b: u64) -> bool {
        p.as_ref().is_none_or(|(x, y)| {
            self.mul(y, y)
                == self.add(
                    &self.add(&self.mul(&self.mul(x, x), x), &self.mul(&self.c(a), x)),
                    &self.c(b),
                )
        })
    }
    pub fn pn(&self, p: &Point) -> Point {
        p.as_ref().map(|(x, y)| (x.clone(), self.neg(y)))
    }
    pub fn pa(&self, p: &Point, q: &Point, a: u64) -> Point {
        let (Some((x, y)), Some((u, v))) = (p, q) else {
            return p.clone().or(q.clone());
        };
        if x == u && self.add(y, v) == self.c(0) {
            return None;
        }
        let slope = if x == u {
            self.div(
                &self.add(&self.mul(&self.c(3), &self.mul(x, x)), &self.c(a)),
                &self.mul(&self.c(2), y),
            )
        } else {
            self.div(&self.sub(v, y), &self.sub(u, x))
        };
        let xx = self.sub(&self.sub(&self.mul(&slope, &slope), x), u);
        let yy = self.sub(&self.mul(&slope, &self.sub(x, &xx)), y);
        Some((xx, yy))
    }
    pub fn scalar(&self, p: &Point, mut n: u64, a: u64) -> Point {
        let mut x = p.clone();
        let mut y = None;
        while n != 0 {
            if n & 1 != 0 {
                y = self.pa(&y, &x, a);
            }
            x = self.pa(&x, &x, a);
            n >>= 1;
        }
        y
    }
    pub fn frob(&self, p: &Point) -> Point {
        p.as_ref()
            .map(|(x, y)| (self.pow(x, self.p), self.pow(y, self.p)))
    }
    fn line(&self, p: &Point, q: &Point, r: &Point, a: u64) -> F {
        let (Some((x, y)), Some((u, v)), Some((rx, ry))) = (p, q, r) else {
            return self.c(1);
        };
        let sum = self.pa(p, q, a);
        if sum.is_none() {
            return self.sub(rx, x);
        }
        let slope = if x == u {
            self.div(
                &self.add(&self.mul(&self.c(3), &self.mul(x, x)), &self.c(a)),
                &self.mul(&self.c(2), y),
            )
        } else {
            self.div(&self.sub(v, y), &self.sub(u, x))
        };
        self.div(
            &self.sub(&self.sub(ry, y), &self.mul(&slope, &self.sub(rx, x))),
            &self.sub(rx, &sum.unwrap().0),
        )
    }
    fn miller(&self, p: &Point, q: &Point, ell: u64, a: u64) -> F {
        let mut v = p.clone();
        let mut f = self.c(1);
        let bits = 64 - ell.leading_zeros();
        for i in (0..bits - 1).rev() {
            f = self.mul(&self.mul(&f, &f), &self.line(&v, &v, q, a));
            v = self.pa(&v, &v, a);
            if ell >> i & 1 != 0 {
                f = self.mul(&f, &self.line(&v, p, q, a));
                v = self.pa(&v, p, a);
            }
        }
        assert_eq!(v, None);
        f
    }
    pub fn weil(&self, p: &Point, q: &Point, ell: u64, a: u64) -> F {
        if p.is_none() || q.is_none() || (0..ell).any(|i| self.scalar(p, i, a) == *q) {
            return self.c(1);
        }
        let z = self.div(&self.miller(p, q, ell, a), &self.miller(q, p, ell, a));
        if ell % 2 == 1 {
            self.neg(&z)
        } else {
            z
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quadratic_field_and_independent_torsion_pairing_have_correct_sign() {
        let k = Field::new(5, vec![3, 0, 1]);
        assert!(k.irreducible());
        let z = vec![0, 1];
        assert_eq!(k.mul(&z, &z), k.c(2));
        for a in 0..5 {
            for b in 0..5 {
                let x = vec![a, b];
                assert_eq!(k.pow(&x, 25), x);
                if x != k.c(0) {
                    assert_eq!(k.mul(&x, &k.inv(&x)), k.c(1));
                }
            }
        }
        let p = Some((k.c(0), k.c(1)));
        let q = Some((k.c(1), z));
        assert!(k.on(&p, 0, 1) && k.on(&q, 0, 1));
        assert_eq!(k.scalar(&p, 3, 0), None);
        assert_eq!(k.scalar(&q, 3, 0), None);
        let w = k.weil(&p, &q, 3, 0);
        assert_eq!(w, vec![2, 2]);
        assert_eq!(k.pow(&w, 3), k.c(1));
        assert_eq!(k.weil(&q, &p, 3, 0), k.inv(&w));
        assert_eq!(k.parse("2 + 2*z"), w);
        assert!(!Field::new(5, vec![4, 0, 1]).irreducible());
    }
}
