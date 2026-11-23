// Prime r is caller-certified. No coefficient vector of length L is constructed.
struct DicksonQuotient{p:polynomial::Poly,q:Vec<u64>,products:usize,max_coefficients:usize}
impl DicksonQuotient{
 fn reduce(&mut self,a:&[u64])->Result<Vec<u64>,Error>{self.max_coefficients=self.max_coefficients.max(a.len());Ok(self.p.divrem(a,&self.q).map_err(|_|Error::Arithmetic)?.1)}
 fn product(&mut self,a:&[u64],b:&[u64])->Result<Vec<u64>,Error>{self.products+=1;let v=self.p.mul(a,b);self.reduce(&v)}
 fn pair(&mut self,n:u64)->Result<(Vec<u64>,Vec<u64>),Error>{
  if n==0{return Ok((self.reduce(&[2])?,self.reduce(&[0,1])?))}
  let(a,b)=self.pair(n/2)?;let aa=self.product(&a,&a)?;let ab=self.product(&a,&b)?;
  let even=self.p.sub(&aa,&[2]);let even=self.reduce(&even)?;
  let odd=self.p.sub(&ab,&[0,1]);let odd=self.reduce(&odd)?;
  if n%2==0{Ok((even,odd))}else{let bb=self.product(&b,&b)?;let next=self.p.sub(&bb,&[2]);Ok((odd,self.reduce(&next)?))}
 }
}
fn implicit_phase(r:u64,l:u64,epsilon:u64,q:&[u64],cap:usize)->Result<(Vec<u64>,usize,usize),Error>{
 if r<7||r>u64::MAX/2||l==0||l>=r||(epsilon!=1&&epsilon!=r-1)||q.is_empty()||q.last()!=Some(&1)||q.iter().any(|&v|v>=r)||q.len()-1>=r as usize{return Err(Error::Domain)}
 if q.len()-1>cap{return Err(Error::Limit)}
 let mut p=polynomial::Poly::new(r).map_err(|_|Error::Domain)?;
 let q=if q.len()==1{q.to_vec()}else{let derivative:Vec<_>=(1..q.len()).map(|j|mul(j as u64,q[j],r)).collect();let g=p.gcd_monic(q,&derivative,2*q.len()).map_err(|_|Error::Arithmetic)?.polynomial;let(sf,rem)=p.divrem(q,&g).map_err(|_|Error::Arithmetic)?;if rem!=[0]{return Err(Error::Arithmetic)}sf};
 let mut d=DicksonQuotient{p,q,products:0,max_coefficients:0};let(mut value,_)=d.pair(l)?;value[0]=(value[0]+r-mul(2,epsilon,r))%r;
 let out=d.p.gcd_monic(&d.q,&value,2*d.q.len()+2).map_err(|_|Error::Arithmetic)?.polynomial;
 Ok((out,d.products,d.max_coefficients))
}
