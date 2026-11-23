//! Re-read complete saved proofs and raw Callgrind totals. No runtime analysis.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs,
    io::Write,
    path::Path,
};
type R<T> = Result<T, Box<dyn Error>>;
fn read(p: &Path) -> R<Vec<u8>> {
    let m = fs::symlink_metadata(p)?;
    if !m.is_file() || m.file_type().is_symlink() || m.len() > 16 << 20 {
        return Err("file type/size".into());
    }
    Ok(fs::read(p)?)
}
fn val(p: &Path) -> R<Value> {
    Ok(serde_json::from_slice(&read(p)?)?)
}
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn main() -> R<()> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 4 {
        return Err("usage:profile-analyze RAW MONITOR NEW_JSON".into());
    }
    let root = Path::new(&a[1]);
    let summary = val(&root.join("summary.json"))?;
    let before = val(&root.join("pins-before.json"))?;
    if before != val(&root.join("pins-after.json"))? {
        return Err("pins changed".into());
    }
    let outcomes = summary["outcomes"].as_array().ok_or("outcomes")?;
    if outcomes.len() != 36 {
        return Err("roster length".into());
    }
    let monitor_text = String::from_utf8(read(Path::new(&a[2]))?)?;
    let monitors: Vec<Value> = monitor_text
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()?;
    let mut rows = Vec::new();
    let mut seen = BTreeSet::new();
    let mut bases = BTreeMap::new();
    for p in [2147483647u64, 9007199254740881] {
        for n in [3, 4, 6] {
            for form in ["compact-sparse", "compact-dag"] {
                for kernel in ["frontier", "packed-products", "packed-batch"] {
                    let id = format!("p{p}-n{n}-{form}-{kernel}");
                    if !seen.insert(id.clone()) {
                        return Err("duplicate roster".into());
                    }
                    let row = outcomes
                        .iter()
                        .find(|x| x["case"] == id)
                        .ok_or("missing case")?;
                    if row["status"] != "complete" || row["complete_second_replay"] != true {
                        return Err(format!("failed case:{id}").into());
                    }
                    let dir = root.join(&id);
                    if row != &val(&dir.join("outcome.json"))? {
                        return Err("summary/receipt differ".into());
                    }
                    let original = read(&dir.join("proof/original.json"))?;
                    let wire = read(&dir.join("proof/witness.json"))?;
                    if row["proof_sha256"] != sha(&wire) {
                        return Err("proof hash".into());
                    }
                    let pin = before["files"]
                        .as_array()
                        .ok_or("files")?
                        .iter()
                        .find(|x| {
                            x["path"].as_str().is_some_and(|s| {
                                s.ends_with(&format!("p{p}-n{n}-frontier/original.json"))
                            })
                        })
                        .ok_or("input pin")?;
                    if pin["sha256"] != sha(&original) {
                        return Err("frozen input hash".into());
                    }
                    let checked = if form == "compact-dag" {
                        relation_replay_checker::packed::verify_dag_json(&original, &wire)?
                    } else {
                        relation_replay_checker::packed::verify_json(&original, &wire)?
                    };
                    let worker = val(&dir.join("profile/command.stdout"))?;
                    let basis = serde_json::to_value(&checked.basis)?;
                    if worker["basis"] != basis
                        || worker["consumer_stats"] != serde_json::to_value(&checked.stats)?
                    {
                        return Err("independent proof/basis/stats binding".into());
                    }
                    if let Some(old) = bases.get(&(p, n)) {
                        if old != &basis {
                            return Err("cross-arm basis mismatch".into());
                        }
                    } else {
                        bases.insert((p, n), basis);
                    }
                    let hot = val(&dir.join("profile/hotspots.json"))?;
                    if hot != row["hotspots"] || hot["method"] != "callgrind" {
                        return Err("hotspot metadata".into());
                    }
                    let raw = String::from_utf8(read(&dir.join("profile/callgrind.out"))?)?;
                    if !raw.lines().any(|l| l == "events: Ir") {
                        return Err("raw event kind".into());
                    }
                    let totals = |prefix: &str| -> R<Vec<u64>> {
                        Ok(raw
                            .lines()
                            .filter_map(|l| l.strip_prefix(prefix))
                            .map(|s| s.trim().parse())
                            .collect::<Result<_, _>>()?)
                    };
                    let start = totals("summary: ")?;
                    let end = totals("totals: ")?;
                    if start.len() != 1 || start != end || hot["total_ir"] != start[0] {
                        return Err("raw/parsed Ir totals differ".into());
                    }
                    let mut allocation = 0u64;
                    let mut axpy = 0u64;
                    let mut vector = 0u64;
                    for h in hot["hotspots"].as_array().ok_or("hotspots")? {
                        let s = h["symbol"].as_str().ok_or("symbol")?;
                        let ir = h["ir"].as_u64().ok_or("self Ir")?;
                        if matches!(
                            s,
                            "malloc"
                                | "free"
                                | "realloc"
                                | "calloc"
                                | "__rustc::__rdl_alloc"
                                | "__rustc::__rdl_dealloc"
                        ) {
                            allocation = allocation.checked_add(ir).ok_or("allocation sum")?;
                        }
                        if s.starts_with("certified_relations::axpy_in_place") {
                            axpy = axpy.checked_add(ir).ok_or("axpy sum")?;
                        }
                        if s.contains("Vec<usize>") && s.contains("from_iter") {
                            vector = vector.checked_add(ir).ok_or("vector sum")?;
                        }
                    }
                    rows.push(json!({"case":id,"prime":p,"n":n,"form":form,"kernel":kernel,"raw_total_ir":start[0],"visible_allocator_self_ir":allocation,"visible_axpy_self_ir":axpy,"visible_vector_from_iter_self_ir":vector,"visible_allocator_self_pct":100.*allocation as f64/start[0] as f64,"complete_replay_and_basis_binding":true,"proof_sha256":sha(&wire)}));
                }
            }
        }
    }
    let report = json!({"schema":"relation-child-profile-analysis/v1","case_count":36,"cases":rows,"full_saved_proof_replays":36,"raw_and_parsed_Ir_totals_match":true,"instruction_shares_from_top25_only":true,"visible_category_shares_are_lower_bounds":true,"inclusive_shares_added":false,"monitors":monitors.iter().map(|m|json!({"exit":m["exit_status"],"flagged_contention_samples":m["contended_samples"],"sample_count":m["samples"].as_array().map(Vec::len)})).collect::<Vec<_>>(),"scope":"one simulated child instruction profile per combination; controller/wrappers excluded; no runtime ratio, confidence interval or speedup admission","timing_claim":false,"standing":"unsigned producer analysis with separate checker arithmetic, same host"});
    fs::File::options()
        .write(true)
        .create_new(true)
        .open(&a[3])?
        .write_all(&serde_json::to_vec_pretty(&report)?)?;
    println!("36 raw Ir totals matched and36 complete saved proofs replayed; no runtime admission");
    for row in report["cases"].as_array().unwrap() {
        println!(
            "{}\t{}\tallocator_self_pct={:.3}",
            row["case"].as_str().unwrap(),
            row["raw_total_ir"],
            row["visible_allocator_self_pct"].as_f64().unwrap()
        );
    }
    Ok(())
}
