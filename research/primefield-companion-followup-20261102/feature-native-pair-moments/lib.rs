//! Low-order moments of two shifted character constraints, without a field-sized indicator.
#[path="../feature-native-half-gcd/polynomial.rs"]mod polynomial;
#[derive(Debug,PartialEq)]pub enum Error{Domain,Limit,Sign,Source,Unresolved,Verification}
#[derive(Debug,PartialEq)]pub struct Moments{pub count:u64,pub first:u64,pub singleton:Option<u64>}
fn mul(a:u64,b:u64,r:u64)->u64{((a as u128*b as u128)%r as u128)as u64}
fn pow(mut a:u64,mut n:u64,r:u64)->u64{let mut v=1;while n>0{if n&1!=0{v=mul(v,a,r)}a=mul(a,a,r);n>>=1}v}
fn inverses(r:u64,n:usize)->Vec<u64>{let mut v=vec![0;n+1];if n>0{v[1]=1}for j in 2..=n{v[j]=r-mul(r/j as u64,v[(r%j as u64)as usize],r)}v}
pub struct Pair{r:u64,s:i8,t:i8,epsilon:i8,count:u64}
impl Pair{
 /// Caller-certified prime r>=5. Constraints are chi(x)=s, chi(x+1)=t with s,t nonzero.
 pub fn new(r:u64,s:i8,t:i8)->Result<Self,Error>{if r<5||r>u64::MAX/2||r%2==0{return Err(Error::Domain)}if ![-1,1].contains(&s)||![-1,1].contains(&t){return Err(Error::Sign)}let epsilon=if r%4==1{1}else{-1};let n=r as i128-2-(s as i128)*(t as i128)-t as i128-(s as i128)*(epsilon as i128);if n<0||n%4!=0{return Err(Error::Domain)}Ok(Self{r,s,t,epsilon,count:(n/4)as u64})}
 pub fn count(&self)->u64{self.count}
 fn prefix(&self,order:usize,max_order:usize)->Result<(Vec<u64>,Vec<u64>),Error>{
  let h=(self.r-1)/2;if order>max_order||order as u128>=h as u128{return Err(Error::Limit)}
  let inv=inverses(self.r,order);let inv4=pow(4,self.r-2,self.r);let mut binomial=1;let mut out=Vec::with_capacity(order+1);
  for j in 0..=order{if j>0{binomial=mul(mul(binomial,h-j as u64+1,self.r),inv[j],self.r)}let parity=if j%2==0{1i128}else{-1};let endpoint0=if j==0{1+self.t as i128}else{0};let value=-(self.s as i128)*(self.t as i128)*(binomial as i128)-parity*(1+(self.s as i128)*(self.epsilon as i128))-endpoint0;out.push(mul(value.rem_euclid(self.r as i128)as u64,inv4,self.r))}
  if out[0]!=self.count{return Err(Error::Verification)}Ok((out,inv))
 }
 /// Moments mu_0 through mu_order. Allocations depend on order, not on r.
 pub fn moments(&self,order:usize,max_order:usize)->Result<Vec<u64>,Error>{self.prefix(order,max_order).map(|p|p.0)}
 pub fn roots(&self,max_degree:usize)->Result<RootSet,Error>{
  if self.count as u128>max_degree as u128{return Err(Error::Limit)}let n=usize::try_from(self.count).map_err(|_|Error::Limit)?;let(moments,inv)=self.prefix(n,max_degree)?;let mut q=vec![0;n+1];q[0]=1;
  for j in 1..=n{let mut sum=0;for i in 1..=j{sum=(sum+mul(moments[i],q[j-i],self.r))%self.r}q[j]=mul((self.r-sum)%self.r,inv[j],self.r)}q.reverse();Ok(RootSet{r:self.r,coefficients:q,poly:polynomial::Poly::new(self.r).map_err(|_|Error::Domain)?})
}
}
include!("roots.rs");
#[path="../feature-native-standalone-source/lib.rs"]mod source;
#[path="../feature-orbit-local-terms/local.rs"]mod group;
pub struct RecoverySource{source:source::Source,curve:group::Curve,base:group::Point,r:u64,max_degree:usize,max_queries:usize}
#[derive(Debug)]pub struct Recovery{pub value:u64,pub signs:Vec<i8>,pub initial_degree:usize}
impl RecoverySource{
 pub fn new(p:u64,a:u64,b:u64,r:u64,base:[u64;2],max_lanes:usize,max_degree:usize,max_queries:usize)->Result<Self,Error>{if max_queries==0||max_queries as u128>r as u128{return Err(Error::Limit)}let source=source::Source::new(p,a,b,r,base,max_lanes).map_err(|_|Error::Source)?;Ok(Self{source,curve:group::Curve::new(p,a,b),base:group::Point{x:base[0],y:base[1]},r,max_degree,max_queries})}
 fn pair(t:Option<group::Point>)->Option<[u64;2]>{t.map(|p|[p.x,p.y])}
 fn verify(&self,k:u64,target:Option<[u64;2]>)->bool{if k>=self.r{return false}let(mut n,mut p,mut out)=(k,Some(self.base),None);while n>0{if n&1!=0{out=self.curve.plus(out,p)}n>>=1;if n>0{p=self.curve.plus(p,p)}}Self::pair(out)==target}
 fn finish(&self,value:u64,target:Option<[u64;2]>,signs:Vec<i8>,initial_degree:usize)->Result<Recovery,Error>{if !self.verify(value,target){return Err(Error::Verification)}Ok(Recovery{value,signs,initial_degree})}
 pub fn recover(&self,target:Option<[u64;2]>)->Result<Recovery,Error>{
  let mut current=target.map(|[x,y]|group::Point{x,y});let s=self.source.sign(target).map_err(|_|Error::Source)?;let mut signs=vec![s];if s==0{return self.finish(0,target,signs,1)}if self.max_queries<2{return Err(Error::Unresolved)}
  current=self.curve.plus(current,Some(self.base));let t=self.source.sign(Self::pair(current)).map_err(|_|Error::Source)?;signs.push(t);if t==0{return self.finish(self.r-1,target,signs,1)}
  let pair=Pair::new(self.r,s,t)?;let mut roots=pair.roots(self.max_degree)?;let initial_degree=roots.polynomial().len()-1;let mut moments=roots.moments();
  for offset in 2..=self.max_queries{
   if let Some(value)=moments.singleton{return self.finish(value,target,signs,initial_degree)}if moments.count==0{return Err(Error::Verification)}if offset==self.max_queries{break}
   current=self.curve.plus(current,Some(self.base));let sign=self.source.sign(Self::pair(current)).map_err(|_|Error::Source)?;signs.push(sign);moments=roots.observe(offset as u64,sign)?;
  }Err(Error::Unresolved)
 }
}
#[cfg(test)]mod pair_tests{
 use super::*;
 fn chi(x:u64,r:u64)->i8{if x==0{0}else if pow(x,(r-1)/2,r)==1{1}else{-1}}
 fn reference(roots:&[u64],r:u64)->Vec<u64>{let mut p=vec![1];for &t in roots{let mut q=vec![0;p.len()+1];for(j,&v)in p.iter().enumerate(){q[j]=(q[j]+mul(v,(r-t)%r,r))%r;q[j+1]=(q[j+1]+v)%r}p=q}p}
 #[test]fn exact_small_moments_and_roots(){let mut classes=0;for r in[5,7,11,13,17,31,43,101,479]{for s in[-1,1]{for t in[-1,1]{let p=Pair::new(r,s,t).unwrap();let candidates:Vec<_>=(0..r).filter(|&x|chi(x,r)==s&&chi((x+1)%r,r)==t).collect();assert_eq!(p.count(),candidates.len()as u64);let n=candidates.len();let moments=p.moments(n,128).unwrap();for j in 0..=n{let expected=candidates.iter().fold(0,|a,&x|(a+pow(x,j as u64,r))%r);assert_eq!(moments[j],expected,"r={r},s={s},t={t},j={j}")}assert_eq!(p.roots(128).unwrap().polynomial(),reference(&candidates,r));classes+=1}}}assert_eq!(classes,36);}
 #[test]fn large_word_prefix_and_caps(){let r=(1u64<<61)-1;let mut sum=vec![0;17];for s in[-1,1]{for t in[-1,1]{let p=Pair::new(r,s,t).unwrap();let values=p.moments(16,16).unwrap();for(j,&v)in values.iter().enumerate(){sum[j]=(sum[j]+v)%r}assert!(matches!(p.roots(128),Err(Error::Limit)));assert_eq!(p.moments(17,16),Err(Error::Limit));}}assert_eq!(sum[0],r-2);for j in 1..17{assert_eq!(sum[j],if j%2==0{r-1}else{1})}assert!(matches!(Pair::new(7,0,1),Err(Error::Sign)));assert!(matches!(Pair::new(8,1,1),Err(Error::Domain)));assert_eq!(Pair::new(7,1,1).unwrap().moments(3,3),Err(Error::Limit));}
 #[test]fn frozen_public_recovery(){let s=RecoverySource::new(509,2,10,479,[3,68],64,128,16).unwrap();let(mut cases,mut endpoint)=(0,0);for line in include_str!("../feature-native-monomial-recovery/recovery.tsv").lines().skip(1){let v:Vec<i64>=line.split('\t').map(|v|v.parse().unwrap()).collect();let target=if v[1]<0{None}else{Some([v[1]as u64,v[2]as u64])};let out=s.recover(target).unwrap();assert_eq!(out.value,v[0]as u64);assert!(out.initial_degree<=120);assert!(!s.verify((out.value+1)%479,target));endpoint+=usize::from(out.signs.contains(&0));cases+=1}assert_eq!(cases,20);println!("frozen_cases={cases} zero_sign_transcripts={endpoint}");let short=RecoverySource::new(509,2,10,479,[3,68],64,128,1).unwrap();assert!(matches!(short.recover(Some([3,68])),Err(Error::Unresolved)));let capped=RecoverySource::new(509,2,10,479,[3,68],64,16,16).unwrap();assert!(matches!(capped.recover(Some([3,68])),Err(Error::Limit)));assert!(matches!(s.recover(Some([509,68])),Err(Error::Source)));}
}
