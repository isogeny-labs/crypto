//! Bounded decision search over the original Boolean equations. No group lift.
use super::*;
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Origin{Original(usize),Branch{parent:usize,variable:u32,value:bool}}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum NodeStatus{Building,Ready,Running,Branching,Branched,Refuted,Witness}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Verdict{Unknown,Witness,UnsatDerived}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum NativeDisposition{Complete,LocalCap,Stopped}
#[derive(Clone,Copy)]pub struct SearchLimits{pub native:Limits,pub nodes:usize}
#[derive(Debug)]pub struct Node{
 pub parent:Option<usize>,pub origins:Vec<Origin>,pub equations:Vec<Polynomial>,pub decisions:Vec<(u32,bool)>,
 pub status:NodeStatus,pub native_disposition:Option<NativeDisposition>,pub native:Option<Attempt>,pub candidate:Vec<bool>,pub candidate_valid:Option<bool>,
 pub children:[Option<usize>;2],pub refutation_row:Option<usize>,
}
#[derive(Debug)]pub struct SearchAttempt{pub result:Result<(),Failure>,pub verdict:Verdict,pub nodes:Vec<Node>,pub queue:Vec<usize>,pub active:Option<usize>,pub witness_node:Option<usize>,pub charge:Charge}
struct NativeOwner<'a,'b,T:Owner>{meter:&'a mut Meter<'b,T>,failure:Option<Failure>}
impl<T:Owner> Owner for NativeOwner<'_,'_,T>{fn reserve(&mut self,q:Charge)->Result<(),()>{self.meter.pay(q.work,q.bytes).map_err(|e|{self.failure=Some(e);})}fn check(&mut self)->Result<(),()>{self.meter.pay(0,0).map_err(|e|{self.failure=Some(e);})}}
fn copied_poly<T:Owner>(p:&Polynomial,m:&mut Meter<T>)->Result<Polynomial,Failure>{m.pay(1,0)?;let mut q=m.alloc(p.len())?;for t in p{q.push(m.mono(t)?)}Ok(q)}
fn empty_node<T:Owner>(parent:Option<usize>,eqs:usize,depth:usize,m:&mut Meter<T>)->Result<Node,Failure>{Ok(Node{parent,origins:m.alloc(eqs)?,equations:m.alloc(eqs)?,decisions:m.alloc(depth)?,status:NodeStatus::Building,native_disposition:None,native:None,candidate:Vec::new(),candidate_valid:None,children:[None,None],refutation_row:None})}
fn eval<T:Owner>(g:&[Polynomial],point:&[bool],m:&mut Meter<T>)->Result<bool,Failure>{let mut all=true;for p in g{m.pay(1,0)?;let mut sum=false;for t in p{m.scan(t.len())?;sum^=t.iter().all(|v|point[*v as usize]);}all&=!sum;}Ok(all)}
/// Stops at the first exactly evaluated Boolean witness. A witness is not a signed
/// ECDLP relation. UnsatDerived requires every visited branch leaf to contain an
/// actual retained constant-one row; source identity checker is a separate gate.
pub fn solve(n:u32,g:&[Polynomial],limits:SearchLimits,deadline:Instant,owner:&mut impl Owner)->SearchAttempt{solve_inner(n,g,limits,deadline,owner,None)}
pub fn solve_with_checkpoint<T:Owner>(n:u32,g:&[Polynomial],limits:SearchLimits,deadline:Instant,owner:&mut T,control:&mut super::CheckpointControl,hook:&mut impl FnMut(&SearchAttempt,usize,&Attempt,Charge,&mut T)->Result<(),()>)->SearchAttempt{solve_inner(n,g,limits,deadline,owner,Some((control,hook)))}
fn solve_inner<T:Owner>(n:u32,g:&[Polynomial],limits:SearchLimits,deadline:Instant,owner:&mut T,mut checkpoint:Option<(&mut super::CheckpointControl,&mut dyn FnMut(&SearchAttempt,usize,&Attempt,Charge,&mut T)->Result<(),()>)>)->SearchAttempt{
 let mut a=SearchAttempt{result:Ok(()),verdict:Verdict::Unknown,nodes:Vec::new(),queue:Vec::new(),active:None,witness_node:None,charge:Charge::default()};
 let mut m=Meter{owner,limits:limits.native,deadline,charge:Charge::default()};
 a.result=(||{
  if checkpoint.as_ref().is_some_and(|(c,_)|c.stop().is_some()){return Err(Failure::Owner)}
  m.pay(1,0)?;if n>limits.native.variables||g.len()>limits.native.generators||limits.nodes==0{return Err(Failure::Cap)}
  for p in g{m.pay(1,0)?;m.validate(p,n)?}
  a.nodes=m.alloc(limits.nodes)?;a.queue=m.alloc(limits.nodes)?;
  a.nodes.push(empty_node(None,g.len(),0,&mut m)?);a.active=Some(0);
  for(i,p)in g.iter().enumerate(){let q=copied_poly(p,&mut m)?;m.pay(1,0)?;a.nodes[0].equations.push(q);a.nodes[0].origins.push(Origin::Original(i));}
  a.nodes[0].status=NodeStatus::Ready;a.queue.push(0);
  loop{
   m.pay(1,0)?;let Some(&idx)=a.queue.last()else{a.verdict=Verdict::UnsatDerived;a.active=None;return Ok(())};let queue_slot=a.queue.len()-1;a.active=Some(idx);
   a.nodes[idx].status=NodeStatus::Running;
   a.charge=m.charge;
   let mut bridge=NativeOwner{meter:&mut m,failure:None};
   let native=if let Some((control,hook))=checkpoint.as_mut(){
    super::run_with_checkpoint(n,&a.nodes[idx].equations,limits.native,deadline,&mut bridge,control,&mut |native,owner|{
     owner.meter.pay(0,0).map_err(|_|())?;
     let result=hook(&a,idx,native,owner.meter.charge,&mut *owner.meter.owner);
     if result.is_err(){return result}owner.meter.pay(0,0).map_err(|_|())
    })
   }else{super::run(n,&a.nodes[idx].equations,limits.native,deadline,&mut bridge)};
   let failure=bridge.failure;
   let result=native.result;a.nodes[idx].native=Some(native);
   if let Some(e)=failure{a.nodes[idx].native_disposition=Some(NativeDisposition::Stopped);return Err(e)}
   match result{
    Ok(())=>a.nodes[idx].native_disposition=Some(NativeDisposition::Complete),
    Err(Failure::Cap)=>a.nodes[idx].native_disposition=Some(NativeDisposition::LocalCap),
    Err(e)=>{a.nodes[idx].native_disposition=Some(NativeDisposition::Stopped);return Err(e)}
   }
   // A local closure cap is not a parent admission grant. Recheck the same
   // cumulative owner/deadline before using only committed basis rows.
   m.pay(1,0)?;
   let native=a.nodes[idx].native.as_ref().unwrap();let mut contradiction=None;
   for(i,r)in native.basis.iter().enumerate(){m.pay(1,0)?;if r.polynomial.len()==1&&r.polynomial[0].is_empty(){contradiction=Some(i);break}}
   if let Some(row)=contradiction{m.pay(1,0)?;a.nodes[idx].refutation_row=Some(row);a.nodes[idx].status=NodeStatus::Refuted;a.queue.pop();a.active=None;continue}
   // Exact assignment constraints and implied unit rows seed a candidate. Free
   // variables are tentatively false; this is accepted only by original evaluation.
   a.nodes[idx].candidate=m.alloc(n as usize)?;
   for _ in 0..n{m.pay(1,0)?;a.nodes[idx].candidate.push(false)}
   for k in 0..a.nodes[idx].decisions.len(){m.pay(1,0)?;let(v,value)=a.nodes[idx].decisions[k];a.nodes[idx].candidate[v as usize]=value;}
   let mut forced=m.alloc::<Option<bool>>(n as usize)?;for _ in 0..n{m.pay(1,0)?;forced.push(None)}
   for r in &a.nodes[idx].native.as_ref().unwrap().basis{
    m.scan(r.polynomial.len())?;let p=&r.polynomial;
    let unit=if p.len()==1&&p[0].len()==1{Some((p[0][0],false))}else if p.len()==2&&p[0].is_empty()&&p[1].len()==1{Some((p[1][0],true))}else{None};
    if let Some((v,value))=unit{if forced[v as usize].is_some_and(|old|old!=value){return Err(Failure::Malformed)}forced[v as usize]=Some(value);}
   }
   for(v,value)in forced.into_iter().enumerate(){m.pay(1,0)?;if let Some(value)=value{a.nodes[idx].candidate[v]=value}}
   let valid=eval(g,&a.nodes[idx].candidate,&mut m)?&&eval(&a.nodes[idx].equations,&a.nodes[idx].candidate,&mut m)?;
   a.nodes[idx].candidate_valid=Some(valid);
   if valid{m.pay(1,0)?;a.nodes[idx].status=NodeStatus::Witness;a.witness_node=Some(idx);a.verdict=Verdict::Witness;a.queue.pop();a.active=None;return Ok(())}
   let mut variable=None;for v in 0..n{m.scan(a.nodes[idx].decisions.len())?;if !a.nodes[idx].decisions.iter().any(|(x,_)|*x==v){variable=Some(v);break}}
   let Some(variable)=variable else{return Err(if a.nodes[idx].native_disposition==Some(NativeDisposition::LocalCap){Failure::Cap}else{Failure::Malformed})};
   if a.nodes.len().checked_add(2).ok_or(Failure::Overflow)?>limits.nodes{return Err(Failure::Cap)}
   let eqs=a.nodes[idx].equations.len().checked_add(1).ok_or(Failure::Overflow)?;if eqs>limits.native.generators{return Err(Failure::Cap)}
   let depth=a.nodes[idx].decisions.len().checked_add(1).ok_or(Failure::Overflow)?;a.nodes[idx].status=NodeStatus::Branching;
   // Push true then false: both are retained; DFS visits false first.
   for value in [true,false]{
    m.pay(1,0)?;let child=a.nodes.len();a.nodes.push(empty_node(Some(idx),eqs,depth,&mut m)?);a.nodes[idx].children[value as usize]=Some(child);
    for k in 0..a.nodes[idx].equations.len(){let q=copied_poly(&a.nodes[idx].equations[k],&mut m)?;let origin=a.nodes[idx].origins[k];m.pay(1,0)?;a.nodes[child].equations.push(q);a.nodes[child].origins.push(origin);}
    for k in 0..a.nodes[idx].decisions.len(){m.pay(1,0)?;let d=a.nodes[idx].decisions[k];a.nodes[child].decisions.push(d);}
    let mut equation=m.alloc(if value{2}else{1})?;if value{equation.push(m.mono(&[])?)}equation.push(m.mono(&[variable])?);
    m.pay(1,0)?;a.nodes[child].equations.push(equation);a.nodes[child].origins.push(Origin::Branch{parent:idx,variable,value});a.nodes[child].decisions.push((variable,value));a.nodes[child].status=NodeStatus::Ready;a.queue.push(child);
   }
   m.scan(a.queue.len()-queue_slot)?;a.queue.remove(queue_slot);a.nodes[idx].status=NodeStatus::Branched;a.active=None;
  }
 })();a.charge=m.charge;a
}
