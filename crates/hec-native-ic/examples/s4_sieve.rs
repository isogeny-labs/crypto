//! Slice S4 (PLAN.md §10): the full-base line sieve against the walk at
//! genus 3 and 4, in counted field products.  For each row and seed, runs
//! the walk (`Solver::Dense`) and the sieve (`Solver::Fast`) on the same
//! instance and planted `k`, and prints one `JSON {...}` line per arm.
//!
//! Build with `--features count-ops` for the counts (the primary build) and
//! without for timing; `HEC_VERIFY=0` turns off the sieve's per-relation
//! Jacobian check (the timing build).  Fields named `ops_*`, `*_ms` and
//! `relations_verified` differ between the two builds by design (gate G11).
//!
//! ```sh
//! cargo run --release --features count-ops --example s4_sieve
//! HEC_VERIFY=0 cargo run --release --example s4_sieve
//! ```
use hec_native_ic::fp::OpCounts;
use hec_native_ic::ic::{solve_dlp, IcParams};
use hec_native_ic::instance::pick_instance_with_budget;
use hec_native_ic::sieve::{solve_dlp_sieve, SieveParams};
use hec_native_ic::solve::Solver;

fn env<T: std::str::FromStr>(k: &str, d: T) -> T {
    std::env::var(k).ok().and_then(|v| v.parse().ok()).unwrap_or(d)
}

fn ops(name: &str, o: &OpCounts) -> String {
    format!(
        "\"ops_{name}_products\":{},\"ops_{name}_redc\":{},\"ops_{name}_inv\":{}",
        o.products, o.redc, o.inv
    )
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
        // §10's rows.
        Err(_) => vec![
            (3, 61),
            (3, 211),
            (3, 401),
            (3, 1021),
            (3, 4001),
            (4, 31),
            (4, 61),
            (4, 127),
            (4, 251),
            (4, 509),
            (4, 1009),
        ],
    };
    let seeds: u64 = env("HEC_SEEDS", 5);
    let seed0: u64 = env("HEC_SEED0", 20261115);
    let bsgs: usize = env("HEC_BSGS_MAX", 2_000_000);
    let verify = env("HEC_VERIFY", 1u8) == 1;
    let fact = |n: usize| (1..=n as u64).product::<u64>() as f64;
    let ms = |ns: u128| ns as f64 / 1e6;
    for (g, p) in rows {
        let Some(inst) = pick_instance_with_budget(p, g, bsgs) else {
            println!("# g={g} p={p}: no usable instance (row dropped, §10)");
            continue;
        };
        let k_true = inst.ell / 3 + 7;
        let d2 = inst.curve.mul(&inst.d1, k_true);
        for s in 0..seeds {
            let seed = seed0 + s;
            let w = solve_dlp(
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
\"trials\":{},\"relations\":{},\"group_ops\":{},\"oracle_calls\":{},\"oracle_smooth\":{},\"solve_rounds\":{},\
\"solve_mulmods\":{},\"solve_row_ops\":{},\"solve_pivots\":{},\"solve_pivot_hash\":{},\
{},{},{},{},\
\"fb_ms\":{:.4},\"solve_ms\":{:.4},\"total_ms\":{:.4},\"k\":{},\"correct\":{}}}",
                inst.c,
                inst.ell,
                w.m,
                w.trials,
                w.relations,
                w.group_ops,
                w.oracle.calls,
                w.oracle.smooth,
                w.solve_rounds,
                w.solve.mulmods,
                w.solve.row_ops,
                w.solve.pivots,
                w.solve.pivot_hash,
                ops("fb", &w.ops_fb),
                ops("rel", &w.ops_rel),
                ops("solve", &w.ops_solve),
                ops("kcheck", &w.ops_kcheck),
                ms(w.fb_ns),
                ms(w.solve_ns),
                ms(w.total_ns),
                w.k.map_or(-1, |k| k as i128),
                w.k == Some(k_true)
            );
            let r = solve_dlp_sieve(
                &inst.curve,
                &inst.d1,
                &d2,
                inst.ell,
                &SieveParams {
                    seed,
                    verify_relations: if verify { usize::MAX } else { 0 },
                    ..SieveParams::default()
                },
            );
            let good = (r.lines - r.degenerate_lines).max(1) as f64;
            println!(
                "JSON {{\"method\":\"sieve\",\"genus\":{g},\"p\":{p},\"c\":{},\"ell\":{},\"m\":{},\"seed\":{seed},\
\"lines\":{},\"degenerate_lines\":{},\"relations\":{},\"relations_verified\":{},\"group_ops\":{},\"solve_rounds\":{},\
\"sieve_points\":{},\"hits\":{},\"field_mults\":{},\"field_invs\":{},\
\"rel_per_line\":{:.4},\"expected_rel_per_line\":{:.4},\"line_adds_per_rel\":{:.4},\
\"solve_mulmods\":{},\"solve_row_ops\":{},\"solve_pivots\":{},\"solve_pivot_hash\":{},\
{},{},{},{},{},\
\"fb_ms\":{:.4},\"sieve_ms\":{:.4},\"solve_ms\":{:.4},\"total_ms\":{:.4},\"k\":{},\"correct\":{}}}",
                inst.c,
                inst.ell,
                r.m,
                r.lines,
                r.degenerate_lines,
                r.relations,
                r.relations_verified,
                r.group_ops,
                r.solve_rounds,
                r.sieve_points,
                r.hits,
                r.field_mults,
                r.field_invs,
                r.relations as f64 / good,
                p as f64 / fact(g + 1),
                r.group_ops as f64 / r.relations.max(1) as f64,
                r.solve.mulmods,
                r.solve.row_ops,
                r.solve.pivots,
                r.solve.pivot_hash,
                ops("fb", &r.ops_fb),
                ops("rel", &r.ops_rel),
                ops("solve", &r.ops_solve),
                ops("kcheck", &r.ops_kcheck),
                ops("verify", &r.ops_verify),
                ms(r.fb_ns),
                ms(r.sieve_ns),
                ms(r.solve_ns),
                ms(r.total_ns),
                r.k.map_or(-1, |k| k as i128),
                r.k == Some(k_true)
            );
        }
    }
}
