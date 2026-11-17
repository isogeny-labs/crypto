//! Independent row/digest validation of the finite exception replay.
use serde_json::Value;
use sha2::{Digest,Sha256};
use std::{collections::{BTreeMap,BTreeSet},fs};
fn digest(path:&str)->String{format!("{:x}",Sha256::digest(fs::read(path).unwrap()))}
fn main(){
 let base="research/kummer_differential_collector_20261022/n16-action-certificate";
 let packet:Value=serde_json::from_str(&fs::read_to_string(format!("{base}/results/summary.json")).unwrap()).unwrap();
 assert_eq!(packet["schema"],"kummer-n16-action-certificate/v1");
 assert!(packet["geometric_all_state_action_identity"].is_null());assert!(packet["rho_ratio"].is_null());
 for source in packet["source_digests"].as_array().unwrap(){let p=source["path"].as_str().unwrap();assert_eq!(source["sha256"],digest(p),"source {p}");}
 let csv=fs::read_to_string(format!("{base}/results/exceptional.csv")).unwrap();
 assert_eq!(packet["exceptional_csv_sha256"],format!("{:x}",Sha256::digest(csv.as_bytes())));
 let mut lines=csv.lines();assert_eq!(lines.next().unwrap(),"schema,bits,p,class,action_index,root,status");
 let mut by_p:BTreeMap<u64,(BTreeMap<String,u64>,BTreeMap<String,BTreeSet<u64>>,u64)>=BTreeMap::new();
 let mut total=0usize;
 for line in lines{
  let c:Vec<_>=line.split(',').collect();assert_eq!(c.len(),7);assert_eq!(c[0],"kummer-n16-exception/v1");
  let p:u64=c[2].parse().unwrap();let bits:u64=c[1].parse().unwrap();let idx:u64=c[4].parse().unwrap();let _:u64=c[5].parse().unwrap();
  assert!(idx<5);assert!([16,20,24,28,32,36,40].contains(&bits));
  assert!(["match","mismatch","missing"].contains(&c[6]));
  let (counts,classes,recorded_bits)=by_p.entry(p).or_default();
  if *recorded_bits==0 {*recorded_bits=bits}else{assert_eq!(*recorded_bits,bits)}
  *counts.entry(c[6].into()).or_default()+=1;
  assert!(classes.entry(c[3].into()).or_default().insert(idx),"duplicate action/class row");
  total+=1;
 }
 let models=packet["models"].as_array().unwrap();assert_eq!(models.len(),21);assert_eq!(by_p.len(),21);
 for model in models{
  let p=model["p"].as_u64().unwrap();let (counts,classes,bits)=&by_p[&p];
  assert_eq!(model["bits"],*bits);assert_eq!(model["action_count"],5);
  assert_eq!(classes.len(),model["class_count"].as_u64().unwrap() as usize);
  assert_eq!(classes.len(),model["degree_one_sample_classes"].as_u64().unwrap() as usize+6);
  assert!(classes.values().all(|set|*set==BTreeSet::from([0,1,2,3,4])));
  for (k,v) in counts{assert_eq!(model["check_counts"][k],*v,"{p} {k}");}
  assert_eq!(model["matrix_determinants"].as_array().unwrap().len(),5);
  assert_eq!(model["square_scalars"].as_array().unwrap().len(),5);
  assert_eq!(model["pairwise_commutation"].as_array().unwrap().len(),10);
 }
 let sums=fs::read_to_string(format!("{base}/SHA256SUMS")).unwrap();
 for line in sums.lines(){let (sum,path)=line.split_once("  ").unwrap();assert_eq!(sum,digest(&format!("{base}/{path}")),"packet {path}");}
 println!("validated {total} exceptional-action rows across 21 models and all source/packet digests");
}
