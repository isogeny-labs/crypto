use std::{fs,path::Path};
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
fn sha(p:&Path)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn load(p:&Path)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn names(text:&str)->Result<Vec<String>,String>{
 let mut ns=vec![];
 for l in text.lines().filter(|l|!l.is_empty()){
  let (n,r)=l.strip_prefix("'").ok_or("bad line")?.split_once("' ").ok_or("bad name")?;
  if r!="does not depend on any axioms"{
   let axs=r.strip_prefix("depends on axioms: [").and_then(|s|s.strip_suffix(']')).ok_or("bad readback")?;
   if !axs.split(", ").all(|s|["propext","Classical.choice","Quot.sound"].contains(&s)){return Err("bad axiom".into());}
  }
  ns.push(n.to_string());
 }Ok(ns)
}
fn main(){
 let d=Path::new("/var/tmp/independent-same-root-composition-review-20261101");
 let i=d.join("reviewer1");let r=load(&i.join("receipt.json"));
 let mods=["Find","Cycles","Union","Forest","Composition","SameRoot","SameComposition","SameControls","SameReadback","IndependentSame","IndependentSameReadback"];
 assert_eq!(r["gate_exit"],0);assert_eq!(r["bindings_unchanged"],true);
 let rows=r["modules"].as_array().unwrap();assert_eq!(rows.len(),mods.len());
 for (row,m) in rows.iter().zip(mods){assert_eq!(row["module"],m);assert_eq!(row["native_exit"],0);assert_eq!(fs::read(i.join("snapshot").join(format!("{m}.lean"))).unwrap(),fs::read(d.join("subject").join(format!("{m}.lean"))).unwrap());}
 for dir in ["reviewer1","known-false"]{
  let p=d.join(dir);let before=load(&p.join("before.json"));assert_eq!(before,load(&p.join("after.json")));
  for (path,h) in before.as_object().unwrap(){assert_eq!(sha(Path::new(path)),h.as_str().unwrap());}
 }
 let text=fs::read_to_string(i.join("SameReadback.stdout")).unwrap()+&fs::read_to_string(i.join("IndependentSameReadback.stdout")).unwrap();
 let observed=names(&text).unwrap();let expected:Vec<_>=include_str!("inventory.txt").lines().collect();
 assert_eq!(expected.len(),33);assert_eq!(observed,expected);
 assert!(names("'x' depends on axioms: [sorryAx]").is_err());
 assert_ne!(names("").unwrap(),expected);
 assert_ne!(&observed[..32],expected);
 let f=load(&d.join("known-false/receipt.json"));assert_eq!(f["gate_exit"],1);assert_eq!(f["modules"][0]["native_exit"],1);
 let false_stdout=fs::read_to_string(d.join("known-false/ExpectedFailure.stdout")).unwrap();assert!(false_stdout.contains("error:"));assert!(!false_stdout.contains("unknown module"));
 println!("{}",serde_json::to_string_pretty(&json!({"accepted":true,"producer_main":6,"producer_controls":12,"independent_controls":15,"readbacks":33,"modules":11,"source_hashes_rechecked":true,"named_inventory":observed,"checker_negative_controls":3,"false_native_exit":1,"false_gate_exit":1,"language":"Rust"})).unwrap());
}
