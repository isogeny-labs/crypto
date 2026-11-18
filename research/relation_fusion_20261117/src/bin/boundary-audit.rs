//! Supplemental child-clock closure and exclusive parent phase audit; no runtime admission.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{error::Error, fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
include!("../packed_roster.rs");
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn read(p: &Path) -> Result<Vec<u8>> {
    let m = fs::symlink_metadata(p)?;
    if !m.is_file() || m.file_type().is_symlink() || m.len() > 16 * 1024 * 1024 {
        return Err("file type/size".into());
    }
    let b = fs::read(p)?;
    if b.len() > 16 * 1024 * 1024 {
        return Err("file growth".into());
    }
    Ok(b)
}
fn value(p: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&read(p)?)?)
}
fn require(b: bool, s: &str) -> Result<()> {
    if b {
        Ok(())
    } else {
        Err(s.into())
    }
}
const PHASES: [&str; 8] = [
    "capture_input_and_deadline_gate",
    "spawn_call",
    "wait_for_child",
    "process_receipt_write",
    "output_parse_and_child_gates",
    "artifact_readback_and_input_binding",
    "controller_whole_proof_replay",
    "basis_stat_cross_arm_binding_and_digests",
];
fn audit(
    raw: &Path,
    pins: &Value,
    trace: bool,
    override_first_process_clock: Option<Value>,
) -> Result<Value> {
    let planned = if trace { 324 } else { 1260 };
    let summary = value(&raw.join("summary.json"))?;
    let rows = summary["outcomes"].as_array().ok_or("outcomes")?;
    let expected = &pins["worker_sha256"];
    require(
        value(&raw.join("workers-before.json"))? == *expected
            && value(&raw.join("workers-after.json"))? == *expected,
        "worker pins",
    )?;
    let complete = rows.len() == planned
        && summary["planned"] == planned
        && summary["all_correct"] == true
        && rows.iter().all(|r| r["status"] == "complete");
    let get = |stage: &str, round: usize, p: u64, n: u64, arm: &str| -> Result<&Value> {
        let found: Vec<_> = rows
            .iter()
            .filter(|r| {
                r["stage"] == stage
                    && r["round"] == round
                    && r["prime"] == p
                    && r["n"] == n
                    && r["arm"] == arm
            })
            .collect();
        require(found.len() == 1, "missing/duplicate cell")?;
        Ok(found[0])
    };
    require(complete, "complete immutable roster required")?;
    let mut fingerprints = std::collections::BTreeMap::new();
    let mut replays = 0usize;
    if complete {
        // Audit the complete immutable roster, raw outputs, original pins and complete proofs.
        for stage in if trace {
            vec!["TRACE"]
        } else {
            vec!["AA", "AB"]
        } {
            for round in 0..if trace { 3 } else { 5 } {
                for p in [2147483647u64, 9007199254740881] {
                    for n in [3u64, 4, 6] {
                        for arm in roster(if trace { "AB" } else { stage }) {
                            let row = get(stage, round, p, n, &arm.name)?;
                            require(
                                row["kind"] == arm.kind
                                    && row["kernel"] == arm.kernel
                                    && row["checker"] == "packed",
                                "arm identity",
                            )?;
                            let dir = raw.join(format!("{stage}-r{round}-p{p}-n{n}-{}", arm.name));
                            require(
                                value(&dir.join("outcome.json"))? == *row,
                                "summary/raw binding",
                            )?;
                            let mut process = value(&dir.join("process.json"))?;
                            if replays == 0 {
                                if let Some(v) = &override_first_process_clock {
                                    process["parent_ns"] = v.clone();
                                }
                            }
                            require(
                                process["parent_ns"].as_u64().is_some()
                                    && row["child_process_ns"] == process["parent_ns"],
                                "process clock cross-binding",
                            )?;
                            require(
                                process["exit"] == 0
                                    && process["driver_sha256"] == expected[arm.worker],
                                "child receipt",
                            )?;
                            let input = read(&dir.join("proof/original.json"))?;
                            let wire = read(&dir.join("proof/witness.json"))?;
                            let input_pin = pins["input_sha256"][format!("p{p}-n{n}")]
                                .as_str()
                                .ok_or("input pin")?;
                            require(
                                sha(&input) == input_pin
                                    && process["input_sha256"] == input_pin
                                    && row["proof_sha256"] == sha(&wire),
                                "original/proof binding",
                            )?;
                            let checked = if arm.kind.ends_with("-dag") {
                                relation_replay_checker::packed::verify_dag_json(&input, &wire)?
                            } else {
                                relation_replay_checker::packed::verify_json(&input, &wire)?
                            };
                            replays += 1;
                            let fingerprint = sha(&serde_json::to_vec(&checked.basis)?);
                            if let Some(old) = fingerprints.insert((p, n), fingerprint.clone()) {
                                require(old == fingerprint, "cross-arm checked basis")?;
                            }
                            let worker = value(&dir.join("call.stdout"))?;
                            require(
                                worker["basis"] == serde_json::to_value(&checked.basis)?
                                    && worker["consumer_stats"]
                                        == serde_json::to_value(&checked.stats)?
                                    && row["basis_sha256"]
                                        == sha(&serde_json::to_vec(&checked.basis)?),
                                "checked basis/stats",
                            )?;
                            require(
                                read(&dir.join("call.stderr"))?.is_empty(),
                                "child diagnostics",
                            )?;
                            let parent = row["parent_ns"].as_u64().ok_or("parent clock")?;
                            let child = row["child_process_ns"].as_u64().ok_or("child clock")?;
                            let phase = |name: &str| -> Result<u64> {
                                Ok(worker["phases_diagnostic_ns"][name]
                                    .as_str()
                                    .ok_or("worker phase")?
                                    .parse()?)
                            };
                            let construct = phase("construct_and_wire")?;
                            let replay = phase("independent_replay")?;
                            let whole = phase("full_worker_including_proof_write")?;
                            require(
                                worker["status"] == "complete"
                                    && worker["prime"] == p
                                    && worker["variables"] == n
                                    && worker["checker"] == "packed"
                                    && worker["kernel"] == arm.kernel
                                    && worker["S_pair_and_input_containment_checks"] == true,
                                "worker identity/status",
                            )?;
                            require(
                                parent > 0
                                    && child > 0
                                    && parent >= child
                                    && whole <= child
                                    && construct.checked_add(replay).is_some_and(|v| v <= whole)
                                    && row["construct_ns"] == construct
                                    && row["replay_ns"] == replay
                                    && row["worker_ns"] == whole,
                                "complete phase/boundary binding",
                            )?;
                            if trace {
                                let parts = row["parent_exclusive_ns"]
                                    .as_object()
                                    .ok_or("exclusive phases")?;
                                require(
                                    parts.len() == PHASES.len() && row["trace_only"] == true,
                                    "trace partition shape",
                                )?;
                                let sum = PHASES
                                    .iter()
                                    .try_fold(0u64, |sum, key| {
                                        sum.checked_add(parts[*key].as_u64()?)
                                    })
                                    .ok_or("phase type/sum")?;
                                let residual =
                                    row["residual_bookkeeping_ns"].as_u64().ok_or("residual")?;
                                require(
                                    sum.checked_add(residual) == Some(parent),
                                    "exclusive phase accounting",
                                )?;
                            }
                        }
                    }
                }
            }
        }
    }

    let mut cases = Vec::new();
    if trace {
        for p in [2147483647u64, 9007199254740881] {
            for n in [3u64, 4, 6] {
                let mut arms = Vec::new();
                for arm in roster("AB") {
                    let mut totals = Vec::new();
                    let mut sum_parent = 0u128;
                    let mut sums = std::collections::BTreeMap::<String, u128>::new();
                    let mut child_outside_worker = Vec::new();
                    for r in 0..3 {
                        let row = get("TRACE", r, p, n, &arm.name)?;
                        let parent = row["parent_ns"].as_u64().ok_or("parent")?;
                        totals.push(parent);
                        sum_parent += parent as u128;
                        for key in PHASES {
                            *sums.entry(key.to_string()).or_default() +=
                                row["parent_exclusive_ns"][key].as_u64().ok_or("phase")? as u128;
                        }
                        *sums.entry("residual_bookkeeping".into()).or_default() +=
                            row["residual_bookkeeping_ns"].as_u64().ok_or("residual")? as u128;
                        let child = row["child_process_ns"].as_u64().ok_or("child")?;
                        let worker = row["worker_ns"].as_u64().ok_or("worker")?;
                        child_outside_worker.push(
                            child
                                .checked_sub(worker)
                                .ok_or("worker/process containment")?,
                        );
                    }
                    totals.sort_unstable();
                    child_outside_worker.sort_unstable();
                    let means = sums
                        .iter()
                        .map(|(k, v)| (k.clone(), *v as f64 / 3.0))
                        .collect::<std::collections::BTreeMap<_, _>>();
                    let shares = sums
                        .iter()
                        .map(|(k, v)| (k.clone(), *v as f64 / sum_parent as f64))
                        .collect::<std::collections::BTreeMap<_, _>>();
                    arms.push(json!({"arm":arm.name,"median_parent_ns":totals[1],"sorted_parent_ns":totals,"mean_parent_ns":sum_parent as f64/3.0,"mean_exclusive_phase_ns":means,"aggregate_exclusive_phase_shares":shares,"median_process_interval_minus_reported_worker_ns":child_outside_worker[1]}));
                }
                cases.push(json!({"prime":p,"n":n,"arms":arms}));
            }
        }
    }
    Ok(
        json!({"schema":"boundary-receipt-audit/v1","trace_only":trace,"planned":planned,"complete_whole_proof_replays":replays,"process_clock_cross_bindings":replays,"phase_partitions_checked":if trace{replays}else{0},"cases":cases,"runtime_improvement_admitted":false,"signed_execution_verdict":false}),
    )
}

fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 6 {
        return Err("usage:boundary-audit 200|trace RAW MONITOR PINS NEW_OUT".into());
    }
    let trace = match a[1].as_str() {
        "200" => false,
        "trace" => true,
        _ => return Err("mode".into()),
    };
    let root = Path::new(&a[2]);
    let pins = value(Path::new(&a[4]))?;
    let mut report = audit(root, &pins, trace, None)?;
    let summary = value(&root.join("summary.json"))?;
    let first = summary["outcomes"][0]["child_process_ns"]
        .as_u64()
        .ok_or("first child clock")?;
    let mut sabotages = Vec::new();
    for (name, clock) in [
        ("changed_process_clock", json!(first + 1)),
        ("null_process_clock", Value::Null),
    ] {
        let err = audit(root, &pins, trace, Some(clock)).expect_err("clock sabotage accepted");
        require(
            err.to_string() == "process clock cross-binding",
            "sabotage rejection reason",
        )?;
        sabotages.push(json!({"kind":name,"rejected":true,"reason":err.to_string()}));
    }
    let monitors: Vec<Value> = fs::read_to_string(&a[3])?
        .lines()
        .map(serde_json::from_str)
        .collect::<std::result::Result<_, _>>()?;
    require(monitors.len() == 1, "monitor count")?;
    report["monitor_exit_status"] = monitors[0]["exit_status"].clone();
    report["monitor_contended_samples"] = monitors[0]["contended_samples"].clone();
    report["clock_sabotages"] = json!(sabotages);
    let out = Path::new(&a[5]);
    fs::create_dir(out)?;
    fs::File::options()
        .write(true)
        .create_new(true)
        .open(out.join("audit.json"))?
        .write_all(&serde_json::to_vec_pretty(&report)?)?;
    println!(
        "{}",
        json!({"whole_proof_replays":report["complete_whole_proof_replays"],"phase_partitions":report["phase_partitions_checked"],"clock_sabotages_rejected":2,"runtime_improvement_admitted":false})
    );
    Ok(())
}
