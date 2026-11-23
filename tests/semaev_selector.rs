//! Independent mechanism checks for the split-selector extractor.
//!
//! Frozen scope: `research/fass_prime31_20261122/selector/PLAN.md`.
//! The references use a Sylvester determinant, explicit polynomial products,
//! exhaustive S3 evaluation, and public group addition. They do not call the
//! candidate recurrence, candidate support finder, or candidate pair extractor.
//! Existing field and point arithmetic is reused; these tests are not an
//! independent certification of that arithmetic or an independent point count.

use std::collections::BTreeSet;

use crypto_lib::cryptanalysis::pkm_smooth_tower::{is_prime, powm, sqrtm, Curve, Pt};
use crypto_lib::cryptanalysis::semaev_prime_poly::{resultant_formal, Curve as PolynomialCurve, Fp};
use crypto_lib::cryptanalysis::semaev_selector::{
    lift_selector_pairs, quadratic_selector_resultant, selector_coordinate_pairs, SelectorError,
    SelectorExtraction, SignedPair, MAX_SELECTOR_DEGREE,
};

const PRIME31: Curve = Curve {
    p: 2_147_483_647,
    a: 847_514_846,
    b: 2_087_444_724,
};
const PRIME31_ORDER: u64 = 2_147_446_831;
const PRIME31_GENERATOR: (u64, u64) = (625_037_715, 2_100_627_251);

fn trim_reference(mut polynomial: Vec<u64>) -> Vec<u64> {
    while polynomial.len() > 1 && polynomial.last() == Some(&0) {
        polynomial.pop();
    }
    polynomial
}

fn multiply_reference(p: u64, left: &[u64], right: &[u64]) -> Vec<u64> {
    let mut product = vec![0; left.len() + right.len() - 1];
    for (i, &a) in left.iter().enumerate() {
        for (j, &b) in right.iter().enumerate() {
            product[i + j] = ((product[i + j] as u128 + a as u128 * b as u128) % p as u128) as u64;
        }
    }
    trim_reference(product)
}

fn reduce_selector_reference(p: u64, polynomial: &[u64], d: usize) -> Vec<u64> {
    let mut reduced = vec![0; d];
    for (i, &coefficient) in polynomial.iter().enumerate() {
        reduced[i % d] = (reduced[i % d] + coefficient) % p;
    }
    trim_reference(reduced)
}

fn evaluate_reference(p: u64, polynomial: &[u64], x: u64) -> u64 {
    polynomial.iter().rev().fold(0, |value, &coefficient| {
        ((value as u128 * x as u128 + coefficient as u128) % p as u128) as u64
    })
}

fn selector_polynomial(p: u64, d: usize) -> Vec<u64> {
    let mut selector = vec![0; d + 1];
    selector[0] = p - 1;
    selector[d] = 1;
    selector
}

/// Generate the known split selector independently of polynomial factorization.
/// The primitive-root search has a fixed finite bound, including on Prime31.
fn selector_roots_reference(p: u64, d: usize) -> Vec<u64> {
    assert!(d > 0 && (p - 1) % d as u64 == 0);
    if d == 1 {
        return vec![1];
    }
    let mut remaining = d as u64;
    let mut prime_divisors = Vec::new();
    let mut divisor = 2;
    while divisor * divisor <= remaining {
        if remaining % divisor == 0 {
            prime_divisors.push(divisor);
            while remaining % divisor == 0 {
                remaining /= divisor;
            }
        }
        divisor += 1;
    }
    if remaining > 1 {
        prime_divisors.push(remaining);
    }
    let generator = (2..=1_024.min(p - 1))
        .map(|candidate| powm(candidate, (p - 1) / d as u64, p))
        .find(|&candidate| {
            powm(candidate, d as u64, p) == 1
                && prime_divisors
                    .iter()
                    .all(|&prime| powm(candidate, d as u64 / prime, p) != 1)
        })
        .expect("the bounded reference generator search must cover this frozen fixture");
    let mut roots = Vec::with_capacity(d);
    let mut value = 1;
    for _ in 0..d {
        roots.push(value);
        value = ((value as u128 * generator as u128) % p as u128) as u64;
    }
    assert_eq!(value, 1);
    roots.sort_unstable();
    roots.dedup();
    assert_eq!(roots.len(), d);
    assert!(roots.iter().all(|&root| powm(root, d as u64, p) == 1));
    roots
}

fn coordinate_oracle(curve: &Curve, target_x: u64, roots: &[u64]) -> Vec<(u64, u64)> {
    let independent_formula = PolynomialCurve::new(curve.p, curve.a, curve.b);
    let mut pairs = Vec::new();
    for &left in roots {
        for &right in roots {
            if independent_formula.s3(left, right, target_x) == 0 {
                pairs.push((left, right));
            }
        }
    }
    pairs
}

fn small_selector_points(curve: &Curve, roots: &[u64]) -> Vec<(u64, u64)> {
    assert!(
        curve.p <= 31,
        "the exhaustive ordinate oracle is small-field only"
    );
    let mut points = Vec::new();
    for &x in roots {
        for y in 0..curve.p {
            if (y * y) % curve.p == curve.rhs(x) {
                points.push((x, y));
            }
        }
    }
    points
}

fn exhaustive_group_oracle(curve: &Curve, target: Pt, points: &[(u64, u64)]) -> Vec<SignedPair> {
    let mut pairs = Vec::new();
    for &left in points {
        for &right in points {
            if curve.add(Some(left), Some(right)) == target {
                pairs.push(SignedPair { left, right });
            }
        }
    }
    pairs.sort_unstable();
    pairs.dedup();
    pairs
}

fn check_support(
    curve: &Curve,
    roots: &[u64],
    extraction: &SelectorExtraction,
    expected: &[(u64, u64)],
) {
    assert_eq!(extraction.selector_degree, roots.len());
    assert!(!extraction.resultant.is_empty());
    assert!(extraction.resultant.len() <= roots.len());
    assert!(extraction
        .resultant
        .iter()
        .all(|&coefficient| coefficient < curve.p));
    assert_eq!(extraction.support_gcd.last(), Some(&1));
    let support: BTreeSet<_> = expected.iter().map(|&(x, _)| x).collect();
    assert_eq!(extraction.support_gcd.len(), support.len() + 1);
    for &x in roots {
        assert_eq!(
            evaluate_reference(curve.p, &extraction.resultant, x) == 0,
            support.contains(&x),
            "resultant support at p={}, d={}, x={x}",
            curve.p,
            roots.len()
        );
        assert_eq!(
            evaluate_reference(curve.p, &extraction.support_gcd, x) == 0,
            support.contains(&x),
            "gcd support at p={}, d={}, x={x}",
            curve.p,
            roots.len()
        );
    }
}

#[test]
fn constant_quadratics_match_all_formal_sylvester_specializations() {
    let mut cases = 0usize;
    for p in [5, 7] {
        let f = Fp { p };
        for d in 1..=6 {
            let selector = selector_polynomial(p, d);
            for a in 0..p {
                for b in 0..p {
                    for c in 0..p {
                        // The second formal degree remains two at a=0.
                        // This independently checks specialization and signs.
                        let expected = resultant_formal(f, &selector, d, &[c, b, a], 2);
                        let actual = quadratic_selector_resultant(f, &[a], &[b], &[c], d)
                            .expect("valid algebraic identity fixture");
                        assert_eq!(actual, vec![expected], "p={p}, d={d}, a={a}, b={b}, c={c}");
                        cases += 1;
                    }
                }
            }
        }
    }
    assert_eq!(cases, 2_808);
    println!("formal_sylvester coefficient_specializations={cases}");
}

#[test]
fn polynomial_coefficients_match_explicit_selector_product() {
    let mut cases = 0usize;
    for p in [17, 31] {
        let f = Fp { p };
        // The first leading coefficient is (X-1)^2, a zero divisor in
        // every selector quotient below. Linear, constant and zero
        // quadratics are separate cases, and degrees above d are retained.
        let fixtures = [
            (vec![1, p - 2, 1], vec![0, 3, 1], vec![5, 2, 0, 1]),
            (vec![0], vec![p - 1, 1], vec![1, 2]),
            (vec![0], vec![0], vec![p - 1, 1]),
            (vec![0], vec![0], vec![0]),
            (vec![3, 0, 2, 1], vec![1, 0, 4], vec![0, 1, 0, 2]),
        ];
        for d in (1..=12).filter(|&d| (p - 1) % d as u64 == 0) {
            let roots = selector_roots_reference(p, d);
            for (fixture, (a, b, c)) in fixtures.iter().enumerate() {
                let mut product = vec![1];
                for &y in &roots {
                    let mut factor = vec![0; a.len().max(b.len()).max(c.len())];
                    for (i, coefficient) in factor.iter_mut().enumerate() {
                        let ay2 = a.get(i).copied().unwrap_or(0) as u128 * y as u128 * y as u128;
                        let by = b.get(i).copied().unwrap_or(0) as u128 * y as u128;
                        let cc = c.get(i).copied().unwrap_or(0) as u128;
                        *coefficient = ((ay2 + by + cc) % p as u128) as u64;
                    }
                    product = multiply_reference(p, &product, &factor);
                }
                let expected = reduce_selector_reference(p, &product, d);
                let actual = quadratic_selector_resultant(f, a, b, c, d).unwrap();
                assert_eq!(
                    actual, expected,
                    "p={p}, d={d}, polynomial_fixture={fixture}"
                );
                cases += 1;
            }
        }
    }
    assert_eq!(cases, 50);
    println!("explicit_selector_product polynomial_specializations={cases}");
}

#[test]
fn complete_small_field_coordinate_and_signed_sets_match_independent_oracles() {
    let mut coordinate_cases = 0usize;
    let mut empty_coordinate_cases = 0usize;
    let mut coordinate_pairs = 0usize;
    let mut rational_targets = 0usize;
    let mut signed_pairs = 0usize;
    for curve in [
        Curve { p: 7, a: 1, b: 0 },
        Curve { p: 13, a: 2, b: 3 },
        Curve { p: 17, a: 2, b: 2 },
        Curve { p: 31, a: 2, b: 3 },
    ] {
        assert!(!curve.is_singular());
        for d in (1..=12).filter(|&d| (curve.p - 1) % d as u64 == 0) {
            let roots = selector_roots_reference(curve.p, d);
            let points = small_selector_points(&curve, &roots);
            for target_x in 0..curve.p {
                let seed = (0x5e1e_c70f_u64 ^ (curve.p << 16) ^ ((d as u64) << 8) ^ target_x) | 1;
                let actual = selector_coordinate_pairs(&curve, target_x, d, seed).unwrap();
                let expected = coordinate_oracle(&curve, target_x, &roots);
                assert_eq!(
                    actual.coordinate_pairs, expected,
                    "p={}, d={d}, t={target_x}",
                    curve.p
                );
                check_support(&curve, &roots, &actual, &expected);
                coordinate_cases += 1;
                empty_coordinate_cases += usize::from(expected.is_empty());
                coordinate_pairs += expected.len();
                // Every ordinate is checked independently; no target root
                // selection or planted witness restricts this comparison.
                for target_y in 0..curve.p {
                    if target_y * target_y % curve.p != curve.rhs(target_x) {
                        continue;
                    }
                    let target = Some((target_x, target_y));
                    let lifted =
                        lift_selector_pairs(&curve, target, &actual.coordinate_pairs, None)
                            .unwrap();
                    let expected_lifted = exhaustive_group_oracle(&curve, target, &points);
                    assert_eq!(
                        lifted, expected_lifted,
                        "p={}, d={d}, target=({target_x},{target_y})",
                        curve.p
                    );
                    rational_targets += 1;
                    signed_pairs += lifted.len();
                }
            }
        }
    }
    assert_eq!(coordinate_cases, 360);
    assert!(empty_coordinate_cases > 0);
    println!(
        "small_fields coordinate_cases={coordinate_cases} empty_cases={empty_coordinate_cases} \
         coordinate_pairs={coordinate_pairs} rational_targets={rational_targets} signed_pairs={signed_pairs}"
    );
}

#[test]
fn two_torsion_target_rejects_extension_only_coordinate_pairs() {
    let curve = Curve { p: 7, a: 1, b: 0 };
    let target = Some((0, 0));
    let extracted = selector_coordinate_pairs(&curve, 0, 3, 0xfa55_0001).unwrap();
    assert_eq!(extracted.coordinate_pairs, vec![(1, 1), (2, 4), (4, 2)]);
    assert_eq!(curve.rhs(2), 3);
    assert_eq!(curve.rhs(4), 5);
    assert!((0..7).all(|y| y * y % 7 != 3 && y * y % 7 != 5));
    assert!(lift_selector_pairs(&curve, target, &[(2, 4), (4, 2)], None)
        .unwrap()
        .is_empty());
    let expected = vec![
        SignedPair {
            left: (1, 3),
            right: (1, 3),
        },
        SignedPair {
            left: (1, 4),
            right: (1, 4),
        },
    ];
    let lifted = lift_selector_pairs(&curve, target, &extracted.coordinate_pairs, None).unwrap();
    assert_eq!(lifted, expected);
    let repeated: Vec<_> = extracted
        .coordinate_pairs
        .iter()
        .copied()
        .chain(extracted.coordinate_pairs.iter().copied())
        .collect();
    assert_eq!(
        lift_selector_pairs(&curve, target, &repeated, None).unwrap(),
        expected
    );
    println!("two_torsion coordinate_pairs=3 extension_only_pairs_rejected=2 signed_pairs=2");
}

#[test]
fn vanishing_quadratic_leading_coefficient_retains_the_finite_doubling_branch() {
    let curve = Curve { p: 7, a: 1, b: 0 };
    let target = Some((3, 3));
    assert!(curve.on_curve(target));
    let doubled = curve.add(target, target).unwrap();
    assert_eq!(doubled, (1, 4));
    let extracted = selector_coordinate_pairs(&curve, 3, 6, 0xfa55_0002).unwrap();
    assert!(extracted.coordinate_pairs.contains(&(3, 1)));
    let witness = SignedPair {
        left: curve.neg(target).unwrap(),
        right: doubled,
    };
    assert_eq!(curve.add(Some(witness.left), Some(witness.right)), target);
    let signed = lift_selector_pairs(&curve, target, &extracted.coordinate_pairs, Some(8)).unwrap();
    assert!(signed.contains(&witness));
    assert_eq!(
        signed,
        exhaustive_group_oracle(
            &curve,
            target,
            &small_selector_points(&curve, &selector_roots_reference(7, 6))
        )
    );
    println!(
        "vanishing_leading_coefficient target=(3,3) finite_root=1 signed_pairs={}",
        signed.len()
    );
}

#[test]
fn algebraic_api_refuses_invalid_fields_degrees_and_coefficients() {
    let mut refusals = 0usize;
    for p in [0, 1, 2, 4, 9, 15, 1u64 << 32, (1u64 << 32) + 15] {
        assert_eq!(
            quadratic_selector_resultant(Fp { p }, &[1], &[1], &[1], 1),
            Err(SelectorError::UnsupportedField)
        );
        refusals += 1;
    }
    let f = Fp { p: 17 };
    for d in [0, MAX_SELECTOR_DEGREE + 1] {
        assert_eq!(
            quadratic_selector_resultant(f, &[1], &[1], &[1], d),
            Err(SelectorError::UnsupportedDegree)
        );
        refusals += 1;
    }
    for invalid in [Vec::new(), vec![17], vec![1; MAX_SELECTOR_DEGREE + 2]] {
        for slot in 0..3 {
            let mut coefficients: [&[u64]; 3] = [&[1], &[1], &[1]];
            coefficients[slot] = &invalid;
            assert_eq!(
                quadratic_selector_resultant(
                    f,
                    coefficients[0],
                    coefficients[1],
                    coefficients[2],
                    2
                ),
                Err(SelectorError::InvalidPolynomial)
            );
            refusals += 1;
        }
    }
    assert_eq!(refusals, 19);
    println!("algebraic_guard refusals={refusals}");
}

#[test]
fn coordinate_api_refuses_invalid_curves_coordinates_and_unsplit_selectors() {
    let curve = Curve { p: 17, a: 2, b: 2 };
    let cases = [
        (
            Curve { p: 2, a: 0, b: 1 },
            1,
            1,
            SelectorError::UnsupportedField,
        ),
        (
            Curve { p: 15, a: 2, b: 2 },
            1,
            1,
            SelectorError::UnsupportedField,
        ),
        (
            Curve {
                p: 1u64 << 32,
                a: 2,
                b: 2,
            },
            1,
            1,
            SelectorError::UnsupportedField,
        ),
        (
            Curve { p: 17, a: 0, b: 0 },
            1,
            1,
            SelectorError::SingularCurve,
        ),
        (
            Curve { p: 17, a: 17, b: 2 },
            1,
            1,
            SelectorError::NoncanonicalInput,
        ),
        (
            Curve { p: 17, a: 2, b: 17 },
            1,
            1,
            SelectorError::NoncanonicalInput,
        ),
        (curve, 17, 1, SelectorError::NoncanonicalInput),
        (curve, 1, 0, SelectorError::UnsupportedDegree),
        (
            curve,
            1,
            MAX_SELECTOR_DEGREE + 1,
            SelectorError::UnsupportedDegree,
        ),
        (curve, 1, 3, SelectorError::SelectorDoesNotSplit),
    ];
    for (invalid_curve, t, d, error) in &cases {
        assert_eq!(
            selector_coordinate_pairs(invalid_curve, *t, *d, 17),
            Err(error.clone())
        );
    }
    println!("coordinate_guard refusals={}", cases.len());
}

#[test]
fn lifting_refuses_invalid_targets_coordinates_and_order_constraints() {
    let curve = Curve { p: 17, a: 2, b: 2 };
    let target = Some((5, 1));
    assert!(curve.on_curve(target));
    assert_eq!(curve.mul(19, target), None);
    assert_eq!(
        lift_selector_pairs(&curve, None, &[], None),
        Err(SelectorError::InvalidTarget)
    );
    assert_eq!(
        lift_selector_pairs(&curve, Some((0, 0)), &[], None),
        Err(SelectorError::InvalidTarget)
    );
    for noncanonical in [Some((17, 1)), Some((5, 17))] {
        assert_eq!(
            lift_selector_pairs(&curve, noncanonical, &[], None),
            Err(SelectorError::NoncanonicalInput)
        );
    }
    for invalid_pair in [(17, 1), (1, 17)] {
        assert_eq!(
            lift_selector_pairs(&curve, target, &[invalid_pair], None),
            Err(SelectorError::NoncanonicalInput)
        );
    }
    for invalid_order in [0, 1] {
        assert_eq!(
            lift_selector_pairs(&curve, target, &[], Some(invalid_order)),
            Err(SelectorError::InvalidOrder)
        );
    }
    assert_eq!(
        lift_selector_pairs(&curve, target, &[], Some(2)),
        Err(SelectorError::TargetOutsideOrderKernel)
    );
    assert!(lift_selector_pairs(&curve, target, &[], Some(19))
        .unwrap()
        .is_empty());
    println!("lifting_guard refusals=9 valid_empty_calls=1");
}

#[test]
fn order_annihilator_filters_summands_separately_from_the_target() {
    let curve = Curve { p: 7, a: 1, b: 0 };
    let target = Some((0, 0));
    let pairs = [(1, 1)];
    assert_eq!(curve.mul(2, target), None);
    assert_eq!(
        lift_selector_pairs(&curve, target, &pairs, None)
            .unwrap()
            .len(),
        2
    );
    assert!(lift_selector_pairs(&curve, target, &pairs, Some(2))
        .unwrap()
        .is_empty());
    assert_eq!(
        lift_selector_pairs(&curve, target, &pairs, Some(4))
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        lift_selector_pairs(&curve, target, &pairs, Some(3)),
        Err(SelectorError::TargetOutsideOrderKernel)
    );
    println!("order_annihilator unconstrained_pairs=2 order2_pairs=0 order4_pairs=2");
}

#[test]
fn registered_prime31_generator_and_declared_order_replay() {
    assert!(is_prime(PRIME31.p));
    assert!(is_prime(PRIME31_ORDER));
    assert!(!PRIME31.is_singular());
    assert!(PRIME31.on_curve(Some(PRIME31_GENERATOR)));
    assert_eq!(PRIME31.mul(PRIME31_ORDER, Some(PRIME31_GENERATOR)), None);
    println!(
        "prime31_generator p={} declared_r={} G=({},{}) order_replay=pass independent_point_count=not_performed",
        PRIME31.p, PRIME31_ORDER, PRIME31_GENERATOR.0, PRIME31_GENERATOR.1
    );
}

fn prime31_positive_fixture(d: usize) {
    let curve = PRIME31;
    let roots = selector_roots_reference(curve.p, d);
    let mut points = BTreeSet::new();
    for &x in &roots {
        if let Some(y) = sqrtm(curve.rhs(x), curve.p) {
            assert!(curve.on_curve(Some((x, y))));
            points.insert((x, y));
            points.insert((x, (curve.p - y) % curve.p));
        }
    }
    // The declared annihilator is checked on each admitted public point;
    // it is not inferred here from an unperformed independent point count.
    let admitted: BTreeSet<_> = points
        .iter()
        .copied()
        .filter(|&point| curve.mul(PRIME31_ORDER, Some(point)).is_none())
        .collect();
    let (target, chosen_pair) = admitted
        .iter()
        .flat_map(|&left| {
            admitted
                .iter()
                .map(move |&right| SignedPair { left, right })
        })
        .find_map(|pair| {
            curve
                .add(Some(pair.left), Some(pair.right))
                .map(|target| (target, pair))
        })
        .expect("each frozen selector must furnish a finite positive public target");
    assert!(curve.on_curve(Some(target)));
    assert_eq!(curve.mul(PRIME31_ORDER, Some(target)), None);
    // Only the public target coordinate reaches candidate extraction. The
    // chosen decomposition never restricts its unknown-coordinate search.
    let extracted = selector_coordinate_pairs(&curve, target.0, d, 0xfa55_0031 ^ d as u64).unwrap();
    let expected = coordinate_oracle(&curve, target.0, &roots);
    assert_eq!(
        extracted.coordinate_pairs, expected,
        "Prime31 d={d}, target={target:?}"
    );
    assert!(expected.contains(&(chosen_pair.left.0, chosen_pair.right.0)));
    check_support(&curve, &roots, &extracted, &expected);
    let actual_signed = lift_selector_pairs(
        &curve,
        Some(target),
        &extracted.coordinate_pairs,
        Some(PRIME31_ORDER),
    )
    .unwrap();
    // Public group subtraction gives every ordered pair independently of
    // the resultant and of the exhaustive coordinate-polynomial oracle.
    let expected_signed: Vec<_> = admitted
        .iter()
        .filter_map(|&left| {
            curve
                .add(Some(target), curve.neg(Some(left)))
                .and_then(|right| {
                    admitted
                        .contains(&right)
                        .then_some(SignedPair { left, right })
                })
        })
        .collect();
    assert_eq!(actual_signed, expected_signed, "Prime31 signed pairs d={d}");
    assert!(actual_signed.contains(&chosen_pair));
    for pair in &actual_signed {
        assert!(curve.on_curve(Some(pair.left)) && curve.on_curve(Some(pair.right)));
        assert_eq!(curve.add(Some(pair.left), Some(pair.right)), Some(target));
        assert_eq!(curve.mul(PRIME31_ORDER, Some(pair.left)), None);
        assert_eq!(curve.mul(PRIME31_ORDER, Some(pair.right)), None);
    }
    println!(
        "prime31_positive d={d} selector_roots={} polynomial_pair_checks={} rational_points={} \
         order_admitted_points={} target=({},{}) coordinate_pairs={} signed_pairs={}",
        roots.len(),
        d * d,
        points.len(),
        admitted.len(),
        target.0,
        target.1,
        extracted.coordinate_pairs.len(),
        actual_signed.len()
    );
}

#[test]
fn prime31_selector_degree_6_positive_public_fixture() {
    prime31_positive_fixture(6);
}

#[test]
fn prime31_selector_degree_31_positive_public_fixture() {
    prime31_positive_fixture(31);
}

#[test]
fn prime31_selector_degree_1302_positive_public_fixture() {
    prime31_positive_fixture(1302);
}

// Attempt-three additions: preserve the preceding frozen tests. These two
// cases extend Prime31 coverage beyond the common diagonal positive fixture.
fn prime31_selector_population_reference(
    roots: &[u64],
) -> (BTreeSet<(u64, u64)>, BTreeSet<(u64, u64)>) {
    let mut points = BTreeSet::new();
    for &x in roots {
        if let Some(y) = sqrtm(PRIME31.rhs(x), PRIME31.p) {
            assert!(PRIME31.on_curve(Some((x, y))));
            points.insert((x, y));
            points.insert((x, (PRIME31.p - y) % PRIME31.p));
        }
    }
    let admitted = points
        .iter()
        .copied()
        .filter(|&point| PRIME31.mul(PRIME31_ORDER, Some(point)).is_none())
        .collect();
    (points, admitted)
}

fn check_prime31_public_target_against_complete_oracles(
    roots: &[u64],
    admitted: &BTreeSet<(u64, u64)>,
    target: (u64, u64),
    seed: u64,
    label: &str,
) -> Vec<SignedPair> {
    assert!(PRIME31.on_curve(Some(target)));
    assert_eq!(PRIME31.mul(PRIME31_ORDER, Some(target)), None);
    let d = roots.len();
    let extracted = selector_coordinate_pairs(&PRIME31, target.0, d, seed).unwrap();
    let expected = coordinate_oracle(&PRIME31, target.0, roots);
    assert_eq!(
        extracted.coordinate_pairs, expected,
        "Prime31 {label}, d={d}, target={target:?}"
    );
    check_support(&PRIME31, roots, &extracted, &expected);
    let actual_signed = lift_selector_pairs(
        &PRIME31,
        Some(target),
        &extracted.coordinate_pairs,
        Some(PRIME31_ORDER),
    )
    .unwrap();
    let expected_signed: Vec<_> = admitted
        .iter()
        .filter_map(|&left| {
            PRIME31
                .add(Some(target), PRIME31.neg(Some(left)))
                .and_then(|right| {
                    admitted
                        .contains(&right)
                        .then_some(SignedPair { left, right })
                })
        })
        .collect();
    assert_eq!(actual_signed, expected_signed, "Prime31 signed {label}");
    for pair in &actual_signed {
        assert!(PRIME31.on_curve(Some(pair.left)));
        assert!(PRIME31.on_curve(Some(pair.right)));
        assert_eq!(PRIME31.add(Some(pair.left), Some(pair.right)), Some(target));
        assert_eq!(PRIME31.mul(PRIME31_ORDER, Some(pair.left)), None);
        assert_eq!(PRIME31.mul(PRIME31_ORDER, Some(pair.right)), None);
    }
    println!(
        "prime31_{label} d={d} selector_roots={} polynomial_pair_checks={} \
         order_admitted_points={} target=({},{}) coordinate_pairs={} signed_pairs={} empty={}",
        roots.len(),
        d * d,
        admitted.len(),
        target.0,
        target.1,
        extracted.coordinate_pairs.len(),
        actual_signed.len(),
        extracted.coordinate_pairs.is_empty()
    );
    actual_signed
}

#[test]
fn prime31_selector_degree_1302_distinct_abscissa_public_fixture() {
    let roots = selector_roots_reference(PRIME31.p, 1302);
    let (points, admitted) = prime31_selector_population_reference(&roots);
    let (target, chosen_pair) = admitted
        .iter()
        .flat_map(|&left| {
            admitted
                .iter()
                .map(move |&right| SignedPair { left, right })
        })
        .filter(|pair| pair.left.0 != pair.right.0)
        .find_map(|pair| {
            PRIME31
                .add(Some(pair.left), Some(pair.right))
                .map(|target| (target, pair))
        })
        .expect("the frozen d=1302 selector has two admitted distinct abscissas");
    assert_ne!(chosen_pair.left.0, chosen_pair.right.0);
    // The public target is fixed before extraction; neither the selected
    // pair nor its coordinates constrain the candidate's search domain.
    let signed = check_prime31_public_target_against_complete_oracles(
        &roots,
        &admitted,
        target,
        0xfa55_1302_0001,
        "distinct_abscissa",
    );
    assert!(signed.contains(&chosen_pair));
    assert!(signed.contains(&SignedPair {
        left: chosen_pair.right,
        right: chosen_pair.left,
    }));
    println!(
        "prime31_distinct_witness left=({},{}) right=({},{}) rational_selector_points={}",
        chosen_pair.left.0,
        chosen_pair.left.1,
        chosen_pair.right.0,
        chosen_pair.right.1,
        points.len()
    );
}

#[test]
fn prime31_selector_degree_1302_fixed_generator_target() {
    // This target is the registered public generator, fixed independently
    // of selector decomposition and before any call to the candidate.
    let target = PRIME31_GENERATOR;
    let roots = selector_roots_reference(PRIME31.p, 1302);
    let (points, admitted) = prime31_selector_population_reference(&roots);
    let signed = check_prime31_public_target_against_complete_oracles(
        &roots,
        &admitted,
        target,
        0xfa55_1302_0002,
        "fixed_generator",
    );
    // An empty set is a valid observed result and is retained by both
    // complete comparisons above; no positive-result assertion is made.
    println!(
        "prime31_fixed_generator_target rational_selector_points={} signed_pairs={}",
        points.len(),
        signed.len()
    );
}
