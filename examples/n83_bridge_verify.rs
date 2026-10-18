//! Independent native replay of one N83 bridge IC result.
//! Usage: n83_bridge_verify <K> <point.jsonl> <scalar> <run-folder>

use std::fs;
use std::path::Path;

use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use crypto_lib::hash::sha256;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde_json::{json, Value};

const SUBGROUP_ORDER: u64 = 8_569_786_107_849_059;
const BASES: &[(usize, &str, &str)] = &[
    (
        1107,
        "08608f6ad2b8c66510246348933740c78d0937cae783d521514f467940edc3dc",
        "b0aea97c1c8b9b46e13df67e6af9c33ca73034d2ec3eff33a2392ceb1452b40e",
    ),
    (
        2304,
        "f51e436c399688ebd710075f16a57cd631a6799ce79891d53250b46bfd6c4534",
        "cc39291fc49ba8d4c25c0c9aba46012bda023b4d7294a46f73d6c3f809727b3f",
    ),
];

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn fail(message: impl Into<String>) -> Box<dyn std::error::Error> {
    message.into().into()
}

fn read_json(path: &Path) -> Result<Value> {
    let bytes = fs::read(path)?;
    let mut lines = bytes
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty());
    let line = lines
        .next()
        .ok_or_else(|| fail(format!("empty {}", path.display())))?;
    if lines.next().is_some() {
        return Err(fail(format!("multiple JSON rows in {}", path.display())));
    }
    Ok(serde_json::from_slice(line)?)
}

fn point(value: &Value) -> Result<BinaryPoint> {
    let coords = value
        .as_array()
        .ok_or_else(|| fail("point is not an array"))?;
    if coords.len() != 2 {
        return Err(fail("point does not have two coordinates"));
    }
    let numbers: Vec<BigUint> = coords
        .iter()
        .map(|coordinate| {
            let text = coordinate
                .as_str()
                .ok_or_else(|| fail("coordinate is not decimal text"))?;
            BigUint::parse_bytes(text.as_bytes(), 10).ok_or_else(|| fail("invalid coordinate"))
        })
        .collect::<Result<Vec<_>>>()?;
    let bound = BigUint::from(1u8) << 83;
    if numbers.iter().any(|number| number >= &bound) {
        return Err(fail("noncanonical N83 coordinate"));
    }
    Ok(BinaryPoint::Affine {
        x: F2mElement::from_biguint(&numbers[0], 83),
        y: F2mElement::from_biguint(&numbers[1], 83),
    })
}

fn number(row: &Value, key: &str) -> Result<f64> {
    let value = row[key]
        .as_f64()
        .ok_or_else(|| fail(format!("missing {key}")))?;
    if !value.is_finite() || value < 0.0 {
        return Err(fail(format!("invalid {key}")));
    }
    Ok(value)
}

fn verify(k: usize, target_path: &Path, scalar_text: &str, folder: &Path) -> Result<Value> {
    let (_, expected_dump_sha, expected_base_hash) = BASES
        .iter()
        .find(|(value, _, _)| *value == k)
        .ok_or_else(|| fail("unsupported K"))?;
    let scalar =
        BigUint::parse_bytes(scalar_text.as_bytes(), 10).ok_or_else(|| fail("invalid scalar"))?;
    if scalar == BigUint::from(0u8) || scalar >= BigUint::from(SUBGROUP_ORDER) {
        return Err(fail("scalar outside the exact subgroup"));
    }
    let curve = KoblitzCurve::new(1, 83).ok_or_else(|| fail("N83 curve unavailable"))?;
    if curve.subgroup_order.to_u64() != Some(SUBGROUP_ORDER)
        || curve.curve.irreducible.low_terms != [0, 2, 4, 7]
    {
        return Err(fail("curve parameters differ"));
    }
    let target = point(&read_json(target_path)?)?;
    if curve.mul(curve.generator(), &scalar) != target
        || curve.mul(&target, &curve.subgroup_order) != BinaryPoint::Infinity
    {
        return Err(fail("public point scalar or subgroup replay failed"));
    }
    let dump_path = folder.join("base_dump.jsonl");
    let dump_bytes = fs::read(&dump_path)?;
    let dump_sha = hex::encode(sha256(&dump_bytes));
    if dump_sha != *expected_dump_sha {
        return Err(fail(
            "ordered base dump differs from independently replayed base",
        ));
    }
    let dump: Value = serde_json::from_slice(&dump_bytes)?;
    let base = dump["factor_base_point_coordinates"]
        .as_array()
        .ok_or_else(|| fail("base points missing"))?;
    if base.len() != 166 * k || dump["base_hash"] != *expected_base_hash {
        return Err(fail("factor-base size or hash differs"));
    }
    let summary = read_json(&folder.join("stdout.jsonl"))?;
    let result = read_json(&folder.join("targets_result.jsonl"))?;
    if summary["n"] != 83
        || summary["a"] != 1
        || summary["orbit_columns"] != k
        || summary["rank"] != k
        || summary["factor_base_points"] != 166 * k
        || summary["targets"] != 1
        || summary["targets_solved"] != 1
        || summary["targets_failed"] != 0
        || summary["target_threads"] != 2
        || summary["target_cancel"] != true
        || summary["root_only11_enabled"] != true
        || summary["indexed5_load75_enabled"] != true
        || summary["root_table_indexed5_id30_enabled"] != false
        || summary["base_hash"] != *expected_base_hash
        || result["group_verified"] != true
        || result["published_fixture_scalar"] != Value::Null
        || result["recovered_scalar"].as_u64() != scalar.to_u64()
        || point(&result["target"])? != target
    {
        return Err(fail("summary, target or two-worker policy differs"));
    }
    let indices = result["point_indices"]
        .as_array()
        .ok_or_else(|| fail("point indices missing"))?;
    if indices.len() != 4 {
        return Err(fail("decomposition does not have four summands"));
    }
    let mut sum = BinaryPoint::Infinity;
    for index in indices {
        let position = index
            .as_u64()
            .ok_or_else(|| fail("bad factor-base index"))? as usize;
        let member = base
            .get(position)
            .ok_or_else(|| fail("factor-base index out of range"))?;
        sum = curve.add(&sum, &point(member)?);
    }
    if sum != target {
        return Err(fail("four-point relation fails independent group replay"));
    }
    let online_ms = number(&result, "target_ms")?;
    let phases = [
        "target_query_ms",
        "target_pdp_ms",
        "target_relation_check_ms",
        "target_descent_ms",
        "target_recovery_check_ms",
    ];
    let phase_sum: f64 = phases
        .iter()
        .map(|key| number(&result, key))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .sum();
    if online_ms <= 0.0 || (phase_sum - online_ms).abs() > 1.0 {
        return Err(fail("exclusive online phase sum differs"));
    }
    let timing = &summary["timing_ms"];
    let cold_ms = number(timing, "process_total")?;
    if cold_ms < online_ms {
        return Err(fail("cold process time is below online time"));
    }
    Ok(json!({"schema":"n83.native-bridge-ic-replay/v1",
              "status":"PASS_INDEPENDENT_REPLAY", "k":k,
              "ordered_base_dump_sha256":dump_sha,
              "base_hash":expected_base_hash,
              "target_online_ms":online_ms,"cold_process_ms":cold_ms,
              "target_phase_sum_ms":phase_sum,
              "rank":k,"relation_indices":indices}))
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        return Err(fail(
            "usage: n83_bridge_verify <K> <point.jsonl> <scalar> <run-folder>",
        ));
    }
    let result = verify(
        args[1].parse()?,
        Path::new(&args[2]),
        &args[3],
        Path::new(&args[4]),
    )?;
    println!("{result}");
    Ok(())
}
