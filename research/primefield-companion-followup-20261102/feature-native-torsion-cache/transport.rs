//! Combined normalized monomial on a public-point window.
#[path="../feature-native-adaptive-extension/geometry.rs"]mod geometry;
#[path="../feature-native-value-division/lib.rs"]mod division;
#[derive(Debug,PartialEq)]pub enum Error{Domain,Limit,Geometry,Phase,Division,Degenerate}
type Point=Option<[u64;2]>;
pub type Pair=[u64;2];
fn mul(a:u64,b:u64,p:u64)->u64{((a as u128*b as u128)%p as u128)as u64}
fn pow(mut a:u64,mut n:u64,p:u64)->u64{let mut v=1;while n>0{if n&1!=0{v=mul(v,a,p);}n>>=1;if n>0{a=mul(a,a,p);}}v}
fn partition(r:u64)->Result<[u64;4],Error>{if r<7||r&1==0{return Err(Error::Domain);}let h=(r-1)/2;let target=(h+1)/2;let(mut lo,mut hi)=(1u64,target);while lo<hi{let mid=lo+(hi-lo)/2;if mid as u128*mid as u128>=target as u128{hi=mid;}else{lo=mid+1;}}let w=lo.min(h/2);let tail=h%(2*w);Ok([w,h/(2*w),tail/2,tail%2])}
fn step(p:u64,delta:u64,square:u64,previous:Pair,older:Pair)->Result<Pair,Error>{
 if delta==0||delta>=p||square==0||square>=p||previous.iter().chain(older.iter()).any(|&v|v==0||v>=p){return Err(Error::Degenerate);}
 let out=[mul(mul(mul(delta,square,p),mul(previous[0],previous[0],p),p),older[1],p),mul(mul(previous[1],previous[1],p),older[0],p)];if out.contains(&0){Err(Error::Degenerate)}else{Ok(out)}
}
pub struct Source{p:u64,a:u64,b:u64,r:u64,base:[u64;2],curve:geometry::Curve,partition:[u64;4],indices:Vec<u64>,lanes:Vec<usize>,base_x:Vec<u64>,squares:Vec<u64>,powers:Vec<u64>,anchors:Vec<(Point,Vec<Pair>)>,max_window:usize}
pub struct Prepared<'a>{source:&'a Source,geometry:geometry::Prepared<'a>}
#[derive(Debug)]pub struct Window{pub grids:Vec<Vec<[u64;2]>>,pub points:Vec<Point>,pub indices:Vec<u64>,pub pairs:Vec<Vec<Pair>>,pub methods:Vec<&'static str>,pub direct_initializations:usize,pub recurrence_rows:usize,pub anchor_rows:usize,pub converted_points:usize}
impl Source{
 pub fn new(p:u64,a:u64,b:u64,r:u64,base:[u64;2],max_lanes:usize,max_window:usize)->Result<Self,Error>{
  let partition=partition(r)?;let[w,k,s,e]=partition;if max_window==0||1u128+w as u128+k as u128+u128::from(s!=0)>max_lanes as u128{return Err(Error::Limit);}
  let curve=geometry::Curve::new(p,a,b).map_err(|_|Error::Geometry)?;let grid=curve.grid(r,base,w,k,s,e).map_err(|_|Error::Geometry)?;
  let mut indices=vec![r/2+1,2*w];let mut lanes=vec![0,1];if s!=0{indices.push(s+e);lanes.push(grid.labels.len()-1);}
  for(&n,&lane)in indices.iter().zip(&lanes){if grid.labels[lane]!=n{return Err(Error::Geometry);}}
  let mut base_x:Vec<u64>=lanes.iter().map(|&i|grid.affine[i][0]).collect();indices.push(1);lanes.push(usize::MAX);base_x.push(base[0]);let double=curve.prepare(r,base,w,k,s,e).map_err(|_|Error::Geometry)?.joint_window(Some(base),2).map_err(|_|Error::Geometry)?.shifts[1].ok_or(Error::Geometry)?;
  let [delta,alpha,beta,gamma]=super::exponents(p,r).map_err(|_|Error::Domain)?;let mut powers=vec![alpha,beta];if s!=0{powers.push(gamma);}powers.push(delta);
  let order=1u64<<(p-1).trailing_zeros();if order>4{return Err(Error::Domain);}let odd=(p-1)/order;
  let powers:Vec<u64>=powers.into_iter().map(|v|((v as u128*odd as u128)%(p-1)as u128)as u64).collect();
  if !seed_closes(p,&indices,&powers){return Err(Error::Domain);}
  let mut out=Self{p,a,b,r,base,curve,partition,indices,lanes,base_x,squares:vec![],powers,anchors:vec![],max_window};
  let first=out.initial(base)?;out.squares=first.iter().map(|v|mul(v[0],v[0],p)).collect();let twice=out.values(double,1)?;
  out.anchors=vec![(None,vec![[0,1]]),(Some(base),first.clone()),(Some([base[0],p-base[1]]),first.iter().map(|v|[if out.powers.iter().fold(0,|a,b|a^(b&1))==0{v[0]}else{p-v[0]},1]).collect()),(Some(double),twice)];Ok(out)
 }
 fn validate(&self,q:[u64;2])->Result<(),Error>{#[cfg(test)]SUBGROUP_CHECKS.with(|n|n.set(n.get()+1));super::super::subgroup::validate(self.p,self.a,self.b,self.r,q).map_err(|_|Error::Phase)}
 fn values(&self,q:[u64;2],_c:u64)->Result<Vec<Pair>,Error>{#[cfg(test)]VALUE_CALLS.with(|n|n.set(n.get()+1));if self.powers.iter().all(|&v|v==0){return Ok(vec![[1,1]]);}
 #[cfg(test)]FACTOR_ENGINES.with(|n|n.set(n.get()+1));
 let mut d=division::ValueDivision::new(self.p,self.a,self.b,q[0],q[1]).map_err(|_|Error::Division)?;let mut product=1;for (&n,&power) in self.indices.iter().zip(&self.powers){if power==0{continue;}
 #[cfg(test)]FACTOR_VALUES.with(|n|n.set(n.get()+1));
 let v=d.value(n);if v==0{return Err(Error::Degenerate);}product=mul(product,pow(v,power,self.p),self.p);}Ok(vec![[product,1]])}

 fn cached_values(&self,q:[u64;2],mut d:super::super::subgroup::Validated)->Result<Vec<Pair>,Error>{if !d.matches(self.p,self.a,self.b,self.r,q){return Err(Error::Domain);}
 #[cfg(test)]VALUE_CALLS.with(|n|n.set(n.get()+1));if self.powers.iter().all(|&v|v==0){return Ok(vec![[1,1]]);}
let mut product=1;for (&n,&power) in self.indices.iter().zip(&self.powers){if power==0{continue;}
 #[cfg(test)]FACTOR_VALUES.with(|n|n.set(n.get()+1));
 let v=d.value(n);if v==0{return Err(Error::Degenerate);}product=mul(product,pow(v,power,self.p),self.p);}Ok(vec![[product,1]])}

 fn initial(&self,q:[u64;2])->Result<Vec<Pair>,Error>{
 #[cfg(test)]SUBGROUP_CHECKS.with(|n|n.set(n.get()+1));
 let d=super::super::subgroup::validated(self.p,self.a,self.b,self.r,q).map_err(|_|Error::Phase)?;
 self.cached_values(q,d)
 }

 pub fn prepare(&self)->Result<Prepared<'_>,Error>{let[w,k,s,e]=self.partition;Ok(Prepared{source:self,geometry:self.curve.prepare(self.r,self.base,w,k,s,e).map_err(|_|Error::Geometry)?})}
}
impl<'a> Prepared<'a>{
 pub fn window(&self,q:Point,length:usize)->Result<Window,Error>{self.begin(q,length,0).map(|(w,_)|w)}
 pub fn begin<'b>(&'b self,q:Point,length:usize,max_extra:usize)->Result<(Window,Cursor<'b,'a>),Error>{
  let s=self.source;if length==0{return Err(Error::Domain);}if length.checked_add(max_extra).ok_or(Error::Limit)?>s.max_window{return Err(Error::Limit);}
  let(joint,tail)=self.geometry.joint_window_tail(q,length).map_err(|_|Error::Geometry)?;let mut x=vec![vec![None;s.indices.len()];length];
  for slot in &joint.geometry.slots{if let Some(j)=s.lanes.iter().position(|&i|i==slot.lane){x[slot.offset][j]=Some(slot.affine[0]);}}
  let lanes=joint.geometry.labels.len();let mut grids=vec![vec![[0,0];lanes];length];let mut seen=vec![0usize;length];for slot in &joint.geometry.slots{grids[slot.offset][slot.lane]=slot.affine;seen[slot.offset]+=1;}for i in 0..length{if let Some(point)=joint.shifts[i]{if seen[i]!=lanes{return Err(Error::Geometry);}x[i][s.indices.len()-1]=Some(point[0]);}}
  let mut out=Window{grids,points:joint.shifts,indices:s.indices.clone(),pairs:Vec::with_capacity(length),methods:Vec::with_capacity(length),direct_initializations:0,recurrence_rows:0,anchor_rows:0,converted_points:joint.converted_points};
  for i in 0..length{let point=out.points[i];let(pair,method)=if let Some((_,values))=s.anchors.iter().find(|(p,_)|*p==point){out.anchor_rows+=1;(values.clone(),"anchor")}else if i<2{let point=point.ok_or(Error::Geometry)?;out.direct_initializations+=1;(s.initial(point)?,"direct")}else{
    let mut delta=1;for j in 0..s.indices.len(){let previous_x=x[i-1][j].ok_or(Error::Geometry)?;let difference=(s.base_x[j]+s.p-previous_x)%s.p;if difference==0{return Err(Error::Degenerate);}delta=mul(delta,pow(difference,s.powers[j],s.p),s.p);}let values=vec![step(s.p,delta,s.squares[0],out.pairs[i-1][0],out.pairs[i-2][0])?];out.recurrence_rows+=1;(values,"recurrence")
   };out.pairs.push(pair);out.methods.push(method);
  }let previous=out.pairs[length-1].clone();let older=if length>=2{out.pairs[length-2].clone()}else{vec![]};let middle_x=x.pop().ok_or(Error::Domain)?;
  Ok((out,Cursor{prepared:self,tail,count:length,previous,older,middle_x,remaining:max_extra}))
 }
}
#[cfg(test)]thread_local!{static SUBGROUP_CHECKS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};static VALUE_CALLS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
#[cfg(test)]pub fn calls()->(usize,usize){(SUBGROUP_CHECKS.with(|n|n.get()),VALUE_CALLS.with(|n|n.get()))}

include!("transport-extra.rs");

#[cfg(test)]thread_local!{
 static FACTOR_VALUES:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};
 static FACTOR_ENGINES:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};
}
#[cfg(test)]mod factor_tests{
 use super::*;
 #[test]fn omitted_direct_factors(){
  for(p,a,b,r,base,active)in[(5,2,1,7,[0,1],1),(7,0,3,13,[1,2],0),(509,2,10,479,[3,68],2)]{
   let s=Source::new(p,a,b,r,base,128,32).unwrap();s.validate(base).unwrap();let c=1;
   let before=(FACTOR_VALUES.with(|n|n.get()),FACTOR_ENGINES.with(|n|n.get()));
   let v=s.values(base,c).unwrap();let after=(FACTOR_VALUES.with(|n|n.get()),FACTOR_ENGINES.with(|n|n.get()));
   assert_eq!(after.0-before.0,active);assert_eq!(after.1-before.1,usize::from(active>0));
   assert_eq!(s.powers.iter().filter(|&&v|v!=0).count(),active);
   if active==0{assert_eq!(v,vec![[1,1]]);}
   println!("p={p} r={r} retained_direct_factor_requests={active} available_factors={} value_engine_created={}",s.indices.len(),after.1-before.1);
  }
 }
}

fn seed_closes(p:u64,indices:&[u64],powers:&[u64])->bool{
 if p<=3||indices.len()!=powers.len(){return false;}
 let m=(p-1)as u128;
 indices.iter().zip(powers).fold(0u128,|sum,(&n,&e)|(sum+(e as u128)*((n as u128*n as u128)%m)%m)%m)==0
}
#[cfg(test)]mod seed_tests{
 use super::*;
 #[test]fn closure_and_mutation(){
  let mut checked=0;
  for p in [5u64,7,13,29,61,101,509]{
   let odd=(p-1)/(1u64<<(p-1).trailing_zeros());
   for r in (7..1001).step_by(2){
    let[w,_,s,e]=partition(r).unwrap();let[d,a,b,g]=super::super::exponents(p,r).unwrap();
    let mut indices=vec![r/2+1,2*w];let mut powers=vec![a,b];if s!=0{indices.push(s+e);powers.push(g);}indices.push(1);powers.push(d);
    let mut powers:Vec<u64>=powers.into_iter().map(|v|((v as u128*odd as u128)%(p-1)as u128)as u64).collect();
    assert!(seed_closes(p,&indices,&powers));
    let last=powers.len()-1;powers[last]=(powers[last]+1)%(p-1);assert!(!seed_closes(p,&indices,&powers));checked+=1;
   }
  }
  println!("algebraic_partitions={checked} changed_seed_exponents_rejected={checked}");
 }
}

#[cfg(test)]mod cache_tests{
 use super::*;
 #[test]fn one_engine_and_binding(){
  for(p,a,b,r,base)in[(5,2,1,7,[0,1]),(7,0,3,13,[1,2]),(509,2,10,479,[3,68])]{
   let s=Source::new(p,a,b,r,base,128,32).unwrap();
   let want=s.values(base,1).unwrap();
   let before=(super::super::super::subgroup::engines(),FACTOR_ENGINES.with(|n|n.get()));
   assert_eq!(s.initial(base).unwrap(),want);
   let after=(super::super::super::subgroup::engines(),FACTOR_ENGINES.with(|n|n.get()));
   assert_eq!(after.0-before.0,1);assert_eq!(after.1-before.1,0);
   let mut cache=super::super::super::subgroup::validated(p,a,b,r,base).unwrap();
   let retained=cache.retained();assert_eq!(cache.value(r),0);assert_eq!(retained,cache.retained());
   assert!(!cache.matches(p,a,b,r,[base[0],p-base[1]]));assert!(!cache.matches(p,a,b,r+2,base));
   assert!(!cache.matches(p+2,a,b,r,base));assert!(!cache.matches(p,(a+1)%p,b,r,base));assert!(!cache.matches(p,a,(b+1)%p,r,base));
   assert!(matches!(s.cached_values([base[0],p-base[1]],cache),Err(Error::Domain)));
   println!("p={p} r={r} validation_engines=1 extra_factor_engines=0 cached_r_request_adds_entries=0");
  }
 }
}
