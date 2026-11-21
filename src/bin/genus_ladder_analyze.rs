//! `genus_ladder_analyze IN.jsonl OUT.json`: the statistics of a
//! `genus_ladder run` session.  Reads the recorded counts and derives cell
//! means, hierarchical bootstrap intervals (curves, then runs within a
//! curve), arm-to-rho ratios across the price band, growth exponents in `p`,
//! and the paired prefilter comparison.  It measures nothing itself.
//!
//! Cost readings of one run:
//!
//! * `gae_w`, `w ∈ {0, 0.1, 1}` (AGENTS.md 5a): group operations plus `w`
//!   per native multiplication (relation phase and linear algebra; for rho
//!   its per-step canonicalisation).  Comparable inside one genus.
//! * `fpm_*`: `F_p`-multiplication equivalents, comparable across genera:
//!   `cantor_mul + I·cantor_inv + rel_mul + κ·la_mul`.  `I` prices a field
//!   inversion, `κ` a multiplication modulo `r ≈ p^g` against one modulo `p`.
//!   `fpm_low`: `I = 1, κ = 1`; `fpm_mid` (primary): `I = 10, κ = g²`;
//!   `fpm_high`: `I = 30, κ = g²`.
//!
//! Protocol: `research/genus_ladder_20261114/PROTOCOL.md`.

use std::collections::BTreeMap;
use std::process::ExitCode;

use serde_json::{json, Value};

const READINGS: [&str; 7] = [
    "gae_w0", "gae_w01", "gae_w1", "fpm_low", "fpm_mid", "fpm_high", "trials",
];
/// Readings that are costs (the last, relation trials, is a count).
const COSTS: usize = 6;
const BOOT: usize = 2000;

#[derive(Clone, Debug)]
struct Run {
    family: String,
    p: u64,
    g: u64,
    curve: u64,
    seed: u64,
    arm: String,
    cost: [f64; 7],
    raw: Value,
}

fn f(v: &Value, k: &str) -> f64 {
    v.get(k).and_then(Value::as_f64).unwrap_or(0.0)
}

fn costs(v: &Value) -> [f64; 7] {
    let g = f(v, "g");
    let grp = f(v, "group_ops");
    let native = f(v, "rel_mul") + f(v, "la_mul") + f(v, "native");
    let base = f(v, "cantor_mul") + f(v, "rel_mul");
    let (inv, la) = (f(v, "cantor_inv"), f(v, "la_mul"));
    [
        grp,
        grp + 0.1 * native,
        grp + native,
        base + inv + la,
        base + 10.0 * inv + g * g * la,
        base + 30.0 * inv + g * g * la,
        f(v, "trials"),
    ]
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// Runs of one cell grouped by curve: `curve → [cost vectors]`.
type ByCurve = BTreeMap<u64, Vec<[f64; 7]>>;

fn mean_of(bc: &ByCurve, k: usize) -> f64 {
    let (mut s, mut n) = (0.0, 0usize);
    for v in bc.values() {
        for c in v {
            s += c[k];
            n += 1;
        }
    }
    s / n.max(1) as f64
}

/// One hierarchical resample: curve indices, and for each a list of run
/// indices; `picks[i] = (curve position, run positions)`.
fn resample(curves: &[u64], sizes: &BTreeMap<u64, usize>, rng: &mut Rng) -> Vec<(u64, Vec<usize>)> {
    (0..curves.len())
        .map(|_| {
            let c = curves[rng.below(curves.len())];
            let n = sizes[&c];
            (c, (0..n).map(|_| rng.below(n)).collect())
        })
        .collect()
}

fn mean_resampled(bc: &ByCurve, picks: &[(u64, Vec<usize>)], k: usize, rng: &mut Rng) -> f64 {
    let (mut s, mut n) = (0.0, 0usize);
    for (c, idx) in picks {
        let Some(v) = bc.get(c) else { continue };
        // The arm may hold a different number of runs on this curve than
        // the arm that fixed the picks: draw inside this arm's runs.
        for &i in idx {
            let j = if v.len() == idx.len() {
                i
            } else {
                rng.below(v.len())
            };
            s += v[j][k];
            n += 1;
        }
    }
    if n == 0 {
        f64::NAN
    } else {
        s / n as f64
    }
}

fn pct(v: &mut Vec<f64>, q: f64) -> f64 {
    v.retain(|x| x.is_finite());
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[((v.len() as f64 - 1.0) * q).round() as usize]
}

fn ols(x: &[f64], y: &[f64]) -> (f64, f64) {
    let n = x.len() as f64;
    let (mx, my) = (x.iter().sum::<f64>() / n, y.iter().sum::<f64>() / n);
    let sxx: f64 = x.iter().map(|a| (a - mx).powi(2)).sum();
    let sxy: f64 = x.iter().zip(y).map(|(a, b)| (a - mx) * (b - my)).sum();
    let slope = sxy / sxx;
    (slope, my - slope * mx)
}

fn main() -> ExitCode {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.len() != 2 {
        eprintln!("usage: genus_ladder_analyze IN.jsonl OUT.json");
        return ExitCode::from(2);
    }
    let text = std::fs::read_to_string(&a[0]).expect("read input");
    let mut runs: Vec<Run> = Vec::new();
    let mut instances: Vec<Value> = Vec::new();
    let mut cells_rec: Vec<Value> = Vec::new();
    let mut census: Vec<Value> = Vec::new();
    let mut skips: Vec<Value> = Vec::new();
    let mut config = Value::Null;
    let (mut n_run, mut n_bad) = (0u64, 0u64);
    let mut bad: Vec<Value> = Vec::new();
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let v: Value = serde_json::from_str(line).expect("a JSON line");
        match v["kind"].as_str().unwrap_or("") {
            "config" => config = v,
            "instance" => instances.push(v),
            "cell" => cells_rec.push(v),
            "smith_census" => census.push(v),
            "skip" => skips.push(v),
            "run" => {
                n_run += 1;
                let good =
                    v["certified"].as_bool() == Some(true) && v["ok"].as_bool() == Some(true);
                if !good {
                    n_bad += 1;
                    bad.push(json!({"family": v["family"], "p": v["p"], "g": v["g"], "arm": v["arm"],
                        "curve_seed": v["curve_seed"], "seed": v["seed"], "exhausted": v["exhausted"]}));
                    continue;
                }
                runs.push(Run {
                    family: v["family"].as_str().unwrap_or("").into(),
                    p: v["p"].as_u64().unwrap(),
                    g: v["g"].as_u64().unwrap(),
                    curve: v["curve_seed"].as_u64().unwrap(),
                    seed: v["seed"].as_u64().unwrap(),
                    arm: v["arm"].as_str().unwrap_or("").into(),
                    cost: costs(&v),
                    raw: v,
                });
            }
            _ => {}
        }
    }

    // cell key: (family, g, p) → arm → curve → runs
    type Key = (String, u64, u64);
    let mut cells: BTreeMap<Key, BTreeMap<String, ByCurve>> = BTreeMap::new();
    for r in &runs {
        cells
            .entry((r.family.clone(), r.g, r.p))
            .or_default()
            .entry(r.arm.clone())
            .or_default()
            .entry(r.curve)
            .or_default()
            .push(r.cost);
    }

    let mut rng = Rng(20_261_114);
    let mut cell_out = Vec::new();
    // (family, g, arm) → reading → Vec<(ln p, ln mean, bootstrap ln means)>
    type Series = Vec<(f64, f64, Vec<f64>)>;
    let mut series: BTreeMap<(String, u64, String), Vec<Series>> = BTreeMap::new();
    for ((family, g, p), arms) in &cells {
        let rho_name = if family == "quartic" {
            "Q_rho_neg"
        } else {
            "R_rho_neg"
        };
        let rho = arms.get(rho_name);
        for (arm, bc) in arms {
            let curves: Vec<u64> = bc.keys().copied().collect();
            let sizes: BTreeMap<u64, usize> = bc.iter().map(|(c, v)| (*c, v.len())).collect();
            let n: usize = sizes.values().sum();
            let mut rec = json!({"family": family, "g": g, "p": p, "arm": arm,
                "curves": curves.len(), "runs": n});
            let mut boots_mean: Vec<Vec<f64>> = (0..7).map(|_| Vec::with_capacity(BOOT)).collect();
            let mut boots_ratio: Vec<Vec<f64>> = (0..7).map(|_| Vec::with_capacity(BOOT)).collect();
            for _ in 0..BOOT {
                let picks = resample(&curves, &sizes, &mut rng);
                for k in 0..7 {
                    let m = mean_resampled(bc, &picks, k, &mut rng);
                    boots_mean[k].push(m);
                    if let Some(rb) = rho {
                        boots_ratio[k].push(m / mean_resampled(rb, &picks, k, &mut rng));
                    }
                }
            }
            let ser = series
                .entry((family.clone(), *g, arm.clone()))
                .or_insert_with(|| vec![Vec::new(); 7]);
            let mut verdicts = Vec::new();
            for k in 0..7 {
                let m = mean_of(bc, k);
                let mut b = boots_mean[k].clone();
                rec[READINGS[k]] =
                    json!({"mean": m, "lo": pct(&mut b, 0.025), "hi": pct(&mut b, 0.975)});
                ser[k].push((
                    (*p as f64).ln(),
                    m.ln(),
                    boots_mean[k].iter().map(|x| x.ln()).collect(),
                ));
                if let Some(rb) = rho {
                    if arm != rho_name && k < COSTS {
                        let ratio = m / mean_of(rb, k);
                        let mut br = boots_ratio[k].clone();
                        let (lo, hi) = (pct(&mut br, 0.025), pct(&mut br, 0.975));
                        rec[format!("{}_over_rho", READINGS[k])] =
                            json!({"ratio": ratio, "lo": lo, "hi": hi});
                        verdicts.push(if hi < 1.0 {
                            "below"
                        } else if lo > 1.0 {
                            "above"
                        } else {
                            "unresolved"
                        });
                    }
                }
            }
            if !verdicts.is_empty() {
                let band = |v: &[&str]| -> &str {
                    if v.iter().all(|x| *x == "below") {
                        "below"
                    } else if v.iter().all(|x| *x == "above") {
                        "above"
                    } else {
                        "unresolved"
                    }
                };
                rec["band_gae"] = json!(band(&verdicts[..3]));
                rec["band_fpm"] = json!(band(&verdicts[3..COSTS]));
            }
            // descriptive means of the recorded counters
            let mine: Vec<&Run> = runs
                .iter()
                .filter(|r| &r.family == family && r.g == *g && r.p == *p && &r.arm == arm)
                .collect();
            let avg = |k: &str| mine.iter().map(|r| f(&r.raw, k)).sum::<f64>() / mine.len() as f64;
            for k in [
                "group_ops",
                "cantor_mul",
                "cantor_inv",
                "rel_mul",
                "la_mul",
                "la_add",
                "trials",
                "candidates",
                "splits",
                "relations",
                "fb_cols",
                "core_cols",
                "prefilter_rejects",
                "steps",
                "wall_ns",
            ] {
                rec[format!("mean_{k}")] = json!(avg(k));
            }
            let cand: f64 = mine.iter().map(|r| f(&r.raw, "candidates")).sum();
            if cand > 0.0 {
                let sp: f64 = mine.iter().map(|r| f(&r.raw, "splits")).sum();
                rec["split_fraction"] = json!(sp / cand);
                rec["split_fraction_times_g_factorial"] =
                    json!(sp / cand * (1..=*g).product::<u64>() as f64);
            }
            let ln_r: f64 =
                mine.iter().map(|r| f(&r.raw, "r").ln()).sum::<f64>() / mine.len() as f64;
            rec["mean_ln_r"] = json!(ln_r);
            cell_out.push(rec);
        }
    }

    // growth exponents in p
    let mut fits = Vec::new();
    for ((family, g, arm), per_reading) in &series {
        for (k, ser) in per_reading.iter().enumerate() {
            if ser.len() < 3 {
                continue;
            }
            let x: Vec<f64> = ser.iter().map(|s| s.0).collect();
            let y: Vec<f64> = ser.iter().map(|s| s.1).collect();
            if y.iter().any(|v| !v.is_finite()) {
                continue;
            }
            let (slope, icpt) = ols(&x, &y);
            let mut bs: Vec<f64> = (0..BOOT)
                .map(|b| {
                    let yb: Vec<f64> = ser.iter().map(|s| s.2[b]).collect();
                    ols(&x, &yb).0
                })
                .collect();
            fits.push(json!({"family": family, "g": g, "arm": arm, "reading": READINGS[k],
                "primes": ser.len(), "p_min": x[0].exp().round(), "p_max": x[x.len() - 1].exp().round(),
                "exponent": slope, "lo": pct(&mut bs, 0.025), "hi": pct(&mut bs, 0.975), "intercept": icpt}));
        }
    }

    // prefilter pairs: same instance, same seed, arm vs arm+pre
    let mut by_id: BTreeMap<(String, u64, u64, u64, u64, String), &Run> = BTreeMap::new();
    for r in &runs {
        by_id.insert(
            (r.family.clone(), r.g, r.p, r.curve, r.seed, r.arm.clone()),
            r,
        );
    }
    let mut pre: BTreeMap<(u64, u64, String), Vec<(f64, f64, f64, bool)>> = BTreeMap::new();
    for ((family, g, p, curve, seed, arm), on) in &by_id {
        let Some(base) = arm.strip_suffix("+pre") else {
            continue;
        };
        let Some(off) = by_id.get(&(family.clone(), *g, *p, *curve, *seed, base.to_string()))
        else {
            continue;
        };
        let same = [
            "trials",
            "relations",
            "splits",
            "group_ops",
            "cantor_mul",
            "la_mul",
            "candidates",
        ]
        .iter()
        .all(|k| f(&on.raw, k) == f(&off.raw, k));
        pre.entry((*g, *p, base.to_string())).or_default().push((
            f(&on.raw, "rel_mul") / f(&off.raw, "rel_mul"),
            on.cost[4] / off.cost[4],
            on.cost[1] / off.cost[1],
            same,
        ));
    }
    let mut pre_out = Vec::new();
    for ((g, p, arm), v) in &pre {
        let n = v.len() as f64;
        let col = |i: usize| -> (f64, f64, f64) {
            let xs: Vec<f64> = v.iter().map(|t| [t.0, t.1, t.2][i]).collect();
            (
                xs.iter().sum::<f64>() / n,
                xs.iter().cloned().fold(f64::INFINITY, f64::min),
                xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            )
        };
        let (a, b, c) = (col(0), col(1), col(2));
        pre_out.push(json!({"g": g, "p": p, "arm": arm, "pairs": v.len(),
            "identical_counts": v.iter().filter(|t| t.3).count(),
            "rel_mul_ratio": {"mean": a.0, "min": a.1, "max": a.2},
            "fpm_mid_ratio": {"mean": b.0, "min": b.1, "max": b.2},
            "gae_w01_ratio": {"mean": c.0, "min": c.1, "max": c.2}}));
    }

    // The split rate of each instance against the count of split divisors:
    // C(n_x, g)·2^g reduced divisors have a u with g distinct roots carrying
    // points with y ≠ 0, out of #J, where n_x is the full base size.
    let mut order: BTreeMap<(u64, u64, u64), f64> = BTreeMap::new();
    for i in &instances {
        if i["family"] == "hyperelliptic" {
            order.insert(
                (
                    i["g"].as_u64().unwrap(),
                    i["p"].as_u64().unwrap(),
                    i["curve_seed"].as_u64().unwrap(),
                ),
                f(i, "order"),
            );
        }
    }
    let mut sm: BTreeMap<(u64, u64, u64), (f64, f64, f64)> = BTreeMap::new();
    for r in runs.iter().filter(|r| r.arm == "A_full") {
        let e = sm.entry((r.g, r.p, r.curve)).or_default();
        e.0 += f(&r.raw, "candidates");
        e.1 += f(&r.raw, "splits");
        e.2 = f(&r.raw, "fb_cols");
    }
    let mut smooth_out = Vec::new();
    for ((g, p, curve), (cand, splits, fb)) in &sm {
        let Some(n) = order.get(&(*g, *p, *curve)) else {
            continue;
        };
        let mut comb = 1.0f64;
        let mut gfact = 1.0f64;
        for i in 0..*g {
            comb *= (fb - i as f64) / (i as f64 + 1.0);
            gfact *= i as f64 + 1.0;
        }
        let predicted = comb * 2f64.powi(*g as i32) / n;
        smooth_out.push(
            json!({"g": g, "p": p, "curve_seed": curve, "fb": fb, "order": n,
            "candidates": cand, "splits": splits, "observed": splits / cand, "predicted": predicted,
            "observed_over_predicted": splits / cand / predicted,
            "predicted_times_g_factorial": predicted * gfact}),
        );
    }

    // Smith on the admitted genus-3 curves
    let mut smith: BTreeMap<u64, (u64, u64, u64)> = BTreeMap::new();
    for i in &instances {
        if let Some(s) = i.get("smith") {
            let e = smith.entry(i["p"].as_u64().unwrap()).or_default();
            e.0 += 1;
            e.1 += u64::from(s["n_tractable"].as_u64().unwrap_or(0) > 0);
            e.2 += u64::from(s["full_success"].as_bool() == Some(true));
        }
    }
    let smith_out: Vec<Value> = smith
        .iter()
        .map(|(p, t)| json!({"p": p, "curves": t.0, "with_tractable_subgroup": t.1, "full_success": t.2}))
        .collect();

    let out = json!({
        "schema": "genus_ladder.analysis/v1",
        "input": a[0],
        "config": config,
        "runs_total": n_run, "runs_not_certified": n_bad, "not_certified": bad,
        "bootstrap": {"resamples": BOOT, "seed": 20_261_114u64, "scheme": "curves, then runs within a curve"},
        "readings": READINGS,
        "cells": cell_out,
        "fits": fits,
        "prefilter": pre_out,
        "smooth_model": smooth_out,
        "smith_admitted": smith_out,
        "smith_census": census,
        "admission": cells_rec,
        "skipped": skips,
        "instances": instances,
    });
    std::fs::write(&a[1], serde_json::to_string_pretty(&out).unwrap()).expect("write output");
    println!(
        "{} runs ({} not certified), {} cells, {} fits -> {}",
        n_run,
        n_bad,
        out["cells"].as_array().unwrap().len(),
        out["fits"].as_array().unwrap().len(),
        a[1]
    );
    ExitCode::SUCCESS
}
