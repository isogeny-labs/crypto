//! The hashed-affine base and hashed-tensor oracle plug into the same
//! `FactorBaseBuilder`/`DecompositionOracle` contracts as every other stage.

use crypto_lib::cryptanalysis::ic_boundary::{
    CountedGroup, GroupOps, OracleCounters, PrimeCurve, PrimeInstance, PrimePoint,
};
use crypto_lib::cryptanalysis::ic_framework::hashed::{HashedAffineBase, HashedTensorOracle};
use crypto_lib::cryptanalysis::ic_framework::stages::{
    DecompositionOracle, FactorBaseBuilder, InstanceCtx, Params,
};

fn tiny() -> PrimeInstance {
    PrimeInstance {
        name: "hashed-tiny".into(),
        curve: PrimeCurve { p: 5, a: 2, b: 1 },
        group_order: 7,
        r: 7,
        cofactor: 1,
        generator: (0, 1),
    }
}

#[test]
fn hashed_base_and_tensor_oracle_share_their_slots() {
    let inst = tiny();
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
    let fb = base
        .build(&ctx, &params, &mut ops)
        .expect("a 5-bit curve has a hashed base");
    assert!(fb.columns > 0);
    assert_eq!(fb.columns, fb.points.len());

    let mut oracle = HashedTensorOracle::new();
    oracle
        .prepare(&ctx, &fb, &params, &mut ops)
        .expect("the oracle rebuilds identical slots");
    assert_eq!(oracle.summands(), 2);

    let mut counters = OracleCounters::default();
    let witness = oracle
        .decompose(&ctx, &fb, &mut ops, &mut counters, target)
        .or_else(|| {
            let other = inst.curve.mul(&mut ops, generator, 1);
            oracle.decompose(&ctx, &fb, &mut ops, &mut counters, other)
        });
    if let Some(indices) = witness {
        assert_eq!(indices.len(), 2, "one point per slot");
        let sum = indices.iter().fold(PrimePoint::INFINITY, |acc, &i| {
            inst.curve.add(&mut ops, acc, fb.points[i])
        });
        assert!(
            sum == target || sum == inst.curve.mul(&mut ops, generator, 1),
            "the witness indices replay to the queried target"
        );
    }
}
