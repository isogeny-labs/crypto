//! A candidate-tuple elliptic section predicate, not a root-discovery solver.
//! Classical divisor theory supplies the geometric interpretation; native
//! controls do not constitute its formal proof. Variable-time research code.
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::collections::{BTreeMap, BTreeSet};
use torus_s4::{curve25519_prime, Curve, Field, Point};

pub mod controls;
pub type Result<T> = std::result::Result<T, String>;
pub const ARITY: usize = 6;
const JET: usize = 7;
type Series = [BigUint; JET];

#[derive(Clone, Debug)]
pub struct IndexedPoint {
    pub column: u64,
    pub point: Point,
}

#[derive(Clone, Debug)]
pub struct Evaluation {
    pub rows: Vec<Vec<BigUint>>,
    pub multiplicities: Vec<usize>,
    pub charts: Vec<&'static str>,
}

#[derive(Clone, Debug)]
pub struct Analysis {
    pub evaluation: Evaluation,
    pub rank: usize,
    pub section: Option<Vec<BigUint>>,
}

pub struct Operator {
    pub curve: Curve,
}

fn zero() -> Series {
    std::array::from_fn(|_| BigUint::zero())
}
fn constant(c: BigUint) -> Series {
    let mut a = zero();
    a[0] = c;
    a
}
fn times_t(a: &Series) -> Series {
    std::array::from_fn(|i| {
        if i == 0 {
            BigUint::zero()
        } else {
            a[i - 1].clone()
        }
    })
}
fn product(f: &Field, a: &Series, b: &Series) -> Series {
    let mut out = zero();
    for (n, c) in out.iter_mut().enumerate() {
        for i in 0..=n {
            *c = f.add(c, &f.mul(&a[i], &b[n - i]));
        }
    }
    out
}

impl Operator {
    /// Native v1's admitted fields are fixed. Primality is not inferred by a
    /// permissive modulus parser; two-torsion and member infinity are refused.
    pub fn new(p: BigUint, a: BigUint) -> Result<Self> {
        if p != BigUint::from(101u32) && p != curve25519_prime() {
            return Err("unsupported field: v1 admits p101 or Curve25519 p".into());
        }
        if a >= p {
            return Err("noncanonical curve coefficient".into());
        }
        let f = Field::new(p)?;
        if f.square(&a) == BigUint::from(4u32) {
            return Err("singular Montgomery cubic".into());
        }
        Ok(Self {
            curve: Curve { f, a },
        })
    }

    fn validate_point(&self, p: &Point, member: bool) -> Result<()> {
        if !self.curve.contains(p) {
            return Err("off-curve or noncanonical point".into());
        }
        match p {
            None if member => Err("unsupported identity member".into()),
            Some((_, y)) if y.is_zero() => Err("unsupported finite two-torsion chart".into()),
            _ => Ok(()),
        }
    }

    /// Taylor/Hasse coefficients with t=x-a. No factorial division occurs.
    fn coordinates(&self, a: &BigUint, b: &BigUint) -> Result<(Series, Series)> {
        let f = &self.curve.f;
        let mut x = constant(a.clone());
        x[1] = BigUint::one();
        let x2 = product(f, &x, &x);
        let x3 = product(f, &x2, &x);
        let rhs: Series =
            std::array::from_fn(|i| f.add(&f.add(&x3[i], &f.mul(&self.curve.a, &x2[i])), &x[i]));
        let mut y = constant(b.clone());
        let inv_2b = f.inv(&f.scale(b, 2))?;
        for n in 1..JET {
            let mut cross = BigUint::zero();
            for i in 1..n {
                cross = f.add(&cross, &f.mul(&y[i], &y[n - i]));
            }
            y[n] = f.mul(&f.sub(&rhs[n], &cross), &inv_2b);
        }
        Ok((x, y))
    }

    fn sections(
        &self,
        p: &(BigUint, BigUint),
        target: &Point,
    ) -> Result<([Series; ARITY], &'static str)> {
        let f = &self.curve.f;
        let (x, y) = self.coordinates(&p.0, &p.1)?;
        let x2 = product(f, &x, &x);
        let xy = product(f, &x, &y);
        let first = [
            constant(BigUint::one()),
            x.clone(),
            y.clone(),
            x2.clone(),
            xy,
        ];
        let Some((r, s)) = target else {
            return Ok((
                [
                    first[0].clone(),
                    x.clone(),
                    y,
                    x2.clone(),
                    first[4].clone(),
                    product(f, &x2, &x),
                ],
                "target-identity-L6O",
            ));
        };
        let mut numerator = y.clone();
        numerator[0] = f.add(&numerator[0], s);
        if p.0 == *r && p.1 == *s {
            return Ok((
                [
                    times_t(&first[0]),
                    times_t(&first[1]),
                    times_t(&first[2]),
                    times_t(&first[3]),
                    times_t(&first[4]),
                    numerator,
                ],
                "member-at-target-local-section",
            ));
        }
        let (h, chart) = if p.0 == *r {
            if !numerator[0].is_zero() {
                return Err("inconsistent same-x chart".into());
            }
            // Only coefficients0..5 are read. Coefficient6 requires y_7 and
            // is deliberately left unused; y_6 suffices for the sixth jet.
            let h = std::array::from_fn(|i| {
                if i + 1 < JET {
                    y[i + 1].clone()
                } else {
                    BigUint::zero()
                }
            });
            (h, "member-at-negative-target-removable")
        } else {
            let inverse = f.inv(&f.sub(&p.0, r))?;
            let mut h = zero();
            for n in 0..JET {
                let previous = if n == 0 {
                    BigUint::zero()
                } else {
                    h[n - 1].clone()
                };
                h[n] = f.mul(&f.sub(&numerator[n], &previous), &inverse);
            }
            (h, "regular")
        };
        Ok((
            [
                first[0].clone(),
                first[1].clone(),
                first[2].clone(),
                first[3].clone(),
                first[4].clone(),
                h,
            ],
            chart,
        ))
    }

    pub fn evaluate(&self, terms: &[IndexedPoint], target: &Point) -> Result<Evaluation> {
        if terms.len() != ARITY {
            return Err("expected exactly six indexed candidates".into());
        }
        self.validate_point(target, false)?;
        let mut ids = BTreeSet::new();
        let mut groups = BTreeMap::new();
        for term in terms {
            if !ids.insert(term.column) {
                return Err("repeated raw column id".into());
            }
            self.validate_point(&term.point, true)?;
            let p = term.point.as_ref().ok_or("unsupported identity member")?;
            *groups.entry(p.clone()).or_insert(0usize) += 1;
        }
        let mut e = Evaluation {
            rows: vec![],
            multiplicities: vec![],
            charts: vec![],
        };
        for (p, multiplicity) in groups {
            let (basis, chart) = self.sections(&p, target)?;
            for order in 0..multiplicity {
                e.rows
                    .push(basis.iter().map(|s| s[order].clone()).collect());
            }
            e.multiplicities.push(multiplicity);
            e.charts.push(chart);
        }
        Ok(e)
    }

    pub fn analyze(&self, terms: &[IndexedPoint], target: &Point) -> Result<Analysis> {
        let evaluation = self.evaluate(terms, target)?;
        let (rank, section) = self.null_section(&evaluation.rows)?;
        Ok(Analysis {
            evaluation,
            rank,
            section,
        })
    }

    pub fn verify_section(
        &self,
        terms: &[IndexedPoint],
        target: &Point,
        c: &[BigUint],
    ) -> Result<()> {
        let f = &self.curve.f;
        if c.len() != ARITY || c.iter().any(|v| v >= &f.p) {
            return Err("malformed/noncanonical section vector".into());
        }
        if c.iter().find(|v| !v.is_zero()) != Some(&BigUint::one()) {
            return Err("section must be nonzero and normalized".into());
        }
        for row in self.evaluate(terms, target)?.rows {
            let mut dot = BigUint::zero();
            for (a, b) in row.iter().zip(c) {
                dot = f.add(&dot, &f.mul(a, b));
            }
            if !dot.is_zero() {
                return Err("section violates an original jet condition".into());
            }
        }
        Ok(())
    }

    fn null_section(&self, rows: &[Vec<BigUint>]) -> Result<(usize, Option<Vec<BigUint>>)> {
        let f = &self.curve.f;
        let mut a = rows.to_vec();
        let mut pivots = vec![];
        for column in 0..ARITY {
            let rank = pivots.len();
            let Some(pivot) = (rank..ARITY).find(|&r| !a[r][column].is_zero()) else {
                continue;
            };
            a.swap(rank, pivot);
            let inverse = f.inv(&a[rank][column])?;
            for v in &mut a[rank] {
                *v = f.mul(v, &inverse);
            }
            for r in 0..ARITY {
                if r == rank {
                    continue;
                }
                let multiple = a[r][column].clone();
                for c in 0..ARITY {
                    a[r][c] = f.sub(&a[r][c], &f.mul(&multiple, &a[rank][c]));
                }
            }
            pivots.push(column);
        }
        if pivots.len() == ARITY {
            return Ok((ARITY, None));
        }
        let free = (0..ARITY)
            .find(|c| !pivots.contains(c))
            .ok_or("missing free column")?;
        let mut v = vec![BigUint::zero(); ARITY];
        v[free] = BigUint::one();
        for (r, &c) in pivots.iter().enumerate() {
            v[c] = f.sub(&BigUint::zero(), &a[r][free]);
        }
        let lead = v.iter().find(|c| !c.is_zero()).ok_or("zero section")?;
        let inverse = f.inv(lead)?;
        for c in &mut v {
            *c = f.mul(c, &inverse);
        }
        Ok((pivots.len(), Some(v)))
    }

    /// Deliberately incorrect references retained solely to demonstrate the
    /// registered alias/pole counterexamples. Not part of the correct API.
    pub fn naive_rank(
        &self,
        terms: &[IndexedPoint],
        target: &Point,
        cleared: bool,
    ) -> Result<usize> {
        let f = &self.curve.f;
        let (r, s) = target
            .as_ref()
            .ok_or("naive reference needs finite target")?;
        let mut rows = vec![];
        for term in terms {
            let (x, y) = term.point.as_ref().ok_or("naive identity")?;
            let x2 = f.square(x);
            let mut row = vec![BigUint::one(), x.clone(), y.clone(), x2, f.mul(x, y)];
            let numerator = f.add(y, s);
            let denominator = f.sub(x, r);
            if cleared {
                for c in &mut row {
                    *c = f.mul(c, &denominator);
                }
                row.push(numerator);
            } else {
                row.push(f.mul(&numerator, &f.inv(&denominator)?));
            }
            rows.push(row);
        }
        self.null_section(&rows).map(|(rank, _)| rank)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formal_coordinates_obey_curve_through_sixth_order() -> Result<()> {
        let op = Operator::new(101u32.into(), 95u32.into())?;
        let (x, y) = op.coordinates(&2u32.into(), &17u32.into())?;
        let f = &op.curve.f;
        let square = product(f, &y, &y);
        let x2 = product(f, &x, &x);
        let x3 = product(f, &x2, &x);
        for i in 0..JET {
            assert_eq!(
                square[i],
                f.add(&f.add(&x3[i], &f.mul(&op.curve.a, &x2[i])), &x[i])
            );
        }
        Ok(())
    }

    #[test]
    fn normalized_kernel_is_nontrivial_and_checks_all_rows() -> Result<()> {
        let op = Operator::new(101u32.into(), 95u32.into())?;
        let rows = vec![
            vec![1u32, 2, 3, 4, 5, 6]
                .into_iter()
                .map(BigUint::from)
                .collect();
            ARITY
        ];
        let (rank, kernel) = op.null_section(&rows)?;
        assert_eq!(rank, 1);
        let kernel = kernel.ok_or("missing kernel")?;
        for row in rows {
            assert!(row
                .iter()
                .zip(&kernel)
                .fold(BigUint::zero(), |a, (b, c)| op
                    .curve
                    .f
                    .add(&a, &op.curve.f.mul(b, c)))
                .is_zero());
        }
        Ok(())
    }
}
