//! A real native worker killed under the production supervisor, then restored
//! using discovery's selected generation. This is synthetic integer progress,
//! not a research benchmark or a test of arbitrary application state.
use checkpoint_discovery::{discover, Options};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

fn atomic(path: &Path, value: &Value) {
    let temporary = path.with_extension("tmp");
    use std::io::Write;
    let mut file = fs::File::create(&temporary).unwrap();
    file.write_all(&serde_json::to_vec(value).unwrap()).unwrap();
    file.sync_all().unwrap();
    fs::rename(temporary, path).unwrap();
    fs::File::open(path.parent().unwrap())
        .unwrap()
        .sync_all()
        .unwrap();
}

#[test]
fn native_checkpoint_worker() {
    let Some(checkpoint) = std::env::var_os("SPOT_CHECKPOINT_DIR") else {
        return;
    };
    let checkpoint = PathBuf::from(checkpoint);
    let output = PathBuf::from(std::env::var_os("SPOT_OUTPUT_DIR").unwrap());
    fs::write(
        checkpoint.join("worker.pid"),
        std::process::id().to_string(),
    )
    .unwrap();
    let mut progress = 0;
    if std::env::var("SPOT_RESUMED").unwrap() == "1" {
        let selected = PathBuf::from(std::env::var_os("SPOT_CHECKPOINT_MANIFEST").unwrap());
        let state: Value = serde_json::from_slice(
            &fs::read(selected.parent().unwrap().join("progress.json")).unwrap(),
        )
        .unwrap();
        progress = state["completed"].as_u64().unwrap();
    } else {
        assert!(std::env::var_os("SPOT_CHECKPOINT_MANIFEST").is_none());
    }
    atomic(&output.join("restored.json"), &json!(progress));
    let spec: Value =
        serde_json::from_slice(&fs::read(checkpoint.parent().unwrap().join("spec.json")).unwrap())
            .unwrap();
    while progress < 12 {
        progress += 1;
        // Never rewrite an older generation, including one rejected as corrupt.
        let generation = checkpoint.join(format!("g{}-{}", progress, std::process::id()));
        fs::create_dir(&generation).unwrap();
        atomic(
            &generation.join("progress.json"),
            &json!({"completed":progress}),
        );
        let bytes = fs::read(generation.join("progress.json")).unwrap();
        atomic(
            &generation.join("checkpoint-manifest.json"),
            &json!({
                "schema":"spot.checkpoint/v1", "job_id":"native", "spec_sha256":spec["sha256"],
                "generation":progress, "committed":true,
                "files":[{"path":"progress.json", "bytes":bytes.len(), "sha256":format!("{:x}",Sha256::digest(&bytes))}]
            }),
        );
        thread::sleep(Duration::from_millis(70));
    }
    atomic(&output.join("answer.json"), &json!(progress));
}

struct Root(PathBuf);
impl Drop for Root {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn supervisor(root: &Path) -> Child {
    let runtime = std::env::var_os("SPOT_SELFTEST_RUNTIME").map(PathBuf::from);
    let runner = runtime
        .as_ref()
        .map(|p| p.join("runner.py"))
        .unwrap_or_else(|| {
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("runner.py")
        });
    let discovery = runtime
        .as_ref()
        .map(|p| p.join("bin/checkpoint-discovery"))
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_checkpoint-discovery")));
    Command::new("python3")
        .arg(runner)
        .args(["--root", root.to_str().unwrap(), "--no-imds", "--once"])
        .env("SPOT_CHECKPOINT_DISCOVERY_BIN", discovery)
        .env("SPOT_CHECKPOINT_MANIFEST", "/must-not-be-inherited")
        .env_remove("SPOT_ACTIVITY_TABLE")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap()
}

fn wait(child: &mut Child) -> std::process::ExitStatus {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if let Some(status) = child.try_wait().unwrap() {
            return status;
        }
        if Instant::now() > deadline {
            child.kill().unwrap();
            panic!("supervisor timed out");
        }
        thread::sleep(Duration::from_millis(10));
    }
}

#[test]
fn supervisor_restores_a_discovered_generation_after_abrupt_worker_death() {
    let parent = std::env::var_os("SPOT_SELFTEST_PARENT")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    let path = parent.join(format!(
        ".spot-native-recovery-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir(&path).unwrap();
    let root = Root(path);
    atomic(
        &root.0.join("jobs.json"),
        &json!([{
            "id":"native", "cwd":".", "checkpoint_contract":"manifest-sha256-v1",
            "argv":[std::env::current_exe().unwrap(), "--exact", "native_checkpoint_worker", "--nocapture"]
        }]),
    );
    let mut first = supervisor(&root.0);
    let mut options = Options::new(root.0.clone());
    options.job = Some("native".into());
    let deadline = Instant::now() + Duration::from_secs(8);
    let selected = loop {
        let report = discover(&options).unwrap();
        if report["jobs"][0]["selected"]["generation"]
            .as_u64()
            .unwrap_or(0)
            >= 3
        {
            break report["jobs"][0]["selected"].clone();
        }
        assert!(
            Instant::now() < deadline,
            "native worker did not checkpoint"
        );
        thread::sleep(Duration::from_millis(10));
    };
    let pid: i32 = fs::read_to_string(root.0.join("state/native/checkpoint/worker.pid"))
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(unsafe { libc::kill(pid, libc::SIGKILL) }, 0);
    assert!(!wait(&mut first).success());
    // Corrupt every generation newer than g1, requiring real fallback at resume.
    for entry in fs::read_dir(root.0.join("state/native/checkpoint")).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir()
            && !path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("g1-")
        {
            fs::write(path.join("progress.json"), b"corrupt").unwrap();
        }
    }
    assert!(selected["generation"].as_u64().unwrap() >= 3);
    assert_eq!(
        discover(&options).unwrap()["jobs"][0]["selected"]["generation"],
        1
    );
    assert!(wait(&mut supervisor(&root.0)).success());
    let output = root.0.join("state/native/output");
    assert_eq!(
        fs::read_to_string(output.join("restored.json")).unwrap(),
        "1"
    );
    assert_eq!(
        fs::read_to_string(output.join("answer.json")).unwrap(),
        "12"
    );
    assert!(root.0.join("state/native/complete.json").is_file());
}
