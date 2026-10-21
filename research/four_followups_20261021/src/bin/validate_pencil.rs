//! Independent replay, source-scalar and representative-price recomputation.
use five_leads_study::three_leads::{Elliptic, Field, Point};
use serde_json::{json, Value};
use std::collections::BTreeMap;
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
fn costs(row: &Value) -> (f64, f64) {
    let f = &row["counts"];
    let a = &row["arithmetic"];
    let u = &row["source_add"];
    let p = row["p"].as_u64().unwrap();
    let denominator = n(u, "m") + 0.5 * n(u, "s") + 10.0 * n(u, "i");
    let m = n(f, "m") + n(a, "base_muls") - n(a, "base_inversions") * bit_count(p - 2) as f64;
    assert!(m >= 0.0);
    let candidate_native = n(f, "field_adds")
        + n(f, "bit_ops")
        + n(a, "base_adds")
        + n(a, "scalar_adds")
        + n(a, "scalar_muls")
        + n(a, "scalar_inversions")
        + n(row, "solver_row_updates");
    let candidate = (m + 0.5 * n(f, "s") + 10.0 * (n(f, "i") + n(a, "base_inversions")))
        / denominator
        + 0.1 * candidate_native;
    let rho = &row["rho"];
    let rc = &rho["counts"];
    let reference = (n(rc, "m") + 0.5 * n(rc, "s") + 10.0 * n(rc, "i")) / denominator
        + 0.1 * (logical(&rho["native"]) + n(rc, "field_adds") + n(rc, "bit_ops"));
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
fn main() {
    let paths: Vec<String> = std::env::args().skip(1).collect();
    assert!(
        !paths.is_empty(),
        "usage: validate-pencil PRIMARY.jsonl ..."
    );
    let summary: Value =
        serde_json::from_slice(&fs::read(format!("{ROOT}/summary.json")).unwrap()).unwrap();
    let mut heads = Vec::new();
    let mut data = vec![
        json!({"path":"../../research/four_followups_20261021/results/pencil-pilot/summary.json"}),
    ];
    let mut runs = Vec::new();
    let mut verified = 0;
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
            let (a, b) = costs(row);
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
        data.push(json!({"path":format!("../../{path}")}));
        runs.push(json!({"primary":path,"replay":replay,"rows":61,"scalars_verified":60}));
    }
    let validator = json!({"schema":"pencil-pilot-independent-validator/v1","status":"pass",
        "independent":true,"method":"separate native raw-row arithmetic and archived source multiplication",
        "raw_data":data,"headline":heads,"source_scalars_checked":verified});
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
