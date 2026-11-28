//! Six signed lifts over one root parameter on selected E/M61.
//! A separated fiber certifies the sum-x field has the full
//! simultaneous-sign quotient degree; it is not a relation locator.
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
fn signed_sum(points: &[Point; 6], mask: usize) -> Point {
    let mut sum = Point::Infinity;
    for (i, &point) in points.iter().enumerate() {
        let signed = if i != 0 && (mask >> (i - 1)) & 1 != 0 {
            SELECTED.neg(point)
        } else {
            point
        };
        sum = SELECTED.add(sum, signed).unwrap();
    }
    sum
}
fn sum_x_classes(points: &[Point; 6]) -> Option<usize> {
    let mut xs = HashSet::new();
    for mask in 0..32 {
        let sum = signed_sum(points, mask);
        let Point::Affine { x, .. } = sum else {
            return None;
        };
        xs.insert(x);
    }
    Some(xs.len())
}
fn choose(n: u128, k: u128) -> u128 {
    (0..k).fold(1, |acc, i| acc * (n - i) / (i + 1))
}
fn main() {
    let ratios = std::array::from_fn::<_, 6, _>(|i| pow(ROOT_GENERATOR, i as u64));
    assert_eq!(ratios.iter().copied().collect::<HashSet<_>>().len(), 6);
    let ratio_set = ratios.into_iter().collect::<HashSet<_>>();
    let mut multiplier = ROOT_GENERATOR;
    for _ in 1..ROOT_ORDER {
        assert!(ratios
            .iter()
            .any(|&ratio| !ratio_set.contains(&mul(multiplier, ratio))));
        multiplier = mul(multiplier, ROOT_GENERATOR);
    }
    assert_eq!(multiplier, 1);
    let mut x = 1;
    let mut all_six_lift = 0;
    let mut fully_separated = 0;
    let mut first_witness = None;
    let mut max_distinct = 0;
    for _ in 0..ROOT_ORDER {
        let points = ratios.map(|ratio| lift(mul(ratio, x)));
        if let [Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)] = points {
            all_six_lift += 1;
            let points = [a, b, c, d, e, f];
            if let Some(distinct) = sum_x_classes(&points) {
                max_distinct = max_distinct.max(distinct);
                if distinct == 32 {
                    fully_separated += 1;
                    first_witness.get_or_insert(x);
                }
            }
        }
        x = mul(x, ROOT_GENERATOR);
    }
    assert_eq!(x, 1);
    assert!(first_witness.is_some());
    assert_eq!(max_distinct, 32);
    println!("ratio_count=6 root_order={ROOT_ORDER} multiplicative_stabilizer=1 all_six_lift={all_six_lift} fully_separated={fully_separated} max_distinct_sum_x={max_distinct} first_witness_x={:?} simultaneous_sign_quotient_degree=32 genus=113", first_witness.unwrap());

    // A planted fixed-ratio descent: target construction is outside the
    // online scan. This says nothing about searching all ratio tuples.
    let planted_x = first_witness.unwrap();
    let planted_points = ratios.map(|ratio| lift(mul(ratio, planted_x)).unwrap());
    let planted_mask = 17;
    let planted_target = SELECTED
        .mul(7, signed_sum(&planted_points, planted_mask))
        .unwrap();
    assert_ne!(planted_target, Point::Infinity);
    let generator = SELECTED
        .mul(
            7,
            Point::Affine {
                x: 0,
                y: 313_662_643_971_843_112,
            },
        )
        .unwrap();
    let control_target = SELECTED.mul(1_777, generator).unwrap();
    assert_ne!(planted_target, control_target);
    let mut online_x = 1;
    let mut online_lifted = 0;
    let mut online_signed_sums = 0;
    let mut planted_hits = Vec::new();
    let mut control_hits = 0;
    let mut projected_sum_xs = HashSet::new();
    for _ in 0..ROOT_ORDER {
        let points = ratios.map(|ratio| lift(mul(ratio, online_x)));
        if let [Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)] = points {
            online_lifted += 1;
            let points = [a, b, c, d, e, f];
            for mask in 0..32 {
                let image = SELECTED.mul(7, signed_sum(&points, mask)).unwrap();
                let Point::Affine { x: image_x, .. } = image else {
                    panic!("projected six-sum unexpectedly vanishes");
                };
                projected_sum_xs.insert(image_x);
                online_signed_sums += 1;
                if image == planted_target {
                    planted_hits.push((online_x, mask));
                }
                if image == control_target {
                    control_hits += 1;
                }
            }
        }
        online_x = mul(online_x, ROOT_GENERATOR);
    }
    assert_eq!(online_x, 1);
    assert_eq!(online_lifted, all_six_lift);
    assert_eq!(online_signed_sums, 32 * online_lifted);
    assert_eq!(projected_sum_xs.len(), online_signed_sums);
    assert_eq!(planted_hits, vec![(planted_x, planted_mask)]);
    assert_eq!(control_hits, 0);
    println!("fixed_ratio_descent planted_target={planted_target:?} online_parameters={ROOT_ORDER} online_lift_tests={} online_lifted={online_lifted} online_signed_sums={online_signed_sums} online_direct_add_calls={} online_cofactor_muls={online_signed_sums} distinct_projected_sum_x={} signed_target_coverage={} planted_hits={planted_hits:?} unrelated_control_hits={control_hits}", 6 * ROOT_ORDER, 6 * online_signed_sums, projected_sum_xs.len(), 2 * projected_sum_xs.len());

    // Compare this tuple's entire six-sum image with the full projected
    // root base. An x intersection is necessary for a signed seven-row.
    let projected_base_xs: HashSet<_> = selected_root_factor_base()
        .into_iter()
        .map(|point| match SELECTED.mul(7, point).unwrap() {
            Point::Affine { x, .. } => x,
            Point::Infinity => panic!("projected root-base column vanishes"),
        })
        .collect();
    assert_eq!(projected_base_xs.len(), 2447);
    let seven_row_x_hits = projected_sum_xs.intersection(&projected_base_xs).count();
    assert_eq!(seven_row_x_hits, 0);
    println!("fixed_ratio_base_intersection projected_base_columns={} six_sum_x_classes={} seven_row_x_hits={seven_row_x_hits}", projected_base_xs.len(), projected_sum_xs.len());

    // The six roots of unity form a ratio set stabilized by rotation.
    // Rotating the parameter permutes the six inputs, so the resulting
    // signed-sum x image should compress by at least a factor of six.
    let sixth_root = pow(ROOT_GENERATOR, ROOT_ORDER / 6);
    let symmetric_ratios = std::array::from_fn::<_, 6, _>(|i| pow(sixth_root, i as u64));
    assert_eq!(
        symmetric_ratios
            .iter()
            .copied()
            .collect::<HashSet<_>>()
            .len(),
        6
    );
    let mut parameter = 1;
    let mut symmetric_lifted = 0;
    let mut symmetric_signed = 0;
    let mut symmetric_xs = HashSet::new();
    for _ in 0..ROOT_ORDER {
        let points = symmetric_ratios.map(|ratio| lift(mul(ratio, parameter)));
        if let [Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)] = points {
            symmetric_lifted += 1;
            let points = [a, b, c, d, e, f];
            for mask in 0..32 {
                let image = SELECTED.mul(7, signed_sum(&points, mask)).unwrap();
                let Point::Affine { x, .. } = image else {
                    panic!("symmetric six-sum unexpectedly vanishes");
                };
                symmetric_xs.insert(x);
                symmetric_signed += 1;
            }
        }
        parameter = mul(parameter, ROOT_GENERATOR);
    }
    assert_eq!(parameter, 1);
    assert_eq!(symmetric_lifted % 6, 0);
    assert_eq!(symmetric_signed, 32 * symmetric_lifted);
    assert!(symmetric_xs.len() <= symmetric_signed / 6);
    let symmetric_seven_hits = symmetric_xs.intersection(&projected_base_xs).count();
    println!("symmetric_six_ratios lifted_parameters={symmetric_lifted} signed_sums={symmetric_signed} distinct_projected_sum_x={} signed_target_coverage={} seven_row_x_hits={symmetric_seven_hits}", symmetric_xs.len(), 2 * symmetric_xs.len());

    let mut representative = 1;
    let mut quotient_lifted = 0;
    let mut quotient_xs = HashSet::new();
    for _ in 0..ROOT_ORDER / 6 {
        let points = symmetric_ratios.map(|ratio| lift(mul(ratio, representative)));
        if let [Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)] = points {
            quotient_lifted += 1;
            let points = [a, b, c, d, e, f];
            for mask in 0..32 {
                let image = SELECTED.mul(7, signed_sum(&points, mask)).unwrap();
                let Point::Affine { x, .. } = image else {
                    panic!("quotient six-sum unexpectedly vanishes");
                };
                assert!(quotient_xs.insert(x));
            }
        }
        representative = mul(representative, ROOT_GENERATOR);
    }
    assert_eq!(representative, sixth_root);
    assert_eq!(quotient_lifted * 6, symmetric_lifted);
    assert_eq!(quotient_xs, symmetric_xs);
    println!("symmetric_parameter_quotient representatives={} lift_tests={} lifted_representatives={quotient_lifted} signed_sum_evaluations={} full_image_recovered=true", ROOT_ORDER / 6, ROOT_ORDER, quotient_xs.len());

    // Any nontrivial stabilizer of a six-element ratio set has order
    // 2, 3, or 6. Count every liftable root orbit for each subgroup;
    // choosing 6/h such orbits gives every h-stable six-x subset.
    for h in [2_u64, 3, 6] {
        let orbit_step = pow(ROOT_GENERATOR, ROOT_ORDER / h);
        let mut root = 1;
        let mut liftable_orbits = 0_u128;
        for _ in 0..ROOT_ORDER / h {
            let mut member = root;
            let mut all_lift = true;
            for _ in 0..h {
                all_lift &= lift(member).is_some();
                member = mul(member, orbit_step);
            }
            assert_eq!(member, root);
            liftable_orbits += u128::from(all_lift);
            root = mul(root, ROOT_GENERATOR);
        }
        assert_eq!(root, orbit_step);
        let stable_six_sets = choose(liftable_orbits, (6 / h) as u128);
        let candidate_x_bound = 32 * stable_six_sets;
        let uniform_base_hit_mean = 2.0 * candidate_x_bound as f64 * projected_base_xs.len() as f64
            / 329_406_144_237_565_159_f64;
        println!("stabilizer_census order={h} root_orbits={} liftable_orbits={liftable_orbits} stable_six_sets={stable_six_sets} signed_sum_x_capacity_bound={candidate_x_bound} uniform_base_hit_mean_bound={uniform_base_hit_mean:.6e}", ROOT_ORDER / h);
    }

    // The entire order-three family is small enough for an exact
    // projected seven-row scan: choose two liftable three-point orbits.
    let third_root = pow(ROOT_GENERATOR, ROOT_ORDER / 3);
    let mut root = 1;
    let mut triple_orbits = Vec::new();
    for _ in 0..ROOT_ORDER / 3 {
        let points = [
            root,
            mul(root, third_root),
            mul(root, mul(third_root, third_root)),
        ]
        .map(lift);
        if let [Some(a), Some(b), Some(c)] = points {
            triple_orbits.push([
                SELECTED.mul(7, a).unwrap(),
                SELECTED.mul(7, b).unwrap(),
                SELECTED.mul(7, c).unwrap(),
            ]);
        }
        root = mul(root, ROOT_GENERATOR);
    }
    assert_eq!(triple_orbits.len(), 194);
    let mut order_three_xs = HashSet::new();
    let mut order_three_hits = 0;
    for i in 0..triple_orbits.len() {
        for j in i + 1..triple_orbits.len() {
            let points = [
                triple_orbits[i][0],
                triple_orbits[i][1],
                triple_orbits[i][2],
                triple_orbits[j][0],
                triple_orbits[j][1],
                triple_orbits[j][2],
            ];
            for mask in 0..32 {
                let Point::Affine { x, .. } = signed_sum(&points, mask) else {
                    panic!("order-three projected six-sum vanishes");
                };
                order_three_xs.insert(x);
                order_three_hits += usize::from(projected_base_xs.contains(&x));
            }
        }
    }
    assert_eq!(order_three_xs.len(), 599_072);
    assert_eq!(order_three_hits, 0);
    println!("order_three_full_scan stable_six_sets=18721 signed_sums=599072 distinct_projected_sum_x={} seven_row_x_hits={order_three_hits}", order_three_xs.len());

    // For the order-two family, each six-set is three antipodal root
    // pairs. A+B = -(C+P) can be checked by hashing two-pair sums and
    // probing one-pair-plus-base sums, both quadratic-size tables.
    let mut root = 1;
    let mut antipodal_pairs = Vec::new();
    for _ in 0..ROOT_ORDER / 2 {
        if let (Some(a), Some(b)) = (lift(root), lift(P - root)) {
            let a = SELECTED.mul(7, a).unwrap();
            let b = SELECTED.mul(7, b).unwrap();
            let neg_a = SELECTED.neg(a);
            let neg_b = SELECTED.neg(b);
            antipodal_pairs.push([
                SELECTED.add(a, b).unwrap(),
                SELECTED.add(a, neg_b).unwrap(),
                SELECTED.add(neg_a, b).unwrap(),
                SELECTED.add(neg_a, neg_b).unwrap(),
            ]);
        }
        root = mul(root, ROOT_GENERATOR);
    }
    assert_eq!(antipodal_pairs.len(), 616);
    let projected_base = selected_root_factor_base()
        .into_iter()
        .map(|point| SELECTED.mul(7, point).unwrap())
        .collect::<Vec<_>>();
    let mut pair_pair_xs = HashSet::with_capacity(3_100_000);
    let mut pair_pair_tests = 0_u64;
    for i in 0..antipodal_pairs.len() {
        for j in i + 1..antipodal_pairs.len() {
            for &a in &antipodal_pairs[i] {
                for &b in &antipodal_pairs[j] {
                    if let Point::Affine { x, .. } = SELECTED.add(a, b).unwrap() {
                        pair_pair_xs.insert(x);
                    }
                    pair_pair_tests += 1;
                }
            }
        }
    }
    let mut pair_base_tests = 0_u64;
    let mut pair_base_x_hits = 0_u64;
    for pair in &antipodal_pairs {
        for &c in pair {
            for &base in &projected_base {
                if let Point::Affine { x, .. } = SELECTED.add(c, base).unwrap() {
                    pair_base_x_hits += u64::from(pair_pair_xs.contains(&x));
                }
                pair_base_tests += 1;
            }
        }
    }
    assert_eq!(pair_base_x_hits, 0);
    println!("order_two_four_sum pair_orbits={} pair_pair_tests={pair_pair_tests} distinct_pair_pair_x={} pair_base_tests={pair_base_tests} raw_x_matches={pair_base_x_hits}", antipodal_pairs.len(), pair_pair_xs.len());

    // A fixed nonidentity ratio between two selected x-columns
    // restricts seven-row supports. Count every directed ratio edge.
    let base_xs = selected_root_factor_base()
        .into_iter()
        .map(|point| match point {
            Point::Affine { x, .. } => x,
            Point::Infinity => unreachable!(),
        })
        .collect::<Vec<_>>();
    let base_set = base_xs.iter().copied().collect::<HashSet<_>>();
    let mut ratio = ROOT_GENERATOR;
    let mut ratio_edges_total = 0_u64;
    let mut ratio_edges_min = usize::MAX;
    let mut ratio_edges_max = 0;
    let mut busiest_ratio = 0;
    for _ in 1..ROOT_ORDER {
        let edges = base_xs
            .iter()
            .filter(|&&x| base_set.contains(&mul(ratio, x)))
            .count();
        ratio_edges_total += edges as u64;
        ratio_edges_min = ratio_edges_min.min(edges);
        if edges > ratio_edges_max {
            ratio_edges_max = edges;
            busiest_ratio = ratio;
        }
        ratio = mul(ratio, ROOT_GENERATOR);
    }
    assert_eq!(ratio, 1);
    assert_eq!(
        ratio_edges_total,
        (base_xs.len() * (base_xs.len() - 1)) as u64
    );
    let fixed_ratio_row_mean_bound =
        ratio_edges_max as f64 * choose((base_xs.len() - 2) as u128, 5) as f64 * 64.0
            / 329_406_144_237_565_159_f64;
    println!("fixed_ratio_edge_census ratios={} directed_edges_total={ratio_edges_total} min_edges={ratio_edges_min} max_edges={ratio_edges_max} busiest_ratio={busiest_ratio} uniform_seven_row_mean_bound_at_max={fixed_ratio_row_mean_bound:.6} rank_target={}", ROOT_ORDER - 1, base_xs.len() - 1);
}
