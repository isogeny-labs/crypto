use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{fs,path::Path,process::{Command,ExitCode},collections::BTreeMap};
const MODULES: [&str;11]=["NonmonicCoefficient","NonmonicStep","NonmonicLoop","NonmonicExact","NonmonicControls","NonmonicStorage","NonmonicWord","NonmonicWordControls","NonmonicReadback","ReviewerControls","ReviewerReadback"];
fn sha(p:&Path)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn load(p:&Path)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn binding(s:&Path,mods:&[String])->BTreeMap<String,String>{fs::read_dir(s).unwrap().map(|e|e.unwrap().path()).filter(|p|p.is_file() && !(p.extension().is_some_and(|e|e=="olean") && mods.iter().any(|m|p.file_stem().unwrap()==m.as_str()))).map(|p|(p.to_string_lossy().into_owned(),sha(&p))).collect()}
fn write(p:&Path,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn replay(source:&Path,out:&Path,mods:&[String])->i32{
 fs::create_dir(out).unwrap();let s=out.join("snapshot");fs::create_dir(&s).unwrap();
 for e in fs::read_dir(source).unwrap(){let p=e.unwrap().path();if p.is_file(){fs::copy(&p,s.join(p.file_name().unwrap())).unwrap();}}
 for m in mods{let p=s.join(format!("{m}.olean"));if p.exists(){fs::remove_file(p).unwrap();}}
 let before=binding(&s,mods);write(&out.join("before.json"),&json!(before));let mut rows=vec![];let mut gate=0;
 for m in mods{
  let lean="/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean";
  let args=vec!["-k".to_owned(),"5s".into(),"120s".into(),lean.into(),"-j1".into(),"-M4096".into(),"-o".into(),s.join(format!("{m}.olean")).display().to_string(),s.join(format!("{m}.lean")).display().to_string()];
  let r=Command::new("timeout").args(&args).current_dir(&s).env("LEAN_SYSROOT","/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain").env("LEAN_PATH",format!("{}:/var/tmp/pencil-mumford-validity-20261031-v1/dependency-union:/var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union",s.display())).output().unwrap();
  let code=r.status.code().unwrap_or(125);fs::write(out.join(format!("{m}.stdout")),r.stdout).unwrap();fs::write(out.join(format!("{m}.stderr")),r.stderr).unwrap();rows.push(json!({"module":m,"command":args,"native_exit":code}));println!("{m}: {code}");if code!=0{gate=code;break}
 }
 let after=binding(&s,mods);write(&out.join("after.json"),&json!(after));if before!=after{gate=126}
 write(&out.join("receipt.json"),&json!({"modules":rows,"gate_exit":gate,"bindings_unchanged":before==after}));gate
}
fn verify(good:&Path,bad:&Path)->Value{
 let r=load(&good.join("receipt.json"));let f=load(&bad.join("receipt.json"));assert_eq!(r["gate_exit"],0);assert_eq!(r["bindings_unchanged"],true);
 let rows=r["modules"].as_array().unwrap();assert_eq!(rows.len(),MODULES.len());for (r,m) in rows.iter().zip(MODULES){assert_eq!(r["module"],m);assert_eq!(r["native_exit"],0)}
 for d in [good,bad]{let before=load(&d.join("before.json"));let after=load(&d.join("after.json"));assert_eq!(before,after);for (p,h) in before.as_object().unwrap(){assert_eq!(sha(Path::new(p)),h.as_str().unwrap());}}
 let expected:Vec<_>=include_str!("inventory.txt").lines().collect();assert_eq!(expected.len(),49);let mut names=vec![];
 for n in ["NonmonicReadback","ReviewerReadback"]{
  let source=fs::read_to_string(good.join("snapshot").join(format!("{n}.lean"))).unwrap();let planned:Vec<_>=source.lines().filter_map(|l|l.strip_prefix("#print axioms ")).collect();let txt=fs::read_to_string(good.join(format!("{n}.stdout"))).unwrap();let mut observed=vec![];
  for l in txt.lines(){let (name,axioms)=l.strip_prefix('\'').unwrap().split_once("' depends on axioms: [").unwrap();let axioms=axioms.strip_suffix(']').unwrap();assert!(axioms.split(", ").all(|a|["propext","Classical.choice","Quot.sound"].contains(&a)));observed.push(name);}
  assert_eq!(planned,observed);names.extend(observed.into_iter().map(str::to_owned));
 }
 assert_eq!(names,expected);assert_eq!(f["gate_exit"],1);assert_eq!(f["modules"][0]["native_exit"],1);assert_eq!(fs::read_to_string(bad.join("snapshot/ExpectedFailure.lean")).unwrap(),"example : (0 : Nat) = 1 := by decide\n");
 json!({"schema":"pencil.native-verification/v1","accepted":true,"main":19,"producer_controls":22,"reviewer_controls":8,"readbacks":49,"good":good,"known_false":bad,"source_hashes_rechecked":true,"language":"Rust","scientific_execution":false})
}
fn main()->ExitCode{let a:Vec<_>=std::env::args().collect();match a[1].as_str(){"replay"=>ExitCode::from(replay(Path::new(&a[2]),Path::new(&a[3]),&a[4..]) as u8),"verify"=>{println!("{}",serde_json::to_string_pretty(&verify(Path::new(&a[2]),Path::new(&a[3]))).unwrap());ExitCode::SUCCESS},_=>panic!("unknown mode")}}
