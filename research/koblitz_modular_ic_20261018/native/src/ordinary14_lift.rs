//! Target and scalar-fiber reconstruction for all five residual functions.
use super::*;
#[path = "ordinary14_images.rs"]
pub(crate) mod images14;
fn targets(k: &K, fi: usize, x: &B, y: &B, seed: &mut u64) -> (P, Vec<(B, B)>) {
    let c = if [0, 2, 4].contains(&fi) {
        one()
    } else {
        vec![]
    };
    if fi == 0 {
        let u = add(x, &one());
        assert!(!u.is_empty());
        let rhs = add(&add(&kp(k, &u, 3), &k.sq(&u)), &one());
        let g = km(k, &rhs, &ki(k, &k.sq(&u)));
        let pts = if let Some(w) = k.asroot(&g) {
            let v = km(k, &u, &w);
            vec![(u.clone(), v.clone()), (u.clone(), add(&v, &u))]
        } else {
            vec![]
        };
        return (vec![u, one()], pts);
    }
    let poly = if fi <= 2 {
        let v = add(y, &c);
        vec![add(&one(), &k.sq(&v)), v, one(), one()]
    } else {
        vec![
            add(&k.sq(x), &c),
            km(k, x, &c),
            add(&add(&k.sq(x), &k.sq(y)), &km(k, x, y)),
            k.sq(x),
        ]
    };
    let pts = proots(k, poly.clone(), seed)
        .into_iter()
        .map(|u| {
            let v = if fi <= 2 {
                add(y, &c)
            } else {
                km(k, &add(&km(k, &u, y), &c), &ki(k, x))
            };
            (u, v)
        })
        .collect();
    (poly, pts)
}
pub fn middles(dir: &Path) {
    REDUCTION_LIMIT.store(1_000_000_000, Ordering::Relaxed);
    let input = crate::round04::read(&dir.join("o20-five-pushed-divisors.json"));
    let base = crate::round04::read(&dir.join("m1-modular-base.json"));
    let terms: Vec<(usize, usize)> =
        serde_json::from_value(base["mod2_phi11_terms"].clone()).unwrap();
    let mut seed = 20261021;
    let mut records = vec![];
    for (fi, row) in input["records"].as_array().unwrap().iter().enumerate() {
        for (pi, p) in row["points"].as_array().unwrap().iter().enumerate() {
            if p["stratum"] != "ordinary" {
                continue;
            }
            let k = K::new(parse(p["x_minpoly_bits"].as_str().unwrap()));
            let x = k.red(&[2]);
            let y = parse(p["y_residue_bits"].as_str().unwrap());
            assert_eq!(p["multiplicity"], 1);
            assert!(valid(&k, &(x.clone(), y.clone())));
            let js = triple(&k, &(x.clone(), y.clone()));
            let (poly, ts) = targets(&k, fi, &x, &y, &mut seed);
            let mut attempts = vec![];
            let mut candidates = vec![];
            for target in ts {
                assert!(valid(&k, &target));
                let c = if [0, 2, 4].contains(&fi) {
                    one()
                } else {
                    vec![]
                };
                let val = match fi {
                    0 => add(&add(&x, &target.0), &c),
                    1 | 2 => add(&add(&y, &target.1), &c),
                    _ => add(&add(&km(&k, &x, &target.1), &km(&k, &target.0, &y)), &c),
                };
                assert!(val.is_empty());
                let jt = triple(&k, &target);
                let commons: Vec<_> = (0..3)
                    .map(|i| common(&k, &terms, &js[i], &jt[i], &mut seed))
                    .collect();
                let mut count = 0;
                for j0 in &commons[0] {
                    for j7 in &commons[1] {
                        let t0s = param(&k, j0, j7, &mut seed);
                        for j49 in &commons[2] {
                            let t7s = param(&k, j7, j49, &mut seed);
                            for t0 in &t0s {
                                for t7 in &t7s {
                                    count += 1;
                                    assert!(count <= 4096);
                                    let mut xx = km(&k, t0, t7);
                                    for _ in 0..k.d - 2 {
                                        xx = k.sq(&xx);
                                    }
                                    let den = add(&add(&k.sq(&xx), &xx), &one());
                                    if den.is_empty() {
                                        continue;
                                    }
                                    let yy = km(&k, &add(&add(t7, &xx), &one()), &ki(&k, &den));
                                    let middle = (xx, yy);
                                    assert!(valid(&k, &middle));
                                    let jm = [j0.clone(), j7.clone(), j49.clone()];
                                    assert_eq!(triple(&k, &middle), jm);
                                    candidates.push(json!({"target":points_json(&target),"target_j":jt.iter().map(|z|hex(z)).collect::<Vec<_>>(),"middle":points_json(&middle),"middle_j":jm.iter().map(|z|hex(z)).collect::<Vec<_>>()}));
                                }
                            }
                        }
                    }
                }
                attempts.push(json!({"target":points_json(&target),"common_j_counts":commons.iter().map(Vec::len).collect::<Vec<_>>(),"compatible_triples":count}));
            }
            println!(
                "O21:fn{fi},point{pi},d{},{} candidates",
                k.d,
                candidates.len()
            );
            let r = json!({"function_index":fi,"pushed_point_index":pi,"field_modulus_bits":hex(&k.p),"field_degree":k.d,"source_target":points_json(&(x,y)),"source_target_j":js.iter().map(|z|hex(z)).collect::<Vec<_>>(),"target_polynomial":poly.iter().map(|z|hex(z)).collect::<Vec<_>>(),"attempts":attempts,"middle_candidates":candidates,"actual_transport_verified":false});
            round02::save(dir, &format!("o21-point-{}.json", records.len()), &r);
            records.push(r);
        }
    }
    round02::save(
        dir,
        "o21-five-middle-candidates.json",
        &json!({"schema":"koblitz.round14/o21-v1","seed":20261021,"records":records,"reduction_steps":REDUCTIONS.load(Ordering::Relaxed),"actual_transport_verified":false}),
    );
}
pub fn fiber(dir: &Path, index: usize) {
    REDUCTION_LIMIT.store(5_000_000_000, Ordering::Relaxed);
    let input = crate::round04::read(&dir.join("o21-five-middle-candidates.json"));
    let row = &input["records"][index];
    assert!(!row.is_null());
    let k = K::new(parse(row["field_modulus_bits"].as_str().unwrap()));
    let mut seed = 20261021 + index as u64;
    let mut success = vec![];
    let mut rejected = vec![];
    for cand in row["middle_candidates"].as_array().unwrap() {
        let decode = |v: &serde_json::Value| {
            v.as_array()
                .unwrap()
                .iter()
                .map(|s| parse(s.as_str().unwrap()))
                .collect::<Vec<_>>()
        };
        let jm = decode(&cand["middle_j"]);
        let js = decode(&row["source_target_j"]);
        let jt = decode(&cand["target_j"]);
        let b = ki(&k, &jm[0]);
        let b7 = ki(&k, &jm[1]);
        let (ks1, r7a) = kernels(&k, &b, 7, &mut seed);
        let (ks2, r7b) = kernels(&k, &b7, 7, &mut seed);
        let k1 = choose(&k, &b, &ks1, &jm[1]);
        let k2 = choose(&k, &b7, &ks2, &jm[2]);
        assert_ne!(jm[0], jm[2]);
        let (ks11, r11) = kernels(&k, &b, 11, &mut seed);
        let kp = choose(&k, &b, &ks11, &js[0]);
        let kq = choose(&k, &b, &ks11, &jt[0]);
        if kp == kq {
            rejected.push(json!({"candidate":cand,"reason":"same actual 11 kernel; excluded noncyclic121 branch","kernel":kp.iter().map(|z|hex(z)).collect::<Vec<_>>(),"division_factorization":r11}));
            continue;
        }
        let gotp = transported(&k, &b, &b7, &k1, &k2, &kp);
        let gotq = transported(&k, &b, &b7, &k1, &k2, &kq);
        if gotp.to_vec() != js || gotq.to_vec() != jt {
            rejected.push(json!({"candidate":cand,"reason":"actual transported 7/11 triples disagree","transported":[gotp.iter().map(|z|hex(z)).collect::<Vec<_>>(),gotq.iter().map(|z|hex(z)).collect::<Vec<_>>()]}));
            continue;
        }
        let (fs, attempts) = pfactor(&k, kp.clone(), &mut seed);
        let pts=fs.iter().map(|f|{assert!([1,5].contains(&(f.len()-1)));json!({"base_field_modulus_bits":hex(&k.p),"extension_polynomial_coefficients":f.iter().map(|z|hex(z)).collect::<Vec<_>>(),"closed_degree":k.d*(f.len()-1),"multiplicity":1,"whole_deck_fiber":fs.len()==1,"even_projected_image":if fs.len()==1{Some("zero")}else{None},"odd_projected_image":if fs.len()==1{Some("zero")}else{None}})}).collect::<Vec<_>>();
        success.push(json!({"middle":cand["middle"],"middle_j":cand["middle_j"],"target":cand["target"],"target_j":cand["target_j"],"cyclic49_kernels":[k1.iter().map(|z|hex(z)).collect::<Vec<_>>(),k2.iter().map(|z|hex(z)).collect::<Vec<_>>()],"first11_kernel":kp.iter().map(|z|hex(z)).collect::<Vec<_>>(),"second11_kernel":kq.iter().map(|z|hex(z)).collect::<Vec<_>>(),"division_factorizations":[r7a,r7b,r11],"four_commuting_square_checks":true,"fiber_factor_attempts":attempts,"source_closed_points":pts}));
    }
    assert_eq!(success.len(), 1);
    let r = json!({"index":index,"function_index":row["function_index"],"pushed_point_index":row["pushed_point_index"],"field_degree":k.d,"verified_lifts":success,"rejected_candidates":rejected,"reduction_steps":REDUCTIONS.load(Ordering::Relaxed)});
    round02::save(dir, &format!("o22-point-{index}.json"), &r);
    println!(
        "O22:point{index},d{},fiber degrees{}",
        k.d,
        r["verified_lifts"][0]["source_closed_points"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| p["closed_degree"].to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}

pub fn verify_point(dir: &Path, index: usize) {
    REDUCTION_LIMIT.store(1_000_000_000, Ordering::Relaxed);
    let middles = crate::round04::read(&dir.join("o21-five-middle-candidates.json"));
    let m = &middles["records"][index];
    let r = crate::round04::read(&dir.join(format!("o22-point-{index}.json")));
    let k = K::new(parse(m["field_modulus_bits"].as_str().unwrap()));
    assert!(irreducible(&k.p));
    let source = decodepoint(&m["source_target"]);
    assert!(valid(&k, &source));
    let js = triple(&k, &source);
    let fi = r["function_index"].as_u64().unwrap() as usize;
    let lift = &r["verified_lifts"][0];
    assert_eq!(r["verified_lifts"].as_array().unwrap().len(), 1);
    let target = decodepoint(&lift["target"]);
    let mid = decodepoint(&lift["middle"]);
    assert!(valid(&k, &target) && valid(&k, &mid));
    let c = if [0, 2, 4].contains(&fi) {
        one()
    } else {
        vec![]
    };
    let value = match fi {
        0 => add(&add(&source.0, &target.0), &c),
        1 | 2 => add(&add(&source.1, &target.1), &c),
        _ => add(
            &add(&km(&k, &source.0, &target.1), &km(&k, &target.0, &source.1)),
            &c,
        ),
    };
    assert!(value.is_empty());
    let jm = triple(&k, &mid);
    let jt = triple(&k, &target);
    assert_ne!(jm[0], jm[2]);
    let decode = |v: &serde_json::Value| {
        v.as_array()
            .unwrap()
            .iter()
            .map(|s| parse(s.as_str().unwrap()))
            .collect::<P>()
    };
    let k1 = decode(&lift["cyclic49_kernels"][0]);
    let k2 = decode(&lift["cyclic49_kernels"][1]);
    let kp = decode(&lift["first11_kernel"]);
    let kq = decode(&lift["second11_kernel"]);
    assert_ne!(kp, kq);
    let b = ki(&k, &jm[0]);
    let b7 = ki(&k, &jm[1]);
    for (kernel, ell, coeff) in [(&k1, 7, &b), (&k2, 7, &b7), (&kp, 11, &b), (&kq, 11, &b)] {
        assert_eq!(kernel.len(), (ell + 1) / 2);
        assert!(pd(&k, &division(&k, coeff, ell), kernel).1.is_empty());
        assert!(pevalmod(&k, kernel, &doubling(&k, coeff, kernel), kernel).is_empty());
        assert_eq!(pg(&k, kernel.clone(), deriv(kernel)).len(), 1);
    }
    assert_eq!(quotient_b(&k, &b, &k1), b7);
    assert_eq!(quotient_b(&k, &b7, &k2), ki(&k, &jm[2]));
    assert_eq!(transported(&k, &b, &b7, &k1, &k2, &kp), js);
    assert_eq!(transported(&k, &b, &b7, &k1, &k2, &kq), jt);
    let mut product = vec![one()];
    let mut degrees = vec![];
    for p in lift["source_closed_points"].as_array().unwrap() {
        let factor = decode(&p["extension_polynomial_coefficients"]);
        assert!(pirred(&k, &factor));
        assert_eq!(p["multiplicity"], 1);
        let d = k.d * (factor.len() - 1);
        assert_eq!(p["closed_degree"], d);
        degrees.push(d);
        product = pm(&k, &product, &factor);
    }
    assert_eq!(product, kp);
    assert_eq!(degrees.iter().sum::<usize>(), 5 * k.d);
    round02::save(
        dir,
        &format!("o23-point-{index}.json"),
        &json!({"schema":"koblitz.round14/o23-v1","index":index,"function_index":fi,"base_degree":k.d,"source_degrees":degrees,"multiplicities_all_one":true,"all_kernel_checks":true,"four_commuting_squares":true,"exact_scalar_factor_product":true,"reduction_steps":REDUCTIONS.load(Ordering::Relaxed),"scope":"parsed verification;shared arithmetic primitives"}),
    );
    println!("O23:point{index},parsed support/kernel/fiber checks pass");
}
pub fn assemble(dir: &Path) {
    REDUCTION_LIMIT.store(1_000_000_000, Ordering::Relaxed);
    let norm = crate::round04::read(&dir.join("o19-five-Hecke-norms.json"));
    let supp = crate::round04::read(&dir.join("o20-five-pushed-divisors.json"));
    let (u, v) = xy(1024);
    for (fi, r) in norm["records"].as_array().unwrap().iter().enumerate() {
        let got = recover(
            &parse(r["q_series_normalized_bits"].as_str().unwrap()),
            r["pole_bound"].as_u64().unwrap() as usize,
            &u,
            &v,
            1024,
        );
        for (key, value) in got.as_object().unwrap() {
            assert_eq!(&r[key], value);
        }
        let (zs, _) = zero_divisor(
            &parse(r["A_bits"].as_str().unwrap()),
            &parse(r["B_bits"].as_str().unwrap()),
        );
        let pts = supp["records"][fi]["points"].as_array().unwrap();
        assert_eq!(zs.len(), pts.len());
        for (z, p) in zs.iter().zip(pts) {
            assert_eq!(hex(&z.p), p["x_minpoly_bits"]);
            assert_eq!(
                z.y.as_ref().map(|y| hex(y)),
                p["y_residue_bits"].as_str().map(str::to_owned)
            );
            assert_eq!(z.degree, p["closed_degree"].as_u64().unwrap() as usize);
            assert_eq!(z.mult, p["multiplicity"].as_u64().unwrap() as usize);
            assert_eq!(z.class, p["stratum"]);
        }
    }
    let mut records = vec![];
    let mut totals = [0usize; 5];
    let mut counts = [0usize; 5];
    let mut split = 0;
    let mut whole = 0;
    for i in 0..25 {
        let r = crate::round04::read(&dir.join(format!("o22-point-{i}.json")));
        let checked = crate::round04::read(&dir.join(format!("o23-point-{i}.json")));
        let fi = r["function_index"].as_u64().unwrap() as usize;
        let pts = r["verified_lifts"][0]["source_closed_points"]
            .as_array()
            .unwrap();
        let ds = pts
            .iter()
            .map(|p| p["closed_degree"].as_u64().unwrap() as usize)
            .collect::<Vec<_>>();
        assert_eq!(json!(ds), checked["source_degrees"]);
        totals[fi] += ds.iter().sum::<usize>();
        counts[fi] += ds.len();
        if pts.len() == 1 {
            whole += 1
        } else {
            split += pts.len();
        }
        records.push(json!({"index":i,"function_index":fi,"pushed_point_index":r["pushed_point_index"],"base_degree":r["field_degree"],"source_degrees":ds,"source_points":pts,"fiber_sum_even":"zero","fiber_sum_odd":"zero","individual_images_complete":pts.len()==1}));
    }
    assert_eq!(totals, [1760, 1880, 2760, 1370, 2180]);
    let data = json!({"schema":"koblitz.round14/o24-v1","records":records,"source_degrees":totals,"source_point_counts":counts,"whole_deck_closed_points":whole,"split_fiber_individual_images_open":split,"all25_closed_supports_verified":true,"multiplicities_all_one":true,"fiber_sums_both_zero":true,"individual_images_all_complete":split==0,"relation_rank_promoted":false,"bridge":"written simple norm,etale scalar cover,moduli/pairing and descent;not whole Lean modular geometry"});
    round02::save(dir, "o24-five-source-supports.json", &data);
    let lists = records.iter().enumerate().fold(
        [vec![], vec![], vec![], vec![], vec![]],
        |mut a, (_, r)| {
            let fi = r["function_index"].as_u64().unwrap() as usize;
            for d in r["source_degrees"].as_array().unwrap() {
                a[fi].push(d.as_u64().unwrap());
            }
            a
        },
    );
    let mut lean = String::from("import KoblitzRound14\nnamespace KoblitzRound14Supports\n");
    for fi in 0..5 {
        lean.push_str(&format!("def degrees{fi} : List Nat := [{}]\ntheorem degree{fi}_total : degrees{fi}.sum={} := by decide +kernel\ntheorem degree{fi}_count : degrees{fi}.length={} := by decide +kernel\n",lists[fi].iter().map(ToString::to_string).collect::<Vec<_>>().join(","),totals[fi],counts[fi]));
    }
    lean.push_str("end KoblitzRound14Supports\n");
    for fi in 0..5 {
        for n in ["total", "count"] {
            lean.push_str(&format!(
                "#print axioms KoblitzRound14Supports.degree{fi}_{n}\n"
            ));
        }
    }
    let path = dir.join("KoblitzRound14Supports.lean");
    assert!(!path.exists());
    std::fs::write(path, lean).unwrap();
    println!(
        "O24:degrees{totals:?},counts{counts:?},whole{whole},split individual images open{split}"
    );
}
