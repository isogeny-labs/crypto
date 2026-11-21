#[cfg(test)]
mod context_unit;
#[cfg(test)]
mod word_unit;
pub use full_trace_zero_backend::{binary_ecc, cryptanalysis};
#[path = "../../baseline_snapshot.rs"]
pub mod baseline;
#[cfg(test)]
#[path = "../../../../src/cryptanalysis/trace_zero_linearized.rs"]
mod projected_unit_tests;
#[cfg(test)]
mod tests {
    use super::binary_ecc::{BinaryPoint, F2mElement, TraceZeroVariety};
    #[test]
    fn half_trace_product_elimination_is_exact_for_all_small_s3_candidates() {
        let c = trace_zero_leads::fixture(5);
        let t = TraceZeroVariety::new(&c).unwrap();
        let mut points = Vec::new();
        for x in 0..32u32 {
            for y in 0..32u32 {
                let p = BinaryPoint::Affine {
                    x: F2mElement::from_biguint(&x.into(), 5),
                    y: F2mElement::from_biguint(&y.into(), 5),
                };
                if t.contains(&p) {
                    points.push(p);
                }
            }
        }
        assert_eq!(points.len(), 10);
        for p in points {
            let BinaryPoint::Affine { x: z, y } = p else {
                unreachable!()
            };
            let ir = &c.irreducible;
            let offset = y.add(&z.mul(&z.half_trace(ir).unwrap(), ir));
            assert_eq!(
                offset.square(ir).add(&z.mul(&offset, ir)),
                F2mElement::one(5)
            );
            for u in 0..32u32 {
                let u = F2mElement::from_biguint(&u.into(), 5);
                if u.trace(ir) != 0 {
                    continue;
                }
                let base = offset.add(&z.mul(&u.square(ir).half_trace(ir).unwrap(), ir));
                for v in 0..32u32 {
                    let v = F2mElement::from_biguint(&v.into(), 5);
                    let old = v
                        .square(ir)
                        .add(&z.mul(&v, ir))
                        .add(&z.square(ir).mul(&u.square(ir), ir))
                        .add(&c.b)
                        .is_zero();
                    let new = v == base || v == base.add(&z);
                    assert_eq!(old, new);
                }
            }
        }
    }
}

use binary_ecc::{
    curve::point_add, operation_tally::OperationTally, BinaryPoint, TraceZeroVariety,
};
use cryptanalysis::{
    group_op_count::ops_snapshot,
    trace_zero_relations::{
        TraceZeroRelationDisposition as Disposition, TraceZeroRelationLimits as Limits,
    },
};
use serde_json::{json, Value};
use trace_zero_experiments::{unwire, wire, Case};
unsafe extern "C" {
    fn trace_phase_dump(phase: *const std::ffi::c_char);
}
fn dump(phase: &'static [u8]) {
    if std::env::var_os("TRACE_NO_PHASE_DUMPS").is_none() {
        unsafe { trace_phase_dump(phase.as_ptr().cast()) };
    }
}
pub fn execute(case: &Case, arm: &str) -> Value {
    let tally = std::env::var_os("TRACE_DISABLE_TALLY")
        .is_none()
        .then(OperationTally::begin);
    let before = ops_snapshot();
    let c = trace_zero_leads::fixture(case.n);
    let v = TraceZeroVariety::new(&c).unwrap();
    if arm.contains("eager") {
        v.prepare_symbolic_models();
    }
    let b: Vec<_> = case.base.iter().map(|p| unwire(p, c.m)).collect();
    let target = unwire(&case.target, c.m);
    let setup = tally.map(OperationTally::finish).unwrap_or_default();
    let tally = std::env::var_os("TRACE_DISABLE_TALLY")
        .is_none()
        .then(OperationTally::begin);
    dump(b"setup\0");
    let limits = Limits {
        wall_budget: None,
        ..Limits::default()
    };
    macro_rules! projected_pack {($l:expr)=>{{let l=$l;(l.relation,json!({"source_dimension":l.source_dimension,"product_dimension":l.product_dimension,"linear_columns":l.linear_columns,"systems":l.systems,"maximum_rank":l.maximum_rank,"maximum_nullity":l.maximum_nullity}),json!({"columns":l.projected_columns,"systems":l.projected_systems,"rows":l.projected_rows,"inconsistent":l.projected_inconsistent}))}};}
    let (r, common, projected) = if arm.starts_with("baseline") {
        projected_pack!(baseline::decompose_trace_zero_linearized(
            &v, &b, &target, limits
        ))
    } else {
        assert_eq!(arm, "lookup");
        projected_pack!(
            cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized(
                &v, &b, &target, limits
            )
        )
    };
    dump(b"oracle\0");
    let points = r
        .indices
        .as_ref()
        .map(|ids| ids.iter().map(|&i| b[i].clone()).collect::<Vec<_>>());
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
    let work = tally.map(OperationTally::finish).unwrap_or_default();
    let group = ops_snapshot().since(&before);
    json!({"outcome":format!("{:?}",r.disposition),"valid":valid,"replay":replay,"indices":r.indices,"points":points.as_ref().map(|ps|ps.iter().map(wire).collect::<Vec<_>>()),"models":r.models,"patterns_attempted":r.patterns_attempted,"patterns_refuted":r.patterns_refuted,"reason":r.reason,"common":common,"projected":projected,"setup_field":setup,"oracle_and_wrapper_replay_field":work,"group":group})
}

#[cfg(test)]
mod full_equivalence {
    use super::*;
    #[test]
    fn all_65_cases_preserve_original_witnesses_across_current_and_lookup_algorithms() {
        let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../trace_zero_linearized_20261115/workload.json");
        let input: Value = serde_json::from_slice(&std::fs::read(p).unwrap()).unwrap();
        let cases: Vec<Case> = serde_json::from_value(input["cases"].clone()).unwrap();
        assert_eq!(cases.len(), 65);
        for case in cases {
            let old = execute(&case, "baseline");
            assert_eq!(old["valid"], true);
            for arm in ["lookup", "baseline-aa"] {
                let new = execute(&case, arm);
                assert_eq!(new["valid"], true, "{}/{arm}", case.id);
                for key in [
                    "outcome",
                    "indices",
                    "points",
                    "models",
                    "patterns_attempted",
                    "patterns_refuted",
                    "reason",
                    "common",
                    "projected",
                ] {
                    assert_eq!(old[key], new[key], "{}/{arm}/{key}", case.id);
                }
            }
        }
    }
}
