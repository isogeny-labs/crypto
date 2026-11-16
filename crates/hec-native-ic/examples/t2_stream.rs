//! Track T2 stream benchmark (PLAN.md §9): the per-trial cost of the walk and
//! the oracle on a fixed, seeded stream of trials with the IC's mix.
//!
//! Each row draws one curve (`thread_curve`, smallest valid `c`), its factor
//! base, two divisors `D₁`, `D₂` (sums of `g` base places) and 64 step places,
//! then a stream of `HEC_TRIALS` trials: `+D₁` or `+D₂` with probability 1/4
//! each, otherwise `+F_j` for one of the 64 step places (as `ic.rs` with
//! `d_step_in = 4`). One untimed pass warms up; `HEC_PASSES` timed passes
//! follow, each a walk loop (keeping every `u`) and then an oracle loop over
//! those `u`. Per-trial costs are medians over the timed passes.
//!
//! No timers inside the loops. Hashes of the walk (`u`, `v` of every
//! position) and of the oracle output (roots and multiplicities, or none)
//! make the stream an exactness check: two builds that agree on both hashes
//! did the same arithmetic. With feature `count-inv` the line also carries
//! `Fp::inv` calls per walk step and per oracle call.
//!
//! ```sh
//! cargo run --release --example t2_stream
//! HEC_ROWS=3:401,4:61 HEC_TRIALS=50000 cargo run --release --example t2_stream
//! ```
use std::hint::black_box;
use std::time::Instant;

use hec_native_ic::fp::inv_calls;
use hec_native_ic::instance::{factor_base, thread_curve};
use hec_native_ic::jac::Div;
use hec_native_ic::oracle::{split, OracleStats};
use hec_native_ic::poly::Poly;
use hec_native_ic::rng::Rng;

const SEED: u64 = 20261115;

fn env<T: std::str::FromStr>(k: &str, d: T) -> T {
    std::env::var(k).ok().and_then(|v| v.parse().ok()).unwrap_or(d)
}

fn fnv(h: &mut u64, x: u64) {
    for b in x.to_le_bytes() {
        *h ^= b as u64;
        *h = h.wrapping_mul(0x100000001b3);
    }
}

fn hash_poly(h: &mut u64, p: &Poly) {
    fnv(h, p.deg() as u64);
    for i in 0..=p.deg().max(0) as usize {
        fnv(h, p.c[i]);
    }
}

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn main() {
    let rows: Vec<(usize, u64)> = match std::env::var("HEC_ROWS") {
        Ok(s) => s
            .split(',')
            .filter_map(|t| {
                let (g, p) = t.split_once(':')?;
                Some((g.trim().parse().ok()?, p.trim().parse().ok()?))
            })
            .collect(),
        Err(_) => vec![
            (2, 101),
            (2, 251),
            (2, 503),
            (2, 1009),
            (2, 2003),
            (2, 4001),
            (3, 61),
            (3, 211),
            (3, 401),
            (4, 31),
            (4, 61),
            (3, 4001),
            (4, 1009),
        ],
    };
    let n: usize = env("HEC_TRIALS", 100_000);
    let passes: usize = env("HEC_PASSES", 3);
    for (g, p) in rows {
        let (c, curve) = (1..1000u64)
            .find_map(|c| thread_curve(p, c, g).map(|cv| (c, cv)))
            .expect("no squarefree f");
        let fp = curve.fp;
        let fb = factor_base(&curve);
        let mut rng = Rng::new(SEED ^ (p << 8) ^ g as u64);
        let rand_div = |rng: &mut Rng| {
            let mut d = curve.zero();
            for _ in 0..g {
                let (x, y) = fb[rng.below(fb.len() as u64) as usize];
                d = curve.add_place(&d, x, y);
            }
            d
        };
        let start = rand_div(&mut rng);
        let (d1, d2) = (rand_div(&mut rng), rand_div(&mut rng));
        let steps: Vec<(u64, u64)> = (0..64)
            .map(|_| fb[rng.below(fb.len() as u64) as usize])
            .collect();
        let picks: Vec<u8> = (0..n)
            .map(|_| match rng.below(4) {
                0 => 64,
                1 => 65,
                _ => rng.below(64) as u8,
            })
            .collect();

        let mut pos: Vec<Div> = Vec::with_capacity(n);
        let (mut walk_ns, mut orc_ns) = (Vec::new(), Vec::new());
        let (mut walk_hash, mut orc_hash) = (0u64, 0u64);
        let (mut walk_inv, mut orc_inv) = (0u64, 0u64);
        let mut st = OracleStats::default();
        let mut smooth = 0u64;
        for pass in 0..=passes {
            pos.clear();
            let mut r = start;
            let i0 = inv_calls();
            let t = Instant::now();
            for &s in &picks {
                r = match s {
                    64 => curve.add(&r, &d1),
                    65 => curve.add(&r, &d2),
                    j => {
                        let (x, y) = steps[j as usize];
                        curve.add_place(&r, x, y)
                    }
                };
                pos.push(r);
            }
            let wt = t.elapsed().as_nanos() as f64;
            let wi = inv_calls() - i0;
            let mut out = Vec::with_capacity(n);
            let mut s = OracleStats::default();
            let i0 = inv_calls();
            let t = Instant::now();
            for d in &pos {
                out.push(split(black_box(&d.u), &fp, &mut s));
            }
            let ot = t.elapsed().as_nanos() as f64;
            let oi = inv_calls() - i0;
            if pass == 0 {
                // Untimed warm-up; hashes and counts from it.
                let mut h = 0xcbf29ce484222325u64;
                for d in &pos {
                    hash_poly(&mut h, &d.u);
                    hash_poly(&mut h, &d.v);
                }
                walk_hash = h;
                let mut h = 0xcbf29ce484222325u64;
                for o in &out {
                    match o {
                        None => fnv(&mut h, u64::MAX),
                        Some(rs) => {
                            smooth += 1;
                            fnv(&mut h, rs.len as u64);
                            for &(x, m) in rs.as_slice() {
                                fnv(&mut h, x);
                                fnv(&mut h, m as u64);
                            }
                        }
                    }
                }
                orc_hash = h;
                (walk_inv, orc_inv, st) = (wi, oi, s);
            } else {
                walk_ns.push(wt / n as f64);
                orc_ns.push(ot / n as f64);
            }
        }
        let (w, o) = (median(&mut walk_ns), median(&mut orc_ns));
        println!(
            "JSON {{\"tool\":\"t2_stream\",\"genus\":{g},\"p\":{p},\"c\":{c},\"m\":{},\"trials\":{n},\"passes\":{passes},\
\"walk_ns\":{w:.2},\"oracle_ns\":{o:.2},\"per_trial_ns\":{:.2},\
\"walk_inv\":{:.4},\"oracle_inv\":{:.4},\"frobenius\":{},\"parity_rejects\":{},\"smooth\":{smooth},\
\"walk_hash\":\"{walk_hash:016x}\",\"oracle_hash\":\"{orc_hash:016x}\"}}",
            fb.len(),
            w + o,
            walk_inv as f64 / n as f64,
            orc_inv as f64 / n as f64,
            st.frobenius,
            st.parity_rejects,
        );
    }
}
