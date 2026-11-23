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
            let mut q = refc.mul(&refc.place(x, y), ell);
            if !refc.is_zero(&q) {
                let mut depth=0;
                while !refc.is_zero(&refc.mul(&q,2)) {q=refc.mul(&q,2);depth+=1;assert!(depth<=2);}
                println!("LOCAL_2TORSION x={x} projection_depth={depth}");
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
    assert!(crate::target::is_smooth(&qf, P));
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
    println!("GEOMETRY registered_kernel=runtime quadratic_rows={} quadratic_rank={rank} gamma00_rank=7 HC={hc:?} QF={qf:?} smooth=true",rows.len());
    (td, u, v)
}
