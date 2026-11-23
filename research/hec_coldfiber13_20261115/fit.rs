fn transform(m: &[[u64; 8]; 8], x: [u64; 8]) -> [u64; 8] {
    std::array::from_fn(|i| m[i].iter().zip(x).fold(0, |z, (a, b)| (z + a * b) % 127))
}
fn store(path: &std::path::Path, x: &[u64]) {
    std::fs::write(
        path,
        x.iter().map(u64::to_string).collect::<Vec<_>>().join(" ") + "\n",
    )
    .unwrap();
}
pub fn run() {
    let out = std::env::args()
        .nth(1)
        .expect("write-once output directory");
    let out = std::path::Path::new(&out);
    assert!(!out.exists());
    std::fs::create_dir(out).unwrap();
    let (src, base, pair) = super::source::calibrate();
    super::setup::verify_model();
    let f = super::setup::qf();
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
    let mut h = j.zero();
    for p in j.d_inf {
        let d = target::difference(&j, p, o);
        h = j.add_raw(&h, &j.add_raw(&d, &d));
    }
    for p in [[101, 43, 1], [34, 113, 1]] {
        h = j.add_raw(&h, &j.neg_raw(&target::difference(&j, p, o)));
    }
    let beta = target::scalar(&j, h, 514447);
    assert_eq!(j.add_raw(&beta, &beta), h);
    let shift = j.neg_raw(&beta);
    let d = target::difference(&j, pair[0], pair[1]);
    let image = j.add_raw(&d, &d);
    assert_ne!(image, j.zero());
    assert_eq!(target::scalar(&j, image, 1028893), j.zero());
    let mut samples = vec![];
    for n in 1..=64 {
        let Some(x) = src.eval(&src.mul_base(&base, n)) else {
            println!("FIT_EXCLUDE n={n} source_chart");
            continue;
        };
        let Some((y, _)) = coord(&j, &sec, target::scalar(&j, image, n), shift) else {
            println!("FIT_EXCLUDE n={n} target_chart");
            continue;
        };
        samples.push((n, x, y));
        if samples.len() == 16 {
            break;
        }
    }
    assert_eq!(samples.len(), 16, "fit_cap64");
    let mut rows = vec![];
    for (_, x, y) in &samples[..12] {
        for a in 0..8 {
            for b in a + 1..8 {
                let mut r = vec![0; 64];
                for k in 0..8 {
                    r[8 * b + k] = y[a] * x[k] % 127;
                    r[8 * a + k] = (127 - y[b] * x[k] % 127) % 127;
                }
                rows.push(r);
            }
        }
    }
    let (rank, kernel) = good::linear_kernel(rows, 64);
    assert_eq!(rank, 63);
    assert_eq!(kernel.len(), 1);
    let m: [[u64; 8]; 8] = std::array::from_fn(|i| std::array::from_fn(|k| kernel[0][8 * i + k]));
    let (mrank, _) = good::linear_kernel(m.iter().map(|r| r.to_vec()).collect(), 8);
    assert_eq!(mrank, 8);
    for (n, x, y) in &samples {
        assert!(proportional(transform(&m, *x), *y));
        println!(
            "FIT_CONTROL n={n} training={} pass=true",
            samples[..12].iter().any(|s| s.0 == *n)
        );
    }
    let mut bad = m;
    let x = samples[12].1;
    let y = samples[12].2;
    let k = x.iter().position(|a| *a != 0).unwrap();
    let p = y.iter().position(|a| *a != 0).unwrap();
    bad[(p + 1) % 8][k] = (bad[(p + 1) % 8][k] + 1) % 127;
    assert!(!proportional(transform(&bad, x), y));
    store(
        &out.join("MATRIX.txt"),
        &m.iter().flatten().copied().collect::<Vec<_>>(),
    );
    store(&out.join("SOURCE-BASE-U.txt"), &base.u.c[..base.u.n]);
    store(&out.join("SOURCE-BASE-V.txt"), &base.v.c[..base.v.n]);
    store(
        &out.join("TARGET-PAIR.txt"),
        &pair.iter().flatten().copied().collect::<Vec<_>>(),
    );
    let twice = src.mul_base(&base, 2);
    for (label, c) in [("D1", src.n(1)), ("B", src.b())] {
        let mut chosen = None;
        for offset in 1..=4 {
            let a = src.add(&c, &src.mul_base(&base, offset));
            let b = src.add(&a, &base);
            let Some(x) = src.eval(&a) else {
                println!("LIFT_EXCLUDE label={label} offset={offset} source_primary");
                continue;
            };
            let Some(y) = src.eval(&b) else {
                println!("LIFT_EXCLUDE label={label} offset={offset} source_adjacent");
                continue;
            };
            let x = transform(&m, x);
            let y = transform(&m, y);
            if x[0] == 0 {
                println!("LIFT_EXCLUDE label={label} offset={offset} h0_zero");
                continue;
            }
            chosen = Some((offset, x, y));
            break;
        }
        let (offset, x, y) = chosen.expect("fixed_translation_chart_cap4");
        store(&out.join(format!("{label}-KUMMER.txt")), &x);
        store(&out.join(format!("{label}-NEXT-KUMMER.txt")), &y);
        std::fs::write(
            out.join(format!("{label}-OFFSET.txt")),
            format!("{offset}\n"),
        )
        .unwrap();
        println!("LIFT_INPUT label={label} offset={offset} lambda={x:?} adjacent={y:?} original_image_or_scalar_used=false");
        let gen = std::env::current_exe()
            .unwrap()
            .with_file_name("generate13");
        let cmd = std::process::Command::new(gen)
            .arg(out.join(label))
            .arg(out.join(format!("{label}-KUMMER.txt")))
            .output()
            .unwrap();
        std::fs::write(out.join(format!("{label}-GENERATE.txt")), &cmd.stdout).unwrap();
        std::fs::write(out.join(format!("{label}-GENERATE.stderr")), &cmd.stderr).unwrap();
        assert!(cmd.status.success(), "equation_generator");
    }
    let _ = twice;
    println!("PASS new_base_alignment=true rank={rank} matrix_rank={mrank} training=12 holdout=4 corrupt_matrix_rejected=true independent_fiber_selected=true old_matrix_loaded=false original_D1_image_loaded=false inverse_systems=2 timing_claim=false");
}
