//! Phase 4 step A (exploratory): wall time per rho iteration across the curves
//! of each class, read from phase 2's isoclass-rho trials.csv (column ns per
//! solve, iters per solve). Recorded at isolation L0 by a 4-thread rayon pool,
//! so this is descriptive: it can show an effect large against the A/A spread,
//! not establish a small one. Every statistic is computed here.

use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::Write;

use serde_json::{json, Value};

use crate::stats::{kruskal, ks_uniform};

fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).map(|i| args[i + 1].clone())
}

pub fn cmd_timing(args: &[String]) {
    let manifest = arg(args, "--manifest").expect("--manifest");
    let runs = arg(args, "--runs").expect("--runs");
    let out = arg(args, "--out").expect("--out");
    let man: Value = serde_json::from_str(&fs::read_to_string(&manifest).unwrap()).unwrap();
    let (mut rows, mut kw_iso, mut kw_null) = (vec![], vec![], vec![]);
    let (mut all_ratio, mut all_null_ratio) = (vec![], vec![]);
    for c in man.as_array().unwrap() {
        let id = c["id"].as_str().unwrap();
        let text = match fs::read_to_string(format!("{runs}/{id}/class/trials.csv")) { Ok(t) => t, Err(_) => continue };
        // per arm: per-solve ns/iter, and pooled sum ns / sum iters
        let mut arms: BTreeMap<String, (String, Vec<f64>, f64, f64)> = BTreeMap::new();
        for line in text.lines().skip(1) {
            let f: Vec<&str> = line.split(',').collect();
            let (iters, ns): (f64, f64) = (f[6].parse().unwrap(), f[10].parse().unwrap());
            if iters <= 0.0 { continue; }
            let e = arms.entry(f[0].to_string()).or_insert((f[1].to_string(), vec![], 0.0, 0.0));
            e.1.push(ns / iters);
            e.2 += ns;
            e.3 += iters;
        }
        let pooled = |k: &str| arms.get(k).map(|a| a.2 / a.3);
        // isogenous curves of the class (v*), and null arms: same curve v0, A/A replica, isomorphic models
        let iso: Vec<(&String, f64)> = arms.iter().filter(|(_, a)| a.0 == "isogenous").map(|(k, a)| (k, a.2 / a.3)).collect();
        let null: Vec<(&String, f64)> = arms.iter().filter(|(k, a)| *k == "v0" || a.0 == "aa-replica" || a.0 == "iso-control").map(|(k, a)| (k, a.2 / a.3)).collect();
        let mm = |v: &[(&String, f64)]| {
            let mn = v.iter().cloned().fold((None, f64::INFINITY), |b, x| if x.1 < b.1 { (Some(x.0.clone()), x.1) } else { b });
            let mx = v.iter().cloned().fold((None, 0.0), |b, x| if x.1 > b.1 { (Some(x.0.clone()), x.1) } else { b });
            (mn, mx)
        };
        let ((mn_k, mn), (mx_k, mx)) = mm(&iso);
        let ((_, nmn), (_, nmx)) = mm(&null);
        let p_iso = if iso.len() >= 2 {
            let g: Vec<Vec<f64>> = arms.values().filter(|a| a.0 == "isogenous").map(|a| a.1.clone()).collect();
            Some(kruskal(&g).2)
        } else { None };
        let p_null = {
            let g: Vec<Vec<f64>> = arms.iter().filter(|(k, a)| *k == "v0" || a.0 == "aa-replica" || a.0 == "iso-control").map(|(_, a)| a.1.clone()).collect();
            if g.len() >= 2 { Some(kruskal(&g).2) } else { None }
        };
        if let Some(p) = p_iso { kw_iso.push(p); }
        if let Some(p) = p_null { kw_null.push(p); }
        if iso.len() >= 2 { all_ratio.push(mx / mn); }
        if null.len() >= 2 { all_null_ratio.push(nmx / nmn); }
        rows.push(json!({"id": id, "stratum": c["stratum"], "curves": iso.len(),
            "fastest": {"arm": mn_k, "ns_per_iter": mn}, "slowest": {"arm": mx_k, "ns_per_iter": mx},
            "max_over_min_isogenous": if iso.len() >= 2 { json!(mx / mn) } else { Value::Null },
            "max_over_min_same_curve_arms": if null.len() >= 2 { json!(nmx / nmn) } else { Value::Null },
            "root_v0_ns_per_iter": pooled("v0"), "aa_ns_per_iter": pooled("aa"),
            "kw_p_across_isogenous": p_iso, "kw_p_across_same_curve_arms": p_null}));
    }
    let med = |v: &mut Vec<f64>| { v.sort_by(|a, b| a.partial_cmp(b).unwrap()); if v.is_empty() { f64::NAN } else { v[v.len() / 2] } };
    let (d1, p1) = ks_uniform(&kw_iso);
    let (d2, p2) = ks_uniform(&kw_null);
    let report = json!({"schema": "fp31.phase4a.timing/v1",
        "caveat": "phase-2 isoclass-rho wall time, isolation L0, 4-thread rayon pool, shared host; descriptive only",
        "classes": rows.len(),
        "median_max_over_min_isogenous": med(&mut all_ratio.clone()), "largest_max_over_min_isogenous": all_ratio.iter().cloned().fold(0.0, f64::max),
        "median_max_over_min_same_curve_arms": med(&mut all_null_ratio.clone()), "largest_max_over_min_same_curve_arms": all_null_ratio.iter().cloned().fold(0.0, f64::max),
        "ks_uniform_kw_p_isogenous": {"classes": kw_iso.len(), "D": d1, "p": p1, "min_p": kw_iso.iter().cloned().fold(1.0, f64::min)},
        "ks_uniform_kw_p_same_curve_arms": {"classes": kw_null.len(), "D": d2, "p": p2, "min_p": kw_null.iter().cloned().fold(1.0, f64::min)},
        "per_class": rows});
    let mut f = OpenOptions::new().write(true).create_new(true).open(&out).expect("output exists");
    f.write_all(serde_json::to_string_pretty(&report).unwrap().as_bytes()).unwrap();
}

// ---------------------------------------------------------------- step B
// `rho-spec`: one ecbench.spec/v1 per class, every curve of the class (walk
// curves + isomorphic models, from phase3/classes.json) with the SAME planted
// scalar list (target_kind planted_shared), arms rho.plain, an identical
// rho.plain A/A arm and a positive-control arm (a second implementation).
// `rho-timing`: reads every class's records.jsonl and computes the frozen
// tests of phase4/PROTOCOL.md on ns per counted group operation.

pub fn cmd_rho_spec(args: &[String]) {
    let classes = arg(args, "--classes").expect("--classes");
    let cid = arg(args, "--class").expect("--class");
    let out = arg(args, "--out").expect("--out");
    let targets: u64 = arg(args, "--targets").expect("--targets").parse().unwrap();
    let rounds: u64 = arg(args, "--rounds").expect("--rounds").parse().unwrap();
    let seed: u64 = arg(args, "--seed").expect("--seed").parse().unwrap();
    let control = arg(args, "--control").expect("--control method id");
    let level = arg(args, "--isolation").unwrap_or("L2".into());
    let doc: Value = serde_json::from_str(&fs::read_to_string(&classes).unwrap()).unwrap();
    let c = doc["classes"].as_array().unwrap().iter().find(|c| c["id"] == cid.as_str()).expect("class id");
    let (p, n) = (c["p"].as_u64().unwrap(), c["n"].as_u64().unwrap());
    let mk = |cv: &Value| json!({"kind": "prime_explicit", "p": p, "a": cv["a"], "b": cv["b"], "group_order": n, "r": n, "gx": cv["gx"], "gy": cv["gy"]});
    let mut curves: Vec<Value> = c["curves"].as_array().unwrap().iter().map(mk).collect();
    curves.extend(c["iso_models"].as_array().unwrap().iter().map(mk));
    let arm = |name: &str, role: &str, id: &str| json!({"name": name, "role": role, "method": {"id": id, "params": {}}});
    let spec = json!({
        "schema": "ecbench.spec/v1",
        "label": format!("fp31 phase 4 {cid}: rho ns per group operation on every isogenous curve, same planted k"),
        "description": format!("research/fp31_whole_field_census_20261111/phase4: class {cid} (p={p}, n={n}); {} walk curves and {} u-scaled isomorphic models; targets planted_shared (same k on every curve, Q transported by the isogeny); arms rho.plain, rho.plain A/A, {control} (positive control).",
            c["curves"].as_array().unwrap().len(), c["iso_models"].as_array().unwrap().len()),
        "workloads": {"curves": curves, "targets_per_curve": targets, "target_seed": seed, "target_kind": "planted_shared"},
        "arms": [arm("plain", "reference", "rho.plain"), arm("plain-aa", "control", "rho.plain"), arm("control", "candidate", &control)],
        "measurement": {"rounds": rounds, "warmup": 1, "order": "shuffle", "seed": seed % 1000 + 7, "isolation_required": level, "timeout_seconds": 3600},
    });
    let mut f = OpenOptions::new().write(true).create_new(true).open(&out).expect("output exists");
    f.write_all(serde_json::to_string_pretty(&spec).unwrap().as_bytes()).unwrap();
    eprintln!("wrote {out}: class {cid}, {} curves x {targets} targets x {rounds} rounds x 3 arms", spec["workloads"]["curves"].as_array().unwrap().len());
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

pub fn cmd_rho_timing(args: &[String]) {
    let classes = arg(args, "--classes").expect("--classes");
    let runs = arg(args, "--runs").expect("--runs");
    let out = arg(args, "--out").expect("--out");
    let doc: Value = serde_json::from_str(&fs::read_to_string(&classes).unwrap()).unwrap();
    let (mut rows, mut fp, mut ratios, mut aa_ratios, mut missing) = (vec![], vec![], vec![], vec![], vec![]);
    let (mut ctl_log, mut aa_log, mut wrong, mut not_l2, mut total) = (vec![], vec![], 0usize, 0usize, 0usize);
    let (mut vc_between, mut vc_samp): (Vec<f64>, Vec<f64>) = (vec![], vec![]);
    let geo = |v: &[f64]| (v.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64).exp();
    for c in doc["classes"].as_array().unwrap() {
        let id = c["id"].as_str().unwrap();
        let text = match fs::read_to_string(format!("{runs}/{id}/session/records.jsonl")) { Ok(t) => t, Err(_) => { missing.push(id.to_string()); continue; } };
        // (arm, curve key (a,b), target index, round) -> ns per GAE
        let mut cell: BTreeMap<(String, (u64, u64), u64, u64), f64> = BTreeMap::new();
        for line in text.lines() {
            let r: Value = serde_json::from_str(line).unwrap();
            if r["warmup"].as_bool() == Some(true) { continue; }
            total += 1;
            if r["outcome"]["matches_planted"].as_bool() != Some(true) && r["outcome"]["status"].as_str() != Some("solved") { wrong += 1; }
            if r["isolation"]["level"].as_str() != Some("L2") { not_l2 += 1; }
            let cs = &r["workload"]["curve_spec"];
            let key = (cs["a"].as_u64().unwrap(), cs["b"].as_u64().unwrap());
            let (wall, gae) = (r["online"]["wall_ns"].as_f64().unwrap(), r["cost"]["total_gae"].as_f64().unwrap());
            cell.insert((r["arm"].as_str().unwrap().to_string(), key, r["workload"]["target_index"].as_u64().unwrap(), r["round"].as_u64().unwrap()), wall / gae);
        }
        let curves: Vec<(u64, u64)> = { let mut v: Vec<_> = cell.keys().map(|k| k.1).collect(); v.sort(); v.dedup(); v };
        let blocks: Vec<(u64, u64)> = { let mut v: Vec<_> = cell.keys().filter(|k| k.0 == "plain").map(|k| (k.2, k.3)).collect(); v.sort(); v.dedup(); v };
        // Friedman across curves on the main arm, blocks = (target, round); only complete blocks
        let m: Vec<Vec<f64>> = blocks.iter().filter_map(|b| curves.iter().map(|cv| cell.get(&("plain".into(), *cv, b.0, b.1)).cloned()).collect::<Option<Vec<f64>>>()).collect();
        let (fq, fdf, fpv) = if m.len() >= 2 && curves.len() >= 2 { friedman(&m) } else { (f64::NAN, 0, f64::NAN) };
        if fpv.is_finite() { fp.push(fpv); }
        let per_curve: Vec<(String, f64, f64)> = curves.iter().map(|cv| {
            let v: Vec<f64> = cell.iter().filter(|(k, _)| k.0 == "plain" && k.1 == *cv).map(|(_, x)| *x).collect();
            let a: Vec<f64> = cell.iter().filter(|(k, _)| k.0 == "plain-aa" && k.1 == *cv).map(|(_, x)| *x).collect();
            (format!("a={},b={}", cv.0, cv.1), geo(&v), geo(&a))
        }).collect();
        let mn = per_curve.iter().cloned().fold(("".to_string(), f64::INFINITY, 0.0), |b, x| if x.1 < b.1 { x } else { b });
        let mx = per_curve.iter().cloned().fold(("".to_string(), 0.0, 0.0), |b, x| if x.1 > b.1 { x } else { b });
        ratios.push(mx.1 / mn.1);
        // A/A: per-curve plain-aa / plain geo-mean ratios; spread of the same statistic under identical arms
        let aa: Vec<f64> = per_curve.iter().map(|x| x.2 / x.1).collect();
        aa_ratios.push(aa.iter().cloned().fold(0.0, f64::max) / aa.iter().cloned().fold(f64::INFINITY, f64::min));
        for (k, x) in &cell {
            if k.0 == "control" { if let Some(b) = cell.get(&("plain".into(), k.1, k.2, k.3)) { ctl_log.push((x / b).ln()); } }
            if k.0 == "plain-aa" { if let Some(b) = cell.get(&("plain".into(), k.1, k.2, k.3)) { aa_log.push((x / b).ln()); } }
        }
        // variance components on log ns/GAE (main arm): between-curve variance of the true
        // per-curve mean = variance of curve means - mean sampling variance of a curve mean
        let logs: Vec<Vec<f64>> = curves.iter().map(|cv| cell.iter().filter(|(k, _)| k.0 == "plain" && k.1 == *cv).map(|(_, x)| x.ln()).collect()).collect();
        let means: Vec<f64> = logs.iter().map(|v| v.iter().sum::<f64>() / v.len() as f64).collect();
        let gm = means.iter().sum::<f64>() / means.len() as f64;
        let var_means = means.iter().map(|x| (x - gm).powi(2)).sum::<f64>() / (means.len() as f64 - 1.0);
        let samp = logs.iter().zip(&means).map(|(v, mu)| v.iter().map(|x| (x - mu).powi(2)).sum::<f64>() / (v.len() as f64 - 1.0) / v.len() as f64).sum::<f64>() / logs.len() as f64;
        vc_between.push(var_means - samp);
        vc_samp.push(samp);
        rows.push(json!({"id": id, "stratum": c["stratum"], "curves": curves.len(), "complete_blocks": m.len(),
            "log_var_curve_means": var_means, "log_sampling_var_of_mean": samp, "log_between_curve_var_estimate": var_means - samp,
            "friedman": {"Q": fq, "df": fdf, "p": fpv},
            "fastest": {"curve": mn.0, "ns_per_gae": mn.1}, "slowest": {"curve": mx.0, "ns_per_gae": mx.1}, "max_over_min": mx.1 / mn.1,
            "aa_max_over_min_of_curve_ratios": aa_ratios.last()}));
    }
    let mean = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
    let se = |v: &[f64]| { let m = mean(v); (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (v.len() as f64 - 1.0) / v.len() as f64).sqrt() };
    let (d, ksp) = ks_uniform(&fp);
    let med = |v: &Vec<f64>| { let mut s = v.clone(); s.sort_by(|a, b| a.partial_cmp(b).unwrap()); s[s.len() / 2] };
    let report = json!({"schema": "fp31.phase4b.rho-timing/v1", "classes": rows.len(), "missing": missing,
        "records_measured": total, "records_not_solved_correctly": wrong, "records_not_L2": not_l2,
        "primary": {"test": "T1 within-class Friedman p-values (curves x (target, round) blocks, main arm) ~ U(0,1)", "classes": fp.len(), "D": d, "p": ksp,
                    "min_p": fp.iter().cloned().fold(1.0, f64::min), "classes_p_below_0.05_over_classes": fp.iter().filter(|&&p| p < 0.05 / fp.len() as f64).count()},
        "spread": {"median_max_over_min_curves": med(&ratios), "largest_max_over_min_curves": ratios.iter().cloned().fold(0.0, f64::max),
                   "median_aa_spread": med(&aa_ratios)},
        "between_curve_sd": {"note": "pooled over classes: sqrt(max(0, mean of per-class between-curve variance estimates)) on log ns/GAE, with a one-sided 95% upper bound from the class-to-class spread of the estimates",
            "estimate": mean(&vc_between).max(0.0).sqrt(), "upper95": (mean(&vc_between) + 1.645 * se(&vc_between)).max(0.0).sqrt(),
            "sampling_sd_of_curve_mean": mean(&vc_samp).sqrt()},
        "controls": {"aa_log_ratio_mean": mean(&aa_log), "aa_log_ratio_se": se(&aa_log), "aa_n": aa_log.len(),
                     "positive_control_log_ratio_mean": mean(&ctl_log), "positive_control_log_ratio_se": se(&ctl_log), "positive_control_n": ctl_log.len(),
                     "positive_control_ratio": mean(&ctl_log).exp()},
        "per_class": rows});
    let mut f = OpenOptions::new().write(true).create_new(true).open(&out).expect("output exists");
    f.write_all(serde_json::to_string_pretty(&report).unwrap().as_bytes()).unwrap();
}
