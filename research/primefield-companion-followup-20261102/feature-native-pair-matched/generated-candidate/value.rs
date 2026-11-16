//! Value-only division polynomials for source paths with a supplied seed.
//! No jets, phase computation, hidden scalar input or derivative storage.
use std::collections::BTreeMap;
use std::ops::{Add,Sub,Mul};
#[derive(Clone,Copy)]
struct F {v:u64,p:u64}
impl F {
 fn c(v:u64,p:u64)->Self{Self{v:v%p,p}}
 fn sq(self)->Self{self*self}
 fn cube(self)->Self{self.sq()*self}
 fn inv(self)->Option<Self>{crate::meter::hit(crate::meter::Component::Value,crate::meter::Op::Inverse);if self.v==1{crate::meter::hit(crate::meter::Component::Value,crate::meter::Op::UnitInverse);}
  let(mut r,mut s,mut x,mut y)=(self.p as i128,self.v as i128,0i128,1i128);
  while s!=0{crate::meter::hit(crate::meter::Component::Value,crate::meter::Op::EuclidStep);let q=r/s;(r,s)=(s,r-q*s);(x,y)=(y,x-q*y);}
  if r==1{Some(Self::c(x.rem_euclid(self.p as i128)as u64,self.p))}else{None}
 }
}
impl Add for F{type Output=Self;fn add(self,b:Self)->Self{crate::meter::hit(crate::meter::Component::Value,crate::meter::Op::AddSub);Self::c(self.v+b.v,self.p)}}
impl Sub for F{type Output=Self;fn sub(self,b:Self)->Self{crate::meter::hit(crate::meter::Component::Value,crate::meter::Op::AddSub);Self::c(self.v+self.p-b.v,self.p)}}
impl Mul for F{type Output=Self;fn mul(self,b:Self)->Self{crate::meter::hit(crate::meter::Component::Value,crate::meter::Op::Product);Self{v:((self.v as u128*b.v as u128)%self.p as u128)as u64,p:self.p}}}
#[derive(Debug,PartialEq)]
pub enum Error{Domain,Point,Singular,ZeroY,Nonunit}
/// Caller certifies prime p. Private state is bound to one public curve/point.
/// There is deliberately no phase or derivative API; use the jet core for those.
pub struct ValueDivision{inverse_two_y:F,cache:BTreeMap<u64,F>}
impl ValueDivision{
 pub fn new(p:u64,a:u64,b:u64,x:u64,y:u64)->Result<Self,Error>{
  if p<5||p>u64::MAX/2{return Err(Error::Domain);}
  if [a,b,x,y].iter().any(|v|*v>=p){return Err(Error::Point);}
  let c=|v|F::c(v,p);let(a,b,x,y)=(c(a),c(b),c(x),c(y));
  if (c(4)*a.cube()+c(27)*b.sq()).v==0{return Err(Error::Singular);}
  if (y.sq()-x.cube()-a*x-b).v!=0{return Err(Error::Point);}
  if y.v==0{return Err(Error::ZeroY);}
  let two_y=c(2)*y;
  let x2=x.sq();let x3=x2*x;let x4=x2.sq();let x6=x3.sq();let a2=a.sq();
  let w3=c(3)*x4+c(6)*a*x2+c(12)*b*x-a2;
  let w4=c(4)*y*(x6+c(5)*a*x4+c(20)*b*x3-c(5)*a2*x2-c(4)*a*b*x-c(8)*b.sq()-a2*a);
  Ok(Self{inverse_two_y:two_y.inv().ok_or(Error::Nonunit)?,cache:BTreeMap::from([(0,c(0)),(1,c(1)),(2,two_y),(3,w3),(4,w4)])})
 }
 fn field_value(&mut self,n:u64)->F{
  if let Some(v)=self.cache.get(&n){return *v;}
  let t=n/2;
  let v=if n&1!=0{
   let(a,b,c,d)=(self.field_value(t+2),self.field_value(t),self.field_value(t-1),self.field_value(t+1));
   a*b.cube()-c*d.cube() // Six field multiplications after dependencies.
  }else{
   let(a,b,c,d,e)=(self.field_value(t),self.field_value(t+2),self.field_value(t-1),self.field_value(t-2),self.field_value(t+1));
   (a*self.inverse_two_y)*(b*c.sq()-d*e.sq()) // Six after dependencies.
  };
  self.cache.insert(n,v);v
 }
 pub fn value(&mut self,n:u64)->u64{self.field_value(n).v}
 pub fn retained_indices(&self)->usize{self.cache.len()}
}
#[cfg(test)]
#[path="jets.rs"]
mod preserved_jet;
#[cfg(test)]
mod value_tests{
 use super::*;
 fn rows()->Vec<Vec<String>>{include_str!("../../feature-native-division/fixtures.tsv").lines().map(|l|l.split('\t').map(str::to_string).collect()).collect()}
 #[test]fn original_values_and_repeated_requests(){
  for row in rows(){let v:Vec<u64>=row[..6].iter().map(|s|s.parse().unwrap()).collect();
   let mut d=ValueDivision::new(v[0],v[1],v[2],v[4],v[5]).unwrap();
   for item in row[7].split(','){let(n,w)=item.split_once(':').unwrap();let n=n.parse().unwrap();let w:u64=w.parse().unwrap();
    assert_eq!(d.value(n),w);let before=d.retained_indices();assert_eq!(d.value(n),w);assert_eq!(before,d.retained_indices());
   }
  }
 }
 #[test]fn jet_agreement_and_antipodal_parity(){
  for row in rows(){let v:Vec<u64>=row[..6].iter().map(|s|s.parse().unwrap()).collect();
   let(p,a,b,r,x,y)=(v[0],v[1],v[2],v[3],v[4],v[5]);
   let mut pos=ValueDivision::new(p,a,b,x,y).unwrap();let mut neg=ValueDivision::new(p,a,b,x,p-y).unwrap();
   let mut jp=preserved_jet::Division::new(p,a,b,x,y).unwrap();let mut jn=preserved_jet::Division::new(p,a,b,x,p-y).unwrap();
   for n in (0..64).chain([r,(r+1)/2,u64::MAX]){
    let w=pos.value(n);let minus=neg.value(n);
    assert_eq!(w,jp.value(n));assert_eq!(minus,jn.value(n));
    assert_eq!(minus,if n&1==0{(p-w)%p}else{w});
   }
  }
 }
 #[test]fn invalid_inputs_and_torsion(){
  assert!(matches!(ValueDivision::new(3,0,0,0,0),Err(Error::Domain)));
  assert!(matches!(ValueDivision::new(u64::MAX,0,0,0,0),Err(Error::Domain)));
  assert!(matches!(ValueDivision::new(509,0,0,0,0),Err(Error::Singular)));
  assert!(matches!(ValueDivision::new(509,2,10,3,69),Err(Error::Point)));
  assert!(matches!(ValueDivision::new(509,2,10,512,68),Err(Error::Point)));
  assert!(matches!(ValueDivision::new(5,1,0,0,0),Err(Error::ZeroY)));
  let mut d=ValueDivision::new(509,2,10,3,68).unwrap();
  assert_eq!(d.value(0),0);assert_eq!(d.value(1),1);assert_eq!(d.value(479),0);
 }
}
