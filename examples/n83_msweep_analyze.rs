//! Analyse the N83 summand sweep (`research/n83_msweep_20261024`) from its
//! raw records only: every cell file under `raw/shard*/`, the rho receipts
//! and the rootfinder reference outputs under `raw/reference/`.
//!
//! Writes `ANALYSIS.json` (every cell with its derived fields, the planted
//! control verdict per engine and m, and the best informative cell per m)
//! and `table.tsv`. Derived fields are named with their formula; a cell
//! without a decided projection carries `null`, never a guess.
//!
//! ```text
//! n83_msweep_analyze <study_dir>
//! ```
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

fn read_json(path: &Path) -> Option<Value> {
    serde_json::from_str(fs::read_to_string(path).ok()?.lines().next()?).ok()
}

fn cells(study: &Path) -> Vec<Value> {
    let mut out = Vec::new();
    let Ok(raw) = fs::read_dir(study.join("raw")) else {
        return out;
    };
    let mut shards: Vec<_> = raw.flatten().map(|e| e.path()).collect();
    shards.sort();
    for shard in shards.iter().filter(|p| p.file_name().is_some_and(|n| n.to_string_lossy().starts_with("shard"))) {
        let mut files: Vec<_> = fs::read_dir(shard).into_iter().flatten().flatten().map(|e| e.path()).collect();
        files.sort();
        for f in files.iter().filter(|p| p.extension().is_some_and(|e| e == "json")) {
            if let Some(mut v) = read_json(f) {
                v["record"] = json!(f.strip_prefix(study).unwrap_or(f).to_string_lossy());
                out.push(v);
            }
        }
    }
    out
}

/// The cell's identity. Every record (complete, process cap or error)
/// carries m, factor base and engine; the curve and probe kind fall back to
/// the file name, `curve__mM__fb__engine__probes.json`.
fn identity(v: &Value) -> (String, u64, String, String, String) {
    let name = v["record"].as_str().unwrap_or("");
    let stem = Path::new(name).file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    let parts: Vec<&str> = stem.split("__").collect();
    let s = |k: &str, i: usize| v[k].as_str().map_or_else(|| parts.get(i).copied().unwrap_or("").to_string(), str::to_string);
    let probes = match s("probes", 4).as_str() {
        "public_subgroup" => "public".to_string(),
        other => other.to_string(),
    };
    (s("curve_spec", 0), v["m"].as_u64().unwrap_or(0), s("factor_base", 2), s("engine", 3), probes)
}

fn rho(study: &Path) -> Value {
    let dir = study.join("raw/reference");
    let mut runs = Vec::new();
    for tag in ["rho_a", "rho_b", "rho_c"] {
        if let Some(r) = read_json(&dir.join(format!("{tag}.receipt.json"))) {
            runs.push(json!({
                "tag": tag,
                "status": r["status"],
                "online_ms": r["online_ms"],
                "walk_seed": r["walk_seed"],
                "group_verified": r["group_verified"],
            }));
        }
    }
    let ok: Vec<f64> = runs
        .iter()
        .filter(|r| r["status"] == "PASS_VERIFIED_PUBLIC_TARGET")
        .filter_map(|r| r["online_ms"].as_f64())
        .collect();
    let mean_s = (!ok.is_empty()).then(|| ok.iter().sum::<f64>() / ok.len() as f64 / 1e3);
    json!({"runs": runs, "verified_runs": ok.len(), "mean_online_seconds": mean_s,
           "rule": "mean of online_ms over receipts with status PASS_VERIFIED_PUBLIC_TARGET"})
}

fn rootfinder(study: &Path) -> Vec<Value> {
    let dir = study.join("raw/reference");
    ["ic_root11_k1107", "ic_root22_k1107"]
        .iter()
        .map(|tag| {
            let wall: Option<f64> = fs::read_to_string(dir.join(format!("{tag}.wall")))
                .ok()
                .and_then(|s| s.trim().parse().ok());
            let result: Vec<Value> = fs::read_to_string(dir.join(tag).join("targets_result.jsonl"))
                .unwrap_or_default()
                .lines()
                .filter_map(|l| serde_json::from_str(l).ok())
                .collect();
            json!({"tag": tag, "process_wall_seconds": wall, "result_lines": result})
        })
        .collect()
}

fn main() {
    let study = std::env::args().nth(1).expect("usage: n83_msweep_analyze <study_dir>");
    let study = Path::new(&study);
    let rho = rho(study);
    let rho_s = rho["mean_online_seconds"].as_f64();
    let mut rows = Vec::new();
    // Planted controls: (curve, engine, m) -> passed (every planted probe verified).
    let mut controls: BTreeMap<String, Value> = BTreeMap::new();
    for v in cells(study) {
        let (curve, m, fb, engine, probes) = identity(&v);
        let status = v["status"].as_str().unwrap_or("missing").to_string();
        let trials = v["trials"].as_u64().unwrap_or(0);
        let verified = v["verified_relations"].as_u64().unwrap_or(0);
        let log2_proj = v["log2_projected_collection_seconds"].as_f64();
        let gap = match (log2_proj, rho_s, curve.as_str(), probes.as_str()) {
            (Some(p), Some(r), "n83k1", "public") => Some(p - r.log2()),
            _ => None,
        };
        if probes == "planted" {
            let key = format!("{curve}|{engine}|{m}|{fb}");
            controls.insert(
                key,
                json!({"curve": curve, "engine": engine, "m": m, "factor_base": fb,
                       "status": status, "planted": trials, "verified": verified,
                       "outcomes": v["outcomes"],
                       "passed": status == "complete" && trials > 0 && verified == trials}),
            );
        }
        rows.push(json!({
            "curve": curve, "m": m, "factor_base": fb, "engine": engine, "probes": probes,
            "status": status, "trials": trials, "outcomes": v["outcomes"],
            "decided_fraction": v["decided_fraction"], "relations": v["relations"],
            "verified_relations": verified, "base_points": v["base_points"],
            "seconds_per_trial": v["seconds_per_trial"],
            "wall_seconds_at_cap": v["wall_seconds"],
            "log2_p_heuristic": v["log2_p_heuristic"], "p_source": v["p_source"],
            "log2_projected_collection_seconds": log2_proj,
            "log2_gap_to_rho": gap,
            "record": v["record"],
        }));
    }
    // Best informative public n83 cell per m (smallest projected time).
    let mut best: BTreeMap<u64, Value> = BTreeMap::new();
    for r in rows.iter().filter(|r| r["curve"] == "n83k1" && r["probes"] == "public") {
        let Some(p) = r["log2_projected_collection_seconds"].as_f64() else { continue };
        let m = r["m"].as_u64().unwrap_or(0);
        if best.get(&m).and_then(|b| b["log2_projected_collection_seconds"].as_f64()).is_none_or(|b| p < b) {
            best.insert(m, r.clone());
        }
    }
    let mut counts: BTreeMap<String, u64> = BTreeMap::new();
    for r in &rows {
        *counts.entry(format!("{}|{}", r["curve"].as_str().unwrap_or(""), r["status"].as_str().unwrap_or(""))).or_default() += 1;
    }
    let analysis = json!({
        "schema": "n83-msweep-analysis/v1",
        "study": "research/n83_msweep_20261024",
        "derivations": {
            "log2_projected_collection_seconds": "cell driver: log2(relations_needed / p * seconds_per_trial), only when >= 90% of queries decided",
            "log2_gap_to_rho": "log2_projected_collection_seconds - log2(rho mean_online_seconds); n83k1 public cells only",
            "control.passed": "status complete and every planted probe returned a group-verified witness",
        },
        "cell_status_counts": counts,
        "rho": rho,
        "rootfinder_reference": rootfinder(study),
        "controls": controls.into_values().collect::<Vec<_>>(),
        "best_projected_per_m": best.into_values().collect::<Vec<_>>(),
        "cells": rows,
    });
    fs::write(study.join("ANALYSIS.json"), serde_json::to_string_pretty(&analysis).unwrap()).unwrap();
    let mut tsv = String::from("curve\tm\tfactor_base\tengine\tprobes\tstatus\ttrials\tdecided\tverified\ts_per_trial\tlog2_proj_s\tlog2_gap_rho\n");
    for r in analysis["cells"].as_array().unwrap() {
        let f = |k: &str| match &r[k] {
            Value::Null => "".to_string(),
            Value::String(s) => s.clone(),
            Value::Number(n) => n.as_f64().map_or(n.to_string(), |x| if x.fract() == 0.0 { format!("{x}") } else { format!("{x:.4}") }),
            other => other.to_string(),
        };
        tsv.push_str(&[f("curve"), f("m"), f("factor_base"), f("engine"), f("probes"), f("status"), f("trials"),
            f("decided_fraction"), f("verified_relations"), f("seconds_per_trial"),
            f("log2_projected_collection_seconds"), f("log2_gap_to_rho")].join("\t"));
        tsv.push('\n');
    }
    fs::write(study.join("table.tsv"), tsv).unwrap();
    eprintln!("{} cells, rho mean {:?} s", analysis["cells"].as_array().unwrap().len(), rho_s);
}
