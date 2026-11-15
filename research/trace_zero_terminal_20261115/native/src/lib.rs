//! Predeclared terminal-domain/slicing comparison; truth never reaches solvers.
use full_trace_zero_backend::{
    binary_ecc::{
        curve::{point_add, point_neg, scalar_mul},
        BinaryPoint, TraceZeroVariety,
    },
    cryptanalysis::{
        group_op_count::ops_snapshot,
        trace_zero_relations::{
            decompose_full_trace_zero_with_encoding, decompose_trace_zero_terminal_slices,
            TraceZeroMembershipEncoding as Encoding, TraceZeroRelationDisposition as Disposition,
            TraceZeroRelationLimits as Limits, TraceZeroRelationReport as Report,
        },
    },
};
use num_bigint::RandBigInt;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};
use trace_zero_experiments::{subspace_base, truth, unwire, wire, Case};
use trace_zero_leads::fixture;
pub const ARMS: [&str; 7] = [
    "constants",
    "identity-first",
    "tail-domain",
    "tail-pairs",
    "sliced",
    "sliced-aa",
    "pair-table",
];

pub fn summarize(rows: &[Value]) -> Value {
    let mut out = trace_zero_comparisons::summarize(rows);
    let mut groups: BTreeMap<String, Vec<&Value>> = BTreeMap::new();
    for r in rows {
        groups
            .entry(format!(
                "{}/n{}/d{}/{}",
                r["arm"].as_str().unwrap(),
                r["n"],
                r["dimension"],
                r["kind"].as_str().unwrap()
            ))
            .or_default()
            .push(r);
    }
    let strata:BTreeMap<_,_>=groups.into_iter().map(|(k,rs)| {
        let count=|s:&str|rs.iter().filter(|r|r["result"]["outcome"]==s).count();
        (k,json!({"rows":rs.len(),"found":count("Found"),"refuted":count("Refuted"),"inconclusive":count("Inconclusive"),
            "peak_variables":rs.iter().map(|r|r["peak_variables"].as_u64().unwrap()).max(),
            "conflicts_total":rs.iter().map(|r|r["conflicts"].as_u64().unwrap()).sum::<u64>(),
            "group_ops_total":rs.iter().map(|r|r["group_ops"].as_u64().unwrap()).sum::<u64>()}))
    }).collect();
    out["strata"] = json!(strata);
    out
}

pub fn prepare(original: Vec<Case>) -> Vec<Case> {
    let mut out = original;
    for n in [83, 131] {
        for d in [6, 8] {
            let c = fixture(n);
            let base = subspace_base(&c, d);
            assert!(base.len() >= 6);
            for (kind_i, kind) in [
                "distinct-planted-0",
                "distinct-planted-1",
                "uniform",
                "cancellation",
            ]
            .iter()
            .enumerate()
            {
                let seed = 202611150500 + n as u64 * 100 + d as u64 * 10 + kind_i as u64;
                let mut rng = StdRng::seed_from_u64(seed);
                let target = if kind.starts_with("distinct") {
                    let mut found = None;
                    for _ in 0..1000 {
                        let ids: [usize; 3] = std::array::from_fn(|_| rng.gen_range(0..base.len()));
                        let xs: Vec<_> = ids
                            .iter()
                            .map(|&i| match &base[i] {
                                BinaryPoint::Affine { x, .. } => x,
                                _ => panic!(),
                            })
                            .collect();
                        if xs[0] == xs[1] || xs[0] == xs[2] || xs[1] == xs[2] {
                            continue;
                        }
                        let t = ids
                            .iter()
                            .fold(BinaryPoint::Infinity, |s, &i| point_add(&c, &s, &base[i]));
                        if t != BinaryPoint::Infinity && !base.contains(&t) {
                            found = Some(t);
                            break;
                        }
                    }
                    found.expect("distinct-x planted control exists; otherwise amend before data")
                } else if *kind == "uniform" {
                    scalar_mul(&c, &c.generator, &rng.gen_biguint_below(&c.order))
                } else {
                    base[rng.gen_range(0..base.len())].clone()
                };
                let witness = truth(&c, &base, &target);
                if kind.starts_with("distinct") || *kind == "cancellation" {
                    assert!(witness.is_some());
                }
                out.push(Case {
                    id: format!("holdout-n{n}-d{d}-{kind}"),
                    n,
                    dimension: d,
                    kind: kind.to_string(),
                    seed,
                    base: base.iter().map(wire).collect(),
                    target: wire(&target),
                    truth: witness.is_some(),
                    truth_witness: witness,
                });
            }
        }
    }
    out
}

fn pair_reference(
    v: &TraceZeroVariety,
    b: &[BinaryPoint],
    t: &BinaryPoint,
    limits: Limits,
) -> Report {
    let mut out = Report::default();
    let deadline = limits.wall_budget.map(|d| Instant::now() + d);
    let mut table = BTreeMap::new();
    let key = |p: &BinaryPoint| serde_json::to_string(&wire(p)).unwrap();
    for (i, p) in b.iter().enumerate() {
        for (j, q) in b.iter().enumerate().skip(i) {
            if deadline.is_some_and(|d| Instant::now() >= d) {
                out.reason = Some("pair-table setup budget");
                return out;
            }
            table
                .entry(key(&point_add(v.curve(), p, q)))
                .or_insert((i, j));
        }
    }
    for (k, p) in b.iter().enumerate() {
        if deadline.is_some_and(|d| Instant::now() >= d) {
            out.reason = Some("pair-table lookup budget");
            return out;
        }
        if let Some(&(i, j)) = table.get(&key(&point_add(v.curve(), t, &point_neg(p)))) {
            out.indices = Some(vec![i, j, k]);
            out.disposition = Disposition::Found;
            return out;
        }
    }
    out.disposition = Disposition::Refuted;
    out
}

pub fn execute(case: &Case, arm: &str, fuse: Option<Duration>) -> Value {
    let start = Instant::now();
    let before = ops_snapshot();
    let c = fixture(case.n);
    let v = TraceZeroVariety::new(&c).unwrap();
    let base: Vec<_> = case.base.iter().map(|p| unwire(p, c.m)).collect();
    let target = unwire(&case.target, c.m);
    assert!(base.iter().all(|p| v.contains(p)) && v.contains(&target));
    let limits = Limits {
        conflict_budget: 2000,
        wall_budget: fuse.map(|d| d.saturating_sub(start.elapsed())),
        ..Limits::default()
    };
    let result = if fuse.is_some_and(|d| start.elapsed() >= d) {
        let mut r = Report::default();
        r.reason = Some("preprocessing budget");
        r
    } else if arm == "pair-table" {
        pair_reference(&v, &base, &target, limits)
    } else if arm == "sliced" || arm == "sliced-aa" {
        decompose_trace_zero_terminal_slices(&v, &base, &target, limits)
    } else {
        let encoding = match arm {
            "constants" => Encoding::CertifiedDomainWithConstants,
            "identity-first" => Encoding::CertifiedDomainIdentityFirst,
            "tail-domain" => Encoding::CertifiedDomainWithTail,
            "tail-pairs" => Encoding::CertifiedDomainWithTailPairs,
            _ => panic!("unknown arm"),
        };
        decompose_full_trace_zero_with_encoding(&v, &base, &target, 3, limits, encoding)
    };
    let points = result.indices.as_ref().map(|ids| {
        assert!(ids.iter().all(|&i| i < base.len()));
        ids.iter().map(|&i| base[i].clone()).collect::<Vec<_>>()
    });
    let replay = points.as_ref().is_some_and(|ps| {
        ps.len() == 3
            && ps
                .iter()
                .fold(BinaryPoint::Infinity, |s, p| point_add(&c, &s, p))
                == target
    });
    let valid = match result.disposition {
        Disposition::Found => case.truth && replay,
        Disposition::Refuted => !case.truth,
        Disposition::Inconclusive => true,
    };
    let work = ops_snapshot().since(&before);
    json!({"outcome":format!("{:?}",result.disposition),"truth":case.truth,"valid":valid,"replay":replay,
        "conflicts":result.conflicts,"peak_variables":result.peak_variables,"models":result.models,"patterns_attempted":result.patterns_attempted,
        "reason":result.reason,"points":points.as_ref().map(|ps|ps.iter().map(wire).collect::<Vec<_>>()),
        "all_group_ops":work,"charged_group_add_double_count":work.adds+work.doubles,
        "cost_scope":"Includes actual group work in setup/terminal translations/solver lifting/final replay; field/ANF and SAT work unpriced. Pair table rebuilt per query, no free setup or cached truth."})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_new_oracles_match_every_small_target_with_signed_and_multiple_x_bases() {
        let c = fixture(5);
        let all = subspace_base(&c, 4);
        let g = all[0].clone();
        let two = scalar_mul(&c, &g, &2u32.into());
        for base in [
            vec![g.clone()],
            vec![g.clone(), point_neg(&g)],
            vec![g.clone(), two],
            all,
        ] {
            for k in 0..11u32 {
                let t = scalar_mul(&c, &g, &k.into());
                let w = truth(&c, &base, &t);
                let case = Case {
                    id: "unit".into(),
                    n: 5,
                    dimension: 4,
                    kind: "unit".into(),
                    seed: 0,
                    base: base.iter().map(wire).collect(),
                    target: wire(&t),
                    truth: w.is_some(),
                    truth_witness: w,
                };
                for arm in ARMS {
                    let r = execute(&case, arm, None);
                    assert_ne!(r["outcome"], "Inconclusive", "{arm}/{k}:{r}");
                    assert_eq!(r["valid"], true, "{arm}/{k}:{r}");
                }
            }
        }
    }
    #[test]
    fn sliced_zero_budgets_and_terminal_wrong_arity_are_inconclusive() {
        let c = fixture(5);
        let v = TraceZeroVariety::new(&c).unwrap();
        let b = subspace_base(&c, 4);
        let t = &b[0];
        for lim in [
            Limits {
                conflict_budget: 0,
                ..Limits::default()
            },
            Limits {
                max_models: 0,
                ..Limits::default()
            },
            Limits {
                max_domain_clauses: 0,
                ..Limits::default()
            },
            Limits {
                wall_budget: Some(Duration::ZERO),
                ..Limits::default()
            },
        ] {
            assert_eq!(
                decompose_trace_zero_terminal_slices(&v, &b, t, lim).disposition,
                Disposition::Inconclusive
            );
        }
        assert_eq!(
            decompose_full_trace_zero_with_encoding(
                &v,
                &b,
                t,
                2,
                Limits::default(),
                Encoding::CertifiedDomainWithTailPairs
            )
            .disposition,
            Disposition::Inconclusive
        );
    }
}
