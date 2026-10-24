//! Round fourteen: do fruitless cycles longer than two cause the folded
//! full-width walk's large-`N` failure?
//!
//! Protocol (registered before this file existed):
//! `research/notes/index-calculus/RESEARCH_HYPERELLIPTIC_IC_RHO.md`,
//! "Round fourteen".  Same 18 instances and 40 seeds as rounds nine to
//! thirteen; three arms: the plain full-width walk, the folded full-width
//! walk with a passive cycle window (round thirteen's walk, instrumented)
//! and with an active one.  Counted operations only, deterministic in the
//! seed; verdicts computed here.
//!
//! ```sh
//! cargo run --release --example hyperelliptic_rho_cycle_window
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

/// Round thirteen's recorded folded walk means, in sweep order
/// (`round13/run1/R13.txt`, column "walk fold").  The passive arm must
/// reproduce them.
const R13_FOLD: [&str; 18] = [
    "36", "35", "60", "80", "104", "325", "96", "74", "76", "358", "965", "2688", "10375", "72",
    "158", "996", "932", "25173",
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
    passive: f64,
    passive_cycle: f64,
    active: f64,
    active_fruitless: f64,
    ok: bool,
}

fn main() {
    println!(
        "Round fourteen. Full-width DP rho: plain, folded with a passive cycle window, folded with an active one ({RHO_RUNS} seeds from {RHO_SEED:#x}; walk ops are means per run)."
    );
    println!(
        "{:>2} {:>4} {:>9} {:>8} {:>9} {:>8} {:>9} {:>9} {:>8} {:>9} {:>8} {:>3}",
        "g",
        "p",
        "N",
        "plain",
        "passive",
        "cycle %",
        "active",
        "act/pl",
        "S plain",
        "S active",
        "fruit %",
        "ok"
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
            let (mut pl, mut pa, mut pc, mut ac, mut af) = (0f64, 0f64, 0f64, 0f64, 0f64);
            let mut ok = true;
            for t in 0..RHO_RUNS {
                let seed = RHO_SEED.wrapping_add(t);
                let run = |v: &RhoVariant| {
                    pollard_rho_jacobian_with(&curve, &d1, &d2, &n, seed, MAX_STEPS, v)
                };
                let a = run(&RhoVariant::DistinguishedPointsFullWidth);
                let b = run(&RhoVariant::DistinguishedPointsNegationFullWidth);
                let c = run(&RhoVariant::DistinguishedPointsNegationFullWidthCycleEscape);
                ok &= [&a, &b, &c].iter().all(|r| r.k.as_ref() == Some(&k));
                pl += a.walk_ops as f64;
                pa += b.walk_ops as f64;
                pc += b.cycle_ops as f64;
                ac += c.walk_ops as f64;
                af += c.fruitless_ops as f64;
            }
            let r = RHO_RUNS as f64;
            let row = Row {
                n: nf,
                plain: pl / r,
                passive: pa / r,
                passive_cycle: pc / pa,
                active: ac / r,
                active_fruitless: af / ac,
                ok,
            };
            if idx < R13_FOLD.len() {
                reproduces &= format!("{:.0}", row.passive) == R13_FOLD[idx];
            }
            idx += 1;
            println!(
                "{:>2} {:>4} {:>9} {:>8.0} {:>9.0} {:>7.1}% {:>9.0} {:>9.3} {:>8.3} {:>9.3} {:>7.1}% {:>3}",
                genus,
                p,
                n,
                row.plain,
                row.passive,
                100.0 * row.passive_cycle,
                row.active,
                row.active / row.plain,
                row.plain / root,
                row.active / root,
                100.0 * row.active_fruitless,
                if ok { "y" } else { "N" }
            );
            io::stdout().flush().ok();
            rows.push(row);
        }
    }

    let big: Vec<&Row> = rows.iter().filter(|r| r.n >= 1e5).collect();
    let mut by_n: Vec<&Row> = rows.iter().collect();
    by_n.sort_by(|a, b| b.n.partial_cmp(&a.n).expect("finite"));
    let top2: Vec<f64> = by_n.iter().take(2).map(|r| r.passive_cycle).collect();
    let c1 = top2.iter().all(|&s| s >= 0.50) && reproduces && idx == R13_FOLD.len();
    let c2 = median(big.iter().map(|r| r.active / r.plain).collect());
    let c3 = median(big.iter().map(|r| r.active / r.n.sqrt()).collect());
    let worst = rows.iter().map(|r| r.active_fruitless).fold(0f64, f64::max);
    let c4 = worst <= 0.10 && rows.iter().all(|r| r.ok);
    let verdict = |b: bool| if b { "HOLDS" } else { "FALSIFIED" };

    println!();
    println!(
        "C14-1 (passive cycle share on the two largest rows >= 50%: {:?}; passive arm reproduces round thirteen: {}) -> {}",
        top2.iter()
            .map(|s| format!("{:.1}%", 100.0 * s))
            .collect::<Vec<_>>(),
        if reproduces { "yes" } else { "NO" },
        verdict(c1)
    );
    println!(
        "C14-2 (active median folded/plain walk ops, N >= 1e5, {} rows): {:.3} (<= 0.80) -> {}",
        big.len(),
        c2,
        verdict(c2 <= 0.80)
    );
    println!(
        "C14-3 (active median folded S_walk, N >= 1e5): {:.3} (<= 1.05) -> {}",
        c3,
        verdict(c3 <= 1.05)
    );
    println!(
        "C14-4 (active fruitless share <= 10% on every row: max {:.1}%; every run verified: {}) -> {}",
        100.0 * worst,
        if rows.iter().all(|r| r.ok) { "yes" } else { "NO" },
        verdict(c4)
    );
    println!(
        "Unpriced native work of the window: up to {} hash comparisons per step.",
        crypto_lib::cryptanalysis::hyperelliptic_ic_bench::CYCLE_WINDOW
    );
    println!("Ideal S_walk: plain 1.2533, folded 0.8862, ratio 0.7071.");
}
