//! Analysis of the root-finder round
//! (`research/pkm_tower_rootfind_20261018/PROTOCOL.md`): the repository's
//! root-finding decomposers against tower F4, signature F5 and the F6
//! analogue, on one Kummer base and the same public targets.
//!
//! Reads only the sessions' `records.jsonl` (and, for the instrument check,
//! the 2026-10-11 round's) and applies the protocol's rules as written.
//!
//! `M`, the `F_p` multiplications of a run, is derived from recorded counts
//! with `PrimeCurve`'s fixed costs: an affine addition is 2 multiplications
//! + 1 squaring + 1 inversion, a doubling 5 + 2 + 1, so
//! `M = 3·adds + 7·doubles` over every phase, plus the solver's multiply-adds
//! (`solver.ops`) for the tower arms. Inversions are `adds + doubles` plus
//! the native `inversions` counter, reported beside `M`.
//!
//! Usage: `pkm_rootfind_analyze OUT.json PRIOR_SESSIONS_DIR SESSION_DIR...`

use serde_json::{json, Value};
use std::collections::BTreeMap;

const ROOT: [&str; 2] = ["rootfind-subtract", "rootfind-mitm"];
const GROEBNER: [&str; 2] = ["f4", "sig-f5"];
const OTHERS: [&str; 2] = ["f6", "enum"];
const CONTEXT: [&str; 2] = ["rootfind-mitm-m3", "rootfind-pairs-m3"];

#[derive(Clone, Debug)]
struct Run {
    arm: String,
    workload: String,
    status: String,
    slug: String,
    bits: u64,
    r: f64,
    s: Option<f64>,
    muls: u64,
    invs: u64,
    solver_ops: u64,
    tree: [Option<u64>; 3],
    relations: Option<u64>,
    trials: Option<u64>,
    wall_ns: Option<u64>,
    /// `total_gae` and the unpriced operations (`cost.unpriced`, with their
    /// counts), for AGENTS.md §5a's band.
    gae: f64,
    unpriced: BTreeMap<String, u64>,
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
        let phases = r["phases"].as_array().cloned().unwrap_or_default();
        let (mut adds, mut doubles, mut native_inv) = (0u64, 0u64, 0u64);
        let mut wall = None;
        for p in &phases {
            adds += p["adds"].as_u64().unwrap_or(0);
            doubles += p["doubles"].as_u64().unwrap_or(0);
            native_inv += p["native"]["inversions"].as_u64().unwrap_or(0);
            if p["name"] == "relations" {
                wall = p["wall_ns"].as_u64();
            }
        }
        let solver_ops = r["solver"]["ops"].as_u64().unwrap_or(0);
        let mut unpriced = BTreeMap::new();
        for u in r["cost"]["unpriced"].as_array().cloned().unwrap_or_default() {
            let name = u.as_str().unwrap_or("").trim_end_matches("_uncharged").to_string();
            let count = if name.starts_with("solver_") {
                solver_ops
            } else {
                phases.iter().map(|p| p["native"][&name].as_u64().unwrap_or(0)).sum()
            };
            unpriced.insert(name, count);
        }
        let extra = &r["solver"]["extra"];
        let curve = &r["workload"]["curve"];
        runs.push(Run {
            arm: r["arm"].as_str().unwrap_or("?").to_string(),
            workload: r["workload"]["workload_id"].as_str().unwrap_or("?").to_string(),
            status: r["outcome"]["status"].as_str().unwrap_or("?").to_string(),
            slug: curve["slug"].as_str().unwrap_or("?").to_string(),
            bits: curve["field_bits"].as_u64().unwrap_or(0),
            r: curve["r"].as_f64().unwrap_or(f64::NAN),
            s: r["cost"]["s"].as_f64(),
            muls: 3 * adds + 7 * doubles + solver_ops,
            invs: adds + doubles + native_inv,
            solver_ops,
            tree: [extra["nodes"].as_u64(), extra["refuted"].as_u64(), extra["leaves"].as_u64()],
            relations: r["counters"]["relations_found"].as_u64(),
            trials: r["counters"]["targets_tried"].as_u64(),
            wall_ns: wall,
            gae: r["cost"]["total_gae"].as_f64().unwrap_or(f64::NAN),
            unpriced,
        });
    }
    Ok((id, runs))
}

/// AGENTS.md §5a: the weights per unpriced native operation, in GAE.
const BAND: [f64; 3] = [0.0, 0.1, 1.0];

impl Run {
    fn s_at(&self, w: f64) -> f64 {
        (self.gae + w * self.unpriced.values().sum::<u64>() as f64) / self.r.sqrt()
    }
}

fn median(v: &mut [f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
    let n = v.len();
    Some(if n % 2 == 1 { v[n / 2] } else { (v[n / 2 - 1] + v[n / 2]) / 2.0 })
}

fn summary(v: &[f64]) -> Value {
    let mut w = v.to_vec();
    let med = median(&mut w);
    json!({"n": v.len(), "median": med, "min": w.first(), "max": w.last(), "values": v})
}

type ByArm<'a> = BTreeMap<&'a str, BTreeMap<&'a str, &'a Run>>;

fn paired(by: &ByArm, a: &str, b: &str, f: &dyn Fn(&Run, &Run) -> Option<f64>) -> Vec<f64> {
    let (Some(ra), Some(rb)) = (by.get(a), by.get(b)) else {
        return Vec::new();
    };
    ra.iter()
        .filter_map(|(w, x)| {
            let y = rb.get(w)?;
            (x.status == "verified" && y.status == "verified").then(|| f(x, y)).flatten()
        })
        .collect()
}

fn cell(slug: &str, runs: &[&Run], prior: &[Run]) -> Value {
    let first = runs[0];
    let mut by: ByArm = BTreeMap::new();
    for r in runs {
        by.entry(r.arm.as_str()).or_default().insert(r.workload.as_str(), r);
    }
    let mut arms = serde_json::Map::new();
    let mut failed = Vec::new();
    for (arm, rs) in &by {
        let n = rs.len();
        let verified = rs.values().filter(|r| r.status == "verified").count();
        let timed_out = rs.values().filter(|r| r.status == "timeout").count();
        if *arm != "rho-negation" && verified + timed_out < n {
            failed.push(arm.to_string());
        }
        let ok: Vec<&&Run> = rs.values().filter(|r| r.status == "verified").collect();
        let field = |f: &dyn Fn(&Run) -> Option<f64>| -> Value {
            let v: Vec<f64> = ok.iter().filter_map(|r| f(r)).collect();
            if v.is_empty() || timed_out * 2 > n { Value::Null } else { summary(&v) }
        };
        let mut statuses: BTreeMap<String, usize> = BTreeMap::new();
        for r in rs.values() {
            *statuses.entry(r.status.clone()).or_default() += 1;
        }
        arms.insert(
            arm.to_string(),
            json!({
                "runs": n,
                "statuses": statuses,
                "s": field(&|r| r.s),
                "unpriced_counters": ok.first().map(|r| r.unpriced.keys().cloned().collect::<Vec<_>>()),
                "unpriced_ops": field(&|r| Some(r.unpriced.values().sum::<u64>() as f64)),
                "s_band_0_1": field(&|r| Some(r.s_at(0.1))),
                "s_band_1": field(&|r| Some(r.s_at(1.0))),
                "fp_multiplications": field(&|r| Some(r.muls as f64)),
                "inversions": field(&|r| Some(r.invs as f64)),
                "solver_muladds": field(&|r| Some(r.solver_ops as f64)),
                "relations": field(&|r| r.relations.map(|x| x as f64)),
                "trials": field(&|r| r.trials.map(|x| x as f64)),
                "relations_wall_s": field(&|r| r.wall_ns.map(|x| x as f64 / 1e9)),
            }),
        );
    }
    let failed_cell = !failed.is_empty();
    let mut q = serde_json::Map::new();
    let mut q1 = serde_json::Map::new();
    let mut q2 = serde_json::Map::new();
    let mut q3 = serde_json::Map::new();
    for root in ROOT.iter().chain(CONTEXT.iter()) {
        let context = CONTEXT.contains(root);
        for g in GROEBNER.iter().chain(OTHERS.iter()) {
            let s = paired(&by, root, g, &|x, y| Some(x.s? / y.s?));
            let m = paired(&by, root, g, &|x, y| Some(x.muls as f64 / y.muls as f64));
            let inv_le = paired(&by, root, g, &|x, y| Some((x.invs <= y.invs) as u8 as f64));
            let wall = paired(&by, root, g, &|x, y| Some(x.wall_ns? as f64 / y.wall_ns? as f64));
            if s.is_empty() {
                continue;
            }
            let key = format!("{root}/{g}");
            let ruled = GROEBNER.contains(g) && !context;
            let s_verdict = if failed_cell {
                "failed cell"
            } else if s.iter().all(|&v| v < 1.0) {
                if *g == "f4" { "root finder proven cheaper than F4" } else { "root finder proven cheaper than F5" }
            } else {
                "not decided by S"
            };
            let m_verdict = if failed_cell {
                "failed cell"
            } else if m.iter().all(|&v| v < 1.0) && inv_le.iter().all(|&v| v == 1.0) {
                "root finder cheaper in field arithmetic"
            } else if m.iter().all(|&v| v > 1.0) {
                if *g == "f4" { "F4 cheaper in field arithmetic" } else { "F5 cheaper in field arithmetic" }
            } else {
                "mixed"
            };
            let band: Vec<Value> = BAND
                .iter()
                .map(|&w| json!({"weight": w, "s_ratio": summary(&paired(&by, root, g, &|x, y| Some(x.s_at(w) / y.s_at(w))))}))
                .collect();
            let all_below = BAND.iter().all(|&w| paired(&by, root, g, &|x, y| Some(x.s_at(w) / y.s_at(w))).iter().all(|&v| v < 1.0));
            let all_above = BAND.iter().all(|&w| paired(&by, root, g, &|x, y| Some(x.s_at(w) / y.s_at(w))).iter().all(|&v| v > 1.0));
            let band_verdict = if failed_cell {
                "failed cell"
            } else if all_below {
                "root finder ranks below (cheaper) at every weight"
            } else if all_above {
                "root finder ranks above (dearer) at every weight"
            } else {
                "unresolved inside the band"
            };
            // Derived: the weight per unpriced operation at which the two arms
            // cost the same, w* = (G_root − G_g)/(U_g − U_root); the root
            // finder is cheaper above it. Not a §5a ranking.
            let crossover = paired(&by, root, g, &|x, y| {
                let ux = x.unpriced.values().sum::<u64>() as f64;
                let uy = y.unpriced.values().sum::<u64>() as f64;
                (uy != ux).then(|| (x.gae - y.gae) / (uy - ux))
            });
            let entry = json!({
                "derived_crossover_weight_gae": summary(&crossover),
                "band_5a": band,
                "band_verdict": band_verdict,
                "s_ratio": summary(&s),
                "fp_multiplication_ratio": summary(&m),
                "targets_with_inversions_at_most": inv_le.iter().filter(|&&v| v == 1.0).count(),
                "relations_wall_ratio": summary(&wall),
                "s_verdict": if ruled { Value::from(s_verdict) } else { Value::Null },
                "m_verdict": if ruled { Value::from(m_verdict) } else { Value::Null },
            });
            if ruled {
                q1.insert(key.clone(), json!({"s_ratio": entry["s_ratio"], "verdict": s_verdict,
                    "band_5a": entry["band_5a"], "band_verdict": band_verdict,
                    "derived_crossover_weight_gae": entry["derived_crossover_weight_gae"]}));
                q2.insert(key, json!({"fp_multiplication_ratio": entry["fp_multiplication_ratio"],
                    "targets_with_inversions_at_most": entry["targets_with_inversions_at_most"],
                    "relations_wall_ratio": entry["relations_wall_ratio"], "verdict": m_verdict}));
            } else {
                q3.insert(key, entry);
            }
        }
    }
    q.insert("q1_s_root_over_groebner".into(), Value::Object(q1));
    q.insert("q2_fp_multiplications_root_over_groebner".into(), Value::Object(q2));
    q.insert("q3_root_over_f6_enum_and_m3_context".into(), Value::Object(q3));
    // Q4: instrument check against the 2026-10-11 round.
    let mut checks = serde_json::Map::new();
    for arm in ["f4", "sig-f5", "f6"] {
        let Some(now) = by.get(arm) else { continue };
        let (mut same, mut total, mut diffs) = (0usize, 0usize, Vec::new());
        for (w, x) in now {
            let Some(y) = prior.iter().find(|p| p.arm == arm && p.workload == *w) else { continue };
            total += 1;
            let eq = x.solver_ops == y.solver_ops && x.tree == y.tree && x.trials == y.trials
                && x.relations == y.relations && x.s == y.s;
            if eq { same += 1 } else { diffs.push(w.to_string()) }
        }
        checks.insert(arm.into(), json!({"targets_compared": total, "identical": same, "differ": diffs}));
    }
    q.insert("q4_instrument_check_vs_20261011".into(), Value::Object(checks));
    // Q5: context against rho.
    let mut q5 = serde_json::Map::new();
    for arm in by.keys().filter(|a| **a != "rho-negation") {
        let v = paired(&by, arm, "rho-negation", &|x, y| Some(x.s? / y.s?));
        if !v.is_empty() {
            q5.insert(arm.to_string(), summary(&v));
        }
    }
    q.insert("q5_context_s_over_rho".into(), Value::Object(q5));
    json!({"slug": slug, "bits": first.bits, "r": first.r, "failed_arms": failed, "arms": arms, "questions": q})
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        return Err("usage: pkm_rootfind_analyze OUT.json PRIOR_SESSIONS_DIR SESSION_DIR...".into());
    }
    let mut sessions = serde_json::Map::new();
    for dir in &args[3..] {
        let name = std::path::Path::new(dir).file_name().and_then(|s| s.to_str()).unwrap_or(dir).to_string();
        let (id, runs) = read(dir)?;
        let prior = read(&format!("{}/{name}", args[2])).map(|(_, r)| r).unwrap_or_default();
        let mut cells: BTreeMap<String, Vec<&Run>> = BTreeMap::new();
        for r in &runs {
            cells.entry(r.slug.clone()).or_default().push(r);
        }
        let cells: Vec<Value> = cells.iter().map(|(slug, rs)| cell(slug, rs, &prior)).collect();
        sessions.insert(name, json!({"session_id": id, "dir": dir, "cells": cells}));
    }
    let out = json!({
        "schema": "pkm-rootfind-analysis/v1",
        "protocol": "research/pkm_tower_rootfind_20261018/PROTOCOL.md",
        "unit": {"s": "ecbench.gae / sqrt(r); root finders charge every group operation, tower arms leave their F_p multiply-adds unpriced",
                 "fp_multiplications": "derived: 3*adds + 7*doubles (PrimeCurve::FIELD_OPS_PER_ADD/DOUBLE) + solver multiply-adds",
                 "inversions": "derived: adds + doubles + native inversions"},
        "sessions": sessions,
    });
    std::fs::write(&args[1], serde_json::to_string_pretty(&out).map_err(|e| e.to_string())? + "\n")
        .map_err(|e| e.to_string())?;
    println!("wrote {}", args[1]);
    Ok(())
}
