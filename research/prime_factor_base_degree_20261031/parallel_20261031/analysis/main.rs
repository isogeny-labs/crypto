//! Frozen exact-count analysis; no cryptographic search or timing inference.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

fn read(path: &Path) -> Result<Value, String> {
    let b = fs::read(path).map_err(|e| e.to_string())?;
    if b.len() > 32 * 1024 * 1024 {
        return Err("analysis JSON exceeds32MiB".into());
    }
    serde_json::from_slice(&b).map_err(|e| e.to_string())
}
fn digest(path: &Path) -> Result<String, String> {
    Ok(format!(
        "{:x}",
        Sha256::digest(fs::read(path).map_err(|e| e.to_string())?)
    ))
}
fn walk(path: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let mut entries = fs::read_dir(path)
        .map_err(|e| e.to_string())?
        .map(|e| e.map(|e| e.path()).map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort();
    for p in entries {
        if p.is_symlink() {
            return Err("symlink in raw record tree".into());
        }
        if p.is_dir() {
            walk(&p, out)?;
        } else if p.extension().is_some_and(|e| e == "json") {
            out.push(p);
        }
    }
    Ok(())
}
fn key(v: &Value) -> String {
    serde_json::to_string(v).expect("JSON value")
}
fn curve_key(r: &Value) -> Value {
    json!({"p":r["p"],"curve":r["curve"],"d":r["d"]})
}
fn target_key(r: &Value) -> String {
    key(&json!({"curve":curve_key(r),"target":r["target"],"base_x":r["base_x"],"arm":r["arm"]}))
}
fn max_field(rows: &[Value], field: &str) -> Option<u64> {
    if rows.iter().any(|r| r[field].as_u64().is_none()) {
        None
    } else {
        rows.iter().filter_map(|r| r[field].as_u64()).max()
    }
}
fn target_scores(records: &[Value]) -> Result<Vec<Value>, String> {
    let mut groups: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    let mut ids = BTreeSet::new();
    for r in records {
        let id = r["id"].as_str().ok_or("case id absent")?;
        if !ids.insert(id) {
            return Err(format!("duplicate case id {id}"));
        }
        if r["target"].as_array().is_none_or(|t| t.len() != 2) || r["base_x"].as_array().is_none() {
            return Err("explicit target/base metadata missing".into());
        }
        groups.entry(target_key(r)).or_default().push(r.clone());
    }
    let mut scores = Vec::new();
    for rows in groups.values() {
        let r = &rows[0];
        let arm = r["arm"].as_str().ok_or("arm absent")?;
        let signed = arm.ends_with("-secant");
        let expected = if signed { 4 } else { 1 };
        let signs: BTreeSet<_> = rows.iter().map(|r| key(&r["signs"])).collect();
        let complete_signs: BTreeSet<_> = [[1, 1], [1, -1], [-1, 1], [-1, -1]]
            .into_iter()
            .map(|s| key(&json!(s)))
            .collect();
        let complete = rows.len() == expected
            && (!signed || signs == complete_signs)
            && rows.iter().all(|r| {
                r["status"] == "verified"
                    && r["false_positives"] == 0
                    && r["false_negatives"] == 0
                    && r["every_reported_relation_group_replayed"] == true
                    && r["gb_checks"]["input_generator_remainders_zero"] == true
                    && r["gb_checks"]["buchberger_criterion"] == true
            });
        let relations = if complete {
            rows.iter()
                .map(|r| r["ordered_signed_relation_count"].as_u64())
                .collect::<Option<Vec<_>>>()
                .map(|v| v.iter().sum::<u64>())
        } else {
            None
        };
        scores.push(json!({"curve":curve_key(r),"target":r["target"],"base_x":r["base_x"],"fb_id":r["fb_id"],"arm":arm,"control_name":r["control_name"],"complete":complete,"expected_case_count":expected,"observed_case_count":rows.len(),"verified_case_count":rows.iter().filter(|r|r["status"]=="verified").count(),"maximum_input_degree":max_field(rows,"maximum_input_degree"),"maximum_selected_pair_degree":max_field(rows,"maximum_f4_selected_pair_degree"),"maximum_handled_degree":if complete{max_field(rows,"observed_f4_degree")}else{None},"peak_trace_columns":if complete{max_field(rows,"peak_matrix_columns_in_trace")}else{None},"peak_trace_rows":if complete{max_field(rows,"peak_matrix_rows_in_trace")}else{None},"ordered_relations":relations,"sat":relations.map(|n|n>0),"complete_field_operation_cost_per_relation":null,"clock_speedup":null}));
    }
    Ok(scores)
}
fn comparison(a: &Value, b: &Value, kind: &str) -> Value {
    let da = a["maximum_handled_degree"].as_u64();
    let db = b["maximum_handled_degree"].as_u64();
    let sat = match (a["sat"].as_bool(), b["sat"].as_bool()) {
        (Some(true), Some(true)) => "common-SAT",
        (Some(false), Some(false)) => "common-UNSAT",
        (Some(true), Some(false)) => "candidate-only-SAT",
        (Some(false), Some(true)) => "control-only-SAT",
        _ => "incomplete",
    };
    let outcome = match (da, db) {
        (Some(x), Some(y)) if x < y => "candidate-lower",
        (Some(x), Some(y)) if x == y => "equal",
        (Some(_), Some(_)) => "candidate-higher",
        _ => "unknown",
    };
    json!({"comparison":kind,"curve":a["curve"],"target":a["target"],"candidate_base":a["base_x"],"reference_base":b["base_x"],"reference_control_name":b["control_name"],"sat_stratum":sat,"candidate_degree":da,"reference_degree":db,"outcome":outcome,"candidate_peak_columns":a["peak_trace_columns"],"reference_peak_columns":b["peak_trace_columns"],"all_candidate_branches_counted":a["observed_case_count"]==a["expected_case_count"],"all_reference_branches_counted":b["observed_case_count"]==b["expected_case_count"]})
}
fn comparisons(scores: &[Value]) -> Vec<Value> {
    let mut out = Vec::new();
    for a in scores.iter().filter(|r| r["arm"] == "candidate-secant") {
        for b in scores {
            if a["curve"] != b["curve"] || a["target"] != b["target"] {
                continue;
            }
            if b["arm"] == "candidate-dense" && a["base_x"] == b["base_x"] {
                out.push(comparison(a, b, "same-base-encoding"));
            }
            if b["arm"] == "control-secant"
                && a["base_x"].as_array().map(Vec::len) == b["base_x"].as_array().map(Vec::len)
            {
                let kind = if a["base_x"] == b["base_x"] {
                    "same-set-control-encoding"
                } else {
                    "same-encoding-base"
                };
                out.push(comparison(a, b, kind));
            }
        }
    }
    out
}
fn summarize(audit: &str, f4: &str, ladder: &str) -> Result<Value, String> {
    let audit = if audit == "-" {
        Value::Null
    } else {
        read(&Path::new(audit).join("manifest.json"))?
    };
    let ladder = if ladder == "-" {
        Value::Null
    } else {
        read(&Path::new(ladder).join("manifest.json"))?
    };
    let mut records = Vec::new();
    let mut inputs = Vec::new();
    let mut flows = Vec::new();
    let mut parse_failures = Vec::new();
    if f4 != "-" {
        let mut files = Vec::new();
        walk(Path::new(f4), &mut files)?;
        for path in files {
            let r = match read(&path) {
                Ok(r) => r,
                Err(error) => {
                    parse_failures.push(json!({"path":path,"sha256":digest(&path)?,"error":error}));
                    continue;
                }
            };
            if r["schema"] == "secant-f4-panel/v1" {
                flows.push(json!({"path":path,"sha256":digest(&path)?,"requested_systems":r["requested_systems"],"completed_cells":r["completed_cells"],"failed_or_unfinished":r["failed_or_unfinished"]}));
            }
            if r.get("id").is_some() && r.get("arm").is_some() && r.get("input_sha256").is_some() {
                inputs.push(json!({"path":path,"sha256":digest(&path)?}));
                records.push(r);
            }
        }
    }
    let scores = target_scores(&records)?;
    let pairs = comparisons(&scores);
    let mut counts: BTreeMap<String, u64> = BTreeMap::new();
    for p in &pairs {
        let k = format!(
            "{}|{}|{}",
            p["comparison"].as_str().unwrap(),
            p["sat_stratum"].as_str().unwrap(),
            p["outcome"].as_str().unwrap()
        );
        *counts.entry(k).or_default() += 1;
    }
    let requested = if flows.is_empty() {
        None
    } else {
        flows
            .iter()
            .map(|f| f["requested_systems"].as_u64())
            .collect::<Option<Vec<_>>>()
            .map(|x| x.iter().sum::<u64>())
    };
    let verified = records.iter().filter(|r| r["status"] == "verified").count();
    let complete =
        requested == Some(records.len() as u64) && verified == records.len() && !flows.is_empty();
    Ok(
        json!({"schema":"secant-parallel-analysis/v1","source_record_hashes":inputs,"audit":{"complete":audit["complete"],"passes":audit["passes"],"mismatch_count":audit["mismatch_count"],"structured_assignments_checked":audit["structured_assignments_checked"],"dense_assignments_checked":audit["dense_assignments_checked"]},"f4":{"panel_flows":flows,"requested_systems":requested,"whole_panel_complete":complete,"unobserved_records":requested.map(|n|n.saturating_sub(records.len() as u64)),"raw_json_parse_failures":parse_failures,"record_count":records.len(),"verified_records":verified,"failed_or_censored_records":records.iter().filter(|r|r["status"]!="verified").count(),"comparison_scope":"paired observed rows; whole population claim requires whole_panel_complete","target_scores":scores,"paired_comparisons":pairs,"paired_counts_by_comparison_stratum_outcome":counts},"ladder":{"requested_cells":ladder["requested_cells"],"accepted_cells":ladder["accepted_cells"],"cells":ladder["cells"]},"interpretation_limits":["Exact finite-population comparison only; all failed/censored records retained.","Max input, selected pair, final basis and homogeneous degree are distinct.","No complete field-operation cost or timing speedup inferred.","No novelty, P256 or larger-arity conclusion."]}),
    )
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 5 {
        return Err(
            "usage: secant-panel-analysis AUDIT_DIR|- F4_DIR|- LADDER_DIR|- FRESH_OUTPUT.json"
                .into(),
        );
    }
    let out = summarize(&a[1], &a[2], &a[3])?;
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&a[4])?;
    f.write_all(&serde_json::to_vec_pretty(&out)?)?;
    f.write_all(b"\n")?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn row(id: &str, arm: &str, signs: Value, degree: Option<u64>, relations: u64) -> Value {
        json!({"id":id,"arm":arm,"signs":signs,"status":"verified","p":17,"curve":{"a":2,"b":2},"d":2,"target":[10,6],"base_x":[5,6,10,16],"maximum_input_degree":2,"maximum_f4_selected_pair_degree":degree,"observed_f4_degree":degree,"peak_matrix_columns_in_trace":4,"peak_matrix_rows_in_trace":3,"ordered_signed_relation_count":relations,"false_positives":0,"false_negatives":0,"every_reported_relation_group_replayed":true,"gb_checks":{"input_generator_remainders_zero":true,"buchberger_criterion":true}})
    }
    #[test]
    fn missing_branch_never_receives_degree_credit() {
        let r = row("one", "candidate-secant", json!([1, 1]), Some(3), 1);
        let s = target_scores(&[r]).unwrap();
        assert_eq!(s[0]["complete"], false);
        assert_eq!(s[0]["maximum_handled_degree"], Value::Null);
    }
    #[test]
    fn complete_four_branches_are_charged_and_stratified() {
        let mut rows = Vec::new();
        for (i, s) in [[1, 1], [1, -1], [-1, 1], [-1, -1]].into_iter().enumerate() {
            rows.push(row(
                &format!("a{i}"),
                "candidate-secant",
                json!(s),
                Some(3),
                u64::from(i == 0),
            ));
        }
        rows.push(row("dense", "candidate-dense", Value::Null, Some(5), 1));
        let s = target_scores(&rows).unwrap();
        let p = comparisons(&s);
        assert_eq!(p.len(), 1);
        assert_eq!(p[0]["outcome"], "candidate-lower");
        assert_eq!(p[0]["sat_stratum"], "common-SAT");
    }
    #[test]
    fn null_trace_and_duplicate_ids_are_not_wins() {
        let r = row("one", "candidate-dense", Value::Null, None, 1);
        let s = target_scores(std::slice::from_ref(&r)).unwrap();
        assert_eq!(s[0]["maximum_handled_degree"], Value::Null);
        assert!(target_scores(&[r.clone(), r]).is_err());
    }
    #[test]
    fn identical_point_sets_cannot_credit_a_base_geometry_difference() {
        let base = json!({"curve":{"p":17},"target":[10,6],"base_x":[5,6,10,16],"arm":"candidate-secant","maximum_handled_degree":3,"sat":true});
        let mut control = base.clone();
        control["arm"] = json!("control-secant");
        let p = comparisons(&[base, control]);
        assert_eq!(p[0]["comparison"], "same-set-control-encoding");
    }
}
