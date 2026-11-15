//! End-to-end check for the large-prime sieve probe (not under protocol):
//! do relations collected to GTTD's target (`fulls + cycles ≥ #B + 10`)
//! actually give `k`?  Per (row, seed), on a prime-order instance, each arm
//! (walk, sieve) runs to `⌈μ·(#B + 10)⌉` for μ = 1, 1.1, 1.25, 1.5, 2 until
//! the captured rows, after singleton large-prime pruning, solve
//! (`Solver::Fast`) to a `k` with `k·D₁ = D₂`.  One `JSON {...}` line per
//! (row, seed, arm) with the first μ that worked.
//!
//! ```sh
//! HEC_ROWS=3:2039,4:509 cargo run --release --features count-ops --example lp_sieve_solve
//! ```
use hec_native_ic::instance::{factor_base, pick_instance_with_budget};
use hec_native_ic::lp_sieve::{lp_sieve, lp_walk, solve_captured, Base, Capture};
use hec_native_ic::solve::{SolveStats, Solver};

fn env<T: std::str::FromStr>(k: &str, d: T) -> T {
    std::env::var(k).ok().and_then(|v| v.parse().ok()).unwrap_or(d)
}

fn main() {
    let rows: Vec<(usize, u64)> = std::env::var("HEC_ROWS")
        .unwrap_or_else(|_| "3:2039,4:509".into())
        .split(',')
        .filter_map(|t| {
            let (g, p) = t.split_once(':')?;
            Some((g.trim().parse().ok()?, p.trim().parse().ok()?))
        })
        .collect();
    let kappa: f64 = env("HEC_KAPPA", 1.0);
    let seeds: u64 = env("HEC_SEEDS", 3);
    let bsgs: usize = env("HEC_BSGS_MAX", 2_000_000);
    let mus = [1.0f64, 1.1, 1.25, 1.5, 2.0];
    // HEC_LP_SOLVER=sgebw solves the pruned 2LP matrices with Solver::SgeBw.
    let solver = match std::env::var("HEC_LP_SOLVER").as_deref() {
        Ok("sgebw") => Solver::SgeBw,
        _ => Solver::Fast,
    };
    for (g, p) in rows {
        let t = std::time::Instant::now();
        let Some(inst) = pick_instance_with_budget(p, g, bsgs) else {
            println!("# g={g} p={p}: no usable instance");
            continue;
        };
        let pick_s = t.elapsed().as_secs_f64();
        let curve = &inst.curve;
        let k_true = inst.ell / 3 + 7;
        let d2 = curve.mul(&inst.d1, k_true);
        let fb = factor_base(curve);
        let nb = (kappa * (p as f64).powf(1.0 - 1.0 / g as f64)).round() as usize;
        let base = Base::new(&fb, nb, p);
        let nb = base.b.len();
        for s in 0..seeds {
            let seed = 20261115 + s;
            for arm in ["walk", "sieve"] {
                let mut done = None;
                for &mu in &mus {
                    let tgt = (mu * (nb + 10) as f64).ceil() as u64;
                    let mut cap = Capture {
                        ell: inst.ell,
                        rows: Vec::new(),
                    };
                    let (reached, products, partials, useful) = if arm == "walk" {
                        let w = lp_walk(curve, &inst.d1, &d2, &base, 100_000_000, seed, tgt, Some(&mut cap));
                        (w.reached, w.ops_rel.products, w.edges.len(), w.useful)
                    } else {
                        let r = lp_sieve(curve, &inst.d1, &d2, &base, 10_000_000, seed, 50, tgt, Some(&mut cap));
                        (r.reached, r.ops_rel.products, r.edges.len(), r.useful)
                    };
                    let mut st = SolveStats::default();
                    let (k, kept, lpc) = solve_captured(&cap.rows, nb, inst.ell, solver, &mut st);
                    let ok = k.is_some_and(|k| curve.mul(&inst.d1, k) == d2);
                    println!(
                        "# {arm} g={g} p={p} seed={seed} mu={mu} target={tgt} reached={reached} rows={} kept={kept} lp_cols={lpc} cols={} useful={useful} partials={partials} k={k:?} ok={ok}",
                        cap.rows.len(),
                        nb + lpc
                    );
                    if ok {
                        done = Some((mu, tgt, products, cap.rows.len(), kept, lpc, partials, st.mulmods));
                        break;
                    }
                }
                match done {
                    Some((mu, tgt, products, captured, kept, lpc, partials, mm)) => println!(
                        "JSON {{\"arm\":\"{arm}\",\"genus\":{g},\"p\":{p},\"ell\":{},\"kappa\":{kappa},\"nb\":{nb},\"seed\":{seed},\"pick_s\":{pick_s:.1},\
\"mu\":{mu},\"target\":{tgt},\"products\":{products},\"rows_captured\":{captured},\"rows_kept\":{kept},\"lp_cols\":{lpc},\"partials\":{partials},\"solve_mulmods\":{mm},\"solver\":\"{solver:?}\",\"k_ok\":true}}",
                        inst.ell
                    ),
                    None => println!(
                        "JSON {{\"arm\":\"{arm}\",\"genus\":{g},\"p\":{p},\"ell\":{},\"kappa\":{kappa},\"nb\":{nb},\"seed\":{seed},\"k_ok\":false}}",
                        inst.ell
                    ),
                }
            }
        }
    }
}
