//! Public-point source integration. Prime p and prime subgroup order r are caller-certified.
mod geometry;
pub mod division;
mod polynomial;
#[derive(Debug,PartialEq)] pub enum Error { Domain, Geometry, Phase, Division, Polynomial, Degenerate, Character }
#[derive(Debug)] pub struct Raw {pub value:u64}
pub struct Source {p:u64,a:u64,b:u64,r:u64,base:[u64;2],curve:geometry::Curve,partition:[u64;4],calibration:u64,schedule:division::Schedule,mode:division::Mode,exponent:u64,prepared:std::sync::OnceLock<Result<geometry::Prepared,geometry::Error>>,#[cfg(source_audit)] preparation_builds:std::sync::atomic::AtomicUsize}
fn mul(a:u64,b:u64,p:u64)->u64{((a as u128*b as u128)%p as u128)as u64}
fn pow(mut a:u64,mut n:u64,p:u64)->u64{let mut z=1;while n!=0{if n&1!=0{z=mul(z,a,p);}n>>=1;if n!=0{a=mul(a,a,p);}}z}
fn inverse(a:u64,p:u64)->Result<u64,Error>{let(mut r,mut s,mut x,mut y)=(p as i128,a as i128,0i128,1i128);while s!=0{let q=r/s;(r,s)=(s,r-q*s);(x,y)=(y,x-q*y);}if r!=1{Err(Error::Degenerate)}else{Ok(x.rem_euclid(p as i128)as u64)}}
fn partition(r:u64)->Result<[u64;4],Error>{if r<7||r&1==0{return Err(Error::Domain);}let h=(r-1)/2;let target=(h+1)/2;let(mut lo,mut hi)=(1u64,target);while lo<hi{let mid=lo+(hi-lo)/2;if mid as u128*mid as u128>=target as u128{hi=mid;}else{lo=mid+1;}}let w=lo.min(h/2);let tail=h%(2*w);Ok([w,h/(2*w),tail/2,tail%2])}
include!("class-source.rs");
impl Source {
 pub fn new(p:u64,a:u64,b:u64,r:u64,base:[u64;2],max_lanes:usize)->Result<Self,Error>{Self::build(p,a,b,r,base,max_lanes,None,4096)}
 pub fn new_with_costs(p:u64,a:u64,b:u64,r:u64,base:[u64;2],max_lanes:usize,costs:division::CostModel,node_cap:usize)->Result<Self,Error>{Self::build(p,a,b,r,base,max_lanes,Some(costs),node_cap)}
 fn build(p:u64,a:u64,b:u64,r:u64,base:[u64;2],max_lanes:usize,costs:Option<division::CostModel>,node_cap:usize)->Result<Self,Error>{
  if p<5||p>u64::MAX/2||r>u64::MAX/2||r==p{return Err(Error::Domain)}
  let(mut x,mut y)=(r,p-1);while y!=0{(x,y)=(y,x%y)}if x!=1{return Err(Error::Domain)}
  let partition=partition(r)?;let[w,k,s,_]=partition;
  if 1+w as u128+k as u128+u128::from(s!=0)>max_lanes as u128{return Err(Error::Domain)}let curve=geometry::Curve::new(p,a,b).map_err(|_|Error::Geometry)?;
  let mut requests=vec![r,r+1,2,r+2,r/2+1,2*w];if s!=0{requests.push(s+partition[3])}let schedule=division::Schedule::new(&requests,node_cap).map_err(|_|Error::Domain)?;let mode=costs.map_or(division::Mode::Normalized,|c|schedule.stats().choose(c));
  let order=1u64<<(p-1).trailing_zeros();let exponent=inverse(mul(r%order,r%order,order),order)?;
  let mut out=Self{p,a,b,r,base,curve,partition,calibration:0,schedule,mode,exponent,prepared:std::sync::OnceLock::new(),#[cfg(source_audit)] preparation_builds:std::sync::atomic::AtomicUsize::new(0)};
  let calibration=out.raw(base)?.value;out.calibration=calibration;Ok(out)
 }
 #[cfg(source_audit)] pub fn preparation_builds(&self)->usize{self.preparation_builds.load(std::sync::atomic::Ordering::Relaxed)}
 pub fn raw(&self,q:[u64;2])->Result<Raw,Error>{let[w,k,s,e]=self.partition;let grid=self.curve.grid(self.r,q,w,k,s,e).map_err(|_|Error::Geometry)?;self.raw_grid(q,&grid.affine)}
 fn raw_grid(&self,q:[u64;2],grid:&[[u64;2]])->Result<Raw,Error>{
  let p=self.p;let[w,k,s,e]=self.partition;let mut poly=polynomial::Poly::new(p).map_err(|_|Error::Polynomial)?;
  let values=self.schedule.evaluate(p,self.a,self.b,q[0],q[1],self.mode).map_err(|_|Error::Division)?;
  if values[0]!=0{return Err(Error::Domain)}
  let order=1u64<<(p-1).trailing_zeros();let exponent=self.exponent;
  let class=|v|poly.image_class(v).unwrap();let one=class(1);let unit=[one,one];
  let w1=class(values[1]);let w2=class(values[2]);let w3=class(values[3]);
  let seed=nonzero([w3.power(exponent),cm(w1.square(),w2).power(exponent)])?;
  let scaled=|n:u64,v:u64|{let factor=pp(seed,((n as u128*n as u128)%order as u128)as u64);[cm(factor[0],class(v)),factor[1]]};
  let u=nonzero(scaled(self.r/2+1,values[4]))?;let step=nonzero(scaled(2*w,values[5]))?;let seed_sq=pp(seed,2);let step_sq=pp(step,2);
  let update=|a:[u64;2],b:[u64;2],current:CP,previous:CP,square:CP|->Result<CP,Error>{let delta=class((a[0]+p-b[0])%p);nonzero([cm(cm(cm(delta,current[0].square()),square[0]),previous[1]),cm(cm(current[1].square(),square[1]),previous[0])])};
  let(mut previous,mut current,mut t)=(neg(u),u,grid[0]);let(mut babies,mut centers)=(vec![],vec![]);let tail_x=if s!=0{Some(grid.last().ok_or(Error::Geometry)?[0])}else{None};let mut tail_resultant:Option<C>=None;let(mut bp,mut ap,mut prefix)=(unit,unit,unit);
  for j in 0..w{babies.push(t[0]);bp=pm(bp,current);if j<s{let delta=class((tail_x.unwrap()+p-t[0])%p);tail_resultant=Some(match tail_resultant{None=>delta,Some(v)=>cm(v,delta)});prefix=pm(prefix,current)}let next=update(q,t,current,previous,seed_sq)?;previous=current;current=next;t=grid[2+j as usize];}
  previous=neg(previous);
  for i in 0..k{centers.push(t[0]);ap=pm(ap,current);if i+1<k{let next=update(grid[1],t,current,previous,step_sq)?;previous=current;current=next;t=grid[2+w as usize+i as usize];}}
  // Tail scaling must precede mutable polynomial access (closure owns shared context).
  let tail_c=if s!=0{Some(neg(nonzero(scaled(s+e,values[6]))?))}else{None};
  let resultant=poly.root_resultant_class(&babies,&centers).map_err(|_|Error::Polynomial)?;
  let mut out=pm(pm([resultant,one],pp(ap,2*w)),pp(bp,2*k));if(w*k)&1!=0{out=neg(out)}
  if let Some(c)=tail_c{let v=tail_resultant.ok_or(Error::Geometry)?;let mut tail=pm(pm([v,one],pp(c,2*s)),pp(prefix,2));if s&1!=0{tail=neg(tail)}out=pm(out,tail)}
  if e!=0{out=pm(out,neg(u))}let out=nonzero(out)?;let quotient=cm(out[0],out[1].inverse().map_err(|_|Error::Degenerate)?);
  Ok(Raw{value:quotient.project()})
 }
 fn normalize(&self,raw:u64)->Result<i8,Error>{if raw==self.calibration{Ok(1)}else if raw==self.p-self.calibration{Ok(-1)}else{Err(Error::Character)}}
 pub fn sign(&self,q:Option<[u64;2]>)->Result<i8,Error>{match q{None=>Ok(0),Some(q)=>self.normalize(self.raw(q)?.value)}}
 /// Produces Q,Q+P,... internally; neither target scalars nor precomputed signs are inputs.
 pub fn window(&self,q:Option<[u64;2]>,length:usize)->Result<Vec<i8>,Error>{
  if length==0{return Ok(Vec::new());}let[w,k,s,e]=self.partition;let prepared=self.prepared.get_or_init(||{#[cfg(source_audit)] self.preparation_builds.fetch_add(1,std::sync::atomic::Ordering::Relaxed);self.curve.prepare(self.r,self.base,w,k,s,e)}).as_ref().map_err(|_|Error::Geometry)?;let joint=prepared.joint_window(q,length).map_err(|_|Error::Geometry)?;
  let lanes=joint.geometry.labels.len();let mut grids=vec![vec![[0,0];lanes];length];let mut seen=vec![0usize;length];
  for slot in &joint.geometry.slots{grids[slot.offset][slot.lane]=slot.affine;seen[slot.offset]+=1;}
  joint.shifts.iter().enumerate().map(|(i,q)|match q{None=>Ok(0),Some(q)=>{if seen[i]!=lanes{return Err(Error::Geometry);}self.normalize(self.raw_grid(*q,&grids[i])?.value)}}).collect()
 }
}
