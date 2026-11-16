//! Factored versus homogeneous seed counted runner. Execute only after protocol freeze and admission.
pub mod meter;
#[global_allocator] static ALLOCATOR:meter::Counting=meter::Counting;
#[path="generated-baseline/recovery.rs"] mod baseline;
#[path="generated-candidate/recovery.rs"] mod candidate;

fn emit(arm:&str,target:Option<u64>,phase:&str,scalar:Option<u64>,verified:bool,counts:meter::Counts){
 let target=target.map_or("null".to_owned(),|x|x.to_string());
 let scalar=scalar.map_or("null".to_owned(),|x|x.to_string());
 println!("{{\"schema\":\"native-seed-factor-row/v1\",\"arm\":\"{}\",\"target\":{},\"phase\":\"{}\",\"scalar\":{},\"verified\":{},\"counts\":{:?}}}",arm,target,phase,scalar,verified,counts);
}
macro_rules! query {($prepared:expr,$arm:expr,$q:expr,$expected:expr,$ok:ident)=>{{
 meter::reset();let result=$prepared.recover($q);let counts=meter::snapshot();
 let scalar=result.as_ref().ok().map(|r|r.scalar);
 let verified=scalar==Some($expected);$ok &= verified;
 emit($arm,Some($expected),"query",scalar,verified,counts);
}}}
fn main(){
 let args:Vec<String>=std::env::args().collect();
 if args.len()==2 && args[1]=="--describe" {println!("native-seed-factor-row/v1; 8 fixed targets; baseline,candidate,aa; setup and prepare separate; no execution");return;}
 if args.len()!=3||args[1]!="--run-frozen-panel"||args[2].len()!=64||!args[2].bytes().all(|b|b.is_ascii_hexdigit()){
  eprintln!("usage: runner --describe | --run-frozen-panel PROTOCOL_SHA256 (requires external freeze/admission)");std::process::exit(2);
 }
 println!("{{\"schema\":\"native-seed-factor-run/v1\",\"protocol_sha256\":\"{}\",\"stages\":{:?},\"components\":{:?},\"operations\":{:?}}}",args[2],meter::STAGES,meter::COMPONENTS,meter::OPS);
 meter::reset();let b=baseline::Decoder::repaired_479().expect("baseline setup");let c=meter::snapshot();emit("baseline",None,"setup",None,false,c);
 meter::reset();let m=candidate::Decoder::repaired_479().expect("candidate setup");let c=meter::snapshot();emit("candidate",None,"setup",None,false,c);
 meter::reset();let a=baseline::Decoder::repaired_479().expect("aa setup");let c=meter::snapshot();emit("aa",None,"setup",None,false,c);
 meter::reset();let bp=b.prepare().expect("baseline preparation");let c=meter::snapshot();emit("baseline",None,"prepare",None,false,c);
 meter::reset();let mp=m.prepare().expect("candidate preparation");let c=meter::snapshot();emit("candidate",None,"prepare",None,false,c);
 meter::reset();let ap=a.prepare().expect("aa preparation");let c=meter::snapshot();emit("aa",None,"prepare",None,false,c);
 let mut ok=true;
 for line in include_str!("../feature-native-recovery/targets.tsv").lines(){
  let f:Vec<i64>=line.split('\t').map(|s|s.parse().expect("fixture integer")).collect();assert_eq!(f.len(),3);
  let expected=f[0] as u64;let q=if f[1]<0{None}else{Some([f[1]as u64,f[2]as u64])};
  if expected%2==0{query!(bp,"baseline",q,expected,ok);query!(mp,"candidate",q,expected,ok);query!(ap,"aa",q,expected,ok);}
  else{query!(ap,"aa",q,expected,ok);query!(mp,"candidate",q,expected,ok);query!(bp,"baseline",q,expected,ok);}
 }
 if !ok{std::process::exit(1);}
}

#[cfg(test)]mod instrument_tests{
 use super::*;
 #[test]fn setup_exponent_hooks_are_separate(){
  meter::reset();let _b=baseline::Decoder::repaired_479().unwrap();let bc=meter::snapshot();
  meter::reset();let c=candidate::Decoder::repaired_479().unwrap();let cc=meter::snapshot();
  assert_eq!(&cc[1][4][17..],&[21,9,6,3]);
  for stage in 0..5{for component in 0..8{
   assert_eq!(&bc[stage][component][17..],&[0;4]);
   if (stage,component)!=(1,4){assert_eq!(&cc[stage][component][17..],&[0;4]);}
  }}
  meter::reset();let _prepared=c.prepare().unwrap();let pc=meter::snapshot();
  for stage in pc{for component in stage{assert_eq!(&component[17..],&[0;4]);}}
 }
 #[test]fn complete_recovery_and_source_inversion_contracts(){
  let b=baseline::Decoder::repaired_479().unwrap();let bp=b.prepare().unwrap();let c=candidate::Decoder::repaired_479().unwrap();let cp=c.prepare().unwrap();
  for line in include_str!("../feature-native-recovery/targets.tsv").lines(){let f:Vec<i64>=line.split('\t').map(|s|s.parse().unwrap()).collect();let q=if f[1]<0{None}else{Some([f[1]as u64,f[2]as u64])};
   meter::reset();let old=bp.recover(q).unwrap();let bc=meter::snapshot();
   meter::reset();let got=cp.recover(q).unwrap();let cc=meter::snapshot();
   assert_eq!(format!("{:?}",old),format!("{:?}",got));assert_eq!(got.scalar,f[0]as u64);
   assert_eq!(bc[3][4][1],0);assert_eq!(cc[3][4][1],0);
   assert!(cc[3][4][0]>0);assert_eq!(cc[3][0][1],1);assert_eq!(cc[3][5][7],477);
   assert_eq!(cc[3][6],[0;21]);assert_eq!(cc[4][4],[0;21]);
   for stage in 0..5{for component in 0..8{assert_eq!(&bc[stage][component][17..],&[0;4]);assert_eq!(&cc[stage][component][17..],&[0;4]);}}
   // Polynomial, geometry, direct phase, value and decoder arithmetic are unchanged.
   for component in [0usize,1,2,3,5,6]{assert_eq!(&bc[3][component][..10],&cc[3][component][..10]);}
  }
 }
}
