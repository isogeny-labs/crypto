//! Complete extended Edwards composition, under the documented curve contracts.
use crate::index::{negate, IndexTable, IndexedTerm};
use crate::{Builder, Circuit, Op, Result, Spec, WitnessInput};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use torus_s4::{Curve, Field, Point, Poly, System};

pub fn shifted(p: &Poly, offset: usize) -> Poly {
    let mut r = p.clone();
    r.terms = p
        .terms
        .iter()
        .map(|(m, c)| (m.iter().map(|i| i + offset).collect(), c.clone()))
        .collect();
    r
}
#[derive(Clone)]
struct Ext {
    x: Poly,
    y: Poly,
    z: Poly,
    t: Poly,
}
struct Geometry {
    f: Field,
    c: BigUint,
    d: BigUint,
    curve: Curve,
}
impl Geometry {
    fn new(s: &Spec) -> Result<Self> {
        let f = Field::new(s.p.clone())?;
        let a = s.curve_a.clone().ok_or("curve coefficient missing")?;
        if (&s.p % 4u32) != BigUint::one() || f.square(&a) == 4u32.into() {
            return Err("complete Edwards curve contracts fail".into());
        }
        let plus = f.add(&a, &2u32.into());
        let root = f
            .sqrt(&f.sub(&0u32.into(), &plus))?
            .ok_or("Montgomery conversion root absent")?;
        let c = root.clone().min(f.sub(&0u32.into(), &root));
        let d = f.mul(
            &f.sub(&0u32.into(), &f.sub(&a, &2u32.into())),
            &f.inv(&plus)?,
        );
        if d.modpow(&((&f.p - 1u32) >> 1), &f.p) != &f.p - 1u32 {
            return Err("Edwards d is not a nonsquare".into());
        }
        Ok(Self {
            curve: Curve { f: f.clone(), a },
            f,
            c,
            d,
        })
    }
    fn target(&self, p: &Point) -> Result<(BigUint, BigUint)> {
        if !self.curve.contains(p) {
            return Err("public target is not on the curve".into());
        }
        match p {
            None => Ok((BigUint::zero(), BigUint::one())),
            Some((u, v)) => Ok((
                self.f.mul(&self.f.mul(&self.c, u), &self.f.inv(v)?),
                self.f.mul(
                    &self.f.sub(u, &1u32.into()),
                    &self.f.inv(&self.f.add(u, &1u32.into()))?,
                ),
            )),
        }
    }
}
fn curve_equation(b: &mut Builder, p: &Ext, d: &Poly) {
    let lhs = &(&(&p.y * &p.y) - &(&p.x * &p.x)) - &(&p.z * &p.z);
    b.system.equations.push(&lhs - &(d * &(&p.t * &p.t)));
}
fn convert(b: &mut Builder, u: &Poly, v: &Poly, c: &Poly, d: &Poly) -> Ext {
    let iv = b.alloc("chart_inv_v", Op::Inverse(v.clone()));
    let iu = b.alloc("chart_inv_up1", Op::Inverse(u + &b.c(1)));
    let x = b.alloc("ed_x", Op::Expr(&(c * u) * &iv));
    let y = b.alloc("ed_y", Op::Expr(&(u - &b.c(1)) * &iu));
    let t = b.alloc("ed_t", Op::Expr(&x * &y));
    let p = Ext { x, y, z: b.c(1), t };
    curve_equation(b, &p, d);
    p
}
fn add(b: &mut Builder, l: &Ext, r: &Ext, d: &Poly) -> Ext {
    // RFC8032 section5.1.4. All eight products have named defining gates.
    let a = b.alloc("add_a", Op::Expr(&(&l.y - &l.x) * &(&r.y - &r.x)));
    let bb = b.alloc("add_b", Op::Expr(&(&l.y + &l.x) * &(&r.y + &r.x)));
    let c = b.alloc("add_c", Op::Expr((d * &(&l.t * &r.t)).scale(2)));
    let dd = b.alloc("add_d", Op::Expr((&l.z * &r.z).scale(2)));
    let e = &bb - &a;
    let f = &dd - &c;
    let g = &dd + &c;
    let h = &bb + &a;
    let x = b.alloc("sum_x", Op::Expr(&e * &f));
    let y = b.alloc("sum_y", Op::Expr(&g * &h));
    let t = b.alloc("sum_t", Op::Expr(&e * &h));
    let z = b.alloc("sum_z", Op::Expr(&f * &g));
    b.alloc("sum_inv_z", Op::Inverse(z.clone()));
    b.system.equations.push(&(&x * &y) - &(&z * &t));
    let p = Ext { x, y, z, t };
    curve_equation(b, &p, d);
    p
}

pub struct TargetCircuit {
    pub system: System,
    member: Circuit,
    spec: Spec,
    target: Point,
    geometry: Geometry,
    extra: Vec<Op>,
    pub arity: usize,
    pub distinct: bool,
}
impl TargetCircuit {
    pub fn compile(s: &Spec, arity: usize, target: Point, distinct: bool) -> Result<Self> {
        if !(2..=16).contains(&arity) {
            return Err("arity outside fixed interface cap".into());
        }
        let geometry = Geometry::new(s)?;
        let (qx, qy) = geometry.target(&target)?;
        let member = Circuit::compile(s)?;
        let n = member.system.variables.len();
        let image = member
            .image_polys()
            .ok_or("member circuit has no cofactor image")?;
        let parameter = member.parameter_polys();
        let mut system = System {
            p: s.p.clone(),
            variables: vec![],
            equations: vec![],
        };
        let mut points = vec![];
        let mut parameters = vec![];
        for i in 0..arity {
            system.variables.extend(
                member
                    .system
                    .variables
                    .iter()
                    .map(|v| format!("member{i}_{v}")),
            );
            system
                .equations
                .extend(member.system.equations.iter().map(|p| shifted(p, i * n)));
            points.push((shifted(&image.0, i * n), shifted(&image.1, i * n)));
            parameters.push((shifted(&parameter.0, i * n), shifted(&parameter.1, i * n)));
        }
        let mut b = Builder {
            system,
            ops: vec![],
            pair_muls: 0,
        };
        if distinct {
            for i in 0..arity {
                for j in i + 1..arity {
                    let da = &parameters[i].0 - &parameters[j].0;
                    let db = &parameters[i].1 - &parameters[j].1;
                    let norm = b.alloc(
                        "distinct_norm",
                        Op::Expr(&(&da * &da) - &(&db * &db).scale(2)),
                    );
                    b.alloc("distinct_inverse", Op::Inverse(norm));
                }
            }
        }
        let c = b.constant(geometry.c.clone());
        let d = b.constant(geometry.d.clone());
        let mut converted = points.iter().map(|(u, v)| convert(&mut b, u, v, &c, &d));
        let mut sum = converted.next().unwrap();
        // Finish conversion before borrowing the builder again for additions.
        let rest: Vec<_> = converted.collect();
        for p in rest {
            sum = add(&mut b, &sum, &p, &d);
        }
        b.system
            .equations
            .push(&sum.x - &(&b.constant(qx) * &sum.z));
        b.system
            .equations
            .push(&sum.y - &(&b.constant(qy) * &sum.z));
        Ok(Self {
            system: b.system,
            extra: b.ops,
            member,
            spec: s.clone(),
            target,
            geometry,
            arity,
            distinct,
        })
    }
    pub fn witness(&self, inputs: &[WitnessInput]) -> Result<Vec<BigUint>> {
        if inputs.len() != self.arity {
            return Err("wrong member-input count".into());
        }
        let mut values = vec![];
        for input in inputs {
            values.extend(self.member.witness(input)?);
        }
        for op in &self.extra {
            values.push(match op {
                Op::Expr(p) => p.evaluate(&values)?,
                Op::Inverse(p) => self.geometry.f.inv(&p.evaluate(&values)?)?,
                Op::Input(_) => return Err("unexpected aggregate input gate".into()),
            });
        }
        Ok(values)
    }
    pub fn decode_row(&self, values: &[BigUint], table: &IndexTable) -> Result<Vec<IndexedTerm>> {
        self.system.check(values)?;
        let n = self.member.system.variables.len();
        let mut row: BTreeMap<(u64, u64), IndexedTerm> = BTreeMap::new();
        for i in 0..self.arity {
            let term = table.decode(&self.spec, &self.member, &values[i * n..(i + 1) * n])?;
            let entry = row.entry((term.j, term.t)).or_insert(IndexedTerm {
                coefficient: 0,
                ..term.clone()
            });
            if entry.point != term.point {
                return Err("raw label replay changed canonical point".into());
            }
            entry.coefficient += term.coefficient;
        }
        let row: Vec<_> = row.into_values().filter(|t| t.coefficient != 0).collect();
        let mut replay = None;
        for term in &row {
            let p = self
                .geometry
                .curve
                .times(&term.point, term.coefficient.unsigned_abs())?;
            replay = self.geometry.curve.add(
                &replay,
                &if term.coefficient < 0 {
                    negate(&p, &self.geometry.f)
                } else {
                    p
                },
            )?;
        }
        if replay != self.target {
            return Err("decoded raw-index row does not replay target".into());
        }
        Ok(row)
    }
    pub fn stats(&self) -> Value {
        json!({"arity":self.arity,"distinct_raw_indices":self.distinct,
            "variables":self.system.variables.len(),"equations":self.system.equations.len(),
            "terms":self.system.equations.iter().map(|p|p.terms.len()).sum::<usize>(),
            "max_degree":self.system.equations.iter().map(Poly::degree).max().unwrap_or(0),
            "member":self.member.stats(),"complete_addition":"RFC8032 section5.1.4",
            "target_discovery_solver":false})
    }
}
