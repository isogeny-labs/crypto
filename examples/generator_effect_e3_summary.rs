//! Aggregate experiment E3 of the generator-effect study
//! (research/generator_effect_ic_20261019/E3-PROTOCOL.md).
//!
//! `cargo run --release --example generator_effect_e3_summary -- OUT.json SIZES.jsonl F4COST.jsonl SESSION_DIR...`
//!
//! Per curve and tower height `t`:
//! - the exact sum counts `N₃` of both bases (from `SIZES.jsonl`) against the
//!   measured hit rates of the full-solve arms, with Wilson 90% intervals;
//! - F4's multiply-adds per call from the capped `sem-chain` / `sem-full`
//!   records, and the per-system spread from `F4COST.jsonl`;
//! - `S(w) = (G − C + w·(U + n_C)) / √r` at `w ∈ {0, 0.1, 1}` for every
//!   measured arm, with `U` and `C` taken by `ecbench::band`'s own functions;
//! - the **substituted** full-solve cost of the F4 arms (PROTOCOL
//!   "substitution"): the grid arm's record, per target, with the grid's
//!   multiplications on refuted trials replaced by F4's mean multiply-adds
//!   per call plus the system multiplications per call;
//! - ratios to `kl-mitm3` and to `rho-neg`, paired by target, with 90%
//!   percentile bootstrap intervals (10 000 resamples, seed 20261019), the
//!   break-even weight `w*` and the crossover `B*` along `t`.
//!
//! Every number it writes is derived from recorded counts; nothing is rerun.
use std::collections::BTreeMap;
use std::path::Path;

use crypto_lib::cryptanalysis::ecbench::band::{calibrated_native, unpriced_total};
use crypto_lib::cryptanalysis::ecbench::record::Record;
use crypto_lib::cryptanalysis::ecbench::runner::read_records;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::{json, Value};

const WEIGHTS: [f64; 3] = [0.0, 0.1, 1.0];
const RESAMPLES: usize = 10_000;

/// A record reduced to what E3 prices: `S(w) = (g + w·u) / √r`.
#[derive(Clone, Debug)]
struct Cost {
    target: u64,
    g: f64,
    u: f64,
    sqrt_r: f64,
    tried: f64,
    found: f64,
    verified: bool,
    calls: f64,
    solver_ops: f64,
    native: BTreeMap<String, f64>,
    group_gae: f64,
}

impl Cost {
    fn s(&self, w: f64) -> f64 {
        (self.g + w * self.u) / self.sqrt_r
    }
}

fn reduce(rec: &Record) -> Cost {
    let (c_gae, n_c) = calibrated_native(rec).expect("calibration");
    let (u, missing) = unpriced_total(rec);
    assert!(
        !missing,
        "{}: an unpriced counter has no count",
        rec.record_id
    );
    let mut native = BTreeMap::new();
    for p in &rec.phases {
        for (k, v) in &p.native {
            *native.entry(k.clone()).or_insert(0.0) += *v as f64;
        }
    }
    let r = rec.workload.curve.r as f64;
    Cost {
        target: rec.workload.target_index,
        g: rec.cost.total_gae.unwrap_or(f64::NAN) - c_gae,
        u: u as f64 + n_c as f64,
        sqrt_r: r.sqrt(),
        tried: rec.counters.get("targets_tried").copied().unwrap_or(0) as f64,
        found: rec.counters.get("relations_found").copied().unwrap_or(0) as f64,
        verified: rec.outcome.status == "verified",
        calls: rec.solver.as_ref().map_or(0.0, |s| s.calls as f64),
        solver_ops: rec.solver.as_ref().map_or(0.0, |s| s.ops as f64),
        native,
        group_gae: rec.phases.iter().map(|p| p.gae).sum(),
    }
}

fn mean(x: &[f64]) -> f64 {
    x.iter().sum::<f64>() / x.len().max(1) as f64
}

fn boot(rng: &mut StdRng, n: usize, stat: impl Fn(&[usize]) -> f64) -> [f64; 2] {
    let mut v: Vec<f64> = (0..RESAMPLES)
        .map(|_| {
            let idx: Vec<usize> = (0..n).map(|_| rng.gen_range(0..n)).collect();
            stat(&idx)
        })
        .collect();
    v.sort_by(|a, b| a.total_cmp(b));
    [v[RESAMPLES * 5 / 100], v[RESAMPLES * 95 / 100]]
}

/// Wilson score interval at 90% (z = 1.6449).
fn wilson(k: f64, n: f64) -> [f64; 2] {
    if n == 0.0 {
        return [f64::NAN, f64::NAN];
    }
    let z = 1.644_853_626_951_472_2_f64;
    let ph = k / n;
    let den = 1.0 + z * z / n;
    let c = (ph + z * z / (2.0 * n)) / den;
    let h = z * (ph * (1.0 - ph) / n + z * z / (4.0 * n * n)).sqrt() / den;
    [(c - h).max(0.0), c + h]
}

fn read_jsonl(path: &str) -> Vec<Value> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

/// Mean over targets of `S_a / S_b` paired by target, with its interval
/// as a ratio of means.
fn paired(rng: &mut StdRng, a: &[f64], b: &[f64]) -> Value {
    let n = a.len().min(b.len());
    let rm = mean(&a[..n]) / mean(&b[..n]);
    let ci = boot(rng, n, |idx| {
        let sa: f64 = idx.iter().map(|&i| a[i]).sum();
        let sb: f64 = idx.iter().map(|&i| b[i]).sum();
        sa / sb
    });
    json!({"ratio": rm, "ci90": ci})
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (out, sizes, f4cost) = (&args[0], &args[1], &args[2]);
    let sizes = read_jsonl(sizes);
    let f4 = read_jsonl(f4cost);
    let mut rng = StdRng::seed_from_u64(20261019);
    let mut curves_out = Vec::new();
    for dir in &args[3..] {
        let recs = read_records(Path::new(dir)).expect("records");
        let session = recs[0].session_id.clone();
        let p: u64 = match &recs[0].workload.curve_spec {
            crypto_lib::cryptanalysis::ecbench::workload::CurveSpec::PrimeExplicit {
                p, ..
            } => *p,
            _ => panic!("E3 runs on explicit prime curves"),
        };
        let r = recs[0].workload.curve.r as f64;
        let mut arms: BTreeMap<String, Vec<Cost>> = BTreeMap::new();
        for rec in recs.iter().filter(|r| !r.warmup) {
            arms.entry(rec.arm.clone()).or_default().push(reduce(rec));
        }
        for v in arms.values_mut() {
            v.sort_by_key(|c| c.target);
        }
        let s_of = |arm: &str, w: f64| -> Vec<f64> {
            arms.get(arm)
                .map(|v| v.iter().map(|c| c.s(w)).collect())
                .unwrap_or_default()
        };
        let rho: Vec<Vec<f64>> = WEIGHTS.iter().map(|&w| s_of("rho-neg", w)).collect();
        let summary_arm = |rng: &mut StdRng, costs: &[Cost], label: &str| -> Value {
            let per_w: Vec<Value> = WEIGHTS
                .iter()
                .enumerate()
                .map(|(i, &w)| {
                    let s: Vec<f64> = costs.iter().map(|c| c.s(w)).collect();
                    let n = s.len();
                    json!({
                        "w": w,
                        "mean_s": mean(&s),
                        "ci90": boot(rng, n, |idx| idx.iter().map(|&k| s[k]).sum::<f64>() / n as f64),
                        "over_rho": paired(rng, &s, &rho[i]),
                    })
                })
                .collect();
            json!({
                "label": label,
                "runs": costs.len(),
                "verified": costs.iter().filter(|c| c.verified).count(),
                "mean_g": mean(&costs.iter().map(|c| c.g).collect::<Vec<_>>()),
                "mean_u": mean(&costs.iter().map(|c| c.u).collect::<Vec<_>>()),
                "by_w": per_w,
            })
        };
        let mut rows = Vec::new();
        let rho_summary = arms
            .get("rho-neg")
            .map(|c| summary_arm(&mut rng, c, "measured"));
        let mut ts: Vec<u32> = arms
            .keys()
            .filter_map(|a| a.strip_prefix("sem-grid-t")?.parse().ok())
            .collect();
        ts.sort_unstable();
        for t in ts {
            let size = sizes
                .iter()
                .find(|v| v["p"].as_u64() == Some(p) && v["t"].as_u64() == Some(t as u64))
                .cloned()
                .unwrap_or(Value::Null);
            let grid = arms[&format!("sem-grid-t{t}")].clone();
            let kl = arms[&format!("kl-mitm3-t{t}")].clone();
            let hit = |c: &[Cost]| {
                let (k, n) = (
                    c.iter().map(|x| x.found).sum::<f64>(),
                    c.iter().map(|x| x.tried).sum::<f64>(),
                );
                json!({"found": k, "tried": n, "rate": k / n, "wilson90": wilson(k, n)})
            };
            // Grid multiplications per call (constant per t) and the
            // consistent systems of each grid run.
            let grid_per_call: Vec<f64> = grid
                .iter()
                .map(|c| {
                    c.native
                        .get("pkm_grid_fp_mults_uncharged")
                        .copied()
                        .unwrap_or(0.0)
                        / c.calls
                })
                .collect();
            let mut presentations = Vec::new();
            for pres in ["chain", "full"] {
                let Some(capped) = arms.get(&format!("sem-{pres}-t{t}")) else {
                    continue;
                };
                let calls: f64 = capped.iter().map(|c| c.calls).sum();
                let c_f4 = capped.iter().map(|c| c.solver_ops).sum::<f64>() / calls;
                let sys = capped
                    .iter()
                    .map(|c| {
                        c.native
                            .get("pkm_system_fp_mults_uncharged")
                            .copied()
                            .unwrap_or(0.0)
                    })
                    .sum::<f64>()
                    / calls;
                let disagreements: f64 = capped
                    .iter()
                    .map(|c| {
                        c.native
                            .get("pkm_crosscheck_disagreements")
                            .copied()
                            .unwrap_or(0.0)
                    })
                    .sum();
                // Per-system spread from the f4cost runs.
                let spec = if pres == "chain" {
                    "m=3"
                } else {
                    "m=3,mode=full"
                };
                let sel = |planted: bool| -> Vec<f64> {
                    f4.iter()
                        .filter(|v| {
                            v["p"].as_u64() == Some(p)
                                && v["t"].as_u64() == Some(t as u64)
                                && v["spec"] == spec
                                && v["planted"].as_bool() == Some(planted)
                        })
                        .map(|v| v["muladds"].as_f64().unwrap())
                        .collect()
                };
                let (refute, consistent) = (sel(false), sel(true));
                let mm = |x: &[f64]| {
                    if x.is_empty() {
                        Value::Null
                    } else {
                        json!({
                            "n": x.len(),
                            "min": x.iter().cloned().fold(f64::INFINITY, f64::min),
                            "max": x.iter().cloned().fold(0.0, f64::max),
                        })
                    }
                };
                // Substitution, per target of the grid arm.
                let subst: Vec<Cost> = grid
                    .iter()
                    .zip(&grid_per_call)
                    .map(|(c, gpc)| {
                        let consistent = c
                            .native
                            .get("pkm_consistent_systems")
                            .copied()
                            .unwrap_or(0.0);
                        let grid_mults = c
                            .native
                            .get("pkm_grid_fp_mults_uncharged")
                            .copied()
                            .unwrap_or(0.0);
                        let mut s = c.clone();
                        s.u = c.u - grid_mults + consistent * gpc + c.calls * (c_f4 + sys);
                        s
                    })
                    .collect();
                // Capped group work per trial, against the other presentation.
                let gae_per_trial = capped.iter().map(|c| c.group_gae).sum::<f64>()
                    / capped.iter().map(|c| c.tried).sum::<f64>();
                let mut by_w = Vec::new();
                for (i, &w) in WEIGHTS.iter().enumerate() {
                    let ss: Vec<f64> = subst.iter().map(|c| c.s(w)).collect();
                    let sk: Vec<f64> = kl.iter().map(|c| c.s(w)).collect();
                    by_w.push(json!({
                        "w": w,
                        "mean_s": mean(&ss),
                        "over_kl": paired(&mut rng, &ss, &sk),
                        "over_rho": paired(&mut rng, &ss, &rho[i]),
                    }));
                }
                // Break-even weight against kl: means of g and u.
                let (gs, us) = (
                    mean(&subst.iter().map(|c| c.g).collect::<Vec<_>>()),
                    mean(&subst.iter().map(|c| c.u).collect::<Vec<_>>()),
                );
                let (gk, uk) = (
                    mean(&kl.iter().map(|c| c.g).collect::<Vec<_>>()),
                    mean(&kl.iter().map(|c| c.u).collect::<Vec<_>>()),
                );
                let w_star = if us != uk {
                    (gk - gs) / (us - uk)
                } else {
                    f64::NAN
                };
                let w_star_ci = boot(&mut rng, subst.len().min(kl.len()), |idx| {
                    let n = idx.len() as f64;
                    let gs: f64 = idx.iter().map(|&i| subst[i].g).sum::<f64>() / n;
                    let us: f64 = idx.iter().map(|&i| subst[i].u).sum::<f64>() / n;
                    let gk: f64 = idx.iter().map(|&i| kl[i].g).sum::<f64>() / n;
                    let uk: f64 = idx.iter().map(|&i| kl[i].u).sum::<f64>() / n;
                    (gk - gs) / (us - uk)
                });
                presentations.push(json!({
                    "presentation": pres,
                    "label": "substituted, not run: the grid arm's full-solve records with F4's measured mean multiply-adds per call in place of the grid multiplications on refuted trials",
                    "capped_runs": capped.len(),
                    "capped_calls": calls,
                    "f4_muladds_per_call": c_f4,
                    "system_mults_per_call": sys,
                    "crosscheck_disagreements": disagreements,
                    "capped_group_gae_per_trial": gae_per_trial,
                    "f4cost_refutations": mm(&refute),
                    "f4cost_consistent": mm(&consistent),
                    "f4_over_grid_per_call": c_f4 / mean(&grid_per_call),
                    "w_star_vs_kl": w_star,
                    "w_star_ci90": w_star_ci,
                    "by_w": by_w,
                }));
            }
            // Grid and kl against each other and rho, measured.
            let mut grid_kl = Vec::new();
            for &w in &WEIGHTS {
                let sg: Vec<f64> = grid.iter().map(|c| c.s(w)).collect();
                let sk: Vec<f64> = kl.iter().map(|c| c.s(w)).collect();
                grid_kl.push(json!({"w": w, "grid_over_kl": paired(&mut rng, &sg, &sk)}));
            }
            let b = size["B"].as_u64().unwrap_or(0);
            rows.push(json!({
                "t": t,
                "B": b,
                "V": size["V"],
                "N3_kummer": size["N_kummer"],
                "N3_knownlog": size["N_knownlog"],
                "h_kummer": size["hit_kummer"],
                "h_knownlog": size["hit_knownlog"],
                "hit_grid": hit(&grid),
                "hit_kl": hit(&kl),
                "grid_mults_per_call": mean(&grid_per_call),
                "grid": summary_arm(&mut rng, &grid, "measured"),
                "kl": summary_arm(&mut rng, &kl, "measured"),
                "grid_vs_kl": grid_kl,
                "f4": presentations,
            }));
        }
        // Crossover along t: where the substituted chain arm over kl
        // crosses 1, per w, interpolated in log B.
        let mut crossover = Vec::new();
        for (i, &w) in WEIGHTS.iter().enumerate() {
            let pts: Vec<(f64, f64)> = rows
                .iter()
                .filter_map(|row| {
                    let f = row["f4"]
                        .as_array()?
                        .iter()
                        .find(|x| x["presentation"] == "chain")?;
                    Some((
                        row["B"].as_f64()?,
                        f["by_w"][i]["over_kl"]["ratio"].as_f64()?,
                    ))
                })
                .collect();
            let mut b_star = Value::Null;
            for win in pts.windows(2) {
                let ((b0, r0), (b1, r1)) = (win[0], win[1]);
                if (r0 - 1.0) * (r1 - 1.0) <= 0.0 && r0 != r1 && b0 > 0.0 && b1 > 0.0 {
                    let (l0, l1) = (r0.ln(), r1.ln());
                    let f = l0 / (l0 - l1);
                    b_star = json!((b0.ln() + f * (b1.ln() - b0.ln())).exp());
                }
            }
            let side = if pts.iter().all(|&(_, r)| r < 1.0) {
                "chain below kl at every t"
            } else if pts.iter().all(|&(_, r)| r > 1.0) {
                "chain above kl at every t"
            } else {
                "crosses"
            };
            crossover.push(json!({"w": w, "B_star": b_star, "side": side, "points": pts}));
        }
        curves_out.push(json!({
            "session": session,
            "p": p,
            "r": r,
            "rho": rho_summary,
            "rows": rows,
            "crossover_chain_vs_kl": crossover,
        }));
    }
    let doc = json!({
        "schema": "generator-effect-e3-summary/v1",
        "protocol": "research/generator_effect_ic_20261019/E3-PROTOCOL.md (frozen in ede431274)",
        "weights": WEIGHTS,
        "resamples": RESAMPLES,
        "seed": 20261019,
        "curves": curves_out,
    });
    std::fs::write(out, serde_json::to_string_pretty(&doc).unwrap()).unwrap();
}
