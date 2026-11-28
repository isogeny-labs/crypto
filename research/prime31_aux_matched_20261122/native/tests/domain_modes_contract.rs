//! Current-production F4 option wiring and small-field solution-set equality.
//! This is a correctness fixture, with no performance assertion.

use prime31_aux_matched::oracle::{self, DomainRoots, OracleLimits, Presentation};
use prime31_aux_matched::replay::{Curve, Point};
use prime31_aux_matched::{CountedGroup, FactorBase, GroupOps, PrimeCurve, PrimePoint};
use std::collections::BTreeSet;

#[test]
fn current_domain_modes_preserve_complete_smallfield_signed_rows() {
    let curve = PrimeCurve { p: 17, a: 2, b: 2 };
    let replay = Curve { p: 17, a: 2, b: 2 };
    let order = 19;
    let target = PrimePoint::affine(5, 1);
    let values = vec![3, 5];
    let mut points = Vec::new();
    let mut columns = Vec::new();
    let mut coefficients = Vec::new();
    for (column, &x) in values.iter().enumerate() {
        let y = curve.sqrt(curve.rhs(x)).unwrap();
        let point = PrimePoint::affine(x, y.min(curve.p - y));
        points.extend([point, curve.neg(point)]);
        columns.extend([column, column]);
        coefficients.extend([1, order - 1]);
    }
    let base = FactorBase::from_column_map(
        "domain-mode-correctness-p17".into(),
        points,
        columns,
        coefficients,
        values.len(),
        |q| curve.key(q),
        |q| curve.key(&curve.neg(*q)),
        |q| q.x,
    )
    .unwrap();

    // Enumerate the 4^3 signed triples with the independent affine verifier.
    let mut expected = BTreeSet::new();
    for i in 0..base.points.len() {
        for j in 0..base.points.len() {
            for k in 0..base.points.len() {
                let mut sum = Point::IDENTITY;
                let mut row = vec![0i64; base.columns];
                for index in [i, j, k] {
                    let q = base.points[index];
                    sum = replay.add(sum, Point::affine(q.x, q.y)).unwrap();
                    row[base.col_of[index]] += if base.coef_of[index] == 1 { 1 } else { -1 };
                }
                if sum == Point::affine(target.x, target.y) {
                    expected.insert(row);
                }
            }
        }
    }
    assert!(!expected.is_empty());

    let mut forced_domain_extractions = 0;
    for (domain_roots, sibling_batch) in [
        (DomainRoots::Scan, false),
        (DomainRoots::Auto, true),
        // Auto can correctly prefer scanning a two-element domain. This
        // forced arm checks that the explicit policy reaches production F4.
        (DomainRoots::Frobenius, true),
    ] {
        let limits = OracleLimits {
            timeout_ms: 20_000,
            stop_after_first: false,
            max_degree: 8,
            domain_roots,
            sibling_batch,
            ..Default::default()
        };
        let result = oracle::decompose(
            &curve,
            order,
            &base,
            &values,
            None,
            target,
            3,
            Presentation::Dense,
            &limits,
            &mut GroupOps::default(),
        )
        .unwrap();
        assert!(
            result.complete,
            "{}",
            serde_json::to_string(&result).unwrap()
        );
        let rows = result
            .relations
            .iter()
            .map(|relation| relation.row.clone())
            .collect::<BTreeSet<_>>();
        assert_eq!(rows, expected);
        if domain_roots == DomainRoots::Frobenius {
            forced_domain_extractions = result
                .charts
                .iter()
                .map(|chart| {
                    chart.visit["domain_frobenius_univariates"]
                        .as_u64()
                        .unwrap_or(0)
                })
                .sum::<u64>();
        }
    }
    assert!(
        forced_domain_extractions > 0,
        "the configured root policy did not reach F4"
    );
}
