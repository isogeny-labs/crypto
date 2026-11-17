//! Shared exact arithmetic for round 04. No external CAS execution.
use serde_json::{json, Value};
use std::path::Path;
#[derive(Clone, Copy, Debug)]
pub struct Field {
    pub m: u32,
    pub modulus: u64,
}
impl Field {
    pub fn mul(self, mut a: u32, mut b: u32) -> u32 {
        let mut v = 0;
        while b > 0 {
            if b & 1 != 0 {
                v ^= a;
            }
            b >>= 1;
            a <<= 1;
            if a & (1 << self.m) != 0 {
                a ^= self.modulus as u32;
            }
        }
        v
    }
    pub fn pow(self, mut a: u32, mut n: u64) -> u32 {
        let mut v = 1;
        while n > 0 {
            if n & 1 != 0 {
                v = self.mul(v, a);
            }
            a = self.mul(a, a);
            n >>= 1;
        }
        v
    }
    pub fn inv(self, a: u32) -> u32 {
        assert_ne!(a, 0);
        self.pow(a, (1u64 << self.m) - 2)
    }
    pub fn sq(self, a: u32) -> u32 {
        self.mul(a, a)
    }
    pub fn frob(self, mut a: u32, n: u32) -> u32 {
        for _ in 0..n {
            a = self.sq(a);
        }
        a
    }
    pub fn eval(self, p: &[u32], x: u32) -> u32 {
        p.iter().rev().fold(0, |v, c| self.mul(v, x) ^ c)
    }
    pub fn trace(self, a: u32) -> u32 {
        let mut x = a;
        let mut s = 0;
        for _ in 0..self.m {
            s ^= x;
            x = self.sq(x);
        }
        assert!(s < 2);
        s
    }
    pub fn quadratic(self, c: u32) -> Option<u32> {
        let n = self.m as usize;
        let mut rows = vec![0u64; n];
        for j in 0..n {
            let v = self.sq(1 << j) ^ (1 << j);
            for (i, r) in rows.iter_mut().enumerate() {
                *r |= (((v >> i) & 1) as u64) << j;
            }
        }
        for (i, r) in rows.iter_mut().enumerate() {
            *r |= (((c >> i) & 1) as u64) << n;
        }
        let mut piv = vec![];
        let mut k = 0;
        for j in 0..n {
            if let Some(i) = (k..n).find(|i| rows[*i] & (1 << j) != 0) {
                rows.swap(k, i);
                for i in 0..n {
                    if i != k && rows[i] & (1 << j) != 0 {
                        rows[i] ^= rows[k];
                    }
                }
                piv.push(j);
                k += 1;
            }
        }
        if rows[k..].iter().any(|r| r & (1 << n) != 0) {
            return None;
        }
        let mut z = 0;
        for (i, j) in piv.iter().enumerate() {
            z |= (((rows[i] >> n) & 1) as u32) << j;
        }
        assert_eq!(self.sq(z) ^ z, c);
        Some(z)
    }
    pub fn find(m: u32) -> Self {
        for low in (1..4096u64).step_by(2) {
            let f = Self {
                m,
                modulus: (1 << m) | low,
            };
            if f.irreducible() {
                return f;
            }
        }
        panic!("Rabin modulus budget exhausted")
    }
    pub fn irreducible(self) -> bool {
        if self.m == 1 {
            return self.modulus == 3;
        }
        fn rem(mut a: u64, b: u64) -> u64 {
            while a != 0 && a.leading_zeros() <= b.leading_zeros() {
                a ^= b << (b.leading_zeros() - a.leading_zeros());
            }
            a
        }
        fn gcd(mut a: u64, mut b: u64) -> u64 {
            while b != 0 {
                let r = rem(a, b);
                a = b;
                b = r;
            }
            a
        }
        let mut x = 2;
        for i in 1..=self.m {
            x = self.sq(x);
            if i <= self.m / 2 && self.m % i == 0 && gcd((x ^ 2) as u64, self.modulus) != 1 {
                return false;
            }
        }
        x == 2
    }
    pub fn embed(self, small: Field) -> Vec<u32> {
        assert_eq!(self.m % small.m, 0);
        let n = (1u64 << self.m) - 1;
        let d = (1u64 << small.m) - 1;
        let mut gen = 0;
        for a in 2..4096 {
            let g = self.pow(a, n / d);
            if self.pow(g, d) == 1 && (1..=d).find(|i| self.pow(g, *i) == 1) == Some(d) {
                gen = g;
                break;
            }
        }
        assert_ne!(gen, 0);
        let coeff: Vec<_> = (0..=small.m)
            .map(|i| ((small.modulus >> i) & 1) as u32)
            .collect();
        let root = (0..d)
            .map(|i| self.pow(gen, i))
            .find(|r| self.eval(&coeff, *r) == 0)
            .unwrap();
        (0..1u32 << small.m)
            .map(|a| {
                let mut v = 0;
                for i in 0..small.m {
                    if a & (1 << i) != 0 {
                        v ^= self.pow(root, i as u64);
                    }
                }
                v
            })
            .collect()
    }
}
pub type Poly = Vec<u32>;
pub fn trim(mut p: Poly) -> Poly {
    while p.last() == Some(&0) {
        p.pop();
    }
    p
}
pub fn add(a: &[u32], b: &[u32]) -> Poly {
    trim(
        (0..a.len().max(b.len()))
            .map(|i| a.get(i).copied().unwrap_or(0) ^ b.get(i).copied().unwrap_or(0))
            .collect(),
    )
}
pub fn scale(f: Field, a: &[u32], c: u32) -> Poly {
    trim(a.iter().map(|v| f.mul(*v, c)).collect())
}
pub fn mul(f: Field, a: &[u32], b: &[u32]) -> Poly {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut p = vec![0; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            p[i + j] ^= f.mul(*x, *y);
        }
    }
    trim(p)
}
pub fn div(f: Field, a: &[u32], b: &[u32]) -> (Poly, Poly) {
    assert!(!b.is_empty());
    let mut r = trim(a.to_vec());
    let mut q = vec![0; r.len().saturating_sub(b.len()) + 1];
    let inv = f.inv(*b.last().unwrap());
    while !r.is_empty() && r.len() >= b.len() {
        let d = r.len() - b.len();
        let c = f.mul(*r.last().unwrap(), inv);
        q[d] ^= c;
        for (i, v) in b.iter().enumerate() {
            r[d + i] ^= f.mul(c, *v);
        }
        r = trim(r);
    }
    (trim(q), r)
}
pub fn monic(f: Field, a: &[u32]) -> Poly {
    if a.is_empty() {
        vec![]
    } else {
        scale(f, a, f.inv(*a.last().unwrap()))
    }
}
pub fn gcd(f: Field, a: &[u32], b: &[u32]) -> Poly {
    let (mut a, mut b) = (a.to_vec(), b.to_vec());
    while !b.is_empty() {
        let r = div(f, &a, &b).1;
        a = b;
        b = r;
    }
    monic(f, &a)
}
pub fn invmod(f: Field, a: &[u32], m: &[u32]) -> Poly {
    let (mut r0, mut r1) = (m.to_vec(), a.to_vec());
    let (mut s0, mut s1) = (vec![], vec![1]);
    while !r1.is_empty() {
        let (q, r) = div(f, &r0, &r1);
        let s = add(&s0, &mul(f, &q, &s1));
        r0 = r1;
        r1 = r;
        s0 = s1;
        s1 = s;
    }
    assert_eq!(r0.len(), 1);
    div(f, &scale(f, &s0, f.inv(r0[0])), m).1
}
pub fn modmul(f: Field, a: &[u32], b: &[u32], m: &[u32]) -> Poly {
    div(f, &mul(f, a, b), m).1
}
pub fn composemod(f: Field, a: &[u32], x: &[u32], m: &[u32]) -> Poly {
    a.iter()
        .rev()
        .fold(vec![], |v, c| add(&modmul(f, &v, x, m), &[*c]))
}
pub fn derivative(a: &[u32]) -> Poly {
    trim(
        a.iter()
            .enumerate()
            .skip(1)
            .map(|(i, v)| if i % 2 == 1 { *v } else { 0 })
            .collect(),
    )
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pt {
    O,
    A(u32, u32),
}
impl Pt {
    pub fn json(self) -> Value {
        match self {
            Self::O => json!({"infinity":true}),
            Self::A(x, y) => json!([x, y]),
        }
    }
}
#[derive(Clone, Copy)]
pub struct Curve {
    pub f: Field,
    pub a: u32,
    pub b: u32,
}
impl Curve {
    pub fn neg(self, p: Pt) -> Pt {
        match p {
            Pt::O => p,
            Pt::A(x, y) => Pt::A(x, x ^ y),
        }
    }
    pub fn valid(self, p: Pt) -> bool {
        match p {
            Pt::O => true,
            Pt::A(x, y) => {
                self.f.sq(y) ^ self.f.mul(x, y)
                    == self.f.mul(self.f.sq(x), x) ^ self.f.mul(self.a, self.f.sq(x)) ^ self.b
            }
        }
    }
    pub fn point(self, x: u32) -> Option<Pt> {
        let f = self.f;
        if x == 0 {
            return Some(Pt::A(0, f.frob(self.b, f.m - 1)));
        }
        let c = x ^ self.a ^ f.mul(self.b, f.inv(f.sq(x)));
        f.quadratic(c).map(|z| Pt::A(x, f.mul(x, z)))
    }
    pub fn add(self, p: Pt, q: Pt) -> Pt {
        let f = self.f;
        let (Pt::A(x, y), Pt::A(u, v)) = (p, q) else {
            return if p == Pt::O { q } else { p };
        };
        if x == u && y != v {
            return Pt::O;
        }
        if x == u && x == 0 {
            return Pt::O;
        }
        let l = if x == u {
            x ^ f.mul(y, f.inv(x))
        } else {
            f.mul(y ^ v, f.inv(x ^ u))
        };
        let z = f.sq(l) ^ l ^ x ^ u ^ self.a;
        let w = f.mul(l, x ^ z) ^ z ^ y;
        let r = Pt::A(z, w);
        debug_assert!(self.valid(r));
        r
    }
    pub fn times(self, mut p: Pt, mut n: u64) -> Pt {
        let mut r = Pt::O;
        while n > 0 {
            if n & 1 != 0 {
                r = self.add(r, p);
            }
            p = self.add(p, p);
            n >>= 1;
        }
        r
    }
    pub fn frob(self, p: Pt, n: u32) -> Pt {
        match p {
            Pt::O => p,
            Pt::A(x, y) => Pt::A(self.f.frob(x, n), self.f.frob(y, n)),
        }
    }
    pub fn points(self) -> Vec<Pt> {
        let mut ps = vec![Pt::O];
        for x in 0..1u32 << self.f.m {
            if let Some(p) = self.point(x) {
                ps.push(p);
                let q = self.neg(p);
                if q != p {
                    ps.push(q);
                }
            }
        }
        ps
    }
}
pub fn order(t: i64, k: u32) -> u64 {
    let (mut s0, mut s1) = (2i128, t as i128);
    if k == 0 {
        return 1;
    }
    for _ in 2..=k {
        let v = t as i128 * s1 - 2 * s0;
        s0 = s1;
        s1 = v;
    }
    ((1i128 << k) + 1 - s1) as u64
}
pub fn read(p: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(p).unwrap()).unwrap()
}
