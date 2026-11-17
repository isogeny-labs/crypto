//! Read-only S3 restoration and explicitly requested producer replay.
//!
//! Restores are isolated and never replace an existing destination. SHA-256
//! checks bind the bytes, while native replay remains producer evidence rather
//! than an independent mathematical proof. No Python process is invoked.

use anyhow::{ensure, Context, Result};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::Read;
use std::path::{Component, Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::{Duration, Instant};

use crate::common::{
    bounded_bytes, check_artifact, contained, digest, hash_file, load_artifact_json,
    read_json_bytes, valid_hex, write_new,
};

const BUCKET: &str = "setec-astronomy-artifacts";
const MAX_REFERENCES: usize = 100_000;
const COMMAND_OUTPUT_LIMIT: usize = 8 * 1024 * 1024;

#[derive(Debug)]
struct CommandOutput {
    status: ExitStatus,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

fn terminate(child: &mut Child) {
    // Every command has its own process group. Kill descendants retaining pipes
    // as well as the direct process; otherwise a timeout can hang on pipe EOF.
    #[cfg(unix)]
    unsafe {
        libc::kill(-(child.id() as libc::pid_t), libc::SIGKILL);
    }
    let _ = child.kill();
    let _ = child.wait();
}

fn read_pipe(mut pipe: impl Read, limit: usize, total: &AtomicUsize) -> Result<Vec<u8>> {
    let mut result = Vec::new();
    let mut buffer = [0u8; 8192];
    loop {
        let count = pipe.read(&mut buffer)?;
        if count == 0 {
            return Ok(result);
        }
        let previous = total.fetch_add(count, Ordering::Relaxed);
        ensure!(
            previous <= limit && count <= limit - previous,
            "subprocess output limit exceeded"
        );
        result.extend_from_slice(&buffer[..count]);
    }
}

/// Bound wall time and combined captured output. A GET additionally monitors
/// its output file; the final artifact check enforces its exact declared size.
/// The monitor is a polling guard, not a filesystem quota against hostile code.
fn run_bounded(
    command: &mut Command,
    timeout: Duration,
    output_limit: usize,
    download: Option<(&Path, u64)>,
) -> Result<CommandOutput> {
    command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }
    let mut child = command.spawn().context("start subprocess")?;
    let stdout = child.stdout.take().context("missing stdout pipe")?;
    let stderr = child.stderr.take().context("missing stderr pipe")?;
    let total = Arc::new(AtomicUsize::new(0));
    let (sender, receiver) = mpsc::channel();
    let counter = Arc::clone(&total);
    let stdout_sender = sender.clone();
    thread::spawn(move || {
        let _ = stdout_sender.send((true, read_pipe(stdout, output_limit, &counter)));
    });
    thread::spawn(move || {
        let _ = sender.send((false, read_pipe(stderr, output_limit, &total)));
    });
    let start = Instant::now();
    let mut out = None;
    let mut err = None;
    let mut status = None;
    let result = (|| -> Result<CommandOutput> {
        loop {
            while let Ok((is_stdout, bytes)) = receiver.try_recv() {
                let bytes = bytes?;
                if is_stdout {
                    out = Some(bytes);
                } else {
                    err = Some(bytes);
                }
            }
            if let Some((path, maximum)) = download {
                match fs::symlink_metadata(path) {
                    Ok(metadata) => ensure!(
                        metadata.is_file()
                            && !metadata.file_type().is_symlink()
                            && metadata.len() <= maximum,
                        "download exceeds declared size or changed file type"
                    ),
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => (),
                    Err(error) => return Err(error.into()),
                }
            }
            if status.is_none() {
                status = child.try_wait()?;
            }
            if status.is_some() && out.is_some() && err.is_some() {
                return Ok(CommandOutput {
                    status: status.take().unwrap(),
                    stdout: out.take().unwrap(),
                    stderr: err.take().unwrap(),
                });
            }
            ensure!(start.elapsed() < timeout, "subprocess timeout");
            thread::sleep(Duration::from_millis(5));
        }
    })();
    if result.is_err() {
        terminate(&mut child);
    }
    result
}

fn require_success(output: &CommandOutput, operation: &str) -> Result<()> {
    ensure!(
        output.status.success(),
        "{operation} failed ({}): {}",
        output.status,
        String::from_utf8_lossy(&output.stderr).trim()
    );
    Ok(())
}

fn relative(value: &str) -> Result<()> {
    ensure!(
        !value.is_empty()
            && !value.contains('\\')
            && !value.contains('\0')
            && !value.chars().any(char::is_control)
            && value
                .split('/')
                .all(|part| !part.is_empty() && part != "." && part != ".."),
        "noncanonical artifact path"
    );
    Ok(())
}

fn remote_prefix(remote: &str) -> Result<String> {
    let base = format!("s3://{BUCKET}");
    let suffix = remote
        .strip_prefix(&base)
        .context("canonical S3 remote required")?;
    ensure!(
        suffix.is_empty() || suffix.starts_with('/'),
        "canonical S3 bucket required"
    );
    ensure!(
        !suffix.contains(['?', '#', '%', '\\']) && !suffix.chars().any(char::is_control),
        "noncanonical S3 remote"
    );
    if suffix.is_empty() || suffix == "/" {
        return Ok(String::new());
    }
    let prefix = suffix.strip_prefix('/').unwrap();
    let prefix = prefix.strip_suffix('/').unwrap_or(prefix);
    relative(prefix)?;
    Ok(prefix.to_owned())
}

fn absolute(path: &Path) -> Result<PathBuf> {
    Ok(if path.is_absolute() {
        path.to_owned()
    } else {
        std::env::current_dir()?.join(path)
    })
}

fn reject_symlink_ancestors(path: &Path) -> Result<()> {
    let mut current = PathBuf::new();
    for part in absolute(path)?.components() {
        ensure!(
            !matches!(part, Component::ParentDir),
            "parent traversal in restore path"
        );
        current.push(part);
        match fs::symlink_metadata(&current) {
            Ok(meta) => ensure!(!meta.file_type().is_symlink(), "symlink in restore path"),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => (),
            Err(error) => return Err(error.into()),
        }
    }
    Ok(())
}

fn members(artifacts: &[Value]) -> Result<BTreeMap<&str, &Value>> {
    let mut result = BTreeMap::new();
    for artifact in artifacts {
        let path = artifact["path"].as_str().context("invalid artifact path")?;
        relative(path)?;
        ensure!(
            result.insert(path, artifact).is_none(),
            "duplicate artifact path"
        );
    }
    Ok(result)
}

pub fn fetch_cell(receipt: &Path, expected: &str, dest: &Path, maxbytes: u64) -> Result<Value> {
    fetch_with_aws(receipt, expected, dest, maxbytes, Path::new("aws"))
}

fn fetch_with_aws(
    receipt: &Path,
    expected: &str,
    dest: &Path,
    maxbytes: u64,
    aws: &Path,
) -> Result<Value> {
    ensure!(valid_hex(expected, 64), "invalid trusted receipt digest");
    // Parse exactly the bytes whose externally supplied digest was checked.
    let raw = bounded_bytes(receipt)?;
    ensure!(
        digest(&raw) == expected,
        "completion receipt digest mismatch"
    );
    let record = read_json_bytes(&raw)?;
    ensure!(
        record["schema"] == "ml-dataset.publication/v1",
        "completion schema"
    );
    let prefix = remote_prefix(record["remote"].as_str().context("missing S3 remote")?)?;
    let objects = record["objects"]
        .as_array()
        .context("missing publication objects")?;
    ensure!(
        !objects.is_empty() && objects.len() <= MAX_REFERENCES,
        "object count limit"
    );
    let manifest_sha = record["manifest_sha256"]
        .as_str()
        .context("missing manifest digest")?;
    ensure!(valid_hex(manifest_sha, 64), "invalid manifest digest");
    let mut seen = BTreeSet::new();
    let mut total = 0u64;
    let mut manifest_artifact = None;
    for object in objects {
        let artifact = object
            .get("artifact")
            .filter(|a| a.is_object())
            .context("invalid artifact")?;
        let path = artifact["path"].as_str().context("invalid artifact path")?;
        relative(path)?;
        ensure!(seen.insert(path), "duplicate downloaded path");
        let sha = artifact["sha256"]
            .as_str()
            .context("invalid object digest")?;
        ensure!(valid_hex(sha, 64), "invalid object digest");
        let bytes = artifact["bytes"].as_u64().context("invalid object size")?;
        total = total
            .checked_add(bytes)
            .context("download budget overflow")?;
        ensure!(total <= maxbytes, "download budget exceeded");
        ensure!(
            object["object_key"] == format!("sha256/{}/{sha}", &sha[..2]),
            "noncanonical content-addressed key"
        );
        if path == "manifest.json" {
            ensure!(sha == manifest_sha, "remote manifest digest mismatch");
            manifest_artifact = Some(artifact);
        }
    }
    let manifest_artifact = manifest_artifact.context("publication lacks manifest")?;
    for path in &seen {
        for (position, _) in path.match_indices('/') {
            ensure!(
                !seen.contains(&path[..position]),
                "artifact path is also a directory"
            );
        }
    }
    reject_symlink_ancestors(dest)?;
    ensure!(
        fs::symlink_metadata(dest).is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound),
        "restore destination must not exist"
    );
    let parent = dest
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    fs::create_dir_all(parent)?;
    fs::create_dir(dest).context("create fresh restore destination")?;
    let incomplete = dest.join("RESTORE_INCOMPLETE");
    write_new(
        &incomplete,
        b"Restore not complete. Retain this directory on failure; do not reuse it.\n",
    )?;
    let scratch = dest.join("incomplete-download");
    fs::create_dir(&scratch)?;
    for object in objects {
        let artifact = &object["artifact"];
        let target = contained(&scratch, artifact["path"].as_str().unwrap())?;
        fs::create_dir_all(target.parent().context("artifact parent")?)?;
        let key = object["object_key"].as_str().unwrap();
        let full_key = if prefix.is_empty() {
            key.to_owned()
        } else {
            format!("{prefix}/{key}")
        };
        let mut head = Command::new(aws);
        head.args([
            "--region",
            "us-east-2",
            "s3api",
            "head-object",
            "--bucket",
            BUCKET,
            "--key",
            &full_key,
        ]);
        let output = run_bounded(
            &mut head,
            Duration::from_secs(60),
            COMMAND_OUTPUT_LIMIT,
            None,
        )?;
        require_success(&output, "S3 HEAD")?;
        let metadata = read_json_bytes(&output.stdout)?;
        let bytes = artifact["bytes"].as_u64().unwrap();
        ensure!(
            metadata["ContentLength"].as_u64() == Some(bytes),
            "remote size differs from declared budget"
        );
        let etag = metadata["ETag"]
            .as_str()
            .filter(|s| !s.is_empty() && s.len() <= 1024 && !s.chars().any(char::is_control))
            .context("missing or invalid S3 ETag")?;
        // Reserve the output name before invoking the trusted transport.
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&target)?;
        let mut get = Command::new(aws);
        get.args([
            "--region",
            "us-east-2",
            "s3api",
            "get-object",
            "--bucket",
            BUCKET,
            "--key",
            &full_key,
            "--if-match",
            etag,
        ])
        .arg(&target);
        let output = run_bounded(
            &mut get,
            Duration::from_secs(300),
            COMMAND_OUTPUT_LIMIT,
            Some((&target, bytes)),
        )?;
        require_success(&output, "S3 GET")?;
        check_artifact(&scratch, artifact)?;
        File::open(&target)?.sync_all()?;
    }
    let manifest = load_artifact_json(&scratch, manifest_artifact)?;
    let declared = manifest["artifacts"]
        .as_array()
        .context("missing manifest members")?;
    let published: Vec<Value> = objects
        .iter()
        .filter(|o| o["artifact"]["path"] != "manifest.json")
        .map(|o| o["artifact"].clone())
        .collect();
    ensure!(
        members(&published)? == members(declared)?,
        "completion member set differs from manifest"
    );
    let indexed = members(&published)?;
    for name in ["plan.json", "run.json", "protocol.json"] {
        let artifact = indexed
            .get(name)
            .context("publication lacks root provenance")?;
        let raw = bounded_bytes(&contained(&scratch, name)?)?;
        ensure!(
            raw.len() as u64 == artifact["bytes"].as_u64().unwrap()
                && digest(&raw) == artifact["sha256"],
            "root provenance changed during restore"
        );
        write_new(&dest.join(name), &raw)?;
    }
    let index = manifest["cell_index"]
        .as_u64()
        .context("invalid restored index")?;
    fs::create_dir(dest.join("cells"))?;
    let cell = dest.join("cells").join(format!("{index:012}"));
    fs::rename(&scratch, &cell)?;
    let result = crate::audit::audit(dest, None, false);
    let (mut report, _) = match result {
        Ok(value) => value,
        Err(error) => {
            // Preserve the downloaded bytes under an explicitly incomplete name.
            let _ = fs::rename(&cell, &scratch);
            return Err(error.context("restored cell audit failed; incomplete restore retained"));
        }
    };
    report["remote_rechecked"] = json!(true);
    report["trusted_completion_receipt_sha256"] = json!(expected);
    fs::remove_file(incomplete)?;
    File::open(dest)?.sync_all()?;
    Ok(report)
}

fn row_identity(rows: &[Value]) -> Result<Vec<(u64, String)>> {
    rows.iter()
        .map(|row| {
            let index = row["index"].as_u64().context("invalid audited row index")?;
            let id = row["id"].as_str().context("invalid audited row digest")?;
            ensure!(valid_hex(id, 64), "invalid audited row digest");
            Ok((index, id.to_owned()))
        })
        .collect()
}

pub fn replay_native(root: &Path, binary: &Path, report: &mut Value, rows: &[Value]) -> Result<()> {
    ensure!(report.is_object(), "audit report must be an object");
    report["producer_native_replay_passed"] = json!(false);
    let run_sha = report["run_sha256"]
        .as_str()
        .context("missing audited run identity")?
        .to_owned();
    ensure!(valid_hex(&run_sha, 64), "invalid audited run identity");
    let expected_rows = row_identity(rows)?;
    let binary = absolute(binary)?;
    ensure!(
        fs::symlink_metadata(&binary)?.is_file(),
        "native verifier must be a regular file"
    );
    let binary_sha = hash_file(&binary)?;
    let mut command = Command::new(&binary);
    command
        .args(["dataset", "verify", "--out"])
        .arg(absolute(root)?);
    let replay = run_bounded(
        &mut command,
        Duration::from_secs(600),
        COMMAND_OUTPUT_LIMIT,
        None,
    )?;
    require_success(&replay, "native replay")?;
    ensure!(
        hash_file(&binary)? == binary_sha,
        "native verifier changed during replay"
    );
    let native = read_json_bytes(&replay.stdout)?;
    ensure!(
        native["schema"] == "crypto.ml-dataset.status/v1"
            && native["known_answer_replay_performed"] == true
            && native["artifact_hashes_verified"] == true
            && native["completed_receipts"].as_u64() == Some(rows.len() as u64),
        "native replay coverage mismatch"
    );
    let arithmetic = report["independent_arithmetic_requested"]
        .as_bool()
        .unwrap_or(false);
    let (_, after) = crate::audit::audit(root, Some(&run_sha), arithmetic)?;
    ensure!(
        row_identity(&after)? == expected_rows,
        "cell contents changed during replay"
    );
    report["producer_native_replay_passed"] = json!(true);
    report["native_verifier_sha256"] = json!(binary_sha);
    report["native_replay_stdout_sha256"] = json!(digest(&replay.stdout));
    Ok(())
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;
    use tempfile::TempDir;

    fn shell(path: &Path, body: &str) {
        fs::write(path, format!("#!/bin/sh\nset -eu\n{body}\n")).unwrap();
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }

    fn quoted(value: &str) -> String {
        format!("'{}'", value.replace('\'', "'\\''"))
    }

    struct Fixture {
        _temp: TempDir,
        root: PathBuf,
        receipt: PathBuf,
        aws: PathBuf,
        expected: String,
    }

    impl Fixture {
        fn new() -> Self {
            let temp = tempfile::tempdir().unwrap();
            let root = temp.path().join("source");
            fs::create_dir(&root).unwrap();
            let protocol = serde_json::to_vec(&json!({"schema":"experiment.protocol/v1",
                "fixture":"software transport only"}))
            .unwrap();
            let plan = json!({"schema":"crypto.ml-dataset.plan/v1","field_sizes":[23],
                "arities":[2],"families":["prime"],"solvers":["bsgs"],"curve_seeds":1,
                "generators":1,"keys":1,"seed":20261028,"factor_base_size":2,
                "degree_bound":3,"work_limit":1000,"twists":true,
                "protocol_sha256":digest(&protocol),"split_rule":"curve-twist-component-sha256-v1"});
            let plan_bytes = serde_json::to_vec(&plan).unwrap();
            let run = json!({"schema":"crypto.ml-dataset.run/v1","plan_sha256":digest(&plan_bytes),
                "source_commit":"a".repeat(40),"binary_sha256":"b".repeat(64),
                "shard_index":0,"shard_count":1});
            let config = crate::audit::config_at(&plan, 0).unwrap();
            let cell = root.join("cells/000000000000");
            fs::create_dir_all(&cell).unwrap();
            let mut artifacts = vec![];
            for (name, bytes, role) in [
                ("plan.json", plan_bytes.clone(), "restricted-provenance"),
                (
                    "run.json",
                    serde_json::to_vec(&run).unwrap(),
                    "restricted-provenance",
                ),
                ("protocol.json", protocol, "restricted-provenance"),
                (
                    "config.json",
                    serde_json::to_vec(&config).unwrap(),
                    "restricted-provenance",
                ),
                ("attempt.json", b"{}".to_vec(), "process-provenance"),
                ("worker-process.json", b"{}".to_vec(), "process-provenance"),
                ("stdout.log", vec![], "process-log"),
                ("stderr.log", vec![], "process-log"),
            ] {
                fs::write(cell.join(name), &bytes).unwrap();
                if ["plan.json", "run.json", "protocol.json"].contains(&name) {
                    fs::write(root.join(name), &bytes).unwrap();
                }
                artifacts.push(json!({"path":name,"role":role,"encoding":"json",
                    "bytes":bytes.len(),"sha256":digest(&bytes)}));
            }
            let manifest = json!({"schema":"crypto.ml-dataset.cell/v1","cell_index":0,
                "plan_sha256":digest(&plan_bytes),"config":config,
                "outcome":{"status":"unsupported","solver_completed":false},"artifacts":artifacts});
            let bytes = serde_json::to_vec(&manifest).unwrap();
            fs::write(cell.join("manifest.json"), &bytes).unwrap();
            artifacts.push(
                json!({"path":"manifest.json","role":"cell-manifest","encoding":"json",
                "bytes":bytes.len(),"sha256":digest(&bytes)}),
            );
            let objects: Vec<_> = artifacts
                .iter()
                .map(|artifact| {
                    let sha = artifact["sha256"].as_str().unwrap();
                    json!({"artifact":artifact,"object_key":format!("sha256/{}/{sha}",&sha[..2])})
                })
                .collect();
            let record = json!({"schema":"ml-dataset.publication/v1",
                "remote":"s3://setec-astronomy-artifacts/software-fixtures/native-transport",
                "manifest_sha256":digest(&bytes),"objects":objects});
            let receipt = temp.path().join("receipt.json");
            let bytes = serde_json::to_vec(&record).unwrap();
            fs::write(&receipt, &bytes).unwrap();
            let aws = temp.path().join("fake-aws");
            let mut body = String::from("[ \"$1\" = --region ] && [ \"$2\" = us-east-2 ]\n[ \"$3\" = s3api ]\n[ \"$5\" = --bucket ] && [ \"$6\" = setec-astronomy-artifacts ]\n[ \"$7\" = --key ]\ncase \"$8\" in\n");
            // Identical bytes can occur under several logical names; CAS deduplicates.
            let mut emitted = BTreeSet::new();
            for artifact in &artifacts {
                let sha = artifact["sha256"].as_str().unwrap();
                if !emitted.insert(sha) {
                    continue;
                }
                let key = format!(
                    "software-fixtures/native-transport/sha256/{}/{sha}",
                    &sha[..2]
                );
                let source = cell.join(artifact["path"].as_str().unwrap());
                body.push_str(&format!(
                    "{}) src={}; size={} ;;\n",
                    quoted(&key),
                    quoted(source.to_str().unwrap()),
                    artifact["bytes"]
                ));
            }
            body.push_str("*) exit 42 ;;\nesac\ncase \"$4\" in\nhead-object) printf '{\"ContentLength\":%s,\"ETag\":\"fixture-etag\"}' \"$size\" ;;\nget-object) [ \"$9\" = --if-match ] && [ \"${10}\" = fixture-etag ]; cp -- \"$src\" \"${11}\"; printf '{}' ;;\n*) exit 43 ;;\nesac\n");
            shell(&aws, &body);
            Self {
                _temp: temp,
                root,
                receipt,
                aws,
                expected: digest(&bytes),
            }
        }
        fn destination(&self) -> PathBuf {
            self._temp.path().join("restored")
        }
        fn fetch(&self) -> Result<Value> {
            fetch_with_aws(
                &self.receipt,
                &self.expected,
                &self.destination(),
                1 << 20,
                &self.aws,
            )
        }
        fn rewrite_receipt(&mut self, mutate: impl FnOnce(&mut Value)) {
            let mut record = crate::common::load(&self.receipt).unwrap();
            mutate(&mut record);
            let bytes = serde_json::to_vec(&record).unwrap();
            fs::write(&self.receipt, &bytes).unwrap();
            self.expected = digest(&bytes);
        }
        fn verifier(&self, completed: u64, before: &str) -> PathBuf {
            let binary = self._temp.path().join("fake-native");
            let status = json!({"schema":"crypto.ml-dataset.status/v1",
                "known_answer_replay_performed":true,"artifact_hashes_verified":true,
                "completed_receipts":completed});
            shell(&binary, &format!("[ \"$1\" = dataset ] && [ \"$2\" = verify ] && [ \"$3\" = --out ]\n{before}\nprintf '%s' {}", quoted(&status.to_string())));
            binary
        }
    }

    #[test]
    fn canonical_remote_validation() {
        assert_eq!(
            remote_prefix("s3://setec-astronomy-artifacts/a/b/").unwrap(),
            "a/b"
        );
        assert_eq!(remote_prefix("s3://setec-astronomy-artifacts").unwrap(), "");
        for uri in [
            "https://setec-astronomy-artifacts/a",
            "s3://other/a",
            "s3://setec-astronomy-artifacts.evil/a",
            "s3://setec-astronomy-artifacts//a",
            "s3://setec-astronomy-artifacts/a/../b",
            "s3://setec-astronomy-artifacts/a?x=1",
            "s3://setec-astronomy-artifacts/a#x",
            "s3://setec-astronomy-artifacts/%2e%2e/a",
        ] {
            assert!(remote_prefix(uri).is_err(), "{uri}");
        }
    }

    #[test]
    fn fetch_roundtrip_uses_etag_and_does_not_replace() {
        let f = Fixture::new();
        let report = f.fetch().unwrap();
        assert_eq!(report["remote_rechecked"], true);
        assert_eq!(report["checked_cells"], 1);
        assert_eq!(report["trusted_completion_receipt_sha256"], f.expected);
        assert!(!f.destination().join("RESTORE_INCOMPLETE").exists());
        assert!(f
            .fetch()
            .unwrap_err()
            .to_string()
            .contains("must not exist"));
    }

    #[test]
    fn invalid_receipt_hash_and_duplicate_json_fail_before_creation() {
        let mut f = Fixture::new();
        f.expected = "0".repeat(64);
        assert!(f.fetch().is_err());
        assert!(!f.destination().exists());
        let bytes = br#"{"schema":"ml-dataset.publication/v1","schema":"other"}"#;
        fs::write(&f.receipt, bytes).unwrap();
        f.expected = digest(bytes);
        assert!(format!("{:#}", f.fetch().unwrap_err()).contains("duplicate"));
        assert!(!f.destination().exists());
    }

    #[test]
    fn object_paths_sizes_and_cas_keys_preflight() {
        for case in 0..6 {
            let mut f = Fixture::new();
            f.rewrite_receipt(|r| match case {
                0 => r["objects"][0]["artifact"]["path"] = json!("../outside"),
                1 => r["objects"][0]["artifact"]["bytes"] = json!(true),
                2 => r["objects"][0]["artifact"]["bytes"] = json!(1u64 << 40),
                3 => r["objects"][0]["object_key"] = json!("wrong"),
                4 => {
                    let first = r["objects"][0].clone();
                    r["objects"].as_array_mut().unwrap().push(first);
                }
                _ => r["objects"][0]["artifact"]["sha256"] = json!("A".repeat(64)),
            });
            assert!(f.fetch().is_err(), "case {case}");
            assert!(!f.destination().exists(), "case {case}");
        }
    }

    #[test]
    fn failed_or_corrupt_download_retains_incomplete_isolation() {
        for body in ["exit 7", "if [ \"$4\" = head-object ]; then printf '{\"ContentLength\":1,\"ETag\":\"tag\"}'; else printf x > \"${11}\"; fi"] {
            let f = Fixture::new();
            shell(&f.aws, body);
            assert!(f.fetch().is_err());
            assert!(f.destination().join("RESTORE_INCOMPLETE").exists());
            assert!(f.destination().join("incomplete-download").is_dir());
            assert!(!f.destination().join("cells").exists());
        }
    }

    #[test]
    fn corrupt_get_is_checked_after_transport_success() {
        let f = Fixture::new();
        // Construct a fixed-size corrupt payload in Rust. The fake command
        // transports bytes only; it need not have an external text filter.
        let original = fs::read(f.root.join("cells/000000000000/plan.json")).unwrap();
        let mut corrupted = original.clone();
        corrupted[0] ^= 1;
        assert_eq!(corrupted.len(), original.len());
        assert_ne!(digest(&corrupted), digest(&original));
        let corrupt_source = f._temp.path().join("same-size-corrupt-plan");
        fs::write(&corrupt_source, corrupted).unwrap();
        let source = fs::read_to_string(&f.aws).unwrap();
        fs::write(
            &f.aws,
            source.replace(
                "cp -- \"$src\" \"${11}\";",
                &format!(
                    "cp -- {} \"${{11}}\";",
                    quoted(corrupt_source.to_str().unwrap())
                ),
            ),
        )
        .unwrap();
        let error = format!("{:#}", f.fetch().unwrap_err());
        assert!(error.contains("hash"), "unexpected rejection: {error}");
        assert!(f.destination().join("RESTORE_INCOMPLETE").exists());
        assert!(f
            .destination()
            .join("incomplete-download/plan.json")
            .exists());
    }

    #[test]
    fn extra_downloaded_member_is_not_admitted_by_valid_hash_alone() {
        let mut f = Fixture::new();
        f.rewrite_receipt(|r| {
            let mut extra = r["objects"][0].clone();
            extra["artifact"]["path"] = json!("unlisted.json");
            r["objects"].as_array_mut().unwrap().push(extra);
        });
        assert!(f
            .fetch()
            .unwrap_err()
            .to_string()
            .contains("member set differs"));
        assert!(f
            .destination()
            .join("incomplete-download/unlisted.json")
            .is_file());
        assert!(f.destination().join("RESTORE_INCOMPLETE").is_file());
    }

    #[test]
    fn reference_count_limit_rejects_before_any_download() {
        let mut f = Fixture::new();
        f.rewrite_receipt(|r| r["objects"] = Value::Array(vec![Value::Null; MAX_REFERENCES + 1]));
        assert!(f
            .fetch()
            .unwrap_err()
            .to_string()
            .contains("object count limit"));
        assert!(!f.destination().exists());
    }

    #[test]
    fn destination_symlink_rejected_without_touching_target() {
        let f = Fixture::new();
        let outside = f._temp.path().join("outside");
        fs::create_dir(&outside).unwrap();
        std::os::unix::fs::symlink(&outside, f.destination()).unwrap();
        assert!(f.fetch().is_err());
        assert_eq!(fs::read_dir(outside).unwrap().count(), 0);
    }

    #[test]
    fn bounded_process_rejects_output_flood_and_timeout() {
        let temp = tempfile::tempdir().unwrap();
        let script = temp.path().join("fake");
        shell(&script, "while :; do printf 0123456789; done");
        assert!(run_bounded(
            &mut Command::new(&script),
            Duration::from_secs(2),
            100,
            None
        )
        .unwrap_err()
        .to_string()
        .contains("output limit"));
        shell(&script, "sleep 30 & wait");
        let start = Instant::now();
        assert!(run_bounded(
            &mut Command::new(&script),
            Duration::from_millis(30),
            100,
            None
        )
        .unwrap_err()
        .to_string()
        .contains("timeout"));
        assert!(start.elapsed() < Duration::from_secs(2));
    }

    #[test]
    fn bounded_process_rejects_oversized_download() {
        let temp = tempfile::tempdir().unwrap();
        let script = temp.path().join("fake");
        let download = temp.path().join("download");
        shell(
            &script,
            &format!(
                "printf 0123456789 > {}; sleep 1",
                quoted(download.to_str().unwrap())
            ),
        );
        let error = run_bounded(
            &mut Command::new(&script),
            Duration::from_secs(2),
            100,
            Some((&download, 2)),
        )
        .unwrap_err();
        assert!(error.to_string().contains("declared size"));
        assert!(download.exists());
    }

    #[test]
    fn replay_requires_exact_coverage_and_records_hashes() {
        let f = Fixture::new();
        let (mut report, rows) = crate::audit::audit(&f.root, None, false).unwrap();
        let verifier = f.verifier(0, "");
        assert!(replay_native(&f.root, &verifier, &mut report, &rows)
            .unwrap_err()
            .to_string()
            .contains("coverage mismatch"));
        let verifier = f.verifier(1, "");
        replay_native(&f.root, &verifier, &mut report, &rows).unwrap();
        assert_eq!(report["producer_native_replay_passed"], true);
        assert_eq!(
            report["native_verifier_sha256"],
            hash_file(&verifier).unwrap()
        );
        assert_eq!(report["mathematics_independently_verified"], false);
    }

    #[test]
    fn replay_detects_verifier_and_dataset_mutation() {
        let f = Fixture::new();
        let (mut report, rows) = crate::audit::audit(&f.root, None, false).unwrap();
        let verifier = f.verifier(1, "printf '\\n#changed\\n' >> \"$0\"");
        assert!(replay_native(&f.root, &verifier, &mut report, &rows)
            .unwrap_err()
            .to_string()
            .contains("verifier changed"));
        let verifier = f.verifier(1, "printf tampered >> \"$4/cells/000000000000/stdout.log\"");
        assert!(replay_native(&f.root, &verifier, &mut report, &rows).is_err());
        assert_eq!(report["producer_native_replay_passed"], false);
    }

    #[test]
    fn replay_rejects_valid_but_changed_manifest_identity() {
        let f = Fixture::new();
        let (mut report, rows) = crate::audit::audit(&f.root, None, false).unwrap();
        let verifier = f.verifier(1, "manifest=\"$4/cells/000000000000/manifest.json\"\nsed 's/unsupported/timeout/' \"$manifest\" > \"$manifest.next\"\nmv \"$manifest.next\" \"$manifest\"");
        assert!(replay_native(&f.root, &verifier, &mut report, &rows)
            .unwrap_err()
            .to_string()
            .contains("cell contents changed"));
        assert_eq!(report["producer_native_replay_passed"], false);
    }

    #[test]
    fn replay_failure_does_not_leave_stale_success_label() {
        let f = Fixture::new();
        let (mut report, rows) = crate::audit::audit(&f.root, None, false).unwrap();
        report["producer_native_replay_passed"] = json!(true);
        let verifier = f.verifier(1, "exit 9");
        assert!(replay_native(&f.root, &verifier, &mut report, &rows)
            .unwrap_err()
            .to_string()
            .contains("native replay failed"));
        assert_eq!(report["producer_native_replay_passed"], false);
    }
}
