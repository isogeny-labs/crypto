//! Analysis of the prime-field PKM tower round
//! (`research/pkm_tower_ecbench_20261011/PROTOCOL.md`).
//!
//! Reads only the sessions' `records.jsonl` and applies the protocol's
//! decision rules as written. `W` is the record's counted solver work
//! (`solver.ops`, `F_p` multiply-adds). A cell whose run fails verification
//! enters no ratio; a cell with more than half its targets timed out gets
//! no median.
//!
//! Usage: `pkm_tower_prime_analyze OUT.json SESSION_DIR...`

use serde_json::{json, Value};
use std::collections::BTreeMap;

/// `F_p` multiplications per affine addition, measured in
/// `RESEARCH_RESIDUAL_WALKS.md` §11; used only for the derived pricing.
const MULADDS_PER_GAE: f64 = 63.0;

#[derive(Clone, Debug)]
struct Run {
    arm: String,
    workload: String,
    status: String,
    slug: String,
    bits: u64,
    r: f64,
    s: Option<f64>,
    lower_bound: bool,
    w: Option<u64>,
    calls: Option<u64>,
    tree: [Option<u64>; 3],
    geometry: Option<u64>,
    relations: Option<u64>,
    trials: Option<u64>,
    wall_ns: Option<u64>,
    base_points: Option<u64>,
    isolation: String,
}

fn read(dir: &str) -> Result<(String, Vec<Run>), String> {
    let path = format!("{dir}/records.jsonl");
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))?;
    let session: Value = std::fs::read_to_string(format!("{dir}/session.json"))
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or(Value::Null);
    let id = session["session_id"].as_str().unwrap_or("?").to_string();
    let mut runs = Vec::new();
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let r: Value = serde_json::from_str(line).map_err(|e| format!("{path}: {e}"))?;
        if r["warmup"].as_bool().unwrap_or(false) {
            continue;
        }
        let curve = &r["workload"]["curve"];
        let extra = &r["solver"]["extra"];
        let relations_wall = r["phases"]
            .as_array()
            .and_then(|ps| ps.iter().find(|p| p["name"] == "relations"))
            .and_then(|p| p["wall_ns"].as_u64());
        runs.push(Run {
            arm: r["arm"].as_str().unwrap_or("?").to_string(),
            workload: r["workload"]["workload_id"]
                .as_str()
                .unwrap_or("?")
                .to_string(),
            status: r["outcome"]["status"].as_str().unwrap_or("?").to_string(),
            slug: curve["slug"].as_str().unwrap_or("?").to_string(),
            bits: curve["field_bits"].as_u64().unwrap_or(0),
            r: curve["r"].as_f64().unwrap_or(f64::NAN),
            s: r["cost"]["s"].as_f64(),
            lower_bound: r["cost"]["lower_bound"].as_bool().unwrap_or(false),
            w: r["solver"]["ops"].as_u64(),
            calls: r["solver"]["calls"].as_u64(),
            tree: [
                extra["nodes"].as_u64(),
                extra["refuted"].as_u64(),
                extra["leaves"].as_u64(),
            ],
            geometry: extra["geometric_group_additions"].as_u64(),
            relations: r["counters"]["relations_found"].as_u64(),
            trials: r["counters"]["targets_tried"].as_u64(),
            wall_ns: relations_wall,
            base_points: r["factor_base"]["signed_points"].as_u64(),
            isolation: r["isolation"]["level"].as_str().unwrap_or("?").to_string(),
        });
    }
    Ok((id, runs))
}

fn median(v: &mut [f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
    let n = v.len();
    Some(if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    })
}

fn summary(v: &[f64]) -> Value {
    let mut w = v.to_vec();
    let med = median(&mut w);
    json!({"n": v.len(), "median": med, "min": w.first(), "max": w.last(), "values": v})
}

const IC_ARMS: [&str; 4] = ["f4", "sig-f5", "f6", "enum"];

fn cell(slug: &str, runs: &[&Run]) -> Value {
    let first = runs[0];
    let mut by: BTreeMap<&str, BTreeMap<&str, &Run>> = BTreeMap::new();
    for r in runs {
        by.entry(r.arm.as_str())
            .or_default()
            .insert(r.workload.as_str(), r);
    }
    let mut arms = serde_json::Map::new();
    let mut failed = Vec::new();
    let mut medians_ok: BTreeMap<String, bool> = BTreeMap::new();
    for (arm, rs) in &by {
        let n = rs.len();
        let verified = rs.values().filter(|r| r.status == "verified").count();
        let timed_out = rs.values().filter(|r| r.status == "timeout").count();
        if IC_ARMS.contains(arm) && verified + timed_out < n {
            failed.push(arm.to_string());
        }
        medians_ok.insert(arm.to_string(), timed_out * 2 <= n);
        let ok: Vec<&&Run> = rs.values().filter(|r| r.status == "verified").collect();
        let field = |f: &dyn Fn(&Run) -> Option<f64>| -> Value {
            let v: Vec<f64> = ok.iter().filter_map(|r| f(r)).collect();
            if v.is_empty() || timed_out * 2 > n {
                Value::Null
            } else {
                summary(&v)
            }
        };
        let mut statuses: BTreeMap<String, usize> = BTreeMap::new();
        let mut isolation: BTreeMap<String, usize> = BTreeMap::new();
        for r in rs.values() {
            *statuses.entry(r.status.clone()).or_default() += 1;
            *isolation.entry(r.isolation.clone()).or_default() += 1;
        }
        arms.insert(
            arm.to_string(),
            json!({
                "runs": n,
                "statuses": statuses,
                "isolation": isolation,
                "lower_bound": ok.iter().any(|r| r.lower_bound),
                "s": field(&|r| r.s),
                "muladds": field(&|r| r.w.map(|x| x as f64)),
                "solver_calls": field(&|r| r.calls.map(|x| x as f64)),
                "nodes": field(&|r| r.tree[0].map(|x| x as f64)),
                "refuted": field(&|r| r.tree[1].map(|x| x as f64)),
                "geometric_additions": field(&|r| r.geometry.map(|x| x as f64)),
                "relations": field(&|r| r.relations.map(|x| x as f64)),
                "trials": field(&|r| r.trials.map(|x| x as f64)),
                "relations_wall_s": field(&|r| r.wall_ns.map(|x| x as f64 / 1e9)),
                // Derived: S with the multiply-adds priced at 1/63 GAE each.
                "derived_s_muladds_priced": field(&|r| Some(r.s? + r.w? as f64 / MULADDS_PER_GAE / r.r.sqrt())),
                "base_points": ok.iter().find_map(|r| r.base_points),
            }),
        );
    }
    let paired = |a: &str, b: &str, f: &dyn Fn(&Run, &Run) -> Option<f64>| -> Vec<f64> {
        let (Some(ra), Some(rb)) = (by.get(a), by.get(b)) else {
            return Vec::new();
        };
        if !medians_ok.get(a).copied().unwrap_or(false)
            || !medians_ok.get(b).copied().unwrap_or(false)
        {
            return Vec::new();
        }
        ra.iter()
            .filter_map(|(w, x)| {
                let y = rb.get(w)?;
                (x.status == "verified" && y.status == "verified")
                    .then(|| f(x, y))
                    .flatten()
            })
            .collect()
    };
    let failed_cell = !failed.is_empty();
    let mut q = serde_json::Map::new();

    // Q1: signature F5 against F4.
    let f5f4 = paired("sig-f5", "f4", &|x, y| Some(x.w? as f64 / y.w? as f64));
    if !f5f4.is_empty() {
        let same = paired("sig-f5", "f4", &|x, y| {
            Some(
                (x.tree == y.tree && x.trials == y.trials && x.relations == y.relations) as u8
                    as f64,
            )
        });
        let all_same = same.iter().all(|&v| v == 1.0);
        let verdict = if failed_cell {
            "failed cell"
        } else if all_same && f5f4.iter().all(|&v| v < 1.0) {
            "F5 cuts F4's work"
        } else if f5f4.iter().all(|&v| v > 1.0) {
            "F5 costs more"
        } else {
            "mixed"
        };
        let wall = paired("sig-f5", "f4", &|x, y| {
            Some(x.wall_ns? as f64 / y.wall_ns? as f64)
        });
        q.insert(
            "q1_sig_f5_over_f4_muladds".into(),
            json!({"ratio": summary(&f5f4), "same_tree_on_every_target": all_same,
                   "relations_wall_sig_over_f4": summary(&wall), "verdict": verdict}),
        );
    }
    // Q2: F6 analogue against F4.
    let l = paired("f4", "f6", &|x, y| Some((x.w? as f64 / y.w? as f64).log2()));
    if !l.is_empty() {
        let mut w = l.clone();
        let med = median(&mut w).unwrap();
        let verdict = if failed_cell {
            "failed cell"
        } else if med >= 0.1 && l.iter().all(|&v| v > 0.0) {
            "F6 saves solver work"
        } else if med.abs() <= 0.05 {
            "no saving"
        } else {
            "mixed"
        };
        let s_ratio = paired("f6", "f4", &|x, y| Some(x.s? / y.s?));
        q.insert(
            "q2_log2_f4_over_f6_muladds".into(),
            json!({"log2_ratio": summary(&l), "verdict": verdict, "s_f6_over_f4": summary(&s_ratio)}),
        );
    }
    // Q3: each arm against rho.
    let mut q3 = serde_json::Map::new();
    for arm in IC_ARMS {
        let v = paired(arm, "rho-negation", &|x, y| Some(x.s? / y.s?));
        if v.is_empty() {
            continue;
        }
        let verdict = if failed_cell {
            "failed cell"
        } else if v.iter().all(|&x| x > 1.0) {
            "slower than rho on every target"
        } else if v.iter().all(|&x| x < 1.0) {
            "below rho on every target"
        } else {
            "mixed"
        };
        q3.insert(
            arm.into(),
            json!({"s_ratio": summary(&v), "verdict": verdict}),
        );
    }
    if !q3.is_empty() {
        q.insert("q3_s_over_rho_negation".into(), Value::Object(q3));
    }
    // Q4: algebra against geometry, reported as is.
    let mut q4 = serde_json::Map::new();
    for arm in ["f4", "sig-f5", "f6"] {
        let s = paired(arm, "enum", &|x, y| Some(x.s? / y.s?));
        let priced = paired(arm, "enum", &|x, y| {
            Some((x.s? + x.w? as f64 / MULADDS_PER_GAE / x.r.sqrt()) / y.s?)
        });
        let wall = paired(arm, "enum", &|x, y| {
            Some(x.wall_ns? as f64 / y.wall_ns? as f64)
        });
        if !s.is_empty() {
            q4.insert(
                arm.into(),
                json!({"s_over_enum": summary(&s), "derived_priced_s_over_enum": summary(&priced),
                       "relations_wall_over_enum": summary(&wall)}),
            );
        }
    }
    if !q4.is_empty() {
        q.insert("q4_algebra_over_enum".into(), Value::Object(q4));
    }
    json!({
        "slug": slug,
        "bits": first.bits,
        "r": first.r,
        "log2_r": first.r.log2(),
        "floor_s": (std::f64::consts::PI / 4.0).sqrt(),
        "failed_ic_arms": failed,
        "arms": arms,
        "questions": q,
    })
}

/// Least-squares slope of `ys` against `xs`.
fn slope(xs: &[f64], ys: &[f64]) -> Option<f64> {
    if xs.len() < 2 {
        return None;
    }
    let n = xs.len() as f64;
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let sxx: f64 = xs.iter().map(|x| (x - mx) * (x - mx)).sum();
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    (sxx > 0.0).then(|| sxy / sxx)
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        return Err("usage: pkm_tower_prime_analyze OUT.json SESSION_DIR...".into());
    }
    let mut sessions = serde_json::Map::new();
    let mut all_cells = Vec::new();
    for dir in &args[2..] {
        let (id, runs) = read(dir)?;
        let mut cells: BTreeMap<String, Vec<&Run>> = BTreeMap::new();
        for r in &runs {
            cells.entry(r.slug.clone()).or_default().push(r);
        }
        let cells: Vec<Value> = cells.iter().map(|(slug, rs)| cell(slug, rs)).collect();
        all_cells.extend(cells.iter().cloned());
        let name = std::path::Path::new(dir)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(dir)
            .to_string();
        sessions.insert(name, json!({"session_id": id, "dir": dir, "cells": cells}));
    }
    // Q5: descriptive slopes of log2 median S against log2 r.
    let mut q5 = serde_json::Map::new();
    for arm in ["rho-negation", "f4", "sig-f5", "f6", "enum"] {
        let (mut xs, mut ys) = (Vec::new(), Vec::new());
        for c in &all_cells {
            if let Some(m) = c["arms"][arm]["s"]["median"].as_f64() {
                xs.push(c["log2_r"].as_f64().unwrap());
                ys.push(m.log2());
            }
        }
        q5.insert(
            arm.into(),
            json!({"cells": xs.len(), "slope_log2_s_per_log2_r": slope(&xs, &ys)}),
        );
    }
    let out = json!({
        "schema": "pkm-tower-prime-analysis/v1",
        "protocol": "research/pkm_tower_ecbench_20261011/PROTOCOL.md",
        "unit": {"s": "ecbench.gae / sqrt(r); tower arms are a lower bound (F_p multiply-adds unpriced)",
                 "muladds": "solver.ops, counted F_p multiply-adds, deterministic",
                 "derived_pricing": format!("multiply-adds at 1/{MULADDS_PER_GAE} GAE each (RESEARCH_RESIDUAL_WALKS.md §11); derived, decides nothing")},
        "sessions": sessions,
        "q5_size_slopes": q5,
    });
    let text = serde_json::to_string_pretty(&out).map_err(|e| e.to_string())?;
    std::fs::write(&args[1], text + "\n").map_err(|e| e.to_string())?;
    println!("wrote {}", args[1]);
    Ok(())
}
