//! Staged S₄ 3-decomp vs S₃ 2-decomp end-to-end on p=271.
//!
//!   TMPDIR=/Volumes/SSD990-2/tmp CARGO_TARGET_DIR=/Volumes/SSD990-2/cargo-target-perf \
//!   /usr/sbin/taskpolicy -a cargo run --release --example ic_s4_vs_s3_ab
use crypto_lib::cryptanalysis::ec_index_calculus::{
    ec_index_calculus_dlp_s4_staged, ec_index_calculus_dlp_staged_seeded,
};
use crypto_lib::ecc::curve::CurveParams;
use num_bigint::BigUint;
use std::hint::black_box;
use std::time::Instant;

fn tiny() -> CurveParams {
    CurveParams {
        name: "test-curve-271",
        p: BigUint::from(271u32),
        a: BigUint::from(1u32),
        b: BigUint::from(19u32),
        gx: BigUint::from(3u32),
        gy: BigUint::from(7u32),
        n: BigUint::from(281u32),
        h: 1,
    }
}

fn med(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn main() {
    unsafe {
        std::env::set_var("IC_SUM_TABLE", "0"); // force Semaev sweep path for S₃
    }
    let curve = tiny();
    let a = curve.a_fe();
    let g = curve.generator();
    let x = BigUint::from(190u32);
    let q = g.scalar_mul_vartime(&x, &a);

    // Correctness
    let (x3, _) = ec_index_calculus_dlp_staged_seeded(&curve, &g, &q, 8, 0, 5000, 64, 7)
        .expect("s3");
    assert_eq!(x3, x);
    let (x4, _) = ec_index_calculus_dlp_s4_staged(&curve, &g, &q, 10, 2, 5000, 64).expect("s4");
    assert_eq!(x4, x);
    println!("correctness: ok");

    let mut w3 = Vec::new();
    let mut w4 = Vec::new();
    for r in 0..7 {
        let order = if r % 2 == 0 {
            ["s3", "s4"]
        } else {
            ["s4", "s3"]
        };
        for name in order {
            if name == "s3" {
                for seed in 0..2u64 {
                    let _ = black_box(ec_index_calculus_dlp_staged_seeded(
                        &curve, &g, &q, 8, 0, 5000, 64, seed,
                    ));
                }
                let t0 = Instant::now();
                for seed in 0..16u64 {
                    let _ = black_box(ec_index_calculus_dlp_staged_seeded(
                        &curve, &g, &q, 8, 0, 5000, 64, seed + 20,
                    ));
                }
                w3.push(t0.elapsed().as_nanos() as f64 / 16.0);
            } else {
                for _ in 0..2 {
                    let _ = black_box(ec_index_calculus_dlp_s4_staged(
                        &curve, &g, &q, 10, 2, 5000, 64,
                    ));
                }
                let t0 = Instant::now();
                for _ in 0..8 {
                    let _ = black_box(ec_index_calculus_dlp_s4_staged(
                        &curve, &g, &q, 10, 2, 5000, 64,
                    ));
                }
                w4.push(t0.elapsed().as_nanos() as f64 / 8.0);
            }
        }
    }
    let m3 = med(w3);
    let m4 = med(w4);
    println!("IC staged S₃ vs S₄ (p=271, sweep forced for S₃)");
    println!("  S₃ 2-decomp : med {m3:.0} ns/solve");
    println!("  S₄ 3-decomp : med {m4:.0} ns/solve");
    println!("  ratio S₄/S₃ : {:.3}x  (<1 => S₄ faster end-to-end)", m4 / m3);
}
