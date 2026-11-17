//! Independent 36-cell recomputation from raw collector and external rows.
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;

const ROOT: &str = "research/projective_law_followup_20261022/results";
const BITS: [u64; 7] = [16, 20, 24, 28, 32, 36, 40];

#[derive(Default)]
struct PhaseTotals {
    targets: u64,
    m: u64,
    s: u64,
    i: u64,
    field_adds: u64,
    bit_ops: u64,
    ec_adds: u64,
    jac_adds: u64,
    logical_ops: u64,
    gae: f64,
}

fn number(v: &Value, name: &str) -> f64 {
    v[name].as_u64().unwrap() as f64
}
fn logic(v: &Value) -> f64 {
    v.as_object()
        .unwrap()
        .iter()
        .filter(|(k, _)| k.ends_with("_uncharged"))
        .map(|(_, v)| v.as_u64().unwrap() as f64)
        .sum()
}
fn cost(v: &Value, s: f64, i: f64, native: f64) -> f64 {
    let c = &v["counts"];
    let u = &v["source_add"];
    let numerator = number(c, "m") + s * number(c, "s") + i * number(c, "i");
    let denominator = number(u, "m") + s * number(u, "s") + i * number(u, "i");
    assert!(denominator > 0.0);
    numerator / denominator
        + native * (number(c, "field_adds") + number(c, "bit_ops") + logic(&v["native"]))
}
fn read_jsonl(path: &str) -> Vec<Value> {
    fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("{path}: {e}"))
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}
fn near(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= 2e-10 * a.abs().max(b.abs()).max(1.0),
        "{a} != {b}"
    );
}
fn cold_phase_rows(csv: &mut String, arm: &str, raw: &[(u64, Value, Value)]) {
    for slice in std::iter::once(None).chain(BITS.map(Some)) {
        let label = slice.map_or_else(|| "all".to_owned(), |bits| bits.to_string());
        let selected: Vec<_> = raw
            .iter()
            .filter(|(bits, _, _)| slice.is_none_or(|want| want == *bits))
            .collect();
        let mut totals: BTreeMap<(String, String), PhaseTotals> = BTreeMap::new();
        for (_, candidate, external) in &selected {
            for (method, record) in [
                ("candidate", candidate),
                ("external", external),
                ("rho", &candidate["rho"]),
            ] {
                let u = &record["source_add"];
                let unit = number(u, "m") + 0.5 * number(u, "s") + 10.0 * number(u, "i");
                for phase in record["phases"].as_array().unwrap() {
                    let name = phase["name"].as_str().unwrap();
                    let c = &phase["counts"];
                    let row = totals
                        .entry((method.to_owned(), name.to_owned()))
                        .or_default();
                    row.targets += 1;
                    row.m += c["m"].as_u64().unwrap();
                    row.s += c["s"].as_u64().unwrap();
                    row.i += c["i"].as_u64().unwrap();
                    row.field_adds += c["field_adds"].as_u64().unwrap();
                    row.bit_ops += c["bit_ops"].as_u64().unwrap();
                    row.ec_adds += c["ec_adds"].as_u64().unwrap();
                    row.jac_adds += c["jac_adds"].as_u64().unwrap();
                    row.gae += (number(c, "m") + 0.5 * number(c, "s") + 10.0 * number(c, "i"))
                        / unit
                        + 0.1 * (number(c, "field_adds") + number(c, "bit_ops"));
                }
                let logical = logic(&record["native"]);
                let row = totals
                    .entry((method.to_owned(), "native_logical_unallocated".to_owned()))
                    .or_default();
                row.targets += 1;
                row.logical_ops += logical as u64;
                row.gae += 0.1 * logical;
            }
        }
        for method in ["candidate", "external", "rho"] {
            let from_phases: f64 = totals
                .iter()
                .filter(|((m, _), _)| m == method)
                .map(|(_, value)| value.gae)
                .sum();
            let from_raw: f64 = selected
                .iter()
                .map(|(_, candidate, external)| {
                    let record = match method {
                        "candidate" => candidate,
                        "external" => external,
                        _ => &candidate["rho"],
                    };
                    cost(record, 0.5, 10.0, 0.1)
                })
                .sum();
            near(from_phases, from_raw);
        }
        for ((method, phase), row) in totals {
            csv.push_str(&format!(
                "{arm},{label},{method},{phase},{},{},{},{},{},{},{},{},{},{:.12}\n",
                row.targets,
                row.m,
                row.s,
                row.i,
                row.field_adds,
                row.bit_ops,
                row.ec_adds,
                row.jac_adds,
                row.logical_ops,
                row.gae
            ));
        }
    }
}
fn main() {
    let mut arms = Vec::new();
    let mut grid_cells = 0u64;
    let mut candidate_grid = Vec::new();
    let mut phase_csv = String::from("study_arm,slice,method,phase,targets,m,s,i,field_adds,bit_ops,ec_adds,jac_adds,native_logical_ops,GAE_at_M_plus_0.5S_plus_10I_and_0.1_native\n");
    for arm in ["projective", "state"] {
        let mut raw = Vec::new();
        for bits in BITS {
            let fresh = read_jsonl(&format!("{ROOT}/{arm}-{bits}.jsonl"));
            let external: BTreeMap<String, Value> = read_jsonl(&format!(
                "research/genus2_external_gate_20261021/results/order-genus-{bits}.jsonl"
            ))
            .into_iter()
            .map(|v| (v["workload_id"].as_str().unwrap().to_owned(), v))
            .collect();
            assert_eq!(fresh.len(), 180);
            for v in fresh {
                let id = v["workload_id"].as_str().unwrap();
                let e = external[id].clone();
                assert_eq!(v["verified"], true);
                assert_eq!(v["rho"]["verified"], true);
                assert_eq!(v["recovered"], v["planted"]);
                assert_eq!(v["rho"]["recovered"], v["planted"]);
                for record in [&v, &v["rho"]] {
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
                        let phases: u64 = record["phases"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|phase| phase["counts"][key].as_u64().unwrap())
                            .sum();
                        assert_eq!(
                            phases,
                            record["counts"][key].as_u64().unwrap(),
                            "phase total {arm} {bits} {id} {key}"
                        );
                    }
                }
                raw.push((bits, v, e));
            }
        }
        assert_eq!(raw.len(), 1260);
        cold_phase_rows(&mut phase_csv, arm, &raw);
        let grid_path = format!("{ROOT}/price-grid-{arm}.csv");
        let csv = fs::read_to_string(&grid_path).unwrap();
        let mut lines = csv.lines();
        let header = lines.next().unwrap();
        assert!(header.starts_with("slice,targets,verified,"));
        let mut arm_cells = 0u64;
        let mut arm_grid = BTreeMap::new();
        for line in lines {
            let parts: Vec<_> = line.split(',').collect();
            assert_eq!(parts.len(), 15, "{line}");
            let label = parts[0];
            let selected: Vec<_> = raw
                .iter()
                .filter(|(bits, _, _)| label == "all" || label.parse::<u64>().unwrap() == *bits)
                .collect();
            assert_eq!(parts[1].parse::<usize>().unwrap(), selected.len());
            assert_eq!(parts[2].parse::<usize>().unwrap(), selected.len());
            assert_eq!(parts[3], "0");
            assert_eq!(parts[4], "0");
            let s = parts[5].parse::<f64>().unwrap();
            let i = parts[6].parse::<f64>().unwrap();
            let native = parts[7].parse::<f64>().unwrap();
            assert!([0.0, 0.5, 1.0].contains(&s));
            assert!([1.0, 10.0, 100.0, 1000.0].contains(&i));
            assert!([0.0, 0.1, 1.0].contains(&native));
            let candidate: f64 = selected.iter().map(|(_, v, _)| cost(v, s, i, native)).sum();
            let external: f64 = selected.iter().map(|(_, _, v)| cost(v, s, i, native)).sum();
            let rho: f64 = selected
                .iter()
                .map(|(_, v, _)| cost(&v["rho"], s, i, native))
                .sum();
            for (column, recomputed) in [
                (8, candidate),
                (9, external),
                (10, rho),
                (11, candidate / external),
                (12, candidate / rho),
            ] {
                near(parts[column].parse::<f64>().unwrap(), recomputed);
            }
            let normalization: u64 = selected
                .iter()
                .map(|(_, v, _)| {
                    v["native"]["weighted_normalization_inversions"]
                        .as_u64()
                        .unwrap()
                })
                .sum();
            let processed: u64 = selected
                .iter()
                .map(|(_, v, _)| v["native"]["processed_divisors"].as_u64().unwrap())
                .sum();
            assert_eq!(parts[13].parse::<u64>().unwrap(), normalization);
            assert_eq!(parts[14].parse::<u64>().unwrap(), processed);
            let key = format!("{label},{s},{i},{native}");
            assert!(arm_grid.insert(key, candidate).is_none());
            arm_cells += 1;
        }
        assert_eq!(arm_cells, 288);
        candidate_grid.push(arm_grid);
        grid_cells += arm_cells;
        let summary: Value = serde_json::from_str(
            &fs::read_to_string(format!("{ROOT}/summary-{arm}.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(summary["targets"], 1260);
        assert_eq!(summary["independent_source_scalar_replays"], 2520);
        let all = summary["summaries"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["slice"] == "all")
            .unwrap();
        let candidate: f64 = raw.iter().map(|(_, v, _)| cost(v, 0.5, 10.0, 0.1)).sum();
        let external: f64 = raw.iter().map(|(_, _, v)| cost(v, 0.5, 10.0, 0.1)).sum();
        let rho: f64 = raw
            .iter()
            .map(|(_, v, _)| cost(&v["rho"], 0.5, 10.0, 0.1))
            .sum();
        near(all["candidate_GAE"].as_f64().unwrap(), candidate);
        near(all["external_GAE"].as_f64().unwrap(), external);
        near(all["rho_GAE"].as_f64().unwrap(), rho);
        arms.push(json!({"arm":arm,"targets":1260,"grid_cells":arm_cells,
            "candidate_GAE":candidate,"external_GAE":external,"rho_GAE":rho,
            "candidate_over_external":candidate/external,"candidate_over_rho":candidate/rho}));
    }
    assert_eq!(grid_cells, 576);
    fs::write(format!("{ROOT}/cold-phase-breakdown.csv"), phase_csv).unwrap();
    let mut paired_grid = String::from(
        "slice,S_per_M,I_per_M,native_GAE_per_logical,wrapper_GAE,state_GAE,state_over_wrapper\n",
    );
    let mut pooled_min = f64::INFINITY;
    let mut pooled_max: f64 = 0.0;
    let mut pooled_representative = None;
    for (key, &wrapper) in &candidate_grid[0] {
        let state = candidate_grid[1][key];
        let ratio = state / wrapper;
        if key.starts_with("all,") {
            pooled_min = pooled_min.min(ratio);
            pooled_max = pooled_max.max(ratio);
            if key == "all,0.5,10,0.1" {
                pooled_representative = Some(ratio);
            }
        }
        paired_grid.push_str(&format!("{key},{wrapper:.12},{state:.12},{ratio:.12}\n"));
    }
    fs::write(
        format!("{ROOT}/price-grid-state-vs-wrapper.csv"),
        paired_grid,
    )
    .unwrap();
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schema":"projective-law-independent-validator/v1","status":"pass",
            "method":"separate native binary recomputes every 36-cell price from raw rows",
            "grid_cells":grid_cells,"arms":arms,
            "paired_state_over_wrapper":{
                "representative":pooled_representative.unwrap(),
                "pooled_36_cell_range":[pooled_min,pooled_max],
                "grid":"results/price-grid-state-vs-wrapper.csv"
            }
        }))
        .unwrap()
    );
}
