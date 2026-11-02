//! Preregistered analysis: paired ratios against E0 with percentile-bootstrap CIs (resampling
//! targets), one-way ICC(1) of the curve effect for the class / A/A / random-curve sets, and
//! exact-profile agreement.  Reads rows.jsonl written by `run`.

use serde_json::{json, Value};
use std::collections::BTreeMap;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

fn median(v: &mut Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        0.5 * (v[n / 2 - 1] + v[n / 2])
    }
}

fn pct(v: &mut Vec<f64>, q: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let i = ((v.len() - 1) as f64 * q).round() as usize;
    v[i]
}

/// Curve variance share from the crossed two-way random-effects ANOVA (curves x targets, one
/// observation per cell, no interaction): sigma_c^2 / (sigma_c^2 + sigma_t^2 + sigma_e^2) with
/// negative component estimates clipped at 0.  This is the ICC for "which curve" with the
/// shared targets as a blocking factor.  y[curve][target]; None if the total variance is 0.
fn icc1(y: &[Vec<f64>]) -> Option<f64> {
    let g = y.len();
    let k = y[0].len();
    if g < 2 || k < 2 {
        return None;
    }
    let rm: Vec<f64> = y.iter().map(|r| r.iter().sum::<f64>() / k as f64).collect();
    let cm: Vec<f64> = (0..k).map(|j| y.iter().map(|r| r[j]).sum::<f64>() / g as f64).collect();
    let grand = rm.iter().sum::<f64>() / g as f64;
    let msc = k as f64 * rm.iter().map(|m| (m - grand).powi(2)).sum::<f64>() / (g as f64 - 1.0);
    let mst = g as f64 * cm.iter().map(|m| (m - grand).powi(2)).sum::<f64>() / (k as f64 - 1.0);
    let mut sse = 0.0;
    for i in 0..g {
        for j in 0..k {
            sse += (y[i][j] - rm[i] - cm[j] + grand).powi(2);
        }
    }
    let mse = sse / ((g as f64 - 1.0) * (k as f64 - 1.0));
    let sc = ((msc - mse) / k as f64).max(0.0);
    let st = ((mst - mse) / g as f64).max(0.0);
    let tot = sc + st + mse;
    if tot <= 0.0 {
        None
    } else {
        Some(sc / tot)
    }
}

const METRICS: [&str; 8] =
    ["cpu_s", "field_mults", "final_rows", "final_cols", "final_rank", "D_solve", "first_fall_degree_obs", "n_standard_monomials_final"];

pub fn cmd_analyze(args: &[String]) {
    let rows_path = crate::arg(args, "--rows", "rows.jsonl");
    let out_path = crate::arg(args, "--out", "analysis.json");
    let boots: usize = crate::arg(args, "--boot", "4000").parse().unwrap();
    let level: f64 = crate::arg(args, "--level", "0.95").parse().unwrap();
    let (qlo, qhi) = ((1.0 - level) / 2.0, 1.0 - (1.0 - level) / 2.0);
    let mask = args.iter().any(|a| a == "--mask");
    let text = std::fs::read_to_string(&rows_path).unwrap();
    let rows: Vec<Value> = text.lines().filter_map(|l| serde_json::from_str(l).ok()).filter(|r: &Value| r["schema"] == "p1.row/v1").collect();
    // (config, kind) -> arm -> tau -> reps
    let mut data: BTreeMap<(String, String), BTreeMap<String, BTreeMap<u64, Vec<&Value>>>> = BTreeMap::new();
    let mut role: BTreeMap<String, String> = BTreeMap::new();
    for r in &rows {
        if r.get("error").is_some() {
            continue;
        }
        let key = (r["config"].as_str().unwrap().to_string(), r["kind"].as_str().unwrap().to_string());
        role.insert(r["arm"].as_str().unwrap().to_string(), r["role"].as_str().unwrap().to_string());
        data.entry(key).or_default().entry(r["arm"].as_str().unwrap().to_string()).or_default().entry(r["tau"].as_u64().unwrap()).or_default().push(r);
    }
    let mut report = Vec::new();
    let mut mask_key: BTreeMap<String, String> = BTreeMap::new();
    for ((cfg, kind), arms) in &data {
        // value of metric for (arm, tau): median over reps for cpu_s, first rep for counts
        let val = |arm: &str, tau: u64, m: &str| -> Option<f64> {
            let reps = arms.get(arm)?.get(&tau)?;
            if m == "cpu_s" {
                let mut v: Vec<f64> = reps.iter().filter_map(|r| r[m].as_f64()).collect();
                if v.is_empty() {
                    None
                } else {
                    Some(median(&mut v))
                }
            } else {
                reps[0][m].as_f64()
            }
        };
        let mismatch = arms.values().flat_map(|t| t.values()).filter(|reps| {
            reps.iter().any(|r| r["final_rank"] != reps[0]["final_rank"] || r["D_solve"] != reps[0]["D_solve"] || r["final_rows"] != reps[0]["final_rows"])
        }).count();
        let taus: Vec<u64> = arms.get("E0").map(|t| t.keys().copied().collect()).unwrap_or_default();
        let mut rng = Rng(20261102);
        let mut per_arm = Vec::new();
        for arm in arms.keys() {
            if arm == "E0" {
                continue;
            }
            let shown = if mask && arm != "AA_repeat" && arm != "AA_perm" { format!("X{}", &crate::sha_hex(&format!("mask|{arm}"))[..8]) } else { arm.clone() };
            if mask && shown != *arm {
                mask_key.insert(shown.clone(), arm.clone());
            }
            let mut ent = json!({"arm": shown, "role": if mask && shown != *arm { "masked".to_string() } else { role[arm].clone() }});
            let common: Vec<u64> = taus.iter().copied().filter(|t| arms[arm].contains_key(t)).collect();
            if common.is_empty() || arms[arm].values().next().map_or(true, |_| false) {
                continue;
            }
            // exact profile agreement with E0 (counts)
            let same = common.iter().filter(|&&t| {
                let a = arms[arm][&t][0];
                let e = arms["E0"][&t][0];
                ["D_solve", "final_rank", "final_rows", "final_cols", "first_fall_degree_obs", "rank_profile_by_leading_degree", "status"].iter().all(|k| a[k] == e[k])
            }).count();
            ent["identical_count_profile_fraction"] = json!(same as f64 / common.len() as f64);
            ent["n_pairs"] = json!(common.len());
            let mut ms = serde_json::Map::new();
            for m in METRICS {
                let pairs: Vec<(f64, f64)> = common.iter().filter_map(|&t| Some((val(arm, t, m)?, val("E0", t, m)?))).filter(|(a, e)| *a > 0.0 && *e > 0.0).collect();
                if pairs.is_empty() {
                    continue;
                }
                let lr: Vec<f64> = pairs.iter().map(|(a, e)| (a / e).ln()).collect();
                let est = (lr.iter().sum::<f64>() / lr.len() as f64).exp();
                let mut bs = Vec::new();
                for _ in 0..boots {
                    let mut s = 0.0;
                    for _ in 0..lr.len() {
                        s += lr[rng.below(lr.len())];
                    }
                    bs.push((s / lr.len() as f64).exp());
                }
                ms.insert(m.to_string(), json!({"geo_mean_ratio_vs_E0": est, "ci95": [pct(&mut bs.clone(), qlo), pct(&mut bs, qhi)], "n": pairs.len()}));
            }
            ent["metrics"] = Value::Object(ms);
            per_arm.push(ent);
        }
        // ICC per set
        let sets: Vec<(&str, Vec<String>)> = vec![
            ("class_E0_plus_neighbours", arms.keys().filter(|a| *a == "E0" || role[*a] == "neighbour").cloned().collect()),
            ("aa_E0_repeat_perm", arms.keys().filter(|a| ["E0", "AA_repeat", "AA_perm"].contains(&a.as_str())).cloned().collect()),
            ("random_curves", arms.keys().filter(|a| role[*a] == "random_curve_null_population").cloned().collect()),
        ];
        let mut icc_out = serde_json::Map::new();
        for (name, set) in sets {
            let common: Vec<u64> = taus.iter().copied().filter(|t| set.iter().all(|a| arms.get(a).map_or(false, |x| x.contains_key(t)))).collect();
            if set.len() < 2 || common.len() < 2 {
                continue;
            }
            let mut mm = serde_json::Map::new();
            for m in METRICS {
                let y: Vec<Vec<f64>> = set.iter().map(|a| common.iter().filter_map(|&t| val(a, t, m)).collect::<Vec<f64>>()).collect();
                if y.iter().any(|r| r.len() != common.len()) {
                    continue;
                }
                let est = icc1(&y);
                let mut bs = Vec::new();
                if est.is_some() {
                    for _ in 0..boots.min(2000) {
                        let idx: Vec<usize> = (0..common.len()).map(|_| rng.below(common.len())).collect();
                        let yy: Vec<Vec<f64>> = y.iter().map(|r| idx.iter().map(|&i| r[i]).collect()).collect();
                        if let Some(v) = icc1(&yy) {
                            bs.push(v);
                        }
                    }
                }
                let ci = if bs.len() > 50 { json!([pct(&mut bs.clone(), qlo), pct(&mut bs, qhi)]) } else { Value::Null };
                let all_equal = y.iter().flatten().all(|v| *v == y[0][0]);
                mm.insert(m.to_string(), json!({"curve_variance_share": est, "ci95_target_resample": ci, "all_values_equal": all_equal}));
            }
            icc_out.insert(name.to_string(), json!({"n_curves": set.len(), "n_targets": common.len(), "metrics": mm}));
        }
        report.push(json!({"config": cfg, "kind": kind, "n_arms": arms.len(), "rep_count_mismatch_cells": mismatch, "per_arm": per_arm, "icc": icc_out}));
    }
    let out = json!({"schema": "p1.analysis/v1", "rows": rows.len(), "boot": boots, "ci_level": level, "arm_labels_masked": mask, "cells": report});
    std::fs::write(&out_path, serde_json::to_string_pretty(&out).unwrap()).unwrap();
    if mask {
        let key = serde_json::to_string_pretty(&mask_key).unwrap();
        std::fs::write(format!("{out_path}.key.json"), &key).unwrap();
        println!("(arm labels masked; key sha256 {})", crate::sha_hex(&key));
    }
    // compact text summary
    for c in out["cells"].as_array().unwrap() {
        println!("== {} {} ({} arms, rep-count mismatches {}) ==", c["config"], c["kind"], c["n_arms"], c["rep_count_mismatch_cells"]);
        for a in c["per_arm"].as_array().unwrap() {
            let g = |m: &str| a["metrics"][m]["geo_mean_ratio_vs_E0"].as_f64().map_or("-".to_string(), |v| format!("{v:.3}"));
            println!(
                "  {:<10} identical_profiles={:.2} cpu_ratio={} mults_ratio={} rank_ratio={} D_ratio={}",
                a["arm"].as_str().unwrap(),
                a["identical_count_profile_fraction"].as_f64().unwrap(),
                g("cpu_s"),
                g("field_mults"),
                g("final_rank"),
                g("D_solve")
            );
        }
    }
}
