//! Line-sieve feasibility probe (LEADS-20261114 lead 1; not under a frozen
//! protocol).  For each row and seed, runs the factor-base walk IC
//! (`Solver::Dense`) and the line sieve (`Solver::Fast`) on the same instance
//! and planted `k`, and prints one `JSON {...}` line per (row, seed, method).
//!
//! ```sh
//! cargo run --release --example sieve_probe
//! HEC_ROWS=2:16411,3:1021,4:127 HEC_BSGS_MAX=2000000 cargo run --release --example sieve_probe
//! ```
use std::time::Instant;

use hec_native_ic::ic::{solve_dlp, IcParams};
use hec_native_ic::instance::pick_instance_with_budget;
use hec_native_ic::sieve::{solve_dlp_sieve, SieveParams};
use hec_native_ic::solve::Solver;

fn env<T: std::str::FromStr>(k: &str, d: T) -> T {
    std::env::var(k).ok().and_then(|v| v.parse().ok()).unwrap_or(d)
}

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
    let seeds: u64 = env("HEC_SEEDS", 5);
    let bsgs: usize = env("HEC_BSGS_MAX", 200_000);
    let skip_walk = std::env::var("HEC_SKIP_WALK").is_ok();
    // HEC_VERIFY=0 turns off the per-relation Jacobian check (for timing).
    let verify = env("HEC_VERIFY", 1u8) == 1;
    let fact = |n: usize| (1..=n as u64).product::<u64>() as f64;
    for (g, p) in rows {
        let t = Instant::now();
        let Some(inst) = pick_instance_with_budget(p, g, bsgs) else {
            println!("# g={g} p={p}: no usable instance");
            continue;
        };
        let pick_ms = t.elapsed().as_secs_f64() * 1e3;
        let k_true = inst.ell / 3 + 7;
        let d2 = inst.curve.mul(&inst.d1, k_true);
        for s in 0..seeds {
            let seed = 20260916 + s;
            if !skip_walk {
                let rep = solve_dlp(
                    &inst.curve,
                    &inst.d1,
                    &d2,
                    inst.ell,
                    &IcParams {
                        seed,
                        solver: Solver::Dense,
                        ..IcParams::default()
                    },
                );
                println!(
                    "JSON {{\"method\":\"walk\",\"genus\":{g},\"p\":{p},\"c\":{},\"ell\":{},\"m\":{},\"seed\":{seed},\
\"trials\":{},\"relations\":{},\"group_ops\":{},\"oracle_calls\":{},\"solve_rounds\":{},\
\"trials_per_rel\":{:.3},\"fb_ms\":{:.4},\"solve_ms\":{:.4},\"total_ms\":{:.4},\"solve_mulmods\":{},\"pick_ms\":{pick_ms:.1},\"correct\":{}}}",
                    inst.c,
                    inst.ell,
                    rep.m,
                    rep.trials,
                    rep.relations,
                    rep.group_ops,
                    rep.oracle.calls,
                    rep.solve_rounds,
                    rep.trials as f64 / rep.relations.max(1) as f64,
                    rep.fb_ns as f64 / 1e6,
                    rep.solve_ns as f64 / 1e6,
                    rep.total_ns as f64 / 1e6,
                    rep.solve.mulmods,
                    rep.k == Some(k_true)
                );
            }
            let rep = solve_dlp_sieve(
                &inst.curve,
                &inst.d1,
                &d2,
                inst.ell,
                &SieveParams {
                    seed,
                    verify_relations: if verify { usize::MAX } else { 0 },
                    stop_mid_line: env("HEC_STOP_MID_LINE", 1u8) == 1,
                    solver: match std::env::var("HEC_SIEVE_SOLVER").as_deref() {
                        Ok("dense") => Solver::Dense,
                        Ok("reference") => Solver::Reference,
                        Ok("sgebw") => Solver::SgeBw,
                        _ => Solver::Fast,
                    },
                    ..SieveParams::default()
                },
            );
            let good = (rep.lines - rep.degenerate_lines).max(1) as f64;
            println!(
                "JSON {{\"method\":\"sieve\",\"genus\":{g},\"p\":{p},\"c\":{},\"ell\":{},\"m\":{},\"seed\":{seed},\
\"lines\":{},\"degenerate_lines\":{},\"relations\":{},\"verified\":{},\"group_ops\":{},\"solve_rounds\":{},\
\"rel_per_line\":{:.3},\"expected_rel_per_line\":{:.3},\"rel_over_m\":{:.4},\
\"field_mults\":{},\"field_invs\":{},\"mults_per_rel\":{:.2},\"line_adds_per_rel\":{:.4},\"sieve_points_per_line\":{:.1},\
\"fb_ms\":{:.4},\"sieve_ms\":{:.4},\"solve_ms\":{:.4},\"total_ms\":{:.4},\"solve_mulmods\":{},\"solve_row_ops\":{},\"correct\":{}}}",
                inst.c,
                inst.ell,
                rep.m,
                rep.lines,
                rep.degenerate_lines,
                rep.relations,
                rep.relations_verified,
                rep.group_ops,
                rep.solve_rounds,
                rep.relations as f64 / good,
                p as f64 / fact(g + 1),
                rep.relations as f64 / rep.m as f64,
                rep.field_mults,
                rep.field_invs,
                rep.field_mults as f64 / rep.relations.max(1) as f64,
                rep.group_ops as f64 / rep.relations.max(1) as f64,
                rep.sieve_points as f64 / good,
                rep.fb_ns as f64 / 1e6,
                rep.sieve_ns as f64 / 1e6,
                rep.solve_ns as f64 / 1e6,
                rep.total_ns as f64 / 1e6,
                rep.solve.mulmods,
                rep.solve.row_ops,
                rep.k == Some(k_true)
            );
        }
    }
}
