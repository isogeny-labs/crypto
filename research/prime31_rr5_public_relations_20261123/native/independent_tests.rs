//! Independent arithmetic controls for the public RR5 extraction boundary.
//!
//! Tuple enumeration below is confined to this test oracle. The candidate
//! receives only the curve, public base, public target, and frozen limits.
//! This file does not call the candidate's curve arithmetic or polynomial
//! builder to compute its expected signed relations.

use crate::extractor::{extract, extract_with_form, ExtractionForm, Limits, Status};
use prime31_algebraic_ic::{Curve, PRIME31};
use std::collections::BTreeSet;
use std::io::Write;

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

// The exact natural-x B8 fixture from main e5106005's separately retained
// b8-positive-02 and b8-empty-02 controls. Construction is test-only and uses
// the existing independent arithmetic above, not the candidate's backend.
fn independent_prime31_natural_b8() -> Vec<(u64, u64)> {
    let e = PRIME31;
    let mut base = Vec::new();
    for x in 0..4096 {
        let r = rhs(e, x);
        let y = power(r, (e.p + 1) / 4, e.p);
        if y != 0 && product(y, y, e.p) == r {
            base.push((x, y.min(e.p - y)));
        }
        if base.len() == 8 {
            break;
        }
    }
    assert_eq!(
        base,
        vec![
            (0, 309_380_359),
            (1, 749_117_756),
            (2, 51_658_786),
            (3, 166_645_833),
            (5, 457_995_111),
            (6, 865_603_774),
            (8, 849_737_830),
            (9, 548_858_265),
        ],
        "the declared main-branch B8 fixture must remain exact"
    );
    base
}

fn oracle_b8_distinct_four(e: Curve, base: &[(u64, u64)], q: Affine) -> (SignedRows, usize) {
    assert_eq!(base.len(), 8);
    let mut rows = BTreeSet::new();
    let mut tuple_evaluations = 0;
    for a in 0..base.len() {
        for b in a + 1..base.len() {
            for c in b + 1..base.len() {
                for d in c + 1..base.len() {
                    let indices = [a, b, c, d];
                    let subset = indices.map(|i| base[i]);
                    // Reuse the independent 16-sign B4 oracle on each of
                    // the 70 subsets; remap only its public base indices.
                    for row in oracle_b4(e, &subset, q) {
                        rows.insert(
                            row.into_iter()
                                .map(|(i, sign)| (indices[i], sign))
                                .collect(),
                        );
                    }
                    tuple_evaluations += 16;
                }
            }
        }
    }
    (rows, tuple_evaluations)
}

fn qualify_prime31_b8_grevlex_cofactor(
    base: &[(u64, u64)],
    q: Affine,
    label: &str,
    expect_empty: bool,
) {
    let e = PRIME31;
    let (expected, tuple_evaluations) = oracle_b8_distinct_four(e, base, q);
    let limits = Limits {
        max_degree: 10,
        seconds: 30,
        max_polynomial_terms: 20_000,
        max_assignments: 4096,
    };
    let form = ExtractionForm::CofactorGrevlex;
    // The parent protocol supplies separate processes, one Rayon/test worker,
    // a 180-second outer timeout and 2,097,152 KiB RLIMIT_AS. The candidate's
    // stricter shared deadline includes validation as well as construction.
    println!(
        "RR5_GREVLEX_B8_INPUT {}",
        serde_json::json!({
            "label": label,
            "scope": "bounded public-fixture correctness; no cost comparison",
            "curve": e,
            "public_base": base,
            "public_target": q,
            "form": form,
            "expected_empty": expect_empty,
            "expected_rows": &expected,
            "oracle_tuple_evaluations": tuple_evaluations,
            "limits": {
                "max_degree": limits.max_degree,
                "seconds": limits.seconds,
                "max_polynomial_terms": limits.max_polynomial_terms,
                "max_assignments": limits.max_assignments,
            },
            "candidate_received_oracle_rows": false,
        })
    );
    std::io::stdout()
        .flush()
        .expect("persist the B8 input record");
    let actual = extract_with_form(e, base, q, limits, form);
    println!(
        "RR5_GREVLEX_B8_RESULT {}",
        serde_json::json!({
            "label": label,
            "curve": e,
            "public_base": base,
            "public_target": q,
            "form": form,
            "expected_empty": expect_empty,
            "expected_rows": &expected,
            "oracle_tuple_evaluations": tuple_evaluations,
            "candidate": &actual,
            "candidate_received_oracle_rows": false,
        })
    );
    std::io::stdout()
        .flush()
        .expect("persist the B8 outcome before qualification assertions");
    assert_eq!(tuple_evaluations, 70 * 16);
    assert_eq!(expected.is_empty(), expect_empty, "fixed oracle control");
    if !expect_empty {
        assert_eq!(
            expected,
            BTreeSet::from([vec![(0, 1), (1, -1), (2, 1), (3, 1)]]),
            "the exact main positive has one independently established row"
        );
    }
    assert_eq!(
        actual.status,
        if expect_empty {
            Status::Empty
        } else {
            Status::Complete
        },
        "{label}: incomplete extraction fails this completion gate"
    );
    let got: SignedRows = actual
        .relations
        .iter()
        .map(|row| row.terms.clone())
        .collect();
    assert_eq!(got, expected, "{label}: all signed distinct-four rows");
    assert_eq!(actual.relations.is_empty(), expect_empty);
    assert_eq!(actual.diagnostics.presentation, "monic_cofactor");
    assert_eq!(actual.diagnostics.monomial_order, "grevlex");
    assert_eq!(actual.diagnostics.input_variables, 7);
    assert_eq!(actual.diagnostics.input_polynomials, 8);
    assert_eq!(actual.diagnostics.input_terms, 99);
    assert_eq!(actual.diagnostics.input_max_degree, 3);
    assert_eq!(actual.diagnostics.forbidden_branches_executed, 0);
    assert_eq!(actual.diagnostics.forbidden_fallbacks_refused, 0);
    assert_eq!(actual.diagnostics.rejected_assignments, 0);
    let trace = actual
        .traversal
        .as_ref()
        .expect("completed B8 extraction retains native traversal evidence");
    assert!(trace.complete);
    assert!(!trace.timed_out && !trace.trace_stopped && !trace.stopped_early);
    assert_eq!(trace.full_field_branches, 0);
    assert_eq!(trace.domain_evaluations, 0);
    assert_eq!(trace.candidates_rejected, 0);
    assert!(trace.timings.is_none(), "visitor timing remains disabled");
    for row in &got {
        assert_eq!(sum_row(e, base, row), q);
    }
}

#[test]
fn independent_prime31_b8_grevlex_cofactor_positive_complete_gate() {
    let base = independent_prime31_natural_b8();
    let q = sum_row(PRIME31, &base, &[(0, 1), (1, -1), (2, 1), (3, 1)]);
    assert_eq!(q, Some((92_816_593, 1_965_281_801)));
    qualify_prime31_b8_grevlex_cofactor(&base, q, "prime31_b8_grevlex_positive", false);
}

#[test]
fn independent_prime31_b8_grevlex_cofactor_generator_empty_gate() {
    let base = independent_prime31_natural_b8();
    qualify_prime31_b8_grevlex_cofactor(
        &base,
        Some(PRIME31.g),
        "prime31_b8_grevlex_generator_empty",
        true,
    );
}
