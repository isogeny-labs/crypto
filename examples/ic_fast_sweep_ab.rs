//! Paired A/B: default WordSweep vs `IC_FAST_SWEEP=0` (BigUint Semaev sweep)
//! on the tiny staged IC solve (p = 271).
//!
//!   /usr/sbin/taskpolicy -a cargo run --release --example ic_fast_sweep_ab

use crypto_lib::cryptanalysis::ec_index_calculus::ec_index_calculus_dlp_staged_seeded;
use crypto_lib::ecc::curve::CurveParams;
use num_bigint::BigUint;
use std::hint::black_box;
use std::time::Instant;

fn tiny_curve() -> CurveParams {
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
    // Force the per-trial Semaev sweep (not the pair-sum table) so WordSweep
    // is on the measured path.
    unsafe { std::env::set_var("IC_SUM_TABLE", "0") };

    let curve = tiny_curve();
    let a_fe = curve.a_fe();
    let g = curve.generator();
    let x_truth = BigUint::from(190u32);
    let q = g.scalar_mul(&x_truth, &a_fe);

    for (label, val) in [("fast", None), ("slow", Some("0"))] {
        match val {
            None => unsafe { std::env::remove_var("IC_FAST_SWEEP") },
            Some(v) => unsafe { std::env::set_var("IC_FAST_SWEEP", v) },
        }
        let (x, _) =
            ec_index_calculus_dlp_staged_seeded(&curve, &g, &q, 8, 0, 5000, 64, 7).expect(label);
        assert_eq!(&x, &x_truth, "{label}");
        println!("{label}: recovered ok");
    }

    let rounds = 7usize;
    let mut wall_fast = Vec::new();
    let mut wall_slow = Vec::new();
    for r in 0..rounds {
        let order = if r % 2 == 0 {
            ["fast", "slow"]
        } else {
            ["slow", "fast"]
        };
        for name in order {
            if name == "fast" {
                unsafe { std::env::remove_var("IC_FAST_SWEEP") };
            } else {
                unsafe { std::env::set_var("IC_FAST_SWEEP", "0") };
            }
            for seed in 0..2u64 {
                black_box(ec_index_calculus_dlp_staged_seeded(
                    &curve, &g, &q, 8, 0, 5000, 64, seed,
                ));
            }
            let t0 = Instant::now();
            for seed in 0..24u64 {
                black_box(ec_index_calculus_dlp_staged_seeded(
                    &curve, &g, &q, 8, 0, 5000, 64, seed + 100,
                ));
            }
            let ns = t0.elapsed().as_nanos() as f64 / 24.0;
            if name == "fast" {
                wall_fast.push(ns);
            } else {
                wall_slow.push(ns);
            }
        }
    }
    let wf = med(wall_fast);
    let ws = med(wall_slow);
    println!("IC staged WordSweep A/B (24 solves/round, {rounds} rounds, p=271)");
    println!("  BigUint sweep (IC_FAST_SWEEP=0): med {ws:.0} ns/solve");
    println!("  WordSweep default             : med {wf:.0} ns/solve");
    println!("  speedup slow/fast             : {:.3}x", ws / wf);
}
