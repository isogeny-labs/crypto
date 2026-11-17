#[derive(Default,Debug)]struct PairingStats{probes:usize,products:usize,wrapped_products:usize}
impl PhaseAlgebra{
 fn selected_product(&mut self,a:&Sparse,b:&Sparse,probe_cap:usize,stats:&mut PairingStats)->Result<[u64;2],Error>{
  if a.iter().chain(b.iter()).any(|(&e,&v)|e>=self.l||v==0||v>=self.r){return Err(Error::Domain)}
  let(a,b)=if a.len()<=b.len(){(a,b)}else{(b,a)};let targets=if self.l==1{1}else{2};let needed=a.len().checked_mul(targets).ok_or(Error::Limit)?;
  if needed>probe_cap.saturating_sub(stats.probes){return self.refuse("probes")}
  let mut coefficients=[0;2];for t in 0..targets{let j=if t==0{0}else{self.l-1};for(&i,&x)in a{stats.probes+=1;let opposite=if j>=i{j-i}else{self.l-(i-j)};
   if let Some(&y)=b.get(&opposite){if self.stats.pairs>=self.caps.pairs{return self.refuse("pairs")}self.stats.pairs+=1;stats.products+=1;let mut c=mul(x,y,self.r);if i>j{c=mul(c,self.epsilon,self.r);stats.wrapped_products+=1;}coefficients[t]=(coefficients[t]+c)%self.r;}
  }}
  let count=mul(self.l as u64,coefficients[0],self.r);let sum=if self.l==1{mul(self.epsilon,count,self.r)}else{mul(mul(self.l as u64,self.epsilon,self.r),coefficients[1],self.r)};Ok([count,sum])
 }
 fn selected_projector(&mut self,sys:&ImplicitTranscript,probe_cap:usize,pairing:&mut PairingStats)->Result<[u64;2],Error>{
  if sys.circuit.r!=self.r{return Err(Error::Domain)}let mut values:Vec<Sparse>=Vec::with_capacity(sys.circuit.nodes.len());
  for &gate in &sys.circuit.nodes{let v=match gate{Gate::Constant(c)=>self.constant(c)?,Gate::Variable=>{let mut v=Sparse::new();self.put(&mut v,1%self.l,if self.l==1{self.epsilon}else{1})?;v},Gate::Add(a,b)=>self.add(&values[a],&values[b],false)?,Gate::Sub(a,b)=>self.add(&values[a],&values[b],true)?,Gate::Mul(a,b)=>self.product(&values[a],&values[b])?};
   if v.len()>self.caps.stored.saturating_sub(self.stats.stored){return self.refuse("stored")}self.stats.stored+=v.len();values.push(v);
  }
  let active:Vec<_>=sys.equations.iter().copied().filter(|&e|!values[e].is_empty()).collect();let one=self.constant(1)?;let mut prefix=one.clone();
  for (i,&e) in active.iter().enumerate(){let nonzero=self.power(&values[e],self.r-1)?;let keep=self.add(&one,&nonzero,true)?;
   if i+1==active.len(){return self.selected_product(&prefix,&keep,probe_cap,pairing)}
   prefix=self.product(&prefix,&keep)?;if prefix.is_empty(){return Ok([0,0])}
  }
  self.selected_product(&one,&one,probe_cap,pairing)
 }
}
impl ProjectorProblem{
 fn selected(&self,caps:Caps,probe_cap:usize)->Result<([u64;2],ProjectorStats,PairingStats),(Error,ProjectorStats,PairingStats)>{let mut pairing=PairingStats::default();let mut ring=PhaseAlgebra::new(self.sys.circuit.r,self.l,self.epsilon,caps).map_err(|e|(e,ProjectorStats::default(),PairingStats::default()))?;
  let result=ring.selected_projector(&self.sys,probe_cap,&mut pairing);match result{Ok(v)=>{if v[0]>self.l as u64{return Err((Error::Arithmetic,ring.stats,pairing))}Ok((v,ring.stats,pairing))},Err(e)=>Err((e,ring.stats,pairing))}
 }
}
