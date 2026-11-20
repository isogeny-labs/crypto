/// Roots are zero and the m-th-power image in F_r. Prime r is caller-certified.
struct ImageDomain{r:u64,m:usize,degree:usize,dense:Option<Vec<u64>>}
impl ImageDomain{
 fn new(r:u64,m:usize)->Result<Self,Error>{
  if r<7||r>u64::MAX/2||m<2||m as u64>=r||(r-1)%(m as u64)!=0{return Err(Error::Domain)}
  let degree=usize::try_from((r-1)/m as u64+1).map_err(|_|Error::Limit)?;
  Ok(Self{r,m,degree,dense:None})
 }
 fn degree(&self)->usize{self.dense.as_ref().map_or(self.degree,|x|x.len()-1)}
 fn linear_root(&self)->Option<u64>{self.dense.as_ref().filter(|x|x.len()==2).map(|x|(self.r-x[0])%self.r)}
 fn intersect(&mut self,p:&mut polynomial::Poly,c:&[u64])->Result<(),Error>{
  if c.is_empty()||c.iter().any(|&x|x>=self.r){return Err(Error::Domain)}
  // Match the original raw input-degree cap before copying/trimming.
  if c.len()-1>self.degree{return Err(Error::Arithmetic)}
  if let Some(g)=&self.dense{self.dense=Some(p.gcd_monic(g,c,self.degree).map_err(|_|Error::Arithmetic)?.polynomial);return Ok(())}
  let end=c.iter().rposition(|&x|x!=0);let Some(end)=end else{return Ok(())};
  if end==0{self.dense=Some(vec![1]);return Ok(())}
  let c=&c[..=end];let x=p.divrem(&[0,1],c).map_err(|_|Error::Arithmetic)?.1;
  let(mut base,mut acc,mut exponent)=(x.clone(),vec![1],self.degree);
  while exponent!=0{
   if exponent&1!=0{let product=p.mul(&acc,&base);acc=p.divrem(&product,c).map_err(|_|Error::Arithmetic)?.1;}
   exponent>>=1;
   if exponent!=0{let product=p.mul(&base,&base);base=p.divrem(&product,c).map_err(|_|Error::Arithmetic)?.1;}
  }
  let residue=p.sub(&acc,&x);self.dense=Some(p.gcd_monic(c,&residue,self.degree).map_err(|_|Error::Arithmetic)?.polynomial);Ok(())
 }
 fn pullback(&self,cap:usize)->Result<Vec<u64>,Error>{
  if let Some(g)=&self.dense{return pullback(g,self.m,self.r,cap)}
  let degree=usize::try_from(self.r).map_err(|_|Error::Limit)?;
  if degree>cap{return Err(Error::Limit)}
  let len=degree.checked_add(1).ok_or(Error::Limit)?;let mut out=vec![0;len];out[1]=self.r-1;out[degree]=1;Ok(out)
 }
}
fn implicit_checks(){
 let(mut cases,mut zero_kept,mut cap_rejects)=(0,0,0);
 for r in [7u64,11,13,17,31,97]{for m in 2..r as usize{if (r-1)%m as u64!=0{continue}let d=((r-1)/m as u64)as usize+1;let mut universe=vec![0;d+1];universe[1]=r-1;universe[d]=1;
  for degree in 0..=d.min(10){for seed in 0..8u64{
   let mut state=ImageDomain::new(r,m).unwrap();let mut old=universe.clone();let mut p=polynomial::Poly::new(r).unwrap();
   let mut c:Vec<_>=(0..=degree).map(|j|(seed*7+j as u64*3)%r).collect();if seed==0{c.fill(0);}
   for constraint in [vec![0],c.clone(),c,vec![r-1,1],vec![1]]{
    state.intersect(&mut p,&constraint).unwrap();old=p.gcd_monic(&old,&constraint,d).unwrap().polynomial;
    if let Some(g)=&state.dense{assert_eq!(g,&old)}else{assert_eq!(old,universe);zero_kept+=1;}
    assert_eq!(state.degree(),old.len()-1);assert_eq!(state.pullback(r as usize).unwrap(),pullback(&old,m,r,r as usize).unwrap());
    if state.pullback(0).is_err(){cap_rejects+=1;}cases+=1;
   }
  }}
 }}
 let r=(1u64<<61)-1;let mut state=ImageDomain::new(r,2).unwrap();assert!(state.dense.is_none());assert!(matches!(state.pullback(64),Err(Error::Limit)));let mut p=polynomial::Poly::new(r).unwrap();state.intersect(&mut p,&[0]).unwrap();assert!(state.dense.is_none());state.intersect(&mut p,&[r-1,1]).unwrap();assert_eq!(state.dense,Some(vec![r-1,1]));assert_eq!(state.linear_root(),Some(1));
 assert!(ImageDomain::new(13,5).is_err());assert!(ImageDomain::new(13,0).is_err());
 let mut s=ImageDomain::new(13,3).unwrap();assert!(s.intersect(&mut polynomial::Poly::new(13).unwrap(),&[]).is_err());assert!(s.intersect(&mut polynomial::Poly::new(13).unwrap(),&[13]).is_err());
 println!("implicit_intersections={cases} zero_constraints_keep_implicit={zero_kept} cap_rejections={cap_rejects} large_prime_domain=2^61-1 large_domain_allocated=false large_linear_constraint_exact=true");
}
