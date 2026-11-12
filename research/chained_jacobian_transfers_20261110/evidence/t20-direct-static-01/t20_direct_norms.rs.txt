//! Direct four-cover norms under frozen protocol c05a75.
//! Setup uses the existing extension construction; runtime is Zp only.
use super::*;

const RAW_T20: [u64; 7] = [25, 3, 69, 63, 52, 62, 76];
const MONIC_T20: [u64; 7] = [100, 12, 74, 50, 6, 46, 1];

// Rational-function assembly over the existing polynomial engine.
#[derive(Clone, Debug)]
struct Fraction {
    n: Vec<ER>,
    d: Vec<ER>,
}
impl Fraction {
    fn new(f: &FpR, n: Vec<ER>, d: Vec<ER>) -> Self {
        assert!(!d.is_empty());
        if n.is_empty() {
            return Self {
                n,
                d: vec![f.one()],
            };
        }
        let gcd = poly::gcd(f, &n, &d);
        let (n, nr) = poly::divrem(f, &n, &gcd);
        let (d, dr) = poly::divrem(f, &d, &gcd);
        assert!(nr.is_empty() && dr.is_empty());
        let scale = f.inv(*d.last().unwrap());
        Self {
            n: poly::scale(f, &n, scale),
            d: poly::scale(f, &d, scale),
        }
    }
    fn polynomial(f: &FpR, n: Vec<ER>) -> Self {
        Self::new(f, n, vec![f.one()])
    }
    fn constant(f: &FpR, n: ER) -> Self {
        Self::polynomial(f, if n == f.zero() { vec![] } else { vec![n] })
    }
    fn add(&self, f: &FpR, b: &Self) -> Self {
        Self::new(
            f,
            poly::add(
                f,
                &poly::mul(f, &self.n, &b.d),
                &poly::mul(f, &b.n, &self.d),
            ),
            poly::mul(f, &self.d, &b.d),
        )
    }
    fn sub(&self, f: &FpR, b: &Self) -> Self {
        self.add(f, &b.scale(f, f.neg(f.one())))
    }
    fn mul(&self, f: &FpR, b: &Self) -> Self {
        Self::new(f, poly::mul(f, &self.n, &b.n), poly::mul(f, &self.d, &b.d))
    }
    fn div(&self, f: &FpR, b: &Self) -> Self {
        assert!(!b.n.is_empty());
        Self::new(f, poly::mul(f, &self.n, &b.d), poly::mul(f, &self.d, &b.n))
    }
    fn scale(&self, f: &FpR, c: ER) -> Self {
        Self::new(f, poly::scale(f, &self.n, c), self.d.clone())
    }
    fn square(&self, f: &FpR) -> Self {
        self.mul(f, self)
    }
}

#[derive(Clone, Debug)]
struct FpFunction {
    n: Vec<u64>,
    d: Vec<u64>,
}
impl FpFunction {
    fn descend(f: &FpR, q: &Fraction) -> Self {
        let project = |p: &[ER]| {
            p.iter()
                .map(|&c| f.project(c).expect("every function coefficient in F101"))
                .collect()
        };
        Self {
            n: project(&q.n),
            d: project(&q.d),
        }
    }
    fn extension(&self, f: &FpR) -> Fraction {
        Fraction::new(f, lift(f, &self.n), lift(f, &self.d))
    }
}

#[derive(Clone, Debug)]
struct BalancedInput {
    field: u64,
    coefficients: [u64; 7],
    u: Vec<u64>,
    v: Vec<u64>,
    plus: i64,
}
impl BalancedInput {
    fn check(&self) -> Result<(), &'static str> {
        if self.field != 101 || self.coefficients != MONIC_T20 {
            return Err("wrong normalized T20 field/model tag");
        }
        if self.u.is_empty()
            || self.u.len() > 3
            || self.u.last() != Some(&1)
            || self.v.len() >= self.u.len()
            || self.v.last() == Some(&0)
            || self.u.iter().chain(&self.v).any(|&c| c >= 101)
            || self.plus < 0
            || self.plus + self.u.len() as i64 - 1 > 2
        {
            return Err("noncanonical balanced support/ordinate/infinity count");
        }
        let f = Zp::new(101);
        if !poly::rem(
            &f,
            &poly::sub(&f, &poly::mul(&f, &self.v, &self.v), &MONIC_T20.to_vec()),
            &self.u,
        )
        .is_empty()
        {
            return Err("balanced support does not lie on monic T20");
        }
        Ok(())
    }
    fn key(&self) -> (Vec<u64>, Vec<u64>, i64) {
        (self.u.clone(), self.v.clone(), self.plus)
    }
}

#[derive(Clone, Debug)]
struct FpCover {
    curve: Curve<u64>,
    x: FpFunction,
    s: FpFunction,
    infinity: Pt<u64>,
}
impl FpCover {
    // This complete evaluation path contains only F101 arithmetic. It accepts
    // no extension context, root cache, class index, or logarithm label.
    fn evaluate(
        &self,
        input: &BalancedInput,
        omit_infinity: bool,
    ) -> Result<Pt<u64>, &'static str> {
        input.check()?;
        let f = Zp::new(101);
        let degree = input.u.len() - 1;
        let poles = poly::gcd(&f, &input.u, &self.x.d);
        let (u, remainder) = poly::divrem(&f, &input.u, &poles);
        assert!(remainder.is_empty());
        let finite = if u.len() == 1 {
            Pt::Inf
        } else {
            let xi = poly::invmod(&f, &self.x.d, &u).expect("remaining X denominator is a unit");
            let yi = poly::invmod(&f, &self.s.d, &u).expect("remaining Y denominator is a unit");
            let x = poly::rem(&f, &poly::mul(&f, &self.x.n, &xi), &u);
            let y = poly::rem(
                &f,
                &poly::scale(
                    &f,
                    &poly::mul(&f, &poly::mul(&f, &input.v, &self.s.n), &yi),
                    50,
                ),
                &u,
            );
            let a = x.first().copied().unwrap_or(0);
            let b = x.get(1).copied().unwrap_or(0);
            let c = y.first().copied().unwrap_or(0);
            let d = y.get(1).copied().unwrap_or(0);
            if u.len() == 2 {
                Pt::Aff(a, c)
            } else if b == 0 {
                if d == 0 {
                    curve::pmul(&f, &self.curve, &Pt::Aff(a, c), 2)
                } else {
                    assert_eq!(
                        f.mul(2, c),
                        f.mul(u[1], d),
                        "opposite-image relation includes repeated jets"
                    );
                    Pt::Inf
                }
            } else {
                let m = f.div(d, b);
                let rx = f.sub(f.sq(m), f.sub(f.mul(2, a), f.mul(u[1], b)));
                Pt::Aff(rx, f.sub(f.mul(m, f.sub(a, rx)), c))
            }
        };
        assert!(curve::on_curve(&f, &self.curve, &finite));
        if omit_infinity {
            return Ok(finite);
        }
        let coefficient = 2 * input.plus + degree as i64 - 2;
        let correction = curve::pmul(
            &f,
            &self.curve,
            &self.infinity,
            coefficient.unsigned_abs() as u128,
        );
        let correction = if coefficient < 0 {
            curve::neg(&f, &correction)
        } else {
            correction
        };
        let out = curve::padd(&f, &self.curve, &finite, &correction);
        assert!(curve::on_curve(&f, &self.curve, &out));
        Ok(out)
    }
}

fn function_point(f: &FpR, x: &Fraction, s: &Fraction, point: EvenPoint) -> Pt<ER> {
    match point {
        EvenPoint::Finite(z, y) => {
            let xd = poly::eval(f, &x.d, z);
            if xd == f.zero() {
                return Pt::Inf;
            }
            let sd = poly::eval(f, &s.d, z);
            assert_ne!(sd, f.zero(), "finite X has a regular Y coordinate");
            Pt::Aff(
                f.div(poly::eval(f, &x.n, z), xd),
                f.div(f.mul(y, poly::eval(f, &s.n, z)), sd),
            )
        }
        EvenPoint::Infinity(rho) => {
            if x.n.len() > x.d.len() {
                return Pt::Inf;
            }
            let xx = if x.n.len() == x.d.len() {
                f.div(*x.n.last().unwrap(), *x.d.last().unwrap())
            } else {
                f.zero()
            };
            let exponent = s.n.len() as isize - s.d.len() as isize + 3;
            let yy = if s.n.is_empty() || exponent < 0 {
                f.zero()
            } else {
                assert_eq!(exponent, 0, "no Y pole above a finite infinity image");
                f.mul(rho, f.div(*s.n.last().unwrap(), *s.d.last().unwrap()))
            };
            Pt::Aff(xx, yy)
        }
    }
}

fn project_point(f: &FpR, point: Pt<ER>) -> Pt<u64> {
    match point {
        Pt::Inf => Pt::Inf,
        Pt::Aff(x, y) => Pt::Aff(
            f.project(x).expect("rational elliptic X"),
            f.project(y).expect("rational elliptic Y"),
        ),
    }
}

fn check_function_equation(f: &FpR, x: &Fraction, s: &Fraction, e: Curve<u64>) {
    let source = Fraction::polynomial(f, lift(f, &RAW_T20));
    let rhs = x
        .square(f)
        .mul(f, x)
        .add(f, &x.scale(f, f.from_u64(e.a)))
        .add(f, &Fraction::constant(f, f.from_u64(e.b)));
    assert!(
        source.mul(f, &s.square(f)).sub(f, &rhs).n.is_empty(),
        "exact rational-function curve equation"
    );
}

fn check_fp_equation(cover: &FpCover) {
    let f = Zp::new(101);
    let sq = |p: &Vec<u64>| poly::mul(&f, p, p);
    let cube = |p: &Vec<u64>| poly::mul(&f, &sq(p), p);
    let rhs = poly::add(
        &f,
        &poly::add(
            &f,
            &cube(&cover.x.n),
            &poly::scale(
                &f,
                &poly::mul(&f, &cover.x.n, &sq(&cover.x.d)),
                cover.curve.a,
            ),
        ),
        &poly::scale(&f, &cube(&cover.x.d), cover.curve.b),
    );
    assert_eq!(
        poly::mul(
            &f,
            &poly::mul(&f, &RAW_T20.to_vec(), &sq(&cover.s.n)),
            &cube(&cover.x.d)
        ),
        poly::mul(&f, &sq(&cover.s.d), &rhs),
        "actual F101 coefficient identity"
    );
    assert_eq!(
        cover.x.n.len().max(cover.x.d.len()),
        5,
        "degree-four elliptic cover"
    );
    assert_eq!(poly::gcd(&f, &cover.x.n, &cover.x.d), vec![1]);
    assert_eq!(poly::gcd(&f, &cover.s.n, &cover.s.d), vec![1]);
}

struct Descent {
    covers: [FpCover; 2],
    raw: [(Fraction, Fraction); 2],
    translations: [Pt<ER>; 2],
}

fn derive_covers(f: &FpR, maps: &RichelotMaps<'_>, c20: &WholeContext<'_>) -> Descent {
    assert_eq!(maps.target.coefficients, RAW_T20);
    assert_eq!(maps.source.coefficients, c20.model.coefficients);
    assert_eq!(maps.source.chart.a, c20.chart.a);
    assert_eq!(c20.model.factors, [Curve::new(59, 27), Curve::new(44, 21)]);
    let factors = &maps.reverse_factors;
    let normalized = &maps.reverse_normalized;
    let u: [Vec<ER>; 3] = std::array::from_fn(|i| {
        poly::add(
            f,
            &poly::scale(f, &factors[0], normalized[0][i]),
            &poly::scale(f, &factors[1], normalized[1][i]),
        )
    });
    let a = Fraction::new(f, u[1].clone(), u[2].clone());
    let b = Fraction::new(f, u[0].clone(), u[2].clone());
    let z = vec![f.zero(), f.one()];
    let c: [Fraction; 4] = std::array::from_fn(|i| {
        let term = if i == 3 {
            vec![f.neg(normalized[0][2])]
        } else {
            let leading = poly::scale(f, &z, normalized[0][i]);
            if i == 0 {
                leading
            } else {
                poly::sub(f, &leading, &vec![normalized[0][i - 1]])
            }
        };
        Fraction::polynomial(f, poly::mul(f, &factors[0], &term))
    });
    let h0 = c[0]
        .sub(f, &b.mul(f, &c[2]))
        .add(f, &a.mul(f, &b).mul(f, &c[3]));
    let h1 = c[1]
        .sub(f, &a.mul(f, &c[2]))
        .add(f, &a.square(f).sub(f, &b).mul(f, &c[3]));
    let source = Fraction::polynomial(f, lift(f, &RAW_T20));
    let one = Fraction::constant(f, f.one());
    let two = f.from_u64(2);
    let infinity_fibre = point_fibre(
        f,
        &maps.target,
        &maps.source,
        factors,
        normalized,
        EvenPoint::Infinity(f.from_u64(50)),
        &mut FibreStats::default(),
    );
    let infinity_norm = c20
        .model
        .norm(
            f,
            &c20.chart,
            &c20.jac,
            &infinity_fibre,
            &mut RootCache::new(),
        )
        .map(|p| curve::neg(f, &p));
    let mut covers: [Option<FpCover>; 2] = [None, None];
    let mut raw: [Option<(Fraction, Fraction)>; 2] = [None, None];
    let mut translations = [Pt::Inf; 2];
    for (branch, cover) in c20.model.covers.iter().enumerate() {
        let (a, b, h0, h1) = if branch == 0 {
            (a.clone(), b.clone(), h0.clone(), h1.clone())
        } else {
            let aa = a.div(f, &b);
            let bb = one.div(f, &b);
            let hh0 = h0.mul(f, &aa).mul(f, &bb).sub(f, &h1.mul(f, &bb));
            let hh1 = h0.mul(f, &aa.square(f).sub(f, &bb)).sub(f, &h1.mul(f, &aa));
            (aa, bb, hh0, hh1)
        };
        let scale = f.from_u64(cover.scale);
        let alpha = f.mul(f.sq(scale), f.from_u64(cover.lead));
        let beta = f.mul(f.sq(scale), f.from_u64(cover.shift));
        let mu = f.mul(f.pow(scale, 3), f.from_u64(cover.lead));
        let m0 = h1.scale(f, f.neg(mu)).div(f, &a.scale(f, alpha));
        let x = m0
            .square(f)
            .div(f, &source)
            .sub(f, &a.square(f).sub(f, &b.scale(f, two)).scale(f, alpha))
            .sub(f, &Fraction::constant(f, f.mul(two, beta)));
        let s = m0
            .mul(
                f,
                &Fraction::constant(f, beta)
                    .sub(f, &b.scale(f, alpha))
                    .sub(f, &x),
            )
            .sub(f, &h0.scale(f, mu))
            .div(f, &source)
            .scale(f, f.neg(f.one()));
        let component = cover.destination;
        let e = c20.model.factors[component];
        check_function_equation(f, &x, &s, e);
        let d = infinity_norm[component];
        assert_eq!(
            function_point(f, &x, &s, EvenPoint::Infinity(f.from_u64(50))),
            d
        );
        let ext = extension_curve(f, e);
        let twice = project_point(f, curve::pmul(f, &ext, &d, 2));
        let half = curve::pmul(&Zp::new(101), &e, &twice, [55, 57][component]);
        let translation = curve::padd(f, &ext, &d, &curve::neg(f, &point_lift(f, half)));
        assert_eq!(curve::pmul(f, &ext, &translation, 2), Pt::Inf);
        let (new_x, new_s) = match translation {
            Pt::Inf => (x.clone(), s.clone()),
            Pt::Aff(e0, y0) => {
                assert_eq!(y0, f.zero());
                let difference = x.sub(f, &Fraction::constant(f, e0));
                let k = f.add(f.mul(f.from_u64(3), f.sq(e0)), ext.a);
                (
                    Fraction::constant(f, e0).add(f, &Fraction::constant(f, k).div(f, &difference)),
                    s.scale(f, f.neg(k)).div(f, &difference.square(f)),
                )
            }
        };
        check_function_equation(f, &new_x, &new_s, e);
        assert_eq!(
            function_point(f, &new_x, &new_s, EvenPoint::Infinity(f.from_u64(50))),
            point_lift(f, half)
        );
        assert_eq!(
            function_point(f, &new_x, &new_s, EvenPoint::Infinity(f.from_u64(51))),
            point_lift(f, curve::neg(&Zp::new(101), &half))
        );
        let out = FpCover {
            curve: e,
            x: FpFunction::descend(f, &new_x),
            s: FpFunction::descend(f, &new_s),
            infinity: half,
        };
        check_fp_equation(&out);
        println!("DERIVED T20cover{component}:x={:?}/{:?},s={:?}/{:?},infinity={half:?},translation={translation:?};exactFpcoefficients/degree4/clearedcurve", out.x.n, out.x.d, out.s.n, out.s.d);
        covers[component] = Some(out);
        raw[component] = Some((x, s));
        translations[component] = translation;
    }
    Descent {
        covers: covers.map(Option::unwrap),
        raw: raw.map(Option::unwrap),
        translations,
    }
}

fn balanced_from_class(
    f: &FpR,
    chart: &Chart,
    jac: &legacy::Hyp<'_, Flat<'_>>,
    d: &legacy::Div<ER>,
) -> BalancedInput {
    assert!(jac.is_valid(d));
    let mut out = BalancedInput {
        field: 101,
        coefficients: MONIC_T20,
        u: vec![1],
        v: vec![],
        plus: 1,
    };
    if *d == jac.identity() {
        return out;
    }
    assert_eq!(
        d.u.len(),
        3,
        "nonidentity rational effective degree2 cannot contain a degree6 branch"
    );
    if d.u[0] == f.zero() {
        let rho = f.div(d.v.first().copied().unwrap_or(f.zero()), f.sq(chart.l));
        assert!(rho == f.from_u64(50) || rho == f.from_u64(51));
        if d.u[1] == f.zero() {
            out.plus = if rho == f.from_u64(50) { 2 } else { 0 };
        } else {
            let w = f.neg(d.u[1]);
            let x = f.add(chart.a, f.div(chart.l, w));
            let y = f.div(f.mul(chart.l, poly::eval(f, &d.v, w)), f.pow(w, 3));
            out.u = vec![f.project(f.neg(x)).expect("rational finite support"), 1];
            let y = f
                .project(f.mul(y, f.from_u64(99)))
                .expect("rational normalized ordinate");
            out.v = if y == 0 { vec![] } else { vec![y] };
            out.plus = i64::from(rho == f.from_u64(50));
        }
    } else {
        let linear = vec![f.neg(chart.a), f.one()];
        let raw_u = poly::add(
            f,
            &poly::add(
                f,
                &vec![f.sq(chart.l)],
                &poly::scale(f, &linear, f.mul(d.u[1], chart.l)),
            ),
            &poly::scale(f, &poly::mul(f, &linear, &linear), d.u[0]),
        );
        let raw_u = poly::monic(f, &raw_u);
        let w = poly::scale(f, &poly::invmod(f, &linear, &raw_u).unwrap(), chart.l);
        let mut vv = vec![];
        let mut power = vec![f.one()];
        for &c in &d.v {
            vv = poly::add(f, &vv, &poly::scale(f, &power, c));
            power = poly::rem(f, &poly::mul(f, &power, &w), &raw_u);
        }
        let w3 = poly::rem(f, &poly::mul(f, &poly::mul(f, &w, &w), &w), &raw_u);
        let raw_v = poly::rem(
            f,
            &poly::scale(
                f,
                &poly::mul(f, &vv, &poly::invmod(f, &w3, &raw_u).unwrap()),
                f.mul(chart.l, f.from_u64(99)),
            ),
            &raw_u,
        );
        out.u = raw_u
            .iter()
            .map(|&c| f.project(c).expect("rational raw support"))
            .collect();
        out.v = raw_v
            .iter()
            .map(|&c| f.project(c).expect("rational raw ordinate"))
            .collect();
        out.plus = 0;
    }
    out.check().unwrap();
    let input = (
        lift(f, &out.u),
        poly::scale(f, &lift(f, &out.v), f.from_u64(50)),
        out.plus,
    );
    assert_eq!(
        chart.class(f, jac, &input),
        *d,
        "exact inverse projective Chart including signed infinity"
    );
    out
}

fn reference_norm(
    f: &FpR,
    maps: &mut RichelotMaps<'_>,
    c20: &WholeContext<'_>,
    d: &legacy::Div<ER>,
) -> [Pt<ER>; 2] {
    let back = maps.adjoint(f, &maps.target.tag(d)).unwrap();
    c20.model
        .norm(f, &c20.chart, &c20.jac, &back, &mut RootCache::new())
}

fn geometric_norm(
    f: &FpR,
    maps: &RichelotMaps<'_>,
    descent: &Descent,
    d: &legacy::Div<ER>,
    cache: &mut RootCache,
) -> [Pt<ER>; 2] {
    let points = original_support(
        f,
        &maps.target.coefficients,
        &maps.target.chart,
        &maps.target.jac,
        d,
        cache,
    );
    std::array::from_fn(|i| {
        let cover = &descent.covers[i];
        let x = cover.x.extension(f);
        let s = cover.s.extension(f);
        let e = extension_curve(f, cover.curve);
        let mut sum = Pt::Inf;
        for &point in &points {
            let p = function_point(f, &x, &s, point);
            assert!(curve::on_curve(f, &e, &p));
            sum = curve::padd(f, &e, &sum, &p);
        }
        let base = function_point(f, &x, &s, EvenPoint::Finite(maps.target.chart.a, f.zero()));
        curve::padd(
            f,
            &e,
            &sum,
            &curve::neg(f, &curve::pmul(f, &e, &base, points.len() as u128)),
        )
    })
}

fn geometric_adjoint(
    f: &FpR,
    maps: &mut RichelotMaps<'_>,
    c20: &WholeContext<'_>,
    p: [Pt<ER>; 2],
) -> legacy::Div<ER> {
    let first = c20.model.pull(f, &c20.chart, &c20.jac, 0, p[0]).0;
    let second = c20.model.pull(f, &c20.chart, &c20.jac, 1, p[1]).0;
    let sum = c20.jac.add(&first, &second);
    maps.forward(f, &maps.source.tag(&sum)).unwrap()
}

#[test]
fn t20_direct_norm_descent_and_geometric_controls() {
    let f = FpR::new(101, 12);
    let flat = Flat(&f);
    let mut maps = RichelotMaps::new(&f, &flat);
    let c20 = WholeContext::new(&f, &flat, models(&f).into_iter().nth(2).unwrap().1);
    let descent = derive_covers(&f, &maps, &c20);
    let mut points = curve_points(&f, &RAW_T20);
    assert_eq!(points.len(), 120);
    let branches = poly::roots(&f, &lift(&f, &RAW_T20), &mut Rng::new(0x5201));
    assert_eq!(branches.len(), 6);
    points.extend(branches.into_iter().map(|x| EvenPoint::Finite(x, f.zero())));
    let mut pole_points = 0;
    for cover in &descent.covers {
        for x in poly::roots(&f, &lift(&f, &cover.x.d), &mut Rng::new(0x5202)) {
            let y = f
                .sqrt(poly::eval(&f, &lift(&f, &RAW_T20), x))
                .expect("planned pole points in F101^12");
            for y in if y == f.zero() {
                vec![y]
            } else {
                vec![y, f.neg(y)]
            } {
                assert_eq!(
                    function_point(
                        &f,
                        &cover.x.extension(&f),
                        &cover.s.extension(&f),
                        EvenPoint::Finite(x, y)
                    ),
                    Pt::Inf
                );
                points.push(EvenPoint::Finite(x, y));
                pole_points += 1;
            }
        }
    }
    let mut cache = RootCache::new();
    let mut repeated_inputs = 0;
    for &point in &points {
        let d = maps.target.point_class(&f, point);
        let norm = geometric_norm(&f, &maps, &descent, &d, &mut cache);
        assert_eq!(norm, reference_norm(&f, &mut maps, &c20, &d));
        let back = geometric_adjoint(&f, &mut maps, &c20, norm);
        assert_eq!(back, maps.target.jac.mul(&d, 4));
        let doubled = maps.target.jac.mul(&d, 2);
        if doubled.u.len() == 3
            && poly::gcd(&f, &doubled.u, &poly::derivative(&f, &doubled.u)).len() > 1
        {
            repeated_inputs += 1;
        }
        assert_eq!(
            geometric_norm(&f, &maps, &descent, &doubled, &mut cache),
            std::array::from_fn(|i| curve::pmul(
                &f,
                &extension_curve(&f, c20.model.factors[i]),
                &norm[i],
                2
            ))
        );
        for i in 0..2 {
            let cover = &descent.covers[i];
            let e = extension_curve(&f, cover.curve);
            let raw = function_point(&f, &descent.raw[i].0, &descent.raw[i].1, point);
            let translated =
                function_point(&f, &cover.x.extension(&f), &cover.s.extension(&f), point);
            assert_eq!(
                translated,
                curve::padd(&f, &e, &raw, &curve::neg(&f, &descent.translations[i])),
                "coherent translation,not a false Jacobian mutant"
            );
            let pi = match point {
                EvenPoint::Finite(x, y) => EvenPoint::Finite(f.frobenius(x), f.frobenius(y)),
                EvenPoint::Infinity(r) => EvenPoint::Infinity(f.frobenius(r)),
            };
            let pi_image = match translated {
                Pt::Inf => Pt::Inf,
                Pt::Aff(x, y) => Pt::Aff(f.frobenius(x), f.frobenius(y)),
            };
            assert_eq!(
                function_point(&f, &cover.x.extension(&f), &cover.s.extension(&f), pi),
                pi_image
            );
        }
    }
    assert!(repeated_inputs > 0);
    let (groups, _) = actual_graph(&f, 0);
    let mut elliptic_models = vec![Curve::new(1, 46)];
    let first = construct_hop(&f, elliptic_models[0], 0, &mut elliptic_models);
    let second = construct_hop(&f, first.target, 0, &mut elliptic_models);
    let e2 = transported_group(&f, &transported_group(&f, &groups[0], &first), &second);
    let mut kernel = HashMap::new();
    for &p in &e2.points {
        for &q in &groups[1].points {
            let d = geometric_adjoint(&f, &mut maps, &c20, [p, q]);
            kernel.insert(class_key(&d), d);
        }
    }
    assert_eq!(kernel.len(), 16);
    let mut killed_two = 0;
    for d in kernel.values() {
        assert_eq!(maps.target.jac.mul(d, 4), maps.target.jac.identity());
        killed_two += usize::from(maps.target.jac.mul(d, 2) == maps.target.jac.identity());
        assert_eq!(
            geometric_norm(&f, &maps, &descent, d, &mut cache),
            [Pt::Inf; 2]
        );
        assert_eq!(reference_norm(&f, &mut maps, &c20, d), [Pt::Inf; 2]);
        for other in kernel.values() {
            assert!(kernel.contains_key(&class_key(&maps.target.jac.add(d, other))));
        }
    }
    assert_eq!(killed_two, 4);
    let identity = BalancedInput {
        field: 101,
        coefficients: MONIC_T20,
        u: vec![1],
        v: vec![],
        plus: 1,
    };
    let mut mutants = vec![];
    let mut d = identity.clone();
    d.field = 103;
    mutants.push(d);
    let mut d = identity.clone();
    d.coefficients[0] = 99;
    mutants.push(d);
    let mut d = identity.clone();
    d.u = vec![];
    mutants.push(d);
    let mut d = identity.clone();
    d.u = vec![0];
    mutants.push(d);
    let mut d = identity.clone();
    d.v = vec![101];
    mutants.push(d);
    let mut d = identity.clone();
    d.plus = 3;
    mutants.push(d);
    for mutant in &mutants {
        for cover in &descent.covers {
            assert!(cover.evaluate(mutant, false).is_err());
        }
    }
    println!("PASS T20directgeometry:twoactualdegree4Fpcovercoefficients/clearedcurves/positiveadjointsign/two-torsiontranslation+rationalhalves;120rationalpointcontrols+6branches+{pole_points}actualpolepoints/bothsignedinfinities/{repeated_inputs}actualrepeatedMumforddoubles/Frobenius/positivecoherenttranslationinvariance/actualMdagM4;actual16kernelclasses/all256sums/killed4/all16killed4/exactly4killed2/type4,4;{}malformedmutantsreject. Extensionfactorycharged;runtimeFp-only,no arbitraryFp12Jacdomain/costclaim",mutants.len());
}

fn evaluate_pair(descent: &Descent, input: &BalancedInput) -> [Pt<u64>; 2] {
    descent
        .covers
        .each_ref()
        .map(|cover| cover.evaluate(input, false).unwrap())
}

#[test]
fn t20_direct_norm_all_rational_classes_and_chain() {
    let f = FpR::new(101, 12);
    let base = Zp::new(101);
    let flat = Flat(&f);
    let mut maps = RichelotMaps::new(&f, &flat);
    let mut c20 = WholeContext::new(&f, &flat, models(&f).into_iter().nth(2).unwrap().1);
    let descent = derive_covers(&f, &maps, &c20);
    let chart50 = Chart::from_sextic(&f, &RAW_T20, f.from_u64(50), &[]);
    assert_eq!(chart50.a, maps.target.chart.a);
    assert_eq!(chart50.quintic, maps.target.chart.quintic);
    let points = c20.model.factors.map(|e| rational_points(&base, &e));
    assert_eq!((points[0].len(), points[1].len()), (109, 113));
    let mut targets = vec![];
    let mut inputs = vec![];
    let mut outputs = vec![];
    let mut degree_counts = [0_usize; 3];
    let mut repeated = 0;
    let mut nonsplit = 0;
    let mut pole_inputs = [0_usize; 2];
    let mut missing_corrections = [0_usize; 2];
    let mut wrong_sign = [0_usize; 2];
    for &p in &points[0] {
        for &q in &points[1] {
            let source = c20.product(&f, [p, q]);
            let d = maps.forward(&f, &maps.source.tag(&source)).unwrap();
            assert_eq!(maps.target.frobenius(&f, &d), d);
            let input = balanced_from_class(&f, &chart50, &maps.target.jac, &d);
            let image = evaluate_pair(&descent, &input);
            let reference = reference_norm(&f, &mut maps, &c20, &d).map(|p| project_point(&f, p));
            assert_eq!(image, reference);
            // This is an assertion about actual maps,never an evaluator input.
            let expected = [p, q].map(|p| p);
            for i in 0..2 {
                assert_eq!(
                    image[i],
                    curve::pmul(&base, &c20.model.factors[i], &expected[i], 4)
                );
            }
            let returned = geometric_adjoint(&f, &mut maps, &c20, image.map(|p| point_lift(&f, p)));
            assert_eq!(returned, maps.target.jac.mul(&d, 4));
            assert_eq!(maps.target.jac.mul(&returned, 9238), d);
            let degree = input.u.len() - 1;
            degree_counts[degree] += 1;
            repeated += usize::from(
                degree == 2
                    && poly::gcd(&base, &input.u, &poly::derivative(&base, &input.u)).len() > 1,
            );
            if degree == 2 {
                let discriminant = base.sub(base.sq(input.u[1]), base.mul(4, input.u[0]));
                nonsplit += usize::from(discriminant != 0 && base.sqrt(discriminant).is_none());
            }
            for i in 0..2 {
                let cover = &descent.covers[i];
                pole_inputs[i] += usize::from(poly::gcd(&base, &input.u, &cover.x.d).len() > 1);
                let coefficient = 2 * input.plus + degree as i64 - 2;
                let correction = curve::pmul(
                    &base,
                    &cover.curve,
                    &cover.infinity,
                    coefficient.unsigned_abs() as u128,
                );
                if correction != Pt::Inf {
                    assert_ne!(cover.evaluate(&input, true).unwrap(), image[i]);
                    missing_corrections[i] += 1;
                }
            }
            let negative = image.map(|p| curve::neg(&base, &p));
            wrong_sign[usize::from(negative == image)] += 1;
            targets.push(d);
            inputs.push(input);
            outputs.push(image);
            if targets.len() % 4096 == 0 {
                println!("PROGRESS T20direct:processed{}/12317", targets.len());
            }
        }
    }
    assert_eq!(targets.len(), 12317);
    assert_eq!(
        targets.iter().map(class_key).collect::<HashSet<_>>().len(),
        12317
    );
    assert_eq!(
        inputs
            .iter()
            .map(BalancedInput::key)
            .collect::<HashSet<_>>()
            .len(),
        12317
    );
    assert_eq!(outputs.iter().copied().collect::<HashSet<_>>().len(), 12317);
    assert_eq!(degree_counts[0], 3);
    assert!(repeated > 0 && nonsplit > 0);
    assert_eq!(wrong_sign, [12316, 1]);
    let generators = [
        geometric_adjoint(&f, &mut maps, &c20, [point_lift(&f, points[0][1]), Pt::Inf]),
        geometric_adjoint(&f, &mut maps, &c20, [Pt::Inf, point_lift(&f, points[1][1])]),
    ];
    let generator_images = generators.each_ref().map(|d| {
        evaluate_pair(
            &descent,
            &balanced_from_class(&f, &chart50, &maps.target.jac, d),
        )
    });
    for (index, d) in targets.iter().enumerate() {
        let negative = maps.target.jac.neg(d);
        assert_eq!(
            evaluate_pair(
                &descent,
                &balanced_from_class(&f, &chart50, &maps.target.jac, &negative)
            ),
            outputs[index].map(|p| curve::neg(&base, &p))
        );
        for component in 0..2 {
            let sum = maps.target.jac.add(d, &generators[component]);
            let got = evaluate_pair(
                &descent,
                &balanced_from_class(&f, &chart50, &maps.target.jac, &sum),
            );
            let expected = std::array::from_fn(|i| {
                curve::padd(
                    &base,
                    &c20.model.factors[i],
                    &outputs[index][i],
                    &generator_images[component][i],
                )
            });
            assert_eq!(got, expected);
        }
    }
    for i in 0..2 {
        let order = [109_u128, 113][i];
        assert_ne!(generator_images[i][i], Pt::Inf);
        assert_eq!(generator_images[i][1 - i], Pt::Inf);
        assert_eq!(
            maps.target.jac.mul(&generators[i], order),
            maps.target.jac.identity()
        );
        let mut orbit = HashSet::new();
        for scalar in 0..order {
            let d = maps.target.jac.mul(&generators[i], scalar);
            let image = evaluate_pair(
                &descent,
                &balanced_from_class(&f, &chart50, &maps.target.jac, &d),
            );
            assert_eq!(
                image[i],
                curve::pmul(
                    &base,
                    &c20.model.factors[i],
                    &generator_images[i][i],
                    scalar
                )
            );
            orbit.insert(image[i]);
        }
        assert_eq!(orbit.len(), order as usize);
    }
    assert_eq!(
        evaluate_pair(
            &descent,
            &BalancedInput {
                field: 101,
                coefficients: MONIC_T20,
                u: vec![1],
                v: vec![],
                plus: 1
            }
        ),
        [Pt::Inf; 2]
    );
    let mut c0 = WholeContext::new(&f, &flat, EvenModel::source());
    let first = crate::seven::construct_edges(&base, &c0.model.factors[0])
        .into_iter()
        .find(|e| e.map.ker == vec![72, 100, 35, 1])
        .unwrap();
    let second = crate::seven::construct_edges(&base, &first.map.cod)
        .into_iter()
        .find(|e| e.map.ker == vec![99, 94, 75, 1])
        .unwrap();
    let edges = [first, second];
    let original_points = c0.model.factors.map(|e| rational_points(&base, &e));
    let mut chain_classes = HashSet::new();
    for &p in &original_points[0] {
        for &q in &original_points[1] {
            let original = c0.product(&f, [p, q]);
            let middle = whole_forward(
                &f,
                &base,
                &edges,
                WholeCase::FortyNine,
                &mut c0,
                &mut c20,
                &original,
            );
            let target = maps.forward(&f, &maps.source.tag(&middle)).unwrap();
            let got = evaluate_pair(
                &descent,
                &balanced_from_class(&f, &chart50, &maps.target.jac, &target),
            );
            let original_norm = c0.rational_norm(&f, &original);
            let expected = [
                curve::pmul(
                    &base,
                    &c20.model.factors[0],
                    &WholeCase::FortyNine.forward(&base, &edges, original_norm[0]),
                    2,
                ),
                curve::pmul(&base, &c20.model.factors[1], &original_norm[1], 2),
            ];
            assert_eq!(
                got, expected,
                "accepted chain normalizes the existing direct norm/isogeny bridge"
            );
            chain_classes.insert(class_key(&target));
        }
    }
    assert_eq!(chain_classes.len(), 12317);
    println!("PASS T20directwhole:all12317actualbalancedinputs/Chart50roundtrip/validdistinctFpoutputpairs/actualN_CRdagagreement/bothactualnorm4/public9238recovery/identitynegationtwogeneratortranslations/all109+113scalarimagesorders/Frobenius;degrees{degree_counts:?}/repeated{repeated}/nonsplit{nonsplit}/actualpoleinputs{pole_inputs:?}/applicablemissinginfinitycorrections{missing_corrections:?}/wrongglobaladjointsign12316different1equal;all12317C0chainM_R_Phi49equals2diag(alpha49,id)N0. FinalruntimeZponly/noinputsupportroots;setupcharged/no costclaim");
}
