//! Round twelve: do the rho reference's short branch coefficients cause
//! its large-`N` excess?
//!
//! Protocol (registered before this file existed):
//! `research/notes/index-calculus/RESEARCH_HYPERELLIPTIC_IC_RHO.md`,
//! "Round twelve".  Same 18 instances and 40 seeds as rounds nine to
//! eleven; four arms: the plain DP walk on the Jacobian and on `Z/N`, each
//! with 8-bit and with full-width branch coefficients.  Counted operations
//! only, deterministic in the seed; verdicts computed here.
//!
//! ```sh
//! cargo run --release --example hyperelliptic_rho_branch_width
//! ```

use std::io::{self, Write};

use num_bigint::BigUint;

use crypto_lib::cryptanalysis::hyperelliptic_ic_bench::{
    pollard_rho_jacobian_with, rho_dp_integer_model_width, thread_instance, RhoVariant,
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

/// `S_walk` per arm: Jacobian 8-bit, Jacobian full, integer 8-bit,
/// integer full.
struct Row {
    n: f64,
    s: [f64; 4],
    full_precompute: f64,
    ok: bool,
}

fn main() {
    println!(
        "Round twelve. Plain DP rho, 8-bit vs full-width branch coefficients (S_walk = mean walk ops / sqrt(N), {RHO_RUNS} seeds from {RHO_SEED:#x})."
    );
    println!(
        "{:>2} {:>4} {:>9} {:>8} {:>8} {:>8} {:>8} {:>9} {:>3}",
        "g", "p", "N", "jac 8b", "jac full", "int 8b", "int full", "full pre", "ok"
    );
    let mut rows = Vec::new();
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
            let mut w = [0f64; 4];
            let mut pre = 0f64;
            let mut ok = true;
            for t in 0..RHO_RUNS {
                let seed = RHO_SEED.wrapping_add(t);
                let runs = [
                    pollard_rho_jacobian_with(
                        &curve,
                        &d1,
                        &d2,
                        &n,
                        seed,
                        MAX_STEPS,
                        &RhoVariant::DistinguishedPoints,
                    ),
                    pollard_rho_jacobian_with(
                        &curve,
                        &d1,
                        &d2,
                        &n,
                        seed,
                        MAX_STEPS,
                        &RhoVariant::DistinguishedPointsFullWidth,
                    ),
                    rho_dp_integer_model_width(&n, &k, seed, MAX_STEPS, false),
                    rho_dp_integer_model_width(&n, &k, seed, MAX_STEPS, true),
                ];
                for (i, r) in runs.iter().enumerate() {
                    ok &= r.k.as_ref() == Some(&k);
                    w[i] += r.walk_ops as f64;
                }
                pre += runs[1].precompute_ops as f64;
            }
            let s = w.map(|x| x / RHO_RUNS as f64 / root);
            let full_precompute = pre / RHO_RUNS as f64;
            println!(
                "{:>2} {:>4} {:>9} {:>8.3} {:>8.3} {:>8.3} {:>8.3} {:>9.0} {:>3}",
                genus,
                p,
                n,
                s[0],
                s[1],
                s[2],
                s[3],
                full_precompute,
                if ok { "y" } else { "N" }
            );
            io::stdout().flush().ok();
            rows.push(Row {
                n: nf,
                s,
                full_precompute,
                ok,
            });
        }
    }

    let big: Vec<&Row> = rows.iter().filter(|r| r.n >= 1e5).collect();
    let small: Vec<&Row> = rows.iter().filter(|r| r.n <= 6e4).collect();
    let mut by_n: Vec<&Row> = rows.iter().collect();
    by_n.sort_by(|a, b| b.n.partial_cmp(&a.n).expect("finite"));
    let bounds = |arm: usize| {
        let med = median(big.iter().map(|r| r.s[arm]).collect());
        let top2: Vec<f64> = by_n.iter().take(2).map(|r| r.s[arm]).collect();
        let holds = (1.10..=1.40).contains(&med) && top2.iter().all(|&s| s <= 1.55);
        (med, top2, holds)
    };
    let fmt = |v: &[f64]| v.iter().map(|s| format!("{s:.3}")).collect::<Vec<_>>();
    let verdict = |b: bool| if b { "HOLDS" } else { "FALSIFIED" };

    println!();
    let (m1, t1, w1) = bounds(3);
    println!(
        "W1 (integer full width, N >= 1e5, {} rows): median {:.3} (in [1.10, 1.40]), two largest {:?} (<= 1.55) -> {}",
        big.len(),
        m1,
        fmt(&t1),
        verdict(w1)
    );
    let (m2, t2, w2) = bounds(1);
    println!(
        "W2 (Jacobian full width, N >= 1e5, {} rows): median {:.3} (in [1.10, 1.40]), two largest {:?} (<= 1.55) -> {}",
        big.len(),
        m2,
        fmt(&t2),
        verdict(w2)
    );
    let w3 = (median(small.iter().map(|r| r.s[1]).collect())
        - median(small.iter().map(|r| r.s[0]).collect()))
    .abs();
    println!(
        "W3 (control, N <= 6e4, {} rows): |median full - median 8-bit| Jacobian S_walk = {:.3} -> {}",
        small.len(),
        w3,
        verdict(w3 < 0.15)
    );
    println!(
        "W4 (every run verified): {}",
        verdict(rows.iter().all(|r| r.ok))
    );
    let largest = by_n[0];
    println!(
        "Largest row: Jacobian 8-bit / full-width S_walk = {:.3}; full-width precompute {:.0} ops = {:.4} sqrt(N).",
        largest.s[0] / largest.s[1],
        largest.full_precompute,
        largest.full_precompute / largest.n.sqrt()
    );
    println!("Ideal S_walk: 1.2533.");
}
