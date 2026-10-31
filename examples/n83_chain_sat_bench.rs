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
//! n83_chain_sat_bench <m> <union_dim> <public> <planted> <conflict_budget> <seed> <arms>
//! ```
//! `arms` is a comma list of `unordered`, `first-pair`, `all-summands`, each
//! optionally suffixed `+affine` to search only the all-affine identity
//! pattern ([`ChainS3Patterns::AffineOnly`]: complete for decompositions
//! with no zero-sum sub-multiset). One JSON line per (target, arm).
use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::koblitz_index_calculus::*;
use num_bigint::BigUint;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::json;
use std::time::Instant;

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.len() != 7 {
        eprintln!("usage: <m> <union_dim> <public> <planted> <conflict_budget> <seed> <arms>");
        std::process::exit(2);
    }
    let m: usize = a[0].parse().expect("m");
    let dim: usize = a[1].parse().expect("union_dim");
    let public: usize = a[2].parse().expect("public");
    let planted: usize = a[3].parse().expect("planted");
    let budget: u64 = a[4].parse().expect("conflict_budget");
    let seed: u64 = a[5].parse().expect("seed");
    let arms: Vec<(String, ChainS3Order, ChainS3Patterns)> = a[6]
        .split(',')
        .map(|s| {
            let (base, patterns) = match s.strip_suffix("+affine") {
                Some(base) => (base, ChainS3Patterns::AffineOnly),
                None => (s, ChainS3Patterns::All),
            };
            let o = match base {
                "unordered" => ChainS3Order::Unordered,
                "first-pair" => ChainS3Order::FirstPair,
                "all-summands" => ChainS3Order::AllSummands,
                other => panic!("unknown arm {other}"),
            };
            (s.to_string(), o, patterns)
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
            let (name, order, patterns) = &arms[(t + j) % arms.len()];
            let start = Instant::now();
            let (found, stats) = sat_decompose_chained_s3_patterns(
                &kc, &fb, target, m, limits, false, *order, *patterns,
            );
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
                    "patterns_mode": format!("{patterns:?}"),
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
