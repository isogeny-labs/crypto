//! Development paired cursor: projection plus unchanged full-ratio validator.
mod source;
mod subgroup;
#[path="../feature-native-monomial-recovery/source.rs"]mod full;
type Point=Option<[u64;2]>;
#[derive(Debug)]pub enum Error{Source,Disagreement}
pub struct Source{p:u64,odd:u64,projected:source::Source,full:full::Source}
pub struct Prepared<'a>{p:u64,odd:u64,projected:source::Prepared<'a>,full:full::Prepared<'a>}
pub struct Cursor<'b,'a>{p:u64,odd:u64,projected:source::Cursor<'b,'a>,full:full::Cursor<'b,'a>}
fn mul(a:u64,b:u64,p:u64)->u64{((a as u128*b as u128)%p as u128)as u64}
fn pow(mut a:u64,mut e:u64,p:u64)->u64{let mut z=1;while e>0{if e&1==1{z=mul(z,a,p);}a=mul(a,a,p);e>>=1;}z}
fn ratio_matches(p:u64,odd:u64,point:Point,got:[u64;2],raw:[u64;2])->bool{
 if point.is_none(){return got==[0,1]&&raw==[0,1];}
 !got.contains(&0)&&!raw.contains(&0)&&mul(got[0],pow(raw[1],odd,p),p)==mul(got[1],pow(raw[0],odd,p),p)
}
impl Source{
 pub fn new(p:u64,a:u64,b:u64,r:u64,base:[u64;2],lanes:usize,window:usize)->Result<Self,Error>{
  let projected=source::Source::new(p,a,b,r,base,lanes,window).map_err(|_|Error::Source)?;
  let full=full::Source::new(p,a,b,r,base,lanes,window).map_err(|_|Error::Source)?;
  Ok(Self{p,odd:(p-1)/(1u64<<(p-1).trailing_zeros()),projected,full})
 }
 pub fn prepare(&self)->Result<Prepared<'_>,Error>{Ok(Prepared{p:self.p,odd:self.odd,projected:self.projected.prepare().map_err(|_|Error::Source)?,full:self.full.prepare().map_err(|_|Error::Source)?})}
}
impl<'a> Prepared<'a>{
 pub fn begin<'b>(&'b self,q:Point,length:usize,extra:usize)->Result<(source::Window,Cursor<'b,'a>),Error>{
  let(w,c)=self.projected.begin(q,length,extra).map_err(|_|Error::Source)?;
  let(v,d)=self.full.begin(q,length,extra).map_err(|_|Error::Source)?;
  if w.points!=v.points||w.signs!=v.signs||w.ratios.len()!=v.ratios.len(){return Err(Error::Disagreement);}
  for i in 0..w.ratios.len(){if !ratio_matches(self.p,self.odd,w.points[i],w.ratios[i],v.ratios[i]){return Err(Error::Disagreement);}}
  Ok((w,Cursor{p:self.p,odd:self.odd,projected:c,full:d}))
 }
}
impl<'b,'a> Cursor<'b,'a>{
 pub fn advance(self)->Result<(source::Extension,Self),Error>{self.step(true)}
 pub fn skip_sign(self)->Result<(source::Extension,Self),Error>{self.step(false)}
 fn step(self,evaluate:bool)->Result<(source::Extension,Self),Error>{
  let(a,c)=if evaluate{self.projected.advance()}else{self.projected.skip_sign()}.map_err(|_|Error::Source)?;
  let(b,d)=if evaluate{self.full.advance()}else{self.full.skip_sign()}.map_err(|_|Error::Source)?;
  if a.point!=b.point||a.sign!=b.sign||a.method!=b.method{return Err(Error::Disagreement);}
  match(a.ratio,b.ratio){(Some(x),Some(y))=>if !ratio_matches(self.p,self.odd,a.point,x,y){return Err(Error::Disagreement);},(None,None)=>{},_=>return Err(Error::Disagreement)}
  Ok((a,Self{p:self.p,odd:self.odd,projected:c,full:d}))
 }
}
#[cfg(test)]mod tests;
