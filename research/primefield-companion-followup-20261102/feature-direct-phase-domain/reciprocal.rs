// Prime r is caller-certified. No root enumeration in this construction.
struct CoefficientReader<'a>{stream:SharedCoeffStream<'a>,buffer:Vec<u64>,at:usize,remaining:usize}
impl<'a> CoefficientReader<'a>{
 fn next(&mut self)->Result<u64,Error>{if self.remaining==0{return Err(Error::Limit)}if self.at==self.buffer.len(){self.buffer=self.stream.take_desc(self.remaining.min(self.stream.cap))?;self.at=0;}let v=self.buffer[self.at];self.at+=1;self.remaining-=1;Ok(v)}
}
struct AffineClenshaw{matrix:[[Vec<u64>;2];2],offset:[Vec<u64>;2]}
fn clenshaw_block(p:&mut polynomial::Poly,reader:&mut CoefficientReader<'_>,length:usize,r:u64)->Result<AffineClenshaw,Error>{
 if length==0{return Err(Error::Domain)}
 if length==1{return Ok(AffineClenshaw{matrix:[[vec![0,1],vec![r-1]],[vec![1],vec![0]]],offset:[vec![reader.next()?],vec![0]]})}
 let left=clenshaw_block(p,reader,length/2,r)?;let right=clenshaw_block(p,reader,length-length/2,r)?;
 let mut matrix:[[Vec<u64>;2];2]=std::array::from_fn(|_|std::array::from_fn(|_|vec![0]));let mut offset=[vec![0],vec![0]];
 for i in 0..2{for j in 0..2{let a=p.mul(&right.matrix[i][0],&left.matrix[0][j]);let b=p.mul(&right.matrix[i][1],&left.matrix[1][j]);matrix[i][j]=p.add(&a,&b);assert!(matrix[i][j].len()<=length+1);}
  let a=p.mul(&right.matrix[i][0],&left.offset[0]);let b=p.mul(&right.matrix[i][1],&left.offset[1]);offset[i]=p.add(&p.add(&a,&b),&right.offset[i]);assert!(offset[i].len()<=length);
 }Ok(AffineClenshaw{matrix,offset})
}
struct ReciprocalMode{r:u64,n:usize,h:Vec<u64>,dn:Vec<u64>}
impl ReciprocalMode{
 fn build(modes:&RecoveryModes,max_degree:usize)->Result<Option<Self>,Error>{
  let r=modes.r;if ((r-1)/modes.m as u64)%4!=0{return Ok(None)}let divisor=4*modes.m as u64;
  let n=usize::try_from((r-1)/divisor).map_err(|_|Error::Limit)?;if n==0||n>max_degree{return Err(Error::Limit)}
  let mut reader=CoefficientReader{stream:modes.stream(0)?,buffer:Vec::new(),at:0,remaining:n+1};let mut p=polynomial::Poly::new(r).map_err(|_|Error::Domain)?;
  let c=clenshaw_block(&mut p,&mut reader,n,r)?;let center=reader.next()?;assert_eq!(reader.remaining,0);
  let wb=p.mul(&[0,1],&c.offset[0]);let twice:Vec<_>=c.offset[1].iter().map(|&x|mul(2,x,r)).collect();let mut h=p.sub(&wb,&twice);h[0]=(h[0]+center)%r;
  let dn=p.add(&c.matrix[0][0],&c.matrix[1][1]);assert_eq!(h.len(),n+1);assert_eq!(dn.len(),n+1);Ok(Some(Self{r,n,h,dn}))
 }
 fn branch(&self,u:u64,c:u64)->Result<Vec<u64>,Error>{
  let r=self.r;if u==0||u>=r||c>=r||pow(u,4,r)!=1{return Err(Error::Domain)}let mut p=polynomial::Poly::new(r).map_err(|_|Error::Domain)?;
  let mut domain=self.dn.clone();domain[0]=(domain[0]+r-(u+pow(u,3,r))%r)%r;
  let mut equation:Vec<_>=self.h.iter().map(|&x|mul(u,x,r)).collect();equation[0]=(equation[0]+r-c)%r;
  Ok(p.gcd_monic(&domain,&equation,self.n).map_err(|_|Error::Arithmetic)?.polynomial)
 }
 fn lift(&self,g:&[u64],u:u64,cap:usize)->Result<Vec<u64>,Error>{
  if g.is_empty()||g.last()!=Some(&1)||g.iter().any(|&x|x>=self.r)||u==0||u>=self.r||pow(u,4,self.r)!=1{return Err(Error::Domain)}
  let d=g.len()-1;if d.checked_mul(2).ok_or(Error::Limit)?>cap{return Err(Error::Limit)}if d==0{return Ok(vec![1])}
  let r=self.r;let mut p=polynomial::Poly::new(r).map_err(|_|Error::Domain)?;let mut lifted=vec![1];
  for j in 1..=d{lifted=p.mul(&lifted,&[1,0,1]);lifted[j]=(lifted[j]+g[d-j])%r;}
  let(mut acc,mut base,mut exponent)=(vec![1],vec![0,1],self.n);while exponent>0{if exponent&1!=0{let v=p.mul(&acc,&base);acc=p.divrem(&v,&lifted).map_err(|_|Error::Arithmetic)?.1;}exponent>>=1;if exponent>0{let v=p.mul(&base,&base);base=p.divrem(&v,&lifted).map_err(|_|Error::Arithmetic)?.1;}}
  acc[0]=(acc[0]+r-u)%r;Ok(p.gcd_monic(&lifted,&acc,2*d).map_err(|_|Error::Arithmetic)?.polynomial)
 }
 fn image_constraint(&self,a:u64,t:u64,f0:u64,i:u64,cap:usize)->Result<Vec<u64>,Error>{
  let r=self.r;if a==0||a>=r||t==0||t>=r||f0>=r||i>=r||mul(i,i,r)!=r-1||pow(t,4*self.n as u64,r)!=1{return Err(Error::Domain)}
  let c=mul(f0,pow(a,r-2,r),r);let mut out=if f0==a{if cap==0{return Err(Error::Limit)}vec![0,1]}else{vec![1]};let mut p=polynomial::Poly::new(r).map_err(|_|Error::Domain)?;
  for u in [1,r-1,i,r-i]{let g=self.branch(u,c)?;let v=self.lift(&g,u,cap)?;if out.len()-1+v.len()-1>cap{return Err(Error::Limit)}out=p.mul(&out,&v);}
  let(mut scale,mut lead)=(1,1);for x in &mut out{*x=mul(*x,scale,r);lead=scale;scale=mul(scale,t,r);}let inverse=pow(lead,r-2,r);for x in &mut out{*x=mul(*x,inverse,r)}Ok(out)
 }
}
std::thread_local!{static RECIPROCAL_COUNTS:std::cell::Cell<(usize,usize,usize)>=const{std::cell::Cell::new((0,0,0))};}
impl RecoveryModes<'_>{
 fn reciprocal_initial(&self,a:u64,t:u64,f0:u64,cap:usize)->Result<Option<Vec<u64>>,Error>{
  if ((self.r-1)/self.m as u64)%4!=0{return Ok(None)}
  // Bounded public constant search; every exponentiation is part of setup.
  let mut fourth=None;for z in 2..self.r.min(66){let i=pow(z,(self.r-1)/4,self.r);if mul(i,i,self.r)==self.r-1{fourth=Some(i);break}}
  let Some(i)=fourth else{return Ok(None)};let model=ReciprocalMode::build(self,((self.r-1)/(4*self.m as u64))as usize)?.ok_or(Error::Arithmetic)?;
  RECIPROCAL_COUNTS.with(|c|{let(a,b,d)=c.get();c.set((a+1,b,d))});
  match model.image_constraint(a,t,f0,i,cap){Ok(g)=>{RECIPROCAL_COUNTS.with(|c|{let(a,b,d)=c.get();c.set((a,b+1,d))});Ok(Some(g))},Err(Error::Limit)=>{RECIPROCAL_COUNTS.with(|c|{let(a,b,d)=c.get();c.set((a,b,d+1))});Ok(None)},Err(e)=>Err(e)}
 }
}
include!("reciprocal-checks.rs");
