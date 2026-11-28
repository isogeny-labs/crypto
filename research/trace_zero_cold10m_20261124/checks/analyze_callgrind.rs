//! Retrospective function/source-local Ir only. No new measurement.
use std::{collections::BTreeMap,env,fs};
fn symbol(s:&str, ids:&mut BTreeMap<u32,String>)->String {
 if let Some(x)=s.strip_prefix('(') {if let Some((n,name))=x.split_once(')') {let i=n.parse().unwrap();if !name.trim().is_empty(){ids.insert(i,name.trim().to_string());}return ids.get(&i).cloned().unwrap_or_else(||format!("unknown-{i}"));}}
 s.to_string()
}
fn tail<'a>(s:&'a str, keys:&[&str])->Option<&'a str>{keys.iter().find_map(|k|s.strip_prefix(k))}
fn main(){
 let args:Vec<_>=env::args().skip(1).collect();let mut functions=BTreeMap::new();let mut files=BTreeMap::new();
 for file in &args {for line in fs::read_to_string(file).unwrap().lines(){
  if let Some(s)=tail(line,&["fn=","cfn="]){symbol(s,&mut functions);}
  else if let Some(s)=tail(line,&["fl=","cfl=","fi=","fe=","cfi="]){symbol(s,&mut files);}
 }}
 for file in args {
  let text=fs::read_to_string(&file).unwrap();let mut fl=String::new();let mut fun=String::new();let mut skip=false;let mut counts=BTreeMap::<(String,String),u64>::new(); let mut positions=BTreeMap::<(String,i64),u64>::new();let mut position=0i64;
  for line in text.lines(){
   if let Some(s)=tail(line,&["fl=","fi=","fe="]){fl=symbol(s,&mut files);}
   else if let Some(s)=line.strip_prefix("fn="){fun=symbol(s,&mut functions);}
   else if line.starts_with("calls="){skip=true;}
   else if line.starts_with(|c:char|c.is_ascii_digit()||c=='+'||c=='-'||c=='*'){
    let cols:Vec<_>=line.split_whitespace().collect(); if let Some(pos)=cols.first(){position=match *pos {"*"=>position,x if x.starts_with('+')||x.starts_with('-')=>position+x.parse::<i64>().unwrap(),x=>x.parse().unwrap()};} if skip{skip=false;continue;}if cols.len()==2 {if let Ok(n)=cols[1].parse::<u64>(){*counts.entry((fl.clone(),fun.clone())).or_default()+=n;*positions.entry((fl.clone(),position)).or_default()+=n;}}
   }
  }
  let mut v:Vec<_>=counts.into_iter().collect();v.sort_by_key(|x|std::cmp::Reverse(x.1));println!("FILE {file}");for ((fl,fun),n) in v.into_iter().take(35){println!("{n:12} {fl}:{fun}");}let mut lines:Vec<_>=positions.into_iter().filter(|((file,_),_)|file.ends_with("trace_zero_linearized.rs")).collect();lines.sort_by_key(|x|std::cmp::Reverse(x.1));for ((_,line),n) in lines.into_iter().take(40){println!("LINE {line}: {n}");}
 }
}
