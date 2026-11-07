#![allow(dead_code)]
mod bqi;
mod sparse{#[derive(Clone,Copy,Default)]pub struct Charge{pub work:u64,pub bytes:u64}pub type Polynomial=Vec<Vec<u32>>;pub trait Owner{fn reserve(&mut self,q:Charge)->Result<(),()>;fn check(&mut self)->Result<(),()>;}}
mod scheduler{pub struct ProofTerm{pub generator:usize,pub multiplier:Vec<u32>}pub struct Row{pub polynomial:super::sparse::Polynomial,pub proof:Vec<ProofTerm>}}
#[path="dynamic_boolean_search_bqi.rs"]mod bridge;
#[cfg(test)]mod tests{
 use super::*;use std::time::{Instant,Duration};
 #[derive(Default)]struct Owner{work:u64,bytes:u64,max_work:u64,checks:usize,fail_check:Option<usize>,delay_bytes:Option<u64>}
 impl sparse::Owner for Owner{fn reserve(&mut self,q:sparse::Charge)->Result<(),()>{if self.work+q.work>self.max_work{return Err(())}self.work+=q.work;self.bytes+=q.bytes;if self.delay_bytes==Some(q.bytes){std::thread::sleep(Duration::from_millis(40));}Ok(())}fn check(&mut self)->Result<(),()>{self.checks+=1;if self.fail_check==Some(self.checks){Err(())}else{Ok(())}}}
 fn limits()->bridge::Limits{bridge::Limits{variables:4096,generators:4096,terms:4096,degree:64,proof:4096,wire:65536,work:4_000_000,bytes:8_000_000}}
 fn checker()->bqi::Limits{bqi::Limits{variables:4096,generators:4096,polynomial_terms:4096,proof_terms:4096,accumulator_entries:4096,caps:[65536,262144,262144,262144,262144,65536,1024,1048576,262144]}}
 fn row()->scheduler::Row{scheduler::Row{polynomial:vec![vec![0]],proof:vec![scheduler::ProofTerm{generator:0,multiplier:vec![]}]}}
 #[test]fn many_small_certificates_share_one_original_budget_below_two_full_envelopes(){
  let mut owner=Owner{max_work:2_000_000,..Default::default()};let end=Instant::now()+Duration::from_secs(10);let mut total=0;for _ in 0..8{let c=bridge::certify(1,&[vec![vec![0]]],&row(),limits(),checker(),end,&mut owner);assert!(c.result.is_ok(),"{:?}",c.result);assert!(c.audit.unwrap().result.is_ok());total+=c.charge.work;}assert_eq!(owner.work,total);assert!(total<2_000_000);assert!(2*checker().caps.iter().sum::<u64>()>2_000_000);println!("8 certificates actual modeled work={total}, reserved bytes={}",owner.bytes);
 }
 #[test]fn parent_refusal_retains_audit_request_and_bridge_reason(){
  let mut owner=Owner{max_work:20,..Default::default()};let c=bridge::certify(1,&[vec![vec![0]]],&row(),limits(),checker(),Instant::now()+Duration::from_secs(10),&mut owner);assert!(c.result.is_err());assert!(owner.work<=20);
  // Refuse inside checker specifically after discovering exact serialization cost.
  let mut baseline=Owner{max_work:2_000_000,..Default::default()};let full=bridge::certify(1,&[vec![vec![0]]],&row(),limits(),checker(),Instant::now()+Duration::from_secs(10),&mut baseline);assert_eq!(full.result,Ok(()));let audit=full.audit.unwrap();assert!(audit.result.is_ok());let parser=full.charge.work-audit.charges.used.iter().sum::<u64>()-1;
  let mut owner=Owner{max_work:parser+1,..Default::default()};let failed=bridge::certify(1,&[vec![vec![0]]],&row(),limits(),checker(),Instant::now()+Duration::from_secs(10),&mut owner);assert_eq!(failed.result,Err(bridge::Failure::Owner));assert!(matches!(failed.audit.unwrap().result,Err(bqi::Failure::Unknown(bqi::Exhaustion::Admission{..}))));
 }
 #[test]fn original_clock_is_not_renewed(){let mut owner=Owner{max_work:2_000_000,..Default::default()};let c=bridge::certify(1,&[vec![vec![0]]],&row(),limits(),checker(),Instant::now(),&mut owner);assert_eq!(c.result,Err(bridge::Failure::Deadline));assert_eq!(owner.work,0);assert!(c.audit.is_none());}
 #[test]fn bridge_callback_expiring_clock_retains_reserved_request_without_checker_allocation(){
  let mut owner=Owner{max_work:2_000_000,delay_bytes:Some(4096*24),..Default::default()};
  let c=bridge::certify(1,&[vec![vec![0]]],&row(),limits(),checker(),Instant::now()+Duration::from_millis(30),&mut owner);
  assert_eq!(c.result,Err(bridge::Failure::Deadline));let a=c.audit.unwrap();
  assert_eq!(a.result,Err(bqi::Failure::Unknown(bqi::Exhaustion::Admission{resource:bqi::Resource::AllocationBytes,amount:98304})));
  assert!(a.charges.used[bqi::Resource::AllocationBytes as usize]<98304);
  assert_eq!(c.charge.work,owner.work);assert_eq!(c.charge.bytes,owner.bytes);assert!(owner.bytes>=98304);
 }
 #[test]fn bridge_postcheck_refusal_preserves_ancestor_accepted_reservation(){
  let mut baseline=Owner{max_work:2_000_000,..Default::default()};let full=bridge::certify(1,&[vec![vec![0]]],&row(),limits(),checker(),Instant::now()+Duration::from_secs(10),&mut baseline);assert_eq!(full.result,Ok(()));
  // The final BQI admitted request is immediately before final bridge pay.
  let mut owner=Owner{max_work:2_000_000,fail_check:Some(baseline.checks-1),..Default::default()};
  let c=bridge::certify(1,&[vec![vec![0]]],&row(),limits(),checker(),Instant::now()+Duration::from_secs(10),&mut owner);assert_eq!(c.result,Err(bridge::Failure::Owner));
  assert!(matches!(c.audit.unwrap().result,Err(bqi::Failure::Unknown(bqi::Exhaustion::Admission{..}))));
  assert_eq!(c.charge.work,owner.work);assert_eq!(c.charge.bytes,owner.bytes);
 }

}
