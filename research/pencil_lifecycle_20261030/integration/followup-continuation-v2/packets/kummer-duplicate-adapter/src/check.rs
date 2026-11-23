mod graph;mod pruned;mod duplicate;
fn main(){let v:serde_json::Value=serde_json::from_str(include_str!("../fixture.json")).unwrap();println!("{}",pruned::duplicate_fixture(&v));}
