//! Paired benchmark of the full-width chained-S3 SAT decomposer on the pinned
//! N83 fixture (`icv1-f2m83-t6151469093347-cdcc5432`), m = 2..9, across the
//! summand-order arms of [`ChainS3Order`].
//!
//! Every arm answers the same targets, in an order rotated per target so
//! drift falls on every arm. A target is either a public probe `[a]G`
//! (almost surely no decomposition: the arm must prove UNSAT for every
//! identity pattern) or a planted sum of `m` base points (the positive
//! control). The counted unit is solver conflicts; wall time is recorded
//! beside it. Every witness is re-added in the curve group.
//!
//! ```text
//! n83_chain_sat_bench <m> <union_dim> <public> <planted> <conflict_budget> <seed> <arms> <wall_seconds>
//! ```
//! `arms` is a comma list of `unordered`, `first-pair`, `all-summands`, each
//! optionally followed by `+affine` (search only the all-affine identity
//! pattern, [`ChainS3Patterns::AffineOnly`]: complete for decompositions with
//! no zero-sum sub-multiset) and `+t<F>.<B>` (root-finder tables of
//! intermediate x-coordinates, [`ChainS3Tables`] with front depth `F`, back
//! depth `B`, at most 300,000 points per table). One JSON line per
//! (target, arm).
use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::koblitz_index_calculus::*;
use num_bigint::BigUint;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::json;
use std::time::Instant;

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.len() != 8 {
        eprintln!(
            "usage: <m> <union_dim> <public> <planted> <conflict_budget> <seed> <arms> <wall_seconds>"
        );
        std::process::exit(2);
    }
    let m: usize = a[0].parse().expect("m");
    let dim: usize = a[1].parse().expect("union_dim");
    let public: usize = a[2].parse().expect("public");
    let planted: usize = a[3].parse().expect("planted");
    let budget: u64 = a[4].parse().expect("conflict_budget");
    let seed: u64 = a[5].parse().expect("seed");
    let wall_seconds: u64 = a[7].parse().expect("wall_seconds");
    let arms: Vec<(String, ChainS3Options)> = a[6]
        .split(',')
        .map(|spec| {
            let mut parts = spec.split('+');
            let order = match parts.next().unwrap_or("") {
                "unordered" => ChainS3Order::Unordered,
                "first-pair" => ChainS3Order::FirstPair,
                "all-summands" => ChainS3Order::AllSummands,
                other => panic!("unknown order {other}"),
            };
            let mut options = ChainS3Options {
                frobenius_orbits: false,
                order,
                patterns: ChainS3Patterns::All,
                tables: ChainS3Tables::default(),
                wall_budget: Some(std::time::Duration::from_secs(wall_seconds)),
            };
            for part in parts {
                if part == "affine" {
                    options.patterns = ChainS3Patterns::AffineOnly;
                } else if let Some(depths) = part.strip_prefix('t') {
                    let (f, b) = depths.split_once('.').expect("t<F>.<B>");
                    options.tables = ChainS3Tables {
                        front: f.parse().expect("front depth"),
                        back: b.parse().expect("back depth"),
                        max_points: 300_000,
                    };
                } else {
                    panic!("unknown arm modifier {part}");
                }
            }
            (spec.to_string(), options)
        })
        .collect();

    let kc = KoblitzCurve::known_n83_k1().expect("N83 K1 fixture");
    let basis: Vec<F2mElement> = (0..dim)
        .map(|i| F2mElement::from_biguint(&(BigUint::from(1u8) << i), kc.n))
        .collect();
    let fb = build_frobenius_union_factor_base(&kc, &basis).expect("union base");
    let limits = ChainS3Limits {
        max_variables: 4_000_000,
        max_domain_clauses: 50_000_000,
        max_models: 64,
        conflict_budget: budget,
    };
    let mut rng = StdRng::seed_from_u64(seed ^ ((m as u64) << 32) ^ (dim as u64));
    let r = kc.subgroup_order.clone();
    let mut targets: Vec<(&str, BinaryPoint)> = Vec::new();
    for _ in 0..public {
        let k = BigUint::from(rng.gen_range(1..u64::MAX)) % &r;
        targets.push(("public", kc.mul(kc.generator(), &k)));
    }
    for _ in 0..planted {
        let p = (0..m).fold(BinaryPoint::Infinity, |acc, _| {
            kc.add(&acc, &fb.points[rng.gen_range(0..fb.points.len())])
        });
        targets.push(("planted", p));
    }
    for (t, (kind, target)) in targets.iter().enumerate() {
        for j in 0..arms.len() {
            let (name, options) = &arms[(t + j) % arms.len()];
            let start = Instant::now();
            let (found, stats) =
                sat_decompose_chained_s3_with(&kc, &fb, target, m, limits, *options);
            let seconds = start.elapsed().as_secs_f64();
            let verified = found.as_ref().is_some_and(|idx| {
                idx.len() == m
                    && idx
                        .iter()
                        .fold(BinaryPoint::Infinity, |acc, &i| kc.add(&acc, &fb.points[i]))
                        == *target
            });
            let verdict = if found.is_some() {
                "witness"
            } else if stats.refuted {
                "refuted"
            } else {
                "undecided"
            };
            println!(
                "{}",
                json!({
                    "schema": "n83-chain-sat-bench/v1",
                    "curve": kc.label(), "m": m, "union_dim": dim,
                    "base_points": fb.points.len(), "target": t, "kind": kind,
                    "arm": name, "position": j, "verdict": verdict,
                    "options": serde_json::to_value(options).unwrap_or_default(),
                    "verified": verified, "conflicts": stats.conflicts,
                    "conflict_budget": budget, "seconds": seconds,
                    "patterns_attempted": stats.patterns_attempted,
                    "patterns_refuted": stats.patterns_refuted,
                    "solver_calls": stats.solver_calls, "models": stats.models,
                    "group_rejected_models": stats.group_rejected_models,
                    "invalid_models": stats.invalid_models,
                    "peak_variables": stats.peak_variables,
                    "peak_clauses": stats.peak_clauses, "peak_xors": stats.peak_xors,
                    "seed": seed,
                })
            );
        }
    }
}
