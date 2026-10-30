//! Summarise and fit the `gaudry_cubic_bench` JSON rows (Lead 3, field degree n = 3).
//!
//! Reads only recorded fields of `GaudryReport` / `RhoReport3`; no re-run. Prints markdown tables and
//! writes `results.json` (per-row records and fitted exponents) for the plotting script.
//!
//! usage: fit_n3 <out.json> <arm-label=file.json>...
//! An arm label is only a tag for the group of files; rows are further split by LA mode:
//! `dense` (la_mode dense, no large primes), `wiedemann` (sparse LA, full base), `2lp`
//! (sparse LA, small base + large primes), or the solver (`mitm`, `pair`) for the other arms.

use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::{json, Value};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
struct Row {
    arm: String,
    p: u64,
    seed_idx: usize,
    log2n: f64,
    residuals: f64,
    decomp: f64,
    indep: f64,
    rate: f64,
    rel_phase: f64,
    la_phase: f64,
    total: f64,
    s: f64,
    rho_s: f64,
    rho_steps: f64,
    sqrt_n: f64,
    wall_s: f64,
    ok: bool,
}

fn f(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap_or_else(|| panic!("field {k}"))
}

fn load(arm_tag: &str, path: &str, out: &mut Vec<Row>) {
    let d: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    let rows = d.as_array().unwrap();
    // The bench writes (p, seed) in order, 3 rows per (p, seed) for --protocol-la.
    let mut idx: BTreeMap<u64, usize> = BTreeMap::new();
    for (i, r) in rows.iter().enumerate() {
        let g = &r["gaudry"];
        let rho = &r["rho"];
        let p = g["p"].as_u64().unwrap();
        let la_mode = g["la_mode"].as_str().unwrap();
        let small = g["small_base"].as_u64().unwrap();
        let base = g["base"].as_u64().unwrap();
        let solver = g["solver"].as_str().unwrap();
        let arm = if arm_tag == "P" {
            if la_mode == "dense" {
                "dense"
            } else if small > 0 && small < base {
                "2lp"
            } else {
                "wiedemann"
            }
        } else {
            arm_tag
        };
        let _ = solver;
        let fpa = f(g, "fp_muls_per_add");
        let rel = f(g, "group_ops") + (f(g, "oracle_fp_muls") + f(g, "precompute_fp_muls")) / fpa;
        let la = f(g, "la_ops") * f(g, "la_fp_equiv") / fpa;
        let total = f(g, "total_ops");
        assert!((rel + la - total).abs() / total < 2e-3, "phase split {rel}+{la} != {total} (row {i})");
        let n = f(g, "n");
        let k = idx.entry(p).or_insert(0);
        // seed index within this (arm, p): count rows of this arm at this p so far
        let seed_idx = out.iter().filter(|x| x.arm == arm && x.p == p).count();
        let _ = k;
        out.push(Row {
            arm: arm.to_string(),
            p,
            seed_idx,
            log2n: n.log2(),
            residuals: f(g, "residuals"),
            decomp: f(g, "decompositions"),
            indep: f(g, "relations_independent"),
            rate: f(g, "decomposition_rate"),
            rel_phase: rel,
            la_phase: la,
            total,
            s: f(g, "s"),
            rho_s: f(rho, "s"),
            rho_steps: f(rho, "steps"),
            sqrt_n: n.sqrt(),
            wall_s: f(g, "wall_ms") / 1000.0,
            ok: g["correct"].as_bool().unwrap_or(false) && rho["correct"].as_bool().unwrap_or(false),
        });
    }
}

fn ols(x: &[f64], y: &[f64]) -> (f64, f64, f64) {
    let n = x.len() as f64;
    let mx = x.iter().sum::<f64>() / n;
    let my = y.iter().sum::<f64>() / n;
    let sxx: f64 = x.iter().map(|a| (a - mx).powi(2)).sum();
    let sxy: f64 = x.iter().zip(y).map(|(a, b)| (a - mx) * (b - my)).sum();
    let b = sxy / sxx;
    let a = my - b * mx;
    let rss: f64 = x.iter().zip(y).map(|(u, v)| (v - a - b * u).powi(2)).sum();
    let se = if x.len() > 2 { (rss / (n - 2.0) / sxx).sqrt() } else { f64::NAN };
    (a, b, se)
}

/// slope, intercept, OLS standard error, bootstrap 95% interval of the slope (rows resampled).
fn fit(x: &[f64], y: &[f64]) -> (f64, f64, f64, f64, f64) {
    let (a, b, se) = ols(x, y);
    let mut rng = StdRng::seed_from_u64(20261030);
    let mut bs = Vec::new();
    for _ in 0..4000 {
        let idx: Vec<usize> = (0..x.len()).map(|_| rng.gen_range(0..x.len())).collect();
        let xs: Vec<f64> = idx.iter().map(|&i| x[i]).collect();
        let ys: Vec<f64> = idx.iter().map(|&i| y[i]).collect();
        let mn = xs.iter().cloned().fold(f64::MAX, f64::min);
        let mxv = xs.iter().cloned().fold(f64::MIN, f64::max);
        if mxv - mn < 1e-9 {
            continue;
        }
        bs.push(ols(&xs, &ys).1);
    }
    bs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let lo = bs[(bs.len() as f64 * 0.025) as usize];
    let hi = bs[(bs.len() as f64 * 0.975) as usize];
    (b, a, se, lo, hi)
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out_path = &args[0];
    let mut rows: Vec<Row> = Vec::new();
    for a in &args[1..] {
        let (tag, path) = a.split_once('=').unwrap();
        load(tag, path, &mut rows);
    }
    // Rho is one stochastic walk per instance (S_rho varies 1.2-1.8 with a ~15% standard error per curve), and its
    // S does not depend on n, so ratios use S_rho pooled over the distinct instances (as RESEARCH_RESIDUAL_WALKS 11.19).
    let mut inst: Vec<(u64, u64)> = rows.iter().map(|r| (r.p, r.rho_steps as u64)).collect();
    inst.sort();
    inst.dedup();
    let pooled: Vec<f64> = inst.iter().map(|&(p, st)| rows.iter().find(|r| r.p == p && r.rho_steps as u64 == st).unwrap().rho_s).collect();
    let s_rho = mean(&pooled);
    let s_rho_se = (pooled.iter().map(|v| (v - s_rho).powi(2)).sum::<f64>() / (pooled.len() as f64 - 1.0)).sqrt() / (pooled.len() as f64).sqrt();
    println!("pooled S_rho = {:.3} +- {:.3} over {} distinct instances", s_rho, s_rho_se, pooled.len());
    let censored: Vec<String> = rows.iter().filter(|r| !r.ok).map(|r| format!("{}@p={}", r.arm, r.p)).collect();
    println!("censored/unverified rows excluded from fits: {:?}", censored);
    for r in rows.iter_mut() { r.rho_s = s_rho; }
    let all_rows = rows.clone();
    rows.retain(|r| r.ok);
    let mut arms: Vec<String> = rows.iter().map(|r| r.arm.clone()).collect();
    arms.sort();
    arms.dedup();
    let mut res = json!({"rows": [], "arms": {}});

    println!("| arm | p | log2 n | instances | residuals | decomp rate | ops/residual | ops/decomp | indep. relations | rel. phase | LA phase | total ops | S | S_rho | S/S_rho | all verified |");
    println!("|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|");
    for arm in &arms {
        let mut ps: Vec<u64> = rows.iter().filter(|r| &r.arm == arm).map(|r| r.p).collect();
        ps.sort();
        ps.dedup();
        for p in ps {
            let g: Vec<&Row> = rows.iter().filter(|r| &r.arm == arm && r.p == p).collect();
            let m = |h: &dyn Fn(&Row) -> f64| mean(&g.iter().map(|r| h(r)).collect::<Vec<_>>());
            println!(
                "| {} | {} | {:.1} | {} | {:.0} | {:.3} | {:.3e} | {:.3e} | {:.0} | {:.3e} | {:.3e} | {:.3e} | {:.0} | {:.2} | {:.0} | {} |",
                arm, p, g[0].log2n, g.len(),
                m(&|r| r.residuals), m(&|r| r.rate),
                m(&|r| r.rel_phase / r.residuals), m(&|r| r.rel_phase / r.decomp.max(1.0)),
                m(&|r| r.indep), m(&|r| r.rel_phase), m(&|r| r.la_phase), m(&|r| r.total),
                m(&|r| r.s), m(&|r| r.rho_s), m(&|r| r.s / r.rho_s),
                if g.iter().all(|r| r.ok) { "yes" } else { "NO" }
            );
        }
    }
    println!();
    println!("| arm | quantity | exponent in n (OLS ± se) | bootstrap 95% | rows |");
    println!("|---|---|---:|---|---:|");
    for arm in &arms {
        let g: Vec<&Row> = rows.iter().filter(|r| &r.arm == arm).collect();
        let x: Vec<f64> = g.iter().map(|r| r.log2n).collect();
        let qs: Vec<(&str, Vec<f64>)> = vec![
            ("total ops", g.iter().map(|r| r.total.log2()).collect()),
            ("relation phase", g.iter().map(|r| r.rel_phase.log2()).collect()),
            ("LA phase", g.iter().map(|r| r.la_phase.max(1.0).log2()).collect()),
            ("residuals", g.iter().map(|r| r.residuals.log2()).collect()),
            ("ops/residual", g.iter().map(|r| (r.rel_phase / r.residuals).log2()).collect()),
            ("rho cost S_rho*sqrt(n)", g.iter().map(|r| (r.rho_s * r.sqrt_n).log2()).collect()),
            ("S/S_rho", g.iter().map(|r| (r.s / r.rho_s).log2()).collect()),
        ];
        for (name, y) in qs {
            let (b, a, se, lo, hi) = fit(&x, &y);
            println!("| {} | {} | {:.3} ± {:.3} | [{:.3}, {:.3}] | {} |", arm, name, b, se, lo, hi, g.len());
            res["arms"][arm][name] = json!({"slope": b, "intercept": a, "se": se, "lo": lo, "hi": hi});
        }
    }
    println!();
    // crossover with rho from the fitted S/S_rho line
    println!("| arm | fitted S/S_rho at largest p | slope | extrapolated crossover log2 n* | distance beyond measured (bits) |");
    println!("|---|---:|---:|---:|---:|");
    for arm in &arms {
        let g: Vec<&Row> = rows.iter().filter(|r| &r.arm == arm).collect();
        let x: Vec<f64> = g.iter().map(|r| r.log2n).collect();
        let y: Vec<f64> = g.iter().map(|r| (r.s / r.rho_s).log2()).collect();
        let (b, a, _se, lo, hi) = fit(&x, &y);
        let xmax = x.iter().cloned().fold(f64::MIN, f64::max);
        let atmax = 2f64.powf(a + b * xmax);
        let cross = if b < 0.0 { -a / b } else { f64::INFINITY };
        let (clo, chi) = {
            // crossover under the bootstrap slope bounds, holding the point at xmax fixed
            let yv = a + b * xmax;
            let c = |s: f64| if s < 0.0 { xmax - yv / s } else { f64::INFINITY };
            (c(hi), c(lo))
        };
        println!("| {} | {:.0} | {:.4} | {:.0} (slope-CI {:.0} .. {:.0}) | {:.0} |", arm, atmax, b, cross, clo, chi, cross - xmax);
        res["arms"][arm]["crossover"] = json!({"ratio_at_max": atmax, "slope": b, "log2n_star": cross, "lo": clo, "hi": chi, "xmax": xmax});
    }
    // Two-phase projection: total(n) = A n^er + B n^el with each phase fitted on its own, divided by the
    // rho cost S_rho * sqrt(n). The single-line extrapolation above hides that the LA phase usually has the
    // larger exponent; this finds the minimum of the ratio and where (if anywhere) it reaches 1.
    println!();
    println!("| arm | er (relation) | el (LA) | min S/S_rho on log2 n in [24,400] | at log2 n | first log2 n with S/S_rho < 1 | theory-exponent crossover log2 n |");
    println!("|---|---:|---:|---:|---:|---:|---:|");
    for arm in &arms {
        let g: Vec<&Row> = rows.iter().filter(|r| &r.arm == arm).collect();
        let x: Vec<f64> = g.iter().map(|r| r.log2n).collect();
        let yr: Vec<f64> = g.iter().map(|r| r.rel_phase.log2()).collect();
        let yl: Vec<f64> = g.iter().map(|r| r.la_phase.max(1.0).log2()).collect();
        let (er, ar, _, _, _) = fit(&x, &yr);
        let (el, al, _, _, _) = fit(&x, &yl);
        let (mut best, mut at, mut first) = (f64::MAX, 0.0, f64::NAN);
        let mut l = 24.0;
        while l <= 400.0 {
            let tot = 2f64.powf(ar + er * l) + 2f64.powf(al + el * l);
            let ratio = tot / (s_rho * 2f64.powf(l / 2.0));
            if ratio < best { best = ratio; at = l; }
            if ratio < 1.0 && first.is_nan() { first = l; }
            l += 0.25;
        }
        // theory crossover: ratio at largest measured size, then slope (theory exponent - 1/2)
        let xmax = x.iter().cloned().fold(f64::MIN, f64::max);
        let r0 = (2f64.powf(ar + er * xmax) + 2f64.powf(al + el * xmax)) / (s_rho * 2f64.powf(xmax / 2.0));
        let th = match arm.as_str() { "2lp" => Some(4.0 / 9.0), _ => None };
        let thc = th.map(|e| xmax + r0.log2() / (0.5 - e));
        println!("| {} | {:.3} | {:.3} | {:.1} | {:.1} | {} | {} |", arm, er, el, best, at,
            if first.is_nan() { "none".to_string() } else { format!("{:.0}", first) },
            thc.map(|v| format!("{:.0}", v)).unwrap_or("n/a".into()));
        res["arms"][arm]["projection"] = json!({"er": er, "el": el, "min_ratio": best, "at_log2n": at, "first_below_1": first, "theory_cross": thc});
    }
    // section 5a price band for unpriced native work (hash probes): IC ~ (1 + max large primes) probes per
    // residual, rho ~ 1 distinguished-point test per step; priced at 0, 0.1, 1 add each.
    println!();
    println!("| arm | p | S/S_rho at 0 | at 0.1 | at 1 GAE per probe |");
    println!("|---|---:|---:|---:|---:|");
    for arm in &arms {
        let mut ps: Vec<u64> = rows.iter().filter(|r| &r.arm == arm).map(|r| r.p).collect();
        ps.sort();
        ps.dedup();
        let p = *ps.last().unwrap();
        let g: Vec<&Row> = rows.iter().filter(|r| &r.arm == arm && r.p == p).collect();
        let probes_ic = |r: &Row| r.residuals * 3.0;
        let band: Vec<f64> = [0.0, 0.1, 1.0]
            .iter()
            .map(|c| mean(&g.iter().map(|r| (r.total + c * probes_ic(r)) / (r.rho_s * r.sqrt_n + c * r.rho_steps)).collect::<Vec<_>>()))
            .collect();
        println!("| {} | {} | {:.1} | {:.1} | {:.1} |", arm, p, band[0], band[1], band[2]);
        res["arms"][arm]["price_band_largest_p"] = json!(band);
    }
    for r in &all_rows {
        res["rows"].as_array_mut().unwrap().push(json!({
            "arm": r.arm, "p": r.p, "seed_idx": r.seed_idx, "log2n": r.log2n, "residuals": r.residuals,
            "decomp": r.decomp, "indep": r.indep, "rate": r.rate, "rel_phase": r.rel_phase, "la_phase": r.la_phase,
            "total": r.total, "s": r.s, "rho_s": r.rho_s, "rho_steps": r.rho_steps, "wall_s": r.wall_s, "verified": r.ok}));
    }
    std::fs::write(out_path, serde_json::to_string_pretty(&res).unwrap()).unwrap();
}
