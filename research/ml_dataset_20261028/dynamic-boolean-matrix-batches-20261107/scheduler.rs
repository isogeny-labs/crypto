//! Arbitrary-width Boolean ideal closure by sparse row elimination.
//! Degree-priority ordinary pairs and leading-variable Boolean field pairs.
//! Uses the quotient-aware field-pair construction of pq_groebner_f2.
//! Same-degree candidates share symbolic preprocessing and sparse matrix pivots.
//! No F5, solution-search, or authenticated dataset claim.
use super::sparse::{Charge, Monomial, Owner, Polynomial};
use std::{cmp::Ordering, mem::size_of, time::Instant};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Failure { Cap, Owner, Deadline, Allocation, Overflow, Malformed }
#[derive(Clone, Copy)]
pub struct Limits { pub batch_rows:usize, pub batch_ops:usize, pub batches:usize, pub variables:u32, pub generators:usize, pub basis:usize, pub tasks:usize, pub terms:usize, pub proof:usize, pub degree:usize, pub work:u64, pub bytes:u64 }
#[derive(Debug, PartialEq, Eq)]
pub struct ProofTerm { pub generator:usize, pub multiplier:Monomial }
#[derive(Debug, Default)]
pub struct Row { pub polynomial:Polynomial, pub proof:Vec<ProofTerm> }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind { Original(usize), Pair(usize,usize), Field(usize,u32) }
#[derive(Debug, Clone, Copy)]
pub struct Task { pub kind:Kind, pub degree:usize, pub serial:u64 }
#[derive(Debug)]
pub struct Attempt { pub batches:Vec<Batch>, pub result:Result<(),Failure>, pub complete:bool, pub basis:Vec<Row>, pub pending:Vec<Task>, pub active:Option<Task>, pub current:Row, pub charge:Charge, pub completed:u64, pub row_xors:u64, pub symbolic_reducers:u64, pub field_tasks:u64, pub ordinary_tasks:u64, pub last_degree:usize }
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

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Stage { Build, Symbolic, Eliminate, PrepareCommit, Committed }
#[derive(Debug)]
pub struct Reducer { pub basis:usize, pub multiplier:Monomial, pub row:usize }
#[derive(Debug,Clone,Copy)]
pub struct Pivot { pub target:usize, pub source:usize }
#[derive(Debug)]
pub struct Batch {
 pub basis_before:usize, pub degree:usize, pub tasks:Vec<Task>, pub initial:Vec<Row>, pub matrix:Vec<Row>,
 pub reducers:Vec<Reducer>, pub operations:Vec<Pivot>, pub stage:Stage,
 pub row_cursor:usize, pub term_cursor:usize, pub basis_cursor:usize,
 pub order:Vec<usize>, pub pivots:Vec<usize>, pub staged_basis:Vec<Row>,
 pub staged_tasks:Vec<Task>, pub additions:Vec<(usize,usize)>,
}
fn build_task<T:Owner>(task:Task,g:&[Polynomial],basis:&[Row],m:&mut Meter<T>)->Result<Row,Failure>{
 match task.kind{
 Kind::Original(i)=>{let mut r=Row{polynomial:m.alloc(g[i].len())?,proof:m.alloc(1)?};for t in &g[i]{r.polynomial.push(m.mono(t)?)}r.proof.push(ProofTerm{generator:i,multiplier:m.mono(&[])?});Ok(r)},
 Kind::Field(i,v)=>m.scaled(&basis[i],&[v]),
 Kind::Pair(i,j)=>{let x=basis[i].polynomial.last().unwrap();let y=basis[j].polynomial.last().unwrap();let lcm=m.union(x,y)?;let u=m.difference(&lcm,x)?;let v=m.difference(&lcm,y)?;let l=m.scaled(&basis[i],&u)?;let r=m.scaled(&basis[j],&v)?;m.xor(&l,&r)} }
}
/// Bounded degree-batch matrix scheduler. All admitted rows and operations remain
/// owned by the attempt. Complete means queued ordinary/Boolean pairs exhausted;
/// no reduced-basis, F5, solution-search, or original-source admission claim.
pub fn run(n:u32,g:&[Polynomial],limits:Limits,deadline:Instant,owner:&mut impl Owner)->Attempt{
 let mut a=Attempt{batches:Vec::new(),result:Ok(()),complete:false,basis:Vec::new(),pending:Vec::new(),active:None,current:Row::default(),charge:Charge::default(),completed:0,row_xors:0,symbolic_reducers:0,field_tasks:0,ordinary_tasks:0,last_degree:0};
 let mut m=Meter{owner,limits,deadline,charge:Charge::default()};
 a.result=(||{
  m.pay(1,0)?;if n>limits.variables||g.len()>limits.generators||g.len()>limits.tasks{return Err(Failure::Cap)}
  if !g.is_empty()&&limits.proof==0{return Err(Failure::Cap)}
  for p in g{m.pay(1,0)?;m.validate(p,n)?}
  // Keep V2 original validation/task admission ordering unchanged.
  a.basis=m.alloc(limits.basis)?;a.pending=m.alloc(limits.tasks)?;let mut serial=0u64;
  for(i,p)in g.iter().enumerate(){m.pay(1,0)?;a.pending.push(Task{kind:Kind::Original(i),degree:p.last().map_or(0,Vec::len),serial});serial=serial.checked_add(1).ok_or(Failure::Overflow)?;}
  a.batches=m.alloc(limits.batches)?;
  loop{
   m.pay(1,0)?;if a.pending.is_empty(){a.complete=true;return Ok(())}
   m.scan(a.pending.len())?;let degree=a.pending.iter().map(|t|t.degree).min().unwrap();a.last_degree=degree;
   if degree>limits.degree||a.batches.len()==limits.batches{return Err(Failure::Cap)}
   let batch=Batch{basis_before:a.basis.len(),degree,tasks:m.alloc(limits.batch_rows)?,initial:m.alloc(limits.batch_rows)?,matrix:m.alloc(limits.batch_rows)?,reducers:m.alloc(limits.batch_rows)?,operations:m.alloc(limits.batch_ops)?,stage:Stage::Build,row_cursor:0,term_cursor:0,basis_cursor:0,order:m.alloc(limits.batch_rows)?,pivots:m.alloc(limits.batch_rows)?,staged_basis:m.alloc(limits.basis)?,staged_tasks:m.alloc(limits.tasks)?,additions:m.alloc(limits.basis)?};
   a.batches.push(batch);let b=a.batches.last_mut().unwrap();
   for t in &a.pending{m.pay(1,0)?;if t.degree==degree{if b.tasks.len()==limits.batch_rows{return Err(Failure::Cap)}b.tasks.push(*t)}}
   m.scan(b.tasks.len().checked_mul(b.tasks.len()).ok_or(Failure::Overflow)?)?;b.tasks.sort_unstable_by_key(|t|t.serial);
   // Every same-degree candidate is materialized before any pivot operation.
   while b.row_cursor<b.tasks.len(){m.pay(1,0)?;let task=b.tasks[b.row_cursor];a.active=Some(task);let row=build_task(task,g,&a.basis,&mut m)?;
    b.initial.push(row);let row=m.scaled(b.initial.last().unwrap(),&[])?;b.matrix.push(row);b.row_cursor+=1;
   }
   b.stage=Stage::Symbolic;b.row_cursor=0;
   // Fixed-point symbolic preprocessing over all candidate AND reducer terms.
   while b.row_cursor<b.matrix.len(){
    while b.term_cursor<b.matrix[b.row_cursor].polynomial.len(){
     m.pay(1,0)?;let term=m.mono(&b.matrix[b.row_cursor].polynomial[b.term_cursor])?;
     let mut found=false;for e in &b.reducers{m.pay(1,0)?;if m.cmp(b.matrix[e.row].polynomial.last().unwrap(),&term)?==Ordering::Equal{found=true;break}}
     if !found{
      while b.basis_cursor<a.basis.len(){m.pay(1,0)?;let i=b.basis_cursor;let r=&a.basis[i];let lead=r.polynomial.last().unwrap();
       if m.divides(lead,&term)?{
        let multiplier=m.difference(&term,lead)?;let row=m.scaled(r,&multiplier)?;
        if let Some(actual)=row.polynomial.last(){if m.cmp(actual,&term)?==Ordering::Equal{
         if b.matrix.len()==limits.batch_rows{return Err(Failure::Cap)}
         // Retain admitted source row before its working-copy allocation.
         let idx=b.initial.len();b.initial.push(row);b.reducers.push(Reducer{basis:i,multiplier,row:idx});
         let row=m.scaled(b.initial.last().unwrap(),&[])?;b.matrix.push(row);a.symbolic_reducers=a.symbolic_reducers.checked_add(1).ok_or(Failure::Overflow)?;break;
        }}
       }b.basis_cursor+=1;
      }
     }
     b.basis_cursor=0;b.term_cursor+=1;
    }b.term_cursor=0;b.row_cursor+=1;
   }
   // Reducer rows first, then candidates; all share one column/pivot dictionary.
   for e in &b.reducers{m.pay(1,0)?;b.order.push(e.row)}
   for i in 0..b.tasks.len(){m.pay(1,0)?;b.order.push(i)}
   b.stage=Stage::Eliminate;b.row_cursor=0;
   while b.row_cursor<b.order.len(){m.pay(1,0)?;let target=b.order[b.row_cursor];
    loop{
     m.pay(1,0)?;let Some(lead)=b.matrix[target].polynomial.last()else{break};let mut pivot=None;
     for &i in &b.pivots{if m.cmp(lead,b.matrix[i].polynomial.last().unwrap())?==Ordering::Equal{pivot=Some(i);break}}
     let Some(source)=pivot else{b.pivots.push(target);break};
     if b.operations.len()==limits.batch_ops{return Err(Failure::Cap)}
     let row=m.xor(&b.matrix[target],&b.matrix[source])?;
     // Prepay recording and commit the actual row plus matching operation together.
     m.pay(1,0)?;b.matrix[target]=row;b.operations.push(Pivot{target,source});a.row_xors=a.row_xors.checked_add(1).ok_or(Failure::Overflow)?;
    }b.row_cursor+=1;
   }
   b.stage=Stage::PrepareCommit;b.row_cursor=0;
   // Preserve novel reducer rows too: dropping them could hide an unresolved pair.
   while b.row_cursor<b.pivots.len(){m.pay(1,0)?;let idx=b.pivots[b.row_cursor];let lead=b.matrix[idx].polynomial.last().unwrap();let mut old=false;
    for r in &a.basis{if m.divides(r.polynomial.last().unwrap(),lead)?{old=true;break}}
    if !old{
     if a.basis.len().checked_add(b.staged_basis.len()).ok_or(Failure::Overflow)?>=limits.basis{return Err(Failure::Cap)}
     let new_index=a.basis.len()+b.staged_basis.len();let row=m.scaled(&b.matrix[idx],&[])?;
     b.staged_basis.push(row);b.additions.push((idx,new_index));
    }b.row_cursor+=1;
   }
   // Stage all pair/field tasks without changing the old coherent basis or queue.
   b.row_cursor=0;
   while b.row_cursor<b.staged_basis.len(){m.pay(1,0)?;let local=b.row_cursor;let i=a.basis.len()+local;let lead=b.staged_basis[local].polynomial.last().unwrap();
    for j in 0..i{m.pay(1,0)?;if b.staged_tasks.len()==limits.tasks{return Err(Failure::Cap)}let previous=if j<a.basis.len(){&a.basis[j]}else{&b.staged_basis[j-a.basis.len()]};let lcm=m.union(lead,previous.polynomial.last().unwrap())?;b.staged_tasks.push(Task{kind:Kind::Pair(j,i),degree:lcm.len(),serial});serial=serial.checked_add(1).ok_or(Failure::Overflow)?;}
    for &v in lead{m.pay(1,0)?;if b.staged_tasks.len()==limits.tasks{return Err(Failure::Cap)}b.staged_tasks.push(Task{kind:Kind::Field(i,v),degree:lead.len().checked_add(1).ok_or(Failure::Overflow)?,serial});serial=serial.checked_add(1).ok_or(Failure::Overflow)?;}
    b.row_cursor+=1;
   }
   let remaining=a.pending.len().checked_sub(b.tasks.len()).ok_or(Failure::Malformed)?;
   if remaining.checked_add(b.staged_tasks.len()).ok_or(Failure::Overflow)?>limits.tasks{return Err(Failure::Cap)}
   let completed=a.completed.checked_add(b.tasks.len() as u64).ok_or(Failure::Overflow)?;
   let mut fields=a.field_tasks;let mut pairs=a.ordinary_tasks;
   for t in &b.tasks{m.pay(1,0)?;match t.kind{Kind::Field(_,_)=>fields=fields.checked_add(1).ok_or(Failure::Overflow)?,Kind::Pair(_,_)=>pairs=pairs.checked_add(1).ok_or(Failure::Overflow)?,_=>()}}
   // A bounded prepaid move-only commit: no allocation or failure after this check.
   m.scan(a.pending.len().checked_add(b.staged_tasks.len()).and_then(|x|x.checked_add(b.staged_basis.len())).ok_or(Failure::Overflow)?)?;
   a.pending.retain(|t|t.degree!=degree);a.pending.append(&mut b.staged_tasks);a.basis.append(&mut b.staged_basis);
   a.completed=completed;a.field_tasks=fields;a.ordinary_tasks=pairs;b.stage=Stage::Committed;a.active=None;
  }
 })();a.charge=m.charge;a
}
