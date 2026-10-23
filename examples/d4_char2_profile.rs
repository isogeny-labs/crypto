//! Per-component mean cost of one `gaudry-d4-bin` trial (Frobenius form) on
//! random targets, baseline roots against `frob_roots`, same targets in the
//! same process (lever study `research/ic_d4_char2_lever_20261023`).  The two
//! paths must return the same rational solutions and decompositions; wall
//! time is descriptive (shared VM).
//!
//! ```sh
//! RAYON_NUM_THREADS=1 cargo run --release --example d4_char2_profile -- --m 12 --seed 1 --trials 300
//! ```
use std::env;
use std::time::Instant;

use crypto_lib::cryptanalysis::gaudry_quartic_bin::{
    find_bin_curve, solve_d4_bin, BinOptions, BinTrace, SymmetrisedBin,
};
use rand::rngs::StdRng;
use rand::SeedableRng;

const NAMES: [&str; 10] = [
    "weil", "triang", "echelon", "nf", "charpoly", "roots", "eigvec", "verify", "split", "TOTAL",
];

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let get = |k: &str, d: u64| {
        args.iter()
            .position(|a| a == k)
            .map_or(d, |i| args[i + 1].parse().expect(k))
    };
    let (m, seed, trials) = (get("--m", 12) as u32, get("--seed", 1), get("--trials", 300));
    let c = find_bin_curve(m, seed, 16).expect("curve");
    let curve = c.curve;
    let mut rng = StdRng::seed_from_u64(seed ^ 0x9e);
    let pre = SymmetrisedBin::precompute(&curve, &mut rng, 8).expect("pre");
    let mk = |frob_roots| BinOptions {
        weighted: true,
        frobenius: true,
        sparsest: true,
        replay: true,
        frob_roots,
    };
    let mut traces = [BinTrace::default(), BinTrace::default()];
    let mut tot = [[0f64; 10]; 2];
    let mut wall = [0f64; 2];
    let (mut rel, mut sols, mut zero_sol, mut disagree) = (0u64, 0u64, 0u64, 0u64);
    let (mut dim, mut chains) = (0u64, 0u64);
    let mut rrng = StdRng::seed_from_u64(77);
    for _ in 0..trials {
        let r = loop {
            let x = curve.f.random(&mut rrng);
            if let Some(p) = curve.lift_x(&x, &mut rrng) {
                break p;
            }
        };
        let mut res = Vec::new();
        for a in 0..2 {
            let mut srng = StdRng::seed_from_u64(5);
            let t = Instant::now();
            let out = solve_d4_bin(&curve, &pre, &r, mk(a == 1), 11, &mut srng, Some(&mut traces[a]));
            wall[a] += t.elapsed().as_secs_f64();
            res.push(out);
        }
        let (Ok(x), Ok(y)) = (&res[0], &res[1]) else {
            continue;
        };
        if x.decompositions != y.decompositions || x.rational_solutions != y.rational_solutions {
            disagree += 1;
        }
        for (a, s) in [x, y].into_iter().enumerate() {
            let b = &s.solve;
            for (i, v) in [
                b.weil_muls, b.triangularise_muls, b.echelon_muls, b.nf_muls, b.charpoly_muls,
                b.roots_muls, b.eigenvector_muls, b.verify_muls, b.split_muls, b.total_muls,
            ]
            .iter()
            .enumerate()
            {
                tot[a][i] += *v as f64;
            }
        }
        dim += x.solve.dim as u64;
        chains += x.solve.chains as u64;
        sols += x.rational_solutions as u64;
        zero_sol += u64::from(x.rational_solutions == 0);
        rel += u64::from(x.witness.is_some());
    }
    let n = trials as f64;
    println!("m {m} seed {seed} trials {trials}: dim {:.1}, chains {:.1}", dim as f64 / n, chains as f64 / n);
    println!("{:10} {:>12} {:>12}", "component", "baseline", "frob_roots");
    for (i, nm) in NAMES.iter().enumerate() {
        println!("{nm:10} {:12.0} {:12.0}", tot[0][i] / n, tot[1][i] / n);
    }
    println!(
        "total ratio frob/base {:.4}; wall per trial {:.3} ms vs {:.3} ms (ratio {:.4})",
        tot[1][9] / tot[0][9],
        1e3 * wall[0] / n,
        1e3 * wall[1] / n,
        wall[1] / wall[0]
    );
    println!(
        "rational sols/trial {:.3}; trials with 0 sols {zero_sol}; relations {rel} ({:.2}%); disagreements {disagree}",
        sols as f64 / n,
        100.0 * rel as f64 / n
    );
}
