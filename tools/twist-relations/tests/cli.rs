use sha2::{Digest, Sha256};
use std::{
    fs,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn cli_round_trip_and_manifest_integrity_from_another_working_directory() {
    let folder = std::env::temp_dir().join(format!(
        "twist-cli-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&folder).unwrap();
    let output = folder.join("artifacts");
    let bin = env!("CARGO_BIN_EXE_twist-relations");
    let args = [
        "--suite",
        "smoke",
        "--seeds",
        "1",
        "--primes",
        "43",
        "--exhaustive-primes",
        "7",
        "--output-dir",
    ];
    let run = Command::new(bin)
        .args(args)
        .arg(&output)
        .current_dir(&folder)
        .output()
        .unwrap();
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(output.join("manifest.json")).unwrap()).unwrap();
    assert_eq!(manifest["status"], "passed");
    assert!(manifest["source"]["compiled_source_sha256"]["core.rs"].is_string());
    for artifact in manifest["artifacts"].as_array().unwrap() {
        let bytes = fs::read(output.join(artifact["path"].as_str().unwrap())).unwrap();
        assert_eq!(artifact["sha256"], format!("{:x}", Sha256::digest(bytes)));
    }
    for artifact in ["benchmark.json", "exhaustive.json"] {
        let replay = Command::new(bin)
            .arg("verify")
            .arg(output.join(artifact))
            .output()
            .unwrap();
        assert!(
            replay.status.success(),
            "{}",
            String::from_utf8_lossy(&replay.stderr)
        );
    }
    assert!(!Command::new(bin)
        .args(args)
        .arg(&output)
        .output()
        .unwrap()
        .status
        .success());
    let mut value: serde_json::Value =
        serde_json::from_slice(&fs::read(output.join("benchmark.json")).unwrap()).unwrap();
    value["records"][0]["recovered_scalar"] = serde_json::json!(0);
    fs::write(
        output.join("corrupt.json"),
        serde_json::to_vec(&value).unwrap(),
    )
    .unwrap();
    assert!(!Command::new(bin)
        .arg("verify")
        .arg(output.join("corrupt.json"))
        .output()
        .unwrap()
        .status
        .success());
    fs::remove_dir_all(folder).unwrap();
}
