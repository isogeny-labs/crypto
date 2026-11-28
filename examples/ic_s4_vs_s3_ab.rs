//! S₄ 3-decomp vs S₃ 2-decomp: p=271 fb sweep + bench-16bit spot-check.
//! Also A/B `IC_S4_FB_PROBE=0` (full roots) vs default fb-x probe.
//!
//!   TMPDIR=/Volumes/SSD990-2/tmp CARGO_TARGET_DIR=/Volumes/SSD990-2/cargo-target-perf-r9 \
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

fn bench16() -> CurveParams {
    CurveParams {
        name: "bench-16bit",
        p: BigUint::from(65353u32),
        a: BigUint::from(1u32),
        b: BigUint::from(16u32),
        gx: BigUint::from(1u32),
        gy: BigUint::from(15165u32),
        n: BigUint::from(65003u32),
        h: 1,
    }
}

fn med(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn time_s3(curve: &CurveParams, g: &crypto_lib::ecc::point::Point, q: &crypto_lib::ecc::point::Point, fb: usize, seeds: u64) -> f64 {
    let t0 = Instant::now();
    for seed in 0..seeds {
        let _ = black_box(ec_index_calculus_dlp_staged_seeded(
            curve, g, q, fb, 0, 8000, 128, seed + 11,
        ));
    }
    t0.elapsed().as_nanos() as f64 / seeds as f64
}

fn time_s4(curve: &CurveParams, g: &crypto_lib::ecc::point::Point, q: &crypto_lib::ecc::point::Point, fb: usize, reps: u64) -> f64 {
    let t0 = Instant::now();
    for _ in 0..reps {
        let _ = black_box(ec_index_calculus_dlp_s4_staged(
            curve, g, q, fb, 2, 8000, 128,
        ));
    }
    t0.elapsed().as_nanos() as f64 / reps as f64
}

fn main() {
    let curve = tiny();
    let a = curve.a_fe();
    let g = curve.generator();
    let x = BigUint::from(190u32);
    let q = g.scalar_mul_vartime(&x, &a);

    // Correctness: probe vs roots agree on known-answer.
    unsafe {
        std::env::remove_var("IC_S4_FB_PROBE");
        std::env::set_var("IC_SUM_TABLE", "0");
    }
    let (x3, _) = ec_index_calculus_dlp_staged_seeded(&curve, &g, &q, 8, 0, 5000, 64, 7)
        .expect("s3");
    assert_eq!(x3, x);
    let (x4, _) = ec_index_calculus_dlp_s4_staged(&curve, &g, &q, 10, 2, 5000, 64).expect("s4");
    assert_eq!(x4, x);
    unsafe {
        std::env::set_var("IC_S4_FB_PROBE", "0");
    }
    let (x4b, _) = ec_index_calculus_dlp_s4_staged(&curve, &g, &q, 10, 2, 5000, 64).expect("s4-roots");
    assert_eq!(x4b, x);
    unsafe {
        std::env::remove_var("IC_S4_FB_PROBE");
    }
    println!("correctness: ok");

    // S4 probe vs full-roots micro (same fb).
    let mut wp = Vec::new();
    let mut wr = Vec::new();
    for r in 0..7 {
        for name in if r % 2 == 0 {
            ["probe", "roots"]
        } else {
            ["roots", "probe"]
        } {
            unsafe {
                if name == "probe" {
                    std::env::remove_var("IC_S4_FB_PROBE");
                } else {
                    std::env::set_var("IC_S4_FB_PROBE", "0");
                }
            }
            let _ = black_box(ec_index_calculus_dlp_s4_staged(
                &curve, &g, &q, 10, 2, 5000, 64,
            ));
            let ns = time_s4(&curve, &g, &q, 10, 6);
            if name == "probe" {
                wp.push(ns);
            } else {
                wr.push(ns);
            }
        }
    }
    let mp = med(wp);
    let mr = med(wr);
    println!("S₄ fb-probe vs roots() on p=271 fb≈10");
    println!("  probe (default) : med {mp:.0} ns/solve");
    println!("  roots (opt-out) : med {mr:.0} ns/solve");
    println!("  speedup roots/probe : {:.3}x", mr / mp);

    // E2E S3 (default table) vs S3 sweep vs S4 across fb on p=271.
    println!("\nS₃ vs S₄ e2e on p=271 (ratio S₄/S₃; <1 => S₄ wins)");
    println!(
        "{:>4} {:>12} {:>12} {:>12} {:>8} {:>8}",
        "fb", "s3_table", "s3_sweep", "s4_probe", "s4/tab", "s4/swp"
    );
    unsafe {
        std::env::remove_var("IC_S4_FB_PROBE");
    }
    for fb in [6usize, 8, 10, 12, 16, 20] {
        let mut wt = Vec::new();
        let mut ws = Vec::new();
        let mut w4 = Vec::new();
        for r in 0..5 {
            let order = if r % 2 == 0 {
                ["tab", "swp", "s4"]
            } else {
                ["s4", "swp", "tab"]
            };
            for name in order {
                match name {
                    "tab" => {
                        unsafe {
                            std::env::remove_var("IC_SUM_TABLE"); // default table for small fb
                        }
                        let _ = time_s3(&curve, &g, &q, fb, 2);
                        wt.push(time_s3(&curve, &g, &q, fb, 8));
                    }
                    "swp" => {
                        unsafe {
                            std::env::set_var("IC_SUM_TABLE", "0");
                        }
                        let _ = time_s3(&curve, &g, &q, fb, 2);
                        ws.push(time_s3(&curve, &g, &q, fb, 8));
                    }
                    _ => {
                        let _ = time_s4(&curve, &g, &q, fb, 1);
                        w4.push(time_s4(&curve, &g, &q, fb.max(8), 4));
                    }
                }
            }
        }
        let mt = med(wt);
        let ms = med(ws);
        let m4 = med(w4);
        println!(
            "{fb:>4} {mt:>12.0} {ms:>12.0} {m4:>12.0} {:>8.2} {:>8.2}",
            m4 / mt,
            m4 / ms
        );
    }

    // Larger public fixture: bench-16bit — relation e2e is heavy for S4;
    // report a few fb points with fewer reps.
    let c16 = bench16();
    let a16 = c16.a_fe();
    let g16 = c16.generator();
    let x16 = BigUint::from(12345u32);
    let q16 = g16.scalar_mul_vartime(&x16, &a16);
    println!("\nS₃ vs S₄ e2e on bench-16bit p=65353 (spot)");
    println!(
        "{:>4} {:>12} {:>12} {:>8}",
        "fb", "s3_table", "s4_probe", "s4/tab"
    );
    for fb in [8usize, 12, 16] {
        let mut wt = Vec::new();
        let mut w4 = Vec::new();
        for r in 0..3 {
            if r % 2 == 0 {
                unsafe {
                    std::env::remove_var("IC_SUM_TABLE");
                }
                let _ = time_s3(&c16, &g16, &q16, fb, 1);
                wt.push(time_s3(&c16, &g16, &q16, fb, 2));
                let _ = time_s4(&c16, &g16, &q16, fb, 1);
                w4.push(time_s4(&c16, &g16, &q16, fb, 2));
            } else {
                let _ = time_s4(&c16, &g16, &q16, fb, 1);
                w4.push(time_s4(&c16, &g16, &q16, fb, 2));
                unsafe {
                    std::env::remove_var("IC_SUM_TABLE");
                }
                let _ = time_s3(&c16, &g16, &q16, fb, 1);
                wt.push(time_s3(&c16, &g16, &q16, fb, 2));
            }
        }
        let mt = med(wt);
        let m4 = med(w4);
        println!("{fb:>4} {mt:>12.0} {m4:>12.0} {:>8.2}", m4 / mt);
    }
}
