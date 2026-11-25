//! Real relation streams from two more collectors, evaluated with
//! `large_prime_graph::eval` (catalog #4917 §8; study
//! `research/lp_graph_algorithms_20261123/`).
//!
//! ```text
//! # prime-field automorphism-orbit IC, single-large-prime variation
//! cargo run --release --example lp_graph_collect_fields -- prime-orbit \
//!     --kind generic|j0|j1728 --bits 32 [--orbits K] --seed 1 --out eval.json
//! # Gaudry index calculus on E(F_{p^3}), double large primes
//! cargo run --release --example lp_graph_collect_fields -- gaudry \
//!     --p 1009 --seed 1 --residuals 200000 --large-primes 2 --out eval.json
//! ```
//!
//! * `prime-orbit`: `solve_logs` with `OrbitIcOptions::capture_relations`;
//!   the captured rows are the one- and two-term relations
//!   `x_o − e(γ)·x_{o₁} ≡ …` that the collector solves with
//!   `solve_two_term_system`. Each row's orbit columns become the LP block,
//!   so the gain-aware DSU is checked against that independent solver
//!   (`collector_system` in the output).
//! * `gaudry`: `run_gaudry_opts` with `GaudryOptions::capture_relations`;
//!   every relation offered to the large-prime eliminator, columns
//!   `< small` small and `≥ small` large, exactly as the eliminator splits them.

use crypto_lib::cryptanalysis::ec_index_calculus_curves::CurveKind;
use crypto_lib::cryptanalysis::gaudry_cubic::{
    generate_instance3, run_gaudry_opts, GaudryOptions, SubspaceBase,
};
use crypto_lib::cryptanalysis::large_prime_filter::PartialRelation;
use crypto_lib::cryptanalysis::large_prime_graph::eval::{evaluate_stream, EvalParams};
use crypto_lib::cryptanalysis::prime_orbit_index_calculus::{
    base_orbits, generate_instance, solve_logs, OrbitFactorBase, OrbitIcOptions, ScaledShape,
};
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde_json::json;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let get = |flag: &str, default: &str| -> String {
        args.iter()
            .position(|a| a == flag)
            .and_then(|i| args.get(i + 1))
            .cloned()
            .unwrap_or_else(|| default.to_string())
    };
    let seed: u64 = get("--seed", "1").parse().expect("--seed");
    let family = args.first().map(String::as_str).unwrap_or("");
    let (label, relations, modulus, collector) = match family {
        "prime-orbit" => {
            let kind = match get("--kind", "generic").as_str() {
                "generic" => CurveKind::Generic,
                "j0" => CurveKind::J0,
                "j1728" => CurveKind::J1728,
                other => panic!("--kind must be generic, j0 or j1728, not {other}"),
            };
            let bits: u32 = get("--bits", "32").parse().expect("--bits");
            let inst =
                generate_instance(ScaledShape::of_kind(kind), bits, None, seed).expect("instance");
            let c = &inst.curve;
            let opts = OrbitIcOptions {
                seed,
                capture_relations: true,
                ..OrbitIcOptions::default()
            };
            // `--orbits 0` sizes the base for one target, as the collector does.
            let orbits = match get("--orbits", "0").parse::<usize>().expect("--orbits") {
                0 => base_orbits(c, &opts, 1),
                k => k,
            };
            let fb = OrbitFactorBase::select(c, orbits, seed);
            let (_db, rep) = solve_logs(c, &fb, &opts);
            let rows = rep.captured_relations.clone().expect("captured relations");
            let relations: Vec<PartialRelation> = rows
                .iter()
                .enumerate()
                .map(|(i, row)| PartialRelation {
                    id: i as u64,
                    small: vec![],
                    large: row.terms.iter().map(|&(o, e)| (o as u64, e)).collect(),
                    rhs: row.rhs,
                })
                .collect();
            let s = rep.system;
            let collector = json!({
                "family": "prime-orbit", "kind": format!("{kind:?}"), "bits": bits, "r": c.r,
                "automorphism_order": c.automorphism_order(), "orbits": fb.len(),
                "relations": rep.relations, "collection_rounds": rep.collection_rounds,
                "certified_columns": rep.certified_columns,
                "collector_system": {"components": s.components,
                    "determined_components": s.determined_components,
                    "inconsistent_components": s.inconsistent_components,
                    "rank": s.rank, "solved_columns": s.solved_columns},
            });
            (
                format!("prime-orbit {kind:?} bits={bits} seed={seed}"),
                relations,
                c.r,
                collector,
            )
        }
        "gaudry" => {
            let p: u64 = get("--p", "1009").parse().expect("--p");
            let inst = generate_instance3(p, seed);
            let mut rng = StdRng::seed_from_u64(seed);
            let base = SubspaceBase::build(&inst, &mut rng);
            let small = (base.len() as f64).powf(2.0 / 3.0).ceil() as usize;
            let opts = GaudryOptions {
                max_residuals: get("--residuals", "200000").parse().expect("--residuals"),
                small_base: small,
                max_large_primes: get("--large-primes", "2").parse().expect("--large-primes"),
                capture_relations: true,
                ..GaudryOptions::default()
            };
            let rep = run_gaudry_opts(&inst, &base, seed, &opts);
            let rows = rep.captured_relations.clone().unwrap_or_default();
            let relations: Vec<PartialRelation> = rows
                .iter()
                .enumerate()
                .map(|(i, r)| PartialRelation {
                    id: i as u64,
                    small: r
                        .cols
                        .iter()
                        .filter(|&&(c, _)| c < small)
                        .map(|&(c, v)| (c as u32, v))
                        .collect(),
                    large: r
                        .cols
                        .iter()
                        .filter(|&&(c, _)| c >= small)
                        .map(|&(c, v)| (c as u64, v))
                        .collect(),
                    rhs: r.rhs,
                })
                .collect();
            let collector = json!({
                "family": "gaudry", "p": p, "n": rep.n, "base": rep.base, "small_base": small,
                "residuals": rep.residuals, "lp_full_relations": rep.lp_full_relations,
                "relations_independent": rep.relations_independent,
                "solved": rep.solved, "recovered_scalar": rep.recovered_scalar,
            });
            (
                format!("gaudry p={p} seed={seed} small={small}"),
                relations,
                rep.n,
                collector,
            )
        }
        _ => panic!("first argument must be prime-orbit or gaudry"),
    };
    let eval = evaluate_stream(&label, &relations, modulus, &EvalParams::default()).expect("eval");
    let out = json!({"collector": collector, "eval": eval});
    let text = serde_json::to_string_pretty(&out).unwrap() + "\n";
    match args
        .iter()
        .position(|a| a == "--out")
        .and_then(|i| args.get(i + 1))
    {
        Some(path) => std::fs::write(path, text).expect("--out"),
        None => print!("{text}"),
    }
}
