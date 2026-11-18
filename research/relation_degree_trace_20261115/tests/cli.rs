use std::{fs, process::Command};
fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_relation-degree-trace"))
}
fn temp(label: &str) -> std::path::PathBuf {
    let p = std::env::temp_dir().join(format!("degree-cli-{}-{label}", std::process::id()));
    if p.exists() {
        fs::remove_dir_all(&p).unwrap();
    }
    p
}
#[test]
fn capped_custom_input_exports_null_and_plot() {
    let root = temp("capped");
    fs::create_dir_all(&root).unwrap();
    let input = root.join("input.json");
    let out = root.join("out");
    fs::write(
        &input,
        r#"{"prime":101,"variables":["x"],"polynomials":[[{"exponents":[1],"coefficient":1}]]}"#,
    )
    .unwrap();
    let result = binary()
        .args([
            "--input",
            input.to_str().unwrap(),
            "--out",
            out.to_str().unwrap(),
            "--max-degree",
            "0",
            "--candidate-limit",
            "0",
        ])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(out.join("trace.json")).unwrap()).unwrap();
    assert!(report["reports"][0]["baseline"]["solving_degree"].is_null());
    assert_eq!(report["reports"][0]["baseline"]["status"], "degree_cap");
    assert!(fs::read_to_string(out.join("relation-effects.svg"))
        .unwrap()
        .contains("cap"));
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn refuses_overwrite() {
    let out = temp("overwrite");
    fs::create_dir_all(&out).unwrap();
    fs::write(out.join("sentinel"), "unchanged").unwrap();
    let result = binary()
        .args(["--out", out.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert_eq!(
        fs::read_to_string(out.join("sentinel")).unwrap(),
        "unchanged"
    );
    fs::remove_dir_all(out).unwrap();
}
#[test]
fn composite_import_is_rejected() {
    let root = temp("composite");
    fs::create_dir_all(&root).unwrap();
    let input = root.join("input.json");
    let out = root.join("out");
    fs::write(&input,r#"{"prime":9007199254740992,"variables":["x"],"polynomials":[[{"exponents":[1],"coefficient":1}]]}"#).unwrap();
    let result = binary()
        .args([
            "--input",
            input.to_str().unwrap(),
            "--out",
            out.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(!out.exists());
    fs::remove_dir_all(root).unwrap();
}
