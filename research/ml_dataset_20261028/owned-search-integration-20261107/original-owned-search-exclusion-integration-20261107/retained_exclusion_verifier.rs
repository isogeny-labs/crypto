//! Exact exclusion pairing and fresh signed arithmetic replay. Records are
//! borrowed projections of the authentic owner's retained callback records.
use super::{continuation,sparse::{Owner,Charge,Polynomial},checked_model::{Progress,VerificationAdmission},signed_lift,Admission,CheckedMap,Error,Quote};
use std::{mem::size_of,time::Instant};
pub struct RecordView<'a>{pub partition:usize,pub record:u64,pub model:&'a [bool],pub callback_result:Option<Result<(),continuation::Failure>>,pub returned_outcome:Option<signed_lift::Outcome>}
/// Production implementations must project private owning LiftRecord fields:
/// returned_outcome is Some only for returned == Some(Ok(outcome)). This trait
/// authenticates no history; supplied projections remain input data to replay.
pub trait Records{fn len(&self)->usize;fn get(&self,index:usize)->Option<RecordView<'_>>;}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Failure{Cap,Owner,Deadline,Overflow,Allocation,Coverage,Pairing,Callback,Replay}
#[derive(Clone,Copy)]pub struct Limits{pub records:usize,pub work:u64,pub bytes:u64,pub coverage:continuation::Limits}
pub struct Replay{pub record:usize,pub partition:usize,pub progress:Progress,pub outcome:Option<signed_lift::Outcome>}
pub struct Audit{pub result:Result<(),Failure>,pub charge:Charge,pub coverage:Option<continuation::CoverageAudit>,pub replays:Vec<Replay>,pub paired:Vec<bool>}
struct Bound<'a,T:Owner+VerificationAdmission>{owner:&'a mut T,limits:Limits,deadline:Instant,charge:Charge,failure:Option<Failure>}
impl<T:Owner+VerificationAdmission> Bound<'_,T>{
 fn time(&mut self)->Result<(),Failure>{if let Some(e)=self.failure{return Err(e)}if Instant::now()>=self.deadline{return Err(Failure::Deadline)}Ok(())}
 fn pay(&mut self,w:u64,b:u64)->Result<(),Failure>{let r=(||{self.time()?;let q=Charge{work:self.charge.work.checked_add(w).ok_or(Failure::Overflow)?,bytes:self.charge.bytes.checked_add(b).ok_or(Failure::Overflow)?};if q.work>self.limits.work||q.bytes>self.limits.bytes{return Err(Failure::Cap)}Owner::reserve(self.owner,Charge{work:w,bytes:b}).map_err(|_|Failure::Owner)?;self.charge=q;Owner::check(self.owner).map_err(|_|Failure::Owner)?;self.time()})();if let Err(e)=r{self.failure=Some(e)}r}
 fn scan(&mut self,n:usize)->Result<(),Failure>{self.pay(u64::try_from(n).map_err(|_|Failure::Overflow)?.checked_add(1).ok_or(Failure::Overflow)?,0)}
 fn alloc<V>(&mut self,v:&mut Vec<V>,n:usize)->Result<(),Failure>{let b=n.checked_mul(size_of::<V>()).and_then(|n|n.checked_add(64)).ok_or(Failure::Overflow)?;self.pay(1,u64::try_from(b).map_err(|_|Failure::Overflow)?)?;v.try_reserve_exact(n).map_err(|_|Failure::Allocation)}
}
impl<T:Owner+VerificationAdmission> Owner for Bound<'_,T>{fn reserve(&mut self,q:Charge)->Result<(),()>{self.pay(q.work,q.bytes).map_err(|_|())}fn check(&mut self)->Result<(),()>{self.pay(0,0).map_err(|_|())}}
impl<T:Owner+VerificationAdmission> Admission for Bound<'_,T>{fn reserve(&mut self,q:Quote)->Result<(),Error>{self.pay(q.work,q.bytes).map_err(|_|Error::Admission)}fn check(&mut self)->Result<(),Error>{self.pay(0,0).map_err(|_|Error::Admission)}}
impl<T:Owner+VerificationAdmission> VerificationAdmission for Bound<'_,T>{fn addition(&mut self)->Result<(),Error>{self.pay(0,0).map_err(|_|Error::Admission)?;if self.owner.addition().is_err(){self.failure=Some(Failure::Owner);return Err(Error::Admission)}self.pay(0,0).map_err(|_|Error::Admission)}}
/// Caller lends its authentic checked source, original signed domain/target,
/// complete retained continuation and original owner/deadline. No stored verdict
/// alone authorizes exclusion: every paired model is reverified from scratch.
pub fn verify(n:u32,g:&[Polynomial],map:&CheckedMap,domain:&[(u64,u64)],target:Option<(u64,u64)>,attempt:&continuation::Attempt,records:&impl Records,limits:Limits,deadline:Instant,owner:&mut(impl Owner+VerificationAdmission))->Audit{
 let mut out=Audit{result:Ok(()),charge:Charge::default(),coverage:None,replays:Vec::new(),paired:Vec::new()};let mut b=Bound{owner,limits,deadline,charge:Charge::default(),failure:None};
 out.result=(||{
  b.pay(1,0)?;if n!=map.variables(){return Err(Failure::Pairing)}let count=records.len();if count>limits.records{return Err(Failure::Cap)}
  let coverage=continuation::audit(n,g,attempt,limits.coverage,deadline,&mut b);let status=coverage.result;let exclusions=coverage.exclusions;out.coverage=Some(coverage);if let Some(e)=b.failure{return Err(e)}if status.is_err(){return Err(Failure::Coverage)}if count!=exclusions{return Err(Failure::Pairing)}
  b.alloc(&mut out.replays,count)?;b.alloc(&mut out.paired,count)?;b.scan(count)?;out.paired.resize(count,false);
  for(partition,p)in attempt.partitions.iter().enumerate(){
   b.pay(1,0)?;if p.status!=continuation::Status::Excluded{continue}
   let index=usize::try_from(p.exclusion_record.ok_or(Failure::Pairing)?).map_err(|_|Failure::Overflow)?;
   if index>=count||out.paired[index]{return Err(Failure::Pairing)}
   let r=records.get(index).ok_or(Failure::Pairing)?;
   if r.partition!=partition||r.record!=index as u64{return Err(Failure::Pairing)}
   if r.callback_result!=Some(Ok(()))||r.returned_outcome!=Some(signed_lift::Outcome::ExhaustedForModel){return Err(Failure::Callback)}
   let native=p.search.as_ref().ok_or(Failure::Pairing)?;let model=&native.nodes.get(native.witness_node.ok_or(Failure::Pairing)?).ok_or(Failure::Pairing)?.candidate;
   b.scan(model.len())?;b.scan(r.model.len())?;if r.model!=model||r.model.len()!=n as usize{return Err(Failure::Pairing)}
   out.replays.push(Replay{record:index,partition,progress:Progress::default(),outcome:None});let replay=out.replays.last_mut().unwrap();
   let outcome=signed_lift::verify_candidate(map,r.model,domain,target,&mut replay.progress,deadline,&mut b);replay.outcome=Some(outcome);
   if let Some(e)=b.failure{return Err(e)}if outcome!=signed_lift::Outcome::ExhaustedForModel{return Err(Failure::Replay)}
   b.pay(1,0)?;out.paired[index]=true;
  }
  b.scan(count)?;if out.paired.iter().any(|v|!*v){return Err(Failure::Pairing)}b.pay(1,0)
 })();out.charge=b.charge;out
}
