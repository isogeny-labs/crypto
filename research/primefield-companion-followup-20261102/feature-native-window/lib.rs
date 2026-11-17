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

/// Prepared public strides borrow the curve, preventing cross-curve reuse.
pub struct Prepared<'a>{curve:&'a Curve,r:u64,base:J,public_base:[u64;2],partition:[u64;4],labels:Vec<u64>,strides:Vec<J>,affine_strides:Vec<[F;2]>}
#[derive(Debug,PartialEq)]
pub struct Slot{pub offset:usize,pub lane:usize,pub label:u64,pub projective:[u64;3],pub affine:[u64;2],pub inverse_z:u64}
#[derive(Debug)]
pub struct Window{pub labels:Vec<u64>,pub initialization:&'static str,pub snapshots:Vec<Vec<Option<[u64;3]>>>,pub slots:Vec<Slot>,pub conversion_batches:usize}
impl Curve{
 fn projective_only(&self,r:u64,q:[u64;2],width:u64,blocks:u64,s:u64,epsilon:u64)->Result<(Vec<u64>,Vec<J>),Error>{
  if r<5||r&1==0{return Err(Error::Partition);}let h=(r-1)/2;
  if width==0||blocks==0||width>h/2||blocks>h/2||s>=width||epsilon>1||2*(width as u128)*(blocks as u128)+2*(s as u128)+epsilon as u128!=h as u128{return Err(Error::Partition);}
  let base=self.point(q)?;let half=r/2+1;let hh=self.mul(half,base);let stride=self.mul(2*width,base);
  let mut points=vec![hh,stride];let mut labels=vec![half,2*width];let mut t=hh;
  for j in 0..width{t=self.add(t,base);points.push(t);labels.push(((half as u128+j as u128+1)%r as u128)as u64);}
  for i in 0..blocks-1{t=self.add(t,stride);points.push(t);labels.push(((half as u128+width as u128+(i as u128+1)*2*width as u128)%r as u128)as u64);}
  if s!=0{points.push(self.mul(s+epsilon,base));labels.push(s+epsilon);}
  Ok((labels,points))
 }
 fn mixed(&self,a:J,b:Option<[F;2]>)->J{
  let embedded=b.map(|[x,y]|[x,y,self.f(1)]);
  let(a,b)=match(a,b){(None,_)=>return embedded,(a,None)=>return a,(Some(a),Some(b))=>(a,b)};
  if Some(a)==embedded{return self.double(a);}let[x,y,z]=a;let[u,v]=b;let zz=z.sq();let uu=u*zz;let ss=v*z*zz;let h=uu-x;let rr=ss-y;
  if h.v==0{return if rr.v==0{self.double(a)}else{None};}
  let hh=h.sq();let hhh=h*hh;let vv=x*hh;let x3=rr.sq()-hhh-self.f(2)*vv;
  Some([x3,rr*(vv-x3)-y*hhh,z*h])
 }
 pub fn prepare(&self,r:u64,p:[u64;2],width:u64,blocks:u64,s:u64,epsilon:u64)->Result<Prepared<'_>,Error>{
  let(labels,strides)=self.projective_only(r,p,width,blocks,s,epsilon)?;
  let(affine,_)=self.batch(&strides)?;
  Ok(Prepared{curve:self,r,base:self.point(p)?,public_base:p,partition:[width,blocks,s,epsilon],labels,strides,affine_strides:affine.iter().map(|[x,y]|[self.f(*x),self.f(*y)]).collect()})
 }
}
impl Prepared<'_>{
 /// Supplied public shifts must be Q,Q+P,...; checked projectively without
 /// introducing an inversion per shift. No scalar label is accepted.
 pub fn window(&self,shifts:&[Option<[u64;2]>])->Result<Window,Error>{
  let first=*shifts.first().ok_or(Error::Empty)?;let c=self.curve;
  let mut current=match first{None=>None,Some(q)=>c.point(q)?};
  let(initialization,mut lanes)=match first{
   None=>("identity",vec![None;self.labels.len()]),
   Some(q) if q==self.public_base=>("prepared_P",self.strides.clone()),
   Some(q)=>{let[w,b,s,e]=self.partition;let(labels,points)=c.projective_only(self.r,q,w,b,s,e)?;if labels!=self.labels{return Err(Error::Partition);}("generated",points)}
  };
  let(mut snapshots,mut flat,mut mapping)=(vec![],vec![],vec![]);
  for(i,point)in shifts.iter().enumerate(){
   if i!=0{current=c.add(current,self.base);for(j,lane)in lanes.iter_mut().enumerate(){*lane=c.mixed(*lane,Some(self.affine_strides[j]));}}
   let expected=match point{None=>None,Some(q)=>c.point(*q)?};
   let valid=match(current,expected){(None,None)=>true,(Some([x,y,z]),Some([u,v,_]))=>x==u*z.sq()&&y==v*z.sq()*z,_=>false};
   if !valid{return Err(Error::Point);}
   snapshots.push(lanes.iter().map(|q|q.map(|a|a.map(|f|f.v))).collect());
   if point.is_none(){if lanes.iter().any(Option::is_some){return Err(Error::Point);}continue;}
   if lanes.iter().any(Option::is_none){return Err(Error::Infinity);}
   for(j,lane)in lanes.iter().enumerate(){flat.push(*lane);mapping.push((i,j));}
  }
  let(affine,inverses,batches)=if flat.is_empty(){(vec![],vec![],0)}else{let(a,z)=c.batch(&flat)?;(a,z,1)};
  let slots=mapping.iter().enumerate().map(|(k,&(offset,lane))|Slot{offset,lane,label:self.labels[lane],projective:flat[k].unwrap().map(|f|f.v),affine:affine[k],inverse_z:inverses[k]}).collect();
  Ok(Window{labels:self.labels.clone(),initialization,snapshots,slots,conversion_batches:batches})
 }
}
#[cfg(test)]mod window_tests{
 use super::*;
 fn inputs()->Vec<Vec<i64>>{include_str!("points.tsv").lines().map(|l|l.split('\t').map(|s|s.parse().unwrap()).collect()).collect()}
 fn shifts(id:i64)->Vec<Option<[u64;2]>>{inputs().into_iter().filter(|r|r[0]==id).enumerate().map(|(i,r)|{assert_eq!(i as i64,r[1]);if r[2]<0{None}else{Some([r[2]as u64,r[3]as u64])}}).collect()}
 #[test]fn four_saved_windows(){
  let c=Curve::new(509,2,10).unwrap();let p=c.prepare(479,[3,68],11,10,9,1).unwrap();
  let rows:Vec<Vec<u64>>=include_str!("slots.tsv").lines().map(|l|l.split('\t').map(|s|s.parse().unwrap()).collect()).collect();
  for id in [0,1,2,478]{let input=shifts(id);let w=p.window(&input).unwrap();let golden:Vec<_>=rows.iter().filter(|r|r[0]==id as u64).collect();assert_eq!(w.slots.len(),golden.len());assert_eq!(w.conversion_batches,1);assert_eq!(w.initialization,if id==0{"identity"}else if id==1{"prepared_P"}else{"generated"});
   for(actual,r)in w.slots.iter().zip(golden){assert_eq!(*actual,Slot{offset:r[1]as usize,lane:r[2]as usize,label:r[3],projective:[r[4],r[5],r[6]],affine:[r[7],r[8]],inverse_z:r[9]});assert_eq!(w.snapshots[actual.offset][actual.lane],Some(actual.projective));assert_eq!(w.labels[actual.lane],actual.label);}
   for(i,q)in input.iter().enumerate(){if q.is_none(){assert!(w.snapshots[i].iter().all(Option::is_none));}}
  }
 }
 #[test]fn empty_identity_and_bad_shifts(){let c=Curve::new(509,2,10).unwrap();let p=c.prepare(479,[3,68],11,10,9,1).unwrap();let w=p.window(&[None]).unwrap();assert!(w.slots.is_empty());assert_eq!(w.conversion_batches,0);assert!(w.snapshots[0].iter().all(Option::is_none));assert!(matches!(p.window(&[]),Err(Error::Empty)));assert!(matches!(p.window(&[Some([3,68]),Some([3,68])]),Err(Error::Point)));assert!(matches!(p.window(&[Some([3,69])]),Err(Error::Point)));}
 #[test]fn prepared_reuse_and_individual_grid(){let c=Curve::new(509,2,10).unwrap();let p=c.prepare(479,[3,68],11,10,9,1).unwrap();let a=p.window(&[Some([3,68])]).unwrap();let b=p.window(&[Some([3,68])]).unwrap();assert_eq!(a.slots,b.slots);let g=c.grid(479,[3,68],11,10,9,1).unwrap();for(i,slot)in a.slots.iter().enumerate(){assert_eq!(slot.label,g.labels[i]);assert_eq!(slot.projective,g.projective[i]);assert_eq!(slot.affine,g.affine[i]);assert_eq!(slot.inverse_z,g.inverse_z[i]);}}
 #[test]fn mixed_exceptions(){let c=Curve::new(509,2,10).unwrap();let j=c.point([3,68]).unwrap();let aff=Some([c.f(3),c.f(68)]);let neg=Some([c.f(3),c.f(441)]);let z=c.f(2);let scaled=Some([c.f(3)*z.sq(),c.f(68)*z.sq()*z,z]);for(a,b)in[(None,aff),(j,None),(None,None),(j,aff),(j,neg),(scaled,aff)]{assert_eq!(c.mixed(a,b),c.add(a,b.map(|[x,y]|[x,y,c.f(1)])));}let c=Curve::new(5,1,0).unwrap();assert_eq!(c.mixed(c.point([0,0]).unwrap(),Some([c.f(0),c.f(0)])),None);}
}
