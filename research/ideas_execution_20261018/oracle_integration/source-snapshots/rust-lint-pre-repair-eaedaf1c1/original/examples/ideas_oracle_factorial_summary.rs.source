//! Descriptive native analysis of the frozen seven-arm diagnostic.
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;

#[derive(Default)]
struct Cell {
    records: usize,
    correct: usize,
    statuses: BTreeMap<String, usize>,
    gae: f64,
    unpriced: f64,
    counts: BTreeMap<String, u64>,
    missing: BTreeSet<String>,
}

fn fingerprint(row: &Value) -> Value {
    json!({"algorithm_seed":row["algorithm_seed"],"cost":row["cost"],
        "counters":row["counters"],"detail":row["detail"],
        "recovered":row["outcome"]["recovered"],
        "matches_target":row["outcome"]["matches_target"],
        "matches_planted":row["outcome"]["matches_planted"]})
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let directory = args.get(1).expect("session directory");
    let output = args.get(2).expect("fresh output JSON");
    let mut writer = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)
        .unwrap();
    let text = fs::read_to_string(format!("{directory}/records.jsonl")).unwrap();
    let rows: Vec<Value> = text
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    let mut cells = BTreeMap::<(u64, String), Cell>::new();
    let mut aa = BTreeMap::<String, BTreeMap<String, Value>>::new();
    let mut registered = BTreeSet::new();
    let mut sessions = BTreeSet::new();
    for row in &rows {
        assert_eq!(row["schema"], "ecbench.record/v1");
        let p = row["workload"]["curve_spec"]["p"].as_u64().unwrap();
        let arm = row["arm"].as_str().unwrap();
        let cell = cells.entry((p, arm.to_string())).or_default();
        cell.records += 1;
        let correct = row["outcome"]["matches_target"] == Value::Bool(true)
            && row["outcome"]["matches_planted"] == Value::Bool(true);
        cell.correct += usize::from(correct);
        *cell
            .statuses
            .entry(row["outcome"]["status"].as_str().unwrap().into())
            .or_default() += 1;
        cell.gae += row["cost"]["total_gae"].as_f64().unwrap();
        for counter in row["cost"]["unpriced"].as_array().unwrap() {
            let name = counter.as_str().unwrap();
            if let Some(count) = row["counters"][name].as_u64() {
                *cell.counts.entry(name.into()).or_default() += count;
                cell.unpriced += count as f64;
            } else {
                cell.missing.insert(name.into());
            }
        }
        if row["workload"]["curve"]["registered"] != Value::Bool(true) {
            registered.insert(
                row["workload"]["curve"]["slug"]
                    .as_str()
                    .unwrap()
                    .to_string(),
            );
        }
        sessions.insert(row["session_id"].as_str().unwrap().to_string());
        if arm == "d4-only" || arm == "d4-aa" {
            aa.entry(row["workload"]["workload_id"].as_str().unwrap().into())
                .or_default()
                .insert(arm.into(), fingerprint(row));
        }
    }
    let aa_checked = aa.values().filter(|p| p.len() == 2).count();
    let aa_equal = aa
        .values()
        .filter(|p| p.len() == 2 && p["d4-only"] == p["d4-aa"])
        .count();
    let mut curves = BTreeMap::<u64, Vec<Value>>::new();
    let mut complete_cost_ratios = Vec::new();
    let arms = [
        "neither",
        "2lp-only",
        "d4-only",
        "d4-2lp",
        "d4-eager-incumbent",
        "d4-aa",
        "rho-negation",
    ];
    for (&(p, ref name), cell) in &cells {
        let charged: Vec<_> = [0.0, 0.1, 1.0]
            .iter()
            .map(|&price| {
                json!({"gae_per_declared_unpriced_op":price,
            "sum_attempted_gae":cell.gae+price*cell.unpriced,
            "mean_attempted_gae":(cell.gae+price*cell.unpriced)/cell.records as f64})
            })
            .collect();
        curves.entry(p).or_default().push(json!({"arm":name,"records":cell.records,"correct":cell.correct,
            "statuses":cell.statuses,"declared_unpriced_counts":cell.counts,
            "missing_declared_counters":cell.missing,"pricing_scenarios":charged,
            "cost_is_per_complete_solve":cell.records==cell.correct,
            "scope":"attempted-run charges; exhausted or failed arms cannot be ranked as completed solves"}));
    }
    for &p in curves.keys() {
        let incumbent = &cells[&(p, "d4-eager-incumbent".into())];
        let reference = &cells[&(p, "rho-negation".into())];
        for arm in arms {
            let cell = &cells[&(p, arm.into())];
            let eligible = cell.records == 8
                && cell.correct == 8
                && incumbent.correct == 8
                && reference.correct == 8
                && cell.missing.is_empty()
                && incumbent.missing.is_empty()
                && reference.missing.is_empty();
            complete_cost_ratios.push(json!({"p":p,"arm":arm,
                "all_targets_correct":cell.records==8 && cell.correct==8 && incumbent.correct==8 && reference.correct==8,
                "ratios_eligible":eligible,
                "ratios_if_all_targets_correct":if eligible {Some([0.0,0.1,1.0].iter().map(|&price|json!({
                    "gae_per_declared_unpriced_op":price,
                    "vs_eager_d4":(cell.gae+price*cell.unpriced)/(incumbent.gae+price*incumbent.unpriced),
                    "vs_rho":(cell.gae+price*cell.unpriced)/(reference.gae+price*reference.unpriced)})).collect::<Vec<_>>())}else{None},
                "admissible_full_cost_ranking":false,
                "reason":"declared-counter sensitivity only; enumeration oracle, unregistered curves and bookkeeping limit admission"}));
        }
    }
    let result = json!({"schema":"ideas-factorial-descriptive/v1","session_directory":directory,"sessions":sessions,
        "records":rows.len(),"requested_records":224,"correct":rows.iter().filter(|r|r["outcome"]["matches_target"]==Value::Bool(true)&&r["outcome"]["matches_planted"]==Value::Bool(true)).count(),
        "aa_paired_workloads":aa_checked,"aa_equal":aa_equal,"unregistered_slugs":registered,
        "by_curve":curves,"diagnostic_ratios":complete_cost_ratios,
        "pricing_scope":"Same0,0.1,1GAE band on every declared unpriced counter in every arm,includingrho; unknown non-emitted bookkeeping is outside this partial band",
        "performance_claim":"none; no wall-time admission or complete-cost ranking"});
    writer
        .write_all(&serde_json::to_vec_pretty(&result).unwrap())
        .unwrap();
    println!(
        "{}",
        json!({"records":rows.len(),"aa_checked":aa_checked,"aa_equal":aa_equal,"output":output})
    );
}
