pub use full_trace_zero_backend::{binary_ecc, cryptanalysis};
#[path = "../../baseline_snapshot.rs"]
pub mod baseline;
#[cfg(test)]
#[path = "../../../../src/cryptanalysis/trace_zero_linearized.rs"]
mod production_unit_tests;
use binary_ecc::{
    curve::point_add, operation_tally::OperationTally, BinaryPoint, TraceZeroVariety,
};
use cryptanalysis::{
    group_op_count::ops_snapshot,
    trace_zero_linearized::decompose_trace_zero_linearized,
    trace_zero_relations::{
        TraceZeroRelationDisposition as Disposition, TraceZeroRelationLimits as Limits,
    },
};
use serde_json::{json, Value};
use trace_zero_experiments::{unwire, wire, Case};
pub fn execute(case: &Case, arm: &str) -> Value {
    let tally = std::env::var_os("TRACE_DISABLE_TALLY")
        .is_none()
        .then(OperationTally::begin);
    let before = ops_snapshot();
    let c = trace_zero_leads::fixture(case.n);
    let v = TraceZeroVariety::new(&c).unwrap();
    let base: Vec<_> = case.base.iter().map(|p| unwire(p, c.m)).collect();
    let target = unwire(&case.target, c.m);
    let limits = Limits {
        wall_budget: None,
        ..Limits::default()
    };
    let (r, diagnostics) = if arm == "baseline" || arm == "baseline-aa" {
        let l = baseline::decompose_trace_zero_linearized(&v, &base, &target, limits);
        (
            l.relation,
            json!({"source_dimension":l.source_dimension,"product_dimension":l.product_dimension,"linear_columns":l.linear_columns,"systems":l.systems,"maximum_rank":l.maximum_rank,"maximum_nullity":l.maximum_nullity}),
        )
    } else {
        assert!(arm == "candidate" || arm == "candidate-aa");
        let l = decompose_trace_zero_linearized(&v, &base, &target, limits);
        (
            l.relation,
            json!({"source_dimension":l.source_dimension,"product_dimension":l.product_dimension,"linear_columns":l.linear_columns,"systems":l.systems,"maximum_rank":l.maximum_rank,"maximum_nullity":l.maximum_nullity}),
        )
    };
    let points = r
        .indices
        .as_ref()
        .map(|ids| ids.iter().map(|&i| base[i].clone()).collect::<Vec<_>>());
    let replay = points.as_ref().is_some_and(|ps| {
        ps.len() == 3
            && ps
                .iter()
                .fold(BinaryPoint::Infinity, |s, p| point_add(&c, &s, p))
                == target
    });
    let valid = match r.disposition {
        Disposition::Found => case.truth && replay,
        Disposition::Refuted => !case.truth,
        Disposition::Inconclusive => true,
    };
    let group = ops_snapshot().since(&before);
    let field = tally.map(OperationTally::finish).unwrap_or_default();
    json!({"outcome":format!("{:?}",r.disposition),"valid":valid,"replay":replay,"indices":r.indices,"points":points.as_ref().map(|ps|ps.iter().map(wire).collect::<Vec<_>>()),"models":r.models,"patterns_attempted":r.patterns_attempted,"patterns_refuted":r.patterns_refuted,"reason":r.reason,"diagnostics":diagnostics,"field_operations":field,"group_operations":group,"cost_scope":"Cold single query includes fixture, variety, base admission, solver, roots/lifting and final replay. Inversion informational; its inner multiplications/squarings counted. Matrix/container/integer work not priced by field counts; use whole-process Callgrind for instruction work."})
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    #[test]
    fn full_65_case_snapshot_equivalence_including_exact_witness_order() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../trace_zero_linearized_20261115/workload.json");
        let input: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        let cases: Vec<Case> = serde_json::from_value(input["cases"].clone()).unwrap();
        assert_eq!(cases.len(), 65);
        for case in cases {
            let old = execute(&case, "baseline");
            let new = execute(&case, "candidate");
            assert_eq!(old["valid"], true, "{}", case.id);
            assert_eq!(new["valid"], true, "{}", case.id);
            for key in ["outcome", "indices", "points", "models", "patterns_attempted", "patterns_refuted", "diagnostics", "reason"] {
                assert_eq!(old[key], new[key], "{}/{}", case.id, key);
            }
        }
    }
}
