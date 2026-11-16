//! Matched finite-panel comparisons. Truth and witnesses never reach the solver.
use full_trace_zero_backend::{
    binary_ecc::{curve::point_add, BinaryPoint, TraceZeroVariety},
    cryptanalysis::{
        group_op_count::ops_snapshot,
        trace_zero_point_chain::decompose_full_point_chain,
        trace_zero_relations::{
            decompose_full_trace_zero_with_encoding, TraceZeroMembershipEncoding as Encoding,
            TraceZeroRelationDisposition as Disposition, TraceZeroRelationLimits as Limits,
        },
        trace_zero_subspace_relations::decompose_trace_zero_subspace,
    },
};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};
use trace_zero_experiments::{unwire, wire, Case};
use trace_zero_leads::{fixture, window};

pub const ENCODING_ARMS: [&str; 5] = [
    "certified",
    "constants",
    "implicit",
    "full-point",
    "constants-aa",
];
pub const TAU_ARMS: [&str; 4] = ["direct", "tau-image", "tau-preimage", "direct-aa"];
pub fn arms(panel: &str) -> &'static [&'static str] {
    match panel {
        "encoding" => &ENCODING_ARMS,
        "tau" => &TAU_ARMS,
        _ => panic!("unknown panel"),
    }
}

pub fn execute(case: &Case, panel: &str, arm: &str, fuse: Option<Duration>) -> Value {
    let begin = Instant::now();
    let before = ops_snapshot();
    let c = fixture(case.n);
    let v = TraceZeroVariety::new(&c).unwrap();
    let original: Vec<_> = case.base.iter().map(|p| unwire(p, c.m)).collect();
    let original_target = unwire(&case.target, c.m);
    assert!(original.iter().all(|p| v.contains(p)) && v.contains(&original_target));
    let transform = |p: &BinaryPoint| match arm {
        "tau-image" => v.cofactor_image(p).unwrap(),
        "tau-preimage" => v.inverse_cofactor_image(p).unwrap(),
        _ => p.clone(),
    };
    let base: Vec<_> = original.iter().map(transform).collect();
    let target = transform(&original_target);
    assert_eq!(base.len(), original.len());
    let preprocess = ops_snapshot().since(&before);
    let mut limits = Limits {
        conflict_budget: 2000,
        max_patterns: 128,
        wall_budget: fuse.map(|d| d.saturating_sub(begin.elapsed())),
        ..Limits::default()
    };
    let mut points = None;
    let (outcome, conflicts, peak_variables, models, patterns, reason) =
        if fuse.is_some_and(|d| begin.elapsed() >= d) {
            (
                Disposition::Inconclusive,
                0,
                0,
                0,
                0,
                Some("preprocessing safety fuse"),
            )
        } else if arm == "implicit" {
            let basis = if case.kind == "negative-control" {
                let BinaryPoint::Affine { x, .. } = &original[0] else {
                    panic!()
                };
                vec![x.clone()]
            } else {
                window(&c, case.dimension)
            };
            let r = decompose_trace_zero_subspace(&v, &basis, &target, 3, limits);
            points = r.points;
            (
                r.disposition,
                r.conflicts,
                r.peak_variables,
                r.models,
                r.patterns_attempted,
                r.reason,
            )
        } else {
            let r = if arm == "full-point" {
                decompose_full_point_chain(&v, &base, &target, 3, limits)
            } else {
                // All x-only S3 patterns fit in2; point chains need their full5/2 paths.
                limits.max_patterns = 2;
                let encoding = if arm == "certified" {
                    Encoding::CertifiedDomain
                } else {
                    Encoding::CertifiedDomainWithConstants
                };
                decompose_full_trace_zero_with_encoding(&v, &base, &target, 3, limits, encoding)
            };
            if let Some(indices) = &r.indices {
                assert!(indices.iter().all(|&i| i < base.len()));
                points = Some(indices.iter().map(|&i| base[i].clone()).collect::<Vec<_>>());
            }
            (
                r.disposition,
                r.conflicts,
                r.peak_variables,
                r.models,
                r.patterns_attempted,
                r.reason,
            )
        };
    let solve_counts = ops_snapshot().since(&before);
    let mapped_replay = points.as_ref().is_some_and(|ps| {
        ps.len() == 3
            && ps.iter().all(|p| base.contains(p))
            && ps
                .iter()
                .fold(BinaryPoint::Infinity, |s, p| point_add(&c, &s, p))
                == target
    });
    let recovered: Option<Vec<_>> = points.as_ref().map(|ps| {
        ps.iter()
            .map(|p| match arm {
                "tau-image" => v.inverse_cofactor_image(p).unwrap(),
                "tau-preimage" => v.cofactor_image(p).unwrap(),
                _ => p.clone(),
            })
            .collect()
    });
    let replay = recovered.as_ref().is_some_and(|ps| {
        ps.len() == 3
            && ps.iter().all(|p| original.contains(p))
            && ps
                .iter()
                .fold(BinaryPoint::Infinity, |s, p| point_add(&c, &s, p))
                == original_target
    });
    let valid = match outcome {
        Disposition::Found => case.truth && mapped_replay && replay,
        Disposition::Refuted => !case.truth,
        Disposition::Inconclusive => true,
    };
    let total = ops_snapshot().since(&before);
    json!({"panel":panel,"outcome":format!("{outcome:?}"),"truth":case.truth,"valid":valid,
        "replay":replay,"mapped_replay":mapped_replay,"conflicts":conflicts,
        "peak_variables":peak_variables,"models":models,"patterns_attempted":patterns,"reason":reason,
        "points":recovered.as_ref().map(|ps|ps.iter().map(wire).collect::<Vec<_>>()),
        "preprocessing_group_ops":preprocess,"through_solver_group_ops":solve_counts,"all_group_ops":total,
        "charged_group_add_double_count":total.adds+total.doubles,
        "uncharged":"Field/ANF construction, SAT propagation and Frobenius squaring are not group-operation counts. Safety fuse and global cap apply; no timing or end-to-end cost claim."})
}

/// Frozen exact finite-panel tabulation, with all caps and A/A discrepancies.
pub fn summarize(rows: &[Value]) -> Value {
    let mut groups: BTreeMap<String, Vec<&Value>> = BTreeMap::new();
    for row in rows {
        for key in [
            row["arm"].as_str().unwrap().to_string(),
            format!("{}/n{}", row["arm"].as_str().unwrap(), row["n"]),
        ] {
            groups.entry(key).or_default().push(row);
        }
    }
    let totals: BTreeMap<_,_> = groups.into_iter().map(|(key, rs)| {
        let count=|s:&str|rs.iter().filter(|r|r["result"]["outcome"]==s).count();
        (key,json!({"rows":rs.len(),"found":count("Found"),"refuted":count("Refuted"),"inconclusive":count("Inconclusive"),
            "invalid":rs.iter().filter(|r|r["result"]["valid"]!=true).count(),
            "conflicts_total":rs.iter().map(|r|r["conflicts"].as_u64().unwrap()).sum::<u64>(),
            "group_ops_total":rs.iter().map(|r|r["group_ops"].as_u64().unwrap()).sum::<u64>(),
            "peak_variables":rs.iter().map(|r|r["peak_variables"].as_u64().unwrap()).max()}))
    }).collect();
    let mut aa = Vec::new();
    for b in rows
        .iter()
        .filter(|r| r["arm"].as_str().unwrap().ends_with("-aa"))
    {
        let name = b["arm"].as_str().unwrap().trim_end_matches("-aa");
        let a = rows
            .iter()
            .find(|r| {
                r["arm"] == name
                    && r["instance"] == b["instance"]
                    && r["replicate"] == b["replicate"]
            })
            .unwrap();
        if a["result"]["outcome"] != b["result"]["outcome"] || a["conflicts"] != b["conflicts"] {
            aa.push(json!({"instance":a["instance"],"replicate":a["replicate"],"a_outcome":a["result"]["outcome"],"b_outcome":b["result"]["outcome"],"a_conflicts":a["conflicts"],"b_conflicts":b["conflicts"]}));
        }
    }
    json!({"schema":"trace-zero-comparison-summary/v1","rows":rows.len(),"groups":totals,"aa_mismatches":aa,
        "scope":"Complete fixed-panel descriptive counts, repeated targets not independent samples; no timing, scaling, population or end-to-end claim"})
}

#[cfg(test)]
mod tests {
    use super::*;
    use trace_zero_experiments::{subspace_base, truth};
    #[test]
    fn new_constants_oracle_and_every_comparison_arm_match_complete_small_truth() {
        let c = fixture(5);
        let base = subspace_base(&c, 3);
        for k in 0..11u32 {
            let t = full_trace_zero_backend::binary_ecc::curve::scalar_mul(&c, &base[0], &k.into());
            let w = truth(&c, &base, &t);
            let case = Case {
                id: format!("test-{k}"),
                n: 5,
                dimension: 3,
                kind: "unit".into(),
                seed: 0,
                base: base.iter().map(wire).collect(),
                target: wire(&t),
                truth: w.is_some(),
                truth_witness: w,
            };
            for panel in ["encoding", "tau"] {
                for arm in arms(panel) {
                    let r = execute(&case, panel, arm, None);
                    assert_ne!(r["outcome"], "Inconclusive", "{arm}/{k}:{r}");
                    assert_eq!(r["valid"], true, "{arm}/{k}:{r}");
                }
            }
        }
    }
    #[test]
    fn zero_fuse_is_a_retained_inconclusive_not_a_false_refutation() {
        let c = fixture(5);
        let b = subspace_base(&c, 3);
        let t = &b[0];
        let case = Case {
            id: "zero".into(),
            n: 5,
            dimension: 3,
            kind: "unit".into(),
            seed: 0,
            base: b.iter().map(wire).collect(),
            target: wire(t),
            truth: true,
            truth_witness: None,
        };
        let r = execute(&case, "encoding", "constants", Some(Duration::ZERO));
        assert_eq!(r["outcome"], "Inconclusive");
        assert_eq!(r["valid"], true);
    }
}
