//! Native audit of retained N83 K2048 m5 solver artifacts.
//! Usage: n83_k2048_m5_native_audit EVIDENCE_DIR NEW_AUDIT_JSON

use crypto_lib::hash::sha256::{sha256, Sha256};
use serde_json::{json, Map, Value};
use std::error::Error;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn file_sha(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut sha = Sha256::new();
    let mut buffer = [0u8; 1024 * 1024];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        sha.update(&buffer[..count]);
    }
    Ok(hex::encode(sha.finalize()))
}

fn json_file(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

fn decompress(path: &Path) -> Result<Vec<u8>> {
    let output = Command::new("/opt/homebrew/bin/zstd")
        .arg("-dc")
        .arg(path)
        .output()?;
    if !output.status.success() {
        return Err(format!("zstd failed on {}", path.display()).into());
    }
    Ok(output.stdout)
}

fn collect_files(
    base: &Path,
    dir: &Path,
    out: &mut std::collections::BTreeSet<String>,
) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let kind = entry.file_type()?;
        if kind.is_dir() {
            collect_files(base, &entry.path(), out)?;
        } else if kind.is_file() {
            out.insert(
                entry
                    .path()
                    .strip_prefix(base)?
                    .to_string_lossy()
                    .into_owned(),
            );
        } else {
            return Err(format!(
                "unexpected native evidence entry: {}",
                entry.path().display()
            )
            .into());
        }
    }
    Ok(())
}

fn check_artifact_pointer(path: &Path) -> Result<bool> {
    let directory = path.parent().ok_or("artifact has no parent")?;
    let manifest_path = directory.join(".artifacts.json");
    if !manifest_path.exists() {
        return Ok(false);
    }
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("artifact name is not UTF-8")?;
    let manifest = json_file(&manifest_path)?;
    let files = manifest["files"]
        .as_object()
        .ok_or("artifact manifest files missing")?;
    let entry = files.get(name).ok_or("artifact pointer missing raw file")?;
    if manifest["schema"] != "artifacts.manifest/v1"
        || files.len() != 1
        || entry["sha256"] != file_sha(path)?
        || entry["bytes"] != fs::metadata(path)?.len()
    {
        return Err(format!("artifact pointer differs from {}", path.display()).into());
    }
    let ignore = fs::read_to_string(directory.join(".gitignore"))?;
    if !ignore.lines().any(|line| line == format!("/{name}")) {
        return Err(format!("artifact ignore rule missing for {}", path.display()).into());
    }
    Ok(true)
}

fn check_host(host: &Value, receipt: &Value) -> Result<Vec<f64>> {
    let samples = host.as_array().ok_or("host preflight is not an array")?;
    if samples.len() != 3 || &receipt["host_preflight"] != host {
        return Err("host preflight and receipt differ".into());
    }
    let mut loads = Vec::new();
    let mut last_ms = None;
    let baseline = samples[0]["swapouts"]
        .as_u64()
        .ok_or("host swapout missing")?;
    for sample in samples {
        let load = sample["load_1m"].as_f64().ok_or("host load missing")?;
        let ms = sample["unix_ms"].as_u64().ok_or("host time missing")?;
        if load > 24.0
            || sample["swapouts"].as_u64() != Some(baseline)
            || last_ms.is_some_and(|prior| ms < prior + 15_000)
        {
            return Err("host load, swapout or sample spacing gate failed".into());
        }
        last_ms = Some(ms);
        loads.push(load);
    }
    if receipt["swapouts_after"].as_u64() != Some(baseline) {
        return Err("post-run swapout count differs".into());
    }
    Ok(loads)
}

fn check_arm(base: &Path, mode: &str, hashes: &mut Map<String, Value>) -> Result<Value> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let study = root.join(
        "research/koblitz_n83_factor_base_sweep_20261008/verification/k2048-m5-xcnf-20261114",
    );
    let lock_name = if mode == "positive" {
        "NATIVE_POSITIVE_LOCK.json"
    } else {
        "NATIVE_NATURAL_LOCK.json"
    };
    let lock_path = study.join(lock_name);
    let lock = json_file(&lock_path)?;
    let arm = base.join(mode);
    let receipt = json_file(&arm.join("receipt.json"))?;
    let host = json_file(&arm.join("host-preflight.json"))?;
    let command = json_file(&arm.join("command.json"))?;
    if receipt["schema"] != "n83.k2048-m5-native-solver-run/v1"
        || receipt["mode"] != mode
        || receipt["lock_sha256"] != file_sha(&lock_path)?
        || receipt["source_commit"] != lock["source_commit"]
        || receipt["runner_binary_sha256"] != lock["runner_binary_sha256"]
        || receipt["solver_sha256"] != lock["solver_sha256"]
        || receipt["formula_sha256"] != lock["formula_sha256"]
        || receipt["export_receipt_sha256"] != lock["export_receipt_sha256"]
        || receipt["command"] != command
        || receipt["timed_out"] != false
    {
        return Err(format!("{mode} native receipt or command binding differs").into());
    }
    let loads = check_host(&host, &receipt)?;
    let stdout_path = arm.join("stdout.txt.zst");
    check_artifact_pointer(&stdout_path)?;
    let stdout = decompress(&stdout_path)?;
    let stdout_sha = hex::encode(sha256(&stdout));
    let stderr_sha = file_sha(&arm.join("stderr.txt"))?;
    if receipt["stdout_sha256"] != stdout_sha || receipt["stderr_sha256"] != stderr_sha {
        return Err(format!("{mode} solver transcript differs").into());
    }
    let text = String::from_utf8(stdout)?;
    let statuses = text
        .lines()
        .filter(|line| line.starts_with("s "))
        .collect::<Vec<_>>();
    let conflicts = text
        .lines()
        .filter_map(|line| line.strip_prefix("c conflicts                :"))
        .map(str::trim)
        .collect::<Vec<_>>();
    let expected_conflicts = if mode == "positive" { 839 } else { 1_000_001 };
    let expected_status = if mode == "positive" {
        "s SATISFIABLE"
    } else {
        "s INDETERMINATE"
    };
    let expected_code = if mode == "positive" { 10 } else { 15 };
    if statuses != [expected_status]
        || conflicts.len() != 1
        || conflicts[0].parse::<u64>()? != expected_conflicts
        || receipt["terminal_conflicts"] != expected_conflicts
        || receipt["exit_code"] != expected_code
        || receipt["status"]
            != if mode == "positive" {
                "SAT_PENDING_REPLAY"
            } else {
                "INDETERMINATE"
            }
        || (mode == "natural" && text.lines().any(|line| line.starts_with("v ")))
    {
        return Err(format!("{mode} terminal solver classification differs").into());
    }
    let formula_path = study.join(format!("evidence/export-{mode}/result.xcnf.zst"));
    check_artifact_pointer(&formula_path)?;
    let formula = decompress(&formula_path)?;
    if hex::encode(sha256(&formula)) != lock["formula_sha256"]
        || formula.len() as u64 != lock["formula_bytes"]
        || formula
            .split(|&byte| byte == b'\n')
            .next()
            .ok_or("formula header absent")?
            != lock["formula_header"]
                .as_str()
                .ok_or("locked header absent")?
                .as_bytes()
    {
        return Err(format!("{mode} original formula differs").into());
    }
    let mut names = vec![
        "command.json",
        "host-preflight.json",
        "receipt.json",
        "stdout.txt.zst",
        "stderr.txt",
    ];
    if mode == "positive" {
        let replay = json_file(&arm.join("replay.json"))?;
        if replay["status"] != "PASS_ORIGINAL_XCNF_GROUP_AND_ROW_REPLAY"
            || replay["solver_stdout_sha256"] != stdout_sha
            || replay["xcnf_sha256"] != lock["formula_sha256"]
            || replay["checked_cnf_clauses"] != 1_141_221
            || replay["checked_xor_rows"] != 4_980
            || replay["checked_archive_rows"] != 339_968
            || replay["group_lift_count"] != 5
        {
            return Err("native positive original-XCNF/group replay differs".into());
        }
        names.push("replay.json");
    }
    for name in names {
        let relative = format!("{mode}/{name}");
        let path = arm.join(name);
        hashes.insert(
            relative,
            json!({"bytes":fs::metadata(&path)?.len(),"sha256":file_sha(&path)?}),
        );
    }
    Ok(json!({
        "status":receipt["status"],"exit_code":expected_code,"conflicts":expected_conflicts,
        "wall_seconds":receipt["wall_seconds"],"cpu_user_seconds":receipt["cpu_user_seconds"],
        "cpu_system_seconds":receipt["cpu_system_seconds"],
        "wait4_peak_rss_bytes":receipt["wait4_peak_rss_bytes"],
        "host_load_1m":loads,"formula_sha256":lock["formula_sha256"],"stdout_sha256":stdout_sha,
    }))
}

fn audit(base: &Path, output: &Path) -> Result<()> {
    let mut hashes = Map::new();
    let legacy = base.join("legacy-positive-native-replay.json");
    let legacy_record = json_file(&legacy)?;
    if legacy_record["status"] != "PASS_ORIGINAL_XCNF_GROUP_AND_ROW_REPLAY"
        || legacy_record["checked_cnf_clauses"] != 1_141_221
        || legacy_record["checked_xor_rows"] != 4_980
    {
        return Err("retrospective native verifier sanity check differs".into());
    }
    hashes.insert(
        "legacy-positive-native-replay.json".into(),
        json!({"bytes":fs::metadata(&legacy)?.len(),"sha256":file_sha(&legacy)?}),
    );
    let positive = check_arm(base, "positive", &mut hashes)?;
    let natural = check_arm(base, "natural", &mut hashes)?;
    let expected = hashes
        .keys()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    let mut actual = std::collections::BTreeSet::new();
    collect_files(base, base, &mut actual)?;
    if base.join("positive/.artifacts.json").exists() {
        actual.remove("positive/.artifacts.json");
        actual.remove("positive/.gitignore");
    }
    if let Ok(relative) = output.strip_prefix(base) {
        actual.remove(relative.to_str().ok_or("audit path is not UTF-8")?);
    }
    if actual != expected {
        return Err("native evidence file set differs".into());
    }
    let report = json!({
        "schema":"n83.k2048-m5-native-audit/v1",
        "status":"PASS_NATIVE_RAW_CUSTODY_AND_TERMINAL_CLASSIFICATION",
        "files":hashes,"positive":positive,"natural":natural,
        "selected_best_total_runtime":Value::Null,
    });
    if output.exists() {
        if json_file(output)? != report {
            return Err("existing native audit differs from recomputation".into());
        }
    } else {
        let mut file = File::create_new(output)?;
        serde_json::to_writer_pretty(&mut file, &report)?;
        file.write_all(b"\n")?;
        file.sync_all()?;
    }
    println!(
        "{}",
        json!({"status":report["status"],"positive":positive["status"],"natural":natural["status"]})
    );
    Ok(())
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let result = if args.len() == 3 {
        audit(Path::new(&args[1]), Path::new(&args[2]))
    } else {
        Err("usage: n83_k2048_m5_native_audit EVIDENCE_DIR NEW_AUDIT_JSON".into())
    };
    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
