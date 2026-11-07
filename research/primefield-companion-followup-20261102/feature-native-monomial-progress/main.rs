//! Matched counted instrument; --describe never executes a kernel.
#[path="../feature-native-monomial-matched/meter.rs"]pub mod meter;
#[global_allocator]static ALLOCATOR:meter::Counting=meter::Counting;
#[path="../feature-native-monomial-matched/generated-baseline/recovery.rs"]mod baseline;
#[path="../feature-native-monomial-matched/generated-candidate/recovery.rs"]mod candidate;
mod verify{
 use crate::meter::{self,Component as C,Op};
 fn mul(a:u64,b:u64)->u64{meter::hit(C::Verifier,Op::Product);((a as u128*b as u128)%509)as u64}
 fn inv(a:u64)->u64{meter::hit(C::Verifier,Op::Inverse);let(mut a,mut n,mut v)=(a,507,1);while n>0{if n&1!=0{v=mul(v,a);}n>>=1;if n>0{a=mul(a,a);}}v}
 fn add(a:Option<[u64;2]>,b:Option<[u64;2]>)->Option<[u64;2]>{let(Some([x,y]),Some([u,v]))=(a,b)else{return a.or(b)};if x==u&&(y+v)%509==0{return None;}let(n,d)=if x==u{((mul(3,mul(x,x))+2)%509,2*y%509)}else{((v+509-y)%509,(u+509-x)%509)};let m=mul(n,inv(d));let xx=(mul(m,m)+1018-x-u)%509;Some([xx,(mul(m,(x+509-xx)%509)+509-y)%509])}
 pub fn point(mut k:u64)->Option<[u64;2]>{let(mut out,mut base)=(None,Some([3,68]));while k>0{if k&1!=0{out=add(out,base);}k>>=1;if k>0{base=add(base,base);}}out}
}

use meter::{Counts,Stage as S};
#[derive(Debug,PartialEq)]struct ResultRow{value:u64,signs:usize,extra:Option<usize>,steps:usize,skips:usize,conversions:usize,initial_direct:usize,initial_conversions:usize,extra_direct:usize}
struct Row{label:u64,point:Option<[u64;2]>,result:ResultRow,counts:Counts}
struct Arm{fixed:Counts,rows:Vec<Row>}
fn progress(event:&str,arm:&str,label:i64){eprintln!("progress\t{{\"event\":\"{}\",\"arm\":\"{}\",\"label\":{}}}",event,arm,label);}
fn sum(a:&mut Counts,b:Counts){for s in 0..14{for c in 0..12{for o in 0..17{a[s][c][o]=a[s][c][o].checked_add(b[s][c][o]).unwrap();}}}}
fn targets()->Vec<(u64,Option<[u64;2]>)>{include_str!("../feature-native-quartic-recovery/recovery.tsv").lines().skip(1).map(|l|{let f:Vec<i64>=l.split('\t').map(|x|x.parse().unwrap()).collect();(f[0]as u64,if f[1]<0{None}else{Some([f[1]as u64,f[2]as u64])})}).collect()}
macro_rules! arm {($name:ident,$module:ident)=>{fn $name(input:&[(u64,Option<[u64;2]>)],tag:&str)->Arm{
 progress("arm_started",tag,-1);
 meter::reset();let program=$module::Program::construct(479,14,2,479,2500).unwrap();let mut fixed=meter::snapshot();
 meter::reset();let source=$module::Source::new(509,2,10,479,[3,68],128,16).unwrap();sum(&mut fixed,meter::snapshot());
 meter::reset();let prepared=source.prepare().unwrap();sum(&mut fixed,meter::snapshot());let mut rows=Vec::new();
 for &(label,point)in input{progress("query_started",tag,label as i64);meter::reset();let out=prepared.recover(&program,point).unwrap();let rebuilt={let _s=meter::enter(S::Verify);verify::point(out.value)};let counts=meter::snapshot();assert_eq!(rebuilt,point);assert_eq!(out.value,label);progress("query_verified",tag,label as i64);
 let result=ResultRow{value:out.value,signs:out.sign_values,extra:out.extra_offset,steps:out.steps,skips:out.skipped_signs,conversions:out.conversions,initial_direct:out.initial_direct_initializations,initial_conversions:out.initial_conversions,extra_direct:out.direct_phases};rows.push(Row{label,point,result,counts});}
 meter::reset();{let _s=meter::enter(S::Prepare);drop(prepared);}sum(&mut fixed,meter::snapshot());
 meter::reset();{let _s=meter::enter(S::Calibration);drop(source);}sum(&mut fixed,meter::snapshot());
 meter::reset();{let _s=meter::enter(S::Compile);drop(program);}sum(&mut fixed,meter::snapshot());progress("arm_finished",tag,-1);Arm{fixed,rows}
}};}
arm!(run_baseline,baseline);arm!(run_candidate,candidate);
fn emit(kind:&str,arm:&str,label:i64,c:&Counts){print!("{{\"kind\":\"{kind}\",\"arm\":\"{arm}\",\"label\":{label},\"counts\":{:?}}}\n",c);}
fn main(){let args:Vec<String>=std::env::args().collect();if args.len()==2&&args[1]=="--describe"{println!("monomial-matched/v1: baseline,candidate,aa; 20 public fixtures; 14 stages x12 components x17 operations; no kernel executed");return;}assert!(args.len()==2&&args[1]=="--run-admitted","use --describe; execution requires frozen admitted protocol");
 let t=targets();assert_eq!(t.len(),20);let b=run_baseline(&t,"baseline");let c=run_candidate(&t,"candidate");let a=run_baseline(&t,"aa");assert_eq!(b.fixed,a.fixed);println!("{{\"schema\":\"monomial-matched/v1\",\"targets\":20,\"shape\":[14,12,17]}}");
 for(name,data)in[("baseline",&b),("candidate",&c),("aa",&a)]{emit("fixed",name,-1,&data.fixed);}
 for i in 0..20{assert_eq!(b.rows[i].result,c.rows[i].result);assert_eq!(b.rows[i].result,a.rows[i].result);assert_eq!(b.rows[i].counts,a.rows[i].counts);assert_eq!(b.rows[i].point,c.rows[i].point);for(name,data)in[("baseline",&b),("candidate",&c),("aa",&a)]{let row=&data.rows[i];let (x,y)=row.point.map(|[x,y]|(x as i64,y as i64)).unwrap_or((-1,-1));let v=&row.result;let result=[v.value as i64,v.signs as i64,v.extra.map(|x|x as i64).unwrap_or(-1),v.steps as i64,v.skips as i64,v.conversions as i64,v.initial_conversions as i64,v.initial_direct as i64,v.extra_direct as i64];println!("{{\"kind\":\"query\",\"arm\":\"{}\",\"label\":{},\"point\":[{},{}],\"result\":{:?},\"counts\":{:?}}}",name,row.label,x,y,result,row.counts);}}
 println!("{{\"complete\":true,\"queries\":60,\"aa_equal\":true,\"forward_checked\":true}}");progress("panel_finished","all",-1);
}
#[cfg(test)]mod tests{use super::*;
 #[test]fn matched_instrument_correctness(){let all=targets();let t:Vec<_>=all.into_iter().filter(|(k,_)|[0,25].contains(k)).collect();assert_eq!(t.len(),2);let b=run_baseline(&t,"baseline");let c=run_candidate(&t,"candidate");let a=run_baseline(&t,"aa");assert_eq!(b.fixed,a.fixed);for i in 0..2{assert_eq!(b.rows[i].result,c.rows[i].result);assert_eq!(b.rows[i].counts,a.rows[i].counts);for s in [1usize,2,3]{assert_eq!(b.rows[i].counts[s],[[0;17];12]);assert_eq!(c.rows[i].counts[s],[[0;17];12]);}for s in [5usize,8,10,13]{for k in 0..12{assert_eq!(&b.rows[i].counts[s][k][..7],&c.rows[i].counts[s][k][..7]);}}}assert!(b.fixed[1][10][10]>0);assert!(c.fixed[2][4][13]>0);}
}
