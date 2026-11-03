use pencil_native_canonical_export::{digest, load};
use serde_json::{json, Value as V};
use std::{fs, path::Path};
#[test]
fn native_draft_cli_keeps_draft_and_rejects_invalid() {
    let d = tempfile::tempdir().unwrap();
    let mut p =
        load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/protocol-draft.json")).unwrap();
    p["analysis_plan"]["script"] = V::Null;
    let file = d.path().join("draft.json");
    fs::write(&file, serde_json::to_vec(&p).unwrap()).unwrap();
    let before = digest(&file).unwrap();
    let bin = env!("CARGO_BIN_EXE_pencil-native-canonical-export");
    let positive = std::process::Command::new(bin)
        .arg("validate-protocol")
        .arg(&file)
        .output()
        .unwrap();
    assert!(
        positive.status.success(),
        "{}",
        String::from_utf8_lossy(&positive.stderr)
    );
    assert_eq!(before, digest(&file).unwrap());
    let result: V = serde_json::from_slice(&positive.stdout).unwrap();
    assert_eq!(result["freeze_status"], "draft");
    assert_eq!(result["launch_authorized"], false);
    p["claims"][1]["decided_by"] = json!(["absent"]);
    fs::write(&file, serde_json::to_vec(&p).unwrap()).unwrap();
    let negative = std::process::Command::new(bin)
        .arg("validate-protocol")
        .arg(&file)
        .output()
        .unwrap();
    assert!(!negative.status.success());
}
