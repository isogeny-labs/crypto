//! Deterministic public AES-128 round-9 single-byte fault-pair corpus.
//! Run `cargo run --release --example aes_dfa_public_fixture > pairs.json`.

use crypto_lib::cryptanalysis::aes::{dfa::inject_fault_round_9, ReducedAes128};
use crypto_lib::utils::encoding::to_hex;
use serde_json::json;

fn main() {
    let master: [u8; 16] = std::array::from_fn(|i| i as u8);
    let cipher = ReducedAes128::new(&master, 10, false);
    let mut pairs = Vec::new();
    for column in 0..4 {
        for (trial, fault) in [0x11, 0x37, 0x83, 0xe5].into_iter().enumerate() {
            let plaintext: [u8; 16] = std::array::from_fn(|i| {
                (column as u8).wrapping_mul(53)
                    ^ (trial as u8).wrapping_mul(29)
                    ^ (i as u8).wrapping_mul(17)
            });
            let (correct_ciphertext, faulted_ciphertext) =
                inject_fault_round_9(&cipher, &plaintext, column * 4 + trial, fault);
            pairs.push(json!({
                "plaintext_hex": to_hex(&plaintext),
                "correct_ciphertext_hex": to_hex(&correct_ciphertext),
                "faulted_ciphertext_hex": to_hex(&faulted_ciphertext),
            }));
        }
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({ "pairs": pairs })).unwrap()
    );
}
