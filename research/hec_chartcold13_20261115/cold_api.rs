pub fn run_coldprep() {
    let args: Vec<_> = std::env::args().collect();
    let input = &args[1];
    let dir = std::path::Path::new(&args[2]);
    assert!(!dir.exists());
    let (refc, d1, b, ell) = cold_input(input);
    assert_eq!(ell, ELL);
    let l = cold_weil(&refc, ell);
    let field = Field::new();
    assert_eq!(field.q % 4, 3);
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
    let _ = cold_geometry(&c, &refc, &d1, ks, &hs, &neg, &alpha, &chars, ell, &j);
    let pts = fast_points();
    assert_eq!(pts.len() as i64, P as i64 + 1 + l[1], "fresh_Serre_sign");
    let hc = cold_hc();
    let qf = cold_qf();
    std::fs::create_dir(dir).unwrap();
    let store = |name: &str, x: &[u64]| {
        std::fs::write(
            dir.join(name),
            x.iter().map(u64::to_string).collect::<Vec<_>>().join(" ") + "\n",
        )
        .unwrap()
    };
    store("HC.txt", &hc);
    store("QF.txt", &qf);
    store("B-U.txt", &b.u.c[..b.u.n]);
    store("B-V.txt", &b.v.c[..b.v.n]);
    let n = l.iter().sum::<i64>();
    let mut public = format!("INPUT 3 {P} 0 {n} {ell}");
    for p in [refc.f, d1.u, d1.v] {
        public.push_str(&format!(" {}", p.n));
        for a in &p.c[..p.n] {
            public.push_str(&format!(" {a}"));
        }
    }
    public.push('\n');
    std::fs::write(dir.join("PUBLIC.txt"), public).unwrap();
    let mut text = format!("FIELD {P} 13 {}\n", c.field.c);
    for k in 0..2 {
        for (i, p) in all[k].iter().enumerate() {
            text.push_str(&format!("POINT {k} {i}"));
            for p in [&p.u, &p.v] {
                text.push_str(&format!(" {}", p.0.len()));
                for a in &p.0 {
                    for v in a {
                        text.push_str(&format!(" {v}"));
                    }
                }
            }
            text.push('\n');
        }
    }
    std::fs::write(dir.join("KERNELS.txt"), text).unwrap();
    let mut text = String::new();
    for (i, a) in alpha.iter().enumerate() {
        text.push_str(&format!("ALPHA 1 {i}"));
        for v in a {
            text.push_str(&format!(" {v}"));
        }
        text.push('\n');
    }
    std::fs::write(dir.join("ALPHA.txt"), text).unwrap();
    println!("COLD_SETUP_PASS fresh_weil=true fresh_kernel_groups=2 fresh_kernel_points=54 fresh_alpha=27 fresh_geometry=true target_points={} HC={hc:?} QF={qf:?} source_N={n} old_map_scans_called=false",pts.len());
}
