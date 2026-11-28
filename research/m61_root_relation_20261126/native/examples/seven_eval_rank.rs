//! Seven-point Riemann--Roch determinant on the selected M61 trace base.
//! This certifies an algebraic relation predicate, not a fast locator.
use ecdlp_pieces::{Point, P, SELECTED};
use std::collections::HashSet;

const TRACE_ORDER: usize = 7_137;
const TRACE_GENERATOR: u64 = 556_975_957_790_817_848;

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
fn eval(point: Point) -> [u64; 7] {
    let Point::Affine { x, y } = point else {
        panic!("affine point required")
    };
    let x2 = mul(x, x);
    [1, x, y, x2, mul(x, y), mul(x2, x), mul(x2, y)]
}
fn determinant(mut rows: [[u64; 7]; 7]) -> u64 {
    let mut result = 1;
    for col in 0..7 {
        let Some(pivot) = (col..7).find(|&row| rows[row][col] != 0) else {
            return 0;
        };
        if pivot != col {
            rows.swap(pivot, col);
            result = sub(0, result);
        }
        let diagonal = rows[col][col];
        result = mul(result, diagonal);
        let inv = pow(diagonal, P - 2);
        for row in col + 1..7 {
            let factor = mul(rows[row][col], inv);
            for j in col..7 {
                rows[row][j] = sub(rows[row][j], mul(factor, rows[col][j]));
            }
        }
    }
    result
}
fn main() {
    let mut t = 1;
    let mut seen_x = HashSet::new();
    let mut base = Vec::new();
    let mut base_exponents = Vec::new();
    for exponent in 0..TRACE_ORDER {
        let x = add(t, pow(t, P - 2));
        if seen_x.insert(x) {
            let rhs = add(sub(mul(mul(x, x), x), mul(3, x)), 40);
            let y = pow(rhs, (P + 1) / 4);
            if mul(y, y) == rhs {
                let point = Point::Affine {
                    x,
                    y: y.min(sub(0, y)),
                };
                assert!(SELECTED.is_on_curve(point));
                base.push(point);
                base_exponents.push(exponent);
            }
        }
        t = mul(t, TRACE_GENERATOR);
    }
    assert_eq!(t, 1);
    assert_eq!(base.len(), 1_703);
    let d = TRACE_ORDER / 3;
    let orbit_ids = base_exponents[..7]
        .iter()
        .map(|&i| {
            [i, (i + d) % TRACE_ORDER, (i + 2 * d) % TRACE_ORDER]
                .into_iter()
                .flat_map(|j| [j, (TRACE_ORDER - j) % TRACE_ORDER])
                .min()
                .unwrap()
        })
        .collect::<HashSet<_>>();
    assert_eq!(orbit_ids.len(), 7);
    let independent = std::array::from_fn(|i| eval(base[i]));
    let independent_det = determinant(independent);
    assert_ne!(independent_det, 0);
    let mut sum = Point::Infinity;
    for &point in &base[..6] {
        sum = SELECTED.add(sum, point).unwrap();
    }
    let seventh = SELECTED.neg(sum);
    let Point::Affine { x: seventh_x, .. } = seventh else {
        panic!("unexpected infinity")
    };
    assert!(base[..6].iter().all(|point| match point {
        Point::Affine { x, .. } => *x != seventh_x,
        Point::Infinity => false,
    }));
    assert!(base.iter().all(|point| match point {
        Point::Affine { x, .. } => *x != seventh_x,
        Point::Infinity => false,
    }));
    assert_eq!(SELECTED.add(sum, seventh).unwrap(), Point::Infinity);
    let dependent = std::array::from_fn(|i| eval(if i == 6 { seventh } else { base[i] }));
    assert_eq!(determinant(dependent), 0);
    // Seven independent evaluation vectors have an exterior-cube basis
    // of C(7,3)=35 triple wedges, so no linear Pluecker rank collapse.
    println!("trace_columns={} first_seven_orbits_distinct=true independent_seven_det={independent_det} constructed_relation_tail={seventh:?} tail_in_base=false relation_det=0 exterior_triple_rank=35", base.len());
}
