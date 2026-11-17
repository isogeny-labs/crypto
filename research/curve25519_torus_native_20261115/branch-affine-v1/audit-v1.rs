use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    env, fs,
    path::{Component, Path},
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const CONTROL_SHA: &str = "de00f0bade0c248a8d771ee66a19791e3026246d8e3300c878c3513aa4b73748";
const MANIFEST_SHA: &str = "587963272adbb808ebaa90b164ac308af5fb71ac88ca46fad476d00e653610dd";
fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn bytes(p: &Path) -> Result<Vec<u8>> {
    if fs::metadata(p)?.len() > 64 << 20 {
        return Err("reader byte refusal".into());
    }
    Ok(fs::read(p)?)
}
fn read(p: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&bytes(p)?)?)
}
fn guard(b: &[u8], expected: &str) -> Result<()> {
    if hash(b) != expected {
        return Err("input digest differs".into());
    }
    Ok(())
}
fn kind(v: &Value) -> Result<&'static str> {
    if v.get("Root").is_some() {
        Ok("Root")
    } else if v["Stopped"].get("Unsupported").is_some() {
        Ok("Unsupported")
    } else if v["Stopped"].get("Cap").is_some() {
        Ok("Cap")
    } else {
        Err("unexpected curve outcome".into())
    }
}
fn case(raw: &Path, c: &Value) -> Result<(Value, Value, Value)> {
    let id = c["id"].as_str().ok_or("case id")?;
    if id.contains('/') || id.contains("..") {
        return Err("case path".into());
    }
    let old = read(&raw.join(id).join("old-v1.json"))?;
    let off = read(&raw.join(id).join("disabled.json"))?;
    let on = read(&raw.join(id).join("enabled.json"))?;
    for field in ["answer", "counts", "decisions", "schedule"] {
        if old[field] != off[field] {
            return Err(format!("old/off differs {id} {field}").into());
        }
    }
    if off["answer"] != c["before_answer"]
        || on["answer"] != c["after_answer"]
        || off["counts"] != c["before_counts"]
        || on["counts"] != c["after_counts"]
    {
        return Err("summary differs from full case artifacts".into());
    }
    Ok((old, off, on))
}
fn run() -> Result<()> {
    let raw = std::path::PathBuf::from(env::args().nth(1).ok_or("RAW_DIR NEW_OUTPUT_DIR")?);
    let out = std::path::PathBuf::from(env::args().nth(2).ok_or("RAW_DIR NEW_OUTPUT_DIR")?);
    fs::create_dir(&out)?;
    let control_bytes = bytes(&raw.join("controls.json"))?;
    guard(&control_bytes, CONTROL_SHA)?;
    let manifest_bytes = bytes(&raw.parent().ok_or("parent")?.join("NATIVE_ARCHIVE.json"))?;
    guard(&manifest_bytes, MANIFEST_SHA)?;
    let control: Value = serde_json::from_slice(&control_bytes)?;
    let manifest: Value = serde_json::from_slice(&manifest_bytes)?;
    fs::write(
        out.join("pre-call.json"),
        serde_json::to_vec_pretty(&json!({
        "schema":"torus.branch-affine.audit-pre-call/v1","standing":"producer unsigned/unverified",
        "source_sha256":hash(include_bytes!("audit-v1.rs")),"binary_sha256":hash(&bytes(&env::current_exe()?)?),
        "controls_sha256":CONTROL_SHA,"manifest_sha256":MANIFEST_SHA,"purpose":"post-call read-only native aggregation, no solver call"}))?,
    )?;
    let members = manifest["members"].as_object().ok_or("members")?;
    for (name, spec) in members {
        let p = Path::new(name);
        if p.components().any(|c| !matches!(c, Component::Normal(_))) {
            return Err("manifest member path".into());
        }
        let b = bytes(&raw.parent().ok_or("parent")?.join(p))?;
        if spec["bytes"].as_u64() != Some(b.len() as u64)
            || spec["sha256"].as_str() != Some(hash(&b).as_str())
        {
            return Err("raw member mismatch".into());
        }
    }
    let curves = control["curves"].as_array().ok_or("curves")?;
    let small = control["small"]["systems"].as_array().ok_or("small")?;
    if curves.len() != 28 || small.len() != 5 {
        return Err("fixed coverage".into());
    }
    let mut before = std::collections::BTreeMap::<String, u64>::new();
    let mut after = before.clone();
    let mut ids = std::collections::BTreeSet::new();
    let mut rows = vec![];
    let mut unchanged_roots = 0;
    for item in curves {
        let c = &item["case"];
        let (_, off, on) = case(&raw, c)?;
        let id = c["id"].as_str().ok_or("id")?;
        if !ids.insert(id) {
            return Err("duplicate curve case".into());
        }
        let b = kind(&off["answer"])?;
        let a = kind(&on["answer"])?;
        *before.entry(b.into()).or_default() += 1;
        *after.entry(a.into()).or_default() += 1;
        if a == "Root" && b == "Root" && off["counts"]["nodes"] == on["counts"]["nodes"] {
            unchanged_roots += 1;
        }
        if a == "Cap" && on["answer"]["Stopped"]["Cap"] != "cumulative modeled reservations" {
            return Err("unexpected cap reason".into());
        }
        rows.push(json!({"id":id,"before":b,"after":a,"before_nodes":off["counts"]["nodes"],"after_nodes":on["counts"]["nodes"],
            "after_cap_reason":on["answer"]["Stopped"]["Cap"],"dynamic_counts":on["dynamic_counts"]}));
    }
    let mut small_rows = vec![];
    for item in small {
        let c = &item["case"];
        let (_, off, on) = case(&raw, c)?;
        small_rows.push(json!({"id":c["id"],"before_answer":off["answer"],"after_answer":on["answer"],
            "before_nodes":off["counts"]["nodes"],"after_nodes":on["counts"]["nodes"],"original_truth_assignments":item["truth_assignments"],
            "original_roots":item["original_roots"],"conditional_assignments_checked":item["conditional_assignments_checked"]}));
    }
    let mut tampered = control_bytes.clone();
    tampered.push(b' ');
    fs::write(out.join("tampered-controls.json"), &tampered)?;
    if guard(&tampered, CONTROL_SHA).is_ok() {
        return Err("digest corruption accepted".into());
    }
    let result = json!({"schema":"torus.branch-affine.native-postcall/v1","standing":"producer unsigned/unverified",
        "raw_members_matched":members.len(),"old_disabled_artifacts_equal":33,"baseline_curve_outcomes":before,
        "enabled_curve_outcomes":after,"enabled_root_node_counts_equal_baseline":unchanged_roots,
        "capped_traces_count_as_gain":false,"rows":rows,"small":small_rows,"tampered_input_refused":true,
        "scope":"native record aggregation; not counter/execution/solver correctness, complete cost, prime-field or signed verification"});
    fs::write(out.join("RESULT.json"), serde_json::to_vec_pretty(&result)?)?;
    println!("native post-call audit: 247 raw member identities and 33 old/disabled comparisons checked; see RESULT.json");
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
