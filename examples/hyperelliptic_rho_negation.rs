//! Round nine: the rho reference with and without the negation map, on the
//! thread's instances.
//!
//! Protocol (registered before this file existed):
//! `research/notes/index-calculus/RESEARCH_HYPERELLIPTIC_IC_RHO.md`,
//! "Round nine".  Reads **counted operations only**, which are
//! deterministic in the seed and do not depend on the host, so one run
//! answers the registered hypotheses; the analysis is done here, natively,
//! and the verdict is printed with the table.
//!
//! ```sh
//! cargo run --release --example hyperelliptic_rho_negation
//! ```

use std::io::{self, Write};

use num_bigint::BigUint;

use crypto_lib::cryptanalysis::hyperelliptic_ic_bench::{
    pollard_rho_jacobian_with, thread_instance, RhoVariant,
};

/// Round seven's instance list, unchanged.
const SWEEP: [(u32, &[u64]); 3] = [
    (2, &[41, 61, 101, 151, 211, 251]),
    (3, &[23, 31, 41, 61, 101, 151, 211]),
    (4, &[17, 23, 31, 41, 61]),
];
const RHO_SEED: u64 = 0xC0FFEE;
const RHO_RUNS: u64 = 40;
const MAX_STEPS: usize = 200_000_000;

struct Row {
    genus: u32,
    p: u64,
    n: f64,
    plain_walk: f64,
    folded_walk: f64,
    folded_fruitless: f64,
    plain_total: f64,
    folded_total: f64,
    all_verified: bool,
}

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
        "Round nine. DP rho on divisors vs on ±-classes, {RHO_RUNS} seeds per row from {RHO_SEED:#x}; \
         counted group operations, means per run."
    );
    println!(
        "{:>2} {:>4} {:>9} {:>11} {:>11} {:>7} {:>9} {:>9} {:>9} {:>8} {:>3}",
        "g",
        "p",
        "N",
        "walk plain",
        "walk fold",
        "ratio",
        "fruitless",
        "S_walk p",
        "S_walk f",
        "S ratio",
        "ok"
    );
    let mut rows = Vec::new();
    for (genus, primes) in SWEEP {
        for &p in primes {
            let (curve, d1, n, _c) = match thread_instance(p, genus) {
                Some(v) => v,
                None => {
                    println!("{genus:>2} {p:>4}   no instance");
                    continue;
                }
            };
            let k = &n / BigUint::from(3u32) + BigUint::from(7u32);
            let d2 = d1.scalar_mul(&k, &curve);
            let (mut pw, mut fw, mut ff, mut pt, mut ft) = (0f64, 0f64, 0f64, 0f64, 0f64);
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
                let b = pollard_rho_jacobian_with(
                    &curve,
                    &d1,
                    &d2,
                    &n,
                    seed,
                    MAX_STEPS,
                    &RhoVariant::DistinguishedPointsNegation,
                );
                ok &= a.k.as_ref() == Some(&k) && b.k.as_ref() == Some(&k);
                pw += a.walk_ops as f64;
                fw += b.walk_ops as f64;
                ff += b.fruitless_ops as f64;
                pt += a.group_ops as f64;
                ft += b.group_ops as f64;
            }
            let r = RHO_RUNS as f64;
            let nf: f64 = n.to_string().parse().expect("N fits f64");
            let row = Row {
                genus,
                p,
                n: nf,
                plain_walk: pw / r,
                folded_walk: fw / r,
                folded_fruitless: ff / r,
                plain_total: pt / r,
                folded_total: ft / r,
                all_verified: ok,
            };
            let root = nf.sqrt();
            println!(
                "{:>2} {:>4} {:>9} {:>11.0} {:>11.0} {:>7.3} {:>8.1}% {:>9.3} {:>9.3} {:>8.3} {:>3}",
                genus,
                p,
                n,
                row.plain_walk,
                row.folded_walk,
                row.folded_walk / row.plain_walk,
                100.0 * row.folded_fruitless / row.folded_walk,
                row.plain_walk / root,
                row.folded_walk / root,
                row.folded_total / row.plain_total,
                if ok { "y" } else { "N" }
            );
            io::stdout().flush().ok();
            rows.push(row);
        }
    }

    // The registered verdicts.
    let big: Vec<&Row> = rows.iter().filter(|r| r.n >= 1e5).collect();
    let r1 = median(big.iter().map(|r| r.folded_walk / r.plain_walk).collect());
    let worst_fruitless = rows
        .iter()
        .map(|r| r.folded_fruitless / r.folded_walk)
        .fold(0f64, f64::max);
    let all_ok = rows.iter().all(|r| r.all_verified);
    println!();
    println!(
        "R1 (median folded/plain walk ops, N >= 1e5, {} rows): {:.3}  -> {}",
        big.len(),
        r1,
        if r1 <= 0.80 {
            "HOLDS (<= 0.80)"
        } else {
            "FALSIFIED (> 0.80)"
        }
    );
    println!(
        "R2 (max fruitless share of folded walk ops): {:.1}%  -> {}",
        100.0 * worst_fruitless,
        if worst_fruitless <= 0.10 {
            "HOLDS (<= 10%)"
        } else {
            "FALSIFIED (> 10%)"
        }
    );
    println!(
        "R3 (every run verified): {}",
        if all_ok { "HOLDS" } else { "FALSIFIED" }
    );
    println!("Ideal S_walk: plain sqrt(pi/2) = 1.2533, folded sqrt(pi/4) = 0.8862, ratio 0.7071.");
}
