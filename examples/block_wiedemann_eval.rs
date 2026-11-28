//! Block Wiedemann block-size schedule on one frozen sparse matrix
//! (catalog #4917 §8 item 6): for each block size `b` (`m = n = b`) record
//! products, projection and generator multiplications, generator degree,
//! seeds needed and wall time. One JSON object per line.
//!
//! ```text
//! cargo run --release --example block_wiedemann_eval -- \
//!     [--dim 2000] [--weight 6] [--dead 4] [--seed 1] [--blocks 1,2,4,8,16] \
//!     [--threads T] \
//!     [--input matrix.json]
//! ```
//!
//! `--input` reads a `SparseMatrix` JSON (`{"n":…,"rows":[[[col,coef],…],…]}`);
//! otherwise rows get `--weight` uniform random entries and the first
//! `--dead` rows are zero (so the matrix is singular). Modulus `2^61 − 1`.

use crypto_lib::cryptanalysis::large_prime_graph::block_wiedemann::{
    kernel_vector, kernel_vector_parallel, BwParams, SparseMatrix,
};
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let get = |flag: &str, default: &str| -> String {
        args.iter()
            .position(|a| a == flag)
            .and_then(|i| args.get(i + 1))
            .cloned()
            .unwrap_or_else(|| default.to_string())
    };
    let p = (1u64 << 61) - 1;
    let a = match args
        .iter()
        .position(|a| a == "--input")
        .and_then(|i| args.get(i + 1))
    {
        Some(path) => {
            serde_json::from_str::<SparseMatrix>(&std::fs::read_to_string(path).expect("--input"))
                .expect("SparseMatrix JSON")
        }
        None => {
            let dim: usize = get("--dim", "2000").parse().unwrap();
            let weight: usize = get("--weight", "6").parse().unwrap();
            let dead: usize = get("--dead", "4").parse().unwrap();
            let mut s: u64 = get("--seed", "1").parse::<u64>().unwrap() | 1;
            let mut next = move || {
                s ^= s << 13;
                s ^= s >> 7;
                s ^= s << 17;
                s
            };
            let rows = (0..dim)
                .map(|i| {
                    if i < dead {
                        Vec::new()
                    } else {
                        (0..weight)
                            .map(|_| ((next() % dim as u64) as usize, 1 + next() % (p - 1)))
                            .collect()
                    }
                })
                .collect();
            SparseMatrix::new(dim, rows, p).unwrap()
        }
    };
    // `--threads 0` (default) runs the serial path; `--threads t` runs
    // `kernel_vector_parallel` on a dedicated pool of `t` threads.
    let threads: usize = get("--threads", "0").parse().unwrap();
    let pool = (threads > 0).then(|| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap()
    });
    for b in get("--blocks", "1,2,4,8,16")
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
    {
        let t = Instant::now();
        let mut out = None;
        let mut seeds = 0;
        for seed in 0..8 {
            seeds += 1;
            let params = BwParams {
                m: b,
                n: b,
                seed,
                extra: 8,
            };
            let attempt = match &pool {
                Some(pool) => pool.install(|| kernel_vector_parallel(&a, p, params)),
                None => kernel_vector(&a, p, params),
            };
            if let Ok((w, stats)) = attempt {
                out = Some((w, stats));
                break;
            }
        }
        let ns = t.elapsed().as_nanos();
        match out {
            Some((w, s)) => {
                assert!(a.mul_vec(&w, p).iter().all(|&x| x == 0));
                println!(
                    "{}",
                    serde_json::json!({
                        "b": b, "dimension": s.dimension, "nonzeros": s.nonzeros,
                        "spmv": s.spmv, "sequence_spmv": s.sequence_spmv,
                        "projection_mults": s.projection_mults, "generator_mults": s.generator_mults,
                        "sequence_length": s.sequence_length, "generator_degree": s.generator_degree,
                        "candidates_tried": s.candidates_tried, "kernel_power": s.kernel_power,
                        "seeds": seeds, "threads": threads, "wall_ns": ns, "verified": true
                    })
                );
            }
            None => println!(
                "{}",
                serde_json::json!({"b": b, "seeds": seeds, "threads": threads, "wall_ns": ns, "verified": false})
            ),
        }
    }
}
