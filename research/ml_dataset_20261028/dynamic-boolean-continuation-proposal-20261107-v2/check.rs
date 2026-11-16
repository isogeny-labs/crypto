//! Independent structural and arithmetic checking of successful search results.
//! Supplied originals are not authenticated curve equations.
use super::{scheduler::search::{SearchAttempt,NodeStatus,Origin,Verdict},sparse::{Owner,Charge,Polynomial},bridge,bqi};
use std::time::Instant;
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Failure{Cap,Owner,Deadline,Overflow,Structure,Model,Arithmetic}
#[derive(Clone,Copy)]pub struct Limits{pub nodes:usize,pub work:u64,pub bytes:u64,pub certificate:bridge::Limits,pub checker:bqi::Limits}
pub struct Audit{pub result:Result<(),Failure>,pub charge:Charge,pub checked_nodes:usize,pub certificates:Vec<bridge::Certificate>,pub source_authenticated:bool}
struct Meter<'a,T:Owner>{owner:&'a mut T,limits:Limits,deadline:Instant,charge:Charge}
impl<T:Owner> Meter<'_,T>{
 fn pay(&mut self,w:u64,b:u64)->Result<(),Failure>{if Instant::now()>=self.deadline{return Err(Failure::Deadline)}let q=Charge{work:self.charge.work.checked_add(w).ok_or(Failure::Overflow)?,bytes:self.charge.bytes.checked_add(b).ok_or(Failure::Overflow)?};if q.work>self.limits.work||q.bytes>self.limits.bytes{return Err(Failure::Cap)}self.owner.reserve(Charge{work:w,bytes:b}).map_err(|_|Failure::Owner)?;self.charge=q;self.owner.check().map_err(|_|Failure::Owner)?;if Instant::now()>=self.deadline{return Err(Failure::Deadline)}Ok(())}
 fn scan(&mut self,n:usize)->Result<(),Failure>{self.pay(u64::try_from(n).map_err(|_|Failure::Overflow)?.checked_add(1).ok_or(Failure::Overflow)?,0)}
 fn poly_eq(&mut self,a:&Polynomial,b:&Polynomial)->Result<(),Failure>{for p in [a,b]{self.pay(1,0)?;for t in p{self.scan(t.len())?}}if a!=b{return Err(Failure::Structure)}Ok(())}
 fn model(&mut self,g:&[Polynomial],x:&[bool])->Result<(),Failure>{for p in g{self.pay(1,0)?;let mut sum=false;for t in p{self.scan(t.len())?;let mut term=true;for v in t{term&=*x.get(*v as usize).ok_or(Failure::Model)?;}sum^=term;}if sum{return Err(Failure::Model)}}Ok(())}
}
struct Nested<'a,'b,T:Owner>{meter:&'a mut Meter<'b,T>,failure:Option<Failure>}
impl<T:Owner> Owner for Nested<'_,'_,T>{fn reserve(&mut self,q:Charge)->Result<(),()>{self.meter.pay(q.work,q.bytes).map_err(|e|{self.failure=Some(e);})}fn check(&mut self)->Result<(),()>{self.meter.pay(0,0).map_err(|e|{self.failure=Some(e);})}}
/// Verify only successful Witness or UnsatDerived outputs. Unknown remains
/// diagnostic. Every refuted leaf must supply an independently checked 1 row.
/// No native trajectory or original CheckedMap authentication is implied.
pub fn verify(n:u32,g:&[Polynomial],a:&SearchAttempt,limits:Limits,deadline:Instant,owner:&mut impl Owner)->Audit{
 let mut out=Audit{result:Ok(()),charge:Charge::default(),checked_nodes:0,certificates:Vec::new(),source_authenticated:false};let mut m=Meter{owner,limits,deadline,charge:Charge::default()};
 out.result=(||{
  m.pay(1,0)?;if a.nodes.is_empty()||a.nodes.len()>limits.nodes||n>limits.certificate.variables{return Err(Failure::Cap)}
  if a.result.is_err()||a.verdict==Verdict::Unknown||a.active.is_some(){return Err(Failure::Structure)}
  let bytes=a.nodes.len().checked_mul(std::mem::size_of::<bridge::Certificate>()).and_then(|x|x.checked_add(64)).ok_or(Failure::Overflow)?;m.pay(1,u64::try_from(bytes).map_err(|_|Failure::Overflow)?)?;out.certificates.try_reserve_exact(a.nodes.len()).map_err(|_|Failure::Cap)?;
  for(i,id)in a.queue.iter().enumerate(){m.scan(i)?;if *id>=a.nodes.len()||a.queue[..i].contains(id)||a.nodes[*id].status!=NodeStatus::Ready{return Err(Failure::Structure)}}
  let mut witnesses=0;
  for(i,node)in a.nodes.iter().enumerate(){m.pay(1,0)?;
   if node.equations.len()!=node.origins.len(){return Err(Failure::Structure)}
   if i==0{
    if node.parent.is_some()||!node.decisions.is_empty()||node.equations.len()!=g.len(){return Err(Failure::Structure)}
    for(j,p)in node.equations.iter().enumerate(){m.poly_eq(p,&g[j])?;if node.origins[j]!=Origin::Original(j){return Err(Failure::Structure)}}
   }else{
    let parent=node.parent.ok_or(Failure::Structure)?;if parent>=i{return Err(Failure::Structure)}let p=&a.nodes[parent];
    if p.status!=NodeStatus::Branched||node.decisions.len()!=p.decisions.len().checked_add(1).ok_or(Failure::Overflow)?||node.equations.len()!=p.equations.len().checked_add(1).ok_or(Failure::Overflow)?{return Err(Failure::Structure)}
    m.scan(p.decisions.len())?;if node.decisions[..p.decisions.len()]!=p.decisions{return Err(Failure::Structure)}
    let(v,value)=*node.decisions.last().unwrap();m.scan(p.decisions.len())?;if v>=n||p.decisions.iter().any(|(w,_)|*w==v)||p.children[value as usize]!=Some(i){return Err(Failure::Structure)}
    for(j,eq)in p.equations.iter().enumerate(){m.poly_eq(eq,&node.equations[j])?;if node.origins[j]!=p.origins[j]{return Err(Failure::Structure)}}
    let eq=node.equations.last().unwrap();m.scan(eq.len())?;
    let shape=if value{eq.len()==2&&eq[0].is_empty()&&eq[1].as_slice()==[v]}else{eq.len()==1&&eq[0].as_slice()==[v]};
    if !shape||*node.origins.last().unwrap()!=(Origin::Branch{parent,variable:v,value}){return Err(Failure::Structure)}
   }
   m.scan(a.queue.len())?;let queued=a.queue.contains(&i);
   match node.status{
    NodeStatus::Branched=>{if queued||node.children[0].is_none()||node.children[1].is_none()||node.children[0]==node.children[1]{return Err(Failure::Structure)}let left=node.children[0].unwrap();let right=node.children[1].unwrap();if left>=a.nodes.len()||right>=a.nodes.len()||a.nodes[left].decisions.last().map(|x|x.0)!=a.nodes[right].decisions.last().map(|x|x.0){return Err(Failure::Structure)}for(value,child)in node.children.iter().enumerate(){let j=child.unwrap();if j<=i||j>=a.nodes.len()||a.nodes[j].parent!=Some(i)||a.nodes[j].decisions.last().map(|x|x.1)!=Some(value==1){return Err(Failure::Structure)}}},
    NodeStatus::Ready=>{if !queued||a.verdict!=Verdict::Witness||node.children!=[None,None]||node.native.is_some(){return Err(Failure::Structure)}},
    NodeStatus::Witness=>{if queued||node.children!=[None,None]||a.witness_node!=Some(i)||a.verdict!=Verdict::Witness||node.candidate.len()!=n as usize{return Err(Failure::Structure)}m.model(g,&node.candidate)?;m.model(&node.equations,&node.candidate)?;witnesses+=1;},
    NodeStatus::Refuted=>{
     if queued||node.children!=[None,None]{return Err(Failure::Structure)}let native=node.native.as_ref().ok_or(Failure::Structure)?;let row=native.basis.get(node.refutation_row.ok_or(Failure::Structure)?).ok_or(Failure::Structure)?;
     if row.polynomial.len()!=1||!row.polynomial[0].is_empty(){return Err(Failure::Arithmetic)}
     let mut nested=Nested{meter:&mut m,failure:None};let cert=bridge::certify(n,&node.equations,row,limits.certificate,limits.checker,deadline,&mut nested);let failure=nested.failure;let valid=cert.result.is_ok();out.certificates.push(cert);if let Some(e)=failure{return Err(e)}if !valid{return Err(Failure::Arithmetic)}
    },_=>return Err(Failure::Structure),
   }
   out.checked_nodes+=1;
  }
  match a.verdict{Verdict::Witness if witnesses==1=>{},Verdict::UnsatDerived if witnesses==0&&a.witness_node.is_none()&&a.queue.is_empty()=>{},_=>return Err(Failure::Structure)}
  m.pay(1,0)
 })();out.charge=m.charge;out
}
