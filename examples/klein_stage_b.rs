//! **E22 Stage B of `RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` (§9.7, §9.10):
//! the `V₄`-invariant weighted solve, per call.**
//!
//! Per curve (`quartic_folds::generate_klein_instance`): the precompute and
//! its span check, then planted targets (a signed sum of four base points)
//! and uniform targets (`aG + bQ`), each decomposed by `klein_v4`'s solve and
//! lift-back and compared with the ground truth: every four-subset with every
//! sign on tiny fields, the exact meet-in-the-middle oracle above them.
//!
//! ```text
//! cargo run --release --example klein_stage_b -- --sizes 13,17,29,37,101,269 --planted 4 --uniform 4 --json experiments/28_e22_stage_b.json
//! ```

use std::collections::BTreeSet;
use std::env;
use std::fs;

use crypto_lib::cryptanalysis::d4_weighted_s5::brute_force_decompositions;
use crypto_lib::cryptanalysis::gaudry_quartic::{mitm_signed, pair_table, Pt4, QuarticSolve};
use crypto_lib::cryptanalysis::klein_v4::{
    lift_back_v4, solve_weighted_traced, v4_system, SymmetrisedV4, MACAULAY_BOUND,
};
use crypto_lib::cryptanalysis::quartic_folds::{generate_klein_instance, klein_base};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut sizes: Vec<u64> = vec![13, 17, 29, 37, 101, 269];
    let (mut planted, mut uniform, mut seed) = (4usize, 4usize, 1u64);
    let mut json_path: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--sizes" => {
                i += 1;
                sizes = args[i].split(',').map(|v| v.parse().unwrap()).collect();
            }
            "--planted" => {
                i += 1;
                planted = args[i].parse().unwrap();
            }
            "--uniform" => {
                i += 1;
                uniform = args[i].parse().unwrap();
            }
            "--seed" => {
                i += 1;
                seed = args[i].parse().unwrap();
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
        let ki = match generate_klein_instance(p, seed, 64) {
            Ok(k) => k,
            Err(e) => {
                println!("e22b p={p}: {e}");
                continue;
            }
        };
        let inst = ki.inst;
        let curve = inst.curve();
        let f = &curve.f;
        let mut rng = StdRng::seed_from_u64(seed ^ 0xE22B ^ p);
        let fb = klein_base(&ki, &mut rng).expect("the V₄ base");
        let keyed: Vec<(u64, Pt4)> = fb
            .points
            .iter()
            .map(|pt| (ki.z_of_x(f, &pt.x).unwrap().0[0], *pt))
            .collect();
        let pre = match SymmetrisedV4::precompute(&ki, &curve, &mut rng, 8) {
            Ok(pre) => pre,
            Err(e) => {
                println!("e22b p={p} seed={seed}: precompute failed: {e}");
                rows.push(
                    json!({"experiment": "e22b", "p": p, "seed": seed, "precompute_error": e}),
                );
                continue;
            }
        };
        let table = (p > 37).then(|| pair_table(&curve, &fb.points));
        for k in 0..planted + uniform {
            let is_planted = k < planted;
            let r = if is_planted {
                let mut idx: Vec<usize> = Vec::new();
                while idx.len() < 4 {
                    let j = rng.gen_range(0..fb.points.len());
                    if !idx.contains(&j) {
                        idx.push(j);
                    }
                }
                idx.iter().fold(Pt4::INF, |acc, &j| {
                    if rng.gen_bool(0.5) {
                        curve.add(&acc, &fb.points[j])
                    } else {
                        curve.sub(&acc, &fb.points[j])
                    }
                })
            } else {
                let (a, b) = (rng.gen_range(0..inst.n), rng.gen_range(1..inst.n));
                curve.add(&curve.mul(&inst.g, a), &curve.mul(&inst.q, b))
            };
            if r.inf {
                continue;
            }
            let Some(z_r) = ki.z_of_x(f, &r.x) else {
                continue;
            };
            let before = f.muls();
            let mut fp = 0u64;
            let comps = v4_system(&pre, f, &z_r, &mut fp);
            let mut st = QuarticSolve {
                weil_muls: f.muls() - before + fp,
                ..Default::default()
            };
            let mut srng = StdRng::seed_from_u64(seed * 1000 + k as u64 + p);
            let (sols, degree) = solve_weighted_traced(
                &comps,
                MACAULAY_BOUND,
                p,
                &mut srng,
                &mut st,
                true,
                false,
                None,
                None,
            );
            let solved = sols.is_some();
            let lb = lift_back_v4(
                &ki,
                &curve,
                &sols.clone().unwrap_or_default(),
                &r,
                &mut srng,
            );
            let got: BTreeSet<[u64; 4]> = lb.decompositions.iter().copied().collect();
            let truth: BTreeSet<[u64; 4]> = match &table {
                None => brute_force_decompositions(&curve, &keyed, &r),
                Some(t) => mitm_signed(&curve, &fb.points, t, &r)
                    .into_iter()
                    .map(|terms| {
                        let mut q: [u64; 4] = core::array::from_fn(|i| keyed[terms[i].0].0);
                        q.sort_unstable();
                        q
                    })
                    .collect(),
            };
            let agree = got == truth;
            println!(
                "e22b p={p} seed={seed} target {k} ({}) | solved {solved} degree {degree} dim {} rows×cols {}×{} | C {:.4e} (weil {:.2e}, echelon {:.2e}, nf {:.2e}) | rational {} | decompositions {} truth {} agree {agree} | lift {:.2e}",
                if is_planted { "planted" } else { "uniform" },
                st.dim, st.rows, st.cols, st.total_muls as f64, st.weil_muls as f64,
                st.echelon_muls as f64, st.nf_muls as f64,
                sols.as_ref().map_or(0, |s| s.len()), got.len(), truth.len(), lb.muls as f64,
            );
            rows.push(json!({
                "experiment": "e22b", "p": p, "seed": seed, "n": inst.n, "cofactor": inst.cofactor,
                "base_points": fb.points.len(), "precompute_muls": pre.precompute_muls,
                "target": k, "planted": is_planted, "solved": solved, "degree": degree,
                "solve": st, "rational": sols.as_ref().map_or(0, |s| s.len()),
                "lift": lb, "truth": truth.len(), "agree": agree,
            }));
            if let Some(path) = &json_path {
                fs::write(path, serde_json::to_string_pretty(&rows).unwrap()).expect("write json");
            }
        }
    }
}
