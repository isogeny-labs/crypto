//! Driver for research/ic_klp_ladder_20261102 Amendment 2: the existing Gaudry
//! engine on E/F_{p^3} with 0..3 large primes per relation, and rho on the same
//! instance. Calls library functions only. Prints one JSON line per run.
//!
//! cargo run --release --example klp_cubic -- P SEEDS

use crypto_lib::cryptanalysis::gaudry_cubic::{
    generate_instance3, run_gaudry_opts, run_rho3, GaudryOptions, SubspaceBase,
};
use rand::{rngs::StdRng, SeedableRng};
use serde_json::json;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let p: u64 = args[1].parse().expect("p");
    let seeds: u64 = args[2].parse().expect("seeds");
    for seed in 0..seeds {
        let inst = generate_instance3(p, 20261102 + seed);
        let mut rng = StdRng::seed_from_u64(seed);
        let base = SubspaceBase::build(&inst, &mut rng);
        let rho = run_rho3(&inst, seed ^ 0x3141_5926);
        println!(
            "{}",
            json!({"p": p, "seed": seed, "arm": "rho", "n": rho.n, "s": rho.s,
                   "group_ops": rho.group_ops, "solved": rho.solved, "correct": rho.correct})
        );
        let arms: [(u8, f64); 5] = [(0, 1.0), (1, 2.0 / 3.0), (2, 2.0 / 3.0), (3, 2.0 / 3.0), (3, 0.75)];
        for (lp, e) in arms {
            let small = if lp == 0 {
                0
            } else {
                ((base.len() as f64).powf(e)).ceil() as usize
            };
            let rep = run_gaudry_opts(
                &inst,
                &base,
                seed,
                &GaudryOptions {
                    max_residuals: 2_000_000,
                    sparse_la: true,
                    small_base: small,
                    max_large_primes: lp,
                    ..GaudryOptions::default()
                },
            );
            println!(
                "{}",
                json!({"p": p, "seed": seed, "arm": format!("lp{lp}_e{e:.3}"), "n": rep.n,
                       "base": rep.base, "small_base": rep.small_base, "max_large_primes": rep.max_large_primes,
                       "s": rep.s, "total_ops": rep.total_ops, "group_ops": rep.group_ops,
                       "residuals": rep.residuals, "lp_histogram": rep.lp_histogram,
                       "lp_full_relations": rep.lp_full_relations, "solved": rep.solved, "correct": rep.correct})
            );
        }
    }
}
