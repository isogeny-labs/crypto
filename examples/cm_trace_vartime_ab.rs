//! CM Frobenius BSGS: default vartime vs `IC_CM_CT=1` constant-time ladders.
//!
//!   TMPDIR=/Volumes/SSD990-2/tmp CARGO_TARGET_DIR=/Volumes/SSD990-2/cargo-target-perf-r10 \
//!   /usr/sbin/taskpolicy -a cargo run --release --example cm_trace_vartime_ab
use crypto_lib::isogeny::cm::frobenius_trace_bsgs;
use crypto_lib::isogeny::SmallCurve;
use std::hint::black_box;
use std::time::Instant;

fn med(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn main() {
    // Public fixture from cm::tests::bsgs_runs_at_22_bits.
    let curve = SmallCurve {
        name: "22b",
        p: 4194319,
        a: 3,
        b: 5,
    };

    unsafe {
        std::env::remove_var("IC_CM_CT");
    }
    let t_vt = frobenius_trace_bsgs(&curve).expect("vartime");
    unsafe {
        std::env::set_var("IC_CM_CT", "1");
    }
    let t_ct = frobenius_trace_bsgs(&curve).expect("ct");
    assert_eq!(t_vt, t_ct, "vartime/CT disagree");
    println!("correctness: ok (trace={t_vt})");

    let mut wv = Vec::new();
    let mut wc = Vec::new();
    let reps = 8usize;
    for r in 0..7 {
        for name in if r % 2 == 0 {
            ["vt", "ct"]
        } else {
            ["ct", "vt"]
        } {
            unsafe {
                if name == "vt" {
                    std::env::remove_var("IC_CM_CT");
                } else {
                    std::env::set_var("IC_CM_CT", "1");
                }
            }
            let _ = black_box(frobenius_trace_bsgs(&curve));
            let t0 = Instant::now();
            for _ in 0..reps {
                let _ = black_box(frobenius_trace_bsgs(&curve));
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
    println!("frobenius_trace_bsgs on p=4194319 (22-bit fixture)");
    println!("  vartime (default) : med {mv:.0} ns");
    println!("  CT (IC_CM_CT=1)   : med {mc:.0} ns");
    println!("  speedup CT/vt     : {:.3}x", mc / mv);
}
