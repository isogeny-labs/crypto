//! Structural test shim, NOT an authenticated CheckedMap constructor.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum Error{InvalidOriginal,Cap,Admission,Identity,Semantics,Formula}
pub struct Quote{pub work:u64,pub bytes:u64}
pub trait Admission{fn reserve(&mut self,q:Quote)->Result<(),Error>;fn check(&mut self)->Result<(),Error>;}
pub enum Constraint{Clause{literals:Vec<i32>},Xor{wires:Vec<u32>,rhs:bool}}
pub struct Identity{n:u32,m:usize,a:Vec<u32>,b:Vec<u32>,target:Option<Vec<u32>>,low_terms:Vec<u32>,codes:Vec<Vec<u32>>,pattern:u32}
pub struct Map{identity:Identity,variables:u32,formula:Vec<Constraint>,blocks:Vec<Vec<u32>>}
pub struct CheckedMap(Map);
impl CheckedMap{fn variables(&self)->u32{self.0.variables}}
#[path="checked_model.shim.rs"]pub mod checked_model;
mod signed_lift;mod sparse;mod scheduler;mod bridge;mod bqi;mod check;mod continuation;mod exclusions;

#[cfg(test)]mod tests{
use super::*;use sparse::{Owner,Charge,Polynomial};use checked_model::VerificationAdmission;use std::time::{Instant,Duration};
 fn map(a:u32,m:usize,xs:&[u64],target:Option<u64>,intermediate:&[u64],pattern:u32)->(CheckedMap,Vec<bool>){let n=3;let mut blocks=vec![];let mut model=vec![];for &x in xs.iter().chain(intermediate).chain(std::iter::once(&target.unwrap_or(0))){let mut block=vec![];for k in 0..n{block.push(model.len()as u32+1);model.push(x>>k&1!=0)}blocks.push(block)}let formula=model.iter().enumerate().map(|(i,v)|Constraint::Clause{literals:vec![if *v{i as i32+1}else{-(i as i32+1)}]}).collect();let mut codes=xs.to_vec();codes.sort();codes.dedup();(CheckedMap(Map{identity:Identity{n,m,a:vec![a],b:vec![1],target:target.map(|v|vec![v as u32]),low_terms:vec![0,1],codes:codes.into_iter().map(|v|vec![v as u32]).collect(),pattern},variables:model.len()as u32,formula,blocks}),model)}
fn limits()->continuation::Limits{continuation::Limits{partitions:64,work:100_000_000_000,bytes:100_000_000_000,search:scheduler::search::SearchLimits{nodes:32,native:scheduler::Limits{batch_rows:128,batch_ops:1024,batches:32,variables:256,generators:256,basis:256,tasks:1024,terms:256,proof:1024,degree:256,work:20_000_000,bytes:200_000_000}},audit:check::Limits{nodes:32,work:2_000_000_000,bytes:400_000_000,certificate:bridge::Limits{variables:256,generators:256,terms:256,degree:256,proof:1024,wire:65536,work:1_000_000_000,bytes:200_000_000},checker:bqi::Limits::software()}}}

#[derive(Default)]struct Ledger{work:u64,bytes:u64,checks:u64,adds:u64,stop:Option<u64>}
impl Owner for Ledger{fn reserve(&mut self,q:Charge)->Result<(),()>{self.work+=q.work;self.bytes+=q.bytes;Ok(())}fn check(&mut self)->Result<(),()>{self.checks+=1;if self.stop.is_some_and(|s|self.checks>=s){Err(())}else{Ok(())}}}
impl Admission for Ledger{fn reserve(&mut self,q:Quote)->Result<(),Error>{Owner::reserve(self,Charge{work:q.work,bytes:q.bytes}).map_err(|_|Error::Admission)}fn check(&mut self)->Result<(),Error>{Owner::check(self).map_err(|_|Error::Admission)}}
impl VerificationAdmission for Ledger{fn addition(&mut self)->Result<(),Error>{self.adds+=1;Ok(())}}
#[derive(Clone,Copy,Debug)]enum Stop{Refused}
include!("owning_projection.rs");
fn end()->Instant{Instant::now()+Duration::from_secs(20)}
fn el()->exclusions::Limits{exclusions::Limits{records:32,work:100_000_000_000,bytes:100_000_000_000,coverage:limits()}}
fn fixture()->(CheckedMap,Vec<Polynomial>,continuation::Attempt,Vec<LiftRecord>){fixture_free(false)}
fn fixture_free(free:bool)->(CheckedMap,Vec<Polynomial>,continuation::Attempt,Vec<LiftRecord>){
 let(mut map,model)=map(0,2,&[1,1],None,&[],0);if free{map.0.variables+=1}let g=model.iter().enumerate().map(|(i,v)|if *v{vec![vec![],vec![i as u32]]}else{vec![vec![i as u32]]}).collect::<Vec<_>>();let mut o=Ledger::default();let mut s=continuation::Session::new(map.variables(),&g,limits(),end(),&mut o);let mut records=Vec::new();
 loop{match s.next().unwrap(){continuation::Outcome::CandidateRetained=>{let p=s.attempt.active.unwrap();let model=s.candidate().unwrap().to_vec();let record=records.len()as u64;let call=s.with_candidate_owner(|x,deadline,owner|signed_lift::verify_candidate(&map,x,&[(1,0)],None,&mut checked_model::Progress::default(),deadline,owner));records.push(LiftRecord{partition:p,record,model,progress:checked_model::Progress::default(),callback_result:Some(call.result),returned:call.value.map(Ok)});s.exclude(record).unwrap()},continuation::Outcome::RemainingBooleanModelsExhaustedUnderRecordedExclusions=>break,_=>panic!()}}
 {let a=s.into_attempt();(map,g,a,records)}
}
fn go(map:&CheckedMap,g:&[Polynomial],a:&continuation::Attempt,r:&Vec<LiftRecord>,domain:&[(u64,u64)],o:&mut Ledger)->exclusions::Audit{exclusions::verify(map.variables(),g,map,domain,None,a,&RetainedRecords(r),el(),end(),o)}
#[test]fn exact_record_pairing_replays_actual_binary_arithmetic(){let(map,g,a,r)=fixture();assert_eq!(r.len(),1);let mut o=Ledger::default();let v=go(&map,&g,&a,&r,&[(1,0)],&mut o);assert_eq!(v.result,Ok(()));assert_eq!(v.replays.len(),1);assert_eq!(v.replays[0].outcome,Some(signed_lift::Outcome::ExhaustedForModel));assert_eq!(v.paired,vec![true]);assert!(o.adds>0);assert_eq!(v.charge.work,o.work);assert_eq!(v.charge.bytes,o.bytes);}
#[test]fn missing_extra_wrong_index_partition_model_and_callback_rejected(){for mode in 0..7{let(map,g,a,mut r)=fixture();match mode{0=>r.clear(),1=>r.push(LiftRecord{partition:0,record:1,model:vec![],progress:checked_model::Progress::default(),callback_result:Some(Ok(())),returned:Some(Ok(signed_lift::Outcome::ExhaustedForModel))}),2=>r[0].record=8,3=>r[0].partition=8,4=>r[0].model[0]=!r[0].model[0],5=>r[0].callback_result=Some(Err(continuation::Failure::Owner)),_=>r[0].returned=Some(Ok(signed_lift::Outcome::VerifiedSignedRelation))};assert!(go(&map,&g,&a,&r,&[(1,0)],&mut Ledger::default()).result.is_err(),"mode {mode}");}}
#[test]fn stored_exhaustion_cannot_override_changed_signed_domain_or_map(){let(mut map,g,a,r)=fixture();let v=go(&map,&g,&a,&r,&[(1,0),(1,1)],&mut Ledger::default());assert_eq!(v.result,Err(exclusions::Failure::Replay));assert_eq!(v.replays[0].outcome,Some(signed_lift::Outcome::VerifiedSignedRelation));map.0.formula[0]=Constraint::Clause{literals:vec![-1]};assert_eq!(go(&map,&g,&a,&r,&[(1,0)],&mut Ledger::default()).result,Err(exclusions::Failure::Replay));}
#[test]fn coverage_mutation_and_bounds_cannot_pass(){let(map,g,mut a,r)=fixture();a.partitions[1].prefix[0]=!a.partitions[1].prefix[0];assert_eq!(go(&map,&g,&a,&r,&[(1,0)],&mut Ledger::default()).result,Err(exclusions::Failure::Coverage));let(map,g,a,r)=fixture();assert_eq!(exclusions::verify(9,&g,&map,&[(1,0)],None,&a,&RetainedRecords(&r),el(),Instant::now(),&mut Ledger::default()).result,Err(exclusions::Failure::Deadline));let mut l=el();l.records=0;assert_eq!(exclusions::verify(9,&g,&map,&[(1,0)],None,&a,&RetainedRecords(&r),l,end(),&mut Ledger::default()).result,Err(exclusions::Failure::Cap));}
#[test]fn parent_refusals_keep_replayed_progress_and_cumulative_charges(){let(map,g,a,r)=fixture();let mut base=Ledger::default();assert_eq!(go(&map,&g,&a,&r,&[(1,0)],&mut base).result,Ok(()));let mut retained=false;for stop in [1,10,base.checks/2,base.checks-2,base.checks-1,base.checks]{let mut o=Ledger{stop:Some(stop),..Default::default()};let v=go(&map,&g,&a,&r,&[(1,0)],&mut o);assert_eq!(v.result,Err(exclusions::Failure::Owner));assert_eq!(v.charge.work,o.work);assert_eq!(v.charge.bytes,o.bytes);retained|=v.replays.iter().any(|p|!p.progress.lift_choices.is_empty());}assert!(retained);}
#[test]fn two_exact_models_require_two_unique_exclusion_records(){let(map,g,mut a,r)=fixture_free(true);assert_eq!(r.len(),2);assert_ne!(r[0].model,r[1].model);assert_eq!(go(&map,&g,&a,&r,&[(1,0)],&mut Ledger::default()).result,Ok(()));let p=r[1].partition;a.partitions[p].exclusion_record=Some(0);assert_eq!(go(&map,&g,&a,&r,&[(1,0)],&mut Ledger::default()).result,Err(exclusions::Failure::Pairing));}

#[test]fn owning_projection_does_not_fabricate_success_from_error_or_absence(){
 let(map,g,a,mut r)=fixture();
 for value in [None,Some(Err(Stop::Refused))]{r[0].returned=value;assert_eq!(go(&map,&g,&a,&r,&[(1,0)],&mut Ledger::default()).result,Err(exclusions::Failure::Callback));}
 r[0].returned=Some(Ok(signed_lift::Outcome::ExhaustedForModel));r[0].callback_result=None;
 assert_eq!(go(&map,&g,&a,&r,&[(1,0)],&mut Ledger::default()).result,Err(exclusions::Failure::Callback));
 assert!(exclusions::Records::get(&RetainedRecords(&r),r.len()).is_none());
}

}
