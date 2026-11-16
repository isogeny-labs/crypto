//! Native audit of the retained N83 K4405 bounded host-swap control.

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const SOURCE_SHA: &str = "58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8";
const BINARY_SHA: &str = "07f7b5b424d0f1ecea1cd7e6564522c4d9c64557a820441c381e7474b13a388a";
const ARCHIVED_SOURCE: &str = "research/n83_k2304_current_source_control_20261026/koblitz_orbit_dlp_fast.executed-20261026.rs";
const PUBLIC: &str = "research/n83_cold_factor_base_20261016/targets.jsonl";
const ANSWERS: &str = "research/n83_cold_factor_base_20261016/answers.jsonl";
type Result<T> = std::result::Result<T, String>;

fn digest(path: &Path) -> Result<String> {
    let bytes = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

fn read(path: &Path) -> Result<Value> {
    serde_json::from_slice(&fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?)
        .map_err(|e| format!("{}: {e}", path.display()))
}

fn require(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.into())
    }
}

fn audited_files(root: &Path) -> Result<BTreeMap<String, String>> {
    let mut actual = BTreeMap::new();
    for entry in fs::read_dir(root).map_err(|e| format!("{}: {e}", root.display()))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if !path.is_file() || path.file_name().and_then(|s| s.to_str()) == Some("manifest.json") {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or("invalid file name")?;
        actual.insert(name.into(), digest(&path)?);
    }
    Ok(actual)
}

fn audit(root: &Path) -> Result<Value> {
    let manifest = read(&root.join("manifest.json"))?;
    require(
        manifest["schema"] == "n83.k4405-local-abort-evidence/v1",
        "manifest schema differs",
    )?;
    let expected: BTreeMap<String, String> = manifest["sha256"]
        .as_object()
        .ok_or("manifest sha256 map missing")?
        .iter()
        .map(|(name, sha)| {
            Ok((
                name.clone(),
                sha.as_str().ok_or("invalid manifest digest")?.into(),
            ))
        })
        .collect::<Result<_>>()?;
    require(
        audited_files(root)? == expected,
        "evidence file inventory or SHA-256 differs",
    )?;

    let config = read(&root.join("config.json"))?;
    let receipt = read(&root.join("receipt.json"))?;
    let abort = read(&root.join("manual_abort.json"))?;
    let pressure = read(&root.join("host_pressure_observation.json"))?;
    let samples = read(&root.join("samples.json"))?;
    require(
        config["schema"] == "n83.k4405-current-source-local-control/v1",
        "config schema differs",
    )?;
    require(
        config["source_sha256"] == SOURCE_SHA && config["binary_sha256"] == BINARY_SHA,
        "producer source or binary differs",
    )?;
    require(
        config["command"][1] == "construct:83:1:4405",
        "producer command differs",
    )?;
    for (key, value) in [
        ("KIC_ROOT_ONLY11", "1"),
        ("KIC_INDEXED5_ID30", "1"),
        ("KIC_ROOT_BLOOM_PROFILE", "wide3"),
        ("KIC_TARGET_CANCEL", "1"),
        ("KIC_TARGET_THREADS", "2"),
    ] {
        require(
            config["algorithm_flags"][key] == value,
            &format!("{key} differs"),
        )?;
    }
    let cap = config["cap_seconds"]
        .as_f64()
        .ok_or("cap_seconds missing")?;
    require(
        cap == 1200.0 && config["rss_cap_bytes"] == 24_u64 * 1024_u64.pow(3),
        "control caps differ",
    )?;
    require(
        config["initial_host_free_percent"].as_u64().unwrap_or(0) >= 50,
        "host admission differs",
    )?;
    require(
        receipt["status"] == "PRODUCER_FAILURE_EXIT" && receipt["exit_code"] == -15,
        "terminal receipt differs",
    )?;
    require(
        abort["status"] == "BOUNDED_UNKNOWN_HOST_SWAP_ABORT"
            && abort["signal"] == "SIGTERM"
            && abort["worker_pid"].as_u64().unwrap_or(0) > 0,
        "manual abort differs",
    )?;
    require(
        abort["receipt_sha256"] == digest(&root.join("receipt.json"))?
            && abort["runner_v1_sha256"] == digest(&root.join("runner_v1.py"))?,
        "manual abort hash binding differs",
    )?;
    require(
        pressure["page_size_bytes"] == 16384
            && pressure["pages_free"] == 3746
            && pressure["pages_occupied_by_compressor"] == 1551744,
        "host pressure observation differs",
    )?;
    for (field, file) in [
        ("samples_sha256", "samples.json"),
        ("stdout_sha256", "stdout.jsonl"),
        ("stderr_sha256", "stderr.log"),
        ("target_sha256", "target.jsonl"),
        ("targets_result_sha256", "targets_result.jsonl"),
    ] {
        require(
            receipt[field] == digest(&root.join(file))?,
            &format!("{field} differs"),
        )?;
    }
    require(
        receipt["base_dump_sha256"].is_null(),
        "unexpected base dump hash",
    )?;
    let sample_rows = samples.as_array().ok_or("samples array missing")?;
    let maximum = sample_rows
        .iter()
        .map(|row| row["rss_bytes"].as_u64().unwrap_or(0))
        .max()
        .unwrap_or(0);
    require(
        receipt["max_sampled_rss_bytes"] == maximum,
        "max sampled RSS differs",
    )?;
    let wall = receipt["wall_seconds"]
        .as_f64()
        .ok_or("wall_seconds missing")?;
    require(wall > 0.0 && wall < cap, "wall interval differs")?;
    require(
        fs::metadata(root.join("stdout.jsonl"))
            .map_err(|e| e.to_string())?
            .len()
            == 0
            && fs::metadata(root.join("targets_result.jsonl"))
                .map_err(|e| e.to_string())?
                .len()
                == 0,
        "unexpected producer output",
    )?;
    Ok(
        json!({"status":"PASS_BOUNDED_HOST_SWAP_ABORT_RECEIPT", "wall_seconds":wall,
        "max_sampled_rss_bytes":maximum}),
    )
}

fn command_output(program: &str, args: &[&str]) -> Result<String> {
    let mut child = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("{program}: {e}"))?;
    let start = Instant::now();
    loop {
        if let Some(status) = child.try_wait().map_err(|e| format!("{program}: {e}"))? {
            let mut stdout = String::new();
            child
                .stdout
                .take()
                .ok_or("telemetry stdout missing")?
                .read_to_string(&mut stdout)
                .map_err(|e| e.to_string())?;
            require(status.success(), &format!("{program} exited {status}"))?;
            return Ok(stdout);
        }
        if start.elapsed() >= Duration::from_secs(5) {
            let _ = child.kill();
            let _ = child.wait();
            return Err(format!("{program} telemetry timeout after 5 seconds"));
        }
        thread::sleep(Duration::from_millis(20));
    }
}

fn first_jsonl(path: &Path) -> Result<Value> {
    let raw = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(raw.lines().next().ok_or("empty fixture")?).map_err(|e| e.to_string())
}

fn preflight(binary: &Path) -> Result<Value> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let binary = binary
        .canonicalize()
        .map_err(|e| format!("{}: {e}", binary.display()))?;
    require(
        digest(&root.join(ARCHIVED_SOURCE))? == SOURCE_SHA,
        "archived producer source differs",
    )?;
    require(digest(&binary)? == BINARY_SHA, "producer binary differs")?;
    let point = first_jsonl(&root.join(PUBLIC))?;
    let answer = first_jsonl(&root.join(ANSWERS))?;
    require(
        answer["fixture_index"] == 0 && answer["point"] == point,
        "earlier public fixture differs",
    )?;
    let free_output = command_output("memory_pressure", &["-Q"])?;
    let free = free_output
        .lines()
        .find_map(|line| {
            line.split_once("System-wide memory free percentage: ")
                .and_then(|(_, value)| value.trim_end_matches('%').parse::<u64>().ok())
        })
        .ok_or("host free-memory telemetry missing")?;
    let swap_output = command_output("sysctl", &["vm.swapusage"])?;
    let swap = swap_output
        .split("used = ")
        .nth(1)
        .and_then(|s| s.split('M').next())
        .and_then(|s| s.parse::<f64>().ok())
        .ok_or("host swap telemetry missing")?;
    let status = if free >= 50 && swap <= 2048.0 {
        "PASS_PREFLIGHT"
    } else {
        "HOST_ADMISSION_BLOCKED"
    };
    Ok(
        json!({"status":status, "host_free_percent":free, "host_swap_used_mib":swap,
        "min_host_free_percent":50, "max_host_swap_used_mib":2048,
        "source_sha256":SOURCE_SHA, "binary_sha256":BINARY_SHA,
        "target":point, "scope":"admission-only"}),
    )
}

fn evidence_path(args: &[String]) -> Result<PathBuf> {
    let default = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("research/n83_k4405_current_source_full_control_20261026/evidence-01");
    match args {
        [] => Ok(default),
        [flag, path] if flag == "--evidence" => Ok(PathBuf::from(path)),
        _ => Err("usage: n83_k4405_audit [--evidence PATH] | --preflight --binary PATH".into()),
    }
}

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let result = match args.as_slice() {
        [flag, key, binary] if flag == "--preflight" && key == "--binary" => {
            preflight(Path::new(binary))
        }
        _ => evidence_path(&args).and_then(|root| audit(&root)),
    };
    match result {
        Ok(report) => {
            println!("{report}");
            if report["status"] == "HOST_ADMISSION_BLOCKED" {
                std::process::exit(2);
            }
        }
        Err(e) => {
            eprintln!("n83_k4405_audit: {e}");
            std::process::exit(1);
        }
    }
}
