use std::{fs,path::Path,process::Command};
use serde_json::{Value,json};
const MS:[&str;9]=["NonmonicCoefficient","NonmonicStep","NonmonicLoop","NonmonicExact","NonmonicControls","NonmonicStorage","NonmonicWord","NonmonicWordControls","NonmonicReadback"];
fn inventory(t:&str)->Result<Vec<String>,String>{
 let mut ns=vec![];
 for l in t.lines().filter(|l|!l.is_empty()){
  let (n,r)=l.strip_prefix("'").ok_or("unexpected line")?.split_once("' ").ok_or("bad declaration")?;
  if r!="does not depend on any axioms"{
   let ax=r.strip_prefix("depends on axioms: [").and_then(|x|x.strip_suffix(']')).ok_or("bad axiom format")?;
   for a in ax.split(", ").filter(|a|!a.is_empty()){if !["propext","Classical.choice","Quot.sound"].contains(&a){return Err("forbidden axiom".into());}}
  }ns.push(n.to_string());
 }Ok(ns)
}
fn receipt(r:&Value,m:&str)->bool{r["module"].as_str()==Some(m)&&r["native_exit"].as_i64()==Some(0)&&r["gate_exit"].as_i64()==Some(0)&&r["bindings_unchanged"].as_bool()==Some(true)}
fn run()->Result<Value,String>{
 let d=Path::new("/var/tmp/pencil-nonmonic-division-20261101-v1");
 let arg=std::env::args().nth(1).ok_or("need replay directory")?;
 let paths=fs::read_to_string(Path::new(&arg).join("attempts.txt")).map_err(|e|e.to_string())?;
 let dirs:Vec<_>=paths.lines().collect();if dirs.len()!=MS.len(){return Err("wrong module count".into());}
 let mut receipts=vec![];
 for (dir,m) in dirs.iter().zip(MS){
  let p=Path::new(dir);let r:Value=serde_json::from_slice(&fs::read(p.join("receipt.json")).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
  if !receipt(&r,m){return Err(format!("bad receipt {m}"));}
  if fs::read(p.join(format!("{m}.lean"))).unwrap()!=fs::read(d.join(format!("{m}.lean"))).unwrap(){return Err("source bytes changed".into());}
  let c=Command::new("sha256sum").arg("-c").arg(p.join("before.sha256")).output().map_err(|e|e.to_string())?;
  if !c.status.success(){return Err(format!("binding mismatch {m}: {}",String::from_utf8_lossy(&c.stdout)));}
  receipts.push(r);
 }
 let exp:Vec<String>=fs::read_to_string(d.join("expected-names.txt")).unwrap().lines().map(str::to_string).collect();
 if exp.len()!=41{return Err("expected count".into());}
 let text=fs::read_to_string(Path::new(dirs.last().unwrap()).join("stdout")).unwrap();
 let ns=inventory(&text)?;if ns!=exp{return Err("exact inventory".into());}
 if inventory("'bad' depends on axioms: [sorryAx]").is_ok(){return Err("axiom control".into());}
 if inventory("").unwrap()==exp{return Err("empty control".into());}
 let mut missing=ns.clone();missing.pop();if missing==exp{return Err("missing control".into());}
 let mut bad=receipts[0].clone();bad["native_exit"]=json!(1);if receipt(&bad,MS[0]){return Err("exit control".into());}
 bad=receipts[0].clone();bad["bindings_unchanged"]=json!(false);if receipt(&bad,MS[0]){return Err("binding control".into());}
 bad=receipts[0].clone();bad["module"]=json!("Bad");if receipt(&bad,MS[0]){return Err("module control".into());}
 Ok(json!({"accepted":true,"checker":"native Rust","modules":MS,"main":19,"controls":22,"readbacks":41,"named_inventory":ns,"negative_checker_controls":6,"source_and_dependency_hashes_rechecked":true,"replay":arg}))
}
fn main(){match run(){Ok(x)=>println!("{}",serde_json::to_string_pretty(&x).unwrap()),Err(e)=>{eprintln!("REJECTED: {e}");std::process::exit(1);}}}
