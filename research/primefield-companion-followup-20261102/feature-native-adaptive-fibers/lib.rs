//! Sparse-fiber adaptive character decoding; no target scalar or answer table.
mod target;
#[path="../feature-native-centered-bitplanes/decoder.rs"]mod decoder;
#[path="../feature-native-seed-factor/routed/source.rs"]mod source;
#[derive(Debug,PartialEq)]pub enum Error{Domain,Limit,Pairs,Signature,Separator,Parity,Span,Sign,Source}
fn mul(a:u64,b:u64,r:u64)->u64{((a as u128*b as u128)%r as u128)as u64}
fn pow(mut a:u64,mut n:u64,r:u64)->u64{let mut v=1;while n>0{if n&1!=0{v=mul(v,a,r);}a=mul(a,a,r);n>>=1;}v}
fn chi(x:u64,r:u64)->i8{if x==0{0}else if pow(x,(r-1)/2,r)==1{1}else{-1}}
fn signature(x:u64,r:u64,q:usize)->Vec<i8>{(0..q).map(|a|chi((x+a as u64)%r,r)).collect()}
struct Pattern{signs:Vec<i8>,coefficient:u64}
pub struct AdaptiveProgram{pub r:u64,pub q:usize,pub terms:Vec<(u64,Vec<usize>)>,pub rank:usize,pub columns:usize,patterns:Vec<Pattern>,compiled:decoder::Program}
#[derive(Debug,PartialEq)]pub struct Output{pub value:u64,pub extra_signs:usize}
impl AdaptiveProgram{
 /// Caller certifies r prime. Supplied exceptional fibers must be disjoint pairs,
 /// complete fibers of the first q signs, and separated by sign offset q.
 /// The centered pair-mean target must be odd and in the cubic feature span.
 pub fn construct(r:u64,q:usize,pairs:&[(u64,u64)],max_rows:usize,max_columns:usize)->Result<Self,Error>{
  if r<3||r>u64::MAX/2||r&1==0||q==0||q>63||q as u64>=r{return Err(Error::Domain);}
  let rows=usize::try_from(r).map_err(|_|Error::Limit)?;if rows>max_rows{return Err(Error::Limit);}
  if pairs.len()>rows/2{return Err(Error::Pairs);}
  let half=(r+1)/2;let center=mul((q-1)as u64,half,r);
  let mut seen=std::collections::BTreeSet::new();let mut patterns:Vec<Pattern>=vec![];
  let mut rhs=vec![0;r as usize];rhs[1]=1;
  for &(a,b)in pairs{
   if a>=r||b>=r||a==b||!seen.insert(a)||!seen.insert(b){return Err(Error::Pairs);}
   let signs=signature(a,r,q);if signs!=signature(b,r,q)||patterns.iter().any(|p|p.signs==signs){return Err(Error::Signature);}
   let extra=chi((a+q as u64)%r,r);if extra==0||chi((b+q as u64)%r,r)!=-extra{return Err(Error::Separator);}
   let mean=mul((a+b)%r,half,r);let coefficient=mul((a+r-mean)%r,if extra==1{1}else{r-1},r);
   patterns.push(Pattern{signs,coefficient});
   for x in [a,b]{let c=(mean+r-x)%r;let t=(x+center)%r;
    // delta_t(Z)=1-(Z-t)^(r-1); binomial coefficients give t^(r-1-k).
    let mut power=1;for k in(0..rows).rev(){rhs[k]=(rhs[k]+r-mul(c,power,r))%r;power=mul(power,t,r);}rhs[0]=(rhs[0]+c)%r;
   }
  }
  // Establish complete exceptional fibers without retaining a scalar answer map.
  for x in 0..r{let s=signature(x,r,q);for(p,&(a,b))in patterns.iter().zip(pairs){if p.signs==s&&x!=a&&x!=b{return Err(Error::Signature);}}}
  if rhs.iter().step_by(2).any(|&v|v!=0){return Err(Error::Parity);}
  let solved=target::construct_target(r,q,&rhs,max_rows,max_columns).map_err(|_|Error::Span)?;
  let compiled=decoder::Program::new(r,q,&solved.terms).map_err(|_|Error::Domain)?;
  Ok(Self{r,q,terms:solved.terms,rank:solved.rank,columns:solved.columns,patterns,compiled})
 }
 /// Reads only supplied public-source signs. The extra oracle is called at most
 /// once, only when a complete exceptional signature matches.
 pub fn evaluate(&self,signs:&[i8],mut extra:impl FnMut()->Result<i8,Error>)->Result<Output,Error>{
  if signs.len()!=self.q||signs.iter().any(|&s|!(-1..=1).contains(&s)){return Err(Error::Sign);}
  let mut value=self.compiled.evaluate(|a|signs[a]).map_err(|_|Error::Sign)?;
  let mut extra_signs=0;
  if let Some(p)=self.patterns.iter().find(|p|p.signs==signs){let sign=extra()?;if sign!=1&&sign!=-1{return Err(Error::Sign);}value=(value+if sign==1{p.coefficient}else{self.r-p.coefficient})%self.r;extra_signs=1;}
  Ok(Output{value,extra_signs})
 }
}
/// Prime-field point adapter. Source.setup and public scalar multiplication are
/// independent of the unknown logarithm. The fallback computes one new sign.
pub struct PublicSource{inner:source::Source,p:u64,a:u64,r:u64,base:[u64;2]}
impl PublicSource{
 pub fn new(p:u64,a:u64,b:u64,r:u64,base:[u64;2])->Result<Self,Error>{Ok(Self{inner:source::Source::new(p,a,b,r,base).map_err(|_|Error::Source)?,p,a,r,base})}
 fn next(&self,point:Option<[u64;2]>)->Result<Option<[u64;2]>,Error>{
  let Some([x,y])=point else{return Ok(Some(self.base));};let [u,v]=self.base;let p=self.p;
  if x==u&&(y+v)%p==0{return Ok(None);}
  let(n,d)=if x==u{((mul(3,mul(x,x,p),p)+self.a)%p,mul(2,y,p))}else{((v+p-y)%p,(u+p-x)%p)};
  let inv=pow(d,p-2,p);if mul(d,inv,p)!=1{return Err(Error::Source);}let slope=mul(n,inv,p);let xx=((mul(slope,slope,p)+p-x)%p+p-u)%p;let yy=(mul(slope,(x+p-xx)%p,p)+p-y)%p;Ok(Some([xx,yy]))
 }
 pub fn recover(&self,program:&AdaptiveProgram,point:Option<[u64;2]>)->Result<Output,Error>{
  if self.r!=program.r{return Err(Error::Domain);}
  let prepared=self.inner.prepare().map_err(|_|Error::Source)?;let initial=prepared.window(point,program.q).map_err(|_|Error::Source)?;
  program.evaluate(&initial.signs,||self.inner.sign(self.next(*initial.shifts.last().ok_or(Error::Source)?)?).map_err(|_|Error::Source))
 }
}
#[cfg(test)]mod tests{
 use super::*;
 fn program()->AdaptiveProgram{AdaptiveProgram::construct(479,15,&[(25,173),(292,440)],479,576).unwrap()}
 #[test]fn complete_field_identity_and_lazy_calls(){
  let p=program();let mut calls=0;for x in 0..479{let output=p.evaluate(&signature(x,479,15),||{calls+=1;Ok(chi((x+15)%479,479))}).unwrap();assert_eq!(output.value,x);assert_eq!(output.extra_signs,usize::from([25,173,292,440].contains(&x)));}assert_eq!(calls,4);
  assert_eq!(p.patterns.iter().map(|p|p.coefficient).collect::<Vec<_>>(),vec![405,405]);
  println!("all_inputs=479 extra_signs={calls} terms={} rank={} columns={}",p.terms.len(),p.rank,p.columns);
  if let Ok(path)=std::env::var("ADAPTIVE_PACKET_OUT"){let mut text=String::new();for(c,label)in &p.terms{text.push_str(&format!("{}\t{}\n",c,label.iter().map(|a|a.to_string()).collect::<Vec<_>>().join(",")));}std::fs::write(path,text).unwrap();}
 }
 #[test]fn public_point_integration(){
  let p=program();let source=PublicSource::new(509,2,10,479,[3,68]).unwrap();let mut calls=0;
  for line in include_str!("../feature-native-recovery/targets.tsv").lines(){let f:Vec<i64>=line.split('\t').map(|v|v.parse().unwrap()).collect();let q=if f[1]<0{None}else{Some([f[1]as u64,f[2]as u64])};let out=source.recover(&p,q).unwrap();assert_eq!(out.value,f[0]as u64);calls+=out.extra_signs;}
  assert_eq!(calls,4);println!("source_integrations=8 extra_signs={calls}");
 }
 #[test]fn negative_controls(){
  assert!(matches!(AdaptiveProgram::construct(479,15,&[(25,173)],478,576),Err(Error::Limit)));
  assert!(matches!(AdaptiveProgram::construct(479,15,&[(25,25)],479,576),Err(Error::Pairs)));
  assert!(matches!(AdaptiveProgram::construct(479,15,&[(25,174)],479,576),Err(Error::Signature)));
  assert!(matches!(AdaptiveProgram::construct(479,15,&[(25,173)],479,576),Err(Error::Parity)));
  assert!(matches!(AdaptiveProgram::construct(7,1,&[(1,2)],7,5),Err(Error::Separator)|Err(Error::Signature)));
  let p=program();assert_eq!(p.evaluate(&[2;15],||panic!("must not call")),Err(Error::Sign));
  assert_eq!(p.evaluate(&signature(25,479,15),||Ok(0)),Err(Error::Sign));
  assert_eq!(p.evaluate(&signature(25,479,15),||Err(Error::Source)),Err(Error::Source));
  assert_eq!(p.evaluate(&signature(1,479,15),||panic!("unneeded sign")),Ok(Output{value:1,extra_signs:0}));
 }
}
