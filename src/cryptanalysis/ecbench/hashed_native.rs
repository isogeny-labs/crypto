//! End-to-end adapter for the actual native hashed-EC library. It never
//! substitutes the separate ecdl-tensor-ic prototype. Metrics are opt-in;
//! unpriced native work is explicit and prevents a fully priced S claim.

use super::canonical::sha256_hex;
use super::methods::{
    unpriced_of, OnlineWindow, ParamDecl, PhaseRecord, ResolvedMethod, SolveReport,
};
use crate::cryptanalysis::ic_boundary::{PrimeInstance, PrimePoint};
use hashed_ecdl_tensor::{
    arithmetic::{hash_prime, Seeded},
    contraction::Counter,
    curve::{Curve, Point},
    finite, metrics, partial,
    reduction::{self, Config, FactorBase, HashSeed},
    width, BigUint, Error, Limits, Meter,
};
use num_traits::ToPrimitive;
use serde_json::json;
use std::collections::BTreeMap;
use std::time::Instant;

/// Every solver policy is explicit before the method identity is hashed.
pub const PARAMS: &[ParamDecl] = &[
    ParamDecl {
        name: "backend",
        default: None,
        help: "source, selectors, width or partial; actual native library entry",
    },
    ParamDecl {
        name: "m",
        default: None,
        help: "independent slot bases, 2..64",
    },
    ParamDecl {
        name: "trials",
        default: None,
        help: "fresh-seed source trials",
    },
    ParamDecl {
        name: "columns",
        default: None,
        help: "maximum source/partial matrix columns",
    },
    ParamDecl {
        name: "steps",
        default: None,
        help: "charged native meter attempts, deterministic work bound",
    },
    ParamDecl {
        name: "vector",
        default: None,
        help: "maximum allocated native vector/table entries",
    },
    ParamDecl {
        name: "bits",
        default: None,
        help: "native integer/count precision policy",
    },
    ParamDecl {
        name: "threshold",
        default: None,
        help: "0 selects original finite default; positive overrides tau",
    },
    ParamDecl {
        name: "derive_last",
        default: None,
        help: "selector final-slot propagation, 0 or 1",
    },
    ParamDecl {
        name: "smallest_first",
        default: None,
        help: "selector slot order, 0 or 1",
    },
    ParamDecl {
        name: "cache_entries",
        default: None,
        help: "bounded per-query selector decode cache",
    },
    ParamDecl {
        name: "rank",
        default: None,
        help: "width backend maximum factor rank",
    },
    ParamDecl {
        name: "slices",
        default: None,
        help: "width backend maximum conditioned bits",
    },
    ParamDecl {
        name: "outer_multiplier",
        default: None,
        help: "partial outer tau multiplier, positive",
    },
    ParamDecl {
        name: "attempts",
        default: None,
        help: "partial query and admitted-row bound",
    },
];
fn number(m: &ResolvedMethod, key: &str) -> Result<u64, String> {
    m.params[key]
        .parse()
        .map_err(|_| format!("{key} must be an unsigned integer"))
}
fn size(m: &ResolvedMethod, key: &str) -> Result<usize, String> {
    usize::try_from(number(m, key)?).map_err(|_| format!("{key} exceeds usize"))
}
fn flag(m: &ResolvedMethod, key: &str) -> Result<bool, String> {
    match number(m, key)? {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(format!("{key} must be 0 or 1")),
    }
}
fn point(p: PrimePoint) -> Point {
    if p.infinity {
        Point::Infinity
    } else {
        Point::Affine {
            x: p.x.into(),
            y: p.y.into(),
        }
    }
}

/// Run the complete selected native backend with setup, all query failures,
/// elimination and internal replay included. The harness verifies the scalar
/// independently; the adapter never receives its planted/expected value.
pub fn solve(
    m: &ResolvedMethod,
    inst: &PrimeInstance,
    target: PrimePoint,
    seed: u64,
) -> Result<SolveReport, String> {
    if inst.cofactor != 1 || inst.group_order != inst.r {
        return Err(
            "native v1 ECBench adapter requires the certified cofactor-one prime-field family"
                .into(),
        );
    }
    let slots = size(m, "m")?;
    if !(2..=64).contains(&slots) {
        return Err("m must be 2..64".into());
    }
    let backend = m.params["backend"].as_str();
    if !["source", "selectors", "width", "partial"].contains(&backend) {
        return Err("unknown native backend".into());
    }
    let options = finite::Options {
        derive_last: flag(m, "derive_last")?,
        smallest_first: flag(m, "smallest_first")?,
        cache_entries: size(m, "cache_entries")?,
    };
    let policy = width::Policy {
        maximum_rank: size(m, "rank")?,
        maximum_slices: size(m, "slices")?,
    };
    let threshold = number(m, "threshold")?;
    if backend == "partial" && threshold == 0 {
        return Err("partial needs positive threshold".into());
    }
    let config = Config {
        slots,
        trials: size(m, "trials")?,
        columns: size(m, "columns")?,
        threshold: (threshold != 0).then(|| BigUint::from(threshold)),
        ..Config::default()
    };
    let attempts = size(m, "attempts")?;
    let multiplier = number(m, "outer_multiplier")?;
    if multiplier == 0 || attempts == 0 || config.trials == 0 {
        return Err("positive trials/attempts/outer_multiplier required".into());
    }
    let mut meter = Meter::new(Limits {
        steps: number(m, "steps")?,
        vector: size(m, "vector")?,
        bits: size(m, "bits")?,
    });
    let mut rng = Seeded(seed);
    let mut counter = Counter::default();
    let mut statistics = finite::Statistics::default();
    let started = Instant::now();
    let session = metrics::Session::start().map_err(|e| e.to_string())?;
    let result = (|| -> hashed_ecdl_tensor::Result<BigUint> {
        let curve = Curve::new(
            inst.curve.p.into(),
            inst.curve.a.into(),
            inst.curve.b.into(),
            inst.r.into(),
            Point::Affine {
                x: inst.generator.0.into(),
                y: inst.generator.1.into(),
            },
            &mut meter,
        )?;
        let target = point(target);
        match backend {
            "source" => {
                reduction::solve(&curve, &target, &config, &mut rng, &mut counter, &mut meter)
                    .map(|c| c.scalar)
            }
            "selectors" => reduction::solve_with_oracle(
                &curve,
                &target,
                &config,
                &mut rng,
                |c, b, q, w| finite::prime_decompose(c, b, q, &options, &mut statistics, w),
                &mut meter,
            )
            .map(|c| c.scalar),
            "width" => reduction::solve_with_oracle(
                &curve,
                &target,
                &config,
                &mut rng,
                |c, b, q, w| {
                    reduction::decompose_with(
                        c,
                        b,
                        q,
                        |network, work| {
                            let pins = vec![None; network.edge_count()];
                            let plan = width::plan(network, &pins, &policy, work)?;
                            width::witness(network, &pins, &plan, work)
                        },
                        w,
                    )
                },
                &mut meter,
            )
            .map(|c| c.scalar),
            "partial" => {
                metrics::phase("factor_base")?;
                let prime = hash_prime(curve.modulus(), &mut rng, &mut meter)?;
                let outer =
                    (BigUint::from(threshold) * BigUint::from(multiplier)).min(prime.clone());
                let mut bases = Vec::new();
                for _ in 0..slots {
                    use hashed_ecdl_tensor::arithmetic::random_below;
                    let hash = HashSeed {
                        alpha: random_below(&(&prime - BigUint::from(1u8)), &mut rng, &mut meter)?
                            + BigUint::from(1u8),
                        beta: random_below(&prime, &mut rng, &mut meter)?,
                    };
                    bases.push(partial::NestedBase::new(
                        FactorBase::new(
                            &curve,
                            prime.clone(),
                            threshold.into(),
                            hash.clone(),
                            &mut meter,
                        )?,
                        FactorBase::new(&curve, prime.clone(), outer.clone(), hash, &mut meter)?,
                    )?);
                }
                partial::solve_with_options(
                    &curve,
                    &target,
                    bases,
                    attempts,
                    config.columns,
                    attempts,
                    &options,
                    &mut statistics,
                    &mut rng,
                    &mut meter,
                )
                .map(|c| c.scalar)
            }
            _ => unreachable!(),
        }
    })();
    let snapshot = session.finish().map_err(|e| e.to_string())?;
    let wall =
        u64::try_from(started.elapsed().as_nanos()).map_err(|_| "native solve clock overflow")?;
    let mut counters = BTreeMap::new();
    let phases: Vec<_> = snapshot
        .phases
        .iter()
        .map(|(name, p)| {
            counters.insert(
                format!("{name}.native_meter_attempts_uncharged"),
                p.meter_steps,
            );
            PhaseRecord {
                name: name.clone(),
                adds: p.adds,
                doubles: p.doubles,
                scalar_mults: p.scalar_mults,
                gae: (p.adds + p.doubles) as f64,
                wall_ns: Some(p.wall_ns),
                native: BTreeMap::from([("native_meter_attempts_uncharged".into(), p.meter_steps)]),
                work_native: BTreeMap::new(),
            }
        })
        .collect();
    for (name, value) in [
        ("native_steps", meter.used),
        ("selector_decodes", statistics.decodes),
        ("selector_cache_hits", statistics.cache_hits),
        ("derived_checks", statistics.derived_checks),
        (
            "functional_assignments",
            counter.statistics.functional_assignments,
        ),
        ("edge_assignments", counter.statistics.edge_assignments),
        ("witness_searches", counter.statistics.witness_searches),
    ] {
        counters.insert(name.into(), value);
    }
    let (recovered, error) = match result {
        Ok(x) => (
            Some(x.to_u128().ok_or("native scalar exceeds adapter width")?),
            None,
        ),
        Err(e @ Error::Limit(_)) => (None, Some(e.to_string())),
        Err(e) => {
            return Err(format!(
                "native input/integrity failure: {e}; partial group GAE={}; meter attempts={}",
                phases.iter().map(|p| p.gae).sum::<f64>(),
                meter.used
            ))
        }
    };
    let online=snapshot.online_wall_ns.map(|wall_ns|{
        let mut phases_ns=snapshot.online_phases_ns;
        let mut zero_phases=BTreeMap::new();
        for name in ["target_query","target_PDP","target_relation_check","target_descent","target_recovery_check"] {
            if !phases_ns.contains_key(name){phases_ns.insert(name.into(),0);zero_phases.insert(name.into(),"stage not entered; no separate descent is used".into());}
        }
        OnlineWindow{wall_ns,phases_ns,zero_phases,start_event:"first target-dependent relation query after initial bases are ready".into(),
            stop_event:"native solve return after scalar and certificate replay, or explicit failure".into(),included_stages:vec!["all attempts and seed retries".into(),"decomposition".into(),"elimination".into(),"internal replay".into()],
            mapping:"relation_target and target-caused base restarts -> query; relation_search -> PDP; relation_check -> check; elimination/final replay -> recovery; no descent".into()}
    });
    let source = BTreeMap::from([
        ("adapter", sha256_hex(include_bytes!("hashed_native.rs"))),
        (
            "lib",
            sha256_hex(include_bytes!(
                "../../../crates/hashed-ecdl-tensor/src/lib.rs"
            )),
        ),
        (
            "arithmetic",
            sha256_hex(include_bytes!(
                "../../../crates/hashed-ecdl-tensor/src/arithmetic.rs"
            )),
        ),
        (
            "curve",
            sha256_hex(include_bytes!(
                "../../../crates/hashed-ecdl-tensor/src/curve.rs"
            )),
        ),
        (
            "encoding",
            sha256_hex(include_bytes!(
                "../../../crates/hashed-ecdl-tensor/src/encoding.rs"
            )),
        ),
        (
            "network",
            sha256_hex(include_bytes!(
                "../../../crates/hashed-ecdl-tensor/src/network.rs"
            )),
        ),
        (
            "contraction",
            sha256_hex(include_bytes!(
                "../../../crates/hashed-ecdl-tensor/src/contraction.rs"
            )),
        ),
        (
            "recursive",
            sha256_hex(include_bytes!(
                "../../../crates/hashed-ecdl-tensor/src/contraction/recursive.rs"
            )),
        ),
        (
            "packing",
            sha256_hex(include_bytes!(
                "../../../crates/hashed-ecdl-tensor/src/contraction/packing.rs"
            )),
        ),
        (
            "deformation",
            sha256_hex(include_bytes!(
                "../../../crates/hashed-ecdl-tensor/src/contraction/deformation.rs"
            )),
        ),
        (
            "ring",
            sha256_hex(include_bytes!(
                "../../../crates/hashed-ecdl-tensor/src/ring.rs"
            )),
        ),
        (
            "primality",
            sha256_hex(include_bytes!(
                "../../../crates/hashed-ecdl-tensor/src/primality.rs"
            )),
        ),
        (
            "serialization",
            sha256_hex(include_bytes!(
                "../../../crates/hashed-ecdl-tensor/src/serialization.rs"
            )),
        ),
        (
            "reduction",
            sha256_hex(include_bytes!(
                "../../../crates/hashed-ecdl-tensor/src/reduction.rs"
            )),
        ),
        (
            "finite",
            sha256_hex(include_bytes!(
                "../../../crates/hashed-ecdl-tensor/src/finite.rs"
            )),
        ),
        (
            "width",
            sha256_hex(include_bytes!(
                "../../../crates/hashed-ecdl-tensor/src/width.rs"
            )),
        ),
        (
            "partial",
            sha256_hex(include_bytes!(
                "../../../crates/hashed-ecdl-tensor/src/partial.rs"
            )),
        ),
        (
            "metrics",
            sha256_hex(include_bytes!(
                "../../../crates/hashed-ecdl-tensor/src/metrics.rs"
            )),
        ),
    ]);
    Ok(SolveReport {
        recovered,
        exhausted: recovered.is_none(),
        total_gae: phases.iter().map(|p| p.gae).sum(),
        phases,
        automorphisms_used: 1,
        unpriced: unpriced_of(&counters),
        counters,
        deterministic: true,
        solve_wall_ns: wall,
        online,
        detail: json!({
            "implementation":"crates/hashed-ecdl-tensor", "native_sources":source,"backend":backend,"error":error,
            "cost_scope":"cold one-target; online begins at first query; all failures/restarts/internal replay included",
            "charge":"all native prime group calls, including identities; meter-step attempts remain unpriced diagnostics, not a complete arithmetic price",
            "limitations":"prime cofactor-one small-field adapter; no 131-bit workload support, no sub-rho claim; width limits are unknown results"
        }),
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::{
        ecbench::methods::{self, MethodSpec},
        ic_boundary::{CountedGroup, GroupOps, PrimeCurve},
    };
    fn method(backend: &str, steps: u64) -> ResolvedMethod {
        let params = BTreeMap::from([
            ("backend", backend.to_string()),
            ("m", "2".into()),
            ("trials", "16".into()),
            ("columns", "128".into()),
            ("steps", steps.to_string()),
            ("vector", "2000000".into()),
            ("bits", "1000000".into()),
            ("threshold", "11".into()),
            ("derive_last", "1".into()),
            ("smallest_first", "1".into()),
            ("cache_entries", "4096".into()),
            ("rank", "16".into()),
            ("slices", "16".into()),
            ("outer_multiplier", "1".into()),
            ("attempts", "128".into()),
        ])
        .into_iter()
        .map(|(k, v)| (k.into(), v))
        .collect();
        methods::resolve(&MethodSpec {
            id: "ic.hashed_native_v1".into(),
            params,
        })
        .unwrap()
    }
    fn instance() -> PrimeInstance {
        PrimeInstance {
            name: "public-known-answer".into(),
            curve: PrimeCurve { p: 5, a: 2, b: 1 },
            group_order: 7,
            r: 7,
            cofactor: 1,
            generator: (0, 1),
        }
    }
    fn stable(report: &SolveReport) -> serde_json::Value {
        json!({"recovered":report.recovered,"exhausted":report.exhausted,"gae":report.total_gae,
            "counters":report.counters,"detail":report.detail,"unpriced":report.unpriced,
            "phases":report.phases.iter().map(|p|json!([p.name,p.adds,p.doubles,p.scalar_mults,p.gae,p.native])).collect::<Vec<_>>()})
    }
    #[test]
    fn native_selector_and_partial_recover_and_replay_deterministic_counts_with_exclusive_clocks() {
        let inst = instance();
        for backend in ["selectors", "partial"] {
            let method = method(backend, 20_000_000);
            let a = solve(&method, &inst, PrimePoint::affine(3, 3), 701).unwrap();
            let b = solve(&method, &inst, PrimePoint::affine(3, 3), 701).unwrap();
            assert_eq!(a.recovered, Some(3));
            assert!(!a.exhausted);
            assert_eq!(stable(&a), stable(&b));
            let online = a.online.as_ref().unwrap();
            assert_eq!(online.phases_ns.values().sum::<u64>(), online.wall_ns);
            assert!(a.solve_wall_ns >= online.wall_ns);
            assert!(!a.unpriced.is_empty());
        }
    }
    #[test]
    fn native_budget_failure_keeps_counts_and_does_not_become_a_fast_success() {
        for backend in ["source", "selectors", "width", "partial"] {
            let report = solve(
                &method(backend, 0),
                &instance(),
                PrimePoint::affine(3, 3),
                1,
            )
            .unwrap();
            assert!(report.exhausted && report.recovered.is_none());
            assert!(report.detail["error"]
                .as_str()
                .unwrap()
                .contains("resource limit"));
            assert!(!report.phases.is_empty());
        }
    }
    #[test]
    fn native_group_call_unit_matches_counted_group_on_every_tiny_pair() {
        let inst = instance();
        let mut meter = Meter::new(Limits::default());
        let curve = Curve::new(
            5u8.into(),
            2u8.into(),
            1u8.into(),
            7u8.into(),
            point(inst.generator_point()),
            &mut meter,
        )
        .unwrap();
        for a in 0..7 {
            for b in 0..7 {
                let mut ignored = GroupOps::default();
                let p = inst.curve.mul(&mut ignored, inst.generator_point(), a);
                let q = inst.curve.mul(&mut ignored, inst.generator_point(), b);
                let session = metrics::Session::start().unwrap();
                let native = curve.add(&point(p), &point(q), &mut meter).unwrap();
                let tally = session.finish().unwrap();
                let mut ops = GroupOps::default();
                let expected = inst.curve.add(&mut ops, p, q);
                assert_eq!(native, point(expected));
                assert_eq!(
                    tally.phases["setup"].adds + tally.phases["setup"].doubles,
                    ops.adds + ops.doubles
                );
            }
        }
    }
}
