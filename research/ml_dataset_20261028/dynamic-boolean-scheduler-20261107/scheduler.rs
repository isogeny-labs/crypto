//! Arbitrary-width Boolean ideal closure by sparse row elimination.
//! Degree-priority ordinary pairs and leading-variable Boolean field pairs.
//! Uses the quotient-aware field-pair construction of pq_groebner_f2.
//! This sparse scheduler is not a batched F4 implementation or F5.
use super::sparse::{Charge, Monomial, Owner, Polynomial};
use std::{cmp::Ordering, mem::size_of, time::Instant};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Failure { Cap, Owner, Deadline, Allocation, Overflow, Malformed }
#[derive(Clone, Copy)]
pub struct Limits { pub variables:u32, pub generators:usize, pub basis:usize, pub tasks:usize, pub terms:usize, pub proof:usize, pub degree:usize, pub work:u64, pub bytes:u64 }
#[derive(Debug, PartialEq, Eq)]
pub struct ProofTerm { pub generator:usize, pub multiplier:Monomial }
#[derive(Debug, Default)]
pub struct Row { pub polynomial:Polynomial, pub proof:Vec<ProofTerm> }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind { Original(usize), Pair(usize,usize), Field(usize,u32) }
#[derive(Debug, Clone, Copy)]
pub struct Task { pub kind:Kind, pub degree:usize, pub serial:u64 }
#[derive(Debug)]
pub struct Attempt { pub result:Result<(),Failure>, pub complete:bool, pub basis:Vec<Row>, pub pending:Vec<Task>, pub active:Option<Task>, pub current:Row, pub charge:Charge, pub completed:u64, pub row_xors:u64, pub symbolic_reducers:u64, pub field_tasks:u64, pub ordinary_tasks:u64, pub last_degree:usize }
fn order(a:&[u32],b:&[u32])->Ordering { a.len().cmp(&b.len()).then_with(||a.cmp(b)) }
struct Meter<'a,T:Owner>{ owner:&'a mut T, limits:Limits, deadline:Instant, charge:Charge }
impl<T:Owner> Meter<'_,T>{
 fn pay(&mut self,w:u64,b:u64)->Result<(),Failure>{
  if Instant::now()>=self.deadline{return Err(Failure::Deadline)}
  let q=Charge{work:self.charge.work.checked_add(w).ok_or(Failure::Overflow)?,bytes:self.charge.bytes.checked_add(b).ok_or(Failure::Overflow)?};
  if q.work>self.limits.work||q.bytes>self.limits.bytes{return Err(Failure::Cap)}
  self.owner.reserve(Charge{work:w,bytes:b}).map_err(|_|Failure::Owner)?;self.charge=q;
  self.owner.check().map_err(|_|Failure::Owner)?;
  if Instant::now()>=self.deadline{return Err(Failure::Deadline)} Ok(())
 }
 fn scan(&mut self,n:usize)->Result<(),Failure>{self.pay(u64::try_from(n).map_err(|_|Failure::Overflow)?.checked_add(1).ok_or(Failure::Overflow)?,0)}
 fn alloc<V>(&mut self,n:usize)->Result<Vec<V>,Failure>{let b=n.checked_mul(size_of::<V>()).and_then(|x|x.checked_add(64)).ok_or(Failure::Overflow)?;self.pay(1,u64::try_from(b).map_err(|_|Failure::Overflow)?)?;let mut v=Vec::new();v.try_reserve_exact(n).map_err(|_|Failure::Allocation)?;Ok(v)}
 fn mono(&mut self,a:&[u32])->Result<Monomial,Failure>{self.scan(a.len())?;let mut v=self.alloc(a.len())?;v.extend_from_slice(a);Ok(v)}
 fn union(&mut self,a:&[u32],b:&[u32])->Result<Monomial,Failure>{self.scan(a.len().checked_add(b.len()).ok_or(Failure::Overflow)?)?;let mut v=self.alloc(a.len().checked_add(b.len()).ok_or(Failure::Overflow)?)?;let(mut i,mut j)=(0,0);while i<a.len()||j<b.len(){let x=if j==b.len()||(i<a.len()&&a[i]<b[j]){let x=a[i];i+=1;x}else if i==a.len()||b[j]<a[i]{let x=b[j];j+=1;x}else{let x=a[i];i+=1;j+=1;x};v.push(x)}if v.len()>self.limits.degree{return Err(Failure::Cap)}Ok(v)}
 fn difference(&mut self,a:&[u32],b:&[u32])->Result<Monomial,Failure>{self.scan(a.len().checked_mul(b.len().max(1)).ok_or(Failure::Overflow)?)?;let mut v=self.alloc(a.len())?;for x in a{if !b.contains(x){v.push(*x)}}Ok(v)}
 fn divides(&mut self,a:&[u32],b:&[u32])->Result<bool,Failure>{self.scan(a.len().checked_mul(b.len().max(1)).ok_or(Failure::Overflow)?)?;Ok(a.iter().all(|x|b.contains(x)))}
 fn cmp(&mut self,a:&[u32],b:&[u32])->Result<Ordering,Failure>{self.scan(a.len().checked_add(b.len()).ok_or(Failure::Overflow)?)?;Ok(order(a,b))}
 fn toggle(&mut self,p:&mut Polynomial,t:Monomial)->Result<(),Failure>{let mut at=0;while at<p.len(){match self.cmp(&p[at],&t)?{Ordering::Equal=>{self.scan(p.len()-at)?;p.remove(at);return Ok(())},Ordering::Greater=>break,_=>at+=1}}if p.len()>=self.limits.terms||p.len()==p.capacity(){return Err(Failure::Cap)}self.scan(p.len()-at)?;p.insert(at,t);Ok(())}
 fn proof_toggle(&mut self,p:&mut Vec<ProofTerm>,t:ProofTerm)->Result<(),Failure>{let mut at=0;while at<p.len(){self.scan(p[at].multiplier.len().checked_add(t.multiplier.len()).ok_or(Failure::Overflow)?)?;match (p[at].generator,&p[at].multiplier).cmp(&(t.generator,&t.multiplier)){Ordering::Equal=>{self.scan(p.len()-at)?;p.remove(at);return Ok(())},Ordering::Greater=>break,_=>at+=1}}if p.len()>=self.limits.proof||p.len()==p.capacity(){return Err(Failure::Cap)}self.scan(p.len()-at)?;p.insert(at,t);Ok(())}
 fn scaled(&mut self,r:&Row,t:&[u32])->Result<Row,Failure>{let mut out=Row{polynomial:self.alloc(r.polynomial.len())?,proof:self.alloc(r.proof.len())?};for a in &r.polynomial{let x=self.union(a,t)?;self.toggle(&mut out.polynomial,x)?}for p in &r.proof{let x=self.union(&p.multiplier,t)?;self.proof_toggle(&mut out.proof,ProofTerm{generator:p.generator,multiplier:x})?}Ok(out)}
 fn xor(&mut self,a:&Row,b:&Row)->Result<Row,Failure>{let mut out=Row{polynomial:self.alloc(a.polynomial.len().checked_add(b.polynomial.len()).ok_or(Failure::Overflow)?)?,proof:self.alloc(a.proof.len().checked_add(b.proof.len()).ok_or(Failure::Overflow)?)?};for r in [a,b]{for t in &r.polynomial{let t=self.mono(t)?;self.toggle(&mut out.polynomial,t)?}for t in &r.proof{let multiplier=self.mono(&t.multiplier)?;self.proof_toggle(&mut out.proof,ProofTerm{generator:t.generator,multiplier})?}}Ok(out)}
 fn validate(&mut self,p:&Polynomial,n:u32)->Result<(),Failure>{if p.len()>self.limits.terms{return Err(Failure::Cap)}for(i,t)in p.iter().enumerate(){self.scan(t.len())?;if t.len()>self.limits.degree{return Err(Failure::Cap)}if t.iter().any(|v|*v>=n)||t.windows(2).any(|w|w[0]>=w[1]){return Err(Failure::Malformed)}if i>0&&self.cmp(&p[i-1],t)?!=Ordering::Less{return Err(Failure::Malformed)}}Ok(())}
}
/// Non-resumable retained attempt: original inputs remain borrowed by caller.
/// Complete means all ordinary and necessary Boolean field pairs processed.
/// Returned basis is not interreduced; no search, F5, or fall-degree claim.
pub fn run(n:u32,g:&[Polynomial],limits:Limits,deadline:Instant,owner:&mut impl Owner)->Attempt{
 let mut a=Attempt{result:Ok(()),complete:false,basis:Vec::new(),pending:Vec::new(),active:None,current:Row::default(),charge:Charge::default(),completed:0,row_xors:0,symbolic_reducers:0,field_tasks:0,ordinary_tasks:0,last_degree:0};
 let mut m=Meter{owner,limits,deadline,charge:Charge::default()};
 a.result=(||{
  m.pay(1,0)?;if n>limits.variables||g.len()>limits.generators||g.len()>limits.tasks{return Err(Failure::Cap)}
  if !g.is_empty()&&limits.proof==0{return Err(Failure::Cap)}
  for p in g{m.pay(1,0)?;m.validate(p,n)?}
  a.basis=m.alloc(limits.basis)?;a.pending=m.alloc(limits.tasks)?;let mut serial=0u64;
  for(i,p)in g.iter().enumerate(){m.pay(1,0)?;a.pending.push(Task{kind:Kind::Original(i),degree:p.last().map_or(0,Vec::len),serial});serial=serial.checked_add(1).ok_or(Failure::Overflow)?;}
  loop{
   m.pay(1,0)?;if a.pending.is_empty(){a.complete=true;return Ok(())}
   m.scan(a.pending.len())?;let k=(0..a.pending.len()).min_by_key(|i|(a.pending[*i].degree,a.pending[*i].serial)).unwrap();let task=a.pending[k];a.active=Some(task);a.last_degree=task.degree;
   if task.degree>limits.degree{return Err(Failure::Cap)}
   let row=match task.kind{
    Kind::Original(i)=>{let mut r=Row{polynomial:m.alloc(g[i].len())?,proof:m.alloc(1)?};for t in &g[i]{r.polynomial.push(m.mono(t)?)}r.proof.push(ProofTerm{generator:i,multiplier:m.mono(&[])?});r},
    Kind::Field(i,v)=>m.scaled(&a.basis[i],&[v])?,
    Kind::Pair(i,j)=>{let x=a.basis[i].polynomial.last().unwrap();let y=a.basis[j].polynomial.last().unwrap();let lcm=m.union(x,y)?;let u=m.difference(&lcm,x)?;let v=m.difference(&lcm,y)?;let left=m.scaled(&a.basis[i],&u)?;let right=m.scaled(&a.basis[j],&v)?;m.xor(&left,&right)?}
   };a.current=row;
   // On-demand admitted symbolic reducer multiples become actual sparse rows.
   // Quotient multiplication can cancel a leading term: use a scaled reducer
   // only if its computed leading column exactly matches the active column.
   loop{
    m.pay(1,0)?;let Some(lead)=a.current.polynomial.last()else{break};
    let mut reducer=None;
    for r in &a.basis{
     let pivot=r.polynomial.last().unwrap();
     if m.divides(pivot,lead)?{
      let multiplier=m.difference(lead,pivot)?;
      let candidate=m.scaled(r,&multiplier)?;
      if let Some(actual)=candidate.polynomial.last(){
       if m.cmp(actual,lead)?==Ordering::Equal{reducer=Some(candidate);break}
      }
     }
    }
    let Some(reducer)=reducer else{break};
    let next=m.xor(&a.current,&reducer)?;
    a.current=next;a.row_xors=a.row_xors.checked_add(1).ok_or(Failure::Overflow)?;
    a.symbolic_reducers=a.symbolic_reducers.checked_add(1).ok_or(Failure::Overflow)?;
   }
   if !a.current.polynomial.is_empty(){
    if a.basis.len()>=limits.basis{return Err(Failure::Cap)}
    let count=a.current.polynomial.last().unwrap().len().checked_add(a.basis.len()).ok_or(Failure::Overflow)?;
    if a.pending.len().checked_add(count).ok_or(Failure::Overflow)?>limits.tasks{return Err(Failure::Cap)}
    let mut new=m.alloc(count)?;let i=a.basis.len();let lead=a.current.polynomial.last().unwrap();
    for(j,r)in a.basis.iter().enumerate(){let lcm=m.union(lead,r.polynomial.last().unwrap())?;new.push(Task{kind:Kind::Pair(j,i),degree:lcm.len(),serial});serial=serial.checked_add(1).ok_or(Failure::Overflow)?;}
    // Field equation x_v^2+x_v paired with g only when v divides LM(g).
    // Queue formal degree LM(g)+1 as the existing reference does.
    for &v in lead{m.pay(1,0)?;let degree=lead.len().checked_add(1).ok_or(Failure::Overflow)?;new.push(Task{kind:Kind::Field(i,v),degree,serial});serial=serial.checked_add(1).ok_or(Failure::Overflow)?;}
    // Commit row and all generated tasks together; capacities already reserved.
    m.scan(new.len())?;a.basis.push(std::mem::take(&mut a.current));a.pending.extend(new);
   }
   a.pending.swap_remove(k);a.active=None;a.completed=a.completed.checked_add(1).ok_or(Failure::Overflow)?;
   match task.kind{Kind::Field(_,_)=>a.field_tasks=a.field_tasks.checked_add(1).ok_or(Failure::Overflow)?,Kind::Pair(_,_)=>a.ordinary_tasks=a.ordinary_tasks.checked_add(1).ok_or(Failure::Overflow)?,_=>()}
  }
 })();a.charge=m.charge;a
}
