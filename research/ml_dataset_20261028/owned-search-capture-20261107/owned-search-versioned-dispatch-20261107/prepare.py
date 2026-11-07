from pathlib import Path
import difflib,hashlib,json
P=Path(__file__).parent; A=P.parent/'owning-search-composed-source-20261107/assembled/src/ml_dataset';inputs=[]
def edit(n,f):
 b=(A/n).read_text();p=f(b);(P/(n+'.base')).write_text(b);(P/n).write_text(p);(P/(n+'.patch')).write_text(''.join(difflib.unified_diff(b.splitlines(True),p.splitlines(True),fromfile='a/src/ml_dataset/'+n,tofile='b/src/ml_dataset/'+n)));inputs.append(dict(path=str(A/n),sha256=hashlib.sha256(b.encode()).hexdigest()))
def types(s):
 s=s.replace('pub struct ChannelRequests {','pub struct ChannelRequests {\n    #[serde(default,skip_serializing_if="Option::is_none",deserialize_with="present_owned_search")]\n    pub owned_boolean_search:Option<OwnedBooleanSearchRequest>,')
 needle='impl ChannelRequests {'
 s=s.replace(needle,'''#[derive(Clone,Copy,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(rename_all="snake_case")]
pub enum OwnedBooleanSearchPolicy { ConservativeAffineEnvelopeV1 }
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(tag="mode",rename_all="snake_case",deny_unknown_fields)]
pub enum OwnedBooleanSearchRequest { MatrixContinuation { pattern:u32, policy:OwnedBooleanSearchPolicy } }
fn present_owned_search<'de,D:serde::Deserializer<'de>>(d:D)->Result<Option<OwnedBooleanSearchRequest>,D::Error>{OwnedBooleanSearchRequest::deserialize(d).map(Some)}
'''+needle)
 s=s.replace('    pub fn validate(&self)->Result<(),String>{','''    pub fn validate(&self)->Result<(),String>{
        if self.schema=="crypto.ml-dataset.channel-requests/v5" {
            if self.owned_boolean_search.is_none() || self.boolean_consequence.is_some() || self.sat_initial.is_some() || self.affine_boolean_row.is_some() || self.sat_search.is_some() {return Err("v5 requires exclusive owned_boolean_search".into())}
            return Ok(());
        }
        if self.owned_boolean_search.is_some(){return Err("owned search requires explicit v5".into())}
''')
 return s
edit('types.rs',types)
worker='''//! Explicit v5 dispatch: one original-budget source pattern, no legacy fallback.
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
    WorkerSolveOutput::from_original_boolean_search(owner.into_boolean_search(sink,policy))
}
'''
(P/'owned_boolean_search_worker.rs').write_text(worker)
edit('sat_original_owner.rs',lambda s:s+'\n#[path="owned_boolean_search_worker.rs"]\npub(in crate::ml_dataset::solvers) mod owned_search_worker;\n')
def solvers(s):
 needle='    // Lifetime-free actual ownership'
 s=s.replace(needle,'''    fn from_original_boolean_search_failure(custody:koblitz_m3::sat_original_owner::worker::Custody)->Self{
        Self{data:SolverData{outcome:Value::Null,artifacts:Vec::new()},stored:Vec::new(),
            residual:WorkerResidualCustody{sat:Some(custody),boolean_search_report_unavailable:true,..Default::default()}}
    }
'''+needle,1)
 needle='    if config.channels.as_ref().is_some_and(|c|c.affine_boolean_row.is_some()) {'
 s=s.replace(needle,'''    if koblitz_m3::sat_original_owner::owned_search_worker::requested(config) {
        config.channels.as_ref().unwrap().validate()?;
        if !koblitz_m3::sat_original_owner::owned_search_worker::supported(config) {
            return Ok(WorkerSolveOutput::legacy(SolverData{outcome:json!({"strategy":"original-boolean-search-unsupported","status":"unsupported","solver_completed":false,"all_patterns_negative":false,"training_export_admitted":false}),artifacts:Vec::new()}));
        }
        let context=context.ok_or("owned Boolean search requires original execution context")?;
        if LIVE_SINK.with(|slot|slot.borrow().is_some()){return Err("nested live owned search sink".into())}
        let sink=CaptureEmitter::create(directory,binding,limits.clone())?;
        return Ok(koblitz_m3::sat_original_owner::owned_search_worker::run(original,case,context,sink,&limits));
    }
'''+needle,1);return s
edit('solvers.rs',solvers)
def policy(s):
 a=s.index('fn eligible(config:');z=s.index('// Pure request gate',a)
 s=s[:a]+'''fn eligible(config: &CaseConfig, policy: ProposedPolicy) -> bool {
    super::super::owned_search_worker::selection(config).is_some_and(|(_,chosen)|chosen==policy)
}
'''+s[z:]
 s=s.replace('if !eligible(owner.original.config, policy) {','if !eligible(owner.original.config, policy)\n        || super::super::owned_search_worker::selection(owner.original.config).map(|(pattern,_)|pattern)!=owner.pattern {')
 # Remove stale unused primitive helper and its now-inapplicable request test.
 a=s.index('// Pure request gate');z=s.index('/// Called by',a);s=s[:a]+s[z:]
 a=s.index('    #[test]\n    fn new_choice_does_not_reinterpret_existing_channel_requests');s=s[:a]+'}\n'
 return s.replace('//! Proposed owner-only entry policy; no CLI schema or active dispatch is added.','//! Finite owner entry for the explicit v5 matrix-continuation request.').replace('/// Explicit source-level opt-in. A future worker request must freeze and bind\n/// this choice before dispatch. No existing v1-v4 channel implies this choice.','/// Explicit finite policy bound by the v5 request. No v1-v4 channel implies it.')
edit('original_boolean_search_policy.rs',policy)
(P/'SOURCE_BINDINGS.json').write_text(json.dumps(inputs,indent=2)+'\n')
