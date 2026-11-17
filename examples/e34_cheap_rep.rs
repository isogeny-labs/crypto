//! **E34 of `RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` (§9.38): a cheaper
//! order-24 class representative on E18's `j = 0` curves, every per-step
//! operation counted and emitted, E29's Stage R re-run.**
//!
//! ```text
//! RAYON_NUM_THREADS=1 cargo run --release --example e34_cheap_rep -- --stage i --json experiments/38_twist24_cheap_rep/identity.json
//! RAYON_NUM_THREADS=1 cargo run --release --example e34_cheap_rep -- --stage r --sizes 241 --json experiments/38_twist24_cheap_rep/stage_r_p241.json
//! RAYON_NUM_THREADS=1 cargo run --release --example e34_cheap_rep -- --stage n --sizes 241 --json experiments/38_twist24_cheap_rep/negative_walks.json
//! ```
//! Each curve's canonical `b₀` comes from E29's kill test (`run_e29_kill(p, 1)`),
//! walk seeds are E29's (`1000 + k`, `k < 16`).  Serial, one core.

use std::env;
use std::fs;

use crypto_lib::cryptanalysis::quartic_folds::{construct_twist24, run_e29_kill};
use crypto_lib::cryptanalysis::twist24_rep::{identity_check, rho_j0_counted, CountedWalk, RepArm};
use serde_json::{json, Value};

const ARMS: [RepArm; 5] = [
    RepArm::Order6Built,
    RepArm::Order6Cheap,
    RepArm::Order24Built,
    RepArm::Order24Cheap,
    RepArm::Order24CheapMulLean,
];

fn mean(ws: &[CountedWalk], f: impl Fn(&CountedWalk) -> u64) -> f64 {
    ws.iter().map(|w| f(w) as f64).sum::<f64>() / ws.len().max(1) as f64
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut stage = "r".to_string();
    let mut sizes: Vec<u64> = vec![241, 337, 541, 601, 733, 769, 997, 1069];
    let mut walks = 16u64;
    let mut points = 200usize;
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
            "--points" => {
                i += 1;
                points = args[i].parse().unwrap();
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
        let kill = run_e29_kill(p, 1).expect("E18's curve");
        let b0 = kill.canonical_b0.expect("a canonical model");
        let inst = construct_twist24(p, b0, 1).expect("E29's canonical model");
        let v = match stage.as_str() {
            "i" => {
                let checks: Vec<Value> = [
                    RepArm::Order24Cheap,
                    RepArm::Order24CheapMulLean,
                    RepArm::Order6Cheap,
                    RepArm::Order24OmitCandidate,
                    RepArm::Order24WrongOrdinate,
                ]
                .iter()
                .map(|&arm| {
                    let c = identity_check(&inst, arm, points, p);
                    println!(
                        "e34 identity p={p} b0={b0} {arm:?} | points {} forced x0=0 {} x=0 {} | members {} mismatches {} | scalar checks {} failures {}",
                        c.points, c.forced_x0_zero, c.x_zero_point, c.members, c.mismatches,
                        c.scalar_checks, c.scalar_failures
                    );
                    serde_json::to_value(&c).unwrap()
                })
                .collect();
                json!({"p": p, "b0": b0, "n": inst.core.n, "checks": checks})
            }
            "r" | "n" => {
                let arms: Vec<RepArm> = if stage == "r" {
                    ARMS.to_vec()
                } else {
                    vec![RepArm::Order24OmitCandidate]
                };
                let arith: &[bool] = if stage == "r" {
                    &[false, true]
                } else {
                    &[false]
                };
                let mut out = Vec::new();
                for &karatsuba in arith {
                    for &arm in &arms {
                        let ws: Vec<CountedWalk> = (0..walks)
                            .map(|k| rho_j0_counted(&inst, arm, 1000 + k, karatsuba))
                            .collect();
                        let steps = mean(&ws, |w| w.run.steps);
                        let calls = mean(&ws, |w| w.rep_calls);
                        println!(
                            "e34 R p={p} b0={b0} {} {arm:?} | steps {:.0} | muls/step {:.2} | rep muls/call {:.3} | native/step {:.3} (cmp {:.3} neg {:.3} modn {:.3} hash {:.3} table {:.3}) | rep cmp/call {:.3} | correct {}",
                            if karatsuba { "karatsuba " } else { "schoolbook" },
                            steps,
                            mean(&ws, |w| w.run.fp_muls - w.run.verify_fp_muls) / steps,
                            mean(&ws, |w| w.rep_fp_muls) / calls,
                            mean(&ws, |w| w.ops.total()) / steps,
                            mean(&ws, |w| w.ops.cmp) / steps,
                            mean(&ws, |w| w.ops.neg) / steps,
                            mean(&ws, |w| w.ops.modn) / steps,
                            mean(&ws, |w| w.ops.hash) / steps,
                            mean(&ws, |w| w.ops.table) / steps,
                            mean(&ws, |w| w.rep_ops.cmp) / calls,
                            ws.iter().all(|w| w.run.correct)
                        );
                        out.extend(ws.iter().map(|w| serde_json::to_value(w).unwrap()));
                    }
                }
                json!({"p": p, "b0": b0, "n": inst.core.n, "bits": (inst.core.n as f64).log2(), "walks": out})
            }
            other => panic!("unknown stage {other}"),
        };
        rows.push(v);
        if let Some(path) = &json_path {
            fs::write(path, serde_json::to_string(&rows).unwrap()).expect("write");
        }
    }
}
