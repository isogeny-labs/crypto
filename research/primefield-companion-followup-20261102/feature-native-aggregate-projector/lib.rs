//! Aggregate transcript projector over a split prime-field function algebra.
#[path="../feature-native-half-gcd/polynomial.rs"]mod polynomial;
#[derive(Debug,PartialEq)]pub enum Error{Domain,Limit,Sign,NoConstraints,Source,Unresolved,Verification}
fn mul(a:u64,b:u64,r:u64)->u64{((a as u128*b as u128)%r as u128)as u64}
fn pow(mut a:u64,mut n:u64,r:u64)->u64{let mut v=1;while n>0{if n&1!=0{v=mul(v,a,r)}a=mul(a,a,r);n>>=1}v}
/// Caller certifies r prime. max_coefficients caps the persistent degree-r representation.
pub struct Projector{r:u64,n:usize,inverses:Vec<u64>,poly:polynomial::Poly,coefficients:Vec<u64>,constraints:usize}
#[derive(Debug,PartialEq)]pub struct Moments{pub count:u64,pub first:u64,pub singleton:Option<u64>}
impl Projector{
 pub fn new(r:u64,max_coefficients:usize)->Result<Self,Error>{
  if r<5||r>u64::MAX/2||r%2==0{return Err(Error::Domain)}let n=usize::try_from(r).map_err(|_|Error::Limit)?;if n>max_coefficients{return Err(Error::Limit)}
  let h=n/2;let mut inverses=vec![0;h+1];inverses[1]=1;for j in 2..=h{inverses[j]=r-mul(r/j as u64,inverses[(r%j as u64)as usize],r)}
  Ok(Self{r,n,inverses,poly:polynomial::Poly::new(r).map_err(|_|Error::Domain)?,coefficients:vec![1],constraints:0})
 }
 fn reduce(&self,v:Vec<u64>)->Vec<u64>{let mut out=vec![0;self.n];for(i,c)in v.into_iter().enumerate(){let j=if i==0{0}else{1+(i-1)%(self.n-1)};out[j]=(out[j]+c)%self.r}while out.len()>1&&out.last()==Some(&0){out.pop();}out}
 fn character(&self,a:u64)->Vec<u64>{let h=self.n/2;let mut out=vec![0;h+1];out[h]=1;for j in(1..=h).rev(){out[j-1]=mul(mul(mul(out[j],j as u64,self.r),a,self.r),self.inverses[h-j+1],self.r)}out}
 /// Multiply by the exact sign-class indicator, including its zero endpoint.
 pub fn observe(&mut self,offset:u64,sign:i8)->Result<Moments,Error>{
  if offset>=self.r{return Err(Error::Domain)}if !(-1..=1).contains(&sign){return Err(Error::Sign)}
  let c=self.character(offset);let square=self.poly.mul(&c,&c);
  let indicator=if sign==0{self.poly.sub(&[1],&square)}else{let t=if sign==1{self.poly.add(&square,&c)}else{self.poly.sub(&square,&c)};t.into_iter().map(|v|mul(v,(self.r+1)/2,self.r)).collect()};
  let product=self.poly.mul(&self.coefficients,&indicator);self.coefficients=self.reduce(product);self.constraints+=1;self.moments()
 }
 pub fn moments(&self)->Result<Moments,Error>{
  if self.constraints==0{return Err(Error::NoConstraints)}
  let count=(self.r-self.coefficients.get(self.n-1).copied().unwrap_or(0))%self.r;
  let first=(self.r-self.coefficients.get(self.n-2).copied().unwrap_or(0))%self.r;
  Ok(Moments{count,first,singleton:if count==1{Some(first)}else{None}})
 }
}
#[path="../feature-native-standalone-source/lib.rs"]mod source;
#[path="../feature-orbit-local-terms/local.rs"]mod group;
pub struct RecoverySource{source:source::Source,curve:group::Curve,base:group::Point,r:u64,max_coefficients:usize,max_queries:usize}
#[derive(Debug)]pub struct Recovery{pub value:u64,pub transcript:Vec<i8>,pub counts:Vec<u64>}
impl RecoverySource{
 pub fn new(p:u64,a:u64,b:u64,r:u64,base:[u64;2],max_lanes:usize,max_coefficients:usize,max_queries:usize)->Result<Self,Error>{
  if max_queries==0||max_queries as u128>r as u128||r as u128>max_coefficients as u128{return Err(Error::Limit)}
  let source=source::Source::new(p,a,b,r,base,max_lanes).map_err(|_|Error::Source)?;
  Ok(Self{source,curve:group::Curve::new(p,a,b),base:group::Point{x:base[0],y:base[1]},r,max_coefficients,max_queries})
 }
 fn pair(t:Option<group::Point>)->Option<[u64;2]>{t.map(|p|[p.x,p.y])}
 fn verify(&self,k:u64,target:Option<[u64;2]>)->bool{if k>=self.r{return false}let(mut n,mut p,mut out)=(k,Some(self.base),None);while n>0{if n&1!=0{out=self.curve.plus(out,p)}n>>=1;if n>0{p=self.curve.plus(p,p)}}Self::pair(out)==target}
 pub fn recover(&self,target:Option<[u64;2]>)->Result<Recovery,Error>{
  let mut state=Projector::new(self.r,self.max_coefficients)?;let mut point=target.map(|[x,y]|group::Point{x,y});let(mut transcript,mut counts)=(vec![],vec![]);
  for offset in 0..self.max_queries{
   // Initial point is checked by source before the first curve addition.
   let sign=self.source.sign(Self::pair(point)).map_err(|_|Error::Source)?;let moments=state.observe(offset as u64,sign)?;transcript.push(sign);counts.push(moments.count);
   if let Some(value)=moments.singleton{if !self.verify(value,target){return Err(Error::Verification)}return Ok(Recovery{value,transcript,counts})}
   if moments.count==0{return Err(Error::Verification)}if offset+1<self.max_queries{point=self.curve.plus(point,Some(self.base))}
  }Err(Error::Unresolved)
 }
}
#[cfg(test)]mod aggregate_tests{
 use super::*;
 fn chi(x:u64,r:u64)->i8{if x==0{0}else if pow(x,(r-1)/2,r)==1{1}else{-1}}
 fn eval(v:&[u64],x:u64,r:u64)->u64{v.iter().rev().fold(0,|a,&c|(mul(a,x,r)+c)%r)}
 #[test]fn all_small_transcripts_and_idempotence(){let mut checks=0;for r in[5,7,11,31]{for code in 0..27{let mut state=Projector::new(r,31).unwrap();let mut c=code;let mut transcript=vec![];for offset in[0,1,3]{let sign=(c%3)as i8-1;c/=3;transcript.push((offset,sign));let m=state.observe(offset,sign).unwrap();let candidates:Vec<_>=(0..r).filter(|&x|transcript.iter().all(|&(a,s)|chi((x+a)%r,r)==s)).collect();assert_eq!(m.count,candidates.len()as u64);assert_eq!(m.first,candidates.iter().fold(0,|a,&v|(a+v)%r));for x in 0..r{assert_eq!(eval(&state.coefficients,x,r),u64::from(candidates.contains(&x)))}let square=state.poly.mul(&state.coefficients,&state.coefficients);assert_eq!(state.reduce(square),state.coefficients);checks+=1}}}assert_eq!(checks,324);}
 #[test]fn repeat_contradiction_and_limits(){let mut p=Projector::new(7,7).unwrap();assert_eq!(p.moments(),Err(Error::NoConstraints));let a=p.observe(0,1).unwrap();assert_eq!(p.observe(0,1).unwrap(),a);assert_eq!(p.observe(0,-1).unwrap().count,0);assert!(matches!(Projector::new(479,478),Err(Error::Limit)));assert_eq!(p.observe(7,1),Err(Error::Domain));assert_eq!(p.observe(0,2),Err(Error::Sign));let mut p=Projector::new(7,7).unwrap();assert_eq!(p.observe(3,0).unwrap().singleton,Some(4));}
 #[test]fn public_point_frozen_twenty(){let s=RecoverySource::new(509,2,10,479,[3,68],64,479,16).unwrap();let mut count=0;for line in include_str!("../feature-native-monomial-recovery/recovery.tsv").lines().skip(1){let v:Vec<i64>=line.split('\t').map(|v|v.parse().unwrap()).collect();let target=if v[1]<0{None}else{Some([v[1]as u64,v[2]as u64])};let out=s.recover(target).unwrap();assert_eq!(out.value,v[0]as u64);assert_eq!(*out.counts.last().unwrap(),1);assert!(out.counts.windows(2).all(|p|p[1]<=p[0]));assert!(!s.verify((out.value+1)%479,target));count+=1}assert_eq!(count,20);let short=RecoverySource::new(509,2,10,479,[3,68],64,479,1).unwrap();assert!(matches!(short.recover(Some([3,68])),Err(Error::Unresolved)));assert!(matches!(s.recover(Some([509,68])),Err(Error::Source)));}
}
