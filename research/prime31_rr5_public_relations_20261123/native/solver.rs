//! Public relation-query integration using the original solver's authentic-RHS
//! contract. The supplied base is an explicit input whose creation must be
//! charged separately; this API never calls a generic decomposition fallback.
use crate::extractor::{extract_with_form_until, ExtractionForm, Limits, ResultRow};
use crate::relation_adapter::{AdapterError, AdapterWork, RelationAdapter, RelationReceipt};
use prime31_algebraic_ic::{Arithmetic, Counts, Curve, Point};
use serde::Serialize;
use std::time::{Duration, Instant};

#[derive(Clone, Debug, Serialize)]
pub struct Attempt {
    pub a: u64,
    pub b: u64,
    pub point: Point,
    pub extraction: ResultRow,
    pub receipts: Vec<RelationReceipt>,
    pub acceptance_error: Option<AdapterError>,
}

#[derive(Clone, Debug, Serialize)]
pub struct SolveRecord {
    pub status: String,
    pub scalar: Option<u64>,
    pub target: Point,
    pub attempts: Vec<Attempt>,
    pub rank: usize,
    pub query_arithmetic: Counts,
    pub consumer_arithmetic: Counts,
    pub consumer_setup_arithmetic: Counts,
    pub rank_work: AdapterWork,
    pub terminal_error: Option<AdapterError>,
    pub deadline_exceeded: bool,
    pub final_replay_passed: bool,
    pub caller_supplied_base: bool,
    pub complete_cost_accounting: bool,
}

/// The coefficient stream is the same SplitMix64 mapping as the original
/// native `solve`; modulo reduction is retained and is not advertised as exact
/// uniform sampling. Every attempted (a,b) and its public point are retained.
fn next(state: &mut u64, n: u64) -> u64 {
    *state = state.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    (z ^ (z >> 31)) % n
}

/// Full signed-relation/rank/recovery loop, bounded by both public-query and
/// time limits. The target scalar is not an input. Diagnostic durations and
/// partial operation counters do not establish a complete-cost comparison.
pub fn solve_public(
    e: Curve,
    base: &[(u64, u64)],
    q: Point,
    seed: u64,
    max_queries: usize,
    total_seconds: u64,
    per_query: Limits,
) -> Result<SolveRecord, AdapterError> {
    solve_public_with_form(
        e,
        base,
        q,
        seed,
        max_queries,
        total_seconds,
        per_query,
        ExtractionForm::ExpandedLex,
    )
}

/// The same bounded public-query loop with an explicit extraction form.
/// Each attempt records the presentation and monomial order in its diagnostics.
#[allow(clippy::too_many_arguments)]
pub fn solve_public_with_form(
    e: Curve,
    base: &[(u64, u64)],
    q: Point,
    mut seed: u64,
    max_queries: usize,
    total_seconds: u64,
    per_query: Limits,
    form: ExtractionForm,
) -> Result<SolveRecord, AdapterError> {
    // Start before setup. Checks are cooperative; no hard wall/RSS guarantee
    // is claimed by this API, and the caller-supplied base is still excluded.
    let deadline = Instant::now() + Duration::from_secs(total_seconds);
    let mut consumer = RelationAdapter::new(e, base.to_vec(), q)?;
    let setup = consumer.setup_arithmetic_counts();
    let mut ar = Arithmetic::default();
    let mut attempts = Vec::new();
    let mut stop = "query_limit";
    let mut terminal_error = None;
    for _ in 0..max_queries {
        if consumer.scalar().is_some() {
            stop = "solved";
            break;
        }
        if Instant::now() >= deadline {
            stop = "deadline";
            break;
        }
        let a = next(&mut seed, e.n);
        let b = next(&mut seed, e.n);
        let ag = ar.scalar(e, a, Some(e.g));
        let bq = ar.scalar(e, b, q);
        let point = ar.add(e, ag, bq);
        // Even when public-point construction exhausts the budget, preserve
        // the spent query with a Deadline extraction record.
        let extraction = extract_with_form_until(e, base, point, per_query, deadline, form);
        let mut receipts = Vec::new();
        let mut acceptance_error = None;
        for relation in &extraction.relations {
            if consumer.scalar().is_some() || Instant::now() >= deadline {
                break;
            }
            match consumer.accept(relation, a, b) {
                Ok(receipt) => receipts.push(receipt),
                Err(error) => {
                    acceptance_error = Some(error.clone());
                    terminal_error = Some(error);
                    break;
                }
            }
        }
        attempts.push(Attempt {
            a,
            b,
            point,
            extraction,
            receipts,
            acceptance_error,
        });
        if terminal_error.is_some() {
            stop = "relation_rejected";
            break;
        }
        if consumer.scalar().is_some() {
            stop = "solved";
            break;
        }
        if Instant::now() >= deadline {
            stop = "deadline";
            break;
        }
    }
    if consumer.scalar().is_some() {
        stop = "solved";
    }
    let mut scalar = consumer.scalar();
    let mut final_replay_passed = false;
    if let Some(d) = scalar {
        if ar.scalar(e, d, Some(e.g)) != q {
            terminal_error = Some(AdapterError::ScalarReplayFailed);
            scalar = None;
            stop = "scalar_replay_failed";
        } else {
            final_replay_passed = true;
        }
    }
    let deadline_exceeded = Instant::now() >= deadline;
    if final_replay_passed && deadline_exceeded {
        stop = "solved_after_deadline";
    }
    Ok(SolveRecord {
        status: stop.into(),
        scalar,
        target: q,
        attempts,
        rank: consumer.rank(),
        query_arithmetic: ar.counts,
        consumer_arithmetic: consumer.arithmetic_counts(),
        consumer_setup_arithmetic: setup,
        rank_work: consumer.work(),
        terminal_error,
        deadline_exceeded,
        final_replay_passed,
        caller_supplied_base: true,
        complete_cost_accounting: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_f11_relations_recover_unknown_target() {
        let e = Curve {
            p: 11,
            a: 1,
            b: 6,
            n: 13,
            g: (2, 7),
        };
        let base = vec![(2, 4), (3, 5), (5, 2), (7, 2)];
        let q = Some((8, 3));
        let result = solve_public(e, &base, q, 0x525235, 64, 30, Limits::default()).unwrap();
        println!(
            "RR5_INTEGRATED_F11 {}",
            serde_json::to_string(&result).unwrap()
        );
        assert_eq!(result.status, "solved");
        let mut ar = Arithmetic::default();
        assert_eq!(ar.scalar(e, result.scalar.unwrap(), Some(e.g)), q);
        assert!(result
            .attempts
            .iter()
            .any(|a| a.receipts.iter().any(|r| r.rank_after > r.rank_before)));
        assert!(result.attempts.iter().all(|a| a
            .extraction
            .diagnostics
            .forbidden_branches_executed
            == 0));
    }

    #[test]
    fn public_f11_grevlex_cofactor_relations_recover_unknown_target() {
        let e = Curve {
            p: 11,
            a: 1,
            b: 6,
            n: 13,
            g: (2, 7),
        };
        let base = vec![(2, 4), (3, 5), (5, 2), (7, 2)];
        let q = Some((8, 3));
        let result = solve_public_with_form(
            e,
            &base,
            q,
            0x525235,
            64,
            30,
            Limits::default(),
            ExtractionForm::CofactorGrevlex,
        )
        .unwrap();
        println!(
            "RR5_INTEGRATED_F11_GREVLEX_COFACTOR {}",
            serde_json::to_string(&result).unwrap()
        );
        assert_eq!(result.status, "solved");
        assert_eq!(result.scalar, Some(3));
        assert!(result.final_replay_passed);
        assert!(!result.deadline_exceeded);
        assert!(result.terminal_error.is_none());
        let mut ar = Arithmetic::default();
        assert_eq!(ar.scalar(e, result.scalar.unwrap(), Some(e.g)), q);
        assert!(result
            .attempts
            .iter()
            .any(|a| a.receipts.iter().any(|r| r.rank_after > r.rank_before)));
        let mut traversals = 0usize;
        for attempt in &result.attempts {
            assert!(attempt.acceptance_error.is_none());
            let diagnostics = &attempt.extraction.diagnostics;
            assert_eq!(diagnostics.presentation, "monic_cofactor");
            assert_eq!(diagnostics.monomial_order, "grevlex");
            assert_eq!(diagnostics.forbidden_branches_executed, 0);
            assert_eq!(diagnostics.forbidden_fallbacks_refused, 0);
            assert_eq!(diagnostics.rejected_assignments, 0);
            if let Some(trace) = &attempt.extraction.traversal {
                traversals += 1;
                assert!(trace.complete);
                assert!(!trace.timed_out && !trace.trace_stopped && !trace.stopped_early);
                assert!(trace.trace_error.is_none());
                assert_eq!(trace.full_field_branches, 0);
                assert_eq!(trace.domain_evaluations, 0);
                assert_eq!(trace.candidates_rejected, 0);
                assert!(trace.timings.is_none());
            }
        }
        assert!(traversals > 0);
    }
}
