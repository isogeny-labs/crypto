//! Frozen one-word prime controls for the blocked-BSGS/rho wall study.
//!
//! These adapters deliberately expose no tuning parameters.  They run only
//! on the five public, registered fixtures frozen by the study and reproduce
//! its charged cold boundaries.  The native implementations do not expose a
//! complete field-operation ledger, so their exact native counters are kept
//! as `*_uncharged` work and the GAE total remains an explicit lower bound.

use std::collections::BTreeMap;
use std::time::Instant;

use num_bigint::BigUint;
use serde_json::json;

use crate::cryptanalysis::bsgs_fast::{
    BsgsFast, BsgsFastPlan, FastCurve as BsgsCurve, FastPoint as BsgsPoint, StepKernel,
};
use crate::cryptanalysis::ecbench::generic::ceil_sqrt;
use crate::cryptanalysis::ic_boundary::PrimeInstance;
use crate::cryptanalysis::ic_measurement::{self as measurement, Phase};
use crate::cryptanalysis::prime_fast::{self, RhoConfig, RhoOpCounts};
use crate::ecc::curve::CurveParams;

use super::exact_scalar::ExactScalar;
use super::methods::{unpriced_of, BatchItemReport, PhaseRecord, SolveReport};
use super::workload::CurveFacts;

/// Frozen paired blocked-BSGS control.
pub const BSGS_ID: &str = "bsgs.prime_paired_v1";
/// Frozen tuned single-thread native-rho control.
pub const RHO_ID: &str = "rho.prime_fast_v1";
/// Frozen paired BSGS with one table shared by exactly 64 ordered targets.
pub const BSGS_BATCH64_ID: &str = "bsgs.prime_paired_batch64_v1";
/// Frozen sequence of exactly 64 independent one-thread rho solves.
pub const RHO_BATCH64_ID: &str = "rho.prime_fast_batch64_v1";

#[derive(Clone, Copy)]
struct Fixture {
    slug: &'static str,
    p: u64,
    a: u64,
    b: u64,
    gx: u64,
    gy: u64,
    n: u64,
}

const FIXTURES: [Fixture; 5] = [
    Fixture {
        slug: "icv1-fp24-tm5099-a6f137c6",
        p: 16_777_199,
        a: 16_777_196,
        b: 31,
        gx: 2,
        gy: 5_412_647,
        n: 16_782_299,
    },
    Fixture {
        slug: "icv1-fp28-t28201-65099091",
        p: 268_435_399,
        a: 268_435_396,
        b: 3,
        gx: 1,
        gy: 1,
        n: 268_407_199,
    },
    Fixture {
        slug: "icv1-fp32-t7319-90da43e9",
        p: 4_294_967_291,
        a: 4_294_967_288,
        b: 31,
        gx: 2,
        gy: 2_120_051_858,
        n: 4_294_959_973,
    },
    Fixture {
        slug: "icv1-fp36-t454193-44a20818",
        p: 68_719_476_731,
        a: 68_719_476_728,
        b: 33,
        gx: 2,
        gy: 51_741_851_039,
        n: 68_719_022_539,
    },
    Fixture {
        slug: "icv1-fp40-tm506303-75549727",
        p: 649_522_587_953,
        a: 552_810_359_288,
        b: 555_863_716_954,
        gx: 549_251_065_028,
        gy: 609_044_633_619,
        n: 649_523_094_257,
    },
];

pub(super) fn is_method(id: &str) -> bool {
    matches!(id, BSGS_ID | RHO_ID | BSGS_BATCH64_ID | RHO_BATCH64_ID)
}

pub(super) fn is_batch_method(id: &str) -> bool {
    matches!(id, BSGS_BATCH64_ID | RHO_BATCH64_ID)
}

fn fixture(slug: &str) -> Option<Fixture> {
    FIXTURES.iter().copied().find(|f| f.slug == slug)
}

/// Refuse every curve except the five exact, public, cofactor-one fixtures.
pub(super) fn applicable(id: &str, inst: &PrimeInstance, facts: &CurveFacts) -> Result<(), String> {
    if !is_method(id) {
        return Err(format!("`{id}` is not a frozen prime control"));
    }
    let expected = fixture(&facts.slug).ok_or_else(|| {
        format!(
            "`{id}` is admitted only on the five fixed blocked-BSGS/rho fixtures; {} is outside that panel",
            facts.slug
        )
    })?;
    let exact = inst.curve.p == expected.p
        && inst.curve.a == expected.a
        && inst.curve.b == expected.b
        && inst.group_order == expected.n
        && inst.r == expected.n
        && inst.cofactor == 1
        && inst.generator == (expected.gx, expected.gy);
    if !exact {
        return Err(format!(
            "`{id}` fixture {} does not match its frozen coefficients, generator, order, and cofactor",
            facts.slug
        ));
    }
    Ok(())
}

fn bsgs_curve(inst: &PrimeInstance) -> Result<BsgsCurve, String> {
    let h = u32::try_from(inst.cofactor).map_err(|_| "prime fixture cofactor exceeds u32")?;
    let params = CurveParams {
        name: "ecbench-fixed-prime-control",
        p: BigUint::from(inst.curve.p),
        a: BigUint::from(inst.curve.a),
        b: BigUint::from(inst.curve.b),
        gx: BigUint::from(inst.generator.0),
        gy: BigUint::from(inst.generator.1),
        n: BigUint::from(inst.r),
        h,
    };
    BsgsCurve::from_params(&params).ok_or_else(|| {
        "frozen paired BSGS requires a valid single-word short-Weierstrass fixture".into()
    })
}

fn elapsed_ns(start: Instant) -> u64 {
    u64::try_from(start.elapsed().as_nanos()).unwrap_or(u64::MAX)
}

fn residual_ns(total: u64, attributed: &[u64], method: &str) -> Result<u64, String> {
    let sum = attributed
        .iter()
        .try_fold(0u64, |acc, &v| acc.checked_add(v))
        .ok_or_else(|| format!("{method} phase-wall sum overflowed"))?;
    total
        .checked_sub(sum)
        .ok_or_else(|| format!("{method} phase clocks exceeded their enclosing cold clock"))
}

fn timed_phase(name: &str, wall_ns: u64) -> PhaseRecord {
    PhaseRecord {
        name: name.into(),
        wall_ns: Some(wall_ns),
        ..Default::default()
    }
}

fn active_chains(total: u64, len: u64, capacity: usize) -> u64 {
    total
        .div_ceil(len.max(1))
        .min(u64::try_from(capacity).unwrap_or(u64::MAX))
}

fn solve_bsgs(inst: &PrimeInstance, tx: u64, ty: u64) -> Result<SolveReport, String> {
    let curve = bsgs_curve(inst)?;
    if tx >= inst.curve.p || ty >= inst.curve.p {
        return Err("paired-BSGS target coordinates are not canonical".into());
    }
    let target = BsgsPoint {
        x: curve.f.to_mont(tx),
        y: curve.f.to_mont(ty),
        infinity: false,
    };
    if !curve.on_curve(target) {
        return Err("paired-BSGS target is not on the frozen curve".into());
    }

    // Plan construction and representation conversion are fixture
    // preparation.  The frozen cold interval begins immediately before the
    // table allocation/build.
    let plan = BsgsFastPlan::new(inst.r, 0, true, 1, 8).with_kernel(StepKernel::Paired, 32);
    let table_bytes = plan.table_bytes();
    let plan_shape = (
        plan.neg_map,
        plan.width,
        plan.x0,
        plan.m,
        plan.stride,
        plan.giant_count,
        plan.table_bits,
        plan.workers,
        plan.chains_per_worker,
        plan.baby_len,
        plan.giant_len,
        plan.kernel.name(),
        plan.pair_k,
    );
    let baby_chains = active_chains(
        plan.m,
        plan.baby_len,
        plan.workers.saturating_mul(plan.chains_per_worker),
    );
    let giant_chains = active_chains(
        plan.giant_count,
        plan.giant_len,
        plan.workers.saturating_mul(plan.chains_per_worker),
    );
    let total_start = Instant::now();
    let build_start = Instant::now();
    let mut solver = BsgsFast::build(curve, plan);
    let build_ns = elapsed_ns(build_start);

    measurement::begin_online(Phase::RhoSolve);
    let search_start = Instant::now();
    let recovered = solver.solve(&target);
    let search_ns = elapsed_ns(search_start);
    let verification_start = Instant::now();
    let evaluator_correct = recovered
        .map(|log| curve.scalar_mul(curve.g, log) == target)
        .unwrap_or(false);
    let verification_ns = elapsed_ns(verification_start);
    measurement::end_online();

    let stats = solver.stats();
    let drop_start = Instant::now();
    drop(solver);
    let drop_ns = elapsed_ns(drop_start);
    let total_ns = elapsed_ns(total_start);
    let residual_ns = residual_ns(
        total_ns,
        &[build_ns, search_ns, verification_ns, drop_ns],
        BSGS_ID,
    )?;

    // A reported kernel visit is the native solver's abstract BSGS step.
    // Candidate scalar replays and chain seeding are not exposed as complete
    // operation ledgers, so they remain visible and unpriced.
    let replay_gae = 1.5 * (inst.r as f64).log2();
    let mut phases = vec![
        PhaseRecord {
            name: "table_build".into(),
            adds: stats.baby_steps,
            gae: stats.baby_steps as f64,
            wall_ns: Some(build_ns),
            ..Default::default()
        },
        PhaseRecord {
            name: "search".into(),
            adds: stats.giant_steps,
            gae: stats.giant_steps as f64,
            wall_ns: Some(search_ns),
            ..Default::default()
        },
        PhaseRecord {
            name: "evaluator_verification".into(),
            scalar_mults: u64::from(recovered.is_some()),
            gae: if recovered.is_some() { replay_gae } else { 0.0 },
            wall_ns: Some(verification_ns),
            ..Default::default()
        },
        timed_phase("teardown", drop_ns),
        timed_phase("timer_residual", residual_ns),
    ];
    let total_gae = phases.iter().map(|p| p.gae).sum();
    let mut counters = BTreeMap::new();
    for (name, value) in [
        ("baby_steps", stats.baby_steps),
        ("giant_steps", stats.giant_steps),
        ("candidates", stats.candidates),
        ("false_candidates", stats.false_candidates),
        ("table_entries", stats.table_entries),
        ("table_bytes", table_bytes),
        ("plan_width", plan_shape.1),
        ("plan_x0", plan_shape.2),
        ("plan_baby_count", plan_shape.3),
        ("plan_stride", plan_shape.4),
        ("plan_giant_count", plan_shape.5),
        ("plan_table_bits", u64::from(plan_shape.6)),
        ("plan_workers", plan_shape.7 as u64),
        ("plan_chains_per_worker", plan_shape.8 as u64),
        ("plan_baby_chain_len", plan_shape.9),
        ("plan_giant_chain_len", plan_shape.10),
        ("plan_pair_k", plan_shape.12 as u64),
        ("plan_negation_map", u64::from(plan_shape.0)),
        (
            "baby_chain_seed_scalar_multiplications_uncharged",
            baby_chains,
        ),
        ("stride_scalar_multiplications_uncharged", 1),
        ("target_shift_scalar_multiplications_uncharged", 1),
        (
            "giant_chain_seed_scalar_multiplications_uncharged",
            giant_chains,
        ),
        ("candidate_matches_uncharged", stats.candidates),
    ] {
        counters.insert(name.into(), value);
    }
    let unpriced = unpriced_of(&counters);
    phases.shrink_to_fit();
    Ok(SolveReport {
        recovered: recovered.map(ExactScalar::from),
        exhausted: recovered.is_none(),
        phases,
        total_gae,
        automorphisms_used: 2,
        counters,
        unpriced,
        deterministic: true,
        nondeterminism: vec![],
        solve_wall_ns: total_ns,
        factor_base: None,
        detail: json!({
            "implementation": "cryptanalysis::bsgs_fast::BsgsFast",
            "plan": {
                "constructor": "BsgsFastPlan::new(r,0,true,1,8)",
                "kernel": plan_shape.11,
                "pair_k": plan_shape.12,
            },
            "cold_boundary": "table allocation/build through search, timed evaluator replay, and table deallocation",
            "evaluator_correct": evaluator_correct,
            "accounting": "one GAE per reported native chain visit plus the repository's 1.5*log2(r) evaluator scalar-replay convention; chain seeding and candidate replay remain unpriced",
        }),
        online: None,
        online_error: None,
        solver: None,
        field_ops: None,
    })
}

#[derive(Clone, Copy)]
struct RhoSnapshot {
    log: u64,
    total_steps: u64,
    distinguished_points: u64,
    wall_secs: f64,
    steps_per_sec: f64,
    threads: usize,
    walkers_per_thread: usize,
    dp_bits: u32,
    ops: RhoOpCounts,
}

fn prepare_rho_curve(
    inst: &PrimeInstance,
) -> Result<(prime_fast::FastCurve, prime_fast::Pt), String> {
    let curve = prime_fast::FastCurve::new(
        "ecbench-fixed-prime-control",
        inst.curve.p,
        inst.curve.a,
        inst.curve.b,
        inst.r,
    );
    let generator = curve.point(inst.generator.0, inst.generator.1);
    if !curve.is_on_curve(generator) {
        return Err("prime-fast rho generator is not on the frozen curve".into());
    }
    Ok((curve, generator))
}

fn prepare_rho_config(inst: &PrimeInstance, seed: u64) -> RhoConfig {
    let mut config = RhoConfig::tuned(inst.r);
    config.threads = 1;
    config.seed = seed;
    config.max_steps = 64u64.saturating_mul(ceil_sqrt(inst.r));
    config
}

fn solve_rho(inst: &PrimeInstance, tx: u64, ty: u64, seed: u64) -> Result<SolveReport, String> {
    if tx >= inst.curve.p || ty >= inst.curve.p {
        return Err("prime-fast rho target coordinates are not canonical".into());
    }
    let (curve, generator) = prepare_rho_curve(inst)?;
    let target = curve.point(tx, ty);
    if !curve.is_on_curve(target) {
        return Err("prime-fast rho target is not on the frozen curve".into());
    }
    let config = prepare_rho_config(inst, seed);
    solve_rho_prepared(inst, &curve, generator, target, &config, RHO_ID, true)
}

fn solve_rho_prepared(
    inst: &PrimeInstance,
    curve: &prime_fast::FastCurve,
    generator: prime_fast::Pt,
    target: prime_fast::Pt,
    config: &RhoConfig,
    boundary_id: &str,
    mark_online: bool,
) -> Result<SolveReport, String> {
    // Curve/point conversion and deterministic config construction are
    // fixture preparation.  The cold clock starts at rho_parallel itself.
    let total_start = Instant::now();
    if mark_online {
        measurement::begin_online(Phase::RhoSolve);
    }
    let solver_start = Instant::now();
    let result = prime_fast::rho_parallel(curve, generator, target, config);
    let solver_ns = elapsed_ns(solver_start);
    let verification_start = Instant::now();
    let evaluator_correct = result
        .as_ref()
        .map(|r| curve.mul(Some(generator), r.log) == Some(target))
        .unwrap_or(false);
    let verification_ns = elapsed_ns(verification_start);
    if mark_online {
        measurement::end_online();
    }
    let snapshot = result.as_ref().map(|r| RhoSnapshot {
        log: r.log,
        total_steps: r.total_steps,
        distinguished_points: r.distinguished_points,
        wall_secs: r.wall_secs,
        steps_per_sec: r.steps_per_sec,
        threads: r.threads,
        walkers_per_thread: r.walkers_per_thread,
        dp_bits: r.dp_bits,
        ops: r.ops,
    });
    let drop_start = Instant::now();
    drop(result);
    let drop_ns = elapsed_ns(drop_start);
    let total_ns = elapsed_ns(total_start);
    let residual_ns = residual_ns(
        total_ns,
        &[solver_ns, verification_ns, drop_ns],
        boundary_id,
    )?;

    let mul_gae = 1.5 * (inst.r as f64).log2();
    let (solver_adds, solver_doubles, collision_checks) = snapshot.map_or((0, 0, 0), |r| {
        (
            r.total_steps
                .saturating_add(r.ops.table_adds)
                .saturating_add(r.ops.initial_start_adds)
                .saturating_add(r.ops.restart_adds),
            r.ops
                .table_doubles
                .saturating_add(r.ops.initial_start_doubles)
                .saturating_add(r.ops.restart_doubles)
                .saturating_add(r.ops.escape_doubles),
            r.ops.collision_checks,
        )
    });
    let replay_count = u64::from(snapshot.is_some());
    let phases = vec![
        PhaseRecord {
            name: "rho_solver".into(),
            adds: solver_adds,
            doubles: solver_doubles,
            scalar_mults: collision_checks,
            gae: (solver_adds + solver_doubles) as f64 + collision_checks as f64 * mul_gae,
            wall_ns: Some(solver_ns),
            ..Default::default()
        },
        PhaseRecord {
            name: "evaluator_verification".into(),
            scalar_mults: replay_count,
            gae: replay_count as f64 * mul_gae,
            wall_ns: Some(verification_ns),
            ..Default::default()
        },
        timed_phase("teardown", drop_ns),
        timed_phase("timer_residual", residual_ns),
    ];
    let total_gae = phases.iter().map(|p| p.gae).sum();
    let mut counters = BTreeMap::new();
    for (name, value) in [
        ("config_threads", config.threads as u64),
        ("config_walkers", config.walkers as u64),
        ("config_table_bits", u64::from(config.table_bits)),
        ("config_dp_bits", u64::from(config.dp_bits)),
        ("config_negation", u64::from(config.negation)),
        ("config_seed", config.seed),
        ("config_max_steps", config.max_steps),
        ("config_look_ahead", u64::from(config.look_ahead)),
        ("config_cycle_window", config.cycle_window as u64),
        ("config_coefficient_bits", u64::from(config.coeff_bits)),
        ("config_stride_starts", u64::from(config.stride_starts)),
        ("config_table_entries", 1u64 << config.table_bits),
    ] {
        counters.insert(name.into(), value);
    }
    if let Some(r) = snapshot {
        for (name, value) in [
            ("steps", r.total_steps),
            ("distinguished_points", r.distinguished_points),
            ("result_threads", r.threads as u64),
            ("result_walkers_per_thread", r.walkers_per_thread as u64),
            ("result_dp_bits", u64::from(r.dp_bits)),
            ("table_additions", r.ops.table_adds),
            ("table_doublings", r.ops.table_doubles),
            ("initial_start_additions", r.ops.initial_start_adds),
            ("initial_start_doublings", r.ops.initial_start_doubles),
            ("restart_additions", r.ops.restart_adds),
            ("restart_doublings", r.ops.restart_doubles),
            ("escape_doublings", r.ops.escape_doubles),
            ("collision_checks", r.ops.collision_checks),
            ("look_ahead_additions", r.ops.look_ahead_adds),
            ("cycles_length_2", r.ops.cycles_2),
            ("cycles_length_4", r.ops.cycles_4),
            ("cycles_other_length", r.ops.cycles_other),
            ("walks_abandoned", r.ops.abandoned),
            ("walks_capped", r.ops.capped),
            ("walk_restarts", r.ops.restarts),
            ("dp_trail_steps", r.ops.dp_trail_steps),
            ("lost_steps", r.ops.lost_steps),
            ("in_flight_steps", r.ops.in_flight_steps),
            ("walk_partition_selections_uncharged", r.total_steps),
            (
                "walk_batch_inversions_uncharged",
                r.total_steps.div_ceil(r.walkers_per_thread.max(1) as u64),
            ),
            (
                "distinguished_point_table_events_uncharged",
                r.distinguished_points,
            ),
        ] {
            counters.insert(name.into(), value);
        }
    } else {
        // prime_fast returns Option and therefore exposes no counters on a
        // capped run.  Preserve that limitation instead of inventing zeroes.
        counters.insert("counts_lost_at_rho_cap_uncharged".into(), 1);
    }
    let unpriced = unpriced_of(&counters);
    Ok(SolveReport {
        recovered: snapshot.map(|r| ExactScalar::from(r.log)),
        exhausted: snapshot.is_none(),
        phases,
        total_gae,
        automorphisms_used: 2,
        counters,
        unpriced,
        deterministic: true,
        nondeterminism: vec![],
        solve_wall_ns: total_ns,
        factor_base: None,
        detail: json!({
            "implementation": "cryptanalysis::prime_fast::rho_parallel",
            "config": {
                "constructor": "RhoConfig::tuned(r)",
                "threads": config.threads,
                "walkers": config.walkers,
                "table_bits": config.table_bits,
                "dp_bits": config.dp_bits,
                "negation": config.negation,
                "seed": config.seed.to_string(),
                "max_steps": config.max_steps,
                "look_ahead": config.look_ahead,
                "cycle_window": config.cycle_window,
                "coefficient_bits": config.coeff_bits,
                "stride_starts": config.stride_starts,
            },
            "native_result": snapshot.map(|r| json!({
                "wall_secs": r.wall_secs,
                "steps_per_sec": r.steps_per_sec,
            })),
            "cold_boundary": "rho_parallel through timed evaluator replay and returned-state deallocation",
            "evaluator_correct": evaluator_correct,
            "accounting": "documented native group-operation counters plus the repository's 1.5*log2(r) scalar-replay convention; partitioning, batching, and DP-table work remain unpriced",
        }),
        online: None,
        online_error: None,
        solver: None,
        field_ops: None,
    })
}

fn parse_public_coordinate(value: &str) -> Result<u64, String> {
    let digits = value
        .strip_prefix("0x")
        .ok_or_else(|| "fixed-batch coordinate lacks 0x prefix".to_owned())?;
    if digits.is_empty()
        || digits.len() > 16
        || !digits
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err("fixed-batch coordinate is not bounded lowercase hexadecimal".into());
    }
    u64::from_str_radix(digits, 16)
        .map_err(|_| "fixed-batch coordinate does not fit u64".to_owned())
}

fn parse_public_target(inst: &PrimeInstance, target: &[String; 2]) -> Result<(u64, u64), String> {
    let tx = parse_public_coordinate(&target[0])?;
    let ty = parse_public_coordinate(&target[1])?;
    if tx >= inst.curve.p || ty >= inst.curve.p {
        return Err("fixed-batch target coordinates are not canonical".into());
    }
    Ok((tx, ty))
}

fn counter_delta(after: u64, before: u64, name: &str) -> Result<u64, String> {
    after
        .checked_sub(before)
        .ok_or_else(|| format!("fixed-batch {name} counter decreased"))
}

fn solve_bsgs_batch64(
    inst: &PrimeInstance,
    items: &[([String; 2], Option<u64>)],
) -> Result<(SolveReport, Vec<BatchItemReport>), String> {
    let curve = bsgs_curve(inst)?;
    let mut targets = Vec::with_capacity(items.len());
    for (target, _) in items {
        let (tx, ty) = parse_public_target(inst, target)?;
        let point = BsgsPoint {
            x: curve.f.to_mont(tx),
            y: curve.f.to_mont(ty),
            infinity: false,
        };
        if !curve.on_curve(point) {
            return Err("fixed-batch BSGS target is not on the frozen curve".into());
        }
        targets.push(point);
    }

    let plan = BsgsFastPlan::new(inst.r, 0, true, 1, 8).with_kernel(StepKernel::Paired, 32);
    let table_bytes = plan.table_bytes();
    let total_start = Instant::now();
    let build_start = Instant::now();
    let mut solver = BsgsFast::build(curve, plan);
    let build_ns = elapsed_ns(build_start);

    measurement::begin_online(Phase::RhoSolve);
    let mut item_reports = Vec::with_capacity(items.len());
    let mut recovered_count = 0u64;
    let mut evaluator_failures = 0u64;
    for (index, ((target_text, rho_seed), target)) in items.iter().zip(targets.iter()).enumerate() {
        let before = solver.stats();
        let search_start = Instant::now();
        let recovered = solver.solve(target);
        let search_ns = elapsed_ns(search_start);
        let verification_start = Instant::now();
        let evaluator_correct = recovered
            .map(|log| curve.scalar_mul(curve.g, log) == *target)
            .unwrap_or(false);
        let verification_ns = elapsed_ns(verification_start);
        let after = solver.stats();
        recovered_count = recovered_count.saturating_add(u64::from(recovered.is_some()));
        evaluator_failures =
            evaluator_failures.saturating_add(u64::from(recovered.is_some() && !evaluator_correct));
        let mut counters = BTreeMap::new();
        counters.insert(
            "giant_steps".into(),
            counter_delta(after.giant_steps, before.giant_steps, "giant_steps")?,
        );
        counters.insert(
            "candidates".into(),
            counter_delta(after.candidates, before.candidates, "candidates")?,
        );
        counters.insert(
            "false_candidates".into(),
            counter_delta(
                after.false_candidates,
                before.false_candidates,
                "false_candidates",
            )?,
        );
        item_reports.push(BatchItemReport {
            item: index as u64,
            target: target_text.clone(),
            rho_seed: rho_seed.map(|seed| format!("{seed:016x}")),
            recovered: recovered.map(ExactScalar::from),
            exhausted: recovered.is_none() || !evaluator_correct,
            evaluator_correct,
            solver_wall_ns: search_ns,
            verification_wall_ns: verification_ns,
            drop_wall_ns: 0,
            counters,
        });
    }
    measurement::end_online();

    let stats = solver.stats();
    let drop_start = Instant::now();
    drop(solver);
    let drop_ns = elapsed_ns(drop_start);
    let total_ns = elapsed_ns(total_start);
    let search_ns = item_reports.iter().try_fold(0u64, |total, item| {
        total
            .checked_add(item.solver_wall_ns)
            .ok_or_else(|| "fixed-batch BSGS search-wall sum overflowed".to_owned())
    })?;
    let verification_ns = item_reports.iter().try_fold(0u64, |total, item| {
        total
            .checked_add(item.verification_wall_ns)
            .ok_or_else(|| "fixed-batch BSGS verification-wall sum overflowed".to_owned())
    })?;
    let residual_ns = residual_ns(
        total_ns,
        &[build_ns, search_ns, verification_ns, drop_ns],
        BSGS_BATCH64_ID,
    )?;
    let replay_gae = 1.5 * (inst.r as f64).log2();
    let phases = vec![
        PhaseRecord {
            name: "table_build".into(),
            adds: stats.baby_steps,
            gae: stats.baby_steps as f64,
            wall_ns: Some(build_ns),
            ..Default::default()
        },
        PhaseRecord {
            name: "batch_search".into(),
            adds: stats.giant_steps,
            gae: stats.giant_steps as f64,
            wall_ns: Some(search_ns),
            ..Default::default()
        },
        PhaseRecord {
            name: "evaluator_verification".into(),
            scalar_mults: recovered_count,
            gae: recovered_count as f64 * replay_gae,
            wall_ns: Some(verification_ns),
            ..Default::default()
        },
        timed_phase("teardown", drop_ns),
        timed_phase("timer_residual", residual_ns),
    ];
    let total_gae = phases.iter().map(|phase| phase.gae).sum();
    let mut counters = BTreeMap::new();
    for (name, value) in [
        ("batch_size", 64),
        ("table_builds", 1),
        (
            "successful_items",
            item_reports.iter().filter(|item| !item.exhausted).count() as u64,
        ),
        ("evaluator_failures", evaluator_failures),
        ("baby_steps", stats.baby_steps),
        ("giant_steps", stats.giant_steps),
        ("candidates", stats.candidates),
        ("false_candidates", stats.false_candidates),
        ("table_entries", stats.table_entries),
        ("table_bytes", table_bytes),
        ("candidate_matches_uncharged", stats.candidates),
        ("batch_public_target_conversions_uncharged", 64),
    ] {
        counters.insert(name.into(), value);
    }
    let unpriced = unpriced_of(&counters);
    let exhausted = item_reports.iter().any(|item| item.exhausted);
    let detail_items = item_reports.clone();
    Ok((
        SolveReport {
            recovered: None,
            exhausted,
            phases,
            total_gae,
            automorphisms_used: 2,
            counters,
            unpriced,
            deterministic: true,
            nondeterminism: vec![],
            solve_wall_ns: total_ns,
            factor_base: None,
            detail: json!({
                "implementation": "cryptanalysis::bsgs_fast::BsgsFast shared-table batch",
                "batch_size": 64,
                "table_builds": 1,
                "plan": {
                    "constructor": "BsgsFastPlan::new(r,0,true,1,8)",
                    "kernel": "paired",
                    "pair_k": 32,
                },
                "batch_items": detail_items,
                "cold_boundary": "one table allocation/build through all 64 ordered searches, evaluator replays, and table deallocation",
            }),
            online: None,
            online_error: None,
            solver: None,
            field_ops: None,
        },
        item_reports,
    ))
}

fn checked_counter_add(
    counters: &mut BTreeMap<String, u64>,
    name: &str,
    value: u64,
) -> Result<(), String> {
    let entry = counters.entry(name.to_owned()).or_default();
    *entry = entry
        .checked_add(value)
        .ok_or_else(|| format!("fixed-batch counter `{name}` overflowed"))?;
    Ok(())
}

fn merge_phase(
    phases: &mut BTreeMap<String, PhaseRecord>,
    source: &PhaseRecord,
) -> Result<(), String> {
    let destination = phases
        .entry(source.name.clone())
        .or_insert_with(|| PhaseRecord {
            name: source.name.clone(),
            wall_ns: Some(0),
            ..Default::default()
        });
    destination.adds = destination
        .adds
        .checked_add(source.adds)
        .ok_or_else(|| "fixed-batch phase additions overflowed".to_owned())?;
    destination.doubles = destination
        .doubles
        .checked_add(source.doubles)
        .ok_or_else(|| "fixed-batch phase doublings overflowed".to_owned())?;
    destination.scalar_mults = destination
        .scalar_mults
        .checked_add(source.scalar_mults)
        .ok_or_else(|| "fixed-batch phase scalar multiplications overflowed".to_owned())?;
    destination.gae += source.gae;
    if !destination.gae.is_finite() {
        return Err("fixed-batch phase GAE is non-finite".into());
    }
    destination.wall_ns = match (destination.wall_ns, source.wall_ns) {
        (Some(left), Some(right)) => Some(
            left.checked_add(right)
                .ok_or_else(|| "fixed-batch phase wall time overflowed".to_owned())?,
        ),
        _ => None,
    };
    for (name, value) in &source.native {
        checked_counter_add(&mut destination.native, name, *value)?;
    }
    for (name, value) in &source.work_native {
        checked_counter_add(&mut destination.work_native, name, *value)?;
    }
    Ok(())
}

fn solve_rho_batch64(
    inst: &PrimeInstance,
    items: &[([String; 2], Option<u64>)],
) -> Result<(SolveReport, Vec<BatchItemReport>), String> {
    // Match the BSGS batch boundary: canonical parsing, curve/generator/point
    // conversion, and deterministic per-item config construction are fixture
    // preparation and finish before the enclosing batch timer starts.
    let (curve, generator) = prepare_rho_curve(inst)?;
    let mut prepared = Vec::with_capacity(items.len());
    for (target, seed) in items {
        let (tx, ty) = parse_public_target(inst, target)?;
        let point = curve.point(tx, ty);
        if !curve.is_on_curve(point) {
            return Err("fixed-batch rho target is not on the frozen curve".into());
        }
        let Some(seed) = *seed else {
            return Err("fixed-batch rho requires one independent seed per item".into());
        };
        prepared.push((point, prepare_rho_config(inst, seed)));
    }

    let total_start = Instant::now();
    let mut item_reports = Vec::with_capacity(items.len());
    let mut reports = Vec::with_capacity(items.len());
    measurement::begin_online(Phase::RhoSolve);
    let batch_result = (|| -> Result<(), String> {
        for (index, ((target, seed), (point, config))) in
            items.iter().zip(prepared.iter()).enumerate()
        {
            let seed = seed.ok_or("fixed-batch rho seed disappeared after validation")?;
            let report = solve_rho_prepared(
                inst,
                &curve,
                generator,
                *point,
                config,
                RHO_BATCH64_ID,
                false,
            )?;
            let evaluator_correct = report.detail["evaluator_correct"]
                .as_bool()
                .unwrap_or(false);
            let phase_wall = |name: &str| {
                report
                    .phases
                    .iter()
                    .find(|phase| phase.name == name)
                    .and_then(|phase| phase.wall_ns)
                    .unwrap_or(0)
            };
            item_reports.push(BatchItemReport {
                item: index as u64,
                target: target.clone(),
                rho_seed: Some(format!("{seed:016x}")),
                recovered: report.recovered,
                exhausted: report.exhausted || !evaluator_correct,
                evaluator_correct,
                solver_wall_ns: phase_wall("rho_solver"),
                verification_wall_ns: phase_wall("evaluator_verification"),
                drop_wall_ns: phase_wall("teardown"),
                counters: report.counters.clone(),
            });
            reports.push(report);
        }
        Ok(())
    })();
    measurement::end_online();
    batch_result?;
    let total_ns = elapsed_ns(total_start);
    let child_total_ns = reports.iter().try_fold(0u64, |total, report| {
        total
            .checked_add(report.solve_wall_ns)
            .ok_or_else(|| "fixed-batch rho child-wall sum overflowed".to_owned())
    })?;
    let dispatch_residual_ns = total_ns
        .checked_sub(child_total_ns)
        .ok_or_else(|| "fixed-batch rho child clocks exceeded batch clock".to_owned())?;

    let mut phase_map = BTreeMap::new();
    for report in &reports {
        for phase in &report.phases {
            merge_phase(&mut phase_map, phase)?;
        }
    }
    phase_map.insert(
        "batch_dispatch_residual".into(),
        timed_phase("batch_dispatch_residual", dispatch_residual_ns),
    );
    let phases: Vec<_> = phase_map.into_values().collect();
    let total_gae = reports.iter().try_fold(0.0, |total, report| {
        let next = total + report.total_gae;
        if next.is_finite() {
            Ok(next)
        } else {
            Err("fixed-batch rho GAE is non-finite".to_owned())
        }
    })?;
    let mut counters = BTreeMap::new();
    counters.insert("batch_size".into(), 64);
    counters.insert("table_builds".into(), 64);
    counters.insert(
        "successful_items".into(),
        item_reports.iter().filter(|item| !item.exhausted).count() as u64,
    );
    counters.insert(
        "exhausted_items".into(),
        item_reports.iter().filter(|item| item.exhausted).count() as u64,
    );
    for report in &reports {
        for (name, value) in &report.counters {
            // The independent seeds are identities sealed by the ordered study
            // metadata, not additive work. Summing arbitrary u64 seeds is both
            // meaningless and can overflow on an otherwise valid batch.
            if name == "config_seed" {
                continue;
            }
            checked_counter_add(&mut counters, &format!("item_sum_{name}"), *value)?;
        }
    }
    let mut unpriced = Vec::new();
    for report in &reports {
        for name in &report.unpriced {
            let aggregate = format!("item_sum_{name}");
            if !unpriced.contains(&aggregate) {
                unpriced.push(aggregate);
            }
        }
    }
    let exhausted = item_reports.iter().any(|item| item.exhausted);
    let detail_items = item_reports.clone();
    Ok((
        SolveReport {
            recovered: None,
            exhausted,
            phases,
            total_gae,
            automorphisms_used: 2,
            counters,
            unpriced,
            deterministic: true,
            nondeterminism: vec![],
            solve_wall_ns: total_ns,
            factor_base: None,
            detail: json!({
                "implementation": "64 sequential cryptanalysis::prime_fast::rho_parallel calls",
                "batch_size": 64,
                "table_builds": 64,
                "batch_items": detail_items,
                "excluded_preparation": "canonical parsing plus curve, generator, target-point, and deterministic RhoConfig construction completed before the batch timer, matching the BSGS fixture-preparation boundary",
                "cold_boundary": "all 64 sequential target-dependent rho tables, walks, collision recovery, evaluator replays, and returned-state deallocation",
            }),
            online: None,
            online_error: None,
            solver: None,
            field_ops: None,
        },
        item_reports,
    ))
}

pub(super) fn solve_batch64(
    id: &str,
    inst: &PrimeInstance,
    facts: &CurveFacts,
    items: &[([String; 2], Option<u64>)],
) -> Result<(SolveReport, Vec<BatchItemReport>), String> {
    applicable(id, inst, facts)?;
    if items.len() != 64 {
        return Err(format!(
            "`{id}` requires exactly 64 ordered public targets, got {}",
            items.len()
        ));
    }
    match id {
        BSGS_BATCH64_ID => solve_bsgs_batch64(inst, items),
        RHO_BATCH64_ID => solve_rho_batch64(inst, items),
        _ => Err(format!("`{id}` is not a fixed 64-target batch method")),
    }
}

pub(super) fn solve(
    id: &str,
    inst: &PrimeInstance,
    facts: &CurveFacts,
    tx: u64,
    ty: u64,
    seed: u64,
) -> Result<SolveReport, String> {
    applicable(id, inst, facts)?;
    match id {
        BSGS_ID => solve_bsgs(inst, tx, ty),
        RHO_ID => solve_rho(inst, tx, ty, seed),
        BSGS_BATCH64_ID | RHO_BATCH64_ID => Err(format!(
            "`{id}` requires solve_batch64_measured and refuses the single-target API"
        )),
        _ => Err(format!("`{id}` is not a frozen prime control")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ecbench::methods::{self, MethodSpec};
    use crate::cryptanalysis::ecbench::workload::{CurveSpec, TargetKind, Workload};

    fn fp24() -> CurveSpec {
        CurveSpec::PrimeExplicit {
            p: 16_777_199,
            a: 16_777_196,
            b: 31,
            group_order: 16_782_299,
            r: 16_782_299,
            gx: 2,
            gy: 5_412_647,
        }
    }

    fn integer_phases(report: &SolveReport) -> Vec<(&str, u64, u64, u64, u64)> {
        report
            .phases
            .iter()
            .map(|p| {
                (
                    p.name.as_str(),
                    p.adds,
                    p.doubles,
                    p.scalar_mults,
                    p.gae.to_bits(),
                )
            })
            .collect()
    }

    fn batch64_fixture() -> (
        CurveSpec,
        crate::cryptanalysis::ecbench::workload::Instance,
        Vec<([String; 2], Option<u64>)>,
        Vec<ExactScalar>,
    ) {
        let spec = fp24();
        let inst = spec.build().unwrap();
        let mut items = Vec::with_capacity(64);
        let mut planted = Vec::with_capacity(64);
        for item in 0..64 {
            let workload = Workload::on(&spec, &inst, 20261126, item, TargetKind::Planted).unwrap();
            items.push((
                workload.target.clone(),
                Some(0x51eed00000000000u64.wrapping_add(item)),
            ));
            planted.push(workload.planted.unwrap());
        }
        (spec, inst, items, planted)
    }

    #[test]
    fn fixed_controls_solve_and_replay_the_fp24_fixture() {
        let spec = fp24();
        let inst = spec.build().unwrap();
        let facts = inst.facts(&spec);
        let workload = Workload::on(&spec, &inst, 20261126, 7, TargetKind::Planted).unwrap();
        for id in [BSGS_ID, RHO_ID] {
            let method = methods::resolve(&MethodSpec {
                id: id.into(),
                params: BTreeMap::new(),
            })
            .unwrap();
            methods::applicable(&method, &inst, &facts).unwrap();
            let first = methods::solve(&method, &inst, &facts, &workload.target, 0x51eed).unwrap();
            let replay = methods::solve(&method, &inst, &facts, &workload.target, 0x51eed).unwrap();
            assert_eq!(first.recovered, workload.planted, "{id}");
            assert_eq!(first.recovered, replay.recovered, "{id}");
            assert_eq!(first.counters, replay.counters, "{id}");
            assert_eq!(integer_phases(&first), integer_phases(&replay), "{id}");
            assert_eq!(
                first.total_gae.to_bits(),
                replay.total_gae.to_bits(),
                "{id}"
            );
            assert!(!first.unpriced.is_empty(), "{id}");
            assert!(first.online.is_some(), "{id}");
        }
    }

    #[test]
    fn fixed_controls_refuse_nonpanel_prime_curves() {
        let spec = CurveSpec::PrimeExplicit {
            p: 223,
            a: 74,
            b: 122,
            group_order: 205,
            r: 41,
            gx: 119,
            gy: 198,
        };
        let inst = spec.build().unwrap();
        let facts = inst.facts(&spec);
        for id in [BSGS_ID, RHO_ID] {
            let method = methods::resolve(&MethodSpec {
                id: id.into(),
                params: BTreeMap::new(),
            })
            .unwrap();
            assert!(methods::applicable(&method, &inst, &facts)
                .unwrap_err()
                .contains("five fixed"));
        }
    }

    #[test]
    fn fixed_batch64_methods_recover_every_ordered_fp24_item() {
        let (spec, inst, items, planted) = batch64_fixture();
        let facts = inst.facts(&spec);
        for id in [BSGS_BATCH64_ID, RHO_BATCH64_ID] {
            let method = methods::resolve(&MethodSpec {
                id: id.into(),
                params: BTreeMap::new(),
            })
            .unwrap();
            let (report, rows) =
                methods::solve_batch64_measured(&method, &inst, &facts, &items).unwrap();
            assert!(report.recovered.is_none(), "{id}");
            assert!(!report.exhausted, "{id}");
            assert_eq!(rows.len(), 64, "{id}");
            assert_eq!(report.counters["batch_size"], 64, "{id}");
            assert!(
                report.online_error.is_none(),
                "{id}: {:?}",
                report.online_error
            );
            assert!(report.online.is_some(), "{id}");
            assert_eq!(
                report.counters["table_builds"],
                if id == BSGS_BATCH64_ID { 1 } else { 64 },
                "{id}"
            );
            if id == RHO_BATCH64_ID {
                assert_eq!(
                    report.detail["excluded_preparation"],
                    "canonical parsing plus curve, generator, target-point, and deterministic RhoConfig construction completed before the batch timer, matching the BSGS fixture-preparation boundary"
                );
                assert_eq!(
                    report.detail["cold_boundary"],
                    "all 64 sequential target-dependent rho tables, walks, collision recovery, evaluator replays, and returned-state deallocation"
                );
            }
            for (index, row) in rows.iter().enumerate() {
                assert_eq!(row.item, index as u64, "{id}");
                assert_eq!(row.recovered, Some(planted[index]), "{id}");
                assert!(row.evaluator_correct, "{id}");
                assert!(!row.exhausted, "{id}");
            }
        }
    }

    #[test]
    fn fixed_batch64_api_rejects_cardinality_missing_seed_and_single_dispatch() {
        let (spec, inst, mut items, _) = batch64_fixture();
        let facts = inst.facts(&spec);
        let bsgs = methods::resolve(&MethodSpec {
            id: BSGS_BATCH64_ID.into(),
            params: BTreeMap::new(),
        })
        .unwrap();
        let rho = methods::resolve(&MethodSpec {
            id: RHO_BATCH64_ID.into(),
            params: BTreeMap::new(),
        })
        .unwrap();

        assert!(methods::solve_batch64_measured(&bsgs, &inst, &facts, &items[..63]).is_err());
        items[17].1 = None;
        assert!(methods::solve_batch64_measured(&rho, &inst, &facts, &items).is_err());
        let target = &items[0].0;
        for method in [&bsgs, &rho] {
            let error = methods::solve(method, &inst, &facts, target, 7).unwrap_err();
            assert!(error.contains("refuses the single-target API"));
        }
    }
}
