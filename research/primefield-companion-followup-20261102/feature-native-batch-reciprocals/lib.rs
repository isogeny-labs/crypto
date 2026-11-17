//! Polynomial recovery with one pair-specific separating character.
#[path="../feature-native-quartic-collapse/lib.rs"]mod collapse;
#[path="../feature-native-centered-bitplanes/decoder.rs"]mod decoder;
#[path="routed/source.rs"]mod source;
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
pub struct Source{inner:source::Source,r:u64}
pub struct Prepared<'a>{inner:source::PreparedSource<'a>,r:u64}
#[derive(Debug)]pub struct Recovery{pub value:u64,pub sign_values:usize,pub extra_offset:Option<usize>,pub steps:usize,pub skipped_signs:usize,pub conversions:usize,pub direct_phases:usize}
impl Source{
 pub fn new(p:u64,a:u64,b:u64,r:u64,base:[u64;2])->Result<Self,Error>{Ok(Self{inner:source::Source::new(p,a,b,r,base).map_err(|_|Error::Source)?,r})}
 pub fn prepare(&self)->Result<Prepared<'_>,Error>{Ok(Prepared{inner:self.inner.prepare().map_err(|_|Error::Source)?,r:self.r})}
}
impl Prepared<'_>{
 pub fn recover(&self,p:&Program,point:Option<[u64;2]>)->Result<Recovery,Error>{
  if p.r!=self.r{return Err(Error::Domain);}
  let(initial,cursor)=self.inner.begin(point,p.q,p.max_extra).map_err(|_|Error::Source)?;let mut cursor=Some(cursor);
  let(mut steps,mut skipped_signs,mut conversions,mut direct_phases)=(0,0,0,0);
  let out=p.evaluate(&initial.signs,|offset|{let mut last=None;for e in p.q..=offset{let c=cursor.take().ok_or(Error::Source)?;let(extension,next)=if e==offset{c.advance()}else{c.skip_sign()}.map_err(|_|Error::Source)?;
    steps+=1;skipped_signs+=usize::from(extension.sign.is_none());conversions+=extension.converted_points;direct_phases+=usize::from(extension.method=="direct");last=extension.sign;cursor=Some(next);
   }last.ok_or(Error::Source)})?;
  Ok(Recovery{value:out.value,sign_values:p.q+usize::from(out.extra_offset.is_some()),extra_offset:out.extra_offset,steps,skipped_signs,conversions,direct_phases})
 }
}
#[cfg(test)]mod tests{
 use super::*;
 fn program()->Program{Program::construct(479,14,2,479,2500).unwrap()}
 // Checker-only known-answer point generation; never called by recovery.
 fn add(a:Option<[u64;2]>,b:Option<[u64;2]>)->Option<[u64;2]>{let(Some([x,y]),Some([u,v]))=(a,b)else{return a.or(b)};if x==u&&(y+v)%509==0{return None;}let(n,d)=if x==u{((3*x*x+2)%509,2*y%509)}else{((v+509-y)%509,(u+509-x)%509)};let m=n*pow(d,507,509)%509;let xx=(m*m+1018-x-u)%509;Some([xx,(m*(x+509-xx)+509-y)%509])}
 fn point(mut k:u64)->Option<[u64;2]>{let(mut out,mut base)=(None,Some([3,68]));while k>0{if k&1!=0{out=add(out,base);}base=add(base,base);k>>=1;}out}
 #[test]fn complete_identity_and_offsets(){let p=program();let mut counts=[0usize;2];for x in 0..479{let o=p.evaluate(&signature(x,479,14),|e|{counts[e-14]+=1;Ok(chi((x+e as u64)%479,479))}).unwrap();assert_eq!(o.value,x);}assert_eq!(counts,[12,4]);println!("field_inputs=479 offset14_requests=12 offset15_requests=4 sign_values=6722");
  if let Ok(dir)=std::env::var("QUARTIC_RECOVERY_OUT"){let terms=p.terms.iter().map(|(c,l)|format!("{}\t{}\n",c,l.iter().map(|a|a.to_string()).collect::<Vec<_>>().join(","))).collect::<String>();std::fs::write(format!("{dir}/program.tsv"),terms).unwrap();let rules=p.corrections.iter().map(|c|format!("{}\t{}\t{}\n",c.offset,c.coefficient,c.signature.iter().map(|s|s.to_string()).collect::<Vec<_>>().join(","))).collect::<String>();std::fs::write(format!("{dir}/corrections.tsv"),rules).unwrap();}
 }
 #[test]fn all_exceptional_public_points_and_original_fixtures(){let p=program();let owner=Source::new(509,2,10,479,[3,68]).unwrap();let prepared=owner.prepare().unwrap();let mut xs=vec![0,1,2,478];
  for line in include_str!("../feature-native-quartic-collapse/fibers.tsv").lines(){xs.extend(line.split('\t').map(|x|x.parse::<u64>().unwrap()));}xs.sort_unstable();xs.dedup();assert_eq!(xs.len(),20);
  let mut rows=String::from("scalar\tx\ty\trecovered\tsign_values\textra_offset\tsteps\tskipped_signs\tconverted_points\tdirect_phases\n");let(mut extras,mut skips,mut steps)=(0,0,0);
  for x in xs{let q=point(x);let before=source::sign_calls();let out=prepared.recover(&p,q).unwrap();let actual=source::sign_calls()-before;assert_eq!(out.value,x);assert_eq!(actual,out.sign_values-usize::from((0..14).any(|a|(x+a)%479==0)));assert_eq!(out.direct_phases,0);extras+=usize::from(out.extra_offset.is_some());skips+=out.skipped_signs;steps+=out.steps;
   let(a,b)=q.map(|[a,b]|(a as i64,b as i64)).unwrap_or((-1,-1));rows.push_str(&format!("{x}\t{a}\t{b}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",out.value,out.sign_values,out.extra_offset.map(|e|e as i64).unwrap_or(-1),out.steps,out.skipped_signs,out.conversions,out.direct_phases));
  }assert_eq!((extras,skips,steps),(16,4,20));
  for line in include_str!("../feature-native-recovery/targets.tsv").lines(){let f:Vec<i64>=line.split('\t').map(|x|x.parse().unwrap()).collect();assert_eq!(point(f[0]as u64),if f[1]<0{None}else{Some([f[1]as u64,f[2]as u64])});}
  println!("public_point_recoveries=20 extra_signs=16 skipped_signs=4 geometry_steps=20");if let Ok(dir)=std::env::var("QUARTIC_RECOVERY_OUT"){std::fs::write(format!("{dir}/recovery.tsv"),rows).unwrap();}
 }
 #[test]fn skip_has_no_character_consumer_and_matches_full_window(){let s=source::Source::new(509,2,10,479,[3,68]).unwrap();let p=s.prepare().unwrap();for x in [25,173,292,440]{let q=point(x);let full=p.window(q,16).unwrap();let(_,cursor)=p.begin(q,14,2).unwrap();let before=(source::sign_calls(),source::phase_calls());let(e,cursor)=cursor.skip_sign().unwrap();assert_eq!((source::sign_calls(),source::phase_calls()),before);assert_eq!(e.sign,None);assert_eq!(e.seed_pair,full.seed_pairs[14]);let(e,cursor)=cursor.advance().unwrap();assert_eq!(e.sign,Some(full.signs[15]));assert_eq!(e.seed_pair,full.seed_pairs[15]);assert_eq!(source::sign_calls(),before.0+1);assert_eq!(source::phase_calls(),before.1);assert!(matches!(cursor.skip_sign(),Err(source::Error::Limit)));}}
 #[test]fn invalid_inputs_and_lazy_oracle(){assert!(matches!(Program::construct(479,14,1,479,2500),Err(Error::Separator)));assert!(Program::construct(479,14,0,479,2500).is_err());let p=program();assert_eq!(p.evaluate(&signature(0,479,14),|_|panic!("unused")),Ok(Output{value:0,extra_offset:None}));assert_eq!(p.evaluate(&signature(25,479,14),|_|Ok(0)),Err(Error::Sign));assert_eq!(p.evaluate(&signature(25,479,14),|_|Err(Error::Source)),Err(Error::Source));assert_eq!(p.evaluate(&[2;14],|_|panic!("invalid")),Err(Error::Sign));}
}
#[cfg(test)]#[path="../feature-native-quartic-recovery/lib.rs"]mod frozen;
#[cfg(test)]mod batch_integration_tests{
 use super::*;
 #[test]fn twenty_recoveries_match_frozen_and_count_all_value_inverses(){
  let p=Program::construct(479,14,2,479,2500).unwrap();let owner=Source::new(509,2,10,479,[3,68]).unwrap();let prepared=owner.prepare().unwrap();
  let oldp=frozen::Program::construct(479,14,2,479,2500).unwrap();let olds=frozen::Source::new(509,2,10,479,[3,68]).unwrap();let old=olds.prepare().unwrap();let mut rows=String::from("scalar\tfinite_prefix\tvalue_inverse_calls\textra_signs\tsteps\tskipped_signs\n");let mut total=0;
  for line in include_str!("../feature-native-quartic-recovery/recovery.tsv").lines().skip(1){let f:Vec<i64>=line.split('\t').map(|s|s.parse().unwrap()).collect();let q=if f[1]<0{None}else{Some([f[1]as u64,f[2]as u64])};let before=source::division_counts();let got=prepared.recover(&p,q).unwrap();let after=source::division_counts();let expected=old.recover(&oldp,q).unwrap();assert_eq!(format!("{got:?}"),format!("{expected:?}"));assert_eq!(got.value,f[0]as u64);
   let extra=usize::from(got.extra_offset.is_some());assert_eq!(after.1-before.1,1+extra);let finite=14-usize::from((0..14).any(|a|(f[0]+a)%479==0));rows.push_str(&format!("{}\t{finite}\t{}\t{extra}\t{}\t{}\n",f[0],after.1-before.1,got.steps,got.skipped_signs));total+=1;
  }assert_eq!(total,20);println!("batch_source_recoveries=20 unchanged_output_agreements=20 value_inverses=one_prefix_plus_one_per_extra_character");if let Ok(dir)=std::env::var("BATCH_RECIPROCALS_OUT"){std::fs::write(format!("{dir}/inverse-counts.tsv"),rows).unwrap();}
 }
 #[test]fn small_prime_windows_with_identity_match_single_sign_reference(){
  for(p,a,b,r,base)in[(5,2,1,7,[0,1]),(7,0,3,13,[1,2]),(509,2,10,479,[3,68])]{let s=source::Source::new(p,a,b,r,base).unwrap();let ready=s.prepare().unwrap();for length in [1,2,5]{for q in [None,Some(base)]{let before=source::division_counts();let out=ready.window(q,length).unwrap();let after=source::division_counts();assert_eq!(after.1-before.1,usize::from(out.shifts.iter().any(|q|q.is_some())));for(i,q)in out.shifts.iter().enumerate(){assert_eq!(out.signs[i],s.sign(*q).unwrap());}}}}
 }
}
