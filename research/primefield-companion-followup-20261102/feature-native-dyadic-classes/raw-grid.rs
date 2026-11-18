 fn raw_grid(&self,q:[u64;2],grid:&[[u64;2]])->Result<Raw,Error>{
  let p=self.p;let[w,k,s,e]=self.partition;let mut poly=polynomial::Poly::new(p).map_err(|_|Error::Polynomial)?;
  let mut values=division::ValueDivision::new(p,self.a,self.b,q[0],q[1]).map_err(|_|Error::Division)?;
  if values.value(self.r)!=0{return Err(Error::Domain)}
  let order=1u64<<(p-1).trailing_zeros();let exponent=inverse(mul(self.r%order,self.r%order,order),order)?;
  let class=|v|poly.image_class(v).unwrap();let one=class(1);let unit=[one,one];
  let w1=class(values.value(self.r+1));let w2=class(values.value(2));let w3=class(values.value(self.r+2));
  let seed=nonzero([w3.power(exponent),cm(w1.square(),w2).power(exponent)])?;
  let mut scaled=|n:u64|{let factor=pp(seed,((n as u128*n as u128)%order as u128)as u64);[cm(factor[0],class(values.value(n))),factor[1]]};
  let u=nonzero(scaled(self.r/2+1))?;let step=nonzero(scaled(2*w))?;let seed_sq=pp(seed,2);let step_sq=pp(step,2);
  let update=|a:[u64;2],b:[u64;2],current:CP,previous:CP,square:CP|->Result<CP,Error>{let delta=class((a[0]+p-b[0])%p);nonzero([cm(cm(cm(delta,current[0].square()),square[0]),previous[1]),cm(cm(current[1].square(),square[1]),previous[0])])};
  let(mut previous,mut current,mut t)=(neg(u),u,grid[0]);let(mut babies,mut centers,mut prefix_x)=(vec![],vec![],vec![]);let(mut bp,mut ap,mut prefix)=(unit,unit,unit);
  for j in 0..w{babies.push(t[0]);bp=pm(bp,current);if j<s{prefix_x.push(t[0]);prefix=pm(prefix,current)}let next=update(q,t,current,previous,seed_sq)?;previous=current;current=next;t=grid[2+j as usize];}
  previous=neg(previous);
  for i in 0..k{centers.push(t[0]);ap=pm(ap,current);if i+1<k{let next=update(grid[1],t,current,previous,step_sq)?;previous=current;current=next;t=grid[2+w as usize+i as usize];}}
  // Tail scaling must precede mutable polynomial access (closure owns shared context).
  let tail_c=if s!=0{Some(neg(nonzero(scaled(s+e))?))}else{None};
  let resultant=poly.root_resultant_class(&babies,&centers).map_err(|_|Error::Polynomial)?;
  let mut out=pm(pm([resultant,one],pp(ap,2*w)),pp(bp,2*k));if(w*k)&1!=0{out=neg(out)}
  if let Some(c)=tail_c{let center=grid.last().ok_or(Error::Geometry)?;let v=poly.root_resultant_class(&prefix_x,&[center[0]]).map_err(|_|Error::Polynomial)?;let mut tail=pm(pm([v,one],pp(c,2*s)),pp(prefix,2));if s&1!=0{tail=neg(tail)}out=pm(out,tail)}
  if e!=0{out=pm(out,neg(u))}let out=nonzero(out)?;let quotient=cm(out[0],out[1].inverse().map_err(|_|Error::Degenerate)?);
  Ok(Raw{value:quotient.project()})
 }
