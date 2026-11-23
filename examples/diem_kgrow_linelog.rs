//! Bit-scale LineOracle ladder on `generate_gls_instance` /
//! `generate_subfield_instance`.
//!
//! ```bash
//! cargo run --release --example diem_kgrow_linelog
//! ```

use crypto_lib::cryptanalysis::diem_kgrow_linelog::{
    default_gls_p_bits, run_linelog_ladder,
};

fn main() {
    let bits = default_gls_p_bits();
    println!("=== diem_kgrow linelog ladder (GLS LineOracle + subfield S₄) ===");
    println!("gls p_bits: {bits:?}; subfield p_bits: Some(7)");
    match run_linelog_ladder(&bits, Some(7), 2, 7) {
        Ok(rep) => {
            for (i, r) in rep.rungs.iter().enumerate() {
                println!(
                    "[{i}] p={} k={} ic(ab/ba)={}/{} rho(ab/ba)={}/{} \
                     log2_ic={:.3} log2_rho={:.3} α_proxy={:.3} beats_ρ={} recovered_ok={}",
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
            eprintln!("linelog ladder FAIL: {e}");
            std::process::exit(1);
        }
    }
}
