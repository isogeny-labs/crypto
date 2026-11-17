//! Retained mocked producer interoperability; never invokes the producer or PMU.
use anyhow::{ensure, Result};
use ml_dataset_independent_consumer::{audit, common};
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn copy_tree(from: &Path, to: &Path) -> Result<()> {
    fs::create_dir(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let kind = entry.file_type()?;
        ensure!(!kind.is_symlink(), "fixture symlink");
        if kind.is_dir() {
            copy_tree(&entry.path(), &to.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), to.join(entry.file_name()))?;
        }
    }
    Ok(())
}
fn replace(root: &Path, name: &str, value: &Value) -> Result<()> {
    let cell = root.join("cells/000000000000");
    let raw = serde_json::to_vec(value)?;
    fs::write(cell.join(name), &raw)?;
    let mut m = common::load(&cell.join("manifest.json"))?;
    let a = m["artifacts"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|a| a["path"] == name)
        .unwrap();
    a["bytes"] = json!(raw.len());
    a["sha256"] = json!(common::digest(&raw));
    fs::write(cell.join("manifest.json"), serde_json::to_vec(&m)?)?;
    Ok(())
}

#[test]
#[ignore = "requires retained final producer mocked fixtures; no producer execution"]
fn retained_checked_worker_and_registryless_prefix_interoperate() -> Result<()> {
    let packet = PathBuf::from(std::env::var("ML_CHECKED_PHASE_PACKET")?);
    let out = PathBuf::from(std::env::var("ML_CHECKED_PHASE_OUT")?);
    ensure!(
        common::hash_file(&packet.join("fixtures.sha256"))?
            == "df5d738c64f6b54ba4e696ac3cf0cc877d38a2fe6c605de09457eea02d1e651b",
        "retained fixture manifest identity"
    );
    ensure!(!out.exists(), "fresh output required");
    fs::create_dir(&out)?;
    let mut results = Vec::new();
    for (name, starts, ends, state) in [
        ("sealed-worker", 4, 4, "completed_recorded_prefix"),
        (
            "sealed-registryless-prefix",
            1,
            0,
            "interrupted_or_unfinished",
        ),
    ] {
        let root = packet.join("fixtures").join(name);
        let (report, rows) = audit::audit(&root, None, true)?;
        ensure!(rows.len() == 1, "one sealed cell");
        let summary = &rows[0]["checked_phase_contract"];
        ensure!(
            summary["supplied_starts"] == starts
                && summary["supplied_ends"] == ends
                && summary["prefix_state"] == state,
            "recorded prefix"
        );
        for key in [
            "independently_verified",
            "hardware_independently_verified",
            "platform_admitted",
            "training_cost_eligible",
            "complete_execution_verified",
            "clock_valid_for_comparison",
            "partial_snapshot_chain_verified",
        ] {
            ensure!(summary[key] == false, "promoted {key}");
        }
        ensure!(
            summary["whole_worker_instruction_count"].is_null(),
            "invented total"
        );
        let exported = out.join(format!("{name}.jsonl"));
        audit::export_rows(&rows, &exported)?;
        let export = common::load(&exported)?;
        ensure!(
            export["labels"]["checked_phase_contract"] == *summary,
            "summary export"
        );
        for m in rows[0]["phase_measurements"].as_array().unwrap() {
            let phase = m["phase"].as_str().unwrap();
            let start = common::load(
                &root.join(format!("cells/000000000000/telemetry/{phase}-start.json")),
            )?;
            ensure!(m["start"] == start, "raw start changed");
            if !m["producer_reported"].is_null() {
                let original =
                    common::load(&root.join(format!("cells/000000000000/telemetry/{phase}.json")))?;
                ensure!(
                    m["producer_reported"] == original && original["consumer_v2_admitted"] == false,
                    "raw producer end changed"
                );
            }
        }
        fs::write(
            out.join(format!("{name}-audit.json")),
            serde_json::to_vec_pretty(&report)?,
        )?;
        results.push(json!({"name":name,"starts":starts,"ends":ends,"prefix_state":state,"manifest_sha256":rows[0]["manifest_sha256"],"export_sha256":common::hash_file(&exported)?,"modeled_storage_reserved":summary["modeled_storage_reserved"]}));
    }
    let root = packet.join("fixtures/sealed-worker");
    let mut rejected = Vec::new();
    for case in [
        "start_binding",
        "start_identity",
        "end_identity",
        "native_id",
        "observation",
        "whole_worker",
        "hardware_claim",
        "operation_error",
    ] {
        let altered = out.join(format!("tamper-{case}"));
        copy_tree(&root, &altered)?;
        let name = if case.starts_with("start_") {
            "telemetry/curve-construction-start.json"
        } else {
            "telemetry/curve-construction.json"
        };
        let mut value = common::load(&altered.join("cells/000000000000").join(name))?;
        match case {
            "start_binding" => value["binding"]["attempt_sha256"] = json!("0".repeat(64)),
            "start_identity" => value["recording_pid"] = json!(0),
            "end_identity" => value["start_sha256"] = json!("0".repeat(64)),
            "native_id" => {
                value["collection"]["attempts"][0]["events"][0]["kernel_event_id"] = json!(999999)
            }
            "observation" => value["observations"][0]["raw"] = json!(999999),
            "whole_worker" => value["whole_worker_retired_instructions"] = json!(1),
            "hardware_claim" => {
                value["collection"]["hardware_independently_verified"] = json!(true)
            }
            _ => value["returned_ok"] = json!(false),
        }
        replace(&altered, name, &value)?;
        let error =
            audit::audit(&altered, None, false).expect_err("rehashed semantic tamper accepted");
        let message = format!("{error:#}");
        fs::write(out.join(format!("tamper-{case}.error")), &message)?;
        ensure!(
            message.contains("phase"),
            "unexpected rejection {case}: {message}"
        );
        rejected.push(json!({"case":case,"error":message,"manifest_sha256":common::hash_file(&altered.join("cells/000000000000/manifest.json"))?}));
    }
    fs::write(
        out.join("summary.json"),
        serde_json::to_vec_pretty(
            &json!({"schema":"consumer-retained-checked-phase-interop/v1","producing_test_source":"b7c11d1b5f8a1f37b769453d3ce0413696b9fc83","producing_test_binary_sha256":"1e7496533221060ea6202ff4fc78c80b0d5a11cb1bdabf1bfb68c430551eec8b","fixture_binding_placeholders":true,"counter_acquisition":"producer test mock","cells":results,"rehashed_tamper_rejections":rejected,"producer_invoked_by_consumer":false,"hardware_verified":false,"scientific_observation":false}),
        )?,
    )?;
    Ok(())
}
