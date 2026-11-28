//! Development correctness checks on one fixed 7-bit curve. No timed p31
//! experiment is executed by this test suite.

use prime31_aux_matched::bases::{
    build_base, build_plain_control, independent_add, independent_scalar, polynomial_from_roots,
    prime31_fixture, verify_base_definition, verify_target_fixture, BaseDefinition, BaseFamily,
    CurveDefinition, GroupWork, PointDefinition,
};
use prime31_aux_matched::{CountedGroup, GroupOps, PrimeCurve, PrimePoint};

fn fixture() -> (PrimeCurve, u64, PrimePoint) {
    (
        PrimeCurve { p: 127, a: 1, b: 7 },
        109,
        PrimePoint::affine(1, 3),
    )
}

fn enumerate(curve: &PrimeCurve) -> Vec<PrimePoint> {
    let mut result = vec![PrimePoint::INFINITY];
    for x in 0..curve.p {
        for y in 0..curve.p {
            if y * y % curve.p == (x * x % curve.p * x + curve.a * x + curve.b) % curve.p {
                result.push(PrimePoint::affine(x, y));
            }
        }
    }
    result
}

fn evaluate(poly: &[u64], x: u64, p: u64) -> u64 {
    poly.iter().rev().fold(0, |acc, &a| {
        ((acc as u128 * x as u128 + a as u128) % p as u128) as u64
    })
}

fn build(family: BaseFamily) -> BaseDefinition {
    let (curve, r, _) = fixture();
    build_base(&curve, r, 2, 0, family, 32).expect("fixed toy fiber")
}

#[test]
fn fixed_curve_order_is_exhaustively_counted_and_bad_fixture_rejected() {
    let (curve, r, generator) = fixture();
    assert_eq!(enumerate(&curve).len() as u64, r);
    verify_target_fixture(&curve, r, generator).unwrap();
    assert!(verify_target_fixture(&curve, 107, generator).is_err());
    assert!(verify_target_fixture(&curve, r, PrimePoint::INFINITY).is_err());
    assert!(verify_target_fixture(&PrimeCurve { p: 127, a: 0, b: 0 }, r, generator).is_err());
    assert!(build_base(&curve, u64::MAX, 2, 0, BaseFamily::Ordinary, 1).is_err());
}

#[test]
fn existing_prime31_fixture_has_an_independent_order_certificate_only() {
    let (curve, r, generator) = prime31_fixture();
    verify_target_fixture(&curve, r, generator).unwrap();
    assert_ne!((curve.p + 1) % r, 0);
    assert_ne!(curve.p + 1 - r, 1);
    // This verifies a public fixture, without building a p31 base or solving DLP.
}

#[test]
fn whole_fibers_and_paired_controls_have_exact_point_and_polynomial_inventories() {
    let (curve, r, _) = fixture();
    for family in [
        BaseFamily::Ordinary,
        BaseFamily::SsJ1728,
        BaseFamily::Dickson,
    ] {
        let base = build(family);
        verify_base_definition(&base).unwrap();
        assert_eq!(base.v.len(), 4);
        assert_eq!(base.columns + base.unusable_x_count, base.v.len());
        let field_roots: Vec<u64> = (0..curve.p)
            .filter(|&x| evaluate(&base.certificate.fiber_polynomial, x, curve.p) == 0)
            .collect();
        assert_eq!(field_roots, base.v);
        let field_target_roots: Vec<u64> = (0..curve.p)
            .filter(|&x| evaluate(&base.certificate.target_membership_polynomial, x, curve.p) == 0)
            .collect();
        assert_eq!(field_target_roots, base.usable_x);
        let fb = base.factor_base().unwrap();
        assert_eq!(fb.columns, base.columns);
        assert_eq!(fb.points.len(), 2 * base.columns);
        for i in 0..fb.points.len() {
            let j = fb.neg_index[i];
            assert_eq!(fb.points[j], curve.neg(fb.points[i]));
            assert_eq!(fb.col_of[i], fb.col_of[j]);
            assert_eq!((fb.coef_of[i] + fb.coef_of[j]) % r, 0);
        }
        let plain = build_plain_control(&curve, r, base.columns, 1, curve.p).unwrap();
        assert_eq!(plain.columns, base.columns);
        assert_eq!(plain.v.len(), base.columns);
        assert_eq!(plain.unusable_x_count, 0);
        assert!(plain.tower.is_none());
        verify_base_definition(&plain).unwrap();
        if let Some(aux) = &base.auxiliary {
            assert_eq!(aux.order == curve.p + 1, family == BaseFamily::SsJ1728);
        }
    }
}

#[test]
fn generation_is_deterministic_and_serialized_definitions_reverify() {
    for family in [
        BaseFamily::Ordinary,
        BaseFamily::SsJ1728,
        BaseFamily::Dickson,
    ] {
        let a = build(family);
        let b = build(family);
        assert_eq!(
            serde_json::to_value(&a).unwrap(),
            serde_json::to_value(&b).unwrap()
        );
        let decoded: BaseDefinition =
            serde_json::from_str(&serde_json::to_string(&a).unwrap()).unwrap();
        verify_base_definition(&decoded).unwrap();
        assert!(decoded.cost.native_operation_counts_partial);
        assert_eq!(
            decoded.cost.attempts.len() as u64,
            decoded.cost.attempted_seed_builds
        );
        assert_eq!(
            decoded.cost.rejected_seed_builds + 1,
            decoded.cost.attempted_seed_builds
        );
    }
}

#[test]
fn corrupted_root_point_polynomial_kernel_and_transition_are_rejected() {
    let original = build(BaseFamily::SsJ1728);
    let mut corrupt = original.clone();
    corrupt.v.pop();
    assert!(verify_base_definition(&corrupt).is_err());
    let mut corrupt = original.clone();
    corrupt.v[1] = corrupt.v[0];
    assert!(verify_base_definition(&corrupt).is_err());
    let mut corrupt = original.clone();
    corrupt.canonical_points[0].y = (corrupt.canonical_points[0].y + 1) % corrupt.curve.p;
    assert!(verify_base_definition(&corrupt).is_err());
    let mut corrupt = original.clone();
    corrupt.certificate.fiber_polynomial[0] ^= 1;
    assert!(verify_base_definition(&corrupt).is_err());
    let mut corrupt = original.clone();
    corrupt.auxiliary.as_mut().unwrap().kernel_generator =
        PointDefinition::from(PrimePoint::INFINITY);
    assert!(verify_base_definition(&corrupt).is_err());
    let mut corrupt = original.clone();
    corrupt.auxiliary.as_mut().unwrap().steps[0].tau += 1;
    assert!(verify_base_definition(&corrupt).is_err());
    let mut corrupt = original.clone();
    corrupt.family = BaseFamily::Ordinary;
    assert!(verify_base_definition(&corrupt).is_err());
    let mut corrupt = original;
    let removed = corrupt.usable_x.pop().unwrap();
    corrupt.canonical_points.pop();
    corrupt.columns -= 1;
    corrupt.unusable_x_count += 1;
    corrupt.certificate.target_membership_polynomial =
        polynomial_from_roots(&corrupt.usable_x, corrupt.curve.p);
    assert!(corrupt.v.contains(&removed));
    assert!(
        verify_base_definition(&corrupt).is_err(),
        "arbitrary truncation must not masquerade as the complete lift"
    );
}

#[test]
fn bounded_failures_preserve_attempts_and_plain_scan_cost() {
    let (curve, r, _) = fixture();
    // Production constructor refuses 2^6 > p/4. Its errors are recorded once
    // per allowed invocation, without starting any large search.
    let failure = build_base(&curve, r, 6, 10, BaseFamily::Ordinary, 3).unwrap_err();
    assert_eq!(failure.cost.attempted_seed_builds, 3);
    assert_eq!(failure.cost.rejected_seed_builds, 3);
    assert_eq!(
        failure
            .cost
            .attempts
            .iter()
            .map(|a| a.seed)
            .collect::<Vec<_>>(),
        vec![10, 11, 12]
    );
    assert!(failure
        .cost
        .attempts
        .iter()
        .all(|a| a.disposition.starts_with("constructor_error:")));
    let failure = build_base(&curve, r, 6, 0, BaseFamily::Dickson, 3).unwrap_err();
    assert_eq!(
        failure.cost.attempted_seed_builds, 1,
        "coordinate shifts cannot fix splitting"
    );
    let failure = build_plain_control(&curve, r, 4, 0, 1).unwrap_err();
    assert_eq!(failure.cost.coordinate_trials, 1);
    assert_eq!(failure.cost.target_legendre_calls, 1);
}

#[test]
fn independent_affine_verifier_matches_every_small_curve_pair() {
    let (curve, r, _) = fixture();
    let independent_curve = CurveDefinition::from(&curve);
    let points = enumerate(&curve);
    for &a in &points {
        for &b in &points {
            let actual = curve.add(&mut GroupOps::default(), a, b);
            let independent = independent_add(&independent_curve, a.into(), b.into());
            assert_eq!(actual, independent.production(), "a={a:?} b={b:?}");
        }
        assert!(independent_scalar(&independent_curve, a.into(), r).infinity);
    }
}

#[test]
fn serialized_group_work_preserves_production_field_counters() {
    let (curve, _, generator) = fixture();
    let mut ops = GroupOps::default();
    curve.mul(&mut ops, generator, 31);
    let work = GroupWork::from_ops(&ops);
    assert_eq!(work.field_muls, ops.field_ops.muls);
    assert_eq!(work.field_sqrs, ops.field_ops.sqrs);
    assert_eq!(work.field_invs, ops.field_ops.invs);
    assert!(work.field_invs > 0);
    let json = serde_json::to_value(work).unwrap();
    assert!(json.get("field_muls").is_some());
    assert!(json.get("field_invs").is_some());
    assert!(json.get("montgomery_reductions").is_some());
}
