//! `smith_near_prime`: Smith's Propositions 2 and 4 on random genus-3 curves
//! `y² = f(x)`, `deg f = 7`, over `F_p`, with the exact group order of every
//! curve, so the pass rate can be conditioned on near-primality of `#J` and
//! on the factorisation pattern of `f`.
//!
//! ```text
//! smith_near_prime run --primes 67,131,257,521,1031 --admitted 150 --max-h 8
//!     [--max-seeds 20000] [--threads 4] --out FILE.jsonl
//! ```
//!
//! One JSON line per curve.  Protocol:
//! `research/smith_near_prime_20261114/PROTOCOL.md`.  All figures
//! `[pcc:unverified]`.

use std::io::Write;
use std::process::ExitCode;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use crypto_lib::cryptanalysis::chained_transfer::ic_control::split_order;
use crypto_lib::cryptanalysis::genus_ladder::{random_f, JCurve};
use crypto_lib::cryptanalysis::smith_kernel_coverage::{cycle_type, has_stable_matching};
use crypto_lib::cryptanalysis::smith_prop24 as smith;
use serde_json::json;

fn main() -> ExitCode {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.first().map(String::as_str) != Some("run") {
        eprintln!("usage: smith_near_prime run --primes P,... --admitted N --out F [--max-h 8] [--max-seeds 20000] [--threads 4]");
        return ExitCode::from(2);
    }
    let (mut primes, mut admitted, mut max_h, mut max_seeds, mut threads, mut out) = (
        Vec::<u64>::new(),
        150usize,
        8u64,
        20_000u64,
        4usize,
        String::new(),
    );
    let mut i = 1;
    while i + 1 < a.len() {
        match a[i].as_str() {
            "--primes" => primes = a[i + 1].split(',').map(|s| s.parse().unwrap()).collect(),
            "--admitted" => admitted = a[i + 1].parse().unwrap(),
            "--max-h" => max_h = a[i + 1].parse().unwrap(),
            "--max-seeds" => max_seeds = a[i + 1].parse().unwrap(),
            "--threads" => threads = a[i + 1].parse().unwrap(),
            "--out" => out = a[i + 1].clone(),
            f => {
                eprintln!("unknown flag {f}");
                return ExitCode::from(2);
            }
        }
        i += 2;
    }
    if primes.is_empty() || out.is_empty() {
        eprintln!("--primes and --out are required");
        return ExitCode::from(2);
    }
    let file = std::fs::File::create(&out).expect("create --out");
    let w = Mutex::new(std::io::BufWriter::new(file));
    let emit = |v: serde_json::Value| {
        let mut g = w.lock().unwrap();
        writeln!(g, "{v}").unwrap();
        g.flush().unwrap();
    };
    emit(
        json!({"kind": "config", "primes": primes, "admitted": admitted, "max_h": max_h,
        "max_seeds": max_seeds, "threads": threads}),
    );
    let t0 = Instant::now();
    // Work items: (prime, seed block of 64 seeds); a prime stops once it has
    // its admitted quota, but every seed of a started block is finished so the
    // record is a prefix of the seed order per prime.
    let quota: Vec<AtomicUsize> = primes.iter().map(|_| AtomicUsize::new(0)).collect();
    let next_block: Vec<AtomicUsize> = primes.iter().map(|_| AtomicUsize::new(0)).collect();
    let pi = AtomicUsize::new(0);
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| loop {
                // round-robin over primes that still need curves
                let mut found = None;
                for _ in 0..primes.len() {
                    let k = pi.fetch_add(1, Ordering::Relaxed) % primes.len();
                    if quota[k].load(Ordering::Relaxed) < admitted {
                        let b = next_block[k].fetch_add(1, Ordering::Relaxed) as u64;
                        if b * 64 < max_seeds {
                            found = Some((k, b));
                            break;
                        }
                    }
                }
                let Some((k, b)) = found else { break };
                let p = primes[k];
                for seed in b * 64 + 1..=(b + 1) * 64 {
                    let f = random_f(p, 3, seed);
                    let Ok(curve) = JCurve::new(p, 3, &f) else {
                        emit(json!({"kind": "curve", "p": p, "seed": seed, "squarefree": false}));
                        continue;
                    };
                    let f32: Vec<u32> = f.iter().map(|&c| c as u32).collect();
                    let ct_f = cycle_type(&f32, p as u32).unwrap_or_default();
                    let kfac = ct_f.len();
                    let mut ct8 = ct_f.clone();
                    ct8.push(1);
                    ct8.sort_unstable();
                    let mut rng = smith::SplitMix64(seed ^ p.rotate_left(17));
                    let sm = smith::analyze_curve(&f, p, &mut rng);
                    let order = curve.jacobian_order(seed);
                    let (ord, h, r, adm, adm2, v2) = match &order {
                        Ok(n) => {
                            let v2 = n.trailing_zeros();
                            match split_order(*n, max_h) {
                                Some((h, r)) => (json!(n), json!(h), json!(r), true, h <= 2, v2),
                                None => (json!(n), json!(null), json!(null), false, false, v2),
                            }
                        }
                        Err(_) => (json!(null), json!(null), json!(null), false, false, 0),
                    };
                    if adm {
                        quota[k].fetch_add(1, Ordering::Relaxed);
                    }
                    let mut rec = json!({"kind": "curve", "p": p, "seed": seed, "squarefree": true,
                        "k": kfac, "cycle_type_f": ct_f, "cycle_type_8": ct8,
                        "stable_matching": has_stable_matching(&ct8),
                        "order": ord, "order_err": order.as_ref().err(), "v2": v2,
                        "h": h, "r": r, "admitted": adm, "admitted_h2": adm2});
                    match sm {
                        Some(o) => {
                            rec["smith"] = json!({"cycle_type": o.cycle_type, "n_tractable": o.n_tractable(),
                                "n_trig": o.n_trig(), "full_success": o.full_success(false),
                                "full_success_twist": o.full_success(true)});
                        }
                        None => rec["smith"] = json!(null),
                    }
                    emit(rec);
                }
            });
        }
    });
    emit(json!({"kind": "done", "wall_s": t0.elapsed().as_secs_f64(),
        "admitted": quota.iter().map(|q| q.load(Ordering::Relaxed)).collect::<Vec<_>>()}));
    ExitCode::SUCCESS
}
