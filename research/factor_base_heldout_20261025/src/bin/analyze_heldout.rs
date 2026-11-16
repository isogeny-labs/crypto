//! Price the frozen evaluation policy; failed targets retain null ratios.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
const ROOT: &str = "research/factor_base_heldout_20261025/results";
const BITS: [u64; 7] = [16, 20, 24, 28, 32, 36, 40];
fn csv_field(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_owned()
    }
}
fn num(v: &Value, k: &str) -> f64 {
    v[k].as_u64().unwrap() as f64
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
fn main() {
    let selection: Value =
        serde_json::from_str(&fs::read_to_string(format!("{ROOT}/selection.json")).unwrap())
            .unwrap();
    let mut groups: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    let mut ids = BTreeSet::new();
    let mut target_json = File::create(format!("{ROOT}/evaluation-targets.jsonl")).unwrap();
    let mut target_csv = File::create(format!("{ROOT}/evaluation-targets.csv")).unwrap();
    writeln!(target_csv,"workload_id,bits,curve_icv1,p,target_index,planted,recovered,verified,rho_recovered,rho_verified,candidate_GAE,rho_GAE,candidate_over_rho,processed_divisors,rejected_large_large,raw_record").unwrap();
    let mut verified = 0u64;
    let mut rho_verified = 0u64;
    let mut failures = 0u64;
    for bits in BITS {
        let path = format!("{ROOT}/raw/evaluation-bits{bits}.jsonl");
        let rs: Vec<Value> = BufReader::new(File::open(&path).unwrap())
            .lines()
            .map(|l| serde_json::from_str(&l.unwrap()).unwrap())
            .collect();
        assert_eq!(rs.len(), 60, "{path}");
        let mut models = BTreeSet::new();
        for row in rs {
            assert_eq!(row["schema"], "factor-base-heldout-record/v1");
            assert_eq!(
                row["replication_id"],
                "factor-base-heldout-evaluation/20261025/v1"
            );
            assert_eq!(row["protocol_sha256"], selection["protocol_sha256"]);
            let id = row["workload_id"].as_str().unwrap().to_owned();
            assert!(ids.insert(id));
            let model = row["curve_icv1"].as_str().unwrap().to_owned();
            models.insert(model.clone());
            let candidate_ok = row["verified"] == true && row["recovered"] == row["planted"];
            let rho_ok =
                row["rho"]["verified"] == true && row["rho"]["recovered"] == row["planted"];
            if candidate_ok {
                verified += 1
            } else {
                failures += 1
            };
            if rho_ok {
                rho_verified += 1
            };
            let c = if candidate_ok {
                Some(price(&row, 0.5, 10.0, 0.1))
            } else {
                None
            };
            let r = if rho_ok {
                Some(price(&row["rho"], 0.5, 10.0, 0.1))
            } else {
                None
            };
            let ratio = match (c, r) {
                (Some(c), Some(r)) => Some(c / r),
                _ => None,
            };
            let summary = json!({"schema":"factor-base-heldout-target/v1","workload_id":row["workload_id"],"bits":bits,"curve_icv1":model,"p":row["p"],"target_index":row["target_index"],"planted":row["planted"],"recovered":row["recovered"],"verified":candidate_ok,"rho_recovered":row["rho"]["recovered"],"rho_verified":rho_ok,"candidate_GAE":c,"rho_GAE":r,"candidate_over_rho":ratio,"processed_divisors":row["native"]["processed_divisors"],"rejected_large_large":row["native"]["rejected_large_large_relations"],"raw_record":path});
            writeln!(target_json, "{summary}").unwrap();
            writeln!(
                target_csv,
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                row["workload_id"].as_str().unwrap(),
                bits,
                csv_field(&model),
                row["p"],
                row["target_index"],
                row["planted"],
                row["recovered"],
                candidate_ok,
                row["rho"]["recovered"],
                rho_ok,
                c.map_or(String::new(), |v| v.to_string()),
                r.map_or(String::new(), |v| v.to_string()),
                ratio.map_or(String::new(), |v| v.to_string()),
                row["native"]["processed_divisors"],
                row["native"]["rejected_large_large_relations"],
                path
            )
            .unwrap();
            groups.entry("all".into()).or_default().push(row.clone());
            groups
                .entry(format!("bits-{bits}"))
                .or_default()
                .push(row.clone());
            groups
                .entry(format!("model-{model}"))
                .or_default()
                .push(row);
        }
        assert_eq!(models.len(), 1);
    }
    assert_eq!(ids.len(), 420);
    let mut grid = File::create(format!("{ROOT}/evaluation-price-grid.csv")).unwrap();
    writeln!(grid,"scope,targets,verified,failures,rho_verified,S_per_M,I_per_M,logical_price,candidate_GAE,rho_GAE,candidate_over_rho,outcome").unwrap();
    let mut summary = Vec::new();
    for (scope, rs) in groups {
        let good: Vec<_> = rs
            .iter()
            .filter(|r| {
                r["verified"] == true
                    && r["recovered"] == r["planted"]
                    && r["rho"]["verified"] == true
                    && r["rho"]["recovered"] == r["planted"]
            })
            .collect();
        let mut representative = Value::Null;
        let mut wins = 0;
        let mut losses = 0;
        let mut ties = 0;
        for s in [0.0, 0.5, 1.0] {
            for i in [1.0, 10.0, 100.0, 1000.0] {
                for l in [0.0, 0.1, 1.0] {
                    let candidate: Option<f64> = if good.len() == rs.len() {
                        Some(rs.iter().map(|r| price(r, s, i, l)).sum())
                    } else {
                        None
                    };
                    let rho: Option<f64> = if rs.iter().all(|r| r["rho"]["verified"] == true) {
                        Some(rs.iter().map(|r| price(&r["rho"], s, i, l)).sum())
                    } else {
                        None
                    };
                    let ratio = match candidate.zip(rho) {
                        Some((c, r)) => Some(c / r),
                        None => None,
                    };
                    let outcome = match ratio {
                        Some(x) if x < 1.0 => {
                            wins += 1;
                            "win"
                        }
                        Some(x) if x > 1.0 => {
                            losses += 1;
                            "loss"
                        }
                        Some(_) => {
                            ties += 1;
                            "tie"
                        }
                        None => "null",
                    };
                    if s == 0.5 && i == 10.0 && l == 0.1 {
                        representative = json!({"candidate_GAE":candidate,"rho_GAE":rho,"candidate_over_rho":ratio});
                    }
                    writeln!(
                        grid,
                        "{},{},{},{},{},{s},{i},{l},{},{},{},{}",
                        csv_field(&scope),
                        rs.len(),
                        good.len(),
                        rs.len() - good.len(),
                        rs.iter().filter(|r| r["rho"]["verified"] == true).count(),
                        candidate.map_or(String::new(), |v| v.to_string()),
                        rho.map_or(String::new(), |v| v.to_string()),
                        ratio.map_or(String::new(), |v| v.to_string()),
                        outcome
                    )
                    .unwrap();
                }
            }
        }
        summary.push(json!({"scope":scope,"targets":rs.len(),"verified":good.len(),"failures":rs.len()-good.len(),"representative":representative,"wins":wins,"losses":losses,"ties":ties}));
    }
    let selection_sha256 = Sha256::digest(fs::read(format!("{ROOT}/selection.json")).unwrap())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>();
    let output = json!({"schema":"factor-base-heldout-summary/v1","selected_policy":selection["selected_policy"],"selection_sha256":selection_sha256,"evaluation_targets":420,"verified":verified,"rho_verified":rho_verified,"failures":failures,"summary":summary});
    fs::write(
        format!("{ROOT}/evaluation-summary.json"),
        serde_json::to_vec_pretty(&output).unwrap(),
    )
    .unwrap();
    println!("verified {verified}/420, rho {rho_verified}/420, failures {failures}");
}
