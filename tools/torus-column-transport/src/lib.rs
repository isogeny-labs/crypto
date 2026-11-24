//! Original cofactor-fibre aliases and public-Q pair queries, not fixed-base IC.
pub mod controls;
pub mod linear;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use torus_coset_circuit::Spec;
use torus_reverse_multiplier::{curve, fp2_json, point_json, raw_choices, verify_lift, Lift};
use torus_s4::{curve25519_prime, decimal, Curve, Point};
use torus_s4_tree::Fp2;
pub type Result<T> = std::result::Result<T, String>;
pub type Key = (BigUint, BigUint);

pub fn subgroup_order(actual: bool) -> Result<BigUint> {
    Ok(if actual {
        (BigUint::one() << 252) + decimal("27742317777372353535851937790883648493")?
    } else {
        13u32.into()
    })
}

pub fn point_key(p: &Point) -> String {
    point_json(p).to_string()
}
pub fn neg(cv: &Curve, p: &Point) -> Point {
    p.as_ref()
        .map(|(x, y)| (x.clone(), cv.f.sub(&BigUint::zero(), y)))
}
pub fn scalar(cv: &Curve, p: &Point, n: &BigUint) -> Result<Point> {
    if n.bits() > 256 || !cv.contains(p) {
        return Err("scalar/point admission".into());
    }
    let mut bits = n.clone();
    let mut a = p.clone();
    let mut r = None;
    while !bits.is_zero() {
        if bits.bit(0) {
            r = cv.add(&r, &a)?;
        }
        bits >>= 1;
        if !bits.is_zero() {
            a = cv.add(&a, &a)?;
        }
    }
    Ok(r)
}
pub fn column(cv: &Curve, p: &Point) -> Result<(Key, i64)> {
    let (x, y) = p.as_ref().ok_or("identity column")?;
    if !cv.contains(p) {
        return Err("invalid column".into());
    }
    let ny = cv.f.sub(&BigUint::zero(), y);
    Ok(if y <= &ny {
        ((x.clone(), y.clone()), 1)
    } else {
        ((x.clone(), ny), -1)
    })
}
pub struct Family {
    pub s: Spec,
    pub cv: Curve,
    pub ell: BigUint,
    pub g: Point,
    pub torsion: Vec<Point>,
    pub inv8: BigUint,
}
impl Family {
    pub fn new(s: &Spec) -> Result<Self> {
        let cv = curve(s)?;
        let ell = subgroup_order(s.p == curve25519_prime())?;
        let mut torsion_generator = None;
        let mut toy_g = None;
        for x in 0u64..64 {
            if let Some(y) = cv.f.sqrt(&cv.rhs(&x.into()))? {
                let p = Some((x.into(), y));
                let t = scalar(&cv, &p, &ell)?;
                if cv.times(&t, 8)?.is_none()
                    && cv.times(&t, 4)?.is_some()
                    && torsion_generator.is_none()
                {
                    torsion_generator = Some(t);
                }
                let image = cv.times(&p, 8)?;
                if image.is_some() && toy_g.is_none() {
                    toy_g = Some(image);
                }
                if torsion_generator.is_some() && toy_g.is_some() {
                    break;
                }
            }
        }
        let t = torsion_generator.ok_or("fixed torsion search cap; incomplete")?;
        let torsion = (0..8)
            .map(|i| cv.times(&t, i))
            .collect::<Result<Vec<_>>>()?;
        if torsion.iter().map(point_key).collect::<BTreeSet<_>>().len() != 8 {
            return Err("torsion distinctness".into());
        }
        let g = if s.p == curve25519_prime() {
            Some((
                9u32.into(),
                decimal(
                    "14781619447589544791020593568409986887264606134616475288964881837755586237401",
                )?,
            ))
        } else {
            toy_g.ok_or("toy generator search cap")?
        };
        if !cv.contains(&g) || g.is_none() || scalar(&cv, &g, &ell)?.is_some() {
            return Err("subgroup generator replay".into());
        }
        let inv8 = BigUint::from(8u32).modpow(&(&ell - 2u32), &ell);
        let explicit_inverse = (&ell * 3u32 + 1u32) / 8u32;
        if inv8 != explicit_inverse || (&inv8 * 8u32) % &ell != BigUint::one() {
            return Err("inverse8 replay".into());
        }
        Ok(Self {
            s: s.clone(),
            cv,
            ell,
            g,
            torsion,
            inv8,
        })
    }
    pub fn check_image(&self, q: &Point) -> Result<()> {
        if q.is_none() || !self.cv.contains(q) || scalar(&self.cv, q, &self.ell)?.is_some() {
            return Err("nonzero prime-subgroup image required".into());
        }
        Ok(())
    }
    pub fn sectors(&self, q: &Point) -> Result<Vec<Point>> {
        self.check_image(q)?;
        let r0 = scalar(&self.cv, q, &self.inv8)?;
        let out = self
            .torsion
            .iter()
            .map(|t| self.cv.add(&r0, t))
            .collect::<Result<Vec<_>>>()?;
        if out.iter().map(point_key).collect::<BTreeSet<_>>().len() != 8 {
            return Err("sector distinctness".into());
        }
        for r in &out {
            if self.cv.times(r, 8)? != *q {
                return Err("sector image replay".into());
            }
        }
        Ok(out)
    }
    pub fn encodings(&self, image: &Point) -> Result<Vec<Encoding>> {
        let mut out = BTreeMap::new();
        let f = &self.cv.f;
        for (sector, raw) in self.sectors(image)?.into_iter().enumerate() {
            let (x, _) = raw.as_ref().ok_or("raw fibre identity")?;
            let d = f.add(&f.scale(x, 2), &self.cv.a);
            if d.is_zero() {
                continue;
            }
            let aa = f.mul(&self.cv.a, &f.inv(&d)?);
            for a in [aa.clone(), f.sub(&BigUint::zero(), &aa)] {
                let square = f.mul(
                    &f.sub(&f.square(&a), &BigUint::one()),
                    &f.inv(&2u32.into())?,
                );
                let Some(b) = f.sqrt(&square)? else {
                    continue;
                };
                for bb in [b.clone(), f.sub(&BigUint::zero(), &b)] {
                    let z = Fp2 {
                        a: a.clone(),
                        b: bb,
                    };
                    if z.norm(f) != BigUint::one() {
                        return Err("inverse encoding norm".into());
                    }
                    if let Some((branch, _)) = raw_choices(&self.cv, &z)?
                        .into_iter()
                        .find(|(_, p)| *p == raw)
                    {
                        out.insert(
                            fp2_json(&z).to_string(),
                            Encoding {
                                z,
                                raw: raw.clone(),
                                sector,
                                branch,
                            },
                        );
                    }
                }
            }
        }
        if out.len() > 32 {
            return Err("inverse encoding bound32".into());
        }
        Ok(out.into_values().collect())
    }
}
pub struct Encoding {
    pub z: Fp2,
    pub raw: Point,
    pub sector: usize,
    pub branch: usize,
}
impl Encoding {
    pub fn json(&self) -> Value {
        json!({"z":fp2_json(&self.z),"raw":point_json(&self.raw),"fibre_sector":self.sector,"branch":self.branch})
    }
}
pub struct Query {
    pub lifts: Vec<Lift>,
    pub record: Value,
}
/// Public prime-group target only; the source family's raw R is not supplied.
pub fn query(f: &Family, q: &Point, cap: u64) -> Result<Query> {
    let sectors = f.sectors(q)?;
    let mut records = vec![];
    let mut values = BTreeMap::new();
    for (i, r) in sectors.iter().enumerate() {
        let a = torus_reverse_multiplier::solve(&f.s, r, cap)?;
        for v in a.lifts {
            values.insert(v.key(), v);
        }
        records.push(json!({"sector":i,"query":a.record}));
    }
    let lifts: Vec<_> = values.into_values().collect();
    let rows = lifts
        .iter()
        .map(|v| v.row_key(&f.cv.f))
        .collect::<Result<BTreeSet<_>>>()?;
    let record = json!({"Q":point_json(q),"raw_targets":sectors.iter().map(point_json).collect::<Vec<_>>(),"sector_count":8,"sectors":records,"ordered_multiplier_raw_lifts":lifts.len(),"geometric_rows":rows.len(),"scope":"complete fixed-template raw-sector union when all eight root calls finish; adapted bases, no fixed-base or cost claim"});
    Ok(Query { lifts, record })
}
pub fn transport(f: &Family, q: &Point, v: &Lift) -> Result<(BTreeSet<String>, Value)> {
    if f.cv.add(&v.images[0], &v.images[1])? != *q {
        return Err("transport input image sum".into());
    }
    let enc = [f.encodings(&v.images[0])?, f.encodings(&v.images[1])?];
    let sectors = f.sectors(q)?;
    let role = |es: &[Encoding], i: u64| -> Result<BTreeMap<String, Fp2>> {
        let hi = f.s.h.pow(&i.into(), &f.cv.f).inverse(&f.cv.f)?;
        Ok(es
            .iter()
            .map(|e| {
                let sigma = e.z.mul(&hi, &f.cv.f);
                (fp2_json(&sigma).to_string(), sigma)
            })
            .collect())
    };
    let mut candidates = BTreeMap::new();
    for (i, j) in [(1, 2), (2, 1)] {
        let a = role(&enc[0], i)?;
        let b = role(&enc[1], j)?;
        for (key, sigma) in a {
            if b.contains_key(&key) {
                candidates.insert(key, sigma);
            }
        }
    }
    if candidates.len() > 64 {
        return Err("transport role bound64".into());
    }
    let expected = v.row_key(&f.cv.f)?;
    let mut found = BTreeMap::new();
    for sigma in candidates.values() {
        let z1 = sigma.mul(&f.s.h, &f.cv.f);
        let z2 = z1.mul(&f.s.h, &f.cv.f);
        for (b1, p1) in raw_choices(&f.cv, &z1)? {
            for (b2, p2) in raw_choices(&f.cv, &z2)? {
                let images = [f.cv.times(&p1, 8)?, f.cv.times(&p2, 8)?];
                if f.cv.add(&images[0], &images[1])? != *q {
                    continue;
                }
                let value = Lift {
                    sigma: sigma.clone(),
                    raw: [p1.clone(), p2],
                    images,
                    branches: [b1, b2],
                    base_key: sigma.pow(&f.s.q.into(), &f.cv.f),
                };
                if value.row_key(&f.cv.f)? != expected {
                    continue;
                }
                let r = f.cv.add(&value.raw[0], &value.raw[1])?;
                let sector = sectors
                    .iter()
                    .position(|x| *x == r)
                    .ok_or("transport missing raw sector")?;
                verify_lift(&f.s, &f.cv, &r, &value)?;
                found.insert(
                    value.key(),
                    json!({"sector":sector,"lift":value.json(&f.cv.f)?}),
                );
            }
        }
    }
    let keys = found.keys().cloned().collect();
    Ok((
        keys,
        json!({"row":expected,"inverse_encodings":[enc[0].iter().map(Encoding::json).collect::<Vec<_>>(),enc[1].iter().map(Encoding::json).collect::<Vec<_>>()],"candidate_sigma":candidates.values().map(fp2_json).collect::<Vec<_>>(),"transport_lifts":found.into_values().collect::<Vec<_>>(),"scope":"finite full original-map cofactor/role aliases for this geometric row"}),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn toy_fibres_and_small_scalars() {
        let s = torus_quotient_query::controls::spec(false, 8).unwrap();
        let f = Family::new(&s).unwrap();
        assert_eq!(f.sectors(&f.g).unwrap().len(), 8);
        for n in 0..32 {
            assert_eq!(
                scalar(&f.cv, &f.g, &BigUint::from(n)).unwrap(),
                f.cv.times(&f.g, n).unwrap()
            );
        }
        assert!(f.sectors(&None).is_err());
    }
    #[test]
    fn original_role_is_in_complete_inverse_fibre() {
        let s = torus_quotient_query::controls::spec(false, 8).unwrap();
        let f = Family::new(&s).unwrap();
        let (_, p) = raw_choices(&f.cv, &s.h).unwrap().remove(0);
        let image = f.cv.times(&p, 8).unwrap();
        let enc = f.encodings(&image).unwrap();
        assert!(enc.len() <= 32);
        assert!(enc.iter().any(|e| fp2_json(&e.z) == fp2_json(&s.h)));
    }
}
