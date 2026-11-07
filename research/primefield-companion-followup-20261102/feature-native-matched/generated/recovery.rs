//! Fresh public-point source -> compiled character program -> verified scalar.
//! Prime field/order and coefficient-identity certification remain explicit inputs.
#[path="source.rs"] mod source;
#[path="decoder.rs"] mod signmask;
#[derive(Clone,Copy,Debug)] pub struct CurveSpec {pub p:u64,pub a:u64,pub b:u64,pub r:u64,pub base:[u64;2]}
#[derive(Debug,PartialEq)] pub enum Error {Program,Source,Verification}
pub struct Decoder {curve:CurveSpec,width:usize,source:source::Source,program:signmask::Program}
pub struct PreparedDecoder<'a>{decoder:&'a Decoder,source:source::PreparedSource<'a>}
#[derive(Debug,PartialEq)] pub struct Recovery {pub scalar:u64,pub sign_queries:usize,pub direct_phases:usize,pub seed_recurrences:usize,pub seed_anchors:usize,pub verification_add_calls:usize,pub verification_double_calls:usize}
fn fm(a:u64,b:u64,p:u64)->u64{crate::meter::hit(crate::meter::Component::Verifier,crate::meter::Op::Product);((a as u128*b as u128)%p as u128)as u64}
fn inv(a:u64,p:u64)->Result<u64,Error>{crate::meter::hit(crate::meter::Component::Verifier,crate::meter::Op::Inverse);if a==1{crate::meter::hit(crate::meter::Component::Verifier,crate::meter::Op::UnitInverse);}let(mut r,mut s,mut x,mut y)=(p as i128,a as i128,0i128,1i128);while s!=0{crate::meter::hit(crate::meter::Component::Verifier,crate::meter::Op::EuclidStep);let q=r/s;(r,s)=(s,r-q*s);(x,y)=(y,x-q*y);}if r==1{Ok(x.rem_euclid(p as i128)as u64)}else{Err(Error::Verification)}}
// Independent affine verification; does not reuse source geometry or seed state.
fn add(c:CurveSpec,left:Option<[u64;2]>,right:Option<[u64;2]>)->Result<Option<[u64;2]>,Error>{
 let([x,y],[u,v])=match(left,right){(None,b)=>return Ok(b),(a,None)=>return Ok(a),(Some(a),Some(b))=>(a,b)};
 let p=c.p;if x==u&&crate::meter::value(crate::meter::Component::Verifier,crate::meter::Op::AddSub,(y+v)%p)==0{return Ok(None);}
 let(n,d)=if x==u{(crate::meter::value(crate::meter::Component::Verifier,crate::meter::Op::AddSub,(fm(3,fm(x,x,p),p)+c.a)%p),fm(2,y,p))}else{(crate::meter::value(crate::meter::Component::Verifier,crate::meter::Op::AddSub,(v+p-y)%p),crate::meter::value(crate::meter::Component::Verifier,crate::meter::Op::AddSub,(u+p-x)%p))};
 let slope=fm(n,inv(d,p)?,p);let xx=crate::meter::value(crate::meter::Component::Verifier,crate::meter::Op::AddSub,(fm(slope,slope,p)+p-x)%p);let xx=crate::meter::value(crate::meter::Component::Verifier,crate::meter::Op::AddSub,(xx+p-u)%p);let yy=crate::meter::value(crate::meter::Component::Verifier,crate::meter::Op::AddSub,(fm(slope,crate::meter::value(crate::meter::Component::Verifier,crate::meter::Op::AddSub,(x+p-xx)%p),p)+p-y)%p);Ok(Some([xx,yy]))
}
fn multiply(c:CurveSpec,mut scalar:u64)->Result<(Option<[u64;2]>,usize,usize),Error>{
 let(mut acc,mut point)=(None,Some(c.base));let(mut additions,mut doubles)=(0,0);
 while scalar!=0{if scalar&1!=0{acc=add(c,acc,point)?;additions+=1;}scalar>>=1;if scalar!=0{point=add(c,point,point)?;doubles+=1;}}
 Ok((acc,additions,doubles))
}
impl Decoder {
 /// Compiles a supplied program for this exact subgroup modulus. This constructor
 /// validates representation, not its identity; recovery always verifies [k]P=Q.
 pub fn new(curve:CurveSpec,width:usize,terms:&[(u64,Vec<usize>)])->Result<Self,Error>{let _meter_stage=crate::meter::enter(crate::meter::Stage::Setup);
  let program=signmask::Program::new(curve.r,width,terms).map_err(|_|Error::Program)?;
  let source=source::Source::new(curve.p,curve.a,curve.b,curve.r,curve.base).map_err(|_|Error::Source)?;
  Ok(Self{curve,width,source,program})
 }
 pub fn prepare(&self)->Result<PreparedDecoder<'_>,Error>{let _meter_stage=crate::meter::enter(crate::meter::Stage::Prepare);Ok(PreparedDecoder{decoder:self,source:self.source.prepare().map_err(|_|Error::Source)?})}
 /// Exact previously checked repaired packet, embedded at compile time.
 pub fn repaired_479()->Result<Self,Error>{let _meter_stage=crate::meter::enter(crate::meter::Stage::Setup);Self::new(CurveSpec{p:509,a:2,b:10,r:479,base:[3,68]},16,&packet()?)}
}
fn packet()->Result<Vec<(u64,Vec<usize>)>,Error>{
 let mut terms=Vec::new();
 for line in include_str!("../../feature-native-signmask/program.tsv").lines(){let(c,rest)=line.split_once('\t').ok_or(Error::Program)?;let c=c.parse().map_err(|_|Error::Program)?;let offsets=if rest.is_empty(){Vec::new()}else{rest.split(',').map(|s|s.parse().map_err(|_|Error::Program)).collect::<Result<Vec<usize>,Error>>()?};terms.push((c,offsets));}
 if terms.len()!=477{return Err(Error::Program);}Ok(terms)
}
impl PreparedDecoder<'_>{
 /// Only the public point enters the online call. No expected scalar, signs,
 /// coefficient lookup by target, or search fallback is accepted.
 pub fn recover(&self,q:Option<[u64;2]>)->Result<Recovery,Error>{let _meter_stage=crate::meter::enter(crate::meter::Stage::Query);
  let window=self.source.window(q,self.decoder.width).map_err(|_|Error::Source)?;
  let mut sign_queries=0;let scalar=self.decoder.program.evaluate(|offset|{sign_queries+=1;window.signs.get(offset).copied().unwrap_or(2)}).map_err(|_|Error::Program)?;
  let(point,verification_add_calls,verification_double_calls)={let _meter_verify=crate::meter::enter(crate::meter::Stage::Verify);multiply(self.decoder.curve,scalar)?};
  if point!=q{return Err(Error::Verification);}
  Ok(Recovery{scalar,sign_queries,direct_phases:window.direct_phases,seed_recurrences:window.recurrence_updates,seed_anchors:window.anchor_hits,verification_add_calls,verification_double_calls})
 }
}
#[cfg(test)]mod recovery_tests {
 use super::*;
 fn cases()->Vec<(u64,Option<[u64;2]>)>{include_str!("../../feature-native-recovery/targets.tsv").lines().map(|l|{let f:Vec<i64>=l.split('\t').map(|s|s.parse().unwrap()).collect();(f[0]as u64,if f[1]<0{None}else{Some([f[1]as u64,f[2]as u64])})}).collect()}
 #[test]fn fresh_recovery_and_fiber_points(){let decoder=Decoder::repaired_479().unwrap();let prepared=decoder.prepare().unwrap();for(expected,point)in cases(){let recovered=prepared.recover(point).unwrap();assert_eq!(recovered.scalar,expected);assert_eq!(recovered.sign_queries,16);assert_eq!(recovered.direct_phases+recovered.seed_recurrences+recovered.seed_anchors,16);println!("recovery scalar={} queries={} direct={} recurrence={} anchors={} verify_add={} verify_double={}",recovered.scalar,recovered.sign_queries,recovered.direct_phases,recovered.seed_recurrences,recovered.seed_anchors,recovered.verification_add_calls,recovered.verification_double_calls);}}
 #[test]fn verification_and_program_rejection(){let curve=CurveSpec{p:509,a:2,b:10,r:479,base:[3,68]};let decoder=Decoder::repaired_479().unwrap();let prepared=decoder.prepare().unwrap();assert_eq!(prepared.recover(Some([0,0])),Err(Error::Source));let mut terms=packet().unwrap();assert_eq!(terms.pop(),Some((405,vec![15])));let wrong=Decoder::new(curve,16,&terms).unwrap();let point=cases().into_iter().find(|(k,_)|*k==173).unwrap().1;assert_eq!(wrong.prepare().unwrap().recover(point),Err(Error::Verification));assert!(matches!(Decoder::new(curve,65,&[]),Err(Error::Program)));assert!(matches!(Decoder::new(curve,16,&[(479,vec![])]),Err(Error::Program)));assert!(matches!(Decoder::new(curve,16,&[(1,vec![16])]),Err(Error::Program)));}
 #[test]fn independent_multiplier_edges(){let c=CurveSpec{p:509,a:2,b:10,r:479,base:[3,68]};assert_eq!(multiply(c,0).unwrap(),(None,0,0));assert_eq!(multiply(c,1).unwrap().0,Some([3,68]));assert_eq!(multiply(c,2).unwrap().0,Some([209,180]));assert_eq!(multiply(c,478).unwrap().0,Some([3,441]));assert_eq!(multiply(c,479).unwrap().0,None);for(k,q)in cases(){assert_eq!(multiply(c,k).unwrap().0,q);}}
}
