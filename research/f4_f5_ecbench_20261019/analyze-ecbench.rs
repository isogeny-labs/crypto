//! Summarize sealed ecbench records and copy the harness's own decisions.
#[path = "../f4_f5_boolean_runtime_20261019/analyze.rs"]
#[allow(dead_code)]
mod common;
use common::{median, spread};
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs, path::Path};

fn read(path: &Path) -> Option<Value> {
    fs::read_to_string(path)
        .ok()
        .map(|s| serde_json::from_str(&s).expect("valid recorded JSON"))
}
fn mean(values: &[f64]) -> Option<f64> {
    (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64)
}
fn native_count(record: &Value, name: &str) -> Option<u64> {
    for key in [Some(name), name.strip_suffix("_uncharged")]
        .into_iter()
        .flatten()
    {
        let values: Vec<_> = record["phases"]
            .as_array()?
            .iter()
            .filter_map(|phase| phase["native"][key].as_u64())
            .collect();
        if !values.is_empty() {
            return Some(values.iter().sum());
        }
        if let Some(value) = record["counters"][key].as_u64() {
            return Some(value);
        }
    }
    if name.starts_with("solver_") {
        record["solver"]["ops"].as_u64()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sensitivity_uses_one_native_ledger_and_preserves_unknowns() {
        let record = json!({"phases":[{"native":{"lookups":3}},{"native":{"lookups":5}}],"counters":{"lookups_uncharged":8},"solver":{"ops":16}});
        assert_eq!(native_count(&record, "lookups_uncharged"), Some(8));
        assert_eq!(
            native_count(&record, "solver_Gray_assignments_uncharged"),
            Some(16)
        );
        assert_eq!(
            native_count(&record, "unmeasured_field_work_uncharged"),
            None
        );
    }
}
fn main() {
    let root = std::env::args()
        .nth(1)
        .expect("analyze-ecbench ECBENCH_OUTPUT");
    let root = Path::new(&root);
    let mut sessions = Vec::new();
    for n in [17, 23] {
        let dir = root.join(format!("session-n{n}"));
        let Some(session) = read(&dir.join("session.json")) else {
            sessions.push(json!({"n":n,"complete":false,"reason":"missing session.json","run_exit":fs::read_to_string(root.join(format!("run-n{n}.exit"))).ok()}));
            continue;
        };
        let records: Vec<Value> = fs::read_to_string(dir.join("records.jsonl"))
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        let spec = read(&dir.join("spec.json")).unwrap();
        let requested = spec["workloads"]["targets_per_curve"].as_u64().unwrap()
            * spec["measurement"]["rounds"].as_u64().unwrap();
        let audit = read(&root.join(format!("audit-n{n}.json")));
        let mut arms = Vec::new();
        for arm in spec["arms"].as_array().unwrap() {
            let name = arm["name"].as_str().unwrap();
            let rows: Vec<_> = records
                .iter()
                .filter(|r| r["arm"] == name && r["warmup"] == false)
                .collect();
            let mut statuses = BTreeMap::<String, usize>::new();
            let mut levels = BTreeMap::<String, usize>::new();
            for record in &rows {
                *statuses
                    .entry(record["outcome"]["status"].as_str().unwrap().into())
                    .or_default() += 1;
                *levels
                    .entry(record["isolation"]["level"].as_str().unwrap().into())
                    .or_default() += 1;
            }
            let verified: Vec<_> = rows
                .iter()
                .copied()
                .filter(|r| r["outcome"]["status"] == "verified")
                .collect();
            let admitted: Vec<_> = verified
                .iter()
                .copied()
                .filter(|r| matches!(r["isolation"]["level"].as_str(), Some("L2" | "L3")))
                .collect();
            let times = |field: &str| {
                admitted
                    .iter()
                    .map(|r| r["time"][field].as_u64().unwrap() as f64)
                    .collect::<Vec<_>>()
            };
            let charged_s: Vec<_> = verified
                .iter()
                .map(|r| r["cost"]["s"].as_f64().unwrap())
                .collect();
            let mut phase_times = BTreeMap::<String, Vec<f64>>::new();
            let mut unknown = Vec::new();
            let mut unpriced_counts = BTreeMap::<String, Vec<u64>>::new();
            let mut bands = [Vec::new(), Vec::new(), Vec::new()];
            let mut workloads = BTreeMap::<String, Value>::new();
            for record in &verified {
                assert_eq!(record["workload"]["curve"]["registered"], true);
                workloads.insert(
                    record["workload"]["workload_id"].as_str().unwrap().into(),
                    record["workload"]["curve"].clone(),
                );
                let mut extra = 0u64;
                for entry in record["cost"]["unpriced"].as_array().unwrap() {
                    let unit = entry.as_str().unwrap();
                    // The explicit counter and generic solver label are aliases
                    // for the generalized engine's one assignment ledger.
                    if unit.starts_with("solver_")
                        && record["solver"]["extra"]["generalized_assignments_uncharged"]
                            .as_u64()
                            .is_some_and(|c| c > 0)
                    {
                        continue;
                    }
                    if let Some(count) = native_count(record, unit) {
                        extra = extra.saturating_add(count);
                        unpriced_counts.entry(unit.into()).or_default().push(count);
                    } else {
                        unknown.push(json!({"run_id":record["run_id"],"unit":unit}));
                    }
                }
                let r = record["workload"]["curve"]["r"]
                    .as_u64()
                    .expect("small registered subgroup") as f64;
                let charged = record["cost"]["s"].as_f64().unwrap();
                for (band, alpha) in bands.iter_mut().zip([0.0, 0.1, 1.0]) {
                    band.push(charged + alpha * extra as f64 / r.sqrt());
                }
            }
            for record in &admitted {
                for phase in record["phases"].as_array().unwrap() {
                    if let Some(wall) = phase["wall_ns"].as_u64() {
                        phase_times
                            .entry(phase["name"].as_str().unwrap().into())
                            .or_default()
                            .push(wall as f64);
                    }
                }
            }
            let phase_times: BTreeMap<_, _> = phase_times
                .into_iter()
                .map(|(name, values)| (name, spread(&values)))
                .collect();
            let counts: BTreeMap<_,_> = unpriced_counts.into_iter().map(|(name,values)| {
                let vals: Vec<_> = values.iter().map(|v|*v as f64).collect();
                (name,json!({"n":values.len(),"median":median(&vals),"min":values.iter().min(),"max":values.iter().max(),"values":values}))
            }).collect();
            arms.push(json!({"name":name,"method":arm["method"],"requested":requested,"measured":rows.len(),"verified":verified.len(),"admitted":admitted.len(),"statuses":statuses,"levels":levels,
                "cold_solve":spread(&times("solve_wall_ns")),"process":spread(&times("process_wall_ns")),"phase_wall":phase_times,"workloads":workloads,
                "s_charged_mean":mean(&charged_s),"s_sensitivity_mean":[mean(&bands[0]),mean(&bands[1]),mean(&bands[2])],"s_sensitivity_alpha":[0,0.1,1],"unpriced_counts":counts,"unquantified":unknown,
                "complete":rows.len() as u64==requested && verified.len() as u64==requested && admitted.len() as u64==requested,
                "lower_bound":verified.iter().any(|r|r["cost"]["lower_bound"]==true)}));
        }
        let mut matched_queries = Vec::new();
        for engine in ["f4", "f5"] {
            for baseline in records.iter().filter(|r| {
                r["arm"] == format!("{engine}-baseline")
                    && r["warmup"] == false
                    && r["outcome"]["status"] == "verified"
            }) {
                let candidate = records.iter().find(|r| {
                    r["arm"] == format!("{engine}-tail")
                        && r["warmup"] == false
                        && r["workload"]["workload_id"] == baseline["workload"]["workload_id"]
                        && r["round"] == baseline["round"]
                });
                let equal = candidate
                    .filter(|r| r["outcome"]["status"] == "verified")
                    .map(|candidate| {
                        ["targets_tried", "relations_found", "matrix_rank"]
                            .iter()
                            .all(|key| candidate["counters"][*key] == baseline["counters"][*key])
                            && candidate["outcome"] == baseline["outcome"]
                            && candidate["factor_base"] == baseline["factor_base"]
                    });
                matched_queries.push(json!({"engine":engine,"workload":baseline["workload"]["workload_id"],"round":baseline["round"],"equal_query_counts_and_verified_outcome":equal}));
            }
        }
        let rho_s = arms
            .iter()
            .find(|a| a["name"] == "rho-strong")
            .and_then(|a| a["s_charged_mean"].as_f64());
        for arm in &mut arms {
            arm["charged_s_over_rho_diagnostic"] = json!(arm["s_charged_mean"]
                .as_f64()
                .zip(rho_s)
                .map(|(a, b)| a / b));
        }
        let mut comparisons = Vec::new();
        for pair in [
            "f4",
            "f5",
            "rho-f4-tail",
            "rho-f5-tail",
            "rho-f6-native",
            "rho-generalized-typed",
            "rho-semaev-batched",
            "rho-semaev-full-table",
        ] {
            comparisons.push(
                json!({"pair":pair,"result":read(&root.join(format!("compare-n{n}-{pair}.json")))}),
            );
        }
        let complete = arms.iter().all(|a| a["complete"] == true)
            && audit.as_ref().is_some_and(|a| a["ok"] == true);
        sessions.push(json!({"n":n,"session":session,"audit":audit,"complete":complete,"arms":arms,"comparisons":comparisons,"matched_baseline_candidate_query_checks":matched_queries,"records_total":records.len(),"warmups":records.iter().filter(|r|r["warmup"]==true).count(),"table":fs::read_to_string(root.join(format!("table-n{n}.txt"))).ok()}));
    }
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"boolean-recursive-ecbench/v1","sessions":sessions,"scope":"Cold complete one-target DLP recovery on matched standard polynomial bases. Solver and whole-child wall are separate recorded clocks. Native ecbench comparison admission is authoritative; failed and below-level rows remain. Partially priced S and sensitivity bands are diagnostic, not full-cost rankings."})).unwrap());
}
