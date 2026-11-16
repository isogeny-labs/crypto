use crypto_lib::cryptanalysis::aes::{dpa, ReducedAes128};
use crypto_lib::utils::encoding::to_hex;
use std::process::Command;

#[test]
fn supplied_traces_recover_and_verify_full_aes128_key() {
    let key: [u8; 16] = [
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee,
        0xff,
    ];
    let (plaintexts, traces_per_byte) = dpa::generate_traces(512, &key, 0.0, 20261018);
    let ciphertext = ReducedAes128::new(&key, 10, false).encrypt(&plaintexts[0]);
    let input = serde_json::json!({
        "plaintexts": plaintexts.iter().map(|p| to_hex(p)).collect::<Vec<_>>(),
        "traces_per_byte": traces_per_byte,
    });
    let path = std::env::temp_dir().join(format!(
        "crypto-aes-cpa-{}-{}.json",
        std::process::id(),
        20261018
    ));
    std::fs::write(&path, serde_json::to_vec(&input).unwrap()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_crypto"))
        .args(["cryptanalysis", "aes-cpa", "--input"])
        .arg(&path)
        .args(["--known-ciphertext-hex", &to_hex(&ciphertext)])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["candidate_key_hex"], to_hex(&key));
    assert_eq!(report["status"], "complete");
    assert_eq!(report["verified"], true);
    assert_eq!(report["samples_per_byte"], 512);

    let mismatch = Command::new(env!("CARGO_BIN_EXE_crypto"))
        .args(["cryptanalysis", "aes-cpa", "--input"])
        .arg(&path)
        .args(["--known-ciphertext-hex", "00000000000000000000000000000000"])
        .output()
        .unwrap();
    assert!(!mismatch.status.success());
    let failed_report: serde_json::Value = serde_json::from_slice(&mismatch.stdout).unwrap();
    assert_eq!(failed_report["status"], "verification_failed");
    assert_eq!(failed_report["verified"], false);

    let candidate = Command::new(env!("CARGO_BIN_EXE_crypto"))
        .args(["cryptanalysis", "aes-cpa", "--input"])
        .arg(&path)
        .output()
        .unwrap();
    let _ = std::fs::remove_file(path);
    assert!(candidate.status.success());
    let candidate_report: serde_json::Value = serde_json::from_slice(&candidate.stdout).unwrap();
    assert_eq!(candidate_report["status"], "candidate");
    assert_eq!(candidate_report["verified"], serde_json::Value::Null);
}
