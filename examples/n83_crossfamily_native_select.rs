//! Recompute frozen N83 four-arm paired selection from archived stratum analyses.
//! Usage: n83_crossfamily_native_select <archive-dir> <provider-jobs.json> <native-arithmetic-replay.json> <native-resource-replay.json>
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::process::Command;

use crypto_lib::hash::sha256::Sha256;
use serde_json::{json, Value};

const ZIP_SHA256: [&str; 4] = [
    "036efe7a2e759524b7367544841e10a0ab3e0cf807580e03514ba4af5f3c968d",
    "d42aa19364ac541810ac6b1ca0bf1aed4882c9006ab17b881142b59b3a3e87d1",
    "cab308248b6abdc9ef6047bf905f6d0c648d749f1ac99289c25614eb49c2aa18",
    "f72a1b2563cce8798271e316042eb7f016f54c8c7089ba99bc0f5f2ac60b33ec",
];
const PROVIDER_SHA256: &str = "fa4126b6ad2343f319af44429feea88c01e049abcbea568355eca536e2a7ce0b";
const ARITHMETIC_REPLAY_SHA256: &str =
    "ccceea9197e9da9fb8fab4667913bd54caa9e20c7d57be9c5dd841a9c398d414";
const RESOURCE_REPLAY_SHA256: &str =
    "3d3231161622e729db7f3d4330d1af644e647802c33424b217e9a7e35a8eed5a";

const IDS: [&str; 4] = [
    "IC1N83Ckb1a1fb467456PDP4rootRCguidedLAgaussTDdirectISO0h7a3cca2a9993",
    "IC1N83Ckb1a1fb509952PDP4rootRCguidedLAgaussTDdirectISO0hde7207572a27",
    "IC1N83Ckb1a1fb572700PDP4rootRCguidedLAgaussTDdirectISO0hd7dca5070efb",
    "IC1N83Ckb1a1fb581000PDP4rootRCguidedLAgaussTDdirectISO0h441d3898d220",
];
const T: f64 = 2.068_657_610_419_04;
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

fn read_zip(zip: &Path, member: &str) -> Result<Value> {
    let output = Command::new("/usr/bin/unzip")
        .arg("-p")
        .arg(zip)
        .arg(member)
        .output()?;
    if !output.status.success() {
        return Err("archived analysis unreadable".into());
    }
    Ok(serde_json::from_slice(&output.stdout)?)
}

fn metric(arm: &Value, key: &str) -> Result<f64> {
    let x = arm[key].as_f64().ok_or("missing metric")?;
    if !x.is_finite() || x <= 0.0 {
        return Err("invalid metric".into());
    }
    Ok(x)
}

fn sign_sums(values: &[f64]) -> Vec<f64> {
    let mut out = vec![0.0];
    for &x in values {
        let mut next = Vec::with_capacity(out.len() * 2);
        for &v in &out {
            next.push(v + x);
        }
        for &v in &out {
            next.push(v - x);
        }
        out = next;
    }
    out
}

fn exact_signflip(values: &[f64]) -> f64 {
    let observed = values.iter().sum::<f64>().abs();
    let tolerance = 1e-10 * values.iter().map(|x| x.abs()).sum::<f64>().max(1.0);
    if observed <= tolerance {
        return 1.0;
    }
    let left = sign_sums(&values[..12]);
    let mut right = sign_sums(&values[12..]);
    right.sort_by(f64::total_cmp);
    let mut count = 0usize;
    for x in left {
        count += right.partition_point(|y| *y <= -observed + tolerance - x);
        count += right.len() - right.partition_point(|y| *y < observed - tolerance - x);
    }
    count as f64 / (1u64 << 24) as f64
}

fn gray_signflip(values: &[f64]) -> f64 {
    let observed = values.iter().sum::<f64>().abs();
    let tolerance = 1e-10 * values.iter().map(|x| x.abs()).sum::<f64>().max(1.0);
    let mut state = 0u32;
    let mut sum = -values.iter().sum::<f64>();
    let mut count = 0usize;
    for index in 0u32..(1u32 << 24) {
        if index > 0 {
            let bit = index.trailing_zeros() as usize;
            let mask = 1u32 << bit;
            sum += if state & mask == 0 {
                2.0 * values[bit]
            } else {
                -2.0 * values[bit]
            };
            state ^= mask;
        }
        if sum.abs() >= observed - tolerance {
            count += 1;
        }
    }
    count as f64 / (1u64 << 24) as f64
}

fn summary(values: &[f64], definition: &str) -> Value {
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / 23.0;
    let p = exact_signflip(values);
    assert!((p - gray_signflip(values)).abs() < 1e-6);
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    json!({"definition":definition,"count":values.len(),"mean":mean,
           "median":(sorted[11]+sorted[12])/2.0,
           "lower_95":mean-T*(variance/24.0).sqrt(),
           "upper_95":mean+T*(variance/24.0).sqrt(),
           "positive":values.iter().filter(|x| **x>0.0).count(),
           "negative":values.iter().filter(|x| **x<0.0).count(),
           "two_sided_signflip_p":p,
           "threshold_status":if p<0.05/6.0 {"PASS_THRESHOLD"} else {"BOUNDED_UNKNOWN"},
           "per_point_differences":values})
}

fn select_metric(points: &BTreeMap<usize, [Value; 4]>, key: &str) -> Result<Value> {
    let online = key == "one_target_online_ms";
    let mut means = [0.0f64; 4];
    let mut medians = [0.0f64; 4];
    for i in 0..4 {
        let mut times = points
            .values()
            .map(|arms| metric(&arms[i], key))
            .collect::<Result<Vec<f64>>>()?;
        means[i] = if online {
            (times.iter().map(|x| x.ln()).sum::<f64>() / 24.0).exp()
        } else {
            times.iter().sum::<f64>() / 24.0
        };
        times.sort_by(f64::total_cmp);
        medians[i] = (times[11] + times[12]) / 2.0;
    }
    let fastest = (0..4)
        .min_by(|&a, &b| means[a].total_cmp(&means[b]))
        .ok_or("no fastest")?;
    let mut comparisons = Vec::new();
    let mut admitted = true;
    for i in 0..4 {
        for j in i + 1..4 {
            let diffs = points
                .values()
                .map(|arms| {
                    let first = metric(&arms[i], key)?;
                    let second = metric(&arms[j], key)?;
                    Ok(if online {
                        (second / first).ln()
                    } else {
                        second - first
                    })
                })
                .collect::<Result<Vec<f64>>>()?;
            let mut result = summary(
                &diffs,
                if online {
                    "log_second_over_first"
                } else {
                    "second_minus_first_ms"
                },
            );
            let mean = result["mean"].as_f64().ok_or("contrast mean")?;
            let p = result["two_sided_signflip_p"]
                .as_f64()
                .ok_or("contrast p")?;
            if fastest == i && (mean <= 0.0 || p >= 0.05 / 6.0) {
                admitted = false;
            }
            if fastest == j && (mean >= 0.0 || p >= 0.05 / 6.0) {
                admitted = false;
            }
            result["first_candidate_id"] = json!(IDS[i]);
            result["second_candidate_id"] = json!(IDS[j]);
            result["geometric_second_over_first"] = if online {
                json!(mean.exp())
            } else {
                Value::Null
            };
            comparisons.push(result);
        }
    }
    Ok(
        json!({"status":if admitted {"SELECTED"} else {"BOUNDED_UNKNOWN"},
              "selected_candidate_id":if admitted {Some(IDS[fastest])} else {None},
              "descriptive_fastest_candidate_id":IDS[fastest],
              "alpha_per_pair":0.05/6.0,
              "mean_by_candidate_ms":means,
              "median_by_candidate_ms":medians,
              "comparisons":comparisons}),
    )
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        return Err("usage: n83_crossfamily_native_select <archive-dir> <provider-jobs.json> <native-arithmetic-replay.json> <native-resource-replay.json>".into());
    }
    let root = Path::new(&args[1]);
    let provider_path = Path::new(&args[2]);
    let arithmetic_path = Path::new(&args[3]);
    let resource_path = Path::new(&args[4]);
    if sha_file(provider_path)? != PROVIDER_SHA256
        || sha_file(arithmetic_path)? != ARITHMETIC_REPLAY_SHA256
        || sha_file(resource_path)? != RESOURCE_REPLAY_SHA256
    {
        return Err("provider or native replay digest differs".into());
    }
    let arithmetic: Value = serde_json::from_slice(&fs::read(arithmetic_path)?)?;
    if arithmetic["status"] != "PASS_NATIVE_IC_AND_PAIRED_RHO_TARGET_REPLAY"
        || arithmetic["points"] != 24
        || arithmetic["ic_runs"] != 96
        || arithmetic["paired_rho_runs"] != 96
    {
        return Err("independent arithmetic replay absent".into());
    }
    let resource: Value = serde_json::from_slice(&fs::read(resource_path)?)?;
    if resource["status"] != "PASS_NATIVE_RAW_RESOURCE_AND_CALIBRATION_REPLAY"
        || resource["calibration_scalar_replay"] != true
        || resource["event_count"] != 768
        || resource["ic_runs"] != 96
        || resource["paired_rho_runs"] != 96
        || resource["calibration_runs"] != 576
        || resource["archive_zip_sha256"] != json!(ZIP_SHA256)
        || resource["maximum_calibration_relative_spread"]
            .as_f64()
            .ok_or("calibration spread absent")?
            > 0.10
    {
        return Err("independent resource replay absent".into());
    }
    let provider: Value = serde_json::from_slice(&fs::read(provider_path)?)?;
    if provider["total_count"] != 4 {
        return Err("four jobs required".into());
    }
    let jobs = provider["jobs"].as_array().ok_or("jobs absent")?;
    let mut runner_ids = BTreeSet::new();
    let mut starts = Vec::new();
    let mut ends = Vec::new();
    for job in jobs {
        if job["status"] != "completed"
            || job["conclusion"] != "success"
            || job["run_id"] != 39075872898u64
        {
            return Err("provider job failed".into());
        }
        if !runner_ids.insert(job["runner_id"].as_u64().ok_or("runner id")?) {
            return Err("runner duplicated".into());
        }
        starts.push(job["started_at"].as_str().ok_or("start")?);
        ends.push(job["completed_at"].as_str().ok_or("end")?);
    }
    if starts.iter().max() >= ends.iter().min() {
        return Err("allocations did not overlap".into());
    }
    let mut points: BTreeMap<usize, [Value; 4]> = BTreeMap::new();
    for stratum in 0..4 {
        let archive = root.join(format!("stratum{stratum}.zip"));
        if sha_file(&archive)? != ZIP_SHA256[stratum] {
            return Err("archived stratum ZIP digest differs".into());
        }
        let analysis = read_zip(&archive, "results/analysis.json")?;
        let host = read_zip(&archive, "results/host.json")?;
        let provider_name = format!("crossfamily-stratum ({stratum})");
        let job = jobs
            .iter()
            .find(|job| job["name"] == provider_name)
            .ok_or("provider stratum absent")?;
        if host["schema"] != "n83.crossfamily-cancel-host/v1"
            || host["stratum"] != stratum
            || host["runner_name"] != job["runner_name"]
            || host["git_sha"] != "577c8cbe9ddfa20a2b6a62d8c39eb1dd339497ed"
            || host["run_url"] != "https://github.com/aburan28/crypto/actions/runs/39075872898"
            || host["recorded_utc"]
                .as_str()
                .ok_or("host receipt timestamp absent")?
                < job["started_at"].as_str().ok_or("provider start absent")?
            || host["recorded_utc"]
                .as_str()
                .ok_or("host receipt timestamp absent")?
                > job["completed_at"].as_str().ok_or("provider end absent")?
        {
            return Err("provider host binding failed".into());
        }
        if analysis["schema"] != "n83.crossfamily-cancel-stratum-analysis/v1"
            || analysis["stratum"] != stratum
            || analysis["status"] != "STRATUM_ADMITTED"
            || analysis["tier_status"] != "PASS_TIER1"
            || analysis["aa_status"] != "PASS_10_PERCENT_PER_PAIR"
            || analysis["source_sha256"]
                != "8109fb33374ce2aa66c6a5a0b6e144d3b0143168ac756f9e0bd70af60a911829"
            || analysis["targets_sha256"]
                != "ac0b84bd916291331e0dd13bfb312aedcd50d7cd3bbc1c3cd5f81eeca22ea73d"
        {
            return Err("stratum admission failed".into());
        }
        let rows = analysis["points"].as_array().ok_or("points absent")?;
        if rows.len() != 6 {
            return Err("six points per stratum required".into());
        }
        for row in rows {
            let index = row["point_index"].as_u64().ok_or("point index")? as usize;
            if index / 6 != stratum || points.contains_key(&index) {
                return Err("point coverage".into());
            }
            let arms = row["arms"].as_array().ok_or("arms absent")?;
            if arms.len() != 4 {
                return Err("four arms required".into());
            }
            let selected = std::array::from_fn(|i| {
                arms.iter()
                    .find(|arm| arm["candidate_id"] == IDS[i])
                    .cloned()
                    .unwrap_or(Value::Null)
            });
            for arm in &selected {
                if arm.is_null()
                    || arm["status"] != "PASS_INDEPENDENT_REPLAY"
                    || arm["resource_status"] != "PASS"
                {
                    return Err("arm replay failed".into());
                }
                metric(arm, "one_target_online_ms")?;
                metric(arm, "cold_process_ms")?;
            }
            points.insert(index, selected);
        }
    }
    if points.keys().copied().collect::<Vec<_>>() != (0..24).collect::<Vec<_>>() {
        return Err("24 point coverage failed".into());
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "status":"NATIVE_SELECTION_OVER_ARCHIVED_ANALYSIS",
            "independent_arithmetic_replay_status":arithmetic["status"],
            "resource_replay":resource["status"],
            "resource_replay_sha256":RESOURCE_REPLAY_SHA256,
            "archive_zip_sha256":ZIP_SHA256,
            "provider_jobs_sha256":PROVIDER_SHA256,
            "provider_runner_ids":runner_ids,
            "online":select_metric(&points,"one_target_online_ms")?,
            "cold":select_metric(&points,"cold_process_ms")?,
        }))?
    );
    Ok(())
}
