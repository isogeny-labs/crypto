use crate::field_e::*;
use std::collections::BTreeMap;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Poly(pub BTreeMap<(usize, usize), u8>);
impl Poly {
    pub fn zero() -> Self {
        Self(BTreeMap::new())
    }
    pub fn term(y: usize, z: usize, c: u8) -> Self {
        if c == 0 {
            Self::zero()
        } else {
            Self(BTreeMap::from([((y, z), c)]))
        }
    }
    pub fn from_coefficients(basis: &[(usize, usize)], coefficients: &[u8]) -> Self {
        assert_eq!(basis.len(), coefficients.len());
        basis
            .iter()
            .zip(coefficients)
            .fold(Self::zero(), |a, (&(y, z), &c)| {
                a.plus(&Self::term(y, z, c))
            })
    }
    pub fn plus(&self, b: &Self) -> Self {
        let mut r = self.0.clone();
        for (&key, &v) in &b.0 {
            let c = add(r.get(&key).copied().unwrap_or(0), v);
            if c == 0 {
                r.remove(&key);
            } else {
                r.insert(key, c);
            }
        }
        Self(r)
    }
    pub fn scaled(&self, c: u8) -> Self {
        Self(
            self.0
                .iter()
                .filter_map(|(&key, &v)| {
                    let a = mul(c, v);
                    (a != 0).then_some((key, a))
                })
                .collect(),
        )
    }
    pub fn minus(&self, b: &Self) -> Self {
        self.plus(&b.scaled(6))
    }
    pub fn times(&self, b: &Self) -> Self {
        let mut r = Self::zero();
        for (&(y, z), &c) in &self.0 {
            for (&(yy, zz), &d) in &b.0 {
                r = r.plus(&Self::term(y + yy, z + zz, mul(c, d)));
            }
        }
        r
    }
    pub fn power(&self, n: usize) -> Self {
        (0..n).fold(Self::term(0, 0, 1), |a, _| a.times(self))
    }
    pub fn jet(&self, y: Jet, z: Jet) -> Jet {
        self.0
            .iter()
            .map(|(&(a, b), &c)| js(jm(jp(y, a), jp(z, b)), c))
            .fold(jc(0), ja)
    }
    pub fn divide_curve(&self, curve: &Self) -> (Self, Self) {
        assert_eq!(curve.0.get(&(0, 4)), Some(&1));
        assert!(curve.0.keys().all(|(y, z)| *z < 4 || (*y, *z) == (0, 4)));
        let mut remainder = self.clone();
        let mut quotient = Self::zero();
        loop {
            let term = remainder
                .0
                .iter()
                .filter(|((_, z), _)| *z >= 4)
                .max_by_key(|((y, z), _)| (*z, *y))
                .map(|(&(y, z), &c)| (y, z, c));
            let Some((y, z, c)) = term else {
                break;
            };
            let factor = Self::term(y, z - 4, c);
            quotient = quotient.plus(&factor);
            remainder = remainder.minus(&curve.times(&factor));
        }
        assert_eq!(curve.times(&quotient).plus(&remainder), *self);
        (quotient, remainder)
    }
}
pub fn basis(degree: usize) -> Vec<(usize, usize)> {
    (0..=degree)
        .flat_map(|y| (0..=degree - y).map(move |z| (y, z)))
        .collect()
}
pub fn actual_curve() -> Poly {
    [
        (0, 0, 1),
        (3, 0, 4),
        (4, 0, 2),
        (0, 2, 5),
        (1, 2, 6),
        (2, 2, 2),
        (0, 4, 1),
    ]
    .into_iter()
    .fold(Poly::zero(), |a, (y, z, c)| a.plus(&Poly::term(y, z, c)))
}
