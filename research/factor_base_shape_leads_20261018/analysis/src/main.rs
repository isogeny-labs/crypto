use serde_json::{json, Value};
mod coverage;
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs,
    path::Path,
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
type PointSet = BTreeSet<(String, String)>;
type Measurement = (Value, PointSet, String);
fn read(p: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn num(v: &Value, key: &str) -> Result<u64> {
    v[key]
        .as_u64()
        .ok_or_else(|| format!("missing integer {key}").into())
}
fn text<'a>(v: &'a Value, key: &str) -> Result<&'a str> {
    v[key]
        .as_str()
        .ok_or_else(|| format!("missing string {key}").into())
}
fn points(base: &Value) -> Result<BTreeSet<(String, String)>> {
    base["points"]
        .as_array()
        .ok_or("missing point inventory")?
        .iter()
        .map(|p| Ok((text(p, "x")?.to_owned(), text(p, "y")?.to_owned())))
        .collect()
}
fn phase(trials: &[Value], which: &str) -> Result<Value> {
    let mut targets = BTreeSet::new();
    let mut truth_targets = BTreeSet::new();
    let mut verified_targets = BTreeSet::new();
    let (mut count, mut true_count, mut verified, mut work, mut successful_work, mut max_degree) =
        (0, 0, 0, 0u64, 0u64, 0u64);
    for t in trials.iter().filter(|t| t["phase"] == which) {
        count += 1;
        let target = serde_json::to_string(&t["target"])?;
        targets.insert(target.clone());
        work = work
            .checked_add(num(t, "field_reduction_multiplications")?)
            .ok_or("work overflow")?;
        if t["ground_truth_decomposable"] == true {
            true_count += 1;
            truth_targets.insert(target.clone());
        }
        if t["witness_verified"] == true {
            verified += 1;
            verified_targets.insert(target);
            successful_work = successful_work
                .checked_add(num(t, "field_reduction_multiplications")?)
                .ok_or("work overflow")?;
            max_degree = max_degree.max(num(t, "observed_solving_degree")?);
        }
    }
    Ok(
        json!({"trials":count,"distinct_targets":targets.len(),"truth_decomposable_trials":true_count,
      "distinct_truth_decomposable_targets":truth_targets.len(),"verified":verified,
      "distinct_verified_targets":verified_targets.len(),"field_reduction_multiplications":work,
      "successful_field_reduction_multiplications":successful_work,"max_successful_observed_degree":if verified>0{Some(max_degree)}else{None}}),
    )
}
fn representation_comparison(a: &Value, b: &Value) -> Result<Value> {
    let aa = a["trials"].as_array().ok_or("missing baseline trials")?;
    let bb = b["trials"].as_array().ok_or("missing candidate trials")?;
    let mut matched = 0;
    let mut successful = 0;
    let (mut a_work, mut b_work) = (0u64, 0u64);
    let mut all_successful_strictly_less = true;
    let mut positive_control_target_equal = true;
    for x in aa {
        let y = bb
            .iter()
            .find(|y| y["phase"] == x["phase"] && y["trial"] == x["trial"])
            .ok_or("missing matched trial")?;
        if x["phase"] == "positive_control" {
            positive_control_target_equal = x["target"] == y["target"];
            continue; // Different constructor ordering can choose another control.
        }
        if x["target"] != y["target"]
            || x["ground_truth_decomposable"] != y["ground_truth_decomposable"]
            || x["status"] != y["status"]
        {
            return Err("matched representation changes target truth or result".into());
        }
        matched += 1;
        if x["witness_verified"] == true {
            successful += 1;
            let u = num(x, "field_reduction_multiplications")?;
            let v = num(y, "field_reduction_multiplications")?;
            a_work = a_work.checked_add(u).ok_or("work overflow")?;
            b_work = b_work.checked_add(v).ok_or("work overflow")?;
            all_successful_strictly_less &= v < u;
        }
    }
    Ok(
        json!({"baseline":a["candidate"]["label"],"candidate":b["candidate"]["label"],
      "matched_random_probes":matched,"matched_successful_probes":successful,
      "baseline_successful_f4_multiplications":a_work,"candidate_successful_f4_multiplications":b_work,
      "all_matched_successful_strictly_less":successful>0&&all_successful_strictly_less,
      "positive_control_target_equal":positive_control_target_equal,
      "positive_controls_in_cost_comparison":false,"full_pipeline_speedup":null}),
    )
}
fn failed_row(
    panel: &str,
    item: &Value,
    base: &Value,
    request: &Value,
    complete: &Value,
    first: &str,
) -> Value {
    json!({"panel":panel,"id":item["id"],"label":item["label"],"outcome":item["outcome"],
      "curve":base["curve"],"configuration":request["candidate"],
      "columns":base["factor_base"]["columns"],"signed_points":base["factor_base"]["signed_points"],
      "ordered_points_sha256":base["factor_base"]["points_sha256"],
      "exact_point_set_group":first,"same_point_set_as_earlier":Some(first)!=item["label"].as_str(),
      "failure":complete["fatal"],"train":null,"holdout":null,"positive_control":null,
      "training_independent_rank":null,"certified_degree_of_regularity":null,"full_pipeline_speedup":null})
}
fn audit(root: &Path, panels: &[String]) -> Result<Value> {
    let mut rows = Vec::new();
    let mut identities: BTreeMap<(String, BTreeSet<(String, String)>), String> = BTreeMap::new();
    let mut measurements: Vec<Measurement> = Vec::new();
    let (mut sealed, mut witnesses, mut trials) = (0, 0, 0);
    for panel in panels {
        let verify = read(&root.join(format!("{panel}.verify.json")))?;
        sealed += num(&verify, "sealed_attempts_verified")?;
        witnesses += num(&verify, "trial_point_witness_checks")?;
        let summary = read(&root.join(panel).join("summary-0001.json"))?;
        for item in summary["candidates"]
            .as_array()
            .ok_or("missing candidates")?
        {
            let id = text(item, "id")?;
            let attempt = Path::new(text(item, "attempt")?)
                .file_name()
                .ok_or("invalid attempt path")?;
            let dir = root.join(panel).join("candidates").join(id).join(attempt);
            let base = read(&dir.join("base.json"))?;
            let curve = text(&base["curve"], "slug")?.to_owned();
            let pts = points(&base)?;
            let key = (curve.clone(), pts.clone());
            let label = text(item, "label")?.to_owned();
            let first = identities
                .entry(key)
                .or_insert_with(|| label.clone())
                .clone();
            if item["outcome"] != "completed" {
                let request = read(&dir.join("request.json"))?;
                let complete = read(&dir.join("complete.json"))?;
                rows.push(failed_row(panel, item, &base, &request, &complete, &first));
                continue;
            }
            let m = &item["measurement"];
            let ts = m["trials"]
                .as_array()
                .ok_or("incomplete candidate: no trial records")?;
            trials += ts.len();
            let row = json!({"panel":panel,"id":id,"label":label,"outcome":item["outcome"],
              "curve":base["curve"],"configuration":m["candidate"],"columns":m["columns"],
              "signed_points":base["factor_base"]["signed_points"],"ordered_points_sha256":base["factor_base"]["points_sha256"],
              "exact_point_set_group":first,"same_point_set_as_earlier":first!=label,
              "training_independent_rank":m["training_independent_rank"],
              "best_observed_successful_solving_degree":m["best_observed_successful_solving_degree"],
              "positive_control":phase(ts,"positive_control")?,"train":phase(ts,"train")?,"holdout":phase(ts,"holdout")?,
              "certified_degree_of_regularity":null,"full_pipeline_speedup":null,"total_cost_winner":null});
            measurements.push((m.clone(), pts, curve));
            rows.push(row);
        }
    }
    if sealed != rows.len() as u64 {
        return Err("verification receipt/candidate count mismatch".into());
    }
    let mut representation = Vec::new();
    for (a, pa, ca) in &measurements {
        let label = text(&a["candidate"], "label")?;
        if label != "primary-coordinate-d6" && !label.ends_with("-natural") {
            continue;
        }
        for (b, pb, cb) in &measurements {
            let other = text(&b["candidate"], "label")?;
            if pa == pb
                && ca == cb
                && label != other
                && (other.starts_with("primary-coordinate-")
                    || other.starts_with("representation-"))
            {
                representation.push(representation_comparison(a, b)?);
            }
        }
    }
    let mut groups: BTreeMap<(String, String, u64, String), Vec<&Value>> = BTreeMap::new();
    for row in &rows {
        if row["panel"] != "secondary" && row["panel"] != "confirmation" {
            continue;
        }
        let Some(shape) = row["configuration"]["shape"].as_str() else {
            continue;
        };
        let key = (
            text(row, "panel")?.to_owned(),
            text(&row["curve"], "slug")?.to_owned(),
            num(&row["configuration"], "dimension")?,
            shape.to_owned(),
        );
        groups.entry(key).or_default().push(row);
    }
    let mut filters = Vec::new();
    for ((panel, curve, dimension, shape), members) in groups {
        let qualifying: Vec<_> = members
            .iter()
            .filter(|r| {
                r["outcome"] == "completed"
                    && r["holdout"]["verified"].as_u64().unwrap_or(0) >= 4
                    && r["training_independent_rank"].as_u64().unwrap_or(0) > 0
            })
            .collect();
        let seeds: BTreeSet<_> = qualifying
            .iter()
            .filter_map(|r| r["configuration"]["seed"].as_u64())
            .collect();
        let sets: BTreeSet<_> = qualifying
            .iter()
            .filter_map(|r| r["exact_point_set_group"].as_str())
            .collect();
        let required = if panel == "confirmation" && shape == "tracezero-gp" {
            3
        } else {
            2
        };
        filters.push(json!({"panel":panel,"curve":curve,"dimension":dimension,"shape":shape,"qualifying_cases":qualifying.len(),
          "qualifying_seeds":seeds,"distinct_qualifying_point_sets":sets.len(),
          "required_distinct_constructor_seeds":required,
          "exploration_filter_pass":seeds.len()>=required&&sets.len()>=required,
          "selection_status":"predeclared exploration filter; not a cost ranking or significance test"}));
    }
    Ok(
        json!({"schema":"factor_base_lead_audit/v1","sealed_attempts":sealed,"point_witness_checks":witnesses,
      "total_trial_records":trials,"distinct_point_sets":identities.len(),"candidates":rows,
      "representation_comparisons":representation,"exploration_filters":filters,
      "pricing":"Incomplete on all arms; no symmetric-band total ranking",
      "timing":"operational shared-host telemetry only","certified_degree_of_regularity":null,"full_pipeline_speedup":null}),
    )
}
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().is_some_and(|s| s == "--coverage") {
        if args.len() < 4 {
            return Err(
                "usage: factor-base-lead-audit --coverage RUNS_ROOT REGISTRY PANEL...".into(),
            );
        }
        let v = coverage::coverage(Path::new(&args[1]), Path::new(&args[2]), &args[3..])?;
        println!("{}", serde_json::to_string_pretty(&v)?);
        return Ok(());
    }
    if args.len() < 2 {
        return Err("usage: factor-base-lead-audit RUNS_ROOT PANEL...".into());
    }
    let v = audit(Path::new(&args[0]), &args[1..])?;
    println!("{}", serde_json::to_string_pretty(&v)?);
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn failed_inventory_is_missing_yield_rather_than_a_zero_yield_completion() {
        let row = failed_row(
            "secondary",
            &json!({"id":"id","label":"bad","outcome":"failed"}),
            &json!({"factor_base":{"columns":0}}),
            &json!({"candidate":{}}),
            &json!({"fatal":{"message":"empty inventory"}}),
            "bad",
        );
        assert!(row["holdout"].is_null());
        assert!(row["training_independent_rank"].is_null());
        assert_eq!(row["failure"]["message"], "empty inventory");
        assert_eq!(row["outcome"], "failed");
    }
    #[test]
    fn distinct_targets_do_not_count_repeated_draws_as_new_evidence() {
        let t = json!({"phase":"holdout","target":[1,2],"ground_truth_decomposable":true,
          "witness_verified":true,"observed_solving_degree":2,"field_reduction_multiplications":7});
        let p = phase(&[t.clone(), t], "holdout").unwrap();
        assert_eq!(p["verified"], 2);
        assert_eq!(p["distinct_verified_targets"], 1);
        assert_eq!(p["successful_field_reduction_multiplications"], 14);
    }
    #[test]
    fn missing_operation_count_is_not_silently_zero() {
        assert!(phase(&[json!({"phase":"train","target":[1,2]})], "train").is_err());
    }
    #[test]
    fn point_set_comparison_ignores_column_order_but_not_coordinates() {
        let a = json!({"points":[{"x":"0x1","y":"0x2","col":0},{"x":"0x3","y":"0x4","col":1}]});
        let b = json!({"points":[{"x":"0x3","y":"0x4","col":0},{"x":"0x1","y":"0x2","col":1}]});
        assert_eq!(points(&a).unwrap(), points(&b).unwrap());
    }
    #[test]
    fn changed_targets_fail_closed_in_representation_comparison() {
        let a = json!({"candidate":{"label":"a"},"trials":[{"phase":"train","trial":0,"target":[1,2],"status":"refuted"}]});
        let b = json!({"candidate":{"label":"b"},"trials":[{"phase":"train","trial":0,"target":[2,3],"status":"refuted"}]});
        assert!(representation_comparison(&a, &b).is_err());
    }
}
