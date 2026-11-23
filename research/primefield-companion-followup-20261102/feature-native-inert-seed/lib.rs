//! Consecutive triple seed in the inert r=3 mod4 branch, over the prime field.
#[path="../feature-native-half-gcd/polynomial.rs"]mod polynomial;
#[path="../feature-native-triple-moments/lib.rs"]mod triple;
#[path="../feature-native-standalone-source/lib.rs"]mod source;
#[path="../feature-orbit-local-terms/local.rs"]mod group;
#[derive(Debug,PartialEq)]pub enum Error{Domain,Limit,Sign,Source,Unresolved,Verification}
#[derive(Debug,PartialEq)]pub struct Moments{pub count:u64,pub first:u64,pub singleton:Option<u64>}
fn mul(a:u64,b:u64,r:u64)->u64{((a as u128*b as u128)%r as u128)as u64}
fn pow(mut a:u64,mut n:u64,r:u64)->u64{let mut v=1;while n>0{if n&1!=0{v=mul(v,a,r)}a=mul(a,a,r);n>>=1}v}
fn ceil_sqrt(n:u64)->u64{let(mut lo,mut hi)=(0u64,1u64<<32);while hi-lo>1{let m=lo+(hi-lo)/2;if(m as u128)*(m as u128)<n as u128{lo=m}else{hi=m}}hi}
#[derive(Debug)]pub struct Seed{pub r:u64,pub b1:u64,pub block_width:usize,pub blocks:usize,pub tail:usize,pub karatsuba_calls:usize,pub newton_steps:usize}
/// Caller-certified prime r>=7, r=3 mod4. All allocation caps checked first.
/// Scalar product evaluation, with no group collision search or extension fields.
pub fn inert_seed(r:u64,max_width:usize,max_points:usize)->Result<Seed,Error>{
 if r<7||r>u64::MAX/2||r%4!=3{return Err(Error::Domain)}
 let m=(r-3)/4;let w=ceil_sqrt(m);let q=m/w;let tail=m%w;
 if w as u128>max_width as u128||2*q as u128>max_points as u128{return Err(Error::Limit)}
 let width=usize::try_from(w).map_err(|_|Error::Limit)?;let blocks=usize::try_from(q).map_err(|_|Error::Limit)?;
 let mut poly=polynomial::Poly::new(r).map_err(|_|Error::Domain)?;
 let roots:Vec<_>=(1..=w).map(|i|r-i).collect();let product=poly.product_tree(&roots).map_err(|_|Error::Domain)?;
 let mut points=Vec::with_capacity(2*blocks);
 for j in 0..q{points.push(j*w)}for j in 0..q{points.push(m+1+j*w)}
 let eval_tree=poly.product_tree(&points).map_err(|_|Error::Domain)?;
 let values=poly.values(product.polynomial(),&eval_tree).map_err(|_|Error::Domain)?;
 let(mut denominator,mut numerator)=(1,1);
 for &v in &values[..blocks]{denominator=mul(denominator,v,r)}
 for &v in &values[blocks..]{numerator=mul(numerator,v,r)}
 for i in q*w+1..=m{denominator=mul(denominator,i,r);numerator=mul(numerator,m+1+i,r)}
 let inverse=pow(denominator,r-2,r);if mul(denominator,inverse,r)!=1{return Err(Error::Verification)}
 let c=mul(numerator,inverse,r);let b1=if(m+1)%2==0{c}else{(r-c)%r};
 Ok(Seed{r,b1,block_width:width,blocks,tail:tail as usize,karatsuba_calls:poly.karatsuba_calls,newton_steps:poly.newton_steps})
}
fn as_error(e:triple::Error)->Error{match e{triple::Error::Limit=>Error::Limit,triple::Error::Sign=>Error::Sign,_=>Error::Domain}}
pub struct Consecutive{r:u64,moments:triple::Triple}
impl Consecutive{
 pub fn new(r:u64,max_width:usize,max_points:usize,max_cm_attempts:u64)->Result<Self,Error>{
  let moments=if r%4==3{let seed=inert_seed(r,max_width,max_points)?;triple::Triple::with_seeds(r,2,[0,seed.b1]).map_err(as_error)?}
  else{triple::consecutive_cm(r,max_cm_attempts).and_then(|c|c.triple()).map_err(as_error)?};Ok(Self{r,moments})
 }
 pub fn moments(&self,signs:[i8;3],order:usize,max_order:usize)->Result<Vec<u64>,Error>{self.moments.moments(signs,order,max_order).map_err(as_error)}
 pub fn roots(&self,signs:[i8;3],max_degree:usize)->Result<RootSet,Error>{
  let count=self.moments(signs,0,0)?[0];if count as u128>max_degree as u128{return Err(Error::Limit)}
  let n=usize::try_from(count).map_err(|_|Error::Limit)?;let moments=self.moments(signs,n,max_degree)?;
  let mut inv=vec![0;n+1];if n>0{inv[1]=1}for j in 2..=n{inv[j]=self.r-mul(self.r/j as u64,inv[(self.r%j as u64)as usize],self.r)}
  let mut q=vec![0;n+1];q[0]=1;for j in 1..=n{let mut s=0;for i in 1..=j{s=(s+mul(moments[i],q[j-i],self.r))%self.r}q[j]=mul((self.r-s)%self.r,inv[j],self.r)}q.reverse();
  Ok(RootSet{r:self.r,coefficients:q,poly:polynomial::Poly::new(self.r).map_err(|_|Error::Domain)?})
 }
}
include!("roots.rs");
pub struct RecoverySource{source:source::Source,curve:group::Curve,base:group::Point,r:u64,seed:Consecutive,max_degree:usize,max_queries:usize}
#[derive(Debug)]pub struct Recovery{pub value:u64,pub signs:Vec<i8>,pub initial_degree:usize}
impl RecoverySource{
 pub fn new(p:u64,a:u64,b:u64,r:u64,base:[u64;2],max_lanes:usize,max_width:usize,max_points:usize,max_cm_attempts:u64,max_degree:usize,max_queries:usize)->Result<Self,Error>{
  if max_queries==0||max_queries as u128>r as u128{return Err(Error::Limit)}
  let source=source::Source::new(p,a,b,r,base,max_lanes).map_err(|_|Error::Source)?;
  let seed=Consecutive::new(r,max_width,max_points,max_cm_attempts)?;
  Ok(Self{source,curve:group::Curve::new(p,a,b),base:group::Point{x:base[0],y:base[1]},r,seed,max_degree,max_queries})
 }
 fn pair(t:Option<group::Point>)->Option<[u64;2]>{t.map(|p|[p.x,p.y])}
 fn verify(&self,k:u64,target:Option<[u64;2]>)->bool{if k>=self.r{return false}let(mut n,mut p,mut out)=(k,Some(self.base),None);while n>0{if n&1!=0{out=self.curve.plus(out,p)}n>>=1;if n>0{p=self.curve.plus(p,p)}}Self::pair(out)==target}
 fn finish(&self,value:u64,target:Option<[u64;2]>,signs:Vec<i8>,initial_degree:usize)->Result<Recovery,Error>{if !self.verify(value,target){return Err(Error::Verification)}Ok(Recovery{value,signs,initial_degree})}
 pub fn recover(&self,target:Option<[u64;2]>)->Result<Recovery,Error>{
  let mut current=target.map(|[x,y]|group::Point{x,y});let mut signs=Vec::new();
  for offset in 0..3{if offset>=self.max_queries{return Err(Error::Unresolved)}
   let sign=self.source.sign(Self::pair(current)).map_err(|_|Error::Source)?;signs.push(sign);
   if sign==0{return self.finish((self.r-offset as u64)%self.r,target,signs,1)}
   if offset<2{current=self.curve.plus(current,Some(self.base))}
  }
  let mut roots=self.seed.roots([signs[0],signs[1],signs[2]],self.max_degree)?;let initial_degree=roots.polynomial().len()-1;let mut moments=roots.moments();
  for offset in 3..=self.max_queries{
   if let Some(value)=moments.singleton{return self.finish(value,target,signs,initial_degree)}if moments.count==0{return Err(Error::Verification)}if offset==self.max_queries{break}
   current=self.curve.plus(current,Some(self.base));let sign=self.source.sign(Self::pair(current)).map_err(|_|Error::Source)?;signs.push(sign);moments=roots.observe(offset as u64,sign)?;
  }Err(Error::Unresolved)
 }
}
#[cfg(test)]mod inert_tests{
 use super::*;
 fn chi(x:u64,r:u64)->i8{if x==0{0}else if pow(x,(r-1)/2,r)==1{1}else{-1}}
 fn prime(r:u64)->bool{let mut d=2;while d*d<=r{if r%d==0{return false}d+=1}true}
 fn direct_seed(r:u64)->u64{let m=(r-3)/4;let(mut a,mut b)=(1,1);for i in 1..=m{a=mul(a,m+1+i,r);b=mul(b,i,r)}let c=mul(a,pow(b,r-2,r),r);if(m+1)%2==0{c}else{(r-c)%r}}
 fn reference(roots:&[u64],r:u64)->Vec<u64>{let mut p=vec![1];for &t in roots{let mut q=vec![0;p.len()+1];for(j,&v)in p.iter().enumerate(){q[j]=(q[j]+mul(v,(r-t)%r,r))%r;q[j+1]=(q[j+1]+v)%r}p=q}p}
 #[test]fn blocked_seed_reference_and_caps(){let mut count=0;let mut covered=false;for r in [7,11,19,23,31,43,47,59,67,71,79,83,103,127,479,1019,4091,65519,100003]{assert!(prime(r));let s=inert_seed(r,256,512).unwrap();assert_eq!(s.b1,direct_seed(r));assert_eq!(s.block_width*s.blocks+s.tail,((r-3)/4)as usize);covered|=s.karatsuba_calls>0&&s.newton_steps>0;count+=1;}assert!(covered);println!("seed_primes={count} largest_prime=100003");assert!(matches!(inert_seed((1u64<<61)-1,256,512),Err(Error::Limit)));assert!(matches!(inert_seed(479,10,512),Err(Error::Limit)));assert!(matches!(inert_seed(479,256,1),Err(Error::Limit)));assert!(matches!(inert_seed(13,256,512),Err(Error::Domain)));}
 #[test]fn exact_classes_moments_roots(){let mut classes=0;let mut moments_checked=0;for r in [7,11,13,17,19,23,31,43,47,101,479]{let t=Consecutive::new(r,256,512,64).unwrap();for s in [-1,1]{for v in [-1,1]{for w in [-1,1]{let signs=[s,v,w];let roots:Vec<_>=(0..r).filter(|&x|[chi(x,r),chi((x+1)%r,r),chi((x+2)%r,r)]==signs).collect();let n=roots.len();let got=t.moments(signs,n,128).unwrap();for j in 0..=n{assert_eq!(got[j],roots.iter().fold(0,|a,&x|(a+pow(x,j as u64,r))%r));moments_checked+=1}assert_eq!(t.roots(signs,128).unwrap().polynomial(),reference(&roots,r));classes+=1}}}}println!("sign_classes={classes} exact_moments={moments_checked}");}
 #[test]fn frozen_public_recovery_and_endpoints(){let s=RecoverySource::new(509,2,10,479,[3,68],64,256,512,64,64,16).unwrap();let(mut cases,mut max_degree)=(0,0);for line in include_str!("../feature-native-monomial-recovery/recovery.tsv").lines().skip(1){let v:Vec<i64>=line.split('\t').map(|v|v.parse().unwrap()).collect();let target=if v[1]<0{None}else{Some([v[1]as u64,v[2]as u64])};let out=s.recover(target).unwrap();assert_eq!(out.value,v[0]as u64);assert!(!s.verify((out.value+1)%479,target));max_degree=max_degree.max(out.initial_degree);cases+=1}assert_eq!(cases,20);
  let mut two=s.curve.plus(Some(s.base),Some(s.base)).unwrap();two.y=(509-two.y)%509;let out=s.recover(Some([two.x,two.y])).unwrap();assert_eq!(out.value,477);assert_eq!(out.signs.len(),3);assert_eq!(out.signs[2],0);
  let short=RecoverySource::new(509,2,10,479,[3,68],64,256,512,64,64,2).unwrap();assert!(matches!(short.recover(Some([3,68])),Err(Error::Unresolved)));
  let small=RecoverySource::new(509,2,10,479,[3,68],64,256,512,64,16,16).unwrap();assert!(matches!(small.recover(Some([3,68])),Err(Error::Limit)));
  assert!(matches!(s.recover(Some([509,68])),Err(Error::Source)));println!("frozen_recoveries={cases} maximum_initial_root_degree={max_degree} extra_endpoint=477");
 }
}
