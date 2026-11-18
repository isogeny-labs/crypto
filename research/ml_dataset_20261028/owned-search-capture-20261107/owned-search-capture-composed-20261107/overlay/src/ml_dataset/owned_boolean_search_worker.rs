//! Explicit v5 dispatch: one original-budget source pattern, no legacy fallback.
use super::*;
use super::super::super::WorkerSolveOutput;
use crate::ml_dataset::types::{CaseConfig,CaseData,OwnedBooleanSearchRequest,OwnedBooleanSearchPolicy};
pub(in crate::ml_dataset::solvers) fn requested(c:&CaseConfig)->bool {
    c.channels.as_ref().is_some_and(|ch|ch.owned_boolean_search.is_some()||ch.schema=="crypto.ml-dataset.channel-requests/v5")
}
pub(in crate::ml_dataset::solvers) fn selection(c:&CaseConfig)->Option<(u32,boolean_search::ProposedPolicy)>{
    let ch=c.channels.as_ref()?;
    if ch.validate().is_err()||ch.schema!="crypto.ml-dataset.channel-requests/v5" {return None}
    let OwnedBooleanSearchRequest::MatrixContinuation{pattern,policy}=ch.owned_boolean_search.as_ref()?;
    if !profile(&c.family,c.n,c.m)||c.solver!="f4"||c.factor_base_size==0||c.work_limit==0||c.degree_bound==0{return None}
    let total=c.m.checked_sub(2).and_then(|n|u32::try_from(n).ok()).and_then(|n|1u32.checked_shl(n))?;
    if *pattern>=total{return None}
    let chosen=match policy{OwnedBooleanSearchPolicy::ConservativeAffineEnvelopeV1=>boolean_search::ProposedPolicy::ConservativeAffineEnvelopeV1};
    Some((*pattern,chosen))
}
pub(in crate::ml_dataset::solvers) fn supported(c:&CaseConfig)->bool{selection(c).is_some()}
fn retained(owner:worker::Owned<'_>,stage:worker::Stage,stop:Option<Stop>,pattern:u32,total:u32)->WorkerSolveOutput{
    WorkerSolveOutput::from_original_boolean_search_failure(worker::Prepared{owner,coverage:worker::Coverage{pattern,total,checked:0},stage,stop,report_admitted:false}.detach())
}
pub(in crate::ml_dataset::solvers) fn run<'a>(original:&'a VerifiedWorkerOriginal<'a>,case:&CaseData,
    context:&execution::SolverExecutionContext,mut sink:capture::CaptureEmitter,limits:&capture::CaptureLimits)->WorkerSolveOutput{
    let (pattern,policy)=selection(original.config).expect("validated owned-search dispatch");
    let total=1u32 << (original.config.m-2);
    let mut owner=match OriginalOwner::admit(original,case,context,&sink,limits){
        Ok(o)=>o,Err(f)=>{let stop=match &f{AdmissionFailure::BeforeBudget(_)=>None,AdmissionFailure::Retained(f)=>f.first_stop()};
            return retained(worker::Owned::Admission{failure:f,sink},worker::Stage::Original,stop,pattern,total)}
    };
    let map_caps=provenance::Limits{variables:4096,codes:owner.math.codes.len(),domain_clauses:4096,work:owner.budget.native_work_limit,bytes:owner.budget.memory_limit};
    if let Err(stop)=owner.build_map(pattern,map_caps){let first=owner.budget.fail::<()>(stop).unwrap_err();return retained(worker::Owned::Map{owner,sink},worker::Stage::Map,Some(first),pattern,total)}
    if let Err(stop)=owner.persist_map(&mut sink){let first=owner.budget.fail::<()>(stop).unwrap_err();return retained(worker::Owned::Map{owner,sink},worker::Stage::MapPublication,Some(first),pattern,total)}
    WorkerSolveOutput::from_original_boolean_search(owner.into_boolean_search(sink,policy).publish_capture())
}
