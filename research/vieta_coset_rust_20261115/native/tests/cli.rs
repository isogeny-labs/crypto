use serde_json::Value;
use std::process::Command;

fn check_rows() -> Vec<Value> {
    let output = Command::new(env!("CARGO_BIN_EXE_vieta-coset-lab"))
        .args(["check", "word17", "8", "2", "30", "three"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect()
}

#[test]
fn full_cli_controls_and_no_check_mode_timings() {
    let rows = check_rows();
    let two: Vec<_> = rows.iter().filter(|r| r["kind"] == "query").collect();
    let three: Vec<_> = rows.iter().filter(|r| r["kind"] == "three-query").collect();
    assert_eq!(two.len(), 16);
    assert_eq!(three.len(), 16);
    for group in [two, three] {
        for arms in group.as_chunks::<4>().0 {
            for r in arms {
                assert_eq!(r["status"], "ok");
                assert!(r["elapsed_ns"].is_null());
                assert_eq!(r["result_sha256"], arms[0]["result_sha256"]);
                assert_eq!(r["relations"], arms[0]["relations"]);
                if r["population"].as_str().unwrap().starts_with("conditioned") {
                    assert!(r["relations"].as_u64().unwrap() > 0);
                }
            }
        }
    }
}

#[test]
fn replay_recomputes_and_rejects_tampered_digest() {
    let mut rows = check_rows();
    let path =
        std::env::temp_dir().join(format!("vieta-coset-replay-{}.jsonl", std::process::id()));
    let write = |rows: &[Value]| {
        std::fs::write(
            &path,
            rows.iter().map(|r| format!("{r}\n")).collect::<String>(),
        )
        .unwrap()
    };
    write(&rows);
    let invoke = || {
        Command::new(env!("CARGO_BIN_EXE_vieta-coset-replay"))
            .arg(&path)
            .output()
            .unwrap()
    };
    let output = invoke();
    assert!(output.status.success());
    let receipt: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(receipt["checked"], 16);
    rows.iter_mut().find(|r| r["kind"] == "query").unwrap()["result_sha256"] =
        Value::String("00".repeat(32));
    write(&rows);
    assert!(!invoke().status.success());
    std::fs::remove_file(path).unwrap();
}

#[test]
fn admission_guard_and_invalid_coset_fail_closed() {
    let output = Command::new(env!("CARGO_BIN_EXE_vieta-coset-lab"))
        .env_remove("VIETA_ISOLATED_RUN")
        .args(["bench", "word17", "8", "2"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let output = Command::new(env!("CARGO_BIN_EXE_vieta-coset-lab"))
        .args(["check", "word17", "3", "2"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let row: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(row["kind"], "failure");
    assert_eq!(row["argv"][3], "3");
}
