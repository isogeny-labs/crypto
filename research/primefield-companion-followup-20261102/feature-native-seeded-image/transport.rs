std::thread_local!{static SIGNED_TRANSPORT:std::cell::Cell<(usize,usize,usize)>=const{std::cell::Cell::new((0,0,0))};}
impl QuadRing{
 fn signed_pair_block(&mut self,reader:&mut CoefficientReader<'_>,length:usize,bases:&[Quad;2])->Result<([Quad;2],[Quad;2]),Error>{
  if length==0{return Err(Error::Domain)}
  if length==1{let c=Quad([vec![reader.next()?],vec![0]]);return Ok(([c.clone(),c],bases.clone()))}
  let(a,ap)=self.signed_pair_block(reader,length/2,bases)?;
  let(b,bp)=self.signed_pair_block(reader,length-length/2,bases)?;
  let mut values=[Quad([vec![0],vec![0]]),Quad([vec![0],vec![0]])];let mut powers=values.clone();
  for j in 0..2{let ab=self.multiply(&a[j],&bp[j])?;values[j]=self.add(&ab,&b[j]);powers[j]=self.multiply(&ap[j],&bp[j])?;}
  Ok((values,powers))
 }
 fn signed_product_scaled(&mut self,modes:&RecoveryModes,scale:u64)->Result<Quad,Error>{
  let r=self.r;if modes.r!=r||modes.m<3||((r-1)/2)%modes.m as u64!=0||scale==0||scale>=r{return Err(Error::Domain)}
  let length=((r-1)/2/modes.m as u64)as usize;
  if scale==1{let f=self.mode(modes,1)?;let norm=self.norm(&f)?;SIGNED_TRANSPORT.with(|c|{let(a,b,n)=c.get();c.set((a+1,b,n+length))});return Ok(Quad([norm,vec![0]]))}
  let inverse=pow(scale,r-2,r);if mul(scale,inverse,r)!=1{return Err(Error::Arithmetic)}
  // V^-1 = W - V. The second base is scale^-1*(W-V), not scale*(W-V).
  let bases=[Quad([vec![0],vec![scale]]),Quad([vec![0,inverse],vec![r-inverse]])];
  let stream=modes.stream(1)?;if stream.left!=length{return Err(Error::Arithmetic)}
  let mut reader=CoefficientReader{stream,buffer:Vec::new(),at:0,remaining:length};
  let(values,_)=self.signed_pair_block(&mut reader,length,&bases)?;assert_eq!(reader.remaining,0);
  let product=self.multiply(&values[0],&values[1])?;
  SIGNED_TRANSPORT.with(|c|{let(a,b,n)=c.get();c.set((a,b+1,n+length))});Ok(product)
 }
 fn signed_equation_scaled(&mut self,modes:&RecoveryModes,scale:u64,f:[u64;2],epsilon:u64)->Result<Quad,Error>{
  let r=self.r;if f.iter().any(|&x|x>=r)||(epsilon!=1&&epsilon!=r-1){return Err(Error::Domain)}
  let product=self.signed_product_scaled(modes,scale)?;let sigma=if (r-1)/2%2==0{1}else{r-1};
  let factor=mul(mul(modes.m as u64,modes.m as u64,r),mul(sigma,epsilon,r),r);
  let mut equation=self.scale(&product,factor);equation.0[0][0]=(equation.0[0][0]+r-mul(f[0],f[1],r))%r;Ok(equation)
 }
}
