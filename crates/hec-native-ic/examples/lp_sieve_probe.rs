//! Reduced-base large-prime line sieve vs the walk (LEADS-20261114 lead 1,
//! follow-up A; not under a frozen protocol).  Per (row, κ, seed): build
//! `B` = the first `κ·p^{1−1/g}` places, run the walk for `HEC_TRIALS`
//! trials and the sieve for `HEC_LINES` lines from the same `D₁, D₂`, and
//! print one `JSON {...}` line per arm with relation classes and counted
//! field products (build with `--features count-ops`).
//!
//! ```sh
//! cargo run --release --features count-ops --example lp_sieve_probe
//! ```
use hec_native_ic::instance::{factor_base, thread_curve};
use hec_native_ic::lp_sieve::{cycles, lp_sieve, lp_walk, Base};
use hec_native_ic::rng::Rng;

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
        Err(_) => vec![(3, 2039), (3, 8191), (3, 32749), (4, 509), (4, 1021), (4, 2039)],
    };
    let kappas: Vec<f64> = std::env::var("HEC_KAPPAS")
        .unwrap_or_else(|_| "0.5,1".into())
        .split(',')
        .filter_map(|t| t.trim().parse().ok())
        .collect();
    let seeds: u64 = env("HEC_SEEDS", 3);
    let trials: u64 = env("HEC_TRIALS", 200_000);
    let lines: u64 = env("HEC_LINES", 2_000);
    let verify: u64 = env("HEC_VERIFY", 50);
    // HEC_TARGET=1: run each arm until fulls + cycles reach #B + 10 (budgets
    // HEC_TRIALS / HEC_LINES) and print only the to-target lines.
    let to_target = env("HEC_TARGET", 0u8) == 1;
    for (g, p) in rows {
        let Some((c, curve)) = (1..200).find_map(|c| thread_curve(p, c, g).map(|cv| (c, cv))) else {
            continue;
        };
        let fb = factor_base(&curve);
        for &kappa in &kappas {
            let nb = (kappa * (p as f64).powf(1.0 - 1.0 / g as f64)).round() as usize;
            let base = Base::new(&fb, nb, p);
            for s in 0..seeds {
                let seed = 20261115 + s;
                let mut rng = Rng::new(seed ^ 0x5eed);
                let mut rd = || {
                    let mut d = curve.zero();
                    for _ in 0..g {
                        let (x, y) = fb[rng.below(fb.len() as u64) as usize];
                        d = curve.add(&d, &curve.place(x, y));
                    }
                    d
                };
                let (d1, d2) = (rd(), rd());
                let head = format!(
                    "\"genus\":{g},\"p\":{p},\"c\":{c},\"kappa\":{kappa},\"nb\":{},\"places\":{},\"seed\":{seed}",
                    base.b.len(),
                    base.places
                );
                if to_target {
                    let tgt = base.b.len() as u64 + 10;
                    let w = lp_walk(&curve, &d1, &d2, &base, trials, seed, tgt);
                    let r = lp_sieve(&curve, &d1, &d2, &base, lines, seed, verify, tgt);
                    println!(
                        "JSON {{\"method\":\"target\",{head},\"target\":{tgt},\"walk_reached\":{},\"walk_trials\":{},\"walk_full\":{},\"walk_partials\":{},\"walk_useful\":{},\"walk_products\":{},\"walk_ms\":{:.3},\"sieve_reached\":{},\"sieve_lines\":{},\"sieve_full\":{},\"sieve_partials\":{},\"sieve_useful\":{},\"sieve_products\":{},\"sieve_verified\":{},\"sieve_ms\":{:.3}}}",
                        w.reached, w.trials, w.classes.full, w.edges.len(), w.useful, w.ops_rel.products, w.ns as f64 / 1e6,
                        r.reached, r.lines, r.classes.full, r.edges.len(), r.useful, r.ops_rel.products, r.verified, r.ns as f64 / 1e6
                    );
                    continue;
                }
                let w = lp_walk(&curve, &d1, &d2, &base, trials, seed, 0);
                println!(
                    "JSON {{\"method\":\"walk\",{head},\"trials\":{},\"split\":{},\"full\":{},\"p1\":{},\"p2\":{},\"discarded\":{},\
\"ops_products\":{},\"ops_redc\":{},\"ops_inv\":{},\"ms\":{:.3}}}",
                    w.trials,
                    w.split,
                    w.classes.full,
                    w.classes.p1,
                    w.classes.p2,
                    w.classes.discarded,
                    w.ops_rel.products,
                    w.ops_rel.redc,
                    w.ops_rel.inv,
                    w.ns as f64 / 1e6
                );
                let r = lp_sieve(&curve, &d1, &d2, &base, lines, seed, verify, 0);
                // Large-prime graph at an equal partial budget: the first
                // n partials of each arm, n = the smaller arm's count.
                let n = w.edges.len().min(r.edges.len());
                // And at equal counted work: truncate the arm that spent more
                // products, assuming its partials accrue evenly over its run.
                let (pw, ps) = (w.ops_rel.products.max(1) as f64, r.ops_rel.products.max(1) as f64);
                let nw_eq = if pw > ps { (w.edges.len() as f64 * ps / pw) as usize } else { w.edges.len() };
                let ns_eq = if ps > pw { (r.edges.len() as f64 * pw / ps) as usize } else { r.edges.len() };
                println!(
                    "JSON {{\"method\":\"graph\",{head},\"partials\":{n},\"walk_cycles\":{},\"sieve_cycles\":{},\
\"walk_partials_all\":{},\"sieve_partials_all\":{},\"walk_cycles_all\":{},\"sieve_cycles_all\":{},\
\"eq_products\":{},\"walk_partials_eq\":{nw_eq},\"sieve_partials_eq\":{ns_eq},\"walk_cycles_eq\":{},\"sieve_cycles_eq\":{}}}",
                    cycles(&w.edges, n),
                    cycles(&r.edges, n),
                    w.edges.len(),
                    r.edges.len(),
                    cycles(&w.edges, usize::MAX),
                    cycles(&r.edges, usize::MAX),
                    pw.min(ps) as u64,
                    cycles(&w.edges, nw_eq),
                    cycles(&r.edges, ns_eq)
                );
                println!(
                    "JSON {{\"method\":\"sieve\",{head},\"lines\":{},\"degenerate_lines\":{},\"full\":{},\"p1\":{},\"p2\":{},\"discarded\":{},\
\"cand_g\":{},\"cand_gm1\":{},\"nonresidue\":{},\"verified\":{},\
\"ops_products\":{},\"ops_redc\":{},\"ops_inv\":{},\"verify_products\":{},\"ms\":{:.3}}}",
                    r.lines,
                    r.degenerate_lines,
                    r.classes.full,
                    r.classes.p1,
                    r.classes.p2,
                    r.classes.discarded,
                    r.cand_g,
                    r.cand_gm1,
                    r.nonresidue,
                    r.verified,
                    r.ops_rel.products,
                    r.ops_rel.redc,
                    r.ops_rel.inv,
                    r.ops_verify.products,
                    r.ns as f64 / 1e6
                );
            }
        }
    }
}
