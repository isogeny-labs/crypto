//! Independent equation construction and HKY fixed-point replay of qualified closures.
use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::{
    koblitz_groebner::{build_decomposition_system, FieldStructure},
    koblitz_index_calculus::find_irreducible_sparse,
    semaev_decomp::Gf2,
};
use num_bigint::BigUint;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};
#[allow(dead_code)]
#[path = "ic_algebraic_followup/hky.rs"]
mod hky;
#[allow(dead_code)]
#[path = "ic_algebraic_followup/wide.rs"]
mod wide;
fn read(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}
fn rows(path: &Path) -> Vec<Value> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}
fn columns(n: usize, d: u32) -> usize {
    let mut term = 1;
    let mut total = 1;
    for k in 1..=d as usize {
        term = term * (n + 1 - k) / k;
        total += term;
    }
    total
}
fn check(study: &Path, unit: &str, draw: u64, degree: u32) -> Value {
    assert!([2, 3].contains(&degree));
    let input_path = study.join(format!(
        "pipeline-receipts/qualified-queries-20261025/{unit}/output.jsonl"
    ));
    let input = std::fs::read(&input_path).unwrap();
    let hash = hex::encode(Sha256::digest(&input));
    let data: Value = std::str::from_utf8(&input)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .find(|r| r["draw"] == draw)
        .unwrap();
    let dir = study.join(format!(
        "pipeline-receipts/qualified-closures-20261025/{unit}-d{draw}-D{degree}"
    ));
    let process = read(&dir.join("status.json"));
    assert_eq!(process["exit_code"], 0);
    let recorded = rows(&dir.join("output.jsonl"));
    assert_eq!(recorded.len(), 1);
    let recorded = &recorded[0];
    assert_eq!(recorded["input_sha256"], hash);
    assert_eq!(recorded["degree"], degree);
    assert_eq!(recorded["draw"], draw);
    assert_eq!(recorded["branch"], 0);
    assert_eq!(recorded["guessed_variables"], json!([]));
    let sat = data["followup_selection"] == "positive";
    assert_eq!(recorded["sat_control"], sat);
    assert_eq!(sat, data["solutions"].as_u64().unwrap() > 0);
    assert_eq!(sat, data["point_decompositions"].as_u64().unwrap() > 0);
    let n = data["n"].as_u64().unwrap() as u32;
    let m = data["m"].as_u64().unwrap() as usize;
    let b = data["curve_b"].as_u64().unwrap();
    assert_eq!(recorded["curve_b"], b);
    let bs: Vec<_> = data["v_basis"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap())
        .collect();
    let irr = find_irreducible_sparse(n).unwrap();
    let gf = Gf2::new(&irr);
    assert_eq!(data["field_modulus"], gf.irr);
    let element = |x| F2mElement::from_biguint(&BigUint::from(x), n);
    let base: Vec<_> = bs.iter().copied().map(element).collect();
    let st = FieldStructure::new(n, &irr);
    let sys = build_decomposition_system(
        &base,
        &element(data["x_r"].as_u64().unwrap()),
        &element(b),
        m,
        &st,
    )
    .unwrap();
    let (eqs, nv) = wide::chain_for_b(&gf, &bs, data["x_r"].as_u64().unwrap(), m, b);
    assert_eq!(nv, sys.n_vars);
    assert_eq!(recorded["variables"], nv);
    let old: BTreeSet<_> = sys
        .equations
        .iter()
        .map(|p| {
            p.terms
                .iter()
                .map(|t| u128::from(t.mask))
                .collect::<BTreeSet<_>>()
        })
        .collect();
    let new: BTreeSet<_> = eqs
        .iter()
        .map(|p| p.iter().copied().collect::<BTreeSet<_>>())
        .collect();
    assert_eq!(old, new);
    let cols = columns(nv, degree);
    assert!(cols <= 40000);
    let (plain, rank, rounds) = hky::closure_par(&eqs, nv as u32, degree, cols);
    let status = if rank == cols {
        "refuted"
    } else {
        "saturated_not_refuted"
    };
    assert_eq!(recorded["status"], status);
    if status == "saturated_not_refuted" {
        assert_eq!(recorded["rank"], rank);
    } else {
        assert!(!sat);
    }
    json!({"schema":"ic-qualified-closure-replay/v1","unit":unit,"draw":draw,"degree":degree,"sat_control":sat,
        "input_sha256":hash,"recorded_rows_sha256":hex::encode(Sha256::digest(std::fs::read(dir.join("output.jsonl")).unwrap())),
        "equation_builders_agree":true,"full_columns":cols,"plain_rank":plain,"closure_rank":rank,"hky_rounds":rounds,
        "recorded_solver_words":recorded["closure_word_ops"],"word_cost_replay":"not independently counted by HKY",
        "closure_status":status,"status":"independent_hky_verdict_and_rank_agree"})
}
fn summary(study: &Path) -> Value {
    let query = read(&study.join("pipeline-receipts/qualified-queries-20261025/summary.json"));
    let mut groups = BTreeMap::<(u64, bool), Value>::new();
    let mut attempts = vec![];
    let mut selected = 0;
    let mut short_negative = 0;
    let mut short_positive = 0;
    for unit in query["units"].as_array().unwrap() {
        let m = unit["m"].as_u64().unwrap();
        let n = unit["n"].as_u64().unwrap();
        let ell = unit["ell"].as_u64().unwrap();
        let family = unit["family"].as_str().unwrap();
        let name = format!("m{m}-n{n}-ell{ell}-{family}");
        let neg = unit["selected_negative_draws"].as_array().unwrap();
        let pos = unit["selected_positive_draws"].as_array().unwrap();
        short_negative += 4 - neg.len();
        short_positive += 2 - pos.len();
        selected += neg.len() + pos.len();
        for (sat, draws) in [(false, neg), (true, pos)] {
            for draw in draws {
                let draw = draw.as_u64().unwrap();
                for degree in [2u64, 3] {
                    let dirname = format!("{name}-d{draw}-D{degree}");
                    let run = study
                        .join("pipeline-receipts/qualified-closures-20261025")
                        .join(&dirname);
                    let process = read(&run.join("status.json"));
                    let rs = rows(&run.join("output.jsonl"));
                    let row = rs.first().cloned().unwrap_or(Value::Null);
                    let completed = process["exit_code"] == 0
                        && ["refuted", "saturated_not_refuted"]
                            .iter()
                            .any(|s| row["status"] == *s);
                    let refdir = study
                        .join("pipeline-receipts/qualified-closure-replay-20261025")
                        .join(&dirname);
                    let verified = if refdir.join("status.json").exists()
                        && read(&refdir.join("status.json"))["exit_code"] == 0
                    {
                        let r = read(&refdir.join("output.json"));
                        assert_eq!(r["status"], "independent_hky_verdict_and_rank_agree");
                        assert_eq!(
                            r["recorded_rows_sha256"],
                            hex::encode(Sha256::digest(
                                std::fs::read(run.join("output.jsonl")).unwrap()
                            ))
                        );
                        true
                    } else {
                        false
                    };
                    let g=groups.entry((degree,sat)).or_insert_with(||json!({"degree":degree,"sat_control":sat,"attempts":0,"refuted":0,"nonrefuting":0,"unknown":0,"independent_hky_verified":0,"known_solver_words":0u64,"all_solver_words":null}));
                    let increment = |g: &mut Value, key: &str, amount: u64| {
                        g[key] = json!(g[key].as_u64().unwrap().checked_add(amount).unwrap());
                    };
                    increment(g, "attempts", 1);
                    if completed {
                        increment(
                            g,
                            if row["status"] == "refuted" {
                                "refuted"
                            } else {
                                "nonrefuting"
                            },
                            1,
                        );
                        increment(
                            g,
                            "known_solver_words",
                            row["closure_word_ops"].as_u64().unwrap(),
                        );
                    } else {
                        increment(g, "unknown", 1);
                    }
                    if verified {
                        increment(g, "independent_hky_verified", 1);
                    }
                    attempts.push(json!({"unit":name,"draw":draw,"degree":degree,"sat_control":sat,"exit_code":process["exit_code"],"closure_status":row["status"],"independent_hky_verified":verified,"solver_words":row["closure_word_ops"]}));
                }
            }
        }
    }
    for g in groups.values_mut() {
        if g["unknown"] == 0 {
            g["all_solver_words"] = g["known_solver_words"].clone();
        }
    }
    json!({"schema":"ic-qualified-closure-summary/v1","selected_queries":selected,"planned_attempts":2*selected,"recorded_attempts":attempts.len(),
        "negative_quota_shortage":short_negative,"positive_quota_shortage":short_positive,"groups":groups.into_values().collect::<Vec<_>>(),"attempts":attempts,
        "status":"recomputed_from_raw_with_independent_replay_coverage_explicit","scope":"D2/D3 zero-guess finite closure stages; missing phase/physical costs, D4 and complete guessing minima remain open"})
}
fn main() {
    let a: Vec<_> = std::env::args().skip(1).collect();
    let result = match a.first().map(String::as_str) {
        Some("case") if a.len() == 5 => check(
            Path::new(&a[1]),
            &a[2],
            a[3].parse().unwrap(),
            a[4].parse().unwrap(),
        ),
        Some("summary") if a.len() == 2 => summary(Path::new(&a[1])),
        _ => panic!("case STUDY UNIT DRAW DEGREE | summary STUDY"),
    };
    println!("{result}");
}
