//! Diagnostic projection of ecbench.record/v1. Never certifies a run or A/A test.
mod plan;
pub use plan::export_with_plan;

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

fn string<'a>(v: &'a Value, pointer: &str) -> Result<&'a str, String> {
    v.pointer(pointer)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing/string field {pointer}"))
}
fn number(v: &Value, pointer: &str) -> Option<u64> {
    v.pointer(pointer).and_then(Value::as_u64)
}
fn field(v: Option<&Value>, pointer: &str) -> Value {
    v.and_then(|v| v.pointer(pointer))
        .cloned()
        .unwrap_or(Value::Null)
}
fn verified(r: &Value) -> bool {
    r.pointer("/outcome/status").and_then(Value::as_str) == Some("verified")
        && r.pointer("/outcome/matches_target") == Some(&Value::Bool(true))
        && r.pointer("/outcome/matches_planted") != Some(&Value::Bool(false))
        && r.pointer("/outcome/error").is_some_and(Value::is_null)
}
fn isolated(r: &Value) -> bool {
    matches!(
        r.pointer("/isolation/level").and_then(Value::as_str),
        Some("L2" | "L3")
    )
}
fn online(r: &Value) -> Option<u64> {
    let wall = number(r, "/online/wall_ns")?;
    let phases = r.pointer("/online/phases_ns")?.as_object()?;
    if phases.is_empty() {
        return None;
    }
    let sum = phases
        .values()
        .try_fold(0u64, |sum, n| sum.checked_add(n.as_u64()?))?;
    (sum == wall).then_some(wall)
}
fn ratio(a: Option<u64>, b: Option<u64>) -> Option<f64> {
    match (a, b) {
        (Some(a), Some(b)) if b > 0 => Some(a as f64 / b as f64),
        _ => None,
    }
}
fn side(r: Option<&Value>) -> Value {
    json!({
        "record_id":field(r,"/record_id"), "method":field(r,"/method"),
        "outcome":field(r,"/outcome"), "isolation_level":field(r,"/isolation/level"),
        "method_call_wall_ns":field(r,"/time/method_call_wall_ns"),
        "max_rss_kib":field(r,"/time/max_rss_kib"),
        "parent_audit_verification_ns":field(r,"/time/parent_audit_verification_ns"),
        "online":field(r,"/online"),
        "online_error_observation_present":r.is_some_and(|v|v.pointer("/time/online_clock_diagnostics/error").is_some()),
        "online_error":field(r,"/time/online_clock_diagnostics/error"),
        "detail":field(r,"/detail"), "cost_observation_not_full_price":field(r,"/cost")
    })
}
fn pair(
    kind: &str,
    workload: &str,
    round: u64,
    seed: &str,
    labels: (&str, &str),
    rows: &BTreeMap<String, Value>,
) -> Value {
    let a = rows.get(labels.0);
    let b = rows.get(labels.1);
    let mut cold_reasons = Vec::new();
    let mut online_reasons = Vec::new();
    for (name, r) in [("numerator", a), ("denominator", b)] {
        match r {
            None => cold_reasons.push(format!("{name}:missing_partner")),
            Some(r) => {
                if !verified(r) {
                    cold_reasons.push(format!("{name}:not_verified"));
                }
                if !isolated(r) {
                    cold_reasons.push(format!("{name}:isolation_below_L2_or_missing"));
                }
                if number(r, "/time/method_call_wall_ns").is_none() {
                    cold_reasons.push(format!("{name}:cold_timer_missing"));
                }
            }
        }
        match r {
            None => online_reasons.push(format!("{name}:missing_partner")),
            Some(r) => {
                if !verified(r) {
                    online_reasons.push(format!("{name}:not_verified"));
                }
                if !isolated(r) {
                    online_reasons.push(format!("{name}:isolation_below_L2_or_missing"));
                }
                if online(r).is_none() {
                    online_reasons.push(format!("{name}:online_missing_or_invalid_phase_sum"));
                }
                match r.pointer("/time/online_clock_diagnostics/error") {
                    None => online_reasons.push(format!("{name}:online_error_not_exported")),
                    Some(Value::Null) => {}
                    Some(_) => online_reasons.push(format!("{name}:online_error")),
                }
            }
        }
    }
    let cold_a = a.and_then(|v| number(v, "/time/method_call_wall_ns"));
    let cold_b = b.and_then(|v| number(v, "/time/method_call_wall_ns"));
    if cold_b == Some(0) {
        cold_reasons.push("denominator:zero_cold_timer".into());
    }
    let online_a = a.and_then(online);
    let online_b = b.and_then(online);
    if online_b == Some(0) {
        online_reasons.push("denominator:zero_online_timer".into());
    }
    json!({"comparison":kind,"workload_id":workload,"round":round,"algorithm_seed":seed,
        "numerator_arm":labels.0,"denominator_arm":labels.1,"numerator":side(a),"denominator":side(b),
        "cold_diagnostic_ratio":if cold_reasons.is_empty(){ratio(cold_a,cold_b)}else{None},
        "online_diagnostic_ratio":if online_reasons.is_empty(){ratio(online_a,online_b)}else{None},
        "cold_diagnostic_blockers":cold_reasons,"online_diagnostic_blockers":online_reasons,
        "qualified_cold_ratio":Value::Null,"qualified_online_ratio":Value::Null,
        "qualification_blockers":["validated_protocol_bound_AA_review_not_supplied","input_record_seals_and_replay_not_validated"],
        "parent_audit_status":"unqualified_parent_clock_not_added_to_child_timers",
        "memory_status":"independent_process_max_rss_observation_not_added_to_time"})
}

/// Four roles: candidate, reference, first A/A arm, second A/A arm.
/// Each pair must use distinct labels; the reference may also be an A/A side.
/// Input is exactly one session. Unselected arms and warm-ups are counted but not paired.
pub fn export(input: &str, labels: [&str; 4]) -> Result<Value, String> {
    export_core(input, labels, false)
}

fn export_core(input: &str, labels: [&str; 4], allow_empty: bool) -> Result<Value, String> {
    if labels.iter().any(|s| s.is_empty()) || labels[0] == labels[1] || labels[2] == labels[3] {
        return Err("require nonempty labels and distinct arms within each comparison".into());
    }
    let mut session = None;
    let mut ignored = 0usize;
    let mut records = 0usize;
    let mut groups: BTreeMap<(String, u64), (String, BTreeMap<String, Value>)> = BTreeMap::new();
    for (line_no, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let r: Value =
            serde_json::from_str(line).map_err(|e| format!("line {}: {e}", line_no + 1))?;
        if string(&r, "/schema")? != "ecbench.record/v1" {
            return Err("unsupported record schema".into());
        }
        let sid = string(&r, "/session_id")?.to_owned();
        if session.as_ref().is_some_and(|old| old != &sid) {
            return Err("mixed sessions".into());
        }
        session = Some(sid);
        records += 1;
        let warmup = r
            .get("warmup")
            .and_then(Value::as_bool)
            .ok_or("missing warmup flag")?;
        let arm = string(&r, "/arm")?.to_owned();
        if warmup || !labels.contains(&arm.as_str()) {
            ignored += 1;
            continue;
        }
        let workload = string(&r, "/workload/workload_id")?.to_owned();
        let round = number(&r, "/round").ok_or("missing round")?;
        let seed = string(&r, "/algorithm_seed")?.to_owned();
        let parsed = seed
            .parse::<u64>()
            .map_err(|_| "algorithm_seed is not u64 decimal")?;
        if parsed.to_string() != seed {
            return Err("noncanonical algorithm_seed".into());
        }
        let group = groups
            .entry((workload, round))
            .or_insert_with(|| (seed.clone(), BTreeMap::new()));
        if group.0 != seed {
            return Err("mismatched seeds within workload/round".into());
        }
        if group.1.insert(arm, r).is_some() {
            return Err("duplicate arm within workload/round".into());
        }
    }
    if records == 0 && !allow_empty {
        return Err("empty records input".into());
    }
    let mut pairs = Vec::new();
    for ((workload, round), (seed, rows)) in &groups {
        pairs.push(pair(
            "candidate_reference",
            workload,
            *round,
            seed,
            (labels[0], labels[1]),
            rows,
        ));
        pairs.push(pair(
            "AA",
            workload,
            *round,
            seed,
            (labels[2], labels[3]),
            rows,
        ));
    }
    Ok(
        json!({"schema":"pencil-paired-diagnostics/v1","status":"unqualified_diagnostics_only",
        "input_sha256":format!("{:x}",Sha256::digest(input.as_bytes())),"session_id":session,
        "arm_labels":labels,"input_record_count":records,"ignored_warmup_or_other_arm_count":ignored,
        "AA_review_status":"not_evaluated_no_protocol_or_validated_review_binding",
        "full_price_ratio":Value::Null,"pairs":pairs}),
    )
}

/// Flat CSV with structured observations quoted as JSON cells. Null is literal `null`.
pub fn csv(bundle: &Value) -> String {
    let headers = [
        "comparison",
        "workload_id",
        "round",
        "algorithm_seed",
        "numerator_arm",
        "denominator_arm",
        "cold_diagnostic_ratio",
        "online_diagnostic_ratio",
        "qualified_cold_ratio",
        "qualified_online_ratio",
        "cold_AA_calibration_passed",
        "online_AA_calibration_passed",
        "cold_AA_calibration_reasons",
        "online_AA_calibration_reasons",
        "cold_diagnostic_blockers",
        "online_diagnostic_blockers",
        "qualification_blockers",
        "numerator",
        "denominator",
    ];
    let mut out = headers.join(",");
    out.push('\n');
    for row in bundle["pairs"].as_array().into_iter().flatten() {
        let cells: Vec<_> = headers
            .iter()
            .map(|key| {
                let v = &row[*key];
                let s = v
                    .as_str()
                    .map(str::to_owned)
                    .unwrap_or_else(|| v.to_string());
                format!("\"{}\"", s.replace('"', "\"\""))
            })
            .collect();
        out.push_str(&cells.join(","));
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    const LABELS: [&str; 4] = ["candidate", "rho", "aa_first", "aa_second"];
    // A field projection, not a sealed Record: keys and values follow Record,
    // Outcome, Timing and OnlineWindow. Irrelevant fields deliberately omitted.
    fn record(arm: &str) -> Value {
        json!({"schema":"ecbench.record/v1","record_id":format!("projection-{arm}"),
            "session_id":"S","round":1,"warmup":false,"arm":arm,
            "algorithm_seed":"13703248144867106833","workload":{"workload_id":"Wcf52db405c0d"},
            "outcome":{"status":"verified","matches_target":true,"matches_planted":true,"error":null},
            "time":{"method_call_wall_ns":20,"max_rss_kib":null,"parent_audit_verification_ns":4},
            "isolation":{"level":"L2"},
            "online":{"wall_ns":10,"phases_ns":{"target_query":6,"target_PDP":4},"zero_phases":{},"start_event":"start","stop_event":"stop","included_stages":["target_query","target_PDP"],"mapping":"fixture"},
            "cost":{"lower_bound":true,"total_gae":1,"unpriced":["native"]},"detail":{}})
    }
    fn input(rows: &[Value]) -> String {
        rows.iter()
            .map(Value::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    }
    fn run(rows: &[Value]) -> Value {
        export(&input(rows), LABELS).unwrap()
    }
    #[test]
    fn shuffled_rows_pair_by_identity_and_keep_unqualified_costs() {
        let mut a = record("candidate");
        a["time"]["method_call_wall_ns"] = json!(10);
        let b = record("rho");
        let x = record("aa_first");
        let y = record("aa_second");
        let one = run(&[a.clone(), b.clone(), x.clone(), y.clone()]);
        let two = run(&[y, x, b, a]);
        assert_eq!(one["pairs"], two["pairs"]);
        let pair = &one["pairs"][0];
        assert_eq!(pair["cold_diagnostic_ratio"], 0.5);
        assert!(pair["qualified_cold_ratio"].is_null());
        assert!(pair["qualified_online_ratio"].is_null());
        assert!(pair["online_diagnostic_ratio"].is_null()); // Historical row lacks online clock diagnostics.
        assert!(pair["numerator"]["max_rss_kib"].is_null());
        assert_eq!(pair["numerator"]["parent_audit_verification_ns"], 4);
        assert!(one["full_price_ratio"].is_null());
        assert!(csv(&one).contains("\"null\""));
    }
    #[test]
    fn three_arm_shared_reference_pairs_once_and_ignores_input_order() {
        let labels = ["candidate", "rho", "rho", "rho_AA"];
        let a = record("candidate");
        let r = record("rho");
        let aa = record("rho_AA");
        let first = export(&input(&[a.clone(), r.clone(), aa.clone()]), labels).unwrap();
        let shuffled = export(&input(&[aa, r.clone(), a]), labels).unwrap();
        assert_eq!(first["pairs"], shuffled["pairs"]);
        assert_eq!(first["input_record_count"], 3);
        assert_eq!(first["pairs"][0]["cold_diagnostic_ratio"], 1.0);
        assert_eq!(first["pairs"][1]["cold_diagnostic_ratio"], 1.0);
        assert_eq!(
            first["pairs"][0]["denominator"]["record_id"],
            first["pairs"][1]["numerator"]["record_id"]
        );
        assert_eq!(first["status"], "unqualified_diagnostics_only");
        assert!(export(&input(&[r.clone(), r]), labels)
            .unwrap_err()
            .contains("duplicate"));
        assert!(export(&input(&[record("rho")]), ["rho", "rho", "rho", "rho_AA"]).is_err());
        assert!(export(&input(&[record("rho")]), ["candidate", "rho", "rho", "rho"]).is_err());
    }

    #[test]
    fn missing_reference_and_absent_aa_are_retained() {
        let out = run(&[record("candidate")]);
        assert_eq!(out["pairs"].as_array().unwrap().len(), 2);
        assert!(out["pairs"][0]["cold_diagnostic_ratio"].is_null());
        assert!(out["pairs"][1]["numerator"]["record_id"].is_null());
        assert_eq!(
            out["AA_review_status"],
            "not_evaluated_no_protocol_or_validated_review_binding"
        );
    }
    #[test]
    fn failures_caps_and_missing_isolation_have_no_ratios() {
        for status in ["exhausted", "timeout", "error", "wrong_answer", "crashed"] {
            let mut a = record("candidate");
            a["outcome"]["status"] = json!(status);
            a["detail"] = json!({"candidate":{"failure":"walk_cap"}});
            let out = run(&[a, record("rho")]);
            assert!(out["pairs"][0]["cold_diagnostic_ratio"].is_null());
            assert_eq!(
                out["pairs"][0]["numerator"]["detail"]["candidate"]["failure"],
                "walk_cap"
            );
        }
        let mut b = record("rho");
        b.as_object_mut().unwrap().remove("isolation");
        assert!(run(&[record("candidate"), b])["pairs"][0]["cold_diagnostic_ratio"].is_null());
    }
    #[test]
    fn duplicates_seed_mismatch_and_mixed_sessions_are_errors() {
        assert!(export(&input(&[record("candidate"), record("candidate")]), LABELS).is_err());
        let mut b = record("rho");
        b["algorithm_seed"] = json!("7");
        assert!(export(&input(&[record("candidate"), b]), LABELS)
            .unwrap_err()
            .contains("mismatched seeds"));
        let mut b = record("rho");
        b["session_id"] = json!("other");
        assert!(export(&input(&[record("candidate"), b]), LABELS).is_err());
    }
    #[test]
    fn zero_denominator_and_bad_online_sums_are_not_ratios() {
        let mut b = record("rho");
        b["time"]["method_call_wall_ns"] = json!(0);
        assert!(run(&[record("candidate"), b])["pairs"][0]["cold_diagnostic_ratio"].is_null());
        let mut a = record("candidate");
        a["online"]["phases_ns"]["target_PDP"] = json!(5);
        assert!(online(&a).is_none());
        a["online"]["phases_ns"] = json!({"one":u64::MAX,"two":1});
        assert!(online(&a).is_none());
        assert_eq!(ratio(Some(1), Some(0)), None);
    }
    #[test]
    fn online_requires_explicit_clean_diagnostics_and_exact_sum() {
        let mut a = record("candidate");
        let mut b = record("rho");
        a["time"]["online_clock_diagnostics"] = json!({"error":null});
        b["time"]["online_clock_diagnostics"] = json!({"error":null});
        assert_eq!(
            run(&[a.clone(), b.clone()])["pairs"][0]["online_diagnostic_ratio"],
            1.0
        );
        b["time"]["online_clock_diagnostics"]["error"] = json!("scope mismatch");
        assert!(run(&[a.clone(), b.clone()])["pairs"][0]["online_diagnostic_ratio"].is_null());
        b["time"]["online_clock_diagnostics"]["error"] = Value::Null;
        b["online"]["phases_ns"]["target_PDP"] = json!(5);
        assert!(run(&[a, b])["pairs"][0]["online_diagnostic_ratio"].is_null());
    }

    #[test]
    fn warmups_ignored_and_missing_timer_never_substitutes_solve_timer() {
        let mut warmup = record("candidate");
        warmup["warmup"] = json!(true);
        let mut a = record("candidate");
        a["time"]
            .as_object_mut()
            .unwrap()
            .remove("method_call_wall_ns");
        a["time"]["solve_wall_ns"] = json!(10);
        let out = run(&[warmup, a, record("rho")]);
        assert_eq!(out["ignored_warmup_or_other_arm_count"], 1);
        assert!(out["pairs"][0]["cold_diagnostic_ratio"].is_null());
    }
}
