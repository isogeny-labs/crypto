//! Multi-word `mont_vt` Jacobian ladder vs BigUint (`IC_MONT_VT=0`) on secp128r1.
//!
//!   TMPDIR=/Volumes/SSD990-2/tmp CARGO_TARGET_DIR=/Volumes/SSD990-2/cargo-target-perf \
//!   /usr/sbin/taskpolicy -a cargo run --release --example mont_vt_scalar_ab
use crypto_lib::ecc::curve::CurveParams;
use num_bigint::BigUint;
use std::hint::black_box;
use std::time::Instant;

fn med(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn main() {
    let curve = CurveParams::secp128r1();
    let a = curve.a_fe();
    let g = curve.generator();
    let mut ks = Vec::new();
    let mut k = BigUint::from(0x1234_5678u64);
    for _ in 0..64 {
        k = (&k * &k + 0x9e37_79b9u32) % &curve.n;
        if k.bits() > 8 {
            ks.push(k.clone());
        }
    }
    // Correctness: mont default matches BigUint.
    unsafe {
        std::env::set_var("IC_MONT_VT", "0");
    }
    let big_pts: Vec<_> = ks
        .iter()
        .map(|k| g.scalar_mul(k, &a))
        .collect();
    unsafe {
        std::env::remove_var("IC_MONT_VT");
    }
    for (k, want) in ks.iter().zip(&big_pts) {
        assert_eq!(&g.scalar_mul(k, &a), want, "mont != biguint k={k}");
    }
    println!("correctness: ok ({} scalars)", ks.len());

    let iters = 200usize;
    let mut wall_mont = Vec::new();
    let mut wall_big = Vec::new();
    for r in 0..7 {
        let order = if r % 2 == 0 {
            ["mont", "big"]
        } else {
            ["big", "mont"]
        };
        for name in order {
            unsafe {
                if name == "mont" {
                    std::env::remove_var("IC_MONT_VT");
                } else {
                    std::env::set_var("IC_MONT_VT", "0");
                }
            }
            for k in ks.iter().take(8) {
                let _ = black_box(g.scalar_mul(k, &a));
            }
            let t0 = Instant::now();
            for _ in 0..iters {
                for k in &ks {
                    let _ = black_box(g.scalar_mul(k, &a));
                }
            }
            let ns = t0.elapsed().as_nanos() as f64 / (iters * ks.len()) as f64;
            if name == "mont" {
                wall_mont.push(ns);
            } else {
                wall_big.push(ns);
            }
        }
    }
    let wm = med(wall_mont);
    let wb = med(wall_big);
    println!("secp128r1 scalar_mul: mont_vt vs BigUint Jacobian");
    println!("  mont_vt (default) : med {wm:.0} ns/op");
    println!("  BigUint (IC_MONT_VT=0): med {wb:.0} ns/op");
    println!("  speedup big/mont  : {:.3}x", wb / wm);
}
