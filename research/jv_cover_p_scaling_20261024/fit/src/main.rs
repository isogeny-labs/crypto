//! Fit of ln(S/rho) against ln p for research/jv_cover_p_scaling_20261024.
//! Usage: jv_p_scaling_fit <results dir>   (writes fit.json, fit.txt, scaling.svg there)
use serde_json::{json, Value};
use std::fs;

struct Pt {
    p: f64,
    seed: u64,
    s_muls: f64,   // total_muls
    sqrt_l: f64,
    c_e: f64,
    own_rho: Option<f64>, // mean rho S of own runs
    steps_mean: Option<f64>,
    ok: bool,
}

fn ols(x: &[f64], y: &[f64]) -> (f64, f64, f64, f64) {
    // intercept a, slope b, se(b), residual df
    let n = x.len() as f64;
    let mx = x.iter().sum::<f64>() / n;
    let my = y.iter().sum::<f64>() / n;
    let sxx: f64 = x.iter().map(|v| (v - mx).powi(2)).sum();
    let sxy: f64 = x.iter().zip(y).map(|(a, b)| (a - mx) * (b - my)).sum();
    let b = sxy / sxx;
    let a = my - b * mx;
    let rss: f64 = x.iter().zip(y).map(|(xi, yi)| (yi - a - b * xi).powi(2)).sum();
    let df = n - 2.0;
    let se = if df > 0.0 { (rss / df / sxx).sqrt() } else { f64::NAN };
    (a, b, se, df)
}

/// two-sided 97.5% Student t quantile (table + interpolation by 1/df)
fn t975(df: f64) -> f64 {
    let t = [
        (1.0, 12.706), (2.0, 4.303), (3.0, 3.182), (4.0, 2.776), (5.0, 2.571), (6.0, 2.447),
        (7.0, 2.365), (8.0, 2.306), (9.0, 2.262), (10.0, 2.228), (12.0, 2.179), (15.0, 2.131),
        (20.0, 2.086), (25.0, 2.060), (30.0, 2.042), (60.0, 2.000), (1e9, 1.960),
    ];
    for w in t.windows(2) {
        if df >= w[0].0 && df <= w[1].0 {
            let f = (1.0 / df - 1.0 / w[1].0) / (1.0 / w[0].0 - 1.0 / w[1].0);
            return w[1].1 + f * (w[0].1 - w[1].1);
        }
    }
    1.960
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

fn pct(v: &mut Vec<f64>, q: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[((v.len() - 1) as f64 * q).round() as usize]
}

fn fit_pts(pts: &[(f64, f64)]) -> (f64, f64, f64, f64) {
    let x: Vec<f64> = pts.iter().map(|q| q.0.ln()).collect();
    let y: Vec<f64> = pts.iter().map(|q| q.1.ln()).collect();
    ols(&x, &y)
}

fn main() {
    let dir = std::env::args().nth(1).expect("results dir");
    let mut files: Vec<_> = fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            let n = p.file_name().unwrap().to_string_lossy().to_string();
            n.starts_with("dlp_p") && n.ends_with(".json")
        })
        .collect();
    files.sort();
    let mut pts: Vec<Pt> = vec![];
    let mut failures: Vec<String> = vec![];
    let mut rho_all: Vec<(f64, f64)> = vec![]; // (p, S sample)
    for f in &files {
        let v: Value = serde_json::from_str(&fs::read_to_string(f).unwrap()).unwrap();
        for r in v.as_array().unwrap() {
            let p = r["p"].as_f64().unwrap();
            let seed = r["seed"].as_u64().unwrap();
            let solved = r["solved"].as_bool().unwrap();
            let correct = r["correct"].as_bool().unwrap();
            let mism = r["mismatches"].as_u64().unwrap();
            let l = r["l"].as_f64().unwrap();
            let rho = r["rho"].as_array().unwrap();
            let ss: Vec<f64> = rho.iter().map(|x| x["s"].as_f64().unwrap()).collect();
            let st: Vec<f64> = rho.iter().map(|x| x["steps"].as_f64().unwrap()).collect();
            for s in &ss {
                rho_all.push((p, *s));
            }
            let ok = solved && correct && mism == 0;
            if !ok {
                failures.push(format!("p={p} seed={seed}: solved={solved} correct={correct} mismatches={mism}"));
            }
            pts.push(Pt {
                p,
                seed,
                s_muls: r["total_muls"].as_f64().unwrap(),
                sqrt_l: l.sqrt(),
                c_e: r["c_add_e"].as_f64().unwrap(),
                own_rho: (!ss.is_empty()).then(|| ss.iter().sum::<f64>() / ss.len() as f64),
                steps_mean: (!st.is_empty()).then(|| st.iter().sum::<f64>() / st.len() as f64),
                ok,
            });
        }
    }
    // pooled rho
    let rho_ref = rho_all.iter().map(|x| x.1).sum::<f64>() / rho_all.len() as f64;
    let rho_sd = (rho_all.iter().map(|x| (x.1 - rho_ref).powi(2)).sum::<f64>() / (rho_all.len() - 1) as f64).sqrt();
    let rho_se = rho_sd / (rho_all.len() as f64).sqrt();
    // trend of rho S on ln p (per-sample OLS)
    let rx: Vec<f64> = rho_all.iter().map(|x| x.0.ln()).collect();
    let ry: Vec<f64> = rho_all.iter().map(|x| x.1).collect();
    let (_ra, rb, rse, rdf) = ols(&rx, &ry);
    let rb_lo = rb - t975(rdf) * rse;
    let rb_hi = rb + t975(rdf) * rse;
    let mut per_prime_rho: Vec<(f64, f64, usize)> = vec![];
    let mut ps: Vec<f64> = rho_all.iter().map(|x| x.0).collect();
    ps.dedup();
    for &p in &ps {
        let v: Vec<f64> = rho_all.iter().filter(|x| x.0 == p).map(|x| x.1).collect();
        per_prime_rho.push((p, v.iter().sum::<f64>() / v.len() as f64, v.len()));
    }

    let ratio = |q: &Pt, rho: f64| q.s_muls / (rho * q.sqrt_l * q.c_e);
    let good: Vec<&Pt> = pts.iter().filter(|q| q.ok).collect();
    let prim: Vec<(f64, f64)> = good.iter().map(|q| (q.p, ratio(q, rho_ref))).collect();
    let own: Vec<(f64, f64)> = good.iter().filter(|q| q.own_rho.is_some()).map(|q| (q.p, ratio(q, q.own_rho.unwrap()))).collect();

    let (a, b, se, df) = fit_pts(&prim);
    let tq = t975(df);
    let (b_lo, b_hi) = (b - tq * se, b + tq * se);
    let pstar = |a: f64, b: f64| (-a / b).exp();
    // p* interval by delta method on log p* = -a/b (Monte Carlo over normal (a,b) not available: use bootstrap below)
    let mut pr_primes: Vec<f64> = prim.iter().map(|x| x.0).collect();
    pr_primes.dedup();
    let mut rng = Lcg(20261024);
    let mut bs_slope = vec![];
    let mut bs_pstar = vec![];
    let groups: Vec<Vec<(f64, f64)>> = pr_primes.iter().map(|&p| prim.iter().filter(|x| x.0 == p).cloned().collect()).collect();
    for _ in 0..10000 {
        let mut s: Vec<(f64, f64)> = vec![];
        for g in &groups {
            for _ in 0..g.len() {
                s.push(g[(rng.next() as usize) % g.len()]);
            }
        }
        // need >= 2 distinct x
        if s.iter().all(|x| x.0 == s[0].0) {
            continue;
        }
        let (aa, bb, _, _) = fit_pts(&s);
        bs_slope.push(bb);
        bs_pstar.push(pstar(aa, bb));
    }
    let (bs_lo, bs_hi) = (pct(&mut bs_slope.clone(), 0.025), pct(&mut bs_slope.clone(), 0.975));
    let (ps_lo, ps_hi) = (pct(&mut bs_pstar.clone(), 0.025), pct(&mut bs_pstar.clone(), 0.975));
    // p* from slope interval ends at pivot (mean point) -- fixed-slope variants
    let n = prim.len() as f64;
    let mx = prim.iter().map(|q| q.0.ln()).sum::<f64>() / n;
    let my = prim.iter().map(|q| q.1.ln()).sum::<f64>() / n;
    let pstar_fixed = |bb: f64| (mx - my / bb).exp();

    // half slopes
    let lo_half: Vec<(f64, f64)> = prim.iter().cloned().filter(|x| x.0 <= 151.0).collect();
    let hi_half: Vec<(f64, f64)> = prim.iter().cloned().filter(|x| x.0 >= 251.0).collect();
    let (_, bl, sel, dfl) = fit_pts(&lo_half);
    let (_, bh, seh, dfh) = fit_pts(&hi_half);
    // fixed-slope -2 intercept & residual scatter
    let c2: f64 = prim.iter().map(|q| q.1.ln() + 2.0 * q.0.ln()).sum::<f64>() / n;

    let (ao, bo, seo, dfo) = fit_pts(&own);
    let tqo = t975(dfo);

    // sensitivity: rho repriced
    let mut sens = vec![];
    for w in [0.0f64, 0.1, 1.0] {
        let f = 1.0 + 3.0 * w;
        sens.push(json!({"w_gae_per_native_op": w, "rho_multiplier": f, "slope": b, "p_star_model": pstar(a - f.ln(), b)}));
    }

    // per prime summary
    let mut table = vec![];
    for g in &groups {
        let p = g[0].0;
        let ys: Vec<f64> = g.iter().map(|x| x.1).collect();
        let gm = (ys.iter().map(|y| y.ln()).sum::<f64>() / ys.len() as f64).exp();
        table.push(json!({"p": p, "n_seeds": ys.len(), "s_over_rho_per_seed": ys, "geo_mean": gm,
            "min": ys.iter().cloned().fold(f64::MAX, f64::min), "max": ys.iter().cloned().fold(0.0, f64::max)}));
    }
    let per_point: Vec<Value> = pts.iter().map(|q| json!({"p": q.p, "seed": q.seed, "ok": q.ok,
        "s_over_rho_pooled": ratio(q, rho_ref), "s_over_rho_own": q.own_rho.map(|r| ratio(q, r)), "own_rho_mean": q.own_rho, "rho_steps_mean": q.steps_mean})).collect();

    let out = json!({
        "labels": "measured: total_muls,l,c_add_e,rho samples; derived: everything below; model: p_star",
        "rho_ref_pooled": {"mean": rho_ref, "sd_single_run": rho_sd, "se": rho_se, "n_runs": rho_all.len(), "per_prime": per_prime_rho},
        "rho_trend_on_ln_p": {"slope": rb, "ci95": [rb_lo, rb_hi]},
        "primary_fit": {"n_points": prim.len(), "intercept": a, "slope": b, "slope_se": se, "df": df, "slope_ci95_t": [b_lo, b_hi],
            "slope_ci95_cluster_bootstrap": [bs_lo, bs_hi], "p_star": pstar(a, b), "p_star_ci95_bootstrap": [ps_lo, ps_hi],
            "p_star_from_slope_ci_ends_fixed_pivot": [pstar_fixed(b_lo), pstar_fixed(b_hi)]},
        "half_slopes": {"p_le_151": {"slope": bl, "se": sel, "df": dfl}, "p_ge_251": {"slope": bh, "se": seh, "df": dfh}, "difference": bh - bl},
        "fixed_slope_minus2": {"ln_intercept": c2, "ratio_const_C": c2.exp(), "p_star_model": c2.exp().sqrt()},
        "own_rho_fit": {"n_points": own.len(), "slope": bo, "slope_se": seo, "slope_ci95_t": [bo - tqo * seo, bo + tqo * seo], "p_star": pstar(ao, bo)},
        "rho_repricing_sensitivity_NOT_A_VALID_BAND": sens,
        "per_prime": table, "per_point": per_point, "failures": failures,
    });
    fs::write(format!("{dir}/fit.json"), serde_json::to_string_pretty(&out).unwrap()).unwrap();

    let mut t = String::new();
    t += &format!("pooled rho S = {rho_ref:.4} (single-run sd {rho_sd:.3}, se {rho_se:.4}, n={})\n", rho_all.len());
    for (p, m, k) in &per_prime_rho { t += &format!("  p={p}: rho S mean {m:.3} ({k} runs)\n"); }
    t += &format!("rho S trend on ln p: slope {rb:.4} CI95 [{rb_lo:.4}, {rb_hi:.4}]\n");
    t += &format!("PRIMARY n={} slope {b:.3} se {se:.3} df {df} t-CI95 [{b_lo:.3}, {b_hi:.3}] bootstrap [{bs_lo:.3}, {bs_hi:.3}]\n", prim.len());
    t += &format!("p* = {:.0} (bootstrap CI95 [{ps_lo:.0}, {ps_hi:.0}]; slope-CI ends [{:.0}, {:.0}])  MODEL extrapolation\n", pstar(a, b), pstar_fixed(b_lo), pstar_fixed(b_hi));
    t += &format!("half slopes: p<=151 {bl:.3} (se {sel:.3}), p>=251 {bh:.3} (se {seh:.3}), diff {:.3}\n", bh - bl);
    t += &format!("fixed slope -2: C = {:.0}, p* = {:.0}\n", c2.exp(), c2.exp().sqrt());
    t += &format!("own-rho fit: slope {bo:.3} CI95 [{:.3}, {:.3}] p* {:.0}\n", bo - tqo * seo, bo + tqo * seo, pstar(ao, bo));
    for r in &table { t += &format!("  {}\n", r); }
    for f in &failures { t += &format!("FAILURE {f}\n"); }
    fs::write(format!("{dir}/fit.txt"), &t).unwrap();
    print!("{t}");

    // SVG: log-log
    let (w, h, ml, mr, mt, mb) = (760.0, 520.0, 70.0, 20.0, 40.0, 60.0);
    let xmin = 40.0f64.ln();
    let xmax = 1500.0f64.ln();
    let ymax_v = prim.iter().map(|x| x.1).fold(1.0, f64::max).max(2.0) * 2.0;
    let ymin_v = prim.iter().map(|x| x.1).fold(f64::MAX, f64::min).min(1.0) / 2.0;
    let (ymin, ymax) = (ymin_v.ln(), ymax_v.ln());
    let sx = |p: f64| ml + (p.ln() - xmin) / (xmax - xmin) * (w - ml - mr);
    let sy = |v: f64| h - mb - (v.ln() - ymin) / (ymax - ymin) * (h - mt - mb);
    let mut s = format!("<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 {w} {h}' width='{w}' height='{h}' font-family='sans-serif' font-size='12'>\n<rect width='100%' height='100%' fill='white'/>\n");
    s += &format!("<text x='{}' y='22' font-size='15' font-weight='bold'>Cover-decomposition S/rho against p (weak class, lower-bound, log-log)</text>\n", ml);
    for tick in [50.0, 100.0, 200.0, 500.0, 1000.0] {
        s += &format!("<line x1='{x}' y1='{}' x2='{x}' y2='{}' stroke='#ddd'/><text x='{x}' y='{}' text-anchor='middle'>{tick}</text>\n", mt, h - mb, h - mb + 18.0, x = sx(tick));
    }
    let mut v = 1.0;
    while v <= ymax_v {
        for m in [1.0, 2.0, 5.0] {
            let yv = v * m;
            if yv >= ymin_v && yv <= ymax_v {
                s += &format!("<line x1='{ml}' y1='{y}' x2='{}' y2='{y}' stroke='#ddd'/><text x='{}' y='{}' text-anchor='end'>{yv}</text>\n", w - mr, ml - 6.0, sy(yv) + 4.0, y = sy(yv));
            }
        }
        v *= 10.0;
    }
    let mut v = 0.1;
    while v < 1.0 {
        for m in [1.0, 2.0, 5.0] {
            let yv = v * m;
            if yv >= ymin_v && yv < 1.0 {
                s += &format!("<line x1='{ml}' y1='{y}' x2='{}' y2='{y}' stroke='#ddd'/><text x='{}' y='{}' text-anchor='end'>{yv}</text>\n", w - mr, ml - 6.0, sy(yv) + 4.0, y = sy(yv));
            }
        }
        v *= 10.0;
    }
    s += &format!("<line x1='{ml}' y1='{y}' x2='{}' y2='{y}' stroke='#c00' stroke-dasharray='6 4'/><text x='{}' y='{}' fill='#c00'>S/rho = 1</text>\n", w - mr, w - mr - 70.0, sy(1.0) - 5.0, y = sy(1.0));
    // fit line and model line
    let line = |a: f64, bb: f64, col: &str, dash: &str| {
        format!("<line x1='{}' y1='{}' x2='{}' y2='{}' stroke='{col}' stroke-width='2' stroke-dasharray='{dash}'/>\n", sx(40.0), sy((a + bb * 40f64.ln()).exp()), sx(1500.0), sy((a + bb * 1500f64.ln()).exp()))
    };
    s += &line(a, b, "#1f77b4", "");
    s += &line(c2, -2.0, "#888", "3 3");
    for q in &prim {
        s += &format!("<circle cx='{}' cy='{}' r='4.5' fill='#1f77b4' fill-opacity='0.75' stroke='#fff'/>\n", sx(q.0), sy(q.1));
    }
    s += &format!("<text x='{}' y='{}'>p (log scale)</text><text x='14' y='{}' transform='rotate(-90 14 {})' text-anchor='middle'>S/rho (log scale, pooled rho S = {rho_ref:.3})</text>\n", w / 2.0 - 40.0, h - 22.0, h / 2.0, h / 2.0);
    s += &format!("<text x='{}' y='{}' fill='#1f77b4'>measured per-seed points; fit slope {b:.2} [{b_lo:.2}, {b_hi:.2}] (t, 95%)</text>\n", ml + 10.0, mt + 14.0);
    s += &format!("<text x='{}' y='{}' fill='#666'>dashed grey: cost-model slope -2 (derived), fitted intercept</text>\n", ml + 10.0, mt + 30.0);
    s += &format!("<text x='{}' y='{}' fill='#1f77b4'>blue line beyond measured p is a model extrapolation: p* = {:.0}</text>\n", ml + 10.0, mt + 46.0, pstar(a, b));
    s += "</svg>\n";
    fs::write(format!("{dir}/scaling.svg"), s).unwrap();
}
