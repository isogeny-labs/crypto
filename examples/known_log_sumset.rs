//! Pre-registered hit-rate prediction for a known-log base
//! (research/generator_effect_ic_20261019/PLAN.md, Amendment 1).
//!
//! `cargo run --release --example known_log_sumset -- P A B R GX GY SIZE SCALARS [SEED]`
//!
//! Builds the base exactly as the `known-log` factor base does, then counts
//! `|2ΣS|`, the distinct nonzero values `±s_i ± s_j` (i ≤ j, mod r) that the
//! negation-folded pair table of `mitm:m=2` can match, and prints
//! `|2ΣS| / r`: the predicted relations per target.  Counted before any run.
use crypto_lib::cryptanalysis::ic_boundary::{CountedGroup, GroupOps, PrimeCurve, PrimeInstance};
use crypto_lib::cryptanalysis::ic_framework::plugins::KnownLogBase;
use crypto_lib::cryptanalysis::ic_framework::stages::{FactorBaseBuilder, InstanceCtx, Params};
use std::collections::HashSet;

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let n = |i: usize| -> u64 { a[i].parse().expect("integer") };
    let (p, ca, cb, r, gx, gy, size) = (n(0), n(1), n(2), n(3), n(4), n(5), n(6));
    let inst = PrimeInstance {
        name: "known-log-sumset".into(),
        curve: PrimeCurve { p, a: ca, b: cb },
        group_order: r,
        r,
        cofactor: 1,
        generator: (gx, gy),
    };
    let g = inst.generator_point();
    let ctx = InstanceCtx {
        group: &inst.curve,
        generator: g,
        target: g,
        r,
        cofactor: 1,
        group_order: r,
        name: inst.name.clone(),
        field_degree: None,
    };
    let mut spec = format!("known-log:size={size},scalars={}", a[7]);
    if let Some(seed) = a.get(8) {
        spec.push_str(&format!(",seed={seed}"));
    }
    let (_, params) = Params::parse_spec(&spec).unwrap();
    let mut ops = GroupOps::default();
    let fb = KnownLogBase.build(&ctx, &params, &mut ops).unwrap();
    let s = fb.known_logs.unwrap();
    let neg = |x: u64| (r - x) % r;
    let mut sums: HashSet<u64> = HashSet::new();
    for i in 0..s.len() {
        for j in i..s.len() {
            for (x, y) in [
                (s[i], s[j]),
                (s[i], neg(s[j])),
                (neg(s[i]), s[j]),
                (neg(s[i]), neg(s[j])),
            ] {
                let t = (x + y) % r;
                if t != 0 {
                    sums.insert(t);
                }
            }
        }
    }
    let _ = CountedGroup::is_identity(&inst.curve, &g);
    println!(
        "{{\"spec\": \"{spec}\", \"r\": {r}, \"size\": {size}, \"sumset\": {}, \"predicted_hit_rate\": {:e}}}",
        sums.len(),
        sums.len() as f64 / r as f64
    );
}
