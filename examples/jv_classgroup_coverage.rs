//! **Coverage of the JV weak class by the class-group isogeny walk**
//! (research/jv_classgroup_walk_20261112).
//!
//! Same measurement as `examples/jv_weak_coverage` but the odd-degree steps use
//! every certified cyclic ℓ-isogeny (Vélu through a kernel polynomial,
//! [`walk_to_weak_classgroup`]), not only the rational-kernel ones.  For `n`
//! random full-2-torsion curves over F_{q³} that pass the proved necessary
//! trace condition (admitted), run the class-group walk to a weak curve, cap
//! `cap` distinct curves.  One JSON line per curve.
//!
//! ```text
//! cargo run --release --example jv_classgroup_coverage -- --p 1009 --n 400 --seed 20261111 --cap 10000 --jumps 3,5,7 [--walk-limit 60]
//! ```

use std::env;
use std::time::Instant;

use crypto_lib::cryptanalysis::jv_isogeny_walk::{
    curve_order, random_curve, trace_may_hold_weak_curve, walk_to_weak_classgroup, WalkCtx,
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
    let n: u64 = get("--n", "400").parse().expect("--n");
    let first: u64 = get("--first", "0").parse().expect("--first");
    let seed: u64 = get("--seed", "20261111").parse().expect("--seed");
    let cap: u64 = get("--cap", "10000").parse().expect("--cap");
    let walk_limit: u64 = get("--walk-limit", "1000000")
        .parse()
        .expect("--walk-limit");
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
        let (mut walk, mut found, mut p2, mut po, mut muls_walk) =
            (false, false, 0usize, 0usize, 0u64);
        if admitted && walked < walk_limit {
            walked += 1;
            walk = true;
            f.reset_muls();
            if let Some((_w, two, odd)) =
                walk_to_weak_classgroup(&ctx, c, &jumps, cap, trace, &mut rng)
            {
                found = true;
                p2 = two;
                po = odd;
            }
            muls_walk = f.muls();
        }
        println!(
            "{{\"p\":{p},\"i\":{i},\"seed\":{seed},\"order\":{order},\"trace\":{trace},\"start_weak\":{start_weak},\
             \"admitted\":{admitted},\"walked\":{walk},\"found\":{found},\"path_two\":{p2},\"path_odd\":{po},\
             \"cap\":{cap},\"jumps\":{jumps:?},\"muls_order\":{muls_order},\"muls_walk\":{muls_walk},\"ms\":{:.1}}}",
            t0.elapsed().as_secs_f64() * 1e3
        );
    }
}
