//! Frozen 16-point paired selector with exact sign-flip enumeration.
use super::{Result, ARMS, CURVE_ID, FILES, ORDER, SOURCE_SHA256};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

const ALPHA_PER_PAIR: f64 = 0.05 / 6.0;
const T_975_DF15: f64 = 2.131_449_545_559_323;

fn finite(row: &Value, key: &str) -> Result<f64> {
    let number = row[key].as_f64().ok_or_else(|| format!("missing {key}"))?;
    if !number.is_finite() || number <= 0.0 {
        return Err(format!("invalid {key}").into());
    }
    Ok(number)
}
fn mean(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / values.len() as f64
}
fn median(values: &[f64]) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    (sorted[(sorted.len() - 1) / 2] + sorted[sorted.len() / 2]) / 2.0
}
fn sign_flip_p(differences: &[f64]) -> Result<f64> {
    if differences.len() != 16 || differences.iter().any(|value| !value.is_finite()) {
        return Err("sixteen finite differences required".into());
    }
    let observed = differences.iter().sum::<f64>().abs();
    let tolerance = 1e-12 * differences.iter().map(|x| x.abs()).sum::<f64>().max(1.0);
    if observed <= tolerance {
        return Ok(1.0);
    }
    let extreme = (0..1usize << 16)
        .filter(|mask| {
            let sum = differences
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    if mask & (1 << index) == 0 {
                        -*value
                    } else {
                        *value
                    }
                })
                .sum::<f64>();
            sum.abs() + tolerance >= observed
        })
        .count();
    Ok(extreme as f64 / 65_536.0)
}
fn summary(differences: &[f64]) -> Result<Value> {
    let average = mean(differences);
    let sd = (differences
        .iter()
        .map(|x| (x - average).powi(2))
        .sum::<f64>()
        / 15.0)
        .sqrt();
    let half = T_975_DF15 * sd / 4.0;
    Ok(
        json!({"mean":average,"median":median(differences),"sample_sd":sd,
        "lower_95":average-half,"upper_95":average+half,
        "two_sided_sign_flip_p":sign_flip_p(differences)?}),
    )
}
#[derive(Clone, Copy)]
struct Times {
    online: f64,
    cold: f64,
    rho: f64,
}

fn arm_matrix(analysis: &[Value; 2]) -> Result<BTreeMap<usize, BTreeMap<&'static str, Times>>> {
    let mut matrix = BTreeMap::new();
    for (stratum, record) in analysis.iter().enumerate() {
        let rows = record["rows"].as_array().ok_or("stratum rows absent")?;
        for index in stratum * 8..stratum * 8 + 8 {
            let mut arms = BTreeMap::new();
            for arm in ARMS {
                let one = |kind: &str| -> Result<&Value> {
                    let matches = rows
                        .iter()
                        .filter(|row| {
                            row["point_index"] == index
                                && row["candidate_id"] == arm.id
                                && row["kind"] == kind
                                && row["status"] == "PASS_INDEPENDENT_REPLAY"
                        })
                        .collect::<Vec<_>>();
                    if matches.len() != 1 {
                        return Err(
                            format!("point {index} {} {kind} coverage differs", arm.name).into(),
                        );
                    }
                    Ok(matches[0])
                };
                let ic = one("ic")?;
                let rho = one("rho")?;
                arms.insert(
                    arm.id,
                    Times {
                        online: finite(ic, "online_ms")?,
                        cold: finite(ic, "cold_process_ms")?,
                        rho: finite(rho, "online_ms")?,
                    },
                );
            }
            matrix.insert(index, arms);
        }
    }
    if matrix.len() != 16 {
        return Err("sixteen paired points absent".into());
    }
    Ok(matrix)
}
fn compare(matrix: &BTreeMap<usize, BTreeMap<&str, Times>>, cold: bool) -> Result<Value> {
    let ids = ARMS.map(|arm| arm.id);
    let mut comparisons = Vec::new();
    let mut wins = BTreeMap::new();
    for id in ids {
        wins.insert(id, 0usize);
    }
    for first in 0..4 {
        for second in first + 1..4 {
            let differences = matrix
                .values()
                .map(|point| {
                    let a = point[ids[first]];
                    let b = point[ids[second]];
                    if cold {
                        b.cold - a.cold
                    } else {
                        (b.online / a.online).ln()
                    }
                })
                .collect::<Vec<_>>();
            let record = summary(&differences)?;
            let average = record["mean"].as_f64().ok_or("mean absent")?;
            let p = record["two_sided_sign_flip_p"].as_f64().ok_or("p absent")?;
            if p < ALPHA_PER_PAIR {
                if average > 0.0 {
                    *wins.get_mut(ids[first]).ok_or("winner absent")? += 1;
                }
                if average < 0.0 {
                    *wins.get_mut(ids[second]).ok_or("winner absent")? += 1;
                }
            }
            comparisons.push(json!({"first":ids[first],"second":ids[second],
                "difference_definition":if cold {"second_minus_first"} else {"log_second_over_first"},
                "summary":record}));
        }
    }
    let winners = wins
        .iter()
        .filter(|(_, count)| **count == 3)
        .map(|(id, _)| *id)
        .collect::<Vec<_>>();
    let means = ids
        .iter()
        .map(|id| {
            let sample = matrix
                .values()
                .map(|point| {
                    let row = point[id];
                    if cold {
                        row.cold
                    } else {
                        row.online.ln()
                    }
                })
                .collect::<Vec<_>>();
            (
                *id,
                if cold {
                    mean(&sample)
                } else {
                    mean(&sample).exp()
                },
            )
        })
        .collect::<BTreeMap<_, _>>();
    let medians = ids
        .iter()
        .map(|id| {
            (
                *id,
                median(
                    &matrix
                        .values()
                        .map(|point| {
                            let row = point[id];
                            if cold {
                                row.cold
                            } else {
                                row.online
                            }
                        })
                        .collect::<Vec<_>>(),
                ),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let fastest = ids
        .iter()
        .min_by(|a, b| means[*a].total_cmp(&means[*b]))
        .ok_or("no candidates")?;
    Ok(
        json!({"status":if winners.len()==1 {"SELECTED"} else {"BOUNDED_UNKNOWN"},
        "selected_candidate_id":if winners.len()==1 {Some(winners[0])} else {None},
        "descriptive_fastest_candidate_id":fastest,
        "mean_by_candidate_ms":means,"median_by_candidate_ms":medians,
        "bonferroni_alpha_per_pair":ALPHA_PER_PAIR,"comparisons":comparisons}),
    )
}
pub fn run(first: &Path, second: &Path, output: &Path) -> Result<Value> {
    let records = [
        serde_json::from_slice::<Value>(&fs::read(first)?)?,
        serde_json::from_slice::<Value>(&fs::read(second)?)?,
    ];
    let admitted = records.iter().enumerate().all(|(index, row)| {
        row["schema"] == "n83.crossfamily-current-native-analysis/v1"
            && row["curve_id"] == CURVE_ID
            && row["subgroup_order"] == ORDER
            && row["source_sha256"] == SOURCE_SHA256
            && row["status"] == "STRATUM_ADMITTED"
            && row["tier_status"] == "PASS_TIER1"
            && row["stratum"] == index
            && row["all_events_pass"] == true
            && row["calibration_status"] == true
            && row["producer_completed"] == true
            && row["targets_sha256"] == FILES[0].1
            && row["answers_sha256"] == FILES[1].1
            && row["rows"].as_array().is_some_and(|rows| rows.len() == 256)
    }) && records[0]["instance_id"] != records[1]["instance_id"]
        && records[0]["source_head_sha"] == records[1]["source_head_sha"]
        && records[0]["host_start_utc"]
            .as_str()
            .is_some_and(|t| !t.is_empty())
        && records[1]["host_start_utc"]
            .as_str()
            .is_some_and(|t| !t.is_empty())
        && records[0]["host_end_utc"]
            .as_str()
            .is_some_and(|t| !t.is_empty())
        && records[1]["host_end_utc"]
            .as_str()
            .is_some_and(|t| !t.is_empty())
        && records[0]["host_start_utc"].as_str() < records[1]["host_end_utc"].as_str()
        && records[1]["host_start_utc"].as_str() < records[0]["host_end_utc"].as_str();
    let mut status = "SOURCE_TARGET_OR_TIER_GATE";
    let mut online = json!({"status":"BOUNDED_UNKNOWN","selected_candidate_id":null});
    let mut cold = online.clone();
    let mut ratios = Value::Null;
    if admitted {
        let matrix = arm_matrix(&records)?;
        let ids = ARMS.map(|arm| arm.id);
        let bases = ids
            .iter()
            .map(|id| {
                records
                    .iter()
                    .flat_map(|record| record["rows"].as_array().into_iter().flatten())
                    .filter(|row| row["candidate_id"] == *id && row["kind"] == "ic")
                    .filter_map(|row| row["base_hash"].as_str())
                    .collect::<BTreeSet<_>>()
            })
            .collect::<Vec<_>>();
        if bases.iter().all(|set| set.len() == 1) {
            status = "PASS";
            online = compare(&matrix, false)?;
            cold = compare(&matrix, true)?;
            ratios = json!(ids
                .iter()
                .map(|id| (
                    *id,
                    matrix
                        .values()
                        .map(|point| point[id].rho / point[id].online)
                        .collect::<Vec<_>>()
                ))
                .collect::<BTreeMap<_, _>>());
        } else {
            status = "BASE_IDENTITY_GATE";
        }
    }
    let result = json!({"schema":"n83.crossfamily-current-native-selection/v1",
        "admission_status":status,"candidate_ids":ARMS.map(|arm|arm.id),
        "point_count":if status=="PASS" {16} else {0},
        "online":online,"cold":cold,"rho_over_ic_online_by_candidate":ratios,
        "stratum_analysis_paths":[first,second]});
    if output.exists() {
        return Err("selection output already exists".into());
    }
    fs::write(
        output,
        format!("{}\n", serde_json::to_string_pretty(&result)?),
    )?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sign_flip_extremes() -> Result<()> {
        assert_eq!(sign_flip_p(&[0.0; 16])?, 1.0);
        assert_eq!(sign_flip_p(&[1.0; 16])?, 2.0 / 65_536.0);
        Ok(())
    }
}
