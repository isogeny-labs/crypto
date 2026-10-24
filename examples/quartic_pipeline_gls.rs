//! **E25 of `RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` (§9.15): the GLS
//! endomorphism on the `k = 4` `V₄` line.**
//!
//! On a GLS twist over `F_{p⁴}` (`quartic_folds::generate_gls_klein_instance`)
//! two arms run E24's pipeline on one curve, one base and one target stream:
//! **A**, the `V₄`-folded base (the control), and **B**, the same points
//! folded by `⟨V₄, ψ⟩`.  The matched walk is the `⟨±ψ⟩` walk; the negation
//! and unfolded walks run on the same curve.
//!
//! ```text
//! cargo run --release --example quartic_pipeline_gls -- --sizes 101 --first-seed 1 --seeds 1 --json experiments/30_gls_v4/p101s1.json
//! ```
//!
//! Tables: `cargo run --release --example glv_invariant_experiment_tables
//! -- experiments/30_gls_v4/p*s*.json`.
//!
//! E31 (§9.22): `--solver chain` or `--solver chain-sparsest` decomposes with
//! the chain solve instead; the default (`macaulay`) is E25's solver.

use std::env;
use std::fs;

use crypto_lib::cryptanalysis::klein_v4::V4Solver;
use crypto_lib::cryptanalysis::quartic_folds::run_k4_gls_pipeline_with;
use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut sizes: Vec<u64> = vec![101, 149, 197, 269, 397];
    let mut seeds = 2u64;
    let mut first_seed = 1u64;
    let mut rho_runs = 16usize;
    let mut max_residuals = 200_000u64;
    let mut json: Option<String> = None;
    let mut solver = V4Solver::Macaulay;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--sizes" => {
                i += 1;
                sizes = args[i]
                    .split(',')
                    .map(|v| v.parse().expect("--sizes"))
                    .collect();
            }
            "--first-seed" => {
                i += 1;
                first_seed = args[i].parse().expect("--first-seed");
            }
            "--seeds" => {
                i += 1;
                seeds = args[i].parse().expect("--seeds");
            }
            "--rho-runs" => {
                i += 1;
                rho_runs = args[i].parse().expect("--rho-runs");
            }
            "--max-residuals" => {
                i += 1;
                max_residuals = args[i].parse().expect("--max-residuals");
            }
            "--solver" => {
                i += 1;
                solver = V4Solver::parse(&args[i]).expect("--solver macaulay|chain|chain-sparsest");
            }
            "--json" => {
                i += 1;
                json = Some(args[i].clone());
            }
            other => panic!("unknown argument {other}"),
        }
        i += 1;
    }
    let mut rows: Vec<Value> = Vec::new();
    for &p in &sizes {
        for seed in first_seed..first_seed + seeds {
            for fold_psi in [false, true] {
                let arm = if fold_psi { "B" } else { "A" };
                let r = match run_k4_gls_pipeline_with(
                    p,
                    seed,
                    rho_runs,
                    max_residuals,
                    fold_psi,
                    solver,
                ) {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("e25 p={p} seed={seed} arm {arm}: {e}");
                        continue;
                    }
                };
                let rel = r.folded.relations.max(1) as f64;
                let pct = |v: u64| 100.0 * v as f64 / r.total_muls as f64;
                println!(
                "e25 p={} seed={} arm {} n=2^{:.1} h={} |F|={} cols {} | residuals {} unsolved {} reruns {} | decs {} off-base {} failures {} | rel {} y {:.1} core {} φ {:.3} | C median {:.3e} mean {:.3e} | solve {:.1}% LA {:.2}% other {:.2}% | S {:.3e} rho(±ψ) {:.3e} rho(neg) {:.3e} ratio {:.3e} | correct {} rho {} | {:.0} s",
                r.p, r.seed, arm, r.bits, r.cofactor, r.base_points, r.folded.columns,
                r.residuals, r.unsolved, r.reruns,
                r.decompositions, r.off_base, r.check_failures,
                r.folded.relations, r.residuals as f64 / rel, r.folded.unknowns, r.folded.phi,
                r.call.median, r.call.mean,
                pct(r.weil_muls + r.solve_muls), pct(r.la_fp_muls),
                pct(r.precompute_muls + r.base_muls + r.generation_muls + r.lift_muls + r.check_muls),
                r.total_muls as f64, r.rho_matched_muls, r.rho_negation_muls, r.ratio,
                r.folded.correct, r.rho.iter().all(|w| w.correct),
                r.wall_ms / 1e3
            );
                let mut v = serde_json::to_value(&r).unwrap();
                v["experiment"] = Value::from("e25");
                v["arm"] = Value::from(arm);
                rows.push(v);
                if let Some(path) = &json {
                    fs::write(path, serde_json::to_string_pretty(&rows).unwrap())
                        .expect("write json");
                }
            }
        }
    }
}
