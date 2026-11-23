//! Bounded exact-module correctness qualifier for the frozen target quotient protocol.
//! Direct products, root enumeration and dense matrices are correctness oracles.
//! This is not the concurrent signed extractor and does not measure performance.

#![recursion_limit = "512"]

mod oracle;
#[allow(dead_code)]
#[path = "../../../../src/cryptanalysis/semaev_prime_poly.rs"]
mod semaev_prime_poly;

use oracle::{binomial, discriminant_nonzero, eliminate, evaluate, partial_norm, Matrix};
use semaev_prime_poly::{
    derivative, interpolate, poly_gcd, poly_mulmod, poly_powmod, poly_rem, trim, Curve, Fp,
};
use serde::Serialize;
use serde_json::json;
use std::collections::BTreeSet;

const PROTOCOL_SHA256: &str = "3eafeab5c2dc73c0add918bb23c9e8cdac1cf82d567d18a49792fd84aee0b20f";

#[derive(Default, Serialize)]
struct Counts {
    full_norms: usize,
    finite_targets: usize,
    rank_one_control_targets: usize,
    cusp_nonreduced_cases: usize,
    empty_subset_cases: usize,
    identity_chart_flags: usize,
    norm_scalar_checks: usize,
    right_scalar_checks: usize,
    rational_root_set_checks: usize,
    f97_exhaustive_root_checks: usize,
    independent_remainder_coefficients: usize,
    quotient_grid_checks: usize,
    coefficient_evaluation_rank_checks: usize,
    full_right_unchanged_checks: usize,
    cusp_minor_checks: usize,
}

#[derive(Clone)]
struct Target {
    category: &'static str,
    x: u64,
    y: u64,
}

fn polynomial_value(f: Fp, polynomial: &[u64], x: u64) -> u64 {
    polynomial
        .iter()
        .rev()
        .fold(0, |a, &b| f.add(f.mul(a, x), b))
}

fn monic(f: Fp, polynomial: &[u64]) -> Vec<u64> {
    let mut out = trim(polynomial.to_vec());
    assert!(out != [0], "zero polynomial has no monic normalization");
    let scale = f.inv(*out.last().unwrap());
    for c in &mut out {
        *c = f.mul(*c, scale);
    }
    out
}

/// Use the published multiplication routine modulo a power above the product degree.
fn multiply_exact(f: Fp, a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut modulus = vec![0; a.len() + b.len()];
    *modulus.last_mut().unwrap() = 1;
    poly_mulmod(f, a, b, &modulus)
}

fn difference(f: Fp, a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut out = vec![0; a.len().max(b.len())];
    for (i, c) in out.iter_mut().enumerate() {
        *c = f.sub(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0));
    }
    trim(out)
}

fn cubic_value(curve: Curve, x: u64) -> u64 {
    let f = curve.f;
    f.add(f.add(f.mul(f.mul(x, x), x), f.mul(curve.a, x)), curve.b)
}

/// Exact fixed-field square-root oracle, using the published field arithmetic.
fn square_root(f: Fp, a: u64) -> Option<u64> {
    if f.p == 97 {
        return (0..f.p).find(|&y| f.mul(y, y) == a);
    }
    assert_eq!(f.p, 2147483647);
    let y = f.pow(a, (f.p + 1) / 4);
    (f.mul(y, y) == a).then_some(y.min(f.neg(y)))
}

fn point_at(curve: Curve, x: u64) -> Option<(u64, u64)> {
    square_root(curve.f, cubic_value(curve, x)).map(|y| (x, y))
}

fn ordinary_scan_points(curve: Curve, count: usize) -> Vec<(u64, u64)> {
    let points: Vec<_> = (0..1024u64.min(curve.f.p))
        .filter_map(|x| point_at(curve, x))
        .filter(|&(_, y)| y != 0)
        .take(count)
        .collect();
    assert_eq!(points.len(), count, "public scan bound exhausted");
    points
}

fn sorted_selector(f: Fp, d: usize) -> Vec<u64> {
    let mut out = oracle::selector_roots(f, d);
    out.sort_unstable();
    out
}

fn quadratic_from_values(f: Fp, q0: u64, q1: u64, qm1: u64) -> Vec<u64> {
    let inv2 = f.inv(2);
    trim(vec![
        q0,
        f.mul(f.sub(q1, qm1), inv2),
        f.mul(f.sub(f.add(q1, qm1), f.mul(2, q0)), inv2),
    ])
}

fn right_factor(curve: Curve, z: u64, x: u64) -> Vec<u64> {
    quadratic_from_values(
        curve.f,
        curve.s3(0, z, x),
        curve.s3(1, z, x),
        curve.s3(curve.f.p - 1, z, x),
    )
}

fn left_factor(curve: Curve, x: u64, z: u64) -> Vec<u64> {
    quadratic_from_values(
        curve.f,
        curve.s3(x, z, 0),
        curve.s3(x, z, 1),
        curve.s3(x, z, curve.f.p - 1),
    )
}

fn quadratic_roots(f: Fp, q: &[u64]) -> Vec<u64> {
    assert!(
        q != [0],
        "identically zero factor needs an all-field branch"
    );
    match q.len() {
        1 => vec![],
        2 => vec![f.mul(f.neg(q[0]), f.inv(q[1]))],
        3 => {
            let discriminant = f.sub(f.mul(q[1], q[1]), f.mul(4, f.mul(q[2], q[0])));
            match square_root(f, discriminant) {
                None => vec![],
                Some(s) => {
                    let scale = f.inv(f.mul(2, q[2]));
                    let mut roots = vec![
                        f.mul(f.add(f.neg(q[1]), s), scale),
                        f.mul(f.sub(f.neg(q[1]), s), scale),
                    ];
                    roots.sort_unstable();
                    roots.dedup();
                    roots
                }
            }
        }
        _ => panic!("factor degree exceeds two"),
    }
}

fn product_right(curve: Curve, roots: &[u64], x: u64) -> Vec<u64> {
    roots.iter().fold(vec![1], |acc, &z| {
        multiply_exact(curve.f, &acc, &right_factor(curve, z, x))
    })
}

fn rational_part(f: Fp, right: &[u64]) -> Vec<u64> {
    assert!(right != [0]);
    if right.len() == 1 {
        return vec![1];
    }
    let frobenius = poly_powmod(f, &[0, 1], f.p, right);
    let t = poly_rem(f, &[0, 1], right);
    monic(f, &poly_gcd(f, right, &difference(f, &frobenius, &t)))
}

fn product_from_roots(f: Fp, roots: &[u64]) -> Vec<u64> {
    roots.iter().fold(vec![1], |acc, &root| {
        multiply_exact(f, &acc, &[f.neg(root), 1])
    })
}

fn remainder_matrix(f: Fp, matrix: &Matrix, modulus: &[u64]) -> Matrix {
    let width = modulus.len() - 1;
    matrix
        .iter()
        .map(|row| {
            let mut r = poly_rem(f, row, modulus);
            r.resize(width, 0);
            r
        })
        .collect()
}

fn matrix_rank(f: Fp, matrix: &Matrix) -> usize {
    if matrix.is_empty() {
        0
    } else {
        eliminate(f, matrix.clone()).0
    }
}

fn nonzero_count(matrix: &Matrix) -> usize {
    matrix.iter().flatten().filter(|&&v| v != 0).count()
}

/// Basis vectors obtained from the published interpolation implementation.
fn interpolation_basis(f: Fp, roots: &[u64]) -> Matrix {
    (0..roots.len())
        .map(|i| {
            let mut values = vec![0; roots.len()];
            values[i] = 1;
            interpolate(f, roots, &values)
        })
        .collect()
}

fn independent_remainder(curve: Curve, roots: &[u64], g: &[u64], basis: &Matrix) -> Matrix {
    let f = curve.f;
    let e = g.len() - 1;
    let mut reconstructed = vec![vec![0; e]; roots.len()];
    for (point_index, &x) in roots.iter().enumerate() {
        let mut value = poly_rem(f, &[1], g);
        for &z in roots {
            value = poly_mulmod(f, &value, &left_factor(curve, x, z), g);
        }
        value.resize(e, 0);
        for (i, row) in reconstructed.iter_mut().enumerate() {
            for (j, entry) in row.iter_mut().enumerate() {
                *entry = f.add(*entry, f.mul(basis[point_index][i], value[j]));
            }
        }
    }
    reconstructed
}

fn check_full_norm(curve: Curve, d: usize, roots: &[u64], h: &Matrix, counts: &mut Counts) {
    oracle::leading_coefficient_check(curve, d, h);
    for &x in roots {
        for t in [0, 1, 2, 17, curve.f.p - 1] {
            let product = roots
                .iter()
                .fold(1, |acc, &z| curve.f.mul(acc, curve.s3(x, z, t)));
            assert_eq!(evaluate(curve.f, h, x, t), product);
            counts.norm_scalar_checks += 1;
        }
    }
}

fn check_target(
    fixture: &str,
    curve: Curve,
    d: usize,
    roots: &[u64],
    h: &Matrix,
    basis: &Matrix,
    target: &Target,
    duplicate_category_target: bool,
    counts: &mut Counts,
) {
    let f = curve.f;
    assert!(discriminant_nonzero(curve));
    assert_eq!(f.mul(target.y, target.y), cubic_value(curve, target.x));
    let r = product_right(curve, roots, target.x);
    assert!(r != [0]);
    let g = rational_part(f, &r);
    let e = g.len() - 1;
    let mut root_union = BTreeSet::new();
    let mut finite_fiber_roots = 0;
    let mut degree_drops = 0;
    let mut repeated_quadratics = 0;
    for &z in roots {
        let q = right_factor(curve, z, target.x);
        let padded: Vec<_> = (0..3).map(|i| *q.get(i).unwrap_or(&0)).collect();
        let delta = f.sub(
            f.mul(padded[1], padded[1]),
            f.mul(4, f.mul(padded[2], padded[0])),
        );
        assert_eq!(
            delta,
            f.mul(
                16,
                f.mul(cubic_value(curve, z), cubic_value(curve, target.x))
            )
        );
        degree_drops += usize::from(q.len() < 3);
        repeated_quadratics += usize::from(q.len() == 3 && delta == 0);
        let fiber = quadratic_roots(f, &q);
        finite_fiber_roots += fiber.len();
        for t in fiber {
            assert_eq!(curve.s3(t, z, target.x), 0);
            root_union.insert(t);
        }
    }
    let rational_roots: Vec<_> = root_union.into_iter().collect();
    assert_eq!(g, product_from_roots(f, &rational_roots));
    assert_eq!(rational_roots.len(), e);
    assert_eq!(poly_rem(f, &r, &g), vec![0]);
    assert_eq!(monic(f, &poly_gcd(f, &g, &derivative(f, &g))), vec![1]);
    counts.rational_root_set_checks += 1;
    if f.p == 97 {
        for t in 0..f.p {
            assert_eq!(
                polynomial_value(f, &r, t) == 0,
                rational_roots.binary_search(&t).is_ok()
            );
            counts.f97_exhaustive_root_checks += 1;
        }
    }
    for t in [0, 1, 2, 17, f.p - 1]
        .into_iter()
        .chain(rational_roots.iter().copied())
    {
        let scalar = roots
            .iter()
            .fold(1, |acc, &z| f.mul(acc, curve.s3(t, z, target.x)));
        assert_eq!(polynomial_value(f, &r, t), scalar);
        counts.right_scalar_checks += 1;
    }
    let full_right_remainder = remainder_matrix(f, h, &r);
    let full_right_unchanged = h.iter().zip(&full_right_remainder).all(|(left, right)| {
        trim(left.clone())
            == trim(if right.is_empty() {
                vec![0]
            } else {
                right.clone()
            })
    });
    if f.pow(target.x, d as u64) != 1 {
        assert_eq!(r.len() - 1, 2 * d);
        assert!(full_right_unchanged);
        counts.full_right_unchanged_checks += 1;
    }
    let reduced = remainder_matrix(f, h, &g);
    assert_eq!(reduced, independent_remainder(curve, roots, &g, basis));
    counts.independent_remainder_coefficients += d * e;
    let mut evaluation = vec![vec![0; e]; d];
    let mut boolean_support = vec![vec![0; e]; d];
    let liftable: Vec<_> = roots
        .iter()
        .map(|&x| point_at(curve, x).is_some())
        .collect();
    let left_two_torsion = roots
        .iter()
        .filter(|&&x| cubic_value(curve, x) == 0)
        .count();
    let mut zero_support = 0;
    let mut lifted_zero_support = 0;
    for (i, &x) in roots.iter().enumerate() {
        for (j, &t) in rational_roots.iter().enumerate() {
            let value = evaluate(f, h, x, t);
            let scalar = roots
                .iter()
                .fold(1, |acc, &z| f.mul(acc, curve.s3(x, z, t)));
            assert_eq!(value, scalar);
            assert_eq!(evaluate(f, &reduced, x, t), value);
            evaluation[i][j] = value;
            boolean_support[i][j] = u64::from(value == 0);
            zero_support += usize::from(value == 0);
            lifted_zero_support += usize::from(value == 0 && liftable[i]);
            counts.quotient_grid_checks += 1;
        }
    }
    let full_rank = matrix_rank(f, h);
    let reduced_rank = matrix_rank(f, &reduced);
    assert!(reduced_rank <= full_rank.min(e));
    assert_eq!(reduced_rank, matrix_rank(f, &evaluation));
    let selector_target_row = roots.binary_search(&target.x).ok();
    if let Some(row) = selector_target_row {
        // S3 symmetry gives H(X,t)=R_V(t), and G_X divides R_V.
        assert!(evaluation[row].iter().all(|&value| value == 0));
        assert!(reduced_rank <= (d - 1).min(e));
        assert!(liftable[row]);
    }
    counts.coefficient_evaluation_rank_checks += 1;
    let lifted_evaluation: Matrix = evaluation
        .iter()
        .enumerate()
        .filter(|&(i, _)| liftable[i])
        .map(|(_, row)| row.clone())
        .collect();
    let lifted_boolean: Matrix = boolean_support
        .iter()
        .enumerate()
        .filter(|&(i, _)| liftable[i])
        .map(|(_, row)| row.clone())
        .collect();
    if selector_target_row.is_some() {
        assert!(matrix_rank(f, &lifted_evaluation) <= (lifted_evaluation.len() - 1).min(e));
    }
    let multiple_factor = poly_gcd(f, &r, &derivative(f, &r));
    let multiplicity_loss = multiple_factor.len() - 1;
    let squarefree_degree = r.len() - 1 - multiplicity_loss;
    assert!(squarefree_degree >= e);
    println!(
        "{}",
        json!({
            "kind": "target_quotient", "fixture": fixture, "p": f.p, "a": curve.a, "b": curve.b,
            "d": d, "target_category": target.category, "target_x": target.x, "target_y": target.y,
            "duplicate_across_categories": duplicate_category_target, "deduplicated": false,
            "target_two_torsion": target.y == 0, "target_x_in_selector": roots.binary_search(&target.x).is_ok(),
            "affine_right_degree_drop_factors": degree_drops,
            "identity_branch_flag": degree_drops > 0, "identity_branch_solved": false,
            "right_repeated_quadratic_factors": repeated_quadratics,
            "right_distinct_fiber_root_occurrences": finite_fiber_roots,
            "right_fiber_collision_excess": finite_fiber_roots - e,
            "degree_full_right": r.len()-1, "degree_squarefree_full_right": squarefree_degree,
            "degree_rational_right": e, "multiplicity_loss": multiplicity_loss,
            "nonrational_squarefree_degree": squarefree_degree - e,
            "empty_rational_message": e == 0,
            "rank_full_norm": full_rank, "rank_full_right_remainder": matrix_rank(f, &full_right_remainder),
            "full_right_remainder_unchanged": full_right_unchanged,
            "rank_rational_remainder": reduced_rank, "rank_rational_grid": matrix_rank(f, &evaluation),
            "rank_dimension_bound": d.min(e),
        "selector_target_zero_row": selector_target_row,
        "selector_target_rank_bound": selector_target_row.map(|_| (d - 1).min(e)),
            "nonzero_full_norm_coefficients": nonzero_count(h),
            "nonzero_full_right_remainder_coefficients": nonzero_count(&full_right_remainder),
            "nonzero_rational_remainder_coefficients": nonzero_count(&reduced),
            "algebraic_zero_support_all_selector_rows": zero_support,
            "boolean_support_rank_over_fp": matrix_rank(f, &boolean_support),
            "liftable_left_rows": lifted_evaluation.len(), "left_selector_two_torsion": left_two_torsion,
            "liftable_left_value_rank": matrix_rank(f, &lifted_evaluation),
            "liftable_left_algebraic_zero_support": lifted_zero_support,
            "liftable_left_boolean_support_rank_over_fp": matrix_rank(f, &lifted_boolean),
            "rational_roots": rational_roots, "monic_rational_message": g,
            "signed_relation_claim": false, "passed": true
        })
    );
    counts.finite_targets += 1;
}

fn cusp_controls(p: u64, d: usize, roots: &[u64], counts: &mut Counts) {
    let curve = Curve::new(p, 0, 0);
    let f = curve.f;
    let h = partial_norm(curve, d, roots);
    check_full_norm(curve, d, roots, &h, counts);
    assert!(oracle::singular_columns_match(f, d, &h));
    let sizes: BTreeSet<_> = [1, (d / 2).max(1), d].into_iter().collect();
    for k in sizes {
        let right = monic(f, &product_right(curve, &roots[..k], 0));
        let mut expected_right = vec![0; 2 * k + 1];
        expected_right[2 * k] = 1;
        assert_eq!(right, expected_right);
        let remainder = remainder_matrix(f, &h, &right);
        let r = d.min(2 * k);
        for j in 0..r {
            let magnitude = binomial(f, 2 * d, j);
            let c = if j % 2 == 0 {
                magnitude
            } else {
                f.neg(magnitude)
            };
            for i in 0..d {
                assert_eq!(remainder[i][j], if i == (d - j) % d { c } else { 0 });
            }
        }
        let selected_rows: Vec<_> = std::iter::once(0).chain(d - r + 1..d).collect();
        let minor: Matrix = selected_rows
            .iter()
            .map(|&i| remainder[i][..r].to_vec())
            .collect();
        let determinant = eliminate(f, minor).2;
        let magnitude = (0..r).fold(1, |acc, j| f.mul(acc, binomial(f, 2 * d, j)));
        let expected_det = if r % 2 == 0 {
            f.neg(magnitude)
        } else {
            magnitude
        };
        assert_eq!(determinant, expected_det);
        assert_ne!(determinant, 0);
        assert_eq!(matrix_rank(f, &remainder), r);
        let radical = rational_part(f, &right);
        assert_eq!(radical, vec![0, 1]);
        let radical_remainder = remainder_matrix(f, &h, &radical);
        assert_eq!(matrix_rank(f, &radical_remainder), 1);
        println!(
            "{}",
            json!({
                "kind":"cusp_nonreduced_control", "p":p, "d":d, "k":k,
                "monic_right":"t^(2k)", "degree_right":2*k,
                "rank_nonreduced_remainder":r, "minor_rows":selected_rows,
                "minor_columns":(0..r).collect::<Vec<_>>(), "minor_determinant":determinant,
                "radical_right":"t", "rank_radical_remainder":1,
                "elliptic_target":false, "passed":true
            })
        );
        counts.cusp_nonreduced_cases += 1;
        counts.cusp_minor_checks += 1;
    }
    let empty = product_right(curve, &[], 0);
    assert_eq!(empty, vec![1]);
    let empty_remainder = remainder_matrix(f, &h, &empty);
    assert!(empty_remainder.iter().all(Vec::is_empty));
    assert_eq!(matrix_rank(f, &empty_remainder), 0);
    assert_eq!(rational_part(f, &empty), vec![1]);
    println!(
        "{}",
        json!({"kind":"empty_subset_control","p":p,"d":d,"right":[1],"rational_degree":0,"rank":0,"passed":true})
    );
    counts.empty_subset_cases += 1;
}

fn run_fixture(fixture: &str, curve: Curve, degrees: &[usize], counts: &mut Counts) {
    assert!(discriminant_nonzero(curve));
    let scanned = ordinary_scan_points(curve, 3);
    for &d in degrees {
        assert!(curve.f.p > 2 * d as u64);
        let roots = sorted_selector(curve.f, d);
        cusp_controls(curve.f.p, d, &roots, counts);
        let h = partial_norm(curve, d, &roots);
        check_full_norm(curve, d, &roots, &h, counts);
        let basis = interpolation_basis(curve.f, &roots);
        println!(
            "{}",
            json!({"kind":"full_norm","fixture":fixture,"p":curve.f.p,"a":curve.a,"b":curve.b,"d":d,"rank":matrix_rank(curve.f,&h),"nonzero_coefficients":nonzero_count(&h),"passed":true})
        );
        counts.full_norms += 1;
        let labels = ["public_scan_0", "public_scan_1", "public_scan_2"];
        let mut targets: Vec<_> = scanned
            .iter()
            .enumerate()
            .map(|(i, &(x, y))| Target {
                category: labels[i],
                x,
                y,
            })
            .collect();
        if let Some((x, y)) = roots.iter().find_map(|&x| point_at(curve, x)) {
            targets.push(Target {
                category: "selector_collision",
                x,
                y,
            });
        }
        if curve.f.p == 97 {
            targets.push(Target {
                category: "two_torsion_30",
                x: 30,
                y: 0,
            });
        }
        let mut seen = BTreeSet::new();
        for target in targets {
            let duplicate = !seen.insert((target.x, target.y));
            check_target(
                fixture, curve, d, &roots, &h, &basis, &target, duplicate, counts,
            );
        }
        println!(
            "{}",
            json!({"kind":"identity_chart_flag","fixture":fixture,"p":curve.f.p,"d":d,"target":"point_at_infinity","status":"outside_affine_qualification","message":null,"rank":null})
        );
        counts.identity_chart_flags += 1;
    }
}

fn main() {
    println!(
        "{}",
        json!({"kind":"scope","protocol_sha256":PROTOCOL_SHA256,"base":"f0ca664eca0081a30adfcc79dcf62a32de40d2af","mode":"bounded_native_correctness_only","rank_saturation_is_acceptance_assertion":false})
    );
    let mut counts = Counts::default();
    run_fixture(
        "f97",
        Curve::new(97, 2, 3),
        &[1, 2, 3, 4, 6, 8, 12, 16],
        &mut counts,
    );
    run_fixture(
        "prime31_project",
        Curve::new(2147483647, 847514846, 2087444724),
        &[1, 2, 3, 6, 7, 9, 14, 18, 21, 31, 42, 63],
        &mut counts,
    );
    let curve = Curve::new(97, 96, 0);
    let roots = sorted_selector(curve.f, 2);
    let h = partial_norm(curve, 2, &roots);
    check_full_norm(curve, 2, &roots, &h, &mut counts);
    for (i, row) in h.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            assert_eq!(c, if i == 0 && j == 2 { 16 } else { 0 });
        }
    }
    assert_eq!(matrix_rank(curve.f, &h), 1);
    let basis = interpolation_basis(curve.f, &roots);
    let ordinary = ordinary_scan_points(curve, 1)[0];
    for target in [
        Target {
            category: "rank_one_two_torsion_0",
            x: 0,
            y: 0,
        },
        Target {
            category: "rank_one_two_torsion_1",
            x: 1,
            y: 0,
        },
        Target {
            category: "rank_one_public_scan_0",
            x: ordinary.0,
            y: ordinary.1,
        },
    ] {
        check_target(
            "f97_rank_one_exception",
            curve,
            2,
            &roots,
            &h,
            &basis,
            &target,
            false,
            &mut counts,
        );
        counts.rank_one_control_targets += 1;
    }
    println!(
        "{}",
        json!({"kind":"summary","counts":counts,"passed":true,"scope":"native_exact_module_development_correctness_only"})
    );
}
