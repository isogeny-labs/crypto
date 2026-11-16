// Prime field and monic nonconstant g are certified by the caller/QuadRing.
struct SectionDecoder<'a>{ring:&'a mut QuadRing,powers:std::collections::BTreeMap<usize,Quad>}
impl SectionDecoder<'_>{
 fn power(&mut self,k:usize)->Result<Quad,Error>{
  if let Some(v)=self.powers.get(&k){return Ok(v.clone())}
  let half=self.power(k/2)?;let square=self.ring.multiply(&half,&half)?;
  let out=if k%2==0{square}else{let base=self.powers.get(&1).unwrap().clone();self.ring.multiply(&square,&base)?};
  self.powers.insert(k,out.clone());Ok(out)
 }
 fn evaluate(&mut self,a:&[u64])->Result<Quad,Error>{
  if a.is_empty(){return Ok(Quad([vec![0],vec![0]]))}
  if a.len()==1{return Ok(Quad([vec![a[0]],vec![0]]))}
  let k=a.len()/2;let low=self.evaluate(&a[..k])?;let high=self.evaluate(&a[k..])?;let power=self.power(k)?;
  let product=self.ring.multiply(&power,&high)?;Ok(self.ring.add(&low,&product))
 }
}
impl QuadRing{
 fn section_modulus(&self,m:usize,scale:u64,cap:usize)->Result<Vec<u64>,Error>{
  if m<2||m as u64>=self.r||scale==0||scale>=self.r||self.g.len()<2{return Err(Error::Domain)}
  let d=self.g.len()-1;let slots=d.checked_mul(2).and_then(|x|x.checked_mul(m)).ok_or(Error::Limit)?;
  if slots>cap{return Err(Error::Limit)}
  let raw=homogeneous_lift(self.r,&self.g)?;let mut modulus=vec![0;slots.checked_add(1).ok_or(Error::Limit)?];let mut factor=1;
  for j in (0..=2*d).rev(){modulus[m*j]=mul(raw[j],factor,self.r);factor=mul(factor,scale,self.r);}
  Ok(modulus)
 }
 fn univariate_sections(&mut self,m:usize,scale:u64,cap:usize)->Result<Vec<Quad>,Error>{
  let modulus=self.section_modulus(m,scale,cap)?;
  let mut acc=vec![1];let mut base=vec![self.r-1,1];let mut n=(self.r-1)/2;
  while n>0{if n&1!=0{let product=self.p.mul(&acc,&base);acc=self.p.divrem(&product,&modulus).map_err(|_|Error::Arithmetic)?.1;}
   n>>=1;if n>0{let product=self.p.mul(&base,&base);base=self.p.divrem(&product,&modulus).map_err(|_|Error::Arithmetic)?.1;}}
  let mut powers=std::collections::BTreeMap::new();powers.insert(0,Quad([vec![1],vec![0]]));powers.insert(1,Quad([vec![0],vec![scale]]));
  let mut decoder=SectionDecoder{ring:self,powers};let mut out=Vec::with_capacity(m);
  for e in 0..m{let coefficients:Vec<_>=acc.iter().skip(e).step_by(m).copied().collect();out.push(decoder.evaluate(&coefficients)?);}
  Ok(out)
 }
}
