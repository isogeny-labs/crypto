impl RecoveryModes{
 fn recover_direct(&self,source:&source::Source,curve:group::Curve,base:group::Point,target:Option<group::Point>,radii:&[u64],cap:usize,extra:&[u64],max_calls:usize)->Result<(Option<u64>,PullStats,&'static str),Error>{
  if radii.len()>4||radii.iter().any(|&b|b==0||b>=self.r)||extra.len()>32||extra.iter().any(|&e|e>=self.r)||max_calls==0{return Err(Error::Domain)}
  if self.m<3||((self.r-1)/self.m as u64)%4!=0{return self.recover_traced(source,curve,base,target,radii,cap,extra,max_calls)}
  let mut fourth=None;for z in 2..self.r.min(66){let i=pow(z,(self.r-1)/4,self.r);if mul(i,i,self.r)==self.r-1{fourth=Some(i);break}}
  let Some(i)=fourth else{return self.recover_traced(source,curve,base,target,radii,cap,extra,max_calls)};
  let mut compressed:Option<DirectImage>=None;
  let r=self.r;let s=source.sign(target.map(|p|[p.x,p.y])).map_err(|_|Error::Domain)?;let mut stats=PullStats{calls:1,..Default::default()};
  let finish=|k,stats,route|{if multiple(curve,base,k)!=target{Err(Error::Arithmetic)}else{Ok((Some(k),stats,route))}};
  if s==0{return finish(0,stats,"identity")}
  let mut history=vec![(0,if s<0{r-1}else{s as u64})];let mut p=polynomial::Poly::new(r).unwrap();
  for &b in radii{let mut zeta=1;let mut signs=Vec::new();for _ in 0..self.m{if stats.calls>=max_calls{return Ok((None,stats,"call-cap"))}let offset=mul(b,zeta,r);let q=curve.plus(target,multiple(curve,base,(r-offset)%r));let s=source.sign(q.map(|p|[p.x,p.y])).map_err(|_|Error::Arithmetic)?;stats.calls+=1;
    if s==0{return finish(offset,stats,"endpoint")}let sign=if s<0{r-1}else{s as u64};signs.push(sign);history.push(((r-offset)%r,sign));zeta=mul(zeta,self.zeta,r);}
   let f=self.observed(&signs)?;
   if let Some(image)=&mut compressed{image.update(self,b,&f)?;}else{compressed=Some(DirectImage::new(self,b,&f,i)?);}
   let dense=match compressed.as_mut().unwrap().materialize(cap,cap/self.m){Ok(g)=>g,Err(Error::Limit)=>{DIRECT_COUNTS.with(|c|{let(a,b,d,e)=c.get();c.set((a,b,d+1,e))});continue},Err(e)=>return Err(e)};
   DIRECT_COUNTS.with(|c|{let(a,b,d,e)=c.get();c.set((a,b,d,e+1))});
   let mut state=ImageDomain::new(r,self.m)?;state.intersect(&mut p,&dense)?;
   if state.degree()==0{return Err(Error::Arithmetic)}
   if let Some(image)=state.linear_root(){let v=mul(image,pow(pow(b,self.m as u64,r),r-2,r),r);
    if let Some(x)=self.phase(v,b,f[1])?{return finish(mul(b,x,r),stats,"mode-phase")}
   }
   let mut fiber=match state.pullback(cap){Ok(f)=>f,Err(Error::Limit)=>continue,Err(e)=>return Err(e)};
   stats.lifted=fiber.len()-1;stats.image_count=state.degree();
   let (next,powers)=trace_history(&mut p,&fiber,&history,self.m,self.zeta,r,cap)?;fiber=next;stats.replayed=history.len(); ORBIT_POWERS.with(|n|n.set(n.get()+powers));
   drop(state);drop(history);
   if fiber.len()==1{return Err(Error::Arithmetic)}if fiber.len()==2{return finish((r-fiber[0])%r,stats,"retained-set")}
   for &offset in extra{if stats.calls>=max_calls{return Ok((None,stats,"call-cap"))}let q=curve.plus(target,multiple(curve,base,offset));let s=source.sign(q.map(|p|[p.x,p.y])).map_err(|_|Error::Arithmetic)?;stats.calls+=1;stats.extra+=1;
    if s==0{return finish((r-offset)%r,stats,"extra-endpoint")}fiber=replay(&mut p,&fiber,offset,if s<0{r-1}else{s as u64},r,cap)?;
    if fiber.len()==1{return Err(Error::Arithmetic)}if fiber.len()==2{return finish((r-fiber[0])%r,stats,"extra-sign")}
   }return Ok((None,stats,"fiber-exhausted"))
  }Ok((None,stats,"image-exhausted"))
 }
}

impl StreamModes{fn recover_direct(&self,source:&source::Source,curve:group::Curve,base:group::Point,target:Option<group::Point>,radii:&[u64],cap:usize,extra:&[u64],max_calls:usize)->Result<(Option<u64>,PullStats,&'static str),Error>{RecoveryModes::from_modes(self)?.recover_direct(source,curve,base,target,radii,cap,extra,max_calls)}}
