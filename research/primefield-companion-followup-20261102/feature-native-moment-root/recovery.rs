#[path="../feature-native-standalone-source/lib.rs"]mod source;
#[path="../feature-orbit-local-terms/local.rs"]mod group;
pub struct RecoverySource{source:source::Source,curve:group::Curve,base:group::Point,r:u64,max_coefficients:usize,max_queries:usize,root_limit:usize}
#[derive(Debug)]pub struct Recovery{pub value:u64,pub transcript:Vec<i8>,pub counts:Vec<u64>,pub root_updates:usize,pub conversion_degree:Option<usize>}
impl RecoverySource{
 pub fn new(p:u64,a:u64,b:u64,r:u64,base:[u64;2],max_lanes:usize,max_coefficients:usize,max_queries:usize,root_limit:usize)->Result<Self,Error>{
  if max_queries==0||max_queries as u128>r as u128||r as u128>max_coefficients as u128{return Err(Error::Limit)}
  let source=source::Source::new(p,a,b,r,base,max_lanes).map_err(|_|Error::Source)?;
  Ok(Self{source,curve:group::Curve::new(p,a,b),base:group::Point{x:base[0],y:base[1]},r,max_coefficients,max_queries,root_limit})
 }
 fn pair(t:Option<group::Point>)->Option<[u64;2]>{t.map(|p|[p.x,p.y])}
 fn verify(&self,k:u64,target:Option<[u64;2]>)->bool{if k>=self.r{return false}let(mut n,mut p,mut out)=(k,Some(self.base),None);while n>0{if n&1!=0{out=self.curve.plus(out,p)}n>>=1;if n>0{p=self.curve.plus(p,p)}}Self::pair(out)==target}
 pub fn recover(&self,target:Option<[u64;2]>)->Result<Recovery,Error>{
  let mut state=Hybrid::new(self.r,self.max_coefficients,self.root_limit)?;let mut point=target.map(|[x,y]|group::Point{x,y});let(mut transcript,mut counts)=(vec![],vec![]);
  for offset in 0..self.max_queries{
   // Initial point is checked by source before the first curve addition.
   let sign=self.source.sign(Self::pair(point)).map_err(|_|Error::Source)?;let moments=state.observe(offset as u64,sign)?;transcript.push(sign);counts.push(moments.count);
   if let Some(value)=moments.singleton{if !self.verify(value,target){return Err(Error::Verification)}return Ok(Recovery{value,transcript,counts,root_updates:state.root_updates,conversion_degree:state.conversion_degree})}
   if moments.count==0{return Err(Error::Verification)}if offset+1<self.max_queries{point=self.curve.plus(point,Some(self.base))}
  }Err(Error::Unresolved)
 }
}
