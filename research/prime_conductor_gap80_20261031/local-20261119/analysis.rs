use crypto_lib::cryptanalysis::ecbench::{runner,stats};
use serde_json::json;
use std::collections::{BTreeMap,BTreeSet};
fn main(){
 let mut sessions=Vec::new();let mut output=Vec::new();
 for dir in std::env::args().skip(1){
  let path=std::path::Path::new(&dir);let s=runner::read_session(path).expect("session");let records=runner::read_records(path).expect("records");
  sessions.push(json!({"dir":dir,"session_id":s.session_id,"status":s.status,"record_count":records.len(),"recorded_status_counts":s.status_counts}));
  let mut groups=BTreeMap::new();
  for r in &records {if !r.warmup{groups.entry((r.workload.curve.slug.clone(),r.arm.clone())).or_insert_with(Vec::new).push(r);}}
  for ((curve,arm),rs) in groups{
   let mut statuses=BTreeMap::<String,usize>::new();let mut unpriced=BTreeSet::new();let mut walls=Vec::new();let mut gae=Vec::new();let mut sn=Vec::new();let mut rss=Vec::new();
   for r in &rs{*statuses.entry(r.outcome.status.clone()).or_default()+=1;unpriced.extend(r.cost.unpriced.iter().cloned());if r.counts(){if let Some(ns)=r.time.solve_wall_ns{walls.push(ns as f64/1e9);}if let Some(x)=r.cost.total_gae{gae.push(x);}if let Some(x)=r.cost.s{sn.push(x);}}rss.push(r.time.max_rss_kib);}
   let complete=rs.iter().all(|r|r.counts());
   output.push(json!({"session_id":s.session_id,"curve_slug":curve,"arm":arm,"method":rs[0].method.id,"field_bits":rs[0].workload.curve.field_bits,"subgroup_order":rs[0].workload.curve.r,"attempts":rs.len(),"outcomes":statuses,"all_verified":complete,"mean_solve_seconds":if complete&&walls.len()==rs.len(){stats::mean(&walls)}else{None},"median_solve_seconds":if complete&&walls.len()==rs.len(){stats::median(&walls)}else{None},"mean_GAE_lower_bound":if complete{stats::mean(&gae)}else{None},"mean_S_lower_bound":if complete{stats::mean(&sn)}else{None},"max_RSS_KiB":rss.iter().max(),"unpriced":unpriced,"any_lower_bound":rs.iter().any(|r|r.cost.lower_bound),"isolation_levels":rs.iter().map(|r|format!("{:?}",r.isolation.level)).collect::<BTreeSet<_>>(),"wall_admitted":false,"scope":"descriptive local diagnostic; no complete-case estimate for failed groups"}));
  }
 }
 println!("{}",serde_json::to_string_pretty(&json!({"sessions":sessions,"groups":output,"endpoint_ratio_admitted":false})).unwrap());
}
