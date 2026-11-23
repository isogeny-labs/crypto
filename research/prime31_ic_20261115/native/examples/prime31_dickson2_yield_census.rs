//! Prime31 original-Rust two-summand coverage census, not an ECDLP solver.
//! Reuses the existing prime31-algebraic-ic Curve/Arithmetic/Base implementation.
use prime31_algebraic_ic::{check_curve, Arithmetic, Base, PRIME31};
use std::collections::HashSet;
use std::time::Instant;

fn main() -> Result<(), String> {
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar)?;
    let fb = Base::build(PRIME31, 10, &mut ar)?;
    assert_eq!(fb.values.len(), 1024, "unfrozen Dickson depth-10 support");
    assert_eq!(fb.points.len(), 509, "factor-base definition drift");

    let mut signed = Vec::with_capacity(2 * fb.points.len());
    for &point in &fb.points {
        let q = Some(point);
        signed.push(q);
        signed.push(ar.neg(PRIME31, q));
    }
    assert_eq!(signed.len(), 1018);
    let mut all: HashSet<Option<(u64, u64)>> = HashSet::with_capacity(520_000);
    let started = Instant::now();
    let mut pairs: u64 = 0;
    let mut infinity: u64 = 0;
    for i in 0..signed.len() {
        for j in i..signed.len() {
            let sum = ar.add(PRIME31, signed[i], signed[j]);
            if sum.is_none() {
                infinity += 1;
            } else {
                assert!(ar.on_curve(PRIME31, sum), "pair arithmetic left curve");
            }
            all.insert(sum);
            pairs += 1;
        }
    }
    let distinct = all.len() as u64;
    assert_eq!(pairs, 518_671);
    assert_eq!(infinity, 509);
    assert_eq!(distinct, 518_085);
    assert!(all.contains(&None));

    let duration = started.elapsed();
    let rate = distinct as f64 / PRIME31.n as f64;
    let nontrivial_records = pairs - infinity;
    let optimistic_full_rank_calls =
        (fb.points.len() as f64 * PRIME31.n as f64) / nontrivial_records as f64;
    // This is only a cross-run *sensitivity* versus a historical shared BSGS
    // result; it is not a measured same-runner complete-solver comparison.
    let max_ms_per_call = 170.113 / optimistic_full_rank_calls;
    println!(
        "{{\"curve_p\":{},\"curve_n\":{},\"depth\":10,\"base_points\":{},\"signed_base\":{},\"unordered_pairs\":{},\"distinct_pair_sums\":{},\"inverse_pair_records\":{},\"uniform_hit_probability\":{:.12},\"optimistic_full_rank_queries\":{:.3},\"cross_run_max_ms_per_call\":{:.10},\"census_ms\":{:.3},\"classification\":\"exact_yield_census_not_complete_solver\"}}",
        PRIME31.p, PRIME31.n, fb.points.len(), signed.len(), pairs, distinct,
        infinity, rate, optimistic_full_rank_calls, max_ms_per_call,
        duration.as_secs_f64() * 1000.0,
    );
    Ok(())
}
