//! **How often a random curve over `F_{p⁶}` lands in a Joux–Vitse weak class**
//! (research/jv_weak_coverage_20261111).
//!
//! For `n` random curves with full rational 2-torsion over `F_{q³}`, `q = p²`
//! ([`random_curve`]): count the points ([`curve_order`], baby-step giant-step),
//! apply the proved necessary trace condition ([`trace_may_hold_weak_curve`]:
//! `t ≡ ±(p⁶ + 1) mod 16`), and for an admitted class run the breadth-first
//! walk of the end-to-end route ([`walk_to_weak_record`], 2-isogenies and the
//! odd degrees `--jumps` that are not inert) until a weak curve is met or
//! `--cap` distinct curves have been. A found weak curve is a witness that the
//! class holds one; a capped walk leaves the class unresolved. One JSON line per
//! curve, with the field multiplications of the count and of the walk.
//!
//! ```text
//! cargo run --release --example jv_weak_coverage -- --p 1009 --n 64 --seed 20261111 --cap 1000000 --jumps 3,5,7 [--first 0] [--walk-limit 64]
//! ```

use std::env;
use std::time::Instant;

use crypto_lib::cryptanalysis::jv_isogeny_walk::{
    curve_order, random_curve, trace_may_hold_weak_curve, walk_to_weak_record, WalkCtx,
};
use rand::rngs::StdRng;
use rand::SeedableRng;

fn main() {
    let args: Vec<String> = env::args().collect();
    let get = |k: &str, d: &str| -> String {
        args.iter()
            .position(|a| a == k)
            .map(|i| args[i + 1].clone())
            .unwrap_or_else(|| d.to_string())
    };
    let p: u64 = get("--p", "1009").parse().expect("--p");
    let n: u64 = get("--n", "64").parse().expect("--n");
    let first: u64 = get("--first", "0").parse().expect("--first");
    let seed: u64 = get("--seed", "20261111").parse().expect("--seed");
    let cap: u64 = get("--cap", "1000000").parse().expect("--cap");
    // walk at most this many admitted classes (the rest are counted and labelled only)
    let walk_limit: u64 = get("--walk-limit", "1000000").parse().expect("--walk-limit");
    let jumps: Vec<u64> = get("--jumps", "3,5,7")
        .split(',')
        .map(|v| v.parse().expect("--jumps"))
        .collect();
    let ctx = WalkCtx::new(p);
    let f = &ctx.f;
    let mut walked = 0u64;
    for i in first..first + n {
        let mut rng = StdRng::seed_from_u64(seed ^ i.wrapping_mul(0x9E37_79B9_7F4A_7C15));
        let t0 = Instant::now();
        f.reset_muls();
        let c = random_curve(f, &mut rng);
        let start_weak = c.weak_by_norms(f);
        let order = curve_order(f, &c, &mut rng);
        let muls_order = f.muls();
        let q3 = (p as i128).pow(6);
        let trace = q3 + 1 - order as i128;
        let admitted = trace_may_hold_weak_curve(p, trace);
        let (mut walk, mut found, mut path, mut muls_walk) = (false, false, 0usize, 0u64);
        if admitted && walked < walk_limit {
            walked += 1;
            walk = true;
            f.reset_muls();
            if let Some((w, steps)) = walk_to_weak_record(&ctx, c, &jumps, cap, trace, &mut rng) {
                found = w.weak_by_norms(f);
                path = steps.len();
            }
            muls_walk = f.muls();
        }
        println!(
            "{{\"p\":{p},\"i\":{i},\"seed\":{seed},\"order\":{order},\"trace\":{trace},\"start_weak\":{start_weak},\
             \"admitted\":{admitted},\"walked\":{walk},\"found\":{found},\"path\":{path},\"cap\":{cap},\
             \"jumps\":{jumps:?},\"muls_order\":{muls_order},\"muls_walk\":{muls_walk},\"ms\":{:.1}}}",
            t0.elapsed().as_secs_f64() * 1e3
        );
    }
}
