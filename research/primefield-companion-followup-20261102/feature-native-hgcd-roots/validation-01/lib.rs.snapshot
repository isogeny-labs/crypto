//! Fast prime-field power-sum conversion and consecutive-sign recovery.
mod polynomial;
#[path="../feature-native-inert-seed/lib.rs"]mod seed;
#[path="../feature-native-standalone-source/lib.rs"]mod source;
#[path="../feature-orbit-local-terms/local.rs"]mod group;
#[derive(Debug,PartialEq)]pub enum Error{Domain,Limit,Sign,Source,Unresolved,Verification}
#[derive(Debug,PartialEq)]pub struct Moments{pub count:u64,pub first:u64,pub singleton:Option<u64>}
fn mul(a:u64,b:u64,r:u64)->u64{((a as u128*b as u128)%r as u128)as u64}
fn pow(mut a:u64,mut n:u64,r:u64)->u64{let mut v=1;while n>0{if n&1!=0{v=mul(v,a,r)}a=mul(a,a,r);n>>=1}v}
#[derive(Debug)]pub struct Conversion{pub coefficients:Vec<u64>,pub doublings:usize,pub karatsuba_calls:usize,pub reciprocal_updates:usize}
/// Caller-certified prime, moments[0]=n<r, then n canonical power sums.
/// The caller establishes that these are the intended roots' moments.
/// Returns a monic degree-n polynomial, including repeated and zero roots.
pub fn from_moments(r:u64,moments:&[u64],max_degree:usize)->Result<Conversion,Error>{
 if r<5||r>u64::MAX/2||r%2==0||moments.is_empty(){return Err(Error::Domain)}
 let count=moments[0];if count>=r{return Err(Error::Domain)}if count as u128>max_degree as u128{return Err(Error::Limit)}
 let n=usize::try_from(count).map_err(|_|Error::Limit)?;let length=n.checked_add(1).ok_or(Error::Limit)?;
 if moments.len()!=length||moments.iter().any(|&v|v>=r){return Err(Error::Domain)}
 let mut inv=vec![0;length];if n>0{inv[1]=1}for j in 2..=n{inv[j]=r-mul(r/j as u64,inv[(r%j as u64)as usize],r)}
 // W = (log q)' = -sum_{j>=1} mu_j Z^(j-1).
 let w:Vec<_>=moments[1..].iter().map(|&v|(r-v)%r).collect();
 let mut p=polynomial::Poly::new(r).map_err(|_|Error::Domain)?;
 let(mut q,mut g,mut precision,mut doublings,mut reciprocal_updates)=(vec![1],vec![1],1usize,0usize,0usize);
 while precision<length{
  let next=precision.saturating_mul(2).min(length);let extra=next-precision;
  // q' has no terms at degrees precision-1 or higher.
  let residual=p.mul_low(&w[..next-1],&q,next-1);
  let high:Vec<_>=(precision-1..next-1).map(|j|residual.get(j).copied().unwrap_or(0)).collect();
  let quotient=p.mul_low(&g,&high,extra);
  let epsilon:Vec<_>=(0..extra).map(|i|mul(quotient.get(i).copied().unwrap_or(0),inv[precision+i],r)).collect();
  let correction=p.mul_low(&q,&epsilon,extra);q.resize(next,0);
  for i in 0..extra{q[precision+i]=correction.get(i).copied().unwrap_or(0)}
  if next<length{
   let qg=p.mul_low(&q,&g,next);
   let error:Vec<_>=(precision..next).map(|j|qg.get(j).copied().unwrap_or(0)).collect();
   let correction=p.mul_low(&g,&error,extra);g.resize(next,0);
   for i in 0..extra{g[precision+i]=(r-correction.get(i).copied().unwrap_or(0))%r}
   reciprocal_updates+=1;
  }
  precision=next;doublings+=1;
 }
 // Deliberately retain the prescribed length: zero roots lower reversed degree.
 q.resize(length,0);q.reverse();
 Ok(Conversion{coefficients:q,doublings,karatsuba_calls:p.karatsuba_calls,reciprocal_updates})
}
fn as_error(e:seed::Error)->Error{match e{seed::Error::Limit=>Error::Limit,seed::Error::Sign=>Error::Sign,_=>Error::Domain}}
pub struct Consecutive{r:u64,seed:seed::Consecutive}
impl Consecutive{
 pub fn new(r:u64,max_width:usize,max_points:usize,max_cm_attempts:u64)->Result<Self,Error>{Ok(Self{r,seed:seed::Consecutive::new(r,max_width,max_points,max_cm_attempts).map_err(as_error)?})}
 pub fn roots(&self,signs:[i8;3],max_degree:usize)->Result<RootSet,Error>{
  let n=self.seed.moments(signs,0,0).map_err(as_error)?[0];if n as u128>max_degree as u128{return Err(Error::Limit)}
  let n=usize::try_from(n).map_err(|_|Error::Limit)?;let moments=self.seed.moments(signs,n,max_degree).map_err(as_error)?;
  let c=from_moments(self.r,&moments,max_degree)?;Ok(RootSet{r:self.r,coefficients:c.coefficients,poly:polynomial::Poly::new(self.r).map_err(|_|Error::Domain)?})
 }
}
include!("roots.rs");
include!("recovery.rs");
#[cfg(test)]mod tests;
