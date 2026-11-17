//! The curve models `y² = x³ − 3x + b` of chosen candidates of a 1997–98
//! NIST prime-curve seed run, with `b` taken by the 1997 root rule
//! ([`x962_b_root`]), for `research/nist_binary_skipped_20261020/`.
//!
//! ```bash
//! cargo run --release --example nist_prime_candidate_models -- --run 871696354 --candidates 876,1842
//! ```
//!
//! Prints `i seed_hex p b` (decimal `p`, `b`) per candidate.

use std::process::exit;

use crypto_lib::cryptanalysis::nist_seed_provenance::{
    prime_seed, x962_b_root, x962_b_squared, PRIME_SEEDS,
};
use num_bigint::BigUint;

fn arg(name: &str) -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
}

fn main() {
    let (Some(run), Some(list)) = (
        arg("--run").and_then(|s| s.parse::<u32>().ok()),
        arg("--candidates"),
    ) else {
        eprintln!("usage: nist_prime_candidate_models --run <srand48 value> --candidates i,j,...");
        exit(2);
    };
    let Some(curve) = PRIME_SEEDS.iter().find(|s| s.srand48 == run) else {
        eprintln!("srand48({run}) is not one of the published runs");
        exit(2);
    };
    let p = BigUint::parse_bytes(curve.p_hex.as_bytes(), 16).expect("p hex");
    for i in list
        .split(',')
        .map(|x| x.trim().parse::<u32>().expect("candidate index"))
    {
        let seed = prime_seed(run, i);
        let hex: String = seed.iter().map(|b| format!("{b:02x}")).collect();
        let b = x962_b_squared(&seed, &p)
            .and_then(|b2| x962_b_root(&p, &b2))
            .unwrap_or_else(|| panic!("candidate {i} has no b"));
        println!("{i} {hex} {p} {b}");
    }
}
