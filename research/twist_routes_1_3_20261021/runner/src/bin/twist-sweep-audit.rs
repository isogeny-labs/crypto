//! Reopen raw sweep receipts and earlier independent certificates before analysis.
#[path = "../../../../../src/hash/sha256.rs"]
mod source_sha256;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs,
    path::{Path, PathBuf},
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn read(p: impl AsRef<Path>) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn sha(p: impl AsRef<Path>) -> Result<String> {
    Ok(hex::encode(source_sha256::sha256(&fs::read(p)?)))
}
fn leaf(root: &Path, id: &str, s: &str) -> PathBuf {
    root.join("cells").join(format!("{id}.{s}"))
}
fn instructions(raw: &str) -> Result<u64> {
    let fields = raw
        .lines()
        .find(|l| l.starts_with("events:"))
        .ok_or("no events")?
        .split_whitespace()
        .collect::<Vec<_>>();
    if fields != ["events:", "Ir"] {
        return Err("unexpected event vector".into());
    }
    let line = raw
        .lines()
        .filter(|l| l.starts_with("totals:"))
        .last()
        .or_else(|| raw.lines().filter(|l| l.starts_with("summary:")).last())
        .ok_or("no instruction total")?;
    let fields = line.split_whitespace().collect::<Vec<_>>();
    if fields.len() != 2 {
        return Err("malformed instruction total".into());
    }
    Ok(fields[1].parse()?)
}
fn certified(panel: &Path, job: &Value, old: &Value) -> Result<bool> {
    let id = job["id"].as_str().ok_or("job")?;
    if old["certificate"]["accepted"] != true {
        return Ok(false);
    }
    if old["input_sha256"] != sha(leaf(panel, id, "input.json"))?
        || old["result_sha256"] != sha(leaf(panel, id, "native.json"))?
    {
        return Err("changed old input/result".into());
    }
    let c = &old["certificate"];
    for (field, suffix) in [
        ("input_sha256", "certificate.sing"),
        ("stdout_sha256", "certificate.stdout"),
        ("stderr_sha256", "certificate.stderr"),
    ] {
        if c[field] != sha(leaf(panel, id, suffix))? {
            return Err("changed old certificate".into());
        }
    }
    let raw = fs::read_to_string(leaf(panel, id, "certificate.stdout"))?;
    let err = fs::read_to_string(leaf(panel, id, "certificate.stderr"))?;
    let dim = raw
        .lines()
        .find_map(|s| s.strip_prefix("VDIM="))
        .ok_or("dimension missing")?
        .parse::<u64>()?;
    Ok(c["exit_code"] == 0
        && raw.lines().any(|s| s == "CERT=1")
        && raw.lines().any(|s| s == "E26_CERTIFICATE_DONE")
        && !raw.contains('?')
        && err.trim().is_empty()
        && job["expected"] == dim)
}
fn median(v: &mut [f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(f64::total_cmp);
    let n = v.len();
    Some(if n % 2 == 0 {
        (v[n / 2 - 1] + v[n / 2]) / 2.
    } else {
        v[n / 2]
    })
}
fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 4 {
        return Err("usage: audit PLAN SWEEP FRESH_OUT".into());
    }
    let plan = read(&args[1])?;
    let sweep = Path::new(&args[2]);
    let out = Path::new(&args[3]);
    if out.exists() {
        return Err("fresh audit required".into());
    }
    let inputs = Path::new(plan["inputs"].as_str().ok_or("inputs")?);
    let panel = Path::new(plan["panel"].as_str().ok_or("panel")?);
    if sha(inputs.join("JOBS.json"))? != plan["catalog_sha256"]
        || sha(panel.join("PANEL.json"))? != plan["parent_panel_sha256"]
    {
        return Err("parent changed".into());
    }
    let catalog = read(inputs.join("JOBS.json"))?;
    let jobs = catalog
        .as_array()
        .unwrap()
        .iter()
        .map(|j| (j["id"].as_str().unwrap().to_string(), j))
        .collect::<BTreeMap<_, _>>();
    let source = read(panel.join("PANEL.json"))?;
    let rows = read(sweep.join("RECORDS.json"))?;
    if rows.as_array().unwrap().len() != 480 || plan["cells"].as_array().unwrap().len() != 480 {
        return Err("not all requested records present".into());
    }
    let mut totals = BTreeMap::<String, [u64; 4]>::new();
    let mut by_case = BTreeMap::<String, BTreeMap<String, u64>>::new();
    let mut case_info = BTreeMap::new();
    let mut targets = BTreeSet::new();
    let mut checked = vec![];
    for (expected, row) in plan["cells"]
        .as_array()
        .unwrap()
        .iter()
        .zip(rows.as_array().unwrap())
    {
        for k in [
            "index",
            "workload",
            "fixture",
            "p",
            "B",
            "arity",
            "arm",
            "source_arm",
            "jobs",
        ] {
            if expected[k] != row[k] {
                return Err(format!("schedule field changed: {k}").into());
            }
        }
        let arm = row["arm"].as_str().unwrap();
        let count = totals.entry(arm.into()).or_default();
        count[0] += 1;
        let work = row["workload"].as_str().unwrap();
        case_info.insert(
            work.to_string(),
            json!({"p":row["p"],"B":row["B"],"arity":row["arity"],"fixture":row["fixture"]}),
        );
        targets.insert(row["fixture"].as_str().unwrap().to_string());
        if row["status"] == "not-started-global-time-or-output-cap" {
            continue;
        }
        count[1] += 1;
        let dir = sweep.join(format!("cell-{:03}", row["index"].as_u64().unwrap()));
        for (key, file) in [
            ("capsule_sha256", "capsule.json"),
            ("trace_sha256", "worker.callgrind"),
            ("output_sha256", "worker.json"),
            ("stdout_sha256", "worker.stdout"),
            ("stderr_sha256", "worker.stderr"),
        ] {
            let p = dir.join(file);
            if p.exists() {
                if row[key] != sha(&p)? {
                    return Err(format!("receipt hash mismatch: {}", p.display()).into());
                }
            } else if !row[key].is_null() {
                return Err("receipt names missing file".into());
            }
        }
        let payload = read(dir.join("capsule.json"))?;
        if payload.as_array().unwrap().len() != row["jobs"].as_array().unwrap().len() {
            return Err("missing branch".into());
        }
        let native = read(dir.join("worker.json")).ok();
        let mut complete = row["exit_code"] == 0
            && native.as_ref().is_some_and(|n| {
                n.as_array()
                    .is_some_and(|a| a.len() == payload.as_array().unwrap().len())
            });
        let prior = source
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["workload"] == row["workload"] && r["arm"] == row["source_arm"])
            .ok_or("missing old candidate")?;
        let mut verified = complete;
        for (i, job) in payload.as_array().unwrap().iter().enumerate() {
            let id = job["id"].as_str().unwrap();
            if job != *jobs.get(id).ok_or("unknown job")? || row["jobs"][i] != id {
                return Err("capsule job mismatch".into());
            }
            let n = native
                .as_ref()
                .and_then(|n| n.as_array())
                .and_then(|a| a.get(i));
            let done = n.is_some_and(|n| {
                n["job"] == id && n["timed_out"] == false && n["pairs_above_bound"] == 0
            });
            complete &= done;
            let b = prior["branches"]
                .as_array()
                .unwrap()
                .iter()
                .find(|b| b["job"] == id);
            let old = read(leaf(panel, id, "native.json")).ok();
            let matches = n.zip(old.as_ref()).is_some_and(|(n, o)| {
                n["basis"] == o["basis"]
                    && n["field_multiplications_row_reduction"]
                        == o["field_multiplications_row_reduction"]
            });
            let cert = match b {
                Some(b) => certified(panel, job, b)?,
                None => false,
            };
            verified &= done
                && matches
                && cert
                && read(leaf(panel, id, "input.json")).is_ok_and(|v| v == *job);
        }
        let ir = fs::read_to_string(dir.join("worker.callgrind"))
            .ok()
            .and_then(|s| instructions(&s).ok());
        if row["guest_instructions_Ir"] != json!(ir) {
            return Err("raw instruction count mismatch".into());
        }
        let accepted = verified && ir.is_some();
        if row["accepted"] != accepted || row["worker_finished"] != complete {
            return Err("acceptance mismatch".into());
        }
        count[2] += complete as u64;
        count[3] += accepted as u64;
        if accepted {
            by_case
                .entry(work.into())
                .or_default()
                .insert(arm.into(), ir.unwrap());
        }
        checked.push(json!({"index":row["index"],"raw_instruction_total":ir,"verified":accepted,"complete":complete}));
    }
    let mut aa = vec![];
    let mut comparisons = vec![];
    for (work, info) in &case_info {
        let empty = BTreeMap::new();
        let costs = by_case.get(work).unwrap_or(&empty);
        let pair = costs.get("direct-aa").zip(costs.get("direct-chain"));
        let spread = pair.map(|(a, b)| a.abs_diff(*b) as f64 / ((*a as f64 + *b as f64) / 2.));
        let pass = spread.is_some_and(|s| s <= 0.05);
        aa.push(json!({"workload":work,"spread":spread,"within_5_percent":pass}));
        for candidate in ["frobenius-block", "hybrid-eight", "first-pair-invariant"] {
            for control in [
                "direct-chain",
                "ordinary-prefix-order",
                "factored-control",
                "bits-control",
            ] {
                let c = costs.get(candidate).copied();
                let b = costs.get(control).copied();
                let paired = pass && c.is_some() && b.is_some();
                comparisons.push(json!({"workload":work,"fixture":info["fixture"],"p":info["p"],"B":info["B"],"arity":info["arity"],"candidate":candidate,"control":control,"paired_accepted":paired,"candidate_Ir":c,"control_Ir":b,"candidate_to_control_ratio":if paired{Some(c.unwrap() as f64/b.unwrap() as f64)}else{None},"candidate_uses_fewer_instructions":paired&&c<b,"AA_pass":pass,"within_AA_absolute_difference":if paired{pair.map(|(x,y)|c.unwrap().abs_diff(b.unwrap())<=x.abs_diff(*y))}else{None}}));
            }
        }
    }
    let mut distributions = vec![];
    for candidate in ["frobenius-block", "hybrid-eight", "first-pair-invariant"] {
        for control in [
            "direct-chain",
            "ordinary-prefix-order",
            "factored-control",
            "bits-control",
        ] {
            let paired = comparisons
                .iter()
                .filter(|r| {
                    r["candidate"] == candidate
                        && r["control"] == control
                        && r["paired_accepted"] == true
                })
                .collect::<Vec<_>>();
            let mut ratios = paired
                .iter()
                .map(|r| r["candidate_to_control_ratio"].as_f64().unwrap())
                .collect::<Vec<_>>();
            let med = median(&mut ratios);
            distributions.push(json!({"candidate":candidate,"control":control,"requested_pairs":60,"accepted_pairs":paired.len(),"fewer_instruction_pairs":paired.iter().filter(|r|r["candidate_uses_fewer_instructions"]==true).count(),"ratios_min":ratios.first(),"ratios_median":med,"ratios_max":ratios.last(),"descriptive_correlated_training_configurations":true}));
        }
    }
    fs::create_dir_all(out)?;
    let summary = read(sweep.join("SUMMARY.json"))?;
    let accepted: u64 = totals.values().map(|r| r[3]).sum();
    if summary["accepted"] != accepted {
        return Err("primary aggregate mismatch".into());
    }
    let report = json!({"schema":"twist.sweep-independent-audit/v1","status":"all-raw-receipts-and-prior-completeness-checked","requested_cells":480,"workloads":case_info.len(),"distinct_fixture_target_pairs":targets.len(),"accepted_cells":accepted,"by_arm":totals.iter().map(|(arm,c)|json!({"arm":arm,"requested":c[0],"attempted":c[1],"worker_finished":c[2],"certified_output_matched":c[3]})).collect::<Vec<_>>(),"AA":aa,"comparisons":comparisons,"distributions":distributions,"records_sha256":sha(sweep.join("RECORDS.json"))?,"plan_sha256":sha(&args[1])?,"summary_sha256":sha(sweep.join("SUMMARY.json"))?,"wall_time_admitted":false,"complete_IC_cost":false,"novelty_or_speedup_established":false,"new_targets_opened":false});
    fs::write(
        out.join("AUDIT.json"),
        serde_json::to_string_pretty(&report)? + "\n",
    )?;
    fs::write(
        out.join("RAW_CHECKS.json"),
        serde_json::to_string_pretty(&checked)? + "\n",
    )?;
    println!(
        "{}",
        json!({"status":"verified","accepted_cells":accepted,"AA_passes":aa.iter().filter(|r|r["within_5_percent"]==true).count(),"distributions":distributions})
    );
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn incomplete_and_multi_event_traces_are_not_single_Ir_totals() {
        assert!(instructions("events: Ir Dr\ntotals: 2 1\n").is_err());
        assert!(instructions("events: Ir\n").is_err());
        assert_eq!(
            instructions("events: Ir\nsummary: 2\ntotals: 7\n").unwrap(),
            7
        );
    }
}
