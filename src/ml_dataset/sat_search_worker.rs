//! Explicit original-owned search dispatch; no initial-only fallback.
use super::*;
use initial_native::checkpoint::search;
use crate::ml_dataset::types::{CaseConfig,CaseData,SatSearchRequest};
pub(in crate::ml_dataset::solvers) fn requested(c:&CaseConfig)->bool{c.channels.as_ref().is_some_and(|r|r.schema=="crypto.ml-dataset.channel-requests/v4"&&r.sat_search.is_some())}
pub(in crate::ml_dataset::solvers) fn supported(c:&CaseConfig)->bool{requested(c)&&c.solver=="sat"&&profile(&c.family,c.n,c.m)}
pub(in crate::ml_dataset::solvers) enum Custody { Search(search::ReportedSearch), BeforeSearch(worker::Custody) }
pub(in crate::ml_dataset::solvers) fn run<'a>(original:&'a VerifiedWorkerOriginal<'a>,case:&CaseData,context:&execution::SolverExecutionContext,sink:capture::CaptureEmitter,limits:&capture::CaptureLimits)->Custody {
    let prepared=worker::prepare(original,case,context,sink,limits);
    let worker::Prepared{owner,coverage,stage,stop,report_admitted}=prepared;
    match owner {
        worker::Owned::Ready(initial)=>{
            let Some(SatSearchRequest::CaptureAndCheckModel{events,conflicts})=original.config.channels.as_ref().and_then(|c|c.sat_search.as_ref()) else {unreachable!("dispatch requires original explicit request")};
            Custody::Search(search::continue_reported(initial.detach(),search::SearchCaps{events:*events as usize,conflicts:*conflicts}))
        },
        owner=>Custody::BeforeSearch(worker::Prepared{owner,coverage,stage,stop,report_admitted}.detach()),
    }
}
impl Custody {
    pub(in crate::ml_dataset::solvers) fn outcome(&mut self)->Option<Value>{match self{Self::Search(s)=>s.outcome(),Self::BeforeSearch(_)=>None}}
}
