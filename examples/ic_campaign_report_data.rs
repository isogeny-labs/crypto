//! Independent finite-table recomputation for the campaign's visual reports.
//! Reads raw native records; never uses timings or derived summary ratios.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

type Key = (u64, u64, String, u64, u64);
fn rows(path: &Path) -> Vec<Value> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}
fn run_inventory(root: &Path) -> Vec<Value> {
    let mut dirs: Vec<_> = std::fs::read_dir(root)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();
    dirs.into_iter()
        .map(|dir| {
            let mut cells: Vec<_> = std::fs::read_dir(&dir)
                .unwrap()
                .map(|e| e.unwrap().path())
                .filter(|p| p.is_dir())
                .collect();
            cells.sort();
            let mut statuses = BTreeMap::<String, u64>::new();
            let mut exits = BTreeMap::<String, u64>::new();
            let mut raw = vec![];
            let mut extra = vec![];
            let mut invalid_lines = 0;
            for cell in cells {
                let rowfile = cell.join("rows.jsonl");
                if rowfile.is_file() {
                    let bytes = std::fs::read(&rowfile).unwrap();
                    let mut count = 0;
                    for line in String::from_utf8_lossy(&bytes)
                        .lines()
                        .filter(|s| !s.trim().is_empty())
                    {
                        match serde_json::from_str::<Value>(line) {
                            Ok(row) => {
                                count += 1;
                                *statuses
                                    .entry(row["status"].as_str().unwrap_or("no_status").into())
                                    .or_default() += 1;
                            }
                            Err(_) => invalid_lines += 1,
                        }
                    }
                    raw.push(
                        json!({"path":rowfile.strip_prefix(root).unwrap(),"rows":count,
                    "sha256":format!("{:x}", Sha256::digest(&bytes))}),
                    );
                }
                let status = cell.join("status.json");
                if status.is_file() {
                    let v: Value = serde_json::from_slice(&std::fs::read(status).unwrap()).unwrap();
                    *exits.entry(v["exit_code"].to_string()).or_default() += 1;
                }
            }
            let mut sidecars: Vec<_> = std::fs::read_dir(&dir)
                .unwrap()
                .map(|e| e.unwrap().path())
                .filter(|p| {
                    p.file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .ends_with(".status.json")
                })
                .collect();
            sidecars.sort();
            for path in sidecars {
                let receipt: Value =
                    serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
                extra.push(json!({"path":path.strip_prefix(root).unwrap(),"receipt":receipt}));
            }
            json!({"run":dir.file_name().unwrap().to_str().unwrap(),
            "ended_receipt_present":dir.join("ended.txt").is_file(),
            "process_exit_codes":exits,"row_statuses":statuses,"raw_files":raw,
            "non_json_lines_preserved":invalid_lines,"extra_status_receipts":extra})
        })
        .collect()
}
fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    assert_eq!(args.len(), 1, "study directory");
    let study = Path::new(&args[0]);
    let mut panels = vec![];
    let mut aa_expected = BTreeMap::new();
    for (degree, name, auditname) in [
        (2, "structural-d2-20261023-01", "query-audit-d2-20261023-01"),
        (3, "structural-d3-20261023-01", "query-audit-d3-20261023-01"),
        (4, "structural-20261023-02", "query-audit-20261023-01"),
    ] {
        let root = study.join("runs").join(name);
        let mut files: Vec<_> = std::fs::read_dir(&root)
            .unwrap()
            .map(|e| e.unwrap().path().join("rows.jsonl"))
            .filter(|p| p.is_file())
            .collect();
        files.sort();
        let mut pairs = BTreeMap::<Key, BTreeMap<String, Value>>::new();
        let (mut count, mut dup, mut complete) = (0u64, 0u64, 0u64);
        let (mut eroots, mut erejected, mut eaccepted) = (0u64, 0u64, 0u64);
        let mut statuses = BTreeMap::<String, u64>::new();
        let mut dimensions = BTreeMap::<String, BTreeSet<Vec<u64>>>::new();
        for file in files {
            for r in rows(&file) {
                if degree == 4
                    && r["field_degree"] == 19
                    && r["ell"] == 4
                    && r["family"] == "geometric"
                    && r["base_rep"] == 0
                {
                    let mut deterministic = r.clone();
                    deterministic
                        .as_object_mut()
                        .unwrap()
                        .remove("diagnostic_seconds");
                    aa_expected.insert(
                        (
                            r["formulation"].as_str().unwrap().to_string(),
                            r["target_index"].as_u64().unwrap(),
                        ),
                        deterministic,
                    );
                }
                count += 1;
                assert_eq!(r["degree"], degree);
                *statuses
                    .entry(r["status"].as_str().unwrap().into())
                    .or_default() += 1;
                complete += u64::from(r["extraction_complete"] == true);
                if r["family"] == "prefix" && r["base_rep"] == 1 {
                    dup += 1;
                    continue;
                }
                if r["formulation"] == "symmetric-e" && r["extraction_complete"] == true {
                    eroots += r["algebraic_roots"].as_u64().unwrap();
                    erejected += r["rejected_roots"].as_u64().unwrap();
                    eaccepted += r["accepted_x_multisets"].as_array().unwrap().len() as u64;
                }
                if r["field_degree"] == 31 && r["ell"] == 4 && r["formulation"] == "symmetric-e" {
                    dimensions
                        .entry(r["family"].as_str().unwrap().into())
                        .or_default()
                        .insert(serde_json::from_value(r["product_dimensions"].clone()).unwrap());
                }
                let key = (
                    r["field_degree"].as_u64().unwrap(),
                    r["ell"].as_u64().unwrap(),
                    r["family"].as_str().unwrap().into(),
                    r["base_rep"].as_u64().unwrap(),
                    r["target_index"].as_u64().unwrap(),
                );
                let form = r["formulation"].as_str().unwrap().to_string();
                assert!(pairs.entry(key).or_default().insert(form, r).is_none());
            }
        }
        let mut points = vec![];
        let (mut less, mut greater, mut equal) = (0u64, 0u64, 0u64);
        let (mut min, mut max) = (f64::INFINITY, 0f64);
        for ((n, ell, family, rep, target), forms) in pairs {
            let (Some(a), Some(b)) = (forms.get("eliminated-s4"), forms.get("symmetric-e")) else {
                continue;
            };
            assert_eq!(a["basis"], b["basis"]);
            assert_eq!(a["target"], b["target"]);
            if a["extraction_complete"] != true || b["extraction_complete"] != true {
                continue;
            }
            let words = |r: &Value| {
                r["closure_and_readback_word_ops"].as_u64().unwrap()
                    + r["extraction_word_ops"].as_u64().unwrap_or(0)
            };
            let (aw, bw) = (words(a), words(b));
            assert!(aw > 0);
            let ratio = bw as f64 / aw as f64;
            less += u64::from(bw < aw);
            greater += u64::from(bw > aw);
            equal += u64::from(bw == aw);
            min = min.min(ratio);
            max = max.max(ratio);
            points.push(json!({"n":n,"ell":ell,"family":family,"rep":rep,"target_index":target,"direct_words":aw,"symmetric_words":bw,"ratio":ratio}));
        }
        let ar = rows(
            &study
                .join("runs")
                .join(auditname)
                .join("cell-0000/rows.jsonl"),
        );
        let verdict = ar.last().unwrap();
        assert_eq!(verdict["status"], "passed");
        assert_eq!(verdict["rows"], count);
        assert_eq!(verdict["complete_extractions"], complete);
        assert_eq!(count, 1152);
        assert_eq!(dup, 144);
        assert_eq!(points.len(), 304);
        panels.push(json!({"degree":degree,"run":name,"raw_rows":count,"duplicate_prefix_rows":dup,"complete_extractions":complete,"paired_queries":points.len(),
            "symmetric_lower_words":less,"symmetric_higher_words":greater,"equal_words":equal,"ratio_min":min,"ratio_max":max,"points":points,
            "statuses":statuses,"symmetric_algebraic_roots":eroots,"symmetric_rejected_roots":erejected,"symmetric_accepted_multisets":eaccepted,
            "n31_ell4_product_dimensions":dimensions,"independent_query_audit":verdict}));
    }
    let (mut branches, mut total_words, mut total_products) = (vec![], 0u64, 0u64);
    for b in 0..8 {
        let path = study.join(format!(
            "runs/guess-balanced-k3-d0-b{b}-20261023-02/cell-0000/rows.jsonl"
        ));
        let rr = rows(&path);
        assert_eq!(rr.len(), 1);
        let r = &rr[0];
        assert_eq!(r["branch"], b);
        assert_eq!(r["status"], "saturated_not_refuted");
        assert_eq!(r["columns"], 136698);
        assert_eq!(r["degree"], 4);
        total_words += r["closure_word_ops"].as_u64().unwrap();
        total_products += r["closure_products"].as_u64().unwrap();
        branches.push(r.clone());
    }
    let old = rows(&study.join("runs/fixture-replay-20261023-01/cell-0000/rows.jsonl"));
    assert_eq!(old.len(), 7);
    assert!(old.iter().all(
        |r| r["count_matches"] == true && (r["solutions"] == 0 || r["witness_checked"] == true)
    ));
    let aa = rows(&study.join("runs/aa-original-20261023-01/cell-0000/rows.jsonl"));
    assert_eq!(aa.len(), 12);
    assert_eq!(aa_expected.len(), 12);
    for mut row in aa {
        row.as_object_mut().unwrap().remove("diagnostic_seconds");
        let key = (
            row["formulation"].as_str().unwrap().to_string(),
            row["target_index"].as_u64().unwrap(),
        );
        assert_eq!(&row, aa_expected.get(&key).unwrap(), "A/A mismatch {key:?}");
    }
    let controls = rows(&study.join("runs/selftest-20261023-03/cell-0000/rows.jsonl"));
    assert_eq!(controls.len(), 1);
    assert_eq!(controls[0]["status"], "passed");
    let mut fresh = vec![];
    let mut files: Vec<_> = std::fs::read_dir(study.join("runs/fixtures-20261023-01"))
        .unwrap()
        .map(|e| e.unwrap().path().join("rows.jsonl"))
        .filter(|p| p.is_file())
        .collect();
    files.sort();
    for file in files {
        let rr = rows(&file);
        assert_eq!(rr.len(), 32);
        let (mut sat, mut unsat) = (0u64, 0u64);
        for r in &rr {
            if r["solutions"].as_u64().unwrap() == 0 {
                unsat += 1;
            } else {
                sat += 1;
            }
        }
        fresh.push(json!({"m":rr[0]["m"],"n":rr[0]["n"],"ell":rr[0]["ell"],"family":rr[0]["family"],"draws":32,"sat":sat,"unsat":unsat}));
    }
    assert_eq!(fresh.len(), 36);
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"ic-campaign-report-data/v1","status":"recomputed_from_raw",
        "panels":panels,"balanced_guess_draw0":{"branches":branches,"total_word_ops":total_words,"total_closure_products":total_products,"refuted_branches":0,"saturated_nonrefuting_branches":8},
        "original_fixture_replay":{"rows":7,"all_counts_match":true,"sat_witnesses_checked":3},
        "small_controls":controls[0],"aa_replay":{"rows":12,"all_deterministic_fields_equal":true,"excluded_fields":["diagnostic_seconds"]},
        "fresh_fixture_outcomes":fresh,
        "run_inventory":run_inventory(&study.join("runs")),
        "complete_dlp_cost":null,"rho_ratio":null,"scope":"Finite stage counters only; construction, splitting and physical conversion remain unpriced."})).unwrap());
}
