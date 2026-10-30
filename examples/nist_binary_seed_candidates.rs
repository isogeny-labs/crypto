//! Enumerate the X9.62 candidates of one 1998 NIST binary-curve seed run,
//! for the census in `research/nist_binary_skipped_20261020/`.
//!
//! ```bash
//! cargo run --release --example nist_binary_seed_candidates -- --run 446413329 --blocks 155
//! cargo run --release --example nist_binary_seed_candidates -- --published
//! ```
//!
//! For each candidate `i = 1..=blocks` (five `lrand48()` words after
//! `srand48(run)`, as [`binary_seed`] reads them) it prints
//! `i seed_hex c_hex`, where `c` is the X9.62 A.3.3.2 value of the seed
//! over the run's field: `b` in the Gaussian normal basis, most significant
//! bit first.  Candidates with `c = 0` are dropped (`b = 0` is singular).
//! `--published` prints `name m seed_hex c_hex` for B-163 … B-571 instead,
//! the calibration input of `census.gp`.

use std::process::exit;

use crypto_lib::cryptanalysis::nist_seed_provenance::{
    binary_seed, seed_bytes, x962_binary_candidate_b, BINARY_CURVES, BINARY_SEEDS,
};

fn arg(name: &str) -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
}

fn hex(seed: &[u8; 20]) -> String {
    seed.iter().map(|b| format!("{b:02x}")).collect()
}

fn main() {
    if std::env::args().any(|a| a == "--published") {
        for c in BINARY_CURVES.iter() {
            let nb = x962_binary_candidate_b(&seed_bytes(c.seed_hex), c.m);
            println!("{} {} {} {:x}", c.name, c.m, c.seed_hex, nb);
        }
        return;
    }
    let (Some(run), Some(blocks)) = (
        arg("--run").and_then(|s| s.parse::<u32>().ok()),
        arg("--blocks").and_then(|s| s.parse::<u32>().ok()),
    ) else {
        eprintln!(
            "usage: nist_binary_seed_candidates --run <srand48 value> --blocks <n> | --published"
        );
        exit(2);
    };
    let Some(s) = BINARY_SEEDS.iter().find(|s| s.srand48 == run) else {
        eprintln!("srand48({run}) is not one of the published binary runs");
        exit(2);
    };
    let m = BINARY_CURVES
        .iter()
        .find(|c| c.name == s.name)
        .expect("curve")
        .m;
    for i in 1..=blocks {
        let seed = binary_seed(run, i);
        let c = x962_binary_candidate_b(&seed, m);
        if c.bits() == 0 {
            continue;
        }
        println!("{i} {} {c:x}", hex(&seed));
    }
}
