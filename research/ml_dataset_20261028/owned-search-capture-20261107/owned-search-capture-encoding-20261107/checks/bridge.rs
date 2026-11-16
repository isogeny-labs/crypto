//! Arithmetic-only scheduler row bridge to the independent BQI1 checker.
//! No original-source authentication, native-operation or closure claim.
use super::{bqi, scheduler::Row, sparse::{Charge,Owner,Polynomial}};
use std::{mem::size_of,time::Instant};
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Failure { Cap,Overflow,Allocation,Malformed,Owner,Deadline,Checker }
#[derive(Clone,Copy)]
pub struct Limits { pub variables:u32,pub generators:usize,pub terms:usize,pub degree:usize,pub proof:usize,pub wire:usize,pub work:u64,pub bytes:u64 }
pub struct Certificate { pub raw:Vec<u8>,pub charge:Charge,pub audit:Option<bqi::Audit>,pub result:Result<(),Failure> }
struct Meter<'a,T:Owner>{owner:&'a mut T,limits:Limits,deadline:Instant,charge:Charge}
impl<T:Owner> Meter<'_,T>{
 fn pay(&mut self,w:u64,b:u64)->Result<(),Failure>{
  if Instant::now()>=self.deadline{return Err(Failure::Deadline)}
  let q=Charge{work:self.charge.work.checked_add(w).ok_or(Failure::Overflow)?,bytes:self.charge.bytes.checked_add(b).ok_or(Failure::Overflow)?};
  if q.work>self.limits.work||q.bytes>self.limits.bytes{return Err(Failure::Cap)}
  self.owner.reserve(Charge{work:w,bytes:b}).map_err(|_|Failure::Owner)?;self.charge=q;
  self.owner.check().map_err(|_|Failure::Owner)?;
  if Instant::now()>=self.deadline{return Err(Failure::Deadline)}Ok(())
 }
 fn scan(&mut self,n:usize)->Result<(),Failure>{self.pay(u64::try_from(n).map_err(|_|Failure::Overflow)?.checked_add(1).ok_or(Failure::Overflow)?,0)}
 fn vec<V>(&mut self,n:usize)->Result<Vec<V>,Failure>{let bytes=n.checked_mul(size_of::<V>()).and_then(|x|x.checked_add(64)).ok_or(Failure::Overflow)?;self.pay(1,u64::try_from(bytes).map_err(|_|Failure::Overflow)?)?;let mut v=Vec::new();v.try_reserve_exact(n).map_err(|_|Failure::Allocation)?;Ok(v)}
 fn mono(&mut self,t:&[u32],n:u32)->Result<(),Failure>{self.scan(t.len())?;if t.len()>self.limits.degree{return Err(Failure::Cap)}if t.iter().any(|v|*v>=n)||t.windows(2).any(|w|w[0]>=w[1]){return Err(Failure::Malformed)}Ok(())}
 fn number(&mut self,out:&mut Vec<u8>,value:usize)->Result<(),Failure>{let mut buf=[0u8;20];let(mut i,mut x)=(20,value);loop{i-=1;buf[i]=b'0'+(x%10)as u8;x/=10;if x==0{break}}let len=21-i;self.scan(len)?;if out.len().checked_add(len).ok_or(Failure::Overflow)?>self.limits.wire{return Err(Failure::Cap)}out.push(b' ');out.extend_from_slice(&buf[i..]);Ok(())}
 fn monowire(&mut self,out:&mut Vec<u8>,t:&[u32])->Result<(),Failure>{self.number(out,t.len())?;for v in t{self.number(out,*v as usize)?}Ok(())}
 fn poly(&mut self,out:&mut Vec<u8>,p:&Polynomial,n:u32)->Result<(),Failure>{
  self.pay(1,0)?;if p.len()>self.limits.terms{return Err(Failure::Cap)}
  // Scheduler is degree-lex; BQI1 is lex. Sort borrowed indices, preserving
  // generator order and zero generators. Never silently normalize duplicates.
  let mut order:Vec<usize>=self.vec(p.len())?;
  for(i,t)in p.iter().enumerate(){self.mono(t,n)?;
   if i>0{self.scan(p[i-1].len().checked_add(t.len()).ok_or(Failure::Overflow)?)?;if (p[i-1].len(),&p[i-1]) >= (t.len(),t){return Err(Failure::Malformed)}}
   let mut at=0;while at<order.len(){self.scan(p[order[at]].len().checked_add(t.len()).ok_or(Failure::Overflow)?)?;if p[order[at]]>*t{break}at+=1}
   self.scan(order.len()-at)?;order.insert(at,i);
  }
  self.number(out,p.len())?;for i in order{self.monowire(out,&p[i])?}Ok(())
 }
}
/// Verifies one retained row, including rows from a capped scheduler attempt.
/// A successful equality never upgrades a capped attempt to complete closure.
/// Checker maximum envelope is prepaid on the SAME owner and same deadline.
/// This conservative reservation is not an actual hardware instruction count.
pub fn certify(n:u32,g:&[Polynomial],row:&Row,limits:Limits,checker:bqi::Limits,deadline:Instant,owner:&mut impl Owner)->Certificate{
 let mut c=Certificate{raw:Vec::new(),charge:Charge::default(),audit:None,result:Ok(())};let mut m=Meter{owner,limits,deadline,charge:Charge::default()};
 c.result=(||{
  m.pay(1,0)?;if n>limits.variables||g.len()>limits.generators||row.proof.len()>limits.proof||limits.wire<5{return Err(Failure::Cap)}
  c.raw=m.vec(limits.wire)?;m.scan(4)?;c.raw.extend_from_slice(b"BQI1");m.number(&mut c.raw,n as usize)?;m.number(&mut c.raw,g.len())?;
  for p in g{m.poly(&mut c.raw,p,n)?}m.poly(&mut c.raw,&row.polynomial,n)?;m.number(&mut c.raw,row.proof.len())?;
  for(i,t)in row.proof.iter().enumerate(){m.pay(1,0)?;if t.generator>=g.len(){return Err(Failure::Malformed)}m.mono(&t.multiplier,n)?;
   if i>0{let prev=&row.proof[i-1];m.scan(prev.multiplier.len().checked_add(t.multiplier.len()).ok_or(Failure::Overflow)?)?;if (prev.generator,&prev.multiplier)>=(t.generator,&t.multiplier){return Err(Failure::Malformed)}}
   m.number(&mut c.raw,t.generator)?;m.monowire(&mut c.raw,&t.multiplier)?;
  }
  m.pay(1,0)?;if c.raw.len()==limits.wire{return Err(Failure::Cap)}c.raw.push(b'\n');
  let work=checker.caps.iter().try_fold(0u64,|a,b|a.checked_add(*b)).ok_or(Failure::Overflow)?;
  m.pay(work,checker.caps[bqi::Resource::AllocationBytes as usize])?;
  c.audit=Some(bqi::verify_untrusted(&c.raw,checker,deadline));
  m.pay(1,0)?;if c.audit.as_ref().unwrap().result.is_err(){return Err(Failure::Checker)}Ok(())
 })();c.charge=m.charge;c
}
