//! One-process, cumulative wall guard for the frozen K600 m6 arm sequence.
//! Build, staging, freeze and zero-trial preflight require native outer child
//! interval receipts. The later full-invocation audit remains separate.
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
const MAX_VERSIONED_ALLOWANCE_MS: u64 = 86_400_000;
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
const CUMULATIVE_SETUP: [&str; 5] = ["build", "host-build", "stage", "freeze", "preflight"];

fn ceil_elapsed_ms(start: Instant) -> u64 {
    let nanos = start.elapsed().as_nanos();
    u64::try_from(nanos.saturating_add(999_999) / 1_000_000).unwrap_or(u64::MAX)
}

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
    build_source: &str,
    controller_sha: &str,
    arm_lock: &Path,
    preflight_run: &Path,
) -> Result<u64> {
    if precharge["schema"] != "n83.k600-m6-outer-precharge/v2"
        || precharge["status"] != "PASS_BOUND_CHILD_INTERVALS"
        || precharge["source_commit"] != source
        || precharge["build_source_commit"] != build_source
        || precharge["controller_sha256"] != controller_sha
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
    let arm: Value = serde_json::from_slice(&fs::read(arm_lock)?)?;
    if arm["schema"] != "n83.k600-m6-native-arms-lock/v1"
        || arm["source_commit"] != source
        || arm["build_source_commit"] != build_source
    {
        return Err("precharge arm lock identity differs".into());
    }
    let mut total = 0u64;
    for (entry, name) in phases.iter().zip(PRECHARGE) {
        let outer_path = field_path(entry, "outer_receipt_path")?;
        let child_path = field_path(entry, "child_receipt_path")?;
        let outer: Value = serde_json::from_slice(&fs::read(&outer_path)?)?;
        let child: Value = serde_json::from_slice(&fs::read(&child_path)?)?;
        let wall = entry["outer_wall_ms"]
            .as_u64()
            .filter(|n| *n > 0)
            .ok_or("precharge outer wall missing")?;
        let phase_source = if name == "build" {
            build_source
        } else {
            source
        };
        if entry["phase"] != name
            || entry["outer_receipt_sha256"] != sha256(&outer_path)?
            || entry["child_receipt_sha256"] != sha256(&child_path)?
            || outer["schema"] != "n83.k600-m6-native-child-interval/v1"
            || outer["status"] != "PASS_CHILD_INTERVAL"
            || outer["phase"] != name
            || outer["source_commit"] != phase_source
            || outer["controller_sha256"] != controller_sha
            || outer["allowance_ms"] != ALLOWANCE_MS
            || outer["child_exit_code"] != 0
            || outer["wall_capped"] != false
            || outer["cleanup_ok"] != true
            || outer["child_receipt_path"] != child_path.to_string_lossy().as_ref()
            || outer["child_receipt_sha256"] != entry["child_receipt_sha256"]
            || outer["outer_wall_to_receipt_start_ms"] != wall
        {
            return Err(format!("precharge {name} receipt differs").into());
        }
        if name == "freeze" && child_path != arm_lock {
            return Err("freeze precharge must name the arm lock".into());
        }
        if name == "preflight" && child_path != preflight_run.join("outer.json") {
            return Err("preflight precharge must name its outer receipt".into());
        }
        let child_ok = match name {
            "build" => {
                child["schema"] == "n83.k600-m6-linux-build/v1"
                    && child["status"] == "PASS_SOURCE_MATCHED_BUILD"
                    && child["source_commit"] == build_source
                    && child["image_id"] == arm["image_id"]
                    && child["exporter_replayer_sha256"] == arm["exporter_replayer_sha256"]
                    && child["solver_sha256"] == arm["solver_sha256"]
                    && child["supervisor_sha256"] == arm["supervisor_sha256"]
            }
            "stage" => {
                child["schema"] == "n83.k600-m6-native-arm-staging/v1"
                    && child["status"] == "PASS_READ_ONLY_STAGE"
                    && child["source_commit"] == source
                    && entry["child_receipt_sha256"] == arm["staging_receipt_sha256"]
            }
            "freeze" => {
                child["schema"] == "n83.k600-m6-native-arms-lock/v1"
                    && child["status"] == "PREPARED_ONLY"
                    && child["source_commit"] == source
                    && child["build_source_commit"] == build_source
            }
            "preflight" => {
                child["schema"] == "n83.k600-m6-native-outer/v1"
                    && child["status"] == "PREFLIGHT_ONLY"
                    && child["worker_exit_code"] == 0
                    && child["timed_out"] == false
                    && child["lock_sha256"] == arm["preflight_lock_sha256"]
            }
            _ => false,
        };
        if !child_ok {
            return Err(format!("precharge {name} child identity differs").into());
        }
        total = total.checked_add(wall).ok_or("precharge wall overflow")?;
    }
    if total != precharge["total_outer_wall_ms"].as_u64().unwrap_or(0) || total >= ALLOWANCE_MS {
        return Err("precharge total exceeds frozen allowance".into());
    }
    Ok(total)
}

// A later controller revision can audit the frozen C build, H2 arm binary and
// C preflight without pretending that their supervisor binaries are identical.
// This is a separate contract; the original v2 receipt remains unchanged.
fn cumulative_precharge_ms(
    ledger: &Value,
    controller_source: &str,
    controller_sha: &str,
    arm_lock: &Path,
    preflight_run: &Path,
    allowance_ms: u64,
) -> Result<(String, u64)> {
    let arm: Value = serde_json::from_slice(&fs::read(arm_lock)?)?;
    let arm_source = arm["source_commit"].as_str().ok_or("arm source missing")?;
    let build_source = arm["build_source_commit"]
        .as_str()
        .ok_or("build source missing")?;
    if ledger["schema"] != "n83.k600-m6-cumulative-precharge/v3"
        || ledger["status"] != "PASS_BOUND_SETUP_LEDGER"
        || ledger["controller_source_commit"] != controller_source
        || ledger["controller_sha256"] != controller_sha
        || ledger["arm_source_commit"] != arm_source
        || ledger["build_source_commit"] != build_source
        || ledger["allowance_ms"] != allowance_ms
        || arm["schema"] != "n83.k600-m6-native-arms-lock/v1"
        || arm["status"] != "PREPARED_ONLY"
    {
        return Err("cumulative ledger identity differs".into());
    }
    let artifacts = &ledger["artifacts"];
    let mut receipts = Vec::new();
    for name in CUMULATIVE_SETUP {
        let entry = &artifacts[name];
        let path = field_path(entry, "path")?;
        if entry["sha256"] != sha256(&path)? || receipts.contains(&path) {
            return Err(format!("{name} artifact digest or path differs").into());
        }
        receipts.push(path);
    }
    let build: Value = serde_json::from_slice(&fs::read(&receipts[0])?)?;
    let host_build: Value = serde_json::from_slice(&fs::read(&receipts[1])?)?;
    let stage: Value = serde_json::from_slice(&fs::read(&receipts[2])?)?;
    let preflight: Value = serde_json::from_slice(&fs::read(&receipts[4])?)?;
    if receipts[3] != fs::canonicalize(arm_lock)?
        || receipts[4] != fs::canonicalize(preflight_run.join("outer.json"))?
        || build["schema"] != "n83.k600-m6-linux-build/v1"
        || build["status"] != "PASS_SOURCE_MATCHED_BUILD"
        || build["source_commit"] != build_source
        || build["image_id"] != arm["image_id"]
        || build["exporter_replayer_sha256"] != arm["exporter_replayer_sha256"]
        || build["solver_sha256"] != arm["solver_sha256"]
        || !build["supervisor_sha256"]
            .as_str()
            .is_some_and(|s| s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit()))
        || host_build["schema"] != "n83.k600-m6-host-build/v1"
        || host_build["status"] != "PASS_SOURCE_MATCHED_BUILD"
        || host_build["source_commit"] != arm_source
        || host_build["supervisor_sha256"] != arm["supervisor_sha256"]
        || stage["schema"] != "n83.k600-m6-native-arm-staging/v1"
        || stage["status"] != "PASS_READ_ONLY_STAGE"
        || stage["source_commit"] != arm_source
        || stage["fresh_s3_get"] != true
        || stage["uploaded"] != false
        || stage["solver_executed"] != false
        || artifacts["stage"]["sha256"] != arm["staging_receipt_sha256"]
        || preflight["schema"] != "n83.k600-m6-native-outer/v1"
        || preflight["status"] != "PREFLIGHT_ONLY"
        || preflight["worker_exit_code"] != 0
        || preflight["timed_out"] != false
        || preflight["lock_sha256"] != arm["preflight_lock_sha256"]
    {
        return Err("cumulative ledger artifact identity differs".into());
    }
    let intervals = ledger["intervals"]
        .as_array()
        .ok_or("cumulative intervals missing")?;
    let mut seen = [false; 5];
    let mut paths = Vec::new();
    let mut total = 0u64;
    for entry in intervals {
        let phase = entry["phase"].as_str().ok_or("interval phase missing")?;
        let slot = CUMULATIVE_SETUP.iter().position(|name| *name == phase);
        if slot.is_none() && phase != "retry" {
            return Err("unknown cumulative interval phase".into());
        }
        if let Some(index) = slot {
            if seen[index] {
                return Err("duplicate required cumulative interval".into());
            }
            seen[index] = true;
        }
        let path = field_path(entry, "path")?;
        if entry["sha256"] != sha256(&path)? || paths.contains(&path) {
            return Err("cumulative interval digest or path differs".into());
        }
        paths.push(path.clone());
        let charged = entry["charged_wall_ms"]
            .as_u64()
            .filter(|n| *n > 0)
            .ok_or("charged interval missing")?;
        if (phase == "build" && path != receipts[0])
            || (phase == "preflight" && path != receipts[4])
        {
            return Err("required interval does not name its frozen artifact".into());
        }
        if phase == "host-build" || phase == "stage" || phase == "freeze" {
            let outer: Value = serde_json::from_slice(&fs::read(&path)?)?;
            let child = match phase {
                "host-build" => &receipts[1],
                "stage" => &receipts[2],
                _ => &receipts[3],
            };
            if outer["schema"] != "n83.k600-m6-native-child-interval/v1"
                || outer["status"] != "PASS_CHILD_INTERVAL"
                || outer["phase"] != phase
                || outer["source_commit"] != controller_source
                || outer["controller_sha256"] != controller_sha
                || outer["child_receipt_path"] != child.to_string_lossy().as_ref()
                || outer["child_receipt_sha256"] != sha256(child)?
                || outer["child_exit_code"] != 0
                || outer["wall_capped"] != false
                || outer["cleanup_ok"] != true
                || entry["meter"] != "json-wall-ms"
            {
                return Err(format!("{phase} outer interval is not bound to artifact").into());
            }
        }
        let observed = match entry["meter"].as_str() {
            Some("json-wall-ms") => {
                let receipt: Value = serde_json::from_slice(&fs::read(&path)?)?;
                receipt["wall_to_receipt_start_ms"]
                    .as_u64()
                    .or_else(|| receipt["outer_wall_to_receipt_start_ms"].as_u64())
                    .filter(|n| *n > 0)
                    .ok_or("metered JSON wall missing")?
            }
            Some("preflight-elapsed-ms") if phase == "preflight" && path == receipts[4] => {
                let value = preflight["elapsed_wall_ms"]
                    .as_f64()
                    .filter(|n| n.is_finite() && *n > 0.0)
                    .ok_or("preflight elapsed wall missing")?;
                value.ceil() as u64
            }
            Some("integer-seconds") => fs::read_to_string(&path)?
                .trim()
                .parse::<u64>()?
                .checked_mul(1_000)
                .ok_or("integer-seconds overflow")?,
            _ => return Err("unknown or mismatched cumulative meter".into()),
        };
        if charged < observed {
            return Err("cumulative interval undercharges observed wall".into());
        }
        total = total
            .checked_add(charged)
            .ok_or("cumulative wall overflow")?;
    }
    if seen != [true; 5]
        || total != ledger["total_charged_wall_ms"].as_u64().unwrap_or(0)
        || total >= allowance_ms
    {
        return Err("cumulative setup ledger incomplete or exhausted".into());
    }
    Ok((arm_source.to_owned(), total))
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

fn preflight_containers() -> Result<Vec<String>> {
    let filter = "name=^n83-k600-m6-native-preflight$";
    let mut list = Command::new("docker");
    list.args(["ps", "-aq", "--filter", filter]);
    let ids = bounded_output(list)?;
    if !ids.status.success() {
        return Err("preflight Docker cleanup inspection failed".into());
    }
    Ok(String::from_utf8(ids.stdout)?
        .lines()
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect())
}

fn preflight_cleanup() -> Result<()> {
    for id in preflight_containers()? {
        let mut kill = Command::new("docker");
        kill.args(["kill", &id]);
        if !bounded_output(kill)?.status.success() {
            return Err("preflight Docker cleanup kill failed".into());
        }
    }
    if !preflight_containers()?.is_empty() {
        return Err("preflight Docker child remains".into());
    }
    Ok(())
}

fn charge(
    repo: &Path,
    phase: &str,
    child_receipt: &Path,
    output: &Path,
    seconds: u64,
    program: &Path,
    args: &[String],
) -> Result<Value> {
    if !PRECHARGE.contains(&phase) && phase != "host-build"
        || seconds == 0
        || seconds > ALLOWANCE_MS / 1_000
        || !output.is_absolute()
        || output.exists()
        || !child_receipt.is_absolute()
        || child_receipt.exists()
    {
        return Err(
            "new absolute precharge output, child receipt and bounded phase required".into(),
        );
    }
    let repo = fs::canonicalize(repo)?;
    let parent = fs::canonicalize(output.parent().ok_or("output parent missing")?)?;
    if parent.starts_with(&repo) {
        return Err("precharge output must be outside source".into());
    }
    let source = clean_source(&repo)?;
    if phase == "preflight" && !preflight_containers()?.is_empty() {
        return Err("another fixed-name K600 preflight container exists".into());
    }
    let program = fs::canonicalize(program)?;
    let program_sha = sha256(&program)?;
    let controller_sha = sha256(&std::env::current_exe()?)?;
    fs::create_dir(output)?;
    let output = fs::canonicalize(output)?;
    let stdout = output.join("child.stdout");
    let stderr = output.join("child.stderr");
    let started = Instant::now();
    let result: Result<Value> = (|| {
        let mut child = Command::new(&program)
            .args(args)
            .current_dir(&repo)
            .stdout(Stdio::from(
                File::options().write(true).create_new(true).open(&stdout)?,
            ))
            .stderr(Stdio::from(
                File::options().write(true).create_new(true).open(&stderr)?,
            ))
            .spawn()?;
        let deadline = Duration::from_secs(seconds);
        let (exit, capped) = loop {
            match child.try_wait() {
                Ok(Some(status)) => break (status.code(), false),
                Ok(None) if started.elapsed() < deadline => {
                    thread::sleep(Duration::from_millis(100))
                }
                Ok(None) => {
                    let _ = child.kill();
                    child.wait()?;
                    break (None, true);
                }
                Err(error) => {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(error.into());
                }
            }
        };
        let cleanup = if phase == "preflight" {
            preflight_cleanup()
        } else {
            Ok(())
        };
        let receipt = fs::canonicalize(child_receipt).ok();
        let receipt_sha = receipt.as_ref().map(|path| sha256(path)).transpose()?;
        let source_after = clean_source(&repo);
        let status = if exit == Some(0)
            && !capped
            && cleanup.is_ok()
            && receipt_sha.is_some()
            && source_after.ok().as_deref() == Some(source.as_str())
            && sha256(&program)? == program_sha
        {
            "PASS_CHILD_INTERVAL"
        } else {
            "STOPPED_CHILD_INTERVAL"
        };
        let outer = json!({
            "schema":"n83.k600-m6-native-child-interval/v1","status":status,
            "phase":phase,"source_commit":source,"controller_sha256":controller_sha,
            "allowance_ms":ALLOWANCE_MS,"child_exit_code":exit,"wall_capped":capped,
            "cleanup_ok":cleanup.is_ok(),"cleanup_error":cleanup.err().map(|e|e.to_string()),
            "child_receipt_path":receipt.as_ref().map(|p|p.to_string_lossy().into_owned()),
            "child_receipt_sha256":receipt_sha,
            "program":program,"program_sha256":program_sha,"arguments":args,
            "stdout_sha256":sha256(&stdout)?,"stderr_sha256":sha256(&stderr)?,
            "outer_wall_to_receipt_start_ms":ceil_elapsed_ms(started),
            "receipt_write_and_process_exit_charged":false,
            "total_index_calculus_runtime_ms":Value::Null,
        });
        write_new(&output.join("outer.json"), &outer)?;
        Ok(outer)
    })();
    if let Err(error) = &result {
        let _ = write_new(
            &output.join("failure.json"),
            &json!({
                "schema":"n83.k600-m6-native-child-interval-failure/v1",
                "phase":phase,"status":"CONTROLLER_FAILURE","reason":error.to_string(),
                "elapsed_before_failure_receipt_ms":started.elapsed().as_millis(),
            }),
        );
    }
    result
}

fn assemble(
    outer_paths: &[PathBuf; 4],
    arm_lock: &Path,
    preflight_run: &Path,
    output: &Path,
) -> Result<Value> {
    if !output.is_absolute() || output.exists() {
        return Err("new absolute precharge manifest path required".into());
    }
    let repo = fs::canonicalize(env!("CARGO_MANIFEST_DIR"))?;
    let parent = fs::canonicalize(output.parent().ok_or("manifest parent missing")?)?;
    if parent.starts_with(&repo) {
        return Err("precharge manifest must be outside source".into());
    }
    let source = clean_source(&repo)?;
    let arm_lock = fs::canonicalize(arm_lock)?;
    let preflight_run = fs::canonicalize(preflight_run)?;
    let arm: Value = serde_json::from_slice(&fs::read(&arm_lock)?)?;
    let build_source = arm["build_source_commit"]
        .as_str()
        .ok_or("build source missing")?;
    let controller_sha = sha256(&std::env::current_exe()?)?;
    let mut phases = Vec::new();
    let mut total = 0u64;
    for (name, path) in PRECHARGE.into_iter().zip(outer_paths) {
        let path = fs::canonicalize(path)?;
        let outer: Value = serde_json::from_slice(&fs::read(&path)?)?;
        let child = field_path(&outer, "child_receipt_path")?;
        let wall = outer["outer_wall_to_receipt_start_ms"]
            .as_u64()
            .filter(|n| *n > 0)
            .ok_or("outer wall missing")?;
        total = total.checked_add(wall).ok_or("precharge total overflow")?;
        phases.push(json!({"phase":name,"outer_receipt_path":path,
            "outer_receipt_sha256":sha256(&path)?,"child_receipt_path":child,
            "child_receipt_sha256":sha256(&child)?,"outer_wall_ms":wall}));
    }
    let value = json!({"schema":"n83.k600-m6-outer-precharge/v2",
        "status":"PASS_BOUND_CHILD_INTERVALS","source_commit":source,
        "build_source_commit":build_source,"controller_sha256":controller_sha,
        "allowance_ms":ALLOWANCE_MS,"phases":phases,"total_outer_wall_ms":total,
        "receipt_write_and_process_exit_charged":false,
        "total_index_calculus_runtime_ms":Value::Null,
    });
    precharge_ms(
        &value,
        &source,
        build_source,
        &controller_sha,
        &arm_lock,
        &preflight_run,
    )?;
    write_new(output, &value)?;
    Ok(value)
}

fn assemble_cumulative(
    request_path: &Path,
    arm_lock: &Path,
    preflight_run: &Path,
    output: &Path,
) -> Result<Value> {
    if !output.is_absolute() || output.exists() {
        return Err("new absolute cumulative ledger path required".into());
    }
    let repo = fs::canonicalize(env!("CARGO_MANIFEST_DIR"))?;
    let parent = fs::canonicalize(output.parent().ok_or("ledger parent missing")?)?;
    if parent.starts_with(&repo) {
        return Err("cumulative ledger must be outside source".into());
    }
    let request_bytes = fs::read(request_path)?;
    let request_sha = format!("{:x}", Sha256::digest(&request_bytes));
    let request: Value = serde_json::from_slice(&request_bytes)?;
    let source = clean_source(&repo)?;
    let arm_lock = fs::canonicalize(arm_lock)?;
    let preflight_run = fs::canonicalize(preflight_run)?;
    let arm: Value = serde_json::from_slice(&fs::read(&arm_lock)?)?;
    let allowance_ms = request["allowance_ms"]
        .as_u64()
        .filter(|n| *n > 0 && *n <= MAX_VERSIONED_ALLOWANCE_MS)
        .ok_or("cumulative allowance missing")?;
    if request["schema"] != "n83.k600-m6-cumulative-request/v3"
        || request["controller_source_commit"] != source
        || request["arm_source_commit"] != arm["source_commit"]
        || request["build_source_commit"] != arm["build_source_commit"]
    {
        return Err("cumulative request source identity differs".into());
    }
    let mut artifacts = serde_json::Map::new();
    for name in CUMULATIVE_SETUP {
        let path = field_path(&request["artifacts"][name], "path")?;
        artifacts.insert(
            name.to_owned(),
            json!({"path":path,"sha256":sha256(&path)?}),
        );
    }
    let entries = request["intervals"]
        .as_array()
        .ok_or("cumulative request intervals missing")?;
    let mut intervals = Vec::new();
    let mut total = 0u64;
    for entry in entries {
        let path = field_path(entry, "path")?;
        let phase = entry["phase"].as_str().ok_or("interval phase missing")?;
        let meter = entry["meter"].as_str().ok_or("interval meter missing")?;
        let wall = entry["charged_wall_ms"]
            .as_u64()
            .filter(|n| *n > 0)
            .ok_or("charged wall missing")?;
        total = total.checked_add(wall).ok_or("cumulative wall overflow")?;
        intervals.push(json!({"phase":phase,"path":path,"sha256":sha256(&path)?,
            "meter":meter,"charged_wall_ms":wall}));
    }
    let controller_sha = sha256(&std::env::current_exe()?)?;
    let ledger = json!({"schema":"n83.k600-m6-cumulative-precharge/v3",
        "status":"PASS_BOUND_SETUP_LEDGER", "controller_source_commit":source,
        "controller_sha256":controller_sha,"arm_source_commit":arm["source_commit"],
        "build_source_commit":arm["build_source_commit"],"allowance_ms":allowance_ms,
        "artifacts":artifacts,"intervals":intervals,"total_charged_wall_ms":total,
        "request_sha256":request_sha,
        "total_index_calculus_runtime_ms":Value::Null});
    cumulative_precharge_ms(
        &ledger,
        &source,
        &controller_sha,
        &arm_lock,
        &preflight_run,
        allowance_ms,
    )?;
    if clean_source(&repo)? != source || sha256(request_path)? != request_sha {
        return Err("source or cumulative request changed during assembly".into());
    }
    write_new(output, &ledger)?;
    Ok(ledger)
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
    let legacy = plan["schema"] == "n83.k600-m6-arm-budget-plan/v1";
    let versioned = plan["schema"] == "n83.k600-m6-arm-budget-plan/v2";
    let allowance_ms = if legacy {
        ALLOWANCE_MS
    } else if versioned {
        plan["allowance_ms"]
            .as_u64()
            .filter(|n| *n > 0 && *n <= MAX_VERSIONED_ALLOWANCE_MS)
            .ok_or("new allowance missing")?
    } else {
        return Err("unknown budget plan schema".into());
    };
    if plan["precharge_sha256"] != sha256(&precharge_path)?
        || plan["allowance_ms"] != allowance_ms
        || lock_json["supervisor_sha256"] != sha256(&arm)?
        || parent.starts_with(&prerun)
    {
        return Err("budget plan, binary or frozen input differs".into());
    }
    let controller_sha = sha256(&std::env::current_exe()?)?;
    let build_source = lock_json["build_source_commit"]
        .as_str()
        .ok_or("build source commit missing")?;
    let (arm_source, precharged) = if legacy {
        if plan["source_commit"] != source || lock_json["source_commit"] != source {
            return Err("legacy budget source differs".into());
        }
        (
            source.clone(),
            precharge_ms(
                &precharge,
                &source,
                build_source,
                &controller_sha,
                &lock,
                &prerun,
            )?,
        )
    } else {
        if plan["controller_source_commit"] != source
            || plan["arm_source_commit"] != lock_json["source_commit"]
            || plan["build_source_commit"] != build_source
        {
            return Err("versioned budget source differs".into());
        }
        cumulative_precharge_ms(
            &precharge,
            &source,
            &controller_sha,
            &lock,
            &prerun,
            allowance_ms,
        )?
    };
    fs::create_dir(output)?;
    let output = fs::canonicalize(output)?;
    write_new(
        &output.join("admission.json"),
        &json!({
            "schema":if legacy {"n83.k600-m6-arm-budget-admission/v1"} else {"n83.k600-m6-arm-budget-admission/v2"},
            "status":"PASS_PRECHARGE_BOUND",
            "source_commit":arm_source,"controller_source_commit":source,
            "plan_sha256":plan_sha,"precharge_sha256":sha256(&precharge_path)?,
            "arm_binary_sha256":sha256(&arm)?,"allowance_ms":allowance_ms,
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
            let remaining = allowance_ms
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
                .is_some_and(|value| receipt_identity(phase, value, &arm_source, &arm_lock_sha));
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
                    "schema":if legacy {"n83.k600-m6-arm-budget-stage/v1"} else {"n83.k600-m6-arm-budget-stage/v2"},
                    "phase":phase,"status":phase_status,
                    "source_commit":arm_source,"controller_source_commit":source,
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
                "schema":if legacy {"n83.k600-m6-arm-budget-failure/v1"} else {"n83.k600-m6-arm-budget-failure/v2"},
                "status":terminal,
                "source_commit":arm_source,"controller_source_commit":source,"plan_sha256":plan_sha,
                "reason":error.to_string(),"completed_phases":completed,
                "elapsed_before_failure_receipt_ms":start.elapsed().as_millis(),
                "precharged_outer_wall_ms":precharged,"allowance_ms":allowance_ms,
            }),
        );
    }
    let charged =
        precharged.saturating_add(u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX));
    if charged > allowance_ms {
        terminal = "BUDGET_OVERRUN".into();
    }
    let result = json!({
        "schema":if legacy {"n83.k600-m6-arm-budget-result/v1"} else {"n83.k600-m6-arm-budget-result/v2"},
        "status":terminal,"source_commit":arm_source,"controller_source_commit":source,
        "plan_sha256":plan_sha,"precharged_outer_wall_ms":precharged,
        "arm_wall_to_summary_start_ms":start.elapsed().as_millis(),
        "charged_total_to_summary_start_ms":charged,
        "summary_write_and_process_exit_charged":false,
        "allowance_ms":allowance_ms,"completed_phases":completed,
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
    if args.len() >= 7 && args[1] == "charge" {
        let seconds: u64 = args[5].parse()?;
        let result = charge(
            &fs::canonicalize(env!("CARGO_MANIFEST_DIR"))?,
            &args[2],
            Path::new(&args[3]),
            Path::new(&args[4]),
            seconds,
            Path::new(&args[6]),
            &args[7..],
        )?;
        println!("{result}");
        if result["status"] != "PASS_CHILD_INTERVAL" {
            return Err("precharge child failed; inspect outer receipt".into());
        }
        return Ok(());
    }
    if args.len() == 9 && args[1] == "assemble" {
        let paths = [
            PathBuf::from(&args[2]),
            PathBuf::from(&args[3]),
            PathBuf::from(&args[4]),
            PathBuf::from(&args[5]),
        ];
        let result = assemble(
            &paths,
            Path::new(&args[6]),
            Path::new(&args[7]),
            Path::new(&args[8]),
        )?;
        println!("{result}");
        return Ok(());
    }
    if args.len() == 6 && args[1] == "assemble-v3" {
        let result = assemble_cumulative(
            Path::new(&args[2]),
            Path::new(&args[3]),
            Path::new(&args[4]),
            Path::new(&args[5]),
        )?;
        println!("{result}");
        return Ok(());
    }
    if args.len() != 4 || args[1] != "run" {
        return Err("usage: charge build|host-build|stage|freeze|preflight NEW_CHILD_RECEIPT NEW_OUTPUT_DIR OUTER_SECONDS PROGRAM [ARG...] | assemble BUILD_OUTER STAGE_OUTER FREEZE_OUTER PREFLIGHT_OUTER ARM_LOCK PREFLIGHT_RUN NEW_PRECHARGE_JSON | assemble-v3 REQUEST_JSON ARM_LOCK PREFLIGHT_RUN NEW_LEDGER_JSON | run PLAN_JSON NEW_OUTPUT_DIR".into());
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
        let empty = json!({"schema":"n83.k600-m6-outer-precharge/v2",
            "status":"PASS_BOUND_CHILD_INTERVALS","source_commit":source,
            "build_source_commit":"b".repeat(40),"controller_sha256":"digest",
            "allowance_ms":ALLOWANCE_MS,"phases":[],"total_outer_wall_ms":0});
        assert!(precharge_ms(
            &empty,
            &source,
            &"b".repeat(40),
            "digest",
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
    #[test]
    fn native_charge_measures_child_through_receipt_creation() {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root =
            std::env::temp_dir().join(format!("n83-precharge-{}-{nonce}", std::process::id()));
        let repo = root.join("repo");
        fs::create_dir_all(&repo).unwrap();
        let git = |args: &[&str]| {
            let out = Command::new("git")
                .args(args)
                .current_dir(&repo)
                .output()
                .unwrap();
            assert!(
                out.status.success(),
                "{}",
                String::from_utf8_lossy(&out.stderr)
            );
        };
        git(&["init", "-q"]);
        git(&["config", "user.name", "N83 Test"]);
        git(&["config", "user.email", "n83@example.invalid"]);
        fs::write(repo.join("source.txt"), b"source").unwrap();
        git(&["add", "source.txt"]);
        git(&["commit", "-qm", "source"]);
        let receipt = root.join("child.json");
        let args = vec![
            "-c".into(),
            "printf '{\"status\":\"PASS\"}\\n' > \"$1\"".into(),
            "sh".into(),
            receipt.to_string_lossy().into_owned(),
        ];
        let outer = charge(
            &repo,
            "build",
            &receipt,
            &root.join("outer"),
            5,
            Path::new("/bin/sh"),
            &args,
        )
        .unwrap();
        assert_eq!(outer["status"], "PASS_CHILD_INTERVAL");
        assert_eq!(outer["child_receipt_sha256"], sha256(&receipt).unwrap());
        assert!(outer["outer_wall_to_receipt_start_ms"].as_u64().unwrap() >= 1);
        let missing = root.join("missing.json");
        let failed = charge(
            &repo,
            "build",
            &missing,
            &root.join("missing-outer"),
            5,
            Path::new("/bin/sh"),
            &["-c".into(), ":".into()],
        )
        .unwrap();
        assert_eq!(failed["status"], "STOPPED_CHILD_INTERVAL");
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn precharge_binds_four_native_outer_and_child_receipts() {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "n83-precharge-chain-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir(&root).unwrap();
        let source = "a".repeat(40);
        let build_source = "b".repeat(40);
        let controller = "controller-digest";
        let image = format!("sha256:{}", "c".repeat(64));
        let stage_path = root.join("stage/staging.json");
        fs::create_dir_all(stage_path.parent().unwrap()).unwrap();
        write_new(
            &stage_path,
            &json!({"schema":"n83.k600-m6-native-arm-staging/v1",
            "status":"PASS_READ_ONLY_STAGE","source_commit":source}),
        )
        .unwrap();
        let arm_path = root.join("arm-lock.json");
        let arm = json!({"schema":"n83.k600-m6-native-arms-lock/v1",
            "status":"PREPARED_ONLY","source_commit":source,"build_source_commit":build_source,
            "image_id":image,"exporter_replayer_sha256":"exporter","solver_sha256":"solver",
            "supervisor_sha256":"supervisor","staging_receipt_sha256":sha256(&stage_path).unwrap(),
            "preflight_lock_sha256":"preflight-lock"});
        write_new(&arm_path, &arm).unwrap();
        let build_path = root.join("build/build.json");
        fs::create_dir_all(build_path.parent().unwrap()).unwrap();
        write_new(
            &build_path,
            &json!({"schema":"n83.k600-m6-linux-build/v1",
            "status":"PASS_SOURCE_MATCHED_BUILD","source_commit":build_source,
            "image_id":image,"exporter_replayer_sha256":"exporter","solver_sha256":"solver",
            "supervisor_sha256":"supervisor"}),
        )
        .unwrap();
        let preflight_root = root.join("preflight");
        fs::create_dir(&preflight_root).unwrap();
        let preflight_path = preflight_root.join("outer.json");
        write_new(
            &preflight_path,
            &json!({"schema":"n83.k600-m6-native-outer/v1",
            "status":"PREFLIGHT_ONLY","worker_exit_code":0,"timed_out":false,
            "lock_sha256":"preflight-lock"}),
        )
        .unwrap();
        let children = [&build_path, &stage_path, &arm_path, &preflight_path];
        let mut entries = Vec::new();
        for (name, child) in PRECHARGE.into_iter().zip(children) {
            let outer_path = root.join(format!("{name}-outer.json"));
            let wall = 7u64;
            let child_sha = sha256(child).unwrap();
            write_new(
                &outer_path,
                &json!({
                    "schema":"n83.k600-m6-native-child-interval/v1",
                    "status":"PASS_CHILD_INTERVAL","phase":name,
                    "source_commit":if name=="build" {&build_source} else {&source},
                    "controller_sha256":controller,"allowance_ms":ALLOWANCE_MS,
                    "child_exit_code":0,"wall_capped":false,"cleanup_ok":true,
                    "child_receipt_path":child,"child_receipt_sha256":child_sha,
                    "outer_wall_to_receipt_start_ms":wall,
                }),
            )
            .unwrap();
            entries.push(json!({"phase":name,"outer_receipt_path":outer_path,
                "outer_receipt_sha256":sha256(&outer_path).unwrap(),
                "child_receipt_path":child,"child_receipt_sha256":child_sha,
                "outer_wall_ms":wall}));
        }
        let precharge = json!({"schema":"n83.k600-m6-outer-precharge/v2",
            "status":"PASS_BOUND_CHILD_INTERVALS","source_commit":source,
            "build_source_commit":build_source,"controller_sha256":controller,
            "allowance_ms":ALLOWANCE_MS,"phases":entries,"total_outer_wall_ms":28});
        assert_eq!(
            precharge_ms(
                &precharge,
                &source,
                &build_source,
                controller,
                &arm_path,
                &preflight_root
            )
            .unwrap(),
            28
        );
        fs::write(root.join("build-outer.json"), b"changed").unwrap();
        assert!(precharge_ms(
            &precharge,
            &source,
            &build_source,
            controller,
            &arm_path,
            &preflight_root
        )
        .is_err());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn cumulative_ledger_accepts_distinct_supervisors_and_rejects_undercharge() {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "n83-cumulative-precharge-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir(&root).unwrap();
        let controller_source = "c".repeat(40);
        let arm_source = "a".repeat(40);
        let build_source = "b".repeat(40);
        let controller_sha = "d".repeat(64);
        let image = format!("sha256:{}", "e".repeat(64));
        let build_path = root.join("build.json");
        write_new(
            &build_path,
            &json!({"schema":"n83.k600-m6-linux-build/v1",
                "status":"PASS_SOURCE_MATCHED_BUILD", "source_commit":build_source,
                "image_id":image,"exporter_replayer_sha256":"exporter",
                "solver_sha256":"solver","supervisor_sha256":"f".repeat(64),
                "wall_to_receipt_start_ms":9}),
        )
        .unwrap();
        let stage_path = root.join("staging.json");
        write_new(
            &stage_path,
            &json!({"schema":"n83.k600-m6-native-arm-staging/v1",
                "status":"PASS_READ_ONLY_STAGE", "source_commit":arm_source,
                "fresh_s3_get":true,"uploaded":false,"solver_executed":false}),
        )
        .unwrap();
        let preflight_root = root.join("preflight");
        fs::create_dir(&preflight_root).unwrap();
        let preflight_path = preflight_root.join("outer.json");
        write_new(
            &preflight_path,
            &json!({"schema":"n83.k600-m6-native-outer/v1",
                "status":"PREFLIGHT_ONLY","worker_exit_code":0,
                "timed_out":false,"lock_sha256":"preflight-lock",
                "elapsed_wall_ms":4.2}),
        )
        .unwrap();
        let arm_path = root.join("arm-lock.json");
        write_new(
            &arm_path,
            &json!({"schema":"n83.k600-m6-native-arms-lock/v1",
                "status":"PREPARED_ONLY","source_commit":arm_source,
                "build_source_commit":build_source,"image_id":image,
                "exporter_replayer_sha256":"exporter","solver_sha256":"solver",
                "supervisor_sha256":"0".repeat(64),
                "staging_receipt_sha256":sha256(&stage_path).unwrap(),
                "preflight_lock_sha256":"preflight-lock"}),
        )
        .unwrap();
        let host_build_path = root.join("host-build.json");
        write_new(
            &host_build_path,
            &json!({"schema":"n83.k600-m6-host-build/v1",
                "status":"PASS_SOURCE_MATCHED_BUILD",
                "source_commit":arm_source,"supervisor_sha256":"0".repeat(64)}),
        )
        .unwrap();
        let artifacts = json!({
            "build":{"path":build_path,"sha256":sha256(&build_path).unwrap()},
            "host-build":{"path":host_build_path,"sha256":sha256(&host_build_path).unwrap()},
            "stage":{"path":stage_path,"sha256":sha256(&stage_path).unwrap()},
            "freeze":{"path":arm_path,"sha256":sha256(&arm_path).unwrap()},
            "preflight":{"path":preflight_path,"sha256":sha256(&preflight_path).unwrap()},
        });
        let mut intervals = Vec::new();
        for (phase, child, wall, meter) in [
            ("build", build_path.clone(), 9, "json-wall-ms"),
            ("host-build", host_build_path.clone(), 7, "json-wall-ms"),
            ("stage", stage_path.clone(), 7, "json-wall-ms"),
            ("freeze", arm_path.clone(), 7, "json-wall-ms"),
            (
                "preflight",
                preflight_path.clone(),
                5,
                "preflight-elapsed-ms",
            ),
        ] {
            let path = if phase == "host-build" || phase == "stage" || phase == "freeze" {
                let outer_path = root.join(format!("{phase}-outer.json"));
                write_new(
                    &outer_path,
                    &json!({"schema":"n83.k600-m6-native-child-interval/v1",
                        "status":"PASS_CHILD_INTERVAL","phase":phase,
                        "source_commit":controller_source,
                        "controller_sha256":controller_sha,
                        "child_receipt_path":child,
                        "child_receipt_sha256":sha256(&child).unwrap(),
                        "child_exit_code":0,"wall_capped":false,"cleanup_ok":true,
                        "outer_wall_to_receipt_start_ms":wall}),
                )
                .unwrap();
                outer_path
            } else {
                child
            };
            intervals.push(json!({"phase":phase,"path":path,
                "sha256":sha256(&path).unwrap(),"meter":meter,
                "charged_wall_ms":wall}));
        }
        let mut ledger = json!({"schema":"n83.k600-m6-cumulative-precharge/v3",
            "status":"PASS_BOUND_SETUP_LEDGER",
            "controller_source_commit":controller_source,
            "controller_sha256":controller_sha,
            "arm_source_commit":arm_source,"build_source_commit":build_source,
            "allowance_ms":100,"artifacts":artifacts,"intervals":intervals,
            "total_charged_wall_ms":35});
        assert_eq!(
            cumulative_precharge_ms(
                &ledger,
                &controller_source,
                &controller_sha,
                &arm_path,
                &preflight_root,
                100
            )
            .unwrap(),
            (arm_source.clone(), 35)
        );
        ledger["intervals"][0]["charged_wall_ms"] = json!(8);
        ledger["total_charged_wall_ms"] = json!(34);
        assert!(cumulative_precharge_ms(
            &ledger,
            &controller_source,
            &controller_sha,
            &arm_path,
            &preflight_root,
            100
        )
        .is_err());
        ledger["intervals"][0]["charged_wall_ms"] = json!(9);
        ledger["total_charged_wall_ms"] = json!(35);
        ledger["intervals"][1]["phase"] = json!("retry");
        assert!(cumulative_precharge_ms(
            &ledger,
            &controller_source,
            &controller_sha,
            &arm_path,
            &preflight_root,
            100
        )
        .is_err());
        fs::remove_dir_all(root).unwrap();
    }
}
