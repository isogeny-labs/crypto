// Native independent original-point check plus independent external-counter graph accounting.
mod replay {
    include!("audit.rs");
    pub fn original(scalar:i64,point:&str){assert!((0..31).contains(&scalar));assert_eq!(point,self::point(mul(Some(((0,0),(10,0))),scalar,21)));}
}
use std::{fs,path::Path,io::Write};
use serde_json::{Value,json};
fn counter(path:&Path,label:&str)->u64{
 let text=fs::read_to_string(path).unwrap();assert!(text.lines().any(|l|l=="events: Ir"));assert!(text.lines().any(|l|l=="positions: line"));
 assert!(text.lines().any(|l|l.starts_with("desc:")&&l.contains(label)),"exact client label missing");
 let mut summary=None;let mut totals=None;let mut independent_sum=0u64;let mut skip_callee=false;
 for line in text.lines(){
  if let Some(v)=line.strip_prefix("summary:"){assert!(summary.replace(v.trim().parse::<u64>().unwrap()).is_none());continue}
  if let Some(v)=line.strip_prefix("totals:"){assert!(totals.replace(v.trim().parse::<u64>().unwrap()).is_none());continue}
  if line.starts_with("calls="){assert!(!skip_callee);skip_callee=true;continue}
  let f:Vec<_>=line.split_whitespace().collect();if f.len()!=2{continue}
  let position=f[0];if !(position=="*"||position.starts_with("0x")||position.parse::<i64>().is_ok()){continue}
  let value=f[1].parse::<u64>().unwrap();if skip_callee{skip_callee=false}else{independent_sum=independent_sum.checked_add(value).unwrap();}
 }
 assert!(!skip_callee);let summary=summary.unwrap();assert_eq!(summary,independent_sum,"independent self-cost sum differs from external summary");assert_eq!(totals.unwrap(),summary);assert!(summary>0);summary
}
fn main(){let args:Vec<_>=std::env::args().collect();if args.len()!=4{std::process::exit(2)}
 let checked=std::panic::catch_unwind(||{
  let input:Value=serde_json::from_slice(&fs::read(&args[1]).unwrap()).unwrap();let prior:Value=serde_json::from_slice(&fs::read(&args[2]).unwrap()).unwrap();assert_eq!(input["schema"],"twist.ir-counter-preflight/v1");assert_eq!(input["field"],61);assert_eq!(input["relation_field"],31);
  let rows=input["records"].as_array().unwrap();assert_eq!(rows.len(),9);let expected_labels=["plain-3-3","plain-3-4","plain-4-3","plain-4-4","plain-5-3","plain-5-4","repeat-aa","positive-inside","negative-outside"];
  let mut counts=vec![];let mut output=vec![];
  for(i,r)in rows.iter().enumerate(){assert_eq!(r["label"],expected_labels[i]);assert_eq!(r["policy"],0);assert_eq!(r["target"],1);let route=r["route"].as_i64().unwrap();let cap=r["cap"].as_i64().unwrap();
   let p=prior["profiles_data"].as_array().unwrap().iter().find(|p|p["route"]==route&&p["policy"]==0&&p["cap"]==cap).unwrap();assert_eq!(r["result"],p["works"][0]["result"]);
   let point=format!("{} {}",input["target_point"][0].as_i64().unwrap(),input["target_point"][1].as_i64().unwrap());replay::original(r["result"]["scalar"].as_i64().unwrap(),&point);
   let ir=counter(Path::new(r["counter_path"].as_str().unwrap()),expected_labels[i]);counts.push(ir);output.push(json!({"label":expected_labels[i],"route":route,"cap":cap,"instruction_reads":ir,"original_scalar":r["result"]["scalar"]}));
  }
  let aa=counts[0].abs_diff(counts[6]);assert!(counts[7]>counts[0].max(counts[6]).max(counts[8])+10000,"planted extra counted work undetected");assert!(counts[8].abs_diff(counts[6])<=aa.max(1000),"outside-counter negative exceeds declared engineering tolerance");
  let receipt=json!({"schema":"twist.ir-counter-preflight.audit/v1","records":output,"aa_instruction_difference":aa,"positive_detected":true,"negative_within_declared_tolerance":true,"speed_claim":false,"signed_verdict":false});
  fs::OpenOptions::new().write(true).create_new(true).open(&args[3]).unwrap().write_all((receipt.to_string()+"\n").as_bytes()).unwrap();
 });if checked.is_err(){std::process::exit(101)}
}
