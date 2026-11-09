//! **Factor-base size of the `fp4_torsion` curves, by chained start seeds.**
//!
//! Companion to `research/gaudry_quartic_d4_exponent_20261015` (amendment 6).
//! For each `p`, walks the chain of curves `fp4_torsion(p, start, H)` returns
//! (the next start seed is one past the previous instance seed) and prints one
//! JSON line per curve with the number of columns of the `Y`-line base. No
//! decomposition, no solve and no index-calculus work is done: this is the
//! cheap quantity the screening scan correlates with cost.
//!
//! ```text
//! cargo run --release --example gaudry_quartic_d4_columns -- --p 101 --curves 50 --max-cofactor 16
//! ```

use std::env;

use crypto_lib::cryptanalysis::gaudry_quartic_d4::{find_d4_curve, y_base};
use rand::rngs::StdRng;
use rand::SeedableRng;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let get = |k: &str, d: u64| {
        args.iter()
            .position(|a| a == k)
            .map_or(d, |i| args[i + 1].parse().expect(k))
    };
    let p = get("--p", 101);
    let curves = get("--curves", 50);
    let h = get("--max-cofactor", 16);
    let mut start = get("--start", 1);
    for chain in 1..=curves {
        let Ok(c) = find_d4_curve(p, start, h) else {
            println!("{}", serde_json::json!({"p": p, "chain": chain, "start": start, "exhausted": true}));
            break;
        };
        // The base is a function of the curve alone: the lift rng only picks
        // which of the two points over each Y is kept.
        let mut rng = StdRng::seed_from_u64(c.instance_seed ^ 0x55);
        let cols = y_base(&c.inst, &mut rng).len();
        println!(
            "{}",
            serde_json::json!({"p": p, "chain": chain, "start": start,
                "instance_seed": c.instance_seed, "cofactor": c.cofactor, "r": c.r,
                "columns": cols, "ratio": cols as f64 / p as f64})
        );
        start = c.instance_seed + 1;
    }
}
