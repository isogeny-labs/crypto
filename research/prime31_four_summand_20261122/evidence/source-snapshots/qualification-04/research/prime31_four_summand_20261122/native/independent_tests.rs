//! Independent arithmetic controls for the public RR5 extraction boundary.
//!
//! Tuple enumeration below is confined to this test oracle. The candidate
//! receives only the curve, public base, public target, and frozen limits.
//! This file does not call the candidate's curve arithmetic or polynomial
//! builder to compute its expected signed relations.

use crate::extractor::{extract, Limits, ResultRow, Status};
use crate::relation_adapter::{AdapterError, Disposition, RelationAdapter, SignedRelation};
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

fn checked_b4_extraction(e: Curve, base: &[(u64, u64)], q: Affine, label: &str) -> ResultRow {
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
    actual
}

fn compare(e: Curve, base: &[(u64, u64)], q: Affine, label: &str) -> usize {
    checked_b4_extraction(e, base, q, label).relations.len()
}

/// Mutations use only an already extracted signed row and public query labels.
/// Both fixed test curves have odd prime order, so changing one sign changes
/// the sum by a nonzero twice-base-point, and adding one to a changes the RHS
/// by the nonzero generator. Neither rejected input may alter the row space.
fn reject_extracted_row_mutations(
    adapter: &mut RelationAdapter,
    e: Curve,
    row: &SignedRelation,
    a: u64,
    b: u64,
) {
    let rank_before = adapter.rank();
    let scalar_before = adapter.scalar();
    let rejected_before = adapter.work().rejected;
    let wrong_rhs = adapter.accept(row, (a + 1) % e.n, b).unwrap_err();
    assert_eq!(wrong_rhs, AdapterError::GroupEqualityFailed);
    let mut wrong_sign = row.clone();
    assert_eq!(wrong_sign.terms[0].1.unsigned_abs(), 1);
    wrong_sign.terms[0].1 = -wrong_sign.terms[0].1;
    let sign_error = adapter.accept(&wrong_sign, a, b).unwrap_err();
    assert_eq!(sign_error, AdapterError::GroupEqualityFailed);
    assert_eq!(adapter.rank(), rank_before);
    assert_eq!(adapter.scalar(), scalar_before);
    assert_eq!(adapter.work().rejected, rejected_before + 2);
    println!(
        "EXTRACTOR_ADAPTER_MUTATIONS {}",
        serde_json::json!({
            "curve": e,
            "public_a": a,
            "public_b": b,
            "original_row": row,
            "wrong_rhs_error": wrong_rhs,
            "mutated_sign_row": wrong_sign,
            "wrong_sign_error": sign_error,
            "rank_before": rank_before,
            "rank_after": adapter.rank(),
        })
    );
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

#[test]
fn extracted_f11_rows_recover_target_through_public_labelled_queries() {
    let e = Curve {
        p: 11,
        a: 1,
        b: 6,
        n: 13,
        g: (2, 7),
    };
    let base = [(2, 4), (3, 5), (5, 2), (7, 2)];
    // Known-answer construction is test-only. The query schedule below is the
    // fixed public list 1..n, independent of this expected value. The adapter
    // receives only public points and query labels; it receives no base logs.
    // This finite correctness control is not an efficient relation collector
    // or a complete-cost ECDLP comparison.
    let expected_scalar = 3;
    let target = scalar(e, expected_scalar, Some(e.g));
    assert_eq!(target, Some((8, 3)));
    let mut adapter = RelationAdapter::new(e, base.to_vec(), target).unwrap();
    let target_extraction = checked_b4_extraction(e, &base, target, "f11_boundary_target");
    assert_eq!(target_extraction.status, Status::Complete);
    let first = target_extraction
        .relations
        .first()
        .expect("the fixed target has a four-distinct-x witness");
    reject_extracted_row_mutations(&mut adapter, e, first, 0, 1);
    let mut receipts = Vec::new();
    let first_receipt = adapter.accept(first, 0, 1).unwrap();
    assert_eq!(first_receipt.disposition, Disposition::Independent);
    assert_eq!(
        (first_receipt.rank_before, first_receipt.rank_after),
        (0, 1)
    );
    assert_eq!(first_receipt.recovered_scalar, None);
    receipts.push(first_receipt);
    let duplicate = adapter.accept(first, 0, 1).unwrap();
    assert_eq!(duplicate.disposition, Disposition::Duplicate);
    assert_eq!((duplicate.rank_before, duplicate.rank_after), (1, 1));
    receipts.push(duplicate);
    for row in target_extraction.relations.iter().skip(1) {
        let receipt = adapter.accept(row, 0, 1).unwrap();
        assert_eq!(
            receipt.rank_after - receipt.rank_before,
            usize::from(receipt.disposition == Disposition::Independent)
        );
        receipts.push(receipt);
    }

    let mut query_records = Vec::new();
    let mut excluded = 0;
    let mut eligible = 0;
    for label in 1..e.n {
        let public_query = scalar(e, label, Some(e.g));
        let point = public_query.expect("nonzero label in the prime-order test group");
        if base.iter().any(|&(x, _)| x == point.0) {
            // Preserve the extractor's actual exceptional disposition instead
            // of fabricating an empty relation query or using a direct-log path.
            let result = extract(e, &base, public_query, Limits::default());
            assert_eq!(result.status, Status::UnsupportedChart);
            query_records.push(serde_json::json!({
                "public_label": label,
                "public_query": public_query,
                "extraction": result,
                "receipts": [],
            }));
            excluded += 1;
            continue;
        }
        let result = checked_b4_extraction(e, &base, public_query, "f11_boundary_labelled");
        let mut labelled_receipts = Vec::new();
        for row in &result.relations {
            let receipt = adapter.accept(row, label, 0).unwrap();
            assert_eq!(
                receipt.rank_after - receipt.rank_before,
                usize::from(receipt.disposition == Disposition::Independent)
            );
            labelled_receipts.push(receipt);
        }
        query_records.push(serde_json::json!({
            "public_label": label,
            "public_query": public_query,
            "extraction": result,
            "receipts": labelled_receipts,
        }));
        eligible += 1;
    }
    assert_eq!((eligible, excluded), (4, 8));
    let recovered = adapter
        .scalar()
        .expect("actual extracted rows determine the target");
    assert_eq!(recovered, expected_scalar);
    assert_eq!(scalar(e, recovered, Some(e.g)), target);
    assert!(adapter.rank() >= 2);
    assert_eq!(adapter.work().independent as usize, adapter.rank());
    assert_eq!(adapter.work().rejected, 2);
    assert_eq!(adapter.work().scalar_replays, 1);
    println!(
        "EXTRACTOR_ADAPTER_F11 {}",
        serde_json::json!({
            "scope": "known-answer boundary correctness; public labels enumerate the small control group; no solving-cost claim",
            "public_target": target,
            "public_base": base,
            "target_extraction": target_extraction,
            "target_receipts": receipts,
            "public_labelled_queries": query_records,
            "candidate_received_expected_scalar": false,
            "candidate_received_base_logs": false,
            "recovered_scalar": recovered,
            "independent_scalar_replay": scalar(e, recovered, Some(e.g)),
            "adapter_work": adapter.work(),
            "arithmetic_counts": adapter.arithmetic_counts(),
        })
    );
}

#[test]
fn extracted_prime31_rows_enter_adapter_with_true_signs_and_partial_rank() {
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
    // Only the public target reaches extraction. Its construction witness is
    // confined to this test, and no logarithm of the target or base is known
    // or supplied to the adapter. Hence this is a partial-rank boundary test.
    let target = sum_row(e, &base, &[(0, 1), (1, -1), (2, 1), (3, 1)]);
    assert!(target.is_some());
    let positive = checked_b4_extraction(e, &base, target, "prime31_boundary_positive");
    assert_eq!(positive.status, Status::Complete);
    let mut adapter = RelationAdapter::new(e, base.clone(), target).unwrap();
    let first = positive
        .relations
        .first()
        .expect("fixed public target is positive");
    reject_extracted_row_mutations(&mut adapter, e, first, 0, 1);
    let mut positive_receipts = Vec::new();
    for row in &positive.relations {
        let receipt = adapter.accept(row, 0, 1).unwrap();
        assert_eq!(
            receipt.rank_after - receipt.rank_before,
            usize::from(receipt.disposition == Disposition::Independent)
        );
        positive_receipts.push(receipt);
    }
    assert_eq!(positive_receipts[0].disposition, Disposition::Independent);
    assert!(adapter.rank() >= 1);
    assert_eq!(adapter.scalar(), None);
    let rank_before_duplicates = adapter.rank();
    let duplicate = adapter.accept(first, 0, 1).unwrap();
    assert_eq!(duplicate.disposition, Disposition::Duplicate);
    assert_eq!(duplicate.rank_before, duplicate.rank_after);

    let negated_target = negative(e, target);
    let negative_result =
        checked_b4_extraction(e, &base, negated_target, "prime31_boundary_negated");
    assert_eq!(negative_result.status, Status::Complete);
    let mut negative_receipts = Vec::new();
    for row in &negative_result.relations {
        assert_eq!(sum_row(e, &base, &row.terms), negated_target);
        let receipt = adapter.accept(row, 0, e.n - 1).unwrap();
        assert_eq!(receipt.disposition, Disposition::Duplicate);
        assert_eq!(receipt.rank_before, receipt.rank_after);
        negative_receipts.push(receipt);
    }
    assert_eq!(negative_receipts.len(), positive_receipts.len());
    assert_eq!(adapter.rank(), rank_before_duplicates);
    assert_eq!(adapter.scalar(), None);
    assert_eq!(adapter.work().independent as usize, adapter.rank());
    assert_eq!(adapter.work().rejected, 2);
    assert_eq!(adapter.work().scalar_replays, 0);
    println!(
        "EXTRACTOR_ADAPTER_PRIME31 {}",
        serde_json::json!({
            "scope": "natural-x-base signed-row and partial-rank boundary correctness; no scalar recovery or cost claim",
            "public_target": target,
            "public_base": base,
            "positive_extraction": positive,
            "positive_receipts": positive_receipts,
            "duplicate_receipt": duplicate,
            "negative_extraction": negative_result,
            "negative_receipts": negative_receipts,
            "candidate_received_known_logs": false,
            "rank": adapter.rank(),
            "recovered_scalar": adapter.scalar(),
            "adapter_work": adapter.work(),
            "arithmetic_counts": adapter.arithmetic_counts(),
        })
    );
}
