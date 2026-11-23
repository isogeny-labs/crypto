//! Post-run independent grid/summary recomputation, not frozen primary analysis.
use serde_json::{json,Value};
use std::{collections::{BTreeSet,BTreeMap},env,fs};
fn main(){
 let a:Vec<_>=env::args().skip(1).collect();
 let rows:Vec<Value>=fs::read_to_string(&a[0]).unwrap().lines().map(|l|serde_json::from_str(l).unwrap()).collect();
 let s:Value=serde_json::from_slice(&fs::read(&a[1]).unwrap()).unwrap();
 let mut seen=BTreeSet::new();let mut common=BTreeMap::<String,Value>::new();let mut found=0;
 let ids:Vec<_>=[83,131].into_iter().flat_map(|n|["distinct-planted-0","uniform","cancellation"].map(|k|format!("scale-n{n}-d12-{k}"))).collect();
 for r in &rows {
  let id=r["instance"].as_str().unwrap();let arm=r["arm"].as_str().unwrap();let rep=r["replicate"].as_u64().unwrap();
  assert!(ids.iter().any(|i|i==id));assert!(["baseline","lookup","baseline-aa"].contains(&arm)&&rep<5);assert!(seen.insert((id.to_string(),arm.to_string(),rep)));
  assert_eq!(r["curve_slug"],if r["n"]==131 {"icv1-f2m131-tm22283658519494248867-115e0dc5"}else{"icv1-f2m83-tm6151469093347-debefd74"});
  let z=&r["result"];assert_eq!(z["valid"],true);
  assert_eq!(r["instructions"].as_u64().unwrap(),r["setup_instructions"].as_u64().unwrap()+r["oracle_instructions"].as_u64().unwrap()+r["post_instructions"].as_u64().unwrap());
  if id.ends_with("uniform"){assert_eq!(z["outcome"],"Refuted");}else{assert_eq!(z["outcome"],"Found");assert_eq!(z["replay"],true);found+=1;}
  if let Some(old)=common.get(id){for k in ["outcome","indices","points","models","patterns_attempted","patterns_refuted","reason","common","projected"]{assert_eq!(old[k],z[k],"{id}/{arm}/{rep}/{k}");}}else{common.insert(id.into(),z.clone());}
  if !id.ends_with("cancellation"){assert_eq!(z["projected"]["columns"],13);assert!(z["projected"]["systems"].as_u64().unwrap()>0);}
 }
 assert_eq!(rows.len(),90);assert_eq!(seen.len(),90);assert_eq!(found,60);
 let mut criteria=Vec::new();let mut goal=true;
 for id in ids {
  let get=|arm:&str,metric:&str|{let mut raw:Vec<_>=rows.iter().filter(|r|r["instance"]==id&&r["arm"]==arm).map(|r|r[metric].as_u64().unwrap()).collect();raw.sort();assert_eq!(raw.len(),5);let summary=&s["cells"][format!("{id}/{arm}")]["metrics"][metric];assert_eq!(summary["min"],raw[0]);assert_eq!(summary["median"],raw[2]);assert_eq!(summary["max"],raw[4]);raw[2]};
  for arm in ["baseline","lookup","baseline-aa"]{for m in ["instructions","setup_instructions","oracle_instructions","post_instructions","field_mul","field_square"]{get(arm,m);}}
  let b=get("baseline","instructions");let c=get("lookup","instructions");let o=get("baseline","oracle_instructions");let p=get("lookup","oracle_instructions");let aa=b.abs_diff(get("baseline-aa","instructions"))as f64/b as f64;
  let cold_fraction=c as f64/b as f64;let oracle_fraction=p as f64/o as f64;
  let passes=aa<=0.01&&if id.ends_with("uniform"){cold_fraction<=0.99&&oracle_fraction<=0.95}else{cold_fraction<=1.01};goal &= passes;
  // Row evaluation does no field arithmetic: its field ledgers must be exact.
  for m in ["field_mul","field_square"]{assert_eq!(get("baseline",m),get("lookup",m));}
  criteria.push(json!({"instance":id,"baseline_over_lookup_whole":1./cold_fraction,"baseline_over_lookup_oracle":1./oracle_fraction,"whole_removed_fraction":1.-cold_fraction,"oracle_removed_fraction":1.-oracle_fraction,"aa_relative_median_difference":aa,"passes":passes}));
 }
 println!("{}",serde_json::to_string_pretty(&json!({"schema":"trace-zero-rowmap-grid-audit/v1","planned":90,"attempted":90,"completed":90,"invalid":0,"missing":0,"retries":0,"unique_grid":true,"exact_common_outputs":true,"phase_sums_match":true,"all_summary_ranges_recomputed":true,"field_ledgers_identical":true,"found_point_tuples":found,"fresh_primary_child_processes":180,"goal_criteria":criteria,"goal_met":goal,"standing":"Producer-written separate post-run recomputation; no independently signed verdict."})).unwrap());
}
