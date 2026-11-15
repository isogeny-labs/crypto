//! `ecbench` adapter for the prime-field orbit relation collector.
//!
//! This is deliberately a component benchmark inside a complete verified
//! solve. The `relations` phase clocks collector construction and probe loops
//! only; `solve_wall_ns` remains the end-to-end one-target run.

use std::collections::BTreeMap;
use std::time::Instant;

use num_bigint::BigUint;
use serde_json::json;

use super::methods::{unpriced_of, ParamDecl, PhaseRecord, ResolvedMethod, SolveReport};
use crate::cryptanalysis::ecbench::workload::CurveFacts;
use crate::cryptanalysis::ic_boundary::{PrimeInstance, PrimePoint};
use crate::cryptanalysis::prime_orbit_index_calculus::{
    run_known_answer, OrbitCurve, OrbitIcOptions, OrbitIcReport, RelationCoordinates,
};
use crate::ecc::curve::CurveParams;

pub const ID: &str = "ic.prime_orbit_relations_v1";

pub const PARAMS: &[ParamDecl] = &[
    ParamDecl {
        name: "coordinates",
        default: None,
        help: "affine (one lane) or jacobian (32 lanes with batch normalization)",
    },
    ParamDecl {
        name: "orbits",
        default: Some("32"),
        help: "explicit number of factor-base automorphism orbits",
    },
    ParamDecl {
        name: "relations_per_orbit",
        default: Some("1.5"),
        help: "relations requested per factor-base orbit in each collection round",
    },
    ParamDecl {
        name: "max_ops",
        default: Some("4294967296"),
        help: "oracle-operation cap for relation collection",
    },
    ParamDecl {
        name: "max_descent_ops",
        default: Some("4294967296"),
        help: "oracle-operation cap for the verified target descent",
    },
];

fn u64_param(m: &ResolvedMethod, name: &str) -> Result<u64, String> {
    m.params[name].parse().map_err(|_| {
        format!(
            "parameter `{name}` is not a nonnegative integer: `{}`",
            m.params[name]
        )
    })
}

fn positive_f64_param(m: &ResolvedMethod, name: &str) -> Result<f64, String> {
    let value: f64 = m.params[name]
        .parse()
        .map_err(|_| format!("parameter `{name}` is not a number: `{}`", m.params[name]))?;
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(format!("parameter `{name}` must be positive: `{value}`"))
    }
}

fn coordinates(m: &ResolvedMethod) -> Result<RelationCoordinates, String> {
    match m.params["coordinates"].as_str() {
        "affine" => Ok(RelationCoordinates::Affine),
        "jacobian" => Ok(RelationCoordinates::Jacobian),
        value => Err(format!(
            "parameter `coordinates` is `affine` or `jacobian`, not `{value}`"
        )),
    }
}

fn orbit_curve(inst: &PrimeInstance) -> Result<OrbitCurve, String> {
    let h = u32::try_from(inst.cofactor)
        .map_err(|_| format!("cofactor {} does not fit the orbit engine", inst.cofactor))?;
    OrbitCurve::from_params(&CurveParams {
        name: "ecbench-prime-orbit",
        p: BigUint::from(inst.curve.p),
        a: BigUint::from(inst.curve.a),
        b: BigUint::from(inst.curve.b),
        gx: BigUint::from(inst.generator.0),
        gy: BigUint::from(inst.generator.1),
        n: BigUint::from(inst.r),
        h,
    })
}

fn phase(name: &str, gae: u64, wall_ns: Option<u64>) -> PhaseRecord {
    PhaseRecord {
        name: name.into(),
        gae: gae as f64,
        wall_ns,
        ..PhaseRecord::default()
    }
}

/// Run the selected relation-coordinate backend inside a complete target
/// descent. Every returned logarithm has already passed `[d]G == Q`; the
/// outer runner independently verifies it again.
pub fn solve(
    m: &ResolvedMethod,
    inst: &PrimeInstance,
    facts: &CurveFacts,
    target: PrimePoint,
    seed: u64,
) -> Result<SolveReport, String> {
    let relation_coordinates = coordinates(m)?;
    let orbits = usize::try_from(u64_param(m, "orbits")?)
        .map_err(|_| "parameter `orbits` does not fit this host".to_string())?;
    if orbits == 0 {
        return Err("parameter `orbits` must be positive".into());
    }
    let opts = OrbitIcOptions {
        orbits,
        relations_per_orbit: positive_f64_param(m, "relations_per_orbit")?,
        relation_coordinates,
        max_ops: u64_param(m, "max_ops")?,
        max_descent_ops: u64_param(m, "max_descent_ops")?,
        learn: false,
        skip_rho: true,
        seed,
        ..OrbitIcOptions::default()
    };
    let started = Instant::now();
    let curve = orbit_curve(inst)?;
    let target = curve.point_from_affine(target.x, target.y)?;
    let report = run_known_answer(&curve, &[target], &opts);
    let solve_wall_ns = started.elapsed().as_nanos() as u64;
    let descent = report
        .descents
        .first()
        .ok_or("prime-orbit run did not emit its target descent")?;

    let relation_ops = report.precompute_ops();
    let descent_ops = OrbitIcReport::descent_ops(descent);
    let other_logs_ns = (report.logs.seconds * 1e9) as u64;
    let other_logs_ns = other_logs_ns.saturating_sub(report.logs.collection_ns);
    let phases = vec![
        phase("relations", relation_ops, Some(report.logs.collection_ns)),
        phase("relation_postprocess", 0, Some(other_logs_ns)),
        phase(
            "target_descent",
            descent_ops,
            Some((descent.seconds * 1e9) as u64),
        ),
    ];
    let mut counters = BTreeMap::from([
        ("relation_lanes".into(), relation_coordinates.lanes() as u64),
        ("factor_base_orbits".into(), report.logs.orbits as u64),
        ("factor_base_points".into(), report.logs.points as u64),
        (
            "factor_base_draws_uncharged".into(),
            report.logs.factor_base_draws,
        ),
        ("relation_trials".into(), report.logs.trials),
        ("relations".into(), report.logs.relations as u64),
        ("full_relations".into(), report.logs.full_relations),
        ("combined_relations".into(), report.logs.combined_relations),
        (
            "distinct_large_primes".into(),
            report.logs.distinct_large_primes,
        ),
        ("known_large_primes".into(), report.logs.known_large_primes),
        (
            "collection_rounds".into(),
            report.logs.collection_rounds as u64,
        ),
        ("walk_restarts".into(), report.logs.walk_restarts),
        ("oracle_ops".into(), report.logs.oracle_ops),
        ("probe_ops".into(), report.logs.probe_ops),
        ("rejected_relations".into(), report.logs.rejected_relations),
        (
            "certified_columns".into(),
            report.logs.certified_columns as u64,
        ),
        (
            "uncertified_columns".into(),
            report.logs.uncertified_columns as u64,
        ),
        (
            "inconsistent_components".into(),
            report.logs.system.inconsistent_components as u64,
        ),
        ("descent_trials".into(), descent.trials),
        ("descent_oracle_ops".into(), descent.oracle_ops),
        ("descent_probe_ops".into(), descent.probe_ops),
        ("descent_restarts".into(), descent.restarts),
    ]);
    if descent.verified {
        counters.insert("verified_descents".into(), 1);
    }
    let recovered = descent.recovered.map(u128::from);
    Ok(SolveReport {
        recovered,
        exhausted: recovered.is_none(),
        total_gae: phases.iter().map(|p| p.gae).sum(),
        phases,
        automorphisms_used: report.automorphism_order as u32,
        unpriced: unpriced_of(&counters),
        counters,
        deterministic: true,
        nondeterminism: Vec::new(),
        solve_wall_ns,
        factor_base: None,
        detail: json!({
            "scope": "shipped affine implementation versus shipped 32-lane Jacobian implementation; not a coordinate-only comparison",
            "curve": facts.slug,
            "relation_coordinates": relation_coordinates.as_str(),
            "relation_lanes": relation_coordinates.lanes(),
            "collection_wall_ns": report.logs.collection_ns,
            "logs_wall_ns": (report.logs.seconds * 1e9) as u64,
            "descent_verified": descent.verified,
        }),
        online: None,
        online_error: None,
        solver: None,
        field_ops: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ecbench::methods::{resolve, solve as solve_method, MethodSpec};
    use crate::cryptanalysis::ecbench::workload::{CurveSpec, TargetKind, Workload};

    fn method(coordinates: &str) -> ResolvedMethod {
        resolve(&MethodSpec {
            id: ID.into(),
            params: BTreeMap::from([
                ("coordinates".into(), coordinates.into()),
                ("orbits".into(), "16".into()),
                ("relations_per_orbit".into(), "2".into()),
            ]),
        })
        .unwrap()
    }

    #[test]
    fn affine_and_jacobian_records_recover_the_same_workload() {
        let spec = CurveSpec::PrimeSearch {
            bits: 15,
            seed: 20261101,
        };
        let (workload, instance) =
            Workload::build(&spec, 20261101, 0, TargetKind::Planted).unwrap();
        for coordinates in ["affine", "jacobian"] {
            let report = solve_method(
                &method(coordinates),
                &instance,
                &workload.curve,
                &workload.target,
                0x5eed,
            )
            .unwrap();
            assert_eq!(report.recovered, workload.planted, "{coordinates}");
            assert!(!report.exhausted, "{coordinates}");
            assert_eq!(report.counters["rejected_relations"], 0, "{coordinates}");
            assert_eq!(report.counters["verified_descents"], 1, "{coordinates}");
            assert_eq!(
                report.counters["relation_lanes"],
                if coordinates == "affine" { 1 } else { 32 },
                "{coordinates}"
            );
            let phase = report
                .phases
                .iter()
                .find(|phase| phase.name == "relations")
                .unwrap();
            assert!(phase.wall_ns.unwrap() > 0, "{coordinates}");
            assert!(phase.gae > 0.0, "{coordinates}");
        }
    }
}
