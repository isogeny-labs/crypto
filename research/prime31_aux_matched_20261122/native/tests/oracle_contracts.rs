use prime31_aux_matched::cryptanalysis::symmetrized_semaev::eval_s3_u64;
use prime31_aux_matched::oracle::{
    self, OracleLimits, OracleStatus, Presentation, TowerDescription,
};
use prime31_aux_matched::{CountedGroup, FactorBase, GroupOps, PrimeCurve, PrimePoint};
use std::collections::BTreeSet;

fn toy() -> (
    PrimeCurve,
    u64,
    PrimePoint,
    Vec<u64>,
    FactorBase<PrimePoint>,
    TowerDescription,
) {
    let curve = PrimeCurve { p: 17, a: 2, b: 2 };
    let r = 19;
    let g = PrimePoint::affine(5, 1);
    let v = vec![3, 5];
    let mut points = vec![];
    let mut cols = vec![];
    let mut coefs = vec![];
    for (column, &x) in v.iter().enumerate() {
        let y = curve.sqrt(curve.rhs(x)).unwrap();
        let y = y.min(curve.p - y);
        let q = PrimePoint::affine(x, y);
        points.extend([q, curve.neg(q)]);
        cols.extend([column, column]);
        coefs.extend([1, r - 1]);
    }
    let fb = FactorBase::from_column_map(
        "oracle-contract".into(),
        points,
        cols,
        coefs,
        2,
        |q| curve.key(q),
        |q| curve.key(&curve.neg(*q)),
        |q| q.x,
    )
    .unwrap();
    // Algebraic two-root Velu-shaped fibre x + 15/x = 8, roots {3,5}.
    // Auxiliary-group certification has separate tests in base_certificates.
    let tower = TowerDescription::Velu {
        steps: vec![(0, 15)],
        terminal: 8,
    };
    (curve, r, g, v, fb, tower)
}
fn limits() -> OracleLimits {
    OracleLimits {
        timeout_ms: 20_000,
        stop_after_first: false,
        max_degree: 8,
        ..Default::default()
    }
}
fn eval(poly: &[(Vec<u32>, u64)], point: &[u64], p: u64) -> u64 {
    poly.iter()
        .map(|(es, c)| {
            let mut v = *c;
            for (&e, &x) in es.iter().zip(point) {
                for _ in 0..e {
                    v = v * x % p
                }
            }
            v
        })
        .sum::<u64>()
        % p
}
fn reference(
    curve: &PrimeCurve,
    fb: &FactorBase<PrimePoint>,
    target: PrimePoint,
    m: usize,
) -> BTreeSet<Vec<i64>> {
    fn dfs(
        curve: &PrimeCurve,
        fb: &FactorBase<PrimePoint>,
        target: PrimePoint,
        left: usize,
        sum: PrimePoint,
        row: &mut Vec<i64>,
        out: &mut BTreeSet<Vec<i64>>,
        ops: &mut GroupOps,
    ) {
        if left == 0 {
            if sum == target {
                out.insert(row.clone());
            }
            return;
        }
        for (i, &q) in fb.points.iter().enumerate() {
            let sign = if fb.coef_of[i] == 1 { 1 } else { -1 };
            let col = fb.col_of[i];
            row[col] += sign;
            dfs(
                curve,
                fb,
                target,
                left - 1,
                curve.add(ops, sum, q),
                row,
                out,
                ops,
            );
            row[col] -= sign;
        }
    }
    let mut rows = BTreeSet::new();
    dfs(
        curve,
        fb,
        target,
        m,
        PrimePoint::INFINITY,
        &mut vec![0; fb.columns],
        &mut rows,
        &mut GroupOps::default(),
    );
    rows
}

#[test]
fn assembled_s3_matches_existing_numeric_polynomial() {
    let (curve, _, _, v, _, _) = toy();
    for xr in 0..curve.p {
        let (equations, _) = oracle::equations_for_chart(
            &curve,
            &v,
            None,
            PrimePoint::affine(xr, 0),
            2,
            Presentation::Dense,
            0,
            &limits(),
        )
        .unwrap()
        .unwrap();
        for x in 0..curve.p {
            for y in 0..curve.p {
                assert_eq!(
                    eval(equations.last().unwrap(), &[x, y], curve.p),
                    eval_s3_u64(x, y, xr, curve.a, curve.b, curve.p)
                );
            }
        }
    }
}

#[test]
fn every_signed_m2_m3_m4_witness_has_its_exact_prefix_chart() {
    let (curve, _, _, v, fb, _) = toy();
    let mut ops = GroupOps::default();
    for m in 2..=4 {
        for mut code in 0..fb.points.len().pow(m as u32) {
            let mut indices = vec![];
            for _ in 0..m {
                indices.push(code % fb.points.len());
                code /= fb.points.len();
            }
            let mut sum = fb.points[indices[0]];
            let mut mask = 0;
            let mut prefixes = vec![];
            for j in 1..m {
                sum = curve.add(&mut ops, sum, fb.points[indices[j]]);
                if j + 1 < m {
                    if sum.infinity {
                        mask |= 1 << (j - 1)
                    } else {
                        prefixes.push(sum.x);
                    }
                }
            }
            let (equations, n) = oracle::equations_for_chart(
                &curve,
                &v,
                None,
                sum,
                m,
                Presentation::Dense,
                mask,
                &limits(),
            )
            .unwrap()
            .unwrap();
            let mut witness: Vec<_> = indices.iter().map(|&i| fb.points[i].x).collect();
            witness.extend(prefixes);
            assert_eq!(witness.len(), n);
            assert!(
                equations.iter().all(|eq| eval(eq, &witness, curve.p) == 0),
                "m={m} mask={mask} indices={indices:?}"
            );
        }
    }
}

#[test]
fn m2_dense_and_tower_f4_recover_exact_signed_rows() {
    let (curve, r, g, v, fb, tower) = toy();
    let mut ops = GroupOps::default();
    for scalar in [0, 1, 2, 4, 8, 13] {
        let target = curve.mul(&mut ops, g, scalar);
        let expected = reference(&curve, &fb, target, 2);
        for presentation in [Presentation::Dense, Presentation::Tower] {
            let result = oracle::decompose(
                &curve,
                r,
                &fb,
                &v,
                Some(&tower),
                target,
                2,
                presentation,
                &limits(),
                &mut ops,
            )
            .unwrap();
            assert!(
                result.complete,
                "{}",
                serde_json::to_string(&result).unwrap()
            );
            assert_eq!(
                result
                    .relations
                    .iter()
                    .map(|r| r.row.clone())
                    .collect::<BTreeSet<_>>(),
                expected
            );
        }
    }
}

#[test]
fn m3_infinite_prefix_is_recovered_by_a_separate_algebraic_chart() {
    let (curve, r, g, v, fb, tower) = toy();
    for presentation in [Presentation::Dense, Presentation::Tower] {
        let result = oracle::decompose(
            &curve,
            r,
            &fb,
            &v,
            Some(&tower),
            g,
            3,
            presentation,
            &limits(),
            &mut GroupOps::default(),
        )
        .unwrap();
        assert!(
            result.complete,
            "{}",
            serde_json::to_string(&result).unwrap()
        );
        assert_eq!(
            result
                .relations
                .iter()
                .map(|r| r.row.clone())
                .collect::<BTreeSet<_>>(),
            reference(&curve, &fb, g, 3)
        );
        assert!(result
            .charts
            .iter()
            .any(|c| c.chart_mask == 1 && c.status == "complete"));
    }
}

#[test]
fn m4_all_prefix_charts_match_complete_signed_reference() {
    let (curve, r, g, v, fb, _) = toy();
    let target = curve.mul(&mut GroupOps::default(), g, 2);
    let result = oracle::decompose(
        &curve,
        r,
        &fb,
        &v,
        None,
        target,
        4,
        Presentation::Dense,
        &limits(),
        &mut GroupOps::default(),
    )
    .unwrap();
    assert!(
        result.complete,
        "{}",
        serde_json::to_string(&result).unwrap()
    );
    assert_eq!(
        result
            .relations
            .iter()
            .map(|r| r.row.clone())
            .collect::<BTreeSet<_>>(),
        reference(&curve, &fb, target, 4)
    );
    assert!(result
        .charts
        .iter()
        .any(|c| c.chart_mask == 3 && c.status.starts_with("impossible")));
}

#[test]
fn first_witness_and_resource_caps_keep_distinct_outcomes() {
    let (curve, r, g, v, fb, tower) = toy();
    let target = curve.mul(&mut GroupOps::default(), g, 2);
    let mut first = limits();
    first.stop_after_first = true;
    let found = oracle::decompose(
        &curve,
        r,
        &fb,
        &v,
        Some(&tower),
        target,
        2,
        Presentation::Tower,
        &first,
        &mut GroupOps::default(),
    )
    .unwrap();
    assert_eq!(found.status, OracleStatus::FoundFirst);
    assert!(found.positive);
    assert!(!found.complete);
    assert_eq!(found.relations.len(), 1);
    let mut capped = limits();
    capped.max_solutions = 0;
    let result = oracle::decompose(
        &curve,
        r,
        &fb,
        &v,
        Some(&tower),
        target,
        2,
        Presentation::Tower,
        &capped,
        &mut GroupOps::default(),
    )
    .unwrap();
    assert_eq!(result.status, OracleStatus::UnknownLimit);
    assert!(!result.complete);
    capped = limits();
    capped.timeout_ms = 0;
    let result = oracle::decompose(
        &curve,
        r,
        &fb,
        &v,
        Some(&tower),
        target,
        2,
        Presentation::Tower,
        &capped,
        &mut GroupOps::default(),
    )
    .unwrap();
    assert_eq!(result.status, OracleStatus::UnknownTimeout);
    assert!(!result.complete);
    let encoded = serde_json::to_string(&result).unwrap();
    let _: oracle::OracleResult = serde_json::from_str(&encoded).unwrap();
}

#[test]
fn malformed_factor_base_inventory_and_mismatched_tower_are_rejected() {
    let (curve, r, g, v, fb, tower) = toy();
    let check = |bad: &FactorBase<PrimePoint>, tower: &TowerDescription| {
        oracle::decompose(
            &curve,
            r,
            bad,
            &v,
            Some(tower),
            g,
            2,
            Presentation::Tower,
            &limits(),
            &mut GroupOps::default(),
        )
    };
    let mut bad = fb.clone();
    bad.x_index.clear();
    assert!(check(&bad, &tower).is_err());
    let mut bad = fb.clone();
    bad.x_index.get_mut(&v[0]).unwrap().push(usize::MAX);
    assert!(check(&bad, &tower).is_err());
    let mut bad = fb.clone();
    bad.point_index.clear();
    assert!(check(&bad, &tower).is_err());
    let mut bad = fb.clone();
    bad.neg_index[0] = 0;
    assert!(check(&bad, &tower).is_err());
    let mut bad = fb.clone();
    bad.coef_of.swap(0, 1);
    assert!(check(&bad, &tower).is_err());
    let mut bad = fb.clone();
    bad.abscissa_list.reverse();
    assert!(check(&bad, &tower).is_err());
    let bad_tower = TowerDescription::Velu {
        steps: vec![(0, 15)],
        terminal: 9,
    };
    assert!(check(&fb, &bad_tower).is_err());
}

#[test]
fn real_depth_two_fibres_have_identical_dense_and_tower_signed_extractions() {
    use prime31_aux_matched::bases::{build_base, BaseFamily};
    let curve = PrimeCurve { p: 127, a: 1, b: 7 };
    let order = 109;
    for family in [
        BaseFamily::Ordinary,
        BaseFamily::SsJ1728,
        BaseFamily::Dickson,
    ] {
        let base = build_base(&curve, order, 2, 0, family, 32).unwrap();
        let fb = base.factor_base().unwrap();
        assert!(!fb.points.is_empty());
        for m in [2, 3] {
            let target = curve.mul(&mut GroupOps::default(), fb.points[0], m as u64);
            let expected = reference(&curve, &fb, target, m);
            for presentation in [Presentation::Dense, Presentation::Tower] {
                let result = oracle::decompose(
                    &curve,
                    order,
                    &fb,
                    &base.v,
                    base.tower.as_ref(),
                    target,
                    m,
                    presentation,
                    &limits(),
                    &mut GroupOps::default(),
                )
                .unwrap();
                assert!(
                    result.complete,
                    "family={family:?} m={m}: {}",
                    serde_json::to_string(&result).unwrap()
                );
                assert_eq!(
                    result
                        .relations
                        .iter()
                        .map(|r| r.row.clone())
                        .collect::<BTreeSet<_>>(),
                    expected,
                    "family={family:?} m={m} presentation={presentation:?}"
                );
            }
        }
    }
}
