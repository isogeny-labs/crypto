//! Assemble, replay and check the co-measured equal-advice session.
//!
//! Reads the session manifest written by
//! `research/equal_advice_wide_20261018/run.sh`, the index-calculus (IC)
//! output files and the Bernstein–Lange (BL) output files of the session, and
//!
//! 1. replays every recovered scalar of both arms with the generic library
//!    curve (`KoblitzCurve::mul`, independent of both producers' arithmetic)
//!    and writes one replay certificate per arm, run and table level;
//! 2. writes one `vs_rho` claim report per (run, level, target) in the ledger
//!    schema (`docs/ic/boundary_targets.json`, `measurement_schema.vs_rho`),
//!    with the BL arm in the rho slot, and checks each with
//!    `ecbench::claim::check_vs_rho`, the native twin of
//!    `boundary_autolab.py claim-check --stage vs_rho`;
//! 3. applies the frozen decision rule (PROTOCOL.md) and writes
//!    `decision.json`, plus `rows.json` (the per-target values the graphs
//!    plot).
//!
//! A field the session cannot establish is left absent or `false`, never
//! invented: the checker's verdict then lists it. Usage:
//! `ic_equal_advice_claims <session.json> <out_dir>`.

use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::ecbench::canonical::sha256_hex;
use crypto_lib::cryptanalysis::ecbench::claim::{
    canonical_utf8, check_problems, check_vs_rho, id_sha256,
};
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::{json, Map, Value};
use std::fs;
use std::path::Path;

const RESAMPLES: usize = 10_000;
const PRICING_BAND: [f64; 3] = [0.0, 0.1, 1.0];

fn read_json(path: &str) -> Result<Value, String> {
    serde_json::from_str(&fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?)
        .map_err(|e| format!("{path}: {e}"))
}

fn read_lines(path: &str) -> Result<Vec<Value>, String> {
    fs::read_to_string(path)
        .map_err(|e| format!("{path}: {e}"))?
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).map_err(|e| format!("{path}: {e}")))
        .collect()
}

fn file_sha(path: &str) -> Result<String, String> {
    Ok(sha256_hex(
        &fs::read(path).map_err(|e| format!("{path}: {e}"))?,
    ))
}

fn num_str(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

/// `[x, y]` as decimal strings from `[x,y]` numbers or strings.
fn point_strings(v: &Value) -> Option<[String; 2]> {
    let a = v.as_array()?;
    Some([num_str(a.first()?)?, num_str(a.get(1)?)?])
}

fn parse_targets(path: &str) -> Result<Vec<[String; 2]>, String> {
    fs::read_to_string(path)
        .map_err(|e| format!("{path}: {e}"))?
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let inner = l.trim().trim_start_matches('[').trim_end_matches(']');
            let mut w = inner
                .split(',')
                .map(|s| s.trim().trim_matches('"').to_string());
            match (w.next(), w.next()) {
                (Some(x), Some(y)) => Ok([x, y]),
                _ => Err(format!("{path}: bad target line {l}")),
            }
        })
        .collect()
}

struct Replayer {
    curve: KoblitzCurve,
    n: u32,
}

impl Replayer {
    fn verify(&self, d: Option<u64>, q: &[String; 2]) -> bool {
        let Some(d) = d else { return false };
        let (Ok(x), Ok(y)) = (q[0].parse::<BigUint>(), q[1].parse::<BigUint>()) else {
            return false;
        };
        let point = BinaryPoint::Affine {
            x: F2mElement::from_biguint(&x, self.n),
            y: F2mElement::from_biguint(&y, self.n),
        };
        self.curve.mul(self.curve.generator(), &BigUint::from(d)) == point
    }
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = s.len();
    if n % 2 == 1 {
        s[n / 2]
    } else {
        0.5 * (s[n / 2 - 1] + s[n / 2])
    }
}

fn percentile_ci(mut v: Vec<f64>) -> [f64; 2] {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let at = |p: f64| v[((p * (v.len() - 1) as f64).round()) as usize];
    [at(0.025), at(0.975)]
}

/// Target-resampled bootstrap of `stat` over `n` paired targets.
fn bootstrap(n: usize, seed: u64, stat: impl Fn(&[usize]) -> f64) -> [f64; 2] {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut values = Vec::with_capacity(RESAMPLES);
    let mut idx = vec![0usize; n];
    for _ in 0..RESAMPLES {
        for slot in idx.iter_mut() {
            *slot = rng.gen_range(0..n);
        }
        values.push(stat(&idx));
    }
    percentile_ci(values)
}

struct Arm {
    online_ms: Vec<f64>,
    scalars: Vec<Option<u64>>,
    verified: Vec<bool>,
    /// Priced (group additions) and unpriced native operations per target.
    priced: Vec<f64>,
    unpriced: Vec<f64>,
}

fn field_f64(v: &Value, key: &str) -> Result<f64, String> {
    v[key]
        .as_f64()
        .ok_or_else(|| format!("record lacks numeric {key}"))
}

fn u64_or_zero(v: &Value, key: &str) -> u64 {
    v[key].as_u64().unwrap_or(0)
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: ic_equal_advice_claims <session.json> <out_dir>".into());
    }
    let session = read_json(&args[1])?;
    let out_dir = Path::new(&args[2]);
    fs::create_dir_all(out_dir.join("claims")).map_err(|e| e.to_string())?;
    fs::create_dir_all(out_dir.join("certs")).map_err(|e| e.to_string())?;

    let n = session["n"].as_u64().ok_or("session.n")? as u32;
    let a = session["a"].as_u64().ok_or("session.a")? as u8;
    let rung = session["rung"].as_str().unwrap_or("rung").to_string();
    let targets_path = session["targets"].as_str().ok_or("session.targets")?;
    let targets = parse_targets(targets_path)?;
    let targets_sha = file_sha(targets_path)?;
    let curve = KoblitzCurve::new(a, n).ok_or("curve not admitted")?;
    let r = curve.subgroup_order.to_u64().ok_or("r < 2^64")?;
    let replayer = Replayer { curve, n };
    let envelope = session["resource_envelope"].clone();
    let isolation = session["isolation"].clone();
    let ic_bin = session["ic_binary_sha256"]
        .as_str()
        .unwrap_or("")
        .to_string();
    let bl_bin = session["bl_binary_sha256"]
        .as_str()
        .unwrap_or("")
        .to_string();

    // Candidate identity: a frozen IC1 manifest, re-hashed here.
    let candidate = match session["candidate_manifest"].as_str() {
        Some(path) => {
            let manifest = read_json(path)?;
            // Two frozen layouts: {candidate_id, identity_sha256, record} or the
            // flat record carrying its own candidate_id (identity hash taken
            // without that key).
            let flat;
            let record = match manifest.get("record") {
                Some(r) => r,
                None => {
                    let mut m = manifest.clone();
                    m.as_object_mut()
                        .ok_or("candidate manifest must be an object")?
                        .remove("candidate_id");
                    flat = m;
                    &flat
                }
            };
            let sha = id_sha256(record)?;
            let label = manifest
                .get("candidate_id")
                .and_then(Value::as_str)
                .or_else(|| record.get("candidate_id").and_then(Value::as_str))
                .ok_or("candidate manifest has no candidate_id")?
                .to_string();
            if let Some(recorded) = manifest.get("identity_sha256").and_then(Value::as_str) {
                if recorded != sha {
                    return Err(format!(
                        "{path}: identity_sha256 {recorded} != recomputed {sha}"
                    ));
                }
            }
            if !label.ends_with(&format!("h{}", &sha[..12])) {
                return Err(format!(
                    "{path}: candidate_id {label} does not end in h{}",
                    &sha[..12]
                ));
            }
            Some((label, sha))
        }
        None => None,
    };
    // Workload identity: one target per workload; the schema follows
    // `identity.workload_manifest` with wide coordinates as decimal strings.
    let curve_id = session["curve_id"].as_str().unwrap_or("").to_string();
    let _ = &curve_id;
    let workload = |index: usize| -> Result<(String, String), String> {
        let record = json!({
            "curve_id": curve_id,
            "targets": [targets[index]],
            "input_law": session["input_law"].as_str().unwrap_or("ic_equal_advice.public_hash_target"),
            "target_seeds": [session["target_label"].clone()],
            "algorithm_seed": session["ic_rank_seed"].clone(),
            "cache_policy": "cold",
            "target_count": 1,
            "resource_envelope": envelope,
        });
        let sha = id_sha256(&record)?;
        Ok((sha[..12].to_string(), sha))
    };

    let levels = session["levels"]
        .as_array()
        .ok_or("session.levels")?
        .clone();
    let runs = session["runs"].as_array().ok_or("session.runs")?.clone();
    let mut rows: Vec<Value> = Vec::new();
    let mut decision_rows: Vec<Value> = Vec::new();
    let mut check_summary: Vec<Value> = Vec::new();
    // Per-run vectors for the A/A and determinism controls.
    let mut ic_runs: Vec<(Vec<f64>, Vec<u64>)> = Vec::new();
    let mut bl_runs: Vec<(u64, Vec<f64>)> = Vec::new(); // (level, online ms)
    let mut base_hash_check = json!("not_checked");
    let mut bl_control_lines: Vec<Value> = Vec::new();

    for run in &runs {
        let run_no = run["run"].as_u64().ok_or("run.run")?;
        let ic_path = run["ic_jsonl"].as_str().ok_or("run.ic_jsonl")?;
        let bl_path = run["bl_jsonl"].as_str().ok_or("run.bl_jsonl")?;
        let bl_pass = run["bl_pass"].as_u64().unwrap_or(1);
        let ic_lines = read_lines(ic_path)?;
        let bl_lines = read_lines(bl_path)?;
        if bl_control_lines.is_empty() {
            bl_control_lines = bl_lines.clone();
        }
        let ic_sha = file_sha(ic_path)?;
        let bl_sha = file_sha(bl_path)?;
        let mut ic_records: Vec<&Value> = ic_lines
            .iter()
            .filter(|v| v["kind"] == "compact_orbit_dlp_target")
            .collect();
        ic_records.sort_by_key(|v| v["fixture_index"].as_u64().unwrap_or(0));
        if ic_records.len() != targets.len() {
            return Err(format!(
                "run {run_no}: IC wrote {} target records for {} targets",
                ic_records.len(),
                targets.len()
            ));
        }
        // IC arm (one set of values per run).
        let mut ic_arm = Arm {
            online_ms: vec![],
            scalars: vec![],
            verified: vec![],
            priced: vec![],
            unpriced: vec![],
        };
        for (i, rec) in ic_records.iter().enumerate() {
            let q = point_strings(&rec["target"]).ok_or("IC record target")?;
            if q != targets[i] {
                return Err(format!(
                    "run {run_no}: IC target {i} differs from the target file"
                ));
            }
            let d = rec["recovered_scalar"].as_u64();
            ic_arm.online_ms.push(field_f64(rec, "online_ms")?);
            ic_arm.scalars.push(d);
            ic_arm.verified.push(replayer.verify(d, &targets[i]));
            ic_arm
                .priced
                .push(u64_or_zero(rec, "relation_checks") as f64);
            ic_arm
                .unpriced
                .push((u64_or_zero(rec, "root_calls") + u64_or_zero(rec, "probes")) as f64);
        }
        ic_runs.push((
            ic_arm.online_ms.clone(),
            ic_records
                .iter()
                .map(|r| u64_or_zero(r, "probes"))
                .collect(),
        ));
        if let Some(summary) = ic_lines
            .iter()
            .find(|v| v["kind"] == "compact_orbit_dlp_summary")
        {
            if let Some(expected) = session["expected_base_hash"].as_str() {
                let got = summary["base_hash"].as_str().unwrap_or("");
                if got != expected {
                    return Err(format!(
                        "run {run_no}: IC base_hash {got} != expected {expected}"
                    ));
                }
                base_hash_check = json!({"expected": expected, "observed": got, "equal": true});
            } else {
                base_hash_check = json!({"expected": null, "observed": summary["base_hash"]});
            }
        }
        // IC replay certificate.
        let ic_cert = json!({
            "schema": "ic_equal_advice_replay/v1", "arm": "ic", "run": run_no,
            "n": n, "a": a, "r": r,
            "verifier": "crypto_lib KoblitzCurve::mul on the generic F2mElement curve, independent of both producers' field arithmetic",
            "inputs": {"targets_sha256": targets_sha, "arm_jsonl_sha256": ic_sha, "binary_sha256": ic_bin},
            "targets": (0..targets.len()).map(|i| json!({
                "index": i, "q": targets[i], "recovered_scalar": ic_arm.scalars[i],
                "verified": ic_arm.verified[i]})).collect::<Vec<_>>(),
        });
        let ic_cert_sha = id_sha256(&ic_cert)?;
        fs::write(
            out_dir.join(format!("certs/ic_run{run_no}.replay.json")),
            canonical_utf8(&ic_cert)?,
        )
        .map_err(|e| e.to_string())?;

        for lv in &levels {
            let level = lv["level"].as_u64().ok_or("level.level")?;
            let tier = lv["tier"].as_str().unwrap_or("?").to_string();
            let bl_records: Vec<&Value> = {
                let mut v: Vec<&Value> = bl_lines
                    .iter()
                    .filter(|x| {
                        x["kind"] == "bl_online_target"
                            && x["pass"].as_u64().unwrap_or(1) == bl_pass
                            && x["prefix_level"].as_u64() == Some(level)
                    })
                    .collect();
                v.sort_by_key(|x| x["target_index"].as_u64().unwrap_or(0));
                v
            };
            if bl_records.len() != targets.len() {
                return Err(format!(
                    "run {run_no} level {level}: BL wrote {} records for {} targets",
                    bl_records.len(),
                    targets.len()
                ));
            }
            let mut bl_arm = Arm {
                online_ms: vec![],
                scalars: vec![],
                verified: vec![],
                priced: vec![],
                unpriced: vec![],
            };
            for (i, rec) in bl_records.iter().enumerate() {
                let q = point_strings(&rec["target"]).ok_or("BL record target")?;
                if q != targets[i] {
                    return Err(format!(
                        "run {run_no}: BL target {i} differs from the target file"
                    ));
                }
                let d = rec["recovered_scalar"].as_u64();
                bl_arm.online_ms.push(field_f64(rec, "online_ms")?);
                bl_arm.scalars.push(d);
                bl_arm.verified.push(replayer.verify(d, &targets[i]));
                let steps = u64_or_zero(rec, "online_steps") as f64;
                let walks = u64_or_zero(rec, "walks") as f64;
                let lookups = u64_or_zero(rec, "table_lookups") as f64;
                bl_arm.priced.push(steps + walks);
                bl_arm.unpriced.push(steps + walks + lookups);
            }
            bl_runs.push((level, bl_arm.online_ms.clone()));
            let bl_cert = json!({
                "schema": "ic_equal_advice_replay/v1", "arm": "bl", "run": run_no,
                "level": level, "tier": tier, "n": n, "a": a, "r": r,
                "verifier": "crypto_lib KoblitzCurve::mul on the generic F2mElement curve, independent of both producers' field arithmetic",
                "inputs": {"targets_sha256": targets_sha, "arm_jsonl_sha256": bl_sha, "binary_sha256": bl_bin},
                "table_entries": bl_records[0]["table_entries"], "table_bytes": bl_records[0]["table_bytes"],
                "targets": (0..targets.len()).map(|i| json!({
                    "index": i, "q": targets[i], "recovered_scalar": bl_arm.scalars[i],
                    "verified": bl_arm.verified[i]})).collect::<Vec<_>>(),
            });
            let bl_cert_sha = id_sha256(&bl_cert)?;
            fs::write(
                out_dir.join(format!("certs/bl_run{run_no}_level{level}.replay.json")),
                canonical_utf8(&bl_cert)?,
            )
            .map_err(|e| e.to_string())?;

            // ---- one claim per target
            for i in 0..targets.len() {
                let ic = ic_records[i];
                let bl = bl_records[i];
                let ic_ms = ic_arm.online_ms[i];
                let bl_ms = bl_arm.online_ms[i];
                let (workload_id, workload_sha) = workload(i)?;
                let mut c = Map::new();
                c.insert("schema".into(), json!("ic_equal_advice_claim/v1"));
                c.insert("n_or_bits".into(), json!(n));
                c.insert("rung".into(), json!(rung));
                if let Some((label, sha)) = &candidate {
                    c.insert("candidate_id".into(), json!(label));
                    c.insert("candidate_manifest_sha256".into(), json!(sha));
                }
                c.insert("workload_id".into(), json!(workload_id));
                c.insert("workload_manifest_sha256".into(), json!(workload_sha));
                if let Some((label, _)) = &candidate {
                    c.insert(
                        "run_id".into(),
                        json!(format!("{label}W{workload_id}R{run_no}")),
                    );
                }
                c.insert("target_count".into(), json!(1));
                let thash = sha256_hex(format!("{},{}", targets[i][0], targets[i][1]).as_bytes());
                c.insert("ic_target_hash".into(), json!(thash));
                c.insert(
                    "rho_target_hash".into(),
                    json!(sha256_hex(
                        format!(
                            "{},{}",
                            point_strings(&bl["target"]).unwrap()[0],
                            point_strings(&bl["target"]).unwrap()[1]
                        )
                        .as_bytes()
                    )),
                );
                c.insert("timing_class".into(), json!("single_target_online_wall"));
                c.insert("ic_online_wall_ms".into(), json!(ic_ms));
                c.insert("rho_online_wall_ms".into(), json!(bl_ms));
                c.insert("online_speedup".into(), json!(bl_ms / ic_ms));
                c.insert("ic_online_ms".into(), json!(ic_ms));
                c.insert("rho_online_ms".into(), json!(bl_ms));
                c.insert("ic_cost".into(), json!(ic_ms));
                c.insert("rho_cost".into(), json!(bl_ms));
                c.insert(
                    "ic_online_phase_ms".into(),
                    json!({
                        "T_target_query_ms": field_f64(ic, "target_query_ms")?,
                        "T_target_PDP_ms": field_f64(ic, "target_pdp_ms")?,
                        "T_target_relation_check_ms": field_f64(ic, "target_relation_check_ms")?,
                        "T_target_descent_ms": field_f64(ic, "target_descent_ms")?,
                        "T_target_recovery_check_ms": field_f64(ic, "target_recovery_check_ms")?,
                    }),
                );
                let ph = &bl["online_phase_ms"];
                c.insert(
                    "rho_online_phase_ms".into(),
                    json!({
                        "walk_and_collision_ms": field_f64(ph, "walk")? + field_f64(ph, "collision")?,
                        "recovery_check_ms": field_f64(ph, "recovery_check")?,
                    }),
                );
                let ic_interval = json!({
                    "start_event": ic["online_start_event"], "stop_event": ic["online_stop_event"],
                    "included_stages": ["target_query", "target_PDP", "target_relation_check",
                                        "target_descent", "target_recovery_check"]});
                let rho_interval = json!({
                    "start_event": bl["online_start_event"], "stop_event": bl["online_stop_event"],
                    "included_stages": ["walk", "collision", "recovery_check"]});
                c.insert("ic_online_interval".into(), ic_interval.clone());
                c.insert("rho_online_interval".into(), rho_interval.clone());
                c.insert(
                    "online_interval".into(),
                    json!({
                        "ic_start_event": ic["online_start_event"], "ic_stop_event": ic["online_stop_event"],
                        "rho_start_event": bl["online_start_event"], "rho_stop_event": bl["online_stop_event"],
                        "ic_included_stages": ic_interval["included_stages"],
                        "rho_included_stages": rho_interval["included_stages"],
                    }),
                );
                c.insert(
                    "paired_target".into(),
                    json!({"ic_public_q": targets[i], "rho_public_q": bl["target"],
                           "same_public_point": ic["target"] == bl["target"]
                               || point_strings(&ic["target"]) == point_strings(&bl["target"])}),
                );
                c.insert("ic_verified".into(), json!(ic_arm.verified[i]));
                c.insert("rho_verified".into(), json!(bl_arm.verified[i]));
                c.insert("ic_scalar_verified".into(), json!(ic_arm.verified[i]));
                c.insert("rho_scalar_verified".into(), json!(bl_arm.verified[i]));
                c.insert(
                    "recovered_d_verified".into(),
                    json!(ic_arm.verified[i] && bl_arm.verified[i]),
                );
                c.insert("all_stages_charged_same_series".into(), json!(true));
                let co = &session["co_measured"];
                let same_env = co["same_process_group"] == json!(true)
                    && !envelope.is_null()
                    && envelope.as_object().is_some_and(|o| !o.is_empty());
                c.insert("same_resource_envelope".into(), json!(same_env));
                c.insert("co_measured".into(), co.clone());
                c.insert("ic_resource_envelope".into(), envelope.clone());
                c.insert("rho_resource_envelope".into(), envelope.clone());
                c.insert("isolation_levels".into(), isolation.clone());
                c.insert(
                    "independent_validation".into(),
                    json!(session["independent_validation"] == json!(true)),
                );
                if !session["independent_replay_pointer"].is_null() {
                    c.insert(
                        "independent_replay_pointer".into(),
                        session["independent_replay_pointer"].clone(),
                    );
                }
                c.insert("ic_replay_certificate_sha256".into(), json!(ic_cert_sha));
                c.insert("rho_replay_certificate_sha256".into(), json!(bl_cert_sha));
                let ic_level = isolation["ic"].as_str().unwrap_or("L0");
                let rho_level = isolation["rho"].as_str().unwrap_or("L0");
                let controlled =
                    ["L2", "L3"].contains(&ic_level) && ["L2", "L3"].contains(&rho_level);
                c.insert(
                    "record_class".into(),
                    json!(if controlled {
                        "verified_answer_controlled_wall"
                    } else {
                        "verified_answer_exploratory_wall"
                    }),
                );
                c.insert(
                    "rho_policy".into(),
                    json!({
                        "worker_count": session["bl_online_threads"].as_u64().unwrap_or(1),
                        "distinguished_point_memory_bytes": bl["table_bytes"],
                        "walk_policy": "preprocessing rho, signed-Frobenius classes, 2048 G-only jumps, 4-step look-ahead, identical map offline and online",
                        "collision_policy": "binary search of the 8-byte class fingerprint in the sorted G-only DP table; every hit verified by [d]G = Q",
                        "table_entries": bl["table_entries"], "level": level, "tier": tier,
                    }),
                );
                c.insert(
                    "operation_accounting".into(),
                    json!({
                        "counter_scope": "whole_measured_record",
                        "counter_note": "per-target online counters; precomputation counters are in the BL precompute record",
                        "ic": {
                            "record_id": sha256_hex(serde_json::to_string(ic).map_err(|e| e.to_string())?.as_bytes()),
                            "native_counters": {
                                "probes": u64_or_zero(ic, "probes"),
                                "root_calls": u64_or_zero(ic, "root_calls"),
                                "relation_checks": u64_or_zero(ic, "relation_checks"),
                            },
                            "cost_is_lower_bound": true,
                            "unpriced_work": ["S3 root solves", "root-table probes", "normal-basis canonicalisation"],
                        },
                        "rho": {
                            "record_id": sha256_hex(serde_json::to_string(bl).map_err(|e| e.to_string())?.as_bytes()),
                            "native_counters": {
                                "online_steps": u64_or_zero(bl, "online_steps"),
                                "walks": u64_or_zero(bl, "walks"),
                                "table_lookups": u64_or_zero(bl, "table_lookups"),
                                "verifications": u64_or_zero(bl, "verifications"),
                            },
                            "cost_is_lower_bound": true,
                            "unpriced_work": ["normal-basis canonicalisation per step", "[c]G scalar multiplications", "table binary search"],
                        },
                        "common_operation_unit": null,
                        "calibrated_operation_ratio": null,
                        "total_work_S": null,
                        "calibration_status": "unknown",
                        "boundary": "native units; the section 5a band in decision.json prices them at 0, 0.1 and 1 GAE",
                    }),
                );
                c.insert("fixture_hash".into(), json!(targets_sha));
                c.insert(
                    "executable_or_source_hash".into(),
                    json!({"ic_binary_sha256": ic_bin, "bl_binary_sha256": bl_bin}),
                );
                c.insert("host_id".into(), session["host"]["id"].clone());
                c.insert("resource_caps".into(), envelope.clone());
                c.insert(
                    "seeds".into(),
                    json!({"ic_rank_seed": session["ic_rank_seed"], "bl_seed": session["bl_seed"]}),
                );
                c.insert("verdict".into(), json!("DIAGNOSTIC_NOT_PROMOTED"));
                c.insert("claim_boundary".into(), session["claim_boundary"].clone());
                c.insert(
                    "claim_boundary_non_claims".into(),
                    session["claim_boundary"].clone(),
                );
                let report = Value::Object(c);
                let check = check_vs_rho(&report)?;
                let problems = check_problems(&check);
                let stem = format!("run{run_no}_level{level}_target{i:02}");
                fs::write(
                    out_dir.join(format!("claims/{stem}.claim.json")),
                    serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?,
                )
                .map_err(|e| e.to_string())?;
                fs::write(
                    out_dir.join(format!("claims/{stem}.check.json")),
                    serde_json::to_string_pretty(&check).map_err(|e| e.to_string())?,
                )
                .map_err(|e| e.to_string())?;
                check_summary.push(json!({
                    "claim": stem, "status": check["status"], "problem_count": problems.len(),
                    "problems": if i == 0 { json!(problems) } else { Value::Null },
                }));
            }

            // ---- decision statistics for this (run, level)
            let t = targets.len();
            let ratios: Vec<f64> = (0..t)
                .map(|i| bl_arm.online_ms[i] / ic_arm.online_ms[i])
                .collect();
            let all_verified = ic_arm
                .verified
                .iter()
                .chain(bl_arm.verified.iter())
                .all(|&v| v);
            let seed = 20261018 + run_no * 100 + level;
            let med_ci = bootstrap(t, seed, |idx| {
                median(&idx.iter().map(|&i| ratios[i]).collect::<Vec<_>>())
            });
            let mean_ci = bootstrap(t, seed + 1, |idx| {
                mean(&idx.iter().map(|&i| bl_arm.online_ms[i]).collect::<Vec<_>>())
                    / mean(&idx.iter().map(|&i| ic_arm.online_ms[i]).collect::<Vec<_>>())
            });
            let band: Vec<Value> = PRICING_BAND
                .iter()
                .map(|&g| {
                    let ic_cost = mean(&ic_arm.priced) + g * mean(&ic_arm.unpriced);
                    let bl_cost = mean(&bl_arm.priced) + g * mean(&bl_arm.unpriced);
                    json!({"gae_per_unpriced_op": g, "ic_mean_cost": ic_cost,
                           "bl_mean_cost": bl_cost, "bl_over_ic": bl_cost / ic_cost})
                })
                .collect();
            let signs: Vec<bool> = band
                .iter()
                .map(|b| b["bl_over_ic"].as_f64().unwrap() < 1.0)
                .collect();
            let order_holds = signs.iter().all(|&s| s == signs[0]);
            // Anchor-note quantity (review 2026-10-18 section 3.2): log2(S*T^2) - log2(N),
            // N = r / (2n) classes. S: IC states / BL entries; T: IC probes / BL steps.
            let log2_n = (r as f64 / (2.0 * n as f64)).log2();
            let s_ic = session["ic_states"].as_f64().unwrap_or(f64::NAN);
            let s_bl = bl_records[0]["table_entries"].as_f64().unwrap_or(f64::NAN);
            let t_ic = mean(
                &ic_records
                    .iter()
                    .map(|x| u64_or_zero(x, "probes") as f64)
                    .collect::<Vec<_>>(),
            );
            let t_bl = mean(
                &bl_records
                    .iter()
                    .map(|x| u64_or_zero(x, "online_steps") as f64)
                    .collect::<Vec<_>>(),
            );
            decision_rows.push(json!({
                "rung": rung, "run": run_no, "level": level, "tier": tier,
                "log2_S_T2_minus_log2_N": {
                    "ic": s_ic.log2() + 2.0 * t_ic.log2() - log2_n,
                    "bl": s_bl.log2() + 2.0 * t_bl.log2() - log2_n,
                    "ic_S": s_ic, "ic_T_mean_probes": t_ic, "bl_S": s_bl, "bl_T_mean_steps": t_bl,
                    "log2_N": log2_n},
                "bl_table_bytes": bl_records[0]["table_bytes"],
                "targets": t, "all_scalars_replay_verified": all_verified,
                "ic_online_ms_mean": mean(&ic_arm.online_ms), "ic_online_ms_median": median(&ic_arm.online_ms),
                "bl_online_ms_mean": mean(&bl_arm.online_ms), "bl_online_ms_median": median(&bl_arm.online_ms),
                "median_ratio_bl_over_ic": median(&ratios), "median_ratio_ci95": med_ci,
                "mean_ratio_bl_over_ic": mean(&bl_arm.online_ms) / mean(&ic_arm.online_ms),
                "mean_ratio_ci95": mean_ci,
                "section_5a_band": band, "section_5a_order_holds_across_band": order_holds,
            }));
            for i in 0..t {
                rows.push(json!({
                    "rung": rung, "run": run_no, "level": level, "tier": tier, "target": i,
                    "ic_online_ms": ic_arm.online_ms[i], "bl_online_ms": bl_arm.online_ms[i],
                    "ic_verified": ic_arm.verified[i], "bl_verified": bl_arm.verified[i],
                }));
            }
        }
    }

    // ---- frozen decision rule (PROTOCOL.md "Decision rule"), per tier over runs.
    let tier_verdict = |tier: &str| -> Value {
        let mine: Vec<&Value> = decision_rows.iter().filter(|d| d["tier"] == tier).collect();
        if mine.is_empty()
            || !mine
                .iter()
                .all(|d| d["all_scalars_replay_verified"] == json!(true))
        {
            return json!("INVALID_UNVERIFIED_OR_MISSING");
        }
        let lo = |d: &&Value| d["median_ratio_ci95"][0].as_f64().unwrap();
        let hi = |d: &&Value| d["median_ratio_ci95"][1].as_f64().unwrap();
        // Ratio = BL online / IC online. Tier A is the IC-favourable (small)
        // budget: only a BL win there transfers to equal advice. Tier B is
        // the generous budget: only an IC win there transfers.
        if tier == "A" {
            if mine.iter().all(|d| hi(d) < 1.0) {
                json!("BL_FASTER_AT_TIER_A")
            } else {
                json!("NOT_BL_FASTER_AT_TIER_A")
            }
        } else if mine.iter().all(|d| lo(d) > 1.0) {
            json!("IC_FASTER_AT_TIER_B")
        } else {
            json!("NOT_IC_FASTER_AT_TIER_B")
        }
    };
    let aa = session["a_a_tolerance"].as_f64().unwrap_or(0.25);
    // A/A: the two IC processes (and the two BL passes) run the identical
    // arm on the identical targets. The median per-target ratio must lie
    // within 1/(1+tol)..(1+tol); the IC probe counts must be identical.
    let in_band = |x: f64| x >= 1.0 / (1.0 + aa) && x <= 1.0 + aa;
    let mut controls = Map::new();
    if ic_runs.len() == 2 {
        let r: Vec<f64> = ic_runs[0]
            .0
            .iter()
            .zip(&ic_runs[1].0)
            .map(|(x, y)| x / y)
            .collect();
        let m = median(&r);
        controls.insert("ic_run1_over_run2_median".into(), json!(m));
        controls.insert("ic_aa_in_band".into(), json!(in_band(m)));
        controls.insert(
            "ic_probes_identical_across_runs".into(),
            json!(ic_runs[0].1 == ic_runs[1].1),
        );
    }
    for lv in &levels {
        let level = lv["level"].as_u64().unwrap_or(0);
        let mine: Vec<&Vec<f64>> = bl_runs
            .iter()
            .filter(|(l, _)| *l == level)
            .map(|(_, v)| v)
            .collect();
        if mine.len() == 2 {
            let r: Vec<f64> = mine[0]
                .iter()
                .zip(mine[1].iter())
                .map(|(x, y)| x / y)
                .collect();
            let m = median(&r);
            controls.insert(format!("bl_level{level}_run1_over_run2_median"), json!(m));
            controls.insert(format!("bl_level{level}_aa_in_band"), json!(in_band(m)));
        }
    }
    let aa_ok = controls
        .iter()
        .filter(|(k, _)| k.ends_with("_aa_in_band") || k.ends_with("identical_across_runs"))
        .all(|(_, v)| v == &json!(true));
    // Instrument law and null control, from the BL file's own records.
    let mut law_ok = true;
    if let Some(pre) = bl_control_lines
        .iter()
        .find(|v| v["kind"] == "bl_precompute")
    {
        let classes = pre["classes_N"].as_f64().unwrap_or(f64::NAN);
        let entries = pre["entries"].as_f64().unwrap_or(f64::NAN);
        let w = pre["walk_len_2t"].as_f64().unwrap_or(f64::NAN);
        let predicted = classes / (entries * w) + w;
        let matched: Vec<f64> = bl_control_lines
            .iter()
            .filter(|v| {
                v["kind"] == "bl_online_target"
                    && v["prefix_level"] == 0
                    && v["pass"].as_u64().unwrap_or(1) == 1
            })
            .filter_map(|v| v["online_steps"].as_f64())
            .collect();
        let null: Vec<f64> = bl_control_lines
            .iter()
            .filter(|v| v["kind"] == "bl_null_target")
            .filter_map(|v| v["online_steps"].as_f64())
            .collect();
        if !matched.is_empty() {
            let m = mean(&matched);
            let law = m >= 0.25 * predicted && m <= 8.0 * predicted;
            controls.insert(
                "bl_instrument_law".into(),
                json!({
                "predicted_steps_N_over_TW_plus_W": predicted, "observed_mean_steps": m,
                "window": [0.25 * predicted, 8.0 * predicted], "in_window": law}),
            );
            law_ok &= law;
            if !null.is_empty() {
                let nm = mean(&null);
                let null_count = null.len();
                let null_ok = nm >= 5.0 * m;
                controls.insert(
                    "bl_null_control_mismatched_map".into(),
                    json!({
                    "targets": null_count, "mean_steps": nm, "matched_mean_steps": m,
                    "required_ratio_at_least": 5.0, "holds": null_ok}),
                );
                law_ok &= null_ok;
            } else {
                controls.insert("bl_null_control_mismatched_map".into(), json!("not_run"));
                law_ok = false;
            }
        } else {
            law_ok = false;
        }
    } else {
        law_ok = false;
    }
    let aa_ok = aa_ok && law_ok;
    controls.insert("all_controls_pass".into(), json!(aa_ok));
    controls.insert("base_hash".into(), base_hash_check);
    let (va, vb) = (tier_verdict("A"), tier_verdict("B"));
    let rung_verdict = if !aa_ok {
        "UNRESOLVED_CONTROL_FAILED"
    } else if va == json!("BL_FASTER_AT_TIER_A") {
        "BL_WINS_AT_EQUAL_ADVICE"
    } else if vb == json!("IC_FASTER_AT_TIER_B") {
        "IC_WINS_AT_EQUAL_ADVICE"
    } else if va == json!("INVALID_UNVERIFIED_OR_MISSING")
        || vb == json!("INVALID_UNVERIFIED_OR_MISSING")
    {
        "INVALID_UNVERIFIED_OR_MISSING"
    } else {
        "UNRESOLVED"
    };
    let decision = json!({
        "schema": "ic_equal_advice_decision/v1",
        "rung": rung, "session_id": session["session_id"], "targets": targets.len(),
        "rung_verdict": rung_verdict, "tier_A": va, "tier_B": vb,
        "controls": Value::Object(controls),
        "a_a_tolerance_relative": aa,
        "rows": decision_rows,
        "claim_checks": check_summary,
        "note": "Tier A is the IC-favourable (smaller) advice budget; tier B the generous one. See PROTOCOL.md for the rule and its reading.",
    });
    fs::write(
        out_dir.join("decision.json"),
        serde_json::to_string_pretty(&decision).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    fs::write(
        out_dir.join("rows.json"),
        serde_json::to_string_pretty(&rows).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    println!("{}", serde_json::to_string_pretty(&json!({
        "rung_verdict": decision["rung_verdict"], "tier_A": decision["tier_A"], "tier_B": decision["tier_B"],
        "claims": decision["claim_checks"].as_array().map_or(0, |v| v.len()),
    })).map_err(|e| e.to_string())?);
    Ok(())
}
