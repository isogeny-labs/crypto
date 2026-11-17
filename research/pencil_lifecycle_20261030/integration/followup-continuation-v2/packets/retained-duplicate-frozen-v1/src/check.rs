mod graph;mod pruned;mod duplicate;
fn main(){let base:serde_json::Value=serde_json::from_str(include_str!("../fixture.json")).unwrap();for (i,line) in include_str!("../archive.jsonl").lines().enumerate(){let v:serde_json::Value=serde_json::from_str(line).unwrap();if v["p"]==739{pruned::archive_target(&base,&v,i);}}}
