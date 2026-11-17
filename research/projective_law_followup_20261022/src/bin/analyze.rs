//! Independent raw-row pairing, source-scalar replay, and 36-cell pricing.
use five_leads_study::three_leads::{Elliptic, Field, Point};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::process::Command;

const ROOT: &str = "research/projective_law_followup_20261022/results";
const ARCHIVE: &str = "research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst";
const BITS: [u64; 7] = [16, 20, 24, 28, 32, 36, 40];

fn read_rows(path: &str) -> Vec<Value> {
    BufReader::new(File::open(path).unwrap_or_else(|e| panic!("{path}: {e}")))
        .lines()
        .map(|line| serde_json::from_str(&line.unwrap()).unwrap())
        .collect()
}
fn archive_rows(bits: u64) -> Vec<Value> {
    let member = format!("sessions_projective_chart_v2/order-genus-{bits}/records.jsonl");
    let out = Command::new("tar")
        .args(["--zstd", "-xOf", ARCHIVE, &member])
        .output()
        .expect("archive reader");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect()
}
fn n(v: &Value, key: &str) -> f64 {
    v[key].as_u64().unwrap_or_else(|| panic!("missing {key}")) as f64
}
fn logical(v: &Value) -> f64 {
    v.as_object()
        .unwrap()
        .iter()
        .filter(|(k, _)| k.ends_with("_uncharged"))
        .map(|(_, value)| value.as_u64().unwrap() as f64)
        .sum()
}
fn price(v: &Value, archived: bool, square: f64, inversion: f64, native: f64) -> f64 {
    if archived {
        let c = &v["counters"];
        let total = n(c, "field_multiplications")
            + square * n(c, "field_squarings")
            + inversion * n(c, "field_inversions");
        let unit = n(c, "source_add_multiplications")
            + square * n(c, "source_add_squarings")
            + inversion * n(c, "source_add_inversions");
        total / unit + native * logical(c)
    } else {
        let c = &v["counts"];
        let u = &v["source_add"];
        let total = n(c, "m") + square * n(c, "s") + inversion * n(c, "i");
        let unit = n(u, "m") + square * n(u, "s") + inversion * n(u, "i");
        total / unit + native * (logical(&v["native"]) + n(c, "field_adds") + n(c, "bit_ops"))
    }
}
fn source_replay(control: &Value, scalar: u64) -> bool {
    let workload = &control["workload"];
    let spec = &workload["curve_spec"];
    let p = spec["p"].as_u64().unwrap();
    let modulus = spec["modulus"].as_u64().unwrap();
    let curve = Elliptic {
        f: Field::new(p, 2, modulus),
        c: [
            spec["b"].as_u64().unwrap(),
            spec["a"].as_u64().unwrap(),
            0,
            1,
        ],
        binary: false,
    };
    let generator = Point::affine(spec["gx"].as_u64().unwrap(), spec["gy"].as_u64().unwrap());
    let target = Point::affine(
        u64::from_str_radix(
            workload["target"][0]
                .as_str()
                .unwrap()
                .trim_start_matches("0x"),
            16,
        )
        .unwrap(),
        u64::from_str_radix(
            workload["target"][1]
                .as_str()
                .unwrap()
                .trim_start_matches("0x"),
            16,
        )
        .unwrap(),
    );
    curve.mul(generator, scalar) == target
}
fn main() {
    let arm = std::env::args()
        .nth(1)
        .expect("usage: analyze-projective projective|state");
    assert!(["projective", "state"].contains(&arm.as_str()));
    let expected_schema = if arm == "projective" {
        "projective-law-collector-record/v1"
    } else {
        "projective-state-collector-record/v1"
    };
    fs::create_dir_all(ROOT).unwrap();
    let mut targets = File::create(format!("{ROOT}/targets-{arm}.jsonl")).unwrap();
    let mut grid = File::create(format!("{ROOT}/price-grid-{arm}.csv")).unwrap();
    writeln!(grid, "slice,targets,verified,candidate_failures,rho_failures,S_per_M,I_per_M,native_GAE_per_logical,candidate_GAE,external_GAE,rho_GAE,candidate_over_external,candidate_over_rho,normalization_inversions,processed_divisors").unwrap();
    let mut paired: Vec<(u64, Value, Value, Value)> = Vec::new();
    let mut ids = BTreeSet::new();
    let mut archived_rho_replayed = 0u64;
    let mut independent_scalar_replays = 0u64;
    let mut collector_replay_rows = 0u64;
    for bits in BITS {
        let controls = archive_rows(bits);
        let archived_rho: BTreeMap<String, Value> = controls
            .into_iter()
            .filter(|v| v["arm"] == "rho_neg")
            .map(|v| (v["workload"]["workload_id"].as_str().unwrap().to_owned(), v))
            .collect();
        assert_eq!(archived_rho.len(), 180);
        let external: BTreeMap<String, Value> = read_rows(&format!(
            "research/genus2_external_gate_20261021/results/order-genus-{bits}.jsonl"
        ))
        .into_iter()
        .map(|v| (v["workload_id"].as_str().unwrap().to_owned(), v))
        .collect();
        assert_eq!(external.len(), 180);
        let raw_path = format!("{ROOT}/{arm}-{bits}.jsonl");
        let fresh = read_rows(&raw_path);
        assert_eq!(fresh.len(), 180, "{raw_path}");
        let firstpass = if arm == "projective" {
            Some(read_rows(&format!(
                "{ROOT}/firstpass/projective-{bits}.jsonl"
            )))
        } else {
            None
        };
        if let Some(earlier) = &firstpass {
            assert_eq!(earlier.len(), 180);
        }
        for (index, row) in fresh.into_iter().enumerate() {
            assert_eq!(row["schema"], expected_schema);
            if let Some(earlier) = &firstpass {
                let old = &earlier[index];
                assert_eq!(row["workload_id"], old["workload_id"]);
                for key in [
                    "recovered",
                    "verified",
                    "exhausted",
                    "counts",
                    "source_add",
                    "phases",
                    "detail",
                    "rho",
                ] {
                    assert_eq!(row[key], old[key], "collector replay {bits} {index} {key}");
                }
                let mut new_native = row["native"].clone();
                new_native
                    .as_object_mut()
                    .unwrap()
                    .remove("weighted_normalized_outputs");
                new_native
                    .as_object_mut()
                    .unwrap()
                    .remove("weighted_normalization_inversions");
                assert_eq!(new_native, old["native"]);
                collector_replay_rows += 1;
            }
            let id = row["workload_id"].as_str().unwrap().to_owned();
            assert!(ids.insert(id.clone()), "duplicate {id}");
            let control = &archived_rho[&id];
            let baseline = &external[&id];
            let workload = &control["workload"];
            assert_eq!(row["workload_sha256"], workload["workload_sha256"]);
            assert_eq!(row["target_index"], workload["target_index"]);
            assert_eq!(row["algorithm_seed"], control["algorithm_seed"]);
            assert_eq!(
                row["cap"].to_string(),
                control["method"]["params"]["trial_cap"].as_str().unwrap()
            );
            assert_eq!(row["planted"], workload["planted"]);
            assert_eq!(row["source_add"], baseline["source_add"]);
            assert_eq!(row["rho"]["source_add"], row["source_add"]);
            let k = row["recovered"].as_u64().expect("candidate exhausted");
            let kr = row["rho"]["recovered"].as_u64().expect("rho exhausted");
            assert_eq!(k, workload["planted"].as_u64().unwrap());
            assert_eq!(kr, k);
            assert_eq!(row["verified"], true);
            assert_eq!(row["rho"]["verified"], true);
            assert!(source_replay(control, k));
            assert!(source_replay(control, kr));
            independent_scalar_replays += 2;
            let fresh_rho = &row["rho"]["counts"];
            let frozen = &control["counters"];
            for (ours, earlier) in [
                ("m", "field_multiplications"),
                ("s", "field_squarings"),
                ("i", "field_inversions"),
                ("field_adds", "field_additions_uncharged"),
                ("bit_ops", "linear_bit_operations_uncharged"),
                ("ec_adds", "source_ec_additions"),
                ("jac_adds", "jacobian_additions"),
                ("sqrt_calls", "square_root_calls"),
            ] {
                assert_eq!(fresh_rho[ours], frozen[earlier], "matched rho {id} {ours}");
            }
            archived_rho_replayed += 1;
            let total_i = row["counts"]["i"].as_u64().unwrap();
            let norm_i = row["native"]["weighted_normalization_inversions"]
                .as_u64()
                .unwrap();
            let normalized = row["native"]["weighted_normalized_outputs"]
                .as_u64()
                .unwrap();
            assert!(norm_i <= total_i);
            assert!(norm_i <= 2 * normalized);
            assert_eq!(norm_i % 2, 0);
            let candidate = price(&row, false, 0.5, 10.0, 0.1);
            let ext = price(baseline, false, 0.5, 10.0, 0.1);
            let rho = price(&row["rho"], false, 0.5, 10.0, 0.1);
            writeln!(targets, "{}", json!({
                "schema":"projective-law-target-price/v1", "arm":arm,"workload_id":id,"bits":bits,
                "p":row["p"],"target_index":row["target_index"],
                "candidate_verified":true,"rho_verified":true,
                "candidate_GAE":candidate,"external_GAE":ext,"rho_GAE":rho,
                "candidate_over_external":candidate/ext,"candidate_over_rho":candidate/rho,
                "weighted_normalization_inversions":norm_i,
                "processed_divisors":row["native"]["processed_divisors"],
                "raw_record":raw_path
            })).unwrap();
            paired.push((bits, row, baseline.clone(), control.clone()));
        }
    }
    assert_eq!(paired.len(), 1260);
    let mut summaries = Vec::new();
    for slice in std::iter::once(None).chain(BITS.map(Some)) {
        let selected: Vec<_> = paired
            .iter()
            .filter(|(bits, _, _, _)| slice.is_none_or(|wanted| wanted == *bits))
            .collect();
        let label = slice.map_or_else(|| "all".to_owned(), |bits| bits.to_string());
        let total_norm_i: u64 = selected
            .iter()
            .map(|(_, v, _, _)| {
                v["native"]["weighted_normalization_inversions"]
                    .as_u64()
                    .unwrap()
            })
            .sum();
        let processed: u64 = selected
            .iter()
            .map(|(_, v, _, _)| v["native"]["processed_divisors"].as_u64().unwrap())
            .sum();
        let mut representative = Value::Null;
        let mut min_rho = f64::INFINITY;
        let mut max_rho: f64 = 0.0;
        let mut min_external = f64::INFINITY;
        let mut max_external: f64 = 0.0;
        let mut cells_below_rho = 0u64;
        let mut cells_below_external = 0u64;
        for square in [0.0, 0.5, 1.0] {
            for inversion in [1.0, 10.0, 100.0, 1000.0] {
                for native in [0.0, 0.1, 1.0] {
                    let cand: f64 = selected
                        .iter()
                        .map(|(_, v, _, _)| price(v, false, square, inversion, native))
                        .sum();
                    let ext: f64 = selected
                        .iter()
                        .map(|(_, _, v, _)| price(v, false, square, inversion, native))
                        .sum();
                    let rho: f64 = selected
                        .iter()
                        .map(|(_, v, _, _)| price(&v["rho"], false, square, inversion, native))
                        .sum();
                    assert!(cand > 0.0 && ext > 0.0 && rho > 0.0);
                    min_rho = min_rho.min(cand / rho);
                    max_rho = max_rho.max(cand / rho);
                    min_external = min_external.min(cand / ext);
                    max_external = max_external.max(cand / ext);
                    cells_below_rho += u64::from(cand < rho);
                    cells_below_external += u64::from(cand < ext);
                    writeln!(grid, "{label},{},{},0,0,{square},{inversion},{native},{cand:.12},{ext:.12},{rho:.12},{:.12},{:.12},{total_norm_i},{processed}",
                        selected.len(), selected.len(), cand/ext, cand/rho).unwrap();
                    if square == 0.5 && inversion == 10.0 && native == 0.1 {
                        representative = json!({"slice":label,"targets":selected.len(),"candidate_GAE":cand,
                            "external_GAE":ext,"rho_GAE":rho,"candidate_over_external":cand/ext,
                            "candidate_over_rho":cand/rho,"normalization_inversions":total_norm_i,
                            "processed_divisors":processed});
                    }
                }
            }
        }
        representative["rho_ratio_range_36_cells"] = json!([min_rho, max_rho]);
        representative["external_ratio_range_36_cells"] = json!([min_external, max_external]);
        representative["cells_below_rho"] = json!(cells_below_rho);
        representative["cells_below_external"] = json!(cells_below_external);
        summaries.push(representative);
    }
    fs::write(format!("{ROOT}/summary-{arm}.json"), serde_json::to_vec_pretty(&json!({
        "schema":"projective-law-summary/v1","arm":arm,"targets":paired.len(),
        "candidate_verified":1260,"rho_verified":1260,"archived_rho_count_replays":archived_rho_replayed,
        "independent_source_scalar_replays":independent_scalar_replays,
        "collector_replay_rows":collector_replay_rows,
        "arithmetic_gate_models":21,"price_cells_per_slice":36,"summaries":summaries
    })).unwrap()).unwrap();
    if arm == "state" {
        let branches = read_rows(&format!("{ROOT}/arithmetic-branches.jsonl"));
        assert_eq!(branches.len(), 21);
        let sum =
            |key: &str| -> u64 { branches.iter().map(|row| row[key].as_u64().unwrap()).sum() };
        let branch_totals = json!({
            "models":branches.len(),
            "weighted_1plus1_checked":sum("cases"),
            "weighted_1plus2_checked":sum("weighted_1plus2_checked"),
            "weighted_1plus2_unchecked":sum("weighted_1plus2_unchecked"),
            "weighted_2plus2_pairs_checked":sum("weighted_all_2plus2_checked"),
            "weighted_2plus2_pairs_unchecked":sum("weighted_all_2plus2_unchecked"),
            "weighted_2plus2_targeted_checked":sum("weighted_targeted_add22_checked"),
            "weighted_2plus2_targeted_unchecked":sum("weighted_targeted_add22_unchecked"),
            "weighted_doubling_checked":sum("weighted_doubling_2plus2_checked"),
            "weighted_special_doubling_checked":sum("weighted_special_doubling_checked"),
            "weighted_degree_drop_doubling_checked":sum("weighted_degree_drop_doubling_checked")
            ,"weighted_state_walk_steps_checked":sum("weighted_state_walk_steps_checked")
        });
        fs::write(
            format!("{ROOT}/arithmetic-summary.json"),
            serde_json::to_vec_pretty(&branch_totals).unwrap(),
        )
        .unwrap();
    }
}
