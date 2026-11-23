//! Growing-arity prime-field ksum ladder (`m = m(L)`).
//!
//! ```bash
//! cargo run --release --example growing_arity_ladder
//! ```

use crypto_lib::cryptanalysis::growing_arity::{
    default_growing_arity_bits, run_growing_arity_ladder,
};

fn main() {
    let bits = default_growing_arity_bits();
    println!("=== growing-arity ksum ladder (prime-field, m=m(L)) ===");
    println!("bits: {bits:?}");
    match run_growing_arity_ladder(&bits, 5) {
        Ok(rep) => {
            for (i, r) in rep.rungs.iter().enumerate() {
                println!(
                    "[{i}] p={} m={} B={}/{} capped={} ic_gae(seed_a/b)={}/{} \
                     rho_expected_gae(seed_a/b)={}/{} log2_ic_observed={:.3} \
                     log2_rho_expected={:.3} below_rho_expectation={} verified={}",
                    r.p,
                    r.m,
                    r.b_used,
                    r.b_schedule,
                    r.capped,
                    r.ic_adds_seed_a,
                    r.ic_adds_seed_b,
                    r.rho_expected_adds_seed_a,
                    r.rho_expected_adds_seed_b,
                    r.log2_ic_observed_mean,
                    r.log2_rho_expected_mean,
                    r.below_rho_expectation,
                    r.recovered_ok
                );
            }
            println!(
                "local power-law OLS slope={:?} intercept={:?} (not L_p[α])",
                rep.local_power_slope_ols, rep.local_power_intercept_ols
            );
            println!("acceptance: {}", rep.acceptance);
            println!("verdict: {}", rep.verdict);
            println!("{}", serde_json::to_string_pretty(&rep).unwrap());
            if rep.verdict.starts_with("INCONCLUSIVE") && rep.rungs.iter().all(|r| !r.recovered_ok)
            {
                std::process::exit(2);
            }
        }
        Err(e) => {
            eprintln!("growing-arity ladder FAIL: {e}");
            std::process::exit(1);
        }
    }
}
