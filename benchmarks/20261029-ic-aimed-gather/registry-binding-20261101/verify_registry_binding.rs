//! Revalidate input custody after the canonical registry grows.
//! This performs no benchmark or scalar-recovery run and changes no old report.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{env, error::Error, fs, path::Path};

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

/// SHA-256 of a raw input: its bytes when the file is in the checkout, else
/// the hash its directory's artifact-store manifest (`.artifacts.json`,
/// `artifacts.manifest/v1`) records for it. Returns whether the manifest was
/// used, so the receipt can count those inputs separately.
fn input_digest(path: &Path) -> Result<(String, bool), Box<dyn Error>> {
    if let Ok(bytes) = fs::read(path) {
        return Ok((digest(&bytes), false));
    }
    let name = path.file_name().and_then(|n| n.to_str()).ok_or("no file name")?;
    let manifest = path.parent().ok_or("no parent")?.join(".artifacts.json");
    let m: Value = serde_json::from_slice(&fs::read(&manifest).map_err(|e| format!("{}: {e}", path.display()))?)?;
    assert_eq!(m["schema"], "artifacts.manifest/v1");
    let sha = m["files"][name]["sha256"].as_str().ok_or_else(|| format!("{} not in {}", name, manifest.display()))?;
    Ok((sha.to_string(), true))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 7, "old-registry current-registry original-report evidence-root successor-report custody-receipt");
    let old_bytes = fs::read(&args[1])?;
    let current_bytes = fs::read(&args[2])?;
    let report_bytes = fs::read(&args[3])?;
    assert_eq!(digest(&old_bytes), "53fe929b9b971fa2c74ee29fcd5cea7fca969d411ae28e8cbf413c9550f3032e");
    assert_eq!(digest(&report_bytes), "505327cf69bf6b6554244ee1214c71c87ee1331ba73dc74e1c36ebaa6388ee57");
    let old: Value = serde_json::from_slice(&old_bytes)?;
    let current: Value = serde_json::from_slice(&current_bytes)?;
    let original: Value = serde_json::from_slice(&report_bytes)?;
    let slug = "icv1-f2m53-tm56619371-dac20a85";
    let select = |registry: &Value| -> Vec<Value> {
        registry["curves"].as_array().unwrap().iter()
            .filter(|row| row["slug"] == slug).cloned().collect()
    };
    let old_target = select(&old);
    let current_target = select(&current);
    assert_eq!(old_target.len(), 1);
    assert_eq!(current_target.len(), 1);
    assert_eq!(old_target, current_target, "complete target record changed");
    let mut successor = original.clone();
    let mut changed = 0;
    let mut verified = 0;
    let mut via_manifest = 0u64;
    for row in successor["raw_data"].as_array_mut().unwrap() {
        let relative = row["path"].as_str().unwrap();
        let path = Path::new(&args[4]).join(relative);
        if relative == "../../docs/curves/registry.json" {
            let bytes = fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            assert_eq!(row["sha256"], digest(&old_bytes));
            assert_eq!(bytes, current_bytes);
            row["sha256"] = json!(digest(&current_bytes));
            changed += 1;
        } else {
            let (sha, from_manifest) = input_digest(&path)?;
            assert_eq!(row["sha256"], sha, "raw input changed: {relative}");
            verified += 1;
            via_manifest += u64::from(from_manifest);
        }
    }
    assert_eq!(changed, 1);
    // Prove that the inherited scientific fields and every other raw binding
    // are unchanged; the original measurement still has its original source.
    let mut restored = successor.clone();
    for row in restored["raw_data"].as_array_mut().unwrap() {
        if row["path"] == "../../docs/curves/registry.json" {
            row["sha256"] = json!(digest(&old_bytes));
        }
    }
    assert_eq!(restored, original);
    let custody = json!({
        "schema": "crypto.aimed-gather.registry-source-equivalence/v1",
        "checked_on": "2026-11-01",
        "original_registry_commit": "ddaba2819f4ee96139c4145fbed905db1c1db46e",
        "original_registry_sha256": digest(&old_bytes),
        "original_registry_rows": old["curves"].as_array().unwrap().len(),
        "current_registry_sha256": digest(&current_bytes),
        "current_registry_rows": current["curves"].as_array().unwrap().len(),
        "original_report_sha256": digest(&report_bytes),
        "target_slug": slug,
        "old_target_matches": old_target.len(),
        "current_target_matches": current_target.len(),
        "complete_target_record_equal": true,
        "other_raw_input_hashes_verified": verified,
        "other_raw_input_hashes_verified_via_store_manifest": via_manifest,
        "predecessor": "registry-binding-20261030 (registry 20,844 rows)",
        "changed_raw_bindings": changed,
        "inherited_scientific_fields_equal": true,
        "scope": "The original native report and performance measurements remain historical and unchanged. This successor validates the same unique complete target record and all unchanged raw inputs against the expanded current registry. No benchmark, scalar replay or timing measurement was executed."
    });
    successor["current_registry_source_equivalence"] = custody.clone();
    fs::write(&args[5], serde_json::to_string_pretty(&successor)? + "\n")?;
    fs::write(&args[6], serde_json::to_string_pretty(&custody)? + "\n")?;
    println!("PASS: unique complete target record equal; {verified} other raw pins verified; exactly one registry binding revalidated; all scientific fields unchanged");
    Ok(())
}
