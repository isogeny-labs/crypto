// Prime r is a caller-established precondition, as in the inherited circuit.
#[derive(Clone,Debug)]struct Word{b:u64,signs:Vec<u64>}
struct WordProblem{circuit:Circuit,output:usize,sections:Vec<Vec<usize>>,modes:Vec<Vec<u64>>,r:u64,m:usize,l:usize,epsilon:u64,stabilizer:usize}
fn gcd(mut a:usize,mut b:usize)->usize{while b!=0{(a,b)=(b,a%b)}a}
fn egcd(a:i64,b:i64)->(i64,i64,i64){if b==0{(a,1,0)}else{let(g,u,v)=egcd(b,a%b);(g,v,u-(a/b)*v)}}
fn signed_power(a:u64,n:i64,r:u64)->u64{if n<0{pow(pow(a,r-2,r),(-n)as u64,r)}else{pow(a,n as u64,r)}}
fn modes(r:u64,m:usize,zeta:u64,w:&Word)->Vec<u64>{(0..m).map(|e|{let q=pow(zeta,e as u64,r);let mut z=1;let mut f=0;for &s in &w.signs{f=(f+mul(z,s,r))%r;z=mul(z,q,r)}f}).collect()}
fn valid_domain(r:u64,m:usize,zeta:u64)->bool{r>=7&&r<=u64::MAX/2&&m>=3&&m<=64&&((r-1)/2)%m as u64==0&&zeta>0&&zeta<r&&pow(zeta,m as u64,r)==1&&(1..m).all(|j|pow(zeta,j as u64,r)!=1)}
impl WordProblem{
 fn build(r:u64,m:usize,zeta:u64,epsilon:u64,words:&[Word],cap:usize)->Result<Self,Error>{
  if !valid_domain(r,m,zeta)||(epsilon!=1&&epsilon!=r-1)||words.is_empty()||words.len()>4||words.iter().any(|w|w.b==0||w.b>=r||w.signs.len()!=m||w.signs.iter().any(|&s|s!=1&&s!=r-1)){return Err(Error::Domain)}
  let stabilizer=(0..m).filter(|&t|words.iter().all(|w|(0..m).all(|j|w.signs[j]==w.signs[(j+t)%m]))).count();
  let modes:Vec<_>=words.iter().map(|w|modes(r,m,zeta,w)).collect();let mut g=m;for f in &modes{for(e,&v)in f.iter().enumerate(){if v!=0{g=gcd(g,e)}}}if g!=stabilizer{return Err(Error::Arithmetic)}
  let mut c=Circuit::new(r,cap)?;let mut joint=vec![0;m];joint[0]=1;let mut sections=vec![];let mut cache=std::collections::BTreeMap::new();let half=pow(2,r-2,r);
  for w in words{let(a,sq)=if let Some(pair)=cache.get(&w.b){let pair:&(Vec<usize>,Vec<usize>)=pair;pair.clone()}else{let a=c.sections(m,w.b)?;let sq=c.section_product(&a,&a)?;cache.insert(w.b,(a.clone(),sq.clone()));(a,sq)};
   for(j,&s)in w.signs.iter().enumerate(){let mut filter=vec![0;m];let sid=c.constant(s)?;for e in 0..m{let v=c.product(sid,a[e])?;let v=c.add(sq[e],v)?;let weight=mul(half,pow(zeta,((m-(j*e)%m)%m)as u64,r),r);let weight=c.constant(weight)?;filter[e]=c.product(weight,v)?;}joint=c.section_product(&joint,&filter)?;}
   sections.push(a);
  }
  let scale=c.constant(mul(m as u64,pow(stabilizer as u64,r-2,r),r))?;let output=c.product(scale,joint[0])?;
  Ok(Self{circuit:c,output,sections,modes,r,m,l:((r-1)/(2*m as u64))as usize,epsilon,stabilizer})
 }
 fn phase_at(&self,y:u64)->Result<Option<u64>,Error>{if y==0||y>=self.r||pow(y,self.l as u64,self.r)!=self.epsilon{return Ok(None)}let values=self.circuit.evaluate(y)?;if values[self.output]!=1{return Ok(None)}
  let mut g=self.m;let mut kg=y;for(i,f)in self.modes.iter().enumerate(){for(e,&v)in f.iter().enumerate().skip(1){if v==0{continue}let den=mul(self.m as u64,values[self.sections[i][e]],self.r);if den==0{return Err(Error::Arithmetic)}let ke=mul(v,pow(den,self.r-2,self.r),self.r);let(next,u,v)=egcd(g as i64,e as i64);kg=mul(signed_power(kg,u,self.r),signed_power(ke,v,self.r),self.r);g=next as usize;}}
  if g!=self.stabilizer{return Err(Error::Arithmetic)}if g==1{if pow(kg,self.m as u64,self.r)!=y{return Err(Error::Arithmetic)}Ok(Some(kg))}else{Ok(None)}
 }
 fn extract(&self,caps:Caps)->Result<([u64;2],ProjectorStats),(Error,ProjectorStats)>{let mut ring=PhaseAlgebra::new(self.r,self.l,self.epsilon,caps).map_err(|e|(e,ProjectorStats::default()))?;
  let result=(||{let mut values:Vec<Sparse>=Vec::with_capacity(self.circuit.nodes.len());for &gate in &self.circuit.nodes{let v=match gate{Gate::Constant(c)=>ring.constant(c)?,Gate::Variable=>{let mut v=Sparse::new();ring.put(&mut v,1%self.l,if self.l==1{self.epsilon}else{1})?;v},Gate::Add(a,b)=>ring.add(&values[a],&values[b],false)?,Gate::Sub(a,b)=>ring.add(&values[a],&values[b],true)?,Gate::Mul(a,b)=>ring.product(&values[a],&values[b])?};if v.len()>ring.caps.stored.saturating_sub(ring.stats.stored){return ring.refuse("stored")}ring.stats.stored+=v.len();values.push(v)}let indicator=&values[self.output];let count=mul(self.l as u64,indicator.get(&0).copied().unwrap_or(0),self.r);if count>self.l as u64{return Err(Error::Arithmetic)}let sum=mul(mul(self.l as u64,self.epsilon,self.r),indicator.get(&(self.l-1)).copied().unwrap_or(0),self.r);Ok([count,sum])})();match result{Ok(v)=>Ok((v,ring.stats)),Err(e)=>Err((e,ring.stats))}
 }
}
struct Recovery{scalar:Option<u64>,calls:usize,route:&'static str,pairs:usize}
fn recover_word(source:&source::Source,curve:group::Curve,base:group::Point,target:Option<group::Point>,r:u64,m:usize,zeta:u64,radii:&[u64],max_calls:usize,node_cap:usize,caps:Caps)->Result<Recovery,Error>{
 if !valid_domain(r,m,zeta)||radii.is_empty()||radii.len()>4||radii.iter().any(|&b|b==0||b>=r)||max_calls==0{return Err(Error::Domain)}
 let finish=|scalar,calls,route,pairs|{if let Some(k)=scalar{if multiple(curve,base,k)!=target{return Err(Error::Arithmetic)}}Ok(Recovery{scalar,calls,route,pairs})};
 let encode=|s:i8|if s<0{r-1}else{s as u64};let epsilon=encode(source.sign(target.map(|p|[p.x,p.y])).map_err(|_|Error::Arithmetic)?);let mut calls=1;if epsilon==0{return finish(Some(0),calls,"endpoint",0)}let mut words=vec![];
 for &b in radii{let mut z=1;let mut signs=vec![];for _ in 0..m{if calls>=max_calls{return finish(None,calls,"call-cap",0)}let off=mul(b,z,r);let pt=curve.plus(target,multiple(curve,base,(r-off)%r));let s=encode(source.sign(pt.map(|p|[p.x,p.y])).map_err(|_|Error::Arithmetic)?);calls+=1;if s==0{return finish(Some(off),calls,"endpoint",0)}signs.push(s);z=mul(z,zeta,r)}words.push(Word{b,signs})}
 let problem=match WordProblem::build(r,m,zeta,epsilon,&words,node_cap){Ok(x)=>x,Err(Error::Limit)=>return finish(None,calls,"node-cap",0),Err(e)=>return Err(e)};
 let(moment,stats)=match problem.extract(caps){Ok(x)=>x,Err((Error::Limit,s))=>return finish(None,calls,"algebra-cap",s.pairs),Err((e,_))=>return Err(e)};
 if moment[0]==0{return Err(Error::Arithmetic)}if moment[0]==1{if let Some(k)=problem.phase_at(moment[1])?{return finish(Some(k),calls,"word-singleton",stats.pairs)}}finish(None,calls,"unresolved",stats.pairs)
}
