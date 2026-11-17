//! Parity-normalized division polynomials: inversion-free value-only cache.
//! No jets, phase computation, hidden scalar input or derivative storage.
include!("schedule.rs");
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
impl Schedule{
 pub fn evaluate(&self,p:u64,a:u64,b:u64,x:u64,y:u64,mode:Mode)->Result<Vec<u64>,Error>{
  if p<5||p>u64::MAX/2{return Err(Error::Domain);}
  if [a,b,x,y].iter().any(|v|*v>=p){return Err(Error::Point);}
  let c=|v|F::c(v,p);let(a,b,x,y)=(c(a),c(b),c(x),c(y));
  if (c(4)*a.cube()+c(27)*b.sq()).v==0{return Err(Error::Singular);}
  if (y.sq()-x.cube()-a*x-b).v!=0{return Err(Error::Point);}
  if y.v==0{return Err(Error::ZeroY);}
  let two_y=c(2)*y;
  let x2=x.sq();let x3=x2*x;let x4=x2.sq();let x6=x3.sq();let a2=a.sq();
  let w3=c(3)*x4+c(6)*a*x2+c(12)*b*x-a2;
  let body=x6+c(5)*a*x4+c(20)*b*x3-c(5)*a2*x2-c(4)*a*b*x-c(8)*b.sq()-a2*a;
  let (w2,w4,z4,inv)=match mode{
   Mode::Normalized=>(c(1),c(2)*body,two_y.sq().sq(),c(1)),
   Mode::Full=>{let(mut r,mut s,mut u,mut v)=(p as i128,two_y.v as i128,0i128,1i128);while s!=0{let q=r/s;(r,s)=(s,r-q*s);(u,v)=(v,u-q*v);}if r!=1{return Err(Error::Nonunit)}(two_y,c(4)*y*body,c(1),c(u.rem_euclid(p as i128)as u64))}
  };
  let mut values=Vec::with_capacity(self.stats().nodes);values.extend([c(0),c(1),w2,w3,w4]);
  let mut squares=vec![c(0);self.stats().nodes];let mut cubes=vec![c(0);self.stats().nodes];
  let prepare=|i:usize,v:F,squares:&mut Vec<F>,cubes:&mut Vec<F>|{let flags=self.power_flags[i];if flags&1!=0{squares[i]=v.sq()}if flags&2!=0{cubes[i]=squares[i]*v}};
  for i in 0..5{prepare(i,values[i],&mut squares,&mut cubes)}
  for step in &self.steps{let z=match *step{
   Step::Odd{a,b,c,d,half_even}=>{let left=values[a]*cubes[b];let right=values[c]*cubes[d];if mode==Mode::Full{left-right}else if half_even{z4*left-right}else{left-z4*right}},
   Step::Even{a,b,c,d,e}=>{let term=values[b]*squares[c]-values[d]*squares[e];if mode==Mode::Full{(values[a]*inv)*term}else{values[a]*term}}
  };prepare(values.len(),z,&mut squares,&mut cubes);values.push(z)}
  Ok(self.outputs.iter().map(|&(i,even)|if mode==Mode::Normalized&&even{(values[i]*two_y).v}else{values[i].v}).collect())
 }
}
