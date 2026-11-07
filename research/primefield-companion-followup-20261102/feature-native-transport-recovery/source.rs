//! Consume normalized transport columns through a geometry-only residual.
#[path="transport.rs"]mod transport;
#[path="../feature-native-multiseed-factor/residual.rs"]mod residual;
pub type Pair=[u64;2];
type Point=Option<[u64;2]>;
#[derive(Debug,PartialEq)]pub enum Error{Domain,Geometry,Polynomial,Degenerate,Transport,Character,Limit}
fn mul(a:u64,b:u64,p:u64)->u64{((a as u128*b as u128)%p as u128)as u64}
fn pow(mut a:u64,mut n:u64,p:u64)->u64{let mut z=1;while n>0{if n&1!=0{z=mul(z,a,p);}n>>=1;if n>0{a=mul(a,a,p);}}z}
fn partition(r:u64)->Result<[u64;4],Error>{if r<7||r&1==0{return Err(Error::Domain);}let h=(r-1)/2;let target=(h+1)/2;let(mut lo,mut hi)=(1u64,target);while lo<hi{let mid=lo+(hi-lo)/2;if mid as u128*mid as u128>=target as u128{hi=mid;}else{lo=mid+1;}}let w=lo.min(h/2);let tail=h%(2*w);Ok([w,h/(2*w),tail/2,tail%2])}
/// Exponents [delta, alpha, beta, gamma] modulo p-1. Prime p is caller-certified.
pub fn exponents(p:u64,r:u64)->Result<[u64;4],Error>{
 if p<=3||p>u64::MAX/2{return Err(Error::Domain);}let[w,k,s,e]=partition(r)?.map(u128::from);let m=(p-1)as u128;
 let add=|a:u128,b:u128|(a+b)%m;let mm=|a:u128,b:u128|(a%m)*(b%m)%m;
 let g=|n:u128|{if n==0{return 0;}let mut f=[n-1,n,n+1];let i=f.iter().position(|v|v%3==0).unwrap();f[i]/=3;mm(mm(f[0],f[1]),f[2])};
 let delta=add(add(mm(2*w,add(mm(k,mm(w,w+1)),mm(w,mm(k,k-1)))),mm(2*k,g(w))),mm(2,g(s)));
 Ok([delta,add(mm(4*w,k),2*s+e),mm(2*w,g(k)),(2*s)%m].map(|v|v as u64))
}
fn apply(p:u64,geometry:Pair,inputs:[Pair;4],powers:[u64;4])->Result<Pair,Error>{
 if geometry.iter().chain(inputs.iter().flatten()).any(|&v|v==0||v>=p){return Err(Error::Degenerate);}
 let mut out=geometry;for(i,&e)in powers.iter().enumerate(){if e!=0{for j in 0..2{out[j]=mul(out[j],pow(inputs[i][j],e,p),p);}}}Ok(out)
}
pub struct Source{p:u64,r:u64,part:[u64;4],powers:[u64;4],transport:transport::Source,calibration:Pair}
pub struct Prepared<'a>{source:&'a Source,transport:transport::Prepared<'a>}
#[derive(Debug)]pub struct Window{pub points:Vec<Point>,pub signs:Vec<i8>,pub ratios:Vec<Pair>,pub direct_initializations:usize,pub converted_points:usize}
impl Source{
 /// Caller certifies prime p,r. Transport enforces its phase and subgroup domain.
 pub fn new(p:u64,a:u64,b:u64,r:u64,base:[u64;2],max_lanes:usize,max_window:usize)->Result<Self,Error>{
  let powers=exponents(p,r)?;let part=partition(r)?;let transport=transport::Source::new(p,a,b,r,base,max_lanes,max_window).map_err(map_transport)?;
  let mut out=Self{p,r,part,powers,transport,calibration:[1,1]};
  let column=out.transport.prepare().map_err(map_transport)?.window(Some(base),1).map_err(map_transport)?;
  out.calibration=out.ratio(base,&column.grids[0],&column.indices,&column.pairs[0])?;Ok(out)
 }
 fn ratio(&self,q:[u64;2],grid:&[[u64;2]],indices:&[u64],values:&[Pair])->Result<Pair,Error>{
  #[cfg(test)]SIGN_CALLS.with(|n|n.set(n.get()+1));
  let[w,_,s,e]=self.part;let expected=if s==0{vec![self.r/2+1,2*w,1]}else{vec![self.r/2+1,2*w,s+e,1]};
  if indices!=expected||values.len()!=expected.len(){return Err(Error::Domain);}
  let inputs=[*values.last().ok_or(Error::Domain)?,values[0],values[1],if s==0{[1,1]}else{values[2]}];
  apply(self.p,residual::geometry_ratio(self.p,self.part,q,grid)?,inputs,self.powers)
 }
 pub fn prepare(&self)->Result<Prepared<'_>,Error>{Ok(Prepared{source:self,transport:self.transport.prepare().map_err(map_transport)?})}
}

include!("source-extra.rs");
