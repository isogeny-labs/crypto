from pathlib import Path
p=Path(__file__).parent
s=(p/'capture_encoding.rs').read_text();s=s[s.index('#[derive(Clone,Copy)]'):s.index('macro_rules! object')]
(p/'borrow_encoding.rs').write_text('use super::*;\n'+s+'\n#[cfg(test)]pub(super) fn state(s:&EncodingSource)->(usize,usize){(s.continuation.map_or(0,|a|a.rows),s.lifts.len())}\n')
h=r'''#![allow(dead_code)]
mod ml_dataset {pub mod solvers {mod owning {
use std::time::Instant;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]enum Stop{Refused}
#[derive(Clone,Copy)]enum OwnerFailure{Failed}
struct Budget{stopped:Option<Stop>,spent:u64}
struct LiftRecord;
mod provenance{#[derive(Clone,Copy)]pub enum Error{Failed}}
mod equations{
 pub type Polynomial=Vec<Vec<u32>>;#[derive(Clone,Copy)]pub enum Failure{Failed}#[derive(Clone,Copy)]pub enum Stage{Retained}#[derive(Clone,Copy)]pub struct Charge;
 pub struct OwnedExpansion{pub equations:Vec<Polynomial>,pub partial:Polynomial,pub scratch:Polynomial}
 impl OwnedExpansion{pub fn retained(&self)->(&Polynomial,&Polynomial,Stage,usize,usize){(&self.partial,&self.scratch,Stage::Retained,0,0)}pub fn variables(&self)->u32{2}pub fn equations(&self)->&[Polynomial]{&self.equations}pub fn status(&self)->Result<(),Failure>{Ok(())}pub fn owner_failure(&self)->Option<super::provenance::Error>{None}pub fn charged(&self)->Charge{Charge}}
}
mod continuation{pub struct Attempt{pub rows:usize}pub struct CoverageAudit;}
mod exclusions{pub struct Audit;}
mod capture{pub struct CommittedEventPosition;pub struct CaptureEmitter{pub published:usize}pub struct CaptureLimits;}
mod capture_projection{use super::*;pub struct SourceIdentity<'a>{pub public_sha256:&'a[u8;32],pub factor_sha256:&'a[u8;32],pub launch_sha256:&'a[u8;32],pub binding_sha256:&'a[u8;32],pub map_bytes:&'a[u8],pub map_ack:Option<&'a capture::CommittedEventPosition>,pub publication_attempted:bool,pub map_published:bool,pub pattern:Option<u32>}pub enum Status{Incomplete}pub struct View;impl View{pub fn of(_:&OwnedAttempt)->Self{Self}pub fn status(&self)->Status{Status::Incomplete}}}
struct Math{signed:Vec<(u64,u64)>,signed_target:Option<(u64,u64)>}
struct Original{public_sha256:[u8;32],factor_sha256:[u8;32],launch_sha256:[u8;32],binding:[u8;32],map_bytes:Vec<u8>,map_ack:Option<capture::CommittedEventPosition>,publication_attempted:bool,map_published:bool,pattern:Option<u32>,math:Math,budget:Budget}
struct OwnedAttempt{original:Original,sink:capture::CaptureEmitter,diagnostic:Option<String>,expansion:Option<equations::OwnedExpansion>,normalized:Vec<equations::Polynomial>,normalization_generator:usize,normalization_term:usize,normalization_sort:usize,continuation:Option<continuation::Attempt>,coverage:Option<continuation::CoverageAudit>,exclusion_audit:Option<exclusions::Audit>,publication:publication::Publication,original_deadline:Instant,failure:Option<OwnerFailure>,lift_records:Vec<LiftRecord>,accepted_record:Option<usize>}
#[path="REPLACE"]mod capture_encoding;
mod publication{use super::*;#[derive(Default)]pub struct Publication{pub states:Vec<(usize,usize)>}pub(super) fn checkpoint_parts(s:&capture_encoding::EncodingSource,b:&mut Budget,sink:&mut capture::CaptureEmitter,p:&mut Publication,_:&capture::CaptureLimits)->Result<(),Stop>{if let Some(x)=b.stopped{return Err(x)}b.spent+=1;p.states.push(capture_encoding::state(s));sink.published+=1;Ok(())}}
#[cfg(test)]mod tests{use super::*;
fn owner()->OwnedAttempt{OwnedAttempt{original:Original{public_sha256:[1;32],factor_sha256:[2;32],launch_sha256:[3;32],binding:[4;32],map_bytes:vec![5],map_ack:Some(capture::CommittedEventPosition),publication_attempted:true,map_published:true,pattern:Some(3),math:Math{signed:vec![(7,8)],signed_target:Some((9,10))},budget:Budget{stopped:None,spent:0}},sink:capture::CaptureEmitter{published:0},diagnostic:None,expansion:Some(equations::OwnedExpansion{equations:vec![vec![vec![1]]],partial:vec![],scratch:vec![]}),normalized:vec![vec![vec![1]]],normalization_generator:1,normalization_term:1,normalization_sort:1,continuation:None,coverage:None,exclusion_audit:None,publication:Default::default(),original_deadline:Instant::now(),failure:None,lift_records:vec![],accepted_record:None}}
#[test]fn actual_split_lends_live_records_and_original_budget_without_cloning_state(){let mut o=owner();let b=&o.original.budget as *const _ as usize;let n=o.normalized.as_ptr()as usize;let end=o.original_deadline;capture_encoding::with_continuation_parts(&mut o,|s,e,budget,records,publication,sink|{assert_eq!(budget as *mut _ as usize,b);assert_eq!(s.normalized().as_ptr()as usize,n);assert_eq!(s.signed_domain(),[(7,8)]);assert_eq!(s.signed_target(),Some((9,10)));assert_eq!(s.deadline(),end);assert_eq!(e.variables(),2);let a=continuation::Attempt{rows:5};records.push(LiftRecord);s.checkpoint_active(&a,records,budget,publication,sink,&capture::CaptureLimits).unwrap();records.push(LiftRecord);s.checkpoint_active(&a,records,budget,publication,sink,&capture::CaptureLimits).unwrap();});assert_eq!(o.publication.states,vec![(5,1),(5,2)]);assert_eq!(o.original.budget.spent,2);assert_eq!(o.sink.published,2);}
#[test]fn actual_split_cannot_refresh_stopped_budget(){let mut o=owner();o.original.budget.stopped=Some(Stop::Refused);capture_encoding::with_continuation_parts(&mut o,|s,_,b,r,p,k|{assert_eq!(s.checkpoint_active(&continuation::Attempt{rows:2},r,b,p,k,&capture::CaptureLimits),Err(Stop::Refused));});assert!(o.publication.states.is_empty());assert_eq!(o.sink.published,0);}
}}}}
'''.replace('REPLACE',str(p/'borrow_encoding.rs'));(p/'borrow_harness.rs').write_text(h)
