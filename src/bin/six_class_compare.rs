//! Paired, bounded native rho comparison on one frozen job and target.
//!
//! Every trial uses baseline / six-class / baseline order. The two baseline
//! arms are an A/A drift control. Timings include fresh branch-table setup,
//! walk, in-memory DP lookup, collision processing and answer checks. They
//! are host observations, never Curve Smash instruction-equivalent scores.

use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use crypto_lib::cryptanalysis::pollard_collab::job::JobSpec;
use crypto_lib::cryptanalysis::pollard_collab::state::solve_collision;
use crypto_lib::cryptanalysis::pollard_collab::walk::{run_walker, DpRecord, WalkerOutcome};
use crypto_lib::cryptanalysis::six_class_rho::{SixClassJob, WALK_VERSION};
use crypto_lib::ecc::point::Point;
use crypto_lib::hash::sha256;
use num_bigint::BigUint;
use serde::Serialize;

#[derive(Serialize)]
struct Arm {
    method: &'static str,
    setup_ns: u128,
    campaign_ns: u128,
    total_ns: u128,
    walkers: u64,
    class_visits: u64,
    distinguished: u64,
    dead_trails: u64,
    collisions: u64,
    short_cycle_escapes: Option<u64>,
    dp_entries: usize,
    log: Option<String>,
    first_dp_verified: bool,
}

fn baseline(spec: &JobSpec, max_walkers: u64) -> Result<Arm, String> {
    let setup = Instant::now();
    let mut spec = spec.clone();
    spec.negation_map = true;
    let ctx = spec.build()?;
    let setup_ns = setup.elapsed().as_nanos();
    let campaign = Instant::now();
    let mut seen: HashMap<String, DpRecord> = HashMap::new();
    let mut walkers = 0;
    let mut class_visits = 0;
    let mut distinguished = 0;
    let mut dead_trails = 0;
    let mut collisions = 0;
    let mut first_dp_verified = false;
    let mut log = None;
    for walker in 0..max_walkers {
        walkers += 1;
        match run_walker(&ctx, walker) {
            WalkerOutcome::Dp(dp) => {
                class_visits += dp.steps + 1;
                distinguished += 1;
                let point = dp.point(&ctx)?;
                let key = ctx.dp_key(&point);
                if !first_dp_verified {
                    dp.verify(&ctx)?;
                    first_dp_verified = true;
                }
                if let Some(previous) = seen.insert(key, dp.clone()) {
                    collisions += 1;
                    previous.verify(&ctx)?;
                    dp.verify(&ctx)?;
                    let (a1, b1) = previous.coefficients()?;
                    let (a2, b2) = dp.coefficients()?;
                    let y1 = match previous.point(&ctx)? {
                        Point::Affine { y, .. } => y.value,
                        Point::Infinity => return Err("infinity DP".into()),
                    };
                    let y2 = match point {
                        Point::Affine { y, .. } => y.value,
                        Point::Infinity => return Err("infinity DP".into()),
                    };
                    if let Some(x) = solve_collision(&ctx, (&a1, &b1, &y1), (&a2, &b2, &y2)) {
                        log = Some(format!("{x:x}"));
                        break;
                    }
                }
            }
            WalkerOutcome::DeadTrail { steps } => {
                class_visits += steps;
                dead_trails += 1;
            }
        }
    }
    let campaign_ns = campaign.elapsed().as_nanos();
    Ok(Arm {
        method: "negation-rho-v1",
        setup_ns,
        campaign_ns,
        total_ns: setup_ns + campaign_ns,
        walkers,
        class_visits,
        distinguished,
        dead_trails,
        collisions,
        short_cycle_escapes: None,
        dp_entries: seen.len(),
        log,
        first_dp_verified,
    })
}

fn sixclass(job: &SixClassJob, max_walkers: u64) -> Result<Arm, String> {
    let setup = Instant::now();
    let ctx = job.build()?;
    let setup_ns = setup.elapsed().as_nanos();
    let campaign = Instant::now();
    let result = ctx.campaign(max_walkers);
    if let Some(dp) = &result.first_dp {
        ctx.verify_record(dp)?;
    }
    if let Some(log) = &result.log {
        let scalar = BigUint::parse_bytes(log.as_bytes(), 16).ok_or("invalid log hex")?;
        if ctx.base.g.scalar_mul_vartime(&scalar, &ctx.base.a) != ctx.base.q {
            return Err("six-class log failed independent point check".into());
        }
    }
    let campaign_ns = campaign.elapsed().as_nanos();
    Ok(Arm {
        method: WALK_VERSION,
        setup_ns,
        campaign_ns,
        total_ns: setup_ns + campaign_ns,
        walkers: result.walkers,
        class_visits: result.class_visits,
        distinguished: result.distinguished,
        dead_trails: result.dead_trails,
        collisions: result.collision_attempts,
        short_cycle_escapes: Some(result.short_cycle_escapes),
        dp_entries: result.unique_dp_entries,
        log: result.log,
        first_dp_verified: result.first_dp.is_some(),
    })
}

#[derive(Serialize)]
struct Trial {
    index: u32,
    baseline_before: Arm,
    six_class: Arm,
    baseline_after: Arm,
}

fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let [path, max_walkers, trials] = args.as_slice() else {
        return Err("usage: six_class_compare JOB.json MAX_WALKERS TRIALS".into());
    };
    let max_walkers: u64 = max_walkers.parse().map_err(|_| "invalid MAX_WALKERS")?;
    let trials: u32 = trials.parse().map_err(|_| "invalid TRIALS")?;
    if !(1..=1_000_000).contains(&max_walkers) || !(1..=100).contains(&trials) {
        return Err("MAX_WALKERS or TRIALS outside bounded range".into());
    }
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    let job: SixClassJob = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let mut rows = Vec::with_capacity(trials as usize);
    for index in 0..trials {
        rows.push(Trial {
            index,
            baseline_before: baseline(&job.curve, max_walkers)?,
            six_class: sixclass(&job, max_walkers)?,
            baseline_after: baseline(&job.curve, max_walkers)?,
        });
    }
    println!("{}", serde_json::to_string_pretty(&serde_json::json!({
        "schema": "crypto.sixclass.paired-native.v1",
        "job_sha256": hex::encode(sha256(&bytes)),
        "six_class_job_id": job.id(),
        "baseline": "negation-rho-v1",
        "max_walkers": max_walkers,
        "trials": rows,
        "scope": "bounded native BigUint A/B/A observation; method-call counts and wall times, not production instruction cost",
        "unpriced": ["field arithmetic primitives", "allocator traffic", "memory high-water", "network transport", "cross-host coordination", "success amplification"]
    })).map_err(|e| e.to_string())?);
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("six_class_compare: {error}");
        std::process::exit(2);
    }
}
