//! Native independent replay and Tier 1 admission for one N83 bridge stratum.
//! Usage: n83_bridge_analyze <stratum 0..3> <fixture-dir> <verify-binary> <results-dir>

use std::fs;
use std::path::Path;
use std::process::Command;

use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use crypto_lib::hash::sha256;
use num_bigint::BigUint;
use serde_json::{json, Value};

const K_VALUES: [usize; 2] = [1107, 2304];
const PAIR_RHO_SEED_BASE: u64 = 2026111400;
const RSS_CAP_BYTES: u64 = 61 * 1024 * 1024 * 1024 / 4;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn fail(message: impl Into<String>) -> Box<dyn std::error::Error> {
    message.into().into()
}
fn read(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
fn sha(path: &Path) -> Result<String> {
    Ok(hex::encode(sha256(&fs::read(path)?)))
}
fn lines(path: &Path) -> Result<Vec<Value>> {
    Ok(fs::read_to_string(path)?
        .lines()
        .map(serde_json::from_str)
        .collect::<std::result::Result<Vec<_>, _>>()?)
}
fn value_text(value: &Value) -> Result<String> {
    if let Some(text) = value.as_str() {
        return Ok(text.into());
    }
    if let Some(number) = value.as_u64() {
        return Ok(number.to_string());
    }
    Err(fail("expected decimal scalar"))
}
fn point(value: &Value) -> Result<BinaryPoint> {
    let coords = value
        .as_array()
        .ok_or_else(|| fail("point is not an array"))?;
    if coords.len() != 2 {
        return Err(fail("point length differs"));
    }
    let mut words = Vec::new();
    for coordinate in coords {
        let word = BigUint::parse_bytes(value_text(coordinate)?.as_bytes(), 10)
            .ok_or_else(|| fail("invalid coordinate"))?;
        if word >= (BigUint::from(1u8) << 83) {
            return Err(fail("field word exceeds N83"));
        }
        words.push(word);
    }
    Ok(BinaryPoint::Affine {
        x: F2mElement::from_biguint(&words[0], 83),
        y: F2mElement::from_biguint(&words[1], 83),
    })
}
fn scalar_replay(curve: &KoblitzCurve, point_value: &Value, scalar: &str) -> Result<()> {
    let value =
        BigUint::parse_bytes(scalar.as_bytes(), 10).ok_or_else(|| fail("invalid scalar"))?;
    let target = point(point_value)?;
    if value == BigUint::from(0u8)
        || value >= curve.subgroup_order
        || curve.mul(curve.generator(), &value) != target
        || curve.mul(&target, &curve.subgroup_order) != BinaryPoint::Infinity
    {
        return Err(fail("independent scalar or subgroup replay failed"));
    }
    Ok(())
}
fn usage(folder: &Path) -> Result<Value> {
    let value = read(&folder.join("usage.json"))?;
    if value["schema"] != "n83.native-bridge-child-usage/v1"
        || value["status"] != "PASS_PRODUCER"
        || value["exit_code"] != 0
        || value["rss_cap_bytes"] != RSS_CAP_BYTES
        || value["peak_rss_bytes"]
            .as_u64()
            .ok_or_else(|| fail("peak RSS missing"))?
            > RSS_CAP_BYTES
        || value["wall_ms"].as_f64().unwrap_or(0.0) <= 0.0
    {
        return Err(fail(format!(
            "producer or resource failure: {}",
            folder.display()
        )));
    }
    if steal_jiffies(&value["system_before"])? != steal_jiffies(&value["system_after"])? {
        return Err(fail(format!("steal time changed: {}", folder.display())));
    }
    if cgroup_counter(&value["system_after"], "cpu_stat", "nr_throttled")?
        > cgroup_counter(&value["system_before"], "cpu_stat", "nr_throttled")?
    {
        return Err(fail(format!("cgroup CPU throttling: {}", folder.display())));
    }
    for key in ["oom", "oom_kill"] {
        if cgroup_counter(&value["system_after"], "memory_events", key)?
            > cgroup_counter(&value["system_before"], "memory_events", key)?
        {
            return Err(fail(format!("cgroup {key} event: {}", folder.display())));
        }
    }
    Ok(value)
}

fn cgroup_counter(system: &Value, field: &str, key: &str) -> Result<u64> {
    let events = system[field]
        .as_str()
        .ok_or_else(|| fail(format!("cgroup {field} absent")))?;
    let line = events
        .lines()
        .find(|line| line.starts_with(&format!("{key} ")))
        .ok_or_else(|| fail(format!("cgroup {key} counter absent")))?;
    Ok(line
        .split_whitespace()
        .nth(1)
        .ok_or("counter value absent")?
        .parse()?)
}
fn replay_rho(
    curve: &KoblitzCurve,
    folder: &Path,
    expected_point: &Value,
    expected_scalar: &str,
    expected_seed: u64,
    target_path: &Path,
) -> Result<Value> {
    let cost = usage(folder)?;
    let receipt = read(&folder.join("receipt.json"))?;
    let recovered = value_text(&receipt["recovered_scalar"])?;
    let online = receipt["online_ms"]
        .as_f64()
        .ok_or_else(|| fail("rho online missing"))?;
    if receipt["status"] != "PASS_VERIFIED_PUBLIC_TARGET"
        || receipt["public_target_q"] != *expected_point
        || recovered != expected_scalar
        || receipt["target_count"] != 1
        || receipt["known_answer_input"] != false
        || receipt["validation_sidecar_opened"] != false
        || receipt["group_verified"] != true
        || receipt["generic_scalar_replay_verified"] != true
        || receipt["walk_seed"] != expected_seed
        || receipt["public_input_sha256"] != sha(target_path)?
        || !online.is_finite()
        || online <= 0.0
    {
        return Err(fail(format!(
            "same-point rho replay failed: {}",
            folder.display()
        )));
    }
    scalar_replay(curve, expected_point, &recovered)?;
    Ok(
        json!({"status":"PASS_INDEPENDENT_REPLAY","online_ms":online,
              "child_wall_ms":cost["wall_ms"],"peak_rss_bytes":cost["peak_rss_bytes"]}),
    )
}
fn median_three(mut values: Vec<f64>) -> Result<f64> {
    if values.len() != 3 || values.iter().any(|v| !v.is_finite() || *v <= 0.0) {
        return Err(fail("three positive calibration times required"));
    }
    values.sort_by(f64::total_cmp);
    Ok(values[1])
}
fn steal_jiffies(system: &Value) -> Result<u64> {
    let stat = system["proc_stat"]
        .as_str()
        .ok_or_else(|| fail("proc stat missing"))?;
    let first = stat.lines().next().ok_or_else(|| fail("proc stat empty"))?;
    let fields: Vec<&str> = first.split_whitespace().collect();
    if fields.len() < 9 || fields[0] != "cpu" {
        return Err(fail("proc stat cpu differs"));
    }
    Ok(fields[8].parse()?)
}
fn affinity(system: &Value) -> Result<String> {
    let status = system["self_status"]
        .as_str()
        .ok_or_else(|| fail("self status missing"))?;
    let line = status
        .lines()
        .find(|line| line.starts_with("Cpus_allowed_list:"))
        .ok_or_else(|| fail("visible CPU affinity missing"))?;
    Ok(line.trim().into())
}
fn verify(stratum: usize, fixture: &Path, verifier: &Path, results: &Path) -> Result<Value> {
    if stratum >= 4 {
        return Err(fail("stratum must be 0..3"));
    }
    let protocol = read(&fixture.join("protocol.json"))?;
    let targets_path = fixture.join("targets.jsonl");
    let answers_path = fixture.join("answers.jsonl");
    let workloads_path = fixture.join("workloads.json");
    let targets = lines(&targets_path)?;
    let answers = lines(&answers_path)?;
    let workloads = read(&workloads_path)?;
    if protocol["schema"] != "n83.native-bridge-protocol/v1"
        || protocol["targets_sha256"] != sha(&targets_path)?
        || protocol["answers_sha256"] != sha(&answers_path)?
        || protocol["workloads_sha256"] != sha(&workloads_path)?
        || targets.len() != 32
        || answers.len() != 32
        || workloads["schema"] != "n83.native-bridge-workloads/v1"
        || workloads["rows"].as_array().map(|rows| rows.len()) != Some(32)
    {
        return Err(fail("frozen N83 fixture differs"));
    }
    let source = read(&results.join("source.json"))?;
    let host = read(&results.join("host.json"))?;
    let preflight = read(&results.join("preflight.json"))?;
    let after = read(&results.join("host_after.json"))?;
    if source["schema"] != "n83.native-bridge-source/v1"
        || source["stratum"] != stratum
        || source["source_sha256"] != protocol["source_sha256"]
        || source["fixture_sha256"] != protocol["targets_sha256"]
        || source["ic_binary_sha256"] != sha(&results.join("executed_bins/koblitz_orbit_dlp_fast"))?
        || source["rho_binary_sha256"]
            != sha(&results.join("executed_bins/n83_rho_optimization_eval"))?
        || host["schema"] != "n83.native-bridge-host/v1"
        || host["stratum"] != stratum
        || preflight["status"] != "PASS_CAPACITY_PREFLIGHT"
        || preflight["visible_cpus"].as_u64().unwrap_or(0) < 4
        || host["lscpu_json"].as_str().unwrap_or("").is_empty()
        || results.join("producer_stop.json").exists()
    {
        return Err(fail("source, host, binary or coverage differs"));
    }
    let events = read(&results.join("events.json"))?;
    let events = events.as_array().ok_or("event order missing")?;
    if events.len() != 8 * 2 * 8 {
        return Err(fail("128 scheduled producer events required"));
    }
    let mut cursor = 0;
    for index in stratum * 8..(stratum + 1) * 8 {
        for order in 0..2 {
            let fixed = (index + order) % 2;
            let k = K_VALUES[fixed];
            let kinds = [
                "cal_before_0",
                "cal_before_1",
                "cal_before_2",
                if (index + fixed) % 2 == 0 {
                    "ic"
                } else {
                    "rho"
                },
                if (index + fixed) % 2 == 0 {
                    "rho"
                } else {
                    "ic"
                },
                "cal_after_0",
                "cal_after_1",
                "cal_after_2",
            ];
            for kind in kinds {
                let event = &events[cursor];
                if event["point_index"] != index
                    || event["k"] != k
                    || event["kind"] != kind
                    || event["name"] != format!("{kind}_k{k}root11")
                    || event["usage"] != "PASS_PRODUCER"
                {
                    return Err(fail(format!("producer event {cursor} differs")));
                }
                cursor += 1;
            }
        }
    }
    let cpu_set = affinity(&host["system_before"])?;
    let cpu_max = host["system_before"]["cpu_max"]
        .as_str()
        .ok_or("frozen cgroup CPU limit absent")?;
    let memory_max = host["system_before"]["memory_max"]
        .as_str()
        .ok_or("frozen cgroup memory limit absent")?;
    for event in events {
        let index = event["point_index"].as_u64().ok_or("event point absent")?;
        let name = event["name"].as_str().ok_or("event name absent")?;
        let event_folder = results
            .join("points")
            .join(format!("p{index:02}"))
            .join("runs")
            .join(name);
        let event_usage = read(&event_folder.join("usage.json"))?;
        for boundary in ["system_before", "system_after"] {
            let receipt = &event_usage[boundary];
            if affinity(receipt)? != cpu_set
                || receipt["cpu_max"].as_str() != Some(cpu_max)
                || receipt["memory_max"].as_str() != Some(memory_max)
            {
                return Err(fail(format!(
                    "frozen host envelope changed: {} {boundary}",
                    event_folder.display()
                )));
            }
        }
    }
    let before_steal = steal_jiffies(&host["system_before"])?;
    let after_steal = steal_jiffies(&after["system_after"])?;
    let curve = KoblitzCurve::new(1, 83).ok_or_else(|| fail("N83 curve unavailable"))?;
    let cal_path = Path::new("research/n83_cold_factor_base_20261016/targets.jsonl");
    let cal_point = lines(cal_path)?.remove(0);
    let cal_answer = lines(Path::new(
        "research/n83_cold_factor_base_20261016/answers.jsonl",
    ))?
    .remove(0);
    let cal_scalar = value_text(&cal_answer["scalar"])?;
    scalar_replay(&curve, &cal_point, &cal_scalar)?;
    let mut rows = Vec::new();
    for index in stratum * 8..(stratum + 1) * 8 {
        let answer = &answers[index];
        let scalar = value_text(&answer["scalar"])?;
        if answer["fixture_index"] != index || answer["point"] != targets[index] {
            return Err(fail(format!("fixture sidecar {index} differs")));
        }
        scalar_replay(&curve, &targets[index], &scalar)?;
        let point_dir = results.join("points").join(format!("p{index:02}"));
        let point_path = point_dir.join("target.jsonl");
        if lines(&point_path)? != vec![targets[index].clone()] {
            return Err(fail(format!("public target file {index} differs")));
        }
        for (fixed, k) in K_VALUES.iter().enumerate() {
            let label = format!("k{k}root11");
            let runs = point_dir.join("runs");
            let ic_folder = runs.join(format!("ic_{label}"));
            let ic_usage = usage(&ic_folder)?;
            let replay = Command::new(verifier)
                .args([
                    k.to_string(),
                    point_path.display().to_string(),
                    scalar.clone(),
                    ic_folder.display().to_string(),
                ])
                .output()?;
            if !replay.status.success() {
                return Err(fail(format!(
                    "p{index:02} K{k} IC replay: {}",
                    String::from_utf8_lossy(&replay.stderr)
                )));
            }
            let ic: Value = serde_json::from_slice(&replay.stdout)?;
            if ic["status"] != "PASS_INDEPENDENT_REPLAY" || ic["k"] != *k {
                return Err(fail("IC verifier status differs"));
            }
            let seed = PAIR_RHO_SEED_BASE + 2 * index as u64 + fixed as u64;
            let rho = replay_rho(
                &curve,
                &runs.join(format!("rho_{label}")),
                &targets[index],
                &scalar,
                seed,
                &point_path,
            )?;
            let mut before = Vec::new();
            let mut after = Vec::new();
            for repeat in 0..3 {
                let name = format!("cal_before_{repeat}_{label}");
                before.push(
                    replay_rho(
                        &curve,
                        &runs.join(name),
                        &cal_point,
                        &cal_scalar,
                        2026101601,
                        cal_path,
                    )?["online_ms"]
                        .as_f64()
                        .ok_or_else(|| fail("calibration online missing"))?,
                );
                let name = format!("cal_after_{repeat}_{label}");
                after.push(
                    replay_rho(
                        &curve,
                        &runs.join(name),
                        &cal_point,
                        &cal_scalar,
                        2026101601,
                        cal_path,
                    )?["online_ms"]
                        .as_f64()
                        .ok_or_else(|| fail("calibration online missing"))?,
                );
            }
            let medians = (median_three(before.clone())?, median_three(after.clone())?);
            let spread = (medians.0 - medians.1).abs() / ((medians.0 + medians.1) / 2.0);
            let workload = &workloads["rows"][index];
            let workload_id = workload["workload_id"]
                .as_str()
                .ok_or_else(|| fail("workload ID missing"))?;
            if workload_id.len() != 12
                || workload["canonical_record"]["public_target_q"] != targets[index]
            {
                return Err(fail(format!("workload {index} differs")));
            }
            let candidate_id = protocol[&format!("candidate_k{k}")]
                .as_str()
                .ok_or_else(|| fail("candidate ID absent"))?;
            rows.push(json!({"point_index":index,"k":k,
                "candidate_id":candidate_id,
                "workload_id":workload_id,
                "run_id":format!("{candidate_id}W{workload_id}R1"),
                "target":targets[index],"ic":ic,"rho":rho,
                "ic_child_wall_ms":ic_usage["wall_ms"],
                "calibration_before_ms":before,"calibration_after_ms":after,
                "calibration_medians_ms":[medians.0,medians.1],
                "calibration_relative_spread":spread,
                "calibration_status":if spread <= 0.10 {"PASS_10_PERCENT"} else {"NOISY_10_PERCENT"},
            }));
        }
    }
    let calibration_pass = rows
        .iter()
        .all(|row| row["calibration_status"] == "PASS_10_PERCENT");
    let tier = if rows.len() == 16 && calibration_pass && after_steal == before_steal {
        "PASS_TIER1"
    } else {
        "BOUNDED_UNKNOWN"
    };
    Ok(json!({"schema":"n83.native-bridge-stratum-analysis/v1",
              "status":tier,"stratum":stratum,"point_indices":(stratum*8..stratum*8+8).collect::<Vec<_>>(),
              "source_sha256":protocol["source_sha256"],
              "targets_sha256":protocol["targets_sha256"],
              "host":host,"affinity":cpu_set,"steal_jiffies_before":before_steal,
              "steal_jiffies_after":after_steal,"rows":rows}))
}
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        return Err(fail(
            "usage: n83_bridge_analyze <stratum> <fixture-dir> <verify-binary> <results-dir>",
        ));
    }
    let output = Path::new(&args[4]);
    let analysis = verify(
        args[1].parse()?,
        Path::new(&args[2]),
        Path::new(&args[3]),
        output,
    )?;
    let path = output.join("analysis.json");
    if path.exists() {
        return Err(fail("analysis already exists"));
    }
    fs::write(&path, serde_json::to_vec_pretty(&analysis)?)?;
    println!("{}", json!({"status":analysis["status"],"analysis":path}));
    Ok(())
}
