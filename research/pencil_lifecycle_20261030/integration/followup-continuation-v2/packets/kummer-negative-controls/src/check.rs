mod pruned;fn main(){let f:serde_json::Value=serde_json::from_str(include_str!("../fixtures.json")).unwrap();println!("{}",pruned::fixed_negative_controls(&f[0]));}
