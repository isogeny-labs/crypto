//! Isogeny graph BFS: default vartime torsion sampling vs `IC_ISO_CT=1`.
//!
//!   TMPDIR=/Volumes/SSD990-2/tmp CARGO_TARGET_DIR=/Volumes/SSD990-2/cargo-target-perf-r11 \
//!   /usr/sbin/taskpolicy -a cargo run --release --example iso_graph_vartime_ab
use crypto_lib::isogeny::graph::build_graph;
use crypto_lib::isogeny::toy_curve_a;
use std::hint::black_box;
use std::time::Instant;

fn med(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn main() {
    let curve = toy_curve_a();
    let primes = [2u64, 3, 5];
    let max_nodes = 64usize;

    unsafe {
        std::env::remove_var("IC_ISO_CT");
    }
    let g_vt = build_graph(&curve, &primes, max_nodes);
    unsafe {
        std::env::set_var("IC_ISO_CT", "1");
    }
    let g_ct = build_graph(&curve, &primes, max_nodes);
    assert_eq!(g_vt.nodes.len(), g_ct.nodes.len());
    assert_eq!(g_vt.edges.len(), g_ct.edges.len());
    println!(
        "correctness: ok (nodes={} edges={})",
        g_vt.nodes.len(),
        g_vt.edges.len()
    );

    let mut wv = Vec::new();
    let mut wc = Vec::new();
    let reps = 10usize;
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
            let _ = black_box(build_graph(&curve, &primes, max_nodes));
            let t0 = Instant::now();
            for _ in 0..reps {
                let _ = black_box(build_graph(&curve, &primes, max_nodes));
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
    println!("build_graph toy-2003 S={{2,3,5}} max_nodes={max_nodes}");
    println!("  vartime (default) : med {mv:.0} ns");
    println!("  CT (IC_ISO_CT=1)  : med {mc:.0} ns");
    println!("  speedup CT/vt     : {:.3}x", mc / mv);
}
