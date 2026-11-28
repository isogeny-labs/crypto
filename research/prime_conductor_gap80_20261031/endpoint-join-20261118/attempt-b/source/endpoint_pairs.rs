//! Lossless structural join of endpoint observations. Never admits a result.
use crypto_lib::cryptanalysis::ecbench::{canonical, record, runner, spec};
use serde::Serialize;
use serde_json::{value::RawValue, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

#[derive(Serialize)]
pub struct Endpoint {
    execution: spec::Execution,
    workload_id: String,
    workload_sha256: String,
    /// Original JSON token bytes, not a re-serialization of parsed floats.
    record: Option<Box<RawValue>>,
}

#[derive(Serialize)]
pub struct Pair {
    r: String,
    target_seed: u64,
    target_index: u64,
    round: u32,
    warmup: bool,
    source: Endpoint,
    destination: Endpoint,
    source_status: String,
    destination_status: String,
}

#[derive(Serialize)]
pub struct Joined {
    schema: &'static str,
    scope: &'static str,
    plan_only: bool,
    spec_id: String,
    spec_sha256: String,
    plan_sha256: String,
    session_sha256: Option<String>,
    records_sha256: Option<String>,
    session: Option<runner::Session>,
    source_curve: String,
    destination_curve: String,
    arm: spec::Arm,
    observations_in_input: usize,
    observations_selected: usize,
    pairs: Vec<Pair>,
    result_admitted: bool,
    conductor_gap: Option<String>,
    cost_ratio: Option<f64>,
    wall_ratio: Option<f64>,
    obligations: [&'static str; 5],
}

fn read(dir: &Path, name: &str) -> Result<String, String> {
    std::fs::read_to_string(dir.join(name)).map_err(|e| format!("{name}: {e}"))
}

fn decode<T: serde::de::DeserializeOwned>(text: &str) -> Result<T, String> {
    canonical::reject_duplicate_keys(text)?;
    serde_json::from_str(text).map_err(|e| e.to_string())
}

fn require(ok: bool, message: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(message.into())
    }
}

/// Inputs are snapshots. Rebuild the original spec/plan; do not relabel either
/// workload to fit the same-curve comparator. A missing record stays missing.
pub fn join(
    dir: &Path,
    source: &str,
    destination: &str,
    arm: &str,
    plan_only: bool,
) -> Result<Joined, String> {
    require(
        source != destination,
        "select distinct endpoint curve slugs",
    )?;
    let spec_text = read(dir, "spec.json")?;
    canonical::reject_duplicate_keys(&spec_text)?;
    let specification = spec::Spec::from_json(&spec_text)?;
    require(
        specification.measurement.order == spec::Order::PairedCurvesV1,
        "endpoint join requires paired_curves_v1",
    )?;
    let planned = spec::plan(specification)?;
    let expected = runner::PlanDoc {
        schema: "ecbench.plan/v1".into(),
        spec_id: planned.spec_id.clone(),
        arms: planned.arms.clone(),
        workloads: planned.workloads.clone(),
        executions: planned.executions.clone(),
    };
    let plan_text = read(dir, "plan.json")?;
    let actual: Value = decode(&plan_text)?;
    require(
        actual == serde_json::to_value(&expected).map_err(|e| e.to_string())?,
        "plan differs from re-expanded spec",
    )?;
    let selected_arm = planned
        .arms
        .iter()
        .position(|a| a.name == arm)
        .ok_or("unknown arm")?;
    let plan_sha256 = canonical::sha256_hex(plan_text.as_bytes());
    let (session, session_sha256, records_text) = if plan_only {
        require(
            !dir.join("session.json").exists()
                && (!dir.join("records.jsonl").exists() || read(dir, "records.jsonl")?.is_empty()),
            "plan-only mode refuses a session or nonempty observations",
        )?;
        (None, None, String::new())
    } else {
        let text = read(dir, "session.json")?;
        let session: runner::Session = decode(&text)?;
        let records = read(dir, "records.jsonl")?;
        let host = read(dir, "host.json")?;
        let _: Value = decode(&host)?;
        require(
            session.schema == "ecbench.session/v1"
                && matches!(session.status.as_str(), "complete" | "interrupted")
                && !session.session_id.is_empty()
                && session.spec_id == planned.spec_id
                && session.spec_sha256 == planned.spec_sha256
                && session.plan_sha256 == plan_sha256
                && session.host_sha256 == canonical::sha256_hex(host.as_bytes())
                && session.records_sha256.as_deref()
                    == Some(canonical::sha256_hex(records.as_bytes()).as_str())
                && session.executions_planned == planned.executions.len() as u64,
            "session identity, status, or input digest mismatch",
        )?;
        require(
            session
                .binary_sha256
                .as_ref()
                .is_some_and(|s| s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit())),
            "session needs an executable SHA-256",
        )?;
        (
            Some(session),
            Some(canonical::sha256_hex(text.as_bytes())),
            records,
        )
    };

    // Index every execution, including other arms/curves, to detect unexpected
    // observations and preserve the runner's method/workload repetition law.
    let mut expected_runs = BTreeMap::new();
    let mut repetitions = BTreeMap::new();
    for execution in &planned.executions {
        let a = &planned.arms[execution.arm];
        let w = &planned.workloads[execution.workload];
        let n = repetitions
            .entry((&a.method.method_id, &w.workload_id))
            .or_insert(0_u64);
        *n += 1;
        expected_runs.insert(
            execution.seq,
            (
                execution,
                format!("{}{}R{n}", a.method.method_id, w.workload_id),
            ),
        );
    }
    let mut observations = BTreeMap::new();
    let mut record_ids = BTreeSet::new();
    let mut counts = BTreeMap::<String, u64>::new();
    for line in records_text.lines().filter(|l| !l.trim().is_empty()) {
        let r: record::Record = decode(line)?;
        let s = session.as_ref().ok_or("observations require a session")?;
        let (e, run_id) = expected_runs.get(&r.seq).ok_or("unplanned execution")?;
        let a = &planned.arms[e.arm];
        let w = &planned.workloads[e.workload];
        require(
            r.schema == record::RECORD_SCHEMA
                && record::check_line_seal(line, &r.record_id)
                && r.session_id == s.session_id
                && r.binary_sha256 == s.binary_sha256
                && r.env_class_id == s.env_class_id
                && r.run_id == *run_id
                && r.round == e.round
                && r.warmup == e.warmup
                && r.arm == a.name
                && serde_json::to_value(a.role).map_err(|e| e.to_string())? == r.role
                && r.method == a.method
                && r.workload == *w
                && r.algorithm_seed == e.algorithm_seed.to_string(),
            "record seal or planned identity mismatch",
        )?;
        require(
            r.cost.unit == record::UNIT
                && [r.cost.total_gae, r.cost.s]
                    .iter()
                    .flatten()
                    .all(|x| x.is_finite() && *x >= 0.0),
            "record cost has unexpected units or invalid values",
        )?;
        require(
            matches!(
                r.outcome.status.as_str(),
                "verified" | "wrong_answer" | "exhausted" | "error" | "timeout" | "crashed"
            ),
            "unknown outcome status",
        )?;
        require(
            record_ids.insert(r.record_id.clone()),
            "duplicate record id",
        )?;
        *counts.entry(r.outcome.status.clone()).or_default() += 1;
        require(
            observations.insert(r.seq, (line, r)).is_none(),
            "duplicate execution observation",
        )?;
    }
    if let Some(s) = &session {
        require(
            s.records_written == observations.len() as u64
                && s.status_counts == counts
                && s.last_seq == observations.keys().next_back().copied()
                && (s.status == "interrupted" || s.records_written == s.executions_planned),
            "session record count, status counts, or last sequence mismatch",
        )?;
    }
    type Key = (String, u64, u64, u32, bool);
    let mut sides = [BTreeMap::<Key, &spec::Execution>::new(), BTreeMap::new()];
    for e in &planned.executions {
        if e.arm != selected_arm {
            continue;
        }
        let w = &planned.workloads[e.workload];
        let side = if w.curve.slug == source {
            0
        } else if w.curve.slug == destination {
            1
        } else {
            continue;
        };
        let key = (
            w.curve.r.to_string(),
            w.target_seed,
            w.target_index,
            e.round,
            e.warmup,
        );
        require(
            sides[side].insert(key, e).is_none(),
            "ambiguous endpoint execution",
        )?;
    }
    require(
        !sides[0].is_empty() && sides[0].keys().eq(sides[1].keys()),
        "endpoint target/order/round coverage mismatch or unknown curve",
    )?;
    let mut pairs = Vec::new();
    let mut selected = 0;
    for ((r, target_seed, target_index, round, warmup), a) in &sides[0] {
        let b = sides[1][&(r.clone(), *target_seed, *target_index, *round, *warmup)];
        let wa = &planned.workloads[a.workload];
        let wb = &planned.workloads[b.workload];
        require(
            a.algorithm_seed == b.algorithm_seed
                && wa.planted.is_some()
                && wa.planted == wb.planted
                && wa.target_law == wb.target_law,
            "paired seed, target law, or planted scalar mismatch",
        )?;
        let mut endpoint = |e: &spec::Execution| -> Result<(Endpoint, String), String> {
            let w = &planned.workloads[e.workload];
            let observation = observations.get(&e.seq);
            selected += usize::from(observation.is_some());
            Ok((
                Endpoint {
                    execution: e.clone(),
                    workload_id: w.workload_id.clone(),
                    workload_sha256: w.workload_sha256.clone(),
                    record: observation
                        .map(|(line, _)| RawValue::from_string((*line).into()))
                        .transpose()
                        .map_err(|e| e.to_string())?,
                },
                observation
                    .map_or("missing", |(_, r)| r.outcome.status.as_str())
                    .into(),
            ))
        };
        let (source, source_status) = endpoint(a)?;
        let (destination, destination_status) = endpoint(b)?;
        pairs.push(Pair {
            r: r.clone(),
            target_seed: *target_seed,
            target_index: *target_index,
            round: *round,
            warmup: *warmup,
            source,
            destination,
            source_status,
            destination_status,
        });
    }
    Ok(Joined {
        schema: "ecbench.endpoint_pairs/v1",
        scope: "structural_join_only",
        plan_only,
        spec_id: planned.spec_id,
        spec_sha256: planned.spec_sha256,
        plan_sha256,
        session_sha256,
        records_sha256: (!plan_only).then(|| canonical::sha256_hex(records_text.as_bytes())),
        session,
        source_curve: source.into(),
        destination_curve: destination.into(),
        arm: planned.arms[selected_arm].clone(),
        observations_in_input: observations.len(),
        observations_selected: selected,
        pairs,
        result_admitted: false,
        conductor_gap: None,
        cost_ratio: None,
        wall_ratio: None,
        obligations: [
            "Independent ecbench verify/replay and record accounting audit",
            "Certified endpoint conductors, explicit isogeny and subgroup/target transport",
            "Missing/failure/censoring and unpriced-work analysis before statistics",
            "Target-cluster statistics; warmups excluded; no complete-case substitution",
            "Wall isolation, host/binary, A/A, registration and sample-size admission",
        ],
    })
}
