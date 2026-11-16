use serde_json::{Value,json};
use sha2::{Sha256,Digest};
use std::{fs,path::Path,collections::{BTreeMap,BTreeSet}};
fn conservation(v:&Value)->Result<(),String>{
 let cs=v["counts"].as_object().ok_or("counts")?;if cs.is_empty(){return Err("empty counters".into())}
 let ps=v["phases"].as_array().ok_or("phases")?;if ps.is_empty(){return Err("empty phases".into())}
 let mut names=BTreeSet::new();let mut sums:BTreeMap<String,u128>=BTreeMap::new();
 for p in ps{let n=p["name"].as_str().ok_or("phase name")?;if n.is_empty()||!names.insert(n){return Err("phase duplicate/empty".into())}
 let fields=p["counts"].as_object().ok_or("phase counts")?;if fields.keys().collect::<Vec<_>>()!=cs.keys().collect::<Vec<_>>(){return Err("counter keys".into())}
 for(k,x)in fields{let y=x.as_u64().ok_or("unsigned count")?;*sums.entry(k.clone()).or_default()+=y as u128;}}
 for(k,x)in cs{if sums[k]!=x.as_u64().ok_or("unsigned total")? as u128{return Err(format!("counter mismatch {k}"))}} Ok(())}
fn csv(s:&str)->Vec<String>{let mut out=vec![];let mut v=String::new();let mut q=false;let mut it=s.trim_end_matches('\r').chars().peekable();while let Some(c)=it.next(){match c{'"' if q&&it.peek()==Some(&'"')=>{v.push('"');it.next();},'"'=>q=!q,',' if !q=>out.push(std::mem::take(&mut v)),_=>v.push(c)}}assert!(!q);out.push(v);out}
fn main(){let root=std::env::args().nth(1).expect("root");let root=Path::new(&root);
 let good=json!({"counts":{"m":3,"i":1},"phases":[{"name":"setup","counts":{"m":1,"i":1}},{"name":"online","counts":{"m":2,"i":0}}]});assert!(conservation(&good).is_ok());
 let mut bad=vec![];let mut a=good.clone();a["phases"].as_array_mut().unwrap().pop();bad.push(a);let mut a=good.clone();a["counts"]["m"]=json!(4);bad.push(a);let mut a=good.clone();a["phases"][1]["name"]=json!("setup");bad.push(a);let mut a=good.clone();a["phases"][1]["counts"]["m"]=json!(-1);bad.push(a);let mut a=good.clone();a["phases"]=json!([]);bad.push(a);for a in bad{assert!(conservation(&a).is_err())}
 let csvpath=root.join("research/four_followups_review_20261022/results/standard-targets.csv");let text=fs::read_to_string(&csvpath).unwrap();let mut lines=text.lines();let h=csv(lines.next().unwrap());let idx=|s:&str|h.iter().position(|x|x==s).unwrap();let mut inputs=BTreeMap::new();inputs.insert(csvpath.display().to_string(),format!("{:x}",Sha256::digest(text.as_bytes())));
 let mut cache:BTreeMap<String,Option<BTreeMap<(u64,u64),Value>>>=BTreeMap::new();let mut summary:BTreeMap<String,BTreeMap<String,u64>>=BTreeMap::new();let mut issues=vec![];let mut standard_seen=BTreeSet::new();
 for line in lines{let r=csv(line);assert_eq!(r.len(),h.len());let lead=&r[idx("lead")];if lead!="factor_base"&&lead!="projective_law"{continue}let arm=format!("{}/{}",lead,r[idx("arm")]);assert!(standard_seen.insert((arm.clone(),r[idx("bits")].clone(),r[idx("p")].clone(),r[idx("target_index")].clone())),"duplicate standardized identity");let s=summary.entry(arm.clone()).or_default();*s.entry("standard_rows".into()).or_default()+=1;let src=r[idx("source_record")].clone();
 if !cache.contains_key(&src){let p=root.join(&src);let data=fs::read(&p).ok();let rows=data.map(|bytes|{inputs.insert(p.display().to_string(),format!("{:x}",Sha256::digest(&bytes)));let text=String::from_utf8(bytes).unwrap();let mut rows=BTreeMap::new();for l in text.lines(){let v:Value=serde_json::from_str(l).unwrap();let key=(v["p"].as_u64().unwrap(),v["target_index"].as_u64().unwrap());assert!(rows.insert(key,v).is_none(),"duplicate key")}rows});cache.insert(src.clone(),rows);}
 let Some(rows)=cache[&src].as_ref() else{*s.entry("missing_source_rows".into()).or_default()+=1;continue};let key=(r[idx("p")].parse().unwrap(),r[idx("target_index")].parse().unwrap());let Some(v)=rows.get(&key) else{*s.entry("missing_native_key".into()).or_default()+=1;continue};
 if v["bits"].as_u64()!=Some(r[idx("bits")].parse().unwrap())||v["workload_id"].as_str()!=Some(&r[idx("workload_id")]){issues.push(json!({"arm":arm,"key":key,"issue":"workload identity"}));continue}
 for(name,row)in[("candidate",v),("rho",&v["rho"]) ]{match conservation(row){Ok(())=>{*s.entry(format!("{name}_phase_conservation_pass")).or_default()+=1;},Err(e)=>{*s.entry(format!("{name}_phase_conservation_unresolved")).or_default()+=1;issues.push(json!({"arm":arm,"key":key,"side":name,"issue":e}));}}}
 }
 for(path,sha)in &inputs{assert_eq!(format!("{:x}",Sha256::digest(fs::read(path).unwrap())),*sha,"input changed")}
 println!("{}",serde_json::to_string_pretty(&json!({"schema":"saved-phase-conservation/v1","scientific_execution":false,"scalar_replay":false,"new_price_ratio":null,"controls":{"positive":1,"negative":5},"arms":summary,"issues":issues,"inputs":inputs,"inputs_unchanged":true})).unwrap());}
