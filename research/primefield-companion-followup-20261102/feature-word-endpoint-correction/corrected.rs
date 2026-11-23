#[derive(Clone,Copy,Default,Debug)]struct CorrectionStats{endpoints:usize,phase_endpoints:usize,nonzero_weights:usize,powers:usize,muls:usize}
struct CorrectionArithmetic{r:u64,cap:usize,stats:CorrectionStats}
impl CorrectionArithmetic{
 fn mul(&mut self,a:u64,b:u64)->Result<u64,Error>{if self.stats.muls>=self.cap{return Err(Error::Limit)}self.stats.muls+=1;Ok(mul(a,b,self.r))}
 fn power(&mut self,mut a:u64,mut n:u64)->Result<u64,Error>{self.stats.powers+=1;let mut v=1;while n>0{if n&1!=0{v=self.mul(v,a)?}n>>=1;if n>0{a=self.mul(a,a)?}}Ok(v)}
}
fn endpoint_corrections(r:u64,m:usize,zeta:u64,epsilon:u64,words:&[Word],t:usize,cap:usize)->Result<(std::collections::BTreeMap<u64,u64>,[u64;2],CorrectionStats),Error>{
 let mut a=CorrectionArithmetic{r,cap,stats:CorrectionStats::default()};let half=a.power(2,r-2)?;let invt=a.power(t as u64,r-2)?;let mut factors=vec![];let mut endpoints=std::collections::BTreeSet::new();
 for w in words{let mut z=1;for &s in &w.signs{let off=a.mul(w.b,z)?;factors.push((off,s));endpoints.insert(off);z=a.mul(z,zeta)?}}
 a.stats.endpoints=endpoints.len();let mut corrections=std::collections::BTreeMap::new();let mut moments=[0,0];
 for endpoint in endpoints{if a.power(endpoint,(r-1)/2)?!=epsilon{continue}a.stats.phase_endpoints+=1;let mut weight=1;for &(off,s)in &factors{let ch=a.power((endpoint+r-off)%r,(r-1)/2)?;let v=a.mul(s,ch)?;let v=a.mul((1+v)%r,half)?;weight=a.mul(weight,v)?;}if weight==0{continue}a.stats.nonzero_weights+=1;let y=a.power(endpoint,m as u64)?;let v=a.mul(weight,invt)?;let value=(corrections.get(&y).copied().unwrap_or(0)+v)%r;if value==0{corrections.remove(&y);}else{corrections.insert(y,value);}moments[0]=(moments[0]+v)%r;moments[1]=(moments[1]+a.mul(y,v)?)%r;}
 Ok((corrections,moments,a.stats))
}
struct CorrectedProblem{inner:WordProblem,corrections:std::collections::BTreeMap<u64,u64>,correction_moments:[u64;2],correction:CorrectionStats}
impl CorrectedProblem{
 fn membership(&self,y:u64)->Result<u64,Error>{if y==0||y>=self.inner.r||pow(y,self.inner.l as u64,self.inner.r)!=self.inner.epsilon{return Ok(0)}Ok((self.inner.circuit.evaluate(y)?[self.inner.output]+self.inner.r-self.corrections.get(&y).copied().unwrap_or(0))%self.inner.r)}
 fn build(r:u64,m:usize,zeta:u64,epsilon:u64,words:&[Word],cap:usize,correction_cap:usize)->Result<Self,Error>{
  if !valid_domain(r,m,zeta)||(epsilon!=1&&epsilon!=r-1)||words.is_empty()||words.len()>4||words.iter().any(|w|w.b==0||w.b>=r||w.signs.len()!=m||w.signs.iter().any(|&s|s!=1&&s!=r-1)){return Err(Error::Domain)}
  let stabilizer=(0..m).filter(|&t|words.iter().all(|w|(0..m).all(|j|w.signs[j]==w.signs[(j+t)%m]))).count();
  let modes:Vec<_>=words.iter().map(|w|modes(r,m,zeta,w)).collect();let mut g=m;for f in &modes{for(e,&v)in f.iter().enumerate(){if v!=0{g=gcd(g,e)}}}if g!=stabilizer{return Err(Error::Arithmetic)}
  let mut c=Circuit::new(r,cap)?;let mut joint=vec![0;m];joint[0]=1;let mut sections=vec![];let mut cache=std::collections::BTreeMap::new();let half=pow(2,r-2,r);
  for w in words{let a=if let Some(a)=cache.get(&w.b){let a:&Vec<usize>=a;a.clone()}else{let a=c.sections(m,w.b)?;cache.insert(w.b,a.clone());a};
   for(j,&s)in w.signs.iter().enumerate(){let mut filter=vec![0;m];let sid=c.constant(s)?;for e in 0..m{let v=c.product(sid,a[e])?;let v=if e==0{c.add(1,v)?}else{v};let weight=mul(half,pow(zeta,((m-(j*e)%m)%m)as u64,r),r);let weight=c.constant(weight)?;filter[e]=c.product(weight,v)?;}joint=c.section_product(&joint,&filter)?;}
   sections.push(a);
  }
  let scale=c.constant(mul(m as u64,pow(stabilizer as u64,r-2,r),r))?;let output=c.product(scale,joint[0])?;
  let (corrections,correction_moments,correction)=endpoint_corrections(r,m,zeta,epsilon,words,stabilizer,correction_cap)?;
  Ok(Self{inner:WordProblem{circuit:c,output,sections,modes,r,m,l:((r-1)/(2*m as u64))as usize,epsilon,stabilizer},corrections,correction_moments,correction})
 }
 fn phase_at(&self,y:u64)->Result<Option<u64>,Error>{if y==0||y>=self.inner.r||pow(y,self.inner.l as u64,self.inner.r)!=self.inner.epsilon{return Ok(None)}let values=self.inner.circuit.evaluate(y)?;if (values[self.inner.output]+self.inner.r-self.corrections.get(&y).copied().unwrap_or(0))%self.inner.r!=1{return Ok(None)}
  let mut g=self.inner.m;let mut kg=y;for(i,f)in self.inner.modes.iter().enumerate(){for(e,&v)in f.iter().enumerate().skip(1){if v==0{continue}let den=mul(self.inner.m as u64,values[self.inner.sections[i][e]],self.inner.r);if den==0{return Err(Error::Arithmetic)}let ke=mul(v,pow(den,self.inner.r-2,self.inner.r),self.inner.r);let(next,u,v)=egcd(g as i64,e as i64);kg=mul(signed_power(kg,u,self.inner.r),signed_power(ke,v,self.inner.r),self.inner.r);g=next as usize;}}
  if g!=self.inner.stabilizer{return Err(Error::Arithmetic)}if g==1{if pow(kg,self.inner.m as u64,self.inner.r)!=y{return Err(Error::Arithmetic)}Ok(Some(kg))}else{Ok(None)}
 }
 fn extract(&self,caps:Caps)->Result<([u64;2],ProjectorStats),(Error,ProjectorStats)>{let mut ring=PhaseAlgebra::new(self.inner.r,self.inner.l,self.inner.epsilon,caps).map_err(|e|(e,ProjectorStats::default()))?;
  let result=(||{let mut values:Vec<Sparse>=Vec::with_capacity(self.inner.circuit.nodes.len());for &gate in &self.inner.circuit.nodes{let v=match gate{Gate::Constant(c)=>ring.constant(c)?,Gate::Variable=>{let mut v=Sparse::new();ring.put(&mut v,1%self.inner.l,if self.inner.l==1{self.inner.epsilon}else{1})?;v},Gate::Add(a,b)=>ring.add(&values[a],&values[b],false)?,Gate::Sub(a,b)=>ring.add(&values[a],&values[b],true)?,Gate::Mul(a,b)=>ring.product(&values[a],&values[b])?};if v.len()>ring.caps.stored.saturating_sub(ring.stats.stored){return ring.refuse("stored")}ring.stats.stored+=v.len();values.push(v)}let indicator=&values[self.inner.output];let count=mul(self.inner.l as u64,indicator.get(&0).copied().unwrap_or(0),self.inner.r);let sum=mul(mul(self.inner.l as u64,self.inner.epsilon,self.inner.r),indicator.get(&(self.inner.l-1)).copied().unwrap_or(0),self.inner.r);let [c0,c1]=self.correction_moments;let count=(count+self.inner.r-c0)%self.inner.r;let sum=(sum+self.inner.r-c1)%self.inner.r;if count>self.inner.l as u64{return Err(Error::Arithmetic)}Ok([count,sum])})();match result{Ok(v)=>Ok((v,ring.stats)),Err(e)=>Err((e,ring.stats))}
 }
}
struct CorrectedRecovery{result:Recovery,correction:CorrectionStats}
fn recover_corrected(source:&source::Source,curve:group::Curve,base:group::Point,target:Option<group::Point>,r:u64,m:usize,zeta:u64,radii:&[u64],max_calls:usize,node_cap:usize,caps:Caps,correction_cap:usize)->Result<CorrectedRecovery,Error>{
 if !valid_domain(r,m,zeta)||radii.is_empty()||radii.len()>4||radii.iter().any(|&b|b==0||b>=r)||max_calls==0{return Err(Error::Domain)}
 let correction=std::cell::Cell::new(CorrectionStats::default());
 let finish=|scalar,calls,route,pairs|{if let Some(k)=scalar{if multiple(curve,base,k)!=target{return Err(Error::Arithmetic)}}Ok(CorrectedRecovery{result:Recovery{scalar,calls,route,pairs},correction:correction.get()})};
 let encode=|s:i8|if s<0{r-1}else{s as u64};let epsilon=encode(source.sign(target.map(|p|[p.x,p.y])).map_err(|_|Error::Arithmetic)?);let mut calls=1;if epsilon==0{return finish(Some(0),calls,"endpoint",0)}let mut words=vec![];
 for &b in radii{let mut z=1;let mut signs=vec![];for _ in 0..m{if calls>=max_calls{return finish(None,calls,"call-cap",0)}let off=mul(b,z,r);let pt=curve.plus(target,multiple(curve,base,(r-off)%r));let s=encode(source.sign(pt.map(|p|[p.x,p.y])).map_err(|_|Error::Arithmetic)?);calls+=1;if s==0{return finish(Some(off),calls,"endpoint",0)}signs.push(s);z=mul(z,zeta,r)}words.push(Word{b,signs})}
 let problem=match CorrectedProblem::build(r,m,zeta,epsilon,&words,node_cap,correction_cap){Ok(x)=>x,Err(Error::Limit)=>return finish(None,calls,"construction-cap",0),Err(e)=>return Err(e)};
 correction.set(problem.correction);
 let(moment,stats)=match problem.extract(caps){Ok(x)=>x,Err((Error::Limit,s))=>return finish(None,calls,"algebra-cap",s.pairs),Err((e,_))=>return Err(e)};
 if moment[0]==0{return Err(Error::Arithmetic)}if moment[0]==1{if let Some(k)=problem.phase_at(moment[1])?{return finish(Some(k),calls,"corrected-singleton",stats.pairs)}}finish(None,calls,"unresolved",stats.pairs)
}
