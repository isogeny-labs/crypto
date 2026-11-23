//! Complete raw-directory archive and byte checks; no scientific calculations.
use serde_json::{json, Value};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
fn walk(root: &Path, base: &Path, files: &mut Vec<Value>) {
    let mut paths: Vec<_> = fs::read_dir(root)
        .unwrap()
        .map(|x| x.unwrap().path())
        .collect();
    paths.sort();
    for p in paths {
        if p.is_dir() {
            walk(&p, base, files)
        } else if p.is_file() {
            let (sha, bytes) = crypto_lib::store::hash_file(&p).unwrap();
            files.push(json!({"path":p.strip_prefix(base).unwrap(),"sha256":sha,"bytes":bytes}));
        }
    }
}
fn pack(root: &Path, names: &[&str], out: &Path) {
    let mut tar = Command::new("tar")
        .args([
            "--sort=name",
            "--mtime=2026-11-19T00:00:00Z",
            "--owner=0",
            "--group=0",
            "--numeric-owner",
            "--format=gnu",
            "--mode=u+rwX,go+rX,go-w",
            "-cf",
            "-",
        ])
        .args(names)
        .current_dir(root)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let status = Command::new("xz")
        .args(["-6", "-T1"])
        .stdin(tar.stdout.take().unwrap())
        .stdout(Stdio::from(File::create(out).unwrap()))
        .status()
        .unwrap();
    assert!(status.success());
    assert!(tar.wait().unwrap().success());
}
fn main() {
    let study = PathBuf::from(std::env::args().nth(1).expect("study"));
    let study = study.canonicalize().unwrap();
    let names = [
        "results",
        "qualification-a",
        "qualification-b",
        "qualification-c",
        "smoke-a",
        "smoke-b",
        "audit-signed-v2",
    ];
    let archive = study.join("raw-results.tar.xz");
    pack(&study, &names, &archive);
    let mut files = Vec::new();
    for name in names {
        walk(&study.join(name), &study, &mut files);
    }
    let roundtrip = PathBuf::from(format!("/tmp/conductor-rust-repack-{}", std::process::id()));
    fs::create_dir(&roundtrip).unwrap();
    assert!(Command::new("tar")
        .args(["-xJf"])
        .arg(&archive)
        .arg("-C")
        .arg(&roundtrip)
        .status()
        .unwrap()
        .success());
    for f in &files {
        let p = roundtrip.join(f["path"].as_str().unwrap());
        let (sha, bytes) = crypto_lib::store::hash_file(&p).unwrap();
        assert_eq!(sha, f["sha256"].as_str().unwrap());
        assert_eq!(bytes, f["bytes"].as_u64().unwrap());
    }
    let repack = roundtrip.join("repack.tar.xz");
    pack(&roundtrip, &names, &repack);
    let (sha, bytes) = crypto_lib::store::hash_file(&archive).unwrap();
    assert_eq!(crypto_lib::store::hash_file(&repack).unwrap().0, sha);
    let receipt = json!({"schema":"conductor.rust-raw-archive/v1","archive":"raw-results.tar.xz","sha256":sha,"bytes":bytes,"extraction_check":"pass","repack_identical":true,"uploaded":false,"remote_verified":false,"desired_s3_uri":format!("s3://setec-astronomy-artifacts/store/sha256/{}/{}",&sha[..2],sha),"files":files});
    fs::write(
        study.join("archive-info.json"),
        serde_json::to_vec_pretty(&receipt).unwrap(),
    )
    .unwrap();
    println!("complete archive {bytes} bytes, SHA256 {sha}; extraction and identical repack pass; upload not claimed");
    fs::remove_dir_all(roundtrip).unwrap();
}
