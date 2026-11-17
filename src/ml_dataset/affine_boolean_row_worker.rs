//! Explicit finite-row dispatch. CLI replay/export integration is a separate gate.
use super::*;
use super::super::super::{WorkerSolveOutput};
use crate::ml_dataset::types::{CaseConfig,CaseData,AffineBooleanRowRequest};

pub(in crate::ml_dataset::solvers) fn supported(c:&CaseConfig)->bool {
    let Some(ch)=&c.channels else{return false};
    let Some(AffineBooleanRowRequest::AffineUnitRow{pattern,..})=&ch.affine_boolean_row else{return false};
    ch.schema=="crypto.ml-dataset.channel-requests/v3" && profile(&c.family,c.n,c.m)
        && matches!(c.solver.as_str(),"f4"|"f5") && c.factor_base_size>0 && c.work_limit>0
        && c.m.checked_sub(2).and_then(|m|u32::try_from(m).ok()).and_then(|m|1u32.checked_shl(m)).is_some_and(|n|*pattern<n)
}
// These are finite local algorithm ceilings, never an authority to extend the
// original ledger. Both ranges and all requested binary arities share this path.
fn caps(work:u64,bytes:u64)->affine_rows::Caps {
    let work=work.min(4_000_000);let bytes=bytes.min(8_000_000);
    use provenance::boolean_equations::{self as e,affine_domain::matrix_rows as rows};
    affine_rows::Caps {
        audit:e::Limits{variables:4096,generators:4096,terms:4096,degree:64,work,bytes},
        expansion:e::Limits{variables:4096,generators:4096,terms:4096,degree:64,work,bytes},
        reduction:rows::Limits{variables:4096,generators:4096,window:1,candidates:4096,basis:4096,terms:4096,degree:64,proof:4096,wire:65536,work,bytes},
        checker:rows::CheckerLimits{variables:4096,generators:4096,polynomial_terms:4096,proof_terms:4096,accumulator_entries:4096,
            caps:[65536,262144,262144,262144,262144,65536,1024,1048576,262144]},
    }
}
fn retained(owner:worker::Owned<'_>,stage:worker::Stage,stop:Option<Stop>,pattern:u32,total:u32)->WorkerSolveOutput {
    let c=worker::Prepared{owner,coverage:worker::Coverage{pattern,total,checked:0},stage,stop,report_admitted:false}.detach();
    WorkerSolveOutput::from_original_row_failure(c)
}
pub(in crate::ml_dataset::solvers) fn run<'a>(original:&'a VerifiedWorkerOriginal<'a>,case:&CaseData,
    context:&execution::SolverExecutionContext,mut sink:capture::CaptureEmitter,limits:&capture::CaptureLimits)->WorkerSolveOutput {
    let AffineBooleanRowRequest::AffineUnitRow{pattern,row_index}=original.config.channels.as_ref().unwrap().affine_boolean_row.as_ref().unwrap();
    let (pattern,row) = (*pattern,*row_index as usize);
    let total=1u32 << (original.config.m-2); // supported() established this range
    let mut owner=match OriginalOwner::admit(original,case,context,&sink,limits){
        Ok(o)=>o,Err(f)=>{let stop=match &f{AdmissionFailure::BeforeBudget(_)=>None,AdmissionFailure::Retained(f)=>f.first_stop()};
            return retained(worker::Owned::Admission{failure:f,sink},worker::Stage::Original,stop,pattern,total)}
    };
    let row_caps=caps(owner.budget.native_work_limit,owner.budget.memory_limit);
    let map_caps=provenance::Limits{variables:4096,codes:owner.math.codes.len(),domain_clauses:4096,work:owner.budget.native_work_limit,bytes:owner.budget.memory_limit};
    if let Err(stop)=owner.build_map(pattern,map_caps){
        let first=owner.budget.fail::<()>(stop).unwrap_err();
        return retained(worker::Owned::Map{owner,sink},worker::Stage::Map,Some(first),pattern,total)
    }
    if let Err(stop)=owner.persist_map(&mut sink){
        let first=owner.budget.fail::<()>(stop).unwrap_err();
        return retained(worker::Owned::Map{owner,sink},worker::Stage::MapPublication,Some(first),pattern,total)
    }
    WorkerSolveOutput::from_original_affine_rows(owner.into_affine_row(sink,row,row_caps).publish())
}

#[cfg(test)]
mod controls {
    use super::*;
    #[test]
    fn full_requested_binary_arity_is_eligible_without_claiming_runtime_success() {
        let mut c:CaseConfig=serde_json::from_value(serde_json::json!({"family":"binary","n":23,"m":3,"twist":false,"curve_seed":0,"generator_index":0,"key_index":0,"seed":42,"factor_base_size":4,"degree_bound":3,"work_limit":16384,"solver":"f4",
            "channels":{"schema":"crypto.ml-dataset.channel-requests/v3","affine_boolean_row":{"mode":"affine_unit_row","pattern":0,"row_index":0}}})).unwrap();
        for family in ["binary","koblitz"] {for m in 2..=15 {for n in [23,31,47,61] {c.family=family.into();c.m=m;c.n=n;assert!(supported(&c));}}}
        c.family="prime".into();assert!(!supported(&c));
        c.family="binary".into();c.m=16;assert!(!supported(&c));
    }
    #[test]
    fn original_row_is_explicit_and_not_ambiguous_with_initial_sat() {
        use crate::ml_dataset::types::ChannelRequests;
        let request=serde_json::json!({"schema":"crypto.ml-dataset.channel-requests/v3","affine_boolean_row":{"mode":"affine_unit_row","pattern":0,"row_index":0}});
        let c:ChannelRequests=serde_json::from_value(request.clone()).unwrap();assert!(c.validate().is_ok());
        for old in ["crypto.ml-dataset.channel-requests/v1","crypto.ml-dataset.channel-requests/v2"] {
            let mut bad=request.clone();bad["schema"]=old.into();let c:ChannelRequests=serde_json::from_value(bad).unwrap();assert!(c.validate().is_err());
        }
        let mut bad=request;bad["sat_initial"]=serde_json::json!({"mode":"capture_and_check_initial"});
        let c:ChannelRequests=serde_json::from_value(bad).unwrap();assert!(c.validate().is_err());
    }
}
