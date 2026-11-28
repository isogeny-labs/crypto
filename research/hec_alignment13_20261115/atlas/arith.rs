use hec_native_ic::{fp::Fp, poly::Poly};
use num_bigint::BigUint;
pub type E = [u64; 13];
pub const P: u64 = 127;
pub const Z: E = [0; 13];
pub const O: E = {
    let mut a = Z;
    a[0] = 1;
    a
};
pub fn base(a: u64) -> E {
    let mut b = Z;
    b[0] = a % P;
    b
}
#[derive(Clone, Copy)]
pub struct Field {
    pub c: u64,
    pub q: u128,
}
impl Field {
    pub fn add(&self, a: E, b: E) -> E {
        std::array::from_fn(|i| (a[i] + b[i]) % P)
    }
    pub fn sub(&self, a: E, b: E) -> E {
        std::array::from_fn(|i| (a[i] + P - b[i]) % P)
    }
    pub fn neg(&self, a: E) -> E {
        self.sub(Z, a)
    }
    pub fn mul(&self, a: E, b: E) -> E {
        let mut w = [0u64; 25];
        for i in 0..13 {
            for j in 0..13 {
                w[i + j] += a[i] * b[j];
            }
        }
        for k in (13..25).rev() {
            let t = w[k] % P;
            w[k - 13] = (w[k - 13] + P - self.c * t % P) % P;
            w[k - 12] = (w[k - 12] + P - t) % P;
        }
        std::array::from_fn(|i| w[i] % P)
    }
    pub fn pow(&self, mut a: E, mut n: u128) -> E {
        let mut b = O;
        while n != 0 {
            if n & 1 != 0 {
                b = self.mul(b, a)
            }
            n >>= 1;
            if n != 0 {
                a = self.mul(a, a)
            }
        }
        b
    }
    pub fn modulus(&self) -> Poly {
        let mut c = [0; 14];
        c[0] = self.c;
        c[1] = 1;
        c[13] = 1;
        Poly::from_coeffs(&c)
    }
    pub fn inv(&self, a: E) -> E {
        assert_ne!(a, Z);
        if a == O {
            return O;
        }
        let fp = Fp::new(P);
        let (g, s, _) = Poly::xgcd(&Poly::from_coeffs(&a), &self.modulus(), &fp);
        assert_eq!(g, Poly::one());
        let s = s.rem(&self.modulus(), &fp);
        assert!(s.n <= 13);
        std::array::from_fn(|i| s.c[i])
    }
    pub fn frob(&self, a: E) -> E {
        self.pow(a, P as u128)
    }
    pub fn irreducible(&self) -> bool {
        let mut x = Z;
        x[1] = 1;
        let xp = self.frob(x);
        let fp = Fp::new(P);
        if Poly::gcd(&self.modulus(), &Poly::from_coeffs(&self.sub(xp, x)), &fp).deg() != 0 {
            return false;
        }
        let mut y = x;
        for _ in 0..13 {
            y = self.frob(y);
        }
        y == x
    }
    pub fn new() -> Self {
        for c in 1..P {
            let f = Self {
                c,
                q: (P as u128).pow(13),
            };
            if f.irreducible() {
                return f;
            }
        }
        panic!("field candidate cap")
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pol(pub Vec<E>);
impl Pol {
    pub fn new(mut c: Vec<E>) -> Self {
        while c.last() == Some(&Z) {
            c.pop();
        }
        Self(c)
    }
    pub fn zero() -> Self {
        Self(Vec::new())
    }
    pub fn one() -> Self {
        Self(vec![O])
    }
    pub fn embed(a: &Poly) -> Self {
        Self::new(a.c[..a.n].iter().map(|&x| base(x)).collect())
    }
    pub fn degree(&self) -> isize {
        self.0.len() as isize - 1
    }
    pub fn coeff(&self, i: usize) -> E {
        self.0.get(i).copied().unwrap_or(Z)
    }
    pub fn add(&self, b: &Self, f: &Field) -> Self {
        Self::new(
            (0..self.0.len().max(b.0.len()))
                .map(|i| f.add(self.coeff(i), b.coeff(i)))
                .collect(),
        )
    }
    pub fn sub(&self, b: &Self, f: &Field) -> Self {
        Self::new(
            (0..self.0.len().max(b.0.len()))
                .map(|i| f.sub(self.coeff(i), b.coeff(i)))
                .collect(),
        )
    }
    pub fn neg(&self, f: &Field) -> Self {
        Self::new(self.0.iter().map(|&x| f.neg(x)).collect())
    }
    pub fn scale(&self, a: E, f: &Field) -> Self {
        Self::new(self.0.iter().map(|&x| f.mul(x, a)).collect())
    }
    pub fn mul(&self, b: &Self, f: &Field) -> Self {
        if self.0.is_empty() || b.0.is_empty() {
            return Self::zero();
        }
        let mut c = vec![Z; self.0.len() + b.0.len() - 1];
        for (i, &a) in self.0.iter().enumerate() {
            for (j, &b) in b.0.iter().enumerate() {
                c[i + j] = f.add(c[i + j], f.mul(a, b));
            }
        }
        Self::new(c)
    }
    pub fn divrem(&self, b: &Self, f: &Field) -> (Self, Self) {
        assert!(!b.0.is_empty());
        if self.degree() < b.degree() {
            return (Self::zero(), self.clone());
        }
        let mut r = self.clone();
        let mut q = vec![Z; (self.degree() - b.degree() + 1) as usize];
        let inv = f.inv(*b.0.last().unwrap());
        while !r.0.is_empty() && r.degree() >= b.degree() {
            let k = (r.degree() - b.degree()) as usize;
            let c = f.mul(*r.0.last().unwrap(), inv);
            q[k] = c;
            for (i, &x) in b.0.iter().enumerate() {
                r.0[i + k] = f.sub(r.0[i + k], f.mul(c, x));
            }
            r = Self::new(r.0);
        }
        (Self::new(q), r)
    }
    pub fn rem(&self, b: &Self, f: &Field) -> Self {
        self.divrem(b, f).1
    }
    pub fn exact(&self, b: &Self, f: &Field) -> Self {
        let (q, r) = self.divrem(b, f);
        assert!(r.0.is_empty());
        q
    }
    pub fn monic(&self, f: &Field) -> Self {
        assert!(!self.0.is_empty());
        self.scale(f.inv(*self.0.last().unwrap()), f)
    }
    pub fn xgcd(a: &Self, b: &Self, f: &Field) -> (Self, Self, Self) {
        let (mut r0, mut r1) = (a.clone(), b.clone());
        let (mut s0, mut s1) = (Self::one(), Self::zero());
        let (mut t0, mut t1) = (Self::zero(), Self::one());
        while !r1.0.is_empty() {
            let (q, r) = r0.divrem(&r1, f);
            r0 = r1;
            r1 = r;
            let s = s0.sub(&q.mul(&s1, f), f);
            s0 = s1;
            s1 = s;
            let t = t0.sub(&q.mul(&t1, f), f);
            t0 = t1;
            t1 = t;
        }
        if r0.0.is_empty() {
            return (Self::zero(), Self::zero(), Self::zero());
        }
        let inv = f.inv(*r0.0.last().unwrap());
        (r0.scale(inv, f), s0.scale(inv, f), t0.scale(inv, f))
    }
    pub fn eval(&self, x: E, f: &Field) -> E {
        self.0.iter().rev().fold(Z, |s, &a| f.add(f.mul(s, x), a))
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Div {
    pub u: Pol,
    pub v: Pol,
}
pub struct Curve {
    pub field: Field,
    pub f: Pol,
}
impl Curve {
    pub fn zero(&self) -> Div {
        Div {
            u: Pol::one(),
            v: Pol::zero(),
        }
    }
    pub fn is_zero(&self, d: &Div) -> bool {
        d.u == Pol::one()
    }
    pub fn valid(&self, d: &Div) -> bool {
        d.u.0.last() == Some(&O)
            && d.u.degree() <= 3
            && d.v.degree() < d.u.degree()
            && self
                .f
                .sub(&d.v.mul(&d.v, &self.field), &self.field)
                .rem(&d.u, &self.field)
                .0
                .is_empty()
    }
    pub fn neg(&self, d: &Div) -> Div {
        Div {
            u: d.u.clone(),
            v: d.v.neg(&self.field),
        }
    }
    pub fn reduce(&self, u: Pol, v: Pol) -> Div {
        let f = &self.field;
        let mut u = u.monic(f);
        let mut v = v.rem(&u, f);
        while u.degree() > 3 {
            let u2 = self.f.sub(&v.mul(&v, f), f).exact(&u, f).monic(f);
            v = v.neg(f).rem(&u2, f);
            u = u2;
        }
        let d = Div { u, v };
        assert!(self.valid(&d));
        d
    }
    pub fn add(&self, a: &Div, b: &Div) -> Div {
        if self.is_zero(a) {
            return b.clone();
        }
        if self.is_zero(b) {
            return a.clone();
        }
        let f = &self.field;
        let (d1, e1, e2) = Pol::xgcd(&a.u, &b.u, f);
        let vs = a.v.add(&b.v, f);
        let (d, c1, c2) = Pol::xgcd(&d1, &vs, f);
        let s1 = c1.mul(&e1, f);
        let s2 = c1.mul(&e2, f);
        let u = a.u.mul(&b.u, f).exact(&d.mul(&d, f), f);
        let t = s1
            .mul(&a.u, f)
            .mul(&b.v, f)
            .add(&s2.mul(&b.u, f).mul(&a.v, f), f)
            .add(&c2.mul(&a.v.mul(&b.v, f).add(&self.f, f), f), f);
        let v = t.exact(&d, f);
        self.reduce(u.clone(), v.rem(&u, f))
    }
    pub fn small(&self, a: &Div, mut k: u64) -> Div {
        let mut r = self.zero();
        let mut b = a.clone();
        while k != 0 {
            if k & 1 != 0 {
                r = self.add(&r, &b)
            }
            k >>= 1;
            if k != 0 {
                b = self.add(&b, &b)
            }
        }
        r
    }
    pub fn scalar(&self, a: &Div, k: &BigUint) -> Div {
        let mut r = self.zero();
        for i in (0..k.bits()).rev() {
            r = self.add(&r, &r);
            if k.bit(i) {
                r = self.add(&r, a)
            }
        }
        r
    }
    pub fn frob(&self, a: &Div) -> Div {
        let d = Div {
            u: Pol::new(a.u.0.iter().map(|&x| self.field.frob(x)).collect()),
            v: Pol::new(a.v.0.iter().map(|&x| self.field.frob(x)).collect()),
        };
        assert!(self.valid(&d));
        d
    }
    pub fn operator(&self, a: &Div, cs: &[u64]) -> Div {
        let mut r = self.zero();
        let mut b = a.clone();
        for (i, &c) in cs.iter().enumerate() {
            r = self.add(&r, &self.small(&b, c));
            if i + 1 < cs.len() {
                b = self.frob(&b);
            }
        }
        r
    }
}
