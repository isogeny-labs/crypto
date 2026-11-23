//! Plan-bound completeness and empirical A/A tolerance. No review certification.
use super::*;
use std::collections::BTreeSet;

fn array<'a>(v: &'a Value, key: &str) -> Result<&'a Vec<Value>, String> {
    v.get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("plan missing array {key}"))
}
fn index(v: &Value, key: &str) -> Result<usize, String> {
    let n = v
        .get(key)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("invalid plan index {key}"))?;
    usize::try_from(n).map_err(|_| format!("plan index overflow {key}"))
}
fn summary(values: &[Value]) -> Value {
    let mut present: Vec<f64> = values.iter().filter_map(Value::as_f64).collect();
    present.sort_by(f64::total_cmp);
    let n = present.len();
    let median = if n == 0 {
        None
    } else if n % 2 == 1 {
        Some(present[n / 2])
    } else {
        Some((present[n / 2 - 1] + present[n / 2]) / 2.0)
    };
    json!({"raw_values":values,"observed_count":n,"missing_count":values.len()-n,
        "median":median,"minimum":present.first(),"maximum":present.last(),
        "interpretation":"observed_values_only_not_confidence_interval"})
}
fn calibrate(rows: &[&Value], clock: &str) -> Value {
    let mut reasons = Vec::new();
    if rows.len() != 5 {
        reasons.push("requires_exactly_five_expected_measured_rounds".to_owned());
    }
    let metric = format!("{clock}_diagnostic_ratio");
    let mut raw = Vec::new();
    for row in rows {
        raw.push(row[&metric].clone());
        let round = &row["round"];
        if row[&metric].as_f64().is_none() {
            reasons.push(format!("round_{round}:ineligible_AA_pair"));
            continue;
        }
        let a = &row["numerator"];
        let b = &row["denominator"];
        if a["method"].is_null() || a["method"] != b["method"] {
            reasons.push(format!("round_{round}:AA_methods_not_identical"));
        }
        let path = if clock == "cold" {
            "/method_call_wall_ns"
        } else {
            "/online/wall_ns"
        };
        match (number(a, path), number(b, path)) {
            (Some(n), Some(d)) if n > 0 && d > 0 => {
                // Exact inclusive [9/10,11/10]; do not compare rounded floats.
                if u128::from(n) * 10 < u128::from(d) * 9 || u128::from(n) * 10 > u128::from(d) * 11
                {
                    reasons.push(format!("round_{round}:outside_operational_tolerance"));
                }
            }
            _ => reasons.push(format!("round_{round}:nonpositive_or_missing_AA_clock")),
        }
    }
    json!({"rule":"operational-tolerance/v1","ratio_direction":"AA_first_over_AA_second",
        "interval_inclusive":[0.90,1.10],"calibration_passed":reasons.is_empty(),
        "reasons":reasons,"observations":summary(&raw),"statistical_interpretation":"empirical_tolerance_not_confidence_interval"})
}

/// Validate every supplied record against the immutable ecbench plan. Missing
/// planned measured records remain explicit null pairs, including empty input.
/// The plan's spec_id is recorded, not independently recomputed without its spec.
pub fn export_with_plan(input: &str, labels: [&str; 4], plan_text: &str) -> Result<Value, String> {
    let doc: Value = serde_json::from_str(plan_text).map_err(|e| format!("plan: {e}"))?;
    if string(&doc, "/schema")? != "ecbench.plan/v1" {
        return Err("unsupported plan schema".into());
    }
    let spec_id = string(&doc, "/spec_id")?;
    let arms = array(&doc, "arms")?;
    let workloads = array(&doc, "workloads")?;
    let executions = array(&doc, "executions")?;
    let mut arm_names = BTreeSet::new();
    for arm in arms {
        if !arm_names.insert(string(arm, "/name")?) {
            return Err("duplicate plan arm".into());
        }
    }
    for label in labels {
        if !arm_names.contains(label) {
            return Err(format!("selected arm {label} absent from plan"));
        }
    }
    let mut ids = BTreeSet::new();
    for w in workloads {
        if !ids.insert(string(w, "/workload_id")?) {
            return Err("duplicate plan workload identity".into());
        }
    }
    let mut by_seq = BTreeMap::new();
    let mut seen = BTreeSet::new();
    let mut expected: BTreeMap<(String, u64), (String, BTreeSet<String>)> = BTreeMap::new();
    for e in executions {
        let wi = index(e, "workload")?;
        let ai = index(e, "arm")?;
        let w = workloads
            .get(wi)
            .ok_or("plan workload index out of range")?;
        let a = arms.get(ai).ok_or("plan arm index out of range")?;
        let name = string(a, "/name")?;
        let wid = string(w, "/workload_id")?;
        let round = number(e, "/round").ok_or("plan missing round")?;
        let seq = number(e, "/seq").ok_or("plan missing seq")?;
        let seed = number(e, "/algorithm_seed")
            .ok_or("plan invalid numeric algorithm_seed")?
            .to_string();
        let warmup = e
            .get("warmup")
            .and_then(Value::as_bool)
            .ok_or("plan missing warmup")?;
        if by_seq.insert(seq, (e, w, a)).is_some() {
            return Err("duplicate plan seq".into());
        }
        if !seen.insert((wid, round, name)) {
            return Err("duplicate plan arm/workload/round".into());
        }
        if !warmup && labels.contains(&name) {
            let g = expected
                .entry((wid.to_owned(), round))
                .or_insert_with(|| (seed.clone(), BTreeSet::new()));
            if g.0 != seed {
                return Err("plan paired seeds differ".into());
            }
            g.1.insert(name.to_owned());
        }
    }
    let selected: BTreeSet<String> = labels.iter().map(|s| s.to_string()).collect();
    for (_, names) in expected.values() {
        if names != &selected {
            return Err("plan missing selected arm from measured round".into());
        }
    }
    for wid in &ids {
        if !expected.keys().any(|(expected_id, _)| expected_id == wid) {
            return Err("plan workload has no selected measured executions".into());
        }
    }
    let mut observed_seq = BTreeSet::new();
    for line in input.lines().filter(|s| !s.trim().is_empty()) {
        let r: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
        let seq = number(&r, "/seq").ok_or("record missing seq for plan binding")?;
        if !observed_seq.insert(seq) {
            return Err("duplicate record seq".into());
        }
        let (e, w, a) = by_seq.get(&seq).ok_or("record seq absent from plan")?;
        if r.get("workload") != Some(*w)
            || r.get("method") != a.get("method")
            || r.get("role") != a.get("role")
            || r.get("arm") != a.get("name")
            || r.get("round") != e.get("round")
            || r.get("warmup") != e.get("warmup")
            || string(&r, "/algorithm_seed")? != number(e, "/algorithm_seed").unwrap().to_string()
        {
            return Err(format!(
                "record seq {seq} identity/seed/arm differs from plan"
            ));
        }
    }
    let mut out = export_core(input, labels, true)?;
    let pairs = out["pairs"].as_array_mut().unwrap();
    let existing: BTreeSet<(String, u64)> = pairs
        .iter()
        .map(|p| {
            (
                p["workload_id"].as_str().unwrap().to_owned(),
                p["round"].as_u64().unwrap(),
            )
        })
        .collect();
    for ((wid, round), (seed, _)) in &expected {
        if !existing.contains(&(wid.clone(), *round)) {
            pairs.push(pair(
                "candidate_reference",
                wid,
                *round,
                seed,
                (labels[0], labels[1]),
                &BTreeMap::new(),
            ));
            pairs.push(pair(
                "AA",
                wid,
                *round,
                seed,
                (labels[2], labels[3]),
                &BTreeMap::new(),
            ));
        }
    }
    pairs.sort_by(|a, b| {
        (
            a["workload_id"].as_str(),
            a["round"].as_u64(),
            a["comparison"].as_str(),
        )
            .cmp(&(
                b["workload_id"].as_str(),
                b["round"].as_u64(),
                b["comparison"].as_str(),
            ))
    });
    let mut targets = Vec::new();
    for wid in expected.keys().map(|(wid, _)| wid).collect::<BTreeSet<_>>() {
        let rows: Vec<_> = pairs
            .iter()
            .filter(|p| p["workload_id"] == wid.as_str())
            .collect();
        let aa: Vec<_> = rows
            .iter()
            .copied()
            .filter(|p| p["comparison"] == "AA")
            .collect();
        let candidate: Vec<_> = rows
            .iter()
            .copied()
            .filter(|p| p["comparison"] == "candidate_reference")
            .collect();
        let mut target = json!({"workload_id":wid,"expected_measured_rounds":aa.iter().map(|p|p["round"].clone()).collect::<Vec<_>>()});
        for clock in ["cold", "online"] {
            let metric = format!("{clock}_diagnostic_ratio");
            target[format!("{clock}_AA")] = calibrate(&aa, clock);
            target[format!("{clock}_candidate_reference")] = summary(
                &candidate
                    .iter()
                    .map(|p| p[&metric].clone())
                    .collect::<Vec<_>>(),
            );
            for (name, side_name) in [("candidate", "numerator"), ("reference", "denominator")] {
                let path = if clock == "cold" {
                    "/method_call_wall_ns"
                } else {
                    "/online/wall_ns"
                };
                target[format!("{clock}_{name}_time_ns")] = summary(
                    &candidate
                        .iter()
                        .map(|p| field(p.get(side_name), path))
                        .collect::<Vec<_>>(),
                );
            }
        }
        targets.push(target);
    }
    for row in pairs.iter_mut() {
        let target = targets
            .iter()
            .find(|t| t["workload_id"] == row["workload_id"])
            .unwrap();
        for clock in ["cold", "online"] {
            let calibration = &target[format!("{clock}_AA")];
            row[format!("{clock}_AA_calibration_passed")] =
                calibration["calibration_passed"].clone();
            row[format!("{clock}_AA_calibration_reasons")] = calibration["reasons"].clone();
        }
    }
    out["expected_measured_target_count"] = json!(targets.len());
    out["target_summaries"] = json!(targets);
    out["plan_binding"] = json!({"schema":"ecbench.plan/v1","spec_id":spec_id,
        "plan_sha256":format!("{:x}",Sha256::digest(plan_text.as_bytes())),
        "expected_execution_count":executions.len(),"observed_execution_count":observed_seq.len(),
        "missing_execution_seqs":by_seq.keys().filter(|s|!observed_seq.contains(s)).copied().collect::<Vec<_>>(),
        "status":"record_fields_matched_plan_not_independently_reviewed"});
    out["AA_review_status"] = json!("empirical_calibration_computed_independent_review_pending");
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    const LABELS: [&str; 4] = ["candidate", "rho", "rho_AA", "rho"];
    // Schema field projections for planner/export logic, not sealed run evidence.
    fn fixture() -> (Value, Vec<Value>) {
        let arms = vec![
            json!({"name":"candidate","role":"candidate","method":{"method_id":"M-candidate"}}),
            json!({"name":"rho","role":"reference","method":{"method_id":"M-rho"}}),
            json!({"name":"rho_AA","role":"control","method":{"method_id":"M-rho"}}),
        ];
        let workloads = vec![
            json!({"workload_id":"W-first","workload_sha256":"first"}),
            json!({"workload_id":"W-second","workload_sha256":"second"}),
        ];
        let mut executions = Vec::new();
        let mut records = Vec::new();
        for round in 0..6u64 {
            for (wi, w) in workloads.iter().enumerate() {
                for (ai, a) in arms.iter().enumerate() {
                    let seq = executions.len() as u64;
                    executions.push(json!({"seq":seq,"round":round,"warmup":round==0,"arm":ai,"workload":wi,"algorithm_seed":100+round}));
                    records.push(json!({"schema":"ecbench.record/v1","record_id":format!("projection-{seq}"),"session_id":"S",
                        "seq":seq,"round":round,"warmup":round==0,"arm":a["name"],"role":a["role"],"method":a["method"],"workload":w,
                        "algorithm_seed":(100+round).to_string(),"outcome":{"status":"verified","matches_target":true,"matches_planted":true,"error":null},
                        "time":{"method_call_wall_ns":100,"online_clock_diagnostics":{"error":null}},"isolation":{"level":"L2"},
                        "online":{"wall_ns":100,"phases_ns":{"rho_solve":100}},"detail":{}}));
                }
            }
        }
        (
            json!({"schema":"ecbench.plan/v1","spec_id":"fixture-spec","arms":arms,"workloads":workloads,"executions":executions}),
            records,
        )
    }
    fn run(p: &Value, r: &[Value]) -> Value {
        export_with_plan(
            &r.iter()
                .map(Value::to_string)
                .collect::<Vec<_>>()
                .join("\n"),
            LABELS,
            &p.to_string(),
        )
        .unwrap()
    }
    #[test]
    fn missing_whole_workload_and_empty_input_emit_expected_null_rows() {
        let (p, mut r) = fixture();
        r.retain(|r| r["workload"]["workload_id"] == "W-first");
        let out = run(&p, &r);
        assert_eq!(out["pairs"].as_array().unwrap().len(), 20);
        assert_eq!(out["expected_measured_target_count"], 2);
        assert_eq!(
            out["target_summaries"][0]["cold_AA"]["calibration_passed"],
            true
        );
        assert_eq!(
            out["target_summaries"][1]["cold_AA"]["calibration_passed"],
            false
        );
        assert_eq!(
            out["target_summaries"][1]["cold_AA"]["observations"]["missing_count"],
            5
        );
        assert_eq!(run(&p, &[])["pairs"].as_array().unwrap().len(), 20);
    }
    #[test]
    fn plan_keeps_absolute_measured_rounds_and_excludes_warmup() {
        let (p, mut r) = fixture();
        r.reverse();
        let out = run(&p, &r);
        assert_eq!(
            out["target_summaries"][0]["expected_measured_rounds"],
            json!([1, 2, 3, 4, 5])
        );
        assert_eq!(
            out["target_summaries"][0]["cold_AA"]["observations"]["raw_values"],
            json!([1.0, 1.0, 1.0, 1.0, 1.0])
        );
        assert_eq!(out["plan_binding"]["missing_execution_seqs"], json!([]));
        assert!(out["pairs"]
            .as_array()
            .unwrap()
            .iter()
            .all(|p| p["qualified_cold_ratio"].is_null()));
    }
    #[test]
    fn exact_inclusive_tolerance_boundaries_and_separate_clock_failures() {
        let (p, r) = fixture();
        for (ns, pass) in [(89, false), (90, true), (110, true), (111, false)] {
            let mut r = r.clone();
            for row in &mut r {
                if row["arm"] == "rho_AA" {
                    row["time"]["method_call_wall_ns"] = json!(ns);
                }
            }
            let out = run(&p, &r);
            assert_eq!(
                out["target_summaries"][0]["cold_AA"]["calibration_passed"],
                pass
            );
            assert_eq!(
                out["target_summaries"][0]["online_AA"]["calibration_passed"],
                true
            );
        }
        let mut r = r;
        for row in &mut r {
            if row["arm"] == "rho_AA" && row["round"] == 2 {
                row["time"]["online_clock_diagnostics"]["error"] = json!("bad clock");
            }
        }
        let out = run(&p, &r);
        assert_eq!(
            out["target_summaries"][0]["cold_AA"]["calibration_passed"],
            true
        );
        assert_eq!(
            out["target_summaries"][0]["online_AA"]["calibration_passed"],
            false
        );
    }
    #[test]
    fn plan_rejects_changed_identity_seed_arm_and_warmup() {
        let (p, r) = fixture();
        for pointer in [
            "/workload/workload_id",
            "/method/method_id",
            "/algorithm_seed",
            "/arm",
        ] {
            let mut altered = r.clone();
            *altered[0].pointer_mut(pointer).unwrap() = json!("different");
            assert!(export_with_plan(
                &altered
                    .iter()
                    .map(Value::to_string)
                    .collect::<Vec<_>>()
                    .join("\n"),
                LABELS,
                &p.to_string()
            )
            .is_err());
        }
        let mut altered = r;
        altered[0]["warmup"] = json!(false);
        assert!(export_with_plan(
            &altered
                .iter()
                .map(Value::to_string)
                .collect::<Vec<_>>()
                .join("\n"),
            LABELS,
            &p.to_string()
        )
        .is_err());
    }
}
