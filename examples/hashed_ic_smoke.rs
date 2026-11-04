//! Smoke run for the `hashed-affine` base and `hashed-tensor` oracle plug-ins.

use crypto_lib::cryptanalysis::ic_boundary::{
    CountedGroup, GroupOps, OracleCounters, PrimeCurve, PrimeInstance, PrimePoint,
};
use crypto_lib::cryptanalysis::ic_framework::hashed::{HashedAffineBase, HashedTensorOracle};
use crypto_lib::cryptanalysis::ic_framework::stages::{
    DecompositionOracle, FactorBaseBuilder, InstanceCtx, Params,
};

fn main() {
    let inst = PrimeInstance {
        name: "hashed-tiny".into(),
        curve: PrimeCurve { p: 5, a: 2, b: 1 },
        group_order: 7,
        r: 7,
        cofactor: 1,
        generator: (0, 1),
    };
    let mut ops = GroupOps::default();
    let generator = inst.generator_point();
    let target = inst.curve.mul(&mut ops, generator, 3);
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
    let fb = base.build(&ctx, &params, &mut ops).expect("base builds");
    println!(
        "hashed base: {} points, {} columns",
        fb.points.len(),
        fb.columns
    );
    let mut oracle = HashedTensorOracle::new();
    oracle
        .prepare(&ctx, &fb, &params, &mut ops)
        .expect("oracle prepares");
    let mut counters = OracleCounters::default();
    match oracle.decompose(&ctx, &fb, &mut ops, &mut counters, target) {
        Some(indices) => {
            let sum = indices.iter().fold(PrimePoint::INFINITY, |acc, &i| {
                inst.curve.add(&mut ops, acc, fb.points[i])
            });
            println!(
                "witness {} slots, replays to target: {}",
                indices.len(),
                sum == target
            );
        }
        None => println!("no witness for this target"),
    }
    println!("counters: {:?}", oracle.relation_counters());
}
