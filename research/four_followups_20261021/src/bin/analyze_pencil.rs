//! Price only complete, source-verified pencil files; partial runs stay raw.
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

const OUT: &str = "research/four_followups_20261021/results/pencil-pilot";

fn n(row: &Value, key: &str) -> f64 {
    row[key].as_u64().unwrap_or_else(|| panic!("missing {key}")) as f64
}
fn bit_count(x: u64) -> u64 {
    64 - x.leading_zeros() as u64 + x.count_ones() as u64
}
fn logical(row: &Value) -> f64 {
    row.as_object()
        .unwrap()
        .iter()
        .filter(|(key, _)| key.ends_with("_uncharged"))
        .map(|(_, value)| value.as_u64().unwrap() as f64)
        .sum()
}
fn price_pencil(row: &Value, square: f64, inversion: f64, native: f64) -> f64 {
    let c = &row["counts"];
    let a = &row["arithmetic"];
    let unit = &row["source_add"];
    let p = row["p"].as_u64().unwrap();
    let direct_inverse_muls = n(a, "base_inversions") * bit_count(p - 2) as f64;
    let direct_muls_outside_inverse = n(a, "base_muls") - direct_inverse_muls;
    assert!(direct_muls_outside_inverse >= 0.0);
    let numerator = n(c, "m")
        + direct_muls_outside_inverse
        + square * n(c, "s")
        + inversion * (n(c, "i") + n(a, "base_inversions"));
    let denominator = n(unit, "m") + square * n(unit, "s") + inversion * n(unit, "i");
    assert!(denominator > 0.0);
    let logical_work = n(c, "field_adds")
        + n(c, "bit_ops")
        + n(a, "base_adds")
        + n(a, "scalar_adds")
        + n(a, "scalar_muls")
        + n(a, "scalar_inversions")
        + n(row, "solver_row_updates");
    numerator / denominator + native * logical_work
}
fn price_rho(row: &Value, square: f64, inversion: f64, native: f64) -> f64 {
    let c = &row["rho"]["counts"];
    let unit = &row["source_add"];
    let numerator = n(c, "m") + square * n(c, "s") + inversion * n(c, "i");
    let denominator = n(unit, "m") + square * n(unit, "s") + inversion * n(unit, "i");
    assert!(denominator > 0.0);
    numerator / denominator
        + native * (logical(&row["rho"]["native"]) + n(c, "field_adds") + n(c, "bit_ops"))
}
fn main() {
    let files: Vec<String> = std::env::args().skip(1).collect();
    assert!(!files.is_empty(), "usage: analyze-pencil FILE.jsonl ...");
    let mut summaries = Vec::new();
    let mut grid = File::create(format!("{OUT}/price-grid.csv")).unwrap();
    writeln!(grid, "bits,p,targets,verified,failures,S_per_M,I_per_M,native_GAE_per_logical,pencil_GAE,rho_GAE,pencil_over_rho,verified_relations,solver_row_updates,stored_nonzero_coefficients").unwrap();
    for path in files {
        let input: Vec<Value> = BufReader::new(File::open(&path).unwrap())
            .lines()
            .map(|line| serde_json::from_str(&line.unwrap()).unwrap())
            .collect();
        assert_eq!(input.len(), 61, "{path}: expected setup plus 60 targets");
        assert_eq!(input[0]["schema"], "pencil-cohort-setup/v1");
        let rows = &input[1..];
        let p = input[0]["p"].as_u64().unwrap();
        let bits = input[0]["bits"].as_u64().unwrap();
        let control_file =
            format!("research/four_followups_20261021/results/matched/fb1-j16-bits{bits}.jsonl");
        let controls: BTreeMap<String, Value> = BufReader::new(File::open(control_file).unwrap())
            .lines()
            .map(|line| serde_json::from_str::<Value>(&line.unwrap()).unwrap())
            .filter(|row| row["p"] == p)
            .map(|row| (row["workload_id"].as_str().unwrap().to_owned(), row))
            .collect();
        assert_eq!(controls.len(), 60);
        let mut ids = BTreeSet::new();
        for row in rows {
            assert_eq!(row["schema"], "pencil-cohort-target/v1");
            assert_eq!(row["p"], p);
            assert_eq!(row["bits"], bits);
            assert_eq!(row["verified"], true);
            assert_eq!(row["rho"]["verified"], true);
            assert_eq!(row["recovered"], row["planted"]);
            assert_eq!(row["rho"]["recovered"], row["planted"]);
            let control = &controls[row["workload_id"].as_str().unwrap()];
            assert_eq!(row["algorithm_seed"], control["algorithm_seed"]);
            assert_eq!(row["source_add"], control["source_add"]);
            assert_eq!(row["rho"]["counts"], control["rho"]["counts"]);
            assert_eq!(row["rho"]["native"], control["rho"]["native"]);
            assert!(ids.insert(row["workload_id"].as_str().unwrap().to_owned()));
        }
        assert_eq!(ids.len(), 60);
        let relations: u64 = rows
            .iter()
            .map(|r| r["verified_relations"].as_u64().unwrap())
            .sum();
        let updates: u64 = rows
            .iter()
            .map(|r| r["solver_row_updates"].as_u64().unwrap())
            .sum();
        let stored: u64 = rows
            .iter()
            .map(|r| r["solver_stored_nonzero_coefficients"].as_u64().unwrap())
            .sum();
        let mut representative = Value::Null;
        for square in [0.0, 0.5, 1.0] {
            for inversion in [1.0, 10.0, 100.0, 1000.0] {
                for native in [0.0, 0.1, 1.0] {
                    let pencil: f64 = rows
                        .iter()
                        .map(|r| price_pencil(r, square, inversion, native))
                        .sum();
                    let rho: f64 = rows
                        .iter()
                        .map(|r| price_rho(r, square, inversion, native))
                        .sum();
                    let ratio = pencil / rho;
                    writeln!(grid, "{bits},{p},60,60,0,{square},{inversion},{native},{pencil:.12},{rho:.12},{ratio:.12},{relations},{updates},{stored}").unwrap();
                    if square == 0.5 && inversion == 10.0 && native == 0.1 {
                        representative =
                            json!({"pencil_GAE":pencil,"rho_GAE":rho,"pencil_over_rho":ratio});
                    }
                }
            }
        }
        summaries.push(
            json!({"bits":bits,"p":p,"targets":60,"candidate_verified":60,
            "rho_verified":60,"verified_relations":relations,"solver_row_updates":updates,
            "stored_nonzero_coefficients":stored,"representative":representative,"raw_path":path}),
        );
    }
    let summary = json!({"schema":"pencil-pilot-priced/v1","pricing":"full setup charged to every one-target solve; base inverse exponentiation multiplications removed when I is charged; scalar modular arithmetic and sparse-map updates charged as native logical work","slices":summaries});
    fs::write(
        Path::new(OUT).join("summary.json"),
        serde_json::to_vec_pretty(&summary).unwrap(),
    )
    .unwrap();
    println!("{}", serde_json::to_string_pretty(&summary).unwrap());
}
