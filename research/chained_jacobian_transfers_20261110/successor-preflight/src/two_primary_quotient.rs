//! Whole geometric A_i[4] action from complete B_i[16] quotient lifts.
//! No division by4 on points;all16 lifts per class must agree.
use super::*;

type ClassKey = (Vec<ER>, Vec<ER>);
fn key(d: &legacy::Div<ER>) -> ClassKey {
    (d.u.clone(), d.v.clone())
}
struct Sixteen {
    four: Four,
    points: Vec<Pt<ER>>,
    index: HashMap<Pt<ER>, usize>,
}
fn halves(f: &FpR, e: &Curve<ER>, q: Pt<ER>, two: &[Pt<ER>]) -> Vec<Pt<ER>> {
    let Pt::Aff(x, y) = q else {
        assert_eq!(two.len(), 4);
        return two.to_vec();
    };
    let u = vec![
        f.sub(f.sq(e.a), f.mul(f.from_u64(4), f.mul(e.b, x))),
        f.neg(f.add(
            f.mul(f.from_u64(8), e.b),
            f.mul(f.from_u64(4), f.mul(e.a, x)),
        )),
        f.neg(f.mul(f.from_u64(2), e.a)),
        f.neg(f.mul(f.from_u64(4), x)),
        f.one(),
    ];
    let mut xs = poly::roots(f, &u, &mut Rng::new(0x3916));
    xs.sort();
    assert_eq!(xs.len(), if y == f.zero() { 2 } else { 4 });
    let mut out = vec![];
    for x in xs {
        let rhs = curve::rhs(f, e, x);
        assert_ne!(rhs, f.zero());
        let mut ys = poly::roots(
            f,
            &vec![f.neg(rhs), f.zero(), f.one()],
            &mut Rng::new(0x3917),
        );
        ys.sort();
        assert_eq!(ys.len(), 2);
        for y in ys {
            let p = Pt::Aff(x, y);
            assert!(curve::on_curve(f, e, &p));
            if curve::pmul(f, e, &p, 2) == q {
                out.push(p);
            }
        }
    }
    assert_eq!(out.len(), 4);
    assert_eq!(out.iter().copied().collect::<HashSet<_>>().len(), 4);
    out
}
fn elliptic_sixteen(f: &FpR, model: usize, component: usize) -> Sixteen {
    let four = elliptic_four(f, model, component);
    let e = elliptic(f, model, component);
    let mut points = four.points.clone();
    for order in [8, 16] {
        let mut next = vec![];
        let mut seen = HashSet::new();
        for &q in &points {
            for p in halves(f, &e, q, &four.points[..4]) {
                assert_eq!(curve::pmul(f, &e, &p, 2), q);
                assert_eq!(curve::pmul(f, &e, &p, order), Pt::Inf);
                assert!(seen.insert(p));
                next.push(p);
            }
        }
        assert_eq!(next.len(), (order * order) as usize);
        points = next;
    }
    assert_eq!(points[0], Pt::Inf);
    let index = points
        .iter()
        .enumerate()
        .map(|(i, &p)| (p, i))
        .collect::<HashMap<_, _>>();
    assert_eq!(index.len(), 256);
    let basis = four.basis.map(|p| {
        *points
            .iter()
            .find(|q| curve::pmul(f, &e, q, 4) == p)
            .expect("all4-basis points lift to16torsion")
    });
    let mut coordinates = HashMap::new();
    for a in 0..16_u128 {
        for b in 0..16_u128 {
            let p = curve::padd(
                f,
                &e,
                &curve::pmul(f, &e, &basis[0], a),
                &curve::pmul(f, &e, &basis[1], b),
            );
            assert!(index.contains_key(&p));
            assert!(coordinates.insert(p, (a, b)).is_none());
        }
    }
    assert_eq!(coordinates.len(), 256);
    let frob_basis = basis.map(|p| coordinates[&frobenius(f, p, 1)]);
    let fixed =
        [1, 2, 3, 6, 12].map(|d| points.iter().filter(|&&p| frobenius(f, p, d) == p).count());
    assert_eq!(fixed, [1, 1, 4, 64, 256]);
    println!("PASS elliptic16 T{model} factor{component}:all4halves each/64then256distinct;basis16 P={:?} Q={:?};Frobcolumns={frob_basis:?};fixed1,1,4,64,256 atdegrees1,2,3,6,12", basis[0],basis[1]);
    Sixteen {
        four,
        points,
        index,
    }
}
struct Atlas {
    groups: [Sixteen; 2],
    pulls: [Vec<legacy::Div<ER>>; 2],
    graph: HashMap<Pt<ER>, Pt<ER>>,
    classes: Vec<legacy::Div<ER>>,
    index: HashMap<ClassKey, usize>,
    lifts: Vec<([usize; 2], usize)>,
    sums: Vec<Vec<usize>>,
    frob: Vec<usize>,
    chart: Chart,
}
fn atlas(f: &FpR, model: usize) -> Atlas {
    let flat = Flat(f);
    let groups = [elliptic_sixteen(f, model, 0), elliptic_sixteen(f, model, 1)];
    let chart = Chart::new(f, model, &[]);
    let j = legacy::Hyp {
        f: &flat,
        h: chart.quintic.clone(),
        g: 2,
    };
    let pulls: [Vec<_>; 2] = std::array::from_fn(|component| {
        let ds = groups[component]
            .points
            .iter()
            .map(|&p| {
                let (d, _) = pull_four(f, model, component, p, &chart, &j);
                assert!(j.is_valid(&d));
                assert_eq!(j.mul(&d, 16), j.identity());
                d
            })
            .collect::<Vec<_>>();
        assert_eq!(ds.iter().map(key).collect::<HashSet<_>>().len(), 256);
        ds
    });
    let mut graph = HashMap::new();
    for &p in &groups[0].four.points {
        for &q in &groups[1].four.points {
            if j.add(
                &pulls[0][groups[0].index[&p]],
                &pulls[1][groups[1].index[&q]],
            ) == j.identity()
            {
                assert!(graph.insert(p, q).is_none());
            }
        }
    }
    assert_eq!(graph.len(), 16);
    assert_eq!(graph.values().copied().collect::<HashSet<_>>().len(), 16);
    let mut buckets = HashMap::<Pt<ER>, Vec<usize>>::new();
    for (i, q) in groups[1].points.iter().enumerate() {
        buckets
            .entry(curve::pmul(f, &elliptic(f, model, 1), q, 4))
            .or_default()
            .push(i);
    }
    assert_eq!(buckets.len(), 16);
    assert!(buckets.values().all(|b| b.len() == 16));
    let mut classes = vec![];
    let mut index = HashMap::new();
    let mut lifts = vec![];
    let mut multiplicities = vec![];
    for (a, p) in groups[0].points.iter().enumerate() {
        let p4 = curve::pmul(f, &elliptic(f, model, 0), p, 4);
        for &b in &buckets[&graph[&p4]] {
            let d = j.add(&pulls[0][a], &pulls[1][b]);
            assert!(j.is_valid(&d));
            let k = key(&d);
            let i = *index.entry(k).or_insert_with(|| {
                assert_eq!(j.mul(&d, 4), j.identity());
                let next = classes.len();
                classes.push(d);
                multiplicities.push(0);
                next
            });
            multiplicities[i] += 1;
            lifts.push(([a, b], i));
        }
    }
    assert_eq!(lifts.len(), 4096);
    assert_eq!(classes.len(), 256);
    assert_eq!(classes[0], j.identity());
    assert!(multiplicities.iter().all(|&n| n == 16));
    let sums = classes
        .iter()
        .map(|a| {
            classes
                .iter()
                .map(|b| {
                    let sum = j.add(a, b);
                    assert!(j.is_valid(&sum));
                    index[&key(&sum)]
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut frob = vec![usize::MAX; 256];
    for &([a, b], source) in &lifts {
        let a = groups[0].index[&frobenius(f, groups[0].points[a], 1)];
        let b = groups[1].index[&frobenius(f, groups[1].points[b], 1)];
        let target = index[&key(&j.add(&pulls[0][a], &pulls[1][b]))];
        if frob[source] == usize::MAX {
            frob[source] = target;
        } else {
            assert_eq!(frob[source], target);
        }
    }
    assert_eq!(frob.iter().copied().collect::<HashSet<_>>().len(), 256);
    let fixed = [1, 2, 3, 6, 12].map(|d| {
        (0..256)
            .filter(|&i| {
                let mut v = i;
                for _ in 0..d {
                    v = frob[v];
                }
                v == i
            })
            .count()
    });
    assert_eq!(fixed[0], 1);
    assert_eq!(fixed[1], 1);
    assert_eq!(fixed[4], 256);
    println!("PASS quotient A{model}[4]:4096complete B16lifts/256validdistinct4annclasses/16liftseach;all65536Cantoradditions;Frobeniusallliftsagree/fixed{fixed:?} atdegrees1,2,3,6,12. ChartFrobenius usesoriginalellipticlifts,notcoordinatewisequinticFrob");
    Atlas {
        groups,
        pulls,
        graph,
        classes,
        index,
        lifts,
        sums,
        frob,
        chart,
    }
}

#[test]
fn full_two_primary_quotient_maps() {
    let f = FpR::new(101, 12);
    let flat = Flat(&f);
    let atlases = (0..3).map(|model| atlas(&f, model)).collect::<Vec<_>>();
    let mut maps = vec![];
    let mut elliptic_maps = vec![];
    for (case, &(source, target)) in ENDPOINTS.iter().enumerate() {
        let src = &atlases[source];
        let dst = &atlases[target];
        let steps = stages(&f, case);
        let ep: [Vec<usize>; 2] = std::array::from_fn(|component| {
            src.groups[component]
                .points
                .iter()
                .map(|&p| {
                    let mut base = [Pt::Inf; 2];
                    base[component] = p;
                    let image = apply(&f, source, base, &steps, [1, 1])[component];
                    dst.groups[component].index[&image]
                })
                .collect()
        });
        for (component, indices) in ep.iter().enumerate() {
            assert_eq!(indices.iter().copied().collect::<HashSet<_>>().len(), 256);
            let degree = if case < 4 { 7 } else { 49 };
            for &p in &src.groups[component].four.points {
                for &q in &src.groups[component].four.points {
                    let a = dst.groups[component].points[indices[src.groups[component].index[&p]]];
                    let b = dst.groups[component].points[indices[src.groups[component].index[&q]]];
                    assert_eq!(
                        pairing(&f, &dst.groups[component].four, a, b),
                        f.pow(pairing(&f, &src.groups[component].four, p, q), degree)
                    );
                }
            }
        }
        for (&p, &q) in &src.graph {
            let a = dst.groups[0].points[ep[0][src.groups[0].index[&p]]];
            let b = dst.groups[1].points[ep[1][src.groups[1].index[&q]]];
            assert_eq!(dst.graph[&a], b);
        }
        let j = legacy::Hyp {
            f: &flat,
            h: dst.chart.quintic.clone(),
            g: 2,
        };
        let mut images = vec![usize::MAX; 256];
        for &([a, b], input) in &src.lifts {
            let image = j.add(&dst.pulls[0][ep[0][a]], &dst.pulls[1][ep[1][b]]);
            let output = dst.index[&key(&image)];
            if images[input] == usize::MAX {
                images[input] = output;
            } else {
                assert_eq!(images[input], output, "all16quotientlifts mustagree");
            }
        }
        assert_eq!(images.iter().copied().collect::<HashSet<_>>().len(), 256);
        let wrong = images.iter().map(|&i| dst.sums[i][i]).collect::<Vec<_>>();
        assert_eq!(wrong.iter().copied().collect::<HashSet<_>>().len(), 16);
        assert_eq!(wrong.iter().filter(|&&i| i == 0).count(), 16);
        assert_eq!(
            wrong.iter().zip(&images).filter(|(a, b)| a != b).count(),
            255
        );
        for a in 0..256 {
            assert_eq!(images[src.frob[a]], dst.frob[images[a]]);
            for b in 0..256 {
                assert_eq!(images[src.sums[a][b]], dst.sums[images[a]][images[b]]);
            }
        }
        maps.push(images);
        elliptic_maps.push(ep);
        println!("PASS whole A4 case{case} T{source}→T{target}:all4096lifts/16representations eachagree;256valid4annbijectiveoutputs/all65536grouplaws/Frobeniusnaturality;all16actual4graphimages/pairingsimilitude256pairs eachfactor;wrongquarter2image16/zero16/differences255.No pointdivision4,globalquotient/Picard identity explicit,no oldproducerA4functioncomparison");
    }
    for (case, &(source, target)) in ENDPOINTS.iter().enumerate() {
        let back = case ^ 1;
        let degree = if case < 4 { 7 } else { 49 };
        let j = legacy::Hyp {
            f: &flat,
            h: atlases[source].chart.quintic.clone(),
            g: 2,
        };
        for i in 0..256 {
            assert_eq!(
                maps[back][maps[case][i]],
                atlases[source].index[&key(&j.mul(&atlases[source].classes[i], degree))]
            );
            for component in 0..2 {
                let p = &atlases[source].groups[component].points[i];
                let expected = curve::pmul(&f, &elliptic(&f, source, component), p, degree);
                let actual = elliptic_maps[back][component][elliptic_maps[case][component][i]];
                assert_eq!(atlases[source].groups[component].points[actual], expected);
            }
        }
        assert_eq!(atlases[target].classes.len(), 256);
    }
    for i in 0..256 {
        assert_eq!(maps[4][i], maps[2][maps[0][i]]);
        assert_eq!(maps[5][i], maps[1][maps[3][i]]);
    }
    println!("PASS whole A4 all6norms7/49 and bothcompressedcompositions on256classes;elliptic16all256factor norms7/49;allprevious31units retained,globaldegree notinferredfromfinitecorpus");
}
