//! Instrumented development fixture. Counters are not a wall-clock cost model.
pub mod meter;
#[global_allocator]static ALLOCATOR:meter::Counting=meter::Counting;
#[path="generated/recovery.rs"]pub mod counted;
#[cfg(test)]#[path="../feature-native-quartic-recovery/lib.rs"]mod reference;
mod verify{
 use crate::meter::{self,Component as C,Op};
 fn mul(a:u64,b:u64)->u64{meter::hit(C::Verifier,Op::Product);((a as u128*b as u128)%509)as u64}
 fn inv(a:u64)->u64{meter::hit(C::Verifier,Op::Inverse);let(mut a,mut n,mut v)=(a,507,1);while n>0{if n&1!=0{v=mul(v,a);}n>>=1;if n>0{a=mul(a,a);}}v}
 fn add(a:Option<[u64;2]>,b:Option<[u64;2]>)->Option<[u64;2]>{let(Some([x,y]),Some([u,v]))=(a,b)else{return a.or(b)};if x==u&&(y+v)%509==0{return None;}let(n,d)=if x==u{((mul(3,mul(x,x))+2)%509,2*y%509)}else{((v+509-y)%509,(u+509-x)%509)};let m=mul(n,inv(d));let xx=(mul(m,m)+1018-x-u)%509;Some([xx,(mul(m,(x+509-xx)%509)+509-y)%509])}
 pub fn point(mut k:u64)->Option<[u64;2]>{let(mut out,mut base)=(None,Some([3,68]));while k>0{if k&1!=0{out=add(out,base);}k>>=1;if k>0{base=add(base,base);}}out}
}
#[cfg(test)]mod accounting_tests{
 use super::*;use meter::{Stage as S,Component as C,Op as O};
 fn print(label:&str,c:&meter::Counts){for(s,stage)in meter::STAGES.iter().enumerate(){for(k,component)in meter::COMPONENTS.iter().enumerate(){for(o,op)in meter::OPS.iter().enumerate(){if c[s][k][o]!=0{println!("count\t{label}\t{stage}\t{component}\t{op}\t{}",c[s][k][o]);}}}}}
 fn count(c:&meter::Counts,s:S,k:C,o:O)->u64{c[s as usize][k as usize][o as usize]}
 #[test]fn complete_path_matches_frozen_source(){
  meter::reset();let program=counted::Program::construct(479,14,2,479,2500).unwrap();let compilation=meter::snapshot();assert!(count(&compilation,S::Compile,C::Target,O::Product)>0);assert!(count(&compilation,S::Compile,C::Recurrence,O::Product)>0);print("compile",&compilation);
  meter::reset();let source=counted::Source::new(509,2,10,479,[3,68]).unwrap();let calibration=meter::snapshot();assert!(count(&calibration,S::Calibration,C::Source,O::Product)>0);print("calibration",&calibration);
  meter::reset();let prepared=source.prepare().unwrap();let prep=meter::snapshot();assert!(count(&prep,S::Prepare,C::Geometry,O::Product)>0);print("prepare",&prep);
  let rp=reference::Program::construct(479,14,2,479,2500).unwrap();let rs=reference::Source::new(509,2,10,479,[3,68]).unwrap();let ready=rs.prepare().unwrap();let(mut rows,mut points)=(0,0);
  for line in include_str!("../feature-native-quartic-recovery/recovery.tsv").lines().skip(1){let f:Vec<i64>=line.split('\t').map(|x|x.parse().unwrap()).collect();let q=if f[1]<0{None}else{Some([f[1]as u64,f[2]as u64])};
   meter::reset();let result=prepared.recover(&program,q).unwrap();let rebuilt={let _stage=meter::enter(S::Verify);verify::point(result.value)};let costs=meter::snapshot();assert_eq!(rebuilt,q);assert_eq!(result.value,f[0]as u64);let old=ready.recover(&rp,q).unwrap();assert_eq!(format!("{result:?}"),format!("{old:?}"));
   for stage in [S::Compile,S::Calibration,S::Prepare]{assert_eq!(costs[stage as usize],[[0;13];11]);}
   let finite=14-u64::from((0..14).any(|a|(f[0]+a)%479==0));assert_eq!(count(&costs,S::InitialCharacter,C::Source,O::Character),finite);assert_eq!(count(&costs,S::InitialCharacter,C::Value,O::Inverse),finite);assert_eq!(count(&costs,S::InitialCharacter,C::Polynomial,O::Inverse),0);
   let extra=u64::from(result.extra_offset.is_some());assert_eq!(count(&costs,S::ExtraCharacter,C::Source,O::Character),extra);assert_eq!(count(&costs,S::ExtraCharacter,C::Value,O::Inverse),extra);assert_eq!(count(&costs,S::ExtraSeed,C::Phase,O::ScalarInverse),0);assert_eq!(count(&costs,S::ExtraGeometry,C::Geometry,O::Inverse),result.steps as u64);assert_eq!(count(&costs,S::InitialGeometry,C::Geometry,O::Inverse),1);assert_eq!(count(&costs,S::Decode,C::Decoder,O::Popcount),128);
   assert_eq!(count(&costs,S::InitialSeed,C::Phase,O::Inverse),2*count(&costs,S::InitialSeed,C::Phase,O::ScalarInverse));assert!(count(&costs,S::Decode,C::Decoder,O::RuleCompare)<=8);assert_eq!(count(&costs,S::Decode,C::Source,O::Product),0);assert_eq!(count(&costs,S::Query,C::Geometry,O::Product),0);
   print(&format!("k{}",f[0]),&costs);rows+=1;points+=usize::from(rebuilt.is_some());
  }
  assert_eq!((rows,points),(20,19));println!("accounting_recoveries=20 frozen_source_agreements=20 public_point_verifications=20");
 }
 #[test]fn scopes_restore_and_threads_isolate(){meter::reset();meter::hit(C::Source,O::Product);assert_eq!(meter::snapshot(),[[[0;13];11];14]);{let _outer=meter::enter(S::Query);meter::hit(C::Source,O::Product);{let _inner=meter::enter(S::ExtraSeed);meter::hit(C::Source,O::Product);}meter::hit(C::Source,O::Product);}let c=meter::snapshot();assert_eq!(count(&c,S::Query,C::Source,O::Product),2);assert_eq!(count(&c,S::ExtraSeed,C::Source,O::Product),1);std::thread::spawn(||assert_eq!(meter::snapshot(),[[[0;13];11];14])).join().unwrap();}
 #[test]fn allocator_forwarding(){use std::alloc::{GlobalAlloc,Layout};meter::reset();{let _stage=meter::enter(S::Query);unsafe{let a=Layout::from_size_align(32,8).unwrap();let b=Layout::from_size_align(8,8).unwrap();let mut p=ALLOCATOR.alloc(a);assert!(!p.is_null());p.write(73);let z=ALLOCATOR.alloc_zeroed(b);assert!(!z.is_null());assert_eq!(z.read(),0);p=ALLOCATOR.realloc(p,a,64);assert!(!p.is_null());assert_eq!(p.read(),73);ALLOCATOR.dealloc(p,Layout::from_size_align(64,8).unwrap());ALLOCATOR.dealloc(z,b);}}let c=meter::snapshot();assert_eq!(count(&c,S::Query,C::Allocator,O::RequestedBytes),104);assert_eq!(count(&c,S::Query,C::Allocator,O::FreedBytes),72);}
}
