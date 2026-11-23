#[derive(Clone,Copy,Debug,PartialEq)]struct Observation{b:u64,f:[u64;3]}
struct ImplicitTranscript{circuit:Circuit,equations:Vec<usize>,phase:Option<(usize,usize)>,anchor_kind:&'static str}
impl ImplicitTranscript{
 fn build(r:u64,m:usize,sign:u64,obs:&[Observation],node_cap:usize)->Result<Self,Error>{
  if r<7||r>u64::MAX/2||m<3||m>64||m as u64>=r||((r-1)/2)%m as u64!=0||(sign!=1&&sign!=r-1)||obs.is_empty()||obs.len()>4||obs.iter().any(|o|o.b==0||o.b>=r||o.f.iter().any(|&v|v>=r)){return Err(Error::Domain)}
  let mut c=Circuit::new(r,node_cap)?;let l=(r-1)/(2*m as u64);let phase=c.power(2,l)?;let s=c.constant(sign)?;let mut eq=vec![c.sub(phase,s)?];let mc=c.constant(m as u64)?;
  let mut cache=std::collections::BTreeMap::new();let mut sections=Vec::new();for o in obs{if !cache.contains_key(&o.b){cache.insert(o.b,c.sections(m,o.b)?);}sections.push(cache[&o.b].clone());}
  let (anchor,kind)=if let Some(a)=obs.iter().position(|o|o.f[1]!=0){(Some(a),"plus")}else if let Some(a)=obs.iter().position(|o|o.f[2]!=0){(Some(a),"minus")}else{(None,"none")};
  let phase=if let Some(a)=anchor{let(n,d)=if kind=="plus"{(c.constant(obs[a].f[1])?,c.product(mc,sections[a][1])?)}else{let ya=c.product(2,sections[a][m-1])?;(c.product(mc,ya)?,c.constant(obs[a].f[2])?)};
   let nm=c.power(n,m as u64)?;let dm=c.power(d,m as u64)?;let yd=c.product(2,dm)?;eq.push(c.sub(nm,yd)?);
   let nlast=c.power(n,(m-1)as u64)?;let dlast=c.power(d,(m-1)as u64)?;
   for (o,a) in obs.iter().zip(&sections){let lhs0=c.product(mc,a[0])?;let f0=c.constant(o.f[0])?;eq.push(c.sub(lhs0,f0)?);
    let np=c.product(n,a[1])?;let lhs=c.product(mc,np)?;let fp=c.constant(o.f[1])?;let rhs=c.product(fp,d)?;eq.push(c.sub(lhs,rhs)?);
    let np=c.product(nlast,a[m-1])?;let lhs=c.product(mc,np)?;let fm=c.constant(o.f[2])?;let rhs=c.product(fm,dlast)?;eq.push(c.sub(lhs,rhs)?);
   }Some((n,d))
  }else{for (o,a) in obs.iter().zip(&sections){let lhs=c.product(mc,a[0])?;let rhs=c.constant(o.f[0])?;eq.push(c.sub(lhs,rhs)?);eq.push(a[1]);eq.push(a[m-1]);}None};
  Ok(Self{circuit:c,equations:eq,phase,anchor_kind:kind})
 }
 fn accepts_values(&self,v:&[u64])->bool{self.equations.iter().all(|&i|v[i]==0)}
 fn phase_at(&self,y:u64)->Result<Option<u64>,Error>{let v=self.circuit.evaluate(y)?;if !self.accepts_values(&v){return Ok(None)}if let Some((n,d))=self.phase{if v[d]==0{return Err(Error::Arithmetic)}return Ok(Some(mul(v[n],pow(v[d],self.circuit.r-2,self.circuit.r),self.circuit.r)))}Ok(None)}
}
