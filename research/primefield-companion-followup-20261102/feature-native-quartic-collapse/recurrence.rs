//! Generate g^h from both ends without a characteristic-r anchor.
#[derive(Debug,PartialEq)]pub enum Error{Domain,Limit,Label}
pub struct Context{r:u64,h:usize,inverses:Vec<u64>}
fn mul(a:u64,b:u64,r:u64)->u64{((a as u128*b as u128)%r as u128)as u64}
fn pow(mut a:u64,mut n:usize,r:u64)->u64{let mut v=1;while n>0{if n&1!=0{v=mul(v,a,r);}a=mul(a,a,r);n>>=1;}v}
impl Context{
 /// Caller certifies primality; max_rows bounds the shared inverse table.
 pub fn new(r:u64,max_rows:usize)->Result<Self,Error>{
  if r<3||r>u64::MAX/2||r&1==0{return Err(Error::Domain);}
  let n=usize::try_from(r).map_err(|_|Error::Limit)?;if n>max_rows{return Err(Error::Limit);}
  let mut inverses=vec![0;n];inverses[1]=1;
  for k in 2..n{inverses[k]=(r-mul(r/k as u64,inverses[(r%k as u64)as usize],r))%r;}
  Ok(Self{r,h:(n-1)/2,inverses})
 }
 fn prefix(&self,g:&[u64],last:usize)->Result<Vec<u64>,Error>{
  let r=self.r;if last>=r as usize{return Err(Error::Limit);}
  let inverse_g0=self.inverses[g[0]as usize];if mul(g[0],inverse_g0,r)!=1{return Err(Error::Domain);}
  let mut f=vec![0;last+1];f[0]=if g[0]==1{1}else{pow(g[0],self.h,r)};
  for n in 1..=last{
   let mut numerator=0;for i in 1..=(g.len()-1).min(n){
    let weight=(((self.h as i128+1)*i as i128-n as i128).rem_euclid(r as i128))as u64;
    numerator=(numerator+mul(mul(weight,g[i],r),f[n-i],r))%r;
   }
   f[n]=mul(mul(numerator,inverse_g0,r),self.inverses[n],r);
  }Ok(f)
 }
 /// Returns the canonical column and number of reverse-generated coefficients.
 pub fn column(&self,label:&[usize])->Result<(Vec<u64>,usize),Error>{
  let r=self.r;let size=r as usize;
  if label.len()>4||label.iter().any(|&a|a>=size)||!label.windows(2).all(|p|p[0]<p[1]){return Err(Error::Label);}
  let shift=if label.first()==Some(&0){self.h}else{0};
  let mut g=vec![1];for &a in label.iter().filter(|&&a|a!=0){let mut next=vec![0;g.len()+1];for(i,&c)in g.iter().enumerate(){next[i]=(next[i]+mul(c,a as u64,r))%r;next[i+1]=(next[i+1]+c)%r;}g=next;}
  let degree=(g.len()-1).checked_mul(self.h).ok_or(Error::Limit)?;
  let mut reduced=vec![0;size];
  let low=self.prefix(&g,degree.min(size-1))?;
  for(i,c)in low.into_iter().enumerate(){let e=i+shift;let k=if e==0{0}else{1+(e-1)%(size-1)};reduced[k]=(reduced[k]+c)%r;}
  let mut reverse_coefficients=0;
  if degree>=size{
   g.reverse();let high=self.prefix(&g,degree-size)?;reverse_coefficients=high.len();
   for(j,c)in high.into_iter().enumerate(){let e=degree-j+shift;let k=1+(e-1)%(size-1);reduced[k]=(reduced[k]+c)%r;}
  }
  Ok((reduced,reverse_coefficients))
 }
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn singular_prefix_is_rejected(){let c=Context::new(7,7).unwrap();assert_eq!(c.prefix(&[1,1],7),Err(Error::Limit));}
 #[test]fn empty_and_zero_roots(){let c=Context::new(7,7).unwrap();assert_eq!(c.column(&[]).unwrap(),(vec![1,0,0,0,0,0,0],0));assert_eq!(c.column(&[0]).unwrap(),(vec![0,0,0,1,0,0,0],0));}
}
