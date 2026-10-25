//! Native reanalysis of the retained 1,260-target two-choice collector cohort.
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;

const BASE: &str = "research/five_leads_e2e_20261019";
const BITS: [u32; 7] = [16, 20, 24, 28, 32, 36, 40];
const ARMS: [&str; 5] = [
    "rho_neg",
    "rho_plain",
    "jac_rho",
    "genus_ic",
    "genus_choice2",
];

fn read_json(path: &str) -> Value {
    serde_json::from_slice(&fs::read(path).expect("read JSON")).expect("parse JSON")
}
fn read_records(path: &str) -> Vec<Value> {
    fs::read_to_string(path)
        .expect("read records")
        .lines()
        .map(|line| serde_json::from_str(line).expect("parse record"))
        .collect()
}
fn counter(v: &Value, key: &str) -> f64 {
    v["counters"][key].as_u64().expect("counter") as f64
}
fn price(v: &Value, s: f64, i: f64, native: f64) -> f64 {
    let source_unit = counter(v, "source_add_multiplications")
        + s * counter(v, "source_add_squarings")
        + i * counter(v, "source_add_inversions");
    assert!(source_unit > 0.0);
    let arithmetic = counter(v, "field_multiplications")
        + s * counter(v, "field_squarings")
        + i * counter(v, "field_inversions");
    let logical: u64 = v["counters"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|(key, _)| key.ends_with("_uncharged"))
        .map(|(_, value)| value.as_u64().unwrap())
        .sum();
    arithmetic / source_unit + native * logical as f64
}
fn main() {
    let mut groups: BTreeMap<(u32, String, u64), BTreeMap<String, Value>> = BTreeMap::new();
    let mut sessions = Vec::new();
    let mut replays = 0;
    for bits in BITS {
        let root = format!("{BASE}/sessions/order-genus-{bits}");
        let session = read_json(&format!("{root}/session.json"));
        let audit = read_json(&format!("{BASE}/replay/order-genus-{bits}.json"));
        assert_eq!(session["status"], "complete");
        assert_eq!(session["records_written"], 900);
        assert_eq!(audit["ok"], true);
        assert_eq!(audit["checks"]["replays"], true);
        assert_eq!(audit["session_id"], session["session_id"]);
        assert_eq!(audit["verified_records"], 900);
        let replay = audit["replays"].as_array().unwrap();
        assert_eq!(replay.len(), 900);
        assert!(replay.iter().all(|v| v["reproduced"] == true));
        replays += replay.len();
        sessions.push(json!({"bits":bits,"session_id":session["session_id"],"records_sha256":session["records_sha256"]}));
        let records = read_records(&format!("{root}/records.jsonl"));
        assert_eq!(records.len(), 900);
        for row in records {
            assert_eq!(row["warmup"], false);
            assert_eq!(row["outcome"]["status"], "verified");
            assert_eq!(row["outcome"]["matches_target"], true);
            assert_eq!(row["outcome"]["matches_planted"], true);
            let arm = row["arm"].as_str().unwrap().to_owned();
            assert!(ARMS.contains(&arm.as_str()));
            let key = (
                bits,
                row["workload"]["curve"]["slug"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                row["workload"]["target_index"].as_u64().unwrap(),
            );
            assert!(groups.entry(key).or_default().insert(arm, row).is_none());
        }
    }
    assert_eq!(replays, 6300);
    assert_eq!(groups.len(), 1260);
    let mut curves: BTreeMap<(u32, String), BTreeSet<u64>> = BTreeMap::new();
    for ((bits, slug, index), arms) in &groups {
        assert_eq!(arms.len(), 5);
        let ids: BTreeSet<&str> = arms
            .values()
            .map(|v| v["workload"]["workload_id"].as_str().unwrap())
            .collect();
        assert_eq!(ids.len(), 1);
        curves
            .entry((*bits, slug.clone()))
            .or_default()
            .insert(*index);
    }
    assert_eq!(curves.len(), 21);
    assert!(curves
        .values()
        .all(|indices| indices.len() == 60 && (0..60).all(|j| indices.contains(&j))));
    let mut slices: Vec<(String, String)> = vec![("all".into(), "all".into())];
    slices.extend(BITS.map(|bits| ("bits".into(), bits.to_string())));
    slices.extend(
        curves
            .keys()
            .map(|(bits, slug)| ("curve".into(), format!("{bits}:{slug}"))),
    );
    let mut grid=String::from("kind,label,targets,S_per_M,I_per_M,native_GAE_per_logical_operation,choice2_GAE,incumbent_GAE,rho_neg_GAE,choice2_over_incumbent,choice2_over_rho\n");
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
        let mut incumbent_ratios = Vec::new();
        let mut rho_ratios = Vec::new();
        let mut representative = None;
        for s in [0.0, 0.5, 1.0] {
            for i in [1.0, 10.0, 100.0, 1000.0] {
                for native in [0.0, 0.1, 1.0] {
                    let sums: Vec<f64> = ["genus_choice2", "genus_ic", "rho_neg"]
                        .iter()
                        .map(|arm| {
                            selected
                                .iter()
                                .map(|arms| price(&arms[*arm], s, i, native))
                                .sum()
                        })
                        .collect();
                    let incumbent = sums[0] / sums[1];
                    let rho = sums[0] / sums[2];
                    incumbent_ratios.push(incumbent);
                    rho_ratios.push(rho);
                    let row = json!({"kind":kind,"label":label,"targets":selected.len(),"S_per_M":s,"I_per_M":i,
                        "native_GAE_per_logical_operation":native,"choice2_GAE":sums[0],"incumbent_GAE":sums[1],
                        "rho_neg_GAE":sums[2],"choice2_over_incumbent":incumbent,"choice2_over_rho":rho});
                    if s == 0.5 && i == 10.0 && native == 0.1 {
                        representative = Some(row);
                    }
                    grid.push_str(&format!(
                        "{kind},{label},{},{s},{i},{native},{},{},{},{incumbent},{rho}\n",
                        selected.len(),
                        sums[0],
                        sums[1],
                        sums[2]
                    ));
                }
            }
        }
        let divisor_total = |arm: &str| -> u64 {
            selected
                .iter()
                .map(|arms| {
                    arms[arm]["counters"]["processed_divisors"]
                        .as_u64()
                        .unwrap()
                })
                .sum()
        };
        summary.push(json!({"kind":kind,"label":label,"targets":selected.len(),"all_verified":true,"failures":0,"exhaustions":0,
            "representative":representative.unwrap(),
            "choice2_over_incumbent_band":[incumbent_ratios.iter().cloned().fold(f64::INFINITY,f64::min),incumbent_ratios.iter().cloned().fold(f64::NEG_INFINITY,f64::max)],
            "choice2_over_rho_band":[rho_ratios.iter().cloned().fold(f64::INFINITY,f64::min),rho_ratios.iter().cloned().fold(f64::NEG_INFINITY,f64::max)],
            "incumbent_win_cells":incumbent_ratios.iter().filter(|&&v|v<1.0).count(),
            "rho_win_cells":rho_ratios.iter().filter(|&&v|v<1.0).count(),
            "choice2_processed_divisors":divisor_total("genus_choice2"),
            "incumbent_processed_divisors":divisor_total("genus_ic")}));
    }
    fs::create_dir_all(format!("{BASE}/results")).unwrap();
    fs::write(
        format!("{BASE}/results/genus2-choice2-native-v3-price-grid.csv"),
        grid,
    )
    .unwrap();
    let output = json!({"schema":"five-leads-choice2-analysis/native-v3","sessions":sessions,"records":6300,
        "replays_reproduced":replays,"paired_targets":groups.len(),"summary":summary});
    fs::write(
        format!("{BASE}/results/genus2-choice2-native-v3-summary.json"),
        serde_json::to_vec_pretty(&output).unwrap(),
    )
    .unwrap();
    println!(
        "{}",
        serde_json::to_string_pretty(&output["summary"][0]).unwrap()
    );
}
