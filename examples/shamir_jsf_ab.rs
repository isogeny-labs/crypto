//! Word Shamir Solinas JSF vs two Word ladders (near-u64 prime, ~63-bit scalars).
//!
//!   TMPDIR=/Volumes/SSD990-2/tmp CARGO_TARGET_DIR=/Volumes/SSD990-2/cargo-target-perf \
//!   /usr/sbin/taskpolicy -a cargo run --release --example shamir_jsf_ab
use crypto_lib::ecc::field::FieldElement;
use crypto_lib::ecc::point::Point;
use num_bigint::BigUint;
use std::hint::black_box;
use std::time::Instant;

fn word_edge() -> (Point, Point, FieldElement) {
    let p = BigUint::from(u64::MAX - 58);
    let fe = |v: u64| FieldElement::new(BigUint::from(v), p.clone());
    let a = fe(u64::MAX - 60);
    let g = Point::Affine {
        x: fe(u64::MAX - 1000),
        y: fe(u64::MAX - 77),
    };
    let q = g.scalar_mul_vartime(&BigUint::from(7u32), &a);
    (g, q, a)
}

fn med(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn two_ladders(p: &Point, k: &BigUint, q: &Point, l: &BigUint, a: &FieldElement) -> Point {
    p.scalar_mul_vartime(k, a)
        .add_vartime(&q.scalar_mul_vartime(l, a), a)
}

fn main() {
    let (g, q, a) = word_edge();
    let iters = 8_000usize;
    let rounds = 9usize;

    // Correctness: small + mid-size scalars.
    let mut k = BigUint::from(1u32);
    let mut l = BigUint::from(3u32);
    for _ in 0..80 {
        let jsf = g.double_scalar_mul_vartime(&k, &q, &l, &a);
        let two = two_ladders(&g, &k, &q, &l, &a);
        assert_eq!(jsf, two, "mismatch k={k} l={l}");
        k = &k * 3u32 + 1u32;
        l = &l * 5u32 + 2u32;
    }
    // Full-word scalars (still single-word BigUint).
    for seed in 0..32u64 {
        let kk = BigUint::from(0x9e37_79b9_7f4a_7c15u64.wrapping_mul(seed | 1));
        let ll = BigUint::from(0xbf58_476d_1ce4_e5b9u64.wrapping_mul(seed.wrapping_add(3) | 1));
        let jsf = g.double_scalar_mul_vartime(&kk, &q, &ll, &a);
        let two = two_ladders(&g, &kk, &q, &ll, &a);
        assert_eq!(jsf, two, "word-scalar mismatch seed={seed}");
    }
    println!("correctness: ok");

    let mut wall_two = Vec::new();
    let mut wall_jsf = Vec::new();
    for r in 0..rounds {
        let order = if r % 2 == 0 {
            ["two", "jsf"]
        } else {
            ["jsf", "two"]
        };
        for name in order {
            for i in 0..64u64 {
                let k = BigUint::from(0x9e37_79b9_7f4a_7c15u64.wrapping_mul(i | 1));
                let l = BigUint::from(0xbf58_476d_1ce4_e5b9u64.wrapping_mul(i.wrapping_add(5) | 1));
                let _ = black_box(if name == "two" {
                    two_ladders(&g, &k, &q, &l, &a)
                } else {
                    g.double_scalar_mul_vartime(&k, &q, &l, &a)
                });
            }
            let t0 = Instant::now();
            for i in 0..iters as u64 {
                let k = BigUint::from(0x9e37_79b9_7f4a_7c15u64.wrapping_mul(i | 1));
                let l = BigUint::from(0xbf58_476d_1ce4_e5b9u64.wrapping_mul(i.wrapping_add(5) | 1));
                let _ = black_box(if name == "two" {
                    two_ladders(&g, &k, &q, &l, &a)
                } else {
                    g.double_scalar_mul_vartime(&k, &q, &l, &a)
                });
            }
            let ns = t0.elapsed().as_nanos() as f64 / iters as f64;
            if name == "two" {
                wall_two.push(ns);
            } else {
                wall_jsf.push(ns);
            }
        }
    }
    let wt = med(wall_two);
    let wj = med(wall_jsf);
    println!("Word double-scalar @ ~63-bit scalars (p=2^64-59)");
    println!("  two Word ladders + add : med {wt:.0} ns/op");
    println!("  JSF Word Shamir        : med {wj:.0} ns/op");
    println!("  speedup two/jsf        : {:.3}x  (>1 => JSF wins)", wt / wj);
}
