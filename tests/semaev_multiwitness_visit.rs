use crypto_lib::cryptanalysis::pkm_smooth_tower::{sqrtm, Curve, Pt};
use crypto_lib::cryptanalysis::semaev_projection::{interval_factor_base, PairProjection};

fn reaches(curve: &Curve, xs: &[u64], target: u64, at: usize, sum: Pt) -> bool {
    if at == xs.len() {
        return sum.is_some_and(|(x, _)| x == target);
    }
    let x = xs[at];
    let y = sqrtm(curve.rhs(x), curve.p).expect("factor-base point");
    let point = Some((x, y));
    [point, curve.neg(point)]
        .into_iter()
        .any(|signed| reaches(curve, xs, target, at + 1, curve.add(sum, signed)))
}

#[test]
fn visit_counts_replays_and_stops() {
    let curve = Curve { p: 101, a: 2, b: 3 };
    let base = interval_factor_base(&curve, 30);
    for projection in [
        PairProjection::build(curve, &base).unwrap(),
        PairProjection::build_fast(curve, &base).unwrap(),
    ] {
        for target in 0..curve.p {
            let mut all = Vec::new();
            let stats = projection
                .decompose_four_visit(target, |witness| {
                    all.push(witness);
                    true
                })
                .unwrap();
            let (_, reference) = projection.decompose_four_exhaustive(target);
            assert_eq!(stats.compatible_keys, reference.compatible_keys);
            assert!(all
                .iter()
                .all(|witness| reaches(&curve, &witness.x, target, 0, None)));
            let mut first = None;
            projection
                .decompose_four_visit(target, |witness| {
                    first = Some(witness);
                    false
                })
                .unwrap();
            assert_eq!(first, projection.decompose_four(target).0);
        }
    }
    let generic = PairProjection::build_generic(curve, &base).unwrap();
    assert!(generic.decompose_four_visit(0, |_| true).is_err());
}
