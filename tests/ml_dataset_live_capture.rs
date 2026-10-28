//! Process-boundary coverage for artifact-only dataset snapshots.
//!
//! These fixtures follow docs/ml-dataset/PARTIAL-SNAPSHOTS.md and the native
//! capture/snapshot contracts. They contain no generated curves or solver
//! output: the public/witness JSON is deliberately unsuitable for algebraic
//! replay. Real `crypto dataset` subprocesses use an isolated directory store.
//! An executable sentinel fails and records any accidental AWS invocation.
//! This does not exercise S3, a real interrupted worker, or solver recovery.

#![cfg(unix)]

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crypto_lib::hash::sha256::Sha256;
use crypto_lib::ml_dataset::capture::{AttemptBinding, CaptureCursor, CaptureEmitter};
use crypto_lib::ml_dataset::live::capture_limits;
use crypto_lib::ml_dataset::snapshot::{RestoreReport, SnapshotAnchor};
use crypto_lib::ml_dataset::types::ArtifactPayload;
use crypto_lib::ml_dataset::DatasetPlan;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

fn digest(bytes: &[u8]) -> String {
    let mut hash = Sha256::new();
    hash.update(bytes);
    hex::encode(hash.finalize())
}

fn write_json(path: &Path, value: &impl Serialize) {
    let mut bytes = serde_json::to_vec_pretty(value).unwrap();
    bytes.push(b'\n');
    fs::write(path, bytes).unwrap();
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> T {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

fn files(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn visit(root: &Path, directory: &Path, result: &mut BTreeMap<String, Vec<u8>>) {
        if !directory.exists() {
            return;
        }
        for entry in fs::read_dir(directory).unwrap() {
            let entry = entry.unwrap();
            let kind = entry.file_type().unwrap();
            assert!(!kind.is_symlink(), "unexpected fixture symlink");
            if kind.is_dir() {
                visit(root, &entry.path(), result);
            } else {
                assert!(kind.is_file());
                result.insert(
                    entry
                        .path()
                        .strip_prefix(root)
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_owned(),
                    fs::read(entry.path()).unwrap(),
                );
            }
        }
    }
    let mut result = BTreeMap::new();
    visit(root, root, &mut result);
    result
}

fn output_text(output: &Output) -> String {
    format!(
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn success(output: &Output) {
    assert!(output.status.success(), "{}", output_text(output));
}

fn failure(output: &Output, expected: &str) {
    assert!(!output.status.success(), "{}", output_text(output));
    assert!(
        output_text(output).contains(expected),
        "expected {expected:?}; {}",
        output_text(output)
    );
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct HelperResult {
    schema: String,
    request_sha256: String,
    cursor: CaptureCursor,
    anchor: Option<SnapshotAnchor>,
    copied_context: BTreeSet<String>,
    published: bool,
}

struct Helper {
    request: PathBuf,
    snapshot: PathBuf,
}

struct Fixture {
    root: PathBuf,
    binding: AttemptBinding,
}

impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "ml-capture-cli-{}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let cell = root.join("cell");
        fs::create_dir(&cell).unwrap();

        // This fixed ASCII body is already in the protocol's sorted-key,
        // compact canonical form; no algebraic solver or experiment is run.
        let canonical_protocol =
            br#"{"purpose":"artifact-only process fixture","schema":"experiment.protocol/v1"}"#;
        let mut protocol: Value = serde_json::from_slice(canonical_protocol).unwrap();
        protocol["freeze"] = json!({
            "status":"frozen", "protocol_sha256":digest(canonical_protocol),
            "filed_before_affected_data":true, "frozen_at":"2026-10-28T00:00:00Z"
        });
        write_json(&cell.join("protocol.json"), &protocol);
        let plan = DatasetPlan {
            schema: "crypto.ml-dataset.plan/v1".into(),
            field_sizes: vec![23],
            arities: vec![2],
            families: vec!["prime".into()],
            solvers: vec!["bsgs".into()],
            curve_seeds: 1,
            generators: 1,
            keys: 1,
            seed: 1,
            factor_base_size: 4,
            degree_bound: 3,
            work_limit: 10,
            twists: true,
            protocol_sha256: Some(digest(&fs::read(cell.join("protocol.json")).unwrap())),
            split_rule: "curve-twist-component-sha256-v1".into(),
        };
        let canonical_plan_sha = digest(&serde_json::to_vec(&plan).unwrap());
        write_json(&cell.join("plan.json"), &plan);
        write_json(
            &cell.join("run.json"),
            &json!({"schema":"crypto.ml-dataset.run/v1", "plan_sha256":canonical_plan_sha,
                "source_commit":"a".repeat(40), "binary_sha256":"b".repeat(64)}),
        );
        write_json(&cell.join("config.json"), &plan.cell(0).unwrap());
        write_json(
            &cell.join("attempt.json"),
            &json!({"schema":"crypto.ml-dataset.attempt/v1", "cell_index":0,
                "plan_sha256":canonical_plan_sha, "started_unix_nanos":"1"}),
        );
        let binding = AttemptBinding::from_cell_files(&cell).unwrap();
        assert_eq!(
            binding.plan_sha256,
            digest(&fs::read(cell.join("plan.json")).unwrap())
        );
        assert_ne!(binding.plan_sha256, canonical_plan_sha);

        let aws = root.join("unexpected-aws");
        fs::write(
            &aws,
            b"#!/bin/sh\nprintf 'unexpected AWS invocation\\n' >> \"$ML_CAPTURE_AWS_SENTINEL\"\nexit 97\n",
        )
        .unwrap();
        fs::set_permissions(aws, fs::Permissions::from_mode(0o700)).unwrap();
        Self { root, binding }
    }

    fn cell(&self) -> PathBuf {
        self.root.join("cell")
    }

    fn remote(&self) -> String {
        format!("file://{}", self.root.join("remote").display())
    }

    fn command(&self) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_crypto"));
        command
            .arg("dataset")
            .current_dir(&self.root)
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .env("CRYPTO_STORE_INDEX", self.root.join("index.json"))
            .env("CRYPTO_STORE_CACHE", self.root.join("cache"))
            .env("CRYPTO_STORE_AWS", self.root.join("unexpected-aws"))
            .env("ML_CAPTURE_AWS_SENTINEL", self.root.join("aws-called"))
            .stdin(Stdio::null())
            .process_group(0);
        command
    }

    fn run(&self, mut command: Command) -> Output {
        // Files avoid pipe deadlock and preserve failure diagnostics. The wall
        // bound is test hygiene, not a measured solver/performance result.
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let stdout = self.root.join(format!("cli-{id}.stdout"));
        let stderr = self.root.join(format!("cli-{id}.stderr"));
        command.stdout(File::create(&stdout).unwrap());
        command.stderr(File::create(&stderr).unwrap());
        let mut child = command
            .spawn()
            .expect("spawn the Cargo-built crypto binary");
        let start = Instant::now();
        let status = loop {
            if let Some(status) = child.try_wait().unwrap() {
                break status;
            }
            if start.elapsed() > Duration::from_secs(60) {
                unsafe { libc::kill(-(child.id() as i32), libc::SIGKILL) };
                let _ = child.wait();
                panic!(
                    "artifact-only CLI exceeded 60 seconds: {}",
                    fs::read_to_string(&stderr).unwrap()
                );
            }
            std::thread::sleep(Duration::from_millis(20));
        };
        unsafe { libc::kill(-(child.id() as i32), libc::SIGKILL) };
        Output {
            status,
            stdout: fs::read(stdout).unwrap(),
            stderr: fs::read(stderr).unwrap(),
        }
    }

    fn helper(&self, previous: Option<&HelperResult>, terminal: bool) -> Helper {
        let ordinal = previous
            .map(|r| r.anchor.as_ref().unwrap().ordinal + 1)
            .unwrap_or(0);
        let directory = self.root.join(format!("helper-{ordinal}"));
        fs::create_dir(&directory).unwrap();
        let snapshot = self.root.join(format!("snapshot-{ordinal}"));
        let request = directory.join("request.json");
        write_json(
            &request,
            &json!({
                "schema":"crypto.ml-dataset.snapshot-helper/v1",
                "cell":self.cell(), "snapshot":snapshot, "binding":self.binding,
                "cursor":previous.map(|r|r.cursor.clone()).unwrap_or_else(||
                    CaptureCursor::initial(&self.binding).unwrap()),
                "limits":capture_limits(64).unwrap(), "ordinal":ordinal,
                "previous":previous.and_then(|r|r.anchor.as_ref()),
                "copied_context":previous.map(|r|r.copied_context.clone()).unwrap_or_default(),
                "observation":{
                    "status":if terminal {"interrupted"} else {"running"},
                    "observed_unix_seconds":1000+ordinal,
                    "exit_code":null, "signal":null,
                    "reason":if terminal {Some("synthetic interruption fixture")} else {None}
                },
                "remote":self.remote(),
                "budget":{"policy":null,"allocation":null,"journal":null}
            }),
        );
        Helper { request, snapshot }
    }

    fn snapshot_output(&self, helper: &Helper) -> Output {
        let mut command = self.command();
        command
            .arg("snapshot-worker")
            .arg("--request")
            .arg(&helper.request);
        self.run(command)
    }

    fn snapshot(&self, helper: &Helper) -> HelperResult {
        success(&self.snapshot_output(helper));
        let result: HelperResult = read_json(&helper.request.parent().unwrap().join("result.json"));
        assert_eq!(result.schema, "crypto.ml-dataset.snapshot-helper-result/v1");
        assert_eq!(
            result.request_sha256,
            digest(&fs::read(&helper.request).unwrap())
        );
        assert!(result.published);
        let anchor = result.anchor.as_ref().unwrap();
        assert_eq!(anchor.binding_sha256, self.binding.digest().unwrap());
        self.assert_no_aws();
        result
    }

    fn restore_output(&self, anchor: &Path, destination: &Path, remote: &str) -> Output {
        let mut command = self.command();
        command
            .arg("restore-partial")
            .arg("--anchor")
            .arg(anchor)
            .arg("--out")
            .arg(destination)
            .arg("--remote")
            .arg(remote);
        self.run(command)
    }

    fn restore(&self, anchor: &SnapshotAnchor, destination: &Path) -> RestoreReport {
        let anchor_path = self.root.join("restore-anchor.json");
        write_json(&anchor_path, anchor);
        let output = self.restore_output(&anchor_path, destination, &self.remote());
        success(&output);
        let report: RestoreReport = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(&report.anchor, anchor);
        assert!(!report.cell_complete);
        assert!(!report.solver_state_restorable);
        assert!(!report.solver_resumed);
        assert!(!report.scientific_result_verified);
        assert!(!report.coverage.solver_resume_supported);
        assert!(!report.coverage.in_flight_solver_memory_captured);
        assert!(!report.coverage.uncommitted_event_tail_captured);
        assert!(!report.coverage.unemitted_backend_internals_captured);
        assert!(report.coverage.artifact_only);
        assert!(!destination.join("cells").exists());
        assert!(!report.restored_directory.join("manifest.json").exists());
        self.assert_no_aws();
        report
    }

    fn emitter(&self) -> CaptureEmitter {
        CaptureEmitter::create(
            &self.cell().join("live"),
            self.binding.clone(),
            capture_limits(64).unwrap(),
        )
        .unwrap()
    }

    fn assert_no_aws(&self) {
        assert!(
            !self.root.join("aws-called").exists(),
            "AWS process was unexpectedly started"
        );
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn cli_restores_exact_initial_context_and_committed_prefix_after_source_is_lost() {
    let fixture = Fixture::new();
    let initial_cell = files(&fixture.cell());
    let initial_helper = fixture.helper(None, false);
    let initial = fixture.snapshot(&initial_helper);
    assert_eq!(
        initial.cursor,
        CaptureCursor::initial(&fixture.binding).unwrap()
    );
    assert_eq!(initial.copied_context.len(), 5);
    assert!(!fixture.cell().join("live").exists());
    assert_eq!(files(&fixture.cell()), initial_cell);

    let initial_restore = fixture.restore(
        initial.anchor.as_ref().unwrap(),
        &fixture.root.join("initial-restore"),
    );
    assert_eq!(initial_restore.snapshot_count, 1);
    assert!(!initial_restore.coverage.capture_present);
    assert!(initial_restore.cursor.is_none());
    let expected_context: BTreeMap<_, _> = initial_cell
        .iter()
        .map(|(name, bytes)| (format!("context/{name}"), bytes.clone()))
        .collect();
    assert_eq!(
        files(&initial_restore.restored_directory.join("artifacts")),
        expected_context
    );

    // These deliberately invalid curve identities must be treated as opaque
    // artifact bytes. Restore would fail if it invoked identity/solver replay.
    for name in ["public.json", "witness.json", "factor-base.json"] {
        write_json(
            &fixture.cell().join(name),
            &json!({"fixture_only":name,"not_a_curve":true}),
        );
    }
    let mut emitter = fixture.emitter();
    emitter
        .event(&json!({"kind":"fixture_row","columns":[0,2],"values":[1,1]}))
        .unwrap();
    let basis = ArtifactPayload::json(
        "fixture-basis.json",
        "synthetic-algebraic-artifact",
        &json!({"rows":[[1,0,1]]}),
    )
    .unwrap();
    emitter.payload(&basis).unwrap();
    let first_helper = fixture.helper(Some(&initial), false);
    let first = fixture.snapshot(&first_helper);
    assert_eq!(first.cursor.event_count, 2);
    assert_eq!(first.cursor.payload_count, 1);

    emitter
        .event(&json!({"kind":"fixture_reduction","input":0,"pivot":2}))
        .unwrap();
    let reduction = ArtifactPayload::json(
        "fixture-reduction.json",
        "synthetic-algebraic-artifact",
        &json!({"before":[1,0,1],"after":[0,0,0]}),
    )
    .unwrap();
    emitter.payload(&reduction).unwrap();
    emitter.finish().unwrap();
    drop(emitter);
    fs::write(
        fixture.cell().join("live/payloads/orphan.json"),
        b"unacknowledged payload",
    )
    .unwrap();
    let mut stream = OpenOptions::new()
        .append(true)
        .open(fixture.cell().join("live/events.jsonl"))
        .unwrap();
    stream.write_all(b"{unacknowledged-torn-event").unwrap();
    stream.sync_all().unwrap();
    drop(stream);
    let terminal_helper = fixture.helper(Some(&first), true);
    let before_snapshot = files(&fixture.cell());
    let terminal = fixture.snapshot(&terminal_helper);
    assert_eq!(
        files(&fixture.cell()),
        before_snapshot,
        "helper mutated worker files"
    );
    assert_eq!(terminal.cursor.event_count, 4);
    assert_eq!(terminal.cursor.payload_count, 2);
    assert_eq!(terminal.cursor.next_chunk, 2);
    assert!(!fixture.cell().join("result.json").exists());
    assert!(!fixture.cell().join("manifest.json").exists());

    let mut expected = BTreeMap::new();
    for helper in [&initial_helper, &first_helper, &terminal_helper] {
        for (path, bytes) in files(&helper.snapshot) {
            if path.starts_with("context/")
                || path.starts_with("events/")
                || path.starts_with("payloads/")
            {
                assert!(
                    expected.insert(path, bytes).is_none(),
                    "snapshots redefined an artifact"
                );
            }
        }
    }
    let committed_events: Vec<u8> = expected
        .iter()
        .filter(|(name, _)| name.starts_with("events/"))
        .flat_map(|(_, bytes)| bytes.iter().copied())
        .collect();
    assert_eq!(committed_events.len() as u64, terminal.cursor.byte_end);
    assert_eq!(committed_events.iter().filter(|&&c| c == b'\n').count(), 4);
    assert!(!String::from_utf8_lossy(&committed_events).contains("unacknowledged"));

    // A fresh consumer must obtain every byte from the directory remote,
    // without the live cell, local sealed snapshots, or cached artifact files.
    fs::remove_dir_all(fixture.cell()).unwrap();
    for helper in [&initial_helper, &first_helper, &terminal_helper] {
        fs::remove_dir_all(&helper.snapshot).unwrap();
    }
    if fixture.root.join("cache").exists() {
        fs::remove_dir_all(fixture.root.join("cache")).unwrap();
    }
    let report = fixture.restore(
        terminal.anchor.as_ref().unwrap(),
        &fixture.root.join("restored"),
    );
    assert_eq!(report.snapshot_count, 3);
    assert_eq!(report.cursor.as_ref(), Some(&terminal.cursor));
    assert_eq!(report.coverage.committed_events, 4);
    assert_eq!(
        serde_json::to_value(&report.observation).unwrap()["status"],
        "interrupted"
    );
    assert_eq!(report.artifact_count as usize, expected.len());
    assert_eq!(
        report.artifact_bytes,
        expected
            .values()
            .map(|bytes| bytes.len() as u64)
            .sum::<u64>()
    );
    let restored = files(&report.restored_directory.join("artifacts"));
    assert_eq!(restored, expected);
    assert_eq!(restored["payloads/fixture-basis.json"], basis.bytes);
    assert_eq!(restored["payloads/fixture-reduction.json"], reduction.bytes);
    assert!(!restored.contains_key("payloads/orphan.json"));
}

#[test]
fn cli_failed_publication_retries_the_prepared_snapshot_after_live_head_advances() {
    let fixture = Fixture::new();
    let initial = fixture.snapshot(&fixture.helper(None, false));
    let mut emitter = fixture.emitter();
    emitter.event(&json!({"kind":"fixture_first"})).unwrap();
    let helper = fixture.helper(Some(&initial), false);

    // A conflicting local-store object makes publication fail after prepare.
    // Removing this test-only obstruction models recovery of the transport;
    // the request and its already sealed snapshot must remain unchanged.
    let chunk_hash = digest(&fs::read(fixture.cell().join("live/events.jsonl")).unwrap());
    let obstruction = fixture
        .root
        .join("remote")
        .join(format!("sha256/{}/{chunk_hash}", &chunk_hash[..2]));
    fs::create_dir_all(obstruction.parent().unwrap()).unwrap();
    fs::write(&obstruction, b"test-only conflicting object").unwrap();
    failure(
        &fixture.snapshot_output(&helper),
        "partial artifact size mismatch",
    );
    assert!(!helper
        .request
        .parent()
        .unwrap()
        .join("result.json")
        .exists());
    assert!(!helper.snapshot.join("anchor.json").exists());
    let prepared_path = helper.request.parent().unwrap().join("prepared.json");
    let prepared = fs::read(&prepared_path).unwrap();
    emitter.event(&json!({"kind":"fixture_later"})).unwrap();
    fs::remove_file(obstruction).unwrap();
    let retried = fixture.snapshot(&helper);
    assert_eq!(retried.cursor.event_count, 1);
    assert_eq!(fs::read(prepared_path).unwrap(), prepared);
    let remote = files(&fixture.root.join("remote"));
    let repeated = fixture.snapshot(&helper);
    assert_eq!(repeated.cursor, retried.cursor);
    assert_eq!(repeated.anchor, retried.anchor);
    assert_eq!(files(&fixture.root.join("remote")), remote);

    let next = fixture.snapshot(&fixture.helper(Some(&retried), true));
    assert_eq!(next.cursor.event_count, 2);
    assert_eq!(next.cursor.next_chunk, 2);
    fixture.assert_no_aws();
}

#[test]
fn cli_rejects_prepared_snapshot_rebound_to_another_attempt() {
    let fixture = Fixture::new();
    let helper = fixture.helper(None, false);
    fixture.snapshot(&helper);
    let remote = files(&fixture.root.join("remote"));
    let result = fs::read(helper.request.parent().unwrap().join("result.json")).unwrap();
    let prepared_path = helper.request.parent().unwrap().join("prepared.json");
    let mut prepared: Value = read_json(&prepared_path);
    prepared["snapshot"]["binding"]["attempt_sha256"] = json!("c".repeat(64));
    write_json(&prepared_path, &prepared);
    failure(
        &fixture.snapshot_output(&helper),
        "prepared snapshot belongs to a different helper request",
    );
    assert_eq!(files(&fixture.root.join("remote")), remote);
    assert_eq!(
        fs::read(helper.request.parent().unwrap().join("result.json")).unwrap(),
        result
    );
    fixture.assert_no_aws();
}

#[test]
fn cli_rejects_changed_attempt_before_publication() {
    let fixture = Fixture::new();
    let helper = fixture.helper(None, false);
    let mut attempt: Value = read_json(&fixture.cell().join("attempt.json"));
    attempt["started_unix_nanos"] = json!("2");
    write_json(&fixture.cell().join("attempt.json"), &attempt);
    failure(
        &fixture.snapshot_output(&helper),
        "cell binding changed before snapshot",
    );
    assert!(!fixture.root.join("remote").exists());
    assert!(!helper
        .request
        .parent()
        .unwrap()
        .join("result.json")
        .exists());
    assert!(!helper.snapshot.join("anchor.json").exists());
    fixture.assert_no_aws();
}

#[test]
fn cli_rejects_corrupt_committed_event_without_extending_remote_chain() {
    let fixture = Fixture::new();
    let initial = fixture.snapshot(&fixture.helper(None, false));
    let mut emitter = fixture.emitter();
    emitter
        .event(&json!({"kind":"fixture_row", "fixture_value":7}))
        .unwrap();
    emitter.finish().unwrap();
    drop(emitter);
    let stream = fixture.cell().join("live/events.jsonl");
    let original = fs::read_to_string(&stream).unwrap();
    let altered = original.replace("\"fixture_value\":7", "\"fixture_value\":8");
    assert_ne!(original, altered);
    assert_eq!(original.len(), altered.len());
    fs::write(stream, altered).unwrap();
    let remote = files(&fixture.root.join("remote"));
    let helper = fixture.helper(Some(&initial), true);
    failure(
        &fixture.snapshot_output(&helper),
        "captured prefix does not match committed watermark",
    );
    assert_eq!(files(&fixture.root.join("remote")), remote);
    assert!(!helper
        .request
        .parent()
        .unwrap()
        .join("result.json")
        .exists());
    assert!(!helper.snapshot.join("anchor.json").exists());
    fixture.assert_no_aws();
}

#[test]
fn cli_restore_rejects_corrupt_remote_payload_without_a_restore_acknowledgement() {
    let fixture = Fixture::new();
    let initial = fixture.snapshot(&fixture.helper(None, false));
    let mut emitter = fixture.emitter();
    let payload = ArtifactPayload::json(
        "fixture-matrix.json",
        "synthetic-matrix",
        &json!({"rows":[[0,1],[1,0]]}),
    )
    .unwrap();
    emitter.payload(&payload).unwrap();
    let terminal = fixture.snapshot(&fixture.helper(Some(&initial), true));
    let hash = digest(&payload.bytes);
    let object = fixture
        .root
        .join("remote")
        .join(format!("sha256/{}/{hash}", &hash[..2]));
    let mut corrupt = fs::read(&object).unwrap();
    assert_eq!(corrupt, payload.bytes);
    corrupt[0] ^= 1;
    fs::write(object, corrupt).unwrap();
    let anchor = fixture.root.join("anchor.json");
    write_json(&anchor, terminal.anchor.as_ref().unwrap());
    let destination = fixture.root.join("failed-restore");
    let output = fixture.restore_output(&anchor, &destination, &fixture.remote());
    failure(&output, "partial artifact hash mismatch");
    assert!(!destination.join("cells").exists());
    assert!(files(&destination)
        .keys()
        .all(|path| !path.contains("/restores/")));
    assert!(!files(&destination)
        .keys()
        .any(|path| path.ends_with("/payloads/fixture-matrix.json")));
    fixture.assert_no_aws();
}

#[test]
fn cli_s3_without_complete_budget_arguments_rejects_before_aws_process() {
    let fixture = Fixture::new();
    let helper = fixture.helper(None, false);
    let mut request: Value = read_json(&helper.request);
    request["remote"] = json!("s3://setec-astronomy-artifacts/synthetic-cli-fixture");
    write_json(&helper.request, &request);
    failure(
        &fixture.snapshot_output(&helper),
        "S3 dataset operations require a cumulative budget",
    );
    assert!(!helper
        .request
        .parent()
        .unwrap()
        .join("result.json")
        .exists());
    assert!(!helper.snapshot.join("anchor.json").exists());

    let unused_anchor = fixture.root.join("unused-anchor.json");
    write_json(
        &unused_anchor,
        &json!({"must_not_be_read_before_budget_admission":true}),
    );
    let destination = fixture.root.join("no-budget-restore");
    failure(
        &fixture.restore_output(
            &unused_anchor,
            &destination,
            "s3://setec-astronomy-artifacts/synthetic-cli-fixture",
        ),
        "S3 dataset operations require a cumulative budget",
    );
    let mut command = fixture.command();
    command
        .arg("restore-partial")
        .arg("--anchor")
        .arg(&unused_anchor)
        .arg("--out")
        .arg(&destination)
        .arg("--remote")
        .arg("s3://setec-astronomy-artifacts/synthetic-cli-fixture")
        .arg("--budget-policy")
        .arg(fixture.root.join("absent-policy.json"));
    failure(&fixture.run(command), "provide all of --budget-policy");
    assert!(!destination.exists());
    fixture.assert_no_aws();
}
