impl QuadRing{
 // Returns product and the two oriented factors from the existing paired stream.
 fn oriented_pair_scaled(&mut self,modes:&RecoveryModes,scale:u64)->Result<(Quad,Quad,Quad),Error>{
  let r=self.r;if modes.r!=r||modes.m<3||((r-1)/2)%modes.m as u64!=0||scale==0||scale>=r{return Err(Error::Domain)}
  let length=((r-1)/2/modes.m as u64)as usize;
  if scale==1{let f=self.mode(modes,1)?;let norm=self.norm(&f)?;let wb=self.p.mul(&[0,1],&f.0[1]);let a=self.p.add(&f.0[0],&wb);let minus:Vec<_>=f.0[1].iter().map(|&v|(r-v)%r).collect();let other=Quad([self.rem(&a)?,self.rem(&minus)?]);SIGNED_TRANSPORT.with(|c|{let(a,b,n)=c.get();c.set((a+1,b,n+length))});return Ok((Quad([norm,vec![0]]),f,other))}
  let inverse=pow(scale,r-2,r);if mul(scale,inverse,r)!=1{return Err(Error::Arithmetic)}
  let bases=[Quad([vec![0],vec![scale]]),Quad([vec![0,inverse],vec![r-inverse]])];let stream=modes.stream(1)?;if stream.left!=length{return Err(Error::Arithmetic)}
  let mut reader=CoefficientReader{stream,buffer:Vec::new(),at:0,remaining:length};let(values,_)=self.signed_pair_block(&mut reader,length,&bases)?;assert_eq!(reader.remaining,0);
  let product=self.multiply(&values[0],&values[1])?;SIGNED_TRANSPORT.with(|c|{let(a,b,n)=c.get();c.set((a,b+1,n+length))});let[first,second]=values;Ok((product,first,second))
 }
}
