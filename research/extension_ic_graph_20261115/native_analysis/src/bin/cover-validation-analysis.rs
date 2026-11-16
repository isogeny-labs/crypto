//! Endpoint validation only. No A/A, ratios, confidence intervals or parity claim.
#[allow(dead_code)]
#[path = "cover-graph-analysis.rs"]
mod common;
use common::{clean, context, int, public_family, valid_graph};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
type Key = (u64, u64, String);
fn key(v: &Value) -> Result<Key, String> {
    Ok((
        int(v, "p")?,
        int(v, "seed")?,
        v["arm"].as_str().ok_or("arm")?.into(),
    ))
}
fn analyze(plan: &Value, rows: Vec<Value>, bad_lines: usize) -> Result<Value, String> {
    let source = plan["source_revision"]
        .as_str()
        .filter(|s| s.len() == 40 && s.bytes().all(|c| c.is_ascii_hexdigit()))
        .ok_or("source SHA")?;
    if plan["driver_kind"] != "cover-sieve-validation"
        || plan
            .get("graph_strategy")
            .is_some_and(|v| v.as_str().is_none_or(|s| !["pivot", "mst"].contains(&s)))
        || plan["rounds"] != 1
        || plan["warmup_rounds"] != 0
        || plan["worker_timeout_seconds"] != 900
        || plan["retained_percent"] != 75
        || plan["allow_comparative_claims"] != false
    {
        return Err("invalid frozen validation plan".into());
    }
    let mut schedule = Vec::new();
    let mut cases = BTreeSet::new();
    for p in plan["primes"].as_array().ok_or("primes")? {
        for seed in plan["seeds"].as_array().ok_or("seeds")? {
            let c = (p.as_u64().ok_or("prime")?, seed.as_u64().ok_or("seed")?);
            if !cases.insert(c) {
                return Err("duplicate planned case".into());
            }
            for arm in ["ic_a", "graph_a"] {
                schedule.push((c.0, c.1, arm.into()));
            }
        }
    }
    if schedule.is_empty() {
        return Err("empty cohort".into());
    }
    let mut problems = BTreeSet::new();
    if bad_lines > 0 {
        problems.insert(format!("{bad_lines} malformed lines"));
    }
    let mut fixtures = BTreeMap::new();
    let mut starts = Vec::new();
    let mut completed = Vec::new();
    let mut executions = BTreeMap::new();
    let mut pending = None;
    let mut raw_count = 0;
    for row in rows {
        if row["schema"] != "cover.sieve.graph-validation/v1"
            || (plan.get("graph_strategy").is_some()
                && row["graph_strategy"] != plan["graph_strategy"])
            || (plan.get("graph_strategy").is_none()
                && row.get("graph_strategy").is_some_and(|v| v != "pivot"))
            || row["source_revision"] != source
            || row["validation_only"] != true
            || row["worker_timeout_seconds"] != 900
            || row["sieve_recipe"] != "incidence_shared"
        {
            problems.insert("schema/source/validation mode/recipe mismatch".into());
        }
        match row["record_type"].as_str() {
            Some("fixture") => {
                let c = (int(&row, "p")?, int(&row, "seed")?);
                if !cases.contains(&c) || fixtures.insert(c, row).is_some() {
                    problems.insert("unexpected/duplicate fixture".into());
                }
            }
            Some("execution_start") | Some("execution") => {
                let k = key(&row)?;
                if row["phase"] != "validation"
                    || row["round"] != 0
                    || fixtures
                        .get(&(k.0, k.1))
                        .is_none_or(|f| context(f) != context(&row))
                {
                    problems.insert("execution identity/phase mismatch".into());
                }
                if row["record_type"] == "execution_start" {
                    if pending.is_some() {
                        problems.insert("overlapping starts".into());
                    }
                    pending = Some(k.clone());
                    starts.push(k);
                } else {
                    raw_count += 1;
                    if pending.as_ref() != Some(&k) {
                        problems.insert("completion without matching start".into());
                    }
                    pending = None;
                    completed.push(k.clone());
                    if executions.insert(k, row).is_some() {
                        problems.insert("duplicate completion".into());
                    }
                }
            }
            _ => {
                problems.insert("unknown record type".into());
            }
        }
    }
    if starts.len() > schedule.len()
        || completed.len() > schedule.len()
        || starts != schedule[..starts.len().min(schedule.len())]
        || completed != schedule[..completed.len().min(schedule.len())]
        || starts.len() < completed.len()
        || starts.len() > completed.len() + 1
    {
        problems.insert("frozen schedule mismatch".into());
    }
    let mut distinct = BTreeSet::new();
    for (&(p, _), f) in &fixtures {
        let digest = blake3::hash(&serde_json::to_vec(&f["family"]).map_err(|e| e.to_string())?)
            .to_hex()
            .to_string();
        let l = f["family"]["l"].as_u64().unwrap_or(0);
        if !public_family(&f["family"], p)
            || f["input_digest"] != digest
            || !distinct.insert(digest)
            || f["expected_scalar"]
                .as_u64()
                .is_none_or(|d| d == 0 || d >= l)
            || f["capacity_model_m9"]
                .as_f64()
                .is_none_or(|v| !v.is_finite() || v < 1.25 * p as f64)
        {
            problems.insert("public family/digest/held-out answer/capacity mismatch".into());
        }
    }
    let mut outcomes = Vec::new();
    for (p, seed, arm) in &schedule {
        let row = executions.get(&(*p, *seed, arm.clone()));
        let fixture = fixtures.get(&(*p, *seed));
        let mut valid = false;
        let mut exercised = false;
        if let (Some(r), Some(f)) = (row, fixture) {
            let w = &r["worker"];
            let work = &w["work"];
            let method = if arm == "ic_a" { "ic" } else { "graph" };
            let mut semantic = work.clone();
            clean(&mut semantic);
            valid = r["valid"] == true
                && (plan.get("graph_strategy").is_none()
                    || w["graph_strategy"] == plan["graph_strategy"])
                && r["child_exit"] == 0
                && w["method"] == method
                && w["solver_answer_placeholder"] == 0
                && w["verified"] == true
                && w["capped"] == false
                && w["recovered_scalar"] == f["expected_scalar"]
                && work["solved"] == true
                && work["correct"] == false
                && work["p"] == *p
                && work["seed"] == *seed
                && work["l"] == f["family"]["l"]
                && r["cold_process_wall_ns"].as_u64().is_some_and(|v| v > 0)
                && w["inner_wall_ns"].as_u64().is_some_and(|v| v > 0)
                && r["cold_process_wall_ns"].as_u64() >= w["inner_wall_ns"].as_u64()
                && w["signature"]
                    == json!({"scalar":w["recovered_scalar"],"verified":true,"capped":false,"work":semantic});
            if method == "graph" {
                let g = &work["graph_filter"];
                valid &= valid_graph(g);
                valid &= (plan["graph_strategy"] == "mst") == !g["mst"].is_null();
                valid &= plan["graph_strategy"] != "mst" || w["graph_strategy"] == "mst";
                exercised = g["projected"].as_u64().is_some_and(|v| v > 0)
                    && g["fallback_solves"] == 0
                    && g["output_rows"].as_u64() < g["input_rows"].as_u64();
                valid &= exercised;
            } else {
                valid &= work["graph_filter"].is_null();
            }
        }
        outcomes.push(json!({"p":p,"seed":seed,"arm":arm,"completed":row.is_some(),"endpoint_valid":valid,"nontrivial_graph_without_fallback":exercised,"child_exit":row.map(|r|r["child_exit"].clone()),"recorded_outer_wall_ns":row.map(|r|r["cold_process_wall_ns"].clone()),"recovered_scalar":row.map(|r|r["worker"]["recovered_scalar"].clone()),"work":row.map(|r|r["worker"]["work"].clone()),"progress_path":row.map(|r|r["progress_path"].clone())}));
    }
    let valid = problems.is_empty()
        && executions.len() == schedule.len()
        && starts.len() == schedule.len()
        && plan["observed_process_exit"] == 0
        && outcomes.iter().all(|r| r["endpoint_valid"] == true);
    Ok(
        json!({"schema":"cover.sieve.validation-summary/v1","source_revision":source,"observed_process_exit":plan["observed_process_exit"],"planned_executions":schedule.len(),"attempted_executions":starts.len(),"completed_executions":executions.len(),"raw_executions":raw_count,"in_flight_without_completion":pending,"unattempted_executions":schedule.len().saturating_sub(starts.len()),"global_problems":problems,"full_recovery_validation_passed":valid,"allows_new_frozen_comparison":valid,"comparative_eligibility":false,"rho_parity":false,"scope":"two public-input endpoint checks; no AA or comparative timing inference; all recorded wall is spent-work diagnostic","outcomes":outcomes}),
    )
}
fn main() -> Result<(), Box<dyn Error>> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 4 {
        return Err("usage: cover-validation-analysis RAW PLAN NEW-DIR".into());
    }
    let mut p: Value = serde_json::from_reader(fs::File::open(&a[2])?)?;
    p["observed_process_exit"] = fs::read_to_string(
        Path::new(&a[1])
            .parent()
            .ok_or("raw parent")?
            .join("exit-code.txt"),
    )
    .ok()
    .and_then(|s| s.trim().parse::<i32>().ok())
    .map_or(Value::Null, |x| json!(x));
    let mut rows = Vec::new();
    let mut bad = 0;
    for line in BufReader::new(fs::File::open(&a[1])?).lines() {
        match serde_json::from_str(&line?) {
            Ok(v) => rows.push(v),
            Err(_) => bad += 1,
        }
    }
    let s = analyze(&p, rows, bad)?;
    fs::create_dir(&a[3])?;
    let mut f = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(Path::new(&a[3]).join("summary.json"))?;
    serde_json::to_writer_pretty(&mut f, &s)?;
    writeln!(f)?;
    println!(
        "completed={}/{}, endpoint-validation={}",
        s["completed_executions"], s["planned_executions"], s["full_recovery_validation_passed"]
    );
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (Value, Vec<Value>) {
        let source = "0".repeat(40);
        let p = json!({"source_revision":source,"driver_kind":"cover-sieve-validation","primes":[1009],"seeds":[251],"rounds":1,"warmup_rounds":0,"worker_timeout_seconds":900,"retained_percent":75,"allow_comparative_claims":false,"observed_process_exit":0});
        let family = json!({"p":1009,"l":(1u64<<61)-1,"alpha":[[1,0],[2,0],[3,0]],"g":{"inf":false,"x":[[1,0],[0,0],[0,0]],"y":[[2,0],[0,0],[0,0]]},"q":{"inf":false,"x":[[3,0],[0,0],[0,0]],"y":[[4,0],[0,0],[0,0]]}});
        let digest = blake3::hash(&serde_json::to_vec(&family).unwrap())
            .to_hex()
            .to_string();
        let base = json!({"schema":"cover.sieve.graph-validation/v1","source_revision":source,"p":1009,"seed":251,"family":family,"input_digest":digest,"validation_only":true,"worker_timeout_seconds":900,"retained_percent":75,"sieve_recipe":"incidence_shared","capacity_model_m9":1300.0,"record_type":"fixture","expected_scalar":3});
        let mut rows = vec![base.clone()];
        for arm in ["ic_a", "graph_a"] {
            let mut row = base.clone();
            row["record_type"] = json!("execution_start");
            row["phase"] = json!("validation");
            row["round"] = json!(0);
            row["arm"] = json!(arm);
            rows.push(row.clone());
            let graph = json!({"calls":1,"projected":1,"refusals":0,"fallback_solves":0,"replayed_rows":4,"input_rows":5,"output_rows":4,"input_digest":"a".repeat(64),"counter_scope":"legacy counters omit graph"});
            let work = json!({"p":1009,"seed":251,"l":(1u64<<61)-1,"solved":true,"correct":false,"graph_filter":if arm=="graph_a"{graph}else{Value::Null}});
            let w = json!({"method":if arm=="ic_a"{"ic"}else{"graph"},"solver_answer_placeholder":0,"recovered_scalar":3,"verified":true,"capped":false,"inner_wall_ns":10,"work":work,"signature":{"scalar":3,"verified":true,"capped":false,"work":work}});
            row["record_type"] = json!("execution");
            row["valid"] = json!(true);
            row["child_exit"] = json!(0);
            row["cold_process_wall_ns"] = json!(20);
            row["worker"] = w;
            rows.push(row)
        }
        (p, rows)
    }
    #[test]
    fn endpoints_are_validation_not_comparative_evidence() {
        let (p, r) = fixture();
        let s = analyze(&p, r, 0).unwrap();
        assert_eq!(s["full_recovery_validation_passed"], true);
        assert_eq!(s["comparative_eligibility"], false);
        assert_eq!(s["rho_parity"], false);
    }
    #[test]
    fn explicit_mst_validation_binds_worker_identity_and_counters() {
        let (mut p, mut rows) = fixture();
        p["graph_strategy"] = json!("mst");
        for r in &mut rows {
            r["graph_strategy"] = json!("mst");
            if r["record_type"] == "execution" {
                r["worker"]["graph_strategy"] = json!("mst");
                if r["worker"]["method"] == "graph" {
                    r["worker"]["work"]["graph_filter"]["mst"] = json!({"trees":1,"edge_evaluations":3,"infeasible_edge_candidates":0,"normalized_rows":3,"selected_edges":2,"largest_column_degree":3,"largest_active_coefficients":20,"input_nonzeros":10,"output_nonzeros":4,"budget_refusals":0});
                    r["worker"]["work"]["graph_filter"]["last_report"] =
                        json!({"pivots":1,"merges":2});
                    let mut work = r["worker"]["work"].clone();
                    clean(&mut work);
                    r["worker"]["signature"] =
                        json!({"scalar":3,"verified":true,"capped":false,"work":work});
                }
            }
        }
        assert_eq!(
            analyze(&p, rows.clone(), 0).unwrap()["full_recovery_validation_passed"],
            true
        );
        rows[4]["worker"]["graph_strategy"] = json!("pivot");
        assert_eq!(
            analyze(&p, rows, 0).unwrap()["full_recovery_validation_passed"],
            false
        );
    }
    #[test]
    fn wrong_scalar_fake_mode_and_fallback_never_qualify() {
        for kind in 0..4 {
            let (p, mut r) = fixture();
            match kind {
                0 => r[4]["worker"]["recovered_scalar"] = json!(4),
                1 => r[1]["validation_only"] = json!(false),
                2 => r[4]["worker"]["work"]["graph_filter"]["fallback_solves"] = json!(1),
                _ => r[4]["child_exit"] = json!(124),
            }
            assert_eq!(
                analyze(&p, r, 0).unwrap()["full_recovery_validation_passed"],
                false
            )
        }
    }
    #[test]
    fn censor_and_nonzero_exit_block_followup() {
        let (mut p, mut r) = fixture();
        p["observed_process_exit"] = json!(124);
        r.truncate(4);
        let s = analyze(&p, r, 0).unwrap();
        assert_eq!(s["attempted_executions"], 2);
        assert_eq!(s["completed_executions"], 1);
        assert!(!s["in_flight_without_completion"].is_null());
        assert_eq!(s["allows_new_frozen_comparison"], false);
    }
}
