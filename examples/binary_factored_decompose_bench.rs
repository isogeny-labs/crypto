//! **End to end: `semaev_decomp::decompose` against the factored oracle.**
//!
//! The whole `m = 3` decision of a target `x_R` over `V = {x < 2^l}` on the
//! Koblitz curve `y² + xy = x³ + x² + 1` over `F_{2ⁿ}`:
//!
//! * **base** — `decompose`: build each pair's quartic, invert its leading
//!   coefficient (batched a row), `gcd(f, L_V mod f)`.
//! * **stream** — `FactoredOracle::decompose`: `S₄ = S₃(u)·S₃(u')`, one
//!   Artin–Schreier step for `u` and one per rational `u` for `X`.
//! * **table** — the same with every pair's `u` precomputed (it does not
//!   depend on the target); table build time and size reported apart.
//!
//! Each target's answer (decomposes or not) is compared across the three,
//! and every witness is checked with `eval_f3`.  Targets are random, so
//! almost all do not decompose: the case an attack spends its time on.
//!
//! ```bash
//! cargo run --release --example binary_factored_decompose_bench
//! ```
use crypto_lib::cryptanalysis::binary_radical_root::{FactoredOracle, FactoredWork};
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible;
use crypto_lib::cryptanalysis::semaev_decomp::{decompose, eval_f3, Gf2};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::time::Instant;

fn main() {
    let ns: Vec<u32> = vec![23, 31, 41, 53, 61];
    let ls: Vec<(u32, usize)> = vec![(6, 3000), (8, 300), (10, 24), (12, 3)];
    println!(
        "{:>3} {:>3} {:>6} {:>5} | {:>11} {:>11} {:>11} | {:>7} {:>7} | {:>8} {:>7} | {:>6} {:>5}",
        "n",
        "l",
        "tgts",
        "hits",
        "base µs/t",
        "stream µs/t",
        "table µs/t",
        "stream",
        "table",
        "build ms",
        "tbl MB",
        "AS/pr",
        "agree"
    );
    for &n in &ns {
        let gf = Gf2::new(&find_irreducible(n).expect("irreducible"));
        for &(l, targets) in &ls {
            let stream = FactoredOracle::new(l, &gf).expect("odd n");
            let t0 = Instant::now();
            let table = FactoredOracle::new(l, &gf).unwrap().with_pair_table(&gf);
            let build = t0.elapsed().as_secs_f64();
            let span = 1u64 << l;
            let table_mb = (span * span / 2) as f64 * 16.0 / 2.0 / 1e6; // ≈ half the pairs, two u each

            let mut rng = StdRng::seed_from_u64(0xdec0 ^ ((n as u64) << 8) ^ l as u64);
            let xrs: Vec<u64> = (0..targets)
                .map(|_| rng.gen::<u64>() & ((1u64 << n) - 1))
                .collect();

            let t = Instant::now();
            let want: Vec<Option<[u64; 3]>> = xrs.iter().map(|&x| decompose(x, l, &gf)).collect();
            let tb = t.elapsed().as_secs_f64();

            let mut ws = FactoredWork::default();
            let t = Instant::now();
            let got_s: Vec<_> = xrs
                .iter()
                .map(|&x| stream.decompose(x, &gf, &mut ws))
                .collect();
            let ts = t.elapsed().as_secs_f64();

            let mut wt = FactoredWork::default();
            let t = Instant::now();
            let got_t: Vec<_> = xrs
                .iter()
                .map(|&x| table.decompose(x, &gf, &mut wt))
                .collect();
            let tt = t.elapsed().as_secs_f64();

            let valid = |g: &Option<[u64; 3]>, x: u64| match g {
                None => true,
                Some([a, b, c]) => {
                    a >> l == 0 && b >> l == 0 && c >> l == 0 && eval_f3(*a, *b, *c, x, &gf) == 0
                }
            };
            let mut agree = true;
            for i in 0..targets {
                let ok = got_s[i].is_some() == want[i].is_some()
                    && got_t[i].is_some() == want[i].is_some()
                    && valid(&want[i], xrs[i])
                    && valid(&got_s[i], xrs[i])
                    && valid(&got_t[i], xrs[i]);
                if !ok {
                    agree = false;
                    eprintln!(
                        "MISMATCH n={n} l={l} xr={:#x} base={:?} stream={:?} table={:?}",
                        xrs[i], want[i], got_s[i], got_t[i]
                    );
                }
            }
            let hits = want.iter().filter(|w| w.is_some()).count();
            let per = |s: f64| 1e6 * s / targets as f64;
            println!(
                "{n:>3} {l:>3} {targets:>6} {hits:>5} | {:>11.1} {:>11.1} {:>11.1} | {:>6.2}x {:>6.2}x | {:>8.1} {:>7.1} | {:>6.3} {:>5}",
                per(tb),
                per(ts),
                per(tt),
                tb / ts,
                tb / tt,
                1e3 * build,
                table_mb,
                ws.as_solves as f64 / ws.pairs.max(1) as f64,
                if agree { "yes" } else { "NO" }
            );
        }
    }
}
