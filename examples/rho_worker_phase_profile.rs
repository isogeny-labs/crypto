//! Deterministic phase trace of the actual collaborative rho walk.
//! Local times are diagnostics; qualified comparisons require the isolated runner.

use crypto_lib::cryptanalysis::pollard_collab::job::{JobContext, JobSpec};
use crypto_lib::cryptanalysis::pollard_collab::walk::{run_walker, step, DpRecord, WalkerOutcome};
use crypto_lib::ecc::curve::CurveParams;
use crypto_lib::ecc::point::Point;
use num_bigint::BigUint;
use serde_json::json;
use std::time::Instant;

fn hex(v: &BigUint) -> String {
    format!("{v:x}")
}

fn trace_walker(ctx: &JobContext, i: u64) -> (WalkerOutcome, u128, u128, u128, u64) {
    assert!(
        !ctx.spec.negation_map,
        "this profile fixes negation_map=false"
    );
    let begin = Instant::now();
    let start = Instant::now();
    let (mut a, mut b, mut r) = ctx.walker_start(i);
    let start_ns = start.elapsed().as_nanos();
    let mut step_ns = 0;
    let mut steps = 0;
    let mut last: Option<Point> = None;
    let outcome = loop {
        if ctx.is_dp(&r) {
            let Point::Affine { x, y } = &r else {
                panic!("identity cannot be distinguished");
            };
            break WalkerOutcome::Dp(DpRecord {
                walker: i,
                steps,
                x: hex(&x.value),
                y: hex(&y.value),
                a: hex(&a),
                b: hex(&b),
            });
        }
        if steps >= ctx.step_cap {
            break WalkerOutcome::DeadTrail { steps };
        }
        let t = Instant::now();
        let (np, na, nb) = step(ctx, &r, &a, &b, last.as_ref());
        step_ns += t.elapsed().as_nanos();
        last = Some(std::mem::replace(&mut r, np));
        a = na;
        b = nb;
        steps += 1;
    };
    let total_ns = begin.elapsed().as_nanos();
    assert_eq!(
        outcome,
        run_walker(ctx, i),
        "instrumented walk changed state"
    );
    (outcome, start_ns, step_ns, total_ns, steps)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let walkers: u64 = args.get(1).map_or(Ok(8), |v| v.parse()).expect("walkers");
    let cap: u64 = args.get(2).map_or(Ok(128), |v| v.parse()).expect("cap");
    let dp_bits: u8 = args.get(3).map_or(Ok(8), |v| v.parse()).expect("dp_bits");
    assert!(walkers > 0 && cap > 0 && dp_bits < 64);

    let curve = CurveParams::secp256k1();
    let q = curve
        .generator()
        .scalar_mul(&BigUint::from(7u32), &curve.a_fe());
    let mut spec = JobSpec::new(&curve, &q, "rho-phase-profile-v1", 0x71cc_2026_1017).unwrap();
    spec.dp_bits = dp_bits;
    spec.num_branches = 32;
    spec.negation_map = false;
    spec.max_steps_per_walker = cap;
    let build_start = Instant::now();
    let (ctx, build) = JobContext::new_profiled(spec.clone()).unwrap();
    let build_total_ns = build_start.elapsed().as_nanos();
    let plain = JobContext::new(spec).unwrap();
    assert_eq!(ctx.job_id, plain.job_id);
    assert_eq!(ctx.branches.len(), plain.branches.len());
    for (a, b) in ctx.branches.iter().zip(&plain.branches) {
        assert_eq!(
            (a.u.clone(), a.v.clone(), a.point.clone()),
            (b.u.clone(), b.v.clone(), b.point.clone())
        );
    }

    let mut starts_ns = 0;
    let mut steps_ns = 0;
    let mut walk_total_ns = 0;
    let mut step_count = 0;
    let mut dp_count = 0;
    let mut transcript = blake3::Hasher::new();
    for i in 0..walkers {
        let (outcome, start_ns, step_ns, total_ns, steps) = trace_walker(&ctx, i);
        starts_ns += start_ns;
        steps_ns += step_ns;
        walk_total_ns += total_ns;
        step_count += steps;
        if matches!(&outcome, WalkerOutcome::Dp(_)) {
            dp_count += 1;
        }
        transcript.update(&i.to_le_bytes());
        transcript.update(
            serde_json::to_string(&format!("{outcome:?}"))
                .unwrap()
                .as_bytes(),
        );
    }
    let other_walk_ns = walk_total_ns.saturating_sub(starts_ns + steps_ns);
    let profiled_ns = build.branch_table_ns + starts_ns + steps_ns + other_walk_ns;
    let fraction = if profiled_ns == 0 {
        0.0
    } else {
        (build.branch_table_ns + starts_ns) as f64 / profiled_ns as f64
    };
    let result = json!({
        "schema": "rho-worker-phase-profile-v1",
        "tier": "local_diagnostic",
        "curve": "secp256k1",
        "job_id": ctx.job_id,
        "negation_map": false,
        "walkers": walkers,
        "step_cap": cap,
        "dp_bits": dp_bits,
        "branch_points": build.branch_points,
        "starts": walkers,
        "walk_steps": step_count,
        "distinguished_points": dp_count,
        "build_validation_ns": build.validation_ns,
        "branch_table_ns": build.branch_table_ns,
        "build_finalization_ns": build.finalization_ns,
        "build_total_ns": build_total_ns,
        "walker_starts_ns": starts_ns,
        "walk_step_calls_ns": steps_ns,
        "other_walk_ns": other_walk_ns,
        "walk_total_ns": walk_total_ns,
        "restart_and_setup_fraction_of_profiled_ns": fraction,
        "independent_replay": "passed_for_every_walker",
        "transcript_blake3": transcript.finalize().to_hex().to_string(),
    });
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
