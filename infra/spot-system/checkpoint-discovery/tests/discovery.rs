use checkpoint_discovery::{admission_allowed, discover, write_report, Options};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::os::unix::fs::{symlink, MetadataExt};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Fixture(PathBuf);
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
impl Fixture {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "checkpoint-discovery-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(path.join("proc")).unwrap();
        let fixture = Self(path);
        fixture.write("jobs.json", &json!([fixture.job()]));
        fixture
    }
    fn job(&self) -> Value {
        json!({"id":"experiment", "cwd":".", "argv":["./worker"], "checkpoint_contract":"manifest-sha256-v1"})
    }
    fn write(&self, path: &str, data: &Value) {
        let path = self.0.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, serde_json::to_vec(data).unwrap()).unwrap();
    }
    fn spec(&self) {
        self.write(
            "state/experiment/spec.json",
            &json!({"sha256":"a".repeat(64), "spec":self.job()}),
        );
    }
    fn generation(&self, number: u64) -> PathBuf {
        self.spec();
        let base = format!("state/experiment/checkpoint/g{number}");
        self.write(
            &format!("{base}/progress.json"),
            &json!({"completed":number}),
        );
        let data = fs::read(self.0.join(&base).join("progress.json")).unwrap();
        self.write(&format!("{base}/checkpoint-manifest.json"), &json!({
            "schema":"spot.checkpoint/v1", "job_id":"experiment", "spec_sha256":"a".repeat(64),
            "generation":number, "committed":true,
            "files":[{"path":"progress.json", "bytes":data.len(), "sha256":format!("{:x}",Sha256::digest(&data))}]
        }));
        self.0.join(base).join("checkpoint-manifest.json")
    }
    fn options(&self) -> Options {
        let mut options = Options::new(self.0.clone());
        options.proc_root = self.0.join("proc");
        options.job = Some("experiment".into());
        options
    }
    fn scan(&self) -> Value {
        discover(&self.options()).unwrap()
    }
}

#[test]
fn empty_job_is_fresh_but_a_temporary_file_is_not() {
    let f = Fixture::new();
    assert!(admission_allowed(&f.scan()));
    f.spec();
    f.write("state/experiment/checkpoint/.partial.tmp", &json!(3));
    let report = f.scan();
    assert_eq!(report["jobs"][0]["status"], "unverified");
    assert!(!admission_allowed(&report));
}

#[test]
fn a_previous_attempt_without_a_checkpoint_cannot_restart_from_zero() {
    let f = Fixture::new();
    f.spec();
    f.write(
        "state/experiment/attempt.json",
        &json!({"sha256":"a".repeat(64)}),
    );
    assert!(!admission_allowed(&f.scan()));
}

#[test]
fn newest_valid_generation_is_selected_by_number_not_write_time() {
    let f = Fixture::new();
    let latest = f.generation(12);
    f.generation(3);
    let report = f.scan();
    assert!(admission_allowed(&report));
    assert_eq!(
        report["jobs"][0]["selected"]["manifest"],
        latest.to_str().unwrap()
    );
    assert_eq!(report["jobs"][0]["selected"]["generation"], 12);
}

#[test]
fn corrupt_latest_generation_falls_back_without_erasing_it() {
    let f = Fixture::new();
    f.generation(1);
    let newest = f.generation(2);
    fs::write(newest.parent().unwrap().join("progress.json"), b"corrupt").unwrap();
    let report = f.scan();
    assert!(admission_allowed(&report));
    assert_eq!(report["jobs"][0]["selected"]["generation"], 1);
    assert_eq!(
        report["jobs"][0]["generations"].as_array().unwrap().len(),
        2
    );
    assert_eq!(
        fs::read(newest.parent().unwrap().join("progress.json")).unwrap(),
        b"corrupt"
    );
}

#[test]
fn same_length_corruption_is_detected_by_digest() {
    let f = Fixture::new();
    let path = f.generation(1);
    let data = fs::read(path.parent().unwrap().join("progress.json")).unwrap();
    fs::write(
        path.parent().unwrap().join("progress.json"),
        vec![b'x'; data.len()],
    )
    .unwrap();
    let report = f.scan();
    assert!(!admission_allowed(&report));
    assert_eq!(
        report["jobs"][0]["generations"][0]["reason"],
        "SHA-256 mismatch"
    );
}

#[test]
fn uncommitted_or_missing_files_never_become_recoverable() {
    for mode in [
        "uncommitted",
        "missing",
        "empty",
        "bad-json",
        "wrong-job",
        "wrong-spec",
    ] {
        let f = Fixture::new();
        let path = f.generation(1);
        let mut manifest: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        match mode {
            "uncommitted" => manifest["committed"] = json!(false),
            "missing" => fs::remove_file(path.parent().unwrap().join("progress.json")).unwrap(),
            "empty" => manifest["files"] = json!([]),
            "wrong-job" => manifest["job_id"] = json!("other"),
            "wrong-spec" => manifest["spec_sha256"] = json!("b".repeat(64)),
            _ => {}
        }
        fs::write(
            &path,
            if mode == "bad-json" {
                b"{".to_vec()
            } else {
                serde_json::to_vec(&manifest).unwrap()
            },
        )
        .unwrap();
        assert!(!admission_allowed(&f.scan()), "mode {mode}");
    }
}

#[test]
fn job_spec_change_cannot_adopt_previous_state() {
    let f = Fixture::new();
    f.generation(1);
    let mut job = f.job();
    job["argv"] = json!(["./different-worker"]);
    f.write("jobs.json", &json!([job]));
    assert_eq!(f.scan()["jobs"][0]["status"], "invalid");
}

#[test]
fn duplicate_generation_is_ambiguous() {
    let f = Fixture::new();
    let first = f.generation(1);
    let second = f.generation(2);
    fs::copy(&first, &second).unwrap();
    fs::copy(
        first.parent().unwrap().join("progress.json"),
        second.parent().unwrap().join("progress.json"),
    )
    .unwrap();
    assert!(!admission_allowed(&f.scan()));
    assert_eq!(
        f.scan()["jobs"][0]["reason"],
        "ambiguous duplicate generation"
    );
}

#[test]
fn escaped_or_duplicate_file_paths_are_rejected() {
    for path_value in [
        "../outside",
        "/etc/passwd",
        "sub/../../outside",
        "progress.json",
    ] {
        let f = Fixture::new();
        let path = f.generation(1);
        let mut manifest: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        let mut file = manifest["files"][0].clone();
        file["path"] = json!(path_value);
        manifest["files"].as_array_mut().unwrap().push(file);
        fs::write(path, serde_json::to_vec(&manifest).unwrap()).unwrap();
        assert!(!admission_allowed(&f.scan()));
    }
}

#[test]
fn symlinks_in_file_directory_or_checkpoint_root_block_admission() {
    for mode in ["file", "directory", "checkpoint-root"] {
        let f = Fixture::new();
        let path = f.generation(1);
        let target = match mode {
            "file" => path.parent().unwrap().join("progress.json"),
            "directory" => path.parent().unwrap().to_path_buf(),
            _ => f.0.join("state/experiment/checkpoint"),
        };
        let moved = f.0.join("original");
        fs::rename(&target, &moved).unwrap();
        symlink(moved, target).unwrap();
        assert!(!admission_allowed(&f.scan()), "{mode}");
    }
}

#[test]
fn resource_limits_and_missing_jobs_are_not_a_clean_inventory() {
    let f = Fixture::new();
    f.generation(1);
    let mut options = f.options();
    options.max_entries = 0;
    assert!(!admission_allowed(&discover(&options).unwrap()));
    options = f.options();
    options.max_hash_bytes = 1;
    assert!(!admission_allowed(&discover(&options).unwrap()));
    options.job = Some("missing".into());
    assert!(!admission_allowed(&discover(&options).unwrap()));
    fs::remove_file(f.0.join("jobs.json")).unwrap();
    assert!(!admission_allowed(&f.scan()));
}

#[test]
fn durable_root_is_scanned_before_a_large_external_archive() {
    let f = Fixture::new();
    f.write("zz-durable/jobs.json", &json!([]));
    f.write("zz-durable/progress.ckpt", &json!(1));
    for number in 0..30 {
        f.write(&format!("aa-archive/file-{number}"), &json!(0));
    }
    let mut options = Options::new(f.0.join("zz-durable"));
    options.search_roots.push(f.0.join("aa-archive"));
    options.proc_root = f.0.join("proc");
    options.max_entries = 8;
    let report = discover(&options).unwrap();
    assert_eq!(report["coverage_complete"], false);
    assert_eq!(
        report["artifacts"][0]["path"],
        f.0.join("zz-durable/progress.ckpt").to_str().unwrap()
    );
}

#[test]
fn completion_marker_must_match_the_job_identity() {
    let f = Fixture::new();
    f.spec();
    f.write(
        "state/experiment/complete.json",
        &json!({"sha256":"b".repeat(64)}),
    );
    assert!(!admission_allowed(&f.scan()));
    f.write(
        "state/experiment/complete.json",
        &json!({"sha256":"a".repeat(64)}),
    );
    assert_eq!(f.scan()["jobs"][0]["status"], "complete");
}

#[test]
fn live_process_inventory_finds_legacy_worker_without_recording_arguments_or_environment() {
    let f = Fixture::new();
    let proc = f.0.join("proc/987654321");
    fs::create_dir_all(proc.join("fd")).unwrap();
    symlink(
        f.0.join("target/release/examples/factor_base_shape_ladder"),
        proc.join("exe"),
    )
    .unwrap();
    symlink(&f.0, proc.join("cwd")).unwrap();
    symlink(f.0.join("output.jsonl"), proc.join("fd/1")).unwrap();
    fs::write(
        proc.join("cmdline"),
        b"factor_base_shape_ladder\0--token\0PRIVATE_ARGUMENT\0",
    )
    .unwrap();
    // Even an invalid/unreadable environment must have no effect on discovery.
    fs::write(
        proc.join("environ"),
        b"AWS_SECRET_ACCESS_KEY=PRIVATE_ENVIRONMENT\0",
    )
    .unwrap();
    let mut options = f.options();
    options.job = None;
    let report = discover(&options).unwrap();
    assert_eq!(
        report["processes"][0]["capability"],
        "no-checkpoint-interface"
    );
    let text = report.to_string();
    assert!(!text.contains("PRIVATE_ARGUMENT"));
    assert!(!text.contains("PRIVATE_ENVIRONMENT"));
    assert_eq!(
        report["processes"][0]["stdout_path"],
        f.0.join("output.jsonl").to_str().unwrap()
    );
}

#[test]
fn orphan_checkpoint_is_only_a_candidate_and_node_identity_is_excluded() {
    let f = Fixture::new();
    f.write("orphan.ckpt", &json!({"looks":"like progress"}));
    f.write(
        "nodes/cairn/checkpoint.json",
        &json!({"private":"node state"}),
    );
    f.write(".ssh/checkpoint.json", &json!({"private":"key state"}));
    let mut options = f.options();
    options.job = None;
    let report = discover(&options).unwrap();
    assert_eq!(report["artifacts"].as_array().unwrap().len(), 1);
    assert_eq!(report["artifacts"][0]["status"], "candidate-only");
}

#[test]
fn atomic_inventory_output_is_private_and_replaces_a_symlink_without_touching_target() {
    let f = Fixture::new();
    let output = f.0.join("report.json");
    let target = f.0.join("untouched");
    fs::write(&target, b"original").unwrap();
    symlink(&target, &output).unwrap();
    write_report(&output, &f.scan()).unwrap();
    assert_eq!(fs::read(&target).unwrap(), b"original");
    assert_eq!(fs::metadata(&output).unwrap().mode() & 0o777, 0o600);
    assert_eq!(
        serde_json::from_slice::<Value>(&fs::read(output).unwrap()).unwrap()["schema"],
        "spot.checkpoint-discovery/v1"
    );
}

#[test]
fn fifo_checkpoint_cannot_hang_the_validator() {
    let f = Fixture::new();
    let path = f.generation(1);
    let file = path.parent().unwrap().join("progress.json");
    fs::remove_file(&file).unwrap();
    use std::os::unix::ffi::OsStrExt;
    let name = std::ffi::CString::new(file.as_os_str().as_bytes()).unwrap();
    assert_eq!(unsafe { libc::mkfifo(name.as_ptr(), 0o600) }, 0);
    assert!(!admission_allowed(&f.scan()));
}

#[test]
fn unsafe_or_uncontracted_jobs_are_not_admitted() {
    let f = Fixture::new();
    for job in [
        json!({"id":"../escape"}),
        json!({"id":"experiment", "cwd":".", "argv":["x"]}),
    ] {
        f.write("jobs.json", &json!([job]));
        assert!(!admission_allowed(&f.scan()));
    }
}
