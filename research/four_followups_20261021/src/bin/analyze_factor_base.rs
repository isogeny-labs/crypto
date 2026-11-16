//! Fail-closed, archive-backed pairing and operation-count pricing.
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;

const ROOT: &str = "research/four_followups_20261021/results";
const ARCHIVE: &str = "research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst";
const BITS: [u64; 7] = [16, 20, 24, 28, 32, 36, 40];
const ARMS: [(u64, u64); 5] = [(1, 16), (2, 16), (4, 16), (1, 8), (1, 32)];

fn rows(path: &Path) -> Vec<Value> {
    BufReader::new(File::open(path).unwrap_or_else(|e| panic!("{}: {e}", path.display())))
        .lines()
        .map(|line| serde_json::from_str(&line.unwrap()).unwrap())
        .collect()
}
fn archive_rows(bits: u64) -> Vec<Value> {
    let member = format!("sessions_projective_chart_v2/order-genus-{bits}/records.jsonl");
    let out = Command::new("tar")
        .args(["--zstd", "-xOf", ARCHIVE, &member])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "tar failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}
fn number(v: &Value, key: &str) -> f64 {
    v[key]
        .as_u64()
        .unwrap_or_else(|| panic!("missing numeric field {key}")) as f64
}
fn logical(v: &Value) -> f64 {
    v.as_object()
        .unwrap()
        .iter()
        .filter(|(k, _)| k.ends_with("_uncharged"))
        .map(|(_, x)| x.as_u64().unwrap() as f64)
        .sum()
}
fn price(row: &Value, fresh: bool, square: f64, inversion: f64, native: f64) -> f64 {
    if fresh {
        let c = &row["counts"];
        let unit = &row["source_add"];
        let arithmetic = number(c, "m") + square * number(c, "s") + inversion * number(c, "i");
        let divisor =
            number(unit, "m") + square * number(unit, "s") + inversion * number(unit, "i");
        assert!(divisor > 0.0);
        arithmetic / divisor
            + native * (logical(&row["native"]) + number(c, "field_adds") + number(c, "bit_ops"))
    } else {
        let c = &row["counters"];
        let arithmetic = number(c, "field_multiplications")
            + square * number(c, "field_squarings")
            + inversion * number(c, "field_inversions");
        let divisor = number(c, "source_add_multiplications")
            + square * number(c, "source_add_squarings")
            + inversion * number(c, "source_add_inversions");
        assert!(divisor > 0.0);
        arithmetic / divisor + native * logical(c)
    }
}
fn arm_name(d: u64, j: u64) -> String {
    format!("fb{d}-j{j}")
}
fn paired(row: &Value, control: &Value, external: &Value) {
    let workload = &control["workload"];
    assert_eq!(row["workload_id"], workload["workload_id"]);
    assert_eq!(row["workload_sha256"], workload["workload_sha256"]);
    assert_eq!(row["algorithm_seed"], control["algorithm_seed"]);
    assert_eq!(row["algorithm_seed"], external["algorithm_seed"]);
    assert_eq!(
        row["cap"].to_string(),
        control["method"]["params"]["trial_cap"].as_str().unwrap()
    );
    assert_eq!(row["planted"], workload["planted"]);
    assert_eq!(row["target_index"], workload["target_index"]);
    assert_eq!(row["source_add"], external["source_add"]);
    assert_eq!(row["recovered"], external["recovered"]);
    assert_eq!(control["outcome"]["status"], "verified");
    assert_eq!(row["verified"], true);
    assert_eq!(row["exhausted"], false);
}
fn main() {
    let mut targets = File::create(format!("{ROOT}/targets.jsonl")).unwrap();
    let mut cells = File::create(format!("{ROOT}/price-grid.csv")).unwrap();
    writeln!(cells, "arm,slice,targets,verified,failures,S_per_M,I_per_M,native_GAE_per_logical,candidate_GAE,external_GAE,rho_GAE,candidate_over_external,candidate_over_rho,processed_divisors,rejected_large_large").unwrap();
    let mut all: BTreeMap<String, Vec<(u64, Value, Value, Value)>> = BTreeMap::new();
    let mut baseline_replayed = 0u64;
    let mut workload_ids = BTreeSet::new();
    for bits in BITS {
        let frozen = archive_rows(bits);
        let rho: BTreeMap<String, Value> = frozen
            .into_iter()
            .filter(|row| row["arm"] == "rho_neg")
            .map(|row| {
                (
                    row["workload"]["workload_id"].as_str().unwrap().to_owned(),
                    row,
                )
            })
            .collect();
        assert_eq!(rho.len(), 180);
        let external: BTreeMap<String, Value> = rows(Path::new(&format!(
            "research/genus2_external_gate_20261021/results/order-genus-{bits}.jsonl"
        )))
        .into_iter()
        .map(|row| (row["workload_id"].as_str().unwrap().to_owned(), row))
        .collect();
        assert_eq!(external.len(), 180);
        for (d, j) in ARMS {
            let arm = arm_name(d, j);
            let path = format!("{ROOT}/{arm}-bits{bits}.jsonl");
            let fresh = rows(Path::new(&path));
            assert_eq!(fresh.len(), 180, "{path}");
            let mut arm_ids = BTreeSet::new();
            for row in fresh {
                let id = row["workload_id"].as_str().unwrap().to_owned();
                assert!(arm_ids.insert(id.clone()), "duplicate {id} in {arm}");
                let r = rho.get(&id).unwrap();
                let e = external.get(&id).unwrap();
                paired(&row, r, e);
                if d == 1 && j == 16 {
                    assert_eq!(row["counts"], e["counts"]);
                    assert_eq!(row["phases"], e["phases"]);
                    baseline_replayed += 1;
                    assert!(workload_ids.insert(id.clone()));
                }
                let candidate = price(&row, true, 0.5, 10.0, 0.1);
                let ext = price(e, true, 0.5, 10.0, 0.1);
                let reference = price(r, false, 0.5, 10.0, 0.1);
                writeln!(
                    targets,
                    "{}",
                    json!({
                        "schema":"four-followups-target/v1", "arm":arm, "workload_id":id,
                        "bits":bits, "curve_p":row["p"], "target_index":row["target_index"],
                        "verified":true, "candidate_GAE":candidate, "external_GAE":ext,
                        "rho_GAE":reference, "candidate_over_external":candidate/ext,
                        "candidate_over_rho":candidate/reference,
                        "processed_divisors":row["native"]["processed_divisors"],
                        "rejected_large_large":row["native"]["rejected_large_large_relations"],
                        "raw_record":path
                    })
                )
                .unwrap();
                all.entry(arm.clone())
                    .or_default()
                    .push((bits, row, e.clone(), r.clone()));
            }
        }
    }
    assert_eq!(baseline_replayed, 1260);
    assert_eq!(workload_ids.len(), 1260);
    let mut summary = Vec::new();
    for (arm, rows) in &all {
        assert_eq!(rows.len(), 1260);
        for slice in std::iter::once(None).chain(BITS.map(Some)) {
            let selected: Vec<_> = rows
                .iter()
                .filter(|(bits, _, _, _)| slice.is_none_or(|wanted| wanted == *bits))
                .collect();
            let label = slice.map_or_else(|| "all".to_owned(), |bits| bits.to_string());
            let attempts: u64 = selected
                .iter()
                .map(|(_, row, _, _)| row["native"]["processed_divisors"].as_u64().unwrap())
                .sum();
            let rejected: u64 = selected
                .iter()
                .map(|(_, row, _, _)| {
                    row["native"]["rejected_large_large_relations"]
                        .as_u64()
                        .unwrap()
                })
                .sum();
            let mut representative = None;
            let mut ratios_rho = Vec::new();
            let mut ratios_external = Vec::new();
            for square in [0.0, 0.5, 1.0] {
                for inversion in [1.0, 10.0, 100.0, 1000.0] {
                    for native in [0.0, 0.1, 1.0] {
                        let candidate: f64 = selected
                            .iter()
                            .map(|(_, row, _, _)| price(row, true, square, inversion, native))
                            .sum();
                        let external: f64 = selected
                            .iter()
                            .map(|(_, _, row, _)| price(row, true, square, inversion, native))
                            .sum();
                        let rho: f64 = selected
                            .iter()
                            .map(|(_, _, _, row)| price(row, false, square, inversion, native))
                            .sum();
                        let re = candidate / external;
                        let rr = candidate / rho;
                        writeln!(cells, "{arm},{label},{},{},0,{square},{inversion},{native},{candidate:.12},{external:.12},{rho:.12},{re:.12},{rr:.12},{attempts},{rejected}", selected.len(), selected.len()).unwrap();
                        ratios_rho.push(rr);
                        ratios_external.push(re);
                        if square == 0.5 && inversion == 10.0 && native == 0.1 {
                            representative = Some(
                                json!({"candidate_GAE":candidate,"external_GAE":external,"rho_GAE":rho,"candidate_over_external":re,"candidate_over_rho":rr}),
                            );
                        }
                    }
                }
            }
            summary.push(json!({"arm":arm,"slice":label,"targets":selected.len(),"verified":selected.len(),"failures":0,
                "representative":representative.unwrap(),"processed_divisors":attempts,"rejected_large_large":rejected,
                "candidate_over_rho_band":[ratios_rho.iter().copied().reduce(f64::min).unwrap(),ratios_rho.iter().copied().reduce(f64::max).unwrap()],
                "candidate_over_external_band":[ratios_external.iter().copied().reduce(f64::min).unwrap(),ratios_external.iter().copied().reduce(f64::max).unwrap()],
                "candidate_win_cells_vs_rho":ratios_rho.iter().filter(|&&x|x<1.0).count(),
                "candidate_win_cells_vs_external":ratios_external.iter().filter(|&&x|x<1.0).count()}));
        }
    }
    fs::write(
        format!("{ROOT}/summary.json"),
        format!(
            "{}\n",
            serde_json::to_string_pretty(&json!({
                "schema":"four-followups-factor-base-analysis/v1", "source_archive":ARCHIVE,
                "baseline_replayed":baseline_replayed,"arm_target_records":6300,"summary":summary
            }))
            .unwrap()
        ),
    )
    .unwrap();
}
