//! Prefix-partition candidate continuation. Exclusion records are caller data,
//! never a claim that signed lift has been checked by this generic core.
use super::{sparse::{Owner,Charge,Polynomial},scheduler::{self,search},check};
use std::{mem::size_of,time::Instant};
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Failure{Cap,Owner,Deadline,Overflow,Allocation,State,Native,Audit,Structure}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Outcome{Unknown,CandidateRetained,RemainingBooleanModelsExhaustedUnderRecordedExclusions}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Status{Building,Ready,Running,Candidate,Splitting,Excluded,Refuted}
#[derive(Clone,Copy)]pub struct Limits{pub partitions:usize,pub work:u64,pub bytes:u64,pub search:search::SearchLimits,pub audit:check::Limits}
pub struct Partition{pub parent:Option<usize>,pub prefix:Vec<bool>,pub equations:Vec<Polynomial>,pub status:Status,pub search:Option<search::SearchAttempt>,pub audit:Option<check::Audit>,pub children:Vec<usize>,pub exclusion_record:Option<u64>}
pub struct Attempt{pub partitions:Vec<Partition>,pub queue:Vec<usize>,pub active:Option<usize>,pub outcome:Outcome,pub failure:Option<Failure>,pub charge:Charge}
struct Meter<'a,T:Owner>{owner:&'a mut T,limits:Limits,deadline:Instant,charge:Charge}
impl<T:Owner> Meter<'_,T>{
 fn pay(&mut self,w:u64,b:u64)->Result<(),Failure>{
  if Instant::now()>=self.deadline{return Err(Failure::Deadline)}
  let q=Charge{work:self.charge.work.checked_add(w).ok_or(Failure::Overflow)?,bytes:self.charge.bytes.checked_add(b).ok_or(Failure::Overflow)?};
  if q.work>self.limits.work||q.bytes>self.limits.bytes{return Err(Failure::Cap)}
  self.owner.reserve(Charge{work:w,bytes:b}).map_err(|_|Failure::Owner)?;self.charge=q;
  self.owner.check().map_err(|_|Failure::Owner)?;if Instant::now()>=self.deadline{return Err(Failure::Deadline)}Ok(())
 }
 fn scan(&mut self,n:usize)->Result<(),Failure>{self.pay(u64::try_from(n).map_err(|_|Failure::Overflow)?.checked_add(1).ok_or(Failure::Overflow)?,0)}
 fn reserve<V>(&mut self,v:&mut Vec<V>,extra:usize)->Result<(),Failure>{let bytes=extra.checked_mul(size_of::<V>()).and_then(|b|b.checked_add(64)).ok_or(Failure::Overflow)?;self.pay(1,u64::try_from(bytes).map_err(|_|Failure::Overflow)?)?;v.try_reserve_exact(extra).map_err(|_|Failure::Allocation)}
 fn poly(&mut self,p:&Polynomial)->Result<Polynomial,Failure>{self.pay(1,0)?;let mut q=Vec::new();self.reserve(&mut q,p.len())?;for t in p{let mut x=Vec::new();self.reserve(&mut x,t.len())?;self.scan(t.len())?;x.extend_from_slice(t);q.push(x)}Ok(q)}
}
struct Bridge<'a,'b,T:Owner>{meter:&'a mut Meter<'b,T>,failure:Option<Failure>}
impl<T:Owner> Owner for Bridge<'_,'_,T>{fn reserve(&mut self,q:Charge)->Result<(),()>{self.meter.pay(q.work,q.bytes).map_err(|e|{self.failure=Some(e);})}fn check(&mut self)->Result<(),()>{self.meter.pay(0,0).map_err(|e|{self.failure=Some(e);})}}
/// Holds the SAME exclusively borrowed owner, source and absolute deadline for
/// all next/exclude calls. Local caps only restrict that owner's remaining grant.
/// Callback output remains diagnostic even when its final admission fails.
pub struct CandidateCall<V>{pub value:Option<V>,pub result:Result<(),Failure>}
pub struct Session<'a,T:Owner>{owner:&'a mut T,original:&'a [Polynomial],n:u32,limits:Limits,deadline:Instant,pub attempt:Attempt}
impl<'a,T:Owner> Session<'a,T>{
 pub fn new(n:u32,g:&'a [Polynomial],limits:Limits,deadline:Instant,owner:&'a mut T)->Self{Self{owner,original:g,n,limits,deadline,attempt:Attempt{partitions:Vec::new(),queue:Vec::new(),active:None,outcome:Outcome::Unknown,failure:None,charge:Charge::default()}}}
 pub fn candidate(&self)->Option<&[bool]>{if self.attempt.failure.is_some()||self.attempt.outcome!=Outcome::CandidateRetained{return None}candidate(&self.attempt.partitions[self.attempt.active?])}
 pub fn next(&mut self)->Result<Outcome,Failure>{
  if let Some(e)=self.attempt.failure{return Err(e)}
  let mut m=Meter{owner:self.owner,limits:self.limits,deadline:self.deadline,charge:self.attempt.charge};
  let result=next(self.n,self.original,&mut self.attempt,&mut m);self.attempt.charge=m.charge;
  if let Err(e)=result{self.attempt.failure=Some(e);self.attempt.outcome=Outcome::Unknown}result
 }
 /// Caller-requested exclusion, explicitly not a cryptographic certificate.
 /// Only the authentic owning integration may attach classified lift exhaustion.
 pub fn exclude(&mut self,record:u64)->Result<(),Failure>{
  if let Some(e)=self.attempt.failure{return Err(e)}
  let mut m=Meter{owner:self.owner,limits:self.limits,deadline:self.deadline,charge:self.attempt.charge};
  let result=exclude(self.n,&mut self.attempt,record,&mut m);self.attempt.charge=m.charge;
  if let Err(e)=result{self.attempt.failure=Some(e);self.attempt.outcome=Outcome::Unknown}result
 }
 /// Lend the SAME owner and immutable pending candidate to the authentic
 /// consumer. The callback admits its own progress/work and returns it by value.
 /// Its costs belong to the parent ledger, not this wrapper's charge counter.
 /// A failed final check preserves the returned value but poisons continuation.
 pub fn with_candidate_owner<V>(&mut self,f:impl FnOnce(&[bool],Instant,&mut T)->V)->CandidateCall<V>{
  if let Some(e)=self.attempt.failure{return CandidateCall{value:None,result:Err(e)}}
  let mut m=Meter{owner:self.owner,limits:self.limits,deadline:self.deadline,charge:self.attempt.charge};
  let before=(||{m.pay(1,0)?;if self.attempt.outcome!=Outcome::CandidateRetained{return Err(Failure::State)}let id=self.attempt.active.ok_or(Failure::State)?;candidate(self.attempt.partitions.get(id).ok_or(Failure::State)?).ok_or(Failure::State)})();
  let out=match before{
   Err(e)=>CandidateCall{value:None,result:Err(e)},
   Ok(model)=>{let value=f(model,m.deadline,&mut *m.owner);let result=m.pay(0,0);CandidateCall{value:Some(value),result}},
  };
  self.attempt.charge=m.charge;
  if let Err(e)=out.result{self.attempt.failure=Some(e);self.attempt.outcome=Outcome::Unknown}
  out
 }
 pub fn into_attempt(self)->Attempt{self.attempt}
}
fn candidate(p:&Partition)->Option<&[bool]>{let a=p.search.as_ref()?;let n=a.nodes.get(a.witness_node?)?;if a.result.is_err()||a.verdict!=search::Verdict::Witness||n.candidate_valid!=Some(true){return None}Some(&n.candidate)}
fn blank(parent:Option<usize>)->Partition{Partition{parent,prefix:Vec::new(),equations:Vec::new(),status:Status::Building,search:None,audit:None,children:Vec::new(),exclusion_record:None}}
fn build<T:Owner>(g:&[Polynomial],p:&mut Partition,m:&mut Meter<T>)->Result<(),Failure>{
 let count=g.len().checked_add(p.prefix.len()).ok_or(Failure::Overflow)?;m.reserve(&mut p.equations,count)?;
 for q in g{p.equations.push(m.poly(q)?)}
 for(i,value)in p.prefix.iter().enumerate(){m.pay(1,0)?;let v=u32::try_from(i).map_err(|_|Failure::Overflow)?;
  let mut q=Vec::new();m.reserve(&mut q,if *value{2}else{1})?;if *value{q.push(Vec::new())}let mut t=Vec::new();m.reserve(&mut t,1)?;t.push(v);q.push(t);p.equations.push(q);
 }p.status=Status::Ready;Ok(())
}
fn next<T:Owner>(n:u32,g:&[Polynomial],a:&mut Attempt,m:&mut Meter<T>)->Result<Outcome,Failure>{
 m.pay(1,0)?;
 if a.outcome==Outcome::CandidateRetained{return Ok(a.outcome)}
 if a.outcome==Outcome::RemainingBooleanModelsExhaustedUnderRecordedExclusions{return Ok(a.outcome)}
 if a.partitions.is_empty(){
  if m.limits.partitions==0||n>m.limits.search.native.variables{return Err(Failure::Cap)}
  m.reserve(&mut a.partitions,m.limits.partitions)?;m.reserve(&mut a.queue,m.limits.partitions)?;
  a.partitions.push(blank(None));a.active=Some(0);build(g,&mut a.partitions[0],m)?;a.queue.push(0);
 }
 loop{
  m.pay(1,0)?;let Some(&id)=a.queue.last()else{a.active=None;a.outcome=Outcome::RemainingBooleanModelsExhaustedUnderRecordedExclusions;return Ok(a.outcome)};
  a.active=Some(id);let p=&mut a.partitions[id];
  if p.status==Status::Building{build(g,p,m)?}if p.status!=Status::Ready{return Err(Failure::State)}p.status=Status::Running;
  let mut bridge=Bridge{meter:m,failure:None};let s=search::solve(n,&p.equations,bridge.meter.limits.search,bridge.meter.deadline,&mut bridge);let failed=bridge.failure;let result=s.result;p.search=Some(s);
  if let Some(e)=failed{return Err(e)}if result.is_err(){return Err(Failure::Native)}
  let mut bridge=Bridge{meter:m,failure:None};let v=check::verify(n,&p.equations,p.search.as_ref().unwrap(),bridge.meter.limits.audit,bridge.meter.deadline,&mut bridge);let failed=bridge.failure;let result=v.result;p.audit=Some(v);
  if let Some(e)=failed{return Err(e)}if result.is_err(){return Err(Failure::Audit)}
  m.pay(1,0)?;
  match p.search.as_ref().unwrap().verdict{
   search::Verdict::Witness=>{p.status=Status::Candidate;a.outcome=Outcome::CandidateRetained;return Ok(a.outcome)},
   search::Verdict::UnsatDerived=>{p.status=Status::Refuted;a.queue.pop();},_=>return Err(Failure::Native),
  }
 }
}
fn exclude<T:Owner>(n:u32,a:&mut Attempt,record:u64,m:&mut Meter<T>)->Result<(),Failure>{
 m.pay(1,0)?;if a.outcome!=Outcome::CandidateRetained{return Err(Failure::State)}let id=a.active.ok_or(Failure::State)?;
 let fixed=a.partitions[id].prefix.len();let model=candidate(&a.partitions[id]).ok_or(Failure::State)?;
 if model.len()!=n as usize||fixed>model.len(){return Err(Failure::State)}m.scan(fixed)?;if model[..fixed]!=a.partitions[id].prefix{return Err(Failure::Structure)}
 let children=model.len()-fixed;let total=a.partitions.len().checked_add(children).ok_or(Failure::Overflow)?;
 if total>m.limits.partitions{return Err(Failure::Cap)}
 // The original retained witness is the exact excluded assignment. Every child
 // differs at one first free position, so children are disjoint and cover its
 // old partition minus precisely this point. No expanded point polynomial.
 m.reserve(&mut a.partitions[id].children,children)?;a.partitions[id].status=Status::Splitting;a.partitions[id].exclusion_record=Some(record);a.outcome=Outcome::Unknown;
 for j in fixed..n as usize{
  m.pay(1,0)?;let child=a.partitions.len();a.partitions.push(blank(Some(id)));a.partitions[id].children.push(child);
  m.reserve(&mut a.partitions[child].prefix,j+1)?;
  for k in 0..=j{m.pay(1,0)?;let v=candidate(&a.partitions[id]).ok_or(Failure::State)?[k];a.partitions[child].prefix.push(if k==j{!v}else{v});}
 }
 m.pay(1,0)?;if a.queue.last()!=Some(&id){return Err(Failure::State)}a.queue.pop();
 for i in 0..a.partitions[id].children.len(){m.pay(1,0)?;a.queue.push(a.partitions[id].children[i]);}
 a.partitions[id].status=Status::Excluded;a.active=None;Ok(())
}
/// Independent complete-coverage audit. Reconstructs every original+prefix
/// system and every disjoint complement split, rechecks all native proofs.
/// Caller exclusions remain explicit assumptions, not certified group results.
pub struct CoverageAudit{pub result:Result<(),Failure>,pub charge:Charge,pub partitions:usize,pub exclusions:usize,pub checks:Vec<check::Audit>}
pub fn audit(n:u32,g:&[Polynomial],a:&Attempt,limits:Limits,deadline:Instant,owner:&mut impl Owner)->CoverageAudit{
 let mut m=Meter{owner,limits,deadline,charge:Charge::default()};let mut out=CoverageAudit{result:Ok(()),charge:Charge::default(),partitions:0,exclusions:0,checks:Vec::new()};
 out.result=(||{
  m.pay(1,0)?;if a.failure.is_some()||a.outcome!=Outcome::RemainingBooleanModelsExhaustedUnderRecordedExclusions||a.active.is_some()||!a.queue.is_empty()||a.partitions.is_empty()||a.partitions.len()>limits.partitions{return Err(Failure::Structure)}
  m.reserve(&mut out.checks,a.partitions.len())?;
  for(i,p)in a.partitions.iter().enumerate(){
   m.pay(1,0)?;if p.prefix.len()>n as usize{return Err(Failure::Structure)}
   if i==0{if p.parent.is_some()||!p.prefix.is_empty(){return Err(Failure::Structure)}}else{
    let parent=p.parent.ok_or(Failure::Structure)?;if parent>=i{return Err(Failure::Structure)}let q=&a.partitions[parent];m.scan(q.children.len())?;if q.children.iter().filter(|j|**j==i).count()!=1{return Err(Failure::Structure)}
   }
   if p.equations.len()!=g.len().checked_add(p.prefix.len()).ok_or(Failure::Overflow)?{return Err(Failure::Structure)}
   for(j,q)in g.iter().enumerate(){m.pay(1,0)?;for t in q{m.scan(t.len())?}for t in &p.equations[j]{m.scan(t.len())?}if q!=&p.equations[j]{return Err(Failure::Structure)}}
   for(j,value)in p.prefix.iter().enumerate(){let q=&p.equations[g.len()+j];m.scan(q.len())?;for t in q{m.scan(t.len())?}let v=j as u32;let correct=if *value{q.len()==2&&q[0].is_empty()&&q[1].as_slice()==[v]}else{q.len()==1&&q[0].as_slice()==[v]};if !correct{return Err(Failure::Structure)}}
   let native=p.search.as_ref().ok_or(Failure::Structure)?;let mut bridge=Bridge{meter:&mut m,failure:None};let checked=check::verify(n,&p.equations,native,limits.audit,deadline,&mut bridge);let failed=bridge.failure;let result=checked.result;out.checks.push(checked);if let Some(e)=failed{return Err(e)}if result.is_err(){return Err(Failure::Audit)}
   match p.status{
    Status::Refuted=>{if native.verdict!=search::Verdict::UnsatDerived||!p.children.is_empty()||p.exclusion_record.is_some(){return Err(Failure::Structure)}},
    Status::Excluded=>{
     if p.exclusion_record.is_none(){return Err(Failure::Structure)}let x=candidate(p).ok_or(Failure::Structure)?;if x.len()!=n as usize||p.children.len()!=x.len()-p.prefix.len(){return Err(Failure::Structure)}m.scan(p.prefix.len())?;if x[..p.prefix.len()]!=p.prefix{return Err(Failure::Structure)}
     for(k,&id)in p.children.iter().enumerate(){m.pay(1,0)?;if id<=i||id>=a.partitions.len(){return Err(Failure::Structure)}let child=&a.partitions[id];let j=p.prefix.len()+k;if child.parent!=Some(i)||child.prefix.len()!=j+1{return Err(Failure::Structure)}m.scan(j+1)?;if child.prefix[..j]!=x[..j]||child.prefix[j]==x[j]{return Err(Failure::Structure)}}out.exclusions+=1;
    },_=>return Err(Failure::Structure),
   }out.partitions+=1;
  }m.pay(1,0)
 })();out.charge=m.charge;out
}
