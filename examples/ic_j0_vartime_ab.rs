//! j=0 orbit IC: default vartime vs `IC_IC_CT=1` CT ladders.
//!
//!   TMPDIR=/Volumes/SSD990-2/tmp CARGO_TARGET_DIR=/Volumes/SSD990-2/cargo-target-perf-r12 \
//!   /usr/sbin/taskpolicy -a cargo run --release --example ic_j0_vartime_ab
use crypto_lib::cryptanalysis::ec_index_calculus_j0::j0_index_calculus_dlp_staged;
use crypto_lib::ecc::curve::CurveParams;
use num_bigint::BigUint;
use num_traits::Zero;
use std::hint::black_box;
use std::time::Instant;

fn j0_curve() -> CurveParams {
    CurveParams {
        name: "j0-test-211",
        p: BigUint::from(211u32),
        a: BigUint::zero(),
        b: BigUint::from(2u32),
        gx: BigUint::from(4u32),
        gy: BigUint::from(53u32),
        n: BigUint::from(199u32),
        h: 1,
    }
}

fn med(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn main() {
    let curve = j0_curve();
    let a = curve.a_fe();
    let g = curve.generator();
    let x = BigUint::from(123u32);
    let q = g.scalar_mul_vartime(&x, &a);

    unsafe {
        std::env::remove_var("IC_IC_CT");
    }
    let (x_vt, _) = j0_index_calculus_dlp_staged(&curve, &g, &q, 6, 2, 5000, 64).expect("vt");
    assert_eq!(x_vt, x);
    unsafe {
        std::env::set_var("IC_IC_CT", "1");
    }
    let (x_ct, _) = j0_index_calculus_dlp_staged(&curve, &g, &q, 6, 2, 5000, 64).expect("ct");
    assert_eq!(x_ct, x);
    println!("correctness: ok");

    let mut wv = Vec::new();
    let mut wc = Vec::new();
    let reps = 8usize;
    for r in 0..7 {
        for name in if r % 2 == 0 {
            ["vt", "ct"]
        } else {
            ["ct", "vt"]
        } {
            unsafe {
                if name == "vt" {
                    std::env::remove_var("IC_IC_CT");
                } else {
                    std::env::set_var("IC_IC_CT", "1");
                }
            }
            let _ = black_box(j0_index_calculus_dlp_staged(
                &curve, &g, &q, 6, 2, 5000, 64,
            ));
            let t0 = Instant::now();
            for _ in 0..reps {
                let _ = black_box(j0_index_calculus_dlp_staged(
                    &curve, &g, &q, 6, 2, 5000, 64,
                ));
            }
            let ns = t0.elapsed().as_nanos() as f64 / reps as f64;
            if name == "vt" {
                wv.push(ns);
            } else {
                wc.push(ns);
            }
        }
    }
    let mv = med(wv);
    let mc = med(wc);
    println!("j0_index_calculus_dlp_staged on j0-test-211");
    println!("  vartime (default) : med {mv:.0} ns/solve");
    println!("  CT (IC_IC_CT=1)   : med {mc:.0} ns/solve");
    println!("  speedup CT/vt     : {:.3}x", mc / mv);
}
