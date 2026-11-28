impl RecoveryModes<'_>{
 fn shared_rescue_phase(&self,b:u64,plus:&[u64],minus:&[u64],i:u64,cap:usize)->Result<Option<u64>,Error>{
  let r=self.r;let m=self.m;if m<3||m%2==0||(r-1)%(8*m as u64)!=0||b==0||b>=r{return Err(Error::Domain)}
  let f=doubled_observation(r,plus,minus)?;doubled_count(0,1);
  let mm=2*m;if cap<mm{doubled_count(2,1);return Ok(None)}
  let eta=(r-pow(self.zeta,((m+1)/2)as u64,r))%r;
  let stream=StreamModes::new(r,mm,eta,mm,((r-1)/2/mm as u64+1)as usize,self.block)?;
  let doubled=RecoveryModes{r:stream.r,m:stream.m,zeta:stream.zeta,block:stream.block,products:self.products.for_doubled(&stream)?};doubled_count(1,1);
  let mut image=DirectImage::new(&doubled,b,&f,i)?;doubled_count(6,((r-1)/(4*mm as u64))as usize+1);
  let g=match image.materialize(cap,cap/mm){Ok(g)=>g,Err(Error::Limit)=>{doubled_count(2,1);return Ok(None)},Err(e)=>return Err(e)};
  if g.len()!=2{doubled_count(3,1);return Ok(None)}
  let z=(r-g[0])%r;let v=mul(z,pow(pow(b,mm as u64,r),r-2,r),r);
  match doubled.phase(v,b,f[1])?{Some(x)=>{doubled_count(5,1);Ok(Some(mul(b,x,r)))},None=>{doubled_count(4,1);Ok(None)}}
 }
}
