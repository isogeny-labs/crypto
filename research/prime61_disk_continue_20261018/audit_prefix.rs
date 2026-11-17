//! Check immutable restored rows against the extended native JSONL dump.
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};
fn rows(path: &str) -> BTreeMap<String, Value> {
    let mut out = BTreeMap::new();
    for line in BufReader::new(File::open(path).unwrap()).lines() {
        let mut v: Value = serde_json::from_str(&line.unwrap()).unwrap();
        let key = match v["kind"].as_str().unwrap() {
            "meta" => format!("meta/{}", v["key"]),
            "run" => format!("run/{}/{}", v["curve"], v["run"]),
            "curve" => {
                v.as_object_mut().unwrap().remove("expanded");
                v.as_object_mut().unwrap().remove("measured");
                format!("curve/{}", v["id"])
            }
            kind => format!("{kind}/{}", v["id"]),
        };
        assert!(out.insert(key, v).is_none(), "duplicate logical key");
    }
    out
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 3, "ARCHIVED.jsonl EXTENDED.jsonl");
    let before = rows(&args[1]);
    let after = rows(&args[2]);
    for (key, value) in &before {
        assert_eq!(after.get(key), Some(value), "changed immutable row: {key}");
    }
    println!(
        "{}",
        json!({"schema":"prime61.immutable-prefix-audit/v1", "status":"PASS", "inherited_rows_matched":before.len(), "final_rows":after.len(), "mutable_fields_excluded":["curve.expanded","curve.measured"]})
    );
}
