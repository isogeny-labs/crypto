//! Public-point source integration. Prime p and prime subgroup order r are caller-certified.
#[path="../feature-native-joint-window/lib.rs"] mod geometry;
#[path="../feature-native-value-division/lib.rs"] mod division;
#[path="../feature-native-polynomial/lib.rs"] mod polynomial;
#[derive(Debug,PartialEq)] pub enum Error { Domain, Geometry, Phase, Division, Polynomial, Degenerate, Character }
#[derive(Debug)] pub struct Raw {pub value:u64,pub seed:u64,pub ratio:[u64;2],pub resultant:u64,pub recurrence_updates:u64,pub partial_value:Option<u64>}
pub struct Source {p:u64,a:u64,b:u64,r:u64,base:[u64;2],curve:geometry::Curve,partition:[u64;4],calibration_inverse:u64}
fn mul(a:u64,b:u64,p:u64)->u64{((a as u128*b as u128)%p as u128)as u64}
fn pow(mut a:u64,mut n:u64,p:u64)->u64{let mut z=1;while n!=0{if n&1!=0{z=mul(z,a,p);}n>>=1;if n!=0{a=mul(a,a,p);}}z}
fn inverse(a:u64,p:u64)->Result<u64,Error>{let(mut r,mut s,mut x,mut y)=(p as i128,a as i128,0i128,1i128);while s!=0{let q=r/s;(r,s)=(s,r-q*s);(x,y)=(y,x-q*y);}if r!=1{Err(Error::Degenerate)}else{Ok(x.rem_euclid(p as i128)as u64)}}
fn partition(r:u64)->Result<[u64;4],Error>{if r<7||r&1==0{return Err(Error::Domain);}let h=(r-1)/2;let target=(h+1)/2;let(mut lo,mut hi)=(1u64,target);while lo<hi{let mid=lo+(hi-lo)/2;if mid as u128*mid as u128>=target as u128{hi=mid;}else{lo=mid+1;}}let w=lo.min(h/2);let tail=h%(2*w);Ok([w,h/(2*w),tail/2,tail%2])}
impl Source {
 pub fn new(p:u64,a:u64,b:u64,r:u64,base:[u64;2],max_lanes:usize)->Result<Self,Error>{
  if p<5||p>u64::MAX/2||r>u64::MAX/2||r==p{return Err(Error::Domain)}
  let(mut x,mut y)=(r,p-1);while y!=0{(x,y)=(y,x%y)}if x!=1{return Err(Error::Domain)}
  let partition=partition(r)?;let[w,k,s,_]=partition;
  if 1+w as u128+k as u128+u128::from(s!=0)>max_lanes as u128{return Err(Error::Domain)}let curve=geometry::Curve::new(p,a,b).map_err(|_|Error::Geometry)?;
  let mut out=Self{p,a,b,r,base,curve,partition,calibration_inverse:0};
  let calibration=out.raw(base)?.value;out.calibration_inverse=inverse(calibration,p)?;Ok(out)
 }
 pub fn raw(&self,q:[u64;2])->Result<Raw,Error>{let[w,k,s,e]=self.partition;let grid=self.curve.grid(self.r,q,w,k,s,e).map_err(|_|Error::Geometry)?;self.raw_grid(q,&grid.affine)}
 fn raw_grid(&self,q:[u64;2],grid:&[[u64;2]])->Result<Raw,Error>{
  let p=self.p;let[w,k,s,e]=self.partition;
  let mut values=division::ValueDivision::new(p,self.a,self.b,q[0],q[1]).map_err(|_|Error::Division)?;
  if values.value(self.r)!=0{return Err(Error::Domain)}
  let order=1u64<<(p-1).trailing_zeros();let r2=mul(self.r%order,self.r%order,order);let exponent=inverse(r2,order)?;
  let w1=values.value(self.r+1);let w2=values.value(2);let w3=values.value(self.r+2);
  if w1==0||w2==0||w3==0{return Err(Error::Degenerate)}
  let beta=mul(mul(mul(w1,w1,p),w2,p),inverse(w3,p)?,p);
  // pi(seed)=pi(beta)^(-1/r^2), all exponent inversion in Z/order Z.
  let seed=pow(inverse(beta,p)?,exponent,p);
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
  Ok(Raw{value:pow(mul(numerator,inverse(denominator,p)?,p),(p-1)/order,p),seed,ratio:[numerator,denominator],resultant,recurrence_updates:updates,partial_value})
 }
 fn normalize(&self,raw:u64)->Result<i8,Error>{match mul(raw,self.calibration_inverse,self.p){1=>Ok(1),v if v==self.p-1=>Ok(-1),_=>Err(Error::Character)}}
 pub fn sign(&self,q:Option<[u64;2]>)->Result<i8,Error>{match q{None=>Ok(0),Some(q)=>self.normalize(self.raw(q)?.value)}}
 /// Produces Q,Q+P,... internally; neither target scalars nor precomputed signs are inputs.
 pub fn window(&self,q:Option<[u64;2]>,length:usize)->Result<Vec<i8>,Error>{
  if length==0{return Ok(Vec::new());}let[w,k,s,e]=self.partition;let prepared=self.curve.prepare(self.r,self.base,w,k,s,e).map_err(|_|Error::Geometry)?;let joint=prepared.joint_window(q,length).map_err(|_|Error::Geometry)?;
  let lanes=joint.geometry.labels.len();let mut grids=vec![vec![[0,0];lanes];length];let mut seen=vec![0usize;length];
  for slot in &joint.geometry.slots{grids[slot.offset][slot.lane]=slot.affine;seen[slot.offset]+=1;}
  joint.shifts.iter().enumerate().map(|(i,q)|match q{None=>Ok(0),Some(q)=>{if seen[i]!=lanes{return Err(Error::Geometry);}self.normalize(self.raw_grid(*q,&grids[i])?.value)}}).collect()
 }
}
