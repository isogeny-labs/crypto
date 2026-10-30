//! **E22 Stage A of `RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` (§9.7): the full
//! 2-torsion symmetry `V₄` on a stable line.**
//!
//! Per curve: a curve over `F_{p⁴}` with full rational 2-torsion and the
//! Möbius coordinate in which the translations are `z ↦ −z, 1/z, −1/z`; the
//! base `z ∈ F_p` with every fold checked on points; a meet-in-the-middle
//! relation stream on the `V₄`-folded columns until `[d]G = Q` verifies.  For
//! direction 3, E18's `τ_T` stream on a curve of the same `p` alongside.
//!
//! ```text
//! cargo run --release --example klein_stage_a -- --sizes 53,101,149,197,269 --seeds 2 --json experiments/28_e22_stage_a.json
//! ```

use std::env;
use std::fs;

use crypto_lib::cryptanalysis::quartic_folds::{run_k4_folds, run_klein_stage_a, Family4};
use serde_json::{json, Value};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut sizes: Vec<u64> = vec![53, 101, 149, 197, 269];
    let mut seeds = 2u64;
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
                seeds = args[i].parse().unwrap();
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
        for seed in 1..=seeds {
            let a = match run_klein_stage_a(p, seed, 20_000_000) {
                Ok(a) => a,
                Err(e) => {
                    println!("e22a p={p} seed={seed}: {e}");
                    rows.push(json!({"experiment": "e22a", "p": p, "seed": seed, "error": e}));
                    continue;
                }
            };
            let multi: Vec<(usize, u64)> = a
                .decompositions_per_target
                .iter()
                .enumerate()
                .filter(|&(k, &v)| k > 0 && v > 0)
                .map(|(k, &v)| (k, v))
                .collect();
            let y = a.residuals as f64 / a.folded.relations.max(1) as f64;
            let tau = run_k4_folds(Family4::Translation, p, seed, 0).ok();
            let tau_y = tau
                .as_ref()
                .map(|t| t.residuals as f64 / t.folded.relations.max(1) as f64);
            println!(
                "e22a p={p} seed={seed} n=2^{:.1} h={} pole∈F_p {} | base {} (density {:.3}) cols {} ({:.2}/col) | residuals {} y {:.1} | decs/target {:?} | solved {} correct {} | tau_T same p: y {:?} cols {:?}",
                a.bits, a.cofactor, a.pole_in_fp, a.base_points, a.density, a.columns,
                a.points_per_column, a.residuals, y, multi, a.folded.solved, a.folded.correct,
                tau_y, tau.as_ref().map(|t| t.folded.columns),
            );
            let mut v = serde_json::to_value(&a).unwrap();
            v["experiment"] = Value::from("e22a");
            v["y"] = json!(y);
            v["tau_same_p"] = json!(tau.as_ref().map(|t| json!({
                "seed": t.seed, "n": t.n, "cofactor": t.cofactor, "base_points": t.base_points,
                "columns": t.folded.columns, "residuals": t.residuals,
                "relations": t.folded.relations, "y": tau_y,
                "decompositions_per_target": t.decompositions_per_target,
                "correct": t.folded.correct,
            })));
            rows.push(v);
            if let Some(path) = &json_path {
                fs::write(path, serde_json::to_string_pretty(&rows).unwrap()).expect("write json");
            }
        }
    }
}
