//! Actual torsion coordinates along bounded oriented seven-isogeny cycles.
//! Existing Kohel/field/elliptic/Cantor engines only; no timing or solver.
use super::*;
use isogeny_algos::field::Zp;

type Graph = Vec<(Pt<ER>, Pt<ER>)>;
struct Hop {
    map: RatIsogeny<FpR>,
    target: Curve<u64>,
    scale: ER,
}
impl Hop {
    fn eval(&self, f: &FpR, p: Pt<ER>) -> Pt<ER> {
        match self.map.eval(f, &p) {
            Pt::Inf => Pt::Inf,
            Pt::Aff(x, y) => Pt::Aff(
                f.mul(f.sq(self.scale), x),
                f.mul(f.mul(f.sq(self.scale), self.scale), y),
            ),
        }
    }
}
#[derive(Clone)]
struct Presentation {
    curves: [Curve<u64>; 2],
    graph: Graph,
}
fn extension_curve(f: &FpR, e: Curve<u64>) -> Curve<ER> {
    Curve::new(f.from_u64(e.a), f.from_u64(e.b))
}
fn rational_points(f: &Zp, e: &Curve<u64>) -> Vec<Pt<u64>> {
    let mut out = vec![Pt::Inf];
    for x in 0..101 {
        for y in 0..101 {
            let p = Pt::Aff(x, y);
            if curve::on_curve(f, e, &p) {
                out.push(p);
            }
        }
    }
    out
}
fn point_lift(f: &FpR, p: Pt<u64>) -> Pt<ER> {
    match p {
        Pt::Inf => Pt::Inf,
        Pt::Aff(x, y) => Pt::Aff(f.from_u64(x), f.from_u64(y)),
    }
}
fn same_graph(f: &FpR, a: &Graph, b: &Graph) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let set = b.iter().copied().collect::<HashSet<_>>();
    [false, true].into_iter().any(|negative| {
        a.iter()
            .all(|&(p, q)| set.contains(&(p, if negative { curve::neg(f, &q) } else { q })))
    })
}
fn doubled(f: &FpR, p: &Presentation) -> Graph {
    p.graph
        .iter()
        .map(|&(a, b)| {
            (
                curve::pmul(f, &extension_curve(f, p.curves[0]), &a, 2),
                curve::pmul(f, &extension_curve(f, p.curves[1]), &b, 2),
            )
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}
fn actual_graph(f: &FpR, model: usize) -> ([Four; 2], Graph) {
    let groups = [elliptic_four(f, model, 0), elliptic_four(f, model, 1)];
    let flat = Flat(f);
    let chart = Chart::new(f, model, &[]);
    let j = legacy::Hyp {
        f: &flat,
        h: chart.quintic.clone(),
        g: 2,
    };
    let pulled: [Vec<_>; 2] = std::array::from_fn(|i| {
        groups[i]
            .points
            .iter()
            .map(|&p| pull_four(f, model, i, p, &chart, &j).0)
            .collect()
    });
    let mut graph = vec![];
    for a in 0..16 {
        for b in 0..16 {
            if j.add(&pulled[0][a], &pulled[1][b]) == j.identity() {
                graph.push((groups[0].points[a], groups[1].points[b]));
            }
        }
    }
    assert_eq!(graph.len(), 16);
    (groups, graph)
}
fn kernel_eigenvalue(f: &FpR, map: &RatIsogeny<FpR>) -> u128 {
    let mut xs = poly::roots(f, &map.ker, &mut Rng::new(0x0457));
    xs.sort();
    assert_eq!(xs.len(), 3);
    let x = xs[0];
    let y = f
        .sqrt(curve::rhs(f, &map.dom, x))
        .expect("Fp12 contains full seven kernel");
    assert_ne!(y, f.zero());
    let p = Pt::Aff(x, y);
    assert_eq!(curve::pmul(f, &map.dom, &p, 7), Pt::Inf);
    assert_eq!(map.eval(f, &p), Pt::Inf);
    (1..7)
        .find(|&n| curve::pmul(f, &map.dom, &p, n) == frobenius(f, p, 1))
        .unwrap()
}
fn construct_hop(
    f: &FpR,
    source: Curve<u64>,
    component: usize,
    models: &mut Vec<Curve<u64>>,
) -> Hop {
    let base = Zp::new(101);
    let mut chosen = None;
    for edge in crate::seven::construct_edges(&base, &source) {
        let map = RatIsogeny {
            dom: extension_curve(f, edge.map.dom),
            cod: extension_curve(f, edge.map.cod),
            deg: edge.map.deg,
            ker: lift(f, &edge.map.ker),
            num: lift(f, &edge.map.num),
            den: lift(f, &edge.map.den),
        };
        if kernel_eigenvalue(f, &map) == [5, 6][component] {
            assert!(chosen.is_none());
            chosen = Some((map, edge.map.cod, edge.map.ker));
        }
    }
    let (map, raw_target, kernel) = chosen.expect("unique selected Frobenius eigenline");
    let j = curve::jinv(&base, &raw_target);
    let target = if let Some(e) = models.iter().find(|e| curve::jinv(&base, e) == j) {
        *e
    } else {
        models.push(raw_target);
        raw_target
    };
    let scale = crate::seven::checked_isomorphism(&base, &raw_target, &target)
        .expect("same trace twist and checked F101 model scale");
    println!("CYCLE EDGE factor{component} source({},{}) j{} kernel{kernel:?} rawtarget({},{}) canonical({},{}) j{j} scale{scale} eigen{}",source.a,source.b,curve::jinv(&base,&source),raw_target.a,raw_target.b,target.a,target.b,[5,6][component]);
    Hop {
        map,
        target,
        scale: f.from_u64(scale),
    }
}
fn transported_group(f: &FpR, old: &Four, hop: &Hop) -> Four {
    let points = old
        .points
        .iter()
        .map(|&p| hop.eval(f, p))
        .collect::<Vec<_>>();
    assert_eq!(points.iter().copied().collect::<HashSet<_>>().len(), 16);
    let basis = old.basis.map(|p| hop.eval(f, p));
    let e = extension_curve(f, hop.target);
    let mut coordinates = HashMap::new();
    for a in 0..4 {
        for b in 0..4 {
            let p = curve::padd(
                f,
                &e,
                &curve::pmul(f, &e, &basis[0], a),
                &curve::pmul(f, &e, &basis[1], b),
            );
            coordinates.insert(p, (a, b));
        }
    }
    assert_eq!(coordinates.len(), 16);
    let eta = basis_pairing(f, &e, basis[0], basis[1], &points);
    assert_eq!(eta, f.pow(old.eta, 7));
    Four {
        points,
        coordinates,
        basis,
        eta,
    }
}
fn check_graph(f: &FpR, p: &Presentation, groups: &[HashMap<Curve<u64>, Four>; 2]) {
    let set = p.graph.iter().copied().collect::<HashSet<_>>();
    assert_eq!(set.len(), 16);
    for i in 0..2 {
        assert_eq!(
            p.graph
                .iter()
                .map(|q| if i == 0 { q.0 } else { q.1 })
                .collect::<HashSet<_>>()
                .len(),
            16
        );
    }
    for &(a, b) in &p.graph {
        assert!(set.contains(&(frobenius(f, a, 1), frobenius(f, b, 1))));
        for &(c, d) in &p.graph {
            assert!(set.contains(&(
                curve::padd(f, &extension_curve(f, p.curves[0]), &a, &c),
                curve::padd(f, &extension_curve(f, p.curves[1]), &b, &d)
            )));
            assert_eq!(
                f.mul(
                    pairing(f, &groups[0][&p.curves[0]], a, c),
                    pairing(f, &groups[1][&p.curves[1]], b, d)
                ),
                f.one()
            );
        }
    }
}
fn complete_factor_census(models: &[Vec<Curve<u64>>; 2]) {
    let f = Zp::new(101);
    assert_eq!(f.pow(2, 50), 100);
    let mut js: [HashSet<u64>; 2] = std::array::from_fn(|_| HashSet::new());
    for j in 0..101 {
        if j == 0 || j == 1728 % 101 {
            continue;
        }
        let e = curve::from_j(&f, j);
        assert_eq!(curve::jinv(&f, &e), j);
        for e in [e, Curve::new(f.mul(4, e.a), f.mul(8, e.b))] {
            let count = rational_points(&f, &e).len();
            for i in 0..2 {
                if count == [109, 113][i] {
                    assert!(js[i].insert(j));
                }
            }
        }
    }
    for t in 1..101 {
        for e in [Curve::new(t, 0), Curve::new(0, t)] {
            assert!(![109, 113].contains(&rational_points(&f, &e).len()));
        }
    }
    for i in 0..2 {
        assert_eq!(
            js[i],
            models[i].iter().map(|e| curve::jinv(&f, e)).collect()
        );
        assert_eq!(js[i].len(), [4, 3][i]);
        println!("PASS complete F101 factor census factor{i}:{} classes j{:?};all generic j/both twists/all exceptional models",js[i].len(),js[i]);
    }
}
fn subgroup_census(f: &FpR, e: Curve<u64>, g: &Four) {
    let curve = extension_curve(f, e);
    let index = g
        .points
        .iter()
        .enumerate()
        .map(|(i, &p)| (p, i))
        .collect::<HashMap<_, _>>();
    assert_eq!(g.points[0], Pt::Inf);
    let sums: [[usize; 16]; 16] = std::array::from_fn(|i| {
        std::array::from_fn(|j| index[&curve::padd(f, &curve, &g.points[i], &g.points[j])])
    });
    let frob: [usize; 16] = std::array::from_fn(|i| index[&frobenius(f, g.points[i], 1)]);
    let mut additive = 0;
    let mut stable = vec![];
    for mask in (1_u32..65536).step_by(2) {
        let members = (0..16)
            .filter(|&i| mask & (1 << i) != 0)
            .collect::<Vec<_>>();
        if !members
            .iter()
            .all(|&i| members.iter().all(|&j| mask & (1 << sums[i][j]) != 0))
        {
            continue;
        }
        additive += 1;
        if members.iter().all(|&i| mask & (1 << frob[i]) != 0) {
            stable.push(members.len());
        }
    }
    stable.sort();
    assert_eq!(additive, 15);
    assert_eq!(stable, vec![1, 4, 16]);
    println!("PASS actual subgroup census ({},{}):all65536subsets/15additive/3Frobstable sizes{stable:?}",e.a,e.b);
}
fn graph_catalog(
    f: &FpR,
    models: &[Vec<Curve<u64>>; 2],
    groups: &[HashMap<Curve<u64>, Four>; 2],
    history: &[Presentation],
) {
    let mut total_four = 0;
    let mut total_two = 0;
    for &e in &models[0] {
        for &d in &models[1] {
            let a = &groups[0][&e];
            let b = &groups[1][&d];
            let target = extension_curve(f, d);
            let mut gal = 0;
            let mut anti: Vec<Graph> = vec![];
            let mut other: Vec<Graph> = vec![];
            let mut two: Vec<Graph> = vec![];
            for &p in &b.points {
                for &q in &b.points {
                    let graph = a
                        .points
                        .iter()
                        .map(|&x| {
                            let (r, s) = a.coordinates[&x];
                            (
                                x,
                                curve::padd(
                                    f,
                                    &target,
                                    &curve::pmul(f, &target, &p, r),
                                    &curve::pmul(f, &target, &q, s),
                                ),
                            )
                        })
                        .collect::<Graph>();
                    if graph.iter().map(|g| g.1).collect::<HashSet<_>>().len() != 16 {
                        continue;
                    }
                    let set = graph.iter().copied().collect::<HashSet<_>>();
                    if !graph
                        .iter()
                        .all(|&(x, y)| set.contains(&(frobenius(f, x, 1), frobenius(f, y, 1))))
                    {
                        continue;
                    }
                    gal += 1;
                    let isotropic = graph.iter().all(|&(x, y)| {
                        graph.iter().all(|&(z, w)| {
                            f.mul(pairing(f, a, x, z), pairing(f, b, y, w)) == f.one()
                        })
                    });
                    let orbits = if isotropic { &mut anti } else { &mut other };
                    if !orbits.iter().any(|g| same_graph(f, g, &graph)) {
                        orbits.push(graph.clone());
                    }
                    let reduced = doubled(
                        f,
                        &Presentation {
                            curves: [e, d],
                            graph,
                        },
                    );
                    if !two.iter().any(|g| same_graph(f, g, &reduced)) {
                        two.push(reduced);
                    }
                }
            }
            assert_eq!((gal, anti.len(), other.len(), two.len()), (12, 3, 3, 3));
            let walked = history
                .iter()
                .filter(|s| s.curves == [e, d])
                .collect::<Vec<_>>();
            assert_eq!(walked.len(), 3);
            for g in &anti {
                assert_eq!(
                    walked.iter().filter(|s| same_graph(f, g, &s.graph)).count(),
                    1
                );
            }
            for g in &two {
                assert_eq!(
                    walked
                        .iter()
                        .filter(|s| same_graph(f, g, &doubled(f, s)))
                        .count(),
                    1
                );
            }
            total_four += anti.len();
            total_two += two.len();
            println!("PASS actual graph catalog E({},{}) F({},{}):12Gal4isos/6anti/3anti-sign-orbits/3other-sign-orbits/3Gal2graphs;walk covers eachanti4and2exactlyonce",e.a,e.b,d.a,d.b);
        }
    }
    assert_eq!((total_four, total_two), (36, 36));
}
fn cm_action(f: &FpR, e: Curve<u64>, p: Pt<ER>, component: usize, negative: bool) -> Pt<ER> {
    let e = extension_curve(f, e);
    let (a, b) = [(31, 5), (22, 1)][component];
    let out = curve::padd(
        f,
        &e,
        &curve::pmul(f, &e, &p, a),
        &curve::pmul(f, &e, &frobenius(f, p, 1), b),
    );
    if negative {
        curve::neg(f, &out)
    } else {
        out
    }
}
fn closed_scalar_control(
    f: &FpR,
    groups: &[HashMap<Curve<u64>, Four>; 2],
    paths: &[Vec<Curve<u64>>; 2],
    hops: &[HashMap<Curve<u64>, Hop>; 2],
    last: &Presentation,
    original: &Presentation,
) {
    let base = Zp::new(101);
    assert_eq!(31_i64 * 31 - 7 * 31 * 5 + 101 * 25, 7_i64.pow(4));
    assert_eq!(22_i64 * 22 - 11 * 22 + 101, 7_i64.pow(3));
    let mut maps: [HashMap<Pt<ER>, Pt<ER>>; 2] = std::array::from_fn(|_| HashMap::new());
    let mut scalars = [0_u128; 2];
    for i in 0..2 {
        let source = original.curves[i];
        let e = extension_curve(f, source);
        let period = [4, 3][i];
        let points = rational_points(&base, &source)
            .into_iter()
            .map(|p| point_lift(f, p))
            .collect::<Vec<_>>();
        assert_eq!(points.len(), [109, 113][i]);
        let evaluate = |mut p: Pt<ER>, n: usize| {
            for curve in &paths[i][..n] {
                p = hops[i][curve].eval(f, p);
            }
            p
        };
        let signs = [false, true]
            .into_iter()
            .filter(|&negative| {
                points
                    .iter()
                    .chain(&groups[i][&source].points)
                    .all(|&p| evaluate(p, period) == cm_action(f, source, p, i, negative))
            })
            .collect::<Vec<_>>();
        assert_eq!(signs.len(), 1);
        let prime = [109_u128, 113][i];
        let cm = [36_u128, 23][i];
        let cm = if signs[0] { prime - cm } else { cm };
        let mut expected = 1;
        for _ in 0..36 / period {
            expected = expected * cm % prime;
        }
        for p in points {
            let output = evaluate(p, 36);
            assert_eq!(output, curve::pmul(f, &e, &p, expected));
            maps[i].insert(p, output);
        }
        scalars[i] = expected;
        println!("PASS actual CM cycle factor{i}:period{period}/negative{}/all16geometric4+{}rational points;36hop publicfactor scalar{expected}",signs[0],[109,113][i]);
    }
    let first = original.graph.iter().copied().collect::<HashSet<_>>();
    let negative = !last.graph.iter().all(|p| first.contains(p));
    assert!(last
        .graph
        .iter()
        .all(|&(p, q)| first.contains(&(p, if negative { curve::neg(f, &q) } else { q }))));
    if negative {
        scalars[1] = (113 - scalars[1]) % 113;
        for value in maps[1].values_mut() {
            *value = curve::neg(f, value);
        }
    }
    let scalar = (0_u128..12317)
        .find(|n| n % 109 == scalars[0] && n % 113 == scalars[1])
        .unwrap();
    let flat = Flat(f);
    let chart = Chart::new(f, 0, &[]);
    let j = legacy::Hyp {
        f: &flat,
        h: chart.quintic.clone(),
        g: 2,
    };
    let classes = decode_classes(f, &fixture("t0-classes.bin"), 0).unwrap();
    let mut cache = RootCache::new();
    let mut differences = 0;
    for input in &classes {
        let d = chart.class(f, &j, input);
        let points = norms(f, 0, input, &mut cache);
        let moved = std::array::from_fn(|i| {
            curve::pmul(f, &elliptic(f, 0, i), &maps[i][&points[i]], [82, 85][i])
        });
        let output = sum_pulls(f, 0, moved, &chart, &j);
        assert_eq!(output, j.mul(&d, scalar));
        differences += usize::from(output != j.mul(&d, scalar + 1));
    }
    assert_eq!(differences, 12316);
    println!("PASS closed36 Jacobian rational control:all12317 oldT0classes outerK-factorwalk-H/4 equals publicCRTscalar{scalar};factorresidues{scalars:?};relativeFsign{negative};scalar+1diff{differences}. Finite rational result,globalCM identity separately written,no DLP solver or timing");
}
#[test]
fn actual_oriented_cycles_and_gluing_closure() {
    let f = FpR::new(101, 12);
    let mut groups: [HashMap<Curve<u64>, Four>; 2] = std::array::from_fn(|_| HashMap::new());
    let mut old = vec![];
    for (model, curves) in CURVES.iter().enumerate() {
        let (gs, graph) = actual_graph(&f, model);
        for (i, g) in gs.into_iter().enumerate() {
            groups[i].insert(Curve::new(curves[i][0], curves[i][1]), g);
        }
        old.push(Presentation {
            curves: curves.map(|[a, b]| Curve::new(a, b)),
            graph,
        });
    }
    let mut models: [Vec<_>; 2] =
        std::array::from_fn(|i| old.iter().map(|p| p.curves[i]).collect());
    let mut hops: [HashMap<Curve<u64>, Hop>; 2] = std::array::from_fn(|_| HashMap::new());
    let mut paths: [Vec<Curve<u64>>; 2] = std::array::from_fn(|_| vec![]);
    let mut state = old[0].clone();
    let mut history = vec![];
    for step in 0..36 {
        check_graph(&f, &state, &groups);
        if let Some(expected) = old.get(step) {
            assert_eq!(state.curves, expected.curves);
            assert!(same_graph(&f, &state.graph, &expected.graph));
        }
        for previous in &history {
            let previous: &Presentation = previous;
            if state.curves == previous.curves {
                assert!(!same_graph(&f, &state.graph, &previous.graph));
                assert!(!same_graph(
                    &f,
                    &doubled(&f, &state),
                    &doubled(&f, previous)
                ));
            }
        }
        println!("CYCLE STATE {step}:E({},{}) F({},{});16anti4/4two/Frobeniusstable/256sums+Weil;new bothgraphclasses",state.curves[0].a,state.curves[0].b,state.curves[1].a,state.curves[1].b);
        history.push(state.clone());
        let mut targets = state.curves;
        for i in 0..2 {
            let source = state.curves[i];
            paths[i].push(source);
            if !hops[i].contains_key(&source) {
                let hop = construct_hop(&f, source, i, &mut models[i]);
                let transported = transported_group(&f, &groups[i][&source], &hop);
                if let Some(previous) = groups[i].get(&hop.target) {
                    assert_eq!(
                        previous.points.iter().copied().collect::<HashSet<_>>(),
                        transported.points.iter().copied().collect()
                    );
                } else {
                    groups[i].insert(hop.target, transported);
                }
                let target = &groups[i][&hop.target];
                let source_group = &groups[i][&source];
                for &p in &source_group.points {
                    for &q in &source_group.points {
                        assert_eq!(
                            pairing(&f, target, hop.eval(&f, p), hop.eval(&f, q)),
                            f.pow(pairing(&f, source_group, p, q), 7)
                        );
                    }
                }
                hops[i].insert(source, hop);
            }
            targets[i] = hops[i][&source].target;
        }
        let graph = state
            .graph
            .iter()
            .map(|&(a, b)| {
                (
                    hops[0][&state.curves[0]].eval(&f, a),
                    hops[1][&state.curves[1]].eval(&f, b),
                )
            })
            .collect();
        state = Presentation {
            curves: targets,
            graph,
        };
    }
    assert_eq!(state.curves, old[0].curves);
    assert!(same_graph(&f, &state.graph, &old[0].graph));
    assert!(same_graph(&f, &doubled(&f, &state), &doubled(&f, &old[0])));
    for n in [12, 24] {
        assert_eq!(history[n].curves, old[0].curves);
        assert!(!same_graph(&f, &history[n].graph, &old[0].graph));
    }
    for i in 0..2 {
        let period = [4, 3][i];
        assert_eq!(paths[i][period], paths[i][0]);
        for a in 0..period {
            for b in a + 1..period {
                assert_ne!(paths[i][a], paths[i][b]);
            }
        }
    }
    complete_factor_census(&models);
    for i in 0..2 {
        for &e in &models[i] {
            subgroup_census(&f, e, &groups[i][&e]);
        }
    }
    graph_catalog(&f, &models, &groups, &history);
    closed_scalar_control(&f, &groups, &paths, &hops, &state, &old[0]);
    println!("PASS actual oriented cycles:elliptic4/3;pair12;actual4graph36/2graph36;first2matchactualT1/T2;all12factorpairs/36anti4+36two classes exhausted;no new curve models,global AV classification separate");
}
