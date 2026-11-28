//! Fixed small-field correctness checks for degree-four membership blocking.
//! No Prime31 campaign, timing comparison, or large-field relation search.
use prime31_aux_matched::bases::{
    build_base, independent_add, BaseDefinition, BaseFamily, CurveDefinition,
};
use prime31_aux_matched::oracle::{self, OracleLimits, Presentation, TowerDescription};
use prime31_aux_matched::{CountedGroup, FactorBase, GroupOps, PrimeCurve, PrimePoint};
use std::collections::BTreeSet;

fn curve() -> PrimeCurve {
    PrimeCurve { p: 127, a: 1, b: 7 }
}
fn base(family: BaseFamily, depth: u32) -> BaseDefinition {
    build_base(&curve(), 109, depth, 0, family, 32).unwrap()
}
fn limits() -> OracleLimits {
    OracleLimits {
        timeout_ms: 20_000,
        stop_after_first: false,
        max_degree: 16,
        ..Default::default()
    }
}
fn eval(poly: &[(Vec<u32>, u64)], point: &[u64], p: u64) -> u64 {
    poly.iter().fold(0, |sum, (powers, coefficient)| {
        let value = powers
            .iter()
            .zip(point)
            .fold(*coefficient, |mut v, (&e, &x)| {
                for _ in 0..e {
                    v = v * x % p;
                }
                v
            });
        (sum + value) % p
    })
}
fn power(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut b = 1;
    while n != 0 {
        if n & 1 != 0 {
            b = b * a % p;
        }
        a = a * a % p;
        n >>= 1;
    }
    b
}

/// Evaluate the ORIGINAL individual maps, not the new composed formula.
fn path(tower: &TowerDescription, leaf: u64, p: u64) -> Option<Vec<u64>> {
    let mut values = vec![leaf];
    match tower {
        TowerDescription::Velu { steps, .. } => {
            let mut x = leaf;
            for &(xi, tau) in steps {
                let d = (x + p - xi) % p;
                if d == 0 {
                    return None;
                }
                x = (x + tau * power(d, p - 2, p)) % p;
                values.push(x);
            }
        }
        TowerDescription::Dickson { depth, shift, .. } => {
            let mut x = (leaf + p - shift) % p;
            for _ in 0..*depth {
                x = (x * x + p - 2) % p;
                values.push(x);
            }
        }
    }
    Some(values)
}
fn terminal(tower: &TowerDescription) -> u64 {
    match tower {
        TowerDescription::Velu { terminal, .. } | TowerDescription::Dickson { terminal, .. } => {
            *terminal
        }
    }
}
fn exact_rows(
    curve: &PrimeCurve,
    fb: &FactorBase<PrimePoint>,
    target: PrimePoint,
    m: usize,
) -> BTreeSet<Vec<i64>> {
    let independent = CurveDefinition::from(curve);
    let mut rows = BTreeSet::new();
    for mut code in 0..fb.points.len().pow(m as u32) {
        let mut sum = PrimePoint::INFINITY;
        let mut row = vec![0i64; fb.columns];
        for _ in 0..m {
            let i = code % fb.points.len();
            code /= fb.points.len();
            sum = independent_add(&independent, sum.into(), fb.points[i].into()).production();
            row[fb.col_of[i]] += if fb.coef_of[i] == 1 { 1 } else { -1 };
        }
        if sum == target {
            rows.insert(row);
        }
    }
    rows
}

#[test]
fn quartic_membership_has_exactly_the_original_full_fibre_and_no_pole_roots() {
    let curve = curve();
    for family in [
        BaseFamily::Ordinary,
        BaseFamily::SsJ1728,
        BaseFamily::Dickson,
    ] {
        let base = base(family, 2);
        let tower = base.tower.as_ref().unwrap();
        let (equations, n) = oracle::equations_for_chart(
            &curve,
            &base.v,
            Some(tower),
            PrimePoint::INFINITY,
            2,
            Presentation::Tower4,
            0,
            &limits(),
        )
        .unwrap()
        .unwrap();
        assert_eq!(n, 2);
        assert_eq!(equations.len(), 3);
        assert!(equations[0].iter().all(|(e, _)| e.iter().sum::<u32>() <= 4));
        let mut roots = vec![];
        for x in 0..curve.p {
            let original = path(tower, x, curve.p);
            let expected = original
                .as_ref()
                .is_some_and(|v| *v.last().unwrap() == terminal(tower));
            let actual = eval(&equations[0], &[x, 0], curve.p) == 0;
            assert_eq!(actual, expected, "family={family:?}, x={x}");
            if original.is_none() {
                assert!(
                    !actual,
                    "denominator clearing admitted an original-chain pole"
                );
            }
            if actual {
                roots.push(x);
            }
        }
        assert_eq!(roots, base.v, "complete full fibre, including nonlifting x");
    }
}

#[test]
fn odd_depth_keeps_the_last_quadratic_and_no_extra_intermediate_solutions() {
    let curve = curve();
    for family in [BaseFamily::SsJ1728, BaseFamily::Dickson] {
        let base = base(family, 3);
        let tower = base.tower.as_ref().unwrap();
        let (equations, n) = oracle::equations_for_chart(
            &curve,
            &base.v,
            Some(tower),
            PrimePoint::INFINITY,
            2,
            Presentation::Tower4,
            0,
            &limits(),
        )
        .unwrap()
        .unwrap();
        assert_eq!(n, 4);
        assert_eq!(equations.len(), 5);
        assert!(equations[1].iter().all(|(e, _)| e.iter().sum::<u32>() <= 2));
        let mut roots = vec![];
        for x in 0..curve.p {
            let accepted: Vec<_> = (0..curve.p)
                .filter(|&middle| {
                    let point = [x, 0, middle, 0];
                    equations[..2].iter().all(|q| eval(q, &point, curve.p) == 0)
                })
                .collect();
            let original = path(tower, x, curve.p);
            let expected = match original {
                Some(v) if *v.last().unwrap() == terminal(tower) => vec![v[2]],
                _ => vec![],
            };
            assert_eq!(accepted, expected, "family={family:?}, x={x}");
            if !accepted.is_empty() {
                roots.push(x);
            }
        }
        assert_eq!(roots, base.v);
    }
}

#[test]
fn tower4_real_f4_signed_rows_equal_dense_tower_and_independent_group_reference() {
    let curve = curve();
    for family in [BaseFamily::SsJ1728, BaseFamily::Dickson] {
        let base = base(family, 2);
        let fb = base.factor_base().unwrap();
        assert!(!fb.points.is_empty());
        for target in [
            PrimePoint::INFINITY,
            curve.mul(&mut GroupOps::default(), fb.points[0], 2),
        ] {
            let expected = exact_rows(&curve, &fb, target, 2);
            for presentation in [
                Presentation::Dense,
                Presentation::Tower,
                Presentation::Tower4,
            ] {
                let result = oracle::decompose(
                    &curve,
                    109,
                    &fb,
                    &base.v,
                    base.tower.as_ref(),
                    target,
                    2,
                    presentation,
                    &limits(),
                    &mut GroupOps::default(),
                )
                .unwrap();
                assert!(
                    result.complete,
                    "family={family:?}, presentation={presentation:?}: {}",
                    serde_json::to_string(&result).unwrap()
                );
                let found = result
                    .relations
                    .iter()
                    .map(|r| r.row.clone())
                    .collect::<BTreeSet<_>>();
                assert_eq!(
                    found, expected,
                    "family={family:?}, presentation={presentation:?}"
                );
            }
        }
    }
}

#[test]
fn tower4_identity_prefix_chart_extracts_exact_signed_rows() {
    let curve = curve();
    let base = base(BaseFamily::SsJ1728, 2);
    let fb = base.factor_base().unwrap();
    let target = fb.points[0];
    let result = oracle::decompose(
        &curve,
        109,
        &fb,
        &base.v,
        base.tower.as_ref(),
        target,
        3,
        Presentation::Tower4,
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
        exact_rows(&curve, &fb, target, 3),
    );
    assert!(result
        .charts
        .iter()
        .any(|c| c.chart_mask == 1 && c.status == "complete"));
    let encoded = serde_json::to_string(&result).unwrap();
    let decoded: oracle::OracleResult = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded.presentation, Presentation::Tower4);
}

#[test]
fn every_signed_m4_witness_retains_its_identity_chart_with_blocked_variables() {
    let curve = curve();
    let independent = CurveDefinition::from(&curve);
    let base = base(BaseFamily::SsJ1728, 3);
    let fb = base.factor_base().unwrap();
    let tower = base.tower.as_ref().unwrap();
    let m = 4;
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
            sum = independent_add(&independent, sum.into(), fb.points[indices[j]].into())
                .production();
            if j + 1 < m {
                if sum.infinity {
                    mask |= 1 << (j - 1);
                } else {
                    prefixes.push(sum.x);
                }
            }
        }
        let (equations, n) = oracle::equations_for_chart(
            &curve,
            &base.v,
            Some(tower),
            sum,
            m,
            Presentation::Tower4,
            mask,
            &limits(),
        )
        .unwrap()
        .unwrap();
        let mut witness: Vec<_> = indices.iter().map(|&i| fb.points[i].x).collect();
        witness.extend(
            indices
                .iter()
                .map(|&i| path(tower, fb.points[i].x, curve.p).unwrap()[2]),
        );
        witness.extend(prefixes);
        assert_eq!(witness.len(), n);
        assert_eq!(n, 2 * m + (m - 2) - mask.count_ones() as usize);
        assert!(
            equations.iter().all(|q| eval(q, &witness, curve.p) == 0),
            "mask={mask}, indices={indices:?}"
        );
    }
}

#[test]
fn tower4_validates_the_entire_original_chain_and_rejects_original_poles() {
    let curve = curve();
    let base = base(BaseFamily::SsJ1728, 3);
    let fb = base.factor_base().unwrap();
    let original = base.tower.as_ref().unwrap();
    let check = |tower: &TowerDescription| {
        oracle::decompose(
            &curve,
            109,
            &fb,
            &base.v,
            Some(tower),
            PrimePoint::INFINITY,
            2,
            Presentation::Tower4,
            &limits(),
            &mut GroupOps::default(),
        )
    };
    let mut bad = original.clone();
    if let TowerDescription::Velu { steps, .. } = &mut bad {
        steps[1].0 = path(original, base.v[0], curve.p).unwrap()[1];
    }
    assert!(check(&bad).unwrap_err().contains("pole"));
    let mut bad = original.clone();
    if let TowerDescription::Velu { steps, .. } = &mut bad {
        steps[2].1 = 0;
    }
    assert!(check(&bad).unwrap_err().contains("invalid canonical Velu"));
    let mut bad = original.clone();
    if let TowerDescription::Velu { terminal, .. } = &mut bad {
        *terminal = (*terminal + 1) % curve.p;
    }
    assert!(check(&bad)
        .unwrap_err()
        .contains("complete retained-tower fibre"));
}
