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
 let arg=std::env::args().nth(1).unwrap();let d=Path::new(&arg);
 let mods=["FastPreludeExpr","FastPreludeDecode","FastPreludeControls","FastPreludeReadback","FastPayloadWords","FastPayloadPolynomial","FastPayloadControls","FastPayloadReadback"];
 for m in mods {assert_eq!(fs::read_to_string(d.join(format!("logs/{m}.exit"))).unwrap().trim(),"0");}
 assert_eq!(fs::read_to_string(d.join("logs/ExpectedFailure.exit")).unwrap().trim(),"1");
 let mut count=0;
 for m in ["FastPreludeReadback","FastPayloadReadback"] {
  let src=fs::read_to_string(d.join(format!("input/{m}.lean"))).unwrap();
  let names:Vec<String>=src.lines().filter_map(|l|l.strip_prefix("#print axioms ").map(str::to_owned)).collect();
  let out=fs::read_to_string(d.join(format!("logs/{m}.stdout"))).unwrap();
  assert!(audit(&names,&out));assert!(!audit(&names,""));
  assert!(!audit(&names,&out.replacen("propext","sorryAx",1)));
  assert!(!audit(&names,&out.replacen(&names[0],"Wrong.theorem",1)));
  count+=names.len();
 }
 assert_eq!(count,27);
 println!("{{\"passed\":true,\"fresh_modules\":8,\"named_readbacks\":27,\"false_exit\":1,\"checker_rejections\":6}}");
}
