//! Crypto-specific root diagnostics; the mirror has no Meter root counter.
use hashed_ecdl_tensor::curve::{Curve, Point};
use hashed_ecdl_tensor::prepared::{PairTable, Statistics};
use hashed_ecdl_tensor::reduction::{FactorBase, HashSeed};
use hashed_ecdl_tensor::{Limits, Meter};

#[test]
fn prepared_queries_make_no_new_root_requests() {
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
    let bases: Vec<_> = [12u8, 24]
        .into_iter()
        .map(|threshold| {
            FactorBase::new(
                &curve,
                37u8.into(),
                threshold.into(),
                HashSeed {
                    alpha: 3u8.into(),
                    beta: 4u8.into(),
                },
                &mut setup,
            )
            .unwrap()
        })
        .collect();
    let mut statistics = Statistics::default();
    let table = PairTable::new(&curve, &bases, &mut statistics, &mut setup).unwrap();
    assert!(setup.square_roots > 0);
    let mut query_work = Meter::new(Limits::default());
    for k in 0..19u8 {
        let target = curve.mul(curve.generator(), &k.into(), &mut setup).unwrap();
        table
            .decompose(&target, true, &mut statistics, &mut query_work)
            .unwrap();
    }
    assert_eq!(statistics.preparations, 1);
    assert_eq!(statistics.queries, 19);
    assert_eq!(query_work.square_roots, 0);
}
