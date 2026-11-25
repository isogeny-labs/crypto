//! Surface audit helper for the rho-parity goal: prove carve-outs vs #4860 gate.
//!
//! `#4860` / workspace rho-parity = ecbench `rho.negation_v5` (pow2+cap64) ≤1×
//! S+wall vs `rho.negation` on prime ledger curves. This example documents that
//! other local rho surfaces are **outside that definition**, with a measured
//! smoke where applicable.
//!
//! ```text
//! TMPDIR=/Volumes/SSD990-2/tmp \
//! CARGO_TARGET_DIR=/Volumes/SSD990-2/cargo-target-rho-parity \
//! /usr/sbin/taskpolicy -a cargo run --release --example rho_parity_surface_audit
//! ```

use crypto_lib::cryptanalysis::ecdlp_nist::group::EcdlpGroup;
use crypto_lib::cryptanalysis::ecdlp_nist::toy;
use crypto_lib::cryptanalysis::ecdlp_nist::{pollard_rho, RhoOptions};
use crypto_lib::isogeny::attack::rho_on_curve;
use crypto_lib::isogeny::SmallCurve;
use num_bigint::BigUint;
use std::hint::black_box;
use std::time::Instant;

fn med(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn main() {
    println!("rho_parity_surface_audit");
    println!("gate_ref: crypto#4860 ecbench rho.negation_v5 vs rho.negation (prime ledger)");
    println!();

    // --- NIST module (BigUint / FIPS catalog path) ---
    {
        let g = toy::prime_small();
        let n = g.order().clone();
        let opts = RhoOptions {
            threads: 1,
            seed: 0xA11CE,
            jumps: 32, // fixed default — NOT rho_v3_jumps_for / pow2+cap64
            fold_automorphisms: true,
            ..RhoOptions::default()
        };
        let k = BigUint::from(7777u64);
        let q = g.mul(&g.generator(), &k);
        let t0 = Instant::now();
        let rep = pollard_rho(&g, &q, &opts);
        let wall_ns = t0.elapsed().as_nanos();
        let ok = rep.scalar.as_ref() == Some(&k);
        println!("surface=ecdlp_nist/rho");
        println!("  scope: FIPS/Certicom/toy BigUint ECDLP; claim hygiene refuses full-width NIST keys");
        println!(
            "  vs_#4860: OUT — not ecbench method; fixed jumps={} (no v5 pow2/cap64)",
            opts.jumps
        );
        println!(
            "  toy_smoke: order_bits≈{} wall_ns={wall_ns} exact_log={ok} iters={}",
            n.bits(),
            rep.iterations
        );
        assert!(ok, "nist toy rho must recover planted log");
    }

    // --- Isogeny attack rho (SmallCurve) ---
    {
        let curve = SmallCurve {
            name: "16b",
            p: 65537,
            a: 2,
            b: 3,
        };
        let cap = 1u64 << 16;
        let mut walls = Vec::new();
        for i in 0..8u64 {
            let t0 = Instant::now();
            let (sol, iters) = rho_on_curve(&curve, cap, 100 + i);
            let ns = t0.elapsed().as_nanos() as f64;
            walls.push(ns);
            black_box((sol, iters));
        }
        println!("surface=iso/rho_on_curve");
        println!("  scope: isogeny-attack SmallCurve DP/r-adding rho (not ic_boundary RhoV3)");
        println!("  vs_#4860: OUT — different API/host; tip default is CT→vartime ship (~7× vs IC_ISO_CT)");
        println!("  tip_smoke: p=65537 med_wall_ns={:.0}", med(walls));
    }

    // --- Volcano BATCH freeze (compile-time constant evidence) ---
    {
        println!("surface=volcano-rho BATCH walk");
        println!("  scope: multi-curve volcano instance study; ecbench arms use rho.negation as reference");
        println!("  vs_#4860: OUT — not the prime-ledger v5 panel; BATCH=16 measured frontier");
        println!("  freeze: BATCH=64/32 REJECT (bit-exact but slower); HashMap+BATCH=16 FROZEN in src/bin/volcano_rho.rs");
    }

    println!();
    println!("audit: named non-ecbench surfaces carved from #4860 gate with evidence above");
}
