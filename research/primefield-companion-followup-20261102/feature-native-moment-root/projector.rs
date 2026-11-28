// Aggregate transcript projector over a split prime-field function algebra.
#[path="../feature-native-half-gcd/polynomial.rs"]mod polynomial;
#[derive(Debug,PartialEq)]pub enum Error{Domain,Limit,Sign,NoConstraints,Source,Unresolved,Verification}
fn mul(a:u64,b:u64,r:u64)->u64{((a as u128*b as u128)%r as u128)as u64}
fn pow(mut a:u64,mut n:u64,r:u64)->u64{let mut v=1;while n>0{if n&1!=0{v=mul(v,a,r)}a=mul(a,a,r);n>>=1}v}
/// Caller certifies r prime. max_coefficients caps the persistent degree-r representation.
pub struct Projector{r:u64,n:usize,inverses:Vec<u64>,poly:polynomial::Poly,coefficients:Vec<u64>,constraints:usize}
#[derive(Debug,PartialEq)]pub struct Moments{pub count:u64,pub first:u64,pub singleton:Option<u64>}
impl Projector{
 pub fn new(r:u64,max_coefficients:usize)->Result<Self,Error>{
  if r<5||r>u64::MAX/2||r%2==0{return Err(Error::Domain)}let n=usize::try_from(r).map_err(|_|Error::Limit)?;if n>max_coefficients{return Err(Error::Limit)}
  let h=n/2;let mut inverses=vec![0;h+1];inverses[1]=1;for j in 2..=h{inverses[j]=r-mul(r/j as u64,inverses[(r%j as u64)as usize],r)}
  Ok(Self{r,n,inverses,poly:polynomial::Poly::new(r).map_err(|_|Error::Domain)?,coefficients:vec![1],constraints:0})
 }
 fn reduce(&self,v:Vec<u64>)->Vec<u64>{let mut out=vec![0;self.n];for(i,c)in v.into_iter().enumerate(){let j=if i==0{0}else{1+(i-1)%(self.n-1)};out[j]=(out[j]+c)%self.r}while out.len()>1&&out.last()==Some(&0){out.pop();}out}
 fn character(&self,a:u64)->Vec<u64>{let h=self.n/2;let mut out=vec![0;h+1];out[h]=1;for j in(1..=h).rev(){out[j-1]=mul(mul(mul(out[j],j as u64,self.r),a,self.r),self.inverses[h-j+1],self.r)}out}
 /// Multiply by the exact sign-class indicator, including its zero endpoint.
 pub fn observe(&mut self,offset:u64,sign:i8)->Result<Moments,Error>{
  if offset>=self.r{return Err(Error::Domain)}if !(-1..=1).contains(&sign){return Err(Error::Sign)}
  let c=self.character(offset);let square=self.poly.mul(&c,&c);
  let indicator=if sign==0{self.poly.sub(&[1],&square)}else{let t=if sign==1{self.poly.add(&square,&c)}else{self.poly.sub(&square,&c)};t.into_iter().map(|v|mul(v,(self.r+1)/2,self.r)).collect()};
  let product=self.poly.mul(&self.coefficients,&indicator);self.coefficients=self.reduce(product);self.constraints+=1;self.moments()
 }
 pub fn moments(&self)->Result<Moments,Error>{
  if self.constraints==0{return Err(Error::NoConstraints)}
  let count=(self.r-self.coefficients.get(self.n-1).copied().unwrap_or(0))%self.r;
  let first=(self.r-self.coefficients.get(self.n-2).copied().unwrap_or(0))%self.r;
  Ok(Moments{count,first,singleton:if count==1{Some(first)}else{None}})
 }
}
