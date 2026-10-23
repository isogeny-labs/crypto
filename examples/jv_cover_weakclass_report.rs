//! Aggregate the paired A/B/A2 `jv_cover --exp dlp` records of
//! `research/ic_cover_weakclass_20261023` into `summary.json`, `table.md` and `s_over_rho.svg`.
//!
//! ```text
//! cargo run --release --example jv_cover_weakclass_report -- research/ic_cover_weakclass_20261023/raw research/ic_cover_weakclass_20261023
//! ```
//!
//! Files in `raw/` are named `p{P}_s{SEED}_{A|B|A2}.json`, each a one-report array written by
//! `jv_cover --json`.  Counts are read, never recomputed; this program only pairs, divides and draws.

use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;

fn f(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap_or(f64::NAN)
}
fn u(v: &Value, k: &str) -> u64 {
    v[k].as_u64().unwrap_or(0)
}
fn mean(x: &[f64]) -> f64 {
    x.iter().sum::<f64>() / x.len().max(1) as f64
}
fn sd(x: &[f64]) -> f64 {
    if x.len() < 2 {
        return 0.0;
    }
    let m = mean(x);
    (x.iter().map(|v| (v - m).powi(2)).sum::<f64>() / (x.len() - 1) as f64).sqrt()
}
fn slope(pts: &[(f64, f64)]) -> (f64, f64) {
    // least squares of ln y on ln x; slope and its standard error
    let n = pts.len() as f64;
    if pts.len() < 3 {
        return (f64::NAN, f64::NAN);
    }
    let xs: Vec<f64> = pts.iter().map(|q| q.0.ln()).collect();
    let ys: Vec<f64> = pts.iter().map(|q| q.1.ln()).collect();
    let (mx, my) = (mean(&xs), mean(&ys));
    let sxx: f64 = xs.iter().map(|x| (x - mx).powi(2)).sum();
    let sxy: f64 = xs.iter().zip(&ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let b = sxy / sxx;
    let a = my - b * mx;
    let rss: f64 = xs.iter().zip(&ys).map(|(x, y)| (y - a - b * x).powi(2)).sum();
    (b, (rss / (n - 2.0) / sxx).sqrt())
}

fn load(dir: &str, p: u64, s: u64, arm: &str) -> Option<Value> {
    let t = fs::read_to_string(format!("{dir}/p{p}_s{s}_{arm}.json")).ok()?;
    let v: Value = serde_json::from_str(&t).ok()?;
    v.as_array()?.first().cloned()
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let (raw, out) = (&a[0], &a[1]);
    let mut cells: BTreeMap<(u64, u64), ()> = BTreeMap::new();
    for e in fs::read_dir(raw).unwrap() {
        let n = e.unwrap().file_name().to_string_lossy().to_string();
        if let Some(rest) = n.strip_prefix('p') {
            let parts: Vec<&str> = rest.trim_end_matches(".json").split('_').collect();
            if parts.len() == 3 && n.ends_with(".json") {
                if let (Ok(p), Ok(s)) = (
                    parts[0].parse::<u64>(),
                    parts[1].trim_start_matches('s').parse::<u64>(),
                ) {
                    cells.insert((p, s), ());
                }
            }
        }
    }
    // rho per p: pooled over the arm-A runs of that p; 503 and any p without runs use the pooled mean of all
    let mut rho_by_p: BTreeMap<u64, Vec<(f64, f64, f64)>> = BTreeMap::new(); // (S_rho, steps, n)
    let mut all_rho: Vec<(f64, f64, f64)> = Vec::new();
    for &(p, s) in cells.keys() {
        if let Some(r) = load(raw, p, s, "A") {
            let k = r["rho"].as_array().map_or(0, |x| x.len()) as f64;
            if k > 0.0 {
                let e = (f(&r, "rho_s_mean"), f(&r, "rho_steps_mean"), k);
                rho_by_p.entry(p).or_default().push(e);
                all_rho.push(e);
            }
        }
    }
    let pool = |v: &[(f64, f64, f64)]| -> (f64, f64, f64) {
        let n: f64 = v.iter().map(|e| e.2).sum();
        (
            v.iter().map(|e| e.0 * e.2).sum::<f64>() / n,
            v.iter().map(|e| e.1 * e.2).sum::<f64>() / n,
            n,
        )
    };
    let pooled_all = pool(&all_rho);
    let mut rows: Vec<Value> = Vec::new();
    let mut md = String::new();
    md.push_str("| p | seed | l (bits) | residuals | relations | digest A=B | A/A | S_A | S_B | S_A/S_B | S/rho A | S/rho B | F4 muls A | F4 muls B (check) | fallbacks | correct A,B | guards |\n|--:|--:|--:|--:|--:|:--|:--|--:|--:|--:|--:|--:|--:|--:|--:|:--|:--|\n");
    let mut per_p: BTreeMap<u64, Vec<(f64, f64, f64, f64, f64, f64, f64)>> = BTreeMap::new();
    let mut all_ok = true;
    for &(p, s) in cells.keys() {
        let (Some(ra), rb, ra2) = (load(raw, p, s, "A"), load(raw, p, s, "B"), load(raw, p, s, "A2")) else {
            continue;
        };
        let rho = rho_by_p.get(&p).map(|v| pool(v)).unwrap_or(pooled_all);
        let rho_src = if rho_by_p.contains_key(&p) { "measured at this p" } else { "pooled over all measured p" };
        let ce = f(&ra, "c_add_e");
        let sqrt_l = (u(&ra, "l") as f64).sqrt();
        let rho_muls = rho.0 * sqrt_l * ce;
        let tot_a = u(&ra, "total_muls") as f64;
        let s_a = f(&ra, "s");
        let mut cell = json!({
            "p": p, "seed": s, "l_bits": f(&ra, "bits"), "rho_source": rho_src,
            "rho_S": rho.0, "rho_runs_pooled": rho.2, "rho_steps_per_group_op": rho.1 / (rho.0 * sqrt_l),
            "A": {"total_muls": tot_a, "S": s_a, "S_over_rho": tot_a / rho_muls,
                  "residuals": u(&ra, "residuals"), "relations": u(&ra, "relations"),
                  "decompositions": u(&ra, "decompositions"), "digest": u(&ra, "relation_digest"),
                  "solved": ra["solved"], "correct": ra["correct"], "mismatches": u(&ra, "mismatches"),
                  "f4_muls": u(&ra, "f4_muls"), "lin_muls": u(&ra, "lin_muls"), "weil_muls": u(&ra, "weil_muls"),
                  "post_muls": u(&ra, "post_muls"), "verify_muls": u(&ra, "verify_muls"),
                  "stream_muls": u(&ra, "stream_muls"), "setup_muls": u(&ra, "setup_muls"),
                  "base_muls": u(&ra, "base_muls"), "la_muls": u(&ra, "la_muls"),
                  "c_cov": f(&ra, "c_cov"), "predicted_S_over_rho_formula": f(&ra, "predicted_s_over_rho"),
                  "wall_s": f(&ra, "wall_ms") / 1e3},
        });
        let mut guard = String::new();
        let mut line = format!("| {p} | {s} | {:.1} | {} | {} |", f(&ra, "bits"), u(&ra, "residuals"), u(&ra, "relations"));
        let mut ratio = f64::NAN;
        let mut digest_eq = "n/a".to_string();
        if let Some(rb) = &rb {
            let tot_b = u(rb, "total_muls") as f64;
            ratio = tot_a / tot_b;
            let eq = u(rb, "residuals") == u(&ra, "residuals")
                && u(rb, "relations") == u(&ra, "relations")
                && u(rb, "decompositions") == u(&ra, "decompositions")
                && u(rb, "relation_digest") == u(&ra, "relation_digest");
            digest_eq = if eq { "equal".into() } else { "DIFFERENT".into() };
            let correct = ra["correct"] == json!(true) && rb["correct"] == json!(true) && ra["solved"] == json!(true) && rb["solved"] == json!(true);
            let mism = u(rb, "mismatches") == 0 && u(&ra, "mismatches") == 0;
            let counter_ok = u(rb, "f4_muls") == u(rb, "f4_muls_per_test_sum");
            let lower = tot_b < tot_a;
            let ok = eq && correct && mism && counter_ok && lower;
            if !eq { guard += "relations differ; "; }
            if !correct { guard += "not solved/correct; "; }
            if !mism { guard += "oracle mismatch; "; }
            if !counter_ok { guard += "f4 counter inconsistent; "; }
            if !lower { guard += "S not lower; "; }
            if guard.is_empty() { guard = "pass".into(); }
            all_ok &= ok;
            let rho_w: Vec<f64> = [0.0, 0.1, 1.0]
                .iter()
                .map(|w| tot_b / (rho_muls * (1.0 + w * 2.0 * rho.1 / (rho.0 * sqrt_l))))
                .collect();
            let rho_wa: Vec<f64> = [0.0, 0.1, 1.0]
                .iter()
                .map(|w| tot_a / (rho_muls * (1.0 + w * 2.0 * rho.1 / (rho.0 * sqrt_l))))
                .collect();
            cell["B"] = json!({"total_muls": tot_b, "S": f(rb, "s"), "S_over_rho": tot_b / rho_muls,
                  "S_over_rho_band_w0_w01_w1": rho_w, "residuals": u(rb, "residuals"), "relations": u(rb, "relations"),
                  "decompositions": u(rb, "decompositions"), "digest": u(rb, "relation_digest"),
                  "solved": rb["solved"], "correct": rb["correct"], "mismatches": u(rb, "mismatches"),
                  "f4_muls": u(rb, "f4_muls"), "f4_muls_per_test_sum": u(rb, "f4_muls_per_test_sum"),
                  "hilbert_applied": u(rb, "hilbert_applied"), "hilbert_fallbacks": u(rb, "hilbert_fallbacks"),
                  "hilbert_check_muls": u(rb, "hilbert_check_muls"),
                  "lin_muls": u(rb, "lin_muls"), "weil_muls": u(rb, "weil_muls"), "post_muls": u(rb, "post_muls"),
                  "verify_muls": u(rb, "verify_muls"), "stream_muls": u(rb, "stream_muls"),
                  "setup_muls": u(rb, "setup_muls"), "base_muls": u(rb, "base_muls"), "la_muls": u(rb, "la_muls"),
                  "c_cov": f(rb, "c_cov"), "predicted_S_over_rho_formula": f(rb, "predicted_s_over_rho"),
                  "wall_s": f(rb, "wall_ms") / 1e3});
            cell["A"]["S_over_rho_band_w0_w01_w1"] = json!(rho_wa);
            cell["ratio_S_A_over_S_B"] = json!(ratio);
            cell["relations_equal"] = json!(eq);
            cell["guards"] = json!(guard);
            per_p.entry(p).or_default().push((
                tot_a / rho_muls,
                tot_b / rho_muls,
                ratio,
                f(&ra, "predicted_s_over_rho"),
                f(rb, "predicted_s_over_rho"),
                s_a,
                f(rb, "s"),
            ));
        }
        let mut aa = "-".to_string();
        if let Some(r2) = &ra2 {
            let same = u(r2, "total_muls") == u(&ra, "total_muls")
                && u(r2, "relation_digest") == u(&ra, "relation_digest")
                && u(r2, "residuals") == u(&ra, "residuals");
            let ratio_aa = f(&ra, "s") / f(r2, "s");
            aa = format!("{ratio_aa:.3}");
            cell["A2"] = json!({"total_muls": u(r2, "total_muls"), "S": f(r2, "s"), "ratio_A_over_A2": ratio_aa, "identical": same});
            all_ok &= same;
        }
        if let Some(rb) = &rb {
            line += &format!(
                " {digest_eq} | {aa} | {:.4e} | {:.4e} | {ratio:.4} | {:.1} | {:.1} | {} | {} ({}) | {}/{} | {} {} |\n",
                s_a,
                f(rb, "s"),
                tot_a / rho_muls,
                u(rb, "total_muls") as f64 / rho_muls,
                u(&ra, "f4_muls"),
                u(rb, "f4_muls"),
                u(rb, "hilbert_check_muls"),
                u(rb, "hilbert_fallbacks"),
                ra["correct"],
                rb["correct"],
                guard,
                ""
            );
        } else {
            line += &format!(" A only | {aa} | {:.4e} | - | - | {:.1} | - | {} | - | - | {} | A only |\n", s_a, tot_a / rho_muls, u(&ra, "f4_muls"), ra["correct"]);
        }
        md += &line.replace("| n/a |", "| n/a |");
        rows.push(cell);
    }
    // per-p aggregates and the slope
    let mut agg = Vec::new();
    let mut fit_a = Vec::new();
    let mut fit_b = Vec::new();
    md.push_str("\n| p | n seeds | S/rho A mean (sd) [min, max] | S/rho B mean (sd) [min, max] | S_A/S_B mean [min, max] | formula-predicted S/rho A | formula-predicted S/rho B |\n|--:|--:|:--|:--|:--|--:|--:|\n");
    for (p, v) in &per_p {
        let ca: Vec<f64> = v.iter().map(|x| x.0).collect();
        let cb: Vec<f64> = v.iter().map(|x| x.1).collect();
        let cr: Vec<f64> = v.iter().map(|x| x.2).collect();
        let pa: Vec<f64> = v.iter().map(|x| x.3).collect();
        let pb: Vec<f64> = v.iter().map(|x| x.4).collect();
        let mm = |x: &[f64]| (x.iter().cloned().fold(f64::INFINITY, f64::min), x.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
        let (a0, a1) = mm(&ca);
        let (b0, b1) = mm(&cb);
        let (r0, r1) = mm(&cr);
        md += &format!(
            "| {p} | {} | {:.1} ({:.1}) [{:.1}, {:.1}] | {:.1} ({:.1}) [{:.1}, {:.1}] | {:.3} [{:.3}, {:.3}] | {:.1} | {:.1} |\n",
            v.len(), mean(&ca), sd(&ca), a0, a1, mean(&cb), sd(&cb), b0, b1, mean(&cr), r0, r1, mean(&pa), mean(&pb)
        );
        agg.push(json!({"p": p, "n": v.len(), "S_over_rho_A": {"mean": mean(&ca), "sd": sd(&ca), "min": a0, "max": a1},
            "S_over_rho_B": {"mean": mean(&cb), "sd": sd(&cb), "min": b0, "max": b1},
            "ratio": {"mean": mean(&cr), "min": r0, "max": r1},
            "formula_A": mean(&pa), "formula_B": mean(&pb)}));
        fit_a.push((*p as f64, mean(&ca)));
        fit_b.push((*p as f64, mean(&cb)));
    }
    let (sa, ea) = slope(&fit_a);
    let (sb, eb) = slope(&fit_b);
    md += &format!("\nlog-log slope of mean S/rho against p over {} sizes: A {:.2} +- {:.2}, B {:.2} +- {:.2}\n", fit_a.len(), sa, ea, sb, eb);
    md += &format!("\nAll cells pass every guard and every A/A is identical: {all_ok}\n");
    fs::write(
        format!("{out}/summary.json"),
        serde_json::to_string_pretty(&json!({"cells": rows, "per_p": agg,
            "slope_A": [sa, ea], "slope_B": [sb, eb], "all_guards_pass": all_ok})).unwrap(),
    )
    .unwrap();
    fs::write(format!("{out}/table.md"), &md).unwrap();
    fs::write(format!("{out}/s_over_rho.svg"), svg(&per_p, &rows)).unwrap();
    print!("{md}");
}

fn svg(per_p: &BTreeMap<u64, Vec<(f64, f64, f64, f64, f64, f64, f64)>>, rows: &[Value]) -> String {
    let (w, h) = (1100.0, 520.0);
    // panel 1: S/rho against p, log-log
    let (x0, x1, y0, y1) = (70.0, 560.0, 60.0, 440.0);
    let (pmin, pmax) = (40.0f64, 700.0f64);
    let (vmin, vmax) = (1.0f64, 2000.0f64);
    let xm = |p: f64| x0 + (p.ln() - pmin.ln()) / (pmax.ln() - pmin.ln()) * (x1 - x0);
    let ym = |v: f64| y1 - (v.ln() - vmin.ln()) / (vmax.ln() - vmin.ln()) * (y1 - y0);
    let mut s = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' viewBox='0 0 {w} {h}' font-family='sans-serif' font-size='12'>\n<rect width='100%' height='100%' fill='white'/>\n");
    s += "<text x='70' y='24' font-size='15' font-weight='bold'>Cover IC, genus 3: S/rho per seed against p (lower-bound comparison)</text>\n";
    s += "<text x='70' y='42' fill='#444'>A = full F4 + sparse FGLM (baseline); B = A + Hilbert early stop; counted F_p multiplications, rho_e reference</text>\n";
    for v in [1.0, 10.0, 100.0, 1000.0] {
        s += &format!("<line x1='{x0}' y1='{:.1}' x2='{x1}' y2='{:.1}' stroke='#ddd'/><text x='{:.1}' y='{:.1}' text-anchor='end'>{v}</text>\n", ym(v), ym(v), x0 - 6.0, ym(v) + 4.0);
    }
    for p in [53.0, 101.0, 251.0, 503.0] {
        s += &format!("<line x1='{:.1}' y1='{y0}' x2='{:.1}' y2='{y1}' stroke='#eee'/><text x='{:.1}' y='{:.1}' text-anchor='middle'>{p}</text>\n", xm(p), xm(p), xm(p), y1 + 16.0);
    }
    s += &format!("<rect x='{x0}' y='{y0}' width='{}' height='{}' fill='none' stroke='#888'/>\n", x1 - x0, y1 - y0);
    s += &format!("<line x1='{x0}' y1='{:.1}' x2='{x1}' y2='{:.1}' stroke='#c00' stroke-dasharray='5 3'/><text x='{:.1}' y='{:.1}' fill='#c00'>rho (S/rho = 1)</text>\n", ym(1.0), ym(1.0), x1 - 110.0, ym(1.0) - 5.0);
    s += &format!("<text x='{:.1}' y='{:.1}' text-anchor='middle'>p</text><text x='18' y='{:.1}' transform='rotate(-90 18 {:.1})' text-anchor='middle'>S / rho (counted F_p multiplications)</text>\n", (x0 + x1) / 2.0, y1 + 36.0, (y0 + y1) / 2.0, (y0 + y1) / 2.0);
    let mut ma = Vec::new();
    let mut mb = Vec::new();
    for (p, v) in per_p {
        let n = v.len() as f64;
        for (i, x) in v.iter().enumerate() {
            let dx = (i as f64 - (n - 1.0) / 2.0) * 4.0;
            s += &format!("<circle cx='{:.1}' cy='{:.1}' r='3.5' fill='#1f5fa8' fill-opacity='0.8'/>\n", xm(*p as f64) - 7.0 + dx, ym(x.0));
            s += &format!("<rect x='{:.1}' y='{:.1}' width='7' height='7' fill='#d97a00' fill-opacity='0.85'/>\n", xm(*p as f64) + 4.0 + dx, ym(x.1) - 3.5);
        }
        ma.push((xm(*p as f64) - 7.0, ym(v.iter().map(|x| x.0).sum::<f64>() / n)));
        mb.push((xm(*p as f64) + 7.0, ym(v.iter().map(|x| x.1).sum::<f64>() / n)));
    }
    for (m, c) in [(&ma, "#1f5fa8"), (&mb, "#d97a00")] {
        let pts: Vec<String> = m.iter().map(|q| format!("{:.1},{:.1}", q.0, q.1)).collect();
        s += &format!("<polyline points='{}' fill='none' stroke='{c}' stroke-width='1.5'/>\n", pts.join(" "));
    }
    s += &format!("<circle cx='{}' cy='{}' r='3.5' fill='#1f5fa8'/><text x='{}' y='{}'>A baseline (per seed; line = mean)</text>\n", x0 + 14.0, y0 + 14.0, x0 + 24.0, y0 + 18.0);
    s += &format!("<rect x='{}' y='{}' width='7' height='7' fill='#d97a00'/><text x='{}' y='{}'>B Hilbert stop (per seed; line = mean)</text>\n", x0 + 10.0, y0 + 26.0, x0 + 24.0, y0 + 34.0);
    // panel 2: counted multiplications per residual by stage, largest p with both arms
    let big = rows.iter().rev().find(|c| c.get("B").is_some());
    if let Some(c) = big {
        let bx0 = 650.0;
        let stages = [("f4", "f4_muls", "#1f5fa8"), ("linear", "lin_muls", "#6a3d9a"), ("weil", "weil_muls", "#2a9d8f"), ("post", "post_muls", "#999999"), ("verify", "verify_muls", "#e0b000")];
        let res = c["A"]["residuals"].as_f64().unwrap_or(1.0);
        let maxv = stages.iter().map(|(_, k, _)| c["A"][*k].as_f64().unwrap_or(0.0) / res).fold(0.0f64, f64::max) * 1.1;
        s += &format!("<text x='{bx0}' y='60' font-weight='bold'>Counted multiplications per residual, p = {}, seed {}</text>\n", c["p"], c["seed"]);
        for (si, (name, key, col)) in stages.iter().enumerate() {
            let yy = 90.0 + si as f64 * 62.0;
            s += &format!("<text x='{bx0}' y='{:.1}'>{name}</text>\n", yy + 12.0);
            for (ai, arm) in ["A", "B"].iter().enumerate() {
                let v = c[*arm][*key].as_f64().unwrap_or(0.0) / res;
                let bw = v / maxv * 300.0;
                let y = yy + 18.0 + ai as f64 * 17.0;
                s += &format!("<rect x='{bx0}' y='{y:.1}' width='{bw:.1}' height='14' fill='{col}' fill-opacity='{}'/><text x='{:.1}' y='{:.1}'>{arm} {:.3e}</text>\n", if ai == 0 { 1.0 } else { 0.55 }, bx0 + bw + 6.0, y + 11.0, v);
            }
        }
    }
    s += "<text x='70' y='500' fill='#444'>Source: raw/*.json (jv_cover --exp dlp); aggregated by examples/jv_cover_weakclass_report.rs. Counts, not wall time.</text>\n</svg>\n";
    s
}
