//! Round eleven: the reference DP walk on `Z/N` as a control.
//!
//! Protocol (registered before this file existed):
//! `research/notes/index-calculus/RESEARCH_HYPERELLIPTIC_IC_RHO.md`,
//! "Round eleven".  Same 18 instances and 40 seeds as rounds nine and ten:
//! the plain Jacobian walk against the identical walk on the logarithms
//! (`rho_dp_integer_model`).  Counted steps only; verdicts computed here.
//!
//! ```sh
//! cargo run --release --example hyperelliptic_rho_integer_control
//! ```

use std::io::{self, Write};

use num_bigint::BigUint;

use crypto_lib::cryptanalysis::hyperelliptic_ic_bench::{
    pollard_rho_jacobian_with, rho_dp_integer_model, thread_instance, RhoVariant,
};

const SWEEP: [(u32, &[u64]); 3] = [
    (2, &[41, 61, 101, 151, 211, 251]),
    (3, &[23, 31, 41, 61, 101, 151, 211]),
    (4, &[17, 23, 31, 41, 61]),
];
const RHO_SEED: u64 = 0xC0FFEE;
const RHO_RUNS: u64 = 40;
const MAX_STEPS: usize = 200_000_000;

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
    let m = v.len() / 2;
    if v.len() % 2 == 1 {
        v[m]
    } else {
        (v[m - 1] + v[m]) / 2.0
    }
}

fn main() {
    println!(
        "Round eleven. Plain DP rho on the Jacobian vs the same walk on Z/N (S_walk = mean walk steps / sqrt(N), {RHO_RUNS} seeds from {RHO_SEED:#x})."
    );
    println!(
        "{:>2} {:>4} {:>9} {:>9} {:>9} {:>7} {:>9} {:>9} {:>3}",
        "g", "p", "N", "jacobian", "integer", "ratio", "int min", "int med", "ok"
    );
    let mut rows: Vec<(f64, f64, f64, bool)> = Vec::new();
    for (genus, primes) in SWEEP {
        for &p in primes {
            let Some((curve, d1, n, _c)) = thread_instance(p, genus) else {
                println!("{genus:>2} {p:>4}   no instance");
                continue;
            };
            let k = &n / BigUint::from(3u32) + BigUint::from(7u32);
            let d2 = d1.scalar_mul(&k, &curve);
            let nf: f64 = n.to_string().parse().expect("N fits f64");
            let root = nf.sqrt();
            let (mut jac, mut int) = (0f64, Vec::new());
            let mut ok = true;
            for t in 0..RHO_RUNS {
                let seed = RHO_SEED.wrapping_add(t);
                let a = pollard_rho_jacobian_with(
                    &curve,
                    &d1,
                    &d2,
                    &n,
                    seed,
                    MAX_STEPS,
                    &RhoVariant::DistinguishedPoints,
                );
                let b = rho_dp_integer_model(&n, &k, seed, MAX_STEPS);
                ok &= a.k.as_ref() == Some(&k) && b.k.as_ref() == Some(&k);
                jac += a.walk_ops as f64;
                int.push(b.walk_ops as f64 / root);
            }
            let jac_s = jac / RHO_RUNS as f64 / root;
            let int_s = int.iter().sum::<f64>() / RHO_RUNS as f64;
            let int_min = int.iter().cloned().fold(f64::INFINITY, f64::min);
            println!(
                "{:>2} {:>4} {:>9} {:>9.3} {:>9.3} {:>7.3} {:>9.3} {:>9.3} {:>3}",
                genus,
                p,
                n,
                jac_s,
                int_s,
                jac_s / int_s,
                int_min,
                median(int.clone()),
                if ok { "y" } else { "N" }
            );
            io::stdout().flush().ok();
            rows.push((nf, jac_s, int_s, ok));
        }
    }
    let big: Vec<&(f64, f64, f64, bool)> = rows.iter().filter(|r| r.0 >= 1e5).collect();
    let c1 = median(big.iter().map(|r| r.2).collect());
    let mut by_n: Vec<&(f64, f64, f64, bool)> = rows.iter().collect();
    by_n.sort_by(|a, b| b.0.partial_cmp(&a.0).expect("finite"));
    let top2: Vec<f64> = by_n.iter().take(2).map(|r| r.1 / r.2).collect();
    println!();
    let c1_ok = (1.10..=1.40).contains(&c1);
    println!(
        "C1 (integer-model median S_walk, N >= 1e5, {} rows): {:.3} -> {}",
        big.len(),
        c1,
        if c1_ok {
            "HOLDS (in [1.10, 1.40])"
        } else {
            "FALSIFIED (model not trusted; C2 not read)"
        }
    );
    if c1_ok {
        let verdict = if top2.iter().all(|&r| r > 1.25) {
            "excess is on the JACOBIAN side (both largest rows > 1.25x the model)"
        } else if top2.iter().all(|&r| r <= 1.25) {
            "excess is in the SHARED WALK LOGIC (both largest rows within 1.25x)"
        } else {
            "MIXED (the two largest rows disagree)"
        };
        println!(
            "C2 (jacobian/integer on the two largest rows): {:?} -> {}",
            top2.iter().map(|r| format!("{r:.3}")).collect::<Vec<_>>(),
            verdict
        );
    }
    println!(
        "C3 (every run verified): {}",
        if rows.iter().all(|r| r.3) {
            "HOLDS"
        } else {
            "FALSIFIED"
        }
    );
}
