//! Native 36-cell price export from exact raw pencil/rho records.
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};

const OUT: &str = "research/pencil_sparse_20261022/results";

fn rows(path: &str) -> Vec<Value> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| serde_json::from_str(&line.unwrap()).unwrap())
        .collect()
}
fn n(value: &Value, key: &str) -> f64 {
    value[key]
        .as_u64()
        .unwrap_or_else(|| panic!("missing {key}")) as f64
}
fn bit_count(x: u64) -> u64 {
    (64 - x.leading_zeros() + x.count_ones()) as u64
}
fn logical(value: &Value) -> f64 {
    value
        .as_object()
        .unwrap()
        .iter()
        .filter(|(key, _)| key.ends_with("_uncharged"))
        .map(|(_, x)| x.as_u64().unwrap() as f64)
        .sum()
}
fn gcd_steps(mut a: usize, mut b: usize) -> u64 {
    let mut steps = 0;
    while b != 0 {
        (a, b) = (b, a % b);
        steps += 1;
    }
    steps
}
fn order_extra_ops(row: &Value) -> u64 {
    if row["solver_node_order"] != "affine" {
        return 0;
    }
    let size = row["solver_nodes"].as_u64().unwrap() as usize;
    let mut stride = (0x9e37_79b9_7f4a_7c15_u64 % size as u64) as usize;
    let mut ops = 2; // initial stride and shift reductions
    loop {
        ops += gcd_steps(stride, size);
        if gcd_value(stride, size) == 1 {
            break;
        }
        stride = (stride + 1) % size;
        ops += 2;
    }
    ops
}
fn gcd_value(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
fn pencil_price(row: &Value, square: f64, inversion: f64, native: f64) -> f64 {
    let f = &row["counts"];
    let a = &row["arithmetic"];
    let unit = &row["source_add"];
    let p = row["p"].as_u64().unwrap();
    let base_muls = n(a, "base_muls") - n(a, "base_inversions") * bit_count(p - 2) as f64;
    assert!(base_muls >= 0.0);
    let denominator = n(unit, "m") + square * n(unit, "s") + inversion * n(unit, "i");
    let field = (n(f, "m")
        + base_muls
        + square * n(f, "s")
        + inversion * (n(f, "i") + n(a, "base_inversions")))
        / denominator;
    let native_ops = n(f, "field_adds")
        + n(f, "bit_ops")
        + n(a, "base_adds")
        + n(a, "scalar_adds")
        + n(a, "scalar_muls")
        + n(a, "scalar_inversions")
        + n(row, "solver_row_updates")
        + row["solver_index_ops"].as_u64().unwrap_or(0) as f64
        + row["solver_node_order_ops"].as_u64().unwrap_or(0) as f64
        + order_extra_ops(row) as f64;
    field + native * native_ops
}
fn rho_price(row: &Value, square: f64, inversion: f64, native: f64) -> f64 {
    let f = &row["rho"]["counts"];
    let unit = &row["source_add"];
    let denominator = n(unit, "m") + square * n(unit, "s") + inversion * n(unit, "i");
    let field = (n(f, "m") + square * n(f, "s") + inversion * n(f, "i")) / denominator;
    field + native * (n(f, "field_adds") + n(f, "bit_ops") + logical(&row["rho"]["native"]))
}
fn verified(row: &Value) -> bool {
    row["verified"] == true
        && row["rho"]["verified"] == true
        && row["recovered"] == row["planted"]
        && row["rho"]["recovered"] == row["planted"]
        && row["failure"].is_null()
}
fn main() {
    let inputs: Vec<String> = std::env::args().skip(1).collect();
    assert!(!inputs.is_empty(), "usage: price-sparse RAW.jsonl ...");
    let out = std::env::var("PENCIL_PRICE_OUT").unwrap_or_else(|_| OUT.to_owned());
    fs::create_dir_all(&out).unwrap();
    let mut grid = File::create(format!("{out}/price-grid.csv")).unwrap();
    let mut targets = File::create(format!("{out}/targets.jsonl")).unwrap();
    writeln!(grid, "raw_path,bits,p,order,target_index,workload_id,status,S_per_M,I_per_M,native_GAE_per_logical,pencil_GAE,rho_GAE,pencil_over_rho").unwrap();
    let mut summaries = Vec::new();
    let mut total_targets = 0;
    let mut total_verified = 0;
    let mut total_cells = 0;
    let mut total_below_rho = 0u64;
    for path in &inputs {
        let input = rows(path);
        assert!(input.len() >= 2);
        assert_eq!(input[0]["schema"], "pencil-cohort-setup/v1");
        let bits = input[0]["bits"].as_u64().unwrap();
        let p = input[0]["p"].as_u64().unwrap();
        let mut seen = BTreeSet::new();
        let mut order = String::new();
        let mut mode = String::new();
        let mut verified_count = 0;
        let mut row_updates = 0u64;
        let mut index_ops = 0u64;
        let mut order_ops = 0u64;
        let mut representative_pencil = 0.0;
        let mut representative_rho = 0.0;
        let mut below_rho = 0u64;
        let mut min_ratio = f64::INFINITY;
        let mut max_ratio = f64::NEG_INFINITY;
        for row in input.iter().skip(1) {
            assert_eq!(row["schema"], "pencil-cohort-target/v1");
            assert_eq!(row["p"], p);
            assert_eq!(row["bits"], bits);
            let current_order = row["solver_node_order"].as_str().unwrap_or("identity");
            let current_mode = row["solver_mode"].as_str().unwrap_or("online");
            if order.is_empty() {
                order = current_order.to_string();
                mode = current_mode.to_string();
            }
            assert_eq!(order, current_order);
            assert_eq!(mode, current_mode);
            let workload = row["workload_id"].as_str().unwrap();
            assert!(
                seen.insert(workload.to_owned()),
                "duplicate workload {workload}"
            );
            let target = row["target_index"].as_u64().unwrap();
            let ok = verified(row);
            let status = if ok { "verified" } else { "incomplete" };
            verified_count += usize::from(ok);
            row_updates += row["solver_row_updates"].as_u64().unwrap();
            index_ops += row["solver_index_ops"].as_u64().unwrap_or(0);
            order_ops += row["solver_node_order_ops"].as_u64().unwrap_or(0) + order_extra_ops(row);
            let mut representative = Value::Null;
            for square in [0.0, 0.5, 1.0] {
                for inversion in [1.0, 10.0, 100.0, 1000.0] {
                    for native in [0.0, 0.1, 1.0] {
                        let reference = if row["rho"]["verified"] == true {
                            Some(rho_price(row, square, inversion, native))
                        } else {
                            None
                        };
                        let candidate = ok.then(|| pencil_price(row, square, inversion, native));
                        let ratio = candidate.zip(reference).map(|(c, r)| c / r);
                        if let Some(q) = ratio {
                            below_rho += u64::from(q < 1.0);
                            min_ratio = min_ratio.min(q);
                            max_ratio = max_ratio.max(q);
                        }
                        writeln!(grid, "{path},{bits},{p},{order},{target},{workload},{status},{square},{inversion},{native},{},{},{}",
                            candidate.map(|x|format!("{x:.12}")).unwrap_or_default(),
                            reference.map(|x|format!("{x:.12}")).unwrap_or_default(),
                            ratio.map(|x|format!("{x:.12}")).unwrap_or_default()).unwrap();
                        total_cells += 1;
                        if square == 0.5 && inversion == 10.0 && native == 0.1 {
                            if let (Some(c), Some(r), Some(q)) = (candidate, reference, ratio) {
                                representative_pencil += c;
                                representative_rho += r;
                                representative =
                                    json!({"pencil_GAE":c,"rho_GAE":r,"pencil_over_rho":q});
                            }
                        }
                    }
                }
            }
            writeln!(targets,"{}",json!({"schema":"pencil-sparse-target/v1","raw_path":path,
                "bits":bits,"curve_p":p,"order":order,"solver_mode":mode,"target_index":target,
                "workload_id":workload,"workload_sha256":row["workload_sha256"],
                "algorithm_seed":row["algorithm_seed"],"planted":row["planted"],
                "recovered":row["recovered"],"candidate_verified":row["verified"],
                "rho_recovered":row["rho"]["recovered"],"rho_verified":row["rho"]["verified"],
                "failure":row["failure"],"status":status,"solver_row_updates":row["solver_row_updates"],
                "solver_index_ops":row["solver_index_ops"],
                "solver_node_order_ops":row["solver_node_order_ops"].as_u64().unwrap_or(0),
                "representative":representative})).unwrap();
        }
        total_targets += seen.len();
        total_verified += verified_count;
        total_below_rho += below_rho;
        summaries.push(json!({"raw_path":path,"bits":bits,"p":p,"order":order,"solver_mode":mode,
            "targets":seen.len(),"verified":verified_count,"failures":seen.len()-verified_count,
            "solver_row_updates":row_updates,"solver_index_ops":index_ops,
            "solver_node_order_ops":order_ops,
            "price_cells_below_rho":below_rho,"min_price_cell_ratio":min_ratio,
            "max_price_cell_ratio":max_ratio,
            "representative":if verified_count == seen.len(){
                json!({"pencil_GAE":representative_pencil,"rho_GAE":representative_rho,
                    "pencil_over_rho":representative_pencil/representative_rho})} else {Value::Null}}));
    }
    let summary = json!({"schema":"pencil-sparse-price-summary/v1","status":if total_verified==total_targets{"screen_complete"}else{"partial_with_failures"},
        "requested_cohort_targets_per_order":1260,"raw_files":inputs.len(),
        "target_rows":total_targets,"source_verified_pairs":total_verified,
        "pricing_cells":total_cells,"price_cells_below_rho":total_below_rho,"slices":summaries,
        "pricing":"cold setup included in each raw target; variable-order logical work added; wall time excluded"});
    fs::write(
        format!("{out}/summary.json"),
        serde_json::to_vec_pretty(&summary).unwrap(),
    )
    .unwrap();
    println!("{}", serde_json::to_string_pretty(&summary).unwrap());
}
