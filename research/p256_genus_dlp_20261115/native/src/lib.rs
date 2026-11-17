//! Native, variable-time correctness checker. No performance or protocol API.
//! Coefficients are ascending; even-degree Jacobian arithmetic is excluded.
use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_traits::{One, ToPrimitive, Zero};
use serde_json::{json, Value};
use std::collections::HashMap;

// Reuse the repository's pre-existing checker, rather than writing a new verifier.
pub mod hash {
    pub mod sha256 {
        pub fn sha256(bytes: &[u8]) -> [u8; 32] {
            use sha2::{Digest, Sha256};
            Sha256::digest(bytes).into()
        }
    }
}
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
pub mod identity_certificate;

pub type Point = Option<(BigUint, BigUint)>;
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Poly(pub Vec<BigUint>);
pub type Divisor = (Poly, Poly);

pub fn hex(s: &str) -> BigUint {
    BigUint::parse_bytes(s.as_bytes(), 16).unwrap()
}
pub fn parameters() -> (BigUint, BigUint, BigUint, Point) {
    (
        hex("FFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFF"),
        hex("5AC635D8AA3A93E7B3EBBD55769886BC651D06B0CC53B0F63BCE3C3E27D2604B"),
        hex("FFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551"),
        Some((
            hex("6B17D1F2E12C4247F8BCE6E563A440F277037D812DEB33A0F4A13945D898C296"),
            hex("4FE342E2FE1A7F9B8EE7EB4A7C0F9E162BCE33576B315ECECBB6406837BF51F5"),
        )),
    )
}
pub fn sub(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
    (a + p - (b % p)) % p
}
pub fn inv(a: &BigUint, p: &BigUint) -> BigUint {
    let modulus = BigInt::from(p.clone());
    let e = BigInt::from(a.clone()).extended_gcd(&modulus);
    assert_eq!(e.gcd, BigInt::one(), "noninvertible coefficient");
    e.x.mod_floor(&modulus).to_biguint().unwrap()
}
impl Poly {
    pub fn new(mut c: Vec<BigUint>, p: &BigUint) -> Self {
        for v in &mut c {
            *v %= p;
        }
        while c.last().is_some_and(Zero::is_zero) {
            c.pop();
        }
        Self(c)
    }
    pub fn small(c: &[i64], p: &BigUint) -> Self {
        Self::new(
            c.iter()
                .map(|x| {
                    if *x >= 0 {
                        BigUint::from(*x as u64)
                    } else {
                        sub(&BigUint::zero(), &BigUint::from(x.unsigned_abs()), p)
                    }
                })
                .collect(),
            p,
        )
    }
    pub fn one() -> Self {
        Self(vec![BigUint::one()])
    }
    pub fn zero() -> Self {
        Self(vec![])
    }
    pub fn monomial(i: usize) -> Self {
        let mut c = vec![BigUint::zero(); i + 1];
        c[i] = BigUint::one();
        Self(c)
    }
    pub fn degree(&self) -> Option<usize> {
        self.0.len().checked_sub(1)
    }
    pub fn coeff(&self, i: usize) -> BigUint {
        self.0.get(i).cloned().unwrap_or_default()
    }
    pub fn add(&self, b: &Self, p: &BigUint) -> Self {
        Self::new(
            (0..self.0.len().max(b.0.len()))
                .map(|i| (self.coeff(i) + b.coeff(i)) % p)
                .collect(),
            p,
        )
    }
    pub fn sub(&self, b: &Self, p: &BigUint) -> Self {
        Self::new(
            (0..self.0.len().max(b.0.len()))
                .map(|i| sub(&self.coeff(i), &b.coeff(i), p))
                .collect(),
            p,
        )
    }
    pub fn neg(&self, p: &BigUint) -> Self {
        Self::zero().sub(self, p)
    }
    pub fn scale(&self, b: &BigUint, p: &BigUint) -> Self {
        Self::new(self.0.iter().map(|a| a * b % p).collect(), p)
    }
    pub fn mul(&self, b: &Self, p: &BigUint) -> Self {
        if self.0.is_empty() || b.0.is_empty() {
            return Self::zero();
        }
        let mut c = vec![BigUint::zero(); self.0.len() + b.0.len() - 1];
        for (i, a) in self.0.iter().enumerate() {
            for (j, b) in b.0.iter().enumerate() {
                c[i + j] = (&c[i + j] + a * b) % p;
            }
        }
        Self::new(c, p)
    }
    pub fn pow(&self, n: usize, p: &BigUint) -> Self {
        (0..n).fold(Self::one(), |a, _| a.mul(self, p))
    }
    pub fn monic(&self, p: &BigUint) -> Self {
        self.scale(&inv(self.0.last().unwrap(), p), p)
    }
    pub fn divrem(&self, b: &Self, p: &BigUint) -> (Self, Self) {
        let db = b.degree().expect("division by zero");
        let mut r = self.clone();
        let mut q = vec![BigUint::zero(); self.0.len()];
        let inverse = inv(b.0.last().unwrap(), p);
        while let Some(dr) = r.degree() {
            if dr < db {
                break;
            }
            let shift = dr - db;
            let c = r.0.last().unwrap() * &inverse % p;
            q[shift] = (&q[shift] + &c) % p;
            r = r.sub(&b.scale(&c, p).mul(&Self::monomial(shift), p), p);
        }
        (Self::new(q, p), r)
    }
    pub fn rem(&self, b: &Self, p: &BigUint) -> Self {
        self.divrem(b, p).1
    }
    pub fn exact(&self, b: &Self, p: &BigUint) -> Self {
        let (q, r) = self.divrem(b, p);
        assert_eq!(r, Self::zero());
        q
    }
    pub fn xgcd(&self, b: &Self, p: &BigUint) -> (Self, Self, Self) {
        let (mut a, mut b) = (self.clone(), b.clone());
        let (mut s, mut ss, mut t, mut tt) = (Self::one(), Self::zero(), Self::zero(), Self::one());
        while !b.0.is_empty() {
            let (q, r) = a.divrem(&b, p);
            a = b;
            b = r;
            let ns = s.sub(&q.mul(&ss, p), p);
            s = ss;
            ss = ns;
            let nt = t.sub(&q.mul(&tt, p), p);
            t = tt;
            tt = nt;
        }
        let k = inv(a.0.last().unwrap(), p);
        (s.scale(&k, p), t.scale(&k, p), a.scale(&k, p))
    }
    pub fn derivative(&self, p: &BigUint) -> Self {
        Self::new(
            self.0
                .iter()
                .enumerate()
                .skip(1)
                .map(|(i, c)| c * BigUint::from(i) % p)
                .collect(),
            p,
        )
    }
}
pub fn degree(g: usize) -> usize {
    match g {
        2 | 3 => 2,
        4 => 3,
        5 => 4,
        _ => panic!("unsupported genus"),
    }
}
pub fn equation(g: usize, b: &BigUint, p: &BigUint) -> Poly {
    let mut c = vec![
        BigUint::zero();
        match g {
            2 => 7,
            3 => 8,
            4 => 10,
            5 => 13,
            _ => panic!("unsupported genus"),
        }
    ];
    if g == 3 {
        c[1] = BigUint::one();
        c[4] = b.clone();
        c[7] = BigUint::one();
    } else {
        c[0] = b.clone();
        c[degree(g)] = sub(&BigUint::zero(), &BigUint::from(3u32), p);
        *c.last_mut().unwrap() = BigUint::one();
    }
    Poly::new(c, p)
}
#[derive(Clone)]
pub struct Elliptic {
    pub p: BigUint,
    pub b: BigUint,
}
impl Elliptic {
    pub fn rhs(&self, x: &BigUint) -> BigUint {
        sub(&(x * x * x + &self.b), &(x * BigUint::from(3u32)), &self.p)
    }
    pub fn contains(&self, a: &Point) -> bool {
        a.as_ref()
            .is_none_or(|(x, y)| y * y % &self.p == self.rhs(x))
    }
    pub fn neg(&self, a: &Point) -> Point {
        a.as_ref()
            .map(|(x, y)| (x.clone(), sub(&BigUint::zero(), y, &self.p)))
    }
    pub fn add(&self, a: &Point, b: &Point) -> Point {
        let (x, y) = match a {
            None => return b.clone(),
            Some(a) => a,
        };
        let (u, v) = match b {
            None => return a.clone(),
            Some(b) => b,
        };
        let p = &self.p;
        if x == u && (y + v) % p == BigUint::zero() {
            return None;
        }
        let slope = if x == u {
            sub(&(BigUint::from(3u32) * x * x), &BigUint::from(3u32), p)
                * inv(&(BigUint::from(2u32) * y % p), p)
                % p
        } else {
            sub(v, y, p) * inv(&sub(u, x, p), p) % p
        };
        let xx = sub(&sub(&(&slope * &slope), x, p), u, p);
        let yy = sub(&(&slope * sub(x, &xx, p)), y, p);
        Some((xx, yy))
    }
    pub fn mul(&self, n: &BigUint, a: &Point) -> Point {
        let (mut n, mut a, mut r) = (n.clone(), a.clone(), None);
        while !n.is_zero() {
            if n.bit(0) {
                r = self.add(&r, &a);
            }
            a = self.add(&a, &a);
            n >>= 1;
        }
        r
    }
}
pub fn transfer(g: usize, a: &Point, p: &BigUint) -> Divisor {
    assert!(g == 3 || g == 4, "two-infinity arithmetic is unsupported");
    let Some((x, y)) = a else {
        return (Poly::one(), Poly::zero());
    };
    if g == 3 {
        (
            Poly::new(
                vec![
                    BigUint::zero(),
                    BigUint::one(),
                    sub(&BigUint::zero(), x, p),
                    BigUint::one(),
                ],
                p,
            ),
            Poly::new(vec![BigUint::zero(), BigUint::zero(), y.clone()], p),
        )
    } else {
        (
            Poly::new(
                vec![
                    sub(&BigUint::zero(), x, p),
                    BigUint::zero(),
                    BigUint::zero(),
                    BigUint::one(),
                ],
                p,
            ),
            Poly::new(vec![y.clone()], p),
        )
    }
}
pub fn decode(g: usize, a: &Divisor, p: &BigUint) -> Point {
    assert!(g == 3 || g == 4);
    if a == &(Poly::one(), Poly::zero()) {
        return None;
    }
    let i = if g == 3 { 2 } else { 0 };
    let out = Some((sub(&BigUint::zero(), &a.0.coeff(i), p), a.1.coeff(i)));
    assert_eq!(
        transfer(g, &out, p),
        *a,
        "divisor is outside the explicit image"
    );
    out
}
pub struct Jacobian {
    pub p: BigUint,
    pub f: Poly,
    pub g: usize,
}
impl Jacobian {
    pub fn new(g: usize, b: &BigUint, p: &BigUint) -> Self {
        assert!(g == 3 || g == 4, "two-infinity arithmetic is unsupported");
        let f = equation(g, b, p);
        assert_eq!(f.xgcd(&f.derivative(p), p).2, Poly::one());
        Self { p: p.clone(), f, g }
    }
    pub fn zero(&self) -> Divisor {
        (Poly::one(), Poly::zero())
    }
    pub fn valid(&self, a: &Divisor) -> bool {
        let (u, v) = a;
        u.0.last() == Some(&BigUint::one())
            && u.degree().unwrap() <= self.g
            && (v.0.is_empty() || v.0.len() < u.0.len())
            && self.f.sub(&v.mul(v, &self.p), &self.p).rem(u, &self.p) == Poly::zero()
    }
    pub fn neg(&self, a: &Divisor) -> Divisor {
        (a.0.clone(), a.1.neg(&self.p).rem(&a.0, &self.p))
    }
    pub fn add(&self, a: &Divisor, b: &Divisor) -> Divisor {
        let p = &self.p;
        let (u1, v1) = a;
        let (u2, v2) = b;
        let (e1, e2, d1) = u1.xgcd(u2, p);
        let (c1, c2, d) = d1.xgcd(&v1.add(v2, p), p);
        let s1 = c1.mul(&e1, p);
        let s2 = c1.mul(&e2, p);
        let mut u = u1.mul(u2, p).exact(&d.mul(&d, p), p).monic(p);
        let numerator = s1
            .mul(u1, p)
            .mul(v2, p)
            .add(&s2.mul(u2, p).mul(v1, p), p)
            .add(&c2.mul(&v1.mul(v2, p).add(&self.f, p), p), p);
        let mut v = numerator.exact(&d, p).rem(&u, p);
        while u.degree().unwrap() > self.g {
            u = self.f.sub(&v.mul(&v, p), p).exact(&u, p).monic(p);
            v = v.neg(p).rem(&u, p);
        }
        (u, v)
    }
    pub fn mul(&self, n: &BigUint, a: &Divisor) -> Divisor {
        let (mut n, mut a, mut r) = (n.clone(), a.clone(), self.zero());
        while !n.is_zero() {
            if n.bit(0) {
                r = self.add(&r, &a);
            }
            a = self.add(&a, &a);
            n >>= 1;
        }
        r
    }
}
pub fn verify_actual() -> Value {
    let (p, b, n, generator) = parameters();
    let e = Elliptic {
        p: p.clone(),
        b: b.clone(),
    };
    assert!(e.contains(&generator));
    assert_eq!(e.mul(&n, &generator), None);
    let mut covers = vec![];
    for g in 2..=5 {
        let f = equation(g, &b, &p);
        assert_eq!(f.xgcd(&f.derivative(&p), &p).2, Poly::one());
        assert_eq!((f.degree().unwrap() - 1) / 2, g);
        let t = Poly::monomial(1);
        let t2 = Poly::monomial(2);
        let bpoly = Poly::new(vec![b.clone()], &p);
        let rhs = if g == 3 {
            let k = t2.add(&Poly::one(), &p);
            t.mul(&k.pow(3, &p), &p)
                .sub(&Poly::monomial(3).mul(&k, &p).scale(&3u32.into(), &p), &p)
                .add(&Poly::monomial(4).scale(&b, &p), &p)
        } else {
            let x = Poly::monomial(degree(g));
            x.pow(3, &p)
                .sub(&x.scale(&3u32.into(), &p), &p)
                .add(&bpoly, &p)
        };
        assert_eq!(f, rhs);
        assert_eq!(n.gcd(&BigUint::from(degree(g))), BigUint::one());
        for k in [1u32, 2, 17] {
            let Some((x, y)) = e.mul(&k.into(), &generator) else {
                panic!()
            };
            let (u, v) = if g == 3 {
                (
                    Poly::new(
                        vec![1u32.into(), sub(&BigUint::zero(), &x, &p), 1u32.into()],
                        &p,
                    ),
                    Poly::new(vec![0u32.into(), 0u32.into(), y], &p),
                )
            } else {
                (
                    Poly::monomial(degree(g)).sub(&Poly::new(vec![x], &p), &p),
                    Poly::new(vec![y], &p),
                )
            };
            assert_eq!(f.sub(&v.mul(&v, &p), &p).rem(&u, &p), Poly::zero());
        }
        covers.push(
            json!({"genus":g,"cover_degree":degree(g),"squarefree":true,"map_identity":true}),
        );
    }
    for g in [3, 4] {
        let j = Jacobian::new(g, &b, &p);
        let a = transfer(g, &generator, &p);
        assert!(j.valid(&a));
        assert_ne!(a, j.zero());
        assert_eq!(decode(g, &a, &p), generator);
        assert_eq!(j.mul(&n, &a), j.zero());
        for k in [2u32, 3, 17] {
            let want = e.mul(&k.into(), &generator);
            let got = j.mul(&k.into(), &a);
            assert_eq!(got, transfer(g, &want, &p));
            assert_eq!(decode(g, &got, &p), want);
        }
    }
    json!({"schema":"p256-genus-native-check/v1","standing":"producer self-check; not independently signed",
      "field_bits":p.bits(),"subgroup_order_hex":format!("0x{n:x}"),"covers":covers,
      "odd_degree_jacobian_checks":{"genera":[3,4],"order_n":true,"scalar_checks":[2,3,17],"direct_inverse":true},
      "two_infinity_jacobian_arithmetic":"unsupported"})
}
pub fn toy_correctness() -> Value {
    let mut rows = vec![];
    for (p, n, scalar) in [
        (1009u32, 109u32, 43u32),
        (4001, 4111, 1377),
        (65537, 7229, 2416),
    ] {
        let pp = BigUint::from(p);
        let b = parameters().1 % &pp;
        let e = Elliptic {
            p: pp.clone(),
            b: b.clone(),
        };
        let total = (0..p).fold(1u32, |s, x| {
            let r = e.rhs(&x.into());
            s + if r.is_zero() {
                1
            } else if r.modpow(&((p - 1) / 2).into(), &pp) == BigUint::one() {
                2
            } else {
                0
            }
        });
        assert_eq!(total % n, 0);
        let cofactor = BigUint::from(total / n);
        let mut generator = None;
        'find: for x in 0..p {
            let rhs = e.rhs(&x.into());
            for y in 0..p {
                if (u64::from(y) * u64::from(y) % u64::from(p)) == rhs.to_u64().unwrap() {
                    generator = e.mul(&cofactor, &Some((x.into(), y.into())));
                    if generator.is_some() {
                        break 'find;
                    }
                    break;
                }
            }
        }
        assert!(generator.is_some());
        assert_eq!(e.mul(&n.into(), &generator), None);
        let target = e.mul(&scalar.into(), &generator);
        for g in [3, 4] {
            let j = Jacobian::new(g, &b, &pp);
            let a = transfer(g, &generator, &pp);
            let target = transfer(g, &target, &pp);
            assert!(j.valid(&a));
            assert_eq!(j.mul(&n.into(), &a), j.zero());
            assert_eq!(j.mul(&scalar.into(), &a), target);
            let m = (f64::from(n).sqrt() as u32) + 1;
            let mut table = HashMap::new();
            let mut step = j.zero();
            for k in 0..m {
                table.entry(step.clone()).or_insert(k);
                step = j.add(&step, &a);
            }
            let stride = j.neg(&j.mul(&m.into(), &a));
            let mut step = target.clone();
            let mut recovered = None;
            for i in 0..=m {
                if let Some(k) = table.get(&step) {
                    let k = (i * m + k) % n;
                    if j.mul(&k.into(), &a) == target {
                        recovered = Some(k);
                        break;
                    }
                }
                step = j.add(&step, &stride);
            }
            assert_eq!(recovered, Some(scalar));
            rows.push(json!({"p":p,"genus":g,"subgroup_order":n,"recovered_known_scalar":scalar}));
        }
    }
    json!({"scope":"native correctness fixtures; no timing or performance claim","cases":rows})
}

struct TinyField {
    p: usize,
    k: usize,
    size: usize,
    modulus: Vec<usize>,
    digits: Vec<Vec<usize>>,
}
impl TinyField {
    fn digits(mut n: usize, p: usize, k: usize) -> Vec<usize> {
        (0..k)
            .map(|_| {
                let v = n % p;
                n /= p;
                v
            })
            .collect()
    }
    fn new(p: usize, k: usize) -> Self {
        let size = p.pow(k as u32);
        let pp = BigUint::from(p);
        let modulus = if k == 1 {
            vec![0, 1]
        } else {
            (1..size)
                .find_map(|n| {
                    let mut c = Self::digits(n, p, k);
                    if c[0] == 0 {
                        return None;
                    }
                    c.push(1);
                    let f = Poly::new(c.iter().map(|x| BigUint::from(*x)).collect(), &pp);
                    for d in 1..=k / 2 {
                        for i in 0..p.pow(d as u32) {
                            let mut dd = Self::digits(i, p, d);
                            dd.push(1);
                            let divisor =
                                Poly::new(dd.into_iter().map(BigUint::from).collect(), &pp);
                            if f.rem(&divisor, &pp) == Poly::zero() {
                                return None;
                            }
                        }
                    }
                    Some(c)
                })
                .expect("irreducible polynomial")
        };
        let digits = (0..size).map(|n| Self::digits(n, p, k)).collect();
        Self {
            p,
            k,
            size,
            modulus,
            digits,
        }
    }
    fn add(&self, a: usize, b: usize) -> usize {
        self.digits[a]
            .iter()
            .zip(&self.digits[b])
            .enumerate()
            .map(|(i, (a, b))| (a + b) % self.p * self.p.pow(i as u32))
            .sum()
    }
    fn mul(&self, a: usize, b: usize) -> usize {
        if self.k == 1 {
            return a * b % self.p;
        }
        let mut out = vec![0usize; 2 * self.k - 1];
        for i in 0..self.k {
            for j in 0..self.k {
                out[i + j] = (out[i + j] + self.digits[a][i] * self.digits[b][j]) % self.p;
            }
        }
        for i in (self.k..out.len()).rev() {
            let c = out[i];
            for j in 0..self.k {
                out[i - self.k + j] =
                    (out[i - self.k + j] + self.p - c * self.modulus[j] % self.p) % self.p;
            }
        }
        out[..self.k]
            .iter()
            .enumerate()
            .map(|(i, x)| x * self.p.pow(i as u32))
            .sum()
    }
    fn count(&self, c: &[i64]) -> i64 {
        let mut roots = vec![0i64; self.size];
        for y in 0..self.size {
            roots[self.mul(y, y)] += 1;
        }
        let mut total = if (c.len() - 1).is_multiple_of(2) {
            2
        } else {
            1
        };
        for x in 0..self.size {
            let mut v = 0;
            for c in c.iter().rev() {
                v = self.add(self.mul(v, x), c.rem_euclid(self.p as i64) as usize);
            }
            total += roots[v];
        }
        total
    }
}
fn frobenius(q: i64, counts: &[i64]) -> Vec<i64> {
    let g = counts.len();
    let sums: Vec<i64> = (1..=g)
        .map(|k| q.pow(k as u32) + 1 - counts[k - 1])
        .collect();
    let mut c = vec![1i64];
    for k in 1..=g {
        let numerator = -(1..=k).map(|i| c[k - i] * sums[i - 1]).sum::<i64>();
        assert_eq!(numerator % (k as i64), 0);
        c.push(numerator / k as i64);
    }
    for k in g + 1..=2 * g {
        c.push(c[2 * g - k] * q.pow((k - g) as u32));
    }
    c
}
pub fn toy_zeta() -> Value {
    let names = ["C2", "C3", "C4", "C5", "E", "F2", "F3", "H5"];
    let mut polynomials = vec![];
    for g in 2..=5 {
        polynomials.push(
            equation(g, &1u32.into(), &7u32.into())
                .0
                .into_iter()
                .map(|x| x.to_i64().unwrap())
                .collect::<Vec<_>>(),
        );
    }
    polynomials.extend([
        vec![1, -3, 0, 1],
        vec![1, 0, -3, 1],
        vec![0, 1, 1, 1],
        vec![0, 1, 0, -3, 0, 0, 0, 1],
    ]);
    let genera: Vec<usize> = polynomials.iter().map(|f| (f.len() - 2) / 2).collect();
    let mut counts = vec![vec![]; 8];
    for k in 1..=5 {
        let field = TinyField::new(7, k);
        for i in 0..8 {
            if k <= genera[i] {
                counts[i].push(field.count(&polynomials[i]));
            }
        }
    }
    let expected: Value =
        serde_json::from_str(include_str!("../../historical/results/toy-zeta.json")).unwrap();
    let mut curves = serde_json::Map::new();
    for i in 0..8 {
        let f = frobenius(7, &counts[i]);
        let order = f.iter().sum::<i64>();
        assert_eq!(
            json!(counts[i]),
            expected["curves"][names[i]]["extension_point_counts"]
        );
        assert_eq!(
            json!(f),
            expected["curves"][names[i]]["frobenius_coefficients_descending"]
        );
        assert_eq!(json!(order), expected["curves"][names[i]]["jacobian_order"]);
        curves.insert(names[i].into(),json!({"extension_point_counts":counts[i],"frobenius_coefficients_descending":f,"jacobian_order":order}));
    }
    fn product(a: &[i64], b: &[i64]) -> Vec<i64> {
        let mut c = vec![0; a.len() + b.len() - 1];
        for (i, a) in a.iter().enumerate() {
            for (j, b) in b.iter().enumerate() {
                c[i + j] += a * b;
            }
        }
        c
    }
    let f: Vec<Vec<i64>> = counts.iter().map(|c| frobenius(7, c)).collect();
    assert_eq!(f[0], product(&f[4], &f[5]));
    assert_eq!(f[1], product(&product(&f[4], &f[4]), &f[6]));
    assert_eq!(f[3], product(&f[0], &f[7]));
    let mut remainder = f[2].clone();
    for i in 0..=remainder.len() - f[4].len() {
        let c = remainder[i];
        for (j, v) in f[4].iter().enumerate() {
            remainder[i + j] -= c * v;
        }
    }
    assert!(remainder.iter().all(|c| *c == 0));
    json!({"scope":"native exact toy point-count replay of frozen historical fixture; producer check",
        "q":7,"b":1,"curves":curves,"factorization_checks":true})
}

pub fn certificates() -> Value {
    use identity_certificate::{check_with_extra, issue, Expr as E, IdentityStatement, Verdict};
    fn power(a: E, n: usize) -> E {
        E::product(vec![a; n])
    }
    let t = E::var("t");
    let b = E::var("b");
    let mut out = vec![];
    for g in 2..=5 {
        let (lhs, rhs) = if g == 3 {
            let k = E::plus(power(t.clone(), 2), E::Const(1));
            (
                E::sum(vec![
                    power(t.clone(), 7),
                    E::times(b.clone(), power(t.clone(), 4)),
                    t.clone(),
                ]),
                E::sum(vec![
                    E::times(t.clone(), power(k.clone(), 3)),
                    E::product(vec![E::Const(-3), power(t.clone(), 3), k]),
                    E::times(b.clone(), power(t.clone(), 4)),
                ]),
            )
        } else {
            let d = degree(g);
            let x = power(t.clone(), d);
            (
                E::sum(vec![
                    power(t.clone(), 3 * d),
                    E::times(E::Const(-3), power(t.clone(), d)),
                    b.clone(),
                ]),
                E::sum(vec![
                    power(x.clone(), 3),
                    E::times(E::Const(-3), x),
                    b.clone(),
                ]),
            )
        };
        let statement=IdentityStatement{name:format!("P-256 genus-{g} cleared cover identity"),
          source:"research/p256_genus_dlp_20261115/REPORT.md; explicit specialization, no novelty claim".into(),
          variables:vec!["t".into(),"b".into()],degree_bound:if g==3 {7}else{(3*degree(g)) as u32},lhs,rhs};
        let certificate = issue(
            statement,
            20261115,
            32,
            "Producer-issued; existing repository checker reused; no independent signed verdict",
            None,
        )
        .unwrap();
        assert!(matches!(
            check_with_extra(&certificate, Some((0x8afc921, 16))).unwrap(),
            Verdict::Accept { .. }
        ));
        out.push(serde_json::to_value(certificate).unwrap());
    }
    json!(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn actual_p256() {
        verify_actual();
    }
    #[test]
    fn extra_quotients_actual_field() {
        let (p, b, _, _) = parameters();
        let t = Poly::monomial(1);
        let h2 = equation(2, &b, &p);
        let b2 = &b * &b % &p;
        let rhs = Poly::new(vec![&b2 * &b % &p], &p)
            .sub(
                &Poly::monomial(2).scale(&(&b2 * BigUint::from(3u32) % &p), &p),
                &p,
            )
            .add(&Poly::monomial(6).scale(&b2, &p), &p);
        assert_eq!(h2.scale(&b2, &p), rhs);
        let h3 = equation(3, &b, &p);
        assert_eq!(
            Poly::monomial(2).mul(&h3, &p),
            Poly::monomial(9)
                .add(&Poly::monomial(6).scale(&b, &p), &p)
                .add(&Poly::monomial(3), &p)
        );
        assert_eq!(
            Poly::monomial(2).mul(&equation(5, &b, &p), &p),
            Poly::monomial(14)
                .sub(&Poly::monomial(6).scale(&3u32.into(), &p), &p)
                .add(&Poly::monomial(2).scale(&b, &p), &p)
        );
        assert_eq!(&p % BigUint::from(3u32), BigUint::one());
        let exponent = (&p - BigUint::one()) / BigUint::from(3u32);
        let zeta = (2u32..)
            .map(|a| BigUint::from(a).modpow(&exponent, &p))
            .find(|z| *z != BigUint::one())
            .unwrap();
        assert_eq!(zeta.modpow(&3u32.into(), &p), BigUint::one());
        assert_ne!(&zeta * &zeta % &p, zeta);
        let k = Poly::new(vec![&zeta * &zeta % &p, BigUint::zero(), zeta], &p);
        let rhs = t
            .mul(&k.pow(3, &p), &p)
            .sub(&Poly::monomial(3).mul(&k, &p).scale(&3u32.into(), &p), &p)
            .add(&Poly::monomial(4).scale(&b, &p), &p);
        assert_eq!(h3, rhs);
    }
    #[test]
    fn known_answer_dlp() {
        toy_correctness();
    }
    #[test]
    fn all_toy_points_and_addition() {
        let p = BigUint::from(13u32);
        let b = BigUint::from(1u32);
        let e = Elliptic {
            p: p.clone(),
            b: b.clone(),
        };
        let mut points = vec![None];
        for x in 0..13u32 {
            for y in 0..13u32 {
                let point = Some((x.into(), y.into()));
                if e.contains(&point) {
                    points.push(point);
                }
            }
        }
        for g in [3, 4] {
            let j = Jacobian::new(g, &b, &p);
            for a in &points {
                let aa = transfer(g, a, &p);
                assert!(j.valid(&aa));
                assert_eq!(decode(g, &aa, &p), *a);
                assert_eq!(j.add(&aa, &j.neg(&aa)), j.zero());
                for b in &points {
                    assert_eq!(
                        j.add(&aa, &transfer(g, b, &p)),
                        transfer(g, &e.add(a, b), &p)
                    );
                }
            }
        }
    }
    #[test]
    #[should_panic(expected = "two-infinity")]
    fn even_degree_rejected() {
        Jacobian::new(2, &1u32.into(), &13u32.into());
    }
}
