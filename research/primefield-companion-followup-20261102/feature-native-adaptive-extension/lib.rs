//! Reusable adaptive recovery with bounded incremental geometry/seed cursors.
#[path="routed/source.rs"]mod source;
#[path="../feature-native-adaptive-fibers/lib.rs"]pub mod adaptive;
pub struct Source{inner:source::Source,r:u64}
pub struct Prepared<'a>{inner:source::PreparedSource<'a>,r:u64}
#[derive(Debug)]pub struct Recovery{pub value:u64,pub extra_signs:usize,pub extension:Option<source::Extension>}
impl Source{
 pub fn new(p:u64,a:u64,b:u64,r:u64,base:[u64;2])->Result<Self,adaptive::Error>{Ok(Self{inner:source::Source::new(p,a,b,r,base).map_err(|_|adaptive::Error::Source)?,r})}
 pub fn prepare(&self)->Result<Prepared<'_>,adaptive::Error>{Ok(Prepared{inner:self.inner.prepare().map_err(|_|adaptive::Error::Source)?,r:self.r})}
}
impl Prepared<'_>{
 pub fn recover(&self,program:&adaptive::AdaptiveProgram,q:Option<[u64;2]>)->Result<Recovery,adaptive::Error>{
  if program.r!=self.r{return Err(adaptive::Error::Domain);}
  let(initial,cursor)=self.inner.begin(q,program.q,1).map_err(|_|adaptive::Error::Source)?;
  let mut cursor=Some(cursor);let mut extension=None;
  let decoded=program.evaluate(&initial.signs,||{let(next,_)=cursor.take().ok_or(adaptive::Error::Source)?.advance().map_err(|_|adaptive::Error::Source)?;let sign=next.sign;extension=Some(next);Ok(sign)})?;
  Ok(Recovery{value:decoded.value,extra_signs:decoded.extra_signs,extension})
 }
}
#[cfg(test)]mod tests{
 use super::*;
 fn fixtures()->Vec<(u64,Option<[u64;2]>)>{include_str!("../feature-native-recovery/targets.tsv").lines().map(|line|{let f:Vec<i64>=line.split('\t').map(|s|s.parse().unwrap()).collect();(f[0]as u64,if f[1]<0{None}else{Some([f[1]as u64,f[2]as u64])})}).collect()}
 #[test]fn reusable_adaptive_recovery(){
  let program=adaptive::AdaptiveProgram::construct(479,15,&[(25,173),(292,440)],479,576).unwrap();
  let owner=Source::new(509,2,10,479,[3,68]).unwrap();let prepared=owner.prepare().unwrap();
  let reference=adaptive::PublicSource::new(509,2,10,479,[3,68]).unwrap();let mut extra=0;let mut rows=String::from("scalar\trecovered\textra_signs\tmethod\tlane_updates\tconverted_points\tbatches\n");
  for (scalar,q)in fixtures(){let out=prepared.recover(&program,q).unwrap();assert_eq!(out.value,scalar);assert_eq!(reference.recover(&program,q).unwrap().value,out.value);extra+=out.extra_signs;
   if let Some(e)=out.extension{assert_eq!(e.method,"recurrence");assert_eq!(e.conversion_batches,1);rows.push_str(&format!("{scalar}\t{}\t1\t{}\t{}\t{}\t{}\n",out.value,e.method,e.lane_updates,e.converted_points,e.conversion_batches));}else{rows.push_str(&format!("{scalar}\t{}\t0\tnone\t0\t0\t0\n",out.value));}
  }assert_eq!(extra,4);println!("reusable_source_integrations=8 extra_signs=4");
  if let Ok(path)=std::env::var("EXTENSION_TRACE_OUT"){std::fs::write(path,rows).unwrap();}
 }
 #[test]fn cursor_matches_whole_windows_and_direct_phases(){
  let s=source::Source::new(509,2,10,479,[3,68]).unwrap();let p=s.prepare().unwrap();let mut extensions=0;
  for(_,q)in fixtures(){let full=p.window(q,18).unwrap();for cut in [1usize,2,14,15]{
   let(prefix,mut cursor)=p.begin(q,cut,18-cut).unwrap();assert_eq!(prefix.signs,full.signs[..cut]);
   for i in cut..18{let before=source::phase_calls();let(e,next)=cursor.advance().unwrap();let after=source::phase_calls();assert_eq!(after-before,usize::from(e.method=="direct"));assert_eq!(e.sign,full.signs[i]);assert_eq!(e.seed_pair,full.seed_pairs[i]);assert_eq!(e.point,full.shifts[i]);assert_eq!(e.method,full.methods[i]);cursor=next;extensions+=1;}
   assert!(matches!(cursor.advance(),Err(source::Error::Limit)));
  }}assert_eq!(extensions,320);println!("whole_window_extension_checks={extensions}");
 }
 #[test]fn anchor_crossings_small_curves_and_limits(){
  let mut extensions=0;let mut anchors=0;let mut recurrences=0;
  for(p,a,b,r,base)in [(5,2,1,7,[0,1]),(7,0,3,13,[1,2])]{let s=source::Source::new(p,a,b,r,base).unwrap();let prepared=s.prepare().unwrap();
   let total=(2*r+3)as usize;let full=prepared.window(None,total).unwrap();let(_,mut cursor)=prepared.begin(None,1,total-1).unwrap();
   for i in 1..total{let before=source::phase_calls();let(e,next)=cursor.advance().unwrap();assert_eq!(source::phase_calls(),before);assert_eq!(e.sign,full.signs[i]);assert_eq!(e.seed_pair,full.seed_pairs[i]);assert_eq!(e.point,full.shifts[i]);assert_eq!(e.conversion_batches,usize::from(e.point.is_some()));if e.method=="recurrence"{recurrences+=1;}else{anchors+=1;}cursor=next;extensions+=1;}
   assert!(matches!(cursor.advance(),Err(source::Error::Limit)));assert!(prepared.begin(None,0,1).is_err());assert!(prepared.begin(Some([0,0]),2,1).is_err());
   let(_,empty)=prepared.begin(None,1,0).unwrap();assert!(matches!(empty.advance(),Err(source::Error::Limit)));
  }assert_eq!(extensions,44);assert!(anchors>0&&recurrences>0);println!("small_curve_extensions={extensions} anchors={anchors} recurrences={recurrences}");
 }
}
