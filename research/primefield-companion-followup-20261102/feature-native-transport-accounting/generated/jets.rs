//! Public-point division polynomials and invariant-differential jets.
//! Caller supplies prime p; no discrete logarithm, target label or orbit table.
use std::collections::BTreeMap;
use std::ops::{Add,Sub,Mul,Neg};
#[derive(Clone,Copy,Debug,PartialEq)]
struct F { v:u64, p:u64 }
impl F {
 fn new(v:u64,p:u64)->Self {Self{v:v%p,p}}
 fn inv(self)->Option<Self>{crate::meter::hit(crate::meter::Component::Phase,crate::meter::Op::Inverse); inverse(self.v,self.p).map(|v|Self{v,p:self.p}) }
 fn pow(self,mut n:u64)->Self {let(mut a,mut v)=(self,Self::new(1,self.p));while n!=0 {if n&1!=0 {v=v*a;}n>>=1;if n!=0 {a=a*a;}}v}
}
impl Add for F {type Output=Self;fn add(self,b:Self)->Self{Self::new(self.v+b.v,self.p)}}
impl Sub for F {type Output=Self;fn sub(self,b:Self)->Self{Self::new(self.v+self.p-b.v,self.p)}}
impl Neg for F {type Output=Self;fn neg(self)->Self{Self::new(self.p-self.v,self.p)}}
impl Mul for F {type Output=Self;fn mul(self,b:Self)->Self{crate::meter::hit(crate::meter::Component::Phase,crate::meter::Op::Product);Self{v:((self.v as u128*b.v as u128)%self.p as u128)as u64,p:self.p}}}
fn inverse(a:u64,m:u64)->Option<u64>{
 if m<2{return None;}let(mut r,mut s,mut x,mut y)=(m as i128,a as i128,0i128,1i128);
 while s!=0 {crate::meter::hit(crate::meter::Component::Phase,crate::meter::Op::EuclidStep);let q=r/s;(r,s)=(s,r-q*s);(x,y)=(y,x-q*y);}
 if r==1{Some(x.rem_euclid(m as i128)as u64)}else{None}
}
#[derive(Clone,Copy,Debug)]
struct Jet {v:F,d:F}
impl Jet {
 fn c(v:u64,p:u64)->Self{Self{v:F::new(v,p),d:F::new(0,p)}}
 fn pow(self,mut n:u64)->Self{let(mut a,mut v)=(self,Self::c(1,self.v.p));while n!=0 {if n&1!=0{v=v*a;}n>>=1;if n!=0{a=a*a;}}v}
 fn inv(self)->Option<Self>{let v=self.v.inv()?;Some(Self{v,d:-self.d*v*v})}
}
impl Add for Jet{type Output=Self;fn add(self,b:Self)->Self{Self{v:self.v+b.v,d:self.d+b.d}}}
impl Neg for Jet{type Output=Self;fn neg(self)->Self{Self{v:-self.v,d:-self.d}}}
impl Sub for Jet{type Output=Self;fn sub(self,b:Self)->Self{self+-b}}
impl Mul for Jet{type Output=Self;fn mul(self,b:Self)->Self{Self{v:self.v*b.v,d:self.d*b.v+self.v*b.d}}}
#[derive(Debug,PartialEq)]
pub enum Error{Domain,Point,Singular,ZeroY,Nonunit,NonTorsion,Derivative}
/// Reusable cache for exactly one curve and public affine point.
/// A cached value request also retains its derivative: price this extra work.
pub struct Division {p:u64, inv_two_y:Jet, cache:BTreeMap<u64,Jet>}
impl Division {
 pub fn new(p:u64,a:u64,b:u64,x:u64,y:u64)->Result<Self,Error>{
  if p<5 || p>u64::MAX/2{return Err(Error::Domain);}
  if [a,b,x,y].iter().any(|v|*v>=p){return Err(Error::Point);}
  let c=|v|Jet::c(v,p);let aa=c(a);let bb=c(b);
  if (c(4)*aa.pow(3)+c(27)*bb.pow(2)).v.v==0{return Err(Error::Singular);}
  if (c(y).pow(2)-c(x).pow(3)-aa*c(x)-bb).v.v!=0{return Err(Error::Point);}
  if y==0{return Err(Error::ZeroY);}
  let xx=Jet{v:F::new(x,p),d:F::new(2,p)*F::new(y,p)};
  let yy=Jet{v:F::new(y,p),d:F::new(3,p)*F::new(x,p).pow(2)+F::new(a,p)};
  let two_y=c(2)*yy;
  let w3=c(3)*xx.pow(4)+c(6)*aa*xx.pow(2)+c(12)*bb*xx-aa.pow(2);
  let w4=c(4)*yy*(xx.pow(6)+c(5)*aa*xx.pow(4)+c(20)*bb*xx.pow(3)-c(5)*aa.pow(2)*xx.pow(2)-c(4)*aa*bb*xx-c(8)*bb.pow(2)-aa.pow(3));
  let cache=BTreeMap::from([(0,c(0)),(1,c(1)),(2,two_y),(3,w3),(4,w4)]);
  Ok(Self{p,inv_two_y:two_y.inv().ok_or(Error::Nonunit)?,cache})
 }
 fn jet(&mut self,n:u64)->Jet {
  if let Some(v)=self.cache.get(&n){return *v;}
  let t=n/2;
  let v=if n&1!=0 {self.jet(t+2)*self.jet(t).pow(3)-self.jet(t-1)*self.jet(t+1).pow(3)}
  else {self.jet(t)*self.inv_two_y*(self.jet(t+2)*self.jet(t-1).pow(2)-self.jet(t-2)*self.jet(t+1).pow(2))};
  self.cache.insert(n,v);v
 }
 pub fn value_and_derivative(&mut self,n:u64)->(u64,u64){let z=self.jet(n);(z.v.v,z.d.v)}
 pub fn value(&mut self,n:u64)->u64{self.jet(n).v.v}
 pub fn retained_indices(&self)->usize{self.cache.len()}
 /// Same normalization contract as normalized-progression-v2/kernel.py.
 /// r odd, r != p, gcd(r,p-1)=1, psi_r(Q)=0, Dpsi_r(Q) != 0.
 pub fn phase(&mut self,r:u64)->Result<u64,Error>{
  if r<3 || r&1==0 || r==self.p{return Err(Error::Domain);}
  let modulus=self.p-1;
  let rr=((r as u128*r as u128)%modulus as u128)as u64;
  let exponent={crate::meter::hit(crate::meter::Component::Phase,crate::meter::Op::ScalarInverse);inverse(rr,modulus)}.ok_or(Error::Nonunit)?;
  let(v,d)=self.value_and_derivative(r);
  if v!=0{return Err(Error::NonTorsion);}
  let derivative=F::new(d,self.p).inv().ok_or(Error::Derivative)?;
  Ok((-F::new(r,self.p)*derivative).pow(exponent).v)
 }
}
#[cfg(test)]
mod tests{
 use super::*;
 fn rows()->Vec<Vec<String>>{include_str!("../../feature-native-division/fixtures.tsv").lines().map(|l|l.split('\t').map(str::to_string).collect()).collect()}
 #[test]fn saved_phase_and_division_values(){
  for row in rows(){let v:Vec<u64>=row[..7].iter().map(|s|s.parse().unwrap()).collect();
   let mut d=Division::new(v[0],v[1],v[2],v[4],v[5]).unwrap();assert_eq!(d.phase(v[3]).unwrap(),v[6]);
   let before=d.retained_indices();assert_eq!(d.phase(v[3]).unwrap(),v[6]);assert_eq!(before,d.retained_indices());
   for item in row[7].split(','){let(n,w)=item.split_once(':').unwrap();let n=n.parse().unwrap();let w:u64=w.parse().unwrap();
    assert_eq!(d.value(n),w);let count=d.retained_indices();assert_eq!(d.value(n),w);assert_eq!(count,d.retained_indices());
   }
  }
 }
 #[test]fn addition_identity_value_and_derivative(){
  // Ward identity is independent of the halving recurrence used by the engine.
  for row in rows(){let v:Vec<u64>=row[..6].iter().map(|s|s.parse().unwrap()).collect();
   let mut d=Division::new(v[0],v[1],v[2],v[4],v[5]).unwrap();
   for m in 4..15 {for n in 2..m {
    let lhs=d.jet(m+n)*d.jet(m-n);
    let rhs=d.jet(m+1)*d.jet(m-1)*d.jet(n).pow(2)-d.jet(n+1)*d.jet(n-1)*d.jet(m).pow(2);
    assert_eq!((lhs.v,lhs.d),(rhs.v,rhs.d));
   }}
  }
 }
 #[test]fn tangent_and_rejections(){
  let mut d=Division::new(509,2,10,3,68).unwrap();let(w,dw)=d.value_and_derivative(2);
  assert_eq!(w,136);assert_eq!(dw,58); // D(2y)=2(3x^2+a).
  assert_eq!(d.phase(3),Err(Error::NonTorsion));assert_eq!(d.phase(2),Err(Error::Domain));
  assert_eq!(d.phase(509),Err(Error::Domain));assert_eq!(d.phase(127),Err(Error::Nonunit));
  assert!(matches!(Division::new(509,0,0,0,0),Err(Error::Singular)));
  assert!(matches!(Division::new(5,1,0,0,0),Err(Error::ZeroY)));
  assert!(matches!(Division::new(509,2,10,3,69),Err(Error::Point)));
  assert!(matches!(Division::new(3,0,0,0,0),Err(Error::Domain)));
  assert!(matches!(Division::new(509,2,10,512,68),Err(Error::Point)));
 }
}
