struct Mapper<'a> {
    c: &'a Curve,
    refc: &'a jac::Curve,
    td: &'a Div,
    u: &'a JP,
    v: &'a JP,
    j: &'a JC,
    ks: &'a [Div],
    hs: &'a [H],
    neg: &'a [usize],
    alpha: &'a [E],
    chars: &'a [Vec<E>],
    pts: &'a [[u64; 3]],
    tj: &'a super::target::QuarticJacobian,
}
impl Mapper<'_> {
    fn map(&self, input: &jac::Div, label: &str) -> super::target::QClass {
        assert!(self.refc.is_valid(input));
        assert!(self.refc.is_zero(&self.refc.mul(input, 1028893)));
        if self.refc.is_zero(input) {
            println!("MAP label={label} zero=true trials=0");
            return self.tj.zero();
        }
        let mut x = self.refc.zero();
        let mut regular = 0;
        let mut zeros = 0;
        let mut ex: HashMap<String, usize> = HashMap::new();
        for n in 1..=8192u64 {
            x = self.refc.add(&x, input);
            let xd = Div {
                u: Pol::embed(&x.u),
                v: Pol::embed(&x.v),
            };
            let Some(sc) = section_scalar(
                self.c, self.ks, self.hs, self.neg, self.alpha, self.chars, &xd,
            ) else {
                *ex.entry("scalar_chart".into()).or_default() += 1;
                continue;
            };
            regular += 1;
            if scalar_g(&sc.1, &self.c.field) != Z {
                continue;
            }
            zeros += 1;
            let (ux, vx) = match translated(self.c, self.td, self.u, self.v, &xd, self.j) {
                Ok(v) => v,
                Err(e) => {
                    *ex.entry(e.into()).or_default() += 1;
                    continue;
                }
            };
            let g = section_jet(
                self.ks, self.hs, self.neg, self.alpha, self.chars, &ux, &vx, self.j, &sc,
            );
            let d = match decode(&g, self.j, self.pts) {
                Ok(d) => d,
                Err(e) => {
                    *ex.entry(e.into()).or_default() += 1;
                    continue;
                }
            };
            // Lambda fixed by the earlier source-jet gate; no fit to these inputs.
            let pair = oriented(&d, 10);
            let q = super::target::difference(self.tj, pair[0], pair[1], self.pts);
            let twice = self.tj.add_raw(&q, &q);
            let k = Fp::new(1028893).inv(2 * n);
            let image = super::target::scalar(self.tj, twice, k);
            assert_ne!(image, self.tj.zero());
            assert_eq!(
                super::target::scalar(self.tj, image, 1028893),
                self.tj.zero()
            );
            let mut ex: Vec<_> = ex.into_iter().collect();
            ex.sort();
            println!("MAP label={label} trials={n} regular={regular} zeros={zeros} exclusions={ex:?} pair={pair:?} r={} inv_2n={k} image={:?}",d.r,image.0);
            return image;
        }
        println!(
            "MAP_CAP label={label} trials=8192 regular={regular} zeros={zeros} exclusions={ex:?}"
        );
        panic!("public_input_transport_cap")
    }
}
pub fn run_publicmap() {
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(600));
        eprintln!("CAP aggregate600seconds");
        std::process::exit(124)
    });
    let args: Vec<_> = std::env::args().collect();
    let (c, refc, d1, ks0) = load_inputs(&args);
    let j = JC::new(c.field);
    jet_controls(&j);
    let ks = &ks0[1];
    let hs: Vec<_> = ks.iter().map(|p| construct(&c, p).unwrap()).collect();
    let neg: Vec<_> = ks
        .iter()
        .map(|p| ks.iter().position(|q| *q == c.neg(p)).unwrap())
        .collect();
    let atext = fs::read_to_string(&args[3]).unwrap();
    let alpha: Vec<E> = atext
        .lines()
        .filter(|l| l.starts_with("ALPHA 1 "))
        .map(|l| {
            let n: Vec<u64> = l
                .split_whitespace()
                .skip(1)
                .map(|s| s.parse().unwrap())
                .collect();
            std::array::from_fn(|i| n[i + 2])
        })
        .collect();
    assert_eq!(alpha.len(), 27);
    let chars = chars3(&c.field);
    let t = refc.mul(&refc.place(0, 44), 1028893);
    assert!(!refc.is_zero(&t));
    assert!(refc.is_zero(&refc.mul(&t, 2)));
    assert_eq!(t.v, Poly::zero());
    let td = Div {
        u: Pol::embed(&t.u),
        v: Pol::embed(&t.v),
    };
    let w0 = c.f.exact(&td.u, &c.field);
    let wi = inverse(&w0, &td.u, &c.field).unwrap();
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
    let pts = super::target::rational_points(&QF, P);
    let tj = super::target::QuarticJacobian::new(P, QF).unwrap();
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
    };
    let places = hec_native_ic::instance::factor_base(&refc);
    let (bx, by) = places[1];
    println!("FIXTURE_V2 B_place=({bx},{by})");
    let b = refc.mul(&refc.place(bx, by), 2);
    assert!(!refc.is_zero(&b));
    assert_ne!(b, d1);
    let inputs = [
        ("D1", d1),
        ("B", b),
        ("D1_plus_B", refc.add(&d1, &b)),
        ("twice_B", refc.mul(&b, 2)),
        ("minus_B", refc.neg(&b)),
        ("scalar_control", refc.mul(&d1, 1729)),
    ];
    let mut out = Vec::new();
    for (label, input) in inputs {
        println!("PUBLIC_INPUT label={label} u={:?} v={:?}", input.u, input.v);
        out.push(mapper.map(&input, label))
    }
    assert_eq!(
        out[2],
        tj.add_raw(&out[0], &out[1]),
        "public_sum_homomorphism"
    );
    assert_eq!(
        out[3],
        tj.add_raw(&out[1], &out[1]),
        "public_double_homomorphism"
    );
    assert_eq!(out[4], tj.neg_raw(&out[1]), "public_negation_homomorphism");
    assert_eq!(
        out[5],
        super::target::scalar(&tj, out[0], 1729),
        "withheld_scalar_control"
    );
    assert_eq!(mapper.map(&refc.zero(), "identity"), tj.zero());
    println!("PASS public_prime_subgroup_transport=true inputs=6 identity=1 addition=1 doubling=1 negation=1 withheld_scalar=1 dlp_scalar_passed_to_mapper=false");
}
