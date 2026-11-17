//! Independent packet integrity and count recomputation from per-state rows.
use serde_json::Value;
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs};
fn digest(path:&str)->String{format!("{:x}",Sha256::digest(fs::read(path).unwrap()))}
fn n(s:&str)->u64{s.parse().unwrap()}
fn main(){
 let base="research/kummer_differential_collector_20261022/n13-concrete-gate";
 let packet:Value=serde_json::from_str(&fs::read_to_string(format!("{base}/results/summary.json")).unwrap()).unwrap();
 assert_eq!(packet["schema"],"kummer-n13-concrete-gate/v1");
 assert_eq!(packet["cohort_eligible"],false);
 for src in packet["source_digests"].as_array().unwrap(){
  let path=src["path"].as_str().unwrap();assert_eq!(src["sha256"],digest(path),"source changed: {path}");
 }
 let csv=fs::read_to_string(format!("{base}/results/states.csv")).unwrap();
 assert_eq!(packet["states_sha256"],format!("{:x}",Sha256::digest(csv.as_bytes())));
 let mut lines=csv.lines();let header:Vec<_>=lines.next().unwrap().split(',').collect();
 assert_eq!(header.len(),24);assert_eq!(header[0],"schema");assert_eq!(header[23],"lift_exception_candidates");
 let mut counts:BTreeMap<u64,BTreeMap<String,u64>>=BTreeMap::new();
 let mut lift_counts:BTreeMap<u64,BTreeMap<String,u64>>=BTreeMap::new();
 let mut next:BTreeMap<u64,u64>=BTreeMap::new();
 let mut totals:BTreeMap<u64,[u64;8]>=BTreeMap::new();
 for line in lines{
  let c:Vec<_>=line.split(',').collect();assert_eq!(c.len(),24);assert_eq!(c[0],"kummer-n13-state/v1");assert_eq!(c[1],"16");
  let (p,r,k)=(n(c[2]),n(c[3]),n(c[4]));let expected=next.entry(p).or_default();assert_eq!(*expected,k,"missing/duplicate scalar for {p}");*expected+=1;assert!(k<r);
  let dst=counts.entry(p).or_default();*dst.entry(format!("projection:{}",c[5])).or_default()+=1;
  for (name,raw,repair,zero) in [("dbl",6,7,0),("mix",8,9,1),("plus_dbl",10,11,2)]{
   *dst.entry(format!("{name}:raw:{}",c[raw])).or_default()+=1;
   *dst.entry(format!("{name}:repair:{}",c[repair])).or_default()+=1;
   let zeroes:Vec<_>=c[12].split('|').collect();assert_eq!(zeroes.len(),3);
   *dst.entry(format!("{name}:diff_zero_count:{}",zeroes[zero])).or_default()+=1;
  }
  assert_eq!(c[13].split('|').count(),3);
  let key=format!("{}:{}",c[14],c[15]);*lift_counts.entry(p).or_default().entry(key).or_default()+=1;
  let sum=totals.entry(p).or_default();for (i,col) in [16,17,18,19,20,21,22,23].into_iter().enumerate(){sum[i]+=n(c[col]);}
 }
 let models=packet["models"].as_array().unwrap();assert_eq!(models.len(),3);
 for model in models{
  let (p,r)=(model["p"].as_u64().unwrap(),model["r"].as_u64().unwrap());assert_eq!(next[&p],r);
  for (key,value) in &counts[&p]{assert_eq!(model["counts"][key],*value,"count {p} {key}");}
  for (key,value) in &lift_counts[&p]{assert_eq!(model["lift_counts"][key],*value,"lift {p} {key}");}
  let fallback=lift_counts[&p].get("enumerated_exception_lift:true").copied().unwrap_or(0);
  let final_null:u64=lift_counts[&p].iter().filter(|(key,_)|key.ends_with(":false")).map(|(_,count)|*count).sum();
  assert_eq!(model["lift_accounting"]["direct_fast_recover_failed"],fallback+final_null);
  assert_eq!(model["lift_accounting"]["enumerated_fallback_succeeded"],fallback);
  assert_eq!(model["lift_accounting"]["final_no_divisor"],final_null);
  assert_eq!(model["lift_accounting"]["unaccounted_states"],0);
  assert_eq!(fallback+final_null+lift_counts[&p].get("computed:true").copied().unwrap_or(0),r);
  let t=totals[&p];let l=&model["lift_ops_totals"];
  assert_eq!(l["m"],t[0]);assert_eq!(l["a"],t[1]);assert_eq!(l["i"],t[2]);
  for (key,i) in [("m",3),("s",4),("i",5),("jac_adds",6)]{assert_eq!(l["jacobian_field_counts"][key],t[i]);}
  assert_eq!(l["exception_candidates"],t[7]);
 }
 let manifest=fs::read_to_string(format!("{base}/SHA256SUMS")).unwrap();
 for line in manifest.lines(){
  let (expected,path)=line.split_once("  ").unwrap();
  assert_eq!(expected,digest(&format!("{base}/{path}")),"packet artifact changed: {path}");
 }
 println!("validated {} complete signed states, {} models, source digests and per-state costs",next.values().sum::<u64>(),models.len());
}
