//! Native summaries of the recorded JSONL (statistics are computed here, never in plotting code).
use crate::field::{derive, Rng};
use std::collections::BTreeMap;

fn num(line: &str, key: &str) -> Option<f64> {
    let k = format!("\"{}\":", key);
    let i = line.find(&k)? + k.len();
    let rest = &line[i..];
    let end = rest.find(|c: char| c == ',' || c == '}').unwrap_or(rest.len());
    rest[..end].trim().parse().ok()
}
fn arr(line: &str, key: &str) -> Vec<f64> {
    let k = format!("\"{}\":[", key);
    let Some(i) = line.find(&k) else { return vec![] };
    let rest = &line[i + k.len()..];
    let end = rest.find(']').unwrap();
    rest[..end].split(',').filter(|s| !s.is_empty()).map(|s| s.trim().parse().unwrap()).collect()
}
fn arr2(line: &str, key: &str) -> Vec<Vec<f64>> {
    let k = format!("\"{}\":[", key);
    let Some(i) = line.find(&k) else { return vec![] };
    let mut rest = &line[i + k.len()..];
    let mut out = vec![];
    while rest.starts_with('[') {
        let end = rest.find(']').unwrap();
        out.push(rest[1..end].split(',').filter(|s| !s.is_empty()).map(|s| s.parse().unwrap()).collect());
        rest = &rest[end + 1..];
        rest = rest.trim_start_matches(',');
    }
    out
}
fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}
fn var(v: &[f64]) -> f64 {
    let m = mean(v);
    v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (v.len() as f64 - 1.0)
}
fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    s[s.len() / 2]
}
fn boot<F: Fn(&[f64]) -> f64>(v: &[f64], stat: F, seed: u64) -> (f64, f64) {
    let mut rng = Rng(derive(seed, "bootstrap", v.len() as u64, 0));
    let mut st: Vec<f64> = (0..1000)
        .map(|_| {
            let s: Vec<f64> = (0..v.len()).map(|_| v[rng.below(v.len() as u64) as usize]).collect();
            stat(&s)
        })
        .collect();
    st.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (st[25], st[974])
}

pub fn run(dir: &str) {
    let read = |f: &str| std::fs::read_to_string(format!("{}/{}", dir, f)).unwrap_or_default();
    // ---- algebra
    for n in [2u32, 3] {
        let txt = read(&format!("algebra_n{}.jsonl", n));
        let mut by: BTreeMap<u64, Vec<&str>> = BTreeMap::new();
        for l in txt.lines() {
            by.entry(num(l, "p").unwrap() as u64).or_default().push(l);
        }
        for (p, ls) in by {
            let sd: Vec<f64> = ls.iter().filter_map(|l| num(l, "solving_degree")).collect();
            let qd: Vec<f64> = ls.iter().filter_map(|l| num(l, "quotient_dim")).collect();
            let ff = ls.iter().filter(|l| num(l, "first_fall").is_some()).count();
            let mults: Vec<f64> = ls.iter().filter_map(|l| num(l, "sd_mults")).collect();
            let rows = num(ls[0], "sd_rows").unwrap_or(0.0);
            let cols = num(ls[0], "sd_cols").unwrap_or(0.0);
            let (mut rat, mut gen, mut nons, mut sing, mut posd, mut orc, mut ne) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0);
            for l in &ls {
                if let Some(r) = num(l, "rational") {
                    ne += 1;
                    rat += r;
                    gen += num(l, "genuine").unwrap_or(0.0);
                    nons += num(l, "nonsplit").unwrap_or(0.0);
                    sing += num(l, "singular").unwrap_or(0.0);
                    posd += num(l, "posdim_lines").unwrap_or(0.0);
                    orc += num(l, "oracle_xsets").unwrap_or(0.0);
                }
            }
            println!(
                "{{\"summary\":\"algebra\",\"n\":{},\"p\":{},\"inst\":{},\"sd_min\":{},\"sd_median\":{},\"sd_max\":{},\"sd_found\":{},\"qdim_min\":{},\"qdim_max\":{},\"instances_with_fall\":{},\"sd_rows\":{},\"sd_cols\":{},\"c_mults_mean\":{:.0},\"enum_targets\":{},\"rational\":{},\"genuine\":{},\"nonsplit\":{},\"singular\":{},\"posdim\":{},\"oracle_xsets\":{}}}",
                n, p, ls.len(),
                sd.iter().cloned().fold(f64::INFINITY, f64::min), median(&sd), sd.iter().cloned().fold(0.0, f64::max), sd.len(),
                qd.iter().cloned().fold(f64::INFINITY, f64::min), qd.iter().cloned().fold(0.0, f64::max),
                ff, rows, cols, mean(&mults), ne, rat, gen, nons, sing, posd, orc
            );
        }
    }
    // ---- yield
    let txt = read("yield.jsonl");
    let mut by: BTreeMap<u64, Vec<&str>> = BTreeMap::new();
    for l in txt.lines() {
        by.entry(num(l, "p").unwrap() as u64).or_default().push(l);
    }
    for (p, ls) in by {
        for (ci, l) in ls.iter().enumerate() {
            let c = arr(l, "counts");
            let lam = num(l, "lambda_pred").unwrap();
            let m = mean(&c);
            let disp = |v: &[f64]| var(v) / mean(v).max(1e-12);
            let (dlo, dhi) = boot(&c, disp, p * 100 + ci as u64);
            let (mlo, mhi) = boot(&c, mean, p * 100 + ci as u64 + 50);
            let p0 = c.iter().filter(|&&x| x == 0.0).count() as f64 / c.len() as f64;
            let mx = c.iter().cloned().fold(0.0, f64::max);
            let mut s = c.clone();
            s.sort_by(|a, b| b.partial_cmp(a).unwrap());
            let top = s[..(c.len() / 100).max(1)].iter().sum::<f64>() / s.iter().sum::<f64>().max(1.0);
            let full = arr2(l, "full_phi");
            let lp1 = arr2(l, "lp1_phi");
            let phis = [1.0f64, 0.5, 0.25, 0.125];
            let y1 = mean(&full[0]);
            let full_ratio: Vec<String> = phis.iter().enumerate().map(|(i, &f)| format!("{:.4}", mean(&full[i]) / (y1 * f.powi(3)))).collect();
            let lp1_ratio: Vec<String> = phis.iter().enumerate().skip(1).map(|(i, &f)| format!("{:.4}", mean(&lp1[i]) / (y1 * 3.0 * f * f * (1.0 - f)))).collect();
            let mm = num(l, "fb_xclasses").unwrap();
            let hyper = |f: f64| { let c = (f * mm).ceil().max(1.0); c * (c - 1.0) * (c - 2.0) / (mm * (mm - 1.0) * (mm - 2.0)) };
            let hyper1 = |f: f64| { let c = (f * mm).ceil().max(1.0); 3.0 * c * (c - 1.0) * (mm - c) / (mm * (mm - 1.0) * (mm - 2.0)) };
            let fh: Vec<String> = phis.iter().map(|&f| format!("{:.5}", hyper(f))).collect();
            let lh: Vec<String> = phis.iter().map(|&f| format!("{:.5}", hyper1(f))).collect();
            let full_mean: Vec<String> = full.iter().map(|v| format!("{:.5}", mean(v))).collect();
            let lp1_mean: Vec<String> = lp1.iter().map(|v| format!("{:.5}", mean(v))).collect();
            println!(
                "{{\"summary\":\"yield\",\"p\":{},\"curve\":{},\"targets\":{},\"mean\":{:.5},\"mean_ci\":[{:.5},{:.5}],\"lambda_pred\":{:.5},\"mean_over_lambda\":{:.4},\"p0\":{:.4},\"p0_poisson\":{:.4},\"dispersion\":{:.4},\"dispersion_ci\":[{:.4},{:.4}],\"max\":{},\"top1pct_share\":{:.4},\"full_mean_phi\":[{}],\"full_over_phi3\":[{}],\"lp1_mean_phi\":[{}],\"lp1_over_model\":[{}],\"hyper_full\":[{}],\"hyper_lp1\":[{}],\"fb\":{}}}",
                p, ci, c.len(), m, mlo, mhi, lam, m / lam, p0, (-m).exp(), disp(&c), dlo, dhi, mx, top,
                full_mean.join(","), full_ratio.join(","), lp1_mean.join(","), lp1_ratio.join(","), fh.join(","), lh.join(","), mm
            );
        }
    }
    // ---- pooled phi test (p >= 251): observed vs exact hypergeometric expectation, Poisson 95% CI
    {
        let phis = [1.0f64, 0.5, 0.25, 0.125];
        for (pi, &f) in phis.iter().enumerate().skip(1) {
            let (mut of, mut ef, mut o1, mut e1) = (0.0, 0.0, 0.0, 0.0);
            for l in txt.lines().filter(|l| num(l, "p").unwrap() >= 251.0) {
                let full = arr2(l, "full_phi");
                let lp1 = arr2(l, "lp1_phi");
                let mm = num(l, "fb_xclasses").unwrap();
                let c = (f * mm).ceil().max(1.0);
                let tot: f64 = full[0].iter().sum();
                of += full[pi].iter().sum::<f64>();
                ef += tot * c * (c - 1.0) * (c - 2.0) / (mm * (mm - 1.0) * (mm - 2.0));
                o1 += lp1[pi].iter().sum::<f64>();
                e1 += tot * 3.0 * c * (c - 1.0) * (mm - c) / (mm * (mm - 1.0) * (mm - 2.0));
            }
            let ci = |o: f64, e: f64| ((o - 1.96 * o.sqrt()).max(0.0) / e, (o + 1.96 * o.sqrt() + 1.0) / e);
            let (a, b) = ci(of, ef);
            let (c1, d1) = ci(o1, e1);
            println!("{{\"summary\":\"phi_pooled\",\"p_min\":251,\"phi\":{},\"full_obs\":{},\"full_exp\":{:.1},\"full_ratio\":{:.4},\"full_ci\":[{:.3},{:.3}],\"lp1_obs\":{},\"lp1_exp\":{:.1},\"lp1_ratio\":{:.4},\"lp1_ci\":[{:.3},{:.3}]}}", f, of, ef, of / ef, a, b, o1, e1, o1 / e1, c1, d1);
        }
    }
    // ---- pipeline + derived crossover model
    let txt = read("pipeline.jsonl");
    let mut fits: Vec<(f64, f64, f64, f64, f64)> = vec![]; // p, trials/p, nnz/rows, c3, h
    for l in txt.lines() {
        let p = num(l, "p").unwrap();
        let band = arr2(l, "band"); // not arrays of numbers; parse ratios below instead
        let _ = band;
        let ratios: Vec<f64> = ["w\":0,", "w\":0.1,", "w\":1,"]
            .iter()
            .map(|w| {
                let i = l.find(w).unwrap();
                num(&l[i..], "ic_over_rho_measured").unwrap()
            })
            .collect();
        let neg: Vec<f64> = ["w\":0,", "w\":0.1,", "w\":1,"]
            .iter()
            .map(|w| {
                let i = l.find(w).unwrap();
                num(&l[i..], "ic_over_rho_negmap").unwrap()
            })
            .collect();
        let trials = num(l, "trials").unwrap();
        let c3 = num(l, "c3_mean").unwrap();
        let rows = num(l, "relations").unwrap();
        let nnz = num(l, "nnz").unwrap();
        fits.push((p, trials / p, nnz / rows, c3, num(l, "h").unwrap()));
        let ordered = |v: &[f64]| if v.iter().all(|&r| r > 1.0) { "rho_ahead" } else if v.iter().all(|&r| r < 1.0) { "ic_ahead" } else { "unresolved" };
        println!(
            "{{\"summary\":\"pipeline\",\"p\":{},\"curve\":{},\"ell\":{},\"k_correct\":{},\"rank\":{},\"unknowns\":{},\"empty_cols\":{},\"signs_dropped_consistent\":{},\"ratio_measured_w\":[{:.4},{:.4},{:.4}],\"ratio_negmap_w\":[{:.4},{:.4},{:.4}],\"verdict_measured\":\"{}\",\"verdict_negmap\":\"{}\"}}",
            p, num(l, "curve").unwrap(), num(l, "ell").unwrap(),
            l.contains("\"k_correct\":true"), num(l, "rank").unwrap(), num(l, "unknowns").unwrap(), num(l, "empty_cols").unwrap(),
            l.contains("\"ctrl_signs_dropped\":{\"consistent\":true"),
            ratios[0], ratios[1], ratios[2], neg[0], neg[1], neg[2], ordered(&ratios), ordered(&neg)
        );
    }
    if fits.is_empty() {
        return;
    }
    // Derived model (labelled as such): trials ~ tau p, LA ~ 3 * (p/2) * (p/2 * rw), c3 constant (n=3),
    // rho_negmap ~ sqrt(pi p^3 / (4 h)). Crossover p* where IC(w) = rho(w) for phi = 1 (no FB reduction),
    // and the best over phi in (0,1] with full-relation yield ~ phi^3 (row 2 measurement).
    let big: Vec<&(f64, f64, f64, f64, f64)> = fits.iter().filter(|f| f.0 >= 127.0).collect();
    let tau = mean(&big.iter().map(|f| f.1).collect::<Vec<_>>());
    let rw = mean(&big.iter().map(|f| f.2).collect::<Vec<_>>());
    let c3 = mean(&big.iter().map(|f| f.3).collect::<Vec<_>>());
    for w in [0.1f64, 1.0] {
        let mut lines = vec![];
        for e in (8..=64).step_by(2) {
            let p = 2f64.powi(e);
            let rho = (std::f64::consts::PI * p.powi(3) / 4.0).sqrt() * (1.0 + w);
            let mut best = (f64::INFINITY, 1.0);
            for k in 0..=200 {
                let phi = 0.5f64.powf(k as f64 * 0.25);
                let fb = phi * p / 2.0;
                let trials = tau * p / phi.powi(2); // trials needed ~ FB / (yield phi^3) = tau p phi / phi^3
                let la = 3.0 * fb * fb * rw;
                let ic = trials * (1.0 + w * c3) + w * la;
                if ic < best.0 {
                    best = (ic, phi);
                }
            }
            lines.push(format!("{{\"log2p\":{},\"log2_group\":{},\"ic_best\":{:.3e},\"phi_best\":{:.3e},\"rho_negmap\":{:.3e},\"ratio\":{:.4e}}}", e, 3 * e, best.0, best.1, rho, best.0 / rho));
        }
        println!("{{\"summary\":\"model\",\"derived\":true,\"w\":{},\"tau_trials_per_p\":{:.4},\"nnz_per_row\":{:.3},\"c3_mults\":{:.0},\"grid\":[{}]}}", w, tau, rw, c3, lines.join(","));
    }
}

/// Arm L (large primes): per-cell aggregates, fitted exponents T ~ A p^alpha per (r, variant),
/// and a derived large-p model built only from those fits and the measured c(3).
pub fn run_dlp(dir: &str) {
    let mut lines: Vec<String> = vec![];
    for e in std::fs::read_dir(dir).unwrap().flatten() {
        let n = e.file_name().to_string_lossy().to_string();
        if n.starts_with("dlp_p") && n.ends_with(".jsonl") {
            lines.extend(std::fs::read_to_string(e.path()).unwrap().lines().map(|s| s.to_string()));
        }
    }
    // key (r*1000, lp) -> p -> list of (reached, T, weight, ops, c3, k_ok, T_square)
    let mut cells: BTreeMap<(u64, u64), BTreeMap<u64, Vec<(bool, f64, f64, f64, f64, bool, Option<f64>)>>> = BTreeMap::new();
    for l in &lines {
        let key = ((num(l, "r").unwrap() * 1000.0).round() as u64, num(l, "lp").unwrap() as u64);
        let p = num(l, "p").unwrap() as u64;
        cells.entry(key).or_default().entry(p).or_default().push((
            l.contains("\"reached\":true"),
            num(l, "T").unwrap(),
            num(l, "mean_row_weight").unwrap(),
            num(l, "merge_ops").unwrap(),
            num(l, "c3").unwrap(),
            l.contains("\"k_correct\":true"),
            num(l, "T_square"),
        ));
    }
    let mut c3_all = vec![];
    let mut fits: BTreeMap<(u64, u64), (f64, f64, f64, f64)> = BTreeMap::new(); // alpha, logA, weight, ops/T
    // Amendment 3: fit both T (target = cut + 21 rows) and T_square (cut + 1 rows); the model uses T_square
    // when every record carries it, since the fixed +20 margin biases the T exponent low at small p.
    let has_sq = lines.iter().all(|l| l.contains("\"T_square\":"));
    for metric in ["T", "T_square"] {
        if metric == "T_square" && !has_sq {
            continue;
        }
        for (&(r, lp), byp) in &cells {
            let mut xs = vec![];
            let mut ys = vec![];
            let mut wts = vec![];
            let mut opt = vec![];
            for (&p, v) in byp {
                if metric == "T" {
                    c3_all.extend(v.iter().map(|x| x.4));
                }
                let vals: Vec<Option<f64>> = v.iter().map(|x| if metric == "T" { if x.0 { Some(x.1) } else { None } } else { x.6 }).collect();
                let all = vals.iter().all(|x| x.is_some());
                let kok = v.iter().filter(|x| x.0).all(|x| x.5);
                let tv: Vec<f64> = v.iter().zip(&vals).map(|(x, y)| y.unwrap_or(x.1)).collect();
                let lt = tv.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64;
                println!(
                    "{{\"summary\":\"dlp_cell\",\"metric\":\"{}\",\"r\":{:.4},\"lp\":{},\"p\":{},\"curves\":{},\"all_reached\":{},\"T_geomean\":{:.1},\"k_correct_all_reached\":{},\"row_weight\":{:.3}}}",
                    metric, r as f64 / 1000.0, lp, p, v.len(), all, lt.exp(), kok, mean(&v.iter().map(|x| x.2).collect::<Vec<_>>())
                );
                if all {
                    xs.push((p as f64).ln());
                    ys.push(lt);
                    wts.push(mean(&v.iter().map(|x| x.2).collect::<Vec<_>>()));
                    opt.push(mean(&v.iter().map(|x| x.3 / x.1).collect::<Vec<_>>()));
                }
            }
            if xs.len() >= 3 {
                let (mx, my) = (mean(&xs), mean(&ys));
                let sxx: f64 = xs.iter().map(|x| (x - mx).powi(2)).sum();
                let sxy: f64 = xs.iter().zip(&ys).map(|(x, y)| (x - mx) * (y - my)).sum();
                let a = sxy / sxx;
                let res: f64 = xs.iter().zip(&ys).map(|(x, y)| (y - my - a * (x - mx)).powi(2)).sum();
                let se = (res / (xs.len() as f64 - 2.0).max(1.0) / sxx).sqrt();
                let w = *wts.last().unwrap();
                let o = *opt.last().unwrap();
                if metric == "T_square" || !has_sq {
                    fits.insert((r, lp), (a, my - a * mx, w, o));
                }
                println!(
                    "{{\"summary\":\"dlp_fit\",\"metric\":\"{}\",\"r\":{:.4},\"lp\":{},\"points\":{},\"p_range\":[{:.0},{:.0}],\"alpha\":{:.4},\"alpha_se\":{:.4},\"zero_lp_law_3_minus_2r\":{:.4},\"row_weight_at_max_p\":{:.3},\"merge_ops_per_trial\":{:.2}}}",
                    metric, r as f64 / 1000.0, lp, xs.len(), xs[0].exp(), xs.last().unwrap().exp(), a, se, 3.0 - 2.0 * r as f64 / 1000.0, w, o
                );
            }
        }
    }
    if fits.is_empty() || c3_all.is_empty() {
        return;
    }
    let c3 = mean(&c3_all);
    for w in [0.1f64, 1.0] {
        let mut grid = vec![];
        let mut crossover: [Option<u32>; 3] = [None; 3];
        for e in (10..=80).step_by(2) {
            let lp_ = (e as f64) * std::f64::consts::LN_2;
            let p = lp_.exp();
            let rho = (std::f64::consts::PI * p.powi(3) / 4.0).sqrt() * (1.0 + w);
            let mut best = [(f64::INFINITY, 0.0f64); 3];
            for (&(r, lp), &(a, la, wt, opt)) in &fits {
                let rr = r as f64 / 1000.0;
                let t = (la + a * lp_).exp();
                let cut = p.powf(rr) / 2.0;
                let cost = t * (1.0 + w * (c3 + opt)) + w * 3.0 * cut * cut * wt;
                if cost < best[lp as usize].0 {
                    best[lp as usize] = (cost, rr);
                }
            }
            let mut parts = vec![];
            for v in 0..3 {
                let ratio = best[v].0 / rho;
                if ratio < 1.0 && crossover[v].is_none() {
                    crossover[v] = Some(3 * e);
                }
                parts.push(format!("{{\"lp\":{},\"r_best\":{:.4},\"ratio\":{:.4e}}}", v, best[v].1, ratio));
            }
            grid.push(format!("{{\"log2p\":{},\"log2_group\":{},\"variants\":[{}]}}", e, 3 * e, parts.join(",")));
        }
        println!(
            "{{\"summary\":\"dlp_model\",\"derived\":true,\"w\":{},\"c3\":{:.0},\"crossover_log2_group\":[{}],\"grid\":[{}]}}",
            w, c3,
            crossover.iter().map(|c| c.map_or("null".into(), |x| x.to_string())).collect::<Vec<_>>().join(","),
            grid.join(",")
        );
    }
}
