//! Go-style benchmarks for representative `crypto_lib` primitives.
//!
//! Every line is in the Go benchmark format, so the output can be compared
//! with `benchstat` against C, Go or another Rust implementation:
//!
//! ```sh
//! cargo bench --bench go_bench -- -benchtime 100ms -count 5 | tee new.txt
//! benchstat old.txt new.txt
//! ```
//!
//! The harness auto-scales `b.N` until a run takes `-benchtime` (default
//! 1 s) and reports `ns/op`; `-bench SHA256/*` selects a subset and
//! `-benchtime 1000x` fixes the iteration count for deterministic work
//! measurements.  See `src/benchfmt.rs`.
//!
//! These are microbenchmarks of the from-scratch implementations, not a
//! claim about optimised production code: they exist to catch regressions
//! and to put our numbers in the same table as a reference implementation.

use crypto_lib::benchfmt::Harness;
use crypto_lib::ecc::p256_field::P256FieldElement;
use crypto_lib::hash::sha256::{sha256, Sha256};
use crypto_lib::hash::sha512::sha512;
use crypto_lib::symmetric::aes::{encrypt_block, AesKey};
use crypto_lib::symmetric::chacha20::chacha20_block;
use std::hint::black_box;

fn main() {
    let mut h = Harness::from_env()
        .expect("bad benchmark flags")
        .with_package("crypto_lib");

    // ── Hashes ───────────────────────────────────────────────────────────
    for len in [64usize, 1024, 16 * 1024] {
        let msg = vec![0xa5u8; len];
        h.bench_bytes(&format!("SHA256/{len}B"), len as u64, || {
            black_box(sha256(black_box(&msg)));
        })
        .unwrap();
        h.bench_bytes(&format!("SHA512/{len}B"), len as u64, || {
            black_box(sha512(black_box(&msg)));
        })
        .unwrap();
    }

    // Streaming SHA-256: four 256-byte updates and a finalize per iteration.
    let chunk = [0x5au8; 256];
    h.bench_bytes("SHA256/stream/1KiB", 1024, || {
        let mut st = Sha256::new();
        for _ in 0..4 {
            st.update(black_box(&chunk));
        }
        black_box(st.finalize());
    })
    .unwrap();

    // ── Symmetric primitives ─────────────────────────────────────────────
    let aes_key = AesKey::new(&[0x42u8; 16]).unwrap();
    let block = [0x11u8; 16];
    h.bench_bytes("AES128/encrypt_block", 16, || {
        black_box(encrypt_block(black_box(&block), &aes_key));
    })
    .unwrap();

    let chacha_key = [0x24u8; 32];
    let nonce = [0x00u8; 12];
    h.bench_bytes("ChaCha20/block", 64, || {
        black_box(chacha20_block(
            black_box(&chacha_key),
            black_box(&nonce),
            black_box(0),
        ));
    })
    .unwrap();

    // ── P-256 field arithmetic ───────────────────────────────────────────
    let a = P256FieldElement::from_bytes_be(&[3u8; 32]);
    let b = P256FieldElement::from_bytes_be(&[5u8; 32]);
    h.bench("P256/field/mul", || {
        black_box(a.mul(black_box(&b)));
    })
    .unwrap();
    h.bench("P256/field/sqr", || {
        black_box(a.sqr());
    })
    .unwrap();
    // Fermat inversion is a full 256-bit ladder; the harness will pick a
    // small b.N, which is exactly what `testing.B` does for slow benches.
    h.bench("P256/field/inv", || {
        black_box(a.inv());
    })
    .unwrap();

    h.flush().unwrap();
}
