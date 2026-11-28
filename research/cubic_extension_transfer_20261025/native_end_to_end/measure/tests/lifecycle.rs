//! Development controls only: no ECDLP instance and no performance comparison.
use cubic_native_measure::{
    command::{run, Config},
    isolation::CpuRequest,
};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

fn build(program: &str, args: &[&str]) {
    let status = Command::new(program)
        .args(args)
        .status()
        .expect("fixture compiler available");
    assert!(status.success(), "fixture compiler failed");
}

fn execute(
    root: &Path,
    library: &Path,
    exe: &Path,
    name: &str,
    mode: Option<&str>,
    timeout: Duration,
) -> serde_json::Value {
    let mut argv = vec![exe.to_str().unwrap().to_owned()];
    argv.extend(mode.map(str::to_owned));
    run(&Config {
        argv,
        directory: root.into(),
        output: root.join(name),
        library: library.into(),
        public_inputs: Vec::new(),
        cpus: CpuRequest::None,
        diagnostic: true,
        timeout,
    })
    .expect("fixture record retained")
}

#[test]
fn resident_hooks_failures_timeout_and_orphans_keep_their_actual_outcome() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let library = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("libcubic_native_measure.so");
    assert!(library.is_file(), "cargo test must build the cdylib");
    let root = std::env::temp_dir().join(format!(
        "cubic-native-measure-lifecycle-{}",
        std::process::id()
    ));
    std::fs::create_dir(&root).expect("fresh fixture output");
    let cpp = root.join("cpp-fixture");
    let rust = root.join("rust-fixture");
    build(
        "g++",
        &[
            "-std=c++17",
            "-O2",
            "-Wall",
            "-Wextra",
            "-Werror",
            manifest.join("tests/fixtures/worker.cpp").to_str().unwrap(),
            "-ldl",
            "-o",
            cpp.to_str().unwrap(),
        ],
    );
    build(
        "rustc",
        &[
            "--edition",
            "2021",
            "-O",
            manifest.join("tests/fixtures/worker.rs").to_str().unwrap(),
            "-o",
            rust.to_str().unwrap(),
        ],
    );
    let second = Duration::from_secs(5);
    for (name, exe) in [("cpp-ok", &cpp), ("rust-ok", &rust)] {
        let row = execute(&root, &library, exe, name, None, second);
        assert_eq!(row["outcome"]["completed"], true);
        assert_eq!(row["child"]["pid"], row["pid"]);
        assert_eq!(row["child"]["solve"]["threads_before"], 1);
        assert_eq!(row["child"]["solve"]["threads_after"], 1);
        assert_eq!(row["isolation"]["level"], "L0");
        assert_eq!(row["isolation_wall_eligible"], false);
        assert_eq!(row["performance_admitted"], false);
        assert_eq!(row["child"]["pmu"]["whole_worker_coverage_verified"], false);
    }
    let row = execute(&root, &library, &cpp, "cpp-fail", Some("fail"), second);
    assert_eq!(row["outcome"]["exit_code"], 7);
    assert_eq!(row["outcome"]["completed"], false);
    assert!(
        row["child"].is_object(),
        "normal error return still runs the hook"
    );
    let row = execute(&root, &library, &rust, "rust-panic", Some("panic"), second);
    assert_eq!(row["outcome"]["completed"], false);
    assert!(row["child"].is_object(), "unwinding still runs the hook");
    let row = execute(
        &root,
        &library,
        &cpp,
        "timeout",
        Some("sleep"),
        Duration::from_millis(100),
    );
    assert_eq!(row["outcome"]["timed_out"], true);
    assert_eq!(row["outcome"]["completed"], false);
    assert!(
        row["child"].is_null(),
        "killed input cannot manufacture a final observation"
    );
    let row = execute(&root, &library, &cpp, "orphan", Some("background"), second);
    assert_eq!(row["outcome"]["descendants_left"], true);
    assert_eq!(row["outcome"]["completed"], false);
    let row = execute(
        &root,
        &library,
        Path::new("/usr/bin/true"),
        "unobserved",
        None,
        second,
    );
    assert_eq!(row["outcome"]["completed"], false);
    assert!(row["outcome"]["observation_error"].is_string());
    let input = root.join("public-input.txt");
    std::fs::write(&input, b"original").unwrap();
    let row = run(&Config {
        argv: vec![
            cpp.to_str().unwrap().into(),
            "mutate-input".into(),
            input.to_str().unwrap().into(),
        ],
        directory: root.clone(),
        output: root.join("input-changed"),
        library: library.clone(),
        public_inputs: vec![input],
        cpus: CpuRequest::None,
        timeout: second,
        diagnostic: true,
    })
    .unwrap();
    assert_eq!(row["outcome"]["inputs_changed"], true);
    assert_eq!(row["outcome"]["completed"], false);
    assert!(run(&Config {
        argv: vec![cpp.to_str().unwrap().into()],
        directory: root.clone(),
        output: root.join("strict-refused"),
        library: library.clone(),
        public_inputs: Vec::new(),
        cpus: CpuRequest::None,
        timeout: second,
        diagnostic: false
    })
    .is_err());
    // An existing directory is never overwritten, even on a failed input.
    assert!(run(&Config {
        argv: vec![cpp.to_str().unwrap().into()],
        directory: root.clone(),
        output: root.join("cpp-ok"),
        library,
        public_inputs: Vec::new(),
        cpus: CpuRequest::None,
        timeout: second,
        diagnostic: true
    })
    .is_err());
    eprintln!("development fixture records retained at {}", root.display());
}
