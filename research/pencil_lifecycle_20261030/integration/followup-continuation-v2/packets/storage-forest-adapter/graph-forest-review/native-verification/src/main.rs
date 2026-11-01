use std::{fs,path::Path};
use serde_json::{Value,json};
const MODULES: [&str;10]=["Find","Cycles","Union","Forest","Composition","PriorControls","ForestControls","Readback","Reviewer","ReviewerReadback"];
fn expected()->Vec<String>{
 let mut v=Vec::new();
 for (prefix,names) in [
 ("GraphForest","root_reach reach_parent attach_forest find_preserves_roots two_finds branch_preserves edge_after_two_finds next_find_exists"),
 ("ForestControls","initial_forest distinct_attach later_find_terminates same_component_cycle unsafe_parent_not_root nonroot_child_can_cut first_find second_find composed_either"),
 ("GraphForest","attachment_roots component_transferred"),
 ("ForestReviewer","initial_forest initial_valid first_find second_find both_nontrivial composed_both_orientations next_find_from_result first_root_survives_second descendant_path transferred_descendant descendant_cycle_rejected proposed_parent_not_root root_components_disjoint same_component_second_find same_root_branch_excluded")
 ] {for n in names.split_whitespace(){v.push(format!("{prefix}.{n}"));}} v
}
fn inventory(text:&str)->Result<Vec<String>,String>{
 let mut names=Vec::new();
 for line in text.lines().filter(|l|!l.trim().is_empty()){
  let body=line.strip_prefix("'").ok_or("unexpected readback line")?;
  let (name,rest)=body.split_once("' ").ok_or("malformed readback")?;
  if rest!="does not depend on any axioms" {
   let a=rest.strip_prefix("depends on axioms: [").and_then(|s|s.strip_suffix(']')).ok_or("malformed axiom list")?;
   for ax in a.split(", ").filter(|s|!s.is_empty()){
    if !["propext","Classical.choice","Quot.sound"].contains(&ax){return Err(format!("forbidden axiom {ax}"));}
   }
  } names.push(name.to_string());
 } Ok(names)
}
fn receipt(r:&Value)->Result<(),String>{
 if r["gate_exit"].as_i64()!=Some(0)||r["bindings_unchanged"].as_bool()!=Some(true){return Err("gate/binding failure".into());}
 let ms=r["modules"].as_array().ok_or("missing modules")?;
 if ms.len()!=MODULES.len(){return Err("module count".into());}
 for (m,n) in ms.iter().zip(MODULES){
  if m["module"].as_str()!=Some(n)||m["native_exit"].as_i64()!=Some(0){return Err("module name/exit".into());}
 } Ok(())
}
fn run()->Result<Value,String>{
 let root=Path::new("/var/tmp/pencil-graph-forest-review-20261101");
 let s=root.join("subject");let i=std::env::args().nth(1).map(std::path::PathBuf::from).unwrap_or_else(||s.join("invocation-a7dpq5t0"));
 let read=|p:&Path|fs::read_to_string(p).map_err(|e|e.to_string());
 let r:Value=serde_json::from_str(&read(&i.join("receipt.json"))?).map_err(|e|e.to_string())?;
 receipt(&r)?;
 for m in MODULES{
  let a=fs::read(i.join("snapshot").join(format!("{m}.lean"))).map_err(|e|e.to_string())?;
  let b=fs::read(s.join(format!("{m}.lean"))).map_err(|e|e.to_string())?;
  if a!=b{return Err(format!("source mismatch {m}"));}
 }
 let text=read(&i.join("Readback.stdout"))?+&read(&i.join("ReviewerReadback.stdout"))?;
 let names=inventory(&text)?;
 if names!=expected(){return Err("exact ordered inventory mismatch".into());}
 let mut bad=r.clone();bad["gate_exit"]=json!(1);if receipt(&bad).is_ok(){return Err("failed gate control".into());}
 bad=r.clone();bad["modules"][0]["native_exit"]=json!(1);if receipt(&bad).is_ok(){return Err("failed native control".into());}
 bad=r.clone();bad["modules"][0]["module"]=json!("Other");if receipt(&bad).is_ok(){return Err("wrong module control".into());}
 if inventory("'Bad' depends on axioms: [sorryAx]").is_ok(){return Err("forbidden axiom control".into());}
 if inventory("").unwrap()==expected(){return Err("empty inventory control".into());}
 let mut missing=names.clone();missing.pop();if missing==expected(){return Err("missing name control".into());}
 let false_r:Value=serde_json::from_str(&read(&s.join("invocation-9j2tzvzh/receipt.json"))?).map_err(|e|e.to_string())?;
 if false_r["gate_exit"].as_i64()!=Some(1)||!false_r["modules"].as_array().ok_or("false modules")?.iter().any(|m|m["native_exit"].as_i64()==Some(1)){return Err("retained false control missing native failure".into());}
 Ok(json!({"accepted":true,"checker":"native Rust","named_inventory":names,"main":10,"producer_controls":9,"reviewer_controls":15,"readbacks":34,"module_count":10,"source_bytes_equal":true,"negative_checker_controls":6,"retained_false_native_exit":1,"retained_false_gate_exit":1,"lean_reexecuted":false,"receipt":i.join("receipt.json")}))
}
fn main(){match run(){Ok(v)=>println!("{}",serde_json::to_string_pretty(&v).unwrap()),Err(e)=>{eprintln!("REJECTED: {e}");std::process::exit(1);}}}
