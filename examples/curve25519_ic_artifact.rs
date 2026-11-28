//! Emit or replay the exact Curve25519 index-calculus model artifacts.
//!
//! ```text
//! cargo run --release --example curve25519_ic_artifact -- manifest
//! cargo run --release --example curve25519_ic_artifact -- certificate
//! cargo run --release --example curve25519_ic_artifact -- replay PATH
//! ```

use std::{env, fs};

use crypto_lib::cryptanalysis::{
    curve25519_ic::{montgomery_translation_identity, Curve25519IcModel},
    identity_certificate::{check_with_extra, issue, IdentityCertificate, Verdict},
};

fn certificate() -> IdentityCertificate {
    let cert = issue(
        montgomery_translation_identity(),
        0x2551_9c1c_2026_1102,
        32,
        "issued by examples/curve25519_ic_artifact.rs; replayed by the native checker with 32 additional domain-separated points",
        None,
    )
    .expect("the integer translation identity holds");
    assert!(matches!(
        check_with_extra(&cert, Some((0x2551_9c1c, 32))).expect("certificate is well formed"),
        Verdict::Accept { .. }
    ));
    cert
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let value = match args.as_slice() {
        [command] if command == "manifest" => Curve25519IcModel::rfc7748().evidence_manifest(),
        [command] if command == "certificate" => {
            serde_json::to_value(certificate()).expect("certificate serialises")
        }
        [command, path] if command == "replay" => {
            let bytes = fs::read(path).expect("certificate file is readable");
            let cert: IdentityCertificate =
                serde_json::from_slice(&bytes).expect("certificate parses");
            let verdict = check_with_extra(&cert, Some((0x2551_9c1c, 32)))
                .expect("certificate is well formed");
            serde_json::json!({
                "schema": "curve25519-index-calculus-identity-replay/v1",
                "certificate_id": cert.id,
                "verdict": format!("{verdict:?}")
            })
        }
        _ => panic!("usage: curve25519_ic_artifact manifest|certificate|replay PATH"),
    };
    println!("{}", serde_json::to_string_pretty(&value).unwrap());
}
