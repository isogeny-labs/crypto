//! Isogeny-class ECDLP rho: default vartime vs `IC_ISO_CT=1` CT ladders.
//!
//!   TMPDIR=/Volumes/SSD990-2/tmp CARGO_TARGET_DIR=/Volumes/SSD990-2/cargo-target-perf-r11 \
//!   /usr/sbin/taskpolicy -a cargo run --release --example iso_attack_vartime_ab
use crypto_lib::isogeny::attack::rho_on_curve;
use crypto_lib::isogeny::SmallCurve;
use std::hint::black_box;
use std::time::Instant;

fn med(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn main() {
    // Public 16-bit fixture from attack tests.
    let curve = SmallCurve {
        name: "16b",
        p: 65537,
        a: 2,
        b: 3,
    };
    let cap = 1u64 << 16;
    let seed = 42u64;

    unsafe {
        std::env::remove_var("IC_ISO_CT");
    }
    let (sol_vt, it_vt) = rho_on_curve(&curve, cap, seed);
    unsafe {
        std::env::set_var("IC_ISO_CT", "1");
    }
    let (sol_ct, it_ct) = rho_on_curve(&curve, cap, seed);
    assert_eq!(
        sol_vt.as_ref().map(|s| &s.x),
        sol_ct.as_ref().map(|s| &s.x),
        "vartime/CT disagree on discrete log"
    );
    println!(
        "correctness: ok (solved={} vt_iters={it_vt} ct_iters={it_ct})",
        sol_vt.is_some()
    );

    let mut wv = Vec::new();
    let mut wc = Vec::new();
    let reps = 6usize;
    for r in 0..7 {
        for name in if r % 2 == 0 {
            ["vt", "ct"]
        } else {
            ["ct", "vt"]
        } {
            unsafe {
                if name == "vt" {
                    std::env::remove_var("IC_ISO_CT");
                } else {
                    std::env::set_var("IC_ISO_CT", "1");
                }
            }
            let _ = black_box(rho_on_curve(&curve, cap, seed + 1));
            let t0 = Instant::now();
            for i in 0..reps {
                let _ = black_box(rho_on_curve(&curve, cap, seed + 2 + i as u64));
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
    println!("rho_on_curve on p=65537 (cap=2^16, alternating seeds)");
    println!("  vartime (default) : med {mv:.0} ns/call");
    println!("  CT (IC_ISO_CT=1)  : med {mc:.0} ns/call");
    println!("  speedup CT/vt     : {:.3}x", mc / mv);
}
