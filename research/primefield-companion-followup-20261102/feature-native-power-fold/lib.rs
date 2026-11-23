//! Prime-field nonlinear image folding. Caller certifies primality of r and source p.
#[path="../feature-native-hgcd-roots/polynomial.rs"]mod polynomial;
#[path="../feature-native-standalone-source/lib.rs"]mod source;
#[path="../feature-orbit-local-terms/local.rs"]mod group;
#[derive(Debug,PartialEq)]pub enum Error{Domain,Limit,Sign,Source,Unresolved,Verification}
#[derive(Debug,PartialEq)]pub struct Moments{pub count:u64,pub first:u64,pub singleton:Option<u64>}
fn mul(a:u64,b:u64,r:u64)->u64{((a as u128*b as u128)%r as u128)as u64}
fn pow(mut a:u64,mut n:u64,r:u64)->u64{let mut v=1;while n>0{if n&1!=0{v=mul(v,a,r)}a=mul(a,a,r);n>>=1}v}
include!("../feature-native-hgcd-roots/roots.rs");
#[derive(Clone,Debug)]pub struct Fold{r:u64,m:usize,zeta:u64,d:usize}
impl Fold{
 /// Caps are checked before order verification, allocation or polynomial work.
 pub fn new(r:u64,m:usize,zeta:u64,max_orbit:usize,max_degree:usize)->Result<Self,Error>{
  if r<5||r>u64::MAX/2||r%2==0||m<2||m as u128>=r as u128||zeta==0||zeta>=r{return Err(Error::Domain)}
  if m>max_orbit{return Err(Error::Limit)}if (r-1)%m as u64!=0{return Err(Error::Domain)}
  let d=usize::try_from((r-1)/m as u64).map_err(|_|Error::Limit)?;
  if d.checked_add(1).ok_or(Error::Limit)?>max_degree{return Err(Error::Limit)}
  let mut v=1;for j in 1..=m{v=mul(v,zeta,r);if v==1&&j!=m{return Err(Error::Domain)}}if v!=1{return Err(Error::Domain)}
  Ok(Self{r,m,zeta,d})
 }
 pub fn initial(&self)->Result<RootSet,Error>{let mut coefficients=vec![0;self.d+2];coefficients[1]=self.r-1;coefficients[self.d+1]=1;Ok(RootSet{r:self.r,coefficients,poly:polynomial::Poly::new(self.r).map_err(|_|Error::Domain)?})}
 pub fn offsets(&self,center:u64,b:u64)->Result<Vec<u64>,Error>{if center>=self.r||b==0||b>=self.r{return Err(Error::Domain)}let mut out=Vec::with_capacity(self.m);let mut t=b;for _ in 0..self.m{out.push((center+self.r-t)%self.r);t=mul(t,self.zeta,self.r)}Ok(out)}
 pub fn observe(&self,state:&mut RootSet,b:u64,sign:i8)->Result<Moments,Error>{if state.r!=self.r||b==0||b>=self.r{return Err(Error::Domain)}state.observe((self.r-pow(b,self.m as u64,self.r))%self.r,sign)}
 /// Fiber polynomial, not enumeration or an auxiliary group power.
 fn fiber(&self,center:u64,z:u64,p:&mut polynomial::Poly)->Vec<u64>{let(mut q,mut a,mut n)=(vec![1],vec![center,1],self.m);while n>0{if n&1!=0{q=p.mul(&q,&a)}n>>=1;if n>0{a=p.mul(&a,&a)}}p.sub(&q,&[z])}
 pub fn intersect(&self,centers:[u64;2],images:[u64;2])->Result<Vec<u64>,Error>{if centers[0]==centers[1]||centers.iter().chain(images.iter()).any(|&x|x>=self.r){return Err(Error::Domain)}let mut p=polynomial::Poly::new(self.r).map_err(|_|Error::Domain)?;let a=self.fiber(centers[0],images[0],&mut p);let b=self.fiber(centers[1],images[1],&mut p);p.gcd_monic(&a,&b,self.m).map(|v|v.polynomial).map_err(|_|Error::Domain)}
}
#[derive(Default,Debug)]pub struct Counts{pub source_calls:usize,pub shift_scalar_muls:usize,pub group_additions:usize,pub folded_updates:usize}
#[derive(Debug)]pub enum Outcome{Recovered{value:u64,route:&'static str,counts:Counts},Unresolved{degrees:[usize;2],fiber:Option<Vec<u64>>,counts:Counts}}
pub struct PublicRecovery{fold:Fold,source:source::Source,curve:group::Curve,base:group::Point,centers:[u64;2],max_rounds:usize,max_calls:usize}
impl PublicRecovery{
 pub fn new(p:u64,a:u64,b:u64,r:u64,base:[u64;2],m:usize,zeta:u64,max_orbit:usize,max_degree:usize,max_lanes:usize,centers:[u64;2],max_rounds:usize,max_calls:usize)->Result<Self,Error>{
  let fold=Fold::new(r,m,zeta,max_orbit,max_degree)?;if centers[0]==centers[1]||centers.iter().any(|&c|c>=r)||max_rounds==0||max_rounds as u128>=r as u128||max_calls==0{return Err(Error::Domain)}
  Ok(Self{fold,source:source::Source::new(p,a,b,r,base,max_lanes).map_err(|_|Error::Source)?,curve:group::Curve::new(p,a,b),base:group::Point{x:base[0],y:base[1]},centers,max_rounds,max_calls})
 }
 fn pair(t:Option<group::Point>)->Option<[u64;2]>{t.map(|p|[p.x,p.y])}
 fn multiple(&self,mut k:u64,counts:&mut Counts)->Option<group::Point>{let(mut a,mut out)=(Some(self.base),None);while k>0{if k&1!=0{out=self.curve.plus(out,a);counts.group_additions+=1}k>>=1;if k>0{a=self.curve.plus(a,a);counts.group_additions+=1}}out}
 fn query(&self,target:Option<[u64;2]>,counts:&mut Counts)->Result<i8,Error>{if counts.source_calls>=self.max_calls{return Err(Error::Limit)}counts.source_calls+=1;self.source.sign(target).map_err(|_|Error::Source)}
 fn finish(&self,value:u64,target:Option<[u64;2]>,route:&'static str,mut counts:Counts)->Result<Outcome,Error>{if Self::pair(self.multiple(value,&mut counts))!=target{return Err(Error::Verification)}Ok(Outcome::Recovered{value,route,counts})}
 pub fn recover(&self,target:Option<[u64;2]>)->Result<Outcome,Error>{
  let mut counts=Counts::default();if self.query(target,&mut counts)?==0{return self.finish(0,target,"identity",counts)} // validate before curve arithmetic
  let point=target.map(|[x,y]|group::Point{x,y});let mut states=[self.fold.initial()?,self.fold.initial()?];
  for b in 1..=self.max_rounds{
   for i in 0..2{if states[i].moments().singleton.is_some(){continue}
    let mut sign=1;for offset in self.fold.offsets(self.centers[i],b as u64)?{
     if counts.source_calls>=self.max_calls{return Ok(Outcome::Unresolved{degrees:[states[0].coefficients.len()-1,states[1].coefficients.len()-1],fiber:None,counts})}
     counts.shift_scalar_muls+=1;let shift=self.multiple(offset,&mut counts);let t=self.curve.plus(point,shift);counts.group_additions+=1;
     let s=self.query(Self::pair(t),&mut counts)?;if s==0{return self.finish((self.fold.r-offset)%self.fold.r,target,"zero-source",counts)}sign*=s;
    }
    self.fold.observe(&mut states[i],b as u64,sign)?;counts.folded_updates+=1;
    if states[i].moments().count==0{return Err(Error::Verification)}
   }
   if let(Some(z),Some(w))=(states[0].moments().singleton,states[1].moments().singleton){
    let fiber=self.fold.intersect(self.centers,[z,w])?;
    if fiber.len()==2{return self.finish((self.fold.r-fiber[0])%self.fold.r,target,"fiber-gcd",counts)}
    return Ok(Outcome::Unresolved{degrees:[1,1],fiber:Some(fiber),counts});
   }
  }
  Ok(Outcome::Unresolved{degrees:[states[0].coefficients.len()-1,states[1].coefficients.len()-1],fiber:None,counts})
 }
}
#[cfg(test)]mod tests;
