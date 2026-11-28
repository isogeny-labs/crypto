//! Fixed parameter-ratio quotient for the selected Dickson trace base.
use ecdlp_pieces::{Point, P, SELECTED};
use std::collections::HashSet;

const ORDER: usize = 7_137;
const GENERATOR: u64 = 556_975_957_790_817_848;

fn add(a: u64, b: u64) -> u64 {
    let sum = a + b;
    if sum >= P {
        sum - P
    } else {
        sum
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
fn power(mut a: u64, mut n: u64) -> u64 {
    let mut result = 1;
    while n != 0 {
        if n & 1 != 0 {
            result = mul(result, a);
        }
        a = mul(a, a);
        n >>= 1;
    }
    result
}
fn inv(a: u64) -> u64 {
    assert_ne!(a, 0);
    power(a, P - 2)
}
fn square_root(a: u64) -> Option<u64> {
    let root = power(a, (P + 1) / 4);
    (mul(root, root) == a).then_some(root.min(P - root))
}
fn trace(t: u64) -> u64 {
    add(t, inv(t))
}
fn rhs(x: u64) -> u64 {
    add(sub(mul(mul(x, x), x), mul(3, x)), 40)
}

fn main() {
    let discriminant = sub(mul(1_598, 1_598), 4);
    let root = square_root(discriminant).unwrap();
    let inverse_two = inv(2);
    for exceptional_alpha in [
        mul(add(1_598, root), inverse_two),
        mul(sub(1_598, root), inverse_two),
    ] {
        assert_eq!(
            sub(
                add(mul(exceptional_alpha, exceptional_alpha), 1),
                mul(1_598, exceptional_alpha)
            ),
            0
        );
        assert_ne!(power(exceptional_alpha, (P - 1) / 3), 1);
    }
    let q = GENERATOR;
    let alpha = power(q, 3);
    assert_ne!(alpha, 1);
    assert_eq!(power(q, ORDER as u64), 1);
    assert_eq!(power(alpha, (ORDER / 3) as u64), 1);
    let mut c = 1;
    let mut alpha_values = HashSet::new();
    let selected_j = mul(sub(0, 6_912), inv(1_596));
    assert_eq!(selected_j, 641_475_122_863_959_967);
    let mut same_j_as_selected = 0;
    for _ in 0..ORDER / 3 {
        assert!(alpha_values.insert(c));
        if c != 1 {
            let common_root_condition = sub(add(mul(c, c), 1), mul(1_598, c));
            assert_ne!(common_root_condition, 0);
            let s = add(c, inv(c));
            let invariant_i = add(sub(mul(s, s), mul(1_600, s)), 2_550_412);
            let invariant_j = sub(
                0,
                mul(2, mul(mul(sub(s, 3_194), sub(s, 800)), add(s, 1_594))),
            );
            let discriminant = sub(mul(4, power(invariant_i, 3)), mul(invariant_j, invariant_j));
            let expected_discriminant = mul(
                68_774_832,
                mul(mul(sub(s, 2), sub(s, 2)), mul(sub(s, 1_598), sub(s, 1_598))),
            );
            assert_eq!(discriminant, expected_discriminant);
            assert_ne!(discriminant, 0);
            let quotient_j = mul(mul(6_912, power(invariant_i, 3)), inv(discriminant));
            same_j_as_selected += usize::from(quotient_j == selected_j);
        }
        c = mul(c, alpha);
    }
    assert_eq!(c, 1);
    assert_eq!(same_j_as_selected, 0);

    let mut t = 1;
    let mut seen_u = HashSet::new();
    let mut both_square = 0;
    let mut both_nonsquare = 0;
    let mut mixed = 0;
    let mut checked_pair_sums = 0;
    let mut three_distinct_pair_sums = 0;
    let zeta = power(q, (ORDER / 3) as u64);
    assert_eq!(power(zeta, 3), 1);
    assert_ne!(zeta, 1);
    for _ in 0..ORDER {
        let u = power(t, 3);
        if seen_u.insert(u) {
            let x1 = trace(t);
            let x2 = trace(mul(q, t));
            let f = add(add(mul(u, u), mul(40, u)), 1);
            let g = add(add(mul(alpha, mul(u, u)), mul(40, u)), inv(alpha));
            let a = mul(f, inv(u));
            let b = mul(g, inv(u));
            assert_eq!(rhs(x1), a);
            assert_eq!(rhs(x2), b);
            let y1 = square_root(a);
            let y2 = square_root(b);
            match (y1, y2) {
                (Some(y1), Some(y2)) => {
                    both_square += 1;
                    let z = mul(u, mul(y1, y2));
                    assert_eq!(mul(z, z), mul(f, g));
                    let p1 = Point::Affine { x: x1, y: y1 };
                    let p2 = Point::Affine { x: x2, y: y2 };
                    let Point::Affine { x: sum_x, .. } = SELECTED.add(p1, p2).unwrap() else {
                        panic!("ratio pair unexpectedly sums to infinity");
                    };
                    let dx = sub(x2, x1);
                    assert_ne!(dx, 0);
                    let numerator = sub(add(a, b), mul(2, mul(z, inv(u))));
                    let quotient_x = sub(mul(numerator, inv(mul(dx, dx))), add(x1, x2));
                    assert_eq!(sum_x, quotient_x);
                    checked_pair_sums += 1;
                    let orbit_sums = (0..3)
                        .map(|i| {
                            let ti = mul(t, power(zeta, i));
                            let pi = Point::Affine {
                                x: trace(ti),
                                y: y1,
                            };
                            let qi = Point::Affine {
                                x: trace(mul(q, ti)),
                                y: y2,
                            };
                            SELECTED.add(pi, qi).unwrap()
                        })
                        .collect::<Vec<_>>();
                    assert_eq!(
                        orbit_sums.iter().fold(Point::Infinity, |acc, &point| {
                            SELECTED.add(acc, point).unwrap()
                        }),
                        Point::Infinity
                    );
                    let three = orbit_sums
                        .iter()
                        .map(|&point| {
                            let Point::Affine { x, .. } = point else {
                                panic!("orbit pair unexpectedly sums to infinity");
                            };
                            x
                        })
                        .collect::<HashSet<_>>();
                    if three.len() == 3 {
                        three_distinct_pair_sums += 1;
                    }
                }
                (None, None) => {
                    both_nonsquare += 1;
                    assert!(square_root(mul(f, g)).is_some());
                }
                _ => mixed += 1,
            }
        }
        t = mul(t, q);
    }
    assert_eq!(t, 1);
    assert_eq!(seen_u.len(), ORDER / 3);
    assert_eq!(both_square + both_nonsquare + mixed, ORDER / 3);
    println!("trace_ratio_order={ORDER} quotient_u_count={} alpha={alpha} smooth_nontrivial_alpha_values={} same_j_as_selected={same_j_as_selected} both_square={} both_nonsquare={} mixed={} checked_pair_sums={checked_pair_sums} three_distinct_pair_sum_x={three_distinct_pair_sums}", seen_u.len(), alpha_values.len()-1, both_square, both_nonsquare, mixed);
}
