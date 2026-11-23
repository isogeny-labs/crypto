//! E31 Stage A (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.22, G2.85–G2.92):
//! the existing `V₄` solve against the chain solve (A) and the chain solve
//! with sparsest pivots (B), on the same calls.
//!
//! For each E25 curve (`generate_gls_klein_instance(p, seed, 16)`, arm A's
//! base and precompute in E25's RNG order) the first `--targets` targets of
//! arm A's stream (drawn as `collect_and_solve` draws them with `--threads`
//! workers) are decomposed by all three solvers in turn, each from the same
//! per-target RNG seed: `v4_system`, the solve, `lift_back_v4`.  Per call:
//! every solver's phase counters and wall time, and agreement of solutions,
//! degree, outcome and decomposition set with the existing solver, plus
//! `chain_quotient_agrees` (`M_{P₂}` and `charpoly`).
//!
//! ```text
//! RAYON_NUM_THREADS=1 cargo run --release --example e31_stage_a -- --sizes 101,149,197,269,397 --seeds 1,2 --targets 1000 --json experiments/31_solve_cost/stage_a.json
//! ```

use std::time::Instant;

use crypto_lib::cryptanalysis::gaudry_quartic::QuarticSolve;
use crypto_lib::cryptanalysis::klein_v4::{
    chain_quotient_agrees, lift_back_v4, solve_v4, v4_system, SymmetrisedV4, V4Solver,
    MACAULAY_BOUND,
};
use crypto_lib::cryptanalysis::quartic_folds::{generate_gls_klein_instance, klein_base_with};
use crypto_lib::cryptanalysis::residual_walk::mix64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;

const SOLVERS: [V4Solver; 3] = [V4Solver::Macaulay, V4Solver::Chain, V4Solver::ChainSparsest];

#[derive(Default, Serialize, Clone)]
struct Arm {
    solver: &'static str,
    /// Per call, in stream order: `call` = the counted call (Weil
    /// restriction + solve, E25's `C`), and the phases.
    call: Vec<u64>,
    echelon: Vec<u64>,
    normal_forms: Vec<u64>,
    charpoly: Vec<u64>,
    roots: Vec<u64>,
    eigenvectors: Vec<u64>,
    other: Vec<u64>,
    lift_back: Vec<u64>,
    wall_us: Vec<f64>,
    fallbacks: u64,
    solved: u64,
}

#[derive(Serialize)]
struct Summary {
    solver: &'static str,
    call_median: u64,
    call_mean: f64,
    echelon_mean: f64,
    normal_forms_mean: f64,
    charpoly_mean: f64,
    roots_mean: f64,
    eigenvectors_mean: f64,
    other_mean: f64,
    wall_us_median: f64,
    wall_us_mean: f64,
    fallbacks: u64,
    solved: u64,
}

#[derive(Serialize)]
struct Curve {
    p: u64,
    seed: u64,
    n: u64,
    calls: usize,
    /// Calls where the solver agreed with the existing one on solutions,
    /// degree, outcome and decomposition set: `[A, B]`.
    agree: [usize; 2],
    /// Calls where `chain_quotient_agrees` returned `(true, true)`, and
    /// calls where it applied (the existing solver did not refuse): `[A, B]`.
    quotient_agree: [usize; 2],
    quotient_checked: [usize; 2],
    summaries: Vec<Summary>,
    arms: Vec<Arm>,
}

fn arg<T: std::str::FromStr>(args: &[String], k: &str, d: T) -> T {
    args.iter()
        .position(|a| a == k)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(d)
}

fn list(args: &[String], k: &str, d: &str) -> Vec<u64> {
    let s: String = arg(args, k, d.to_string());
    s.split(',').map(|x| x.trim().parse().unwrap()).collect()
}

fn median_u(v: &[u64]) -> u64 {
    let mut s = v.to_vec();
    s.sort_unstable();
    s[s.len() / 2]
}

fn median_f(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    s[s.len() / 2]
}

fn mean_u(v: &[u64]) -> f64 {
    v.iter().sum::<u64>() as f64 / v.len().max(1) as f64
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let sizes = list(&args, "--sizes", "101,149,197,269,397");
    let seeds = list(&args, "--seeds", "1,2");
    let targets: usize = arg(&args, "--targets", 1000);
    let threads: u64 = arg(&args, "--threads", 4);
    let json: String = arg(&args, "--json", String::new());
    let mut out = Vec::new();
    for &p in &sizes {
        for &seed in &seeds {
            let t_curve = Instant::now();
            let ki = generate_gls_klein_instance(p, seed, 16).expect("instance");
            let inst = ki.inst;
            let mut rng = StdRng::seed_from_u64(seed ^ 0xE25A_6175);
            let _fb = klein_base_with(&ki, false, &mut rng).expect("base");
            let pcurve = inst.curve();
            let pre = SymmetrisedV4::precompute(&ki, &pcurve, &mut rng, 8).expect("precompute");
            let n = inst.n;
            let mut arms: Vec<Arm> = SOLVERS
                .iter()
                .map(|s| Arm {
                    solver: s.name(),
                    ..Default::default()
                })
                .collect();
            let (mut agree, mut qa, mut qc) = ([0usize; 2], [0usize; 2], [0usize; 2]);
            let mut calls = 0usize;
            let mut residuals = 0u64;
            'outer: loop {
                let batch: Vec<(u64, u64, u64)> = (0..threads * 2)
                    .map(|k| {
                        (
                            rng.gen_range(0..n),
                            rng.gen_range(1..n),
                            mix64(seed ^ (residuals + k).wrapping_mul(0x9E37_79B9_7F4A_7C15)),
                        )
                    })
                    .collect();
                for (a, b, tseed) in batch {
                    if calls >= targets {
                        break 'outer;
                    }
                    residuals += 1;
                    let c = inst.curve();
                    let r = c.add(&c.mul(&inst.g, a), &c.mul(&inst.q, b));
                    let c = inst.curve();
                    let before = c.f.muls();
                    let Some(z_r) = ki.z_of_x(&c.f, &r.x) else {
                        continue;
                    };
                    let mut fp = 0u64;
                    let comps = v4_system(&pre, &c.f, &z_r, &mut fp);
                    let weil = c.f.muls() - before + fp;
                    calls += 1;
                    let mut results = Vec::new();
                    for (k, &solver) in SOLVERS.iter().enumerate() {
                        let mut trng = StdRng::seed_from_u64(tseed);
                        let mut st = QuarticSolve {
                            weil_muls: weil,
                            ..Default::default()
                        };
                        let t0 = Instant::now();
                        let (sols, deg, fell) = solve_v4(solver, &comps, p, &mut trng, &mut st);
                        let wall = t0.elapsed().as_secs_f64() * 1e6;
                        let lb =
                            lift_back_v4(&ki, &c, sols.as_deref().unwrap_or(&[]), &r, &mut trng);
                        let arm = &mut arms[k];
                        arm.call.push(st.total_muls);
                        arm.echelon.push(st.echelon_muls);
                        arm.normal_forms.push(st.nf_muls);
                        arm.charpoly.push(st.charpoly_muls);
                        arm.roots.push(st.roots_muls);
                        arm.eigenvectors.push(st.eigenvector_muls);
                        arm.other.push(
                            st.weil_muls + st.triangularise_muls + st.verify_muls + st.split_muls,
                        );
                        arm.lift_back.push(lb.muls);
                        arm.wall_us.push(wall);
                        arm.fallbacks += u64::from(fell);
                        arm.solved += u64::from(sols.is_some());
                        results.push((sols, deg, st.outcome.clone(), lb.decompositions));
                    }
                    for k in 0..2 {
                        if results[k + 1] == results[0] {
                            agree[k] += 1;
                        }
                        if let Some((m, cp)) =
                            chain_quotient_agrees(&comps, MACAULAY_BOUND, p, k == 1)
                        {
                            qc[k] += 1;
                            if m && cp {
                                qa[k] += 1;
                            }
                        }
                    }
                }
            }
            let summaries: Vec<Summary> = arms
                .iter()
                .map(|a| Summary {
                    solver: a.solver,
                    call_median: median_u(&a.call),
                    call_mean: mean_u(&a.call),
                    echelon_mean: mean_u(&a.echelon),
                    normal_forms_mean: mean_u(&a.normal_forms),
                    charpoly_mean: mean_u(&a.charpoly),
                    roots_mean: mean_u(&a.roots),
                    eigenvectors_mean: mean_u(&a.eigenvectors),
                    other_mean: mean_u(&a.other),
                    wall_us_median: median_f(&a.wall_us),
                    wall_us_mean: a.wall_us.iter().sum::<f64>() / a.wall_us.len() as f64,
                    fallbacks: a.fallbacks,
                    solved: a.solved,
                })
                .collect();
            eprintln!(
                "p={p} seed={seed}: {calls} calls, agree A {} B {}, quotient A {}/{} B {}/{}, {:.0} s",
                agree[0],
                agree[1],
                qa[0],
                qc[0],
                qa[1],
                qc[1],
                t_curve.elapsed().as_secs_f64()
            );
            for s in &summaries {
                eprintln!(
                    "  {:>14}: C median {:>9} mean {:>11.0} | ech {:>9.0} nf {:>9.0} cp {:>9.0} roots {:>7.0} eig {:>7.0} other {:>6.0} | wall median {:>8.0} us mean {:>8.0} | fallbacks {} solved {}",
                    s.solver,
                    s.call_median,
                    s.call_mean,
                    s.echelon_mean,
                    s.normal_forms_mean,
                    s.charpoly_mean,
                    s.roots_mean,
                    s.eigenvectors_mean,
                    s.other_mean,
                    s.wall_us_median,
                    s.wall_us_mean,
                    s.fallbacks,
                    s.solved
                );
            }
            out.push(Curve {
                p,
                seed,
                n,
                calls,
                agree,
                quotient_agree: qa,
                quotient_checked: qc,
                summaries,
                arms,
            });
            if !json.is_empty() {
                std::fs::write(&json, serde_json::to_string(&out).unwrap()).unwrap();
            }
        }
    }
}
