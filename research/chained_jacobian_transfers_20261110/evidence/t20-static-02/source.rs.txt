//! Actual primitive-four kernel and normalized nonsplit Richelot geometry.
//! Field/poly/elliptic/Cantor/determinant/count engines are unchanged.
use super::*;

fn perfect_matchings(left: &[usize]) -> Vec<Vec<[usize; 2]>> {
    if left.is_empty() {
        return vec![vec![]];
    }
    let mut out = vec![];
    for i in 1..left.len() {
        let rest = left
            .iter()
            .enumerate()
            .filter_map(|(j, &index)| (j != 0 && j != i).then_some(index))
            .collect::<Vec<_>>();
        for mut matching in perfect_matchings(&rest) {
            matching.insert(0, [left[0], left[i]]);
            out.push(matching);
        }
    }
    out
}

fn point_key(p: Pt<ER>) -> (u8, ER, ER) {
    match p {
        Pt::Inf => (0, [0; 16], [0; 16]),
        Pt::Aff(x, y) => (1, x, y),
    }
}

fn json_point(p: Pt<ER>) -> String {
    match p {
        Pt::Inf => "null".to_string(),
        Pt::Aff(x, y) => format!("{{\"x\":{x:?},\"y\":{y:?}}}"),
    }
}

struct T20Geometry {
    source: EvenModel,
    q20: Graph,
    kernel: Vec<legacy::Div<ER>>,
    kernel_frobenius: Vec<usize>,
    roots: Vec<ER>,
    matching: Vec<[usize; 2]>,
    stable_determinants: Vec<u64>,
    common_scale: u64,
    delta: ER,
    factors: [Vec<ER>; 3],
    normalized: [Vec<ER>; 3],
    target: Vec<u64>,
}

fn t20_geometry(f: &FpR) -> T20Geometry {
    let base = Zp::new(101);
    let (groups, original) = actual_graph(f, 0);
    let mut elliptic_models = vec![Curve::new(1, 46)];
    let first = construct_hop(f, elliptic_models[0], 0, &mut elliptic_models);
    let second = construct_hop(f, first.target, 0, &mut elliptic_models);
    assert_eq!(second.target, Curve::new(59, 27));
    let e2four = transported_group(f, &transported_group(f, &groups[0], &first), &second);
    let factors = [second.target, Curve::new(44, 21)];
    let mut q20 = original
        .iter()
        .map(|&(p, q)| (second.eval(f, first.eval(f, p)), q))
        .collect::<Graph>();
    q20.sort_by_key(|&(p, q)| (point_key(p), point_key(q)));
    assert_eq!(q20.len(), 16);
    assert_eq!(q20.iter().copied().collect::<HashSet<_>>().len(), 16);
    for &(p, q) in &q20 {
        for &(r, s) in &q20 {
            let sum = (
                curve::padd(f, &extension_curve(f, factors[0]), &p, &r),
                curve::padd(f, &extension_curve(f, factors[1]), &q, &s),
            );
            assert!(q20.contains(&sum));
            assert_eq!(
                f.mul(pairing(f, &e2four, p, r), pairing(f, &groups[1], q, s)),
                f.one()
            );
        }
    }
    let gamma = doubled(
        f,
        &Presentation {
            curves: factors,
            graph: q20.clone(),
        },
    );
    assert_eq!(gamma.len(), 4);
    let source = construct_even(f, factors, &gamma);
    assert_eq!(source.coefficients, vec![40, 0, 45, 0, 75, 0, 18]);
    let flat = Flat(f);
    let chart = source.chart(f);
    let jac = legacy::Hyp {
        f: &flat,
        h: chart.quintic.clone(),
        g: 2,
    };
    let mut cache = RootCache::new();
    let product = |p, q| {
        let a = source.pull(f, &chart, &jac, 0, p).0;
        let b = source.pull(f, &chart, &jac, 1, q).0;
        jac.add(&a, &b)
    };
    for &(p, q) in &gamma {
        assert_eq!(product(p, q), jac.identity());
    }
    let mut kernel = vec![];
    for &(p, q) in &q20 {
        let d = product(p, q);
        assert!(jac.is_valid(&d));
        assert_eq!(jac.mul(&d, 2), jac.identity());
        if !kernel.contains(&d) {
            kernel.push(d);
        }
    }
    kernel.sort_by_key(class_key);
    assert_eq!(kernel.len(), 4);
    let mut kernel_frobenius = vec![];
    for a in &kernel {
        for b in &kernel {
            assert!(kernel.contains(&jac.add(a, b)));
        }
        let pi = source.frobenius_class(f, &chart, &jac, a, &mut cache);
        let next = kernel.iter().position(|d| *d == pi).unwrap();
        if *a != jac.identity() {
            assert_ne!(*a, pi);
        }
        kernel_frobenius.push(next);
    }
    for i in 0..4 {
        assert_eq!(kernel_frobenius[kernel_frobenius[kernel_frobenius[i]]], i);
    }
    let mut roots = poly::roots(f, &lift(f, &source.coefficients), &mut Rng::new(0x2044));
    roots.sort();
    assert_eq!(roots.len(), 6);
    let root_frobenius = roots
        .iter()
        .map(|&x| roots.iter().position(|&y| y == f.frobenius(x)).unwrap())
        .collect::<Vec<_>>();
    for i in 0..6 {
        assert_ne!(root_frobenius[i], i);
        assert_eq!(root_frobenius[root_frobenius[root_frobenius[i]]], i);
    }
    let branch_classes = roots
        .iter()
        .map(|&x| source.point_class(f, &chart, &jac, EvenPoint::Finite(x, f.zero())))
        .collect::<Vec<_>>();
    let matchings = perfect_matchings(&[0, 1, 2, 3, 4, 5]);
    assert_eq!(matchings.len(), 15);
    let mut stable_determinants = vec![];
    let mut chosen = None;
    for matching in matchings {
        let quadratics: [Vec<ER>; 3] = std::array::from_fn(|i| {
            let [a, b] = matching[i];
            poly::mul(
                f,
                &vec![f.neg(roots[a]), f.one()],
                &vec![f.neg(roots[b]), f.one()],
            )
        });
        let stable = matching.iter().all(|pair| {
            let mut image = pair.map(|i| root_frobenius[i]);
            image.sort();
            matching.contains(&image)
        });
        let monic_delta = isogeny_algos::genus2::richelot(f, &quadratics).delta;
        if stable {
            stable_determinants.push(f.project(monic_delta).unwrap());
        }
        let this_kernel = matching.iter().all(|&[a, b]| {
            let d = jac.add(&branch_classes[a], &branch_classes[b]);
            assert_ne!(d, jac.identity());
            kernel.contains(&d)
        });
        if this_kernel {
            assert!(stable);
            assert!(chosen.is_none());
            assert!([34, 67].contains(&f.project(monic_delta).unwrap()));
            chosen = Some((matching, quadratics));
        }
    }
    stable_determinants.sort();
    assert_eq!(stable_determinants, vec![0, 34, 67]);
    let (matching, monic) = chosen.expect("unique actual primitive-four kernel matching");
    let common_scale = base.pow(18, 67);
    assert_eq!(base.pow(common_scale, 3), 18);
    let factors = monic.map(|q| poly::scale(f, &q, f.from_u64(common_scale)));
    assert_eq!(
        poly::mul(f, &poly::mul(f, &factors[0], &factors[1]), &factors[2]),
        lift(f, &source.coefficients)
    );
    let richelot = isogeny_algos::genus2::richelot(f, &factors);
    let delta = richelot.delta;
    assert_ne!(delta, f.zero());
    assert!([6, 95].contains(&f.project(delta).unwrap()));
    let normalized = richelot.h.map(|q| poly::scale(f, &q, f.inv(delta)));
    let target = poly::mul(
        f,
        &poly::mul(f, &normalized[0], &normalized[1]),
        &normalized[2],
    )
    .iter()
    .map(|&x| f.project(x).expect("raw coefficientwise F101 descent"))
    .collect::<Vec<_>>();
    assert!([
        vec![25, 98, 69, 38, 52, 39, 76],
        vec![25, 3, 69, 63, 52, 62, 76],
    ]
    .contains(&target));
    assert_eq!(
        poly::gcd(&base, &target, &poly::derivative(&base, &target)),
        vec![1]
    );
    assert_eq!(
        isogeny_algos::genus2::point_counts(&base, &target),
        (120, 10436)
    );
    assert_eq!(
        isogeny_algos::genus2::lpoly(&base, &target).jac_order(101),
        12317
    );
    let target_roots = poly::roots(f, &lift(f, &target), &mut Rng::new(0x2045));
    assert_eq!(target_roots.len(), 6);
    let start = target_roots[0];
    let mut orbit = vec![];
    let mut point = start;
    loop {
        assert!(!orbit.contains(&point));
        orbit.push(point);
        point = f.frobenius(point);
        if point == start {
            break;
        }
    }
    assert_eq!(orbit.len(), 6);
    println!("PASS T20geometry:actualQ20sixteenanti4/256sums+Weil;Gamma20four;P20Q20fourvalid2torsion/16sums/Frobeniusnonzerothreecycle;uniqueof15matchings/3Fpstable/deltas{stable_determinants:?};q{common_scale}uniformcube/delta{:?};rawtarget{target:?};smooth/irreducible6cycle/N1=120/N2=10436/J12317. ExternalCT49credited,notaretwist;wholemapsnext",f.project(delta).unwrap());
    T20Geometry {
        source,
        q20,
        kernel,
        kernel_frobenius,
        roots,
        matching,
        stable_determinants,
        common_scale,
        delta,
        factors,
        normalized,
        target,
    }
}

#[test]
fn actual_t20_primitive_four_kernel_and_raw_target() {
    let f = FpR::new(101, 12);
    let data = t20_geometry(&f);
    if let Ok(path) = std::env::var("CHAINED_T20_GEOMETRY_OUT") {
        let points = data
            .q20
            .iter()
            .map(|&(p, q)| format!("[{},{}]", json_point(p), json_point(q)))
            .collect::<Vec<_>>()
            .join(",");
        let kernel = data
            .kernel
            .iter()
            .map(|d| format!("{{\"u\":{:?},\"v\":{:?}}}", d.u, d.v))
            .collect::<Vec<_>>()
            .join(",");
        let output = format!(
            concat!(
                "{{\"schema\":\"chained-t20-geometry/v1\",\"p\":101,\"extension_degree\":12,",
                "\"field_modulus\":{:?},\"source\":{:?},\"target\":{:?},",
                "\"Q20\":[{}],\"kernel\":[{}],\"kernel_frobenius\":{:?},",
                "\"branch_roots\":{:?},\"matching\":{:?},\"stable_monic_determinants\":{:?},",
                "\"common_scale\":{},\"delta\":{:?},\"factors\":{:?},\"normalized_brackets\":{:?},",
                "\"N1\":120,\"N2\":10436,\"J\":12317,\"standing\":\"unsigned\"}}\n"
            ),
            f.modulus(),
            data.source.coefficients,
            data.target,
            points,
            kernel,
            data.kernel_frobenius,
            data.roots,
            data.matching,
            data.stable_determinants,
            data.common_scale,
            data.delta,
            data.factors,
            data.normalized
        );
        std::fs::write(path, output).expect("write complete T20 geometry data");
    }
}
