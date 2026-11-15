//! Homogeneous coordinate ring of the pinned smooth plane quartic.
use crate::linear::{Row, Span};
use quotient_calibration_preflight::field::F;
use std::collections::BTreeMap;

pub type Monomial = [usize; 3];
pub type Form = Vec<F>;
pub struct Ring {
    pub quartic: Vec<(Monomial, F)>,
    pub monomials: Vec<Vec<Monomial>>,
    pub indices: Vec<BTreeMap<Monomial, usize>>,
    reductions: Vec<BTreeMap<Monomial, Vec<(usize, u8)>>>,
}
pub fn scale(a: F, c: u8) -> F {
    if c == 0 {
        F::ZERO
    } else if c == 1 {
        a
    } else {
        F(a.0.map(|x| (x * c) % 7))
    }
}
fn raw_monomials(d: usize) -> Vec<Monomial> {
    (0..=d)
        .rev()
        .flat_map(|x| (0..=d - x).rev().map(move |y| [x, y, d - x - y]))
        .collect()
}
impl Ring {
    pub fn new(quartic: Vec<(Monomial, F)>, maximum: usize) -> Self {
        assert_eq!(quartic.iter().max_by_key(|(m, _)| m).unwrap().0, [3, 0, 1]);
        assert!(quartic
            .iter()
            .all(|(_, c)| c.0[1..].iter().all(|x| *x == 0)));
        let monomials: Vec<_> = (0..=maximum)
            .map(|d| {
                raw_monomials(d)
                    .into_iter()
                    .filter(|m| m[0] < 3 || m[2] == 0)
                    .collect::<Vec<_>>()
            })
            .collect();
        let indices = monomials
            .iter()
            .map(|m| m.iter().enumerate().map(|(i, m)| (*m, i)).collect())
            .collect();
        let mut result = Self {
            quartic,
            monomials,
            indices,
            reductions: vec![BTreeMap::new(); maximum + 1],
        };
        for d in 0..=maximum {
            for m in raw_monomials(d) {
                result.build_reduction(d, m);
            }
        }
        for d in 2..=maximum {
            assert_eq!(result.dimension(d), if d == 2 { 6 } else { 4 * d - 2 });
        }
        result
    }
    fn build_reduction(&mut self, d: usize, m: Monomial) -> Vec<(usize, u8)> {
        if let Some(r) = self.reductions[d].get(&m) {
            return r.clone();
        }
        let out = if m[0] < 3 || m[2] == 0 {
            vec![(self.indices[d][&m], 1)]
        } else {
            let lead = self
                .quartic
                .iter()
                .find(|(a, _)| *a == [3, 0, 1])
                .unwrap()
                .1;
            let inverse = lead.inv().0[0];
            let mut row = vec![0u8; self.dimension(d)];
            for (term, c) in self.quartic.clone() {
                if term == [3, 0, 1] {
                    continue;
                }
                let n = [m[0] - 3 + term[0], m[1] + term[1], m[2] - 1 + term[2]];
                let factor = (7 - c.0[0]) * inverse % 7;
                for (i, k) in self.build_reduction(d, n) {
                    row[i] = (row[i] + factor * k) % 7;
                }
            }
            row.into_iter()
                .enumerate()
                .filter(|(_, c)| *c != 0)
                .collect()
        };
        self.reductions[d].insert(m, out.clone());
        out
    }
    pub fn dimension(&self, d: usize) -> usize {
        self.monomials[d].len()
    }
    pub fn unit(&self, d: usize, index: usize) -> Form {
        let mut r = vec![F::ZERO; self.dimension(d)];
        r[index] = F::c(1);
        r
    }
    pub fn multiply(&self, a: &[F], da: usize, b: &[F], db: usize) -> Form {
        assert_eq!(a.len(), self.dimension(da));
        assert_eq!(b.len(), self.dimension(db));
        let mut out = vec![F::ZERO; self.dimension(da + db)];
        for (i, x) in a.iter().enumerate().filter(|(_, x)| **x != F::ZERO) {
            for (j, y) in b.iter().enumerate().filter(|(_, y)| **y != F::ZERO) {
                let m =
                    std::array::from_fn(|k| self.monomials[da][i][k] + self.monomials[db][j][k]);
                let value = *x * *y;
                for (k, c) in &self.reductions[da + db][&m] {
                    out[*k] = out[*k] + scale(value, *c);
                }
            }
        }
        out
    }
    pub fn curve_value(&self, p: [F; 3]) -> F {
        self.quartic.iter().fold(F::ZERO, |s, (m, c)| {
            s + *c * p[0].pow(m[0] as u64) * p[1].pow(m[1] as u64) * p[2].pow(m[2] as u64)
        })
    }
    pub fn full(&self, d: usize) -> Space {
        Space {
            degree: d,
            divisor_degree: 0,
            span: Span::full(self.dimension(d)),
        }
    }
    pub fn smoothness_rank(&self) -> usize {
        let raw = raw_monomials(7);
        let indices: BTreeMap<_, _> = raw.iter().enumerate().map(|(i, m)| (*m, i)).collect();
        let mut rows = vec![];
        for derivative in 0..3 {
            for multiplier in raw_monomials(4) {
                let mut row = vec![F::ZERO; raw.len()];
                for (m, c) in &self.quartic {
                    if m[derivative] > 0 {
                        let mut mon = std::array::from_fn(|i| m[i] + multiplier[i]);
                        mon[derivative] -= 1;
                        row[indices[&mon]] = row[indices[&mon]] + scale(*c, m[derivative] as u8);
                    }
                }
                rows.push(row);
            }
        }
        Span::from_rows(raw.len(), rows).rows.len()
    }
}

#[derive(Clone, Debug)]
pub struct Space {
    pub degree: usize,
    pub divisor_degree: usize,
    pub span: Span,
}
impl Space {
    pub fn from_rows(
        ring: &Ring,
        degree: usize,
        divisor_degree: usize,
        rows: impl IntoIterator<Item = Row>,
    ) -> Self {
        Self {
            degree,
            divisor_degree,
            span: Span::from_rows(ring.dimension(degree), rows),
        }
    }
    pub fn assert_dimension(&self, ring: &Ring) {
        assert_eq!(
            self.span.rows.len(),
            ring.dimension(self.degree) - self.divisor_degree,
            "divisor-space dimension at degree {}",
            self.degree
        );
    }
    pub fn product(&self, other: &Self, ring: &Ring) -> Self {
        assert!(4 * self.degree - self.divisor_degree >= 7);
        assert!(4 * other.degree - other.divisor_degree >= 7);
        let degree = self.degree + other.degree;
        let rows = self.span.rows.iter().flat_map(|a| {
            other
                .span
                .rows
                .iter()
                .map(|b| ring.multiply(a, self.degree, b, other.degree))
        });
        let result = Self::from_rows(
            ring,
            degree,
            self.divisor_degree + other.divisor_degree,
            rows,
        );
        result.assert_dimension(ring);
        result
    }
    pub fn promote(&self, degree: usize, ring: &Ring) -> Self {
        assert!(degree >= self.degree);
        if degree == self.degree {
            return self.clone();
        }
        // A length <=6 subscheme of this smooth quartic is handled only from
        // a verified generated ideal; check the full Riemann--Roch dimension.
        let rows = self.span.rows.iter().flat_map(|a| {
            (0..ring.dimension(degree - self.degree)).map(|i| {
                ring.multiply(
                    a,
                    self.degree,
                    &ring.unit(degree - self.degree, i),
                    degree - self.degree,
                )
            })
        });
        let result = Self::from_rows(ring, degree, self.divisor_degree, rows);
        result.assert_dimension(ring);
        result
    }
}
