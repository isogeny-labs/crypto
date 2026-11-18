//! Polynomial recovery with one pair-specific separating character.
#[path="../feature-native-quartic-collapse/lib.rs"]mod collapse;
#[path="../feature-native-centered-bitplanes/decoder.rs"]mod decoder;
#[derive(Debug,PartialEq)]pub enum Error{Domain,Compile,NonbinaryFiber,Separator,Sign,Source}
fn mul(a:u64,b:u64,r:u64)->u64{((a as u128*b as u128)%r as u128)as u64}
fn pow(mut a:u64,mut n:u64,r:u64)->u64{let mut v=1;while n>0{if n&1!=0{v=mul(v,a,r);}a=mul(a,a,r);n>>=1;}v}
fn chi(x:u64,r:u64)->i8{if x==0{0}else if pow(x,(r-1)/2,r)==1{1}else{-1}}
fn signature(x:u64,r:u64,q:usize)->Vec<i8>{(0..q).map(|a|chi((x+a as u64)%r,r)).collect()}
struct Correction{signature:Vec<i8>,offset:usize,coefficient:u64}
pub struct Program{r:u64,q:usize,max_extra:usize,terms:Vec<(u64,Vec<usize>)>,corrections:Vec<Correction>,decoder:decoder::Program}
#[derive(Debug,PartialEq)]pub struct Output{pub value:u64,pub extra_offset:Option<usize>}
impl Program{
 pub fn construct(r:u64,q:usize,max_extra:usize,max_rows:usize,max_columns:usize)->Result<Self,Error>{
  if max_extra==0||q.checked_add(max_extra).ok_or(Error::Domain)?as u128>r as u128{return Err(Error::Domain);}
  let p=collapse::construct(r,q,4,max_rows,max_columns).map_err(|_|Error::Compile)?;let mut corrections=vec![];
  for fiber in p.fibers{if fiber.len()!=2{return Err(Error::NonbinaryFiber);}let(a,b)=(fiber[0],fiber[1]);let mean=mul((a+b)%r,(r+1)/2,r);
   let offset=(q..q+max_extra).find(|&e|{let s=chi((a+e as u64)%r,r);s!=0&&chi((b+e as u64)%r,r)==-s}).ok_or(Error::Separator)?;
   let sign=chi((a+offset as u64)%r,r);let coefficient=mul((a+r-mean)%r,if sign==1{1}else{r-1},r);
   corrections.push(Correction{signature:signature(a,r,q),offset,coefficient});
  }
  let decoder=decoder::Program::new(r,q,&p.terms).map_err(|_|Error::Compile)?;Ok(Self{r,q,max_extra,terms:p.terms,corrections,decoder})
 }
 pub fn evaluate(&self,signs:&[i8],mut oracle:impl FnMut(usize)->Result<i8,Error>)->Result<Output,Error>{
  if signs.len()!=self.q||signs.iter().any(|&s|!(-1..=1).contains(&s)){return Err(Error::Sign);}
  let mut value=self.decoder.evaluate(|a|signs[a]).map_err(|_|Error::Sign)?;let mut extra_offset=None;
  if let Some(c)=self.corrections.iter().find(|c|c.signature==signs){let s=oracle(c.offset)?;if s!=1&&s!=-1{return Err(Error::Sign);}value=(value+if s==1{c.coefficient}else{self.r-c.coefficient})%self.r;extra_offset=Some(c.offset);}
  Ok(Output{value,extra_offset})
 }
}

impl Program {pub(crate) fn shape(&self)->(u64,usize,usize){(self.r,self.q,self.max_extra)}}
#[cfg(test)]impl Program {pub(crate) fn corrupt_for_test(&mut self){self.decoder=decoder::Program::new(self.r,self.q,&[(1,vec![])]).unwrap();self.corrections.clear();}}
