//! Exact 1+1+2+3 seven-row screen on selected M61.
//! Quadratic work for this restricted asymmetric support family.
use ecdlp_pieces::{selected_root_factor_base, Point, P, ROOT_GENERATOR, ROOT_ORDER, SELECTED};
use std::collections::HashSet;

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
fn lift(x: u64) -> Option<Point> {
    let rhs = (mul(mul(x, x), x) + P - mul(3, x) + 40) % P;
    let y = pow(rhs, (P + 1) / 4);
    (mul(y, y) == rhs && y != 0).then_some(Point::Affine { x, y: y.min(P - y) })
}
fn x_of(point: Point) -> u64 {
    let Point::Affine { x, .. } = point else {
        panic!("unexpected infinity")
    };
    x
}
fn signed_sums(points: &[Point]) -> Vec<Point> {
    let mut sums = Vec::with_capacity(1 << points.len());
    for mask in 0..1 << points.len() {
        let mut sum = Point::Infinity;
        for (i, &point) in points.iter().enumerate() {
            let signed = if mask & (1 << i) == 0 {
                point
            } else {
                SELECTED.neg(point)
            };
            sum = SELECTED.add(sum, signed).unwrap();
        }
        sums.push(sum);
    }
    sums
}
fn main() {
    let base = selected_root_factor_base();
    let projected = base
        .iter()
        .map(|&point| SELECTED.mul(7, point).unwrap())
        .collect::<Vec<_>>();
    let mut singleton_pair_xs = HashSet::with_capacity(base.len() * (base.len() - 1));
    let mut singleton_pair_tests = 0_u64;
    for i in 0..projected.len() {
        for j in i + 1..projected.len() {
            for &signed_j in &[projected[j], SELECTED.neg(projected[j])] {
                singleton_pair_xs.insert(x_of(SELECTED.add(projected[i], signed_j).unwrap()));
                singleton_pair_tests += 1;
            }
        }
    }
    assert_eq!(singleton_pair_xs.len() as u64, singleton_pair_tests);

    let mut root = 1;
    let mut antipodal_sums = Vec::new();
    for _ in 0..ROOT_ORDER / 2 {
        if let (Some(a), Some(b)) = (lift(root), lift(P - root)) {
            let a = SELECTED.mul(7, a).unwrap();
            let b = SELECTED.mul(7, b).unwrap();
            antipodal_sums.extend(signed_sums(&[a, b]));
        }
        root = mul(root, ROOT_GENERATOR);
    }
    assert_eq!(antipodal_sums.len(), 616 * 4);

    let third_root = pow(ROOT_GENERATOR, ROOT_ORDER / 3);
    let third_root_squared = mul(third_root, third_root);
    let mut root = 1;
    let mut triple_sums = Vec::new();
    for _ in 0..ROOT_ORDER / 3 {
        if let [Some(a), Some(b), Some(c)] = [
            lift(root),
            lift(mul(root, third_root)),
            lift(mul(root, third_root_squared)),
        ] {
            let points = [a, b, c].map(|point| SELECTED.mul(7, point).unwrap());
            triple_sums.extend(signed_sums(&points));
        }
        root = mul(root, ROOT_GENERATOR);
    }
    assert_eq!(triple_sums.len(), 194 * 8);

    let mut mixed_tests = 0_u64;
    let mut raw_x_matches = 0_u64;
    for &pair_sum in &antipodal_sums {
        for &triple_sum in &triple_sums {
            let image = SELECTED.add(pair_sum, triple_sum).unwrap();
            if let Point::Affine { x, .. } = image {
                raw_x_matches += u64::from(singleton_pair_xs.contains(&x));
            }
            mixed_tests += 1;
        }
    }
    assert_eq!(raw_x_matches, 0);
    println!("mixed_orbit_seven base_columns={} singleton_pair_tests={singleton_pair_tests} singleton_pair_x_classes={} antipodal_orbits=616 triple_orbits=194 mixed_sum_tests={mixed_tests} raw_x_matches={raw_x_matches}", base.len(), singleton_pair_xs.len());
}
