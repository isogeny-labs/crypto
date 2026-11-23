//! Versioned tagged wire and byte-bounded streaming. No data-sized clone.
use std::{fmt::{self,Debug,Write},time::Instant};
#[derive(Clone,Copy)]pub struct Limits{pub chunk_bytes:usize,pub total_bytes:u64,pub chunks:u64,pub work:u64}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Failure{Cap,Owner,Deadline,Overflow,Allocation,Publish,Format}
pub trait AdmissionSink{fn reserve(&mut self,work:u64,bytes:u64)->Result<(),()>;fn check(&mut self)->Result<(),()>;fn publish(&mut self,index:u64,bytes:&[u8])->Result<(),()>;}
pub struct Attempt{pub pending:Vec<u8>,pub acknowledged_chunks:u64,pub acknowledged_bytes:u64,pub encoded_bytes:u64,pub work:u64,pub allocation_bytes:u64,pub result:Result<(),Failure>,pub complete:bool}
pub struct Stream<'a,T:AdmissionSink>{pub out:Attempt,io:&'a mut T,limits:Limits,deadline:Instant}
impl<T:AdmissionSink> Stream<'_,T>{
 fn check(&mut self)->Result<(),Failure>{if Instant::now()>=self.deadline{return Err(Failure::Deadline)}self.io.check().map_err(|_|Failure::Owner)?;if Instant::now()>=self.deadline{return Err(Failure::Deadline)}Ok(())}
 fn pay(&mut self,work:u64,bytes:u64)->Result<(),Failure>{self.check()?;let next=self.out.work.checked_add(work).ok_or(Failure::Overflow)?;if next>self.limits.work{return Err(Failure::Cap)}self.io.reserve(work,bytes).map_err(|_|Failure::Owner)?;self.out.work=next;self.out.allocation_bytes=self.out.allocation_bytes.checked_add(bytes).ok_or(Failure::Overflow)?;self.check()}
 fn flush(&mut self)->Result<(),Failure>{if self.out.pending.is_empty(){return Ok(())}self.check()?;if self.out.acknowledged_chunks>=self.limits.chunks{return Err(Failure::Cap)}let next=self.out.acknowledged_chunks.checked_add(1).ok_or(Failure::Overflow)?;let bytes=self.out.acknowledged_bytes.checked_add(self.out.pending.len() as u64).ok_or(Failure::Overflow)?;self.io.publish(self.out.acknowledged_chunks,&self.out.pending).map_err(|_|Failure::Publish)?;self.out.acknowledged_chunks=next;self.out.acknowledged_bytes=bytes;self.out.pending.clear();self.check()}
 pub fn raw(&mut self,mut bytes:&[u8])->Result<(),Failure>{while !bytes.is_empty(){self.check()?;if self.out.pending.len()==self.limits.chunk_bytes{self.flush()?}let n=bytes.len().min(self.limits.chunk_bytes-self.out.pending.len());let next=self.out.encoded_bytes.checked_add(n as u64).ok_or(Failure::Overflow)?;if next>self.limits.total_bytes{return Err(Failure::Cap)}self.pay((n as u64).checked_add(1).ok_or(Failure::Overflow)?,0)?;self.out.pending.extend_from_slice(&bytes[..n]);self.out.encoded_bytes=next;bytes=&bytes[n..];}Ok(())}
 pub fn null(&mut self)->Result<(),Failure>{self.raw(&[0])}
 pub fn boolean(&mut self,v:bool)->Result<(),Failure>{self.raw(&[1,v as u8])}
 pub fn number(&mut self,v:u64)->Result<(),Failure>{self.raw(&[2])?;self.raw(&v.to_le_bytes())}
 pub fn usize(&mut self,v:usize)->Result<(),Failure>{self.number(u64::try_from(v).map_err(|_|Failure::Overflow)?)}
 fn length(&mut self,tag:u8,n:usize)->Result<(),Failure>{self.raw(&[tag])?;self.raw(&u64::try_from(n).map_err(|_|Failure::Overflow)?.to_le_bytes())}
 pub fn bytes(&mut self,v:&[u8])->Result<(),Failure>{self.length(4,v.len())?;self.raw(v)}
 pub fn text(&mut self,v:&str)->Result<(),Failure>{self.length(3,v.len())?;self.raw(v.as_bytes())}
 pub fn list(&mut self,n:usize)->Result<(),Failure>{self.length(5,n)}
 pub fn map(&mut self,n:usize)->Result<(),Failure>{self.length(6,n)}
 /// Only finite scalar enums/results enter here; nested payloads use typed walks.
 pub fn debug(&mut self,v:&impl Debug)->Result<(),Failure>{self.pay(512,0)?;let mut b=Small{bytes:[0;256],len:0};write!(&mut b,"{v:?}").map_err(|_|Failure::Format)?;let s=std::str::from_utf8(&b.bytes[..b.len]).map_err(|_|Failure::Format)?;self.text(s)}
 pub fn option<V>(&mut self,v:Option<&V>,f:impl FnOnce(&mut Self,&V)->Result<(),Failure>)->Result<(),Failure>{match v{Some(v)=>f(self,v),None=>self.null()}}
 pub fn opt_usize(&mut self,v:Option<usize>)->Result<(),Failure>{match v{Some(v)=>self.usize(v),None=>self.null()}}
 pub fn opt_u64(&mut self,v:Option<u64>)->Result<(),Failure>{match v{Some(v)=>self.number(v),None=>self.null()}}
 pub fn array<V>(&mut self,v:&[V],mut f:impl FnMut(&mut Self,&V)->Result<(),Failure>)->Result<(),Failure>{self.list(v.len())?;for x in v{f(self,x)?}Ok(())}
}
struct Small{bytes:[u8;256],len:usize}
impl Write for Small{fn write_str(&mut self,s:&str)->fmt::Result{let end=self.len.checked_add(s.len()).ok_or(fmt::Error)?;if end>self.bytes.len(){return Err(fmt::Error)}self.bytes[self.len..end].copy_from_slice(s.as_bytes());self.len=end;Ok(())}}
pub fn run<T:AdmissionSink>(limits:Limits,deadline:Instant,io:&mut T,f:impl FnOnce(&mut Stream<'_,T>)->Result<(),Failure>)->Attempt{
 let mut s=Stream{out:Attempt{pending:Vec::new(),acknowledged_chunks:0,acknowledged_bytes:0,encoded_bytes:0,work:0,allocation_bytes:0,result:Ok(()),complete:false},io,limits,deadline};
 s.out.result=(||{if limits.chunk_bytes==0||limits.chunks==0{return Err(Failure::Cap)}let alloc=u64::try_from(limits.chunk_bytes).map_err(|_|Failure::Overflow)?.checked_add(64).ok_or(Failure::Overflow)?;s.pay(1,alloc)?;s.out.pending.try_reserve_exact(limits.chunk_bytes).map_err(|_|Failure::Allocation)?;s.check()?;s.raw(b"OWNSEARCH\x01")?;f(&mut s)?;s.flush()?;s.check()?;s.out.complete=true;Ok(())})();s.out
}
