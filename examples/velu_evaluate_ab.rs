//! Vélu `evaluate`: precomputed v/u vs `IC_VELU_PRECOMP=0` (recompute each call).
//!
//!   TMPDIR=/Volumes/SSD990-2/tmp CARGO_TARGET_DIR=/Volumes/SSD990-2/cargo-target-perf \
//!   /usr/sbin/taskpolicy -a cargo run --release --example velu_evaluate_ab
use crypto_lib::isogeny::cm::tonelli_shanks_u64;
use crypto_lib::isogeny::toy_curve_a;
use crypto_lib::isogeny::velu::{velu_isogeny_2, velu_isogeny_from_kernel, velu_isogeny_odd, VeluIsogeny};
use std::hint::black_box;
use std::time::Instant;

fn med(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn main() {
    let curve = toy_curve_a();
    let mut isos: Vec<VeluIsogeny> = velu_isogeny_odd(&curve, 3);
    for x in 0..curve.p {
        if curve.rhs(x) == 0 {
            if let Some(iso) = velu_isogeny_2(&curve, x) {
                isos.push(iso);
            }
        }
    }
    let pts_all: Vec<(u64, u64)> = (0..curve.p)
        .filter_map(|x| {
            let rhs = curve.rhs(x);
            if rhs == 0 {
                Some((x, 0))
            } else {
                tonelli_shanks_u64(rhs, curve.p).map(|y| (x, y))
            }
        })
        .take(512)
        .collect();
    // Synthetic half-set of 16 distinct-x points ≈ degree-33 shape so the
    // evaluate hot loop (Montgomery inv + Σ v/u terms) is measurable.
    assert!(pts_all.len() >= 16);
    let half16: Vec<(u64, u64)> = pts_all.iter().copied().take(16).collect();
    isos.push(velu_isogeny_from_kernel(&curve, &half16));
    assert!(!isos.is_empty());
    println!(
        "built {} isogenies on {} (max half={})",
        isos.len(),
        curve.name,
        isos.iter().map(|i| i.kernel_half.len()).max().unwrap_or(0)
    );

    let pts: Vec<(u64, u64)> = pts_all.into_iter().take(128).collect();
    for iso in &isos {
        for &(kx, ky) in &iso.kernel_half {
            assert!(iso.evaluate(kx, ky).is_none());
        }
        for &(x, y) in &pts {
            unsafe {
                std::env::remove_var("IC_VELU_PRECOMP");
            }
            let a = iso.evaluate(x, y);
            unsafe {
                std::env::set_var("IC_VELU_PRECOMP", "0");
            }
            let b = iso.evaluate(x, y);
            assert_eq!(a, b, "precomp != recompute at ({x},{y})");
        }
    }
    println!("correctness: ok");

    let iters = 400usize;
    let mut wp = Vec::new();
    let mut wr = Vec::new();
    for r in 0..9 {
        for name in if r % 2 == 0 {
            ["pre", "re"]
        } else {
            ["re", "pre"]
        } {
            unsafe {
                if name == "pre" {
                    std::env::remove_var("IC_VELU_PRECOMP");
                } else {
                    std::env::set_var("IC_VELU_PRECOMP", "0");
                }
            }
            for iso in &isos {
                for &(x, y) in pts.iter().take(8) {
                    let _ = black_box(iso.evaluate(x, y));
                }
            }
            let t0 = Instant::now();
            for _ in 0..iters {
                for iso in &isos {
                    for &(x, y) in &pts {
                        let _ = black_box(iso.evaluate(x, y));
                    }
                }
            }
            let n = (iters * isos.len() * pts.len()) as f64;
            let ns = t0.elapsed().as_nanos() as f64 / n;
            if name == "pre" {
                wp.push(ns);
            } else {
                wr.push(ns);
            }
        }
    }
    let mp = med(wp);
    let mr = med(wr);
    println!("Vélu evaluate mixed ({} isos × {} pts)", isos.len(), pts.len());
    println!("  precomputed v/u     : med {mp:.1} ns/eval");
    println!("  recompute (opt-out) : med {mr:.1} ns/eval");
    println!("  speedup re/pre      : {:.3}x", mr / mp);

    let big = isos.last().expect("half=16");
    let iters16 = 800usize;
    let mut wp16 = Vec::new();
    let mut wr16 = Vec::new();
    for r in 0..9 {
        for name in if r % 2 == 0 {
            ["pre", "re"]
        } else {
            ["re", "pre"]
        } {
            unsafe {
                if name == "pre" {
                    std::env::remove_var("IC_VELU_PRECOMP");
                } else {
                    std::env::set_var("IC_VELU_PRECOMP", "0");
                }
            }
            for &(x, y) in pts.iter().take(8) {
                let _ = black_box(big.evaluate(x, y));
            }
            let t0 = Instant::now();
            for _ in 0..iters16 {
                for &(x, y) in &pts {
                    let _ = black_box(big.evaluate(x, y));
                }
            }
            let ns = t0.elapsed().as_nanos() as f64 / (iters16 * pts.len()) as f64;
            if name == "pre" {
                wp16.push(ns);
            } else {
                wr16.push(ns);
            }
        }
    }
    let mp16 = med(wp16);
    let mr16 = med(wr16);
    println!("Vélu evaluate half=16 only ({} pts)", pts.len());
    println!("  precomputed v/u     : med {mp16:.1} ns/eval");
    println!("  recompute (opt-out) : med {mr16:.1} ns/eval");
    println!("  speedup re/pre      : {:.3}x", mr16 / mp16);
}
