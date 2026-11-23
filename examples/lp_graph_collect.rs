//! Collect a real large-prime relation stream from the binary-curve
//! `ecbench_large_prime` collector and evaluate it with
//! `large_prime_graph::eval` (catalog #4917 §8).
//!
//! ```text
//! cargo run --release --example lp_graph_collect -- \
//!     --n 20 --seed 20261123 --small 4 --envelope 7 --summands 4 \
//!     --large-primes 2 --trials 40000 --graph filter|orbit \
//!     --relations out.jsonl --out eval.json
//! ```
//!
//! The curve is `ic_boundary::random_binary_instance(n, seed, 16)`, or with
//! `--koblitz` `ic_boundary::koblitz_instance_best(n)` (needed by `--graph
//! orbit`, the signed-Frobenius-orbit large-prime graph); the
//! target is `[7]G`. The relations are the exact `PartialRelation` batch the
//! solver hands to `filter_large_primes` (`SolveOptions::capture_relations`).

use crypto_lib::cryptanalysis::ecbench_large_prime::{
    solve_with_options, LargePrimeGraph, SolveConfig, SolveOptions,
};
use crypto_lib::cryptanalysis::ic_boundary::{koblitz_instance_best, random_binary_instance};
use crypto_lib::cryptanalysis::large_prime_graph::eval::{evaluate_stream, EvalParams};
use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let get = |flag: &str, default: &str| -> String {
        args.iter()
            .position(|a| a == flag)
            .and_then(|i| args.get(i + 1))
            .cloned()
            .unwrap_or_else(|| default.to_string())
    };
    let n: u32 = get("--n", "20").parse().expect("--n");
    let seed: u64 = get("--seed", "20261123").parse().expect("--seed");
    let graph = match get("--graph", "filter").as_str() {
        "filter" => LargePrimeGraph::Filter,
        "orbit" => LargePrimeGraph::FilterOrbit,
        other => panic!("--graph must be filter or orbit, not {other}"),
    };
    let koblitz = args.iter().any(|a| a == "--koblitz");
    let instance = if koblitz {
        koblitz_instance_best(n).expect("Koblitz instance (n prime)")
    } else {
        random_binary_instance(n, seed, 16).expect("instance")
    };
    let target = instance.fast.mul_u64(instance.generator, 7);
    let config = SolveConfig {
        small_dimension: get("--small", "4").parse().expect("--small"),
        envelope_dimension: get("--envelope", "7").parse().expect("--envelope"),
        summands: get("--summands", "4").parse().expect("--summands"),
        max_large_primes: get("--large-primes", "2").parse().expect("--large-primes"),
        max_trials: get("--trials", "40000").parse().expect("--trials"),
        max_states: get("--states", "4000000").parse().expect("--states"),
        seed,
        large_prime_graph: graph,
    };
    let report = solve_with_options(
        &instance,
        target,
        config.clone(),
        SolveOptions {
            capture_relations: true,
            ..Default::default()
        },
    )
    .expect("solve");
    let pipeline = report.filter_pipeline.as_ref().expect("filter arm");
    let relations = pipeline.relations.clone().expect("captured relations");
    eprintln!(
        "n={n} r={} recovered={:?} relations={} pipeline_rank={} complete={}",
        instance.r,
        report.recovered,
        relations.len(),
        pipeline.rank,
        pipeline.complete_relations
    );
    if let Some(path) = args
        .iter()
        .position(|a| a == "--relations")
        .and_then(|i| args.get(i + 1))
    {
        let mut f = std::io::BufWriter::new(std::fs::File::create(path).expect("--relations"));
        for r in &relations {
            writeln!(f, "{}", serde_json::to_string(r).unwrap()).unwrap();
        }
    }
    let label = format!(
        "ecbench_large_prime binary n={n} seed={seed} koblitz={koblitz} r={} graph={:?} config={:?} recovered={:?}",
        instance.r, graph, config, report.recovered
    );
    let eval =
        evaluate_stream(&label, &relations, instance.r, &EvalParams::default()).expect("evaluate");
    let json = serde_json::to_string_pretty(&eval).unwrap() + "\n";
    match args
        .iter()
        .position(|a| a == "--out")
        .and_then(|i| args.get(i + 1))
    {
        Some(path) => std::fs::write(path, json).expect("--out"),
        None => print!("{json}"),
    }
}
