//! Default p-aware table vs IC_SUM_TABLE=0 at fb=384 on 18-bit.
use crypto_lib::cryptanalysis::ec_index_calculus::ec_index_calculus_dlp_staged_seeded;
use crypto_lib::ecc::curve::CurveParams;
use num_bigint::BigUint;
use std::hint::black_box;
use std::time::Instant;
fn curve() -> CurveParams {
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
fn med(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}
fn main() {
    let curve = curve();
    let a = curve.a_fe();
    let g = curve.generator();
    let x = BigUint::from(54321u32);
    let q = g.scalar_mul(&x, &a);
    let fb = 384usize;
    let mut wd = Vec::new();
    let mut ws = Vec::new();
    for r in 0..5 {
        for name in if r % 2 == 0 { ["default", "sweep"] } else { ["sweep", "default"] } {
            unsafe {
                if name == "default" {
                    std::env::remove_var("IC_SUM_TABLE");
                } else {
                    std::env::set_var("IC_SUM_TABLE", "0");
                }
            }
            let t0 = Instant::now();
            for seed in 0..2u64 {
                let (xx, _) = ec_index_calculus_dlp_staged_seeded(
                    &curve, &g, &q, fb, 0, 100_000, 64, seed + 3,
                )
                .expect(name);
                assert_eq!(xx, x);
                black_box(xx);
            }
            let ns = t0.elapsed().as_nanos() as f64 / 2.0;
            if name == "default" { wd.push(ns); } else { ws.push(ns); }
        }
    }
    let d = med(wd);
    let s = med(ws);
    println!("18-bit fb=384 default(p-aware) vs forced sweep");
    println!("  default: med {d:.0} ns");
    println!("  sweep  : med {s:.0} ns");
    println!("  speedup sweep/default: {:.3}x", s / d);
}
