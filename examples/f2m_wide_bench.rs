//! Latency of `F2mElement` mul / square / inverse at the wide NIST binary
//! fields (B-283, B-409, B-571), with B-163 as a reference row.
//!
//! ```bash
//! cargo run --release --example f2m_wide_bench -- [reps]
//! ```
//!
//! Each op is a dependency chain (`a = a·b`, `a = a²`, `a = a⁻¹ + b`), as in
//! `f2m_field_bench`; the best of `reps` (default 5) passes is printed with a
//! checksum, so two builds can be compared for speed and for identical
//! answers in one table.

use crypto_lib::binary_ecc::{F2mElement, IrreduciblePoly};
use std::hint::black_box;
use std::time::Instant;

fn rand_elem(state: &mut u64, m: u32) -> F2mElement {
    let words: Vec<u64> = (0..m.div_ceil(64))
        .map(|_| {
            *state ^= *state << 13;
            *state ^= *state >> 7;
            *state ^= *state << 17;
            *state
        })
        .collect();
    F2mElement::from_words(&words, m)
}

fn time<F: FnMut() -> u64>(label: &str, reps: usize, iters: u64, mut f: F) {
    let mut best = f64::INFINITY;
    let mut chk = 0u64;
    for _ in 0..reps {
        let t = Instant::now();
        let mut acc = 0u64;
        for _ in 0..iters {
            acc ^= f();
        }
        best = best.min(t.elapsed().as_nanos() as f64 / iters as f64);
        chk ^= acc;
    }
    println!("{label:<24} {best:>10.1} ns/op   (chk {chk:016x})");
}

fn main() {
    let reps: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(5);
    let fields = [
        ("B-163", IrreduciblePoly::deg_163()),
        ("B-283", IrreduciblePoly::deg_283()),
        ("B-409", IrreduciblePoly::deg_409()),
        ("B-571", IrreduciblePoly::deg_571()),
    ];
    let mut s = 0x2545_F491_4F6C_DD1Du64;
    for (name, irr) in &fields {
        let m = irr.degree;
        let (a0, b0) = (rand_elem(&mut s, m), rand_elem(&mut s, m));
        let iters = 200_000;
        let mut a = a0.clone();
        time(&format!("{name} mul"), reps, iters, || {
            a = a.mul(black_box(&b0), irr);
            a.raw_bits()[0]
        });
        let mut a = a0.clone();
        time(&format!("{name} square"), reps, iters, || {
            a = a.square(irr);
            a.raw_bits()[0]
        });
        let mut a = a0.clone();
        time(&format!("{name} flt_inverse"), reps, iters / 100, || {
            a = a.flt_inverse(irr).unwrap().add(&b0);
            a.raw_bits()[0]
        });
    }
}
