//! Deterministic M31 API controls. Fixture labels are disclosed and test-only;
//! the production API receives public points and public query coefficients.

use crate::extractor::{extract, extract_cofactor, extract_cofactor_visit, extract_visit, Limits, Status};
use crate::query_pipeline::{PipelineError, QueryPipeline, QueryState};
use crate::relation_adapter::{AdapterError, RelationAdapter};
use crypto_lib::cryptanalysis::f4_fp::F4ResourceLimits;
use prime31_algebraic_ic::{Arithmetic, Point, PRIME31};
use std::collections::BTreeSet;

fn fixture() -> (Vec<(u64, u64)>, Point, Point) {
    let e = PRIME31;
    let mut a = Arithmetic::default();
    let base = [2, 3, 5, 11].into_iter()
        .map(|k| a.scalar(e, k, Some(e.g)).unwrap()).collect();
    let query = a.scalar(e, 21, Some(e.g));
    let target = a.scalar(e, 7, Some(e.g));
    (base, query, target)
}

fn resources() -> F4ResourceLimits {
    F4ResourceLimits {
        max_pairs: 20_000,
        max_symbolic_monomials: 20_000,
        max_matrix_entries: 2_000_000,
    }
}

#[test]
fn m31_stream_matches_batch_signed_rows() {
    let (base, query, _) = fixture();
    let batch = extract(PRIME31, &base, query, Limits::default());
    let mut delivered = Vec::new();
    let stream = extract_visit(PRIME31, &base, query, Limits::default(), |row| {
        delivered.push(row.clone());
        false
    });
    assert_eq!(batch.status, Status::Complete);
    assert_eq!(stream.status, Status::Complete);
    let set = |rows: &[crate::relation_adapter::SignedRelation]| {
        rows.iter().map(|r| r.terms.clone()).collect::<BTreeSet<_>>()
    };
    assert_eq!(set(&batch.relations), set(&stream.relations));
    assert_eq!(delivered, stream.relations);
    assert_eq!(stream.diagnostics.assignments_extracted, batch.diagnostics.assignments_extracted);
    assert_eq!(stream.diagnostics.forbidden_domain_evaluations, 0);
    assert_eq!(stream.diagnostics.forbidden_branch_attempts, 0);
}

#[test]
fn m31_consumer_stop_is_explicitly_incomplete() {
    let (base, query, _) = fixture();
    let mut calls = 0;
    let result = extract_visit(PRIME31, &base, query, Limits::default(), |_| {
        calls += 1;
        true
    });
    assert_eq!(calls, 1);
    assert_eq!(result.relations.len(), 1);
    assert_eq!(result.status, Status::ConsumerStopped);
    let traversal = result.traversal.unwrap();
    assert!(traversal.stopped_early);
    assert!(!traversal.complete);
}

#[test]
fn m31_cofactor_stream_and_batch_agree() {
    let (base, query, _) = fixture();
    let batch = extract_cofactor(PRIME31, &base, query, Limits::default(), resources());
    let mut delivered = Vec::new();
    let stream = extract_cofactor_visit(
        PRIME31, &base, query, Limits::default(), resources(), |row| {
            delivered.push(row.clone());
            false
        },
    );
    assert_eq!(batch.status, Status::Complete);
    assert_eq!(stream.status, Status::Complete);
    assert_eq!(batch.relations, stream.relations);
    assert_eq!(delivered, stream.relations);
}

#[test]
fn m31_false_callback_continues_and_assignment_cap_remains_explicit() {
    let mut arithmetic = Arithmetic::default();
    let base: Vec<_> = [2, 3, 5, 11, 13].into_iter()
        .map(|k| arithmetic.scalar(PRIME31, k, Some(PRIME31.g)).unwrap()).collect();
    let query = arithmetic.scalar(PRIME31, 21, Some(PRIME31.g));
    let limits = Limits { seconds: 30, ..Limits::default() };
    // Two disclosed distinct-four witnesses: 2+3+5+11 and 2-5+11+13.
    let batch = extract_cofactor(PRIME31, &base, query, limits, resources());
    assert_eq!(batch.status, Status::Complete);
    assert!(batch.relations.len() >= 2);
    let mut delivered = Vec::new();
    let stream = extract_cofactor_visit(PRIME31, &base, query, limits, resources(), |row| {
        delivered.push(row.clone());
        false
    });
    assert_eq!(stream.status, Status::Complete);
    assert_eq!(delivered, batch.relations);
    assert_eq!(stream.relations, batch.relations);
    let capped = extract_cofactor_visit(
        PRIME31, &base, query, Limits { max_assignments: 1, ..limits }, resources(), |_| false,
    );
    assert_eq!(capped.status, Status::OutputLimit);
    assert_eq!(capped.diagnostics.assignments_extracted, 1);
    assert!(!capped.traversal.unwrap().complete);
}

#[test]
fn m31_empty_or_unsupported_query_never_calls_consumer() {
    let (base, _, _) = fixture();
    let mut arithmetic = Arithmetic::default();
    // Signed sums of these four disclosed labels lie in [-21,21].
    // G is not empty: (-2-3-5+11)G=G.
    let empty_query = arithmetic.scalar(PRIME31, 100, Some(PRIME31.g));
    for query in [None, Some(base[0]), empty_query] {
        let mut calls = 0;
        let result = extract_visit(PRIME31, &base, query, Limits::default(), |_| {
            calls += 1;
            false
        });
        assert_eq!(calls, 0);
        assert!(matches!(result.status, Status::Empty | Status::UnsupportedChart));
    }
}

#[test]
fn m31_public_queries_recover_target_before_full_rank() {
    let (base, query, target) = fixture();
    let mut pipeline = QueryPipeline::new(PRIME31, base, target).unwrap();
    let first = pipeline.submit(21, 0, Limits::default(), resources());
    assert_eq!(first.query_point, query);
    assert_eq!(first.state, QueryState::QueryExhausted);
    assert_eq!(first.rank_after, 1);
    assert_eq!(first.recovered_scalar, None);
    let second = pipeline.submit(7, 2, Limits::default(), resources());
    assert_eq!(second.query_point, query);
    assert_eq!(second.state, QueryState::TargetRecovered);
    assert_eq!(second.rank_after, 2);
    assert_eq!(second.recovered_scalar, Some(7));
    assert!(second.rank_after < 5);
    assert_eq!(second.extraction.as_ref().unwrap().status, Status::ConsumerStopped);
    assert!(!second.extraction.as_ref().unwrap().traversal.as_ref().unwrap().complete);
    assert_eq!(pipeline.adapter_work().scalar_replays, 1);
    let mut replay = Arithmetic::default();
    assert_eq!(replay.scalar(PRIME31, pipeline.scalar().unwrap(), Some(PRIME31.g)), target);
    assert!(!second.phase_accounting_complete);
    println!("M31_STREAM_TARGET {}", serde_json::to_string(&second).unwrap());
    let third = pipeline.submit(0, 1, Limits::default(), resources());
    assert_eq!(third.state, QueryState::AlreadyRecovered);
    assert!(third.extraction.is_none());
    assert!(third.consumed.is_empty());
    assert_eq!(third.query_arithmetic, Default::default());
}

#[test]
fn m31_duplicate_query_adds_no_rank_or_target_information() {
    let (base, _, target) = fixture();
    let mut pipeline = QueryPipeline::new(PRIME31, base, target).unwrap();
    let first = pipeline.submit(21, 0, Limits::default(), resources());
    let second = pipeline.submit(21, 0, Limits::default(), resources());
    assert_eq!(first.rank_after, 1);
    assert_eq!(second.rank_after, 1);
    assert_eq!(second.recovered_scalar, None);
    assert_eq!(second.cumulative_adapter_work.duplicates, 1);
}

#[test]
fn m31_streamed_row_with_false_rhs_is_rejected() {
    let (base, query, target) = fixture();
    let mut adapter = RelationAdapter::new(PRIME31, base.clone(), target).unwrap();
    let result = extract_visit(PRIME31, &base, query, Limits::default(), |row| {
        assert_eq!(adapter.accept(row, 20, 0), Err(AdapterError::GroupEqualityFailed));
        false
    });
    assert_eq!(result.status, Status::Complete);
    assert_eq!(adapter.rank(), 0);
    assert_eq!(adapter.scalar(), None);
}

#[test]
fn m31_pipeline_rejects_wrong_field_and_duplicate_abscissae() {
    let (base, _, target) = fixture();
    let mut wrong = PRIME31;
    wrong.p = wrong.n;
    assert!(matches!(QueryPipeline::new(wrong, base.clone(), target),
        Err(PipelineError::CoordinateFieldMustBeM31)));
    let mut duplicated = base;
    duplicated[1] = (duplicated[0].0, PRIME31.p - duplicated[0].1);
    assert!(matches!(QueryPipeline::new(PRIME31, duplicated, target),
        Err(PipelineError::InvalidDistinctAbscissaBase)));
}

#[test]
fn m31_pipeline_resource_limit_is_not_empty_or_recovered() {
    let (base, _, target) = fixture();
    let mut pipeline = QueryPipeline::new(PRIME31, base, target).unwrap();
    let limited = F4ResourceLimits { max_matrix_entries: 1, ..resources() };
    let receipt = pipeline.submit(21, 0, Limits::default(), limited);
    assert_eq!(receipt.state, QueryState::Incomplete);
    assert_eq!(receipt.recovered_scalar, None);
    assert_eq!(receipt.rank_after, 0);
    assert_eq!(receipt.extraction.unwrap().status, Status::ResourceLimit);
}
