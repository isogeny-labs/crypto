//! Generic cover adapters around the existing field/poly/elliptic/Cantor code.
//! No rational logarithm labels enter a norm or pullback.
use super::*;
#[path = "even_kernel_certificates.rs"]
mod even_kernel_certificates;

#[derive(Clone, Copy, Debug)]
enum EvenPoint {
    Finite(ER, ER),
    Infinity(ER),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PullCase {
    Origin,
    Finite,
    Repeated,
    BranchCorner,
    Infinity,
}
impl EvenModel {
    fn source() -> Self {
        Self {
            coefficients: vec![56, 0, 13, 0, 6, 0, 1],
            factors: CURVES[0].map(|[a, b]| Curve::new(a, b)),
            covers: [
                EvenCover {
                    destination: 0,
                    lead: 1,
                    shift: 2,
                    scale: 1,
                },
                EvenCover {
                    destination: 1,
                    lead: 56,
                    shift: 38,
                    scale: 1,
                },
            ],
        }
    }
    fn leading_root(&self, f: &FpR) -> ER {
        if self.coefficients[6] == 1 {
            f.one()
        } else if let Some(root) = Zp::new(101).sqrt(self.coefficients[6]) {
            f.from_u64(root)
        } else {
            f.sqrt(f.from_u64(self.coefficients[6])).unwrap()
        }
    }
    fn chart(&self, f: &FpR) -> Chart {
        Chart::from_sextic(f, &self.coefficients, self.leading_root(f), &[])
    }
    fn point_image(&self, f: &FpR, p: EvenPoint) -> [Pt<ER>; 2] {
        let mut out = [Pt::Inf; 2];
        for (branch, cover) in self.covers.iter().enumerate() {
            out[cover.destination] = match p {
                EvenPoint::Finite(x, y) => cover.finite(f, branch, x, y),
                EvenPoint::Infinity(rho) => cover.infinity(f, branch, rho),
            };
            assert!(curve::on_curve(
                f,
                &extension_curve(f, self.factors[cover.destination]),
                &out[cover.destination]
            ));
        }
        out
    }
    fn point_class(
        &self,
        f: &FpR,
        chart: &Chart,
        j: &legacy::Hyp<'_, Flat<'_>>,
        p: EvenPoint,
    ) -> legacy::Div<ER> {
        match p {
            EvenPoint::Infinity(rho) => {
                let z = f.mul(f.sq(chart.l), rho);
                assert!(z == chart.q.v[0] || z == f.neg(chart.q.v[0]));
                if z == chart.q.v[0] {
                    chart.q.clone()
                } else {
                    j.neg(&chart.q)
                }
            }
            EvenPoint::Finite(x, y) => {
                assert_eq!(f.sq(y), poly::eval(f, &lift(f, &self.coefficients), x));
                if x == chart.a {
                    assert_eq!(y, f.zero());
                    return j.identity();
                }
                let w = f.div(chart.l, f.sub(x, chart.a));
                let z = f.div(f.mul(y, f.pow(w, 3)), chart.l);
                let raw = legacy::Div {
                    u: vec![f.neg(w), f.one()],
                    v: vec![z],
                };
                assert!(j.is_valid(&raw));
                j.reduce(raw)
            }
        }
    }
    fn support(
        &self,
        f: &FpR,
        chart: &Chart,
        j: &legacy::Hyp<'_, Flat<'_>>,
        d: &legacy::Div<ER>,
        cache: &mut RootCache,
    ) -> Vec<EvenPoint> {
        assert!(j.is_valid(d) && d.u.len() <= 3);
        let degree = d.u.len() - 1;
        let roots = cache.entry(d.u.clone()).or_insert_with(|| {
            let mut roots = poly::roots(f, &d.u, &mut Rng::new(0x4901));
            if degree == 2 && roots.len() == 1 {
                roots.push(roots[0]);
            }
            assert_eq!(roots.len(), degree, "complete support with multiplicity");
            roots
        });
        roots
            .iter()
            .map(|&w| {
                let z = poly::eval(f, &d.v, w);
                if w == f.zero() {
                    let rho = f.div(z, f.sq(chart.l));
                    assert_eq!(f.sq(rho), f.from_u64(self.coefficients[6]));
                    EvenPoint::Infinity(rho)
                } else {
                    let x = f.add(chart.a, f.div(chart.l, w));
                    let y = f.div(f.mul(z, chart.l), f.pow(w, 3));
                    assert_eq!(f.sq(y), poly::eval(f, &lift(f, &self.coefficients), x));
                    EvenPoint::Finite(x, y)
                }
            })
            .collect()
    }
    fn norm(
        &self,
        f: &FpR,
        chart: &Chart,
        j: &legacy::Hyp<'_, Flat<'_>>,
        d: &legacy::Div<ER>,
        cache: &mut RootCache,
    ) -> [Pt<ER>; 2] {
        let points = self.support(f, chart, j, d, cache);
        let base = self.point_image(f, EvenPoint::Finite(chart.a, f.zero()));
        std::array::from_fn(|i| {
            let e = extension_curve(f, self.factors[i]);
            let mut sum = Pt::Inf;
            for &p in &points {
                sum = curve::padd(f, &e, &sum, &self.point_image(f, p)[i]);
            }
            let correction = curve::pmul(f, &e, &base[i], points.len() as u128);
            curve::padd(f, &e, &sum, &curve::neg(f, &correction))
        })
    }
    fn frobenius_class(
        &self,
        f: &FpR,
        chart: &Chart,
        j: &legacy::Hyp<'_, Flat<'_>>,
        d: &legacy::Div<ER>,
        cache: &mut RootCache,
    ) -> legacy::Div<ER> {
        let points = self.support(f, chart, j, d, cache);
        let mut out = j.identity();
        for &p in &points {
            let image = match p {
                EvenPoint::Finite(x, y) => EvenPoint::Finite(f.frobenius(x), f.frobenius(y)),
                EvenPoint::Infinity(rho) => EvenPoint::Infinity(f.frobenius(rho)),
            };
            out = j.add(&out, &self.point_class(f, chart, j, image));
        }
        let changed_base = self.point_class(
            f,
            chart,
            j,
            EvenPoint::Finite(f.frobenius(chart.a), f.zero()),
        );
        j.add(&out, &j.neg(&j.mul(&changed_base, points.len() as u128)))
    }
    fn pull(
        &self,
        f: &FpR,
        chart: &Chart,
        j: &legacy::Hyp<'_, Flat<'_>>,
        component: usize,
        p: Pt<ER>,
    ) -> (legacy::Div<ER>, PullCase) {
        assert!(curve::on_curve(
            f,
            &extension_curve(f, self.factors[component]),
            &p
        ));
        let Pt::Aff(x, y) = p else {
            return (j.identity(), PullCase::Origin);
        };
        let (branch, cover) = self
            .covers
            .iter()
            .enumerate()
            .find(|(_, c)| c.destination == component)
            .unwrap();
        let scale = f.from_u64(cover.scale);
        let x = f.div(x, f.sq(scale));
        let y = f.div(y, f.pow(scale, 3));
        let lead = f.from_u64(cover.lead);
        let offset = f.sub(x, f.from_u64(cover.shift));
        if branch == 1 && offset == f.zero() {
            let positive_y = f.mul(lead, self.leading_root(f));
            assert!(y == positive_y || y == f.neg(positive_y));
            let plus = j.mul(&chart.q, 2);
            return (
                if y == positive_y { plus } else { j.neg(&plus) },
                PullCase::Infinity,
            );
        }
        let t = if branch == 0 {
            f.div(offset, lead)
        } else {
            f.div(lead, offset)
        };
        let u = vec![f.neg(t), f.zero(), f.one()];
        let v = if branch == 0 {
            vec![f.div(y, lead)]
        } else {
            vec![f.zero(), f.div(f.mul(y, t), lead)]
        };
        assert!(poly::rem(
            f,
            &poly::sub(f, &poly::mul(f, &v, &v), &lift(f, &self.coefficients)),
            &u
        )
        .is_empty());
        if poly::eval(f, &u, chart.a) == f.zero() {
            assert_eq!(y, f.zero());
            let (h, rem) = poly::divrem(f, &u, &vec![f.neg(chart.a), f.one()]);
            assert!(rem.is_empty() && h.len() == 2);
            let raw = legacy::Div {
                u: vec![f.div(chart.l, poly::eval(f, &h, chart.a)), f.one()],
                v: vec![],
            };
            assert!(j.is_valid(&raw));
            (j.reduce(raw), PullCase::BranchCorner)
        } else {
            (
                chart.class(f, j, &(u, v, 0)),
                if t == f.zero() {
                    PullCase::Repeated
                } else {
                    PullCase::Finite
                },
            )
        }
    }
}

fn models(f: &FpR) -> Vec<(&'static str, EvenModel, Graph)> {
    let (_, q0) = actual_graph(f, 0);
    let source = EvenModel::source();
    let start = Presentation {
        curves: source.factors,
        graph: q0,
    };
    let gamma = doubled(f, &start);
    let mut e_models = vec![source.factors[0]];
    let first = construct_hop(f, source.factors[0], 0, &mut e_models);
    let second = construct_hop(f, first.target, 0, &mut e_models);
    let gamma10 = gamma
        .iter()
        .map(|&(p, q)| (first.eval(f, p), q))
        .collect::<Graph>();
    let gamma20 = gamma10
        .iter()
        .map(|&(p, q)| (second.eval(f, p), q))
        .collect::<Graph>();
    let c10 = construct_even(f, [first.target, source.factors[1]], &gamma10);
    let c20 = construct_even(f, [second.target, source.factors[1]], &gamma20);
    let gamma_pi1 = gamma
        .iter()
        .map(|&(p, q)| (frobenius(f, p, 1), q))
        .collect::<Graph>();
    let gamma_pi2 = gamma
        .iter()
        .map(|&(p, q)| (frobenius(f, p, 2), q))
        .collect::<Graph>();
    let pi1 = construct_even(f, source.factors, &gamma_pi1);
    let pi2 = construct_even(f, source.factors, &gamma_pi2);
    vec![
        ("C0", source, gamma),
        ("C10", c10, gamma10),
        ("C20", c20, gamma20),
        ("Cpi1", pi1, gamma_pi1),
        ("Cpi2", pi2, gamma_pi2),
    ]
}

#[test]
fn generic_even_charts_pull_norms_and_corners() {
    let f = FpR::new(101, 12);
    let base = Zp::new(101);
    let flat = Flat(&f);
    for (name, model, gamma) in models(&f) {
        let expected = match name {
            "C0" => [56, 0, 13, 0, 6, 0, 1],
            "C10" => [89, 0, 93, 0, 8, 0, 61],
            "C20" => [40, 0, 45, 0, 75, 0, 18],
            "Cpi1" => [52, 0, 25, 0, 77, 0, 84],
            "Cpi2" => [72, 0, 85, 0, 14, 0, 93],
            _ => unreachable!(),
        };
        assert_eq!(model.coefficients, expected);
        let chart = model.chart(&f);
        let j = legacy::Hyp {
            f: &flat,
            h: chart.quintic.clone(),
            g: 2,
        };
        assert!(j.is_valid(&chart.q));
        let mut cache = RootCache::new();
        let branch_image = model.point_image(&f, EvenPoint::Finite(chart.a, f.zero()));
        assert!(branch_image.iter().all(|p| *p != Pt::Inf));
        let mut curve_points = vec![];
        for x in 0..101 {
            for y in 0..101 {
                if base.sq(y) == poly::eval(&base, &model.coefficients, x) {
                    curve_points.push(EvenPoint::Finite(f.from_u64(x), f.from_u64(y)));
                }
            }
        }
        let rho = model.leading_root(&f);
        curve_points.extend([EvenPoint::Infinity(rho), EvenPoint::Infinity(f.neg(rho))]);
        let mut base_mutant = [0; 2];
        for &p in &curve_points {
            let d = model.point_class(&f, &chart, &j, p);
            let actual = model.norm(&f, &chart, &j, &d, &mut cache);
            let images = model.point_image(&f, p);
            for i in 0..2 {
                let e = extension_curve(&f, model.factors[i]);
                let expected = curve::padd(&f, &e, &images[i], &curve::neg(&f, &branch_image[i]));
                assert_eq!(actual[i], expected);
                base_mutant[i] += usize::from(actual[i] != images[i]);
            }
            let fp = match p {
                EvenPoint::Finite(x, y) => EvenPoint::Finite(f.frobenius(x), f.frobenius(y)),
                EvenPoint::Infinity(rho) => EvenPoint::Infinity(f.frobenius(rho)),
            };
            let expected = j.add(
                &model.point_class(&f, &chart, &j, fp),
                &j.neg(&model.point_class(
                    &f,
                    &chart,
                    &j,
                    EvenPoint::Finite(f.frobenius(chart.a), f.zero()),
                )),
            );
            assert_eq!(
                model.frobenius_class(&f, &chart, &j, &d, &mut cache),
                expected
            );
        }
        assert!(base_mutant.iter().all(|&n| n > 0));
        let mut cases = [0; 5];
        for component in 0..2 {
            let e = extension_curve(&f, model.factors[component]);
            let mut points = rational_points(&base, &model.factors[component])
                .into_iter()
                .map(|p| point_lift(&f, p))
                .collect::<Vec<_>>();
            let old_index = (0..3)
                .find(|&m| {
                    Curve::new(CURVES[m][component][0], CURVES[m][component][1])
                        == model.factors[component]
                })
                .unwrap();
            let four = elliptic_four(&f, old_index, component);
            let pulled_four = four
                .points
                .iter()
                .map(|&p| model.pull(&f, &chart, &j, component, p).0)
                .collect::<Vec<_>>();
            let keys = pulled_four
                .iter()
                .map(|d| (d.u.clone(), d.v.clone()))
                .collect::<HashSet<_>>();
            assert_eq!(
                keys.len(),
                16,
                "optimal elliptic embedding is injective on four-torsion"
            );
            for (&p, dp) in four.points.iter().zip(&pulled_four) {
                assert_eq!(j.mul(dp, 4), j.identity());
                for (&q, dq) in four.points.iter().zip(&pulled_four) {
                    let sum = curve::padd(&f, &e, &p, &q);
                    assert_eq!(j.add(dp, dq), model.pull(&f, &chart, &j, component, sum).0);
                }
            }
            points.extend(four.points.iter().copied());
            let zero_root = f.sqrt(f.from_u64(model.coefficients[0])).unwrap();
            for sign in [1, 100] {
                points.push(
                    model.point_image(&f, EvenPoint::Infinity(f.mul(f.from_u64(sign), rho)))
                        [component],
                );
                points.push(
                    model.point_image(
                        &f,
                        EvenPoint::Finite(f.zero(), f.mul(f.from_u64(sign), zero_root)),
                    )[component],
                );
            }
            for p in points {
                let (d, case) = model.pull(&f, &chart, &j, component, p);
                cases[case as usize] += 1;
                assert!(j.is_valid(&d));
                let norms = model.norm(&f, &chart, &j, &d, &mut cache);
                assert_eq!(norms[component], curve::pmul(&f, &e, &p, 2));
                assert_eq!(norms[1 - component], Pt::Inf);
                let expected = model.pull(&f, &chart, &j, component, frobenius(&f, p, 1)).0;
                assert_eq!(
                    model.frobenius_class(&f, &chart, &j, &d, &mut cache),
                    expected
                );
            }
        }
        assert!(cases[PullCase::Repeated as usize] >= 2);
        assert!(cases[PullCase::BranchCorner as usize] >= 2);
        assert!(cases[PullCase::Infinity as usize] >= 2);
        for (p, q) in gamma {
            assert_eq!(
                j.add(
                    &model.pull(&f, &chart, &j, 0, p).0,
                    &model.pull(&f, &chart, &j, 1, q).0
                ),
                j.identity()
            );
        }
        println!("PASS genericEven {name}:raw{:?};rho{rho:?};all{}curvepointclasses norm/basepoint/Frobenius;omittedbasepointmutant{base_mutant:?};rational109+113andgeometric4torsion/infinity/x0pullnorm2/cross0/Frobenius;pullcases{cases:?};all4actualgraphkernel", model.coefficients, curve_points.len());
    }
}

type EvenClassKey = (Vec<ER>, Vec<ER>);
fn class_key(d: &legacy::Div<ER>) -> EvenClassKey {
    (d.u.clone(), d.v.clone())
}

struct WholeContext<'a> {
    model: EvenModel,
    chart: Chart,
    jac: legacy::Hyp<'a, Flat<'a>>,
    roots: RootCache,
    norms: HashMap<EvenClassKey, [Pt<u64>; 2]>,
    pulls: [HashMap<Pt<u64>, legacy::Div<ER>>; 2],
}
impl<'a> WholeContext<'a> {
    fn new(f: &'a FpR, flat: &'a Flat<'a>, model: EvenModel) -> Self {
        let chart = model.chart(f);
        let jac = legacy::Hyp {
            f: flat,
            h: chart.quintic.clone(),
            g: 2,
        };
        Self {
            model,
            chart,
            jac,
            roots: RootCache::new(),
            norms: HashMap::new(),
            pulls: std::array::from_fn(|_| HashMap::new()),
        }
    }
    // Lazy memoization is keyed only by the actual Mumford input. A miss
    // executes the pointwise divisor norm; no enumeration index is accepted.
    fn rational_norm(&mut self, f: &FpR, d: &legacy::Div<ER>) -> [Pt<u64>; 2] {
        let key = class_key(d);
        if let Some(points) = self.norms.get(&key) {
            return *points;
        }
        let out = self
            .model
            .norm(f, &self.chart, &self.jac, d, &mut self.roots)
            .map(|p| match p {
                Pt::Inf => Pt::Inf,
                Pt::Aff(x, y) => Pt::Aff(
                    f.project(x).expect("rational norm x"),
                    f.project(y).expect("rational norm y"),
                ),
            });
        self.norms.insert(key, out);
        out
    }
    fn component_pull(&mut self, f: &FpR, component: usize, p: Pt<u64>) -> legacy::Div<ER> {
        if let Some(d) = self.pulls[component].get(&p) {
            return d.clone();
        }
        let d = self
            .model
            .pull(f, &self.chart, &self.jac, component, point_lift(f, p))
            .0;
        self.pulls[component].insert(p, d.clone());
        d
    }
    fn product(&mut self, f: &FpR, points: [Pt<u64>; 2]) -> legacy::Div<ER> {
        let first = self.component_pull(f, 0, points[0]);
        let second = self.component_pull(f, 1, points[1]);
        let out = self.jac.add(&first, &second);
        assert!(self.jac.is_valid(&out));
        out
    }
    fn assert_rational(&mut self, f: &FpR, d: &legacy::Div<ER>) {
        assert_eq!(
            self.model
                .frobenius_class(f, &self.chart, &self.jac, d, &mut self.roots),
            *d
        );
    }
}

#[derive(Clone, Copy, Debug)]
enum WholeCase {
    Seven,
    FortyNine,
    Pi1,
    Pi2,
}
impl WholeCase {
    fn degree(self) -> u128 {
        match self {
            Self::Seven => 7,
            Self::FortyNine => 49,
            Self::Pi1 => 101,
            Self::Pi2 => 10201,
        }
    }
    fn inverse_norm(self) -> u128 {
        match self {
            Self::Seven => 78,
            Self::FortyNine => 89,
            Self::Pi1 => 68,
            Self::Pi2 => 46,
        }
    }
    fn forward(self, base: &Zp, edges: &[crate::seven::SevenEdge; 2], p: Pt<u64>) -> Pt<u64> {
        match self {
            Self::Seven => edges[0].map.eval(base, &p),
            Self::FortyNine => edges[1].map.eval(base, &edges[0].map.eval(base, &p)),
            // Only F101 rational points reach this stage,where pi is identity.
            Self::Pi1 | Self::Pi2 => p,
        }
    }
    fn adjoint(self, base: &Zp, edges: &[crate::seven::SevenEdge; 2], p: Pt<u64>) -> Pt<u64> {
        match self {
            Self::Seven => edges[0].reverse(base, &p),
            Self::FortyNine => edges[0].reverse(base, &edges[1].reverse(base, &p)),
            // V*pi=[101],pi=id on rational points. This is not a formula
            // for Verschiebung on arbitrary extension-field points.
            Self::Pi1 | Self::Pi2 => curve::pmul(base, &edges[0].map.dom, &p, self.degree()),
        }
    }
}

fn whole_forward(
    f: &FpR,
    base: &Zp,
    edges: &[crate::seven::SevenEdge; 2],
    kind: WholeCase,
    source: &mut WholeContext<'_>,
    target: &mut WholeContext<'_>,
    d: &legacy::Div<ER>,
) -> legacy::Div<ER> {
    let points = source.rational_norm(f, d);
    let half_e = curve::pmul(base, &source.model.factors[0], &points[0], 55);
    let half_f = curve::pmul(base, &source.model.factors[1], &points[1], 57);
    target.product(f, [kind.forward(base, edges, half_e), half_f])
}

fn whole_adjoint_recovery(
    f: &FpR,
    base: &Zp,
    edges: &[crate::seven::SevenEdge; 2],
    kind: WholeCase,
    source: &mut WholeContext<'_>,
    target: &mut WholeContext<'_>,
    d: &legacy::Div<ER>,
) -> (legacy::Div<ER>, legacy::Div<ER>) {
    let points = target.rational_norm(f, d);
    let first = curve::pmul(
        base,
        &source.model.factors[0],
        &kind.adjoint(base, edges, points[0]),
        55,
    );
    let second = curve::pmul(base, &source.model.factors[1], &points[1], 57);
    let adjoint = source.product(f, [first, second]);
    let recovered_first = curve::pmul(base, &source.model.factors[0], &first, kind.inverse_norm());
    let recovered = source.product(f, [recovered_first, second]);
    (adjoint, recovered)
}

#[test]
fn unbalanced_and_partial_whole_rational_maps() {
    let f = FpR::new(101, 12);
    let base = Zp::new(101);
    let flat = Flat(&f);
    let mut contexts = models(&f)
        .into_iter()
        .map(|(_, model, _)| WholeContext::new(&f, &flat, model))
        .collect::<Vec<_>>();
    let (source, targets) = contexts.split_first_mut().unwrap();
    assert_eq!(targets.len(), 4);
    let first = crate::seven::construct_edges(&base, &source.model.factors[0])
        .into_iter()
        .find(|edge| edge.map.ker == vec![72, 100, 35, 1])
        .unwrap();
    assert_eq!(first.map.cod, targets[0].model.factors[0]);
    let second = crate::seven::construct_edges(&base, &first.map.cod)
        .into_iter()
        .find(|edge| edge.map.ker == vec![99, 94, 75, 1])
        .unwrap();
    assert_eq!(second.map.cod, targets[1].model.factors[0]);
    let edges = [first, second];
    let kinds = [
        WholeCase::Seven,
        WholeCase::FortyNine,
        WholeCase::Pi1,
        WholeCase::Pi2,
    ];
    let rational = source.model.factors.map(|e| rational_points(&base, &e));
    assert_eq!([rational[0].len(), rational[1].len()], [109, 113]);
    let mut corpus = Vec::with_capacity(12317);
    let mut witnesses = Vec::with_capacity(12317);
    let mut source_index = HashMap::new();
    for &p in &rational[0] {
        for &q in &rational[1] {
            let d = source.product(&f, [p, q]);
            source.assert_rational(&f, &d);
            let expected = [
                curve::pmul(&base, &source.model.factors[0], &p, 2),
                curve::pmul(&base, &source.model.factors[1], &q, 2),
            ];
            assert_eq!(source.rational_norm(&f, &d), expected);
            assert!(source_index.insert(class_key(&d), corpus.len()).is_none());
            corpus.push(d);
            witnesses.push([p, q]);
        }
    }
    assert_eq!(corpus.len(), 12317);
    assert_eq!(corpus[0], source.jac.identity());
    println!("PASS wholeEven source:all12317 valid/distinct/Frobeniusfixed actualC0classes;normfromMumford=2actualproductcoordinates;no inputloglabels in evaluator");
    let mut outputs: [Vec<legacy::Div<ER>>; 4] = std::array::from_fn(|_| Vec::with_capacity(12317));
    let mut mutant_differences = [0; 4];
    let mut mutant_equalities = [0; 4];
    for (case, target) in targets.iter_mut().enumerate() {
        let kind = kinds[case];
        let mut target_keys = HashSet::new();
        for (index, (d, &[p, q])) in corpus.iter().zip(&witnesses).enumerate() {
            let out = whole_forward(&f, &base, &edges, kind, source, target, d);
            target.assert_rational(&f, &out);
            let expected = target.product(&f, [kind.forward(&base, &edges, p), q]);
            assert_eq!(out, expected);
            assert!(target_keys.insert(class_key(&out)));
            let (adjoint, recovered) =
                whole_adjoint_recovery(&f, &base, &edges, kind, source, target, &out);
            assert_eq!(recovered, *d);
            let first = curve::pmul(&base, &source.model.factors[0], &p, kind.degree());
            let expected_norm = source.product(&f, [first, q]);
            assert_eq!(adjoint, expected_norm);
            let wrong_second = curve::pmul(&base, &source.model.factors[1], &q, kind.degree());
            let wrong_uniform = source.product(&f, [first, wrong_second]);
            mutant_differences[case] += usize::from(adjoint != wrong_uniform);
            mutant_equalities[case] += usize::from(adjoint == wrong_uniform);
            if matches!(kind, WholeCase::Pi1 | WholeCase::Pi2) {
                let n = source.rational_norm(&f, d);
                let direct_bridge = target.product(&f, n);
                assert_eq!(target.jac.mul(&out, 2), direct_bridge);
            }
            outputs[case].push(out);
            if index % 4096 == 0 {
                println!("PROGRESS wholeEven {kind:?}:processed{index}/12317");
            }
        }
        assert_eq!(target_keys.len(), 12317);
        assert_eq!(mutant_differences[case], 12208);
        assert_eq!(mutant_equalities[case], 109);
        println!("PASS wholeEven {kind:?}:all12317 actualMumfordforward/valid/distinct/Frobeniusfixed/expectedproduct/actualadjoint/recovery;nonuniformdiag{},1;wronguniform12208different109equal;lazycoordinate-and-Mumford-keycaches,no timingclaim", kind.degree());
    }
    let generators = [
        source.product(&f, [rational[0][1], Pt::Inf]),
        source.product(&f, [Pt::Inf, rational[1][1]]),
    ];
    let generator_indices = generators.each_ref().map(|d| source_index[&class_key(d)]);
    for (case, target) in targets.iter().enumerate() {
        assert_eq!(outputs[case][0], target.jac.identity());
        for (index, d) in corpus.iter().enumerate() {
            let negative = source.jac.neg(d);
            assert_eq!(
                outputs[case][source_index[&class_key(&negative)]],
                target.jac.neg(&outputs[case][index])
            );
            for (component, generator) in generators.iter().enumerate() {
                let sum = source.jac.add(d, generator);
                assert_eq!(
                    outputs[case][source_index[&class_key(&sum)]],
                    target.jac.add(
                        &outputs[case][index],
                        &outputs[case][generator_indices[component]]
                    )
                );
            }
        }
        for (component, generator) in generators.iter().enumerate() {
            let order = [109, 113][component];
            let image = &outputs[case][generator_indices[component]];
            assert_ne!(*image, target.jac.identity());
            assert_eq!(target.jac.mul(image, order), target.jac.identity());
            let mut scalar_images = HashSet::new();
            for scalar in 0..order {
                let input = source.jac.mul(generator, scalar);
                let actual = &outputs[case][source_index[&class_key(&input)]];
                let expected = target.jac.mul(image, scalar);
                assert_eq!(*actual, expected);
                scalar_images.insert(class_key(actual));
            }
            assert_eq!(scalar_images.len(), order as usize);
        }
        println!("PASS wholeEven {:?}:all12317identity/negation/bothgeneratortranslations;all109+113embeddedDLPscalars/injectiveorders", kinds[case]);
    }
    let (_, partials) = targets.split_at_mut(2);
    let (first_partial, second_partial) = partials.split_at_mut(1);
    let pi1 = &mut first_partial[0];
    let pi2 = &mut second_partial[0];
    for (index, d) in corpus.iter().enumerate() {
        let second = whole_forward(
            &f,
            &base,
            &edges,
            WholeCase::Pi1,
            pi1,
            pi2,
            &outputs[2][index],
        );
        assert_eq!(second, outputs[3][index]);
        let closed = whole_forward(&f, &base, &edges, WholeCase::Pi1, pi2, source, &second);
        assert_eq!(closed, *d);
        let n = source.rational_norm(&f, &closed);
        let first = curve::neg(
            &base,
            &curve::pmul(&base, &source.model.factors[0], &n[0], 55),
        );
        let second = curve::pmul(&base, &source.model.factors[1], &n[1], 57);
        let signed = source.product(&f, [first, second]);
        assert_eq!(signed, source.jac.mul(d, 6103));
    }
    assert_eq!((6103 % 109, 6103 % 113), (108, 1));
    println!("PASS wholeEven rationaltransfer:all4targets/all12317Mumfordinputs/actual7+49normalizedduals/FrobeniusadjointV=p on rationalfactors/recovery/nonuniformmutants12208vs109;allpointgroupcontrols/DLPscalars;actualthree-stepC0-Cpi1-Cpi2-C0loopidentityandoptionalCRT6103all12317;Frobeniusclearedmaps=existingdirectbridgeall12317. Geometric7/49kernelscheme certificates remainnextstage;no cost/signatureclaim");
}
