//! Contraction-timer micro-run for pinning `ns_per_hashed_tensor_step`.
//!
//! Prints one JSON line: primitive-step wall ratios on a roster prime curve,
//! using the same timer fields as `ic bench` (`hashed_tensor_contraction_wall_ns`,
//! `hashed_tensor_primitive_steps`).

use crypto_lib::cryptanalysis::ic_boundary::{
    CountedGroup, GroupOps, OracleCounters, roster_prime_instance,
};
use crypto_lib::cryptanalysis::ic_framework::hashed::{HashedAffineBase, HashedTensorOracle};
use crypto_lib::cryptanalysis::ic_framework::stages::{
    DecompositionOracle, FactorBaseBuilder, InstanceCtx, Params,
};
use serde_json::json;

fn main() {
    let bits: u32 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(16);
    let trials: u64 = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(32);

    let inst = roster_prime_instance(bits).expect("roster prime instance");
    let mut ops = GroupOps::default();
    let generator = inst.generator_point();
    let target = inst.curve.mul(&mut ops, generator, inst.r / 3 + 1);
    let ctx = InstanceCtx {
        group: &inst.curve,
        generator,
        target,
        r: inst.r,
        cofactor: inst.cofactor,
        group_order: inst.group_order,
        name: inst.name.clone(),
        field_degree: None,
    };
    let params = Params::default();
    let base = HashedAffineBase { instance: &inst };
    let fb = base.build(&ctx, &params, &mut ops).expect("hashed base");
    let mut oracle = HashedTensorOracle::new();
    oracle
        .prepare(&ctx, &fb, &params, &mut ops)
        .expect("oracle prepare");

    let mut counters = OracleCounters::default();
    for _ in 0..trials {
        let _ = oracle.decompose(&ctx, &fb, &mut ops, &mut counters, target);
    }
    let rel = oracle.relation_counters();
    let wall = rel.get("hashed_tensor_contraction_wall_ns").copied().unwrap_or(0);
    let steps = rel.get("hashed_tensor_primitive_steps").copied().unwrap_or(0);
    let ratio = if steps > 0 {
        wall as f64 / steps as f64
    } else {
        0.0
    };
    println!(
        "{}",
        serde_json::to_string(&json!({
            "instance": inst.name,
            "bits": bits,
            "trials": trials,
            "hashed_tensor_contraction_wall_ns": wall,
            "hashed_tensor_primitive_steps": steps,
            "ns_per_primitive_step_wall": ratio,
        }))
        .expect("json")
    );
}