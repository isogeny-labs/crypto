//! Independent source-scalar, non-wall replay, and all-cell price validation.
use five_leads_study::three_leads::{Elliptic, Field, Point};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::process::Command;

const ROOT: &str = "research/pencil_sparse_20261022/results";
fn rows(path: &str) -> Vec<Value> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| serde_json::from_str(&line.unwrap()).unwrap())
        .collect()
}
fn trim_wall(v: &mut Value) {
    match v {
        Value::Object(map) => {
            map.remove("elapsed_ns");
            map.remove("online_elapsed_ns");
            map.remove("setup_elapsed_ns");
            // The pre-peel online binary omitted these zero/default metadata
            // fields; the later replay binary emits them explicitly.
            if map.get("schema").and_then(Value::as_str) == Some("pencil-cohort-target/v1") {
                map.entry("solver_mode".to_owned())
                    .or_insert_with(|| json!("online"));
                map.entry("solver_attempts".to_owned())
                    .or_insert_with(|| json!(0));
                map.entry("solver_index_ops".to_owned())
                    .or_insert_with(|| json!(0));
            }
            for child in map.values_mut() {
                trim_wall(child);
            }
        }
        Value::Array(items) => {
            for child in items {
                trim_wall(child);
            }
        }
        _ => {}
    }
}
fn n(v: &Value, k: &str) -> f64 {
    v[k].as_u64().unwrap() as f64
}
fn logical(v: &Value) -> f64 {
    v.as_object()
        .unwrap()
        .iter()
        .filter(|(k, _)| k.ends_with("_uncharged"))
        .map(|(_, x)| x.as_u64().unwrap() as f64)
        .sum()
}
fn order_work(row: &Value) -> u64 {
    let count = row["solver_nodes"].as_u64().unwrap();
    let recorded = row["solver_node_order_ops"].as_u64().unwrap_or(0);
    match row["solver_node_order"].as_str().unwrap_or("identity") {
        "identity" => {
            assert_eq!(recorded, 0);
            0
        }
        "reverse" => {
            assert_eq!(recorded, 2 * count);
            recorded
        }
        "affine" => {
            assert_eq!(recorded, 3 * count);
            let mut stride = 0x9e37_79b9_7f4a_7c15_u64 % count;
            let mut overhead = 2u64;
            loop {
                let (mut a, mut b) = (stride, count);
                while b > 0 {
                    let remainder = a % b;
                    a = b;
                    b = remainder;
                    overhead += 1;
                }
                if a == 1 {
                    break;
                }
                stride = (stride + 1) % count;
                overhead += 2;
            }
            recorded + overhead
        }
        _ => panic!("unknown order"),
    }
}
fn costs(row: &Value, square: f64, inversion: f64, native: f64) -> (f64, f64) {
    let unit = &row["source_add"];
    let unit_cost = n(unit, "m") + square * n(unit, "s") + inversion * n(unit, "i");
    assert!(unit_cost > 0.0);
    let f = &row["counts"];
    let a = &row["arithmetic"];
    let p = row["p"].as_u64().unwrap();
    let inverse_muls = n(a, "base_inversions")
        * (64 - (p - 2).leading_zeros() as u64 + (p - 2).count_ones() as u64) as f64;
    let numerator = n(f, "m") + n(a, "base_muls") - inverse_muls
        + square * n(f, "s")
        + inversion * (n(f, "i") + n(a, "base_inversions"));
    assert!(numerator >= 0.0);
    let logical_candidate = n(f, "field_adds")
        + n(f, "bit_ops")
        + n(a, "base_adds")
        + n(a, "scalar_adds")
        + n(a, "scalar_muls")
        + n(a, "scalar_inversions")
        + n(row, "solver_row_updates")
        + row["solver_index_ops"].as_u64().unwrap_or(0) as f64
        + order_work(row) as f64;
    let candidate = numerator / unit_cost + native * logical_candidate;
    let rf = &row["rho"]["counts"];
    let reference = (n(rf, "m") + square * n(rf, "s") + inversion * n(rf, "i")) / unit_cost
        + native * (n(rf, "field_adds") + n(rf, "bit_ops") + logical(&row["rho"]["native"]));
    (candidate, reference)
}
fn archive(bits: u64) -> BTreeMap<String, Value> {
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
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .filter(|v| v["arm"] == "genus_projective")
        .map(|v| (v["workload"]["workload_id"].as_str().unwrap().to_owned(), v))
        .collect()
}
fn main() {
    let root = std::env::var("PENCIL_PRICE_OUT").unwrap_or_else(|_| ROOT.to_owned());
    let paths: Vec<String> = std::env::args().skip(1).collect();
    assert!(!paths.is_empty(), "usage: validate-sparse RAW.jsonl ...");
    let mut raw_by_key = BTreeMap::new();
    let mut cached_archive = BTreeMap::new();
    let mut cached_rho = BTreeMap::new();
    let mut scalars_checked = 0u64;
    let mut replay_rows_checked = 0u64;
    let mut missing_replays = Vec::new();
    let mut files = Vec::new();
    for path in &paths {
        let primary = rows(path);
        let setup = &primary[0];
        assert_eq!(setup["schema"], "pencil-cohort-setup/v1");
        let bits = setup["bits"].as_u64().unwrap();
        let p = setup["p"].as_u64().unwrap();
        cached_archive.entry(bits).or_insert_with(|| archive(bits));
        cached_rho.entry(bits).or_insert_with(|| {
            let path = format!(
                "research/four_followups_20261021/results/matched/fb1-j16-bits{bits}.jsonl"
            );
            rows(&path)
                .into_iter()
                .map(|v| (v["workload_id"].as_str().unwrap().to_owned(), v))
                .collect::<BTreeMap<String, Value>>()
        });
        let source = &cached_archive[&bits];
        let rho_controls = &cached_rho[&bits];
        let replay_path = if path.contains("/results/pencil-pilot/") {
            path.replace("/results/pencil-pilot/", "/results/pencil-pilot/replay/")
        } else {
            path.replace("/results/", "/results/replay/")
        };
        let replay_ready = if fs::metadata(&replay_path).is_ok() {
            let replay = rows(&replay_path);
            assert!(replay.len() <= primary.len(), "replay too long {path}");
            if replay.len() == primary.len() {
                for (left, right) in primary.iter().zip(&replay) {
                    let mut left = left.clone();
                    let mut right = right.clone();
                    trim_wall(&mut left);
                    trim_wall(&mut right);
                    assert_eq!(left, right, "non-wall replay {path}");
                    replay_rows_checked += 1;
                }
                true
            } else {
                missing_replays.push(path.clone());
                false
            }
        } else {
            missing_replays.push(path.clone());
            false
        };
        let mut workload_ids = BTreeSet::new();
        for row in primary.iter().skip(1) {
            assert_eq!(row["p"], p);
            assert_eq!(row["bits"], bits);
            let matrix_cap = row["matrix_coefficient_cap"].as_u64().unwrap_or(10_000_000);
            assert_eq!(
                matrix_cap,
                setup["matrix_coefficient_cap"]
                    .as_u64()
                    .unwrap_or(10_000_000)
            );
            if row["failure"] == "sparse_matrix_cap" {
                assert_eq!(row["verified"], false);
                assert!(row["recovered"].is_null());
                assert!(row["solver_stored_nonzero_coefficients"].as_u64().unwrap() > matrix_cap);
            }
            let workload_id = row["workload_id"].as_str().unwrap();
            assert!(workload_ids.insert(workload_id.to_owned()));
            let control = &source[workload_id];
            assert_eq!(
                row["workload_sha256"],
                control["workload"]["workload_sha256"]
            );
            assert_eq!(row["algorithm_seed"], control["algorithm_seed"]);
            let rho_control = &rho_controls[workload_id];
            assert_eq!(row["algorithm_seed"], rho_control["algorithm_seed"]);
            assert_eq!(row["source_add"], rho_control["source_add"]);
            assert_eq!(row["rho"]["counts"], rho_control["rho"]["counts"]);
            assert_eq!(row["rho"]["native"], rho_control["rho"]["native"]);
            let spec = &control["workload"]["curve_spec"];
            assert_eq!(spec["p"], p);
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
            let hex = |v: &Value| {
                u64::from_str_radix(v.as_str().unwrap().trim_start_matches("0x"), 16).unwrap()
            };
            let target = Point::affine(
                hex(&control["workload"]["target"][0]),
                hex(&control["workload"]["target"][1]),
            );
            assert!(curve.valid(target));
            assert_eq!(curve.mul(generator, spec["r"].as_u64().unwrap()), Point::O);
            assert_eq!(
                curve.mul(generator, row["planted"].as_u64().unwrap()),
                target
            );
            for (key, is_verified) in [
                ("recovered", row["verified"] == true),
                ("rho", row["rho"]["verified"] == true),
            ] {
                let k = if key == "rho" {
                    &row["rho"]["recovered"]
                } else {
                    &row["recovered"]
                };
                if is_verified {
                    assert_eq!(curve.mul(generator, k.as_u64().unwrap()), target);
                    assert_eq!(*k, row["planted"]);
                    scalars_checked += 1;
                }
            }
            let order = row["solver_node_order"].as_str().unwrap_or("identity");
            let target_index = row["target_index"].as_u64().unwrap();
            assert!(raw_by_key
                .insert((path.clone(), target_index), row.clone())
                .is_none());
            assert!(["identity", "reverse", "affine"].contains(&order));
            let _ = order_work(row);
        }
        let verified_candidate_rows = primary
            .iter()
            .skip(1)
            .filter(|row| row["verified"] == true)
            .count();
        let failure_codes: BTreeSet<String> = primary
            .iter()
            .skip(1)
            .filter_map(|row| row["failure"].as_str().map(str::to_owned))
            .collect();
        let mode = primary
            .get(1)
            .and_then(|row| row["solver_mode"].as_str())
            .unwrap_or("online");
        files.push(json!({
            "path":path,"bits":bits,"p":p,"targets":primary.len()-1,
            "solver_mode":mode,"verified_candidate_rows":verified_candidate_rows,
            "failed_candidate_rows":primary.len()-1-verified_candidate_rows,
            "failure_codes":failure_codes,
            "replay":if replay_ready {json!(replay_path)}else{Value::Null},
            "replay_rows":if replay_ready {json!(primary.len()-1)}else{Value::Null},
            "replay_equivalence":if replay_ready && bits==28 && mode=="online" {
                json!("field_equivalent_metadata_defaults")
            }else if replay_ready {json!("nonwall_equal")}else{Value::Null}
        }));
    }
    let grid = fs::read_to_string(format!("{root}/price-grid.csv")).unwrap();
    let mut cells = 0u64;
    let mut valid_cells = 0u64;
    let mut missing_cells = 0u64;
    let mut below_rho = 0u64;
    let mut slice_bounds: BTreeMap<String, (u64, f64, f64)> = BTreeMap::new();
    for line in grid.lines().skip(1) {
        let fields: Vec<_> = line.split(',').collect();
        assert_eq!(fields.len(), 13);
        let path = fields[0].to_string();
        let slice_path = path.clone();
        let target_index: u64 = fields[4].parse().unwrap();
        let row = &raw_by_key[&(path, target_index)];
        assert_eq!(
            fields[1].parse::<u64>().unwrap(),
            row["bits"].as_u64().unwrap()
        );
        assert_eq!(
            fields[2].parse::<u64>().unwrap(),
            row["p"].as_u64().unwrap()
        );
        assert_eq!(
            fields[3],
            row["solver_node_order"].as_str().unwrap_or("identity")
        );
        assert_eq!(fields[5], row["workload_id"].as_str().unwrap());
        let square: f64 = fields[7].parse().unwrap();
        let inversion: f64 = fields[8].parse().unwrap();
        let native: f64 = fields[9].parse().unwrap();
        assert!([0.0, 0.5, 1.0].contains(&square));
        assert!([1.0, 10.0, 100.0, 1000.0].contains(&inversion));
        assert!([0.0, 0.1, 1.0].contains(&native));
        let (candidate, reference) = costs(row, square, inversion, native);
        if row["verified"] == true && row["rho"]["verified"] == true && row["failure"].is_null() {
            for (measured, text) in [
                (candidate, fields[10]),
                (reference, fields[11]),
                (candidate / reference, fields[12]),
            ] {
                let recorded: f64 = text.parse().unwrap();
                assert!(
                    (measured - recorded).abs() <= 1e-10 * measured.abs().max(1.0),
                    "price {line}"
                );
            }
            valid_cells += 1;
            let ratio = candidate / reference;
            below_rho += u64::from(ratio < 1.0);
            let bound =
                slice_bounds
                    .entry(slice_path)
                    .or_insert((0, f64::INFINITY, f64::NEG_INFINITY));
            bound.0 += u64::from(ratio < 1.0);
            bound.1 = bound.1.min(ratio);
            bound.2 = bound.2.max(ratio);
        } else {
            assert!(fields[10].is_empty() && fields[12].is_empty());
            if row["rho"]["verified"] == true {
                let recorded_rho: f64 = fields[11].parse().unwrap();
                assert!((reference - recorded_rho).abs() <= 1e-10 * reference.abs().max(1.0));
            }
            missing_cells += 1;
        }
        cells += 1;
    }
    assert_eq!(cells, raw_by_key.len() as u64 * 36);
    let summary: Value =
        serde_json::from_slice(&fs::read(format!("{root}/summary.json")).unwrap()).unwrap();
    assert_eq!(
        summary["price_cells_below_rho"].as_u64().unwrap(),
        below_rho
    );
    for slice in summary["slices"].as_array().unwrap() {
        if let Some(&(below, min, max)) = slice_bounds.get(slice["raw_path"].as_str().unwrap()) {
            assert_eq!(slice["price_cells_below_rho"].as_u64().unwrap(), below);
            for (recorded, computed) in [
                (slice["min_price_cell_ratio"].as_f64().unwrap(), min),
                (slice["max_price_cell_ratio"].as_f64().unwrap(), max),
            ] {
                assert!((recorded - computed).abs() <= 1e-10 * computed.abs().max(1.0));
            }
        } else {
            assert_eq!(slice["verified"], 0);
            assert_eq!(slice["price_cells_below_rho"], 0);
            assert!(slice["min_price_cell_ratio"].is_null());
            assert!(slice["max_price_cell_ratio"].is_null());
        }
    }
    let mut paired_cells_checked = 0u64;
    let mut headline = Vec::new();
    for entry in fs::read_dir(&root).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy();
        if !name.contains("-vs-") || !name.ends_with(".json") {
            continue;
        }
        let pair: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        assert_eq!(pair["schema"], "pencil-sparse-paired-comparison/v1");
        let baseline = pair["baseline_raw"].as_str().unwrap();
        let candidate = pair["candidate_raw"].as_str().unwrap();
        let to_map = |path: &str| {
            rows(path)
                .into_iter()
                .skip(1)
                .map(|row| (row["workload_id"].as_str().unwrap().to_owned(), row))
                .collect::<BTreeMap<String, Value>>()
        };
        let baseline_rows = to_map(baseline);
        let candidate_rows = to_map(candidate);
        assert!(baseline_rows.keys().eq(candidate_rows.keys()));
        assert_eq!(
            baseline_rows.len(),
            pair["targets"].as_u64().unwrap() as usize
        );
        let csv = fs::read_to_string(pair["grid"].as_str().unwrap()).unwrap();
        let lines: Vec<_> = csv.lines().skip(1).collect();
        assert_eq!(lines.len(), 36);
        let mut lower = 0u64;
        let mut below_rho_pair = 0u64;
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        let mut representative_ratios = None;
        for line in lines {
            let fields: Vec<_> = line.split(',').collect();
            assert_eq!(fields.len(), 11);
            let square: f64 = fields[2].parse().unwrap();
            let inverse: f64 = fields[3].parse().unwrap();
            let native: f64 = fields[4].parse().unwrap();
            let mut bsum = 0.0;
            let mut csum = 0.0;
            let mut rsum = 0.0;
            for (workload, brow) in &baseline_rows {
                let (b, r) = costs(brow, square, inverse, native);
                let (c, cr) = costs(&candidate_rows[workload], square, inverse, native);
                assert!((r - cr).abs() <= 1e-10 * r.abs().max(1.0));
                bsum += b;
                csum += c;
                rsum += r;
            }
            let ratio = csum / bsum;
            for (expected, field) in [
                (bsum, fields[5]),
                (csum, fields[6]),
                (rsum, fields[7]),
                (ratio, fields[8]),
                (bsum / rsum, fields[9]),
                (csum / rsum, fields[10]),
            ] {
                let recorded: f64 = field.parse().unwrap();
                assert!((recorded - expected).abs() <= 1e-10 * expected.abs().max(1.0));
            }
            lower += u64::from(ratio < 1.0);
            below_rho_pair += u64::from(csum < rsum);
            min = min.min(ratio);
            max = max.max(ratio);
            if square == 0.5 && inverse == 10.0 && native == 0.1 {
                let representative = &pair["representative"];
                for (key, expected) in [
                    ("baseline_GAE", bsum),
                    ("candidate_GAE", csum),
                    ("rho_GAE", rsum),
                    ("candidate_over_baseline", ratio),
                    ("baseline_over_rho", bsum / rsum),
                    ("candidate_over_rho", csum / rsum),
                ] {
                    let recorded = representative[key].as_f64().unwrap();
                    assert!((recorded - expected).abs() <= 1e-10 * expected.abs().max(1.0));
                }
                representative_ratios = Some((ratio, csum / rsum));
            }
            paired_cells_checked += 1;
        }
        assert_eq!(
            pair["cells_candidate_below_baseline"].as_u64().unwrap(),
            lower
        );
        assert_eq!(
            pair["cells_candidate_below_rho"].as_u64().unwrap(),
            below_rho_pair
        );
        for (key, expected) in [
            ("min_candidate_over_baseline", min),
            ("max_candidate_over_baseline", max),
        ] {
            let recorded = pair[key].as_f64().unwrap();
            assert!((recorded - expected).abs() <= 1e-10 * expected.abs().max(1.0));
        }
        let (paired_ratio, rho_ratio) = representative_ratios.unwrap();
        for (suffix, claimed, recomputed) in [
            (
                "candidate_over_baseline",
                pair["representative"]["candidate_over_baseline"]
                    .as_f64()
                    .unwrap(),
                paired_ratio,
            ),
            (
                "candidate_over_rho",
                pair["representative"]["candidate_over_rho"]
                    .as_f64()
                    .unwrap(),
                rho_ratio,
            ),
        ] {
            headline.push(json!({"name":format!("{name}/{suffix}"),
                "claimed":claimed,"recomputed":recomputed,"rel_tol":1e-10}));
        }
    }
    // The validation gate resolves raw_data relative to this evidence directory.
    let evidence_paths: Vec<String> = paths
        .iter()
        .map(|path| {
            path.strip_prefix(&format!("{root}/"))
                .map(|relative| format!("results/{relative}"))
                .unwrap_or_else(|| format!("../{}", path.strip_prefix("research/").unwrap()))
        })
        .collect();
    let validation = json!({"schema":"pencil-sparse-validation/v1",
        "status":if missing_replays.is_empty(){"pass"}else{"partial_replays_pending"},
        "independent":true,"raw_data":evidence_paths,"headline":headline,
        "files":files,"source_scalars_checked":scalars_checked,"replay_rows_checked":replay_rows_checked,
        "missing_replays":missing_replays,"price_cells_checked":valid_cells,
        "unpriced_cells":missing_cells,"total_cells":cells,"price_cells_below_rho":below_rho,
        "paired_comparison_cells_checked":paired_cells_checked,
        "method":"independent native source multiplication, raw/replay equality after wall removal and pre-peel online zero/default metadata normalization, all-cell arithmetic recomputation"});
    fs::write(
        format!("{root}/validation.json"),
        serde_json::to_vec_pretty(&validation).unwrap(),
    )
    .unwrap();
    let replay_certificate = json!({"schema":"pencil-replay-certificate/v1",
        "status":if missing_replays.is_empty(){"verified"}else{"partial_replays_pending"},
        "independent":true,"scalars_verified":true,"runs":files,
        "source_scalars_checked":scalars_checked,"nonwall_rows_checked":replay_rows_checked,
        "missing_replays":missing_replays,
        "normalization":"wall fields removed; pre-peel online rows gain solver_mode=online, solver_attempts=0, solver_index_ops=0"});
    fs::write(
        format!("{root}/replay-certificate.json"),
        serde_json::to_vec_pretty(&replay_certificate).unwrap(),
    )
    .unwrap();
    println!("{}", serde_json::to_string_pretty(&validation).unwrap());
}
