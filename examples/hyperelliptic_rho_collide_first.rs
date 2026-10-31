//! Round fifteen: does trying a short cycle as a collision before escaping
//! it repair the folded walk on small groups?
//!
//! Protocol (registered before this file existed):
//! `research/notes/index-calculus/RESEARCH_HYPERELLIPTIC_IC_RHO.md`,
//! "Round fifteen".  Same 18 instances and 40 seeds as rounds nine to
//! fourteen; three arms: the plain full-width walk, the folded walk with
//! round fourteen's active window, and the folded walk whose window tries
//! the collision first.  Counted operations only, deterministic in the
//! seed; verdicts computed here.
//!
//! ```sh
//! cargo run --release --example hyperelliptic_rho_collide_first
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

/// Round fourteen's recorded active-window walk means, in sweep order
/// (`round14/run1/R14.txt`, column "active").  The second arm must
/// reproduce them.
const R14_ACTIVE: [&str; 18] = [
    "47", "50", "86", "114", "116", "285", "110", "94", "100", "369", "642", "1023", "2556", "93",
    "181", "561", "594", "4164",
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
    plain: f64,
    active: f64,
    first: f64,
    first_fruitless: f64,
    ok: bool,
}

fn main() {
    println!(
        "Round fifteen. Full-width DP rho: plain, folded with the active cycle window, folded with collide-first ({RHO_RUNS} seeds from {RHO_SEED:#x}; walk ops are means per run)."
    );
    println!(
        "{:>2} {:>4} {:>9} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>3}",
        "g", "p", "N", "plain", "active", "first", "act/pl", "1st/pl", "S plain", "fruit %", "ok"
    );
    let mut rows = Vec::new();
    let mut reproduces = true;
    let mut idx = 0usize;
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
            let (mut pl, mut ac, mut fi, mut ff) = (0f64, 0f64, 0f64, 0f64);
            let mut ok = true;
            for t in 0..RHO_RUNS {
                let seed = RHO_SEED.wrapping_add(t);
                let run = |v: &RhoVariant| {
                    pollard_rho_jacobian_with(&curve, &d1, &d2, &n, seed, MAX_STEPS, v)
                };
                let a = run(&RhoVariant::DistinguishedPointsFullWidth);
                let b = run(&RhoVariant::DistinguishedPointsNegationFullWidthCycleEscape);
                let c = run(&RhoVariant::DistinguishedPointsNegationFullWidthCollideFirst);
                ok &= [&a, &b, &c].iter().all(|r| r.k.as_ref() == Some(&k));
                pl += a.walk_ops as f64;
                ac += b.walk_ops as f64;
                fi += c.walk_ops as f64;
                ff += c.fruitless_ops as f64;
            }
            let r = RHO_RUNS as f64;
            let row = Row {
                n: nf,
                plain: pl / r,
                active: ac / r,
                first: fi / r,
                first_fruitless: ff / fi,
                ok,
            };
            if idx < R14_ACTIVE.len() {
                reproduces &= format!("{:.0}", row.active) == R14_ACTIVE[idx];
            }
            idx += 1;
            println!(
                "{:>2} {:>4} {:>9} {:>8.0} {:>8.0} {:>8.0} {:>8.3} {:>8.3} {:>8.3} {:>7.1}% {:>3}",
                genus,
                p,
                n,
                row.plain,
                row.active,
                row.first,
                row.active / row.plain,
                row.first / row.plain,
                row.first / root,
                100.0 * row.first_fruitless,
                if ok { "y" } else { "N" }
            );
            io::stdout().flush().ok();
            rows.push(row);
        }
    }

    let big: Vec<&Row> = rows.iter().filter(|r| r.n >= 1e5).collect();
    let small: Vec<&Row> = rows.iter().filter(|r| r.n <= 6e4).collect();
    let r1 = median(small.iter().map(|r| r.first / r.plain).collect());
    let r2 = median(big.iter().map(|r| r.first / r.plain).collect());
    let r3 = median(big.iter().map(|r| r.first / r.n.sqrt()).collect());
    let worst = rows.iter().map(|r| r.first_fruitless).fold(0f64, f64::max);
    let all_ok = rows.iter().all(|r| r.ok);
    let r4 = worst <= 0.10 && all_ok && reproduces && idx == R14_ACTIVE.len();
    let verdict = |b: bool| if b { "HOLDS" } else { "FALSIFIED" };

    println!();
    println!(
        "R15-1 (collide-first median folded/plain, N <= 6e4, {} rows): {:.3} (<= 0.85) -> {}",
        small.len(),
        r1,
        verdict(r1 <= 0.85)
    );
    println!(
        "R15-2 (collide-first median folded/plain, N >= 1e5, {} rows): {:.3} (<= 0.80) -> {}",
        big.len(),
        r2,
        verdict(r2 <= 0.80)
    );
    println!(
        "R15-3 (collide-first median folded S_walk, N >= 1e5): {:.3} (<= 1.05) -> {}",
        r3,
        verdict(r3 <= 1.05)
    );
    println!(
        "R15-4 (fruitless share <= 10% on every row: max {:.1}%; every run verified: {}; active arm reproduces round fourteen: {}) -> {}",
        100.0 * worst,
        if all_ok { "yes" } else { "NO" },
        if reproduces { "yes" } else { "NO" },
        verdict(r4)
    );
    println!("Ideal S_walk: plain 1.2533, folded 0.8862, ratio 0.7071.");
}
