//! Recheck raw resource and A/A calibration receipts for the four-stratum N83 panel.
//! Usage: n83_crossfamily_native_resource_replay <archive-dir>

use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use crypto_lib::hash::sha256::Sha256;
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::process::Command;

const ZIP_SHA256: [&str; 4] = [
    "036efe7a2e759524b7367544841e10a0ab3e0cf807580e03514ba4af5f3c968d",
    "d42aa19364ac541810ac6b1ca0bf1aed4882c9006ab17b881142b59b3a3e87d1",
    "cab308248b6abdc9ef6047bf905f6d0c648d749f1ac99289c25614eb49c2aa18",
    "f72a1b2563cce8798271e316042eb7f016f54c8c7089ba99bc0f5f2ac60b33ec",
];
const CAL_TARGET_SHA256: &str = "2c068d2f659588edcdbaa685fc73bb146e2bfee3be123d377d537658f8021c6d";
const CAL_ANSWER_SHA256: &str = "b71c5a8bfdae603931fa6c03fc6f6faa3b261288cf787ac682bf8a3e3afe262d";
const RSS_CAP_BYTES: u64 = 16_374_562_816;
const ARMS: [(&str, u64); 4] = [
    ("k2816_root11", 900),
    ("k3072_root11", 900),
    ("k3450_wide3", 1200),
    ("k3500_root4", 1200),
];

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn sha_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 1024 * 1024];
    loop {
        let count = file.read(&mut buf)?;
        if count == 0 {
            break;
        }
        hasher.update(&buf[..count]);
    }
    Ok(hex::encode(hasher.finalize()))
}

fn zip_member(zip: &Path, member: &str) -> Result<Vec<u8>> {
    let output = Command::new("/usr/bin/unzip")
        .arg("-p")
        .arg(zip)
        .arg(member)
        .output()?;
    if !output.status.success() || output.stdout.is_empty() {
        return Err(format!("missing ZIP member {member} in {}", zip.display()).into());
    }
    Ok(output.stdout)
}

fn zip_json(zip: &Path, member: &str) -> Result<Value> {
    Ok(serde_json::from_slice(&zip_member(zip, member)?)?)
}

fn counter(snapshot: &Value, section: &str, name: &str) -> Result<u64> {
    let text = snapshot[section]
        .as_str()
        .ok_or_else(|| format!("missing cgroup {section}"))?;
    let line = text
        .lines()
        .find(|line| line.split_whitespace().next() == Some(name))
        .ok_or_else(|| format!("missing cgroup counter {section}:{name}"))?;
    Ok(line
        .split_whitespace()
        .nth(1)
        .ok_or("missing cgroup counter value")?
        .parse()?)
}

fn metric(row: &Value, key: &str) -> Result<f64> {
    let value = row[key].as_f64().ok_or_else(|| format!("missing {key}"))?;
    if !value.is_finite() || value <= 0.0 {
        return Err(format!("invalid {key}").into());
    }
    Ok(value)
}

fn median_three(mut values: [f64; 3]) -> f64 {
    values.sort_by(f64::total_cmp);
    values[1]
}

fn calibration_scalar_replay(target: &Value, scalar: &str) -> Result<()> {
    let coords = target
        .as_array()
        .ok_or("calibration point is not an array")?;
    if coords.len() != 2 {
        return Err("calibration point arity differs".into());
    }
    let words = coords
        .iter()
        .map(|coord| {
            BigUint::parse_bytes(coord.as_str().ok_or("coordinate absent")?.as_bytes(), 10)
                .ok_or("invalid calibration coordinate".into())
        })
        .collect::<Result<Vec<BigUint>>>()?;
    let curve = KoblitzCurve::new(1, 83).ok_or("N83 curve unavailable")?;
    let point = BinaryPoint::Affine {
        x: F2mElement::from_biguint(&words[0], 83),
        y: F2mElement::from_biguint(&words[1], 83),
    };
    let scalar = BigUint::parse_bytes(scalar.as_bytes(), 10).ok_or("invalid calibration scalar")?;
    if scalar == BigUint::from(0u8)
        || scalar >= curve.subgroup_order
        || curve.mul(curve.generator(), &scalar) != point
        || curve.mul(&point, &curve.subgroup_order) != BinaryPoint::Infinity
    {
        return Err("calibration scalar or subgroup replay failed".into());
    }
    Ok(())
}

struct Envelope<'a> {
    cpu_max: &'a str,
    memory_max: &'a str,
    cal_target: &'a Value,
    cal_scalar: &'a str,
}

fn check_event(
    zip: &Path,
    name: &str,
    cap: u64,
    envelope: &Envelope<'_>,
    calibration: bool,
) -> Result<(Option<f64>, u64, String, String)> {
    let launch = zip_json(zip, &format!("{name}/launch.json"))?;
    let usage = zip_json(zip, &format!("{name}/usage.json"))?;
    let telemetry = zip_json(zip, &format!("{name}/host_telemetry.json"))?;
    let exit = String::from_utf8(zip_member(zip, &format!("{name}/exit_status.txt"))?)?;
    if exit.trim() != "0"
        || launch["cap_seconds"] != cap
        || launch["rss_cap_bytes"] != RSS_CAP_BYTES
        || usage["status"] != "PASS_PRODUCER"
        || usage["exit_status"] != 0
        || !usage["stop_reason"].is_null()
        || usage["peak_rss_bytes"].as_u64().ok_or("missing peak RSS")? > RSS_CAP_BYTES
        || metric(&usage, "wall_seconds").is_err()
    {
        return Err(format!("producer, cap or RSS failure: {name}").into());
    }
    for boundary in ["cgroup_before", "cgroup_after"] {
        let snapshot = &usage[boundary];
        if snapshot["cpu.max"] != envelope.cpu_max || snapshot["memory.max"] != envelope.memory_max
        {
            return Err(format!("cgroup envelope changed: {name} {boundary}").into());
        }
    }
    for (section, key) in [
        ("cpu.stat", "nr_throttled"),
        ("cpu.stat", "throttled_usec"),
        ("memory.events", "oom"),
        ("memory.events", "oom_kill"),
    ] {
        let before = counter(&usage["cgroup_before"], section, key)?;
        let after = counter(&usage["cgroup_after"], section, key)?;
        if before != after {
            return Err(format!("cgroup {section}:{key} changed: {name}").into());
        }
    }
    let steal_before = telemetry["before"]["steal_jiffies"]
        .as_u64()
        .ok_or("missing steal before")?;
    let steal_after = telemetry["after"]["steal_jiffies"]
        .as_u64()
        .ok_or("missing steal after")?;
    if steal_before != steal_after {
        return Err(format!("steal time changed: {name}").into());
    }
    let started = launch["started_utc"]
        .as_str()
        .ok_or("missing start timestamp")?
        .to_owned();
    let finished = usage["finished_utc"]
        .as_str()
        .ok_or("missing finish timestamp")?
        .to_owned();
    if started >= finished {
        return Err(format!("invalid event interval: {name}").into());
    }
    let online = if calibration {
        let receipt = zip_json(zip, &format!("{name}/receipt.json"))?;
        if receipt["status"] != "PASS_VERIFIED_PUBLIC_TARGET"
            || receipt["public_target_q"] != *envelope.cal_target
            || receipt["recovered_scalar"] != envelope.cal_scalar
            || receipt["walk_seed"] != 2026101601u64
            || receipt["target_count"] != 1
            || receipt["known_answer_input"] != false
            || receipt["validation_sidecar_opened"] != false
            || receipt["group_verified"] != true
            || receipt["generic_scalar_replay_verified"] != true
        {
            return Err(format!("calibration target or scalar differs: {name}").into());
        }
        Some(metric(&receipt, "online_ms")?)
    } else {
        None
    };
    Ok((
        online,
        usage["peak_rss_bytes"].as_u64().unwrap(),
        started,
        finished,
    ))
}

fn run(root: &Path) -> Result<Value> {
    let cal_target_path = Path::new("research/n83_cold_factor_base_20261016/targets.jsonl");
    let cal_answer_path = Path::new("research/n83_cold_factor_base_20261016/answers.jsonl");
    if sha_file(cal_target_path)? != CAL_TARGET_SHA256
        || sha_file(cal_answer_path)? != CAL_ANSWER_SHA256
    {
        return Err("calibration fixture digest differs".into());
    }
    let cal_target: Value = serde_json::from_slice(&fs::read(cal_target_path)?)?;
    let cal_answer: Value = serde_json::from_slice(&fs::read(cal_answer_path)?)?;
    let cal_scalar = cal_answer["scalar"]
        .as_str()
        .ok_or("calibration scalar absent")?;
    if cal_answer["point"] != cal_target || cal_answer["fixture_index"] != 0 {
        return Err("calibration answer fixture differs".into());
    }
    calibration_scalar_replay(&cal_target, cal_scalar)?;
    let mut strata = Vec::new();
    let mut global_max_spread = 0.0f64;
    let mut global_peak_rss = 0u64;
    for stratum in 0..4 {
        let zip = root.join(format!("stratum{stratum}.zip"));
        if sha_file(&zip)? != ZIP_SHA256[stratum] {
            return Err(format!("stratum {stratum} ZIP digest differs").into());
        }
        let host = zip_json(&zip, "results/host.json")?;
        let host_after = zip_json(&zip, "results/host_telemetry_after.json")?;
        if host["schema"] != "n83.crossfamily-cancel-host/v1"
            || host["stratum"] != stratum
            || host["visible_affinity"] != json!([0, 1, 2, 3])
            || host["topology"]["cgroup_effective_cpus"] != "0-3"
            || host["cpu_telemetry_before"]["steal_jiffies"] != host_after["steal_jiffies"]
        {
            return Err(format!("host topology or steal gate failed: {stratum}").into());
        }
        let cpu_max = host["cgroup_before"]["cpu.max"]
            .as_str()
            .ok_or("host cpu.max absent")?;
        let memory_max = host["cgroup_before"]["memory.max"]
            .as_str()
            .ok_or("host memory.max absent")?;
        let envelope = Envelope {
            cpu_max,
            memory_max,
            cal_target: &cal_target,
            cal_scalar,
        };
        let mut intervals = Vec::new();
        let mut max_spread = 0.0f64;
        let mut peak_rss = 0u64;
        for point in stratum * 6..stratum * 6 + 6 {
            for (arm, ic_cap) in ARMS {
                let prefix = format!("results/points/p{point:02}/runs");
                let ic_name = format!("{prefix}/{arm}");
                let rho_name = format!("{prefix}/rho_{arm}");
                for (name, cap) in [(ic_name, ic_cap), (rho_name, 120)] {
                    let (_, rss, start, end) = check_event(&zip, &name, cap, &envelope, false)?;
                    peak_rss = peak_rss.max(rss);
                    intervals.push((start, end));
                }
                let mut before = [0.0; 3];
                let mut after = [0.0; 3];
                for repeat in 0..3 {
                    for (side, values) in [("before", &mut before), ("after", &mut after)] {
                        let name = format!("{prefix}/cal_{side}_{repeat}_{arm}");
                        let (online, rss, start, end) =
                            check_event(&zip, &name, 120, &envelope, true)?;
                        values[repeat] = online.ok_or("calibration online missing")?;
                        peak_rss = peak_rss.max(rss);
                        intervals.push((start, end));
                    }
                }
                let left = median_three(before);
                let right = median_three(after);
                let spread = (left - right).abs() / ((left + right) / 2.0);
                if spread > 0.10 {
                    return Err(format!("calibration noise gate failed: p{point:02} {arm}").into());
                }
                max_spread = max_spread.max(spread);
            }
        }
        if intervals.len() != 192 {
            return Err("stratum event count differs".into());
        }
        intervals.sort();
        for pair in intervals.windows(2) {
            if pair[0].1 > pair[1].0 {
                return Err(format!("events overlap in stratum {stratum}").into());
            }
        }
        global_max_spread = global_max_spread.max(max_spread);
        global_peak_rss = global_peak_rss.max(peak_rss);
        strata.push(json!({
            "stratum":stratum,
            "runner_name":host["runner_name"],
            "event_count":192,
            "ic_runs":24,
            "paired_rho_runs":24,
            "calibration_runs":144,
            "maximum_calibration_relative_spread":max_spread,
            "peak_rss_bytes":peak_rss,
            "status":"PASS_NATIVE_RAW_RESOURCE_AND_CALIBRATION_REPLAY"
        }));
    }
    Ok(json!({
        "schema":"n83.crossfamily-native-resource-replay/v1",
        "status":"PASS_NATIVE_RAW_RESOURCE_AND_CALIBRATION_REPLAY",
        "archive_zip_sha256":ZIP_SHA256,
        "calibration_targets_sha256":CAL_TARGET_SHA256,
        "calibration_answers_sha256":CAL_ANSWER_SHA256,
        "calibration_scalar_replay":true,
        "event_count":768,
        "ic_runs":96,
        "paired_rho_runs":96,
        "calibration_runs":576,
        "maximum_calibration_relative_spread":global_max_spread,
        "peak_rss_bytes":global_peak_rss,
        "strata":strata
    }))
}

fn main() -> Result<()> {
    let root = std::env::args()
        .nth(1)
        .ok_or("usage: n83_crossfamily_native_resource_replay <archive-dir>")?;
    println!("{}", serde_json::to_string_pretty(&run(Path::new(&root))?)?);
    Ok(())
}
