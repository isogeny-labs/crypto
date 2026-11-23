use full_trace_zero_backend::{
    binary_ecc::{
        curve::{point_add, scalar_mul},
        BinaryPoint, TraceZeroVariety,
    },
    cryptanalysis::{
        group_op_count::ops_snapshot,
        trace_zero_linearized::decompose_trace_zero_linearized,
        trace_zero_relations::{
            decompose_trace_zero_terminal_slices_prepared,
            TraceZeroRelationDisposition as Disposition, TraceZeroRelationLimits as Limits,
        },
    },
};
use num_bigint::RandBigInt;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::{json, Value};
use std::time::{Duration, Instant};
use trace_zero_experiments::{subspace_base, truth, unwire, wire, Case};
use trace_zero_leads::fixture;
pub const ARMS: [&str; 5] = [
    "sliced-original",
    "prepared-sliced",
    "linearized",
    "linearized-aa",
    "pair-table",
];
pub fn prepare(original: Vec<Case>) -> Vec<Case> {
    let mut out = original;
    for n in [83, 131] {
        for d in [10, 12] {
            let c = fixture(n);
            let base = subspace_base(&c, d);
            for (i, kind) in [
                "distinct-planted-0",
                "distinct-planted-1",
                "uniform",
                "cancellation",
            ]
            .iter()
            .enumerate()
            {
                let seed = 202611151000 + n as u64 * 100 + d as u64 * 10 + i as u64;
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
                    found.expect("pre-data distinct planted target")
                } else if *kind == "uniform" {
                    scalar_mul(&c, &c.generator, &rng.gen_biguint_below(&c.order))
                } else {
                    base[rng.gen_range(0..base.len())].clone()
                };
                let w = truth(&c, &base, &target);
                if *kind != "uniform" {
                    assert!(w.is_some());
                }
                out.push(Case {
                    id: format!("scale-n{n}-d{d}-{kind}"),
                    n,
                    dimension: d,
                    kind: kind.to_string(),
                    seed,
                    base: base.iter().map(wire).collect(),
                    target: wire(&target),
                    truth: w.is_some(),
                    truth_witness: w,
                });
            }
        }
    }
    out
}
pub fn execute(case: &Case, arm: &str, fuse: Option<Duration>) -> Value {
    if arm == "sliced-original" || arm == "pair-table" {
        let mut r = trace_zero_terminal::execute(
            case,
            if arm == "sliced-original" {
                "sliced"
            } else {
                "pair-table"
            },
            fuse,
        );
        r["linear_columns"] = json!(0);
        return r;
    }
    let start = Instant::now();
    let before = ops_snapshot();
    let c = fixture(case.n);
    let v = TraceZeroVariety::new(&c).unwrap();
    let b: Vec<_> = case.base.iter().map(|p| unwire(p, c.m)).collect();
    let t = unwire(&case.target, c.m);
    let lim = Limits {
        conflict_budget: 2000,
        wall_budget: fuse.map(|d| d.saturating_sub(start.elapsed())),
        ..Limits::default()
    };
    let (r, diag) = if arm == "prepared-sliced" {
        (
            decompose_trace_zero_terminal_slices_prepared(&v, &b, &t, lim),
            json!({"linear_columns":0}),
        )
    } else if arm == "linearized" || arm == "linearized-aa" {
        let l = decompose_trace_zero_linearized(&v, &b, &t, lim);
        (
            l.relation,
            json!({"source_dimension":l.source_dimension,"product_dimension":l.product_dimension,"linear_columns":l.linear_columns,"systems":l.systems,"maximum_rank_consistent":l.maximum_rank,"maximum_nullity_consistent":l.maximum_nullity}),
        )
    } else {
        panic!("unknown arm")
    };
    let points = r.indices.as_ref().map(|ids| {
        assert!(ids.iter().all(|&i| i < b.len()));
        ids.iter().map(|&i| b[i].clone()).collect::<Vec<_>>()
    });
    let replay = points.as_ref().is_some_and(|ps| {
        ps.len() == 3
            && ps
                .iter()
                .fold(BinaryPoint::Infinity, |s, p| point_add(&c, &s, p))
                == t
    });
    let valid = match r.disposition {
        Disposition::Found => case.truth && replay,
        Disposition::Refuted => !case.truth,
        Disposition::Inconclusive => true,
    };
    let work = ops_snapshot().since(&before);
    json!({"outcome":format!("{:?}",r.disposition),"truth":case.truth,"valid":valid,"replay":replay,"points":points.as_ref().map(|ps|ps.iter().map(wire).collect::<Vec<_>>()),
  "conflicts":r.conflicts,"peak_variables":r.peak_variables,"models":r.models,"patterns_attempted":r.patterns_attempted,"reason":r.reason,
  "charged_group_add_double_count":work.adds+work.doubles,"all_group_ops":work,"linear_columns":diag["linear_columns"],"linear_systems":diag,
  "cost_scope":"Actual group work includes setup/admission/lifting/replay; linearization, field, SAT and quadratic root work not fully priced by that unit. No runtime claim."})
}
pub fn summarize(rows: &[Value]) -> Value {
    let mut out = trace_zero_terminal::summarize(rows);
    for r in rows {
        let arm = r["arm"].as_str().unwrap();
        let columns = r["linear_columns"].as_u64().unwrap_or(0);
        for key in [arm.to_string(), format!("{arm}/n{}", r["n"])] {
            let old = out["groups"][&key]["max_linear_columns"]
                .as_u64()
                .unwrap_or(0);
            out["groups"][&key]["max_linear_columns"] = json!(old.max(columns));
        }
        let key = format!(
            "{arm}/n{}/d{}/{}",
            r["n"],
            r["dimension"],
            r["kind"].as_str().unwrap()
        );
        let old = out["strata"][&key]["max_linear_columns"]
            .as_u64()
            .unwrap_or(0);
        out["strata"][&key]["max_linear_columns"] = json!(old.max(columns));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn linear_and_prepared_oracles_match_complete_small_truth_including_sign_restrictions() {
        let c = fixture(5);
        let all = subspace_base(&c, 4);
        let g = all[0].clone();
        for base in [
            vec![g.clone()],
            vec![
                g.clone(),
                full_trace_zero_backend::binary_ecc::curve::point_neg(&g),
            ],
            vec![g.clone(), scalar_mul(&c, &g, &2u32.into())],
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
    fn zero_models_deadline_or_columns_are_inconclusive_not_refuted() {
        let c = fixture(5);
        let v = TraceZeroVariety::new(&c).unwrap();
        let b = subspace_base(&c, 4);
        for lim in [
            Limits {
                max_models: 0,
                ..Limits::default()
            },
            Limits {
                max_variables: 0,
                ..Limits::default()
            },
            Limits {
                wall_budget: Some(Duration::ZERO),
                ..Limits::default()
            },
        ] {
            assert_eq!(
                decompose_trace_zero_linearized(&v, &b, &b[0], lim)
                    .relation
                    .disposition,
                Disposition::Inconclusive
            );
        }
    }
    #[test]
    fn deficient_spaces_obey_model_caps_without_false_refutations() {
        let c = fixture(5);
        let v = TraceZeroVariety::new(&c).unwrap();
        let b = subspace_base(&c, 4);
        let mut capped = 0;
        for k in 0..11u32 {
            let t = scalar_mul(&c, &b[0], &k.into());
            assert!(truth(&c, &b, &t).is_some());
            let r = decompose_trace_zero_linearized(
                &v,
                &b,
                &t,
                Limits {
                    max_models: 1,
                    ..Limits::default()
                },
            );
            assert_ne!(r.relation.disposition, Disposition::Refuted);
            if r.relation.disposition == Disposition::Inconclusive {
                capped += 1;
            }
        }
        assert!(capped > 0, "must exercise a deficient-space cap");
    }
}
