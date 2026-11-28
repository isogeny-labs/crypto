//! Word joint window w=2/4 vs Solinas JSF (one-shot double-scalar).
//!
//!   TMPDIR=/Volumes/SSD990-2/tmp CARGO_TARGET_DIR=/Volumes/SSD990-2/cargo-target-perf \
//!   /usr/sbin/taskpolicy -a cargo run --release --example shamir_window_ab
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

fn mul(mode: &str, g: &Point, k: &BigUint, q: &Point, l: &BigUint, a: &FieldElement) -> Point {
    unsafe {
        match mode {
            "jsf" => std::env::remove_var("IC_WORD_SHAMIR"),
            "w2" => std::env::set_var("IC_WORD_SHAMIR", "w2"),
            "w4" => std::env::set_var("IC_WORD_SHAMIR", "w4"),
            _ => unreachable!(),
        }
    }
    g.double_scalar_mul_vartime(k, q, l, a)
}

fn main() {
    let (g, q, a) = word_edge();
    // Correctness: all three match two ladders.
    let mut k = BigUint::from(1u32);
    let mut l = BigUint::from(3u32);
    for _ in 0..60 {
        let want = g
            .scalar_mul_vartime(&k, &a)
            .add_vartime(&q.scalar_mul_vartime(&l, &a), &a);
        for mode in ["jsf", "w2", "w4"] {
            assert_eq!(mul(mode, &g, &k, &q, &l, &a), want, "{mode} k={k} l={l}");
        }
        k = &k * 3u32 + 1u32;
        l = &l * 5u32 + 2u32;
    }
    for seed in 0..24u64 {
        let kk = BigUint::from(0x9e37_79b9_7f4a_7c15u64.wrapping_mul(seed | 1));
        let ll = BigUint::from(0xbf58_476d_1ce4_e5b9u64.wrapping_mul(seed.wrapping_add(3) | 1));
        let want = g
            .scalar_mul_vartime(&kk, &a)
            .add_vartime(&q.scalar_mul_vartime(&ll, &a), &a);
        for mode in ["jsf", "w2", "w4"] {
            assert_eq!(mul(mode, &g, &kk, &q, &ll, &a), want, "{mode} seed={seed}");
        }
    }
    println!("correctness: ok");

    let iters = 6_000usize;
    let modes = ["jsf", "w2", "w4"];
    let mut walls: [Vec<f64>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for r in 0..9usize {
        let order = if r % 2 == 0 {
            [0usize, 1, 2]
        } else {
            [2, 1, 0]
        };
        for &mi in &order {
            let mode = modes[mi];
            for i in 0..48u64 {
                let k = BigUint::from(0x9e37_79b9_7f4a_7c15u64.wrapping_mul(i | 1));
                let l = BigUint::from(0xbf58_476d_1ce4_e5b9u64.wrapping_mul(i.wrapping_add(5) | 1));
                let _ = black_box(mul(mode, &g, &k, &q, &l, &a));
            }
            let t0 = Instant::now();
            for i in 0..iters as u64 {
                let k = BigUint::from(0x9e37_79b9_7f4a_7c15u64.wrapping_mul(i | 1));
                let l = BigUint::from(0xbf58_476d_1ce4_e5b9u64.wrapping_mul(i.wrapping_add(5) | 1));
                let _ = black_box(mul(mode, &g, &k, &q, &l, &a));
            }
            walls[mi].push(t0.elapsed().as_nanos() as f64 / iters as f64);
        }
    }
    let mj = med(walls[0].clone());
    let m2 = med(walls[1].clone());
    let m4 = med(walls[2].clone());
    println!("Word double-scalar window vs JSF (~63-bit, one-shot, p=2^64-59)");
    println!("  JSF (default) : med {mj:.0} ns/op");
    println!("  window w=2    : med {m2:.0} ns/op  (JSF/w2={:.3}x)", mj / m2);
    println!("  window w=4    : med {m4:.0} ns/op  (JSF/w4={:.3}x)", mj / m4);
    let best = if m2 <= m4 { ("w2", m2) } else { ("w4", m4) };
    if best.1 < mj * 0.95 {
        println!("  verdict: {} beats JSF — candidate to land", best.0);
    } else {
        println!("  verdict: keep JSF (windows do not beat it on one-shot)");
    }
}
