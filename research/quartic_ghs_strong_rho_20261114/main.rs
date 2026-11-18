//! Driver for the N4 plane-quartic strong-rho control.
//!
//! Build: `rustc --edition 2021 -O main.rs -o ghs_strong_rho`

#![allow(dead_code, static_mut_refs)]

#[path = "../quartic_n4_20261019/common.rs"]
mod common;
#[path = "../quartic_n4_20261019/ghs.rs"]
mod ghs;
#[path = "../../src/cryptanalysis/plane_quartic_pic.rs"]
mod pic;

mod ic2 {
    use crate::common::{bump_ladd, bump_lmul};

    #[inline(always)]
    pub fn la2(a: u64, b: u64, ell: u64) -> u64 {
        bump_ladd(1);
        let (s, overflow) = a.overflowing_add(b);
        if overflow || s >= ell {
            s.wrapping_sub(ell)
        } else {
            s
        }
    }

    #[inline(always)]
    pub fn ls2(a: u64, b: u64, ell: u64) -> u64 {
        bump_ladd(1);
        if a >= b {
            a - b
        } else {
            a + (ell - b)
        }
    }

    #[inline(always)]
    pub fn lm2(a: u64, b: u64, ell: u64) -> u64 {
        bump_lmul(1);
        ((a as u128 * b as u128) % ell as u128) as u64
    }
}

#[path = "../quartic_n4v2_20261020/rho2.rs"]
mod rho2;
mod strong_rho;

use common::*;
use ghs::*;
use strong_rho::{strong_rho, FieldCounts, NativeCounts, StrongOpts};

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

/// Interleaved same-host calibration for the two priced scalar operations.
/// This is a conversion factor, not a throughput result.
fn calibration(q: u64, ell: u64, triples: usize) -> (f64, f64, f64, f64) {
    use std::hint::black_box;
    use std::time::Instant;
    assert!(triples > 0);
    let (mut x, mut z, mut w) = (12_345 % q, 3u64, 12_345 % ell);
    let (y, yl) = (54_321 % q, 54_321 % ell);
    let mut ri = Vec::with_capacity(triples);
    let mut rl = Vec::with_capacity(triples);
    let mut tm = Vec::with_capacity(triples);
    for _ in 0..triples {
        let t = Instant::now();
        for _ in 0..20_000 {
            // Match the frozen N4-v2 calibration kernel exactly.  q < 2^31,
            // so this product cannot overflow u64.
            x = black_box(x * y % q);
            x ^= 1;
        }
        let mul_s = t.elapsed().as_secs_f64() / 20_000.0;
        let t = Instant::now();
        for _ in 0..2_000 {
            z = black_box(inv_mod_u64(z % (q - 1) + 1, q));
        }
        let inv_s = t.elapsed().as_secs_f64() / 2_000.0;
        let t = Instant::now();
        for _ in 0..20_000 {
            w = black_box(((w as u128 * yl as u128) % ell as u128) as u64);
            w ^= 1;
        }
        let lmul_s = t.elapsed().as_secs_f64() / 20_000.0;
        ri.push(inv_s / mul_s);
        rl.push(lmul_s / mul_s);
        tm.push(mul_s * 1e9);
    }
    black_box((x, z, w));
    let min_ns = tm.iter().copied().fold(f64::INFINITY, f64::min);
    (median(&mut ri), median(&mut rl), median(&mut tm), min_ns)
}

fn target_stream(inst: &Inst, seed: u64) -> (u64, Rng) {
    // Match N4-v2's preamble so paired runs use its public synthetic targets.
    let mut rng = Rng::new(seed ^ 0x1234);
    let p = inst.ec.rand_point(&mut rng);
    let p2 = inst.ec.rand_point(&mut rng);
    let mut acc = p;
    for _ in 0..100 {
        acc = inst.ec.add(&acc, &p2);
    }
    std::hint::black_box(acc);
    let ic_target = rng.below(inst.ell - 1) + 1;
    (ic_target, rng)
}

fn flat_field(prefix: &str, c: FieldCounts) -> String {
    format!(
        "\"{p}.fq_mul\":{},\"{p}.fq_inv\":{},\"{p}.fq_add\":{},\"{p}.f3_mul\":{},\"{p}.f3_sq\":{},\"{p}.f3_inv\":{},\"{p}.batch_inv\":{},\"{p}.group_add\":{}",
        c.fq_mul, c.fq_inv, c.fq_add, c.f3_mul, c.f3_sq, c.f3_inv, c.batch_inv, c.group_add,
        p = prefix,
    )
}

fn flat_native(prefix: &str, c: NativeCounts) -> String {
    format!(
        "\"{p}.rng\":{},\"{p}.hash\":{},\"{p}.canon\":{},\"{p}.lookahead_test\":{},\"{p}.dp\":{},\"{p}.table\":{},\"{p}.cycle_cmp\":{},\"{p}.scalar_add\":{},\"{p}.scalar_mul\":{},\"{p}.scalar_inv\":{},\"{p}.band_ops\":{}",
        c.rng_draws, c.hashes, c.canonicalisations, c.lookahead_tests, c.dp_tests, c.table_ops, c.cycle_compares,
        c.scalar_adds, c.scalar_muls, c.scalar_invs, c.band_ops(), p = prefix,
    )
}

fn old_counts(c: &Cnt) -> String {
    format!(
        "\"c.mul\":{},\"c.inv\":{},\"c.add\":{},\"c.lmul\":{},\"c.ladd\":{},\"c.hash\":{},\"c.cmp\":{}",
        c.mul, c.inv, c.add, c.lmul, c.ladd, c.hash, c.cmp,
    )
}

fn run(q: u64, seed: u64, i0: usize, i1: usize, target_policy: &str, arms: &str) {
    assert!(matches!(target_policy, "ic-repeat" | "sequence"));
    let inst = Inst::search(q, seed, 200_000).expect("no GHS instance");
    let (ic_target, mut targets) = target_stream(&inst, seed);
    let triples = std::env::var("GHS_CALIB_TRIPLES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(600usize);
    let (r_inv, r_lmul, mul_ns_median, mul_ns_min) = calibration(q, inst.ell, triples);
    println!(
        "{{\"rec\":\"instance\",\"q\":{},\"seed\":{},\"ell\":{},\"order\":{},\"tries\":{},\"field_c\":{},\"ic_target\":{},\"target_policy\":\"{}\",\"source_revision\":\"{}\"}}",
        q,
        seed,
        inst.ell,
        inst.order,
        inst.tries,
        inst.kf.c,
        ic_target,
        target_policy,
        option_env!("GHS_SOURCE_REVISION").unwrap_or("unbound"),
    );
    println!(
        "{{\"rec\":\"calibration\",\"q\":{},\"seed\":{},\"triples\":{},\"r_inv\":{:.9},\"r_lmul\":{:.9},\"mul_ns_median\":{:.9},\"mul_ns_min\":{:.9},\"isolation\":\"{}\"}}",
        q,
        seed,
        triples,
        r_inv,
        r_lmul,
        mul_ns_median,
        mul_ns_min,
        std::env::var("GHS_ISOLATION").unwrap_or_else(|_| "development".into()),
    );
    for i in 0..i1 {
        let sequence_target = if i == 0 {
            ic_target
        } else {
            targets.below(inst.ell - 1) + 1
        };
        let (k, target_kind) = if target_policy == "ic-repeat" {
            (ic_target, "ic")
        } else if i == 0 {
            (sequence_target, "ic")
        } else {
            (sequence_target, "holdout")
        };
        if i < i0 {
            continue;
        }
        for arm in arms.split(',') {
            if arm == "old-neg" {
                let out = rho2::rho_neg(&inst, k, seed * 1000 + i as u64);
                println!(
                    "{{\"rec\":\"rho\",\"arm\":\"old-neg\",\"q\":{},\"seed\":{},\"i\":{},\"target_kind\":\"{}\",\"ell\":{},\"k\":{},\"ok\":{},\"steps\":{},\"dps\":{},\"restarts\":{},\"cycles\":{},\"lookahead\":{},\"wall_s\":{:.9},{}}}",
                    q, seed, i, target_kind, inst.ell, k, out.ok, out.steps, out.dps, out.restarts, out.cycles,
                    out.lookahead, out.wall, old_counts(&out.cnt),
                );
                continue;
            }
            let setup_control = arm.ends_with("-legacy-setup");
            let width_text = arm
                .strip_prefix("strong-b")
                .unwrap_or_else(|| panic!("unknown arm {arm}"));
            let width: usize = width_text
                .strip_suffix("-legacy-setup")
                .unwrap_or(width_text)
                .parse()
                .expect("batch width");
            let mut opts = StrongOpts::for_instance(&inst, width);
            opts.batch_setup = !setup_control;
            let out = strong_rho(&inst, k, seed * 1000 + i as u64, opts);
            println!(
                "{{\"rec\":\"rho\",\"arm\":\"{}\",\"batch\":{},\"setup_mode\":\"{}\",\"q\":{},\"seed\":{},\"i\":{},\"target_kind\":\"{}\",\"ell\":{},\"k\":{},\"k_found\":{},\"ok\":{},\"steps\":{},\"dps\":{},\"restarts\":{},\"lookahead\":{},\"cycles\":{},\"cycle_adds\":{},\"wall_s\":{:.9},{},{},{},{},{}}}",
                arm,
                width,
                if setup_control { "legacy" } else { "batched-window2" },
                q,
                seed,
                i,
                target_kind,
                inst.ell,
                k,
                out.found.map(|v| v.to_string()).unwrap_or_else(|| "null".into()),
                out.ok,
                out.steps,
                out.dps,
                out.restarts,
                out.lookahead,
                out.cycles,
                out.cycle_adds,
                out.wall_s,
                flat_field("setup", out.setup_field),
                flat_field("walk", out.walk_field),
                flat_field("verify", out.verify_field),
                flat_native("setup_native", out.setup_native),
                flat_native("walk_native", out.walk_native),
            );
        }
    }
}

fn usage() -> ! {
    eprintln!(
        "usage: ghs_strong_rho selftest | run <q> <seed> <i0> <i1> <ic-repeat|sequence> <old-neg,strong-b1,strong-b1-legacy-setup,...>"
    );
    std::process::exit(2)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 2 && args[1] == "selftest" {
        strong_rho::selftest();
        return;
    }
    if args.len() != 8 || args[1] != "run" {
        usage();
    }
    run(
        args[2].parse().unwrap(),
        args[3].parse().unwrap(),
        args[4].parse().unwrap(),
        args[5].parse().unwrap(),
        &args[6],
        &args[7],
    );
}
