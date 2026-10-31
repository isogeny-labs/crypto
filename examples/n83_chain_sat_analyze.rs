//! Analyse `research/n83_chain_sat_order_20261031` from its raw records only
//! (`raw/shard*/m*.jsonl`, written by `n83_chain_sat_bench`).
//!
//! Writes `ANALYSIS.json` and `table.tsv`: per (m, arm, kind) the decided,
//! refuted, witness, verified and undecided counts with conflicts and
//! seconds; per (m, arm) the paired comparison against the baseline arm on
//! the targets both decide; and every soundness flag (a planted target
//! refuted, or a witness that does not re-add to its target).
//!
//! ```text
//! n83_chain_sat_analyze <study_dir>
//! ```
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const BASELINE: &str = "unordered";

fn median(mut v: Vec<f64>) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    Some(if n % 2 == 1 { v[n / 2] } else { (v[n / 2 - 1] + v[n / 2]) / 2.0 })
}

fn main() {
    let study = std::env::args().nth(1).expect("usage: n83_chain_sat_analyze <study_dir>");
    let study = Path::new(&study);
    let mut rows: Vec<Value> = Vec::new();
    let mut shards: Vec<_> = fs::read_dir(study.join("raw")).expect("raw/").flatten().map(|e| e.path()).collect();
    shards.sort();
    for shard in shards {
        let mut files: Vec<_> = fs::read_dir(&shard).into_iter().flatten().flatten().map(|e| e.path()).collect();
        files.sort();
        for f in files.iter().filter(|p| p.extension().is_some_and(|e| e == "jsonl")) {
            for line in fs::read_to_string(f).unwrap_or_default().lines() {
                if let Ok(mut v) = serde_json::from_str::<Value>(line) {
                    v["record"] = json!(f.strip_prefix(study).unwrap_or(f).to_string_lossy());
                    rows.push(v);
                }
            }
        }
    }
    // Group by (m, arm, kind).
    let mut groups: BTreeMap<(u64, String, String), Vec<&Value>> = BTreeMap::new();
    for r in &rows {
        let key = (
            r["m"].as_u64().unwrap_or(0),
            r["arm"].as_str().unwrap_or("").to_string(),
            r["kind"].as_str().unwrap_or("").to_string(),
        );
        groups.entry(key).or_default().push(r);
    }
    let mut cells = Vec::new();
    for ((m, arm, kind), rs) in &groups {
        let count = |v: &str| rs.iter().filter(|r| r["verdict"] == v).count();
        let decided: Vec<&&Value> = rs.iter().filter(|r| r["verdict"] != "undecided").collect();
        cells.push(json!({
            "m": m, "arm": arm, "kind": kind, "queries": rs.len(),
            "refuted": count("refuted"), "witness": count("witness"),
            "verified": rs.iter().filter(|r| r["verified"] == true).count(),
            "undecided": count("undecided"),
            "median_conflicts_decided": median(decided.iter().filter_map(|r| r["conflicts"].as_f64()).collect()),
            "median_seconds_decided": median(decided.iter().filter_map(|r| r["seconds"].as_f64()).collect()),
            "median_seconds_all": median(rs.iter().filter_map(|r| r["seconds"].as_f64()).collect()),
        }));
    }
    // Paired comparison against the baseline on the same (m, target).
    let mut by_target: BTreeMap<(u64, u64), BTreeMap<String, &Value>> = BTreeMap::new();
    for r in &rows {
        by_target
            .entry((r["m"].as_u64().unwrap_or(0), r["target"].as_u64().unwrap_or(0)))
            .or_default()
            .insert(r["arm"].as_str().unwrap_or("").to_string(), r);
    }
    let mut pairs: BTreeMap<(u64, String), Vec<Value>> = BTreeMap::new();
    for ((m, t), arms) in &by_target {
        let Some(base) = arms.get(BASELINE) else { continue };
        for (arm, r) in arms {
            if arm == BASELINE {
                continue;
            }
            pairs.entry((*m, arm.clone())).or_default().push(json!({
                "target": t, "kind": r["kind"],
                "baseline_verdict": base["verdict"], "arm_verdict": r["verdict"],
                "baseline_conflicts": base["conflicts"], "arm_conflicts": r["conflicts"],
                "baseline_seconds": base["seconds"], "arm_seconds": r["seconds"],
            }));
        }
    }
    let paired: Vec<Value> = pairs
        .into_iter()
        .map(|((m, arm), ps)| {
            let both: Vec<&Value> = ps
                .iter()
                .filter(|p| p["baseline_verdict"] != "undecided" && p["arm_verdict"] != "undecided")
                .collect();
            let ratios: Vec<f64> = both
                .iter()
                .filter_map(|p| {
                    let a = p["arm_conflicts"].as_f64()?;
                    let b = p["baseline_conflicts"].as_f64()?;
                    (b > 0.0).then_some(a / b)
                })
                .collect();
            let only_arm = ps.iter().filter(|p| p["baseline_verdict"] == "undecided" && p["arm_verdict"] != "undecided").count();
            let only_base = ps.iter().filter(|p| p["baseline_verdict"] != "undecided" && p["arm_verdict"] == "undecided").count();
            json!({
                "m": m, "arm": arm, "targets": ps.len(),
                "both_decided": both.len(),
                "decided_by_arm_only": only_arm, "decided_by_baseline_only": only_base,
                "conflict_ratio_arm_over_baseline_on_both": ratios,
                "max_conflict_ratio": ratios.iter().cloned().fold(None, |a: Option<f64>, x| Some(a.map_or(x, |a| a.max(x)))),
                "pairs": ps,
            })
        })
        .collect();
    let flags: Vec<Value> = rows
        .iter()
        .filter(|r| {
            (r["kind"] == "planted" && r["verdict"] == "refuted")
                || (r["verdict"] == "witness" && r["verified"] != true)
                || r["invalid_models"].as_u64().unwrap_or(0) > 0
        })
        .map(|r| json!({"m": r["m"], "arm": r["arm"], "target": r["target"], "verdict": r["verdict"], "record": r["record"]}))
        .collect();
    let analysis = json!({
        "schema": "n83-chain-sat-analysis/v1",
        "study": "research/n83_chain_sat_order_20261031",
        "baseline_arm": BASELINE,
        "records": rows.len(),
        "derivations": {
            "conflict_ratio_arm_over_baseline_on_both": "arm conflicts / baseline conflicts per target both decided (refuted or witness)",
            "decided": "verdict refuted or witness; undecided = conflict or wall cap",
        },
        "soundness_flags": flags,
        "cells": cells,
        "paired": paired,
    });
    fs::write(study.join("ANALYSIS.json"), serde_json::to_string_pretty(&analysis).unwrap()).unwrap();
    let mut tsv = String::from("m\tarm\tkind\tqueries\trefuted\twitness\tverified\tundecided\tmedian_conflicts_decided\tmedian_seconds_decided\n");
    for c in analysis["cells"].as_array().unwrap() {
        let f = |k: &str| match &c[k] {
            Value::Null => String::new(),
            Value::Number(n) => n.as_f64().map_or(n.to_string(), |x| if x.fract() == 0.0 { format!("{x}") } else { format!("{x:.3}") }),
            Value::String(s) => s.clone(),
            v => v.to_string(),
        };
        tsv.push_str(&["m", "arm", "kind", "queries", "refuted", "witness", "verified", "undecided", "median_conflicts_decided", "median_seconds_decided"].map(f).join("\t"));
        tsv.push('\n');
    }
    fs::write(study.join("table.tsv"), tsv).unwrap();
    eprintln!("{} records, {} soundness flags", rows.len(), analysis["soundness_flags"].as_array().unwrap().len());
}
