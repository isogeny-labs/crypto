//! One-process, cumulative wall guard for the frozen K600 m6 arm sequence.
//! Build, staging, freeze and zero-trial preflight require a separate audited
//! outer precharge receipt; this runner never creates or trusts one by itself.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const ALLOWANCE_MS: u64 = 3_600_000;
const PHASES: [&str; 9] = [
    "pinned-export",
    "pinned-solve",
    "pinned-replay",
    "unpinned-export",
    "unpinned-solve",
    "unpinned-replay",
    "natural-export",
    "natural-solve",
    "natural-replay",
];
const PRECHARGE: [&str; 4] = ["build", "stage", "freeze", "preflight"];

fn sha256(path: &Path) -> Result<String> {
    let bytes = fs::read(path)?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

fn write_new(path: &Path, value: &Value) -> Result<()> {
    let mut file = File::options().write(true).create_new(true).open(path)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
}

fn field_path(value: &Value, field: &str) -> Result<PathBuf> {
    let path = PathBuf::from(value[field].as_str().ok_or(format!("{field} missing"))?);
    if !path.is_absolute() {
        return Err(format!("{field} must be absolute").into());
    }
    Ok(fs::canonicalize(path)?)
}

fn clean_source(repo: &Path) -> Result<String> {
    let head = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(repo)
        .output()?;
    let status = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(repo)
        .output()?;
    if !head.status.success() || !status.status.success() || !status.stdout.is_empty() {
        return Err("source checkout is not clean".into());
    }
    let commit = String::from_utf8(head.stdout)?.trim().to_string();
    if commit.len() != 40 || !commit.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err("invalid source commit".into());
    }
    Ok(commit)
}

fn precharge_ms(
    precharge: &Value,
    source: &str,
    arm_lock: &Path,
    preflight_run: &Path,
) -> Result<u64> {
    if precharge["schema"] != "n83.k600-m6-outer-precharge/v1"
        || precharge["status"] != "PASS_AUDITED_PRECHARGE"
        || precharge["source_commit"] != source
        || precharge["allowance_ms"] != ALLOWANCE_MS
    {
        return Err("precharge identity or allowance differs".into());
    }
    let phases = precharge["phases"]
        .as_array()
        .ok_or("precharge phases missing")?;
    if phases.len() != PRECHARGE.len() {
        return Err("precharge phase count differs".into());
    }
    let mut total = 0u64;
    for (entry, name) in phases.iter().zip(PRECHARGE) {
        let path = field_path(entry, "receipt_path")?;
        let wall = entry["outer_wall_ms"]
            .as_u64()
            .filter(|n| *n > 0)
            .ok_or("precharge outer wall missing")?;
        if entry["phase"] != name || entry["receipt_sha256"] != sha256(&path)? {
            return Err(format!("precharge {name} receipt differs").into());
        }
        if name == "freeze" && path != arm_lock {
            return Err("freeze precharge must name the arm lock".into());
        }
        if name == "preflight" && path != preflight_run.join("outer.json") {
            return Err("preflight precharge must name its outer receipt".into());
        }
        total = total.checked_add(wall).ok_or("precharge wall overflow")?;
    }
    if total != precharge["total_outer_wall_ms"].as_u64().unwrap_or(0) || total >= ALLOWANCE_MS {
        return Err("precharge total exceeds frozen allowance".into());
    }
    Ok(total)
}

fn phase_command(
    phase: &str,
    arm: &Path,
    lock: &Path,
    prelock: &Path,
    prerun: &Path,
    root: &Path,
    output: &Path,
    seconds: u64,
) -> Result<Vec<String>> {
    let p = |name: &str| root.join(name).to_string_lossy().into_owned();
    let mut args = vec![arm.to_string_lossy().into_owned()];
    let common = [lock, prelock, prerun];
    let mut push = |command: &str| {
        args.push(command.into());
        args.extend(
            common
                .iter()
                .map(|path| path.to_string_lossy().into_owned()),
        );
    };
    match phase {
        "pinned-export" => {
            push("export");
            args.push("positive-pinned".into());
        }
        "pinned-solve" => {
            push("solve");
            args.extend(["positive-pinned".into(), p("pinned-export")]);
        }
        "pinned-replay" => {
            push("replay-pinned");
            args.extend([p("pinned-export"), p("pinned-solve")]);
        }
        "unpinned-export" => {
            push("export");
            args.push("positive-unpinned".into());
        }
        "unpinned-solve" => {
            push("solve-unpinned");
            args.extend([
                p("unpinned-export"),
                p("pinned-export"),
                p("pinned-solve"),
                p("pinned-replay"),
            ]);
        }
        "unpinned-replay" => {
            push("replay-unpinned");
            args.extend([
                p("unpinned-export"),
                p("unpinned-solve"),
                p("pinned-export"),
                p("pinned-solve"),
                p("pinned-replay"),
            ]);
        }
        "natural-export" => {
            push("export-natural");
            args.extend([
                p("pinned-export"),
                p("pinned-solve"),
                p("pinned-replay"),
                p("unpinned-export"),
                p("unpinned-solve"),
                p("unpinned-replay"),
            ]);
        }
        "natural-solve" => {
            push("solve-natural");
            args.extend([
                p("natural-export"),
                p("pinned-export"),
                p("pinned-solve"),
                p("pinned-replay"),
                p("unpinned-export"),
                p("unpinned-solve"),
                p("unpinned-replay"),
            ]);
        }
        "natural-replay" => {
            push("replay-natural");
            args.extend([
                p("natural-export"),
                p("natural-solve"),
                p("pinned-export"),
                p("pinned-solve"),
                p("pinned-replay"),
                p("unpinned-export"),
                p("unpinned-solve"),
                p("unpinned-replay"),
            ]);
        }
        _ => return Err("unknown frozen arm phase".into()),
    }
    args.push(output.to_string_lossy().into_owned());
    args.push(seconds.to_string());
    Ok(args)
}

fn expected(phase: &str) -> (&'static str, &'static str) {
    match phase {
        "pinned-export" | "unpinned-export" | "natural-export" => {
            ("export-stage.json", "PASS_EXPORT_ONLY")
        }
        "pinned-solve" | "unpinned-solve" | "natural-solve" => {
            ("solver-receipt.json", "SAT_PENDING_XCNF_REPLAY")
        }
        _ => (
            "replay-stage.json",
            match phase {
                "pinned-replay" => "PASS_PINNED_MODEL_REPLAY",
                "unpinned-replay" => "PASS_UNPINNED_MODEL_REPLAY",
                _ => "PASS_NATURAL_MODEL_REPLAY",
            },
        ),
    }
}

fn receipt_identity(phase: &str, receipt: &Value, source: &str, lock_sha: &str) -> bool {
    let schema = match phase {
        "pinned-export" | "unpinned-export" | "natural-export" => {
            "n83.k600-m6-native-arm-export/v1"
        }
        "pinned-solve" | "unpinned-solve" | "natural-solve" => "n83.k600-m6-native-solver/v1",
        "pinned-replay" => "n83.k600-m6-native-pinned-replay-stage/v1",
        "unpinned-replay" => "n83.k600-m6-native-unpinned-replay-stage/v1",
        "natural-replay" => "n83.k600-m6-native-natural-replay-stage/v1",
        _ => return false,
    };
    let mode = if phase.starts_with("pinned-") {
        "positive-pinned"
    } else if phase.starts_with("unpinned-") {
        "positive-unpinned"
    } else {
        "natural"
    };
    receipt["schema"] == schema
        && receipt["mode"] == mode
        && receipt["source_commit"] == source
        && receipt["lock_sha256"] == lock_sha
}

fn bounded_output(mut command: Command) -> Result<Output> {
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let start = Instant::now();
    loop {
        if child.try_wait()?.is_some() {
            return Ok(child.wait_with_output()?);
        }
        if start.elapsed() >= Duration::from_secs(10) {
            let _ = child.kill();
            let _ = child.wait();
            return Err("Docker cleanup command exceeded ten seconds".into());
        }
        thread::sleep(Duration::from_millis(50));
    }
}

fn docker_cleanup(pid: u32) -> Result<()> {
    let filter = format!("name=^n83-k600-m6-.*-{pid}$");
    let mut list_command = Command::new("docker");
    list_command.args(["ps", "-aq", "--filter", &filter]);
    let list = bounded_output(list_command)?;
    if !list.status.success() {
        return Err("Docker cleanup inspection failed".into());
    }
    let ids = String::from_utf8(list.stdout)?;
    for id in ids.lines().filter(|s| !s.is_empty()) {
        let mut kill_command = Command::new("docker");
        kill_command.args(["kill", id]);
        let killed = bounded_output(kill_command)?;
        if !killed.status.success() {
            return Err("Docker child cleanup failed".into());
        }
    }
    let mut after_command = Command::new("docker");
    after_command.args(["ps", "-aq", "--filter", &filter]);
    let after = bounded_output(after_command)?;
    if !after.status.success() || !after.stdout.is_empty() {
        return Err("Docker child remains after cleanup".into());
    }
    Ok(())
}

fn run(plan_path: &Path, output: &Path) -> Result<Value> {
    let start = Instant::now();
    if !output.is_absolute() || output.exists() {
        return Err("new absolute output directory required".into());
    }
    let repo = fs::canonicalize(env!("CARGO_MANIFEST_DIR"))?;
    let parent = fs::canonicalize(output.parent().ok_or("output parent missing")?)?;
    if parent.starts_with(&repo) {
        return Err("budget output must be outside source".into());
    }
    let plan_path = fs::canonicalize(plan_path)?;
    let plan_sha = sha256(&plan_path)?;
    let plan: Value = serde_json::from_slice(&fs::read(&plan_path)?)?;
    let source = clean_source(&repo)?;
    let arm = field_path(&plan, "arm_binary")?;
    let lock = field_path(&plan, "arm_lock")?;
    let prelock = field_path(&plan, "preflight_lock")?;
    let prerun = field_path(&plan, "preflight_run")?;
    let precharge_path = field_path(&plan, "precharge_receipt")?;
    let lock_json: Value = serde_json::from_slice(&fs::read(&lock)?)?;
    let precharge: Value = serde_json::from_slice(&fs::read(&precharge_path)?)?;
    if plan["schema"] != "n83.k600-m6-arm-budget-plan/v1"
        || plan["source_commit"] != source
        || plan["allowance_ms"] != ALLOWANCE_MS
        || plan["precharge_sha256"] != sha256(&precharge_path)?
        || lock_json["source_commit"] != source
        || lock_json["supervisor_sha256"] != sha256(&arm)?
        || parent.starts_with(&prerun)
    {
        return Err("budget plan, source, binary or frozen input differs".into());
    }
    let precharged = precharge_ms(&precharge, &source, &lock, &prerun)?;
    fs::create_dir(output)?;
    let output = fs::canonicalize(output)?;
    write_new(
        &output.join("admission.json"),
        &json!({
            "schema":"n83.k600-m6-arm-budget-admission/v1","status":"PASS_PRECHARGE_BOUND",
            "source_commit":source,"plan_sha256":plan_sha,"precharge_sha256":sha256(&precharge_path)?,
            "arm_binary_sha256":sha256(&arm)?,"allowance_ms":ALLOWANCE_MS,
            "precharged_outer_wall_ms":precharged,"arm_stages_executed":0,
            "total_index_calculus_runtime_ms":Value::Null,"selected_best_total_runtime":Value::Null,
        }),
    )?;
    let mut terminal = "PASS_NATURAL_MODEL_REPLAY".to_string();
    let mut completed = 0usize;
    let arm_lock_sha = sha256(&lock)?;
    let stages: Result<()> = (|| {
        for phase in PHASES {
            let elapsed = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);
            let remaining = ALLOWANCE_MS
                .saturating_sub(precharged)
                .saturating_sub(elapsed);
            let minimum = if phase.ends_with("solve") {
                16_000
            } else {
                11_000
            };
            if remaining < minimum {
                terminal = "BUDGET_EXHAUSTED".into();
                break;
            }
            let seconds = (remaining - 10_000) / 1_000;
            let phase_output = output.join(phase);
            let args = phase_command(
                phase,
                &arm,
                &lock,
                &prelock,
                &prerun,
                &output,
                &phase_output,
                seconds,
            )?;
            let stdout = output.join(format!("{phase}.stdout"));
            let stderr = output.join(format!("{phase}.stderr"));
            let phase_start = Instant::now();
            let mut child = Command::new(&arm)
                .args(&args[1..])
                .stdout(Stdio::from(
                    File::options().write(true).create_new(true).open(&stdout)?,
                ))
                .stderr(Stdio::from(
                    File::options().write(true).create_new(true).open(&stderr)?,
                ))
                .spawn()?;
            let pid = child.id();
            let deadline = Duration::from_secs(seconds + 5);
            let (exit, capped) = loop {
                if let Some(status) = child.try_wait()? {
                    break (status.code(), false);
                }
                if phase_start.elapsed() >= deadline {
                    let _ = child.kill();
                    child.wait()?;
                    break (None, true);
                }
                thread::sleep(Duration::from_millis(100));
            };
            let cleanup = docker_cleanup(pid);
            let (receipt_name, want) = expected(phase);
            let receipt_path = phase_output.join(receipt_name);
            let receipt = fs::read(&receipt_path)
                .ok()
                .and_then(|bytes| serde_json::from_slice::<Value>(&bytes).ok());
            let got = receipt
                .as_ref()
                .and_then(|value| value["status"].as_str())
                .unwrap_or("MISSING_RECEIPT");
            let identity_ok = receipt
                .as_ref()
                .is_some_and(|value| receipt_identity(phase, value, &source, &arm_lock_sha));
            let phase_status = if cleanup.is_err() {
                "CLEANUP_FAILURE"
            } else if capped {
                "BUDGET_WALL_CAP"
            } else if exit == Some(0) && got == want && identity_ok {
                "PASS"
            } else if phase == "natural-solve"
                && identity_ok
                && [
                    "UNSAT_NATURAL_QUERY",
                    "UNKNOWN_solver_cap_or_exit0",
                    "UNKNOWN_solver_wall_cap",
                    "RESOURCE_EXIT",
                ]
                .contains(&got)
            {
                "CENSORED_NATURAL"
            } else {
                "STOPPED_STAGE_FAILURE"
            };
            write_new(
                &output.join(format!("{phase}.outer.json")),
                &json!({
                    "schema":"n83.k600-m6-arm-budget-stage/v1","phase":phase,"status":phase_status,
                    "child_exit_code":exit,"child_wall_capped":capped,"cleanup_ok":cleanup.is_ok(),
                    "receipt_status":got,"receipt_identity_ok":identity_ok,
                    "receipt_sha256":if receipt.is_some() {Some(sha256(&receipt_path)?)} else {None},
                    "stdout_sha256":sha256(&stdout)?,"stderr_sha256":sha256(&stderr)?,
                    "command":args,"charged_outer_wall_ms":phase_start.elapsed().as_millis(),
                    "remaining_before_ms":remaining,"outer_seconds":seconds,
                    "total_index_calculus_runtime_ms":Value::Null,"selected_best_total_runtime":Value::Null,
                }),
            )?;
            completed += 1;
            if phase_status != "PASS" {
                terminal = phase_status.into();
                break;
            }
            if clean_source(&repo)? != source
                || sha256(&arm)? != lock_json["supervisor_sha256"]
                || sha256(&plan_path)? != plan_sha
            {
                terminal = "SOURCE_OR_PLAN_CHANGED".into();
                break;
            }
        }
        Ok(())
    })();
    if let Err(error) = stages {
        terminal = "CONTROLLER_FAILURE".into();
        let _ = write_new(
            &output.join("failure.json"),
            &json!({
                "schema":"n83.k600-m6-arm-budget-failure/v1","status":terminal,
                "source_commit":source,"plan_sha256":plan_sha,
                "reason":error.to_string(),"completed_phases":completed,
                "elapsed_before_failure_receipt_ms":start.elapsed().as_millis(),
                "precharged_outer_wall_ms":precharged,"allowance_ms":ALLOWANCE_MS,
            }),
        );
    }
    let charged =
        precharged.saturating_add(u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX));
    if charged > ALLOWANCE_MS {
        terminal = "BUDGET_OVERRUN".into();
    }
    let result = json!({
        "schema":"n83.k600-m6-arm-budget-result/v1","status":terminal,
        "source_commit":source,"plan_sha256":plan_sha,"precharged_outer_wall_ms":precharged,
        "arm_wall_to_summary_start_ms":start.elapsed().as_millis(),
        "charged_total_to_summary_start_ms":charged,
        "summary_write_and_process_exit_charged":false,
        "allowance_ms":ALLOWANCE_MS,"completed_phases":completed,
        "all_phases_completed":completed==PHASES.len() && terminal=="PASS_NATURAL_MODEL_REPLAY",
        "total_index_calculus_runtime_ms":Value::Null,"selected_best_total_runtime":Value::Null,
    });
    write_new(&output.join("summary.json"), &result)?;
    if terminal == "CONTROLLER_FAILURE" || terminal == "BUDGET_OVERRUN" {
        return Err(
            format!("arm controller stopped with {terminal}; inspect output receipts").into(),
        );
    }
    Ok(result)
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 || args[1] != "run" {
        return Err("usage: run PLAN_JSON NEW_OUTPUT_DIR".into());
    }
    println!("{}", run(Path::new(&args[2]), Path::new(&args[3]))?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frozen_phase_order_and_arguments() {
        assert_eq!(PHASES.len(), 9);
        let args = phase_command(
            "natural-solve",
            Path::new("/arm"),
            Path::new("/lock"),
            Path::new("/prelock"),
            Path::new("/prerun"),
            Path::new("/run"),
            Path::new("/run/natural-solve"),
            95,
        )
        .unwrap();
        assert_eq!(args[1], "solve-natural");
        assert_eq!(args[5], "/run/natural-export");
        assert_eq!(args[6], "/run/pinned-export");
        assert_eq!(args.last().map(String::as_str), Some("95"));
    }
    #[test]
    fn precharge_rejects_missing_or_exhausted_phases() {
        let source = "a".repeat(40);
        let empty = json!({"schema":"n83.k600-m6-outer-precharge/v1",
            "status":"PASS_AUDITED_PRECHARGE","source_commit":source,
            "allowance_ms":ALLOWANCE_MS,"phases":[],"total_outer_wall_ms":0});
        assert!(precharge_ms(
            &empty,
            &source,
            Path::new("/missing-lock"),
            Path::new("/missing-preflight")
        )
        .is_err());
    }
    #[test]
    fn every_frozen_phase_has_a_command_and_receipt_identity() {
        let source = "a".repeat(40);
        for phase in PHASES {
            let args = phase_command(
                phase,
                Path::new("/arm"),
                Path::new("/lock"),
                Path::new("/prelock"),
                Path::new("/prerun"),
                Path::new("/run"),
                Path::new("/run/output"),
                95,
            )
            .unwrap();
            assert_eq!(args.first().map(String::as_str), Some("/arm"));
            assert_eq!(args.last().map(String::as_str), Some("95"));
            let schema = match phase {
                "pinned-export" | "unpinned-export" | "natural-export" => {
                    "n83.k600-m6-native-arm-export/v1"
                }
                "pinned-solve" | "unpinned-solve" | "natural-solve" => {
                    "n83.k600-m6-native-solver/v1"
                }
                "pinned-replay" => "n83.k600-m6-native-pinned-replay-stage/v1",
                "unpinned-replay" => "n83.k600-m6-native-unpinned-replay-stage/v1",
                _ => "n83.k600-m6-native-natural-replay-stage/v1",
            };
            let mode = if phase.starts_with("pinned-") {
                "positive-pinned"
            } else if phase.starts_with("unpinned-") {
                "positive-unpinned"
            } else {
                "natural"
            };
            let receipt = json!({"schema":schema,"mode":mode,
                "source_commit":source,"lock_sha256":"deadbeef"});
            assert!(receipt_identity(phase, &receipt, &source, "deadbeef"));
            assert!(!receipt_identity(phase, &receipt, &source, "wrong"));
        }
    }
}
