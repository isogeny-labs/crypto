//! Independent replay, source-scalar and representative-price recomputation.
use five_leads_study::three_leads::{Elliptic, Field, Point};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::process::Command;

const ROOT: &str = "research/four_followups_20261021/results/pencil-pilot";
fn rows(path: &str) -> Vec<Value> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| serde_json::from_str(&line.unwrap()).unwrap())
        .collect()
}
fn remove_wall(value: &mut Value) {
    match value {
        Value::Object(map) => {
            map.remove("elapsed_ns");
            map.remove("setup_elapsed_ns");
            map.remove("online_elapsed_ns");
            for child in map.values_mut() {
                remove_wall(child);
            }
        }
        Value::Array(array) => {
            for child in array {
                remove_wall(child);
            }
        }
        _ => {}
    }
}
fn n(value: &Value, key: &str) -> f64 {
    value[key].as_u64().unwrap() as f64
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
fn bit_count(x: u64) -> u64 {
    (64 - x.leading_zeros() + x.count_ones()) as u64
}
fn costs(row: &Value, square: f64, inversion: f64, native: f64) -> (f64, f64) {
    let f = &row["counts"];
    let a = &row["arithmetic"];
    let u = &row["source_add"];
    let p = row["p"].as_u64().unwrap();
    let denominator = n(u, "m") + square * n(u, "s") + inversion * n(u, "i");
    let m = n(f, "m") + n(a, "base_muls") - n(a, "base_inversions") * bit_count(p - 2) as f64;
    assert!(m >= 0.0);
    let candidate_native = n(f, "field_adds")
        + n(f, "bit_ops")
        + n(a, "base_adds")
        + n(a, "scalar_adds")
        + n(a, "scalar_muls")
        + n(a, "scalar_inversions")
        + n(row, "solver_row_updates");
    let candidate = (m + square * n(f, "s") + inversion * (n(f, "i") + n(a, "base_inversions")))
        / denominator
        + native * candidate_native;
    let rho = &row["rho"];
    let rc = &rho["counts"];
    let reference = (n(rc, "m") + square * n(rc, "s") + inversion * n(rc, "i")) / denominator
        + native * (logical(&rho["native"]) + n(rc, "field_adds") + n(rc, "bit_ops"));
    (candidate, reference)
}
fn archive_rows(bits: u64) -> Vec<Value> {
    let member = format!("sessions_projective_chart_v2/order-genus-{bits}/records.jsonl");
    let output = Command::new("tar")
        .args([
            "--zstd",
            "-xOf",
            "research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst",
            &member,
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}
fn validate_single(path: &str) {
    let primary = rows(path);
    assert_eq!(primary.len(), 2, "one setup and one target");
    let row = &primary[1];
    let p = row["p"].as_u64().unwrap();
    let bits = row["bits"].as_u64().unwrap();
    let replay_path = format!("{ROOT}/replay/f{p}-one-target.jsonl");
    let replay = rows(&replay_path);
    assert_eq!(replay.len(), 2);
    for (left, right) in primary.iter().zip(&replay) {
        let mut left = left.clone();
        let mut right = right.clone();
        remove_wall(&mut left);
        remove_wall(&mut right);
        assert_eq!(left, right, "single-target replay mismatch");
    }
    let control = archive_rows(bits)
        .into_iter()
        .find(|r| {
            r["arm"] == "genus_projective"
                && r["workload"]["curve_spec"]["p"].as_u64() == Some(p)
                && r["workload"]["workload_id"] == row["workload_id"]
        })
        .expect("frozen control missing");
    assert_eq!(
        row["workload_sha256"],
        control["workload"]["workload_sha256"]
    );
    assert_eq!(row["algorithm_seed"], control["algorithm_seed"]);
    let spec = &control["workload"]["curve_spec"];
    let curve = Elliptic {
        f: Field::new(p, 2, 1),
        c: [
            spec["b"].as_u64().unwrap(),
            spec["a"].as_u64().unwrap(),
            0,
            1,
        ],
        binary: false,
    };
    let generator = Point::affine(spec["gx"].as_u64().unwrap(), spec["gy"].as_u64().unwrap());
    let hex =
        |v: &Value| u64::from_str_radix(v.as_str().unwrap().trim_start_matches("0x"), 16).unwrap();
    let target = Point::affine(
        hex(&control["workload"]["target"][0]),
        hex(&control["workload"]["target"][1]),
    );
    assert_eq!(row["verified"], true);
    assert_eq!(row["rho"]["verified"], true);
    assert_eq!(row["recovered"], row["planted"]);
    assert_eq!(row["rho"]["recovered"], row["planted"]);
    for key in ["planted", "recovered"] {
        assert_eq!(curve.mul(generator, row[key].as_u64().unwrap()), target);
    }
    assert_eq!(
        curve.mul(generator, row["rho"]["recovered"].as_u64().unwrap()),
        target
    );
    let (candidate, rho) = costs(row, 0.5, 10.0, 0.1);
    let summary: Value =
        serde_json::from_slice(&fs::read(format!("{ROOT}/f{p}-one-target-summary.json")).unwrap())
            .unwrap();
    let mut headline = Vec::new();
    for (key, computed) in [
        ("pencil_GAE", candidate),
        ("rho_GAE", rho),
        ("pencil_over_rho", candidate / rho),
    ] {
        let claimed = summary["representative"][key].as_f64().unwrap();
        assert!(
            (claimed - computed).abs() <= 1e-10 * claimed.abs().max(1.0),
            "price {key}"
        );
        headline.push(json!({"name":key,"claimed":claimed,"recomputed":computed,"rel_tol":1e-10}));
    }
    let grid = fs::read_to_string(format!("{ROOT}/f{p}-one-target-price-grid.csv")).unwrap();
    let mut seen = BTreeSet::new();
    let mut min_ratio = f64::INFINITY;
    let mut max_ratio = f64::NEG_INFINITY;
    let mut below = 0u64;
    for line in grid.lines().skip(1) {
        let fields: Vec<&str> = line.split(',').collect();
        assert_eq!(fields.len(), 9);
        assert_eq!(fields[0].parse::<u64>().unwrap(), bits);
        assert_eq!(fields[1].parse::<u64>().unwrap(), p);
        assert_eq!(fields[2].parse::<u64>().unwrap(), 0);
        let square: f64 = fields[3].parse().unwrap();
        let inversion: f64 = fields[4].parse().unwrap();
        let native: f64 = fields[5].parse().unwrap();
        assert!([0.0, 0.5, 1.0].contains(&square));
        assert!([1.0, 10.0, 100.0, 1000.0].contains(&inversion));
        assert!([0.0, 0.1, 1.0].contains(&native));
        assert!(seen.insert(format!("{square},{inversion},{native}")));
        let (candidate, reference) = costs(row, square, inversion, native);
        let ratio = candidate / reference;
        for (actual, recorded) in [
            (candidate, fields[6].parse::<f64>().unwrap()),
            (reference, fields[7].parse::<f64>().unwrap()),
            (ratio, fields[8].parse::<f64>().unwrap()),
        ] {
            assert!((actual - recorded).abs() <= 1e-10 * actual.abs().max(1.0));
        }
        min_ratio = min_ratio.min(ratio);
        max_ratio = max_ratio.max(ratio);
        below += u64::from(ratio < 1.0);
    }
    assert_eq!(seen.len(), 36);
    assert_eq!(summary["pricing_cell_count"], 36);
    assert_eq!(summary["price_cells_below_rho"], below);
    for (key, actual) in [
        ("min_price_cell_ratio", min_ratio),
        ("max_price_cell_ratio", max_ratio),
    ] {
        let recorded = summary[key].as_f64().unwrap();
        assert!((actual - recorded).abs() <= 1e-10 * actual.abs().max(1.0));
        headline.push(json!({"name":key,"claimed":recorded,"recomputed":actual,"rel_tol":1e-10}));
    }
    let validator = json!({"schema":"pencil-single-independent-validator/v1","status":"pass",
        "independent":true,"source_scalars_checked":2,"headline":headline,
        "price_grid_cells_recomputed":36,"price_grid_claims_checked":108,
        "price_cells_below_rho":below,
        "raw_data":[{"path":format!("../../{path}")}]});
    fs::write(
        format!("{ROOT}/f{p}-one-target-validator-report.json"),
        serde_json::to_vec_pretty(&validator).unwrap(),
    )
    .unwrap();
    let cert = json!({"schema":"pencil-single-replay-certificate/v1","status":"verified",
        "independent":true,"scalars_verified":true,"candidate_scalars_checked":1,
        "rho_scalars_checked":1,"primary":path,"replay":replay_path,
        "method":"fresh archive-input process; every non-wall JSON field compared exactly; independent source multiplication"});
    fs::write(
        format!("{ROOT}/f{p}-one-target-replay-certificate.json"),
        serde_json::to_vec_pretty(&cert).unwrap(),
    )
    .unwrap();
    println!("validated single target p={p}");
}
fn main() {
    let paths: Vec<String> = std::env::args().skip(1).collect();
    if paths.first().map(String::as_str) == Some("--single") {
        assert_eq!(
            paths.len(),
            2,
            "usage: validate-pencil --single PRIMARY.jsonl"
        );
        validate_single(&paths[1]);
        return;
    }
    assert!(
        !paths.is_empty(),
        "usage: validate-pencil PRIMARY.jsonl ..."
    );
    let summary: Value =
        serde_json::from_slice(&fs::read(format!("{ROOT}/summary.json")).unwrap()).unwrap();
    let grid = fs::read_to_string(format!("{ROOT}/price-grid.csv")).unwrap();
    let grid: BTreeMap<String, (f64, f64, f64)> = grid
        .lines()
        .skip(1)
        .map(|line| {
            let fields: Vec<&str> = line.split(',').collect();
            assert_eq!(fields.len(), 14);
            let key = format!("{},{},{},{}", fields[1], fields[5], fields[6], fields[7]);
            let prices = (
                fields[8].parse().unwrap(),
                fields[9].parse().unwrap(),
                fields[10].parse().unwrap(),
            );
            (key, prices)
        })
        .collect();
    let mut heads = Vec::new();
    let mut data = vec![
        json!({"path":"../../research/four_followups_20261021/results/pencil-pilot/summary.json"}),
    ];
    let mut runs = Vec::new();
    let mut verified = 0;
    let mut grid_checked = 0usize;
    let mut overall_below = 0u64;
    let mut overall_min = f64::INFINITY;
    let mut overall_max = f64::NEG_INFINITY;
    for path in paths {
        let primary = rows(&path);
        let replay = format!("{ROOT}/replay/{}", path.rsplit('/').next().unwrap());
        let second = rows(&replay);
        assert_eq!(primary.len(), 61);
        assert_eq!(second.len(), 61);
        for (left, right) in primary.iter().zip(&second) {
            let mut left = left.clone();
            let mut right = right.clone();
            remove_wall(&mut left);
            remove_wall(&mut right);
            assert_eq!(left, right, "replay mismatch {path}");
        }
        let p = primary[0]["p"].as_u64().unwrap();
        let bits = primary[0]["bits"].as_u64().unwrap();
        let controls: BTreeMap<String, Value> = archive_rows(bits)
            .into_iter()
            .filter(|row| {
                row["arm"] == "genus_projective"
                    && row["workload"]["curve_spec"]["p"].as_u64() == Some(p)
            })
            .map(|row| {
                (
                    row["workload"]["workload_id"].as_str().unwrap().to_owned(),
                    row,
                )
            })
            .collect();
        assert_eq!(controls.len(), 60);
        let mut candidate = 0.0;
        let mut reference = 0.0;
        for row in &primary[1..] {
            let control = &controls[row["workload_id"].as_str().unwrap()];
            assert_eq!(
                row["workload_sha256"],
                control["workload"]["workload_sha256"]
            );
            assert_eq!(row["algorithm_seed"], control["algorithm_seed"]);
            assert_eq!(row["recovered"], row["planted"]);
            assert_eq!(row["rho"]["recovered"], row["planted"]);
            let spec = &control["workload"]["curve_spec"];
            let curve = Elliptic {
                f: Field::new(p, 2, 1),
                c: [
                    spec["b"].as_u64().unwrap(),
                    spec["a"].as_u64().unwrap(),
                    0,
                    1,
                ],
                binary: false,
            };
            let generator =
                Point::affine(spec["gx"].as_u64().unwrap(), spec["gy"].as_u64().unwrap());
            let target = &control["workload"]["target"];
            let hex = |v: &Value| {
                u64::from_str_radix(v.as_str().unwrap().trim_start_matches("0x"), 16).unwrap()
            };
            let target = Point::affine(hex(&target[0]), hex(&target[1]));
            assert_eq!(
                curve.mul(generator, row["planted"].as_u64().unwrap()),
                target
            );
            assert_eq!(
                curve.mul(generator, row["recovered"].as_u64().unwrap()),
                target
            );
            assert_eq!(
                curve.mul(generator, row["rho"]["recovered"].as_u64().unwrap()),
                target
            );
            let (a, b) = costs(row, 0.5, 10.0, 0.1);
            candidate += a;
            reference += b;
            verified += 1;
        }
        let claimed = summary["slices"]
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["p"] == p)
            .expect("missing summary slice");
        for (name, computed) in [
            ("pencil_GAE", candidate),
            ("rho_GAE", reference),
            ("pencil_over_rho", candidate / reference),
        ] {
            heads.push(json!({"name":format!("p{p}_{name}"),
                "claimed":claimed["representative"][name].as_f64().unwrap(),
                "recomputed":computed,"rel_tol":1e-10}));
        }
        let mut min_ratio = f64::INFINITY;
        let mut max_ratio = f64::NEG_INFINITY;
        let mut below = 0u64;
        for square in [0.0, 0.5, 1.0] {
            for inversion in [1.0, 10.0, 100.0, 1000.0] {
                for native in [0.0, 0.1, 1.0] {
                    let (candidate, rho) = primary[1..]
                        .iter()
                        .map(|row| costs(row, square, inversion, native))
                        .fold((0.0, 0.0), |(a, b), (c, d)| (a + c, b + d));
                    let ratio = candidate / rho;
                    let key = format!("{p},{square},{inversion},{native}");
                    let (reported_candidate, reported_rho, reported_ratio) = grid[&key];
                    for (actual, reported) in [
                        (candidate, reported_candidate),
                        (rho, reported_rho),
                        (ratio, reported_ratio),
                    ] {
                        assert!((actual - reported).abs() <= 1e-10 * actual.abs().max(1.0));
                    }
                    min_ratio = min_ratio.min(ratio);
                    max_ratio = max_ratio.max(ratio);
                    below += u64::from(ratio < 1.0);
                    grid_checked += 1;
                }
            }
        }
        assert_eq!(claimed["pricing_cell_count"], 36);
        assert_eq!(claimed["price_cells_below_rho"], below);
        for (key, actual) in [
            ("min_price_cell_ratio", min_ratio),
            ("max_price_cell_ratio", max_ratio),
        ] {
            let recorded = claimed[key].as_f64().unwrap();
            assert!((actual - recorded).abs() <= 1e-10 * actual.abs().max(1.0));
        }
        overall_below += below;
        overall_min = overall_min.min(min_ratio);
        overall_max = overall_max.max(max_ratio);
        data.push(json!({"path":format!("../../{path}")}));
        runs.push(json!({"primary":path,"replay":replay,"rows":61,"scalars_verified":60}));
    }
    assert_eq!(grid_checked, grid.len());
    assert_eq!(
        summary["price_grid_summary"]["pricing_cell_count"],
        grid_checked
    );
    assert_eq!(
        summary["price_grid_summary"]["price_cells_below_rho"],
        overall_below
    );
    for (key, actual) in [
        ("min_price_cell_ratio", overall_min),
        ("max_price_cell_ratio", overall_max),
    ] {
        let recorded = summary["price_grid_summary"][key].as_f64().unwrap();
        assert!((actual - recorded).abs() <= 1e-10 * actual.abs().max(1.0));
        heads.push(json!({"name":key,"claimed":recorded,"recomputed":actual,"rel_tol":1e-10}));
    }
    let validator = json!({"schema":"pencil-pilot-independent-validator/v1","status":"pass",
        "independent":true,"method":"separate native raw-row arithmetic and archived source multiplication",
        "raw_data":data,"headline":heads,"source_scalars_checked":verified,
        "price_grid_cells_recomputed":grid_checked,"price_grid_claims_checked":grid_checked*3,
        "price_cells_below_rho":overall_below});
    fs::write(
        format!("{ROOT}/validator-report.json"),
        serde_json::to_vec_pretty(&validator).unwrap(),
    )
    .unwrap();
    let certificate = json!({"schema":"pencil-pilot-replay-certificate/v1","status":"verified",
        "independent":true,"scalars_verified":true,"method":"fresh archive-input process; all non-wall JSON fields compared exactly; independent source multiplication",
        "candidate_scalars_checked":verified,"rho_scalars_checked":verified,"runs":runs});
    fs::write(
        format!("{ROOT}/replay-certificate.json"),
        serde_json::to_vec_pretty(&certificate).unwrap(),
    )
    .unwrap();
    println!("validated {verified} candidate and {verified} rho scalars");
}
