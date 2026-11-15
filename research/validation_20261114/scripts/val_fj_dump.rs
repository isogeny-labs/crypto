//! Validator V (2026-11-14): certificate dump for research/full_jacobian_ic_20261113.
//!
//! Built only in the validator's scratch export of the merged tree (copied to
//! `src/bin/val_fj_dump.rs` there); never added to the repository's Cargo
//! targets.  It reconstructs every instance exactly as `full_jacobian_ic`
//! does (same `cover_select(p, i)`, same `cover_instance(ci, k)`, same seeds
//! and parameters) and runs the solver arms, but checks nothing itself: it
//! prints one JSON line per prime with the curve data, the planted logs and
//! every recovered log and per-run counter, so that independent Python code
//! (`check_fj_certs.py`) can verify each log on E1 and on J_C and recompute
//! the per-prime means of the committed json.
//!
//! Usage: val_fj_dump --primes 211,419 [--stage 1|2] [--index-offset N]
//!        [--instances 3] [--ic-seeds 4] [--ctl-seeds 4] [--rho-seeds 16] [--threads 2]

use rayon::prelude::*;
use serde_json::{json, Value};

use crypto_lib::cryptanalysis::chained_transfer::full_jacobian_ic::{
    cover_instance, run_full_ic, CoverInstance, Source,
};
use crypto_lib::cryptanalysis::chained_transfer::hyper64::H64Div;
use crypto_lib::cryptanalysis::chained_transfer::ic_control::{Arm as IcArm, IcParams, IcResult};
use crypto_lib::cryptanalysis::chained_transfer::instrument::{fb_size_g, ic_on_rung};
use crypto_lib::cryptanalysis::chained_transfer::negation_remeasure::{cover_select, rho_full_ec};

fn arg(name: &str, default: &str) -> String {
    let a: Vec<String> = std::env::args().collect();
    a.iter()
        .position(|x| x == name)
        .and_then(|i| a.get(i + 1).cloned())
        .unwrap_or_else(|| default.to_string())
}

fn div(d: &H64Div) -> Value {
    json!({"deg": d.deg, "u": d.u_coeffs(), "v": d.v_coeffs()})
}

fn pt(p: &Option<(crypto_lib::cryptanalysis::chained_transfer::field::F2, crypto_lib::cryptanalysis::chained_transfer::field::F2)>) -> Value {
    match p {
        None => Value::Null,
        Some((x, y)) => json!([[x.a, x.b], [y.a, y.b]]),
    }
}

fn ic_json(r: &IcResult, rate: f64) -> Value {
    json!({
        "recovered": r.recovered, "certified": r.certified, "exhausted": r.exhausted,
        "group_ops": r.group_ops(), "setup_ops": r.setup_ops, "walk_ops": r.walk_ops,
        "verify_ops": r.verify_ops, "rel_mul": r.rel.mul, "rel_add": r.rel.add,
        "la_mul": r.la.mul, "la_add": r.la.add, "trials": r.trials, "relations": r.relations,
        "fb_cols": r.fb_cols, "bad_relations": r.bad_relations, "attempts": r.attempts,
        "rels_by_lp": r.rels_by_lp, "smooth_rate": if rate.is_nan() { Value::Null } else { json!(rate) },
    })
}

fn main() {
    let threads: usize = arg("--threads", "2").parse().unwrap();
    rayon::ThreadPoolBuilder::new().num_threads(threads).build_global().ok();
    let primes: Vec<u64> = arg("--primes", "211")
        .split(',')
        .filter_map(|x| x.trim().parse().ok())
        .collect();
    let stage: u8 = arg("--stage", "1").parse().unwrap();
    let off: usize = arg("--index-offset", "0").parse().unwrap();
    let ninst: usize = arg("--instances", "3").parse().unwrap();
    let nseeds: u64 = arg("--ic-seeds", "4").parse().unwrap();
    let ctl: u64 = arg("--ctl-seeds", "4").parse().unwrap();
    let rho_seeds: u64 = arg("--rho-seeds", "16").parse().unwrap();
    let max_trials: u64 = 200_000_000;
    for (j, &p) in primes.iter().enumerate() {
        let i = off + j;
        let ci = cover_select(p, i).expect("admissible cover");
        let insts: Vec<CoverInstance> = (0..ninst).map(|k| cover_instance(&ci, k).unwrap()).collect();
        let b = &ci.b;
        let hc = &insts[0].inst.curve;
        let mut inst_v = vec![];
        for c in &insts {
            let te = b.e1.mul(&ci.gen, c.x as u128);
            inst_v.push(json!({
                "x": c.x, "seed": c.seed, "T_E1": pt(&te),
                "G_J": div(&c.inst.g), "T_J": div(&c.inst.target), "Z": div(&c.z),
                "z_order": c.z_order, "jac_order_inst": c.inst.order, "h_inst": c.inst.h,
            }));
        }
        // rho on E1, negation (primary) and plain, the binary's seeds
        let mut rho = vec![];
        for (k, c) in insts.iter().enumerate() {
            let te = b.e1.mul(&ci.gen, c.x as u128);
            for neg in [true, false] {
                let rs: Vec<Value> = (0..rho_seeds)
                    .into_par_iter()
                    .map(|s| {
                        let v = rho_full_ec(&b.e1, ci.gen, te, ci.r, 1000 + s + c.seed % 997, neg);
                        json!({"k": k, "s": s, "neg": neg, "recovered": v.recovered,
                               "certified": v.certified, "total_ops": v.total_ops,
                               "native": v.native()})
                    })
                    .collect();
                rho.extend(rs);
            }
        }
        let mut arms = serde_json::Map::new();
        for label in ["A", "B", "A_S", "B_S"] {
            let n = if label.ends_with("_S") { ctl } else { nseeds };
            let jobs: Vec<(usize, u64)> = (0..insts.len()).flat_map(|k| (0..n).map(move |s| (k, s))).collect();
            let runs: Vec<Value> = jobs
                .into_par_iter()
                .map(|(k, s)| {
                    let c = &insts[k];
                    let (res, rate) = match label {
                        "A" | "B" => {
                            let arm = if label == "A" { IcArm::Full } else { IcArm::DoubleLp };
                            let nb = if label == "A" { 0 } else { fb_size_g(IcArm::DoubleLp, 2, p, 2.0) };
                            let out = run_full_ic(
                                &c.inst,
                                c.z,
                                Source::WholeJ,
                                IcParams { arm, nb, seed: 700 + s + 10 * k as u64, max_trials, verify_relations: false },
                                false,
                            );
                            (out.ic, out.smooth_rate)
                        }
                        _ => {
                            let arm = if label == "A_S" { IcArm::Full } else { IcArm::DoubleLp };
                            (ic_on_rung(&c.inst, arm, 500 + s + 10 * k as u64, max_trials), f64::NAN)
                        }
                    };
                    let mut v = ic_json(&res, rate);
                    v["k"] = json!(k);
                    v["s"] = json!(s);
                    v
                })
                .collect();
            arms.insert(label.into(), Value::Array(runs));
        }
        let line = json!({
            "p": p, "i": i, "stage": stage, "candidate": ci.candidate, "seed0": ci.seed0, "r": ci.r,
            "F_c": b.c, "x0": b.x0,
            "E1": [b.e1.a2, b.e1.a4, b.e1.a6], "E2": [b.e2.a2, b.e2.a4, b.e2.a6],
            "e1_order": b.e1.order(), "e2_order": b.e2.order(),
            "G_E1": pt(&ci.gen), "H64_f": hc.f_coeffs(),
            "instances": inst_v, "rho_E": rho, "arms": arms,
        });
        println!("{}", serde_json::to_string(&line).unwrap());
        eprintln!("p={p} done");
    }
}
