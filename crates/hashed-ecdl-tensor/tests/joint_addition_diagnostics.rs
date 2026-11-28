//! Development operation controls only; no runtime or production-curve claim.
use hashed_ecdl_tensor::arithmetic::Seeded;
use hashed_ecdl_tensor::curve::{Curve, Point};
use hashed_ecdl_tensor::joint::JointTable;
use hashed_ecdl_tensor::prepared::{self, Statistics};
use hashed_ecdl_tensor::reduction::Config;
use hashed_ecdl_tensor::{BigUint, Limits, Meter};

#[test]
fn dense_131_bit_public_scalars_share_the_doubling_chain() {
    let mut setup = Meter::new(Limits::default());
    let curve = Curve::new(
        17u8.into(),
        2u8.into(),
        2u8.into(),
        19u8.into(),
        Point::Affine {
            x: 5u8.into(),
            y: 1u8.into(),
        },
        &mut setup,
    )
    .unwrap();
    let q = curve
        .mul(curve.generator(), &3u8.into(), &mut setup)
        .unwrap();
    let table = JointTable::new(&curve, curve.generator(), &q, &mut setup).unwrap();
    let scalar = (BigUint::from(1u8) << 131) - 1u8;
    let mut control = Meter::new(Limits::default());
    let p = curve.mul(curve.generator(), &scalar, &mut control).unwrap();
    let q = curve.mul(&q, &scalar, &mut control).unwrap();
    let expected = curve.add(&p, &q, &mut control).unwrap();
    let mut candidate = Meter::new(Limits::default());
    assert_eq!(
        table.evaluate(&scalar, &scalar, &mut candidate).unwrap(),
        expected
    );
    assert_eq!(control.curve_additions, 523);
    assert_eq!(candidate.curve_additions, 260);
    // Setup is deliberately kept distinct, including constructor subgroup checks.
    println!("F17 dense 131-bit scalars: independent_query_additions=523 joint_query_additions=260; setup separately metered; no runtime measurement");
}

#[test]
fn complete_fixture_collector_reduces_additions_including_joint_setup() {
    let mut setup = Meter::new(Limits::default());
    let curve = Curve::new(
        17u8.into(),
        2u8.into(),
        2u8.into(),
        19u8.into(),
        Point::Affine {
            x: 5u8.into(),
            y: 1u8.into(),
        },
        &mut setup,
    )
    .unwrap();
    let target = curve
        .mul(curve.generator(), &3u8.into(), &mut setup)
        .unwrap();
    let config = Config {
        slots: 2,
        threshold: Some(12u8.into()),
        columns: 64,
        trials: 32,
        ..Config::default()
    };
    for seed in [1, 42, 83] {
        let mut control = Meter::new(Limits::default());
        let mut candidate = Meter::new(Limits::default());
        let old = prepared::solve(
            &curve,
            &target,
            &config,
            &mut Seeded(seed),
            true,
            &mut Statistics::default(),
            &mut control,
        )
        .unwrap();
        let new = prepared::solve_joint(
            &curve,
            &target,
            &config,
            &mut Seeded(seed),
            true,
            &mut Statistics::default(),
            &mut candidate,
        )
        .unwrap();
        assert_eq!(
            serde_json::to_value(&old).unwrap(),
            serde_json::to_value(&new).unwrap()
        );
        assert!(candidate.curve_additions < control.curve_additions);
        println!("F17 whole collector seed={seed}: independent_additions={} joint_additions={} (joint setup and unchanged final verification included; no runtime measurement)", control.curve_additions, candidate.curve_additions);
    }
}
