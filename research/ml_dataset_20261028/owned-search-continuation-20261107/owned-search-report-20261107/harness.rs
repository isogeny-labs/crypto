#![allow(dead_code)]
mod ml_dataset { pub mod solvers { pub mod koblitz {
 #[derive(Clone,Copy,Debug,PartialEq)]pub enum Stop{Phase,Observer,Overflow,Deadline,Work,Trace}
 pub struct Limits;
 pub struct Quote{pub work:u64,pub bytes:u64,pub trace:u64}
 pub fn quote_parent_storage_length(_:u64,_:bool,_:&Limits)->Result<Quote,Stop>{Ok(Quote{work:327680,bytes:2113536,trace:20480})}
 pub mod owner {pub mod boolean_search{
 use super::super::{Stop,Limits};use std::time::Instant;
 #[derive(Clone,Copy)]pub enum Failure{Admission(Stop),Expansion(()),Search(()),Audit(()),LiftStopped(()),Continuation(()),Coverage(()),Exclusions(()),Publication(Stop)}
 #[derive(Default)]pub struct Counter{pub work:u64,pub bytes:u64}
 pub struct Budget{pub native:Counter,pub checking_work:u64,pub adapter_bytes:u64,pub checking_performed_work:u64,pub trace_bytes:u64,pub additions:u64,pub stopped:Option<Stop>,pub expired:bool,pub refuse_work:bool,pub refuse_trace:bool,pub checks:u32,pub expire_at:u32}
 impl Budget{pub fn fail<T>(&mut self,s:Stop)->Result<T,Stop>{if self.stopped.is_none(){self.stopped=Some(s)}Err(s)}pub fn check(&mut self,_:Instant)->Result<(),Stop>{self.checks+=1;if let Some(s)=self.stopped{return Err(s)}if self.expired||self.checks==self.expire_at{self.stopped=Some(Stop::Deadline);return Err(Stop::Deadline)}Ok(())}pub fn reserve_checking(&mut self,w:u64,b:u64,n:Instant)->Result<(),Stop>{self.check(n)?;if self.refuse_work{self.stopped=Some(Stop::Work);return Err(Stop::Work)}self.checking_work+=w;self.adapter_bytes+=b;Ok(())}pub fn reserve_trace(&mut self,t:u64,n:Instant)->Result<(),Stop>{self.check(n)?;if self.refuse_trace{self.stopped=Some(Stop::Trace);return Err(Stop::Trace)}self.trace_bytes+=t;Ok(())}}
 #[derive(Clone,PartialEq)]pub struct Ack{pub last_event_sha256:String,pub event_count:u64}
 pub struct Sink{pub head:Ack,pub calls:std::cell::Cell<u32>}impl Sink{pub fn committed_event_position(&self)->Ack{self.calls.set(self.calls.get()+1);self.head.clone()}}
 pub struct Original{pub budget:Budget,pub map_published:bool,pub pattern:Option<u32>,pub capture_limits:Limits,pub public_sha256:[u8;32],pub factor_sha256:[u8;32],pub launch_sha256:[u8;32],pub binding:[u8;32]}
 pub struct Chunk{pub ack:Option<Ack>,pub exact:bool}pub struct Snapshot{pub complete:bool,pub failure:Option<Stop>,pub chunks:Vec<Chunk>,pub index:u64}pub struct Publication{pub failure:Option<Stop>,pub snapshots:Vec<Snapshot>}
 pub struct OwnedAttempt{pub report_taken:bool,pub original:Original,pub publication:Publication,pub sink:Sink,pub lift_records:Vec<()>,pub failure:Option<Failure>,pub accepted:bool}
 impl OwnedAttempt{pub fn fail(&mut self,f:Failure,s:Stop){self.failure=Some(f);self.original.budget.stopped=Some(s)}pub fn accepted_relation(&self)->Option<()>{self.accepted.then_some(())}pub fn conditional_coverage(&self)->Option<()>{None}pub fn verified_model_exclusions(&self)->Option<()>{None}}
 #[path="/home/ec2-user/ml-dataset-resume-20261029/owned-search-report-20261107/owned_search_report.rs"]mod report;
 #[cfg(test)]mod tests{use super::*;
 fn owner()->OwnedAttempt{let ack=Ack{last_event_sha256:"a".repeat(64),event_count:7};OwnedAttempt{report_taken:false,original:Original{budget:Budget{native:Counter{work:9,bytes:12},checking_work:0,adapter_bytes:0,checking_performed_work:2,trace_bytes:0,additions:3,stopped:None,expired:false,refuse_work:false,refuse_trace:false,checks:0,expire_at:u32::MAX},map_published:true,pattern:Some(2),capture_limits:Limits,public_sha256:[1;32],factor_sha256:[2;32],launch_sha256:[3;32],binding:[4;32]},publication:Publication{failure:None,snapshots:vec![Snapshot{complete:true,failure:None,index:0,chunks:vec![Chunk{ack:Some(ack.clone()),exact:true}]}]},sink:Sink{head:ack,calls:std::cell::Cell::new(0)},lift_records:vec![],failure:None,accepted:true}}
 #[test]fn fixed_partial_report_is_paid_and_one_shot(){let mut o=owner();let v=o.outcome().unwrap();assert_eq!(v["status"],"partial");assert_eq!(v["pattern"],2);assert_eq!(v["solver_completed"],false);assert_eq!(v["all_patterns_covered"],false);assert_eq!(v["producer_signed_relation_accepted"],true);assert_eq!(v["independent_original_source_replay"],false);assert_eq!(v["s3_custody_verified"],false);assert!(v["last_fall_degree"].is_null());assert_eq!(v["reserved_work"],9+o.original.budget.checking_work);assert!(o.original.budget.adapter_bytes>262144);assert!(serde_json::to_vec(&v).unwrap().len()<8192);assert!(o.outcome().is_none());assert_eq!(o.sink.calls.get(),1);}
 #[test]fn deadline_refuses_before_head_copy(){let mut o=owner();o.original.budget.expired=true;assert!(o.outcome().is_none());assert_eq!(o.sink.calls.get(),0);assert_eq!(o.publication.snapshots.len(),1);}
 #[test]fn exhausted_ledger_never_gets_report_allowance(){let mut o=owner();o.original.budget.stopped=Some(Stop::Work);assert!(o.outcome().is_none());assert_eq!(o.original.budget.checking_work,0);}
 #[test]fn work_admission_refuses_before_allocation(){let mut o=owner();o.original.budget.refuse_work=true;assert!(o.outcome().is_none());assert_eq!(o.sink.calls.get(),0);}
 #[test]fn trace_admission_failure_retains_work_and_custody(){let mut o=owner();o.original.budget.refuse_trace=true;assert!(o.outcome().is_none());assert!(o.original.budget.checking_work>0);assert_eq!(o.sink.calls.get(),0);assert_eq!(o.publication.snapshots.len(),1);}
 #[test]fn changed_capture_head_cannot_report(){let mut o=owner();o.sink.head.event_count+=1;assert!(o.outcome().is_none());assert_eq!(o.original.budget.stopped,Some(Stop::Observer));}
 #[test]fn incomplete_snapshot_and_unpublished_map_refuse(){let mut o=owner();o.publication.snapshots[0].complete=false;assert!(o.outcome().is_none());let mut o=owner();o.original.map_published=false;assert!(o.outcome().is_none());}
 #[test]fn final_deadline_rejects_paid_report(){let mut o=owner();o.original.budget.expire_at=4;assert!(o.outcome().is_none());assert!(o.original.budget.checking_work>0);assert_eq!(o.sink.calls.get(),1);}
 #[test]fn maximum_scalar_encoding_fits_bound(){let mut o=owner();let mut v=o.outcome().unwrap();fn maximize(v:&mut serde_json::Value){match v{serde_json::Value::Number(_)=>*v=serde_json::json!(u64::MAX),serde_json::Value::Array(a)=>for x in a{maximize(x)},serde_json::Value::Object(m)=>for x in m.values_mut(){maximize(x)},_=>{}}}maximize(&mut v);v["failure_stage"]=serde_json::json!("continuation");assert!(serde_json::to_vec(&v).unwrap().len()<=8192);}
 #[test]fn report_failure_preserves_prior_stage(){let mut o=owner();o.failure=Some(Failure::Search(()));o.original.budget.expired=true;assert!(o.outcome().is_none());assert!(matches!(o.failure,Some(Failure::Search(()))));}
 #[test]fn fixed_failure_category_does_not_copy_diagnostics(){let mut o=owner();o.failure=Some(Failure::Search(()));let v=o.outcome().unwrap();assert_eq!(v["failure_stage"],"search");}
 }
 }}}
}}
