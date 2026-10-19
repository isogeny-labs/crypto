//! **Plain `S₅` `C₄` with the `D₄` oracle's solver options, paired.**
//!
//! §11.17 of `research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md`
//! measured `C₄ = 1.213·10¹²` for the plain `S₅` solve.  #1792 then cut the
//! `D₄`-symmetric oracle's `C₄` about 370× with four options, 26× of it from
//! sparsest pivots alone.  Three of those options (`sparsest`, `chains`,
//! shape replay) do not depend on the `D₄` form; this runner applies them to
//! the plain system on §11.17's instances.
//!
//! ```text
//! cargo run --release --example gaudry_quartic_c4_options -- --p 269 --seed 1 --residuals 4 \
//!     --arms committed,sparsest,sparsest+chains --json out.json
//! cargo run --release --example gaudry_quartic_c4_options -- --generic 4,5,6 \
//!     --arms committed,sparsest,sparsest+chains --json generic.json
//! ```
//!
//! Instances: the curve, base and residuals are §11.17's (`p = 269`, seed 1),
//! because the residual stream advances with the `committed` arm's solve, as
//! it did there.  Every arm of a residual starts from a clone of the same
//! generator state; the arm order rotates by one per residual.  Every solve
//! is checked against the meet-in-the-middle oracle.  `--constructed n`
//! makes the first `n` residuals planted four-point sums (default 1, as
//! §11.17), so replayed solves are checked on nonempty decompositions.  An arm ending in
//! `+replay` records its first solve's shape and replays it afterwards.

use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

use crypto_lib::cryptanalysis::gaudry_quartic::{
    base_quadruples, factor_base, mitm_decompositions, solve_s5_subspace_with, solve_system4_with,
    Curve4, PlainOptions, Pt4, QuarticSolve, ShapeTrace, SymmetrisedS5,
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;

#[derive(Clone, Debug)]
struct Arm {
    name: String,
    opts: PlainOptions,
    replay: bool,
}

fn parse_arm(s: &str) -> Arm {
    let mut opts = PlainOptions::default();
    let mut replay = false;
    for part in s.split('+') {
        match part {
            "committed" => {}
            "sparsest" => opts.sparsest = true,
            "chains" => opts.chains = true,
            "replay" => replay = true,
            other => panic!("unknown arm option {other}"),
        }
    }
    Arm {
        name: s.to_string(),
        opts,
        replay,
    }
}

#[derive(Serialize)]
struct Row {
    arm: String,
    options: PlainOptions,
    replay_arm: bool,
    /// Position of this arm in the residual's run order (0 = first).
    order: usize,
    solve: QuarticSolve,
    solver_quadruples: Vec<[u64; 4]>,
    agree: bool,
    wall_s: f64,
}

#[derive(Serialize)]
struct ResidualRows {
    index: usize,
    constructed: bool,
    planted: Option<[u64; 4]>,
    oracle_quadruples: Vec<[u64; 4]>,
    arms: Vec<Row>,
}

#[derive(Serialize)]
struct Report {
    p: u64,
    seed: u64,
    base: usize,
    fp_muls_per_add: f64,
    precompute_muls: u64,
    arms: Vec<String>,
    residuals: Vec<ResidualRows>,
}

#[derive(Serialize)]
struct GenericRow {
    equation_degree: u8,
    arm: String,
    order: usize,
    solve: QuarticSolve,
    planted_found: bool,
    wall_s: f64,
}

fn generic_system(d: u8, seed: u64, p: u64) -> (Vec<HashMap<[u8; 4], u64>>, [u64; 4], StdRng) {
    // The same planted system as `gaudry_quartic_c4 --generic`.
    let mut rng = StdRng::seed_from_u64(seed);
    let root: [u64; 4] = std::array::from_fn(|_| rng.gen_range(0..p));
    let mut monos = Vec::new();
    for a in 0..=d {
        for b in 0..=(d - a) {
            for c in 0..=(d - a - b) {
                for e in 0..=(d - a - b - c) {
                    monos.push([a, b, c, e]);
                }
            }
        }
    }
    let eval = |c: &HashMap<[u8; 4], u64>| -> u64 {
        c.iter().fold(0u64, |acc, (m, &k)| {
            let mut t = k;
            for i in 0..4 {
                for _ in 0..m[i] {
                    t = t * root[i] % p;
                }
            }
            (acc + t) % p
        })
    };
    let comps = (0..4)
        .map(|_| {
            let mut c: HashMap<[u8; 4], u64> =
                monos.iter().map(|m| (*m, rng.gen_range(0..p))).collect();
            let v = eval(&c);
            let e = c.entry([0, 0, 0, 0]).or_insert(0);
            *e = (*e + p - v) % p;
            c
        })
        .collect();
    (comps, root, rng)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut p = 269u64;
    let mut seed = 1u64;
    let mut residuals = 4usize;
    let mut constructed_n = 1usize;
    let mut json: Option<String> = None;
    let mut generic_degrees: Option<Vec<u8>> = None;
    let mut arms: Vec<Arm> = vec![parse_arm("committed"), parse_arm("sparsest")];
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--p" => {
                i += 1;
                p = args[i].parse().expect("--p");
            }
            "--seed" => {
                i += 1;
                seed = args[i].parse().expect("--seed");
            }
            "--residuals" => {
                i += 1;
                residuals = args[i].parse().expect("--residuals");
            }
            "--constructed" => {
                i += 1;
                constructed_n = args[i].parse().expect("--constructed");
            }
            "--json" => {
                i += 1;
                json = Some(args[i].clone());
            }
            "--arms" => {
                i += 1;
                arms = args[i].split(',').map(parse_arm).collect();
            }
            "--generic" => {
                i += 1;
                generic_degrees = Some(
                    args[i]
                        .split(',')
                        .map(|v| v.parse().expect("--generic"))
                        .collect(),
                );
            }
            other => panic!("unknown argument {other}"),
        }
        i += 1;
    }
    // The residual stream advances with the committed arm's solve, as in
    // §11.17; without that arm, with the first arm's (other instances).
    let committed = arms.iter().position(|a| a.name == "committed").unwrap_or(0);

    if let Some(ds) = generic_degrees {
        let mut out = Vec::new();
        for (di, &d) in ds.iter().enumerate() {
            for k in 0..arms.len() {
                let arm = &arms[(di + k) % arms.len()];
                let (comps, root, mut rng) = generic_system(d, seed, 269);
                let mut st = QuarticSolve::default();
                let t = Instant::now();
                let sols = solve_system4_with(
                    &comps,
                    arm.opts,
                    d,
                    4 * (d - 1) + 1,
                    269,
                    &mut rng,
                    &mut st,
                    false,
                    None,
                    None,
                );
                let wall_s = t.elapsed().as_secs_f64();
                let planted_found = sols.as_ref().is_some_and(|s| s.contains(&root));
                println!(
                    "generic d={d} {:<24} C = {:.4e} (echelon {:.3e}, nf {:.3e}, charpoly {:.3e}, eigen {:.3e}) dim {} planted {planted_found} | {wall_s:.1} s",
                    arm.name, st.total_muls as f64, st.echelon_muls as f64, st.nf_muls as f64,
                    st.charpoly_muls as f64, st.eigenvector_muls as f64, st.dim
                );
                out.push(GenericRow {
                    equation_degree: d,
                    arm: arm.name.clone(),
                    order: k,
                    solve: st,
                    planted_found,
                    wall_s,
                });
                if let Some(path) = &json {
                    fs::write(path, serde_json::to_string_pretty(&out).unwrap()).expect("write");
                }
            }
        }
        return;
    }

    let mut rng = StdRng::seed_from_u64(seed);
    let curve = Curve4::random(p, &mut rng);
    let f = &curve.f;
    let base = factor_base(&curve, &mut rng);
    let fp_per_add = {
        let q = base[1];
        let mut acc = base[0];
        f.reset_muls();
        for _ in 0..64 {
            acc = curve.add(&acc, &q);
        }
        f.muls() as f64 / 64.0
    };
    f.reset_muls();
    let pre = SymmetrisedS5::precompute(&curve, &mut rng);
    println!(
        "p={p} seed={seed}: base {}, {fp_per_add:.1} F_p muls per addition, precompute {:.3e} muls; arms {:?}",
        base.len(),
        pre.precompute_muls as f64,
        arms.iter().map(|a| a.name.as_str()).collect::<Vec<_>>()
    );
    let mut traces: Vec<Option<ShapeTrace>> = vec![None; arms.len()];
    let mut report = Report {
        p,
        seed,
        base: base.len(),
        fp_muls_per_add: fp_per_add,
        precompute_muls: pre.precompute_muls,
        arms: arms.iter().map(|a| a.name.clone()).collect(),
        residuals: Vec::new(),
    };
    for idx in 0..residuals {
        // Residual generation exactly as `gaudry_quartic_c4`.
        let constructed = idx < constructed_n;
        let (r, planted) = if constructed {
            let mut ids: Vec<usize> = Vec::new();
            while ids.len() < 4 {
                let k = rng.gen_range(0..base.len());
                if !ids.contains(&k) {
                    ids.push(k);
                }
            }
            let r = ids
                .iter()
                .fold(Pt4::INF, |acc, &k| curve.add(&acc, &base[k]));
            let mut xs: [u64; 4] = std::array::from_fn(|k| base[ids[k]].x.0[0]);
            xs.sort_unstable();
            (r, Some(xs))
        } else {
            let r = loop {
                let x = f.random(&mut rng);
                if let Some(pt) = curve.lift_x(&x, &mut rng) {
                    break pt;
                }
            };
            (r, None)
        };
        let oracle_q: Vec<[u64; 4]> = mitm_decompositions(&curve, &base, &r).into_iter().collect();
        let start = rng.clone();
        let mut after_committed: Option<StdRng> = None;
        let mut rows: Vec<Row> = Vec::new();
        for k in 0..arms.len() {
            let ai = (idx + k) % arms.len();
            let arm = &arms[ai];
            let mut arm_rng = start.clone();
            let mut st = QuarticSolve::default();
            f.reset_muls();
            let t = Instant::now();
            let sols = if arm.replay {
                match &traces[ai] {
                    Some(tr) => solve_s5_subspace_with(
                        &curve,
                        &pre,
                        &r.x,
                        arm.opts,
                        Some(tr),
                        None,
                        &mut arm_rng,
                        &mut st,
                        false,
                    ),
                    None => {
                        let mut tr = ShapeTrace::default();
                        let s = solve_s5_subspace_with(
                            &curve,
                            &pre,
                            &r.x,
                            arm.opts,
                            None,
                            Some(&mut tr),
                            &mut arm_rng,
                            &mut st,
                            false,
                        );
                        if s.is_some() {
                            traces[ai] = Some(tr);
                        }
                        s
                    }
                }
            } else {
                solve_s5_subspace_with(
                    &curve,
                    &pre,
                    &r.x,
                    arm.opts,
                    None,
                    None,
                    &mut arm_rng,
                    &mut st,
                    false,
                )
            };
            let wall_s = t.elapsed().as_secs_f64();
            if ai == committed {
                after_committed = Some(arm_rng);
            }
            let solver_q: Vec<[u64; 4]> = sols
                .as_ref()
                .map(|s| base_quadruples(s, &base).into_iter().collect())
                .unwrap_or_default();
            let agree = sols.is_some() && solver_q == oracle_q;
            println!(
                "residual {idx} [{k}] {:<24} C4 = {:.4e} (echelon {:.3e}, nf {:.3e}, charpoly {:.3e}, eigen {:.3e}) dim {} replay {:?} | solver {:?} oracle {:?} agree {agree} | {wall_s:.0} s",
                arm.name, st.total_muls as f64, st.echelon_muls as f64, st.nf_muls as f64,
                st.charpoly_muls as f64, st.eigenvector_muls as f64, st.dim, st.replay,
                solver_q, oracle_q
            );
            rows.push(Row {
                arm: arm.name.clone(),
                options: arm.opts,
                replay_arm: arm.replay,
                order: k,
                solve: st,
                solver_quadruples: solver_q,
                agree,
                wall_s,
            });
            report.residuals.retain(|r| r.index != idx);
            report.residuals.push(ResidualRows {
                index: idx,
                constructed,
                planted,
                oracle_quadruples: oracle_q.clone(),
                arms: rows
                    .iter()
                    .map(|r| Row {
                        arm: r.arm.clone(),
                        options: r.options,
                        replay_arm: r.replay_arm,
                        order: r.order,
                        solve: r.solve.clone(),
                        solver_quadruples: r.solver_quadruples.clone(),
                        agree: r.agree,
                        wall_s: r.wall_s,
                    })
                    .collect(),
            });
            if let Some(path) = &json {
                fs::write(path, serde_json::to_string_pretty(&report).unwrap()).expect("write");
            }
        }
        rng = after_committed.expect("committed arm ran");
    }
}
