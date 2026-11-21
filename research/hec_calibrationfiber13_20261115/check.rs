pub fn verify_pair(pair: [[u64; 3]; 2], source: [u64; 8]) {
    let j = QuarticJacobian::with_points(
        127,
        [1, 58, 103, 47, 66, 88, 51, 80, 84, 123, 34, 83, 109, 1, 111],
        vec![
            [18, 52, 1],
            [28, 86, 1],
            [43, 125, 1],
            [101, 43, 1],
            [34, 113, 1],
        ],
    )
    .unwrap();
    let s = good::Sections::new();
    let o = [18, 52, 1];
    let mut h = j.zero();
    for p in j.d_inf {
        let d = target::difference(&j, p, o);
        h = j.add_raw(&h, &j.add_raw(&d, &d));
    }
    for p in [[101, 43, 1], [34, 113, 1]] {
        h = j.add_raw(&h, &j.neg_raw(&target::difference(&j, p, o)));
    }
    let beta = target::scalar(&j, h, 514447);
    let p = target::difference(&j, pair[0], pair[1]);
    let image = j.add_raw(&p, &p);
    assert_ne!(image, j.zero());
    assert_eq!(target::scalar(&j, image, 1028893), j.zero());
    let (y, _) = coord(&j, &s, image, j.neg_raw(&beta)).expect("target_chart");
    let raw: Vec<u64> = include_str!("../hec_alignment13_20261115/atlas/EXPECTED-MATRIX.txt")
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let x = std::array::from_fn(|i| (0..8).fold(0, |z, k| (z + raw[8 * i + k] * source[k]) % 127));
    assert!(proportional(x, y), "post_selection_expected_matrix");
    println!("CALIBRATION_CONTROL existing_matrix_consistent=true nonzero_ell_pair=true");
}
