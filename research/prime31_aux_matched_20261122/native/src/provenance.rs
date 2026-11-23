//! Bind the running executable's embedded source inventory to the checkout.
//! This checks source custody, not compiler reproducibility or execution data.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path};
use std::process::Command;

use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use crate::SOURCE_MANIFEST;

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn string<'a>(value: &'a Value, field: &str) -> Result<&'a str, String> {
    value[field]
        .as_str()
        .ok_or_else(|| format!("source manifest lacks string {field}"))
}

fn array<'a>(value: &'a Value, field: &str) -> Result<&'a Vec<Value>, String> {
    value[field]
        .as_array()
        .ok_or_else(|| format!("source manifest lacks array {field}"))
}

fn register(
    files: &mut BTreeMap<String, (String, u64)>,
    value: &Value,
    hash_field: &str,
    size_field: &str,
) -> Result<(), String> {
    let path = string(value, "path")?;
    if path.is_empty()
        || Path::new(path)
            .components()
            .any(|c| !matches!(c, Component::Normal(_)))
    {
        return Err(format!(
            "source inventory path must be repository-relative: {path}"
        ));
    }
    let hash = string(value, hash_field)?;
    if hash.len() != 64 || !hash.bytes().all(|c| c.is_ascii_hexdigit()) {
        return Err(format!("invalid source SHA256 for {path}"));
    }
    let size = value[size_field]
        .as_u64()
        .ok_or_else(|| format!("source inventory lacks byte count for {path}"))?;
    let expected = (hash.to_ascii_lowercase(), size);
    if let Some(previous) = files.insert(path.to_string(), expected.clone()) {
        if previous != expected {
            return Err(format!("conflicting source inventory entries for {path}"));
        }
    }
    Ok(())
}

/// Check an explicit inventory against files. Exposed separately so stale,
/// missing and malformed source inventories can be tested on tiny fixtures.
/// This helper alone makes no claim that files are committed or admitted.
pub fn check_manifest_files(root: &Path, manifest: &Value) -> Result<Value, String> {
    if manifest["schema"] != "prime31-aux-matched-source-manifest/v1" {
        return Err("unknown source manifest schema".into());
    }
    let root = root.canonicalize().map_err(|e| e.to_string())?;
    let mut files = BTreeMap::new();
    for entry in array(manifest, "whole_files")? {
        register(&mut files, entry, "sha256", "bytes")?;
    }
    for entry in array(manifest, "extracted_production_ranges")? {
        if string(entry, "whole_source_sha256")? != string(entry, "pinned_whole_source_sha256")?
            || entry["rewritten"] != false
            || entry["inserted_bytes"] != 0
        {
            return Err("source selection is not pinned verbatim production content".into());
        }
        register(
            &mut files,
            entry,
            "whole_source_sha256",
            "whole_source_bytes",
        )?;
    }
    for entry in array(&manifest["layout_preserving_f4_copy"], "copied_files")? {
        if entry["rewritten"] != false {
            return Err("F4 source copy is not verbatim".into());
        }
        register(&mut files, entry, "source_and_copy_sha256", "bytes")?;
    }
    if files.is_empty() {
        return Err("source manifest has no files".into());
    }
    let mut checked = Vec::new();
    for (relative, (expected_hash, expected_bytes)) in &files {
        let path = root
            .join(relative)
            .canonicalize()
            .map_err(|e| format!("cannot resolve source {relative}: {e}"))?;
        if !path.starts_with(&root) || !path.is_file() {
            return Err(format!(
                "source path escapes checkout or is not a file: {relative}"
            ));
        }
        let bytes = fs::read(&path).map_err(|e| format!("cannot read {relative}: {e}"))?;
        let actual = sha256(&bytes);
        if actual != *expected_hash || bytes.len() as u64 != *expected_bytes {
            return Err(format!(
                "source content differs from running binary: {relative}"
            ));
        }
        checked.push(json!({"path":relative,"sha256":actual,"bytes":bytes.len()}));
    }
    Ok(json!({"checked_files":checked,"checked_file_count":files.len()}))
}

/// Verify the running executable's full inventory, and require every recorded
/// input to exist in HEAD. The launcher separately requires a clean checkout.
pub fn check_sources(root: &Path) -> Result<Value, String> {
    let manifest: Value = serde_json::from_str(SOURCE_MANIFEST).map_err(|e| e.to_string())?;
    let root = root.canonicalize().map_err(|e| e.to_string())?;
    let mut result = check_manifest_files(&root, &manifest)?;
    let paths: Vec<_> = array(&result, "checked_files")?
        .iter()
        .map(|entry| string(entry, "path"))
        .collect::<Result<_, _>>()?;
    for required in [
        "research/prime31_aux_matched_20261122/native/Cargo.lock",
        "research/prime31_aux_matched_20261122/native/build.rs",
        "research/prime31_aux_matched_20261122/native/src/main.rs",
        "research/prime31_aux_matched_20261122/native/src/provenance.rs",
        "research/prime31_aux_matched_20261122/isogeny-choice/upstream/manifest.json",
        "research/prime31_aux_matched_20261122/isogeny-choice/upstream/f4_fp.rs",
        "research/prime31_aux_matched_20261122/isogeny-choice/upstream/ic_boundary.rs",
        "research/prime31_aux_matched_20261122/isogeny-choice/upstream/prime_fast.rs",
        "research/prime31_aux_matched_20261122/isogeny-choice/upstream/groebner_f4.rs",
        "research/prime31_aux_matched_20261122/isogeny-choice/fixtures/kernel_correctness_fixtures.json",
        "src/cryptanalysis/f4_fp/reuse.rs",
        "src/cryptanalysis/f4_fp/pair_certificate.rs",
        "src/cryptanalysis/f4_fp/certified.rs",
    ] {
        if !paths.contains(&required) {
            return Err(format!(
                "required source absent from embedded inventory: {required}"
            ));
        }
    }
    let tree = Command::new("git")
        .arg("-C")
        .arg(&root)
        .args([
            "ls-tree",
            "-r",
            "--name-only",
            "-z",
            "--full-tree",
            "HEAD",
            "--",
        ])
        .args(&paths)
        .output()
        .map_err(|e| format!("cannot check tracked sources: {e}"))?;
    if !tree.status.success() {
        return Err("git could not enumerate HEAD source inventory".into());
    }
    let tracked: BTreeSet<_> = tree
        .stdout
        .split(|&b| b == 0)
        .filter(|b| !b.is_empty())
        .map(|b| {
            std::str::from_utf8(b)
                .map(str::to_owned)
                .map_err(|e| e.to_string())
        })
        .collect::<Result<_, _>>()?;
    if let Some(path) = paths.iter().find(|path| !tracked.contains(**path)) {
        return Err(format!("source input is absent from HEAD: {path}"));
    }
    let revision = Command::new("git")
        .arg("-C")
        .arg(&root)
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(|e| e.to_string())?;
    if !revision.status.success() {
        return Err("git HEAD is unavailable".into());
    }
    let revision = String::from_utf8(revision.stdout).map_err(|e| e.to_string())?;
    let revision = revision.trim();
    if revision.len() != 40 || !revision.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err("git HEAD is not a full SHA1 revision".into());
    }
    // Linux /proc resolves the running executable inode, even if its original
    // filename was replaced. Hashing the filename alone would miss that race.
    let executable_bytes = fs::read("/proc/self/exe")
        .map_err(|e| format!("cannot hash running Linux executable: {e}"))?;
    result["schema"] = json!("prime31-aux-matched.source-check/v1");
    result["verified"] = json!(true);
    result["source_revision"] = json!(revision);
    result["source_manifest_sha256"] = json!(sha256(SOURCE_MANIFEST.as_bytes()));
    result["running_binary_sha256"] = json!(sha256(&executable_bytes));
    result["all_inventoried_sources_present_in_head"] = json!(true);
    result["cargo_profile"] = manifest["cargo_profile"].clone();
    result["opt_level"] = manifest["opt_level"].clone();
    result["root"] = json!(root);
    result["scope"] = json!("Embedded source bytes match checkout and every inventoried file is in HEAD. Launcher must also enforce clean checkout and input custody. Compiler, registry package bodies and external admission are not verified here.");
    Ok(result)
}
