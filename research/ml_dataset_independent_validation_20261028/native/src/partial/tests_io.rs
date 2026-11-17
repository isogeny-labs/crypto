use super::{
    budget::Budget,
    io::Root,
    restore,
    source::{CommandRunner, RemoteSource},
    test_support::*,
};
use serde_json::json;
use std::{
    fs,
    os::unix::process::ExitStatusExt,
    path::{Path, PathBuf},
    process::Command,
    sync::{Arc, Mutex},
};

#[test]
fn root_replacement_cannot_mix_valid_bytes_and_corrupt_inventory() {
    let mut f = Fixture::new(false);
    f.add_context(0, "host.json", b"opaque host A".to_vec());
    let a = f.publish();
    let b = f.publish();
    a.restore().unwrap();
    b.restore().unwrap();
    fs::write(a.artifact_path("context/host.json"), b"corrupt host!").unwrap();
    let root = Root::open(&a.out).unwrap();
    fs::rename(&a.out, a.temp.path().join("original-root")).unwrap();
    fs::rename(&b.out, &a.out).unwrap();
    let error =
        restore::audit_root(&root, &a.pin, false, false, false, &mut Budget::new()).unwrap_err();
    assert!(
        format!("{error:#}").contains("consumed artifact identity"),
        "{error:#}"
    );
    // The independently opened replacement is valid, proving this is not a
    // fixture transport failure; the retained descriptor still sees corruption.
    assert!(super::audit_restored(&a.out, &a.pin).is_ok());
    a.retain(
        "R09-root-swap",
        "same opened root must supply byte and namespace proof",
        &json!({"error":format!("{error:#}")}),
    );
}
#[test]
fn fd_pinned_inventory_can_be_repeated_without_shared_readdir_offset() {
    let p = Fixture::new(false).publish();
    p.restore().unwrap();
    let root = Root::open(&p.out).unwrap();
    assert_eq!(
        root.inventory(&mut Budget::new()).unwrap(),
        root.inventory(&mut Budget::new()).unwrap()
    );
}
#[test]
fn syscall_failures_keep_incomplete_state_and_refuse_automatic_adoption() {
    for point in [
        "file-sync",
        "intermediate-sync",
        "rename-before",
        "rename-after",
        "marker-remove",
    ] {
        let p = Fixture::new(false).publish();
        let guard = super::io::inject_fault(point);
        let error = p.restore().unwrap_err();
        drop(guard);
        assert!(
            format!("{error:#}").contains(&format!("injected partial I/O failure: {point}")),
            "{error:#}"
        );
        assert!(p.out.join(restore::MARKER).is_file());
        assert!(format!("{:#}", p.restore().unwrap_err()).contains("destination must be new"));
        p.retain(
            &format!("R13-{point}"),
            "injected I/O boundary failure; marker retained; no adoption",
            &json!({"error":format!("{error:#}")}),
        );
    }
}
#[test]
fn exact_namespace_rejects_extra_empty_directory_and_missing_member() {
    let p = Fixture::new(false).publish();
    p.restore().unwrap();
    fs::create_dir(p.out.join("extra")).unwrap();
    assert!(format!("{:#}", p.audit().unwrap_err()).contains("exact restored namespace"));
    fs::remove_dir(p.out.join("extra")).unwrap();
    fs::remove_file(p.artifact_path("context/config.json")).unwrap();
    assert!(p.audit().is_err());
}
#[test]
fn phase_ledger_is_shared_through_prefixes_and_final_audit() {
    let single = Fixture::new(true).publish();
    let baseline = single.restore().unwrap();
    let single_limit = baseline["phase_modeled_allocation_used"].as_u64().unwrap() as usize;
    let mut f = Fixture::new(true);
    for _ in 0..3 {
        f.push_snapshot(true);
    }
    let p = f.publish();
    let mut ledger = Budget::new();
    let (anchor, raw) = restore::pinned_anchor(&p.anchor, &p.pin, &mut ledger).unwrap();
    let mut source = super::source::LocalSource::mirror(&p.mirror).unwrap();
    // Policy-only prefixes repeatedly reconstruct/parse the same checked
    // context under one child ledger. A separate reset per prefix would fit.
    ledger.phase = crate::phase::PhaseBudget::new(single_limit);
    let error =
        restore::restore(&mut source, anchor, &raw, &p.pin, &p.out, &mut ledger).unwrap_err();
    assert!(format!("{error:#}").contains("consumer_resource_limit"));
    assert!(p.out.join(restore::MARKER).is_file());
    assert!(ledger.phase.used() > 0);
    p.retain(
        "R06-cumulative-phase",
        "one cumulative child ledger rejects, incomplete marker retained",
        &json!({"error":format!("{error:#}"),"phase_reserved":ledger.phase.used()}),
    );
}
#[test]
fn remote_retry_override_survives_conflicting_inherited_setting() {
    let mut command = Command::new(std::env::current_exe().unwrap());
    command.args(["--exact","partial::tests_io::p15_mock_remote_full_chain_conditioned_get_and_false_actual_remote_claim"])
        .env("AWS_MAX_ATTEMPTS","11").env("AWS_PAGER","invalid-pager").env("AWS_CLI_AUTO_PROMPT","on")
        .env_remove("PARTIAL_FIXTURE_OUTPUT");
    let out = crate::transport::run_bounded(
        &mut command,
        std::time::Duration::from_secs(10),
        32768,
        None,
    )
    .unwrap();
    crate::transport::require_success(&out, "inherited remote command settings").unwrap();
}

struct Mock {
    mirror: PathBuf,
    calls: Arc<Mutex<Vec<Vec<String>>>>,
    failure: Option<&'static str>,
}
impl CommandRunner for Mock {
    fn run(
        &mut self,
        command: &mut Command,
        guard: Option<(&Path, u64)>,
    ) -> anyhow::Result<crate::transport::CommandOutput> {
        let args: Vec<String> = command
            .get_args()
            .map(|a| a.to_str().unwrap().to_owned())
            .collect();
        let env: std::collections::BTreeMap<_, _> = command
            .get_envs()
            .map(|(k, v)| {
                (
                    k.to_str().unwrap().to_owned(),
                    v.unwrap().to_str().unwrap().to_owned(),
                )
            })
            .collect();
        assert_eq!(env["AWS_MAX_ATTEMPTS"], "1");
        assert_eq!(env["AWS_PAGER"], "");
        assert_eq!(env["AWS_CLI_AUTO_PROMPT"], "off");
        assert!(args.windows(2).any(|p| p == ["--output", "json"]));
        assert!(args.contains(&"--no-cli-pager".to_owned()));
        let value = |flag: &str| args[args.iter().position(|a| a == flag).unwrap() + 1].clone();
        assert_eq!(value("--region"), "us-east-2");
        assert_eq!(value("--bucket"), "setec-astronomy-artifacts");
        let key = value("--key");
        let relative = key.strip_prefix("prefix/").unwrap();
        let bytes = fs::read(self.mirror.join(relative)).unwrap();
        self.calls.lock().unwrap().push(args.clone());
        let mut stdout = Vec::new();
        let mut status = 0;
        let mut stderr = Vec::new();
        if args.contains(&"head-object".to_owned()) {
            assert!(guard.is_none());
            let length = bytes.len() + usize::from(self.failure == Some("head-length"));
            stdout = serde_json::to_vec(
                &json!({"ContentLength":length,"ETag":"\"immutable mock etag\""}),
            )
            .unwrap();
        } else {
            assert!(args.contains(&"get-object".to_owned()));
            assert_eq!(value("--if-match"), "\"immutable mock etag\"");
            let (path, size) = guard.unwrap();
            assert_eq!(size, bytes.len() as u64);
            assert_eq!(Path::new(args.last().unwrap()), path);
            match self.failure {
                Some("412") => {
                    status = 1 << 8;
                    stderr = b"412 PreconditionFailed".to_vec();
                }
                Some("timeout") => anyhow::bail!("mock command exceeded time bound"),
                Some("corrupt") => {
                    let mut changed = bytes;
                    changed[0] ^= 1;
                    fs::write(path, changed).unwrap();
                }
                Some("overlong") => {
                    let mut changed = bytes;
                    changed.push(0);
                    fs::write(path, changed).unwrap();
                }
                _ => fs::write(path, bytes).unwrap(),
            }
        }
        Ok(crate::transport::CommandOutput {
            status: std::process::ExitStatus::from_raw(status),
            stdout,
            stderr,
        })
    }
}
#[test]
fn p15_mock_remote_full_chain_conditioned_get_and_false_actual_remote_claim() {
    let mut f = Fixture::new(false);
    f.remote = "s3://setec-astronomy-artifacts/prefix".into();
    f.capture(
        0,
        vec![json!({"kind":"checkpoint"})],
        vec![("payloads/raw.bin", vec![0, 255])],
        None,
    );
    let p = f.publish();
    let calls = Arc::new(Mutex::new(Vec::new()));
    let mut b = Budget::new();
    let (anchor, raw) = restore::pinned_anchor(&p.anchor, &p.pin, &mut b).unwrap();
    let mut source = RemoteSource::mocked(
        &f.remote,
        Box::new(Mock {
            mirror: p.mirror.clone(),
            calls: calls.clone(),
            failure: None,
        }),
        &mut b,
    )
    .unwrap();
    let result = restore::restore(&mut source, anchor, &raw, &p.pin, &p.out, &mut b).unwrap();
    assert_eq!(result["fresh_remote_readback_verified"], false);
    assert_eq!(result["cursor"]["event_count"], 2);
    let calls = calls.lock().unwrap();
    assert_eq!(calls.len(), 20);
    for pair in calls.chunks(2) {
        assert!(pair[0].contains(&"head-object".into()));
        assert!(pair[1].contains(&"get-object".into()));
    }
    p.retain(
        "P15-mocked-remote",
        "HEAD/conditioned GET only; no actual remote claim",
        &result,
    );
}
#[test]
fn remote_failures_have_no_retry_or_fallback() {
    for mode in ["head-length", "412", "timeout", "corrupt", "overlong"] {
        let mut f = Fixture::new(false);
        f.remote = "s3://setec-astronomy-artifacts/prefix".into();
        let p = f.publish();
        let calls = Arc::new(Mutex::new(Vec::new()));
        let mut b = Budget::new();
        let (anchor, raw) = restore::pinned_anchor(&p.anchor, &p.pin, &mut b).unwrap();
        let mut source = RemoteSource::mocked(
            &f.remote,
            Box::new(Mock {
                mirror: p.mirror.clone(),
                calls: calls.clone(),
                failure: Some(mode),
            }),
            &mut b,
        )
        .unwrap();
        let error =
            restore::restore(&mut source, anchor, &raw, &p.pin, &p.out, &mut b).unwrap_err();
        let count = calls.lock().unwrap().len();
        assert_eq!(count, if mode == "head-length" { 1 } else { 2 });
        assert!(p.out.join(restore::MARKER).is_file());
        p.retain(
            &format!("R11-{mode}"),
            "single HEAD/GET at most; no retry/fallback",
            &json!({"error":format!("{error:#}"),"calls":count}),
        );
    }
}
#[test]
fn caller_pin_precedes_new_destination_and_file_io() {
    let p = Fixture::new(false).publish();
    let pin = "0".repeat(64);
    assert!(format!(
        "{:#}",
        super::restore_local(&p.anchor, &pin, &p.mirror, &p.out).unwrap_err()
    )
    .contains("caller anchor pin mismatch"));
    assert!(!p.out.exists());
}

#[test]
fn exclusive_link_install_preserves_complete_bytes_and_never_overwrites() {
    use std::io::Write;
    let _force = super::io::force_link_install();
    let p = Fixture::new(true).publish();
    let report = p.restore().unwrap();
    assert_eq!(report["partial_snapshot_chain_verified"], true);
    p.audit().unwrap();
    p.retain(
        "R14-link-install-positive",
        "exclusive link installation and stage removal preserve complete proof",
        &report,
    );
    let temp = tempfile::tempdir().unwrap();
    let root = Root::open(temp.path()).unwrap();
    root.create_file("stage")
        .unwrap()
        .write_all(b"new")
        .unwrap();
    root.create_file("exists")
        .unwrap()
        .write_all(b"old")
        .unwrap();
    assert!(
        format!("{:#}", root.rename_new("stage", "exists").unwrap_err())
            .contains("exclusive link installation")
    );
    assert_eq!(fs::read(temp.path().join("exists")).unwrap(), b"old");
    assert_eq!(fs::read(temp.path().join("stage")).unwrap(), b"new");
    std::os::unix::fs::symlink("exists", temp.path().join("symlink")).unwrap();
    assert!(root.rename_new("stage", "symlink").is_err());
    assert_eq!(
        fs::read_link(temp.path().join("symlink")).unwrap(),
        Path::new("exists")
    );
    assert_eq!(fs::read(temp.path().join("exists")).unwrap(), b"old");
    assert_eq!(fs::read(temp.path().join("stage")).unwrap(), b"new");
    fs::hard_link(temp.path().join("stage"), temp.path().join("extra-link")).unwrap();
    assert!(
        format!("{:#}", root.rename_new("stage", "new-name").unwrap_err())
            .contains("one regular staged file")
    );
    fs::remove_file(temp.path().join("extra-link")).unwrap();
    fs::create_dir(temp.path().join("directory-stage")).unwrap();
    assert!(format!(
        "{:#}",
        root.rename_new("directory-stage", "new-name").unwrap_err()
    )
    .contains("one regular staged file"));
    assert!(root.rename_new("symlink", "new-name").is_err());
    assert!(!temp.path().join("new-name").exists());
}

#[test]
fn link_install_rejects_replaced_multilink_and_symlink_sources() {
    let _force = super::io::force_link_install();
    for kind in ["replace", "multilink", "symlink"] {
        let p = Fixture::new(false).publish();
        let stage = p.out.join(".partial-stage-00000000");
        let saved = p.out.join("saved-stage");
        super::io::on_link_source_open(move || match kind {
            "replace" => {
                fs::rename(&stage, &saved).unwrap();
                fs::write(&stage, b"replacement").unwrap();
            }
            "multilink" => fs::hard_link(&stage, &saved).unwrap(),
            "symlink" => {
                fs::rename(&stage, &saved).unwrap();
                std::os::unix::fs::symlink("saved-stage", &stage).unwrap();
            }
            _ => unreachable!(),
        });
        let error = p.restore().unwrap_err();
        if kind != "symlink" {
            assert!(format!("{error:#}").contains("source identity changed"));
        }
        assert!(p.out.join(restore::MARKER).exists());
        assert!(p.out.join("saved-stage").exists());
        assert!(p
            .out
            .join(".partial-stage-00000000")
            .symlink_metadata()
            .is_ok());
        assert!(format!("{:#}", p.restore().unwrap_err()).contains("destination must be new"));
        p.retain(
            &format!("R14-link-source-{kind}"),
            "changed source inode or link topology rejects without cleanup or adoption",
            &json!({"error":format!("{error:#}")}),
        );
    }
}
#[test]
fn interrupted_link_install_keeps_both_names_and_incomplete_marker() {
    for point in ["link-after", "link-directory-sync", "link-unlink-before"] {
        let _force = super::io::force_link_install();
        let guard = super::io::inject_fault(point);
        let p = Fixture::new(false).publish();
        let error = p.restore().unwrap_err();
        drop(guard);
        assert!(format!("{error:#}").contains(point));
        assert!(p.out.join(restore::MARKER).exists());
        assert!(p.out.join(".partial-stage-00000000").exists());
        assert!(p.artifact_path("context/run.json").exists());
        assert!(format!("{:#}", p.restore().unwrap_err()).contains("destination must be new"));
        p.retain(
            &format!("R14-{point}"),
            "nonzero with both names and incomplete marker; no adoption",
            &json!({"error":format!("{error:#}")}),
        );
    }
}
