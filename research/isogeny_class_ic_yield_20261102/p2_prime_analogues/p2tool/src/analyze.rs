//! `p2tool analyze`: variance partition of per-curve metrics across one class.
//!
//! Reads ecbench `records.jsonl` sessions of one class and one m, and for each
//! metric (yield per attempt, relations-phase GAE per attempt, online wall ns
//! per attempt) reports, for the real curves of the class under the `main` arm:
//!   * ICC(1) curve share of variance, unbalanced one-way ANOVA estimator,
//!     with a two-level bootstrap one-sided 95% upper bound and a permutation
//!     p-value (cells permuted across curves);
//!   * pooled per-curve means, max/min ratio, and the permutation null of that
//!     ratio; the A/A spread (round-parity halves of the same curve);
//!   * the isomorphic-model control: |log ratio| of each u-scaled model to its
//!     base curve against the A/A spread;
//!   * the positive control: the pc arm (planted +20% expected yield) is
//!     substituted into one curve at a time, and the detectors are re-run.
use crate::arg;
use serde_json::{json, Value};
use std::collections::BTreeMap;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

#[derive(Clone, Debug)]
struct Cell {
    round: u32,
    target: u32,
    y: f64,
    rel_gae: f64,
    wall: f64,
    setup_gae: f64,
    rel: f64,
    att: f64,
    iso_level: String,
}
const METRICS: [&str; 3] = ["yield", "rel_gae_per_attempt", "online_wall_ns_per_attempt"];
fn metric(c: &Cell, m: usize) -> f64 {
    [c.y, c.rel_gae, c.wall][m]
}

/// ICC(1) with unbalanced groups: returns (sigma2_between, sigma2_within).
fn anova(groups: &[Vec<f64>]) -> (f64, f64) {
    let g: Vec<&Vec<f64>> = groups.iter().filter(|v| !v.is_empty()).collect();
    let k = g.len() as f64;
    let n: f64 = g.iter().map(|v| v.len() as f64).sum();
    let grand: f64 = g.iter().flat_map(|v| v.iter()).sum::<f64>() / n;
    let mut ssb = 0.0;
    let mut ssw = 0.0;
    for v in &g {
        let m = v.iter().sum::<f64>() / v.len() as f64;
        ssb += v.len() as f64 * (m - grand).powi(2);
        ssw += v.iter().map(|x| (x - m).powi(2)).sum::<f64>();
    }
    let msb = ssb / (k - 1.0);
    let msw = ssw / (n - k);
    let n0 = (n - g.iter().map(|v| (v.len() as f64).powi(2)).sum::<f64>() / n) / (k - 1.0);
    ((msb - msw) / n0, msw)
}
fn icc(groups: &[Vec<f64>]) -> f64 {
    let (b, w) = anova(groups);
    b / (b + w) // raw (may be negative); clipped only for reporting
}
fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}
fn quantile(v: &mut Vec<f64>, q: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[((v.len() - 1) as f64 * q).round() as usize]
}
fn cv_of_means(groups: &[Vec<f64>]) -> f64 {
    let ms: Vec<f64> = groups.iter().map(|v| mean(v)).collect();
    let m = mean(&ms);
    (ms.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (ms.len() as f64 - 1.0)).sqrt() / m
}
fn ratio_range(groups: &[Vec<f64>]) -> f64 {
    let ms: Vec<f64> = groups.iter().map(|v| mean(v)).collect();
    ms.iter().cloned().fold(f64::MIN, f64::max) / ms.iter().cloned().fold(f64::MAX, f64::min)
}
/// max over curves of |z| of (curve mean - mean of the others) / its standard error (pooled within SD).
fn max_z(groups: &[Vec<f64>]) -> f64 {
    let (_, w) = anova(groups);
    let sd = w.max(1e-300).sqrt();
    let n: f64 = groups.iter().map(|v| v.len() as f64).sum();
    let tot: f64 = groups.iter().flat_map(|v| v.iter()).sum();
    let mut best = 0.0f64;
    for v in groups {
        let k = v.len() as f64;
        let s: f64 = v.iter().sum();
        let rest = (tot - s) / (n - k);
        let z = (s / k - rest) / (sd * (1.0 / k + 1.0 / (n - k)).sqrt());
        best = best.max(z.abs());
    }
    best
}
fn permuted(groups: &[Vec<f64>], rng: &mut Rng) -> Vec<Vec<f64>> {
    let mut all: Vec<f64> = groups.iter().flat_map(|v| v.iter().cloned()).collect();
    for i in (1..all.len()).rev() {
        all.swap(i, rng.below(i + 1));
    }
    let mut out = vec![];
    let mut at = 0;
    for v in groups {
        out.push(all[at..at + v.len()].to_vec());
        at += v.len();
    }
    out
}

fn analyse_metric(groups_all: &[Vec<f64>], pc_all: &[Vec<f64>], boots: usize, perms: usize, plants: usize, rng: &mut Rng) -> Value {
    // curves without data (a partial run) are dropped from every statistic
    let keep: Vec<usize> = (0..groups_all.len()).filter(|&i| groups_all[i].len() >= 2).collect();
    let groups: &[Vec<f64>] = &keep.iter().map(|&i| groups_all[i].clone()).collect::<Vec<_>>();
    let pc: &[Vec<f64>] = &if pc_all.is_empty() { vec![] } else { keep.iter().map(|&i| pc_all[i].clone()).collect::<Vec<_>>() };
    let h = groups.len();
    let icc_raw = icc(groups);
    let (sb, sw) = anova(groups);
    // two-level bootstrap: curves with replacement, then cells within each drawn curve
    let mut bs = vec![];
    for _ in 0..boots {
        let mut g = vec![];
        for _ in 0..h {
            let v = &groups[rng.below(h)];
            g.push((0..v.len()).map(|_| v[rng.below(v.len())]).collect::<Vec<f64>>());
        }
        bs.push(icc(&g).max(0.0));
    }
    let mut null_icc = vec![];
    let mut null_rr = vec![];
    let mut null_z = vec![];
    for _ in 0..perms {
        let g = permuted(groups, rng);
        null_icc.push(icc(&g));
        null_rr.push(ratio_range(&g));
        null_z.push(max_z(&g));
    }
    let p_icc = null_icc.iter().filter(|&&x| x >= icc_raw).count() as f64 + 1.0;
    let rr = ratio_range(groups);
    let p_rr = null_rr.iter().filter(|&&x| x >= rr).count() as f64 + 1.0;
    let mz = max_z(groups);
    let p_z = null_z.iter().filter(|&&x| x >= mz).count() as f64 + 1.0;
    let (icc_crit, rr_crit, z_crit) = (quantile(&mut null_icc.clone(), 0.95), quantile(&mut null_rr.clone(), 0.95), quantile(&mut null_z.clone(), 0.95));
    // A/A: even vs odd round halves are handled by the caller; here the within-curve noise.
    let mut out = json!({
        "curves": h, "cells_per_curve": mean(&groups.iter().map(|v| v.len() as f64).collect::<Vec<_>>()),
        "grand_mean": mean(&groups.iter().flat_map(|v| v.iter().cloned()).collect::<Vec<_>>()),
        "icc_raw": icc_raw, "icc_clipped": icc_raw.max(0.0),
        "icc_bootstrap_upper95": quantile(&mut bs.clone(), 0.95),
        "icc_bootstrap_median": quantile(&mut bs, 0.5),
        "icc_perm_p": p_icc / (perms as f64 + 1.0), "icc_perm_null_q95": icc_crit,
        "sigma2_between": sb, "sigma2_within": sw,
        "curve_mean_cv": cv_of_means(groups),
        "max_over_min": rr, "max_over_min_perm_p": p_rr / (perms as f64 + 1.0), "max_over_min_perm_null_q95": rr_crit,
        "max_abs_z": mz, "max_abs_z_perm_p": p_z / (perms as f64 + 1.0), "max_abs_z_perm_null_q95": z_crit,
    });
    // positive control: substitute the pc cells of one curve at a time
    if !pc.is_empty() && plants > 0 {
        let mut hit_icc = 0;
        let mut hit_z = 0;
        let mut hit_rr = 0;
        let mut trials = 0;
        let mut ratios = vec![];
        for _ in 0..plants {
            let c = rng.below(h);
            if pc[c].is_empty() {
                continue;
            }
            let mut g = groups.to_vec();
            ratios.push(mean(&pc[c]) / mean(&groups[c]));
            g[c] = pc[c].clone();
            trials += 1;
            hit_icc += (icc(&g) > icc_crit) as u32;
            hit_z += (max_z(&g) > z_crit) as u32;
            hit_rr += (ratio_range(&g) > rr_crit) as u32;
        }
        out["positive_control"] = json!({
            "planted_curve_trials": trials,
            "detect_icc_gt_null_q95": hit_icc as f64 / trials as f64,
            "detect_max_abs_z_gt_null_q95": hit_z as f64 / trials as f64,
            "detect_max_over_min_gt_null_q95": hit_rr as f64 / trials as f64,
            "planted_effect_pc_over_main_median": quantile(&mut ratios, 0.5),
            "note": "null thresholds are the 95th percentile of the cell-permutation null of the unplanted data"});
    }
    out
}

pub fn run(args: &[String]) {
    let classes_path = arg(args, "--classes").expect("--classes");
    let cid = arg(args, "--class").expect("--class c24A");
    let m: u32 = arg(args, "--m").map(|s| s.parse().unwrap()).unwrap_or(2);
    let boots: usize = arg(args, "--boots").map(|s| s.parse().unwrap()).unwrap_or(2000);
    let perms: usize = arg(args, "--perms").map(|s| s.parse().unwrap()).unwrap_or(2000);
    let plants: usize = arg(args, "--plants").map(|s| s.parse().unwrap()).unwrap_or(200);
    let seed: u64 = arg(args, "--seed").map(|s| s.parse().unwrap()).unwrap_or(20261102);
    let out = arg(args, "--out").unwrap_or("analysis.json".into());
    let sessions: Vec<String> = args.iter().enumerate().filter(|(_, a)| *a == "--session").filter_map(|(i, _)| args.get(i + 1).cloned()).collect();
    assert!(!sessions.is_empty(), "--session DIR (repeatable)");
    let doc: Value = serde_json::from_str(&std::fs::read_to_string(&classes_path).unwrap()).unwrap();
    let cls = doc["classes"].as_array().unwrap().iter().find(|c| c["id"] == cid.as_str()).expect("class");
    // curve key (a,b) -> ("real", idx) | ("iso", idx)
    let mut key: BTreeMap<(u64, u64), (bool, usize)> = BTreeMap::new();
    for (i, c) in cls["curves"].as_array().unwrap().iter().enumerate() {
        key.insert((c["a"].as_u64().unwrap(), c["b"].as_u64().unwrap()), (false, i));
    }
    for (i, c) in cls["iso_models"].as_array().unwrap().iter().enumerate() {
        key.insert((c["a"].as_u64().unwrap(), c["b"].as_u64().unwrap()), (true, i));
    }
    let (n_real, n_iso) = (cls["curves"].as_array().unwrap().len(), cls["iso_models"].as_array().unwrap().len());
    let arms = [format!("m{m}-main"), format!("m{m}-aa"), format!("m{m}-pc")];
    // cells[arm][(is_iso, idx)] -> Vec<Cell>
    let mut cells: BTreeMap<(usize, bool, usize), Vec<Cell>> = BTreeMap::new();
    let mut determinism = (0u64, 0u64); // main vs aa identical relation counts
    let mut aa_wall: Vec<f64> = vec![];
    let mut levels: BTreeMap<String, u64> = BTreeMap::new();
    let mut status: BTreeMap<String, u64> = BTreeMap::new();
    let mut by_run: BTreeMap<(usize, bool, usize, u32, u32, usize), (f64, f64)> = BTreeMap::new();
    for (si, dir) in sessions.iter().enumerate() {
        let text = std::fs::read_to_string(format!("{dir}/records.jsonl")).unwrap();
        for line in text.lines() {
            let r: Value = serde_json::from_str(line).unwrap();
            if r["warmup"].as_bool().unwrap_or(false) {
                continue;
            }
            let ai = match arms.iter().position(|a| r["arm"] == a.as_str()) {
                Some(i) => i,
                None => continue,
            };
            let cs = &r["workload"]["curve_spec"];
            let k = match key.get(&(cs["a"].as_u64().unwrap(), cs["b"].as_u64().unwrap())) {
                Some(k) => *k,
                None => continue,
            };
            let att = r["counters"]["targets_tried"].as_f64().unwrap();
            let rel = r["counters"]["relations_found"].as_f64().unwrap();
            let phase = |n: &str| r["phases"].as_array().unwrap().iter().find(|p| p["name"] == n).map(|p| p["gae"].as_f64().unwrap_or(0.0)).unwrap_or(0.0);
            let level = r["isolation"]["level"].as_str().unwrap_or("?").to_string();
            *levels.entry(level.clone()).or_default() += 1;
            *status.entry(r["outcome"]["status"].as_str().unwrap_or("?").to_string()).or_default() += 1;
            let round = r["round"].as_u64().unwrap() as u32 + 1000 * si as u32;
            let target = r["workload"]["target_index"].as_u64().unwrap() as u32;
            let wall = r["online"]["wall_ns"].as_f64().unwrap_or(f64::NAN);
            by_run.insert((ai, k.0, k.1, round, target, si), (rel, wall));
            cells.entry((ai, k.0, k.1)).or_default().push(Cell {
                round, target, y: rel / att, rel_gae: phase("relations") / att, wall: wall / att, setup_gae: phase("oracle_setup"), rel, att, iso_level: level,
            });
        }
    }
    for (&(ai, iso, ci, round, target, si), &(rel, wall)) in by_run.iter() {
        if ai == 0 {
            if let Some(&(rel2, wall2)) = by_run.get(&(1, iso, ci, round, target, si)) {
                determinism.1 += 1;
                determinism.0 += (rel == rel2) as u64;
                if !iso && wall.is_finite() && wall2.is_finite() {
                    aa_wall.push((wall / wall2).ln());
                }
            }
        }
    }
    let mut rng = Rng(seed);
    let groups = |arm: usize, iso: bool, n: usize, mi: usize| -> Vec<Vec<f64>> {
        (0..n).map(|c| cells.get(&(arm, iso, c)).map(|v| v.iter().map(|x| metric(x, mi)).filter(|x| x.is_finite()).collect()).unwrap_or_default()).collect()
    };
    let mut report = json!({"schema": "p2.analysis/v1", "class": cid, "m": m, "sessions": sessions, "seed": seed,
        "bootstrap": boots, "permutations": perms, "plants": plants,
        "isolation_levels_of_rows": levels, "outcome_status_of_rows": status,
        "class_facts": {"D": cls["D"], "h": cls["h"], "p": cls["p"], "n": cls["n"], "log2_p": cls["log2_p"]},
        "aa_in_session_identical_relation_counts": {"identical": determinism.0, "pairs": determinism.1,
           "note": "ecbench derives one algorithm seed per (workload, round) shared by every arm, so the in-session A/A arm reproduces the counts exactly (determinism check) and differs only in wall; the yield A/A is the round-parity split below"},
        "aa_online_wall_log_ratio": if aa_wall.is_empty() { Value::Null } else { json!({"n": aa_wall.len(), "mean": mean(&aa_wall), "sd": (aa_wall.iter().map(|x| (x - mean(&aa_wall)).powi(2)).sum::<f64>() / (aa_wall.len() as f64 - 1.0).max(1.0)).sqrt()}) },
    });
    let mut mets = serde_json::Map::new();
    for (mi, name) in METRICS.iter().enumerate() {
        let g = groups(0, false, n_real, mi);
        if g.iter().filter(|v| v.len() >= 2).count() < 3 {
            continue;
        }
        let pcg = groups(2, false, n_real, mi);
        let mut res = analyse_metric(&g, if mi == 0 { &pcg } else { &[] }, boots, perms, if mi == 0 { plants } else { 0 }, &mut rng);
        // A/A: round-parity halves of the same curve
        let mut half: Vec<(f64, f64)> = vec![];
        for c in 0..n_real {
            let cs = match cells.get(&(0, false, c)) { Some(v) => v, None => continue };
            let (mut a, mut b) = (vec![], vec![]);
            for x in cs {
                let v = metric(x, mi);
                if !v.is_finite() { continue; }
                if x.round % 2 == 0 { a.push(v) } else { b.push(v) }
            }
            if !a.is_empty() && !b.is_empty() {
                half.push((mean(&a), mean(&b)));
            }
        }
        let lr: Vec<f64> = half.iter().map(|(a, b)| (a / b).ln()).collect();
        if !lr.is_empty() {
            let mut abs: Vec<f64> = lr.iter().map(|x| x.abs()).collect();
            res["aa_round_halves"] = json!({"curves": lr.len(), "sd_log_ratio": (lr.iter().map(|x| x * x).sum::<f64>() / lr.len() as f64).sqrt(),
                "median_abs_log_ratio": quantile(&mut abs.clone(), 0.5), "q95_abs_log_ratio": quantile(&mut abs, 0.95),
                "max_ratio": lr.iter().map(|x| x.abs()).fold(0.0, f64::max).exp()});
        }
        // isomorphic-model control
        let ig = groups(0, true, n_iso, mi);
        let mut iso_lr = vec![];
        for (i, c) in cls["iso_models"].as_array().unwrap().iter().enumerate() {
            let base = c["of"].as_u64().unwrap() as usize;
            if ig[i].is_empty() || g[base].is_empty() { continue; }
            iso_lr.push((mean(&ig[i]) / mean(&g[base])).ln());
        }
        if !iso_lr.is_empty() {
            let abs: Vec<f64> = iso_lr.iter().map(|x| x.abs()).collect();
            res["isomorphic_model_control"] = json!({"pairs": iso_lr.len(), "rms_log_ratio": (iso_lr.iter().map(|x| x * x).sum::<f64>() / iso_lr.len() as f64).sqrt(),
                "median_abs_log_ratio": quantile(&mut abs.clone(), 0.5), "max_ratio": abs.iter().cloned().fold(0.0, f64::max).exp(),
                "note": "a u-scaling changes x to u^2 x and so the smallest-abscissa factor base; the same curve under another model is the presentation noise"});
        }
        mets.insert(name.to_string(), res);
    }
    report["metrics"] = Value::Object(mets);
    // per-curve table for the primary metric
    let mut table = vec![];
    for c in 0..n_real {
        if let Some(v) = cells.get(&(0, false, c)) {
            let rel: f64 = v.iter().map(|x| x.rel).sum();
            let att: f64 = v.iter().map(|x| x.att).sum();
            table.push(json!({"curve": c, "j": cls["curves"][c]["j"], "relations": rel, "attempts": att, "pooled_yield": rel / att,
                              "setup_gae_mean": mean(&v.iter().map(|x| x.setup_gae).collect::<Vec<_>>())}));
        }
    }
    report["per_curve"] = Value::Array(table);
    std::fs::write(&out, serde_json::to_string_pretty(&report).unwrap()).unwrap();
    eprintln!("wrote {out}");
}
