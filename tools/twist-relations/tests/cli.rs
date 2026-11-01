use sha2::{Digest, Sha256};
use std::{
    fs,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn temporary_folder(label: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "twist-{label}-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&path).unwrap();
    path
}

#[test]
fn later_budget_failure_retains_verified_cases_and_exact_failure_context() {
    let folder = temporary_folder("budget-failure");
    let out = folder.join("artifacts");
    let bin = env!("CARGO_BIN_EXE_twist-relations");
    let run = Command::new(bin)
        .args([
            "--suite",
            "benchmark",
            "--primes",
            "43",
            "--seeds",
            "1",
            "--seed-start",
            "9",
            "--max-attempts-per-solve",
            "1",
            "--output-dir",
        ])
        .arg(&out)
        .output()
        .unwrap();
    assert!(!run.status.success());
    let manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(out.join("manifest.json")).unwrap()).unwrap();
    assert_eq!(manifest["status"], "failed");
    assert!(!out.join("benchmark.json").exists());
    let saved = out.join("cases/00000000.json");
    assert!(Command::new(bin)
        .arg("verify")
        .arg(&saved)
        .output()
        .unwrap()
        .status
        .success());
    let failure: serde_json::Value =
        serde_json::from_slice(&fs::read(out.join("benchmark-failure.json")).unwrap()).unwrap();
    assert_eq!(failure["case"]["parameters"]["p"], 43);
    assert_eq!(failure["case"]["seed"], 9);
    assert_eq!(failure["case"]["method"], "random");
    assert_eq!(failure["progress"]["phase"], "relation_search");
    assert_eq!(failure["progress"]["attempts"], 1);
    assert!(
        failure["progress"]["counts"]["group_add_calls"]
            .as_u64()
            .unwrap()
            > 0
    );
    assert!(failure["error"]
        .as_str()
        .unwrap()
        .contains("budget exhausted"));
    let active: serde_json::Value =
        serde_json::from_slice(&fs::read(out.join("active-case.json")).unwrap()).unwrap();
    assert_eq!(active["data"]["event"], "failed");
    let journal = fs::read_to_string(out.join("benchmark-cases.jsonl")).unwrap();
    assert!(journal.ends_with('\n'));
    let events: Vec<serde_json::Value> = journal
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    assert!(events.iter().any(|e| e["data"]["event"] == "completed"));
    assert_eq!(events.last().unwrap()["data"]["event"], "failed");
    for artifact in manifest["artifacts"].as_array().unwrap() {
        let bytes = fs::read(out.join(artifact["path"].as_str().unwrap())).unwrap();
        assert_eq!(artifact["sha256"], format!("{:x}", Sha256::digest(bytes)));
    }
    fs::remove_dir_all(folder).unwrap();
}

#[test]
fn forced_process_kill_keeps_auditable_completed_prefix_without_a_pass() {
    use std::{
        io,
        process::Stdio,
        thread,
        time::{Duration, Instant},
    };
    struct Guard(std::process::Child);
    impl Drop for Guard {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
    let folder = temporary_folder("process-kill");
    let out = folder.join("artifacts");
    let bin = env!("CARGO_BIN_EXE_twist-relations");
    // Deliberately interrupt tiny known-answer correctness cases, not a timed study.
    let mut child = Guard(
        Command::new(bin)
            .args([
                "--suite",
                "benchmark",
                "--primes",
                "43",
                "--seeds",
                "10000",
                "--output-dir",
            ])
            .arg(&out)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap(),
    );
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let active = fs::read(out.join("active-case.json"))
            .ok()
            .and_then(|b| serde_json::from_slice::<serde_json::Value>(&b).ok());
        if active.as_ref().is_some_and(|a| {
            a["data"]["case"]["ordinal"]
                .as_u64()
                .is_some_and(|i| i >= 1)
        }) {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "Child never published its second case"
        );
        assert!(
            child.0.try_wait().unwrap().is_none(),
            "Child exited before interruption"
        );
        thread::sleep(Duration::from_millis(5));
    }
    match child.0.kill() {
        Ok(()) => {}
        Err(e) if e.kind() == io::ErrorKind::InvalidInput => {}
        Err(e) => panic!("{e}"),
    }
    assert!(!child.0.wait().unwrap().success());
    let manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(out.join("manifest.json")).unwrap()).unwrap();
    assert_eq!(manifest["status"], "running");
    assert!(manifest.get("finished_unix_utc").is_none());
    assert!(!out.join("benchmark.json").exists());
    let cases: Vec<_> = fs::read_dir(out.join("cases"))
        .unwrap()
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "json"))
        .collect();
    assert!(!cases.is_empty());
    for path in cases {
        assert!(Command::new(bin)
            .arg("verify")
            .arg(path)
            .output()
            .unwrap()
            .status
            .success());
    }
    let active: serde_json::Value =
        serde_json::from_slice(&fs::read(out.join("active-case.json")).unwrap()).unwrap();
    assert!(active["data"]["case"]["method"].is_string());
    let journal = fs::read_to_string(out.join("benchmark-cases.jsonl")).unwrap();
    // A kill can truncate only the final event; preceding newline records remain usable.
    for line in journal
        .split_inclusive('\n')
        .filter(|line| line.ends_with('\n'))
    {
        serde_json::from_str::<serde_json::Value>(line).unwrap();
    }
    fs::remove_dir_all(folder).unwrap();
}

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
