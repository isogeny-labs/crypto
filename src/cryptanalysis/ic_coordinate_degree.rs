//! Matched coordinate systems for prime-field index-calculus qualification.
//!
//! This module is deliberately small-field and structural.  It uses the
//! order-251 subgroup of
//!
//! `v^2 = u^3 + 5u^2 + u` over `F_1019`
//!
//! as a public known-answer fixture.  The curve has the Montgomery rational
//! two-torsion geometry used by Curve25519, but it is **not** Curve25519 and
//! carries no production-curve security claim.  Every system describes the
//! same planted relation `[9]G + [27]G = [36]G` and is solved by the shared
//! [`super::f4_fp`] implementation.
//!
//! The quotient coordinate is `w = u + u^{-1}`.  For the two-isogeny with
//! kernel `(0,0)`, the destination Montgomery coordinate is `U = w + A` and
//! the destination curve has coefficients `A' = -2A`, `b' = A^2 - 4b`.
//! Consequently the square-free quotient relation is the ordinary native
//! Montgomery `f_3` on the destination.  The independently computed norm of
//! the source `f_3` over all three quadratic fibres is checked to equal the
//! fourth power of that square-free relation on the fixture.

use std::collections::BTreeMap;

use serde::Serialize;

use super::f4_fp::{self, F4Options, Ordering, Poly};

pub const FIXTURE_ID: &str = "curve25519-shaped-fp1019-a5-r251-v1";
pub const FIXTURE_P: u64 = 1_019;
pub const FIXTURE_A: u64 = 5;
pub const FIXTURE_SUBGROUP_ORDER: u64 = 251;
pub const FIXTURE_COFACTOR: u64 = 4;
pub const CURVE25519_A: u64 = 486_662;
pub const FIXTURE_FIELD_SHA256: &str =
    "2bf0c220697af4be98d156034f640cde42e92619b7d3fb49ae8ef7858b06ff35";
pub const FIXTURE_SOURCE_CURVE_ID: &str = "EC1P10Cquotienttoyhc0736364223b";
pub const FIXTURE_SOURCE_CURVE_UID: &str =
    "urn:ec-record:1:sha256:c0736364223b72593d85a84671b6818fd2ff13ad19a95884d774321eaefbf10e";
pub const FIXTURE_DESTINATION_CURVE_ID: &str = "EC1P10Cquotientisoghd0b8c6a79324";
pub const FIXTURE_DESTINATION_CURVE_UID: &str =
    "urn:ec-record:1:sha256:d0b8c6a7932492aed28e352113b14a87d59099971a69ab093476a975769aef3f";

fn add_mod(left: u64, right: u64, modulus: u64) -> u64 {
    ((u128::from(left) + u128::from(right)) % u128::from(modulus)) as u64
}

fn sub_mod(left: u64, right: u64, modulus: u64) -> u64 {
    ((u128::from(left) + u128::from(modulus) - u128::from(right % modulus)) % u128::from(modulus))
        as u64
}

fn mul_mod(left: u64, right: u64, modulus: u64) -> u64 {
    ((u128::from(left) * u128::from(right)) % u128::from(modulus)) as u64
}

fn pow_mod(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;
    while exponent != 0 {
        if exponent & 1 == 1 {
            result = mul_mod(result, base, modulus);
        }
        base = mul_mod(base, base, modulus);
        exponent >>= 1;
    }
    result
}

fn inv_mod(value: u64, modulus: u64) -> u64 {
    assert_ne!(value % modulus, 0, "attempted to invert zero");
    pow_mod(value, modulus - 2, modulus)
}

fn neg_mod(value: u64, modulus: u64) -> u64 {
    sub_mod(0, value, modulus)
}

fn gcd(mut left: u64, mut right: u64) -> u64 {
    while right != 0 {
        (left, right) = (right, left % right);
    }
    left
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Point {
    Infinity,
    Affine { x: u64, y: u64 },
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct MontgomeryCurve {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub capital_b: u64,
}

impl MontgomeryCurve {
    fn fixture() -> Self {
        Self {
            p: FIXTURE_P,
            a: FIXTURE_A,
            b: 1,
            capital_b: 1,
        }
    }

    fn rhs(self, x: u64) -> u64 {
        let x_squared = mul_mod(x, x, self.p);
        add_mod(
            add_mod(
                mul_mod(x_squared, x, self.p),
                mul_mod(self.a, x_squared, self.p),
                self.p,
            ),
            mul_mod(self.b, x, self.p),
            self.p,
        )
    }

    fn is_on_curve(self, point: Point) -> bool {
        match point {
            Point::Infinity => true,
            Point::Affine { x, y } => {
                mul_mod(self.capital_b, mul_mod(y, y, self.p), self.p) == self.rhs(x)
            }
        }
    }

    fn lift_x(self, x: u64) -> Option<Point> {
        let square = mul_mod(self.rhs(x), inv_mod(self.capital_b, self.p), self.p);
        (0..self.p)
            .find(|&y| mul_mod(y, y, self.p) == square)
            .map(|y| Point::Affine {
                x,
                y: y.min(neg_mod(y, self.p)),
            })
    }

    fn add(self, left: Point, right: Point) -> Point {
        match (left, right) {
            (Point::Infinity, point) | (point, Point::Infinity) => point,
            (Point::Affine { x: x1, y: y1 }, Point::Affine { x: x2, y: y2 }) => {
                if x1 == x2 && add_mod(y1, y2, self.p) == 0 {
                    return Point::Infinity;
                }
                let slope = if x1 == x2 && y1 == y2 {
                    let numerator = add_mod(
                        add_mod(
                            mul_mod(3, mul_mod(x1, x1, self.p), self.p),
                            mul_mod(mul_mod(2, self.a, self.p), x1, self.p),
                            self.p,
                        ),
                        self.b,
                        self.p,
                    );
                    let denominator = mul_mod(mul_mod(2, self.capital_b, self.p), y1, self.p);
                    if denominator == 0 {
                        return Point::Infinity;
                    }
                    mul_mod(numerator, inv_mod(denominator, self.p), self.p)
                } else {
                    mul_mod(
                        sub_mod(y2, y1, self.p),
                        inv_mod(sub_mod(x2, x1, self.p), self.p),
                        self.p,
                    )
                };
                let x3 = sub_mod(
                    sub_mod(
                        sub_mod(
                            mul_mod(self.capital_b, mul_mod(slope, slope, self.p), self.p),
                            self.a,
                            self.p,
                        ),
                        x1,
                        self.p,
                    ),
                    x2,
                    self.p,
                );
                let result = Point::Affine {
                    x: x3,
                    y: sub_mod(mul_mod(slope, sub_mod(x1, x3, self.p), self.p), y1, self.p),
                };
                debug_assert!(self.is_on_curve(result));
                result
            }
        }
    }

    fn scalar_mul(self, mut scalar: u64, point: Point) -> Point {
        let mut result = Point::Infinity;
        let mut addend = point;
        while scalar != 0 {
            if scalar & 1 == 1 {
                result = self.add(result, addend);
            }
            addend = self.add(addend, addend);
            scalar >>= 1;
        }
        result
    }

    fn point_count(self) -> u64 {
        let mut square_counts = vec![0_u64; self.p as usize];
        for y in 0..self.p {
            let square = mul_mod(self.capital_b, mul_mod(y, y, self.p), self.p);
            square_counts[square as usize] += 1;
        }
        1 + (0..self.p)
            .map(|x| square_counts[self.rhs(x) as usize])
            .sum::<u64>()
    }

    fn find_subgroup_generator(self) -> Point {
        for x in 0..self.p {
            let Some(point) = self.lift_x(x) else {
                continue;
            };
            let candidate = self.scalar_mul(FIXTURE_COFACTOR, point);
            if candidate != Point::Infinity
                && self.scalar_mul(FIXTURE_SUBGROUP_ORDER, candidate) == Point::Infinity
            {
                return candidate;
            }
        }
        panic!("fixture subgroup generator missing")
    }

    fn quotient_curve(self) -> Self {
        Self {
            p: self.p,
            a: neg_mod(mul_mod(2, self.a, self.p), self.p),
            b: sub_mod(
                mul_mod(self.a, self.a, self.p),
                mul_mod(4, self.b, self.p),
                self.p,
            ),
            capital_b: self.capital_b,
        }
    }

    fn isogeny(self, point: Point) -> Point {
        match point {
            Point::Infinity | Point::Affine { x: 0, .. } => Point::Infinity,
            Point::Affine { x, y } => {
                let inverse_x = inv_mod(x, self.p);
                let destination = Point::Affine {
                    x: add_mod(
                        add_mod(x, self.a, self.p),
                        mul_mod(self.b, inverse_x, self.p),
                        self.p,
                    ),
                    y: mul_mod(
                        y,
                        sub_mod(
                            1,
                            mul_mod(self.b, mul_mod(inverse_x, inverse_x, self.p), self.p),
                            self.p,
                        ),
                        self.p,
                    ),
                };
                debug_assert!(self.quotient_curve().is_on_curve(destination));
                destination
            }
        }
    }

    fn w(self, point: Point) -> u64 {
        match point {
            Point::Affine { x, .. } if x != 0 => {
                add_mod(x, mul_mod(self.b, inv_mod(x, self.p), self.p), self.p)
            }
            _ => panic!("w is undefined on the isogeny kernel"),
        }
    }

    fn to_twisted_edwards(self, point: Point) -> Point {
        assert_eq!(self.b, 1, "the fixture Montgomery--Edwards map assumes b=1");
        match point {
            Point::Affine { x: u, y: v } if v != 0 && add_mod(u, 1, self.p) != 0 => Point::Affine {
                x: mul_mod(u, inv_mod(v, self.p), self.p),
                y: mul_mod(
                    sub_mod(u, 1, self.p),
                    inv_mod(add_mod(u, 1, self.p), self.p),
                    self.p,
                ),
            },
            _ => panic!("point is exceptional for the Montgomery--Edwards map"),
        }
    }
}

fn affine(point: Point) -> (u64, u64) {
    match point {
        Point::Affine { x, y } => (x, y),
        Point::Infinity => panic!("fixture unexpectedly reached infinity"),
    }
}

#[derive(Clone, Debug)]
struct MatchedFixture {
    curve: MontgomeryCurve,
    generator: Point,
    left: Point,
    right: Point,
    target: Point,
}

impl MatchedFixture {
    fn new() -> Self {
        let curve = MontgomeryCurve::fixture();
        let generator = curve.find_subgroup_generator();
        let left = curve.scalar_mul(9, generator);
        let right = curve.scalar_mul(27, generator);
        let target = curve.add(left, right);
        Self {
            curve,
            generator,
            left,
            right,
            target,
        }
    }

    fn factor_points(&self) -> [Point; 2] {
        [self.left, self.right]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SparsePolynomial {
    variables: usize,
    modulus: u64,
    terms: BTreeMap<Vec<u32>, u64>,
}

impl SparsePolynomial {
    fn zero(variables: usize, modulus: u64) -> Self {
        Self {
            variables,
            modulus,
            terms: BTreeMap::new(),
        }
    }

    fn constant(variables: usize, modulus: u64, value: u64) -> Self {
        let mut polynomial = Self::zero(variables, modulus);
        polynomial.add_term(vec![0; variables], value);
        polynomial
    }

    fn variable(variables: usize, modulus: u64, index: usize) -> Self {
        assert!(index < variables);
        let mut exponents = vec![0; variables];
        exponents[index] = 1;
        let mut polynomial = Self::zero(variables, modulus);
        polynomial.add_term(exponents, 1);
        polynomial
    }

    fn monomial(variables: usize, modulus: u64, exponents: Vec<u32>, coefficient: u64) -> Self {
        let mut polynomial = Self::zero(variables, modulus);
        polynomial.add_term(exponents, coefficient);
        polynomial
    }

    fn add_term(&mut self, exponents: Vec<u32>, coefficient: u64) {
        assert_eq!(exponents.len(), self.variables);
        let coefficient = coefficient % self.modulus;
        if coefficient == 0 {
            return;
        }
        let current = self.terms.get(&exponents).copied().unwrap_or(0);
        let updated = add_mod(current, coefficient, self.modulus);
        if updated == 0 {
            self.terms.remove(&exponents);
        } else {
            self.terms.insert(exponents, updated);
        }
    }

    fn assert_compatible(&self, other: &Self) {
        assert_eq!(self.variables, other.variables);
        assert_eq!(self.modulus, other.modulus);
    }

    fn add(&self, other: &Self) -> Self {
        self.assert_compatible(other);
        let mut result = self.clone();
        for (exponents, coefficient) in &other.terms {
            result.add_term(exponents.clone(), *coefficient);
        }
        result
    }

    fn sub(&self, other: &Self) -> Self {
        self.assert_compatible(other);
        let mut result = self.clone();
        for (exponents, coefficient) in &other.terms {
            result.add_term(exponents.clone(), neg_mod(*coefficient, self.modulus));
        }
        result
    }

    fn scale(&self, scalar: u64) -> Self {
        let mut result = Self::zero(self.variables, self.modulus);
        for (exponents, coefficient) in &self.terms {
            result.add_term(
                exponents.clone(),
                mul_mod(*coefficient, scalar, self.modulus),
            );
        }
        result
    }

    fn mul(&self, other: &Self) -> Self {
        self.assert_compatible(other);
        let mut result = Self::zero(self.variables, self.modulus);
        for (left_exponents, left_coefficient) in &self.terms {
            for (right_exponents, right_coefficient) in &other.terms {
                result.add_term(
                    left_exponents
                        .iter()
                        .zip(right_exponents)
                        .map(|(left, right)| left.checked_add(*right).expect("exponent overflow"))
                        .collect(),
                    mul_mod(*left_coefficient, *right_coefficient, self.modulus),
                );
            }
        }
        result
    }

    fn pow(&self, mut exponent: usize) -> Self {
        let mut result = Self::constant(self.variables, self.modulus, 1);
        let mut base = self.clone();
        while exponent != 0 {
            if exponent & 1 == 1 {
                result = result.mul(&base);
            }
            base = base.mul(&base);
            exponent >>= 1;
        }
        result
    }

    fn degree(&self) -> u32 {
        self.terms
            .keys()
            .map(|exponents| exponents.iter().sum())
            .max()
            .unwrap_or(0)
    }

    fn term_count(&self) -> usize {
        self.terms.len()
    }

    fn evaluate(&self, values: &[u64]) -> u64 {
        assert_eq!(values.len(), self.variables);
        self.terms.iter().fold(0, |sum, (exponents, coefficient)| {
            let term =
                values
                    .iter()
                    .zip(exponents)
                    .fold(*coefficient, |product, (value, exponent)| {
                        mul_mod(
                            product,
                            pow_mod(*value, u64::from(*exponent), self.modulus),
                            self.modulus,
                        )
                    });
            add_mod(sum, term, self.modulus)
        })
    }

    fn to_f4(&self) -> Poly {
        let terms: Vec<_> = self
            .terms
            .iter()
            .map(|(exponents, coefficient)| (exponents.clone(), *coefficient))
            .collect();
        f4_fp::normalise(&terms, self.modulus, Ordering::Grevlex)
    }
}

fn constant(variables: usize, modulus: u64, value: u64) -> SparsePolynomial {
    SparsePolynomial::constant(variables, modulus, value)
}

fn factor_base_polynomial(variable: &SparsePolynomial, roots: &[u64]) -> SparsePolynomial {
    roots.iter().fold(
        SparsePolynomial::constant(variable.variables, variable.modulus, 1),
        |product, root| {
            product.mul(&variable.sub(&constant(variable.variables, variable.modulus, *root)))
        },
    )
}

fn montgomery_f3(
    x1: &SparsePolynomial,
    x2: &SparsePolynomial,
    x3: &SparsePolynomial,
    a: u64,
    b: u64,
) -> SparsePolynomial {
    let pair_sum = x1.mul(x2).add(&x1.mul(x3)).add(&x2.mul(x3));
    pair_sum
        .sub(&constant(x1.variables, x1.modulus, b))
        .pow(2)
        .sub(
            &x1.mul(x2)
                .mul(x3)
                .mul(
                    &x1.add(x2)
                        .add(x3)
                        .add(&constant(x1.variables, x1.modulus, a)),
                )
                .scale(4),
        )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpectedOutcome {
    PlantedSolution,
    Inconsistent,
}

#[derive(Clone, Debug)]
struct CoordinateSystem {
    name: &'static str,
    coordinate_model: &'static str,
    control_role: &'static str,
    variable_names: Vec<String>,
    equations: Vec<SparsePolynomial>,
    known_solution: Option<Vec<u64>>,
    expected_outcome: ExpectedOutcome,
}

impl CoordinateSystem {
    fn variables(&self) -> usize {
        self.variable_names.len()
    }

    fn modulus(&self) -> u64 {
        self.equations[0].modulus
    }

    fn input_degree(&self) -> u32 {
        self.equations
            .iter()
            .map(SparsePolynomial::degree)
            .max()
            .unwrap_or(0)
    }

    fn input_monomials(&self) -> usize {
        self.equations
            .iter()
            .map(SparsePolynomial::term_count)
            .sum()
    }

    fn validates_known_solution(&self) -> bool {
        self.known_solution.as_ref().is_none_or(|solution| {
            self.equations
                .iter()
                .all(|equation| equation.evaluate(solution) == 0)
        })
    }

    fn f4_input(&self) -> Vec<Poly> {
        self.equations.iter().map(SparsePolynomial::to_f4).collect()
    }
}

fn montgomery_roots(fixture: &MatchedFixture) -> Vec<u64> {
    fixture
        .factor_points()
        .into_iter()
        .map(|point| affine(point).0)
        .collect()
}

fn quotient_roots(fixture: &MatchedFixture) -> Vec<u64> {
    fixture
        .factor_points()
        .into_iter()
        .map(|point| fixture.curve.w(point))
        .collect()
}

fn edwards_roots(fixture: &MatchedFixture) -> Vec<u64> {
    fixture
        .factor_points()
        .into_iter()
        .map(|point| affine(fixture.curve.to_twisted_edwards(point)).1)
        .collect()
}

fn montgomery_x_system(
    fixture: &MatchedFixture,
    name: &'static str,
    control_role: &'static str,
    target: Point,
    expected_outcome: ExpectedOutcome,
) -> CoordinateSystem {
    let variables = 2;
    let p = fixture.curve.p;
    let u1 = SparsePolynomial::variable(variables, p, 0);
    let u2 = SparsePolynomial::variable(variables, p, 1);
    let target_u = affine(target).0;
    let roots = montgomery_roots(fixture);
    CoordinateSystem {
        name,
        coordinate_model: "Montgomery x-only u",
        control_role,
        variable_names: vec!["u1".into(), "u2".into()],
        equations: vec![
            factor_base_polynomial(&u1, &roots),
            factor_base_polynomial(&u2, &roots),
            montgomery_f3(
                &u1,
                &u2,
                &constant(variables, p, target_u),
                fixture.curve.a,
                fixture.curve.b,
            ),
        ],
        known_solution: (expected_outcome == ExpectedOutcome::PlantedSolution)
            .then(|| vec![affine(fixture.left).0, affine(fixture.right).0]),
        expected_outcome,
    }
}

fn short_weierstrass_system(fixture: &MatchedFixture) -> CoordinateSystem {
    let variables = 2;
    let p = fixture.curve.p;
    let x1 = SparsePolynomial::variable(variables, p, 0);
    let x2 = SparsePolynomial::variable(variables, p, 1);
    let inverse_three = inv_mod(3, p);
    let inverse_twenty_seven = inv_mod(27, p);
    let shift = mul_mod(fixture.curve.a, inverse_three, p);
    let a_squared = mul_mod(fixture.curve.a, fixture.curve.a, p);
    let a_cubed = mul_mod(a_squared, fixture.curve.a, p);
    let short_a = sub_mod(fixture.curve.b, mul_mod(a_squared, inverse_three, p), p);
    let short_b = sub_mod(
        mul_mod(mul_mod(2, a_cubed, p), inverse_twenty_seven, p),
        mul_mod(
            mul_mod(fixture.curve.a, fixture.curve.b, p),
            inverse_three,
            p,
        ),
        p,
    );
    let to_x = |point| add_mod(affine(point).0, shift, p);
    let roots: Vec<_> = fixture.factor_points().into_iter().map(to_x).collect();
    let target = constant(variables, p, to_x(fixture.target));
    let relation = x1
        .sub(&x2)
        .pow(2)
        .mul(&target.pow(2))
        .sub(
            &x1.add(&x2)
                .mul(&x1.mul(&x2).add(&constant(variables, p, short_a)))
                .add(&constant(variables, p, mul_mod(2, short_b, p)))
                .mul(&target)
                .scale(2),
        )
        .add(&x1.mul(&x2).sub(&constant(variables, p, short_a)).pow(2))
        .sub(&x1.add(&x2).scale(mul_mod(4, short_b, p)));
    CoordinateSystem {
        name: "weierstrass-x-semaev",
        coordinate_model: "short Weierstrass x-only",
        control_role: "matched_model",
        variable_names: vec!["x1".into(), "x2".into()],
        equations: vec![
            factor_base_polynomial(&x1, &roots),
            factor_base_polynomial(&x2, &roots),
            relation,
        ],
        known_solution: Some(vec![to_x(fixture.left), to_x(fixture.right)]),
        expected_outcome: ExpectedOutcome::PlantedSolution,
    }
}

fn montgomery_affine_system(fixture: &MatchedFixture) -> CoordinateSystem {
    let variables = 5;
    let p = fixture.curve.p;
    let u1 = SparsePolynomial::variable(variables, p, 0);
    let v1 = SparsePolynomial::variable(variables, p, 1);
    let u2 = SparsePolynomial::variable(variables, p, 2);
    let v2 = SparsePolynomial::variable(variables, p, 3);
    let lambda = SparsePolynomial::variable(variables, p, 4);
    let (left_u, left_v) = affine(fixture.left);
    let (right_u, right_v) = affine(fixture.right);
    let (target_u, target_v) = affine(fixture.target);
    let roots = montgomery_roots(fixture);
    let curve_equation = |u: &SparsePolynomial, v: &SparsePolynomial| {
        v.pow(2)
            .scale(fixture.curve.capital_b)
            .sub(&u.pow(3))
            .sub(&u.pow(2).scale(fixture.curve.a))
            .sub(&u.scale(fixture.curve.b))
    };
    let slope = mul_mod(
        sub_mod(right_v, left_v, p),
        inv_mod(sub_mod(right_u, left_u, p), p),
        p,
    );
    let target_u_polynomial = constant(variables, p, target_u);
    let target_v_polynomial = constant(variables, p, target_v);
    CoordinateSystem {
        name: "montgomery-affine-slope",
        coordinate_model: "Montgomery affine (u,v,lambda)",
        control_role: "auxiliary_variable_candidate",
        variable_names: ["u1", "v1", "u2", "v2", "lambda"]
            .into_iter()
            .map(str::to_owned)
            .collect(),
        equations: vec![
            factor_base_polynomial(&u1, &roots),
            factor_base_polynomial(&u2, &roots),
            curve_equation(&u1, &v1),
            curve_equation(&u2, &v2),
            lambda.mul(&u2.sub(&u1)).sub(&v2.sub(&v1)),
            lambda
                .pow(2)
                .scale(fixture.curve.capital_b)
                .sub(&constant(variables, p, fixture.curve.a))
                .sub(&u1)
                .sub(&u2)
                .sub(&target_u_polynomial),
            lambda
                .mul(&u1.sub(&target_u_polynomial))
                .sub(&v1)
                .sub(&target_v_polynomial),
        ],
        known_solution: Some(vec![left_u, left_v, right_u, right_v, slope]),
        expected_outcome: ExpectedOutcome::PlantedSolution,
    }
}

fn quotient_system(fixture: &MatchedFixture, raw_norm: bool) -> CoordinateSystem {
    let variables = 2;
    let p = fixture.curve.p;
    let w1 = SparsePolynomial::variable(variables, p, 0);
    let w2 = SparsePolynomial::variable(variables, p, 1);
    let roots = quotient_roots(fixture);
    let quotient = fixture.curve.quotient_curve();
    let destination_x1 = w1.add(&constant(variables, p, fixture.curve.a));
    let destination_x2 = w2.add(&constant(variables, p, fixture.curve.a));
    let destination_x3 = constant(
        variables,
        p,
        add_mod(fixture.curve.w(fixture.target), fixture.curve.a, p),
    );
    let square_free = montgomery_f3(
        &destination_x1,
        &destination_x2,
        &destination_x3,
        quotient.a,
        quotient.b,
    );
    CoordinateSystem {
        name: if raw_norm {
            "quotient-w-raw-norm"
        } else {
            "quotient-w-squarefree"
        },
        coordinate_model: if raw_norm {
            "w=u+1/u, unsquare-free norm h3^4"
        } else {
            "w=u+1/u on the 2-isogenous curve"
        },
        control_role: if raw_norm {
            "multiplicity_candidate"
        } else {
            "quotient_reference"
        },
        variable_names: vec!["w1".into(), "w2".into()],
        equations: vec![
            factor_base_polynomial(&w1, &roots),
            factor_base_polynomial(&w2, &roots),
            if raw_norm {
                square_free.pow(4)
            } else {
                square_free
            },
        ],
        known_solution: Some(vec![
            fixture.curve.w(fixture.left),
            fixture.curve.w(fixture.right),
        ]),
        expected_outcome: ExpectedOutcome::PlantedSolution,
    }
}

fn edwards_y_system(fixture: &MatchedFixture) -> CoordinateSystem {
    let variables = 2;
    let p = fixture.curve.p;
    let y1 = SparsePolynomial::variable(variables, p, 0);
    let y2 = SparsePolynomial::variable(variables, p, 1);
    let roots = edwards_roots(fixture);
    let a = add_mod(fixture.curve.a, 2, p);
    let d = sub_mod(fixture.curve.a, 2, p);
    let target_y = affine(fixture.curve.to_twisted_edwards(fixture.target)).1;
    let y1_squared = y1.pow(2);
    let y2_squared = y2.pow(2);
    let coefficient = y1_squared
        .mul(&y2_squared)
        .scale(d)
        .sub(&y1_squared.scale(d))
        .sub(&y2_squared.scale(d))
        .add(&constant(variables, p, a));
    let relation = coefficient
        .scale(mul_mod(target_y, target_y, p))
        .add(
            &y1.mul(&y2)
                .scale(mul_mod(mul_mod(2, sub_mod(d, a, p), p), target_y, p)),
        )
        .add(
            &y1_squared
                .add(&y2_squared)
                .sub(&constant(variables, p, 1))
                .scale(a),
        )
        .sub(&y1_squared.mul(&y2_squared).scale(d));
    CoordinateSystem {
        name: "edwards-y-semaev",
        coordinate_model: "twisted Edwards y-only",
        control_role: "matched_model",
        variable_names: vec!["y1".into(), "y2".into()],
        equations: vec![
            factor_base_polynomial(&y1, &roots),
            factor_base_polynomial(&y2, &roots),
            relation,
        ],
        known_solution: Some(vec![
            affine(fixture.curve.to_twisted_edwards(fixture.left)).1,
            affine(fixture.curve.to_twisted_edwards(fixture.right)).1,
        ]),
        expected_outcome: ExpectedOutcome::PlantedSolution,
    }
}

fn edwards_bilinear_system(fixture: &MatchedFixture) -> CoordinateSystem {
    let variables = 7;
    let p = fixture.curve.p;
    let x1 = SparsePolynomial::variable(variables, p, 0);
    let y1 = SparsePolynomial::variable(variables, p, 1);
    let t1 = SparsePolynomial::variable(variables, p, 2);
    let x2 = SparsePolynomial::variable(variables, p, 3);
    let y2 = SparsePolynomial::variable(variables, p, 4);
    let t2 = SparsePolynomial::variable(variables, p, 5);
    let q = SparsePolynomial::variable(variables, p, 6);
    let one = constant(variables, p, 1);
    let roots = edwards_roots(fixture);
    let a = add_mod(fixture.curve.a, 2, p);
    let d = sub_mod(fixture.curve.a, 2, p);
    let curve_equation = |x: &SparsePolynomial, y: &SparsePolynomial, t: &SparsePolynomial| {
        x.pow(2)
            .scale(a)
            .add(&y.pow(2))
            .sub(&one)
            .sub(&t.pow(2).scale(d))
    };
    let (left_x, left_y) = affine(fixture.curve.to_twisted_edwards(fixture.left));
    let (right_x, right_y) = affine(fixture.curve.to_twisted_edwards(fixture.right));
    let (target_x, target_y) = affine(fixture.curve.to_twisted_edwards(fixture.target));
    let left_t = mul_mod(left_x, left_y, p);
    let right_t = mul_mod(right_x, right_y, p);
    let x_numerator = x1.mul(&y2).add(&y1.mul(&x2));
    let y_numerator = y1.mul(&y2).sub(&x1.mul(&x2).scale(a));
    CoordinateSystem {
        name: "edwards-affine-bilinear",
        coordinate_model: "twisted Edwards affine + product auxiliary",
        control_role: "bilinear_candidate",
        variable_names: ["x1", "y1", "t1", "x2", "y2", "t2", "q"]
            .into_iter()
            .map(str::to_owned)
            .collect(),
        equations: vec![
            factor_base_polynomial(&y1, &roots),
            factor_base_polynomial(&y2, &roots),
            t1.sub(&x1.mul(&y1)),
            t2.sub(&x2.mul(&y2)),
            curve_equation(&x1, &y1, &t1),
            curve_equation(&x2, &y2, &t2),
            q.sub(&t1.mul(&t2)),
            one.add(&q.scale(d)).scale(target_x).sub(&x_numerator),
            one.sub(&q.scale(d)).scale(target_y).sub(&y_numerator),
        ],
        known_solution: Some(vec![
            left_x,
            left_y,
            left_t,
            right_x,
            right_y,
            right_t,
            mul_mod(left_t, right_t, p),
        ]),
        expected_outcome: ExpectedOutcome::PlantedSolution,
    }
}

fn matched_systems(fixture: &MatchedFixture) -> Vec<CoordinateSystem> {
    vec![
        montgomery_x_system(
            fixture,
            "montgomery-u-semaev",
            "reference",
            fixture.target,
            ExpectedOutcome::PlantedSolution,
        ),
        montgomery_x_system(
            fixture,
            "montgomery-u-aa",
            "aa_control",
            fixture.target,
            ExpectedOutcome::PlantedSolution,
        ),
        short_weierstrass_system(fixture),
        montgomery_affine_system(fixture),
        quotient_system(fixture, false),
        quotient_system(fixture, true),
        edwards_y_system(fixture),
        edwards_bilinear_system(fixture),
        montgomery_x_system(
            fixture,
            "montgomery-u-unsatisfiable-control",
            "negative_control",
            fixture.curve.scalar_mul(37, fixture.generator),
            ExpectedOutcome::Inconsistent,
        ),
    ]
}

fn binomial(n: usize, k: usize) -> i128 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut result = 1_i128;
    for index in 0..k {
        result = result
            .checked_mul((n - index) as i128)
            .expect("Hilbert coefficient overflow")
            / (index + 1) as i128;
    }
    result
}

fn semi_regular_degree(system: &CoordinateSystem, search_limit: usize) -> Option<usize> {
    let variables = system.variables();
    let mut coefficients: Vec<i128> = (0..=search_limit)
        .map(|degree| binomial(variables + degree - 1, degree))
        .collect();
    for equation in &system.equations {
        let degree = equation.degree() as usize;
        if degree == 0 || degree > search_limit {
            continue;
        }
        let previous = coefficients.clone();
        for index in degree..=search_limit {
            coefficients[index] = previous[index] - previous[index - degree];
        }
    }
    (1..=search_limit).find(|degree| coefficients[*degree] <= 0)
}

fn divides(divisor: &[u32], dividend: &[u32]) -> bool {
    divisor
        .iter()
        .zip(dividend)
        .all(|(left, right)| left <= right)
}

fn affine_regularity_index(
    basis: &[Poly],
    variables: usize,
    enumeration_limit: usize,
) -> Option<usize> {
    if basis.is_empty() {
        return None;
    }
    let leading: Vec<&[u32]> = basis
        .iter()
        .filter_map(|polynomial| {
            polynomial
                .first()
                .map(|(exponents, _)| exponents.as_slice())
        })
        .collect();
    if leading
        .iter()
        .any(|monomial| monomial.iter().all(|exponent| *exponent == 0))
    {
        return None;
    }
    let mut bounds = vec![None; variables];
    for monomial in &leading {
        for variable in 0..variables {
            if monomial[variable] != 0
                && monomial
                    .iter()
                    .enumerate()
                    .all(|(index, exponent)| index == variable || *exponent == 0)
            {
                let exponent = monomial[variable] as usize;
                bounds[variable] =
                    Some(bounds[variable].map_or(exponent, |old: usize| old.min(exponent)));
            }
        }
    }
    let bounds: Vec<usize> = bounds.into_iter().collect::<Option<Vec<_>>>()?;
    let candidate_count = bounds
        .iter()
        .try_fold(1_usize, |count, bound| count.checked_mul(*bound))?;
    if candidate_count > enumeration_limit {
        return None;
    }
    fn visit(
        index: usize,
        exponents: &mut [u32],
        bounds: &[usize],
        leading: &[&[u32]],
        maximum: &mut usize,
    ) {
        if index == bounds.len() {
            if leading.iter().all(|monomial| !divides(monomial, exponents)) {
                *maximum = (*maximum).max(exponents.iter().map(|value| *value as usize).sum());
            }
            return;
        }
        for exponent in 0..bounds[index] {
            exponents[index] = exponent as u32;
            visit(index + 1, exponents, bounds, leading, maximum);
        }
    }
    let mut maximum = 0;
    visit(0, &mut vec![0; variables], &bounds, &leading, &mut maximum);
    Some(maximum + 1)
}

fn basis_max_degree(basis: &[Poly]) -> u32 {
    basis
        .iter()
        .flat_map(|polynomial| {
            polynomial
                .iter()
                .map(|(exponents, _)| exponents.iter().sum())
        })
        .max()
        .unwrap_or(0)
}

fn quadratic_power_pair(
    variables: usize,
    modulus: u64,
    exponent: u32,
    w_index: usize,
    fibre_product: u64,
) -> (SparsePolynomial, SparsePolynomial) {
    let w = SparsePolynomial::variable(variables, modulus, w_index);
    let mut constant_part = SparsePolynomial::constant(variables, modulus, 1);
    let mut linear_part = SparsePolynomial::zero(variables, modulus);
    for _ in 0..exponent {
        let next_constant = linear_part.scale(neg_mod(fibre_product, modulus));
        let next_linear = constant_part.add(&linear_part.mul(&w));
        constant_part = next_constant;
        linear_part = next_linear;
    }
    (constant_part, linear_part)
}

fn quadratic_norm(
    polynomial: &SparsePolynomial,
    fibre_index: usize,
    w_index: usize,
    fibre_product: u64,
) -> SparsePolynomial {
    let mut constant_part = SparsePolynomial::zero(polynomial.variables, polynomial.modulus);
    let mut linear_part = SparsePolynomial::zero(polynomial.variables, polynomial.modulus);
    for (exponents, coefficient) in &polynomial.terms {
        let mut base_exponents = exponents.clone();
        let fibre_exponent = base_exponents[fibre_index];
        base_exponents[fibre_index] = 0;
        let base = SparsePolynomial::monomial(
            polynomial.variables,
            polynomial.modulus,
            base_exponents,
            *coefficient,
        );
        let (power_constant, power_linear) = quadratic_power_pair(
            polynomial.variables,
            polynomial.modulus,
            fibre_exponent,
            w_index,
            fibre_product,
        );
        constant_part = constant_part.add(&base.mul(&power_constant));
        linear_part = linear_part.add(&base.mul(&power_linear));
    }
    let w = SparsePolynomial::variable(polynomial.variables, polynomial.modulus, w_index);
    constant_part
        .pow(2)
        .add(&constant_part.mul(&linear_part).mul(&w))
        .add(&linear_part.pow(2).scale(fibre_product))
}

fn raw_norm_identity_holds(curve: MontgomeryCurve) -> bool {
    let variables = 6;
    let source_coordinates: Vec<_> = (0..3)
        .map(|index| SparsePolynomial::variable(variables, curve.p, index))
        .collect();
    let quotient_coordinates: Vec<_> = (3..6)
        .map(|index| SparsePolynomial::variable(variables, curve.p, index))
        .collect();
    let source_f3 = montgomery_f3(
        &source_coordinates[0],
        &source_coordinates[1],
        &source_coordinates[2],
        curve.a,
        curve.b,
    );
    let raw_norm = (0..3).fold(source_f3, |norm, index| {
        quadratic_norm(&norm, index, index + 3, curve.b)
    });
    let destination = curve.quotient_curve();
    let destination_coordinates: Vec<_> = quotient_coordinates
        .iter()
        .map(|w| w.add(&constant(variables, curve.p, curve.a)))
        .collect();
    let square_free = montgomery_f3(
        &destination_coordinates[0],
        &destination_coordinates[1],
        &destination_coordinates[2],
        destination.a,
        destination.b,
    );
    raw_norm == square_free.pow(4)
}

fn radical_solution_sets_match(square_free: &CoordinateSystem, raw: &CoordinateSystem) -> bool {
    let roots = quotient_roots(&MatchedFixture::new());
    roots.iter().all(|left| {
        roots.iter().all(|right| {
            let values = [*left, *right];
            (square_free.equations[2].evaluate(&values) == 0)
                == (raw.equations[2].evaluate(&values) == 0)
        })
    })
}

#[derive(Clone, Debug, Serialize)]
pub struct SystemInventory {
    pub name: &'static str,
    pub coordinate_model: &'static str,
    pub control_role: &'static str,
    pub variables: usize,
    pub equations: usize,
    pub input_monomials: usize,
    pub max_input_degree: u32,
    pub semi_regular_degree: Option<usize>,
    pub expected_outcome: &'static str,
    pub known_solution_valid: bool,
}

fn inventory_row(system: &CoordinateSystem) -> SystemInventory {
    SystemInventory {
        name: system.name,
        coordinate_model: system.coordinate_model,
        control_role: system.control_role,
        variables: system.variables(),
        equations: system.equations.len(),
        input_monomials: system.input_monomials(),
        max_input_degree: system.input_degree(),
        semi_regular_degree: semi_regular_degree(system, 64),
        expected_outcome: match system.expected_outcome {
            ExpectedOutcome::PlantedSolution => "planted_solution",
            ExpectedOutcome::Inconsistent => "inconsistent",
        },
        known_solution_valid: system.validates_known_solution(),
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct FixtureCertificate {
    pub fixture_id: &'static str,
    pub scope: &'static str,
    pub production_curve25519_coefficient: u64,
    pub is_production_curve25519: bool,
    pub field_sha256: &'static str,
    pub source_curve_id: &'static str,
    pub source_curve_uid: &'static str,
    pub source_curve: MontgomeryCurve,
    pub source_equation: &'static str,
    pub source_group_order: u64,
    pub subgroup_order: u64,
    pub cofactor: u64,
    pub generator: Point,
    pub left: Point,
    pub right: Point,
    pub target: Point,
    pub planted_relation: &'static str,
    pub isogeny_degree: u64,
    pub isogeny_kernel: Point,
    pub destination_curve_id: &'static str,
    pub destination_curve_uid: &'static str,
    pub destination_curve: MontgomeryCurve,
    pub destination_generator: Point,
    pub destination_equation: String,
    pub map_x: &'static str,
    pub map_y: &'static str,
    pub quotient_coordinate: &'static str,
    pub subgroup_degree_gcd: u64,
    pub subgroup_map_checks: usize,
    pub subgroup_map_passed: bool,
    pub raw_norm_identity: &'static str,
    pub raw_norm_identity_passed: bool,
    pub hessian_included: bool,
    pub hessian_exclusion: &'static str,
}

fn fixture_certificate(fixture: &MatchedFixture) -> FixtureCertificate {
    let mapped_generator = fixture.curve.isogeny(fixture.generator);
    let subgroup_map_passed = (0..FIXTURE_SUBGROUP_ORDER).all(|scalar| {
        fixture
            .curve
            .isogeny(fixture.curve.scalar_mul(scalar, fixture.generator))
            == fixture
                .curve
                .quotient_curve()
                .scalar_mul(scalar, mapped_generator)
    });
    let destination = fixture.curve.quotient_curve();
    FixtureCertificate {
        fixture_id: FIXTURE_ID,
        scope: "public deterministic toy for exact identities and F4 qualification only",
        production_curve25519_coefficient: CURVE25519_A,
        is_production_curve25519: false,
        field_sha256: FIXTURE_FIELD_SHA256,
        source_curve_id: FIXTURE_SOURCE_CURVE_ID,
        source_curve_uid: FIXTURE_SOURCE_CURVE_UID,
        source_curve: fixture.curve,
        source_equation: "v^2 = u^3 + 5u^2 + u over F_1019",
        source_group_order: fixture.curve.point_count(),
        subgroup_order: FIXTURE_SUBGROUP_ORDER,
        cofactor: FIXTURE_COFACTOR,
        generator: fixture.generator,
        left: fixture.left,
        right: fixture.right,
        target: fixture.target,
        planted_relation: "[9]G + [27]G = [36]G",
        isogeny_degree: 2,
        isogeny_kernel: Point::Affine { x: 0, y: 0 },
        destination_curve_id: FIXTURE_DESTINATION_CURVE_ID,
        destination_curve_uid: FIXTURE_DESTINATION_CURVE_UID,
        destination_curve: destination,
        destination_generator: mapped_generator,
        destination_equation: format!(
            "v^2 = U^3 + {}U^2 + {}U over F_{}",
            destination.a, destination.b, destination.p
        ),
        map_x: "U = u + A + 1/u",
        map_y: "V = v(1 - 1/u^2)",
        quotient_coordinate: "w = u + 1/u; U = w + A",
        subgroup_degree_gcd: gcd(2, FIXTURE_SUBGROUP_ORDER),
        subgroup_map_checks: FIXTURE_SUBGROUP_ORDER as usize,
        subgroup_map_passed,
        raw_norm_identity: "Norm_{u_i^2-w_i*u_i+1}(f3) = h3(w1,w2,w3)^4",
        raw_norm_identity_passed: raw_norm_identity_holds(fixture.curve),
        hessian_included: false,
        hessian_exclusion: "#E(F_1019)=1004 is not divisible by 3; a smooth rational Hessian model has rational 3-torsion and would require a different fixture",
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct DegreeInventory {
    pub schema: &'static str,
    pub fixture: FixtureCertificate,
    pub systems: Vec<SystemInventory>,
    pub controls: InventoryControls,
    pub scope_boundary: &'static str,
}

#[derive(Clone, Debug, Serialize)]
pub struct InventoryControls {
    pub all_planted_solutions_validate: bool,
    pub raw_norm_radical_matches_squarefree: bool,
    pub aa_equations_identical: bool,
}

pub fn degree_inventory() -> DegreeInventory {
    let fixture = MatchedFixture::new();
    let systems = matched_systems(&fixture);
    let reference = &systems[0];
    let aa = &systems[1];
    let square_free = &systems[4];
    let raw = &systems[5];
    DegreeInventory {
        schema: "ic.coordinate_rosetta.degree_inventory/v1",
        fixture: fixture_certificate(&fixture),
        systems: systems.iter().map(inventory_row).collect(),
        controls: InventoryControls {
            all_planted_solutions_validate: systems
                .iter()
                .all(CoordinateSystem::validates_known_solution),
            raw_norm_radical_matches_squarefree: radical_solution_sets_match(square_free, raw),
            aa_equations_identical: reference.variable_names == aa.variable_names
                && reference.equations == aa.equations
                && reference.known_solution == aa.known_solution,
        },
        scope_boundary: "The fixture preserves the relevant Montgomery 2-torsion/isogeny geometry but is not Curve25519; observations are toy-scope and do not establish an ECDLP speedup.",
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct DegreeStudyConfig {
    pub protocol_sha256: String,
    pub instrument_commit: String,
    pub monomial_order: &'static str,
    pub max_degree: u32,
    pub standard_monomial_limit: usize,
    pub wall_time_admissible: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct SystemObservation {
    pub name: &'static str,
    pub coordinate_model: &'static str,
    pub control_role: &'static str,
    pub expected_outcome: ExpectedOutcome,
    pub variables: usize,
    pub equations: usize,
    pub input_monomials: usize,
    pub max_input_degree: u32,
    pub semi_regular_degree: Option<usize>,
    pub known_solution_valid: bool,
    pub solver_complete: bool,
    pub expected_outcome_passed: bool,
    pub inconsistent: bool,
    pub pairs_above_bound: usize,
    pub selected_pairs: usize,
    pub f4_steps: usize,
    pub productive_solving_degree: u32,
    pub total_solving_degree: u32,
    pub raw_max_step_degree: u32,
    pub affine_regularity_index: Option<usize>,
    pub basis_max_degree: u32,
    pub basis_polynomials: usize,
    pub basis_monomials: usize,
    pub max_rows: usize,
    pub max_columns: usize,
    pub field_multiplications: u64,
}

fn observe_system(
    system: &CoordinateSystem,
    max_degree: u32,
    standard_monomial_limit: usize,
) -> SystemObservation {
    let input = system.f4_input();
    let mut options = F4Options::new(Ordering::Grevlex, max_degree);
    options.capture_run_log = true;
    options.record_hilbert_diagnostics = true;
    let _ = f4_fp::take_run_log();
    let report = f4_fp::f4(&input, system.variables(), system.modulus(), &options);
    let run_log = f4_fp::take_run_log();
    let solver_complete = !report.timed_out
        && report.resource_limit.is_none()
        && report.pairs_above_bound == 0
        && report.staircase_at_stop.is_none();
    let known_solution_valid = system.validates_known_solution();
    let expected_outcome_passed = match system.expected_outcome {
        ExpectedOutcome::PlantedSolution => known_solution_valid && !report.inconsistent,
        ExpectedOutcome::Inconsistent => report.inconsistent,
    };
    let input_degree = system.input_degree();
    SystemObservation {
        name: system.name,
        coordinate_model: system.coordinate_model,
        control_role: system.control_role,
        expected_outcome: system.expected_outcome,
        variables: system.variables(),
        equations: system.equations.len(),
        input_monomials: system.input_monomials(),
        max_input_degree: input_degree,
        semi_regular_degree: semi_regular_degree(system, 64),
        known_solution_valid,
        solver_complete,
        expected_outcome_passed,
        inconsistent: report.inconsistent,
        pairs_above_bound: report.pairs_above_bound,
        selected_pairs: run_log.steps.iter().map(|step| step.pairs).sum(),
        f4_steps: report.steps,
        productive_solving_degree: report.solving_degree_max,
        total_solving_degree: input_degree.max(report.solving_degree_max),
        raw_max_step_degree: input_degree.max(report.degree_reached),
        affine_regularity_index: solver_complete
            .then(|| {
                affine_regularity_index(&report.basis, system.variables(), standard_monomial_limit)
            })
            .flatten(),
        basis_max_degree: basis_max_degree(&report.basis),
        basis_polynomials: report.basis.len(),
        basis_monomials: report.basis.iter().map(Vec::len).sum(),
        max_rows: report.max_rows,
        max_columns: report.max_cols,
        field_multiplications: report.field_ops,
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct DegreeStudyControls {
    pub inventory_passed: bool,
    pub aa_observation_identical: bool,
    pub positive_controls_passed: bool,
    pub negative_control_passed: bool,
    pub all_solver_runs_complete: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct DegreeStudyReport {
    pub schema: &'static str,
    pub evidence_status: &'static str,
    pub config: DegreeStudyConfig,
    pub fixture: FixtureCertificate,
    pub observations: Vec<SystemObservation>,
    pub controls: DegreeStudyControls,
    pub solving_degree_convention: &'static str,
    pub regularity_convention: &'static str,
    pub limitations: Vec<&'static str>,
}

fn observation_signature(
    observation: &SystemObservation,
) -> (
    bool,
    bool,
    usize,
    usize,
    u32,
    u32,
    Option<usize>,
    u32,
    usize,
    usize,
    u64,
) {
    (
        observation.solver_complete,
        observation.inconsistent,
        observation.selected_pairs,
        observation.f4_steps,
        observation.productive_solving_degree,
        observation.total_solving_degree,
        observation.affine_regularity_index,
        observation.basis_max_degree,
        observation.max_rows,
        observation.max_columns,
        observation.field_multiplications,
    )
}

pub fn run_degree_study(
    protocol_sha256: String,
    instrument_commit: String,
    max_degree: u32,
) -> Result<DegreeStudyReport, String> {
    if protocol_sha256.len() != 64 || !protocol_sha256.bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        return Err("--protocol-sha256 must be 64 hexadecimal characters".into());
    }
    if instrument_commit.len() < 10
        || !instrument_commit
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        return Err("--instrument-commit must be a hexadecimal Git commit".into());
    }
    if max_degree < 16 {
        return Err("--max-degree must be at least 16 for the raw-norm arm".into());
    }
    let inventory = degree_inventory();
    let fixture = MatchedFixture::new();
    let systems = matched_systems(&fixture);
    let standard_monomial_limit = 1_000_000;
    let observations: Vec<_> = systems
        .iter()
        .map(|system| observe_system(system, max_degree, standard_monomial_limit))
        .collect();
    let reference = observations
        .iter()
        .find(|observation| observation.name == "montgomery-u-semaev")
        .ok_or("reference observation missing")?;
    let aa = observations
        .iter()
        .find(|observation| observation.name == "montgomery-u-aa")
        .ok_or("A/A observation missing")?;
    let positive_controls_passed = observations
        .iter()
        .filter(|observation| observation.expected_outcome == ExpectedOutcome::PlantedSolution)
        .all(|observation| observation.expected_outcome_passed);
    let negative_control_passed = observations
        .iter()
        .find(|observation| observation.expected_outcome == ExpectedOutcome::Inconsistent)
        .is_some_and(|observation| observation.expected_outcome_passed);
    let all_solver_runs_complete = observations
        .iter()
        .all(|observation| observation.solver_complete);
    let aa_observation_identical = observation_signature(reference) == observation_signature(aa);
    let inventory_passed = inventory.controls.all_planted_solutions_validate
        && inventory.controls.raw_norm_radical_matches_squarefree
        && inventory.controls.aa_equations_identical
        && inventory.fixture.subgroup_map_passed
        && inventory.fixture.raw_norm_identity_passed
        && inventory.fixture.source_group_order == 1_004;
    Ok(DegreeStudyReport {
        schema: "ic.coordinate_rosetta.degree_study/v1",
        evidence_status: "pcc:unverified",
        config: DegreeStudyConfig {
            protocol_sha256,
            instrument_commit,
            monomial_order: "grevlex",
            max_degree,
            standard_monomial_limit,
            wall_time_admissible: false,
        },
        fixture: inventory.fixture,
        observations,
        controls: DegreeStudyControls {
            inventory_passed,
            aa_observation_identical,
            positive_controls_passed,
            negative_control_passed,
            all_solver_runs_complete,
        },
        solving_degree_convention: "total_solving_degree is max(max input total degree, highest productive F4 step); raw_max_step_degree additionally includes trailing certification steps",
        regularity_convention: "affine_regularity_index is one plus the maximum total degree of a standard monomial of the completed grevlex leading ideal",
        limitations: vec![
            "single deterministic F_1019 fixture with a two-element factor base",
            "no local wall time is reported or admissible",
            "selected_pairs are F4 pair rows, not Buchberger pair counts from another engine",
            "no external-CAS replay is included in this qualification output",
            "no result transfers to production Curve25519 or establishes an ECDLP advantage",
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_group_and_two_isogeny_are_exact() {
        let fixture = MatchedFixture::new();
        assert_eq!(fixture.generator, Point::Affine { x: 328, y: 796 });
        assert_eq!(fixture.curve.point_count(), 1_004);
        assert_eq!(
            fixture.target,
            fixture.curve.scalar_mul(36, fixture.generator)
        );
        assert_eq!(
            fixture
                .curve
                .scalar_mul(FIXTURE_SUBGROUP_ORDER, fixture.generator),
            Point::Infinity
        );
        assert_eq!(gcd(2, FIXTURE_SUBGROUP_ORDER), 1);
        let certificate = fixture_certificate(&fixture);
        assert!(certificate.subgroup_map_passed);
        assert_eq!(
            certificate.subgroup_map_checks,
            FIXTURE_SUBGROUP_ORDER as usize
        );
        assert!(certificate
            .destination_curve
            .is_on_curve(fixture.curve.isogeny(fixture.left)));
        assert_eq!(
            affine(fixture.curve.isogeny(fixture.left)).0,
            add_mod(
                fixture.curve.w(fixture.left),
                fixture.curve.a,
                fixture.curve.p
            )
        );
    }

    #[test]
    fn every_native_system_encodes_the_same_planted_relation() {
        let fixture = MatchedFixture::new();
        let systems = matched_systems(&fixture);
        for system in systems
            .iter()
            .filter(|system| system.expected_outcome == ExpectedOutcome::PlantedSolution)
        {
            assert!(
                system.validates_known_solution(),
                "{} rejected the relation",
                system.name
            );
        }
        let degrees: Vec<_> = systems[..8]
            .iter()
            .map(CoordinateSystem::input_degree)
            .collect();
        assert_eq!(degrees, vec![4, 4, 4, 3, 4, 16, 4, 2]);
    }

    #[test]
    fn quotient_norm_factors_as_the_fourth_power() {
        let fixture = MatchedFixture::new();
        assert!(raw_norm_identity_holds(fixture.curve));
        let systems = matched_systems(&fixture);
        assert!(radical_solution_sets_match(&systems[4], &systems[5]));
    }

    #[test]
    fn aa_and_negative_controls_are_well_formed() {
        let fixture = MatchedFixture::new();
        let systems = matched_systems(&fixture);
        assert_eq!(systems[0].equations, systems[1].equations);
        assert_eq!(systems[0].known_solution, systems[1].known_solution);
        assert_eq!(systems[8].expected_outcome, ExpectedOutcome::Inconsistent);
        assert!(systems[8].known_solution.is_none());
        let roots = montgomery_roots(&fixture);
        assert!(roots.iter().all(|left| {
            roots
                .iter()
                .all(|right| systems[8].equations[2].evaluate(&[*left, *right]) != 0)
        }));
    }

    #[test]
    fn hessian_is_excluded_by_rational_three_torsion() {
        let fixture = MatchedFixture::new();
        assert_eq!(fixture.curve.point_count() % 3, 2);
        assert!(!fixture_certificate(&fixture).hessian_included);
    }

    #[test]
    fn inventory_controls_pass_without_running_f4() {
        let inventory = degree_inventory();
        assert!(inventory.controls.all_planted_solutions_validate);
        assert!(inventory.controls.raw_norm_radical_matches_squarefree);
        assert!(inventory.controls.aa_equations_identical);
        assert!(inventory.fixture.raw_norm_identity_passed);
    }
}
