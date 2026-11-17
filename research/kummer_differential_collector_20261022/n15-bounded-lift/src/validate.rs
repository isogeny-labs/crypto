//! Independent N15 packet integrity and count recomputation from per-state rows.
use serde_json::Value;
use sha2::{Digest,Sha256};
use std::{collections::{BTreeMap,BTreeSet},fs};
fn digest(path:&str)->String{format!("{:x}",Sha256::digest(fs::read(path).unwrap()))}
fn n(s:&str)->u64{s.parse().unwrap()}
fn sample(r:u64,bits:u64)->Vec<u64>{
 if bits==16 {return (0..r).collect()}
 let mut keys=BTreeSet::from([0,1,2,r-2,r-1]);
 for i in 0..1024u64 {keys.insert(((2*i+1) as u128*r as u128/2048) as u64);}
 keys.into_iter().collect()
}
fn main(){
 let base="research/kummer_differential_collector_20261022/n15-bounded-lift";
 let packet:Value=serde_json::from_str(&fs::read_to_string(format!("{base}/results/summary.json")).unwrap()).unwrap();
 assert_eq!(packet["schema"],"kummer-n15-bounded-lift/v1");assert_eq!(packet["cohort_eligible"],false);assert!(packet["all_state_exception_bound"].is_null());assert!(packet["rho_cost_ratio"].is_null());
 for source in packet["source_digests"].as_array().unwrap(){
  let path=source["path"].as_str().unwrap();assert_eq!(source["sha256"],digest(path),"source changed: {path}");
 }
 let csv=fs::read_to_string(format!("{base}/results/states.csv")).unwrap();
 assert_eq!(packet["states_sha256"],format!("{:x}",Sha256::digest(csv.as_bytes())));
 let mut lines=csv.lines();let header:Vec<_>=lines.next().unwrap().split(',').collect();
 assert_eq!(header.len(),30);assert_eq!(header[0],"schema");assert_eq!(header[28],"lift_exception_candidates");assert_eq!(header[29],"lift_zero_b1_candidates");
 let mut counts:BTreeMap<u64,BTreeMap<String,u64>>=BTreeMap::new();
 let mut lifts:BTreeMap<u64,BTreeMap<String,u64>>=BTreeMap::new();
 let mut keys:BTreeMap<u64,Vec<u64>>=BTreeMap::new();
 let mut totals:BTreeMap<u64,[u64;13]>=BTreeMap::new();
 let mut bits_by_p:BTreeMap<u64,u64>=BTreeMap::new();let mut r_by_p:BTreeMap<u64,u64>=BTreeMap::new();
 for line in lines{
  let c:Vec<_>=line.split(',').collect();assert_eq!(c.len(),30);assert_eq!(c[0],"kummer-n15-state/v1");
  let (bits,p,r,k)=(n(c[1]),n(c[2]),n(c[3]),n(c[4]));assert!([16,20,24,28,32,36,40].contains(&bits));
  assert!(bits_by_p.insert(p,bits).is_none_or(|v|v==bits));assert!(r_by_p.insert(p,r).is_none_or(|v|v==r));
  keys.entry(p).or_default().push(k);
  let dst=counts.entry(p).or_default();*dst.entry(format!("projection:{}",c[5])).or_default()+=1;
  let zeros:Vec<_>=c[12].split('|').collect();assert_eq!(zeros.len(),3);
  for (name,raw,repair,idx) in [("dbl",6,7,0),("mix",8,9,1),("plus_dbl",10,11,2)]{
   *dst.entry(format!("{name}:raw:{}",c[raw])).or_default()+=1;
   *dst.entry(format!("{name}:repair:{}",c[repair])).or_default()+=1;
   *dst.entry(format!("{name}:diff_zero_count:{}",zeros[idx])).or_default()+=1;
  }
  assert_eq!(c[13].split('|').count(),3);
  *lifts.entry(p).or_default().entry(format!("{}:{}:{}",c[14],c[15],c[16])).or_default()+=1;
  let sum=totals.entry(p).or_default();for (i,col) in (17..30).enumerate(){sum[i]+=n(c[col]);}
 }
 let models=packet["models"].as_array().unwrap();assert_eq!(models.len(),21);assert_eq!(keys.len(),21);
 for model in models{
  let (p,r,bits)=(model["p"].as_u64().unwrap(),model["r"].as_u64().unwrap(),model["bits"].as_u64().unwrap());
  assert_eq!(bits_by_p[&p],bits);assert_eq!(r_by_p[&p],r);assert_eq!(keys[&p],sample(r,bits));assert_eq!(model["sample_states"],keys[&p].len());
  if bits!=16 {assert_eq!(keys[&p].len(),1029)}
  for (name,value) in &counts[&p]{assert_eq!(model["counts"][name],*value,"count {p} {name}");}
  for (name,value) in &lifts[&p]{assert_eq!(model["lift_counts"][name],*value,"lift {p} {name}");}
  let fallback:u64=lifts[&p].iter().filter(|(key,_)|key.ends_with(":true")&&!key.contains(":not_needed:")).map(|(_,v)|*v).sum();
  let null:u64=lifts[&p].iter().filter(|(name,_)|name.ends_with(":false")).map(|(_,value)|*value).sum();
  assert_eq!(model["lift_accounting"]["direct_fast_recover_failed"],fallback+null);
  assert_eq!(model["lift_accounting"]["enumerated_fallback_succeeded"],fallback);
  assert_eq!(model["lift_accounting"]["final_no_divisor"],null);
  assert_eq!(model["lift_accounting"]["unaccounted_states"],0);
  let guard=&model["denominator_guards"];
  assert_eq!(guard["theta_inverse_zero_count"],0);
  assert!(guard["theta_inverse_constants"].as_array().unwrap().iter().all(|x|x.as_u64().unwrap()!=0));
  for (label,field) in [("k0","k0_zero_coordinates"),("kg","kg_zero_coordinates")]{
   let zeros=guard[label].as_array().unwrap().iter().filter(|x|x.as_u64().unwrap()==0).count();assert_eq!(guard[field],zeros);
  }
  let t=totals[&p];let l=&model["lift_ops_totals"];
  for (name,i) in [("m",0),("a",1),("i",2)]{assert_eq!(l[name],t[i]);}
  for (name,i) in [("m",3),("s",4),("i",5),("field_adds",6),("bit_ops",7),("ec_adds",8),("jac_adds",9),("sqrt_calls",10)]{assert_eq!(l["jacobian_field_counts"][name],t[i]);}
  assert_eq!(l["exception_candidates"],t[11]);
  assert_eq!(l["zero_b1_candidates"],t[12]);
 }
 let manifest=fs::read_to_string(format!("{base}/SHA256SUMS")).unwrap();
 for line in manifest.lines(){let (sha,path)=line.split_once("  ").unwrap();assert_eq!(sha,digest(&format!("{base}/{path}")),"packet changed: {path}");}
 println!("validated {} finite-state rows across 21 frozen models, all source digests and complete counters",keys.values().map(Vec::len).sum::<usize>());
}
