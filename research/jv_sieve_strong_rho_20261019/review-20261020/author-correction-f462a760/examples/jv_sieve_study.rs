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
    if let Ok(s) = fs::read_to_string(format!("{dir}/raw/{name}.json")) {
        return serde_json::from_str(&s).ok();
    }
    // per-instance rho jobs: `<name>_s<seed>.json`, merged in seed order
    let mut out: Vec<(u64, Vec<Value>)> = vec![];
    for e in fs::read_dir(format!("{dir}/raw")).ok()?.flatten() {
        let fname = e.file_name().to_string_lossy().to_string();
        let Some(rest) = fname.strip_prefix(&format!("{name}_s")) else {
            continue;
        };
        let Some(seed) = rest
            .strip_suffix(".json")
            .and_then(|x| x.parse::<u64>().ok())
        else {
            continue;
        };
        if !e.path().with_extension("done").exists() {
            continue; // an unfinished job
        }
        let v: Vec<Value> = serde_json::from_str(&fs::read_to_string(e.path()).ok()?).ok()?;
        out.push((seed, v));
    }
    if out.is_empty() {
        return None;
    }
    out.sort_by_key(|x| x.0);
    Some(out.into_iter().flat_map(|x| x.1).collect())
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
    let ci = boot(x.len(), seed, |i| {
        mean(&i.iter().map(|&j| x[j]).collect::<Vec<_>>())
    });
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
    /// native work as the run recorded it
    native_rec: f64,
    /// plus the `hash_x` evaluations the run did not count (derived, see PROTOCOL.md)
    native: f64,
    wall: f64,
    ok: bool,
    correct: bool,
    capped: bool,
}
fn rho_rows(v: &[Value]) -> Vec<Rho> {
    v.iter()
        .map(|r| {
            let native_rec: f64 = [
                "canon_cmp",
                "hash",
                "dp_check",
                "scalar_ops",
                "table_ops",
                "cycle_cmp",
            ]
            .iter()
            .map(|k| fo(r, k))
            .sum();
            // The runs (commit 8f21253f7) counted one `hash_x` per step (the partition
            // hash).  They also evaluated it for the DP check (every arm) and, with the
            // negation map, twice more for the cycle window: 1 or 3 uncounted per step.
            let neg = r["negation"].as_bool() == Some(true);
            let steps = f(r, "steps");
            let correct = r["correct"].as_bool() == Some(true);
            let capped = r["capped"].as_bool() == Some(true);
            Rho {
                seed: r["seed"].as_u64().unwrap(),
                l: r["l"].as_f64().unwrap(),
                mh: f(r, "muls_harness"),
                mt: f(r, "muls_tuned"),
                steps,
                fp_adds: fo(r, "fp_adds"),
                native_rec,
                native: native_rec + steps * if neg { 3.0 } else { 1.0 },
                wall: f(r, "wall_ms"),
                ok: correct && !capped,
                correct,
                capped,
            }
        })
        .collect()
}

/// The frozen sample of each cell (PROTOCOL.md): 16 per arm per size, except R0 at
/// p = 1511 (8) and p = 1823 (4); R1, R2 at p <= 1009; the A/A arms at p = 1009.
fn expected_n(p: u64, arm: &str) -> usize {
    match (arm, p) {
        ("R3", _) => 16,
        ("R0", 1511) => 8,
        ("R0", 1823) => 4,
        ("R0", _) => 16,
        ("R1" | "R2", 251 | 503 | 1009) => 16,
        ("R0_aa" | "R3_aa", 1009) => 16,
        _ => 0,
    }
}

/// OLS slope of ln y on ln x.
fn slope(x: &[f64], y: &[f64]) -> f64 {
    let (lx, ly): (Vec<f64>, Vec<f64>) = (
        x.iter().map(|v| v.ln()).collect(),
        y.iter().map(|v| v.ln()).collect(),
    );
    let (mx, my) = (mean(&lx), mean(&ly));
    lx.iter()
        .zip(&ly)
        .map(|(a, b)| (a - mx) * (b - my))
        .sum::<f64>()
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
                (
                    idx.iter().map(|&i| gx[i]).collect(),
                    idx.iter().map(|&i| gy[i]).collect(),
                )
            })
            .collect();
        out.push(flat(&g));
    }
    json!({"slope": flat(groups), "ci95": [pct(out.clone(), 0.025), pct(out, 0.975)]})
}

fn main() {
    let dir = std::env::args().nth(1).expect("study directory");
    let mut summary = BTreeMap::<String, Value>::new();
    let mut per_size: Vec<Value> = vec![];
    let mut fit_ic: Vec<(u64, Vec<f64>, Vec<f64>)> = vec![];
    let mut fit_r3: Vec<(u64, Vec<f64>, Vec<f64>)> = vec![];
    let mut fit_r3t: Vec<(u64, Vec<f64>, Vec<f64>)> = vec![];
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
            let ic_valid = ic.len() == 16 && ic.iter().all(|r| r.ok);
            entry.insert("ic_n".into(), json!(ic.len()));
            entry.insert("ic_valid_and_complete".into(), json!(ic_valid));
            entry.insert("ic_S".into(), mean_ci(&g(&|r| r.s), sd_ + 1));
            entry.insert("ic_muls".into(), mean_ci(&g(&|r| r.muls), sd_ + 7));
            entry.insert("ic_S_plus".into(), mean_ci(&g(&|r| r.s_plus), sd_ + 2));
            entry.insert("ic_wall_s".into(), mean_ci(&g(&|r| r.wall / 1e3), sd_ + 3));
            // band, layers A (all non-multiplication work) and B (no F_p adds)
            let mut band = vec![];
            for &w in &BAND {
                let a_emit = g(&|r| (r.muls / GAE + w * (r.adds + r.lookups + r.enum_adds)) / sq);
                let a_bound = g(&|r| {
                    (r.muls / GAE + w * (r.adds + r.lookups + r.enum_adds + r.unemitted_bound)) / sq
                });
                let b_emit = g(&|r| (r.muls / GAE + w * r.lookups) / sq);
                let b_bound =
                    g(&|r| (r.muls / GAE + w * (r.lookups + r.unemitted_bound * 0.0)) / sq);
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
                    let same = ic
                        .iter()
                        .zip(&aa)
                        .filter(|(a, b)| a.muls == b.muls && a.digest == b.digest)
                        .count();
                    entry.insert(
                        "ic_aa".into(),
                        json!({"pairs": ic.len().min(aa.len()), "identical_counts": same}),
                    );
                }
            }
            // rho arms
            let mut rho_out = serde_json::Map::new();
            for (name, rs) in &arms {
                let h: Vec<f64> = rs.iter().map(|r| r.mh / (GAE * sq)).collect();
                let t: Vec<f64> = rs.iter().map(|r| r.mt / (GAE * sq)).collect();
                let hp: Vec<f64> = rs
                    .iter()
                    .map(|r| (r.mh + r.fp_adds + r.native) / (GAE * sq))
                    .collect();
                let tp: Vec<f64> = rs
                    .iter()
                    .map(|r| (r.mt + r.fp_adds + r.native) / (GAE * sq))
                    .collect();
                let steps: Vec<f64> = rs.iter().map(|r| r.steps / sq).collect();
                let mut e = serde_json::Map::new();
                let want = expected_n(p, name);
                let n_bad = rs.iter().filter(|r| !r.correct).count();
                let n_capped = rs.iter().filter(|r| r.capped).count();
                let arm_complete = rs.len() == want && n_bad == 0 && n_capped == 0;
                e.insert("all_correct".into(), json!(rs.iter().all(|r| r.ok)));
                e.insert("n_expected".into(), json!(want));
                e.insert("n_incorrect".into(), json!(n_bad));
                e.insert("n_capped".into(), json!(n_capped));
                e.insert(
                    "status".into(),
                    json!(if arm_complete {
                        "complete".to_string()
                    } else {
                        format!("INCOMPLETE OR INVALID: {} of {} runs, {} incorrect, {} capped; comparisons withheld", rs.len(), want, n_bad, n_capped)
                    }),
                );
                e.insert("steps_over_sqrt_l".into(), mean_ci(&steps, sd_ + 10));
                e.insert("S_harness".into(), mean_ci(&h, sd_ + 11));
                e.insert(
                    "muls_harness".into(),
                    mean_ci(&rs.iter().map(|r| r.mh).collect::<Vec<_>>(), sd_ + 16),
                );
                e.insert(
                    "muls_tuned".into(),
                    mean_ci(&rs.iter().map(|r| r.mt).collect::<Vec<_>>(), sd_ + 17),
                );
                e.insert("S_tuned".into(), mean_ci(&t, sd_ + 12));
                e.insert("S_plus_harness".into(), mean_ci(&hp, sd_ + 13));
                e.insert("S_plus_tuned".into(), mean_ci(&tp, sd_ + 14));
                // as the runs recorded it (hash evaluations outside the counter left out)
                let hp_rec: Vec<f64> = rs
                    .iter()
                    .map(|r| (r.mh + r.fp_adds + r.native_rec) / (GAE * sq))
                    .collect();
                e.insert(
                    "S_plus_harness_as_recorded".into(),
                    mean_ci(&hp_rec, sd_ + 18),
                );
                e.insert(
                    "wall_s".into(),
                    mean_ci(
                        &rs.iter().map(|r| r.wall / 1e3).collect::<Vec<_>>(),
                        sd_ + 15,
                    ),
                );
                let mut band = vec![];
                for &w in &BAND {
                    let a: Vec<f64> = rs
                        .iter()
                        .map(|r| (r.mh / GAE + w * (r.native + r.fp_adds)) / sq)
                        .collect();
                    let at: Vec<f64> = rs
                        .iter()
                        .map(|r| (r.mt / GAE + w * (r.native + r.fp_adds)) / sq)
                        .collect();
                    let b: Vec<f64> = rs
                        .iter()
                        .map(|r| (r.mh / GAE + w * r.native) / sq)
                        .collect();
                    let bt: Vec<f64> = rs
                        .iter()
                        .map(|r| (r.mt / GAE + w * r.native) / sq)
                        .collect();
                    band.push(json!({"w": w, "A_harness": mean(&a), "A_tuned": mean(&at), "B_harness": mean(&b), "B_tuned": mean(&bt)}));
                }
                e.insert("band".into(), json!(band));
                // paired ratios against the sieve (same instance seeds)
                let seeds_r: Vec<u64> = rs.iter().map(|r| r.seed).collect();
                let pick = |fx: &dyn Fn(&Ic) -> f64| -> Vec<f64> {
                    ic.iter()
                        .filter(|r| seeds_r.contains(&r.seed))
                        .map(fx)
                        .collect()
                };
                let rho_sorted: Vec<&Rho> = ic
                    .iter()
                    .filter(|r| seeds_r.contains(&r.seed))
                    .map(|r| rs.iter().find(|x| x.seed == r.seed).unwrap())
                    .collect();
                if !rho_sorted.is_empty() && arm_complete && ic_valid {
                    let ic_s = pick(&|r| r.s);
                    let ic_sp = pick(&|r| r.s_plus);
                    let rh: Vec<f64> = rho_sorted.iter().map(|r| r.mh / (GAE * sq)).collect();
                    let rt: Vec<f64> = rho_sorted.iter().map(|r| r.mt / (GAE * sq)).collect();
                    let rhp: Vec<f64> = rho_sorted
                        .iter()
                        .map(|r| (r.mh + r.fp_adds + r.native) / (GAE * sq))
                        .collect();
                    let rtp: Vec<f64> = rho_sorted
                        .iter()
                        .map(|r| (r.mt + r.fp_adds + r.native) / (GAE * sq))
                        .collect();
                    e.insert(
                        "ic_over_rho".into(),
                        json!({
                            "S_harness": ratio_ci(&ic_s, &rh, sd_ + 20),
                            "S_tuned": ratio_ci(&ic_s, &rt, sd_ + 21),
                            "S_plus_harness": ratio_ci(&ic_sp, &rhp, sd_ + 22),
                            "S_plus_tuned": ratio_ci(&ic_sp, &rtp, sd_ + 23),
                        }),
                    );
                    // band ratios, layer A with the IC's unemitted bound (a fortiori rank)
                    let mut br = vec![];
                    for &w in &BAND {
                        let ic_a: Vec<f64> = ic
                            .iter()
                            .filter(|r| seeds_r.contains(&r.seed))
                            .map(|r| {
                                (r.muls / GAE
                                    + w * (r.adds + r.lookups + r.enum_adds + r.unemitted_bound))
                                    / sq
                            })
                            .collect();
                        let ic_b: Vec<f64> = ic
                            .iter()
                            .filter(|r| seeds_r.contains(&r.seed))
                            .map(|r| (r.muls / GAE + w * r.lookups) / sq)
                            .collect();
                        let ra: Vec<f64> = rho_sorted
                            .iter()
                            .map(|r| (r.mh / GAE + w * (r.native + r.fp_adds)) / sq)
                            .collect();
                        let rat: Vec<f64> = rho_sorted
                            .iter()
                            .map(|r| (r.mt / GAE + w * (r.native + r.fp_adds)) / sq)
                            .collect();
                        let rb: Vec<f64> = rho_sorted
                            .iter()
                            .map(|r| (r.mh / GAE + w * r.native) / sq)
                            .collect();
                        let rbt: Vec<f64> = rho_sorted
                            .iter()
                            .map(|r| (r.mt / GAE + w * r.native) / sq)
                            .collect();
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
                    let vd: BTreeMap<&str, &str> =
                        ["S_harness", "S_tuned", "S_plus_harness", "S_plus_tuned"]
                            .iter()
                            .map(|k| (*k, verdict(&io[*k])))
                            .collect();
                    e.insert("verdict".into(), json!(vd));
                }
                rho_out.insert(name.clone(), Value::Object(e));
            }
            if p == 1009 {
                // A/A: the same arm re-run with new walk randomness on the same instances
                let mut aa = serde_json::Map::new();
                for (a, b) in [("R0", "R0_aa"), ("R3", "R3_aa")] {
                    if let (Some(x), Some(y)) = (arms.get(a), arms.get(b)) {
                        let sx: Vec<f64> = x.iter().map(|r| r.mh / (GAE * sq)).collect();
                        let sy: Vec<f64> = y.iter().map(|r| r.mh / (GAE * sq)).collect();
                        let rc = ratio_ci(&sx, &sy, sd_ + 40);
                        let (lo, hi) = (
                            rc["ci95"][0].as_f64().unwrap(),
                            rc["ci95"][1].as_f64().unwrap(),
                        );
                        let pass = x.len() == 16 && y.len() == 16 && lo <= 1.0 && 1.0 <= hi;
                        aa.insert(
                            a.into(),
                            json!({"ratio_first_over_repeat": rc, "pass": pass}),
                        );
                    } else {
                        aa.insert(a.into(), json!({"pass": false, "reason": "arm missing"}));
                    }
                }
                entry.insert("rho_aa".into(), Value::Object(aa));
            }
            entry.insert("rho".into(), Value::Object(rho_out));
            // exponent data
            if ic_valid {
                fit_ic.push((
                    p,
                    ic.iter().map(|r| r.l).collect(),
                    ic.iter().map(|r| r.muls).collect(),
                ));
            }
            if let Some(r3) = arms
                .get("R3")
                .filter(|r| r.len() == expected_n(p, "R3") && r.iter().all(|x| x.ok))
            {
                fit_r3.push((
                    p,
                    r3.iter().map(|r| r.l).collect(),
                    r3.iter().map(|r| r.mh).collect(),
                ));
                fit_r3t.push((
                    p,
                    r3.iter().map(|r| r.l).collect(),
                    r3.iter().map(|r| r.mt).collect(),
                ));
            }
        }
        per_size.push(Value::Object(entry));
    }
    // ---- controls (PROTOCOL.md): every comparative decision is withheld unless they pass
    let groups = |g: &Vec<(u64, Vec<f64>, Vec<f64>)>, min_p: u64| -> Vec<(Vec<f64>, Vec<f64>)> {
        g.iter()
            .filter(|x| x.0 >= min_p)
            .map(|x| (x.1.clone(), x.2.clone()))
            .collect()
    };
    let exp_ic = fit(&groups(&fit_ic, 0), 77);
    let exp_r3 = fit(&groups(&fit_r3, 0), 78);
    let exp_r3t = fit(&groups(&fit_r3t, 0), 79);
    let at = |v: &Value, i: usize| v["ci95"][i].as_f64().unwrap_or(f64::NAN);
    // positive control: the plain arm reproduces the ledger's S ~ 1.3 wherever it ran
    let pos: Vec<Value> = per_size
        .iter()
        .filter_map(|e| {
            let r0 = e["rho"].get("R0")?;
            let (lo, hi) = (at(&r0["S_harness"], 0), at(&r0["S_harness"], 1));
            Some(json!({"p": e["p"], "n": r0["S_harness"]["n"], "ci95": r0["S_harness"]["ci95"],
                        "contains_ledger_1_3": lo <= 1.3 && 1.3 <= hi, "arm_complete": r0["status"] == json!("complete")}))
        })
        .collect();
    let pos_ok = !pos.is_empty()
        && pos
            .iter()
            .all(|x| x["contains_ledger_1_3"] == json!(true) && x["arm_complete"] == json!(true));
    let e1009 = per_size.iter().find(|e| e["p"] == json!(1009));
    let aa_ok = e1009
        .and_then(|e| e.get("rho_aa"))
        .map(|a| ["R0", "R3"].iter().all(|k| a[*k]["pass"] == json!(true)))
        .unwrap_or(false);
    let ic_aa_ok = e1009
        .and_then(|e| e.get("ic_aa"))
        .map(|a| a["pairs"] == json!(16) && a["identical_counts"] == json!(16))
        .unwrap_or(false);
    let digest_ok = per_size.iter().all(|e| {
        e.get("dense_vs_sparse")
            .map(|d| {
                d["pairs"] == json!(16)
                    && d["identical_descent_digest"] == json!(16)
                    && d["identical_outcome"] == json!(16)
            })
            .unwrap_or(false)
    });
    let exp_ok = !exp_r3.is_null() && at(&exp_r3, 0) <= 0.5 && 0.5 <= at(&exp_r3, 1);
    let controls_pass = pos_ok && aa_ok && ic_aa_ok && digest_ok && exp_ok;
    summary.insert(
        "controls".into(),
        json!({"positive_control_plain_rho_reproduces_ledger": {"pass": pos_ok, "rows": pos},
               "rho_A_A_p1009": {"pass": aa_ok}, "sieve_A_A_p1009_identical_counts": {"pass": ic_aa_ok},
               "dense_sparse_identical_descent_all_sizes": {"pass": digest_ok},
               "instrument_exponent_contains_one_half": {"pass": exp_ok},
               "all_pass": controls_pass}),
    );
    if !controls_pass {
        for e in per_size.iter_mut() {
            if let Some(rho) = e["rho"].as_object_mut() {
                for r in rho.values_mut() {
                    if let Some(o) = r.as_object_mut() {
                        o.insert(
                            "verdict".into(),
                            json!("withheld: control validation failed"),
                        );
                        o.insert("ranked_across_band_ic_below_rho".into(), Value::Null);
                    }
                }
            }
        }
    }
    // pre-registered rules (PROTOCOL.md): K1 on the harness S⁺ at p >= 1009, K2 on the
    // tuned price or the band; a size whose arm or sieve sample is not the frozen one,
    // or any run of which is invalid, is withheld, not decided
    let mut k1 = vec![];
    let mut k2 = vec![];
    let mut withheld = vec![];
    for e in &per_size {
        let p = e["p"].as_u64().unwrap();
        if p < 1009 {
            continue;
        }
        let r3 = e["rho"].get("R3");
        let decided = controls_pass
            && r3
                .map(|r| r["status"] == json!("complete") && r.get("ic_over_rho").is_some())
                .unwrap_or(false)
            && e["ic_valid_and_complete"] == json!(true);
        if !decided {
            withheld.push(p);
            continue;
        }
        let r3 = r3.unwrap();
        let hi = |k: &str| r3["ic_over_rho"][k]["ci95"][1].as_f64().unwrap();
        if hi("S_plus_harness") >= 1.0 {
            k1.push(p);
        }
        if hi("S_plus_tuned") >= 1.0 || r3["ranked_across_band_ic_below_rho"] != json!(true) {
            k2.push(p);
        }
    }
    summary.insert("K1_killed_at_p".into(), json!(k1));
    summary.insert("K2_narrowed_at_p".into(), json!(k2));
    summary.insert("K_withheld_at_p".into(), json!(withheld));
    summary.insert("sizes".into(), json!(per_size));
    summary.insert("exponent_ic_muls_vs_l".into(), exp_ic);
    summary.insert("exponent_rho_R3_harness_muls_vs_l".into(), exp_r3);
    summary.insert("exponent_rho_R3_tuned_muls_vs_l".into(), exp_r3t);
    // POST HOC (not in PROTOCOL.md): the sizes where the sieve runs at m = 9, chosen by p
    summary.insert(
        "exponent_ic_muls_vs_l_p_ge_1009_POST_HOC".into(),
        fit(&groups(&fit_ic, 1009), 80),
    );
    summary.insert(
        "exponent_rho_R3_harness_muls_vs_l_p_ge_1009_POST_HOC".into(),
        fit(&groups(&fit_r3, 1009), 81),
    );
    // local exponents between adjacent sizes (means)
    let mut local = vec![];
    for w in fit_ic.windows(2) {
        let (a, b) = (&w[0], &w[1]);
        local.push(
            json!({"p_from": a.0, "p_to": b.0, "from_l": mean(&a.1), "to_l": mean(&b.1),
            "ic": (mean(&b.2) / mean(&a.2)).ln() / (mean(&b.1) / mean(&a.1)).ln()}),
        );
    }
    summary.insert("local_exponents_ic".into(), json!(local));
    summary.insert(
        "provenance".into(),
        json!({"raw_runs": "produced by the jv_cover binary built from commit 8f21253f7 (jv_rho_strong.rs there is identical to later commits up to rustfmt); no run was repeated after code repairs",
               "hash_scope": "the recorded `hash` counter holds the partition hash only; the DP-check hash (all arms) and the two cycle-window hashes (negation arms) were evaluated but not counted. Native work used for S+ and the 5a band adds them, derived from the recorded step count (1 per step plain, 3 per step with negation); S_plus_harness_as_recorded keeps the recorded scope",
               "dp_table": "runs before the repair keyed the DP table by x only; opposite points at one x would have been read as equal. Every recorded run recovered the planted logarithm, so none was affected; the repaired engine stores y and handles both cases"}),
    );
    summary.insert("protocol".into(), json!("PROTOCOL.md"));
    summary.insert("gae_f_p_mults".into(), json!(GAE));
    fs::write(
        format!("{dir}/summary.json"),
        serde_json::to_string_pretty(&summary).unwrap(),
    )
    .unwrap();
    println!("wrote {dir}/summary.json");
}
