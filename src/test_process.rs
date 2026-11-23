//! Fresh-process controls for tests whose contract requires exclusive process state.

/// Return true after the parent has run exactly one successful child test.
/// In the selected child, return false so the original assertions execute.
/// A missing test and a failed child are both failures, never successful skips.
pub(crate) fn parent_runs_isolated_case(name: &str) -> bool {
    const MARKER: &str = "CRYPTO_TEST_ISOLATED_CASE";
    if std::env::var(MARKER).as_deref() == Ok(name) {
        return false;
    }
    let output = std::process::Command::new(std::env::current_exe().expect("unit-test executable"))
        .args(["--exact", name, "--nocapture", "--test-threads=1"])
        .env(MARKER, name)
        .output()
        .expect("start isolated unit-test case");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "isolated case {name} failed: {}\n{stdout}\n{stderr}",
        output.status
    );
    assert!(
        stdout.lines().any(|line| line.trim() == "running 1 test"),
        "isolated case {name} did not run exactly one test:\n{stdout}\n{stderr}"
    );
    true
}
