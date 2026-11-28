//! Independent generic-field replay of the N83 K2048 m5 planted controls.
use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use flate2::read::GzDecoder;
use num_bigint::BigUint;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const CURVE: &str = "icv1-f2m83-tm6151469093347-debefd74";
const ORDER: &str = "2417851639230796216685689";
const U_SHA256: &str = "aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a";
const D_SHA256: &str = "2406a7105bff162fc80e448d441709cc9f725e36da9751a2e9feb6f3e23512c7";
const PLANTED_BINARY_SHA256: &str =
    "c2143a444886b7d87e8af67b5c211c6a68ed1b2a81c6af6b9a55c61aee9fe004";
const FULLCHAIN_BINARY_SHA256: &str =
    "d6e2edb0588e909630a36d362ccd14c6083ef0c660eebab4c3a388d37cddd980";
const INDICES: [usize; 5] = [0, 17, 53, 97, 211];

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn decimal(value: &Value) -> Result<BigUint> {
    let s = value.as_str().ok_or("coordinate is not a string")?;
    let n = BigUint::parse_bytes(s.as_bytes(), 10).ok_or("coordinate is not decimal")?;
    if n.bits() > 83 || n.to_string() != s {
        return Err("noncanonical coordinate".into());
    }
    Ok(n)
}

fn point(value: &Value) -> Result<BinaryPoint> {
    let pair = value.as_array().ok_or("point is not a pair")?;
    if pair.len() != 2 {
        return Err("point width differs".into());
    }
    Ok(BinaryPoint::Affine {
        x: F2mElement::from_biguint(&decimal(&pair[0])?, 83),
        y: F2mElement::from_biguint(&decimal(&pair[1])?, 83),
    })
}

fn coords(value: &BinaryPoint) -> Result<Value> {
    let BinaryPoint::Affine { x, y } = value else {
        return Err("sum is infinity".into());
    };
    Ok(json!([
        x.to_biguint().to_string(),
        y.to_biguint().to_string()
    ]))
}

fn header(path: &Path, expected_hash: &str, policy: &str) -> Result<Value> {
    let compressed = fs::read(path)?;
    if sha256(&compressed) != expected_hash {
        return Err("compressed base digest differs".into());
    }
    let mut reader = BufReader::new(GzDecoder::new(&compressed[..]));
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let header: Value = serde_json::from_str(&line)?;
    if header["curve_slug"] != CURVE
        || header["policy"] != policy
        || header["closure"] != "signed_frobenius"
        || header["orbit_columns"] != 2048
        || header["point_count"] != 339_968
        || header["representatives"].as_array().map(Vec::len) != Some(2048)
    {
        return Err("base header differs".into());
    }
    Ok(header)
}

fn verify_one(
    kc: &KoblitzCurve,
    base: &Value,
    arm: &str,
    policy: &str,
    object_sha256: &str,
    path: &Path,
    fullchain: bool,
) -> Result<Value> {
    let bytes = fs::read(path)?;
    let receipt: Value = serde_json::from_slice(&bytes)?;
    let (schema, binary_hash, source_hash) = if fullchain {
        (
            "n83.k2048-m5-fullchain/v1",
            FULLCHAIN_BINARY_SHA256,
            blake3::hash(include_bytes!("n83_k2048_m5_fullchain.rs"))
                .to_hex()
                .to_string(),
        )
    } else {
        (
            "n83.k2048-m5-planted/v1",
            PLANTED_BINARY_SHA256,
            blake3::hash(include_bytes!("n83_k2048_m5_planted.rs"))
                .to_hex()
                .to_string(),
        )
    };
    if receipt["schema"] != schema
        || receipt["arm"] != arm
        || receipt["curve_id"] != "EC1N83Ckb0h252dacc102a4"
        || receipt["policy"] != policy
        || receipt["object_sha256"] != object_sha256
        || receipt["source_blake3"] != source_hash
        || receipt["binary_sha256"] != binary_hash
        || receipt["indices"] != json!(INDICES)
        || receipt["group_sum_verified"] != true
        || receipt["subgroup_verified"] != true
        || receipt["unpinned_variables"] != 111_478
        || receipt["unpinned_clauses"] != 1_140_557
        || receipt["unpinned_xor_rows"] != 4_980
        || receipt["root_status"] != "Fixpoint"
        || receipt["factor_base_selection"] != Value::Null
        || receipt["single_target_online_speedup"] != Value::Null
    {
        return Err("planted receipt identity or model contract differs".into());
    }
    if fullchain {
        if receipt["solve_status"] != "Sat"
            || receipt["model_verified"] != true
            || receipt["conflicts"].as_u64().is_none_or(|n| n >= 5_000)
        {
            return Err("full-chain SAT control differs".into());
        }
    } else if receipt["solve_status"] != "Unknown"
        || receipt["model_verified"] != false
        || receipt["conflicts"] != 5_000
        || receipt["pinned_chain_x"] != Value::Null
    {
        return Err("summand-only cap status differs".into());
    }
    let summands = receipt["summands"].as_array().ok_or("summands absent")?;
    let pinned_x = receipt["pinned_summand_x"]
        .as_array()
        .ok_or("pinned summand x absent")?;
    if summands.len() != 5 || pinned_x.len() != 5 {
        return Err("summand count differs".into());
    }
    let reps = base["representatives"]
        .as_array()
        .ok_or("representatives absent")?;
    let mut sum = BinaryPoint::Infinity;
    let mut chain_x = Vec::new();
    for (position, index) in INDICES.iter().enumerate() {
        let rep = reps[*index]
            .as_array()
            .ok_or("representative shape differs")?;
        let p = point(&summands[position])?;
        let BinaryPoint::Affine { x, .. } = &p else {
            return Err("summand is infinity".into());
        };
        let x_decimal = x.to_biguint().to_string();
        if decimal(&rep[0])?.to_string() != x_decimal
            || pinned_x[position] != x_decimal
            || !kc.curve.is_on_curve(&p)
            || kc.mul(&p, &kc.subgroup_order) != BinaryPoint::Infinity
        {
            return Err("summand membership or subgroup replay failed".into());
        }
        sum = kc.add(&sum, &p);
        if (1..=3).contains(&position) {
            let BinaryPoint::Affine { x, .. } = &sum else {
                return Err("intermediate sum is infinity".into());
            };
            chain_x.push(x.to_biguint().to_string());
        }
    }
    if coords(&sum)? != receipt["target"]
        || kc.mul(&sum, &kc.subgroup_order) != BinaryPoint::Infinity
    {
        return Err("target group sum or subgroup replay failed".into());
    }
    if fullchain && receipt["pinned_chain_x"] != json!(chain_x) {
        return Err("intermediate chain replay failed".into());
    }
    Ok(json!({
        "arm":arm,"mode":if fullchain {"fullchain"} else {"summand_only"},
        "receipt_sha256":sha256(&bytes),"target":receipt["target"],
        "solve_status":receipt["solve_status"],"conflicts":receipt["conflicts"],
        "group_sum_replayed":true,"subgroup_replayed":true,
        "intermediate_chain_replayed":fullchain
    }))
}

fn run() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 8 {
        return Err("usage: verifier U_OBJECT D_OBJECT U_SUM D_SUM U_FULL D_FULL OUTPUT".into());
    }
    let output = Path::new(&args[7]);
    if output.exists() {
        return Err("output already exists".into());
    }
    let kc = KoblitzCurve::known_n83_k0().ok_or("N83 curve unavailable")?;
    if kc.label() != CURVE || kc.subgroup_order.to_string() != ORDER {
        return Err("curve identity differs".into());
    }
    let u = header(Path::new(&args[1]), U_SHA256, "public_x_hash")?;
    let d = header(
        Path::new(&args[2]),
        D_SHA256,
        "direct_subgroup_x_d18_prefix2048",
    )?;
    let rows = vec![
        verify_one(
            &kc,
            &u,
            "U",
            "public_x_hash",
            U_SHA256,
            Path::new(&args[3]),
            false,
        )?,
        verify_one(
            &kc,
            &d,
            "D",
            "direct_subgroup_x_d18_prefix2048",
            D_SHA256,
            Path::new(&args[4]),
            false,
        )?,
        verify_one(
            &kc,
            &u,
            "U",
            "public_x_hash",
            U_SHA256,
            Path::new(&args[5]),
            true,
        )?,
        verify_one(
            &kc,
            &d,
            "D",
            "direct_subgroup_x_d18_prefix2048",
            D_SHA256,
            Path::new(&args[6]),
            true,
        )?,
    ];
    let result = json!({
        "schema":"n83.k2048-m5-planted-audit/v1","status":"PASS",
        "curve_id":"EC1N83Ckb0h252dacc102a4","rows":rows,
        "factor_base_selection":Value::Null,"single_target_online_speedup":Value::Null,
        "verifier_source_blake3":blake3::hash(include_bytes!("n83_k2048_m5_planted_verify.rs")).to_hex().to_string(),
        "verifier_binary_sha256":sha256(&fs::read(std::env::current_exe()?)?)
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
        eprintln!("N83 K2048 planted verifier: {error}");
        std::process::exit(1);
    }
}
