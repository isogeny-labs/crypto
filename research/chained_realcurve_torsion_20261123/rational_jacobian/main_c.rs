#[path = "../elliptic_chain/base27.rs"]
mod base27;
#[path = "../elliptic_chain/chart_b.rs"]
mod chart;
#[path = "../elliptic_chain/field_b.rs"]
mod field;
#[path = "../elliptic_chain/inherited.rs"]
mod inherited;
#[path = "kernel_b.rs"]
mod kernel;

use chart::{key, Chart, Divisor};
use field::{Field, Q};
use inherited::{peval, Div, Fld};
use kernel::Kernel;
use std::collections::{BTreeMap, BTreeSet};

const ORIGINAL: [u32; 7] = [1, 0, 1, 0, 0, 1, 1];
const TARGET0: [u32; 7] = [1, 0, 1, 1, 0, 1, 1];
const TARGET2: [u32; 7] = [1, 0, 1, 2, 0, 1, 2];
const G0: [[u32; 3]; 3] = [[4, 24, 1], [5, 19, 1], [3, 21, 1]];
const G1: [[u32; 3]; 3] = [[16, 23, 1], [13, 26, 1], [9, 18, 1]];
const G2: [[u32; 3]; 3] = [[25, 20, 1], [20, 22, 1], [22, 25, 1]];

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Point {
    a: u32,
    b: u32,
    y: u32,
}

impl Point {
    fn frobenius(self, f: &Field) -> Self {
        Self {
            a: f.pow(self.a, 3),
            b: f.pow(self.b, 3),
            y: f.pow(self.y, 3),
        }
    }
}

struct Cohort {
    points: Vec<Point>,
    pairs: Vec<(usize, usize, usize)>,
    classes: Vec<Divisor>,
    index: BTreeMap<Vec<u32>, usize>,
    addition: Vec<Vec<usize>>,
}

fn build_cohort(label: &str, chart: &Chart<'_>) -> Cohort {
    let f = chart.h.f;
    let f9: Vec<_> = (0..Q).filter(|&x| f.pow(x, 9) == x).collect();
    assert_eq!(f9.len(), 9);
    let mut points = Vec::new();
    for &x in &f9 {
        for &y in &f9 {
            if f.mul(&y, &y) == peval(f, &chart.model, &x) {
                points.push(Point { a: x, b: 1, y });
            }
        }
    }
    for &y in &f9 {
        if f.mul(&y, &y) == *chart.model.last().unwrap() {
            points.push(Point { a: 1, b: 0, y });
        }
    }
    points.sort();
    println!("ACTUAL_F9_POINTS label={label} points={points:?}");
    assert_eq!(points.len(), 12);
    assert_eq!(points.iter().filter(|&&p| p.frobenius(f) == p).count(), 6);
    assert!(points.iter().all(|p| p.y != 0));
    let permutation: Vec<_> = points
        .iter()
        .map(|&p| points.iter().position(|&q| q == p.frobenius(f)).unwrap())
        .collect();
    let mut class_map: BTreeMap<Vec<u32>, (Divisor, usize)> = BTreeMap::new();
    let mut raw_pairs = Vec::new();
    for (i, &p) in points.iter().enumerate() {
        for (j, &q) in points.iter().enumerate().skip(i) {
            let mut image = [permutation[i], permutation[j]];
            image.sort();
            if image != [i, j] {
                continue;
            }
            let a = chart.point_homogeneous(p.a, p.b, p.y);
            let b = chart.point_homogeneous(q.a, q.b, q.y);
            let d = chart.h.add(&a, &b);
            assert!(kernel::eligible(chart, &d));
            let k = key(&d);
            class_map
                .entry(k.clone())
                .and_modify(|entry| entry.1 += 1)
                .or_insert((d, 1));
            raw_pairs.push((i, j, k));
        }
    }
    assert_eq!(raw_pairs.len(), 24);
    assert_eq!(class_map.len(), 21);
    let mut classes = Vec::new();
    let mut index = BTreeMap::new();
    for (k, (d, multiplicity)) in class_map {
        let identity = chart.h.is_identity(&d);
        assert_eq!(multiplicity, if identity { 4 } else { 1 });
        assert!(identity || d.u.len() == 3);
        if !identity {
            let support = kernel::support(chart, &d).map(|(a, b, y)| {
                if b == 0 {
                    Point {
                        a: 1,
                        b: 0,
                        y: f.mul(&y, &f.inv(&f.pow(a, 3))),
                    }
                } else {
                    let inverse = f.inv(&b);
                    Point {
                        a: f.mul(&a, &inverse),
                        b: 1,
                        y: f.mul(&y, &f.pow(inverse, 3)),
                    }
                }
            });
            let mut support_ids = support.map(|p| points.iter().position(|&q| q == p).unwrap());
            support_ids.sort();
            assert!(raw_pairs
                .iter()
                .any(|(i, j, old_key)| [*i, *j] == support_ids && *old_key == k));
        }
        let i = classes.len();
        println!("RATIONAL_CLASS label={label} id={i} multiplicity={multiplicity} divisor={k:?} F3_fixed=pass");
        index.insert(k, i);
        classes.push(d);
    }
    let pairs = raw_pairs
        .into_iter()
        .map(|(i, j, k)| {
            let c = index[&k];
            println!("EFFECTIVE_PAIR label={label} first={i} second={j} class={c}");
            (i, j, c)
        })
        .collect();
    let identity = index[&key(&chart.h.identity())];
    let mut addition = vec![vec![0; 21]; 21];
    for (i, a) in classes.iter().enumerate() {
        assert_eq!(chart.h.add(a, &chart.h.identity()), *a);
        assert_eq!(chart.h.add(a, &chart.h.neg(a)), chart.h.identity());
        for (j, b) in classes.iter().enumerate() {
            let sum = chart.h.add(a, b);
            assert!(chart.h.is_valid(&sum));
            assert_eq!(chart.frobenius(&sum), sum);
            assert_eq!(chart.h.add(b, a), sum);
            addition[i][j] = index[&key(&sum)];
        }
    }
    for i in 0..21 {
        assert_eq!(addition[i][identity], i);
        for j in 0..21 {
            for k in 0..21 {
                assert_eq!(addition[addition[i][j]][k], addition[i][addition[j][k]]);
            }
        }
    }
    let mut orders = BTreeMap::new();
    for d in &classes {
        let order = (1..=21)
            .find(|&n| chart.h.is_identity(&chart.h.mul(d, n)))
            .unwrap();
        *orders.entry(order).or_insert(0) += 1;
    }
    assert_eq!(orders, BTreeMap::from([(1, 1), (3, 2), (7, 6), (21, 12)]));
    let nonrational = points
        .iter()
        .map(|p| chart.point_homogeneous(p.a, p.b, p.y))
        .find(|d| !kernel::eligible(chart, d))
        .unwrap();
    assert!(chart.h.is_valid(&nonrational));
    assert_ne!(chart.frobenius(&nonrational), nonrational);
    let positive = classes.iter().find(|d| !chart.h.is_identity(d)).unwrap();
    assert!(kernel::eligible(chart, positive));
    let invalid = [1, 2]
        .into_iter()
        .find_map(|change| {
            let mut v = positive.v.clone();
            if v.is_empty() {
                v.push(0);
            }
            v[0] = f.add(&v[0], &change);
            let d = Div {
                u: positive.u.clone(),
                v,
            };
            let valid = chart.h.is_valid(&d);
            println!(
                "INPUT_MUTANT label={label} constant_change={change} base={:?} mutated={:?} valid={valid}",
                key(positive),
                key(&d)
            );
            (!valid).then_some(d)
        })
        .unwrap();
    assert!(!kernel::eligible(chart, &invalid));
    println!("COHORT label={label} F9_points=12 F3_points=6 stable_degree2_divisors=24 distinct_classes=21 identity_multiplicity=4 addition_pairs=441 table_associativity_triples=9261 orders={orders:?} real_invalid_and_nonrational_controls=pass");
    Cohort {
        points,
        pairs,
        classes,
        index,
        addition,
    }
}

fn qualify_map(
    label: &str,
    source: &Chart<'_>,
    a: &Cohort,
    target: &Chart<'_>,
    b: &Cohort,
    kernel: &Kernel,
) -> Vec<Divisor> {
    let images: Vec<_> = a
        .classes
        .iter()
        .map(|d| kernel::map(source, target, kernel, d))
        .collect();
    let indices: Vec<_> = images.iter().map(|d| b.index[&key(d)]).collect();
    assert_eq!(indices.iter().copied().collect::<BTreeSet<_>>().len(), 21);
    for (i, d) in images.iter().enumerate() {
        assert_eq!(target.frobenius(d), *d);
        assert_eq!(target.h.is_identity(d), source.h.is_identity(&a.classes[i]));
        assert_eq!(
            kernel::map(source, target, kernel, &source.frobenius(&a.classes[i])),
            target.frobenius(d)
        );
        for j in 0..21 {
            assert_eq!(
                indices[a.addition[i][j]],
                b.addition[indices[i]][indices[j]]
            );
        }
        for scalar in 0..21 {
            let expected = a.index[&key(&source.h.mul(&a.classes[i], scalar))];
            assert_eq!(target.h.mul(d, scalar), images[expected]);
        }
        println!("JACOBIAN_IMAGE map={label} source_class={i} target_class={} divisor={:?} F3_fixed=pass", indices[i], key(d));
    }
    for &(i, j, c) in &a.pairs {
        let p = a.points[i];
        let q = a.points[j];
        let x = kernel::image_point(target, kernel, p.a, p.b, p.y);
        let y = kernel::image_point(target, kernel, q.a, q.b, q.y);
        assert_eq!(target.h.add(&x, &y), images[c]);
    }
    println!("JACOBIAN_MAP map={label} rational_classes=21 bijective=pass addition_pairs=441 scalar_controls=441 independent_effective_pair_controls=24 F3_compatibility=pass scalar_labels_used_by_map=0");
    images
}

fn dual_composite(
    label: &str,
    source: &Chart<'_>,
    a: &Cohort,
    target: &Chart<'_>,
    forward: &[Divisor],
    dual: &Kernel,
) {
    let mut wrong_sign_rejected = 0;
    for (i, d) in a.classes.iter().enumerate() {
        let raw = kernel::map(target, source, dual, &forward[i]);
        let doubled = source.h.mul(d, 2);
        assert_eq!(raw, source.h.neg(&doubled));
        assert_eq!(source.h.neg(&raw), doubled);
        if !source.h.is_identity(d) {
            assert_ne!(raw, doubled);
            wrong_sign_rejected += 1;
        } else {
            assert_eq!(raw, doubled);
        }
        println!("DUAL_COMPOSITE map={label} source_class={i} raw_divisor={:?} expected_exact_neg2=pass divided_by2_expected_pos2=pass", key(&raw));
    }
    assert_eq!(wrong_sign_rejected, 20);
    println!("DUAL_SIGN_CONTROL map={label} exact_raw_minus2_classes=21 wrong_plus2_rejected_nonidentity=20 identity_positive=1 restored_pos2=21");
}

fn main() {
    let f = Field::new();
    println!(
        "INHERITED_FIELD q={Q} generator={} setup_controls=pass",
        f.generator
    );
    let source = Chart::new(&f, &ORIGINAL);
    source.controls();
    let a = build_cohort("C", &source);
    assert!(Kernel::new(&f, &ORIGINAL, G1.map(|p| p.to_vec())).is_err());
    let mut nondual_total = 0;
    for (id, g, expected) in [(0, G0, TARGET0), (2, G2, TARGET2)] {
        let first = Kernel::new(&f, &ORIGINAL, g.map(|p| p.to_vec())).unwrap();
        assert_eq!(first.codomain, expected);
        let target = Chart::new(&f, &first.codomain);
        target.controls();
        let b = build_cohort(&format!("C{id}"), &target);
        let dual = first.dual(&f);
        nondual_total += kernel::stable_matchings(&format!("C{id}"), &target, &dual);
        let forward = qualify_map(&format!("C_to_C{id}"), &source, &a, &target, &b, &first);
        let reverse = qualify_map(
            &format!("C{id}_rawdual_to_C"),
            &target,
            &b,
            &source,
            &a,
            &dual,
        );
        dual_composite(
            &format!("C_via_C{id}_to_C"),
            &source,
            &a,
            &target,
            &forward,
            &dual,
        );
        dual_composite(
            &format!("C{id}_via_C_to_C{id}"),
            &target,
            &b,
            &source,
            &reverse,
            &first,
        );
    }
    println!("SUMMARY genuine_rational_Jacobian_cohorts=3 each_classes=21 each_effective_pairs=24 each_addition_pairs=441 each_table_associativity_triples=9261 genuine_Jacobian_maps=4 each_bijective_classes=21 each_map_addition_pairs=441 total_map_addition_pairs=1764 each_scalar_controls=441 exact_raw_minus2_composite_classes=84 wrong_plus2_nonidentity_controls=80 target_matchings_examined=30 nondual_rational_second_kernels={nondual_total} product_endpoint_rejected=1 transfer_scalar_labels=0 actual_weil8=not_computed costs=unmeasured");
}
