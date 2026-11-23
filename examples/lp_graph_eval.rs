//! Run the large-prime graph evaluator (catalog #4917 §8) on one frozen stream.
//!
//! ```text
//! cargo run --release --example lp_graph_eval -- \
//!     [--input relations.jsonl] [--modulus P] [--seed S] [--rows R] \
//!     [--lp-columns C] [--lp-min M] [--fb-columns F] [--arity w0,w1,w2,w3] [--unit] \
//!     [--label NAME] [--out eval.json] [--exact-forest] [--forest-max-rows 4096]
//! ```
//!
//! With `--input`, each line is a `PartialRelation` JSON object
//! (`{"id","small","large","rhs"}`); otherwise a seeded synthetic stream is
//! generated. The output is one `lp-graph-eval/v1` JSON record.

use crypto_lib::cryptanalysis::large_prime_filter::PartialRelation;
use crypto_lib::cryptanalysis::large_prime_graph::eval::{
    evaluate_stream, synthetic_stream, EvalParams, SyntheticParams,
};
use std::io::BufRead;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let get = |flag: &str| {
        args.iter()
            .position(|a| a == flag)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    // 2^61 - 1, a Mersenne prime.
    let modulus: u64 = get("--modulus").map_or((1u64 << 61) - 1, |v| v.parse().expect("--modulus"));
    let mut synth = SyntheticParams::default();
    if let Some(v) = get("--seed") {
        synth.seed = v.parse().expect("--seed");
    }
    if let Some(v) = get("--rows") {
        synth.rows = v.parse().expect("--rows");
    }
    if let Some(v) = get("--lp-columns") {
        synth.lp_columns = v.parse().expect("--lp-columns");
    }
    if let Some(v) = get("--lp-min") {
        synth.lp_min = v.parse().expect("--lp-min");
    }
    if let Some(v) = get("--fb-columns") {
        synth.fb_columns = v.parse().expect("--fb-columns");
    }
    if let Some(v) = get("--arity") {
        synth.arity_weights = v.split(',').map(|w| w.parse().expect("--arity")).collect();
    }
    synth.unit_coefficients = args.iter().any(|a| a == "--unit");
    let (relations, label) = match get("--input") {
        Some(path) => {
            let file = std::fs::File::open(&path).expect("open --input");
            let rows: Vec<PartialRelation> = std::io::BufReader::new(file)
                .lines()
                .map(|l| l.expect("read line"))
                .filter(|l| !l.trim().is_empty())
                .map(|l| serde_json::from_str(&l).expect("PartialRelation JSON"))
                .collect();
            (rows, get("--label").unwrap_or(path))
        }
        None => (
            synthetic_stream(&synth, modulus),
            get("--label")
                .unwrap_or_else(|| format!("synthetic {}", serde_json::to_string(&synth).unwrap())),
        ),
    };
    // Explicit opt-in avoids changing historical serialized records.
    // Oversized or >2-LP inputs are reported as a skip by evaluate_stream.
    let mut params = EvalParams::default();
    params.compare_exact_forest = args.iter().any(|arg| arg == "--exact-forest");
    if let Some(v) = get("--forest-max-rows") {
        params.forest_max_rows = v.parse().expect("--forest-max-rows integer");
        assert!(
            (1..=4096).contains(&params.forest_max_rows),
            "--forest-max-rows must be between 1 and 4096"
        );
    }
    let eval = evaluate_stream(&label, &relations, modulus, &params).expect("evaluate");
    if params.compare_exact_forest {
        // These are source-coefficient replay checks, not group replay.
        assert_eq!(eval.pivot_filter.replay_failures, 0, "pivot source replay");
        if !eval.mst_filter.refused {
            assert_eq!(eval.mst_filter.replay_failures, 0, "MST source replay");
        }
        if let Some(ref forest) = eval.forest_filter {
            if !forest.refused {
                assert_eq!(forest.replay_failures, 0, "forest source replay");
            }
        }
    }
    let json = serde_json::to_string_pretty(&eval).unwrap();
    match get("--out") {
        Some(path) => std::fs::write(path, json + "\n").expect("write --out"),
        None => println!("{json}"),
    }
}
