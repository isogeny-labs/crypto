fn singular_polynomials(text: &str) -> [Vec<u64>; 2] {
    let mut out = [Vec::new(), Vec::new()];
    for k in 0..2 {
        let prefix = format!("POLYNOMIAL kernel={k} degree=4 index=0 coefficients ");
        let l = text.lines().find(|l| l.starts_with(&prefix)).unwrap();
        out[k] = l[prefix.len()..]
            .split_whitespace()
            .map(|a| a.parse().unwrap())
            .collect();
        assert_eq!(out[k].len(), 330)
    }
    out
}
fn quartic_hessian(q: &[u64], x: &[u64; 8]) -> usize {
    let es = monomials(4);
    let mut rows = vec![vec![0; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            for (e, &a) in es.iter().zip(q) {
                if e[i] == 0 {
                    continue;
                }
                let mut d = *e;
                let c = a * (d[i] as u64) % P;
                d[i] -= 1;
                if d[j] == 0 {
                    continue;
                }
                let c = c * (d[j] as u64) % P;
                d[j] -= 1;
                rows[i][j] = (rows[i][j] + c * mono(&d, x)) % P
            }
        }
    }
    pivots_fp(rows).len()
}
fn check_image_node(q: &[u64], x: &[u64; 8]) {
    let m = monomials(4);
    assert_eq!(dot_fp(q, &row_at(&m, x)), 0);
    for i in 0..8 {
        assert_eq!(dot_fp(q, &derivative_row(&m, x, i)), 0)
    }
}
pub fn run_geometry() {
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(600));
        eprintln!("CAP aggregate600seconds");
        std::process::exit(124)
    });
    let args: Vec<_> = std::env::args().collect();
    let (c, refc, _d1, ks) = load_inputs(&args);
    let f = &c.field;
    let j = JC::new(c.field);
    jet_controls(&j);
    smooth_controls();
    let raw = fs::read_to_string(&args[4]).unwrap();
    let quartics = singular_polynomials(&raw);
    let mut alpha = [Vec::new(), Vec::new()];
    let atext = fs::read_to_string(&args[3]).unwrap();
    for l in atext.lines().filter(|l| l.starts_with("ALPHA ")) {
        let ns: Vec<u64> = l
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(ns.len(), 15);
        let k = ns[0] as usize;
        assert!(k < 2);
        assert_eq!(ns[1] as usize, alpha[k].len());
        let a = std::array::from_fn(|i| ns[i + 2]);
        assert!(a.iter().all(|&v| v < P));
        alpha[k].push(a)
    }
    assert!(alpha.iter().all(|v| v.len() == 27));
    let hs: [Vec<H>; 2] =
        std::array::from_fn(|k| ks[k].iter().map(|p| construct(&c, p).unwrap()).collect());
    let neg: [Vec<usize>; 2] = std::array::from_fn(|k| {
        ks[k]
            .iter()
            .map(|p| ks[k].iter().position(|q| *q == c.neg(p)).unwrap())
            .collect()
    });
    let mut found = None;
    for (i, (x, y)) in hec_native_ic::instance::factor_base(&refc)
        .into_iter()
        .take(16)
        .enumerate()
    {
        let r = refc.place(x, y);
        let t = refc.mul(&r, 1028893);
        let nonzero = !refc.is_zero(&t);
        println!("TORSION_ATTEMPT index={i} x={x} y={y} nonzero={nonzero}");
        if nonzero {
            assert!(refc.is_zero(&refc.mul(&t, 2)));
            found = Some(t);
            break;
        }
    }
    let t = found.expect("CAP rational2torsion");
    println!("SOURCE_TORSION {t:?}");
    assert_eq!(t.u.deg(), 3);
    assert_eq!(t.v, Poly::zero());
    let td = Div {
        u: Pol::embed(&t.u),
        v: Pol::embed(&t.v),
    };
    assert!(squarefree(&td.u, f));
    let w0 = c.f.exact(&td.u, f);
    let wi = inverse(&w0, &td.u, f).expect("ramified torsion chart");
    let u0 = JP::embed(&td.u, &j);
    let w = JP::embed(&w0, &j);
    let vin = JP(vec![j.var(0), j.var(1), j.var(2)]);
    let v2 = vin.mul(&vin, &j);
    let u2 = v2.neg(&j).mul(&JP::embed(&wi, &j), &j).rem(&u0, &j);
    let w2 = v2.neg(&j).sub(&u2.mul(&w, &j), &j).exact(&u0, &j);
    let u4 = u2
        .mul(&w2, &j)
        .neg(&j)
        .mul(&JP::embed(&wi, &j), &j)
        .rem(&u0, &j);
    let u = u0.add(&u2, &j).add(&u4, &j);
    assert!(JP::embed(&c.f, &j).sub(&v2, &j).rem(&u, &j).0.is_empty());
    println!("LOCAL_CURVE_JET degree=4 coefficients=35 pass=true");
    let chars: Vec<Vec<E>> = (0..27)
        .map(|a| {
            (0..27)
                .map(|p| {
                    let a = digits(a);
                    let p = digits(p);
                    f.pow(
                        base(19),
                        ((0..3).map(|i| a[i] * p[i]).sum::<usize>() % 3) as u128,
                    )
                })
                .collect()
        })
        .collect();
    let mut z = Z;
    z[1] = 1;
    let mut pointcounts = Vec::new();
    let mut scalar_checks = 0;
    let mut node_ranks = Vec::new();
    for k in 0..2 {
        let mut funcs = Vec::new();
        let mut scalar = Vec::new();
        for i in 0..27 {
            let n = neg[k][i];
            let v = evaluate_jet(&ks[k][n], &hs[k][n], &u, &vin, &j);
            let a = evaluate(&c, &ks[k][n], &hs[k][n], &td).expect("scalar chart unsupported");
            assert_eq!(v.0[0], a);
            scalar_checks += 1;
            funcs.push(v);
            scalar.push(a)
        }
        let projection = |a: usize| {
            funcs.iter().enumerate().fold(j.z(), |out, (i, v)| {
                j.add(&out, &j.scale(v, f.mul(alpha[k][i], chars[a][i])))
            })
        };
        let ph0 = projection(0);
        let ph1 = projection(1);
        let ph2 = projection(2);
        let product = j.mul(&ph1, &ph2);
        let mut sections = vec![j.scale(&j.mul(&ph0, &ph0), base(13))];
        for power in 0..7 {
            sections.push(j.trace(&j.scale(&product, f.pow(z, power))))
        }
        let direct = projections(&scalar, &alpha[k], &chars, f);
        let dprod = f.mul(direct[1], direct[2]);
        let mut ds = vec![f.mul(base(13), f.mul(direct[0], direct[0]))];
        for power in 0..7 {
            ds.push(field_trace(f.mul(f.pow(z, power), dprod), f))
        }
        for i in 0..8 {
            assert_eq!(sections[i].0[0], ds[i]);
            for (s, e) in sections[i].0.iter().zip(&j.es) {
                assert!(s[1..].iter().all(|&a| a == 0));
                if e.iter().sum::<u8>() % 2 == 1 {
                    assert_eq!(*s, Z)
                }
            }
        }
        let coords: [u64; 8] = std::array::from_fn(|i| sections[i].0[0][0]);
        assert!(coords.iter().any(|&a| a != 0));
        println!("NODE kernel={k} coordinates={coords:?}");
        check_image_node(&quartics[k], &coords);
        let hr = quartic_hessian(&quartics[k], &coords);
        node_ranks.push(hr);
        println!("NODE_HESSIAN kernel={k} rank={hr}");
        let mut controls = 0;
        for line in raw
            .lines()
            .filter(|l| l.starts_with(&format!("POINT kernel={k} ")) && l.contains("holdout=true"))
        {
            let tail = line.split("values=[").nth(1).unwrap().trim_end_matches(']');
            let v: Vec<u64> = tail.split(',').map(|s| s.trim().parse().unwrap()).collect();
            let x: [u64; 8] = v.try_into().unwrap();
            check_image_node(&quartics[k], &x);
            println!(
                "ORDINARY_HESSIAN kernel={k} index={controls} rank={}",
                quartic_hessian(&quartics[k], &x)
            );
            controls += 1;
            if controls == 4 {
                break;
            }
        }
        assert_eq!(controls, 4);
        let low: Vec<usize> =
            j.es.iter()
                .enumerate()
                .filter(|(_, e)| e.iter().sum::<u8>() == 0 || e.iter().sum::<u8>() == 2)
                .map(|(i, _)| i)
                .collect();
        assert_eq!(low.len(), 7);
        let matrix: Vec<Vec<u64>> = low
            .iter()
            .map(|&at| sections.iter().map(|s| s.0[at][0]).collect())
            .collect();
        let ns = nullspace_fp(&matrix, 8);
        println!("GAMMA00 kernel={k} nullity={}", ns.len());
        assert_eq!(ns.len(), 1, "Gamma00 rank mismatch");
        let hyper = &ns[0];
        println!("HYPERPLANE kernel={k} coefficients={hyper:?}");
        let section = sections
            .iter()
            .zip(hyper)
            .fold(j.z(), |out, (s, &a)| j.add(&out, &j.scale(s, base(a))));
        for (i, e) in j.es.iter().enumerate() {
            if e.iter().sum::<u8>() < 4 {
                assert_eq!(section.0[i], Z)
            }
        }
        let mut q: Vec<u64> =
            j.es.iter()
                .enumerate()
                .filter(|(_, e)| e.iter().sum::<u8>() == 4)
                .map(|(i, _)| section.0[i][0])
                .collect();
        assert_eq!(q.len(), 15);
        let inv = refc
            .fp
            .inv(*q.iter().find(|&&a| a != 0).expect("zero quartic jet"));
        for a in &mut q {
            *a = refc.fp.mul(*a, inv)
        }
        println!(
            "PLANE_QUARTIC kernel={k} exponents={:?} coefficients={q:?}",
            ternary_exponents()
        );
        for i in 0..8 {
            for (at, e) in j.es.iter().enumerate() {
                println!(
                    "SECTION_JET kernel={k} section={i} exponents={e:?} coefficient={}",
                    sections[i].0[at][0]
                )
            }
        }
        let smooth = smooth_check(&q).expect("smoothness cap");
        println!("CANDIDATE kernel={k} smooth={smooth} jacobian_identified=false");
        assert!(smooth, "singular quartic jet");
        let count = count_points(&q);
        pointcounts.push(count);
        println!(
            "POINT_COUNT kernel={k} p=127 count={count} source_count=129 trace_sign_candidate={}",
            if count == 129 {
                "same"
            } else if count == 127 {
                "opposite"
            } else {
                "neither"
            }
        );
    }
    println!("TERMINAL kernels=2 jet_degree=4 scalar_weil_checks={scalar_checks} local_curve_identity=true odd_coefficients_zero=true gamma00_ranks=[7,7] node_hessian_ranks={node_ranks:?} plane_quartics_smooth=true pointcounts={pointcounts:?} jacobian_identified=false pass=true");
}
