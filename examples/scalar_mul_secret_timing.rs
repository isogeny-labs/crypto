//! Timing driver for `research/scalar_mul_ct_timing_20261022` (PROTOCOL.md):
//! wall time of `ecc::ct::scalar_mul_secret(G, k, curve)` against the bit
//! length of the secret `k`, on the curves that take the generic affine
//! ladder (SM2 and the GOST R 34.10-2012 256-bit test curve).
//!
//! Uses only API present before the fix, so the same source builds on both
//! arms.  Prints one JSON line: per curve, per scalar, the scalar's length,
//! every repetition's nanoseconds and a digest of the result (the pinned
//! output, which must agree between arms).
//!
//! ```sh
//! cargo run --release --example scalar_mul_secret_timing -- <label> [reps]
//! ```

use std::time::Instant;

use crypto_lib::ecc::ct::scalar_mul_secret;
use crypto_lib::ecc::point::Point;
use crypto_lib::ecc::CurveParams;
use crypto_lib::hash::sha256;
use num_bigint::BigUint;
use serde_json::json;

/// Leading zero bits below the order's length, one class per entry.
const LEADING_ZEROS: [usize; 8] = [0, 8, 16, 32, 64, 128, 192, 224];
/// Scalars per class.
const PER_CLASS: usize = 8;
/// Frozen scalar seed (xorshift64).
const SEED: u64 = 0x5ca1_ab1e_7111_e5ed;

fn xorshift(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

/// A scalar of exactly `bits` bits (top bit set), from the stream.
fn scalar_of_bits(s: &mut u64, bits: usize) -> BigUint {
    let words: Vec<u64> = (0..bits.div_ceil(64)).map(|_| xorshift(s)).collect();
    let mut k = BigUint::from_slice(
        &words
            .iter()
            .flat_map(|w| [*w as u32, (*w >> 32) as u32])
            .collect::<Vec<_>>(),
    );
    k &= (BigUint::from(1u32) << bits) - 1u32;
    k | (BigUint::from(1u32) << (bits - 1))
}

fn digest(p: &Point) -> String {
    let bytes = match p {
        Point::Infinity => b"infinity".to_vec(),
        Point::Affine { x, y } => [x.value.to_bytes_be(), y.value.to_bytes_be()].concat(),
    };
    sha256(&bytes)[..8]
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn main() {
    let mut args = std::env::args().skip(1);
    let label = args.next().expect("label");
    let reps: usize = args.next().map_or(5, |r| r.parse().expect("reps"));
    let mut curves_out = Vec::new();
    for curve in [CurveParams::sm2(), CurveParams::gost_3410_2012_256_test()] {
        let g = curve.generator();
        let order_bits = curve.order_bits();
        let mut s = SEED;
        let mut scalars = Vec::new();
        for &z in &LEADING_ZEROS {
            for i in 0..PER_CLASS {
                scalars.push((z, i, scalar_of_bits(&mut s, order_bits - z)));
            }
        }
        let mut ns = vec![Vec::with_capacity(reps); scalars.len()];
        let mut results = vec![String::new(); scalars.len()];
        // Each repetition visits every scalar, starting one class later than
        // the previous repetition, so no class always runs first.
        for rep in 0..reps {
            for j in 0..scalars.len() {
                let at = (j + rep * PER_CLASS) % scalars.len();
                let k = &scalars[at].2;
                let t = Instant::now();
                let out = scalar_mul_secret(&g, k, &curve);
                ns[at].push(t.elapsed().as_nanos() as u64);
                let d = digest(&out);
                assert!(
                    results[at].is_empty() || results[at] == d,
                    "unstable result"
                );
                results[at] = d;
            }
        }
        let rows: Vec<_> = scalars
            .iter()
            .zip(ns.iter().zip(&results))
            .map(|((z, i, k), (ns, d))| {
                json!({"leading_zeros": z, "scalar_bits": k.bits(), "idx": i, "ns": ns, "result": d})
            })
            .collect();
        curves_out.push(json!({"curve": curve.name, "order_bits": order_bits, "rows": rows}));
    }
    println!(
        "{}",
        json!({"label": label, "schema": "scalar-mul-secret-timing/v1", "reps": reps, "curves": curves_out})
    );
}
