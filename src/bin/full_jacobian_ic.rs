//! `full_jacobian_ic`: open case O3 of the chained-transfer proof note.
//! Genus-2 index calculus with relations from all of `J_C(F_p)` (arm A: full
//! degree-1 base; arm B: double large primes), against the negation-map rho on
//! `E` in the same process, with the subgroup-walk controls and rho on the
//! `r`-subgroup of `J_C` for context.
//!
//! ```text
//! full_jacobian_ic [--primes 211,...] [--stage2 25601,51203,102407]
//!                  [--out-dir DIR] [--threads 4] [--budget-s 3600]
//! ```
//!
//! Protocol (frozen by hash): `research/full_jacobian_ic_20261113/PROTOCOL.md`.
//! Writes `full_jacobian.json`; the table, the SVG and the verdict text are
//! `render.py`.  Exit status 0 only when every log of every arm was certified
//! by the independent `[k]G == T` on `E1` and `[k]phi^*G == phi^*T` on `J_C`.
//! Every figure is `[pcc:unverified]`.

use std::process::ExitCode;
use std::time::Instant;

use clap::Parser;
use rayon::prelude::*;
use serde_json::{json, Value};

use crypto_lib::cryptanalysis::chained_transfer::full_jacobian_ic::{
    certify_both, cover_instance, fit_loglog, run_full_ic, selection_skips, CoverInstance, Source,
};
use crypto_lib::cryptanalysis::chained_transfer::hyper64::{H64Curve, H64Div};
use crypto_lib::cryptanalysis::chained_transfer::ic_control::{Arm as IcArm, IcParams, IcResult};
use crypto_lib::cryptanalysis::chained_transfer::instrument::{fb_size_g, ic_on_rung};
use crypto_lib::cryptanalysis::chained_transfer::negation_remeasure::{
    best_of, cover_select, mean_rho, measure_class_times, report_arm, rho_full, rho_full_ec, Arm,
    Class, ClassTimes, RefSet, RhoFull, BAND, CAL_FACTORS, TIMING_BATCHES, TIMING_BATCH_LEN,
};

#[derive(Parser, Debug)]
#[command(about = "O3: index calculus with relations from all of J_C(F_p) against matched rho")]
struct Args {
    #[arg(long, default_value = "211,419,821,1619,3203,6421,12841")]
    primes: String,
    #[arg(long, default_value = "25601,51203,102407")]
    stage2: String,
    #[arg(long, default_value = "research/full_jacobian_ic_20261113")]
    out_dir: String,
    #[arg(long, default_value_t = 4)]
    threads: usize,
    /// Instances per prime (stage 1).
    #[arg(long, default_value_t = 3)]
    instances: usize,
    /// Algorithm seeds per instance for arms A, B (stage 1).
    #[arg(long, default_value_t = 4)]
    ic_seeds: u64,
    /// Instances and seeds per instance in stage 2.
    #[arg(long, default_value_t = 1)]
    stage2_instances: usize,
    #[arg(long, default_value_t = 2)]
    stage2_ic_seeds: u64,
    #[arg(long, default_value_t = 4)]
    ctl_seeds: u64,
    #[arg(long, default_value_t = 16)]
    rho_seeds: u64,
    #[arg(long, default_value_t = 4)]
    rho_j_seeds: u64,
    #[arg(long, default_value_t = 200_000_000)]
    max_trials: u64,
    /// Wall-clock budget in seconds: a stage-2 prime is not started after it.
    #[arg(long, default_value_t = 3000)]
    budget_s: u64,
    /// Smoke run: unit class times, no calibration (the part says so).
    #[arg(long, default_value_t = false)]
    no_calibration: bool,
}

fn host() -> Value {
    let cpu = std::fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("model name"))
                .and_then(|l| l.split(':').nth(1))
                .map(|x| x.trim().to_string())
        })
        .unwrap_or_default();
    let cpus = std::thread::available_parallelism().map_or(0, |n| n.get());
    json!({"cpu": cpu, "cpus": cpus, "isolation": "none: shared VM, other processes may run"})
}

struct Sample {
    res: IcResult,
    good: bool,
    smooth_rate: f64,
}

fn ic_mean(rs: &[Sample]) -> (Arm, Value) {
    let n = rs.len().max(1) as f64;
    let m = |f: &dyn Fn(&IcResult) -> f64| rs.iter().map(|s| f(&s.res)).sum::<f64>() / n;
    let (go, rm, ra, lm, la) = (
        m(&|r| r.group_ops() as f64),
        m(&|r| r.rel.mul as f64),
        m(&|r| r.rel.add as f64),
        m(&|r| r.la.mul as f64),
        m(&|r| r.la.add as f64),
    );
    let certified = rs.iter().filter(|s| s.good).count();
    let exhausted = rs.iter().filter(|s| s.res.exhausted).count();
    let sr = rs.iter().map(|s| s.smooth_rate).sum::<f64>() / n;
    let (mn, mx) = rs.iter().fold((f64::MAX, 0f64), |(a, b), s| {
        let g = s.res.group_ops() as f64;
        (a.min(g), b.max(g))
    });
    let arm = Arm {
        group_ops: go,
        native: vec![(Class::MulP, rm), (Class::MulR, lm)],
        lower_bound: false,
    };
    let j = json!({
        "runs": rs.len(), "certified": certified, "exhausted": exhausted,
        "group_ops": go, "group_ops_min": mn, "group_ops_max": mx,
        "setup_ops": m(&|r| r.setup_ops as f64),
        "walk_ops": m(&|r| r.walk_ops as f64), "verify_ops": m(&|r| r.verify_ops as f64),
        "rel_mul": rm, "rel_add": ra, "la_mul": lm, "la_add": la,
        "la_graph_mul": m(&|r| r.la_graph.mul as f64), "la_peel_mul": m(&|r| r.la_peel.mul as f64),
        "la_wiedemann_mul": m(&|r| r.la_wied.mul as f64),
        "trials": m(&|r| r.trials as f64), "relations": m(&|r| r.relations as f64),
        "columns": m(&|r| r.fb_cols as f64), "core_cols": m(&|r| r.core_cols as f64),
        "core_rows": m(&|r| r.core_rows as f64),
        "attempts": m(&|r| f64::from(r.attempts)),
        "rels_with_0_lp": m(&|r| r.rels_by_lp[0] as f64),
        "rels_with_1_lp": m(&|r| r.rels_by_lp[1] as f64),
        "rels_with_2_lp": m(&|r| r.rels_by_lp[2] as f64),
        "bad_relations": m(&|r| r.bad_relations as f64),
        "smooth_rate_deg2": sr,
        "wall_s": m(&|r| r.wall_ns as f64 / 1e9),
        "total_w1_with_additions": go + rm + ra + lm + la,
    });
    (arm, j)
}

fn entry(
    name: &str,
    arm: &Arm,
    t_add: f64,
    times: &ClassTimes,
    refs: &RefSet,
    extra: Value,
) -> Value {
    let mut v = serde_json::to_value(report_arm(name, arm, t_add, times, refs)).unwrap();
    if let (Some(o), Some(e)) = (v.as_object_mut(), extra.as_object()) {
        for (k, x) in e {
            o.insert(k.clone(), x.clone());
        }
    }
    v
}

fn unit_times() -> ClassTimes {
    ClassTimes {
        mul_p: 1.0,
        mul_r: 1.0,
        inv_r: 1.0,
        canon: 1.0,
        hash: 1.0,
        lookup: 1.0,
        insert: 1.0,
        draw: 1.0,
        word: 1.0,
    }
}

/// One prime: all arms, references, calibration.  `None` if no admissible cover.
fn run_prime(a: &Args, i: usize, p: u64, stage: u8) -> (Option<Value>, bool) {
    let t0 = Instant::now();
    let Some(ci) = cover_select(p, i) else {
        println!("p={p}: no admissible cover");
        return (
            Some(
                json!({"p": p, "stage": stage, "skipped": "no admissible cover candidate in 4000"}),
            ),
            false,
        );
    };
    let skips = selection_skips(p, i, ci.candidate);
    let (ninst, nseeds) = if stage == 1 {
        (a.instances, a.ic_seeds)
    } else {
        (a.stage2_instances, a.stage2_ic_seeds)
    };
    let insts: Vec<CoverInstance> = (0..ninst)
        .filter_map(|k| match cover_instance(&ci, k) {
            Ok(c) => Some(c),
            Err(e) => {
                println!("p={p} instance {k}: {e}");
                None
            }
        })
        .collect();
    if insts.len() != ninst {
        return (
            Some(json!({"p": p, "stage": stage, "skipped": "instance construction failed"})),
            false,
        );
    }
    let (b, r, e1, ge) = (&ci.b, ci.r, ci.b.e1, ci.gen);
    let hc: &H64Curve = &insts[0].inst.curve;
    let jac_order = e1.order() * b.e2.order();
    let h = jac_order / r;
    // calibration (sequential, before the parallel work)
    let no_cal = a.no_calibration;
    let times = if no_cal {
        unit_times()
    } else {
        measure_class_times(p, r, &e1, ge, TIMING_BATCHES, TIMING_BATCH_LEN)
    };
    let timing = |f: &mut dyn FnMut()| -> f64 {
        if no_cal {
            1.0
        } else {
            best_of(TIMING_BATCHES, TIMING_BATCH_LEN, f)
        }
    };
    let t_add_e = {
        let pts: Vec<_> = (1..=64u128).map(|m| e1.mul(&ge, m * 7)).collect();
        let mut acc = None;
        let mut k = 0usize;
        timing(&mut || {
            acc = e1.add(&acc, &pts[k % 64]);
            k += 1;
            std::hint::black_box(&acc);
        })
    };
    let t_add_h = {
        let g0 = insts[0].inst.g;
        let ring: Vec<H64Div> = (1..=64u64).map(|m| hc.mul_rtl(g0, m * 7)).collect();
        let mut acc = ring[0];
        let mut k = 0usize;
        timing(&mut || {
            acc = hc.cantor(acc, ring[k % 64]);
            k += 1;
            std::hint::black_box(&acc);
        })
    };
    let mut ok = true;

    // matched rho on E1 (negation = primary, plain = context)
    let mut rneg: Vec<RhoFull> = vec![];
    let mut rplain: Vec<RhoFull> = vec![];
    for c in &insts {
        let te = e1.mul(&ge, c.x as u128);
        let run = |neg: bool| -> Vec<RhoFull> {
            (0..a.rho_seeds)
                .into_par_iter()
                .map(|s| rho_full_ec(&e1, ge, te, r, 1000 + s + c.seed % 997, neg))
                .collect()
        };
        rneg.extend(run(true));
        rplain.extend(run(false));
    }
    ok &= rneg.iter().chain(&rplain).all(|x| x.certified);
    let refs = RefSet {
        neg: mean_rho(&rneg).arm(),
        plain: mean_rho(&rplain).arm(),
        t_add_ns: t_add_e,
        times,
    };
    // rho on the order-r subgroup of J_C (context)
    let mut rj: Vec<RhoFull> = vec![];
    for c in &insts {
        let rs: Vec<RhoFull> = (0..a.rho_j_seeds)
            .into_par_iter()
            .map(|s| {
                let (g, t, x) = (c.inst.g, c.inst.target, c.x);
                let mut v = rho_full(&c.inst.curve, g, t, r, 2000 + s, true, &|k| {
                    c.inst.curve.mul_rtl(g, k) == t && k % r == x
                });
                // independent certificate on the library Jacobian
                if let Some(k) = v.recovered {
                    let (_, on_j) = certify_both(&ci, x, k);
                    v.certified &= on_j;
                }
                v
            })
            .collect();
        rj.extend(rs);
    }
    ok &= rj.iter().all(|x| x.certified);
    let rj_mean = mean_rho(&rj);
    let rho_j_arm = rj_mean.arm();

    // IC arms: whole-J A and B; subgroup controls A_S (full) and B_S (double LP)
    let run_arm = |label: &str| -> Vec<Sample> {
        let jobs: Vec<(usize, u64)> = (0..insts.len())
            .flat_map(|k| {
                let n = if label.ends_with("_S") {
                    a.ctl_seeds
                } else {
                    nseeds
                };
                (0..n).map(move |s| (k, s))
            })
            .collect();
        jobs.into_par_iter()
            .map(|(k, s)| {
                let c = &insts[k];
                let (res, rate) = match label {
                    "A" | "B" => {
                        let arm = if label == "A" {
                            IcArm::Full
                        } else {
                            IcArm::DoubleLp
                        };
                        let nb = if label == "A" {
                            0
                        } else {
                            fb_size_g(IcArm::DoubleLp, 2, p, 2.0)
                        };
                        let out = run_full_ic(
                            &c.inst,
                            c.z,
                            Source::WholeJ,
                            IcParams {
                                arm,
                                nb,
                                seed: 700 + s + 10 * k as u64,
                                max_trials: a.max_trials,
                                verify_relations: false,
                            },
                            false,
                        );
                        (out.ic, out.smooth_rate)
                    }
                    _ => {
                        let arm = if label == "A_S" {
                            IcArm::Full
                        } else {
                            IcArm::DoubleLp
                        };
                        (
                            ic_on_rung(&c.inst, arm, 500 + s + 10 * k as u64, a.max_trials),
                            f64::NAN,
                        )
                    }
                };
                let good = res.recovered.is_some_and(|kk| {
                    let (e, j) = certify_both(&ci, c.x, kk);
                    res.certified && c.inst.curve.mul_rtl(c.inst.g, kk) == c.inst.target && e && j
                });
                Sample {
                    res,
                    good,
                    smooth_rate: rate,
                }
            })
            .collect()
    };
    let mut arms = serde_json::Map::new();
    for label in ["A", "B", "A_S", "B_S"] {
        let tt = Instant::now();
        let rs = run_arm(label);
        let (arm, j) = ic_mean(&rs);
        ok &= rs.iter().all(|s| s.good) && rs.iter().all(|s| !s.res.exhausted);
        let mut e = entry(label, &arm, t_add_h, &times, &refs, j);
        e["arm_wall_s"] = json!(tt.elapsed().as_secs_f64());
        println!(
            "p={p} arm {label}: runs {} group_ops {:.0} class {} ratio_band {:?} wall {:.1}s",
            rs.len(),
            arm.group_ops,
            e["class_neg_band"],
            e["ratio_neg_band"],
            tt.elapsed().as_secs_f64()
        );
        arms.insert(label.into(), e);
    }
    arms.insert(
        "rho_J_negation".into(),
        entry(
            "rho_J_negation",
            &rho_j_arm,
            t_add_h,
            &times,
            &refs,
            json!({"note": "context: rho on the order-r subgroup of J_C; canon/hash/lookup classes timed on E1",
                   "runs": rj.len(), "certified": rj.iter().filter(|x| x.certified).count(),
                   "mean": serde_json::to_value(&rj_mean).unwrap()}),
        ),
    );
    let neg_m = mean_rho(&rneg);
    let plain_m = mean_rho(&rplain);
    let z_cov: Vec<f64> = insts
        .iter()
        .map(|c| c.z_order as f64 / jac_order as f64)
        .collect();
    let row = json!({
        "p": p, "stage": stage, "r": r, "x": insts.iter().map(|c| c.x).collect::<Vec<_>>(),
        "candidate": ci.candidate, "seed0": ci.seed0,
        "skipped_candidates": serde_json::to_value(&skips).unwrap(),
        "e1_order": e1.order(), "e2_order": b.e2.order(), "jac_order": jac_order, "h": h,
        "r_squared_divides_jac_order": jac_order % (r * r) == 0,
        "r_divides_e2_order": b.e2.order() % r == 0,
        "z_order_over_jac_order": z_cov,
        "instances": insts.len(), "ic_seeds": nseeds, "ctl_seeds": a.ctl_seeds,
        "rho_seeds_per_instance": a.rho_seeds,
        "calibration_skipped": no_cal,
        "class_times_ns": serde_json::to_value(times).unwrap(),
        "t_add_ns": {"E1": t_add_e, "J_word": t_add_h},
        "rho_E": {"negation": serde_json::to_value(&neg_m).unwrap(),
                  "plain": serde_json::to_value(&plain_m).unwrap(),
                  "band_negation": refs.neg.band(), "cal_negation": refs.neg.cal(&times, t_add_e)},
        "arms": arms,
        "wall_s": t0.elapsed().as_secs_f64(),
    });
    println!("p={p}: done in {:.1}s ok={ok}", t0.elapsed().as_secs_f64());
    (Some(row), ok)
}

/// Fits over a set of rows: slopes of ln(quantity) on ln p.
fn fits(rows: &[Value], label: &str) -> Value {
    let get = |row: &Value, arm: &str, f: &dyn Fn(&Value) -> f64| f(&row["arms"][arm]);
    let ps: Vec<f64> = rows.iter().map(|r| r["p"].as_f64().unwrap()).collect();
    let mut out = serde_json::Map::new();
    let mut put = |name: &str, ys: Vec<f64>| {
        let v = fit_loglog(&ps, &ys).map_or(Value::Null, |(s, a, se, ci)| {
            json!({"slope": s, "intercept": a, "se": se, "ci95": ci, "n": ps.len()})
        });
        out.insert(name.into(), v);
    };
    for arm in ["A", "B", "A_S", "B_S"] {
        put(
            &format!("{arm}_group_ops"),
            rows.iter()
                .map(|r| get(r, arm, &|e| e["group_ops"].as_f64().unwrap()))
                .collect(),
        );
        for (i, w) in ["w0", "w0.1", "w1"].iter().enumerate() {
            put(
                &format!("{arm}_ratio_to_rho_E_{w}"),
                rows.iter()
                    .map(|r| get(r, arm, &|e| e["ratio_neg_band"][i].as_f64().unwrap()))
                    .collect(),
            );
            put(
                &format!("{arm}_total_{w}"),
                rows.iter()
                    .map(|r| get(r, arm, &|e| e["total_band"][i].as_f64().unwrap()))
                    .collect(),
            );
        }
    }
    put(
        "rho_E_negation_group_ops",
        rows.iter()
            .map(|r| r["rho_E"]["negation"]["total_ops"].as_f64().unwrap())
            .collect(),
    );
    put(
        "rho_E_negation_total_w0.1",
        rows.iter()
            .map(|r| r["rho_E"]["band_negation"][1].as_f64().unwrap())
            .collect(),
    );
    put(
        "rho_J_negation_group_ops",
        rows.iter()
            .map(|r| r["arms"]["rho_J_negation"]["group_ops"].as_f64().unwrap())
            .collect(),
    );
    let mut v = Value::Object(out);
    v["label"] = json!(label);
    v
}

/// The frozen decision rule over the rows of stage 1.
fn verdict(rows: &[Value], fit: &Value) -> Value {
    let mut cells_blocking: Vec<String> = vec![];
    let mut per_arm = serde_json::Map::new();
    let mut all_closed = true;
    let mut any_below = false;
    for arm in ["A", "B"] {
        let mut above_band = 0;
        let mut above_cal = 0;
        for r in rows {
            let e = &r["arms"][arm];
            if e["class_neg_band"] == "above" {
                above_band += 1;
            } else {
                cells_blocking.push(format!("{arm} p={} band {}", r["p"], e["class_neg_band"]));
            }
            if e["class_neg_cal"] == "above" {
                above_cal += 1;
            } else {
                cells_blocking.push(format!("{arm} p={} cal {}", r["p"], e["class_neg_cal"]));
            }
            if e["class_neg_band"] == "below" {
                any_below = true;
            }
        }
        let lo = fit[format!("{arm}_ratio_to_rho_E_w0.1")]["ci95"][0].as_f64();
        let slope = fit[format!("{arm}_ratio_to_rho_E_w0.1")]["slope"].as_f64();
        let closed =
            above_band == rows.len() && above_cal == rows.len() && lo.is_some_and(|v| v > 0.2);
        all_closed &= closed;
        per_arm.insert(
            arm.into(),
            json!({"cells": rows.len(), "above_band": above_band, "above_cal": above_cal,
                   "ratio_slope_w0.1": slope, "ratio_slope_ci95_lower": lo, "meets_closed_rule": closed}),
        );
    }
    let v = if any_below {
        "O3 open: a cell is below at all three band prices (re-read with every native counter before using the word advantage)"
    } else if all_closed {
        "O3 closed by data: every cell above at the band and the calibrated reading, loss grows with p (ratio-slope CI lower end > 0.2) for A and B"
    } else {
        "unresolved: the frozen rule is not met by both arms; blocking cells listed"
    };
    json!({"verdict": v, "per_arm": per_arm, "blocking_cells": cells_blocking})
}

fn main() -> ExitCode {
    let a = Args::parse();
    rayon::ThreadPoolBuilder::new()
        .num_threads(a.threads)
        .build_global()
        .ok();
    let t_all = Instant::now();
    let parse =
        |s: &str| -> Vec<u64> { s.split(',').filter_map(|x| x.trim().parse().ok()).collect() };
    let s1 = parse(&a.primes);
    let s2 = parse(&a.stage2);
    let mut rows: Vec<Value> = vec![];
    let mut all_ok = true;
    for (i, &p) in s1.iter().enumerate() {
        let (row, ok) = run_prime(&a, i, p, 1);
        all_ok &= ok;
        rows.extend(row);
    }
    let stage1_rows: Vec<Value> = rows
        .iter()
        .filter(|r| r.get("arms").is_some())
        .cloned()
        .collect();
    let mut stage2_rows: Vec<Value> = vec![];
    let mut not_run: Vec<Value> = vec![];
    for (j, &p) in s2.iter().enumerate() {
        if t_all.elapsed().as_secs() > a.budget_s {
            not_run.push(json!({"p": p, "reason": format!("budget of {} s reached before it started", a.budget_s)}));
            continue;
        }
        let (row, ok) = run_prime(&a, s1.len() + j, p, 2);
        all_ok &= ok;
        if let Some(r) = row {
            if r.get("arms").is_some() {
                stage2_rows.push(r.clone());
            }
            rows.push(r);
        }
    }
    let f1 = fits(&stage1_rows, "stage 1 primes");
    let mut both: Vec<Value> = stage1_rows.clone();
    both.extend(stage2_rows.iter().cloned());
    let f_all = fits(&both, "stage 1 and 2 primes");
    let v = verdict(&stage1_rows, &f1);
    let out = json!({
        "protocol": "research/full_jacobian_ic_20261113/PROTOCOL.md",
        "args": format!("{a:?}"),
        "host": host(),
        "calibration": {
            "method": "best-of-N timing (not perf-ipc; no isolated host): minimum over batches of ns per operation",
            "batches": TIMING_BATCHES, "batch_len": TIMING_BATCH_LEN,
            "definition": "w_cal(class) = time(native operation) / time(one group addition of the arm's own group), same host and process",
            "calibrated_reading": "every native class at 0, 1 and 10 times its own w_cal",
            "band": BAND, "cal_factors": CAL_FACTORS,
        },
        "ok": all_ok,
        "rows": rows,
        "stage2_not_run": not_run,
        "fits_stage1": f1,
        "fits_all": f_all,
        "verdict": v,
        "wall_s": t_all.elapsed().as_secs_f64(),
    });
    let path = format!("{}/full_jacobian.json", a.out_dir);
    std::fs::write(&path, serde_json::to_string_pretty(&out).unwrap()).expect("write json");
    println!(
        "wrote {path}; ok={all_ok}; verdict: {}",
        out["verdict"]["verdict"]
    );
    if all_ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
