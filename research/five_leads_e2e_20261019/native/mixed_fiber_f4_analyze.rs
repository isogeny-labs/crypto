//! Native validation and 12x3 lower-bound grid for F4-backed DLP records.
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;

const BASE: &str = "research/five_leads_e2e_20261019";
const ARMS: [&str; 3] = ["homogeneous", "mixed", "random"];

fn u(v: &Value, key: &str) -> u64 {
    v[key].as_u64().expect("u64 field")
}
fn native_candidate(v: &Value) -> u64 {
    let c = &v["candidate"];
    u(v, "selection_draws_uncharged")
        + u(c, "rng_draws_uncharged")
        + u(c, "sign_lift_checks")
        + u(c, "row_operations_uncharged")
        + u(c, "scalar_adds_uncharged")
        + u(c, "scalar_muls_uncharged")
        + u(c, "scalar_inversions_uncharged")
}
fn native_table(v: &Value) -> u64 {
    let c = &v["candidate"];
    u(v, "selection_draws_uncharged")
        + u(c, "rng_draws_uncharged")
        + u(c, "table_lookups_uncharged")
        + u(c, "row_operations_uncharged")
        + u(c, "scalar_adds_uncharged")
        + u(c, "scalar_muls_uncharged")
        + u(c, "scalar_inversions_uncharged")
        + u(c, "pair_insertions")
}
fn native_rho(v: &Value) -> u64 {
    v["rho"]["native"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|(k, _)| k.ends_with("_uncharged"))
        .map(|(_, value)| value.as_u64().unwrap())
        .sum()
}
fn price(c: &Value, s: f64, i: f64, native: u64, weight: f64) -> f64 {
    u(c, "m") as f64 + s * u(c, "s") as f64 + i * u(c, "i") as f64 + weight * native as f64
}
fn main() {
    let rows: Vec<Value> =
        fs::read_to_string(format!("{BASE}/correctness/mixed-fiber-f4-dlp-v1.jsonl"))
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
    assert_eq!(rows.len(), 540);
    let oracle: Vec<Value> = fs::read_to_string(format!(
        "{BASE}/correctness/mixed-fiber-f4-dlp-oracle-v1.jsonl"
    ))
    .unwrap()
    .lines()
    .map(|line| serde_json::from_str(line).unwrap())
    .collect();
    assert_eq!(oracle.len(), 540);
    let relation_trials: usize = oracle
        .iter()
        .map(|r| r["queries"].as_array().unwrap().len())
        .sum();
    let identity_trials: usize = oracle
        .iter()
        .flat_map(|r| r["queries"].as_array().unwrap())
        .filter(|q| q["x"].is_null())
        .count();
    assert_eq!(relation_trials, 2316);
    assert_eq!(identity_trials, 26);
    assert!(oracle.iter().all(|r| r["complete_and_matching"] == true));
    let target_gate: Vec<Value> = fs::read_to_string(format!(
        "{BASE}/correctness/mixed-fiber-f4-target-cohort-v1.jsonl"
    ))
    .unwrap()
    .lines()
    .map(|line| serde_json::from_str(line).unwrap())
    .collect();
    assert_eq!(target_gate.len(), 540);
    assert!(target_gate
        .iter()
        .all(|r| r["complete_and_matching"] == true && r["timed_out"] == false));
    let oracle_by_key: BTreeMap<(u64, u64, u64, u64, String), &Value> = oracle
        .iter()
        .map(|row| {
            (
                (
                    u(row, "p"),
                    u(row, "a"),
                    u(row, "b"),
                    u(row, "target_index"),
                    row["arm"].as_str().unwrap().to_string(),
                ),
                row,
            )
        })
        .collect();
    let exhaustive: Vec<Value> = fs::read_to_string(format!(
        "{BASE}/correctness/mixed-fiber-polynomial-dlp-v1.jsonl"
    ))
    .unwrap()
    .lines()
    .map(|line| serde_json::from_str(line).unwrap())
    .collect();
    assert_eq!(exhaustive.len(), 540);
    let exhaustive_by_key: BTreeMap<(u64, u64, u64, u64, String), &Value> = exhaustive
        .iter()
        .map(|row| {
            (
                (
                    u(row, "p"),
                    u(row, "a"),
                    u(row, "b"),
                    u(row, "target_index"),
                    row["arm"].as_str().unwrap().to_string(),
                ),
                row,
            )
        })
        .collect();
    let old: Vec<Value> =
        fs::read_to_string(format!("{BASE}/correctness/mixed-fiber-dlp-v2.jsonl"))
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
    assert_eq!(old.len(), rows.len());
    let old_by_key: BTreeMap<(u64, u64, u64, u64, String), &Value> = old
        .iter()
        .map(|row| {
            (
                (
                    u(row, "p"),
                    u(row, "a"),
                    u(row, "b"),
                    u(row, "target_index"),
                    row["arm"].as_str().unwrap().to_string(),
                ),
                row,
            )
        })
        .collect();
    let mut targets: BTreeMap<(u64, u64, u64, u64), Vec<&Value>> = BTreeMap::new();
    for row in &rows {
        assert_eq!(row["schema"], "mixed-fiber-f4-dlp/v1");
        assert_eq!(row["candidate"]["verified"], true);
        assert_eq!(row["candidate"]["exhausted"], false);
        assert_eq!(row["rho"]["verified"], true);
        assert_eq!(
            row["candidate"]["recovered"],
            row["planted_scalar_check_only"]
        );
        assert_eq!(row["rho"]["recovered"], row["planted_scalar_check_only"]);
        let key = (
            u(row, "p"),
            u(row, "a"),
            u(row, "b"),
            u(row, "target_index"),
            row["arm"].as_str().unwrap().to_string(),
        );
        let matched = old_by_key.get(&key).expect("missing pair-table control");
        let checked_oracle = oracle_by_key.get(&key).expect("missing F4 oracle");
        let checked_exhaustive = exhaustive_by_key
            .get(&key)
            .expect("missing exhaustive polynomial control");
        assert_eq!(checked_oracle["complete_and_matching"], true);
        assert_eq!(
            row["candidate"]["f4_field_ops"],
            checked_oracle["total_f4_field_ops"]
        );
        assert_eq!(row["candidate"]["recovered"], checked_oracle["recovered"]);
        assert_eq!(
            row["candidate"]["query_xs"].as_array().unwrap().len(),
            checked_oracle["queries"].as_array().unwrap().len()
        );
        for (query, verified) in row["candidate"]["query_xs"]
            .as_array()
            .unwrap()
            .iter()
            .zip(checked_oracle["queries"].as_array().unwrap())
        {
            assert_eq!(*query, verified["x"]);
            assert_eq!(verified["complete_and_matching"], true);
        }
        for field in ["recovered", "trials", "relations", "rank", "query_xs"] {
            assert_eq!(
                row["candidate"][field],
                checked_exhaustive["candidate"][field]
            );
        }
        assert_eq!(row["target"], matched["target"]);
        assert_eq!(row["fibers"], matched["fibers"]);
        assert_eq!(row["generator"], matched["generator"]);
        assert_eq!(row["rho"], matched["rho"]);
        assert!(u(&row["candidate"], "trials") <= u(&row["candidate"], "cap"));
        assert_eq!(u(&row["candidate"], "polynomial_checks"), 0);
        assert!(u(&row["candidate"], "f4_field_ops") > 0);
        assert!(u(&row["candidate"], "sign_lift_checks") >= u(&row["candidate"], "relations"));
        for field in ["m", "s", "i"] {
            assert!(
                u(&row["candidate"]["total_counts"], field) >= u(&row["selection_counts"], field)
            );
        }
        targets
            .entry((
                u(row, "p"),
                u(row, "a"),
                u(row, "b"),
                u(row, "target_index"),
            ))
            .or_default()
            .push(row);
    }
    assert_eq!(targets.len(), 180);
    let mut curves: BTreeMap<(u64, u64, u64), BTreeSet<u64>> = BTreeMap::new();
    for (&(p, a, b, index), group) in &targets {
        assert_eq!(group.len(), 3);
        let arms: BTreeSet<&str> = group.iter().map(|v| v["arm"].as_str().unwrap()).collect();
        assert_eq!(arms, BTreeSet::from(ARMS));
        assert!(group
            .iter()
            .all(|v| v["target"] == group[0]["target"] && v["rho"] == group[0]["rho"]));
        curves.entry((p, a, b)).or_default().insert(index);
    }
    assert_eq!(curves.len(), 9);
    assert!(curves
        .values()
        .all(|v| v.len() == 20 && (0..20).all(|j| v.contains(&j))));
    assert!([97, 193, 257]
        .iter()
        .all(|&p| curves.keys().filter(|key| key.0 == p).count() == 3));
    let mut slices: Vec<(String, String)> = vec![("all".into(), "all".into())];
    slices.extend([97, 193, 257].map(|p| ("prime".into(), p.to_string())));
    slices.extend(
        curves
            .keys()
            .map(|(p, a, b)| ("curve".into(), format!("{p}:{a}:{b}"))),
    );
    let mut grid=String::from("kind,label,arm,targets,S_per_M,I_per_M,native_M_per_logical_op,candidate_M,rho_M,pair_table_M,candidate_over_rho,candidate_over_pair_table\n");
    let mut grid_rows = Vec::new();
    let mut summary = Vec::new();
    for (kind, label) in slices.clone() {
        for arm in ARMS {
            let subset: Vec<&Value> = rows
                .iter()
                .filter(|row| {
                    row["arm"] == arm
                        && match kind.as_str() {
                            "all" => true,
                            "prime" => u(row, "p").to_string() == label,
                            "curve" => {
                                format!("{}:{}:{}", u(row, "p"), u(row, "a"), u(row, "b")) == label
                            }
                            _ => unreachable!(),
                        }
                })
                .collect();
            let mut ratios = Vec::new();
            let mut table_ratios = Vec::new();
            let mut representative = None;
            for s in [0.0, 0.5, 1.0] {
                for i in [1.0, 10.0, 100.0, 1000.0] {
                    for weight in [0.0, 0.1, 1.0] {
                        let candidate: f64 = subset
                            .iter()
                            .map(|row| {
                                price(
                                    &row["candidate"]["total_counts"],
                                    s,
                                    i,
                                    native_candidate(row),
                                    weight,
                                ) + u(&row["candidate"], "f4_field_ops") as f64
                            })
                            .sum();
                        let rho: f64 = subset
                            .iter()
                            .map(|row| price(&row["rho"]["counts"], s, i, native_rho(row), weight))
                            .sum();
                        let table: f64 = subset
                            .iter()
                            .map(|row| {
                                let key = (
                                    u(row, "p"),
                                    u(row, "a"),
                                    u(row, "b"),
                                    u(row, "target_index"),
                                    row["arm"].as_str().unwrap().to_string(),
                                );
                                let matched = old_by_key[&key];
                                price(
                                    &matched["candidate"]["total_counts"],
                                    s,
                                    i,
                                    native_table(matched),
                                    weight,
                                )
                            })
                            .sum();
                        let ratio = candidate / rho;
                        ratios.push(ratio);
                        let table_ratio = candidate / table;
                        table_ratios.push(table_ratio);
                        let value = json!({"kind":kind,"label":label,"arm":arm,"targets":subset.len(),
                            "S_per_M":s,"I_per_M":i,"native_M_per_logical_op":weight,
                            "candidate_M":candidate,"rho_M":rho,"pair_table_M":table,"candidate_over_rho":ratio,"candidate_over_pair_table":table_ratio});
                        grid_rows.push(value.clone());
                        if s == 0.5 && i == 10.0 && weight == 0.1 {
                            representative = Some(value);
                        }
                        grid.push_str(&format!(
                            "{kind},{label},{arm},{},{s},{i},{weight},{candidate},{rho},{table},{ratio},{table_ratio}\n",
                            subset.len()
                        ));
                    }
                }
            }
            summary.push(json!({"kind":kind,"label":label,"arm":arm,"targets":subset.len(),
                "candidate_verified":subset.len(),"rho_verified":subset.len(),"failures":0,"exhaustions":0,
                "representative":representative.unwrap(),
                "ratio_band":[ratios.iter().cloned().fold(f64::INFINITY,f64::min),ratios.iter().cloned().fold(f64::NEG_INFINITY,f64::max)],
                "pair_table_ratio_band":[table_ratios.iter().cloned().fold(f64::INFINITY,f64::min),table_ratios.iter().cloned().fold(f64::NEG_INFINITY,f64::max)],
                "pair_table_win_cells":table_ratios.iter().filter(|&&ratio|ratio<1.0).count(),
                "pair_table_loss_cells":table_ratios.iter().filter(|&&ratio|ratio>1.0).count(),
                "rho_win_cells":ratios.iter().filter(|&&ratio|ratio<1.0).count(),
                "rho_loss_cells":ratios.iter().filter(|&&ratio|ratio>1.0).count(),
                "candidate_trials":subset.iter().map(|v|u(&v["candidate"],"trials")).sum::<u64>(),
                "relations":subset.iter().map(|v|u(&v["candidate"],"relations")).sum::<u64>(),
                "selection_M":subset.iter().map(|v|u(&v["selection_counts"],"m")).sum::<u64>(),
                "f4_field_ops":subset.iter().map(|v|u(&v["candidate"],"f4_field_ops")).sum::<u64>(),
                "f4_ms_unqualified":subset.iter().map(|v|v["candidate"]["f4_ms_unqualified"].as_f64().unwrap()).sum::<f64>(),
                "polynomial_roots":subset.iter().map(|v|u(&v["candidate"],"polynomial_roots")).sum::<u64>(),
                "sign_lift_checks":subset.iter().map(|v|u(&v["candidate"],"sign_lift_checks")).sum::<u64>()}));
        }
    }
    let mut cross_arm = Vec::new();
    for (kind, label) in &slices {
        for (numerator, denominator) in [("mixed", "homogeneous"), ("mixed", "random")] {
            let mut ratios = Vec::new();
            let mut representative = None;
            for s in [0.0, 0.5, 1.0] {
                for i in [1.0, 10.0, 100.0, 1000.0] {
                    for weight in [0.0, 0.1, 1.0] {
                        let lookup = |arm: &str| -> f64 {
                            grid_rows
                                .iter()
                                .find(|row| {
                                    row["kind"] == *kind
                                        && row["label"] == *label
                                        && row["arm"] == arm
                                        && row["S_per_M"] == s
                                        && row["I_per_M"] == i
                                        && row["native_M_per_logical_op"] == weight
                                })
                                .unwrap()["candidate_M"]
                                .as_f64()
                                .unwrap()
                        };
                        let ratio = lookup(numerator) / lookup(denominator);
                        if s == 0.5 && i == 10.0 && weight == 0.1 {
                            representative = Some(ratio);
                        }
                        ratios.push(ratio);
                    }
                }
            }
            cross_arm.push(json!({"kind":kind,"label":label,"numerator":numerator,"denominator":denominator,
                "representative":representative.unwrap(),"ratio_band":[ratios.iter().copied().fold(f64::INFINITY,f64::min),ratios.iter().copied().fold(f64::NEG_INFINITY,f64::max)],
                "numerator_win_cells":ratios.iter().filter(|&&r|r<1.0).count(),"numerator_loss_cells":ratios.iter().filter(|&&r|r>1.0).count()}));
        }
    }
    fs::create_dir_all(format!("{BASE}/results")).unwrap();
    fs::write(
        format!("{BASE}/results/mixed-fiber-f4-dlp-lower-bound-price-grid.csv"),
        grid,
    )
    .unwrap();
    let output = json!({"schema":"mixed-fiber-f4-dlp-analysis/v1","rows":rows.len(),"targets":targets.len(),"curves":curves.len(),"summary":summary,"cross_arm":cross_arm,
        "relation_trials":relation_trials,"identity_right_hand_sides":identity_trials,"f4_systems":relation_trials-identity_trials,"target_coordinate_f4_systems":target_gate.len(),
        "solver":"native_degree_bounded_F4_all_roots_with_sign_lifts","external_f4_comparison":false,
        "price_scope":"lower bound: F4 row-reduction field multiplications plus curve field M/S/I and declared native events; symbolic preprocessing and substitution field operations uncounted"});
    fs::write(
        format!("{BASE}/results/mixed-fiber-f4-dlp-lower-bound-summary.json"),
        serde_json::to_vec_pretty(&output).unwrap(),
    )
    .unwrap();
    println!(
        "{}",
        serde_json::to_string_pretty(&output["summary"][0]).unwrap()
    );
}
