//! Driver controls with stand-in arm executables. No solver runs here: the
//! arms copy a genuine candidate receipt (or a corrupted copy, or nothing),
//! so the tests exercise scheduling, retention and independent validation.
#![cfg(unix)]

use serde_json::Value;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const PANEL: &str = env!("CARGO_BIN_EXE_hashed-ecdl-panel");
/// Genuine receipt produced by `hashed-ecdl solve` (candidate c866ee97) for
/// problems/p5-development-s001-t0.json; expected scalar 1.
const RECEIPT: &str = include_str!("data/panel-receipt-p5-development-s001-t0.json");
const CASE: &str = "p5-development-s001-t0";

struct Fixture {
    dir: PathBuf,
}

impl Fixture {
    fn new(name: &str) -> Self {
        let dir =
            std::env::temp_dir().join(format!("hashed-ecdl-driver-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir(&dir).unwrap();
        let ok = Command::new(PANEL)
            .arg("prepare")
            .arg(dir.join("panel"))
            .output()
            .unwrap();
        assert!(ok.status.success(), "{ok:?}");
        let hashes = Command::new(PANEL)
            .arg("hash")
            .arg(dir.join("panel"))
            .output()
            .unwrap();
        assert!(hashes.status.success());
        fs::write(dir.join("SHA256SUMS"), hashes.stdout).unwrap();
        Fixture { dir }
    }

    /// An arm that writes `receipt` (if any) for `solve` and exits `code`;
    /// `verify` exits 0; `sleep` seconds before answering.
    fn arm(&self, name: &str, receipt: Option<&str>, code: i32, sleep: u32) -> PathBuf {
        let mut body = String::from("#!/bin/sh\n");
        if sleep > 0 {
            body.push_str(&format!("sleep {sleep}\n"));
        }
        body.push_str("if [ \"$1\" = verify ]; then exit 0; fi\n");
        if let Some(receipt) = receipt {
            let source = self.dir.join(format!("{name}.receipt.json"));
            fs::write(&source, receipt).unwrap();
            body.push_str(&format!("cp '{}' \"$3\"\n", source.display()));
        }
        body.push_str(&format!("exit {code}\n"));
        let path = self.dir.join(format!("{name}.sh"));
        fs::write(&path, body).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
        path
    }

    fn run(&self, out: &str, baseline: &Path, candidate: &Path, extra: &[&str]) -> Output {
        let mut command = Command::new(PANEL);
        command
            .arg("run")
            .arg("--panel")
            .arg(self.dir.join("panel"))
            .arg("--inputs-sha256")
            .arg(self.dir.join("SHA256SUMS"))
            .arg("--baseline")
            .arg(baseline)
            .arg("--candidate")
            .arg(candidate)
            .arg("--out")
            .arg(self.dir.join(out))
            .args(["--seed", "7", "--case-limit-secs", "20"]);
        command.args(extra).output().unwrap()
    }

    fn rows(&self, out: &str) -> Vec<Value> {
        fs::read_to_string(self.dir.join(out).join("rows.jsonl"))
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect()
    }

    fn analysis(&self, out: &str) -> Value {
        serde_json::from_slice(&fs::read(self.dir.join(out).join("analysis.json")).unwrap())
            .unwrap()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

const SMALL: &[&str] = &[
    "--cases",
    CASE,
    "--aa-rounds",
    "1",
    "--ab-rounds",
    "1",
    "--allow-incomplete-panel",
];

#[test]
fn aa_identity_validates_every_row_and_matches_certificates() {
    let f = Fixture::new("aa");
    let good = f.arm("good", Some(RECEIPT), 0, 0);
    let out = f.run("out", &good, &good, SMALL);
    assert!(out.status.success(), "{out:?}");
    let rows = f.rows("out");
    // One warmup pair, one A/A pair and one A/B pair.
    assert_eq!(rows.len(), 6);
    for row in &rows {
        assert_eq!(row["status"], "validated", "{row}");
        assert_eq!(row["replay_status"], "verified");
        assert_eq!(row["scalar"], "1");
        assert!(row["checks"]
            .as_object()
            .unwrap()
            .values()
            .all(|v| v == true));
        assert!(row["s_over_rho"].is_null());
        assert!(row["phase_ns"]["witness_search"].is_null());
    }
    let analysis = f.analysis("out");
    assert_eq!(analysis["aa"]["complete_pairs"], 1);
    assert_eq!(analysis["aa"]["certificate_pairs_identical"], 1);
    assert_eq!(analysis["ab"]["certificate_pairs_identical"], 1);
    assert_eq!(analysis["protocol_complete"], false);
    assert_eq!(analysis["evidence_class"], "diagnostic_incomplete_panel");
    assert!(analysis["runtime_claim_admissible"].is_null());
    assert_eq!(analysis["arms"]["unchanged_after_run"], true);
    // `analyze` recomputes the same paired summary from retained rows.
    let again = Command::new(PANEL)
        .arg("analyze")
        .arg(f.dir.join("out/rows.jsonl"))
        .output()
        .unwrap();
    assert!(again.status.success());
    let again: Value = serde_json::from_slice(&again.stdout).unwrap();
    assert_eq!(again["aa"], analysis["aa"]);
    assert_eq!(again["ab"], analysis["ab"]);
    // Two copies of the same rows are refused, never silently merged.
    let rows_text = fs::read_to_string(f.dir.join("out/rows.jsonl")).unwrap();
    fs::write(f.dir.join("doubled.jsonl"), rows_text.repeat(2)).unwrap();
    let doubled = Command::new(PANEL)
        .arg("analyze")
        .arg(f.dir.join("doubled.jsonl"))
        .output()
        .unwrap();
    assert!(!doubled.status.success());
    assert!(String::from_utf8_lossy(&doubled.stderr).contains("repeat a (round, case, member)"));
}

fn corrupted(edit: impl Fn(&mut Value)) -> String {
    let mut receipt: Value = serde_json::from_str(RECEIPT).unwrap();
    edit(&mut receipt);
    serde_json::to_string_pretty(&receipt).unwrap()
}

#[test]
fn corrupted_certificates_are_rejected_and_retained() {
    let f = Fixture::new("corrupt");
    let good = f.arm("good", Some(RECEIPT), 0, 0);
    // A wrong scalar, kept consistent with the diagnostic copy.
    let wrong_scalar = f.arm(
        "scalar",
        Some(&corrupted(|r| {
            r["certificate"]["scalar"] = serde_json::json!([2]);
            r["diagnostics"]["scalar_decimal"] = serde_json::json!("2");
        })),
        0,
        0,
    );
    // The right scalar with a forged dependency vector: only the certificate
    // replay can notice this one.
    let forged = f.arm(
        "forged",
        Some(&corrupted(|r| {
            let dependency = r["certificate"]["dependency"].as_array_mut().unwrap();
            let first = dependency[0].as_array_mut().unwrap();
            if first.is_empty() {
                first.push(serde_json::json!(1));
            } else {
                let x = first[0].as_u64().unwrap();
                first[0] = serde_json::json!((x + 1) % 7);
            }
        })),
        0,
        0,
    );
    // A different echoed problem.
    let echo = f.arm(
        "echo",
        Some(&corrupted(|r| {
            r["problem"]["trials"] = serde_json::json!(15)
        })),
        0,
        0,
    );
    let not_json = f.arm(
        "notjson",
        Some("{\"schema\": \"hashed-ecdl-tensor/v1\""),
        0,
        0,
    );
    for (name, arm, failed_check) in [
        ("scalar", &wrong_scalar, "oracle_scalar_maps_to_target"),
        ("forged", &forged, "library_certificate_replay"),
        ("echo", &echo, "echoed_problem"),
        ("notjson", &not_json, "strict_json"),
    ] {
        let out = f.run(name, &good, arm, SMALL);
        assert!(out.status.success(), "{name}: {out:?}");
        let rows = f.rows(name);
        assert_eq!(rows.len(), 6, "{name}");
        for row in &rows {
            if row["arm"] == "candidate" {
                assert_eq!(row["status"], "rejected", "{name}: {row}");
                assert_eq!(row["checks"][failed_check], false, "{name}: {row}");
                assert!(row["reason"].is_string());
            } else {
                assert_eq!(row["status"], "validated", "{name}: {row}");
            }
        }
        let analysis = f.analysis(name);
        assert_eq!(analysis["ab"]["complete_pairs"], 0, "{name}");
        assert_eq!(analysis["ab"]["excluded_pairs"], 1, "{name}");
        assert_eq!(analysis["aa"]["complete_pairs"], 1, "{name}");
    }
}

#[test]
fn missing_failed_and_timed_out_rows_are_retained() {
    let f = Fixture::new("missing");
    let good = f.arm("good", Some(RECEIPT), 0, 0);
    let silent = f.arm("silent", None, 0, 0);
    let failing = f.arm("failing", None, 3, 0);
    let slow = f.arm("slow", Some(RECEIPT), 0, 30);
    for (name, arm, status) in [
        ("silent", &silent, "rejected"),
        ("failing", &failing, "failed"),
        ("slow", &slow, "timeout"),
    ] {
        let mut extra = SMALL.to_vec();
        extra.extend(["--case-limit-secs", "1"]);
        let out = f.run(name, &good, arm, &extra);
        assert!(out.status.success(), "{name}: {out:?}");
        let rows = f.rows(name);
        assert_eq!(
            rows.len(),
            6,
            "{name}: every scheduled execution keeps a row"
        );
        let candidate: Vec<_> = rows.iter().filter(|r| r["arm"] == "candidate").collect();
        assert_eq!(candidate.len(), 2);
        for row in candidate {
            assert_eq!(row["status"], status, "{name}: {row}");
            assert_eq!(row["checks"]["receipt_present"], false, "{name}");
        }
        if name == "failing" {
            assert!(rows.iter().any(|r| r["exit_code"] == 3));
        }
        if name == "silent" {
            assert!(rows.iter().any(|r| r["reason"] == "missing receipt"));
        }
        let analysis = f.analysis(name);
        assert_eq!(analysis["ab"]["status_counts"]["member1"][status], 1);
        assert!(analysis["ab"]["mean_log_ratio"].is_null());
    }
    // A zero deadline skips everything, but still records every execution.
    let out = f.run(
        "deadline",
        &good,
        &good,
        &[SMALL, &["--deadline-secs", "0"]].concat(),
    );
    assert!(out.status.success(), "{out:?}");
    let rows = f.rows("deadline");
    assert_eq!(rows.len(), 6);
    assert!(rows.iter().all(|r| r["status"] == "not_run_deadline"));
}

#[test]
fn incomplete_schedules_unpinned_inputs_and_old_evidence_are_refused() {
    let f = Fixture::new("refuse");
    let good = f.arm("good", Some(RECEIPT), 0, 0);
    let out = f.run("partial", &good, &good, &["--cases", CASE]);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("--allow-incomplete-panel"));
    assert!(!f.dir.join("partial").exists());
    let out = f.run("rounds", &good, &good, &["--aa-rounds", "4"]);
    assert!(!out.status.success());
    // A repeated case id would put two pairs under one analysis key.
    let twice = format!("{CASE},{CASE}");
    let out = f.run(
        "twice",
        &good,
        &good,
        &["--cases", &twice, "--allow-incomplete-panel"],
    );
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("more than once"));
    assert!(!f.dir.join("twice").exists());
    // An existing output directory is never reused.
    fs::create_dir(f.dir.join("exists")).unwrap();
    let out = f.run("exists", &good, &good, SMALL);
    assert!(!out.status.success());
    // A changed input byte is refused before any arm runs.
    let problem = f.dir.join(format!("panel/problems/{CASE}.json"));
    let mut bytes = fs::read(&problem).unwrap();
    bytes.push(b'\n');
    fs::write(&problem, bytes).unwrap();
    let out = f.run("tampered", &good, &good, SMALL);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("not pinned"));
    assert!(!f.dir.join("tampered").exists());
}
