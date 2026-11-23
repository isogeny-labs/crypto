#[derive(Debug)]
pub struct GcdResult{pub polynomial:Vec<u64>,pub half_batches:usize,pub ordinary_steps:usize,pub recorded_transitions:usize}
impl Poly{
 /// Monic gcd, with gcd(0,0)=0. Raw coefficient length is capped before copying.
 /// Coefficients are reduced modulo caller-certified prime p by trim.
 pub fn gcd_monic(&mut self,a:&[u64],b:&[u64],max_degree:usize)->Result<GcdResult,Error>{
  if a.len().saturating_sub(1)>max_degree||b.len().saturating_sub(1)>max_degree{return Err(Error::Order)}
  let(mut a,mut b)=(self.trim(a),self.trim(b));if a.len()<b.len(){std::mem::swap(&mut a,&mut b)}
  let(mut half_batches,mut ordinary_steps,mut recorded_transitions)=(0,0,0);
  while b!=[0]{
   if a.len()>9 && b.len()<a.len() && b.len()-1>=a.len()/2{
    let old_b=b.len();let batch=self.half_gcd(&a,&b)?;
    if batch.b!=[0]&&batch.b.len()>=old_b{return Err(Error::Invariant)}
    half_batches+=1;recorded_transitions+=batch.steps.len();a=batch.a;b=batch.b;
   }else{let(_,rem)=self.divrem(&a,&b)?;a=b;b=rem;ordinary_steps+=1;}
  }
  if a!=[0]{let lead=*a.last().ok_or(Error::Invariant)?;let inverse=if lead==1{1}else{self.inverse(lead)?};a=a.iter().map(|&v|self.fm(v,inverse)).collect();}
  Ok(GcdResult{polynomial:a,half_batches,ordinary_steps,recorded_transitions})
 }
}
