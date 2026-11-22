//! Public IC verify: CT scalar_mul vs scalar_mul_vartime.
use crypto_lib::ecc::curve::CurveParams;
use crypto_lib::ecc::field::FieldElement;
use crypto_lib::ecc::point::Point;
use num_bigint::BigUint;
use std::hint::black_box;
use std::time::Instant;
fn med(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}
fn bench(label: &str, g: &Point, q: &Point, x: &BigUint, a: &FieldElement, iters: usize) {
    assert_eq!(&g.scalar_mul(x, a), q);
    assert_eq!(&g.scalar_mul_vartime(x, a), q);
    let mut wct = Vec::new();
    let mut wvt = Vec::new();
    for r in 0..9 {
        for name in if r % 2 == 0 { ["ct", "vt"] } else { ["vt", "ct"] } {
            for _ in 0..32 {
                let _ = black_box(if name == "ct" {
                    g.scalar_mul(x, a)
                } else {
                    g.scalar_mul_vartime(x, a)
                });
            }
            let t0 = Instant::now();
            for _ in 0..iters {
                let _ = black_box(if name == "ct" {
                    g.scalar_mul(x, a)
                } else {
                    g.scalar_mul_vartime(x, a)
                });
            }
            let ns = t0.elapsed().as_nanos() as f64 / iters as f64;
            if name == "ct" {
                wct.push(ns);
            } else {
                wvt.push(ns);
            }
        }
    }
    let ct = med(wct);
    let vt = med(wvt);
    println!("{label}");
    println!("  CT scalar_mul         : {ct:.0} ns");
    println!("  scalar_mul_vartime    : {vt:.0} ns");
    println!("  speedup CT/VT         : {:.3}x", ct / vt);
}
fn main() {
    // Word-edge ~63-bit
    let p = BigUint::from(u64::MAX - 58);
    let fe = |v: u64| FieldElement::new(BigUint::from(v), p.clone());
    let a = fe(u64::MAX - 60);
    let g = Point::Affine {
        x: fe(u64::MAX - 1000),
        y: fe(u64::MAX - 77),
    };
    let x = BigUint::from(0x9e37_79b9_7f4a_7c15u64);
    let q = g.scalar_mul_vartime(&x, &a);
    bench("word-edge ~63-bit scalar", &g, &q, &x, &a, 2000);

    let curve = CurveParams::p256();
    let a = curve.a_fe();
    let g = curve.generator();
    let x = BigUint::from(0xfeed_beef_dead_f00du64).pow(2);
    let q = g.scalar_mul_vartime(&x, &a);
    bench("P-256 multi-word scalar", &g, &q, &x, &a, 400);
}
