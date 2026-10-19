//! The E4 analysis of the generator-effect study
//! (research/generator_effect_ic_20261019/E0_E4.md, "E4 PROTOCOL"):
//! "the standard's G" (`gstd`) against four random generators (`grand1-4`)
//! per curve, under the abscissa and the random-scalar known-log bases.
//!
//! `cargo run --release --example generator_effect_e4_analysis -- OUT.json GENERATORS.jsonl SESSION_DIR...`
//!
//! Every session directory holds an `ecbench` `records.jsonl`.  A record's
//! cell is found from its curve's generator in `GENERATORS.jsonl`; a session
//! whose `target_seed` is 20261020 is the A/A cell `gstd-aa`.  Metrics per
//! run: yield (relations found / targets tried, pooled as ratio of sums) and
//! total S at w in {0, 0.1, 1} GAE per unpriced operation (AGENTS.md §5a).
//! Primary: the gstd/grand ratio (geometric mean of per-curve ratios) with a
//! 90% percentile bootstrap over runs within cells; secondary: Cochran Q,
//! DerSimonian-Laird tau and max|z| with permutation nulls; controls: A/A
//! and a planted +3% on gstd.
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap};

const WEIGHTS: [f64; 3] = [0.0, 0.1, 1.0];
const METRICS: [&str; 4] = ["yield", "S_w0", "S_w0.1", "S_w1"];
const RESAMPLES: usize = 10_000;
const PERMS: usize = 2_000;
const MARGIN: f64 = 0.02;
const PLANT: f64 = 1.03;
/// Cells: 0 = gstd, 1..=4 = grand1..4, 5 = gstd-aa.
const CELLS: [&str; 6] = ["gstd", "grand1", "grand2", "grand3", "grand4", "gstd-aa"];
const AA: usize = 5;

#[derive(Clone)]
struct Run {
    curve: usize,
    cell: usize,
    /// (numerator, denominator) per metric.
    v: [(f64, f64); 4],
}

struct Exact {
    fb: Vec<f64>,
    setup: Vec<f64>,
}

fn unpriced_ops(v: &Value) -> f64 {
    let unpriced: Vec<String> = v["cost"]["unpriced"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str()?.strip_suffix("_uncharged").map(String::from))
                .collect()
        })
        .unwrap_or_default();
    let mut total = 0.0;
    for p in v["phases"].as_array().into_iter().flatten() {
        if let Some(native) = p["native"].as_object() {
            for k in &unpriced {
                total += native.get(k).and_then(Value::as_f64).unwrap_or(0.0);
            }
        }
    }
    total
}

fn phase_key(v: &Value, name: &str) -> f64 {
    v["phases"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|p| p["name"] == name)
        .map_or(0.0, |p| {
            let native: f64 = p["native"]
                .as_object()
                .map_or(0.0, |m| m.values().filter_map(Value::as_f64).sum());
            // GAE and the phase's native work together: identical cells
            // agree on both.
            p["gae"].as_f64().unwrap_or(0.0) * 1e6 + native
        })
}

fn ratio(runs: &[&Run], m: usize) -> f64 {
    let (n, d) = runs
        .iter()
        .fold((0.0, 0.0), |(n, d), r| (n + r.v[m].0, d + r.v[m].1));
    n / d
}

fn ratio_se2(runs: &[&Run], m: usize) -> (f64, f64) {
    let mean = ratio(runs, m);
    let d: f64 = runs.iter().map(|r| r.v[m].1).sum();
    let k = runs.len() as f64;
    let ss: f64 = runs
        .iter()
        .map(|r| (r.v[m].0 - mean * r.v[m].1).powi(2))
        .sum();
    (mean, ss / (k - 1.0).max(1.0) * k / (d * d))
}

fn quantile(v: &mut [f64], q: f64) -> f64 {
    v.sort_by(|a, b| a.total_cmp(b));
    v[((q * (v.len() - 1) as f64).round() as usize).min(v.len() - 1)]
}

/// `cells[curve][cell]` = the runs of that cell.
type Cells<'a> = Vec<Vec<Vec<&'a Run>>>;

/// Geometric mean over curves of ratio(num cells) / ratio(den cells).
fn geo_ratio(cells: &Cells, m: usize, num: &[usize], den: &[usize], plant: f64) -> f64 {
    let mut acc = 0.0;
    let mut k = 0;
    for per in cells {
        let a: Vec<&Run> = num.iter().flat_map(|&c| per[c].iter().copied()).collect();
        let b: Vec<&Run> = den.iter().flat_map(|&c| per[c].iter().copied()).collect();
        if a.is_empty() || b.is_empty() {
            continue;
        }
        acc += (plant * ratio(&a, m) / ratio(&b, m)).ln();
        k += 1;
    }
    (acc / k as f64).exp()
}

/// Cochran Q, DL tau (relative to the stratum mean) and max|z| over the
/// given cells of every curve.
fn cochran(cells: &Cells, m: usize, use_cells: &[usize]) -> (f64, f64, usize, f64) {
    let (mut q, mut maxz, mut sw, mut sw2, mut df) = (0.0, 0f64, 0.0, 0.0, 0usize);
    for per in cells {
        let stats: Vec<(f64, f64)> = use_cells.iter().map(|&c| ratio_se2(&per[c], m)).collect();
        let mbar_raw = stats.iter().map(|s| s.0).sum::<f64>() / stats.len() as f64;
        // Relative scale, so curves of different size pool.
        let stats: Vec<(f64, f64)> = stats
            .iter()
            .map(|s| (s.0 / mbar_raw, s.1 / (mbar_raw * mbar_raw)))
            .filter(|s| s.1 > 0.0)
            .collect();
        if stats.len() < 2 {
            continue;
        }
        let w: Vec<f64> = stats.iter().map(|s| 1.0 / s.1).collect();
        let tw: f64 = w.iter().sum();
        let mbar = stats.iter().zip(&w).map(|(s, w)| s.0 * w).sum::<f64>() / tw;
        for (s, w) in stats.iter().zip(&w) {
            q += w * (s.0 - mbar).powi(2);
            maxz = maxz.max((s.0 - mbar).abs() * w.sqrt());
        }
        sw += tw;
        sw2 += w.iter().map(|w| w * w).sum::<f64>();
        df += stats.len() - 1;
    }
    let tau2 = ((q - df as f64) / (sw - sw2 / sw.max(1e-300))).max(0.0);
    (q, tau2.sqrt(), df, maxz)
}

fn heterogeneity(
    runs: &[Run],
    ncurves: usize,
    m: usize,
    use_cells: &[usize],
    rng: &mut StdRng,
) -> Value {
    let cells = index(runs, ncurves);
    let (q, tau, df, z) = cochran(&cells, m, use_cells);
    let (mut qs, mut taus, mut zs) = (Vec::new(), Vec::new(), Vec::new());
    for _ in 0..PERMS {
        let mut shuffled: Vec<Run> = Vec::with_capacity(runs.len());
        for per in &cells {
            let mut labels: Vec<usize> = use_cells
                .iter()
                .flat_map(|&c| std::iter::repeat(c).take(per[c].len()))
                .collect();
            labels.shuffle(rng);
            let members = use_cells.iter().flat_map(|&c| per[c].iter());
            for (r, cell) in members.zip(labels) {
                shuffled.push(Run {
                    cell,
                    ..(*r).clone()
                });
            }
        }
        let sc = index(&shuffled, ncurves);
        let (q, t, _, z) = cochran(&sc, m, use_cells);
        qs.push(q);
        taus.push(t);
        zs.push(z);
    }
    let p = |obs: f64, null: &[f64]| {
        (null.iter().filter(|&&x| x >= obs - 1e-12).count() + 1) as f64 / (null.len() + 1) as f64
    };
    json!({
        "q": q, "df": df, "q_perm_p": p(q, &qs), "q_null_p95": quantile(&mut qs.clone(), 0.95),
        "tau_rel": tau, "tau_null_p95": quantile(&mut taus.clone(), 0.95),
        "max_abs_z": z, "max_z_perm_p": p(z, &zs), "perms": PERMS,
    })
}

fn index(runs: &[Run], ncurves: usize) -> Cells<'_> {
    let mut cells: Cells = vec![vec![Vec::new(); CELLS.len()]; ncurves];
    for r in runs {
        cells[r.curve][r.cell].push(r);
    }
    cells
}

/// Ratio with a bootstrap over runs within every (curve, cell).
fn ratio_with_ci(
    runs: &[Run],
    ncurves: usize,
    num: &[usize],
    den: &[usize],
    rng: &mut StdRng,
) -> Vec<[f64; 3]> {
    let cells = index(runs, ncurves);
    let obs: Vec<f64> = (0..METRICS.len())
        .map(|m| geo_ratio(&cells, m, num, den, 1.0))
        .collect();
    let mut boots: Vec<Vec<f64>> = vec![Vec::with_capacity(RESAMPLES); METRICS.len()];
    let used: Vec<usize> = num.iter().chain(den).copied().collect();
    for _ in 0..RESAMPLES {
        let mut sample: Cells = vec![vec![Vec::new(); CELLS.len()]; ncurves];
        for (ci, per) in cells.iter().enumerate() {
            for &c in &used {
                let n = per[c].len();
                sample[ci][c] = (0..n).map(|_| per[c][rng.gen_range(0..n)]).collect();
            }
        }
        for (m, b) in boots.iter_mut().enumerate() {
            b.push(geo_ratio(&sample, m, num, den, 1.0));
        }
    }
    obs.iter()
        .zip(boots.iter_mut())
        .map(|(&o, b)| [o, quantile(b, 0.05), quantile(b, 0.95)])
        .collect()
}

fn verdict(ci: [f64; 3]) -> &'static str {
    let (lo, hi) = (ci[1], ci[2]);
    if lo >= 1.0 - MARGIN && hi <= 1.0 + MARGIN {
        "equivalent"
    } else if lo > 1.0 || hi < 1.0 {
        "different"
    } else {
        "inconclusive"
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out = &args[0];
    let gens: Vec<Value> = std::fs::read_to_string(&args[1])
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let mut curve_ids: Vec<String> = gens
        .iter()
        .map(|g| g["curve"].as_str().unwrap().to_string())
        .collect();
    curve_ids.dedup();
    let ncurves = curve_ids.len();
    let cell_of: HashMap<(u64, u64, u64), (usize, usize)> = gens
        .iter()
        .map(|g| {
            let curve = curve_ids
                .iter()
                .position(|c| c == g["curve"].as_str().unwrap())
                .unwrap();
            let cell = CELLS
                .iter()
                .position(|c| *c == g["cell"].as_str().unwrap())
                .unwrap();
            let key = (
                g["p"].as_u64().unwrap(),
                g["gx"].as_u64().unwrap(),
                g["gy"].as_u64().unwrap(),
            );
            (key, (curve, cell))
        })
        .collect();
    let mut by_arm: BTreeMap<String, Vec<Run>> = BTreeMap::new();
    // Exact by-products: (arm, curve) -> distinct factor-base and
    // oracle-set-up readings over every cell.
    let mut exact: BTreeMap<(String, usize), Exact> = BTreeMap::new();
    let (mut total, mut verified) = (0u64, 0u64);
    for dir in &args[2..] {
        let text = std::fs::read_to_string(format!("{dir}/records.jsonl")).unwrap();
        for line in text.lines() {
            let v: Value = serde_json::from_str(line).unwrap();
            if v["warmup"].as_bool().unwrap_or(false) {
                continue;
            }
            total += 1;
            let ok = v["outcome"]["status"] == "verified";
            verified += u64::from(ok);
            assert!(ok, "unverified record {}", v["record_id"]);
            let w = &v["workload"];
            let cs = &w["curve_spec"];
            let key = (
                cs["p"].as_u64().unwrap(),
                cs["gx"].as_u64().unwrap(),
                cs["gy"].as_u64().unwrap(),
            );
            let (curve, mut cell) = cell_of[&key];
            if w["target_seed"].as_u64() == Some(20261020) {
                assert_eq!(cell, 0, "A/A records are gstd");
                cell = AA;
            }
            let r = w["curve"]["r"].as_f64().unwrap();
            let gae = v["cost"]["total_gae"].as_f64().unwrap();
            let ops = unpriced_ops(&v);
            let c = &v["counters"];
            let mut vals = [(0.0, 1.0); 4];
            vals[0] = (
                c["relations_found"].as_f64().unwrap(),
                c["targets_tried"].as_f64().unwrap(),
            );
            for (i, wt) in WEIGHTS.iter().enumerate() {
                vals[i + 1] = ((gae + wt * ops) / r.sqrt(), 1.0);
            }
            let arm = v["arm"].as_str().unwrap().to_string();
            let e = exact.entry((arm.clone(), curve)).or_insert(Exact {
                fb: Vec::new(),
                setup: Vec::new(),
            });
            e.fb.push(phase_key(&v, "factor_base"));
            e.setup.push(phase_key(&v, "oracle_setup"));
            by_arm.entry(arm).or_default().push(Run {
                curve,
                cell,
                v: vals,
            });
        }
    }
    let mut rng = StdRng::seed_from_u64(20261019);
    let mut arms_out = Vec::new();
    for (arm, runs) in &by_arm {
        eprintln!("{arm}: {} runs", runs.len());
        let cells = index(runs, ncurves);
        let per_cell: Vec<Value> = (0..ncurves)
            .flat_map(|ci| {
                let cells = &cells;
                (0..CELLS.len()).filter_map(move |c| {
                    let rs = &cells[ci][c];
                    (!rs.is_empty()).then(|| {
                        let m: Vec<Value> = (0..METRICS.len())
                            .map(|m| {
                                let (mean, se2) = ratio_se2(rs, m);
                                json!({"mean": mean, "se": se2.sqrt()})
                            })
                            .collect();
                        json!({"curve": ci + 1, "cell": CELLS[c], "runs": rs.len(),
                               "found": rs.iter().map(|r| r.v[0].0).sum::<f64>(),
                               "tried": rs.iter().map(|r| r.v[0].1).sum::<f64>(),
                               "metrics": m})
                    })
                })
            })
            .collect();
        // The planted control's data: every gstd run's numerator times 1.03.
        let planted_runs: Vec<Run> = runs
            .iter()
            .map(|r| {
                let mut p = r.clone();
                if p.cell == 0 {
                    for x in &mut p.v {
                        x.0 *= PLANT;
                    }
                }
                p
            })
            .collect();
        let main = ratio_with_ci(runs, ncurves, &[0], &[1, 2, 3, 4], &mut rng);
        let aa = ratio_with_ci(runs, ncurves, &[0], &[AA], &mut rng);
        let mut metrics = Vec::new();
        for (m, name) in METRICS.iter().enumerate() {
            let planted = [main[m][0] * PLANT, main[m][1] * PLANT, main[m][2] * PLANT];
            metrics.push(json!({
                "metric": name,
                "gstd_over_grand": {"ratio": main[m][0], "ci90": [main[m][1], main[m][2]], "verdict": verdict(main[m])},
                "aa_gstd_over_gstd_aa": {"ratio": aa[m][0], "ci90": [aa[m][1], aa[m][2]], "verdict": verdict(aa[m])},
                "planted_plus3": {"ratio": planted[0], "ci90": [planted[1], planted[2]], "detected": planted[1] > 1.0},
                "heterogeneity_generators": heterogeneity(runs, ncurves, m, &[0, 1, 2, 3, 4], &mut rng),
                "heterogeneity_generators_planted": heterogeneity(&planted_runs, ncurves, m, &[0, 1, 2, 3, 4], &mut rng),
                "heterogeneity_aa": heterogeneity(runs, ncurves, m, &[0, AA], &mut rng),
            }));
        }
        let exact_rows: Vec<Value> = exact
            .iter()
            .filter(|((a, _), _)| a == arm)
            .map(|((_, c), e)| {
                let distinct = |v: &[f64]| {
                    let mut s = v.to_vec();
                    s.sort_by(|a, b| a.total_cmp(b));
                    s.dedup();
                    s.len()
                };
                json!({"curve": c + 1, "records": e.fb.len(),
                       "distinct_factor_base": distinct(&e.fb), "distinct_oracle_setup": distinct(&e.setup)})
            })
            .collect();
        arms_out.push(json!({"arm": arm, "runs": runs.len(), "metrics": metrics,
                             "cells": per_cell, "exact": exact_rows}));
    }
    let doc = json!({
        "study": "research/generator_effect_ic_20261019 E4",
        "records": total, "verified": verified, "curves": curve_ids,
        "resamples": RESAMPLES, "margin": MARGIN, "plant": PLANT, "weights": WEIGHTS,
        "arms": arms_out,
    });
    std::fs::write(out, serde_json::to_string_pretty(&doc).unwrap()).unwrap();
    eprintln!("wrote {out}: {total} records, {verified} verified");
}
