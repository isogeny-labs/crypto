//! Compare partial-relation retention policies under a row budget, scored by
//! exact GainDsu useful rank (`large_prime_graph::retention`).
//!
//! ```text
//! # F_p* toy pools (workspace#144 T3 sizes): p = nextprime(2^31 + 11e6·i)
//! cargo run --release --example lp_retention_eval -- --source fp --p-index 0 \
//!     --repeats 5 --draws 150000 --smooth 4096 --lp 65536 \
//!     --budgets 0.25,0.1,0.05,0.02,0.01 [--relabel] [--dump pool.json] --out fp0.json
//! # the module's synthetic stream (LP arity weights 0:1:2), modulus 2^61−1
//! cargo run --release --example lp_retention_eval -- --source synthetic \
//!     --rows 40000 --arity 0,6,12 [--unit] [--relabel] --repeats 5 --out syn.json
//! # a captured stream, e.g. from `lp_graph_collect --relations out.jsonl`
//! cargo run --release --example lp_retention_eval -- --source relations \
//!     --relations out.jsonl --modulus 65557 --repeats 5 --out real.json
//! ```
//!
//! The pool is the rows of canonical LP arity 1 or 2 (`split_by_arity`):
//! complete relations are counted and kept out, rows of arity above 2 are
//! counted and dropped (the exact DSU oracle covers arity ≤ 2).
//!
//! `--relabel` replaces every LP identity by a fixed bijective 64-bit mix of
//! itself before retention. The graph, and so every rank, is unchanged;
//! only the order that `smallest_lp` reads is destroyed. It stands for streams
//! whose LP identities carry no size information (curve point labels).
//!
//! Repeat `r` uses seed `1000·i + r` (`i` = `--p-index`, else 0) for
//! collection and for the seeded policies.

use crypto_lib::cryptanalysis::large_prime_filter::PartialRelation;
use crypto_lib::cryptanalysis::large_prime_graph::eval::{synthetic_stream, SyntheticParams};
use crypto_lib::cryptanalysis::large_prime_graph::retention::{
    compare_policies, fp_toy, split_by_arity, RetentionOutcome, RetentionPolicy,
};
use serde::Serialize;
use std::io::BufRead;

#[derive(Serialize)]
struct Repeat {
    seed: u64,
    pool: usize,
    complete: usize,
    dropped_wide: usize,
    replayed: Option<usize>,
    /// Useful rank of the whole pool (no budget).
    full: RetentionOutcome,
    /// One entry per budget fraction, each with every policy.
    budgets: Vec<BudgetRow>,
}

#[derive(Serialize)]
struct BudgetRow {
    fraction: f64,
    outcomes: Vec<RetentionOutcome>,
}

#[derive(Serialize)]
struct Report {
    schema: &'static str,
    source: String,
    modulus: u64,
    relabel: bool,
    params: serde_json::Value,
    repeats: Vec<Repeat>,
}

/// Bijective on u64 (each step is invertible), so distinct LP identities stay distinct.
fn mix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn relabel(pool: &mut [PartialRelation]) {
    for r in pool {
        for t in &mut r.large {
            t.0 = mix64(t.0);
        }
    }
}

fn evaluate(
    pool: &[PartialRelation],
    modulus: u64,
    fractions: &[f64],
    seed: u64,
) -> (RetentionOutcome, Vec<BudgetRow>) {
    let full = compare_policies(pool, modulus, pool.len(), &[RetentionPolicy::Random], seed)
        .expect("full pool")
        .remove(0);
    let budgets = fractions
        .iter()
        .map(|&fraction| BudgetRow {
            fraction,
            outcomes: compare_policies(
                pool,
                modulus,
                (pool.len() as f64 * fraction) as usize,
                &RetentionPolicy::ALL,
                seed,
            )
            .expect("compare"),
        })
        .collect();
    (full, budgets)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let get = |flag: &str, default: &str| -> String {
        args.iter()
            .position(|a| a == flag)
            .and_then(|i| args.get(i + 1))
            .cloned()
            .unwrap_or_else(|| default.to_string())
    };
    let source = get("--source", "fp");
    let repeats: u64 = get("--repeats", "5").parse().expect("--repeats");
    let fractions: Vec<f64> = get("--budgets", "0.25,0.1,0.05,0.02,0.01")
        .split(',')
        .map(|x| x.parse().expect("--budgets"))
        .collect();
    let p_index: u64 = get("--p-index", "0").parse().expect("--p-index");
    let do_relabel = args.iter().any(|a| a == "--relabel");
    let base = 1000 * p_index;

    // Each source yields (pool, complete, wide, replayed) for repeat r.
    let modulus: u64;
    let params: serde_json::Value;
    let mut dumped = Vec::new();
    let dump = get("--dump", "");
    let mut batches: Vec<(u64, Vec<PartialRelation>, usize, usize, Option<usize>)> = Vec::new();
    match source.as_str() {
        "fp" => {
            let p = match get("--p", "").as_str() {
                "" => fp_toy::Instance::next_prime((1u64 << 31) + 11_000_000 * p_index),
                s => s.parse().expect("--p"),
            };
            let inst = fp_toy::Instance::new(p, get("--g-min", "1001").parse().expect("--g-min"));
            let smooth: u64 = get("--smooth", "4096").parse().unwrap();
            let lp: u64 = get("--lp", "65536").parse().unwrap();
            let draws: usize = get("--draws", "150000").parse().unwrap();
            modulus = inst.ell;
            params = serde_json::json!({"instance": inst, "smooth_bound": smooth, "lp_bound": lp, "draws": draws});
            for r in 0..repeats {
                let seed = base + r;
                let col = fp_toy::collect(&inst, smooth, lp, draws, seed);
                let replayed = fp_toy::replay(&col).unwrap_or_else(|i| panic!("replay failed at row {i}"));
                let (pool, _, wide) = split_by_arity(&col.partials, modulus).expect("split");
                batches.push((seed, pool, col.complete, wide, Some(replayed)));
                if !dump.is_empty() {
                    dumped.push(col);
                }
            }
        }
        "synthetic" => {
            modulus = get("--modulus", &((1u64 << 61) - 1).to_string()).parse().unwrap();
            let arity: Vec<u64> = get("--arity", "0,6,12").split(',').map(|x| x.parse().unwrap()).collect();
            let rows: usize = get("--rows", "40000").parse().unwrap();
            let lp_columns: u64 = get("--lp-columns", &(2 * rows as u64 / 3).to_string()).parse().unwrap();
            let unit = args.iter().any(|a| a == "--unit");
            params = serde_json::json!({"rows": rows, "lp_columns": lp_columns, "arity_weights": arity, "unit": unit});
            for r in 0..repeats {
                let seed = base + r + 7;
                let sp = SyntheticParams {
                    seed,
                    rows,
                    lp_columns,
                    arity_weights: arity.clone(),
                    unit_coefficients: unit,
                    ..SyntheticParams::default()
                };
                let (pool, complete, wide) = split_by_arity(&synthetic_stream(&sp, modulus), modulus).expect("split");
                batches.push((seed, pool, complete.len(), wide, None));
            }
        }
        "relations" => {
            modulus = get("--modulus", "0").parse().expect("--modulus (the relation prime ℓ)");
            let path = get("--relations", "");
            let f = std::io::BufReader::new(std::fs::File::open(&path).expect("--relations"));
            let rels: Vec<PartialRelation> = f
                .lines()
                .map(|l| serde_json::from_str(&l.unwrap()).expect("PartialRelation JSON line"))
                .collect();
            params = serde_json::json!({"relations": path, "rows": rels.len()});
            let (pool, complete, wide) = split_by_arity(&rels, modulus).expect("split");
            // One fixed stream: repeats vary only the seeded policies.
            for r in 0..repeats {
                batches.push((base + r, pool.clone(), complete.len(), wide, None));
            }
        }
        other => panic!("--source must be fp, synthetic or relations, not {other}"),
    }
    if !dump.is_empty() {
        std::fs::write(&dump, serde_json::to_string(&dumped).unwrap()).expect("--dump");
    }

    let mut out = Vec::new();
    for (seed, mut pool, complete, wide, replayed) in batches {
        if do_relabel {
            relabel(&mut pool);
        }
        let (full, budgets) = evaluate(&pool, modulus, &fractions, seed);
        let line: Vec<String> = budgets[0]
            .outcomes
            .iter()
            .map(|o| format!("{}={}", o.policy.name(), o.rank.left_nullity))
            .collect();
        eprintln!(
            "{source} seed {seed} pool {} full {} | at {}: {}",
            pool.len(),
            full.rank.left_nullity,
            budgets[0].fraction,
            line.join(" ")
        );
        out.push(Repeat { seed, pool: pool.len(), complete, dropped_wide: wide, replayed, full, budgets });
    }
    let report = Report {
        schema: "lp-retention-eval/v1",
        source,
        modulus,
        relabel: do_relabel,
        params,
        repeats: out,
    };
    let json = serde_json::to_string_pretty(&report).unwrap() + "\n";
    match get("--out", "").as_str() {
        "" => print!("{json}"),
        path => std::fs::write(path, json).expect("--out"),
    }
}
