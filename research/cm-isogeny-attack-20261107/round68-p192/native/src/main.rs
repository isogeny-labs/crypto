#![allow(dead_code, unused_imports)]
include!("helpers.rs");
#[derive(Clone)]
struct Base<const P: i64, const D: i64> {
    baby: Vec<Pt<P, D>>,
    giants: Vec<Pt<P, D>>,
    b: u64,
    r: u64,
}
fn base<const P: i64, const D: i64>(t: Pt<P, D>, a: i64, r: u64) -> Base<P, D> {
    let mut s = 1;
    while (s + 1) * (s + 1) <= r {
        s += 1
    }
    let b = (s / 2).max(1);
    let m = 2 * b + 1;
    let n = r.div_ceil(m);
    let mut baby = vec![None, t];
    for j in 2..=b {
        baby.push(add(baby[(j - 1) as usize], t, a))
    }
    let step = add(add(baby[b as usize], baby[b as usize], a), t, a);
    let mut giants = vec![baby[b as usize]];
    for i in 1..n {
        giants.push(add(giants[(i - 1) as usize], step, a))
    }
    Base { baby, giants, b, r }
}
fn hk<const P: i64, const D: i64>(u: Pt<P, D>, v: Pt<P, D>, a: i64, b: i64) -> (F<P, D>, F<P, D>) {
    let (x, y) = u.unwrap();
    let (t, z) = v.unwrap();
    let den = (x - t) * (x - t);
    assert_ne!(den, F::c(0));
    {
        let iz = den.inv();
        (
            (t * x * x + (F::c(a) + t * t) * x + F::c(a) * t + F::c(2 * b)) * iz,
            F::c(2) * y * z * iz,
        )
    }
}
fn upd<const P: i64, const D: i64>(v: [F<P, D>; 2], w: [F<P, D>; 2]) -> [F<P, D>; 2] {
    [v[0] * w[0], v[1] * w[0] + v[0] * w[1]]
}
fn jetdet<const P: i64, const D: i64>(mut a: Vec<Vec<[F<P, D>; 2]>>) -> ([F<P, D>; 2], u64) {
    let n = a.len();
    let mut v = [F::c(1), F::c(0)];
    let mut pivots = 0;
    for k in 0..n {
        let pick = (k..n).find_map(|i| (k..n).find(|&j| a[i][j][0] != F::c(0)).map(|j| (i, j)));
        let Some((i, j)) = pick else {
            if n - k == 1 {
                return (upd(v, a[k][k]), pivots);
            } else {
                return ([F::c(0), F::c(0)], pivots);
            }
        };
        if i != k {
            a.swap(i, k);
            v[0] = -v[0];
            v[1] = -v[1]
        }
        if j != k {
            for row in &mut a {
                row.swap(j, k)
            }
            v[0] = -v[0];
            v[1] = -v[1]
        }
        let p = a[k][k];
        assert_ne!(p[0], F::c(0));
        let z = p[0].inv();
        let inv = [z, -p[1] * z * z];
        pivots += 1;
        v = upd(v, p);
        for i in k + 1..n {
            let q = upd(a[i][k], inv);
            for j in k..n {
                let t = upd(q, a[k][j]);
                a[i][j] = [a[i][j][0] - t[0], a[i][j][1] - t[1]]
            }
        }
    }
    (v, pivots)
}
fn red<const P: i64, const D: i64>(v: Vec<F<P, D>>, h: &[F<P, D>]) -> Vec<F<P, D>> {
    divide(&v, h).1
}
fn interp<const P: i64, const D: i64>(
    xs: &[F<P, D>],
    ys: &[F<P, D>],
    h: &[F<P, D>],
) -> Vec<F<P, D>> {
    let mut out = vec![F::c(0)];
    for (&x, &y) in xs.iter().zip(ys) {
        let l = rd(h, x);
        let c = y / ev(&l, x);
        pa(&mut out, &l, c)
    }
    trim(out)
}
struct Alg<const P: i64, const D: i64> {
    h: Vec<F<P, D>>,
    beta: Vec<F<P, D>>,
    gamma: Vec<F<P, D>>,
    hq: Vec<F<P, D>>,
    cache_counts: [u64; 3],
    tc: Vec<Vec<F<P, D>>>,
    jc: Vec<Vec<F<P, D>>>,
    tensor_counts: [u64; 3],
}
fn tensor_polys<const P: i64, const D: i64>(
    h: &[F<P, D>],
    beta: &[F<P, D>],
    a: i64,
    b: i64,
) -> (Vec<Vec<F<P, D>>>, Vec<Vec<F<P, D>>>) {
    let t = vec![F::c(0), F::c(1)];
    let t2 = pm(&t, &t);
    let v = beta.to_vec();
    let v2 = pm(&v, &v);
    let dg = [t2.clone(), scaled(&t, F::c(-2)), vec![F::c(1)]];
    let dq = [v2.clone(), scaled(&v, F::c(-2)), vec![F::c(1)]];
    let ng = [
        sum(&scaled(&t, F::c(a)), &[F::c(2 * b)], F::c(1)),
        sum(&t2, &[F::c(a)], F::c(1)),
        t.clone(),
    ];
    let nq = [
        sum(&scaled(&v, F::c(a)), &[F::c(2 * b)], F::c(1)),
        sum(&v2, &[F::c(a)], F::c(1)),
        v.clone(),
    ];
    let fg = [
        sum(&[F::c(a) * F::c(a)], &t, F::c(-4 * b)),
        sum(&scaled(&t, F::c(-2 * a)), &[F::c(-4 * b)], F::c(1)),
        t2,
    ];
    let fq = [
        sum(&[F::c(a) * F::c(a)], &v, F::c(-4 * b)),
        sum(&scaled(&v, F::c(-2 * a)), &[F::c(-4 * b)], F::c(1)),
        v2,
    ];
    let mut tc = vec![];
    let mut jc = vec![];
    for i in 0..3 {
        for j in 0..3 {
            let mut c = sum(&pm(&fg[i], &dq[j]), &pm(&dg[i], &fq[j]), F::c(1));
            pa(&mut c, &pm(&ng[i], &nq[j]), F::c(-2));
            tc.push(red(c, h));
            jc.push(red(
                scaled(
                    &sum(&pm(&ng[i], &dq[j]), &pm(&dg[i], &nq[j]), F::c(-1)),
                    F::c(2),
                ),
                h,
            ));
        }
    }
    (tc, jc)
}
fn algebra<const P: i64, const D: i64>(
    g: &Base<P, D>,
    q: &Base<P, D>,
    a: i64,
    b: i64,
) -> Alg<P, D> {
    let xs: Vec<_> = g.baby.iter().skip(1).map(|x| x.unwrap().0).collect();
    let ys: Vec<_> = q.baby.iter().skip(1).map(|x| x.unwrap().0).collect();
    let wz: Vec<_> = g
        .baby
        .iter()
        .zip(&q.baby)
        .skip(1)
        .map(|(x, y)| x.unwrap().1 * y.unwrap().1)
        .collect();
    let mut h = vec![F::c(1)];
    for &x in &xs {
        h = pm(&h, &[-x, F::c(1)])
    }
    let beta = interp(&xs, &ys, &h);
    let gamma = interp(&xs, &wz, &h);
    for i in 0..xs.len() {
        assert_eq!(ev(&beta, xs[i]), ys[i]);
        assert_eq!(ev(&gamma, xs[i]), wz[i])
    }
    let st = counts();
    let mut hq = vec![F::c(1)];
    for &x in &ys {
        hq = pm(&hq, &[-x, F::c(1)])
    }
    let cache_counts = diff(st, counts());
    let st = counts();
    let (tc, jc) = tensor_polys(&h, &beta, a, b);
    let tensor_counts = diff(st, counts());
    Alg {
        h,
        beta,
        gamma,
        hq,
        cache_counts,
        tc,
        jc,
        tensor_counts,
    }
}
fn normjet_dense<const P: i64, const D: i64>(
    al: &Alg<P, D>,
    e0: Vec<F<P, D>>,
    e1: Vec<F<P, D>>,
) -> ([F<P, D>; 2], u64) {
    let n = al.h.len() - 1;
    let mut matrix = vec![vec![[F::c(0), F::c(0)]; n]; n];
    for j in 0..n {
        let mut t = vec![F::c(0); j + 1];
        t[j] = F::c(1);
        let a = red(pm(&e0, &t), &al.h);
        let b = red(pm(&e1, &t), &al.h);
        for i in 0..n {
            matrix[i][j] = [
                a.get(i).copied().unwrap_or(F::c(0)),
                b.get(i).copied().unwrap_or(F::c(0)),
            ]
        }
    }
    jetdet(matrix)
}
fn algebra_block<const P: i64, const D: i64>(
    al: &Alg<P, D>,
    ap: Pt<P, D>,
    bp: Pt<P, D>,
    a: i64,
    b: i64,
) -> ([F<P, D>; 2], u64) {
    if std::env::args().any(|s| s == "--direct-numerator") {
        return direct_block(al, ap, bp, a, b);
    }
    let (x, y) = ap.unwrap();
    let (z, w) = bp.unwrap();
    let xp = [F::c(1), x, x * x];
    let zp = [F::c(1), z, z * z];
    let mut m0 = vec![F::c(0)];
    let mut m1 = vec![F::c(0)];
    for i in 0..3 {
        for j in 0..3 {
            let q = xp[i] * zp[j];
            pa(&mut m0, &al.tc[3 * i + j], q);
            pa(&mut m1, &al.jc[3 * i + j], q)
        }
    }
    pa(&mut m0, &al.gamma, F::c(8) * y * w);
    let (v, _) = field_normjet(&al.h, &trim(m0), &trim(m1));
    let d = ev(&al.h, x) * ev(&al.hq, z);
    let d = d * d;
    assert_ne!(d, F::c(0));
    ([v[0] / d, v[1] / d], 0)
}
fn direct_block<const P: i64, const D: i64>(
    al: &Alg<P, D>,
    ap: Pt<P, D>,
    bp: Pt<P, D>,
    a: i64,
    b: i64,
) -> ([F<P, D>; 2], u64) {
    if std::env::args().any(|s| s == "--expanded-numerator") {
        return expanded_block(al, ap, bp, a, b);
    }
    let h = &al.h;
    let (x, y) = ap.unwrap();
    let (z, w) = bp.unwrap();
    let be = &al.beta;
    let dg = pm(&[-x, F::c(1)], &[-x, F::c(1)]);
    let zz = sum(be, &[-z], F::c(1));
    let dq = pm(&zz, &zz);
    let ng = vec![F::c(a) * x + F::c(2 * b), x * x + F::c(a), x];
    let nq = sum(
        &sum(
            &[F::c(a) * z + F::c(2 * b)],
            &scaled(be, z * z + F::c(a)),
            F::c(1),
        ),
        &scaled(&pm(be, be), z),
        F::c(1),
    );
    let pg = vec![
        F::c(a) * F::c(a) - F::c(4 * b) * x,
        F::c(-2 * a) * x - F::c(4 * b),
        x * x,
    ];
    let pq = sum(
        &sum(
            &[F::c(a) * F::c(a) - F::c(4 * b) * z],
            &scaled(be, F::c(-2 * a) * z - F::c(4 * b)),
            F::c(1),
        ),
        &scaled(&pm(be, be), z * z),
        F::c(1),
    );
    let l = red(sum(&pm(&ng, &dq), &pm(&nq, &dg), F::c(-1)), h);
    let mut m = sum(&pm(&pg, &dq), &pm(&pq, &dg), F::c(1));
    pa(&mut m, &pm(&ng, &nq), F::c(-2));
    pa(&mut m, &al.gamma, F::c(8) * y * w);
    let m0 = red(m, h);
    let m1 = scaled(&l, F::c(2));
    let (v, _) = field_normjet(h, &m0, &m1);
    let d = ev(h, x) * ev(&al.hq, z);
    let d = d * d;
    assert_ne!(d, F::c(0));
    ([v[0] / d, v[1] / d], 0)
}
fn expanded_block<const P: i64, const D: i64>(
    al: &Alg<P, D>,
    ap: Pt<P, D>,
    bp: Pt<P, D>,
    a: i64,
    b: i64,
) -> ([F<P, D>; 2], u64) {
    let h = &al.h;
    let (x, y) = ap.unwrap();
    let (z, w) = bp.unwrap();
    let be = &al.beta;
    let dg = pm(&[-x, F::c(1)], &[-x, F::c(1)]);
    let dq = pm(&sum(be, &[-z], F::c(1)), &sum(be, &[-z], F::c(1)));
    let ng = vec![F::c(a) * x + F::c(2 * b), x * x + F::c(a), x];
    let nq = sum(
        &sum(
            &[F::c(a) * z + F::c(2 * b)],
            &scaled(be, z * z + F::c(a)),
            F::c(1),
        ),
        &scaled(&pm(be, be), z),
        F::c(1),
    );
    let l = red(sum(&pm(&ng, &dq), &pm(&nq, &dg), F::c(-1)), h);
    let den = red(pm(&dg, &dq), h);
    let ft = vec![F::c(b), F::c(a), F::c(0), F::c(1)];
    let fb = sum(&sum(&[F::c(b)], be, F::c(a)), &pm(&pm(be, be), be), F::c(1));
    let mut e = pm(&l, &l);
    pa(&mut e, &pm(&ft, &pm(&dq, &dq)), F::c(-4) * y * y);
    pa(&mut e, &pm(&fb, &pm(&dg, &dg)), F::c(-4) * w * w);
    pa(&mut e, &pm(&al.gamma, &den), F::c(8) * y * w);
    let e0 = red(e, h);
    let e1 = red(scaled(&pm(&l, &den), F::c(2)), h);
    let (v, _) = field_normjet(h, &e0, &e1);
    let d = if std::env::args().any(|s| s == "--legacy-denominator") {
        let d2 = red(pm(&den, &den), h);
        field_normjet(h, &d2, &[F::c(0)]).0[0]
    } else {
        let t = ev(h, x) * ev(&al.hq, z);
        let t = t * t;
        t * t
    };
    assert_ne!(d, F::c(0));
    ([v[0] / d, v[1] / d], 0)
}
fn paired<const P: i64, const D: i64>(
    g: &Base<P, D>,
    q: &Base<P, D>,
    p: Pt<P, D>,
    u: Pt<P, D>,
    a: i64,
    b: i64,
    al: &Alg<P, D>,
) -> ([F<P, D>; 2], Value) {
    let start = counts();
    let rr = add(p, u, a);
    let mut v = [F::c(1), F::c(0)];
    let mut pairs = 0;
    let mut singles = 0;
    let mut blocks = vec![];
    let mut alg_counts = [0u64; 3];
    let mut alg_pivots = 0;
    let mut complete_validation = [0u64; 3];
    let mut covered = vec![false; g.r as usize];
    for i in 0..g.giants.len() {
        let center = g.b + i as u64 * (2 * g.b + 1);
        let ap = add(p, g.giants[i], a);
        let bp = add(rr, q.giants[i], a);
        let direct_start = counts();
        let mut block = [F::c(1), F::c(0)];
        if center < g.r {
            covered[center as usize] = true;
            let d = ap.unwrap().0 - bp.unwrap().0;
            block = upd(block, [d, F::c(1)]);
            singles += 1
        }
        for j in 1..=g.b {
            let plus = center + j < g.r;
            let minus = center >= j && center - j < g.r;
            if !plus && !minus {
                continue;
            }
            let (hg, kg) = hk(ap, g.baby[j as usize], a, b);
            let (hq, kq) = hk(bp, q.baby[j as usize], a, b);
            let aa = hg - hq;
            let bb = kg - kq;
            if plus && minus {
                let e = [aa * aa - bb * bb, F::c(2) * aa];
                assert_ne!(e[0], aa * aa - (kg + kq) * (kg + kq));
                block = upd(block, e);
                pairs += 1;
                for n in [center - j, center + j] {
                    assert!(!covered[n as usize]);
                    covered[n as usize] = true
                }
            } else {
                let idx = if plus { center + j } else { center - j };
                let d = if plus { aa - bb } else { aa + bb };
                block = upd(block, [d, F::c(1)]);
                singles += 1;
                assert!(!covered[idx as usize]);
                covered[idx as usize] = true
            }
        }
        if center + g.b < g.r {
            let dc = diff(direct_start, counts());
            for z in 0..3 {
                complete_validation[z] += dc[z]
            }
            let st = counts();
            let (w, np) = algebra_block(al, ap, bp, a, b);
            let dc = diff(st, counts());
            for z in 0..3 {
                alg_counts[z] += dc[z]
            }
            alg_pivots += np;
            let d0 = ap.unwrap().0 - bp.unwrap().0;
            let chosen = upd([d0, F::c(1)], w);
            assert_eq!(block, chosen);
            block = chosen;
        }
        blocks.push(block.map(|x| x.code()));
        v = upd(v, block)
    }
    assert!(covered.iter().all(|&v| v));
    assert_eq!(2 * pairs + singles, g.r);
    let mut c = diff(start, counts());
    for z in 0..3 {
        c[z] -= complete_validation[z]
    }
    (
        v,
        json!({"selected_response_counts":c,"selected_complete_algebra_counts":alg_counts,"complete_direct_validation_counts":complete_validation,"algebra_unit_pivots":alg_pivots,"paired_factors":pairs,"single_factors":singles,"covered":g.r,"block_coefficients":blocks}),
    )
}
fn fullgraph<const P: i64, const D: i64>(g: Pt<P, D>, a: i64, r: u64) -> Vec<Pt<P, D>> {
    let mut v = vec![];
    let mut t = None;
    for _ in 0..r {
        v.push(t);
        t = add(t, g, a)
    }
    assert!(t.is_none());
    v
}
fn full<const P: i64, const D: i64>(
    gg: &[Pt<P, D>],
    qq: &[Pt<P, D>],
    p: Pt<P, D>,
    u: Pt<P, D>,
    a: i64,
) -> [F<P, D>; 2] {
    let rr = add(p, u, a);
    let mut ds = vec![];
    for (&g, &q) in gg.iter().zip(qq) {
        ds.push(add(p, g, a).unwrap().0 - add(rr, q, a).unwrap().0)
    }
    let n = ds.len();
    let mut pre = vec![F::c(1); n + 1];
    let mut post = pre.clone();
    for i in 0..n {
        pre[i + 1] = pre[i] * ds[i]
    }
    for i in (0..n).rev() {
        post[i] = post[i + 1] * ds[i]
    }
    [
        pre[n],
        (0..n).fold(F::c(0), |s, i| s + pre[i] * post[i + 1]),
    ]
}
fn decoder<const P: i64, const D: i64>(ys: &[F<P, D>], g: Pt<P, D>, a: i64, b: i64) -> Pt<P, D> {
    let mut v = vec![];
    for i in 0..P {
        for j in 0..P {
            let x = F::new(i, j);
            if x * x * x + F::c(a) * x + F::c(b) != ys[0] * ys[0] {
                continue;
            }
            let p = Some((x, ys[0]));
            if (0..7).all(|k| {
                add(p, mul(k, g, a), a)
                    .map(|(_, y)| y == ys[k as usize])
                    .unwrap_or(false)
            }) {
                v.push(p)
            }
        }
    }
    assert_eq!(v.len(), 1);
    v[0]
}
fn panel<const P: i64, const D: i64>(
    g: Pt<P, D>,
    p: Pt<P, D>,
    a: i64,
    b: i64,
    r: u64,
    ks: &[u64],
    ns: &[u64],
    reference: bool,
) -> (Value, Value) {
    assert_ne!(
        add(p, p, a),
        p.map(|(x, y)| (F::new(x.0, -x.1), F::new(y.0, -y.1)))
    );
    let st = counts();
    let bg = base(g, a, r);
    let gc = diff(st, counts());
    assert_eq!(gc[2], bg.b + bg.giants.len() as u64);
    let st = counts();
    let gg = fullgraph(g, a, r);
    let fg = diff(st, counts());
    let mut rows = vec![];
    let mut receipts = vec![];
    let mut setups = vec![];
    for &k in ks {
        let q = mul(k, g, a);
        let st = counts();
        let bq = base(q, a, r);
        let qc = diff(st, counts());
        assert_eq!(qc[2], bq.b + bq.giants.len() as u64);
        let st = counts();
        let al = algebra(&bg, &bq, a, b);
        let ac = diff(st, counts());
        let st = counts();
        let qq = fullgraph(q, a, r);
        let fc = diff(st, counts());
        let mut query = |shift: Pt<P, D>, u: Pt<P, D>| {
            let (v, c) = paired(&bg, &bq, shift, u, a, b, &al);
            let st = counts();
            let w = full(&gg, &qq, shift, u, a);
            let rc = diff(st, counts());
            assert_eq!(v, w);
            receipts.push(json!({"k_fixture":k,"paired":c,"full_reference_counts":rc}));
            if reference {
                w[1]
            } else {
                v[1]
            }
        };
        let cal = query(p, None);
        assert_ne!(cal, F::c(0));
        for &n in ns {
            let u = mul(n, g, a);
            let mut bs = vec![];
            let mut ys = vec![];
            for i in 0..7 {
                let s = add(p, mul(i, g, a), a);
                let v = query(s, u);
                assert_ne!(v, F::c(0));
                bs.push(v);
                ys.push(cal * p.unwrap().1 / v)
            }
            let st = counts();
            let found = decoder(&ys, g, a, b);
            let out = add(found, neg(p), a);
            let decode_counts = diff(st, counts());
            let inv = (1..r).find(|&v| (r + 1 - k) * v % r == 1).unwrap();
            assert_eq!(out, mul(inv, u, a));
            rows.push(json!({"k_fixture":k,"U_fixture":n,"B":bs.iter().map(|x|x.code()).collect::<Vec<_>>(),"Bcal":cal.code(),"output":pc(out),"decoder_counts":decode_counts}))
        }
        setups.push(json!({"k_fixture":k,"candidate_target_setup_counts":qc,"baby_algebra_setup_counts":std::array::from_fn::<_,3,_>(|i|ac[i]-al.cache_counts[i]-al.tensor_counts[i]),"H_Q_cache_setup_counts":al.cache_counts,"tensor_cache_setup_counts":al.tensor_counts,"tensor_coefficient_slots":al.tc.iter().chain(&al.jc).map(Vec::len).sum::<usize>(),"tensor_slot_bound":18*bg.b,"H_Q_coefficient_slots":al.hq.len(),"H_Q":al.hq.iter().map(|x|x.code()).collect::<Vec<_>>(),"reference_full_setup_counts":fc,"target_point_slots":bq.baby.len()+bq.giants.len(),"beta":al.beta.iter().map(|x|x.code()).collect::<Vec<_>>(),"gamma":al.gamma.iter().map(|x|x.code()).collect::<Vec<_>>()}));
    }
    (
        json!({"p":P,"D":D,"r":r,"a":a,"b":b,"G":pc(g),"shift":pc(p),"baby_width":bg.b,"block_width":2*bg.b+1,"giants":bg.giants.len(),"source_point_slots":bg.baby.len()+bg.giants.len(),"source_setup_counts":gc,"reference_source_setup_counts":fg,"H":algebra(&bg,&bg,a,b).h.iter().map(|x|x.code()).collect::<Vec<_>>(),"target_setups":setups,"rows":rows}),
        json!({"p":P,"r":r,"mode":if reference{"full_reference"}else{"paired_candidate"},"count_order":["quadratic_field_multiplications_including_inversion_powers","quadratic_field_inversions","group_add_calls_including_identity_and_doubling"],"queries":receipts}),
    )
}
include!("subresultant.rs");
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let reference = args.iter().any(|s| s == "--full-reference");
    if args.iter().any(|s| s == "--norm-controls") {
        let x = norm_controls::<11, 2>(Some((F::c(6), F::c(2))), 2, 7, 7);
        let y = norm_controls::<17, 3>(Some((F::c(5), F::c(1))), 2, 2, 19);
        let z = norm_controls::<211, 210>(Some((F::c(3), F::c(33))), 0, 7, 199);
        let out = json!({"all_checks_pass":true,"panels":[x,y,z],"scope":"unit/simple/multiple/zero first-coefficient controls"});
        if let Some(i) = args.iter().position(|s| s == "--check") {
            let v: Value = serde_json::from_slice(&fs::read(&args[i + 1]).unwrap()).unwrap();
            assert_eq!(out, v, "changed norm control")
        }
        println!("{}", serde_json::to_string_pretty(&out).unwrap());
        return;
    }

    let (x, cx) = panel::<11, 2>(
        Some((F::c(6), F::c(2))),
        mul(6, Some((F::c(0), F::new(0, 3))), 2),
        2,
        7,
        7,
        &[2, 3, 4, 5],
        &(0..7).collect::<Vec<_>>(),
        reference,
    );
    let (y, cy) = panel::<17, 3>(
        Some((F::c(5), F::c(1))),
        mul(6, Some((F::c(1), F::new(0, 8))), 2),
        2,
        2,
        19,
        &(2..18).collect::<Vec<_>>(),
        &(0..19).collect::<Vec<_>>(),
        reference,
    );
    let (z, cz) = panel::<211, 210>(
        Some((F::c(3), F::c(33))),
        Some((F::c(1), F::new(0, 25))),
        0,
        7,
        199,
        &[2, 3, 5, 7, 11, 17, 197],
        &[0, 1, 2, 3, 5, 17, 198],
        reference,
    );
    let out = json!({"all_checks_pass":true,"panels":[x,y,z],"scope":"complete B-only paired contraction and calibrated decoder; candidate point builder is square-root size, contraction costs retained"});
    if let Some(i) = args.iter().position(|s| s == "--check") {
        let v: Value = serde_json::from_slice(&fs::read(&args[i + 1]).unwrap()).unwrap();
        assert_eq!(out, v, "changed coefficient record")
    }
    eprintln!(
        "{}",
        json!({"panels":[cx,cy,cz],"whole_qualifier_counts":counts(),"norm_backend_stats":normstats(),"norm_backend_stat_order":["field_multiplications","field_inversions","dense_multiplications","dense_inversions","calls","unit","simple_common_root","multiple_common_roots","zero_perturbation"]})
    );
    println!("{}", serde_json::to_string_pretty(&out).unwrap())
}
