//! Actual bridge implementation; explicitly structural owner/capture/encoder fixtures.
#![allow(dead_code)]
mod ml_dataset { pub mod solvers { mod koblitz {
use std::time::Instant;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum Stop{Phase,Overflow,Allocation,Observer,Serialization,Deadline,Work,Bytes,Trace,NativeFailure}
pub struct Budget{native_work_limit:u64,native:Native,checking_work:u64,stopped:Option<Stop>,checks:usize,fail_check:usize,trace:u64}
pub struct Native{work:u64}
impl Budget{fn reserve_checking(&mut self,w:u64,_b:u64,n:Instant)->Result<(),Stop>{self.check(n)?;self.checking_work+=w;Ok(())}fn check(&mut self,_:Instant)->Result<(),Stop>{self.checks+=1;if self.checks==self.fail_check{self.stopped=Some(Stop::Deadline)}self.stopped.map_or(Ok(()),Err)}fn fail<T>(&mut self,s:Stop)->Result<T,Stop>{Err(*self.stopped.get_or_insert(s))}fn reserve_trace(&mut self,n:u64,t:Instant)->Result<(),Stop>{self.check(t)?;self.trace+=n;Ok(())}}
mod capture{
#[derive(Clone)]pub struct CaptureLimits{pub chunk_bytes:u64,pub max_payload_bytes:u64,pub max_payload_count:u64}
#[derive(Clone,Debug,PartialEq,Eq)]pub struct CommittedEventPosition{pub event_count:u64,pub binding_sha256:String,pub byte_end:u64}
#[derive(Debug)]pub struct CaptureError;
pub struct CaptureEmitter{pub head:CommittedEventPosition,pub calls:usize,pub fail_call:usize,pub corrupt_call:usize}
impl CaptureEmitter{pub fn committed_event_position(&self)->CommittedEventPosition{self.head.clone()}pub fn payload_borrowed(&mut self,_:&str,_:&str,_:&str,b:&[u8])->Result<(),CaptureError>{self.calls+=1;if self.calls==self.fail_call{return Err(CaptureError)}self.head.event_count+=1;self.head.byte_end+=b.len()as u64;if self.calls==self.corrupt_call{self.head.event_count+=1}Ok(())}}
}
pub struct Quote{work:u64,bytes:u64,trace:u64}
fn quote_parent_storage_length(n:u64,_:bool,_:&capture::CaptureLimits)->Result<Quote,Stop>{Ok(Quote{work:n,bytes:n,trace:n})}
mod original{use super::*;mod boolean{use super::*;
#[derive(Clone,Copy,Debug)]enum Failure{Publication(Stop)}
struct Original{budget:Budget,capture_limits:capture::CaptureLimits,map_ack:capture::CommittedEventPosition}
struct OwnedAttempt{original:Original,sink:capture::CaptureEmitter,publication:publication::Publication,failure:Option<Failure>}
impl OwnedAttempt{fn fail(&mut self,f:Failure,s:Stop){self.failure=Some(f);let _:Result<(),Stop>=self.original.budget.fail(s);}}
mod capture_encoding{
use super::*;
pub struct Limits{pub chunk_bytes:usize,pub total_bytes:u64,pub chunks:u64,pub work:u64}
#[derive(Clone,Copy,Debug)]pub enum Failure{Cap,Owner,Deadline,Overflow,Allocation,Publish,Format}
pub struct Attempt{pub pending:Vec<u8>,pub acknowledged_chunks:u64,pub acknowledged_bytes:u64,pub encoded_bytes:u64,pub work:u64,pub allocation_bytes:u64,pub result:Result<(),Failure>,pub complete:bool}
pub trait AdmissionSink{fn reserve(&mut self,w:u64,b:u64)->Result<(),()>;fn check(&mut self)->Result<(),()>;fn publish(&mut self,i:u64,b:&[u8])->Result<(),()>;}
pub struct EncodingSource<'a>{ack:&'a capture::CommittedEventPosition,corrupt:bool,pending:bool}
impl EncodingSource<'_>{pub fn map_ack(&self)->Option<&capture::CommittedEventPosition>{Some(self.ack)}pub fn pattern(&self)->Option<u32>{Some(0)}}
pub fn with_parts<R>(o:&mut OwnedAttempt,f:impl FnOnce(&EncodingSource<'_>,&mut Budget,&mut capture::CaptureEmitter)->R)->R{f(&EncodingSource{ack:&o.original.map_ack,corrupt:o.original.map_ack.binding_sha256=="bad-accounting",pending:o.original.map_ack.binding_sha256=="bad-pending"},&mut o.original.budget,&mut o.sink)}
pub fn encode(source:&EncodingSource,_:u64,_:Limits,io:&mut impl AdmissionSink)->Attempt{let mut a=Attempt{pending:vec![1,2],acknowledged_chunks:0,acknowledged_bytes:0,encoded_bytes:4,work:0,allocation_bytes:2,result:Ok(()),complete:false};for i in 0..2{if io.publish(i,&a.pending).is_err(){a.result=Err(Failure::Owner);return a}a.acknowledged_chunks+=1;a.acknowledged_bytes+=2}a.pending.clear();a.complete=true;if source.corrupt{a.acknowledged_bytes+=1}if source.pending{a.pending.push(9)}a}
}
#[path="/home/ec2-user/ml-dataset-resume-20261029/owned-search-midcontinuation-20261107/owned_search_capture_publication.rs"]mod publication;
#[cfg(test)]mod tests{use super::*;
fn owner()->OwnedAttempt{let head=capture::CommittedEventPosition{event_count:1,binding_sha256:"original".into(),byte_end:10};OwnedAttempt{original:Original{budget:Budget{native_work_limit:1000000,native:Native{work:0},checking_work:0,stopped:None,checks:0,fail_check:usize::MAX,trace:0},capture_limits:capture::CaptureLimits{chunk_bytes:1024,max_payload_bytes:10000,max_payload_count:10},map_ack:head.clone()},sink:capture::CaptureEmitter{head,calls:0,fail_call:usize::MAX,corrupt_call:usize::MAX},publication:Default::default(),failure:None}}
#[test]fn exact_checkpoints_chain_original_head_and_keep_unique_indices(){let mut o=owner();assert!(o.checkpoint_capture().is_ok());assert!(o.checkpoint_capture().is_ok());assert_eq!(o.publication.snapshots.len(),2);for(i,s)in o.publication.snapshots.iter().enumerate(){assert!(s.complete);assert_eq!(s.index,i as u64);assert_eq!(s.chunks.len(),2);assert!(s.chunks.iter().all(|c|c.ack.is_some()&&c.exact));}assert_eq!(o.sink.calls,4);}
#[test]fn second_chunk_failure_retains_first_ack_and_pending_bytes(){let mut o=owner();o.sink.fail_call=2;assert!(o.checkpoint_capture().is_err());let s=&o.publication.snapshots[0];assert!(!s.complete);assert!(s.chunks[0].ack.is_some());assert!(s.chunks[1].ack.is_none());assert_eq!(s.encoding.as_ref().unwrap().pending,vec![1,2]);assert!(s.capture_error.is_some());let c=o.sink.calls;assert!(o.checkpoint_capture().is_err());assert_eq!(o.sink.calls,c);}
#[test]fn malformed_ack_survives_and_cannot_complete(){let mut o=owner();o.sink.corrupt_call=1;assert!(o.checkpoint_capture().is_err());let c=&o.publication.snapshots[0].chunks[0];assert!(c.ack.is_some());assert!(!c.exact);assert!(!o.publication.snapshots[0].complete);}
#[test]fn deadline_after_ack_retains_receipt_but_no_success(){let mut found=false;for n in 1..20{let mut o=owner();o.original.budget.fail_check=n;let r=o.checkpoint_capture();if r==Err(Stop::Deadline)&&o.publication.snapshots.first().is_some_and(|s|s.chunks.iter().any(|c|c.ack.is_some())){assert!(!o.publication.snapshots[0].complete);found=true;}}assert!(found);}
#[test]fn inconsistent_encoder_byte_summary_cannot_complete(){let mut o=owner();o.original.map_ack.binding_sha256="bad-accounting".into();o.sink.head=o.original.map_ack.clone();assert_eq!(o.checkpoint_capture(),Err(Stop::Observer));assert!(!o.publication.snapshots[0].complete);assert_eq!(o.publication.snapshots[0].chunks.len(),2);assert!(o.publication.snapshots[0].chunks.iter().all(|c|c.ack.is_some()));}
#[test]fn claimed_complete_with_pending_bytes_retains_receipts_but_refuses(){let mut o=owner();o.original.map_ack.binding_sha256="bad-pending".into();o.sink.head=o.original.map_ack.clone();assert_eq!(o.checkpoint_capture(),Err(Stop::Observer));let s=&o.publication.snapshots[0];assert!(!s.complete);assert_eq!(s.encoding.as_ref().unwrap().pending,vec![9]);assert!(s.chunks.iter().all(|c|c.ack.is_some()));}
#[test]fn stale_head_and_stopped_budget_never_publish(){let mut o=owner();o.sink.head.event_count+=1;assert!(o.checkpoint_capture().is_err());assert_eq!(o.sink.calls,0);let mut o=owner();o.original.budget.stopped=Some(Stop::Work);assert!(o.checkpoint_capture().is_err());assert_eq!(o.sink.calls,0);assert!(o.publication.snapshots.is_empty());}
}
}}}}

}
