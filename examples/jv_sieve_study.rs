//! Analysis of `research/jv_sieve_strong_rho_20261019` (PROTOCOL.md): reads the
//! raw runs of `jv_cover --exp sieve` and `--exp rhostrong` and writes
//! `summary.json`: per-size means with bootstrap intervals, paired ratios of the
//! sieve to each rho arm, the §5a pricing band, the dense/sparse pairing and the
//! exponent fit.  Computation lives here, in Rust; plotting only reads the JSON.
//!
//! ```text
//! cargo run --release --example jv_sieve_study -- research/jv_sieve_strong_rho_20261019
//! ```

use std::collections::BTreeMap;
use std::fs;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};

const GAE: f64 = 331.0;
const BAND: [f64; 3] = [0.0, 0.1, 1.0];
const SIZES: [u64; 5] = [251, 503, 1009, 1511, 1823];
const B: usize = 10_000;

fn load(dir: &str, name: &str) -> Option<Vec<Value>> {
    let s = fs::read_to_string(format!("{dir}/raw/{name}.json")).ok()?;
    serde_json::from_str(&s).ok()
}
fn f(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap_or(f64::NAN)
}
fn fo(v: &Value, k: &str) -> f64 {
    v["ops"][k].as_f64().unwrap_or(0.0)
}
fn mean(x: &[f64]) -> f64 {
    x.iter().sum::<f64>() / x.len() as f64
}
fn sd(x: &[f64]) -> f64 {
    if x.len() < 2 {
        return 0.0;
    }
    let m = mean(x);
    (x.iter().map(|v| (v - m).powi(2)).sum::<f64>() / (x.len() - 1) as f64).sqrt()
}
fn pct(mut v: Vec<f64>, q: f64) -> f64 {
    v.sort_by(|a, b| a.total_cmp(b));
    v[((v.len() - 1) as f64 * q).round() as usize]
}

/// Bootstrap interval of `stat` over index resamples of `n` items.
fn boot<F: Fn(&[usize]) -> f64>(n: usize, seed: u64, stat: F) -> [f64; 2] {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut out = Vec::with_capacity(B);
    for _ in 0..B {
        let idx: Vec<usize> = (0..n).map(|_| rng.gen_range(0..n)).collect();
        out.push(stat(&idx));
    }
    [pct(out.clone(), 0.025), pct(out, 0.975)]
}
fn mean_ci(x: &[f64], seed: u64) -> Value {
    let ci = boot(x.len(), seed, |i| mean(&i.iter().map(|&j| x[j]).collect::<Vec<_>>()));
    json!({"n": x.len(), "mean": mean(x), "sd": sd(x), "ci95": ci})
}
/// "IC<rho" when the whole interval of IC/rho lies below 1, "rho<IC" when above,
/// otherwise "unresolved" (AGENTS.md §5a: never a win for either side).
fn verdict(r: &Value) -> &'static str {
    let ci = &r["ci95"];
    let (lo, hi) = (ci[0].as_f64().unwrap(), ci[1].as_f64().unwrap());
    if hi < 1.0 {
        "IC<rho"
    } else if lo > 1.0 {
        "rho<IC"
    } else {
        "unresolved"
    }
}
fn ratio_ci(a: &[f64], b: &[f64], seed: u64) -> Value {
    // paired by position (same instance seeds)
    let n = a.len().min(b.len());
    let r = |i: &[usize]| {
        mean(&i.iter().map(|&j| a[j]).collect::<Vec<_>>())
            / mean(&i.iter().map(|&j| b[j]).collect::<Vec<_>>())
    };
    let all: Vec<usize> = (0..n).collect();
    json!({"n": n, "ratio": r(&all), "ci95": boot(n, seed, r)})
}

struct Ic {
    seed: u64,
    l: f64,
    muls: f64,
    s: f64,
    s_plus: f64,
    adds: f64,
    lookups: f64,
    enum_adds: f64,
    unemitted_bound: f64,
    wall: f64,
    lin_muls: f64,
    descent_muls: f64,
    digest: u64,
    ok: bool,
    fallback: f64,
}
fn ic_rows(v: &[Value]) -> Vec<Ic> {
    v.iter()
        .map(|r| {
            let muls = f(r, "total_muls");
            Ic {
                seed: r["seed"].as_u64().unwrap(),
                l: r["l"].as_f64().unwrap(),
                muls,
                s: f(r, "s"),
                s_plus: f(r, "s_plus"),
                adds: f(r, "sieve_adds"),
                lookups: f(r, "sieve_lookups"),
                enum_adds: f(r, "enum_adds"),
                unemitted_bound: muls - f(r, "sieve_muls") - f(r, "enum_muls"),
                wall: f(r, "wall_ms"),
                lin_muls: f(r, "descent_lin_muls"),
                descent_muls: f(r, "descent_muls"),
                digest: r["descent_digest"].as_u64().unwrap_or(0),
                ok: r["solved"].as_bool() == Some(true) && r["correct"].as_bool() == Some(true),
                fallback: f(r, "descent_lin_fallback"),
            }
        })
        .collect()
}

struct Rho {
    seed: u64,
    l: f64,
    mh: f64,
    mt: f64,
    steps: f64,
    fp_adds: f64,
    native: f64,
    wall: f64,
    ok: bool,
}
fn rho_rows(v: &[Value]) -> Vec<Rho> {
    v.iter()
        .map(|r| Rho {
            seed: r["seed"].as_u64().unwrap(),
            l: r["l"].as_f64().unwrap(),
            mh: f(r, "muls_harness"),
            mt: f(r, "muls_tuned"),
            steps: f(r, "steps"),
            fp_adds: fo(r, "fp_adds"),
            native: ["canon_cmp", "hash", "dp_check", "scalar_ops", "table_ops", "cycle_cmp"]
                .iter()
                .map(|k| fo(r, k))
                .sum(),
            wall: f(r, "wall_ms"),
            ok: r["correct"].as_bool() == Some(true) && r["capped"].as_bool() != Some(true),
        })
        .collect()
}

/// OLS slope of ln y on ln x.
fn slope(x: &[f64], y: &[f64]) -> f64 {
    let (lx, ly): (Vec<f64>, Vec<f64>) = (x.iter().map(|v| v.ln()).collect(), y.iter().map(|v| v.ln()).collect());
    let (mx, my) = (mean(&lx), mean(&ly));
    lx.iter().zip(&ly).map(|(a, b)| (a - mx) * (b - my)).sum::<f64>()
        / lx.iter().map(|a| (a - mx).powi(2)).sum::<f64>()
}
/// Slope with a stratified bootstrap (instances resampled within each size).
fn fit(groups: &[(Vec<f64>, Vec<f64>)], seed: u64) -> Value {
    if groups.len() < 2 {
        return Value::Null;
    }
    let flat = |g: &[(Vec<f64>, Vec<f64>)]| {
        let mut x = vec![];
        let mut y = vec![];
        for (gx, gy) in g {
            x.extend(gx);
            y.extend(gy);
        }
        slope(&x, &y)
    };
    let mut rng = StdRng::seed_from_u64(seed);
    let mut out = vec![];
    for _ in 0..B {
        let g: Vec<(Vec<f64>, Vec<f64>)> = groups
            .iter()
            .map(|(gx, gy)| {
                let idx: Vec<usize> = (0..gx.len()).map(|_| rng.gen_range(0..gx.len())).collect();
                (idx.iter().map(|&i| gx[i]).collect(), idx.iter().map(|&i| gy[i]).collect())
            })
            .collect();
        out.push(flat(&g));
    }
    json!({"slope": flat(groups), "ci95": [pct(out.clone(), 0.025), pct(out, 0.975)]})
}

fn main() {
    let dir = std::env::args().nth(1).expect("study directory");
    let mut summary = BTreeMap::<String, Value>::new();
    let mut per_size = vec![];
    let mut fit_ic: Vec<(Vec<f64>, Vec<f64>)> = vec![];
    let mut fit_r3: Vec<(Vec<f64>, Vec<f64>)> = vec![];
    let mut fit_r3t: Vec<(Vec<f64>, Vec<f64>)> = vec![];
    for &p in &SIZES {
        let sd_ = p;
        let sparse = load(&dir, &format!("ic_{p}_sparse")).map(|v| ic_rows(&v));
        let dense = load(&dir, &format!("ic_{p}_dense")).map(|v| ic_rows(&v));
        let mut arms: BTreeMap<String, Vec<Rho>> = BTreeMap::new();
        for a in ["R0", "R1", "R2", "R3", "R0_aa", "R3_aa"] {
            if let Some(v) = load(&dir, &format!("rho_{p}_{a}")) {
                arms.insert(a.into(), rho_rows(&v));
            }
        }
        let mut entry = serde_json::Map::new();
        entry.insert("p".into(), json!(p));
        if let Some(ic) = &sparse {
            let l = ic[0].l;
            entry.insert("l".into(), json!(l));
            entry.insert("log2_l".into(), json!(l.log2()));
            let sq = l.sqrt();
            let g = |fx: &dyn Fn(&Ic) -> f64| ic.iter().map(fx).collect::<Vec<f64>>();
            entry.insert("ic_all_correct".into(), json!(ic.iter().all(|r| r.ok)));
            entry.insert("ic_S".into(), mean_ci(&g(&|r| r.s), sd_ + 1));
            entry.insert("ic_muls".into(), mean_ci(&g(&|r| r.muls), sd_ + 7));
            entry.insert("ic_S_plus".into(), mean_ci(&g(&|r| r.s_plus), sd_ + 2));
            entry.insert("ic_wall_s".into(), mean_ci(&g(&|r| r.wall / 1e3), sd_ + 3));
            // band, layers A (all non-multiplication work) and B (no F_p adds)
            let mut band = vec![];
            for &w in &BAND {
                let a_emit = g(&|r| (r.muls / GAE + w * (r.adds + r.lookups + r.enum_adds)) / sq);
                let a_bound = g(&|r| (r.muls / GAE + w * (r.adds + r.lookups + r.enum_adds + r.unemitted_bound)) / sq);
                let b_emit = g(&|r| (r.muls / GAE + w * r.lookups) / sq);
                let b_bound = g(&|r| (r.muls / GAE + w * (r.lookups + r.unemitted_bound * 0.0)) / sq);
                band.push(json!({"w": w, "A_emitted": mean(&a_emit), "A_with_unemitted_bound": mean(&a_bound),
                                 "B_emitted": mean(&b_emit), "B": mean(&b_bound)}));
            }
            entry.insert("ic_band".into(), json!(band));
            if let Some(dn) = &dense {
                let n = ic.len().min(dn.len());
                let s_d: Vec<f64> = dn.iter().take(n).map(|r| r.s).collect();
                let s_s: Vec<f64> = ic.iter().take(n).map(|r| r.s).collect();
                let lin_d: Vec<f64> = dn.iter().take(n).map(|r| r.lin_muls).collect();
                let lin_s: Vec<f64> = ic.iter().take(n).map(|r| r.lin_muls).collect();
                entry.insert(
                    "dense_vs_sparse".into(),
                    json!({
                        "pairs": n,
                        "identical_descent_digest": (0..n).filter(|&i| dn[i].digest == ic[i].digest).count(),
                        "identical_outcome": (0..n).filter(|&i| dn[i].ok == ic[i].ok).count(),
                        "S_dense_over_sparse": ratio_ci(&s_d, &s_s, sd_ + 4),
                        "linear_stage_dense_over_sparse": ratio_ci(&lin_d, &lin_s, sd_ + 5),
                        "descent_muls_dense_over_sparse": ratio_ci(
                            &dn.iter().take(n).map(|r| r.descent_muls).collect::<Vec<_>>(),
                            &ic.iter().take(n).map(|r| r.descent_muls).collect::<Vec<_>>(), sd_ + 6),
                        "sparse_fallbacks_per_run": mean(&ic.iter().map(|r| r.fallback).collect::<Vec<_>>()),
                        "S_dense": mean(&s_d), "S_sparse": mean(&s_s),
                    }),
                );
            }
            if p == 1009 {
                if let Some(aa) = load(&dir, "ic_1009_sparse_aa").map(|v| ic_rows(&v)) {
                    let same = ic.iter().zip(&aa).filter(|(a, b)| a.muls == b.muls && a.digest == b.digest).count();
                    entry.insert("ic_aa".into(), json!({"pairs": ic.len().min(aa.len()), "identical_counts": same}));
                }
            }
            // rho arms
            let mut rho_out = serde_json::Map::new();
            for (name, rs) in &arms {
                let h: Vec<f64> = rs.iter().map(|r| r.mh / (GAE * sq)).collect();
                let t: Vec<f64> = rs.iter().map(|r| r.mt / (GAE * sq)).collect();
                let hp: Vec<f64> = rs.iter().map(|r| (r.mh + r.fp_adds + r.native) / (GAE * sq)).collect();
                let tp: Vec<f64> = rs.iter().map(|r| (r.mt + r.fp_adds + r.native) / (GAE * sq)).collect();
                let steps: Vec<f64> = rs.iter().map(|r| r.steps / sq).collect();
                let mut e = serde_json::Map::new();
                e.insert("all_correct".into(), json!(rs.iter().all(|r| r.ok)));
                e.insert("steps_over_sqrt_l".into(), mean_ci(&steps, sd_ + 10));
                e.insert("S_harness".into(), mean_ci(&h, sd_ + 11));
                e.insert("muls_harness".into(), mean_ci(&rs.iter().map(|r| r.mh).collect::<Vec<_>>(), sd_ + 16));
                e.insert("muls_tuned".into(), mean_ci(&rs.iter().map(|r| r.mt).collect::<Vec<_>>(), sd_ + 17));
                e.insert("S_tuned".into(), mean_ci(&t, sd_ + 12));
                e.insert("S_plus_harness".into(), mean_ci(&hp, sd_ + 13));
                e.insert("S_plus_tuned".into(), mean_ci(&tp, sd_ + 14));
                e.insert("wall_s".into(), mean_ci(&rs.iter().map(|r| r.wall / 1e3).collect::<Vec<_>>(), sd_ + 15));
                let mut band = vec![];
                for &w in &BAND {
                    let a: Vec<f64> = rs.iter().map(|r| (r.mh / GAE + w * (r.native + r.fp_adds)) / sq).collect();
                    let at: Vec<f64> = rs.iter().map(|r| (r.mt / GAE + w * (r.native + r.fp_adds)) / sq).collect();
                    let b: Vec<f64> = rs.iter().map(|r| (r.mh / GAE + w * r.native) / sq).collect();
                    let bt: Vec<f64> = rs.iter().map(|r| (r.mt / GAE + w * r.native) / sq).collect();
                    band.push(json!({"w": w, "A_harness": mean(&a), "A_tuned": mean(&at), "B_harness": mean(&b), "B_tuned": mean(&bt)}));
                }
                e.insert("band".into(), json!(band));
                // paired ratios against the sieve (same instance seeds)
                let seeds_r: Vec<u64> = rs.iter().map(|r| r.seed).collect();
                let pick = |fx: &dyn Fn(&Ic) -> f64| -> Vec<f64> {
                    ic.iter().filter(|r| seeds_r.contains(&r.seed)).map(fx).collect()
                };
                let rho_sorted: Vec<&Rho> = ic
                    .iter()
                    .filter(|r| seeds_r.contains(&r.seed))
                    .map(|r| rs.iter().find(|x| x.seed == r.seed).unwrap())
                    .collect();
                if !rho_sorted.is_empty() {
                    let ic_s = pick(&|r| r.s);
                    let ic_sp = pick(&|r| r.s_plus);
                    let rh: Vec<f64> = rho_sorted.iter().map(|r| r.mh / (GAE * sq)).collect();
                    let rt: Vec<f64> = rho_sorted.iter().map(|r| r.mt / (GAE * sq)).collect();
                    let rhp: Vec<f64> = rho_sorted.iter().map(|r| (r.mh + r.fp_adds + r.native) / (GAE * sq)).collect();
                    let rtp: Vec<f64> = rho_sorted.iter().map(|r| (r.mt + r.fp_adds + r.native) / (GAE * sq)).collect();
                    e.insert("ic_over_rho".into(), json!({
                        "S_harness": ratio_ci(&ic_s, &rh, sd_ + 20),
                        "S_tuned": ratio_ci(&ic_s, &rt, sd_ + 21),
                        "S_plus_harness": ratio_ci(&ic_sp, &rhp, sd_ + 22),
                        "S_plus_tuned": ratio_ci(&ic_sp, &rtp, sd_ + 23),
                    }));
                    // band ratios, layer A with the IC's unemitted bound (a fortiori rank)
                    let mut br = vec![];
                    for &w in &BAND {
                        let ic_a: Vec<f64> = ic.iter().filter(|r| seeds_r.contains(&r.seed))
                            .map(|r| (r.muls / GAE + w * (r.adds + r.lookups + r.enum_adds + r.unemitted_bound)) / sq).collect();
                        let ic_b: Vec<f64> = ic.iter().filter(|r| seeds_r.contains(&r.seed))
                            .map(|r| (r.muls / GAE + w * r.lookups) / sq).collect();
                        let ra: Vec<f64> = rho_sorted.iter().map(|r| (r.mh / GAE + w * (r.native + r.fp_adds)) / sq).collect();
                        let rat: Vec<f64> = rho_sorted.iter().map(|r| (r.mt / GAE + w * (r.native + r.fp_adds)) / sq).collect();
                        let rb: Vec<f64> = rho_sorted.iter().map(|r| (r.mh / GAE + w * r.native) / sq).collect();
                        let rbt: Vec<f64> = rho_sorted.iter().map(|r| (r.mt / GAE + w * r.native) / sq).collect();
                        br.push(json!({"w": w,
                            "A_harness": ratio_ci(&ic_a, &ra, sd_ + 30), "A_tuned": ratio_ci(&ic_a, &rat, sd_ + 31),
                            "B_harness": ratio_ci(&ic_b, &rb, sd_ + 32), "B_tuned": ratio_ci(&ic_b, &rbt, sd_ + 33)}));
                    }
                    let mut all_below = true;
                    for b in &br {
                        for k in ["A_harness", "A_tuned", "B_harness", "B_tuned"] {
                            all_below &= verdict(&b[k]) == "IC<rho";
                        }
                    }
                    e.insert("band_ratio_ic_over_rho".into(), json!(br));
                    e.insert("ranked_across_band_ic_below_rho".into(), json!(all_below));
                    let io = &e["ic_over_rho"];
                    let vd: BTreeMap<&str, &str> = ["S_harness", "S_tuned", "S_plus_harness", "S_plus_tuned"]
                        .iter().map(|k| (*k, verdict(&io[*k]))).collect();
                    e.insert("verdict".into(), json!(vd));
                }
                rho_out.insert(name.clone(), Value::Object(e));
            }
            entry.insert("rho".into(), Value::Object(rho_out));
            // exponent data
            fit_ic.push((ic.iter().map(|r| r.l).collect(), ic.iter().map(|r| r.muls).collect()));
            if let Some(r3) = arms.get("R3") {
                fit_r3.push((r3.iter().map(|r| r.l).collect(), r3.iter().map(|r| r.mh).collect()));
                fit_r3t.push((r3.iter().map(|r| r.l).collect(), r3.iter().map(|r| r.mt).collect()));
            }
        }
        per_size.push(Value::Object(entry));
    }
    // pre-registered rules (PROTOCOL.md): K1 on the harness S⁺ at p >= 1009, K2 on the
    // tuned price or the band
    let mut k1 = vec![];
    let mut k2 = vec![];
    for e in &per_size {
        let p = e["p"].as_u64().unwrap();
        let Some(r3) = e["rho"].get("R3") else { continue };
        if p < 1009 {
            continue;
        }
        let hi = |k: &str| r3["ic_over_rho"][k]["ci95"][1].as_f64().unwrap();
        if hi("S_plus_harness") >= 1.0 {
            k1.push(p);
        }
        if hi("S_plus_tuned") >= 1.0 || e["ranked_across_band_ic_below_rho"] != json!(true) {
            k2.push(p);
        }
    }
    summary.insert("K1_killed_at_p".into(), json!(k1));
    summary.insert("K2_narrowed_at_p".into(), json!(k2));
    summary.insert("sizes".into(), json!(per_size));
    summary.insert("exponent_ic_muls_vs_l".into(), fit(&fit_ic, 77));
    summary.insert("exponent_rho_R3_harness_muls_vs_l".into(), fit(&fit_r3, 78));
    summary.insert("exponent_rho_R3_tuned_muls_vs_l".into(), fit(&fit_r3t, 79));
    // POST HOC (not in PROTOCOL.md): the sizes where the sieve runs at m = 9 (p >= 1009)
    let tail = |g: &Vec<(Vec<f64>, Vec<f64>)>| g.iter().skip(g.len().saturating_sub(3)).cloned().collect::<Vec<_>>();
    summary.insert("exponent_ic_muls_vs_l_p_ge_1009_POST_HOC".into(), fit(&tail(&fit_ic), 80));
    summary.insert("exponent_rho_R3_harness_muls_vs_l_p_ge_1009_POST_HOC".into(), fit(&tail(&fit_r3), 81));
    // local exponents between adjacent sizes (means)
    let mut local = vec![];
    for w in fit_ic.windows(2) {
        let (a, b) = (&w[0], &w[1]);
        local.push(json!({"from_l": mean(&a.0), "to_l": mean(&b.0),
            "ic": (mean(&b.1) / mean(&a.1)).ln() / (mean(&b.0) / mean(&a.0)).ln()}));
    }
    summary.insert("local_exponents_ic".into(), json!(local));
    summary.insert("protocol".into(), json!("PROTOCOL.md"));
    summary.insert("gae_f_p_mults".into(), json!(GAE));
    fs::write(format!("{dir}/summary.json"), serde_json::to_string_pretty(&summary).unwrap()).unwrap();
    println!("wrote {dir}/summary.json");
}
