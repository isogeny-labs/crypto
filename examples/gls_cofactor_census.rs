//! E26 (note §9.17): GLS twists over `F_{p⁴}` from E25's construction without
//! the halves condition; per curve `t mod 4`, `v₂(#E)`, whether the halves of
//! `T₂` have rational abscissae, and whether `E₀(F_{p²})` has a point of order 4.
//!
//! `cargo run --release --example gls_cofactor_census -- --sizes 29,37 --seeds 40 --json OUT`

use crypto_lib::cryptanalysis::quartic_folds::gls_cofactor_census;
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
    let seeds: u64 = arg("--seeds").map_or(40, |s| s.parse().unwrap());
    let mut rows = Vec::new();
    for &p in &sizes {
        let (mut t2, mut t0, mut viol39, mut viol40, mut both) = (0, 0, 0, 0, 0);
        for seed in 1..=seeds {
            match gls_cofactor_census(p, seed) {
                Ok(r) => {
                    if r.t_mod_4 == 2 {
                        t2 += 1;
                        if r.halves_t2_rational_x {
                            viol39 += 1;
                        }
                        if r.v2_order != 2 || !r.e0_has_order4_point {
                            both += 1;
                        }
                    } else {
                        t0 += 1;
                    }
                    if r.halves_t2_rational_x && (r.t_mod_4 != 0 || r.v2_order < 4) {
                        viol40 += 1;
                    }
                    rows.push(serde_json::to_value(&r).unwrap());
                }
                Err(e) => {
                    eprintln!("p {p} seed {seed}: {e}");
                    rows.push(serde_json::json!({"p": p, "seed": seed, "error": e}));
                }
            }
        }
        eprintln!(
            "p {p}: t≡2 (mod 4) {t2}, 4|t {t0}; G2.38 exceptions {both}; G2.39 violations {viol39}; G2.40 violations {viol40}"
        );
    }
    let out = serde_json::to_string_pretty(&rows).unwrap() + "\n";
    match arg("--json") {
        Some(path) => std::fs::write(path, out).expect("write"),
        None => print!("{out}"),
    }
}
