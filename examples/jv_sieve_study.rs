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

fn is_posthoc_tail_size(p: u64) -> bool {
    p >= 1009
}

fn k2_descriptive_band_result(size: &Value) -> Option<bool> {
    size.get("rho")?
        .get("R3")?
        .get("descriptive_band_interval_all_below")?
        .as_bool()
}

fn load(dir: &str, name: &str) -> Option<Vec<Value>> {
    if let Ok(s) = fs::read_to_string(format!("{dir}/raw/{name}.json")) {
        return serde_json::from_str::<Vec<Value>>(&s)
            .ok()
            .filter(|rows| !rows.is_empty());
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
    let rows: Vec<Value> = out.into_iter().flat_map(|x| x.1).collect();
    (!rows.is_empty()).then_some(rows)
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
    let (Some(lo), Some(hi)) = (ci[0].as_f64(), ci[1].as_f64()) else {
        return "unavailable";
    };
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

fn seed_inventory(seeds: impl Iterator<Item = u64>, expected: &[u64]) -> Value {
    let mut counts = BTreeMap::<u64, usize>::new();
    for seed in seeds {
        *counts.entry(seed).or_default() += 1;
    }
    let observed: Vec<u64> = counts.keys().copied().collect();
    let missing: Vec<u64> = expected
        .iter()
        .copied()
        .filter(|s| !counts.contains_key(s))
        .collect();
    let duplicates: Vec<u64> = counts
        .iter()
        .filter_map(|(&s, &n)| (n > 1).then_some(s))
        .collect();
    json!({"expected": expected, "observed": observed, "missing": missing,
           "duplicates": duplicates, "complete": missing.is_empty() && duplicates.is_empty()
               && counts.keys().all(|s| expected.contains(s))})
}

// Frozen run.sh schedules A/A run=1 with sixteen instance seeds, not one seed.
fn expected_rho_seeds(p: u64, arm: &str) -> Vec<u64> {
    let n = match (arm, p) {
        ("R0_aa" | "R3_aa", 1009) => 16,
        ("R0_aa" | "R3_aa", _) => 0,
        ("R1" | "R2", 1511 | 1823) => 0,
        ("R0", 1511) => 8,
        ("R0", 1823) => 4,
        _ => 16,
    };
    (1..=n).collect()
}

fn paired_indices<A, C>(
    left: &[A],
    right: &[C],
    left_seed: impl Fn(&A) -> u64,
    right_seed: impl Fn(&C) -> u64,
) -> Vec<(usize, usize)> {
    let mut right_by_seed = BTreeMap::new();
    for (i, row) in right.iter().enumerate() {
        right_by_seed.entry(right_seed(row)).or_insert(i);
    }
    let mut seen = std::collections::BTreeSet::new();
    left.iter()
        .enumerate()
        .filter_map(|(i, row)| {
            let seed = left_seed(row);
            if !seen.insert(seed) {
                return None;
            }
            right_by_seed.get(&seed).copied().map(|j| (i, j))
        })
        .collect()
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
                seed: r["seed"].as_u64().unwrap_or(0),
                l: f(r, "l"),
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
                ok: r["solved"].as_bool() == Some(true)
                    && r["correct"].as_bool() == Some(true)
                    && r["seed"].as_u64().is_some()
                    && muls.is_finite()
                    && muls > 0.0,
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
            seed: r["seed"].as_u64().unwrap_or(0),
            l: f(r, "l"),
            mh: f(r, "muls_harness"),
            mt: f(r, "muls_tuned"),
            steps: f(r, "steps"),
            fp_adds: fo(r, "fp_adds"),
            native: [
                "canon_cmp",
                "hash",
                "dp_check",
                "scalar_ops",
                "table_ops",
                "cycle_cmp",
            ]
            .iter()
            .map(|k| fo(r, k))
            .sum(),
            wall: f(r, "wall_ms"),
            ok: r["correct"].as_bool() == Some(true)
                && r["capped"].as_bool() == Some(false)
                && r["seed"].as_u64().is_some()
                && f(r, "muls_harness").is_finite()
                && f(r, "muls_harness") > 0.0
                && f(r, "muls_tuned").is_finite()
                && f(r, "muls_tuned") > 0.0,
        })
        .collect()
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
    let mut per_size = vec![];
    let mut fit_ic: Vec<(Vec<f64>, Vec<f64>)> = vec![];
    let mut fit_ic_tail = vec![];
    let mut fit_r3_tail = vec![];
    let mut fit_r3: Vec<(Vec<f64>, Vec<f64>)> = vec![];
    let mut fit_r3t: Vec<(Vec<f64>, Vec<f64>)> = vec![];
    for &p in &SIZES {
        let sd_ = p;
        let sparse: Option<Vec<Ic>> = load(&dir, &format!("ic_{p}_sparse")).map(|v| ic_rows(&v));
        let dense: Option<Vec<Ic>> = load(&dir, &format!("ic_{p}_dense")).map(|v| ic_rows(&v));
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
                let pairs = paired_indices(ic, dn, |r| r.seed, |r| r.seed);
                let n = pairs.len();
                let s_d: Vec<f64> = pairs.iter().map(|&(_, j)| dn[j].s).collect();
                let s_s: Vec<f64> = pairs.iter().map(|&(i, _)| ic[i].s).collect();
                let lin_d: Vec<f64> = pairs.iter().map(|&(_, j)| dn[j].lin_muls).collect();
                let lin_s: Vec<f64> = pairs.iter().map(|&(i, _)| ic[i].lin_muls).collect();
                entry.insert(
                    "dense_vs_sparse".into(),
                    json!({
                        "pairs": n,
                        "paired_seeds": pairs.iter().map(|&(i, _)| ic[i].seed).collect::<Vec<_>>(),
                        "identical_descent_digest": pairs.iter().filter(|&&(i,j)| dn[j].digest == ic[i].digest).count(),
                        "identical_outcome": pairs.iter().filter(|&&(i,j)| dn[j].ok == ic[i].ok).count(),
                        "S_dense_over_sparse": ratio_ci(&s_d, &s_s, sd_ + 4),
                        "linear_stage_dense_over_sparse": ratio_ci(&lin_d, &lin_s, sd_ + 5),
                        "descent_muls_dense_over_sparse": ratio_ci(
                            &pairs.iter().map(|&(_,j)| dn[j].descent_muls).collect::<Vec<_>>(),
                            &pairs.iter().map(|&(i,_)| ic[i].descent_muls).collect::<Vec<_>>(), sd_ + 6),
                        "sparse_fallbacks_per_run": mean(&ic.iter().map(|r| r.fallback).collect::<Vec<_>>()),
                        "S_dense": mean(&s_d), "S_sparse": mean(&s_s),
                    }),
                );
            }
            if p == 1009 {
                if let Some(aa) = load(&dir, "ic_1009_sparse_aa").map(|v| ic_rows(&v)) {
                    let pairs = paired_indices(ic, &aa, |r| r.seed, |r| r.seed);
                    let same = pairs
                        .iter()
                        .filter(|&&(i, j)| ic[i].muls == aa[j].muls && ic[i].digest == aa[j].digest)
                        .count();
                    entry.insert(
                        "ic_aa".into(),
                        json!({"pairs": pairs.len(), "paired_seeds":pairs.iter().map(|&(i,_)| ic[i].seed).collect::<Vec<_>>(), "seed_inventory":seed_inventory(aa.iter().map(|r| r.seed), &(1..=16).collect::<Vec<_>>()), "identical_counts": same}),
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
                e.insert("all_correct".into(), json!(rs.iter().all(|r| r.ok)));
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
                let pairs = paired_indices(ic, rs, |r| r.seed, |r| r.seed);
                let pick = |fx: &dyn Fn(&Ic) -> f64| -> Vec<f64> {
                    pairs.iter().map(|&(i, _)| fx(&ic[i])).collect()
                };
                let rho_sorted: Vec<&Rho> = pairs.iter().map(|&(_, j)| &rs[j]).collect();
                if !rho_sorted.is_empty() {
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
                        let ic_a: Vec<f64> = pairs
                            .iter()
                            .map(|&(i, _)| {
                                let r = &ic[i];
                                (r.muls / GAE
                                    + w * (r.adds + r.lookups + r.enum_adds + r.unemitted_bound))
                                    / sq
                            })
                            .collect();
                        let ic_b: Vec<f64> = pairs
                            .iter()
                            .map(|&(i, _)| {
                                let r = &ic[i];
                                (r.muls / GAE + w * r.lookups) / sq
                            })
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
                    e.insert(
                        "descriptive_band_interval_all_below".into(),
                        json!(all_below),
                    );
                    let io = &e["ic_over_rho"];
                    let vd: BTreeMap<&str, &str> =
                        ["S_harness", "S_tuned", "S_plus_harness", "S_plus_tuned"]
                            .iter()
                            .map(|k| (*k, verdict(&io[*k])))
                            .collect();
                    e.insert("descriptive_interval_classification".into(), json!(vd));
                    e.insert("verdict".into(), json!({"status":"withheld", "reason":"frozen control acceptance criteria are not numerically executable"}));
                    e.insert(
                        "ranked_across_band_ic_below_rho".into(),
                        json!({"status":"withheld", "reason":"frozen control acceptance criteria unresolved"}),
                    );
                }
                rho_out.insert(name.clone(), Value::Object(e));
            }
            entry.insert("rho".into(), Value::Object(rho_out));
            // exponent data
            let ic_group = (
                ic.iter().map(|r| r.l).collect(),
                ic.iter().map(|r| r.muls).collect(),
            );
            if is_posthoc_tail_size(p) {
                fit_ic_tail.push(ic_group.clone());
            }
            fit_ic.push(ic_group);
            if let Some(r3) = arms.get("R3") {
                let rho_group = (
                    r3.iter().map(|r| r.l).collect(),
                    r3.iter().map(|r| r.mh).collect(),
                );
                if is_posthoc_tail_size(p) {
                    fit_r3_tail.push(rho_group.clone());
                }
                fit_r3.push(rho_group);
                fit_r3t.push((
                    r3.iter().map(|r| r.l).collect(),
                    r3.iter().map(|r| r.mt).collect(),
                ));
            }
        }
        let expected_ic: Vec<u64> = (1..=16).collect();
        let mut inventories = serde_json::Map::new();
        if let Some(rows) = &sparse {
            inventories.insert(
                "ic_sparse".into(),
                seed_inventory(rows.iter().map(|r| r.seed), &expected_ic),
            );
            entry.insert(
                "ic_sparse_valid_rows".into(),
                json!(rows
                    .iter()
                    .filter(|r| r.ok
                        && r.muls.is_finite()
                        && r.muls > 0.0
                        && r.l.is_finite()
                        && r.l > 0.0)
                    .count()),
            );
            entry.insert(
                "ic_sparse_invalid_rows".into(),
                json!(rows
                    .iter()
                    .filter(|r| !r.ok
                        || !r.muls.is_finite()
                        || r.muls <= 0.0
                        || !r.l.is_finite()
                        || r.l <= 0.0)
                    .count()),
            );
        } else {
            inventories.insert("ic_sparse".into(), json!({"expected":expected_ic,"observed":[],"missing":expected_ic,"duplicates":[],"complete":false}));
        }
        if let Some(rows) = &dense {
            inventories.insert(
                "ic_dense".into(),
                seed_inventory(rows.iter().map(|r| r.seed), &expected_ic),
            );
            entry.insert(
                "ic_dense_valid_rows".into(),
                json!(rows
                    .iter()
                    .filter(|r| r.ok
                        && r.muls.is_finite()
                        && r.muls > 0.0
                        && r.l.is_finite()
                        && r.l > 0.0)
                    .count()),
            );
            entry.insert(
                "ic_dense_invalid_rows".into(),
                json!(rows
                    .iter()
                    .filter(|r| !r.ok
                        || !r.muls.is_finite()
                        || r.muls <= 0.0
                        || !r.l.is_finite()
                        || r.l <= 0.0)
                    .count()),
            );
        }
        for arm in ["R0", "R1", "R2", "R3", "R0_aa", "R3_aa"] {
            if let Some(rows) = arms.get(arm) {
                let expected = expected_rho_seeds(p, arm);
                inventories.insert(
                    format!("rho_{arm}"),
                    seed_inventory(rows.iter().map(|r| r.seed), &expected),
                );
                entry.insert(
                    format!("rho_{arm}_valid_rows"),
                    json!(rows
                        .iter()
                        .filter(|r| r.ok
                            && r.mh.is_finite()
                            && r.mh > 0.0
                            && r.mt.is_finite()
                            && r.mt > 0.0
                            && r.l.is_finite()
                            && r.l > 0.0)
                        .count()),
                );
                entry.insert(
                    format!("rho_{arm}_invalid_rows"),
                    json!(rows
                        .iter()
                        .filter(|r| !r.ok
                            || !r.mh.is_finite()
                            || r.mh <= 0.0
                            || !r.mt.is_finite()
                            || r.mt <= 0.0
                            || !r.l.is_finite()
                            || r.l <= 0.0)
                        .count()),
                );
            } else {
                let expected = expected_rho_seeds(p, arm);
                if !expected.is_empty() {
                    inventories.insert(
                        format!("rho_{arm}"),
                        seed_inventory(std::iter::empty(), &expected),
                    );
                }
            }
        }
        entry.insert("seed_inventory".into(), Value::Object(inventories));
        entry.insert("comparison_status".into(), json!("withheld"));
        entry.insert("comparison_blockers".into(), json!([
            "control acceptance criteria are unresolved in the frozen protocol",
            "all scheduled seeds and correctly solved, uncapped rows must be present before comparison eligibility"
        ]));
        per_size.push(Value::Object(entry));
    }
    // The frozen protocol does not define a numeric acceptance margin for either
    // the R0 positive control (“S≈1.3”) or the R0/R3 A/A bootstrap comparison.
    // Retain measured summaries above, but do not turn interval descriptions into
    // executable acceptance rules or comparative verdicts.
    let control_blockers = vec![
        "positive-control acceptance is unresolved: PROTOCOL.md says R0 S≈1.3 without a numeric tolerance".to_string(),
        "A/A acceptance is unresolved: PROTOCOL.md gives no numeric rule for samples differing beyond bootstrap intervals".to_string(),
    ];
    summary.insert("eligibility_schema_version".into(), json!(1));
    summary.insert("comparison_status".into(), json!("withheld"));
    summary.insert("comparison_blockers".into(), json!(control_blockers));
    let p1009 = per_size.iter().find(|e| e["p"] == json!(1009));
    let ic_aa = p1009
        .and_then(|e| e.get("ic_aa"))
        .cloned()
        .unwrap_or(Value::Null);
    let ic_aa_status = if ic_aa.is_null() {
        "missing"
    } else if ic_aa["pairs"].as_u64() != Some(16) {
        "incomplete"
    } else if ic_aa["identical_counts"].as_u64() == Some(16) {
        "observed_match"
    } else {
        "mismatch"
    };
    let rho_arms_1009 = p1009
        .and_then(|e| e.get("rho"))
        .cloned()
        .unwrap_or(Value::Null);
    let positive_observed = rho_arms_1009
        .get("R0")
        .and_then(|v| v.get("S_harness"))
        .cloned()
        .unwrap_or(Value::Null);
    let aa_r0 = rho_arms_1009.get("R0_aa").cloned().unwrap_or(Value::Null);
    let aa_r3 = rho_arms_1009.get("R3_aa").cloned().unwrap_or(Value::Null);
    let positive_status = if positive_observed.is_null() {
        "missing"
    } else {
        "acceptance_unresolved"
    };
    let rho_aa_status = if aa_r0.is_null() || aa_r3.is_null() {
        "missing"
    } else {
        "acceptance_unresolved"
    };
    summary.insert("controls".into(), json!({
        "positive_r0": {
            "status":positive_status,
            "observed":positive_observed,
            "acceptance":"PROTOCOL.md specifies S≈1.3 but no numeric tolerance"
        },
        "rho_aa_r0_r3": {
            "status":rho_aa_status,
            "r0_observed":aa_r0,
            "r3_observed":aa_r3,
            "acceptance":"PROTOCOL.md does not define an executable numeric comparison boundary for bootstrap-interval overlap"
        },
        "ic_sparse_repeat": {
            "status":ic_aa_status,
            "observed":ic_aa,
            "required_pairs":16
        }
    }));
    summary.insert(
        "K1_status".into(),
        json!({"status":"withheld", "reason":"required control acceptance criteria unresolved"}),
    );
    summary.insert(
        "K2_status".into(),
        json!({"status":"withheld", "reason":"required control acceptance criteria unresolved"}),
    );
    summary.insert(
        "K2_descriptive_band_observations".into(),
        json!(per_size
            .iter()
            .filter_map(|e| {
                k2_descriptive_band_result(e)
                    .map(|value| json!({"p":e["p"],"all_intervals_below":value}))
            })
            .collect::<Vec<_>>()),
    );
    summary.insert("sizes".into(), json!(per_size));
    summary.insert("exponent_ic_muls_vs_l".into(), fit(&fit_ic, 77));
    summary.insert("exponent_rho_R3_harness_muls_vs_l".into(), fit(&fit_r3, 78));
    summary.insert("exponent_rho_R3_tuned_muls_vs_l".into(), fit(&fit_r3t, 79));
    summary.insert("exponent_status".into(), json!("withheld"));
    summary.insert("exponent_blockers".into(), json!(["complete validated seed coverage and accepted controls are required for scaling claims"]));
    // POST HOC (not in PROTOCOL.md): the sizes where the sieve runs at m = 9 (p >= 1009)
    summary.insert(
        "exponent_ic_muls_vs_l_p_ge_1009_POST_HOC".into(),
        fit(&fit_ic_tail, 80),
    );
    summary.insert(
        "exponent_rho_R3_harness_muls_vs_l_p_ge_1009_POST_HOC".into(),
        fit(&fit_r3_tail, 81),
    );
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
    fs::write(
        format!("{dir}/summary.json"),
        serde_json::to_string_pretty(&summary).unwrap(),
    )
    .unwrap();
    println!("wrote {dir}/summary.json");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aa_inventory_matches_the_frozen_sixteen_seed_schedule() {
        assert_eq!(
            expected_rho_seeds(1009, "R0_aa"),
            (1..=16).collect::<Vec<_>>()
        );
        assert_eq!(expected_rho_seeds(1009, "R3_aa").len(), 16);
        assert!(expected_rho_seeds(1511, "R1").is_empty());
        assert!(expected_rho_seeds(251, "R0_aa").is_empty());
        assert_eq!(expected_rho_seeds(1823, "R0").len(), 4);
    }

    #[test]
    fn unavailable_intervals_do_not_panic_or_classify_a_winner() {
        assert_eq!(verdict(&json!({"ci95":[null, null]})), "unavailable");
        assert_eq!(verdict(&json!({"ci95":[0.5, null]})), "unavailable");
    }

    #[test]
    fn incomplete_and_duplicate_seed_sets_are_not_complete() {
        let expected: Vec<u64> = (1..=16).collect();
        let partial = seed_inventory([1, 2, 4].into_iter(), &expected);
        assert_eq!(partial["complete"], false);
        assert_eq!(
            partial["missing"],
            json!([3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16])
        );
        let duplicate = seed_inventory([1, 1, 2].into_iter(), &expected);
        assert_eq!(duplicate["complete"], false);
        assert_eq!(duplicate["duplicates"], json!([1]));
    }

    #[test]
    fn seed_pairs_match_identity_not_position() {
        let left = vec![(7, 1.0), (3, 2.0)];
        let right = vec![(3, 20.0), (9, 90.0), (7, 70.0)];
        let pairs = paired_indices(&left, &right, |x| x.0, |x| x.0);
        assert_eq!(pairs, vec![(0, 2), (1, 0)]);
        let mismatched = paired_indices(&left, &right[..2], |x| x.0, |x| x.0);
        assert_eq!(mismatched, vec![(1, 0)]);
    }

    #[test]
    fn incorrect_or_capped_rows_are_retained_but_invalid_for_claims() {
        let bad_ic = json!({"seed":1,"l":16,"total_muls":50,"solved":true,"correct":false});
        let ic = ic_rows(&[bad_ic]);
        assert_eq!(ic.len(), 1);
        assert!(!ic[0].ok);
        let bad_rho =
            json!({"seed":1,"l":16,"muls_harness":50,"muls_tuned":40,"correct":true,"capped":true});
        let rho = rho_rows(&[bad_rho]);
        assert_eq!(rho.len(), 1);
        assert!(!rho[0].ok);
    }

    #[test]
    fn posthoc_tail_uses_actual_p_threshold() {
        assert!(!is_posthoc_tail_size(503));
        assert!(is_posthoc_tail_size(1009));
        assert!(is_posthoc_tail_size(1511));
    }

    #[test]
    fn k2_reads_r3_nested_observation_not_an_absent_top_level_field() {
        let per_size = json!({"rho":{"R3":{"descriptive_band_interval_all_below":true}}});
        assert_eq!(k2_descriptive_band_result(&per_size), Some(true));
        assert_eq!(
            k2_descriptive_band_result(&json!({"descriptive_band_interval_all_below":true})),
            None
        );
    }
}
