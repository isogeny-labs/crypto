mod sparse;mod scheduler;mod bridge;mod bqi;
#[cfg(test)]mod integration{
 use super::*;use scheduler::search::*;use sparse::{Owner,Charge};use std::time::{Instant,Duration};
 #[derive(Default)]struct Ledger{work:u64,bytes:u64}
 impl Owner for Ledger{fn reserve(&mut self,q:Charge)->Result<(),()>{self.work=self.work.checked_add(q.work).ok_or(())?;self.bytes=self.bytes.checked_add(q.bytes).ok_or(())?;Ok(())}fn check(&mut self)->Result<(),()>{Ok(())}}
 fn search_limits()->SearchLimits{SearchLimits{nodes:32,native:scheduler::Limits{batch_rows:128,batch_ops:1024,batches:32,variables:256,generators:64,basis:64,tasks:1024,terms:256,proof:1024,degree:256,work:20_000_000,bytes:200_000_000}}}
 fn cert_limits()->bridge::Limits{bridge::Limits{variables:256,generators:64,terms:256,degree:256,proof:1024,wire:65536,work:1_000_000_000,bytes:200_000_000}}
 #[test]fn conditional_refutation_binds_branch_equation_and_same_owner(){
  let g=vec![vec![vec![],vec![0]],vec![vec![],vec![1],vec![2]]];let d=Instant::now()+Duration::from_secs(10);let mut owner=Ledger::default();let a=solve(3,&g,search_limits(),d,&mut owner);assert_eq!(a.verdict,Verdict::Witness);
  let node=&a.nodes[a.nodes[0].children[0].unwrap()];assert_eq!(node.status,NodeStatus::Refuted);let row=&node.native.as_ref().unwrap().basis[node.refutation_row.unwrap()];let branch=row.proof.iter().find_map(|p|match node.origins[p.generator]{Origin::Branch{variable:0,value:false,..}=>Some(p.generator),_=>None}).unwrap();let before=owner.work;
  let c=bridge::certify(3,&node.equations,row,cert_limits(),bqi::Limits::software(),d,&mut owner);assert_eq!(c.result,Ok(()));assert_eq!(owner.work,before+c.charge.work);assert!(!c.audit.unwrap().source_authenticated);
  let mut altered=node.equations.clone();altered[branch]=vec![vec![],vec![0]];
  let c=bridge::certify(3,&altered,row,cert_limits(),bqi::Limits::software(),d,&mut owner);assert_eq!(c.result,Err(bridge::Failure::Checker));assert_eq!(c.audit.unwrap().result,Err(bqi::Failure::EqualityMismatch));
 }
 #[test]fn original_unsat_row_is_independently_checked(){let g=vec![vec![vec![0]],vec![vec![],vec![0]]];let d=Instant::now()+Duration::from_secs(10);let mut owner=Ledger::default();let a=solve(1,&g,search_limits(),d,&mut owner);assert_eq!(a.verdict,Verdict::UnsatDerived);let node=&a.nodes[0];let row=&node.native.as_ref().unwrap().basis[node.refutation_row.unwrap()];assert_eq!(bridge::certify(1,&node.equations,row,cert_limits(),bqi::Limits::software(),d,&mut owner).result,Ok(()));}
}
