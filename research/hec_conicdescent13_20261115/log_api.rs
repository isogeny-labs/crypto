pub fn run_logs() {
    const ELL: u64 = 1028893;
    let line = include_str!("POINT-LOG-RECORD.txt");
    let nums = line.split_once('[').unwrap().1.split_once(']').unwrap().0;
    let logs: Vec<u64> = nums.split(',').map(|s| s.trim().parse().unwrap()).collect();
    let pts = target::rational_points(&RAW, P);
    assert_eq!(pts.len(), logs.len());
    assert_eq!(logs.len(), 129);
    let o = ANCHORS[0];
    let idx = |p: [u64; 3]| pts.iter().position(|q| *q == p).expect("point_in_base");
    let lp = |p: [u64; 3]| (logs[idx(p)] + ELL - logs[idx(o)]) % ELL;
    let k0 = (lp(ANCHORS[3]) + lp(ANCHORS[4])) % ELL;
    let gl = 2 * 1005372 * ((lp(ANCHORS[2]) + ELL - lp(ANCHORS[3])) % ELL) % ELL;
    assert_ne!(gl, 0);
    let ef = crate::fp::Fp::new(ELL);
    let half = ef.inv(2);
    let sf = crate::fp::Fp::new(P);
    let curve = crate::jac::Curve {
        fp: sf,
        f: crate::poly::Poly::from_coeffs(&[31, 89, 73, 48, 91, 41, 123, 1]),
        g: 3,
    };
    let d1 = curve.mul(&curve.place(0, 44), 2);
    let btarget = curve.mul(&curve.place(3, 21), 2);
    let mut ks = vec![];
    let mut counts = [0; 2];
    for (case, (u, v)) in [
        (vec![9, 31, 82, 1], vec![50, 30, 116]),
        (vec![80, 89, 51, 1], vec![97, 30, 21]),
    ]
    .into_iter()
    .enumerate()
    {
        assert!(subst(&u, &v).is_empty());
        let mons = cmons(&u, &v);
        for a in 0..5 {
            for b in a + 1..5 {
                let mut rows: Vec<Vec<u64>> = (0..3)
                    .map(|i| {
                        mons.iter()
                            .map(|m| m.get(i).copied().unwrap_or(0))
                            .collect()
                    })
                    .collect();
                rows.push(row(ANCHORS[a]));
                rows.push(row(ANCHORS[b]));
                let Some(c) = null(rows) else { continue };
                if c[5] == 0 {
                    continue;
                }
                let r = resultant(&c);
                if r.len() != 9 {
                    continue;
                }
                let known = mul(
                    &u,
                    &mul(&[(P - ANCHORS[a][0]) % P, 1], &[(P - ANCHORS[b][0]) % P, 1]),
                );
                let (res, remn) = div(&r, &known);
                assert!(remn.is_empty());
                if gcd(res.clone(), derivative(&res)).len() != 1
                    || gcd(res.clone(), known).len() != 1
                {
                    continue;
                }
                if !add(&mpow(vec![0, 1], P, &res), &[0, 1], true).is_empty() {
                    continue;
                }
                let mut roots = vec![];
                assert!(split(res, &mut 20261115, &mut roots));
                roots.sort();
                assert_eq!(roots.len(), 3);
                let mut total = (lp(ANCHORS[a]) + lp(ANCHORS[b])) % ELL;
                for x in roots {
                    let h = gcd(fy(x), trim(cy(&c, x)));
                    assert_eq!(h.len(), 2);
                    let y = (P - h[0]) * power(h[1], 125) % P;
                    total = (total + lp([x, y, 1])) % ELL;
                }
                let xl = (3 * k0 % ELL * half % ELL + ELL - total) % ELL;
                let yl = if case == 0 { xl } else { (ELL - xl) % ELL };
                let bl = (yl + ELL - gl) % ELL;
                let k = bl * ef.inv(gl) % ELL;
                assert_eq!(
                    curve.mul(&d1, k),
                    btarget,
                    "source_public_scalar_verification"
                );
                ks.push(k);
                counts[case] += 1;
                println!("LOG_DESCENT case={case} pair={a},{b} centered_log={xl} generator_log={gl} target_log={bl} k={k} source_verified=true target_scalar_supplied=false");
            }
        }
    }
    assert_eq!(counts, [3, 4]);
    assert!(ks.iter().all(|k| *k == ks[0]));
    println!("PASS logarithmic_conic_descent=true accepted=7 beta_group_operations=0 Dinf_group_operations=0 candidate_jacobian_additions=0 public_sign_reused=true cold_measurement=false");
}
