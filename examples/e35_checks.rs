//! E35 controls (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.40, G2.146):
//! corrupted traces must be detected by the replay's counted check, and
//! the check must be load-bearing.
//!
//! On the first `--targets` calls of arm A's stream on each curve (E33's
//! Stage A stream), with the curve's learned trace:
//!
//! - (i)  the first contested column of the call (after at least one
//!   pivot) where the clean replay used the trace, pointed at a row
//!   already used as an earlier pivot;
//! - (ii) the same column pointed at a row whose lead is a later column;
//! - (iii) a trace of another shape (`rows + 1`).
//!
//! Each corruption, replayed with the check, must be detected on every
//! call (the corrupted column misses, or the shape fallback) with the
//! existing solver's outputs; replayed *without* the check (the
//! positive control) it must give a wrong echelon (B's pivot count or
//! quotient dimension changes, or the outputs differ), a chain fallback, or
//! an abort, on at least one call.
//!
//! ```text
//! RAYON_NUM_THREADS=1 e35_checks --curves 101:2,397:1 --targets 32 --json experiments/39_chain_trace/checks.json
//! ```

use std::panic::{catch_unwind, AssertUnwindSafe};

use crypto_lib::cryptanalysis::chain_trace::{
    last_replay, learn_gls, learn_one, reset_last_replay, PivotTrace,
};
use crypto_lib::cryptanalysis::gaudry_quartic::QuarticSolve;
use crypto_lib::cryptanalysis::klein_v4::{
    lift_back_v4, solve_weighted_chain_with, solve_weighted_traced, v4_system, Pivot,
    SymmetrisedV4, MACAULAY_BOUND,
};
use crypto_lib::cryptanalysis::quartic_folds::{generate_gls_klein_instance, klein_base_with};
use crypto_lib::cryptanalysis::residual_walk::mix64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;

#[derive(Serialize, Default, Clone)]
struct Control {
    kind: &'static str,
    calls: usize,
    /// Checked replay: corruption detected (extra miss or shape fallback).
    detected: usize,
    /// Checked replay: outputs equal the existing solver's.
    outputs_equal: usize,
    /// Unchecked replay: aborted / chain fallback / wrong echelon or outputs.
    unchecked_abort: usize,
    unchecked_fallback: usize,
    unchecked_wrong: usize,
    unchecked_fine: usize,
}

#[derive(Serialize)]
struct CurveOut {
    p: u64,
    seed: u64,
    clean_outputs_equal: usize,
    /// Calls where the clean replay used the trace in no contested column
    /// after the first pivot (nothing to corrupt; not counted in controls).
    calls_without_traced_column: usize,
    controls: Vec<Control>,
}

fn arg<T: std::str::FromStr>(args: &[String], k: &str, d: T) -> T {
    args.iter()
        .position(|a| a == k)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(d)
}

type Res = (Option<Vec<[u64; 4]>>, u16, String, Vec<[u64; 4]>);

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let curves: String = arg(&args, "--curves", "101:2,397:1".to_string());
    let targets: usize = arg(&args, "--targets", 32);
    let json: String = arg(&args, "--json", String::new());
    std::panic::set_hook(Box::new(|_| {}));
    let mut out = Vec::new();
    for cs in curves.split(',') {
        let (p, seed) = cs.split_once(':').unwrap();
        let (p, seed): (u64, u64) = (p.parse().unwrap(), seed.parse().unwrap());
        let ki = generate_gls_klein_instance(p, seed, 16).expect("instance");
        let inst = ki.inst;
        let mut rng = StdRng::seed_from_u64(seed ^ 0xE25A_6175);
        let _fb = klein_base_with(&ki, false, &mut rng).expect("base");
        let pcurve = inst.curve();
        let pre = SymmetrisedV4::precompute(&ki, &pcurve, &mut rng, 8).expect("precompute");
        let (trace, _) = learn_gls(&ki, &pre, seed).expect("learning");
        let clean: &'static PivotTrace = Box::leak(Box::new(trace));
        let mut ctl: Vec<Control> = ["used-pivot row", "later-lead row", "other shape"]
            .iter()
            .map(|k| Control {
                kind: k,
                ..Default::default()
            })
            .collect();
        let mut clean_eq = 0usize;
        let mut no_hit = 0usize;
        let (n, threads) = (inst.n, 4u64);
        let (mut calls, mut residuals) = (0usize, 0u64);
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
                let Some(z_r) = ki.z_of_x(&c.f, &r.x) else {
                    continue;
                };
                let mut fp = 0u64;
                let comps = v4_system(&pre, &c.f, &z_r, &mut fp);
                let Ok(bl) = learn_one(&comps, p) else {
                    continue;
                };
                calls += 1;
                let run = |pv: Option<Pivot>| -> (Res, QuarticSolve, bool) {
                    let mut trng = StdRng::seed_from_u64(tseed);
                    let mut st = QuarticSolve::default();
                    reset_last_replay();
                    let (sols, deg, fell) = match pv {
                        None => {
                            let (s, d) = solve_weighted_traced(
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
                            (s, d, false)
                        }
                        Some(pv) => {
                            let (s, d, info) = solve_weighted_chain_with(
                                &comps,
                                MACAULAY_BOUND,
                                p,
                                &mut trng,
                                &mut st,
                                pv,
                            );
                            (s, d, info.fell_back)
                        }
                    };
                    let lb = lift_back_v4(&ki, &c, sols.as_deref().unwrap_or(&[]), &r, &mut trng);
                    ((sols, deg, st.outcome.clone(), lb.decompositions), st, fell)
                };
                let (want, _, _) = run(None);
                let (got, st_clean, fell_clean) = run(Some(Pivot::Trace(clean, false)));
                let clean_w = last_replay();
                clean_eq += usize::from(got == want);
                // The column to corrupt: the first contested column, after at
                // least one pivot, where the clean replay used the trace (so
                // the corrupted entry is tested on the same candidates); an
                // earlier pivot row, and a row whose lead is later (B's pivot
                // of its last pivot column).
                let first_piv = bl.choice.iter().position(|c| c.0 != u32::MAX);
                let Some(col) = clean_w
                    .hit_cols
                    .iter()
                    .map(|&c| c as usize)
                    .find(|&c| first_piv.is_some_and(|f| f < c))
                else {
                    no_hit += 1;
                    continue;
                };
                let used = bl.choice[first_piv.unwrap()].0;
                let later = bl.choice.iter().rev().find(|c| c.0 != u32::MAX).unwrap().0;
                for (k, ctrl) in ctl.iter_mut().enumerate() {
                    let mut t = clean.clone();
                    match k {
                        0 => t.choice[col].0 = used,
                        1 => t.choice[col].0 = later,
                        _ => t.rows += 1,
                    }
                    let t: &'static PivotTrace = Box::leak(Box::new(t));
                    ctrl.calls += 1;
                    let (g, _, _) = run(Some(Pivot::Trace(t, false)));
                    let w = last_replay();
                    if (k < 2 && w.miss_cols.contains(&(col as u32))) || (k == 2 && w.shape_fallback) {
                        ctrl.detected += 1;
                    }
                    ctrl.outputs_equal += usize::from(g == want);
                    let u = catch_unwind(AssertUnwindSafe(|| run(Some(Pivot::Trace(t, true)))));
                    match u {
                        Err(_) => ctrl.unchecked_abort += 1,
                        Ok((g, st, fell)) => {
                            if fell && !fell_clean {
                                ctrl.unchecked_fallback += 1;
                            } else if g != want || st.pivots != st_clean.pivots || st.dim != st_clean.dim {
                                ctrl.unchecked_wrong += 1;
                            } else {
                                ctrl.unchecked_fine += 1;
                            }
                        }
                    }
                }
            }
        }
        eprintln!("p={p} seed={seed}: {calls} calls, clean replay = existing on {clean_eq}, calls without a traced column (skipped) {no_hit}");
        for c in &ctl {
            eprintln!(
                "  {:>15}: detected {}/{}, outputs equal {}/{}; unchecked: abort {}, fallback {}, wrong {}, fine {}",
                c.kind, c.detected, c.calls, c.outputs_equal, c.calls, c.unchecked_abort,
                c.unchecked_fallback, c.unchecked_wrong, c.unchecked_fine
            );
        }
        out.push(CurveOut {
            p,
            seed,
            clean_outputs_equal: clean_eq,
            calls_without_traced_column: no_hit,
            controls: ctl,
        });
    }
    if !json.is_empty() {
        std::fs::write(&json, serde_json::to_string_pretty(&out).unwrap()).unwrap();
    }
}
