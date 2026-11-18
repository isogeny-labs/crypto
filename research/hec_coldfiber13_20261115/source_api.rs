pub struct Source {
    c: Curve,
    refc: jac::Curve,
    d1: jac::Div,
    b: jac::Div,
    ks: Vec<Div>,
    hs: Vec<H>,
    neg: Vec<usize>,
    alpha: Vec<E>,
    chars: Vec<Vec<E>>,
}
impl Source {
    pub fn new() -> Self {
        let setup = super::setup::dir();
        super::setup::verify_model();
        let args = vec![
            "alignment".into(),
            setup.join("KERNELS.txt").to_string_lossy().into_owned(),
            setup.join("PUBLIC.txt").to_string_lossy().into_owned(),
        ];
        let (c, refc, d1, all) = load_inputs(&args);
        let ks = all[1].clone();
        let hs = ks.iter().map(|p| construct(&c, p).unwrap()).collect();
        let neg = ks
            .iter()
            .map(|p| ks.iter().position(|q| *q == c.neg(p)).unwrap())
            .collect();
        let mut alpha = Vec::new();
        for l in std::fs::read_to_string(setup.join("ALPHA.txt"))
            .unwrap()
            .lines()
        {
            let n: Vec<u64> = l
                .split_whitespace()
                .skip(1)
                .map(|x| x.parse().unwrap())
                .collect();
            assert_eq!(n[0], 1);
            assert_eq!(n[1] as usize, alpha.len());
            assert_eq!(n.len(), 15);
            alpha.push(std::array::from_fn(|i| n[i + 2]));
        }
        assert_eq!(alpha.len(), 27);
        assert_eq!(alpha[0], O);
        let zeta = (2..P).find(|a| refc.fp.pow(*a, 3) == 1).unwrap();
        let chars = (0..3)
            .map(|a| {
                (0..27)
                    .map(|p| {
                        let a = digits(a);
                        let p = digits(p);
                        base(refc.fp.pow(
                            zeta,
                            ((0..3).map(|i| a[i] * p[i]).sum::<usize>() % 3) as u64,
                        ))
                    })
                    .collect()
            })
            .collect();
        let read_poly = |name: &str| {
            Poly::from_coeffs(
                &std::fs::read_to_string(setup.join(name))
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<_>>(),
            )
        };
        let b = jac::Div {
            u: read_poly("B-U.txt"),
            v: read_poly("B-V.txt"),
        };
        assert!(refc.is_valid(&b) && !refc.is_zero(&b));
        assert!(refc.is_zero(&refc.mul(&b, 1028893)));
        Self {
            c,
            b,
            refc,
            d1,
            ks,
            hs,
            neg,
            alpha,
            chars,
        }
    }
    pub fn n(&self, n: u64) -> jac::Div {
        self.refc.mul(&self.d1, n)
    }
    pub fn b(&self) -> jac::Div {
        self.b
    }
    pub fn add(&self, a: &jac::Div, b: &jac::Div) -> jac::Div {
        self.refc.add(a, b)
    }
    pub fn neg(&self, a: &jac::Div) -> jac::Div {
        self.refc.neg(a)
    }
    pub fn eval(&self, x: &jac::Div) -> Option<[u64; 8]> {
        assert!(self.refc.is_valid(x));
        let x = Div {
            u: Pol::embed(&x.u),
            v: Pol::embed(&x.v),
        };
        let vals: Option<Vec<_>> = (0..27)
            .map(|i| evaluate(&self.c, &self.ks[self.neg[i]], &self.hs[self.neg[i]], &x))
            .collect();
        let ph = projections(&vals?, &self.alpha, &self.chars, &self.c.field);
        let f = &self.c.field;
        let prod = f.mul(ph[1], ph[2]);
        let mut z = Z;
        z[1] = 1;
        let mut out = [0; 8];
        let a = f.mul(base(13), f.mul(ph[0], ph[0]));
        assert!(a[1..].iter().all(|x| *x == 0));
        out[0] = a[0];
        for i in 0..7 {
            let a = field_trace(f.mul(f.pow(z, i as u128), prod), f);
            assert!(a[1..].iter().all(|x| *x == 0));
            out[i + 1] = a[0];
        }
        out.iter().any(|x| *x != 0).then_some(out)
    }
}
