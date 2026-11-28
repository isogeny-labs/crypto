/// Lagrange convolution on a scaled integer grid. Prime r is caller-certified.
struct ProgressionValues{r:u64,d:usize,inv_step:u64,weights:Vec<u64>,samples:Vec<u64>}
impl ProgressionValues{
 fn new(r:u64,p:&mut polynomial::Poly,f:&[u64],step:u64)->Result<Self,Error>{
  if r<7||r>u64::MAX/2||f.len()<2||f.len()-1>=r as usize||f.iter().any(|&x|x>=r)||step==0||step>=r{return Err(Error::Domain)}
  let d=f.len()-1;let inv_step=pow(step,r-2,r);if mul(step,inv_step,r)!=1{return Err(Error::Arithmetic)}
  // Only d points in a setup tree; the (d+1)-st sample uses scalar Horner.
  let points:Vec<_>=(0..d).map(|i|mul(step,i as u64,r)).collect();let tree=p.product_tree(&points).map_err(|_|Error::Arithmetic)?;let mut values=p.values(f,&tree).map_err(|_|Error::Arithmetic)?;values.push(eval(f,mul(step,d as u64,r),r));drop(tree);
  let mut factorial=1;for i in 1..=d{factorial=mul(factorial,i as u64,r)}let mut invfact=vec![0;d+1];invfact[d]=pow(factorial,r-2,r);if mul(factorial,invfact[d],r)!=1{return Err(Error::Arithmetic)}
  for i in(1..=d).rev(){invfact[i-1]=mul(invfact[i],i as u64,r)}
  let weights=values.iter().enumerate().map(|(i,&v)|{let x=mul(mul(v,invfact[i],r),invfact[d-i],r);if(d-i)%2!=0{(r-x)%r}else{x}}).collect();Ok(Self{r,d,inv_step,weights,samples:values})
 }
 fn evaluate(&self,p:&mut polynomial::Poly,first:u64,count:usize)->Result<Option<Vec<u64>>,Error>{
  if first>=self.r||count==0||count>self.d{return Err(Error::Domain)}
  self.evaluate_normalized(p,mul(first,self.inv_step,self.r),count)
 }
 fn evaluate_normalized(&self,p:&mut polynomial::Poly,a:u64,count:usize)->Result<Option<Vec<u64>>,Error>{
  if a>=self.r||count==0||count>self.d{return Err(Error::Domain)}let r=self.r;let n=self.d.checked_add(count).ok_or(Error::Limit)?;let mut current=(a+r-self.d as u64)%r;let mut denominators=Vec::with_capacity(n);let mut prefix=vec![1];
  for _ in 0..n{if current==0{return Ok(None)}denominators.push(current);prefix.push(mul(*prefix.last().unwrap(),current,r));current=(current+1)%r;}
  let mut delta=prefix[self.d+1];let mut inv=pow(prefix[n],r-2,r);if mul(inv,prefix[n],r)!=1{return Err(Error::Arithmetic)}
  for i in(0..n).rev(){let value=denominators[i];denominators[i]=mul(inv,prefix[i],r);inv=mul(inv,value,r)}
  let convolution=p.mul_low(&self.weights,&denominators,n);let mut out=Vec::with_capacity(count);let mut numerator=(a+1)%r;
  for k in 0..count{out.push(mul(delta,convolution.get(self.d+k).copied().unwrap_or(0),r));if k+1<count{delta=mul(mul(delta,numerator,r),denominators[k],r);numerator=(numerator+1)%r;}}
  Ok(Some(out))
 }
}
include!("progression-checks.rs");

include!("cached.rs");
