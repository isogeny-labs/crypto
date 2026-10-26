//! Independent raw-counter, task/output and PARI-replay audit of worker intervals.
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
fn ir(s: &str) -> Option<u64> {
    let mut event = false;
    let mut summary = None;
    let mut total = None;
    for line in s.lines() {
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.first() == Some(&"events:") {
            event = fields == ["events:", "Ir"];
        }
        if fields.len() == 2 && fields[0] == "summary:" {
            summary = fields[1].parse::<u64>().ok();
        }
        if fields.len() == 2 && fields[0] == "totals:" {
            total = fields[1].parse::<u64>().ok();
        }
    }
    if event {
        total.or(summary)
    } else {
        None
    }
}
fn main() -> Result<()> {
    let a = std::env::args().collect::<Vec<_>>();
    if a.len() != 6 {
        return Err(
            "usage: audit decomposition|transfer PLAN RUN CAS_LOG_PREFIX FRESH_OUTPUT".into(),
        );
    }
    let mode = &a[1];
    if mode != "decomposition" && mode != "transfer" {
        return Err("unknown interval".into());
    }
    let plan = read(&a[2])?;
    let run = Path::new(&a[3]);
    let out = Path::new(&a[5]);
    if out.exists() {
        return Err("fresh audit required".into());
    }
    let rows = read(run.join("RECORDS.json"))?;
    let expected = plan["cells"].as_array().ok_or("plan")?;
    if rows.as_array().ok_or("records")?.len() != expected.len() {
        return Err("missing planned cells".into());
    }
    let mut totals = BTreeMap::<String, [u64; 3]>::new();
    let mut costs = BTreeMap::<String, BTreeMap<String, u64>>::new();
    let mut info = BTreeMap::<String, Value>::new();
    let mut targets = BTreeSet::new();
    let mut replay_checks = 0;
    let mut accepted_cells = 0;
    let mut matched_cells = 0;
    let mut raw_checks = vec![];
    for (cell, row) in expected.iter().zip(rows.as_array().unwrap()) {
        for (k, v) in cell.as_object().unwrap() {
            if row[k] != *v {
                return Err(format!("frozen plan differs in {k}").into());
            }
        }
        let arm = row["arm"].as_str().unwrap();
        let count = totals.entry(arm.into()).or_default();
        count[0] += 1;
        let key = if mode == "decomposition" {
            row["workload"].as_str().unwrap().to_string()
        } else {
            format!(
                "{}-l{}",
                row["fixture"].as_str().unwrap(),
                row["label"].as_u64().unwrap()
            )
        };
        info.insert(key.clone(),json!({"fixture":row["fixture"],"p":row["task"]["fixture"]["p"],"B":row["B"],"arity":row["arity"],"label":row["label"]}));
        targets.insert((
            row["fixture"].as_str().unwrap().to_string(),
            row["task"]["target"].to_string(),
        ));
        if row["status"] == "not-started-global-cap" {
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
            let p = dir.join(name);
            if p.exists() {
                if row[key] != sha(&p)? {
                    return Err(format!("file hash mismatch: {}", p.display()).into());
                }
            } else if !row[key].is_null() {
                return Err("hashed file absent".into());
            }
        }
        if read(dir.join("task.json"))? != row["task"] {
            return Err("worker input not frozen task".into());
        }
        if row["task"].get("expected_scalar").is_some()
            || row["task"].get("canonical_projected_tuples").is_some()
        {
            return Err("expected answer leaked into task".into());
        }
        let result = read(dir.join("worker.json")).ok();
        let instructions = fs::read_to_string(dir.join("worker.callgrind"))
            .ok()
            .and_then(|s| ir(&s));
        if row["guest_instructions_Ir"] != json!(instructions) {
            return Err("raw Ir differs".into());
        }
        let mut matched = row["exit_code"] == 0 && result.is_some();
        if let Some(r) = &result {
            matched &= r["target"] == row["task"]["target"] && r["arm"] == row["task"]["arm"];
            if mode == "decomposition" {
                let p = Path::new(row["expected"].as_str().unwrap());
                if sha(p)? != row["expected_sha256"] {
                    return Err("frozen census changed".into());
                }
                let truth = read(p)?;
                matched &= r["complete_projection"] == true
                    && r["canonical_projected_tuples"] == truth["canonical_projected_tuples"]
                    && r["base"] == truth["base"];
                if plan["strict_signed_witness_equality"] == true {
                    matched &= r["signed_witnesses"] == truth["witnesses"]
                        && r["lift_method"] == row["task"]["lift_method"];
                }
                let branches = r["branches"].as_array().ok_or("branches")?;
                let wanted = match row["task"]["arm"].as_str().unwrap() {
                    "direct-chain" => 1,
                    "hybrid-eight" => 8,
                    "first-pair-invariant" => 2,
                    "group-mitm" => 0,
                    _ => return Err("unexpected decomposition arm".into()),
                };
                matched &= branches.len() == wanted
                    && branches
                        .iter()
                        .all(|b| b["solutions"].is_array() && b["timed_out"] == false);
                if matched {
                    let witnesses = r["signed_witnesses"].as_array().ok_or("witnesses")?;
                    let xs = witnesses.iter().map(|w| w["x"].clone()).collect::<Vec<_>>();
                    if json!(xs) != r["canonical_projected_tuples"] {
                        return Err("witness coverage differs".into());
                    }
                    replay_checks += witnesses.len();
                }
                if row["projection_matched"] != matched {
                    return Err("primary projection acceptance differs".into());
                }
            } else {
                if sha(Path::new(row["constructor_path"].as_str().unwrap()))?
                    != row["constructor_sha256"]
                {
                    return Err("public map changed".into());
                }
                matched &= r["source_scalar_verified"] == true
                    && r["source_scalar"] == row["expected_scalar"];
                if matched {
                    if row["arm"].as_str().unwrap().starts_with("transfer")
                        && (r["details"]["destination_constructed"] != true
                            || r["details"]["source_point_recovered_by_inverse_three"]
                                != row["task"]["target"])
                    {
                        return Err("missing transferred recovery".into());
                    }
                    replay_checks += 1;
                }
                if row["accepted_by_parent"] != (matched && instructions.is_some()) {
                    return Err("primary transfer acceptance differs".into());
                }
            }
        } else if row[if mode == "decomposition" {
            "projection_matched"
        } else {
            "accepted_by_parent"
        }] == true
        {
            return Err("accepted missing result".into());
        }
        matched_cells += matched as usize;
        let accepted = matched && instructions.is_some();
        if accepted {
            accepted_cells += 1;
            count[2] += 1;
            costs
                .entry(key.clone())
                .or_default()
                .insert(arm.into(), instructions.unwrap());
        }
        raw_checks.push(json!({"index":row["index"],"worker_output_matched":matched,"Ir":instructions,"accepted_after_replay":accepted}));
    }
    // Fresh independent source-group replay: require actual captured exit and hashes.
    let prefix = &a[4];
    let meta = read(format!("{prefix}.json"))?;
    let stdout = fs::read_to_string(format!("{prefix}.stdout"))?;
    let stderr = fs::read_to_string(format!("{prefix}.stderr"))?;
    if meta["exit_code"] != 0
        || meta["stdout_sha256"] != sha(format!("{prefix}.stdout"))?
        || meta["stderr_sha256"] != sha(format!("{prefix}.stderr"))?
        || meta["script_sha256"] != sha(run.join("verify.gp"))?
    {
        return Err("independent replay receipt failed".into());
    }
    if !stderr
        .lines()
        .all(|l| l.trim().is_empty() || l.contains("Warning: new maximum stack size ="))
    {
        return Err("independent replay errors".into());
    }
    let marker = if mode == "decomposition" {
        "ALL_DECOMPOSITION_REPLAY_OK"
    } else {
        "ALL_TRANSFER_COST_REPLAY_OK"
    };
    let line = stdout
        .lines()
        .find(|l| l.starts_with(marker))
        .ok_or("replay marker missing")?
        .split_whitespace()
        .collect::<Vec<_>>();
    if line.len() != 3
        || line[1].parse::<usize>()? != replay_checks
        || line[2].parse::<usize>()? != replay_checks
        || replay_checks == 0
    {
        return Err("independent replay coverage differs".into());
    }
    let mut comparisons = vec![];
    let mut aa = vec![];
    for (key, case) in &info {
        let empty = BTreeMap::new();
        let c = costs.get(key).unwrap_or(&empty);
        let (control, repeat) = if mode == "decomposition" {
            ("direct-chain", "direct-aa")
        } else {
            ("source-bsgs", "source-aa")
        };
        let spread = c
            .get(control)
            .zip(c.get(repeat))
            .map(|(x, y)| x.abs_diff(*y) as f64 / ((*x as f64 + *y as f64) / 2.));
        let mut pass = spread.is_some_and(|s| s <= 0.05);
        let transfer_spread = if mode == "transfer" {
            c.get("transfer-bsgs")
                .zip(c.get("transfer-aa"))
                .map(|(x, y)| x.abs_diff(*y) as f64 / ((*x as f64 + *y as f64) / 2.))
        } else {
            None
        };
        if mode == "transfer" {
            pass &= transfer_spread.is_some_and(|s| s <= 0.05);
        }
        let mut extra_aa = vec![];
        if let Some(pairs) = plan["AA_pairs"].as_array() {
            for pair in pairs {
                let control = pair[0].as_str().ok_or("AA control")?;
                let repeat = pair[1].as_str().ok_or("AA repeat")?;
                let spread = c
                    .get(control)
                    .zip(c.get(repeat))
                    .map(|(x, y)| x.abs_diff(*y) as f64 / ((*x as f64 + *y as f64) / 2.));
                let gate = spread.is_some_and(|s| s <= 0.05);
                pass &= gate;
                extra_aa
                    .push(json!({"control":control,"repeat":repeat,"spread":spread,"pass":gate}));
            }
        }
        aa.push(json!({"case":key,"source_or_direct_spread":spread,"transfer_spread":transfer_spread,"additional_pairs":extra_aa,"within_frozen_5_percent_gate":pass}));
        let pairs = if let Some(pairs) = plan["comparisons"].as_array() {
            pairs
                .iter()
                .map(|p| {
                    (
                        p["candidate"].as_str().unwrap(),
                        p["control"].as_str().unwrap(),
                    )
                })
                .collect::<Vec<_>>()
        } else if mode == "decomposition" {
            vec![("hybrid-eight", control), ("first-pair-invariant", control)]
        } else {
            vec![("transfer-bsgs", control)]
        };
        let mut seen_pairs = BTreeSet::new();
        for (candidate, control) in pairs {
            if !seen_pairs.insert((candidate, control)) {
                continue;
            }
            let p = c.get(candidate).zip(c.get(control));
            let paired = pass && p.is_some();
            comparisons.push(json!({"case":key,"info":case,"candidate":candidate,"control":control,"paired_accepted":paired,"candidate_Ir":c.get(candidate),"control_Ir":c.get(control),"candidate_to_control_ratio":if paired{p.map(|(x,y)|*x as f64/ *y as f64)}else{None},"candidate_uses_fewer_instructions":paired&&p.is_some_and(|(x,y)|x<y)}));
        }
    }
    let pairs = if let Some(pairs) = plan["comparisons"].as_array() {
        pairs
            .iter()
            .map(|p| {
                (
                    p["candidate"].as_str().unwrap(),
                    p["control"].as_str().unwrap(),
                )
            })
            .collect::<Vec<_>>()
    } else if mode == "decomposition" {
        vec![
            ("hybrid-eight", "direct-chain"),
            ("first-pair-invariant", "direct-chain"),
        ]
    } else {
        vec![("transfer-bsgs", "source-bsgs")]
    };
    let mut distributions = vec![];
    let mut seen_pairs = BTreeSet::new();
    for (candidate, control) in pairs {
        if !seen_pairs.insert((candidate, control)) {
            continue;
        }
        let pairs = comparisons
            .iter()
            .filter(|r| {
                r["candidate"] == candidate
                    && r["control"] == control
                    && r["paired_accepted"] == true
            })
            .collect::<Vec<_>>();
        let mut ratios = pairs
            .iter()
            .map(|r| r["candidate_to_control_ratio"].as_f64().unwrap())
            .collect::<Vec<_>>();
        ratios.sort_by(f64::total_cmp);
        let n = ratios.len();
        let median = if n == 0 {
            None
        } else {
            Some(if n % 2 == 0 {
                (ratios[n / 2 - 1] + ratios[n / 2]) / 2.
            } else {
                ratios[n / 2]
            })
        };
        distributions.push(json!({"candidate":candidate,"control":control,"requested_pairs":info.len(),"accepted_pairs":n,"fewer_instruction_pairs":pairs.iter().filter(|r|r["candidate_uses_fewer_instructions"]==true).count(),"ratio_min":ratios.first(),"ratio_median":median,"ratio_max":ratios.last(),"descriptive_only":true}));
    }
    fs::create_dir_all(out)?;
    let result = json!({"schema":"twist.interval-independent-audit/v1","status":"raw-counts-projections-or-scalars-and-independent-replay-verified","mode":mode,"requested_cells":expected.len(),"accepted_cells":accepted_cells,"matched_output_cells":matched_cells,"configurations":info.len(),"distinct_fixture_target_pairs":targets.len(),"independent_PARI_positive_and_negative_checks_each":replay_checks,"by_arm":totals.iter().map(|(arm,c)|json!({"arm":arm,"requested":c[0],"attempted":c[1],"accepted":c[2]})).collect::<Vec<_>>(),"AA":aa,"comparisons":comparisons,"distributions":distributions,"records_sha256":sha(run.join("RECORDS.json"))?,"plan_sha256":sha(&a[2])?,"CAS_receipt_sha256":sha(format!("{prefix}.json"))?,"unit":"Callgrind simulated guest instructions Ir","complete_IC_cost_or_fastest_claim":false,"wall_time_admitted":false});
    fs::write(
        out.join("AUDIT.json"),
        serde_json::to_string_pretty(&result)? + "\n",
    )?;
    fs::write(
        out.join("RAW_CHECKS.json"),
        serde_json::to_string_pretty(&raw_checks)? + "\n",
    )?;
    println!(
        "{}",
        json!({"accepted_cells":accepted_cells,"replay_checks":replay_checks,"distributions":distributions})
    );
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn multi_event_profiles_and_missing_footers_are_not_Ir_totals() {
        assert_eq!(ir("events: Ir Dr\ntotals: 1 2\n"), None);
        assert_eq!(ir("events: Ir\n"), None);
        assert_eq!(ir("events: Ir\nsummary: 3\ntotals: 6\n"), Some(6));
    }
}
