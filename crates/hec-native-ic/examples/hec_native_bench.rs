//! Slice S1 of `research/hec_ic_speedup_20261113/PLAN.md`: full-base
//! factor-base-walk index calculus on the thread's curves, native engine.
//!
//! ```sh
//! cargo run --release --example hec_native_bench            # round-eight rows
//! HEC_ROWS=2:16411,3:1021,4:127 HEC_SEEDS=3 cargo run --release --example hec_native_bench
//! ```
//!
//! One human-readable line and one `JSON {...}` line per (row, seed).  Wall
//! time covers the factor base, the walk, the oracle and the solve
//! (`total_ms`); instance selection (BSGS order) is reported separately and
//! is not part of the index-calculus cost, as in the BigUint example.

use std::time::Instant;

use hec_native_ic::ic::{solve_dlp, IcParams};
use hec_native_ic::instance::pick_instance_with_budget;
use hec_native_ic::solve::Solver;

fn main() {
    let rows: Vec<(usize, u64)> = match std::env::var("HEC_ROWS") {
        Ok(s) => s
            .split(',')
            .filter_map(|t| {
                let (g, p) = t.split_once(':')?;
                Some((g.trim().parse().ok()?, p.trim().parse().ok()?))
            })
            .collect(),
        Err(_) => vec![
            (2, 101),
            (2, 251),
            (2, 503),
            (2, 1009),
            (2, 2003),
            (2, 4001),
            (3, 61),
            (3, 211),
            (3, 401),
            (4, 31),
            (4, 61),
        ],
    };
    let bsgs_max: usize = std::env::var("HEC_BSGS_MAX")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(200_000);
    let seeds: u64 = std::env::var("HEC_SEEDS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(5);
    // S2 (PLAN §7): `reference`, `fast` (default) or `both`.  `both` runs the
    // two solvers on every (row, seed) in the same process, alternating the
    // order by seed parity, and asserts the exactness gate E3.
    let solver_mode = std::env::var("HEC_SOLVER").unwrap_or_else(|_| "fast".into());
    let solver_of = |s: &str| match s {
        "reference" => Solver::Reference,
        "fast" => Solver::Fast,
        other => panic!("HEC_SOLVER: unknown solver {other:?}"),
    };

    println!(
        "{:>2} {:>6} {:>12} {:>6} {:>4} {:>9} {:>9} {:>6} {:>9} {:>6} {:>6} {:>6}  ok",
        "g",
        "p",
        "ell",
        "m",
        "seed",
        "trials",
        "ops",
        "smooth",
        "total_ms",
        "walk%",
        "orac%",
        "solv%"
    );
    for (g, p) in rows {
        let t = Instant::now();
        let Some(inst) = pick_instance_with_budget(p, g, bsgs_max) else {
            println!("{g:>2} {p:>6}   skipped: no usable prime-order subgroup");
            continue;
        };
        let pick_ms = t.elapsed().as_secs_f64() * 1e3;
        let k_true = inst.ell / 3 + 7;
        let d2 = inst.curve.mul(&inst.d1, k_true);
        for s in 0..seeds {
            let order: Vec<&str> = match solver_mode.as_str() {
                "both" if s % 2 == 0 => vec!["reference", "fast"],
                "both" => vec!["fast", "reference"],
                one => vec![one],
            };
            let mut gate: Vec<(Option<u64>, u64, u64, u64, u64, u64)> = Vec::new();
            for name in order {
                let params = IcParams {
                    seed: 20260916 + s,
                    solver: solver_of(name),
                    ..IcParams::default()
                };
                let rep = solve_dlp(&inst.curve, &inst.d1, &d2, inst.ell, &params);
                gate.push((
                    rep.k,
                    rep.trials,
                    rep.solve.pivot_hash,
                    rep.solve.pivots,
                    rep.solve.row_ops,
                    rep.solve.mulmods,
                ));
                let ok = rep.k == Some(k_true);
                let tot = rep.total_ns as f64;
                let pct = |x: u128| 100.0 * x as f64 / tot.max(1.0);
                println!(
                "{:>2} {:>6} {:>12} {:>6} {:>4} {:>9} {:>9} {:>6.3} {:>9.3} {:>6.1} {:>6.1} {:>6.1}  {} {}",
                g,
                p,
                inst.ell,
                rep.m,
                s,
                rep.trials,
                rep.group_ops,
                rep.smoothness_rate(),
                tot / 1e6,
                pct(rep.walk_ns),
                pct(rep.oracle_ns),
                pct(rep.solve_ns),
                if ok { "ok" } else { "UNSOLVED - not a result" },
                name
            );
                println!(
                "JSON {{\"engine\":\"hec-native-ic\",\"solver\":\"{}\",\"genus\":{},\"p\":{},\"c\":{},\"ell\":{},\"m\":{},\"seed\":{},\
                 \"trials\":{},\"relations\":{},\"group_ops\":{},\"precompute_ops\":{},\"smoothness\":{:.5},\
                 \"oracle_calls\":{},\"parity_rejects\":{},\"frobenius\":{},\"solve_rounds\":{},\"solve_pivots\":{},\
                 \"solve_mulmods\":{},\"solve_row_ops\":{},\"solve_pivot_hash\":\"{:016x}\",\"fb_ms\":{:.4},\"walk_ms\":{:.4},\"oracle_ms\":{:.4},\"row_ms\":{:.4},\
                 \"solve_ms\":{:.4},\"total_ms\":{:.4},\"pick_ms\":{:.3},\"correct\":{}}}",
                name,
                g,
                p,
                inst.c,
                inst.ell,
                rep.m,
                20260916 + s,
                rep.trials,
                rep.relations,
                rep.group_ops,
                rep.precompute_ops,
                rep.smoothness_rate(),
                rep.oracle.calls,
                rep.oracle.parity_rejects,
                rep.oracle.frobenius,
                rep.solve_rounds,
                rep.solve.pivots,
                rep.solve.mulmods,
                rep.solve.row_ops,
                rep.solve.pivot_hash,
                rep.fb_ns as f64 / 1e6,
                rep.walk_ns as f64 / 1e6,
                rep.oracle_ns as f64 / 1e6,
                rep.row_ns as f64 / 1e6,
                rep.solve_ns as f64 / 1e6,
                tot / 1e6,
                pick_ms,
                ok
            );
            }
            if gate.len() == 2 {
                assert_eq!(
                    gate[0], gate[1],
                    "E3 exactness gate failed: g={g} p={p} seed={s}"
                );
                println!("GATE g={g} p={p} seed={s} identical");
            }
        }
    }
}
