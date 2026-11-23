//! Aggregate the paired A / B / B0 / A2 `jv_cover --exp dlp` records of
//! `research/ic_cover_weakclass_20261023` into `summary.json`, `table.md` and `s_over_rho.svg`.
//!
//! ```text
//! cargo run --release --example jv_cover_weakclass_report -- research/ic_cover_weakclass_20261023/raw research/ic_cover_weakclass_20261023
//! ```
//!
//! Files in `raw/` are named `p{P}_s{SEED}_{A|B|B0|A2}.json`, each a one-report array written by
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
fn minmax(x: &[f64]) -> (f64, f64) {
    (
        x.iter().cloned().fold(f64::INFINITY, f64::min),
        x.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
    )
}
/// Least squares of ln y on ln x: slope and its standard error.
fn slope(pts: &[(f64, f64)]) -> (f64, f64) {
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
    let rss: f64 = xs
        .iter()
        .zip(&ys)
        .map(|(x, y)| (y - a - b * x).powi(2))
        .sum();
    (b, (rss / (n - 2.0) / sxx).sqrt())
}

fn load(dir: &str, p: u64, s: u64, arm: &str) -> Option<Value> {
    let t = fs::read_to_string(format!("{dir}/p{p}_s{s}_{arm}.json")).ok()?;
    let v: Value = serde_json::from_str(&t).ok()?;
    v.as_array()?.first().cloned()
}

fn arm_json(r: &Value, rho_muls: f64, steps_per_op: f64) -> Value {
    let tot = u(r, "total_muls") as f64;
    let band: Vec<f64> = [0.0, 0.1, 1.0]
        .iter()
        .map(|w| tot / (rho_muls * (1.0 + w * 2.0 * steps_per_op)))
        .collect();
    json!({"total_muls": tot, "S": f(r, "s"), "S_over_rho": tot / rho_muls, "S_over_rho_rho_band_w0_w01_w1": band,
        "stop_mode": r["stop_mode"], "residuals": u(r, "residuals"), "relations": u(r, "relations"),
        "decompositions": u(r, "decompositions"), "digest": u(r, "relation_digest"),
        "solved": r["solved"], "correct": r["correct"], "mismatches": u(r, "mismatches"),
        "f4_muls": u(r, "f4_muls"), "f4_muls_per_test_sum": u(r, "f4_muls_per_test_sum"),
        "f4_native_ops": u(r, "f4_native_ops"), "f4_stair_ops": u(r, "f4_stair_ops"),
        "hilbert_applied": u(r, "hilbert_applied"), "hilbert_fallbacks": u(r, "hilbert_fallbacks"),
        "hilbert_check_muls": u(r, "hilbert_check_muls"),
        "lin_muls": u(r, "lin_muls"), "weil_muls": u(r, "weil_muls"), "post_muls": u(r, "post_muls"),
        "verify_muls": u(r, "verify_muls"), "stream_muls": u(r, "stream_muls"),
        "setup_muls": u(r, "setup_muls"), "base_muls": u(r, "base_muls"), "la_muls": u(r, "la_muls"),
        "c_cov": f(r, "c_cov"), "predicted_S_over_rho_formula": f(r, "predicted_s_over_rho"),
        "wall_s": f(r, "wall_ms") / 1e3})
}

/// Compare a candidate to the baseline in one cell; status is pass, unresolved (only the native
/// counter fails) or fail.
fn compare(ra: &Value, rb: &Value) -> Value {
    let (ta, tb) = (u(ra, "total_muls") as f64, u(rb, "total_muls") as f64);
    let (na, nb) = (u(ra, "f4_native_ops") as f64, u(rb, "f4_native_ops") as f64);
    let eq = u(rb, "residuals") == u(ra, "residuals")
        && u(rb, "relations") == u(ra, "relations")
        && u(rb, "decompositions") == u(ra, "decompositions")
        && u(rb, "relation_digest") == u(ra, "relation_digest");
    let correct = ra["correct"] == json!(true)
        && rb["correct"] == json!(true)
        && ra["solved"] == json!(true)
        && rb["solved"] == json!(true);
    let mism = u(rb, "mismatches") == 0 && u(ra, "mismatches") == 0;
    // the global counter holds the check and fallback work; it exceeds the per-report sum by the
    // work no report claims (0.3 % in the baseline too), never falls short of it
    let (g, sm) = (
        u(rb, "f4_muls") as f64,
        u(rb, "f4_muls_per_test_sum") as f64,
    );
    let counter = g >= sm && (g - sm) / g <= 0.01;
    let lower = tb < ta;
    let native_ok = nb <= na;
    let wstar = if native_ok {
        f64::INFINITY
    } else {
        (ta - tb) / (nb - na)
    };
    let mut why = Vec::new();
    if !eq {
        why.push("relations differ");
    }
    if !correct {
        why.push("not solved/correct");
    }
    if !mism {
        why.push("oracle mismatch");
    }
    if !counter {
        why.push("f4 counter inconsistent");
    }
    if !lower {
        why.push("S not lower");
    }
    let status = if !why.is_empty() {
        "fail"
    } else if !native_ok {
        "unresolved"
    } else {
        "pass"
    };
    json!({"status": status, "why": why, "relations_equal": eq, "f4_global_over_per_test_sum": g / sm, "ratio_S_A_over_S_B": ta / tb,
           "native_B_over_A": nb / na, "mul_saving": ta - tb, "break_even_w_muls_per_native_op": wstar,
           "break_even_w_GAE": wstar / f(ra, "c_add_e"), "wall_B_over_A": f(rb, "wall_ms") / f(ra, "wall_ms")})
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
    // rho per p, pooled over the arm-A runs of that p; p without rho runs use the pool over all p.
    // Each entry is (S, steps per group op, runs). Steps are normalised per run by that run's own
    // sqrt(l) before pooling, so a pool over runs of different p stays a dimensionless ratio.
    let mut rho_by_p: BTreeMap<u64, Vec<(f64, f64, f64)>> = BTreeMap::new();
    let mut all_rho: Vec<(f64, f64, f64)> = Vec::new();
    for &(p, s) in cells.keys() {
        if let Some(r) = load(raw, p, s, "A") {
            let k = r["rho"].as_array().map_or(0, |x| x.len()) as f64;
            if k > 0.0 {
                let s_mean = f(&r, "rho_s_mean");
                let ops = s_mean * (u(&r, "l") as f64).sqrt();
                let per_op = if ops > 0.0 {
                    f(&r, "rho_steps_mean") / ops
                } else {
                    1.0
                };
                let e = (s_mean, per_op, k);
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
    let pooled_all = if all_rho.is_empty() {
        (1.3, 1.0, 0.0)
    } else {
        pool(&all_rho)
    };
    let mut rows: Vec<Value> = Vec::new();
    let mut md = String::from("| p | seed | residuals | relations | digest | S_A | S_B | S_A/S_B | S/rho A | S/rho B | native B/A | F4 muls A | F4 muls B | check muls | fallbacks | B status | B0 status (S_A/S_B0, native B0/A) | A/A |\n|--:|--:|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|:--|:--|\n");
    let mut per_p: BTreeMap<u64, Vec<(f64, f64, f64, f64, f64, f64)>> = BTreeMap::new(); // S/rho A, B, ratio, formula A, formula B, native ratio
    let (mut n_pass, mut n_unres, mut n_fail, mut aa_bad) = (0, 0, 0, 0);
    for &(p, s) in cells.keys() {
        let Some(ra) = load(raw, p, s, "A") else {
            continue;
        };
        let (rb, rb0, ra2) = (
            load(raw, p, s, "B"),
            load(raw, p, s, "B0"),
            load(raw, p, s, "A2"),
        );
        let rho = rho_by_p.get(&p).map(|v| pool(v)).unwrap_or(pooled_all);
        let rho_src = if rho_by_p.contains_key(&p) {
            "measured at this p"
        } else {
            "pooled over all measured p"
        };
        let sqrt_l = (u(&ra, "l") as f64).sqrt();
        let ce = f(&ra, "c_add_e");
        let rho_muls = rho.0 * sqrt_l * ce;
        let steps_per_op = rho.1;
        let mut cell = json!({"p": p, "seed": s, "l_bits": f(&ra, "bits"), "rho_source": rho_src, "rho_S": rho.0,
            "rho_runs_pooled": rho.2, "rho_steps_per_group_op": steps_per_op, "A": arm_json(&ra, rho_muls, steps_per_op)});
        let tot_a = u(&ra, "total_muls") as f64;
        let mut line = format!(
            "| {p} | {s} | {} | {} | {} |",
            u(&ra, "residuals"),
            u(&ra, "relations"),
            if rb.is_some() { "" } else { "A only" }
        );
        if let Some(rb) = &rb {
            let c = compare(&ra, rb);
            cell["B"] = arm_json(rb, rho_muls, steps_per_op);
            cell["B_vs_A"] = c.clone();
            match c["status"].as_str().unwrap() {
                "pass" => n_pass += 1,
                "unresolved" => n_unres += 1,
                _ => n_fail += 1,
            }
            let tot_b = u(rb, "total_muls") as f64;
            per_p.entry(p).or_default().push((
                tot_a / rho_muls,
                tot_b / rho_muls,
                tot_a / tot_b,
                f(&ra, "predicted_s_over_rho"),
                f(rb, "predicted_s_over_rho"),
                c["native_B_over_A"].as_f64().unwrap(),
            ));
            line = format!("| {p} | {s} | {} | {} | {} | {:.4e} | {:.4e} | {:.4} | {:.1} | {:.1} | {:.3} | {} | {} | {} | {} | {} |",
                u(&ra, "residuals"), u(&ra, "relations"), if c["relations_equal"] == json!(true) { "equal" } else { "DIFFERENT" },
                f(&ra, "s"), f(rb, "s"), c["ratio_S_A_over_S_B"].as_f64().unwrap(), tot_a / rho_muls, tot_b / rho_muls,
                c["native_B_over_A"].as_f64().unwrap(), u(&ra, "f4_muls"), u(rb, "f4_muls"), u(rb, "hilbert_check_muls"),
                u(rb, "hilbert_fallbacks"), c["status"].as_str().unwrap());
        } else {
            line += &format!(
                " {:.4e} | - | - | {:.1} | - | - | {} | - | - | - | - |",
                f(&ra, "s"),
                tot_a / rho_muls,
                u(&ra, "f4_muls")
            );
        }
        if let Some(r0) = &rb0 {
            let c = compare(&ra, r0);
            cell["B0"] = arm_json(r0, rho_muls, steps_per_op);
            cell["B0_vs_A"] = c.clone();
            line += &format!(
                " {} ({:.3}, {:.2}) |",
                c["status"].as_str().unwrap(),
                c["ratio_S_A_over_S_B"].as_f64().unwrap(),
                c["native_B_over_A"].as_f64().unwrap()
            );
        } else {
            line += " - |";
        }
        if let Some(r2) = &ra2 {
            let same = u(r2, "total_muls") == u(&ra, "total_muls")
                && u(r2, "relation_digest") == u(&ra, "relation_digest")
                && u(r2, "residuals") == u(&ra, "residuals");
            let ratio_aa = f(&ra, "s") / f(r2, "s");
            cell["A2"] = json!({"total_muls": u(r2, "total_muls"), "S": f(r2, "s"), "ratio_A_over_A2": ratio_aa, "identical": same});
            if !same {
                aa_bad += 1;
            }
            line += &format!(" {ratio_aa:.3} |\n");
        } else {
            line += " - |\n";
        }
        md += &line;
        rows.push(cell);
    }
    let mut agg = Vec::new();
    let (mut fit_a, mut fit_b) = (Vec::new(), Vec::new());
    md += "\n| p | n seeds | S/rho A mean (sd) [min, max] | S/rho B mean (sd) [min, max] | S_A/S_B mean [min, max] | native B/A mean [min, max] | formula S/rho A | formula S/rho B |\n|--:|--:|:--|:--|:--|:--|--:|--:|\n";
    for (p, v) in &per_p {
        let col = |i: usize| -> Vec<f64> {
            v.iter()
                .map(|x| [x.0, x.1, x.2, x.3, x.4, x.5][i])
                .collect()
        };
        let (ca, cb, cr, pa, pb, cn) = (col(0), col(1), col(2), col(3), col(4), col(5));
        let (a0, a1) = minmax(&ca);
        let (b0, b1) = minmax(&cb);
        let (r0, r1) = minmax(&cr);
        let (n0, n1) = minmax(&cn);
        md += &format!("| {p} | {} | {:.1} ({:.1}) [{:.1}, {:.1}] | {:.1} ({:.1}) [{:.1}, {:.1}] | {:.3} [{:.3}, {:.3}] | {:.3} [{:.3}, {:.3}] | {:.1} | {:.1} |\n",
            v.len(), mean(&ca), sd(&ca), a0, a1, mean(&cb), sd(&cb), b0, b1, mean(&cr), r0, r1, mean(&cn), n0, n1, mean(&pa), mean(&pb));
        agg.push(json!({"p": p, "n": v.len(),
            "S_over_rho_A": {"mean": mean(&ca), "sd": sd(&ca), "min": a0, "max": a1},
            "S_over_rho_B": {"mean": mean(&cb), "sd": sd(&cb), "min": b0, "max": b1},
            "ratio_S_A_over_S_B": {"mean": mean(&cr), "min": r0, "max": r1},
            "native_B_over_A": {"mean": mean(&cn), "min": n0, "max": n1},
            "formula_A": mean(&pa), "formula_B": mean(&pb)}));
        fit_a.push((*p as f64, mean(&ca)));
        fit_b.push((*p as f64, mean(&cb)));
    }
    let (sa, ea) = slope(&fit_a);
    let (sb, eb) = slope(&fit_b);
    md += &format!("\nlog-log slope of mean S/rho against p over {} sizes: A {:.2} +- {:.2}, B {:.2} +- {:.2}\n", fit_a.len(), sa, ea, sb, eb);
    md += &format!("\nB against A, cells: pass {n_pass}, unresolved (native counter higher) {n_unres}, fail {n_fail}; A/A cells not identical: {aa_bad}\n");
    fs::write(
        format!("{out}/summary.json"),
        serde_json::to_string_pretty(&json!({"cells": rows, "per_p": agg, "slope_A": [sa, ea], "slope_B": [sb, eb],
            "B_cells": {"pass": n_pass, "unresolved": n_unres, "fail": n_fail}, "aa_not_identical": aa_bad})).unwrap(),
    )
    .unwrap();
    fs::write(format!("{out}/table.md"), &md).unwrap();
    fs::write(format!("{out}/s_over_rho.svg"), svg(&per_p, &rows)).unwrap();
    print!("{md}");
}

fn svg(per_p: &BTreeMap<u64, Vec<(f64, f64, f64, f64, f64, f64)>>, rows: &[Value]) -> String {
    let (w, h) = (1120.0, 520.0);
    let (x0, x1, y0, y1) = (70.0, 560.0, 60.0, 440.0);
    let (pmin, pmax) = (40.0f64, 700.0f64);
    let (vmin, vmax) = (1.0f64, 2000.0f64);
    let xm = |p: f64| x0 + (p.ln() - pmin.ln()) / (pmax.ln() - pmin.ln()) * (x1 - x0);
    let ym = |v: f64| y1 - (v.ln() - vmin.ln()) / (vmax.ln() - vmin.ln()) * (y1 - y0);
    let mut s = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' viewBox='0 0 {w} {h}' font-family='sans-serif' font-size='12'>\n<rect width='100%' height='100%' fill='white'/>\n");
    s += "<text x='70' y='24' font-size='15' font-weight='bold'>Cover IC, genus 3: S/rho per seed against p (lower-bound comparison)</text>\n";
    s += "<text x='70' y='42' fill='#444'>A = full F4 + sparse FGLM (baseline); B = A + Hilbert early stop (hilbert:16); counted F_p multiplications, rho_e reference</text>\n";
    for v in [1.0, 10.0, 100.0, 1000.0] {
        s += &format!("<line x1='{x0}' y1='{:.1}' x2='{x1}' y2='{:.1}' stroke='#ddd'/><text x='{:.1}' y='{:.1}' text-anchor='end'>{v}</text>\n", ym(v), ym(v), x0 - 6.0, ym(v) + 4.0);
    }
    for p in [53.0, 101.0, 251.0, 503.0] {
        s += &format!("<line x1='{:.1}' y1='{y0}' x2='{:.1}' y2='{y1}' stroke='#eee'/><text x='{:.1}' y='{:.1}' text-anchor='middle'>{p}</text>\n", xm(p), xm(p), xm(p), y1 + 16.0);
    }
    s += &format!(
        "<rect x='{x0}' y='{y0}' width='{}' height='{}' fill='none' stroke='#888'/>\n",
        x1 - x0,
        y1 - y0
    );
    s += &format!("<line x1='{x0}' y1='{:.1}' x2='{x1}' y2='{:.1}' stroke='#c00' stroke-dasharray='5 3'/><text x='{:.1}' y='{:.1}' fill='#c00'>rho (S/rho = 1)</text>\n", ym(1.0), ym(1.0), x1 - 110.0, ym(1.0) - 5.0);
    s += &format!("<text x='{:.1}' y='{:.1}' text-anchor='middle'>p</text><text x='18' y='{:.1}' transform='rotate(-90 18 {:.1})' text-anchor='middle'>S / rho (counted F_p multiplications)</text>\n", (x0 + x1) / 2.0, y1 + 36.0, (y0 + y1) / 2.0, (y0 + y1) / 2.0);
    let (mut ma, mut mb) = (Vec::new(), Vec::new());
    for (p, v) in per_p {
        let n = v.len() as f64;
        for (i, x) in v.iter().enumerate() {
            let dx = (i as f64 - (n - 1.0) / 2.0) * 3.0;
            s += &format!(
                "<circle cx='{:.1}' cy='{:.1}' r='3' fill='#1f5fa8' fill-opacity='0.75'/>\n",
                xm(*p as f64) - 7.0 + dx,
                ym(x.0)
            );
            s += &format!("<rect x='{:.1}' y='{:.1}' width='6' height='6' fill='#d97a00' fill-opacity='0.8'/>\n", xm(*p as f64) + 4.0 + dx, ym(x.1) - 3.0);
        }
        ma.push((
            xm(*p as f64) - 7.0,
            ym(v.iter().map(|x| x.0).sum::<f64>() / n),
        ));
        mb.push((
            xm(*p as f64) + 7.0,
            ym(v.iter().map(|x| x.1).sum::<f64>() / n),
        ));
    }
    for (m, c) in [(&ma, "#1f5fa8"), (&mb, "#d97a00")] {
        let pts: Vec<String> = m.iter().map(|q| format!("{:.1},{:.1}", q.0, q.1)).collect();
        s += &format!(
            "<polyline points='{}' fill='none' stroke='{c}' stroke-width='1.5'/>\n",
            pts.join(" ")
        );
    }
    s += &format!("<circle cx='{}' cy='{}' r='3.5' fill='#1f5fa8'/><text x='{}' y='{}'>A baseline (per seed; line = mean)</text>\n", x0 + 14.0, y0 + 14.0, x0 + 24.0, y0 + 18.0);
    s += &format!("<rect x='{}' y='{}' width='7' height='7' fill='#d97a00'/><text x='{}' y='{}'>B Hilbert stop (per seed; line = mean)</text>\n", x0 + 10.0, y0 + 26.0, x0 + 24.0, y0 + 34.0);
    // right panel: per-residual counted multiplications and native operations, A and B, largest p with both
    if let Some(c) = rows.iter().rev().find(|c| c.get("B").is_some()) {
        let bx0 = 650.0;
        let res = c["A"]["residuals"].as_f64().unwrap_or(1.0);
        let stages = [
            ("F4 muls", "f4_muls", "#1f5fa8"),
            ("linear muls", "lin_muls", "#6a3d9a"),
            ("Weil muls", "weil_muls", "#2a9d8f"),
            ("post muls", "post_muls", "#999999"),
            ("verify muls", "verify_muls", "#e0b000"),
            (
                "native ops (F4 driver, unpriced)",
                "f4_native_ops",
                "#c0392b",
            ),
        ];
        let maxv = stages
            .iter()
            .map(|(_, k, _)| c["A"][*k].as_f64().unwrap_or(0.0) / res)
            .fold(0.0f64, f64::max)
            * 1.15;
        s += &format!("<text x='{bx0}' y='60' font-weight='bold'>Per residual, p = {}, seed {} (counts)</text>\n", c["p"], c["seed"]);
        for (si, (name, key, col)) in stages.iter().enumerate() {
            let yy = 78.0 + si as f64 * 56.0;
            s += &format!("<text x='{bx0}' y='{:.1}'>{name}</text>\n", yy + 10.0);
            for (ai, arm) in ["A", "B"].iter().enumerate() {
                let v = c[*arm][*key].as_f64().unwrap_or(0.0) / res;
                let bw = v / maxv * 300.0;
                let y = yy + 15.0 + ai as f64 * 16.0;
                s += &format!("<rect x='{bx0}' y='{y:.1}' width='{bw:.1}' height='13' fill='{col}' fill-opacity='{}'/><text x='{:.1}' y='{:.1}'>{arm} {:.3e}</text>\n", if ai == 0 { 1.0 } else { 0.55 }, bx0 + bw + 6.0, y + 11.0, v);
            }
        }
    }
    s += "<text x='70' y='500' fill='#444'>Source: raw/*.json (jv_cover --exp dlp); aggregated by examples/jv_cover_weakclass_report.rs. Counts, not wall time. Native ops are a lower bound.</text>\n</svg>\n";
    s
}
