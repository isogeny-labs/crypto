//! Class-group Lagrange reduce + compose: i64 fast path vs BigInt (`IC_CLASS_I64=0`).
//!
//!   TMPDIR=/Volumes/SSD990-2/tmp CARGO_TARGET_DIR=/Volumes/SSD990-2/cargo-target-perf \
//!   /usr/sbin/taskpolicy -a cargo run --release --example class_group_reduce_ab
use crypto_lib::isogeny::class_group::{class_number, enumerate_reduced_forms, BinaryQuadraticForm};
use num_bigint::BigInt;
use std::hint::black_box;
use std::time::Instant;

fn med(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn d(n: i64) -> BigInt {
    BigInt::from(n)
}

fn main() {
    // Correctness vs known class numbers + i64/BigInt agreement.
    for disc in [-3i64, -4, -7, -11, -19, -23, -47, -163, -399, -12007] {
        unsafe {
            std::env::remove_var("IC_CLASS_I64");
        }
        let h_fast = class_number(&d(disc));
        unsafe {
            std::env::set_var("IC_CLASS_I64", "0");
        }
        let h_big = class_number(&d(disc));
        assert_eq!(h_fast, h_big, "h({disc})");
        let forms = enumerate_reduced_forms(&d(disc));
        for f in &forms {
            unsafe {
                std::env::remove_var("IC_CLASS_I64");
            }
            let r1 = f.reduce();
            unsafe {
                std::env::set_var("IC_CLASS_I64", "0");
            }
            let r2 = f.reduce();
            assert_eq!(r1.a, r2.a);
            assert_eq!(r1.b, r2.b);
            assert_eq!(r1.c, r2.c);
            assert!(r1.is_reduced());
        }
        // Compose agreement on a small subset.
        let n = forms.len().min(8);
        for i in 0..n {
            for j in 0..n {
                unsafe {
                    std::env::remove_var("IC_CLASS_I64");
                }
                let c1 = forms[i].compose(&forms[j]);
                unsafe {
                    std::env::set_var("IC_CLASS_I64", "0");
                }
                let c2 = forms[i].compose(&forms[j]);
                assert_eq!((&c1.a, &c1.b, &c1.c), (&c2.a, &c2.b, &c2.c), "compose {disc}");
            }
        }
    }
    println!("correctness: ok");

    // Build a pool of unreduced forms by composing reduced ones of D=-12007.
    let disc = d(-12007);
    unsafe {
        std::env::remove_var("IC_CLASS_I64");
    }
    let base = enumerate_reduced_forms(&disc);
    let mut pool: Vec<BinaryQuadraticForm> = Vec::new();
    for i in 0..base.len().min(32) {
        for j in 0..base.len().min(32) {
            // compose returns reduced; stretch by inverse+compose cycles
            let mut f = base[i].compose(&base[j]);
            for _ in 0..3 {
                f = BinaryQuadraticForm {
                    a: f.a.clone(),
                    b: -&f.b * 3 + &f.a * 2, // unreduced-ish
                    c: f.c.clone(),
                };
                // fix discriminant by adjusting c
                let want = &disc;
                let bb = &f.b * &f.b;
                let four_a = &f.a * 4;
                // c = (b² - D) / (4a)
                f.c = (&bb - want) / &four_a;
                pool.push(f.clone());
            }
        }
    }
    assert!(!pool.is_empty());

    let iters = 80usize;
    let mut wi = Vec::new();
    let mut wb = Vec::new();
    for r in 0..9 {
        for name in if r % 2 == 0 {
            ["i64", "big"]
        } else {
            ["big", "i64"]
        } {
            unsafe {
                if name == "i64" {
                    std::env::remove_var("IC_CLASS_I64");
                } else {
                    std::env::set_var("IC_CLASS_I64", "0");
                }
            }
            for f in pool.iter().take(16) {
                let _ = black_box(f.reduce());
            }
            let t0 = Instant::now();
            for _ in 0..iters {
                for f in &pool {
                    let _ = black_box(f.reduce());
                }
            }
            let ns = t0.elapsed().as_nanos() as f64 / (iters * pool.len()) as f64;
            if name == "i64" {
                wi.push(ns);
            } else {
                wb.push(ns);
            }
        }
    }
    let mi = med(wi);
    let mb = med(wb);
    println!("class-group reduce on |D|=12007 unreduced pool ({})", pool.len());
    println!("  i64 fast path     : med {mi:.1} ns");
    println!("  BigInt (opt-out)  : med {mb:.1} ns");
    println!("  speedup big/i64   : {:.3}x", mb / mi);

    // Also enumerate wall time.
    let mut we_i = Vec::new();
    let mut we_b = Vec::new();
    for r in 0..7 {
        for name in if r % 2 == 0 {
            ["i64", "big"]
        } else {
            ["big", "i64"]
        } {
            unsafe {
                if name == "i64" {
                    std::env::remove_var("IC_CLASS_I64");
                } else {
                    std::env::set_var("IC_CLASS_I64", "0");
                }
            }
            let t0 = Instant::now();
            for _ in 0..20 {
                let _ = black_box(enumerate_reduced_forms(&disc));
            }
            let ns = t0.elapsed().as_nanos() as f64 / 20.0;
            if name == "i64" {
                we_i.push(ns);
            } else {
                we_b.push(ns);
            }
        }
    }
    let ei = med(we_i);
    let eb = med(we_b);
    println!("enumerate_reduced_forms(D=-12007)");
    println!("  i64 : med {ei:.0} ns");
    println!("  big : med {eb:.0} ns");
    println!("  speedup big/i64 : {:.3}x", eb / ei);

    // Compose A/B on reduced forms of D=-12007.
    let n = base.len().min(48);
    let mut wc_i = Vec::new();
    let mut wc_b = Vec::new();
    let compose_iters = 40usize;
    for r in 0..9 {
        for name in if r % 2 == 0 {
            ["i64", "big"]
        } else {
            ["big", "i64"]
        } {
            unsafe {
                if name == "i64" {
                    std::env::remove_var("IC_CLASS_I64");
                } else {
                    std::env::set_var("IC_CLASS_I64", "0");
                }
            }
            for i in 0..n.min(4) {
                let _ = black_box(base[i].compose(&base[(i + 3) % n]));
            }
            let t0 = Instant::now();
            for _ in 0..compose_iters {
                for i in 0..n {
                    for j in 0..n {
                        let _ = black_box(base[i].compose(&base[j]));
                    }
                }
            }
            let ns = t0.elapsed().as_nanos() as f64 / (compose_iters * n * n) as f64;
            if name == "i64" {
                wc_i.push(ns);
            } else {
                wc_b.push(ns);
            }
        }
    }
    let ci = med(wc_i);
    let cb = med(wc_b);
    println!("class-group compose on reduced |D|=12007 ({n}×{n})");
    println!("  i64 fast path     : med {ci:.1} ns");
    println!("  BigInt (opt-out)  : med {cb:.1} ns");
    println!("  speedup big/i64   : {:.3}x", cb / ci);
}
