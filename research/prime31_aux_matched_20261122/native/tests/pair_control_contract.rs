//! Small-field correctness contracts, not Prime31 timing or recovery evidence.
use prime31_aux_matched::collector::{
    collect_with_decomposer, to_replay, CollectionLimits, PublicTarget, QueryStream,
};
use prime31_aux_matched::cryptanalysis::ic_boundary::{
    decompose_mitm, OracleCounters as ProductionOracleCounters, PairTable,
};
use prime31_aux_matched::oracle::{OracleLimits, OracleResult, OracleStatus, Presentation};
use prime31_aux_matched::pair_control::{PairControl, PairMode, PROBE_BLOCK};
use prime31_aux_matched::replay::{Curve, Point};
use prime31_aux_matched::{CountedGroup, FactorBase, GroupOps, PrimeCurve, PrimePoint};
use std::collections::BTreeSet;

fn independent(curve: &PrimeCurve) -> Curve {
    Curve {
        p: curve.p,
        a: curve.a,
        b: curve.b,
    }
}

fn all_points(curve: &PrimeCurve) -> Vec<PrimePoint> {
    let verifier = independent(curve);
    let mut points = vec![PrimePoint::INFINITY];
    for x in 0..curve.p {
        for y in 0..curve.p {
            if verifier.on_curve(Point::affine(x, y)) {
                points.push(PrimePoint::affine(x, y));
            }
        }
    }
    points
}

fn signed_base(curve: &PrimeCurve, order: u64, xs: &[u64]) -> FactorBase<PrimePoint> {
    let verifier = independent(curve);
    let mut points = Vec::new();
    let mut columns = Vec::new();
    let mut coefficients = Vec::new();
    for (column, &x) in xs.iter().enumerate() {
        let y = (0..curve.p)
            .find(|&y| verifier.on_curve(Point::affine(x, y)))
            .expect("test abscissa must lift");
        points.push(PrimePoint::affine(x, y));
        columns.push(column);
        coefficients.push(1);
        if y != 0 {
            points.push(PrimePoint::affine(x, curve.p - y));
            columns.push(column);
            coefficients.push(order - 1);
        }
    }
    FactorBase::from_column_map(
        "pair-control-contract".into(),
        points,
        columns,
        coefficients,
        xs.len(),
        |p| curve.key(p),
        |p| curve.key(&curve.neg(*p)),
        |p| p.x,
    )
    .unwrap()
}

fn limits() -> OracleLimits {
    OracleLimits {
        timeout_ms: 20_000,
        stop_after_first: true,
        ..Default::default()
    }
}

fn brute_rows(
    curve: &PrimeCurve,
    base: &FactorBase<PrimePoint>,
    target: PrimePoint,
) -> BTreeSet<Vec<i64>> {
    let verifier = independent(curve);
    let mut rows = BTreeSet::new();
    for i in 0..base.points.len() {
        for j in 0..base.points.len() {
            for k in 0..base.points.len() {
                let indices = [i, j, k];
                if verifier
                    .sum(indices.iter().map(|&s| to_replay(base.points[s])))
                    .unwrap()
                    == to_replay(target)
                {
                    let mut row = vec![0; base.columns];
                    for s in indices {
                        row[base.col_of[s]] += if base.coef_of[s] == 1 { 1 } else { -1 };
                    }
                    rows.insert(row);
                }
            }
        }
    }
    rows
}

fn check_result(
    curve: &PrimeCurve,
    base: &FactorBase<PrimePoint>,
    target: PrimePoint,
    result: &OracleResult,
    expected: &BTreeSet<Vec<i64>>,
    ops: &GroupOps,
) {
    assert_eq!(result.positive, !expected.is_empty());
    assert_eq!(result.complete, expected.is_empty());
    assert_eq!(
        result.status,
        if expected.is_empty() {
            OracleStatus::CompleteNegative
        } else {
            OracleStatus::FoundFirst
        }
    );
    assert!(result.charts.is_empty());
    assert_eq!(result.counters.f4_runs, 0);
    assert_eq!(result.counters.f4_field_operations, 0);
    assert_eq!(result.polynomial_build_ns, 0);
    let work = result.backend_work.as_ref().unwrap();
    assert_eq!(work["backend"], "pair_table");
    let native = &work["native"];
    let visited = native["candidate_complements"].as_u64().unwrap();
    let identity = native["identity_complements"].as_u64().unwrap();
    let probes = native["table_probe_calls"].as_u64().unwrap();
    let hits = native["pair_witness_hits"].as_u64().unwrap();
    let replays = native["replay_additions"].as_u64().unwrap();
    assert_eq!(probes + identity, visited);
    assert_eq!(ops.adds, visited + hits + replays);
    assert_eq!(
        work["group_and_field"]["group"]["adds"].as_u64(),
        Some(ops.adds)
    );
    assert_eq!(
        work["group_and_field"]["field"]["invs"].as_u64(),
        Some(ops.field_ops.invs)
    );
    if expected.is_empty() {
        assert!(result.relations.is_empty());
        assert_eq!(visited as usize, base.points.len());
        assert_eq!(native["cancellation_hits"], 0);
        assert_eq!(replays, 0);
    } else {
        assert_eq!(result.relations.len(), 1);
        let relation = &result.relations[0];
        assert_eq!(relation.indices.len(), 3);
        assert!(expected.contains(&relation.row));
        assert_eq!(
            independent(curve)
                .sum(relation.indices.iter().map(|&i| to_replay(base.points[i])))
                .unwrap(),
            to_replay(target)
        );
        assert_eq!(replays, 3);
    }
}

#[test]
fn exhaustive_small_field_matches_literal_triples_and_production_scalar_witnesses() {
    let curve = PrimeCurve { p: 17, a: 2, b: 2 };
    let order = 19;
    let base = signed_base(&curve, order, &[3, 5]);
    let scalar = PairControl::prepare(&curve, order, &base, PairMode::Scalar).unwrap();
    let batched = PairControl::prepare(&curve, order, &base, PairMode::Batched).unwrap();
    let production = PairTable::build_negation_folded(&curve, &base);
    assert_eq!(all_points(&curve).len(), order as usize);
    for target in all_points(&curve) {
        let expected = brute_rows(&curve, &base, target);
        let mut scalar_ops = GroupOps::default();
        let a = scalar
            .decompose(target, &limits(), Presentation::Dense, &mut scalar_ops)
            .unwrap();
        let mut batched_ops = GroupOps::default();
        let b = batched
            .decompose(target, &limits(), Presentation::Dense, &mut batched_ops)
            .unwrap();
        check_result(&curve, &base, target, &a, &expected, &scalar_ops);
        check_result(&curve, &base, target, &b, &expected, &batched_ops);
        assert_eq!(a.relations, b.relations);
        assert_eq!(a.status, b.status);
        if let Some(i) = base.index_of_key(curve.key(&target)) {
            assert_eq!(a.relations[0].indices, vec![i, 0, base.neg_index[0]]);
            assert_eq!(
                a.backend_work.as_ref().unwrap()["native"]["cancellation_hits"],
                1
            );
            assert_eq!(
                a.backend_work.as_ref().unwrap()["native"]["table_probe_calls"],
                0
            );
        } else {
            let reference = decompose_mitm(
                &curve,
                &base,
                &production,
                3,
                &mut GroupOps::default(),
                &mut ProductionOracleCounters::default(),
                target,
            );
            assert_eq!(a.relations.first().map(|r| r.indices.clone()), reference);
        }
    }
    let setup = batched.setup_report();
    assert_eq!(setup.table_kind, "negation_folded");
    assert_eq!(setup.table_entries, scalar.setup_report().table_entries);
    assert_eq!(setup.insert_attempts, 4);
    assert_eq!(setup.index_lookups, 2);
    assert_eq!(setup.table_group_and_field["group"]["adds"], 6);
    assert_eq!(setup.validation_group_and_field["field"]["muls"], 21);
    assert_eq!(setup.requested_hash_entry_capacity, 5);
    assert_eq!(setup.requested_payload_bytes, 80);
    assert_eq!(setup.logical_entry_payload_bytes, 16 * setup.table_entries);
    assert_eq!(setup.production_native["table_insert_attempts"], 4);
    assert_eq!(setup.production_native["lookups"], 2);
}

#[test]
fn repetitions_and_doubling_produce_a_literal_three_point_witness() {
    let curve = PrimeCurve { p: 17, a: 2, b: 2 };
    let base = signed_base(&curve, 19, &[5]);
    let target = independent(&curve)
        .scalar(to_replay(base.points[0]), 3)
        .unwrap();
    let target = PrimePoint::affine(target.x, target.y);
    let pair = PairControl::prepare(&curve, 19, &base, PairMode::Batched).unwrap();
    let result = pair
        .decompose(
            target,
            &limits(),
            Presentation::Dense,
            &mut GroupOps::default(),
        )
        .unwrap();
    assert_eq!(result.status, OracleStatus::FoundFirst);
    assert_eq!(result.relations[0].indices, vec![0, 0, 0]);
    assert_eq!(result.relations[0].row, vec![3]);
    let result = pair
        .decompose(
            PrimePoint::INFINITY,
            &limits(),
            Presentation::Dense,
            &mut GroupOps::default(),
        )
        .unwrap();
    assert_eq!(result.status, OracleStatus::CompleteNegative);
    assert!(result.complete);
}

#[test]
fn omitted_identity_pairs_are_closed_by_cancellation_even_for_two_torsion() {
    let curve = PrimeCurve { p: 17, a: 16, b: 0 };
    let base = signed_base(&curve, 2, &[0]);
    assert_eq!(
        independent(&curve)
            .scalar(to_replay(base.points[0]), 2)
            .unwrap(),
        Point::IDENTITY
    );
    for mode in [PairMode::Scalar, PairMode::Batched] {
        let pair = PairControl::prepare(&curve, 2, &base, mode).unwrap();
        assert_eq!(pair.setup_report().table_entries, 0);
        let positive = pair
            .decompose(
                base.points[0],
                &limits(),
                Presentation::Dense,
                &mut GroupOps::default(),
            )
            .unwrap();
        assert_eq!(positive.status, OracleStatus::FoundFirst);
        assert_eq!(positive.relations[0].indices, vec![0, 0, 0]);
        assert_eq!(positive.relations[0].row, vec![3]);
        let negative = pair
            .decompose(
                PrimePoint::INFINITY,
                &limits(),
                Presentation::Dense,
                &mut GroupOps::default(),
            )
            .unwrap();
        assert_eq!(negative.status, OracleStatus::CompleteNegative);
    }
    // The entire rational two-torsion subgroup also admits an infinite triple
    // with three distinct nonzero points. The table's finite pair entries must
    // still find it; an infinite target is not itself an error or an auto-hit.
    let base = signed_base(&curve, 4, &[0, 1, 16]);
    for mode in [PairMode::Scalar, PairMode::Batched] {
        let pair = PairControl::prepare(&curve, 4, &base, mode).unwrap();
        for target in std::iter::once(PrimePoint::INFINITY).chain(base.points.iter().copied()) {
            let expected = brute_rows(&curve, &base, target);
            let mut ops = GroupOps::default();
            let result = pair
                .decompose(target, &limits(), Presentation::Dense, &mut ops)
                .unwrap();
            check_result(&curve, &base, target, &result, &expected, &ops);
        }
    }
}

#[test]
fn production_batched_table_preserves_every_scalar_key_and_first_witness_across_blocks() {
    let curve = PrimeCurve { p: 257, a: 2, b: 2 };
    let points = all_points(&curve);
    let xs: Vec<u64> = points
        .iter()
        .filter(|p| !p.infinity)
        .map(|p| p.x)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let base = signed_base(&curve, points.len() as u64, &xs);
    assert!(base.points.len() > PROBE_BLOCK);
    let scalar = PairTable::build_negation_folded(&curve, &base);
    let batch = PairTable::build_negation_folded_batched(&curve, &base);
    assert_eq!(scalar.entries, batch.entries);
    assert_eq!(scalar.insert_attempts, batch.insert_attempts);
    assert_eq!(scalar.build_ops.adds, batch.build_ops.adds);
    assert_eq!(scalar.index_lookups, batch.index_lookups);
    for folded in 0..=curve.p + 1 {
        assert_eq!(scalar.lookup_folded(folded), batch.lookup_folded(folded));
    }
    // These are execution counters, not a measured speed ratio.
    assert!(batch.build_ops.field_ops.invs > 0);
    assert!(batch.build_ops.montgomery_reductions > 0);
}

#[test]
fn capped_queries_are_unknown_and_every_receipt_identifies_the_backend() {
    let curve = PrimeCurve { p: 17, a: 2, b: 2 };
    let base = signed_base(&curve, 19, &[3, 5]);
    let pair = PairControl::prepare(&curve, 19, &base, PairMode::Batched).unwrap();
    for (setting, status) in [
        (
            OracleLimits {
                timeout_ms: 0,
                ..limits()
            },
            OracleStatus::UnknownTimeout,
        ),
        (
            OracleLimits {
                max_solutions: 0,
                ..limits()
            },
            OracleStatus::UnknownLimit,
        ),
        (
            OracleLimits {
                stop_after_first: false,
                ..limits()
            },
            OracleStatus::UnknownLimit,
        ),
    ] {
        let mut ops = GroupOps::default();
        ops.adds = 17;
        let result = pair
            .decompose(base.points[0], &setting, Presentation::Dense, &mut ops)
            .unwrap();
        assert_eq!(result.status, status);
        assert!(!result.complete && !result.positive && result.relations.is_empty());
        let work = result.backend_work.as_ref().unwrap();
        assert_eq!(work["backend"], "pair_table");
        assert_eq!(work["mode"], "batched");
        assert_eq!(work["group_and_field"]["group"]["adds"], 0);
        assert_eq!(ops.adds, 17);
    }
    let invalid = PrimePoint {
        x: 1,
        y: 0,
        infinity: true,
    };
    assert!(pair
        .decompose(
            invalid,
            &limits(),
            Presentation::Dense,
            &mut GroupOps::default()
        )
        .is_err());
    assert!(pair
        .decompose(
            PrimePoint::affine(5, 2),
            &limits(),
            Presentation::Dense,
            &mut GroupOps::default()
        )
        .is_err());
    let mut bad = base.clone();
    bad.neg_index[0] = 0;
    assert!(PairControl::prepare(&curve, 19, &bad, PairMode::Batched).is_err());
    let mut bad = base.clone();
    bad.coef_of[1] = 1;
    assert!(PairControl::prepare(&curve, 19, &bad, PairMode::Batched).is_err());
    let mut bad = base;
    bad.x_index.get_mut(&3).unwrap().clear();
    assert!(PairControl::prepare(&curve, 19, &bad, PairMode::Batched).is_err());
}

#[test]
fn same_natural_query_collector_replays_rows_and_recovers_a_small_public_target() {
    let verifier = Curve { p: 19, a: 0, b: 2 };
    let generator = Point::affine(4, 3);
    let public = PublicTarget {
        p: 19,
        a: 0,
        b: 2,
        order: 13,
        generator,
        target: verifier.scalar(generator, 7).unwrap(),
    };
    let curve = public.curve();
    let base = signed_base(&curve, public.order, &[4, 6]);
    let pair = PairControl::prepare(&curve, public.order, &base, PairMode::Batched).unwrap();
    let limits = CollectionLimits {
        max_queries: 80,
        max_seconds: 30,
        oracle: limits(),
    };
    let mut records = Vec::new();
    let summary = collect_with_decomposer(
        &public,
        &base,
        3,
        Presentation::Dense,
        4188,
        &limits,
        Some("pair_table"),
        &mut |record| {
            records.push(record.clone());
            Ok(())
        },
        &mut |target, limits, ops| pair.decompose(target, limits, Presentation::Dense, ops),
    )
    .unwrap();
    assert_eq!(summary.recovered_scalar, Some(7));
    assert!(summary.recovery_replayed);
    assert!(summary.queries > 0);
    assert_eq!(summary.accepted_rows, summary.independent_replay_rows);
    let mut stream = QueryStream::new(4188);
    for record in records.iter().filter(|v| v["event"] == "query_start") {
        assert_eq!(record["alpha"].as_u64(), Some(stream.scalar(13)));
        assert_eq!(record["beta"].as_u64(), Some(stream.scalar(13)));
    }
    for record in records.iter().filter(|v| v["event"] == "query_result") {
        assert_eq!(record["oracle"]["backend_work"]["backend"], "pair_table");
        assert_eq!(record["oracle"]["backend_work"]["mode"], "batched");
    }
}
