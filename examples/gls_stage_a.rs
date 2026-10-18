//! E25 Stage A (note §9.15, G2.32): GLS twists over `F_{p⁴}` with the `V₄`
//! line, the `ψ` fold and the confinement test, one JSON array.
//!
//! `cargo run --release --example gls_stage_a -- --sizes 29,37,41 --seeds 4 --json OUT`

use crypto_lib::cryptanalysis::quartic_folds::run_gls_stage_a;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = |k: &str| {
        args.iter()
            .position(|a| a == k)
            .map(|i| args[i + 1].clone())
    };
    let sizes: Vec<u64> = arg("--sizes")
        .expect("--sizes")
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let seeds: u64 = arg("--seeds").map_or(4, |s| s.parse().unwrap());
    let mut rows = Vec::new();
    for &p in &sizes {
        for seed in 1..=seeds {
            match run_gls_stage_a(p, seed) {
                Ok(r) => {
                    eprintln!(
                        "p {p} seed {seed}: h {} base {}/{} cols {} → {} (ppc {:.2} → {:.2}) ν² ∈ F_p {} distinct logs {:?}",
                        r.cofactor, r.base_points, r.eligible, r.columns_v4, r.columns_psi,
                        r.points_per_column_v4, r.points_per_column_psi, r.nu2_in_fp,
                        r.distinct_column_logs
                    );
                    rows.push(serde_json::to_value(&r).unwrap());
                }
                Err(e) => {
                    eprintln!("p {p} seed {seed}: {e}");
                    rows.push(serde_json::json!({"p": p, "seed": seed, "error": e}));
                }
            }
        }
    }
    let out = serde_json::to_string_pretty(&rows).unwrap() + "\n";
    match arg("--json") {
        Some(path) => std::fs::write(path, out).expect("write"),
        None => print!("{out}"),
    }
}
