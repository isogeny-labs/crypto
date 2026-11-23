//! Exact polynomial-basis arithmetic for the frozen public toy fixture.
//! Variable time; this is a correctness preflight, not a production field.
use std::ops::{Add, Div, Mul, Neg, Sub};

pub const MODULUS: [u8; 19] = [3, 2, 6, 0, 0, 3, 1, 5, 6, 1, 6, 2, 1, 0, 0, 0, 0, 0, 1];
pub const ORDER: u64 = 7_u64.pow(18);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct F(pub [u8; 18]);

impl F {
    pub const ZERO: Self = Self([0; 18]);
    pub fn c(n: u8) -> Self {
        let mut a = [0; 18];
        a[0] = n % 7;
        Self(a)
    }
    pub fn generator() -> Self {
        let mut a = [0; 18];
        a[1] = 1;
        Self(a)
    }
    pub fn pow(self, mut n: u64) -> Self {
        let (mut a, mut out) = (self, Self::c(1));
        while n > 0 {
            if n & 1 == 1 {
                out = out * a;
            }
            n >>= 1;
            if n > 0 {
                a = a * a;
            }
        }
        out
    }
    pub fn inv(self) -> Self {
        assert_ne!(self, Self::ZERO, "field division by zero");
        let v = self.pow(ORDER - 2);
        assert_eq!(self * v, Self::c(1), "noninvertible field element");
        v
    }
    pub fn sqrt(self) -> Option<Self> {
        if self == Self::ZERO {
            return Some(self);
        }
        if self.pow((ORDER - 1) / 2) != Self::c(1) {
            return None;
        }
        let mut q = ORDER - 1;
        let mut m = 0;
        while q.is_multiple_of(2) {
            q /= 2;
            m += 1;
        }
        let mut z = Self::generator();
        // Deterministic bounded search, beyond the prime subfield.
        for n in 0..1024_u64 {
            let mut a = [0; 18];
            a[1] = 1;
            let mut k = n;
            for c in &mut a {
                *c = (*c + (k % 7) as u8) % 7;
                k /= 7;
            }
            z = Self(a);
            if z.pow((ORDER - 1) / 2) == -Self::c(1) {
                break;
            }
        }
        assert_eq!(
            z.pow((ORDER - 1) / 2),
            -Self::c(1),
            "nonresidue search exhausted"
        );
        let (mut c, mut x, mut t) = (z.pow(q), self.pow(q.div_ceil(2)), self.pow(q));
        while t != Self::c(1) {
            let (mut i, mut v) = (0, t);
            while v != Self::c(1) && i < m {
                v = v * v;
                i += 1;
            }
            assert!(i < m, "Tonelli-Shanks failed");
            let b = c.pow(1 << (m - i - 1));
            x = x * b;
            c = b * b;
            t = t * c;
            m = i;
        }
        assert_eq!(x * x, self);
        Some(x.min(-x))
    }
    pub fn frob(self, n: usize) -> Self {
        self.pow(7_u64.pow(n as u32))
    }
}

impl Add for F {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        Self(std::array::from_fn(|i| (self.0[i] + b.0[i]) % 7))
    }
}
impl Neg for F {
    type Output = Self;
    fn neg(self) -> Self {
        Self(self.0.map(|x| (7 - x) % 7))
    }
}
impl Sub for F {
    type Output = Self;
    fn sub(self, b: Self) -> Self {
        self + -b
    }
}
impl Mul for F {
    type Output = Self;
    fn mul(self, b: Self) -> Self {
        if b.0[1..].iter().all(|x| *x == 0) {
            return Self(self.0.map(|x| x * b.0[0] % 7));
        }
        if self.0[1..].iter().all(|x| *x == 0) {
            return Self(b.0.map(|x| x * self.0[0] % 7));
        }
        let mut t = [0_i32; 35];
        for i in 0..18 {
            for j in 0..18 {
                t[i + j] += i32::from(self.0[i]) * i32::from(b.0[j]);
            }
        }
        for i in (18..35).rev() {
            let c = t[i].rem_euclid(7);
            for j in 0..18 {
                t[i - 18 + j] -= c * i32::from(MODULUS[j]);
            }
        }
        Self(std::array::from_fn(|i| t[i].rem_euclid(7) as u8))
    }
}
impl Div for F {
    type Output = Self;
    fn div(self, b: Self) -> Self {
        Mul::mul(self, b.inv())
    }
}

type Poly = Vec<F>;
fn trim(mut p: Poly) -> Poly {
    while p.last() == Some(&F::ZERO) {
        p.pop();
    }
    p
}
pub fn rem(mut a: Poly, b: &[F]) -> Poly {
    assert!(!b.is_empty());
    let scale = b.last().unwrap().inv();
    while a.len() >= b.len() {
        let j = a.len() - b.len();
        let c = *a.last().unwrap() * scale;
        for i in 0..b.len() {
            a[j + i] = a[j + i] - c * b[i];
        }
        a = trim(a);
    }
    a
}
fn mul_mod(a: &[F], b: &[F], modulus: &[F]) -> Poly {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut c = vec![F::ZERO; a.len() + b.len() - 1];
    for i in 0..a.len() {
        for j in 0..b.len() {
            c[i + j] = c[i + j] + a[i] * b[j];
        }
    }
    rem(c, modulus)
}
fn pow_mod(mut a: Poly, mut n: u64, modulus: &[F]) -> Poly {
    let mut out = vec![F::c(1)];
    while n > 0 {
        if n & 1 == 1 {
            out = mul_mod(&out, &a, modulus);
        }
        n >>= 1;
        if n > 0 {
            a = mul_mod(&a, &a, modulus);
        }
    }
    out
}
fn gcd(mut a: Poly, mut b: Poly) -> Poly {
    while !b.is_empty() {
        let c = rem(a, &b);
        a = b;
        b = c;
    }
    let scale = a.last().unwrap().inv();
    a.into_iter().map(|v| v * scale).collect()
}
fn subtract_x(mut a: Poly) -> Poly {
    a.resize(a.len().max(2), F::ZERO);
    a[1] = a[1] - F::c(1);
    trim(a)
}
pub fn verify_modulus() {
    // Rabin irreducibility over F7: the coefficient arithmetic here stays
    // in the prime subfield and does not assume the degree-18 quotient is a field.
    let p: Poly = MODULUS.into_iter().map(F::c).collect();
    let mut x = vec![F::ZERO, F::c(1)];
    for i in 1..=18 {
        x = pow_mod(x, 7, &p);
        if i == 6 || i == 9 {
            assert_eq!(gcd(p.clone(), subtract_x(x.clone())).len(), 1);
        }
    }
    assert!(subtract_x(x).is_empty(), "degree-18 modulus is reducible");
}
pub fn cubic_roots(a: [F; 4]) -> Vec<F> {
    fn split(p: Poly, out: &mut Vec<F>) {
        if p.len() == 2 {
            out.push(-p[0] / p[1]);
            return;
        }
        for counter in 0..1024_u64 {
            let mut coeffs = F::generator().0;
            let mut c = counter;
            for x in &mut coeffs {
                *x = (*x + (c % 7) as u8) % 7;
                c /= 7;
            }
            let mut v = pow_mod(vec![F(coeffs), F::c(1)], (ORDER - 1) / 2, &p);
            if v.is_empty() {
                v.push(F::ZERO);
            }
            v[0] = v[0] - F::c(1);
            let g = gcd(p.clone(), trim(v));
            if g.len() > 1 && g.len() < p.len() {
                let mut q = vec![F::ZERO; p.len() - g.len() + 1];
                let mut r = p;
                while r.len() >= g.len() {
                    let i = r.len() - g.len();
                    q[i] = *r.last().unwrap() / *g.last().unwrap();
                    for j in 0..g.len() {
                        r[i + j] = r[i + j] - q[i] * g[j];
                    }
                    r = trim(r);
                }
                assert!(r.is_empty());
                split(g, out);
                split(trim(q), out);
                return;
            }
        }
        panic!("bounded cubic splitting search exhausted");
    }
    let mut roots = vec![];
    split(a.to_vec(), &mut roots);
    roots.sort();
    roots.dedup();
    assert_eq!(roots.len(), 3);
    for x in &roots {
        assert_eq!(
            a[0] + a[1] * *x + a[2] * x.pow(2) + a[3] * x.pow(3),
            F::ZERO
        );
    }
    roots
}

pub type Point = Option<(F, F)>;
pub fn neg(p: Point) -> Point {
    p.map(|(x, y)| (x, -y))
}
pub fn add(a: F, p: Point, q: Point) -> Point {
    match (p, q) {
        (None, q) => q,
        (p, None) => p,
        (Some((x, y)), Some((u, v))) => {
            if x == u && y + v == F::ZERO {
                return None;
            }
            let m = if x == u {
                (F::c(3) * x * x + a) / (F::c(2) * y)
            } else {
                (v - y) / (u - x)
            };
            let r = m * m - x - u;
            Some((r, m * (x - r) - y))
        }
    }
}
