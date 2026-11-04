//! Stage diagnostic for the relation-matrix solvers of `ic_framework::linalg`: planted
//! sparse systems with the shape of an index-calculus relation matrix, so the solvers can
//! be compared on the linear-algebra stage alone, at row weights and sizes no curve
//! workload reaches cheaply.
//!
//! Every row has `w` base columns (distinct, random non-zero coefficients) and a non-zero
//! entry in the target column (the last one), as the ladder's relations do (`row[d_col] =
//! −h·b`); the right-hand side is the planted solution's.  Each arm is fed the same rows
//! in the same order until it determines the target (target-pinned arms) or the whole
//! system (full-rank arms), and the recovered value is checked against the planted one.
//!
//! One JSON line per (weight, size, seed, arm): rows used, `row_ops` in the dense
//! matrices' unit, wall time, and whether the value was right.  The unit is the counted
//! `row_ops`; the wall time is a practicality note.  This is a stage diagnostic on
//! synthetic rows: it says nothing about end-to-end `S` and nothing about real relation
//! structure (cycles of two-summand rows, repeated columns).
//!
//! ```sh
//! cargo run --release --example sparse_la_stage -- --weights 2,3,4 --sizes 256,512 --seeds 2 \
//!     --arms structured-gauss,pin-wiedemann
//! ```
use std::time::Instant;

use crypto_lib::cryptanalysis::ic_boundary::{RelationSolver, RowStatus};
use crypto_lib::cryptanalysis::ic_framework::linalg::matrix_by_name;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

const MODULUS: u64 = (1u64 << 61) - 1;

fn mulmod(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % MODULUS as u128) as u64
}

fn addmod(a: u64, b: u64) -> u64 {
    ((a as u128 + b as u128) % MODULUS as u128) as u64
}

fn arg<'a>(args: &'a [String], name: &str, default: &'a str) -> &'a str {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
        .unwrap_or(default)
}

fn list(s: &str) -> Vec<String> {
    s.split(',')
        .filter(|x| !x.is_empty())
        .map(str::to_string)
        .collect()
}

/// Arms that stop at the target pin, as the default completion rule does.
fn stops_at_pin(arm: &str) -> bool {
    matches!(
        arm,
        "structured-gauss" | "incremental-gauss" | "pin-wiedemann" | "pin-wiedemann-every"
    )
}

/// One row: `w` distinct base columns below `base`, and the target column.
fn make_row(rng: &mut StdRng, base: usize, w: usize, x: &[u64]) -> (Vec<u64>, u64) {
    let mut row = vec![0u64; base + 1];
    let mut placed = 0;
    while placed < w {
        let c = rng.gen_range(0..base);
        if row[c] == 0 {
            row[c] = rng.gen_range(1..MODULUS);
            placed += 1;
        }
    }
    row[base] = rng.gen_range(1..MODULUS);
    let rhs = row
        .iter()
        .zip(x)
        .fold(0u64, |acc, (&a, &b)| addmod(acc, mulmod(a, b)));
    (row, rhs)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let weights: Vec<usize> = list(arg(&args, "--weights", "2,3,4"))
        .iter()
        .map(|v| v.parse().expect("weight"))
        .collect();
    let sizes: Vec<usize> = list(arg(&args, "--sizes", "128,256,512"))
        .iter()
        .map(|v| v.parse().expect("size"))
        .collect();
    let seeds: u64 = arg(&args, "--seeds", "2").parse().expect("seeds");
    let arms = list(arg(
        &args,
        "--arms",
        "structured-gauss,incremental-gauss,pin-wiedemann,pin-wiedemann-every",
    ));
    let max_rows_factor: usize = arg(&args, "--max-rows-factor", "4")
        .parse()
        .expect("factor");
    for &w in &weights {
        for &base in &sizes {
            let cols = base + 1;
            for seed in 0..seeds {
                let mut rng = StdRng::seed_from_u64(
                    0x5354_4147_4500 ^ (w as u64) << 40 ^ (base as u64) << 16 ^ seed,
                );
                let x: Vec<u64> = (0..cols).map(|_| rng.gen_range(0..MODULUS)).collect();
                let rows: Vec<(Vec<u64>, u64)> = (0..max_rows_factor * cols)
                    .map(|_| make_row(&mut rng, base, w, &x))
                    .collect();
                for arm in &arms {
                    let full_rank_name = arm == "igfr";
                    let name = if full_rank_name {
                        "incremental-gauss"
                    } else {
                        arm.as_str()
                    };
                    let at_pin = stops_at_pin(arm);
                    let mut m = matrix_by_name(name, cols, MODULUS).expect("matrix");
                    let started = Instant::now();
                    let mut used = 0usize;
                    let mut value = None;
                    for (row, rhs) in &rows {
                        used += 1;
                        if matches!(m.add_row(row.clone(), *rhs), RowStatus::Inconsistent) {
                            break;
                        }
                        let pinned = m.pinned(base);
                        if pinned.is_some() && (at_pin || m.rank() == cols) {
                            value = pinned;
                            break;
                        }
                    }
                    let wall_ns = started.elapsed().as_nanos() as u64;
                    let (row_ops, unit) = m.work();
                    println!(
                        "{}",
                        serde_json::json!({
                            "schema": "sparse_la_stage/v1",
                            "weight": w,
                            "columns": cols,
                            "seed": seed,
                            "arm": arm,
                            "rows_used": used,
                            "solved": value.is_some(),
                            "correct": value == Some(x[base]),
                            "row_ops": row_ops,
                            "unit": unit,
                            "wall_ns": wall_ns,
                        })
                    );
                }
            }
        }
    }
}
