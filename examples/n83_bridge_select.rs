//! Native exact paired selector for four independently replayed N83 bridge strata.
//! Usage: n83_bridge_select <fixture-dir> <provider-jobs.json> <run-id> <head-sha>
//!        <analysis0.json> ... <analysis3.json> <archive0.json> ... <archive3.json> <output.json>

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::Path;

use crypto_lib::hash::sha256;
use serde_json::{json, Value};

const N: usize = 32;
const T_975_DF31: f64 = 2.039_513_446_396_407_7;
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
fn metric(row: &Value, name: &str) -> Result<f64> {
    let value = match name {
        "online" => &row["ic"]["target_online_ms"],
        "cold" => &row["ic"]["cold_process_ms"],
        "child_wall" => &row["ic_child_wall_ms"],
        _ => return Err(fail("unknown metric")),
    };
    let result = value
        .as_f64()
        .ok_or_else(|| fail(format!("{name} missing")))?;
    if !result.is_finite() || result <= 0.0 {
        return Err(fail(format!("{name} is not positive")));
    }
    Ok(result)
}
fn sign_test_p(positive: usize, negative: usize, ties: usize) -> Option<f64> {
    if ties > 0 || positive + negative != N {
        return None;
    }
    let dominant = positive.max(negative);
    let mut choose = 1u64;
    let mut tail = 0u64;
    for k in 0..=N {
        if k >= dominant {
            tail += choose;
        }
        if k < N {
            choose = choose * (N - k) as u64 / (k + 1) as u64;
        }
    }
    Some((2.0 * tail as f64 / (1u64 << N) as f64).min(1.0))
}
fn summarize(matrix: &[[Value; 2]], name: &str, ids: &[String; 2]) -> Result<Value> {
    let mut differences = Vec::new();
    let mut first_times = Vec::new();
    let mut second_times = Vec::new();
    for pair in matrix {
        let a = metric(&pair[0], name)?;
        let b = metric(&pair[1], name)?;
        first_times.push(a);
        second_times.push(b);
        differences.push(if name == "online" {
            (b / a).ln()
        } else {
            b - a
        });
    }
    let positive = differences.iter().filter(|x| **x > 0.0).count();
    let negative = differences.iter().filter(|x| **x < 0.0).count();
    let ties = N - positive - negative;
    let mean = differences.iter().sum::<f64>() / N as f64;
    let variance = differences.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (N - 1) as f64;
    let halfwidth = T_975_DF31 * variance.sqrt() / (N as f64).sqrt();
    let p = sign_test_p(positive, negative, ties);
    let selected = if p.is_some_and(|value| value <= 0.05) {
        if positive >= 23 && mean > 0.0 {
            Some(ids[0].as_str())
        } else if negative >= 23 && mean < 0.0 {
            Some(ids[1].as_str())
        } else {
            None
        }
    } else {
        None
    };
    let first_mean = first_times.iter().sum::<f64>() / N as f64;
    let second_mean = second_times.iter().sum::<f64>() / N as f64;
    Ok(json!({
        "status":if selected.is_some(){"SELECTED"}else{"BOUNDED_UNKNOWN"},
        "selected_candidate_id":selected,
        "first_candidate_id":ids[0],"second_candidate_id":ids[1],
        "positive_second_minus_first":positive,"negative_second_minus_first":negative,
        "ties":ties,"two_sided_exact_sign_p":p,
        "mean_difference":mean,"lower_95":mean-halfwidth,"upper_95":mean+halfwidth,
        "difference_definition":if name=="online"{"log_second_over_first"}else{"second_minus_first_ms"},
        "geometric_second_over_first":if name=="online"{Some(mean.exp())}else{None},
        "first_arithmetic_mean_ms":first_mean,"second_arithmetic_mean_ms":second_mean,
        "per_point_differences":differences,
    }))
}
fn select(
    fixture: &Path,
    provider: &Path,
    run_id: &str,
    head_sha: &str,
    analyses: &[&Path; 4],
    archive_receipts: &[&Path; 4],
) -> Result<Value> {
    let protocol = read(&fixture.join("protocol.json"))?;
    let workloads_path = fixture.join("workloads.json");
    let workloads = read(&workloads_path)?;
    if protocol["workloads_sha256"] != sha(&workloads_path)?
        || workloads["schema"] != "n83.native-bridge-workloads/v1"
        || workloads["rows"].as_array().map(|rows| rows.len()) != Some(N)
    {
        return Err(fail("32 frozen workload records required"));
    }
    let ids = [
        protocol["candidate_k1107"]
            .as_str()
            .ok_or("K1107 ID missing")?
            .to_owned(),
        protocol["candidate_k2304"]
            .as_str()
            .ok_or("K2304 ID missing")?
            .to_owned(),
    ];
    let jobs = read(provider)?;
    if jobs["total_count"] != 4 {
        return Err(fail("four provider jobs required"));
    }
    let provider_rows = jobs["jobs"].as_array().ok_or("provider jobs missing")?;
    if provider_rows.len() != 4 {
        return Err(fail("four provider rows required"));
    }
    let mut runner_ids = HashSet::new();
    let mut runner_names = HashSet::new();
    let mut starts = Vec::new();
    let mut ends = Vec::new();
    let mut matrix: BTreeMap<usize, [Option<Value>; 2]> = BTreeMap::new();
    let mut artifact_hashes = Vec::new();
    for stratum in 0..4 {
        let analysis = read(analyses[stratum])?;
        let archive = read(archive_receipts[stratum])?;
        let member = archive["members"]
            .as_array()
            .and_then(|members| {
                members
                    .iter()
                    .find(|member| member["path"] == "results/analysis.json")
            })
            .ok_or_else(|| fail(format!("stratum {stratum}: archived analysis absent")))?;
        if archive["schema"] != "n83.native-bridge-archive/v1"
            || archive["member_count"].as_u64().unwrap_or(0) < 100
            || member["sha256"] != sha(analyses[stratum])?
        {
            return Err(fail(format!(
                "stratum {stratum}: raw archive binding differs"
            )));
        }
        artifact_hashes.push(archive["artifact_sha256"].clone());
        if analysis["schema"] != "n83.native-bridge-stratum-analysis/v1"
            || analysis["status"] != "PASS_TIER1"
            || analysis["stratum"] != stratum
            || analysis["targets_sha256"] != protocol["targets_sha256"]
            || analysis["source_sha256"] != protocol["source_sha256"]
            || analysis["host"]["git_sha"] != head_sha
            || analysis["host"]["github_run_id"] != run_id
        {
            return Err(fail(format!("stratum {stratum}: replay or source gate")));
        }
        let name = format!("bridge-stratum ({stratum})");
        let job = provider_rows
            .iter()
            .find(|job| job["name"] == name)
            .ok_or_else(|| fail(format!("provider job {name} absent")))?;
        if job["status"] != "completed"
            || job["conclusion"] != "success"
            || job["run_id"].to_string().trim_matches('"') != run_id
            || job["runner_name"] != analysis["host"]["runner_name"]
        {
            return Err(fail(format!("stratum {stratum}: provider binding differs")));
        }
        let runner_id = job["runner_id"]
            .as_u64()
            .ok_or("provider runner id absent")?;
        let runner_name = job["runner_name"]
            .as_str()
            .ok_or("provider runner name absent")?;
        if runner_id == 0
            || !runner_ids.insert(runner_id)
            || !runner_names.insert(runner_name.to_owned())
        {
            return Err(fail("four distinct provider runners required"));
        }
        starts.push(
            job["started_at"]
                .as_str()
                .ok_or("job start absent")?
                .to_owned(),
        );
        ends.push(
            job["completed_at"]
                .as_str()
                .ok_or("job end absent")?
                .to_owned(),
        );
        let rows = analysis["rows"].as_array().ok_or("stratum rows missing")?;
        if rows.len() != 16 {
            return Err(fail("16 arm-point rows required per stratum"));
        }
        for row in rows {
            let point = row["point_index"].as_u64().ok_or("point index absent")? as usize;
            let k = row["k"].as_u64().ok_or("K absent")? as usize;
            let fixed = if k == 1107 {
                0
            } else if k == 2304 {
                1
            } else {
                return Err(fail("K differs"));
            };
            if point / 8 != stratum
                || row["candidate_id"] != ids[fixed]
                || row["calibration_status"] != "PASS_10_PERCENT"
                || row["ic"]["status"] != "PASS_INDEPENDENT_REPLAY"
                || row["rho"]["status"] != "PASS_INDEPENDENT_REPLAY"
            {
                return Err(fail(format!(
                    "stratum {stratum}: point {point} admission differs"
                )));
            }
            let entry = matrix.entry(point).or_insert_with(|| [None, None]);
            if entry[fixed].replace(row.clone()).is_some() {
                return Err(fail("duplicate candidate-point row"));
            }
        }
    }
    if starts.iter().max() >= ends.iter().min() {
        return Err(fail("provider allocations did not overlap"));
    }
    if matrix.len() != N || matrix.keys().copied().collect::<Vec<_>>() != (0..N).collect::<Vec<_>>()
    {
        return Err(fail("32 paired public points required"));
    }
    let matrix: Vec<[Value; 2]> = matrix
        .into_values()
        .map(|pair| {
            Ok([
                pair[0].clone().ok_or("K1107 missing")?,
                pair[1].clone().ok_or("K2304 missing")?,
            ])
        })
        .collect::<Result<_>>()?;
    for (index, pair) in matrix.iter().enumerate() {
        let workload_id = workloads["rows"][index]["workload_id"]
            .as_str()
            .ok_or_else(|| fail("workload ID missing"))?;
        for fixed in 0..2 {
            let run_id = format!("{}W{workload_id}R1", ids[fixed]);
            if pair[fixed]["workload_id"] != workload_id
                || pair[fixed]["run_id"] != run_id
                || pair[fixed]["target"]
                    != workloads["rows"][index]["canonical_record"]["public_target_q"]
            {
                return Err(fail(format!(
                    "point {index}: canonical workload or run ID differs"
                )));
            }
        }
    }
    let ratios: Vec<[f64; 2]> = matrix
        .iter()
        .map(|pair| {
            Ok([
                pair[0]["rho"]["online_ms"]
                    .as_f64()
                    .ok_or("rho online missing")?
                    / metric(&pair[0], "online")?,
                pair[1]["rho"]["online_ms"]
                    .as_f64()
                    .ok_or("rho online missing")?
                    / metric(&pair[1], "online")?,
            ])
        })
        .collect::<Result<_>>()?;
    Ok(json!({
        "schema":"n83.native-bridge-selection/v1","status":"PASS_TIER1",
        "run_id":run_id,"head_sha":head_sha,"candidate_ids":ids,
        "point_count":N,"online":summarize(&matrix,"online",&ids)?,
        "cold":summarize(&matrix,"cold",&ids)?,
        "child_wall":summarize(&matrix,"child_wall",&ids)?,
        "paired_rho_over_ic_online":ratios,
        "all_32_same_point_ic_faster_than_rho_by_candidate":[
            ratios.iter().all(|r|r[0]>1.0),ratios.iter().all(|r|r[1]>1.0)],
        "provider_runner_ids":runner_ids,
        "raw_artifact_sha256":artifact_hashes,
    }))
}
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 14 {
        return Err(fail("usage: n83_bridge_select <fixture-dir> <provider-jobs.json> <run-id> <head-sha> <analysis0> ... <analysis3> <archive0> ... <archive3> <output.json>"));
    }
    let analyses = [
        Path::new(&args[5]),
        Path::new(&args[6]),
        Path::new(&args[7]),
        Path::new(&args[8]),
    ];
    let archive_receipts = [
        Path::new(&args[9]),
        Path::new(&args[10]),
        Path::new(&args[11]),
        Path::new(&args[12]),
    ];
    let selection = select(
        Path::new(&args[1]),
        Path::new(&args[2]),
        &args[3],
        &args[4],
        &analyses,
        &archive_receipts,
    );
    let (result, admitted) = match selection {
        Ok(value) => (value, true),
        Err(error) => (
            json!({
                "schema":"n83.native-bridge-selection/v1",
                "status":"BOUNDED_UNKNOWN","admission_error":error.to_string(),
                "online":{"status":"BOUNDED_UNKNOWN","selected_candidate_id":null},
                "cold":{"status":"BOUNDED_UNKNOWN","selected_candidate_id":null},
                "child_wall":{"status":"BOUNDED_UNKNOWN","selected_candidate_id":null},
            }),
            false,
        ),
    };
    let output = Path::new(&args[13]);
    if output.exists() {
        return Err(fail("selector output already exists"));
    }
    fs::write(output, serde_json::to_vec_pretty(&result)?)?;
    println!(
        "{}",
        json!({"status":result["status"],
                           "online_selected":result["online"]["selected_candidate_id"],
                           "cold_selected":result["cold"]["selected_candidate_id"],
                           "child_wall_selected":result["child_wall"]["selected_candidate_id"]})
    );
    if !admitted {
        std::process::exit(2);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_sign_tail_is_capped_at_one() {
        assert_eq!(sign_test_p(16, 16, 0), Some(1.0));
        assert_eq!(sign_test_p(32, 0, 0), Some(2.0 / 2f64.powi(32)));
        assert_eq!(sign_test_p(31, 0, 1), None);
    }
}
