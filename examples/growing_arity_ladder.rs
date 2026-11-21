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
                    "[{i}] p={} m={} ic_gae(ab/ba)={}/{} rho_gae(ab/ba)={}/{} \
                     log2_ic={:.3} log2_rho={:.3} α_proxy={:.3} beats_ρ={} verified={}",
                    r.p,
                    r.k,
                    r.ic_adds_ab,
                    r.ic_adds_ba,
                    r.rho_adds_ab,
                    r.rho_adds_ba,
                    r.log2_ic_obs_mean,
                    r.log2_rho_obs_mean,
                    r.alpha_proxy,
                    r.beats_sqrt_r_obs,
                    r.recovered_ok
                );
            }
            println!("OLS α={:?} β={:?}", rep.alpha_ols, rep.beta_ols);
            println!("acceptance: {}", rep.acceptance);
            println!("verdict: {}", rep.verdict);
            println!("{}", serde_json::to_string_pretty(&rep).unwrap());
            if rep.verdict.starts_with("INCONCLUSIVE")
                && rep.rungs.iter().all(|r| !r.recovered_ok)
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
