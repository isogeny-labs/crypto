use std::{fs, process::Command};
fn path(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "aps-architectures-{}-{name}.json",
        std::process::id()
    ))
}
#[test]
fn actual_wrapper_retains_later_mock_failures_and_refuses_overwrite() {
    let p = path("transport");
    let _ = fs::remove_file(&p);
    let binary = env!("CARGO_BIN_EXE_aps-collection-architectures");
    let first = Command::new(binary)
        .arg(&p)
        .arg("--transport-control")
        .output()
        .unwrap();
    assert!(first.status.success());
    let bytes = fs::read(&p).unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(
        v["flow"],
        serde_json::json!({"planned":4,"attempted":4,"completed":0,"excluded":0})
    );
    assert_eq!(v["arms"].as_array().unwrap().len(), 4);
    assert!(v["arms"]
        .as_array()
        .unwrap()
        .iter()
        .all(|v| v["error"].is_string()));
    let second = Command::new(binary).arg(&p).output().unwrap();
    assert!(!second.status.success());
    assert_eq!(fs::read(&p).unwrap(), bytes);
    fs::remove_file(p).unwrap();
}
#[test]
fn invalid_option_refused_before_output_ownership() {
    let p = path("bad-option");
    let _ = fs::remove_file(&p);
    assert!(
        !Command::new(env!("CARGO_BIN_EXE_aps-collection-architectures"))
            .arg(&p)
            .arg("--bad")
            .status()
            .unwrap()
            .success()
    );
    assert!(!p.exists());
}
