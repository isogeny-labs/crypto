//! kappa at genus 3: the wall-clock cost of one counted mod-r multiplication
//! (`la_mul`) in units of one counted F_p multiplication (`cantor_mul`).
//!
//! Protocol: `research/genus_ladder_kappa_20261122/PROTOCOL.md` (frozen first).
//!
//! ```text
//! cargo run --release -j2 --example kappa_g3 -- INSTANCES.jsonl OUT.jsonl TRIALS
//! ```
//!
//! One instance (the first `instance` line) of `INSTANCES.jsonl` is measured; one
//! JSON line per trial goes to `OUT.jsonl`, a `kind:"meta"` line first. Single thread,
//! wall clock, no PMU. `[pcc:unverified]`.
//!
//! * `t_F` in situ: `JCurve::mul_rtl` batches, time over the instrument's own
//!   `fmul + 10 finv` counters.
//! * `t_R` in situ: `run_ic` with SGE(8,3), whole run minus the same run stopped
//!   one trial before the linear algebra starts (`max_trials = trials - 1`; the
//!   relation stream is deterministic in the seed), divided by `la_mul`.
//! * Bare: micro-benchmarks. `mulmod`/`addm` below copy
//!   `chained_transfer::ic_control` (`pub(crate)`, so not reachable from here).

use crypto_lib::cryptanalysis::genus_ladder as gl;
use crypto_lib::cryptanalysis::genus_ladder::{Arm, IcParams, SgeParams};
use serde_json::{json, Value};
use std::hint::black_box;
use std::io::Write;
use std::time::Instant;

/// Copy of `ic_control::mulmod`.
#[inline]
fn mulmod(a: u64, b: u64, r: u64) -> u64 {
    if r < (1u64 << 50) {
        let q = ((a as f64) * (b as f64) / (r as f64)) as u64;
        let t = a.wrapping_mul(b).wrapping_sub(q.wrapping_mul(r)) as i64;
        let ri = r as i64;
        (if t < 0 {
            t + ri
        } else if t >= ri {
            t - ri
        } else {
            t
        }) as u64
    } else {
        ((a as u128 * b as u128) % r as u128) as u64
    }
}

/// Copy of `ic_control::addm`.
#[inline]
fn addm(a: u64, b: u64, r: u64) -> u64 {
    let s = a + b;
    if s >= r {
        s - r
    } else {
        s
    }
}

fn load1() -> f64 {
    std::fs::read_to_string("/proc/loadavg")
        .ok()
        .and_then(|s| s.split_whitespace().next().and_then(|x| x.parse().ok()))
        .unwrap_or(f64::NAN)
}

fn mix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}

/// Repeat `f` (which returns the number of operations it did) until `min_ns`
/// have passed; returns ns per operation.
fn per_op(min_ns: u128, mut f: impl FnMut() -> u64) -> f64 {
    let t = Instant::now();
    let mut ops = 0u64;
    while t.elapsed().as_nanos() < min_ns {
        ops += f();
    }
    t.elapsed().as_nanos() as f64 / ops as f64
}

const N: usize = 4096;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 4 {
        eprintln!("usage: kappa_g3 INSTANCES.jsonl OUT.jsonl TRIALS");
        std::process::exit(2);
    }
    let trials: usize = a[3].parse().expect("TRIALS");
    let text = std::fs::read_to_string(&a[1]).expect("instances");
    let inst_line: Value = text
        .lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .find(|v| v["kind"] == "instance")
        .expect("an instance line");
    let p = inst_line["p"].as_u64().unwrap();
    let f: Vec<u64> = inst_line["f"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_u64().unwrap())
        .collect();
    let (order, h, r) = (
        inst_line["order"].as_u64().unwrap(),
        inst_line["h"].as_u64().unwrap(),
        inst_line["r"].as_u64().unwrap(),
    );
    let curve = gl::JCurve::new(p, 3, &f).expect("curve");
    let inst = gl::instance_from(curve, order, h, r, 1).expect("instance");
    let mut out = std::fs::File::create(&a[2]).expect("out");
    let nb = gl::fb_size_2lp(p, 3, 2.0);
    let prm = |max_trials: u64| IcParams {
        arm: Arm::DoubleLp,
        nb,
        seed: 1,
        max_trials,
        restart: gl::RESTART,
        prefilter: true,
        verify_relations: false,
        sge: Some(SgeParams {
            w_max: 8,
            fill_max: 3.0,
        }),
    };

    // Untimed reference run: trial count and counters (also a warm-up).
    let t0 = Instant::now();
    let full = gl::run_ic(&inst, prm(400_000_000));
    assert!(
        full.certified && full.attempts == 1,
        "need a one-attempt solve"
    );
    let ref_s = t0.elapsed().as_secs_f64();
    let rel_only = gl::run_ic(&inst, prm(full.trials - 1));
    assert!(rel_only.exhausted && rel_only.la.mul == 0 && rel_only.attempts == 0);
    writeln!(
        out,
        "{}",
        json!({"kind":"meta","p":p,"r":r,"r_bits":64 - r.leading_zeros(),
               "mulmod_branch": if r < (1u64<<50) {"f64-Barrett"} else {"u128 %"},
               "trials_ic":full.trials,"la_mul":full.la.mul,"la_add":full.la.add,
               "la_wied_mul":full.la_wied.mul,"la_graph_mul":full.la_graph.mul,
               "la_peel_mul":full.la_peel.mul,"la_sge_mul":full.la_sge.mul,
               "cantor_mul":full.cantor_mul,"cantor_inv":full.cantor_inv,
               "rel_mul":full.rel.mul,"core_rows":full.core_rows,"core_cols":full.core_cols,
               "nnz_after_sge":full.core_nnz_after_sge,"ref_run_s":ref_s,"trials":trials})
    )
    .unwrap();

    // Operands for the micro-benchmarks and Cantor.
    let mut s = 7u64;
    let mut rnd = |m: u64| {
        s = mix(s);
        s % m
    };
    let xs_r: Vec<u64> = (0..N).map(|_| 1 + rnd(r - 1)).collect();
    let vs_r: Vec<u64> = (0..N).map(|_| 1 + rnd(r - 1)).collect();
    let xs_p: Vec<u64> = (0..N).map(|_| 1 + rnd(p - 1)).collect();
    let vs_p: Vec<u64> = (0..N).map(|_| 1 + rnd(p - 1)).collect();
    let mut st = 99u64;
    let divs: Vec<gl::JDiv> = (0..64)
        .map(|_| inst.curve.random_divisor(&mut st))
        .collect();
    let ks: Vec<u64> = (0..64).map(|_| r / 2 + rnd(r / 2)).collect();

    let min_ns = 60_000_000u128;
    let cantor_batch = || -> (f64, u64, u64) {
        let (m0, i0) = (inst.curve.fmul(), inst.curve.finv());
        let t = Instant::now();
        let mut acc = gl::JDiv::IDENTITY;
        for (d, &k) in divs.iter().zip(ks.iter()) {
            acc = inst.curve.mul_rtl(*d, k);
        }
        black_box(acc);
        (
            t.elapsed().as_nanos() as f64,
            inst.curve.fmul() - m0,
            inst.curve.finv() - i0,
        )
    };

    for trial in 0..trials + 2 {
        let warm = trial < 2;
        let la0 = load1();
        // t_F in situ: batches until min_ns, counters from the instrument.
        let (mut ns, mut dm, mut di) = (0f64, 0u64, 0u64);
        while ns < min_ns as f64 {
            let (n, m, i) = cantor_batch();
            ns += n;
            dm += m;
            di += i;
        }
        let tf_situ = ns / (dm + 10 * di) as f64;
        // Bare F_p multiply: dependent chain, and throughput (independent).
        let pb = black_box(p);
        let tf_lat = per_op(min_ns, || {
            let mut x = 12345u64;
            for &y in xs_p.iter() {
                x = (x * y) % pb;
            }
            black_box(x);
            N as u64
        });
        let tf_thr = per_op(min_ns, || {
            let mut acc = 0u64;
            for (&x, &y) in xs_p.iter().zip(vs_p.iter()) {
                acc ^= (x * y) % pb;
            }
            black_box(acc);
            N as u64
        });
        // Bare mod-r: mulmod+addm (mat-vec shape), mulmod chain, add.
        let rb = black_box(r);
        let tr_madd = per_op(min_ns, || {
            let mut acc = 0u64;
            for (&x, &v) in xs_r.iter().zip(vs_r.iter()) {
                acc = addm(acc, mulmod(v, x, rb), rb);
            }
            black_box(acc);
            N as u64
        });
        let tr_lat = per_op(min_ns, || {
            let mut x = 12345u64;
            for &y in xs_r.iter() {
                x = mulmod(x, y, rb);
            }
            black_box(x);
            N as u64
        });
        let tr_mul = per_op(min_ns, || {
            let mut acc = 0u64;
            for (&x, &v) in xs_r.iter().zip(vs_r.iter()) {
                acc ^= mulmod(v, x, rb);
            }
            black_box(acc);
            N as u64
        });
        let ta_p = per_op(min_ns, || {
            let mut acc = 0u64;
            for &x in xs_p.iter() {
                acc = addm(acc, x, pb);
            }
            black_box(acc);
            N as u64
        });
        let ta_r = per_op(min_ns, || {
            let mut acc = 0u64;
            for &x in xs_r.iter() {
                acc = addm(acc, x, rb);
            }
            black_box(acc);
            N as u64
        });
        // t_R in situ: whole run minus relation-only run, interleaved.
        let t = Instant::now();
        let fr = gl::run_ic(&inst, prm(400_000_000));
        let w_full = t.elapsed().as_nanos() as f64;
        let t = Instant::now();
        let rr = gl::run_ic(&inst, prm(full.trials - 1));
        let w_rel = t.elapsed().as_nanos() as f64;
        assert_eq!(fr.la.mul, full.la.mul, "run not deterministic");
        assert_eq!(rr.la.mul, 0);
        let t_la = w_full - w_rel;
        let la1 = load1();
        if warm {
            continue;
        }
        writeln!(
            out,
            "{}",
            json!({"kind":"trial","trial":trial-2,"p":p,
              "load_before":la0,"load_after":la1,
              "cantor_ns":ns,"cantor_dmul":dm,"cantor_dinv":di,
              "tF_situ_ns":tf_situ,"tF_bare_lat_ns":tf_lat,"tF_bare_thr_ns":tf_thr,
              "tR_bare_madd_ns":tr_madd,"tR_bare_lat_ns":tr_lat,"tR_bare_mul_ns":tr_mul,
              "tAdd_p_ns":ta_p,"tAdd_r_ns":ta_r,
              "wall_full_ns":w_full,"wall_rel_ns":w_rel,"t_la_ns":t_la,
              "tR_situ_ns":t_la/full.la.mul as f64,
              "tLA_per_add_ns":t_la/full.la.add as f64})
        )
        .unwrap();
        out.flush().unwrap();
    }
}
