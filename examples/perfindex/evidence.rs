//! `replay` and `bundle`: recomputing a recorded run from its raw rows, and
//! packaging one that replays for a reviewer.

use crate::compare::{make_schedule, summarize_from_plan, Plan};
use crate::util::{read_json, read_jsonl, sha256_file};
use serde_json::Value;
use std::collections::BTreeSet;
use std::path::Path;

/// The fields of `compare.json` that follow from the raw rows by the formula
/// (the others describe the host and the cost, which a replay cannot know).
const STABLE_KEYS: [&str; 8] = [
    "valid",
    "performance_index",
    "performance_index_ci95",
    "areas",
    "kernels",
    "rounds",
    "aa_noise_median_pct",
    "exclude_contended",
];

/// Equality that tolerates the last digits of a float, nothing else.
pub fn close(x: &Value, y: &Value, tol: f64) -> bool {
    match (x, y) {
        (Value::Number(a), Value::Number(b)) => match (a.as_f64(), b.as_f64()) {
            (Some(p), Some(q)) => p == q || (p - q).abs() <= tol.max(tol * p.abs().max(q.abs())),
            _ => a == b,
        },
        (Value::Object(a), Value::Object(b)) => {
            a.len() == b.len()
                && a.iter()
                    .all(|(k, v)| b.get(k).is_some_and(|w| close(v, w, tol)))
        }
        (Value::Array(a), Value::Array(b)) => {
            a.len() == b.len() && a.iter().zip(b).all(|(p, q)| close(p, q, tol))
        }
        _ => x == y,
    }
}

/// Recomputes `compare.json` from `plan.json` and `raw.jsonl`.  What it
/// checks is the statistics: that the reported numbers follow from the raw
/// rows by the documented formula, and that every planned invocation is there
/// exactly once.  It does not re-time anything.
pub fn replay_dir(dir: &Path) -> Result<(bool, Vec<String>), String> {
    let plan: Plan = serde_json::from_value(read_json(&dir.join("plan.json"))?)
        .map_err(|e| format!("plan.json: {e}"))?;
    let raw = read_jsonl(&dir.join("raw.jsonl"))?;
    let told = read_json(&dir.join("compare.json"))?;
    let mut again = summarize_from_plan(&plan, &raw);
    if raw.iter().any(|r| r.get("row_problems").is_some()) {
        again.valid = false;
        again.performance_index = None;
        again.performance_index_ci95 = None;
    }
    let again = serde_json::to_value(&again).map_err(|e| e.to_string())?;
    let mut diffs: Vec<String> = Vec::new();
    for key in STABLE_KEYS {
        let (a, b) = (
            told.get(key).unwrap_or(&Value::Null),
            again.get(key).unwrap_or(&Value::Null),
        );
        if !close(a, b, 1e-9) {
            diffs.push(key.to_string());
        }
    }
    let ids: Vec<String> = plan.kernels.iter().map(|k| k.id.clone()).collect();
    let planned: BTreeSet<(usize, String, String)> =
        make_schedule(&ids, plan.rounds, plan.aa, plan.seed)
            .into_iter()
            .map(|i| (i.round, i.kernel, i.arm.name().to_string()))
            .collect();
    let got: Vec<(usize, String, String)> = raw
        .iter()
        .filter(|r| r.get("type").and_then(Value::as_str) == Some("invocation"))
        .map(|r| {
            (
                r["round"].as_u64().unwrap_or(u64::MAX) as usize,
                r["kernel"].as_str().unwrap_or("").to_string(),
                r["arm"].as_str().unwrap_or("").to_string(),
            )
        })
        .collect();
    let got_set: BTreeSet<_> = got.iter().cloned().collect();
    if got.len() != got_set.len() {
        diffs.push("raw.jsonl repeats an invocation".into());
    }
    let stray = got_set.difference(&planned).count();
    if stray > 0 {
        diffs.push(format!(
            "raw.jsonl holds {stray} invocation(s) outside the plan"
        ));
    }
    let stopped = told.get("stopped_by_budget").and_then(Value::as_bool) == Some(true);
    let missing = planned.difference(&got_set).count();
    if !stopped && missing > 0 {
        diffs.push(format!(
            "raw.jsonl is missing {missing} planned invocation(s)"
        ));
    }
    Ok((diffs.is_empty(), diffs))
}

pub fn run_replay(dir: &Path) -> Result<i32, String> {
    let (ok, diffs) = replay_dir(dir)?;
    if ok {
        println!(
            "replay MATCH: {}/compare.json follows from plan.json and raw.jsonl",
            dir.display()
        );
        Ok(0)
    } else {
        eprintln!("replay MISMATCH in: {diffs:?}");
        Ok(1)
    }
}

pub fn run_bundle(dir: &Path, dest: &Path, max_mb: f64) -> Result<i32, String> {
    let (ok, diffs) = replay_dir(dir)?;
    if !ok {
        eprintln!("refused: the run does not replay ({diffs:?})");
        return Ok(1);
    }
    if dest.exists() {
        eprintln!("refused: {} exists", dest.display());
        return Ok(2);
    }
    let names = [
        "plan.json",
        "raw.jsonl",
        "compare.json",
        "compare.md",
        "cost.json",
    ];
    let size: u64 = names
        .iter()
        .filter_map(|n| std::fs::metadata(dir.join(n)).ok())
        .map(|m| m.len())
        .sum();
    if size as f64 > max_mb * 1e6 {
        eprintln!(
            "refused: {:.1} MB exceeds --max-mb {max_mb}",
            size as f64 / 1e6
        );
        return Ok(2);
    }
    std::fs::create_dir_all(dest).map_err(|e| e.to_string())?;
    let mut sums = String::new();
    for n in names {
        if dir.join(n).exists() {
            std::fs::copy(dir.join(n), dest.join(n)).map_err(|e| e.to_string())?;
            sums.push_str(&format!(
                "{}  {n}\n",
                sha256_file(&dest.join(n)).map_err(|e| e.to_string())?
            ));
        }
    }
    std::fs::write(dest.join("SHA256SUMS"), sums).map_err(|e| e.to_string())?;
    let plan = read_json(&dir.join("plan.json"))?;
    let told = read_json(&dir.join("compare.json"))?;
    let short = |v: &Value, k: &str| {
        v.get(k)
            .and_then(Value::as_str)
            .map_or("?".to_string(), |s| s[..s.len().min(16)].to_string())
    };
    let rev = |side: &str| {
        plan[side]
            .get("rev")
            .and_then(Value::as_str)
            .unwrap_or("?")
            .to_string()
    };
    let pi = told.get("performance_index").and_then(Value::as_f64);
    let mut readme = vec![
        format!(
            "# perfbench evidence: {}",
            dest.file_name()
                .map_or(String::new(), |n| n.to_string_lossy().into_owned())
        ),
        String::new(),
        format!(
            "* base      `{}` (binary `{}`)",
            rev("base"),
            short(&plan["base"], "binary_sha256")
        ),
        format!(
            "* candidate `{}` (binary `{}`)",
            rev("cand"),
            short(&plan["cand"], "binary_sha256")
        ),
        format!(
            "* threads {}, {} planned rounds, A/A {}, seed {}, schedule `{}`",
            plan["threads"],
            plan["rounds"],
            if plan["aa"].as_bool() == Some(true) {
                "on"
            } else {
                "OFF"
            },
            plan["seed"],
            plan["schedule_hash"].as_str().unwrap_or("?")
        ),
        format!(
            "* performance index: {}; qualified: {}; valid: {}",
            pi.map_or("refused".to_string(), |p| format!("{p:.3}x")),
            told["qualified"],
            told["valid"]
        ),
        String::new(),
        "Warnings recorded with the run:".into(),
    ];
    let warns: Vec<String> = told["warnings"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(Value::as_str)
                .map(|w| format!("* {w}"))
                .collect()
        })
        .unwrap_or_default();
    if warns.is_empty() {
        readme.push("* none".into());
    } else {
        readme.extend(warns);
    }
    readme.extend([
        String::new(),
        "Check that the numbers follow from the raw rows (this re-times nothing):".into(),
        String::new(),
        "```bash".into(),
        "sha256sum -c SHA256SUMS".into(),
        "cargo run --release --example perfindex -- replay .".into(),
        "```".into(),
        String::new(),
        "`raw.jsonl` holds every harness row exactly as the harness printed it, with the machine's".into(),
        "load and pressure sampled at each round's start and end.  Wall-time rows are evidence only".into(),
        "when uncontended (AGENTS.md section 10); `compare.json` says how many rounds were left out.".into(),
        "These are host-specific observations, not a claim about another host class.".into(),
    ]);
    std::fs::write(dest.join("README.md"), readme.join("\n") + "\n").map_err(|e| e.to_string())?;
    println!(
        "bundle written: {} ({:.2} MB), replay MATCH",
        dest.display(),
        size as f64 / 1e6
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn close_tolerates_last_digits_only() {
        assert!(close(&json!(1.0), &json!(1.0 + 1e-12), 1e-9));
        assert!(!close(&json!(1.0), &json!(1.001), 1e-9));
        assert!(close(
            &json!({"a": [1, 2.0]}),
            &json!({"a": [1.0, 2]}),
            1e-9
        ));
        assert!(!close(&json!({"a": 1}), &json!({"a": 1, "b": 2}), 1e-9));
        assert!(!close(&json!(null), &json!(0), 1e-9));
        assert!(close(&json!(null), &json!(null), 1e-9));
        assert!(!close(&json!("x"), &json!("y"), 1e-9));
    }
}
