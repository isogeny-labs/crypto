//! **E21 of `RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` (§9.7): trace replay on
//! the weighted `D₄` solve.**
//!
//! On E19's 2-torsion instances, every target's Weil-restricted system is
//! solved twice: in full (as E19 does) and replaying a shape trace recorded
//! on the curve's first target, which skips the rows that reduced to zero.
//! Both decomposition sets (after the charged lift-back) are compared, and
//! both costs are kept, per target.
//!
//! ```text
//! cargo run --release --example d4_replay -- --sizes 53,101 --seed 1 --targets 200 --json experiments/27_d4_replay.json
//! ```

use std::env;
use std::fs;

use crypto_lib::cryptanalysis::d4_weighted_s5::{
    lift_back, solve_weighted_traced, weighted_system, WeightedTrace, MACAULAY_BOUND,
};
use crypto_lib::cryptanalysis::gaudry_quartic::{QuarticSolve, ReplayOutcome};
use crypto_lib::cryptanalysis::gaudry_quartic_d4::{D4Instance, SymmetrisedD4};
use crypto_lib::cryptanalysis::quartic_folds::generate_translation_instance;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use serde_json::{json, Value};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut sizes: Vec<u64> = vec![53, 101];
    let mut seed = 1u64;
    let mut targets = 200usize;
    let mut json_path: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--sizes" => {
                i += 1;
                sizes = args[i].split(',').map(|v| v.parse().unwrap()).collect();
            }
            "--seed" => {
                i += 1;
                seed = args[i].parse().unwrap();
            }
            "--targets" => {
                i += 1;
                targets = args[i].parse().unwrap();
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
        let inst = generate_translation_instance(p, seed, 4).expect("instance");
        let d4 = D4Instance {
            curve: inst.curve(),
            x0: inst.x0,
            c: inst.c,
        };
        let mut rng = StdRng::seed_from_u64(seed ^ 0xE21);
        let pre = SymmetrisedD4::precompute(&d4, &mut rng, 8).expect("H is D₄-invariant");
        let draws: Vec<(u64, u64)> = (0..targets)
            .map(|_| (rng.gen_range(0..inst.n), rng.gen_range(1..inst.n)))
            .collect();
        // One target: the full solve (optionally recording), then the replay.
        let run = |k: usize, trace: Option<&WeightedTrace>, record: Option<&mut WeightedTrace>| {
            let (a, b) = draws[k];
            let di = D4Instance {
                curve: inst.curve(),
                x0: inst.x0,
                c: inst.c,
            };
            let c = &di.curve;
            let r = c.add(&c.mul(&inst.g, a), &c.mul(&inst.q, b));
            let before = c.f.muls();
            let comps = weighted_system(&pre, &c.f, &di.y_of_x(&r.x));
            let weil = c.f.muls() - before;
            let solve = |tr: Option<&WeightedTrace>, rec: Option<&mut WeightedTrace>| {
                let mut st = QuarticSolve {
                    weil_muls: weil,
                    ..Default::default()
                };
                let mut srng = StdRng::seed_from_u64(seed * 100_000 + k as u64);
                let (sols, deg) = solve_weighted_traced(
                    &comps,
                    MACAULAY_BOUND,
                    p,
                    &mut srng,
                    &mut st,
                    true,
                    false,
                    tr,
                    rec,
                );
                let solved = sols.is_some();
                let lb = lift_back(&di, &sols.unwrap_or_default(), &r, &mut srng);
                (st, deg, solved, lb.decompositions)
            };
            let full = solve(None, record);
            let replayed = trace.map(|t| solve(Some(t), None));
            (full, replayed)
        };
        let mut trace = WeightedTrace::default();
        let (first, _) = run(0, None, Some(&mut trace));
        assert_eq!(
            trace.degree, MACAULAY_BOUND,
            "the first target solved at the base degree"
        );
        let outs: Vec<_> = (1..targets)
            .into_par_iter()
            .map(|k| (k, run(k, Some(&trace), None)))
            .collect();
        let (mut agree, mut disagree, mut fell_back) = (0u64, 0u64, 0u64);
        let (mut full_c, mut rep_c) = (Vec::new(), Vec::new());
        let mut per_target = Vec::new();
        for (k, (full, rep)) in outs {
            let rep = rep.unwrap();
            if full.3 == rep.3 && full.2 == rep.2 {
                agree += 1;
            } else {
                disagree += 1;
            }
            if rep.0.replay == ReplayOutcome::FellBack {
                fell_back += 1;
            }
            full_c.push(full.0.total_muls as f64);
            rep_c.push(rep.0.total_muls as f64);
            per_target.push(json!({
                "target": k,
                "full_muls": full.0.total_muls, "replay_muls": rep.0.total_muls,
                "full_echelon": full.0.echelon_muls, "replay_echelon": rep.0.echelon_muls,
                "full_rows": full.0.rows, "replay_rows": rep.0.rows,
                "full_degree": full.1, "replay_degree": rep.1,
                "replay": format!("{:?}", rep.0.replay),
                "pivots": full.0.pivots, "reached_pivots": full.0.reached_pivots,
                "nf_muls": full.0.nf_muls,
                "decompositions": full.3.len(), "agree": full.3 == rep.3 && full.2 == rep.2,
            }));
        }
        let median = |v: &mut Vec<f64>| {
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            v[v.len() / 2]
        };
        let sum = |v: &[f64]| v.iter().sum::<f64>();
        let ratio_sum = sum(&full_c) / sum(&rep_c);
        let (mf, mr) = (median(&mut full_c.clone()), median(&mut rep_c.clone()));
        println!(
            "e21 p={p} seed={seed} targets={} | trace zero rows {} pivots {} dim {} | agree {agree} disagree {disagree} fell back {fell_back} | C full/replay: median {:.4e}/{:.4e} ({:.3}x), total {:.3}x | first: pivots {} reached {}",
            targets - 1, trace.zero_ids.len(), trace.pivot_cols.len(), trace.dim,
            mf, mr, mf / mr, ratio_sum, first.0.pivots, first.0.reached_pivots,
        );
        rows.push(json!({
            "experiment": "e21", "p": p, "seed": seed, "n": inst.n, "targets": targets - 1,
            "trace_zero_rows": trace.zero_ids.len(), "trace_pivots": trace.pivot_cols.len(),
            "trace_dim": trace.dim, "agree": agree, "disagree": disagree, "fell_back": fell_back,
            "median_full": mf, "median_replay": mr, "ratio_median": mf / mr, "ratio_total": ratio_sum,
            "per_target": per_target,
        }));
        if let Some(path) = &json_path {
            fs::write(path, serde_json::to_string_pretty(&rows).unwrap()).expect("write json");
        }
    }
}
