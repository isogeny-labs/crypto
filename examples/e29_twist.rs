//! **E29 of `RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` (§9.27): the order-24
//! twist action on E18's `j = 0` line — kill test, Stage R, Stage A.**
//!
//! ```text
//! cargo run --release --example e29_twist -- --stage k --sizes 241,337,541,601,733,769,997,1069 --json experiments/34_twist24_stage_r/kill.json
//! cargo run --release --example e29_twist -- --stage r --sizes 241 --json experiments/34_twist24_stage_r/stage_r_p241.json
//! cargo run --release --example e29_twist -- --stage a --sizes 241 --json experiments/34_twist24_stage_r/stage_a_p241.json
//! ```
//! Stages `r` and `a` take each curve's canonical `b₀` from the kill test
//! (the least `b₀` with `#(y² = x³ + b₀t³) = n_E18`).

use std::env;
use std::fs;

use crypto_lib::cryptanalysis::quartic_folds::{
    run_e29_kill, run_e29_stage_r, run_k4_twist24, RhoRun4F,
};
use serde_json::{json, Value};

fn mean(ws: &[RhoRun4F], f: impl Fn(&RhoRun4F) -> u64) -> f64 {
    ws.iter().map(|w| f(w) as f64).sum::<f64>() / ws.len().max(1) as f64
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut stage = "k".to_string();
    let mut sizes: Vec<u64> = vec![241, 337, 541, 601, 733, 769, 997, 1069];
    let mut walks = 16usize;
    let mut json_path: Option<String> = None;
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
            "--walks" => {
                i += 1;
                walks = args[i].parse().unwrap();
            }
            "--json" => {
                i += 1;
                json_path = Some(args[i].clone());
            }
            other => panic!("unknown argument {other}"),
        }
        i += 1;
    }
    let mut rows: Vec<Value> = Vec::new();
    for &p in &sizes {
        // E18 ran one curve per p, seed 1.
        let kill = run_e29_kill(p, 1).expect("E18's curve");
        let v = match stage.as_str() {
            "k" => {
                println!(
                    "e29 kill p={} n=2^{:.1} n mod 24 {} mod 96 {} | ψ_p on E {} | λ_p found {} ord {} | ord χ₆(b) {} | ord λ_ω {} | |⟨λ_p, λ_ω, −1⟩| {} | u² ∈ F_p {} | canonical b₀ {:?}",
                    kill.p, kill.bits, kill.n_mod_24, kill.n_mod_96, kill.psi_p_on_curve,
                    kill.lambda_p_found, kill.order_lambda_p, kill.order_chi6_b,
                    kill.order_lambda_omega, kill.group_order, kill.u2_in_fp, kill.canonical_b0
                );
                serde_json::to_value(&kill).unwrap()
            }
            "r" => {
                let b0 = kill.canonical_b0.expect("a canonical model");
                let mut out = Vec::new();
                for karatsuba in [false, true] {
                    let r = run_e29_stage_r(p, b0, 1, walks, karatsuba).expect("stage R");
                    let w = |ws: &[RhoRun4F]| mean(ws, |w| w.fp_muls - w.verify_fp_muls);
                    let s = |ws: &[RhoRun4F]| mean(ws, |w| w.steps);
                    println!(
                        "e29 R p={p} b0={b0} n=2^{:.1} {} | steps 6 {:.0} 24 {:.0} ratio {:.3} | muls 6 {:.4e} 24 {:.4e} ratio {:.3} | all correct {}",
                        r.bits,
                        if karatsuba { "karatsuba " } else { "schoolbook" },
                        s(&r.order6), s(&r.order24), s(&r.order6) / s(&r.order24),
                        w(&r.order6), w(&r.order24), w(&r.order6) / w(&r.order24),
                        r.order6.iter().chain(&r.order24).all(|w| w.correct)
                    );
                    out.push(serde_json::to_value(&r).unwrap());
                }
                json!({"p": p, "kill": kill, "runs": out})
            }
            "a" => {
                let b0 = kill.canonical_b0.expect("a canonical model");
                let r = run_k4_twist24(p, b0, 1, walks).expect("stage A");
                println!(
                    "e29 A p={p} b0={b0} | flat cols {} rel {} correct {} | cubic cols {} rel {} correct {} la {} | twist24 cols {} rel {} correct {} la {}",
                    r.flat.columns, r.flat.relations, r.flat.correct,
                    r.cubic.columns, r.cubic.relations, r.cubic.correct, r.cubic.la_ops,
                    r.twist24.columns, r.twist24.relations, r.twist24.correct, r.twist24.la_ops
                );
                json!({"p": p, "b0": b0, "kill": kill, "report": serde_json::to_value(&r).unwrap()})
            }
            other => panic!("unknown stage {other}"),
        };
        rows.push(v);
        if let Some(path) = &json_path {
            fs::write(path, serde_json::to_string_pretty(&rows).unwrap()).expect("write");
        }
    }
}
