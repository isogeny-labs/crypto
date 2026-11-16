use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};
fn sha(v: &[u8]) -> String {
    format!("{:x}", Sha256::digest(v))
}
fn execute(config: &Path, check: bool) -> std::process::Output {
    let mut c = Command::new(env!("CARGO_BIN_EXE_isolab-fresh-owner"));
    if check {
        c.arg("--check");
    }
    c.arg(config)
        .arg(sha(&fs::read(config).unwrap()))
        .output()
        .unwrap()
}
#[test]
fn signal_missing_binding_and_receipt_identity_are_not_accepted() {
    let root = std::env::temp_dir().join(format!("fresh-boundaries-{}", std::process::id()));
    fs::create_dir(&root).unwrap();
    let spec = root.join("spec.json");
    fs::write(&spec, b"{}").unwrap();
    let binary = env!("CARGO_BIN_EXE_owner-fixture");
    for (label, phase, flag, exit) in [
        ("signal", "replay", "--fixture-signal", 143),
        ("failure", "replay", "--fixture-failure", 7),
        ("success", "replay", "--fixture-ok", 0),
        ("missing", "run", "--fixture-ok", 0),
    ] {
        let receipt = root.join(format!("{label}-receipt.json"));
        let config = root.join(format!("{label}-config.json"));
        let argv = if phase == "run" {
            json!([
                "run",
                "--spec",
                spec,
                "--out",
                root,
                "--diagnostic-launch",
                spec,
                flag
            ])
        } else {
            json!(["verify", "--dir", root, "--replay-all", flag])
        };
        let mut cfg = json!({"schema":"isolab.fresh-owner-config/v1","phase":phase,"request_sha256":"a".repeat(64),"launch_sha256":sha(b"{}"),"launch":spec,"pod_uid":"fixture-pod","job_uid":"fixture-job","image_digest":"b".repeat(64),"binary_sha256":sha(&fs::read(binary).unwrap()),"binary":binary,"argv":argv,"spec":spec,"spec_sha256":sha(b"{}"),"session":root,"receipt":receipt,"stop_utc_seconds":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()+10});
        fs::write(&config, serde_json::to_vec(&cfg).unwrap()).unwrap();
        let result = execute(&config, false);
        assert_eq!(
            result.status.code(),
            Some(exit),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let v: Value = serde_json::from_slice(&fs::read(&receipt).unwrap()).unwrap();
        assert_eq!(v["exit_code"], exit);
        assert!(v["total_active_worker_ns"].is_null());
        if label == "success" {
            let checked = execute(&config, true);
            assert!(
                checked.status.success(),
                "{}",
                String::from_utf8_lossy(&checked.stderr)
            );
            cfg["pod_uid"] = json!("different-pod");
            fs::write(&config, serde_json::to_vec(&cfg).unwrap()).unwrap();
            assert!(!execute(&config, true).status.success());
        } else {
            assert!(!execute(&config, true).status.success());
        }
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn owner_and_literal_wrapper_signal_paths_reap_before_returning() {
    let root = std::env::temp_dir().join(format!("fresh-signal-path-{}", std::process::id()));
    fs::create_dir(&root).unwrap();
    let spec = root.join("spec.json");
    fs::write(&spec, b"{}").unwrap();
    let binary = env!("CARGO_BIN_EXE_owner-fixture");
    for wrapper in [false, true] {
        let receipt = root.join(format!("{wrapper}-receipt.json"));
        let config = root.join(format!("{wrapper}-config.json"));
        let marker = root.join(format!("{wrapper}-ready"));
        let cfg = json!({"schema":"isolab.fresh-owner-config/v1","phase":"replay","request_sha256":"a".repeat(64),"launch_sha256":sha(b"{}"),"launch":spec,"pod_uid":"fixture-pod","job_uid":"fixture-job","image_digest":"b".repeat(64),"binary_sha256":sha(&fs::read(binary).unwrap()),"binary":binary,"argv":["verify","--dir",root,"--replay-all","--fixture-wait","--fixture-marker",marker],"spec":spec,"spec_sha256":sha(b"{}"),"session":root,"receipt":receipt,"stop_utc_seconds":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()+10});
        let raw = serde_json::to_vec(&cfg).unwrap();
        fs::write(&config, &raw).unwrap();
        let mut command = if wrapper {
            let source = include_str!("../../../run-ecbench");
            let start = source.find("OWNER_PID=\"\"\nOWNER_INTERRUPTED").unwrap();
            let end = source[start..].find("cp /opt/isolab").unwrap() + start;
            let owner_start = source
                .find("  /usr/local/bin/isolab-fresh-owner \"$config\" \"$config_sha\" &")
                .unwrap();
            let owner_end = source[owner_start..].find("  OWNER_PID=\"\"").unwrap()
                + owner_start
                + "  OWNER_PID=\"\"".len();
            let script = format!(
                "set +e\nconfig=$1\nconfig_sha=$2\n{}\n{}\nexit \"$child_exit\"\n",
                &source[start..end],
                source[owner_start..owner_end].replace(
                    "/usr/local/bin/isolab-fresh-owner",
                    env!("CARGO_BIN_EXE_isolab-fresh-owner")
                )
            );
            let path = root.join("wrapper-signal.sh");
            fs::write(&path, script).unwrap();
            let mut c = Command::new("bash");
            c.arg(path);
            c
        } else {
            Command::new(env!("CARGO_BIN_EXE_isolab-fresh-owner"))
        };
        let mut child = command.arg(&config).arg(sha(&raw)).spawn().unwrap();
        for _ in 0..200 {
            if marker.exists() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        if !marker.exists() {
            child.kill().unwrap();
            child.wait().unwrap();
            panic!("fake child did not become ready");
        }
        unsafe {
            assert_eq!(libc::kill(child.id() as i32, libc::SIGTERM), 0);
        }
        let status = child.wait().unwrap();
        assert_eq!(status.code(), Some(137));
        let v: Value = serde_json::from_slice(&fs::read(receipt).unwrap()).unwrap();
        assert_eq!(v["reap_observed"], true);
        assert_eq!(v["exit_code"], 137);
        assert!(v["findings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v.as_str().unwrap().contains("incoming signal 15")));
    }
    fs::remove_dir_all(root).unwrap();
}
