//! Exact source selection for an isolated maintained incumbent. No arithmetic
//! or solver runs during this build; its manifest is source provenance only.
use std::{env, fs, path::PathBuf};

use serde_json::{json, Value};
use sha2::{Digest, Sha256};

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn main() {
    let own = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let root = own
        .join("../../../..")
        .canonicalize()
        .expect("repository root");
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let relative =
        "research/prime31_aux_matched_20261122/fiber-domain-followup/upstream/manifest.json";
    let manifest_path = root.join(relative);
    println!("cargo:rerun-if-changed={}", manifest_path.display());
    let manifest: Value = serde_json::from_slice(&fs::read(&manifest_path).unwrap()).unwrap();
    assert_eq!(
        manifest["schema"],
        "prime31-specialized-incumbent-upstream/v1"
    );
    assert_eq!(
        manifest["upstream_revision"],
        "457ae1818aaf8134bd833ad9c1b9d57015264510"
    );
    let snapshots = manifest["snapshots"].as_array().unwrap();
    assert_eq!(snapshots.len(), 4);
    let selections = manifest["selections"].as_array().unwrap();
    assert_eq!(selections.len(), 4);
    let mut receipts = Vec::new();
    for selection in selections {
        let relative = selection["path"].as_str().unwrap();
        let path = root.join(relative);
        println!("cargo:rerun-if-changed={}", path.display());
        let bytes = fs::read(&path).expect("source snapshot");
        let snapshot = snapshots.iter().find(|s| s["path"] == relative).unwrap();
        assert_eq!(
            digest(&bytes),
            snapshot["sha256"].as_str().unwrap(),
            "snapshot changed: {relative}"
        );
        assert_eq!(bytes.len() as u64, snapshot["bytes"].as_u64().unwrap());
        let source = std::str::from_utf8(&bytes).unwrap();
        let mut generated = Vec::new();
        let mut ranges = Vec::new();
        let mut previous_end = 0;
        for range in selection["ranges"].as_array().unwrap() {
            let anchor = |key: &str| {
                let needle = range[key].as_str().unwrap();
                let found: Vec<_> = source.match_indices(needle).collect();
                assert_eq!(
                    found.len(),
                    1,
                    "nonunique/missing {key} in {relative}: {needle}"
                );
                found[0].0
            };
            let start = anchor("start_anchor");
            let end = anchor("end_anchor");
            assert!(previous_end <= start && start < end);
            previous_end = end;
            let generated_start = generated.len();
            generated.extend_from_slice(&bytes[start..end]);
            ranges.push(json!({
                "scope":range["scope"], "start_anchor":range["start_anchor"], "end_anchor":range["end_anchor"],
                "byte_start_inclusive":start, "byte_end_exclusive":end,
                "generated_byte_start_inclusive":generated_start, "generated_byte_end_exclusive":generated.len(),
                "selected_sha256":digest(&bytes[start..end]), "selected_bytes":end-start, "rewritten":false
            }));
        }
        let name = selection["generated_name"].as_str().unwrap();
        fs::write(out.join(name), &generated).unwrap();
        let written = fs::read(out.join(name)).unwrap();
        assert_eq!(written, generated);
        for range in &ranges {
            let at = |key: &str| range[key].as_u64().unwrap() as usize;
            assert_eq!(
                &written[at("generated_byte_start_inclusive")..at("generated_byte_end_exclusive")],
                &bytes[at("byte_start_inclusive")..at("byte_end_exclusive")]
            );
        }
        receipts.push(json!({
            "path":relative, "whole_source_sha256":digest(&bytes), "whole_source_bytes":bytes.len(),
            "git_blob":snapshot["git_blob"], "generated_name":name, "generated_sha256":digest(&generated),
            "generated_bytes":generated.len(), "ranges":ranges, "inserted_bytes":0, "rewritten":false
        }));
    }
    let receipt = json!({
        "schema":"prime31-specialized-incumbent-source/v1",
        "repository":manifest["repository"], "revision":manifest["upstream_revision"],
        "source_manifest_sha256":digest(&fs::read(manifest_path).unwrap()),
        "extracted_production_ranges":receipts, "algorithm_substitutions":false,
        "upstream_module_tests_included":false, "build_is_not_execution_evidence":true
    });
    fs::write(
        out.join("source-manifest.json"),
        serde_json::to_vec_pretty(&receipt).unwrap(),
    )
    .unwrap();
}
