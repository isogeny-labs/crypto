//! Pin native-validated records and source/binary hashes in a review manifest.
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const ROOT: &str = "research/pencil_markowitz_20261022";
const ARCHIVE: &str = "research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst";
const CONTROL: &str = "research/pencil_sparse_20261022/results/p186379-peel-cap25m-t0.jsonl";
const BINARY: &str = "research/pencil_sparse_20261022/target/release/pencil-sparse";
const ARCHIVE_SHA: &str = "550f7178e30b60db62931a752b0c179beeef7ea99a96cf94486ff8aee631d339";
const CONTROL_SHA: &str = "1a1e59e22ea23c1824f57bcf626c7270c685039732ad31d164e6f576df616873";
const BINARY_SHA: &str = "03a431e1ae71d022ec8f1c6fedfbe26240eb28446c55f68fd2460e115daca92f";

fn digest(path: impl AsRef<Path>) -> String {
    let output = Command::new("sha256sum")
        .arg(path.as_ref())
        .output()
        .unwrap();
    assert!(output.status.success(), "sha256sum {:?}", path.as_ref());
    String::from_utf8(output.stdout)
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .to_owned()
}
fn pin(relative: &str) -> Value {
    json!({"path":relative,"sha256":digest(Path::new(ROOT).join(relative))})
}
fn read(relative: &str) -> Value {
    serde_json::from_slice(&fs::read(Path::new(ROOT).join(relative)).unwrap()).unwrap()
}
fn command_text(program: &str, args: &[&str]) -> String {
    let output = Command::new(program).args(args).output().unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}
fn visit_files(root: &Path, directory: &Path, output: &mut BTreeMap<String, String>) {
    for entry in fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            if path.file_name().unwrap() != "__pycache__" {
                visit_files(root, &path, output);
            }
        } else if path.file_name().unwrap() != "manifest.json"
            && path.extension().and_then(|part| part.to_str()) != Some("pyc")
        {
            let relative = path
                .strip_prefix(root)
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();
            assert!(output.insert(relative, digest(&path)).is_none());
        }
    }
}
fn main() {
    let validation = read("results/validation.json");
    let replay = read("results/replay-certificate.json");
    let upper_replay = read("results/upper-process-replay.json");
    assert_eq!(validation["status"], "pass");
    assert_eq!(validation["independent"], true);
    assert_eq!(replay["status"], "verified");
    assert_eq!(replay["independent"], true);
    assert_eq!(upper_replay["status"], "verified");
    assert_eq!(upper_replay["independent"], true);
    assert_eq!(digest(ARCHIVE), ARCHIVE_SHA);
    assert_eq!(digest(CONTROL), CONTROL_SHA);
    assert_eq!(digest(BINARY), BINARY_SHA);

    let mut rho = read("results/rho-band.json");
    rho["symmetry_check"] = pin("results/rho-symmetry-check.json");
    let gate = json!({
        "schema":"validation-gate/v1",
        "scope":"fixed Markowitz-priority pencil gate and upper-size diagnostic",
        "replay_certificate":pin("results/replay-certificate.json"),
        "validator_report":pin("results/validation.json"),
        "rho_reference":rho
    });
    fs::write(
        Path::new(ROOT).join("VALIDATION.json"),
        format!("{}\n", serde_json::to_string_pretty(&gate).unwrap()),
    )
    .unwrap();

    let native_root = Path::new("research/pencil_sparse_20261022");
    let mut sources = BTreeMap::new();
    for relative in [
        "Cargo.toml",
        "Cargo.lock",
        "src/bin/pencil_sparse.rs",
        "src/bin/price_sparse.rs",
        "src/bin/compare_sparse.rs",
        "src/bin/validate_sparse.rs",
        "src/bin/cap_process_status.rs",
    ] {
        let path = native_root.join(relative);
        sources.insert(path.to_str().unwrap().to_owned(), digest(&path));
    }
    let mut files = BTreeMap::new();
    visit_files(Path::new(ROOT), Path::new(ROOT), &mut files);
    let manifest = json!({
        "schema":"pencil-markowitz-manifest/v1",
        "generated_utc":command_text("date", &["-u", "+%Y-%m-%dT%H:%M:%SZ"]),
        "host":command_text("hostname", &[]),
        "machine":command_text("uname", &["-m"]),
        "archive_path":ARCHIVE,"archive_sha256":ARCHIVE_SHA,
        "control_path":CONTROL,"control_sha256":CONTROL_SHA,
        "frozen_binary_path":BINARY,"frozen_binary_sha256":BINARY_SHA,
        "current_binary_sha256":digest(BINARY),
        "native_source_sha256":sources,
        "artifact_sha256":files
    });
    let manifest_path: PathBuf = Path::new(ROOT).join("results/manifest.json");
    fs::write(
        manifest_path,
        format!("{}\n", serde_json::to_string_pretty(&manifest).unwrap()),
    )
    .unwrap();
    println!("native Markowitz package: {} artifact hashes", files.len());
}
