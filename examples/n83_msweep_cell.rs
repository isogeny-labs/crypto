//! One cell of the N83 summand sweep: a factor-base construction, a
//! decomposition engine and a summand count `m`, measured on the pinned
//! `K_1 / F_2^83` fixture (`KoblitzCurve::known_n83_k1`, 53-bit subgroup).
//!
//! Every query is a fresh public probe `R = [a]G` from a seeded stream and
//! goes straight to the library's decomposer, so the collector's cofactor
//! admissibility walk (a sumset over `h ≈ 1.1·10^9` classes at this width)
//! is not on the path. Each outcome is the decomposer's own verdict and
//! every witness is re-added in the curve group before it is counted.
//!
//! Relation collection only: a row is a stage diagnostic. The projected
//! collection time uses the measured seconds per query and either the
//! observed hit rate or, when no relation was seen, the heuristic
//! `p ≈ |B|^m / (m! · #E)` (named as such in the record), never both.
//!
//! ```text
//! n83_msweep_cell cell <m> <fb> <engine> <trials> [seed] [per_query_ms] [public|planted] [curve]
//! ```
//!
//! `planted` replaces each probe with a sum of `m` random base points: the
//! positive control showing the engine finds a relation that exists.
//! `curve` is `n83k1` (default, the pinned fixture) or `k<a>n<degree>`
//! (`KoblitzCurve::new`), for the exact-field companion rows below 64 bits.
//!
//! fb:     subspace:D | subspace-2t:D | projected:D | union:S | orbit:K
//! engine: enumerate | f4-matrix | f5-matrix | f4-inherited | f6-ic
//!         | f6-ic-pair | crossbred | sat-native | sat-cnf | sat-restrict
//!         | mq-fes | chain-s3 | chain-s3-orbit | chain-s3-orbit-ordered
use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::koblitz_groebner::{FieldStructure, SolveStats, SolverEngine};
use crypto_lib::cryptanalysis::koblitz_index_calculus::*;
use crypto_lib::cryptanalysis::mq_fes::mq_fes_decompose;
use crypto_lib::cryptanalysis::semaev_sat::XorEncoding;
use num_bigint::BigUint;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::json;
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

fn factor_base(kc: &KoblitzCurve, spec: &str, seed: u64) -> Result<FrobeniusFactorBase, String> {
    let (kind, arg) = spec.split_once(':').ok_or("factor base is kind:value")?;
    let v: usize = arg
        .parse()
        .map_err(|_| format!("bad factor-base size `{arg}`"))?;
    match kind {
        "subspace" => build_standard_subspace_factor_base(kc, v as u32),
        "subspace-2t" => {
            let fb = build_standard_subspace_factor_base(kc, v as u32)?;
            saturate_factor_base_two_torsion(kc, &fb).ok_or("two-torsion saturation failed".into())
        }
        "projected" => {
            let fb = build_standard_subspace_factor_base(kc, v as u32)?;
            cofactor_project_factor_base(kc, &fb)
        }
        "union" => {
            let basis: Vec<F2mElement> = (0..v)
                .map(|i| F2mElement::from_biguint(&(BigUint::from(1u8) << i), kc.n))
                .collect();
            build_frobenius_union_factor_base(kc, &basis).ok_or("union base failed".into())
        }
        "orbit" => build_subgroup_orbit_factor_base(kc, seed, v),
        other => Err(format!("unknown factor base `{other}`")),
    }
}

/// What one query returned, in the collector's vocabulary.
struct Query {
    points: Option<Vec<usize>>,
    verdict: &'static str,
    work: u64,
}

fn verdict(found: bool, refuted: bool, exhausted: bool, unsupported: bool) -> &'static str {
    if found {
        "witness"
    } else if unsupported {
        "unsupported"
    } else if refuted {
        "proved_unsat"
    } else if exhausted {
        "incomplete"
    } else {
        "unresolved"
    }
}

#[allow(clippy::too_many_arguments)]
fn query(
    kc: &KoblitzCurve,
    fb: &FrobeniusFactorBase,
    index_of: &std::collections::HashMap<(BigUint, BigUint), usize>,
    st: &FieldStructure,
    target: &BinaryPoint,
    m: usize,
    engine: &str,
    budget: Duration,
) -> Result<Query, String> {
    let deg = 3;
    let node_budget = 8192;
    let sat = |enc: XorEncoding, restrict: bool| SatDecompositionOptions {
        encoding: enc,
        restrict_to_factor_base: restrict,
        conflict_budget: 200_000,
        wall_budget: Some(budget),
        ..SatDecompositionOptions::default()
    };
    let chain = ChainS3Limits {
        max_variables: 4_000_000,
        max_domain_clauses: 50_000_000,
        max_models: 64,
        conflict_budget: 200_000,
    };
    let groebner = |e: SolverEngine, f: fn(_, _, _, _, _, _, _, _) -> _| {
        let (p, s): (Option<Vec<usize>>, SolveStats) =
            f(kc, fb, index_of, st, target, m, e, node_budget);
        Query {
            verdict: verdict(
                p.is_some(),
                !s.exhausted && !s.unsupported,
                s.exhausted,
                s.unsupported,
            ),
            points: p,
            work: s.reductions as u64,
        }
    };
    let inh = SolverEngine::InheritedF4 { max_degree: deg };
    Ok(match engine {
        "enumerate" => {
            let p = enumerate_decompose(kc, fb, index_of, target, m);
            Query {
                verdict: verdict(p.is_some(), true, false, false),
                points: p,
                work: 0,
            }
        }
        "f4-matrix" => groebner(
            SolverEngine::MatrixF4 { max_degree: deg },
            groebner_decompose,
        ),
        "f5-matrix" => groebner(
            SolverEngine::MatrixF5 { max_degree: deg },
            groebner_decompose,
        ),
        "f4-inherited" => groebner(inh, groebner_decompose),
        "f6-ic" => groebner(inh, groebner_decompose_f6_ic),
        "f6-ic-pair" => groebner(inh, groebner_decompose_f6_ic_pair_index),
        "crossbred" => {
            let (p, s) = crossbred_decompose(kc, fb, index_of, st, target, m, None);
            Query {
                verdict: verdict(
                    p.is_some(),
                    !s.exhausted && !s.unsupported,
                    s.exhausted,
                    s.unsupported,
                ),
                points: p,
                work: 0,
            }
        }
        "sat-native" | "sat-cnf" | "sat-restrict" | "mq-fes" => {
            let (p, s) = if engine == "mq-fes" {
                mq_fes_decompose(kc, fb, index_of, st, target, m)
            } else {
                let enc = if engine == "sat-cnf" {
                    XorEncoding::Cnf
                } else {
                    XorEncoding::Native
                };
                sat_decompose_with(
                    kc,
                    fb,
                    index_of,
                    st,
                    target,
                    m,
                    64,
                    Some(2),
                    sat(enc, engine == "sat-restrict"),
                )
            };
            Query {
                verdict: verdict(p.is_some(), s.refuted, s.exhausted, s.unsupported),
                points: p,
                work: s.solver_calls as u64,
            }
        }
        "chain-s3" | "chain-s3-orbit" | "chain-s3-orbit-ordered" => {
            let (p, s) = sat_decompose_chained_s3_any_m(
                kc,
                fb,
                target,
                m,
                chain,
                engine != "chain-s3",
                engine == "chain-s3-orbit-ordered",
            );
            let v = if s.invalid_models != 0 {
                "invalid_model"
            } else {
                verdict(p.is_some(), s.refuted, s.exhausted, false)
            };
            Query {
                verdict: v,
                points: p,
                work: s.conflicts,
            }
        }
        other => return Err(format!("unknown engine `{other}`")),
    })
}

fn factorial(m: usize) -> f64 {
    (1..=m).map(|i| i as f64).product()
}

fn cell(
    m: usize,
    fb_spec: &str,
    engine: &str,
    trials: u64,
    seed: u64,
    per_query: Duration,
    planted: bool,
    curve: &str,
) -> Result<serde_json::Value, String> {
    let kc = if curve == "n83k1" {
        KoblitzCurve::known_n83_k1().ok_or("N83 K1 fixture")?
    } else {
        let (a, n) = curve
            .strip_prefix('k')
            .and_then(|c| c.split_once('n'))
            .ok_or_else(|| format!("bad curve `{curve}`"))?;
        let (a, n): (u8, u32) = (a.parse().map_err(|_| "a")?, n.parse().map_err(|_| "n")?);
        KoblitzCurve::new(a, n).ok_or_else(|| format!("no Koblitz curve K_{a} at degree {n}"))?
    };
    let built = Instant::now();
    let fb = factor_base(&kc, fb_spec, seed)?;
    let fb_seconds = built.elapsed().as_secs_f64();
    let index_of = fb.shared_index_map();
    let st = FieldStructure::new(kc.n, &kc.curve.irreducible);
    let r = kc.subgroup_order.clone();
    let group_order = &r * &kc.cofactor;
    let mut rng = StdRng::seed_from_u64(seed ^ ((m as u64) << 40));
    let mut outcomes: BTreeMap<&str, u64> = BTreeMap::new();
    let (mut relations, mut verified, mut work, mut done) = (0u64, 0u64, 0u64, 0u64);
    let mut per_trial = Vec::new();
    let begin = Instant::now();
    for _ in 0..trials {
        let target = if planted {
            (0..m).fold(BinaryPoint::Infinity, |acc, _| {
                kc.add(&acc, &fb.points[rng.gen_range(0..fb.points.len())])
            })
        } else {
            let a = BigUint::from(rng.gen_range(1..u64::MAX)) % &r;
            kc.mul(kc.generator(), &a)
        };
        let t = Instant::now();
        let q = query(&kc, &fb, &index_of, &st, &target, m, engine, per_query)?;
        per_trial.push(t.elapsed().as_secs_f64());
        done += 1;
        work += q.work;
        *outcomes.entry(q.verdict).or_default() += 1;
        if let Some(points) = q.points {
            relations += 1;
            let sum = points
                .iter()
                .fold(BinaryPoint::Infinity, |acc, &i| kc.add(&acc, &fb.points[i]));
            if points.len() == m && sum == target {
                verified += 1;
            }
        }
    }
    let seconds = begin.elapsed().as_secs_f64();
    let t = done.max(1) as f64;
    let spt = seconds / t;
    per_trial.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = per_trial.get(per_trial.len() / 2).copied().unwrap_or(0.0);
    // log2 p for the heuristic: |B|^m / (m! #E), B the signed point set.
    let log2_order = {
        let bits = group_order.bits();
        let top = (&group_order >> bits.saturating_sub(53)).to_u64_digits()[0] as f64;
        top.log2() + bits.saturating_sub(53) as f64
    };
    let log2_p_heur =
        (m as f64 * (fb.points.len() as f64).log2() - factorial(m).log2() - log2_order).min(0.0);
    let need = fb.unknowns() as f64 + 4.0;
    let decided = outcomes.get("proved_unsat").copied().unwrap_or(0) + relations;
    let decided_fraction = decided as f64 / t;
    let (p_used, p_source) = if relations > 0 {
        (relations as f64 / t, "observed")
    } else {
        (2f64.powf(log2_p_heur), "heuristic |B|^m/(m!#E)")
    };
    // A query the engine did not decide (unsupported, incomplete, budget)
    // returns early and is not a price: project only from decided cells.
    let projection = (decided_fraction >= 0.9).then(|| (need / p_used * spt).log2());
    Ok(json!({
        "schema": "n83-msweep-cell/v2",
        "curve_spec": curve,
        "curve": kc.label(),
        "field_degree": kc.n,
        "subgroup_order": r.to_string(),
        "group_order": group_order.to_string(),
        "m": m, "factor_base": fb_spec, "engine": engine, "seed": seed,
        "probes": if planted { "planted" } else { "public_subgroup" },
        "per_query_budget_ms": per_query.as_millis() as u64,
        "base_points": fb.points.len(), "columns": fb.unknowns(),
        "factor_base_seconds": fb_seconds,
        "status": "complete",
        "trials": done, "outcomes": outcomes,
        "decided_fraction": decided_fraction,
        "relations": relations, "verified_relations": verified,
        "engine_work": work,
        "collection_seconds": seconds,
        "seconds_per_trial": spt, "median_seconds_per_trial": median,
        "log2_p_heuristic": log2_p_heur,
        "p_used": p_used, "p_source": p_source,
        "relations_needed": need,
        "log2_projected_collection_seconds": projection,
        "projection_rule": "need / p * seconds_per_trial, only when >= 90% of queries were decided",
    }))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out = match args.first().map(String::as_str) {
        Some("cell") if args.len() >= 5 => cell(
            args[1].parse().expect("m"),
            &args[2],
            &args[3],
            args[4].parse().expect("trials"),
            args.get(5).map_or(0x83_5EED, |s| s.parse().expect("seed")),
            Duration::from_millis(args.get(6).map_or(10_000, |s| s.parse().expect("ms"))),
            args.get(7).is_some_and(|s| s == "planted"),
            args.get(8).map_or("n83k1", String::as_str),
        ),
        _ => Err("usage: cell <m> <fb> <engine> <trials> [seed] [per_query_ms]".into()),
    };
    match out {
        Ok(v) => println!("{v}"),
        Err(e) => {
            println!("{}", json!({"status": "error", "error": e, "args": args}));
            std::process::exit(2);
        }
    }
}
