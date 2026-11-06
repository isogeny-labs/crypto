//! Independent source-curve replay of every held-out answer and price cell.
use five_leads_study::three_leads::{Elliptic, Field, Point};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::process::Command;
const ROOT: &str = "research/factor_base_heldout_20261025/results";
const ARCHIVE: &str = "research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst";
const COHORT: &str = "factor-base-heldout-evaluation/20261025/v1";
const TRAIN_COHORT: &str = "factor-base-graph-replication/20261024/v1";
const BITS: [u64; 7] = [16, 20, 24, 28, 32, 36, 40];
fn hex(d: &[u8]) -> String {
    d.iter().map(|x| format!("{x:02x}")).collect()
}
fn digest_for(cohort: &str, domain: &str, bits: u64, w: &Value) -> [u8; 32] {
    let mut h = Sha256::new();
    for x in [
        cohort,
        domain,
        &bits.to_string(),
        w["curve"]["icv1"].as_str().unwrap(),
        &w["target_index"].as_u64().unwrap().to_string(),
    ] {
        h.update(x.as_bytes());
        h.update([0]);
    }
    h.finalize().into()
}
fn digest(domain: &str, bits: u64, w: &Value) -> [u8; 32] {
    digest_for(COHORT, domain, bits, w)
}
fn d64(x: &[u8; 32]) -> u64 {
    u64::from_be_bytes(x[..8].try_into().unwrap())
}
fn records(path: &str) -> Vec<Value> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| serde_json::from_str(&l.unwrap()).unwrap())
        .collect()
}
fn csv_fields(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '"' if quoted && chars.peek() == Some(&'"') => {
                field.push('"');
                chars.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => {
                fields.push(std::mem::take(&mut field));
            }
            _ => field.push(ch),
        }
    }
    assert!(!quoted, "unterminated CSV quote");
    fields.push(field);
    fields
}
fn archive(bits: u64) -> BTreeMap<String, Value> {
    let member = format!("sessions_projective_chart_v2/order-genus-{bits}/records.jsonl");
    let out = Command::new("tar")
        .args(["--zstd", "-xOf", ARCHIVE, &member])
        .output()
        .unwrap();
    assert!(out.status.success());
    String::from_utf8(out.stdout)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .filter(|r| r["arm"] == "genus_projective")
        .map(|r| (r["workload"]["workload_id"].as_str().unwrap().to_owned(), r))
        .collect()
}
fn curve(w: &Value) -> (Elliptic, Point, u64) {
    let s = &w["curve_spec"];
    (
        Elliptic {
            f: Field::new(s["p"].as_u64().unwrap(), 2, s["modulus"].as_u64().unwrap()),
            c: [s["b"].as_u64().unwrap(), s["a"].as_u64().unwrap(), 0, 1],
            binary: false,
        },
        Point::affine(s["gx"].as_u64().unwrap(), s["gy"].as_u64().unwrap()),
        s["r"].as_u64().unwrap(),
    )
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
fn close(x: f64, y: f64) {
    assert!(
        (x - y).abs() <= 1e-9 * x.abs().max(y.abs()).max(1.0),
        "{x} != {y}"
    );
}
fn check_phases(row: &Value) {
    for key in [
        "m",
        "s",
        "i",
        "field_adds",
        "bit_ops",
        "ec_adds",
        "jac_adds",
        "sqrt_calls",
    ] {
        let sum: u64 = row["phases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| p["counts"][key].as_u64().unwrap())
            .sum();
        assert_eq!(row["counts"][key].as_u64().unwrap(), sum, "phase {key}");
    }
}
fn main() {
    let selection: Value =
        serde_json::from_str(&fs::read_to_string(format!("{ROOT}/selection.json")).unwrap())
            .unwrap();
    let summary: Value = serde_json::from_str(
        &fs::read_to_string(format!("{ROOT}/evaluation-summary.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(selection["selected_policy"], summary["selected_policy"]);
    let selection_hash = hex(&Sha256::digest(
        fs::read(format!("{ROOT}/selection.json")).unwrap(),
    ));
    assert_eq!(summary["selection_sha256"], selection_hash);
    assert!(fs::read_to_string(format!("{ROOT}/selection.SHA256"))
        .unwrap()
        .starts_with(&selection_hash));
    let prior_manifest: Value = serde_json::from_str(
        &fs::read_to_string(
            "research/factor_base_replication_20261024/results/raw/.artifacts.json",
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(selection["input_files"].as_array().unwrap().len(), 49);
    for item in selection["input_files"].as_array().unwrap() {
        let path = item["path"].as_str().unwrap();
        let bytes = fs::read(path).unwrap();
        assert_eq!(item["bytes"], bytes.len());
        let digest = hex(&Sha256::digest(&bytes));
        assert_eq!(item["sha256"], digest);
        if path.starts_with("research/factor_base_replication_20261024/results/raw/") {
            let name = path.rsplit('/').next().unwrap();
            assert_eq!(prior_manifest["files"][name]["sha256"], digest);
            assert_eq!(prior_manifest["files"][name]["bytes"], bytes.len());
        }
    }
    let arms = [
        (1, "any", 8),
        (1, "any", 16),
        (1, "any", 32),
        (2, "one", 16),
        (4, "one", 16),
        (2, "both", 16),
        (4, "both", 16),
    ];
    let mut best: Option<(f64, String)> = None;
    let mut train_ids = BTreeSet::new();
    let mut training_source_scalar_replays = 0u64;
    let train_grid = fs::read_to_string(format!("{ROOT}/training-price-grid.csv")).unwrap();
    let training_cells: Vec<Vec<String>> = train_grid.lines().skip(1).map(csv_fields).collect();
    assert_eq!(training_cells.len(), 7 * 36);
    for (d, a, j) in arms {
        let policy = format!("fb{d}-{a}-j{j}");
        let mut all = Vec::new();
        let mut ids = BTreeSet::new();
        for bits in BITS {
            let src = archive(bits);
            let mut models: Vec<String> = src
                .values()
                .map(|r| r["workload"]["curve"]["icv1"].as_str().unwrap().to_owned())
                .collect();
            models.sort();
            models.dedup();
            assert_eq!(models.len(), 3);
            let path = if a == "both" {
                format!("{ROOT}/raw/train-fb{d}-both-j{j}-bits{bits}.jsonl")
            } else {
                format!("research/factor_base_replication_20261024/results/raw/fb{d}-j{j}-bits{bits}.jsonl")
            };
            let rows = records(&path);
            assert_eq!(rows.len(), if a == "both" { 120 } else { 180 });
            for r in rows {
                if r["curve_icv1"] == models[2] {
                    continue;
                }
                assert!(r["curve_icv1"] == models[0] || r["curve_icv1"] == models[1]);
                assert_eq!(r["verified"], true);
                assert_eq!(r["rho"]["verified"], true);
                assert_eq!(r["recovered"], r["planted"]);
                assert_eq!(r["rho"]["recovered"], r["planted"]);
                check_phases(&r);
                check_phases(&r["rho"]);
                let w = &src[r["source_workload_id"].as_str().unwrap()]["workload"];
                let (e, g, order) = curve(w);
                let planted =
                    1 + d64(&digest_for(TRAIN_COHORT, "target-scalar/v1", bits, w)) % (order - 1);
                let target = e.mul(g, planted);
                assert_eq!(r["planted"], planted);
                assert_eq!(
                    r["algorithm_seed"]
                        .as_str()
                        .unwrap()
                        .parse::<u64>()
                        .unwrap(),
                    d64(&digest_for(TRAIN_COHORT, "algorithm-seed/v1", bits, w))
                );
                assert_eq!(r["target"][0], format!("0x{:x}", target.x));
                assert_eq!(r["target"][1], format!("0x{:x}", target.y));
                assert_eq!(e.mul(g, r["recovered"].as_u64().unwrap()), target);
                assert_eq!(e.mul(g, r["rho"]["recovered"].as_u64().unwrap()), target);
                training_source_scalar_replays += 2;
                assert!(ids.insert(r["workload_id"].as_str().unwrap().to_owned()));
                all.push(r);
            }
        }
        assert_eq!(all.len(), 840);
        assert_eq!(ids.len(), 840);
        if train_ids.is_empty() {
            train_ids = ids;
        } else {
            assert_eq!(train_ids, ids);
        }
        let c: f64 = all.iter().map(|r| price(r, 0.5, 10.0, 0.1)).sum();
        let rho: f64 = all.iter().map(|r| price(&r["rho"], 0.5, 10.0, 0.1)).sum();
        let ratio = c / rho;
        for s in [0.0, 0.5, 1.0] {
            for i in [1.0, 10.0, 100.0, 1000.0] {
                for l in [0.0, 0.1, 1.0] {
                    let cell = training_cells
                        .iter()
                        .find(|x| {
                            x[0] == policy
                                && x[4].parse::<f64>().unwrap() == s
                                && x[5].parse::<f64>().unwrap() == i
                                && x[6].parse::<f64>().unwrap() == l
                        })
                        .unwrap();
                    let candidate: f64 = all.iter().map(|r| price(r, s, i, l)).sum();
                    let reference: f64 = all.iter().map(|r| price(&r["rho"], s, i, l)).sum();
                    close(cell[7].parse().unwrap(), candidate);
                    close(cell[8].parse().unwrap(), reference);
                    close(cell[9].parse().unwrap(), candidate / reference);
                }
            }
        }
        let recorded = selection["candidate_scores"]
            .as_array()
            .unwrap()
            .iter()
            .find(|x| x["policy"] == policy)
            .unwrap();
        close(recorded["representative_ratio"].as_f64().unwrap(), ratio);
        if best
            .as_ref()
            .is_none_or(|(score, winner)| ratio < *score || (ratio == *score && policy < *winner))
        {
            best = Some((ratio, policy));
        }
    }
    assert_eq!(
        best.unwrap().1,
        selection["selected_policy"].as_str().unwrap()
    );
    let grid = fs::read_to_string(format!("{ROOT}/evaluation-price-grid.csv")).unwrap();
    let csv: Vec<Vec<String>> = grid.lines().skip(1).map(csv_fields).collect();
    assert_eq!(csv.len(), 15 * 36);
    let mut groups: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    let mut ids = BTreeSet::new();
    let mut model_ids = BTreeSet::new();
    let mut candidate_ok = 0;
    let mut rho_ok = 0;
    for bits in BITS {
        let source = archive(bits);
        assert_eq!(source.len(), 180);
        let mut model_names: Vec<String> = source
            .values()
            .map(|r| r["workload"]["curve"]["icv1"].as_str().unwrap().to_owned())
            .collect();
        model_names.sort();
        model_names.dedup();
        assert_eq!(model_names.len(), 3);
        let model = &model_names[2];
        model_ids.insert(model.clone());
        let rs = records(&format!("{ROOT}/raw/evaluation-bits{bits}.jsonl"));
        assert_eq!(rs.len(), 60);
        for row in rs {
            check_phases(&row);
            check_phases(&row["rho"]);
            assert_eq!(row["curve_icv1"], *model);
            assert_eq!(row["bits"], bits);
            assert!(!train_ids.contains(row["workload_id"].as_str().unwrap()));
            assert_eq!(row["protocol_sha256"], selection["protocol_sha256"]);
            let chosen = selection["candidate_scores"]
                .as_array()
                .unwrap()
                .iter()
                .find(|x| x["policy"] == selection["selected_policy"])
                .unwrap();
            assert_eq!(
                row["detail"]["base_denominator"],
                chosen["base_denominator"]
            );
            assert_eq!(row["detail"]["jump_count"], chosen["jumps"]);
            assert_eq!(
                row["detail"]["large_prime_acceptance"],
                chosen["large_prime_acceptance"]
            );
            let w = &source[row["source_workload_id"].as_str().unwrap()]["workload"];
            assert_eq!(row["source_workload_sha256"], w["workload_sha256"]);
            assert_eq!(row["target_index"], w["target_index"]);
            let (e, g, r) = curve(w);
            assert!(e.valid(g));
            let td = digest("target-scalar/v1", bits, w);
            let ad = digest("algorithm-seed/v1", bits, w);
            let k = 1 + d64(&td) % (r - 1);
            let seed = d64(&ad);
            let target = e.mul(g, k);
            assert_eq!(row["planted"], k);
            assert_eq!(
                row["algorithm_seed"]
                    .as_str()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap(),
                seed
            );
            assert_eq!(row["target_derivation"]["target_digest"], hex(&td));
            assert_eq!(row["target_derivation"]["algorithm_digest"], hex(&ad));
            let mut h = Sha256::new();
            h.update(digest("workload/v1", bits, w));
            h.update(k.to_be_bytes());
            h.update(target.x.to_be_bytes());
            h.update(target.y.to_be_bytes());
            let wd: [u8; 32] = h.finalize().into();
            let wh = hex(&wd);
            assert_eq!(row["workload_sha256"], wh);
            assert_eq!(row["workload_id"], format!("W{}", &wh[..12]));
            assert_eq!(row["target"][0], format!("0x{:x}", target.x));
            assert_eq!(row["target"][1], format!("0x{:x}", target.y));
            assert!(ids.insert(wh));
            if let Some(got) = row["recovered"].as_u64() {
                assert_eq!(e.mul(g, got), target);
                assert_eq!(got, k);
                assert_eq!(row["verified"], true);
                candidate_ok += 1;
            } else {
                assert_eq!(row["verified"], false);
            }
            if let Some(got) = row["rho"]["recovered"].as_u64() {
                assert_eq!(e.mul(g, got), target);
                assert_eq!(got, k);
                assert_eq!(row["rho"]["verified"], true);
                rho_ok += 1;
            } else {
                assert_eq!(row["rho"]["verified"], false);
            }
            assert_eq!(row["rho"]["source_add"], row["source_add"]);
            groups
                .entry("all".to_owned())
                .or_default()
                .push(row.clone());
            groups
                .entry(format!("bits-{bits}"))
                .or_default()
                .push(row.clone());
            groups
                .entry(format!("model-{model}"))
                .or_default()
                .push(row);
        }
    }
    assert_eq!(ids.len(), 420);
    assert_eq!(model_ids.len(), 7);
    let standardized = records(&format!("{ROOT}/evaluation-targets.jsonl"));
    assert_eq!(standardized.len(), 420);
    let raw_by_id: BTreeMap<String, &Value> = groups["all"]
        .iter()
        .map(|r| (r["workload_id"].as_str().unwrap().to_owned(), r))
        .collect();
    let mut standardized_ids = BTreeSet::new();
    for row in &standardized {
        let id = row["workload_id"].as_str().unwrap();
        assert!(standardized_ids.insert(id.to_owned()));
        let raw = raw_by_id[id];
        assert_eq!(row["curve_icv1"], raw["curve_icv1"]);
        assert_eq!(row["verified"], raw["verified"]);
        assert_eq!(row["rho_verified"], raw["rho"]["verified"]);
        let c = if raw["verified"] == true {
            Some(price(raw, 0.5, 10.0, 0.1))
        } else {
            None
        };
        let rho = if raw["rho"]["verified"] == true {
            Some(price(&raw["rho"], 0.5, 10.0, 0.1))
        } else {
            None
        };
        for (field, expected) in [
            ("candidate_GAE", c),
            ("rho_GAE", rho),
            ("candidate_over_rho", c.zip(rho).map(|(x, y)| x / y)),
        ] {
            match expected {
                Some(v) => close(row[field].as_f64().unwrap(), v),
                None => assert!(row[field].is_null()),
            }
        }
    }
    assert_eq!(standardized_ids.len(), 420);
    let csv_targets = fs::read_to_string(format!("{ROOT}/evaluation-targets.csv")).unwrap();
    let csv_rows: Vec<Vec<String>> = csv_targets.lines().skip(1).map(csv_fields).collect();
    assert_eq!(csv_rows.len(), 420);
    let mut csv_ids = BTreeSet::new();
    for cells in csv_rows {
        assert_eq!(cells.len(), 16);
        assert!(csv_ids.insert(cells[0].clone()));
        let raw = raw_by_id[&cells[0]];
        assert_eq!(cells[2], raw["curve_icv1"].as_str().unwrap());
        let candidate = if raw["verified"] == true {
            Some(price(raw, 0.5, 10.0, 0.1))
        } else {
            None
        };
        let rho = if raw["rho"]["verified"] == true {
            Some(price(&raw["rho"], 0.5, 10.0, 0.1))
        } else {
            None
        };
        for (field, expected) in [
            (10, candidate),
            (11, rho),
            (12, candidate.zip(rho).map(|(x, y)| x / y)),
        ] {
            match expected {
                Some(v) => close(cells[field].parse().unwrap(), v),
                None => assert!(cells[field].is_empty()),
            }
        }
    }
    assert_eq!(csv_ids, standardized_ids);
    assert_eq!(selection["train_models"].as_array().unwrap().len(), 14);
    for item in selection["train_models"].as_array().unwrap() {
        assert!(!model_ids.contains(item[1].as_str().unwrap()));
    }
    assert_eq!(summary["verified"], candidate_ok);
    assert_eq!(summary["rho_verified"], rho_ok);
    let mut cells = 0;
    for (scope, rs) in groups {
        for s in [0.0, 0.5, 1.0] {
            for i in [1.0, 10.0, 100.0, 1000.0] {
                for l in [0.0, 0.1, 1.0] {
                    let x = csv
                        .iter()
                        .find(|x| {
                            x[0] == scope
                                && x[5].parse::<f64>().unwrap() == s
                                && x[6].parse::<f64>().unwrap() == i
                                && x[7].parse::<f64>().unwrap() == l
                        })
                        .unwrap();
                    let c: Option<f64> = if rs.iter().all(|r| r["verified"] == true) {
                        Some(rs.iter().map(|r| price(r, s, i, l)).sum())
                    } else {
                        None
                    };
                    let rho: Option<f64> = if rs.iter().all(|r| r["rho"]["verified"] == true) {
                        Some(rs.iter().map(|r| price(&r["rho"], s, i, l)).sum())
                    } else {
                        None
                    };
                    let ratio = c.zip(rho).map(|(a, b)| a / b);
                    for (expected, actual) in [(&x[8], c), (&x[9], rho), (&x[10], ratio)] {
                        match actual {
                            Some(v) => close(expected.parse::<f64>().unwrap(), v),
                            None => assert!(expected.is_empty()),
                        }
                    }
                    cells += 1;
                }
            }
        }
    }
    assert_eq!(cells, 540);
    let report = json!({"schema":"factor-base-heldout-validation/v1","ok":true,"evaluation_targets":420,"training_source_scalar_replays":training_source_scalar_replays,"candidate_source_scalar_replays":candidate_ok,"rho_source_scalar_replays":rho_ok,"training_price_cells_recomputed":training_cells.len(),"evaluation_price_cells_recomputed":cells,"price_cells_recomputed":training_cells.len()+cells,"training_models":14,"evaluation_models":7,"model_overlap":0});
    fs::write(
        format!("{ROOT}/validator-report.json"),
        serde_json::to_vec_pretty(&report).unwrap(),
    )
    .unwrap();
    println!("{report}");
}
