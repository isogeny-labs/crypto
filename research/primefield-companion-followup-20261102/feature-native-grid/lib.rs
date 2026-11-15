//! Native prescribed-multiplier source grid. No unknown-scalar search.
use std::ops::{Add,Sub,Mul};
#[derive(Clone,Copy,Debug,PartialEq)]
struct F{v:u64,p:u64}
impl F{
 fn c(v:u64,p:u64)->Self{Self{v:v%p,p}}
 fn sq(self)->Self{self*self}
 fn inv(self)->Option<Self>{let(mut r,mut s,mut x,mut y)=(self.p as i128,self.v as i128,0i128,1i128);while s!=0{let q=r/s;(r,s)=(s,r-q*s);(x,y)=(y,x-q*y);}if r==1{Some(Self::c(x.rem_euclid(self.p as i128)as u64,self.p))}else{None}}
}
impl Add for F{type Output=Self;fn add(self,b:Self)->Self{Self::c(self.v+b.v,self.p)}}
impl Sub for F{type Output=Self;fn sub(self,b:Self)->Self{Self::c(self.v+self.p-b.v,self.p)}}
impl Mul for F{type Output=Self;fn mul(self,b:Self)->Self{Self{v:((self.v as u128*b.v as u128)%self.p as u128)as u64,p:self.p}}}
type J=Option<[F;3]>;
#[derive(Debug,PartialEq)]
pub enum Error{Domain,Point,Singular,Partition,Empty,Infinity,Nonunit}
pub struct Curve{p:u64,a:F,b:F}
#[derive(Debug)]
pub struct Grid{pub labels:Vec<u64>,pub projective:Vec<[u64;3]>,pub affine:Vec<[u64;2]>,pub inverse_z:Vec<u64>}
impl Curve{
 pub fn new(p:u64,a:u64,b:u64)->Result<Self,Error>{
  if p<5||p>u64::MAX/2{return Err(Error::Domain);}if a>=p||b>=p{return Err(Error::Point);}
  let(a,b)=(F::c(a,p),F::c(b,p));if (F::c(4,p)*a.sq()*a+F::c(27,p)*b.sq()).v==0{return Err(Error::Singular);}Ok(Self{p,a,b})
 }
 fn f(&self,n:u64)->F{F::c(n,self.p)}
 fn point(&self,q:[u64;2])->Result<J,Error>{if q.iter().any(|v|*v>=self.p){return Err(Error::Point);}let(x,y)=(self.f(q[0]),self.f(q[1]));if y.sq()!=x.sq()*x+self.a*x+self.b{return Err(Error::Point);}Ok(Some([x,y,self.f(1)]))}
 fn double(&self,t:[F;3])->J{
  let[x,y,z]=t;if y.v==0{return None;}let(xx,yy,zz)=(x.sq(),y.sq(),z.sq());let s=self.f(4)*x*yy;let m=self.f(3)*xx+self.a*zz.sq();let u=m.sq()-self.f(2)*s;
  Some([u,m*(s-u)-self.f(8)*yy.sq(),self.f(2)*y*z])
 }
 fn add(&self,a:J,b:J)->J{
  let(a,b)=match(a,b){(None,b)=>return b,(a,None)=>return a,(Some(a),Some(b))=>(a,b)};
  if a==b{return self.double(a);}let[x1,y1,z1]=a;let[x2,y2,z2]=b;
  let(z11,z22)=(z1.sq(),z2.sq());let(u1,u2)=(x1*z22,x2*z11);let(s1,s2)=(y1*z2*z22,y2*z1*z11);let(h,rr)=(u2-u1,s2-s1);
  if h.v==0{return if rr.v==0{self.double(a)}else{None};}
  let hh=h.sq();let hhh=h*hh;let v=u1*hh;let x3=rr.sq()-hhh-self.f(2)*v;
  Some([x3,rr*(v-x3)-s1*hhh,z1*z2*h])
 }
 fn mul(&self,mut n:u64,base:J)->J{let(mut t,mut a)=(None,base);while n!=0{if n&1!=0{t=self.add(t,a);}n>>=1;if n!=0{a=self.add(a,a);}}t}
 fn batch(&self,points:&[J])->Result<(Vec<[u64;2]>,Vec<u64>),Error>{
  if points.is_empty(){return Err(Error::Empty);}if points.iter().any(|q|q.is_none()||q.unwrap()[2].v==0){return Err(Error::Infinity);}
  let mut prefix=vec![points[0].unwrap()[2]];
  for q in &points[1..]{prefix.push(*prefix.last().unwrap()*q.unwrap()[2]);}
  let mut acc=prefix.last().unwrap().inv().ok_or(Error::Nonunit)?;let mut inverses=vec![self.f(0);points.len()];
  for i in (1..points.len()).rev(){inverses[i]=acc*prefix[i-1];acc=acc*points[i].unwrap()[2];}inverses[0]=acc;
  let affine=points.iter().zip(&inverses).map(|(q,&v)|{let[x,y,_]=q.unwrap();let v2=v.sq();let v3=v2*v;[(x*v2).v,(y*v3).v]}).collect();
  Ok((affine,inverses.iter().map(|v|v.v).collect()))
 }
 /// Prime p and intended subgroup order r are caller-certified.
 /// Require the original partition h=2*width*blocks+2*s+epsilon.
 pub fn grid(&self,r:u64,q:[u64;2],width:u64,blocks:u64,s:u64,epsilon:u64)->Result<Grid,Error>{
  if r<5||r&1==0{return Err(Error::Partition);}let h=(r-1)/2;
  if width==0||blocks==0||width>h/2||blocks>h/2||s>=width||epsilon>1||2*(width as u128)*(blocks as u128)+2*(s as u128)+epsilon as u128!=h as u128{return Err(Error::Partition);}
  let base=self.point(q)?;let half=r/2+1;let hh=self.mul(half,base);let stride=self.mul(2*width,base);
  let mut points=vec![hh,stride];let mut labels=vec![half,2*width];let mut t=hh;
  for j in 0..width{t=self.add(t,base);points.push(t);labels.push(((half as u128+j as u128+1)%r as u128)as u64);}
  for i in 0..blocks-1{t=self.add(t,stride);points.push(t);labels.push(((half as u128+width as u128+(i as u128+1)*2*width as u128)%r as u128)as u64);}
  if s!=0{points.push(self.mul(s+epsilon,base));labels.push(s+epsilon);}
  let(affine,inverse_z)=self.batch(&points)?;
  Ok(Grid{labels,projective:points.into_iter().map(|q|q.unwrap().map(|v|v.v)).collect(),affine,inverse_z})
 }
}
#[cfg(test)]mod tests{
 use super::*;
 fn data()->Vec<Vec<u64>>{include_str!("fixtures.tsv").lines().map(|l|l.split('\t').map(|x|x.parse().unwrap()).collect()).collect()}
 fn inv(p:u64,x:u64)->u64{let(mut n,mut a,mut v)=(p-2,x,1u64);while n!=0{if n&1!=0{v=((v as u128*a as u128)%p as u128)as u64;}n>>=1;a=((a as u128*a as u128)%p as u128)as u64;}v}
 fn affine_add(p:u64,a:u64,u:Option<[u64;2]>,v:Option<[u64;2]>)->Option<[u64;2]>{
  let(u,v)=match(u,v){(None,v)=>return v,(u,None)=>return u,(Some(u),Some(v))=>(u,v)};
  if u[0]==v[0]&&(u[1]+v[1])%p==0{return None;}
  let(num,den)=if u==v{((3*u[0]*u[0]+a)%p,2*u[1]%p)}else{((v[1]+p-u[1])%p,(v[0]+p-u[0])%p)};
  let m=num*inv(p,den)%p;let x=(m*m+2*p-u[0]-v[0])%p;Some([x,(m*((u[0]+p-x)%p)+p-u[1])%p])
 }
 fn affine_mul(p:u64,a:u64,mut n:u64,q:[u64;2])->Option<[u64;2]>{let(mut out,mut base)=(None,Some(q));while n!=0{if n&1!=0{out=affine_add(p,a,out,base);}n>>=1;base=affine_add(p,a,base,base);}out}
 fn normalize(c:&Curve,t:J)->Option<[u64;2]>{t.map(|[x,y,z]|{let v=inv(c.p,z.v);[x.v*v%c.p*v%c.p,y.v*v%c.p*v%c.p*v%c.p]})}
 #[test]fn saved_grids_and_affine_reference(){
  let slots:Vec<Vec<u64>>=include_str!("slots.tsv").lines().map(|l|l.split('\t').map(|v|v.parse().unwrap()).collect()).collect();
  for f in data(){let c=Curve::new(f[0],f[1],f[2]).unwrap();let g=c.grid(f[3],[f[4],f[5]],f[6],f[7],f[8],f[9]).unwrap();let expected:Vec<_>=slots.iter().filter(|s|s[0]==f[3]).collect();assert_eq!(g.labels.len(),expected.len());
   for(i,s)in expected.iter().enumerate(){assert_eq!(s[1],i as u64);assert_eq!(g.labels[i],s[2]);assert_eq!(g.projective[i],[s[3],s[4],s[5]]);assert_eq!(g.affine[i],[s[6],s[7]]);assert_eq!(g.inverse_z[i],s[8]);assert_eq!(Some(g.affine[i]),affine_mul(f[0],f[1],g.labels[i],[f[4],f[5]]));}
  }
 }
 #[test]fn exceptional_and_scaled_coordinates(){
  for f in data(){let c=Curve::new(f[0],f[1],f[2]).unwrap();let p=[f[4],f[5]];let j=c.point(p).unwrap();let q=c.mul(2,j);let neg=c.point([p[0],f[0]-p[1]]).unwrap();let [x,y,_]=j.unwrap();let z=c.f(2);let scaled=Some([x*z.sq(),y*z.sq()*z,z]);
   for(a,b)in[(None,j),(j,None),(None,None),(j,j),(j,neg),(j,q),(scaled,j),(scaled,q)]{assert_eq!(normalize(&c,c.add(a,b)),affine_add(c.p,f[1],normalize(&c,a),normalize(&c,b)));}
   let (af,iv)=c.batch(&[scaled]).unwrap();assert_eq!(af,vec![p]);assert_eq!(iv,vec![inv(c.p,2)]);
  }
  let c=Curve::new(5,1,0).unwrap();let t=c.point([0,0]).unwrap();assert_eq!(c.add(t,t),None);
 }
 #[test]fn invalid_inputs(){let c=Curve::new(509,2,10).unwrap();assert_eq!(c.batch(&[]),Err(Error::Empty));assert_eq!(c.batch(&[None]),Err(Error::Infinity));assert_eq!(c.batch(&[Some([c.f(3),c.f(68),c.f(0)])]),Err(Error::Infinity));assert!(matches!(c.grid(479,[3,68],0,10,9,1),Err(Error::Partition)));assert!(matches!(c.grid(479,[3,69],11,10,9,1),Err(Error::Point)));assert!(matches!(Curve::new(509,0,0),Err(Error::Singular)));assert!(matches!(Curve::new(3,0,0),Err(Error::Domain)));}
}
