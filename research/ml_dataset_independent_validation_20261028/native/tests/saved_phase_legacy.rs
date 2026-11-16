//! Exact legacy-byte differential over already retained producer software cells.
use anyhow::{ensure, Result};
use ml_dataset_independent_consumer::common;
use serde_json::json;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn invoke(binary: &Path, args: &[&str], run: &Path, out: Option<&Path>) -> Result<Vec<u8>> {
    let mut c = Command::new(binary);
    c.args(args).arg("--run").arg(run).arg("--arithmetic");
    if let Some(path) = out {
        c.arg("--out").arg(path).arg("--allow-unreplayed-preview");
    }
    let output = c.output()?;
    ensure!(
        output.status.success(),
        "consumer {}: {}",
        binary.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(output.stdout)
}

#[test]
#[ignore = "requires retained fixed legacy panel and source-bound prior consumer; no producer execution"]
fn retained_legacy_audit_and_export_are_byte_identical() -> Result<()> {
    let panel = PathBuf::from(std::env::var("ML_PHASE_LEGACY_PANEL")?);
    let baseline = PathBuf::from(std::env::var("ML_PHASE_BASELINE")?);
    let output = PathBuf::from(std::env::var("ML_PHASE_LEGACY_OUT")?);
    ensure!(
        common::hash_file(&baseline)?
            == "31ac84b125fcdaa382df7650833d4295ec99a4aa87b9e49dcbd36e60ee4f0047",
        "baseline hash"
    );
    ensure!(!output.exists(), "fresh differential output required");
    fs::create_dir(&output)?;
    let candidate = Path::new(env!("CARGO_BIN_EXE_dataset-consumer"));
    let mut entries = fs::read_dir(panel)?
        .map(|e| e.map(|e| e.path()))
        .collect::<std::io::Result<Vec<_>>>()?;
    entries.sort();
    let mut checked = Vec::new();
    for entry in entries {
        let run = entry.join("run");
        if !run.join("run.json").is_file() {
            continue;
        }
        let binding = common::load(&run.join("run.json"))?;
        ensure!(
            binding["source_commit"] == "58bd5ab64424f7710540e0aea502225de6e6c438"
                && binding["binary_sha256"]
                    == "968a316daab4084ef61fd46c779832b9ebd9e482e58492674bc82174c4b1cfb7",
            "retained producer provenance"
        );
        let name = entry.file_name().unwrap().to_str().unwrap();
        let dest = output.join(name);
        fs::create_dir(&dest)?;
        let before = invoke(&baseline, &["audit"], &run, None)?;
        let after = invoke(candidate, &["audit"], &run, None)?;
        fs::write(dest.join("baseline-audit.json"), &before)?;
        fs::write(dest.join("candidate-audit.json"), &after)?;
        ensure!(before == after, "audit byte difference {name}");
        let old = dest.join("baseline.jsonl");
        let new = dest.join("candidate.jsonl");
        fs::write(
            dest.join("baseline-export-receipt.json"),
            invoke(&baseline, &["export"], &run, Some(&old))?,
        )?;
        fs::write(
            dest.join("candidate-export-receipt.json"),
            invoke(candidate, &["export"], &run, Some(&new))?,
        )?;
        let old_bytes = fs::read(&old)?;
        ensure!(
            old_bytes == fs::read(&new)?,
            "export byte difference {name}"
        );
        checked.push(json!({"name":name,"run_sha256":common::hash_file(&run.join("run.json"))?,"audit_sha256":common::digest(&before),"export_sha256":common::digest(&old_bytes),"byte_identical":true}));
    }
    ensure!(
        checked.len() == 40,
        "expected exactly40 retained cells, got{}",
        checked.len()
    );
    fs::write(
        output.join("summary.json"),
        serde_json::to_vec_pretty(
            &json!({"schema":"consumer-phase-legacy-differential/v1","baseline_source":"086b4cd590d2e871ca9e119c389470c9a9232f94","baseline_sha256":common::hash_file(&baseline)?,"candidate_sha256":common::hash_file(candidate)?,"cells":checked,"producer_invoked":false,"scientific_observation":false}),
        )?,
    )?;
    Ok(())
}
