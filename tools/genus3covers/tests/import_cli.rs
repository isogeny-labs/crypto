use std::io::Write;
use std::process::{Command, Stdio};

const VALID_ROW: &str = "0:2:2:[[0,1,-4,10,4,1],[]]:256:[0,1,0,-3,1]:1:[0,1]:0.050";

#[test]
fn deep_nesting_returns_validation_error_without_partial_output() {
    let curve = format!("{}0{}", "[".repeat(50_000), "]".repeat(50_000));
    let malformed = VALID_ROW.replace("[[0,1,-4,10,4,1],[]]", &curve);
    for (input, line) in [
        (format!("{malformed}\n"), 1),
        (format!("{VALID_ROW}\n{malformed}\n"), 2),
    ] {
        let mut child = Command::new(env!("CARGO_BIN_EXE_genus3covers-import"))
            .args(["import", "-"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(input.as_bytes())
            .unwrap();
        let output = child.wait_with_output().unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        let error = String::from_utf8(output.stderr).unwrap();
        assert!(error.starts_with(&format!("line {line}:")), "{error}");
        assert!(error.contains("at most two levels"), "{error}");
    }
}

#[test]
fn pinned_example_imports_every_row() {
    let output = Command::new(env!("CARGO_BIN_EXE_genus3covers-import"))
        .args([
            "import",
            concat!(env!("CARGO_MANIFEST_DIR"), "/upstream/P2example.txt"),
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let rows = String::from_utf8(output.stdout).unwrap();
    assert_eq!(rows.lines().count(), 654);
    assert!(rows
        .lines()
        .last()
        .unwrap()
        .starts_with("{\"source_line\":654,"));
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        "validated 654 rows\n"
    );
}
