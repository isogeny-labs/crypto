//! Independent arithmetic and Tier 1 receipt replay for one frozen stratum.
use super::{hash, jsonl, point, Arm, Result, ARMS, CURVE_ID, FILES, ORDER, SOURCE_SHA256};
use crypto_lib::binary_ecc::BinaryPoint;
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

const RSS_CAP: u64 = 61 * 1024 * 1024 * 1024 / 4;
const CAL_SEED: u64 = 2_026_101_601;
const PAIR_SEED_BASE: u64 = 2_026_122_600;

fn read(root: &Path, name: &str) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(root.join(name))?)?)
}
fn checked_number(row: &Value, key: &str) -> Result<f64> {
    let number = row[key].as_f64().ok_or_else(|| format!("missing {key}"))?;
    if !number.is_finite() || number <= 0.0 {
        return Err(format!("invalid {key}").into());
    }
    Ok(number)
}
fn counter(snapshot: &Value, section: &str, key: &str) -> Result<u64> {
    let body = snapshot[section]
        .as_str()
        .ok_or_else(|| format!("missing {section}"))?;
    let row = body
        .lines()
        .find(|line| line.split_whitespace().next() == Some(key))
        .ok_or_else(|| format!("missing {section}:{key}"))?;
    Ok(row
        .split_whitespace()
        .nth(1)
        .ok_or("counter absent")?
        .parse()?)
}
fn median_three(values: &[f64]) -> Result<f64> {
    if values.len() != 3 {
        return Err("calibration repetitions differ".into());
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    Ok(sorted[1])
}
fn event_path(index: usize, name: &str) -> String {
    format!("points/p{index:02}/runs/{name}")
}
struct EventInputs<'a> {
    output: &'a Path,
    index: usize,
    name: &'a str,
    kind: &'a str,
    arm: Arm,
    fixed: usize,
    target: &'a BinaryPoint,
    scalar: u64,
    calibration: &'a BinaryPoint,
    calibration_scalar: u64,
    curve: &'a KoblitzCurve,
    manifest: &'a Value,
    cpu_max: &'a Value,
    memory_max: &'a Value,
}
fn receipt(inputs: EventInputs<'_>) -> Result<Value> {
    let EventInputs {
        output,
        index,
        name,
        kind,
        arm,
        fixed,
        target,
        scalar,
        calibration,
        calibration_scalar,
        curve,
        manifest,
        cpu_max,
        memory_max,
    } = inputs;
    let prefix = event_path(index, name);
    let launch = read(output, &format!("{prefix}/launch.json"))?;
    let usage = read(output, &format!("{prefix}/usage.json"))?;
    let telemetry = read(output, &format!("{prefix}/host_telemetry.json"))?;
    let command = launch["command"]
        .as_array()
        .ok_or("launch command absent")?;
    let cap = if kind == "ic" { 1200 } else { 120 };
    if launch["cap_seconds"] != cap
        || launch["rss_cap_bytes"] != RSS_CAP
        || usage["status"] != "PASS_PRODUCER"
        || usage["exit_status"] != 0
        || !usage["stop_reason"].is_null()
        || usage["peak_rss_bytes"].as_u64().unwrap_or(u64::MAX) > RSS_CAP
        || usage["sampled_peak_rss_bytes"].as_u64().unwrap_or(u64::MAX) > RSS_CAP
        || checked_number(&usage, "wall_seconds").is_err()
        || fs::read_to_string(output.join(format!("{prefix}/exit_status.txt")))?.trim() != "0"
        || launch["started_utc"].as_str().unwrap_or("")
            >= usage["finished_utc"].as_str().unwrap_or("")
    {
        return Err("child status, cap, clock or RSS differs".into());
    }
    for side in ["cgroup_before", "cgroup_after"] {
        if usage[side]["cpu.max"] != *cpu_max || usage[side]["memory.max"] != *memory_max {
            return Err(format!("cgroup envelope changed at {side}").into());
        }
    }
    for (section, key) in [
        ("cpu.stat", "nr_throttled"),
        ("cpu.stat", "throttled_usec"),
        ("memory.events", "oom"),
        ("memory.events", "oom_kill"),
    ] {
        if counter(&usage["cgroup_before"], section, key)?
            != counter(&usage["cgroup_after"], section, key)?
        {
            return Err(format!("resource noise {section}:{key}").into());
        }
    }
    let before_steal = telemetry["before"]["steal_jiffies"]
        .as_u64()
        .ok_or("steal before absent")?;
    let after_steal = telemetry["after"]["steal_jiffies"]
        .as_u64()
        .ok_or("steal after absent")?;
    if before_steal != after_steal {
        return Err("CPU steal observed".into());
    }
    let command_text = command
        .iter()
        .map(|v| v.as_str().unwrap_or(""))
        .collect::<Vec<_>>();
    let public = if kind == "calibration" {
        calibration
    } else {
        target
    };
    let expected_scalar = if kind == "calibration" {
        calibration_scalar
    } else {
        scalar
    };
    if kind == "ic" {
        if command_text.len() != 5
            || command_text[1] != format!("construct:83:1:{}", arm.k)
            || !command_text[2].ends_with(&format!("/points/p{index:02}/target.jsonl"))
            || command_text[3] != "29"
            || !command_text[4].ends_with(&format!("/{prefix}/targets_result.jsonl"))
            || !command_text[0].ends_with("/executed_bins/koblitz_orbit_dlp_fast")
            || launch["environment_flags"]["KIC_TARGET_THREADS"] != "2"
            || launch["environment_flags"]["KIC_TARGET_CANCEL"] != "1"
            || launch["environment_flags"]["KIC_INDEXED5_ID30"] != "0"
            || launch["environment_flags"]["KIC_INDEX_ROOT_BATCH"] != "1"
        {
            return Err("IC launch differs".into());
        }
        let dump_path = output.join(format!("{prefix}/base_dump.jsonl"));
        let bytes = fs::read(&dump_path)?;
        if hash(&bytes) != manifest["factor_base"]["ordered_base_dump_sha256"] {
            return Err("factor-base dump hash differs".into());
        }
        let dump: Value = serde_json::from_slice(&bytes)?;
        let base = dump["factor_base_point_coordinates"]
            .as_array()
            .ok_or("base absent")?;
        if base.len() != 166 * arm.k
            || dump["base_hash"] != manifest["factor_base"]["source_base_hash_blake3"]
        {
            return Err("factor-base identity differs".into());
        }
        let summary = read(output, &format!("{prefix}/stdout.jsonl"))?;
        let result = read(output, &format!("{prefix}/targets_result.jsonl"))?;
        if summary["n"] != 83
            || summary["a"] != 1
            || summary["orbit_columns"] != arm.k
            || summary["rank"] != arm.k
            || summary["rank_relations"]
                .as_u64()
                .is_none_or(|rows| rows < arm.k as u64)
            || summary["factor_base_points"] != 166 * arm.k
            || summary["base_hash"] != dump["base_hash"]
            || summary["root_table_indexed5_id30_enabled"] != false
            || summary["root_table_allocated_bytes"]
                != if arm.layout == "root11_wide3" {
                    5u64 << 30
                } else {
                    4u64 << 31
                }
            || summary["targets"] != 1
            || summary["targets_solved"] != 1
            || summary["targets_failed"] != 0
            || summary["target_threads"] != 2
            || summary["target_cancel"] != true
            || result["group_verified"] != true
            || result["fixture_index"] != 0
            || result["recovered_scalar"].as_u64() != Some(expected_scalar)
            || point(&result["target"])? != *public
            || point(&result["published_q"])? != *public
            || point(&result["generator"])? != *curve.generator()
        {
            return Err("IC scalar or rank replay differs".into());
        }
        let indices = result["point_indices"]
            .as_array()
            .ok_or("relation absent")?;
        if indices.len() != 4 {
            return Err("relation arity differs".into());
        }
        let mut sum = BinaryPoint::Infinity;
        for member in indices {
            let position = member.as_u64().ok_or("base index absent")? as usize;
            sum = curve.add(
                &sum,
                &point(base.get(position).ok_or("base index out of range")?)?,
            );
        }
        if sum != *public {
            return Err("relation point sum differs".into());
        }
        let online = checked_number(&result, "target_ms")?;
        let phase_sum = [
            "target_query_ms",
            "target_pdp_ms",
            "target_relation_check_ms",
            "target_descent_ms",
            "target_recovery_check_ms",
        ]
        .iter()
        .try_fold(0.0, |total, key| -> Result<f64> {
            let value = result[*key].as_f64().ok_or("phase absent")?;
            if !value.is_finite() || value < 0.0 {
                return Err("phase invalid".into());
            }
            Ok(total + value)
        })?;
        let cold = checked_number(&summary["timing_ms"], "process_total")?;
        if (phase_sum - online).abs() > 1.0 || cold < online {
            return Err("online phase or cold time differs".into());
        }
        Ok(
            json!({"status":"PASS_INDEPENDENT_REPLAY","online_ms":online,
            "cold_process_ms":cold,"base_hash":dump["base_hash"],"point_index":index,
            "candidate_id":arm.id,"kind":kind,"name":name,
            "peak_rss_bytes":usage["peak_rss_bytes"],
            "started_utc":launch["started_utc"],"finished_utc":usage["finished_utc"]}),
        )
    } else {
        let seed = if kind == "calibration" {
            CAL_SEED
        } else {
            PAIR_SEED_BASE + 4 * index as u64 + fixed as u64
        };
        if command_text.len() != 5
            || command_text[2] != seed.to_string()
            || command_text[3] != "research/n83_dense_root_20261013/rho_config.json"
                && !command_text[3].ends_with("/research/n83_dense_root_20261013/rho_config.json")
            || !command_text[4].ends_with(&format!("/{prefix}/receipt.json"))
            || !command_text[0].ends_with("/executed_bins/n83_rho_optimization_eval")
            || launch["environment_flags"] != json!({})
        {
            return Err("rho launch differs".into());
        }
        let expected_target = if kind == "calibration" {
            "/research/n83_cold_factor_base_20261016/targets.jsonl".to_owned()
        } else {
            format!("/points/p{index:02}/target.jsonl")
        };
        if !command_text[1].ends_with(&expected_target) {
            return Err("rho target path differs".into());
        }
        let result = read(output, &format!("{prefix}/receipt.json"))?;
        let summary = read(output, &format!("{prefix}/stdout.jsonl"))?;
        for row in [&result, &summary] {
            if row["status"] != "PASS_VERIFIED_PUBLIC_TARGET"
                || row["recovered_scalar"].as_str() != Some(expected_scalar.to_string().as_str())
                || point(&row["public_target_q"])? != *public
                || point(&row["published_q"])? != *public
                || point(&row["generator"])? != *curve.generator()
                || row["target_count"] != 1
                || row["target_on_curve"] != true
                || row["target_in_subgroup"] != true
            {
                return Err("same-point rho scalar replay differs".into());
            }
        }
        if result["walk_seed"] != seed
            || result["known_answer_input"] != false
            || result["validation_sidecar_opened"] != false
            || result["group_verified"] != true
            || result["generic_scalar_replay_verified"] != true
        {
            return Err("rho receipt verification flags differ".into());
        }
        let online = checked_number(&result, "online_ms")?;
        let phase = &result["online_phase_ns"];
        let phase_sum = [
            "target_query_and_validation",
            "walk_and_collision",
            "generic_scalar_replay",
        ]
        .iter()
        .try_fold(0u64, |total, key| -> Result<u64> {
            Ok(total
                .checked_add(phase[*key].as_u64().ok_or("rho phase absent")?)
                .ok_or("rho phase overflow")?)
        })?;
        let interval = result["online_interval_ns"]
            .as_u64()
            .ok_or("rho interval absent")?;
        if phase_sum != result["online_phase_sum_ns"]
            || phase_sum > interval
            || result["online_timer_boundary_residual_ns"] != interval - phase_sum
            || ((interval as f64 / 1_000_000.0) - online).abs() > 0.001
        {
            return Err("rho online phase accounting differs".into());
        }
        Ok(
            json!({"status":"PASS_INDEPENDENT_REPLAY","online_ms":online,
            "point_index":index,"candidate_id":arm.id,"kind":kind,"name":name,
            "peak_rss_bytes":usage["peak_rss_bytes"],
            "started_utc":launch["started_utc"],"finished_utc":usage["finished_utc"]}),
        )
    }
}

pub fn run(
    root: &Path,
    output: &Path,
    stratum: usize,
    events: &Value,
    analysis_output: &Path,
) -> Result<Value> {
    if stratum > 1 {
        return Err("stratum differs".into());
    }
    let here = root.join("research/n83_crossfamily_current_20261025");
    let targets = jsonl(&here.join("targets.jsonl"))?;
    let answers = jsonl(&here.join("answers.jsonl"))?;
    let host = read(output, "host.json")?;
    let source = read(output, "source.json")?;
    let preflight = read(output, "preflight.json")?;
    let allocation = read(output, "aws_allocation.json")?;
    let identity = &host["instance_identity"]["document"];
    if host["schema"] != "n83.crossfamily-current-host/v2"
        || source["schema"] != "n83.crossfamily-current-source/v2"
        || preflight["status"] != "PASS_CAPACITY_PREFLIGHT"
        || host["stratum"] != stratum
        || source["stratum"] != stratum
        || source["source_sha256"] != SOURCE_SHA256
        || source["choice"] != super::arms(root)?
        || source["events"] != *events
        || source["runner_sha256"]
            != hash(&fs::read(root.join(
                "research/n83_crossfamily_current_20261025/native/src/runner.rs",
            ))?)
        || source["aws_allocation_sha256"] != hash(&fs::read(output.join("aws_allocation.json"))?)
        || source["source_archive_sha256"] != allocation["source_archive_sha256"]
        || source["source_manifest_sha256"] != allocation["source_manifest_sha256"]
        || host["parent_instance_id"] != allocation["instance_id"]
        || host["aws_allocation_sha256"] != source["aws_allocation_sha256"]
        || host["source_archive_sha256"] != source["source_archive_sha256"]
        || host["source_manifest_sha256"] != source["source_manifest_sha256"]
        || host["tier"] != "cloud_instance_tier1"
        || host["git_sha"] != allocation["source_head_sha"]
        || identity["instanceId"] != allocation["instance_id"]
        || identity["accountId"] != allocation["account_id"]
        || identity["instanceType"] != allocation["instance_type"]
        || identity["availabilityZone"] != allocation["availability_zone"]
        || identity["imageId"] != allocation["image_id"]
        || host["instance_identity"]["instance_life_cycle"] != "spot"
        || host["topology"]["lscpu_json"]
            .as_str()
            .is_none_or(str::is_empty)
        || host["topology"]["visible_cpu_topology"]
            .as_object()
            .is_none_or(|cpus| cpus.len() < 4)
        || host["mem_total_kib"]
            .as_u64()
            .unwrap_or(0)
            .saturating_mul(1024)
            < RSS_CAP + 256 * 1024 * 1024
        || host["disk_free_bytes"].as_u64().unwrap_or(0) < 3 * 1024 * 1024 * 1024
        || host["visible_affinity"]
            .as_array()
            .is_none_or(|cpus| cpus.len() < 4)
    {
        return Err("stratum source, host, or completion receipt differs".into());
    }
    for (name, expected) in FILES {
        if source["hashes"][name] != expected {
            return Err(format!("sealed hash differs: {name}").into());
        }
    }
    for (name, key) in [
        ("koblitz_orbit_dlp_fast", "binary_sha256"),
        ("n83_rho_optimization_eval", "rho_binary_sha256"),
    ] {
        if source[key] != hash(&fs::read(output.join("executed_bins").join(name))?) {
            return Err(format!("binary differs: {name}").into());
        }
    }
    let curve = KoblitzCurve::new(1, 83).ok_or("N83 curve absent")?;
    if curve.subgroup_order != BigUint::from(ORDER) {
        return Err("subgroup differs".into());
    }
    let calibration_target = point(&serde_json::from_slice::<Value>(&fs::read(
        root.join("research/n83_cold_factor_base_20261016/targets.jsonl"),
    )?)?)?;
    let calibration_answer: Value = serde_json::from_slice(&fs::read(
        root.join("research/n83_cold_factor_base_20261016/answers.jsonl"),
    )?)?;
    let calibration_scalar = calibration_answer["scalar"]
        .as_str()
        .ok_or("calibration scalar absent")?
        .parse::<u64>()?;
    if curve.mul(curve.generator(), &BigUint::from(calibration_scalar)) != calibration_target {
        return Err("calibration scalar replay differs".into());
    }
    let cpu_max = &host["cgroup_before"]["cpu.max"];
    let memory_max = &host["cgroup_before"]["memory.max"];
    let mut manifests = BTreeMap::new();
    for arm in ARMS {
        let path = root
            .join("research")
            .join(arm.manifest_dir)
            .join(format!("candidate_{}.json", arm.id));
        manifests.insert(arm.name, serde_json::from_slice::<Value>(&fs::read(path)?)?);
    }
    let rows = events
        .as_array()
        .ok_or("events absent")?
        .iter()
        .map(|event| {
            let index = event["point_index"].as_u64().ok_or("point index absent")? as usize;
            let name = event["name"].as_str().ok_or("event name absent")?;
            let kind = event["kind"].as_str().ok_or("event kind absent")?;
            let candidate = event["candidate"].as_str().ok_or("candidate absent")?;
            let (fixed, arm) = ARMS
                .iter()
                .copied()
                .enumerate()
                .find(|(_, arm)| arm.name == candidate)
                .ok_or("arm absent")?;
            let scalar = answers[index]["scalar"]
                .as_str()
                .ok_or("scalar absent")?
                .parse::<u64>()?;
            let target = point(&targets[index])?;
            if curve.mul(curve.generator(), &BigUint::from(scalar)) != target
                || curve.mul(&target, &curve.subgroup_order) != BinaryPoint::Infinity
            {
                return Err("fixture arithmetic differs".into());
            }
            let record = (|| -> Result<Value> {
                let supplied = read(output, &format!("points/p{index:02}/target.jsonl"))?;
                if point(&supplied)? != target {
                    return Err("supplied public point differs".into());
                }
                receipt(EventInputs {
                    output,
                    index,
                    name,
                    kind,
                    arm,
                    fixed,
                    target: &target,
                    scalar,
                    calibration: &calibration_target,
                    calibration_scalar,
                    curve: &curve,
                    manifest: &manifests[candidate],
                    cpu_max,
                    memory_max,
                })
            })();
            Ok(match record {
                Ok(row) => row,
                Err(error) => json!({"status":"VERIFICATION_FAILED","point_index":index,
                "candidate_id":arm.id,"name":name,"kind":kind,"error":error.to_string()}),
            })
        })
        .collect::<Result<Vec<Value>>>()?;
    let mut calibration_status = true;
    for index in stratum * 8..stratum * 8 + 8 {
        for arm in ARMS {
            let timings = |before: bool| -> Vec<f64> {
                let prefix = if before { "cal_before_" } else { "cal_after_" };
                rows.iter()
                    .filter(|row| {
                        row["point_index"] == index
                            && row["candidate_id"] == arm.id
                            && row["name"]
                                .as_str()
                                .is_some_and(|name| name.starts_with(prefix))
                            && row["status"] == "PASS_INDEPENDENT_REPLAY"
                    })
                    .filter_map(|row| row["online_ms"].as_f64())
                    .collect()
            };
            let b = timings(true);
            let a = timings(false);
            match (median_three(&b), median_three(&a)) {
                (Ok(before), Ok(after)) if (after - before).abs() / before <= 0.1 => (),
                _ => calibration_status = false,
            }
        }
    }
    let all_events_pass = rows.len() == 256
        && rows
            .iter()
            .all(|row| row["status"] == "PASS_INDEPENDENT_REPLAY");
    let order_status = all_events_pass
        && rows
            .windows(2)
            .all(|pair| pair[0]["finished_utc"].as_str() < pair[1]["finished_utc"].as_str());
    let completed = !output.join("producer_stop.json").exists()
        && output.join("host_telemetry_after.json").is_file();
    let admitted = all_events_pass && calibration_status && order_status && completed;
    let host_end = if completed {
        read(output, "host_telemetry_after.json")?["recorded_utc"].clone()
    } else {
        Value::Null
    };
    let analysis = json!({"schema":"n83.crossfamily-current-native-analysis/v1",
        "curve_id":CURVE_ID,"subgroup_order":ORDER,"source_sha256":SOURCE_SHA256,
        "status":if admitted {"STRATUM_ADMITTED"} else {"BOUNDED_UNKNOWN"},
        "tier_status":if admitted {"PASS_TIER1"} else {"NOT_ADMITTED"},
        "stratum":stratum,"instance_id":host["parent_instance_id"],
        "host_start_utc":host["recorded_utc"],"host_end_utc":host_end,
        "source_head_sha":allocation["source_head_sha"],
        "targets_sha256":FILES[0].1,"answers_sha256":FILES[1].1,
        "all_events_pass":all_events_pass,"calibration_status":calibration_status,
        "execution_order_status":order_status,
        "producer_completed":completed,
        "rows":rows});
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(analysis_output)?;
    serde_json::to_writer_pretty(&mut file, &analysis)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(analysis)
}
