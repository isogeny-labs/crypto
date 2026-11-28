//! Read-only accounting of archived PKM JSONL records.
//!
//! This adapter reports producer counters, never verifies a discrete logarithm
//! or promotes a historical subtotal to a qualified cold cost. Failed timed
//! rows remain in the same denominator as successful timed rows. Preflights
//! remain visible in separate groups. Solver work is already included in the
//! collection phase and is not added to that phase a second time.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Metric {
    pub sum: f64,
    pub present: usize,
    pub missing: usize,
}

impl Metric {
    fn add(&mut self, value: Option<f64>) {
        if let Some(value) = value.filter(|v| v.is_finite() && *v >= 0.0) {
            self.sum += value;
            self.present += 1;
        } else {
            self.missing += 1;
        }
    }

    pub fn complete_mean(&self) -> Option<f64> {
        (self.missing == 0 && self.present > 0).then(|| self.sum / self.present as f64)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GroupAccounting {
    pub source: String,
    pub stage: String,
    pub arm: String,
    pub method: String,
    pub engine: String,
    pub arity: Option<u64>,
    pub factor_base_parameters: Value,
    pub linalg: String,
    pub instance: String,
    pub subgroup_order: Option<f64>,
    pub rows: usize,
    pub producer_verified: usize,
    pub recovered: usize,
    pub exhausted: usize,
    pub full_rank_reported: usize,
    pub missing_cost_qualification: usize,
    pub missing_source_commit: usize,
    pub phase_sum_mismatch: usize,
    pub complete_cold_gae: Option<f64>,
    pub recorded_gae: Metric,
    pub row_wall_seconds: Metric,
    pub solver_ops: Metric,
    pub solver_gae: Metric,
    pub solver_calls: Metric,
    pub solver_budget_exceeded: Metric,
    pub attempts: Metric,
    pub relations: Metric,
    pub final_rank: Metric,
    pub rank_goal: Metric,
    pub signed_population: Metric,
    pub base_columns: Metric,
    pub recorded_s_over_rho: Metric,
    pub phases_gae: BTreeMap<String, Metric>,
    pub unpriced_counters: BTreeMap<String, f64>,
    pub gaps: BTreeMap<String, usize>,
    pub solver_units: BTreeSet<String>,
    pub comparison_identities: BTreeMap<String, usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceAccounting {
    pub source: String,
    pub json_lines: usize,
    pub envelopes_with_rows: usize,
    pub failed_envelopes: usize,
    pub envelopes_without_rows: usize,
    pub failed_timed_envelopes: usize,
    pub missing_output_by_stage_arm: BTreeMap<String, usize>,
    pub groups: Vec<GroupAccounting>,
}

fn number(value: &Value, path: &str) -> Option<f64> {
    value.pointer(path).and_then(Value::as_f64)
}

fn text_at(value: &Value, path: &str) -> String {
    value
        .pointer(path)
        .and_then(Value::as_str)
        .unwrap_or("unknown")
        .into()
}

fn gap(group: &mut GroupAccounting, name: &str) {
    *group.gaps.entry(name.into()).or_default() += 1;
}

fn unpriced(value: &Value, prefix: &str, output: &mut BTreeMap<String, f64>) {
    match value {
        Value::Object(map) => {
            for (key, child) in map {
                let path = format!("{prefix}/{key}");
                if key.ends_with("_uncharged") || key.ends_with("_unpriced") {
                    if let Some(amount) = child.as_f64().filter(|n| *n > 0.0) {
                        *output.entry(path.clone()).or_default() += amount;
                    }
                }
                unpriced(child, &path, output);
            }
        }
        Value::Array(rows) => {
            for (i, child) in rows.iter().enumerate() {
                unpriced(child, &format!("{prefix}/{i}"), output);
            }
        }
        _ => {}
    }
}

/// Parse every nonempty line. Malformed JSON or malformed result.rows fails
/// closed with a source/line diagnostic; it is never silently discarded.
pub fn account_jsonl(source: &str, input: &str) -> Result<SourceAccounting, String> {
    let mut groups = BTreeMap::<String, GroupAccounting>::new();
    let mut report = SourceAccounting {
        source: source.into(),
        json_lines: 0,
        envelopes_with_rows: 0,
        failed_envelopes: 0,
        envelopes_without_rows: 0,
        failed_timed_envelopes: 0,
        missing_output_by_stage_arm: BTreeMap::new(),
        groups: Vec::new(),
    };
    for (index, line) in input
        .lines()
        .enumerate()
        .filter(|(_, s)| !s.trim().is_empty())
    {
        report.json_lines += 1;
        let envelope: Value =
            serde_json::from_str(line).map_err(|e| format!("{source}:{}: {e}", index + 1))?;
        let id = envelope["id"].as_str().unwrap_or("unknown");
        let stage = if id.starts_with("timed:") {
            "timed"
        } else if id.starts_with("preflight:") {
            "preflight"
        } else {
            "other"
        };
        if envelope
            .get("exit_code")
            .and_then(Value::as_i64)
            .is_some_and(|x| x != 0)
        {
            report.failed_envelopes += 1;
            report.failed_timed_envelopes += usize::from(stage == "timed");
        }
        let result = &envelope["result"];
        if result.is_null() {
            report.envelopes_without_rows += 1;
            if stage == "timed"
                || envelope.get("command").is_some()
                || envelope.get("exit_code").is_some()
            {
                let arm = envelope["arm"].as_str().unwrap_or(id);
                *report
                    .missing_output_by_stage_arm
                    .entry(format!("{stage}:{arm}"))
                    .or_default() += 1;
            }
            continue;
        }
        let rows = result
            .get("rows")
            .and_then(Value::as_array)
            .ok_or_else(|| format!("{source}:{}: result.rows must be an array", index + 1))?;
        if rows.is_empty() {
            report.envelopes_without_rows += 1;
            let arm = envelope["arm"].as_str().unwrap_or(id);
            *report
                .missing_output_by_stage_arm
                .entry(format!("{stage}:{arm}"))
                .or_default() += 1;
        } else {
            report.envelopes_with_rows += 1;
        }
        for row in rows {
            if !row.is_object() {
                return Err(format!("{source}:{}: non-object row", index + 1));
            }
            let method = text_at(row, "/factor_base/name");
            let arm = envelope["arm"]
                .as_str()
                .map(str::to_owned)
                .unwrap_or_else(|| {
                    if id.contains("baseline") {
                        "baseline".into()
                    } else if id.contains("candidate") {
                        "candidate".into()
                    } else {
                        id.split(':').nth(1).unwrap_or("unknown").into()
                    }
                });
            let engine = row
                .pointer("/spec/oracle_params/engine")
                .and_then(Value::as_str)
                .or_else(|| row.pointer("/decomposition/name").and_then(Value::as_str))
                .unwrap_or("unknown")
                .to_owned();
            let params = row
                .pointer("/spec/factor_base_params")
                .cloned()
                .unwrap_or(Value::Null);
            let instance = text_at(row, "/instance");
            let linalg = text_at(row, "/linear_algebra/name");
            let arity = row
                .pointer("/decomposition/summands")
                .and_then(Value::as_u64);
            let order = number(row, "/r");
            let key = format!(
                "{stage}|{arm}|{method}|{engine}|{params}|{instance}|{linalg}|{arity:?}|{order:?}"
            );
            let group = groups.entry(key).or_insert_with(|| GroupAccounting {
                source: source.into(),
                stage: stage.into(),
                arm,
                method,
                engine,
                arity,
                factor_base_parameters: params,
                linalg,
                instance,
                subgroup_order: order,
                ..Default::default()
            });
            group.rows += 1;
            let identity_paths = [
                "/r",
                "/instance",
                "/decomposition/summands",
                "/linear_algebra/name",
                "/spec/seed",
                "/spec/max_trials",
                "/spec/solver_budget_seconds",
                "/spec/targets",
                "/decomposition/solver/op_unit",
            ];
            if identity_paths
                .iter()
                .all(|path| row.pointer(path).is_some_and(|v| !v.is_null()))
                && row
                    .pointer("/decomposition/solver/op_unit")
                    .and_then(Value::as_str)
                    .is_some_and(|unit| !unit.trim().is_empty())
            {
                let identity = json!(identity_paths
                    .iter()
                    .map(|path| row.pointer(path).unwrap())
                    .collect::<Vec<_>>())
                .to_string();
                *group.comparison_identities.entry(identity).or_default() += 1;
            } else {
                gap(group, "comparison_identity_incomplete");
            }
            group.producer_verified += usize::from(row["verified"] == true);
            group.recovered += usize::from(!row["recovered"].is_null());
            group.exhausted += usize::from(row["exhausted"] == true);
            group.full_rank_reported += usize::from(
                number(row, "/linear_algebra/cost/native/full_rank_reached") == Some(1.0),
            );
            if result["cost_qualification"].is_null() {
                group.missing_cost_qualification += 1;
                gap(group, "missing_cost_qualification");
            }
            if result
                .pointer("/software/git_commit")
                .and_then(Value::as_str)
                .is_none()
            {
                group.missing_source_commit += 1;
                gap(group, "producer_binary_source_commit_missing");
            }
            // This legacy adapter does not validate a cost qualification object
            // or an independent replay signature, even if an object is present.
            gap(group, "independent_complete_cost_replay_not_performed");
            let paths = [
                ("factor_base", "/factor_base/cost"),
                ("decomposition_setup", "/decomposition/setup"),
                (
                    "relation_collection_including_solver",
                    "/decomposition/cost",
                ),
                ("linear_algebra_and_rank", "/linear_algebra/cost"),
                ("recovery_verification", "/verify"),
            ];
            let mut phase_sum = 0.0;
            let mut phases_known = true;
            for (name, path) in paths {
                let phase_gae = number(row, &format!("{path}/gae"));
                group
                    .phases_gae
                    .entry(name.into())
                    .or_default()
                    .add(phase_gae);
                match phase_gae {
                    Some(v) => phase_sum += v,
                    None => phases_known = false,
                }
                if phase_gae == Some(0.0)
                    && number(row, &format!("{path}/wall_ns")).is_some_and(|n| n > 0.0)
                {
                    gap(group, &format!("{name}:positive_wall_zero_gae"));
                }
            }
            let total = number(row, "/total_gae");
            if phases_known
                && total.is_some_and(|t| (phase_sum - t).abs() > 1e-7 * t.abs().max(1.0))
            {
                group.phase_sum_mismatch += 1;
            }
            group.recorded_gae.add(total);
            group.row_wall_seconds.add(number(row, "/wall_seconds"));
            group
                .solver_ops
                .add(number(row, "/decomposition/solver/ops"));
            group
                .solver_gae
                .add(number(row, "/decomposition/solver/gae"));
            group
                .solver_calls
                .add(number(row, "/decomposition/solver/calls"));
            group
                .solver_budget_exceeded
                .add(number(row, "/decomposition/solver/budget_exceeded"));
            group
                .attempts
                .add(number(row, "/decomposition/targets_tried"));
            group
                .relations
                .add(number(row, "/decomposition/relations_found"));
            group.final_rank.add(number(row, "/linear_algebra/rank"));
            group
                .rank_goal
                .add(number(row, "/linear_algebra/cost/native/columns"));
            group
                .signed_population
                .add(number(row, "/factor_base/signed_points"));
            group.base_columns.add(number(row, "/factor_base/columns"));
            group.recorded_s_over_rho.add(number(row, "/s_over_rho"));
            if let Some(unit) = row
                .pointer("/decomposition/solver/op_unit")
                .and_then(Value::as_str)
            {
                group.solver_units.insert(unit.into());
            }
            let mut row_unpriced = BTreeMap::new();
            unpriced(row, "", &mut row_unpriced);
            if !row_unpriced.is_empty() {
                gap(group, "explicit_positive_unpriced_native_work");
            }
            for (path, amount) in row_unpriced {
                *group.unpriced_counters.entry(path).or_default() += amount;
            }
            for (counter, calibration) in [
                ("legendre_symbols", "ns_per_legendre"),
                ("sqrt_solves", "ns_per_sqrt"),
            ] {
                if number(row, &format!("/factor_base/cost/native/{counter}"))
                    .is_some_and(|n| n > 0.0)
                    && number(result, &format!("/calibration/{calibration}"))
                        .is_none_or(|n| n <= 0.0)
                {
                    gap(group, &format!("positive_{counter}_without_calibration"));
                }
            }
            // No value is assigned to complete_cold_gae. This is deliberately
            // not a certification adapter for the old recorded total_gae.
        }
    }
    report.groups = groups.into_values().collect();
    Ok(report)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DescriptiveFit {
    pub observations: usize,
    pub distinct_sizes: usize,
    pub span_bits: f64,
    pub raw_slope: f64,
    pub normalized_to_sqrt_n_slope: f64,
    pub r_squared: f64,
    pub scope: String,
}

/// OLS on log2(n), log2(recorded work). Replicate file means are observations,
/// not independent curve sizes. This never returns an asymptotic certificate.
pub fn descriptive_fit(points: &[(f64, f64)]) -> Result<DescriptiveFit, String> {
    if points.len() < 2
        || points
            .iter()
            .any(|(n, t)| !n.is_finite() || !t.is_finite() || *n <= 1.0 || *t <= 0.0)
    {
        return Err("fit needs at least two finite positive work/order points".into());
    }
    let distinct_sizes = points
        .iter()
        .map(|(n, _)| n.to_bits())
        .collect::<BTreeSet<_>>()
        .len();
    if distinct_sizes < 2 {
        return Err("fit requires distinct subgroup orders".into());
    }
    let values: Vec<_> = points.iter().map(|(n, t)| (n.log2(), t.log2())).collect();
    let len = values.len() as f64;
    let mx = values.iter().map(|(x, _)| x).sum::<f64>() / len;
    let my = values.iter().map(|(_, y)| y).sum::<f64>() / len;
    let xx = values.iter().map(|(x, _)| (x - mx).powi(2)).sum::<f64>();
    if xx == 0.0 {
        return Err("fit requires distinct subgroup orders".into());
    }
    let xy = values.iter().map(|(x, y)| (x - mx) * (y - my)).sum::<f64>();
    let yy = values.iter().map(|(_, y)| (y - my).powi(2)).sum::<f64>();
    let slope = xy / xx;
    let min = values.iter().map(|(x, _)| *x).fold(f64::INFINITY, f64::min);
    let max = values
        .iter()
        .map(|(x, _)| *x)
        .fold(f64::NEG_INFINITY, f64::max);
    Ok(DescriptiveFit { observations: values.len(),
        distinct_sizes,
        span_bits: max-min, raw_slope: slope, normalized_to_sqrt_n_slope: slope-0.5,
        r_squared: if yy == 0.0 {1.0} else {(xy*xy/(xx*yy)).clamp(0.0,1.0)},
        scope: "descriptive archived file means; heterogeneous curves/calibration; fixed nominal t; incomplete cost; no asymptotic inference".into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn missing_costs_stay_unknown_and_failed_rows_stay_in_denominator() {
        let row = json!({"r":101,"instance":"toy","factor_base":{"name":"toy","columns":3,"signed_points":6,"cost":{"gae":2}},
            "decomposition":{"name":"toy","summands":2,"setup":{"gae":0,"wall_ns":10},"cost":{"gae":7,"native":{"work_uncharged":5}},"targets_tried":10,"relations_found":4,"solver":{"ops":9,"gae":6}},
            "linear_algebra":{"name":"rank","rank":3,"cost":{"gae":1,"native":{"columns":4}}},"verify":{"gae":0},"total_gae":10,"verified":false,"exhausted":true});
        let envelope =
            json!({"id":"timed:baseline","result":{"rows":[row.clone(),row]},"exit_code":1});
        let result = account_jsonl("test", &envelope.to_string()).unwrap();
        let group = &result.groups[0];
        assert_eq!(group.rows, 2);
        assert_eq!(group.exhausted, 2);
        assert_eq!(group.producer_verified, 0);
        assert_eq!(group.recorded_gae.sum, 20.0);
        assert_eq!(group.solver_gae.sum, 12.0);
        assert_eq!(group.phase_sum_mismatch, 0); // solver not added twice
        assert!(group.complete_cold_gae.is_none());
        assert_eq!(group.gaps["explicit_positive_unpriced_native_work"], 2);
        assert_eq!(group.gaps["decomposition_setup:positive_wall_zero_gae"], 2);
        assert_eq!(group.signed_population.sum, 12.0);
        assert_eq!(group.base_columns.sum, 6.0);
        assert_eq!(result.failed_envelopes, 1);
    }

    #[test]
    fn preflights_are_separate_and_malformed_input_rejected() {
        let input = "{\"id\":\"header\"}\n{\"id\":\"preflight:a\",\"result\":{\"rows\":[{}]}}\n{\"id\":\"timed:a\",\"result\":{\"rows\":[{}]}}";
        let report = account_jsonl("test", input).unwrap();
        assert_eq!(report.groups.len(), 2);
        assert_eq!(report.envelopes_without_rows, 1);
        assert!(report
            .groups
            .iter()
            .all(|g| g.recorded_gae.complete_mean().is_none()));
        assert!(account_jsonl("test", "{\"result\":{}}").is_err());
        assert!(account_jsonl("test", "bad json")
            .unwrap_err()
            .contains("test:1"));
    }

    #[test]
    fn slope_units_and_distinct_sizes() {
        let fit = descriptive_fit(&[(16., 4.), (64., 8.), (256., 16.), (256., 16.)]).unwrap();
        assert!((fit.raw_slope - 0.5).abs() < 1e-12);
        assert!(fit.normalized_to_sqrt_n_slope.abs() < 1e-12);
        assert_eq!(fit.distinct_sizes, 3);
        assert_eq!(fit.span_bits, 4.0);
        assert!(descriptive_fit(&[(16., 4.), (16., 8.)]).is_err());
        assert!(descriptive_fit(&[(65003., 4.), (65003., 8.), (65003., 7.)]).is_err());
    }

    #[test]
    fn crashed_timed_envelopes_are_retained_without_rows() {
        let report = account_jsonl(
            "crash",
            r#"{"id":"timed:a","arm":"a","exit_code":137,"result":null}"#,
        )
        .unwrap();
        assert_eq!(report.failed_timed_envelopes, 1);
        assert_eq!(report.missing_output_by_stage_arm["timed:a"], 1);
        assert!(report.groups.is_empty());
        let unknown=account_jsonl("unknown", "{\"id\":\"timed:a\",\"arm\":\"a\"}\n{\"id\":\"timed:a\",\"arm\":\"a\",\"result\":{\"rows\":[]}}").unwrap();
        assert_eq!(unknown.missing_output_by_stage_arm["timed:a"], 2);
        assert_eq!(unknown.envelopes_with_rows, 0);
    }
}
