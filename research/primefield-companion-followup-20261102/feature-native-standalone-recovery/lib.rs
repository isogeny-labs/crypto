//! Public-point recovery using standalone projected signs and mandatory forward verification.
mod program;
pub use program::{Program,Error as ProgramError};
#[path="../feature-native-standalone-source/lib.rs"]mod character;
#[path="../feature-orbit-local-terms/local.rs"]mod group;
#[derive(Debug,PartialEq)]pub enum Error{Domain,Source,Decode,Verification}
#[derive(Default,Debug,Clone,PartialEq)]pub struct Work {pub sign_queries:usize,pub finite_sign_queries:usize,pub point_advances:usize,pub verification_additions:usize}
#[derive(Debug)]pub struct Recovery {pub value:u64,pub extra_offset:Option<usize>,pub work:Work}
pub struct Source {inner:character::Source,curve:group::Curve,base:group::Point,r:u64,max_window:usize}
fn pair(t:Option<group::Point>)->Option<[u64;2]>{t.map(|t|[t.x,t.y])}
impl Source {
 pub fn new(p:u64,a:u64,b:u64,r:u64,base:[u64;2],max_lanes:usize,max_window:usize)->Result<Self,Error>{
  if max_window==0{return Err(Error::Domain)}let inner=character::Source::new(p,a,b,r,base,max_lanes).map_err(|_|Error::Source)?;
  Ok(Self{inner,curve:group::Curve::new(p,a,b),base:group::Point{x:base[0],y:base[1]},r,max_window})
 }
 fn query(&self,t:Option<group::Point>,work:&mut Work)->Result<i8,Error>{work.sign_queries+=1;work.finite_sign_queries+=usize::from(t.is_some());self.inner.sign(pair(t)).map_err(|_|Error::Source)}
 fn verify(&self,value:u64,target:Option<[u64;2]>,work:&mut Work)->Result<(),Error>{
  if value>=self.r{return Err(Error::Verification)}let(mut n,mut out,mut cur)=(value,None,Some(self.base));
  while n>0 {if n&1==1{out=self.curve.plus(out,cur);work.verification_additions+=1}n>>=1;if n>0{cur=self.curve.plus(cur,cur);work.verification_additions+=1}}
  if pair(out)!=target{return Err(Error::Verification)}Ok(())
 }
 /// Input contains no scalar label or scalar-to-point table. Every returned result passed [value]P=target.
 pub fn recover(&self,p:&Program,target:Option<[u64;2]>)->Result<Recovery,Error>{
  let(r,width,extra)=p.shape();if r!=self.r||width==0||width.checked_add(extra).ok_or(Error::Domain)?>self.max_window{return Err(Error::Domain)}
  let mut work=Work::default();let mut current=target.map(|[x,y]|group::Point{x,y});let mut offset=0;let mut signs=Vec::with_capacity(width);
  // query validates the initial target before any curve addition; subsequent points are its shifts.
  for i in 0..width{if i>0{current=self.curve.plus(current,Some(self.base));work.point_advances+=1;offset+=1}signs.push(self.query(current,&mut work)?)}
  let out=p.evaluate(&signs,|wanted|{
   if wanted<width||wanted>=width+extra{return Err(ProgramError::Domain)}
   while offset<wanted{current=self.curve.plus(current,Some(self.base));work.point_advances+=1;offset+=1}
   self.query(current,&mut work).map_err(|_|ProgramError::Source)
  }).map_err(|_|Error::Decode)?;
  self.verify(out.value,target,&mut work)?;
  Ok(Recovery{value:out.value,extra_offset:out.extra_offset,work})
 }
}
#[cfg(test)]mod recovery_tests {
 use super::*;
 #[test]fn frozen_twenty_and_mandatory_verification(){
  let mut p=Program::construct(479,14,2,479,2500).unwrap();let s=Source::new(509,2,10,479,[3,68],128,16).unwrap();
  let(mut count,mut extras)=(0,0);for line in include_str!("../feature-native-monomial-recovery/recovery.tsv").lines().skip(1){let cols:Vec<i64>=line.split('\t').map(|v|v.parse().unwrap()).collect();let target=if cols[1]<0{None}else{Some([cols[1]as u64,cols[2]as u64])};let out=s.recover(&p,target).unwrap();assert_eq!(out.value,cols[0]as u64);assert_eq!(out.value,cols[3]as u64);assert_eq!(out.extra_offset.map(|x|x as i64).unwrap_or(-1),cols[5]);assert_eq!(out.work.sign_queries,cols[4]as usize);assert_eq!(out.work.point_advances,13+cols[6]as usize);extras+=usize::from(out.extra_offset.is_some());count+=1;
   assert_eq!(s.verify((out.value+1)%479,target,&mut Work::default()),Err(Error::Verification));
  }assert_eq!((count,extras),(20,16));
  p.corrupt_for_test();assert!(matches!(s.recover(&p,None),Err(Error::Verification)));
 }
 #[test]fn rejection_paths(){let p=Program::construct(479,14,2,479,2500).unwrap();let short=Source::new(509,2,10,479,[3,68],128,15).unwrap();assert!(matches!(short.recover(&p,None),Err(Error::Domain)));let s=Source::new(509,2,10,479,[3,68],128,16).unwrap();assert!(matches!(s.recover(&p,Some([0,0])),Err(Error::Source)));assert!(matches!(s.recover(&p,Some([509,68])),Err(Error::Source)));assert_eq!(s.verify(479,None,&mut Work::default()),Err(Error::Verification));}
}
