//! Fixed small-field contracts for source-bound specialized MITM. No Prime31
//! campaign, timing comparison or performance inference is made here.
use prime31_aux_matched::collector::to_replay;
use prime31_aux_matched::oracle::{OracleLimits, OracleStatus, Presentation};
use prime31_aux_matched::pair_control::{PairControl, PairMode};
use prime31_aux_matched::replay::{Curve, Point};
use prime31_aux_matched::specialized_control::{
    SpecializedConfig, SpecializedControl, SPECIALIZED_SOURCE_MANIFEST,
};
use prime31_aux_matched::{CountedGroup, FactorBase, GroupOps, PrimeCurve, PrimePoint};

fn fixture(scrambled: bool) -> (PrimeCurve, FactorBase<PrimePoint>) {
    let curve = PrimeCurve { p: 17, a: 2, b: 2 };
    let triples = if scrambled {
        [(6, 3, 2), (3, 1, 0), (5, 1, 1)]
    } else {
        [(3, 1, 0), (5, 1, 1), (6, 3, 2)]
    };
    let mut points = Vec::new();
    let mut columns = Vec::new();
    let mut signs = Vec::new();
    for (x, y, column) in triples {
        points.extend([PrimePoint::affine(x, y), PrimePoint::affine(x, 17 - y)]);
        columns.extend([column, column]);
        signs.extend([1, 18]);
    }
    let base = FactorBase::from_column_map(
        "specialized-contract".into(),
        points,
        columns,
        signs,
        3,
        |point| curve.key(point),
        |point| curve.key(&curve.neg(*point)),
        |point| point.x,
    )
    .unwrap();
    (curve, base)
}

fn limits() -> OracleLimits {
    OracleLimits {
        stop_after_first: true,
        timeout_ms: 20_000,
        ..Default::default()
    }
}

fn check_replay(base: &FactorBase<PrimePoint>, target: PrimePoint, indices: &[usize], row: &[i64]) {
    let curve = Curve { p: 17, a: 2, b: 2 };
    assert_eq!(
        curve
            .sum(indices.iter().map(|&i| to_replay(base.points[i])))
            .unwrap(),
        to_replay(target)
    );
    let mut expected = vec![0i64; base.columns];
    for &i in indices {
        expected[base.col_of[i]] += if base.coef_of[i] == 1 { 1 } else { -1 };
    }
    assert_eq!(row, expected);
}

#[test]
fn specialized_finite_and_identity_decisions_match_scalar_reference() {
    let (curve, base) = fixture(false);
    let scalar = PairControl::prepare(&curve, 19, &base, PairMode::Scalar).unwrap();
    let mut specialized = SpecializedControl::prepare(&curve, 19, &base).unwrap();
    assert_eq!(
        specialized.setup_report()["native"]["mitm_compact_map32_backend"],
        1
    );
    let independent = Curve { p: 17, a: 2, b: 2 };
    let generator = Point::affine(5, 1);
    for k in 0..19 {
        let point = independent.scalar(generator, k).unwrap();
        let target = if point.infinity {
            PrimePoint::INFINITY
        } else {
            PrimePoint::affine(point.x, point.y)
        };
        let expected = scalar
            .decompose(
                target,
                &limits(),
                Presentation::Dense,
                &mut GroupOps::default(),
            )
            .unwrap();
        let actual = specialized
            .decompose(
                target,
                &limits(),
                Presentation::Dense,
                &mut GroupOps::default(),
            )
            .unwrap();
        assert_eq!(actual.status, expected.status, "k={k}");
        assert_eq!(actual.relations, expected.relations, "k={k}");
        assert_eq!(actual.complete, expected.complete);
        for relation in &actual.relations {
            check_replay(&base, target, &relation.indices, &relation.row);
        }
        if k == 0 {
            assert_eq!(
                actual.backend_work.as_ref().unwrap()["native"]["scalar_identity_fallback"],
                true
            );
        }
    }
}

#[test]
fn copied_backend_keeps_original_signed_indices_and_columns_on_scrambled_base() {
    let (curve, base) = fixture(true);
    let scalar = PairControl::prepare(&curve, 19, &base, PairMode::Scalar).unwrap();
    let mut specialized = SpecializedControl::prepare(&curve, 19, &base).unwrap();
    let independent = Curve { p: 17, a: 2, b: 2 };
    for k in [0, 1, 2, 4, 7, 11, 18] {
        let point = independent.scalar(Point::affine(5, 1), k).unwrap();
        let target = if point.infinity {
            PrimePoint::INFINITY
        } else {
            PrimePoint::affine(point.x, point.y)
        };
        let expected = scalar
            .decompose(
                target,
                &limits(),
                Presentation::Dense,
                &mut GroupOps::default(),
            )
            .unwrap();
        let actual = specialized
            .decompose(
                target,
                &limits(),
                Presentation::Dense,
                &mut GroupOps::default(),
            )
            .unwrap();
        // The maintained finite oracle scans sorted abscissa groups, so on
        // scrambled input the first witness need not be the scalar one's.
        assert_eq!(actual.positive, expected.positive);
        assert_eq!(actual.complete, expected.complete);
        for relation in &actual.relations {
            check_replay(&base, target, &relation.indices, &relation.row);
        }
    }
}

#[test]
fn unsupported_caps_and_corrupt_metadata_fail_without_false_negatives() {
    let (curve, base) = fixture(false);
    let mut specialized = SpecializedControl::prepare(&curve, 19, &base).unwrap();
    for (config, status) in [
        (
            OracleLimits {
                timeout_ms: 0,
                ..limits()
            },
            OracleStatus::UnknownTimeout,
        ),
        (
            OracleLimits {
                stop_after_first: false,
                ..limits()
            },
            OracleStatus::UnknownLimit,
        ),
        (
            OracleLimits {
                max_solutions: 0,
                ..limits()
            },
            OracleStatus::UnknownLimit,
        ),
    ] {
        let result = specialized
            .decompose(
                base.points[0],
                &config,
                Presentation::Dense,
                &mut GroupOps::default(),
            )
            .unwrap();
        assert_eq!(result.status, status);
        assert!(!result.complete && !result.positive);
        assert!(result.backend_work.unwrap()["native"].is_null());
    }
    assert!(specialized
        .decompose(
            PrimePoint {
                x: 1,
                y: 0,
                infinity: true
            },
            &limits(),
            Presentation::Dense,
            &mut GroupOps::default()
        )
        .is_err());
    assert!(SpecializedControl::prepare_with_config(
        &curve,
        19,
        &base,
        SpecializedConfig {
            setup_chunk: 0,
            probe_chunk: 16
        }
    )
    .is_err());
    let mut bad = base.clone();
    bad.neg_index[0] = 0;
    assert!(SpecializedControl::prepare(&curve, 19, &bad).is_err());
    let mut bad = base.clone();
    bad.coef_of[1] = 1;
    assert!(SpecializedControl::prepare(&curve, 19, &bad).is_err());
    let mut bad = base;
    bad.x_index.get_mut(&3).unwrap().reverse();
    assert!(SpecializedControl::prepare(&curve, 19, &bad).is_err());
}

#[test]
fn specialized_source_receipt_binds_distinct_exact_native_namespace() {
    let source: serde_json::Value = serde_json::from_str(SPECIALIZED_SOURCE_MANIFEST).unwrap();
    assert_eq!(
        source["revision"],
        "457ae1818aaf8134bd833ad9c1b9d57015264510"
    );
    let selections = source["extracted_production_ranges"].as_array().unwrap();
    assert_eq!(selections.len(), 4);
    assert!(selections
        .iter()
        .all(|v| v["rewritten"] == false && v["inserted_bytes"] == 0));
    assert!(selections
        .iter()
        .any(|v| v["git_blob"] == "b77edb2569cf0bdde83f0456434d559ea84460fd"));
}
