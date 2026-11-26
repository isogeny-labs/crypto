//! Bounded dynamic polynomials for the NEW coefficient block only.
//! Original quadratic Poly and every frozen compiler remain unchanged.
use crate::Result;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::{cell::Cell, collections::BTreeMap};
use torus_s4::{Field, Poly};

#[derive(Clone, Debug)]
pub struct P(pub(crate) BTreeMap<Vec<usize>, BigUint>);
#[derive(Clone, Copy)]
pub struct Limits {
    pub terms: usize,
    pub pairs: u64,
    pub degree: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            terms: 50_000,
            pairs: 20_000_000,
            degree: 12,
        }
    }
}
pub struct E {
    pub f: Field,
    limits: Limits,
    pairs: Cell<u64>,
}
impl E {
    pub fn new(p: BigUint, limits: Limits) -> Result<Self> {
        if limits.terms == 0
            || limits.terms > 50_000
            || limits.pairs > 20_000_000
            || limits.degree > 12
        {
            return Err("sparse limits outside declared ceiling".into());
        }
        Ok(Self {
            f: Field::new(p)?,
            limits,
            pairs: Cell::new(0),
        })
    }
    fn checked(&self, p: P) -> Result<P> {
        if p.0.len() > self.limits.terms || p.degree() > self.limits.degree {
            return Err("sparse term/degree cap; incomplete compilation".into());
        }
        Ok(p)
    }
    pub fn c(&self, v: BigUint) -> P {
        let v = v % &self.f.p;
        P(if v.is_zero() {
            BTreeMap::new()
        } else {
            BTreeMap::from([(vec![], v)])
        })
    }
    pub fn var(&self, i: usize) -> Result<P> {
        self.checked(P(BTreeMap::from([(vec![i], BigUint::one())])))
    }
    pub fn original(&self, p: &Poly) -> Result<P> {
        if p.terms.values().any(|x| x >= &self.f.p) {
            return Err("noncanonical original coefficient".into());
        }
        self.checked(P(p.terms.clone()))
    }
    pub fn add(&self, a: &P, b: &P) -> Result<P> {
        let mut r = a.clone();
        for (m, c) in &b.0 {
            let v = self.f.add(r.0.get(m).unwrap_or(&BigUint::zero()), c);
            if v.is_zero() {
                r.0.remove(m);
            } else {
                r.0.insert(m.clone(), v);
            }
            if r.0.len() > self.limits.terms {
                return Err("sparse term cap".into());
            }
        }
        self.checked(r)
    }
    pub fn neg(&self, p: &P) -> P {
        P(p.0
            .iter()
            .map(|(m, c)| (m.clone(), self.f.sub(&BigUint::zero(), c)))
            .collect())
    }
    pub fn sub(&self, a: &P, b: &P) -> Result<P> {
        self.add(a, &self.neg(b))
    }
    pub fn mul(&self, a: &P, b: &P) -> Result<P> {
        if !a.0.is_empty() && !b.0.is_empty() && a.degree() + b.degree() > self.limits.degree {
            return Err("sparse degree cap before products".into());
        }
        let n =
            a.0.len()
                .checked_mul(b.0.len())
                .ok_or("pair reservation overflow")?;
        let next = self
            .pairs
            .get()
            .checked_add(n.try_into().map_err(|_| "pair width")?)
            .ok_or("pair counter overflow")?;
        if next > self.limits.pairs {
            return Err("sparse product-pair cap before products".into());
        }
        self.pairs.set(next);
        let mut r = BTreeMap::new();
        for (ma, ca) in &a.0 {
            for (mb, cb) in &b.0 {
                let mut m = ma.clone();
                m.extend(mb);
                m.sort_unstable();
                let v = self
                    .f
                    .add(r.get(&m).unwrap_or(&BigUint::zero()), &self.f.mul(ca, cb));
                if v.is_zero() {
                    r.remove(&m);
                } else {
                    r.insert(m, v);
                }
                if r.len() > self.limits.terms {
                    return Err("sparse term cap during products".into());
                }
            }
        }
        self.checked(P(r))
    }
    pub fn pairs(&self) -> u64 {
        self.pairs.get()
    }
}
impl P {
    pub fn degree(&self) -> usize {
        self.0.keys().map(Vec::len).max().unwrap_or(0)
    }
    pub fn evaluate(&self, f: &Field, v: &[BigUint]) -> Result<BigUint> {
        let mut s = BigUint::zero();
        for (m, c) in &self.0 {
            let mut t = c.clone();
            for i in m {
                t = f.mul(&t, v.get(*i).ok_or("missing dynamic variable")?);
            }
            s = f.add(&s, &t);
        }
        Ok(s)
    }
    pub fn encoded(&self) -> Value {
        json!(self
            .0
            .iter()
            .map(|(m, c)| json!([m, c.to_string()]))
            .collect::<Vec<_>>())
    }
}
pub fn det(e: &E, a: &[Vec<P>]) -> Result<P> {
    let n = a.len();
    if n == 0 || n > 5 || a.iter().any(|r| r.len() != n) {
        return Err("determinant shape".into());
    }
    let mut dp = vec![e.c(BigUint::zero()); 1 << n];
    dp[0] = e.c(BigUint::one());
    for mask in 1usize..(1 << n) {
        let row = mask.count_ones() as usize - 1;
        let mut sum = e.c(BigUint::zero());
        for col in 0..n {
            if mask & (1 << col) != 0 {
                let p = e.mul(&a[row][col], &dp[mask ^ (1 << col)])?;
                let below = (mask & ((1 << col) - 1)).count_ones() as usize;
                sum = e.add(
                    &sum,
                    &if (row + below).is_multiple_of(2) {
                        p
                    } else {
                        e.neg(&p)
                    },
                )?;
            }
        }
        dp[mask] = sum;
    }
    Ok(dp.pop().unwrap())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn caps_precede_products() -> Result<()> {
        let e = E::new(
            101u32.into(),
            Limits {
                terms: 1,
                pairs: 0,
                degree: 1,
            },
        )?;
        let x = e.var(0)?;
        assert!(e.mul(&x, &x).is_err());
        assert_eq!(e.pairs(), 0);
        assert!(e.add(&x, &e.var(1)?).is_err());
        let e = E::new(
            101u32.into(),
            Limits {
                terms: 10,
                pairs: 0,
                degree: 2,
            },
        )?;
        assert!(e.mul(&e.var(0)?, &e.var(1)?).is_err());
        assert_eq!(e.pairs(), 0);
        Ok(())
    }
    #[test]
    fn determinant_fixed_matrix() -> Result<()> {
        let e = E::new(101u32.into(), Limits::default())?;
        let a = vec![
            vec![e.c(3u32.into()), e.c(4u32.into())],
            vec![e.c(5u32.into()), e.c(6u32.into())],
        ];
        assert_eq!(det(&e, &a)?.evaluate(&e.f, &[])?, 99u32.into());
        assert!(det(&e, &[]).is_err());
        Ok(())
    }
    #[test]
    fn cramer_residual_and_rank_deficient_reference() -> Result<()> {
        for p in [BigUint::from(101u32), torus_s4::curve25519_prime()] {
            let e = E::new(p, Limits::default())?;
            let a = (0..5)
                .map(|i| {
                    (0..5)
                        .map(|j| e.c(BigUint::from(u32::from(i == j))))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let rhs = [2u32, 3, 5, 7, 11].map(|v| e.c(v.into()));
            let d = det(&e, &a)?;
            assert_eq!(d.evaluate(&e.f, &[])?, BigUint::one());
            let mut sum = e.c(BigUint::zero());
            for col in 0..5 {
                let mut replaced = a.clone();
                for i in 0..5 {
                    replaced[i][col] = rhs[i].clone();
                }
                let n = det(&e, &replaced)?;
                assert_eq!(n.evaluate(&e.f, &[])?, rhs[col].evaluate(&e.f, &[])?);
                sum = e.add(&sum, &n)?;
            }
            assert!(e
                .sub(&sum, &e.mul(&e.c(28u32.into()), &d)?)?
                .evaluate(&e.f, &[])?
                .is_zero());
            assert!(!e
                .sub(&sum, &e.mul(&e.c(29u32.into()), &d)?)?
                .evaluate(&e.f, &[])?
                .is_zero());
            let zero = vec![vec![e.c(BigUint::zero()); 5]; 5];
            let dz = det(&e, &zero)?.evaluate(&e.f, &[])?;
            assert!(dz.is_zero());
            assert!(e.f.inv(&dz).is_err());
            // Two distinct assignments still satisfy the retained zero-row
            // reference; no invertible chart is falsely substituted for it.
            for v in [BigUint::zero(), BigUint::one()] {
                for row in &zero {
                    let mut sum = BigUint::zero();
                    for x in row {
                        sum = e.f.add(&sum, &e.f.mul(&x.evaluate(&e.f, &[])?, &v));
                    }
                    assert!(sum.is_zero());
                }
            }
        }
        Ok(())
    }
}
