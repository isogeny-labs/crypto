//! Native calculation of recorded XOR comparisons. Rendering consumes these values.
use super::{read, write, R};
use serde_json::{json, Value};
use std::path::Path;

fn f(v: &Value) -> Option<f64> {
    v.as_f64().filter(|v| v.is_finite())
}
fn range(xs: &[f64]) -> Value {
    if xs.is_empty() {
        return Value::Null;
    }
    let mut x = xs.to_vec();
    x.sort_by(f64::total_cmp);
    let n = x.len();
    json!({"n":n,"min":x[0],"max":x[n-1],"median":if n%2==0 {(x[n/2-1]+x[n/2])/2.0}else{x[n/2]}})
}
fn cms_seconds(raw: &str, prefix: &str) -> Option<f64> {
    raw.lines()
        .find_map(|s| {
            s.strip_prefix(prefix)?
                .split_whitespace()
                .next()?
                .parse::<f64>()
                .ok()
        })
        .filter(|s| s.is_finite() && *s >= 0.)
}
pub fn summarize(manifest: &Path, output: &Path) -> R<()> {
    let m = read(manifest)?;
    let mut rows = Vec::new();
    let mut a_times = Vec::new();
    let mut b_times = Vec::new();
    let mut e_times = Vec::new();
    let mut e_cpu = Vec::new();
    let mut expected: Option<Value> = None;
    let mut all_identical = true;
    for plan in m.as_array().ok_or("manifest must be an array")? {
        let dir = Path::new(plan["dir"].as_str().ok_or("run path")?);
        if !dir.join("audit.json").exists() {
            let status =
                if dir.join("isolation-before.json").exists() && !dir.join("outer.json").exists() {
                    "PREFLIGHT_REFUSED_NO_WORKER"
                } else if dir.exists() {
                    "FAILED_NO_AUDIT"
                } else {
                    "NOT_ATTEMPTED"
                };
            rows.push(json!({"id":plan["id"],"arm":plan["arm"],"dir":plan["dir"],"status":status,"planned_status":plan["status"]}));
            continue;
        }
        let a = read(dir.join("audit.json"))?;
        let cfg = read(dir.join("config.json"))?;
        let mut row = json!({"id":plan["id"],"arm":plan["arm"],"dir":plan["dir"],"status":a["status"],"outer_wall_ms":a["outer_wall_ms"],"terminal_counters":a["terminal_counters"],"source_commit":cfg["source_commit"],"worker_blake3":cfg["worker_blake3"],"columns":cfg["columns"],"conflict_cap":cfg["conflicts"],"wall_cap_seconds":cfg["wall_seconds"]});
        if let Some(ps) = a["phases"].as_array() {
            row["phases"] = a["phases"].clone();
            let start = ps
                .iter()
                .find(|p| p["phase"] == "solver_enter")
                .and_then(|p| f(&p["elapsed_ms"]));
            let end = ps
                .iter()
                .find(|p| p["phase"] == "solver_return")
                .and_then(|p| f(&p["elapsed_ms"]));
            row["natural_solver_ms"] = start
                .zip(end)
                .filter(|(a, b)| b >= a)
                .map(|(a, b)| Value::from(b - a))
                .unwrap_or(Value::Null);
        }
        if dir.join("isolation-before.json").exists() {
            row["isolation"] = read(dir.join("isolation-before.json"))?;
        }
        let control = &a["control"];
        if !control.is_null() {
            row["selector_assignments"] = control["selector_assignments"].clone();
            row["xor_profile"] = control["xor_profile"].clone();
            row["solver_ms"] = a["sat_trace"]["terminal_counters"]["elapsed_ms"].clone();
            row["external_process_ms"] = control["external_process_ms"].clone();
            if dir.join("cms.stdout").exists() {
                let raw = std::fs::read_to_string(dir.join("cms.stdout"))?;
                row["external_algorithm_cpu_ms"] =
                    cms_seconds(&raw, "c time in solve()/simplify():")
                        .map(|s| Value::from(s * 1000.))
                        .unwrap_or(Value::Null);
                row["external_parsing_cpu_ms"] = cms_seconds(&raw, "c Parsing time:")
                    .map(|s| Value::from(s * 1000.))
                    .unwrap_or(Value::Null);
            }
            if a["status"] == "PASS_KNOWN_WITNESS_MODEL_AND_INDEPENDENT_GROUP_SUM" {
                let arm = plan["arm"].as_str().unwrap_or("");
                if arm == "A" || arm == "B" {
                    let p = &control["xor_profile"];
                    let signature = json!({"conflicts":control["conflicts"],"selectors":control["selector_assignments"],"decisions":p["decisions"],"restarts":p["restarts"],"passes":p["xor_passes"],"repivots":p["xor_repivots"],"row_ops":p["xor_row_ops"],"row_scans":p["xor_row_scans"],"reason_lits":p["xor_reason_lits"]});
                    if let Some(e) = &expected {
                        all_identical &= *e == signature;
                    } else {
                        expected = Some(signature);
                    }
                    if let Some(ms) = f(&row["solver_ms"]) {
                        if arm == "A" {
                            a_times.push(ms);
                        } else {
                            b_times.push(ms);
                        }
                    }
                    let words = if arm == "A" {
                        p["xor_row_ops"]
                            .as_u64()
                            .zip(p["words_per_dense_row"].as_u64())
                            .map(|(a, b)| a * b)
                    } else {
                        p["xor_word_updates"].as_u64()
                    };
                    row["actual_word_updates"] = words.map(Value::from).unwrap_or(Value::Null);
                } else if let Some(ms) = f(&row["external_process_ms"]) {
                    e_times.push(ms);
                    if let Some(ms) = f(&row["external_algorithm_cpu_ms"]) {
                        e_cpu.push(ms);
                    }
                }
            }
        }
        rows.push(row);
    }
    let mut pairs = Vec::new();
    for round in 1..=5 {
        let a = rows.iter().find(|r| r["id"] == format!("r{round}-a"));
        let b = rows.iter().find(|r| r["id"] == format!("r{round}-b"));
        let e = rows.iter().find(|r| r["id"] == format!("r{round}-e"));
        if let (Some(a), Some(b)) = (a, b) {
            pairs.push(json!({"round":round,"solver_a_over_b":f(&a["solver_ms"]).zip(f(&b["solver_ms"])).filter(|(_,b)|*b>0.).map(|(a,b)|a/b),"word_updates_a_over_b":f(&a["actual_word_updates"]).zip(f(&b["actual_word_updates"])).filter(|(_,b)|*b>0.).map(|(a,b)|a/b),"candidate_wall_over_external_algorithm_cpu":e.and_then(|e|f(&e["external_algorithm_cpu_ms"])).zip(f(&b["solver_ms"])).filter(|(e,_)|*e>0.).map(|(e,b)|b/e),"external_timer_note":"native solve wall / CMS solve-simplify reported CPU; different timer kinds"}));
        }
    }
    let complete = a_times.len() == 7
        && b_times.len() == 7
        && e_cpu.len() == 7
        && pairs.iter().all(|p| p["solver_a_over_b"].is_number());
    let summary = json!({"schema":"n83.native-xor-summary/v2","class":"engineering","scope":"known-witness SAT component and separately bounded natural trials","manifest":manifest,"rows":rows,"complete_comparison_coverage":complete,"identical_search_and_selector_signature":complete && all_identical && expected.is_some(),"signature":expected,"baseline_solver_ms":range(&a_times),"candidate_solver_ms":range(&b_times),"external_process_ms":range(&e_times),"external_algorithm_cpu_ms":range(&e_cpu),"pairs":pairs,"timing_scope":"descriptive recorded conditions; no automatic L2 or full-cold timing admission; missing worker context-switch/fault accounting","natural_yield_winner":Value::Null,"total_index_calculus_runtime_ms":Value::Null});
    write(output, &summary)
}

#[test]
fn cms_internal_clock_is_not_process_time() {
    assert_eq!(
        cms_seconds(
            "c Parsing time: 0.09 s\nc time in solve()/simplify(): 2.83 (97%)",
            "c time in solve()/simplify():"
        ),
        Some(2.83)
    );
    assert_eq!(
        cms_seconds("c Total time: 3.0", "c time in solve()/simplify():"),
        None
    );
    assert_eq!(
        cms_seconds(
            "c time in solve()/simplify(): NaN",
            "c time in solve()/simplify():"
        ),
        None
    );
}
