pub struct RecoverySource{source:source::Source,curve:group::Curve,base:group::Point,r:u64,seed:Consecutive,max_degree:usize,max_queries:usize}
#[derive(Debug)]pub struct Recovery{pub value:u64,pub signs:Vec<i8>,pub initial_degree:usize}
impl RecoverySource{
 pub fn new(p:u64,a:u64,b:u64,r:u64,base:[u64;2],max_lanes:usize,max_width:usize,max_points:usize,max_cm_attempts:u64,max_degree:usize,max_queries:usize)->Result<Self,Error>{
  if max_queries==0||max_queries as u128>r as u128{return Err(Error::Limit)}
  let source=source::Source::new(p,a,b,r,base,max_lanes).map_err(|_|Error::Source)?;
  let seed=Consecutive::new(r,max_width,max_points,max_cm_attempts)?;
  Ok(Self{source,curve:group::Curve::new(p,a,b),base:group::Point{x:base[0],y:base[1]},r,seed,max_degree,max_queries})
 }
 fn pair(t:Option<group::Point>)->Option<[u64;2]>{t.map(|p|[p.x,p.y])}
 fn verify(&self,k:u64,target:Option<[u64;2]>)->bool{if k>=self.r{return false}let(mut n,mut p,mut out)=(k,Some(self.base),None);while n>0{if n&1!=0{out=self.curve.plus(out,p)}n>>=1;if n>0{p=self.curve.plus(p,p)}}Self::pair(out)==target}
 fn finish(&self,value:u64,target:Option<[u64;2]>,signs:Vec<i8>,initial_degree:usize)->Result<Recovery,Error>{if !self.verify(value,target){return Err(Error::Verification)}Ok(Recovery{value,signs,initial_degree})}
 pub fn recover(&self,target:Option<[u64;2]>)->Result<Recovery,Error>{
  let mut current=target.map(|[x,y]|group::Point{x,y});let mut signs=Vec::new();
  for offset in 0..3{if offset>=self.max_queries{return Err(Error::Unresolved)}
   let sign=self.source.sign(Self::pair(current)).map_err(|_|Error::Source)?;signs.push(sign);
   if sign==0{return self.finish((self.r-offset as u64)%self.r,target,signs,1)}
   if offset<2{current=self.curve.plus(current,Some(self.base))}
  }
  let mut roots=self.seed.roots([signs[0],signs[1],signs[2]],self.max_degree)?;let initial_degree=roots.polynomial().len()-1;let mut moments=roots.moments();
  for offset in 3..=self.max_queries{
   if let Some(value)=moments.singleton{return self.finish(value,target,signs,initial_degree)}if moments.count==0{return Err(Error::Verification)}if offset==self.max_queries{break}
   current=self.curve.plus(current,Some(self.base));let sign=self.source.sign(Self::pair(current)).map_err(|_|Error::Source)?;signs.push(sign);moments=roots.observe(offset as u64,sign)?;
  }Err(Error::Unresolved)
 }
}
