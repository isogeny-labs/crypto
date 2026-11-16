//! Universal affine-linear shifted-character decoder over a caller-certified odd prime.
#[derive(Debug,PartialEq)]pub enum Error{Domain,Limit,Sign}
pub struct Program{r:u64,constant:u64,coefficients:Vec<u64>}
fn mul(a:u64,b:u64,r:u64)->u64{((a as u128*b as u128)%r as u128)as u64}
fn pow(mut a:u64,mut n:u64,r:u64)->u64{let mut z=1;while n!=0{if n&1!=0{z=mul(z,a,r);}a=mul(a,a,r);n>>=1;}z}
impl Program{
 /// Primality is a caller obligation, not inferred from the inverse check.
 /// max_terms bounds the coefficient allocation before any field loop.
 pub fn new(r:u64,max_terms:usize)->Result<Self,Error>{
  if r<3||r>u64::MAX/2||r&1==0{return Err(Error::Domain);}
  let h=(r-1)/2;let count=usize::try_from(h).map_err(|_|Error::Limit)?;
  if count>max_terms{return Err(Error::Limit);}
  let n=count-1;let mut factorial=1;for a in 1..=n{factorial=mul(factorial,a as u64,r);}
  let inverse=pow(factorial,r-2,r);if mul(inverse,factorial,r)!=1{return Err(Error::Domain);}
  let mut invfacts=vec![1;count];invfacts[n]=inverse;
  for a in (1..=n).rev(){invfacts[a-1]=mul(invfacts[a],a as u64,r);}
  // h = -1/2 modulo r, hence 1/h = -2, avoiding a second inversion.
  let mut coefficients=Vec::with_capacity(count);
  for a in 0..count{let c=mul(r-2,mul(invfacts[a],invfacts[n-a],r),r);coefficients.push(if(n-a)&1!=0{r-c}else{c});}
  let half=(r+1)/2;let constant=(r-mul(n as u64,half,r))%r;
  Ok(Self{r,constant,coefficients})
 }
 pub fn queries(&self)->usize{self.coefficients.len()}
 pub fn evaluate(&self,mut character:impl FnMut(usize)->i8)->Result<u64,Error>{
  let mut value=self.constant;
  for(a,&c)in self.coefficients.iter().enumerate(){match character(a){
   0=>(),1=>value=(value+c)%self.r,-1=>value=(value+self.r-c)%self.r,_=>return Err(Error::Sign)
  }}Ok(value)
 }
}
#[cfg(test)]#[path="../feature-native-seed-factor/routed/source.rs"]mod source;
#[cfg(test)]mod tests{
 use super::*;
 fn prime(r:u64)->bool{r>=2&&(2..).take_while(|d|d*d<=r).all(|d|r%d!=0)}
 fn character(x:u64,r:u64)->i8{match pow(x,(r-1)/2,r){0=>0,1=>1,_=>-1}}
 #[test]fn all_inputs_on_small_primes_and_saved_modulus(){
  let mut count=0;let mut fields=0;
  for r in (3..=127).filter(|&r|prime(r)).chain([479]){let p=Program::new(r,239).unwrap();assert_eq!(p.queries()as u64,(r-1)/2);
   for x in 0..r{assert_eq!(p.evaluate(|a|character((x+a as u64)%r,r)),Ok(x));count+=1;}fields+=1;
  }println!("exact input identities: {count}; prime fields: {fields}");
 }
 #[test]fn coefficient_identity_by_independent_polynomial_expansion(){
  for r in (3..=127).filter(|&r|prime(r)){let p=Program::new(r,63).unwrap();let h=p.queries();let mut sum=vec![0;h+1];sum[0]=p.constant;
   for(a,&c)in p.coefficients.iter().enumerate(){let mut poly=vec![1];
    for _ in 0..h{let mut next=vec![0;poly.len()+1];for(i,&v)in poly.iter().enumerate(){next[i]=(next[i]+mul(v,a as u64,r))%r;next[i+1]=(next[i+1]+v)%r;}poly=next;}
    for(i,v)in poly.into_iter().enumerate(){sum[i]=(sum[i]+mul(c,v,r))%r;}
   }
   let mut expected=vec![0;h+1];expected[1]=1;assert_eq!(sum,expected,"modulus {r}");
  }
 }
 #[test]fn limits_and_failure_order(){
  for r in [0,1,2,4,u64::MAX]{assert!(matches!(Program::new(r,100),Err(Error::Domain)));}
  assert!(matches!(Program::new(479,238),Err(Error::Limit)));
  assert!(matches!(Program::new(u64::MAX/2,1),Err(Error::Limit)));
  assert!(matches!(Program::new(9,4),Err(Error::Domain)));
  let p=Program::new(11,5).unwrap();let mut seen=vec![];
  assert_eq!(p.evaluate(|a|{seen.push(a);if a==2{2}else{1}}),Err(Error::Sign));assert_eq!(seen,vec![0,1,2]);
 }
 #[test]fn public_point_source_integration(){
  let p=Program::new(479,239).unwrap();let source=source::Source::new(509,2,10,479,[3,68]).unwrap();let prepared=source.prepare().unwrap();
  for line in include_str!("../feature-native-recovery/targets.tsv").lines(){let f:Vec<i64>=line.split('\t').map(|s|s.parse().unwrap()).collect();let q=if f[1]<0{None}else{Some([f[1]as u64,f[2]as u64])};
   // Only public point q feeds the source; expected label is checker-only.
   let window=prepared.window(q,p.queries()).unwrap();assert_eq!(window.signs.len(),239);
   assert_eq!(p.evaluate(|a|window.signs[a]),Ok(f[0]as u64));
  }
 }
}
