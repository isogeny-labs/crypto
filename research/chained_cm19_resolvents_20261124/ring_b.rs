use crate::field_b::*;
use std::collections::BTreeMap;

/// Actual coordinate ring F7[u,v]/(v^2-u^3-2u-6).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct R(pub BTreeMap<(usize, usize), u8>);
impl R {
    pub fn zero() -> Self {
        Self(BTreeMap::new())
    }
    pub fn term(u: usize, v: usize, c: u8) -> Self {
        assert!(v < 2 && c < 7);
        if c == 0 {
            Self::zero()
        } else {
            Self(BTreeMap::from([((u, v), c)]))
        }
    }
    pub fn constant(c: u8) -> Self {
        Self::term(0, 0, c)
    }
    pub fn plus(&self, b: &Self) -> Self {
        let mut out = self.0.clone();
        for (&key, &c) in &b.0 {
            let c = add(out.get(&key).copied().unwrap_or(0), c);
            if c == 0 {
                out.remove(&key);
            } else {
                out.insert(key, c);
            }
        }
        Self(out)
    }
    pub fn scaled(&self, c: u8) -> Self {
        Self(
            self.0
                .iter()
                .filter_map(|(&k, &v)| {
                    let c = mul(c, v);
                    (c != 0).then_some((k, c))
                })
                .collect(),
        )
    }
    pub fn minus(&self, b: &Self) -> Self {
        self.plus(&b.scaled(sub(0, 1)))
    }
    pub fn times(&self, b: &Self) -> Self {
        let mut out = Self::zero();
        for (&(u, v), &c) in &self.0 {
            for (&(uu, vv), &d) in &b.0 {
                let c = mul(c, d);
                if v + vv < 2 {
                    out = out.plus(&Self::term(u + uu, v + vv, c));
                } else {
                    for (degree, coeff) in [(3, 1), (1, 2), (0, 6)] {
                        out = out.plus(&Self::term(u + uu + degree, 0, mul(c, coeff)));
                    }
                }
            }
        }
        out
    }
    pub fn power(&self, n: usize) -> Self {
        (0..n).fold(Self::constant(1), |p, _| p.times(self))
    }
}
pub fn binomial(n: usize, k: usize) -> u8 {
    let v = (0..k).fold(1usize, |a, j| a * (n - j) / (j + 1));
    u8::try_from(v % 7).unwrap()
}
pub fn poly_add(a: &[R], b: &[R]) -> Vec<R> {
    (0..a.len().max(b.len()))
        .map(|i| {
            a.get(i)
                .cloned()
                .unwrap_or(R::zero())
                .plus(&b.get(i).cloned().unwrap_or(R::zero()))
        })
        .collect()
}
pub fn linear_power(constant: &R, n: usize) -> Vec<R> {
    (0..=n)
        .map(|i| constant.power(n - i).scaled(binomial(n, i)))
        .collect()
}
pub fn shift(poly: &[R], constant: &R) -> Vec<R> {
    poly.iter()
        .enumerate()
        .fold(vec![R::zero(); poly.len()], |a, (n, c)| {
            poly_add(
                &a,
                &linear_power(constant, n)
                    .iter()
                    .map(|v| v.times(c))
                    .collect::<Vec<_>>(),
            )
        })
}
fn determinant(m: &[Vec<R>]) -> R {
    if m.len() == 1 {
        return m[0][0].clone();
    }
    m[0].iter()
        .enumerate()
        .filter(|(_, r)| !r.0.is_empty())
        .fold(R::zero(), |out, (j, c)| {
            let minor: Vec<Vec<R>> = m
                .iter()
                .skip(1)
                .map(|row| {
                    row.iter()
                        .enumerate()
                        .filter(|&(k, _)| k != j)
                        .map(|(_, v)| v.clone())
                        .collect()
                })
                .collect();
            let p = c.times(&determinant(&minor));
            if j.is_multiple_of(2) {
                out.plus(&p)
            } else {
                out.minus(&p)
            }
        })
}
pub fn discriminant(name: &str, f: &[R]) -> R {
    let n = f.len() - 1;
    assert_eq!(f[n], R::constant(1));
    let derivative: Vec<R> = f
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, c)| c.scaled(u8::try_from(i % 7).unwrap()))
        .collect();
    let mut sylvester = vec![vec![R::zero(); 2 * n - 1]; 2 * n - 1];
    for (row, values) in sylvester.iter_mut().enumerate() {
        let (start, coeff) = if row < n - 1 {
            (row, f)
        } else {
            (row - (n - 1), derivative.as_slice())
        };
        for (i, c) in coeff.iter().rev().enumerate() {
            values[start + i] = c.clone();
        }
    }
    let resultant = determinant(&sylvester);
    let disc = if (n * (n - 1) / 2).is_multiple_of(2) {
        resultant
    } else {
        resultant.scaled(6)
    };
    println!("FULL_SYLVESTER_DISCRIMINANT\t{name}\tmatrix={sylvester:?}\tdiscriminant={disc:?}");
    disc
}

/// Monic quartic root algebra over R; no arbitrary function-field inversion.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct K(pub [R; 4]);
impl K {
    pub fn constant(c: R) -> Self {
        Self([c, R::zero(), R::zero(), R::zero()])
    }
    pub fn root() -> Self {
        Self([R::zero(), R::constant(1), R::zero(), R::zero()])
    }
    pub fn plus(&self, b: &Self) -> Self {
        Self(std::array::from_fn(|i| self.0[i].plus(&b.0[i])))
    }
    pub fn scaled(&self, c: &R) -> Self {
        Self(self.0.clone().map(|v| v.times(c)))
    }
    pub fn times(&self, b: &Self, f: &[R]) -> Self {
        let mut raw: [R; 7] = std::array::from_fn(|_| R::zero());
        for (i, a) in self.0.iter().enumerate() {
            for (j, b) in b.0.iter().enumerate() {
                raw[i + j] = raw[i + j].plus(&a.times(b));
            }
        }
        for d in (4..7).rev() {
            let leading = raw[d].clone();
            for (i, c) in f.iter().take(4).enumerate() {
                raw[d - 4 + i] = raw[d - 4 + i].minus(&leading.times(c));
            }
            raw[d] = R::zero();
        }
        Self(std::array::from_fn(|i| raw[i].clone()))
    }
    pub fn power(&self, n: usize, f: &[R]) -> Self {
        (0..n).fold(Self::constant(R::constant(1)), |v, _| v.times(self, f))
    }
}
pub fn kpoly_mul(a: &[K], b: &[K], f: &[R]) -> Vec<K> {
    let mut out = vec![K::constant(R::zero()); a.len() + b.len() - 1];
    for (i, a) in a.iter().enumerate() {
        for (j, b) in b.iter().enumerate() {
            out[i + j] = out[i + j].plus(&a.times(b, f));
        }
    }
    out
}
