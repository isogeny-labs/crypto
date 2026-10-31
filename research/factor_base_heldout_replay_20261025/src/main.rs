//! Source-curve validation and matched-price comparison of a frozen held-out replay.
use five_leads_study::three_leads::{Elliptic, Field, Point};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

const BITS: [u64; 7] = [16, 20, 24, 28, 32, 36, 40];
const COHORT: &str = "factor-base-heldout-evaluation/20261025/v1";
const ORIGINAL: &str = "research/factor_base_heldout_20261025/results/raw";
const PROTOCOL_SHA: &str = "325d0efb2c1e7b7328bd472bda310464f4817d62a5ca3a4ca6481c67cee84c9c";
fn read_rows(path: &Path) -> Vec<Value> {
    if !path.exists() {
        return vec![];
    }
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|x| serde_json::from_str(&x.unwrap()).unwrap())
        .collect()
}
fn hex(x: &[u8]) -> String {
    x.iter().map(|v| format!("{v:02x}")).collect()
}
fn digest(domain: &str, bits: u64, w: &Value) -> [u8; 32] {
    let mut h = Sha256::new();
    for x in [
        COHORT,
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
fn d64(x: &[u8; 32]) -> u64 {
    u64::from_be_bytes(x[..8].try_into().unwrap())
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
fn phases(row: &Value) -> bool {
    [
        "m",
        "s",
        "i",
        "field_adds",
        "bit_ops",
        "ec_adds",
        "jac_adds",
        "sqrt_calls",
    ]
    .iter()
    .all(|key| {
        row["counts"][key].as_u64()
            == Some(
                row["phases"]
                    .as_array()
                    .map(|ps| {
                        ps.iter()
                            .map(|p| p["counts"][key].as_u64().unwrap_or(u64::MAX))
                            .sum()
                    })
                    .unwrap_or(u64::MAX),
            )
    })
}
fn valid_source(
    row: &Value,
    bits: u64,
    source: &BTreeMap<String, Value>,
) -> Result<(bool, bool), String> {
    if row["schema"] != "factor-base-heldout-record/v1"
        || row["replication_id"] != COHORT
        || row["bits"] != bits
        || row["protocol_sha256"] != PROTOCOL_SHA
    {
        return Err("schema/cohort/protocol mismatch".into());
    }
    if row["detail"]["base_denominator"] != 1
        || row["detail"]["jump_count"] != 32
        || row["detail"]["large_prime_acceptance"] != "any"
    {
        return Err("policy mismatch".into());
    }
    if !phases(row) || !phases(&row["rho"]) {
        return Err("phase totals mismatch".into());
    }
    let source_id = row["source_workload_id"]
        .as_str()
        .ok_or("source ID absent")?;
    let w = source.get(source_id).ok_or("source workload absent")?;
    if row["source_workload_sha256"] != w["workload_sha256"]
        || row["target_index"] != w["target_index"]
        || row["curve_icv1"] != w["curve"]["icv1"]
    {
        return Err("source identity mismatch".into());
    }
    let (e, g, r) = curve(w);
    if !e.valid(g) {
        return Err("invalid source generator".into());
    }
    let td = digest("target-scalar/v1", bits, w);
    let ad = digest("algorithm-seed/v1", bits, w);
    let k = 1 + d64(&td) % (r - 1);
    let target = e.mul(g, k);
    let mut h = Sha256::new();
    h.update(digest("workload/v1", bits, w));
    h.update(k.to_be_bytes());
    h.update(target.x.to_be_bytes());
    h.update(target.y.to_be_bytes());
    let wd: [u8; 32] = h.finalize().into();
    let wh = hex(&wd);
    if row["workload_sha256"] != wh
        || row["workload_id"] != format!("W{}", &wh[..12])
        || row["planted"] != k
        || row["algorithm_seed"]
            .as_str()
            .and_then(|x| x.parse::<u64>().ok())
            != Some(d64(&ad))
        || row["target_derivation"]["target_digest"] != hex(&td)
        || row["target_derivation"]["algorithm_digest"] != hex(&ad)
        || row["target"][0] != format!("0x{:x}", target.x)
        || row["target"][1] != format!("0x{:x}", target.y)
    {
        return Err("derived target mismatch".into());
    }
    if row["rho"]["source_add"] != row["source_add"] {
        return Err("source unit mismatch".into());
    }
    let scalar_ok = |answer: &Value| {
        answer["recovered"]
            .as_u64()
            .is_some_and(|got| answer["verified"] == true && got == k && e.mul(g, got) == target)
    };
    Ok((scalar_ok(row), scalar_ok(&row["rho"])))
}
fn num(v: &Value, key: &str) -> f64 {
    v[key].as_u64().unwrap() as f64
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
    let args: Vec<String> = env::args().collect();
    assert_eq!(
        args.len(),
        4,
        "usage: factor-base-heldout-replay SOURCE_BUNDLE_ROOT REPLAY_RESULTS_ROOT OUTPUT_DIR"
    );
    let source_root = Path::new(&args[1]);
    let replay_root = Path::new(&args[2]);
    let output = Path::new(&args[3]);
    fs::create_dir_all(output).unwrap();
    let mut original_all = BTreeMap::<String, Value>::new();
    let mut replay_all = BTreeMap::<String, Value>::new();
    let mut source_all = BTreeMap::<u64, BTreeMap<String, Value>>::new();
    let mut file_rows = Vec::new();
    for bits in BITS {
        let src=read_rows(&source_root.join(format!("research/five_leads_e2e_20261019/sessions_projective_chart_v2/order-genus-{bits}/records.jsonl")));
        assert_eq!(src.len(), 540, "frozen source records bits {bits}");
        let source: BTreeMap<String, Value> = src
            .into_iter()
            .filter(|x| x["arm"] == "genus_projective")
            .map(|x| {
                (
                    x["workload"]["workload_id"].as_str().unwrap().to_owned(),
                    x["workload"].clone(),
                )
            })
            .collect();
        assert_eq!(source.len(), 180);
        source_all.insert(bits, source);
        let orig = read_rows(&Path::new(ORIGINAL).join(format!("evaluation-bits{bits}.jsonl")));
        assert_eq!(orig.len(), 60, "frozen original rows bits {bits}");
        for r in orig {
            assert!(original_all
                .insert(r["workload_id"].as_str().unwrap().to_owned(), r)
                .is_none());
        }
        let original_path = Path::new(ORIGINAL).join(format!("evaluation-bits{bits}.jsonl"));
        let replay_path = replay_root.join(format!("raw/evaluation-bits{bits}.jsonl"));
        let original_bytes = fs::read(&original_path).unwrap();
        let replay_bytes = fs::read(&replay_path).unwrap_or_default();
        let replay = read_rows(&replay_path);
        file_rows.push(json!({"bits":bits,"rows":replay.len(),"original_sha256":hex(&Sha256::digest(&original_bytes)),"replay_sha256":hex(&Sha256::digest(&replay_bytes)),"byte_identical":original_bytes==replay_bytes}));
        for r in replay {
            let id = r["workload_id"].as_str().unwrap_or("").to_owned();
            assert!(
                !id.is_empty() && replay_all.insert(id, r).is_none(),
                "duplicate/empty replay ID"
            );
        }
    }
    assert_eq!(original_all.len(), 420);
    let mut targets = File::create(output.join("targets.jsonl")).unwrap();
    let mut target_csv = File::create(output.join("targets.csv")).unwrap();
    writeln!(target_csv,"bits,workload_id,source_workload_id,original_verified,replay_present,replay_verified,rho_verified,identity_match,counts_match,candidate_GAE,rho_GAE,candidate_over_rho,error").unwrap();
    let mut valid = 0usize;
    let mut rho_valid = 0usize;
    let mut identity_match = 0usize;
    let mut counts_match = 0usize;
    let mut unexpected = Vec::new();
    let mut good = Vec::<Value>::new();
    for (id, orig) in &original_all {
        let bits = orig["bits"].as_u64().unwrap();
        let replay = replay_all.get(id);
        let mut errors = Vec::<String>::new();
        let mut identity = false;
        let mut count_eq = false;
        let mut candidate_ok = false;
        let mut rho_ok = false;
        let mut candidate = None;
        let mut rho = None;
        if let Some(r) = replay {
            identity = [
                "workload_id",
                "workload_sha256",
                "source_workload_id",
                "source_workload_sha256",
                "target_index",
                "curve_icv1",
                "p",
                "target",
                "planted",
                "algorithm_seed",
                "target_derivation",
                "protocol_sha256",
            ]
            .iter()
            .all(|key| r[key] == orig[key]);
            if identity {
                identity_match += 1;
            } else {
                errors.push("original/replay target identity mismatch".into());
            }
            count_eq = ["counts", "native", "phases", "source_add"]
                .iter()
                .all(|key| r[key] == orig[key])
                && ["counts", "native", "phases", "source_add"]
                    .iter()
                    .all(|key| r["rho"][key] == orig["rho"][key]);
            if count_eq {
                counts_match += 1;
            }
            match valid_source(r, bits, &source_all[&bits]) {
                Ok((c, reference)) => {
                    candidate_ok = c;
                    rho_ok = reference;
                    if c {
                        valid += 1;
                    } else {
                        errors.push("candidate scalar invalid or missing".into());
                    }
                    if reference {
                        rho_valid += 1;
                    } else {
                        errors.push("rho scalar invalid or missing".into());
                    }
                }
                Err(e) => errors.push(e),
            }
            if candidate_ok && identity {
                candidate = Some(price(r, 0.5, 10.0, 0.1));
            }
            if rho_ok && identity {
                rho = Some(price(&r["rho"], 0.5, 10.0, 0.1));
            }
            if candidate_ok && rho_ok && identity {
                good.push(r.clone());
            }
        } else {
            errors.push("missing replay row".into());
        }
        let ratio = candidate.zip(rho).map(|(a, b)| a / b);
        let record = json!({"schema":"factor-base-heldout-replay-target/v1","bits":bits,"workload_id":id,"source_workload_id":orig["source_workload_id"],"original_verified":orig["verified"]==true && orig["rho"]["verified"]==true,"replay_present":replay.is_some(),"replay_verified":candidate_ok,"rho_verified":rho_ok,"identity_match":identity,"counts_match":count_eq,"candidate_GAE":candidate,"rho_GAE":rho,"candidate_over_rho":ratio,"error":if errors.is_empty(){Value::Null}else{json!(errors.join("; "))}});
        writeln!(targets, "{record}").unwrap();
        writeln!(
            target_csv,
            "{},{},{},{},{},{},{},{},{},{},{},{},{}",
            bits,
            id,
            orig["source_workload_id"].as_str().unwrap(),
            orig["verified"] == true && orig["rho"]["verified"] == true,
            replay.is_some(),
            candidate_ok,
            rho_ok,
            identity,
            count_eq,
            candidate.map_or(String::new(), |x| x.to_string()),
            rho.map_or(String::new(), |x| x.to_string()),
            ratio.map_or(String::new(), |x| x.to_string()),
            if errors.is_empty() {
                String::new()
            } else {
                errors.join("; ")
            }
        )
        .unwrap();
    }
    for id in replay_all.keys() {
        if !original_all.contains_key(id) {
            unexpected.push(id.clone());
        }
    }
    let mut grid = File::create(output.join("price-grid.csv")).unwrap();
    writeln!(grid,"scope,targets,verified,S_per_M,I_per_M,logical_price,candidate_GAE,rho_GAE,candidate_over_rho,outcome").unwrap();
    let mut scopes = vec![("all".to_owned(), good.clone())];
    for bits in BITS {
        scopes.push((
            format!("bits-{bits}"),
            good.iter().filter(|r| r["bits"] == bits).cloned().collect(),
        ));
    }
    let mut summary = Vec::new();
    for (scope, rows) in scopes {
        let expected = if scope == "all" { 420 } else { 60 };
        let complete = rows.len() == expected;
        let mut wins = 0;
        let mut losses = 0;
        let mut ties = 0;
        let mut nulls = 0;
        let mut representative = Value::Null;
        for s in [0.0, 0.5, 1.0] {
            for i in [1.0, 10.0, 100.0, 1000.0] {
                for l in [0.0, 0.1, 1.0] {
                    let c = complete.then(|| rows.iter().map(|r| price(r, s, i, l)).sum::<f64>());
                    let rho = complete
                        .then(|| rows.iter().map(|r| price(&r["rho"], s, i, l)).sum::<f64>());
                    let ratio = c.zip(rho).map(|(a, b)| a / b);
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
                        None => {
                            nulls += 1;
                            "null"
                        }
                    };
                    if s == 0.5 && i == 10.0 && l == 0.1 {
                        representative =
                            json!({"candidate_GAE":c,"rho_GAE":rho,"candidate_over_rho":ratio});
                    }
                    writeln!(
                        grid,
                        "{scope},{expected},{},{s},{i},{l},{},{},{},{}",
                        rows.len(),
                        c.map_or(String::new(), |x| x.to_string()),
                        rho.map_or(String::new(), |x| x.to_string()),
                        ratio.map_or(String::new(), |x| x.to_string()),
                        outcome
                    )
                    .unwrap();
                }
            }
        }
        let mut target_ratios: Vec<f64> = rows
            .iter()
            .map(|r| price(r, 0.5, 10.0, 0.1) / price(&r["rho"], 0.5, 10.0, 0.1))
            .collect();
        target_ratios.sort_by(|a, b| a.total_cmp(b));
        let target_wins = target_ratios.iter().filter(|x| **x < 1.0).count();
        let target_median = if target_ratios.is_empty() {
            None
        } else if target_ratios.len().is_multiple_of(2) {
            Some(
                (target_ratios[target_ratios.len() / 2 - 1]
                    + target_ratios[target_ratios.len() / 2])
                    / 2.0,
            )
        } else {
            Some(target_ratios[target_ratios.len() / 2])
        };
        summary.push(json!({"scope":scope,"targets":expected,"verified":rows.len(),"representative":representative,"wins":wins,"losses":losses,"ties":ties,"nulls":nulls,"verified_target_wins":target_wins,"verified_target_median_ratio":target_median}));
    }
    let report = json!({"schema":"factor-base-heldout-replay-summary/v1","original_targets":420,"replay_rows":replay_all.len(),"verified_candidate":valid,"verified_rho":rho_valid,"identity_matches":identity_match,"count_matches":counts_match,"unexpected_replay_ids":unexpected,"files":file_rows,"summary":summary});
    fs::write(
        output.join("summary.json"),
        serde_json::to_vec_pretty(&report).unwrap(),
    )
    .unwrap();
    println!("{report}");
}
