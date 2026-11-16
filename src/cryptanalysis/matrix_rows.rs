//! Bounded sparse Boolean row construction. Arithmetic only, not source authority.
use super::sparse::{Charge, Monomial, Owner, Polynomial};
use std::{mem::size_of, time::Instant};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Failure { Malformed, Cap, Overflow, Allocation, Owner, Incomplete, Checker }
#[derive(Clone, Copy)]
pub struct Limits {
 pub variables:u32, pub generators:usize, pub window:usize, pub candidates:usize,
 pub basis:usize, pub terms:usize, pub degree:usize, pub proof:usize,
 pub wire:usize, pub work:u64, pub bytes:u64,
}
#[derive(Debug, Default)]
pub struct Row { pub polynomial:Polynomial, pub proof:Vec<usize> }
#[derive(Debug)]
pub struct Attempt {
 pub result:Result<(),Failure>, pub basis:Vec<Row>, pub current:Row,
 pub cursor:usize, pub candidates_completed:usize, pub reductions:u64, pub zero_rows:usize,
 pub charge:Charge,
}
struct Meter<'a,T:Owner> { owner:&'a mut T, limits:Limits, charge:Charge }
impl<T:Owner> Meter<'_,T> {
 fn pay(&mut self,w:u64,b:u64)->Result<(),Failure> {
  let q=Charge{work:self.charge.work.checked_add(w).ok_or(Failure::Overflow)?,bytes:self.charge.bytes.checked_add(b).ok_or(Failure::Overflow)?};
  if q.work>self.limits.work||q.bytes>self.limits.bytes{return Err(Failure::Cap)}
  self.owner.reserve(Charge{work:w,bytes:b}).map_err(|_|Failure::Owner)?;
  self.charge=q; self.owner.check().map_err(|_|Failure::Owner)
 }
 fn vec<V>(&mut self,n:usize)->Result<Vec<V>,Failure> {
  let b=n.checked_mul(size_of::<V>()).and_then(|n|n.checked_add(64)).ok_or(Failure::Overflow)?;
  self.pay(1,u64::try_from(b).map_err(|_|Failure::Overflow)?)?;
  let mut out=Vec::new();out.try_reserve_exact(n).map_err(|_|Failure::Allocation)?;Ok(out)
 }
 fn scan(&mut self,n:usize)->Result<(),Failure>{self.pay(u64::try_from(n).map_err(|_|Failure::Overflow)?.checked_add(1).ok_or(Failure::Overflow)?,0)}
 fn mono(&mut self,a:&[u32])->Result<Monomial,Failure>{self.scan(a.len())?;let mut v=self.vec(a.len())?;v.extend_from_slice(a);Ok(v)}
 fn union(&mut self,a:&[u32],b:&[u32])->Result<Monomial,Failure>{
  let cap=a.len().checked_add(b.len()).ok_or(Failure::Overflow)?;self.scan(cap)?;let mut v=self.vec(cap)?;
  let(mut i,mut j)=(0,0);while i<a.len()||j<b.len(){
   let x=if j==b.len()||(i<a.len()&&a[i]<b[j]){let x=a[i];i+=1;x}else if i==a.len()||b[j]<a[i]{let x=b[j];j+=1;x}else{let x=a[i];i+=1;j+=1;x};v.push(x);
  }
  if v.len()>self.limits.degree{return Err(Failure::Cap)}Ok(v)
 }
 fn compare(&mut self,a:&[u32],b:&[u32])->Result<std::cmp::Ordering,Failure>{self.scan(a.len().checked_add(b.len()).ok_or(Failure::Overflow)?)?;Ok(a.cmp(b))}
 fn poly_xor(&mut self,a:&Polynomial,b:&Polynomial)->Result<Polynomial,Failure>{
  // No uncharged geometric growth; source-size scratch can exceed final row cap.
  let cap=a.len().checked_add(b.len()).ok_or(Failure::Overflow)?;let mut out=self.vec(cap)?;let(mut i,mut j)=(0,0);
  while i<a.len()||j<b.len(){
   let c=if i==a.len(){std::cmp::Ordering::Greater}else if j==b.len(){std::cmp::Ordering::Less}else{self.compare(&a[i],&b[j])?};
   match c {std::cmp::Ordering::Equal=>{i+=1;j+=1},std::cmp::Ordering::Less=>{out.push(self.mono(&a[i])?);i+=1},std::cmp::Ordering::Greater=>{out.push(self.mono(&b[j])?);j+=1}}
   if out.len()>self.limits.terms{return Err(Failure::Cap)}
  }Ok(out)
 }
 fn ids_xor(&mut self,a:&[usize],b:&[usize])->Result<Vec<usize>,Failure>{
  let cap=a.len().checked_add(b.len()).ok_or(Failure::Overflow)?;self.scan(cap)?;let mut out=self.vec(cap)?;let(mut i,mut j)=(0,0);
  while i<a.len()||j<b.len(){if j==b.len()||(i<a.len()&&a[i]<b[j]){out.push(a[i]);i+=1}else if i==a.len()||b[j]<a[i]{out.push(b[j]);j+=1}else{i+=1;j+=1}if out.len()>self.limits.proof{return Err(Failure::Cap)}}Ok(out)
 }
 fn validate_mono(&mut self,a:&[u32],n:u32)->Result<(),Failure>{self.scan(a.len())?;if a.iter().any(|v|*v>=n)||a.windows(2).any(|w|w[0]>=w[1]){return Err(Failure::Malformed)}if a.len()>self.limits.degree{return Err(Failure::Cap)}Ok(())}
 fn validate_poly(&mut self,p:&Polynomial,n:u32)->Result<(),Failure>{if p.len()>self.limits.terms{return Err(Failure::Cap)}for(i,t)in p.iter().enumerate(){self.validate_mono(t,n)?;if i>0&&self.compare(&p[i-1],t)?!=std::cmp::Ordering::Less{return Err(Failure::Malformed)}}Ok(())}
}
pub fn reduce(n:u32,g:&[Polynomial],window:&[Monomial],limits:Limits,owner:&mut impl Owner)->Attempt{
 let mut a=Attempt{result:Ok(()),basis:Vec::new(),current:Row::default(),cursor:0,candidates_completed:0,reductions:0,zero_rows:0,charge:Charge::default()};
 let mut m=Meter{owner,limits,charge:Charge::default()};
 a.result=(||{
  m.pay(1,0)?;
  if n>limits.variables||g.len()>limits.generators||window.len()>limits.window{return Err(Failure::Cap)}
  let count=g.len().checked_mul(window.len()).ok_or(Failure::Overflow)?;if count>limits.candidates{return Err(Failure::Cap)}
  for p in g {m.validate_poly(p,n)?}
  for(i,t)in window.iter().enumerate(){m.validate_mono(t,n)?;if i>0&&m.compare(&window[i-1],t)?!=std::cmp::Ordering::Less{return Err(Failure::Malformed)}}
  a.basis=m.vec(limits.basis.min(count))?;
  for (gi,p)in g.iter().enumerate(){for(wi,t)in window.iter().enumerate(){
   a.cursor=gi*window.len()+wi;m.pay(1,0)?;
   // Build candidate transactionally; current is the previous coherent row until complete.
   let mut product=m.vec(0)?;
   for term in p {let u=m.union(term,t)?;let mut singleton=m.vec(1)?;singleton.push(u);product=m.poly_xor(&product,&singleton)?;}
   let mut proof=m.vec(1)?;proof.push(a.cursor);if proof.len()>limits.proof{return Err(Failure::Cap)}
   a.current=Row{polynomial:product,proof};
   loop{
    m.pay(1,0)?;let Some(lead)=a.current.polynomial.last()else{a.zero_rows+=1;break};
    let mut pivot=None;for(i,row)in a.basis.iter().enumerate(){if m.compare(lead,row.polynomial.last().unwrap())?==std::cmp::Ordering::Equal{pivot=Some(i);break}}
    let Some(i)=pivot else{if a.basis.len()==limits.basis{return Err(Failure::Cap)}a.basis.push(std::mem::take(&mut a.current));break};
    let polynomial=m.poly_xor(&a.current.polynomial,&a.basis[i].polynomial)?;
    let proof=m.ids_xor(&a.current.proof,&a.basis[i].proof)?;
    a.current=Row{polynomial,proof};a.reductions=a.reductions.checked_add(1).ok_or(Failure::Overflow)?;
   }
   a.candidates_completed+=1;
  }}m.pay(1,0)
 })();a.charge=m.charge;a
}
fn number(out:&mut Vec<u8>,n:usize){let mut b=[0u8;20];let mut n=n;let mut i=20;loop{i-=1;b[i]=b'0'+(n%10)as u8;n/=10;if n==0{break}}out.push(b' ');out.extend_from_slice(&b[i..]);}
fn mono_wire(out:&mut Vec<u8>,m:&Monomial){number(out,m.len());for x in m {number(out,*x as usize)}}
fn poly_wire(out:&mut Vec<u8>,p:&Polynomial){number(out,p.len());for m in p {mono_wire(out,m)}}
fn token_poly(p:&Polynomial)->Result<usize,Failure>{p.iter().try_fold(1usize,|n,m|n.checked_add(1)?.checked_add(m.len())).ok_or(Failure::Overflow)}
/// Internal arithmetic certificate encoding, not an original-authority seal.
pub struct Encoded { pub result:Result<(),Failure>, pub raw:Vec<u8>, pub charge:Charge }
pub fn encode_retained(n:u32,g:&[Polynomial],window:&[Monomial],a:&Attempt,row:usize,limits:Limits,owner:&mut impl Owner)->Encoded{
 let mut out=Vec::new();let mut m=Meter{owner,limits,charge:Charge::default()};
 let result=(||{
 m.pay(1,0)?;
 if a.result.is_err(){return Err(Failure::Incomplete)}let r=a.basis.get(row).ok_or(Failure::Malformed)?;
 if n>limits.variables||g.len()>limits.generators||window.len()>limits.window{return Err(Failure::Cap)}
 let mut tokens=3usize;for p in g{m.validate_poly(p,n)?;tokens=tokens.checked_add(token_poly(p)?).ok_or(Failure::Overflow)?}m.validate_poly(&r.polynomial,n)?;tokens=tokens.checked_add(token_poly(&r.polynomial)?).ok_or(Failure::Overflow)?;
 if r.proof.len()>limits.proof{return Err(Failure::Cap)}
 for (i,id) in r.proof.iter().enumerate(){m.pay(1,0)?;if window.is_empty()||*id/window.len()>=g.len()||i>0&&r.proof[i-1]>=*id{return Err(Failure::Malformed)}m.validate_mono(&window[*id%window.len()],n)?;tokens=tokens.checked_add(2+window[*id%window.len()].len()).ok_or(Failure::Overflow)?;}
 let cap=tokens.checked_mul(21).and_then(|n|n.checked_add(5)).ok_or(Failure::Overflow)?;if cap>limits.wire{return Err(Failure::Cap)}
 m.scan(cap)?;out=m.vec(cap)?;
 out.extend_from_slice(b"BQI1");number(&mut out,n as usize);number(&mut out,g.len());for p in g {poly_wire(&mut out,p)}poly_wire(&mut out,&r.polynomial);number(&mut out,r.proof.len());for id in &r.proof{number(&mut out,*id/window.len());mono_wire(&mut out,&window[*id%window.len()]);}out.push(b'\n');m.pay(1,0)
 })();Encoded{result,raw:out,charge:m.charge}
}
#[cfg(test)]
pub fn encode(n:u32,g:&[Polynomial],window:&[Monomial],a:&Attempt,row:usize,limits:Limits,owner:&mut impl Owner)->Result<Vec<u8>,Failure>{let e=encode_retained(n,g,window,a,row,limits,owner);e.result.map(|_|e.raw)}
/// Prepay checker independently on the SAME owner; its audit authenticates arithmetic only.
pub struct Checked { pub audit:Option<super::bqi::Audit>, pub failure:Option<Failure> }
pub fn independent_check_retained(raw:&[u8],limits:super::bqi::Limits,deadline:Instant,owner:&mut impl Owner)->Checked{
 let mut out=Checked{audit:None,failure:None};
 let result=(||{
 let work=limits.caps.iter().try_fold(0u64,|n,x|n.checked_add(*x)).ok_or(Failure::Overflow)?;
 owner.reserve(Charge{work,bytes:limits.caps[super::bqi::Resource::AllocationBytes as usize]}).map_err(|_|Failure::Owner)?;
 owner.check().map_err(|_|Failure::Owner)?;
 out.audit=Some(super::bqi::verify_untrusted(raw,limits,deadline));
 owner.check().map_err(|_|Failure::Owner)
 })();out.failure=result.err();out
}
#[cfg(test)]
pub fn independent_check(raw:&[u8],limits:super::bqi::Limits,deadline:Instant,owner:&mut impl Owner)->Result<super::bqi::Audit,Failure>{let c=independent_check_retained(raw,limits,deadline,owner);if let Some(f)=c.failure{Err(f)}else{Ok(c.audit.unwrap())}}
