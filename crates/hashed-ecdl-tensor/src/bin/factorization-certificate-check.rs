//! Verify one serialized recursive-factorization certificate.

use hashed_ecdl_tensor::factorization_certificate::{
    description_envelope, verify, verify_source_dimensions, Certificate, SourceDimensionWitness,
};
use hashed_ecdl_tensor::{Limits, Meter};
use serde_json::json;
use std::io::Read;

const MAX_CERTIFICATE_BYTES: u64 = 64 * 1024 * 1024;

fn read_input(path: Option<&str>) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    match path {
        Some(path) => {
            let metadata = std::fs::metadata(path).map_err(|error| error.to_string())?;
            if metadata.len() > MAX_CERTIFICATE_BYTES {
                return Err("certificate exceeds 64 MiB input limit".into());
            }
            bytes = std::fs::read(path).map_err(|error| error.to_string())?;
        }
        None => {
            std::io::stdin()
                .take(MAX_CERTIFICATE_BYTES + 1)
                .read_to_end(&mut bytes)
                .map_err(|error| error.to_string())?;
            if bytes.len() as u64 > MAX_CERTIFICATE_BYTES {
                return Err("certificate exceeds 64 MiB input limit".into());
            }
        }
    }
    Ok(bytes)
}

fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let parse_source =
        |a: &str, u: &str, coordinate: &str| -> Result<SourceDimensionWitness, String> {
            Ok(SourceDimensionWitness {
                a: a.parse::<u32>().map_err(|error| format!("a: {error}"))?,
                u: u.parse::<u32>().map_err(|error| format!("u: {error}"))?,
                coordinate: coordinate
                    .parse::<u32>()
                    .map_err(|error| format!("coordinate: {error}"))?,
                require_normalized_precision: true,
            })
        };
    let (path, source) = match args.as_slice() {
        [] => (None, None),
        [path] => (Some(path.as_str()), None),
        [flag, a, u, coordinate] if flag == "--source" => {
            (None, Some(parse_source(a, u, coordinate)?))
        }
        [path, flag, a, u, coordinate] if flag == "--source" => {
            (Some(path.as_str()), Some(parse_source(a, u, coordinate)?))
        }
        _ => {
            return Err(
                "usage: factorization-certificate-check [CERTIFICATE.json] [--source A U I]".into(),
            )
        }
    };
    let bytes = read_input(path)?;
    let certificate: Certificate =
        serde_json::from_slice(&bytes).map_err(|error| format!("certificate JSON: {error}"))?;
    let mut meter = Meter::new(Limits::default());
    verify(&certificate, &mut meter)?;
    let description = description_envelope(&certificate)?;
    let source_dimensions = source
        .as_ref()
        .map(|source| verify_source_dimensions(&certificate, source))
        .transpose()?;
    let report = json!({
        "schema": "hashed-ecdl-factorization-certificate-check/v1",
        "valid": true,
        "input_bytes": bytes.len(),
        "charged_steps": meter.used,
        "description_envelope": description,
        "source_dimensions": source_dimensions,
        "scope": "finite certificate verification using exact ring arithmetic; not a theorem-assistant proof or practical Curve25519 attack"
    });
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("factorization-certificate-check: {error}");
        std::process::exit(2);
    }
}
