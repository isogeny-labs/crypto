//! Adapter for the public synthetic Scholten pencil workload.
//! Full native ledgers are retained; group-only GAE is explicitly a lower bound.
use crate::cryptanalysis::ic_measurement as measurement;
use crate::cryptanalysis::three_leads as engine;
use pencil_symbolic_graph as kernel;
#[path = "../../../research/pencil_large_prime_20261029/src/native_solver.rs"]
mod native_solver;

use super::super::methods::{PhaseRecord, ResolvedMethod, SolveReport};
use engine::{Counts, Point, StudyInstance};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, time::Instant};

fn config(m: &ResolvedMethod) -> Result<native_solver::Config, String> {
    let parse = |key: &str| -> Result<usize, String> {
        m.params
            .get(key)
            .ok_or_else(|| format!("missing {key}"))?
            .parse()
            .map_err(|_| format!("invalid {key}"))
    };
    let config = native_solver::Config {
        denominator: parse("small_denominator")?,
        matrix_coefficient_cap: parse("matrix_coefficient_cap")?,
    };
    config.validate().map_err(|error| error.to_string())?;
    Ok(config)
}

// ecbench replay compares counters, not arbitrary detail. Bind the entire
// deterministic ledger (including failure and phase attribution) into four
// exact u64 words. Timings and allocator observations are not deterministic.
fn replay_ledger(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let sorted: BTreeMap<_, _> = map
                .iter()
                .filter(|(key, _)| {
                    !matches!(
                        key.as_str(),
                        "allocation"
                            | "candidate_envelope_allocation"
                            | "cold_setup_allocation"
                            | "allocation_observer_available"
                            | "elapsed_ns"
                            | "candidate_elapsed_ns"
                            | "cold_setup_elapsed_ns"
                            | "candidate_phase_overhead_ns"
                    )
                })
                .map(|(key, value)| (key.clone(), replay_ledger(value)))
                .collect();
            serde_json::to_value(sorted).expect("JSON ledger")
        }
        Value::Array(values) => Value::Array(values.iter().map(replay_ledger).collect()),
        other => other.clone(),
    }
}
fn ledger_digest(candidate: &Value, setup: &Value) -> [u64; 4] {
    let value = replay_ledger(&json!({"candidate":candidate,"setup":setup}));
    let hash = Sha256::digest(serde_json::to_vec(&value).expect("JSON ledger"));
    std::array::from_fn(|i| u64::from_be_bytes(hash[i * 8..i * 8 + 8].try_into().unwrap()))
}

pub(super) fn solve(
    m: &ResolvedMethod,
    inst: &StudyInstance,
    target: Point,
    seed: u64,
) -> Result<SolveReport, String> {
    let adapter_start = Instant::now();
    let geometry_start = engine::geometry_events::snapshot();
    let config = config(m)?;
    let start = Instant::now();
    // Root ecbench has no installed ObservedSystem. Never fabricate zero memory.
    let result = native_solver::solve_one(inst, target, seed, config, None)
        .map_err(|error| error.to_string())?;
    let geometry_after_solver = engine::geometry_events::snapshot();
    let mut group: Counts = serde_json::from_value(result.detail["counts"].clone())
        .map_err(|e| format!("pencil group ledger: {e}"))?;
    let mut verifier_counts = Counts::default();
    if let Some(k) = result.recovered {
        let verifier = inst.fresh();
        if !result.source_verified || verifier.curve.mul(verifier.generator, k) != target {
            return Err("fresh source verification rejected pencil result".into());
        }
        verifier_counts = verifier.curve.f.counts();
        group = group + verifier_counts;
    }
    let solve_wall_ns =
        u64::try_from(start.elapsed().as_nanos()).map_err(|_| "pencil interval overflow")?;
    let geometry_after_verification = engine::geometry_events::snapshot();
    let unit = inst.fresh();
    let twice = unit.curve.mul(unit.generator, 2);
    let calibration_preparation = unit.curve.f.counts();
    unit.curve.f.reset();
    let _ = unit.curve.add(unit.generator, twice);
    let source_add = unit.curve.f.counts();
    let geometry_after_calibration = engine::geometry_events::snapshot();
    let price = |c: Counts| c.m as f64 + 0.75 * c.s as f64 + 10.0 * c.i as f64;
    if price(source_add) <= 0.0 {
        return Err("zero source-addition unit".into());
    }
    let group_only_gae = price(group) / price(source_add);
    let mut counters = BTreeMap::from([
        ("group_field_multiplications".into(), group.m),
        ("group_field_squarings".into(), group.s),
        ("group_field_inversions".into(), group.i),
        ("group_field_adds_uncharged".into(), group.field_adds),
        ("group_bit_ops_uncharged".into(), group.bit_ops),
    ]);
    for (i, word) in ledger_digest(&result.detail, &result.setup)
        .into_iter()
        .enumerate()
    {
        counters.insert(format!("pencil_replay_ledger_sha256_word_{i}"), word);
    }
    let mut report = SolveReport {
        recovered: result.recovered.map(u128::from),
        exhausted: result.recovered.is_none(),
        phases: vec![PhaseRecord {
            name: "counted_group_component_only".into(),
            adds: group.ec_adds + group.jac_adds,
            doubles: 0,
            scalar_mults: 0,
            gae: group_only_gae,
            wall_ns: None,
            native: counters.clone(),
            work_native: BTreeMap::new(),
        }],
        total_gae: group_only_gae,
        automorphisms_used: 1,
        counters,
        unpriced: vec![
            "pencil_native_and_symbolic_arithmetic".into(),
            "pencil_logical_work".into(),
            "allocation_observer_unavailable".into(),
        ],
        deterministic: true,
        nondeterminism: vec![],
        solve_wall_ns,
        factor_base: None,
        detail: json!({"schema":"ecbench-pencil-symbolic/v2","candidate":result.detail,
            "cold_setup":result.setup,"additional_source_verification":verifier_counts,
            "source_addition":source_add,"cost_status":"incomplete",
            "total_gae_scope":"counted group arithmetic only; lower bound",
            "candidate_over_rho":null,"allocation_observer_available":false}),
        online: None,
        online_error: None,
        solver: None,
        field_ops: None,
    };
    let geometry_end = engine::geometry_events::snapshot();
    report.detail["adapter_accounting"] = json!({
        "schema":"pencil-adapter-accounting/v1-draft",
        "geometry": engine::geometry_events::partition(geometry_start, &[
            ("solver_including_cold_setup", geometry_after_solver),
            ("additional_source_verification", geometry_after_verification),
            ("source_unit_calibration", geometry_after_calibration),
            ("report_assembly", geometry_end),
        ]).ok_or("invalid adapter geometry partition")?,
        "source_unit_calibration_arithmetic": {
            "preparation":calibration_preparation,"addition":source_add,
            "total":calibration_preparation+source_add,
            "included_in_legacy_group_total":false
        },
        "adapter_elapsed_ns_before_accounting_attachment":adapter_start.elapsed().as_nanos(),
        "coverage_complete":false,
        "report_allocation_and_serialization_counted":false,
        "partition_policy":"whole geometry is total; named segments partition it and must not be added again",
        "accounting_attachment_is_observer_overhead":true
    });
    let accounting_replay = json!({
        "geometry":report.detail["adapter_accounting"]["geometry"],
        "calibration":report.detail["adapter_accounting"]["source_unit_calibration_arithmetic"]
    });
    for (i, word) in ledger_digest(&accounting_replay, &Value::Null)
        .into_iter()
        .enumerate()
    {
        report
            .counters
            .insert(format!("adapter_accounting_sha256_word_{i}"), word);
    }
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn audited_digest_covers_work_and_failures_but_excludes_observations() {
        let a = json!({"arithmetic":{"base_muls":17},"failure":"walk_cap",
            "phases":{"sieve":{"elapsed_ns":10,"symbolic":{"map_calls":2}}},
            "candidate_elapsed_ns":19,"allocation":{"alloc_calls":4}});
        let mut b = a.clone();
        b["candidate_elapsed_ns"] = json!(91);
        b["phases"]["sieve"]["elapsed_ns"] = json!(80);
        b["allocation"]["alloc_calls"] = json!(40);
        assert_eq!(
            ledger_digest(&a, &Value::Null),
            ledger_digest(&b, &Value::Null)
        );
        let mut future_work = a.clone();
        future_work["allocation_work_units"] = json!(7);
        assert_ne!(
            ledger_digest(&a, &Value::Null),
            ledger_digest(&future_work, &Value::Null)
        );
        b["phases"]["sieve"]["symbolic"]["map_calls"] = json!(3);
        assert_ne!(
            ledger_digest(&a, &Value::Null),
            ledger_digest(&b, &Value::Null)
        );
        b = a.clone();
        b["failure"] = Value::Null;
        assert_ne!(
            ledger_digest(&a, &Value::Null),
            ledger_digest(&b, &Value::Null)
        );
    }
}
