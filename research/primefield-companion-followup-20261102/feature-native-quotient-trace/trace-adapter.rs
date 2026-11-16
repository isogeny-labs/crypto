impl Modes{
 fn recover_traced(&self,source:&source::Source,curve:group::Curve,base:group::Point,target:Option<group::Point>,radii:&[u64],cap:usize,extra:&[u64],max_calls:usize)->Result<(Option<u64>,PullStats,&'static str),Error>{
  if radii.len()>4||radii.iter().any(|&b|b==0||b>=self.r)||extra.len()>32||extra.iter().any(|&e|e>=self.r)||max_calls==0{return Err(Error::Domain)}
  let r=self.r;let s=source.sign(target.map(|p|[p.x,p.y])).map_err(|_|Error::Domain)?;let mut stats=PullStats{calls:1,..Default::default()};
  let finish=|k,stats,route|{if multiple(curve,base,k)!=target{Err(Error::Arithmetic)}else{Ok((Some(k),stats,route))}};
  if s==0{return finish(0,stats,"identity")}
  let mut history=vec![(0,if s<0{r-1}else{s as u64})];let d=((r-1)/self.m as u64)as usize;let mut state=vec![0;d+2];state[1]=r-1;state[d+1]=1;let mut p=polynomial::Poly::new(r).unwrap();
  for &b in radii{let mut zeta=1;let mut signs=Vec::new();for _ in 0..self.m{if stats.calls>=max_calls{return Ok((None,stats,"call-cap"))}let offset=mul(b,zeta,r);let q=curve.plus(target,multiple(curve,base,(r-offset)%r));let s=source.sign(q.map(|p|[p.x,p.y])).map_err(|_|Error::Arithmetic)?;stats.calls+=1;
    if s==0{return finish(offset,stats,"endpoint")}let sign=if s<0{r-1}else{s as u64};signs.push(sign);history.push(((r-offset)%r,sign));zeta=mul(zeta,self.zeta,r);}
   let f=self.observed(&signs)?;for constraint in self.constraints(b,&f){state=p.gcd_monic(&state,&constraint,d+1).map_err(|_|Error::Arithmetic)?.polynomial;}
   if state.len()==1{return Err(Error::Arithmetic)}
   if state.len()==2{let image=(r-state[0])%r;let v=mul(image,pow(pow(b,self.m as u64,r),r-2,r),r);
    if let Some(x)=self.phase(v,b,f[1])?{return finish(mul(b,x,r),stats,"mode-phase")}
   }
   let mut fiber=match pullback(&state,self.m,r,cap){Ok(f)=>f,Err(Error::Limit)=>continue,Err(e)=>return Err(e)};
   stats.lifted=fiber.len()-1;stats.image_count=state.len()-1;
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
