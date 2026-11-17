//! Raw-label adapter using HAC Algorithm 3.56; no elliptic discrete log.
use crate::{Circuit, Result, Spec};
use num_bigint::BigUint;
use num_traits::One;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use torus_s4::{Curve, Field, Point};
use torus_s4_tree::Fp2;

const MAX_BABIES: u64 = 1 << 20;
type Key = (BigUint, BigUint);
fn key(x: &Fp2) -> Key {
    (x.a.clone(), x.b.clone())
}

pub struct IndexTable {
    f: Field,
    generator: Fp2,
    q: u64,
    n: u64,
    babies: BTreeMap<Key, u64>,
    giant: Fp2,
}
#[derive(Clone, Debug)]
pub struct IndexedTerm {
    pub j: u64,
    pub t: u64,
    pub coefficient: i64,
    pub point: Point,
}
impl IndexTable {
    pub fn new(s: &Spec) -> Result<Self> {
        if s.q == 0 {
            return Err("zero subgroup order".into());
        }
        let n = (s.q - 1).isqrt() + 1;
        if n > MAX_BABIES {
            return Err("index baby-table resource cap exceeded".into());
        }
        let f = Field::new(s.p.clone())?;
        let generator = s.h.pow(&s.order_factor.into(), &f);
        if generator.pow(&s.q.into(), &f) != Fp2::one() {
            return Err("index generator is not a qth root".into());
        }
        let mut babies = BTreeMap::new();
        let mut cur = Fp2::one();
        for j in 0..n {
            if babies.insert(key(&cur), j).is_some() {
                return Err("index baby steps repeat".into());
            }
            cur = cur.mul(&generator, &f);
        }
        let giant = generator.pow(&n.into(), &f).inverse(&f)?;
        Ok(Self {
            f,
            generator,
            q: s.q,
            n,
            babies,
            giant,
        })
    }
    pub fn recover(&self, r: &Fp2) -> Result<u64> {
        let mut cur = r.clone();
        for i in 0..self.n {
            if let Some(j) = self.babies.get(&key(&cur)) {
                let t = i
                    .checked_mul(self.n)
                    .and_then(|v| v.checked_add(*j))
                    .ok_or("index arithmetic overflow")?;
                if t < self.q && self.generator.pow(&t.into(), &self.f) == *r {
                    return Ok(t);
                }
            }
            cur = cur.mul(&self.giant, &self.f);
        }
        Err("no subgroup index found".into())
    }
    pub fn stats(&self) -> Value {
        json!({"q":self.q,"baby_entries":self.babies.len(),"giant_step_bound":self.n,
            "baby_entry_cap":MAX_BABIES,"method":"HAC Algorithm3.56 over norm-one Fp2"})
    }
    pub fn decode(&self, s: &Spec, member: &Circuit, values: &[BigUint]) -> Result<IndexedTerm> {
        member.system.check(values)?;
        if self.f.p != s.p
            || self.q != s.q
            || self.generator != s.h.pow(&s.order_factor.into(), &self.f)
        {
            return Err("index table/spec mismatch".into());
        }
        let j = member.selector_value(values)?;
        let (a, b) = member.parameter_polys();
        let z = Fp2 {
            a: a.evaluate(values)?,
            b: b.evaluate(values)?,
        };
        let r = z.mul(&s.h.pow(&j.into(), &self.f).inverse(&self.f)?, &self.f);
        let t = self.recover(&r)?;
        let exponent = BigUint::from(j) + BigUint::from(s.order_factor) * t;
        if j > s.k || s.h.pow(&exponent, &self.f) != z {
            return Err("decoded parameter does not replay".into());
        }
        let canonical = canonical_point(s, j, t)?;
        if canonical.is_none() {
            return Err("identity column returned as a retained member".into());
        }
        let actual = member.image(values)?;
        let coefficient = if actual == canonical {
            1
        } else if actual == negate(&canonical, &self.f) {
            -1
        } else {
            return Err("cofactor image does not match canonical parameter".into());
        };
        Ok(IndexedTerm {
            j,
            t,
            coefficient,
            point: canonical,
        })
    }
}
pub fn negate(p: &Point, f: &Field) -> Point {
    p.as_ref().map(|(u, v)| (u.clone(), f.sub(&0u32.into(), v)))
}
/// Canonical orientation is the smaller raw v root; image aliases stay separate.
pub fn canonical_point(s: &Spec, j: u64, t: u64) -> Result<Point> {
    if j == 0 || j > s.k || t >= s.q {
        return Err("raw column label outside bounds".into());
    }
    let f = Field::new(s.p.clone())?;
    let a = s.curve_a.clone().ok_or("point coefficient missing")?;
    let curve = Curve {
        f: f.clone(),
        a: a.clone(),
    };
    let z =
        s.h.pow(&(BigUint::from(j) + BigUint::from(s.order_factor) * t), &f);
    if z.norm(&f) != BigUint::one() {
        return Err("column outside norm-one torus".into());
    }
    let half = f.mul(&a, &f.inv(&2u32.into())?);
    let center = f.sub(&0u32.into(), &half);
    let delta = f.mul(&half, &f.inv(&z.a)?);
    let mut points = vec![];
    for u in [f.add(&center, &delta), f.sub(&center, &delta)] {
        if let Some(v) = f.sqrt(&curve.rhs(&u))? {
            let negative = f.sub(&0u32.into(), &v);
            points.push(Some((u, v.min(negative))));
        }
    }
    if points.len() != 1 {
        return Err("canonical raw branch is not unique".into());
    }
    curve.times(&points.remove(0), 8)
}
