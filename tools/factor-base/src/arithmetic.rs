use anyhow::{ensure, Context, Result};
use num_bigint::{BigInt, BigUint};
use num_traits::{One, ToPrimitive, Zero};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub type N = BigUint;
pub fn n(x: u64) -> N {
    N::from(x)
}
pub fn parse(s: &str) -> Result<N> {
    let (s, radix) = s.strip_prefix("0x").map_or((s, 10), |s| (s, 16));
    ensure!(
        !s.is_empty() && s.len() <= 300,
        "invalid or oversized integer"
    );
    N::parse_bytes(s.as_bytes(), radix).context("invalid integer")
}
pub fn enc(x: &N) -> String {
    format!("0x{}", x.to_str_radix(16))
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum FieldSpec {
    Binary { degree: usize, modulus: String },
    Prime { modulus: String },
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PointSpec {
    pub x: String,
    pub y: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CurveSpec {
    pub field: FieldSpec,
    pub a: String,
    pub b: String,
    pub group_order: String,
    pub subgroup_order: String,
    pub cofactor: String,
    pub generator: PointSpec,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: N,
    pub y: N,
}
pub type P = Option<Point>;
impl Point {
    pub fn spec(&self) -> PointSpec {
        PointSpec {
            x: enc(&self.x),
            y: enc(&self.y),
        }
    }
    pub fn from_spec(s: &PointSpec) -> Result<Self> {
        Ok(Self {
            x: parse(&s.x)?,
            y: parse(&s.y)?,
        })
    }
}
#[derive(Clone, Debug)]
pub enum Field {
    Binary { degree: usize, modulus: N },
    Prime { modulus: N },
}
fn poly_rem(mut a: N, b: &N) -> N {
    while !a.is_zero() && a.bits() >= b.bits() {
        let shift = a.bits() - b.bits();
        a ^= b << shift as usize;
    }
    a
}
fn poly_gcd(mut a: N, mut b: N) -> N {
    while !b.is_zero() {
        let t = poly_rem(a, &b);
        a = b;
        b = t;
    }
    a
}
pub fn probable_prime(p: &N) -> bool {
    if p < &n(2) {
        return false;
    }
    for q in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if p == &n(q) {
            return true;
        }
        if p % q == N::zero() {
            return false;
        }
    }
    let mut d = p - N::one();
    let mut s = 0;
    while !d.bit(0) {
        d >>= 1;
        s += 1;
    }
    // The first seven bases are deterministic for integers below 2^64.
    for a in [
        2, 325, 9375, 28178, 450775, 9780504, 1795265022, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37,
    ] {
        let a = n(a) % p;
        if a.is_zero() {
            continue;
        }
        let mut x = a.modpow(&d, p);
        if x == N::one() || x == p - N::one() {
            continue;
        }
        let mut passed = false;
        for _ in 1..s {
            x = &x * &x % p;
            if x == p - N::one() {
                passed = true;
                break;
            }
        }
        if !passed {
            return false;
        }
    }
    true
}
impl Field {
    pub fn new(spec: &FieldSpec) -> Result<Self> {
        match spec {
            FieldSpec::Prime { modulus } => {
                let p = parse(modulus)?;
                ensure!(
                    p.bits() <= 576 && p > n(3) && probable_prime(&p),
                    "prime field requires a prime modulus > 3 (maximum 576 bits)"
                );
                Ok(Self::Prime { modulus: p })
            }
            FieldSpec::Binary { degree, modulus } => {
                let f = parse(modulus)?;
                ensure!(
                    (2..=571).contains(degree) && f.bits() == (*degree + 1) as u64 && f.bit(0),
                    "invalid binary modulus or degree"
                );
                let field = Self::Binary {
                    degree: *degree,
                    modulus: f.clone(),
                };
                let x = n(2);
                let mut h = x.clone();
                for i in 1..=*degree {
                    h = field.mul(&h, &h);
                    if i <= degree / 2 {
                        ensure!(
                            poly_gcd(&h ^ &x, f.clone()) == N::one(),
                            "binary modulus is reducible"
                        );
                    }
                }
                ensure!(h == x, "binary modulus fails Frobenius identity");
                Ok(field)
            }
        }
    }
    pub fn size(&self) -> N {
        match self {
            Self::Prime { modulus } => modulus.clone(),
            Self::Binary { degree, .. } => N::one() << degree,
        }
    }
    pub fn degree(&self) -> Option<usize> {
        if let Self::Binary { degree, .. } = self {
            Some(*degree)
        } else {
            None
        }
    }
    pub fn add(&self, a: &N, b: &N) -> N {
        match self {
            Self::Prime { modulus } => (a + b) % modulus,
            Self::Binary { .. } => a ^ b,
        }
    }
    pub fn sub(&self, a: &N, b: &N) -> N {
        match self {
            Self::Prime { modulus } => (a + modulus - b) % modulus,
            Self::Binary { .. } => a ^ b,
        }
    }
    pub fn mul(&self, a: &N, b: &N) -> N {
        match self {
            Self::Prime { modulus } => a * b % modulus,
            Self::Binary { degree, modulus } => {
                let mut a = a.clone();
                let mut b = b.clone();
                let mut r = N::zero();
                while !b.is_zero() {
                    if b.bit(0) {
                        r ^= &a;
                    }
                    b >>= 1;
                    a <<= 1;
                    if a.bit(*degree as u64) {
                        a ^= modulus;
                    }
                }
                r
            }
        }
    }
    #[cfg(test)]
    pub fn pow(&self, a: &N, e: &N) -> N {
        let mut a = a.clone();
        let mut e = e.clone();
        let mut r = N::one();
        while !e.is_zero() {
            if e.bit(0) {
                r = self.mul(&r, &a);
            }
            e >>= 1;
            a = self.mul(&a, &a);
        }
        r
    }
    pub fn inv(&self, a: &N) -> N {
        assert!(!a.is_zero());
        match self {
            Self::Binary { modulus, .. } => {
                let (mut u, mut v, mut g1, mut g2) =
                    (a.clone(), modulus.clone(), N::one(), N::zero());
                while u != N::one() {
                    if u.bits() < v.bits() {
                        std::mem::swap(&mut u, &mut v);
                        std::mem::swap(&mut g1, &mut g2);
                    }
                    let shift = (u.bits() - v.bits()) as usize;
                    u ^= &v << shift;
                    g1 ^= &g2 << shift;
                }
                poly_rem(g1, modulus)
            }
            Self::Prime { modulus } => {
                let p = BigInt::from(modulus.clone());
                let (mut r0, mut r1) = (p.clone(), BigInt::from(a.clone()));
                let (mut t0, mut t1) = (BigInt::zero(), BigInt::one());
                while !r1.is_zero() {
                    let q = &r0 / &r1;
                    let next_r = &r0 - &q * &r1;
                    r0 = r1;
                    r1 = next_r;
                    let next_t = &t0 - &q * &t1;
                    t0 = t1;
                    t1 = next_t;
                }
                assert_eq!(r0, BigInt::one());
                (((t0 % &p) + &p) % &p).to_biguint().unwrap()
            }
        }
    }
    pub fn div(&self, a: &N, b: &N) -> N {
        self.mul(a, &self.inv(b))
    }
    pub fn sq(&self, a: &N) -> N {
        self.mul(a, a)
    }
    pub fn trace(&self, a: &N) -> N {
        let mut t = a.clone();
        let mut z = a.clone();
        for _ in 1..self.degree().unwrap() {
            z = self.sq(&z);
            t ^= &z;
        }
        t
    }
    pub fn sqrt(&self, a: &N) -> Option<N> {
        match self {
            Self::Binary { degree, .. } => {
                let mut r = a.clone();
                for _ in 1..*degree {
                    r = self.sq(&r);
                }
                Some(r)
            }
            Self::Prime { modulus: p } => {
                if a.is_zero() {
                    return Some(N::zero());
                }
                if a.modpow(&((p - N::one()) >> 1), p) != N::one() {
                    return None;
                }
                if p % n(4) == n(3) {
                    return Some(a.modpow(&((p + N::one()) >> 2), p));
                }
                let mut q = p - N::one();
                let mut s = 0usize;
                while !q.bit(0) {
                    q >>= 1;
                    s += 1;
                }
                let mut z = n(2);
                while z.modpow(&((p - N::one()) >> 1), p) == N::one() {
                    z += N::one();
                }
                let mut c = z.modpow(&q, p);
                let mut x = a.modpow(&((&q + N::one()) >> 1), p);
                let mut t = a.modpow(&q, p);
                let mut m = s;
                while t != N::one() {
                    let mut i = 1;
                    let mut v = &t * &t % p;
                    while v != N::one() {
                        v = &v * &v % p;
                        i += 1;
                        if i >= m {
                            return None;
                        }
                    }
                    let b = c.modpow(&(N::one() << (m - i - 1)), p);
                    x = x * &b % p;
                    c = &b * &b % p;
                    t = t * &c % p;
                    m = i;
                }
                Some(x)
            }
        }
    }
    pub fn artin_schreier(&self, c: &N) -> Option<N> {
        let degree = self.degree()?;
        if !self.trace(c).is_zero() {
            return None;
        }
        if degree % 2 == 1 {
            let mut z = c.clone();
            for _ in 1..=(degree - 1) / 2 {
                z = self.sq(&self.sq(&z)) ^ c;
            }
            return Some(z);
        }
        let mut pivots: Vec<Option<(N, N)>> = vec![None; degree];
        for i in 0..degree {
            let mut witness = N::one() << i;
            let mut v = self.sq(&witness) ^ &witness;
            while !v.is_zero() {
                let k = v.bits() as usize - 1;
                if let Some((p, w)) = &pivots[k] {
                    v ^= p;
                    witness ^= w;
                } else {
                    pivots[k] = Some((v, witness));
                    break;
                }
            }
        }
        let mut v = c.clone();
        let mut z = N::zero();
        while !v.is_zero() {
            let k = v.bits() as usize - 1;
            let (p, w) = pivots[k].as_ref()?;
            v ^= p;
            z ^= w;
        }
        Some(z)
    }
}
#[derive(Clone)]
pub struct Curve {
    pub spec: CurveSpec,
    pub f: Field,
    pub a: N,
    pub b: N,
    pub r: N,
    pub h: N,
    pub order: N,
    pub g: Point,
    pub validation: Value,
}
impl Curve {
    pub fn new(mut spec: CurveSpec) -> Result<Self> {
        let f = Field::new(&spec.field)?;
        let a = parse(&spec.a)?;
        let b = parse(&spec.b)?;
        let r = parse(&spec.subgroup_order)?;
        let h = parse(&spec.cofactor)?;
        let order = parse(&spec.group_order)?;
        let g = Point::from_spec(&spec.generator)?;
        ensure!(
            a < f.size() && b < f.size(),
            "noncanonical curve coefficient"
        );
        ensure!(
            probable_prime(&r) && !h.is_zero() && &r * &h == order && &h % &r != N::zero(),
            "invalid subgroup order/cofactor; require prime r, gcd(r,h)=1, N=r*h"
        );
        let q = f.size();
        let delta = if order > &q + N::one() {
            &order - &q - N::one()
        } else {
            &q + N::one() - &order
        };
        ensure!(&delta * &delta <= n(4) * &q, "order outside Hasse bound");
        match &f {
            Field::Binary { .. } => ensure!(!b.is_zero(), "singular binary curve"),
            Field::Prime { .. } => ensure!(
                !f.add(
                    &f.mul(&n(4), &f.mul(&f.sq(&a), &a)),
                    &f.mul(&n(27), &f.sq(&b))
                )
                .is_zero(),
                "singular prime curve"
            ),
        }
        spec.a = enc(&a);
        spec.b = enc(&b);
        spec.group_order = enc(&order);
        spec.subgroup_order = enc(&r);
        spec.cofactor = enc(&h);
        spec.generator = g.spec();
        match &mut spec.field {
            FieldSpec::Prime { modulus } | FieldSpec::Binary { modulus, .. } => {
                *modulus = enc(&parse(modulus)?)
            }
        }
        let field_status = match &f {
            Field::Prime { modulus } if modulus.bits() > 64 => "probable_prime_only",
            _ => "proved",
        };
        let subgroup_status = if r.bits() > 64 {
            "probable_prime_only"
        } else {
            "proved"
        };
        let mut curve = Self {
            spec,
            f,
            a,
            b,
            r,
            h,
            order,
            g,
            validation: json!({"field":field_status,"subgroup_primality":subgroup_status,"group_order":"caller_asserted","generator":"checked_annihilated_by_r"}),
        };
        ensure!(
            curve.on_curve(&curve.g) && curve.scale(Some(curve.g.clone()), &curve.r).is_none(),
            "invalid subgroup generator"
        );
        if curve.is_koblitz() {
            let mut s0 = BigInt::from(2);
            let mut s1 = if curve.a.is_zero() {
                BigInt::from(-1)
            } else {
                BigInt::from(1)
            };
            let mu = s1.clone();
            for _ in 2..=curve.f.degree().unwrap() {
                let t = &mu * &s1 - BigInt::from(2) * &s0;
                s0 = s1;
                s1 = t;
            }
            ensure!(
                BigInt::from(curve.order.clone())
                    == BigInt::from(curve.f.size()) + BigInt::from(1) - s1,
                "incorrect Koblitz group order"
            );
            curve.validation["group_order"] = json!("proved_frobenius_recurrence");
        } else if curve.f.size() <= n(4096) {
            let mut count = 1u64;
            for x in 0..curve.f.size().to_u64().unwrap() {
                count += curve.lift(&n(x)).len() as u64;
            }
            ensure!(
                curve.order == n(count),
                "group order disagrees with exact census"
            );
            curve.validation["group_order"] = json!("proved_exact_census");
        }
        Ok(curve)
    }
    pub fn is_koblitz(&self) -> bool {
        self.f.degree().is_some() && self.a <= N::one() && self.b == N::one()
    }
    pub fn on_curve(&self, p: &Point) -> bool {
        if p.x >= self.f.size() || p.y >= self.f.size() {
            return false;
        }
        let f = &self.f;
        let x2 = f.sq(&p.x);
        let x3 = f.mul(&x2, &p.x);
        if f.degree().is_some() {
            f.add(&f.sq(&p.y), &f.mul(&p.x, &p.y))
                == f.add(&f.add(&x3, &f.mul(&self.a, &x2)), &self.b)
        } else {
            f.sq(&p.y) == f.add(&f.add(&x3, &f.mul(&self.a, &p.x)), &self.b)
        }
    }
    pub fn neg(&self, p: &Point) -> Point {
        Point {
            x: p.x.clone(),
            y: if self.f.degree().is_some() {
                &p.x ^ &p.y
            } else {
                self.f.sub(&N::zero(), &p.y)
            },
        }
    }
    pub fn lift(&self, x: &N) -> Vec<Point> {
        let f = &self.f;
        let mut points = Vec::new();
        let y = if f.degree().is_some() {
            if x.is_zero() {
                f.sqrt(&self.b)
            } else {
                let c = f.add(&f.add(x, &self.a), &f.div(&self.b, &f.sq(x)));
                f.artin_schreier(&c).map(|z| f.mul(x, &z))
            }
        } else {
            f.sqrt(&f.add(&f.add(&f.mul(&f.sq(x), x), &f.mul(&self.a, x)), &self.b))
        };
        if let Some(y) = y {
            let p = Point { x: x.clone(), y };
            let neg = self.neg(&p);
            points.push(p.clone());
            if neg != p {
                points.push(neg);
            }
            points.sort();
        }
        points
    }
    pub fn add(&self, p: &P, q: &P) -> P {
        let (p, q) = match (p, q) {
            (None, _) => return q.clone(),
            (_, None) => return p.clone(),
            (Some(p), Some(q)) => (p, q),
        };
        let f = &self.f;
        if p.x == q.x && (p != q || self.neg(p) == *p) {
            return None;
        }
        let (x, y) = if f.degree().is_some() {
            let l = if p == q {
                f.add(&p.x, &f.div(&p.y, &p.x))
            } else {
                f.div(&f.add(&p.y, &q.y), &f.add(&p.x, &q.x))
            };
            let x = f.add(&f.add(&f.add(&f.sq(&l), &l), &f.add(&p.x, &q.x)), &self.a);
            let y = if p == q {
                f.add(&f.sq(&p.x), &f.mul(&f.add(&l, &N::one()), &x))
            } else {
                f.add(&f.add(&f.mul(&l, &f.add(&p.x, &x)), &x), &p.y)
            };
            (x, y)
        } else {
            let l = if p == q {
                f.div(
                    &f.add(&f.mul(&n(3), &f.sq(&p.x)), &self.a),
                    &f.mul(&n(2), &p.y),
                )
            } else {
                f.div(&f.sub(&q.y, &p.y), &f.sub(&q.x, &p.x))
            };
            let x = f.sub(&f.sub(&f.sq(&l), &p.x), &q.x);
            let y = f.sub(&f.mul(&l, &f.sub(&p.x, &x)), &p.y);
            (x, y)
        };
        Some(Point { x, y })
    }
    pub fn scale(&self, mut p: P, k: &N) -> P {
        let mut k = k.clone();
        let mut r = None;
        while !k.is_zero() {
            if k.bit(0) {
                r = self.add(&r, &p);
            }
            k >>= 1;
            p = self.add(&p, &p);
        }
        r
    }
    pub fn subgroup(&self, p: &Point) -> bool {
        self.on_curve(p) && self.scale(Some(p.clone()), &self.r).is_none()
    }
    pub fn requirements(&self) -> Value {
        let subfields = self
            .f
            .degree()
            .map(|d| (2..d).filter(|i| d % i == 0).collect::<Vec<_>>());
        let ord = self.f.degree().and_then(|d| {
            if d % 2 == 0 {
                return None;
            }
            let mut v = 2 % d;
            let mut k = 1;
            while v != 1 && k < d {
                v = 2 * v % d;
                k += 1;
            }
            Some(k)
        });
        json!({"family":if self.is_koblitz(){"koblitz_anomalous_binary"}else if self.f.degree().is_some(){"ordinary_binary"}else if self.order==self.f.size(){"prime_field_anomalous"}else{"prime_field"},
            "binary_extension_degree":self.f.degree(),"prime_extension_degree":self.f.degree().map(|d|probable_prime(&n(d as u64))),
            "proper_intermediate_subfield_degrees":subfields,"multiplicative_order_of_2_mod_degree":ord,
            "cofactor":enc(&self.h),
            "frobenius_automorphism_over_f2":self.is_koblitz(),"folding":"none_all_points_materialized",
            "subgroup_policy":"filter_by_rP_equals_identity_no_cofactor_projection","validation":self.validation})
    }
}
pub fn rank(rows: &[N]) -> usize {
    let mut pivots = std::collections::BTreeMap::<u64, N>::new();
    for r in rows {
        let mut v = r.clone();
        while !v.is_zero() {
            let k = v.bits() - 1;
            if let Some(p) = pivots.get(&k) {
                v ^= p;
            } else {
                pivots.insert(k, v);
                break;
            }
        }
    }
    pivots.len()
}
