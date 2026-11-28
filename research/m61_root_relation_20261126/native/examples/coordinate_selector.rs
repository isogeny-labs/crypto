//! Exhaustive toy screen for coordinate-selected known-coefficient shifts.
//! The target scalar is used only to enumerate every Q; each selector
//! reads Q's coordinates and outputs u for R=Q+[u]G.
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Point {
    Zero,
    Affine(u64, u64),
}
fn power(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut out = 1;
    while n != 0 {
        if n & 1 != 0 {
            out = out * a % p;
        }
        a = a * a % p;
        n >>= 1;
    }
    out
}
fn neg(point: Point, p: u64) -> Point {
    match point {
        Point::Zero => Point::Zero,
        Point::Affine(x, y) => Point::Affine(x, if y == 0 { 0 } else { p - y }),
    }
}
fn add(left: Point, right: Point, p: u64) -> Point {
    match (left, right) {
        (Point::Zero, q) | (q, Point::Zero) => q,
        (Point::Affine(x1, y1), Point::Affine(x2, y2)) => {
            if x1 == x2 && (y1 + y2) % p == 0 {
                return Point::Zero;
            }
            let (numerator, denominator) = if x1 == x2 {
                ((3 * x1 * x1 + p - 3) % p, 2 * y1 % p)
            } else {
                ((y2 + p - y1) % p, (x2 + p - x1) % p)
            };
            let slope = numerator * power(denominator, p - 2, p) % p;
            let x3 = (slope * slope + 2 * p - x1 - x2) % p;
            Point::Affine(x3, (slope * ((x1 + p - x3) % p) + p - y1) % p)
        }
    }
}
fn multiply(mut k: u64, mut point: Point, p: u64) -> Point {
    let mut out = Point::Zero;
    while k != 0 {
        if k & 1 != 0 {
            out = add(out, point, p);
        }
        k >>= 1;
        if k != 0 {
            point = add(point, point, p);
        }
    }
    out
}
fn lift(x: u64, p: u64) -> Option<Point> {
    let rhs = (x * x % p * x % p + p - 3 * x % p + 40) % p;
    let y = power(rhs, (p + 1) / 4, p);
    (y * y % p == rhs && y != 0).then_some(Point::Affine(x, y.min(p - y)))
}
fn screen(p: u64, root_order: u64, root_generator: u64, h: u64, r: u64) {
    let mut x = 1;
    let mut base = Vec::new();
    for _ in 0..root_order {
        if let Some(point) = lift(x, p) {
            base.push(point);
        }
        x = x * root_generator % p;
    }
    assert_eq!(x, 1);
    let signed_base = base
        .iter()
        .flat_map(|&point| {
            let projected = multiply(h, point, p);
            [projected, neg(projected, p)]
        })
        .filter(|&point| point != Point::Zero)
        .collect::<HashSet<_>>();
    let generator = base
        .iter()
        .map(|&point| multiply(h, point, p))
        .find(|&point| point != Point::Zero)
        .unwrap();
    assert_eq!(multiply(r, generator, p), Point::Zero);
    let mut multiples = Vec::with_capacity(r as usize);
    let mut point = Point::Zero;
    for _ in 0..r {
        multiples.push(point);
        point = add(point, generator, p);
    }
    assert_eq!(point, Point::Zero);

    let mut hits = [0_u64; 4];
    let mut coordinate_union = 0_u64;
    let mut fixed_shift_union = 0_u64;
    let mut q = generator;
    for _ in 1..r {
        let Point::Affine(qx, qy) = q else {
            panic!("nonzero scalar gave infinity");
        };
        let coefficients = [qx % r, qy % r, (qx + qy) % r, (qx * qx) % r];
        let mut any_coordinate_hit = false;
        for (j, &u) in coefficients.iter().enumerate() {
            let image = add(q, multiples[u as usize], p);
            let hit = signed_base.contains(&image);
            hits[j] += u64::from(hit);
            any_coordinate_hit |= hit;
        }
        coordinate_union += u64::from(any_coordinate_hit);
        let mut any_fixed_hit = false;
        for &shift in &multiples[..4] {
            any_fixed_hit |= signed_base.contains(&add(q, shift, p));
        }
        fixed_shift_union += u64::from(any_fixed_hit);
        q = add(q, generator, p);
    }
    assert_eq!(q, Point::Zero);
    let fixed_shift_hits = signed_base.len();
    println!("coordinate_selector p={p} subgroup_order={r} root_columns={} signed_projected_base={} fixed_shift_hits={fixed_shift_hits} full_target_count={} x_hits={} y_hits={} x_plus_y_hits={} x_squared_hits={} coordinate_four_union={coordinate_union} fixed_four_union={fixed_shift_union} precomputed_generator_multiples={r} target_enumeration_adds={} coordinate_evaluations={} coordinate_group_adds_after_table={} fixed_control_adds_after_table={}", base.len(), signed_base.len(), r - 1, hits[0], hits[1], hits[2], hits[3], r - 1, 4 * (r - 1), 4 * (r - 1), 4 * (r - 1));
}
fn main() {
    for (p, root_order, root_generator, h, r) in [
        (8191, 42, 7036, 136, 59),
        (131071, 102, 37206, 37, 3541),
        (524287, 171, 477103, 32, 16421),
    ] {
        screen(p, root_order, root_generator, h, r);
    }
}
