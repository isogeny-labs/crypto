//! Round ten: does the rho reference's walk hash cause its large-`N`
//! degradation?
//!
//! Protocol (registered before this file existed):
//! `research/notes/index-calculus/RESEARCH_HYPERELLIPTIC_IC_RHO.md`,
//! "Round ten".  Four arms on round seven's instances and seeds: plain and
//! `±`-folded DP rho, each with the old `divisor_hash` and with it passed
//! through SplitMix64's finaliser.  Counted operations only, deterministic
//! in the seed; the verdicts are computed here, natively.
//!
//! ```sh
//! cargo run --release --example hyperelliptic_rho_hash_mixing
//! ```

use std::io::{self, Write};

use num_bigint::BigUint;

use crypto_lib::cryptanalysis::hyperelliptic_ic_bench::{
    pollard_rho_jacobian_with, thread_instance, RhoVariant,
};

const SWEEP: [(u32, &[u64]); 3] = [
    (2, &[41, 61, 101, 151, 211, 251]),
    (3, &[23, 31, 41, 61, 101, 151, 211]),
    (4, &[17, 23, 31, 41, 61]),
];
const RHO_SEED: u64 = 0xC0FFEE;
const RHO_RUNS: u64 = 40;
const MAX_STEPS: usize = 200_000_000;

const ARMS: [(&str, RhoVariant); 4] = [
    ("plain", RhoVariant::DistinguishedPoints),
    ("fold", RhoVariant::DistinguishedPointsNegation),
    ("plain+mix", RhoVariant::DistinguishedPointsMixed),
    ("fold+mix", RhoVariant::DistinguishedPointsNegationMixed),
];

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
    let m = v.len() / 2;
    if v.len() % 2 == 1 {
        v[m]
    } else {
        (v[m - 1] + v[m]) / 2.0
    }
}

struct Row {
    n: f64,
    s_walk: [f64; 4],
    fruitless: [f64; 4],
    verified: bool,
}

fn main() {
    println!(
        "Round ten. DP rho walk cost by arm (S_walk = mean walk group ops / sqrt(N), {RHO_RUNS} seeds from {RHO_SEED:#x})."
    );
    println!(
        "{:>2} {:>4} {:>9} {:>7} {:>7} {:>9} {:>8} {:>9} {:>10} {:>3}",
        "g", "p", "N", "plain", "fold", "plain+mix", "fold+mix", "fold/pl", "mix f/pl", "ok"
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
            let mut s_walk = [0f64; 4];
            let mut fruitless = [0f64; 4];
            let mut verified = true;
            for (i, (_, variant)) in ARMS.iter().enumerate() {
                let (mut w, mut f) = (0f64, 0f64);
                for t in 0..RHO_RUNS {
                    let r = pollard_rho_jacobian_with(
                        &curve,
                        &d1,
                        &d2,
                        &n,
                        RHO_SEED.wrapping_add(t),
                        MAX_STEPS,
                        variant,
                    );
                    verified &= r.k.as_ref() == Some(&k);
                    w += r.walk_ops as f64;
                    f += r.fruitless_ops as f64;
                }
                s_walk[i] = w / RHO_RUNS as f64 / root;
                fruitless[i] = if w > 0.0 { f / w } else { 0.0 };
            }
            println!(
                "{:>2} {:>4} {:>9} {:>7.3} {:>7.3} {:>9.3} {:>8.3} {:>9.3} {:>10.3} {:>3}",
                genus,
                p,
                n,
                s_walk[0],
                s_walk[1],
                s_walk[2],
                s_walk[3],
                s_walk[1] / s_walk[0],
                s_walk[3] / s_walk[2],
                if verified { "y" } else { "N" }
            );
            io::stdout().flush().ok();
            rows.push(Row {
                n: nf,
                s_walk,
                fruitless,
                verified,
            });
        }
    }

    let big: Vec<&Row> = rows.iter().filter(|r| r.n >= 1e5).collect();
    let small: Vec<&Row> = rows.iter().filter(|r| r.n <= 6e4).collect();
    let mut by_n: Vec<&Row> = rows.iter().collect();
    by_n.sort_by(|a, b| b.n.partial_cmp(&a.n).expect("finite"));
    let largest_two: Vec<f64> = by_n.iter().take(2).map(|r| r.s_walk[2]).collect();

    let h1_med = median(big.iter().map(|r| r.s_walk[2]).collect());
    let h1 = h1_med <= 1.45 && largest_two.iter().all(|&s| s <= 1.55);
    let h2 = median(big.iter().map(|r| r.s_walk[3] / r.s_walk[2]).collect());
    let h3 = (median(small.iter().map(|r| r.s_walk[2]).collect())
        - median(small.iter().map(|r| r.s_walk[0]).collect()))
    .abs();
    let h4 = rows.iter().all(|r| r.verified);
    let worst_fruitless = rows.iter().map(|r| r.fruitless[3]).fold(0f64, f64::max);

    println!();
    println!(
        "H1 (mixed plain S_walk, N >= 1e5, {} rows): median {:.3} (<= 1.45), two largest {:?} (<= 1.55) -> {}",
        big.len(),
        h1_med,
        largest_two.iter().map(|s| format!("{s:.3}")).collect::<Vec<_>>(),
        if h1 { "HOLDS" } else { "FALSIFIED" }
    );
    println!(
        "H2 (mixed fold/plain walk ops, N >= 1e5): median {:.3} -> {}",
        h2,
        if h2 <= 0.80 {
            "HOLDS (<= 0.80)"
        } else {
            "FALSIFIED (> 0.80)"
        }
    );
    println!(
        "H3 (control, N <= 6e4, {} rows): |median mixed - median old| plain S_walk = {:.3} -> {}",
        small.len(),
        h3,
        if h3 < 0.15 {
            "HOLDS (< 0.15)"
        } else {
            "FALSIFIED (>= 0.15)"
        }
    );
    println!(
        "H4 (every run verified): {}",
        if h4 { "HOLDS" } else { "FALSIFIED" }
    );
    println!(
        "Max fruitless share, fold+mix: {:.1}%",
        100.0 * worst_fruitless
    );
    println!("Ideal S_walk: plain 1.2533, folded 0.8862.");
}
