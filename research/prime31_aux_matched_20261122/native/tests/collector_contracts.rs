use prime31_aux_matched::collector::{
    collect, CollectionLimits, PublicTarget, QueryStream, Recovery,
};
use prime31_aux_matched::oracle::{OracleLimits, Presentation};
use prime31_aux_matched::replay::{Curve, Point};
use prime31_aux_matched::{CountedGroup, FactorBase, GroupOps, PrimePoint};

#[test]
fn duplicate_a_row_can_recover_without_full_factor_base_rank() {
    // l0=9, d=7 modulo13. Both rows have A=(1,0,0), but different beta.
    let mut rank = Recovery::new(3, 13);
    assert_eq!(rank.insert(&[1, 0, 0], 2, 1).unwrap(), None);
    assert_eq!(rank.insert(&[1, 0, 0], 8, 2).unwrap(), Some(7));
    assert_eq!(rank.rank_a(), 1);
    assert_eq!(rank.rank_augmented(), 2);
    assert_eq!(rank.duplicates, 1);
    assert_eq!(rank.useful_dependencies, 1);
}

#[test]
fn a_dependency_with_zero_target_coefficient_is_not_recovery() {
    let mut rank = Recovery::new(1, 13);
    assert_eq!(rank.insert(&[1], 2, 1).unwrap(), None);
    assert_eq!(rank.insert(&[2], 4, 2).unwrap(), None);
    assert_eq!(rank.useful_dependencies, 0);
    // A=0 directly contains d even though rank(A) stays one.
    assert_eq!(rank.insert(&[0], 6, 1).unwrap(), Some(7));
    assert_eq!(rank.useful_dependencies, 1);
}

#[test]
fn inconsistent_rows_are_rejected() {
    let mut rank = Recovery::new(1, 13);
    rank.insert(&[1], 2, 1).unwrap();
    assert!(rank.insert(&[1], 3, 1).is_err());
}

#[test]
fn independent_replay_rejects_noncanonical_points_and_handles_identity() {
    let e = Curve { p: 19, a: 0, b: 2 };
    let g = Point::affine(4, 3);
    assert_eq!(e.add(g, Point::affine(4, 16)).unwrap(), Point::IDENTITY);
    assert_eq!(e.scalar(g, 13).unwrap(), Point::IDENTITY);
    assert!(e.add(g, Point::affine(23, 3)).is_err());
    assert!(e.add(g, Point::affine(4, 4)).is_err());
    let noncanonical_infinity = Point {
        x: 1,
        y: 0,
        infinity: true,
    };
    assert!(e.add(g, noncanonical_infinity).is_err());
}

#[test]
fn public_target_natural_queries_recover_on_fixed_small_fixture() {
    // Correctness test only: no Prime31 benchmark and no preselected relations.
    // This curve has exactly13 points, checked by PublicTarget::validate.
    let e = Curve { p: 19, a: 0, b: 2 };
    let g = Point::affine(4, 3);
    let target = PublicTarget {
        p: 19,
        a: 0,
        b: 2,
        order: 13,
        generator: g,
        target: e.scalar(g, 7).unwrap(),
    };
    let curve = target.curve();
    let points = vec![
        PrimePoint::affine(4, 3),
        PrimePoint::affine(4, 16),
        PrimePoint::affine(6, 3),
        PrimePoint::affine(6, 16),
    ];
    let fb = FactorBase::from_column_map(
        "test c2".into(),
        points,
        vec![0, 0, 1, 1],
        vec![1, 12, 1, 12],
        2,
        |p| curve.key(p),
        |p| curve.key(&curve.neg(*p)),
        |p| p.x,
    )
    .unwrap();
    let limits = CollectionLimits {
        max_queries: 80,
        max_seconds: 30,
        oracle: OracleLimits {
            timeout_ms: 1000,
            max_degree: 12,
            ..Default::default()
        },
    };
    let mut records = Vec::new();
    let summary = collect(
        &target,
        &fb,
        &[4, 6],
        None,
        2,
        Presentation::Dense,
        4188,
        &limits,
        &mut |v| {
            records.push(v.clone());
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(summary.recovered_scalar, Some(7));
    assert!(summary.recovery_replayed);
    assert!(summary.queries > 0);
    assert_eq!(summary.accepted_rows, summary.independent_replay_rows);
    assert_eq!(
        records
            .iter()
            .filter(|v| v["event"] == "query_start")
            .count() as u64,
        summary.queries
    );
    let mut validation = GroupOps::default();
    target.validate(&mut validation).unwrap();
    let mut stream = QueryStream::new(4188);
    for record in records.iter().filter(|v| v["event"] == "query_start") {
        assert_eq!(record["alpha"].as_u64(), Some(stream.scalar(13)));
        assert_eq!(record["beta"].as_u64(), Some(stream.scalar(13)));
    }
}
