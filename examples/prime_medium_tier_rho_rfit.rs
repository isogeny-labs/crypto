//! Finite-size law of a sealed rho bound: fit `R(N) = 1 + c / N^beta` and
//! `R(N) = R_inf + c / N^beta` to the per-size constants of `ecbench.bound/v1`
//! records (research/prime_medium_tier_rho_20261107/PROTOCOL.md, section 6).
//!
//! `R` is a size's mean `S` divided by the method's declared `S` (the
//! registry's leading-order constant the bound records as `declared_s`), and
//! `N = sqrt(r)`. Each size is one point, weighted by the inverse square of
//! the half-width of its committed 95 % interval (two-stage or cluster
//! bootstrap, read from the record, never recomputed here). Uncertainty on
//! the fitted parameters is a parametric bootstrap: every point is redrawn
//! from a normal law with its own standard error, `--resamples` times from a
//! fixed `--seed` (SplitMix64, Box-Muller), and the percentile 95 % interval
//! is reported. `beta` is profiled on a fixed grid, so the fit is exact for
//! the grid and re-derives bit for bit.
//!
//! This reads sealed records only; it runs no solve and counts nothing. It is
//! the analysis step the protocol freezes, not an instrument.
//!
//! ```text
//! cargo run --release --example prime_medium_tier_rho_rfit -- \
//!     --seed 20261107 --resamples 2000 --out fit.json REC1.json REC2.json ...
//! ```

use serde_json::{json, Value};
use std::fs;

#[derive(Clone, Debug)]
struct Point {
    slug: String,
    tier: String,
    log2_r: f64,
    n: f64,
    y: f64,
    se: f64,
    runs: u64,
}

struct SplitMix(u64);
impl SplitMix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn unit(&mut self) -> f64 {
        ((self.next() >> 11) as f64 + 0.5) / (1u64 << 53) as f64
    }
    fn gauss(&mut self) -> f64 {
        let (u, v) = (self.unit(), self.unit());
        (-2.0 * u.ln()).sqrt() * (2.0 * std::f64::consts::PI * v).cos()
    }
}

fn f(v: &Value, k: &str) -> Option<f64> {
    v.get(k).and_then(|x| x.as_f64())
}

fn load(path: &str) -> Result<Vec<Point>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let doc: Value = serde_json::from_str(&text).map_err(|e| format!("{path}: {e}"))?;
    if doc.get("schema").and_then(|s| s.as_str()) != Some("ecbench.bound/v1") {
        return Err(format!("{path}: not an ecbench.bound/v1 record"));
    }
    let tier = doc["domain"]["tier"].as_str().unwrap_or("?").to_string();
    let mut out = Vec::new();
    for s in doc["sizes"].as_array().ok_or(format!("{path}: no sizes"))? {
        let log2_r = f(s, "log2_r").ok_or("size without log2_r")?;
        let mean_s = f(s, "mean_s").ok_or("size without mean_s")?;
        let declared = f(s, "declared_s").ok_or("size without declared_s")?;
        let ci = s["s_ci95"].as_array().ok_or("size without s_ci95")?;
        let (lo, hi) = (ci[0].as_f64().unwrap_or(f64::NAN), ci[1].as_f64().unwrap_or(f64::NAN));
        let se = (hi - lo) / (2.0 * 1.959_963_984_540_054) / declared;
        if !(se.is_finite() && se > 0.0 && declared > 0.0) {
            return Err(format!("{path}: unusable interval at log2 r {log2_r}"));
        }
        out.push(Point {
            slug: s["slug"].as_str().unwrap_or("").to_string(),
            tier: tier.clone(),
            log2_r,
            n: (log2_r / 2.0).exp2(),
            y: mean_s / declared,
            se,
            runs: s["runs"].as_u64().unwrap_or(0),
        });
    }
    Ok(out)
}

const BETA_LO: f64 = 0.10;
const BETA_HI: f64 = 3.00;
const BETA_STEP: f64 = 0.002;

/// Model A, `y = 1 + c z`, `z = N^-beta`: best (chi2, beta, c) on the grid.
fn fit_a(p: &[Point], y: &[f64]) -> (f64, f64, f64) {
    let mut best = (f64::INFINITY, f64::NAN, f64::NAN);
    let steps = ((BETA_HI - BETA_LO) / BETA_STEP).round() as usize;
    for k in 0..=steps {
        let beta = BETA_LO + k as f64 * BETA_STEP;
        let (mut szz, mut szy) = (0.0, 0.0);
        for (q, &yy) in p.iter().zip(y) {
            let (w, z) = (1.0 / (q.se * q.se), q.n.powf(-beta));
            szz += w * z * z;
            szy += w * z * (yy - 1.0);
        }
        let c = szy / szz;
        let chi2: f64 = p
            .iter()
            .zip(y)
            .map(|(q, &yy)| ((yy - 1.0 - c * q.n.powf(-beta)) / q.se).powi(2))
            .sum();
        if chi2 < best.0 {
            best = (chi2, beta, c);
        }
    }
    best
}

/// Model B, `y = a + c z`: best (chi2, beta, a, c) on the grid.
fn fit_b(p: &[Point], y: &[f64]) -> (f64, f64, f64, f64) {
    let mut best = (f64::INFINITY, f64::NAN, f64::NAN, f64::NAN);
    let steps = ((BETA_HI - BETA_LO) / BETA_STEP).round() as usize;
    for k in 0..=steps {
        let beta = BETA_LO + k as f64 * BETA_STEP;
        let (mut sw, mut sz, mut szz, mut sy, mut szy) = (0.0, 0.0, 0.0, 0.0, 0.0);
        for (q, &yy) in p.iter().zip(y) {
            let (w, z) = (1.0 / (q.se * q.se), q.n.powf(-beta));
            sw += w;
            sz += w * z;
            szz += w * z * z;
            sy += w * yy;
            szy += w * z * yy;
        }
        let det = sw * szz - sz * sz;
        if det.abs() < 1e-300 {
            continue;
        }
        let c = (sw * szy - sz * sy) / det;
        let a = (sy - c * sz) / sw;
        let chi2: f64 = p
            .iter()
            .zip(y)
            .map(|(q, &yy)| ((yy - a - c * q.n.powf(-beta)) / q.se).powi(2))
            .sum();
        if chi2 < best.0 {
            best = (chi2, beta, a, c);
        }
    }
    best
}

/// Regularized upper incomplete gamma Q(s, x) (Numerical Recipes 6.2).
fn gamma_q(s: f64, x: f64) -> f64 {
    fn ln_gamma(z: f64) -> f64 {
        let g = [
            76.180_091_729_471_46,
            -86.505_320_329_416_77,
            24.014_098_240_830_91,
            -1.231_739_572_450_155,
            0.001_208_650_973_866_179,
            -0.000_005_395_239_384_953,
        ];
        let mut y = z;
        let tmp = z + 5.5 - (z + 0.5) * (z + 5.5).ln();
        let mut ser = 1.000_000_000_190_015;
        for c in g {
            y += 1.0;
            ser += c / y;
        }
        -tmp + (2.506_628_274_631_000_5 * ser / z).ln()
    }
    if x <= 0.0 {
        return 1.0;
    }
    if x < s + 1.0 {
        let (mut sum, mut del, mut ap) = (1.0 / s, 1.0 / s, s);
        for _ in 0..1000 {
            ap += 1.0;
            del *= x / ap;
            sum += del;
            if del.abs() < sum.abs() * 1e-15 {
                break;
            }
        }
        1.0 - sum * (-x + s * x.ln() - ln_gamma(s)).exp()
    } else {
        let tiny = 1e-300;
        let mut b = x + 1.0 - s;
        let mut c = 1.0 / tiny;
        let mut d = 1.0 / b;
        let mut h = d;
        for i in 1..1000 {
            let an = -(i as f64) * (i as f64 - s);
            b += 2.0;
            d = an * d + b;
            if d.abs() < tiny {
                d = tiny;
            }
            c = b + an / c;
            if c.abs() < tiny {
                c = tiny;
            }
            d = 1.0 / d;
            let del = d * c;
            h *= del;
            if (del - 1.0).abs() < 1e-15 {
                break;
            }
        }
        (-x + s * x.ln() - ln_gamma(s)).exp() * h
    }
}

fn pct(v: &mut [f64], q: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let i = ((v.len() - 1) as f64 * q).round() as usize;
    v[i]
}

fn main() {
    let mut args = std::env::args().skip(1);
    let (mut seed, mut resamples, mut out, mut files) = (20261107u64, 2000usize, None, vec![]);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--seed" => seed = args.next().and_then(|s| s.parse().ok()).expect("--seed N"),
            "--resamples" => {
                resamples = args.next().and_then(|s| s.parse().ok()).expect("--resamples N")
            }
            "--out" => out = args.next(),
            _ => files.push(a),
        }
    }
    if files.is_empty() {
        eprintln!("usage: prime_medium_tier_rho_rfit [--seed N] [--resamples N] [--out F] RECORD...");
        std::process::exit(2);
    }
    let mut p = Vec::new();
    for path in &files {
        match load(path) {
            Ok(mut v) => p.append(&mut v),
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(2);
            }
        }
    }
    p.sort_by(|a, b| a.log2_r.partial_cmp(&b.log2_r).unwrap());
    let y: Vec<f64> = p.iter().map(|q| q.y).collect();
    let (chi_a, beta_a, c_a) = fit_a(&p, &y);
    let (chi_b, beta_b, a_b, c_b) = fit_b(&p, &y);
    let dof_a = p.len() as f64 - 2.0;
    let dof_b = p.len() as f64 - 3.0;
    let mut rng = SplitMix(seed);
    let (mut bs_ca, mut bs_ba, mut bs_ab, mut bs_cb, mut bs_bb) =
        (vec![], vec![], vec![], vec![], vec![]);
    for _ in 0..resamples {
        let ys: Vec<f64> = p.iter().map(|q| q.y + q.se * rng.gauss()).collect();
        let (_, ba, ca) = fit_a(&p, &ys);
        let (_, bb, ab, cb) = fit_b(&p, &ys);
        bs_ca.push(ca);
        bs_ba.push(ba);
        bs_ab.push(ab);
        bs_cb.push(cb);
        bs_bb.push(bb);
    }
    let ci = |v: &mut Vec<f64>| json!([pct(v, 0.025), pct(v, 0.975)]);
    let medium: Vec<&Point> = p.iter().filter(|q| q.tier == "medium").collect();
    let toy: Vec<&Point> = p.iter().filter(|q| q.tier == "toy").collect();
    let wmean = |v: &[&Point]| {
        let (sw, swy) = v
            .iter()
            .fold((0.0, 0.0), |(a, b), q| (a + 1.0 / (q.se * q.se), b + q.y / (q.se * q.se)));
        if sw > 0.0 {
            json!({"value": swy / sw, "se": (1.0 / sw).sqrt(), "points": v.len()})
        } else {
            Value::Null
        }
    };
    let report = json!({
        "schema": "prime_medium_tier_rho.rfit/v1",
        "what_this_is": "Weighted least-squares fits of R(N) = 1 + c N^-beta (model A, the lead's form) and R(N) = R_inf + c N^-beta (model B) to the per-size R = mean S / declared S of the named bound records, N = sqrt(r); parametric bootstrap intervals. Analysis of sealed records; no solve.",
        "inputs": files,
        "seed": seed,
        "resamples": resamples,
        "beta_grid": {"lo": BETA_LO, "hi": BETA_HI, "step": BETA_STEP},
        "points": p.iter().map(|q| json!({"slug": q.slug, "tier": q.tier, "log2_r": q.log2_r,
            "R": q.y, "se": q.se, "runs": q.runs})).collect::<Vec<_>>(),
        "model_a": {"form": "R = 1 + c * N^-beta", "c": c_a, "beta": beta_a,
            "c_ci95": ci(&mut bs_ca), "beta_ci95": ci(&mut bs_ba),
            "chi2": chi_a, "dof": dof_a, "p_value": gamma_q(dof_a / 2.0, chi_a / 2.0)},
        "model_b": {"form": "R = R_inf + c * N^-beta", "r_inf": a_b, "c": c_b, "beta": beta_b,
            "r_inf_ci95": ci(&mut bs_ab), "c_ci95": ci(&mut bs_cb), "beta_ci95": ci(&mut bs_bb),
            "chi2": chi_b, "dof": dof_b, "p_value": gamma_q(dof_b / 2.0, chi_b / 2.0)},
        "weighted_mean_R": {"toy": wmean(&toy), "medium": wmean(&medium)},
    });
    let text = serde_json::to_string_pretty(&report).unwrap() + "\n";
    match out {
        Some(path) => fs::write(&path, &text).expect("write --out"),
        None => print!("{text}"),
    }
}
