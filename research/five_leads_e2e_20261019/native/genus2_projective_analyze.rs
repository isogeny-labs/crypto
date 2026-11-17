//! Native audit and 12x3 operation-price grid for the frozen projective cohort.
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

const BASE: &str = "research/five_leads_e2e_20261019";
const BITS: [u32; 7] = [16, 20, 24, 28, 32, 36, 40];
const ARMS: [&str; 3] = ["rho_neg", "genus_choice2", "genus_projective"];

fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).expect("read JSON")).expect("parse JSON")
}
fn records(path: impl AsRef<Path>) -> Vec<Value> {
    fs::read_to_string(path)
        .expect("read records")
        .lines()
        .map(|line| serde_json::from_str(line).expect("parse record"))
        .collect()
}
fn number(v: &Value, key: &str) -> f64 {
    v["counters"][key].as_u64().expect("counter") as f64
}
fn price(v: &Value, s: f64, i: f64, native: f64) -> f64 {
    let unit = number(v, "source_add_multiplications")
        + s * number(v, "source_add_squarings")
        + i * number(v, "source_add_inversions");
    assert!(unit > 0.0);
    let arithmetic = number(v, "field_multiplications")
        + s * number(v, "field_squarings")
        + i * number(v, "field_inversions");
    let logical: u64 = v["counters"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|(key, _)| key.ends_with("_uncharged"))
        .map(|(_, value)| value.as_u64().unwrap())
        .sum();
    arithmetic / unit + native * logical as f64
}
fn quote(s: &str) -> String {
    format!("\"{}\"", s.replace('"', "\"\""))
}
fn main() {
    let mut groups: BTreeMap<(u32, String, u64), BTreeMap<String, Value>> = BTreeMap::new();
    let mut sessions = Vec::new();
    let mut record_count = 0;
    let mut replay_count = 0;
    for bits in BITS {
        let fresh = format!("{BASE}/sessions_projective_chart_v2/order-genus-{bits}");
        let old = format!("{BASE}/sessions/order-genus-{bits}");
        let session = read_json(format!("{fresh}/session.json"));
        assert_eq!(session["status"], "complete");
        assert_eq!(session["records_written"], 540);
        let audit = read_json(format!(
            "{BASE}/replay_projective_chart_v2/order-genus-{bits}.json"
        ));
        assert_eq!(audit["ok"], true);
        assert_eq!(audit["checks"]["replays"], true);
        assert_eq!(audit["session_id"], session["session_id"]);
        assert_eq!(audit["verified_records"], 540);
        let replays = audit["replays"].as_array().unwrap();
        assert_eq!(replays.len(), 540);
        assert!(replays.iter().all(|v| v["reproduced"] == true));
        replay_count += replays.len();
        sessions.push(json!({"bits":bits,"session_id":session["session_id"],"records_sha256":session["records_sha256"]}));
        let old_rows = records(format!("{old}/records.jsonl"));
        let fresh_rows = records(format!("{fresh}/records.jsonl"));
        assert_eq!(old_rows.len(), 900);
        assert_eq!(fresh_rows.len(), 540);
        let old_index: BTreeMap<(String, String), Value> = old_rows
            .into_iter()
            .map(|v| {
                let key = (
                    v["workload"]["workload_id"].as_str().unwrap().to_owned(),
                    v["arm"].as_str().unwrap().to_owned(),
                );
                (key, v)
            })
            .collect();
        for row in fresh_rows {
            assert_eq!(row["outcome"]["status"], "verified");
            assert_eq!(row["outcome"]["matches_target"], true);
            assert_eq!(row["outcome"]["matches_planted"], true);
            assert_eq!(row["warmup"], false);
            let arm = row["arm"].as_str().unwrap().to_owned();
            assert!(ARMS.contains(&arm.as_str()));
            let wid = row["workload"]["workload_id"].as_str().unwrap();
            if arm != "genus_projective" {
                let prior = &old_index[&(wid.to_owned(), arm.clone())];
                assert_eq!(row["algorithm_seed"], prior["algorithm_seed"]);
                assert_eq!(row["workload"], prior["workload"]);
                assert_eq!(row["outcome"]["recovered"], prior["outcome"]["recovered"]);
                assert_eq!(row["counters"], prior["counters"]);
            }
            let key = (
                bits,
                row["workload"]["curve"]["slug"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                row["workload"]["target_index"].as_u64().unwrap(),
            );
            assert!(groups.entry(key).or_default().insert(arm, row).is_none());
            record_count += 1;
        }
    }
    assert_eq!(record_count, 3780);
    assert_eq!(replay_count, 3780);
    assert_eq!(groups.len(), 1260);
    let mut curves: BTreeMap<(u32, String), BTreeSet<u64>> = BTreeMap::new();
    for ((bits, slug, index), arms) in &groups {
        assert_eq!(arms.len(), 3);
        let ids: BTreeSet<&str> = arms
            .values()
            .map(|r| r["workload"]["workload_id"].as_str().unwrap())
            .collect();
        assert_eq!(ids.len(), 1);
        let p = &arms["genus_projective"]["counters"];
        let a = &arms["genus_choice2"]["counters"];
        assert_eq!(p["processed_divisors"], a["processed_divisors"]);
        curves
            .entry((*bits, slug.clone()))
            .or_default()
            .insert(*index);
    }
    assert_eq!(curves.len(), 21);
    assert!(curves
        .values()
        .all(|v| v.len() == 60 && (0..60).all(|j| v.contains(&j))));
    let mut slices: Vec<(String, String)> = vec![("all".into(), "all".into())];
    slices.extend(BITS.map(|bits| ("bits".into(), bits.to_string())));
    slices.extend(
        curves
            .keys()
            .map(|(bits, slug)| ("curve".into(), format!("{bits}:{slug}"))),
    );
    let mut grid = String::from("kind,label,targets,S_per_M,I_per_M,native_GAE_per_logical_operation,rho_neg_GAE,genus_choice2_GAE,genus_projective_GAE,projective_over_affine,projective_over_rho\n");
    let mut summary = Vec::new();
    for (kind, label) in slices {
        let selected: Vec<_> = groups
            .iter()
            .filter(|((bits, slug, _), _)| match kind.as_str() {
                "all" => true,
                "bits" => bits.to_string() == label,
                "curve" => format!("{bits}:{slug}") == label,
                _ => unreachable!(),
            })
            .map(|(_, arms)| arms)
            .collect();
        let mut ratios_affine = Vec::new();
        let mut ratios_rho = Vec::new();
        let mut representative = None;
        for s in [0.0, 0.5, 1.0] {
            for i in [1.0, 10.0, 100.0, 1000.0] {
                for native in [0.0, 0.1, 1.0] {
                    let sums: Vec<f64> = ARMS
                        .iter()
                        .map(|arm| {
                            selected
                                .iter()
                                .map(|arms| price(&arms[*arm], s, i, native))
                                .sum()
                        })
                        .collect();
                    let affine = sums[2] / sums[1];
                    let rho = sums[2] / sums[0];
                    ratios_affine.push(affine);
                    ratios_rho.push(rho);
                    let row = json!({"kind":kind,"label":label,"targets":selected.len(),"S_per_M":s,"I_per_M":i,
                        "native_GAE_per_logical_operation":native,"rho_neg_GAE":sums[0],"genus_choice2_GAE":sums[1],
                        "genus_projective_GAE":sums[2],"projective_over_affine":affine,"projective_over_rho":rho});
                    if s == 0.5 && i == 10.0 && native == 0.1 {
                        representative = Some(row);
                    }
                    grid.push_str(&format!(
                        "{},{},{},{},{},{},{},{},{},{},{}\n",
                        kind,
                        quote(&label),
                        selected.len(),
                        s,
                        i,
                        native,
                        sums[0],
                        sums[1],
                        sums[2],
                        affine,
                        rho
                    ));
                }
            }
        }
        let projective: Vec<_> = selected
            .iter()
            .map(|arms| &arms["genus_projective"]["counters"])
            .collect();
        let counter_sum = |name: &str| -> u64 {
            projective
                .iter()
                .map(|c| c[name].as_u64().unwrap_or(0))
                .sum()
        };
        summary.push(json!({"kind":kind,"label":label,"targets":selected.len(),"all_verified":true,"failures":0,"exhaustions":0,
            "representative":representative.unwrap(),
            "projective_over_affine_band":[ratios_affine.iter().cloned().fold(f64::INFINITY,f64::min),ratios_affine.iter().cloned().fold(f64::NEG_INFINITY,f64::max)],
            "projective_over_rho_band":[ratios_rho.iter().cloned().fold(f64::INFINITY,f64::min),ratios_rho.iter().cloned().fold(f64::NEG_INFINITY,f64::max)],
            "affine_win_cells":ratios_affine.iter().filter(|&&ratio|ratio<1.0).count(),
            "rho_win_cells":ratios_rho.iter().filter(|&&ratio|ratio<1.0).count(),
            "ordinary_chart_additions":counter_sum("projective_chart_additions"),
            "exceptional_fallbacks":counter_sum("projective_exceptional_fallbacks"),
            "processed_divisors":counter_sum("processed_divisors")}));
    }
    let result_dir = format!("{BASE}/results");
    fs::create_dir_all(&result_dir).unwrap();
    fs::write(
        format!("{result_dir}/genus2-projective-price-grid.csv"),
        grid,
    )
    .unwrap();
    let output = json!({"schema":"five-leads-projective-analysis/v2","sessions":sessions,"records":record_count,
        "replays_reproduced":replay_count,
        "paired_targets":groups.len(),"summary":summary});
    fs::write(
        format!("{result_dir}/genus2-projective-summary.json"),
        serde_json::to_vec_pretty(&output).unwrap(),
    )
    .unwrap();
    println!(
        "{}",
        serde_json::to_string_pretty(&output["summary"][0]).unwrap()
    );
}
