//! Independent identity, schedule and arithmetic audit of the N83 cadence card.
#![recursion_limit = "256"]
use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use num_bigint::BigUint;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const SOURCE_SHA256: &str = "6402da58e23b2b3e8badf7f8794c615d227b52b9550e3cd639ffd258eba77f03";
const SOURCE_BLAKE3: &str = "4be446e224a06e1aca6a9daacedc0f4e32fabd6bf5ddfbf20d68c613bf4cdb0a";
const BINARY_SHA256: &str = "0fdbb812d25c3bb9ee78f1f8740ee05e7db1d05b52af8e1a80cf59da82d55852";
const CORPUS_SHA256: &str = "760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd";
const U_SHA256: &str = "aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a";
const D_SHA256: &str = "2406a7105bff162fc80e448d441709cc9f725e36da9751a2e9feb6f3e23512c7";
const TARGET_X: &str = "1834913121178197242856238";
const TARGET_Y: &str = "8797964190732846213981377";
const SCHEDULE: [(&str, &str, u64, bool); 8] = [
    ("01-u-c1.json", "U", 1, true),
    ("02-d-c8.json", "D", 8, true),
    ("03-d-c1.json", "D", 1, true),
    ("04-u-c8.json", "U", 8, true),
    ("05-u-c8.json", "U", 8, true),
    ("06-d-c1.json", "D", 1, true),
    ("07-d-c8.json", "D", 8, true),
    ("09-u-c1-supplement.json", "U", 1, false),
];

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn must(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.into())
    }
}

fn positive_number(row: &Value, key: &str) -> Result<f64> {
    let value = row[key].as_f64().ok_or("timing is not numeric")?;
    must(value.is_finite() && value >= 0.0, key)?;
    Ok(value)
}

fn verify_receipt(path: &Path, arm: &str, cadence: u64, scheduled: bool) -> Result<(Value, Value)> {
    let bytes = fs::read(path)?;
    let row: Value = serde_json::from_slice(&bytes)?;
    let expected_object = if arm == "U" { U_SHA256 } else { D_SHA256 };
    let expected_policy = if arm == "U" {
        "public_x_hash"
    } else {
        "direct_subgroup_x_d18_prefix2048"
    };
    must(
        row["schema"] == "n83.k2048-m5-cadence/v1"
            && row["arm"] == arm
            && row["cadence"] == cadence
            && row["fixture_index"] == 0
            && row["curve_id"] == "EC1N83Ckb0h252dacc102a4"
            && row["target_point"] == json!([TARGET_X, TARGET_Y])
            && row["object_sha256"] == expected_object
            && row["policy"] == expected_policy
            && row["corpus_sha256"] == CORPUS_SHA256
            && row["binary_sha256"] == BINARY_SHA256
            && row["host_os"] == "macos"
            && row["host_arch"] == "aarch64"
            && row["columns"] == 2048
            && row["points"] == 339_968
            && row["representative_x_count"] == 2048
            && row["sat_variables"] == 111_478
            && row["sat_clauses"] == 1_140_557
            && row["sat_xor_rows"] == 4_980
            && row["root_status"] == "Fixpoint"
            && row["solve_status"] == "Unknown"
            && row["conflicts"] == 5_000
            && row["model_verified"] == Value::Null
            && row["group_relation_verified"] == false
            && row["rank"] == Value::Null
            && row["candidate_id"] == Value::Null
            && row["online_ms"] == Value::Null,
        "receipt identity, cap or result differs",
    )?;
    must(
        row["source_blake3"] == SOURCE_BLAKE3,
        "worker source digest differs",
    )?;
    for key in [
        "input_ms",
        "model_ms",
        "root_ms",
        "search_ms",
        "process_wall_ms",
    ] {
        positive_number(&row, key)?;
    }
    let word_ops = row["word_ops"].as_u64().ok_or("word_ops absent")?;
    let xor_passes = row["xor_passes"].as_u64().ok_or("xor_passes absent")?;
    let xor_row_scans = row["xor_row_scans"].as_u64().ok_or("xor scans absent")?;
    must(
        word_ops > 0 && xor_passes > 0 && xor_row_scans > 0,
        "zero solver work",
    )?;
    must(
        positive_number(&row, "process_wall_ms")? >= positive_number(&row, "search_ms")?,
        "search interval exceeds process interval",
    )?;
    let compact = json!({
        "file":path.file_name().ok_or("receipt filename absent")?.to_string_lossy(),
        "arm":arm,"cadence":cadence,"scheduled":scheduled,
        "receipt_sha256":sha256(&bytes),
        "word_ops":word_ops,"conflicts":row["conflicts"],
        "decisions":row["decisions"],"xor_passes":xor_passes,
        "xor_row_scans":xor_row_scans,
        "search_ms":row["search_ms"],"status":row["solve_status"]
    });
    Ok((row, compact))
}

fn run() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 5 {
        return Err("usage: verifier U_OBJECT D_OBJECT EVIDENCE_DIR OUTPUT".into());
    }
    must(
        sha256(include_bytes!("n83_k2048_m5_cadence.rs")) == SOURCE_SHA256,
        "worker source differs from lock",
    )?;
    must(
        sha256(&fs::read(&args[1])?) == U_SHA256,
        "U archive differs",
    )?;
    must(
        sha256(&fs::read(&args[2])?) == D_SHA256,
        "D archive differs",
    )?;
    let kc = KoblitzCurve::known_n83_k0().ok_or("N83 curve unavailable")?;
    must(
        kc.label() == "icv1-f2m83-tm6151469093347-debefd74"
            && kc.subgroup_order.to_string() == "2417851639230796216685689",
        "curve identity differs",
    )?;
    let x = BigUint::parse_bytes(TARGET_X.as_bytes(), 10).ok_or("target x invalid")?;
    let y = BigUint::parse_bytes(TARGET_Y.as_bytes(), 10).ok_or("target y invalid")?;
    let target = BinaryPoint::Affine {
        x: F2mElement::from_biguint(&x, 83),
        y: F2mElement::from_biguint(&y, 83),
    };
    must(
        kc.curve.is_on_curve(&target)
            && kc.mul(&target, &kc.subgroup_order) == BinaryPoint::Infinity,
        "public target group replay failed",
    )?;
    let dir = Path::new(&args[3]);
    let output = Path::new(&args[4]);
    must(!output.exists(), "audit output already exists")?;
    let failure_bytes = fs::read(dir.join("launch-error-08.json"))?;
    let failure: Value = serde_json::from_slice(&failure_bytes)?;
    must(
        failure["schema"] == "n83.k2048-m5-cadence-launch-error/v1"
            && failure["scheduled_position"] == 8
            && failure["arm"] == "U"
            && failure["cadence"] == 1
            && failure["exit_code"] == 1
            && failure["status"] == "INPUT_PATH_FAILURE"
            && failure["worker_receipt_written"] == false
            && failure["model_built"] == false
            && failure["solver_started"] == false,
        "scheduled launch failure differs",
    )?;
    let mut rows = Vec::new();
    let mut compact = Vec::new();
    for (filename, arm, cadence, scheduled) in SCHEDULE {
        let (row, summary) = verify_receipt(&dir.join(filename), arm, cadence, scheduled)?;
        rows.push(row);
        compact.push(summary);
    }
    let mut comparisons = Vec::new();
    for arm in ["U", "D"] {
        let selected = |cadence: u64| -> Result<Vec<u64>> {
            rows.iter()
                .filter(|row| row["arm"] == arm && row["cadence"] == cadence)
                .map(|row| row["word_ops"].as_u64().ok_or("word_ops absent".into()))
                .collect()
        };
        let baseline = selected(1)?;
        let deferred = selected(8)?;
        must(
            baseline.len() == 2 && deferred.len() == 2,
            "repeat count differs",
        )?;
        must(
            baseline[0] == baseline[1] && deferred[0] == deferred[1],
            "deterministic repeat work differs",
        )?;
        let base_sum = baseline.iter().map(|&n| u128::from(n)).sum::<u128>();
        let deferred_sum = deferred.iter().map(|&n| u128::from(n)).sum::<u128>();
        let ratio = base_sum as f64 / deferred_sum as f64;
        comparisons.push(json!({
            "arm":arm,"baseline_word_ops":baseline,"deferred_word_ops":deferred,
            "baseline_sum":base_sum.to_string(),"deferred_sum":deferred_sum.to_string(),
            "baseline_over_deferred_word_ops":format!("{ratio:.6}"),
            "baseline_spread":baseline.iter().max().unwrap()-baseline.iter().min().unwrap(),
            "deferred_spread":deferred.iter().max().unwrap()-deferred.iter().min().unwrap(),
            "scheduled_baseline_repeats":if arm == "U" {1} else {2},
            "supplemental_baseline_repeats":if arm == "U" {1} else {0}
        }));
    }
    let result = json!({
        "schema":"n83.k2048-m5-cadence-audit/v1","status":"PASS",
        "source_sha256":SOURCE_SHA256,"binary_sha256":BINARY_SHA256,
        "archives_rehashed":true,"public_target_subgroup_replayed":true,
        "scheduled_completed":7,"scheduled_input_failures":1,"supplemental_completed":1,
        "launch_failure_sha256":sha256(&failure_bytes),
        "rows":compact,"comparisons":comparisons,
        "factor_base_selection":Value::Null,"single_target_online_speedup":Value::Null,
        "source_blake3":blake3::hash(include_bytes!("n83_k2048_m5_cadence_verify.rs")).to_hex().to_string()
    });
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)?;
    serde_json::to_writer_pretty(&mut file, &result)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("N83 K2048 cadence verifier: {error}");
        std::process::exit(1);
    }
}
