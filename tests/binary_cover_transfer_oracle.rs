//! Independent small-field oracle for the binary cubic cover norm.
//!
//! The oracle below deliberately does not call the production closed-point
//! factorization, residue-field, Frobenius-trace, or target-group routines.
//! It implements those operations again with byte-sized polynomial bases for
//! the exact `GF(8)` control.  Production code supplies the checked Mumford
//! representatives and (for the homomorphism cross-check) the source
//! Jacobian addition.  Thus this is independent evidence for the divisor-map
//! value, not an independent implementation of the source group law. Solver
//! behavior and performance are outside this test's scope.

use crypto_lib::binary_ecc::{
    BinaryPoint, F2mElement, F2mPoly, IrreduciblePoly, MumfordDivisor, OrdinaryBinaryCover,
};
use num_bigint::BigUint;
use std::collections::{HashMap, HashSet};

/// `GF(8) = GF(2)[t]/(t^3+t+1)`, encoded as three coefficient bits.
fn f8_mul(mut left: u8, mut right: u8) -> u8 {
    let mut product = 0u8;
    for _ in 0..3 {
        if right & 1 != 0 {
            product ^= left;
        }
        right >>= 1;
        let carry = left & 0b100 != 0;
        left = (left << 1) & 0b111;
        if carry {
            // t^3 = t+1 modulo t^3+t+1.
            left ^= 0b011;
        }
    }
    product
}

fn f8_pow(mut base: u8, mut exponent: u16) -> u8 {
    let mut result = 1u8;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = f8_mul(result, base);
        }
        exponent >>= 1;
        if exponent != 0 {
            base = f8_mul(base, base);
        }
    }
    result
}

fn trim(mut polynomial: Vec<u8>) -> Vec<u8> {
    while polynomial.last() == Some(&0) {
        polynomial.pop();
    }
    polynomial
}

fn polynomial_eval(polynomial: &[u8], value: u8) -> u8 {
    polynomial.iter().rev().fold(0, |accumulator, coefficient| {
        f8_mul(accumulator, value) ^ coefficient
    })
}

/// Exact division by a monic polynomial over `GF(8)`.
fn polynomial_divide_monic(numerator: &[u8], denominator: &[u8]) -> (Vec<u8>, Vec<u8>) {
    assert_eq!(denominator.last(), Some(&1));
    let mut remainder = trim(numerator.to_vec());
    let mut quotient = if remainder.len() >= denominator.len() {
        vec![0; remainder.len() - denominator.len() + 1]
    } else {
        Vec::new()
    };
    while remainder.len() >= denominator.len() {
        let shift = remainder.len() - denominator.len();
        let lead = *remainder.last().expect("nonzero remainder");
        quotient[shift] ^= lead;
        for (index, coefficient) in denominator.iter().enumerate() {
            remainder[shift + index] ^= f8_mul(lead, *coefficient);
        }
        remainder = trim(remainder);
    }
    (trim(quotient), remainder)
}

/// Factor a monic polynomial of degree at most three. After every base-field
/// root (with multiplicity) is removed, the degree-two or degree-three
/// remainder has no `GF(8)` root and is therefore irreducible.
fn factor_small_monic(polynomial: &[u8]) -> Vec<Vec<u8>> {
    assert!(polynomial.len() <= 4);
    assert_eq!(polynomial.last(), Some(&1));
    let mut remaining = polynomial.to_vec();
    let mut factors = Vec::new();
    for root in 0..8u8 {
        while remaining.len() > 1 && polynomial_eval(&remaining, root) == 0 {
            let factor = vec![root, 1];
            let (quotient, remainder) = polynomial_divide_monic(&remaining, &factor);
            assert!(remainder.is_empty());
            factors.push(factor);
            remaining = quotient;
        }
    }
    if remaining.len() > 1 {
        assert!(remaining.len() == 3 || remaining.len() == 4);
        assert!((0..8u8).all(|root| polynomial_eval(&remaining, root) != 0));
        factors.push(remaining);
    }
    factors
}

/// A separately implemented residue field `GF(8)[z]/factor(z)`.
#[derive(Clone, Debug)]
struct ResidueField {
    factor: Vec<u8>,
    degree: usize,
}

type ResidueElement = Vec<u8>;

impl ResidueField {
    fn new(factor: Vec<u8>) -> Self {
        assert_eq!(factor.last(), Some(&1));
        let degree = factor.len() - 1;
        assert!((1..=3).contains(&degree));
        Self { factor, degree }
    }

    fn normalize(&self, mut value: Vec<u8>) -> ResidueElement {
        value.resize(self.degree, 0);
        value.truncate(self.degree);
        value
    }

    fn constant(&self, value: u8) -> ResidueElement {
        let mut result = vec![0; self.degree];
        result[0] = value;
        result
    }

    fn alpha(&self) -> ResidueElement {
        let mut polynomial = vec![0, 1];
        if self.degree == 1 {
            // z + factor[0] = 0, and subtraction equals addition.
            polynomial = vec![self.factor[0]];
        }
        self.normalize(polynomial)
    }

    fn add(&self, left: &[u8], right: &[u8]) -> ResidueElement {
        (0..self.degree).map(|i| left[i] ^ right[i]).collect()
    }

    fn multiply(&self, left: &[u8], right: &[u8]) -> ResidueElement {
        let mut product = vec![0; 2 * self.degree - 1];
        for (i, left_coefficient) in left.iter().enumerate() {
            for (j, right_coefficient) in right.iter().enumerate() {
                product[i + j] ^= f8_mul(*left_coefficient, *right_coefficient);
            }
        }
        for degree in (self.degree..product.len()).rev() {
            let lead = product[degree];
            if lead == 0 {
                continue;
            }
            for (index, coefficient) in self.factor[..self.degree].iter().enumerate() {
                product[degree - self.degree + index] ^= f8_mul(lead, *coefficient);
            }
        }
        product.truncate(self.degree);
        product
    }

    fn square(&self, value: &[u8]) -> ResidueElement {
        self.multiply(value, value)
    }

    fn power(&self, mut base: ResidueElement, mut exponent: u16) -> ResidueElement {
        let mut result = self.constant(1);
        while exponent != 0 {
            if exponent & 1 != 0 {
                result = self.multiply(&result, &base);
            }
            exponent >>= 1;
            if exponent != 0 {
                base = self.square(&base);
            }
        }
        result
    }

    fn inverse(&self, value: &[u8]) -> ResidueElement {
        assert!(value.iter().any(|coefficient| *coefficient != 0));
        let order = 8u16.pow(self.degree as u32);
        self.power(value.to_vec(), order - 2)
    }

    fn divide(&self, numerator: &[u8], denominator: &[u8]) -> ResidueElement {
        self.multiply(numerator, &self.inverse(denominator))
    }

    fn evaluate(&self, polynomial: &[u8], value: &[u8]) -> ResidueElement {
        polynomial
            .iter()
            .rev()
            .fold(self.constant(0), |accumulator, coefficient| {
                let product = self.multiply(&accumulator, value);
                self.add(&product, &self.constant(*coefficient))
            })
    }

    fn is_zero(&self, value: &[u8]) -> bool {
        value.iter().all(|coefficient| *coefficient == 0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ExtensionPoint {
    Infinity,
    Affine {
        x: ResidueElement,
        y: ResidueElement,
    },
}

/// Independent characteristic-two group law for
/// `y^2+x*y=x^3+a*x^2+b`.
fn extension_add(
    field: &ResidueField,
    left: &ExtensionPoint,
    right: &ExtensionPoint,
    a: u8,
) -> ExtensionPoint {
    match (left, right) {
        (ExtensionPoint::Infinity, point) | (point, ExtensionPoint::Infinity) => point.clone(),
        (ExtensionPoint::Affine { x: x1, y: y1 }, ExtensionPoint::Affine { x: x2, y: y2 }) => {
            if x1 == x2 {
                if y1 != y2 || field.is_zero(x1) {
                    return ExtensionPoint::Infinity;
                }
                let lambda = field.add(x1, &field.divide(y1, x1));
                let x3 = field.add(
                    &field.add(&field.square(&lambda), &lambda),
                    &field.constant(a),
                );
                let y3 = field.add(
                    &field.square(x1),
                    &field.multiply(&field.add(&lambda, &field.constant(1)), &x3),
                );
                ExtensionPoint::Affine { x: x3, y: y3 }
            } else {
                let lambda = field.divide(&field.add(y1, y2), &field.add(x1, x2));
                let x3 = field.add(
                    &field.add(
                        &field.add(&field.square(&lambda), &lambda),
                        &field.add(x1, x2),
                    ),
                    &field.constant(a),
                );
                let y3 = field.add(
                    &field.add(&field.multiply(&lambda, &field.add(x1, &x3)), &x3),
                    y1,
                );
                ExtensionPoint::Affine { x: x3, y: y3 }
            }
        }
    }
}

fn extension_frobenius(field: &ResidueField, point: &ExtensionPoint) -> ExtensionPoint {
    match point {
        ExtensionPoint::Infinity => ExtensionPoint::Infinity,
        ExtensionPoint::Affine { x, y } => ExtensionPoint::Affine {
            x: field.power(x.clone(), 8),
            y: field.power(y.clone(), 8),
        },
    }
}

fn extension_is_on_curve(field: &ResidueField, point: &ExtensionPoint, a: u8, b: u8) -> bool {
    match point {
        ExtensionPoint::Infinity => true,
        ExtensionPoint::Affine { x, y } => {
            let left = field.add(&field.square(y), &field.multiply(x, y));
            let x_squared = field.square(x);
            let right = field.add(
                &field.add(
                    &field.multiply(&x_squared, x),
                    &field.multiply(&field.constant(a), &x_squared),
                ),
                &field.constant(b),
            );
            left == right
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum BasePoint {
    Infinity,
    Affine { x: u8, y: u8 },
}

fn base_add(left: BasePoint, right: BasePoint, a: u8) -> BasePoint {
    match (left, right) {
        (BasePoint::Infinity, point) | (point, BasePoint::Infinity) => point,
        (BasePoint::Affine { x: x1, y: y1 }, BasePoint::Affine { x: x2, y: y2 }) => {
            if x1 == x2 {
                if y1 != y2 || x1 == 0 {
                    return BasePoint::Infinity;
                }
                let lambda = x1 ^ f8_mul(y1, f8_pow(x1, 6));
                let x3 = f8_mul(lambda, lambda) ^ lambda ^ a;
                let y3 = f8_mul(x1, x1) ^ f8_mul(lambda ^ 1, x3);
                BasePoint::Affine { x: x3, y: y3 }
            } else {
                let lambda = f8_mul(y1 ^ y2, f8_pow(x1 ^ x2, 6));
                let x3 = f8_mul(lambda, lambda) ^ lambda ^ x1 ^ x2 ^ a;
                let y3 = f8_mul(lambda, x1 ^ x3) ^ x3 ^ y1;
                BasePoint::Affine { x: x3, y: y3 }
            }
        }
    }
}

fn base_is_on_curve(point: BasePoint, a: u8, b: u8) -> bool {
    match point {
        BasePoint::Infinity => true,
        BasePoint::Affine { x, y } => {
            f8_mul(y, y) ^ f8_mul(x, y) == f8_mul(f8_mul(x, x), x) ^ f8_mul(a, f8_mul(x, x)) ^ b
        }
    }
}

fn trace_closed_point(field: &ResidueField, point: ExtensionPoint, a: u8) -> BasePoint {
    let mut trace = ExtensionPoint::Infinity;
    let mut conjugate = point;
    for _ in 0..field.degree {
        trace = extension_add(field, &trace, &conjugate, a);
        conjugate = extension_frobenius(field, &conjugate);
    }
    match trace {
        ExtensionPoint::Infinity => BasePoint::Infinity,
        ExtensionPoint::Affine { x, y } => {
            assert!(x[1..].iter().all(|coefficient| *coefficient == 0));
            assert!(y[1..].iter().all(|coefficient| *coefficient == 0));
            BasePoint::Affine { x: x[0], y: y[0] }
        }
    }
}

fn coefficients(polynomial: &F2mPoly) -> Vec<u8> {
    polynomial
        .coeffs
        .iter()
        .map(|coefficient| {
            coefficient
                .to_biguint()
                .to_u32_digits()
                .first()
                .copied()
                .unwrap_or(0) as u8
        })
        .collect()
}

/// Independent factor/evaluate/trace computation of the divisor norm.
fn oracle_pushforward(divisor: &MumfordDivisor, a: u8, b: u8, d: u8) -> BasePoint {
    let u = coefficients(&divisor.u);
    let v = coefficients(&divisor.v);
    let mut image = BasePoint::Infinity;
    for factor in factor_small_monic(&u) {
        let field = ResidueField::new(factor);
        let alpha = field.alpha();
        let source_y = field.evaluate(&v, &alpha);
        let target_x = field.power(alpha.clone(), 3);
        let target_y = field.add(&field.multiply(&alpha, &source_y), &field.constant(d));
        let target_point = ExtensionPoint::Affine {
            x: target_x,
            y: target_y,
        };
        assert!(extension_is_on_curve(&field, &target_point, a, b));
        let component = trace_closed_point(&field, target_point, a);
        assert!(base_is_on_curve(component, a, b));
        image = base_add(image, component, a);
    }
    assert!(base_is_on_curve(image, a, b));
    image
}

fn production_point(point: &BinaryPoint) -> BasePoint {
    match point {
        BinaryPoint::Infinity => BasePoint::Infinity,
        BinaryPoint::Affine { x, y } => {
            let word = |value: &F2mElement| {
                value
                    .to_biguint()
                    .to_u32_digits()
                    .first()
                    .copied()
                    .unwrap_or(0) as u8
            };
            BasePoint::Affine {
                x: word(x),
                y: word(y),
            }
        }
    }
}

fn element(value: usize) -> F2mElement {
    F2mElement::from_biguint(&BigUint::from(value), 3)
}

fn coefficients_from_code(mut code: usize, length: usize) -> Vec<F2mElement> {
    (0..length)
        .map(|_| {
            let coefficient = element(code % 8);
            code /= 8;
            coefficient
        })
        .collect()
}

fn all_reduced_divisors(cover: &OrdinaryBinaryCover) -> Vec<MumfordDivisor> {
    let curve = cover.source_curve();
    let mut divisors = vec![MumfordDivisor::identity(3)];
    for u_degree in 1..=3usize {
        let coefficient_vectors = 8usize.pow(u_degree as u32);
        for u_code in 0..coefficient_vectors {
            let mut u_coefficients = coefficients_from_code(u_code, u_degree);
            u_coefficients.push(F2mElement::one(3));
            let u = F2mPoly::from_coeffs(u_coefficients, 3);
            for v_code in 0..coefficient_vectors {
                let v = F2mPoly::from_coeffs(coefficients_from_code(v_code, u_degree), 3);
                let equation = v
                    .mul(&v, &curve.irr)
                    .add(&curve.h.mul(&v, &curve.irr))
                    .add(&curve.f);
                if equation.rem(&u, &curve.irr).is_zero() {
                    divisors.push(MumfordDivisor { u: u.clone(), v });
                }
            }
        }
    }
    divisors
}

fn divisor_key(divisor: &MumfordDivisor) -> (Vec<u8>, Vec<u8>) {
    (coefficients(&divisor.u), coefficients(&divisor.v))
}

#[test]
fn independent_f8_oracle_matches_every_class_and_all_ordered_sums() {
    let modulus = IrreduciblePoly {
        degree: 3,
        low_terms: vec![0, 1],
    };
    let cover = OrdinaryBinaryCover::new(3, modulus, element(1), element(3)).unwrap();
    let divisors = all_reduced_divisors(&cover);
    assert_eq!(divisors.len(), 486);

    let mut images = HashMap::new();
    let mut factor_degree_profiles = HashSet::new();
    let mut repeated_support = 0usize;
    let mut nonsplit_support = 0usize;
    for divisor in &divisors {
        let factors = factor_small_monic(&coefficients(&divisor.u));
        let degrees: Vec<_> = factors.iter().map(|factor| factor.len() - 1).collect();
        if factors.windows(2).any(|pair| pair[0] == pair[1]) {
            repeated_support += 1;
        }
        if degrees.iter().any(|degree| *degree > 1) {
            nonsplit_support += 1;
        }
        factor_degree_profiles.insert(degrees);

        let oracle = oracle_pushforward(divisor, 1, 3, 7);
        let production =
            production_point(&cover.pushforward_arbitrary_jacobian_class(divisor).unwrap());
        assert_eq!(oracle, production, "divisor {:?}", divisor_key(divisor));
        assert!(images.insert(divisor_key(divisor), oracle).is_none());
    }
    assert_eq!(images.len(), 486);
    assert!(repeated_support > 0);
    assert!(nonsplit_support > 0);
    assert!(factor_degree_profiles.contains(&vec![3]));
    assert!(factor_degree_profiles.contains(&vec![1, 2]));
    assert!(factor_degree_profiles.contains(&vec![1, 1, 1]));

    // The exceptional ramified support `(u,v)=(x,0)` is present and mapped
    // by the separate oracle, rather than skipped as a generic-only case.
    assert!(images.contains_key(&(vec![0, 1], Vec::new())));

    // This homomorphism loop uses the production source-Jacobian addition,
    // but both input images and the target addition are computed by the
    // separate oracle above.
    for (left_index, left) in divisors.iter().enumerate() {
        for (right_index, right) in divisors.iter().enumerate() {
            let sum = left.add_fast(right, cover.source_curve());
            let actual = images[&divisor_key(&sum)];
            let expected = base_add(images[&divisor_key(left)], images[&divisor_key(right)], 1);
            assert_eq!(
                actual, expected,
                "oracle homomorphism mismatch for ({left_index},{right_index})"
            );
        }
    }
}

#[test]
fn independent_oracle_also_exercises_the_general_non_table_path() {
    let modulus = IrreduciblePoly {
        degree: 3,
        low_terms: vec![0, 1],
    };
    // Only (a,b)=(1,3) selects the retained 486-class lookup table. This
    // second model therefore reaches the catalog-general residue-field path.
    let cover = OrdinaryBinaryCover::new(3, modulus, element(0), element(1)).unwrap();
    assert_eq!(cover.square_root_b(), &element(1));
    let divisors = all_reduced_divisors(&cover);
    assert!(!divisors.is_empty());

    let mut saw_repeated = false;
    let mut saw_nonsplit = false;
    for divisor in &divisors {
        let factors = factor_small_monic(&coefficients(&divisor.u));
        saw_repeated |= factors.windows(2).any(|pair| pair[0] == pair[1]);
        saw_nonsplit |= factors.iter().any(|factor| factor.len() > 2);
        let oracle = oracle_pushforward(divisor, 0, 1, 1);
        let production =
            production_point(&cover.pushforward_arbitrary_jacobian_class(divisor).unwrap());
        assert_eq!(oracle, production, "divisor {:?}", divisor_key(divisor));
    }
    assert!(saw_repeated);
    assert!(saw_nonsplit);
}
