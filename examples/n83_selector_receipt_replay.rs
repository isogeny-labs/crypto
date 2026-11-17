//! Replay the public N83 selector-positive relation from a retained object
//! and worker receipt, without constructing or running the SAT model.
use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use flate2::read::GzDecoder;
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

const OBJECT: &str = "1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d";
const POINT_SET: &str = "564c6bb72b346b73112240bb0bddc4ef68b7525f7ef2556d6046e52c32f976e0";
const CURVE: &str = "icv1-f2m83-tm6151469093347-debefd74";
const INDICES: [usize; 4] = [0, 166, 332, 498];

fn decimal(value: &Value) -> Result<BigUint, String> {
    let text = value.as_str().ok_or("expected decimal string")?;
    BigUint::parse_bytes(text.as_bytes(), 10).ok_or_else(|| "invalid decimal integer".into())
}

fn x(point: &BinaryPoint) -> Result<F2mElement, String> {
    match point {
        BinaryPoint::Affine { x, .. } => Ok(x.clone()),
        BinaryPoint::Infinity => Err("expected affine point".into()),
    }
}

fn object_hash(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0u8; 1 << 20];
    loop {
        let len = file.read(&mut buffer)?;
        if len == 0 {
            break;
        }
        hasher.update(&buffer[..len]);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

fn replay(
    expected_source: &str,
    arm: &str,
    object: &Path,
    receipt: &Path,
) -> Result<Value, Box<dyn std::error::Error>> {
    let (schema, success) = match arm {
        "full" => (
            "n83.k16627-m4-selector-positive/v1",
            "PASS_SELECTOR_PINNED_WITNESS",
        ),
        "selector-only" => (
            "n83.k16627-m4-selector-only/v1",
            "PASS_SELECTOR_ONLY_WITNESS",
        ),
        "phase-only" => ("n83.k16627-m4-phase-only/v1", "PASS_PHASE_ONLY_WITNESS"),
        _ => return Err("unknown selector-control arm".into()),
    };
    let worker: Value = serde_json::from_reader(File::open(receipt)?)?;
    if worker["schema"] != schema
        || worker["status"] != success
        || worker["source_commit"] != expected_source
        || worker["source_attestation_mode"] != "supervised_snapshot"
        || worker["object_blake3"] != OBJECT
        || worker["point_set_blake3"] != POINT_SET
        || worker["point_indices"] != json!(INDICES)
        || worker["relation"]["point_indices"] != json!(INDICES)
        || worker["solver_models"] != 1
        || worker["relation"]["group_verified"] != true
        || worker["relation"]["matches_preflight_row"] != true
        || worker["relation"]["rank_after_insert"] != 1
        || worker["relation"]["tamper_rejected"] != true
    {
        return Err("worker identity or success fields differ".into());
    }
    if object_hash(object)? != OBJECT {
        return Err("compressed object BLAKE3 mismatch".into());
    }
    let curve = KoblitzCurve::known_n83_k0().ok_or("pinned primary curve unavailable")?;
    if curve.label() != CURVE {
        return Err("curve identity mismatch".into());
    }

    let reader = BufReader::new(GzDecoder::new(File::open(object)?));
    let mut points: Vec<BinaryPoint> = Vec::with_capacity(4);
    for (line_index, line) in reader.lines().enumerate() {
        if line_index == 0 {
            continue; // Object metadata precedes its point records.
        }
        let point_index = line_index - 1;
        if !INDICES.contains(&point_index) {
            if point_index > INDICES[3] {
                break;
            }
            continue;
        }
        let row: Value = serde_json::from_str(&line?)?;
        if row["column"] != points.len()
            || row["phase"] != 0
            || row["negative"] != false
            || row["coefficient"] != "1"
        {
            return Err("selected row is not the expected positive representative".into());
        }
        let coordinates = row["point"].as_array().ok_or("missing point coordinates")?;
        if coordinates.len() != 2 {
            return Err("wrong point coordinate count".into());
        }
        let point = BinaryPoint::Affine {
            x: F2mElement::from_biguint(&decimal(&coordinates[0])?, 83),
            y: F2mElement::from_biguint(&decimal(&coordinates[1])?, 83),
        };
        if !curve.curve.is_on_curve(&point)
            || curve.mul(&point, &curve.subgroup_order) != BinaryPoint::Infinity
        {
            return Err("stored representative fails curve or subgroup check".into());
        }
        points.push(point);
    }
    if points.len() != 4 {
        return Err("missing selected point records".into());
    }

    let receipt_x = worker["x_decimal"].as_array().ok_or("missing x blocks")?;
    if receipt_x.len() != 7 {
        return Err("wrong x block count".into());
    }
    let mut sum = BinaryPoint::Infinity;
    let mut group_x = Vec::with_capacity(7);
    let mut intermediates = Vec::with_capacity(4);
    for point in &points {
        group_x.push(x(point)?);
        sum = curve.add(&sum, point);
        intermediates.push(sum.clone());
    }
    group_x.push(x(&intermediates[1])?);
    group_x.push(x(&intermediates[2])?);
    group_x.push(x(&intermediates[3])?);
    for (reported, computed) in receipt_x.iter().zip(&group_x) {
        if decimal(reported)? != computed.to_biguint() {
            return Err("worker x block differs from independent group addition".into());
        }
    }
    let mut distinct = group_x[..4]
        .iter()
        .map(F2mElement::to_biguint)
        .collect::<Vec<_>>();
    distinct.sort_unstable();
    distinct.dedup();
    if distinct.len() != 4 {
        return Err("summand x values are not distinct".into());
    }
    if curve.add(&sum, curve.generator()) == sum {
        return Err("tampered target was not rejected".into());
    }

    // Directly evaluate all three original-field S3 identities, without
    // reading the SAT encoder or the worker's preflight calculation.
    let irr = &curve.curve.irreducible;
    let b = &curve.curve.b;
    for (a, c, output) in [(0, 1, 4), (4, 2, 5), (5, 3, 6)] {
        let sum_xy = group_x[a].add(&group_x[c]);
        let product = group_x[a].mul(&group_x[c], irr);
        let value = sum_xy
            .square(irr)
            .mul(&group_x[output].square(irr), irr)
            .add(&product.mul(&group_x[output], irr))
            .add(&product.square(irr))
            .add(b);
        if !value.is_zero() {
            return Err("original-field S3 identity failed".into());
        }
    }

    // Phase-zero positive representatives make the first four coefficients
    // one. Serialize the dense row exactly as the worker's relation format.
    let row: Vec<String> = (0..16_627)
        .map(|column| if column < 4 { "1" } else { "0" }.to_string())
        .collect();
    let row_hash = blake3::hash(&serde_json::to_vec(&row)?)
        .to_hex()
        .to_string();
    if worker["relation"]["dense_decimal_blake3"] != row_hash {
        return Err("relation row hash differs from independent phase-zero row".into());
    }
    let choices = worker["orbit_choice_pins"]
        .as_array()
        .ok_or("missing orbit pins")?;
    if choices.len() != 4 {
        return Err("wrong orbit-pin count".into());
    }
    for (slot, choice) in choices.iter().enumerate() {
        if choice["slot"] != slot
            || choice["phase"] != 0
            || choice["phase_wires"].as_array().map(Vec::len) != Some(7)
            || choice["selector_pinned"] != (arm != "phase-only")
            || choice["phase_pinned"] != (arm != "selector-only")
            || decimal(&choice["representative_x_decimal"])? != group_x[slot].to_biguint()
        {
            return Err("orbit pin differs from selected representative".into());
        }
    }
    let receipt_hash = blake3::hash(&fs::read(receipt)?).to_hex().to_string();
    Ok(json!({
        "schema":"n83.selector-relation-replay/v1",
        "status":"PASS_RELATION_REPLAY",
        "curve_id":CURVE,
        "source_commit":expected_source,
        "arm":arm,
        "object_blake3":OBJECT,
        "worker_receipt_blake3":receipt_hash,
        "point_indices":INDICES,
        "checks":["object_bytes", "curve_and_subgroup", "seven_group_x_blocks", "three_original_field_s3", "four_phase_zero_representatives", "dense_relation_row", "nonzero_rank_one_row", "tampered_target"],
        "solver_model_bits_replayed":false
    }))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 6
        || args[1].len() != 40
        || !args[1].bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        return Err(
            "usage: n83_selector_receipt_replay EXPECTED_SOURCE_COMMIT full|selector-only|phase-only OBJECT_JSONL_GZ WORKER_JSON NEW_OUTPUT_JSON".into(),
        );
    }
    let result = replay(&args[1], &args[2], Path::new(&args[3]), Path::new(&args[4]))?;
    let output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[5])?;
    serde_json::to_writer_pretty(output, &result)?;
    println!("{}", result);
    Ok(())
}
