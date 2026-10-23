//! Charge a verified pre-search admission refusal inside its original arm slot.
//! This accounting record grants no search admission and no new time allowance.
use super::{hash, json, read, require, write, R};
use serde_json::Value;
use std::{fs, path::Path};

fn remaining_budget(receipt: &Value) -> R<Value> {
    require(
        receipt["status"] == "stopped at command 1"
            && receipt["card_changed"] == false
            && receipt["dirty_before"] == ""
            && receipt["dirty_after"] == "",
        "a stopped immutable receipt is required",
    )?;
    let commands = receipt["commands"].as_array().ok_or("commands")?;
    require(commands.len() == 1, "one refused command required")?;
    let cmd = &commands[0];
    require(
        cmd["attempts"] == 1
            && cmd["exit_code"] == 1
            && cmd["timed_out"] == false
            && cmd["succeeded"] == false,
        "a single non-timeout refusal required",
    )?;
    let attempts = cmd["attempt_log"].as_array().ok_or("attempt log")?;
    require(
        attempts.len() == 1 && attempts[0]["exit_code"] == 1 && attempts[0]["timed_out"] == false,
        "attempt custody differs",
    )?;
    let seconds = cmd["seconds"].as_f64().ok_or("command seconds")?;
    require(
        seconds.is_finite()
            && seconds > 0.0
            && seconds < 9000.0
            && attempts[0]["seconds"].as_f64() == Some(seconds),
        "invalid refused duration",
    )?;
    let charged = seconds.ceil() as u64;
    require(charged < 9000, "no corrective time remains")?;
    Ok(json!({"schema":"n83.admission-refusal-budget/v1",
        "status":"PASS_CHARGE_REFUSAL_WITHIN_ORIGINAL_ARM",
        "original_arm_worker_allowance_seconds":9000,
        "recorded_command_seconds":seconds,"charged_worker_seconds":charged,
        "corrective_worker_ceiling_seconds":9000-charged,
        "charged_plus_corrective_seconds":9000,
        "worker_search_seconds":null,"new_allowance_seconds":0,
        "scope":"round the entire refused command upward; retain charge inside the same arm; no transfer, automatic retry or extension; separate verified card custody required"}))
}

pub fn account(dir: &Path, receipt_path: &Path, out: &Path) -> R<()> {
    let cfg = read(dir.join("config.json"))?;
    let outer = read(dir.join("outer.json"))?;
    let final_state = read(dir.join("inspect-final.json"))?;
    let receipt = read(receipt_path)?;
    require(
        cfg["columns"] == 2048
            && cfg["conflicts"] == 1500000
            && cfg["wall_seconds"] == 9000
            && cfg["source_commit"] == receipt["commit"],
        "specific frozen K2048 continuation refusal required",
    )?;
    require(
        outer["exit_code"] == 1
            && outer["cleanup_verified"] == true
            && outer["systemd_runtime_cleanup_verified"] == true
            && outer["wall_censored"] == false,
        "cleanup/exit custody differs",
    )?;
    require(
        final_state["state"]["ExitCode"] == 1
            && final_state["state"]["OOMKilled"] == false
            && final_state["state"]["Running"] == false,
        "worker termination differs",
    )?;
    require(fs::read_to_string(dir.join("stderr.log"))?.trim()
        == "Error: \"primary chained-S3 search caps or base selection outside supported domain\"",
        "not the known pre-search admission refusal")?;
    require(
        fs::read(dir.join("stdout.log"))?.is_empty()
            && fs::read(dir.join("memory-profile.jsonl"))?.is_empty()
            && !dir.join("run").exists(),
        "search/output evidence exists; do not respecify as admission refusal",
    )?;
    let mut budget = remaining_budget(&receipt)?;
    budget["dir"] = json!(dir);
    budget["receipt_blake3"] = json!(hash(receipt_path)?);
    budget["config_blake3"] = json!(hash(dir.join("config.json"))?);
    budget["outer_blake3"] = json!(hash(dir.join("outer.json"))?);
    budget["worker_stderr_blake3"] = json!(hash(dir.join("stderr.log"))?);
    budget["original_source_commit"] = cfg["source_commit"].clone();
    write(out, &budget)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn receipt() -> Value {
        json!({"status":"stopped at command 1","card_changed":false,
            "dirty_before":"","dirty_after":"","commands":[{
                "attempts":1,"exit_code":1,"timed_out":false,"succeeded":false,
                "seconds":12.87025458,"attempt_log":[{"exit_code":1,
                    "timed_out":false,"seconds":12.87025458}]}]})
    }
    #[test]
    fn charges_entire_command_upward_without_transferring_or_extending_allowance() {
        let budget = remaining_budget(&receipt()).unwrap();
        assert_eq!(budget["charged_worker_seconds"], 13);
        assert_eq!(budget["corrective_worker_ceiling_seconds"], 8987);
        assert_eq!(budget["charged_plus_corrective_seconds"], 9000);
        assert_eq!(budget["new_allowance_seconds"], 0);
        assert!(budget["worker_search_seconds"].is_null());
    }
    #[test]
    fn rejects_completed_retried_censored_missing_and_excessive_costs() {
        for (field, value) in [
            ("attempts", json!(2)),
            ("exit_code", json!(0)),
            ("timed_out", json!(true)),
            ("succeeded", json!(true)),
            ("seconds", json!(-1)),
            ("seconds", Value::Null),
            ("seconds", json!(9000)),
        ] {
            let mut bad = receipt();
            bad["commands"][0][field] = value;
            assert!(remaining_budget(&bad).is_err());
        }
        let mut bad = receipt();
        bad["commands"][0]["attempt_log"][0]["seconds"] = json!(1);
        assert!(remaining_budget(&bad).is_err());
        let mut bad = receipt();
        bad["status"] = json!("completed");
        assert!(remaining_budget(&bad).is_err());
        let mut bad = receipt();
        bad["card_changed"] = json!(true);
        assert!(remaining_budget(&bad).is_err());
    }
}
