//! Sum-table vs WordSweep crossover across factor-base sizes (bench-16bit).
//! Prefer table while it wins; default threshold should track the crossover.
//!
//!   TMPDIR=/Volumes/SSD990-2/tmp CARGO_TARGET_DIR=/Volumes/SSD990-2/cargo-target-perf \
//!   /usr/sbin/taskpolicy -a cargo run --release --example ic_fb_crossover_ab
use crypto_lib::cryptanalysis::ec_index_calculus::{
    build_factor_base, build_pair_sum_table, ec_index_calculus_dlp_staged_seeded,
};
use crypto_lib::ecc::curve::CurveParams;
use num_bigint::BigUint;
use std::hint::black_box;
use std::time::Instant;

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

fn main() {
    let curve = bench16();
    let a_fe = curve.a_fe();
    let g = curve.generator();
    let x_truth = BigUint::from(12345u32);
    let q = g.scalar_mul(&x_truth, &a_fe);

    println!("IC staged table vs WordSweep crossover (bench-16bit, p=65353)");
    println!(
        "{:>4} {:>4} {:>10} {:>12} {:>12} {:>8}  winner",
        "want", "got", "build_ms", "table_ns", "sweep_ns", "tab/swp"
    );
    for want in [8usize, 16, 32, 48, 64, 96, 128, 192, 256, 384] {
        let fb = build_factor_base(&curve, want);
        let got = fb.len();
        let t_build = Instant::now();
        let _ = black_box(build_pair_sum_table(&curve, &fb));
        let build_ms = t_build.elapsed().as_secs_f64() * 1e3;

        let timed = if got <= 64 {
            4
        } else if got <= 128 {
            3
        } else {
            2
        };
        let mut wall_table = Vec::new();
        let mut wall_sweep = Vec::new();
        for r in 0..5usize {
            let order = if r % 2 == 0 {
                ["table", "sweep"]
            } else {
                ["sweep", "table"]
            };
            for name in order {
                if name == "table" {
                    unsafe {
                        std::env::set_var("IC_SUM_TABLE", "1");
                        std::env::remove_var("IC_FAST_SWEEP");
                    }
                } else {
                    unsafe {
                        std::env::set_var("IC_SUM_TABLE", "0");
                        std::env::remove_var("IC_FAST_SWEEP");
                    }
                }
                let t0 = Instant::now();
                let mut ok = 0usize;
                for seed in 0..timed as u64 {
                    if let Some((x, _)) = ec_index_calculus_dlp_staged_seeded(
                        &curve,
                        &g,
                        &q,
                        want,
                        0,
                        40_000,
                        64,
                        seed + 100,
                    ) {
                        assert_eq!(&x, &x_truth, "want={want} {name} seed={}", seed + 100);
                        ok += 1;
                        black_box(x);
                    }
                }
                let ns = t0.elapsed().as_nanos() as f64 / timed.max(1) as f64;
                if ok == 0 {
                    eprintln!("  warn: want={want} {name} recovered 0/{timed}");
                }
                if name == "table" {
                    wall_table.push(ns);
                } else {
                    wall_sweep.push(ns);
                }
            }
        }
        let wt = med(wall_table);
        let ws = med(wall_sweep);
        let ratio = wt / ws;
        let winner = if ratio > 1.05 {
            "sweep"
        } else if ratio < 0.95 {
            "table"
        } else {
            "tie"
        };
        println!(
            "{want:>4} {got:>4} {build_ms:>10.2} {wt:>12.0} {ws:>12.0} {ratio:>8.3}  {winner}"
        );
    }
}
