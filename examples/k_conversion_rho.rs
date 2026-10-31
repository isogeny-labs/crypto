//! Rho side of the K conversion (research/k_conversion_20261018/PROTOCOL.md).
//!
//! One signed-Frobenius strong-rho solve, wrapped in a never-inlined function
//! so that `valgrind --tool=callgrind --toggle-collect='*k_rho_solve*'` counts
//! the instructions of exactly that interval.  The walk-step count is the
//! solver's own `walk_steps` (complete solve, n = 61) or the step cap
//! (capped walk, n = 83, where a complete walk of about 2^39 steps cannot run).
//! The algebra and the walk are unchanged; this file only drives them.
//!
//! `k_conversion_rho n61 SEED` | `k_conversion_rho n83 SEED`
//!
//! Prints one JSON object on stdout.
use std::hint::black_box;

use crypto_lib::binary_ecc::IrreduciblePoly;
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use crypto_lib::cryptanalysis::koblitz_strong_rho::{StrongRho, StrongRhoCharges, StrongRhoParams};
use crypto_lib::cryptanalysis::koblitz_wide::WideKoblitz;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[inline(never)]
fn k_rho_solve_n61(
    rho: &StrongRho,
    q: crypto_lib::cryptanalysis::koblitz_strong_rho::RawPointG<u64>,
    jumps: &[crypto_lib::cryptanalysis::koblitz_strong_rho::JumpG<u64, u64>],
    seed: u64,
    params: &StrongRhoParams,
    charges: StrongRhoCharges,
) -> Option<(u64, u64)> {
    let mut start = StdRng::seed_from_u64(seed);
    rho.solve(black_box(q), jumps, &mut start, params, charges)
        .map(|o| (o.scalar, o.walk_steps))
}

#[inline(never)]
fn k_rho_solve_n83(
    rho: &crypto_lib::cryptanalysis::koblitz_strong_rho::WideStrongRho,
    q: crypto_lib::cryptanalysis::koblitz_strong_rho::RawPointG<u128>,
    jumps: &[crypto_lib::cryptanalysis::koblitz_strong_rho::JumpG<u128, u128>],
    seed: u64,
    params: &StrongRhoParams,
    charges: StrongRhoCharges,
) -> bool {
    let mut start = StdRng::seed_from_u64(seed);
    rho.solve(black_box(q), jumps, &mut start, params, charges)
        .is_none()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let which = args.get(1).map(String::as_str).unwrap_or("n61");
    let seed: u64 = args.get(2).map_or(1, |s| s.parse().expect("seed"));
    match which {
        "n61" => {
            let kc = KoblitzCurve::new(0, 61).expect("Koblitz a=0 n=61");
            let rho = StrongRho::new(&kc);
            let g = rho.generator();
            let secret = 0x0123_4567_89ab_cdefu64 % rho.modulus();
            let q = rho.scalar_mul(g, secret ^ (seed.wrapping_mul(0x9e37_79b9) % rho.modulus()));
            let want = secret ^ (seed.wrapping_mul(0x9e37_79b9) % rho.modulus());
            let params = StrongRhoParams::default();
            let mut charges = StrongRhoCharges::default();
            let jumps = rho.jumps(seed, &mut charges);
            let out = k_rho_solve_n61(&rho, q, &jumps, seed, &params, charges);
            let (found, steps) = out.expect("n=61 solve reaches no cap");
            println!(
                "{{\"n\":61,\"seed\":{seed},\"walk_steps\":{steps},\"verified\":{}}}",
                found == want
            );
        }
        "n83" => {
            let irr = IrreduciblePoly {
                degree: 83,
                low_terms: vec![0, 1, 2, 45],
            };
            let kc = WideKoblitz::new(0, &irr).expect("E_0 over GF(2^83)");
            let rho = kc.strong_rho();
            let g = rho.generator();
            let q = rho.scalar_mul(g, 0x5eed_0083);
            let params = StrongRhoParams {
                step_cap_factor: 0,
                ..StrongRhoParams::default()
            };
            let mut charges = StrongRhoCharges::default();
            let jumps = rho.jumps(seed, &mut charges);
            let capped = k_rho_solve_n83(&rho, q, &jumps, seed, &params, charges);
            println!(
                "{{\"n\":83,\"seed\":{seed},\"walk_steps\":1000000,\"capped\":{capped},\"note\":\"step_cap_factor 0 leaves the cap at its floor of 10^6\"}}"
            );
        }
        other => panic!("unknown mode {other}"),
    }
}
