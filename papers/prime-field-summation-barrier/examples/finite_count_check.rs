//! Exact finite sanity checks for the manuscript's counting layer.
//!
//! This is deliberately not a performance benchmark and does not test an
//! asymptotic lower bound.  It exhaustively enumerates small cyclic-group
//! instances and checks the identities used before any algebraic-solving
//! model is introduced.

use std::collections::BTreeSet;

fn binomial(n: u128, k: u128) -> u128 {
    let k = k.min(n - k);
    (0..k).fold(1, |acc, i| acc * (n - i) / (i + 1))
}

fn enumerate_multisets(
    base: &[usize],
    modulus: usize,
    arity: usize,
    start: usize,
    depth: usize,
    sum: usize,
    sums: &mut BTreeSet<usize>,
    tuples: &mut u128,
) {
    if depth == arity {
        *tuples += 1;
        sums.insert(sum % modulus);
        return;
    }

    for index in start..base.len() {
        enumerate_multisets(
            base,
            modulus,
            arity,
            index,
            depth + 1,
            (sum + base[index]) % modulus,
            sums,
            tuples,
        );
    }
}

fn check_instance(modulus: usize, base_size: usize, arity: usize) {
    let base: Vec<_> = (1..=base_size).collect();
    let mut sums = BTreeSet::new();
    let mut tuples = 0;
    enumerate_multisets(
        &base,
        modulus,
        arity,
        0,
        0,
        0,
        &mut sums,
        &mut tuples,
    );

    let expected = binomial((base_size + arity - 1) as u128, arity as u128);
    assert_eq!(tuples, expected);
    assert!(sums.len() as u128 <= tuples);
    assert!(sums.len() <= modulus);

    // Counting with multiplicity is the finite kernel of the static-provenance
    // direct-sum theorem: summing fibre sizes over every target recovers the
    // number of stored candidate leaves exactly.
    let mut fibre_sizes = vec![0_u128; modulus];
    fn count_fibres(
        base: &[usize],
        modulus: usize,
        arity: usize,
        start: usize,
        depth: usize,
        sum: usize,
        fibres: &mut [u128],
    ) {
        if depth == arity {
            fibres[sum % modulus] += 1;
            return;
        }
        for index in start..base.len() {
            count_fibres(
                base,
                modulus,
                arity,
                index,
                depth + 1,
                (sum + base[index]) % modulus,
                fibres,
            );
        }
    }
    count_fibres(&base, modulus, arity, 0, 0, 0, &mut fibre_sizes);
    assert_eq!(fibre_sizes.iter().sum::<u128>(), tuples);

    // Exhausting every target makes the uniform-target identity exact:
    // the number of targets with a decomposition is precisely |im sigma_m|.
    let accepted_targets = (0..modulus).filter(|target| sums.contains(target)).count();
    assert_eq!(accepted_targets, sums.len());
}

#[test]
fn exact_uniform_target_and_multiset_bounds() {
    for modulus in [17, 31, 101] {
        for base_size in 2..=7 {
            for arity in 2..=5 {
                check_instance(modulus, base_size, arity);
            }
        }
    }
}

#[test]
fn audited_fixed_arity_exponents_exceed_one_half() {
    for arity in 2_u128..=32 {
        let numerator = arity;
        let denominator = 2 * arity - 1;
        assert!(2 * numerator > denominator);
    }

    assert_eq!((3_u128, 2 * 3 - 1), (3, 5));
    assert_eq!((4_u128, 2 * 4 - 1), (4, 7));
    assert_eq!((5_u128, 2 * 5 - 1), (5, 9));
}

fn implicit_ds_exponent(m: f64, s: f64, t: f64) -> f64 {
    if s - t >= 1.0 {
        s / (s + m - 1.0 - t)
    } else {
        (1.0 + t) / m
    }
}

#[test]
fn piecewise_implicit_data_structure_balance_matches_grid_search() {
    let cases = [
        (3.0, 1.0, 0.0),
        (3.0, 2.5, 0.5),
        (4.0, 1.0, 1.5),
        (4.0, 3.0, 0.5),
        (5.0, 1.25, 0.75),
        (5.0, 4.0, 1.0),
    ];

    for (m, s, t) in cases {
        assert!(t < m - 1.0);
        let predicted = implicit_ds_exponent(m, s, t);
        let mut observed = f64::INFINITY;
        for step in 1..=100_000 {
            let alpha = (step as f64) / (100_000.0 * m);
            let exponent = (s * alpha).max(1.0 - (m - 1.0 - t) * alpha);
            observed = observed.min(exponent);
        }
        assert!((predicted - observed).abs() < 1e-5);
    }
}

#[test]
fn audited_indexing_family_small_arity_fractions() {
    // (m, delta numerator, delta denominator, expected numerator, denominator)
    let cases = [
        (3_i128, 0_i128, 1_i128, 7_i128, 11_i128),
        (3, 1, 2, 2, 3),
        (3, 1, 1, 5, 7),
        (4, 0, 1, 3, 5),
        (4, 1, 2, 8, 13),
        (4, 1, 1, 7, 11),
        (5, 0, 1, 11, 19),
        (5, 1, 2, 10, 17),
        (5, 1, 1, 3, 5),
    ];

    for (m, delta_num, delta_den, expected_num, expected_den) in cases {
        let numerator = (2 * m + 1) * delta_den - 2 * delta_num;
        let denominator = (4 * m - 1) * delta_den - 4 * delta_num;
        assert_eq!(numerator * expected_den, expected_num * denominator);
        assert!(2 * numerator > denominator);
    }
}

#[test]
fn prime_cyclic_groups_have_no_smaller_nontrivial_homomorphic_bucket() {
    for prime_order in [5_u128, 7, 11, 13, 17, 19] {
        for bucket_order in 1..prime_order {
            // A homomorphism Z/nZ -> Z/hZ is determined by the image of 1,
            // which must satisfy n*a = 0 modulo h.  Coprimality forces a=0.
            let valid_images = (0..bucket_order)
                .filter(|image| (prime_order * image) % bucket_order == 0)
                .count();
            assert_eq!(valid_images, 1);
        }
    }
}

fn has_exact_composition_rule(labels: &[usize], label_count: usize) -> bool {
    let group_order = labels.len();
    let mut table = vec![None; label_count * label_count];
    for left in 0..group_order {
        for right in 0..group_order {
            let slot = labels[left] * label_count + labels[right];
            let sum_label = labels[(left + right) % group_order];
            match table[slot] {
                Some(previous) if previous != sum_label => return false,
                Some(_) => {}
                None => table[slot] = Some(sum_label),
            }
        }
    }
    true
}

#[test]
fn prime_cyclic_groups_have_no_smaller_exact_compositional_labels() {
    for prime_order in [3_usize, 5, 7] {
        for label_count in 1..prime_order {
            let number_of_maps = label_count.pow(prime_order as u32);
            for code in 0..number_of_maps {
                let mut remainder = code;
                let mut labels = vec![0; prime_order];
                for label in &mut labels {
                    *label = remainder % label_count;
                    remainder /= label_count;
                }
                let constant = labels.iter().all(|label| *label == labels[0]);
                assert!(constant || !has_exact_composition_rule(&labels, label_count));
            }
        }
    }
}

fn decode_labels(mut code: usize, group_order: usize, label_count: usize) -> Vec<usize> {
    let mut labels = vec![0; group_order];
    for label in &mut labels {
        *label = code % label_count;
        code /= label_count;
    }
    labels
}

fn exact_addition_predicate_exists(
    left_labels: &[usize],
    right_labels: &[usize],
    target_labels: &[usize],
    label_count: usize,
) -> bool {
    let group_order = left_labels.len();
    let mut predicate = vec![None; label_count.pow(3)];
    for left in 0..group_order {
        for right in 0..group_order {
            for target in 0..group_order {
                let slot = (left_labels[left] * label_count + right_labels[right]) * label_count
                    + target_labels[target];
                let should_accept = (left + right) % group_order == target;
                match predicate[slot] {
                    Some(previous) if previous != should_accept => return false,
                    Some(_) => {}
                    None => predicate[slot] = Some(should_accept),
                }
            }
        }
    }
    true
}

#[test]
fn smaller_separate_labels_cannot_decide_addition_exactly() {
    for (prime_order, label_count) in [(3_usize, 1_usize), (3, 2), (5, 2)] {
        let number_of_maps = label_count.pow(prime_order as u32);
        for left_code in 0..number_of_maps {
            let left = decode_labels(left_code, prime_order, label_count);
            for right_code in 0..number_of_maps {
                let right = decode_labels(right_code, prime_order, label_count);
                for target_code in 0..number_of_maps {
                    let target = decode_labels(target_code, prime_order, label_count);
                    assert!(!exact_addition_predicate_exists(
                        &left,
                        &right,
                        &target,
                        label_count,
                    ));
                }
            }
        }
    }
}

fn one_sided_false_count(
    group_order: usize,
    left: &[usize],
    right: &[usize],
    left_labels: &[usize],
    right_labels: &[usize],
    target_labels: &[usize],
) -> usize {
    let mut accepted_label_triples = BTreeSet::new();
    for &a in left {
        for &b in right {
            accepted_label_triples.insert((
                left_labels[a],
                right_labels[b],
                target_labels[(a + b) % group_order],
            ));
        }
    }

    let mut false_count = 0;
    for &a in left {
        for &b in right {
            for target in 0..group_order {
                let labels = (left_labels[a], right_labels[b], target_labels[target]);
                if target != (a + b) % group_order
                    && accepted_label_triples.contains(&labels)
                {
                    false_count += 1;
                }
            }
        }
    }
    false_count
}

fn collision_lower_bound(set: &[usize], labels: &[usize], other_size: usize) -> usize {
    let label_count = set
        .iter()
        .map(|&element| labels[element])
        .max()
        .map_or(0, |maximum| maximum + 1);
    let mut fibre_sizes = vec![0_usize; label_count];
    for &element in set {
        fibre_sizes[labels[element]] += 1;
    }
    other_size
        * fibre_sizes
            .iter()
            .map(|size| size * size.saturating_sub(1))
            .sum::<usize>()
}

#[test]
fn one_sided_separate_labels_force_false_candidates() {
    let fixtures = [
        (7_usize, 5_usize, 4_usize, 2_usize, 3_usize, 4_usize),
        (11, 7, 6, 3, 2, 5),
        (13, 9, 8, 4, 3, 6),
    ];

    for (group_order, left_size, right_size, left_buckets, right_buckets, target_buckets) in
        fixtures
    {
        let left: Vec<_> = (0..left_size).collect();
        let right: Vec<_> = (0..right_size).collect();
        let left_labels: Vec<_> = (0..group_order)
            .map(|element| element % left_buckets)
            .collect();
        let right_labels: Vec<_> = (0..group_order)
            .map(|element| element % right_buckets)
            .collect();
        let target_labels: Vec<_> = (0..group_order)
            .map(|element| element % target_buckets)
            .collect();

        let false_count = one_sided_false_count(
            group_order,
            &left,
            &right,
            &left_labels,
            &right_labels,
            &target_labels,
        );
        let left_bound = collision_lower_bound(&left, &left_labels, right.len());
        let right_bound = collision_lower_bound(&right, &right_labels, left.len());
        let sumset: Vec<_> = (0..group_order)
            .filter(|target| {
                left.iter().any(|&a| {
                    right
                        .iter()
                        .any(|&b| (a + b) % group_order == *target)
                })
            })
            .collect();
        let target_bound = collision_lower_bound(&sumset, &target_labels, 1);
        assert!(false_count >= left_bound.max(right_bound).max(target_bound));
    }
}

#[test]
fn target_shift_scan_exponents_remain_above_one_half() {
    let expected = [(3_i128, 2_i128, 3_i128), (4, 3, 5), (5, 4, 7)];
    for (arity, expected_numerator, expected_denominator) in expected {
        let numerator = arity - 1;
        let denominator = 2 * arity - 3;
        assert_eq!(
            numerator * expected_denominator,
            expected_numerator * denominator
        );
        assert!(2 * numerator > denominator);

        // Explicit batching of all m-tuples is slightly better, though it too
        // remains above one-half: m/(2m-1) < (m-1)/(2m-3).
        assert!(arity * denominator < numerator * (2 * arity - 1));
    }
}

fn maximum_fibre_size(map: &[usize], range_size: usize) -> usize {
    let mut fibres = vec![0_usize; range_size];
    for &value in map {
        fibres[value] += 1;
    }
    fibres.into_iter().max().unwrap_or(0)
}

#[test]
fn finite_selector_catalogues_obey_fibre_mass_bound() {
    for group_order in [11_usize, 17] {
        for factor_size in 2..=5 {
            let factor: BTreeSet<_> = (0..factor_size).collect();
            let mut total_successes = 0_usize;
            let mut total_degree_mass = 0_usize;

            // Each coefficient tuple sums to one, so the branch outputs sum
            // to the target. Constants are included as coefficient zero.
            for coefficients in [
                [1, 0, 0],
                [2, group_order - 1, 0],
                [3, 4, group_order - 6],
            ] {
                let maps: Vec<Vec<usize>> = coefficients
                    .iter()
                    .map(|&coefficient| {
                        (0..group_order)
                            .map(|target| coefficient * target % group_order)
                            .collect()
                    })
                    .collect();

                let success_count = (0..group_order)
                    .filter(|&target| {
                        maps.iter().all(|map| factor.contains(&map[target]))
                            && maps.iter().map(|map| map[target]).sum::<usize>() % group_order
                                == target
                    })
                    .count();
                let degree_mass = 1
                    + maps
                        .iter()
                        .filter(|map| !map.iter().all(|value| *value == map[0]))
                        .map(|map| maximum_fibre_size(map, group_order))
                        .sum::<usize>();

                assert!(success_count <= factor_size * degree_mass);
                total_successes += success_count;
                total_degree_mass += degree_mass;
            }

            assert!(total_successes <= factor_size * total_degree_mass);
        }
    }
}

#[test]
fn compact_global_map_branches_are_constant_or_injective() {
    for group_order in [11_usize, 17] {
        for factor_size in 3..=6 {
            let factor: BTreeSet<_> = (0..factor_size).collect();
            let mut total_successes = 0_usize;
            let mut branch_count = 0_usize;

            for coefficient in [0_usize, 1, 2, 5] {
                let coefficients = [
                    coefficient % group_order,
                    (group_order + 1 - coefficient % group_order) % group_order,
                    0,
                ];
                let constants = [2_usize, group_order - 2, 0];
                let maps: Vec<Vec<usize>> = coefficients
                    .iter()
                    .zip(constants)
                    .map(|(&scalar, constant)| {
                        (0..group_order)
                            .map(|target| (scalar * target + constant) % group_order)
                            .collect()
                    })
                    .collect();

                for map in &maps {
                    let image_size = map.iter().copied().collect::<BTreeSet<_>>().len();
                    assert!(image_size == 1 || image_size == group_order);
                }

                let success_count = (0..group_order)
                    .filter(|&target| {
                        maps.iter().all(|map| factor.contains(&map[target]))
                            && maps.iter().map(|map| map[target]).sum::<usize>() % group_order
                                == target
                    })
                    .count();
                assert!(success_count <= factor_size);
                total_successes += success_count;
                branch_count += 1;
            }

            // A branch with only constant leaf maps can satisfy the target
            // equation at one group element at most.
            let constant_outputs = [0_usize, 1, 2];
            let constant_successes = (0..group_order)
                .filter(|target| {
                    constant_outputs.iter().all(|value| factor.contains(value))
                        && constant_outputs.iter().sum::<usize>() % group_order == *target
                })
                .count();
            assert!(constant_successes <= 1);
            total_successes += constant_successes;
            branch_count += 1;

            assert!(total_successes <= factor_size * branch_count);
        }
    }
}

fn main() {}
