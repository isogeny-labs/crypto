//! `analyze`: per-covariate regressions with log n as nuisance, Bonferroni
//! decision, planted positive control, permutation negative control, A/A.
use crate::stats::*;
use crate::util::*;
use std::collections::HashMap;

struct Outcome {
    name: &'static str,
    y: Vec<f64>,
}

fn by_idx(path: &str, kind: &str, field: &str) -> HashMap<u64, f64> {
    read_lines(path, kind).iter().map(|o| (num(o, "idx").unwrap() as u64, num(o, field).unwrap())).collect()
}

fn jf(x: f64) -> String {
    if x.is_finite() { format!("{x:.6e}") } else { "null".into() }
}

fn fit_json(f: &Fit, mean_y: f64) -> String {
    format!(
        "\"slope_frac\":{},\"ci_lo\":{},\"ci_hi\":{},\"se_frac\":{},\"t\":{},\"p\":{},\"df\":{}",
        jf(f.b / mean_y), jf(f.lo / mean_y), jf(f.hi / mean_y), jf(f.se / mean_y), jf(f.t), jf(f.p), f.df
    )
}

pub fn cmd(args: &[String]) {
    let curves_path = opt(args, "--curves").expect("--curves");
    let cov_path = opt(args, "--cov").expect("--cov");
    let ra = opt(args, "--rho-a").expect("--rho-a");
    let rb = opt(args, "--rho-b");
    let yp = opt(args, "--yield").expect("--yield");
    let seed = opt_u64(args, "--seed", Some(20261107));
    let perms = opt_u64(args, "--perms", Some(1000)) as usize;
    let alpha: f64 = opt(args, "--alpha").map(|s| s.parse().unwrap()).unwrap_or(0.05);
    let planted: f64 = opt(args, "--planted").map(|s| s.parse().unwrap()).unwrap_or(0.02);
    let covs: Vec<String> = opt(args, "--covariates")
        .unwrap_or_else(|| "z_691,z_5000,z_50000,logL1_200000,psi3_roots,psi5_roots".into())
        .split(',')
        .map(|s| s.to_string())
        .collect();
    let mut sink = Sink::open(opt(args, "--out"));

    // ---- join on idx ----
    let n_of = by_idx(&curves_path, "curve", "n");
    let cov_rows = read_lines(&cov_path, "cov");
    let a_y = by_idx(&ra, "rho", "mean_over_sqrt_n");
    let a_se = by_idx(&ra, "rho", "se_over_sqrt_n");
    let a_uns = by_idx(&ra, "rho", "unsolved");
    let b_y = rb.as_ref().map(|p| by_idx(p, "rho", "mean_over_sqrt_n"));
    let b_se = rb.as_ref().map(|p| by_idx(p, "rho", "se_over_sqrt_n"));
    let b_uns = rb.as_ref().map(|p| by_idx(p, "rho", "unsolved"));
    let cov_norm = by_idx(&yp, "yield", "cov_norm");
    let mut idxs: Vec<u64> = vec![];
    let mut covcols: HashMap<String, Vec<f64>> = HashMap::new();
    for o in &cov_rows {
        let i = num(o, "idx").unwrap() as u64;
        if !(n_of.contains_key(&i) && a_y.contains_key(&i) && cov_norm.contains_key(&i) && b_y.as_ref().map_or(true, |b| b.contains_key(&i))) {
            continue;
        }
        idxs.push(i);
        for c in covs.iter().chain(std::iter::once(&"z_691".to_string())) {
            covcols.entry(c.clone()).or_default().push(num(o, c).unwrap_or_else(|| panic!("covariate {c} missing")));
        }
    }
    let m = idxs.len();
    assert!(m > 10, "need more than 10 joined curves, have {m}");
    let w_raw: Vec<f64> = idxs.iter().map(|i| n_of[i].ln()).collect();
    let w = standardize(&w_raw).expect("log n has no variance");
    let ya: Vec<f64> = idxs.iter().map(|i| a_y[i]).collect();
    let yb: Option<Vec<f64>> = b_y.as_ref().map(|b| idxs.iter().map(|i| b[i]).collect());
    let y_rho: Vec<f64> = match &yb {
        Some(yb) => ya.iter().zip(yb).map(|(a, b)| 0.5 * (a + b)).collect(),
        None => ya.clone(),
    };
    let y_cov: Vec<f64> = idxs.iter().map(|i| cov_norm[i]).collect();
    let outcomes = vec![
        Outcome { name: "rho_ops_over_sqrt_n", y: y_rho.clone() },
        Outcome { name: "m2_coverage_over_2N2_over_n", y: y_cov },
    ];
    let mut zs: HashMap<String, Vec<f64>> = HashMap::new();
    let mut degenerate = vec![];
    for c in &covs {
        match standardize(&covcols[c]) {
            Some(z) => {
                zs.insert(c.clone(), z);
            }
            None => degenerate.push(c.clone()),
        }
    }
    let live: Vec<&String> = covs.iter().filter(|c| zs.contains_key(*c)).collect();
    let family = opt(args, "--family-size").map(|s| s.parse::<usize>().unwrap()).unwrap_or(outcomes.len() * live.len());
    let alpha_b = alpha / family as f64;
    let z_alpha = norm_quantile(1.0 - alpha_b / 2.0);
    let z80 = norm_quantile(0.8);

    let mut out = String::new();
    out += &format!("{{\"kind\":\"analysis\",\"schema\":\"p3.analysis/v1\",\"curves_joined\":{m},\"nuisance\":\"standardised ln n\",\"alpha\":{alpha},\"family_size\":{family},\"alpha_bonferroni\":{},\"degenerate_covariates\":{:?},\"seed\":{seed},\"perms\":{perms}", jf(alpha_b), degenerate);
    out += &format!(",\"unsolved_targets_rho_A\":{}", idxs.iter().map(|i| a_uns[i]).sum::<f64>());
    if let Some(u) = &b_uns {
        out += &format!(",\"unsolved_targets_rho_B\":{}", idxs.iter().map(|i| u[i]).sum::<f64>());
    }

    // ---- main tests ----
    let mut rows = vec![];
    let mut human = String::new();
    human += &format!("curves joined: {m}; family {family}; Bonferroni alpha {alpha_b:.2e}\n");
    human += &format!("{:<30} {:<14} {:>10} {:>10} {:>10} {:>9} {:>9}\n", "outcome", "covariate", "slope/mean", "ci_lo", "ci_hi", "p", "decision");
    for oc in &outcomes {
        let ym = mean(&oc.y);
        for c in &live {
            let f = ols(&oc.y, &zs[*c], &w);
            let reject = f.p < alpha_b;
            let equiv = f.lo / ym > -0.01 && f.hi / ym < 0.01;
            rows.push(format!("{{\"outcome\":\"{}\",\"covariate\":\"{}\",\"mean_y\":{},{},\"p_bonferroni\":{},\"reject_null\":{},\"ci_within_pm1pct\":{}}}", oc.name, c, jf(ym), fit_json(&f, ym), jf((f.p * family as f64).min(1.0)), reject, equiv));
            human += &format!("{:<30} {:<14} {:>+10.5} {:>+10.5} {:>+10.5} {:>9.3e} {:>9}\n", oc.name, c, f.b / ym, f.lo / ym, f.hi / ym, f.p, if reject { "REJECT" } else { "null" });
        }
    }
    out += &format!(",\"tests\":[{}]", rows.join(","));

    // ---- (a) planted positive control on standardised E_691 (= z_691 up to affine map) ----
    let ze = standardize(&covcols["z_691"]).expect("z_691 has no variance");
    let mut prow = vec![];
    for oc in &outcomes {
        let ym = mean(&oc.y);
        let yp2: Vec<f64> = oc.y.iter().zip(&ze).map(|(y, z)| y + planted * ym * z).collect();
        let f0 = ols(&oc.y, &ze, &w);
        let f = ols(&yp2, &ze, &w);
        let se_f = f0.se / ym;
        let power = norm_cdf(planted / se_f - z_alpha) + norm_cdf(-planted / se_f - z_alpha);
        let mde = (z_alpha + z80) * se_f;
        prow.push(format!("{{\"outcome\":\"{}\",\"planted_slope_frac\":{planted},\"unplanted_slope_frac\":{},\"recovered\":{{{}}},\"detected_at_bonferroni\":{},\"analytic_power\":{},\"mde_80pct_power_frac\":{}}}", oc.name, jf(f0.b / ym), fit_json(&f, ym), f.p < alpha_b, jf(power), jf(mde)));
        human += &format!("planted +{:.1}% on {}: recovered {:+.4} [{:+.4},{:+.4}] p={:.2e} detected={} analytic power {:.3}; MDE(80%) = {:.4}\n", planted * 100.0, oc.name, f.b / ym, f.lo / ym, f.hi / ym, f.p, f.p < alpha_b, power, mde);
    }
    out += &format!(",\"planted_control\":[{}]", prow.join(","));

    // ---- (b) permutation negative control ----
    let mut rng = Rng::new(derive(seed, &[0xBE12]));
    let mut perm_rows = vec![];
    for oc in &outcomes {
        let ym = mean(&oc.y);
        for c in &live {
            let obs = ols(&oc.y, &zs[*c], &w);
            let mut slopes = Vec::with_capacity(perms);
            let (mut n05, mut nb, mut nge) = (0usize, 0usize, 0usize);
            let mut zp = zs[*c].clone();
            for _ in 0..perms {
                for i in (1..m).rev() {
                    let j = rng.below(i as u64 + 1) as usize;
                    zp.swap(i, j);
                }
                let f = ols(&oc.y, &zp, &w);
                slopes.push(f.b / ym);
                if f.p < 0.05 { n05 += 1; }
                if f.p < alpha_b { nb += 1; }
                if (f.b / ym).abs() >= (obs.b / ym).abs() { nge += 1; }
            }
            let mut sorted = slopes.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            perm_rows.push(format!(
                "{{\"outcome\":\"{}\",\"covariate\":\"{}\",\"observed_slope_frac\":{},\"null_mean\":{},\"null_sd\":{},\"null_q025\":{},\"null_q975\":{},\"perm_p_two_sided\":{},\"false_positive_rate_at_0.05\":{},\"false_positive_rate_at_bonferroni\":{}}}",
                oc.name, c, jf(obs.b / ym), jf(mean(&slopes)), jf(var(&slopes).sqrt()), jf(quantile(&sorted, 0.025)), jf(quantile(&sorted, 0.975)),
                jf((1 + nge) as f64 / (1 + perms) as f64), jf(n05 as f64 / perms as f64), jf(nb as f64 / perms as f64)
            ));
        }
    }
    out += &format!(",\"permutation_control\":[{}]", perm_rows.join(","));

    // ---- (c) A/A ----
    if let Some(yb) = &yb {
        let n = m as f64;
        let (ma, mb) = (mean(&ya), mean(yb));
        let (va, vb) = (var(&ya), var(yb));
        let cov_ab: f64 = ya.iter().zip(yb).map(|(a, b)| (a - ma) * (b - mb)).sum::<f64>() / (n - 1.0);
        let r = cov_ab / (va * vb).sqrt();
        let fz = 0.5 * ((1.0 + r) / (1.0 - r)).ln();
        let sez = 1.0 / (n - 3.0).sqrt();
        let (rlo, rhi) = ((fz - 1.96 * sez).tanh(), (fz + 1.96 * sez).tanh());
        let za = standardize(&ya).unwrap();
        let fba = ols(yb, &za, &w);
        let diffs: Vec<f64> = ya.iter().zip(yb).map(|(a, b)| b - a).collect();
        let md = mean(&diffs);
        let tdiff = md / (var(&diffs) / n).sqrt();
        let se_a = idxs.iter().map(|i| a_se[i] * a_se[i]).sum::<f64>() / n;
        let se_b = idxs.iter().map(|i| b_se.as_ref().unwrap()[i].powi(2)).sum::<f64>() / n;
        out += &format!(
            ",\"aa_control\":{{\"mean_A\":{},\"mean_B\":{},\"mean_diff_B_minus_A\":{},\"paired_t\":{},\"paired_p\":{},\"pearson_r\":{},\"r_ci95\":[{},{}],\"slope_B_on_A_per_sd_A_frac_of_mean\":{{{}}},\"var_ratio_A\":{},\"var_ratio_B\":{},\"note\":\"var_ratio = between-curve variance of per-curve means / mean within-curve sampling variance; 1 means curve means differ only by sampling noise\"}}",
            jf(ma), jf(mb), jf(md), jf(tdiff), jf(t_p_two_sided(tdiff, n - 1.0)), jf(r), jf(rlo), jf(rhi), fit_json(&fba, mb), jf(va / se_a), jf(vb / se_b)
        );
        human += &format!("A/A: r(A,B)={r:+.4} [{rlo:+.4},{rhi:+.4}]; slope B on A {:+.4} (per SD of A, frac of mean) p={:.3}; mean B-A {:+.3e} (t={tdiff:+.2}); var ratios A {:.3} B {:.3}\n", fba.b / mb, fba.p, md, va / se_a, vb / se_b);
        // replication: per-covariate slopes on A and on B separately
        let mut rep = vec![];
        for c in &live {
            let fa = ols(&ya, &zs[*c], &w);
            let fb = ols(yb, &zs[*c], &w);
            rep.push(format!("{{\"covariate\":\"{}\",\"slope_frac_A\":{},\"p_A\":{},\"slope_frac_B\":{},\"p_B\":{}}}", c, jf(fa.b / ma), jf(fa.p), jf(fb.b / mb), jf(fb.p)));
        }
        out += &format!(",\"replication_rho_A_vs_B\":[{}]", rep.join(","));
    }
    out += "}";
    sink.line(&out);
    sink.flush();
    eprint!("{human}");
}

