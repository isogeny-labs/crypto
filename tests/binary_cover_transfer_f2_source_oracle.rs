//! Complete independent source-law oracle for one tiny binary cover.
//!
//! The fixture is
//!
//! ```text
//! E : y^2 + x*y = x^3 + 1,
//! H : v^2 + u^2*v = u^7 + u,
//! phi(u,v) = (u^3, u*v + 1)
//! ```
//!
//! over `F_2`.  The oracle enumerates the 20 canonical source classes and
//! constructs their complete group table from principal divisors of
//! `A(u)+B(u)v`.  It does not use the production polynomial, Mumford, or
//! Jacobian law while constructing that table.  Production objects appear
//! only at the final differential boundary.  This is exhaustive evidence for
//! this exact tiny fixture, not a catalog-wide oracle, solver, or performance
//! claim.

use crypto_lib::binary_ecc::{
    BinaryPoint, F2mElement, F2mPoly, IrreduciblePoly, MumfordDivisor, OrdinaryBinaryCover,
};
use num_bigint::BigUint;
use std::collections::{BTreeMap, HashMap};

type Poly = u32;

const X: Poly = 0b10;
const H: Poly = 0b100; // x^2
const F: Poly = 0b1000_0010; // x^7+x
const MAX_POLE: usize = 9;

fn degree(value: Poly) -> Option<usize> {
    (value != 0).then(|| (u32::BITS - 1 - value.leading_zeros()) as usize)
}

fn multiply(mut left: Poly, mut right: Poly) -> Poly {
    let mut result = 0;
    while right != 0 {
        if right & 1 != 0 {
            result ^= left;
        }
        right >>= 1;
        left <<= 1;
    }
    result
}

fn power(mut base: Poly, mut exponent: usize) -> Poly {
    let mut result = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = multiply(result, base);
        }
        exponent >>= 1;
        if exponent != 0 {
            base = multiply(base, base);
        }
    }
    result
}

fn divrem(mut numerator: Poly, denominator: Poly) -> (Poly, Poly) {
    let denominator_degree = degree(denominator).expect("nonzero denominator");
    let mut quotient = 0;
    while let Some(numerator_degree) = degree(numerator) {
        if numerator_degree < denominator_degree {
            break;
        }
        let shift = numerator_degree - denominator_degree;
        quotient ^= 1 << shift;
        numerator ^= denominator << shift;
    }
    (quotient, numerator)
}

fn remainder(numerator: Poly, denominator: Poly) -> Poly {
    divrem(numerator, denominator).1
}

fn curve_equation(y: Poly) -> Poly {
    multiply(y, y) ^ multiply(H, y) ^ F
}

fn monic_polynomials(polynomial_degree: usize) -> impl Iterator<Item = Poly> {
    let lead = 1 << polynomial_degree;
    (0..lead).map(move |low| lead | low)
}

fn is_irreducible(polynomial: Poly) -> bool {
    let polynomial_degree = degree(polynomial).expect("monic polynomial");
    if polynomial_degree == 1 {
        return true;
    }
    for divisor_degree in 1..=polynomial_degree / 2 {
        for divisor in monic_polynomials(divisor_degree) {
            if remainder(polynomial, divisor) == 0 {
                return false;
            }
        }
    }
    true
}

fn irreducibles() -> Vec<Poly> {
    (1..=3)
        .flat_map(monic_polynomials)
        .filter(|polynomial| is_irreducible(*polynomial))
        .collect()
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct DivisorKey {
    u: Poly,
    v: Poly,
}

impl DivisorKey {
    fn support_degree(self) -> usize {
        degree(self.u).expect("u is nonzero")
    }

    fn involution(self) -> Self {
        Self {
            u: self.u,
            v: remainder(self.v ^ H, self.u),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Place {
    polynomial: Poly,
    branch: Poly,
}

impl Place {
    fn degree(self) -> usize {
        degree(self.polynomial).expect("place polynomial")
    }

    fn is_ramified(self) -> bool {
        self.polynomial == X
    }
}

fn enumerate_places(irreducible: &[Poly]) -> Vec<Place> {
    let mut places = Vec::new();
    for &polynomial in irreducible {
        let d = degree(polynomial).unwrap();
        for branch in 0..(1 << d) {
            if remainder(curve_equation(branch), polynomial) == 0 {
                places.push(Place { polynomial, branch });
            }
        }
    }
    places.sort();
    assert_eq!(places.iter().filter(|place| place.is_ramified()).count(), 1);
    places
}

fn enumerate_divisors() -> Vec<DivisorKey> {
    let mut divisors = vec![DivisorKey { u: 1, v: 0 }];
    for d in 1..=3 {
        for u in monic_polynomials(d) {
            for v in 0..(1 << d) {
                if remainder(curve_equation(v), u) == 0 {
                    divisors.push(DivisorKey { u, v });
                }
            }
        }
    }
    divisors.sort_by_key(|divisor| (divisor.support_degree(), divisor.u, divisor.v));
    assert_eq!(divisors.len(), 20);
    let mut distribution = [0usize; 4];
    for divisor in &divisors {
        distribution[divisor.support_degree()] += 1;
    }
    assert_eq!(distribution, [1, 3, 6, 10]);
    divisors
}

fn place_vector(divisor: DivisorKey, places: &[Place], irreducible: &[Poly]) -> Vec<u8> {
    let mut vector = vec![0u8; places.len()];
    let mut remaining = divisor.u;
    for &polynomial in irreducible {
        let mut multiplicity = 0u8;
        loop {
            let (quotient, rest) = divrem(remaining, polynomial);
            if rest != 0 {
                break;
            }
            remaining = quotient;
            multiplicity += 1;
        }
        if multiplicity == 0 {
            continue;
        }
        let branch = remainder(divisor.v, polynomial);
        let index = places
            .iter()
            .position(|place| place.polynomial == polynomial && place.branch == branch)
            .expect("divisor branch must define a curve place");
        vector[index] = multiplicity;
    }
    assert_eq!(remaining, 1);
    let weighted_degree: usize = vector
        .iter()
        .zip(places)
        .map(|(multiplicity, place)| usize::from(*multiplicity) * place.degree())
        .sum();
    assert_eq!(weighted_degree, divisor.support_degree());
    vector
}

fn polynomial_modulus_power(polynomial: Poly, exponent: usize) -> Poly {
    power(polynomial, exponent)
}

#[derive(Clone, Debug)]
struct PlaceLift {
    place: Place,
    powers: Vec<Poly>,
    branch_lift: Option<Poly>,
}

fn place_lifts(places: &[Place]) -> Vec<PlaceLift> {
    places
        .iter()
        .copied()
        .map(|place| {
            if place.is_ramified() {
                return PlaceLift {
                    place,
                    powers: Vec::new(),
                    branch_lift: None,
                };
            }
            let maximum = MAX_POLE / place.degree();
            let powers: Vec<_> = (1..=maximum)
                .map(|exponent| polynomial_modulus_power(place.polynomial, exponent))
                .collect();
            let modulus = *powers.last().expect("positive lift precision");
            let modulus_degree = degree(modulus).unwrap();
            let roots: Vec<_> = (0..(1 << modulus_degree))
                .filter(|candidate| {
                    remainder(*candidate, place.polynomial) == place.branch
                        && remainder(curve_equation(*candidate), modulus) == 0
                })
                .collect();
            assert_eq!(roots.len(), 1, "unique Hensel branch for {place:?}");
            PlaceLift {
                place,
                powers,
                branch_lift: Some(roots[0]),
            }
        })
        .collect()
}

fn evaluate_at_series(polynomial: Poly, value: Poly, mask: Poly) -> Poly {
    let mut result = 0;
    for index in (0..=degree(polynomial).unwrap_or(0)).rev() {
        result = multiply(result, value) & mask;
        if polynomial & (1 << index) != 0 {
            result ^= 1;
        }
    }
    result & mask
}

fn ramified_x_series() -> (Poly, Poly) {
    // Precision one beyond the largest possible pole distinguishes every
    // finite order through MAX_POLE.
    let precision = MAX_POLE + 2;
    let modulus = 1 << precision;
    let mask = modulus - 1;
    let t = X;
    let solutions: Vec<_> = (0..modulus)
        .filter(|candidate| candidate & 1 == 0)
        .filter(|candidate| {
            let x = *candidate;
            let equation =
                (1 << 2) ^ (multiply(multiply(x, x), t) & mask) ^ (power(x, 7) & mask) ^ x;
            equation & mask == 0
        })
        .collect();
    assert_eq!(solutions.len(), 1);
    (solutions[0], mask)
}

fn finite_valuation(a: Poly, b: Poly, lift: &PlaceLift, ramified: (Poly, Poly)) -> usize {
    if lift.place.is_ramified() {
        let (x_series, mask) = ramified;
        let value = evaluate_at_series(a, x_series, mask)
            ^ (multiply(evaluate_at_series(b, x_series, mask), X) & mask);
        assert_ne!(value, 0, "nonzero function vanished to retained precision");
        return value.trailing_zeros() as usize;
    }
    let branch = lift.branch_lift.expect("unramified Hensel branch");
    let value = a ^ multiply(b, branch);
    lift.powers
        .iter()
        .take_while(|modulus| remainder(value, **modulus) == 0)
        .count()
}

fn function_pole(a: Poly, b: Poly) -> usize {
    let a_pole = degree(a).map_or(0, |d| 2 * d);
    let b_pole = degree(b).map_or(0, |d| 7 + 2 * d);
    a_pole.max(b_pole)
}

fn principal_zero_vectors(places: &[Place]) -> HashMap<Vec<u8>, (Poly, Poly, usize)> {
    let lifts = place_lifts(places);
    let ramified = ramified_x_series();
    let mut principal = HashMap::new();
    for a in 0..(1 << 5) {
        for b in 0..(1 << 2) {
            if a == 0 && b == 0 {
                continue;
            }
            let pole = function_pole(a, b);
            if pole > MAX_POLE {
                continue;
            }
            let vector: Vec<u8> = lifts
                .iter()
                .map(|lift| finite_valuation(a, b, lift, ramified) as u8)
                .collect();
            let known_degree: usize = vector
                .iter()
                .zip(places)
                .map(|(multiplicity, place)| usize::from(*multiplicity) * place.degree())
                .sum();
            if known_degree == pole {
                assert!(principal.insert(vector, (a, b, pole)).is_none());
            }
        }
    }
    principal
}

fn add_vectors(left: &[u8], right: &[u8], third: &[u8]) -> Vec<u8> {
    left.iter()
        .zip(right)
        .zip(third)
        .map(|((a, b), c)| a + b + c)
        .collect()
}

fn build_group_table(
    divisors: &[DivisorKey],
    vectors: &[Vec<u8>],
    involution_vectors: &[Vec<u8>],
    principal: &HashMap<Vec<u8>, (Poly, Poly, usize)>,
) -> Vec<Vec<usize>> {
    let mut table = vec![vec![usize::MAX; divisors.len()]; divisors.len()];
    for left in 0..divisors.len() {
        for right in 0..divisors.len() {
            let matches: Vec<_> = (0..divisors.len())
                .filter(|candidate| {
                    let zeros = add_vectors(
                        &vectors[left],
                        &vectors[right],
                        &involution_vectors[*candidate],
                    );
                    principal.get(&zeros).is_some_and(|(_, _, pole)| {
                        *pole
                            == divisors[left].support_degree()
                                + divisors[right].support_degree()
                                + divisors[*candidate].support_degree()
                    })
                })
                .collect();
            assert_eq!(matches.len(), 1, "sum ({left},{right}) was not unique");
            table[left][right] = matches[0];
        }
    }
    table
}

fn inverse_indices(table: &[Vec<usize>]) -> Vec<usize> {
    (0..table.len())
        .map(|index| {
            (0..table.len())
                .find(|candidate| table[index][*candidate] == 0)
                .expect("every class has an inverse")
        })
        .collect()
}

fn scalar_index(table: &[Vec<usize>], element: usize, scalar: usize) -> usize {
    let mut accumulator = 0;
    for _ in 0..scalar {
        accumulator = table[accumulator][element];
    }
    accumulator
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum TargetPoint {
    Infinity,
    Affine { x: u8, y: u8 },
}

fn target_is_on_curve(point: TargetPoint) -> bool {
    match point {
        TargetPoint::Infinity => true,
        // Squaring and cubing are the identity on F2.
        TargetPoint::Affine { x, y } => y ^ (x & y) == x ^ 1,
    }
}

fn target_add(left: TargetPoint, right: TargetPoint) -> TargetPoint {
    match (left, right) {
        (TargetPoint::Infinity, point) | (point, TargetPoint::Infinity) => point,
        (TargetPoint::Affine { x: x1, y: y1 }, TargetPoint::Affine { x: x2, y: y2 }) => {
            if x1 == x2 {
                if y1 != y2 || x1 == 0 {
                    return TargetPoint::Infinity;
                }
                let lambda = x1 ^ y1; // division by x1=1
                let x3 = 0; // lambda^2+lambda=0 in F2
                let y3 = x1 ^ ((lambda ^ 1) & x3);
                TargetPoint::Affine { x: x3, y: y3 }
            } else {
                let lambda = y1 ^ y2; // x1+x2=1
                let x3 = x1 ^ x2; // lambda^2+lambda=0 in F2
                let y3 = (lambda & (x1 ^ x3)) ^ x3 ^ y1;
                TargetPoint::Affine { x: x3, y: y3 }
            }
        }
    }
}

fn target_scalar(point: TargetPoint, scalar: usize) -> TargetPoint {
    let mut result = TargetPoint::Infinity;
    for _ in 0..scalar {
        result = target_add(result, point);
    }
    result
}

fn residue_multiply(left: Poly, right: Poly, modulus: Poly) -> Poly {
    remainder(multiply(left, right), modulus)
}

fn residue_power(mut base: Poly, mut exponent: usize, modulus: Poly) -> Poly {
    let mut result = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = residue_multiply(result, base, modulus);
        }
        exponent >>= 1;
        if exponent != 0 {
            base = residue_multiply(base, base, modulus);
        }
    }
    result
}

fn source_point_count(modulus: Poly) -> usize {
    let field_degree = degree(modulus).expect("field modulus");
    let field_size = 1 << field_degree;
    let mut points = 1usize; // the unique rational point at infinity
    for x in 0..field_size {
        for y in 0..field_size {
            let left = residue_multiply(y, y, modulus)
                ^ residue_multiply(residue_multiply(x, x, modulus), y, modulus);
            let right = residue_power(x, 7, modulus) ^ x;
            points += usize::from(left == right);
        }
    }
    points
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExtensionTargetPoint {
    Infinity,
    Affine { x: Poly, y: Poly },
}

fn extension_target_add(
    left: ExtensionTargetPoint,
    right: ExtensionTargetPoint,
    modulus: Poly,
) -> ExtensionTargetPoint {
    let d = degree(modulus).unwrap();
    let inverse = |value: Poly| residue_power(value, (1 << d) - 2, modulus);
    match (left, right) {
        (ExtensionTargetPoint::Infinity, point) | (point, ExtensionTargetPoint::Infinity) => point,
        (
            ExtensionTargetPoint::Affine { x: x1, y: y1 },
            ExtensionTargetPoint::Affine { x: x2, y: y2 },
        ) => {
            if x1 == x2 {
                if y1 != y2 || x1 == 0 {
                    return ExtensionTargetPoint::Infinity;
                }
                let lambda = x1 ^ residue_multiply(y1, inverse(x1), modulus);
                let x3 = residue_multiply(lambda, lambda, modulus) ^ lambda;
                let y3 =
                    residue_multiply(x1, x1, modulus) ^ residue_multiply(lambda ^ 1, x3, modulus);
                ExtensionTargetPoint::Affine { x: x3, y: y3 }
            } else {
                let lambda = residue_multiply(y1 ^ y2, inverse(x1 ^ x2), modulus);
                let x3 = residue_multiply(lambda, lambda, modulus) ^ lambda ^ x1 ^ x2;
                let y3 = residue_multiply(lambda, x1 ^ x3, modulus) ^ x3 ^ y1;
                ExtensionTargetPoint::Affine { x: x3, y: y3 }
            }
        }
    }
}

fn trace_place_image(place: Place) -> TargetPoint {
    let modulus = place.polynomial;
    let alpha = remainder(X, modulus);
    let target_x = residue_power(alpha, 3, modulus);
    let target_y = residue_multiply(alpha, place.branch, modulus) ^ 1;
    let left = residue_multiply(target_y, target_y, modulus)
        ^ residue_multiply(target_x, target_y, modulus);
    let right = residue_multiply(
        residue_multiply(target_x, target_x, modulus),
        target_x,
        modulus,
    ) ^ 1;
    assert_eq!(left, right);
    let mut sum = ExtensionTargetPoint::Infinity;
    let mut conjugate = ExtensionTargetPoint::Affine {
        x: target_x,
        y: target_y,
    };
    for _ in 0..place.degree() {
        sum = extension_target_add(sum, conjugate, modulus);
        conjugate = match conjugate {
            ExtensionTargetPoint::Infinity => ExtensionTargetPoint::Infinity,
            ExtensionTargetPoint::Affine { x, y } => ExtensionTargetPoint::Affine {
                x: residue_multiply(x, x, modulus),
                y: residue_multiply(y, y, modulus),
            },
        };
    }
    match sum {
        ExtensionTargetPoint::Infinity => TargetPoint::Infinity,
        ExtensionTargetPoint::Affine { x, y } => {
            assert!(x < 2 && y < 2);
            TargetPoint::Affine {
                x: x as u8,
                y: y as u8,
            }
        }
    }
}

fn independent_pushforward(vector: &[u8], places: &[Place]) -> TargetPoint {
    let mut result = TargetPoint::Infinity;
    for (multiplicity, place) in vector.iter().zip(places) {
        let component = trace_place_image(*place);
        for _ in 0..*multiplicity {
            result = target_add(result, component);
        }
    }
    result
}

fn independent_pullback(point: TargetPoint) -> DivisorKey {
    match point {
        TargetPoint::Infinity => DivisorKey { u: 1, v: 0 },
        TargetPoint::Affine { x: 0, y: 1 } => DivisorKey { u: X, v: 0 },
        TargetPoint::Affine { x: 1, y } => DivisorKey {
            u: 0b1001, // u^3+1
            v: if y == 0 { H } else { 0 },
        },
        point => panic!("unexpected target point {point:?}"),
    }
}

fn f2_element(value: u8) -> F2mElement {
    F2mElement::from_biguint(&BigUint::from(value), 1)
}

fn production_polynomial(polynomial: Poly) -> F2mPoly {
    let coefficients = match degree(polynomial) {
        None => Vec::new(),
        Some(d) => (0..=d)
            .map(|index| f2_element(((polynomial >> index) & 1) as u8))
            .collect(),
    };
    F2mPoly::from_coeffs(coefficients, 1)
}

fn production_divisor(key: DivisorKey, cover: &OrdinaryBinaryCover) -> MumfordDivisor {
    MumfordDivisor::try_new(
        cover.source_curve(),
        production_polynomial(key.u),
        production_polynomial(key.v),
    )
    .unwrap()
}

fn production_key(divisor: &MumfordDivisor) -> DivisorKey {
    let encode = |polynomial: &F2mPoly| {
        polynomial
            .coeffs
            .iter()
            .enumerate()
            .fold(0, |bits, (index, coefficient)| {
                if coefficient.is_zero() {
                    bits
                } else {
                    bits | (1 << index)
                }
            })
    };
    DivisorKey {
        u: encode(&divisor.u),
        v: encode(&divisor.v),
    }
}

fn production_target(point: TargetPoint) -> BinaryPoint {
    match point {
        TargetPoint::Infinity => BinaryPoint::Infinity,
        TargetPoint::Affine { x, y } => BinaryPoint::Affine {
            x: f2_element(x),
            y: f2_element(y),
        },
    }
}

fn target_from_production(point: &BinaryPoint) -> TargetPoint {
    match point {
        BinaryPoint::Infinity => TargetPoint::Infinity,
        BinaryPoint::Affine { x, y } => {
            let bit = |element: &F2mElement| u8::from(!element.is_zero());
            TargetPoint::Affine {
                x: bit(x),
                y: bit(y),
            }
        }
    }
}

#[test]
fn independent_f2_principal_divisor_oracle_checks_complete_source_law_and_transfers() {
    let n1 = source_point_count(0b11); // x+1
    let n2 = source_point_count(0b111); // x^2+x+1
    let n3 = source_point_count(0b1011); // x^3+x+1
    assert_eq!((n1, n2, n3), (4, 8, 4));
    let power_sums = (
        3isize - n1 as isize,
        5isize - n2 as isize,
        9isize - n3 as isize,
    );
    let c1 = -power_sums.0;
    let c2 = -(power_sums.1 + c1 * power_sums.0) / 2;
    let c3 = -(power_sums.2 + c1 * power_sums.1 + c2 * power_sums.0) / 3;
    assert_eq!((c1, c2, c3), (1, 2, 0));
    // L(T)=1+T+2T^2+4T^4+4T^5+8T^6, hence #Jac(F2)=L(1)=20.
    let jacobian_order = 1 + c1 + c2 + c3 + 2 * c2 + 4 * c1 + 8;
    assert_eq!(jacobian_order, 20);

    let irreducible = irreducibles();
    let places = enumerate_places(&irreducible);
    let divisors = enumerate_divisors();
    let vectors: Vec<_> = divisors
        .iter()
        .map(|divisor| place_vector(*divisor, &places, &irreducible))
        .collect();
    let index_by_key: BTreeMap<_, _> = divisors
        .iter()
        .copied()
        .enumerate()
        .map(|(index, divisor)| (divisor, index))
        .collect();
    let involution_vectors: Vec<_> = divisors
        .iter()
        .map(|divisor| {
            let involution = divisor.involution();
            vectors[index_by_key[&involution]].clone()
        })
        .collect();
    let principal = principal_zero_vectors(&places);
    let table = build_group_table(&divisors, &vectors, &involution_vectors, &principal);
    let inverses = inverse_indices(&table);

    for left in 0..divisors.len() {
        assert_eq!(table[0][left], left);
        assert_eq!(table[left][0], left);
        assert_eq!(table[left][inverses[left]], 0);
        assert_eq!(inverses[left], index_by_key[&divisors[left].involution()]);
        for right in 0..divisors.len() {
            assert_eq!(table[left][right], table[right][left]);
            for third in 0..divisors.len() {
                assert_eq!(
                    table[table[left][right]][third], table[left][table[right][third]],
                    "associativity ({left},{right},{third})"
                );
            }
        }
    }

    let mut repeated_support = 0usize;
    let mut nonsplit_support = 0usize;
    for vector in &vectors {
        repeated_support += usize::from(vector.iter().any(|multiplicity| *multiplicity > 1));
        nonsplit_support += usize::from(
            vector
                .iter()
                .zip(&places)
                .any(|(multiplicity, place)| *multiplicity > 0 && place.degree() > 1),
        );
    }
    assert_eq!(repeated_support, 6);
    assert_eq!(nonsplit_support, 8);
    let shared_support = (0..divisors.len())
        .flat_map(|left| (0..divisors.len()).map(move |right| (left, right)))
        .filter(|(left, right)| {
            vectors[*left]
                .iter()
                .zip(&vectors[*right])
                .any(|(a, b)| *a > 0 && *b > 0)
        })
        .count();
    let conjugate_support = (0..divisors.len())
        .flat_map(|left| (0..divisors.len()).map(move |right| (left, right)))
        .filter(|(left, right)| {
            vectors[*left]
                .iter()
                .zip(&involution_vectors[*right])
                .any(|(a, b)| *a > 0 && *b > 0)
        })
        .count();
    assert_eq!(shared_support, 165);
    assert_eq!(conjugate_support, 165);

    let modulus = IrreduciblePoly {
        degree: 1,
        low_terms: vec![0],
    };
    let cover = OrdinaryBinaryCover::new(1, modulus, f2_element(0), f2_element(1)).unwrap();
    let production: Vec<_> = divisors
        .iter()
        .map(|divisor| production_divisor(*divisor, &cover))
        .collect();

    for left in 0..divisors.len() {
        assert_eq!(
            production_key(&production[left].neg_fast(cover.source_curve())),
            divisors[inverses[left]]
        );
        for scalar in 0..=40 {
            assert_eq!(
                production_key(
                    &production[left].scalar_mul_fast(&BigUint::from(scalar), cover.source_curve())
                ),
                divisors[scalar_index(&table, left, scalar)]
            );
        }
        for right in 0..divisors.len() {
            assert_eq!(
                production_key(
                    &production[left].add_fast(&production[right], cover.source_curve())
                ),
                divisors[table[left][right]],
                "production sum ({left},{right})"
            );
        }
    }

    let target_points: Vec<_> = std::iter::once(TargetPoint::Infinity)
        .chain((0..=1).flat_map(|x| {
            (0..=1)
                .map(move |y| TargetPoint::Affine { x, y })
                .filter(|point| target_is_on_curve(*point))
        }))
        .collect();
    assert_eq!(target_points.len(), 4);
    let images: Vec<_> = vectors
        .iter()
        .map(|vector| independent_pushforward(vector, &places))
        .collect();
    let mut fiber_counts = BTreeMap::new();
    for image in &images {
        *fiber_counts.entry(*image).or_insert(0usize) += 1;
    }
    assert_eq!(fiber_counts.len(), 4);
    assert!(
        fiber_counts.values().all(|count| *count == 5),
        "unequal independent transfer fibers: {fiber_counts:?}"
    );

    for (index, divisor) in production.iter().enumerate() {
        let actual = cover.pushforward_arbitrary_jacobian_class(divisor).unwrap();
        assert_eq!(target_from_production(&actual), images[index]);
        for right in 0..divisors.len() {
            assert_eq!(
                images[table[index][right]],
                target_add(images[index], images[right]),
                "independent pushforward additivity ({index},{right})"
            );
        }
    }

    for point in target_points {
        let pullback = independent_pullback(point);
        let pullback_index = index_by_key[&pullback];
        assert_eq!(images[pullback_index], target_scalar(point, 3));
        let production_pullback = cover
            .pullback_point_class(&production_target(point))
            .unwrap();
        assert_eq!(production_key(&production_pullback), pullback);
        let certificate = cover
            .pull_push_composition_certificate(&production_target(point))
            .unwrap();
        assert!(certificate.identity_holds);
        assert_eq!(
            target_from_production(&certificate.formal_pushforward),
            target_scalar(point, 3)
        );
    }

    // e=phi^*phi_* satisfies e^2=[3]e on all 20 source classes. This is the
    // source-side transfer composition check; it uses only the independent
    // group table and independently derived map values.
    for index in 0..divisors.len() {
        let e = index_by_key[&independent_pullback(images[index])];
        let e_squared = index_by_key[&independent_pullback(images[e])];
        assert_eq!(e_squared, scalar_index(&table, e, 3));
    }
}
