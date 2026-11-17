use pencil_native_input_custody::*;
use serde_json::{Value,json};
use std::{fs,path::PathBuf,collections::BTreeMap,os::unix::fs::{PermissionsExt,symlink},os::unix::net::UnixListener};
fn mode(p:&PathBuf,m:u32){fs::set_permissions(p,fs::Permissions::from_mode(m)).unwrap()}
fn record(v:&mut Vec<Value>,name:&str,r:Result<Value,String>,expected:bool){assert_eq!(r.is_ok(),expected,"{name}: {r:?}");v.push(json!({"case":name,"expected":expected,"accepted":r.is_ok(),"error":r.err()}));}
fn main(){
 let base=PathBuf::from(std::env::args().nth(1).unwrap());assert!(!base.exists());fs::create_dir(&base).unwrap();let base=fs::canonicalize(base).unwrap();
 let replay=PathBuf::from(std::env::args().nth(2).unwrap());
 let fixtures=base.join("fixtures");fs::create_dir(&fixtures).unwrap();let file=fixtures.join("member");fs::write(&file,b"independent bytes").unwrap();mode(&file,0o444);mode(&fixtures,0o555);
 let gate=base.join("gate");fs::write(&gate,b"inert").unwrap();mode(&gate,0o555);
 let c=Custody{fixtures:fixtures.clone(),files:BTreeMap::from([("member".into(),digest(b"independent bytes"))]),gate:gate.clone(),gate_sha256:digest(b"inert"),file_cap:1024};
 let mut rows=vec![];
 for (name,raw,wanted) in [
 ("escaped-null-duplicate",r#"{"\u0000":0,"\u0000":1}"#,false),
 ("surrogate-alias-duplicate",r#"{"𝄞":0,"\uD834\uDD1E":1}"#,false),
 ("deep-array-duplicate",r#"[[{"b":{"key":1,"k\u0065y":2}}]]"#,false),
 ("distinct-normalization",r#"{"é":0,"e\u0301":1}"#,true),
 ("lone-surrogate",r#"{"x":"\uD800"}"#,false),
 ("trailing-comma",r#"{"x":1,}"#,false),
 ("comment",r#"{}/*bad*/"#,false),
 ("leading-zero",r#"01"#,false),
 ("unescaped-control","\"a\nb\"",false),
 ("trailing-token","true false",false),
 ("array-scalar-json","[null,false,0]",true)] {record(&mut rows,name,strict_json(raw.as_bytes(),1024),wanted)}
 record(&mut rows,"exact-json-cap",strict_json(b"{}",2),true);
 record(&mut rows,"zero-json-cap",strict_json(b"0",0),false);
 let deep=format!("{}0{}","[".repeat(200),"]".repeat(200));record(&mut rows,"serde-depth-limit",strict_json(deep.as_bytes(),1024),false);
 record(&mut rows,"fresh-custody",snapshot(&c),true);
 let mut bad=c.clone();bad.files.clear();record(&mut rows,"empty-inventory",snapshot(&bad),false);
 bad=c.clone();bad.file_cap=17;record(&mut rows,"exact-file-cap",snapshot(&bad),true);
 bad.file_cap=16;record(&mut rows,"one-byte-over-cap",snapshot(&bad),false);
 bad=c.clone();let pin=bad.files.remove("member").unwrap();bad.files.insert("member/".into(),pin);record(&mut rows,"trailing-slash-key-alias",snapshot(&bad),false);
 bad=c.clone();bad.fixtures=PathBuf::from(format!("{}/./fixtures",base.display()));record(&mut rows,"dot-path",snapshot(&bad),false);
 bad.fixtures=PathBuf::from(format!("{}//fixtures",base.display()));record(&mut rows,"double-slash-path",snapshot(&bad),false);
 bad=c.clone();bad.gate_sha256=bad.gate_sha256.to_uppercase();record(&mut rows,"uppercase-digest",snapshot(&bad),false);
 mode(&file,0o4444);record(&mut rows,"special-mode-bit",snapshot(&c),false);mode(&file,0o444);
 let h=base.join("gatehard");fs::hard_link(&gate,&h).unwrap();record(&mut rows,"gate-hardlink",snapshot(&c),false);fs::remove_file(&h).unwrap();
 let s=base.join("gatesym");symlink(&gate,&s).unwrap();bad=c.clone();bad.gate=s;record(&mut rows,"gate-symlink",snapshot(&bad),false);
 mode(&fixtures,0o755);fs::remove_file(&file).unwrap();fs::create_dir(&file).unwrap();mode(&file,0o444);mode(&fixtures,0o555);record(&mut rows,"directory-member",snapshot(&c),false);
 mode(&fixtures,0o755);fs::remove_dir(&file).unwrap();let socket=UnixListener::bind(&file).unwrap();mode(&file,0o444);mode(&fixtures,0o555);record(&mut rows,"socket-member-before-open",snapshot(&c),false);drop(socket);
 mode(&fixtures,0o755);fs::remove_file(&file).unwrap();fs::write(&file,b"independent bytes").unwrap();mode(&file,0o444);let extra=fixtures.join("extra");fs::write(&extra,b"").unwrap();mode(&extra,0o444);mode(&fixtures,0o555);record(&mut rows,"extra-empty-file",snapshot(&c),false);
 mode(&fixtures,0o755);fs::remove_file(&extra).unwrap();mode(&fixtures,0o555);
 let empty=base.join("empty");fs::write(&empty,b"").unwrap();mode(&empty,0o444);record(&mut rows,"empty-file-allowed-with-pin",bound_file(&empty,&digest(b""),0o444,0),true);
 let mut inspect:Value=serde_json::from_slice(&fs::read(replay.join("inspect.json")).unwrap()).unwrap();let mut expected:Value=serde_json::from_slice(&fs::read(replay.join("expected.json")).unwrap()).unwrap();
 expected["fixtures_source"]=json!(fixtures);expected["gate_source"]=json!(gate);inspect["Mounts"][0]["Source"]=json!(fixtures);inspect["Mounts"][1]["Source"]=json!(gate);
 let diff=b"A /qualification/exec-gate\nA /fixtures\nA /qualification\n";
 let ir=serde_json::to_vec(&inspect).unwrap();let er=serde_json::to_vec(&expected).unwrap();
 let pass=qualify_inputs("Q1",0,diff,&ir,&er,&c).unwrap();assert_eq!(pass["runtime_qualification"],false);record(&mut rows,"actual-composition",Ok(pass),true);
 let mut altered=inspect.clone();altered["Mounts"][0]["RW"]=json!(true);record(&mut rows,"composition-writable-mount",qualify_inputs("Q1",0,diff,&serde_json::to_vec(&altered).unwrap(),&er,&c),false);
 altered=expected.clone();altered["owner_nonce"]=json!("00000000-0000-0000-0000-000000000000");record(&mut rows,"composition-owner-mismatch",qualify_inputs("Q1",0,diff,&ir,&serde_json::to_vec(&altered).unwrap(),&c),false);
 record(&mut rows,"composition-transport",qualify_inputs("Q1",17,diff,&ir,&er,&c),false);
 record(&mut rows,"composition-extra-diff",qualify_inputs("Q1",0,b"A /fixtures\nA /qualification\nA /qualification/exec-gate\nC /etc\n",&ir,&er,&c),false);
 let mut dup=String::from_utf8(er.clone()).unwrap();dup.pop();dup.push_str(",\"gate_sourc\\u0065\":\"other\"}");record(&mut rows,"composition-expected-alias",qualify_inputs("Q1",0,diff,&ir,dup.as_bytes(),&c),false);
 mode(&gate,0o755);record(&mut rows,"composition-custody-mode",qualify_inputs("Q1",0,diff,&ir,&er,&c),false);mode(&gate,0o555);
 record(&mut rows,"final-composition",qualify_inputs("Q1",0,diff,&ir,&er,&c),true);
 println!("{}",serde_json::to_string_pretty(&json!({"accepted":true,"count":rows.len(),"rows":rows,"scientific_execution":false,"runtime_qualification":false})).unwrap());
}
