//! Exact-module development qualifier, not a performance implementation.
//! All field arithmetic and scalar S3/resultant checks use the unchanged
//! published repository module. No claim is made about ECDLP solving cost.

#[allow(dead_code)]
#[path = "../../../../src/cryptanalysis/semaev_prime_poly.rs"]
mod semaev_prime_poly;

use semaev_prime_poly::{resultant_formal, Curve, Fp};

type Matrix = Vec<Vec<u64>>;

fn prime_divisors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut q = 2;
    while q * q <= n {
        if n % q == 0 {
            factors.push(q);
            while n % q == 0 {
                n /= q;
            }
        }
        q += 1;
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

fn selector_roots(f: Fp, d: usize) -> Vec<u64> {
    assert!(d > 0 && (f.p - 1) % d as u64 == 0);
    if d == 1 {
        return vec![1];
    }
    let factors = prime_divisors(d as u64);
    let omega = (2..1024)
        .map(|a| f.pow(a, (f.p - 1) / d as u64))
        .find(|&w| factors.iter().all(|&q| f.pow(w, d as u64 / q) != 1))
        .expect("fixed bound did not find a primitive selector root");
    let mut roots = Vec::with_capacity(d);
    let mut x = 1;
    for _ in 0..d {
        roots.push(x);
        x = f.mul(x, omega);
    }
    assert_eq!(x, 1);
    let mut distinct = roots.clone();
    distinct.sort_unstable();
    distinct.dedup();
    assert_eq!(distinct.len(), d);
    roots
}

/// Coefficients [x exponent][t exponent] of S3(x,z,t).
fn factor(curve: Curve, z: u64) -> [[u64; 3]; 3] {
    let f = curve.f;
    let z2 = f.mul(z, z);
    let az2_b4 = f.neg(f.add(f.mul(2, f.mul(curve.a, z)), f.mul(4, curve.b)));
    [
        [
            f.sub(f.mul(curve.a, curve.a), f.mul(4, f.mul(curve.b, z))),
            az2_b4,
            z2,
        ],
        [
            az2_b4,
            f.neg(f.mul(2, f.add(z2, curve.a))),
            f.neg(f.mul(2, z)),
        ],
        [z2, f.neg(f.mul(2, z)), 1],
    ]
}

/// Direct product oracle in Fp[x,t]/(x^d-1); x only is reduced.
fn partial_norm(curve: Curve, d: usize, roots: &[u64]) -> Matrix {
    let f = curve.f;
    let mut out = vec![vec![0]; d];
    out[0][0] = 1;
    for &z in roots {
        let terms = factor(curve, z);
        let width = out[0].len();
        let mut next = vec![vec![0; width + 2]; d];
        for (i, row) in out.iter().enumerate() {
            for (j, &value) in row.iter().enumerate() {
                if value == 0 {
                    continue;
                }
                for (k, terms_row) in terms.iter().enumerate() {
                    for (ell, &coefficient) in terms_row.iter().enumerate() {
                        if coefficient == 0 {
                            continue;
                        }
                        let slot = &mut next[(i + k) % d][j + ell];
                        *slot = f.add(*slot, f.mul(value, coefficient));
                    }
                }
            }
        }
        out = next;
    }
    out
}

fn evaluate(f: Fp, matrix: &Matrix, x: u64, t: u64) -> u64 {
    matrix.iter().rev().fold(0, |acc_x, row| {
        let coefficient = row
            .iter()
            .rev()
            .fold(0, |acc_t, &c| f.add(f.mul(acc_t, t), c));
        f.add(f.mul(acc_x, x), coefficient)
    })
}

fn eliminate(f: Fp, mut matrix: Matrix) -> (usize, Vec<usize>, u64) {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut rank = 0;
    let mut determinant = 1;
    let mut pivot_columns = Vec::new();
    for col in 0..cols {
        let Some(pivot) = (rank..rows).find(|&r| matrix[r][col] != 0) else {
            continue;
        };
        if pivot != rank {
            matrix.swap(rank, pivot);
            determinant = f.neg(determinant);
        }
        let leading = matrix[rank][col];
        determinant = f.mul(determinant, leading);
        let inverse = f.inv(leading);
        for r in rank + 1..rows {
            let scale = f.mul(matrix[r][col], inverse);
            if scale == 0 {
                continue;
            }
            for c in col..cols {
                matrix[r][c] = f.sub(matrix[r][c], f.mul(scale, matrix[rank][c]));
            }
        }
        pivot_columns.push(col);
        rank += 1;
        if rank == rows {
            break;
        }
    }
    if rank < rows {
        determinant = 0;
    }
    (rank, pivot_columns, determinant)
}

fn binomial(f: Fp, n: usize, k: usize) -> u64 {
    assert!(n < f.p as usize);
    (0..k).fold(1, |acc, j| {
        f.mul(f.mul(acc, (n - j) as u64), f.inv((j + 1) as u64))
    })
}

fn singular_columns_match(f: Fp, d: usize, matrix: &Matrix) -> bool {
    (0..d).all(|j| {
        let c = binomial(f, 2 * d, j);
        let c = if j % 2 == 1 { f.neg(c) } else { c };
        (0..d).all(|i| matrix[i][j] == if i == (d - j) % d { c } else { 0 })
    })
}

fn discriminant_nonzero(curve: Curve) -> bool {
    let f = curve.f;
    f.add(
        f.mul(4, f.mul(f.mul(curve.a, curve.a), curve.a)),
        f.mul(27, f.mul(curve.b, curve.b)),
    ) != 0
}

fn leading_coefficient_check(curve: Curve, d: usize, matrix: &Matrix) {
    let f = curve.f;
    assert!(matrix.iter().all(|row| row[2 * d] == 0));
    if d >= 2 {
        let mut expected = vec![0; d];
        let scale = f.neg(f.mul(4, f.mul(d as u64, d as u64)));
        expected[1 % d] = f.add(expected[1 % d], scale);
        expected[(d - 1) % d] = f.add(expected[(d - 1) % d], f.mul(scale, curve.a));
        expected[(d - 2) % d] = f.add(expected[(d - 2) % d], f.mul(scale, curve.b));
        for i in 0..d {
            assert_eq!(
                matrix[i][2 * d - 1],
                expected[i],
                "leading coefficient d={d}, i={i}"
            );
        }
    }
}

#[derive(Default)]
struct Counts {
    cells: usize,
    scalar_products: usize,
    sylvester: usize,
    cusp_columns: usize,
    cusp_determinants: usize,
    leading_checks: usize,
}

fn check_case(label: &str, curve: Curve, d: usize, singular: bool, counts: &mut Counts) {
    let f = curve.f;
    assert!(f.p > 2 * d as u64);
    if !singular {
        assert!(discriminant_nonzero(curve));
    }
    let roots = selector_roots(f, d);
    let matrix = partial_norm(curve, d, &roots);
    let (rank, pivots, _) = eliminate(f, matrix.clone());
    let minor: Matrix = matrix.iter().map(|row| row[..d].to_vec()).collect();
    let (_, _, determinant) = eliminate(f, minor);
    leading_coefficient_check(curve, d, &matrix);
    counts.leading_checks += 1;
    let test_t = [0, 1, 2, 17, f.p - 1];
    for &x in &roots {
        for &t in &test_t {
            let expected = roots
                .iter()
                .fold(1, |acc, &z| f.mul(acc, curve.s3(x, z, t)));
            assert_eq!(evaluate(f, &matrix, x, t), expected);
            counts.scalar_products += 1;
        }
    }
    if d <= 8 {
        let mut selector = vec![0; d + 1];
        selector[0] = f.p - 1;
        selector[d] = 1;
        let inv2 = f.inv(2);
        for &x in roots.iter().take(3) {
            for &t in &test_t {
                let q0 = curve.s3(x, 0, t);
                let q1 = curve.s3(x, 1, t);
                let qm1 = curve.s3(x, f.p - 1, t);
                let q = [
                    q0,
                    f.mul(f.sub(q1, qm1), inv2),
                    f.mul(f.sub(f.add(q1, qm1), f.mul(2, q0)), inv2),
                ];
                let expected = resultant_formal(f, &selector, d, &q, 2);
                assert_eq!(evaluate(f, &matrix, x, t), expected);
                counts.sylvester += 1;
            }
        }
    }
    if singular {
        assert!(singular_columns_match(f, d, &matrix));
        counts.cusp_columns += d;
        let mut expected = (0..d).fold(1, |acc, j| f.mul(acc, binomial(f, 2 * d, j)));
        if (d - 1) % 2 == 1 {
            expected = f.neg(expected);
        }
        assert_eq!(determinant, expected);
        assert_eq!(rank, d);
        counts.cusp_determinants += 1;
    }
    let degree_t = (0..matrix[0].len())
        .rev()
        .find(|&j| matrix.iter().any(|row| row[j] != 0));
    let nonzeros = matrix
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&c| c != 0)
        .count();
    println!("{{\"kind\":\"coefficient_rank\",\"fixture\":\"{label}\",\"p\":{},\"a\":{},\"b\":{},\"d\":{d},\"singular_algebraic_control\":{singular},\"rank\":{rank},\"first_minor\":{determinant},\"degree_t\":{},\"nonzero_coefficients\":{nonzeros},\"pivot_columns\":{:?}}}", f.p, curve.a, curve.b, degree_t.map(|v| v.to_string()).unwrap_or_else(|| "null".to_owned()), pivots);
    counts.cells += 1;
}

fn exception_check() {
    let curve = Curve::new(97, 96, 0);
    assert!(discriminant_nonzero(curve));
    let matrix = partial_norm(curve, 2, &selector_roots(curve.f, 2));
    for (i, row) in matrix.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            assert_eq!(c, if i == 0 && j == 2 { 16 } else { 0 });
        }
    }
    assert_eq!(eliminate(curve.f, matrix).0, 1);
}

fn main() {
    let mut counts = Counts::default();
    for d in [1, 2, 3, 4, 6, 8, 12, 16] {
        check_case(
            "f97_cusp_control",
            Curve::new(97, 0, 0),
            d,
            true,
            &mut counts,
        );
        check_case(
            "f97_nonsingular",
            Curve::new(97, 2, 3),
            d,
            false,
            &mut counts,
        );
    }
    for d in [1, 2, 3, 6, 7, 9, 14, 18, 21, 31, 42, 63] {
        check_case(
            "prime31_cusp_control",
            Curve::new(2147483647, 0, 0),
            d,
            true,
            &mut counts,
        );
        check_case(
            "prime31_project",
            Curve::new(2147483647, 847514846, 2087444724),
            d,
            false,
            &mut counts,
        );
    }
    exception_check();
    println!("{{\"kind\":\"nonsingular_rank_exception\",\"p\":97,\"a\":96,\"b\":0,\"d\":2,\"rank\":1,\"H\":\"16*t^2\",\"passed\":true}}");
    println!("{{\"kind\":\"summary\",\"cells\":{},\"scalar_product_checks\":{},\"sylvester_checks\":{},\"cusp_coefficient_columns\":{},\"cusp_determinant_checks\":{},\"leading_coefficient_checks\":{},\"exception_checks\":1,\"passed\":true,\"scope\":\"native_exact_module_development_correctness_only\"}}", counts.cells, counts.scalar_products, counts.sylvester, counts.cusp_columns, counts.cusp_determinants, counts.leading_checks);
}

#[cfg(test)]
mod qualifier_tests {
    use super::*;

    #[test]
    fn detects_altered_cusp_coefficient() {
        let curve = Curve::new(97, 0, 0);
        let mut matrix = partial_norm(curve, 8, &selector_roots(curve.f, 8));
        assert!(singular_columns_match(curve.f, 8, &matrix));
        matrix[0][0] = curve.f.add(matrix[0][0], 1);
        assert!(!singular_columns_match(curve.f, 8, &matrix));
    }

    #[test]
    fn nonsingular_rank_is_not_universal() {
        exception_check();
    }

    #[test]
    fn specializations_and_degree_drops_match_sylvester() {
        let mut counts = Counts::default();
        for curve in [
            Curve::new(97, 0, 0),
            Curve::new(97, 2, 3),
            Curve::new(97, 96, 0),
        ] {
            for d in [1, 2, 3, 4, 6, 8] {
                check_case("unit", curve, d, curve.a == 0 && curve.b == 0, &mut counts);
            }
        }
        assert_eq!(counts.cells, 18);
    }
}
