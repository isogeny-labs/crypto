//! Prime-field Miller recurrence with one Frobenius anchor for cubic powers.
#[derive(Debug,PartialEq)]pub enum Error{Domain,Limit,Label}
pub struct Context{r:u64,h:usize,inverses:Vec<u64>}
fn mul(a:u64,b:u64,r:u64)->u64{((a as u128*b as u128)%r as u128)as u64}
fn pow(mut a:u64,mut n:usize,r:u64)->u64{let mut v=1;while n>0{if n&1!=0{v=mul(v,a,r);}a=mul(a,a,r);n>>=1;}v}
impl Context{
 /// Caller certifies primality. The shared inverse table is bounded explicitly.
 pub fn new(r:u64,max_rows:usize)->Result<Self,Error>{
  if r<3||r>u64::MAX/2||r&1==0{return Err(Error::Domain);}
  let n=usize::try_from(r).map_err(|_|Error::Limit)?;if n>max_rows{return Err(Error::Limit);}
  let mut inverses=vec![0;n];inverses[1]=1;
  for k in 2..n{inverses[k]=(r-mul(r/k as u64,inverses[(r%k as u64)as usize],r))%r;}
  Ok(Self{r,h:(n-1)/2,inverses})
 }
 /// Returns canonical polynomial coefficients and the number of singular anchors.
 pub fn column(&self,label:&[usize])->Result<(Vec<u64>,usize),Error>{
  let r=self.r;let size=r as usize;
  if label.len()>3||label.iter().any(|&a|a>=size)||!label.windows(2).all(|p|p[0]<p[1]){return Err(Error::Label);}
  let shift=if label.first()==Some(&0){self.h}else{0};
  let mut g=vec![1];for &a in label.iter().filter(|&&a|a!=0){let mut next=vec![0;g.len()+1];for(i,&c)in g.iter().enumerate(){next[i]=(next[i]+mul(c,a as u64,r))%r;next[i+1]=(next[i+1]+c)%r;}g=next;}
  let degree=g.len()-1;let last=degree.checked_mul(self.h).ok_or(Error::Limit)?;let mut f=vec![0;last.checked_add(1).ok_or(Error::Limit)?];f[0]=pow(g[0],self.h,r);
  let inverse_g0=self.inverses[g[0]as usize];if mul(g[0],inverse_g0,r)!=1{return Err(Error::Domain);}
  let mut anchors=0;
  for n in 1..=last{
   if n==size{
    let mut rest=0;
    for(i,&gi)in g.iter().enumerate(){let mut convolution=0;for j in 0..=size-i{if i==0&&(j==0||j==size){continue;}convolution=(convolution+mul(f[j],f[size-i-j],r))%r;}rest=(rest+mul(gi,convolution,r))%r;}
    let denominator=mul(2,mul(g[0],f[0],r),r);let inv=self.inverses[denominator as usize];if mul(denominator,inv,r)!=1{return Err(Error::Domain);}
    f[n]=mul((g[1]+r-rest)%r,inv,r);anchors+=1;
   }else{
    let mut numerator=0;
    for i in 1..=degree.min(n){let weight=(((self.h as i128+1)*i as i128-n as i128).rem_euclid(r as i128))as u64;numerator=(numerator+mul(mul(weight,g[i],r),f[n-i],r))%r;}
    f[n]=mul(mul(numerator,inverse_g0,r),self.inverses[n%size],r);
   }
  }
  let mut reduced=vec![0;size];for(i,&c)in f.iter().enumerate(){let e=i+shift;let k=if e==0{0}else{1+(e-1)%(size-1)};reduced[k]=(reduced[k]+c)%r;}
  Ok((reduced,anchors))
 }
}
