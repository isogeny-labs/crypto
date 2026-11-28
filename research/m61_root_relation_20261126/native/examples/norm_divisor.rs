//! Riemann--Roch norm certificates for full and cofactor-projected
//! seven-point relations. This verifies a predicate, not a locator.
use ecdlp_pieces::{selected_root_factor_base, Point, P, R, ROOT_ORDER, SELECTED};
use std::collections::HashSet;

fn add(a: u64, b: u64) -> u64 {
    let s = a + b;
    if s >= P {
        s - P
    } else {
        s
    }
}
fn sub(a: u64, b: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        P - (b - a)
    }
}
fn mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}
fn pow(mut a: u64, mut n: u64) -> u64 {
    let mut out = 1;
    while n != 0 {
        if n & 1 != 0 {
            out = mul(out, a);
        }
        a = mul(a, a);
        n >>= 1;
    }
    out
}
fn coords(point: Point) -> (u64, u64) {
    let Point::Affine { x, y } = point else {
        panic!("affine point required")
    };
    (x, y)
}
fn eval7(point: Point) -> [u64; 7] {
    let (x, y) = coords(point);
    let x2 = mul(x, x);
    [1, x, x2, mul(x2, x), y, mul(x, y), mul(x2, y)]
}
fn eval8(point: Point) -> [u64; 8] {
    let (x, y) = coords(point);
    let x2 = mul(x, x);
    let x3 = mul(x2, x);
    [1, x, x2, x3, mul(x3, x), y, mul(x, y), mul(x2, y)]
}
fn nullvector<const N: usize>(mut rows: [[u64; N]; N]) -> [u64; N] {
    let mut pivots = Vec::new();
    let mut rank = 0;
    for col in 0..N {
        let Some(pivot) = (rank..N).find(|&row| rows[row][col] != 0) else {
            continue;
        };
        rows.swap(rank, pivot);
        let inv = pow(rows[rank][col], P - 2);
        for entry in &mut rows[rank][col..] {
            *entry = mul(*entry, inv);
        }
        for row in 0..N {
            if row == rank {
                continue;
            }
            let factor = rows[row][col];
            for j in col..N {
                rows[row][j] = sub(rows[row][j], mul(factor, rows[rank][j]));
            }
        }
        pivots.push(col);
        rank += 1;
    }
    assert_eq!(rank, N - 1);
    let free = (0..N).find(|col| !pivots.contains(col)).unwrap();
    let mut vector = [0; N];
    vector[free] = 1;
    for (row, &col) in pivots.iter().enumerate() {
        vector[col] = sub(0, rows[row][free]);
    }
    vector
}
fn product(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, &ai) in a.iter().enumerate() {
        for (j, &bj) in b.iter().enumerate() {
            out[i + j] = add(out[i + j], mul(ai, bj));
        }
    }
    out
}
fn norm(a: &[u64], b: &[u64]) -> Vec<u64> {
    let aa = product(a, a);
    let fb = product(&[40, P - 3, 0, 1], &product(b, b));
    (0..aa.len().max(fb.len()))
        .map(|i| sub(*aa.get(i).unwrap_or(&0), *fb.get(i).unwrap_or(&0)))
        .collect()
}
fn evaluate(coefficients: &[u64], x: u64) -> u64 {
    coefficients
        .iter()
        .rev()
        .fold(0, |value, &coefficient| add(mul(value, x), coefficient))
}
fn divide_linear(coefficients: &[u64], root: u64) -> Vec<u64> {
    let degree = coefficients.len() - 1;
    let mut quotient = vec![0; degree];
    quotient[degree - 1] = coefficients[degree];
    for i in (1..degree).rev() {
        quotient[i - 1] = add(coefficients[i], mul(root, quotient[i]));
    }
    assert_eq!(add(coefficients[0], mul(root, quotient[0])), 0);
    quotient
}
fn distinct_x(points: &[Point]) {
    assert_eq!(
        points
            .iter()
            .map(|&point| coords(point).0)
            .collect::<HashSet<_>>()
            .len(),
        points.len()
    );
}
fn main() {
    let base = selected_root_factor_base();
    let first_six = &base[1..7];
    assert!(first_six
        .iter()
        .all(|&point| pow(coords(point).0, ROOT_ORDER) == 1));
    let sum = first_six.iter().fold(Point::Infinity, |total, &point| {
        SELECTED.add(total, point).unwrap()
    });

    let full_tail = SELECTED.neg(sum);
    let full_points = [
        first_six[0],
        first_six[1],
        first_six[2],
        first_six[3],
        first_six[4],
        first_six[5],
        full_tail,
    ];
    distinct_x(&full_points);
    let full_coefficients = nullvector(full_points.map(eval7));
    let full_norm = norm(&full_coefficients[..4], &full_coefficients[4..]);
    assert_eq!(full_norm.len(), 8);
    assert_ne!(full_norm[7], 0);
    assert!(full_points
        .iter()
        .all(|&point| evaluate(&full_norm, coords(point).0) == 0));
    println!("full_sum_norm base_roots=6 seven_distinct_x=true norm_degree=7 norm_lead={} tail_x={} tail_in_root_base={}", full_norm[7], coords(full_tail).0, pow(coords(full_tail).0, ROOT_ORDER) == 1);

    let full_order_point = Point::Affine {
        x: 0,
        y: 313_662_643_971_843_112,
    };
    let torsion = SELECTED.mul(R, full_order_point).unwrap();
    assert_ne!(torsion, Point::Infinity);
    assert_eq!(SELECTED.mul(7, torsion).unwrap(), Point::Infinity);
    let target = SELECTED.mul(7, sum).unwrap();
    assert_ne!(target, Point::Infinity);
    let inverse_seven = (1..7)
        .find_map(|k| (k * R + 1).is_multiple_of(7).then_some((k * R + 1) / 7))
        .unwrap();
    assert_eq!(7 * inverse_seven % R, 1);
    let prime_lift = SELECTED.mul(inverse_seven, target).unwrap();
    assert_eq!(SELECTED.mul(7, prime_lift).unwrap(), target);
    let torsion_shift = SELECTED.add(sum, SELECTED.neg(prime_lift)).unwrap();
    assert_eq!(SELECTED.mul(7, torsion_shift).unwrap(), Point::Infinity);
    let torsion_index = (0..7)
        .find(|&index| SELECTED.mul(index, torsion).unwrap() == torsion_shift)
        .unwrap();
    assert_ne!(torsion_index, 0);
    let descent_tail = SELECTED.neg(SELECTED.add(prime_lift, torsion_shift).unwrap());
    assert_eq!(descent_tail, full_tail);
    assert_ne!(pow(coords(descent_tail).0, ROOT_ORDER), 1);
    println!("planted_target_norm target_x={} inverse_seven={inverse_seven} torsion_index={torsion_index} six_root_factors=6 appended_tail_x={} norm_degree=7", coords(target).0, coords(descent_tail).0);
    let projected_tail = SELECTED.add(torsion, SELECTED.neg(sum)).unwrap();
    let projected_points = [
        first_six[0],
        first_six[1],
        first_six[2],
        first_six[3],
        first_six[4],
        first_six[5],
        projected_tail,
    ];
    let mut eight_points = projected_points.to_vec();
    eight_points.push(SELECTED.neg(torsion));
    distinct_x(&eight_points);
    let numerator = nullvector(std::array::from_fn(|i| eval8(eight_points[i])));
    let numerator_norm = norm(&numerator[..5], &numerator[5..]);
    assert_eq!(numerator_norm.len(), 9);
    assert_ne!(numerator_norm[8], 0);
    assert!(eight_points
        .iter()
        .all(|&point| evaluate(&numerator_norm, coords(point).0) == 0));
    let quotient = divide_linear(&numerator_norm, coords(torsion).0);
    assert_eq!(quotient.len(), 8);
    assert!(projected_points
        .iter()
        .all(|&point| evaluate(&quotient, coords(point).0) == 0));
    let projected_sum = projected_points
        .iter()
        .fold(Point::Infinity, |total, &point| {
            SELECTED.add(total, point).unwrap()
        });
    assert_eq!(projected_sum, torsion);
    println!("torsion_sum_norm base_roots=6 eight_distinct_x=true numerator_degree=8 fixed_torsion_root={} quotient_degree=7 quotient_lead={} tail_x={} tail_in_root_base={}", coords(torsion).0, quotient[7], coords(projected_tail).0, pow(coords(projected_tail).0, ROOT_ORDER) == 1);
}
