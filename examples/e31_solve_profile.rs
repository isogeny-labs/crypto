//! E31 step 1 (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`, E31): where one
//! `k = 4` decomposition call on the `V₄` line spends its `F_p`
//! multiplications.
//!
//! Rebuilds an E25 curve (`generate_gls_klein_instance(p, seed, 16)`, arm A's
//! base and precompute, in E25's RNG order) and replays the first `--targets`
//! targets of arm A's stream exactly as `collect_and_solve` draws them with
//! `--threads` workers (E25 recorded `RAYON_NUM_THREADS=4`).  Each target goes
//! through `v4_system`, `solve_weighted_traced` at weighted degree 11 and
//! `lift_back_v4`, the E25 call; the solver's own per-phase counters
//! (`QuarticSolve`) are summed.  No solver code is changed by this example.
//!
//! ```text
//! cargo run --release --example e31_solve_profile -- --sizes 269,397 --seeds 1,2 --targets 400 --json out.json
//! ```

use std::time::Instant;

use crypto_lib::cryptanalysis::gaudry_quartic::QuarticSolve;
use crypto_lib::cryptanalysis::klein_v4::{
    chain_structure, lift_back_v4, solve_weighted_traced, v4_system, SymmetrisedV4, MACAULAY_BOUND,
};
use crypto_lib::cryptanalysis::quartic_folds::{generate_gls_klein_instance, klein_base_with};
use crypto_lib::cryptanalysis::residual_walk::mix64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;

#[derive(Default, Serialize, Clone)]
struct Phases {
    calls: u64,
    solved: u64,
    weil: u64,
    triangularise: u64,
    echelon: u64,
    normal_forms: u64,
    charpoly: u64,
    roots: u64,
    eigenvectors: u64,
    verify: u64,
    lift_back: u64,
    total_call: u64,
    build_ms: f64,
    echelon_ms: f64,
    nf_ms: f64,
    charpoly_ms: f64,
    eigen_ms: f64,
    solve_ms: f64,
    reruns: u64,
    rows_sum: u64,
    cols_sum: u64,
    pivots_sum: u64,
    zero_rows_sum: u64,
    reached_sum: u64,
    dim_sum: u64,
    rational_eigs_sum: u64,
    e_solutions_sum: u64,
    /// Structure (`chain_structure`), summed over calls.
    chains_sum: u64,
    chains_min: u64,
    chains_max: u64,
    reached_union_sum: u64,
    reached_sum_sum: u64,
    union_nnz_nonstd_sum: u64,
    union_nnz_std_sum: u64,
    left_looking_bound_sum: u64,
    structure_calls: u64,
}

#[derive(Serialize)]
struct Row {
    p: u64,
    seed: u64,
    targets: usize,
    threads: u64,
    phases: Phases,
    per_call_median: u64,
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let sizes = list(&args, "--sizes", "269,397");
    let seeds = list(&args, "--seeds", "1,2");
    let targets: usize = arg(&args, "--targets", 400);
    let threads: u64 = arg(&args, "--threads", 4);
    let json: String = arg(&args, "--json", String::new());
    let mut rows = Vec::new();
    for &p in &sizes {
        for &seed in &seeds {
            let ki = generate_gls_klein_instance(p, seed, 16).expect("instance");
            let inst = ki.inst;
            let mut rng = StdRng::seed_from_u64(seed ^ 0xE25A_6175);
            let _fb = klein_base_with(&ki, false, &mut rng).expect("base");
            let pcurve = inst.curve();
            let pre = SymmetrisedV4::precompute(&ki, &pcurve, &mut rng, 8).expect("precompute");
            let n = inst.n;
            let mut ph = Phases::default();
            let mut calls = Vec::new();
            let mut residuals = 0u64;
            let t_all = Instant::now();
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
                    if calls.len() >= targets {
                        break 'outer;
                    }
                    residuals += 1;
                    let c = inst.curve();
                    let r = c.add(&c.mul(&inst.g, a), &c.mul(&inst.q, b));
                    let mut trng = StdRng::seed_from_u64(tseed);
                    let c = ki.inst.curve();
                    let mut st = QuarticSolve::default();
                    let before = c.f.muls();
                    let Some(z_r) = ki.z_of_x(&c.f, &r.x) else {
                        continue;
                    };
                    let mut fp = 0u64;
                    let comps = v4_system(&pre, &c.f, &z_r, &mut fp);
                    st.weil_muls = c.f.muls() - before + fp;
                    let t0 = Instant::now();
                    let (sols, deg) = solve_weighted_traced(
                        &comps,
                        MACAULAY_BOUND,
                        p,
                        &mut trng,
                        &mut st,
                        true,
                        false,
                        None,
                        None,
                    );
                    ph.solve_ms += t0.elapsed().as_secs_f64() * 1e3;
                    if let Some(cs) = chain_structure(&comps, MACAULAY_BOUND, p) {
                        if !cs.unreachable {
                            let m = cs.chains as u64;
                            ph.structure_calls += 1;
                            ph.chains_sum += m;
                            ph.chains_min = if ph.structure_calls == 1 {
                                m
                            } else {
                                ph.chains_min.min(m)
                            };
                            ph.chains_max = ph.chains_max.max(m);
                            ph.reached_union_sum += cs.reached_union as u64;
                            ph.reached_sum_sum += cs.reached_sum as u64;
                            ph.union_nnz_nonstd_sum += cs.union_nnz_nonstd;
                            ph.union_nnz_std_sum += cs.union_nnz_std;
                            ph.left_looking_bound_sum += cs.left_looking_bound;
                        }
                    }
                    let solved = sols.is_some();
                    let sols = sols.unwrap_or_default();
                    let lb = lift_back_v4(&ki, &c, &sols, &r, &mut trng);
                    ph.calls += 1;
                    ph.solved += u64::from(solved);
                    ph.reruns += u64::from(deg > MACAULAY_BOUND);
                    ph.weil += st.weil_muls;
                    ph.triangularise += st.triangularise_muls;
                    ph.echelon += st.echelon_muls;
                    ph.normal_forms += st.nf_muls;
                    ph.charpoly += st.charpoly_muls;
                    ph.roots += st.roots_muls;
                    ph.eigenvectors += st.eigenvector_muls;
                    ph.verify += st.verify_muls;
                    ph.lift_back += lb.muls;
                    ph.total_call += st.total_muls;
                    ph.build_ms += st.build_ms;
                    ph.echelon_ms += st.echelon_ms;
                    ph.nf_ms += st.nf_ms;
                    ph.charpoly_ms += st.charpoly_ms;
                    ph.eigen_ms += st.eigen_ms;
                    ph.rows_sum += st.rows as u64;
                    ph.cols_sum += st.cols as u64;
                    ph.pivots_sum += st.pivots as u64;
                    ph.zero_rows_sum += st.zero_rows as u64;
                    ph.reached_sum += st.reached_pivots as u64;
                    ph.dim_sum += st.dim as u64;
                    ph.rational_eigs_sum += st.rational_eigenvalues as u64;
                    ph.e_solutions_sum += st.e_solutions as u64;
                    calls.push(st.total_muls);
                }
            }
            calls.sort_unstable();
            let med = calls[calls.len() / 2];
            let c = ph.calls as f64;
            let pct = |x: u64| 100.0 * x as f64 / ph.total_call as f64;
            eprintln!(
                "p={p} seed={seed}: {} calls ({} solved, {} reruns), call median {med}, mean {:.0}; {:.1} s",
                ph.calls,
                ph.solved,
                ph.reruns,
                ph.total_call as f64 / c,
                t_all.elapsed().as_secs_f64()
            );
            for (name, v) in [
                ("weil", ph.weil),
                ("triangularise", ph.triangularise),
                ("echelon", ph.echelon),
                ("normal_forms", ph.normal_forms),
                ("charpoly", ph.charpoly),
                ("roots", ph.roots),
                ("eigenvectors", ph.eigenvectors),
                ("verify", ph.verify),
            ] {
                eprintln!(
                    "  {name:>14}: {:>12.0} per call  {:5.1} %",
                    v as f64 / c,
                    pct(v)
                );
            }
            eprintln!(
                "  {:>14}: {:>12.0} per call (outside the call count)",
                "lift_back",
                ph.lift_back as f64 / c
            );
            eprintln!(
                "  rows {:.1} cols {:.1} pivots {:.1} zero rows {:.1} reached {:.1} dim {:.2} rational eigs {:.2} solutions {:.2}",
                ph.rows_sum as f64 / c,
                ph.cols_sum as f64 / c,
                ph.pivots_sum as f64 / c,
                ph.zero_rows_sum as f64 / c,
                ph.reached_sum as f64 / c,
                ph.dim_sum as f64 / c,
                ph.rational_eigs_sum as f64 / c,
                ph.e_solutions_sum as f64 / c
            );
            eprintln!(
                "  ms per call: build {:.3} echelon {:.3} nf {:.3} charpoly {:.3} eigen {:.3} solve {:.3}",
                ph.build_ms / c,
                ph.echelon_ms / c,
                ph.nf_ms / c,
                ph.charpoly_ms / c,
                ph.eigen_ms / c,
                ph.solve_ms / c
            );
            let sc = ph.structure_calls.max(1) as f64;
            eprintln!(
                "  structure: chains {:.2} (min {} max {}), reached union {:.1}, reached sum {:.1}, union nnz nonstd {:.0} std {:.0}, left-looking bound {:.0}",
                ph.chains_sum as f64 / sc,
                ph.chains_min,
                ph.chains_max,
                ph.reached_union_sum as f64 / sc,
                ph.reached_sum_sum as f64 / sc,
                ph.union_nnz_nonstd_sum as f64 / sc,
                ph.union_nnz_std_sum as f64 / sc,
                ph.left_looking_bound_sum as f64 / sc
            );
            rows.push(Row {
                p,
                seed,
                targets,
                threads,
                phases: ph,
                per_call_median: med,
            });
        }
    }
    if !json.is_empty() {
        std::fs::write(&json, serde_json::to_string_pretty(&rows).unwrap()).unwrap();
    }
}
