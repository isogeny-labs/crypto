use super::{mul,pow,Error,Pair};
#[path="polynomial.rs"]mod polynomial;
pub(crate) fn geometry_ratio(p:u64,part:[u64;4],q:[u64;2],grid:&[[u64;2]])->Result<Pair,Error>{
  let [w,k,s,e]=part;
  if grid.len()!=1+(w+k)as usize+usize::from(s!=0){return Err(Error::Geometry);}
  let(u,seed_sq,step_sq,tail)=(1,1,1,1);
  let update=|a:[u64;2],b:[u64;2],n:[u64;2],prev:[u64;2],sq:u64|->Result<[u64;2],Error>{let out=[mul(mul(mul((a[0]+p-b[0])%p,mul(n[0],n[0],p),p),sq,p),prev[1],p),mul(mul(n[1],n[1],p),prev[0],p)];if out.contains(&0){Err(Error::Degenerate)}else{Ok(out)}};
  let pairmul=|a:[u64;2],b:[u64;2]|[mul(a[0],b[0],p),mul(a[1],b[1],p)];
  let mut previous=[(p-u)%p,1];let mut current=[u,1];let mut t=grid[0];let(mut babies,mut centers,mut prefix_x)=(Vec::new(),Vec::new(),Vec::new());let(mut bp,mut ap,mut prefix)=([1,1],[1,1],[1,1]);
  for j in 0..w{babies.push(t[0]);bp=pairmul(bp,current);if j<s{prefix_x.push(t[0]);prefix=pairmul(prefix,current);}let next=update(q,t,current,previous,seed_sq)?;previous=current;current=next;t=grid[2+j as usize];}
  previous[0]=(p-previous[0])%p;
  for i in 0..k{centers.push(t[0]);ap=pairmul(ap,current);if i+1<k{let next=update(grid[1],t,current,previous,step_sq)?;previous=current;current=next;t=grid[2+w as usize+i as usize];}}
  let mut poly=polynomial::Poly::new(p).map_err(|_|Error::Polynomial)?;
  let resultant=poly.root_resultant(&babies,&centers).map_err(|_|Error::Polynomial)?;
  let sign=if (w*k)&1==0{1}else{p-1};
  let mut numerator=mul(mul(mul(sign,resultant,p),pow(ap[0],2*w,p),p),pow(bp[0],2*k,p),p);
  let mut denominator=mul(pow(ap[1],2*w,p),pow(bp[1],2*k,p),p);
  if s!=0{let c=(p-tail)%p;let center=grid.last().ok_or(Error::Geometry)?;let v=poly.root_resultant(&prefix_x,&[center[0]]).map_err(|_|Error::Polynomial)?;let factor=mul(mul(mul(if s&1==0{1}else{p-1},v,p),pow(c,2*s,p),p),mul(prefix[0],prefix[0],p),p);numerator=mul(numerator,factor,p);denominator=mul(denominator,mul(prefix[1],prefix[1],p),p);}
  if e!=0{numerator=mul(numerator,(p-u)%p,p);}if numerator==0||denominator==0{return Err(Error::Degenerate);}
  Ok([numerator,denominator])
}
#[cfg(test)] pub(crate) fn reference_ratio(p:u64,part:[u64;4],q:[u64;2],grid:&[[u64;2]],inputs:[u64;4])->Result<Pair,Error>{
  let [w,k,s,e]=part;
  if grid.len()!=1+(w+k)as usize+usize::from(s!=0){return Err(Error::Geometry);}
  let [seed,u,step,tail]=inputs;let seed_sq=mul(seed,seed,p);let step_sq=mul(step,step,p);
  let update=|a:[u64;2],b:[u64;2],n:[u64;2],prev:[u64;2],sq:u64|->Result<[u64;2],Error>{let out=[mul(mul(mul((a[0]+p-b[0])%p,mul(n[0],n[0],p),p),sq,p),prev[1],p),mul(mul(n[1],n[1],p),prev[0],p)];if out.contains(&0){Err(Error::Degenerate)}else{Ok(out)}};
  let pairmul=|a:[u64;2],b:[u64;2]|[mul(a[0],b[0],p),mul(a[1],b[1],p)];
  let mut previous=[(p-u)%p,1];let mut current=[u,1];let mut t=grid[0];let(mut babies,mut centers,mut prefix_x)=(Vec::new(),Vec::new(),Vec::new());let(mut bp,mut ap,mut prefix)=([1,1],[1,1],[1,1]);
  for j in 0..w{babies.push(t[0]);bp=pairmul(bp,current);if j<s{prefix_x.push(t[0]);prefix=pairmul(prefix,current);}let next=update(q,t,current,previous,seed_sq)?;previous=current;current=next;t=grid[2+j as usize];}
  previous[0]=(p-previous[0])%p;
  for i in 0..k{centers.push(t[0]);ap=pairmul(ap,current);if i+1<k{let next=update(grid[1],t,current,previous,step_sq)?;previous=current;current=next;t=grid[2+w as usize+i as usize];}}
  let mut poly=polynomial::Poly::new(p).map_err(|_|Error::Polynomial)?;
  let resultant=poly.root_resultant(&babies,&centers).map_err(|_|Error::Polynomial)?;
  let sign=if (w*k)&1==0{1}else{p-1};
  let mut numerator=mul(mul(mul(sign,resultant,p),pow(ap[0],2*w,p),p),pow(bp[0],2*k,p),p);
  let mut denominator=mul(pow(ap[1],2*w,p),pow(bp[1],2*k,p),p);
  if s!=0{let c=(p-tail)%p;let center=grid.last().ok_or(Error::Geometry)?;let v=poly.root_resultant(&prefix_x,&[center[0]]).map_err(|_|Error::Polynomial)?;let factor=mul(mul(mul(if s&1==0{1}else{p-1},v,p),pow(c,2*s,p),p),mul(prefix[0],prefix[0],p),p);numerator=mul(numerator,factor,p);denominator=mul(denominator,mul(prefix[1],prefix[1],p),p);}
  if e!=0{numerator=mul(numerator,(p-u)%p,p);}if numerator==0||denominator==0{return Err(Error::Degenerate);}
  Ok([numerator,denominator])
}
