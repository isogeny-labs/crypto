//! Native prescribed-multiplier source grid. No unknown-scalar search.
use std::ops::{Add,Sub,Mul};
#[derive(Clone,Copy,Debug,PartialEq)]
struct F{v:u64,p:u64}
impl F{
 fn c(v:u64,p:u64)->Self{Self{v:v%p,p}}
 fn sq(self)->Self{self*self}
 fn inv(self)->Option<Self>{#[cfg(test)] bump_inverse();let(mut r,mut s,mut x,mut y)=(self.p as i128,self.v as i128,0i128,1i128);while s!=0{let q=r/s;(r,s)=(s,r-q*s);(x,y)=(y,x-q*y);}if r==1{Some(Self::c(x.rem_euclid(self.p as i128)as u64,self.p))}else{None}}
}
impl Add for F{type Output=Self;fn add(self,b:Self)->Self{Self::c(self.v+b.v,self.p)}}
impl Sub for F{type Output=Self;fn sub(self,b:Self)->Self{Self::c(self.v+self.p-b.v,self.p)}}
impl Mul for F{type Output=Self;fn mul(self,b:Self)->Self{#[cfg(test)] bump_product();Self{v:((self.v as u128*b.v as u128)%self.p as u128)as u64,p:self.p}}}
type J=Option<[F;3]>;
#[derive(Debug,PartialEq)]
pub enum Error{Domain,Point,Singular,Partition,Empty,Infinity,Nonunit}
#[derive(Clone)]
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
pub struct Prepared{curve:Curve,r:u64,base:J,public_base:[u64;2],partition:[u64;4],labels:Vec<u64>,strides:Vec<J>,affine_strides:Vec<[F;2]>}
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
 pub fn prepare(&self,r:u64,p:[u64;2],width:u64,blocks:u64,s:u64,epsilon:u64)->Result<Prepared,Error>{
  let(labels,strides)=self.projective_only(r,p,width,blocks,s,epsilon)?;
  let(affine,_)=self.batch(&strides)?;
  Ok(Prepared{curve:self.clone(),r,base:self.point(p)?,public_base:p,partition:[width,blocks,s,epsilon],labels,strides,affine_strides:affine.iter().map(|[x,y]|[self.f(*x),self.f(*y)]).collect()})
 }
}
impl Prepared{
 /// Supplied public shifts must be Q,Q+P,...; checked projectively without
 /// introducing an inversion per shift. No scalar label is accepted.
 pub fn window(&self,shifts:&[Option<[u64;2]>])->Result<Window,Error>{
  let first=*shifts.first().ok_or(Error::Empty)?;let c=&self.curve;
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

#[cfg(test)]thread_local!{static COUNTS:std::cell::Cell<(u64,u64)>=const{std::cell::Cell::new((0,0))};}
#[cfg(test)]fn bump_product(){COUNTS.with(|c|{let(m,i)=c.get();c.set((m+1,i));});}
#[cfg(test)]fn bump_inverse(){COUNTS.with(|c|{let(m,i)=c.get();c.set((m,i+1));});}
#[derive(Debug)]
pub struct JointWindow{pub shifts:Vec<Option<[u64;2]>>,pub geometry:Window,pub converted_points:usize}
impl Prepared{
 /// Generate the public prefix from Q; jointly normalize prefix and grid lanes.
 /// No shifted-point list or scalar label is accepted.
 pub fn joint_window(&self,q:Option<[u64;2]>,length:usize)->Result<JointWindow,Error>{
  if length==0{return Err(Error::Empty);}let c=&self.curve;
  let mut current=match q{None=>None,Some(point)=>c.point(point)?};
  let(initialization,mut lanes)=match q{
   None=>("identity",vec![None;self.labels.len()]),
   Some(point)if point==self.public_base=>("prepared_P",self.strides.clone()),
   Some(point)=>{let[w,b,s,e]=self.partition;let(labels,points)=c.projective_only(self.r,point,w,b,s,e)?;if labels!=self.labels{return Err(Error::Partition);}("generated",points)}
  };
  let(mut snapshots,mut flat,mut shift_indices,mut mapping)=(vec![],vec![],vec![],vec![]);
  for offset in 0..length{
   if offset!=0{current=c.add(current,self.base);for(j,lane)in lanes.iter_mut().enumerate(){*lane=c.mixed(*lane,Some(self.affine_strides[j]));}}
   snapshots.push(lanes.iter().map(|p|p.map(|a|a.map(|f|f.v))).collect());
   if current.is_none(){if lanes.iter().any(Option::is_some){return Err(Error::Point);}shift_indices.push(None);continue;}
   if lanes.iter().any(Option::is_none){return Err(Error::Infinity);}
   shift_indices.push(Some(flat.len()));flat.push(current);
   for(lane,point)in lanes.iter().enumerate(){mapping.push((offset,lane,flat.len()));flat.push(*point);}
  }
  let(affine,inverses,batches)=if flat.is_empty(){(vec![],vec![],0)}else{let(a,z)=c.batch(&flat)?;(a,z,1)};
  let shifts=shift_indices.iter().map(|i|i.map(|j|affine[j])).collect();
  let slots=mapping.iter().map(|&(offset,lane,j)|Slot{offset,lane,label:self.labels[lane],projective:flat[j].unwrap().map(|f|f.v),affine:affine[j],inverse_z:inverses[j]}).collect();
  Ok(JointWindow{shifts,converted_points:flat.len(),geometry:Window{labels:self.labels.clone(),initialization,snapshots,slots,conversion_batches:batches}})
 }
}
