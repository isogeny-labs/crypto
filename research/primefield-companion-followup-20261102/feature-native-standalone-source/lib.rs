//! Standalone projected source. Prime p,r are caller-certified; no full-value runtime reference.
#[path="../feature-native-joint-window/lib.rs"]mod geometry;
#[path="../feature-native-value-division/lib.rs"]mod division;
#[path="../feature-native-half-gcd/residual.rs"]mod residual;
pub type Pair=[u64;2];
#[derive(Debug,PartialEq)]pub enum Error{Domain,Point,NonTorsion,Geometry,Polynomial,Degenerate,Character,Limit}
fn mul(a:u64,b:u64,p:u64)->u64{((a as u128*b as u128)%p as u128)as u64}
fn pow(mut a:u64,mut n:u64,p:u64)->u64{let mut z=1;while n>0{if n&1==1{z=mul(z,a,p)}a=mul(a,a,p);n>>=1;}z}
fn partition(r:u64)->[u64;4]{let h=(r-1)/2;let target=(h+1)/2;let(mut lo,mut hi)=(1,target);while lo<hi{let m=lo+(hi-lo)/2;if(m as u128)*(m as u128)>=target as u128{hi=m}else{lo=m+1}}let w=lo.min(h/2);let t=h%(2*w);[w,h/(2*w),t/2,t%2]}
fn powers(part:[u64;4],q:u64)->[u64;3]{let[w,k,s,e]=part;let mut f=[k-1,k,k+1];let i=f.iter().position(|n|n%3==0).unwrap();f[i]/=3;let g=f.iter().fold(1,|a,b|a*(b%q)%q);[((4*(w as u128)*(k as u128)+2*(s as u128)+e as u128)%q as u128)as u64,(2*(w%q)*g)%q,(2*(s%q))%q]}
pub struct Source {p:u64,a:u64,b:u64,r:u64,part:[u64;4],powers:[u64;3],curve:geometry::Curve,calibration:u64}
impl Source{
 /// max_lanes bounds materialized geometry before allocation. p,r primality is a caller obligation.
 pub fn new(p:u64,a:u64,b:u64,r:u64,base:[u64;2],max_lanes:usize)->Result<Self,Error>{
  if p<5||p>u64::MAX/2||r<7||r>u64::MAX/2||r%2==0||r==p{return Err(Error::Domain)}
  let q=1u64<<(p-1).trailing_zeros();if q>4{return Err(Error::Domain)}let(mut x,mut y)=(r,p-1);while y!=0{(x,y)=(y,x%y)}if x!=1{return Err(Error::Domain)}
  let part=partition(r);let[w,k,s,_]=part;let lanes=1+w as u128+k as u128+u128::from(s!=0);if lanes>max_lanes as u128{return Err(Error::Limit)}
  let curve=geometry::Curve::new(p,a,b).map_err(|_|Error::Point)?;let mut out=Self{p,a,b,r,part,powers:powers(part,q),curve,calibration:1};out.calibration=out.raw(base)?;if out.calibration==0{return Err(Error::Degenerate)}Ok(out)
 }
 /// Uncalibrated projected source. One value-only division cache per query, including torsion check.
 pub fn raw(&self,t:[u64;2])->Result<u64,Error>{
  let mut d=division::ValueDivision::new(self.p,self.a,self.b,t[0],t[1]).map_err(|_|Error::Point)?;
  if d.value(self.r)!=0{return Err(Error::NonTorsion)}
  let[w,k,s,e]=self.part;let indices=[self.r/2+1,2*w,if s==0{1}else{s+e}];let mut factor=1;
  for(i,&n)in indices.iter().enumerate(){if self.powers[i]==0{continue}let v=d.value(n);if v==0{return Err(Error::Degenerate)}factor=mul(factor,pow(v,self.powers[i],self.p),self.p);}
  let grid=self.curve.grid(self.r,t,w,k,s,e).map_err(|_|Error::Geometry)?;
  let g=residual::geometry_ratio(self.p,self.part,t,&grid.affine)?;if g[1]!=1{return Err(Error::Geometry)}
  let q=1u64<<(self.p-1).trailing_zeros();Ok(mul(g[0],pow(factor,(self.p-1)/q,self.p),self.p))
 }
 /// Public point input only. Identity maps to0; nonzero points must be in the intended prime subgroup.
 pub fn sign(&self,t:Option<[u64;2]>)->Result<i8,Error>{let t=match t{None=>return Ok(0),Some(t)=>t};let v=self.raw(t)?;if v==self.calibration{Ok(1)}else if v==self.p-self.calibration{Ok(-1)}else{Err(Error::Character)}}
}
#[cfg(test)]#[path="../feature-native-source/lib.rs"]mod original;
#[cfg(test)]#[path="../feature-orbit-local-terms/local.rs"]mod ec;
#[cfg(test)]mod standalone_tests{
 use super::*;
 #[test]fn complete_fixture_signs(){let mut count=0;for(p,a,b,r,base)in[(5,2,1,7,[0,1]),(7,0,3,13,[1,2]),(509,2,10,479,[3,68])]{let s=Source::new(p,a,b,r,base,64).unwrap();let old=original::Source::new(p,a,b,r,base).unwrap();assert_eq!(s.sign(None),Ok(0));let c=ec::Curve::new(p,a,b);let base=ec::Point{x:base[0],y:base[1]};let mut t=Some(base);for scalar in 1..r{let v=t.unwrap();let point=[v.x,v.y];let expected=if pow(scalar,(r-1)/2,r)==1{1}else{-1};assert_eq!(s.sign(Some(point)),Ok(expected));assert_eq!(old.sign(Some(point)),Ok(expected));let q=1u64<<(p-1).trailing_zeros();assert_eq!(s.raw(point).unwrap(),pow(old.raw(point).unwrap().value,(p-1)/q,p));t=c.plus(t,Some(base));count+=1;}assert_eq!(t,None);}assert_eq!(count,496);}
 fn beta(p:u64,a:u64,b:u64,r:u64,t:[u64;2])->u64{let mut d=division::ValueDivision::new(p,a,b,t[0],t[1]).unwrap();let x=d.value(r+1);let two=d.value(2);let y=d.value(r+2);mul(mul(mul(x,x,p),two,p),pow(y,p-2,p),p)}
 #[test]fn beta_covariance(){for(p,a,b,r,base)in[(5,2,1,7,[0,1]),(7,0,3,13,[1,2]),(509,2,10,479,[3,68])]{let c=ec::Curve::new(p,a,b);let bp=ec::Point{x:base[0],y:base[1]};let b0=beta(p,a,b,r,base);let mut d=division::ValueDivision::new(p,a,b,base[0],base[1]).unwrap();let mut t=Some(bp);for n in 1..r{let t0=t.unwrap();let b1=beta(p,a,b,r,[t0.x,t0.y]);assert_eq!(mul(b1,pow(d.value(n),r*r,p),p),pow(b0,n*n,p));t=c.plus(t,Some(bp));}}}
 #[test]fn domain_and_limits(){assert!(matches!(Source::new(509,2,10,479,[3,68],1),Err(Error::Limit)));assert!(matches!(Source::new(17,2,2,19,[0,6],64),Err(Error::Domain)));assert!(matches!(Source::new(5,0,1,7,[0,1],64),Err(Error::NonTorsion)));let s=Source::new(509,2,10,479,[3,68],64).unwrap();assert_eq!(s.sign(Some([0,0])),Err(Error::Point));assert_eq!(s.sign(Some([509,68])),Err(Error::Point));}
}
