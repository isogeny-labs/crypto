use super::*;
use crate::prime_hyperelliptic::FpPoly;
use rand::{rngs::StdRng, SeedableRng};

fn polynomial(p: &BigUint, coefficients: &[u32]) -> FpPoly {
    FpPoly::from_coeffs(
        coefficients.iter().copied().map(BigUint::from).collect(),
        p.clone(),
    )
}

fn divisor(curve: &HyperellipticCurveP, u: &[u32], v: &[u32]) -> MumfordDivisorP {
    MumfordDivisorP::try_from_polynomials(curve, polynomial(&curve.p, u), polynomial(&curve.p, v))
        .unwrap()
}

fn genus_three_fixture() -> (HyperellipticCurveP, MumfordDivisorP, MumfordDivisorP) {
    let p = BigUint::from(3u32);
    let curve = HyperellipticCurveP::new(p.clone(), polynomial(&p, &[1, 0, 0, 2, 0, 0, 0, 1]), 3);
    let generator = divisor(&curve, &[1, 0, 1, 1], &[1, 0, 1]);
    let target = divisor(&curve, &[0, 2, 2, 1], &[2, 0, 2]);
    (curve, generator, target)
}

#[test]
fn solves_genus_three_when_generator_is_divisible_by_seven() {
    let (curve, generator, target) = genus_three_fixture();
    let order = BigUint::from(7u32);
    let preimage = divisor(&curve, &[0, 0, 1], &[1]);
    assert_eq!(
        preimage.checked_scalar_mul(&order, &curve).unwrap(),
        generator
    );
    assert!(!generator.is_identity());

    // This valid relation cannot be truncated modulo 7 before cancellation.
    let point_class = divisor(&curve, &[0, 1], &[1]);
    assert_eq!(
        point_class.scalar_mul(&BigUint::from(14u32), &curve),
        generator
            .scalar_mul(&BigUint::from(6u32), &curve)
            .add(&target, &curve)
    );

    let mut rng = StdRng::seed_from_u64(20261118);
    for smooth_degree in [None, Some(1)] {
        let config = OrderFreeIcConfig {
            smooth_degree,
            ..OrderFreeIcConfig::default()
        };
        let result =
            solve_order_free(&curve, &generator, &target, &order, config, &mut rng).unwrap();
        assert_eq!(result.scalar, BigUint::from(2u32));
        assert!(result.factor_base_size > 0);
        assert!(result.relation_trials > result.factor_base_size);
    }
}

#[test]
fn solves_genus_two_on_the_frey_ruck_fixture() {
    let p = BigUint::from(29u32);
    let curve = HyperellipticCurveP::new(p.clone(), polynomial(&p, &[4, 12, 14, 20, 16, 1]), 2);
    let generator = divisor(&curve, &[26, 3, 1], &[7, 17]);
    let target = divisor(&curve, &[19, 12, 1], &[16, 23]);
    let order = BigUint::from(7u32);
    let config = OrderFreeIcConfig {
        smooth_degree: Some(1),
        ..OrderFreeIcConfig::default()
    };
    let mut rng = StdRng::seed_from_u64(92517);
    let result = solve_order_free(&curve, &generator, &target, &order, config, &mut rng).unwrap();
    assert_eq!(result.scalar, BigUint::from(3u32));
    assert_eq!(
        generator
            .checked_scalar_mul(&result.scalar, &curve)
            .unwrap(),
        target
    );
}

#[test]
fn reports_separate_factor_base_and_relation_limits() {
    let (curve, generator, target) = genus_three_fixture();
    let order = BigUint::from(7u32);
    let mut rng = StdRng::seed_from_u64(17);
    let base_limit = OrderFreeIcConfig {
        max_base_polynomials: 2,
        ..OrderFreeIcConfig::default()
    };
    assert_eq!(
        solve_order_free(&curve, &generator, &target, &order, base_limit, &mut rng),
        Err(OrderFreeIcError::FactorBaseBudgetExceeded)
    );
    let relation_limit = OrderFreeIcConfig {
        max_relation_trials: 1,
        ..OrderFreeIcConfig::default()
    };
    assert_eq!(
        solve_order_free(
            &curve,
            &generator,
            &target,
            &order,
            relation_limit,
            &mut rng
        ),
        Err(OrderFreeIcError::RelationBudgetExceeded)
    );
}

#[test]
fn validates_subgroup_and_handles_identity_target() {
    let (curve, generator, target) = genus_three_fixture();
    let mut rng = StdRng::seed_from_u64(0);
    let config = OrderFreeIcConfig::default();
    assert_eq!(
        solve_order_free(
            &curve,
            &generator,
            &target,
            &BigUint::from(5u32),
            config,
            &mut rng
        ),
        Err(OrderFreeIcError::InvalidSubgroup)
    );
    let identity = MumfordDivisorP::try_identity(&curve).unwrap();
    let order = BigUint::from(7u32);
    assert_eq!(
        solve_order_free(&curve, &identity, &target, &order, config, &mut rng),
        Err(OrderFreeIcError::InvalidSubgroup)
    );
    assert_eq!(
        solve_order_free(&curve, &generator, &identity, &order, config, &mut rng),
        Ok(OrderFreeIcSolution {
            scalar: BigUint::zero(),
            factor_base_size: 0,
            relation_trials: 0
        })
    );
}

#[test]
fn rejects_a_field_too_wide_to_enumerate_before_curve_validation() {
    let (mut curve, generator, target) = genus_three_fixture();
    curve.p = (BigUint::one() << usize::BITS) + 3u32;
    let order = BigUint::from(7u32);
    let mut rng = StdRng::seed_from_u64(0);
    assert_eq!(
        solve_order_free(
            &curve,
            &generator,
            &target,
            &order,
            OrderFreeIcConfig::default(),
            &mut rng,
        ),
        Err(OrderFreeIcError::FactorBaseBudgetExceeded)
    );
}
