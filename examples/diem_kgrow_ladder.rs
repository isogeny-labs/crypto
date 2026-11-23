//! Charged `(p, k(p))` Diem ladder with matched ρ and α fit.
//!
//! ```bash
//! cargo run --release --example diem_kgrow_ladder
//! ```
//!
//! Runs the smallest discriminating ABBA/BAAB ladder of growing
//! `(p,k)` cells through [`crypto_lib::cryptanalysis::diem_kgrow::run_charged_ladder`],
//! the same charged path `ic.diem_kgrow` uses.  Prints predicted vs
//! observed costs and the OLS α fit.  Absolute F_p same-target is out of
//! scope.

use crypto_lib::cryptanalysis::diem_kgrow::{default_ladder_rungs, run_charged_ladder};

fn main() {
    let rungs = default_ladder_rungs();
    println!("=== diem_kgrow charged ladder (ABBA/BAAB, matched ρ) ===");
    println!("rungs: {rungs:?}");
    match run_charged_ladder(&rungs, 2_000_000) {
        Ok(rep) => {
            for (i, r) in rep.rungs.iter().enumerate() {
                println!(
                    "[{i}] p={} k={} ic_adds(ab/ba)={}/{} rho_adds(ab/ba)={}/{} \
                     log2_ic_obs={:.3} log2_rho_obs={:.3} log2_ic_pred={:.3} \
                     α_proxy={:.3} beats_ρ={} recovered_ok={}",
                    r.p,
                    r.k,
                    r.ic_adds_ab,
                    r.ic_adds_ba,
                    r.rho_adds_ab,
                    r.rho_adds_ba,
                    r.log2_ic_obs_mean,
                    r.log2_rho_obs_mean,
                    r.log2_ic_pred,
                    r.alpha_proxy,
                    r.beats_sqrt_r_obs,
                    r.recovered_ok
                );
            }
            println!(
                "OLS α={:?} β={:?}",
                rep.alpha_ols, rep.beta_ols
            );
            println!("acceptance: {}", rep.acceptance);
            println!("verdict: {}", rep.verdict);
            println!("{}", serde_json::to_string_pretty(&rep).unwrap());
            // Exit 0 even on FAIL_FOR_GOAL_α — that is a scientific result,
            // not a harness crash.  Exit 1 only on empty/infra failure.
            if rep.verdict.starts_with("INCONCLUSIVE") && rep.rungs.iter().all(|r| !r.recovered_ok)
            {
                std::process::exit(2);
            }
        }
        Err(e) => {
            eprintln!("ladder FAIL: {e}");
            std::process::exit(1);
        }
    }
}
