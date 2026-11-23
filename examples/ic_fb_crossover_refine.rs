//! Refine sum-table vs WordSweep crossover across bit sizes (16/18/20).
//!
//!   TMPDIR=/Volumes/SSD990-2/tmp CARGO_TARGET_DIR=/Volumes/SSD990-2/cargo-target-perf \
//!   /usr/sbin/taskpolicy -a cargo run --release --example ic_fb_crossover_refine
use crypto_lib::cryptanalysis::ec_index_calculus::{
    build_factor_base, ec_index_calculus_dlp_staged_seeded,
};
use crypto_lib::ecc::curve::CurveParams;
use num_bigint::BigUint;
use std::hint::black_box;
use std::time::Instant;

fn c16() -> CurveParams {
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
fn c18() -> CurveParams {
    CurveParams {
        name: "bench-18bit",
        p: BigUint::from(261847u32),
        a: BigUint::from(3u32),
        b: BigUint::from(2u32),
        gx: BigUint::from(2u32),
        gy: BigUint::from(4u32),
        n: BigUint::from(261673u32),
        h: 1,
    }
}
fn c20() -> CurveParams {
    CurveParams {
        name: "bench-20bit",
        p: BigUint::from(1048291u32),
        a: BigUint::from(1u32),
        b: BigUint::from(11u32),
        gx: BigUint::from(1u32),
        gy: BigUint::from(123004u32),
        n: BigUint::from(1046999u32),
        h: 1,
    }
}

fn med(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn run(curve: &CurveParams, x: u32, fbs: &[usize], seeds: usize, rounds: usize) {
    let a = curve.a_fe();
    let g = curve.generator();
    let x_truth = BigUint::from(x);
    let q = g.scalar_mul(&x_truth, &a);
    println!(
        "== {} (p bits≈{}) ==",
        curve.name,
        curve.p.bits()
    );
    println!(
        "{:>4} {:>4} {:>12} {:>12} {:>8}  winner",
        "want", "got", "table_ns", "sweep_ns", "tab/swp"
    );
    for &want in fbs {
        let got = build_factor_base(curve, want).len();
        let mut wt = Vec::new();
        let mut ws = Vec::new();
        for r in 0..rounds {
            let order = if r % 2 == 0 {
                ["table", "sweep"]
            } else {
                ["sweep", "table"]
            };
            for name in order {
                unsafe {
                    std::env::set_var("IC_SUM_TABLE", if name == "table" { "1" } else { "0" });
                    std::env::remove_var("IC_FAST_SWEEP");
                }
                let t0 = Instant::now();
                let mut ok = 0usize;
                for seed in 0..seeds as u64 {
                    if let Some((xx, _)) = ec_index_calculus_dlp_staged_seeded(
                        curve,
                        &g,
                        &q,
                        want,
                        0,
                        100_000,
                        64,
                        seed + 11,
                    ) {
                        assert_eq!(&xx, &x_truth);
                        ok += 1;
                        black_box(xx);
                    }
                }
                let ns = t0.elapsed().as_nanos() as f64 / seeds.max(1) as f64;
                if ok == 0 {
                    eprintln!("  warn: {name} want={want} 0/{seeds}");
                }
                if name == "table" {
                    wt.push(ns);
                } else {
                    ws.push(ns);
                }
            }
        }
        let a = med(wt);
        let b = med(ws);
        let ratio = a / b;
        let winner = if ratio > 1.05 {
            "sweep"
        } else if ratio < 0.95 {
            "table"
        } else {
            "tie"
        };
        println!("{want:>4} {got:>4} {a:>12.0} {b:>12.0} {ratio:>8.3}  {winner}");
    }
}

fn main() {
    // Focus on the disputed band around / above 256.
    run(&c16(), 12345, &[256, 320, 384, 448, 512], 2, 3);
    run(&c18(), 54321, &[256, 384, 512, 640, 768], 2, 3);
    run(&c20(), 98765, &[256, 384, 512, 768], 1, 2);
}
