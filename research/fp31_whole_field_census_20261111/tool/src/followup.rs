//! Phase 5 (randomised-order follow-up of phase 4, and IC timing): `p5-spec` writes
//! one ecbench.spec/v1 per (class, kind, session) with the class's curves (walk curves
//! and isomorphic models) in a seeded random order and fresh planted_shared targets;
//! `p5-analyze` computes the frozen tests of phase5/PROTOCOL.md. ecbench runs workloads
//! in spec order within a session, so the order is randomised across sessions instead.

use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::Write;

use serde_json::{json, Value};

use crate::stats::{gamma_q, ks_uniform, SplitMix};

fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).map(|i| args[i + 1].clone())
}

/// p2tool's fixed-|F| rule (research/isogeny_class_ic_yield_20261102/p2_prime_analogues/p2tool/src/spec.rs),
/// reproduced exactly: factor base size, predicted yield, planted-control size, attempts per run.
fn ic_params(n: u64, m: u32) -> (u64, u64, u64) {
    let (y, table_cap) = (if m == 2 { 0.02 } else { 0.1 }, 4194304.0f64);
    let nf = n as f64;
    let ny = if m == 2 { (y * nf / 2.0).sqrt().ceil() } else { (y * nf / 4.0).cbrt().ceil() };
    let nfb = ny.min((table_cap / 2.0).sqrt().floor()) as u64;
    let f = nfb as f64;
    let y_eff = (if m == 2 { 2.0 * f * f / nf } else { 4.0 / 3.0 * f * f * f / nf }).min(1.0);
    let rel = 200f64.min(0.6 * f);
    let attempts = (rel / y_eff).ceil() as u64;
    let pc = (f * 1.2f64.powf(1.0 / m as f64)).round() as u64;
    (nfb, pc, attempts)
}

pub fn cmd_p5_spec(args: &[String]) {
    let classes = arg(args, "--classes").expect("--classes");
    let cid = arg(args, "--class").expect("--class");
    let kind = arg(args, "--kind").expect("--kind rho|ic");
    let session: u64 = arg(args, "--session").expect("--session").parse().unwrap();
    let targets: u64 = arg(args, "--targets").expect("--targets").parse().unwrap();
    let seed: u64 = arg(args, "--seed").expect("--seed").parse().unwrap();
    let warmup: u64 = arg(args, "--warmup").map(|s| s.parse().unwrap()).unwrap_or(1);
    let out = arg(args, "--out").expect("--out");
    let doc: Value = serde_json::from_str(&fs::read_to_string(&classes).unwrap()).unwrap();
    let c = doc["classes"].as_array().unwrap().iter().find(|c| c["id"] == cid.as_str()).expect("class id");
    let (p, n) = (c["p"].as_u64().unwrap(), c["n"].as_u64().unwrap());
    let mk = |cv: &Value| json!({"kind": "prime_explicit", "p": p, "a": cv["a"], "b": cv["b"], "group_order": n, "r": n, "gx": cv["gx"], "gy": cv["gy"]});
    let mut curves: Vec<Value> = c["curves"].as_array().unwrap().iter().map(mk).collect();
    curves.extend(c["iso_models"].as_array().unwrap().iter().map(mk));
    // seeded Fisher-Yates over the curve list, one permutation per (class, kind, session)
    let cnum: u64 = cid.bytes().fold(0u64, |h, b| h.wrapping_mul(131).wrapping_add(b as u64));
    let ktag: u64 = match kind.as_str() { "rho" => 1, "ic3" => 3, "ic2" => 2, _ => panic!("--kind rho|ic3|ic2") };
    let mut rng = SplitMix(seed ^ (cnum << 32) ^ (ktag << 24) ^ session);
    for i in (1..curves.len()).rev() { let j = rng.below(i as u64 + 1) as usize; curves.swap(i, j); }
    let arms = if kind == "rho" {
        let a = |name: &str, role: &str, id: &str| json!({"name": name, "role": role, "method": {"id": id, "params": {}}});
        vec![a("plain", "reference", "rho.plain"), a("plain-aa", "control", "rho.plain"), a("control", "candidate", "rho.negation")]
    } else {
        let m: u32 = if kind == "ic3" { 3 } else { 2 };
        let (nfb, pc, attempts) = ic_params(n, m);
        let a = |name: &str, role: &str, size: u64| json!({"name": name, "role": role, "method": {"id": "ic.pipeline", "params": {
            "factor_base": format!("prime-abscissa:size={size}"), "oracle": format!("mitm:m={m}"),
            "max_trials": attempts.to_string(), "linalg": "incremental-gauss-full-rank"}}});
        vec![a("main", "reference", nfb), a("main-aa", "control", nfb), a("pc", "candidate", pc)]
    };
    let spec = json!({
        "schema": "ecbench.spec/v1",
        "label": format!("fp31 phase 5 {kind} {cid} session {session}: per-curve cost, randomised curve order, same planted k"),
        "description": format!("research/fp31_whole_field_census_20261111/phase5: class {cid} (p={p}, n={n}), kind {kind}, session {session}; curves (walk curves and u-scaled isomorphic models) in a seeded random order; targets planted_shared with a fresh seed per session."),
        "workloads": {"curves": curves, "targets_per_curve": targets, "target_seed": seed + 1000 * ktag + session, "target_kind": "planted_shared"},
        "arms": arms,
        "measurement": {"rounds": 1, "warmup": warmup, "order": "shuffle", "seed": (seed + session) % 1000 + 11, "isolation_required": "L2", "timeout_seconds": 7200},
    });
    let mut f = OpenOptions::new().write(true).create_new(true).open(&out).expect("output exists");
    f.write_all(serde_json::to_string_pretty(&spec).unwrap().as_bytes()).unwrap();
}

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
    (q, k - 1, gamma_q((k - 1) as f64 / 2.0, q / 2.0))
}

/// Metric per record: rho -> wall ns per GAE; ic -> PDP ns per attempt and wall ns per relation.
pub fn cmd_p5_analyze(args: &[String]) {
    let classes = arg(args, "--classes").expect("--classes");
    let runs = arg(args, "--runs").expect("--runs");
    let kind = arg(args, "--kind").expect("--kind rho|ic3|ic2");
    let sessions: u64 = arg(args, "--sessions").expect("--sessions").parse().unwrap();
    let out = arg(args, "--out").expect("--out");
    let (main, aa, ctl) = if kind == "rho" { ("plain", "plain-aa", "control") } else { ("main", "main-aa", "pc") };
    let doc: Value = serde_json::from_str(&fs::read_to_string(&classes).unwrap()).unwrap();
    let mean = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
    let se = |v: &[f64]| { let m = v.iter().sum::<f64>() / v.len() as f64; (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (v.len() as f64 - 1.0) / v.len() as f64).sqrt() };
    let (mut rows, mut fp, mut vcb, mut ratios, mut missing) = (vec![], vec![], vec![], vec![], vec![]);
    let (mut aa_l, mut ctl_l, mut ctl_rel, mut iso_d, mut other_d) = (vec![], vec![], vec![], vec![], vec![]);
    let (mut total, mut not_l2, mut wrong) = (0usize, 0usize, 0usize);
    for c in doc["classes"].as_array().unwrap() {
        let id = c["id"].as_str().unwrap();
        // (arm, curve, session, target) -> (primary metric, ns per relation)
        let mut cell: BTreeMap<(String, (u64, u64), u64, u64), (f64, f64)> = BTreeMap::new();
        let mut got = 0;
        for s in 0..sessions {
            let text = match fs::read_to_string(format!("{runs}/{id}/{kind}_s{s}/session/records.jsonl")) { Ok(t) => t, Err(_) => continue };
            got += 1;
            for line in text.lines() {
                let r: Value = serde_json::from_str(line).unwrap();
                if r["warmup"].as_bool() == Some(true) { continue; }
                total += 1;
                if r["isolation"]["level"].as_str() != Some("L2") { not_l2 += 1; }
                let o = &r["outcome"];
                if kind == "rho" && o["status"].as_str() != Some("verified") { wrong += 1; }
                let cs = &r["workload"]["curve_spec"];
                let key = (cs["a"].as_u64().unwrap(), cs["b"].as_u64().unwrap());
                let v = if kind == "rho" {
                    (r["online"]["wall_ns"].as_f64().unwrap() / r["cost"]["total_gae"].as_f64().unwrap(), f64::NAN)
                } else {
                    let tried = r["counters"]["targets_tried"].as_f64().unwrap();
                    let rel = r["counters"]["relations_found"].as_f64().unwrap();
                    (r["online"]["phases_ns"]["target_PDP"].as_f64().unwrap() / tried,
                     if rel > 0.0 { r["online"]["wall_ns"].as_f64().unwrap() / rel } else { f64::NAN })
                };
                cell.insert((r["arm"].as_str().unwrap().to_string(), key, s, r["workload"]["target_index"].as_u64().unwrap()), v);
            }
        }
        if got < sessions { missing.push(json!({"class": id, "sessions_found": got})); }
        if cell.is_empty() { continue; }
        let curves: Vec<(u64, u64)> = { let mut v: Vec<_> = cell.keys().map(|k| k.1).collect(); v.sort(); v.dedup(); v };
        let blocks: Vec<(u64, u64)> = { let mut v: Vec<_> = cell.keys().map(|k| (k.2, k.3)).collect(); v.sort(); v.dedup(); v };
        let mat: Vec<Vec<f64>> = blocks.iter().filter_map(|b| curves.iter().map(|cv| cell.get(&(main.into(), *cv, b.0, b.1)).map(|x| x.0)).collect::<Option<Vec<f64>>>()).collect();
        let (fq, fdf, fpv) = if mat.len() >= 2 && curves.len() >= 2 { friedman(&mat) } else { (f64::NAN, 0, f64::NAN) };
        if fpv.is_finite() { fp.push(fpv); }
        let lm: BTreeMap<(u64, u64), (f64, Vec<f64>)> = curves.iter().map(|cv| {
            let v: Vec<f64> = cell.iter().filter(|(k, _)| k.0 == main && k.1 == *cv).map(|(_, x)| x.0.ln()).collect();
            (*cv, (mean(&v), v))
        }).collect();
        let means: Vec<f64> = lm.values().map(|x| x.0).collect();
        let gm = mean(&means);
        if means.len() >= 2 {
            let vm = means.iter().map(|x| (x - gm).powi(2)).sum::<f64>() / (means.len() as f64 - 1.0);
            let samp = mean(&lm.values().map(|(mu, v)| v.iter().map(|x| (x - mu).powi(2)).sum::<f64>() / (v.len() as f64 - 1.0) / v.len() as f64).collect::<Vec<_>>());
            vcb.push(vm - samp);
            let (mn, mx) = means.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |b, &x| (b.0.min(x), b.1.max(x)));
            ratios.push((mx - mn).exp());
        }
        // isomorphic model vs its base curve, against all other pairs (secondary)
        let cur = c["curves"].as_array().unwrap();
        let mut iso_pairs = vec![];
        for im in c["iso_models"].as_array().unwrap() {
            let base = &cur[im["of"].as_u64().unwrap() as usize];
            let (kb, ki) = ((base["a"].as_u64().unwrap(), base["b"].as_u64().unwrap()), (im["a"].as_u64().unwrap(), im["b"].as_u64().unwrap()));
            if let (Some(x), Some(y)) = (lm.get(&kb), lm.get(&ki)) { iso_d.push((x.0 - y.0).abs()); iso_pairs.push((kb, ki)); }
        }
        let keys: Vec<(u64, u64)> = lm.keys().cloned().collect();
        for i in 0..keys.len() { for j in i + 1..keys.len() {
            if iso_pairs.iter().any(|&(a, b)| (a, b) == (keys[i], keys[j]) || (b, a) == (keys[i], keys[j])) { continue; }
            other_d.push((lm[&keys[i]].0 - lm[&keys[j]].0).abs());
        } }
        for (k, x) in &cell {
            if let Some(b) = cell.get(&(main.into(), k.1, k.2, k.3)) {
                if k.0 == aa { aa_l.push((x.0 / b.0).ln()); }
                if k.0 == ctl { ctl_l.push((x.0 / b.0).ln()); if x.1.is_finite() && b.1.is_finite() { ctl_rel.push((x.1 / b.1).ln()); } }
            }
        }
        rows.push(json!({"id": id, "stratum": c["stratum"], "curves": curves.len(), "complete_blocks": mat.len(),
            "friedman": {"Q": fq, "df": fdf, "p": fpv}, "max_over_min_curve_mean": ratios.last()}));
    }
    let (d, ksp) = ks_uniform(&fp);
    let mut smaller = 0.0;
    for a in &iso_d { for b in &other_d { if a < b { smaller += 1.0 } else if a == b { smaller += 0.5 } } }
    let mut sr = ratios.clone(); sr.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let metric = if kind == "rho" { "wall ns per GAE" } else { "PDP ns per decomposition attempt" };
    let med = |v: &Vec<f64>| { let mut s = v.clone(); s.sort_by(|a, b| a.partial_cmp(b).unwrap()); s.get(s.len() / 2).cloned() };
    let (iso_med, oth_med) = (med(&iso_d), med(&other_d));
    let prob_closer = smaller / (iso_d.len() * other_d.len()).max(1) as f64;
    let sr_med = sr.get(sr.len() / 2).cloned();
    let sr_max = sr.last().cloned();
    let n_low = fp.iter().filter(|&&p| p < 0.05 / fp.len() as f64).count();
    let min_p = fp.iter().cloned().fold(1.0, f64::min);
    let bsd = (mean(&vcb).max(0.0).sqrt(), (mean(&vcb) + 1.645 * se(&vcb)).max(0.0).sqrt());
    let ctl_rel_v: Value = if ctl_rel.is_empty() { Value::Null } else { json!(mean(&ctl_rel).exp()) };
    let (aam, aas, ctm, cts) = (mean(&aa_l), se(&aa_l), mean(&ctl_l), se(&ctl_l));
    let report = json!({"schema": "fp31.phase5.analysis/v1", "kind": kind, "metric": metric,
        "classes": rows.len(), "missing": missing, "records_measured": total, "records_not_L2": not_l2, "rho_records_not_verified": wrong,
        "T1_friedman_ks_uniform": {"classes": fp.len(), "D": d, "p": ksp, "min_p": min_p},
        "T2_classes_p_below_0.05_over_classes": n_low,
        "between_curve_sd_log": {"estimate": bsd.0, "upper95": bsd.1},
        "max_over_min_curve_mean": {"median": sr_med, "max": sr_max},
        "S1_iso_vs_base": {"iso_pairs": iso_d.len(), "median_abs_log_diff_iso": iso_med, "median_abs_log_diff_other": oth_med, "prob_iso_pair_closer": prob_closer},
        "controls": {"aa_log_ratio_mean": aam, "aa_log_ratio_se": aas, "aa_n": aa_l.len(),
                     "positive_control_log_ratio_mean": ctm, "positive_control_log_ratio_se": cts, "positive_control_ratio": ctm.exp(),
                     "positive_control_ns_per_relation_ratio": ctl_rel_v},
        "per_class": rows});
    let mut f = OpenOptions::new().write(true).create_new(true).open(&out).expect("output exists");
    f.write_all(serde_json::to_string_pretty(&report).unwrap().as_bytes()).unwrap();
}
