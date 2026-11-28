// Product and matrix helpers copied from the published coupled/native/main.rs.
// Source SHA256: 585172d4871b3b55591d1562eca95fb93ff44f9d08d0fb1bf92f2138f7e663ee.
// Only visibility/imports change; original field arithmetic is included separately.
#![allow(dead_code)]
use crate::semaev_prime_poly::{Curve, Fp};

pub(crate) type Matrix = Vec<Vec<u64>>;

pub(crate) fn prime_divisors(mut n: u64) -> Vec<u64> {
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

pub(crate) fn selector_roots(f: Fp, d: usize) -> Vec<u64> {
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
pub(crate) fn factor(curve: Curve, z: u64) -> [[u64; 3]; 3] {
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
pub(crate) fn partial_norm(curve: Curve, d: usize, roots: &[u64]) -> Matrix {
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

pub(crate) fn evaluate(f: Fp, matrix: &Matrix, x: u64, t: u64) -> u64 {
    matrix.iter().rev().fold(0, |acc_x, row| {
        let coefficient = row
            .iter()
            .rev()
            .fold(0, |acc_t, &c| f.add(f.mul(acc_t, t), c));
        f.add(f.mul(acc_x, x), coefficient)
    })
}

pub(crate) fn eliminate(f: Fp, mut matrix: Matrix) -> (usize, Vec<usize>, u64) {
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

pub(crate) fn binomial(f: Fp, n: usize, k: usize) -> u64 {
    assert!(n < f.p as usize);
    (0..k).fold(1, |acc, j| {
        f.mul(f.mul(acc, (n - j) as u64), f.inv((j + 1) as u64))
    })
}

pub(crate) fn singular_columns_match(f: Fp, d: usize, matrix: &Matrix) -> bool {
    (0..d).all(|j| {
        let c = binomial(f, 2 * d, j);
        let c = if j % 2 == 1 { f.neg(c) } else { c };
        (0..d).all(|i| matrix[i][j] == if i == (d - j) % d { c } else { 0 })
    })
}

pub(crate) fn discriminant_nonzero(curve: Curve) -> bool {
    let f = curve.f;
    f.add(
        f.mul(4, f.mul(f.mul(curve.a, curve.a), curve.a)),
        f.mul(27, f.mul(curve.b, curve.b)),
    ) != 0
}

pub(crate) fn leading_coefficient_check(curve: Curve, d: usize, matrix: &Matrix) {
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
