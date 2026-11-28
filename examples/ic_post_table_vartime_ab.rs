//! Semaev WordSweep past sum-table crossover (table off / infeasible).
//! Measures `IC_IC_CT=1` CT sign-finalise vs default vartime.
//!
//! Fixture: bench-16bit, fb=384 — prior crossover A/B shows table loses
//! (~1.17× slower) so default/forced sweep is the production path.
//!
//!   TMPDIR=/Volumes/SSD990-2/tmp CARGO_TARGET_DIR=/Volumes/SSD990-2/cargo-target-perf-r12 \
//!   /usr/sbin/taskpolicy -a cargo run --release --example ic_post_table_vartime_ab
use crypto_lib::cryptanalysis::ec_index_calculus::{
    build_factor_base, ec_index_calculus_dlp_staged_seeded,
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
    let a = curve.a_fe();
    let g = curve.generator();
    let x = BigUint::from(12345u32);
    let q = g.scalar_mul_vartime(&x, &a);
    let fb_want = 384usize;
    let got = build_factor_base(&curve, fb_want).len();
    println!("fixture {} fb_want={fb_want} got={got} (sum-table off)", curve.name);

    unsafe {
        std::env::set_var("IC_SUM_TABLE", "0");
        std::env::remove_var("IC_IC_CT");
    }
    let (x_vt, _) =
        ec_index_calculus_dlp_staged_seeded(&curve, &g, &q, fb_want, 0, 8000, 256, 7).expect("vt");
    assert_eq!(x_vt, x);
    unsafe {
        std::env::set_var("IC_IC_CT", "1");
    }
    let (x_ct, _) =
        ec_index_calculus_dlp_staged_seeded(&curve, &g, &q, fb_want, 0, 8000, 256, 7).expect("ct");
    assert_eq!(x_ct, x);
    println!("correctness: ok");

    let mut wv = Vec::new();
    let mut wc = Vec::new();
    let seeds = 3usize;
    for r in 0..5 {
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
                std::env::set_var("IC_SUM_TABLE", "0");
            }
            let _ = black_box(ec_index_calculus_dlp_staged_seeded(
                &curve, &g, &q, fb_want, 0, 8000, 256, 1,
            ));
            let t0 = Instant::now();
            for s in 0..seeds {
                let _ = black_box(ec_index_calculus_dlp_staged_seeded(
                    &curve,
                    &g,
                    &q,
                    fb_want,
                    0,
                    8000,
                    256,
                    20 + s as u64,
                ));
            }
            let ns = t0.elapsed().as_nanos() as f64 / seeds as f64;
            if name == "vt" {
                wv.push(ns);
            } else {
                wc.push(ns);
            }
        }
    }
    let mv = med(wv);
    let mc = med(wc);
    println!("IC staged WordSweep @ fb={got} (table forced off)");
    println!("  vartime finalise  : med {mv:.0} ns/solve");
    println!("  CT (IC_IC_CT=1)   : med {mc:.0} ns/solve");
    println!("  speedup CT/vt     : {:.3}x", mc / mv);
}
