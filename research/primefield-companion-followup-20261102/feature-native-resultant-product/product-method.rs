 /// Product of evaluations, retaining full-field remainders and scalar returns.
 pub fn value_product(&mut self,a:&[u64],tree:&Tree)->Result<u64,Error>{
  if tree.p!=self.p{return Err(Error::Domain);}
  if tree.children.is_none(){let x=(self.p-tree.polynomial[0])%self.p;let mut coefficients=a.iter().rev();let mut value=coefficients.next().map_or(0,|c|c%self.p);for &coefficient in coefficients{value=(self.fm(value,x)+coefficient%self.p)%self.p;}return Ok(value);}
  let rem=self.monic_remainder(a,&tree.polynomial)?;
  let(left,right)=tree.children.as_ref().ok_or(Error::Invariant)?;
  let l=self.value_product(&rem,left)?;let r=self.value_product(&rem,right)?;
  Ok(self.fm(l,r))
 }
