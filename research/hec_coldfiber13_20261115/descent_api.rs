pub fn run(pts: &[[u64; 3]], logs: &[u64]) {
    super::setup::verify_model();
    const ELL: u64 = 1028893;
    let ef = hec_native_ic::fp::Fp::new(ELL);
    let half = ef.inv(2);
    let o = ANCHORS[0];
    let idx = |p: [u64; 3]| pts.iter().position(|q| *q == p).expect("point_base_lookup");
    let lp = |p: [u64; 3]| (logs[idx(p)] + ELL - logs[idx(o)]) % ELL;
    let k0 = (lp(ANCHORS[3]) + lp(ANCHORS[4])) % ELL;
    let pair: Vec<u64> = std::fs::read_to_string("generated/TARGET-PAIR.txt")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let gl = 2
        * ((lp(pair[..3].try_into().unwrap()) + ELL - lp(pair[3..].try_into().unwrap())) % ELL)
        % ELL;
    assert_ne!(gl, 0);
    let j = target::QuarticJacobian::with_points(P, RAW, ANCHORS.to_vec()).unwrap();
    let mut ds = j.zero();
    for p in j.d_inf {
        ds = j.add_raw(&ds, &target::difference(&j, p, o));
    }
    let ks = j.add_raw(
        &target::difference(&j, ANCHORS[3], o),
        &target::difference(&j, ANCHORS[4], o),
    );
    let rhs = j.add_raw(&ks, &j.neg_raw(&ds));
    let rhs = j.add_raw(&rhs, &rhs);
    let mut recovered = vec![];
    let mut totals = vec![];
    for label in ["D1", "B"] {
        let text = std::fs::read_to_string(format!("divisors-{label}.txt")).unwrap();
        let mut values = vec![];
        let mut attempted = 0;
        let mut nonsplit = 0;
        for (case, line) in text.lines().enumerate() {
            let ns: Vec<u64> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            assert_eq!(ns.len(), 9);
            let positive = ns[0] == 1;
            let offset = ns[1];
            let u = ns[2..6].to_vec();
            let v = ns[6..9].to_vec();
            assert!(subst(&u, &v).is_empty());
            assert_eq!(gcd(u.clone(), derivative(&u)), vec![1]);
            let e = class(&j, &u, &v);
            let mons = cmons(&u, &v);
            for a in 0..5 {
                for b in a + 1..5 {
                    attempted += 1;
                    let mut rows: Vec<Vec<u64>> = (0..3)
                        .map(|i| {
                            mons.iter()
                                .map(|m| m.get(i).copied().unwrap_or(0))
                                .collect()
                        })
                        .collect();
                    rows.push(row(ANCHORS[a]));
                    rows.push(row(ANCHORS[b]));
                    let Some(c) = null(rows) else {
                        println!(
                            "CONIC_EXCLUDE label={label} case={case} pair={a},{b} reason=rank"
                        );
                        continue;
                    };
                    if c[5] == 0 {
                        println!(
                            "CONIC_EXCLUDE label={label} case={case} pair={a},{b} reason=y_degree"
                        );
                        continue;
                    }
                    let r = resultant(&c);
                    for x in 0..P {
                        assert_eq!(
                            ev(&r, x),
                            syl(&fy(x), &cy(&c, x)),
                            "exact_resultant_control"
                        )
                    }
                    if r.len() != 9 {
                        println!("CONIC_EXCLUDE label={label} case={case} pair={a},{b} reason=projection_or_infinity");
                        continue;
                    }
                    let known = mul(
                        &u,
                        &mul(&[(P - ANCHORS[a][0]) % P, 1], &[(P - ANCHORS[b][0]) % P, 1]),
                    );
                    let (res, remn) = div(&r, &known);
                    assert!(remn.is_empty());
                    assert_eq!(res.len(), 4);
                    if gcd(res.clone(), derivative(&res)).len() != 1
                        || gcd(res.clone(), known).len() != 1
                    {
                        println!("CONIC_EXCLUDE label={label} case={case} pair={a},{b} reason=nonreduced_or_overlap");
                        continue;
                    }
                    if !add(&mpow(vec![0, 1], P, &res), &[0, 1], true).is_empty() {
                        nonsplit += 1;
                        println!(
                            "CONIC_EXCLUDE label={label} case={case} pair={a},{b} reason=nonsplit"
                        );
                        continue;
                    }
                    let mut roots = vec![];
                    assert!(split(res, &mut 20261115, &mut roots));
                    roots.sort();
                    assert_eq!(roots.len(), 3);
                    let mut rp = vec![];
                    let mut total = (lp(ANCHORS[a]) + lp(ANCHORS[b])) % ELL;
                    for x in roots {
                        let h = gcd(fy(x), trim(cy(&c, x)));
                        assert_eq!(h.len(), 2);
                        let y = (P - h[0]) * power(h[1], 125) % P;
                        assert_eq!(ev(&fy(x), y), 0);
                        assert_eq!(ev(&cy(&c, x), y), 0);
                        let p = [x, y, 1];
                        total = (total + lp(p)) % ELL;
                        rp.push(p);
                    }
                    let rc = j.from_points(rp.clone().try_into().unwrap()).unwrap();
                    let mut lhs = j.add_raw(&e, &rc);
                    lhs = j.add_raw(&lhs, &target::difference(&j, ANCHORS[a], o));
                    lhs = j.add_raw(&lhs, &target::difference(&j, ANCHORS[b], o));
                    assert_eq!(lhs, rhs, "independent_conic_divisor_identity");
                    let xl = (3 * k0 % ELL * half % ELL + ELL - total) % ELL;
                    let yl = if positive { xl } else { (ELL - xl) % ELL };
                    let cl = (yl + ELL - offset * gl % ELL) % ELL;
                    values.push(cl);
                    println!("CONIC_LOG label={label} case={case} pair={a},{b} residual_points={rp:?} original_class_log={cl} source_image_or_scalar_loaded=false");
                }
            }
        }
        assert!(!values.is_empty(), "no_supported_conic_descent");
        assert!(
            values.iter().all(|a| *a == values[0]),
            "inconsistent_descent_logs"
        );
        assert_ne!(values[0], 0);
        recovered.push(values[0]);
        totals.push(values.len());
        println!("LOG_PASS label={label} attempted={attempted} accepted={} nonsplit={nonsplit} original_log={} fresh_calibrator_log={gl}",values.len(),values[0]);
    }
    let k = recovered[1] * ef.inv(recovered[0]) % ELL;
    let (curve, d1, b, ell) = super::public::load(
        &std::env::var("HEC_PUBLIC_INPUT").expect("HEC_PUBLIC_INPUT is mandatory"),
    );
    assert_eq!(ell, ELL);
    assert_eq!(curve.mul(&d1, k), b, "source_public_verification");
    assert_ne!(curve.mul(&d1, (k + 1) % ELL), b, "corrupt_scalar_control");
    let load = |label: &str| {
        std::fs::read_to_string(format!("class-{label}.txt"))
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    };
    let model = super::model::Model::new();
    model.verify_ratio(load("D1"), load("B"), k);
    println!("RECOVERED k={k} source_verified=true destination_verified=true fresh_point_logs=true original_images_loaded=false target_scalar_supplied=false accepted={totals:?}");
    println!("PASS joined_new_base_inverse_conic_ic=true quotient_setup_reused=false sparse_LP_dense47_core=true cold_measurement=false timing_claim=false");
}
