use std::cell::RefCell;
thread_local! {static LIVE:RefCell<(Option<[u64;8]>,Option<[u64;15]>,u64)>=const{RefCell::new((None,None,0))};}
fn cold_hc() -> [u64; 8] {
    LIVE.with(|v| v.borrow().0.expect("HC_not_built"))
}
fn cold_qf() -> [u64; 15] {
    LIVE.with(|v| v.borrow().1.expect("quartic_not_built"))
}
fn cold_zeta() -> u64 {
    LIVE.with(|v| {
        let z = v.borrow().2;
        assert_ne!(z, 0);
        z
    })
}
fn cold_input(path: &str) -> (jac::Curve, jac::Div, jac::Div, u64) {
    let raw = fs::read_to_string(path).unwrap();
    let mut it = raw.split_whitespace();
    assert_eq!(it.next(), Some("COLD_INPUT"));
    let ns: Vec<u64> = it.map(|s| s.parse().unwrap()).collect();
    assert_eq!(&ns[..2], &[3, P]);
    let ell = ns[2];
    assert!(ell > 2 && (2u64..).take_while(|d| d * d <= ell).all(|d| ell % d != 0));
    let mut at = 3;
    let mut ps = Vec::new();
    for _ in 0..5 {
        let n = ns[at] as usize;
        at += 1;
        ps.push(Poly::from_coeffs(&ns[at..at + n]));
        at += n
    }
    assert_eq!(at, ns.len());
    let refc = jac::Curve::new(Fp::new(P), ps[0]);
    let d = jac::Div { u: ps[1], v: ps[2] };
    let b = jac::Div { u: ps[3], v: ps[4] };
    for x in [&d, &b] {
        assert!(refc.is_valid(x) && !refc.is_zero(x));
        assert!(refc.is_zero(&refc.mul(x, ell)))
    }
    (refc, d, b, ell)
}
fn cold_weil(refc: &jac::Curve, ell: u64) -> [i64; 7] {
    let (n1, n2) = super::target::source_counts(&refc.f.c[..refc.f.n]);
    let a1 = n1 as i64 - P as i64 - 1;
    let numer = a1 * a1 - (P * P + 1 - n2) as i64;
    assert_eq!(numer % 2, 0);
    let a2 = numer / 2;
    let mut s = 1i64;
    while s * s < (P * P * P) as i64 {
        s += 1
    }
    let bound = 20 * s;
    let base = (P * P * P + 1) as i64 + (P * P + 1) as i64 * a1 + (P + 1) as i64 * a2;
    let lo = (base - bound + ell as i64 - 1) / ell as i64;
    let hi = (base + bound) / ell as i64;
    assert_eq!(lo, hi, "ambiguous_Jacobian_order");
    let n = lo * ell as i64;
    let a3 = n - base;
    let l = [
        1,
        a1,
        a2,
        a3,
        P as i64 * a2,
        (P * P) as i64 * a1,
        (P * P * P) as i64,
    ];
    println!("WEIL source_n1={n1} source_n2={n2} jacobian_order={n} cofactor={lo} L={l:?}");
    l
}
fn cold_primary(l: &[i64; 7]) -> [Vec<u64>; 2] {
    let fp = Fp::new(3);
    let chi = Poly::from_coeffs(
        &l.iter()
            .rev()
            .map(|a| a.rem_euclid(3) as u64)
            .collect::<Vec<_>>(),
    );
    for n in 0..27u64 {
        let f = Poly::from_coeffs(&[n % 3, n / 3 % 3, n / 9, 1]);
        if (0..3).any(|a| f.eval(a, &fp) == 0) {
            continue;
        }
        let (g, r) = chi.divrem(&f, &fp);
        if !r.is_zero() || g.deg() != 3 || f == g || (0..3).any(|a| g.eval(a, &fp) == 0) {
            continue;
        }
        let (d, s, t) = Poly::xgcd(&f, &g, &fp);
        assert_eq!(d, Poly::one());
        for q in [&f, &g] {
            assert_eq!(Poly::x().powmod(13, q, &fp), Poly::one())
        }
        let es = [t.mul(&g, &fp).rem(&chi, &fp), s.mul(&f, &fp).rem(&chi, &fp)];
        println!("PRIMARY factors={:?},{:?} projectors={:?}", f, g, es);
        return es.map(|a| a.c[..a.n].to_vec());
    }
    panic!("no_order13_primary_pair")
}
fn cold_kernels(c: &Curve, p: &Div) -> Vec<Div> {
    let q = c.frob(p);
    let r = c.frob(&q);
    let mut out = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                out.push(c.add(&c.add(&c.small(p, i), &c.small(&q, j)), &c.small(&r, k)))
            }
        }
    }
    out
}
fn cold_build_kernels(c: &Curve, l: [i64; 7], ell: u64) -> [Vec<Div>; 2] {
    let (n, h, v3) = super::orders_dynamic::extension_order(l, ell);
    let project = cold_primary(&l);
    let f = &c.field;
    let mut rng = hec_native_ic::rng::Rng::new(2026111501301);
    for attempt in 0..16 {
        let mut point = None;
        for _ in 0..32 {
            let x: E = std::array::from_fn(|_| rng.below(P));
            if f.frob(x) == x {
                continue;
            }
            let value = c.f.eval(x, f);
            let y = f.pow(value, (f.q + 1) / 4);
            if f.mul(y, y) == value {
                point = Some((x, y));
                break;
            }
        }
        let Some((x, y)) = point else {
            println!("KERNEL_ATTEMPT index={attempt} square_cap=true");
            continue;
        };
        let r = Div {
            u: Pol::new(vec![f.neg(x), O]),
            v: Pol::new(vec![y]),
        };
        assert!(c.valid(&r));
        assert!(c.is_zero(&c.scalar(&r, &n)));
        let mut q = c.scalar(&r, &h);
        if c.is_zero(&q) {
            continue;
        }
        let mut depth = 0;
        loop {
            let t = c.small(&q, 3);
            if c.is_zero(&t) {
                break;
            }
            q = t;
            depth += 1;
            assert!(depth <= v3)
        }
        let ps = project.each_ref().map(|e| c.operator(&q, e));
        if ps.iter().any(|p| c.is_zero(p)) {
            continue;
        }
        let ks = ps.each_ref().map(|p| cold_kernels(c, p));
        for set in &ks {
            let unique: std::collections::HashSet<_> = set.iter().cloned().collect();
            assert_eq!(unique.len(), 27);
            for p in set {
                assert!(c.valid(p) && c.is_zero(&c.small(p, 3)));
                assert!(unique.contains(&c.frob(p)))
            }
        }
        println!("KERNEL_ATTEMPT index={attempt} depth={depth} v3={v3} pass=true");
        return ks;
    }
    panic!("kernel_cap")
}
fn cold_normalize(
    c: &Curve,
    ks: &[Div],
    hs: &[H],
    neg: &[usize],
    d1: &jac::Div,
    refc: &jac::Curve,
) -> Vec<E> {
    let f = &c.field;
    for n in 1..=32 {
        let d = refc.mul(d1, n);
        let x = Div {
            u: Pol::embed(&d.u),
            v: Pol::embed(&d.v),
        };
        let (Ok(v), Ok(mv)) = (
            values(c, ks, hs, neg, &x),
            values(c, ks, hs, neg, &c.neg(&x)),
        ) else {
            continue;
        };
        let mut alpha = Vec::new();
        let mut ok = true;
        for i in 0..27 {
            let shift = c.add(&x, &ks[i]);
            let p = neg[i];
            let Some(vi) = evaluate(c, &ks[p], &hs[p], &shift) else {
                ok = false;
                break;
            };
            let Some(vn) = evaluate(c, &ks[i], &hs[i], &shift) else {
                ok = false;
                break;
            };
            if vi == Z || vn == Z {
                ok = false;
                break;
            }
            let twice = neg[i];
            let cii = quotient(f.mul(v[i], vi), v[twice], f);
            let cp = f.mul(v[i], vn);
            let a3 = f.inv(f.mul(cii, cp));
            let a2 = quotient(quotient(v[neg[i]], mv[i], f), cp, f);
            let a = quotient(a3, a2, f);
            assert_eq!(f.pow(a, 2), a2);
            assert_eq!(f.pow(a, 3), a3);
            alpha.push(a)
        }
        if ok {
            assert_eq!(alpha.len(), 27);
            println!("NORMALIZATION sample_multiple={n} root_extraction=false");
            return alpha;
        }
    }
    panic!("normalization_sample_cap")
}
fn cold_local(c: &Curve, t: &Div, j: &JC) -> (JP, JP) {
    let w0 = c.f.exact(&t.u, &c.field);
    let wi = inverse(&w0, &t.u, &c.field).unwrap();
    let u0 = JP::embed(&t.u, j);
    let w = JP::embed(&w0, j);
    let v = JP(vec![j.var(0), j.var(1), j.var(2)]);
    let v2 = v.mul(&v, j);
    let u2 = v2.neg(j).mul(&JP::embed(&wi, j), j).rem(&u0, j);
    let w2 = v2.neg(j).sub(&u2.mul(&w, j), j).exact(&u0, j);
    let u4 = u2.mul(&w2, j).neg(j).mul(&JP::embed(&wi, j), j).rem(&u0, j);
    let u = u0.add(&u2, j).add(&u4, j);
    assert!(JP::embed(&c.f, j).sub(&v2, j).rem(&u, j).0.is_empty());
    (u, v)
}
fn cold_geometry(
    c: &Curve,
    refc: &jac::Curve,
    d1: &jac::Div,
    ks: &[Div],
    hs: &[H],
    neg: &[usize],
    alpha: &[E],
    chars: &[Vec<E>],
    ell: u64,
    j: &JC,
) -> (Div, JP, JP) {
    let mut t = None;
    for x in 0..P {
        if let Some(y) = refc.fp.sqrt(refc.f.eval(x, &refc.fp)) {
            let y = y.min(P - y);
            let q = refc.mul(&refc.place(x, y), ell);
            if !refc.is_zero(&q) {
                assert!(refc.is_zero(&refc.mul(&q, 2)));
                t = Some(q);
                break;
            }
        }
    }
    let t = t.expect("no_rational2torsion");
    assert_eq!(t.u.deg(), 3);
    assert_eq!(t.v, Poly::zero());
    let td = Div {
        u: Pol::embed(&t.u),
        v: Pol::embed(&t.v),
    };
    let (u, v) = cold_local(c, &td, j);
    let vals: Vec<_> = (0..27)
        .map(|i| evaluate_jet(&ks[neg[i]], &hs[neg[i]], &u, &v, j))
        .collect();
    let ph: Vec<_> = (0..3)
        .map(|a| {
            (0..27).fold(j.z(), |o, i| {
                j.add(&o, &j.scale(&vals[i], j.f.mul(alpha[i], chars[a][i])))
            })
        })
        .collect();
    let prod = j.mul(&ph[1], &ph[2]);
    let mut z = Z;
    z[1] = 1;
    let mut ss = vec![j.scale(&j.mul(&ph[0], &ph[0]), base(13))];
    for k in 0..7 {
        ss.push(j.trace(&j.scale(&prod, j.f.pow(z, k))))
    }
    for s in &ss {
        for (a, e) in s.0.iter().zip(&j.es) {
            assert!(a[1..].iter().all(|a| *a == 0));
            if e.iter().sum::<u8>() % 2 == 1 {
                assert_eq!(*a, Z)
            }
        }
    }
    let low: Vec<_> =
        j.es.iter()
            .enumerate()
            .filter(|(_, e)| {
                let d = e.iter().sum::<u8>();
                d == 0 || d == 2
            })
            .map(|(i, _)| i)
            .collect();
    let matrix: Vec<_> = low
        .iter()
        .map(|i| ss.iter().map(|s| s.0[*i][0]).collect())
        .collect();
    let ns = nullspace_fp(&matrix, 8);
    assert_eq!(ns.len(), 1);
    let hc: [u64; 8] = ns[0].clone().try_into().unwrap();
    let section = ss
        .iter()
        .zip(hc)
        .fold(j.z(), |a, (s, h)| j.add(&a, &j.scale(s, base(h))));
    let mut q: Vec<_> =
        j.es.iter()
            .enumerate()
            .filter(|(_, e)| e.iter().sum::<u8>() == 4)
            .map(|(i, _)| section.0[i][0])
            .collect();
    assert_eq!(q.len(), 15);
    let inv = refc.fp.inv(q[14]);
    for v in &mut q {
        *v = *v * inv % P
    }
    let qf: [u64; 15] = q.into_iter().rev().collect::<Vec<_>>().try_into().unwrap();
    assert!(super::target::is_smooth(&qf, P));
    LIVE.with(|v| {
        let mut v = v.borrow_mut();
        v.0 = Some(hc);
        v.1 = Some(qf)
    });
    // Cheap global quadric exclusion; no cubic/Coble interpolation stage is used.
    let m2 = monomials(2);
    let mut rows = Vec::new();
    for n in 1..=64 {
        let x = refc.mul(d1, n);
        let x = Div {
            u: Pol::embed(&x.u),
            v: Pol::embed(&x.v),
        };
        if let Some((_, s)) = section_scalar(c, ks, hs, neg, alpha, chars, &x) {
            let s: [u64; 8] = s.map(|a| {
                assert!(a[1..].iter().all(|a| *a == 0));
                a[0]
            });
            rows.push(row_at(&m2, &s))
        }
    }
    let rank = pivots_fp(rows.clone()).len();
    assert_eq!(rank, 36);
    println!("GEOMETRY kernel=1 quadratic_rows={} quadratic_rank={rank} gamma00_rank=7 HC={hc:?} QF={qf:?} smooth=true",rows.len());
    (td, u, v)
}
fn cold_emit(ks: &[Vec<Div>; 2], alpha: &[E]) {
    for k in 0..2 {
        for (i, p) in ks[k].iter().enumerate() {
            print!("POINT {k} {i}");
            for pol in [&p.u, &p.v] {
                print!(" {}", pol.0.len());
                for a in &pol.0 {
                    for v in a {
                        print!(" {v}")
                    }
                }
            }
            println!()
        }
    }
    for (i, a) in alpha.iter().enumerate() {
        print!("ALPHA 1 {i}");
        for v in a {
            print!(" {v}")
        }
        println!()
    }
}
pub fn run_coldbuild() {
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(600));
        eprintln!("CAP aggregate600seconds");
        std::process::exit(124)
    });
    let args: Vec<_> = std::env::args().collect();
    let (refc, d1, b, ell) = cold_input(&args[1]);
    assert_eq!(ell, ELL);
    if args.get(2).is_some_and(|a| a == "--native") {
        let mut p = hec_native_ic::ic::IcParams::default();
        p.max_trials = 200000;
        p.reference_oracle = false;
        let r = hec_native_ic::ic::solve_dlp(&refc, &d1, &b, ell, &p);
        let k = r.k.expect("native_solve_cap");
        assert_eq!(refc.mul(&d1, k), b);
        println!(
            "NATIVE k={k} trials={} relations={} source_verified=true",
            r.trials, r.relations
        );
        return;
    }
    let l = cold_weil(&refc, ell);
    let field = Field::new();
    assert_eq!(field.q % 4, 3);
    println!("FIELD {P} 13 {}", field.c);
    let c = Curve {
        field,
        f: Pol::embed(&refc.f),
    };
    let all = cold_build_kernels(&c, l, ell);
    let ks = &all[1];
    let hs: Vec<_> = ks.iter().map(|p| construct(&c, p).unwrap()).collect();
    let neg: Vec<_> = ks
        .iter()
        .map(|p| ks.iter().position(|q| *q == c.neg(p)).unwrap())
        .collect();
    let alpha = cold_normalize(&c, ks, &hs, &neg, &d1, &refc);
    let root = refc.fp.sqrt(P - 3).unwrap();
    let z1 = refc.fp.mul(refc.fp.sub(root, 1), refc.fp.inv(2));
    let zeta = z1.min(refc.fp.neg(refc.fp.add(z1, 1)));
    assert_ne!(zeta, 1);
    assert_eq!(refc.fp.pow(zeta, 3), 1);
    LIVE.with(|v| v.borrow_mut().2 = zeta);
    let chars = chars3(&c.field);
    let j = JC::new(c.field);
    let (td, u, vin) = cold_geometry(&c, &refc, &d1, ks, &hs, &neg, &alpha, &chars, ell, &j);
    let pts = fast_points();
    let source_n1 = (P as i64 + 1 + l[1]) as u64;
    assert_eq!(pts.len() as u64, source_n1, "Serre_sign_mismatch");
    let tj = super::target::QuarticJacobian::with_points(P, cold_qf(), pts.clone()).unwrap();
    let mapper = Mapper {
        c: &c,
        refc: &refc,
        td: &td,
        u: &u,
        v: &vin,
        j: &j,
        ks,
        hs: &hs,
        neg: &neg,
        alpha: &alpha,
        chars: &chars,
        pts: &pts,
        tj: &tj,
        lambda: std::cell::Cell::new(None),
    };
    let logs = pair_relations(&pts, &[]);
    let g = mapper.map(&d1, "D1");
    let q = mapper.map(&b, "B");
    let gl = descend_known(&tj, g, &pts, &logs, "D1");
    let ql = descend_known(&tj, q, &pts, &logs, "B");
    let k = ql * Fp::new(ell).inv(gl) % ell;
    assert_eq!(refc.mul(&d1, k), b);
    assert_eq!(super::target::scalar(&tj, g, k), q);
    cold_emit(&all, &alpha);
    println!("COLD_RESULT k={k} source_verified=true destination_verified=true lambda={:?} loaded_prior_artifacts=false",mapper.lambda.get());
    println!("PASS cold_public_input_construction=true timing_comparison=false");
}
