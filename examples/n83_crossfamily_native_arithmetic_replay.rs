//! Recheck all fresh public targets, IC relations, and paired rho scalars in
//! the completed four-arm N83 crossfamily archive without Python arithmetic.
//! Usage: n83_crossfamily_native_arithmetic_replay <archive-dir> <fixture-dir>

use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use crypto_lib::hash::sha256::{sha256, Sha256};
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde_json::{json, Value};
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::process::Command;

const SUBGROUP_ORDER: u64 = 8_569_786_107_849_059;
const TARGETS_SHA256: &str = "ac0b84bd916291331e0dd13bfb312aedcd50d7cd3bbc1c3cd5f81eeca22ea73d";
const ANSWERS_SHA256: &str = "2c47e48393346ca742eddff8e0cbfeb59b9bd486b154428f28aee2a90a5fa49b";
const ZIP_SHA256: [&str; 4] = [
    "036efe7a2e759524b7367544841e10a0ab3e0cf807580e03514ba4af5f3c968d",
    "d42aa19364ac541810ac6b1ca0bf1aed4882c9006ab17b881142b59b3a3e87d1",
    "cab308248b6abdc9ef6047bf905f6d0c648d749f1ac99289c25614eb49c2aa18",
    "f72a1b2563cce8798271e316042eb7f016f54c8c7089ba99bc0f5f2ac60b33ec",
];

struct Arm {
    name: &'static str,
    k: usize,
    base_size: usize,
    dump_sha256: &'static str,
    base_hash: &'static str,
}

const ARMS: [Arm; 4] = [
    Arm {
        name: "k2816_root11",
        k: 2816,
        base_size: 467456,
        dump_sha256: "e3b9a30d57ee61a31ae5b80d585ba6aee55475654062ad8dda17693a38adc5cb",
        base_hash: "9208460bf63be634f0aee6091abaaed855e05004fc2894e28a9c1fd394c35ec4",
    },
    Arm {
        name: "k3072_root11",
        k: 3072,
        base_size: 509952,
        dump_sha256: "a58965693e46b7af7e8de7852649c101d51f70854473459c99b5e8a8c6692874",
        base_hash: "c5511247735bc183359d5425c4dd794aaad49ad38d2ca964799e1caa76afaa08",
    },
    Arm {
        name: "k3450_wide3",
        k: 3450,
        base_size: 572700,
        dump_sha256: "5ff072e150a5c900ecc33930b8526aa9aaaea0951046e91040453f7ad0beeef3",
        base_hash: "ae75c86565f5fa12fb6f30a5539f6f4638bcf1ca8b347a5cefa20f5073a9f572",
    },
    Arm {
        name: "k3500_root4",
        k: 3500,
        base_size: 581000,
        dump_sha256: "7899b23a1d6dee9ca09561a743039b0b6fdbcac1213a95fc33f878948e9ca2dd",
        base_hash: "3387d0c40335c49256ad2048542455572980bef501c3735fd8a26f42ac035e67",
    },
];

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn sha_hex(bytes: &[u8]) -> String {
    hex::encode(sha256(bytes))
}

fn sha_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 1024 * 1024];
    loop {
        let count = file.read(&mut buf)?;
        if count == 0 {
            break;
        }
        hasher.update(&buf[..count]);
    }
    Ok(hex::encode(hasher.finalize()))
}

fn zip_member(zip: &Path, name: &str) -> Result<Vec<u8>> {
    let output = Command::new("unzip")
        .arg("-p")
        .arg(zip)
        .arg(name)
        .output()?;
    if !output.status.success() || output.stdout.is_empty() {
        return Err(format!("missing ZIP member {} in {}", name, zip.display()).into());
    }
    Ok(output.stdout)
}

fn zip_json(zip: &Path, name: &str) -> Result<Value> {
    Ok(serde_json::from_slice(&zip_member(zip, name)?)?)
}

fn point(value: &Value) -> Result<BinaryPoint> {
    let coords = value.as_array().ok_or("point is not an array")?;
    if coords.len() != 2 {
        return Err("point has wrong arity".into());
    }
    let mut words = Vec::new();
    for coord in coords {
        let word = BigUint::parse_bytes(
            coord.as_str().ok_or("coordinate is not text")?.as_bytes(),
            10,
        )
        .ok_or("invalid decimal coordinate")?;
        if word >= (BigUint::from(1u8) << 83) {
            return Err("noncanonical coordinate".into());
        }
        words.push(word);
    }
    Ok(BinaryPoint::Affine {
        x: F2mElement::from_biguint(&words[0], 83),
        y: F2mElement::from_biguint(&words[1], 83),
    })
}

fn one_json_line(bytes: &[u8]) -> Result<Value> {
    let mut lines = bytes
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty());
    let first = lines.next().ok_or("missing JSON row")?;
    if lines.next().is_some() {
        return Err("multiple JSON rows".into());
    }
    Ok(serde_json::from_slice(first)?)
}

fn positive(row: &Value, key: &str) -> Result<f64> {
    let value = row[key].as_f64().ok_or_else(|| format!("missing {key}"))?;
    if !value.is_finite() || value <= 0.0 {
        return Err(format!("invalid {key}").into());
    }
    Ok(value)
}

fn replay_arm(
    zip: &Path,
    index: usize,
    arm: &Arm,
    target: &BinaryPoint,
    scalar: u64,
    curve: &KoblitzCurve,
) -> Result<Value> {
    let supplied = zip_json(zip, &format!("results/points/p{index:02}/target.jsonl"))?;
    if point(&supplied)? != *target {
        return Err(format!("point {index} supplied target differs").into());
    }
    let prefix = format!("results/points/p{index:02}/runs/{}", arm.name);
    let dump_bytes = zip_member(zip, &format!("{prefix}/base_dump.jsonl"))?;
    if sha_hex(&dump_bytes) != arm.dump_sha256 {
        return Err(format!("point {index} {} base dump SHA-256 differs", arm.name).into());
    }
    let dump = one_json_line(&dump_bytes)?;
    let base = dump["factor_base_point_coordinates"]
        .as_array()
        .ok_or("base points absent")?;
    if base.len() != arm.base_size || dump["base_hash"] != arm.base_hash {
        return Err(format!("point {index} {} base identity differs", arm.name).into());
    }
    let summary = zip_json(zip, &format!("{prefix}/stdout.jsonl"))?;
    let result = zip_json(zip, &format!("{prefix}/targets_result.jsonl"))?;
    if summary["n"] != 83
        || summary["a"] != 1
        || summary["orbit_columns"] != arm.k
        || summary["rank"] != arm.k
        || summary["rank_relations"]
            .as_u64()
            .is_none_or(|count| count < arm.k as u64)
        || summary["factor_base_points"] != arm.base_size
        || summary["base_hash"] != arm.base_hash
        || summary["targets"] != 1
        || summary["targets_solved"] != 1
        || summary["targets_failed"] != 0
        || summary["target_threads"] != 2
        || summary["target_cancel"] != true
        || result["group_verified"] != true
        || result["fixture_index"] != 0
        || result["recovered_scalar"].as_u64() != Some(scalar)
        || point(&result["target"])? != *target
        || point(&result["published_q"])? != *target
        || !point(&result["generator"])?.eq(curve.generator())
    {
        return Err(format!("point {index} {} IC summary differs", arm.name).into());
    }
    let indices = result["point_indices"]
        .as_array()
        .ok_or("relation indices absent")?;
    if indices.len() != 4 {
        return Err("relation has wrong arity".into());
    }
    let mut sum = BinaryPoint::Infinity;
    for value in indices {
        let position = value.as_u64().ok_or("invalid base index")? as usize;
        let member = base.get(position).ok_or("base index out of range")?;
        sum = curve.add(&sum, &point(member)?);
    }
    if sum != *target {
        return Err(format!("point {index} {} relation differs", arm.name).into());
    }
    let online_ms = positive(&result, "target_ms")?;
    let phases = [
        "target_query_ms",
        "target_pdp_ms",
        "target_relation_check_ms",
        "target_descent_ms",
        "target_recovery_check_ms",
    ];
    let phase_values: Vec<f64> = phases
        .iter()
        .map(|key| result[*key].as_f64().unwrap_or(f64::NAN))
        .collect();
    if phase_values
        .iter()
        .any(|value| !value.is_finite() || *value < 0.0)
    {
        return Err(format!("point {index} {} has invalid online phase", arm.name).into());
    }
    let phase_sum: f64 = phase_values.iter().sum();
    if !phase_sum.is_finite() || (phase_sum - online_ms).abs() > 1.0 {
        return Err(format!("point {index} {} online phases differ", arm.name).into());
    }
    let cold_ms = positive(&summary["timing_ms"], "process_total")?;
    if cold_ms < online_ms {
        return Err("cold time below online time".into());
    }
    let rho = zip_json(
        zip,
        &format!(
            "results/points/p{index:02}/runs/rho_{}/stdout.jsonl",
            arm.name
        ),
    )?;
    let rho_scalar = rho["recovered_scalar"]
        .as_str()
        .ok_or("rho scalar absent")?
        .parse::<u64>()?;
    if rho["status"] != "PASS_VERIFIED_PUBLIC_TARGET"
        || rho_scalar != scalar
        || point(&rho["public_target_q"])? != *target
        || point(&rho["published_q"])? != *target
        || !point(&rho["generator"])?.eq(curve.generator())
        || rho["target_on_curve"] != true
        || rho["target_in_subgroup"] != true
        || rho["target_count"] != 1
    {
        return Err(format!("point {index} {} paired rho differs", arm.name).into());
    }
    let rho_online_ms = positive(&rho, "online_ms")?;
    Ok(json!({"point_index":index,"arm":arm.name,"k":arm.k,
              "ic_online_ms":online_ms,"ic_cold_ms":cold_ms,
              "rho_online_ms":rho_online_ms,
              "rho_over_ic_online":rho_online_ms/online_ms,
              "status":"PASS_NATIVE_TARGET_RELATION_AND_SCALAR_REPLAY"}))
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err(
            "usage: n83_crossfamily_native_arithmetic_replay ARCHIVE_DIR FIXTURE_DIR".into(),
        );
    }
    let root = Path::new(&args[1]);
    let fixture = Path::new(&args[2]);
    let targets = fs::read(fixture.join("fresh_targets.jsonl"))?;
    let answers = fs::read(fixture.join("fresh_answers.jsonl"))?;
    if sha_hex(&targets) != TARGETS_SHA256 || sha_hex(&answers) != ANSWERS_SHA256 {
        return Err("fresh fixture hashes differ".into());
    }
    let target_rows: Vec<Value> = std::str::from_utf8(&targets)?
        .lines()
        .map(serde_json::from_str)
        .collect::<std::result::Result<_, _>>()?;
    let answer_rows: Vec<Value> = std::str::from_utf8(&answers)?
        .lines()
        .map(serde_json::from_str)
        .collect::<std::result::Result<_, _>>()?;
    if target_rows.len() != 24 || answer_rows.len() != 24 {
        return Err("fixture coverage differs".into());
    }
    let curve = KoblitzCurve::new(1, 83).ok_or("N83 curve unavailable")?;
    if curve.subgroup_order.to_u64() != Some(SUBGROUP_ORDER)
        || curve.curve.irreducible.low_terms != [0, 2, 4, 7]
    {
        return Err("curve identity differs".into());
    }
    let mut rows = Vec::new();
    for stratum in 0..4 {
        let zip = root.join(format!("stratum{stratum}.zip"));
        if sha_file(&zip)? != ZIP_SHA256[stratum] {
            return Err(format!("stratum {stratum} ZIP SHA-256 differs").into());
        }
        for index in (stratum * 6)..(stratum * 6 + 6) {
            let target = point(&target_rows[index])?;
            let scalar = answer_rows[index]["scalar"]
                .as_str()
                .ok_or("fixture scalar absent")?
                .parse::<u64>()?;
            if answer_rows[index]["fixture_index"] != index
                || scalar == 0
                || scalar >= SUBGROUP_ORDER
                || curve.mul(curve.generator(), &BigUint::from(scalar)) != target
                || curve.mul(&target, &curve.subgroup_order) != BinaryPoint::Infinity
            {
                return Err(format!("fixture point {index} arithmetic differs").into());
            }
            for arm in &ARMS {
                rows.push(replay_arm(&zip, index, arm, &target, scalar, &curve)?);
            }
        }
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schema":"n83.crossfamily-native-arithmetic-replay/v1",
            "status":"PASS_NATIVE_IC_AND_PAIRED_RHO_TARGET_REPLAY",
            "archive_zip_sha256":ZIP_SHA256,
            "targets_sha256":TARGETS_SHA256,
            "answers_sha256":ANSWERS_SHA256,
            "points":24,"ic_runs":rows.len(),"paired_rho_runs":rows.len(),
            "calibration_and_host_replay":"ARCHIVED_ANALYSIS_ONLY",
            "rows":rows,
        }))?
    );
    Ok(())
}
