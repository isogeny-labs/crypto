//! Post-run conformance check of stored frozen Rust summaries, no re-estimation.
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs, path::Path};
fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    assert!(
        args.len().is_multiple_of(2) && !args.is_empty(),
        "SUMMARY RAW_ROOT pairs"
    );
    let mut cohorts = vec![];
    for pair in args.as_chunks::<2>().0 {
        let summary = load(&pair[0]);
        let mut groups = vec![];
        let mut all_goals = true;
        let mut all_controls = true;
        for g in summary["groups"].as_array().unwrap() {
            let ratios = &g["paired_ratios"];
            let expected = g["arms"]["fiber-half"]["attempted"].as_u64().unwrap();
            let exact = |key: &str, want: f64| {
                let v = &ratios[key];
                v["n"].as_u64() == Some(expected)
                    && v["values"]
                        .as_array()
                        .is_some_and(|xs| xs.iter().all(|x| x.as_f64() == Some(want)))
            };
            let median = |key: &str| ratios[key]["median"].as_f64();
            let count_aa = exact("fiber-half/fiber-half-aa:query_muls", 1.0);
            let count_positive = exact("fiber-half-double/fiber-half:query_muls", 2.0);
            let clock_aa = median("fiber-half/fiber-half-aa:query_ns")
                .is_some_and(|x| (0.8..=1.25).contains(&x));
            let clock_positive = median("fiber-half-double/fiber-half:query_ns")
                .is_some_and(|x| (1.5..=2.5).contains(&x));
            let controls = count_aa && count_positive && clock_aa && clock_positive;
            let goal = &ratios["newton/fiber-half:cold_ns"];
            let complete = goal["n"].as_u64() == Some(expected);
            let observed = complete
                && goal["values"]
                    .as_array()
                    .is_some_and(|xs| xs.iter().all(|x| x.as_f64().is_some_and(|x| x >= 2.0)))
                && goal["bootstrap_median95"][0]
                    .as_f64()
                    .is_some_and(|x| x >= 2.0);
            all_controls &= controls;
            all_goals &= observed;
            groups.push(json!({"group":g["group"],"retained_targets":expected,"count_aa":count_aa,"count_positive":count_positive,"clock_aa":clock_aa,"clock_positive":clock_positive,"complete_reference_pairs":complete,"local_goal":observed&&controls,"verdict":if !controls{"invalid_measurement"}else if !complete{"inconclusive"}else if observed{"local_goal_observed_unverified"}else{"descriptive_only"}}));
        }
        let (mut checked, mut equal) = (0, true);
        for dir in fs::read_dir(&pair[1]).unwrap() {
            let dir = dir.unwrap().path();
            if !dir.is_dir() {
                continue;
            }
            let mut cells: BTreeMap<u64, BTreeMap<String, Value>> = BTreeMap::new();
            for entry in fs::read_dir(dir).unwrap() {
                let path = entry.unwrap().path();
                if path.extension().is_none_or(|x| x != "json")
                    || !path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .starts_with("cell-")
                {
                    continue;
                }
                let row = load(&path);
                if row["arm"] != "fiber-half" && row["arm"] != "lookup-fiber" {
                    continue;
                }
                cells
                    .entry(row["input_index"].as_u64().unwrap())
                    .or_default()
                    .insert(row["arm"].as_str().unwrap().into(), row);
            }
            for rows in cells.values() {
                checked += 1;
                equal &= rows
                    .get("fiber-half")
                    .zip(rows.get("lookup-fiber"))
                    .is_some_and(|(a, b)| {
                        a["status"] == "ok"
                            && b["status"] == "ok"
                            && a["construction_work"] == b["construction_work"]
                            && a["field_setup_inside_cold"] == true
                            && b["field_setup_inside_cold"] == true
                    });
            }
        }
        let failed = summary["failed"].as_u64().unwrap();
        cohorts.push(json!({"summary":pair[0],"raw_root":pair[1],"attempted":summary["attempted"],"successful":summary["successful"],"failed":failed,"groups":groups,"all_control_groups_pass":all_controls,"matched_setup_pairs_including_warmups":checked,"matched_setup_primitive_counts_equal":equal,"cohort_verdict":if !all_controls||!equal{"invalid_measurement"}else if failed>0{"inconclusive"}else if all_goals{"local_goal_observed_unverified"}else{"descriptive_only"}}));
    }
    println!(
        "{}",
        json!({"schema":"vieta-control-audit/v1","source_authored_after_data":true,"method":"Conformance checks of frozen stored native statistics and exact setup counters; no statistical re-estimation, no changed criteria","standing":"producer-owned, belowL2 and unverified; not an independent validator report","cohorts":cohorts})
    );
}
