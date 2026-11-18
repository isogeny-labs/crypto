//! Part A of `research/ic_sketch_mixed_solver_20261115` (PROTOCOL.md):
//! every relation-matrix arm on the same seeded, planted, index-calculus
//! shaped row streams.
//!
//! A row has `m` non-zeros on uniformly drawn factor-base columns plus a
//! non-zero on the target column (the `b` of `[a]G + [b]Q`), and its
//! right-hand side is consistent with a planted solution modulo a 61-bit
//! prime.  Each arm reads rows until the framework's stop rule holds
//! (full-rank arms: `rank == cols` and the target pinned; target-pinned
//! arms: the target pinned), and the record keeps the rows read, the arm's
//! `row_ops`, its work counters and whether the answer equals the planted
//! one.  All counts are exact and deterministic; no clock is read.
//!
//! ```text
//! cargo run --release --example ic_mixed_solver_stream -- OUT.jsonl [SEEDS]
//! ```
use std::io::Write;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crypto_lib::cryptanalysis::ic_boundary::RowStatus;
use crypto_lib::cryptanalysis::ic_framework::linalg::{is_deferred_sparse, matrix_by_name};

const MODULUS: u64 = (1u64 << 61) - 1;

const ARMS: [&str; 11] = [
    "incremental-gauss",
    "incremental-gauss-full-rank",
    "structured-gauss",
    "filter+wiedemann",
    "filter+wiedemann+match",
    "filter+lanczos",
    "filter+lanczos+match",
    "mixed-peel",
    "mixed-peel+bloom",
    "mixed-peel-lazy",
    "incremental-gauss-echelon",
];

fn mulmod(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % MODULUS as u128) as u64
}
fn addmod(a: u64, b: u64) -> u64 {
    ((a as u128 + b as u128) % MODULUS as u128) as u64
}

fn stream(seed: u64, fb: usize, m: usize, len: usize) -> (Vec<u64>, Vec<(Vec<u64>, u64)>) {
    let cols = fb + 1;
    let mut rng = StdRng::seed_from_u64(
        0x1C_5EED ^ seed.wrapping_mul(0x9E37_79B9) ^ (fb as u64) << 40 ^ (m as u64) << 56,
    );
    let x: Vec<u64> = (0..cols).map(|_| rng.gen_range(0..MODULUS)).collect();
    let rows = (0..len)
        .map(|_| {
            let mut row = vec![0u64; cols];
            for _ in 0..m {
                row[rng.gen_range(0..fb)] = rng.gen_range(1..MODULUS);
            }
            row[fb] = rng.gen_range(1..MODULUS);
            let rhs = row
                .iter()
                .zip(&x)
                .fold(0, |a, (&p, &q)| addmod(a, mulmod(p, q)));
            (row, rhs)
        })
        .collect();
    (x, rows)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "stream.jsonl".into());
    let seeds: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(16);
    let mut f = std::fs::File::create(&out).expect("create output");
    for &fb in &[32usize, 64, 128, 256, 512] {
        for &m in &[2usize, 3] {
            for seed in 0..seeds {
                let cols = fb + 1;
                let len = 40 * cols;
                let (x, rows) = stream(seed, fb, m, len);
                for arm in ARMS {
                    let full_rank = is_deferred_sparse(arm) || arm == "incremental-gauss-full-rank";
                    let mut mat = matrix_by_name(arm, cols, MODULUS).expect("arm");
                    let mut read = 0usize;
                    let mut answer = None;
                    let mut inconsistent = false;
                    for (row, rhs) in &rows {
                        read += 1;
                        if mat.add_row(row.clone(), *rhs) == RowStatus::Inconsistent {
                            inconsistent = true;
                            break;
                        }
                        let pin = mat.pinned(fb);
                        if pin.is_some() && (!full_rank || mat.rank() == cols) {
                            answer = pin;
                            break;
                        }
                    }
                    let (work, unit) = mat.work();
                    let counters = mat.work_counters();
                    let rec = serde_json::json!({
                        "fb": fb, "m": m, "seed": seed, "cols": cols, "arm": arm,
                        "rule": if full_rank { "full-rank" } else { "target-pinned" },
                        "rows_read": read, "work": work, "unit": unit,
                        "rank": mat.rank(), "dependent": mat.dependent(),
                        "solved": answer.is_some(), "correct": answer == Some(x[fb]),
                        "inconsistent": inconsistent, "counters": counters,
                    });
                    writeln!(f, "{rec}").expect("write");
                }
            }
            eprintln!("fb={fb} m={m} done");
        }
    }
}
