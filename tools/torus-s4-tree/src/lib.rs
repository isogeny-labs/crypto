//! Quadratic Semaev trees with a native actual-field norm-one torus base.
//! This is a construction and candidate checker, not a complexity result.
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use torus_s4::{curve25519_prime, Case, Curve, Field, Point, Poly, System};
pub type Result<T> = std::result::Result<T, String>;
fn num(n: BigUint) -> serde_json::Number {
    n.to_string().parse().expect("nonnegative integer")
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fp2 {
    pub a: BigUint,
    pub b: BigUint,
}
impl Fp2 {
    pub fn one() -> Self {
        Self {
            a: 1u32.into(),
            b: 0u32.into(),
        }
    }
    pub fn mul(&self, r: &Self, f: &Field) -> Self {
        Self {
            a: f.add(&f.mul(&self.a, &r.a), &f.scale(&f.mul(&self.b, &r.b), 2)),
            b: f.add(&f.mul(&self.a, &r.b), &f.mul(&self.b, &r.a)),
        }
    }
    pub fn norm(&self, f: &Field) -> BigUint {
        f.sub(&f.square(&self.a), &f.scale(&f.square(&self.b), 2))
    }
    pub fn neg(&self, f: &Field) -> Self {
        Self {
            a: f.sub(&0u32.into(), &self.a),
            b: f.sub(&0u32.into(), &self.b),
        }
    }
    pub fn inverse(&self, f: &Field) -> Result<Self> {
        let d = f.inv(&self.norm(f))?;
        Ok(Self {
            a: f.mul(&self.a, &d),
            b: f.sub(&0u32.into(), &f.mul(&self.b, &d)),
        })
    }
    pub fn pow(&self, n: &BigUint, f: &Field) -> Self {
        let mut a = self.clone();
        let mut r = Self::one();
        for i in 0..n.bits() {
            if n.bit(i) {
                r = r.mul(&a, f);
            }
            if i + 1 < n.bits() {
                a = a.mul(&a, f);
            }
        }
        r
    }
}
#[derive(Clone, Debug)]
pub struct TorusParameters {
    pub f: Field,
    pub m: u128,
    pub h: Fp2,
    pub g: Fp2,
    pub shift: Fp2,
    pub betas: Vec<BigUint>,
}
fn prime_u32(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let mut d = 3;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}
impl TorusParameters {
    pub fn actual_four() -> Result<Self> {
        let f = Field::new(curve25519_prime())?;
        let factors = [60824497u64, 213156431u64];
        if !factors.into_iter().all(prime_u32) {
            return Err("nonprime torus-order factor".into());
        }
        let m = factors[0] as u128 * factors[1] as u128;
        let mb = BigUint::from(m);
        if (&f.p + 1u32) % &mb != BigUint::from(0u32) {
            return Err("M does not divide p+1".into());
        }
        if BigUint::from(2u32).modpow(&((&f.p - 1u32) >> 1), &f.p) != &f.p - 1u32 {
            return Err("quadratic extension is split".into());
        }
        let g = Fp2 {
            a: &f.p - 3u32,
            b: &f.p - 2u32,
        };
        let h = g.pow(&((&f.p + 1u32) / &mb), &f);
        let one = Fp2::one();
        let minus = Fp2 {
            a: &f.p - 1u32,
            b: 0u32.into(),
        };
        if g.norm(&f) != BigUint::from(1u32)
            || h.norm(&f) != BigUint::from(1u32)
            || h.pow(&mb, &f) != one
            || factors
                .into_iter()
                .any(|q| h.pow(&BigUint::from(m / q as u128), &f) == one)
        {
            return Err("exact subgroup order check failed".into());
        }
        let c = g.pow(&mb, &f);
        for d in 1u32..=8 {
            let z = c.pow(&d.into(), &f);
            if z == one || z == minus {
                return Err("cosets can collide under the coordinate orbit".into());
            }
        }
        let betas = (1u32..=4).map(|j| c.pow(&(2 * j).into(), &f).a).collect();
        Ok(Self {
            f,
            m,
            h,
            g,
            shift: Fp2::one(),
            betas,
        })
    }
    /// Cayley parametrization: 0..p enumerates T(Fp), with p denoting -1.
    pub fn shift_parameter(&self, t: &BigUint) -> Result<Fp2> {
        let f = &self.f;
        if t > &f.p {
            return Err("torus parameter outside 0..p".into());
        }
        if t == &f.p {
            return Ok(Fp2 {
                a: &f.p - 1u32,
                b: 0u32.into(),
            });
        }
        let q = f.scale(&f.square(t), 2);
        let d = f.inv(&f.sub(&1u32.into(), &q))?;
        Ok(Fp2 {
            a: f.mul(&f.add(&1u32.into(), &q), &d),
            b: f.mul(&f.scale(t, 2), &d),
        })
    }
    pub fn shifted(&self, shift: Fp2) -> Result<Self> {
        if shift.norm(&self.f) != BigUint::from(1u32) {
            return Err("shift is outside norm-one torus".into());
        }
        let c = self.g.pow(&BigUint::from(self.m), &self.f);
        let d = shift.pow(&BigUint::from(self.m), &self.f);
        let d2 = d.mul(&d, &self.f);
        for n in 2u32..=8 {
            let z = d2.mul(&c.pow(&n.into(), &self.f), &self.f);
            if z == Fp2::one() || z == Fp2::one().neg(&self.f) {
                return Err("inadmissible inversion orbit after shift".into());
            }
        }
        let mut out = self.clone();
        out.shift = shift;
        out.betas = (1u32..=4)
            .map(|j| {
                let z = d.mul(&c.pow(&j.into(), &self.f), &self.f);
                z.mul(&z, &self.f).a
            })
            .collect();
        Ok(out)
    }
    pub fn point(&self, coset: u32, k: u128) -> Result<Point> {
        if !(1..=4).contains(&coset) {
            return Err("coset outside 1..4".into());
        }
        let z = self
            .g
            .pow(&coset.into(), &self.f)
            .mul(&self.h.pow(&BigUint::from(k), &self.f), &self.f)
            .mul(&self.shift, &self.f);
        if z.norm(&self.f) != BigUint::from(1u32) {
            return Err("invalid torus point".into());
        }
        // u = -A/2 +/- A/(2a); exactly one branch lifts for this map.
        let a = BigUint::from(486662u32);
        let half = self.f.inv(&2u32.into())?;
        let center = self.f.sub(&0u32.into(), &self.f.mul(&a, &half));
        let delta = self.f.mul(&self.f.mul(&a, &half), &self.f.inv(&z.a)?);
        let curve = Curve {
            f: self.f.clone(),
            a,
        };
        let mut points = vec![];
        for u in [self.f.add(&center, &delta), self.f.sub(&center, &delta)] {
            if let Some(v) = self.f.sqrt(&curve.rhs(&u))? {
                points.push(Some((u, v)));
            }
        }
        if points.len() != 1 {
            return Err("Elligator branch count is not one".into());
        }
        Ok(points.remove(0))
    }
    pub fn record(&self) -> serde_json::Value {
        serde_json::json!({"schema":"torus-s4-tree.parameters/v2","p":self.f.p.to_string(),"M":self.m.to_string(),"prime_factors":[60824497u64,213156431u64],"h":[self.h.a.to_string(),self.h.b.to_string()],"g":[self.g.a.to_string(),self.g.b.to_string()],"shift":[self.shift.a.to_string(),self.shift.b.to_string()],"betas":self.betas.iter().map(ToString::to_string).collect::<Vec<_>>(),"cosets":4,"raw_coordinate_count":(4*self.m).to_string(),"prime_class_lower_bound_assuming_curve_order_8ell":((self.m-1)/2).to_string(),"prime_class_upper_bound":(4*self.m).to_string(),"group_order_source":"RFC 7748; not recomputed here","pcc_standing":"unverified"})
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TreeInput {
    pub curve: Case,
    pub arity: usize,
    pub betas: Vec<serde_json::Number>,
}
impl TreeInput {
    pub fn from_points(
        p: BigUint,
        a: BigUint,
        m: u128,
        betas: Vec<BigUint>,
        points: &[Point],
    ) -> Result<Self> {
        if betas.is_empty() {
            return Err("empty membership union".into());
        }
        let curve = Curve {
            f: Field::new(p.clone())?,
            a: a.clone(),
        };
        let mut target = None;
        for point in points {
            if !curve.contains(point) || point.is_none() {
                return Err("invalid factor-base point".into());
            }
            target = curve.add(&target, point)?;
        }
        let (u, v) = target.ok_or("target is identity; requires a different target card")?;
        let c = Case {
            id: format!("torus{}_planted", points.len()),
            p: num(p),
            a: num(a),
            m: num(m.into()),
            beta: num(betas[0].clone()),
            target: [num(u), num(v)],
        };
        Ok(Self {
            curve: c,
            arity: points.len(),
            betas: betas.into_iter().map(num).collect(),
        })
    }
    fn beta_values(&self) -> Result<Vec<BigUint>> {
        self.betas
            .iter()
            .map(|v| torus_s4::decimal(&v.to_string()))
            .collect()
    }
}
#[derive(Clone, Debug)]
enum Op {
    Leaf(usize),
    Internal(usize),
    Trace(usize),
    Cheb(usize, u128),
    Den(usize),
    Value(usize),
    Left(usize),
    Right(usize),
    Square(usize),
    A(usize),
    B(usize),
}
#[derive(Clone, Debug)]
pub struct TreeSystem {
    pub system: System,
    pub arity: usize,
    ops: Vec<Op>,
}
struct Builder {
    system: System,
    ops: Vec<Op>,
}
impl Builder {
    fn var(&mut self, name: String, op: Op) -> Poly {
        let i = self.system.variables.len();
        self.system.variables.push(name);
        self.ops.push(op);
        Poly::var(&self.system.p, i)
    }
    fn get(&mut self, i: usize, n: u128, x: &Poly, cache: &mut BTreeMap<u128, Poly>) -> Poly {
        if let Some(p) = cache.get(&n) {
            return p.clone();
        }
        let p = self.expr(i, n, x, cache);
        let v = self.var(format!("c{i}_{n}"), Op::Cheb(i, n));
        self.system.equations.push(&v - &p);
        cache.insert(n, v.clone());
        v
    }
    fn expr(&mut self, i: usize, n: u128, x: &Poly, cache: &mut BTreeMap<u128, Poly>) -> Poly {
        let a = self.get(i, n / 2, x, cache);
        if n % 2 == 0 {
            &(&a * &a).scale(2) - &Poly::small(&self.system.p, 1)
        } else {
            let b = self.get(i, n / 2 + 1, x, cache);
            &(&a * &b).scale(2) - x
        }
    }
}
fn edge(n: usize, j: usize) -> (usize, usize, Option<usize>) {
    (
        if j == 0 { 0 } else { n + j - 1 },
        j + 1,
        if j == n - 2 { None } else { Some(n + j) },
    )
}
fn coordinate(point: &Point, f: &Field) -> Result<BigUint> {
    match point {
        None => Ok(1u32.into()),
        Some((u, _)) => Ok(f.mul(&f.sub(u, &1u32.into()), &f.inv(&f.add(u, &1u32.into()))?)),
    }
}
impl TreeSystem {
    pub fn compile(input: &TreeInput) -> Result<Self> {
        input.curve.validate()?;
        let n = input.arity;
        let betas = input.beta_values()?;
        let f = input.curve.field()?;
        let p = &f.p;
        let a = input.curve.coefficient()?;
        if !(2..=8).contains(&n)
            || ![1, 4].contains(&betas.len())
            || betas.iter().any(|b| b >= p)
            || betas[0] != input.curve.beta()?
        {
            return Err("invalid tree/union parameters".into());
        }
        let mut b = Builder {
            system: System {
                variables: vec![],
                equations: vec![],
                p: p.clone(),
            },
            ops: vec![],
        };
        let mut nodes: Vec<_> = (0..n)
            .map(|i| b.var(format!("y{i}"), Op::Leaf(i)))
            .collect();
        let one = Poly::small(p, 1);
        let am = Poly::constant(p, f.sub(&a, &2u32.into()));
        let ap = Poly::constant(p, f.add(&a, &2u32.into()));
        for (i, y) in nodes.clone().iter().enumerate() {
            let x = b.var(format!("x{i}"), Op::Trace(i));
            let mut cache = BTreeMap::from([(0, one.clone()), (1, x.clone())]);
            let top = b.expr(i, input.curve.order()?, &x, &mut cache);
            if betas.len() == 1 {
                b.system
                    .equations
                    .push(&top - &Poly::constant(p, betas[0].clone()));
            } else {
                let v = b.var(format!("v{i}"), Op::Value(i));
                let l = b.var(format!("ml{i}"), Op::Left(i));
                let r = b.var(format!("mr{i}"), Op::Right(i));
                b.system.equations.extend([
                    &v - &top,
                    &l - &(&(&v - &Poly::constant(p, betas[0].clone()))
                        * &(&v - &Poly::constant(p, betas[1].clone()))),
                    &r - &(&(&v - &Poly::constant(p, betas[2].clone()))
                        * &(&v - &Poly::constant(p, betas[3].clone()))),
                    &l * &r,
                ]);
            }
            let q = b.var(format!("densq{i}"), Op::Den(i));
            let d = &ap - &(&am * y);
            b.system.equations.push(&q - &(&d * &d));
            let z = &one - y;
            let coeff = Poly::constant(p, f.scale(&f.square(&a), 2));
            b.system
                .equations
                .push(&(&q * &(&x + &one)) - &(&coeff * &(&z * &z)));
        }
        for i in 0..n - 2 {
            nodes.push(b.var(format!("j{i}"), Op::Internal(i)));
        }
        let squares: Vec<_> = nodes
            .iter()
            .enumerate()
            .map(|(i, y)| {
                let q = b.var(format!("sq{i}"), Op::Square(i));
                b.system.equations.push(&q - &(y * y));
                q
            })
            .collect();
        let t = Poly::constant(p, input.curve.target_y()?);
        let t2 = &t * &t;
        for j in 0..n - 1 {
            let (l, r, z) = edge(n, j);
            let aa = b.var(format!("e{j}a"), Op::A(j));
            let bb = b.var(format!("e{j}b"), Op::B(j));
            b.system.equations.push(
                &(&aa - &(&am * &(&(&squares[l] - &one) * &(&squares[r] - &one))))
                    - &Poly::small(p, 4),
            );
            b.system
                .equations
                .push(&bb + &(&nodes[l] * &nodes[r]).scale(8));
            let zv = z.map(|i| nodes[i].clone()).unwrap_or_else(|| t.clone());
            let zs = z.map(|i| squares[i].clone()).unwrap_or_else(|| t2.clone());
            b.system.equations.push(
                &(&(&aa * &zs) + &(&bb * &zv)) + &(&(&squares[l] + &squares[r]).scale(4) - &aa),
            );
        }
        Ok(Self {
            system: b.system,
            arity: n,
            ops: b.ops,
        })
    }
    pub fn witness(&self, input: &TreeInput, points: &[Point]) -> Result<Vec<BigUint>> {
        if points.len() != self.arity {
            return Err("wrong number of points".into());
        }
        let c = Curve::from_case(&input.curve)?;
        let f = &c.f;
        let a = &c.a;
        let betas = input.beta_values()?;
        let mut sum = None;
        let mut internal = vec![];
        for (i, p) in points.iter().enumerate() {
            if p.is_none() || !c.contains(p) {
                return Err("invalid leaf point".into());
            }
            sum = c.add(&sum, p)?;
            if i >= 1 && i < points.len() - 1 {
                internal.push(sum.clone());
            }
        }
        if sum != input.curve.point()? {
            return Err("point sum differs from target".into());
        }
        let mut yy: Vec<_> = points
            .iter()
            .map(|p| coordinate(p, f))
            .collect::<Result<_>>()?;
        let leaf_y = yy.clone();
        yy.extend(
            internal
                .iter()
                .map(|p| coordinate(p, f))
                .collect::<Result<Vec<_>>>()?,
        );
        let mut den = vec![];
        let mut xx = vec![];
        let mut vv = vec![];
        for y in &leaf_y {
            let d = f.square(&f.sub(&f.add(a, &2u32.into()), &f.mul(&f.sub(a, &2u32.into()), y)));
            let x = f.sub(
                &f.mul(
                    &f.scale(&f.square(a), 2),
                    &f.mul(&f.square(&f.sub(&1u32.into(), y)), &f.inv(&d)?),
                ),
                &1u32.into(),
            );
            den.push(d);
            vv.push(f.chebyshev(input.curve.order()?, &x));
            xx.push(x);
        }
        self.ops
            .iter()
            .map(|op| {
                Ok(match *op {
                    Op::Leaf(i) => leaf_y[i].clone(),
                    Op::Internal(i) => yy[self.arity + i].clone(),
                    Op::Trace(i) => xx[i].clone(),
                    Op::Cheb(i, n) => f.chebyshev(n, &xx[i]),
                    Op::Den(i) => den[i].clone(),
                    Op::Value(i) => vv[i].clone(),
                    Op::Left(i) => f.mul(&f.sub(&vv[i], &betas[0]), &f.sub(&vv[i], &betas[1])),
                    Op::Right(i) => f.mul(&f.sub(&vv[i], &betas[2]), &f.sub(&vv[i], &betas[3])),
                    Op::Square(i) => f.square(&yy[i]),
                    Op::A(j) => {
                        let (l, r, _) = edge(self.arity, j);
                        f.add(
                            &f.mul(
                                &f.sub(a, &2u32.into()),
                                &f.mul(
                                    &f.sub(&f.square(&yy[l]), &1u32.into()),
                                    &f.sub(&f.square(&yy[r]), &1u32.into()),
                                ),
                            ),
                            &4u32.into(),
                        )
                    }
                    Op::B(j) => {
                        let (l, r, _) = edge(self.arity, j);
                        f.sub(&0u32.into(), &f.scale(&f.mul(&yy[l], &yy[r]), 8))
                    }
                })
            })
            .collect()
    }
}
/// Public known-answer four-coset construction. It is not a uniform target sample.
pub fn actual_four_control() -> Result<(TorusParameters, TreeInput, TreeSystem, Vec<Point>)> {
    let params = TorusParameters::actual_four()?;
    let points = (1u32..=4)
        .map(|j| params.point(j, 17 * j as u128 + 1))
        .collect::<Result<Vec<_>>>()?;
    let input = TreeInput::from_points(
        params.f.p.clone(),
        486662u32.into(),
        params.m,
        params.betas.clone(),
        &points,
    )?;
    let system = TreeSystem::compile(&input)?;
    system.system.check(&system.witness(&input, &points)?)?;
    Ok((params, input, system, points))
}

// Reuse the repository's native identity-certificate implementation unchanged.
#[allow(dead_code)]
#[path = "../../../src/hash/sha256.rs"]
mod native_sha256;
pub mod hash {
    pub mod sha256 {
        pub use crate::native_sha256::sha256;
    }
}
#[path = "../../../src/cryptanalysis/identity_certificate.rs"]
pub mod identity;

pub fn s3_statement() -> identity::IdentityStatement {
    use identity::Expr as E;
    let add = |v: Vec<E>| E::Add(v);
    let mul = |v: Vec<E>| E::Mul(v);
    let sub = |a: E, b: E| E::plus(a, E::Neg(Box::new(b)));
    let square = |a: E| E::times(a.clone(), a);
    let x = E::var("x");
    let y = E::var("y");
    let z = E::var("z");
    let a = E::var("A");
    let ns = [x.clone(), y.clone(), z.clone()].map(|v| E::plus(E::Const(1), v));
    let ds = [x.clone(), y.clone(), z.clone()].map(|v| sub(E::Const(1), v));
    let s = sub(
        add(vec![
            mul(vec![ns[0].clone(), ns[1].clone(), ds[2].clone()]),
            mul(vec![ns[0].clone(), ns[2].clone(), ds[1].clone()]),
            mul(vec![ns[1].clone(), ns[2].clone(), ds[0].clone()]),
        ]),
        mul(ds.to_vec()),
    );
    let t = add(vec![
        mul(vec![ns[0].clone(), ds[1].clone(), ds[2].clone()]),
        mul(vec![ns[1].clone(), ds[0].clone(), ds[2].clone()]),
        mul(vec![ns[2].clone(), ds[0].clone(), ds[1].clone()]),
        mul(vec![a.clone(), ds[0].clone(), ds[1].clone(), ds[2].clone()]),
    ]);
    let lhs = sub(
        square(s),
        mul(vec![
            E::Const(4),
            ns[0].clone(),
            ns[1].clone(),
            ns[2].clone(),
            t,
        ]),
    );
    let f = add(vec![
        mul(vec![
            sub(a, E::Const(2)),
            sub(square(x.clone()), E::Const(1)),
            sub(square(y.clone()), E::Const(1)),
            sub(square(z.clone()), E::Const(1)),
        ]),
        mul(vec![
            E::Const(4),
            sub(
                add(vec![
                    square(x.clone()),
                    square(y.clone()),
                    square(z.clone()),
                ]),
                E::Const(1),
            ),
        ]),
        mul(vec![E::Const(-8), x, y, z]),
    ]);
    identity::IdentityStatement{name:"Montgomery S3 to Edwards-y, cleared denominators".into(),source:"Known Montgomery S3 and u=(1+y)/(1-y); exact integer expansion tested in torus-s4-tree/tests/identity.rs. No novelty or formal proof claimed.".into(),variables:vec!["A".into(),"x".into(),"y".into(),"z".into()],degree_bound:7,lhs,rhs:mul(vec![E::Const(4),f])}
}

pub mod energy;
pub mod transport;

/// Cleared equation of either elliptic quotient of the genus-two cover.
pub fn elliptic_map_statement(plus: bool) -> identity::IdentityStatement {
    use identity::Expr as E;
    let add = |v: Vec<E>| E::Add(v);
    let mul = |v: Vec<E>| E::Mul(v);
    let sub = |a: E, b: E| E::plus(a, E::Neg(Box::new(b)));
    let sq = |a: E| E::times(a.clone(), a);
    let a = E::var("A");
    let z = E::var("z");
    let n = E::plus(z.clone(), E::Const(if plus { 1 } else { -1 }));
    let d = E::plus(sq(z.clone()), E::Const(1));
    let b = mul(vec![
        d.clone(),
        sub(
            mul(vec![E::Const(4), sq(d.clone())]),
            mul(vec![sq(a.clone()), sq(sub(sq(z), E::Const(1)))]),
        ),
    ]);
    let lhs = mul(vec![E::Const(-2), a.clone(), sq(n.clone()), b]);
    let a3 = mul(vec![a.clone(), a.clone(), a.clone()]);
    let n2 = sq(n);
    let n4 = sq(n2.clone());
    let n6 = mul(vec![n2.clone(), n4.clone()]);
    let rhs = add(vec![
        mul(vec![E::Const(-2), a3.clone(), n6, d.clone()]),
        mul(vec![E::Const(4), a3, n4, sq(d.clone())]),
        mul(vec![E::Const(-8), a, n2, d.clone(), d.clone(), d]),
    ]);
    identity::IdentityStatement{name:format!("Genus-two elliptic quotient {}: cleared equation",if plus{"plus"}else{"minus"}),source:"u=-A(z+/-1)^2/(2(z^2+1)), v=(z+/-1)V/(4(z^2+1)^2), V^2=-2AB(z). Exact integer expansion; no formal geometric proof claimed.".into(),variables:vec!["A".into(),"z".into()],degree_bound:11,lhs,rhs}
}
