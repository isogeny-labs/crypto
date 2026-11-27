//! Reproduce the matched PKM Gröbner-degree speedup in native Rust.
//!
//! This runs the repository's complete prime-field relation pipeline in a
//! balanced interval / candidate / candidate / interval order on the same
//! planted 13-bit discrete logarithm:
//!
//! - an 18-abscissa interval, encoded by five Boolean membership variables;
//! - an 18-abscissa PKM auxiliary-curve fibre, encoded by five degree-2 Vélu
//!   maps (composed degree 32).
//!
//! The two systems have the same number of variables and equations and the
//! same equation-degree vector.  The selected Vélu coefficients lower the
//! observed ordinary F4 solving degree from 6 to 5.  The executable refuses
//! to print a success unless both pipelines recover the same planted scalar,
//! both bases have 18 abscissae, the degree falls, and the counted F4 work is
//! at least 20x smaller and the complete candidate charge is at most half the
//! matched interval charge. This is a small, selected-instance demonstration,
//! not an asymptotic claim or a Pollard-rho crossover.
//!
//! ```text
//! cargo run --release --example pkm_groebner_drastic
//! ```

use crypto_lib::cryptanalysis::ic_boundary::{
    calibrate_group, calibrate_row_ops, prime_instance_for, Calibration, CountedGroup, GroupOps,
};
use crypto_lib::cryptanalysis::ic_framework::pkm_semaev::PkmSemaevOracle;
use crypto_lib::cryptanalysis::ic_framework::plugins::PrimeAbscissaBase;
use crypto_lib::cryptanalysis::ic_framework::prime_tower::PrimeIsogenyBase;
use crypto_lib::cryptanalysis::ic_framework::stages::{InstanceCtx, Params, Targets};
use crypto_lib::cryptanalysis::ic_framework::{run_pipeline, PipelineSpec, RunReport};
use serde::Serialize;
use std::time::Instant;

const DEFAULT_INSTANCE_SEED: u64 = 20_261_185;
const PLANTED_LOG: u64 = 2_261;
const EXPECTED_ABSCISSAE: usize = 18;
const MIN_F4_SPEEDUP: u64 = 20;
const DEFAULT_MAX_E2E_GAE_RATIO: f64 = 0.5;
const DEFAULT_E2E_PAIRS: usize = 1;

#[derive(Serialize)]
struct PhaseSummary {
    factor_base_gae: f64,
    oracle_setup_gae: f64,
    relations_and_solver_gae: f64,
    linear_algebra_gae: f64,
    verification_gae: f64,
}

#[derive(Serialize)]
struct ArmSummary {
    label: &'static str,
    instance: String,
    base_abscissae: usize,
    system_variables: usize,
    system_equations: usize,
    system_degrees: Vec<u32>,
    solver_calls: u64,
    solving_degree_max: u32,
    f4_multiply_adds: u64,
    solver_gae: f64,
    solver_priced_by: String,
    total_gae: f64,
    s: f64,
    pipeline_wall_seconds: f64,
    cold_wall_seconds: f64,
    phases: PhaseSummary,
    cost_qualification: Option<String>,
    recovered: u64,
    verified: bool,
}

fn spec(factor_base: &str, oracle: &str, instance_seed: u64) -> PipelineSpec {
    let (factor_base, factor_base_params) = Params::parse_spec(factor_base)
        .unwrap_or_else(|e| panic!("invalid factor-base fixture: {e}"));
    let (oracle, oracle_params) =
        Params::parse_spec(oracle).unwrap_or_else(|e| panic!("invalid oracle fixture: {e}"));
    PipelineSpec {
        factor_base,
        factor_base_params,
        oracle,
        oracle_params,
        targets: Targets::Walk,
        linalg: "incremental-gauss".into(),
        max_trials: 1_000,
        seed: instance_seed,
        solver_budget_seconds: 5,
        ..Default::default()
    }
}

fn summarize(label: &'static str, report: RunReport, cold_wall_seconds: f64) -> ArmSummary {
    let shape = report
        .decomposition
        .system
        .as_ref()
        .unwrap_or_else(|| panic!("{label}: PKM pipeline did not report its system shape"));
    let solver = report
        .decomposition
        .solver
        .as_ref()
        .unwrap_or_else(|| panic!("{label}: PKM pipeline did not report F4 work"));
    ArmSummary {
        label,
        instance: report.instance,
        base_abscissae: report.factor_base.abscissae,
        system_variables: shape.n_vars,
        system_equations: shape.n_equations,
        system_degrees: shape.degrees.clone(),
        solver_calls: solver.calls,
        solving_degree_max: solver.solving_degree_max,
        f4_multiply_adds: solver.ops,
        solver_gae: solver.gae,
        solver_priced_by: solver.priced_by.clone(),
        total_gae: report.total_gae,
        s: report.s,
        pipeline_wall_seconds: report.wall_seconds,
        cold_wall_seconds,
        phases: PhaseSummary {
            factor_base_gae: report.factor_base.cost.gae,
            oracle_setup_gae: report.decomposition.setup.gae,
            relations_and_solver_gae: report.decomposition.cost.gae,
            linear_algebra_gae: report.linear_algebra.cost.gae,
            verification_gae: report.verify.gae,
        },
        cost_qualification: report.cost_qualification,
        recovered: report
            .recovered
            .unwrap_or_else(|| panic!("{label}: the relation pipeline did not recover a scalar")),
        verified: report.verified,
    }
}

fn run_interval(
    ctx: &InstanceCtx<'_, crypto_lib::cryptanalysis::ic_boundary::PrimeCurve>,
    instance: &crypto_lib::cryptanalysis::ic_boundary::PrimeInstance,
    calibration: &Calibration,
    instance_seed: u64,
) -> ArmSummary {
    let started = Instant::now();
    let interval_spec = spec(
        "prime-abscissa:below=32",
        "pkm-semaev:m=2,engine=f4,degree=8,bits=5,lo=0,mode=chain,trace=0",
        instance_seed,
    );
    let mut interval_oracle = PkmSemaevOracle::from_spec(2, &interval_spec, instance)
        .expect("build the interval PKM oracle");
    let interval_builder = PrimeAbscissaBase { instance };
    let report = run_pipeline(
        ctx,
        &interval_spec,
        &interval_builder,
        &mut interval_oracle,
        PLANTED_LOG,
        calibration,
        None,
    )
    .expect("run the matched interval pipeline");
    summarize("interval", report, started.elapsed().as_secs_f64())
}

fn run_velu(
    ctx: &InstanceCtx<'_, crypto_lib::cryptanalysis::ic_boundary::PrimeCurve>,
    instance: &crypto_lib::cryptanalysis::ic_boundary::PrimeInstance,
    calibration: &Calibration,
    instance_seed: u64,
) -> ArmSummary {
    let started = Instant::now();
    let velu_spec = spec(
        "prime-isogeny:t=5,seed=5,aux=ordinary",
        "pkm-semaev:m=2,engine=f4,degree=8,mode=chain,trace=0",
        instance_seed,
    );
    let mut velu_oracle = PkmSemaevOracle::from_spec(2, &velu_spec, instance)
        .expect("build the degree-32 Velu PKM oracle");
    let velu_builder = PrimeIsogenyBase { instance };
    let report = run_pipeline(
        ctx,
        &velu_spec,
        &velu_builder,
        &mut velu_oracle,
        PLANTED_LOG,
        calibration,
        None,
    )
    .expect("run the degree-32 Velu pipeline");
    summarize("degree32_velu", report, started.elapsed().as_secs_f64())
}

fn assert_matched(interval: &ArmSummary, velu: &ArmSummary) {
    assert!(
        interval.verified && velu.verified,
        "both scalar checks must pass"
    );
    assert_eq!(interval.recovered, PLANTED_LOG);
    assert_eq!(velu.recovered, PLANTED_LOG);
    assert_eq!(interval.base_abscissae, EXPECTED_ABSCISSAE);
    assert_eq!(velu.base_abscissae, EXPECTED_ABSCISSAE);
    assert_eq!(interval.system_variables, velu.system_variables);
    assert_eq!(interval.system_equations, velu.system_equations);
    assert_eq!(interval.system_degrees, velu.system_degrees);
    assert_eq!(interval.solving_degree_max, 6);
    assert_eq!(velu.solving_degree_max, 5);
    assert!(interval.cost_qualification.is_none());
    assert!(velu.cost_qualification.is_none());
    assert_eq!(interval.solver_priced_by, "measured");
    assert_eq!(velu.solver_priced_by, "measured");
    for run in [interval, velu] {
        let phase_total = run.phases.factor_base_gae
            + run.phases.oracle_setup_gae
            + run.phases.relations_and_solver_gae
            + run.phases.linear_algebra_gae
            + run.phases.verification_gae;
        assert!(
            (phase_total - run.total_gae).abs() <= f64::EPSILON * run.total_gae.abs() * 8.0,
            "{}: phase charges do not sum to total_gae: {phase_total} != {}",
            run.label,
            run.total_gae,
        );
    }
    assert!(
        velu.f4_multiply_adds.saturating_mul(MIN_F4_SPEEDUP) <= interval.f4_multiply_adds,
        "degree-32 Velu F4 speedup is below the {MIN_F4_SPEEDUP}x demo gate: {} / {}",
        interval.f4_multiply_adds,
        velu.f4_multiply_adds,
    );
}

fn main() {
    let instance_seed = std::env::var("PKM_INSTANCE_SEED")
        .ok()
        .map(|value| {
            value
                .parse::<u64>()
                .expect("PKM_INSTANCE_SEED must be an integer")
        })
        .unwrap_or(DEFAULT_INSTANCE_SEED);
    let max_e2e_gae_ratio = std::env::var("PKM_MAX_E2E_GAE_RATIO")
        .ok()
        .map(|value| {
            value
                .parse::<f64>()
                .expect("PKM_MAX_E2E_GAE_RATIO must be a number")
        })
        .unwrap_or(DEFAULT_MAX_E2E_GAE_RATIO);
    assert!(
        max_e2e_gae_ratio > 0.0 && max_e2e_gae_ratio <= 1.0,
        "PKM_MAX_E2E_GAE_RATIO must be in (0, 1]"
    );
    let instance = prime_instance_for(13, instance_seed);
    let curve = &instance.curve;
    let mut target_ops = GroupOps::default();
    let target = curve.mul(
        &mut target_ops,
        instance.generator_point(),
        PLANTED_LOG % instance.r,
    );
    let ctx = InstanceCtx {
        group: curve,
        generator: instance.generator_point(),
        target,
        r: instance.r,
        cofactor: instance.cofactor,
        group_order: instance.group_order,
        name: instance.name.clone(),
        field_degree: None,
    };
    let mut sample_points = Vec::new();
    for k in 1..=8 {
        let mut ops = GroupOps::default();
        sample_points.push(curve.mul(&mut ops, instance.generator_point(), k));
    }
    let mut calibration = Calibration::default();
    calibrate_group(curve, &sample_points, &mut calibration);
    calibrate_row_ops(instance.r, &mut calibration);
    let pin = calibration.pin("prime", &instance.name);

    let pairs = std::env::var("PKM_E2E_PAIRS")
        .ok()
        .map(|value| {
            value
                .parse::<usize>()
                .expect("PKM_E2E_PAIRS must be an integer")
        })
        .unwrap_or(DEFAULT_E2E_PAIRS);
    assert!((1..=8).contains(&pairs), "PKM_E2E_PAIRS must be in 1..=8");

    let mut intervals = Vec::with_capacity(2 * pairs);
    let mut velus = Vec::with_capacity(2 * pairs);
    for _ in 0..pairs {
        intervals.push(run_interval(&ctx, &instance, &calibration, instance_seed));
        velus.push(run_velu(&ctx, &instance, &calibration, instance_seed));
        velus.push(run_velu(&ctx, &instance, &calibration, instance_seed));
        intervals.push(run_interval(&ctx, &instance, &calibration, instance_seed));
    }
    for (interval, velu) in intervals.iter().zip(&velus) {
        assert_matched(interval, velu);
    }

    let interval_total_gae: f64 = intervals.iter().map(|run| run.total_gae).sum();
    let velu_total_gae: f64 = velus.iter().map(|run| run.total_gae).sum();
    let interval_cold_wall_seconds: f64 = intervals.iter().map(|run| run.cold_wall_seconds).sum();
    let velu_cold_wall_seconds: f64 = velus.iter().map(|run| run.cold_wall_seconds).sum();
    assert!(
        velu_total_gae <= max_e2e_gae_ratio * interval_total_gae,
        "degree-32 Velu complete charged cost missed the {max_e2e_gae_ratio} ratio gate: {velu_total_gae} / {interval_total_gae}",
    );

    let result = serde_json::json!({
        "schema": "pkm-groebner-drastic/v2",
        "claim_scope": "selected 13-bit matched complete native pipeline; host-measured F4 pricing; not random-instance, rho, or asymptotic",
        "balanced_order": "interval, degree32_velu, degree32_velu, interval",
        "instance_seed": instance_seed,
        "pairs": pairs,
        "maximum_e2e_gae_candidate_over_interval": max_e2e_gae_ratio,
        "calibration": {
            "ns_per_add": calibration.ns_per_add,
            "ns_per_row_op": calibration.ns_per_row_op,
            "pinned_units": pin.pinned,
            "measured_units": pin.measured,
        },
        "interval": intervals,
        "degree32_velu": velus,
        "aggregate": {
            "interval_total_gae": interval_total_gae,
            "degree32_velu_total_gae": velu_total_gae,
            "e2e_gae_candidate_over_interval": velu_total_gae / interval_total_gae,
            "e2e_gae_speedup_x": interval_total_gae / velu_total_gae,
            "interval_cold_wall_seconds": interval_cold_wall_seconds,
            "degree32_velu_cold_wall_seconds": velu_cold_wall_seconds,
            "cold_wall_candidate_over_interval": velu_cold_wall_seconds / interval_cold_wall_seconds,
            "cold_wall_speedup_x": interval_cold_wall_seconds / velu_cold_wall_seconds,
        },
        "f4_speedup_x": intervals[0].f4_multiply_adds as f64 / velus[0].f4_multiply_adds as f64,
        "solving_degree": {"interval": 6, "degree32_velu": 5},
    });
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
