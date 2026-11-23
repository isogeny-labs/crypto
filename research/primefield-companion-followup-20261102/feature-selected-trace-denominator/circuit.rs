#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord)]enum Gate{Constant(u64),Variable,Add(usize,usize),Sub(usize,usize),Mul(usize,usize)}
struct Circuit{r:u64,nodes:Vec<Gate>,intern:std::collections::BTreeMap<Gate,usize>,cap:usize}
impl Circuit{
 fn new(r:u64,cap:usize)->Result<Self,Error>{if r<7||r>u64::MAX/2{return Err(Error::Domain)}if cap<3{return Err(Error::Limit)}let mut s=Self{r,nodes:vec![],intern:std::collections::BTreeMap::new(),cap};s.put(Gate::Constant(0))?;s.put(Gate::Constant(1))?;s.put(Gate::Variable)?;Ok(s)}
 fn put(&mut self,g:Gate)->Result<usize,Error>{if let Some(&i)=self.intern.get(&g){return Ok(i)}if self.nodes.len()>=self.cap{return Err(Error::Limit)}let i=self.nodes.len();self.nodes.push(g);self.intern.insert(g,i);Ok(i)}
 fn constant(&mut self,v:u64)->Result<usize,Error>{self.put(Gate::Constant(v%self.r))}
 fn add(&mut self,a:usize,b:usize)->Result<usize,Error>{if a==0{return Ok(b)}if b==0{return Ok(a)}if let(Gate::Constant(x),Gate::Constant(y))=(self.nodes[a],self.nodes[b]){return self.constant((x+y)%self.r)}self.put(Gate::Add(a.min(b),a.max(b)))}
 fn sub(&mut self,a:usize,b:usize)->Result<usize,Error>{if b==0{return Ok(a)}if a==b{return Ok(0)}if let(Gate::Constant(x),Gate::Constant(y))=(self.nodes[a],self.nodes[b]){return self.constant((x+self.r-y)%self.r)}self.put(Gate::Sub(a,b))}
 fn product(&mut self,a:usize,b:usize)->Result<usize,Error>{if a==0||b==0{return Ok(0)}if a==1{return Ok(b)}if b==1{return Ok(a)}if let(Gate::Constant(x),Gate::Constant(y))=(self.nodes[a],self.nodes[b]){return self.constant(mul(x,y,self.r))}self.put(Gate::Mul(a.min(b),a.max(b)))}
 fn power(&mut self,mut a:usize,mut n:u64)->Result<usize,Error>{let mut v=1;while n>0{if n&1!=0{v=self.product(v,a)?}n>>=1;if n>0{a=self.product(a,a)?}}Ok(v)}
 fn section_product(&mut self,a:&[usize],b:&[usize])->Result<Vec<usize>,Error>{let m=a.len();let mut out=vec![0;m];for i in 0..m{for j in 0..m{let mut v=self.product(a[i],b[j])?;if i+j>=m{v=self.product(v,2)?}out[(i+j)%m]=self.add(out[(i+j)%m],v)?;}}Ok(out)}
 fn sections(&mut self,m:usize,b:u64)->Result<Vec<usize>,Error>{let mut a=vec![0;m];a[0]=1;let mut base=vec![0;m];base[0]=self.constant(self.r-b)?;base[1]=1;let mut h=(self.r-1)/2;
  while h>0{if h&1!=0{a=self.section_product(&a,&base)?}h>>=1;if h>0{base=self.section_product(&base,&base)?}}Ok(a)
 }
 fn evaluate(&self,y:u64)->Result<Vec<u64>,Error>{if y>=self.r{return Err(Error::Domain)}let mut values=Vec::with_capacity(self.nodes.len());for &node in &self.nodes{let v=match node{Gate::Constant(c)=>c,Gate::Variable=>y,Gate::Add(a,b)=>(values[a]+values[b])%self.r,Gate::Sub(a,b)=>(values[a]+self.r-values[b])%self.r,Gate::Mul(a,b)=>mul(values[a],values[b],self.r)};values.push(v)}Ok(values)}
 // Caller-supplied modulus is a specialization, never a recovered annihilator.
 fn modulo(&self,q:&[u64],word_cap:usize)->Result<Vec<Vec<u64>>,Error>{if q.len()<2||q.last()!=Some(&1)||q.iter().any(|&a|a>=self.r){return Err(Error::Domain)}let d=q.len()-1;if self.nodes.len().checked_mul(d).ok_or(Error::Limit)?>word_cap{return Err(Error::Limit)}
  let mut p=polynomial::Poly::new(self.r).map_err(|_|Error::Domain)?;let mut values:Vec<Vec<u64>>=Vec::with_capacity(self.nodes.len());
  for &node in &self.nodes{let v=match node{Gate::Constant(c)=>vec![c],Gate::Variable=>p.divrem(&[0,1],q).map_err(|_|Error::Arithmetic)?.1,Gate::Add(a,b)=>p.add(&values[a],&values[b]),Gate::Sub(a,b)=>p.sub(&values[a],&values[b]),Gate::Mul(a,b)=>{let z=p.mul(&values[a],&values[b]);p.divrem(&z,q).map_err(|_|Error::Arithmetic)?.1}};values.push(v)}Ok(values)
 }
}
