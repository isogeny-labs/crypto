//! Online admission of partial relations under a memory cap
//! (`large_prime_graph::retention`): for each rule and cap, the draw at which
//! the useful factor-base rank first reaches its target.
//!
//! ```text
//! # F_p* toy: p = nextprime(2^31 + 11e6·i), B, L, up to --draws draws per seed
//! cargo run --release --example lp_retention_eval -- --source fp --p-index 0 \
//!     --repeats 5 --smooth 1024 --lp 65536 --draws 400000 --caps 0.25,1,4 --out fp0.json
//! # a captured stream (`lp_graph_collect --relations out.jsonl`); draw = line number
//! cargo run --release --example lp_retention_eval -- --source relations \
//!     --relations out.jsonl --modulus 65557 --caps 0.25,1,4 --out real.json
//! ```
//!
//! Caps are multiples of the factor-base size (the relation-matrix width).
//! Target: `fp` uses the factor-base size; `relations` uses the useful rank
//! `all` reaches on the whole stream. `MaxLp` rules (`--max-lp`, default L/4
//! and L/16 for `fp`) are run for `fp` only: on curves an LP identity is a
//! point label, not a size.

use crypto_lib::cryptanalysis::large_prime_filter::PartialRelation;
use crypto_lib::cryptanalysis::large_prime_graph::retention::{
    simulate_admission, AdmissionOutcome, AdmissionRule, StreamRow,
};
use serde::Serialize;
use std::io::BufRead;

#[path = "lp_retention_support/fp_toy.rs"]
mod fp_toy;

#[derive(Serialize)]
struct Run {
    seed: u64,
    rows: usize,
    draws_available: u64,
    fb_columns: usize,
    target: usize,
    replayed: Option<usize>,
    outcomes: Vec<AdmissionOutcome>,
}

#[derive(Serialize)]
struct Report {
    schema: &'static str,
    source: String,
    modulus: u64,
    cap_multiples: Vec<f64>,
    params: serde_json::Value,
    runs: Vec<Run>,
}

fn evaluate(
    stream: &[StreamRow],
    fb: usize,
    ell: u64,
    target: usize,
    caps: &[f64],
    max_lp: &[u64],
) -> Vec<AdmissionOutcome> {
    let run = |rule, cap| {
        simulate_admission(stream, fb, ell, rule, cap, target, false).expect("simulate")
    };
    let mut out = vec![
        run(AdmissionRule::All, None),
        run(AdmissionRule::CompleteOnly, None),
    ];
    for &m in caps {
        let cap = ((fb as f64) * m).round().max(1.0) as usize;
        let mut rules = vec![
            AdmissionRule::FirstCome,
            AdmissionRule::SinglesOnly,
            AdmissionRule::SeenLp,
        ];
        rules.extend(max_lp.iter().map(|&b| AdmissionRule::MaxLp(b)));
        for rule in rules {
            out.push(run(rule, Some(cap)));
        }
    }
    out
}

fn summary(outcomes: &[AdmissionOutcome]) -> String {
    outcomes
        .iter()
        .map(|o| format!("{}@{:?}={:?}", o.rule.name(), o.cap, o.draws_to_target))
        .collect::<Vec<_>>()
        .join(" ")
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
    let caps: Vec<f64> = get("--caps", "0.25,1,4")
        .split(',')
        .map(|x| x.parse().expect("--caps"))
        .collect();
    let repeats: u64 = get("--repeats", "5").parse().expect("--repeats");
    let mut runs = Vec::new();
    let (modulus, params);
    match source.as_str() {
        "fp" => {
            let i: u64 = get("--p-index", "0").parse().unwrap();
            let p = fp_toy::Instance::next_prime((1u64 << 31) + 11_000_000 * i);
            let inst = fp_toy::Instance::new(p, 1001);
            let smooth: u64 = get("--smooth", "1024").parse().unwrap();
            let lp: u64 = get("--lp", "65536").parse().unwrap();
            let draws: u64 = get("--draws", "400000").parse().unwrap();
            let max_lp: Vec<u64> = match get("--max-lp", "").as_str() {
                "" => vec![lp / 4, lp / 16],
                s => s.split(',').map(|x| x.parse().unwrap()).collect(),
            };
            modulus = inst.ell;
            params = serde_json::json!({
                "instance": inst, "smooth_bound": smooth, "lp_bound": lp,
                "draws": draws, "max_lp": max_lp
            });
            for r in 0..repeats {
                let seed = 1000 * i + r;
                let col = fp_toy::collect(&inst, smooth, lp, draws, seed);
                let replayed =
                    fp_toy::replay(&col).unwrap_or_else(|j| panic!("replay failed at row {j}"));
                let fb = col.factor_base.len();
                let outcomes = evaluate(&col.stream, fb, modulus, fb, &caps, &max_lp);
                eprintln!(
                    "fp p={p} seed={seed} rows={} | {}",
                    col.stream.len(),
                    summary(&outcomes)
                );
                runs.push(Run {
                    seed,
                    rows: col.stream.len(),
                    draws_available: draws,
                    fb_columns: fb,
                    target: fb,
                    replayed: Some(replayed),
                    outcomes,
                });
            }
        }
        "relations" => {
            modulus = get("--modulus", "0")
                .parse()
                .expect("--modulus (the relation prime ℓ)");
            let path = get("--relations", "");
            let f = std::io::BufReader::new(std::fs::File::open(&path).expect("--relations"));
            let stream: Vec<StreamRow> = f
                .lines()
                .enumerate()
                .map(|(i, l)| StreamRow {
                    draw: i as u64 + 1,
                    relation: serde_json::from_str::<PartialRelation>(&l.unwrap())
                        .expect("PartialRelation JSON line"),
                })
                .collect();
            let fb = stream
                .iter()
                .flat_map(|r| r.relation.small.iter().map(|&(c, _)| c as usize + 1))
                .max()
                .unwrap_or(0);
            let full = simulate_admission(
                &stream,
                fb,
                modulus,
                AdmissionRule::All,
                None,
                usize::MAX,
                false,
            )
            .expect("full pass");
            let target = full.final_rank;
            params = serde_json::json!({
                "relations": path, "rows": stream.len(), "fb_columns": fb,
                "target_from_full_stream": target
            });
            // One fixed stream in collection order; the rules are deterministic.
            let outcomes = evaluate(&stream, fb, modulus, target, &caps, &[]);
            eprintln!(
                "relations rows={} fb={fb} target={target} | {}",
                stream.len(),
                summary(&outcomes)
            );
            runs.push(Run {
                seed: 0,
                rows: stream.len(),
                draws_available: stream.len() as u64,
                fb_columns: fb,
                target,
                replayed: None,
                outcomes,
            });
        }
        other => panic!("--source must be fp or relations, not {other}"),
    }
    let report = Report {
        schema: "lp-admission-eval/v1",
        source,
        modulus,
        cap_multiples: caps,
        params,
        runs,
    };
    let json = serde_json::to_string_pretty(&report).unwrap() + "\n";
    match get("--out", "").as_str() {
        "" => print!("{json}"),
        path => std::fs::write(path, json).expect("--out"),
    }
}
