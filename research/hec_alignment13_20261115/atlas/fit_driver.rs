fn transform(m: &[[u64; 8]; 8], x: [u64; 8]) -> [u64; 8] {
    std::array::from_fn(|i| m[i].iter().zip(x).fold(0, |z, (a, b)| (z + a * b) % 127))
}
pub fn run_fit() {
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(600));
        eprintln!("CAP aggregate600seconds");
        std::process::exit(124)
    });
    let f = [1, 58, 103, 47, 66, 88, 51, 80, 84, 123, 34, 83, 109, 1, 111];
    let pts = vec![
        [18, 52, 1],
        [28, 86, 1],
        [43, 125, 1],
        [101, 43, 1],
        [34, 113, 1],
    ];
    let j = QuarticJacobian::with_points(127, f, pts).unwrap();
    let sec = good::Sections::new();
    good::verify_canonical_line();
    let o = [18, 52, 1];
    let ell = 1028893;
    let mut h = j.zero();
    for p in j.d_inf {
        let d = target::difference(&j, p, o);
        h = j.add_raw(&h, &j.add_raw(&d, &d));
    }
    for p in [[101, 43, 1], [34, 113, 1]] {
        h = j.add_raw(&h, &j.neg_raw(&target::difference(&j, p, o)));
    }
    assert_eq!(target::scalar(&j, h, ell), j.zero());
    let beta = target::scalar(&j, h, (ell + 1) / 2);
    assert_eq!(j.add_raw(&beta, &beta), h);
    let shift = j.neg_raw(&beta);
    // Geometric old-map certificate, converted via differences in THIS Dinf model.
    let pair = target::difference(&j, [43, 125, 1], [101, 43, 1]);
    let qref = target::scalar(&j, j.add_raw(&pair, &pair), 1005372);
    assert_ne!(qref, j.zero());
    assert_eq!(target::scalar(&j, qref, ell), j.zero());
    let src = super::source::Source::new();
    let mut samples = Vec::new();
    for n in 1..=64 {
        let Some(x) = src.eval(&src.n(n)) else {
            println!("FIT_EXCLUDE n={n} source_chart");
            continue;
        };
        let y = target::scalar(&j, qref, n);
        let Some((y, _)) = coord(&j, &sec, y, shift) else {
            println!("FIT_EXCLUDE n={n} target_chart");
            continue;
        };
        samples.push((n, x, y));
        println!("FIT_SAMPLE n={n} source={x:?} target={y:?}");
        if samples.len() == 16 {
            break;
        }
    }
    assert_eq!(samples.len(), 16, "calibration cap");
    let mut rows = Vec::new();
    for (_, x, y) in &samples[..12] {
        for i in 0..8 {
            for j in i + 1..8 {
                let mut r = vec![0; 64];
                for k in 0..8 {
                    r[8 * j + k] = y[i] * x[k] % 127;
                    r[8 * i + k] = (127 - y[j] * x[k] % 127) % 127;
                }
                rows.push(r);
            }
        }
    }
    let (rank, basis) = good::linear_kernel(rows, 64);
    assert_eq!(rank, 63, "projective alignment not unique");
    assert_eq!(basis.len(), 1);
    let m: [[u64; 8]; 8] = std::array::from_fn(|i| std::array::from_fn(|k| basis[0][8 * i + k]));
    let expected: Vec<u64> = include_str!("EXPECTED-MATRIX.txt")
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(
        m.iter().flatten().copied().collect::<Vec<_>>(),
        expected,
        "same frozen calibration matrix"
    );
    assert_eq!(
        samples.iter().map(|z| z.0).collect::<Vec<_>>(),
        vec![2, 3, 4, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]
    );
    let (mrank, _) = good::linear_kernel(m.iter().map(|r| r.to_vec()).collect(), 8);
    assert_eq!(mrank, 8);
    for i in 0..8 {
        println!("ALIGN_ROW {i} {:?}", m[i]);
    }
    for (n, x, y) in &samples {
        assert!(proportional(transform(&m, *x), *y));
        println!(
            "ALIGN_CONTROL n={n} fit={} pass=true",
            samples[..12].iter().any(|z| z.0 == *n)
        );
    }
    let b = src.b();
    assert!(
        src.eval(&b).is_none(),
        "registered degree2source chart premise changed"
    );
    println!("RAW_B_CHART unsupported_degree2=true");
    let bp = target::difference(&j, [28, 86, 1], [18, 52, 1]);
    let qb = target::scalar(&j, j.add_raw(&bp, &bp), 203071);
    assert_ne!(qb, j.zero());
    assert_eq!(target::scalar(&j, qb, ell), j.zero());
    let d1 = src.n(1);
    let d2 = src.n(2);
    let b2 = src.add(&b, &b);
    let nb = src.neg(&b);
    let cases = [
        ("B_plus_D1", src.add(&b, &d1), j.add_raw(&qb, &qref)),
        ("two_B", b2, j.add_raw(&qb, &qb)),
        (
            "minus_B_plus_D1",
            src.add(&nb, &d1),
            j.add_raw(&j.neg_raw(&qb), &qref),
        ),
        (
            "B_plus_two_D1",
            src.add(&b, &d2),
            j.add_raw(&qb, &j.add_raw(&qref, &qref)),
        ),
    ];
    let mut hold = Vec::new();
    for (label, x, y) in cases {
        let x = src.eval(&x).expect("registered source holdout chart");
        let (y, kind) = coord(&j, &sec, y, shift).expect("registered target holdout chart");
        let got = transform(&m, x);
        assert!(proportional(got, y), "mixed holdout {label}");
        println!("SOURCE_HOLDOUT label={label} source={x:?} mapped={got:?} oracle={y:?} rational_factors={kind}");
        hold.push((x, y));
    }
    let (x, y) = hold[0];
    let k = x.iter().position(|a| *a != 0).unwrap();
    let p = y.iter().position(|a| *a != 0).unwrap();
    let row = (p + 1) % 8;
    let mut bad = m;
    bad[row][k] = (bad[row][k] + 1) % 127;
    assert!(!proportional(transform(&bad, x), y));
    let cross = CROSS.load(std::sync::atomic::Ordering::Relaxed);
    let swapped = SWAPPED.load(std::sync::atomic::Ordering::Relaxed);
    assert!(cross > 0 && swapped > 0);
    println!("ATLAS cross_checks={cross} swapped_required={swapped} same_matrix=true");
    println!("PASS alignment_rank={rank} matrix_rank={mrank} training=12 scalar_holdout=4 mixed_source_holdout=4 corrupt_matrix_rejected=true raw_B_fixed_translate_required=true inverse_lift=false timing_claim=false");
}
