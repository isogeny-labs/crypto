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
pub fn summarize(manifest: &Path, output: &Path) -> R<()> {
    let m = read(manifest)?;
    let mut rows = Vec::new();
    let mut a_times = Vec::new();
    let mut b_times = Vec::new();
    let mut e_times = Vec::new();
    let mut expected: Option<Value> = None;
    let mut all_identical = true;
    for plan in m.as_array().ok_or("manifest must be an array")? {
        let dir = Path::new(plan["dir"].as_str().ok_or("run path")?);
        if !dir.join("audit.json").exists() {
            rows.push(json!({"id":plan["id"],"arm":plan["arm"],"dir":plan["dir"],"status":if dir.exists(){"FAILED_NO_AUDIT"}else{"NOT_ATTEMPTED"}}));
            continue;
        }
        let a = read(dir.join("audit.json"))?;
        let cfg = read(dir.join("config.json"))?;
        let mut row = json!({"id":plan["id"],"arm":plan["arm"],"dir":plan["dir"],"status":a["status"],"outer_wall_ms":a["outer_wall_ms"],"terminal_counters":a["terminal_counters"],"source_commit":cfg["source_commit"],"worker_blake3":cfg["worker_blake3"]});
        if dir.join("isolation-before.json").exists() {
            row["isolation"] = read(dir.join("isolation-before.json"))?;
        }
        let control = &a["control"];
        if !control.is_null() {
            row["selector_assignments"] = control["selector_assignments"].clone();
            row["xor_profile"] = control["xor_profile"].clone();
            row["solver_ms"] = a["sat_trace"]["terminal_counters"]["elapsed_ms"].clone();
            row["external_process_ms"] = control["external_process_ms"].clone();
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
                }
            }
        }
        rows.push(row);
    }
    let mut pairs = Vec::new();
    for round in 1..=5 {
        let a = rows.iter().find(|r| r["id"] == format!("r{round}-a"));
        let b = rows.iter().find(|r| r["id"] == format!("r{round}-b"));
        if let (Some(a), Some(b)) = (a, b) {
            pairs.push(json!({"round":round,"solver_a_over_b":f(&a["solver_ms"]).zip(f(&b["solver_ms"])).filter(|(_,b)|*b>0.).map(|(a,b)|a/b),"word_updates_a_over_b":f(&a["actual_word_updates"]).zip(f(&b["actual_word_updates"])).filter(|(_,b)|*b>0.).map(|(a,b)|a/b)}));
        }
    }
    let summary = json!({"schema":"n83.native-xor-summary/v1","class":"engineering","scope":"known-witness SAT component and separately bounded natural trials","manifest":manifest,"rows":rows,"identical_search_and_selector_signature":all_identical && expected.is_some(),"signature":expected,"baseline_solver_ms":range(&a_times),"candidate_solver_ms":range(&b_times),"external_process_ms":range(&e_times),"pairs":pairs,"timing_scope":"descriptive recorded conditions; no automatic L2 or full-cold timing admission","natural_yield_winner":Value::Null,"total_index_calculus_runtime_ms":Value::Null});
    write(output, &summary)
}
