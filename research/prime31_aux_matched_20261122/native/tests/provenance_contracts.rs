use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use prime31_aux_matched::provenance::check_manifest_files;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Fixture {
    root: PathBuf,
    manifest: Value,
}
impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "prime31-source-contract-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        fs::create_dir(root.join("source")).unwrap();
        let content = b"exact production bytes\n";
        let hash = format!("{:x}", Sha256::digest(content));
        for file in ["study.rs", "selected.rs", "f4.rs"] {
            fs::write(root.join("source").join(file), content).unwrap();
        }
        let manifest = json!({
            "schema":"prime31-aux-matched-source-manifest/v1",
            "whole_files":[{"path":"source/study.rs","sha256":hash,"bytes":content.len()}],
            "extracted_production_ranges":[{"path":"source/selected.rs","whole_source_sha256":hash,
                "pinned_whole_source_sha256":hash,"whole_source_bytes":content.len(),"rewritten":false,"inserted_bytes":0}],
            "layout_preserving_f4_copy":{"copied_files":[{"path":"source/f4.rs",
                "source_and_copy_sha256":hash,"bytes":content.len(),"rewritten":false}]}
        });
        Self { root, manifest }
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn whole_selected_and_f4_sources_are_all_verified() {
    let fixture = Fixture::new();
    assert_eq!(
        check_manifest_files(&fixture.root, &fixture.manifest).unwrap()["checked_file_count"],
        3
    );
    for path in ["source/study.rs", "source/selected.rs", "source/f4.rs"] {
        let original = fs::read(fixture.root.join(path)).unwrap();
        let mut changed = original.clone();
        changed[0] ^= 1;
        fs::write(fixture.root.join(path), changed).unwrap();
        assert!(check_manifest_files(&fixture.root, &fixture.manifest)
            .unwrap_err()
            .contains(path));
        fs::write(fixture.root.join(path), original).unwrap();
    }
}

#[test]
fn missing_files_conflicting_pins_and_path_escapes_fail_closed() {
    let fixture = Fixture::new();
    let mut malformed = fixture.manifest.clone();
    malformed["whole_files"][0]["path"] = json!("../outside.rs");
    assert!(check_manifest_files(&fixture.root, &malformed).is_err());
    let mut malformed = fixture.manifest.clone();
    malformed["extracted_production_ranges"][0]["pinned_whole_source_sha256"] =
        json!("0".repeat(64));
    assert!(check_manifest_files(&fixture.root, &malformed).is_err());
    let mut malformed = fixture.manifest.clone();
    let duplicate = json!({"path":"source/study.rs","sha256":"0".repeat(64),"bytes":23});
    malformed["whole_files"]
        .as_array_mut()
        .unwrap()
        .push(duplicate);
    assert!(check_manifest_files(&fixture.root, &malformed).is_err());
    fs::remove_file(fixture.root.join("source/f4.rs")).unwrap();
    assert!(check_manifest_files(&fixture.root, &fixture.manifest).is_err());
}
