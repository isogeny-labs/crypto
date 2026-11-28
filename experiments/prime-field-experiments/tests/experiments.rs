use prime_field_experiments::maps::{measure, Config, MapSpec};
use prime_field_experiments::{find_roots, PrimeField};
use rand::{rngs::StdRng, Rng, SeedableRng};

#[test]
fn roots_match_exhaustive_field_evaluation() {
    let mut rng = StdRng::seed_from_u64(1423);
    for p in [2_i128, 3, 5, 13, 17, 101, 257] {
        let field = PrimeField::new(p as u64).unwrap();
        for degree in [1, 2, 4, 9, 16, 31, 32, 33, 64] {
            let coefficients: Vec<_> = (0..degree)
                .map(|_| rng.random_range(-p..p))
                .chain([1])
                .collect();
            let expected: Vec<_> = (0..p)
                .filter(|&x| {
                    coefficients
                        .iter()
                        .rev()
                        .fold(0, |f, a| (f * x + a).rem_euclid(p))
                        == 0
                })
                .map(|x| x as u64)
                .collect();

            for fold in [false, true] {
                let report = find_roots(field, &coefficients, fold, 7).unwrap();
                assert_eq!(report.roots, expected, "p={p} degree={degree} fold={fold}");
            }
        }
    }
}

#[test]
fn reflection_lifts_repeated_roots_over_a_full_word_prime() {
    let field = PrimeField::new(18_446_744_073_709_551_557).unwrap();
    // ((X-3)^2) * ((X-3)^2 - 1) * ((X-3)^2 - 1).
    let coefficients = [576, -1248, 1108, -516, 133, -18, 1];
    let run = find_roots(field, &coefficients, true, 0).unwrap();
    assert_eq!(run.roots, [2, 3, 4]);
    assert_eq!(run.reflection_center, Some(3));
    assert_eq!(run.extraction_degree, 3);
    assert!(find_roots(field, &[0, 0], true, 0).is_err());
}

#[test]
fn historical_fixture_matches_in_serial_and_parallel() {
    let config: Config = serde_json::from_str(include_str!("../example_maps.json")).unwrap();
    let serial = measure(&config, 1).unwrap();
    let parallel = measure(&config, 3).unwrap();
    assert_eq!(
        serde_json::to_vec(&serial).unwrap(),
        serde_json::to_vec(&parallel).unwrap()
    );
    assert_eq!(serial.genus, 1);
    assert_eq!(serial.rational_point_count, Some(109));
    assert_eq!(serial.prime_curve_order, Some(true));
    assert_eq!(
        serial
            .maps
            .iter()
            .map(|map| map.map_degree)
            .collect::<Vec<_>>(),
        [2, 3, 4, 8, 2, 2]
    );

    let expected_histograms: &[&[(usize, usize)]] = &[
        &[(0, 47), (1, 1), (2, 54)],
        &[(0, 37), (1, 43), (3, 22)],
        &[(0, 61), (1, 1), (2, 26), (4, 14)],
        &[(0, 70), (1, 1), (2, 14), (4, 11), (6, 6)],
        &[(0, 47), (1, 1), (2, 54)],
        &[(0, 47), (1, 1), (2, 54)],
    ];
    for (map, expected) in serial.maps.iter().zip(expected_histograms) {
        assert!(map.verified_by_curve_enumeration);
        assert_eq!(map.fibers.len(), 102);
        assert_eq!(
            map.fiber_size_histogram
                .iter()
                .map(|(&a, &b)| (a, b))
                .collect::<Vec<_>>(),
            *expected
        );
        assert_eq!(
            map.fibers
                .iter()
                .map(|fiber| fiber.rational_points)
                .sum::<usize>(),
            109
        );
    }
    let quartic = &serial.maps[3].fibers[0];
    assert_eq!(
        (
            quartic.specialized_norm_degree,
            quartic.elimination_degree,
            quartic.reflection_extraction_degree
        ),
        (8, 4, 2)
    );
    assert_eq!(serial.maps[4].fixed_norm_degree, 1);
    assert_eq!(serial.maps[5].fixed_norm_degree, 1);
}

#[test]
fn maps_handle_poles_and_inseparability() {
    let config = Config {
        p: 5,
        curve: vec![0, 1, 0, 1],
        maps: vec![
            MapSpec {
                name: "x/y".into(),
                numerator: [vec![0, 1], vec![]],
                denominator: [vec![], vec![1]],
            },
            MapSpec {
                name: "x^5".into(),
                numerator: [vec![0, 0, 0, 0, 0, 1], vec![]],
                denominator: [vec![1], vec![]],
            },
        ],
        targets: Some(vec![Some(-5), None, Some(0)]),
    };
    let output = measure(&config, 2).unwrap();
    let ramified = &output.maps[0];
    assert_eq!(ramified.infinity_image, Some(0));
    assert_eq!(ramified.fibers[0].rational_points, 2);
    assert_eq!(ramified.fibers[1].target, None);
    assert_eq!(ramified.fibers.len(), 2);

    let inseparable = &output.maps[1];
    assert!(!inseparable.separable);
    assert_eq!(inseparable.map_degree, 10);
}
