//! Phase 5 step A (exploratory): index-calculus timing per curve from phase 3's
//! ecbench sessions. Kept apart from phase4.rs, whose sha256 a running card binds.

use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::Write;

use serde_json::{json, Value};

use crate::stats::ks_uniform;

fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).map(|i| args[i + 1].clone())
}

/// Friedman test: blocks x treatments matrix (complete), ranks within each block.
fn friedman(m: &[Vec<f64>]) -> (f64, usize, f64) {
    let (nb, k) = (m.len() as f64, m[0].len());
    let mut rsum = vec![0.0; k];
    for row in m {
        let mut idx: Vec<usize> = (0..k).collect();
        idx.sort_by(|&a, &b| row[a].partial_cmp(&row[b]).unwrap());
        let mut i = 0;
        while i < k {
            let mut j = i;
            while j + 1 < k && row[idx[j + 1]] == row[idx[i]] { j += 1; }
            for t in i..=j { rsum[idx[t]] += (i + j) as f64 / 2.0 + 1.0; }
            i = j + 1;
        }
    }
    let kf = k as f64;
    let q = 12.0 / (nb * kf * (kf + 1.0)) * rsum.iter().map(|r| r * r).sum::<f64>() - 3.0 * nb * (kf + 1.0);
    (q, k - 1, crate::stats::gamma_q((k - 1) as f64 / 2.0, q / 2.0))
}


// ------------------------------------------------- IC timing (phase 5 step A)
// `ic-timing`: wall time per decomposition attempt and per relation, per curve,
// from phase 3's ecbench sessions (each run pinned to one CPU, alternate arm order).
// Exploratory: phase 3 was not designed as a timing study (no A/A arm, curve-specific
// planted targets). Every statistic is computed here.
pub fn cmd_ic_timing(args: &[String]) {
    let classes = arg(args, "--classes").expect("--classes");
    let runs = arg(args, "--runs").expect("--runs");
    let out = arg(args, "--out").expect("--out");
    let doc: Value = serde_json::from_str(&fs::read_to_string(&classes).unwrap()).unwrap();
    let mean = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
    let se = |v: &[f64]| { let m = v.iter().sum::<f64>() / v.len() as f64; (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (v.len() as f64 - 1.0) / v.len() as f64).sqrt() };
    let geo = |v: &[f64]| (v.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64).exp();
    let mut per_m = vec![];
    for m in [3u32, 2] {
        let (mut rows, mut fp, mut ratios, mut pc_att, mut pc_rel, mut vcb) = (vec![], vec![], vec![], vec![], vec![], vec![]);
        let (mut total, mut not_l2) = (0usize, 0usize);
        for c in doc["classes"].as_array().unwrap() {
            let id = c["id"].as_str().unwrap();
            let text = match fs::read_to_string(format!("{runs}/{id}_m{m}/session/records.jsonl")) { Ok(t) => t, Err(_) => continue };
            // (arm, curve, target, round) -> (ns per attempt, ns per relation)
            let mut cell: BTreeMap<(String, (u64, u64), u64, u64), (f64, f64)> = BTreeMap::new();
            for line in text.lines() {
                let r: Value = serde_json::from_str(line).unwrap();
                if r["warmup"].as_bool() == Some(true) { continue; }
                total += 1;
                if r["isolation"]["level"].as_str() != Some("L2") { not_l2 += 1; }
                let cs = &r["workload"]["curve_spec"];
                let tried = r["counters"]["targets_tried"].as_f64().unwrap();
                let rel = r["counters"]["relations_found"].as_f64().unwrap();
                let pdp = r["online"]["phases_ns"]["target_PDP"].as_f64().unwrap();
                let wall = r["online"]["wall_ns"].as_f64().unwrap();
                let arm = if r["arm"].as_str().unwrap().ends_with("-pc") { "pc" } else { "main" };
                cell.insert((arm.into(), (cs["a"].as_u64().unwrap(), cs["b"].as_u64().unwrap()), r["workload"]["target_index"].as_u64().unwrap(), r["round"].as_u64().unwrap()),
                            (pdp / tried, if rel > 0.0 { wall / rel } else { f64::NAN }));
            }
            let curves: Vec<(u64, u64)> = { let mut v: Vec<_> = cell.keys().map(|k| k.1).collect(); v.sort(); v.dedup(); v };
            let blocks: Vec<(u64, u64)> = { let mut v: Vec<_> = cell.keys().map(|k| (k.2, k.3)).collect(); v.sort(); v.dedup(); v };
            let mat: Vec<Vec<f64>> = blocks.iter().filter_map(|b| curves.iter().map(|cv| cell.get(&("main".into(), *cv, b.0, b.1)).map(|x| x.0)).collect::<Option<Vec<f64>>>()).collect();
            let (fq, fdf, fpv) = if mat.len() >= 2 && curves.len() >= 2 { friedman(&mat) } else { (f64::NAN, 0, f64::NAN) };
            if fpv.is_finite() { fp.push(fpv); }
            let pcurve: Vec<(String, f64, f64)> = curves.iter().map(|cv| {
                let a: Vec<f64> = cell.iter().filter(|(k, _)| k.0 == "main" && k.1 == *cv).map(|(_, x)| x.0).collect();
                let r: Vec<f64> = cell.iter().filter(|(k, x)| k.0 == "main" && k.1 == *cv && x.1.is_finite()).map(|(_, x)| x.1).collect();
                (format!("a={},b={}", cv.0, cv.1), geo(&a), geo(&r))
            }).collect();
            let mn = pcurve.iter().cloned().fold(("".to_string(), f64::INFINITY, 0.0), |b, x| if x.1 < b.1 { x } else { b });
            let mx = pcurve.iter().cloned().fold(("".to_string(), 0.0, 0.0), |b, x| if x.1 > b.1 { x } else { b });
            let mnr = pcurve.iter().map(|x| x.2).fold(f64::INFINITY, f64::min);
            let mxr = pcurve.iter().map(|x| x.2).fold(0.0, f64::max);
            if curves.len() >= 2 { ratios.push(mx.1 / mn.1); }
            // variance components on log ns/attempt
            let logs: Vec<Vec<f64>> = curves.iter().map(|cv| cell.iter().filter(|(k, _)| k.0 == "main" && k.1 == *cv).map(|(_, x)| x.0.ln()).collect()).collect();
            if logs.len() >= 2 {
                let means: Vec<f64> = logs.iter().map(|v| v.iter().sum::<f64>() / v.len() as f64).collect();
                let gm = means.iter().sum::<f64>() / means.len() as f64;
                let vm = means.iter().map(|x| (x - gm).powi(2)).sum::<f64>() / (means.len() as f64 - 1.0);
                let samp = logs.iter().zip(&means).map(|(v, mu)| v.iter().map(|x| (x - mu).powi(2)).sum::<f64>() / (v.len() as f64 - 1.0) / v.len() as f64).sum::<f64>() / logs.len() as f64;
                vcb.push(vm - samp);
            }
            for (k, x) in &cell {
                if k.0 == "pc" { if let Some(b) = cell.get(&("main".into(), k.1, k.2, k.3)) {
                    pc_att.push((x.0 / b.0).ln());
                    if x.1.is_finite() && b.1.is_finite() { pc_rel.push((x.1 / b.1).ln()); }
                } }
            }
            rows.push(json!({"id": id, "stratum": c["stratum"], "curves": curves.len(), "complete_blocks": mat.len(),
                "friedman_ns_per_attempt": {"Q": fq, "df": fdf, "p": fpv},
                "fastest": {"curve": mn.0, "ns_per_attempt": mn.1}, "slowest": {"curve": mx.0, "ns_per_attempt": mx.1},
                "max_over_min_ns_per_attempt": mx.1 / mn.1, "max_over_min_ns_per_relation": mxr / mnr}));
        }
        let (d, ksp) = ks_uniform(&fp);
        let mut sr = ratios.clone(); sr.sort_by(|a, b| a.partial_cmp(b).unwrap());
        per_m.push(json!({"m": m, "classes": rows.len(), "records_measured": total, "records_not_L2": not_l2,
            "friedman_ks_uniform": {"classes": fp.len(), "D": d, "p": ksp, "min_p": fp.iter().cloned().fold(1.0, f64::min),
                                    "classes_p_below_0.05_over_classes": fp.iter().filter(|&&p| p < 0.05 / fp.len() as f64).count()},
            "median_max_over_min_ns_per_attempt": sr.get(sr.len() / 2), "largest_max_over_min_ns_per_attempt": sr.last(),
            "between_curve_sd_log_ns_per_attempt": {"estimate": mean(&vcb).max(0.0).sqrt(), "upper95": (mean(&vcb) + 1.645 * se(&vcb)).max(0.0).sqrt()},
            "positive_control_pc_over_main": {"ns_per_attempt_ratio": mean(&pc_att).exp(), "ns_per_attempt_log_se": se(&pc_att),
                                              "ns_per_relation_ratio": mean(&pc_rel).exp(), "ns_per_relation_log_se": se(&pc_rel), "n": pc_att.len()},
            "per_class": rows}));
    }
    let report = json!({"schema": "fp31.phase5a.ic-timing/v1",
        "caveat": "phase-3 ecbench sessions: runs pinned to a reserved CPU and graded per row, alternate arm order; no A/A arm and curve-specific targets, so exploratory",
        "by_m": per_m});
    let mut f = OpenOptions::new().write(true).create_new(true).open(&out).expect("output exists");
    f.write_all(serde_json::to_string_pretty(&report).unwrap().as_bytes()).unwrap();
}
