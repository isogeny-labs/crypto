//! Verify one serialized parametric-refinement certificate.

use hashed_ecdl_tensor::parametric_refinement_certificate::{verify, Certificate};
use serde_json::json;
use std::io::Read;

const MAX_CERTIFICATE_BYTES: u64 = 8 * 1024 * 1024;

fn read_input(path: Option<&str>) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    match path {
        Some(path) => {
            let metadata = std::fs::metadata(path).map_err(|error| error.to_string())?;
            if metadata.len() > MAX_CERTIFICATE_BYTES {
                return Err("certificate exceeds 8 MiB input limit".into());
            }
            bytes = std::fs::read(path).map_err(|error| error.to_string())?;
        }
        None => {
            std::io::stdin()
                .take(MAX_CERTIFICATE_BYTES + 1)
                .read_to_end(&mut bytes)
                .map_err(|error| error.to_string())?;
            if bytes.len() as u64 > MAX_CERTIFICATE_BYTES {
                return Err("certificate exceeds 8 MiB input limit".into());
            }
        }
    }
    Ok(bytes)
}

fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let path = match args.as_slice() {
        [] => None,
        [path] => Some(path.as_str()),
        _ => {
            return Err("usage: parametric-refinement-certificate-check [CERTIFICATE.json]".into())
        }
    };
    let bytes = read_input(path)?;
    let certificate: Certificate =
        serde_json::from_slice(&bytes).map_err(|error| format!("certificate JSON: {error}"))?;
    let summary = verify(&certificate)?;
    let report = json!({
        "schema": "hashed-ecdl-parametric-refinement-certificate-check/v2",
        "valid": true,
        "status": "conditional_parametric_implication",
        "input_bytes": bytes.len(),
        "summary": summary,
        "scope": "exact algebraic implication from explicit generic premises; not premise realization, an external theorem proof, or a practical Curve25519 attack"
    });
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("parametric-refinement-certificate-check: {error}");
        std::process::exit(2);
    }
}
