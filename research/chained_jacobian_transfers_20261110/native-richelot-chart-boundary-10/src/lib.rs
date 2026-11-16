//! Fixed-F101 model-chart correctness component. No Jacobian evaluator or solver.
#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::fmt;

pub const PRIME: u64 = 101;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Fp(u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    Noncanonical,
    DegreeDrop,
    Singular,
    SearchExhausted,
    InvalidParameters,
    NotOnSource,
    NotOnTarget,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl Fp {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);

    pub fn from_canonical(value: u64) -> Result<Self, Error> {
        if value < PRIME {
            Ok(Self(value))
        } else {
            Err(Error::Noncanonical)
        }
    }

    pub fn value(self) -> u64 {
        self.0
    }

    fn plus(self, other: Self) -> Self {
        Self((self.0 + other.0) % PRIME)
    }

    fn minus(self, other: Self) -> Self {
        Self((self.0 + PRIME - other.0) % PRIME)
    }

    fn times(self, other: Self) -> Self {
        Self(self.0 * other.0 % PRIME)
    }

    fn opposite(self) -> Self {
        Self::ZERO.minus(self)
    }

    fn power(self, mut exponent: u64) -> Self {
        let mut base = self;
        let mut result = Self::ONE;
        while exponent != 0 {
            if exponent & 1 == 1 {
                result = result.times(base);
            }
            base = base.times(base);
            exponent >>= 1;
        }
        result
    }

    fn inverse(self) -> Option<Self> {
        (self != Self::ZERO).then(|| self.power(PRIME - 2))
    }
}

fn trim(mut polynomial: Vec<Fp>) -> Vec<Fp> {
    while polynomial.last() == Some(&Fp::ZERO) {
        polynomial.pop();
    }
    polynomial
}

fn remainder(mut left: Vec<Fp>, right: &[Fp]) -> Vec<Fp> {
    let inverse = right
        .last()
        .and_then(|x| x.inverse())
        .expect("nonzero divisor");
    while left.len() >= right.len() {
        let shift = left.len() - right.len();
        let factor = left.last().expect("nonzero dividend").times(inverse);
        for (index, coefficient) in right.iter().enumerate() {
            left[shift + index] = left[shift + index].minus(factor.times(*coefficient));
        }
        left = trim(left);
    }
    left
}

fn squarefree(coefficients: &[Fp; 7]) -> bool {
    let mut left = coefficients.to_vec();
    let mut right = trim(
        coefficients
            .iter()
            .enumerate()
            .skip(1)
            .map(|(degree, value)| value.times(Fp(degree as u64)))
            .collect(),
    );
    while !right.is_empty() {
        let next = remainder(left, &right);
        left = right;
        right = next;
    }
    left.len() == 1
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Curve {
    coefficients: [Fp; 7],
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Point {
    Affine {
        x: Fp,
        y: Fp,
    },
    /// The nonzero leading ratio y/x^3 at a rational infinity.
    Infinity {
        leading_y: Fp,
    },
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Affine { x, y } => write!(f, "A,{},{}", x.value(), y.value()),
            Self::Infinity { leading_y } => write!(f, "I,{},-", leading_y.value()),
        }
    }
}

impl Point {
    pub fn opposite_y(self) -> Self {
        match self {
            Self::Affine { x, y } => Self::Affine { x, y: y.opposite() },
            Self::Infinity { leading_y } => Self::Infinity {
                leading_y: leading_y.opposite(),
            },
        }
    }
}

impl Curve {
    pub fn new(coefficients: [u64; 7]) -> Result<Self, Error> {
        let mut canonical = [Fp::ZERO; 7];
        for (output, input) in canonical.iter_mut().zip(coefficients) {
            *output = Fp::from_canonical(input)?;
        }
        if canonical[6] == Fp::ZERO {
            return Err(Error::DegreeDrop);
        }
        if !squarefree(&canonical) {
            return Err(Error::Singular);
        }
        Ok(Self {
            coefficients: canonical,
        })
    }

    pub fn coefficients(&self) -> [u64; 7] {
        self.coefficients.map(Fp::value)
    }

    fn evaluate(&self, x: Fp) -> Fp {
        self.coefficients
            .iter()
            .rev()
            .fold(Fp::ZERO, |value, coefficient| {
                value.times(x).plus(*coefficient)
            })
    }

    /// Independent monomial evaluation of the homogeneous binary sextic.
    pub fn homogeneous(&self, x: Fp, z: Fp) -> Fp {
        self.coefficients
            .iter()
            .enumerate()
            .fold(Fp::ZERO, |sum, (degree, coefficient)| {
                sum.plus(
                    coefficient
                        .times(x.power(degree as u64))
                        .times(z.power((6 - degree) as u64)),
                )
            })
    }

    pub fn contains(&self, point: Point) -> bool {
        match point {
            Point::Affine { x, y } => y.times(y) == self.evaluate(x),
            Point::Infinity { leading_y } => leading_y.times(leading_y) == self.coefficients[6],
        }
    }

    pub fn rational_points(&self) -> BTreeSet<Point> {
        let mut points = BTreeSet::new();
        for x in 0..PRIME {
            for y in 0..PRIME {
                let point = Point::Affine { x: Fp(x), y: Fp(y) };
                if self.contains(point) {
                    points.insert(point);
                }
            }
        }
        for leading_y in 1..PRIME {
            let point = Point::Infinity {
                leading_y: Fp(leading_y),
            };
            if self.contains(point) {
                points.insert(point);
            }
        }
        points
    }

    /// Character-sum oracle, independent of enumerating y and of Horner evaluation.
    pub fn character_count(&self) -> usize {
        let mut count = 0;
        for x in 0..PRIME {
            let value = self.homogeneous(Fp(x), Fp::ONE);
            count += if value == Fp::ZERO {
                1
            } else if value.power((PRIME - 1) / 2) == Fp::ONE {
                2
            } else {
                0
            };
        }
        if self.coefficients[6].power((PRIME - 1) / 2) == Fp::ONE {
            count += 2;
        }
        count
    }
}

fn binomial(n: usize, k: usize) -> u64 {
    let mut result = 1;
    for index in 0..k {
        result = result * (n - index) as u64 / (index + 1) as u64;
    }
    result
}

#[derive(Clone, Debug)]
pub struct Chart {
    source: Curve,
    target: Curve,
    a: Fp,
    b: Fp,
}

impl Chart {
    pub fn new(source: Curve) -> Result<Self, Error> {
        Self::with_candidates(source, &(0..PRIME).collect::<Vec<_>>())
    }

    /// Candidate restrictions are explicit; exhaustion does not assert a curve obstruction.
    pub fn with_candidates(source: Curve, candidates: &[u64]) -> Result<Self, Error> {
        let mut admitted = BTreeSet::new();
        for candidate in candidates {
            admitted.insert(Fp::from_canonical(*candidate)?);
        }
        for a in admitted {
            let value = source.evaluate(a);
            if value == Fp::ZERO {
                continue;
            }
            if let Some(b) = (1..PRIME).map(Fp).find(|b| b.times(*b) == value) {
                return Self::with_parameters(source, a.value(), b.value());
            }
        }
        Err(Error::SearchExhausted)
    }

    /// Both signs of a valid b are accepted. Only the default selection fixes its sign.
    pub fn with_parameters(source: Curve, a: u64, b: u64) -> Result<Self, Error> {
        let a = Fp::from_canonical(a)?;
        let b = Fp::from_canonical(b)?;
        if b == Fp::ZERO || b.times(b) != source.evaluate(a) {
            return Err(Error::InvalidParameters);
        }
        let inverse_square = b.inverse().expect("nonzero b").power(2);
        let mut coefficients = [Fp::ZERO; 7];
        for (degree, coefficient) in source.coefficients.iter().enumerate() {
            for exponent in 0..=degree {
                let contribution = coefficient
                    .times(Fp(binomial(degree, exponent)))
                    .times(a.power(exponent as u64));
                let target_degree = 6 - degree + exponent;
                coefficients[target_degree] = coefficients[target_degree].plus(contribution);
            }
        }
        let target = Curve::new(coefficients.map(|x| x.times(inverse_square).value()))?;
        Ok(Self {
            source,
            target,
            a,
            b,
        })
    }

    pub fn target(&self) -> &Curve {
        &self.target
    }

    pub fn parameters(&self) -> (u64, u64) {
        (self.a.value(), self.b.value())
    }

    pub fn forward(&self, point: Point) -> Result<Point, Error> {
        if !self.source.contains(point) {
            return Err(Error::NotOnSource);
        }
        let inverse_b = self.b.inverse().expect("nonzero b");
        let result = match point {
            Point::Affine { x, y } if x == self.a => Point::Infinity {
                leading_y: y.times(inverse_b),
            },
            Point::Affine { x, y } => {
                let u = x.minus(self.a).inverse().expect("nonzero x-a");
                Point::Affine {
                    x: u,
                    y: y.times(u.power(3)).times(inverse_b),
                }
            }
            Point::Infinity { leading_y } => Point::Affine {
                x: Fp::ZERO,
                y: leading_y.times(inverse_b),
            },
        };
        if !self.target.contains(result) {
            return Err(Error::NotOnTarget);
        }
        Ok(result)
    }

    pub fn inverse(&self, point: Point) -> Result<Point, Error> {
        if !self.target.contains(point) {
            return Err(Error::NotOnTarget);
        }
        let result = match point {
            Point::Affine { x, y } if x == Fp::ZERO => Point::Infinity {
                leading_y: self.b.times(y),
            },
            Point::Affine { x, y } => {
                let reciprocal = x.inverse().expect("nonzero u");
                Point::Affine {
                    x: self.a.plus(reciprocal),
                    y: self.b.times(y).times(reciprocal.power(3)),
                }
            }
            Point::Infinity { leading_y } => Point::Affine {
                x: self.a,
                y: self.b.times(leading_y),
            },
        };
        if !self.source.contains(result) {
            return Err(Error::NotOnSource);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixtures() -> Vec<(String, [u64; 7])> {
        let mut lines = include_str!("../fixtures-10.csv").lines();
        assert_eq!(lines.next(), Some("name,h0,h1,h2,h3,h4,h5,h6"));
        let rows: Vec<_> = lines
            .map(|line| {
                let columns: Vec<_> = line.split(',').collect();
                assert_eq!(columns.len(), 8);
                let values = std::array::from_fn(|index| columns[index + 1].parse().unwrap());
                (columns[0].to_owned(), values)
            })
            .collect();
        assert_eq!(rows.len(), 12);
        rows
    }

    #[test]
    fn canonical_inverse_and_domain_controls() {
        assert_eq!(Fp::from_canonical(PRIME), Err(Error::Noncanonical));
        assert_eq!(Fp::ZERO.inverse(), None);
        for value in 1..PRIME {
            let x = Fp(value);
            let inverse = x.inverse().unwrap();
            println!("trace_base_inverse,{value},{}", inverse.value());
            assert_eq!(x.times(inverse), Fp::ONE);
        }
        assert_eq!(Curve::new([0; 7]), Err(Error::DegreeDrop));
        assert_eq!(Curve::new([0, 0, 0, 0, 0, 0, 1]), Err(Error::Singular));
        assert_eq!(
            Curve::new([101, 0, 0, 0, 0, 0, 1]),
            Err(Error::Noncanonical)
        );
        let source = Curve::new(fixtures()[8].1).unwrap();
        assert_eq!(
            Chart::with_candidates(source.clone(), &[]).unwrap_err(),
            Error::SearchExhausted
        );
        assert_eq!(
            Chart::with_candidates(source.clone(), &[101]).unwrap_err(),
            Error::Noncanonical
        );
        assert_eq!(
            Chart::with_parameters(source.clone(), 101, 7).unwrap_err(),
            Error::Noncanonical
        );
        assert_eq!(
            Chart::with_parameters(source.clone(), 0, 101).unwrap_err(),
            Error::Noncanonical
        );
        assert_eq!(
            Chart::with_parameters(source.clone(), 0, 0).unwrap_err(),
            Error::InvalidParameters
        );
        assert_eq!(
            Chart::with_parameters(source, 0, 8).unwrap_err(),
            Error::InvalidParameters
        );
        println!("field_inverse_nonzero_inputs=100; malformed/singular/degree/parameter/search controls passed");
    }

    #[test]
    fn every_frozen_curve_full_rational_chart_and_projective_identity() {
        for (name, coefficients) in fixtures() {
            let source = Curve::new(coefficients).unwrap();
            let chart = Chart::new(source.clone()).unwrap();
            assert_eq!(chart.target.coefficients[6], Fp::ONE);
            for (u, v) in (0..PRIME)
                .map(|u| (Fp(u), Fp::ONE))
                .chain(std::iter::once((Fp::ONE, Fp::ZERO)))
            {
                let expected = source
                    .homogeneous(chart.a.times(u).plus(v), u)
                    .times(chart.b.inverse().unwrap().power(2));
                let actual = chart.target.homogeneous(u, v);
                println!(
                    "trace_projective,{name},{},{},{},{}",
                    u.value(),
                    v.value(),
                    actual.value(),
                    expected.value()
                );
                assert_eq!(actual, expected, "{name}");
            }
            let input = source.rational_points();
            let target = chart.target.rational_points();
            assert_eq!(input.len(), source.character_count(), "{name}");
            assert_eq!(target.len(), chart.target.character_count(), "{name}");
            if name.starts_with("target_") {
                assert_eq!(input.len(), 120, "prior CT49 target count: {name}");
            }
            let mut images = BTreeSet::new();
            for point in &input {
                let image = chart.forward(*point).unwrap();
                let recovered = chart.inverse(image).unwrap();
                println!("trace_forward,{name},{point},{image},{recovered}");
                assert!(images.insert(image), "injectivity: {name}");
                assert_eq!(recovered, *point, "{name}");
            }
            assert_eq!(images, target, "surjectivity: {name}");
            for point in &target {
                let preimage = chart.inverse(*point).unwrap();
                let recovered = chart.forward(preimage).unwrap();
                println!("trace_inverse,{name},{point},{preimage},{recovered}");
                assert_eq!(recovered, *point, "{name}");
            }
            for leading_y in [Fp::ONE, Fp::ONE.opposite()] {
                let infinity = Point::Infinity { leading_y };
                let preimage = Point::Affine {
                    x: chart.a,
                    y: chart.b.times(leading_y),
                };
                assert_eq!(chart.forward(preimage), Ok(infinity));
                assert_eq!(chart.inverse(infinity), Ok(preimage));
            }
            let affine_only: BTreeSet<_> = images
                .iter()
                .copied()
                .filter(|point| matches!(point, Point::Affine { .. }))
                .collect();
            assert_eq!(images.len() - affine_only.len(), 2);
            assert_ne!(affine_only, target, "lost infinities must fail: {name}");
            if source.coefficients[6].power((PRIME - 1) / 2) != Fp::ONE {
                assert!(!input.iter().any(|p| matches!(p, Point::Infinity { .. })));
                assert!(!target
                    .iter()
                    .any(|p| matches!(p, Point::Affine { x, .. } if *x == Fp::ZERO)));
            }
            println!(
                "fixture={name}; a={}; b={}; source_points={}; target_points={}; target_coefficients={:?}; projective_arguments=102; full_roundtrips=ok; lost_infinity_control=rejected",
                chart.a.value(), chart.b.value(), input.len(), target.len(), chart.target.coefficients()
            );
        }
    }

    #[test]
    fn normalization_reversal_coefficient_and_invalid_point_controls() {
        let source = Curve::new(fixtures()[8].1).unwrap();
        let chart = Chart::new(source).unwrap();
        assert_eq!(chart.parameters(), (0, 7));
        let selected = Point::Affine {
            x: chart.a,
            y: chart.b,
        };
        let mut reversed = chart.clone();
        reversed.target.coefficients.reverse();
        assert_eq!(reversed.forward(selected), Err(Error::NotOnTarget));
        let mut unnormalized = chart.clone();
        for coefficient in &mut unnormalized.target.coefficients {
            *coefficient = coefficient.times(chart.b.power(2));
        }
        assert_eq!(unnormalized.forward(selected), Err(Error::NotOnTarget));
        let mut corrupted = chart.clone();
        corrupted.target.coefficients[0] = corrupted.target.coefficients[0].plus(Fp::ONE);
        assert!(chart
            .source
            .rational_points()
            .iter()
            .any(|p| corrupted.forward(*p).is_err()));
        assert_eq!(
            chart.forward(Point::Affine {
                x: chart.a,
                y: Fp::ZERO
            }),
            Err(Error::NotOnSource)
        );
        assert_eq!(
            chart.inverse(Point::Infinity {
                leading_y: Fp::ZERO
            }),
            Err(Error::NotOnTarget)
        );
        println!(
            "wrong normalization/reversal/coefficient and invalid source/target points rejected"
        );
    }

    #[test]
    fn opposite_sign_is_a_valid_curve_isomorphism_not_a_dual_orientation_certificate() {
        for (name, coefficients) in fixtures() {
            let source = Curve::new(coefficients).unwrap();
            let chart = Chart::new(source.clone()).unwrap();
            let other =
                Chart::with_parameters(source.clone(), chart.a.value(), chart.b.opposite().value())
                    .unwrap();
            assert_eq!(chart.target, other.target);
            for point in source.rational_points() {
                let image = other.forward(point).unwrap();
                println!(
                    "trace_sign,{name},{point},{},{}",
                    chart.forward(point).unwrap(),
                    image
                );
                assert_eq!(image, chart.forward(point).unwrap().opposite_y(), "{name}");
                assert_eq!(other.inverse(image), Ok(point), "{name}");
            }
            assert_ne!(
                chart.forward(Point::Affine {
                    x: chart.a,
                    y: chart.b
                }),
                other.forward(Point::Affine {
                    x: chart.a,
                    y: chart.b
                })
            );
            println!("fixture={name}; both_b_signs_valid=true; dual_orientation=unestablished");
        }
    }
}
