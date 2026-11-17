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
pub fn cold_build_kernels(c: &Curve, l: [i64; 7], ell: u64) -> [Vec<Div>; 2] {
    let (n, h, v3) = super::orders::extension_order(l, ell);
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
