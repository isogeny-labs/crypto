//! Optional local CLI replay using finite retained fixtures and immutable binaries.
//! No producer or remote service is invoked.
use anyhow::{ensure, Result};
use ml_dataset_independent_consumer::common;
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
fn binary() -> Result<PathBuf> {
    let p = PathBuf::from(std::env::var("ML_PARTIAL_BINARY")?);
    ensure!(
        common::hash_file(&p)? == std::env::var("ML_PARTIAL_BINARY_SHA256")?,
        "immutable candidate binary"
    );
    Ok(p)
}
fn run(binary: &Path, args: &[&str], out: &Path, label: &str) -> Result<Vec<u8>> {
    let result = Command::new(binary).args(args).output()?;
    fs::write(out.join(format!("{label}.stdout")), &result.stdout)?;
    fs::write(out.join(format!("{label}.stderr")), &result.stderr)?;
    fs::write(
        out.join(format!("{label}.exit")),
        format!("{:?}\n", result.status.code()),
    )?;
    fs::write(
        out.join(format!("{label}.argv.json")),
        serde_json::to_vec_pretty(&json!({"binary":binary,"argv":args}))?,
    )?;
    ensure!(
        result.status.success(),
        "{label}: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    Ok(result.stdout)
}
#[test]
#[ignore = "requires source-bound CLI and final finite fixture packet; no producer invocation"]
fn retained_partial_inputs_restore_and_fresh_audit_through_actual_cli() -> Result<()> {
    let binary = binary()?;
    let fixtures = PathBuf::from(std::env::var("ML_PARTIAL_FIXTURES")?);
    let out = PathBuf::from(std::env::var("ML_PARTIAL_CLI_OUT")?);
    fs::create_dir(&out)?;
    let mut records = Vec::new();
    for name in [
        "P01-retained-producer",
        "P03-P10-capture-payloads",
        "P04-phase-prefixes",
        "P05-operation-error",
        "P06-full-recorded-prefix",
        "P07-fallback-zero-id",
        "P08-unsupported-big-endian",
        "P13-large-integer",
    ] {
        let input = fixtures.join(name);
        let dest = out.join(name);
        fs::create_dir(&dest)?;
        let anchor = input.join("anchor.json");
        let mirror = input.join("objects");
        let restored = dest.join("restored");
        let pin = common::hash_file(&anchor)?;
        let raw = run(
            &binary,
            &[
                "partial-restore",
                "--anchor",
                anchor.to_str().unwrap(),
                "--expected-sha256",
                &pin,
                "--objects",
                mirror.to_str().unwrap(),
                "--out",
                restored.to_str().unwrap(),
            ],
            &dest,
            "restore",
        )?;
        let report: Value = serde_json::from_slice(&raw)?;
        ensure!(
            report["partial_snapshot_chain_verified"] == true
                && report["fresh_remote_readback_verified"] == false,
            "local claim"
        );
        for key in [
            "cell_complete",
            "solver_state_restorable",
            "solver_resumed",
            "scientific_result_verified",
            "training_cost_eligible",
            "worker_publication_verified",
        ] {
            ensure!(report[key] == false, "prohibited {key}");
        }
        ensure!(report["public_rows"] == 0, "no public rows");
        let audit = run(
            &binary,
            &[
                "partial-audit",
                "--restore",
                restored.to_str().unwrap(),
                "--expected-anchor-sha256",
                &pin,
            ],
            &dest,
            "audit",
        )?;
        let fresh: Value = serde_json::from_slice(&audit)?;
        ensure!(
            fresh["partial_snapshot_chain_verified"] == true && fresh["restored_locally"] == false,
            "fresh audit"
        );
        ensure!(
            fresh["binding"] == report["binding"] && fresh["cursor"] == report["cursor"],
            "independent fresh result"
        );
        records.push(json!({"fixture":name,"anchor_sha256":pin,"restore_sha256":common::digest(&raw),"audit_sha256":common::digest(&audit)}));
    }
    // Invalid/duplicate/unknown flags cannot become a generation or export path.
    for args in [
        vec!["partial-restore", "--anchor", "x", "--anchor", "y"],
        vec!["partial-audit", "--export"],
        vec!["partial-restore", "--run", "x"],
    ] {
        let result = Command::new(&binary).args(&args).output()?;
        ensure!(
            !result.status.success() && result.stdout.is_empty(),
            "invalid option accepted"
        );
    }
    fs::write(
        out.join("summary.json"),
        serde_json::to_vec_pretty(
            &json!({"schema":"consumer-partial-cli-replay/v1","binary_sha256":common::hash_file(&binary)?,"cases":records,"producer_invoked":false,"actual_remote_invoked":false}),
        )?,
    )?;
    Ok(())
}
#[test]
#[ignore = "requires retained40-cell legacy panel and two mocked checked cells plus pinned baseline"]
fn sealed_legacy_and_checked_audit_export_bytes_are_unchanged() -> Result<()> {
    let candidate = binary()?;
    let baseline = PathBuf::from(std::env::var("ML_PARTIAL_BASELINE")?);
    ensure!(
        common::hash_file(&baseline)?
            == "2512b52b8d4061febd068746b0e15fffb72cc5c1bfb8e4f54f45469236d8523c",
        "baseline source0ceec70 binary"
    );
    let panel = PathBuf::from(std::env::var("ML_PHASE_LEGACY_PANEL")?);
    let checked = PathBuf::from(std::env::var("ML_CHECKED_PHASE_PACKET")?).join("fixtures");
    let out = PathBuf::from(std::env::var("ML_PARTIAL_SEALED_OUT")?);
    fs::create_dir(&out)?;
    let mut inputs = fs::read_dir(&panel)?
        .map(|e| e.map(|e| e.path()))
        .collect::<std::io::Result<Vec<_>>>()?;
    inputs.sort();
    let mut roots: Vec<(String, PathBuf)> = inputs
        .into_iter()
        .filter_map(|p| {
            let r = p.join("run");
            r.join("run.json")
                .is_file()
                .then(|| (p.file_name().unwrap().to_str().unwrap().to_owned(), r))
        })
        .collect();
    ensure!(roots.len() == 40, "exact legacy panel");
    for n in ["sealed-worker", "sealed-registryless-prefix"] {
        roots.push((n.into(), checked.join(n)));
    }
    let mut rows = Vec::new();
    for (name, root) in roots {
        let dest = out.join(&name);
        fs::create_dir(&dest)?;
        let mut audit_bytes = Vec::new();
        let mut export_bytes = Vec::new();
        for (label, bin) in [("before", &baseline), ("after", &candidate)] {
            audit_bytes.push(run(
                bin,
                &["audit", "--run", root.to_str().unwrap(), "--arithmetic"],
                &dest,
                &format!("{label}-audit"),
            )?);
            let path = dest.join(format!("{label}.jsonl"));
            run(
                bin,
                &[
                    "export",
                    "--run",
                    root.to_str().unwrap(),
                    "--arithmetic",
                    "--out",
                    path.to_str().unwrap(),
                    "--allow-unreplayed-preview",
                ],
                &dest,
                &format!("{label}-export"),
            )?;
            export_bytes.push(fs::read(path)?);
        }
        ensure!(
            audit_bytes[0] == audit_bytes[1] && export_bytes[0] == export_bytes[1],
            "sealed byte regression {name}"
        );
        rows.push(json!({"fixture":name,"audit_sha256":common::digest(&audit_bytes[0]),"export_sha256":common::digest(&export_bytes[0]),"byte_identical":true}));
    }
    fs::write(
        out.join("summary.json"),
        serde_json::to_vec_pretty(
            &json!({"schema":"partial-extraction-sealed-byte-comparison/v1","baseline_sha256":common::hash_file(&baseline)?,"candidate_sha256":common::hash_file(&candidate)?,"cells":rows,"producer_invoked":false}),
        )?,
    )?;
    Ok(())
}
