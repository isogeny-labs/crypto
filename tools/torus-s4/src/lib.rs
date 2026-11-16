//! Public-input research arithmetic and a sparse quadratic Chebyshev/S4 circuit.
//! The field modulus is assumed prime. This is variable-time research code.
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::ops::{Add, Mul, Sub};

pub type Result<T> = std::result::Result<T, String>;
pub fn decimal(s: &str) -> Result<BigUint> {
    BigUint::parse_bytes(s.as_bytes(), 10).ok_or_else(|| format!("invalid integer {s}"))
}
pub fn curve25519_prime() -> BigUint {
    (BigUint::one() << 255) - 19u32
}

#[derive(Clone, Debug)]
pub struct Field {
    pub p: BigUint,
}
impl Field {
    pub fn new(p: BigUint) -> Result<Self> {
        if p < 5u32.into() || !p.bit(0) {
            return Err("expected odd prime >= 5".into());
        }
        Ok(Self { p })
    }
    pub fn add(&self, a: &BigUint, b: &BigUint) -> BigUint {
        (a + b) % &self.p
    }
    pub fn sub(&self, a: &BigUint, b: &BigUint) -> BigUint {
        (a + &self.p - (b % &self.p)) % &self.p
    }
    pub fn mul(&self, a: &BigUint, b: &BigUint) -> BigUint {
        (a * b) % &self.p
    }
    pub fn scale(&self, a: &BigUint, n: u64) -> BigUint {
        (a * n) % &self.p
    }
    pub fn square(&self, a: &BigUint) -> BigUint {
        self.mul(a, a)
    }
    pub fn inv(&self, a: &BigUint) -> Result<BigUint> {
        if (a % &self.p).is_zero() {
            return Err("inverse of zero".into());
        }
        Ok(a.modpow(&(&self.p - 2u32), &self.p))
    }
    pub fn sqrt(&self, a: &BigUint) -> Result<Option<BigUint>> {
        let a = a % &self.p;
        if a.is_zero() {
            return Ok(Some(a));
        }
        if &self.p % 8u32 != 5u32.into() {
            return Err("sqrt requires p = 5 mod 8".into());
        }
        let mut r = a.modpow(&((&self.p + 3u32) >> 3), &self.p);
        if self.square(&r) != a {
            let i = BigUint::from(2u32).modpow(&((&self.p - 1u32) >> 2), &self.p);
            r = self.mul(&r, &i);
        }
        Ok((self.square(&r) == a).then_some(r))
    }
    /// First-kind Chebyshev C_n, using binary doubling; no n-sized table.
    pub fn chebyshev(&self, n: u128, x: &BigUint) -> BigUint {
        let (mut a, mut b) = (BigUint::one(), x % &self.p);
        if n == 0 {
            return a;
        }
        for bit in (0..128 - n.leading_zeros()).rev() {
            let ab = self.sub(&self.scale(&self.mul(&a, &b), 2), x);
            if n >> bit & 1 == 0 {
                b = ab;
                a = self.sub(&self.scale(&self.square(&a), 2), &1u32.into());
            } else {
                a = ab;
                b = self.sub(&self.scale(&self.square(&b), 2), &1u32.into());
            }
        }
        a
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Case {
    pub id: String,
    pub p: serde_json::Number,
    #[serde(rename = "A")]
    pub a: serde_json::Number,
    #[serde(rename = "M")]
    pub m: serde_json::Number,
    pub beta: serde_json::Number,
    pub target: [serde_json::Number; 2],
}
impl Case {
    pub fn field(&self) -> Result<Field> {
        Field::new(decimal(&self.p.to_string())?)
    }
    pub fn order(&self) -> Result<u128> {
        decimal(&self.m.to_string())?
            .to_u128()
            .filter(|n| *n > 0)
            .ok_or("M outside 1..2^128".into())
    }
    pub fn coefficient(&self) -> Result<BigUint> {
        decimal(&self.a.to_string())
    }
    pub fn beta(&self) -> Result<BigUint> {
        decimal(&self.beta.to_string())
    }
    pub fn point(&self) -> Result<Point> {
        Ok(Some((
            decimal(&self.target[0].to_string())?,
            decimal(&self.target[1].to_string())?,
        )))
    }
    pub fn target_y(&self) -> Result<BigUint> {
        let f = self.field()?;
        let u = decimal(&self.target[0].to_string())?;
        Ok(f.mul(&f.sub(&u, &1u32.into()), &f.inv(&f.add(&u, &1u32.into()))?))
    }
    pub fn validate(&self) -> Result<()> {
        let f = self.field()?;
        self.order()?;
        if self.coefficient()? >= f.p || self.beta()? >= f.p {
            return Err("noncanonical field constant".into());
        }
        if f.sub(&f.square(&self.coefficient()?), &4u32.into())
            .is_zero()
        {
            return Err("singular Montgomery curve".into());
        }
        if !Curve::from_case(self)?.contains(&self.point()?) {
            return Err("target is not on curve".into());
        }
        self.target_y()?;
        Ok(())
    }
}

/// Only monomials of degree 0, 1 or 2 are stored. Indices are sorted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Poly {
    p: BigUint,
    pub terms: BTreeMap<Vec<usize>, BigUint>,
}
impl Poly {
    pub fn constant(p: &BigUint, c: BigUint) -> Self {
        let c = c % p;
        let mut terms = BTreeMap::new();
        if !c.is_zero() {
            terms.insert(vec![], c);
        }
        Self {
            p: p.clone(),
            terms,
        }
    }
    pub fn small(p: &BigUint, n: u64) -> Self {
        Self::constant(p, n.into())
    }
    pub fn var(p: &BigUint, i: usize) -> Self {
        Self {
            p: p.clone(),
            terms: BTreeMap::from([(vec![i], BigUint::one())]),
        }
    }
    pub fn scale(&self, n: u64) -> Self {
        self * &Self::small(&self.p, n)
    }
    pub fn degree(&self) -> usize {
        self.terms.keys().map(Vec::len).max().unwrap_or(0)
    }
    pub fn evaluate(&self, values: &[BigUint]) -> Result<BigUint> {
        let mut total = BigUint::zero();
        for (m, c) in &self.terms {
            let mut term = c.clone();
            for i in m {
                term = term * values.get(*i).ok_or("missing variable")? % &self.p;
            }
            total = (total + term) % &self.p;
        }
        Ok(total)
    }
}
impl Add<&Poly> for &Poly {
    type Output = Poly;
    fn add(self, b: &Poly) -> Poly {
        assert_eq!(self.p, b.p);
        let mut r = self.clone();
        for (m, c) in &b.terms {
            let v = (r.terms.get(m).cloned().unwrap_or_default() + c) % &self.p;
            if v.is_zero() {
                r.terms.remove(m);
            } else {
                r.terms.insert(m.clone(), v);
            }
        }
        r
    }
}
impl Sub<&Poly> for &Poly {
    type Output = Poly;
    fn sub(self, b: &Poly) -> Poly {
        self + &(b * &Poly::constant(&self.p, &self.p - 1u32))
    }
}
impl Mul<&Poly> for &Poly {
    type Output = Poly;
    fn mul(self, b: &Poly) -> Poly {
        assert_eq!(self.p, b.p);
        let mut r = Poly::small(&self.p, 0);
        for (a, ac) in &self.terms {
            for (b, bc) in &b.terms {
                let mut m = a.clone();
                m.extend(b);
                m.sort_unstable();
                assert!(m.len() <= 2, "nonquadratic circuit operation");
                let c = (r.terms.get(&m).cloned().unwrap_or_default() + ac * bc) % &self.p;
                if c.is_zero() {
                    r.terms.remove(&m);
                } else {
                    r.terms.insert(m, c);
                }
            }
        }
        r
    }
}

#[derive(Clone, Debug)]
pub struct System {
    pub variables: Vec<String>,
    pub equations: Vec<Poly>,
    pub p: BigUint,
}
impl System {
    fn variable(&mut self, name: String) -> Poly {
        let v = Poly::var(&self.p, self.variables.len());
        self.variables.push(name);
        v
    }
    fn cheb_get(
        &mut self,
        side: usize,
        n: u128,
        x: &Poly,
        cache: &mut BTreeMap<u128, Poly>,
    ) -> Poly {
        if let Some(v) = cache.get(&n) {
            return v.clone();
        }
        let rhs = self.cheb_expr(side, n, x, cache);
        let v = self.variable(format!("c{side}_{n}"));
        self.equations.push(&v - &rhs);
        cache.insert(n, v.clone());
        v
    }
    fn cheb_expr(
        &mut self,
        side: usize,
        n: u128,
        x: &Poly,
        cache: &mut BTreeMap<u128, Poly>,
    ) -> Poly {
        let h = self.cheb_get(side, n / 2, x, cache);
        if n & 1 == 0 {
            &(&h * &h).scale(2) - &Poly::small(&self.p, 1)
        } else {
            let k = self.cheb_get(side, n / 2 + 1, x, cache);
            &(&h * &k).scale(2) - x
        }
    }
    pub fn compile(case: &Case) -> Result<Self> {
        case.validate()?;
        let p = case.field()?.p;
        let a = case.coefficient()?;
        let n = case.order()?;
        let mut s = Self {
            variables: vec![],
            equations: vec![],
            p: p.clone(),
        };
        let ys: Vec<_> = (0..3).map(|i| s.variable(format!("y{i}"))).collect();
        let one = Poly::small(&p, 1);
        let am = Poly::constant(&p, (&a + &p - 2u32) % &p);
        let ap = Poly::constant(&p, (&a + 2u32) % &p);
        let mut memberships = vec![];
        for (i, y) in ys.iter().enumerate() {
            let x = s.variable(format!("x{i}"));
            let den = &ap - &(&am * y);
            memberships.push((i, x.clone(), y.clone(), &den * &den));
            let mut cache = BTreeMap::from([(0, one.clone()), (1, x.clone())]);
            let top = s.cheb_expr(i, n, &x, &mut cache);
            s.equations.push(&top - &Poly::constant(&p, case.beta()?));
        }
        for (i, x, y, den) in memberships {
            let q = s.variable(format!("densq{i}"));
            s.equations.push(&q - &den);
            let z = &one - &y;
            let c = Poly::constant(&p, (BigUint::from(2u32) * &a * &a) % &p);
            s.equations.push(&(&q * &(&x + &one)) - &(&c * &(&z * &z)));
        }
        let aux: Vec<_> = ["sx", "sy", "sz", "qa", "qb", "ru", "rv", "rw"]
            .iter()
            .map(|n| s.variable(n.to_string()))
            .collect();
        let (sx, sy, sz, qa, qb, u, v, w) = (
            &aux[0], &aux[1], &aux[2], &aux[3], &aux[4], &aux[5], &aux[6], &aux[7],
        );
        let t = Poly::constant(&p, case.target_y()?);
        let four = Poly::small(&p, 4);
        let c = &(sx + sy).scale(4) - qa;
        let d = &(&(&am * &(sz - &one)) * &(&(&t * &t) - &one)) + &four;
        let e = &Poly::small(&p, 0) - &(&ys[2] * &t).scale(8);
        let ff = &(sz + &(&t * &t)).scale(4) - &d;
        s.equations.extend([
            sx - &(&ys[0] * &ys[0]),
            sy - &(&ys[1] * &ys[1]),
            sz - &(&ys[2] * &ys[2]),
            &(qa - &(&am * &(&(sx - &one) * &(sy - &one)))) - &four,
            qb + &(&ys[0] * &ys[1]).scale(8),
            &(u - &(qa * &ff)) + &(&c * &d),
            &(v - &(qa * &e)) + &(qb * &d),
            &(w - &(qb * &ff)) + &(&c * &e),
            &(u * u) - &(v * w),
        ]);
        Ok(s)
    }
    pub fn check(&self, values: &[BigUint]) -> Result<()> {
        if values.len() != self.variables.len() {
            return Err("assignment length mismatch".into());
        }
        for (i, e) in self.equations.iter().enumerate() {
            if !e.evaluate(values)?.is_zero() {
                return Err(format!("equation {i} rejected"));
            }
        }
        Ok(())
    }
    pub fn witness(&self, case: &Case, us: &[BigUint; 3]) -> Result<Vec<BigUint>> {
        let f = case.field()?;
        let a = case.coefficient()?;
        let one = BigUint::one();
        let mut map = BTreeMap::new();
        let mut yy = vec![];
        let mut xx = vec![];
        for (i, u) in us.iter().enumerate() {
            let y = f.mul(&f.sub(u, &one), &f.inv(&f.add(u, &one))?);
            let den = f.square(&f.sub(
                &f.add(&a, &2u32.into()),
                &f.mul(&f.sub(&a, &2u32.into()), &y),
            ));
            let x = f.sub(
                &f.mul(
                    &f.scale(&f.square(&a), 2),
                    &f.mul(&f.square(&f.sub(&one, &y)), &f.inv(&den)?),
                ),
                &one,
            );
            map.insert(format!("y{i}"), y.clone());
            map.insert(format!("x{i}"), x.clone());
            map.insert(format!("densq{i}"), den);
            yy.push(y);
            xx.push(x);
        }
        let t = case.target_y()?;
        let sq: Vec<_> = yy.iter().map(|y| f.square(y)).collect();
        let qa = f.add(
            &f.mul(
                &f.sub(&a, &2u32.into()),
                &f.mul(&f.sub(&sq[0], &one), &f.sub(&sq[1], &one)),
            ),
            &4u32.into(),
        );
        let qb = f.sub(&0u32.into(), &f.scale(&f.mul(&yy[0], &yy[1]), 8));
        let c = f.sub(&f.scale(&f.add(&sq[0], &sq[1]), 4), &qa);
        let d = f.add(
            &f.mul(
                &f.sub(&a, &2u32.into()),
                &f.mul(&f.sub(&sq[2], &one), &f.sub(&f.square(&t), &one)),
            ),
            &4u32.into(),
        );
        let e = f.sub(&0u32.into(), &f.scale(&f.mul(&yy[2], &t), 8));
        let ff = f.sub(&f.scale(&f.add(&sq[2], &f.square(&t)), 4), &d);
        for (name, value) in [
            ("sx", sq[0].clone()),
            ("sy", sq[1].clone()),
            ("sz", sq[2].clone()),
            ("qa", qa.clone()),
            ("qb", qb.clone()),
            ("ru", f.sub(&f.mul(&qa, &ff), &f.mul(&c, &d))),
            ("rv", f.sub(&f.mul(&qa, &e), &f.mul(&qb, &d))),
            ("rw", f.sub(&f.mul(&qb, &ff), &f.mul(&c, &e))),
        ] {
            map.insert(name.into(), value);
        }
        self.variables
            .iter()
            .map(|name| {
                if let Some(v) = map.get(name) {
                    return Ok(v.clone());
                }
                let (side, n) = name
                    .strip_prefix('c')
                    .ok_or("unknown witness name")?
                    .split_once('_')
                    .ok_or("bad chain name")?;
                Ok(f.chebyshev(
                    n.parse().map_err(|_| "bad chain index")?,
                    &xx[side.parse::<usize>().map_err(|_| "bad side")?],
                ))
            })
            .collect()
    }
    pub fn canonical_json(&self) -> serde_json::Value {
        serde_json::json!({"schema":"torus-s4.quadratic/v1","p":self.p.to_string(),"variables":self.variables,"equations":self.equations.iter().map(|e|e.terms.iter().map(|(m,c)|serde_json::json!({"monomial":m,"coefficient":c.to_string()})).collect::<Vec<_>>()).collect::<Vec<_>>()})
    }
    /// Export to Singular's arbitrary-modulus coefficient ring. std supports
    /// this interface; slimgb/FGLM must not be assumed supported.
    pub fn singular(&self) -> String {
        let mut out = format!(
            "ring r=(integer,{}),({}),dp;\nideal I=\n",
            self.p,
            self.variables.join(",")
        );
        let polys: Vec<_> = self
            .equations
            .iter()
            .map(|e| {
                if e.terms.is_empty() {
                    return "0".into();
                }
                e.terms
                    .iter()
                    .map(|(m, c)| {
                        let mut term = c.to_string();
                        for i in m {
                            term.push('*');
                            term.push_str(&self.variables[*i]);
                        }
                        term
                    })
                    .collect::<Vec<_>>()
                    .join("+")
            })
            .collect();
        out.push_str(&polys.join(",\n"));
        out.push_str(";\nshort=0;\nprint(\"TORUS_STD_BEGIN\");\nideal G=std(I);\nprint(\"TORUS_STD_END\");\nif(size(G)==1 && G[1]==1){print(\"TORUS_UNIT_IDEAL\");}else{print(\"TORUS_NONUNIT_BASIS\");}\nprint(G);\nprint(\"TORUS_NATIVE_DONE\");\nquit;\n");
        out
    }
}

pub type Point = Option<(BigUint, BigUint)>;
pub struct Curve {
    pub f: Field,
    pub a: BigUint,
}
impl Curve {
    pub fn from_case(c: &Case) -> Result<Self> {
        Ok(Self {
            f: c.field()?,
            a: c.coefficient()?,
        })
    }
    pub fn rhs(&self, u: &BigUint) -> BigUint {
        let f = &self.f;
        f.add(&f.mul(&f.square(u), &f.add(u, &self.a)), u)
    }
    pub fn contains(&self, p: &Point) -> bool {
        match p {
            None => true,
            Some((u, v)) => u < &self.f.p && v < &self.f.p && self.f.square(v) == self.rhs(u),
        }
    }
    pub fn add(&self, p: &Point, q: &Point) -> Result<Point> {
        let f = &self.f;
        let (Some((x, y)), Some((u, v))) = (p, q) else {
            return Ok(if p.is_none() { q.clone() } else { p.clone() });
        };
        let lambda = if x == u {
            if f.add(y, v).is_zero() {
                return Ok(None);
            }
            f.mul(
                &f.add(
                    &f.add(&f.scale(&f.square(x), 3), &f.scale(&f.mul(&self.a, x), 2)),
                    &1u32.into(),
                ),
                &f.inv(&f.scale(y, 2))?,
            )
        } else {
            f.mul(&f.sub(v, y), &f.inv(&f.sub(u, x))?)
        };
        let xx = f.sub(&f.sub(&f.sub(&f.square(&lambda), &self.a), x), u);
        let yy = f.sub(&f.mul(&lambda, &f.sub(x, &xx)), y);
        Ok(Some((xx, yy)))
    }
    pub fn times(&self, p: &Point, mut n: u64) -> Result<Point> {
        let mut a = p.clone();
        let mut r = None;
        while n > 0 {
            if n & 1 == 1 {
                r = self.add(&r, &a)?;
            }
            a = self.add(&a, &a)?;
            n >>= 1;
        }
        Ok(r)
    }
    pub fn member(&self, c: &Case, u: &BigUint) -> Result<bool> {
        let f = &self.f;
        let d = f.add(&f.scale(u, 2), &self.a);
        if d.is_zero() {
            return Ok(false);
        }
        let x = f.sub(
            &f.mul(&f.scale(&f.square(&self.a), 2), &f.inv(&f.square(&d))?),
            &1u32.into(),
        );
        Ok(f.chebyshev(c.order()?, &x) == c.beta()?)
    }
    /// Separate from circuit evaluation: lift, test membership, enumerate only
    /// the eight possible point signs, and compare the actual group sum.
    pub fn verify_triple(&self, c: &Case, us: &[BigUint; 3]) -> Result<[bool; 3]> {
        c.validate()?;
        if self.f.p != c.field()?.p || self.a != c.coefficient()? {
            return Err("curve/case mismatch".into());
        }
        let mut points = vec![];
        for u in us {
            if u >= &self.f.p || !self.member(c, u)? {
                return Err("nonmember u".into());
            }
            let v = self
                .f
                .sqrt(&self.rhs(u))?
                .ok_or("u does not lift to E(Fp)")?;
            points.push((u.clone(), v));
        }
        let target = c.point()?;
        for mask in 0..8 {
            let mut sum = None;
            for (i, (u, v)) in points.iter().enumerate() {
                let v = if mask >> i & 1 == 1 {
                    self.f.sub(&0u32.into(), v)
                } else {
                    v.clone()
                };
                sum = self.add(&sum, &Some((u.clone(), v)))?;
            }
            if sum == target {
                return Ok([mask & 1 != 0, mask & 2 != 0, mask & 4 != 0]);
            }
        }
        Err("no signed sum equals target".into())
    }
}

/// SHA-256 used for immutable input and qualifier manifests, not a proof of execution.
pub fn sha256(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(bytes))
}
