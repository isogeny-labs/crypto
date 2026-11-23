use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
type Key = (String, String, u64, bool);
fn array<'a>(v: &'a Value, name: &str) -> Result<&'a Vec<Value>, String> {
    v[name]
        .as_array()
        .ok_or_else(|| format!("missing array {name}"))
}
fn text<'a>(v: &'a Value, name: &str) -> Result<&'a str, String> {
    v[name]
        .as_str()
        .ok_or_else(|| format!("missing string {name}"))
}
fn number(v: &Value, name: &str) -> Result<u64, String> {
    v[name]
        .as_u64()
        .ok_or_else(|| format!("not exact u64: {name}"))
}
fn inventory(p: &Value) -> Result<BTreeMap<Key, &Value>, String> {
    if text(p, "schema")? != "ecbench.plan/v1" {
        return Err("wrong plan schema".into());
    }
    let w = array(p, "workloads")?;
    let a = array(p, "arms")?;
    let mut ids = BTreeSet::new();
    for x in w {
        if !ids.insert(text(x, "workload_id")?) {
            return Err("duplicate workload id".into());
        }
    }
    ids.clear();
    for x in a {
        if !ids.insert(text(x, "name")?) {
            return Err("duplicate arm name".into());
        }
    }
    let mut seqs = BTreeSet::new();
    let mut result = BTreeMap::new();
    for e in array(p, "executions")? {
        if !seqs.insert(number(e, "seq")?) {
            return Err("duplicate seq".into());
        }
        let wi = usize::try_from(number(e, "workload")?).map_err(|_| "workload index overflow")?;
        let ai = usize::try_from(number(e, "arm")?).map_err(|_| "arm index overflow")?;
        let workload = w.get(wi).ok_or("invalid workload index")?;
        let arm = a.get(ai).ok_or("invalid arm index")?;
        number(e, "algorithm_seed")?;
        let key = (
            text(workload, "workload_id")?.to_owned(),
            text(arm, "name")?.to_owned(),
            number(e, "round")?,
            e["warmup"].as_bool().ok_or("invalid warmup")?,
        );
        if result.insert(key, e).is_some() {
            return Err("duplicate execution identity".into());
        }
    }
    Ok(result)
}
pub fn compare(gate: &Value, cohort: &Value) -> Result<Value, String> {
    let gi = inventory(gate)?;
    let ci = inventory(cohort)?;
    let gw = array(gate, "workloads")?;
    let cw = array(cohort, "workloads")?;
    let ga = array(gate, "arms")?;
    let ca = array(cohort, "arms")?;
    if ga.len() != ca.len() {
        return Err("arm count differs".into());
    }
    for a in ga {
        if !ca.iter().any(|b| b == a) {
            return Err("resolved arm object differs".into());
        }
    }
    for w in gw {
        if !cw.iter().any(|c| c == w) {
            return Err("full workload object differs or missing".into());
        }
    }
    let ids: BTreeSet<_> = gw
        .iter()
        .map(|w| text(w, "workload_id"))
        .collect::<Result<_, _>>()?;
    let selected: BTreeMap<_, _> = ci
        .iter()
        .filter(|(k, _)| ids.contains(k.0.as_str()))
        .collect();
    let mut missing = 0;
    let mut mismatched_seed = 0;
    let mut mappings = Vec::new();
    for (key, g) in &gi {
        if let Some(c) = ci.get(key) {
            let gs = number(g, "algorithm_seed")?;
            let cs = number(c, "algorithm_seed")?;
            if gs != cs {
                mismatched_seed += 1;
            }
            mappings.push(json!({"workload_id":key.0,"arm":key.1,"round":key.2,"warmup":key.3,"gate_seq":number(g,"seq")?,"cohort_seq":number(c,"seq")?,"gate_algorithm_seed":gs,"cohort_algorithm_seed":cs,"exact_seed_match":gs==cs}));
        } else {
            missing += 1;
        }
    }
    let extra = selected.keys().filter(|k| !gi.contains_key(*k)).count();
    Ok(
        json!({"gate_workloads":gw.len(),"cohort_workloads":cw.len(),"resolved_arms":ga.len(),"gate_executions":gi.len(),"cohort_subset_executions":selected.len(),"mapped":mappings.len(),"missing":missing,"extra_subset_executions":extra,"duplicate":0,"seed_mismatches":mismatched_seed,"full_workload_objects_equal":true,"full_resolved_arm_objects_equal":true,"bijection":missing==0&&extra==0&&mismatched_seed==0,"seq_differences":mappings.iter().filter(|m|m["gate_seq"]!=m["cohort_seq"]).count(),"mappings":mappings}),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Value {
        json!({"schema":"ecbench.plan/v1","workloads":[{"workload_id":"a","target":[1,2]},{"workload_id":"b","target":[3,4]}],"arms":[{"name":"p","method":{"id":"x"}}],"executions":[{"seq":0,"workload":0,"arm":0,"round":1,"warmup":false,"algorithm_seed":18446744073709551615u64},{"seq":1,"workload":1,"arm":0,"round":1,"warmup":false,"algorithm_seed":9007199254740993u64}]})
    }
    #[test]
    fn exact_u64_and_seq_mapping() {
        let g = fixture();
        let mut c = g.clone();
        c["executions"][0]["seq"] = json!(10);
        let r = compare(&g, &c).unwrap();
        assert_eq!(r["bijection"], true);
        assert_eq!(r["seq_differences"], 1);
        assert_eq!(
            r["mappings"][0]["gate_algorithm_seed"].as_u64(),
            Some(u64::MAX)
        );
    }
    #[test]
    fn missing_execution() {
        let g = fixture();
        let mut c = g.clone();
        c["executions"].as_array_mut().unwrap().pop();
        let r = compare(&g, &c).unwrap();
        assert_eq!(r["missing"], 1);
        assert_eq!(r["bijection"], false);
    }
    #[test]
    fn duplicate_execution() {
        let g = fixture();
        let mut c = g.clone();
        let mut e = c["executions"][0].clone();
        e["seq"] = json!(9);
        c["executions"].as_array_mut().unwrap().push(e);
        assert!(compare(&g, &c).unwrap_err().contains("duplicate execution"));
    }
    #[test]
    fn seed_mutation_above_f64_precision() {
        let g = fixture();
        let mut c = g.clone();
        c["executions"][1]["algorithm_seed"] = json!(9007199254740992u64);
        let r = compare(&g, &c).unwrap();
        assert_eq!(r["seed_mismatches"], 1);
        assert_eq!(r["bijection"], false);
    }
    #[test]
    fn wrong_workload() {
        let g = fixture();
        let mut c = g.clone();
        c["workloads"][0]["target"] = json!([1, 3]);
        assert!(compare(&g, &c).unwrap_err().contains("full workload"));
    }
    #[test]
    fn wrong_arm() {
        let g = fixture();
        let mut c = g.clone();
        c["arms"][0]["method"]["id"] = json!("y");
        assert!(compare(&g, &c).unwrap_err().contains("resolved arm"));
    }
    #[test]
    fn invalid_numeric_type() {
        let mut g = fixture();
        g["executions"][0]["algorithm_seed"] = json!(1.5);
        assert!(inventory(&g).is_err());
    }
}
