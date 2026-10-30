//! **E28 of `RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` (§9.26): the GLS fold
//! on E19's single-`τ_T` line, stages 0 and A (A2 is in `e28_tables`).**
//!
//! ```text
//! cargo run --release --example e28_tau -- --stage 0 --sizes 53,101,149,197,269 --seeds 1-8 \
//!   --json experiments/33_tau_stage_a/stage0.json
//! cargo run --release --example e28_tau -- --stage a --sizes 53 --seeds 1-1 --control 3 \
//!   --json experiments/33_tau_stage_a/stage_a_p53s1.json
//! ```

use std::env;
use std::fs;

use crypto_lib::cryptanalysis::quartic_folds::{
    run_gls_tau_control_decomposed, run_gls_tau_stage0, run_gls_tau_stage_a,
};
use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut stage = "0".to_string();
    let mut sizes: Vec<u64> = vec![53, 101, 149, 197, 269];
    let (mut s0, mut s1) = (1u64, 8u64);
    let mut control = 3usize;
    let mut rho_runs = 16usize;
    let mut max_residuals = 20_000_000u64;
    let mut json: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--stage" => {
                i += 1;
                stage = args[i].clone();
            }
            "--sizes" => {
                i += 1;
                sizes = args[i].split(',').map(|v| v.parse().unwrap()).collect();
            }
            "--seeds" => {
                i += 1;
                let (a, b) = args[i].split_once('-').expect("--seeds a-b");
                (s0, s1) = (a.parse().unwrap(), b.parse().unwrap());
            }
            "--control" => {
                i += 1;
                control = args[i].parse().unwrap();
            }
            "--rho-runs" => {
                i += 1;
                rho_runs = args[i].parse().unwrap();
            }
            "--max-residuals" => {
                i += 1;
                max_residuals = args[i].parse().unwrap();
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
        for seed in s0..=s1 {
            let v = if stage == "c" {
                // Supplementary control (§9.30): D4 solves on decomposed targets.
                match run_gls_tau_control_decomposed(p, seed, control) {
                    Ok((calls, drawn)) => {
                        for c in &calls {
                            println!(
                                "e28 control-decomposed p={p} seed={seed} C={:.3e} {} solve {} oracle {} agrees {}",
                                c.call_muls as f64, c.outcome, c.decompositions, c.mitm_decompositions, c.agrees
                            );
                        }
                        serde_json::json!({"p": p, "seed": seed, "drawn": drawn, "calls": calls})
                    }
                    Err(e) => serde_json::json!({"p": p, "seed": seed, "error": e}),
                }
            } else if stage == "0" {
                match run_gls_tau_stage0(p, seed) {
                    Ok(r) => {
                        println!(
                            "e28 stage0 p={} seed={} n=2^{:.1} h={} j∈F_p² {} ψ²=−1 {} ψG=λG {} λ²=−1 {} ψT=T {} | base {} density {:.3} cols τ {} τψ {} | pts/col {:.2} / {:.2} | line checks {}",
                            r.p, r.seed, r.bits, r.cofactor, r.j_in_fp2, r.psi_squared_is_minus_one,
                            r.psi_g_is_lambda_g, r.lambda_squared_is_minus_one, r.psi_fixes_t,
                            r.base_points, r.density, r.cols_tau, r.cols_tau_psi,
                            r.points_per_column_tau, r.points_per_column_tau_psi, r.line_checks
                        );
                        serde_json::to_value(&r).unwrap()
                    }
                    Err(e) => {
                        println!("e28 stage0 p={p} seed={seed}: {e}");
                        serde_json::json!({"p": p, "seed": seed, "error": e})
                    }
                }
            } else {
                match run_gls_tau_stage_a(p, seed, rho_runs, control, max_residuals) {
                    Ok(r) => {
                        let ctrl: Vec<String> = r
                            .control_calls
                            .iter()
                            .map(|c| {
                                format!(
                                    "{:.3e}/{}/{}={}",
                                    c.call_muls as f64,
                                    c.outcome,
                                    c.decompositions,
                                    c.mitm_decompositions
                                )
                            })
                            .collect();
                        println!(
                            "e28 stageA p={} seed={} n=2^{:.1} h={} base {} | residuals {} decs {} decomposed {} | τ: cols {} rel {} solved {} correct {} | τψ: cols {} rel {} solved {} correct {} | control {:?} | {:.0} s",
                            r.p, r.seed, r.bits, r.cofactor, r.base_points, r.residuals,
                            r.decompositions, r.targets_decomposed, r.tau.columns, r.tau.relations,
                            r.tau.solved, r.tau.correct, r.tau_psi.columns, r.tau_psi.relations,
                            r.tau_psi.solved, r.tau_psi.correct, ctrl, r.wall_ms / 1e3
                        );
                        serde_json::to_value(&r).unwrap()
                    }
                    Err(e) => {
                        println!("e28 stageA p={p} seed={seed}: {e}");
                        serde_json::json!({"p": p, "seed": seed, "error": e})
                    }
                }
            };
            rows.push(v);
            if let Some(path) = &json {
                fs::write(path, serde_json::to_string_pretty(&rows).unwrap()).expect("write");
            }
        }
    }
}
