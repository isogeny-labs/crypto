//! Exact first models for one-factor transfers; whole Jacobian maps follow.
//! All arithmetic is the existing field/poly/Kohel/elliptic/Cantor code.
use super::*;
use isogeny_algos::genus2;
#[path = "partial_frobenius.rs"]
mod partial_frobenius;

#[derive(Clone, Copy, Debug)]
struct EvenCover {
    destination: usize,
    lead: u64,
    shift: u64,
    scale: u64,
}
impl EvenCover {
    fn finite<F: Field>(&self, f: &F, branch: usize, x: F::E, y: F::E) -> Pt<F::E> {
        let lead = f.from_u64(self.lead);
        let shift = f.from_u64(self.shift);
        let scale = f.from_u64(self.scale);
        let (u, v) = if branch == 0 {
            (f.sq(x), y)
        } else if x == f.zero() {
            return Pt::Inf;
        } else {
            (f.inv(f.sq(x)), f.div(y, f.pow(x, 3)))
        };
        Pt::Aff(
            f.mul(f.sq(scale), f.add(f.mul(lead, u), shift)),
            f.mul(f.pow(scale, 3), f.mul(lead, v)),
        )
    }
    fn infinity(&self, f: &FpR, branch: usize, leading_root: ER) -> Pt<ER> {
        if branch == 0 {
            Pt::Inf
        } else {
            Pt::Aff(
                f.mul(f.pow(f.from_u64(self.scale), 2), f.from_u64(self.shift)),
                f.mul(
                    f.pow(f.from_u64(self.scale), 3),
                    f.mul(f.from_u64(self.lead), leading_root),
                ),
            )
        }
    }
}
struct EvenModel {
    coefficients: Vec<u64>,
    factors: [Curve<u64>; 2],
    covers: [EvenCover; 2],
}
fn short_graph_from_even(f: &FpR, model: &EvenModel) -> Graph {
    let mut graph = vec![(Pt::Inf, Pt::Inf)];
    let roots = poly::roots(f, &lift(f, &model.coefficients), &mut Rng::new(0x4601));
    assert_eq!(roots.len(), 6);
    for x in roots {
        assert_ne!(x, f.zero());
        let mut points = [Pt::Inf; 2];
        for (branch, cover) in model.covers.iter().enumerate() {
            let p = cover.finite(f, branch, x, f.zero());
            assert_ne!(p, Pt::Inf);
            assert!(curve::on_curve(
                f,
                &extension_curve(f, model.factors[cover.destination]),
                &p
            ));
            assert_eq!(
                curve::pmul(
                    f,
                    &extension_curve(f, model.factors[cover.destination]),
                    &p,
                    2
                ),
                Pt::Inf
            );
            points[cover.destination] = p;
        }
        graph.push((points[0], points[1]));
    }
    let graph = graph
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Graph>();
    assert_eq!(graph.len(), 4);
    graph
}
fn construct_even(f: &FpR, factors: [Curve<u64>; 2], gamma: &Graph) -> EvenModel {
    let base = Zp::new(101);
    let &(p, q) = gamma.iter().find(|g| g.0 != Pt::Inf).unwrap();
    let paired = [0, 1, 2].map(|n| (frobenius(f, p, n), frobenius(f, q, n)));
    assert_eq!(frobenius(f, p, 3), p);
    assert_eq!(paired.iter().copied().collect::<HashSet<_>>().len(), 3);
    assert!(paired.iter().all(|p| gamma.contains(p)));
    let abscissas = [0, 1].map(|i| {
        paired.map(|pair| {
            let point = if i == 0 { pair.0 } else { pair.1 };
            let Pt::Aff(x, y) = point else {
                panic!("nonzero two-torsion")
            };
            assert_eq!(y, f.zero());
            x
        })
    });
    let raw = genus2::glue(f, abscissas[0], abscissas[1])
        .expect("actual stable nonisomorphic two-gluing");
    let coefficients = raw
        .iter()
        .map(|&a| {
            f.project(a)
                .expect("actual coefficients descend without a retwist")
        })
        .collect::<Vec<_>>();
    assert_eq!(coefficients.len(), 7);
    assert_ne!(coefficients[0], 0);
    assert_ne!(coefficients[6], 0);
    assert!([1, 3, 5].into_iter().all(|i| coefficients[i] == 0));
    assert_eq!(
        poly::gcd(
            &base,
            &coefficients,
            &poly::derivative(&base, &coefficients)
        ),
        vec![1]
    );
    let [c0, c2, c4, c6] = [0, 2, 4, 6].map(|i| coefficients[i]);
    let cubics = [vec![c0, c2, c4, c6], vec![c6, c4, c2, c0]];
    let covers = cubics.map(|c| {
        let e = genus2::cubic_to_short(&base, &c);
        let matches = factors
            .iter()
            .enumerate()
            .filter_map(|(i, target)| {
                crate::seven::checked_isomorphism(&base, &e, target).map(|scale| (i, scale))
            })
            .collect::<Vec<_>>();
        assert_eq!(matches.len(), 1);
        EvenCover {
            destination: matches[0].0,
            lead: c[3],
            shift: base.div(c[2], 3),
            scale: matches[0].1,
        }
    });
    assert_ne!(covers[0].destination, covers[1].destination);
    let model = EvenModel {
        coefficients,
        factors,
        covers,
    };
    assert_eq!(
        short_graph_from_even(f, &model)
            .into_iter()
            .collect::<HashSet<_>>(),
        gamma.iter().copied().collect()
    );
    model
}
fn check_even(f: &FpR, model: &EvenModel, name: &str) {
    let base = Zp::new(101);
    let coefficients = &model.coefficients;
    for (branch, cover) in model.covers.iter().enumerate() {
        let e = model.factors[cover.destination];
        let scale2 = base.sq(cover.scale);
        let (n, den) = if branch == 0 {
            (
                vec![
                    base.mul(scale2, cover.shift),
                    0,
                    base.mul(scale2, cover.lead),
                ],
                vec![1],
            )
        } else {
            (
                vec![
                    base.mul(scale2, cover.lead),
                    0,
                    base.mul(scale2, cover.shift),
                ],
                vec![0, 0, 1],
            )
        };
        let m = base.mul(base.pow(cover.scale, 3), cover.lead);
        let n2 = poly::mul(&base, &n, &n);
        let d2 = poly::mul(&base, &den, &den);
        let right = poly::add(
            &base,
            &poly::add(
                &base,
                &poly::mul(&base, &n2, &n),
                &poly::scale(&base, &poly::mul(&base, &n, &d2), e.a),
            ),
            &poly::scale(&base, &poly::mul(&base, &d2, &den), e.b),
        );
        assert_eq!(
            poly::scale(&base, coefficients, base.sq(m)),
            right,
            "full coefficientwise cover identity"
        );
    }
    let mut finite = 0;
    for x in 0..101 {
        for y in 0..101 {
            if base.sq(y) != poly::eval(&base, coefficients, x) {
                continue;
            }
            finite += 1;
            for (branch, cover) in model.covers.iter().enumerate() {
                let image = cover.finite(&base, branch, x, y);
                assert!(curve::on_curve(
                    &base,
                    &model.factors[cover.destination],
                    &image
                ));
            }
        }
    }
    let leading_root = f.sqrt(f.from_u64(coefficients[6])).unwrap();
    let zero_root = f.sqrt(f.from_u64(coefficients[0])).unwrap();
    for sign in [1, 100] {
        for (branch, cover) in model.covers.iter().enumerate() {
            let e = extension_curve(f, model.factors[cover.destination]);
            assert!(curve::on_curve(
                f,
                &e,
                &cover.infinity(f, branch, f.mul(f.from_u64(sign), leading_root))
            ));
            assert!(curve::on_curve(
                f,
                &e,
                &cover.finite(f, branch, f.zero(), f.mul(f.from_u64(sign), zero_root))
            ));
        }
    }
    let counts = genus2::point_counts(&base, coefficients);
    assert_eq!(counts, (120, 10436));
    assert_eq!(
        finite
            + if base.sqrt(coefficients[6]).is_some() {
                2
            } else {
                0
            },
        120
    );
    let l = genus2::lpoly(&base, coefficients);
    assert_eq!(l, genus2::LPoly::product(-7, -11, 101));
    assert_eq!(l.jac_order(101), 12317);
    println!("PASS unbalanced {name}:raw{coefficients:?};factors{:?};covers{:?};leading_root{:?};constant_root{:?};both coefficientidentities/all{finite}finite F101points/bothinfinity+bothx0overFp12/all6ramifiedroots/actual2graph;N1=120/N2=10436/Jac12317;no retwist,no whole Jacobian map acceptance yet",model.factors,model.covers,base.sqrt(coefficients[6]),base.sqrt(coefficients[0]));
}
#[test]
fn unbalanced_two_glued_models_and_pairing_control() {
    let f = FpR::new(101, 12);
    let (groups, q0) = actual_graph(&f, 0);
    let start = Presentation {
        curves: CURVES[0].map(|[a, b]| Curve::new(a, b)),
        graph: q0,
    };
    let gamma0 = doubled(&f, &start);
    let roots = poly::roots(&f, &lift(&f, &[56, 13, 6, 1]), &mut Rng::new(0x4602));
    assert_eq!(roots.len(), 3);
    let mut original = vec![(Pt::Inf, Pt::Inf)];
    original.extend(roots.into_iter().map(|u| {
        (
            Pt::Aff(f.add(u, f.from_u64(2)), f.zero()),
            Pt::Aff(f.add(f.div(f.from_u64(56), u), f.from_u64(38)), f.zero()),
        )
    }));
    assert_eq!(
        gamma0.iter().copied().collect::<HashSet<_>>(),
        original.iter().copied().collect()
    );
    let mut models: [Vec<_>; 2] = std::array::from_fn(|i| {
        CURVES
            .iter()
            .map(|cs| Curve::new(cs[i][0], cs[i][1]))
            .collect()
    });
    let mut hops: [HashMap<Curve<u64>, Hop>; 2] = std::array::from_fn(|_| HashMap::new());
    let mut state = start.clone();
    let mut history = vec![];
    for _ in 0..22 {
        history.push(state.clone());
        for (i, source) in state.curves.iter().copied().enumerate() {
            if !hops[i].contains_key(&source) {
                hops[i].insert(source, construct_hop(&f, source, i, &mut models[i]));
            }
        }
        let curves = std::array::from_fn(|i| hops[i][&state.curves[i]].target);
        let graph = state
            .graph
            .iter()
            .map(|&(p, q)| {
                (
                    hops[0][&state.curves[0]].eval(&f, p),
                    hops[1][&state.curves[1]].eval(&f, q),
                )
            })
            .collect();
        state = Presentation { curves, graph };
    }
    let first = &hops[0][&start.curves[0]];
    let second = &hops[0][&first.target];
    let gamma10 = gamma0
        .iter()
        .map(|&(p, q)| (first.eval(&f, p), q))
        .collect::<Graph>();
    let gamma20 = gamma10
        .iter()
        .map(|&(p, q)| (second.eval(&f, p), q))
        .collect::<Graph>();
    for (index, gamma) in [gamma10, gamma20].into_iter().enumerate() {
        let curves = [
            if index == 0 {
                first.target
            } else {
                second.target
            },
            start.curves[1],
        ];
        let expected = &history[[21, 6][index]];
        assert_eq!(expected.curves, curves);
        assert!(same_graph(&f, &gamma, &doubled(&f, expected)));
        let model = construct_even(&f, curves, &gamma);
        check_even(&f, &model, if index == 0 { "C10" } else { "C20" });
    }
    let e1four = transported_group(&f, &groups[0], first);
    let wrong = start
        .graph
        .iter()
        .map(|&(p, q)| (first.eval(&f, p), q))
        .collect::<Graph>();
    let mut bad = 0;
    for &(p, q) in &wrong {
        for &(r, s) in &wrong {
            let value = f.mul(pairing(&f, &e1four, p, r), pairing(&f, &groups[1], q, s));
            if value != f.one() {
                assert_eq!(value, f.neg(f.one()));
                bad += 1;
            }
        }
    }
    assert_eq!(bad, 96);
    let q20 = Presentation {
        curves: [second.target, start.curves[1]],
        graph: start
            .graph
            .iter()
            .map(|&(p, q)| (second.eval(&f, first.eval(&f, p)), q))
            .collect(),
    };
    assert!(same_graph(&f, &q20.graph, &history[6].graph));
    println!("PASS unbalanced phaseA:actualC0twoGraph=[2]Q0;C10Gamma21/C20Gamma6;Q20fourGraphA6uprelative sign;onefactor7fourGraph96nonisotropic(-1)witnesses/all256pairs. Existing CT44 obstruction credited;degrees7/49/normdiag7,1 andwholeJacobianmaps remain explicitnextphase,not nativeacceptance here");
}
