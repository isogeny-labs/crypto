//! Bounded public-fixture adapter: auxiliary fibers to native RR5 preparation.
//! No relation solver, scalar search or performance harness is invoked here.
use auxiliary_two_isogeny::{Chain, Curve, Fiber, Point};
use prime31_algebraic_ic::Curve as NativeCurve;
use prime31_four_summand::extractor::{prepare_cofactor, Limits, PreparedCofactor, ResultRow};
use prime31_four_summand::relation_adapter::SignedRelation;
use std::collections::BTreeMap;

type Result<T> = std::result::Result<T, String>;
/// The same exponent/coefficient representation used by native RR5 equations.
pub type Equation = Vec<(Vec<u32>, u64)>;

fn add(a: u64, b: u64, p: u64) -> u64 {
    (a + b) % p
}
fn sub(a: u64, b: u64, p: u64) -> u64 {
    (a + p - b) % p
}
fn mul(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn pow(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut value = 1;
    while n > 0 {
        if n & 1 != 0 {
            value = mul(value, a, p);
        }
        a = mul(a, a, p);
        n >>= 1;
    }
    value
}
#[cfg(test)]
fn evaluate(coefficients: &[u64], x: u64, p: u64) -> u64 {
    coefficients
        .iter()
        .rev()
        .fold(0, |value, &c| add(mul(value, x, p), c, p))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Presentation {
    Chained,
    Dense,
}

#[derive(Clone, Debug)]
pub struct LeafSystem {
    prime: u64,
    names: Vec<String>,
    equations: Vec<Equation>,
}

impl LeafSystem {
    pub fn variable_names(&self) -> &[String] {
        &self.names
    }
    pub fn equations(&self) -> &[Equation] {
        &self.equations
    }
    pub fn satisfied(&self, assignment: &[u64]) -> bool {
        assignment.len() == self.names.len()
            && assignment.iter().all(|&x| x < self.prime)
            && self.equations.iter().all(|equation| {
                equation.iter().fold(0, |sum, (powers, coefficient)| {
                    let term = powers.iter().zip(assignment).fold(
                        *coefficient,
                        |value, (&exponent, &x)| {
                            mul(value, pow(x, exponent as u64, self.prime), self.prime)
                        },
                    );
                    add(sum, term, self.prime)
                }) == 0
            })
    }
}

/// Preserve public curve metadata; preparation validates geometry only.
/// The later recovery consumer must separately validate n, g and subgroup use.
#[derive(Clone, Debug)]
pub struct AuxiliaryBase {
    public_curve: NativeCurve,
    target: Curve,
    chain: Chain,
    fiber: Fiber,
    points: Vec<(u64, u64)>,
    labels: BTreeMap<(u64, u64), i32>,
    base_polynomial: Vec<u64>,
}

impl AuxiliaryBase {
    pub fn new(public_curve: NativeCurve, chain: &Chain, anchor: Point) -> Result<Self> {
        let p = public_curve.p;
        if p != chain.source().prime() || public_curve.a >= p || public_curve.b >= p {
            return Err("mismatched field or noncanonical target coefficient".into());
        }
        let target = Curve::new(p, 0, public_curve.a as i64, public_curve.b as i64)?;
        let fiber = chain.fiber(anchor)?;
        let mut points = fiber
            .lift_to(target)?
            .into_iter()
            .map(|point| {
                point
                    .coordinates()
                    .ok_or("affine lift required".to_string())
            })
            .collect::<Result<Vec<_>>>()?;
        points.sort_unstable();
        let mut labels = BTreeMap::new();
        let mut base_polynomial = vec![1];
        for (i, &(x, y)) in points.iter().enumerate() {
            if y == 0 || y > p - y || (i > 0 && points[i - 1].0 == x) {
                return Err("invalid canonical lifted base".into());
            }
            let label = i as i32 + 1;
            labels.insert((x, y), label);
            labels.insert((x, p - y), -label);
            let mut next = vec![0; base_polynomial.len() + 1];
            for (j, &coefficient) in base_polynomial.iter().enumerate() {
                next[j] = sub(next[j], mul(x, coefficient, p), p);
                next[j + 1] = add(next[j + 1], coefficient, p);
            }
            base_polynomial = next;
        }
        Ok(Self {
            public_curve,
            target,
            chain: chain.clone(),
            fiber,
            points,
            labels,
            base_polynomial,
        })
    }

    pub fn public_curve(&self) -> NativeCurve {
        self.public_curve
    }
    pub fn points(&self) -> &[(u64, u64)] {
        &self.points
    }
    pub fn full_fiber_polynomial(&self) -> &[u64] {
        self.fiber.polynomial().coefficients()
    }
    pub fn base_polynomial(&self) -> &[u64] {
        &self.base_polynomial
    }
    pub fn label(&self, point: (u64, u64)) -> Option<i32> {
        self.labels.get(&point).copied()
    }
    pub fn point(&self, label: i32) -> Option<(u64, u64)> {
        let index = label.unsigned_abs().checked_sub(1)? as usize;
        let &(x, y) = self.points.get(index)?;
        Some((
            x,
            if label > 0 {
                y
            } else {
                self.public_curve.p - y
            },
        ))
    }
    pub fn signed_relation(&self, labels: &[i32]) -> Result<SignedRelation> {
        if labels.is_empty() || labels.len() > 4 {
            return Err("one through four labels required".into());
        }
        let mut terms = Vec::with_capacity(labels.len());
        for &label in labels {
            self.point(label).ok_or("invalid signed label")?;
            terms.push((
                label.unsigned_abs() as usize - 1,
                if label > 0 { 1 } else { -1 },
            ));
        }
        Ok(SignedRelation { terms })
    }

    /// Call the unchanged native equation builder, preserving its status channel.
    pub fn prepare_cofactor(
        &self,
        target: Option<(u64, u64)>,
        limits: Limits,
    ) -> std::result::Result<PreparedCofactor, Box<ResultRow>> {
        prepare_cofactor(self.public_curve, &self.points, target, limits)
    }

    pub fn leaf_system(&self, presentation: Presentation) -> LeafSystem {
        let p = self.public_curve.p;
        let mut names = vec!["x".into(), "y".into(), "s".into()];
        if presentation == Presentation::Chained {
            names.extend((1..self.chain.steps().len()).map(|i| format!("x{i}")));
        }
        let variables = names.len();
        let mut equations = vec![
            equation(
                p,
                variables,
                &[
                    (1, vec![(1, 2)]),
                    (p - 1, vec![(0, 3)]),
                    (sub(0, self.public_curve.a, p), vec![(0, 1)]),
                    (sub(0, self.public_curve.b, p), vec![]),
                ],
            ),
            equation(p, variables, &[(1, vec![(1, 1), (2, 1)]), (p - 1, vec![])]),
        ];
        if presentation == Presentation::Dense {
            equations.push(equation(
                p,
                variables,
                &self
                    .base_polynomial
                    .iter()
                    .enumerate()
                    .map(|(degree, &coefficient)| (coefficient, vec![(0, degree as u32)]))
                    .collect::<Vec<_>>(),
            ));
        } else {
            let chi = self.fiber.image().coordinates().expect("regular fiber").0;
            for (i, step) in self.chain.steps().iter().enumerate() {
                let r = step.kernel_x();
                let [a2, a4, _] = step.source().coefficients();
                let a = (3 * r + a2) % p;
                let b = (3 * r * r + 2 * a2 * r + a4) % p;
                let x = if i == 0 { 0 } else { i + 2 };
                let mut terms = vec![
                    (p - 1, vec![(x, 2)]),
                    (sub(2 * r % p, a, p), vec![(x, 1)]),
                    (sub(sub(mul(a, r, p), mul(r, r, p), p), b, p), vec![]),
                ];
                if i + 1 == self.chain.steps().len() {
                    terms.push((chi, vec![(x, 1)]));
                    terms.push((sub(0, mul(r, chi, p), p), vec![]));
                } else {
                    terms.push((1, vec![(x, 1), (i + 3, 1)]));
                    terms.push((sub(0, r, p), vec![(i + 3, 1)]));
                }
                equations.push(equation(p, variables, &terms));
            }
        }
        LeafSystem {
            prime: p,
            names,
            equations,
        }
    }

    /// Compute a rational-map witness, without consulting the label dictionary.
    pub fn witness(&self, presentation: Presentation, x: u64, y: u64) -> Option<Vec<u64>> {
        let p = self.public_curve.p;
        if x >= p || y == 0 || y >= p || self.target.point(x, y).is_err() {
            return None;
        }
        let mut assignment = vec![x, y, pow(y, p - 2, p)];
        if presentation == Presentation::Chained {
            let mut image = x;
            for (i, step) in self.chain.steps().iter().enumerate() {
                image = step.map_x(image)?;
                if i + 1 < self.chain.steps().len() {
                    assignment.push(image);
                }
            }
        }
        self.leaf_system(presentation)
            .satisfied(&assignment)
            .then_some(assignment)
    }
}

fn equation(p: u64, variables: usize, terms: &[(u64, Vec<(usize, u32)>)]) -> Equation {
    let mut output = BTreeMap::new();
    for (coefficient, monomial) in terms {
        let mut powers = vec![0; variables];
        for &(variable, exponent) in monomial {
            powers[variable] += exponent;
        }
        let value = add(output.get(&powers).copied().unwrap_or(0), *coefficient, p);
        if value == 0 {
            output.remove(&powers);
        } else {
            output.insert(powers, value);
        }
    }
    output.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use prime31_four_summand::extractor::Status;

    fn fixture(b: u64) -> (NativeCurve, Chain, Point) {
        let auxiliary = Curve::auxiliary(31).unwrap();
        let g = auxiliary.point(6, 6).unwrap();
        // n/g are unset for the filtering-only fixture; preparation uses geometry.
        let (n, metadata_g) = if b == 0 { (32, (6, 6)) } else { (0, (0, 0)) };
        let curve = NativeCurve {
            p: 31,
            a: 1,
            b,
            n,
            g: metadata_g,
        };
        (curve, Chain::new(31, g, 3).unwrap(), g)
    }

    #[test]
    fn stable_signed_labels_preserve_the_public_base_order() {
        let (curve, chain, anchor) = fixture(0);
        let base = AuxiliaryBase::new(curve, &chain, anchor).unwrap();
        assert_eq!(base.points().len(), 8);
        let opposite = chain.source().point(6, 25).unwrap();
        let translated = chain
            .source()
            .add(anchor, chain.source().multiply(4, anchor).unwrap())
            .unwrap();
        for other in [opposite, translated] {
            assert_eq!(
                base.points(),
                AuxiliaryBase::new(curve, &chain, other).unwrap().points()
            );
        }
        for (i, &(x, y)) in base.points().iter().enumerate() {
            assert!(y < 31 - y);
            assert_eq!(base.label((x, y)), Some(i as i32 + 1));
            assert_eq!(base.label((x, 31 - y)), Some(-(i as i32 + 1)));
            assert_eq!(base.point(i as i32 + 1), Some((x, y)));
            assert_eq!(base.point(-(i as i32 + 1)), Some((x, 31 - y)));
        }
        assert_eq!(
            base.signed_relation(&[1, -2, 1, 3]).unwrap().terms,
            vec![(0, 1), (1, -1), (0, 1), (2, 1)]
        );
    }

    #[test]
    fn dense_and_chained_leaf_systems_match_all_f31_affine_pairs() {
        for b in [0, 5] {
            let (curve, chain, anchor) = fixture(b);
            let base = AuxiliaryBase::new(curve, &chain, anchor).unwrap();
            for x in 0..31 {
                for y in 0..31 {
                    let expected = base.label((x, y)).is_some();
                    for mode in [Presentation::Dense, Presentation::Chained] {
                        assert_eq!(
                            base.witness(mode, x, y).is_some(),
                            expected,
                            "b={b} x={x} y={y} mode={mode:?}"
                        );
                    }
                }
                assert_eq!(
                    evaluate(base.base_polynomial(), x, 31) == 0,
                    base.points().iter().any(|point| point.0 == x)
                );
            }
            let low = base.leaf_system(Presentation::Chained);
            assert_eq!(low.variable_names(), ["x", "y", "s", "x1", "x2"]);
            assert!(low.equations()[2..]
                .iter()
                .flatten()
                .all(|(e, _)| e.iter().sum::<u32>() <= 2));
        }
    }

    #[test]
    fn filtering_keeps_actual_base_polynomial_separate_from_full_fiber() {
        let (curve, chain, anchor) = fixture(5);
        let base = AuxiliaryBase::new(curve, &chain, anchor).unwrap();
        assert!(!base.points().is_empty());
        assert!(base.points().len() < 8);
        assert_eq!(base.full_fiber_polynomial().len(), 9);
        assert_eq!(base.base_polynomial().len(), base.points().len() + 1);
        for &(x, y) in base.points() {
            assert_eq!(evaluate(base.full_fiber_polynomial(), x, 31), 0);
            assert!(Curve::new(31, 0, 1, 5).unwrap().point(x, y).is_ok());
        }
    }

    #[test]
    fn poles_mutations_and_noncanonical_assignments_are_rejected() {
        let (curve, chain, anchor) = fixture(0);
        let base = AuxiliaryBase::new(curve, &chain, anchor).unwrap();
        let &(x, y) = base.points().first().unwrap();
        let original = base.witness(Presentation::Chained, x, y).unwrap();
        let low = base.leaf_system(Presentation::Chained);
        for index in [2, 3, 4] {
            let mut altered = original.clone();
            altered[index] = (altered[index] + 1) % 31;
            assert!(!low.satisfied(&altered));
        }
        let mut outside = original.clone();
        outside[0] += 31;
        assert!(!low.satisfied(&outside));
        assert!(!low.satisfied(&original[..4]));
        assert!(base.witness(Presentation::Chained, 31, y).is_none());
        assert!(base.witness(Presentation::Dense, x, 0).is_none());
        for next in 0..31 {
            let mut pole = original.clone();
            pole[0] = 0;
            pole[3] = next;
            let first_step_only = LeafSystem {
                equations: vec![low.equations[2].clone()],
                ..low.clone()
            };
            assert!(!first_step_only.satisfied(&pole));
            pole = original.clone();
            pole[3] = chain.steps()[1].kernel_x();
            pole[4] = next;
            let second_step_only = LeafSystem {
                equations: vec![low.equations[3].clone()],
                ..low.clone()
            };
            assert!(!second_step_only.satisfied(&pole));
        }
        let mut corrupted = base.leaf_system(Presentation::Dense);
        corrupted.equations[2].push((vec![0; 3], 1));
        for &(x, y) in base.points() {
            assert!(!corrupted.satisfied(&base.witness(Presentation::Dense, x, y).unwrap()));
        }
    }

    #[test]
    fn native_cofactor_preparation_receives_the_exact_sorted_points() {
        let (curve, chain, anchor) = fixture(0);
        let base = AuxiliaryBase::new(curve, &chain, anchor).unwrap();
        let limits = Limits {
            max_degree: 10,
            seconds: 3,
            max_polynomial_terms: 20_000,
            max_assignments: 4096,
        };
        let mut independent_points = Vec::new();
        for k in (1..32).step_by(2) {
            let (x, y) = chain
                .source()
                .multiply(k, anchor)
                .unwrap()
                .coordinates()
                .unwrap();
            independent_points.push((x, y.min(31 - y)));
        }
        independent_points.sort_unstable();
        independent_points.dedup();
        assert_eq!(base.points(), independent_points);
        let actual = base.prepare_cofactor(Some((7, 3)), limits).unwrap();
        let expected = prepare_cofactor(curve, &independent_points, Some((7, 3)), limits).unwrap();
        assert_eq!(actual.prime, expected.prime);
        assert_eq!(actual.variables, expected.variables);
        assert_eq!(actual.equations, expected.equations);
        assert_eq!(actual.quartic, expected.quartic);
        assert_eq!(actual.equation_multipliers, expected.equation_multipliers);
        assert_eq!(actual.diagnostics.base_size, 8);
        for target in [None, Some(base.points()[0])] {
            assert_eq!(
                base.prepare_cofactor(target, limits).unwrap_err().status,
                Status::UnsupportedChart
            );
        }
    }

    #[test]
    fn invalid_labels_fields_coefficients_and_fibers_are_rejected() {
        let (curve, chain, anchor) = fixture(0);
        let base = AuxiliaryBase::new(curve, &chain, anchor).unwrap();
        for label in [0, 9, -9, i32::MIN, i32::MAX] {
            assert!(base.point(label).is_none());
            assert!(base.signed_relation(&[label]).is_err());
        }
        assert!(base.signed_relation(&[]).is_err());
        assert!(base.signed_relation(&[1; 5]).is_err());
        assert!(base.label((31, 6)).is_none());
        for invalid in [
            NativeCurve { p: 127, ..curve },
            NativeCurve { a: 31, ..curve },
            NativeCurve {
                a: 0,
                b: 0,
                ..curve
            },
        ] {
            assert!(AuxiliaryBase::new(invalid, &chain, anchor).is_err());
        }
        assert!(AuxiliaryBase::new(curve, &chain, Point::infinity()).is_err());
    }
}
