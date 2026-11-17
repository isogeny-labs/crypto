type Sparse=std::collections::BTreeMap<usize,u64>;
#[derive(Clone,Copy)]struct Caps{terms:usize,stored:usize,pairs:usize,output:usize}
#[derive(Default,Debug)]struct ProjectorStats{pairs:usize,updates:usize,stored:usize,max_terms:usize,limit:Option<&'static str>}
struct PhaseAlgebra{r:u64,l:usize,epsilon:u64,caps:Caps,stats:ProjectorStats}
impl PhaseAlgebra{
 fn new(r:u64,l:usize,epsilon:u64,caps:Caps)->Result<Self,Error>{if r<7||r>u64::MAX/2||l==0||l as u64>=r||(r-1)%l as u64!=0||(epsilon!=1&&epsilon!=r-1)||pow(epsilon,(r-1)/l as u64,r)!=1{return Err(Error::Domain)}Ok(Self{r,l,epsilon,caps,stats:ProjectorStats::default()})}
 fn refuse<T>(&mut self,why:&'static str)->Result<T,Error>{self.stats.limit=Some(why);Err(Error::Limit)}
 fn put(&mut self,out:&mut Sparse,e:usize,v:u64)->Result<(),Error>{if v==0{return Ok(())}self.stats.updates=self.stats.updates.checked_add(1).ok_or(Error::Limit)?;let value=(out.get(&e).copied().unwrap_or(0)+v)%self.r;if value==0{out.remove(&e);}else{if !out.contains_key(&e)&&out.len()>=self.caps.terms{return self.refuse("terms")}out.insert(e,value);self.stats.max_terms=self.stats.max_terms.max(out.len());}Ok(())}
 fn constant(&mut self,c:u64)->Result<Sparse,Error>{let mut out=Sparse::new();self.put(&mut out,0,c%self.r)?;Ok(out)}
 fn add(&mut self,a:&Sparse,b:&Sparse,subtract:bool)->Result<Sparse,Error>{let mut out=a.clone();for(&e,&c)in b{self.put(&mut out,e,if subtract{(self.r-c)%self.r}else{c})?;}Ok(out)}
 fn product(&mut self,a:&Sparse,b:&Sparse)->Result<Sparse,Error>{let count=a.len().checked_mul(b.len()).ok_or(Error::Limit)?;if count>self.caps.pairs.saturating_sub(self.stats.pairs){return self.refuse("pairs")}let mut out=Sparse::new();for(&i,&x)in a{for(&j,&y)in b{self.stats.pairs+=1;let e=i.checked_add(j).ok_or(Error::Limit)?;let mut v=mul(x,y,self.r);if e>=self.l{v=mul(v,self.epsilon,self.r)}self.put(&mut out,e%self.l,v)?;}}Ok(out)}
 fn power(&mut self,a:&Sparse,mut n:u64)->Result<Sparse,Error>{let mut out=self.constant(1)?;let mut base=a.clone();while n>0{if n&1!=0{out=self.product(&out,&base)?}n>>=1;if n>0{base=self.product(&base,&base)?}}Ok(out)}
 fn projector(&mut self,sys:&ImplicitTranscript)->Result<Sparse,Error>{if sys.circuit.r!=self.r{return Err(Error::Domain)}let mut values:Vec<Sparse>=Vec::with_capacity(sys.circuit.nodes.len());
  for &gate in &sys.circuit.nodes{let v=match gate{Gate::Constant(c)=>self.constant(c)?,Gate::Variable=>{let mut v=Sparse::new();self.put(&mut v,1%self.l,if self.l==1{self.epsilon}else{1})?;v},Gate::Add(a,b)=>self.add(&values[a],&values[b],false)?,Gate::Sub(a,b)=>self.add(&values[a],&values[b],true)?,Gate::Mul(a,b)=>self.product(&values[a],&values[b])?};
   if v.len()>self.caps.stored.saturating_sub(self.stats.stored){return self.refuse("stored")}self.stats.stored+=v.len();values.push(v);
  }
  let one=self.constant(1)?;let mut indicator=one.clone();for &e in &sys.equations{if values[e].is_empty(){continue}let nonzero=self.power(&values[e],self.r-1)?;let keep=self.add(&one,&nonzero,true)?;indicator=self.product(&indicator,&keep)?;if indicator.is_empty(){break}}
  Ok(indicator)
 }
 fn annihilator(&mut self,indicator:&Sparse)->Result<Vec<u64>,Error>{let count=mul(self.l as u64,indicator.get(&0).copied().unwrap_or(0),self.r)as usize;if count>self.l{return Err(Error::Arithmetic)}if count>self.caps.output{return self.refuse("output")}
  let mut sums=vec![0;count+1];for j in 1..=count{let e=if j==self.l{0}else{self.l-j};sums[j]=mul(mul(self.l as u64,self.epsilon,self.r),indicator.get(&e).copied().unwrap_or(0),self.r);}
  // Descending monic coefficients, Newton identities: j*a_j = -sum a_(j-i)*p_i.
  let mut a=vec![1];for j in 1..=count{let mut s=0;for i in 1..=j{s=(s+mul(a[j-i],sums[i],self.r))%self.r}a.push(mul((self.r-s)%self.r,pow(j as u64,self.r-2,self.r),self.r));}a.reverse();Ok(a)
 }
}
struct ProjectorProblem{sys:ImplicitTranscript,l:usize,epsilon:u64}
impl ProjectorProblem{
 fn build(r:u64,m:usize,sign:u64,obs:&[Observation],node_cap:usize)->Result<Self,Error>{let sys=ImplicitTranscript::build(r,m,sign,obs,node_cap)?;Ok(Self{sys,l:((r-1)/(2*m as u64))as usize,epsilon:sign})}
 fn extract(&self,caps:Caps)->Result<(Vec<u64>,Sparse,ProjectorStats),(Error,ProjectorStats)>{let mut ring=PhaseAlgebra::new(self.sys.circuit.r,self.l,self.epsilon,caps).map_err(|e|(e,ProjectorStats::default()))?;let result=(||{let indicator=ring.projector(&self.sys)?;let q=ring.annihilator(&indicator)?;Ok((q,indicator))})();match result{Ok((q,i))=>Ok((q,i,ring.stats)),Err(e)=>Err((e,ring.stats))}}
}
