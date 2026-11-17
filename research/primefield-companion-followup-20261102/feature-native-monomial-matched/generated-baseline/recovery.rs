//! Polynomial recovery with one pair-specific separating character.
#[path="collapse.rs"]mod collapse;
#[path="decoder.rs"]mod decoder;
#[path="source.rs"]mod source;
#[derive(Debug,PartialEq)]pub enum Error{Domain,Compile,NonbinaryFiber,Separator,Sign,Source}
fn mul(a:u64,b:u64,r:u64)->u64{crate::meter::hit(crate::meter::Component::Compiler,crate::meter::Op::Product);((a as u128*b as u128)%r as u128)as u64}
fn pow(mut a:u64,mut n:u64,r:u64)->u64{let mut v=1;while n>0{if n&1!=0{v=mul(v,a,r);}a=mul(a,a,r);n>>=1;}v}
fn chi(x:u64,r:u64)->i8{if x==0{0}else if pow(x,(r-1)/2,r)==1{1}else{-1}}
fn signature(x:u64,r:u64,q:usize)->Vec<i8>{(0..q).map(|a|chi((x+a as u64)%r,r)).collect()}
struct Correction{signature:Vec<i8>,offset:usize,coefficient:u64}
pub struct Program{r:u64,q:usize,max_extra:usize,terms:Vec<(u64,Vec<usize>)>,corrections:Vec<Correction>,decoder:decoder::Program}
#[derive(Debug,PartialEq)]pub struct Output{pub value:u64,pub extra_offset:Option<usize>}
impl Program{
 pub fn construct(r:u64,q:usize,max_extra:usize,max_rows:usize,max_columns:usize)->Result<Self,Error>{let _accounting_scope=crate::meter::granular(crate::meter::Stage::Compile);
  if max_extra==0||q.checked_add(max_extra).ok_or(Error::Domain)?as u128>r as u128{return Err(Error::Domain);}
  let p=collapse::construct(r,q,4,max_rows,max_columns).map_err(|_|Error::Compile)?;let mut corrections=vec![];
  for fiber in p.fibers{if fiber.len()!=2{return Err(Error::NonbinaryFiber);}let(a,b)=(fiber[0],fiber[1]);let mean=mul((a+b)%r,(r+1)/2,r);
   let offset=(q..q+max_extra).find(|&e|{let s=chi((a+e as u64)%r,r);s!=0&&chi((b+e as u64)%r,r)==-s}).ok_or(Error::Separator)?;
   let sign=chi((a+offset as u64)%r,r);let coefficient=mul((a+r-mean)%r,if sign==1{1}else{r-1},r);
   corrections.push(Correction{signature:signature(a,r,q),offset,coefficient});
  }
  let decoder=decoder::Program::new(r,q,&p.terms).map_err(|_|Error::Compile)?;Ok(Self{r,q,max_extra,terms:p.terms,corrections,decoder})
 }
 pub fn evaluate(&self,signs:&[i8],mut oracle:impl FnMut(usize)->Result<i8,Error>)->Result<Output,Error>{let _accounting_scope=crate::meter::granular(crate::meter::Stage::Decode);
  if signs.len()!=self.q||signs.iter().any(|&s|!(-1..=1).contains(&s)){return Err(Error::Sign);}
  let mut value=self.decoder.evaluate(|a|signs[a]).map_err(|_|Error::Sign)?;let mut extra_offset=None;
  if let Some(c)=self.corrections.iter().find(|c|{crate::meter::hit(crate::meter::Component::Decoder,crate::meter::Op::RuleCompare);c.signature==signs}){let s=oracle(c.offset)?;if s!=1&&s!=-1{return Err(Error::Sign);}value=(value+if s==1{c.coefficient}else{self.r-c.coefficient})%self.r;extra_offset=Some(c.offset);}
  Ok(Output{value,extra_offset})
 }
}
pub struct Source{inner:source::Source,r:u64}
pub struct Prepared<'a>{inner:source::Prepared<'a>,r:u64}
#[derive(Debug)]pub struct Recovery{pub value:u64,pub sign_values:usize,pub extra_offset:Option<usize>,pub steps:usize,pub skipped_signs:usize,pub conversions:usize,pub direct_phases:usize,pub initial_direct_initializations:usize,pub initial_conversions:usize}
impl Source{
 pub fn new(p:u64,a:u64,b:u64,r:u64,base:[u64;2],max_lanes:usize,max_window:usize)->Result<Self,Error>{let _accounting_scope=crate::meter::granular(crate::meter::Stage::Calibration);Ok(Self{inner:source::Source::new(p,a,b,r,base,max_lanes,max_window).map_err(|_|Error::Source)?,r})}
 pub fn prepare(&self)->Result<Prepared<'_>,Error>{let _accounting_scope=crate::meter::granular(crate::meter::Stage::Prepare);Ok(Prepared{inner:self.inner.prepare().map_err(|_|Error::Source)?,r:self.r})}
}
impl Prepared<'_>{
 pub fn recover(&self,p:&Program,point:Option<[u64;2]>)->Result<Recovery,Error>{let _accounting_scope=crate::meter::granular(crate::meter::Stage::Query);
  if p.r!=self.r{return Err(Error::Domain);}
  let(initial,cursor)=self.inner.begin(point,p.q,p.max_extra).map_err(|_|Error::Source)?;let mut cursor=Some(cursor);
  let(mut steps,mut skipped_signs,mut conversions,mut direct_phases)=(0,0,0,0);
  let out=p.evaluate(&initial.signs,|offset|{let _accounting_scope=crate::meter::granular(crate::meter::Stage::Extension);let mut last=None;for e in p.q..=offset{let c=cursor.take().ok_or(Error::Source)?;let(extension,next)=if e==offset{c.advance()}else{c.skip_sign()}.map_err(|_|Error::Source)?;
    steps+=1;skipped_signs+=usize::from(extension.sign.is_none());conversions+=extension.converted_points;direct_phases+=usize::from(extension.method=="direct");last=extension.sign;cursor=Some(next);
   }last.ok_or(Error::Source)})?;
  Ok(Recovery{value:out.value,sign_values:p.q+usize::from(out.extra_offset.is_some()),extra_offset:out.extra_offset,steps,skipped_signs,conversions,direct_phases,initial_direct_initializations:initial.direct_initializations,initial_conversions:initial.converted_points})
 }
}

#[cfg(test)]#[path="../../feature-native-transport-recovery/tests.rs"]mod tests;
