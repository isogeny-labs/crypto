//! Phase 3 (index-calculus relation yield across isogeny classes):
//! `ic-classes` writes p2tool's `p2.classes/v1` input from the phase-2 walks;
//! `ic-analyze` computes the frozen cross-class tests of phase3/PROTOCOL.md
//! from p2tool's per-class analysis.json files. Every statistic is computed here
//! or in p2tool (both Rust).

use std::fs::{self, OpenOptions};
use std::io::Write;

use serde_json::{json, Value};

use crate::stats::{gamma_q, kruskal, ks_uniform, spearman_perm, SplitMix};

fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).map(|i| args[i + 1].clone())
}
fn mulm(a: u64, b: u64, p: u64) -> u64 { ((a as u128 * b as u128) % p as u128) as u64 }
fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1 % p;
    a %= p;
    while e > 0 { if e & 1 == 1 { r = mulm(r, a, p); } a = mulm(a, a, p); e >>= 1; }
    r
}
type Pt = Option<(u64, u64)>;
fn add(p: u64, a: u64, s: Pt, t: Pt) -> Pt {
    let (x1, y1) = match s { None => return t, Some(v) => v };
    let (x2, y2) = match t { None => return s, Some(v) => v };
    let lam = if x1 == x2 {
        if (y1 + y2) % p == 0 { return None; }
        mulm((3 * mulm(x1, x1, p) + a) % p, powm(2 * y1 % p, p - 2, p), p)
    } else {
        mulm((y2 + p - y1) % p, powm((x2 + p - x1) % p, p - 2, p), p)
    };
    let x3 = (mulm(lam, lam, p) + 2 * p - x1 - x2) % p;
    Some((x3, (mulm(lam, (x1 + p - x3) % p, p) + p - y1) % p))
}
fn mul(p: u64, a: u64, mut k: u64, s: Pt) -> Pt {
    let (mut r, mut q) = (None, s);
    while k > 0 { if k & 1 == 1 { r = add(p, a, r, q); } q = add(p, a, q, q); k >>= 1; }
    r
}
fn on(p: u64, a: u64, b: u64, pt: Pt) -> bool {
    match pt { None => true, Some((x, y)) => mulm(y, y, p) == (mulm(mulm(x, x, p), x, p) + mulm(a, x, p) + b) % p }
}
fn jinv(p: u64, a: u64, b: u64) -> u64 {
    let a3 = mulm(mulm(a, a, p), a, p);
    let den = (4 * a3 + 27 * mulm(b, b, p)) % p;
    mulm(mulm(1728, 4 * a3 % p, p), powm(den, p - 2, p), p)
}

pub fn cmd_ic_classes(args: &[String]) {
    let manifest = arg(args, "--manifest").expect("--manifest");
    let runs = arg(args, "--runs").expect("--runs");
    let out = arg(args, "--out").expect("--out");
    let n_iso: usize = arg(args, "--iso").map(|s| s.parse().unwrap()).unwrap_or(2);
    let seed: u64 = arg(args, "--seed").expect("--seed").parse().unwrap();
    let man: Value = serde_json::from_str(&fs::read_to_string(&manifest).unwrap()).unwrap();
    let mut classes = vec![];
    for c in man.as_array().unwrap() {
        let st = c["stratum"].as_str().unwrap();
        if !st.starts_with('S') { continue; }
        let id = c["id"].as_str().unwrap();
        let (p, n) = (c["p"].as_u64().unwrap(), c["N"].as_u64().unwrap());
        assert_eq!(n, c["r"].as_u64().unwrap(), "{id}: main classes have prime order");
        let w: Value = serde_json::from_str(&fs::read_to_string(format!("{runs}/{id}/walk/walk.json")).unwrap()).unwrap();
        let mut curves = vec![];
        for v in w["vertices"].as_array().unwrap() {
            if v["measured"].as_bool() != Some(true) { continue; }
            let (a, b, j) = (v["a"].as_u64().unwrap(), v["b"].as_u64().unwrap(), v["j"].as_u64().unwrap());
            let inst = &v["inst"][0];
            let (gx, gy) = (inst[0].as_u64().unwrap(), inst[1].as_u64().unwrap());
            let g = Some((gx, gy));
            assert!(on(p, a, b, g) && mul(p, a, n, g).is_none() && jinv(p, a, b) == j, "{id} vertex {}: check failed", v["id"]);
            curves.push(json!({"vertex": v["id"], "a": a, "b": b, "gx": gx, "gy": gy, "j": j,
                               "conductor": v["conductor"], "group_order": n, "r": n}));
        }
        let mut rng = SplitMix(seed ^ c["seed"].as_u64().unwrap());
        let mut iso = vec![];
        for k in 0..n_iso {
            let of = (rng.below(curves.len() as u64)) as usize;
            let u = 2 + rng.below(p - 3);
            let (u2, u3) = (mulm(u, u, p), mulm(mulm(u, u, p), u, p));
            let base = &curves[of];
            let (a, b) = (mulm(base["a"].as_u64().unwrap(), mulm(u2, u2, p), p), mulm(base["b"].as_u64().unwrap(), mulm(u3, u3, p), p));
            let (gx, gy) = (mulm(base["gx"].as_u64().unwrap(), u2, p), mulm(base["gy"].as_u64().unwrap(), u3, p));
            let g = Some((gx, gy));
            assert!(on(p, a, b, g) && mul(p, a, n, g).is_none() && jinv(p, a, b) == base["j"].as_u64().unwrap());
            iso.push(json!({"k": k, "of": of, "u": u, "a": a, "b": b, "gx": gx, "gy": gy, "j": base["j"], "group_order": n, "r": n}));
        }
        let t = c["t"].as_i64().unwrap();
        classes.push(json!({"id": id, "stratum": st, "p": p, "n": n, "t": t.to_string(),
            "D": t as i128 * t as i128 - 4 * p as i128, "h": c["H_num"], "H_den": c["H_den"], "f": c["f"], "n_orders": c["n_orders"],
            "D_K": c["D_K"], "log2_p": (p as f64).log2(), "curves": curves, "iso_models": iso}));
    }
    let doc = json!({"schema": "p2.classes/v1", "source": "fp31-class-census ic-classes from phase2 manifest and walks; every curve checked: G on curve, n*G = O, j", "classes": classes});
    let mut f = OpenOptions::new().write(true).create_new(true).open(&out).expect("output exists");
    f.write_all(serde_json::to_string_pretty(&doc).unwrap().as_bytes()).unwrap();
    let nc: usize = doc["classes"].as_array().unwrap().iter().map(|c| c["curves"].as_array().unwrap().len()).sum();
    eprintln!("wrote {out}: {} classes, {nc} curves, {} iso models each", doc["classes"].as_array().unwrap().len(), n_iso);
}

fn predicted(n: f64, m: u32, nfb: f64) -> f64 { if m == 2 { 2.0 * nfb * nfb / n } else { 4.0 / 3.0 * nfb * nfb * nfb / n } }

pub fn cmd_ic_analyze(args: &[String]) {
    let classes = arg(args, "--classes").expect("--classes");
    let runs = arg(args, "--runs").expect("--runs");
    let m: u32 = arg(args, "--m").expect("--m").parse().unwrap();
    let out = arg(args, "--out").expect("--out");
    let perms: usize = arg(args, "--perms").map(|s| s.parse().unwrap()).unwrap_or(20000);
    let doc: Value = serde_json::from_str(&fs::read_to_string(&classes).unwrap()).unwrap();
    let (mut groups, mut cm, mut lh, mut lf, mut no, mut ldk) = (vec![], vec![], vec![], vec![], vec![], vec![]);
    let (mut icc_p, mut pc_ratio, mut pc_detect, mut iso_rms) = (vec![], vec![], vec![], vec![]);
    let (mut rows, mut missing) = (vec![], vec![]);
    let (mut chi2, mut chi_df) = (0.0f64, 0usize);
    for c in doc["classes"].as_array().unwrap() {
        let id = c["id"].as_str().unwrap();
        let dir = format!("{runs}/{id}_m{m}");
        let (a, sp) = match (fs::read_to_string(format!("{dir}/analysis.json")), fs::read_to_string(format!("{dir}/spec.json"))) {
            (Ok(a), Ok(s)) => (serde_json::from_str::<Value>(&a).unwrap(), serde_json::from_str::<Value>(&s).unwrap()),
            _ => { missing.push(id.to_string()); continue; }
        };
        let nfb: f64 = sp["arms"][0]["method"]["params"]["factor_base"].as_str().unwrap().trim_start_matches("prime-abscissa:size=").parse().unwrap();
        let n = c["n"].as_f64().unwrap();
        let pred = predicted(n, m, nfb);
        let pcs = a["per_curve"].as_array().unwrap();
        let ys: Vec<f64> = pcs.iter().map(|x| x["relations"].as_f64().unwrap() / x["attempts"].as_f64().unwrap() / pred).collect();
        let (rel, att): (f64, f64) = pcs.iter().fold((0.0, 0.0), |s, x| (s.0 + x["relations"].as_f64().unwrap(), s.1 + x["attempts"].as_f64().unwrap()));
        let pooled = rel / att;
        // within-class binomial homogeneity: Pearson chi-square of the curves' relation counts
        for x in pcs {
            let (r, t) = (x["relations"].as_f64().unwrap(), x["attempts"].as_f64().unwrap());
            let e = t * pooled;
            chi2 += (r - e).powi(2) / (e * (1.0 - pooled));
        }
        chi_df += pcs.len().saturating_sub(1);
        let y = &a["metrics"]["yield"];
        if pcs.len() >= 2 { if let Some(p) = y["icc_perm_p"].as_f64() { icc_p.push(p); } }
        if let Some(r) = y["positive_control"]["planted_effect_pc_over_main_median"].as_f64() { pc_ratio.push(r); }
        if let Some(d) = y["positive_control"]["detect_icc_gt_null_q95"].as_f64() { pc_detect.push(d); }
        if let Some(r) = y["isomorphic_model_control"]["rms_log_ratio"].as_f64() { iso_rms.push(r); }
        let mean_c = pooled / pred;
        groups.push(ys.clone());
        cm.push(mean_c);
        lh.push((c["h"].as_f64().unwrap() / c["H_den"].as_f64().unwrap()).log2());
        lf.push(c["f"].as_f64().unwrap().log2());
        no.push(c["n_orders"].as_f64().unwrap());
        ldk.push(c["D_K"].as_f64().unwrap().abs().log2());
        rows.push(json!({"id": id, "stratum": c["stratum"], "curves": pcs.len(), "fb_size": nfb, "predicted_yield": pred,
            "relations": rel, "attempts": att, "pooled_yield": pooled, "yield_over_predicted": mean_c,
            "icc_perm_p": y["icc_perm_p"], "max_abs_z_perm_p": y["max_abs_z_perm_p"],
            "planted_pc_over_main": y["positive_control"]["planted_effect_pc_over_main_median"],
            "iso_rms_log_ratio": y["isomorphic_model_control"]["rms_log_ratio"]}));
    }
    let (hk, dfk, pk) = kruskal(&groups);
    let mut tests = vec![json!({"test": "T1 cross-class Kruskal-Wallis on per-curve yield/predicted", "H": hk, "df": dfk, "p": pk})];
    for (i, (name, x)) in [("log2_H", &lh), ("log2_f", &lf), ("n_orders", &no), ("log2_abs_DK", &ldk)].iter().enumerate() {
        let (rho, p) = spearman_perm(x, &cm, perms, 0x5EED3 + i as u64 + 100 * m as u64);
        tests.push(json!({"test": format!("T{} Spearman class yield/predicted vs {name}", i + 2), "rho": rho, "p_perm": p, "n": cm.len()}));
    }
    let (d, pks) = ks_uniform(&icc_p);
    tests.push(json!({"test": "T6 within-class ICC permutation p-values ~ U(0,1)", "classes": icc_p.len(), "D": d, "p": pks}));
    let p_chi = gamma_q(chi_df as f64 / 2.0, chi2 / 2.0);
    tests.push(json!({"test": "T7 within-class binomial over-dispersion, pooled Pearson chi-square", "chi2": chi2, "df": chi_df, "p": p_chi}));
    let mean = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
    let sd = |v: &[f64]| { let mu = mean(v); (v.iter().map(|x| (x - mu).powi(2)).sum::<f64>() / (v.len() as f64 - 1.0)).sqrt() };
    let report = json!({"schema": "fp31.phase3.ic-analysis/v1", "m": m, "classes_analysed": rows.len(), "missing": missing,
        "alpha_bonferroni": 0.05 / 14.0, "primary_tests": tests,
        "class_yield_over_predicted": {"mean": mean(&cm), "sd_between": sd(&cm),
            "min": cm.iter().cloned().fold(f64::INFINITY, f64::min), "max": cm.iter().cloned().fold(0.0, f64::max)},
        "controls": {"planted_pc_over_main_median_mean": mean(&pc_ratio), "planted_detect_rate_mean": mean(&pc_detect), "classes_with_plant": pc_ratio.len(),
                     "iso_rms_log_ratio_mean": mean(&iso_rms), "classes_with_iso": iso_rms.len()},
        "classes": rows});
    let mut f = OpenOptions::new().write(true).create_new(true).open(&out).expect("output exists");
    f.write_all(serde_json::to_string_pretty(&report).unwrap().as_bytes()).unwrap();
}
