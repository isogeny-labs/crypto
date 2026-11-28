//! Hasse-interval BSGS for selected, quotient, CM, and engineered trace curves.
use ecdlp_pieces::{P, ROOT_GENERATOR, ROOT_ORDER};
use std::collections::{HashMap, HashSet};

const HASSE_HALF_WIDTH: u64 = 3_037_000_499; // floor(2*sqrt(P))
const STEP: u64 = 77_936; // STEP^2 > 2*HASSE_HALF_WIDTH
const _: () = {
    assert!(HASSE_HALF_WIDTH * HASSE_HALF_WIDTH <= 4 * P);
    assert!((HASSE_HALF_WIDTH + 1) * (HASSE_HALF_WIDTH + 1) > 4 * P);
    assert!(STEP * STEP > 2 * HASSE_HALF_WIDTH);
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Point {
    Zero,
    Affine(u64, u64),
}

fn add_field(a: u64, b: u64) -> u64 {
    let sum = a + b;
    if sum >= P {
        sum - P
    } else {
        sum
    }
}
fn sub_field(a: u64, b: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        P - (b - a)
    }
}
fn mul_field(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}
fn pow_field(mut a: u64, mut exponent: u64) -> u64 {
    let mut result = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = mul_field(result, a);
        }
        a = mul_field(a, a);
        exponent >>= 1;
    }
    result
}
fn is_prime_u64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for prime in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n == prime {
            return true;
        }
        if n.is_multiple_of(prime) {
            return false;
        }
    }
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    for base in [2, 325, 9_375, 28_178, 450_775, 9_780_504, 1_795_265_022] {
        let base = base % n;
        if base == 0 {
            continue;
        }
        let mut x = pow_mod(base, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut witness = true;
        for _ in 1..s {
            x = mul_mod(x, x, n);
            if x == n - 1 {
                witness = false;
                break;
            }
        }
        if witness {
            return false;
        }
    }
    true
}
fn mul_mod(a: u64, b: u64, modulus: u64) -> u64 {
    ((a as u128 * b as u128) % modulus as u128) as u64
}
fn pow_mod(mut a: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;
    a %= modulus;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = mul_mod(result, a, modulus);
        }
        a = mul_mod(a, a, modulus);
        exponent >>= 1;
    }
    result
}
fn neg(point: Point) -> Point {
    match point {
        Point::Zero => Point::Zero,
        Point::Affine(x, y) => Point::Affine(x, sub_field(0, y)),
    }
}
fn add(left: Point, right: Point, a: u64, b: u64) -> Point {
    match (left, right) {
        (Point::Zero, q) | (q, Point::Zero) => q,
        (Point::Affine(x1, y1), Point::Affine(x2, y2)) => {
            debug_assert_eq!(mul_field(y1, y1), rhs(x1, a, b));
            debug_assert_eq!(mul_field(y2, y2), rhs(x2, a, b));
            if x1 == x2 && add_field(y1, y2) == 0 {
                return Point::Zero;
            }
            let (numerator, denominator) = if x1 == x2 {
                (
                    add_field(mul_field(3, mul_field(x1, x1)), a),
                    add_field(y1, y1),
                )
            } else {
                (sub_field(y2, y1), sub_field(x2, x1))
            };
            let slope = mul_field(numerator, pow_field(denominator, P - 2));
            let x3 = sub_field(sub_field(mul_field(slope, slope), x1), x2);
            let y3 = sub_field(mul_field(slope, sub_field(x1, x3)), y1);
            let result = Point::Affine(x3, y3);
            debug_assert_eq!(mul_field(y3, y3), rhs(x3, a, b));
            result
        }
    }
}
fn multiply(mut scalar: u64, mut point: Point, a: u64, b: u64) -> Point {
    let mut result = Point::Zero;
    while scalar != 0 {
        if scalar & 1 != 0 {
            result = add(result, point, a, b);
        }
        scalar >>= 1;
        if scalar != 0 {
            point = add(point, point, a, b);
        }
    }
    result
}
fn rhs(x: u64, a: u64, b: u64) -> u64 {
    add_field(add_field(mul_field(mul_field(x, x), x), mul_field(a, x)), b)
}
fn first_point(a: u64, b: u64) -> Point {
    // On a=0 curves, (0,sqrt(b)) has order three; skip it for BSGS.
    for x in (if a == 0 { 1 } else { 0 })..1000 {
        let value = rhs(x, a, b);
        if value == 0 {
            return Point::Affine(x, 0);
        }
        let y = pow_field(value, (P + 1) / 4); // P = 3 mod 4
        if mul_field(y, y) == value {
            return Point::Affine(x, y);
        }
    }
    panic!("no lift in first thousand x-coordinates");
}

fn signed_three_rows(
    columns: &[(u64, Point)],
    lookup: &HashMap<Point, u64>,
    orbit_ids: &HashMap<u64, usize>,
    b: u64,
) -> (usize, usize) {
    let mut total = 0;
    let mut cross_orbit = 0;
    for i in 0..columns.len() {
        for j in i + 1..columns.len() {
            for second in [columns[j].1, neg(columns[j].1)] {
                let third = neg(add(columns[i].1, second, P - 3, b));
                if let Some(&third_x) = lookup.get(&third) {
                    if third_x > columns[j].0 {
                        total += 1;
                        if orbit_ids[&columns[i].0] != orbit_ids[&columns[j].0]
                            || orbit_ids[&columns[j].0] != orbit_ids[&third_x]
                        {
                            cross_orbit += 1;
                        }
                    }
                }
            }
        }
    }
    (total, cross_orbit)
}

fn signed_pair_sum_x_collisions(columns: &[(u64, Point)], b: u64) -> usize {
    let mut xs = Vec::with_capacity(columns.len() * (columns.len() - 1));
    for i in 0..columns.len() {
        for j in i + 1..columns.len() {
            for second in [columns[j].1, neg(columns[j].1)] {
                let Point::Affine(x, _) = add(columns[i].1, second, P - 3, b) else {
                    panic!("distinct columns unexpectedly sum to zero");
                };
                xs.push(x);
            }
        }
    }
    xs.sort_unstable();
    xs.windows(2).filter(|pair| pair[0] == pair[1]).count()
}

fn engineered_ratio_census(h: u64, q: u64, s: u64, b: u64) {
    let mut t = 1;
    let mut seen_t = HashSet::new();
    let mut powers = Vec::with_capacity(7_137);
    let mut base = HashMap::new();
    for _ in 0..7_137 {
        assert!(seen_t.insert(t));
        powers.push(t);
        let x = add_field(t, pow_field(t, P - 2));
        let value = rhs(x, P - 3, b);
        let y = pow_field(value, (P + 1) / 4);
        if mul_field(y, y) == value {
            base.insert(x, Point::Affine(x, y.min(sub_field(0, y))));
        }
        t = mul_field(t, h);
    }
    assert_eq!(t, 1);
    assert_eq!(base.len(), 1_775);
    let mut seen_orbit = vec![false; 7_137];
    let mut orbit_ids = HashMap::new();
    let mut free_rows = 0;
    let mut orbit_columns = 0;
    for i in 0..7_137 {
        if seen_orbit[i] {
            continue;
        }
        let d = 7_137 / 3;
        for j in [i, (i + d) % 7_137, (i + 2 * d) % 7_137] {
            seen_orbit[j] = true;
            seen_orbit[(7_137 - j) % 7_137] = true;
        }
        let xs = [i, (i + d) % 7_137, (i + 2 * d) % 7_137]
            .map(|j| add_field(powers[j], pow_field(powers[j], P - 2)));
        if base.contains_key(&xs[0]) {
            let points = xs.map(|x| *base.get(&x).unwrap());
            let sum = points
                .into_iter()
                .fold(Point::Zero, |acc, point| add(acc, point, P - 3, b));
            assert_eq!(sum, Point::Zero);
            free_rows += 1;
            let unique = xs.into_iter().collect::<HashSet<_>>();
            orbit_columns += unique.len();
            for x in unique {
                assert!(orbit_ids.insert(x, free_rows).is_none());
            }
        }
    }
    assert!(seen_orbit.iter().all(|&seen| seen));
    assert_eq!(orbit_columns, base.len());
    assert_eq!(free_rows, 592);
    let mut full = HashMap::new();
    let mut projected = HashMap::new();
    for (&x, &point) in &base {
        assert!(full.insert(point, x).is_none());
        assert!(full.insert(neg(point), x).is_none());
        let image = multiply(32, point, P - 3, b);
        assert_ne!(image, Point::Zero);
        assert!(projected.insert(image, x).is_none());
        assert!(projected.insert(neg(image), x).is_none());
    }
    let mut pairs = HashSet::new();
    let mut full_completions = 0;
    let mut projected_completions = 0;
    t = 1;
    for _ in 0..7_137 {
        let qt = mul_field(q, t);
        let x1 = add_field(t, pow_field(t, P - 2));
        let x2 = add_field(qt, pow_field(qt, P - 2));
        if let (Some(&p1), Some(&p2)) = (base.get(&x1), base.get(&x2)) {
            assert_ne!(x1, x2);
            let key = (x1.min(x2), x1.max(x2));
            if pairs.insert(key) {
                let u = pow_field(t, 3);
                let Point::Affine(_, y1) = p1 else {
                    unreachable!()
                };
                let Point::Affine(_, y2) = p2 else {
                    unreachable!()
                };
                let w = mul_field(y2, pow_field(mul_field(s, y1), P - 2));
                let lhs = mul_field(w, w);
                let rhs_conic = mul_field(
                    add_field(u, pow_field(s, P - 4)),
                    pow_field(add_field(u, s), P - 2),
                );
                assert_eq!(lhs, rhs_conic);
                assert_eq!(
                    u,
                    mul_field(
                        sub_field(pow_field(s, P - 4), mul_field(s, lhs)),
                        pow_field(sub_field(lhs, 1), P - 2),
                    )
                );
                for signed_p2 in [p2, neg(p2)] {
                    let sum = add(p1, signed_p2, P - 3, b);
                    if let Some(&third) = full.get(&neg(sum)) {
                        if third != x1 && third != x2 {
                            full_completions += 1;
                        }
                    }
                    let image = multiply(32, sum, P - 3, b);
                    if let Some(&third) = projected.get(&neg(image)) {
                        if third != x1 && third != x2 {
                            projected_completions += 1;
                        }
                    }
                }
            }
        }
        t = mul_field(t, h);
    }
    assert_eq!(pairs.len(), 888);
    assert_eq!(full_completions, 0);
    assert_eq!(projected_completions, 0);
    let zeta = pow_field(h, 7_137 / 3);
    let special_ratios = [q, pow_field(q, P - 2)]
        .into_iter()
        .flat_map(|initial| (0..3).map(move |j| mul_field(initial, pow_field(zeta, j))))
        .collect::<HashSet<_>>();
    assert_eq!(special_ratios.len(), 6);
    let mut special_pair_union = HashSet::new();
    for ratio in special_ratios.iter().copied() {
        let mut ratio_pairs = HashSet::new();
        for &parameter in &powers {
            let next = mul_field(ratio, parameter);
            let x1 = add_field(parameter, pow_field(parameter, P - 2));
            let x2 = add_field(next, pow_field(next, P - 2));
            if base.contains_key(&x1) && base.contains_key(&x2) {
                assert_ne!(x1, x2);
                ratio_pairs.insert((x1.min(x2), x1.max(x2)));
            }
        }
        assert_eq!(ratio_pairs.len(), 888);
        special_pair_union.extend(ratio_pairs);
    }
    let mut columns = base
        .iter()
        .map(|(&x, &point)| (x, point))
        .collect::<Vec<_>>();
    columns.sort_unstable_by_key(|entry| entry.0);
    let projected_columns = columns
        .iter()
        .map(|&(x, point)| (x, multiply(32, point, P - 3, b)))
        .collect::<Vec<_>>();
    let full_three = signed_three_rows(&columns, &full, &orbit_ids, b);
    let projected_three = signed_three_rows(&projected_columns, &projected, &orbit_ids, b);
    assert_eq!(full_three, (591, 0));
    assert_eq!(projected_three, full_three);
    let full_pair_collisions = signed_pair_sum_x_collisions(&columns, b);
    let projected_pair_collisions = signed_pair_sum_x_collisions(&projected_columns, b);
    assert_eq!(full_pair_collisions, 0);
    assert_eq!(projected_pair_collisions, 0);
    println!("engineered_trace_base_columns={} free_orbit_rank={free_rows} fixed_ratio_pairs={} special_ratio_count={} special_pair_union={} full_completions={full_completions} projected_completions={projected_completions} signed_three_rows_full={full_three:?} projected={projected_three:?} signed_pair_sum_x_collisions_full={full_pair_collisions} projected={projected_pair_collisions}", base.len(), pairs.len(), special_ratios.len(), special_pair_union.len());
}

fn equal_j_trace_short_row_census(h: u64, b: u64) {
    let mut t = 1;
    let mut powers = Vec::with_capacity(7_137);
    let mut base = HashMap::new();
    for _ in 0..7_137 {
        powers.push(t);
        let x = add_field(t, pow_field(t, P - 2));
        let value = rhs(x, P - 3, b);
        let y = pow_field(value, (P + 1) / 4);
        if mul_field(y, y) == value {
            base.insert(x, Point::Affine(x, y.min(sub_field(0, y))));
        }
        t = mul_field(t, h);
    }
    assert_eq!(t, 1);
    assert_eq!(powers.iter().copied().collect::<HashSet<_>>().len(), 7_137);
    let mut seen_orbit = vec![false; 7_137];
    let mut orbit_ids = HashMap::new();
    let mut free_rows = 0;
    for i in 0..7_137 {
        if seen_orbit[i] {
            continue;
        }
        let d = 7_137 / 3;
        let xs = [i, (i + d) % 7_137, (i + 2 * d) % 7_137].map(|j| {
            seen_orbit[j] = true;
            seen_orbit[(7_137 - j) % 7_137] = true;
            add_field(powers[j], pow_field(powers[j], P - 2))
        });
        if base.contains_key(&xs[0]) {
            let sum = xs
                .into_iter()
                .fold(Point::Zero, |acc, x| add(acc, base[&x], P - 3, b));
            assert_eq!(sum, Point::Zero);
            free_rows += 1;
            for x in xs.into_iter().collect::<HashSet<_>>() {
                assert!(orbit_ids.insert(x, free_rows).is_none());
            }
        }
    }
    assert!(seen_orbit.iter().all(|&seen| seen));
    assert_eq!(orbit_ids.len(), base.len());
    let mut columns = base.into_iter().collect::<Vec<_>>();
    columns.sort_unstable_by_key(|entry| entry.0);
    let mut lookup = HashMap::new();
    let mut projected_lookup = HashMap::new();
    let projected_columns = columns
        .iter()
        .map(|&(x, point)| {
            assert!(lookup.insert(point, x).is_none());
            assert!(lookup.insert(neg(point), x).is_none());
            let image = multiply(104, point, P - 3, b);
            assert_ne!(image, Point::Zero);
            assert!(projected_lookup.insert(image, x).is_none());
            assert!(projected_lookup.insert(neg(image), x).is_none());
            (x, image)
        })
        .collect::<Vec<_>>();
    let full_three = signed_three_rows(&columns, &lookup, &orbit_ids, b);
    let projected_three = signed_three_rows(&projected_columns, &projected_lookup, &orbit_ids, b);
    let full_pair_collisions = signed_pair_sum_x_collisions(&columns, b);
    let projected_pair_collisions = signed_pair_sum_x_collisions(&projected_columns, b);
    println!("equal_j_trace_base_columns={} free_orbit_rank={free_rows} signed_three_rows_full={full_three:?} projected={projected_three:?} signed_pair_sum_x_collisions_full={full_pair_collisions} projected={projected_pair_collisions}", columns.len());

    // The T_b ~= E_b isomorphism gives an E-valued point R(t) whose
    // x-coordinate is rational in t. Test whether R(t) or P(t) +/- R(t)
    // returns to the same trace x-base, and count collisions in the sums.
    let lambda_inv = pow_field(899_262_398_837_438_475, P - 2);
    let lambda_inv2 = pow_field(lambda_inv, 2);
    let lambda_inv3 = pow_field(lambda_inv, 3);
    let b_over_3 = mul_field(b, pow_field(3, P - 2));
    let base_by_x = columns.iter().copied().collect::<HashMap<_, _>>();
    let mut lifted_parameters = 0;
    let mut second_image_base_hits = 0;
    let mut sum_base_hits = [0; 2];
    let mut sum_x_seen = HashMap::new();
    let mut sum_x_collisions = 0;
    let mut collision_witnesses = Vec::new();
    let mut projected_second_zero = 0;
    let mut projected_second_base_hits = 0;
    let mut projected_sum_base_hits = [0; 2];
    let mut projected_sum_x_classes = HashMap::<u64, Vec<(u64, usize)>>::new();
    for t in powers {
        let x = add_field(t, pow_field(t, P - 2));
        let Some(&point) = base_by_x.get(&x) else {
            continue;
        };
        lifted_parameters += 1;
        let Point::Affine(_, y) = point else {
            unreachable!()
        };
        let u = pow_field(t, 3);
        let r_x = mul_field(lambda_inv2, add_field(u, b_over_3));
        let r_y = mul_field(lambda_inv3, mul_field(u, y));
        assert_eq!(rhs(r_x, P - 3, b), mul_field(r_y, r_y));
        if base_by_x.contains_key(&r_x) {
            second_image_base_hits += 1;
        }
        let r = Point::Affine(r_x, r_y);
        let projected_r = multiply(104, r, P - 3, b);
        if projected_r == Point::Zero {
            projected_second_zero += 1;
        } else if projected_lookup.contains_key(&projected_r) {
            projected_second_base_hits += 1;
        }
        for (sign, second) in [r, neg(r)].into_iter().enumerate() {
            let sum = add(point, second, P - 3, b);
            let projected_sum = multiply(104, sum, P - 3, b);
            if projected_lookup.contains_key(&projected_sum) {
                projected_sum_base_hits[sign] += 1;
                assert_eq!(u, 1);
            }
            if let Point::Affine(projected_x, _) = projected_sum {
                projected_sum_x_classes
                    .entry(projected_x)
                    .or_default()
                    .push((t, sign));
            }
            if let Point::Affine(sum_x, _) = sum {
                if base_by_x.contains_key(&sum_x) {
                    sum_base_hits[sign] += 1;
                }
                if let Some((previous_t, previous_sign)) = sum_x_seen.insert(sum_x, (t, sign)) {
                    sum_x_collisions += 1;
                    collision_witnesses.push((previous_t, previous_sign, t, sign, sum_x));
                }
            }
        }
    }
    assert_eq!(lifted_parameters, 3_549);
    assert_eq!(second_image_base_hits, 0);
    assert_eq!(sum_base_hits, [0, 0]);
    assert_eq!(sum_x_seen.len(), 7_096);
    assert_eq!(sum_x_collisions, 2);
    assert!(collision_witnesses
        .iter()
        .all(|&(previous_t, previous_sign, t, sign, _)| {
            previous_sign == sign
                && mul_field(previous_t, t) == 1
                && pow_field(previous_t, 3) == 1
                && previous_t != t
        }));
    let projected_collisions: usize = projected_sum_x_classes
        .values()
        .map(|members| members.len() - 1)
        .sum();
    let projected_cross_inversion_classes = projected_sum_x_classes
        .values()
        .filter(|members| {
            let canonical = |t: u64| t.min(pow_field(t, P - 2));
            members
                .iter()
                .any(|&(t, _)| canonical(t) != canonical(members[0].0))
        })
        .count();
    assert_eq!(projected_second_zero, 3);
    assert_eq!(projected_second_base_hits, 0);
    assert_eq!(projected_sum_base_hits, [3, 3]);
    assert_eq!(projected_sum_x_classes.len(), 3_548);
    assert_eq!(projected_collisions, 3_550);
    assert_eq!(projected_cross_inversion_classes, 0);
    println!("equal_j_mixed_projected second_zero={projected_second_zero} second_base_hits={projected_second_base_hits} sum_base_hits={projected_sum_base_hits:?} distinct_sum_x={} collisions={projected_collisions} cross_inversion_classes={projected_cross_inversion_classes}", projected_sum_x_classes.len());
    println!("equal_j_mixed_map lifted_parameters={lifted_parameters} second_image_base_hits={second_image_base_hits} sum_base_hits={sum_base_hits:?} sum_x_distinct={} sum_x_collisions={sum_x_collisions} collision_witnesses={collision_witnesses:?}", sum_x_seen.len());
}

fn hasse_bsgs(a: u64, b: u64) -> (Point, Vec<u64>) {
    let q = first_point(a, b);
    let lower = P + 1 - HASSE_HALF_WIDTH;
    let upper = P + 1 + HASSE_HALF_WIDTH;
    let mut babies = HashMap::with_capacity(STEP as usize);
    let mut baby = Point::Zero;
    for j in 0..STEP {
        assert!(
            babies.insert(baby, j).is_none(),
            "baby collision would hide candidates"
        );
        baby = add(baby, q, a, b);
    }
    let giant_step = baby;
    let mut target = neg(multiply(lower, q, a, b));
    let mut candidates = Vec::new();
    for i in 0..=(2 * HASSE_HALF_WIDTH / STEP) {
        if let Some(&j) = babies.get(&target) {
            let candidate = lower + i * STEP + j;
            if candidate <= upper {
                candidates.push(candidate);
            }
        }
        target = add(target, neg(giant_step), a, b);
    }
    candidates.sort_unstable();
    candidates.dedup();
    (q, candidates)
}

fn integer_sqrt(value: u64) -> u64 {
    let mut low = 0_u64;
    let mut high = 1_u64 << 32;
    while low + 1 < high {
        let mid = low + (high - low) / 2;
        if (mid as u128) * (mid as u128) <= value as u128 {
            low = mid;
        } else {
            high = mid;
        }
    }
    low
}

fn count_trace_elliptic_quotient(label: &str, original_b: u64, original_order: u64) -> u64 {
    // C: v^2=t^7+b*t^4+t has quotient W^2=u^3+b*u^2+u,
    // where u=t^3 and W=t*v. Translate u=X-b/3.
    let inv3 = pow_field(3, P - 2);
    let inv27 = pow_field(27, P - 2);
    let b_over_3 = mul_field(original_b, inv3);
    let quotient_a = sub_field(1, mul_field(mul_field(original_b, original_b), inv3));
    let quotient_b = sub_field(
        mul_field(2, mul_field(pow_field(original_b, 3), inv27)),
        b_over_3,
    );
    assert_eq!(rhs(b_over_3, quotient_a, quotient_b), 0);
    let (quotient_point, quotient_candidates) = hasse_bsgs(quotient_a, quotient_b);
    assert_eq!(quotient_candidates.len(), 1);
    let quotient_order = quotient_candidates[0];
    assert_eq!(
        multiply(quotient_order, quotient_point, quotient_a, quotient_b),
        Point::Zero
    );
    println!("model={label}_trace_elliptic_quotient a={quotient_a} b={quotient_b} order={quotient_order} trace={} original_order={original_order} same_isogeny_class={} point={quotient_point:?}",
        (P + 1) as i128 - quotient_order as i128, quotient_order == original_order);
    quotient_order
}

fn main() {
    for (label, a, b, expected_order) in [
        ("selected", P - 3, 40, 2_305_843_009_662_956_113),
        ("quotient_1", P - 3, 1598, 2_305_843_011_224_697_631),
        ("quotient_2", 9573, P - 2_531_254, 2_305_843_008_686_133_067),
        ("cm_j0", 0, 40, 2_305_843_008_054_377_925),
    ] {
        let (point, candidates) = hasse_bsgs(a, b);
        assert_eq!(candidates.len(), 1, "Hasse interval must give one order");
        let order = candidates[0];
        assert_eq!(order, expected_order);
        assert_eq!(multiply(order, point, a, b), Point::Zero);
        if label == "selected" {
            count_trace_elliptic_quotient(label, b, order);
            let trace = (P + 1) as i128 - order as i128;
            let discriminant = 4 * P as i128 - trace * trace;
            assert!(discriminant > 0);
            assert_eq!(discriminant % 3, 0);
            let third = (discriminant / 3) as u64;
            let root = integer_sqrt(third);
            assert_ne!(root * root, third);
            assert_eq!(P % 4, 3);
            assert_ne!(trace, 0);
            println!("selected_trace={trace} cm_minus3_quotient={third} floor_sqrt={root} is_square=0 p_mod_4=3");
        }
        if label == "quotient_1" {
            let check_point = Point::Affine(2, 40);
            assert_eq!(multiply(order, check_point, a, b), Point::Zero);
            assert_ne!(multiply(order / 121_067, check_point, a, b), Point::Zero);
            assert_ne!(
                multiply(order / 19_046_007_675_293, check_point, a, b),
                Point::Zero
            );
        } else if label == "quotient_2" {
            let check_point = Point::Affine(1597, 1_263_391_228_187_558_620);
            assert_eq!(multiply(order, check_point, a, b), Point::Zero);
            for prime in [23, 31, 2_181_169, 1_482_691_811] {
                assert_ne!(multiply(order / prime, check_point, a, b), Point::Zero);
            }
        } else if label == "cm_j0" {
            const Q: u64 = 51_131_959_441;
            const LAMBDA: u64 = 39_764_776_730;
            assert_eq!(order % Q, 0);
            let root_point = Point::Affine(1, 2_030_123_778_694_854_804);
            let subgroup_point = multiply(order / Q, root_point, a, b);
            assert_ne!(subgroup_point, Point::Zero);
            assert_eq!(multiply(Q, subgroup_point, a, b), Point::Zero);
            let zeta = pow_field(ROOT_GENERATOR, ROOT_ORDER / 3);
            let transformed = match subgroup_point {
                Point::Affine(x, y) => Point::Affine(mul_field(zeta, x), y),
                Point::Zero => unreachable!(),
            };
            assert_eq!(multiply(LAMBDA, subgroup_point, a, b), transformed);
            assert_eq!(
                (LAMBDA as u128 * LAMBDA as u128 + LAMBDA as u128 + 1) % Q as u128,
                0
            );
            let mut x = 1;
            let mut checked_orbits = 0;
            for _ in 0..ROOT_ORDER / 3 {
                let value = rhs(x, a, b);
                let y = pow_field(value, (P + 1) / 4);
                if mul_field(y, y) == value {
                    let projected = multiply(order / Q, Point::Affine(x, y), a, b);
                    assert_ne!(projected, Point::Zero);
                    let image = match projected {
                        Point::Affine(px, py) => Point::Affine(mul_field(zeta, px), py),
                        Point::Zero => unreachable!(),
                    };
                    assert_eq!(multiply(LAMBDA, projected, a, b), image);
                    checked_orbits += 1;
                }
                x = mul_field(x, ROOT_GENERATOR);
            }
            assert_eq!(checked_orbits, 811);
            println!("cm_subgroup_order={Q} automorphism_scalar={LAMBDA} subgroup_point={subgroup_point:?} checked_orbits={checked_orbits}");
        }
        println!("model={label} a={a} b={b} point={point:?} hasse_candidates={candidates:?}");
    }

    let h = 556_975_957_790_817_848_u64;
    let k = 14;
    let s = pow_field(h, 3 * k);
    let engineered_b = add_field(s, pow_field(s, P - 2));
    assert_ne!(mul_field(engineered_b, engineered_b), 4);
    let q = pow_field(h, 2 * k);
    let alpha = pow_field(q, 3);
    assert_eq!(
        add_field(
            sub_field(
                mul_field(alpha, alpha),
                mul_field(sub_field(mul_field(engineered_b, engineered_b), 2), alpha),
            ),
            1,
        ),
        0,
    );
    let (point, candidates) = hasse_bsgs(P - 3, engineered_b);
    const PRIME_FACTOR: u64 = 72_057_594_068_108_333;
    assert!(is_prime_u64(PRIME_FACTOR));
    assert_eq!(candidates, vec![32 * PRIME_FACTOR]);
    count_trace_elliptic_quotient("engineered", engineered_b, candidates[0]);
    let subgroup_point = multiply(32, point, P - 3, engineered_b);
    assert_ne!(subgroup_point, Point::Zero);
    assert_eq!(
        multiply(PRIME_FACTOR, subgroup_point, P - 3, engineered_b),
        Point::Zero
    );
    let hk = pow_field(h, k);
    let two_torsion_x = sub_field(0, add_field(hk, pow_field(hk, P - 2)));
    let two_torsion = Point::Affine(two_torsion_x, 0);
    assert_eq!(rhs(two_torsion_x, P - 3, engineered_b), 0);
    assert_eq!(multiply(2, two_torsion, P - 3, engineered_b), Point::Zero);
    let zeta = pow_field(h, 7_137 / 3);
    assert_ne!(zeta, 1);
    assert_eq!(pow_field(zeta, 3), 1);
    let torsion_xs = (0..3)
        .map(|j| {
            let hj = mul_field(hk, pow_field(zeta, j));
            let x = sub_field(0, add_field(hj, pow_field(hj, P - 2)));
            assert_eq!(rhs(x, P - 3, engineered_b), 0);
            x
        })
        .collect::<HashSet<_>>();
    assert_eq!(torsion_xs.len(), 3);
    engineered_ratio_census(h, q, s, engineered_b);
    println!("model=engineered_trace_ratio k={k} b={engineered_b} q={q} alpha={alpha} order={} prime_factor={PRIME_FACTOR} cofactor=32 two_torsion_x={two_torsion_x} point={point:?}", candidates[0]);

    // Equality of j(E_b) and j(T_b) reduces to (b^2-3)^3=-27.
    // One nonzero root b exists over M61; test whether it is the
    // isomorphic or quadratic-twist branch by counting both curves.
    let zeta = pow_field(37, (P - 1) / 3);
    assert_ne!(zeta, 1);
    assert_eq!(pow_field(zeta, 3), 1);
    let other_b_squared = mul_field(3, sub_field(1, zeta));
    assert_eq!(pow_field(other_b_squared, (P - 1) / 2), P - 1);
    let equal_j_b_squared = mul_field(3, sub_field(1, mul_field(zeta, zeta)));
    let equal_j_b = pow_field(equal_j_b_squared, (P + 1) / 4);
    assert_eq!(mul_field(equal_j_b, equal_j_b), equal_j_b_squared);
    let (equal_j_point, equal_j_candidates) = hasse_bsgs(P - 3, equal_j_b);
    assert_eq!(equal_j_candidates.len(), 1);
    let equal_j_order = equal_j_candidates[0];
    assert_eq!(
        multiply(equal_j_order, equal_j_point, P - 3, equal_j_b),
        Point::Zero
    );
    let equal_j_trace_order = count_trace_elliptic_quotient("equal_j", equal_j_b, equal_j_order);
    const EQUAL_J_PRIME: u64 = 22_171_567_371_951_581;
    assert!(is_prime_u64(EQUAL_J_PRIME));
    assert_eq!(equal_j_order, 104 * EQUAL_J_PRIME);
    let equal_j_subgroup_point = multiply(104, equal_j_point, P - 3, equal_j_b);
    assert_ne!(equal_j_subgroup_point, Point::Zero);
    assert_eq!(
        multiply(EQUAL_J_PRIME, equal_j_subgroup_point, P - 3, equal_j_b),
        Point::Zero
    );
    let equal_j_a_trace = sub_field(
        1,
        mul_field(mul_field(equal_j_b, equal_j_b), pow_field(3, P - 2)),
    );
    let equal_j_b_trace = sub_field(
        mul_field(2, mul_field(pow_field(equal_j_b, 3), pow_field(27, P - 2))),
        mul_field(equal_j_b, pow_field(3, P - 2)),
    );
    let lambda = 899_262_398_837_438_475;
    assert_eq!(equal_j_a_trace, mul_field(P - 3, pow_field(lambda, 4)));
    assert_eq!(equal_j_b_trace, mul_field(equal_j_b, pow_field(lambda, 6)));
    assert_eq!(equal_j_order, equal_j_trace_order);
    equal_j_trace_short_row_census(h, equal_j_b);
    println!("model=equal_j b={equal_j_b} b_squared={equal_j_b_squared} order={equal_j_order} prime_factor={EQUAL_J_PRIME} cofactor=104 trace_quotient_order={equal_j_trace_order} isomorphism_lambda={lambda} subgroup_point={equal_j_subgroup_point:?}");

    // The only other nonsingular equal-j b is the quadratic-twist sign.
    let equal_j_twist_b = sub_field(0, equal_j_b);
    let (twist_point, twist_candidates) = hasse_bsgs(P - 3, equal_j_twist_b);
    assert_eq!(twist_candidates.len(), 1);
    let twist_order = twist_candidates[0];
    assert_eq!(twist_order, 2 * (P + 1) - equal_j_order);
    const TWIST_PRIME: u64 = 11_296_507_014_229;
    assert!(is_prime_u64(TWIST_PRIME));
    assert_eq!(twist_order, 204_120 * TWIST_PRIME);
    let twist_trace_order =
        count_trace_elliptic_quotient("equal_j_twist", equal_j_twist_b, twist_order);
    assert_eq!(twist_order, twist_trace_order);
    assert_eq!(pow_field(add_field(equal_j_twist_b, 2), (P - 1) / 2), P - 1,);
    let mut t = 1;
    let mut twist_base = HashSet::new();
    for _ in 0..7_137 {
        let x = add_field(t, pow_field(t, P - 2));
        let value = rhs(x, P - 3, equal_j_twist_b);
        let y = pow_field(value, (P + 1) / 4);
        if mul_field(y, y) == value {
            twist_base.insert(x);
        }
        t = mul_field(t, h);
    }
    assert_eq!(t, 1);
    assert_eq!(twist_base.len(), 1_767);
    // The sole exceptional dihedral orbit (t^3=1) is unlifted, so
    // every lifted orbit contributes three distinct x-columns.
    assert_eq!(twist_base.len() % 3, 0);
    let free_orbit_rank = twist_base.len() / 3;
    assert_eq!(free_orbit_rank, 589);
    println!("model=equal_j_twist b={equal_j_twist_b} order={twist_order} prime_factor={TWIST_PRIME} cofactor=204120 trace_base_columns={} free_orbit_rank={free_orbit_rank} point={twist_point:?}", twist_base.len());
}

#[cfg(test)]
mod dihedral_split_tests {
    use super::{add, add_field, mul_field, pow_field, sub_field, Point, P};

    fn count_points(p: u64, rhs: impl Fn(u64) -> u64) -> u64 {
        let mut square_counts = vec![0_u64; p as usize];
        for y in 0..p {
            square_counts[((y * y) % p) as usize] += 1;
        }
        1 + (0..p).map(|x| square_counts[rhs(x) as usize]).sum::<u64>()
    }

    #[test]
    fn genus_three_dihedral_point_count_identity() {
        // Two E_b factors and one trace quotient give
        // #C = 2#E_b + #E_trace - 2(p+1).
        for (p, expected_e, expected_trace, expected_c) in
            [(103, 118, 100, 128), (127, 133, 112, 122)]
        {
            let b = 40;
            let e = count_points(p, |x| (x.pow(3) + p * 3 - 3 * x + b) % p);
            let trace = count_points(p, |u| (u.pow(3) + b * u.pow(2) + u) % p);
            let c = count_points(p, |t| (t.pow(7) + b * t.pow(4) + t) % p);
            assert_eq!((e, trace, c), (expected_e, expected_trace, expected_c));
            assert_eq!(c, 2 * e + trace - 2 * (p + 1));
        }
    }

    #[test]
    fn inversion_fibers_have_constant_trace_sum() {
        // On T_b, R(t)=(t^3,t^3*y) and
        // R(1/t)=(t^-3,t^-3*y) sum to the fixed point (0,0).
        for b in [40, 1_913_898_821_915_072_477] {
            let b_over_3 = mul_field(b, pow_field(3, P - 2));
            let a = sub_field(1, mul_field(mul_field(b, b), pow_field(3, P - 2)));
            let short_b = sub_field(
                mul_field(2, mul_field(pow_field(b, 3), pow_field(27, P - 2))),
                b_over_3,
            );
            let mut checked = 0;
            for t in 2..100 {
                let u = pow_field(t, 3);
                let inv_u = pow_field(u, P - 2);
                let square = add_field(add_field(u, inv_u), b);
                let y = pow_field(square, (P + 1) / 4);
                if mul_field(y, y) != square {
                    continue;
                }
                let w = mul_field(u, y);
                let inverse_w = mul_field(w, pow_field(u, P - 3));
                let left = Point::Affine(add_field(u, b_over_3), w);
                let right = Point::Affine(add_field(inv_u, b_over_3), inverse_w);
                assert_eq!(add(left, right, a, short_b), Point::Affine(b_over_3, 0));
                checked += 1;
            }
            assert!(checked >= 20);
        }
    }
}
