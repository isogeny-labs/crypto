//! Public-point source integration. Prime p and prime subgroup order r are caller-certified.
#[path="../../feature-native-adaptive-extension/geometry.rs"] mod geometry;
#[path="../../feature-native-division/lib.rs"] mod jets;
#[path="../../feature-native-value-division/lib.rs"] mod division;
#[path="../../feature-native-low-product/polynomial.rs"] mod polynomial;
#[derive(Debug,PartialEq)] pub enum Error { Domain, Geometry, Phase, Division, Polynomial, Degenerate, Character, Limit }
#[derive(Debug)] pub struct Raw {pub value:u64,pub seed:u64,pub ratio:[u64;2],pub resultant:u64,pub recurrence_updates:u64,pub partial_value:Option<u64>}
struct RawParts {seed:u64,ratio:[u64;2],resultant:u64,recurrence_updates:u64,partial_value:Option<u64>}
pub struct Source {p:u64,a:u64,b:u64,r:u64,base:[u64;2],curve:geometry::Curve,partition:[u64;4],calibration_inverse:u64,base_seed:u64,seed_exponent:u64}
fn mul(a:u64,b:u64,p:u64)->u64{((a as u128*b as u128)%p as u128)as u64}
fn pow(mut a:u64,mut n:u64,p:u64)->u64{let mut z=1;while n!=0{if n&1!=0{z=mul(z,a,p);}n>>=1;if n!=0{a=mul(a,a,p);}}z}
fn inverse(a:u64,p:u64)->Result<u64,Error>{let(mut r,mut s,mut x,mut y)=(p as i128,a as i128,0i128,1i128);while s!=0{let q=r/s;(r,s)=(s,r-q*s);(x,y)=(y,x-q*y);}if r!=1{Err(Error::Degenerate)}else{Ok(x.rem_euclid(p as i128)as u64)}}
// Compare a nonzero calibrated ratio directly; no affine field value is needed.
fn ratio_sign(n:u64,d:u64,calibration_inverse:u64,p:u64)->Result<i8,Error>{
 if n==0||d==0||calibration_inverse==0||n>=p||d>=p||calibration_inverse>=p{return Err(Error::Degenerate);}
 let calibrated=mul(n,calibration_inverse,p);
 if calibrated==d{Ok(1)}else if calibrated==p-d{Ok(-1)}else{Err(Error::Character)}
}
fn seed_step(delta:u64,square:u64,previous:[u64;2],older:[u64;2],p:u64)->Result<[u64;2],Error>{
 if previous.iter().chain(older.iter()).any(|&v|v==0||v>=p)||delta==0||delta>=p||square==0||square>=p{return Err(Error::Degenerate);}
 let n=mul(mul(mul(delta,square,p),mul(previous[0],previous[0],p),p),older[1],p);
 let d=mul(mul(previous[1],previous[1],p),older[0],p);
 if n==0||d==0{return Err(Error::Degenerate);}Ok([n,d])
}
fn partition(r:u64)->Result<[u64;4],Error>{if r<7||r&1==0{return Err(Error::Domain);}let h=(r-1)/2;let target=(h+1)/2;let(mut lo,mut hi)=(1u64,target);while lo<hi{let mid=lo+(hi-lo)/2;if mid as u128*mid as u128>=target as u128{hi=mid;}else{lo=mid+1;}}let w=lo.min(h/2);let tail=h%(2*w);Ok([w,h/(2*w),tail/2,tail%2])}
// Public integer exponent arithmetic modulo p-1, not field arithmetic.
fn seed_exponent(p:u64,r:u64,part:[u64;4])->u64{
 let m=(p-1)as u128;let [w,k,s,e]=part.map(|v|v as u128);let t=(r/2+1)as u128;
 let add=|a:u128,b:u128|(a+b)%m;let product=|a:u128,b:u128|((a%m)*(b%m))%m;
 // Sum j(j+1), j=0..n-1. Divide the integer factor before reducing;
 // 3 need not be invertible modulo p-1.
 let g=|n:u128|{if n==0{return 0;}let mut f=[n-1,n,n+1];let i=f.iter().position(|x|x%3==0).unwrap();f[i]/=3;product(product(f[0],f[1]),f[2])};
 let t2=product(t,t);let babies=add(product(w,t2),g(w));
 let centers=add(add(product(k,add(t2,product(w,w+1))),product(w,product(k,k-1))),product(product(4,product(w,w)),g(k)));
 let prefix=add(product(s,t2),g(s));
 let mut total=add(product(2*w,centers),product(2*k,babies));
 if s!=0{total=add(total,add(product(2*s,product(s+e,s+e)),product(2,prefix)));}
 if e!=0{total=add(total,t2);}total as u64
}
impl Source {
 pub fn new(p:u64,a:u64,b:u64,r:u64,base:[u64;2])->Result<Self,Error>{
  let partition=partition(r)?;let curve=geometry::Curve::new(p,a,b).map_err(|_|Error::Geometry)?;
  let mut out=Self{p,a,b,r,base,curve,partition,calibration_inverse:0,base_seed:0,seed_exponent:seed_exponent(p,r,partition)};
  let calibration=out.raw(base)?;out.base_seed=calibration.seed;out.calibration_inverse=inverse(calibration.value,p)?;Ok(out)
 }
 pub fn raw(&self,q:[u64;2])->Result<Raw,Error>{let[w,k,s,e]=self.partition;let grid=self.curve.grid(self.r,q,w,k,s,e).map_err(|_|Error::Geometry)?;self.raw_grid(q,&grid.affine)}
 fn raw_grid(&self,q:[u64;2],grid:&[[u64;2]])->Result<Raw,Error>{
  let seed=self.phase(q)?;self.raw_grid_seed(q,grid,seed)
 }
 fn phase(&self,q:[u64;2])->Result<u64,Error>{#[cfg(test)] PHASE_CALLS.with(|n|n.set(n.get()+1));let mut jet=jets::Division::new(self.p,self.a,self.b,q[0],q[1]).map_err(|_|Error::Phase)?;jet.phase(self.r).map_err(|_|Error::Phase)}
 fn raw_grid_seed(&self,q:[u64;2],grid:&[[u64;2]],seed:u64)->Result<Raw,Error>{
  let parts=self.raw_grid_seed_parts(q,grid,seed)?;
  let value=mul(parts.ratio[0],inverse(parts.ratio[1],self.p)?,self.p);
  Ok(Raw{value,seed:parts.seed,ratio:parts.ratio,resultant:parts.resultant,recurrence_updates:parts.recurrence_updates,partial_value:parts.partial_value})
 }
 fn sign_grid_seed(&self,q:[u64;2],grid:&[[u64;2]],seed:u64)->Result<i8,Error>{
  let parts=self.raw_grid_seed_parts(q,grid,seed)?;
  ratio_sign(parts.ratio[0],parts.ratio[1],self.calibration_inverse,self.p)
 }
 fn raw_grid_seed_parts(&self,q:[u64;2],grid:&[[u64;2]],seed:u64)->Result<RawParts,Error>{
  let p=self.p;let[w,k,s,e]=self.partition;
  if seed==0||seed>=p{return Err(Error::Degenerate);}
  let mut values=division::ValueDivision::new(p,self.a,self.b,q[0],q[1]).map_err(|_|Error::Division)?;
  let mut scaled=|n:u64|mul(pow(seed,((n as u128*n as u128)%(p-1)as u128)as u64,p),values.value(n),p);
  let u=scaled(self.r/2+1);let step=scaled(2*w);let seed_sq=mul(seed,seed,p);let step_sq=mul(step,step,p);
  let update=|a:[u64;2],b:[u64;2],n:[u64;2],prev:[u64;2],sq:u64|->Result<[u64;2],Error>{let out=[mul(mul(mul((a[0]+p-b[0])%p,mul(n[0],n[0],p),p),sq,p),prev[1],p),mul(mul(n[1],n[1],p),prev[0],p)];if out.contains(&0){Err(Error::Degenerate)}else{Ok(out)}};
  let pairmul=|a:[u64;2],b:[u64;2]|[mul(a[0],b[0],p),mul(a[1],b[1],p)];
  let mut previous=[(p-u)%p,1];let mut current=[u,1];let mut t=grid[0];let(mut babies,mut centers,mut prefix_x)=(Vec::new(),Vec::new(),Vec::new());let(mut bp,mut ap,mut prefix)=([1,1],[1,1],[1,1]);let mut updates=0;
  for j in 0..w{babies.push(t[0]);bp=pairmul(bp,current);if j<s{prefix_x.push(t[0]);prefix=pairmul(prefix,current);}let next=update(q,t,current,previous,seed_sq)?;previous=current;current=next;t=grid[2+j as usize];updates+=1;}
  previous[0]=(p-previous[0])%p;
  for i in 0..k{centers.push(t[0]);ap=pairmul(ap,current);if i+1<k{let next=update(grid[1],t,current,previous,step_sq)?;previous=current;current=next;t=grid[2+w as usize+i as usize];updates+=1;}}
  let mut poly=polynomial::Poly::new(p).map_err(|_|Error::Polynomial)?;
  let resultant=poly.root_resultant(&babies,&centers).map_err(|_|Error::Polynomial)?;
  let sign=if (w*k)&1==0{1}else{p-1};
  let mut numerator=mul(mul(mul(sign,resultant,p),pow(ap[0],2*w,p),p),pow(bp[0],2*k,p),p);
  let mut denominator=mul(pow(ap[1],2*w,p),pow(bp[1],2*k,p),p);let mut partial_value=None;
  if s!=0{let c=(p-scaled(s+e))%p;let center=grid.last().ok_or(Error::Geometry)?;let v=poly.root_resultant(&prefix_x,&[center[0]]).map_err(|_|Error::Polynomial)?;partial_value=Some(v);let factor=mul(mul(mul(if s&1==0{1}else{p-1},v,p),pow(c,2*s,p),p),mul(prefix[0],prefix[0],p),p);numerator=mul(numerator,factor,p);denominator=mul(denominator,mul(prefix[1],prefix[1],p),p);}
  if e!=0{numerator=mul(numerator,(p-u)%p,p);}if numerator==0||denominator==0{return Err(Error::Degenerate);}
  Ok(RawParts{seed,ratio:[numerator,denominator],resultant,recurrence_updates:updates,partial_value})
 }
 fn unit_grid_parts(&self,q:[u64;2],grid:&[[u64;2]])->Result<RawParts,Error>{
  let p=self.p;let[w,k,s,e]=self.partition;
  let seed=1;
  let mut values=division::ValueDivision::new(p,self.a,self.b,q[0],q[1]).map_err(|_|Error::Division)?;
  let mut scaled=|n:u64|values.value(n);
  let u=scaled(self.r/2+1);let step=scaled(2*w);let seed_sq=1;let step_sq=mul(step,step,p);
  let update=|a:[u64;2],b:[u64;2],n:[u64;2],prev:[u64;2],sq:u64|->Result<[u64;2],Error>{let out=[mul(mul(mul((a[0]+p-b[0])%p,mul(n[0],n[0],p),p),sq,p),prev[1],p),mul(mul(n[1],n[1],p),prev[0],p)];if out.contains(&0){Err(Error::Degenerate)}else{Ok(out)}};
  let pairmul=|a:[u64;2],b:[u64;2]|[mul(a[0],b[0],p),mul(a[1],b[1],p)];
  let mut previous=[(p-u)%p,1];let mut current=[u,1];let mut t=grid[0];let(mut babies,mut centers,mut prefix_x)=(Vec::new(),Vec::new(),Vec::new());let(mut bp,mut ap,mut prefix)=([1,1],[1,1],[1,1]);let mut updates=0;
  for j in 0..w{babies.push(t[0]);bp=pairmul(bp,current);if j<s{prefix_x.push(t[0]);prefix=pairmul(prefix,current);}let next=update(q,t,current,previous,seed_sq)?;previous=current;current=next;t=grid[2+j as usize];updates+=1;}
  previous[0]=(p-previous[0])%p;
  for i in 0..k{centers.push(t[0]);ap=pairmul(ap,current);if i+1<k{let next=update(grid[1],t,current,previous,step_sq)?;previous=current;current=next;t=grid[2+w as usize+i as usize];updates+=1;}}
  let mut poly=polynomial::Poly::new(p).map_err(|_|Error::Polynomial)?;
  let resultant=poly.root_resultant(&babies,&centers).map_err(|_|Error::Polynomial)?;
  let sign=if (w*k)&1==0{1}else{p-1};
  let mut numerator=mul(mul(mul(sign,resultant,p),pow(ap[0],2*w,p),p),pow(bp[0],2*k,p),p);
  let mut denominator=mul(pow(ap[1],2*w,p),pow(bp[1],2*k,p),p);let mut partial_value=None;
  if s!=0{let c=(p-scaled(s+e))%p;let center=grid.last().ok_or(Error::Geometry)?;let v=poly.root_resultant(&prefix_x,&[center[0]]).map_err(|_|Error::Polynomial)?;partial_value=Some(v);let factor=mul(mul(mul(if s&1==0{1}else{p-1},v,p),pow(c,2*s,p),p),mul(prefix[0],prefix[0],p),p);numerator=mul(numerator,factor,p);denominator=mul(denominator,mul(prefix[1],prefix[1],p),p);}
  if e!=0{numerator=mul(numerator,(p-u)%p,p);}if numerator==0||denominator==0{return Err(Error::Degenerate);}
  Ok(RawParts{seed,ratio:[numerator,denominator],resultant,recurrence_updates:updates,partial_value})
 }
 #[cfg(test)]
 fn reference_grid_pair(&self,q:[u64;2],grid:&[[u64;2]],seed:[u64;2])->Result<i8,Error>{
  let p=self.p;let[w,k,s,e]=self.partition;
  if seed.iter().any(|&v|v==0||v>=p){return Err(Error::Degenerate);}
  let mut values=division::ValueDivision::new(p,self.a,self.b,q[0],q[1]).map_err(|_|Error::Division)?;
  let mut scaled=|n:u64|{let exponent=((n as u128*n as u128)%(p-1)as u128)as u64;[mul(pow(seed[0],exponent,p),values.value(n),p),pow(seed[1],exponent,p)]};
  let u=scaled(self.r/2+1);let step=scaled(2*w);let seed_sq=[mul(seed[0],seed[0],p),mul(seed[1],seed[1],p)];let step_sq=[mul(step[0],step[0],p),mul(step[1],step[1],p)];
  let update=|a:[u64;2],b:[u64;2],n:[u64;2],prev:[u64;2],sq:[u64;2]|->Result<[u64;2],Error>{let out=[mul(mul(mul((a[0]+p-b[0])%p,mul(n[0],n[0],p),p),sq[0],p),prev[1],p),mul(mul(mul(n[1],n[1],p),sq[1],p),prev[0],p)];if out.contains(&0){Err(Error::Degenerate)}else{Ok(out)}};
  let pairmul=|a:[u64;2],b:[u64;2]|[mul(a[0],b[0],p),mul(a[1],b[1],p)];
  let mut previous=[(p-u[0])%p,u[1]];let mut current=u;let mut t=grid[0];let(mut babies,mut centers,mut prefix_x)=(Vec::new(),Vec::new(),Vec::new());let(mut bp,mut ap,mut prefix)=([1,1],[1,1],[1,1]);let mut updates=0;
  for j in 0..w{babies.push(t[0]);bp=pairmul(bp,current);if j<s{prefix_x.push(t[0]);prefix=pairmul(prefix,current);}let next=update(q,t,current,previous,seed_sq)?;previous=current;current=next;t=grid[2+j as usize];updates+=1;}
  previous[0]=(p-previous[0])%p;
  for i in 0..k{centers.push(t[0]);ap=pairmul(ap,current);if i+1<k{let next=update(grid[1],t,current,previous,step_sq)?;previous=current;current=next;t=grid[2+w as usize+i as usize];updates+=1;}}
  let mut poly=polynomial::Poly::new(p).map_err(|_|Error::Polynomial)?;
  let resultant=poly.root_resultant(&babies,&centers).map_err(|_|Error::Polynomial)?;
  let sign=if (w*k)&1==0{1}else{p-1};
  let mut numerator=mul(mul(mul(sign,resultant,p),pow(ap[0],2*w,p),p),pow(bp[0],2*k,p),p);
  let mut denominator=mul(pow(ap[1],2*w,p),pow(bp[1],2*k,p),p);
  if s!=0{let mut c=scaled(s+e);c[0]=(p-c[0])%p;let center=grid.last().ok_or(Error::Geometry)?;let v=poly.root_resultant(&prefix_x,&[center[0]]).map_err(|_|Error::Polynomial)?;let factor=mul(mul(mul(if s&1==0{1}else{p-1},v,p),pow(c[0],2*s,p),p),mul(prefix[0],prefix[0],p),p);numerator=mul(numerator,factor,p);denominator=mul(mul(denominator,mul(prefix[1],prefix[1],p),p),pow(c[1],2*s,p),p);}
  if e!=0{numerator=mul(numerator,(p-u[0])%p,p);denominator=mul(denominator,u[1],p);}if numerator==0||denominator==0{return Err(Error::Degenerate);}
  ratio_sign(numerator,denominator,self.calibration_inverse,p)
 }
 fn factored_grid_ratio(&self,q:[u64;2],grid:&[[u64;2]],seed:[u64;2])->Result<[u64;2],Error>{
  if seed.iter().any(|&v|v==0||v>=self.p){return Err(Error::Degenerate);}
  let unit=self.unit_grid_parts(q,grid)?;
  Ok([mul(unit.ratio[0],pow(seed[0],self.seed_exponent,self.p),self.p),mul(unit.ratio[1],pow(seed[1],self.seed_exponent,self.p),self.p)])
 }
 fn sign_grid_pair(&self,q:[u64;2],grid:&[[u64;2]],seed:[u64;2])->Result<i8,Error>{#[cfg(test)] SIGN_CALLS.with(|n|n.set(n.get()+1));
  let ratio=self.factored_grid_ratio(q,grid,seed)?;
  ratio_sign(ratio[0],ratio[1],self.calibration_inverse,self.p)
 }
 fn normalize(&self,raw:u64)->Result<i8,Error>{match mul(raw,self.calibration_inverse,self.p){1=>Ok(1),v if v==self.p-1=>Ok(-1),_=>Err(Error::Character)}}
 pub fn sign(&self,q:Option<[u64;2]>)->Result<i8,Error>{match q{None=>Ok(0),Some(q)=>{let[w,k,s,e]=self.partition;let grid=self.curve.grid(self.r,q,w,k,s,e).map_err(|_|Error::Geometry)?;self.sign_grid_seed(q,&grid.affine,self.phase(q)?)}}}
 /// Produces Q,Q+P,... internally; neither target scalars nor precomputed signs are inputs.
 pub fn window(&self,q:Option<[u64;2]>,length:usize)->Result<Vec<i8>,Error>{
  if length==0{return Ok(Vec::new());}let[w,k,s,e]=self.partition;let prepared=self.curve.prepare(self.r,self.base,w,k,s,e).map_err(|_|Error::Geometry)?;let joint=prepared.joint_window(q,length).map_err(|_|Error::Geometry)?;
  let lanes=joint.geometry.labels.len();let mut grids=vec![vec![[0,0];lanes];length];let mut seen=vec![0usize;length];
  for slot in &joint.geometry.slots{grids[slot.offset][slot.lane]=slot.affine;seen[slot.offset]+=1;}
  joint.shifts.iter().enumerate().map(|(i,q)|match q{None=>Ok(0),Some(q)=>{if seen[i]!=lanes{return Err(Error::Geometry);}self.sign_grid_seed(*q,&grids[i],self.phase(*q)?)}}).collect()
 }
}
#[cfg(test)] mod integration_tests {
 use super::*;
 fn rows()->Vec<Vec<u64>>{include_str!("../../feature-native-shared-seeds/fixtures.tsv").lines().map(|l|l.split('\t').map(|s|s.parse().unwrap()).collect()).collect()}
 #[test] fn saved_raw_ratios(){for f in rows(){let source=Source::new(f[0],f[1],f[2],f[3],[f[4],f[5]]).unwrap();let raw=source.raw([f[4],f[5]]).unwrap();assert_eq!(raw.value,f[6]);assert_eq!(raw.seed,f[7]);assert_eq!(raw.ratio,[f[8],f[9]]);assert_eq!(raw.resultant,f[10]);assert_eq!(raw.partial_value,Some(f[11]));assert_eq!(raw.recurrence_updates,source.partition[0]+source.partition[1]-1);assert_eq!(source.normalize(raw.value),Ok(1));}}
 #[test] fn identity_crossing_windows(){let s=Source::new(509,2,10,479,[3,68]).unwrap();for (q,first) in [(None,0u64),(Some([3,68]),1),(Some([3,441]),478)]{let actual=s.window(q,3).unwrap();let expected:Vec<i8>=(0..3).map(|j|{let k=(first+j)%479;if k==0{0}else if pow(k,239,479)==1{1}else{-1}}).collect();assert_eq!(actual,expected);}assert_eq!(s.sign(None),Ok(0));assert!(s.window(None,0).unwrap().is_empty());assert!(s.sign(Some([0,0])).is_err());}
 #[test] fn partitions_and_rejections(){assert_eq!(partition(7),Ok([1,1,0,1]));assert_eq!(partition(17),Ok([2,2,0,0]));assert_eq!(partition(5),Err(Error::Domain));assert!(Source::new(509,2,10,478,[3,68]).is_err());assert!(Source::new(509,2,10,509,[3,68]).is_err());}
}

/// Borrows immutable curve/calibration; reusable target-independent geometry and anchors.
pub struct PreparedSource<'a>{source:&'a Source,geometry:geometry::Prepared<'a>,square:u64,anchors:[(Option<[u64;2]>,u64,&'static str);4]}
#[derive(Debug,PartialEq)] pub struct PairSeedWindow {pub signs:Vec<i8>,pub seed_pairs:Vec<[u64;2]>,pub methods:Vec<&'static str>,pub shifts:Vec<Option<[u64;2]>>,pub direct_phases:usize,pub recurrence_updates:usize,pub anchor_hits:usize}
impl Source {
 pub fn prepare(&self)->Result<PreparedSource<'_>,Error>{
  let p=self.p;let[x,y]=self.base;let[w,k,s,e]=self.partition;
  let geometry=self.curve.prepare(self.r,self.base,w,k,s,e).map_err(|_|Error::Geometry)?;
  let slope=mul((mul(3,mul(x,x,p),p)+self.a)%p,inverse(mul(2,y,p),p)?,p);
  let xx=(mul(slope,slope,p)+p-mul(2,x,p))%p;let yy=(mul(slope,(x+p-xx)%p,p)+p-y)%p;
  let square=mul(self.base_seed,self.base_seed,p);let c2=mul(mul(square,square,p),mul(2,y,p),p);
  Ok(PreparedSource{source:self,geometry,square,anchors:[(None,0,"O"),(Some(self.base),self.base_seed,"P"),(Some([x,(p-y)%p]),(p-self.base_seed)%p,"-P"),(Some([xx,yy]),c2,"2P")]})
 }
}
impl PreparedSource<'_>{
 pub fn window(&self,q:Option<[u64;2]>,length:usize)->Result<PairSeedWindow,Error>{
  let mut out=PairSeedWindow{signs:Vec::with_capacity(length),seed_pairs:Vec::with_capacity(length),methods:Vec::with_capacity(length),shifts:Vec::new(),direct_phases:0,recurrence_updates:0,anchor_hits:0};
  if length==0{return Ok(out);}let joint=self.geometry.joint_window(q,length).map_err(|_|Error::Geometry)?;let lanes=joint.geometry.labels.len();let mut grids=vec![vec![[0,0];lanes];length];let mut seen=vec![0usize;length];for slot in &joint.geometry.slots{grids[slot.offset][slot.lane]=slot.affine;seen[slot.offset]+=1;}
  let p=self.source.p;
  for(i,point)in joint.shifts.iter().enumerate(){
   let(seed,method)=if let Some((_,c,label))=self.anchors.iter().find(|(a,_,_)|a==point){out.anchor_hits+=1;([*c,1],*label)}else if i<2{out.direct_phases+=1;([self.source.phase(point.ok_or(Error::Geometry)?)?,1],"direct")}else{
    let middle=joint.shifts[i-1].ok_or(Error::Degenerate)?;let seed=seed_step((self.source.base[0]+p-middle[0])%p,self.square,out.seed_pairs[i-1],out.seed_pairs[i-2],p)?;out.recurrence_updates+=1;(seed,"recurrence")
   };
   let sign=match point{None=>0,Some(point)=>{if seen[i]!=lanes{return Err(Error::Geometry);}self.source.sign_grid_pair(*point,&grids[i],seed)?}};
   out.seed_pairs.push(seed);out.methods.push(method);out.signs.push(sign);
  }
  out.shifts=joint.shifts;Ok(out)
 }
}
#[cfg(test)]mod seed_tests{
 use super::*;
 fn expected(k:u64,r:u64)->i8{if k==0{0}else if pow(k,(r-1)/2,r)==1{1}else{-1}}
 #[test]fn reference_windows_and_phase(){
  let source=Source::new(509,2,10,479,[3,68]).unwrap();let prepared=source.prepare().unwrap();
  for(q,start,direct)in[(None,0u64,0),(Some([3,68]),1,0),(Some([209,180]),2,1),(Some([3,441]),478,0),(Some([156,427]),3,2)]{
   let out=prepared.window(q,16).unwrap();assert_eq!(out.direct_phases,direct);assert_eq!(out.direct_phases+out.recurrence_updates+out.anchor_hits,16);
   for i in 0..16{assert_eq!(out.signs[i],expected((start+i as u64)%479,479));match out.shifts[i]{None=>assert_eq!(out.seed_pairs[i],[0,1]),Some(point)=>assert_eq!(out.seed_pairs[i][0],mul(source.phase(point).unwrap(),out.seed_pairs[i][1],source.p))}}
   for line in include_str!("../../feature-native-shared-seeds/points.tsv").lines(){let row:Vec<i64>=line.split('\t').map(|s|s.parse().unwrap()).collect();if row[0]==start as i64{let point=if row[2]<0{None}else{Some([row[2]as u64,row[3]as u64])};assert_eq!(out.shifts[row[1]as usize],point);}}
   println!("seed_counts start={} direct={} recurrence={} anchors={}",start,out.direct_phases,out.recurrence_updates,out.anchor_hits);
  }
 }
 #[test]fn reusable_setup_and_saved_records(){for line in include_str!("../../feature-native-shared-seeds/fixtures.tsv").lines(){let f:Vec<u64>=line.split('\t').map(|s|s.parse().unwrap()).collect();let source=Source::new(f[0],f[1],f[2],f[3],[f[4],f[5]]).unwrap();assert_eq!(source.base_seed,f[7]);let raw=source.raw(source.base).unwrap();assert_eq!(raw.value,f[6]);assert_eq!(raw.ratio,[f[8],f[9]]);let prepared=source.prepare().unwrap();let first=prepared.window(None,5).unwrap();let second=prepared.window(None,5).unwrap();assert_eq!(first,second);for i in 0..5{assert_eq!(first.signs[i],expected(i as u64,f[3]));if let Some(point)=first.shifts[i]{assert_eq!(first.seed_pairs[i][0],mul(source.phase(point).unwrap(),first.seed_pairs[i][1],source.p));}}assert_eq!(first.direct_phases,0);assert!(prepared.window(None,0).unwrap().signs.is_empty());assert!(prepared.window(Some([0,0]),3).is_err());}}
 #[test]fn partial_and_singleton_branches(){for(p,a,b,r,base,partition_expected)in[(7,0,3,13,[1,2],[2,1,1,0]),(5,2,1,7,[0,1],[1,1,0,1])]{let source=Source::new(p,a,b,r,base).unwrap();assert_eq!(source.partition,partition_expected);let prepared=source.prepare().unwrap();let out=prepared.window(None,5).unwrap();for i in 0..5{assert_eq!(out.signs[i],expected(i as u64,r));if let Some(point)=out.shifts[i]{assert_eq!(out.seed_pairs[i][0],mul(source.phase(point).unwrap(),out.seed_pairs[i][1],source.p));}}}}
}

#[cfg(test)]mod ratio_tests{
 use super::*;
 #[test]fn exhaustive_ratio_predicate_and_scaling(){
  for p in [7u64,13]{for c in 1..p{for n in 1..p{for d in 1..p{
   let value=mul(mul(n,pow(d,p-2,p),p),c,p);let expected=match value{1=>Ok(1),v if v==p-1=>Ok(-1),_=>Err(Error::Character)};
   assert_eq!(ratio_sign(n,d,c,p),expected);
   for scale in [2,p-1]{assert_eq!(ratio_sign(mul(n,scale,p),mul(d,scale,p),c,p),expected);}
  }}}}
  for (n,d,c) in [(0,1,1),(1,0,1),(1,1,0),(7,1,1),(1,7,1),(1,1,7)]{assert_eq!(ratio_sign(n,d,c,7),Err(Error::Degenerate));}
 }
 #[test]fn direct_sign_matches_raw_on_saved_curves(){
  for f in integration_tests_rows(){let source=Source::new(f[0],f[1],f[2],f[3],[f[4],f[5]]).unwrap();let prepared=source.prepare().unwrap();let out=prepared.window(None,5).unwrap();
   for point in out.shifts{let expected=match point{None=>Ok(0),Some(q)=>source.normalize(source.raw(q).unwrap().value)};assert_eq!(source.sign(point),expected);}
  }
 }
 fn integration_tests_rows()->Vec<Vec<u64>>{include_str!("../../feature-native-source/fixtures.tsv").lines().map(|line|line.split('\t').map(|s|s.parse().unwrap()).collect()).collect()}
}

#[cfg(test)]mod pair_tests{
 use super::*;
 #[test]fn seed_step_matches_affine_with_independent_rescalings(){
  let p=13;for previous in 1..p{for older in 1..p{for alpha in 1..p{for beta in 1..p{
   let expected=mul(mul(mul(3,4,p),mul(previous,previous,p),p),pow(older,p-2,p),p);
   let pair=seed_step(3,4,[mul(previous,alpha,p),alpha],[mul(older,beta,p),beta],p).unwrap();assert_eq!(pair[0],mul(expected,pair[1],p));
  }}}}
  assert!(seed_step(3,4,[1,0],[1,1],p).is_err());assert!(seed_step(0,4,[1,1],[1,1],p).is_err());
 }
 #[test]fn scaled_seed_consumption_matches_scalar_source(){
  for line in include_str!("../../feature-native-source/fixtures.tsv").lines(){let f:Vec<u64>=line.split('\t').map(|x|x.parse().unwrap()).collect();let source=Source::new(f[0],f[1],f[2],f[3],[f[4],f[5]]).unwrap();let[w,k,s,e]=source.partition;let q=source.base;let grid=source.curve.grid(source.r,q,w,k,s,e).unwrap();let seed=source.phase(q).unwrap();let expected=source.sign_grid_seed(q,&grid.affine,seed).unwrap();for scale in [1,2,source.p-1]{assert_eq!(source.sign_grid_pair(q,&grid.affine,[mul(seed,scale,source.p),scale]),Ok(expected));}}
 }
}

#[cfg(test)] mod factor_tests {
 use super::*;
 #[test] fn closed_exponent_matches_symbolic_recurrence(){
  for p in [5u64,7,13,509,2305843009213693951]{for r in (7u64..=1023).step_by(2){
   let [w,k,s,e]=partition(r).unwrap();let m=(p-1)as i128;let t=(r/2+1)as i128;
   let(mut prev,mut curr)=(t*t%m,t*t%m);let(mut bp,mut ap,mut prefix)=(0i128,0i128,0i128);
   for j in 0..w{bp=(bp+curr)%m;if j<s{prefix=(prefix+curr)%m;}let next=(2*curr+2-prev).rem_euclid(m);prev=curr;curr=next;}
   for i in 0..k{ap=(ap+curr)%m;if i+1<k{let next=(2*curr+8*(w as i128).pow(2)-prev).rem_euclid(m);prev=curr;curr=next;}}
   let mut total=(2*w as i128*ap+2*k as i128*bp)%m;
   if s!=0{total=(total+2*s as i128*((s+e)as i128).pow(2)+2*prefix)%m;}
   if e!=0{total=(total+t*t)%m;}
   assert_eq!(seed_exponent(p,r,[w,k,s,e]),total as u64);
  }}
 }
 #[test] fn factored_ratios_match_scalar_and_pair_references(){
  let mut checks=0;
  for line in include_str!("../../feature-native-source/fixtures.tsv").lines(){
   let f:Vec<u64>=line.split('\t').map(|v|v.parse().unwrap()).collect();let source=Source::new(f[0],f[1],f[2],f[3],[f[4],f[5]]).unwrap();
   let out=source.prepare().unwrap().window(None,5).unwrap();let [w,k,s,e]=source.partition;
   for point in out.shifts.into_iter().flatten(){let grid=source.curve.grid(source.r,point,w,k,s,e).unwrap();
    for seed in [1,2,source.p-1,source.phase(point).unwrap()]{let reference=source.raw_grid_seed_parts(point,&grid.affine,seed).unwrap();
     for scale in [1,2,source.p-1]{let pair=[mul(seed,scale,source.p),scale];let got=source.factored_grid_ratio(point,&grid.affine,pair).unwrap();
      assert_eq!(mul(got[0],reference.ratio[1],source.p),mul(reference.ratio[0],got[1],source.p));
      assert_eq!(source.sign_grid_pair(point,&grid.affine,pair),source.reference_grid_pair(point,&grid.affine,pair));checks+=1;
     }
    }
   }
   assert!(source.factored_grid_ratio(source.base,&[],[0,1]).is_err());
   assert!(source.factored_grid_ratio(source.base,&[],[1,0]).is_err());
   println!("seed_exponent p={} r={} exponent={}",source.p,source.r,source.seed_exponent);
  }
  assert_eq!(checks,144);println!("factored_ratio_checks={checks}");
 }
}

#[cfg(test)]thread_local!{static PHASE_CALLS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
#[cfg(test)]pub fn phase_calls()->usize{PHASE_CALLS.with(|n|n.get())}
/// Retains only the last two seeds, last shifted point and projective lane tail.
pub struct Cursor<'b,'a>{prepared:&'b PreparedSource<'a>,tail:geometry::Tail<'b,'a>,count:usize,previous:[u64;2],older:[u64;2],middle:Option<[u64;2]>,remaining:usize}
#[derive(Debug,PartialEq)]pub struct Extension{pub sign:Option<i8>,pub seed_pair:[u64;2],pub method:&'static str,pub point:Option<[u64;2]>,pub lane_updates:usize,pub converted_points:usize,pub conversion_batches:usize}
impl<'a> PreparedSource<'a>{
 pub fn begin<'b>(&'b self,q:Option<[u64;2]>,length:usize,max_extra:usize)->Result<(PairSeedWindow,Cursor<'b,'a>),Error>{
  let mut out=PairSeedWindow{signs:Vec::with_capacity(length),seed_pairs:Vec::with_capacity(length),methods:Vec::with_capacity(length),shifts:Vec::new(),direct_phases:0,recurrence_updates:0,anchor_hits:0};
  if length==0{return Err(Error::Domain);}let(joint,tail)=self.geometry.joint_window_tail(q,length).map_err(|_|Error::Geometry)?;let lanes=joint.geometry.labels.len();let mut grids=vec![vec![[0,0];lanes];length];let mut seen=vec![0usize;length];for slot in &joint.geometry.slots{grids[slot.offset][slot.lane]=slot.affine;seen[slot.offset]+=1;}
  let p=self.source.p;
  for(i,point)in joint.shifts.iter().enumerate(){
   let(seed,method)=if let Some((_,c,label))=self.anchors.iter().find(|(a,_,_)|a==point){out.anchor_hits+=1;([*c,1],*label)}else if i<2{out.direct_phases+=1;([self.source.phase(point.ok_or(Error::Geometry)?)?,1],"direct")}else{
    let middle=joint.shifts[i-1].ok_or(Error::Degenerate)?;let seed=seed_step((self.source.base[0]+p-middle[0])%p,self.square,out.seed_pairs[i-1],out.seed_pairs[i-2],p)?;out.recurrence_updates+=1;(seed,"recurrence")
   };
   let sign=match point{None=>0,Some(point)=>{if seen[i]!=lanes{return Err(Error::Geometry);}self.source.sign_grid_pair(*point,&grids[i],seed)?}};
   out.seed_pairs.push(seed);out.methods.push(method);out.signs.push(sign);
  }
  out.shifts=joint.shifts;let previous=*out.seed_pairs.last().ok_or(Error::Domain)?;
  let older=if length>=2{out.seed_pairs[length-2]}else{[0,1]};let middle=*out.shifts.last().ok_or(Error::Domain)?;
  Ok((out,Cursor{prepared:self,tail,count:length,previous,older,middle,remaining:max_extra}))
 }
}
impl<'b,'a> Cursor<'b,'a>{
 pub fn advance(self)->Result<(Extension,Self),Error>{self.step(true)}
 pub fn skip_sign(self)->Result<(Extension,Self),Error>{self.step(false)}
 fn step(self,evaluate:bool)->Result<(Extension,Self),Error>{
  if self.remaining==0{return Err(Error::Limit);}let(next,tail)=self.tail.advance().map_err(|_|Error::Geometry)?;
  let prepared=self.prepared;let source=prepared.source;let p=source.p;let point=next.point;
  let(seed,method)=if let Some((_,c,label))=prepared.anchors.iter().find(|(q,_,_)|q==&point){([*c,1],*label)}else if self.count<2{([source.phase(point.ok_or(Error::Geometry)?)?,1],"direct")}else{
   let middle=self.middle.ok_or(Error::Degenerate)?;(seed_step((source.base[0]+p-middle[0])%p,prepared.square,self.previous,self.older,p)?,"recurrence")
  };
  let sign=if evaluate{Some(match point{None=>0,Some(point)=>source.sign_grid_pair(point,&next.grid,seed)?})}else{None};
  let extension=Extension{sign,seed_pair:seed,method,point,lane_updates:next.lane_updates,converted_points:next.converted_points,conversion_batches:next.batches};
  Ok((extension,Self{prepared,tail,count:self.count.checked_add(1).ok_or(Error::Limit)?,previous:seed,older:self.previous,middle:point,remaining:self.remaining-1}))
 }
}

#[cfg(test)]thread_local!{static SIGN_CALLS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
#[cfg(test)]pub fn sign_calls()->usize{SIGN_CALLS.with(|n|n.get())}
