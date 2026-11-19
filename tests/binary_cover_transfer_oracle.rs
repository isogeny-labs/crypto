//! Independent source-law and transfer oracle for the fixed binary cover.
//!
//! The fixed fixture is
//!
//! ```text
//! GF(8) = GF(2)[z]/(z^3+z+1),
//! E : y^2+x*y = x^3+x^2+3,
//! H : v^2+u^2*v = u^7+u^4+7*u,
//! phi(u,v) = (u^3,u*v+7).
//! ```
//!
//! A byte-sized implementation enumerates all 486 reduced source classes and
//! constructs their complete group table from principal divisors of
//! `A(u)+B(u)v` in `L(9*infinity)`. It also implements the target group,
//! pullback, and pushforward without production arithmetic. Production
//! objects are constructed only after that independent table and all transfer
//! identities have been checked. This is exhaustive evidence for this exact
//! fixture, not a catalog-wide oracle, solver, or performance claim.

use crypto_lib::binary_ecc::{
    BinaryPoint, F2mElement, F2mPoly, IrreduciblePoly, MumfordDivisor, OrdinaryBinaryCover,
};
use num_bigint::BigUint;

mod independent {
    use std::collections::{BTreeSet, HashMap, HashSet};
    use std::hash::{BuildHasherDefault, Hasher};

    const Q: usize = 8;
    const TARGET_A: u8 = 1;
    const TARGET_B: u8 = 3;
    const COVER_D: u8 = 7;
    const GENUS: usize = 3;
    const MAX_POLE: usize = 9;
    const SERIES_PRECISION: usize = MAX_POLE + 2;
    const MAX_POLY_DEGREE: usize = 20;

    #[inline]
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
                left ^= 0b011;
            }
        }
        product
    }

    fn f8_pow(mut base: u8, mut exponent: u32) -> u8 {
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

    #[inline]
    fn f8_inverse(value: u8) -> u8 {
        assert_ne!(value, 0);
        f8_pow(value, 6)
    }

    /// A polynomial over GF(8), packed in three bits per coefficient.
    #[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub(super) struct Poly(u64);

    impl Poly {
        const ZERO: Self = Self(0);
        const ONE: Self = Self(1);
        const X: Self = Self(1 << 3);

        pub(super) fn from_coefficients(coefficients: &[u8]) -> Self {
            assert!(coefficients.len() <= MAX_POLY_DEGREE + 1);
            let mut packed = 0u64;
            for (index, coefficient) in coefficients.iter().copied().enumerate() {
                assert!(coefficient < 8);
                packed |= u64::from(coefficient) << (3 * index);
            }
            Self(packed)
        }

        fn from_code(mut code: usize, coefficient_count: usize) -> Self {
            let mut coefficients = vec![0u8; coefficient_count];
            for coefficient in &mut coefficients {
                *coefficient = (code % Q) as u8;
                code /= Q;
            }
            assert_eq!(code, 0);
            Self::from_coefficients(&coefficients)
        }

        pub(super) fn coefficients(self) -> Vec<u8> {
            let Some(degree) = self.degree() else {
                return Vec::new();
            };
            (0..=degree).map(|index| self.coefficient(index)).collect()
        }

        #[inline]
        fn coefficient(self, index: usize) -> u8 {
            ((self.0 >> (3 * index)) & 0b111) as u8
        }

        #[inline]
        fn degree(self) -> Option<usize> {
            (self.0 != 0).then(|| (63 - self.0.leading_zeros() as usize) / 3)
        }

        #[inline]
        fn add(self, other: Self) -> Self {
            Self(self.0 ^ other.0)
        }

        fn scalar_mul(self, scalar: u8) -> Self {
            if scalar == 0 || self == Self::ZERO {
                return Self::ZERO;
            }
            let degree = self.degree().unwrap();
            let mut result = 0u64;
            for index in 0..=degree {
                result |= u64::from(f8_mul(self.coefficient(index), scalar)) << (3 * index);
            }
            Self(result)
        }

        fn shift(self, amount: usize) -> Self {
            if self == Self::ZERO {
                return self;
            }
            assert!(self.degree().unwrap() + amount <= MAX_POLY_DEGREE);
            Self(self.0 << (3 * amount))
        }

        fn mul(self, other: Self) -> Self {
            let (Some(left_degree), Some(right_degree)) = (self.degree(), other.degree()) else {
                return Self::ZERO;
            };
            assert!(left_degree + right_degree <= MAX_POLY_DEGREE);
            let mut result = 0u64;
            for left_index in 0..=left_degree {
                let left = self.coefficient(left_index);
                if left == 0 {
                    continue;
                }
                for right_index in 0..=right_degree {
                    let right = other.coefficient(right_index);
                    if right != 0 {
                        result ^=
                            u64::from(f8_mul(left, right)) << (3 * (left_index + right_index));
                    }
                }
            }
            Self(result)
        }

        #[inline]
        fn square(self) -> Self {
            self.mul(self)
        }

        fn divrem_monic(self, denominator: Self) -> (Self, Self) {
            let denominator_degree = denominator.degree().expect("nonzero denominator");
            assert_eq!(denominator.coefficient(denominator_degree), 1);
            let mut quotient = Self::ZERO;
            let mut remainder = self;
            while let Some(remainder_degree) = remainder.degree() {
                if remainder_degree < denominator_degree {
                    break;
                }
                let shift = remainder_degree - denominator_degree;
                let leading = remainder.coefficient(remainder_degree);
                quotient = quotient.add(Self::from_coefficients(&[leading]).shift(shift));
                remainder = remainder.add(denominator.scalar_mul(leading).shift(shift));
            }
            (quotient, remainder)
        }

        #[inline]
        fn rem(self, denominator: Self) -> Self {
            self.divrem_monic(denominator).1
        }

        fn monic(self) -> Self {
            let degree = self
                .degree()
                .expect("zero polynomial has no monic associate");
            self.scalar_mul(f8_inverse(self.coefficient(degree)))
        }

        fn evaluate_f8(self, value: u8) -> u8 {
            let Some(degree) = self.degree() else {
                return 0;
            };
            (0..=degree).rev().fold(0, |accumulator, index| {
                f8_mul(accumulator, value) ^ self.coefficient(index)
            })
        }
    }

    fn source_h() -> Poly {
        Poly::from_coefficients(&[0, 0, 1])
    }

    fn source_f() -> Poly {
        Poly::from_coefficients(&[0, COVER_D, 0, 0, TARGET_A, 0, 0, 1])
    }

    fn parameterized_source_f(a: u8, d: u8) -> Poly {
        Poly::from_coefficients(&[0, d, 0, 0, a, 0, 0, 1])
    }

    #[inline]
    fn source_equation(v: Poly) -> Poly {
        v.square().add(source_h().mul(v)).add(source_f())
    }

    fn parameterized_source_equation(v: Poly, a: u8, d: u8) -> Poly {
        v.square()
            .add(source_h().mul(v))
            .add(parameterized_source_f(a, d))
    }

    fn monic_polynomials(degree: usize) -> impl Iterator<Item = Poly> {
        (0..Q.pow(degree as u32))
            .map(move |code| Poly::from_code(code, degree).add(Poly::ONE.shift(degree)))
    }

    fn irreducibles_through_degree_three() -> Vec<Poly> {
        let mut irreducibles = Vec::new();
        for degree in 1..=3 {
            for polynomial in monic_polynomials(degree) {
                if degree == 1 || (0..8u8).all(|root| polynomial.evaluate_f8(root) != 0) {
                    irreducibles.push(polynomial);
                }
            }
        }
        irreducibles
    }

    #[derive(Clone, Copy, Debug)]
    struct ResidueField {
        modulus: Poly,
        degree: usize,
    }

    impl ResidueField {
        fn new(modulus: Poly) -> Self {
            let degree = modulus.degree().expect("field modulus");
            assert!((1..=3).contains(&degree));
            assert_eq!(modulus.coefficient(degree), 1);
            Self { modulus, degree }
        }

        #[inline]
        fn add(self, left: Poly, right: Poly) -> Poly {
            left.add(right)
        }

        #[inline]
        fn multiply(self, left: Poly, right: Poly) -> Poly {
            left.mul(right).rem(self.modulus)
        }

        #[inline]
        fn square(self, value: Poly) -> Poly {
            self.multiply(value, value)
        }

        fn power(self, mut base: Poly, mut exponent: u32) -> Poly {
            let mut result = Poly::ONE;
            while exponent != 0 {
                if exponent & 1 != 0 {
                    result = self.multiply(result, base);
                }
                exponent >>= 1;
                if exponent != 0 {
                    base = self.square(base);
                }
            }
            result
        }

        fn inverse(self, value: Poly) -> Poly {
            assert_ne!(value, Poly::ZERO);
            self.power(value, 8u32.pow(self.degree as u32) - 2)
        }

        #[inline]
        fn divide(self, numerator: Poly, denominator: Poly) -> Poly {
            self.multiply(numerator, self.inverse(denominator))
        }
    }

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub(super) enum BasePoint {
        Infinity,
        Affine { x: u8, y: u8 },
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum ExtensionPoint {
        Infinity,
        Affine { x: Poly, y: Poly },
    }

    fn extension_add(
        field: ResidueField,
        left: ExtensionPoint,
        right: ExtensionPoint,
        a: u8,
    ) -> ExtensionPoint {
        match (left, right) {
            (ExtensionPoint::Infinity, point) | (point, ExtensionPoint::Infinity) => point,
            (ExtensionPoint::Affine { x: x1, y: y1 }, ExtensionPoint::Affine { x: x2, y: y2 }) => {
                if x1 == x2 {
                    if y1 != y2 || x1 == Poly::ZERO {
                        return ExtensionPoint::Infinity;
                    }
                    let lambda = field.add(x1, field.divide(y1, x1));
                    let x3 = field.add(
                        field.add(field.square(lambda), lambda),
                        Poly::from_coefficients(&[a]),
                    );
                    let y3 = field.add(
                        field.square(x1),
                        field.multiply(field.add(lambda, Poly::ONE), x3),
                    );
                    ExtensionPoint::Affine { x: x3, y: y3 }
                } else {
                    let lambda = field.divide(field.add(y1, y2), field.add(x1, x2));
                    let x3 = field.add(
                        field.add(field.add(field.square(lambda), lambda), field.add(x1, x2)),
                        Poly::from_coefficients(&[a]),
                    );
                    let y3 =
                        field.add(field.add(field.multiply(lambda, field.add(x1, x3)), x3), y1);
                    ExtensionPoint::Affine { x: x3, y: y3 }
                }
            }
        }
    }

    fn extension_is_on_curve(field: ResidueField, point: ExtensionPoint, a: u8, b: u8) -> bool {
        match point {
            ExtensionPoint::Infinity => true,
            ExtensionPoint::Affine { x, y } => {
                let left = field.add(field.square(y), field.multiply(x, y));
                let x_squared = field.square(x);
                let right = field.add(
                    field.add(
                        field.multiply(x_squared, x),
                        field.multiply(Poly::from_coefficients(&[a]), x_squared),
                    ),
                    Poly::from_coefficients(&[b]),
                );
                left == right
            }
        }
    }

    #[inline]
    pub(super) fn base_add(left: BasePoint, right: BasePoint, a: u8) -> BasePoint {
        match (left, right) {
            (BasePoint::Infinity, point) | (point, BasePoint::Infinity) => point,
            (BasePoint::Affine { x: x1, y: y1 }, BasePoint::Affine { x: x2, y: y2 }) => {
                if x1 == x2 {
                    if y1 != y2 || x1 == 0 {
                        return BasePoint::Infinity;
                    }
                    let lambda = x1 ^ f8_mul(y1, f8_inverse(x1));
                    let x3 = f8_mul(lambda, lambda) ^ lambda ^ a;
                    let y3 = f8_mul(x1, x1) ^ f8_mul(lambda ^ 1, x3);
                    BasePoint::Affine { x: x3, y: y3 }
                } else {
                    let lambda = f8_mul(y1 ^ y2, f8_inverse(x1 ^ x2));
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

    fn trace_closed_point(field: ResidueField, point: ExtensionPoint, a: u8) -> BasePoint {
        let mut trace = ExtensionPoint::Infinity;
        let mut conjugate = point;
        for _ in 0..field.degree {
            trace = extension_add(field, trace, conjugate, a);
            conjugate = match conjugate {
                ExtensionPoint::Infinity => ExtensionPoint::Infinity,
                ExtensionPoint::Affine { x, y } => ExtensionPoint::Affine {
                    x: field.power(x, 8),
                    y: field.power(y, 8),
                },
            };
        }
        match trace {
            ExtensionPoint::Infinity => BasePoint::Infinity,
            ExtensionPoint::Affine { x, y } => {
                assert!(x.degree().is_none_or(|degree| degree == 0));
                assert!(y.degree().is_none_or(|degree| degree == 0));
                BasePoint::Affine {
                    x: x.coefficient(0),
                    y: y.coefficient(0),
                }
            }
        }
    }

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub(super) struct DivisorKey {
        pub(super) u: Poly,
        pub(super) v: Poly,
    }

    impl DivisorKey {
        fn degree(self) -> usize {
            self.u.degree().expect("nonzero u")
        }

        pub(super) fn involution(self) -> Self {
            Self {
                u: self.u,
                v: self.v.add(source_h()).rem(self.u),
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    struct FactorData {
        polynomial: Poly,
        degree: usize,
        roots: [Poly; 2],
        root_count: usize,
        place_indices: [u8; 2],
        powers: [Poly; MAX_POLE + 1],
        lifts: [[Poly; MAX_POLE + 1]; 2],
    }

    fn factor_data() -> (Vec<FactorData>, Vec<u8>) {
        let irreducibles = irreducibles_through_degree_three();
        assert_eq!(irreducibles.len(), 204);
        let mut result = Vec::with_capacity(irreducibles.len());
        let mut place_degrees = Vec::new();
        let mut split = [0usize; 4];
        let mut ramified = [0usize; 4];
        let mut inert = [0usize; 4];

        for polynomial in irreducibles {
            let degree = polynomial.degree().unwrap();
            let field = ResidueField::new(polynomial);
            let mut roots = [Poly::ZERO; 2];
            let mut root_count = 0usize;
            for code in 0..Q.pow(degree as u32) {
                let candidate = Poly::from_code(code, degree);
                if source_equation(candidate).rem(polynomial) == Poly::ZERO {
                    assert!(root_count < 2);
                    roots[root_count] = candidate;
                    root_count += 1;
                }
            }
            match root_count {
                0 => inert[degree] += 1,
                1 => {
                    ramified[degree] += 1;
                    assert_eq!(polynomial, Poly::X);
                    assert_eq!(roots[0], Poly::ZERO);
                }
                2 => split[degree] += 1,
                count => panic!("unexpected branch count {count}"),
            }

            let maximum = MAX_POLE / degree;
            let mut powers = [Poly::ZERO; MAX_POLE + 1];
            powers[0] = Poly::ONE;
            for exponent in 1..=maximum {
                powers[exponent] = powers[exponent - 1].mul(polynomial);
            }

            let mut lifts = [[Poly::ZERO; MAX_POLE + 1]; 2];
            if root_count == 2 {
                for root_index in 0..root_count {
                    let mut lift = roots[root_index];
                    lifts[root_index][1] = lift;
                    for exponent in 1..maximum {
                        let next_modulus = powers[exponent + 1];
                        let equation = source_equation(lift).rem(next_modulus);
                        let (quotient, remainder) = equation.divrem_monic(powers[exponent]);
                        assert_eq!(remainder, Poly::ZERO);
                        let error = quotient.rem(polynomial);
                        let h_inverse = field.inverse(source_h().rem(polynomial));
                        let correction = field.multiply(error, h_inverse);
                        lift = lift.add(powers[exponent].mul(correction));
                        lift = lift.rem(next_modulus);
                        assert_eq!(source_equation(lift).rem(next_modulus), Poly::ZERO);
                        assert_eq!(lift.rem(polynomial), roots[root_index]);
                        lifts[root_index][exponent + 1] = lift;
                    }
                }
            }

            let mut place_indices = [0u8; 2];
            for index in 0..root_count {
                place_indices[index] = u8::try_from(place_degrees.len()).unwrap();
                place_degrees.push(u8::try_from(degree).unwrap());
            }
            result.push(FactorData {
                polynomial,
                degree,
                roots,
                root_count,
                place_indices,
                powers,
                lifts,
            });
        }

        assert_eq!((split[1], ramified[1], inert[1]), (2, 1, 5));
        assert_eq!((split[2], ramified[2], inert[2]), (22, 0, 6));
        assert_eq!((split[3], ramified[3], inert[3]), (92, 0, 76));
        assert_eq!(
            result
                .iter()
                .filter(|factor| factor.root_count != 0)
                .count(),
            117
        );
        assert_eq!(place_degrees.len(), 233);
        (result, place_degrees)
    }

    /// Up to nine `(place,multiplicity)` pairs, packed in twelve bits each.
    #[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
    struct SparseDivisor(u128);

    impl SparseDivisor {
        fn from_pairs(mut pairs: Vec<(u8, u8)>) -> Self {
            pairs.retain(|(_, multiplicity)| *multiplicity != 0);
            pairs.sort_unstable();
            for pair in pairs.windows(2) {
                assert_ne!(pair[0].0, pair[1].0);
            }
            assert!(pairs.len() <= MAX_POLE);
            let mut packed = 0u128;
            for (position, (place, multiplicity)) in pairs.into_iter().enumerate() {
                assert!(multiplicity <= MAX_POLE as u8);
                let slot = u16::from(place) + 1 | (u16::from(multiplicity) << 8);
                packed |= u128::from(slot) << (12 * position);
            }
            Self(packed)
        }

        #[inline]
        fn entry(self, position: usize) -> Option<(u8, u8)> {
            let slot = ((self.0 >> (12 * position)) & 0xfff) as u16;
            (slot != 0).then(|| (((slot & 0xff) - 1) as u8, (slot >> 8) as u8))
        }

        fn entries(self) -> Vec<(u8, u8)> {
            (0..MAX_POLE)
                .map_while(|position| self.entry(position))
                .collect()
        }

        fn weighted_degree(self, place_degrees: &[u8]) -> usize {
            self.entries()
                .into_iter()
                .map(|(place, multiplicity)| {
                    usize::from(place_degrees[usize::from(place)]) * usize::from(multiplicity)
                })
                .sum()
        }

        fn shares_support(self, other: Self) -> bool {
            let mut left = 0usize;
            let mut right = 0usize;
            loop {
                match (self.entry(left), other.entry(right)) {
                    (Some((a, _)), Some((b, _))) if a == b => return true,
                    (Some((a, _)), Some((b, _))) if a < b => left += 1,
                    (Some(_), Some(_)) => right += 1,
                    _ => return false,
                }
            }
        }

        #[inline]
        fn merge_three(left: Self, right: Self, third: Self) -> Self {
            let inputs = [left, right, third];
            let mut positions = [0usize; 3];
            let mut packed = 0u128;
            let mut output_position = 0usize;
            loop {
                let mut least = u8::MAX;
                for index in 0..3 {
                    if let Some((place, _)) = inputs[index].entry(positions[index]) {
                        least = least.min(place);
                    }
                }
                if least == u8::MAX {
                    break;
                }
                let mut multiplicity = 0u8;
                for index in 0..3 {
                    if let Some((place, value)) = inputs[index].entry(positions[index]) {
                        if place == least {
                            multiplicity += value;
                            positions[index] += 1;
                        }
                    }
                }
                assert!(output_position < MAX_POLE);
                let slot = u16::from(least) + 1 | (u16::from(multiplicity) << 8);
                packed |= u128::from(slot) << (12 * output_position);
                output_position += 1;
            }
            Self(packed)
        }
    }

    #[derive(Default)]
    struct FastHasher(u64);

    impl Hasher for FastHasher {
        fn write(&mut self, bytes: &[u8]) {
            let mut state = if self.0 == 0 {
                0xcbf2_9ce4_8422_2325
            } else {
                self.0
            };
            for byte in bytes {
                state ^= u64::from(*byte);
                state = state.wrapping_mul(0x100_0000_01b3);
            }
            self.0 = state;
        }

        fn finish(&self) -> u64 {
            self.0
        }
    }

    type FastSet<T> = HashSet<T, BuildHasherDefault<FastHasher>>;

    fn divisor_support(
        divisor: DivisorKey,
        factors: &[FactorData],
        place_degrees: &[u8],
    ) -> SparseDivisor {
        let mut remaining = divisor.u;
        let mut pairs = Vec::new();
        for factor in factors {
            if factor.degree > remaining.degree().unwrap_or(0) {
                break;
            }
            let mut multiplicity = 0u8;
            loop {
                let (quotient, remainder) = remaining.divrem_monic(factor.polynomial);
                if remainder != Poly::ZERO {
                    break;
                }
                remaining = quotient;
                multiplicity += 1;
            }
            if multiplicity == 0 {
                continue;
            }
            let branch = divisor.v.rem(factor.polynomial);
            let root_index = (0..factor.root_count)
                .find(|index| factor.roots[*index] == branch)
                .expect("valid divisor factor has a graph branch");
            pairs.push((factor.place_indices[root_index], multiplicity));
        }
        assert_eq!(remaining, Poly::ONE);
        let support = SparseDivisor::from_pairs(pairs);
        assert_eq!(support.weighted_degree(place_degrees), divisor.degree());
        support
    }

    fn enumerate_divisors() -> Vec<DivisorKey> {
        let mut divisors = vec![DivisorKey {
            u: Poly::ONE,
            v: Poly::ZERO,
        }];
        let mut candidate_checks = 0usize;
        for degree in 1..=GENUS {
            for u in monic_polynomials(degree) {
                for code in 0..Q.pow(degree as u32) {
                    candidate_checks += 1;
                    let v = Poly::from_code(code, degree);
                    if source_equation(v).rem(u) == Poly::ZERO {
                        divisors.push(DivisorKey { u, v });
                    }
                }
            }
        }
        assert_eq!(candidate_checks, 266_304);
        divisors.sort_by_key(|divisor| (divisor.degree(), divisor.u, divisor.v));
        assert_eq!(divisors.len(), 486);
        assert_eq!(divisors.iter().copied().collect::<BTreeSet<_>>().len(), 486);
        let mut distribution = [0usize; GENUS + 1];
        for divisor in &divisors {
            distribution[divisor.degree()] += 1;
        }
        assert_eq!(distribution, [1, 5, 56, 424]);
        divisors
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    struct Series([u8; SERIES_PRECISION]);

    impl Series {
        fn parameter() -> Self {
            let mut result = Self::default();
            result.0[1] = 1;
            result
        }

        fn constant(value: u8) -> Self {
            let mut result = Self::default();
            result.0[0] = value;
            result
        }

        fn add(self, other: Self) -> Self {
            let mut result = Self::default();
            for index in 0..SERIES_PRECISION {
                result.0[index] = self.0[index] ^ other.0[index];
            }
            result
        }

        fn scalar_mul(self, scalar: u8) -> Self {
            let mut result = Self::default();
            for index in 0..SERIES_PRECISION {
                result.0[index] = f8_mul(self.0[index], scalar);
            }
            result
        }

        fn mul(self, other: Self) -> Self {
            let mut result = Self::default();
            for left in 0..SERIES_PRECISION {
                if self.0[left] == 0 {
                    continue;
                }
                for right in 0..SERIES_PRECISION - left {
                    if other.0[right] != 0 {
                        result.0[left + right] ^= f8_mul(self.0[left], other.0[right]);
                    }
                }
            }
            result
        }

        fn pow(mut self, mut exponent: usize) -> Self {
            let mut result = Self::constant(1);
            while exponent != 0 {
                if exponent & 1 != 0 {
                    result = result.mul(self);
                }
                exponent >>= 1;
                if exponent != 0 {
                    self = self.mul(self);
                }
            }
            result
        }

        fn evaluate(polynomial: Poly, value: Self) -> Self {
            let Some(degree) = polynomial.degree() else {
                return Self::default();
            };
            (0..=degree)
                .rev()
                .fold(Self::default(), |accumulator, index| {
                    accumulator
                        .mul(value)
                        .add(Self::constant(polynomial.coefficient(index)))
                })
        }

        fn valuation(self) -> Option<usize> {
            self.0.iter().position(|coefficient| *coefficient != 0)
        }
    }

    fn ramified_u_series() -> Series {
        let t = Series::parameter();
        let mut u = Series::default();
        for _ in 0..SERIES_PRECISION {
            let next = t
                .pow(2)
                .add(u.pow(2).mul(t))
                .add(u.pow(7))
                .add(u.pow(4))
                .scalar_mul(f8_inverse(COVER_D));
            if next == u {
                break;
            }
            u = next;
        }
        let equation = t
            .pow(2)
            .add(u.pow(2).mul(t))
            .add(u.pow(7))
            .add(u.pow(4))
            .add(u.scalar_mul(COVER_D));
        assert_eq!(equation, Series::default());
        assert_eq!(u.valuation(), Some(2));
        u
    }

    fn ramified_valuation(a: Poly, b: Poly, u_series: Series) -> usize {
        let value = Series::evaluate(a, u_series)
            .add(Series::evaluate(b, u_series).mul(Series::parameter()));
        value
            .valuation()
            .expect("nonzero bounded-pole function exceeds retained precision")
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Rejection {
        Inert,
        HigherDegree,
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub(super) struct Counters {
        pub(super) divisor_candidates: usize,
        pub(super) projective_functions: usize,
        pub(super) retained_principal: usize,
        pub(super) rejected_inert: usize,
        pub(super) rejected_higher_degree: usize,
        pub(super) factor_trials: usize,
        pub(super) source_table_lookups: usize,
        pub(super) associativity_triples: usize,
        pub(super) transfer_pairs: usize,
        pub(super) repeated_classes: usize,
        pub(super) nonsplit_classes: usize,
        pub(super) ramified_classes: usize,
        pub(super) shared_support_pairs: usize,
        pub(super) conjugate_support_pairs: usize,
    }

    fn function_pole(a: Poly, b: Poly) -> usize {
        let a_pole = a.degree().map_or(0, |degree| 2 * degree);
        let b_pole = b.degree().map_or(0, |degree| 7 + 2 * degree);
        a_pole.max(b_pole)
    }

    fn principal_zero_divisor(
        a: Poly,
        b: Poly,
        factors: &[FactorData],
        place_degrees: &[u8],
        u_series: Series,
        counters: &mut Counters,
    ) -> Result<SparseDivisor, Rejection> {
        let pole = function_pole(a, b);
        assert!(pole <= MAX_POLE);
        let norm = a
            .square()
            .add(a.mul(b).mul(source_h()))
            .add(b.square().mul(source_f()));
        assert_eq!(norm.degree(), Some(pole));
        let mut remaining = norm.monic();
        let mut pairs = Vec::new();

        for factor in factors {
            if remaining == Poly::ONE {
                break;
            }
            if factor.degree > remaining.degree().unwrap() {
                break;
            }
            counters.factor_trials += 1;
            let mut multiplicity = 0usize;
            loop {
                let (quotient, remainder) = remaining.divrem_monic(factor.polynomial);
                if remainder != Poly::ZERO {
                    break;
                }
                remaining = quotient;
                multiplicity += 1;
            }
            if multiplicity == 0 {
                continue;
            }
            if factor.root_count == 0 {
                return Err(Rejection::Inert);
            }
            if factor.root_count == 1 {
                assert_eq!(factor.polynomial, Poly::X);
                let valuation = ramified_valuation(a, b, u_series);
                assert_eq!(valuation, multiplicity);
                pairs.push((factor.place_indices[0], valuation as u8));
                continue;
            }

            let mut valuation_sum = 0usize;
            for root_index in 0..2 {
                let mut valuation = 0usize;
                for exponent in 1..=multiplicity {
                    let lift = factor.lifts[root_index][exponent];
                    let value = a.add(b.mul(lift));
                    if value.rem(factor.powers[exponent]) == Poly::ZERO {
                        valuation = exponent;
                    } else {
                        break;
                    }
                }
                valuation_sum += valuation;
                if valuation != 0 {
                    pairs.push((factor.place_indices[root_index], valuation as u8));
                }
            }
            assert_eq!(valuation_sum, multiplicity);
        }
        if remaining != Poly::ONE {
            return Err(Rejection::HigherDegree);
        }
        let zeros = SparseDivisor::from_pairs(pairs);
        assert_eq!(zeros.weighted_degree(place_degrees), pole);
        Ok(zeros)
    }

    fn enumerate_principal_relations(
        factors: &[FactorData],
        place_degrees: &[u8],
        u_series: Series,
        counters: &mut Counters,
    ) -> FastSet<SparseDivisor> {
        // L(9*infinity) = <1,u,u^2,u^3,u^4,v,u*v>.  Normalize the first
        // nonzero coefficient to one, so each projective function appears
        // exactly once.
        let mut relations = FastSet::default();
        let mut retained_by_pole = [0usize; MAX_POLE + 1];
        for pivot in 0..7usize {
            let tail_length = 6 - pivot;
            for mut code in 0..Q.pow(tail_length as u32) {
                let mut coefficients = [0u8; 7];
                coefficients[pivot] = 1;
                for coefficient in &mut coefficients[pivot + 1..] {
                    *coefficient = (code % Q) as u8;
                    code /= Q;
                }
                assert_eq!(code, 0);
                let a = Poly::from_coefficients(&coefficients[..5]);
                let b = Poly::from_coefficients(&coefficients[5..]);
                counters.projective_functions += 1;
                match principal_zero_divisor(a, b, factors, place_degrees, u_series, counters) {
                    Ok(zeros) => {
                        let pole = function_pole(a, b);
                        retained_by_pole[pole] += 1;
                        counters.retained_principal += 1;
                        assert!(
                            relations.insert(zeros),
                            "duplicate projective principal divisor"
                        );
                    }
                    Err(Rejection::Inert) => counters.rejected_inert += 1,
                    Err(Rejection::HigherDegree) => counters.rejected_higher_degree += 1,
                }
            }
        }
        assert_eq!(counters.projective_functions, 299_593);
        assert_eq!(
            retained_by_pole,
            [1, 0, 3, 0, 28, 0, 168, 1_138, 5_798, 26_720]
        );
        assert_eq!(counters.retained_principal, 33_856);
        assert_eq!(relations.len(), 33_856);
        assert_eq!(
            counters.retained_principal + counters.rejected_inert + counters.rejected_higher_degree,
            counters.projective_functions
        );
        relations
    }

    #[inline]
    fn table_add(table: &[u16], size: usize, left: usize, right: usize) -> usize {
        usize::from(table[left * size + right])
    }

    fn table_scalar(table: &[u16], size: usize, point: usize, mut scalar: usize) -> usize {
        let mut result = 0usize;
        let mut base = point;
        while scalar != 0 {
            if scalar & 1 != 0 {
                result = table_add(table, size, result, base);
            }
            scalar >>= 1;
            if scalar != 0 {
                base = table_add(table, size, base, base);
            }
        }
        result
    }

    fn build_group_table(
        divisors: &[DivisorKey],
        supports: &[SparseDivisor],
        principal_relations: &FastSet<SparseDivisor>,
        counters: &mut Counters,
    ) -> (Vec<u16>, Vec<usize>) {
        let size = divisors.len();
        let indices: HashMap<_, _> = divisors
            .iter()
            .copied()
            .enumerate()
            .map(|(index, divisor)| (divisor, index))
            .collect();
        let inverses = divisors
            .iter()
            .map(|divisor| indices[&divisor.involution()])
            .collect::<Vec<_>>();
        let inverse_supports = inverses
            .iter()
            .map(|index| supports[*index])
            .collect::<Vec<_>>();
        let mut table = vec![u16::MAX; size * size];
        for left in 0..size {
            for right in left..size {
                let mut found = None;
                for candidate in 0..size {
                    counters.source_table_lookups += 1;
                    let zeros = SparseDivisor::merge_three(
                        supports[left],
                        supports[right],
                        inverse_supports[candidate],
                    );
                    if principal_relations.contains(&zeros) {
                        assert!(found.replace(candidate).is_none(), "nonunique class sum");
                    }
                }
                let sum = found.expect("principal relations did not determine a class sum");
                table[left * size + right] = u16::try_from(sum).unwrap();
                table[right * size + left] = u16::try_from(sum).unwrap();
            }
        }
        assert_eq!(counters.source_table_lookups, 57_513_726);
        assert!(table.iter().all(|entry| *entry != u16::MAX));

        for point in 0..size {
            assert_eq!(table_add(&table, size, 0, point), point);
            assert_eq!(table_add(&table, size, point, inverses[point]), 0);
            assert_eq!(inverses[inverses[point]], point);
        }
        for left in 0..size {
            for right in 0..size {
                for third in 0..size {
                    counters.associativity_triples += 1;
                    let left_associated =
                        table_add(&table, size, table_add(&table, size, left, right), third);
                    let right_associated =
                        table_add(&table, size, left, table_add(&table, size, right, third));
                    assert_eq!(left_associated, right_associated, "associativity mismatch");
                }
            }
        }
        assert_eq!(counters.associativity_triples, 114_791_256);
        (table, inverses)
    }

    fn group_structure_certificate(table: &[u16], size: usize) {
        let mut orders = vec![0usize; size];
        for point in 0..size {
            let mut multiple = 0usize;
            for order in 1..=size {
                multiple = table_add(table, size, multiple, point);
                if multiple == 0 {
                    orders[point] = order;
                    break;
                }
            }
            assert_ne!(orders[point], 0);
            assert_eq!(size % orders[point], 0);
        }
        assert_eq!(orders.iter().copied().max(), Some(54));

        // Independently exhibit J(H)(GF(8)) as 486 normal forms
        // i*A+j*B (0<=i<9, 0<=j<54), with ord(A)=27, ord(B)=54 and
        // 9*A=18*B.
        let mut presentation = None;
        'outer: for a in 0..size {
            if orders[a] != 27 {
                continue;
            }
            for b in 0..size {
                if orders[b] != 54
                    || table_scalar(table, size, a, 9) != table_scalar(table, size, b, 18)
                {
                    continue;
                }
                let mut forms = BTreeSet::new();
                for i in 0..9 {
                    for j in 0..54 {
                        forms.insert(table_add(
                            table,
                            size,
                            table_scalar(table, size, a, i),
                            table_scalar(table, size, b, j),
                        ));
                    }
                }
                if forms.len() == size {
                    presentation = Some((a, b));
                    break 'outer;
                }
            }
        }
        assert!(presentation.is_some(), "missing 9-by-54 group presentation");
    }

    fn place_images(factors: &[FactorData], place_count: usize) -> Vec<BasePoint> {
        let mut images = vec![None; place_count];
        for factor in factors {
            let field = ResidueField::new(factor.polynomial);
            let alpha = Poly::X.rem(factor.polynomial);
            for root_index in 0..factor.root_count {
                let source_v = factor.roots[root_index];
                assert_eq!(source_equation(source_v).rem(factor.polynomial), Poly::ZERO);
                let target = ExtensionPoint::Affine {
                    x: field.power(alpha, 3),
                    y: field.add(
                        field.multiply(alpha, source_v),
                        Poly::from_coefficients(&[COVER_D]),
                    ),
                };
                assert!(extension_is_on_curve(field, target, TARGET_A, TARGET_B));
                let image = trace_closed_point(field, target, TARGET_A);
                assert!(base_is_on_curve(image, TARGET_A, TARGET_B));
                let place = usize::from(factor.place_indices[root_index]);
                assert!(images[place].replace(image).is_none());
            }
        }
        images
            .into_iter()
            .map(|image| image.expect("graph place has no target image"))
            .collect()
    }

    fn push_support(support: SparseDivisor, place_images: &[BasePoint]) -> BasePoint {
        let mut image = BasePoint::Infinity;
        for (place, multiplicity) in support.entries() {
            for _ in 0..multiplicity {
                image = base_add(image, place_images[usize::from(place)], TARGET_A);
            }
        }
        image
    }

    fn target_points() -> Vec<BasePoint> {
        let mut points = vec![BasePoint::Infinity];
        for x in 0..8u8 {
            for y in 0..8u8 {
                let point = BasePoint::Affine { x, y };
                if base_is_on_curve(point, TARGET_A, TARGET_B) {
                    points.push(point);
                }
            }
        }
        points.sort_unstable();
        assert_eq!(points.len(), 6);
        let set = points.iter().copied().collect::<BTreeSet<_>>();
        for left in &points {
            for right in &points {
                assert!(set.contains(&base_add(*left, *right, TARGET_A)));
            }
        }
        for left in &points {
            for right in &points {
                for third in &points {
                    assert_eq!(
                        base_add(base_add(*left, *right, TARGET_A), *third, TARGET_A),
                        base_add(*left, base_add(*right, *third, TARGET_A), TARGET_A)
                    );
                }
            }
        }
        points
    }

    fn base_scalar(point: BasePoint, mut scalar: usize) -> BasePoint {
        let mut result = BasePoint::Infinity;
        let mut base = point;
        while scalar != 0 {
            if scalar & 1 != 0 {
                result = base_add(result, base, TARGET_A);
            }
            scalar >>= 1;
            if scalar != 0 {
                base = base_add(base, base, TARGET_A);
            }
        }
        result
    }

    fn has_conjugate_support(
        left: SparseDivisor,
        right: SparseDivisor,
        conjugates: &[Option<u8>],
    ) -> bool {
        let right_places = right
            .entries()
            .into_iter()
            .map(|(place, _)| place)
            .collect::<BTreeSet<_>>();
        left.entries().into_iter().any(|(place, _)| {
            conjugates[usize::from(place)].is_some_and(|other| right_places.contains(&other))
        })
    }

    /// Enumerate every reduced class for another member of the same cover
    /// family and compute its pushforward by an independent
    /// factor/evaluate/Frobenius-trace implementation.  This retains the
    /// catalog-general production regression without importing the fixed
    /// fixture's principal-relation table.
    pub(super) fn general_non_table_classes(a: u8, b: u8, d: u8) -> Vec<(DivisorKey, BasePoint)> {
        assert_eq!(f8_mul(d, d), b);
        let irreducibles = irreducibles_through_degree_three();
        let mut divisors = vec![DivisorKey {
            u: Poly::ONE,
            v: Poly::ZERO,
        }];
        for degree in 1..=GENUS {
            for u in monic_polynomials(degree) {
                for code in 0..Q.pow(degree as u32) {
                    let v = Poly::from_code(code, degree);
                    if parameterized_source_equation(v, a, d).rem(u) == Poly::ZERO {
                        divisors.push(DivisorKey { u, v });
                    }
                }
            }
        }
        divisors.sort_by_key(|divisor| (divisor.degree(), divisor.u, divisor.v));
        assert_eq!(
            divisors.iter().copied().collect::<BTreeSet<_>>().len(),
            divisors.len()
        );

        let mut saw_repeated = false;
        let mut saw_nonsplit = false;
        let mut degree_profiles = BTreeSet::new();
        let mut result = Vec::with_capacity(divisors.len());
        for divisor in divisors {
            if divisor.u == Poly::ONE {
                result.push((divisor, BasePoint::Infinity));
                continue;
            }
            let mut remaining = divisor.u;
            let mut image = BasePoint::Infinity;
            let mut profile = Vec::new();
            for factor in &irreducibles {
                let degree = factor.degree().unwrap();
                if degree > remaining.degree().unwrap_or(0) {
                    break;
                }
                let mut multiplicity = 0usize;
                loop {
                    let (quotient, remainder) = remaining.divrem_monic(*factor);
                    if remainder != Poly::ZERO {
                        break;
                    }
                    remaining = quotient;
                    multiplicity += 1;
                }
                if multiplicity == 0 {
                    continue;
                }
                saw_repeated |= multiplicity > 1;
                saw_nonsplit |= degree > 1;
                for _ in 0..multiplicity {
                    profile.push(u8::try_from(degree).unwrap());
                }

                let field = ResidueField::new(*factor);
                let alpha = Poly::X.rem(*factor);
                let source_v = divisor.v.rem(*factor);
                assert_eq!(
                    parameterized_source_equation(source_v, a, d).rem(*factor),
                    Poly::ZERO
                );
                let target = ExtensionPoint::Affine {
                    x: field.power(alpha, 3),
                    y: field.add(
                        field.multiply(alpha, source_v),
                        Poly::from_coefficients(&[d]),
                    ),
                };
                assert!(extension_is_on_curve(field, target, a, b));
                let component = trace_closed_point(field, target, a);
                assert!(base_is_on_curve(component, a, b));
                for _ in 0..multiplicity {
                    image = base_add(image, component, a);
                }
            }
            assert_eq!(remaining, Poly::ONE);
            profile.sort_unstable();
            degree_profiles.insert(profile);
            assert!(base_is_on_curve(image, a, b));
            result.push((divisor, image));
        }
        assert!(saw_repeated);
        assert!(saw_nonsplit);
        assert!(degree_profiles.contains(&vec![3]));
        assert!(degree_profiles.contains(&vec![1, 2]));
        assert!(degree_profiles.contains(&vec![1, 1, 1]));
        result
    }

    pub(super) struct Oracle {
        pub(super) divisors: Vec<DivisorKey>,
        pub(super) table: Vec<u16>,
        pub(super) inverses: Vec<usize>,
        pub(super) images: Vec<BasePoint>,
        pub(super) target_points: Vec<BasePoint>,
        pub(super) pullbacks: HashMap<BasePoint, usize>,
        pub(super) counters: Counters,
    }

    impl Oracle {
        pub(super) fn sum(&self, left: usize, right: usize) -> usize {
            table_add(&self.table, self.divisors.len(), left, right)
        }

        pub(super) fn scalar(&self, point: usize, scalar: usize) -> usize {
            table_scalar(&self.table, self.divisors.len(), point, scalar)
        }
    }

    pub(super) fn build_oracle() -> Oracle {
        assert_eq!(f8_mul(COVER_D, COVER_D), TARGET_B);
        for value in 1..8u8 {
            assert_eq!(f8_mul(value, f8_inverse(value)), 1);
        }
        // At infinity, ord_H(u)=-2 and ord_H(v)=-7. Therefore the mapped
        // coordinates have poles ord_H(u^3)=-6 and ord_H(u*v+d)=-9,
        // exactly three times ord_E(x)=-2 and ord_E(y)=-3. The unique
        // source infinity consequently has ramification index three.
        let source_u_pole = 2usize;
        let source_v_pole = 7usize;
        let target_x_pole = 2usize;
        let target_y_pole = 3usize;
        let infinity_ramification_index = 3usize;
        assert_eq!(
            3 * source_u_pole,
            infinity_ramification_index * target_x_pole
        );
        assert_eq!(
            source_u_pole + source_v_pole,
            infinity_ramification_index * target_y_pole
        );
        let (factors, place_degrees) = factor_data();
        let inert_u_plus_one = Poly::from_coefficients(&[1, 1]);
        assert_eq!(
            factors
                .iter()
                .find(|factor| factor.polynomial == inert_u_plus_one)
                .expect("u+1 factor")
                .root_count,
            0,
            "the inert u+1 fiber must not be admitted as a graph place"
        );
        let u_series = ramified_u_series();
        assert_eq!(u_series.mul(Series::parameter()).valuation(), Some(3));

        let divisors = enumerate_divisors();
        let supports = divisors
            .iter()
            .map(|divisor| divisor_support(*divisor, &factors, &place_degrees))
            .collect::<Vec<_>>();
        let mut counters = Counters {
            divisor_candidates: 266_304,
            ..Counters::default()
        };

        let ramified_place = factors
            .iter()
            .find(|factor| factor.polynomial == Poly::X)
            .expect("ramified u=0 factor")
            .place_indices[0];
        let mut degree_profiles = BTreeSet::new();
        for support in &supports {
            let entries = support.entries();
            counters.repeated_classes +=
                entries.iter().any(|(_, multiplicity)| *multiplicity > 1) as usize;
            counters.nonsplit_classes += (entries
                .iter()
                .any(|(place, _)| place_degrees[usize::from(*place)] > 1))
                as usize;
            counters.ramified_classes +=
                entries.iter().any(|(place, _)| *place == ramified_place) as usize;
            let mut profile = Vec::new();
            for (place, multiplicity) in entries {
                for _ in 0..multiplicity {
                    profile.push(place_degrees[usize::from(place)]);
                }
            }
            profile.sort_unstable();
            degree_profiles.insert(profile);
        }
        assert!(counters.repeated_classes > 0);
        assert!(counters.nonsplit_classes > 0);
        assert!(counters.ramified_classes > 0);
        assert!(degree_profiles.contains(&vec![3]));
        assert!(degree_profiles.contains(&vec![1, 2]));
        assert!(degree_profiles.contains(&vec![1, 1, 1]));

        let mut conjugates = vec![None; place_degrees.len()];
        for factor in &factors {
            if factor.root_count == 2 {
                let left = factor.place_indices[0];
                let right = factor.place_indices[1];
                conjugates[usize::from(left)] = Some(right);
                conjugates[usize::from(right)] = Some(left);
            }
        }
        for left in 0..supports.len() {
            for right in left..supports.len() {
                counters.shared_support_pairs +=
                    supports[left].shares_support(supports[right]) as usize;
                counters.conjugate_support_pairs +=
                    has_conjugate_support(supports[left], supports[right], &conjugates) as usize;
            }
        }
        assert!(counters.shared_support_pairs > 0);
        assert!(counters.conjugate_support_pairs > 0);

        let principal_relations =
            enumerate_principal_relations(&factors, &place_degrees, u_series, &mut counters);
        let (table, inverses) =
            build_group_table(&divisors, &supports, &principal_relations, &mut counters);
        group_structure_certificate(&table, divisors.len());

        let mapped_places = place_images(&factors, place_degrees.len());
        let images = supports
            .iter()
            .map(|support| push_support(*support, &mapped_places))
            .collect::<Vec<_>>();
        assert!(images
            .iter()
            .all(|image| base_is_on_curve(*image, TARGET_A, TARGET_B)));
        for left in 0..divisors.len() {
            for right in 0..divisors.len() {
                counters.transfer_pairs += 1;
                assert_eq!(
                    images[table_add(&table, divisors.len(), left, right)],
                    base_add(images[left], images[right], TARGET_A),
                    "pushforward homomorphism mismatch"
                );
            }
        }
        assert_eq!(counters.transfer_pairs, 236_196);

        let target_points = target_points();
        let target_set = target_points.iter().copied().collect::<BTreeSet<_>>();
        assert_eq!(images.iter().copied().collect::<BTreeSet<_>>(), target_set);
        let mut fiber_sizes = HashMap::<BasePoint, usize>::new();
        for image in &images {
            *fiber_sizes.entry(*image).or_default() += 1;
        }
        assert!(target_points
            .iter()
            .all(|point| fiber_sizes.get(point) == Some(&81)));

        let divisor_indices: HashMap<_, _> = divisors
            .iter()
            .copied()
            .enumerate()
            .map(|(index, divisor)| (divisor, index))
            .collect();
        let ramified_divisor = divisor_indices[&DivisorKey {
            u: Poly::X,
            v: Poly::ZERO,
        }];
        assert_eq!(table_scalar(&table, divisors.len(), ramified_divisor, 2), 0);
        assert_eq!(
            table_scalar(&table, divisors.len(), ramified_divisor, 3),
            ramified_divisor
        );

        let mut pullbacks = HashMap::new();
        for point in &target_points {
            let pullback = match *point {
                BasePoint::Infinity => 0,
                BasePoint::Affine { x: 0, y } => {
                    assert_eq!(y, COVER_D);
                    assert_eq!(base_scalar(*point, 2), BasePoint::Infinity);
                    table_scalar(&table, divisors.len(), ramified_divisor, 3)
                }
                BasePoint::Affine { x, y } => {
                    let coefficient = f8_mul(y ^ COVER_D, f8_inverse(x));
                    let divisor = DivisorKey {
                        u: Poly::from_coefficients(&[x, 0, 0, 1]),
                        v: Poly::from_coefficients(&[0, 0, coefficient]),
                    };
                    assert_eq!(source_equation(divisor.v).rem(divisor.u), Poly::ZERO);
                    divisor_indices[&divisor]
                }
            };
            assert!(pullbacks.insert(*point, pullback).is_none());
            assert_eq!(images[pullback], base_scalar(*point, 3));
        }
        for left in &target_points {
            for right in &target_points {
                assert_eq!(
                    pullbacks[&base_add(*left, *right, TARGET_A)],
                    table_add(&table, divisors.len(), pullbacks[left], pullbacks[right])
                );
            }
        }

        for point in 0..divisors.len() {
            let e = pullbacks[&images[point]];
            let e_squared = pullbacks[&images[e]];
            let three_e = table_scalar(&table, divisors.len(), e, 3);
            assert_eq!(e_squared, three_e, "projector identity e^2=[3]e failed");
        }

        Oracle {
            divisors,
            table,
            inverses,
            images,
            target_points,
            pullbacks,
            counters,
        }
    }
}

fn element(value: u8) -> F2mElement {
    F2mElement::from_biguint(&BigUint::from(value), 3)
}

fn independent_poly(polynomial: &F2mPoly) -> independent::Poly {
    let coefficients = polynomial
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
        .collect::<Vec<_>>();
    independent::Poly::from_coefficients(&coefficients)
}

fn production_divisor(divisor: independent::DivisorKey) -> MumfordDivisor {
    let polynomial = |value: independent::Poly| {
        F2mPoly::from_coeffs(value.coefficients().into_iter().map(element).collect(), 3)
    };
    MumfordDivisor {
        u: polynomial(divisor.u),
        v: polynomial(divisor.v),
    }
}

fn production_key(divisor: &MumfordDivisor) -> independent::DivisorKey {
    independent::DivisorKey {
        u: independent_poly(&divisor.u),
        v: independent_poly(&divisor.v),
    }
}

fn production_point(point: &BinaryPoint) -> independent::BasePoint {
    let word = |value: &F2mElement| {
        value
            .to_biguint()
            .to_u32_digits()
            .first()
            .copied()
            .unwrap_or(0) as u8
    };
    match point {
        BinaryPoint::Infinity => independent::BasePoint::Infinity,
        BinaryPoint::Affine { x, y } => independent::BasePoint::Affine {
            x: word(x),
            y: word(y),
        },
    }
}

fn binary_point(point: independent::BasePoint) -> BinaryPoint {
    match point {
        independent::BasePoint::Infinity => BinaryPoint::Infinity,
        independent::BasePoint::Affine { x, y } => BinaryPoint::Affine {
            x: element(x),
            y: element(y),
        },
    }
}

#[test]
fn independent_f8_principal_divisor_oracle_checks_every_class_and_map() {
    // This call completes every independent source-law and transfer check
    // before any production object is constructed.
    let oracle = independent::build_oracle();
    eprintln!("fixed-GF(8) independent counters: {:#?}", oracle.counters);
    assert_eq!(oracle.counters.divisor_candidates, 266_304);
    assert_eq!(oracle.counters.projective_functions, 299_593);
    assert_eq!(oracle.counters.retained_principal, 33_856);
    assert_eq!(oracle.counters.rejected_inert, 23_277);
    assert_eq!(oracle.counters.rejected_higher_degree, 242_460);
    assert_eq!(oracle.counters.factor_trials, 53_627_780);
    assert_eq!(oracle.counters.source_table_lookups, 57_513_726);
    assert_eq!(oracle.counters.associativity_triples, 114_791_256);
    assert_eq!(oracle.counters.transfer_pairs, 236_196);
    assert_eq!(oracle.counters.repeated_classes, 20);
    assert_eq!(oracle.counters.nonsplit_classes, 448);
    assert_eq!(oracle.counters.ramified_classes, 57);
    assert_eq!(oracle.counters.shared_support_pairs, 9_077);
    assert_eq!(oracle.counters.conjugate_support_pairs, 7_262);

    let modulus = IrreduciblePoly {
        degree: 3,
        low_terms: vec![0, 1],
    };
    let cover = OrdinaryBinaryCover::new(3, modulus, element(1), element(3)).unwrap();
    let production_divisors = oracle
        .divisors
        .iter()
        .copied()
        .map(production_divisor)
        .collect::<Vec<_>>();
    let indices = oracle
        .divisors
        .iter()
        .copied()
        .enumerate()
        .map(|(index, divisor)| (divisor, index))
        .collect::<std::collections::HashMap<_, _>>();

    for (index, divisor) in production_divisors.iter().enumerate() {
        divisor.validate(cover.source_curve()).unwrap();
        let negative = divisor.neg_fast(cover.source_curve());
        assert_eq!(indices[&production_key(&negative)], oracle.inverses[index]);
        let pushforward = cover.pushforward_arbitrary_jacobian_class(divisor).unwrap();
        assert_eq!(production_point(&pushforward), oracle.images[index]);
        let endomorphism = cover.elliptic_image_endomorphism(divisor).unwrap();
        assert_eq!(
            indices[&production_key(&endomorphism)],
            oracle.pullbacks[&oracle.images[index]]
        );
        for scalar in 0..=54usize {
            let product = divisor.scalar_mul_fast(&BigUint::from(scalar), cover.source_curve());
            assert_eq!(
                indices[&production_key(&product)],
                oracle.scalar(index, scalar),
                "production scalar mismatch at class {index}, scalar {scalar}"
            );
        }
    }

    for left in 0..production_divisors.len() {
        for right in 0..production_divisors.len() {
            let sum = production_divisors[left]
                .add_fast(&production_divisors[right], cover.source_curve());
            assert_eq!(
                indices[&production_key(&sum)],
                oracle.sum(left, right),
                "production addition mismatch for ({left},{right})"
            );
        }
    }

    for point in &oracle.target_points {
        let production = binary_point(*point);
        assert!(cover.target_is_on_curve(&production));
        let pullback = cover.pullback_point_class(&production).unwrap();
        assert_eq!(indices[&production_key(&pullback)], oracle.pullbacks[point]);
    }
}

#[test]
fn independent_all_class_oracle_also_exercises_the_general_non_table_path() {
    // The independent factor/evaluate/trace work is completed before the
    // production cover is constructed.
    let oracle = independent::general_non_table_classes(0, 1, 1);
    assert!(oracle.len() > 1);
    let modulus = IrreduciblePoly {
        degree: 3,
        low_terms: vec![0, 1],
    };
    // Only (a,b)=(1,3) selects the retained fixed-fixture table.
    let cover = OrdinaryBinaryCover::new(3, modulus, element(0), element(1)).unwrap();
    for (key, expected) in oracle {
        let divisor = production_divisor(key);
        divisor.validate(cover.source_curve()).unwrap();
        let actual = cover
            .pushforward_arbitrary_jacobian_class(&divisor)
            .unwrap();
        assert_eq!(production_point(&actual), expected, "divisor {key:?}");
    }
}
