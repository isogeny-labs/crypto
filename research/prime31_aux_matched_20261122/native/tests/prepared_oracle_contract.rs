//! Bounded development contracts for once-per-cell validation and exact target
//! leaf domains. No Prime31 search, performance comparison or campaign runs.
use std::collections::BTreeSet;

use prime31_aux_matched::oracle::{
    self, DomainRoots, OracleLimits, OracleResult, OracleStatus, PreparedOracle, Presentation,
    TowerDescription,
};
use prime31_aux_matched::replay::{Curve, Point};
use prime31_aux_matched::{CountedGroup, FactorBase, GroupOps, PrimeCurve, PrimePoint};

fn fixture() -> (
    PrimeCurve,
    FactorBase<PrimePoint>,
    Vec<u64>,
    TowerDescription,
) {
    let curve = PrimeCurve { p: 17, a: 2, b: 2 };
    // x=1 does not lift on this target, while (3,+/-1) does. Both x values
    // belong to the complete algebraic fibre x + 3/x = 4. Auxiliary-curve
    // group certification is intentionally outside this small solver fixture.
    let values = vec![1, 3];
    let points = vec![PrimePoint::affine(3, 1), PrimePoint::affine(3, 16)];
    let fb = FactorBase::from_column_map(
        "prepared-oracle-p17-one-unliftable-leaf".into(),
        points,
        vec![0, 0],
        vec![1, 18],
        1,
        |q| curve.key(q),
        |q| curve.key(&curve.neg(*q)),
        |q| q.x,
    )
    .unwrap();
    let tower = TowerDescription::Velu {
        steps: vec![(0, 3)],
        terminal: 4,
    };
    (curve, fb, values, tower)
}

fn limits() -> OracleLimits {
    OracleLimits {
        timeout_ms: 20_000,
        stop_after_first: false,
        max_degree: 8,
        domain_roots: DomainRoots::Auto,
        sibling_batch: true,
        ..Default::default()
    }
}

fn rows(result: &OracleResult) -> BTreeSet<Vec<i64>> {
    result.relations.iter().map(|r| r.row.clone()).collect()
}

fn independent_rows(
    fb: &FactorBase<PrimePoint>,
    target: PrimePoint,
    m: usize,
) -> BTreeSet<Vec<i64>> {
    let curve = Curve { p: 17, a: 2, b: 2 };
    let want = if target.infinity {
        Point::IDENTITY
    } else {
        Point::affine(target.x, target.y)
    };
    let mut expected = BTreeSet::new();
    // At most 2^3=8 signed tuples in this fixed correctness fixture.
    for mut code in 0..fb.points.len().pow(m as u32) {
        let mut sum = Point::IDENTITY;
        let mut row = vec![0i64; fb.columns];
        for _ in 0..m {
            let index = code % fb.points.len();
            code /= fb.points.len();
            let q = fb.points[index];
            sum = curve.add(sum, Point::affine(q.x, q.y)).unwrap();
            row[fb.col_of[index]] += if fb.coef_of[index] == 1 { 1 } else { -1 };
        }
        if sum == want {
            expected.insert(row);
        }
    }
    expected
}

fn verify_witnesses(fb: &FactorBase<PrimePoint>, target: PrimePoint, result: &OracleResult) {
    let curve = Curve { p: 17, a: 2, b: 2 };
    let want = if target.infinity {
        Point::IDENTITY
    } else {
        Point::affine(target.x, target.y)
    };
    for relation in &result.relations {
        let mut sum = Point::IDENTITY;
        let mut row = vec![0i64; fb.columns];
        for &index in &relation.indices {
            let q = fb.points[index];
            sum = curve.add(sum, Point::affine(q.x, q.y)).unwrap();
            row[fb.col_of[index]] += if fb.coef_of[index] == 1 { 1 } else { -1 };
        }
        assert_eq!(sum, want);
        assert_eq!(row, relation.row);
    }
}

#[test]
fn prepared_complete_signed_rows_match_legacy_and_independent_reference() {
    let (curve, fb, values, tower) = fixture();
    let twice = curve.mul(&mut GroupOps::default(), fb.points[0], 2);
    let mut saw_positive = false;
    let mut saw_negative = false;
    for m in [2, 3] {
        for presentation in [
            Presentation::Dense,
            Presentation::Tower,
            Presentation::Tower4,
        ] {
            let prepared =
                PreparedOracle::prepare(&curve, 19, &fb, &values, Some(&tower), m, presentation)
                    .unwrap();
            let setup_before = serde_json::to_value(prepared.setup_report()).unwrap();
            assert_eq!(setup_before["full_membership_values"], 2);
            assert_eq!(setup_before["leaf_domain_values"], 1);
            assert_eq!(setup_before["fixed_validation_passes"], 1);
            assert_eq!(setup_before["logical_leaf_domain_payload_bytes"], 8);
            // Each of two tower edges performs seven powering products and
            // one tau/inverse product. Dense keeps the original no-tower check.
            let validation = if presentation == Presentation::Dense {
                0
            } else {
                16
            };
            assert_eq!(
                prepared.setup_report().validation_field_multiplications,
                validation
            );
            for target in [PrimePoint::INFINITY, fb.points[0], twice] {
                let old = oracle::decompose(
                    &curve,
                    19,
                    &fb,
                    &values,
                    Some(&tower),
                    target,
                    m,
                    presentation,
                    &limits(),
                    &mut GroupOps::default(),
                )
                .unwrap();
                let new = prepared
                    .decompose(target, &limits(), &mut GroupOps::default())
                    .unwrap();
                assert!(old.complete, "legacy: {old:?}");
                assert!(new.complete, "prepared: {new:?}");
                assert_eq!(new.status, old.status);
                assert_eq!(new.positive, old.positive);
                assert_eq!(rows(&new), rows(&old));
                assert_eq!(rows(&new), independent_rows(&fb, target, m));
                verify_witnesses(&fb, target, &new);
                assert_eq!(new.counters.validation_field_multiplications, 0);
                assert_eq!(old.counters.validation_field_multiplications, validation);
                assert!(old.backend_work.is_none());
                assert_eq!(
                    new.backend_work.as_ref().unwrap()["mode"],
                    "prepared_target_liftable_domains"
                );
                assert_eq!(
                    old.charts
                        .iter()
                        .map(|c| (c.variables, c.equations, c.input_terms))
                        .collect::<Vec<_>>(),
                    new.charts
                        .iter()
                        .map(|c| (c.variables, c.equations, c.input_terms))
                        .collect::<Vec<_>>()
                );
                saw_positive |= new.status == OracleStatus::CompletePositive;
                saw_negative |= new.status == OracleStatus::CompleteNegative;
            }
            // Calling it for three queries has not mutated or repeated setup.
            assert_eq!(
                serde_json::to_value(prepared.setup_report()).unwrap(),
                setup_before
            );
        }
    }
    assert!(saw_positive && saw_negative);
}

#[test]
fn prepared_first_hit_caps_and_timeout_keep_their_exact_status_contract() {
    let (curve, fb, values, tower) = fixture();
    let prepared = PreparedOracle::prepare(
        &curve,
        19,
        &fb,
        &values,
        Some(&tower),
        3,
        Presentation::Tower4,
    )
    .unwrap();
    let target = fb.points[0];
    let mut first = limits();
    first.stop_after_first = true;
    let hit = prepared
        .decompose(target, &first, &mut GroupOps::default())
        .unwrap();
    assert_eq!(hit.status, OracleStatus::FoundFirst);
    assert!(!hit.complete && hit.positive);
    verify_witnesses(&fb, target, &hit);

    for capped in [
        OracleLimits {
            max_solutions: 0,
            ..limits()
        },
        OracleLimits {
            max_signed_lift_attempts: 0,
            ..limits()
        },
    ] {
        let result = prepared
            .decompose(target, &capped, &mut GroupOps::default())
            .unwrap();
        assert_eq!(result.status, OracleStatus::UnknownLimit);
        assert!(!result.complete);
        assert!(result.relations.is_empty());
    }
    let expired = OracleLimits {
        timeout_ms: 0,
        ..limits()
    };
    let result = prepared
        .decompose(target, &expired, &mut GroupOps::default())
        .unwrap();
    assert_eq!(result.status, OracleStatus::UnknownTimeout);
    assert!(!result.complete);
    assert_eq!(result.counters.validation_field_multiplications, 0);
    assert_eq!(prepared.setup_report().validation_field_multiplications, 16);
}

#[test]
fn canonical_identity_has_cancellation_witnesses_but_malformed_identity_is_rejected() {
    let (curve, fb, values, tower) = fixture();
    let prepared = PreparedOracle::prepare(
        &curve,
        19,
        &fb,
        &values,
        Some(&tower),
        2,
        Presentation::Tower4,
    )
    .unwrap();
    let fiber = PreparedOracle::prepare_fiber(
        &curve,
        19,
        &fb,
        &values,
        Some(&tower),
        2,
        Presentation::Tower4,
    )
    .unwrap();
    for result in [
        prepared
            .decompose(PrimePoint::INFINITY, &limits(), &mut GroupOps::default())
            .unwrap(),
        fiber
            .decompose_fiber(PrimePoint::INFINITY, &limits(), &mut GroupOps::default())
            .unwrap(),
    ] {
        assert_eq!(result.status, OracleStatus::CompletePositive);
        assert!(result.complete);
        assert!(result
            .relations
            .iter()
            .any(|r| r.row.iter().all(|&v| v == 0)));
        verify_witnesses(&fb, PrimePoint::INFINITY, &result);
    }
    for malformed in [
        PrimePoint {
            x: 1,
            y: 0,
            infinity: true,
        },
        PrimePoint {
            x: 0,
            y: 1,
            infinity: true,
        },
    ] {
        assert!(prepared
            .decompose(malformed, &limits(), &mut GroupOps::default())
            .is_err());
        assert!(fiber
            .decompose_fiber(malformed, &limits(), &mut GroupOps::default())
            .is_err());
        assert!(oracle::decompose(
            &curve,
            19,
            &fb,
            &values,
            Some(&tower),
            malformed,
            2,
            Presentation::Tower4,
            &limits(),
            &mut GroupOps::default(),
        )
        .is_err());
    }
}

#[test]
fn prepared_rejects_invalid_fixed_inputs_and_still_checks_each_target() {
    let (curve, fb, values, tower) = fixture();
    let prepare = |v: &[u64], t: Option<&TowerDescription>, m| {
        PreparedOracle::prepare(&curve, 19, &fb, v, t, m, Presentation::Tower).is_err()
            && PreparedOracle::prepare_fiber(&curve, 19, &fb, v, t, m, Presentation::Tower).is_err()
    };
    assert!(prepare(&[3, 1], Some(&tower), 3));
    assert!(prepare(&[1, 1], Some(&tower), 3));
    assert!(prepare(&[1, 17], Some(&tower), 3));
    assert!(prepare(&values, None, 3));
    assert!(prepare(&values, Some(&tower), 1));
    assert!(prepare(&values, Some(&tower), 5));
    for bad in [
        TowerDescription::Velu {
            steps: vec![(0, 3)],
            terminal: 5,
        },
        TowerDescription::Velu {
            steps: vec![(0, 0)],
            terminal: 4,
        },
        TowerDescription::Velu {
            steps: vec![(1, 3)],
            terminal: 4,
        },
        TowerDescription::Velu {
            steps: vec![(0, 3), (0, 3)],
            terminal: 4,
        },
    ] {
        assert!(prepare(&values, Some(&bad), 3));
    }
    let mut bad_base = fb.clone();
    bad_base.points[0] = PrimePoint::affine(3, 0);
    assert!(PreparedOracle::prepare(
        &curve,
        19,
        &bad_base,
        &values,
        Some(&tower),
        3,
        Presentation::Tower,
    )
    .is_err());
    assert!(PreparedOracle::prepare_fiber(
        &curve,
        19,
        &bad_base,
        &values,
        Some(&tower),
        3,
        Presentation::Tower,
    )
    .is_err());
    let prepared = PreparedOracle::prepare(
        &curve,
        19,
        &fb,
        &values,
        Some(&tower),
        3,
        Presentation::Tower,
    )
    .unwrap();
    for target in [PrimePoint::affine(0, 0), PrimePoint::affine(17, 1)] {
        assert!(prepared
            .decompose(target, &limits(), &mut GroupOps::default())
            .is_err());
    }
    for bad in [
        OracleLimits {
            max_degree: 0,
            ..limits()
        },
        OracleLimits {
            max_pairs_per_step: 0,
            ..limits()
        },
        OracleLimits {
            max_variables: 0,
            ..limits()
        },
    ] {
        assert!(prepared
            .decompose(fb.points[0], &bad, &mut GroupOps::default())
            .is_err());
    }
}
