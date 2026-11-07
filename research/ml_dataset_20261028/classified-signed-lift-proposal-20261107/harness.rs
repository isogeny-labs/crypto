//! Structural test shim, NOT an authenticated CheckedMap constructor.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum Error{InvalidOriginal,Cap,Admission,Identity,Semantics,Formula}
pub struct Quote{pub work:u64,pub bytes:u64}
pub trait Admission{fn reserve(&mut self,q:Quote)->Result<(),Error>;fn check(&mut self)->Result<(),Error>;}
pub enum Constraint{Clause{literals:Vec<i32>},Xor{wires:Vec<u32>,rhs:bool}}
pub struct Identity{n:u32,m:usize,a:Vec<u32>,b:Vec<u32>,target:Option<Vec<u32>>,low_terms:Vec<u32>,codes:Vec<Vec<u32>>,pattern:u32}
pub struct Map{identity:Identity,variables:u32,formula:Vec<Constraint>,blocks:Vec<Vec<u32>>}
pub struct CheckedMap(Map);
#[path="checked_model.shim.rs"]pub mod checked_model;
mod adapter;
#[cfg(test)]mod tests{
 use super::*;use adapter::*;use checked_model::*;use std::time::{Instant,Duration};
 #[derive(Default)]struct Ledger{work:u64,bytes:u64,adds:u64,checks:u64,stop:Option<u64>,add_cap:Option<u64>}
 impl Admission for Ledger{fn reserve(&mut self,q:Quote)->Result<(),Error>{self.work+=q.work;self.bytes+=q.bytes;Ok(())}fn check(&mut self)->Result<(),Error>{self.checks+=1;if self.stop.is_some_and(|s|self.checks>=s){Err(Error::Admission)}else{Ok(())}}}
 impl VerificationAdmission for Ledger{fn addition(&mut self)->Result<(),Error>{if self.add_cap.is_some_and(|c|self.adds>=c){return Err(Error::Admission)}self.adds+=1;Ok(())}}
 fn map(a:u32,m:usize,xs:&[u64],target:Option<u64>,intermediate:&[u64],pattern:u32)->(CheckedMap,Vec<bool>){let n=3;let mut blocks=vec![];let mut model=vec![];for &x in xs.iter().chain(intermediate).chain(std::iter::once(&target.unwrap_or(0))){let mut block=vec![];for k in 0..n{block.push(model.len()as u32+1);model.push(x>>k&1!=0)}blocks.push(block)}let formula=model.iter().enumerate().map(|(i,v)|Constraint::Clause{literals:vec![if *v{i as i32+1}else{-(i as i32+1)}]}).collect();let mut codes=xs.to_vec();codes.sort();codes.dedup();(CheckedMap(Map{identity:Identity{n,m,a:vec![a],b:vec![1],target:target.map(|v|vec![v as u32]),low_terms:vec![0,1],codes:codes.into_iter().map(|v|vec![v as u32]).collect(),pattern},variables:model.len()as u32,formula,blocks}),model)}
 fn go(map:&CheckedMap,model:&[bool],domain:&[(u64,u64)],target:Option<(u64,u64)>,p:&mut Progress,o:&mut Ledger)->Outcome{verify_candidate(map,model,domain,target,p,Instant::now()+Duration::from_secs(5),o)}
 #[test]fn actual_allowed_signs_are_not_synthesized(){// GF(8), f=t^3+t+1, y²+xy=x³+1. P=(1,0), -P=(1,1), 2P=(0,1).
 let(map,model)=map(0,2,&[1,1],None,&[],0);let mut p=Progress::default();let r=go(&map,&model,&[(1,0)],None,&mut p,&mut Ledger::default());assert_eq!(r,Outcome::RejectedUnclassified(Error::Semantics));assert!(!p.signed_relation);let mut p=Progress::default();assert_eq!(go(&map,&model,&[(1,0),(1,1)],None,&mut p,&mut Ledger::default()),Outcome::VerifiedSignedRelation);assert_eq!(p.sum,None);assert_eq!(p.path.len(),2);assert!(p.path.contains(&(1,1)));}
 #[test]fn infinity_and_affine_zero_distinct_for_both_twists_all_arities(){// Real order-two point (0,1), curves a=0,1 are GF8 quadratic twists.
 for a in [0,1]{for m in 2..=15{let target=if m%2==0{None}else{Some((0,1))};let mut pattern=0;for step in 1..m-1{if (step+1)%2==0{pattern|=1<<(step-1)}}let(map,model)=map(a,m,&vec![0;m],target.map(|p|p.0),&vec![0;m-2],pattern);let mut p=Progress::default();assert_eq!(go(&map,&model,&[(0,1)],target,&mut p,&mut Ledger::default()),Outcome::VerifiedSignedRelation);assert_eq!(p.sum,target);assert_eq!(p.path.len(),m);let mut wrong=Progress::default();let other=if target.is_some(){None}else{Some((0,1))};assert_eq!(go(&map,&model,&[(0,1)],other,&mut wrong,&mut Ledger::default()),Outcome::RejectedUnclassified(Error::Identity));}}}
 #[test]fn owner_refusal_retains_diagnostics_without_verified_relation(){let(map,model)=map(0,2,&[1,1],None,&[],0);let domain=[(1,0),(1,1)];let mut full=Ledger::default();assert_eq!(go(&map,&model,&domain,None,&mut Progress::default(),&mut full),Outcome::VerifiedSignedRelation);let mut prefix=false;for stop in 1..=full.checks{let mut o=Ledger{stop:Some(stop),..Default::default()};let mut p=Progress::default();assert_eq!(go(&map,&model,&domain,None,&mut p,&mut o),Outcome::Stopped(Error::Admission));assert!(!p.signed_relation);prefix|=!p.path.is_empty();}assert!(prefix);let mut p=Progress::default();let mut o=Ledger{add_cap:Some(1),..Default::default()};assert_eq!(go(&map,&model,&domain,None,&mut p,&mut o),Outcome::Stopped(Error::Admission));assert_eq!(o.adds,1);assert!(!p.path.is_empty());}
 #[test]fn original_deadline_and_poisoned_state_are_not_renewed(){let(map,model)=map(0,2,&[1,1],None,&[],0);let mut o=Ledger::default();assert_eq!(verify_candidate(&map,&model,&[(1,0),(1,1)],None,&mut Progress::default(),Instant::now(),&mut o),Outcome::Deadline);assert_eq!(o.work,0);let mut p=Progress::default();p.path.push((1,0));assert_eq!(go(&map,&model,&[(1,0),(1,1)],None,&mut p,&mut o),Outcome::RejectedUnclassified(Error::Identity));assert_eq!(p.path,vec![(1,0)]);}
 #[test] fn classified_exhaustion_requires_final_original_owner_check(){
  let(map,model)=map(0,2,&[1,1],None,&[],0);
  let mut p=Progress::default();let mut owner=Ledger::default();
  assert_eq!(verify_classified(&map,&model,&[(1,0)],None,&mut p,&mut owner),Ok(LiftVerdict::ExhaustedForModel));
  assert!(!p.signed_relation);assert_eq!(p.sign_mask,3);
  for stop in 1..=owner.checks {
   let mut o=Ledger{stop:Some(stop),..Default::default()};
   assert_eq!(verify_classified(&map,&model,&[(1,0)],None,&mut Progress::default(),&mut o),Err(Error::Admission));
  }
 }
 #[test] fn malformed_model_is_not_classified_as_exhausted(){
  let(map,mut model)=map(0,2,&[1,1],None,&[],0);model[0]=!model[0];
  assert_eq!(verify_classified(&map,&model,&[(1,0)],None,&mut Progress::default(),&mut Ledger::default()),Err(Error::Formula));
 }
 #[test] fn signed_success_and_empty_domain_keep_distinct_outcomes(){
  let(map,model)=map(0,2,&[1,1],None,&[],0);
  assert_eq!(verify_classified(&map,&model,&[(1,0),(1,1)],None,&mut Progress::default(),&mut Ledger::default()),Ok(LiftVerdict::SignedRelation));
  assert_eq!(verify_classified(&map,&model,&[],None,&mut Progress::default(),&mut Ledger::default()),Err(Error::Semantics));
 }

}
