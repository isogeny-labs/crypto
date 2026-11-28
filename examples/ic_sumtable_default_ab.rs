//! Default (no env) vs IC_SUM_TABLE=0 at fb=96 on bench-16bit.
use crypto_lib::cryptanalysis::ec_index_calculus::ec_index_calculus_dlp_staged_seeded;
use crypto_lib::ecc::curve::CurveParams;
use num_bigint::BigUint;
use std::hint::black_box;
use std::time::Instant;
fn curve() -> CurveParams {
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
    let curve = curve();
    let a = curve.a_fe();
    let g = curve.generator();
    let x = BigUint::from(12345u32);
    let q = g.scalar_mul(&x, &a);
    let fb = 96usize;
    let mut wd = Vec::new();
    let mut ws = Vec::new();
    for r in 0..7 {
        for name in if r % 2 == 0 { ["default", "sweep"] } else { ["sweep", "default"] } {
            unsafe {
                if name == "default" {
                    std::env::remove_var("IC_SUM_TABLE");
                } else {
                    std::env::set_var("IC_SUM_TABLE", "0");
                }
            }
            let t0 = Instant::now();
            for seed in 0..3u64 {
                let (xx, _) = ec_index_calculus_dlp_staged_seeded(
                    &curve, &g, &q, fb, 0, 40_000, 64, seed + 20,
                )
                .expect(name);
                assert_eq!(xx, x);
                black_box(xx);
            }
            let ns = t0.elapsed().as_nanos() as f64 / 3.0;
            if name == "default" {
                wd.push(ns);
            } else {
                ws.push(ns);
            }
        }
    }
    let d = med(wd);
    let s = med(ws);
    println!("fb=96 default vs forced WordSweep (bench-16bit)");
    println!("  default (expect table<=256): med {d:.0} ns/solve");
    println!("  IC_SUM_TABLE=0 (sweep)      : med {s:.0} ns/solve");
    println!("  speedup sweep/default       : {:.3}x", s / d);
}
