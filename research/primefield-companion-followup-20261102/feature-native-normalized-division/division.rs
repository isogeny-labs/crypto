//! Parity-normalized division polynomials: inversion-free value-only cache.
//! No jets, phase computation, hidden scalar input or derivative storage.
use std::collections::BTreeMap;
use std::ops::{Add,Sub,Mul};
#[derive(Clone,Copy)]
struct F {v:u64,p:u64}
impl F {
 fn c(v:u64,p:u64)->Self{Self{v:v%p,p}}
 fn sq(self)->Self{self*self}
 fn cube(self)->Self{self.sq()*self}

}
impl Add for F{type Output=Self;fn add(self,b:Self)->Self{Self::c(self.v+b.v,self.p)}}
impl Sub for F{type Output=Self;fn sub(self,b:Self)->Self{Self::c(self.v+self.p-b.v,self.p)}}
impl Mul for F{type Output=Self;fn mul(self,b:Self)->Self{Self{v:((self.v as u128*b.v as u128)%self.p as u128)as u64,p:self.p}}}
#[derive(Debug,PartialEq)]
pub enum Error{Domain,Point,Singular,ZeroY,Nonunit}
/// Caller certifies prime p. Private state is bound to one public curve/point.
/// There is deliberately no phase or derivative API; use the jet core for those.
pub struct ValueDivision{two_y:F,two_y_fourth:F,cache:BTreeMap<u64,F>}
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
  let w4=c(2)*(x6+c(5)*a*x4+c(20)*b*x3-c(5)*a2*x2-c(4)*a*b*x-c(8)*b.sq()-a2*a);
  Ok(Self{two_y,two_y_fourth:two_y.sq().sq(),cache:BTreeMap::from([(0,c(0)),(1,c(1)),(2,c(1)),(3,w3),(4,w4)])})
 }
 fn field_value(&mut self,n:u64)->F{
  if let Some(v)=self.cache.get(&n){return *v;}
  let t=n/2;
  let v=if n&1!=0{
   let(a,b,c,d)=(self.field_value(t+2),self.field_value(t),self.field_value(t-1),self.field_value(t+1));
   let left=a*b.cube();let right=c*d.cube();
   if t&1==0{self.two_y_fourth*left-right}else{left-self.two_y_fourth*right} // Seven multiplications.
  }else{
   let(a,b,c,d,e)=(self.field_value(t),self.field_value(t+2),self.field_value(t-1),self.field_value(t-2),self.field_value(t+1));
   a*(b*c.sq()-d*e.sq()) // Five multiplications; no division.
  };
  self.cache.insert(n,v);v
 }
 pub fn value(&mut self,n:u64)->u64{let v=self.field_value(n);if n&1==0{(v*self.two_y).v}else{v.v}}
 pub fn retained_indices(&self)->usize{self.cache.len()}
}
