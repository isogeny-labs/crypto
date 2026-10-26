//! Recompute finite campaign tables from raw native rows. No timings enter.
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

#[derive(Default)]
struct Cell {
    rows: u64,
    complete: u64,
    refuted: u64,
    roots: u64,
    rejected: u64,
    accepted: u64,
    words: u128,
    statuses: BTreeMap<String, u64>,
    dimensions: BTreeSet<Vec<u64>>,
}
fn read_rows(path: &Path) -> Vec<Value> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}
fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    assert_eq!(args.len(), 2, "structural-run query-audit-rows.jsonl");
    let root = Path::new(&args[0]);
    let audit = read_rows(Path::new(&args[1]));
    let summary = audit.last().unwrap();
    assert_eq!(summary["schema"], "ic-algebraic-query-audit-summary/v1");
    assert_eq!(summary["status"], "passed");
    let mut paths: Vec<_> = std::fs::read_dir(root)
        .unwrap()
        .map(|e| e.unwrap().path().join("rows.jsonl"))
        .filter(|p| p.is_file())
        .collect();
    paths.sort();
    let mut cells = BTreeMap::<(u64, u64, String, String), Cell>::new();
    let mut paired =
        BTreeMap::<(u64, u64, String, u64, u64), BTreeMap<String, (bool, u128)>>::new();
    let (mut raw_rows, mut duplicate_rows) = (0u64, 0u64);
    for path in paths {
        for row in read_rows(&path) {
            raw_rows += 1;
            assert_eq!(row["schema"], "ic-algebraic-profile/v1");
            let n = row["field_degree"].as_u64().unwrap();
            let ell = row["ell"].as_u64().unwrap();
            let family = row["family"].as_str().unwrap().to_string();
            let form = row["formulation"].as_str().unwrap().to_string();
            if family == "prefix" && row["base_rep"] == 1 {
                duplicate_rows += 1;
                continue;
            }
            let c = cells
                .entry((n, ell, family.clone(), form.clone()))
                .or_default();
            c.rows += 1;
            let done = row["extraction_complete"] == true;
            c.complete += u64::from(done);
            c.refuted += u64::from(row["closure_refuted"] == true);
            let status = row["status"].as_str().unwrap().to_string();
            *c.statuses.entry(status).or_default() += 1;
            c.dimensions
                .insert(serde_json::from_value(row["product_dimensions"].clone()).unwrap());
            if done {
                c.roots += row["algebraic_roots"].as_u64().unwrap();
                c.rejected += row["rejected_roots"].as_u64().unwrap();
                c.accepted += row["accepted_x_multisets"].as_array().unwrap().len() as u64;
            }
            let words = row["closure_and_readback_word_ops"].as_u64().unwrap_or(0) as u128
                + row["extraction_word_ops"].as_u64().unwrap_or(0) as u128;
            if done {
                c.words += words;
            }
            paired
                .entry((
                    n,
                    ell,
                    family,
                    row["base_rep"].as_u64().unwrap(),
                    row["target_index"].as_u64().unwrap(),
                ))
                .or_default()
                .insert(form, (done, words));
        }
    }
    assert_eq!(raw_rows, summary["rows"].as_u64().unwrap());
    let table:Vec<Value>=cells.into_iter().map(|((n,ell,family,form),c)|json!({
        "n":n,"ell":ell,"family":family,"formulation":form,"queries":c.rows,"complete":c.complete,
        "closure_refutations":c.refuted,"algebraic_roots_when_complete":c.roots,
        "rejected_roots_when_complete":c.rejected,"accepted_multisets_when_complete":c.accepted,
        "solver_word_ops_sum_when_complete":c.words.to_string(),
        "solver_word_ops_mean_when_complete":if c.complete>0{json!(c.words as f64/c.complete as f64)}else{Value::Null},
        "product_dimensions":c.dimensions,"statuses":c.statuses})).collect();
    let pair_table: Vec<Value> = paired
        .into_iter()
        .filter_map(|((n, ell, family, rep, target), p)| {
            let &(a, aw) = p.get("eliminated-s4")?;
            let &(b, bw) = p.get("symmetric-e")?;
            if !(a && b) {
                return None;
            }
            Some(
                json!({"n":n,"ell":ell,"family":family,"rep":rep,"target_index":target,
            "direct_solver_word_ops":aw.to_string(),"symmetric_solver_word_ops":bw.to_string(),
            "symmetric_over_direct_words":if aw>0{json!(bw as f64/aw as f64)}else{Value::Null}}),
            )
        })
        .collect();
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"ic-algebraic-summary/v1",
        "raw_rows":raw_rows,"excluded_duplicate_prefix_rows":duplicate_rows,
        "query_audit":summary,"table":table,"complete_direct_symmetric_pairs":pair_table,
        "scope":"finite stage diagnostics; extraction may exceed degree four for direct S4; construction, split and lift are not priced as word operations",
        "complete_dlp_cost":null,"rho_ratio":null})).unwrap());
}
