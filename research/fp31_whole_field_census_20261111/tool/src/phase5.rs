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

// ------------------------------------------- phase 4 consistency diagnostic
// `rho-timing-diag` (exploratory, after phase 4's frozen analysis): does a curve that
// is cheap per GAE in the `plain` arm also come out cheap in the independent `plain-aa`
// runs and in the `control` (rho.negation) runs on the same curves and targets?
// Per class: Spearman across curves of per-curve geometric means; pooled over classes:
// the mean rho, a sign test, and a permutation reference (curve labels shuffled
// within class, seeded).
pub fn cmd_rho_timing_diag(args: &[String]) {
    use crate::stats::{spearman_perm, SplitMix};
    let classes = arg(args, "--classes").expect("--classes");
    let runs = arg(args, "--runs").expect("--runs");
    let out = arg(args, "--out").expect("--out");
    let doc: Value = serde_json::from_str(&fs::read_to_string(&classes).unwrap()).unwrap();
    let geo = |v: &[f64]| (v.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64).exp();
    let (mut rows, mut r_aa, mut r_ctl) = (vec![], vec![], vec![]);
    for c in doc["classes"].as_array().unwrap() {
        let id = c["id"].as_str().unwrap();
        let text = match fs::read_to_string(format!("{runs}/{id}/session/records.jsonl")) { Ok(t) => t, Err(_) => continue };
        let mut by: BTreeMap<(String, (u64, u64)), Vec<f64>> = BTreeMap::new();
        for line in text.lines() {
            let r: Value = serde_json::from_str(line).unwrap();
            if r["warmup"].as_bool() == Some(true) { continue; }
            let cs = &r["workload"]["curve_spec"];
            let v = r["online"]["wall_ns"].as_f64().unwrap() / r["cost"]["total_gae"].as_f64().unwrap();
            by.entry((r["arm"].as_str().unwrap().to_string(), (cs["a"].as_u64().unwrap(), cs["b"].as_u64().unwrap()))).or_default().push(v);
        }
        let curves: Vec<(u64, u64)> = { let mut v: Vec<_> = by.keys().map(|k| k.1).collect(); v.sort(); v.dedup(); v };
        if curves.len() < 3 { continue; }
        let g = |arm: &str| -> Vec<f64> { curves.iter().map(|cv| geo(&by[&(arm.to_string(), *cv)])).collect() };
        let (p, a, ctl) = (g("plain"), g("plain-aa"), g("control"));
        let (ra, pa) = spearman_perm(&p, &a, 2000, 0xD1A6 + rows.len() as u64);
        let (rc, pc) = spearman_perm(&p, &ctl, 2000, 0xD1A7 + rows.len() as u64);
        r_aa.push(ra); r_ctl.push(rc);
        rows.push(json!({"id": id, "curves": curves.len(), "spearman_plain_vs_aa": ra, "p_perm_aa": pa, "spearman_plain_vs_negation": rc, "p_perm_negation": pc}));
    }
    let mean = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
    // sign test: P(at least k positive of n) under p = 1/2
    let sign = |v: &[f64]| {
        let n = v.len(); let k = v.iter().filter(|&&x| x > 0.0).count();
        let mut tail = 0.0f64; let mut comb = 1.0f64;
        for i in 0..=n { if i > 0 { comb = comb * (n - i + 1) as f64 / i as f64; } if i >= k { tail += comb; } }
        (k, n, tail / 2f64.powi(n as i32))
    };
    let (ka, na, psa) = sign(&r_aa);
    let (kc, nc, psc) = sign(&r_ctl);
    let _ = SplitMix(0);
    let report = json!({"schema": "fp31.phase4.diag/v1",
        "note": "exploratory diagnostic run after the frozen phase-4 analysis; not a preregistered test",
        "plain_vs_aa": {"mean_spearman": mean(&r_aa), "positive": ka, "classes": na, "sign_test_p_one_sided": psa},
        "plain_vs_negation": {"mean_spearman": mean(&r_ctl), "positive": kc, "classes": nc, "sign_test_p_one_sided": psc},
        "per_class": rows});
    let mut f = OpenOptions::new().write(true).create_new(true).open(&out).expect("output exists");
    f.write_all(serde_json::to_string_pretty(&report).unwrap().as_bytes()).unwrap();
}

// `rho-timing-diag2` (exploratory): (1) does an isomorphic model (same group, other
// a, b) time like its base curve? Compare |log(iso/base)| with |log(x/y)| over all
// other curve pairs of the class. (2) pooled Spearman of class-normalised per-curve
// log ns/GAE against popcount and bit length of a and of b.
pub fn cmd_rho_timing_diag2(args: &[String]) {
    use crate::stats::spearman_perm;
    let classes = arg(args, "--classes").expect("--classes");
    let runs = arg(args, "--runs").expect("--runs");
    let out = arg(args, "--out").expect("--out");
    let doc: Value = serde_json::from_str(&fs::read_to_string(&classes).unwrap()).unwrap();
    let (mut iso_d, mut other_d) = (vec![], vec![]);
    let (mut feat_y, mut f_pa, mut f_pb, mut f_la, mut f_lb) = (vec![], vec![], vec![], vec![], vec![]);
    for c in doc["classes"].as_array().unwrap() {
        let id = c["id"].as_str().unwrap();
        let text = match fs::read_to_string(format!("{runs}/{id}/session/records.jsonl")) { Ok(t) => t, Err(_) => continue };
        let mut by: BTreeMap<(u64, u64), Vec<f64>> = BTreeMap::new();
        for line in text.lines() {
            let r: Value = serde_json::from_str(line).unwrap();
            if r["warmup"].as_bool() == Some(true) || r["arm"].as_str() != Some("plain") { continue; }
            let cs = &r["workload"]["curve_spec"];
            by.entry((cs["a"].as_u64().unwrap(), cs["b"].as_u64().unwrap())).or_default()
                .push((r["online"]["wall_ns"].as_f64().unwrap() / r["cost"]["total_gae"].as_f64().unwrap()).ln());
        }
        let lm: BTreeMap<(u64, u64), f64> = by.iter().map(|(k, v)| (*k, v.iter().sum::<f64>() / v.len() as f64)).collect();
        let cm = lm.values().sum::<f64>() / lm.len() as f64;
        for (k, v) in &lm {
            feat_y.push(v - cm);
            f_pa.push(k.0.count_ones() as f64); f_pb.push(k.1.count_ones() as f64);
            f_la.push((64 - k.0.leading_zeros()) as f64); f_lb.push((64 - k.1.leading_zeros()) as f64);
        }
        let curves = c["curves"].as_array().unwrap();
        let mut iso_pairs = vec![];
        for im in c["iso_models"].as_array().unwrap() {
            let base = &curves[im["of"].as_u64().unwrap() as usize];
            let (kb, ki) = ((base["a"].as_u64().unwrap(), base["b"].as_u64().unwrap()), (im["a"].as_u64().unwrap(), im["b"].as_u64().unwrap()));
            if let (Some(x), Some(y)) = (lm.get(&kb), lm.get(&ki)) { iso_d.push((x - y).abs()); iso_pairs.push((kb, ki)); }
        }
        let keys: Vec<(u64, u64)> = lm.keys().cloned().collect();
        for i in 0..keys.len() { for j in i + 1..keys.len() {
            let pr = (keys[i], keys[j]);
            if iso_pairs.iter().any(|&(a, b)| (a, b) == pr || (b, a) == pr) { continue; }
            other_d.push((lm[&keys[i]] - lm[&keys[j]]).abs());
        } }
    }
    let mean = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
    let med = |v: &[f64]| { let mut s = v.to_vec(); s.sort_by(|a, b| a.partial_cmp(b).unwrap()); s[s.len() / 2] };
    // Mann-Whitney style: fraction of (iso, other) pairs with iso difference smaller
    let mut smaller = 0.0;
    for a in &iso_d { for b in &other_d { if a < b { smaller += 1.0 } else if a == b { smaller += 0.5 } } }
    let auc = smaller / (iso_d.len() * other_d.len()) as f64;
    let feats = [("popcount_a", &f_pa), ("popcount_b", &f_pb), ("bitlen_a", &f_la), ("bitlen_b", &f_lb)];
    let fr: Vec<Value> = feats.iter().enumerate().map(|(i, (n, x))| { let (rho, p) = spearman_perm(x, &feat_y, 5000, 0xFEA7 + i as u64); json!({"feature": n, "spearman": rho, "p_perm": p, "n": x.len()}) }).collect();
    let report = json!({"schema": "fp31.phase4.diag2/v1", "note": "exploratory diagnostic after the frozen phase-4 analysis; not preregistered",
        "iso_vs_base_abs_log_diff": {"n": iso_d.len(), "mean": mean(&iso_d), "median": med(&iso_d)},
        "other_pairs_abs_log_diff": {"n": other_d.len(), "mean": mean(&other_d), "median": med(&other_d)},
        "prob_iso_pair_closer_than_other_pair": auc,
        "features": fr});
    let mut f = OpenOptions::new().write(true).create_new(true).open(&out).expect("output exists");
    f.write_all(serde_json::to_string_pretty(&report).unwrap().as_bytes()).unwrap();
}

// `rho-timing-diag3` (exploratory): execution-order confound. ecbench runs workloads
// in spec order within a round (only the arms are shuffled), so each curve occupies
// one time window. Per class: Spearman of per-curve mean log ns/GAE (plain arm)
// against the curve's first measured seq (its time position); pooled: mean rho, sign
// test; plus the pooled per-run trend of log ns/GAE against seq within the session.
pub fn cmd_rho_timing_diag3(args: &[String]) {
    use crate::stats::spearman_perm;
    let classes = arg(args, "--classes").expect("--classes");
    let runs = arg(args, "--runs").expect("--runs");
    let out = arg(args, "--out").expect("--out");
    let doc: Value = serde_json::from_str(&fs::read_to_string(&classes).unwrap()).unwrap();
    let (mut rows, mut rhos) = (vec![], vec![]);
    let mut profile: BTreeMap<usize, Vec<f64>> = BTreeMap::new(); // decile of session position -> normalised log ns/GAE
    for c in doc["classes"].as_array().unwrap() {
        let id = c["id"].as_str().unwrap();
        let text = match fs::read_to_string(format!("{runs}/{id}/session/records.jsonl")) { Ok(t) => t, Err(_) => continue };
        let mut pts: Vec<(f64, (u64, u64), f64)> = vec![];
        for line in text.lines() {
            let r: Value = serde_json::from_str(line).unwrap();
            if r["warmup"].as_bool() == Some(true) || r["arm"].as_str() != Some("plain") { continue; }
            let cs = &r["workload"]["curve_spec"];
            pts.push((r["seq"].as_f64().unwrap(), (cs["a"].as_u64().unwrap(), cs["b"].as_u64().unwrap()),
                      (r["online"]["wall_ns"].as_f64().unwrap() / r["cost"]["total_gae"].as_f64().unwrap()).ln()));
        }
        let mu = pts.iter().map(|x| x.2).sum::<f64>() / pts.len() as f64;
        let (smin, smax) = pts.iter().fold((f64::INFINITY, 0.0f64), |b, x| (b.0.min(x.0), b.1.max(x.0)));
        for p in &pts { let d = (((p.0 - smin) / (smax - smin + 1.0)) * 10.0) as usize; profile.entry(d.min(9)).or_default().push(p.2 - mu); }
        let mut by: BTreeMap<(u64, u64), (f64, f64, usize)> = BTreeMap::new();
        for p in &pts { let e = by.entry(p.1).or_insert((f64::INFINITY, 0.0, 0)); e.0 = e.0.min(p.0); e.1 += p.2; e.2 += 1; }
        if by.len() < 3 { continue; }
        let first: Vec<f64> = by.values().map(|v| v.0).collect();
        let mean: Vec<f64> = by.values().map(|v| v.1 / v.2 as f64).collect();
        let (rho, p) = spearman_perm(&first, &mean, 2000, 0x0D3 + rows.len() as u64);
        rhos.push(rho);
        rows.push(json!({"id": id, "curves": by.len(), "spearman_position_vs_ns_per_gae": rho, "p_perm": p}));
    }
    let n = rhos.len(); let k = rhos.iter().filter(|&&x| x > 0.0).count();
    let prof: Vec<Value> = profile.iter().map(|(d, v)| json!({"session_decile": d, "mean_log_ns_per_gae_minus_class_mean": v.iter().sum::<f64>() / v.len() as f64, "n": v.len()})).collect();
    let report = json!({"schema": "fp31.phase4.diag3/v1", "note": "exploratory; execution-order confound check after the frozen phase-4 analysis",
        "position_vs_speed": {"mean_spearman": rhos.iter().sum::<f64>() / n as f64, "positive": k, "classes": n},
        "session_profile_by_decile": prof, "per_class": rows});
    let mut f = OpenOptions::new().write(true).create_new(true).open(&out).expect("output exists");
    f.write_all(serde_json::to_string_pretty(&report).unwrap().as_bytes()).unwrap();
}
