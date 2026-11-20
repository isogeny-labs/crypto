//! Independent Picard-class oracle core for a split-sextic prime cover.
//!
//! This module does not use the production balanced enumerator, Cantor
//! composition/reduction, negation, scalar multiplication, or target group
//! law to construct its oracle. For the configured monic split sextic and
//! `D_0 = infinity_+ + infinity_-`,
//! every canonical balanced class is `[E-D_0]` for an effective degree-two
//! divisor `E`.  Since `L(3D_0)=<1,x,x^2,x^3,y>`, the sum is characterized by
//!
//!     [E_1-D_0] + [E_2-D_0] = [E_3-D_0]
//!       iff E_1 + E_2 + iota(E_3) = div(F) + 3D_0.
//!
//! We separately enumerate every projective function in `L(3D_0)` and
//! retain the complete zero divisors, including infinity, that can match sums
//! of canonical graph places. Tiny byte-polynomial and quadratic-field
//! implementations then check the entire group table and both transfer
//! compositions.
//! This is exhaustive evidence for this one fixture, not a general proof,
//! scaling result, or solver.

use super::{
    COVER_SHIFT, CURVE_COEFFICIENTS, EXPECTED_JACOBIAN_ORDER, EXPECTED_PRINCIPAL_ZERO_DIVISORS,
    EXPECTED_SOURCE_FQ2_POINTS, EXPECTED_SOURCE_FQ_POINTS, EXPECTED_SUPPORT_COUNTERS,
    EXPECTED_TARGET_ORDER, EXTENSION_NON_SQUARE, MONTGOMERY_MODEL, P, TARGET_A, TARGET_B,
};
use crypto_lib::prime_hyperelliptic::{
    BalancedDivisorP, FpPoly, PrimeEllipticPointP, PrimeMontgomeryPointP,
    PrimeMontgomeryQuadraticCoverP, PrimeQuadraticCoverP,
};
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use std::collections::{BTreeMap, BTreeSet};

fn fp(value: i16) -> u8 {
    value.rem_euclid(i16::from(P)) as u8
}

fn fp_inv(value: u8) -> u8 {
    assert_ne!(value, 0);
    (1..P)
        .find(|candidate| value * candidate % P == 1)
        .expect("nonzero base-field element is invertible")
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Poly(Vec<u8>);

impl Poly {
    fn new(mut coefficients: Vec<u8>) -> Self {
        for coefficient in &mut coefficients {
            *coefficient %= P;
        }
        while coefficients.last() == Some(&0) {
            coefficients.pop();
        }
        Self(coefficients)
    }

    fn zero() -> Self {
        Self(Vec::new())
    }

    fn one() -> Self {
        Self(vec![1])
    }

    fn degree(&self) -> Option<usize> {
        self.0.len().checked_sub(1)
    }

    fn coefficient(&self, index: usize) -> u8 {
        self.0.get(index).copied().unwrap_or(0)
    }

    fn is_zero(&self) -> bool {
        self.0.is_empty()
    }

    fn add(&self, other: &Self) -> Self {
        let length = self.0.len().max(other.0.len());
        Self::new(
            (0..length)
                .map(|index| (self.coefficient(index) + other.coefficient(index)) % P)
                .collect(),
        )
    }

    fn neg(&self) -> Self {
        Self::new(
            self.0
                .iter()
                .map(|coefficient| fp(-i16::from(*coefficient)))
                .collect(),
        )
    }

    fn sub(&self, other: &Self) -> Self {
        self.add(&other.neg())
    }

    fn scalar_mul(&self, scalar: u8) -> Self {
        Self::new(
            self.0
                .iter()
                .map(|coefficient| coefficient * scalar % P)
                .collect(),
        )
    }

    fn mul(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }
        let mut result = vec![0; self.0.len() + other.0.len() - 1];
        for (left_index, left) in self.0.iter().enumerate() {
            for (right_index, right) in other.0.iter().enumerate() {
                result[left_index + right_index] =
                    (result[left_index + right_index] + left * right) % P;
            }
        }
        Self::new(result)
    }

    fn square(&self) -> Self {
        self.mul(self)
    }

    fn derivative(&self) -> Self {
        Self::new(
            self.0
                .iter()
                .enumerate()
                .skip(1)
                .map(|(degree, coefficient)| (degree as u8 % P) * coefficient % P)
                .collect(),
        )
    }

    fn monic(&self) -> Self {
        let degree = self
            .degree()
            .expect("zero polynomial has no monic associate");
        self.scalar_mul(fp_inv(self.coefficient(degree)))
    }

    fn divrem_monic(&self, denominator: &Self) -> (Self, Self) {
        let denominator_degree = denominator
            .degree()
            .expect("polynomial division by zero is undefined");
        assert_eq!(denominator.coefficient(denominator_degree), 1);
        let mut remainder = self.clone();
        let mut quotient = Vec::<u8>::new();
        while let Some(remainder_degree) = remainder.degree() {
            if remainder_degree < denominator_degree {
                break;
            }
            let shift = remainder_degree - denominator_degree;
            if quotient.len() <= shift {
                quotient.resize(shift + 1, 0);
            }
            let leading = remainder.coefficient(remainder_degree);
            quotient[shift] = (quotient[shift] + leading) % P;
            let mut subtract = vec![0; shift + denominator.0.len()];
            for (index, coefficient) in denominator.0.iter().enumerate() {
                subtract[shift + index] = coefficient * leading % P;
            }
            remainder = remainder.sub(&Self::new(subtract));
        }
        (Self::new(quotient), remainder)
    }

    fn rem(&self, denominator: &Self) -> Self {
        self.divrem_monic(denominator).1
    }

    fn gcd_monic(&self, other: &Self) -> Self {
        let mut left = self.clone();
        let mut right = other.clone();
        while !right.is_zero() {
            right = right.monic();
            let remainder = left.rem(&right);
            left = right;
            right = remainder;
        }
        assert!(!left.is_zero());
        left.monic()
    }

    fn eval_base(&self, value: u8) -> u8 {
        self.0.iter().rev().fold(0, |accumulator, coefficient| {
            (accumulator * value + coefficient) % P
        })
    }

    fn compose_neg(&self) -> Self {
        Self::new(
            self.0
                .iter()
                .enumerate()
                .map(|(index, coefficient)| {
                    if index % 2 == 0 {
                        *coefficient
                    } else {
                        fp(-i16::from(*coefficient))
                    }
                })
                .collect(),
        )
    }
}

fn curve_polynomial() -> Poly {
    assert_eq!(CURVE_COEFFICIENTS.len(), 7);
    assert_eq!(CURVE_COEFFICIENTS[6], 1);
    Poly::new(CURVE_COEFFICIENTS.to_vec())
}

fn monic_polynomials(degree: usize) -> Vec<Poly> {
    let count = usize::from(P).pow(degree as u32);
    (0..count)
        .map(|mut code| {
            let mut coefficients = Vec::with_capacity(degree + 1);
            for _ in 0..degree {
                coefficients.push((code % usize::from(P)) as u8);
                code /= usize::from(P);
            }
            coefficients.push(1);
            Poly::new(coefficients)
        })
        .collect()
}

fn irreducibles_through_degree_three() -> Vec<Poly> {
    let mut irreducibles = Vec::new();
    for degree in 1..=3 {
        for polynomial in monic_polynomials(degree) {
            if degree == 1 || (0..P).all(|root| polynomial.eval_base(root) != 0) {
                irreducibles.push(polynomial);
            }
        }
    }
    irreducibles
}

/// Factor a nonzero polynomial of degree at most six. Any reducible
/// polynomial in this range has an irreducible factor of degree at most three.
fn factor_small(polynomial: &Poly) -> Vec<(Poly, u8)> {
    assert!(!polynomial.is_zero());
    assert!(polynomial.degree().unwrap() <= 6);
    let irreducibles = irreducibles_through_degree_three();
    let mut remaining = polynomial.monic();
    let mut factors = Vec::new();
    for candidate in &irreducibles {
        let mut multiplicity = 0u8;
        loop {
            let (quotient, remainder) = remaining.divrem_monic(candidate);
            if !remainder.is_zero() {
                break;
            }
            remaining = quotient;
            multiplicity += 1;
            if remaining.degree() == Some(0) {
                break;
            }
        }
        if multiplicity != 0 {
            factors.push((candidate.clone(), multiplicity));
        }
        if remaining.degree() == Some(0) {
            break;
        }
    }
    if remaining.degree().unwrap_or(0) > 0 {
        assert!(irreducibles
            .iter()
            .all(|candidate| !remaining.rem(candidate).is_zero()));
        factors.push((remaining, 1));
    }
    factors
}

/// A finite prime divisor `(h(x), y-v(x))`.  Multiplicity is stored by the
/// containing effective divisor; thus equality below compares product-ideal
/// valuations, not just the set of geometric support points.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Place {
    h: Poly,
    v: Poly,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct EffectiveDivisor {
    finite: BTreeMap<Place, u8>,
    infinity_plus: u8,
    infinity_minus: u8,
}

impl EffectiveDivisor {
    fn add_place(&mut self, place: Place, multiplicity: u8) {
        *self.finite.entry(place).or_default() += multiplicity;
    }

    fn degree(&self) -> usize {
        self.finite
            .iter()
            .map(|(place, multiplicity)| place.h.degree().unwrap() * usize::from(*multiplicity))
            .sum::<usize>()
            + usize::from(self.infinity_plus)
            + usize::from(self.infinity_minus)
    }

    fn add(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for (place, multiplicity) in &other.finite {
            result.add_place(place.clone(), *multiplicity);
        }
        result.infinity_plus += other.infinity_plus;
        result.infinity_minus += other.infinity_minus;
        result
    }

    fn involution(&self) -> Self {
        let mut result = Self {
            finite: BTreeMap::new(),
            infinity_plus: self.infinity_minus,
            infinity_minus: self.infinity_plus,
        };
        for (place, multiplicity) in &self.finite {
            result.add_place(
                Place {
                    h: place.h.clone(),
                    v: place.v.neg().rem(&place.h),
                },
                *multiplicity,
            );
        }
        result
    }

    fn deck_transform(&self) -> Self {
        let mut result = Self {
            finite: BTreeMap::new(),
            infinity_plus: self.infinity_minus,
            infinity_minus: self.infinity_plus,
        };
        for (place, multiplicity) in &self.finite {
            let degree = place.h.degree().unwrap();
            let transformed_h = Poly::new(
                place
                    .h
                    .0
                    .iter()
                    .enumerate()
                    .map(|(index, coefficient)| {
                        if (degree + index) % 2 == 0 {
                            *coefficient
                        } else {
                            fp(-i16::from(*coefficient))
                        }
                    })
                    .collect(),
            );
            assert_eq!(transformed_h.coefficient(degree), 1);
            result.add_place(
                Place {
                    v: place.v.compose_neg().rem(&transformed_h),
                    h: transformed_h,
                },
                *multiplicity,
            );
        }
        result
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ClassTriple {
    u: Poly,
    v: Poly,
    n_plus: u8,
}

fn valid_class(triple: &ClassTriple) -> bool {
    curve_polynomial()
        .sub(&triple.v.square())
        .rem(&triple.u)
        .is_zero()
}

fn enumerate_classes() -> Vec<ClassTriple> {
    let mut classes = Vec::new();
    for n_plus in 0..=2 {
        classes.push(ClassTriple {
            u: Poly::one(),
            v: Poly::zero(),
            n_plus,
        });
    }
    for u in monic_polynomials(1) {
        for v0 in 0..P {
            let v = Poly::new(vec![v0]);
            for n_plus in 0..=1 {
                let triple = ClassTriple {
                    u: u.clone(),
                    v: v.clone(),
                    n_plus,
                };
                if valid_class(&triple) {
                    classes.push(triple);
                }
            }
        }
    }
    for u in monic_polynomials(2) {
        for v0 in 0..P {
            for v1 in 0..P {
                let triple = ClassTriple {
                    u: u.clone(),
                    v: Poly::new(vec![v0, v1]),
                    n_plus: 0,
                };
                if valid_class(&triple) {
                    classes.push(triple);
                }
            }
        }
    }
    classes
}

fn effective_divisor(triple: &ClassTriple) -> EffectiveDivisor {
    let support_degree = triple.u.degree().unwrap();
    let mut divisor = EffectiveDivisor {
        finite: BTreeMap::new(),
        infinity_plus: triple.n_plus,
        infinity_minus: (2 - support_degree) as u8 - triple.n_plus,
    };
    if support_degree != 0 {
        for (factor, multiplicity) in factor_small(&triple.u) {
            divisor.add_place(
                Place {
                    v: triple.v.rem(&factor),
                    h: factor,
                },
                multiplicity,
            );
        }
    }
    assert_eq!(divisor.degree(), 2);
    divisor
}

fn residue_y_roots(factor: &Poly) -> Vec<Poly> {
    let degree = factor.degree().unwrap();
    let count = usize::from(P).pow(degree as u32);
    (0..count)
        .filter_map(|mut code| {
            let mut coefficients = Vec::with_capacity(degree);
            for _ in 0..degree {
                coefficients.push((code % usize::from(P)) as u8);
                code /= usize::from(P);
            }
            let candidate = Poly::new(coefficients);
            candidate
                .square()
                .sub(&curve_polynomial())
                .rem(factor)
                .is_zero()
                .then_some(candidate)
        })
        .collect()
}

/// Complete zero divisor of `y+a(x)`, after adding `3D_0`.
fn zeros_y_plus_a(a: &Poly) -> EffectiveDivisor {
    let norm = curve_polynomial().sub(&a.square());
    assert!(!norm.is_zero());
    let norm_degree = norm.degree().unwrap();
    let mut divisor = EffectiveDivisor::default();
    for (factor, multiplicity) in factor_small(&norm) {
        divisor.add_place(
            Place {
                v: a.neg().rem(&factor),
                h: factor,
            },
            multiplicity,
        );
    }
    // y/x^3 tends to +1 at infinity_+ and -1 at infinity_-.
    // If the cubic terms cancel, the norm degree records the exact remaining
    // valuation, including higher-order cancellation.
    if a.coefficient(3) == P - 1 {
        divisor.infinity_plus = (6 - norm_degree) as u8;
    } else if a.coefficient(3) == 1 {
        divisor.infinity_minus = (6 - norm_degree) as u8;
    } else {
        assert_eq!(norm_degree, 6);
    }
    assert_eq!(divisor.degree(), 6);
    divisor
}

/// Complete zero divisor of `a(x)`, after adding `3D_0`.
///
/// A nonsplit x-place cannot equal a sum of graph places from canonical
/// balanced representatives. Such a function is omitted from the lookup,
/// rather than weakened to a support-only congruence.
fn zeros_polynomial(a: &Poly) -> Option<EffectiveDivisor> {
    let degree = a.degree().unwrap();
    let mut divisor = EffectiveDivisor {
        finite: BTreeMap::new(),
        infinity_plus: (3 - degree) as u8,
        infinity_minus: (3 - degree) as u8,
    };
    for (factor, multiplicity) in factor_small(a) {
        let roots = residue_y_roots(&factor);
        match roots.len() {
            0 => return None,
            1 => {
                assert!(roots[0].is_zero(), "unique residue root must be ramified");
                divisor.add_place(
                    Place {
                        h: factor,
                        v: Poly::zero(),
                    },
                    2 * multiplicity,
                );
            }
            2 => {
                for root in roots {
                    divisor.add_place(
                        Place {
                            h: factor.clone(),
                            v: root,
                        },
                        multiplicity,
                    );
                }
            }
            count => panic!("quadratic cover had {count} residue roots"),
        }
    }
    assert_eq!(divisor.degree(), 6);
    Some(divisor)
}

fn principal_zero_divisors() -> BTreeSet<EffectiveDivisor> {
    let mut divisors = BTreeSet::new();

    // Projective functions with nonzero y coefficient normalize to y+a(x).
    for code in 0..usize::from(P).pow(4) {
        let mut digits = code;
        let mut coefficients = Vec::with_capacity(4);
        for _ in 0..4 {
            coefficients.push((digits % usize::from(P)) as u8);
            digits /= usize::from(P);
        }
        divisors.insert(zeros_y_plus_a(&Poly::new(coefficients)));
    }

    // With zero y coefficient, normalize the leading coefficient of a(x).
    // Together the loops cover p^4 + sum_{d=0}^3 p^d projective functions.
    for degree in 0..=3 {
        for polynomial in monic_polynomials(degree) {
            if let Some(divisor) = zeros_polynomial(&polynomial) {
                divisors.insert(divisor);
            }
        }
    }
    divisors
}

fn oracle_group_table(
    effective: &[EffectiveDivisor],
    principal: &BTreeSet<EffectiveDivisor>,
) -> Vec<Vec<usize>> {
    let mut table = vec![vec![usize::MAX; effective.len()]; effective.len()];
    for (left_index, left) in effective.iter().enumerate() {
        for (right_index, right) in effective.iter().enumerate() {
            let partial = left.add(right);
            let matches = effective
                .iter()
                .enumerate()
                .filter_map(|(candidate_index, candidate)| {
                    // This is exact equality of every prime-place valuation
                    // (and both infinity valuations), hence of product ideals.
                    principal
                        .contains(&partial.add(&candidate.involution()))
                        .then_some(candidate_index)
                })
                .collect::<Vec<_>>();
            assert_eq!(
                matches.len(),
                1,
                "Picard oracle lacked a unique sum for {left_index}+{right_index}"
            );
            table[left_index][right_index] = matches[0];
        }
    }
    table
}

fn oracle_scalar_mul(
    table: &[Vec<usize>],
    identity: usize,
    point: usize,
    mut scalar: u32,
) -> usize {
    let mut accumulator = identity;
    let mut addend = point;
    while scalar != 0 {
        if scalar & 1 != 0 {
            accumulator = table[accumulator][addend];
        }
        scalar >>= 1;
        if scalar != 0 {
            addend = table[addend][addend];
        }
    }
    accumulator
}

/// `F_(p^2) = F_p[t]/(t^2-EXTENSION_NON_SQUARE)`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Fp2 {
    a: u8,
    b: u8,
}

impl Fp2 {
    fn new(a: u8, b: u8) -> Self {
        Self { a: a % P, b: b % P }
    }

    fn base(value: u8) -> Self {
        Self::new(value, 0)
    }

    fn add(self, other: Self) -> Self {
        Self::new((self.a + other.a) % P, (self.b + other.b) % P)
    }

    fn neg(self) -> Self {
        Self::new(fp(-i16::from(self.a)), fp(-i16::from(self.b)))
    }

    fn sub(self, other: Self) -> Self {
        self.add(other.neg())
    }

    fn mul(self, other: Self) -> Self {
        Self::new(
            (self.a * other.a + EXTENSION_NON_SQUARE * self.b * other.b) % P,
            (self.a * other.b + self.b * other.a) % P,
        )
    }

    fn square(self) -> Self {
        self.mul(self)
    }

    fn inv(self) -> Self {
        let norm = fp(i16::from(self.a * self.a)
            - i16::from(EXTENSION_NON_SQUARE) * i16::from(self.b * self.b));
        assert_ne!(norm, 0);
        let inverse_norm = fp_inv(norm);
        Self::new(
            self.a * inverse_norm % P,
            fp(-i16::from(self.b)) * inverse_norm % P,
        )
    }
}

fn eval_fp2(polynomial: &Poly, value: Fp2) -> Fp2 {
    polynomial
        .0
        .iter()
        .rev()
        .fold(Fp2::default(), |accumulator, coefficient| {
            accumulator.mul(value).add(Fp2::base(*coefficient))
        })
}

fn roots_fp2(polynomial: &Poly) -> Vec<Fp2> {
    (0..P)
        .flat_map(|a| (0..P).map(move |b| Fp2::new(a, b)))
        .filter(|value| eval_fp2(polynomial, *value) == Fp2::default())
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OraclePoint {
    Infinity,
    Affine { x: Fp2, y: Fp2 },
}

fn target_add(left: OraclePoint, right: OraclePoint) -> OraclePoint {
    let (x1, y1, x2, y2) = match (left, right) {
        (OraclePoint::Infinity, point) | (point, OraclePoint::Infinity) => return point,
        (OraclePoint::Affine { x: x1, y: y1 }, OraclePoint::Affine { x: x2, y: y2 }) => {
            (x1, y1, x2, y2)
        }
    };
    let slope = if x1 == x2 {
        if y1.add(y2) == Fp2::default() {
            return OraclePoint::Infinity;
        }
        Fp2::base(3)
            .mul(x1.square())
            .add(Fp2::base(TARGET_A))
            .mul(Fp2::base(2).mul(y1).inv())
    } else {
        y2.sub(y1).mul(x2.sub(x1).inv())
    };
    let x3 = slope.square().sub(x1).sub(x2);
    let y3 = slope.mul(x1.sub(x3)).sub(y1);
    OraclePoint::Affine { x: x3, y: y3 }
}

fn target_points() -> Vec<OraclePoint> {
    let mut points = vec![OraclePoint::Infinity];
    for x in 0..P {
        for y in 0..P {
            let rhs = (x * x % P * x + TARGET_A * x + TARGET_B) % P;
            if y * y % P == rhs {
                points.push(OraclePoint::Affine {
                    x: Fp2::base(x),
                    y: Fp2::base(y),
                });
            }
        }
    }
    points
}

fn raw_source_point_counts() -> (usize, usize) {
    // The smooth projective even-degree model has the two independently
    // labelled rational infinity points over both fields.
    let n1 = 2
        + (0..P)
            .flat_map(|x| (0..P).map(move |y| (x, y)))
            .filter(|(x, y)| y * y % P == curve_polynomial().eval_base(*x))
            .count();

    let elements = (0..P)
        .flat_map(|a| (0..P).map(move |b| Fp2::new(a, b)))
        .collect::<Vec<_>>();
    let n2 = 2 + elements
        .iter()
        .flat_map(|x| elements.iter().map(move |y| (*x, *y)))
        .filter(|(x, y)| y.square() == eval_fp2(&curve_polynomial(), *x))
        .count();
    (n1, n2)
}

fn oracle_pushforward(divisor: &EffectiveDivisor) -> OraclePoint {
    let mut sum = OraclePoint::Infinity;
    for (place, multiplicity) in &divisor.finite {
        let degree = place.h.degree().unwrap();
        assert!(degree <= 2, "canonical support must split over F_(p^2)");
        let roots = roots_fp2(&place.h);
        assert_eq!(roots.len(), degree);
        for root in roots {
            let y = eval_fp2(&place.v, root);
            let source_rhs = eval_fp2(&curve_polynomial(), root);
            assert_eq!(y.square(), source_rhs);
            let point = OraclePoint::Affine {
                x: root.square().add(Fp2::base(COVER_SHIFT)),
                y,
            };
            for _ in 0..*multiplicity {
                sum = target_add(sum, point);
            }
        }
    }
    match sum {
        OraclePoint::Infinity => {}
        OraclePoint::Affine { x, y } => {
            assert_eq!(x.b, 0, "pushforward x-coordinate did not descend to F_p");
            assert_eq!(y.b, 0, "pushforward y-coordinate did not descend to F_p");
        }
    }
    sum
}

fn class_index(classes: &[ClassTriple], sought: &ClassTriple) -> usize {
    classes
        .iter()
        .position(|candidate| candidate == sought)
        .expect("canonical class must occur in independent enumeration")
}

fn effective_index(effective: &[EffectiveDivisor], sought: &EffectiveDivisor) -> usize {
    effective
        .iter()
        .position(|candidate| candidate == sought)
        .expect("effective divisor must have a canonical representative")
}

fn oracle_pullback(point: OraclePoint, classes: &[ClassTriple]) -> usize {
    let triple = match point {
        OraclePoint::Infinity => ClassTriple {
            u: Poly::one(),
            v: Poly::zero(),
            n_plus: 1,
        },
        OraclePoint::Affine { x, y } => {
            assert_eq!(x.b, 0);
            assert_eq!(y.b, 0);
            ClassTriple {
                u: Poly::new(vec![fp(i16::from(COVER_SHIFT) - i16::from(x.a)), 0, 1]),
                v: Poly::new(vec![y.a]),
                n_plus: 0,
            }
        }
    };
    class_index(classes, &triple)
}

fn fp_poly(polynomial: &Poly) -> FpPoly {
    FpPoly::from_coeffs(
        polynomial
            .0
            .iter()
            .map(|coefficient| BigUint::from(*coefficient))
            .collect(),
        BigUint::from(P),
    )
}

fn production_divisor(cover: &PrimeQuadraticCoverP, triple: &ClassTriple) -> BalancedDivisorP {
    BalancedDivisorP::try_from_polynomials(
        cover.source_curve(),
        fp_poly(&triple.u),
        fp_poly(&triple.v),
        u32::from(triple.n_plus),
    )
    .unwrap()
}

fn production_class(divisor: &BalancedDivisorP) -> ClassTriple {
    ClassTriple {
        u: Poly::new(
            divisor
                .u
                .coeffs
                .iter()
                .map(|coefficient| coefficient.to_u8().unwrap())
                .collect(),
        ),
        v: Poly::new(
            divisor
                .v
                .coeffs
                .iter()
                .map(|coefficient| coefficient.to_u8().unwrap())
                .collect(),
        ),
        n_plus: divisor.n_plus as u8,
    }
}

fn production_point(point: &PrimeEllipticPointP) -> OraclePoint {
    match point {
        PrimeEllipticPointP::Infinity => OraclePoint::Infinity,
        PrimeEllipticPointP::Affine { x, y } => OraclePoint::Affine {
            x: Fp2::base(x.to_u8().unwrap()),
            y: Fp2::base(y.to_u8().unwrap()),
        },
    }
}

fn to_production_point(point: OraclePoint) -> PrimeEllipticPointP {
    match point {
        OraclePoint::Infinity => PrimeEllipticPointP::Infinity,
        OraclePoint::Affine { x, y } => {
            assert_eq!(x.b, 0);
            assert_eq!(y.b, 0);
            PrimeEllipticPointP::Affine {
                x: BigUint::from(x.a),
                y: BigUint::from(y.a),
            }
        }
    }
}

fn short_to_montgomery_coordinates(
    point: OraclePoint,
    montgomery_a: u8,
    montgomery_b: u8,
) -> PrimeMontgomeryPointP {
    match point {
        OraclePoint::Infinity => PrimeMontgomeryPointP::Infinity,
        OraclePoint::Affine { x, y } => {
            assert_eq!(x.b, 0);
            assert_eq!(y.b, 0);
            let shift_a_over_3 = montgomery_a * fp_inv(3) % P;
            PrimeMontgomeryPointP::Affine {
                x: BigUint::from(fp(i16::from(montgomery_b * x.a) - i16::from(shift_a_over_3))),
                y: BigUint::from(montgomery_b * y.a % P),
            }
        }
    }
}

fn montgomery_on_curve(point: &PrimeMontgomeryPointP, a: u8, b: u8) -> bool {
    match point {
        PrimeMontgomeryPointP::Infinity => true,
        PrimeMontgomeryPointP::Affine { x, y } => {
            let x = x.to_u8().unwrap();
            let y = y.to_u8().unwrap();
            b * y % P * y % P == (x * x % P * x + a * x % P * x + x) % P
        }
    }
}

#[test]
fn exhaustive_picard_oracle_checks_arithmetic_and_both_transfer_compositions() {
    assert!(
        (1..P).all(|value| value * value % P != EXTENSION_NON_SQUARE),
        "configured quadratic-extension constant must be a nonsquare"
    );
    let c = COVER_SHIFT;
    let expected_curve = Poly::new(vec![
        (c * c % P * c + TARGET_A * c + TARGET_B) % P,
        0,
        (3 * c % P * c + TARGET_A) % P,
        0,
        3 * c % P,
        0,
        1,
    ]);
    assert_eq!(curve_polynomial(), expected_curve);
    assert_eq!(
        curve_polynomial().gcd_monic(&curve_polynomial().derivative()),
        Poly::one(),
        "source sextic must be squarefree"
    );
    let (n1, n2) = raw_source_point_counts();
    assert_eq!(
        (n1, n2),
        (EXPECTED_SOURCE_FQ_POINTS, EXPECTED_SOURCE_FQ2_POINTS)
    );
    let independent_jacobian_order = (n1 * n1 + n2) / 2 - usize::from(P);
    assert_eq!(independent_jacobian_order, EXPECTED_JACOBIAN_ORDER);

    let classes = enumerate_classes();
    assert_eq!(classes.len(), independent_jacobian_order);
    assert_eq!(
        classes.iter().collect::<BTreeSet<_>>().len(),
        EXPECTED_JACOBIAN_ORDER
    );

    let effective = classes.iter().map(effective_divisor).collect::<Vec<_>>();
    assert_eq!(
        effective.iter().collect::<BTreeSet<_>>().len(),
        EXPECTED_JACOBIAN_ORDER
    );
    let irreducible_quadratic_classes = effective
        .iter()
        .filter(|divisor| {
            divisor.finite.keys().any(|place| {
                place.h.degree() == Some(2) && (0..P).all(|x| place.h.eval_base(x) != 0)
            })
        })
        .count();
    assert!(irreducible_quadratic_classes > 0);

    let principal = principal_zero_divisors();
    // Functions a(x) with a nonsplit x-place are intentionally absent.
    if let Some(expected) = EXPECTED_PRINCIPAL_ZERO_DIVISORS {
        assert_eq!(principal.len(), expected);
    }

    // Known-false controls: preserving the same infinity support while
    // changing its valuations must not pass a support-only comparison.
    let constant_zeros = EffectiveDivisor {
        finite: BTreeMap::new(),
        infinity_plus: 3,
        infinity_minus: 3,
    };
    assert!(principal.contains(&constant_zeros));
    let mut wrong_infinity_multiplicity = constant_zeros.clone();
    wrong_infinity_multiplicity.infinity_plus -= 1;
    wrong_infinity_multiplicity.infinity_minus += 1;
    assert!(!principal.contains(&wrong_infinity_multiplicity));

    // Also seek a same-support finite mutation: move one valuation between
    // two equal-degree places while retaining both places in the support.
    let finite_multiplicity_control = principal.iter().any(|divisor| {
        let places = divisor.finite.keys().cloned().collect::<Vec<_>>();
        places.iter().any(|from| {
            divisor.finite[from] >= 2
                && places.iter().any(|to| {
                    if from == to || from.h.degree() != to.h.degree() {
                        return false;
                    }
                    let mut mutation = divisor.clone();
                    *mutation.finite.get_mut(from).unwrap() -= 1;
                    *mutation.finite.get_mut(to).unwrap() += 1;
                    !principal.contains(&mutation)
                })
        })
    });
    assert!(finite_multiplicity_control);
    let table = oracle_group_table(&effective, &principal);

    let identity = class_index(
        &classes,
        &ClassTriple {
            u: Poly::one(),
            v: Poly::zero(),
            n_plus: 1,
        },
    );
    for index in 0..classes.len() {
        assert_eq!(table[identity][index], index);
        assert_eq!(table[index][identity], index);
        let inverse = effective_index(&effective, &effective[index].involution());
        assert_eq!(table[index][inverse], identity);
    }
    for left in 0..classes.len() {
        for right in 0..classes.len() {
            assert_eq!(table[left][right], table[right][left]);
            for third in 0..classes.len() {
                assert_eq!(
                    table[table[left][right]][third], table[left][table[right][third]],
                    "oracle table was not associative for ({left},{right},{third})"
                );
            }
        }
    }

    // Confirm that the exhaustive pair comparison includes the exceptional
    // support patterns that motivated this independent oracle.
    let repeated_support = effective
        .iter()
        .filter(|divisor| {
            divisor
                .finite
                .values()
                .any(|multiplicity| *multiplicity > 1)
        })
        .count();
    let shared_support_pairs = effective
        .iter()
        .flat_map(|left| effective.iter().map(move |right| (left, right)))
        .filter(|(left, right)| {
            left.finite
                .keys()
                .any(|place| right.finite.contains_key(place))
        })
        .count();
    let conjugate_support_pairs = effective
        .iter()
        .flat_map(|left| effective.iter().map(move |right| (left, right)))
        .filter(|(left, right)| {
            left.finite.keys().any(|place| {
                right.finite.contains_key(&Place {
                    h: place.h.clone(),
                    v: place.v.neg().rem(&place.h),
                })
            })
        })
        .count();
    assert!(repeated_support > 0);
    assert!(shared_support_pairs > 0);
    assert!(conjugate_support_pairs > 0);
    let support_counters = (
        irreducible_quadratic_classes,
        repeated_support,
        shared_support_pairs,
        conjugate_support_pairs,
    );
    eprintln!(
        "independent split-sextic counters: principal={}, irreducible_quadratic_classes={}, repeated_support_classes={}, shared_support_pairs={}, conjugate_support_pairs={}",
        principal.len(),
        irreducible_quadratic_classes,
        repeated_support,
        shared_support_pairs,
        conjugate_support_pairs,
    );
    if let Some(expected) = EXPECTED_SUPPORT_COUNTERS {
        assert_eq!(support_counters, expected);
    }

    // Both identities are established using only the independent Picard
    // table, divisor maps, deck action, and target arithmetic.
    for point in target_points() {
        let pullback = oracle_pullback(point, &classes);
        assert_eq!(
            oracle_pushforward(&effective[pullback]),
            target_add(point, point),
            "independent pi_* pi^* = [2] failed"
        );
    }
    for (index, divisor) in effective.iter().enumerate() {
        let pushed = oracle_pushforward(divisor);
        let pulled = oracle_pullback(pushed, &classes);
        let deck = effective_index(&effective, &divisor.deck_transform());
        assert_eq!(
            pulled, table[index][deck],
            "independent pi^* pi_* = 1 + deck_* failed for class {index}"
        );
        assert_eq!(effective[deck].deck_transform(), *divisor);
    }

    for left in 0..effective.len() {
        for right in 0..effective.len() {
            assert_eq!(
                oracle_pushforward(&effective[table[left][right]]),
                target_add(
                    oracle_pushforward(&effective[left]),
                    oracle_pushforward(&effective[right])
                ),
                "independent pushforward was not additive for {left}+{right}"
            );
        }
    }

    assert_eq!(target_points().len(), EXPECTED_TARGET_ORDER);

    // Only now compare production code against the complete oracle above.
    if let Some((montgomery_a, montgomery_b)) = MONTGOMERY_MODEL {
        assert_montgomery_production_matches(
            montgomery_a,
            montgomery_b,
            &classes,
            &effective,
            &table,
            identity,
        );
    } else {
        assert_direct_production_matches(&classes, &effective, &table, identity);
    }
}

fn assert_direct_production_matches(
    classes: &[ClassTriple],
    effective: &[EffectiveDivisor],
    table: &[Vec<usize>],
    identity: usize,
) {
    let cover = PrimeQuadraticCoverP::try_new(
        BigUint::from(P),
        BigUint::from(TARGET_A),
        BigUint::from(TARGET_B),
        BigUint::from(COVER_SHIFT),
    )
    .unwrap();
    let production = classes
        .iter()
        .map(|triple| production_divisor(&cover, triple))
        .collect::<Vec<_>>();

    for left in 0..classes.len() {
        for right in 0..classes.len() {
            let sum = production[left]
                .checked_add(&production[right], cover.source_curve())
                .unwrap();
            assert_eq!(production_class(&sum), classes[table[left][right]]);
        }
    }

    for (index, divisor) in production.iter().enumerate() {
        let inverse = effective_index(effective, &effective[index].involution());
        assert_eq!(
            production_class(&divisor.checked_neg(cover.source_curve()).unwrap()),
            classes[inverse]
        );
        for scalar in 0..=EXPECTED_JACOBIAN_ORDER as u32 {
            let expected = oracle_scalar_mul(table, identity, index, scalar);
            assert_eq!(
                production_class(
                    &divisor
                        .checked_scalar_mul(&BigUint::from(scalar), cover.source_curve())
                        .unwrap()
                ),
                classes[expected],
                "production scalar multiplication mismatch for class {index}, scalar {scalar}"
            );
        }
    }

    for (index, divisor) in production.iter().enumerate() {
        let oracle_push = oracle_pushforward(&effective[index]);
        let production_push = cover.pushforward_arbitrary_balanced_class(divisor).unwrap();
        assert_eq!(production_point(&production_push), oracle_push);

        let oracle_deck = effective_index(effective, &effective[index].deck_transform());
        assert_eq!(
            production_class(&cover.deck_transform(divisor).unwrap()),
            classes[oracle_deck]
        );

        let oracle_pull = oracle_pullback(oracle_push, classes);
        assert_eq!(
            production_class(&cover.pullback_point_class(&production_push).unwrap()),
            classes[oracle_pull]
        );
        assert_eq!(oracle_pull, table[index][oracle_deck]);
    }

    for point in target_points() {
        let production_point_value = to_production_point(point);
        cover
            .validate_target_point(&production_point_value)
            .unwrap();
        let oracle_pull = oracle_pullback(point, classes);
        assert_eq!(
            production_class(&cover.pullback_point_class(&production_point_value).unwrap()),
            classes[oracle_pull]
        );
        assert_eq!(
            production_point(
                &cover
                    .checked_target_add(&production_point_value, &production_point_value)
                    .unwrap()
            ),
            target_add(point, point)
        );
    }

    let points = target_points();
    for left in &points {
        for right in &points {
            let production_left = to_production_point(*left);
            let production_right = to_production_point(*right);
            assert_eq!(
                production_point(
                    &cover
                        .checked_target_add(&production_left, &production_right)
                        .unwrap()
                ),
                target_add(*left, *right)
            );
        }
    }
}

fn assert_montgomery_production_matches(
    montgomery_a: u8,
    montgomery_b: u8,
    classes: &[ClassTriple],
    effective: &[EffectiveDivisor],
    table: &[Vec<usize>],
    identity: usize,
) {
    // These controls use only the independently enumerated short-model
    // points and the explicit projective-isomorphism formulas. They run
    // before any production cover or adapter is constructed.
    let points = target_points();
    let mapped = points
        .iter()
        .copied()
        .map(|point| short_to_montgomery_coordinates(point, montgomery_a, montgomery_b))
        .collect::<Vec<_>>();
    assert!(mapped
        .iter()
        .all(|point| montgomery_on_curve(point, montgomery_a, montgomery_b)));

    let altered_b = if montgomery_b == 1 { 2 } else { 1 };
    assert!(points.iter().copied().any(|point| {
        matches!(point, OraclePoint::Affine { .. })
            && !montgomery_on_curve(
                &short_to_montgomery_coordinates(point, montgomery_a, altered_b),
                montgomery_a,
                montgomery_b,
            )
    }));

    let altered_shift = (COVER_SHIFT + 1) % P;
    assert!(points.iter().copied().any(|point| {
        let OraclePoint::Affine { x, y } = point else {
            return false;
        };
        let wrong_pullback = ClassTriple {
            u: Poly::new(vec![fp(i16::from(altered_shift) - i16::from(x.a)), 0, 1]),
            v: Poly::new(vec![y.a]),
            n_plus: 0,
        };
        !valid_class(&wrong_pullback)
    }));

    let wrong_infinity = class_index(
        classes,
        &ClassTriple {
            u: Poly::one(),
            v: Poly::zero(),
            n_plus: 0,
        },
    );
    assert_ne!(wrong_infinity, identity);
    assert_eq!(
        oracle_pushforward(&effective[wrong_infinity]),
        OraclePoint::Infinity
    );
    assert_eq!(table[identity][wrong_infinity], wrong_infinity);

    // Production comparisons begin only after the independent group law,
    // maps, compositions, and altered-parameter controls have passed.
    let cover = PrimeMontgomeryQuadraticCoverP::try_new(
        BigUint::from(P),
        BigUint::from(montgomery_a),
        BigUint::from(montgomery_b),
        BigUint::from(COVER_SHIFT),
    )
    .unwrap();
    assert_eq!(cover.adapter().short_a(), &BigUint::from(TARGET_A));
    assert_eq!(cover.adapter().short_b(), &BigUint::from(TARGET_B));
    assert_eq!(cover.source_curve().f, fp_poly(&curve_polynomial()));

    let production = classes
        .iter()
        .map(|triple| production_divisor(cover.short_cover(), triple))
        .collect::<Vec<_>>();

    for left in 0..classes.len() {
        for right in 0..classes.len() {
            let sum = production[left]
                .checked_add(&production[right], cover.source_curve())
                .unwrap();
            assert_eq!(production_class(&sum), classes[table[left][right]]);
        }
    }

    for (index, divisor) in production.iter().enumerate() {
        let inverse = effective_index(effective, &effective[index].involution());
        assert_eq!(
            production_class(&divisor.checked_neg(cover.source_curve()).unwrap()),
            classes[inverse]
        );
        for scalar in 0..=EXPECTED_JACOBIAN_ORDER as u32 {
            let expected = oracle_scalar_mul(table, identity, index, scalar);
            assert_eq!(
                production_class(
                    &divisor
                        .checked_scalar_mul(&BigUint::from(scalar), cover.source_curve())
                        .unwrap()
                ),
                classes[expected],
                "production scalar multiplication mismatch for class {index}, scalar {scalar}"
            );
        }
    }

    for (index, divisor) in production.iter().enumerate() {
        let oracle_push = oracle_pushforward(&effective[index]);
        let production_push = cover.pushforward_arbitrary_balanced_class(divisor).unwrap();
        assert_eq!(
            production_push,
            short_to_montgomery_coordinates(oracle_push, montgomery_a, montgomery_b)
        );

        let oracle_deck = effective_index(effective, &effective[index].deck_transform());
        assert_eq!(
            production_class(&cover.deck_transform(divisor).unwrap()),
            classes[oracle_deck]
        );

        let oracle_pull = oracle_pullback(oracle_push, classes);
        assert_eq!(
            production_class(&cover.pullback_point_class(&production_push).unwrap()),
            classes[oracle_pull]
        );
        assert_eq!(oracle_pull, table[index][oracle_deck]);
    }

    for point in points.iter().copied() {
        let production_point = short_to_montgomery_coordinates(point, montgomery_a, montgomery_b);
        cover.validate_target_point(&production_point).unwrap();
        let oracle_pull = oracle_pullback(point, classes);
        assert_eq!(
            production_class(&cover.pullback_point_class(&production_point).unwrap()),
            classes[oracle_pull]
        );
        assert_eq!(
            cover
                .checked_target_add(&production_point, &production_point)
                .unwrap(),
            short_to_montgomery_coordinates(target_add(point, point), montgomery_a, montgomery_b,)
        );
    }

    for (left_index, left) in points.iter().copied().enumerate() {
        for (right_index, right) in points.iter().copied().enumerate() {
            assert_eq!(
                cover
                    .checked_target_add(&mapped[left_index], &mapped[right_index])
                    .unwrap(),
                short_to_montgomery_coordinates(
                    target_add(left, right),
                    montgomery_a,
                    montgomery_b,
                )
            );
        }
    }
}
