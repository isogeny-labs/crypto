//! Native arithmetic and complete solvers for the three-leads replication.
//! The standalone research driver and ecbench compile this same source.
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::cell::{Cell, OnceCell};
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Counts {
    pub m: u64,
    pub s: u64,
    pub i: u64,
    pub field_adds: u64,
    pub bit_ops: u64,
    pub ec_adds: u64,
    pub jac_adds: u64,
    pub sqrt_calls: u64,
}
impl std::ops::Sub for Counts {
    type Output = Self;
    fn sub(self, b: Self) -> Self {
        Self {
            m: self.m - b.m,
            s: self.s - b.s,
            i: self.i - b.i,
            field_adds: self.field_adds - b.field_adds,
            bit_ops: self.bit_ops - b.bit_ops,
            ec_adds: self.ec_adds - b.ec_adds,
            jac_adds: self.jac_adds - b.jac_adds,
            sqrt_calls: self.sqrt_calls - b.sqrt_calls,
        }
    }
}
impl std::ops::Add for Counts {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        Self {
            m: self.m + b.m,
            s: self.s + b.s,
            i: self.i + b.i,
            field_adds: self.field_adds + b.field_adds,
            bit_ops: self.bit_ops + b.bit_ops,
            ec_adds: self.ec_adds + b.ec_adds,
            jac_adds: self.jac_adds + b.jac_adds,
            sqrt_calls: self.sqrt_calls + b.sqrt_calls,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Field {
    pub p: u64,
    pub n: usize,
    /// Low coefficients of the monic modulus, packed in base p; for p=2
    /// this includes the leading bit, for compatibility with binary fixtures.
    pub modulus: u64,
    #[serde(skip)]
    counts: Rc<Cell<Counts>>,
    #[serde(skip)]
    as_basis: Rc<OnceCell<Vec<(u64, u64)>>>,
    /// Diagnostic censuses only. Cold measured instances always drop these.
    #[serde(skip)]
    tables: Option<Rc<(Vec<u64>, Vec<u64>)>>,
}
impl Field {
    pub fn new(p: u64, n: usize, modulus: u64) -> Self {
        assert!(n > 0 && n <= 5 || p == 2 && n <= 30);
        Self {
            p,
            n,
            modulus,
            counts: Default::default(),
            as_basis: Default::default(),
            tables: None,
        }
    }
    pub fn size(&self) -> u64 {
        self.p.pow(self.n as u32)
    }
    pub fn counts(&self) -> Counts {
        self.counts.get()
    }
    pub fn reset(&self) {
        self.counts.set(Counts::default());
    }
    pub fn fresh(&self) -> Self {
        Self::new(self.p, self.n, self.modulus)
    }
    pub fn diagnostic_tables(&mut self) {
        assert!(self.size() <= 1 << 20);
        assert!(self.tables.is_none());
        let n = self.size() - 1;
        let fs = factor(n);
        let g = (2..self.size())
            .find(|&g| fs.iter().all(|&(p, _)| self.pow(g, n / p) != 1))
            .expect("primitive diagnostic field element");
        let mut exp = vec![0; n as usize];
        let mut log = vec![u64::MAX; self.size() as usize];
        let mut x = 1;
        for j in 0..n {
            assert_eq!(log[x as usize], u64::MAX, "reducible diagnostic field");
            exp[j as usize] = x;
            log[x as usize] = j;
            x = self.mul(x, g);
        }
        assert_eq!(x, 1);
        let mut rng = Rng(20261018);
        for _ in 0..1000 {
            let a = rng.next() % self.size();
            let b = rng.next() % self.size();
            let expected = if a == 0 || b == 0 {
                0
            } else {
                exp[((log[a as usize] + log[b as usize]) % n) as usize]
            };
            assert_eq!(self.mul(a, b), expected);
        }
        self.tables = Some(Rc::new((exp, log)));
        self.reset();
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.tables.is_some() {
            return Err("diagnostic tables cannot enter measured instances".into());
        }
        if self.p == 2 {
            if self.n < 2
                || self.n > 30
                || self.modulus >> (self.n + 1) != 0
                || self.modulus & (1 << self.n) == 0
                || !binary_irreducible(self.modulus, self.n)
            {
                return Err("invalid binary extension modulus".into());
            }
        } else if self.p < 5 || !is_prime(self.p) || self.p >= 1 << 31 || self.n > 2 {
            return Err("study instances support odd prime and quadratic fields below 2^31".into());
        } else if self.n == 2
            && (self.modulus == 0
                || self.modulus >= self.p
                || modpow(self.nr(), (self.p - 1) / 2, self.p) != self.p - 1)
        {
            return Err("invalid quadratic binomial".into());
        }
        Ok(())
    }
    fn count(&self, action: impl FnOnce(&mut Counts)) {
        let mut c = self.counts.get();
        action(&mut c);
        self.counts.set(c);
    }
    pub fn constant(&self, a: u64) -> u64 {
        a % self.p
    }
    pub fn signed(&self, a: i64) -> u64 {
        a.rem_euclid(self.p as i64) as u64
    }
    pub fn add(&self, mut a: u64, mut b: u64) -> u64 {
        self.count(|c| c.field_adds += self.n as u64);
        if self.p == 2 {
            return a ^ b;
        }
        if self.n == 1 {
            return (a + b) % self.p;
        }
        let (mut r, mut k) = (0, 1);
        for _ in 0..self.n {
            r += ((a % self.p + b % self.p) % self.p) * k;
            a /= self.p;
            b /= self.p;
            k *= self.p;
        }
        r
    }
    pub fn neg(&self, mut a: u64) -> u64 {
        if self.p == 2 {
            return a;
        }
        self.count(|c| c.field_adds += self.n as u64);
        let (mut r, mut k) = (0, 1);
        for _ in 0..self.n {
            r += ((self.p - a % self.p) % self.p) * k;
            a /= self.p;
            k *= self.p;
        }
        r
    }
    pub fn sub(&self, a: u64, b: u64) -> u64 {
        self.add(a, self.neg(b))
    }
    fn bm(&self, a: u64, b: u64) -> u64 {
        self.count(|c| c.m += 1);
        (a * b) % self.p
    }
    fn bs(&self, a: u64) -> u64 {
        self.count(|c| c.s += 1);
        (a * a) % self.p
    }
    fn bi(&self, a: u64) -> u64 {
        assert_ne!(a, 0);
        self.count(|c| c.i += 1);
        modpow(a, self.p - 2, self.p)
    }
    fn nr(&self) -> u64 {
        (self.p - self.modulus % self.p) % self.p
    }
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        if let Some(t) = &self.tables {
            return if a == 0 || b == 0 {
                0
            } else {
                t.0[((t.1[a as usize] + t.1[b as usize]) % (self.size() - 1)) as usize]
            };
        }
        if self.p == 2 {
            self.count(|c| c.m += 1);
            return binary_mul(a, b, self.modulus, self.n);
        }
        if self.n == 1 {
            return self.bm(a, b);
        }
        if self.n == 2 {
            let (x, y, u, v) = (a % self.p, a / self.p, b % self.p, b / self.p);
            let z = self.bm(x, u);
            let w = self.bm(y, v);
            let cross = self.bm((x + y) % self.p, (u + v) % self.p);
            let nw = if self.nr() == self.p - 1 {
                (self.p - w) % self.p
            } else {
                self.bm(self.nr(), w)
            };
            self.count(|c| c.field_adds += 6);
            return (z + nw) % self.p + ((cross + 2 * self.p - z - w) % self.p) * self.p;
        }
        let mut aa = vec![0; self.n];
        let mut bb = aa.clone();
        let (mut x, mut y) = (a, b);
        for j in 0..self.n {
            aa[j] = x % self.p;
            bb[j] = y % self.p;
            x /= self.p;
            y /= self.p;
        }
        let mut c = vec![0; 2 * self.n - 1];
        for j in 0..self.n {
            for k in 0..self.n {
                c[j + k] = (c[j + k] + self.bm(aa[j], bb[k])) % self.p;
                self.count(|c| c.field_adds += 1);
            }
        }
        let mut tail = self.modulus;
        let mut co = vec![0; self.n];
        for v in &mut co {
            *v = tail % self.p;
            tail /= self.p;
        }
        for j in (self.n..c.len()).rev() {
            for k in 0..self.n {
                c[j - self.n + k] = (c[j - self.n + k] + self.p - self.bm(c[j], co[k])) % self.p;
                self.count(|c| c.field_adds += 1);
            }
        }
        c[..self.n].iter().rev().fold(0, |r, &x| r * self.p + x)
    }
    pub fn sq(&self, a: u64) -> u64 {
        if self.tables.is_some() {
            return self.mul(a, a);
        }
        if self.p == 2 {
            self.count(|c| c.s += 1);
            return binary_mul(a, a, self.modulus, self.n);
        }
        if self.n == 1 {
            return self.bs(a);
        }
        if self.n == 2 {
            let (x, y) = (a % self.p, a / self.p);
            let xx = self.bs(x);
            let yy = self.bs(y);
            let xy = self.bm(x, y);
            let n = if self.nr() == self.p - 1 {
                (self.p - yy) % self.p
            } else {
                self.bm(self.nr(), yy)
            };
            self.count(|c| c.field_adds += 2);
            return (xx + n) % self.p + (2 * xy % self.p) * self.p;
        }
        self.mul(a, a)
    }
    pub fn pow(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1;
        while e > 0 {
            if e & 1 != 0 {
                r = self.mul(r, a);
            }
            e >>= 1;
            if e > 0 {
                a = self.sq(a);
            }
        }
        r
    }
    pub fn inv(&self, a: u64) -> u64 {
        assert_ne!(a, 0);
        if let Some(t) = &self.tables {
            return t.0[((self.size() - 1 - t.1[a as usize]) % (self.size() - 1)) as usize];
        }
        if self.p == 2 {
            self.count(|c| c.i += 1);
            let mut x = a;
            let mut e = self.size() - 2;
            let mut r = 1;
            while e > 0 {
                if e & 1 != 0 {
                    r = binary_mul(r, x, self.modulus, self.n);
                }
                e >>= 1;
                if e > 0 {
                    x = binary_mul(x, x, self.modulus, self.n);
                }
            }
            return r;
        }
        if self.n == 1 {
            return self.bi(a);
        }
        if self.n == 2 {
            let (x, y) = (a % self.p, a / self.p);
            let xx = self.bs(x);
            let yy = self.bs(y);
            let ny = if self.nr() == self.p - 1 {
                (self.p - yy) % self.p
            } else {
                self.bm(self.nr(), yy)
            };
            let norm = (xx + self.p - ny) % self.p;
            self.count(|c| c.field_adds += 2);
            let inv = self.bi(norm);
            let u = self.bm(x, inv);
            let v = self.bm((self.p - y) % self.p, inv);
            return u + self.p * v;
        }
        if self.n == 3 && self.modulus < self.p {
            // For w^3 + d = 0, multiply by the two Frobenius conjugates.
            // This leaves one base-field norm inversion instead of expanding
            // an inversion into hundreds of extension-field multiplications.
            let (x, y, z) = (a % self.p, a / self.p % self.p, a / (self.p * self.p));
            let d = self.modulus;
            let u = (self.bs(x) + self.bm(d, self.bm(y, z))) % self.p;
            let v = (self.p - (self.bm(x, y) + self.bm(d, self.bs(z))) % self.p) % self.p;
            let w = (self.bs(y) + self.p - self.bm(x, z)) % self.p;
            let cross = (self.bm(z, v) + self.bm(y, w)) % self.p;
            let norm = (self.bm(x, u) + self.p - self.bm(d, cross)) % self.p;
            let inverse_norm = self.bi(norm);
            self.count(|c| c.field_adds += 6);
            return self.bm(u, inverse_norm)
                + self.p * self.bm(v, inverse_norm)
                + self.p * self.p * self.bm(w, inverse_norm);
        }
        // Expanded exponentiation: no extra inversion macro is charged.
        self.pow(a, self.size() - 2)
    }
    pub fn div(&self, a: u64, b: u64) -> u64 {
        self.mul(a, self.inv(b))
    }
    pub fn conjugate(&self, a: u64) -> u64 {
        assert_eq!(self.n, 2);
        self.count(|c| c.field_adds += 1);
        a % self.p + ((self.p - a / self.p) % self.p) * self.p
    }
    pub fn sqrt(&self, a: u64) -> Option<u64> {
        self.count(|c| c.sqrt_calls += 1);
        if a == 0 {
            return Some(0);
        }
        if let Some(t) = &self.tables {
            let e = t.1[a as usize];
            return if self.p == 2 {
                Some(t.0[((e * (self.size() / 2)) % (self.size() - 1)) as usize])
            } else if e % 2 == 0 {
                Some(t.0[(e / 2) as usize])
            } else {
                None
            };
        }
        if self.p == 2 {
            return Some(self.pow(a, self.size() / 2));
        }
        if self.n == 2 {
            let (x, y) = (a % self.p, a / self.p);
            if y == 0 {
                if let Some(r) = self.sqrt_base(x) {
                    return Some(r);
                }
                let nr_inv = self.bi(self.nr());
                let t = self.bm(x, nr_inv);
                return self.sqrt_base(t).map(|r| r * self.p);
            }
            let xx = self.bs(x);
            let yy = self.bs(y);
            let ny = if self.nr() == self.p - 1 {
                (self.p - yy) % self.p
            } else {
                self.bm(self.nr(), yy)
            };
            let norm = (xx + self.p - ny) % self.p;
            let s = self.sqrt_base(norm)?;
            let half = self.p.div_ceil(2);
            for sign_s in [s, (self.p - s) % self.p] {
                let t = self.bm((x + sign_s) % self.p, half);
                if let Some(u) = self.sqrt_base(t) {
                    if u != 0 {
                        let inv = self.bi(2 * u % self.p);
                        let v = self.bm(y, inv);
                        let out = u + self.p * v;
                        assert_eq!(self.sq(out), a);
                        return Some(out);
                    }
                }
            }
            return None;
        }
        let q = self.size();
        if self.pow(a, (q - 1) / 2) != 1 {
            return None;
        }
        let mut odd = q - 1;
        let mut s = 0;
        while odd & 1 == 0 {
            odd >>= 1;
            s += 1;
        }
        let z = (2..q).find(|&z| self.pow(z, (q - 1) / 2) != 1)?;
        let (mut c, mut x, mut t, mut m) = (
            self.pow(z, odd),
            self.pow(a, odd.div_ceil(2)),
            self.pow(a, odd),
            s,
        );
        while t != 1 {
            let mut j = 1;
            let mut u = self.sq(t);
            while u != 1 && j < m {
                u = self.sq(u);
                j += 1;
            }
            if j == m {
                return None;
            }
            let b = self.pow(c, 1 << (m - j - 1));
            x = self.mul(x, b);
            c = self.sq(b);
            t = self.mul(t, c);
            m = j;
        }
        Some(x)
    }
    fn sqrt_base(&self, a: u64) -> Option<u64> {
        if a == 0 {
            return Some(0);
        }
        // Count the actual base-field exponentiation, never a free lookup.
        let pow = |mut x: u64, mut e: u64| {
            let mut r = 1;
            while e > 0 {
                if e & 1 != 0 {
                    r = self.bm(r, x);
                }
                e >>= 1;
                if e > 0 {
                    x = self.bs(x);
                }
            }
            r
        };
        if pow(a, (self.p - 1) / 2) != 1 {
            return None;
        }
        if self.p % 4 == 3 {
            return Some(pow(a, (self.p + 1) / 4));
        }
        let (mut d, mut s) = (self.p - 1, 0);
        while d & 1 == 0 {
            d >>= 1;
            s += 1;
        }
        let z = (2..self.p).find(|&z| pow(z, (self.p - 1) / 2) != 1)?;
        let (mut c, mut x, mut t, mut m) = (pow(z, d), pow(a, d.div_ceil(2)), pow(a, d), s);
        while t != 1 {
            let mut j = 1;
            let mut u = self.bs(t);
            while u != 1 && j < m {
                u = self.bs(u);
                j += 1;
            }
            if j == m {
                return None;
            }
            let b = pow(c, 1 << (m - j - 1));
            x = self.bm(x, b);
            c = self.bs(b);
            t = self.bm(t, c);
            m = j;
        }
        Some(x)
    }
    pub fn artin_schreier(&self, a: u64) -> Option<u64> {
        assert_eq!(self.p, 2);
        let basis = self.as_basis.get_or_init(|| {
            let mut pivots = vec![(0, 0); self.n];
            for j in 0..self.n {
                let x = 1 << j;
                let mut image = self.sq(x) ^ x;
                let mut pre = x;
                while image != 0 {
                    let k = 63 - image.leading_zeros() as usize;
                    if pivots[k].0 == 0 {
                        pivots[k] = (image, pre);
                        break;
                    }
                    image ^= pivots[k].0;
                    pre ^= pivots[k].1;
                }
            }
            pivots
        });
        let (mut v, mut out) = (a, 0);
        while v != 0 {
            self.count(|c| c.bit_ops += 2);
            let k = 63 - v.leading_zeros() as usize;
            if basis[k].0 == 0 {
                return None;
            }
            v ^= basis[k].0;
            out ^= basis[k].1;
        }
        Some(out)
    }
}
fn binary_mul(mut a: u64, mut b: u64, modulus: u64, n: usize) -> u64 {
    let mut r = 0;
    while b != 0 {
        if b & 1 != 0 {
            r ^= a;
        }
        b >>= 1;
        a <<= 1;
        if a & (1 << n) != 0 {
            a ^= modulus;
        }
    }
    r
}
fn binary_rem(mut a: u64, b: u64) -> u64 {
    while a != 0 && a.leading_zeros() <= b.leading_zeros() {
        a ^= b << (b.leading_zeros() - a.leading_zeros());
    }
    a
}
fn binary_gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = binary_rem(a, b);
        a = b;
        b = r;
    }
    a
}
fn binary_irreducible(modulus: u64, n: usize) -> bool {
    let mut x = 2;
    for i in 1..=n {
        x = binary_mul(x, x, modulus, n);
        if i <= n / 2 && binary_gcd(x ^ 2, modulus) != 1 {
            return false;
        }
    }
    x == 2
}
pub fn binary_modulus(n: usize) -> u64 {
    ((1u64 << n) + 1..1u64 << (n + 1))
        .step_by(2)
        .find(|&f| binary_irreducible(f, n))
        .unwrap()
}
pub fn modpow(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1;
    while e != 0 {
        if e & 1 != 0 {
            r = (r as u128 * a as u128 % p as u128) as u64;
        }
        e >>= 1;
        if e != 0 {
            a = (a as u128 * a as u128 % p as u128) as u64;
        }
    }
    r
}
pub fn is_prime(p: u64) -> bool {
    if p < 2 {
        return false;
    }
    for q in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if p.is_multiple_of(q) {
            return p == q;
        }
    }
    let s = (p - 1).trailing_zeros();
    let d = (p - 1) >> s;
    'a: for a in [2, 325, 9375, 28178, 450775, 9780504, 1795265022] {
        if a % p == 0 {
            continue;
        }
        let mut x = modpow(a % p, d, p);
        if x == 1 || x == p - 1 {
            continue;
        }
        for _ in 1..s {
            x = (x as u128 * x as u128 % p as u128) as u64;
            if x == p - 1 {
                continue 'a;
            }
        }
        return false;
    }
    true
}
pub fn factor(mut n: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    let mut p = 2;
    while p * p <= n {
        if n.is_multiple_of(p) {
            let mut e = 0;
            while n.is_multiple_of(p) {
                n /= p;
                e += 1;
            }
            out.push((p, e));
        }
        p += if p == 2 { 1 } else { 2 };
    }
    if n > 1 {
        out.push((n, 1));
    }
    out
}
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Point {
    pub x: u64,
    pub y: u64,
    pub inf: bool,
}
impl Point {
    pub const O: Self = Self {
        x: 0,
        y: 0,
        inf: true,
    };
    pub fn affine(x: u64, y: u64) -> Self {
        Self { x, y, inf: false }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Elliptic {
    pub f: Field,
    /// Odd: y^2=c3*x^3+c2*x^2+c1*x+c0. Binary uses normalized ordinary form.
    pub c: [u64; 4],
    pub binary: bool,
}
impl Elliptic {
    pub fn rhs(&self, x: u64) -> u64 {
        let f = &self.f;
        f.add(
            f.mul(
                f.add(f.mul(f.add(f.mul(self.c[3], x), self.c[2]), x), self.c[1]),
                x,
            ),
            self.c[0],
        )
    }
    pub fn valid(&self, p: Point) -> bool {
        p.inf
            || self.f.add(
                self.f.sq(p.y),
                if self.binary { self.f.mul(p.x, p.y) } else { 0 },
            ) == self.rhs(p.x)
    }
    pub fn neg(&self, p: Point) -> Point {
        if p.inf {
            p
        } else {
            Point::affine(
                p.x,
                if self.binary {
                    p.y ^ p.x
                } else {
                    self.f.neg(p.y)
                },
            )
        }
    }
    pub fn add(&self, p: Point, q: Point) -> Point {
        self.f.count(|c| c.ec_adds += 1);
        if p.inf {
            return q;
        }
        if q.inf {
            return p;
        }
        let f = &self.f;
        if self.binary {
            if p.x == q.x {
                if p.y != q.y || p.x == 0 {
                    return Point::O;
                }
                let l = p.x ^ f.div(p.y, p.x);
                let x = f.sq(l) ^ l ^ self.c[2];
                return Point::affine(x, f.sq(p.x) ^ f.mul(l ^ 1, x));
            }
            let l = f.div(p.y ^ q.y, p.x ^ q.x);
            let x = f.sq(l) ^ l ^ p.x ^ q.x ^ self.c[2];
            return Point::affine(x, f.mul(l, p.x ^ x) ^ x ^ p.y);
        }
        if p.x == q.x && f.add(p.y, q.y) == 0 {
            return Point::O;
        }
        let l = if p.x == q.x {
            let x2 = f.sq(p.x);
            let numerator = f.add(
                f.add(
                    f.mul(f.constant(3), f.mul(self.c[3], x2)),
                    f.mul(f.constant(2), f.mul(self.c[2], p.x)),
                ),
                self.c[1],
            );
            f.div(numerator, f.mul(f.constant(2), p.y))
        } else {
            f.div(f.sub(q.y, p.y), f.sub(q.x, p.x))
        };
        let l2 = f.sub(f.sq(l), self.c[2]);
        let x = f.sub(
            f.sub(
                if self.c[3] == 1 {
                    l2
                } else {
                    f.div(l2, self.c[3])
                },
                p.x,
            ),
            q.x,
        );
        Point::affine(x, f.sub(f.mul(l, f.sub(p.x, x)), p.y))
    }
    pub fn mul(&self, mut p: Point, mut k: u64) -> Point {
        let mut r = Point::O;
        while k > 0 {
            if k & 1 != 0 {
                r = self.add(r, p);
            }
            k >>= 1;
            if k > 0 {
                p = self.add(p, p);
            }
        }
        r
    }
    pub fn lift(&self, x: u64) -> Vec<Point> {
        let f = &self.f;
        if self.binary {
            if x == 0 {
                return vec![Point::affine(0, f.sqrt(self.c[0]).unwrap())];
            }
            let t = x ^ self.c[2] ^ f.div(self.c[0], f.sq(x));
            return match f.artin_schreier(t) {
                Some(z) => {
                    let y = f.mul(x, z);
                    vec![Point::affine(x, y), Point::affine(x, y ^ x)]
                }
                None => vec![],
            };
        }
        match f.sqrt(self.rhs(x)) {
            None => vec![],
            Some(0) => vec![Point::affine(x, 0)],
            Some(y) => vec![Point::affine(x, y), Point::affine(x, f.neg(y))],
        }
    }
}

type Poly = Vec<u64>;
fn trim(mut a: Poly) -> Poly {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}
fn padd(f: &Field, a: &[u64], b: &[u64]) -> Poly {
    let mut r = vec![0; a.len().max(b.len())];
    for (i, v) in r.iter_mut().enumerate() {
        *v = f.add(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0));
    }
    trim(r)
}
fn pneg(f: &Field, a: &[u64]) -> Poly {
    a.iter().map(|&x| f.neg(x)).collect()
}
fn psub(f: &Field, a: &[u64], b: &[u64]) -> Poly {
    padd(f, a, &pneg(f, b))
}
fn pscale(f: &Field, a: &[u64], s: u64) -> Poly {
    trim(a.iter().map(|&x| f.mul(x, s)).collect())
}
fn pmul(f: &Field, a: &[u64], b: &[u64]) -> Poly {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] = f.add(r[i + j], f.mul(x, y));
        }
    }
    trim(r)
}
fn pdiv(f: &Field, a: &[u64], b: &[u64]) -> (Poly, Poly) {
    assert!(!b.is_empty());
    let mut r = trim(a.to_vec());
    let mut q = vec![0; a.len().saturating_sub(b.len()) + 1];
    let il = if b.last() == Some(&1) {
        1
    } else {
        f.inv(*b.last().unwrap())
    };
    while !r.is_empty() && r.len() >= b.len() {
        let j = r.len() - b.len();
        let c = if il == 1 {
            *r.last().unwrap()
        } else {
            f.mul(*r.last().unwrap(), il)
        };
        q[j] = c;
        for (k, &x) in b.iter().enumerate() {
            r[j + k] = f.sub(r[j + k], f.mul(c, x));
        }
        r = trim(r);
    }
    (trim(q), r)
}
fn prem(f: &Field, a: &[u64], b: &[u64]) -> Poly {
    pdiv(f, a, b).1
}
fn pexact(f: &Field, a: &[u64], b: &[u64]) -> Poly {
    let (q, r) = pdiv(f, a, b);
    assert!(r.is_empty(), "inexact polynomial division");
    q
}
fn pmonic(f: &Field, a: &[u64]) -> Poly {
    if a.is_empty() {
        vec![]
    } else if a.last() == Some(&1) {
        a.to_vec()
    } else {
        pscale(f, a, f.inv(*a.last().unwrap()))
    }
}
fn pegcd(f: &Field, a: &[u64], b: &[u64]) -> (Poly, Poly, Poly) {
    let (mut r0, mut r1, mut s0, mut s1, mut t0, mut t1) =
        (a.to_vec(), b.to_vec(), vec![1], vec![], vec![], vec![1]);
    while !r1.is_empty() {
        let (q, r) = pdiv(f, &r0, &r1);
        r0 = r1;
        r1 = r;
        let s = psub(f, &s0, &pmul(f, &q, &s1));
        s0 = s1;
        s1 = s;
        let t = psub(f, &t0, &pmul(f, &q, &t1));
        t0 = t1;
        t1 = t;
    }
    let inv = f.inv(*r0.last().unwrap());
    (
        pscale(f, &r0, inv),
        pscale(f, &s0, inv),
        pscale(f, &t0, inv),
    )
}
fn peval(f: &Field, a: &[u64], x: u64) -> u64 {
    a.iter().rev().fold(0, |r, &c| f.add(f.mul(r, x), c))
}
fn ppow(f: &Field, a: &[u64], n: usize) -> Poly {
    (0..n).fold(vec![1], |r, _| pmul(f, &r, a))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Divisor {
    pub degree: usize,
    pub u: [u64; 3],
    pub v: [u64; 2],
}
impl Divisor {
    pub const O: Self = Self {
        degree: 0,
        u: [1, 0, 0],
        v: [0, 0],
    };
    fn from(u: &[u64], v: &[u64]) -> Self {
        assert!(!u.is_empty() && u.len() <= 3 && v.len() < u.len());
        let mut d = Self::O;
        d.degree = u.len() - 1;
        d.u = [0; 3];
        d.u[..u.len()].copy_from_slice(u);
        d.v[..v.len()].copy_from_slice(v);
        d
    }
    fn up(&self) -> Poly {
        self.u[..=self.degree].to_vec()
    }
    fn vp(&self) -> Poly {
        trim(self.v[..self.degree].to_vec())
    }
}
#[derive(Clone, Debug)]
pub struct Jacobian {
    pub f: Field,
    pub polynomial: Poly,
}
impl Jacobian {
    pub fn valid(&self, d: Divisor) -> bool {
        d.degree <= 2
            && d.u[d.degree] == 1
            && prem(
                &self.f,
                &psub(&self.f, &self.polynomial, &pmul(&self.f, &d.vp(), &d.vp())),
                &d.up(),
            )
            .is_empty()
    }
    pub fn neg(&self, d: Divisor) -> Divisor {
        let mut out = d;
        for x in &mut out.v[..d.degree] {
            *x = self.f.neg(*x);
        }
        out
    }
    fn reduce(&self, mut u: Poly, mut v: Poly) -> Divisor {
        let f = &self.f;
        u = pmonic(f, &u);
        v = prem(f, &v, &u);
        while u.len() > 3 {
            let new_u = pmonic(
                f,
                &pexact(f, &psub(f, &self.polynomial, &pmul(f, &v, &v)), &u),
            );
            v = prem(f, &pneg(f, &v), &new_u);
            u = new_u;
        }
        Divisor::from(&u, &v)
    }
    pub fn add(&self, a: Divisor, b: Divisor) -> Divisor {
        self.f.count(|c| c.jac_adds += 1);
        if a.degree == 0 {
            return b;
        }
        if b.degree == 0 {
            return a;
        }
        let f = &self.f;
        let (ua, ub, va, vb) = (a.up(), b.up(), a.vp(), b.vp());
        if a.degree == 2 && b.degree == 2 {
            let aa = f.sub(a.u[1], b.u[1]);
            let bb = f.sub(a.u[0], b.u[0]);
            let cc = f.sub(b.v[1], a.v[1]);
            let dd = f.sub(b.v[0], a.v[0]);
            let norm = f.add(
                f.sub(f.sq(bb), f.mul(f.mul(aa, bb), b.u[1])),
                f.mul(f.sq(aa), b.u[0]),
            );
            let s1 = f.sub(f.mul(cc, bb), f.mul(aa, dd));
            if norm != 0 && s1 != 0 {
                // CRT composition and reduction, with the two variable
                // denominators batched into one inversion. Exceptional inputs
                // fall through to complete Cantor addition below.
                let both = f.inv(f.mul(norm, s1));
                let ni = f.mul(both, s1);
                let si = f.mul(both, norm);
                let s0 = f.add(
                    f.mul(dd, f.sub(bb, f.mul(aa, b.u[1]))),
                    f.mul(f.mul(aa, cc), b.u[0]),
                );
                let slope = vec![f.mul(s0, ni), f.mul(s1, ni)];
                let v = padd(f, &va, &pmul(f, &slope, &ua));
                let u = pmul(f, &ua, &ub);
                let quotient = pexact(f, &psub(f, &self.polynomial, &pmul(f, &v, &v)), &u);
                let inverse_lead = f.neg(f.sq(f.mul(norm, si)));
                let out_u = pscale(f, &quotient, inverse_lead);
                assert_eq!(out_u.last(), Some(&1));
                return Divisor::from(&out_u, &prem(f, &pneg(f, &v), &out_u));
            }
        }
        let (d1, e1, e2) = pegcd(f, &ua, &ub);
        let (d, c1, c2) = pegcd(f, &d1, &padd(f, &va, &vb));
        let s1 = pmul(f, &c1, &e1);
        let s2 = pmul(f, &c1, &e2);
        let u = pexact(f, &pmul(f, &ua, &ub), &pmul(f, &d, &d));
        let vnum = padd(
            f,
            &padd(
                f,
                &pmul(f, &pmul(f, &s1, &ua), &vb),
                &pmul(f, &pmul(f, &s2, &ub), &va),
            ),
            &pmul(f, &c2, &padd(f, &pmul(f, &va, &vb), &self.polynomial)),
        );
        let v = prem(f, &pexact(f, &vnum, &d), &u);
        self.reduce(u, v)
    }
    pub fn mul(&self, mut a: Divisor, mut k: u64) -> Divisor {
        let mut r = Divisor::O;
        while k > 0 {
            if k & 1 != 0 {
                r = self.add(r, a);
            }
            k >>= 1;
            if k > 0 {
                a = self.add(a, a);
            }
        }
        r
    }
    pub fn point(&self, x: u64, y: u64) -> Divisor {
        assert_eq!(self.f.sq(y), peval(&self.f, &self.polynomial, x));
        Divisor::from(&[self.f.neg(x), 1], &trim(vec![y]))
    }
}

#[derive(Clone, Debug)]
pub struct Scholten {
    pub p: u64,
    pub e: Elliptic,
    pub j: Jacobian,
    pub j2: Jacobian,
    pub ww: u64,
    pub rcoef: u64,
    pub scoef: u64,
}
impl Scholten {
    pub fn new(p: u64) -> Self {
        assert!(p > 13 && p % 4 == 3 && is_prime(p));
        let f = Field::new(p, 2, p - (-1i64).rem_euclid(p as i64) as u64);
        // x^2+1: the packed low constant is 1.
        assert_eq!(f.modulus, 1);
        let i = p;
        let r = f.add(f.constant(33), f.mul(f.constant(56), i));
        let s = f.add(f.constant(159), f.mul(f.constant(56), i));
        let ww = f.signed(-384);
        let iw = f.inv(ww);
        let e = Elliptic {
            f: f.clone(),
            c: [
                f.mul(f.conjugate(r), iw),
                f.mul(f.conjugate(s), iw),
                f.mul(s, iw),
                f.mul(r, iw),
            ],
            binary: false,
        };
        let b = vec![f.neg(i), f.sub(1, i)];
        let a = vec![i, f.add(1, i)];
        // F(t)=t^6 f_C(1+1/t), via the cover identity, avoids fractions.
        let polynomial = pscale(
            &f,
            &padd(
                &f,
                &padd(
                    &f,
                    &pscale(&f, &ppow(&f, &a, 6), r),
                    &pscale(&f, &pmul(&f, &ppow(&f, &a, 4), &ppow(&f, &b, 2)), s),
                ),
                &padd(
                    &f,
                    &pscale(
                        &f,
                        &pmul(&f, &ppow(&f, &a, 2), &ppow(&f, &b, 4)),
                        f.conjugate(s),
                    ),
                    &pscale(&f, &ppow(&f, &b, 6), f.conjugate(r)),
                ),
            ),
            iw,
        );
        assert_eq!(polynomial.len(), 6);
        assert!(polynomial.iter().all(|&x| x < p));
        let base = Field::new(p, 1, 0);
        let derivative: Poly = polynomial
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, &x)| base.mul(x, i as u64))
            .collect();
        assert_eq!(pegcd(&base, &polynomial, &derivative).0, vec![1]);
        let j = Jacobian {
            f: base,
            polynomial: polynomial.clone(),
        };
        let j2 = Jacobian { f, polynomial };
        Self {
            p,
            e,
            j,
            j2,
            ww,
            rcoef: r,
            scoef: s,
        }
    }
    pub fn tau(&self, p: Point) -> Divisor {
        if p.inf {
            return Divisor::O;
        }
        let f = &self.e.f;
        let i = self.p;
        let a = vec![i, f.add(1, i)];
        let b = vec![f.neg(i), f.sub(1, i)];
        let u = pmonic(
            f,
            &psub(f, &pmul(f, &a, &a), &pscale(f, &pmul(f, &b, &b), p.x)),
        );
        let v = prem(f, &pscale(f, &ppow(f, &b, 3), p.y), &u);
        let d = Divisor::from(&u, &v);
        assert!(self.j2.valid(d));
        let mut conjugate = d;
        for x in &mut conjugate.u {
            *x = f.conjugate(*x);
        }
        for x in &mut conjugate.v {
            *x = f.conjugate(*x);
        }
        let out = self.j2.add(d, conjugate);
        assert!(out.u.iter().chain(&out.v).all(|&x| x < self.p));
        assert!(self.j.valid(out));
        out
    }
    pub fn cover(&self, t: u64, y: u64) -> Point {
        let f = &self.e.f;
        let i = self.p;
        let a = f.add(f.mul(f.add(1, i), t), i);
        let b = f.sub(f.mul(f.sub(1, i), t), i);
        if b == 0 {
            return Point::O;
        }
        Point::affine(f.sq(f.div(a, b)), f.div(y, f.mul(f.sq(b), b)))
    }
    pub fn norm(&self, d: Divisor) -> Point {
        if d.degree == 0 {
            return Point::O;
        }
        let f = &self.e.f;
        let mut roots = Vec::new();
        if d.degree == 1 {
            roots.push(f.neg(d.u[0]));
        } else {
            let discriminant = f.sub(f.sq(d.u[1]), f.mul(f.constant(4), d.u[0]));
            let s = f.sqrt(discriminant).expect("Fp quadratic splits in Fp2");
            let half = f.inv(2);
            roots.push(f.mul(f.sub(s, d.u[1]), half));
            roots.push(f.mul(f.sub(f.neg(s), d.u[1]), half));
        }
        let mut out = Point::O;
        for t in roots {
            let y = peval(f, &d.vp(), t);
            let pt = self.cover(t, y);
            assert!(self.e.valid(pt));
            out = self.e.add(out, pt);
        }
        if d.degree == 1 {
            out = self.e.add(out, self.e.neg(Point::affine(f.signed(-1), 0)));
        }
        out
    }
    pub fn short_curve(&self) -> Elliptic {
        let f = &self.e.f;
        let c = &self.e.c;
        let third = f.inv(3);
        let shift = f.mul(c[2], third);
        let a = f.sub(f.mul(c[3], c[1]), f.mul(f.sq(c[2]), third));
        let b = f.add(
            f.sub(
                f.mul(
                    f.constant(2),
                    f.mul(f.mul(f.sq(c[2]), c[2]), f.pow(third, 3)),
                ),
                f.mul(f.mul(c[3], c[1]), shift),
            ),
            f.mul(f.sq(c[3]), c[0]),
        );
        Elliptic {
            f: f.clone(),
            c: [b, a, 0, 1],
            binary: false,
        }
    }
    pub fn to_short(&self, p: Point) -> Point {
        if p.inf {
            return p;
        }
        let f = &self.e.f;
        let c = &self.e.c;
        Point::affine(f.add(f.mul(c[3], p.x), f.div(c[2], 3)), f.mul(c[3], p.y))
    }
    pub fn from_short(&self, p: Point) -> Point {
        if p.inf {
            return p;
        }
        let f = &self.e.f;
        let c = &self.e.c;
        let inverse = f.inv(c[3]);
        Point::affine(
            f.mul(f.sub(p.x, f.div(c[2], 3)), inverse),
            f.mul(p.y, inverse),
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StudyInstance {
    pub curve: Elliptic,
    pub order: u64,
    pub r: u64,
    pub h: u64,
    pub generator: Point,
    pub family: String,
    pub parameter: u64,
    pub evidence: Value,
}
impl StudyInstance {
    pub fn fresh(&self) -> Self {
        let mut out = self.clone();
        out.curve.f = self.curve.f.fresh();
        out
    }
    pub fn validate(&self) -> Result<(), String> {
        self.curve.f.validate()?;
        let q = self.curve.f.size();
        if self.curve.c.iter().any(|&c| c >= q) || self.generator.x >= q || self.generator.y >= q {
            return Err("noncanonical study coefficient or generator coordinate".into());
        }
        let delta = (self.order as i128 - q as i128 - 1).unsigned_abs();
        if delta * delta > 4 * q as u128 {
            return Err("order outside Hasse interval".into());
        }
        if self.curve.binary {
            if self.curve.c[0] == 0 || self.curve.c[1] != 0 || self.curve.c[3] != 1 {
                return Err("singular or non-normalized ordinary binary curve".into());
            }
        } else if self.curve.c[2] != 0
            || self.curve.c[3] != 1
            || self.curve.f.add(
                self.curve.f.mul(
                    4,
                    self.curve
                        .f
                        .mul(self.curve.f.sq(self.curve.c[1]), self.curve.c[1]),
                ),
                self.curve
                    .f
                    .mul(27 % self.curve.f.p, self.curve.f.sq(self.curve.c[0])),
            ) == 0
        {
            return Err("singular or non-short study curve".into());
        }
        if !is_prime(self.r)
            || self.r < 5
            || self.r.checked_mul(self.h) != Some(self.order)
            || self.h.is_multiple_of(self.r)
            || !self.curve.valid(self.generator)
            || self.generator.inf
            || !self.curve.mul(self.generator, self.r).inf
        {
            return Err("invalid study subgroup fixture".into());
        }
        if (self.r as u128) * (self.r as u128) <= 16 * q as u128 {
            if q > 1 << 18 {
                return Err("order needs a stronger certificate".into());
            }
            if exhaustive_order(&self.curve) != self.order {
                return Err("exhaustive group-order mismatch".into());
            }
        }
        match self.family.as_str() {
            "scholten" => {
                if self.curve.f.n != 2
                    || self.curve.f.p <= 13
                    || self.curve.f.p % 4 != 3
                    || self.curve.f.modulus != 1
                    || Scholten::new(self.curve.f.p).short_curve().c != self.curve.c
                {
                    return Err("Scholten family/model mismatch".into());
                }
            }
            "subfield" => {
                let valid = if self.curve.binary {
                    (1..=15).contains(&self.parameter)
                        && self.curve.f.n == 2 * self.parameter as usize
                } else {
                    self.curve.f.n == 2 && self.parameter == 1
                };
                if !valid {
                    return Err("unsupported subfield degree".into());
                }
            }
            "pkm_kummer" | "pkm_dickson" => {
                if self.curve.f.n != 1
                    || !(2..=4).contains(&self.parameter)
                    || !(self.curve.f.p - 1).is_multiple_of(1 << self.parameter)
                {
                    return Err("unsupported tower parameters".into());
                }
            }
            _ => return Err("unknown three-leads family".into()),
        }
        Ok(())
    }
}

pub fn exhaustive_order(e: &Elliptic) -> u64 {
    (0..e.f.size()).map(|x| e.lift(x).len() as u64).sum::<u64>() + 1
}
pub fn interval_order(e: &Elliptic, seed: u64) -> Option<(u64, Value)> {
    assert_eq!(e.f.n, 2);
    let p = e.f.p;
    let lower = (p - 1) * (p - 1);
    let width = 4 * p;
    let m = (width as f64).sqrt().ceil() as u64 + 1;
    let mut rng = Rng(seed);
    for attempt in 0..32 {
        let mut point = Point::O;
        for _ in 0..100 {
            let x = rng.next() % e.f.size();
            if let Some(&p) = e.lift(x).first() {
                point = p;
                break;
            }
        }
        if point.inf {
            continue;
        }
        let mut baby = HashMap::new();
        let mut cur = Point::O;
        for j in 0..m {
            baby.entry(cur).or_insert(j);
            cur = e.add(cur, point);
        }
        let giant = e.neg(e.mul(point, m));
        let mut q = e.neg(e.mul(point, lower));
        for i in 0..=width / m + 1 {
            if let Some(&j) = baby.get(&q) {
                let offset = i * m + j;
                if offset <= width {
                    let order = lower + offset;
                    let fs = factor(order);
                    if let Some(&(r, 1)) = fs.last() {
                        let h = order / r;
                        if r > 4 * p && !h.is_multiple_of(r) {
                            let g = e.mul(point, h);
                            if !g.inf && e.mul(g, r).inf {
                                return Some((
                                    order,
                                    json!({"kind":"Hasse_unique_prime_order_witness","p":p,"point":g,"r":r,"h":h,"order":order,"attempt":attempt,"hasse_lower":lower,"hasse_width":width}),
                                ));
                            }
                        }
                    }
                }
            }
            q = e.add(q, giant);
        }
    }
    None
}
pub fn scholten_instance(p: u64, seed: u64) -> Option<StudyInstance> {
    let s = Scholten::new(p);
    let curve = s.short_curve();
    let (order, evidence) = interval_order(&curve, seed)?;
    let r = evidence["r"].as_u64()?;
    let generator: Point = serde_json::from_value(evidence["point"].clone()).ok()?;
    let out = StudyInstance {
        curve,
        order,
        r,
        h: order / r,
        generator,
        family: "scholten".into(),
        parameter: 0,
        evidence,
    };
    out.validate().ok()?;
    Some(out)
}
#[derive(Clone, Copy)]
pub struct Rng(pub u64);
#[allow(clippy::should_implement_trait)]
impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        mix(self.0)
    }
}
pub fn mix(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

#[derive(Clone, Copy, Default, Debug, Serialize)]
pub struct ScalarCounts {
    pub add: u64,
    pub mul: u64,
    pub inv: u64,
}
#[derive(Clone)]
pub struct Ring {
    pub r: u64,
    pub counts: Rc<Cell<ScalarCounts>>,
}
impl Ring {
    pub fn new(r: u64) -> Self {
        assert!(is_prime(r) && r > 2);
        Self {
            r,
            counts: Default::default(),
        }
    }
    pub fn add(&self, a: u64, b: u64) -> u64 {
        let mut c = self.counts.get();
        c.add += 1;
        self.counts.set(c);
        (a + b) % self.r
    }
    pub fn neg(&self, a: u64) -> u64 {
        let mut c = self.counts.get();
        c.add += 1;
        self.counts.set(c);
        (self.r - a) % self.r
    }
    pub fn sub(&self, a: u64, b: u64) -> u64 {
        self.add(a, self.neg(b))
    }
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        let mut c = self.counts.get();
        c.mul += 1;
        self.counts.set(c);
        (a as u128 * b as u128 % self.r as u128) as u64
    }
    pub fn inv(&self, a: u64) -> u64 {
        assert_ne!(a, 0);
        let mut c = self.counts.get();
        c.inv += 1;
        self.counts.set(c);
        modpow(a, self.r - 2, self.r)
    }
    fn signed(&self, sign: i8, a: u64) -> u64 {
        if sign == 1 {
            a
        } else {
            self.neg(a)
        }
    }
}
#[derive(Clone, Copy, Debug)]
struct Potential {
    s: i8,
    a: u64,
    b: u64,
}
impl Potential {
    const ID: Self = Self { s: 1, a: 0, b: 0 };
}
/// Signed disjoint-set forest. A node denotes s*root+a+b*k. A negative
/// graph cycle anchors a root; it is not automatically a target relation.
pub struct SignedGraph {
    ring: Ring,
    parent: Vec<usize>,
    size: Vec<usize>,
    pot: Vec<Potential>,
    fixed: Vec<Option<(u64, u64)>>,
    pub cycles: u64,
    pub negative_cycles: u64,
    pub zero_coefficient: u64,
    pub operations: u64,
}
impl SignedGraph {
    pub fn new(ring: Ring) -> Self {
        Self {
            ring,
            parent: vec![0],
            size: vec![1],
            pot: vec![Potential::ID],
            fixed: vec![Some((0, 0))],
            cycles: 0,
            negative_cycles: 0,
            zero_coefficient: 0,
            operations: 0,
        }
    }
    pub fn node(&mut self) -> usize {
        let i = self.parent.len();
        self.parent.push(i);
        self.size.push(1);
        self.pot.push(Potential::ID);
        self.fixed.push(None);
        i
    }
    fn find(&mut self, x: usize) -> (usize, Potential) {
        self.operations += 1;
        let parent = self.parent[x];
        if parent == x {
            return (x, Potential::ID);
        }
        let (root, q) = self.find(parent);
        let p = self.pot[x];
        let out = Potential {
            s: p.s * q.s,
            a: self.ring.add(p.a, self.ring.signed(p.s, q.a)),
            b: self.ring.add(p.b, self.ring.signed(p.s, q.b)),
        };
        self.parent[x] = root;
        self.pot[x] = out;
        (root, out)
    }
    fn constraint(&mut self, a: u64, b: u64) -> Option<u64> {
        if b != 0 {
            Some(self.ring.mul(self.ring.neg(a), self.ring.inv(b)))
        } else {
            self.zero_coefficient += 1;
            assert_eq!(a, 0, "inconsistent signed relation");
            None
        }
    }
    fn anchor(&mut self, root: usize, a: u64, b: u64) -> Option<u64> {
        match self.fixed[root] {
            Some((x, y)) => self.constraint(self.ring.sub(x, a), self.ring.sub(y, b)),
            None => {
                self.fixed[root] = Some((a, b));
                None
            }
        }
    }
    /// Rank effect of an edge after reducing its target coefficient against
    /// the current signed potentials. This performs counted `find` and ring
    /// operations, but does not insert the edge. Scores are: 0 target-free,
    /// 1 target-bearing merge, 2 target-bearing anchor, 3 target equation.
    pub fn target_rank_score(&mut self, u: usize, su: i8, v: usize, sv: i8, b: u64) -> u8 {
        let (ru, pu) = self.find(u);
        let (rv, pv) = self.find(v);
        let cu = su * pu.s;
        let cv = sv * pv.s;
        let b = self.ring.sub(
            self.ring.sub(b, self.ring.signed(su, pu.b)),
            self.ring.signed(sv, pv.b),
        );
        if ru == rv {
            if cu + cv == 0 {
                return u8::from(b != 0) * 3;
            }
            if let Some((_, fixed_b)) = self.fixed[ru] {
                let doubled = self.ring.mul(2, fixed_b);
                return u8::from(b != self.ring.signed(cu, doubled)) * 3;
            }
            return u8::from(b != 0) * 2;
        }
        let residual = if let Some((_, fixed_b)) = self.fixed[ru] {
            self.ring.sub(b, self.ring.signed(cu, fixed_b))
        } else {
            b
        };
        let residual = if let Some((_, fixed_b)) = self.fixed[rv] {
            self.ring.sub(residual, self.ring.signed(cv, fixed_b))
        } else {
            residual
        };
        if residual == 0 {
            0
        } else if self.fixed[ru].is_some() && self.fixed[rv].is_some() {
            3
        } else if self.fixed[ru].is_some() || self.fixed[rv].is_some() {
            2
        } else {
            1
        }
    }
    /// Edge su*xu+sv*xv=a+b*k, all coefficients reduced modulo the subgroup order.
    pub fn edge(&mut self, u: usize, su: i8, v: usize, sv: i8, a: u64, b: u64) -> Option<u64> {
        let (ru, pu) = self.find(u);
        let (rv, pv) = self.find(v);
        let cu = su * pu.s;
        let cv = sv * pv.s;
        let a = self.ring.sub(
            self.ring.sub(a, self.ring.signed(su, pu.a)),
            self.ring.signed(sv, pv.a),
        );
        let b = self.ring.sub(
            self.ring.sub(b, self.ring.signed(su, pu.b)),
            self.ring.signed(sv, pv.b),
        );
        if ru == rv {
            self.cycles += 1;
            if cu + cv == 0 {
                return self.constraint(self.ring.neg(a), self.ring.neg(b));
            }
            self.negative_cycles += 1;
            let ci = self.ring.inv(if cu == 1 { 2 } else { self.ring.r - 2 });
            return self.anchor(ru, self.ring.mul(a, ci), self.ring.mul(b, ci));
        }
        // ru = s*rv + aa+bb*k.
        let s = -cu * cv;
        let aa = self.ring.signed(cu, a);
        let bb = self.ring.signed(cu, b);
        let (child, parent, pot) = if self.size[ru] <= self.size[rv] {
            (ru, rv, Potential { s, a: aa, b: bb })
        } else {
            (
                rv,
                ru,
                Potential {
                    s,
                    a: self.ring.signed(-s, aa),
                    b: self.ring.signed(-s, bb),
                },
            )
        };
        let child_fixed = self.fixed[child];
        let parent_fixed = self.fixed[parent];
        self.parent[child] = parent;
        self.pot[child] = pot;
        self.size[parent] += self.size[child];
        if let Some((x, y)) = child_fixed {
            let a = self.ring.signed(pot.s, self.ring.sub(x, pot.a));
            let b = self.ring.signed(pot.s, self.ring.sub(y, pot.b));
            if parent_fixed.is_some() {
                return self.anchor(parent, a, b);
            }
            self.fixed[parent] = Some((a, b));
        }
        None
    }
    pub fn nodes(&self) -> usize {
        self.parent.len()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct NativePhase {
    pub name: String,
    pub counts: Counts,
}
#[derive(Clone, Debug, Serialize)]
pub struct Outcome {
    pub recovered: Option<u64>,
    pub exhausted: bool,
    pub phases: Vec<NativePhase>,
    pub counts: Counts,
    pub native: BTreeMap<String, u64>,
    pub detail: Value,
}
fn uniform(rng: &mut Rng, r: u64, draws: &mut u64) -> u64 {
    let upper = u64::MAX - u64::MAX % r;
    loop {
        *draws += 1;
        let a = rng.next();
        if a < upper {
            return a % r;
        }
    }
}
pub fn solve_genus2(
    inst: &StudyInstance,
    target: Point,
    seed: u64,
    trial_cap: u64,
    independent: bool,
    unique_relations: bool,
) -> Outcome {
    genus2_inner(
        inst,
        target,
        seed,
        trial_cap,
        independent,
        unique_relations,
        false,
    )
}
pub fn solve_genus2_v2(
    inst: &StudyInstance,
    target: Point,
    seed: u64,
    trial_cap: u64,
    independent: bool,
    unique_relations: bool,
) -> Outcome {
    genus2_inner(
        inst,
        target,
        seed,
        trial_cap,
        independent,
        unique_relations,
        true,
    )
}
fn genus2_inner(
    inst: &StudyInstance,
    target: Point,
    seed: u64,
    trial_cap: u64,
    independent: bool,
    unique_relations: bool,
    restart_unsplit: bool,
) -> Outcome {
    assert_eq!(inst.family, "scholten");
    assert!(inst.curve.valid(target));
    let s = Scholten::new(inst.curve.f.p);
    let source = s.short_curve();
    assert_eq!(source.c, inst.curve.c);
    let counts = || s.e.f.counts() + s.j.f.counts();
    let mut phases = vec![NativePhase {
        name: "destination_construction".into(),
        counts: counts(),
    }];
    let mut previous = counts();
    let p = s.tau(s.from_short(inst.generator));
    let q = s.tau(s.from_short(target));
    assert_ne!(p, Divisor::O);
    assert_ne!(q, Divisor::O);
    phases.push(NativePhase {
        name: "transfer".into(),
        counts: counts() - previous,
    });
    previous = counts();
    let ring = Ring::new(inst.r);
    let mut rng = Rng(seed);
    let mut draws = 0;
    let mut jumps = Vec::new();
    if !independent {
        for _ in 0..16 {
            let a = uniform(&mut rng, inst.r.min(4096), &mut draws);
            let b = uniform(&mut rng, inst.r.min(4096), &mut draws);
            let point = s.j.add(s.j.mul(p, a), s.j.mul(q, b));
            jumps.push((point, a, b));
        }
    }
    let (mut a, mut b) = (
        uniform(&mut rng, inst.r, &mut draws),
        uniform(&mut rng, inst.r, &mut draws),
    );
    let mut d = if independent {
        Divisor::O
    } else {
        s.j.add(s.j.mul(p, a), s.j.mul(q, b))
    };
    phases.push(NativePhase {
        name: "walk_initialization".into(),
        counts: counts() - previous,
    });
    previous = counts();
    let mut graph = SignedGraph::new(ring.clone());
    let mut nodes = BTreeMap::new();
    let mut seen = std::collections::BTreeSet::new();
    let mut duplicates = 0;
    let mut walk_seen = std::collections::HashSet::new();
    let mut walk_restarts = 0;
    let mut recovered = None;
    let mut first_cycle = None;
    let mut split = 0;
    let mut hash_probes = 0;
    let mut trials = 0;
    for trial in 1..=trial_cap {
        trials = trial;
        if restart_unsplit && !independent {
            hash_probes += 1;
            if !walk_seen.insert(d) {
                walk_restarts += 1;
                walk_seen.clear();
                a = uniform(&mut rng, inst.r, &mut draws);
                b = uniform(&mut rng, inst.r, &mut draws);
                d = s.j.add(s.j.mul(p, a), s.j.mul(q, b));
            }
        }
        if independent {
            a = uniform(&mut rng, inst.r, &mut draws);
            b = uniform(&mut rng, inst.r, &mut draws);
            // Uniform known-label source point followed by the actual transfer;
            // all scalar generation and map work remains in the counters.
            let r = source.add(source.mul(inst.generator, a), source.mul(target, b));
            d = s.tau(s.from_short(r));
        } else {
            let key = mix(d.u[0]
                ^ d.u[1].rotate_left(19)
                ^ d.v[0].rotate_left(37)
                ^ d.v[1].rotate_left(7));
            let j = &jumps[key as usize % jumps.len()];
            d = s.j.add(d, j.0);
            a = ring.add(a, j.1);
            b = ring.add(b, j.2);
        }
        if d.degree == 0 {
            if b != 0 {
                recovered = Some(ring.mul(ring.neg(a), ring.inv(b)));
                break;
            }
            continue;
        }
        let f = &s.j.f;
        let roots = if d.degree == 1 {
            vec![f.neg(d.u[0])]
        } else {
            let disc = f.sub(f.sq(d.u[1]), f.mul(4, d.u[0]));
            let Some(root) = f.sqrt(disc) else {
                continue;
            };
            let half = f.p.div_ceil(2);
            vec![
                f.mul(f.sub(root, d.u[1]), half),
                f.mul(f.sub(f.neg(root), d.u[1]), half),
            ]
        };
        let mut edge = vec![];
        let canonical = d.min(s.j.neg(d));
        hash_probes += 1;
        if !seen.insert(canonical) {
            duplicates += 1;
            if unique_relations {
                if !independent {
                    a = uniform(&mut rng, inst.r, &mut draws);
                    b = uniform(&mut rng, inst.r, &mut draws);
                    d = s.j.add(s.j.mul(p, a), s.j.mul(q, b));
                }
                continue;
            }
        }
        for x in roots {
            let y = peval(f, &d.vp(), x);
            assert_eq!(f.sq(y), peval(f, &s.j.polynomial, x));
            let sign = if y <= f.p / 2 { 1 } else { -1 };
            hash_probes += 1;
            let node = *nodes.entry(x).or_insert_with(|| graph.node());
            edge.push((node, sign));
        }
        if edge.len() == 1 {
            edge.push((0, 1));
        }
        split += 1;
        let before = graph.cycles;
        recovered = graph.edge(edge[0].0, edge[0].1, edge[1].0, edge[1].1, a, b);
        if graph.cycles > before && first_cycle.is_none() {
            first_cycle = Some(trial);
        }
        if recovered.is_some() {
            break;
        }
    }
    phases.push(NativePhase {
        name: "relations_and_signed_elimination".into(),
        counts: counts() - previous,
    });
    let scalar = ring.counts.get();
    let mut native = BTreeMap::from([
        ("rng_draws_uncharged".into(), draws),
        ("hash_probes_uncharged".into(), hash_probes),
        ("graph_operations_uncharged".into(), graph.operations),
        ("scalar_adds_uncharged".into(), scalar.add),
        ("scalar_muls_uncharged".into(), scalar.mul),
        ("scalar_inversions_uncharged".into(), scalar.inv),
        ("processed_divisors".into(), trials),
        ("split_relations".into(), split),
        ("graph_cycles".into(), graph.cycles),
        ("negative_cycles".into(), graph.negative_cycles),
        ("zero_target_coefficients".into(), graph.zero_coefficient),
        ("duplicate_relation_divisors".into(), duplicates),
        ("first_graph_cycle_trial".into(), first_cycle.unwrap_or(0)),
        ("large_prime_nodes".into(), graph.nodes() as u64),
        (
            "logical_graph_bytes".into(),
            (graph.nodes()
                * (std::mem::size_of::<usize>() * 2
                    + std::mem::size_of::<Potential>()
                    + std::mem::size_of::<Option<(u64, u64)>>())
                + nodes.len() * 24) as u64,
        ),
    ]);
    if restart_unsplit {
        native.insert("walk_cycle_restarts".into(), walk_restarts);
        native.insert(
            "logical_walk_seen_bytes".into(),
            (walk_seen.capacity() * std::mem::size_of::<Divisor>()) as u64,
        );
    }
    Outcome {
        recovered,
        exhausted: recovered.is_none(),
        phases,
        counts: counts(),
        native,
        detail: json!({"mode":if independent{"independent_source_scalars_then_tau"}else{"jacobian_r_adding_walk"},"unique_relations":unique_relations,"p":inst.curve.f.p,"r":inst.r,"h":inst.h,"field_size":inst.curve.f.size(),"first_cycle_trial":first_cycle,"solve_trial":trials,"quintic":s.j.polynomial,"subgroup_map":"tau_pullback_plus_conjugate","dual_composition":"[2]"}),
    }
}

#[derive(Clone)]
pub struct FactorBase {
    pub raw: Vec<Point>,
    pub xs: Vec<u64>,
    pub columns: usize,
    map: BTreeMap<Point, Option<(usize, i8)>>,
    subfield_as: Vec<(u64, u64)>,
    probes: Cell<u64>,
    draws: u64,
    trace_mask: Option<u64>,
    trace_tests: Cell<u64>,
    trace_skips: Cell<u64>,
}
impl FactorBase {
    pub fn prepare_trace_filter(&mut self, f: &Field) {
        assert_eq!(f.p, 2);
        let mut mask = 0;
        for j in 0..f.n {
            let mut x = 1 << j;
            let mut trace = 0;
            for k in 0..f.n {
                trace ^= x;
                f.count(|c| c.bit_ops += 1);
                if k + 1 < f.n {
                    x = f.sq(x);
                }
            }
            assert!(trace <= 1);
            if trace == 1 {
                mask |= 1 << j;
            }
        }
        self.trace_mask = Some(mask);
    }
}
pub fn subfield_elements(f: &Field, q: u64) -> Vec<u64> {
    if f.p != 2 {
        return (0..q).collect();
    }
    let n = f.size() - 1;
    let factors = factor(n);
    let g = (2..f.size())
        .find(|&g| factors.iter().all(|&(p, _)| f.pow(g, n / p) != 1))
        .expect("primitive element");
    let generator = f.pow(g, n / (q - 1));
    let mut out = vec![0];
    let mut x = 1;
    for _ in 0..q - 1 {
        out.push(x);
        x = f.mul(x, generator);
    }
    assert_eq!(x, 1);
    out.sort_unstable();
    out.dedup();
    assert_eq!(out.len(), q as usize);
    out
}
pub fn tower_elements(f: &Field, depth: u64, dickson: bool) -> Vec<u64> {
    assert_eq!(f.n, 1);
    let d = 1 << depth;
    assert_eq!((f.p - 1) % d, 0);
    let fs = factor(f.p - 1);
    let g = (2..f.p)
        .find(|&g| fs.iter().all(|&(q, _)| f.pow(g, (f.p - 1) / q) != 1))
        .unwrap();
    let z = f.pow(g, (f.p - 1) / d);
    let u = if dickson {
        (2..f.p).find(|&u| f.pow(u, 2 * d) != 1).unwrap()
    } else {
        1
    };
    let mut x = u;
    let mut out = vec![];
    for _ in 0..d {
        out.push(if dickson { f.add(x, f.inv(x)) } else { x });
        x = f.mul(x, z);
    }
    out.sort_unstable();
    out.dedup();
    assert_eq!(out.len(), d as usize);
    out
}
fn as_basis_on(f: &Field, elements: &[u64]) -> Vec<(u64, u64)> {
    let mut basis = vec![(0, 0); f.n];
    for &x in elements {
        let (mut image, mut pre) = (f.sq(x) ^ x, x);
        while image != 0 {
            let j = 63 - image.leading_zeros() as usize;
            if basis[j].0 == 0 {
                basis[j] = (image, pre);
                break;
            }
            f.count(|c| c.bit_ops += 2);
            image ^= basis[j].0;
            pre ^= basis[j].1;
        }
    }
    basis
}
fn as_solve_on(f: &Field, basis: &[(u64, u64)], mut image: u64) -> Option<u64> {
    let mut out = 0;
    while image != 0 {
        let j = 63 - image.leading_zeros() as usize;
        if basis[j].0 == 0 {
            return None;
        }
        f.count(|c| c.bit_ops += 2);
        image ^= basis[j].0;
        out ^= basis[j].1;
    }
    Some(out)
}
pub fn build_factor_base(inst: &StudyInstance, random: bool, base_seed: u64) -> FactorBase {
    let e = &inst.curve;
    let f = &e.f;
    let xs = match inst.family.as_str() {
        "subfield" => subfield_elements(f, if f.p == 2 { 1 << inst.parameter } else { f.p }),
        "pkm_kummer" => tower_elements(f, inst.parameter, false),
        "pkm_dickson" => tower_elements(f, inst.parameter, true),
        _ => panic!("unsupported factor base"),
    };
    let subfield_as = if f.p == 2 {
        as_basis_on(f, &xs)
    } else {
        vec![]
    };
    let mut raw = vec![];
    for &x in &xs {
        raw.extend(e.lift(x));
    }
    raw.sort();
    let mut draws = 0;
    if random {
        let torsion: Vec<_> = raw.iter().copied().filter(|p| e.neg(*p) == *p).collect();
        let pairs = (raw.len() - torsion.len()) / 2;
        let mut selected = BTreeMap::new();
        let mut rng = Rng(base_seed);
        while selected.len() < pairs {
            let x = uniform(&mut rng, f.size(), &mut draws);
            if selected.contains_key(&x) || torsion.iter().any(|p| p.x == x) {
                continue;
            }
            let pts = e.lift(x);
            if pts.len() == 2 {
                selected.insert(x, pts);
            }
        }
        raw = torsion;
        for pts in selected.into_values() {
            raw.extend(pts);
        }
        raw.sort();
    }
    let mut columns = BTreeMap::new();
    let mut map = BTreeMap::new();
    for &point in &raw {
        let image = e.mul(point, inst.h);
        let projected = if image.inf {
            None
        } else {
            let neg = e.neg(image);
            let key = image.min(neg);
            let next = columns.len();
            let index = *columns.entry(key).or_insert(next);
            Some((index, if image == key { 1 } else { -1 }))
        };
        map.insert(point, projected);
    }
    let mut usable_x: Vec<_> = raw.iter().map(|p| p.x).collect();
    usable_x.sort_unstable();
    usable_x.dedup();
    FactorBase {
        raw,
        xs: usable_x,
        columns: columns.len(),
        map,
        subfield_as,
        probes: Cell::new(0),
        draws,
        trace_mask: None,
        trace_tests: Cell::new(0),
        trace_skips: Cell::new(0),
    }
}
fn verified_pair(
    e: &Elliptic,
    base: &FactorBase,
    target: Point,
    x: u64,
    y: u64,
) -> Option<Vec<Point>> {
    base.probes.set(base.probes.get() + base.raw.len() as u64);
    for &p in base.raw.iter().filter(|p| p.x == x) {
        let q = e.add(target, e.neg(p));
        if !q.inf && q.x == y && base.map.contains_key(&q) {
            assert_eq!(e.add(p, q), target);
            return Some(vec![p, q]);
        }
    }
    None
}
pub fn decompose(
    e: &Elliptic,
    base: &FactorBase,
    target: Point,
    oracle: &str,
    parameter: u64,
    root_calls: &mut u64,
) -> Option<Vec<Point>> {
    if target.inf {
        if let Some(&p) = base.raw.first() {
            return Some(vec![p, e.neg(p)]);
        }
        return None;
    }
    if oracle == "subtract" {
        for &p in &base.raw {
            base.probes.set(base.probes.get() + 1);
            let q = e.add(target, e.neg(p));
            if base.map.contains_key(&q) {
                assert_eq!(e.add(p, q), target);
                return Some(vec![p, q]);
            }
        }
        return None;
    }
    let f = &e.f;
    if oracle == "linearized_filtered" {
        f.count(|c| c.bit_ops += 4);
        base.trace_tests.set(base.trace_tests.get() + 1);
        if ((target.x ^ e.c[2]) & base.trace_mask.expect("prepared binary trace filter"))
            .count_ones()
            & 1
            == 1
        {
            base.trace_skips.set(base.trace_skips.get() + 1);
            return None;
        }
    }
    if e.binary && oracle.starts_with("linearized") {
        let q = 1 << parameter;
        let t = target.x;
        let tq = f.pow(t, q);
        let trace = t ^ tq;
        if trace == 0 {
            return decompose(e, base, target, "subtract", parameter, root_calls);
        }
        *root_calls += 1;
        let norm = f.mul(t, tq);
        let b1 = f.div(e.c[0] ^ f.pow(e.c[0], q), trace);
        let b0 = e.c[0] ^ f.mul(b1, t);
        let a = f.sq(trace);
        let c = f.sq(b1) ^ b0;
        let rhs = f.div(f.mul(a, c), f.sq(norm));
        let z = as_solve_on(f, &base.subfield_as, rhs)?;
        let scale = f.div(norm, a);
        for root in [z, z ^ 1] {
            let w = f.mul(scale, root);
            let u = f.pow(w, q / 2);
            let v = f.mul(trace, w) ^ b1;
            *root_calls += 1;
            let x = if u == 0 {
                f.pow(v, q / 2)
            } else {
                let z = as_solve_on(f, &base.subfield_as, f.div(v, f.sq(u)));
                let Some(z) = z else {
                    continue;
                };
                f.mul(u, z)
            };
            let y = x ^ u;
            if let Some(pair) = verified_pair(e, base, target, x, y) {
                return Some(pair);
            }
        }
        return None;
    }
    assert!(
        !e.binary && e.c[2] == 0 && e.c[3] == 1,
        "S3 odd short model"
    );
    let t = target.x;
    let a = e.c[1];
    let b = e.c[0];
    for &u in &base.xs {
        *root_calls += 1;
        let ut = f.mul(u, t);
        let sum = f.add(u, t);
        let aa = f.sq(f.sub(u, t));
        let bb = f.neg(f.mul(
            f.constant(2),
            f.add(f.mul(sum, f.add(ut, a)), f.mul(f.constant(2), b)),
        ));
        let cc = f.sub(f.sq(f.sub(ut, a)), f.mul(f.constant(4), f.mul(b, sum)));
        let roots = if aa == 0 {
            if bb == 0 {
                if cc == 0 {
                    base.xs.clone()
                } else {
                    vec![]
                }
            } else {
                vec![f.div(f.neg(cc), bb)]
            }
        } else {
            let disc = f.sub(f.sq(bb), f.mul(f.constant(4), f.mul(aa, cc)));
            let Some(s) = f.sqrt(disc) else {
                continue;
            };
            let den = f.inv(f.mul(f.constant(2), aa));
            vec![f.mul(f.sub(s, bb), den), f.mul(f.sub(f.neg(s), bb), den)]
        };
        for x in roots {
            if base.xs.binary_search(&x).is_ok() {
                if let Some(pair) = verified_pair(e, base, target, u, x) {
                    return Some(pair);
                }
            }
        }
    }
    None
}
struct Elimination {
    ring: Ring,
    pivots: Vec<Option<Vec<u64>>>,
    rank: usize,
}
impl Elimination {
    fn new(ring: Ring, variables: usize) -> Self {
        Self {
            ring,
            pivots: vec![None; variables],
            rank: 0,
        }
    }
    fn insert(&mut self, mut row: Vec<u64>) -> Option<u64> {
        let n = self.pivots.len();
        assert_eq!(row.len(), n + 1);
        for j in 0..n {
            if row[j] == 0 {
                continue;
            }
            if let Some(pivot) = &self.pivots[j] {
                let scale = row[j];
                for k in j..=n {
                    row[k] = self.ring.sub(row[k], self.ring.mul(scale, pivot[k]));
                }
            } else {
                let inverse = self.ring.inv(row[j]);
                for value in &mut row[j..] {
                    *value = self.ring.mul(*value, inverse);
                }
                self.rank += 1;
                if j == n - 1 {
                    return Some(row[n]);
                }
                self.pivots[j] = Some(row);
                return None;
            }
        }
        assert_eq!(row[n], 0, "inconsistent IC row");
        None
    }
}
pub fn solve_subfield_or_pkm(
    inst: &StudyInstance,
    target: Point,
    seed: u64,
    trial_cap: u64,
    oracle: &str,
    random: bool,
    base_seed: u64,
) -> Outcome {
    decomposition_inner(
        inst, target, seed, trial_cap, oracle, random, base_seed, false,
    )
}
pub fn solve_subfield_or_pkm_v2(
    inst: &StudyInstance,
    target: Point,
    seed: u64,
    trial_cap: u64,
    oracle: &str,
    random: bool,
    base_seed: u64,
) -> Outcome {
    decomposition_inner(
        inst, target, seed, trial_cap, oracle, random, base_seed, true,
    )
}
#[allow(clippy::too_many_arguments)]
fn decomposition_inner(
    inst: &StudyInstance,
    target: Point,
    seed: u64,
    trial_cap: u64,
    oracle: &str,
    random: bool,
    base_seed: u64,
    include_restart_targets: bool,
) -> Outcome {
    let fresh = inst.fresh();
    let inst = &fresh;
    let e = &inst.curve;
    let f = &e.f;
    f.reset();
    let mut base = build_factor_base(inst, random, base_seed);
    if oracle == "linearized_filtered" {
        base.prepare_trace_filter(f);
    }
    let mut phases = vec![NativePhase {
        name: "factor_base_and_projection".into(),
        counts: f.counts(),
    }];
    let mut previous = f.counts();
    let ring = Ring::new(inst.r);
    let mut matrix = Elimination::new(ring.clone(), base.columns + 1);
    let mut rng = Rng(seed);
    let mut draws = 0;
    let mut jumps = vec![];
    let offset = |rng: &mut Rng, draws: &mut u64| loop {
        let x = uniform(rng, f.size(), draws);
        let pts = e.lift(x);
        if !pts.is_empty() {
            let j = uniform(rng, pts.len() as u64, draws) as usize;
            break e.mul(pts[j], inst.r);
        }
    };
    for _ in 0..16 {
        let a = uniform(&mut rng, inst.r.min(4096), &mut draws);
        let b = uniform(&mut rng, inst.r.min(4096), &mut draws);
        let torsion = offset(&mut rng, &mut draws);
        jumps.push((
            e.add(e.add(e.mul(inst.generator, a), e.mul(target, b)), torsion),
            a,
            b,
        ));
    }
    let (mut a, mut b) = (
        uniform(&mut rng, inst.r, &mut draws),
        uniform(&mut rng, inst.r, &mut draws),
    );
    let mut point = e.add(
        e.add(e.mul(inst.generator, a), e.mul(target, b)),
        offset(&mut rng, &mut draws),
    );
    phases.push(NativePhase {
        name: "walk_initialization".into(),
        counts: f.counts() - previous,
    });
    previous = f.counts();
    let mut recovered = None;
    let mut relations = 0;
    let mut trials = 0;
    let mut root_calls = 0;
    let mut oracle_counts = Counts::default();
    let mut seen = std::collections::BTreeSet::new();
    let mut repeated = 0;
    let mut walk_seen = std::collections::BTreeSet::new();
    let mut restarts = 0;
    for trial in 1..=trial_cap {
        trials = trial;
        if include_restart_targets && trial > 1 {
            // A fresh jump choice avoids the image bias of a fixed
            // point-partition map when the pair sumset is very small.
            let j = uniform(&mut rng, jumps.len() as u64, &mut draws) as usize;
            let jump = &jumps[j];
            point = e.add(point, jump.0);
            a = ring.add(a, jump.1);
            b = ring.add(b, jump.2);
        }
        if !walk_seen.insert(point) {
            restarts += 1;
            walk_seen.clear();
            a = uniform(&mut rng, inst.r, &mut draws);
            b = uniform(&mut rng, inst.r, &mut draws);
            point = e.add(
                e.add(e.mul(inst.generator, a), e.mul(target, b)),
                offset(&mut rng, &mut draws),
            );
        }
        if !include_restart_targets {
            let key = mix(point.x ^ point.y.rotate_left(31));
            let jump = &jumps[key as usize % jumps.len()];
            point = e.add(point, jump.0);
            a = ring.add(a, jump.1);
            b = ring.add(b, jump.2);
        }
        if point.inf {
            if b != 0 {
                recovered = Some(ring.mul(ring.neg(a), ring.inv(b)));
                break;
            }
            continue;
        }
        let before = f.counts();
        let pair = decompose(e, &base, point, oracle, inst.parameter, &mut root_calls);
        oracle_counts = oracle_counts + (f.counts() - before);
        if let Some(pair) = pair {
            relations += 1;
            if !seen.insert(point) {
                repeated += 1;
            }
            let mut row = vec![0; base.columns + 2];
            for p in pair {
                if let Some((column, sign)) = base.map[&p] {
                    row[column] = ring.add(row[column], if sign == 1 { 1 } else { ring.r - 1 });
                }
            }
            row[base.columns] = ring.neg(b);
            row[base.columns + 1] = a;
            recovered = matrix.insert(row);
            if recovered.is_some() {
                break;
            }
        }
    }
    phases.push(NativePhase {
        name: "decomposition_and_linear_algebra".into(),
        counts: f.counts() - previous,
    });
    let scalar = ring.counts.get();
    let mut native = BTreeMap::from([
        ("rng_draws_uncharged".into(), draws + base.draws),
        ("scalar_adds_uncharged".into(), scalar.add),
        ("scalar_muls_uncharged".into(), scalar.mul),
        ("scalar_inversions_uncharged".into(), scalar.inv),
        (
            "table_probes_uncharged".into(),
            trials + relations + base.probes.get(),
        ),
        ("decomposition_calls".into(), trials),
        ("root_calls".into(), root_calls),
        ("relations".into(), relations),
        ("linear_rank".into(), matrix.rank as u64),
        ("repeated_relation_targets".into(), repeated),
        ("random_restarts".into(), restarts),
        ("base_signed_points".into(), base.raw.len() as u64),
        ("base_columns".into(), base.columns as u64),
        ("oracle_muls".into(), oracle_counts.m),
        ("oracle_squares".into(), oracle_counts.s),
        ("oracle_inversions".into(), oracle_counts.i),
    ]);
    if oracle == "linearized_filtered" {
        native.insert("trace_filter_tests".into(), base.trace_tests.get());
        native.insert("trace_filter_skips".into(), base.trace_skips.get());
    }
    Outcome {
        recovered,
        exhausted: recovered.is_none(),
        phases,
        counts: f.counts(),
        native,
        detail: json!({"family":inst.family,"parameter":inst.parameter,"oracle":oracle,"random_base":random,"base_seed":base_seed,"raw_factor_base":base.raw,"projected_columns":base.columns,"r":inst.r,"h":inst.h,"root_call_definition":"one quadratic or S3 polynomial; decomposition calls reported separately"}),
    }
}

trait NativeGroup {
    type Element: Copy + Eq + Ord + std::hash::Hash;
    fn identity(&self) -> Self::Element;
    fn add(&self, a: Self::Element, b: Self::Element) -> Self::Element;
    fn neg(&self, a: Self::Element) -> Self::Element;
    fn hash(&self, a: Self::Element) -> u64;
    fn mul(&self, mut a: Self::Element, mut k: u64) -> Self::Element {
        let mut r = self.identity();
        while k > 0 {
            if k & 1 != 0 {
                r = self.add(r, a);
            }
            k >>= 1;
            if k > 0 {
                a = self.add(a, a);
            }
        }
        r
    }
}
impl NativeGroup for Elliptic {
    type Element = Point;
    fn identity(&self) -> Point {
        Point::O
    }
    fn add(&self, a: Point, b: Point) -> Point {
        Elliptic::add(self, a, b)
    }
    fn neg(&self, a: Point) -> Point {
        Elliptic::neg(self, a)
    }
    fn hash(&self, a: Point) -> u64 {
        mix(a.x ^ a.y.rotate_left(29) ^ u64::from(a.inf))
    }
}
impl NativeGroup for Jacobian {
    type Element = Divisor;
    fn identity(&self) -> Divisor {
        Divisor::O
    }
    fn add(&self, a: Divisor, b: Divisor) -> Divisor {
        Jacobian::add(self, a, b)
    }
    fn neg(&self, a: Divisor) -> Divisor {
        Jacobian::neg(self, a)
    }
    fn hash(&self, a: Divisor) -> u64 {
        mix(a.u[0]
            ^ a.u[1].rotate_left(19)
            ^ a.v[0].rotate_left(37)
            ^ a.v[1].rotate_left(7)
            ^ a.degree as u64)
    }
}
fn canonical<G: NativeGroup>(
    g: &G,
    point: G::Element,
    a: u64,
    b: u64,
    ring: &Ring,
    fold: bool,
) -> (G::Element, u64, u64, bool) {
    if fold {
        let negative = g.neg(point);
        if negative < point {
            return (negative, ring.neg(a), ring.neg(b), true);
        }
    }
    (point, a, b, false)
}
/// A complete r-adding collision search with an explicit table. Its memory
/// trade is reported alongside the large-prime graph. Negation uses a
/// one-step fruitless-two-cycle test and charged doubling/restart escapes.
#[allow(clippy::too_many_arguments)]
fn rho_native<G: NativeGroup>(
    g: &G,
    p: G::Element,
    q: G::Element,
    r: u64,
    seed: u64,
    cap: u64,
    fold: bool,
    counts: impl Fn() -> Counts,
) -> (Option<u64>, Vec<NativePhase>, BTreeMap<String, u64>) {
    let ring = Ring::new(r);
    let mut rng = Rng(seed);
    let mut draws = 0;
    let before = counts();
    let mut jumps = vec![];
    while jumps.len() < 16 {
        let a = uniform(&mut rng, r.min(4096), &mut draws);
        let b = uniform(&mut rng, r.min(4096), &mut draws);
        let x = g.add(g.mul(p, a), g.mul(q, b));
        if x != g.identity() {
            jumps.push((x, a, b));
        }
    }
    let (mut a, mut b) = (
        uniform(&mut rng, r, &mut draws),
        uniform(&mut rng, r, &mut draws),
    );
    let start = g.add(g.mul(p, a), g.mul(q, b));
    let (mut x, aa, bb, _) = canonical(g, start, a, b, &ring, fold);
    a = aa;
    b = bb;
    let mut phases = vec![NativePhase {
        name: "rho_initialization".into(),
        counts: counts() - before,
    }];
    let previous = counts();
    let mut seen = HashMap::new();
    let mut probes = 0;
    let mut hashes = 0;
    let mut steps = 0;
    let mut rejected = 0;
    let mut escapes = 0;
    let mut restarts = 0;
    let mut canon = 1;
    let mut recovered = None;
    let mut consecutive = 0;
    for _ in 0..cap {
        if x == g.identity() && b != 0 {
            recovered = Some(ring.mul(ring.neg(a), ring.inv(b)));
            break;
        }
        probes += 1;
        if let Some(&(old_a, old_b)) = seen.get(&x) {
            let delta_b = ring.sub(b, old_b);
            let delta_a = ring.sub(old_a, a);
            if delta_b != 0 {
                recovered = Some(ring.mul(delta_a, ring.inv(delta_b)));
                break;
            }
            assert_eq!(delta_a, 0);
            escapes += 1;
            consecutive += 1;
            if consecutive >= 4 || x == g.identity() {
                restarts += 1;
                consecutive = 0;
                a = uniform(&mut rng, r, &mut draws);
                b = uniform(&mut rng, r, &mut draws);
                x = g.add(g.mul(p, a), g.mul(q, b));
            } else {
                x = g.add(x, x);
                a = ring.add(a, a);
                b = ring.add(b, b);
            }
            let (xx, aa, bb, _) = canonical(g, x, a, b, &ring, fold);
            x = xx;
            a = aa;
            b = bb;
            canon += 1;
            continue;
        }
        consecutive = 0;
        seen.insert(x, (a, b));
        steps += 1;
        hashes += 1;
        let index = g.hash(x) as usize % 16;
        let mut next = None;
        for offset in 0..16 {
            let j = (index + offset) % 16;
            let jump = &jumps[j];
            let point = g.add(x, jump.0);
            let ca = ring.add(a, jump.1);
            let cb = ring.add(b, jump.2);
            let candidate = canonical(g, point, ca, cb, &ring, fold);
            canon += 1;
            if fold && candidate.3 {
                hashes += 1;
                if g.hash(candidate.0) as usize % 16 == j {
                    rejected += 1;
                    continue;
                }
            }
            next = Some(candidate);
            break;
        }
        if let Some((point, aa, bb, _)) = next {
            x = point;
            a = aa;
            b = bb;
        } else {
            escapes += 1;
            let point = g.add(x, x);
            let candidate = canonical(g, point, ring.add(a, a), ring.add(b, b), &ring, fold);
            x = candidate.0;
            a = candidate.1;
            b = candidate.2;
            canon += 1;
        }
    }
    phases.push(NativePhase {
        name: "rho_search".into(),
        counts: counts() - previous,
    });
    let scalar = ring.counts.get();
    let native = BTreeMap::from([
        ("walk_steps".into(), steps),
        ("lookahead_rejected_additions".into(), rejected),
        ("cycle_escape_doublings".into(), escapes),
        ("random_restarts".into(), restarts),
        ("hash_probes_uncharged".into(), probes),
        ("partition_hashes_uncharged".into(), hashes),
        ("canonicalisations_uncharged".into(), canon),
        ("rng_draws_uncharged".into(), draws),
        ("scalar_adds_uncharged".into(), scalar.add),
        ("scalar_muls_uncharged".into(), scalar.mul),
        ("scalar_inversions_uncharged".into(), scalar.inv),
        ("stored_collision_points".into(), seen.len() as u64),
        (
            "logical_collision_table_bytes".into(),
            (seen.capacity() * (std::mem::size_of::<G::Element>() + 16 + 1)) as u64,
        ),
    ]);
    (recovered, phases, native)
}
pub fn solve_rho(
    inst: &StudyInstance,
    target: Point,
    seed: u64,
    cap: u64,
    fold: bool,
    transferred: bool,
) -> Outcome {
    let fresh = inst.fresh();
    let inst = &fresh;
    if transferred {
        assert_eq!(inst.family, "scholten");
        let s = Scholten::new(inst.curve.f.p);
        let source = s.short_curve();
        assert_eq!(source.c, inst.curve.c);
        let counts = || s.e.f.counts() + s.j.f.counts();
        let mut phases = vec![NativePhase {
            name: "destination_construction".into(),
            counts: counts(),
        }];
        let before = counts();
        let p = s.tau(s.from_short(inst.generator));
        let q = s.tau(s.from_short(target));
        phases.push(NativePhase {
            name: "transfer".into(),
            counts: counts() - before,
        });
        let (recovered, mut walk, native) = rho_native(&s.j, p, q, inst.r, seed, cap, fold, counts);
        phases.append(&mut walk);
        Outcome {
            recovered,
            exhausted: recovered.is_none(),
            phases,
            counts: counts(),
            native,
            detail: json!({"method":"memoized_r_adding_rho","negation":fold,"transferred":true,"collision_table_is_charged":true,"r":inst.r,"p":inst.curve.f.p}),
        }
    } else {
        let e = &inst.curve;
        e.f.reset();
        let (recovered, phases, native) =
            rho_native(e, inst.generator, target, inst.r, seed, cap, fold, || {
                e.f.counts()
            });
        Outcome {
            recovered,
            exhausted: recovered.is_none(),
            phases,
            counts: e.f.counts(),
            native,
            detail: json!({"method":"memoized_r_adding_rho","negation":fold,"transferred":false,"collision_table_is_charged":true,"r":inst.r,"p":inst.curve.f.p}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn small(e: Elliptic, family: &str, parameter: u64) -> Option<StudyInstance> {
        let order = exhaustive_order(&e);
        let &(r, exp) = factor(order).last()?;
        if r < 5 || exp != 1 || (order / r).is_multiple_of(r) {
            return None;
        }
        let h = order / r;
        let generator = (0..e.f.size())
            .flat_map(|x| e.lift(x))
            .map(|p| e.mul(p, h))
            .find(|p| !p.inf)?;
        let i = StudyInstance {
            curve: e,
            order,
            r,
            h,
            generator,
            family: family.into(),
            parameter,
            evidence: json!({"test":true}),
        };
        i.validate().ok()?;
        Some(i)
    }
    #[test]
    fn binary_linearized_oracle_and_all_solvers() {
        let inst = (2..40)
            .find_map(|b| {
                small(
                    Elliptic {
                        f: Field::new(2, 6, binary_modulus(6)),
                        c: [b, 0, 3, 1],
                        binary: true,
                    },
                    "subfield",
                    3,
                )
            })
            .unwrap();
        let base = build_factor_base(&inst, false, 19);
        let mut filtered = base.clone();
        filtered.prepare_trace_filter(&inst.curve.f);
        let mut roots = 0;
        for x in 0..inst.curve.f.size() {
            for p in inst.curve.lift(x) {
                assert_eq!(
                    decompose(&inst.curve, &base, p, "subtract", 3, &mut roots).is_some(),
                    decompose(&inst.curve, &base, p, "linearized", 3, &mut roots).is_some(),
                    "point {p:?}"
                );
                assert_eq!(
                    decompose(&inst.curve, &base, p, "subtract", 3, &mut roots).is_some(),
                    decompose(
                        &inst.curve,
                        &filtered,
                        p,
                        "linearized_filtered",
                        3,
                        &mut roots
                    )
                    .is_some()
                );
            }
        }
        assert_eq!(filtered.trace_skips.get(), inst.order / 2);
        for k in 1..inst.r.min(12) {
            let q = inst.curve.mul(inst.generator, k);
            for fold in [false, true] {
                assert_eq!(
                    solve_rho(&inst, q, 17 + k, 10000, fold, false).recovered,
                    Some(k)
                );
            }
            assert_eq!(
                solve_subfield_or_pkm(&inst, q, 17 + k, 10000, "linearized", false, 19).recovered,
                Some(k)
            );
            let plain = solve_subfield_or_pkm_v2(&inst, q, 17 + k, 10000, "linearized", false, 19);
            let screened =
                solve_subfield_or_pkm_v2(&inst, q, 17 + k, 10000, "linearized_filtered", false, 19);
            assert_eq!(screened.recovered, Some(k));
            assert_eq!(
                screened.native["decomposition_calls"],
                plain.native["decomposition_calls"]
            );
            assert_eq!(screened.native["relations"], plain.native["relations"]);
        }
    }
    #[test]
    fn prime_tower_s3_and_diagnostic_table_separation() {
        let inst = (65..1000)
            .filter(|p| p % 4 == 1 && is_prime(*p))
            .find_map(|p| {
                small(
                    Elliptic {
                        f: Field::new(p, 1, 0),
                        c: [3, 2, 0, 1],
                        binary: false,
                    },
                    "pkm_kummer",
                    2,
                )
            })
            .unwrap();
        let base = build_factor_base(&inst, false, 19);
        let mut roots = 0;
        for x in 0..inst.curve.f.size() {
            for p in inst.curve.lift(x) {
                assert_eq!(
                    decompose(&inst.curve, &base, p, "subtract", 2, &mut roots).is_some(),
                    decompose(&inst.curve, &base, p, "semaev", 2, &mut roots).is_some()
                );
            }
        }
        for k in 1..inst.r.min(5) {
            let q = inst.curve.mul(inst.generator, k);
            let out = solve_subfield_or_pkm(&inst, q, k + 5, 10000, "semaev", false, 19);
            assert_eq!(
                out.recovered,
                Some(k),
                "inst={:?} detail={} native={:?}",
                inst,
                out.detail,
                out.native
            );
        }
        let mut f = inst.curve.f.fresh();
        f.diagnostic_tables();
        assert!(f.validate().is_err());
        let raw = f.fresh();
        for a in 0..f.size() {
            assert_eq!(f.sqrt(a).is_some(), raw.sqrt(a).is_some());
        }
        assert!(raw.validate().is_ok());
    }
    #[test]
    fn quadratic_fields_and_transfer() {
        for p in [19, 23, 31, 43, 59, 67, 83, 103] {
            let s = Scholten::new(p);
            let f = &s.e.f;
            for a in 1..f.size() {
                assert_eq!(f.mul(a, f.inv(a)), 1);
                if let Some(x) = f.sqrt(a) {
                    assert_eq!(f.sq(x), a);
                }
            }
            let mut points = vec![Point::O];
            for x in 0..f.size() {
                points.extend(s.e.lift(x));
            }
            for &p in &points {
                let d = s.tau(p);
                assert_eq!(s.norm(d), s.e.add(p, p), "norm-tau at p={}", s.p);
                assert_eq!(s.from_short(s.to_short(p)), p);
                assert!(s.short_curve().valid(s.to_short(p)));
            }
            for k in 0..100 {
                let p = points[k % points.len()];
                let q = points[(k * 13 + 7) % points.len()];
                assert_eq!(s.tau(s.e.add(p, q)), s.j.add(s.tau(p), s.tau(q)));
            }
        }
    }
    #[test]
    fn signed_graph_does_not_solve_a_negative_cycle() {
        let ring = Ring::new(101);
        let mut graph = SignedGraph::new(ring.clone());
        let x = graph.node();
        let y = graph.node();
        let z = graph.node();
        assert_eq!(graph.edge(x, 1, y, 1, 7, 1), None);
        assert_eq!(graph.edge(y, 1, z, 1, 11, 2), None);
        assert_eq!(graph.edge(z, 1, x, 1, 10, 3), None);
        assert_eq!(graph.negative_cycles, 1);
        for seed in 0..100 {
            let mut graph = SignedGraph::new(ring.clone());
            let nodes: Vec<_> = (0..20).map(|_| graph.node()).collect();
            let mut rng = Rng(seed);
            let k = 37;
            let logs: Vec<_> = (0..20).map(|_| rng.next() % 101).collect();
            let mut done = false;
            for _ in 0..200 {
                let u = rng.next() as usize % 20;
                let v = rng.next() as usize % 20;
                let su = if rng.next() & 1 == 0 { 1 } else { -1 };
                let sv = if rng.next() & 1 == 0 { 1 } else { -1 };
                let b = rng.next() % 101;
                let a = ring.sub(
                    ring.add(ring.signed(su, logs[u]), ring.signed(sv, logs[v])),
                    ring.mul(b, k),
                );
                if let Some(got) = graph.edge(nodes[u], su, nodes[v], sv, a, b) {
                    assert_eq!(got, k);
                    done = true;
                    break;
                }
            }
            assert!(done);
        }
    }
    #[test]
    fn real_genus2_end_to_end() {
        let inst = (199..1000)
            .filter(|p| p % 4 == 3 && is_prime(*p))
            .find_map(|p| scholten_instance(p, 123))
            .unwrap();
        for k in [1, 3, 7, 13] {
            let q = inst.curve.mul(inst.generator, k);
            for mode in [false, true] {
                let out = solve_genus2(&inst, q, 19 + k, 100 * inst.curve.f.p + 10000, mode, true);
                assert_eq!(out.recovered, Some(k), "{}", out.detail);
            }
        }
    }
    #[test]
    fn genus2_v2_restarts_unsplit_cycles_without_using_collisions() {
        let inst = scholten_instance(311, 20261018 + 311).unwrap();
        for k in 1..=120 {
            let target = inst.curve.mul(inst.generator, k);
            let out = solve_genus2_v2(&inst, target, 20261018 + k, 47900, false, true);
            assert_eq!(out.recovered, Some(k), "{} {:?}", out.detail, out.native);
        }
    }
    #[test]
    fn decomposition_v2_samples_tiny_sumsets() {
        let inst = small(
            Elliptic {
                f: Field::new(16529, 1, 0),
                c: [3, 2, 0, 1],
                binary: false,
            },
            "pkm_kummer",
            2,
        )
        .unwrap();
        for k in 1..inst.r.min(17) {
            let q = inst.curve.mul(inst.generator, k);
            for random in [false, true] {
                let out = solve_subfield_or_pkm_v2(
                    &inst,
                    q,
                    20261018 + k,
                    100 * 16529 + 10000,
                    "semaev",
                    random,
                    20261018,
                );
                assert_eq!(out.recovered, Some(k), "{} {:?}", out.detail, out.native);
            }
        }
    }
    #[test]
    fn fixtures_reject_noncanonical_coordinates_and_wrong_family_parameters() {
        let i = small(
            Elliptic {
                f: Field::new(73, 1, 0),
                c: [3, 2, 0, 1],
                binary: false,
            },
            "pkm_kummer",
            2,
        )
        .unwrap();
        let mut bad = i.clone();
        bad.generator.x = bad.curve.f.size();
        assert!(bad.validate().is_err());
        let mut bad = i;
        bad.curve.c[1] = bad.curve.f.size();
        assert!(bad.validate().is_err());
        let mut b = (2..40)
            .find_map(|b| {
                small(
                    Elliptic {
                        f: Field::new(2, 6, binary_modulus(6)),
                        c: [b, 0, 3, 1],
                        binary: true,
                    },
                    "subfield",
                    3,
                )
            })
            .unwrap();
        b.parameter = u64::MAX;
        assert!(b.validate().is_err());
        let mut s = small(
            Elliptic {
                f: Field::new(7, 2, 1),
                c: [1, 1, 0, 1],
                binary: false,
            },
            "subfield",
            1,
        )
        .unwrap();
        s.family = "scholten".into();
        assert!(s.validate().is_err());
    }
}
