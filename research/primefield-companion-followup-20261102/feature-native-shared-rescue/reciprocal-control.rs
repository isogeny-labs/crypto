struct ReciprocalControl{r:u64,m:usize,zeta:u64,block:usize,products:RecoveryProducts}
impl ReciprocalControl{
 fn new(m:&StreamModes)->Result<Self,Error>{Ok(Self{r:m.r,m:m.m,zeta:m.zeta,block:m.block,products:RecoveryProducts::new(m.r,m.m,m.block)?})}
 fn stream(&self,e:usize)->Result<SharedCoeffStream<'_>,Error>{SharedCoeffStream::new(self.r,self.m,e,self.block,&self.products)}
 fn observed(&self,signs:&[u64])->Result<Vec<u64>,Error>{
  if signs.len()!=self.m||signs.iter().any(|&s|s!=0&&s!=1&&s!=self.r-1){return Err(Error::Domain)}
  let h=(self.r-1)/2;Ok((0..self.m.min(3)).map(|e|{let root=pow(self.zeta,(e as u64+self.m as u64-h%self.m as u64)%self.m as u64,self.r);let(mut w,mut sum)=(1,0);for &s in signs{sum=(sum+mul(s,w,self.r))%self.r;w=mul(w,root,self.r)}sum}).collect())
 }
 fn evaluate(&self,e:usize,x:u64)->Result<u64,Error>{
  if x>=self.r{return Err(Error::Domain)}let mut s=self.stream(e)?;let mut out=0;
  while s.left>0{for c in s.take_desc(s.left.min(self.block))?{out=(mul(out,x,self.r)+c)%self.r}}Ok(out)
 }
 fn phase(&self,v:u64,b:u64,f1:u64)->Result<Option<u64>,Error>{
  let r=self.r;if v>=r||b==0||b>=r||f1>=r{return Err(Error::Domain)}let a=mul(self.m as u64,pow(b,(r-1)/2,r),r);let den=mul(a,self.evaluate(1,v)?,r);if den==0{return Ok(None)}let x=mul(f1,pow(den,r-2,r),r);if pow(x,self.m as u64,r)!=v{return Err(Error::Arithmetic)}Ok(Some(x))
 }
 fn residue(&self,p:&mut polynomial::Poly,e:usize,t:u64,g:&[u64])->Result<Vec<u64>,Error>{
  let r=self.r;if g.len()<2||g.last()!=Some(&1)||g.iter().any(|&x|x>=r)||t==0||t>=r{return Err(Error::Domain)}
  let n=g.len()-1;let w=n.min(self.block);let mut powers=Vec::with_capacity(w);let mut tw=1;for _ in 0..w{powers.push(tw);tw=mul(tw,t,r)}
  let step=if w==n{g[..n].iter().map(|&x|mul((r-x)%r,tw,r)).collect::<Vec<_>>()}else{let mut v=vec![0;w+1];v[w]=tw;v};
  let mut s=self.stream(e)?;let mut acc=vec![0];let mut first=true;
  while s.left>0{
   let count=if first{(s.left-1)%w+1}else{w};let chunk=s.take_desc(count)?;
   if !first{let product=p.mul(&acc,&step);assert!(product.len()<=2*n-1);acc=p.divrem(&product,g).map_err(|_|Error::Arithmetic)?.1;}first=false;
   if acc.len()<count{acc.resize(count,0)}for (i,&c) in chunk.iter().rev().enumerate(){acc[i]=(acc[i]+mul(c,powers[i],r))%r}
   while acc.len()>1&&acc.last()==Some(&0){acc.pop();}
  }Ok(acc)
 }
 fn first_c0(&self,a:u64,t:u64,f0:u64)->Result<Vec<u64>,Error>{
  if let Some(g)=self.reciprocal_initial(a,t,f0,self.block)?{return Ok(g)}
  let mut s=self.stream(0)?;let mut out=vec![0;s.left];let mut at=out.len();
  while s.left>0{for c in s.take_desc(s.left.min(self.block))?{at-=1;out[at]=c;}}
  let mut scale=a;for c in &mut out{*c=mul(*c,scale,self.r);scale=mul(scale,t,self.r)}out[0]=(out[0]+self.r-f0)%self.r;Ok(out)
 }
 fn intersect_modes(&self,p:&mut polynomial::Poly,state:&mut ImageDomain,b:u64,f:&[u64])->Result<(),Error>{
  let r=self.r;if state.r!=r||state.m!=self.m||b==0||b>=r||f.len()!=self.m.min(3)||f.iter().any(|&x|x>=r){return Err(Error::Domain)}if state.degree()==0{return Ok(())}
  let a=mul(self.m as u64,pow(b,(r-1)/2,r),r);let t=pow(pow(b,self.m as u64,r),r-2,r);
  let c0=if let Some(g)=&state.dense{let mut c=self.residue(p,0,t,g)?;for x in &mut c{*x=mul(*x,a,r)}c[0]=(c[0]+r-f[0])%r;c}else{self.first_c0(a,t,f[0])?};
  state.intersect(p,&c0)?;drop(c0);if self.m<3||state.degree()==0{return Ok(())}
  let c2=if let Some(g)=&state.dense{let u=self.residue(p,1,t,g)?;let mut v=self.residue(p,2,t,g)?;let square=p.mul(&u,&u);assert!(square.len()<=2*(g.len()-1)-1);let mut square=p.divrem(&square,g).map_err(|_|Error::Arithmetic)?.1;
   let left=mul(f[1],f[1],r);let right=mul(a,f[2],r);for x in &mut v{*x=mul(*x,left,r)}for x in &mut square{*x=mul(*x,right,r)}p.sub(&v,&square)
  }else{if self.m as u64!=r-1{return Err(Error::Arithmetic)}let u=self.evaluate(1,0)?;let v=self.evaluate(2,0)?;vec![(mul(mul(f[1],f[1],r),v,r)+r-mul(mul(a,f[2],r),mul(u,u,r),r))%r]};state.intersect(p,&c2)
 }
}

impl ReciprocalControl{
 fn reciprocal_initial(&self,a:u64,t:u64,f0:u64,cap:usize)->Result<Option<Vec<u64>>,Error>{
  if ((self.r-1)/self.m as u64)%4!=0{return Ok(None)}
  // Bounded public constant search; every exponentiation is part of setup.
  let mut fourth=None;for z in 2..self.r.min(66){let i=pow(z,(self.r-1)/4,self.r);if mul(i,i,self.r)==self.r-1{fourth=Some(i);break}}
  let Some(i)=fourth else{return Ok(None)};let model=self.reciprocal_model(((self.r-1)/(4*self.m as u64))as usize)?.ok_or(Error::Arithmetic)?;
  RECIPROCAL_COUNTS.with(|c|{let(a,b,d)=c.get();c.set((a+1,b,d))});
  match model.image_constraint(a,t,f0,i,cap){Ok(g)=>{RECIPROCAL_COUNTS.with(|c|{let(a,b,d)=c.get();c.set((a,b+1,d))});Ok(Some(g))},Err(Error::Limit)=>{RECIPROCAL_COUNTS.with(|c|{let(a,b,d)=c.get();c.set((a,b,d+1))});Ok(None)},Err(e)=>Err(e)}
 }
}
impl ReciprocalControl{ fn reciprocal_model(&self,max_degree:usize)->Result<Option<ReciprocalMode>,Error>{
  let modes=self;let r=modes.r;if ((r-1)/modes.m as u64)%4!=0{return Ok(None)}let divisor=4*modes.m as u64;
  let n=usize::try_from((r-1)/divisor).map_err(|_|Error::Limit)?;if n==0||n>max_degree{return Err(Error::Limit)}
  let mut reader=CoefficientReader{stream:modes.stream(0)?,buffer:Vec::new(),at:0,remaining:n+1};let mut p=polynomial::Poly::new(r).map_err(|_|Error::Domain)?;
  let c=clenshaw_block(&mut p,&mut reader,n,r)?;let center=reader.next()?;assert_eq!(reader.remaining,0);
  let wb=p.mul(&[0,1],&c.offset[0]);let twice:Vec<_>=c.offset[1].iter().map(|&x|mul(2,x,r)).collect();let mut h=p.sub(&wb,&twice);h[0]=(h[0]+center)%r;
  let dn=p.add(&c.matrix[0][0],&c.matrix[1][1]);assert_eq!(h.len(),n+1);assert_eq!(dn.len(),n+1);Ok(Some(ReciprocalMode{r,n,h,dn}))
 }
}
