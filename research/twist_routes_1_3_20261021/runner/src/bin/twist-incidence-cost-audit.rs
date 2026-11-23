//! Independent raw-Ir, complete finite-root and PARI checks of box costs.
#[path = "../../../../../src/hash/sha256.rs"]
mod source_sha256;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs,
    path::Path,
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn read(p: impl AsRef<Path>) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn sha(p: impl AsRef<Path>) -> Result<String> {
    Ok(hex::encode(source_sha256::sha256(&fs::read(p)?)))
}
fn ir(s: &str) -> Option<u64> {
    if !s
        .lines()
        .any(|l| l.split_whitespace().collect::<Vec<_>>() == ["events:", "Ir"])
    {
        return None;
    }
    s.lines()
        .rev()
        .find_map(|l| l.strip_prefix("totals:"))
        .or_else(|| s.lines().rev().find_map(|l| l.strip_prefix("summary:")))
        .and_then(|s| s.trim().parse().ok())
}
fn pow(mut x: u64, mut n: u64, p: u64) -> u64 {
    let mut y = 1;
    while n != 0 {
        if n & 1 != 0 {
            y = y * x % p;
        }
        x = x * x % p;
        n >>= 1;
    }
    y
}
fn eval(poly: &Value, x: &[u64], p: u64) -> Result<u64> {
    let mut sum = 0;
    for term in poly.as_array().ok_or("polynomial")? {
        let exps = term[0].as_array().ok_or("exponents")?;
        if exps.len() != x.len() {
            return Err("polynomial variable count".into());
        }
        let mut value = term[1].as_u64().ok_or("coefficient")?;
        if value >= p {
            return Err("coefficient outside field".into());
        }
        for (e, x) in exps.iter().zip(x) {
            value = value * pow(*x, e.as_u64().ok_or("exponent")?, p) % p;
        }
        sum = (sum + value) % p;
    }
    Ok(sum)
}
fn finite_roots(r: &Value, truth: &Value, task: &Value) -> Result<bool> {
    let arm = task["arm"].as_str().unwrap();
    if arm == "group-mitm" {
        return Ok(r["equations"] == json!([]) && r["solutions"].is_null());
    }
    let Some(solutions) = r["solutions"].as_array() else {
        return Ok(false);
    };
    let p = task["fixture"]["p"].as_u64().unwrap();
    let h = task["h"].as_u64().unwrap();
    let k = task["k"].as_u64().unwrap();
    let lambda = task["lambda"].as_u64().unwrap();
    let base: Vec<u64> = serde_json::from_value(truth["base_x"].clone())?;
    let us: Vec<u64> = serde_json::from_value(truth["U"].clone())?;
    let mut unique = BTreeSet::new();
    let pairs = truth["canonical_pairs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| serde_json::from_value::<Vec<u64>>(p.clone()).unwrap())
        .collect::<BTreeSet<_>>();
    for s in solutions {
        let s: Vec<u64> = serde_json::from_value(s.clone())?;
        if s.iter().any(|x| *x >= p) || !unique.insert(s.clone()) {
            return Ok(false);
        }
        let mut t = if arm == "direct-product" {
            if s.len() != 2 {
                return Ok(false);
            }
            s.clone()
        } else {
            if s.len() != 4
                || s[0] >= h
                || s[2] >= h
                || s[1] >= k
                || s[3] >= k
                || !us.contains(&s[1])
                || !us.contains(&s[3])
            {
                return Ok(false);
            }
            vec![
                (s[0] + p - lambda * s[1] % p) % p,
                (s[2] + p - lambda * s[3] % p) % p,
            ]
        };
        if !t.iter().all(|x| base.contains(x)) {
            return Ok(false);
        }
        t.sort();
        if !pairs.contains(&t) {
            return Ok(false);
        }
        for q in r["equations"].as_array().ok_or("equations")? {
            if eval(q, &s, p)? != 0 {
                return Ok(false);
            }
        }
    }
    let wanted = if arm == "direct-product" {
        pairs
            .iter()
            .map(|t| if t[0] == t[1] { 1 } else { 2 })
            .sum::<u64>()
    } else {
        truth["F3_pair_image_solutions"].as_u64().unwrap()
    };
    Ok(unique.len() as u64 == wanted)
}
fn main() -> Result<()> {
    let a = std::env::args().collect::<Vec<_>>();
    if a.len() != 6 {
        return Err("usage: PLAN RUN CAS_PREFIX FRONTIER_ADMISSION FRESH_OUT".into());
    }
    let plan = read(&a[1])?;
    let run = Path::new(&a[2]);
    let prefix = &a[3];
    let out = Path::new(&a[5]);
    if out.exists() {
        return Err("fresh audit required".into());
    }
    let admission = read(&a[4])?;
    if admission["accepted"] != true || admission["case_count"] != 600 {
        return Err("frontier not independently admitted".into());
    }
    let rows = read(run.join("RECORDS.json"))?;
    let cells = plan["cells"].as_array().ok_or("plan cells")?;
    if rows.as_array().ok_or("rows")?.len() != cells.len() {
        return Err("missing records".into());
    }
    let mut by_arm = BTreeMap::<String, [usize; 3]>::new();
    let mut costs = BTreeMap::<String, BTreeMap<String, u64>>::new();
    let mut targets = BTreeSet::new();
    let mut configs = BTreeSet::new();
    let mut checks = vec![];
    let mut replay_count = 0usize;
    let mut accepted_count = 0usize;
    let mut failure_reasons = BTreeMap::<String, usize>::new();
    for (cell, row) in cells.iter().zip(rows.as_array().unwrap()) {
        for (k, v) in cell.as_object().unwrap() {
            if row[k] != *v {
                return Err("frozen plan changed".into());
            }
        }
        let arm = cell["arm"].as_str().unwrap();
        let workload = cell["workload"].as_str().unwrap();
        let count = by_arm.entry(arm.into()).or_default();
        count[0] += 1;
        configs.insert(workload.to_string());
        targets.insert((
            cell["fixture"].to_string(),
            cell["task"]["target"].to_string(),
        ));
        if row["status"] == "not-started-global-cap" {
            *failure_reasons
                .entry("not-started-global-cap".into())
                .or_default() += 1;
            continue;
        }
        count[1] += 1;
        let dir = run.join(format!("cell-{:03}", row["index"].as_u64().unwrap()));
        for (key, name) in [
            ("task_sha256", "task.json"),
            ("output_sha256", "worker.json"),
            ("trace_sha256", "worker.callgrind"),
            ("stdout_sha256", "worker.stdout"),
            ("stderr_sha256", "worker.stderr"),
        ] {
            let file = dir.join(name);
            if file.exists() {
                if row[key] != sha(&file)? {
                    return Err("raw hash mismatch".into());
                }
            } else if !row[key].is_null() {
                return Err("hashed file missing".into());
            }
        }
        let task = read(dir.join("task.json"))?;
        if task != cell["task"] {
            return Err("task mismatch".into());
        }
        if task.get("base_x").is_some()
            || task.get("expected").is_some()
            || task.get("canonical_pairs").is_some()
        {
            return Err("answer leaked into task".into());
        }
        let expected = Path::new(cell["expected"].as_str().unwrap());
        if sha(expected)? != cell["expected_sha256"] {
            return Err("frontier changed".into());
        }
        let truth = read(expected)?;
        let raw = fs::read_to_string(dir.join("worker.callgrind"))
            .ok()
            .and_then(|s| ir(&s));
        if row["guest_instructions_Ir"] != json!(raw) {
            return Err("raw Ir mismatch".into());
        }
        let result = read(dir.join("worker.json")).ok();
        let mut basic = false;
        let mut roots = false;
        if let Some(r) = &result {
            basic = row["exit_code"] == 0
                && r["complete_projection"] == true
                && r["canonical_pairs"] == truth["canonical_pairs"]
                && r["explicit_y_witnesses"] == truth["explicit_y_witnesses"]
                && r["U"] == truth["U"]
                && r["target"] == task["target"]
                && r["arm"] == task["arm"];
            if basic {
                replay_count += r["explicit_y_witnesses"].as_array().unwrap().len();
                roots = finite_roots(r, &truth, &task)?;
            }
        }
        if row["projection_matched"] != basic {
            return Err("parent acceptance mismatch".into());
        }
        let accepted = basic && roots && raw.is_some();
        if accepted {
            count[2] += 1;
            accepted_count += 1;
            costs
                .entry(workload.into())
                .or_default()
                .insert(arm.into(), raw.unwrap());
        } else {
            let reason = if row["exit_code"] == 124 {
                "hard-timeout"
            } else if row["exit_code"] != 0 {
                "worker-process-failed-or-killed"
            } else if result.as_ref().is_some_and(|r| r["timed_out"] == true) {
                "cooperative-budget"
            } else if result
                .as_ref()
                .is_some_and(|r| r["complete_projection"] != true)
            {
                "solver-undetermined"
            } else if !basic {
                "projection-or-signed-y-mismatch"
            } else if !roots {
                "finite-root-coverage-failed"
            } else {
                "raw-instruction-total-missing"
            };
            *failure_reasons.entry(reason.into()).or_default() += 1;
        }
        checks.push(json!({"index":row["index"],"projection_and_signed_y_matched":basic,"complete_independent_finite_root_count_matched":roots,"Ir":raw,"accepted_after_replay":accepted}));
    }
    let meta = read(format!("{prefix}.json"))?;
    let stdout = fs::read_to_string(format!("{prefix}.stdout"))?;
    let stderr = fs::read_to_string(format!("{prefix}.stderr"))?;
    if meta["exit_code"] != 0
        || meta["script_sha256"] != sha(run.join("verify.gp"))?
        || meta["stdout_sha256"] != sha(format!("{prefix}.stdout"))?
        || meta["stderr_sha256"] != sha(format!("{prefix}.stderr"))?
        || !stderr
            .lines()
            .all(|s| s.trim().is_empty() || s.contains("Warning: new maximum stack size ="))
    {
        return Err("CAS receipt/error gate".into());
    }
    if !stdout
        .lines()
        .any(|s| s == format!("ALL_INCIDENCE_COST_REPLAY_OK {replay_count} {replay_count}"))
        || replay_count == 0
    {
        return Err("CAS coverage marker".into());
    }
    let mut aa = vec![];
    let mut paired = vec![];
    for case in &configs {
        let empty = BTreeMap::new();
        let c = costs.get(case).unwrap_or(&empty);
        let mut gates = BTreeMap::new();
        for (control, repeat) in [
            ("direct-product", "direct-aa"),
            ("reduced-u", "reduced-u-aa"),
            ("group-mitm", "group-aa"),
        ] {
            let spread = c
                .get(control)
                .zip(c.get(repeat))
                .map(|(x, y)| x.abs_diff(*y) as f64 / ((*x as f64 + *y as f64) / 2.));
            let pass = spread.is_some_and(|s| s <= 0.05);
            gates.insert(control, pass);
            aa.push(
                json!({"case":case,"control":control,"repeat":repeat,"spread":spread,"pass":pass}),
            );
        }
        gates.insert("full-mask", gates["direct-product"]);
        for (candidate, control) in [
            ("full-mask", "direct-product"),
            ("reduced-u", "direct-product"),
            ("direct-product", "group-mitm"),
            ("full-mask", "group-mitm"),
            ("reduced-u", "group-mitm"),
        ] {
            let x = c.get(candidate);
            let y = c.get(control);
            let accepted = gates[candidate] && gates[control] && x.is_some() && y.is_some();
            paired.push(json!({"case":case,"candidate":candidate,"control":control,"paired_accepted":accepted,"candidate_Ir":x,"control_Ir":y,"ratio":if accepted{Some(*x.unwrap()as f64/ *y.unwrap()as f64)}else{None},"fewer":accepted&&x<y}));
        }
    }
    let mut distributions = vec![];
    for (candidate, control) in [
        ("full-mask", "direct-product"),
        ("reduced-u", "direct-product"),
        ("direct-product", "group-mitm"),
        ("full-mask", "group-mitm"),
        ("reduced-u", "group-mitm"),
    ] {
        let ps = paired
            .iter()
            .filter(|r| {
                r["candidate"] == candidate
                    && r["control"] == control
                    && r["paired_accepted"] == true
            })
            .collect::<Vec<_>>();
        let mut ratios = ps
            .iter()
            .map(|r| r["ratio"].as_f64().unwrap())
            .collect::<Vec<_>>();
        ratios.sort_by(f64::total_cmp);
        let n = ratios.len();
        let median = if n == 0 {
            None
        } else if n % 2 == 0 {
            Some((ratios[n / 2 - 1] + ratios[n / 2]) / 2.)
        } else {
            Some(ratios[n / 2])
        };
        distributions.push(json!({"candidate":candidate,"control":control,"requested_pairs":configs.len(),"accepted_pairs":n,"fewer_instruction_pairs":ps.iter().filter(|r|r["fewer"]==true).count(),"ratio_min":ratios.first(),"ratio_median":median,"ratio_max":ratios.last(),"descriptive_only":true}));
    }
    fs::create_dir_all(out)?;
    let result = json!({"schema":"twist.incidence-cost-independent-audit/v1","requested_cells":cells.len(),"accepted_cells":accepted_count,"configurations":configs.len(),"distinct_fixture_target_pairs":targets.len(),"independent_PARI_positive_and_negative_checks_each":replay_count,"by_arm":by_arm.iter().map(|(arm,c)|json!({"arm":arm,"requested":c[0],"attempted":c[1],"accepted":c[2]})).collect::<Vec<_>>(),"failure_reasons":failure_reasons,"AA":aa,"comparisons":paired,"distributions":distributions,"plan_sha256":sha(&a[1])?,"records_sha256":sha(run.join("RECORDS.json"))?,"frontier_admission_sha256":sha(&a[4])?,"CAS_receipt_sha256":sha(format!("{prefix}.json"))?,"unit":"Callgrind simulated guest instructions Ir","certifies_complete_Fp_answers_not_algebraic_closure_bases":true,"full_600_cost_grid_complete":false,"complete_IC_cost_or_fastest_or_novelty_claim":false});
    fs::write(
        out.join("AUDIT.json"),
        serde_json::to_string_pretty(&result)? + "\n",
    )?;
    fs::write(
        out.join("RAW_CHECKS.json"),
        serde_json::to_string_pretty(&checks)? + "\n",
    )?;
    println!(
        "{}",
        json!({"accepted_cells":accepted_count,"checks_each":replay_count,"by_arm":result["by_arm"],"distributions":distributions})
    );
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn incomplete_profiles_are_not_costs() {
        assert_eq!(ir("events: Ir\n"), None);
        assert_eq!(ir("events: Ir Dr\ntotals: 9 1\n"), None);
        assert_eq!(ir("events: Ir\nsummary: 2\ntotals: 7\n"), Some(7));
    }
}
