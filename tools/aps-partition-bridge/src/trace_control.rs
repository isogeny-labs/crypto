use crypto_lib::cryptanalysis::f4_fp::{self, F4Options, Ordering, Poly, TracePool, TraceStats};
use serde::Serialize;
use std::{collections::BTreeSet, time::Duration};

#[derive(Serialize)]
pub struct CoefficientCase {
    pub coefficients: [u64; 3],
    pub expected: Vec<Vec<u64>>,
    pub observed: Vec<Vec<u64>>,
    pub complete: bool,
    pub timed_out: bool,
}

#[derive(Serialize)]
pub struct ControlReport {
    pub raw_trace_damage_refused: bool,
    pub poisoned_pool_fallback_tested: bool,
    pub cases: Vec<CoefficientCase>,
    pub stats: TraceStats,
    pub complete_and_equal: bool,
}

fn input(a: u64, b: u64, c: u64) -> Vec<Poly> {
    let mut polys = vec![
        vec![(vec![1, 1], 1)],
        vec![(vec![2, 0], 1)],
        vec![(vec![1, 0], 1), (vec![0, 1], 1)],
    ];
    for (poly, constant) in polys.iter_mut().zip([a, b, c]) {
        if constant != 0 {
            poly.push((vec![0, 0], constant));
        }
    }
    polys
}

pub fn run() -> Result<ControlReport, String> {
    let simple = vec![
        vec![(vec![1, 1], 1), (vec![0, 0], 4)],
        vec![(vec![2, 0], 1), (vec![0, 0], 4)],
    ];
    let opts = F4Options::new(Ordering::Grevlex, 8).with_budget(Duration::from_secs(2));
    let (recorded, mut trace) = f4_fp::f4_record(&simple, 2, 5, &opts);
    if recorded.timed_out || trace.input_lms.is_empty() {
        return Err("raw trace control did not finish recording".into());
    }
    trace.input_lms[0][0] += 1;
    let bad = f4_fp::f4_replay(&simple, 2, 5, &opts, &trace);
    let mut pool = TracePool::new("partition-bridge:fixed-coefficient-controls", 8, 100000)?;
    let mut cases = Vec::new();
    let mut correct = bad.trace_mismatch;
    // Fixed full grid, never expanded in response to refusal/acceptance counts.
    for a in 0u64..3 {
        for b in 0u64..3 {
            for c in 0u64..3 {
                let mut expected = BTreeSet::new();
                for x in 0u64..5 {
                    for y in 0u64..5 {
                        if (x * y + a).is_multiple_of(5)
                            && (x * x + b).is_multiple_of(5)
                            && (x + y + c).is_multiple_of(5)
                        {
                            expected.insert(vec![x, y]);
                        }
                    }
                }
                let mut roots = BTreeSet::new();
                let report = f4_fp::solve_visit_reusing(
                    &input(a, b, c),
                    2,
                    5,
                    &F4Options::new(Ordering::Grevlex, 8).with_budget(Duration::from_secs(2)),
                    &[Some((0..5).collect()), Some((0..5).collect())],
                    &mut pool,
                    |x| {
                        roots.insert(x.to_vec());
                        false
                    },
                )?;
                correct &= report.complete && !report.timed_out && roots == expected;
                cases.push(CoefficientCase {
                    coefficients: [a, b, c],
                    expected: expected.into_iter().collect(),
                    observed: roots.into_iter().collect(),
                    complete: report.complete,
                    timed_out: report.timed_out,
                });
            }
        }
    }
    Ok(ControlReport {
        raw_trace_damage_refused: bad.trace_mismatch,
        poisoned_pool_fallback_tested: false,
        cases,
        stats: pool.stats(),
        complete_and_equal: correct,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coefficient_family_replay_matches_literal_roots_and_raw_damage_is_refused() {
        let report = run().unwrap();
        assert!(report.complete_and_equal);
        assert!(report.raw_trace_damage_refused);
        assert!(!report.poisoned_pool_fallback_tested);
        assert_eq!(report.cases.len(), 27);
        assert!(report.stats.replay_attempts > 0);
    }

    #[test]
    fn expired_visitor_is_not_a_complete_empty_root_set() {
        let result = f4_fp::solve_visit(
            &input(1, 1, 1),
            2,
            5,
            &F4Options::new(Ordering::Grevlex, 8).with_budget(Duration::ZERO),
            &[Some((0..5).collect()), Some((0..5).collect())],
            |_| false,
        );
        let report = result.unwrap();
        assert!(!report.complete && report.timed_out);
    }
}
