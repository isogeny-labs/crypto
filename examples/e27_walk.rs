//! **E27 of `RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` (§9.25), Stage W: the
//! matched walk with cheaper arithmetic, checked bit for bit.**
//!
//! On E25's curves (`generate_gls_klein_instance(p, seed, 16)`) the
//! `⟨±ψ⟩` and negation walks with E25's seeds (`1000·seed + k`, `k < 16`)
//! run three ways: (i) E25's arithmetic (schoolbook products, one `F_p`
//! inversion per addition), serial; (ii) Karatsuba products, serial; (iii)
//! Karatsuba products with the norm inversions of a pool of walks batched
//! by Montgomery's trick (`quartic_folds::rho4_folded_batched`).  (i) is
//! compared field for field with E25's records, and (ii) and (iii) with (i):
//! steps, group operations, cycle escapes, recovered logarithm, correctness
//! and the digest of every representative visited.
//!
//! ```text
//! cargo run --release --example e27_walk -- --sizes 101,149,197,269,397 --seeds 1,2 \
//!   --e25 experiments/30_gls_v4 --json experiments/32_walk_karatsuba/stage_w.json
//! ```

use std::env;
use std::fs;

use crypto_lib::cryptanalysis::gaudry_quartic::Fp4;
use crypto_lib::cryptanalysis::quartic_folds::{
    generate_gls_klein_instance, rho4_folded_batched, rho4_folded_with, RhoFold, RhoRun4F,
};
use rand::rngs::StdRng;
use rand::SeedableRng;
use rayon::prelude::*;
use serde_json::{json, Value};

fn fold_name(f: RhoFold) -> &'static str {
    match f {
        RhoFold::Gls => "Gls",
        RhoFold::Negation => "Negation",
        RhoFold::None => "None",
        RhoFold::Psi => "Psi",
        RhoFold::Twist24 => "Twist24",
    }
}

fn mean(ws: &[RhoRun4F], f: impl Fn(&RhoRun4F) -> u64) -> f64 {
    ws.iter().map(|w| f(w) as f64).sum::<f64>() / ws.len().max(1) as f64
}

/// A walk's cost by E25's convention: the `[d]G = Q` check is not charged.
fn walk_muls(w: &RhoRun4F) -> u64 {
    w.fp_muls - w.verify_fp_muls
}

fn same_walk(a: &RhoRun4F, b: &RhoRun4F) -> bool {
    (
        a.steps,
        a.group_ops,
        a.cycle_escapes,
        a.recovered,
        a.correct,
        a.visit_digest,
    ) == (
        b.steps,
        b.group_ops,
        b.cycle_escapes,
        b.recovered,
        b.correct,
        b.visit_digest,
    )
}

/// Does a recorded E25 walk equal walk (i), field for field?
fn equals_record(rec: &Value, w: &RhoRun4F) -> bool {
    rec["steps"].as_u64() == Some(w.steps)
        && rec["group_ops"].as_u64() == Some(w.group_ops - w.verify_group_ops)
        && rec["fp_muls"].as_u64() == Some(w.fp_muls - w.verify_fp_muls)
        && rec["cycle_escapes"].as_u64() == Some(w.cycle_escapes)
        && rec["correct"].as_bool() == Some(w.correct)
        // E25 records carry `correct` but not `recovered`.
        && (rec.get("recovered").is_none() || rec["recovered"].as_u64() == w.recovered)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut sizes: Vec<u64> = vec![101, 149, 197, 269, 397];
    let mut seeds: Vec<u64> = vec![1, 2];
    let mut walks = 16u64;
    let mut pool = 16usize;
    let mut e25_dir = "experiments/30_gls_v4".to_string();
    let mut json_path: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--sizes" => {
                i += 1;
                sizes = args[i].split(',').map(|v| v.parse().unwrap()).collect();
            }
            "--seeds" => {
                i += 1;
                seeds = args[i].split(',').map(|v| v.parse().unwrap()).collect();
            }
            "--walks" => {
                i += 1;
                walks = args[i].parse().unwrap();
            }
            "--pool" => {
                i += 1;
                pool = args[i].parse().unwrap();
            }
            "--e25" => {
                i += 1;
                e25_dir = args[i].clone();
            }
            "--json" => {
                i += 1;
                json_path = Some(args[i].clone());
            }
            other => panic!("unknown argument {other}"),
        }
        i += 1;
    }

    // Per-operation costs, measured on the counters (no walk needed).
    let ops = {
        let ki = generate_gls_klein_instance(101, 1, 16).unwrap();
        let inst = ki.inst;
        let mut out = serde_json::Map::new();
        for karatsuba in [false, true] {
            let f = Fp4::with_arith(101, karatsuba);
            let mut rng = StdRng::seed_from_u64(7);
            let (a, b) = (f.random(&mut rng), f.random(&mut rng));
            f.reset_muls();
            f.mul(&a, &b);
            let mul = f.muls();
            f.reset_muls();
            f.inv(&a);
            let inv = f.muls();
            let c = crypto_lib::cryptanalysis::gaudry_quartic::Curve4::with_params_arith(
                inst.p, inst.a, inst.b, karatsuba,
            );
            let p2 = c.mul(&inst.g, 12345);
            c.f.reset_muls();
            c.add(&inst.g, &p2);
            let add = c.f.muls();
            c.f.reset_muls();
            c.add(&p2, &p2);
            let dbl = c.f.muls();
            c.f.reset_muls();
            inst.psi_gls(&c, &p2);
            let psi = c.f.muls();
            out.insert(
                if karatsuba { "karatsuba" } else { "schoolbook" }.into(),
                json!({"fp4_mul": mul, "fp4_inv_unbatched": inv, "add_distinct_unbatched": add,
                       "double_unbatched": dbl, "psi_gls": psi}),
            );
        }
        Value::Object(out)
    };
    println!("e27 per-op costs: {ops}");

    let jobs: Vec<(u64, u64)> = sizes
        .iter()
        .flat_map(|&p| seeds.iter().map(move |&s| (p, s)))
        .collect();
    let rows: Vec<Value> = jobs
        .par_iter()
        .map(|&(p, seed)| {
            let ki = generate_gls_klein_instance(p, seed, 16).expect("E25's curve");
            let inst = ki.inst;
            let e25: Value = serde_json::from_str(
                &fs::read_to_string(format!("{e25_dir}/p{p}s{seed}.json")).expect("E25 record"),
            )
            .unwrap();
            let e25_rho = e25[0]["rho"].as_array().cloned().unwrap_or_default();
            let mut folds_out = serde_json::Map::new();
            for fold in [RhoFold::Gls, RhoFold::Negation] {
                let ws: Vec<u64> = (0..walks).map(|k| seed * 1000 + k).collect();
                let school: Vec<RhoRun4F> = ws
                    .par_iter()
                    .map(|&s| rho4_folded_with(&inst, fold, s, false))
                    .collect();
                let kara: Vec<RhoRun4F> = ws
                    .par_iter()
                    .map(|&s| rho4_folded_with(&inst, fold, s, true))
                    .collect();
                let (batched, stats) =
                    rho4_folded_batched(&inst, fold, &ws, pool, seed * 1_000_000 + 500_000, true);
                let recorded: Vec<&Value> = e25_rho
                    .iter()
                    .filter(|r| r["fold"].as_str() == Some(fold_name(fold)))
                    .collect();
                let replay_ok = recorded.len() == school.len()
                    && school.iter().all(|w| {
                        recorded
                            .iter()
                            .find(|r| r["seed"].as_u64() == Some(w.seed))
                            .is_some_and(|r| equals_record(r, w))
                    });
                let kara_ok = school.iter().zip(&kara).all(|(a, b)| same_walk(a, b));
                let batch_ok = school.iter().zip(&batched).all(|(a, b)| same_walk(a, b));
                let m = [
                    mean(&school, walk_muls),
                    mean(&kara, walk_muls),
                    mean(&batched, walk_muls),
                ];
                let verify = [
                    mean(&school, |w| w.verify_fp_muls),
                    mean(&kara, |w| w.verify_fp_muls),
                    mean(&batched, |w| w.verify_fp_muls),
                ];
                let gops = mean(&school, |w| w.group_ops - w.verify_group_ops);
                let steps = mean(&school, |w| w.steps);
                println!(
                    "e27 p={p} seed={seed} {:<8} replay(i)=E25 {replay_ok} (ii)=(i) {kara_ok} (iii)=(i) {batch_ok} | steps {steps:.0} gops {gops:.0} | muls (i) {:.4e} (ii) {:.4e} (iii) {:.4e} | (i)/(iii) {:.4} | inv share {:.3} fillers {}",
                    fold_name(fold), m[0], m[1], m[2], m[0] / m[2],
                    stats.recorded_batch_muls as f64 / stats.recorded_inversions.max(1) as f64,
                    stats.filler_walks
                );
                folds_out.insert(
                    fold_name(fold).into(),
                    json!({
                        "replay_matches_e25": replay_ok,
                        "e25_records": recorded.len(),
                        "karatsuba_identical": kara_ok,
                        "batched_identical": batch_ok,
                        "mean_steps": steps,
                        "mean_group_ops": gops,
                        "mean_fp_muls": {"schoolbook": m[0], "karatsuba": m[1], "batched": m[2]},
                        "mean_fp_muls_note": "walk only, E25's convention: the [d]G = Q check is excluded",
                        "mean_verify_fp_muls": {"schoolbook": verify[0], "karatsuba": verify[1], "batched": verify[2]},
                        "batch": stats,
                        "inversion_share": stats.recorded_batch_muls as f64 / stats.recorded_inversions.max(1) as f64,
                        "walks": {"schoolbook": school, "karatsuba": kara, "batched": batched},
                    }),
                );
            }
            json!({"p": p, "seed": seed, "n": inst.n, "bits": (inst.n as f64).log2(),
                   "cofactor": inst.cofactor, "folds": folds_out})
        })
        .collect();
    let out = json!({
        "experiment": "e27",
        "stage": "W",
        "pool": pool,
        "walks_per_fold": walks,
        "per_op_costs": ops,
        "curves": rows,
    });
    if let Some(path) = json_path {
        fs::write(&path, serde_json::to_string_pretty(&out).unwrap()).expect("write json");
    }
}
