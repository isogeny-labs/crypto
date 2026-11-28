//! Exact arbitrary-prime arithmetic used for small torsion certificates.
//! Fp4 = Fp[v]/(v^4-2e*v^2+e^2-d), w=v^2-e, w^2=d.
use num_bigint::BigUint;
use num_traits::{One, Zero};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct F(pub [BigUint; 4]);
pub type Point = Option<(F, F)>;
pub type BPoly = Vec<BigUint>;
#[derive(Clone)]
pub struct Field {
    pub p: BigUint,
    pub d: BigUint,
    pub eta: BigUint,
}
impl Field {
    pub fn new(p: BigUint, d: BigUint) -> Self {
        assert_eq!(
            &p % 4u32,
            BigUint::from(3u32),
            "certificate square-root adapter requires p=3 mod 4"
        );
        let mut k = Self {
            p,
            d,
            eta: BigUint::zero(),
        };
        assert!(!k.is_square(&k.d));
        for i in 0..128u32 {
            let e = BigUint::from(i);
            if !k.is_square(&k.subp(&k.mulp(&e, &e), &k.d)) {
                k.eta = e;
                return k;
            }
        }
        panic!("no nonsquare tower parameter in frozen search bound")
    }
    pub fn addp(&self, x: &BigUint, y: &BigUint) -> BigUint {
        (x + y) % &self.p
    }
    pub fn subp(&self, x: &BigUint, y: &BigUint) -> BigUint {
        (x + &self.p - y) % &self.p
    }
    pub fn mulp(&self, x: &BigUint, y: &BigUint) -> BigUint {
        x * y % &self.p
    }
    pub fn invp(&self, x: &BigUint) -> BigUint {
        assert!(!x.is_zero());
        x.modpow(&(&self.p - 2u32), &self.p)
    }
    pub fn divp(&self, x: &BigUint, y: &BigUint) -> BigUint {
        self.mulp(x, &self.invp(y))
    }
    pub fn is_square(&self, x: &BigUint) -> bool {
        x.is_zero() || x.modpow(&((&self.p - 1u32) / 2u32), &self.p).is_one()
    }
    pub fn sqrtp(&self, x: &BigUint) -> Option<BigUint> {
        let y = x.modpow(&((&self.p + 1u32) / 4u32), &self.p);
        if self.mulp(&y, &y) == *x {
            Some(y.clone().min(self.subp(&BigUint::zero(), &y)))
        } else {
            None
        }
    }
    pub fn z(&self) -> F {
        F(std::array::from_fn(|_| BigUint::zero()))
    }
    pub fn c(&self, n: u64) -> F {
        self.fp(&BigUint::from(n))
    }
    pub fn fp(&self, n: &BigUint) -> F {
        let mut x = self.z();
        x.0[0] = n % &self.p;
        x
    }
    pub fn w(&self) -> F {
        let mut x = self.z();
        x.0[0] = self.subp(&BigUint::zero(), &self.eta);
        x.0[2] = BigUint::one();
        x
    }
    pub fn v(&self) -> F {
        let mut x = self.z();
        x.0[1] = BigUint::one();
        x
    }
    pub fn add(&self, x: &F, y: &F) -> F {
        F(std::array::from_fn(|i| self.addp(&x.0[i], &y.0[i])))
    }
    pub fn neg(&self, x: &F) -> F {
        F(std::array::from_fn(|i| {
            self.subp(&BigUint::zero(), &x.0[i])
        }))
    }
    pub fn sub(&self, x: &F, y: &F) -> F {
        self.add(x, &self.neg(y))
    }
    pub fn mul(&self, x: &F, y: &F) -> F {
        let mut r: [BigUint; 7] = std::array::from_fn(|_| BigUint::zero());
        for i in 0..4 {
            for j in 0..4 {
                r[i + j] += &x.0[i] * &y.0[j];
            }
        }
        let c0 = self.subp(&self.d, &self.mulp(&self.eta, &self.eta));
        for i in (4..=6).rev() {
            let a = &r[i] % &self.p;
            r[i - 2] += &a * &self.eta * 2u32;
            r[i - 4] += &a * &c0;
        }
        F(std::array::from_fn(|i| &r[i] % &self.p))
    }
    pub fn pow(&self, x: &F, n: &BigUint) -> F {
        let mut r = self.c(1);
        let mut b = x.clone();
        let mut n = n.clone();
        while !n.is_zero() {
            if n.bit(0) {
                r = self.mul(&r, &b);
            }
            b = self.mul(&b, &b);
            n >>= 1;
        }
        r
    }
    fn qa(&self, x: &(BigUint, BigUint), y: &(BigUint, BigUint)) -> (BigUint, BigUint) {
        (self.addp(&x.0, &y.0), self.addp(&x.1, &y.1))
    }
    fn qn(&self, x: &(BigUint, BigUint)) -> (BigUint, BigUint) {
        (
            self.subp(&BigUint::zero(), &x.0),
            self.subp(&BigUint::zero(), &x.1),
        )
    }
    fn qm(&self, x: &(BigUint, BigUint), y: &(BigUint, BigUint)) -> (BigUint, BigUint) {
        (
            self.addp(
                &self.mulp(&x.0, &y.0),
                &self.mulp(&self.d, &self.mulp(&x.1, &y.1)),
            ),
            self.addp(&self.mulp(&x.0, &y.1), &self.mulp(&x.1, &y.0)),
        )
    }
    fn qi(&self, x: &(BigUint, BigUint)) -> (BigUint, BigUint) {
        let n = self.invp(&self.subp(
            &self.mulp(&x.0, &x.0),
            &self.mulp(&self.d, &self.mulp(&x.1, &x.1)),
        ));
        (
            self.mulp(&x.0, &n),
            self.subp(&BigUint::zero(), &self.mulp(&x.1, &n)),
        )
    }
    fn qt(&self, x: &F) -> ((BigUint, BigUint), (BigUint, BigUint)) {
        (
            (
                self.addp(&x.0[0], &self.mulp(&self.eta, &x.0[2])),
                x.0[2].clone(),
            ),
            (
                self.addp(&x.0[1], &self.mulp(&self.eta, &x.0[3])),
                x.0[3].clone(),
            ),
        )
    }
    fn qf(&self, a: &(BigUint, BigUint), b: &(BigUint, BigUint)) -> F {
        F([
            self.subp(&a.0, &self.mulp(&self.eta, &a.1)),
            self.subp(&b.0, &self.mulp(&self.eta, &b.1)),
            a.1.clone(),
            b.1.clone(),
        ])
    }
    pub fn inv(&self, x: &F) -> F {
        assert_ne!(*x, self.z());
        let (a, b) = self.qt(x);
        let den = self.qa(
            &self.qm(&a, &a),
            &self.qn(&self.qm(&(self.eta.clone(), BigUint::one()), &self.qm(&b, &b))),
        );
        let di = self.qi(&den);
        self.qf(&self.qm(&a, &di), &self.qn(&self.qm(&b, &di)))
    }
    pub fn div(&self, x: &F, y: &F) -> F {
        self.mul(x, &self.inv(y))
    }
    pub fn sqrtq(&self, x: &F) -> Option<F> {
        let ((a, b), bb) = self.qt(x);
        assert!(bb.0.is_zero() && bb.1.is_zero());
        if b.is_zero() {
            if let Some(s) = self.sqrtp(&a) {
                return Some(self.fp(&s));
            }
            return self
                .sqrtp(&self.divp(&a, &self.d))
                .map(|s| self.mul(&self.fp(&s), &self.w()));
        }
        let n =
            self.sqrtp(&self.subp(&self.mulp(&a, &a), &self.mulp(&self.d, &self.mulp(&b, &b))))?;
        for n in [n.clone(), self.subp(&BigUint::zero(), &n)] {
            let s2 = self.divp(&self.addp(&a, &n), &BigUint::from(2u32));
            if let Some(s) = self.sqrtp(&s2) {
                if !s.is_zero() {
                    let t = self.divp(&b, &self.mulp(&BigUint::from(2u32), &s));
                    let z = self.qf(&(s, t), &(BigUint::zero(), BigUint::zero()));
                    if self.mul(&z, &z) == *x {
                        return Some(z);
                    }
                }
            }
        }
        None
    }
    pub fn rhs(&self, x: &F, a: &BigUint, b: &BigUint) -> F {
        self.add(
            &self.add(&self.mul(&self.mul(x, x), x), &self.mul(&self.fp(a), x)),
            &self.fp(b),
        )
    }
    pub fn on(&self, p: &Point, a: &BigUint, b: &BigUint) -> bool {
        p.as_ref()
            .is_none_or(|(x, y)| self.mul(y, y) == self.rhs(x, a, b))
    }
    pub fn pn(&self, p: &Point) -> Point {
        p.as_ref().map(|(x, y)| (x.clone(), self.neg(y)))
    }
    pub fn pa(&self, p: &Point, q: &Point, a: &BigUint) -> Point {
        let Some((x, y)) = p else { return q.clone() };
        let Some((u, v)) = q else { return p.clone() };
        if x == u && self.add(y, v) == self.z() {
            return None;
        }
        let slope = if x == u {
            self.div(
                &self.add(&self.mul(&self.c(3), &self.mul(x, x)), &self.fp(a)),
                &self.mul(&self.c(2), y),
            )
        } else {
            self.div(&self.sub(v, y), &self.sub(u, x))
        };
        let xx = self.sub(&self.sub(&self.mul(&slope, &slope), x), u);
        let yy = self.sub(&self.mul(&slope, &self.sub(x, &xx)), y);
        Some((xx, yy))
    }
    pub fn pm(&self, p: &Point, mut n: u64, a: &BigUint) -> Point {
        let mut b = p.clone();
        let mut r = None;
        while n > 0 {
            if n & 1 == 1 {
                r = self.pa(&r, &b, a);
            }
            n >>= 1;
            if n > 0 {
                b = self.pa(&b, &b, a);
            }
        }
        r
    }
    pub fn frob(&self, p: &Point) -> Point {
        p.as_ref()
            .map(|(x, y)| (self.pow(x, &self.p), self.pow(y, &self.p)))
    }
    pub fn weil3(&self, p: &Point, q: &Point, a: &BigUint) -> F {
        if p.is_none() || q.is_none() || p == q || p == &self.pn(q) {
            return self.c(1);
        }
        let line = |p: &Point, q: &Point| {
            let (x, y) = p.as_ref().unwrap();
            let (u, v) = q.as_ref().unwrap();
            let m = self.div(
                &self.add(&self.mul(&self.c(3), &self.mul(x, x)), &self.fp(a)),
                &self.mul(&self.c(2), y),
            );
            self.sub(&self.sub(v, y), &self.mul(&m, &self.sub(u, x)))
        };
        self.neg(&self.div(&line(p, q), &line(q, p)))
    }
    pub fn strings(&self, x: &F) -> Vec<String> {
        x.0.iter().map(ToString::to_string).collect()
    }
    pub fn point_json(&self, p: &Point) -> serde_json::Value {
        match p {
            None => serde_json::Value::Null,
            Some((x, y)) => serde_json::json!([self.strings(x), self.strings(y)]),
        }
    }
    pub fn trim(&self, mut f: BPoly) -> BPoly {
        while f.last().is_some_and(Zero::is_zero) {
            f.pop();
        }
        f
    }
    pub fn psub(&self, a: &[BigUint], b: &[BigUint]) -> BPoly {
        self.trim(
            (0..a.len().max(b.len()))
                .map(|i| {
                    self.subp(
                        a.get(i).unwrap_or(&BigUint::zero()),
                        b.get(i).unwrap_or(&BigUint::zero()),
                    )
                })
                .collect(),
        )
    }
    pub fn pmul(&self, a: &[BigUint], b: &[BigUint]) -> BPoly {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let mut r = vec![BigUint::zero(); a.len() + b.len() - 1];
        for (i, x) in a.iter().enumerate() {
            for (j, y) in b.iter().enumerate() {
                r[i + j] = self.addp(&r[i + j], &self.mulp(x, y));
            }
        }
        self.trim(r)
    }
    pub fn pdiv(&self, a: &[BigUint], b: &[BigUint]) -> (BPoly, BPoly) {
        assert!(!b.is_empty());
        let mut r = self.trim(a.to_vec());
        let mut q = vec![BigUint::zero(); a.len().saturating_sub(b.len()) + 1];
        let bi = self.invp(b.last().unwrap());
        while r.len() >= b.len() {
            let i = r.len() - b.len();
            let c = self.mulp(r.last().unwrap(), &bi);
            q[i] = c.clone();
            for j in 0..b.len() {
                r[i + j] = self.subp(&r[i + j], &self.mulp(&c, &b[j]));
            }
            r = self.trim(r);
        }
        (self.trim(q), r)
    }
    pub fn pmonic(&self, f: &[BigUint]) -> BPoly {
        let i = self.invp(f.last().unwrap());
        f.iter().map(|c| self.mulp(c, &i)).collect()
    }
    pub fn pgcd(&self, a: &[BigUint], b: &[BigUint]) -> BPoly {
        let (mut a, mut b) = (a.to_vec(), b.to_vec());
        while !b.is_empty() {
            let r = self.pdiv(&a, &b).1;
            a = b;
            b = r;
        }
        self.pmonic(&a)
    }
    pub fn ppow(&self, a: &[BigUint], n: &BigUint, m: &[BigUint]) -> BPoly {
        let mut r = vec![BigUint::one()];
        let mut b = self.pdiv(a, m).1;
        let mut n = n.clone();
        while !n.is_zero() {
            if n.bit(0) {
                r = self.pdiv(&self.pmul(&r, &b), m).1;
            }
            b = self.pdiv(&self.pmul(&b, &b), m).1;
            n >>= 1;
        }
        r
    }
    pub fn division_quadratics(&self, a: &BigUint, b: &BigUint) -> (BPoly, BPoly, u32) {
        let f = vec![
            self.subp(
                &BigUint::zero(),
                &self.divp(&self.mulp(a, a), &BigUint::from(3u32)),
            ),
            self.mulp(&BigUint::from(4u32), b),
            self.mulp(&BigUint::from(2u32), a),
            BigUint::zero(),
            BigUint::one(),
        ];
        let exp = (&self.p * &self.p - 1u32) / 2u32;
        for c in 0..128u32 {
            let g = self.pgcd(
                &f,
                &self.psub(
                    &self.ppow(&[BigUint::from(c), BigUint::one()], &exp, &f),
                    &[BigUint::one()],
                ),
            );
            if g.len() == 3 {
                let (h, rem) = self.pdiv(&f, &g);
                assert!(rem.is_empty());
                assert_eq!(self.pmul(&g, &h), f);
                return (g, h, c + 1);
            }
        }
        panic!("quartic equal-degree factorization exhausted 128 trials")
    }
    pub fn quadratic_roots(&self, f: &[BigUint]) -> [F; 2] {
        assert_eq!(f.len(), 3);
        let delta = self.subp(
            &self.mulp(&f[1], &f[1]),
            &self.mulp(&BigUint::from(4u32), &self.mulp(&f[0], &f[2])),
        );
        assert!(!self.is_square(&delta));
        let s = self
            .sqrtp(&self.divp(&delta, &self.d))
            .expect("nonsquare ratio has square root");
        let den = self.mulp(&BigUint::from(2u32), &f[2]);
        let x = self.fp(&self.divp(&self.subp(&BigUint::zero(), &f[1]), &den));
        let y = self.mul(&self.fp(&self.divp(&s, &den)), &self.w());
        [self.add(&x, &y), self.sub(&x, &y)]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tower_field_roundtrip_and_inverses() {
        let k = Field::new(7u32.into(), 3u32.into());
        assert_eq!(k.mul(&k.w(), &k.w()), k.c(3));
        assert_eq!(k.mul(&k.v(), &k.v()), k.add(&k.fp(&k.eta), &k.w()));
        for i in 0..128u64 {
            let x = F(std::array::from_fn(|j| BigUint::from((i >> (j * 2)) % 7)));
            if x != k.z() {
                assert_eq!(k.mul(&x, &k.inv(&x)), k.c(1));
                assert_eq!(k.pow(&x, &BigUint::from(2401u32)), x);
            }
        }
    }
    #[test]
    fn quadratic_squares_and_polynomial_euclid() {
        let k = Field::new(7u32.into(), 3u32.into());
        for a in 0..7 {
            for b in 0..7 {
                let x = k.add(&k.c(a), &k.mul(&k.c(b), &k.w()));
                let y = k.mul(&x, &x);
                let z = k.sqrtq(&y).unwrap();
                assert_eq!(k.mul(&z, &z), y);
            }
        }
        let a = vec![1u32.into(), 2u32.into(), 1u32.into()];
        let b = vec![1u32.into(), 1u32.into()];
        assert_eq!(k.pdiv(&a, &b), (b.clone(), vec![]));
        assert_eq!(k.pgcd(&a, &b), b);
    }
}
