use std::{fs, process::Command};

#[test]
fn existing_output_is_refused_before_a_solver_call_and_left_unchanged() {
    let path =
        std::env::temp_dir().join(format!("aps-partition-bridge-{}.json", std::process::id()));
    fs::write(&path, b"sentinel").unwrap();
    for binary in [
        env!("CARGO_BIN_EXE_aps-partition-bridge"),
        env!("CARGO_BIN_EXE_quartic"),
        env!("CARGO_BIN_EXE_encoding-compare"),
        env!("CARGO_BIN_EXE_four-leads"),
    ] {
        let mode = if binary == env!("CARGO_BIN_EXE_four-leads") {
            "aux_domains"
        } else {
            "structural"
        };
        let result = Command::new(binary)
            .args([
                mode,
                "--source-revision",
                &"a".repeat(40),
                "--source-manifest-sha256",
                &"b".repeat(64),
                "--out",
            ])
            .arg(&path)
            .output()
            .unwrap();
        assert!(!result.status.success());
        assert_eq!(fs::read(&path).unwrap(), b"sentinel");
    }
    fs::remove_file(path).unwrap();
}

#[test]
fn missing_source_pins_cannot_start_the_fixed_panel() {
    for binary in [
        env!("CARGO_BIN_EXE_aps-partition-bridge"),
        env!("CARGO_BIN_EXE_quartic"),
        env!("CARGO_BIN_EXE_encoding-compare"),
        env!("CARGO_BIN_EXE_four-leads"),
    ] {
        let result = Command::new(binary).arg("structural").output().unwrap();
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stderr).contains("usage:"));
    }
}
