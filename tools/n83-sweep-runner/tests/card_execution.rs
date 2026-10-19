use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};
#[test]
fn card_report_preserves_original_float_bytes() {
    let (p, card, _) = setup("/usr/bin/true", vec![], 1);
    let raw =
        b"{\n  \"card_id\": \"CARD-n83-native-test-1\",\n  \"seconds\": 1.1790702610000001\n}\n";
    fs::write(p.join("exec/receipt.json"), raw).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_n83-sweep-runner"))
        .current_dir(&p)
        .arg("card-report")
        .arg(&card)
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, raw);
    fs::remove_dir_all(p).unwrap();
}
fn git(p: &PathBuf, args: &[&str]) {
    assert!(Command::new("git")
        .arg("-C")
        .arg(p)
        .args(args)
        .output()
        .unwrap()
        .status
        .success());
}
fn setup(program: &str, args: Vec<&str>, timeout: u64) -> (PathBuf, PathBuf, Value) {
    let p = std::env::temp_dir().join(format!(
        "n83-card-test-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir(&p).unwrap();
    fs::write(p.join(".gitignore"), "exec/\n").unwrap();
    git(&p, &["init", "-q"]);
    git(&p, &["add", ".gitignore"]);
    git(
        &p,
        &[
            "-c",
            "user.name=NativeTest",
            "-c",
            "user.email=native@example.invalid",
            "commit",
            "-qm",
            "fixture",
        ],
    );
    let head = String::from_utf8(
        Command::new("git")
            .arg("-C")
            .arg(&p)
            .args(["rev-parse", "HEAD"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .trim()
    .to_owned();
    fs::create_dir(p.join("exec")).unwrap();
    let mut argv = vec![program];
    argv.extend(args);
    let c = json!({"schema":"crypto.autoresearch.mechanical-task-card.v1","card_id":"CARD-n83-native-test-1","experiment_id":"n83-native-test","author_role":"experiment-director","executor_role":"executor-mechanical","kind":"frozen_command","bus_address":"n83-test","background":false,"write_scope":["exec"],"stop_when":["Stop on a failed command."],"commit":head,"receipt":"exec/receipt.json","binds":[{"path":program,"sha256":format!("{:x}",Sha256::digest(fs::read(program).unwrap()))}],"must_not_exist":["exec/receipt.json"],"commands":[{"id":"native-test","argv":argv,"timeout_seconds":timeout,"success_exit_codes":[0],"stdout":"exec/stdout.txt","stderr":"exec/stderr.txt","outputs":[]}]});
    let card = p.join("exec/card.json");
    fs::write(&card, serde_json::to_vec(&c).unwrap()).unwrap();
    (p, card, c)
}
fn run(p: &PathBuf, card: &PathBuf) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_n83-sweep-runner"))
        .args(["card-run", card.to_str().unwrap()])
        .current_dir(p)
        .output()
        .unwrap()
}
#[test]
fn native_card_capture_and_write_once() {
    let (p, card, _) = setup("/usr/bin/printf", vec!["observed\\n"], 2);
    let o = run(&p, &card);
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    let r: Value = serde_json::from_slice(&o.stdout).unwrap();
    assert_eq!(r["status"], "completed");
    assert_eq!(r["commands"][0]["exit_code"], 0);
    assert_eq!(
        fs::read_to_string(p.join("exec/stdout.txt")).unwrap(),
        "observed\n"
    );
    let old = fs::read(p.join("exec/receipt.json")).unwrap();
    assert!(!run(&p, &card).status.success());
    assert_eq!(old, fs::read(p.join("exec/receipt.json")).unwrap());
    fs::remove_dir_all(p).unwrap();
}
#[test]
fn altered_binding_refuses_before_execute() {
    let (p, card, mut c) = setup("/usr/bin/printf", vec!["bad"], 2);
    c["binds"][0]["sha256"] = json!("0".repeat(64));
    fs::write(&card, serde_json::to_vec(&c).unwrap()).unwrap();
    assert!(!run(&p, &card).status.success());
    assert!(!p.join("exec/receipt.json").exists());
    assert!(!p.join("exec/stdout.txt").exists());
    fs::remove_dir_all(p).unwrap();
}
#[test]
fn deadline_retains_failure_receipt() {
    let (p, card, _) = setup("/usr/bin/sleep", vec!["5"], 1);
    let o = run(&p, &card);
    assert!(!o.status.success());
    let r: Value = serde_json::from_slice(&fs::read(p.join("exec/receipt.json")).unwrap()).unwrap();
    assert_eq!(r["status"], "stopped at command 1");
    assert_eq!(r["commands"][0]["timed_out"], true);
    assert_eq!(r["commands"][0]["succeeded"], false);
    fs::remove_dir_all(p).unwrap();
}

#[test]
fn spawn_error_writes_stopped_receipt() {
    let (p, card, mut c) = setup("/usr/bin/printf", vec!["unused"], 2);
    let nonexec = p.join(".gitignore");
    c["commands"][0]["argv"] = json!([nonexec]);
    c["binds"] = json!([{"path":nonexec,"sha256":format!("{:x}",Sha256::digest(fs::read(&nonexec).unwrap()))}]);
    fs::write(&card, serde_json::to_vec(&c).unwrap()).unwrap();
    let o = run(&p, &card);
    assert_eq!(o.status.code(), Some(3));
    let r: Value = serde_json::from_slice(&fs::read(p.join("exec/receipt.json")).unwrap()).unwrap();
    assert_eq!(r["commands"][0]["succeeded"], false);
    assert!(r["commands"][0]["execution_error"].is_string());
    fs::remove_dir_all(p).unwrap();
}
#[test]
fn card_edit_during_execution_retains_original_hash() {
    let (p, card, mut c) = setup("/usr/bin/sleep", vec!["2"], 5);
    let original = format!("{:x}", Sha256::digest(fs::read(&card).unwrap()));
    let child = Command::new(env!("CARGO_BIN_EXE_n83-sweep-runner"))
        .args(["card-run", card.to_str().unwrap()])
        .current_dir(&p)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let start = std::time::Instant::now();
    while !p.join("exec/stdout.txt").exists() {
        assert!(start.elapsed().as_secs() < 5);
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    c["notes"] = json!("changed after load");
    fs::write(&card, serde_json::to_vec(&c).unwrap()).unwrap();
    assert_eq!(child.wait_with_output().unwrap().status.code(), Some(3));
    let r: Value = serde_json::from_slice(&fs::read(p.join("exec/receipt.json")).unwrap()).unwrap();
    assert_eq!(r["card_sha256"], original);
    assert_eq!(r["card_changed"], true);
    fs::remove_dir_all(p).unwrap();
}

#[test]
fn relay_verification_rejects_stream_edit() {
    let (p, card, _) = setup("/usr/bin/printf", vec!["observed\\n"], 2);
    let o = run(&p, &card);
    assert!(o.status.success());
    let relay = p.join("exec/relay.txt");
    fs::write(&relay, &o.stdout).unwrap();
    let check = || {
        Command::new(env!("CARGO_BIN_EXE_n83-sweep-runner"))
            .args([
                "card-check",
                card.to_str().unwrap(),
                relay.to_str().unwrap(),
            ])
            .current_dir(&p)
            .output()
            .unwrap()
    };
    assert!(check().status.success());
    fs::write(p.join("exec/stdout.txt"), "forged\n").unwrap();
    assert!(!check().status.success());
    fs::remove_dir_all(p).unwrap();
}
