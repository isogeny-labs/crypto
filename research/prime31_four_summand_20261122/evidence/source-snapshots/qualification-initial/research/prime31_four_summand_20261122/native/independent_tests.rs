//! Independent arithmetic controls for the public RR5 extraction boundary.
//!
//! Tuple enumeration below is confined to this test oracle. The candidate
//! receives only the curve, public base, public target, and frozen limits.
//! This file does not call the candidate's curve arithmetic or polynomial
//! builder to compute its expected signed relations.

use crate::extractor::{extract, Limits, Status};
use prime31_algebraic_ic::{Curve, PRIME31};
use std::collections::BTreeSet;

type Affine = Option<(u64, u64)>;
type SignedRows = BTreeSet<Vec<(usize, i64)>>;

fn product(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn power(mut a: u64, mut exponent: u64, p: u64) -> u64 {
    let mut result = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = product(result, a, p);
        }
        a = product(a, a, p);
        exponent >>= 1;
    }
    result
}

fn rhs(e: Curve, x: u64) -> u64 {
    (product(product(x, x, e.p), x, e.p) + product(e.a, x, e.p) + e.b) % e.p
}

fn on_curve(e: Curve, q: Affine) -> bool {
    q.is_none_or(|(x, y)| x < e.p && y < e.p && product(y, y, e.p) == rhs(e, x))
}

fn negative(e: Curve, q: Affine) -> Affine {
    q.map(|(x, y)| (x, (e.p - y) % e.p))
}

fn addition(e: Curve, p: Affine, q: Affine) -> Affine {
    let (Some((x1, y1)), Some((x2, y2))) = (p, q) else {
        return p.or(q);
    };
    if x1 == x2 && (y1 + y2) % e.p == 0 {
        return None;
    }
    let (numerator, denominator) = if x1 == x2 {
        assert_eq!(y1, y2);
        ((3 * product(x1, x1, e.p) + e.a) % e.p, 2 * y1 % e.p)
    } else {
        ((y2 + e.p - y1) % e.p, (x2 + e.p - x1) % e.p)
    };
    assert_ne!(denominator, 0);
    let slope = product(numerator, power(denominator, e.p - 2, e.p), e.p);
    let x3 = (product(slope, slope, e.p) + 2 * e.p - x1 - x2) % e.p;
    let y3 = (product(slope, (x1 + e.p - x3) % e.p, e.p) + e.p - y1) % e.p;
    let result = Some((x3, y3));
    assert!(on_curve(e, result));
    result
}

fn scalar(e: Curve, mut k: u64, mut point: Affine) -> Affine {
    let mut result = None;
    while k != 0 {
        if k & 1 != 0 {
            result = addition(e, result, point);
        }
        point = addition(e, point, point);
        k >>= 1;
    }
    result
}

fn sum_row(e: Curve, base: &[(u64, u64)], terms: &[(usize, i64)]) -> Affine {
    let mut result = None;
    for &(index, sign) in terms {
        assert!(sign == 1 || sign == -1);
        let point = if sign == 1 {
            Some(base[index])
        } else {
            negative(e, Some(base[index]))
        };
        result = addition(e, result, point);
    }
    result
}

fn oracle_b4(e: Curve, base: &[(u64, u64)], q: Affine) -> SignedRows {
    assert_eq!(base.len(), 4);
    let mut rows = BTreeSet::new();
    for mask in 0..16 {
        let row: Vec<_> = (0..4)
            .map(|index| (index, if mask & (1 << index) == 0 { 1 } else { -1 }))
            .collect();
        if sum_row(e, base, &row) == q {
            rows.insert(row);
        }
    }
    rows
}

fn compare(e: Curve, base: &[(u64, u64)], q: Affine, label: &str) -> usize {
    let expected = oracle_b4(e, base, q);
    let actual = extract(e, base, q, Limits::default());
    println!(
        "INDEPENDENT_RR5 {}",
        serde_json::json!({
            "label": label,
            "public_target": q,
            "expected_rows": &expected,
            "candidate": &actual,
            "oracle_tuple_evaluations": 16,
            "candidate_received_oracle_rows": false,
        })
    );
    assert_eq!(
        actual.status,
        if expected.is_empty() {
            Status::Empty
        } else {
            Status::Complete
        },
        "{label}: extraction must finish or honestly fail the correctness gate"
    );
    let got: SignedRows = actual
        .relations
        .iter()
        .map(|row| row.terms.clone())
        .collect();
    assert_eq!(got, expected, "{label}: complete signed row-set parity");
    let trace = actual
        .traversal
        .as_ref()
        .expect("eligible B4 target has a traversal");
    assert_eq!(trace.full_field_branches, 0);
    assert_eq!(trace.domain_evaluations, 0);
    assert!(!trace.trace_stopped && !trace.timed_out && trace.complete);
    for row in &got {
        assert_eq!(sum_row(e, base, row), q);
    }
    got.len()
}

#[test]
fn independent_all_finite_targets_on_ordinary_f11_curve() {
    let e = Curve {
        p: 11,
        a: 1,
        b: 6,
        n: 13,
        g: (2, 7),
    };
    let base = [(2, 4), (3, 5), (5, 2), (7, 2)];
    assert!(base.iter().all(|&point| on_curve(e, Some(point))));
    let mut finite = Vec::new();
    for x in 0..e.p {
        for y in 0..e.p {
            if on_curve(e, Some((x, y))) {
                finite.push((x, y));
            }
        }
    }
    assert_eq!(finite.len(), 12);
    assert!(on_curve(e, Some(e.g)));
    assert_eq!(scalar(e, e.n, Some(e.g)), None);
    // trace = p+1-#E = -1, so this is ordinary, not a supersingular control.
    let mut eligible = 0;
    let mut excluded = 0;
    let mut signed_rows = 0;
    for point in finite {
        if base.iter().any(|&(x, _)| x == point.0) {
            assert_eq!(
                extract(e, &base, Some(point), Limits::default()).status,
                Status::UnsupportedChart
            );
            excluded += 1;
        } else {
            signed_rows += compare(e, &base, Some(point), "ordinary_f11_all_finite");
            eligible += 1;
        }
    }
    assert_eq!((eligible, excluded), (4, 8));
    assert!(signed_rows > 0);
    assert_eq!(
        extract(e, &base, None, Limits::default()).status,
        Status::UnsupportedChart
    );
}

#[test]
fn independent_prime31_b4_target_and_its_negative() {
    let e = PRIME31;
    let mut base = Vec::new();
    for x in 0..4096 {
        let r = rhs(e, x);
        let y = power(r, (e.p + 1) / 4, e.p);
        if y != 0 && product(y, y, e.p) == r {
            base.push((x, y.min(e.p - y)));
        }
        if base.len() == 4 {
            break;
        }
    }
    assert_eq!(base.len(), 4);
    let witness = [(0, 1), (1, -1), (2, 1), (3, 1)];
    let q = sum_row(e, &base, &witness);
    assert!(q.is_some());
    assert!(!base.iter().any(|&(x, _)| x == q.unwrap().0));
    assert!(compare(e, &base, q, "prime31_independent_positive") > 0);
    assert!(compare(e, &base, negative(e, q), "prime31_independent_negated") > 0);
}

#[test]
fn independent_invalid_curve_and_coordinate_boundaries() {
    let e = Curve {
        p: 11,
        a: 1,
        b: 6,
        n: 13,
        g: (2, 7),
    };
    let base = [(2, 4), (3, 5), (5, 2), (7, 2)];
    assert_eq!(
        extract(
            Curve { a: 0, b: 0, ..e },
            &base,
            Some((8, 3)),
            Limits::default()
        )
        .status,
        Status::InvalidInput
    );
    assert_eq!(
        extract(Curve { a: 12, ..e }, &base, Some((8, 3)), Limits::default()).status,
        Status::InvalidInput
    );
    assert_eq!(
        extract(e, &base, Some((19, 3)), Limits::default()).status,
        Status::InvalidInput
    );
    let mut aliased = base;
    aliased[0].0 += e.p;
    assert_eq!(
        extract(e, &aliased, Some((8, 3)), Limits::default()).status,
        Status::InvalidInput
    );
}
