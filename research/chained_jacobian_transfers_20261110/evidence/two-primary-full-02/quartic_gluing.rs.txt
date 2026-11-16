//! Actual four-gluing on the admitted T_i via ramified pullbacks and the
//! existing field/elliptic/poly/separate Cantor engines. No producer maps.
use super::*;
#[path = "two_primary_quotient.rs"]
mod two_primary_quotient;

struct Four {
    points: Vec<Pt<ER>>,
    coordinates: HashMap<Pt<ER>, (u128, u128)>,
    basis: [Pt<ER>; 2],
    eta: ER,
}
fn frobenius(f: &FpR, p: Pt<ER>, degree: usize) -> Pt<ER> {
    match p {
        Pt::Inf => Pt::Inf,
        Pt::Aff(mut x, mut y) => {
            for _ in 0..degree {
                x = f.frobenius(x);
                y = f.frobenius(y);
            }
            Pt::Aff(x, y)
        }
    }
}
// Miller's n=4 recurrence: the second doubling is vertical at [2]P.
// Return None at an undefined presentation or zero,never silently cancel.
fn miller_four(f: &FpR, e: &Curve<ER>, p: Pt<ER>, t: Pt<ER>) -> Option<ER> {
    let (Pt::Aff(x, y), Pt::Aff(tx, ty)) = (p, t) else {
        return None;
    };
    if y == f.zero() {
        return None;
    }
    let Pt::Aff(dx, dy) = curve::pmul(f, e, &p, 2) else {
        return None;
    };
    assert_eq!(dy, f.zero());
    let den = f.sub(tx, dx);
    if den == f.zero() {
        return None;
    }
    let slope = f.div(
        f.add(f.mul(f.from_u64(3), f.sq(x)), e.a),
        f.mul(f.from_u64(2), y),
    );
    let line = f.sub(f.sub(ty, y), f.mul(slope, f.sub(tx, x)));
    let val = f.div(f.sq(line), den);
    (val != f.zero()).then_some(val)
}
fn basis_pairing(f: &FpR, e: &Curve<ER>, p: Pt<ER>, q: Pt<ER>, points: &[Pt<ER>]) -> ER {
    // Galbraith26.1 convention; inverse conventions on both factors agree
    // on the graph-isotropy question. All vertical factors are retained.
    let eta = f.div(
        miller_four(f, e, q, p).expect("independent basis evaluation"),
        miller_four(f, e, p, q).expect("independent basis evaluation"),
    );
    assert_eq!(f.sq(eta), f.neg(f.one()));
    assert_eq!(f.pow(eta, 4), f.one());
    assert_eq!(f.frobenius(eta), eta);
    assert!(eta == f.from_u64(10) || eta == f.from_u64(91));
    // Cross-check the normalized shortcut with genuinely disjoint divisors:
    // D_P=(P)-(O), D_Q=(Q+S)-(S), using translated f_Q(T-S).
    let mut shifted = 0;
    for &s in points {
        let ps = curve::padd(f, e, &p, &curve::neg(f, &s));
        let qs = curve::padd(f, e, &q, &s);
        if s == Pt::Inf || s == p || qs == Pt::Inf || qs == p {
            continue;
        }
        let values = (
            miller_four(f, e, q, ps),
            miller_four(f, e, p, s),
            miller_four(f, e, q, curve::neg(f, &s)),
            miller_four(f, e, p, qs),
        );
        if let (Some(a), Some(b), Some(c), Some(d)) = values {
            assert_eq!(f.div(f.mul(a, b), f.mul(c, d)), eta);
            shifted += 1;
        }
    }
    assert!(shifted > 0, "at least one disjoint divisor must be checked");
    eta
}
fn elliptic_four(f: &FpR, model: usize, component: usize) -> Four {
    let e = elliptic(f, model, component);
    assert!(curve::is_smooth(f, &e));
    let cubic = vec![e.b, e.a, f.zero(), f.one()];
    let mut two_x = poly::roots(f, &cubic, &mut Rng::new(0x3842));
    two_x.sort();
    assert_eq!(two_x.len(), 3);
    let a2 = f.sq(e.a);
    let h4 = vec![
        f.neg(f.add(f.mul(f.from_u64(8), f.sq(e.b)), f.mul(a2, e.a))),
        f.neg(f.mul(f.from_u64(4), f.mul(e.a, e.b))),
        f.neg(f.mul(f.from_u64(5), a2)),
        f.mul(f.from_u64(20), e.b),
        f.mul(f.from_u64(5), e.a),
        f.zero(),
        f.one(),
    ];
    let mut four_x = poly::roots(f, &h4, &mut Rng::new(0x3844));
    four_x.sort();
    assert_eq!(four_x.len(), 6);
    let mut points = vec![Pt::Inf];
    points.extend(two_x.iter().map(|&x| Pt::Aff(x, f.zero())));
    for x in four_x {
        let mut ys = poly::roots(
            f,
            &vec![f.neg(curve::rhs(f, &e, x)), f.zero(), f.one()],
            &mut Rng::new(0x3845),
        );
        ys.sort();
        assert_eq!(ys.len(), 2);
        for y in ys {
            assert_ne!(y, f.zero());
            let p = Pt::Aff(x, y);
            assert_ne!(curve::pmul(f, &e, &p, 2), Pt::Inf);
            points.push(p);
        }
    }
    let set = points.iter().copied().collect::<HashSet<_>>();
    assert_eq!(points.len(), 16);
    assert_eq!(set.len(), 16);
    for p in &points {
        assert!(curve::on_curve(f, &e, p));
        assert_eq!(curve::pmul(f, &e, p, 4), Pt::Inf);
        assert_eq!(frobenius(f, *p, 6), *p);
        for q in &points {
            assert!(set.contains(&curve::padd(f, &e, p, q)));
        }
    }
    let p = points[4];
    for &q in &points[4..] {
        let mut coordinates = HashMap::new();
        for a in 0..4 {
            for b in 0..4 {
                let t = curve::padd(
                    f,
                    &e,
                    &curve::pmul(f, &e, &p, a),
                    &curve::pmul(f, &e, &q, b),
                );
                coordinates.insert(t, (a, b));
            }
        }
        if coordinates.len() == 16 {
            assert_eq!(coordinates.keys().copied().collect::<HashSet<_>>(), set);
            let eta = basis_pairing(f, &e, p, q, &points);
            println!("INPUT T{model} factor{component} E4 basis P={p:?} Q={q:?} eta={eta:?};points={points:?}");
            return Four {
                points,
                coordinates,
                basis: [p, q],
                eta,
            };
        }
    }
    panic!("full elliptic4 basis missing");
}
fn pairing(f: &FpR, g: &Four, p: Pt<ER>, q: Pt<ER>) -> ER {
    let (a, b) = g.coordinates[&p];
    let (c, d) = g.coordinates[&q];
    f.pow(g.eta, (a * d + 16 - b * c) % 4)
}
fn pull_four(
    f: &FpR,
    model: usize,
    component: usize,
    p: Pt<ER>,
    chart: &Chart,
    j: &legacy::Hyp<'_, Flat<'_>>,
) -> (legacy::Div<ER>, bool) {
    let Pt::Aff(x, y) = p else {
        return (j.identity(), false);
    };
    if y != f.zero() {
        return (pull(f, model, component, &p, chart, j), false);
    }
    let c = &COVERS[model][component];
    let q = lift(f, &c.q);
    let u = poly::monic(
        f,
        &poly::sub(f, &lift(f, &c.n), &poly::scale(f, &poly::mul(f, &q, &q), x)),
    );
    assert_eq!(u.len(), 5);
    let g = poly::gcd(f, &lift(f, &MODELS[model]), &u);
    assert_eq!(g.len(), 3);
    assert_eq!(poly::gcd(f, &g, &poly::derivative(f, &g)), vec![f.one()]);
    let (remaining, remainder) = poly::divrem(f, &u, &g);
    assert!(remainder.is_empty());
    let mut roots = poly::roots(f, &remaining, &mut Rng::new(0x3846));
    roots.sort();
    assert_eq!(roots.len(), 1);
    let linear = vec![f.neg(roots[0]), f.one()];
    assert_eq!(remaining, poly::mul(f, &linear, &linear));
    assert_ne!(poly::eval(f, &lift(f, &MODELS[model]), roots[0]), f.zero());
    let corner = poly::eval(f, &g, chart.a) == f.zero();
    let out = if corner {
        let (h, rem) = poly::divrem(f, &g, &vec![f.neg(chart.a), f.one()]);
        assert!(rem.is_empty());
        assert_eq!(h.len(), 2);
        let ha = poly::eval(f, &h, chart.a);
        assert_ne!(ha, f.zero());
        j.reduce(legacy::Div {
            u: vec![f.div(chart.l, ha), f.one()],
            v: vec![],
        })
    } else {
        chart.class(f, j, &(g, vec![], 0))
    };
    assert!(j.is_valid(&out));
    assert_ne!(out, j.identity());
    assert_eq!(j.mul(&out, 2), j.identity());
    (out, corner)
}

#[test]
fn actual_quartic_four_gluing() {
    let f = FpR::new(101, 12);
    let flat = Flat(&f);
    for model in 0..3 {
        let groups = [elliptic_four(&f, model, 0), elliptic_four(&f, model, 1)];
        let chart = Chart::new(&f, model, &[]);
        let j = legacy::Hyp {
            f: &flat,
            h: chart.quintic.clone(),
            g: 2,
        };
        let mut pulled = vec![];
        for (component, group) in groups.iter().enumerate() {
            let mut corner_count = 0;
            let mut two_count = 0;
            let mut ds = vec![];
            for &p in &group.points {
                let (d, corner) = pull_four(&f, model, component, p, &chart, &j);
                corner_count += usize::from(corner);
                two_count += usize::from(matches!(p, Pt::Aff(_, y) if y == f.zero()));
                assert!(j.is_valid(&d));
                assert_eq!(j.mul(&d, 4), j.identity());
                ds.push(d);
            }
            assert_eq!(corner_count, 1);
            assert_eq!(two_count, 3);
            assert_eq!(
                ds.iter()
                    .map(|d| (d.u.clone(), d.v.clone()))
                    .collect::<HashSet<_>>()
                    .len(),
                16
            );
            let e = elliptic(&f, model, component);
            for (a, p) in group.points.iter().enumerate() {
                for (b, q) in group.points.iter().enumerate() {
                    let sum = curve::padd(&f, &e, p, q);
                    let index = group.points.iter().position(|&v| v == sum).unwrap();
                    assert_eq!(j.add(&ds[a], &ds[b]), ds[index]);
                }
            }
            assert_eq!(curve::pmul(&f, &e, &group.basis[0], 4), Pt::Inf);
            pulled.push(ds);
        }
        let mut kernel = vec![];
        let mut image = HashSet::new();
        for a in 0..16 {
            for b in 0..16 {
                let d = j.add(&pulled[0][a], &pulled[1][b]);
                assert!(j.is_valid(&d));
                assert_eq!(j.mul(&d, 4), j.identity());
                image.insert((d.u.clone(), d.v.clone()));
                if d == j.identity() {
                    kernel.push((groups[0].points[a], groups[1].points[b]));
                }
            }
        }
        assert_eq!(kernel.len(), 16);
        assert_eq!(image.len(), 16);
        let graph = kernel.iter().copied().collect::<HashSet<_>>();
        for component in 0..2 {
            let projection = kernel
                .iter()
                .map(|p| if component == 0 { p.0 } else { p.1 })
                .collect::<HashSet<_>>();
            assert_eq!(projection.len(), 16);
            assert_eq!(
                kernel
                    .iter()
                    .filter(|p| if component == 0 {
                        p.1 == Pt::Inf
                    } else {
                        p.0 == Pt::Inf
                    })
                    .count(),
                1
            );
        }
        let mut wrong_sign = 0;
        for &(p, q) in &kernel {
            assert!(graph.contains(&(frobenius(&f, p, 1), frobenius(&f, q, 1))));
            for &(r, s) in &kernel {
                assert!(graph.contains(&(
                    curve::padd(&f, &elliptic(&f, model, 0), &p, &r),
                    curve::padd(&f, &elliptic(&f, model, 1), &q, &s),
                )));
                let ep = pairing(&f, &groups[0], p, r);
                let eq = pairing(&f, &groups[1], q, s);
                assert_eq!(f.mul(ep, eq), f.one());
                wrong_sign += usize::from(f.div(ep, eq) != f.one());
            }
        }
        assert_eq!(wrong_sign, 96);
        let fixed = [1, 2, 3, 6].map(|degree| {
            kernel
                .iter()
                .filter(|&&(p, q)| frobenius(&f, p, degree) == p && frobenius(&f, q, degree) == q)
                .count()
        });
        assert_eq!(fixed, [1, 1, 4, 16]);
        println!("PASS actual quartic4glue T{model}:full16 elliptic4points each;all256productpulls valid/4ann;kernel16/image16;bothfactorpull256additiontables;3ramifiedY0/1branchWcorner per factor;kernelprojections bijective/trivialfactorintersections;all256graphsums/anti-isometrypairs;wrongsign96;Frobeniusfixed1,1,4,16 atdegrees1,2,3,6. Shared existing engines;globaldegree/polarization/Tateprimitivity separate written dependencies,no timing claim");
    }
}
