//! Prespecified training-only selection over fourteen complete curve models.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};

const ROOT: &str = "research/factor_base_heldout_20261025/results";
const BITS: [u64; 7] = [16, 20, 24, 28, 32, 36, 40];
const ARMS: [(u64, &str, u64); 7] = [
    (1, "any", 8),
    (1, "any", 16),
    (1, "any", 32),
    (2, "one", 16),
    (4, "one", 16),
    (2, "both", 16),
    (4, "both", 16),
];

fn rows(path: &str) -> Vec<Value> {
    BufReader::new(File::open(path).unwrap_or_else(|e| panic!("{path}: {e}")))
        .lines()
        .map(|line| serde_json::from_str(&line.unwrap()).unwrap())
        .collect()
}
fn num(v: &Value, key: &str) -> f64 {
    v[key].as_u64().unwrap_or_else(|| panic!("missing {key}")) as f64
}
fn logical(v: &Value) -> f64 {
    v.as_object()
        .unwrap()
        .iter()
        .filter(|(k, _)| k.ends_with("_uncharged"))
        .map(|(_, v)| v.as_u64().unwrap() as f64)
        .sum()
}
fn price(row: &Value, s: f64, i: f64, l: f64) -> f64 {
    let c = &row["counts"];
    let u = &row["source_add"];
    (num(c, "m") + s * num(c, "s") + i * num(c, "i"))
        / (num(u, "m") + s * num(u, "s") + i * num(u, "i"))
        + l * (logical(&row["native"]) + num(c, "field_adds") + num(c, "bit_ops"))
}
fn filename(d: u64, a: &str, j: u64, bits: u64) -> String {
    if a == "both" {
        format!("{ROOT}/raw/train-fb{d}-both-j{j}-bits{bits}.jsonl")
    } else {
        format!("research/factor_base_replication_20261024/results/raw/fb{d}-j{j}-bits{bits}.jsonl")
    }
}
fn file_sha(path: &str) -> String {
    let digest = Sha256::digest(fs::read(path).unwrap());
    digest.iter().map(|b| format!("{b:02x}")).collect()
}
fn main() {
    let mut all: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    let mut files = Vec::new();
    let mut models = BTreeSet::new();
    let mut base_ids: BTreeSet<String> = BTreeSet::new();
    for (d, a, j) in ARMS {
        let key = format!("fb{d}-{a}-j{j}");
        let mut ids = BTreeSet::new();
        for bits in BITS {
            let path = filename(d, a, j, bits);
            let source = rows(&path);
            assert_eq!(source.len(), if a == "both" { 120 } else { 180 }, "{path}");
            let mut curves: Vec<_> = source
                .iter()
                .map(|r| r["curve_icv1"].as_str().unwrap().to_owned())
                .collect();
            curves.sort();
            curves.dedup();
            if a == "both" {
                assert_eq!(curves.len(), 2);
            } else {
                assert_eq!(curves.len(), 3);
            }
            let evaluation = if a == "both" {
                None
            } else {
                Some(curves[2].clone())
            };
            for row in source {
                if evaluation.as_deref() == row["curve_icv1"].as_str() {
                    continue;
                }
                assert_eq!(row["bits"], bits);
                assert_eq!(row["detail"]["base_denominator"], d);
                assert_eq!(row["detail"]["jump_count"], j);
                if a == "both" {
                    assert_eq!(row["detail"]["large_prime_acceptance"], "both");
                }
                if row["verified"] == true {
                    assert_eq!(row["recovered"], row["planted"]);
                }
                if row["rho"]["verified"] == true {
                    assert_eq!(row["rho"]["recovered"], row["planted"]);
                }
                let id = row["workload_id"].as_str().unwrap().to_owned();
                assert!(ids.insert(id));
                models.insert((bits, row["curve_icv1"].as_str().unwrap().to_owned()));
                all.entry(key.clone()).or_default().push(row);
            }
            files.push(json!({"path":path,"sha256":file_sha(&path),"bytes":fs::metadata(&path).unwrap().len()}));
        }
        assert_eq!(ids.len(), 840);
        if base_ids.is_empty() {
            base_ids = ids;
        } else {
            assert_eq!(base_ids, ids, "target drift for {key}");
        }
    }
    assert_eq!(models.len(), 14);
    let mut csv = File::create(format!("{ROOT}/training-price-grid.csv")).unwrap();
    writeln!(csv,"policy,targets,verified,failures,S_per_M,I_per_M,logical_price,candidate_GAE,rho_GAE,candidate_over_rho").unwrap();
    let mut candidates = Vec::new();
    let mut best: Option<(f64, String)> = None;
    for (d, a, j) in ARMS {
        let key = format!("fb{d}-{a}-j{j}");
        let records = &all[&key];
        assert_eq!(records.len(), 840);
        let complete = records
            .iter()
            .all(|r| r["verified"] == true && r["rho"]["verified"] == true);
        let verified = records.iter().filter(|r| r["verified"] == true).count();
        let rho_verified = records
            .iter()
            .filter(|r| r["rho"]["verified"] == true)
            .count();
        let mut representative = None;
        for s in [0.0, 0.5, 1.0] {
            for i in [1.0, 10.0, 100.0, 1000.0] {
                for l in [0.0, 0.1, 1.0] {
                    let c =
                        complete.then(|| records.iter().map(|r| price(r, s, i, l)).sum::<f64>());
                    let rho = (rho_verified == 840).then(|| {
                        records
                            .iter()
                            .map(|r| price(&r["rho"], s, i, l))
                            .sum::<f64>()
                    });
                    let ratio = c.zip(rho).map(|(c, rho)| c / rho);
                    if s == 0.5 && i == 10.0 && l == 0.1 {
                        representative = ratio;
                    }
                    writeln!(
                        csv,
                        "{key},840,{verified},{},{s},{i},{l},{},{},{}",
                        840 - verified,
                        c.map_or(String::new(), |v| format!("{v:.12}")),
                        rho.map_or(String::new(), |v| format!("{v:.12}")),
                        ratio.map_or(String::new(), |v| format!("{v:.12}"))
                    )
                    .unwrap();
                }
            }
        }
        candidates.push(json!({"policy":key,"base_denominator":d,"large_prime_acceptance":a,"jumps":j,"targets":840,"verified":verified,"rho_verified":rho_verified,"failures":840-verified,"representative_ratio":representative}));
        if let Some(representative) = representative {
            if best.as_ref().is_none_or(|(score, winner)| {
                representative < *score || (representative == *score && key < *winner)
            }) {
                best = Some((representative, key));
            }
        }
    }
    let (score, selected) = best.unwrap();
    let output = json!({"schema":"factor-base-heldout-selection/v1","protocol_sha256":"325d0efb2c1e7b7328bd472bda310464f4817d62a5ca3a4ca6481c67cee84c9c","train_models":models,"train_targets_per_policy":840,"selection_price":{"S_per_M":0.5,"I_per_M":10,"logical_price":0.1},"candidate_scores":candidates,"selected_policy":selected,"selected_ratio":score,"input_files":files});
    fs::write(
        format!("{ROOT}/selection.json"),
        serde_json::to_vec_pretty(&output).unwrap(),
    )
    .unwrap();
    println!("{}", output["selected_policy"]);
}
