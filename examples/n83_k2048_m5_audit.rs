//! Independent native audit of the frozen paired N83 K2048 model receipts.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const BINARY: &str = "5123ef012623a0f425f93a91979a918f7e2df369c1309352919a47e9eef52f06";
const SOURCE: &str = "a70610fb78a193bece7c387a72594dbad0f233b277ec7430fbb59213fd51f12a";
const CORPUS: &str = "760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd";
const U_OBJECT: &str = "aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a";
const D_OBJECT: &str = "2406a7105bff162fc80e448d441709cc9f725e36da9751a2e9feb6f3e23512c7";
const IMAGE: &str = "sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31";
const MEMORY: u64 = 4 * 1024 * 1024 * 1024;

fn must(ok: bool, message: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(message.into())
    }
}

fn read_json(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn audit(directory: &Path) -> Result<Value> {
    let cells = [
        ("u-0-preflight", "U", 0, "preflight"),
        ("d-0-preflight", "D", 0, "preflight"),
        ("u-0-search", "U", 0, "search"),
        ("d-0-search", "D", 0, "search"),
        ("d-1-search", "D", 1, "search"),
        ("u-1-search", "U", 1, "search"),
    ];
    let targets = [
        ["1834913121178197242856238", "8797964190732846213981377"],
        ["5809312440517013698248638", "3568442427236746854022671"],
    ];
    let mut previous_end = 0_u64;
    let mut rows = Vec::with_capacity(cells.len());
    for (cell, arm, fixture, mode) in cells {
        let guard = read_json(&directory.join(format!("{cell}-native-guard.json")))?;
        let worker = read_json(&directory.join(format!("{cell}-native.json")))?;
        let object = if arm == "U" { U_OBJECT } else { D_OBJECT };
        must(
            guard["schema"] == "n83.k2048-m5-native-guard/v1"
                && guard["cell"] == cell
                && guard["status"] == "PASS",
            "native guard identity/status differs",
        )?;
        must(
            worker["schema"] == "n83.k2048-m5-model/v1"
                && worker["arm"] == arm
                && worker["fixture_index"] == fixture
                && worker["mode"] == mode,
            "worker cell identity differs",
        )?;
        must(
            worker["target_point"] == json!(targets[fixture]),
            "target point differs",
        )?;
        must(
            worker["curve_id"] == "EC1N83Ckb0h252dacc102a4",
            "curve identity differs",
        )?;
        must(
            worker["columns"] == 2048
                && worker["points"] == 339968
                && worker["representative_x_count"] == 2048,
            "base dimensions differ",
        )?;
        must(
            guard["object_sha256"] == object && worker["object_sha256"] == object,
            "object digest differs",
        )?;
        must(
            guard["corpus_sha256"] == CORPUS && worker["corpus_sha256"] == CORPUS,
            "corpus digest differs",
        )?;
        must(
            guard["binary_sha256"] == BINARY && worker["binary_sha256"] == BINARY,
            "binary digest differs",
        )?;
        must(
            worker["source_blake3"] == SOURCE,
            "worker source digest differs",
        )?;
        must(
            guard["image_id"] == IMAGE
                && guard["cpu_limit"] == 1
                && guard["memory_limit_bytes"] == MEMORY,
            "declared resource envelope differs",
        )?;
        must(
            guard["actual_memory_limit_bytes"] == MEMORY
                && guard["actual_memory_swap_bytes"] == MEMORY
                && guard["actual_nano_cpus"] == 1_000_000_000_u64,
            "actual CPU/memory envelope differs",
        )?;
        must(
            guard["actual_network_mode"] == "none" && guard["actual_read_only_rootfs"] == true,
            "network or rootfs envelope differs",
        )?;
        must(
            guard["exit_code"] == 0
                && guard["oom_killed"] == false
                && guard["outer_watchdog_fired"] == false
                && guard["worker_receipt_exists"] == true,
            "guard termination differs",
        )?;
        let peak = guard["cgroup_memory_peak_bytes"]
            .as_u64()
            .ok_or("guard peak missing")?;
        must(
            peak <= MEMORY
                && peak
                    >= worker["memory_cgroup_peak_bytes"]
                        .as_u64()
                        .ok_or("worker peak missing")?,
            "cgroup peak differs",
        )?;
        must(
            worker["sat_variables"] == 111478
                && worker["sat_xor_rows"] == 4980
                && worker["sat_clauses"].as_u64().ok_or("clauses absent")? >= 1140557,
            "model dimensions differ",
        )?;
        must(
            worker["group_relation_verified"] == false
                && worker["rank"].is_null()
                && worker["candidate_id"].is_null()
                && worker["online_ms"].is_null(),
            "unverified model promoted",
        )?;
        if mode == "preflight" {
            must(
                worker["root_status"] == "SKIPPED"
                    && worker["solve_status"] == "SKIPPED"
                    && worker["conflicts"] == 0
                    && worker["sat_clauses"] == 1140557,
                "preflight stage differs",
            )?;
        } else {
            must(worker["root_status"] == "Fixpoint", "root status differs")?;
            must(
                ["Unknown", "Sat", "Unsat"]
                    .iter()
                    .any(|&status| worker["solve_status"] == status),
                "solver status differs",
            )?;
            must(
                worker["conflicts"]
                    .as_u64()
                    .ok_or("conflict count absent")?
                    <= 5000,
                "conflict cap exceeded",
            )?;
            if worker["solve_status"] == "Sat" {
                must(worker["model_verified"] == true, "SAT model unverified")?;
            }
        }
        let start = guard["started_epoch_ms"]
            .as_u64()
            .ok_or("start time absent")?;
        let end = guard["ended_epoch_ms"].as_u64().ok_or("end time absent")?;
        must(
            start >= previous_end && end >= start,
            "cell order or interval differs",
        )?;
        previous_end = end;
        rows.push(json!({
            "cell":cell,"solve_status":worker["solve_status"],"conflicts":worker["conflicts"],
            "model_ms":worker["model_ms"],"root_ms":worker["root_ms"],"search_ms":worker["search_ms"],
            "process_wall_ms":worker["process_wall_ms"],"peak_bytes":peak,
            "started_epoch_ms":start,"ended_epoch_ms":end,
        }));
    }
    Ok(json!({
        "schema":"n83.k2048-m5-panel-audit-native/v1","status":"PASS","cells":rows,
        "factor_base_selection":Value::Null,"single_target_online_speedup":Value::Null,
        "auditor_source_blake3":blake3::hash(include_bytes!("n83_k2048_m5_audit.rs")).to_hex().to_string(),
        "auditor_binary_sha256":sha256(&fs::read(std::env::current_exe()?)?),
    }))
}

fn run() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let [_, directory, output] = args.as_slice() else {
        return Err("usage: n83_k2048_m5_audit RECEIPT_DIR OUTPUT".into());
    };
    let value = audit(Path::new(directory))?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)?;
    serde_json::to_writer_pretty(&mut file, &value)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    println!("{}", serde_json::to_string(&value)?);
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("n83 K2048 m5 native audit: {error}");
        std::process::exit(1);
    }
}
