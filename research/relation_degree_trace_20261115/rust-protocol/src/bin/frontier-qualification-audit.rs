//! Read-only composition audit of frozen native cells and capture qualification.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{error::Error, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn read(p: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 6 {
        return Err("usage: frontier-qualification-audit OLD BASELINE DIRECT CAPTURE NEW_AUDIT".into());
    }
    let old = Path::new(&a[1]);
    let mut records = Vec::new();
    for (dir, mode) in a[2..4].iter().zip(["both_updates_fused", "both_updates_frontier"]) {
        let dir = Path::new(dir);
        let s = read(&dir.join("summary.json"))?;
        let rows = s["records"]
            .as_array()
            .filter(|r| r.len() == 15)
            .ok_or("15 native cells")?;
        if s["all_passed"] != true || s["fused_acceptance_cases"] != 15 {
            return Err("cell qualifier failed".into());
        }
        for r in rows {
            let name = r["case"].as_str().ok_or("case")?;
            let case = dir.join(name);
            let reference = old.join(name);
            let answer = read(&case.join("fused.stdout"))?;
            let prior = read(&reference.join("fused.stdout"))?;
            let wire = fs::read(case.join("proof/witness.json"))?;
            if r["passed"] != true
                || r["fresh_singular_accepts"] != true
                || r["complete_basis_equal"] != true
                || answer["completion_exports"] != 0
                || answer["kernel_mode"] != mode
                || answer["root_api"] != true
                || answer["native_product_criterion"] != true
                || answer["basis"] != prior["basis"]
                || answer["consumer"] != prior["consumer"]
                || wire != fs::read(reference.join("proof/witness.json"))?
                || fs::read(case.join("input.json"))? != fs::read(reference.join("input.json"))?
            {
                return Err(
                    format!("mode/wire/basis/consumer/input mismatch {mode}/{name}").into(),
                );
            }
            records.push(json!({"case":name,"kernel":mode,"fresh_singular_accepts":true,"wire_equals_frozen_old":true,"wire_sha256":format!("{:x}",Sha256::digest(&wire)),"consumer_counters_equal":true}));
        }
    }
    let capture = read(&Path::new(&a[4]).join("summary.json"))?;
    if capture["schema"] != "exact-frontier-paid-screen/v1"
        || capture["mode"] != "qualification"
        || capture["identities_unchanged"] != true
        || !capture["selected_followup_family"].is_null()
        || !capture["selected_kernel"].is_null()
    {
        return Err("capture qualification header".into());
    }
    let cases = capture["cases"]
        .as_array()
        .filter(|c| c.len() == 1)
        .ok_or("single n3 qualification case")?;
    let c = &cases[0];
    let samples = c["samples"]
        .as_array()
        .filter(|s| s.len() == 32)
        .ok_or("32 workflows")?;
    if c["id"] != "n3-s7"
        || c["all_complete_bases_equal"] != true
        || !c["comparisons"].is_null()
        || !c["comparison_candidate_index"].is_null()
    {
        return Err("not suppressed n3 qualification".into());
    }
    let digest = samples[0]["basis_sha256"]
        .as_str()
        .filter(|h| h.len() == 64)
        .ok_or("basis hash")?;
    for s in samples {
        if s["status"] != "complete" || s["basis_sha256"] != digest || s["used_recovery"] != false {
            return Err("incomplete/mismatched/recovered qualification".into());
        }
    }
    for (i, name, mode) in [
        (19, "root-kernel-legacy", "legacy"),
        (20, "root-kernel-ordered", "ordered"),
        (21, "root-kernel-cached", "cached"),
        (22, "root-kernel-both", "both"),
        (23, "root-update-baseline", "both"),
        (24, "root-update-in-place", "both_updates"),
        (25, "root-update-fastsum", "both_updates_sum"),
        (26, "root-lead-baseline", "both_updates"),
        (27, "root-lead-cached", "both_updates_lead"),
        (28, "root-fused-baseline", "both_updates"),
        (29, "root-fused-direct", "both_updates_fused"),
        (30, "root-frontier-baseline", "both_updates_fused"),
        (31, "root-frontier-direct", "both_updates_frontier"),
    ] {
        let s = &samples[i];
        if s["pipeline"] != name
            || s["answer"]["kernel_mode"] != mode
            || s["answer"]["completion_exports"] != 0
            || s["answer"]["root_api"] != true
            || s["answer"]["native_product_criterion"] != true
        {
            return Err("capture kernel variant routing".into());
        }
    }
    let out = Path::new(&a[5]);
    fs::create_dir(out)?;
    fs::write(
        out.join("audit.json"),
        serde_json::to_vec_pretty(
            &json!({"all_passed":true,"native_cases":records.len(),"capture_workflows":samples.len(),"records":records,"scope":"known fixtures, correctness only; overlap with native qualifier; unsigned producer audit"}),
        )?,
    )?;
    Ok(())
}
