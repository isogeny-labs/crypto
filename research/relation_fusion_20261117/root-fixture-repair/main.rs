pub use crypto_lib::{binary_ecc, hash, cryptanalysis};
mod geometric;
#[allow(dead_code)]
#[path = "old/curve_id.rs"]
mod legacy;
#[allow(dead_code)]
#[path = "sorted/curve_id.rs"]
mod sorted;
use serde_json::json;
fn main() {
    let mode = std::env::args().nth(1).expect("aliases-before or aliases-after");
    if mode == "geometric" { geometric::run(); return; }
    assert!(["aliases-before", "aliases-after"].contains(&mode.as_str()));
    let table: serde_json::Value = serde_json::from_str(include_str!("old/curve_aliases.json")).unwrap();
    let map = table["aliases"].as_object().unwrap();
    let resolve: fn(&str)->Option<&'static str> = if mode == "aliases-before" { legacy::resolve } else { sorted::resolve };
    let mut misses = vec![];
    for (key,value) in map {
        assert_eq!(sorted::normalise(key), *key, "reference key must already be normalized");
        let expected = value.as_str();
        let got = resolve(key);
        if got != expected { misses.push(json!({"name":key,"expected":expected,"actual":got})); }
        let between=format!("{key}~");
        assert_eq!(resolve(&between), map.get(&between).and_then(|v| v.as_str()), "between {key}");
    }
    for key in ["", "no-such-curve", "icv1-fp20-t0-00000000", "a\"b", "a\\b", "secp256k", "!", "~~~~", "0"] {
        assert_eq!(resolve(key), map.get(key).and_then(|v|v.as_str()));
    }
    println!("{}",json!({"schema":"root-alias-order-qualification/v1","mode":mode,"aliases_checked":map.len(),"mismatches":misses,"all_values_preserved":true,"performance_comparison":false}));
    if !misses.is_empty() { std::process::exit(1); }
}
