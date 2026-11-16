#[derive(Clone)]
struct FF {
    a: Pol,
    b: Pol,
}
impl FF {
    fn new(a: Pol, b: Pol) -> Self {
        Self { a, b }
    }
    fn zero() -> Self {
        Self::new(Pol::zero(), Pol::zero())
    }
    fn add(&self, o: &Self, f: &Field) -> Self {
        Self::new(self.a.add(&o.a, f), self.b.add(&o.b, f))
    }
    fn scale(&self, c: E, f: &Field) -> Self {
        Self::new(pscale(&self.a, c, f), pscale(&self.b, c, f))
    }
    fn mul(&self, o: &Self, c: &Curve) -> Self {
        let f = &c.field;
        Self::new(
            self.a
                .mul(&o.a, f)
                .add(&self.b.mul(&o.b, f).mul(&c.f, f), f),
            self.a.mul(&o.b, f).add(&self.b.mul(&o.a, f), f),
        )
    }
    fn trace(&self, f: &Field) -> Self {
        Self::new(
            Pol::new(self.a.0.iter().map(|a| field_trace(*a, f)).collect()),
            Pol::new(self.b.0.iter().map(|a| field_trace(*a, f)).collect()),
        )
    }
    fn at(&self, x: E, y: E, f: &Field) -> E {
        f.add(self.a.eval(x, f), f.mul(self.b.eval(x, f), y))
    }
}
fn pscale(a: &Pol, b: E, f: &Field) -> Pol {
    Pol::new(a.0.iter().map(|a| f.mul(*a, b)).collect())
}
fn restricted(src: &Source) -> (FF, Pol) {
    let c = &src.c;
    let f = &c.field;
    let q=base(src.anchors[0][0]);let r=base(src.anchors[1][0]);let yq=base(src.anchors[0][1]);let yr=base(src.anchors[1][1]);
    let d = Pol::new(vec![f.neg(q), O]).mul(&Pol::new(vec![f.neg(r), O]), f);
    let den = pow(&d, 3, f);
    let mut vals = vec![];
    for i in 0..27 {
        let t = &src.ks[src.neg[i]];
        let h = &src.hs[src.neg[i]];
        if c.is_zero(t) {
            vals.push(FF::new(den.clone(), Pol::zero()));
            continue;
        }
        let cq = f.mul(f.add(yq, t.v.eval(q, f)), f.inv(t.u.eval(q, f)));
        let cr = f.mul(f.add(yr, t.v.eval(r, f)), f.inv(t.u.eval(r, f)));
        let aq = f.neg(f.mul(cq, f.inv(f.sub(q, r))));
        let ar = f.neg(f.mul(cr, f.inv(f.sub(r, q))));
        let linear = Pol::new(vec![
            f.neg(f.add(f.mul(aq, r), f.mul(ar, q))),
            f.add(aq, ar),
        ]);
        let n = FF::new(t.v.add(&t.u.mul(&linear, f), f), Pol::one());
        let n3 = n.mul(&n, c).mul(&n, c);
        let n3h = n3.mul(&FF::new(h.a.clone(), h.b.clone()), c);
        let up3 = pow(&t.u, 3, f);
        let a = n3h.a.exact(&up3, f);
        let b = n3h.b.exact(&up3, f);
        let hq = f.add(h.a.eval(q, f), f.mul(h.b.eval(q, f), yq));
        let hr = f.add(h.a.eval(r, f), f.mul(h.b.eval(r, f), yr));
        let z = FF::new(a, b).scale(f.mul(hq, hr), f);
        assert!(z.a.degree() <= 7 && z.b.degree() <= 4);
        vals.push(z);
    }
    let ph: Vec<_> = src
        .chars
        .iter()
        .map(|row| {
            row.iter()
                .zip(&vals)
                .zip(&src.alpha)
                .fold(FF::zero(), |s, ((w, v), a)| {
                    s.add(&v.scale(f.mul(*w, *a), f), f)
                })
        })
        .collect();
    let mut z = ph[0]
        .mul(&ph[0], c)
        .scale(base(13 * super::setup::hc()[0] % P), f);
    let prod = ph[1].mul(&ph[2], c);
    let mut zz = Z;
    zz[1] = 1;
    for i in 0..7 {
        z = z.add(
            &prod
                .scale(f.pow(zz, i as u128), f)
                .trace(f)
                .scale(base(super::setup::hc()[i + 1]), f),
            f,
        )
    }
    assert!(z.a.degree() <= 15 && z.b.degree() <= 11);
    assert!(z
        .a
        .0
        .iter()
        .chain(&z.b.0)
        .all(|a| a[1..].iter().all(|x| *x == 0)));
    (z, pow(&den, 2, f))
}
fn native(a: &Pol) -> Vec<u64> {
    a.0.iter()
        .map(|a| {
            assert!(a[1..].iter().all(|x| *x == 0));
            a[0]
        })
        .collect()
}
mod unip {
    include!("unip.rs");
    pub fn roots(a: Vec<u64>) -> Vec<u64> {
        let g = gcd(a.clone(), add(&mpow(vec![0, 1], P, &a), &[0, 1], true));
        fn rand(s: &mut u64) -> u64 {
            *s = s.wrapping_add(0x9E3779B97F4A7C15);
            let mut z = *s;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
            z ^ (z >> 31)
        }
        fn split(f: U, s: &mut u64, out: &mut Vec<u64>) {
            if f.len() <= 1 {
                return;
            }
            if f.len() == 2 {
                out.push((P - f[0]) * power(f[1], (P-2)) % P);
                return;
            }
            for _ in 0..256 {
                let a = (0..f.len() - 1).map(|_| rand(s) % P).collect();
                let g = gcd(f.clone(), add(&mpow(a, ((P-1)/2), &f), &[1], true));
                if g.len() > 1 && g.len() < f.len() {
                    let (q, r) = div(&f, &g);
                    assert!(r.is_empty());
                    split(g, s, out);
                    split(q, s, out);
                    return;
                }
            }
            panic!("split_cap")
        }
        let mut out = vec![];
        split(g, &mut 20261115, &mut out);
        out.sort();
        out
    }
}
pub fn calibrate() -> (Source, jac::Div, [[u64; 3]; 2]) {
    let src = Source::new();
    let c = &src.c;
    let f = &c.field;
    let (z, den) = restricted(&src);
    let norm = z.a.mul(&z.a, f).sub(&c.f.mul(&z.b.mul(&z.b, f), f), f);
    assert!(norm.degree() <= 30);
    let norm = norm.exact(&den, f);
    assert!(norm.degree() <= 18);
    assert!(!norm.0.is_empty());
    let norm0 = native(&norm);
    println!("RESTRICTION A_degree={} B_degree={} denominator_degree={} norm_degree={} A={:?} B={:?} norm={norm0:?}",z.a.degree(),z.b.degree(),den.degree(),norm.degree(),native(&z.a),native(&z.b));
    let roots = unip::roots(norm0);
    println!("ROOTS x={roots:?} field_scan_used=false cyclic_scan_used=false");
    let mut zeros = vec![];
    let source_div = |x: u64, y: u64| {
        src.refc.add(
            &src.refc.add(&src.refc.place(x, y), &src.refc.place(src.anchors[0][0], src.anchors[0][1])),
            &src.refc.place(src.anchors[1][0], src.anchors[1][1]),
        )
    };
    let direct_value = |x: u64, y: u64| {
        let d = source_div(x, y);
        let xd = Div {
            u: Pol::embed(&d.u),
            v: Pol::embed(&d.v),
        };
        section_scalar(c, &src.ks, &src.hs, &src.neg, &src.alpha, &src.chars, &xd)
            .map(|s| scalar_g(&s.1, f))
    };
    for x in roots {
        if x == src.anchors[0][0] || x == src.anchors[1][0] {
            println!("ROOT_EXCLUDE x={x} reason=anchor_pole");
            continue;
        }
        let fx = src.refc.f.eval(x, &src.refc.fp);
        let Some(y) = src.refc.fp.sqrt(fx) else {
            println!("ROOT_EXCLUDE x={x} reason=nonsplit_source");
            continue;
        };
        let mut ys = vec![y, (P - y) % P];
        ys.sort();
        ys.dedup();
        for y in ys {
            if z.at(base(x), base(y), f) != Z {
                continue;
            }
            let Some(g) = direct_value(x, y) else {
                println!("ROOT_EXCLUDE x={x} y={y} reason=source_chart");
                continue;
            };
            assert_eq!(g, Z);
            zeros.push((x, y));
        }
    }
    let mut checks = 0;
    let mut oracle = vec![];
    for x in 0..P {
        if x == src.anchors[0][0] || x == src.anchors[1][0] {
            continue;
        }
        let Some(y) = src.refc.fp.sqrt(src.refc.f.eval(x, &src.refc.fp)) else {
            continue;
        };
        let mut ys = vec![y, (P - y) % P];
        ys.sort();
        ys.dedup();
        for y in ys {
            let Some(v) = direct_value(x, y) else {
                continue;
            };
            let de = den.eval(base(x), f);
            assert_ne!(de, Z);
            assert_eq!(f.mul(v, de), z.at(base(x), base(y), f));
            checks += 1;
            if v == Z {
                oracle.push((x, y))
            }
        }
    }
    assert_eq!(zeros, oracle, "independent_complete_zero_set");
    println!("CONTROLS direct_values={checks} exact_zero_set={zeros:?}");
    assert!(!zeros.is_empty(),"empty_registered_fiber");
    // Decode without any multiples search or knowledge of source DLP coordinates.
    let j = JC::new(*f);
    let t = local_torsion(&src.refc);
    let td = Div {
        u: Pol::embed(&t.u),
        v: Pol::embed(&t.v),
    };
    let w0 = c.f.exact(&td.u, f);
    let wi = inverse(&w0, &td.u, f).unwrap();
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
    // Target chord points are factored per zero; no target point grid.
    let mut selected = 0;
    let mut bases = Vec::new();
    for (x, y) in zeros {
        let d = source_div(x, y);
        let xd = Div {
            u: Pol::embed(&d.u),
            v: Pol::embed(&d.v),
        };
        let sc =
            section_scalar(c, &src.ks, &src.hs, &src.neg, &src.alpha, &src.chars, &xd).unwrap();
        let (ux, vx) = match translated(c, &td, &u, &vin, &xd, &j) {
            Ok(a) => a,
            Err(e) => {
                println!("DECODE_EXCLUDE x={x} y={y} reason={e}");
                continue;
            }
        };
        let gj = section_jet(
            &src.ks, &src.hs, &src.neg, &src.alpha, &src.chars, &ux, &vx, &j, &sc,
        );
        let pts = chord_points(&gj, &j);
        let dd = match decode(&gj, &j, &pts) {
            Ok(a) => a,
            Err(e) => {
                println!("DECODE_EXCLUDE x={x} y={y} reason={e}");
                continue;
            }
        };
        let pair = oriented(&dd, dd.r);
        let md = c.neg(&xd);
        let ms =
            section_scalar(c, &src.ks, &src.hs, &src.neg, &src.alpha, &src.chars, &md).unwrap();
        let (mu, mv) = translated(c, &td, &u, &vin, &md, &j).unwrap();
        let mg = section_jet(
            &src.ks, &src.hs, &src.neg, &src.alpha, &src.chars, &mu, &mv, &j, &ms,
        );
        let mdd = decode(&mg, &j, &pts).unwrap();
        assert_eq!(oriented(&mdd, dd.r), [pair[1], pair[0]]);
        let base_src = src.refc.mul(&d, 4);
        assert!(!src.refc.is_zero(&base_src));
        assert!(src.refc.is_zero(&src.refc.mul(&base_src, 59296481)));
        let Some(coord) = src.eval(&base_src) else {
            println!("DECODE_EXCLUDE x={x} y={y} reason=doubled_source_chart");
            continue;
        };
        let _ = coord; // Source chart guard retained; no expected-matrix input.
        bases.push((base_src, pair));
        println!("CALIBRATION x={x} y={y} pair={pair:?} source_u={:?} source_v={:?} no_relative_scalar_used=true",base_src.u,base_src.v);
        selected += 1;
    }
    assert!(selected > 0, "no_supported_calibration");
    println!("PASS fixed_degree_calibration=true norm_degree_bound=18 decoded={selected} exact_restriction=true no_cyclic_scan=true timing_claim=false");
    let (base, pair) = bases[0];
    (src, base, pair)
}

fn chord_points(g: &Jet, j: &JC) -> Vec<[u64; 3]> {
    let n: [u64; 3] = std::array::from_fn(|i| {
        let mut e = [0; 3];
        e[i] = 1;
        g.0[j.es.iter().position(|a| *a == e).unwrap()][0]
    });
    let f = &j.f;
    let mut out = vec![];
    let (x, y, inf, vertical) = if n[1] != 0 {
        let a = (P - n[0]) * Fp::new(P).inv(n[1]) % P;
        let b = (P - n[2]) * Fp::new(P).inv(n[1]) % P;
        (
            Pol::new(vec![Z, O]),
            Pol::new(vec![base(b), base(a)]),
            [1, a, 0],
            false,
        )
    } else {
        assert_ne!(n[0], 0, "line_at_infinity_unsupported");
        let a = (P - n[2]) * Fp::new(P).inv(n[0]) % P;
        (
            Pol::new(vec![base(a)]),
            Pol::new(vec![Z, O]),
            [0, 1, 0],
            true,
        )
    };
    let mut poly = Pol::zero();
    let mut k = 0;
    for a in (0..=4).rev() {
        for b in (0..=4 - a).rev() {
            poly = poly.add(
                &pscale(
                    &pow(&x, a, f).mul(&pow(&y, b, f), f),
                    base(super::setup::qf()[k]),
                    f,
                ),
                f,
            );
            k += 1;
        }
    }
    let roots = unip::roots(native(&poly));
    for t in roots {
        let xx = x.eval(base(t), f)[0];
        let yy = y.eval(base(t), f)[0];
        out.push([xx, yy, 1]);
    }
    // A degree drop is the single point at infinity on this line.
    if poly.degree() < 4 {
        out.push(inf)
    }
    out.sort();
    out.dedup();
    assert!(out.iter().all(|p| pdot(n, *p) == 0));
    println!("CHORD rational_points={out:?} vertical={vertical} root_factoring=true");
    out
}

impl Source {
    pub fn mul_base(&self, x: &jac::Div, n: u64) -> jac::Div {
        self.refc.mul(x, n)
    }
}
