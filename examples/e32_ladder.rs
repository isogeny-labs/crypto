//! **E32 of `RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` (§9.34): the handover
//! ladder.**  Two modes on E25's GLS curves (`generate_gls_klein_instance(p,
//! seed, 16)`), any prime `p ≡ 1 (mod 4)`:
//!
//! - `ic`: the `k = 4` `V₄`/GLS index calculus, arm B (the `⟨V₄, ψ⟩`-folded
//!   base) by default, with E31's solver A (`chain`) and E27's Karatsuba
//!   products — exactly `quartic_pipeline_gls --solver chain --karatsuba`
//!   restricted to one arm (`run_k4_gls_pipeline_with`, unchanged).
//! - `walk`: the matched rho, E27's walk (iii) — the `⟨±ψ⟩` walk with
//!   Karatsuba products and the norm inversions of a pool of walks batched by
//!   Montgomery's trick (`rho4_folded_batched`, unchanged), seeds
//!   `1000·seed + k`, fillers from `1,000,000·seed + 500,000`, as E27.  Walk
//!   cost excludes the `[d]G = Q` check (E25's convention).  Unlike
//!   `e27_walk`, needs no E25 record and runs only arithmetic (iii).
//!
//! ```text
//! RAYON_NUM_THREADS=4 taskset -c 1-3 target/release/examples/e32_ladder ic --p 557 --seed 1 \
//!   --rho-runs 2 --max-residuals 40000000 --json experiments/36_handover_ladder/ic_p557s1.json
//! target/release/examples/e32_ladder walk --p 557 --seed 1 --walks 16 --pool 16 \
//!   --json experiments/36_handover_ladder/walk_p557s1.json
//! ```

use std::env;
use std::fs;
use std::time::Instant;

use crypto_lib::cryptanalysis::klein_v4::V4Solver;
use crypto_lib::cryptanalysis::quartic_folds::{
    generate_gls_klein_instance, rho4_folded_batched, run_k4_gls_pipeline_with, RhoFold,
};
use serde_json::{json, Value};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mode = args.first().cloned().expect("mode: ic | walk");
    let (mut p, mut seed) = (0u64, 0u64);
    let mut rho_runs = 2usize;
    let mut max_residuals = 40_000_000u64;
    let mut arm = "B".to_string();
    let mut walks = 16u64;
    let mut pool = 16usize;
    let mut json_path: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--p" => {
                i += 1;
                p = args[i].parse().unwrap();
            }
            "--seed" => {
                i += 1;
                seed = args[i].parse().unwrap();
            }
            "--rho-runs" => {
                i += 1;
                rho_runs = args[i].parse().unwrap();
            }
            "--max-residuals" => {
                i += 1;
                max_residuals = args[i].parse().unwrap();
            }
            "--arm" => {
                i += 1;
                arm = args[i].clone();
            }
            "--walks" => {
                i += 1;
                walks = args[i].parse().unwrap();
            }
            "--pool" => {
                i += 1;
                pool = args[i].parse().unwrap();
            }
            "--json" => {
                i += 1;
                json_path = Some(args[i].clone());
            }
            other => panic!("unknown argument {other}"),
        }
        i += 1;
    }
    let t0 = Instant::now();
    let out: Value = match mode.as_str() {
        "ic" => {
            crypto_lib::cryptanalysis::gaudry_quartic::set_karatsuba_default(true);
            let fold_psi = match arm.as_str() {
                "A" => false,
                "B" => true,
                _ => panic!("--arm A|B"),
            };
            let r = run_k4_gls_pipeline_with(
                p,
                seed,
                rho_runs,
                max_residuals,
                fold_psi,
                V4Solver::Chain,
            )
            .unwrap_or_else(|e| panic!("e32 ic p={p} seed={seed} arm {arm}: {e}"));
            println!(
                "e32 ic p={} seed={} arm {} n=2^{:.2} h={} cols {} | residuals {} unsolved {} reruns {} fallbacks {:?} | rel {} y {:.1} | C median {:.4e} | S {:.4e} entries {:.4e} | correct {} | {:.0} s",
                r.p, r.seed, arm, r.bits, r.cofactor, r.folded.columns, r.residuals, r.unsolved,
                r.reruns, r.solver_fallbacks, r.folded.relations,
                r.residuals as f64 / r.folded.relations.max(1) as f64, r.call.median,
                r.total_muls as f64, r.macaulay_entries as f64, r.folded.correct, r.wall_ms / 1e3
            );
            let mut v = serde_json::to_value(&r).unwrap();
            v["experiment"] = Value::from("e32");
            v["arm"] = Value::from(arm.as_str());
            v["karatsuba"] = Value::from(true);
            v
        }
        "walk" => {
            let ki = generate_gls_klein_instance(p, seed, 16).expect("E25's curve");
            let inst = ki.inst;
            let ws: Vec<u64> = (0..walks).map(|k| seed * 1000 + k).collect();
            let (batched, stats) = rho4_folded_batched(
                &inst,
                RhoFold::Gls,
                &ws,
                pool,
                seed * 1_000_000 + 500_000,
                true,
            );
            let k = batched.len().max(1) as f64;
            let muls = batched
                .iter()
                .map(|w| (w.fp_muls - w.verify_fp_muls) as f64)
                .sum::<f64>()
                / k;
            let steps = batched.iter().map(|w| w.steps as f64).sum::<f64>() / k;
            let all_correct = batched.iter().all(|w| w.correct);
            println!(
                "e32 walk p={p} seed={seed} n=2^{:.2} | steps {steps:.1} muls (iii) {muls:.6e} per step {:.2} | correct {all_correct} | {:.0} s",
                (inst.n as f64).log2(),
                muls / steps,
                t0.elapsed().as_secs_f64()
            );
            json!({
                "experiment": "e32", "mode": "walk", "p": p, "seed": seed, "n": inst.n,
                "bits": (inst.n as f64).log2(), "cofactor": inst.cofactor,
                "fold": "Gls", "arithmetic": "karatsuba+batched (E27 walk (iii))",
                "pool": pool, "walks": walks,
                "rho_iii_mean_fp_muls": muls,
                "rho_iii_mean_fp_muls_note": "walk only, E25's convention: the [d]G = Q check is excluded",
                "mean_steps": steps, "all_correct": all_correct, "batch": stats,
                "runs": batched.iter().map(|w| json!({
                    "seed": w.seed, "steps": w.steps, "group_ops": w.group_ops,
                    "fp_muls": w.fp_muls, "verify_fp_muls": w.verify_fp_muls,
                    "verify_group_ops": w.verify_group_ops, "cycle_escapes": w.cycle_escapes,
                    "correct": w.correct, "recovered": w.recovered,
                    "visit_digest": format!("{:016x}", w.visit_digest),
                })).collect::<Vec<_>>(),
                "wall_s": t0.elapsed().as_secs_f64(),
            })
        }
        other => panic!("unknown mode {other}"),
    };
    if let Some(path) = json_path {
        fs::write(&path, serde_json::to_string_pretty(&out).unwrap()).expect("write json");
    }
}
