//! Native paired aggregation over the independently validated per-target grid.
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};

const ROOT: &str = "research/pencil_sparse_20261022/results";
fn target_ids(path: &str) -> BTreeSet<String> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .skip(1)
        .map(|line| {
            let v: Value = serde_json::from_str(&line.unwrap()).unwrap();
            assert_eq!(v["verified"], true);
            assert_eq!(v["rho"]["verified"], true);
            v["workload_id"].as_str().unwrap().to_owned()
        })
        .collect()
}
fn main() {
    let root = std::env::var("PENCIL_PRICE_OUT").unwrap_or_else(|_| ROOT.to_owned());
    let args: Vec<String> = std::env::args().skip(1).collect();
    assert_eq!(
        args.len(),
        3,
        "compare-sparse BASE.jsonl CANDIDATE.jsonl STEM"
    );
    let baseline = &args[0];
    let candidate = &args[1];
    let stem = &args[2];
    let targets = target_ids(baseline);
    assert_eq!(
        targets,
        target_ids(candidate),
        "workloads must match exactly"
    );
    let grid = fs::read_to_string(format!("{root}/price-grid.csv")).unwrap();
    type Key = (String, String, String, String);
    let mut control: BTreeMap<Key, (f64, f64)> = BTreeMap::new();
    let mut trial: BTreeMap<Key, (f64, f64)> = BTreeMap::new();
    let mut p = None;
    for line in grid.lines().skip(1) {
        let fields: Vec<_> = line.split(',').collect();
        assert_eq!(fields.len(), 13);
        let map = if fields[0] == baseline {
            &mut control
        } else if fields[0] == candidate {
            &mut trial
        } else {
            continue;
        };
        assert!(targets.contains(fields[5]));
        let field_p = fields[2].parse::<u64>().unwrap();
        if let Some(p) = p {
            assert_eq!(p, field_p);
        } else {
            p = Some(field_p);
        }
        let key = (
            fields[5].to_owned(),
            fields[7].to_owned(),
            fields[8].to_owned(),
            fields[9].to_owned(),
        );
        let candidate_price: f64 = fields[10].parse().unwrap();
        let rho_price: f64 = fields[11].parse().unwrap();
        assert!(map.insert(key, (candidate_price, rho_price)).is_none());
    }
    assert_eq!(control.len(), targets.len() * 36);
    assert_eq!(control.len(), trial.len());
    assert!(control.keys().eq(trial.keys()));
    let mut file = File::create(format!("{root}/{stem}.csv")).unwrap();
    writeln!(file,"p,targets,S_per_M,I_per_M,native_GAE_per_logical,baseline_GAE,candidate_GAE,rho_GAE,candidate_over_baseline,baseline_over_rho,candidate_over_rho").unwrap();
    let mut representative = Value::Null;
    let mut cells_lower = 0u64;
    let mut cells_below_rho = 0u64;
    let mut min_ratio = f64::INFINITY;
    let mut max_ratio = f64::NEG_INFINITY;
    for square in ["0", "0.5", "1"] {
        for inverse in ["1", "10", "100", "1000"] {
            for native in ["0", "0.1", "1"] {
                let mut baseline_sum = 0.0;
                let mut candidate_sum = 0.0;
                let mut rho_sum = 0.0;
                for workload in &targets {
                    let key = (
                        workload.clone(),
                        square.into(),
                        inverse.into(),
                        native.into(),
                    );
                    let (b, rho_b) = control[&key];
                    let (c, rho_c) = trial[&key];
                    assert!((rho_b - rho_c).abs() <= 1e-9 * rho_b.abs().max(1.0));
                    baseline_sum += b;
                    candidate_sum += c;
                    rho_sum += rho_b;
                }
                let ratio = candidate_sum / baseline_sum;
                cells_lower += u64::from(ratio < 1.0);
                cells_below_rho += u64::from(candidate_sum < rho_sum);
                min_ratio = min_ratio.min(ratio);
                max_ratio = max_ratio.max(ratio);
                writeln!(file,"{},{},{square},{inverse},{native},{baseline_sum:.12},{candidate_sum:.12},{rho_sum:.12},{ratio:.12},{:.12},{:.12}",
                    p.unwrap(),targets.len(),baseline_sum/rho_sum,candidate_sum/rho_sum).unwrap();
                if square == "0.5" && inverse == "10" && native == "0.1" {
                    representative = json!({"baseline_GAE":baseline_sum,"candidate_GAE":candidate_sum,
                        "rho_GAE":rho_sum,"candidate_over_baseline":ratio,
                        "baseline_over_rho":baseline_sum/rho_sum,"candidate_over_rho":candidate_sum/rho_sum});
                }
            }
        }
    }
    let summary = json!({"schema":"pencil-sparse-paired-comparison/v1",
        "p":p.unwrap(),"targets":targets.len(),"baseline_raw":baseline,"candidate_raw":candidate,
        "representative":representative,"pricing_cells":36,"cells_candidate_below_baseline":cells_lower,
        "cells_candidate_below_rho":cells_below_rho,"min_candidate_over_baseline":min_ratio,
        "max_candidate_over_baseline":max_ratio,"grid":format!("{root}/{stem}.csv")});
    fs::write(
        format!("{root}/{stem}.json"),
        serde_json::to_vec_pretty(&summary).unwrap(),
    )
    .unwrap();
    println!("{}", serde_json::to_string_pretty(&summary).unwrap());
}
