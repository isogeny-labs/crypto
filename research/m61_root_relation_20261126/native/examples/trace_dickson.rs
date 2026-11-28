//! Dickson trace-root x-base on selected E: y²=x³−3x+40 over M61.
use ecdlp_pieces::{Point, P, R, ROOT_GENERATOR, ROOT_ORDER, SELECTED};
use std::collections::{HashMap, HashSet};

fn signed_three_rows(columns: &[Point], orbit_ids: &[usize]) -> (usize, usize) {
    let mut point_index = HashMap::new();
    for (index, &point) in columns.iter().enumerate() {
        assert!(point_index.insert(point, index).is_none());
        assert!(point_index.insert(SELECTED.neg(point), index).is_none());
    }
    let mut total = 0;
    let mut cross_orbit = 0;
    for i in 0..columns.len() {
        for j in i + 1..columns.len() {
            for second in [columns[j], SELECTED.neg(columns[j])] {
                let pair = SELECTED.add(columns[i], second).unwrap();
                let third = SELECTED.neg(pair);
                if let Some(&k) = point_index.get(&third) {
                    if k > j {
                        total += 1;
                        if orbit_ids[i] != orbit_ids[j] || orbit_ids[j] != orbit_ids[k] {
                            cross_orbit += 1;
                        }
                    }
                }
            }
        }
    }
    (total, cross_orbit)
}

fn pair_sum_collisions(columns: &[Point]) -> (usize, usize) {
    // Fix the first sign. The sum's x-coordinate identifies both global
    // signs, so +/+ and +/- cover every signed distinct-index pair.
    let mut sums = Vec::with_capacity(columns.len() * (columns.len() - 1));
    for i in 0..columns.len() {
        for j in i + 1..columns.len() {
            for second in [columns[j], SELECTED.neg(columns[j])] {
                let point = SELECTED.add(columns[i], second).unwrap();
                let Point::Affine { x, .. } = point else {
                    panic!("distinct columns have opposite points");
                };
                sums.push((x, i as u32, j as u32));
            }
        }
    }
    sums.sort_unstable_by_key(|entry| entry.0);
    let mut collisions = 0;
    let mut disjoint = 0;
    for pair in sums.windows(2) {
        if pair[0].0 == pair[1].0 {
            collisions += 1;
            let (_, i, j) = pair[0];
            let (_, k, l) = pair[1];
            if i != k && i != l && j != k && j != l {
                disjoint += 1;
            }
        }
    }
    (collisions, disjoint)
}

fn add(a: u64, b: u64) -> u64 {
    let sum = a + b;
    if sum >= P {
        sum - P
    } else {
        sum
    }
}
fn mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}
fn power(mut a: u64, mut exponent: u64) -> u64 {
    let mut result = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = mul(result, a);
        }
        a = mul(a, a);
        exponent >>= 1;
    }
    result
}
fn rhs(x: u64) -> u64 {
    let cube = mul(mul(x, x), x);
    add(
        if cube >= mul(3, x) {
            cube - mul(3, x)
        } else {
            P - (mul(3, x) - cube)
        },
        40,
    )
}
fn lift(x: u64) -> Option<Point> {
    let value = rhs(x);
    let y = power(value, (P + 1) / 4);
    (mul(y, y) == value).then_some(Point::Affine { x, y: y.min(P - y) })
}
fn case(m: usize, generator: u64) {
    assert_eq!(m % 3, 0);
    let d = m / 3;
    let mut powers = Vec::with_capacity(m);
    let mut distinct_powers = HashSet::new();
    let mut t = 1;
    for _ in 0..m {
        assert!(distinct_powers.insert(t));
        powers.push(t);
        t = mul(t, generator);
    }
    assert_eq!(t, 1);
    let trace = (0..m)
        .map(|i| add(powers[i], powers[(m - i) % m]))
        .collect::<Vec<_>>();
    for i in 0..m {
        assert_eq!(trace[i], trace[(m - i) % m]);
        let x = trace[i];
        assert_eq!(
            rhs(x),
            add(add(powers[(3 * i) % m], powers[(m - 3 * i % m) % m]), 40)
        );
    }

    let mut seen_parameters = vec![false; m];
    let mut seen_x = HashSet::new();
    let mut columns = Vec::new();
    let mut orbit_ids = Vec::new();
    let mut orbit_sizes = Vec::new();
    let mut valid_orbits = 0;
    let mut valid_columns = 0;
    let mut regular_orbits = 0;
    let mut exceptional_orbits = 0;
    let mut first_row = None;
    let mut exceptional_rows = Vec::new();
    for i in 0..m {
        if seen_parameters[i] {
            continue;
        }
        let six = [
            i,
            (i + d) % m,
            (i + 2 * d) % m,
            (m - i) % m,
            (m - i + d) % m,
            (m - i + 2 * d) % m,
        ];
        for &index in &six {
            seen_parameters[index] = true;
        }
        let orbit_x = [trace[i], trace[(i + d) % m], trace[(i + 2 * d) % m]];
        assert!(orbit_x.iter().all(|&x| rhs(x) == rhs(orbit_x[0])));
        let unique = orbit_x.iter().copied().collect::<HashSet<_>>();
        assert!(unique.len() == 2 || unique.len() == 3);
        if let Some(Point::Affine { y, .. }) = lift(orbit_x[0]) {
            let points = orbit_x.map(|x| Point::Affine { x, y });
            let sum = points.into_iter().fold(Point::Infinity, |acc, point| {
                SELECTED.add(acc, point).unwrap()
            });
            assert_eq!(sum, Point::Infinity);
            for &x in &unique {
                assert!(seen_x.insert(x));
                columns.push(Point::Affine { x, y });
                orbit_ids.push(valid_orbits);
            }
            valid_columns += unique.len();
            valid_orbits += 1;
            orbit_sizes.push(unique.len());
            if unique.len() == 3 {
                regular_orbits += 1;
            } else {
                exceptional_orbits += 1;
                exceptional_rows.push(orbit_x);
            }
            first_row.get_or_insert((orbit_x, y));
        }
    }
    assert!(seen_parameters.iter().all(|&seen| seen));
    assert_eq!(seen_x.len(), valid_columns);
    println!("field={P} curve_a=-3 curve_b=40 torus_order={m} generator={generator} valid_columns={valid_columns} valid_orbits={valid_orbits} regular_three_column_rows={regular_orbits} exceptional_two_column_rows={exceptional_orbits} exact_row_rank={valid_orbits} nullity={}", valid_columns-valid_orbits);
    println!(
        "additional_rows_for_rank_B_minus_1={}",
        valid_columns - 1 - valid_orbits
    );
    println!("first_verified_relation={first_row:?} exceptional_rows={exceptional_rows:?}");

    let full_generator = Point::Affine {
        x: 0,
        y: 313_662_643_971_843_112,
    };
    let torsion_generator = SELECTED.mul(R, full_generator).unwrap();
    let torsion =
        std::array::from_fn::<_, 7, _>(|j| SELECTED.mul(j as u64, torsion_generator).unwrap());
    let mut classes = [0_u64; 7];
    let mut signed = [[0_u128; 7]; 8];
    signed[0][0] = 1;
    for (i, point) in columns.iter().enumerate() {
        let component = SELECTED.mul(R, *point).unwrap();
        let class = torsion.iter().position(|&t| t == component).unwrap();
        classes[class] += 1;
        for k in (1..=usize::min(i + 1, 7)).rev() {
            for h in 0..7 {
                signed[k][h] += signed[k - 1][(h + 7 - class) % 7] + signed[k - 1][(h + class) % 7];
            }
        }
    }
    for k in [6, 7] {
        let total: u128 = signed[k].iter().sum();
        let choose = (0..k).fold(1_u128, |acc, j| {
            acc * (valid_columns - j) as u128 / (j + 1) as u128
        });
        assert_eq!(total, (1_u128 << k) * choose);
    }
    let six_total: u128 = signed[6].iter().sum();
    let seven_total: u128 = signed[7].iter().sum();
    println!("cofactor_classes={classes:?} signed_six_total={six_total} signed_six_class_zero={} signed_seven_total={seven_total} signed_seven_class_zero={}", signed[6][0], signed[7][0]);
    println!("uniform_projected_six_target_expectation={:.6} uniform_projected_seven_zero_rows_sign_folded={:.6} residual_rank_after_free_rows={}", six_total as f64 / R as f64, seven_total as f64 / (2 * R) as f64, valid_columns - 1 - valid_orbits);
    let mut distinct_orbit_subsets = [0_u128; 8];
    distinct_orbit_subsets[0] = 1;
    for (i, &size) in orbit_sizes.iter().enumerate() {
        for k in (1..=usize::min(i + 1, 7)).rev() {
            distinct_orbit_subsets[k] += size as u128 * distinct_orbit_subsets[k - 1];
        }
    }
    let six_distinct = (1_u128 << 6) * distinct_orbit_subsets[6];
    let seven_distinct = (1_u128 << 7) * distinct_orbit_subsets[7];
    assert!(six_distinct <= six_total && seven_distinct <= seven_total);
    println!("distinct_orbit_signed_six={six_distinct} distinct_orbit_signed_seven={seven_distinct} seven_fraction={:.6} uniform_projected_distinct_orbit_seven_rows={:.6}", seven_distinct as f64 / seven_total as f64, seven_distinct as f64 / (2 * R) as f64);
    if m == 7_137 || m == 8_190 {
        let full = signed_three_rows(&columns, &orbit_ids);
        let projected = columns
            .iter()
            .map(|&point| SELECTED.mul(7, point).unwrap())
            .collect::<Vec<_>>();
        let prime = signed_three_rows(&projected, &orbit_ids);
        assert_eq!(full, (regular_orbits, 0));
        assert_eq!(prime, full);
        println!(
            "signed_distinct_three_rows_full={full:?} projected={prime:?} (total,cross_orbit)"
        );
        let full_pairs = pair_sum_collisions(&columns);
        let projected_pairs = pair_sum_collisions(&projected);
        assert_eq!(full_pairs, (0, 0));
        assert_eq!(projected_pairs, (0, 0));
        println!("signed_pair_sum_x_collisions_full={full_pairs:?} projected={projected_pairs:?} (adjacent_collisions,disjoint_pairs)");
    }
}

fn main() {
    for (order, generator) in [
        (ROOT_ORDER as usize, ROOT_GENERATOR),
        (6_930, 1_364_568_923_720_204_790),
        (6_510, 1_592_996_259_130_628_683),
        (7_137, 556_975_957_790_817_848),
        (8_190, 710_490_119_262_523_924),
        (17_325, 1_500_985_792_431_335_074),
    ] {
        case(order, generator);
    }
}
