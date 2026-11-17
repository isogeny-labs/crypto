use std::{fs,path::Path};
fn audit(expected:&[String], output:&str)->bool {
 let lines:Vec<_>=output.lines().filter(|l| !l.trim().is_empty()).collect();
 if lines.len()!=expected.len() || expected.is_empty(){return false;}
 for (name,line) in expected.iter().zip(lines) {
  let prefix=format!("'{}' depends on axioms: [",name);
  if let Some(rest)=line.strip_prefix(&prefix).and_then(|x|x.strip_suffix(']')) {
   if !rest.split(", ").all(|a|["propext","Classical.choice","Quot.sound"].contains(&a)){return false;}
  } else if line!=format!("'{}' does not depend on any axioms",name){return false;}
 }
 true
}
fn main(){
 let d=Path::new("/var/tmp/pencil-stored-valid-wrapper-20261101-v1");
 let src=fs::read_to_string(d.join("StoredValidReadback.lean")).unwrap();
 let names:Vec<String>=src.lines().filter_map(|l|l.strip_prefix("#print axioms ").map(str::to_owned)).collect();
 let out=fs::read_to_string(d.join("StoredValidReadback-attempt-lK5w8PVn/stdout")).unwrap();
 assert_eq!(names.len(),12);assert!(audit(&names,&out));
 assert!(!audit(&names,""));assert!(!audit(&names,&out.replacen("propext","sorryAx",1)));
 assert!(!audit(&names,&out.replacen(&names[0],"Wrong.name",1)));
 println!("12 exact axiom readbacks passed; empty, sorryAx and wrong-name controls rejected");
}
