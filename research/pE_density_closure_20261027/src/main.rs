//! Analysis of RECORDED values only (no experiment is run, no field arithmetic
//! is redone): reads the twist-obstruction `rows.jsonl`, and recomputes
//!   * mean solutions per target and p_E per (n, l, m, curve);
//!   * the counting-identity prediction C(|S|+m-1, m) / |G|, |S| = 2 v_E + 1;
//!   * the log-log slope of mean yield on p_E at fixed (n, l, m);
//!   * the normalised character sum z = (2p-1) sqrt(2^l - 1) per group;
//!   * the large-l budget numbers used in the closure note (REPORT.typ sec. 4).
//! Usage: pe-density-closure <rows.jsonl> <out_dir>
//! Writes closure.json, groups.tsv (graph data) and prints a summary.

use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;

#[derive(Clone)]
struct Group {
    n: u64,
    l: u64,
    m: u64,
    curve: String,
    a: u64,
    b: u64,
    v_e: f64,
    v_nz: f64,
    all: Vec<f64>,
    clean: Vec<f64>,
    pure_clean: Vec<f64>,
    contam: usize,
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}
fn var(v: &[f64]) -> f64 {
    let mu = mean(v);
    v.iter().map(|x| (x - mu) * (x - mu)).sum::<f64>() / (v.len() as f64 - 1.0)
}
fn choose(n: f64, k: u64) -> f64 {
    // C(n, k) for real n >= k
    let mut r = 1.0;
    for i in 0..k {
        r *= (n - i as f64) / (i as f64 + 1.0);
    }
    r
}
fn u(v: &Value, k: &str) -> u64 {
    v[k].as_u64().unwrap_or_else(|| panic!("missing {k}"))
}

/// Inverse standard normal upper tail: z with P(Z > z) = q (Acklam-style
/// refinement by bisection on erfc, adequate for q >= 1e-30).
fn z_upper(q: f64) -> f64 {
    let (mut lo, mut hi) = (0.0f64, 40.0f64);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if 0.5 * erfc(mid / 2f64.sqrt()) > q {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}
fn erfc(x: f64) -> f64 {
    // continued fraction for large x, series for small x
    if x < 2.5 {
        // erf series
        let mut sum = x;
        let mut term = x;
        for k in 1..200 {
            term *= -x * x / k as f64;
            sum += term / (2 * k + 1) as f64;
        }
        1.0 - 2.0 / std::f64::consts::PI.sqrt() * sum
    } else {
        let mut f = 0.0;
        for k in (1..=100).rev() {
            f = k as f64 / 2.0 / (x + f);
        }
        (-x * x).exp() / (std::f64::consts::PI.sqrt() * (x + f))
    }
}

/// Weighted fixed-effect slope of y on x (group = (n,l,m) key within one m).
/// Returns (slope, se, dof).
fn fe_slope(pts: &[(u64, f64, f64, f64)]) -> (f64, f64, usize) {
    // pts: (group id, x, y, weight)
    let mut by: BTreeMap<u64, Vec<(f64, f64, f64)>> = BTreeMap::new();
    for &(g, x, y, w) in pts {
        by.entry(g).or_default().push((x, y, w));
    }
    let (mut sxx, mut sxy, mut k) = (0.0, 0.0, 0usize);
    let mut dem: Vec<(f64, f64, f64)> = vec![];
    for v in by.values() {
        let sw: f64 = v.iter().map(|t| t.2).sum();
        let mx = v.iter().map(|t| t.0 * t.2).sum::<f64>() / sw;
        let my = v.iter().map(|t| t.1 * t.2).sum::<f64>() / sw;
        k += 1;
        for &(x, y, w) in v {
            sxx += w * (x - mx) * (x - mx);
            sxy += w * (x - mx) * (y - my);
            dem.push((x - mx, y - my, w));
        }
    }
    let b = sxy / sxx;
    let n = pts.len();
    let dof = n - k - 1;
    let rss: f64 = dem
        .iter()
        .map(|&(x, y, w)| w * (y - b * x) * (y - b * x))
        .sum();
    let s2 = rss / dof as f64;
    (b, (s2 / sxx).sqrt(), dof)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rows_path = &args[1];
    let out_dir = &args[2];
    fs::create_dir_all(out_dir).unwrap();
    let text = fs::read_to_string(rows_path).expect("rows.jsonl");

    let mut census: BTreeMap<(u64, u64, u64), f64> = BTreeMap::new();
    let mut groups: BTreeMap<(u64, u64, u64, u64, u64), Group> = BTreeMap::new();
    let mut checks = (0usize, 0usize); // (rows where solutions != sum of parts, rows used)
    for line in text.lines() {
        let r: Value = serde_json::from_str(line).unwrap();
        match r["family"].as_str().unwrap() {
            "census" => {
                census.insert(
                    (u(&r, "n"), u(&r, "a"), u(&r, "b")),
                    r["E_Fq"].as_f64().unwrap(),
                );
            }
            "decomp" if r["arm"] == "A_random" => {
                let cm = &r["curve_model"];
                let (n, l, m, a, b) = (u(&r, "n"), u(&r, "l"), u(&r, "m"), u(&r, "a"), u(&r, "b"));
                let sol = u(cm, "solutions") as f64;
                let parts = u(cm, "pure")
                    + u(cm, "pure_with_2tor")
                    + u(cm, "off_O")
                    + u(cm, "off_T")
                    + u(cm, "violation");
                checks.1 += 1;
                if parts as f64 != sol {
                    checks.0 += 1;
                }
                // Contaminated target: x_R in V (low-degree polynomial basis,
                // so bit pattern < 2^l) or any off-parity solution recorded
                // (the predicted degenerate families (x_R,x,x), (x(R+T),..)).
                let xr_in_v = u(&r, "target_x") < (1u64 << l);
                let off = u(cm, "off_O") + u(cm, "off_T");
                let contaminated = xr_in_v || off > 0;
                let g = groups.entry((n, l, m, a, b)).or_insert_with(|| Group {
                    n,
                    l,
                    m,
                    curve: r["curve"].as_str().unwrap().to_string(),
                    a,
                    b,
                    v_e: u(&r, "v_E_type") as f64,
                    v_nz: u(&r, "v_nonzero") as f64,
                    all: vec![],
                    clean: vec![],
                    pure_clean: vec![],
                    contam: 0,
                });
                g.all.push(sol);
                if contaminated {
                    g.contam += 1;
                } else {
                    g.clean.push(sol);
                    g.pure_clean
                        .push((u(cm, "pure") + u(cm, "pure_with_2tor")) as f64);
                }
            }
            _ => {}
        }
    }

    let mut tsv = String::from("n\tl\tm\tcurve\ta\tb\tN\tN_clean\tv_E\tp_E\tG\tG_src\tmean_all\tmean_clean\tse_clean\tfano_clean\tpred\tratio\tmean_over_pm\tz\n");
    let mut jgroups = vec![];
    // per (n,l,m) bookkeeping
    let mut cell: BTreeMap<(u64, u64, u64), Vec<(f64, f64, f64, f64, f64)>> = BTreeMap::new(); // p, mean_clean, se, ratio, mean/p^m
    let mut fe_pts: BTreeMap<u64, Vec<(u64, f64, f64, f64)>> = BTreeMap::new();
    let mut fe_pts_all: BTreeMap<u64, Vec<(u64, f64, f64, f64)>> = BTreeMap::new();
    let mut zs: BTreeMap<u64, Vec<f64>> = BTreeMap::new();
    for g in groups.values() {
        let p = g.v_e / g.v_nz;
        let (gord, src) = match census.get(&(g.n, g.a, g.b)) {
            Some(&e) => (e, "census"),
            None => (
                (1u64 << g.n) as f64 + 1.0,
                "2^n+1 (Hasse error <= 2^(n/2+1)/2^n)",
            ),
        };
        let s = 2.0 * g.v_e + 1.0;
        let pred = choose(s + g.m as f64 - 1.0, g.m) / gord;
        let mc = mean(&g.clean);
        let vc = var(&g.clean);
        let se = (vc / g.clean.len() as f64).sqrt();
        let z = (2.0 * p - 1.0) * g.v_nz.sqrt();
        let ratio = mc / pred;
        let mpm = mc / p.powi(g.m as i32);
        let ma = mean(&g.all);
        let _ = writeln!(
            tsv,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:.5}\t{}\t{}\t{:.4}\t{:.4}\t{:.4}\t{:.3}\t{:.4}\t{:.4}\t{:.3}\t{:.3}",
            g.n, g.l, g.m, g.curve, g.a, g.b, g.all.len(), g.clean.len(), g.v_e, p, gord,
            src.split(' ').next().unwrap(), ma, mc, se, vc / mc, pred, ratio, mpm, z
        );
        cell.entry((g.n, g.l, g.m))
            .or_default()
            .push((p, mc, se, ratio, mpm));
        let gid = g.n * 100 + g.l;
        // weight = 1 / var(log mean) ~ N_clean * mean / fano (delta method)
        let w = g.clean.len() as f64 * mc * mc / vc;
        fe_pts
            .entry(g.m)
            .or_default()
            .push((gid, p.ln(), mc.ln(), w));
        // same regression with the recorded totals (outliers included)
        let va = var(&g.all);
        fe_pts_all.entry(g.m).or_default().push((
            gid,
            p.ln(),
            ma.ln(),
            g.all.len() as f64 * ma * ma / va,
        ));
        zs.entry(g.l).or_default().push(z);
        jgroups.push(json!({
            "n": g.n, "l": g.l, "m": g.m, "curve": g.curve, "a": g.a, "b": g.b,
            "targets": g.all.len(), "targets_clean": g.clean.len(), "targets_contaminated": g.contam,
            "v_E_type": g.v_e, "v_nonzero": g.v_nz, "p_E": p, "group_order": gord, "group_order_source": src,
            "mean_solutions_recorded": ma, "mean_solutions_clean": mc, "se_clean": se,
            "fano_clean": vc / mc, "mean_pure_clean": mean(&g.pure_clean),
            "pred_multiset_over_G": pred, "obs_over_pred": ratio, "mean_over_p_pow_m": mpm, "z_charsum": z,
        }));
    }

    // spread per (n,l,m)
    let mut jcells = vec![];
    for (&(n, l, m), v) in &cell {
        let means: Vec<f64> = v.iter().map(|t| t.1).collect();
        let mx = means.iter().cloned().fold(f64::MIN, f64::max);
        let mn = means.iter().cloned().fold(f64::MAX, f64::min);
        let ratios: Vec<f64> = v.iter().map(|t| t.3).collect();
        let mpm: Vec<f64> = v.iter().map(|t| t.4).collect();
        let cv = |x: &[f64]| var(x).sqrt() / mean(x);
        // expected CV of a ratio from sampling error alone
        let rel_se2: f64 = v.iter().map(|t| (t.2 / t.1).powi(2)).sum::<f64>() / v.len() as f64;
        jcells.push(json!({
            "n": n, "l": l, "m": m, "curves": v.len(), "max_over_min_mean": mx / mn,
            "p_min": v.iter().map(|t| t.0).fold(f64::MAX, f64::min), "p_max": v.iter().map(|t| t.0).fold(f64::MIN, f64::max),
            "cv_mean_over_pm": cv(&mpm), "cv_obs_over_pred": cv(&ratios),
            "cv_expected_from_sampling_se": rel_se2.sqrt(),
            "mean_obs_over_pred": mean(&ratios),
        }));
    }

    let mut jslopes = vec![];
    for (&m, pts) in &fe_pts {
        let (b, se, dof) = fe_slope(pts);
        let (ba, sea, _) = fe_slope(&fe_pts_all[&m]);
        jslopes.push(json!({
            "m": m, "groups": pts.len(), "dof": dof,
            "slope_clean": b, "se_clean": se, "z_vs_m": (b - m as f64) / se, "z_vs_0": b / se,
            "slope_recorded_totals": ba, "se_recorded_totals": sea,
        }));
    }

    let mut jz = vec![];
    let all_z: Vec<f64> = zs.values().flatten().cloned().collect();
    let pooled_rms = (all_z.iter().map(|z| z * z).sum::<f64>() / all_z.len() as f64).sqrt();
    for (&l, v) in &zs {
        let rms = (v.iter().map(|z| z * z).sum::<f64>() / v.len() as f64).sqrt();
        jz.push(
            json!({"l": l, "groups": v.len(), "rms_z": rms, "n_distinct_expected_sd": 1.0,
            "sigma_p_predicted": 0.5 / ((1u64 << l) as f64 - 1.0).sqrt()}),
        );
    }

    // Budget for ECC2K-130: n = 131, l = 44, m = 3, p ~ 1/2.
    let l = 44.0f64;
    let sigma_p = 0.5 / (2f64.powf(l) - 1.0).sqrt();
    let p0 = 0.5;
    let mut budget = vec![];
    for (label, log2k) in [
        ("2^10 curves", 10.0f64),
        ("2^20 curves", 20.0),
        ("class-size ceiling 2^66", 66.0),
        ("all 44-dim subspaces of F_2^131 (2^3828)", 3828.0),
    ] {
        let lnk = log2k * std::f64::consts::LN_2;
        // exact Gaussian upper quantile when representable, else sqrt(2 lnK) asymptotics
        let z = if log2k <= 66.0 {
            z_upper((-lnk).exp())
        } else {
            (2.0 * lnk).sqrt()
        };
        let dp = z * sigma_p;
        let gain_m3 = ((p0 + dp) / p0).powi(3) - 1.0;
        let bits_dimv = (p0 / (p0 + dp)).log2().abs(); // dim V saved at fixed |F|
        budget.push(
            json!({"candidates": label, "log2_K": log2k, "z_max": z, "delta_p": dp,
            "relative_yield_gain_fixedV_m3": gain_m3, "dimV_bits_saved_fixed_F": bits_dimv}),
        );
    }
    let need_dp = 0.01 * p0 / 3.0; // 1% yield gain at m = 3
    let need_z = need_dp / sigma_p;
    let need_ln_k = need_z * need_z / 2.0;
    let adversarial = json!({
        "yield_gain_if_p_were_1_fixedV_m3": (1.0f64 / p0).powi(3),
        "dimV_bits_saved_if_p_were_1_fixed_F": (1.0f64 / p0).log2(),
    });

    let out = json!({
        "source": rows_path,
        "decomp_rows_used": checks.1,
        "rows_where_solutions_ne_sum_of_parts": checks.0,
        "pooled_rms_z": pooled_rms, "pooled_rms_z_groups": all_z.len(), "groups": jgroups, "cells": jcells, "slopes": jslopes, "charsum": jz,
        "budget_n131_l44_m3": { "sigma_p": sigma_p, "rows": budget,
            "z_needed_for_1pct_yield": need_z, "ln_K_needed_for_1pct_yield": need_ln_k,
            "log2_K_needed_for_1pct_yield": need_ln_k / std::f64::consts::LN_2,
            "adversarial_bound": adversarial },
    });
    fs::write(
        format!("{out_dir}/closure.json"),
        serde_json::to_string_pretty(&out).unwrap(),
    )
    .unwrap();
    fs::write(format!("{out_dir}/groups.tsv"), &tsv).unwrap();
    print!("{tsv}");
    println!("{}", serde_json::to_string_pretty(&out["cells"]).unwrap());
    println!("{}", serde_json::to_string_pretty(&out["slopes"]).unwrap());
    println!("{}", serde_json::to_string_pretty(&out["charsum"]).unwrap());
    println!(
        "{}",
        serde_json::to_string_pretty(&out["budget_n131_l44_m3"]).unwrap()
    );
    println!("parts-mismatch rows: {} of {}", checks.0, checks.1);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn choose_works() {
        assert!((choose(5.0, 3) - 10.0).abs() < 1e-12);
        assert!((choose(71.0 + 2.0, 3) - 73.0 * 72.0 * 71.0 / 6.0).abs() < 1e-6);
    }
    #[test]
    fn normal_tail() {
        assert!((z_upper(0.025) - 1.95996).abs() < 1e-3);
        assert!((z_upper(1e-9) - 5.9978).abs() < 1e-2);
    }
    #[test]
    fn slope_recovers_exponent() {
        let pts: Vec<(u64, f64, f64, f64)> = (1..6)
            .map(|i| (1u64, (i as f64).ln(), 3.0 * (i as f64).ln() + 2.0, 1.0))
            .chain((1..6).map(|i| {
                (
                    2u64,
                    (i as f64).ln(),
                    3.0 * (i as f64).ln() - 1.0 + 0.001 * i as f64,
                    1.0,
                )
            }))
            .collect();
        let (b, _, _) = fe_slope(&pts);
        assert!((b - 3.0).abs() < 0.01);
    }
}
