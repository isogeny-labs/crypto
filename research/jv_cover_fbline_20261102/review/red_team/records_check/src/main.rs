//! Red-team record check for research/jv_cover_fbline_20261102 (derived numbers only;
//! every input is a committed run record). Usage: fbline_rt_records <repo root>
//!
//! 1. Decomposition rate against (2B/p)^6/720 and against the falling-factorial
//!    model 64*C(B,6)/p^6, plain arm vs line arm (is the rate law unchanged on the
//!    moved model?).
//! 2. Per-residual costs (F4, linear stage, Weil, stream, post, F4 native ops),
//!    paired line/plain (is the per-test cost unchanged?).
//! 3. Accounting identity: sum of the phase counters = total_muls; S recomputed.
//! 4. Regression to the mean: z of B_plain against the random-line model
//!    N((p-1)/2, p/4); extreme-value prediction for B_best (max of 65536 draws);
//!    the "typical-instance" speed-up.
//! 5. Small-sample statistics: t intervals (n = 9; n = 3 per prime) next to the
//!    study's percentile bootstrap.
//! 6. Section 5a with the search's own native operations added; tuned-cover
//!    thresholds (the factor a cover F_p-mult count must shrink by to reach the
//!    tuned rho).
use serde_json::Value;
use std::fs;

const PRIMES: [u64; 3] = [1301, 1451, 1601];
const N_SEARCH: f64 = 65536.0;
// Frozen reference constants of #3278 (estimator (b), pooled, 126 runs),
// read from research/jv_cover_fbline_20261102/analysis/rescore_fit.txt.
const REF_U1: f64 = 0.9322;
const REF_U2: f64 = 0.3879;
const REF_NATIVE: f64 = 15.81;
const REF_SD_STEPS: f64 = 0.434; // sd of steps/sqrt(l), n = 126
const REF_N: f64 = 126.0;

fn load(path: &str) -> Value {
    let v: Value = serde_json::from_str(&fs::read_to_string(path).expect(path)).expect(path);
    v[0].clone()
}
fn g(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap_or_else(|| panic!("missing {k}"))
}

/// erfc with fractional error < 1.2e-7 (Numerical Recipes erfcc).
fn erfc(x: f64) -> f64 {
    let z = x.abs();
    let t = 1.0 / (1.0 + 0.5 * z);
    let ans = t
        * (-z * z - 1.26551223
            + t * (1.00002368
                + t * (0.37409196
                    + t * (0.09678418
                        + t * (-0.18628806
                            + t * (0.27886807
                                + t * (-1.13520398
                                    + t * (1.48851587 + t * (-0.82215223 + t * 0.17087277)))))))))
            .exp();
    if x >= 0.0 {
        ans
    } else {
        2.0 - ans
    }
}
fn phi_cdf(z: f64) -> f64 {
    0.5 * erfc(-z / std::f64::consts::SQRT_2)
}
/// Mean and sd of the max of n iid N(0,1).
fn max_normal(n: f64) -> (f64, f64) {
    let (lo, hi, h) = (-8.0f64, 12.0f64, 1e-4f64);
    let mut m1 = 0.0;
    let mut m2 = 0.0;
    let mut z = lo;
    while z < hi {
        // density of the max: n phi(z) Phi(z)^(n-1)
        let pdf = (-(z * z) / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt();
        let c = phi_cdf(z);
        let d = n * pdf * ((n - 1.0) * c.ln()).exp();
        m1 += z * d * h;
        m2 += z * z * d * h;
        z += h;
    }
    (m1, (m2 - m1 * m1).sqrt())
}
fn mean(x: &[f64]) -> f64 {
    x.iter().sum::<f64>() / x.len() as f64
}
fn sd(x: &[f64]) -> f64 {
    let m = mean(x);
    (x.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / (x.len() as f64 - 1.0)).sqrt()
}
/// Falling-factorial decomposition rate 64*C(B,6)/p^6.
fn rate_ff(b: f64, p: f64) -> f64 {
    let mut prod = 1.0;
    for k in 0..6 {
        prod *= (b - k as f64) / p;
    }
    64.0 * prod / 720.0
}
fn rate_simple(b: f64, p: f64) -> f64 {
    (2.0 * b / p).powi(6) / 720.0
}
/// Residuals needed under the model: (B+1) relations at rate_ff.
fn resid_model(b: f64, p: f64) -> f64 {
    (b + 1.0) / rate_ff(b, p)
}

struct Inst {
    p: u64,
    seed: u64,
    plain: Value,
    line: Value,
}

fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| ".".into());
    let mut inst = Vec::new();
    for &p in &PRIMES {
        for s in 1..=3u64 {
            let plain = load(&format!(
                "{root}/research/jv_cover_crossover_20261101/results/dlp_p{p}_s{s}.json"
            ));
            let line = load(&format!(
                "{root}/research/jv_cover_fbline_20261102/results/line_p{p}_s{s}.json"
            ));
            inst.push(Inst { p, seed: s, plain, line });
        }
    }

    println!("## 0. Record identity and options");
    for i in &inst {
        let (a, b) = (&i.plain, &i.line);
        assert_eq!(a["l"], b["l"], "same l");
        assert_eq!(a["c_add_e"], b["c_add_e"]);
        assert_eq!(a["base"].as_u64(), b["fb_line"]["b_plain"].as_u64(), "b_plain = plain base");
        println!(
            "p={} s={} l equal; c_add_e {} ; plain stop_mode {} trace {} ; line stop_mode {} trace {} ; JV options otherwise as recorded",
            i.p, i.seed, g(a, "c_add_e"), a["stop_mode"], a["trace"], b["stop_mode"], b["trace"]
        );
    }

    println!("\n## 1. Decomposition rate vs model, both arms");
    println!("| p | s | arm | B | decomp | residuals | rate/simple | rate/ff | poisson rel se |");
    println!("|---|---|---|---|---|---|---|---|---|");
    let mut rs = [Vec::new(), Vec::new()];
    let mut rf = [Vec::new(), Vec::new()];
    for i in &inst {
        for (k, (lab, r)) in [("plain", &i.plain), ("line", &i.line)].iter().enumerate() {
            let b = g(r, "base");
            let p = i.p as f64;
            let rate = g(r, "decompositions") / g(r, "residuals");
            let a1 = rate / rate_simple(b, p);
            let a2 = rate / rate_ff(b, p);
            rs[k].push(a1.ln());
            rf[k].push(a2.ln());
            println!(
                "| {} | {} | {} | {} | {} | {} | {:.3} | {:.3} | {:.3} |",
                i.p,
                i.seed,
                lab,
                b,
                g(r, "decompositions"),
                g(r, "residuals"),
                a1,
                a2,
                1.0 / g(r, "decompositions").sqrt()
            );
        }
    }
    for (k, lab) in ["plain", "line"].iter().enumerate() {
        println!(
            "{lab}: geo-mean rate/simple {:.4} (sd of log {:.4}); rate/ff {:.4}",
            mean(&rs[k]).exp(),
            sd(&rs[k]),
            mean(&rf[k]).exp()
        );
    }
    // paired difference line - plain of log(rate/ff)
    let d: Vec<f64> = (0..9).map(|j| rf[1][j] - rf[0][j]).collect();
    let (md, sdd) = (mean(&d), sd(&d));
    let t = md / (sdd / 3.0);
    println!(
        "paired log(rate/ff) line - plain: mean {:+.4} (factor {:.4}), sd {:.4}, t(8) {:+.2}; expected noise sd of a difference ~ sqrt(1/700+1/700) = {:.4}",
        md,
        md.exp(),
        sdd,
        t,
        (2.0f64 / 700.0).sqrt()
    );

    println!("\n## 2. Per-residual costs (F_p multiplications per residual), paired");
    println!("| p | s | f4/res plain | f4/res line | ratio | lin/res plain | lin/res line | ratio | weil+stream+post /res ratio | f4_native/res ratio | (f4+lin)/res ratio |");
    println!("|---|---|---|---|---|---|---|---|---|---|---|");
    let mut r_f4 = Vec::new();
    let mut r_lin = Vec::new();
    let mut r_nat = Vec::new();
    for i in &inst {
        let pr = |r: &Value, k: &str| g(r, k) / g(r, "residuals");
        let (a, b) = (&i.plain, &i.line);
        let f4 = pr(b, "f4_muls") / pr(a, "f4_muls");
        let lin = pr(b, "lin_muls") / pr(a, "lin_muls");
        let oth = (pr(b, "weil_muls") + pr(b, "stream_muls") + pr(b, "post_muls"))
            / (pr(a, "weil_muls") + pr(a, "stream_muls") + pr(a, "post_muls"));
        let nat = pr(b, "f4_native_ops") / pr(a, "f4_native_ops");
        let both = (pr(b, "f4_muls") + pr(b, "lin_muls")) / (pr(a, "f4_muls") + pr(a, "lin_muls"));
        r_f4.push(f4.ln());
        r_lin.push(lin.ln());
        r_nat.push(nat.ln());
        println!(
            "| {} | {} | {:.0} | {:.0} | {:.4} | {:.0} | {:.0} | {:.4} | {:.4} | {:.4} | {:.4} |",
            i.p,
            i.seed,
            pr(a, "f4_muls"),
            pr(b, "f4_muls"),
            f4,
            pr(a, "lin_muls"),
            pr(b, "lin_muls"),
            lin,
            oth,
            nat,
            both
        );
    }
    println!(
        "geo-mean ratio line/plain: f4/res {:.4}, lin/res {:.4}, f4_native/res {:.4}",
        mean(&r_f4).exp(),
        mean(&r_lin).exp(),
        mean(&r_nat).exp()
    );

    println!("\n## 3. Accounting identity and phase shares (line arm)");
    let keys = [
        "setup_muls", "base_muls", "stream_muls", "weil_muls", "f4_muls", "lin_muls", "post_muls",
        "verify_muls", "la_muls",
    ];
    for i in &inst {
        for (lab, r) in [("plain", &i.plain), ("line", &i.line)] {
            let sum: f64 = keys.iter().map(|k| g(r, k)).sum();
            let tot = g(r, "total_muls");
            let s = tot / (g(r, "c_add_e") * (g(r, "l")).sqrt());
            let search = if lab == "line" { g(&r["fb_line"], "muls") } else { 0.0 };
            println!(
                "p={} s={} {lab}: sum(phases)/total {:.6}  S recomputed {:.4} (record {:.4})  f4 {:.1}% lin {:.1}% search {:.2}% la {:.3}%",
                i.p,
                i.seed,
                sum / tot,
                s,
                g(r, "s"),
                100.0 * g(r, "f4_muls") / tot,
                100.0 * g(r, "lin_muls") / tot,
                100.0 * search / tot,
                100.0 * g(r, "la_muls") / tot
            );
        }
    }

    println!("\n## 4. Regression to the mean and extreme values (random-line model B ~ N((p-1)/2, p/4))");
    let (ezmax, sdzmax) = max_normal(N_SEARCH);
    println!("max of 65536 iid N(0,1): mean {:.3}, sd {:.3}", ezmax, sdzmax);
    println!("| p | s | B_plain | z_plain | B_best | z_best | E[B_max] model | residual ratio measured | model (B+1)/rate_ff | (B_plain/B_best)^5 |");
    println!("|---|---|---|---|---|---|---|---|---|---|");
    let mut zp = Vec::new();
    let mut zb = Vec::new();
    let mut real = Vec::new();
    let mut typ_real_best = Vec::new();
    let mut typ_exp_best = Vec::new();
    for i in &inst {
        let p = i.p as f64;
        let mu = (p - 1.0) / 2.0;
        let sg = p.sqrt() / 2.0;
        let bp = g(&i.line["fb_line"], "b_plain");
        let bb = g(&i.line["fb_line"], "b_best");
        let z1 = (bp - mu) / sg;
        let z2 = (bb - mu) / sg;
        zp.push(z1);
        zb.push(z2);
        let rr = g(&i.line, "residuals") / g(&i.plain, "residuals");
        let rm = resid_model(bb, p) / resid_model(bp, p);
        let ratio = g(&i.line, "s") / g(&i.plain, "s");
        real.push(ratio.ln());
        // typical plain instance: E over B ~ N(mu, sg^2) of resid_model(B), by quadrature
        let mut e_plain = 0.0;
        let mut z: f64 = -8.0;
        let h: f64 = 1e-3;
        while z < 8.0 {
            let w = (-(z * z) / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt() * h;
            e_plain += w * resid_model(mu + sg * z, p);
            z += h;
        }
        // cost ratio ~ residual ratio (per-residual cost unchanged, section 2)
        typ_real_best.push((resid_model(bb, p) / e_plain).ln());
        let bexp = mu + sg * ezmax;
        typ_exp_best.push((resid_model(bexp, p) / e_plain).ln());
        println!(
            "| {} | {} | {} | {:+.2} | {} | {:+.2} | {:.1} | {:.3} | {:.3} | {:.3} |",
            i.p,
            i.seed,
            bp,
            z1,
            bb,
            z2,
            bexp,
            rr,
            rm,
            (bp / bb).powi(5)
        );
    }
    println!(
        "z_plain over 9 instances: mean {:+.3} (se under H0 {:.3}), sd {:.3}  -> plain instances were {}",
        mean(&zp),
        1.0 / 3.0,
        sd(&zp),
        if mean(&zp).abs() < 2.0 / 3.0 { "not unlucky on average" } else { "atypical" }
    );
    println!(
        "z_best over 9 instances: mean {:+.3}, sd {:.3}; EVT (Gaussian) predicts {:.3} +- {:.3}",
        mean(&zb),
        sd(&zb),
        ezmax,
        sdzmax
    );
    println!(
        "realised geo-mean S_line/S_plain {:.4} (x{:.3}); typical-plain (E over B_plain) vs realised B_best: {:.4} (x{:.3}); vs EVT-expected B_best: {:.4} (x{:.3})",
        mean(&real).exp(),
        1.0 / mean(&real).exp(),
        mean(&typ_real_best).exp(),
        1.0 / mean(&typ_real_best).exp(),
        mean(&typ_exp_best).exp(),
        1.0 / mean(&typ_exp_best).exp()
    );

    println!("\n## 5. Small-sample statistics");
    let m = mean(&real);
    let s = sd(&real);
    let t8 = 2.306;
    println!(
        "paired log ratio n=9: mean {:.4} sd {:.4}; t(8) 95%: ratio {:.4} [{:.4}, {:.4}] speed-up x{:.3} [{:.3}, {:.3}] (study bootstrap 0.5865 [0.5301, 0.6508])",
        m,
        s,
        m.exp(),
        (m - t8 * s / 3.0).exp(),
        (m + t8 * s / 3.0).exp(),
        (-m).exp(),
        (-(m + t8 * s / 3.0)).exp(),
        (-(m - t8 * s / 3.0)).exp()
    );
    // model-residual: log(measured/predicted) spread
    let resid: Vec<f64> = inst
        .iter()
        .map(|i| {
            let p = i.p as f64;
            let bp = g(&i.line["fb_line"], "b_plain");
            let bb = g(&i.line["fb_line"], "b_best");
            (g(&i.line, "s") / g(&i.plain, "s")).ln() - (resid_model(bb, p) / resid_model(bp, p)).ln()
        })
        .collect();
    println!(
        "log(measured/model) per instance: mean {:+.4} sd {:.4} (Poisson floor for two ~700-count runs {:.4})",
        mean(&resid),
        sd(&resid),
        (2.0f64 / 700.0).sqrt()
    );
    let t2 = 4.303;
    let se_ref = REF_SD_STEPS / REF_N.sqrt() / REF_U1; // relative se of the reference mean
    println!("U1/U2 per prime with a t(2) cover-side interval combined with the reference se ({:.4} relative):", se_ref);
    for (pi, &p) in PRIMES.iter().enumerate() {
        let v: Vec<f64> = (0..3).map(|k| g(&inst[3 * pi + k].line, "s").ln()).collect();
        let (mc, sc) = (mean(&v), sd(&v) / 3f64.sqrt());
        let half = ((t2 * sc).powi(2) + (1.96 * se_ref).powi(2)).sqrt();
        for (u, r) in [("U1", REF_U1), ("U2", REF_U2)] {
            let c = mc.exp() / r;
            println!(
                "  p={p} {u}: {:.3} [{:.3}, {:.3}] (cover log-sd {:.4}, 3 seeds)",
                c,
                c * (-half).exp(),
                c * half.exp(),
                sd(&v)
            );
        }
    }

    println!("\n## 6. Section 5a with the search's native operations, and tuned-cover thresholds");
    println!("search natives per (line, point): k = 1 (square-table lookup) or 3 (lookup, zero test, counter)");
    for (pi, &p) in PRIMES.iter().enumerate() {
        let rows: Vec<&Inst> = (0..3).map(|k| &inst[3 * pi + k]).collect();
        let sqrt_l = g(&rows[0].line, "l").sqrt();
        let nat: Vec<f64> = rows.iter().map(|i| g(&i.line, "f4_native_ops") / sqrt_l).collect();
        let sv: Vec<f64> = rows.iter().map(|i| g(&i.line, "s")).collect();
        let search_nat = N_SEARCH * p as f64 / sqrt_l;
        for k in [0.0, 1.0, 3.0] {
            let mut line = format!("  p={p} k={k}: search natives/sqrt(l) {:.4};", k * search_nat);
            for w in [0.0, 0.1, 1.0] {
                let cov: Vec<f64> = (0..3)
                    .map(|j| (sv[j] + w * (nat[j] + k * search_nat)).ln())
                    .collect();
                let c = mean(&cov).exp();
                line += &format!(
                    " w={w}: U1 {:.3} U2 {:.3};",
                    c / (REF_U1 + w * REF_NATIVE),
                    c / (REF_U2 + w * REF_NATIVE)
                );
            }
            println!("{line}");
        }
        let gm = mean(&sv.iter().map(|v| v.ln()).collect::<Vec<_>>()).exp();
        println!(
            "  p={p}: cover geo-mean S {:.4}; tuned rho/harness rho = {:.4}; cover must shrink its F_p-mult count by x{:.3} (tau* = {:.3}) to reach the tuned negation rho",
            gm,
            REF_U2 / REF_U1,
            gm / REF_U2,
            REF_U2 / gm
        );
    }

    // ── 7. Census-based fair speed-up, its decay with p, and the crossover slope
    println!("\n## 7. Census (line_census_*.txt): fair speed-up per prime, decay with p, crossover under the decaying gain (model)");
    let rt = format!("{root}/research/jv_cover_fbline_20261102/review/red_team");
    let mut census: Vec<Value> = Vec::new();
    for f in ["line_census_p1301.txt", "line_census_p1451_p1601.txt"] {
        for l in fs::read_to_string(format!("{rt}/{f}")).expect(f).lines() {
            if let Some(js) = l.strip_prefix("JSON ") {
                census.push(serde_json::from_str(js).unwrap());
            }
        }
    }
    assert_eq!(census.len(), 9, "nine census rows");
    let all_match = census.iter().all(|c| c["replication_match"].as_bool() == Some(true));
    let any_subfield_j = census
        .iter()
        .any(|c| c["j_in_fp2"].as_bool() == Some(true) || c["j_in_fp3"].as_bool() == Some(true));
    println!("replication of b_plain/b_best/best_index on all 9: {all_match}; j(E) in F_p2 or F_p3 on any instance: {any_subfield_j}");
    let mut lg = Vec::new();
    let mut lp = Vec::new();
    let mut fair_s = Vec::new();
    let mut extra = Vec::new();
    let mut pct = Vec::new();
    for (k, c) in census.iter().enumerate() {
        let i = &inst[k];
        assert_eq!(c["p"].as_u64(), Some(i.p));
        assert_eq!(c["seed"].as_u64(), Some(i.seed));
        let share = g(&i.line["fb_line"], "muls") / g(&i.line, "total_muls");
        let fr = g(c, "fair_ratio") / (1.0 - share); // residual ratio, then the search's share of the line run
        fair_s.push(fr.ln());
        lg.push(-g(c, "fair_ratio").ln());
        lp.push((i.p as f64).ln());
        extra.push(g(c, "exhaustive_extra_speedup").ln());
        pct.push(g(c, "pct_plain"));
    }
    println!(
        "fair expected speed-up in S (E over the plain line's B, max of 65536, search charged): geo-mean x{:.3}; per prime x{:.3} / x{:.3} / x{:.3}",
        (-mean(&fair_s)).exp(),
        (-mean(&fair_s[0..3])).exp(),
        (-mean(&fair_s[3..6])).exp(),
        (-mean(&fair_s[6..9])).exp()
    );
    println!(
        "plain-line percentiles among all lines: {:?} (mean {:.3}; uniform expects 0.5 +- {:.3})",
        pct.iter().map(|v| (v * 1000.0).round() / 1000.0).collect::<Vec<_>>(),
        mean(&pct),
        (1.0f64 / 12.0 / 9.0).sqrt()
    );
    println!(
        "exhaustive search over all p(p+1)/2 square-direction lines would add x{:.3} (geo-mean; range x{:.3}-x{:.3})",
        mean(&extra).exp(),
        extra.iter().cloned().fold(f64::INFINITY, f64::min).exp(),
        extra.iter().cloned().fold(f64::NEG_INFINITY, f64::max).exp()
    );
    // OLS slope of ln G on ln p
    let (mx, my) = (mean(&lp), mean(&lg));
    let sxy: f64 = lp.iter().zip(&lg).map(|(x, y)| (x - mx) * (y - my)).sum();
    let sxx: f64 = lp.iter().map(|x| (x - mx) * (x - mx)).sum();
    let slope_g = sxy / sxx;
    let line_slope = -2.10 - slope_g;
    println!(
        "d ln G / d ln p = {:+.3} (G = fair residual speed-up); line-arm S/rho slope = -2.10 - ({:+.3}) = {:.3} (model)",
        slope_g, slope_g, line_slope
    );
    for (pi, &p) in PRIMES.iter().enumerate() {
        let v: Vec<f64> = (0..3).map(|k| g(&inst[3 * pi + k].line, "s").ln()).collect();
        let gm = mean(&v).exp();
        for (u, r) in [("U1", REF_U1), ("U2", REF_U2)] {
            let ratio = gm / r;
            println!(
                "  anchored at p={p}: {u} {:.3} -> p* at slope -2.10: {:.0}; at slope {:.2}: {:.0} (model)",
                ratio,
                p as f64 * ratio.powf(1.0 / 2.10),
                line_slope,
                p as f64 * ratio.powf(-1.0 / line_slope)
            );
        }
    }
    println!(
        "gain model (1 + k/sqrt p)^5 with k = 4.29 (n = 65536): p=2039 x{:.3}, p=1e4 x{:.3}, p=1e5 x{:.3} (model: the lever decays as p grows)",
        (1.0 + 4.29 / 2039f64.sqrt()).powi(5),
        (1.0 + 4.29 / 1e4f64.sqrt()).powi(5),
        (1.0 + 4.29 / 1e5f64.sqrt()).powi(5)
    );

    // ── 8. Exploratory pilot: the line search on the frontier sieve route
    println!("\n## 8. EXPLORATORY pilot (not frozen, not evidence): line search on the jv_sieve route, p = 1511, seeds 17-24, JV_LIN=sparse --stop 64");
    let v2: Value = serde_json::from_str(
        &fs::read_to_string(format!(
            "{root}/research/jv_sieve_strong_rho_v2_20261020/raw/ic_1511_sparse.json"
        ))
        .expect("v2 record"),
    )
    .unwrap();
    let mut rows: Vec<Value> = Vec::new();
    for l in fs::read_to_string(format!("{rt}/sieve_transfer_pilot_p1511.jsonl")).expect("pilot").lines() {
        if let Some(js) = l.strip_prefix("JSON ") {
            rows.push(serde_json::from_str(js).unwrap());
        }
    }
    let mut repro = 0;
    let mut checked = 0;
    for r in &rows {
        let seed = r["seed"].as_u64().unwrap();
        if let Some(v) = v2.as_array().unwrap().iter().find(|v| v["seed"].as_u64() == Some(seed)) {
            checked += 1;
            if v["total_muls"] == r["plain"]["total_muls"] {
                repro += 1;
            }
        }
    }
    println!("plain arm reproduces the V2 record's total_muls on {repro}/{checked} (seed, n) rows");
    // Pareto check against the in-tree sieve route (same class, same unit 331*sqrt(l) F_p mults)
    let v2a = v2.as_array().unwrap();
    let s_sieve = v2a.iter().map(|v| g(v, "s")).sum::<f64>() / v2a.len() as f64;
    let s_plus = v2a.iter().map(|v| g(v, "s_plus")).sum::<f64>() / v2a.len() as f64;
    let bases: Vec<f64> = v2a.iter().map(|v| g(v, "base")).collect();
    for (pi, &p) in PRIMES.iter().enumerate() {
        let v: Vec<f64> = (0..3).map(|k| g(&inst[3 * pi + k].line, "s").ln()).collect();
        let gm = mean(&v).exp();
        println!(
            "  Pareto: line-search Nagao arm S at p={p} {:.4} vs jv_sieve S at p=1511 (V2 sparse, n={}, mean) {:.5} -> x{:.1}; vs sieve S+ (adds, lookups charged) {:.5} -> x{:.1}; sieve base mean {:.0} (same F_p-abscissa base)",
            gm,
            v2a.len(),
            s_sieve,
            gm / s_sieve,
            s_plus,
            gm / s_plus,
            mean(&bases)
        );
    }
    for n in [65536u64, 4096] {
        let sel: Vec<&Value> = rows.iter().filter(|r| r["n_lines"].as_u64() == Some(n)).collect();
        if sel.is_empty() {
            continue;
        }
        let route: Vec<f64> = sel
            .iter()
            .map(|r| (g(&r["line"], "total_muls_route") / g(&r["plain"], "total_muls")).ln())
            .collect();
        let charged: Vec<f64> = sel.iter().map(|r| g(r, "ratio_total_muls").ln()).collect();
        let share: Vec<f64> = sel
            .iter()
            .map(|r| g(&r["line"], "search_muls") / g(&r["line"], "total_muls_charged"))
            .collect();
        let ok = sel
            .iter()
            .all(|r| r["line"]["correct"].as_bool() == Some(true) && r["plain"]["correct"].as_bool() == Some(true));
        let nn = sel.len() as f64;
        let t7 = 2.365;
        let (mc, sc) = (mean(&charged), sd(&charged) / nn.sqrt());
        let (mr, sr) = (mean(&route), sd(&route) / nn.sqrt());
        println!(
            "  n_lines {n}: {} seeds, all correct {ok}; route-only ratio {:.3} [{:.3}, {:.3}] (x{:.2}); search-charged ratio {:.3} [{:.3}, {:.3}] (x{:.2}); search share of the line arm {:.1}% (t(7) 95%)",
            sel.len(),
            mr.exp(),
            (mr - t7 * sr).exp(),
            (mr + t7 * sr).exp(),
            (-mr).exp(),
            mc.exp(),
            (mc - t7 * sc).exp(),
            (mc + t7 * sc).exp(),
            (-mc).exp(),
            100.0 * mean(&share)
        );
    }

    // ── 9. F_p additions (unemitted on the cover, emitted-but-unpriced on rho)
    println!("\n## 9. F_p additions: sensitivity of U1/U2 (MODEL: cover additions = r_c x cover multiplications, not emitted)");
    let rho_adds_per_step = 42.1 / 0.9324; // emitted: fp_adds/sqrt(l) over steps/sqrt(l), estimator (b)
    let tuned_step = 331.0 * REF_U2 / REF_U1;
    println!("rho F_p additions per step {:.1} (emitted); rho step = 331 harness / {:.1} tuned F_p mults", rho_adds_per_step, tuned_step);
    for (pi, &p) in PRIMES.iter().enumerate() {
        let v: Vec<f64> = (0..3).map(|k| g(&inst[3 * pi + k].line, "s").ln()).collect();
        let gm = mean(&v).exp();
        let mut line = format!("  p={p}:");
        for rc in [0.5, 1.0] {
            for alpha in [0.1, 0.3] {
                let u1 = gm / REF_U1 * (1.0 + alpha * rc) / (1.0 + alpha * rho_adds_per_step / 331.0);
                let u2 = gm / REF_U2 * (1.0 + alpha * rc) / (1.0 + alpha * rho_adds_per_step / tuned_step);
                line += &format!(" [r_c {rc} add/mul price {alpha}: U1 {:.3} U2 {:.3}]", u1, u2);
            }
        }
        println!("{line}");
        // section 5a literal: additions as native operations at w GAE each, both arms
        let sqrt_l = g(&inst[3 * pi].line, "l").sqrt();
        let nat: f64 = (0..3).map(|k| g(&inst[3 * pi + k].line, "f4_native_ops") / sqrt_l).sum::<f64>() / 3.0;
        let mut l2 = format!("  p={p} (5a literal, additions priced as native ops, r_c = 1):");
        for w in [0.0, 0.1, 1.0] {
            let cov = gm + w * (nat + gm * 331.0);
            let u1 = cov / (REF_U1 + w * (REF_NATIVE + 42.1));
            l2 += &format!(" w={w}: U1 {:.2};", u1);
        }
        println!("{l2}");
    }
}
