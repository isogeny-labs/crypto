//! Default pair-sum table vs WordSweep Semaev sweep on tiny staged IC.
use crypto_lib::cryptanalysis::ec_index_calculus::ec_index_calculus_dlp_staged_seeded;
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
    let curve = tiny();
    let a_fe = curve.a_fe();
    let g = curve.generator();
    let q = g.scalar_mul(&BigUint::from(190u32), &a_fe);
    let rounds = 7usize;
    let mut wall_table = Vec::new();
    let mut wall_sweep = Vec::new();
    for r in 0..rounds {
        let order = if r % 2 == 0 { ["table", "sweep"] } else { ["sweep", "table"] };
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
            for seed in 0..2u64 {
                black_box(ec_index_calculus_dlp_staged_seeded(&curve, &g, &q, 8, 0, 5000, 64, seed));
            }
            let t0 = Instant::now();
            for seed in 0..24u64 {
                black_box(ec_index_calculus_dlp_staged_seeded(
                    &curve, &g, &q, 8, 0, 5000, 64, seed + 100,
                ));
            }
            let ns = t0.elapsed().as_nanos() as f64 / 24.0;
            if name == "table" { wall_table.push(ns); } else { wall_sweep.push(ns); }
        }
    }
    let wt = med(wall_table);
    let ws = med(wall_sweep);
    println!("IC staged table vs WordSweep sweep (fb=8, p=271)");
    println!("  sum-table (IC_SUM_TABLE=1): med {wt:.0} ns/solve");
    println!("  WordSweep sweep           : med {ws:.0} ns/solve");
    println!("  speedup table/sweep       : {:.3}x", wt / ws);
}
