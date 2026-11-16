//! Full geometric7-torsion action using the accepted separate evaluator.
//! Norms here start with its reduced quintic classes,including the basepoint.
use super::*;
use isogeny_algos::field::Zp;

type Key = (Vec<ER>, Vec<ER>);
fn key(d: &legacy::Div<ER>) -> Key {
    (d.u.clone(), d.v.clone())
}
fn fixed_six(f: &FpR, mut x: ER) -> bool {
    let original = x;
    for _ in 0..6 {
        x = f.frobenius(x);
    }
    x == original
}
fn full_elliptic_seven(f: &FpR, model: usize, component: usize) -> Vec<Pt<ER>> {
    let [a, b] = CURVES[model][component];
    let e = elliptic(f, model, component);
    let edges = crate::seven::construct_edges(&Zp::new(101), &Curve::new(a, b));
    assert_eq!(edges.len(), 2);
    let generators = edges
        .iter()
        .map(|edge| {
            let mut roots = poly::roots(f, &lift(f, &edge.map.ker), &mut Rng::new(0x377));
            roots.sort();
            assert_eq!(roots.len(), 3);
            let x = roots[0];
            let rhs = f.add(f.add(f.mul(f.sq(x), x), f.mul(e.a, x)), e.b);
            let mut ys = poly::roots(
                f,
                &vec![f.neg(rhs), f.zero(), f.one()],
                &mut Rng::new(0x378),
            );
            ys.sort();
            assert_eq!(ys.len(), 2);
            let p = Pt::Aff(x, ys[0]);
            assert!(curve::on_curve(f, &e, &p));
            assert_eq!(curve::pmul(f, &e, &p, 7), Pt::Inf);
            p
        })
        .collect::<Vec<_>>();
    let mut points = vec![];
    let mut seen = HashSet::new();
    for a in 0..7 {
        for b in 0..7 {
            let p = curve::padd(
                f,
                &e,
                &curve::pmul(f, &e, &generators[0], a),
                &curve::pmul(f, &e, &generators[1], b),
            );
            assert!(seen.insert(p));
            assert_eq!(curve::pmul(f, &e, &p, 7), Pt::Inf);
            if let Pt::Aff(x, y) = p {
                assert!(fixed_six(f, x) && fixed_six(f, y));
            }
            points.push(p);
        }
    }
    assert_eq!(points.len(), 49);
    assert_eq!(points[0], Pt::Inf);
    points
}

fn image_finite(f: &FpR, model: usize, component: usize, x: ER, y: ER) -> Pt<ER> {
    assert_eq!(f.sq(y), poly::eval(f, &lift(f, &MODELS[model]), x));
    let c = &COVERS[model][component];
    let q = poly::eval(f, &lift(f, &c.q), x);
    let n = poly::eval(f, &lift(f, &c.n), x);
    let m = poly::eval(f, &lift(f, &c.m), x);
    let p = if q == f.zero() {
        assert_ne!(n, f.zero());
        assert_ne!(m, f.zero());
        Pt::Inf
    } else {
        Pt::Aff(f.div(n, f.sq(q)), f.div(f.mul(y, m), f.pow(q, 3)))
    };
    assert!(curve::on_curve(f, &elliptic(f, model, component), &p));
    p
}
fn quintic_norms(
    f: &FpR,
    model: usize,
    chart: &Chart,
    j: &legacy::Hyp<'_, Flat<'_>>,
    d: &legacy::Div<ER>,
    cache: &mut RootCache,
) -> [Pt<ER>; 2] {
    assert!(j.is_valid(d) && d.u.len() <= 3);
    let degree = d.u.len() - 1;
    let roots = cache.entry(d.u.clone()).or_insert_with(|| {
        let mut roots = poly::roots(f, &d.u, &mut Rng::new(0x379));
        if degree == 2 && roots.len() == 1 {
            roots.push(roots[0]);
        }
        assert_eq!(roots.len(), degree, "complete quintic support splitting");
        let product = roots.iter().fold(vec![f.one()], |a, &x| {
            poly::mul(f, &a, &vec![f.neg(x), f.one()])
        });
        assert_eq!(product, d.u);
        roots
    });
    std::array::from_fn(|component| {
        let e = elliptic(f, model, component);
        let mut sum = Pt::Inf;
        for &w in roots.iter() {
            let z = poly::eval(f, &d.v, w);
            assert_eq!(f.sq(z), poly::eval(f, &chart.quintic, w));
            let p = if w == f.zero() {
                let c = &COVERS[model][component];
                let q = Pt::Aff(f.from_u64(c.infinity[0]), f.from_u64(c.infinity[1]));
                if z == f.sq(chart.l) {
                    q
                } else {
                    assert_eq!(z, f.neg(f.sq(chart.l)));
                    curve::neg(f, &q)
                }
            } else {
                let x = f.add(chart.a, f.div(chart.l, w));
                let y = f.div(f.mul(chart.l, z), f.pow(w, 3));
                image_finite(f, model, component, x, y)
            };
            sum = curve::padd(f, &e, &sum, &p);
        }
        let base = image_finite(f, model, component, chart.a, f.zero());
        let base = curve::pmul(f, &e, &base, degree as u128);
        curve::padd(f, &e, &sum, &curve::neg(f, &base))
    })
}

struct SevenGroup {
    chart: Chart,
    classes: Vec<legacy::Div<ER>>,
    norms: Vec<[Pt<ER>; 2]>,
    points: Vec<[Pt<ER>; 2]>,
    indices: HashMap<Key, usize>,
}
fn group(f: &FpR, model: usize) -> SevenGroup {
    let chart = Chart::new(f, model, &[]);
    assert!(fixed_six(f, chart.a) && fixed_six(f, chart.l));
    let flat = Flat(f);
    let j = legacy::Hyp {
        f: &flat,
        h: chart.quintic.clone(),
        g: 2,
    };
    let points = [
        full_elliptic_seven(f, model, 0),
        full_elliptic_seven(f, model, 1),
    ];
    let pulls: [Vec<legacy::Div<ER>>; 2] = std::array::from_fn(|i| {
        points[i]
            .iter()
            .map(|p| pull(f, model, i, p, &chart, &j))
            .collect()
    });
    let mut classes = vec![];
    let mut norms = vec![];
    let mut product = vec![];
    let mut indices = HashMap::new();
    let mut cache = RootCache::new();
    for (a, p) in points[0].iter().enumerate() {
        for (b, q) in points[1].iter().enumerate() {
            let d = j.add(&pulls[0][a], &pulls[1][b]);
            assert!(j.is_valid(&d));
            assert!(j.is_identity(&j.mul(&d, 7)));
            assert!(d.u.iter().chain(&d.v).all(|&x| fixed_six(f, x)));
            assert!(indices.insert(key(&d), classes.len()).is_none());
            let n = quintic_norms(f, model, &chart, &j, &d, &mut cache);
            assert_eq!(
                n,
                [
                    curve::pmul(f, &elliptic(f, model, 0), p, 4),
                    curve::pmul(f, &elliptic(f, model, 1), q, 4)
                ]
            );
            classes.push(d);
            norms.push(n);
            product.push([*p, *q]);
        }
    }
    assert_eq!(classes.len(), 2401);
    assert_eq!(indices.len(), 2401);
    assert!(j.is_identity(&classes[0]));
    println!("PASS T{model} complete geometric7 source:49points per elliptic factor;2401actual K_i classes valid/distinct/7annihilated/Frobenius6fixed;independent pointwise quintic norms equal4times every embedded product,all supports/basepoint/infinities retained");
    SevenGroup {
        chart,
        classes,
        norms,
        points: product,
        indices,
    }
}

#[test]
fn complete_geometric_seven_torsion_action_and_chain_controls() {
    let f = FpR::new(101, 12);
    let flat = Flat(&f);
    let groups: [SevenGroup; 3] = std::array::from_fn(|m| group(&f, m));
    let mut kernels: [HashSet<usize>; 6] = std::array::from_fn(|_| HashSet::new());
    let m1 = 1_u64 + 101 * 12317;
    let m0 = 1_u64 + 4 * 101 * 12317;
    let deeper = 1_u64 + 7 * 101 * 12317;
    assert_eq!((m1, m0, deeper), (1244018, 4976069, 8708120));
    assert_eq!((deeper % 101, deeper % 12317, deeper % 7), (1, 1, 1));
    for (case, &(source, target)) in ENDPOINTS.iter().enumerate() {
        let src = &groups[source];
        let dst = &groups[target];
        let source_j = legacy::Hyp {
            f: &flat,
            h: src.chart.quintic.clone(),
            g: 2,
        };
        let target_j = legacy::Hyp {
            f: &flat,
            h: dst.chart.quintic.clone(),
            g: 2,
        };
        let steps = stages(&f, case);
        let reverse = if case == 0 || case == 2 {
            Some(stages(&f, case + 1))
        } else {
            None
        };
        let mut images = HashSet::new();
        let mut shadow_differences = 0;
        let mut backtracking_zero = 0;
        for i in 0..2401 {
            let points = apply(&f, source, src.norms[i], &steps, [2, 2]);
            let direct = apply(&f, source, src.points[i], &steps, [1, 1]);
            assert_eq!(points, direct);
            let image = sum_pulls(&f, target, points, &dst.chart, &target_j);
            assert!(target_j.is_valid(&image));
            assert!(target_j.is_identity(&target_j.mul(&image, 7)));
            let image_key = key(&image);
            assert!(dst.indices.contains_key(&image_key));
            images.insert(image_key.clone());
            if target_j.is_identity(&image) {
                kernels[case].insert(i);
            }
            // Every image is checked7-annihilated;public scalar residue reduction
            // therefore implements the shadow controls exactly on this domain.
            let zero = target_j.mul(&image, u128::from(m0 % 7));
            assert!(target_j.is_identity(&zero));
            let neg = target_j.mul(&image, u128::from(m1 % 7));
            assert_eq!(key(&neg), key(&target_j.neg(&image)));
            if key(&neg) != image_key {
                shadow_differences += 1;
            }
            assert_eq!(
                key(&target_j.mul(&image, u128::from(deeper % 7))),
                image_key
            );
            if let Some(reverse) = &reverse {
                let middle = dst.indices[&image_key];
                let back_points = apply(&f, target, dst.norms[middle], reverse, [2, 2]);
                let back = sum_pulls(&f, source, back_points, &src.chart, &source_j);
                assert!(source_j.is_identity(&back));
                backtracking_zero += 1;
            }
        }
        assert_eq!(kernels[case].len(), 49);
        assert_eq!(images.len(), 49);
        assert_eq!(shadow_differences, 2352);
        if reverse.is_some() {
            assert_eq!(backtracking_zero, 2401);
        }
        println!("PASS fullJ7 case{case} T{source}→T{target}:all2401 actual7torsion classes match direct elliptic route;kernel49/image49;allimages7annihilated;CT38m0zero2401/m1neg2352differences;deeper8708120passesall7controls;backtrackingzero{backtracking_zero}. Shared existing arithmetic/globaldegree/Picard dependencies explicit,no timing claim");
    }
    assert_eq!(kernels[4], kernels[0]);
    assert_eq!(kernels[5], kernels[3]);
    assert_eq!(kernels[1].intersection(&kernels[2]).count(), 1);
    println!("PASS independent nondual chain controls:sharedT1 firstadjoint/secondforward7kernels intersect only identity;compressedforward7kernel equalsfirstforward49 and compressedreverse equalssecondadjoint49;single-hop backtrackingkillsall2401. Conditionaldegree2401/norm49 pluskernel7size49 impliesZ49×Z49 by finiteabelianclassification,not a nativeFp42/49torsion enumeration;globaldegree blindspot remains");
    assert_eq!(
        groups.iter().map(|g| g.classes.len()).collect::<Vec<_>>(),
        vec![2401; 3]
    );
}
