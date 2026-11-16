//! Exact original CheckedMap -> complete Boolean expansion -> captured generators.
//! The authentic source and every expansion prefix remain owned on all exits.
use super::{provenance::{self,boolean_equations as equations,CheckedMap},wire::{self,Value}};
use std::{mem::size_of,time::Instant};
#[derive(Clone,Copy,Debug)]
pub struct Limits {pub expansion:equations::Limits,pub work:u64,pub bytes:u64}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Failure {Owner,Deadline,Cap,Overflow,Allocation,Missing,Incomplete,Malformed,Mismatch,Expansion(equations::Failure)}
pub struct Audit {expansion:equations::OwnedExpansion,pub result:Result<(),Failure>,pub charge:equations::Charge,pub compared_generators:usize,order:Vec<Vec<usize>>}
impl Audit {
 pub fn source(&self)->&CheckedMap{self.expansion.source()}
 pub fn expansion(&self)->&equations::OwnedExpansion{&self.expansion}
 pub fn original_descent_verified(&self)->bool{self.result.is_ok()}
}
struct Meter<'a,T:wire::Admission>{owner:&'a mut T,limits:Limits,deadline:Instant,charge:equations::Charge,failure:Option<Failure>}
impl<T:wire::Admission> Meter<'_,T>{
 fn stop<U>(&mut self,f:Failure)->Result<U,Failure>{let f=*self.failure.get_or_insert(f);Err(f)}
 fn check(&mut self)->Result<(),Failure>{if let Some(f)=self.failure{return Err(f)}if Instant::now()>=self.deadline{return self.stop(Failure::Deadline)}if self.owner.check().is_err(){return self.stop(Failure::Owner)}if Instant::now()>=self.deadline{return self.stop(Failure::Deadline)}Ok(())}
 fn pay(&mut self,w:u64,b:u64)->Result<(),Failure>{self.check()?;let Some(work)=self.charge.work.checked_add(w)else{return self.stop(Failure::Overflow)};let Some(bytes)=self.charge.bytes.checked_add(b)else{return self.stop(Failure::Overflow)};if work>self.limits.work||bytes>self.limits.bytes{return self.stop(Failure::Cap)}if self.owner.reserve(w,b).is_err(){return self.stop(Failure::Owner)}self.charge=equations::Charge{work,bytes};self.check()}
 fn field<'v>(&mut self,v:&'v Value,key:&str)->Result<&'v Value,Failure>{let fields=v.as_object().ok_or(Failure::Malformed)?;let mut found=None;for(k,v)in fields{self.pay(u64::try_from(k.len()).map_err(|_|Failure::Overflow)?.checked_add(1).ok_or(Failure::Overflow)?,0)?;if k==key{if found.is_some(){return Err(Failure::Malformed)}found=Some(v)}}found.ok_or(Failure::Missing)}
 fn array<'v>(&mut self,v:&'v Value)->Result<&'v [Value],Failure>{self.pay(1,0)?;v.as_array().ok_or(Failure::Malformed)}
 fn reserve<V>(&mut self,v:&mut Vec<V>,count:usize)->Result<(),Failure>{let b=count.checked_mul(size_of::<V>()).and_then(|x|x.checked_add(64)).ok_or(Failure::Overflow)?;self.pay(1,u64::try_from(b).map_err(|_|Failure::Overflow)?)?;v.try_reserve_exact(count).map_err(|_|Failure::Allocation)}
 fn monomial(&mut self,captured:&Value,actual:&[u32])->Result<(),Failure>{let values=self.array(captured)?;if values.len()!=actual.len(){return Err(Failure::Mismatch)}for(v,n)in values.iter().zip(actual){self.pay(1,0)?;if v.as_u64()!=Some(*n as u64){return Err(Failure::Mismatch)}}Ok(())}
}
impl<T:wire::Admission> provenance::Admission for Meter<'_,T>{
 fn reserve(&mut self,q:provenance::Quote)->Result<(),provenance::Error>{self.pay(q.work,q.bytes).map_err(|_|provenance::Error::Admission)}
 fn check(&mut self)->Result<(),provenance::Error>{Meter::check(self).map_err(|_|provenance::Error::Admission)}
}
/// `map` must come from the caller's authenticated original-map construction.
/// Moving it here retains all failures without reconstructing or resetting its owner.
/// Success establishes the authenticated pattern's auxiliary Boolean gate equations,
/// not a signed PDP relation, negative result, or all-pattern exhaustion.
/// Both local expansion caps and total checker caps constrain the same caller ledger.
pub fn verify(map:CheckedMap,root:&Value,limits:Limits,deadline:Instant,owner:&mut impl wire::Admission)->Audit {
 let mut meter=Meter{owner,limits,deadline,charge:equations::Charge::default(),failure:None};
 let expansion=equations::expand_owned(map,limits.expansion,&mut meter);
 let mut out=Audit{expansion,result:Err(Failure::Missing),charge:equations::Charge::default(),compared_generators:0,order:Vec::new()};
 out.result=(||{
  if let Some(f)=meter.failure{return Err(f)}
  out.expansion.status().map_err(Failure::Expansion)?;meter.check()?;
  let captured=meter.field(root,"expansion")?;
  if matches!(captured,Value::Null){return Err(Failure::Missing)}
  if meter.field(captured,"variables")?.as_u64()!=Some(out.source().variables() as u64){return Err(Failure::Mismatch)}
  let status=meter.field(captured,"status")?.as_str().ok_or(Failure::Malformed)?;
  let stage=meter.field(captured,"stage")?.as_str().ok_or(Failure::Malformed)?;
  if !matches!(stage,"And"|"Xor"|"Unit"|"Impossible"|"Domain"|"Done"){return Err(Failure::Malformed)}
  match status {
   "Ok(())"=>if stage!="Done"{return Err(Failure::Mismatch)},
   "Err(Unknown(Cap))"|"Err(Unknown(Overflow))"|"Err(Unknown(Allocation))"|"Err(Unknown(Owner))"=>return Err(if stage=="Done"{Failure::Mismatch}else{Failure::Incomplete}),
   "Err(InvalidWire)"=>return Err(Failure::Mismatch),
   _=>return Err(Failure::Malformed),
  }
  if !matches!(meter.field(captured,"owner_failure")?,Value::Null){return Err(Failure::Mismatch)}
  for key in ["partial","scratch"]{let p=meter.field(captured,key)?;if !meter.array(p)?.is_empty(){return Err(Failure::Mismatch)}}
  let raw=meter.field(captured,"equations")?;let raw=meter.array(raw)?;
  let normalized=meter.field(root,"normalized")?;let normalized=meter.array(normalized)?;
  let actual=out.expansion.equations();
  if raw.len()!=actual.len()||normalized.len()!=actual.len(){return Err(Failure::Mismatch)}
  meter.reserve(&mut out.order,actual.len())?;
  for(g,polynomial)in actual.iter().enumerate(){
   let lex=meter.array(&raw[g])?;let degreelex=meter.array(&normalized[g])?;
   if lex.len()!=polynomial.len()||degreelex.len()!=polynomial.len(){return Err(Failure::Mismatch)}
   for(c,t)in lex.iter().zip(polynomial){meter.monomial(c,t)?}
   out.order.push(Vec::new());let order=&mut out.order[g];meter.reserve(order,polynomial.len())?;
   for t in 0..polynomial.len(){meter.pay(1,0)?;order.push(t)}
   // Same degree-then-lex order as producer normalize; no reordering generators,
   // no deduplication, and no omitted zero generators. Only index swaps allocate.
   for i in 1..order.len(){let mut j=i;while j>0{
    let a=&polynomial[order[j-1]];let b=&polynomial[order[j]];
    let cost=a.len().checked_add(b.len()).and_then(|x|x.checked_add(2)).ok_or(Failure::Overflow)?;
    meter.pay(u64::try_from(cost).map_err(|_|Failure::Overflow)?,0)?;
    if a.len().cmp(&b.len()).then_with(||a.cmp(b))!=std::cmp::Ordering::Greater{break}order.swap(j-1,j);j-=1;
   }}
   for(c,&i)in degreelex.iter().zip(order.iter()){meter.monomial(c,&polynomial[i])?}
   out.compared_generators+=1;
  }
  meter.check()
 })();out.charge=meter.charge;out
}
