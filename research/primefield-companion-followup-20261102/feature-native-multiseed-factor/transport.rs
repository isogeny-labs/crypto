//! Fixed-index normalized division values on a public-point window.
#[path="../feature-native-adaptive-extension/geometry.rs"]mod geometry;
#[path="../feature-native-division/lib.rs"]mod jets;
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
pub struct Source{p:u64,a:u64,b:u64,r:u64,base:[u64;2],curve:geometry::Curve,partition:[u64;4],indices:Vec<u64>,lanes:Vec<usize>,base_x:Vec<u64>,squares:Vec<u64>,anchors:Vec<(Point,Vec<Pair>)>,max_window:usize}
pub struct Prepared<'a>{source:&'a Source,geometry:geometry::Prepared<'a>}
#[derive(Debug)]pub struct Window{pub grids:Vec<Vec<[u64;2]>>,pub points:Vec<Point>,pub indices:Vec<u64>,pub pairs:Vec<Vec<Pair>>,pub methods:Vec<&'static str>,pub direct_initializations:usize,pub recurrence_rows:usize,pub anchor_rows:usize,pub converted_points:usize}
impl Source{
 pub fn new(p:u64,a:u64,b:u64,r:u64,base:[u64;2],max_lanes:usize,max_window:usize)->Result<Self,Error>{
  let partition=partition(r)?;let[w,k,s,e]=partition;if max_window==0||1u128+w as u128+k as u128+u128::from(s!=0)>max_lanes as u128{return Err(Error::Limit);}
  let curve=geometry::Curve::new(p,a,b).map_err(|_|Error::Geometry)?;let grid=curve.grid(r,base,w,k,s,e).map_err(|_|Error::Geometry)?;
  let mut indices=vec![r/2+1,2*w];let mut lanes=vec![0,1];if s!=0{indices.push(s+e);lanes.push(grid.labels.len()-1);}
  for(&n,&lane)in indices.iter().zip(&lanes){if grid.labels[lane]!=n{return Err(Error::Geometry);}}
  let mut base_x:Vec<u64>=lanes.iter().map(|&i|grid.affine[i][0]).collect();indices.push(1);lanes.push(usize::MAX);base_x.push(base[0]);let double=curve.prepare(r,base,w,k,s,e).map_err(|_|Error::Geometry)?.joint_window(Some(base),2).map_err(|_|Error::Geometry)?.shifts[1].ok_or(Error::Geometry)?;
  let mut out=Self{p,a,b,r,base,curve,partition,indices,lanes,base_x,squares:vec![],anchors:vec![],max_window};
  let c=out.phase(base)?;let first=out.values(base,c)?;out.squares=first.iter().map(|v|mul(v[0],v[0],p)).collect();let c2=mul(pow(c,4,p),mul(2,base[1],p),p);let twice=out.values(double,c2)?;
  out.anchors=vec![(None,vec![[0,1];out.indices.len()]),(Some(base),first.clone()),(Some([base[0],p-base[1]]),first.iter().map(|v|[p-v[0],1]).collect()),(Some(double),twice)];Ok(out)
 }
 fn phase(&self,q:[u64;2])->Result<u64,Error>{#[cfg(test)]PHASE_CALLS.with(|n|n.set(n.get()+1));jets::Division::new(self.p,self.a,self.b,q[0],q[1]).map_err(|_|Error::Phase)?.phase(self.r).map_err(|_|Error::Phase)}
 fn values(&self,q:[u64;2],c:u64)->Result<Vec<Pair>,Error>{#[cfg(test)]VALUE_CALLS.with(|n|n.set(n.get()+1));let mut d=division::ValueDivision::new(self.p,self.a,self.b,q[0],q[1]).map_err(|_|Error::Division)?;self.indices.iter().map(|&n|{let exponent=((n as u128*n as u128)%(self.p-1)as u128)as u64;let v=mul(d.value(n),pow(c,exponent,self.p),self.p);if v==0{Err(Error::Degenerate)}else{Ok([v,1])}}).collect()}
 pub fn prepare(&self)->Result<Prepared<'_>,Error>{let[w,k,s,e]=self.partition;Ok(Prepared{source:self,geometry:self.curve.prepare(self.r,self.base,w,k,s,e).map_err(|_|Error::Geometry)?})}
}
impl Prepared<'_>{
 pub fn window(&self,q:Point,length:usize)->Result<Window,Error>{
  let s=self.source;if length==0{return Err(Error::Domain);}if length>s.max_window{return Err(Error::Limit);}
  let joint=self.geometry.joint_window(q,length).map_err(|_|Error::Geometry)?;let mut x=vec![vec![None;s.indices.len()];length];
  for slot in &joint.geometry.slots{if let Some(j)=s.lanes.iter().position(|&i|i==slot.lane){x[slot.offset][j]=Some(slot.affine[0]);}}
  let lanes=joint.geometry.labels.len();let mut grids=vec![vec![[0,0];lanes];length];let mut seen=vec![0usize;length];for slot in &joint.geometry.slots{grids[slot.offset][slot.lane]=slot.affine;seen[slot.offset]+=1;}for i in 0..length{if let Some(point)=joint.shifts[i]{if seen[i]!=lanes{return Err(Error::Geometry);}x[i][s.indices.len()-1]=Some(point[0]);}}
  let mut out=Window{grids,points:joint.shifts,indices:s.indices.clone(),pairs:Vec::with_capacity(length),methods:Vec::with_capacity(length),direct_initializations:0,recurrence_rows:0,anchor_rows:0,converted_points:joint.converted_points};
  for i in 0..length{let point=out.points[i];let(pair,method)=if let Some((_,values))=s.anchors.iter().find(|(p,_)|*p==point){out.anchor_rows+=1;(values.clone(),"anchor")}else if i<2{let point=point.ok_or(Error::Geometry)?;let c=s.phase(point)?;out.direct_initializations+=1;(s.values(point,c)?,"direct")}else{
    let mut values=Vec::with_capacity(s.indices.len());for j in 0..s.indices.len(){let previous_x=x[i-1][j].ok_or(Error::Geometry)?;values.push(step(s.p,(s.base_x[j]+s.p-previous_x)%s.p,s.squares[j],out.pairs[i-1][j],out.pairs[i-2][j])?);}out.recurrence_rows+=1;(values,"recurrence")
   };out.pairs.push(pair);out.methods.push(method);
  }Ok(out)
 }
}
#[cfg(test)]thread_local!{static PHASE_CALLS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};static VALUE_CALLS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
#[cfg(test)]pub fn calls()->(usize,usize){(PHASE_CALLS.with(|n|n.get()),VALUE_CALLS.with(|n|n.get()))}
