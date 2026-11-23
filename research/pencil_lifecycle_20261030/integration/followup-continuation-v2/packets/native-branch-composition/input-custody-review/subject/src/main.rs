use pencil_native_input_custody::*;
use serde_json::{Value,json};
use std::{fs,path::Path,collections::BTreeMap,os::unix::fs::{PermissionsExt,symlink}};
fn mode(p:&Path,m:u32){fs::set_permissions(p,fs::Permissions::from_mode(m)).unwrap()}
fn record(rows:&mut Vec<Value>,name:&str,r:Result<Value,String>,wanted:bool){assert_eq!(r.is_ok(),wanted,"{name}: {r:?}");rows.push(json!({"name":name,"expected_accept":wanted,"accepted":r.is_ok(),"error":r.err()}));}
fn main(){
 let root=std::env::args().nth(1).expect("fresh fixed-control directory");let root=Path::new(&root);assert!(!root.exists());fs::create_dir(root).unwrap();let root=fs::canonicalize(root).unwrap();
 let dir=root.join("fixtures");fs::create_dir(&dir).unwrap();let member=dir.join("request.json");let contents=b"{\"fixed\":true}\n";fs::write(&member,contents).unwrap();mode(&member,0o444);mode(&dir,0o555);
 let gate=root.join("gate");fs::write(&gate,b"inert gate fixture\n").unwrap();mode(&gate,0o555);
 let c=Custody{fixtures:dir.clone(),files:BTreeMap::from([("request.json".into(),digest(contents))]),gate:gate.clone(),gate_sha256:digest(b"inert gate fixture\n"),file_cap:4096};let mut rows=Vec::new();
 for (name,raw,wanted) in [("valid-json",r#"{"a":[null,true,1,-2,0.25,"x"],"b":{}}"#,true),("duplicate",r#"{"a":1,"a":2}"#,false),("nested-duplicate",r#"{"a":{"x":1,"x":2}}"#,false),("array-duplicate",r#"[{"x":1,"x":2}]"#,false),("escaped-duplicate",r#"{"a":1,"\u0061":2}"#,false),("trailing-value","{} []",false),("nan","NaN",false),("infinite","1e400",false),("truncated","{",false),("empty","",false),("whitespace"," {} \n",true)]{record(&mut rows,name,strict_json(raw.as_bytes(),4096),wanted)}
 record(&mut rows,"json-cap",strict_json(b"{}",1),false);record(&mut rows,"nonutf8",strict_json(&[255],100),false);record(&mut rows,"custody-valid",snapshot(&c),true);
 let mut bad=c.clone();bad.files.insert("request.json".into(),"0".repeat(64));record(&mut rows,"wrong-digest",snapshot(&bad),false);
 bad=c.clone();bad.gate_sha256="0".repeat(64);record(&mut rows,"wrong-gate-digest",snapshot(&bad),false);
 bad=c.clone();bad.file_cap=1;record(&mut rows,"file-cap",snapshot(&bad),false);
 bad=c.clone();bad.files.clear();record(&mut rows,"empty-inventory",snapshot(&bad),false);
 bad=c.clone();bad.files.insert("../request.json".into(),digest(contents));record(&mut rows,"parent-member",snapshot(&bad),false);
 bad=c.clone();bad.fixtures=root.join("fixtures/../fixtures");record(&mut rows,"parent-directory",snapshot(&bad),false);
 bad=c.clone();bad.fixtures=std::path::PathBuf::from("fixtures");record(&mut rows,"relative-directory",snapshot(&bad),false);
 mode(&dir,0o755);record(&mut rows,"directory-writable",snapshot(&c),false);mode(&dir,0o555);
 mode(&member,0o644);record(&mut rows,"member-writable",snapshot(&c),false);mode(&member,0o444);
 mode(&gate,0o755);record(&mut rows,"gate-writable",snapshot(&c),false);mode(&gate,0o555);
 let alias=root.join("alias");symlink(&dir,&alias).unwrap();bad=c.clone();bad.fixtures=alias;record(&mut rows,"symlink-directory",snapshot(&bad),false);
 let hard=root.join("hard");fs::hard_link(&member,&hard).unwrap();record(&mut rows,"hardlink-member",snapshot(&c),false);fs::remove_file(hard).unwrap();
 mode(&dir,0o755);let extra=dir.join("extra");fs::write(&extra,b"extra").unwrap();mode(&extra,0o444);mode(&dir,0o555);record(&mut rows,"extra-member",snapshot(&c),false);mode(&dir,0o755);fs::remove_file(extra).unwrap();mode(&dir,0o555);
 let nonce="0159ca8a-9eec-479a-9aa2-04d62b131f89";let id="c".repeat(64);let image=format!("sha256:{}","d".repeat(64));let name=format!("isolab-runtime-q-{nonce}");
 let expected=json!({"container_id":id,"image_id":image,"container_name":name,"owner_nonce":nonce,"fixtures_source":dir,"gate_source":gate});
 let inspect=json!({"Id":id,"Image":image,"Name":format!("/{name}"),"Config":{"User":"1000:1000","Entrypoint":["/qualification/exec-gate"],"Labels":{"isolab.runtime-qualification.owner":nonce}},"State":{"Running":false,"ExitCode":0},"HostConfig":{"ReadonlyRootfs":true,"NetworkMode":"none","PidsLimit":1,"Memory":536870912,"MemorySwap":536870912,"NanoCpus":1000000000,"CapDrop":["ALL"],"SecurityOpt":["no-new-privileges"],"Privileged":false,"PidMode":"","UTSMode":"","IpcMode":"private","CgroupnsMode":"private","Devices":[],"DeviceRequests":null,"CapAdd":null,"Tmpfs":{"/tmp":"rw,noexec,nosuid,nodev,size=16m,mode=1777","/work":"rw,noexec,nosuid,nodev,size=16m,mode=1777"}},"Mounts":[{"Type":"bind","Source":dir,"Destination":"/fixtures","RW":false},{"Type":"bind","Source":gate,"Destination":"/qualification/exec-gate","RW":false}]});
 let eraw=serde_json::to_vec(&expected).unwrap();let diff=b"A /fixtures\nA /qualification\nA /qualification/exec-gate\n";
 for (case,code) in [("Q1",0),("Q2",2),("Q3",125)]{let mut i=inspect.clone();i["State"]["ExitCode"]=json!(code);record(&mut rows,case,qualify_inputs(case,0,diff,&serde_json::to_vec(&i).unwrap(),&eraw,&c),true)}
 let iraw=serde_json::to_vec(&inspect).unwrap();let mut wrong=expected.clone();wrong["gate_source"]=json!(root.join("other"));record(&mut rows,"custody-context-mismatch",qualify_inputs("Q1",0,diff,&iraw,&serde_json::to_vec(&wrong).unwrap(),&c),false);
 record(&mut rows,"valid-custody-bad-diff",qualify_inputs("Q1",0,b"",&iraw,&eraw,&c),false);
 record(&mut rows,"valid-custody-bad-transport",qualify_inputs("Q1",1,diff,&iraw,&eraw,&c),false);
 let mut dup=String::from_utf8(iraw.clone()).unwrap();dup.pop();dup.push_str(",\"State\":{\"Running\":false,\"ExitCode\":0}}");record(&mut rows,"duplicate-inspect-composed",qualify_inputs("Q1",0,diff,dup.as_bytes(),&eraw,&c),false);
 record(&mut rows,"final-custody",snapshot(&c),true);
 fs::write(root.join("inspect.json"),&iraw).unwrap();fs::write(root.join("expected.json"),&eraw).unwrap();
 println!("{}",serde_json::to_string_pretty(&json!({"schema":"software.native-input-custody-controls/v1","accepted":true,"count":rows.len(),"rows":rows,"fixture_directory":root,"scientific_execution":false,"runtime_qualification":false,"gate_executed":false})).unwrap());
}
