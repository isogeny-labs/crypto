//! Reproduce the matched PKM Gröbner-degree speedup in native Rust.
//!
//! This runs the repository's complete prime-field relation pipeline twice on
//! the same planted 13-bit discrete logarithm:
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
//! at least 20x smaller.  This is a small, selected-instance component
//! demonstration, not an asymptotic claim or a Pollard-rho crossover.
//!
//! ```text
//! cargo run --release --example pkm_groebner_drastic
//! ```

use crypto_lib::cryptanalysis::ic_boundary::{
    prime_instance_for, Calibration, CountedGroup, GroupOps,
};
use crypto_lib::cryptanalysis::ic_framework::pkm_semaev::PkmSemaevOracle;
use crypto_lib::cryptanalysis::ic_framework::plugins::PrimeAbscissaBase;
use crypto_lib::cryptanalysis::ic_framework::prime_tower::PrimeIsogenyBase;
use crypto_lib::cryptanalysis::ic_framework::stages::{InstanceCtx, Params, Targets};
use crypto_lib::cryptanalysis::ic_framework::{run_pipeline, PipelineSpec, RunReport};
use serde::Serialize;

const INSTANCE_SEED: u64 = 20_261_185;
const PLANTED_LOG: u64 = 2_261;
const EXPECTED_ABSCISSAE: usize = 18;
const MIN_F4_SPEEDUP: u64 = 20;

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
    recovered: u64,
    verified: bool,
}

fn spec(factor_base: &str, oracle: &str) -> PipelineSpec {
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
        seed: INSTANCE_SEED,
        solver_budget_seconds: 5,
        ..Default::default()
    }
}

fn summarize(label: &'static str, report: RunReport) -> ArmSummary {
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
        recovered: report
            .recovered
            .unwrap_or_else(|| panic!("{label}: the relation pipeline did not recover a scalar")),
        verified: report.verified,
    }
}

fn main() {
    let instance = prime_instance_for(13, INSTANCE_SEED);
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
    let calibration = Calibration::default();

    let interval_spec = spec(
        "prime-abscissa:below=32",
        "pkm-semaev:m=2,engine=f4,degree=8,bits=5,lo=0,mode=chain,trace=0",
    );
    let mut interval_oracle = PkmSemaevOracle::from_spec(2, &interval_spec, &instance)
        .expect("build the interval PKM oracle");
    let interval_builder = PrimeAbscissaBase {
        instance: &instance,
    };
    let interval = summarize(
        "interval",
        run_pipeline(
            &ctx,
            &interval_spec,
            &interval_builder,
            &mut interval_oracle,
            PLANTED_LOG,
            &calibration,
            None,
        )
        .expect("run the matched interval pipeline"),
    );

    let velu_spec = spec(
        "prime-isogeny:t=5,seed=5,aux=ordinary",
        "pkm-semaev:m=2,engine=f4,degree=8,mode=chain,trace=0",
    );
    let mut velu_oracle = PkmSemaevOracle::from_spec(2, &velu_spec, &instance)
        .expect("build the degree-32 Vélu PKM oracle");
    let velu_builder = PrimeIsogenyBase {
        instance: &instance,
    };
    let velu = summarize(
        "degree32_velu",
        run_pipeline(
            &ctx,
            &velu_spec,
            &velu_builder,
            &mut velu_oracle,
            PLANTED_LOG,
            &calibration,
            None,
        )
        .expect("run the degree-32 Vélu pipeline"),
    );

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
    assert!(
        velu.f4_multiply_adds.saturating_mul(MIN_F4_SPEEDUP) <= interval.f4_multiply_adds,
        "degree-32 Vélu F4 speedup is below the {MIN_F4_SPEEDUP}x demo gate: {} / {}",
        interval.f4_multiply_adds,
        velu.f4_multiply_adds,
    );

    let result = serde_json::json!({
        "schema": "pkm-groebner-drastic/v1",
        "claim_scope": "selected 13-bit matched relation-extraction component; not rho or asymptotic",
        "interval": interval,
        "degree32_velu": velu,
        "f4_speedup_x": interval.f4_multiply_adds as f64 / velu.f4_multiply_adds as f64,
        "solving_degree": {"interval": 6, "degree32_velu": 5},
    });
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
