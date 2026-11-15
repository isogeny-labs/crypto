//! Descriptive native analysis; no extrapolation or significance test.
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn quantile(v: &mut [u128], numerator: usize, denominator: usize) -> Option<String> {
    v.sort_unstable();
    if v.is_empty() {
        return None;
    }
    Some(v[(v.len() - 1) * numerator / denominator].to_string())
}
fn main() {
    let files: Vec<_> = std::env::args().skip(1).collect();
    assert!(
        !files.is_empty(),
        "usage: vieta-coset-analyze records.jsonl [...]"
    );
    let mut rows: BTreeMap<(String, u64, String, String, String), Vec<Value>> = BTreeMap::new();
    for path in files {
        for line in BufReader::new(File::open(path).unwrap()).lines() {
            let row: Value = serde_json::from_str(&line.unwrap()).unwrap();
            if row["warmup"] == true {
                continue;
            }
            let s = &row["scope"];
            if !s.is_object() {
                continue;
            }
            let key = (
                s["curve"].as_str().unwrap().into(),
                s["m"].as_u64().unwrap(),
                row["kind"].as_str().unwrap().into(),
                row["population"].as_str().unwrap_or("setup").into(),
                row["arm"].as_str().unwrap_or("setup").into(),
            );
            rows.entry(key).or_default().push(row);
        }
    }
    let mut out = vec![];
    for ((curve, m, kind, population, arm), group) in rows {
        let success: Vec<_> = group
            .iter()
            .filter(|r| r["status"].as_str().is_none_or(|s| s == "ok"))
            .collect();
        let mut ns: Vec<_> = success
            .iter()
            .filter_map(|r| r["elapsed_ns"].as_str().map(|s| s.parse::<u128>().unwrap()))
            .collect();
        let n = ns.len();
        let low = quantile(&mut ns, 1, 4);
        let median = quantile(&mut ns, 1, 2);
        let high = quantile(&mut ns, 3, 4);
        let mut muls: Vec<_> = success
            .iter()
            .filter_map(|r| r["work"]["muls"].as_u64().map(u128::from))
            .collect();
        out.push(json!({"curve":curve,"m":m,"kind":kind,"population":population,"arm":arm,"attempted":group.len(),"succeeded":success.len(),"failed":group.len()-success.len(),"timed_rows":n,"elapsed_ns_lower_quartile":low,"elapsed_ns_median":median,"elapsed_ns_upper_quartile":high,"muls_median":quantile(&mut muls,1,2),"relation_counts":success.iter().filter_map(|r|r["relations"].as_u64()).collect::<Vec<_>>() }));
    }
    println!(
        "{}",
        json!({"schema":"vieta-coset-analysis/v1","quantile":"sorted floor((n-1)*q), descriptive only","groups":out,"standing":"unverified; read isolation and A/A before interpreting clocks"})
    );
}
