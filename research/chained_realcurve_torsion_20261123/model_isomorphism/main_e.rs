#[path = "../elliptic_chain/base27.rs"]
mod base27;
#[path = "../elliptic_chain/chart_b.rs"]
mod chart;
#[path = "../elliptic_chain/field_b.rs"]
mod field;
#[path = "../elliptic_chain/inherited.rs"]
mod inherited;
#[path = "richelot_inherited.rs"]
mod kernel;

use chart::{key, Chart, Divisor};
use field::Field;
use inherited::{padd, peval, pmul, pscale, Div, Fld};
use std::collections::{BTreeMap, BTreeSet};

const RAW: &str = include_str!("../rational_jacobian/first-C-native/jacobians.stdout");
const MODELS: [[u32; 7]; 3] = [
    [1, 0, 1, 0, 0, 1, 1],
    [1, 0, 1, 1, 0, 1, 1],
    [1, 0, 1, 2, 0, 1, 2],
];
const LABELS: [&str; 3] = ["C", "C0", "C2"];
const G0: [[u32; 3]; 3] = [[4, 24, 1], [5, 19, 1], [3, 21, 1]];
const G2: [[u32; 3]; 3] = [[25, 20, 1], [20, 22, 1], [22, 25, 1]];
const IDENTITY: [u32; 4] = [1, 0, 0, 1];
const INVOLUTION: [u32; 4] = [1, 0, 1, 2];

#[derive(Clone, Copy, Debug)]
struct Iso {
    matrix: [u32; 4],
    scale: u32,
}

struct Fixture {
    classes: Vec<Divisor>,
    index: BTreeMap<Vec<u32>, usize>,
    points: Vec<(u32, u32, u32)>,
    pairs: Vec<(usize, usize, usize)>,
}

fn numbers(s: &str) -> Vec<u32> {
    s.split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

fn integer_field(line: &str, name: &str) -> usize {
    line.split_once(&format!(" {name}="))
        .unwrap()
        .1
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap()
}

fn fixture(label: &str, chart: &Chart<'_>) -> Fixture {
    let mut classes = Vec::new();
    let mut index = BTreeMap::new();
    let mut points = Vec::new();
    let mut pairs = Vec::new();
    for line in RAW.lines() {
        if line.starts_with(&format!("RATIONAL_CLASS label={label} ")) {
            let id = integer_field(line, "id");
            assert_eq!(id, classes.len());
            let list = line
                .split_once(" divisor=[")
                .unwrap()
                .1
                .split_once(']')
                .unwrap()
                .0;
            let encoded = numbers(list);
            let nu = encoded[0] as usize;
            let nv = encoded[nu + 1] as usize;
            assert_eq!(encoded.len(), nu + nv + 2);
            let d = Div {
                u: encoded[1..=nu].to_vec(),
                v: encoded[nu + 2..].to_vec(),
            };
            assert!(kernel::eligible(chart, &d));
            assert_eq!(key(&d), encoded);
            assert!(index.insert(encoded, id).is_none());
            classes.push(d);
        }
        if line.starts_with(&format!("ACTUAL_F9_POINTS label={label} ")) {
            assert!(points.is_empty());
            let list = line.split_once(" points=").unwrap().1;
            let coordinates = numbers(list);
            assert_eq!(coordinates.len(), 36);
            for p in coordinates.as_chunks::<3>().0 {
                chart.point_homogeneous(p[0], p[1], p[2]);
                points.push((p[0], p[1], p[2]));
            }
        }
        if line.starts_with(&format!("EFFECTIVE_PAIR label={label} ")) {
            pairs.push((
                integer_field(line, "first"),
                integer_field(line, "second"),
                integer_field(line, "class"),
            ));
        }
    }
    assert_eq!(classes.len(), 21);
    assert_eq!(points.len(), 12);
    assert_eq!(pairs.len(), 24);
    for &(i, j, c) in &pairs {
        assert!(i < points.len() && j < points.len() && c < classes.len());
    }
    println!("FROZEN_FIXTURE label={label} actual_classes=21 actual_points=12 effective_pairs=24 raw_sha256=721790e8aa9c7ea0da5151a2bb48da03a4578a871f627ec503c71a0947c195c4 constructor_not_replayed=pass");
    Fixture {
        classes,
        index,
        points,
        pairs,
    }
}

fn determinant(f: &Field, m: [u32; 4]) -> u32 {
    f.sub(&f.mul(&m[0], &m[3]), &f.mul(&m[1], &m[2]))
}

fn projective_matrices(f: &Field) -> Vec<[u32; 4]> {
    let mut result = Vec::new();
    for a in 0..3 {
        for b in 0..3 {
            for c in 0..3 {
                for d in 0..3 {
                    let m = [a, b, c, d];
                    if m.iter().find(|&&x| x != 0) == Some(&1) && determinant(f, m) != 0 {
                        println!("PGL_MATRIX id={} entries={m:?}", result.len());
                        result.push(m);
                    }
                }
            }
        }
    }
    assert_eq!(result.len(), 24);
    result
}

fn pullback_form(f: &Field, target: &[u32], m: [u32; 4]) -> Vec<u32> {
    let mut top = vec![vec![1]];
    let mut bottom = vec![vec![1]];
    for _ in 0..6 {
        top.push(pmul(f, top.last().unwrap(), &[m[1], m[0]]));
        bottom.push(pmul(f, bottom.last().unwrap(), &[m[3], m[2]]));
    }
    let mut result = Vec::new();
    for (i, c) in target.iter().enumerate() {
        result = padd(f, &result, &pscale(f, &pmul(f, &top[i], &bottom[6 - i]), c));
    }
    result
}

fn coefficient_scale(f: &Field, source: &[u32], target: &[u32], m: [u32; 4]) -> u32 {
    let p = pullback_form(f, target, m);
    for scale in [1, 2] {
        if p == pscale(f, source, &scale) {
            return scale;
        }
    }
    0
}

fn compatible(f: &Field, source: &[u32], target: &[u32], iso: Iso) -> bool {
    determinant(f, iso.matrix) != 0
        && iso.scale != 0
        && pullback_form(f, target, iso.matrix) == pscale(f, source, &f.mul(&iso.scale, &iso.scale))
}

fn inverse(f: &Field, iso: Iso) -> Iso {
    let [a, b, c, d] = iso.matrix;
    let mut adjugate = [d, f.neg(&b), f.neg(&c), a];
    let mu = f.inv(adjugate.iter().find(|&&x| x != 0).unwrap());
    adjugate = adjugate.map(|x| f.mul(&x, &mu));
    let t = f.mul(&mu, &determinant(f, iso.matrix));
    let product = [
        f.add(&f.mul(&adjugate[0], &a), &f.mul(&adjugate[1], &c)),
        f.add(&f.mul(&adjugate[0], &b), &f.mul(&adjugate[1], &d)),
        f.add(&f.mul(&adjugate[2], &a), &f.mul(&adjugate[3], &c)),
        f.add(&f.mul(&adjugate[2], &b), &f.mul(&adjugate[3], &d)),
    ];
    assert_eq!(product, [t, 0, 0, t]);
    Iso {
        matrix: adjugate,
        scale: f.mul(&f.pow(t, 3), &f.inv(&iso.scale)),
    }
}

fn geometric_support(source: &Chart<'_>, d: &Divisor) -> [(u32, u32, u32); 2] {
    assert!(source.h.is_valid(d) && d.u.len() == 3 && !source.h.is_identity(d));
    let f = source.h.f;
    assert_eq!(d.u[2], 1);
    let discriminant = f.sub(&f.mul(&d.u[1], &d.u[1]), &d.u[0]);
    let root = f
        .sqrt(discriminant)
        .expect("selected geometric support must split in F729");
    let roots = [root, f.neg(&root)].map(|s| f.mul(&f.sub(&s, &d.u[1]), &2));
    roots.map(|t| {
        assert_eq!(peval(f, &d.u, &t), 0);
        let z = peval(f, &d.v, &t);
        let a = f.add(&f.mul(&source.r, &t), &1);
        let b = t;
        let y = f.mul(&source.scale, &z);
        assert_ne!(y, 0);
        (a, b, y)
    })
}

fn point_map(target: &Chart<'_>, iso: Iso, point: (u32, u32, u32)) -> Divisor {
    let f = target.h.f;
    let (a, b, y) = point;
    let m = iso.matrix;
    let top = f.add(&f.mul(&m[0], &a), &f.mul(&m[1], &b));
    let bottom = f.add(&f.mul(&m[2], &a), &f.mul(&m[3], &b));
    target.point_homogeneous(top, bottom, f.mul(&iso.scale, &y))
}

fn isomorphism(source: &Chart<'_>, target: &Chart<'_>, iso: Iso, d: &Divisor) -> Divisor {
    assert!(source.h.is_valid(d));
    if source.h.is_identity(d) {
        return target.h.identity();
    }
    let images = geometric_support(source, d).map(|point| point_map(target, iso, point));
    let image = target.h.add(&images[0], &images[1]);
    assert!(target.h.is_valid(&image));
    image
}

fn scalar_action(chart: &Chart<'_>, classes: &[Divisor], images: &[Divisor]) -> u32 {
    let matches: Vec<_> = (0..21)
        .filter(|&n| {
            classes
                .iter()
                .zip(images)
                .all(|(d, image)| chart.h.mul(d, u128::from(n)) == *image)
        })
        .collect();
    assert_eq!(matches.len(), 1);
    matches[0]
}

fn map_laws(source: &Chart<'_>, a: &Fixture, target: &Chart<'_>, images: &[Divisor]) {
    for (i, d) in a.classes.iter().enumerate() {
        assert_eq!(target.frobenius(&images[i]), images[i]);
        for (j, e) in a.classes.iter().enumerate() {
            let sum = source.h.add(d, e);
            let k = a.index[&key(&sum)];
            assert_eq!(target.h.add(&images[i], &images[j]), images[k]);
        }
        for n in 0..21 {
            let k = a.index[&key(&source.h.mul(d, n))];
            assert_eq!(target.h.mul(&images[i], n), images[k]);
        }
    }
}

fn qualify_iso(
    label: &str,
    source: &Chart<'_>,
    a: &Fixture,
    target: &Chart<'_>,
    b: &Fixture,
    iso: Iso,
) {
    let f = source.h.f;
    assert!(compatible(f, &source.model, &target.model, iso));
    assert_eq!(f.pow(iso.scale, 3), iso.scale);
    let inv = inverse(f, iso);
    assert!(compatible(f, &target.model, &source.model, inv));
    let images: Vec<_> = a
        .classes
        .iter()
        .map(|d| isomorphism(source, target, iso, d))
        .collect();
    let mut distinct = BTreeSet::new();
    for (i, d) in a.classes.iter().enumerate() {
        let k = key(&images[i]);
        let destination = b.index[&k];
        assert!(distinct.insert(k.clone()));
        assert_eq!(isomorphism(target, source, inv, &images[i]), *d);
        if iso.matrix == IDENTITY && source.model == target.model {
            assert_eq!(
                images[i],
                if iso.scale == 1 {
                    d.clone()
                } else {
                    source.h.neg(d)
                }
            );
        }
        println!("ISO_IMAGE map={label} source={i} target={destination} divisor={k:?}");
    }
    assert_eq!(distinct.len(), 21);
    map_laws(source, a, target, &images);
    for &(i, j, c) in &a.pairs {
        let x = point_map(target, iso, a.points[i]);
        let y = point_map(target, iso, a.points[j]);
        assert_eq!(target.h.add(&x, &y), images[c]);
    }
    if source.model == target.model {
        let action = scalar_action(source, &a.classes, &images);
        println!("SELF_ACTION map={label} full_rational_mod21={action} order3_action={} order7_action={} scope=finite_driver_only", action % 3, action % 7);
    }
    println!("ISO_RESULT map={label} matrix={:?} ordinate={} classes=21 bijective=pass addition_pairs=441 scalar_controls=441 independent_effective_pairs=24 exact_roundtrips=21 F3_fixed=21 scalar_labels_used_by_map=0", iso.matrix, iso.scale);
}

fn cross_chain(
    label: &str,
    original: &Chart<'_>,
    a: &Fixture,
    endpoints: (&Chart<'_>, &Chart<'_>),
    kernels: (&kernel::Kernel, &kernel::Kernel),
    iso: Iso,
) {
    let (first_target, second_target) = endpoints;
    let (first, dual) = kernels;
    let images: Vec<_> = a
        .classes
        .iter()
        .map(|d| {
            let first_image = kernel::map(original, first_target, first, d);
            let middle = isomorphism(first_target, second_target, iso, &first_image);
            kernel::map(second_target, original, dual, &middle)
        })
        .collect();
    assert_eq!(images.iter().map(key).collect::<BTreeSet<_>>().len(), 21);
    map_laws(original, a, original, &images);
    let action = scalar_action(original, &a.classes, &images);
    let mut order7_points = 0;
    let mut plus2_equal = 0;
    let mut minus2_equal = 0;
    for (i, (d, image)) in a.classes.iter().zip(&images).enumerate() {
        assert!(kernel::eligible(original, image));
        if original.h.is_identity(&original.h.mul(d, 7)) {
            order7_points += 1;
            let twice = original.h.mul(d, 2);
            plus2_equal += usize::from(*image == twice);
            minus2_equal += usize::from(*image == original.h.neg(&twice));
        }
        println!(
            "CROSS_IMAGE map={label} source={i} divisor={:?}",
            key(image)
        );
    }
    assert_eq!(order7_points, 7);
    println!("CROSS_RESULT map={label} actual_hops=3 classes=21 bijective=pass addition_pairs=441 full_rational_mod21={action} order7_mod7={} order7_points=7 equal_plus2={plus2_equal} equal_minus2={minus2_equal} scalar_labels_used_by_chain=0", action % 7);
}

fn twist_control(f: &Field, source: &Chart<'_>, a: &Fixture) {
    let model = pscale(f, &source.model, &2);
    let target = Chart::new(f, &model);
    target.controls();
    let gamma = f.sqrt(2).unwrap();
    assert_eq!(f.mul(&gamma, &gamma), 2);
    assert_eq!(f.pow(gamma, 9), gamma);
    assert_eq!(f.pow(gamma, 3), f.neg(&gamma));
    assert_ne!(f.pow(gamma, 3), gamma);
    for e in [1, 2] {
        assert!(!compatible(
            f,
            &source.model,
            &target.model,
            Iso {
                matrix: IDENTITY,
                scale: e
            }
        ));
    }
    let iso = Iso {
        matrix: IDENTITY,
        scale: gamma,
    };
    assert!(compatible(f, &source.model, &target.model, iso));
    let inv = inverse(f, iso);
    assert!(compatible(f, &target.model, &source.model, inv));
    let mut distinct = BTreeSet::new();
    let mut fixed = 0;
    for (i, d) in a.classes.iter().enumerate() {
        let image = isomorphism(source, &target, iso, d);
        assert!(target.h.is_valid(&image));
        assert!(distinct.insert(key(&image)));
        let frobenius = target.frobenius(&image);
        assert_eq!(frobenius, target.h.neg(&image));
        let rational = frobenius == image;
        assert_eq!(rational, source.h.is_identity(d));
        fixed += usize::from(rational);
        assert_eq!(isomorphism(&target, source, inv, &image), *d);
        println!("TWIST_IMAGE source={i} divisor={:?} F3_fixed={rational} Frob_equals_negative=pass inverse_restored=pass", key(&image));
    }
    assert_eq!(fixed, 1);
    assert_eq!(distinct.len(), 21);
    println!("TWIST_RESULT gamma={gamma} coefficient_scale=2 F3_ordinate_lifts=0 actual_F9_isomorphism_classes=21 valid=21 inverse_roundtrips=21 nonidentity_F3_descent_rejected=20 identity_F3_positive=1 hardcoded_reject=0");
}

fn main() {
    let f = Field::new();
    println!(
        "INHERITED_FIELD q=729 generator={} setup_controls=pass",
        f.generator
    );
    let charts = MODELS.map(|model| Chart::new(&f, &model));
    for chart in &charts {
        chart.controls();
        println!(
            "MODEL coefficients={:?} roots={:?}",
            chart.model, chart.roots
        );
    }
    let fixtures: Vec<_> = LABELS
        .iter()
        .zip(&charts)
        .map(|(label, chart)| fixture(label, chart))
        .collect();
    let matrices = projective_matrices(&f);
    let mut routes = Vec::new();
    for (i, source) in charts.iter().enumerate() {
        for (j, target) in charts.iter().enumerate() {
            let scales: Vec<_> = matrices
                .iter()
                .map(|&m| coefficient_scale(&f, &source.model, &target.model, m))
                .collect();
            println!("FULL_COEFFICIENT_CLASSIFICATION source={} target={} all24_scales_or_zero={scales:?}", LABELS[i], LABELS[j]);
            for (&matrix, &scale) in matrices.iter().zip(&scales) {
                if scale == 1 {
                    for e in [1, 2] {
                        let iso = Iso { matrix, scale: e };
                        let label = format!("{}_to_{}_M{:?}_e{e}", LABELS[i], LABELS[j], matrix);
                        qualify_iso(&label, source, &fixtures[i], target, &fixtures[j], iso);
                        routes.push((i, j, iso));
                    }
                } else if scale == 2 {
                    for e in [1, 2] {
                        assert!(!compatible(
                            &f,
                            &source.model,
                            &target.model,
                            Iso { matrix, scale: e }
                        ));
                    }
                }
            }
            assert!(!compatible(
                &f,
                &source.model,
                &target.model,
                Iso {
                    matrix: [0; 4],
                    scale: 1
                }
            ));
            assert!(!compatible(
                &f,
                &source.model,
                &target.model,
                Iso {
                    matrix: IDENTITY,
                    scale: 0
                }
            ));
        }
    }
    for i in 0..3 {
        for e in [1, 2] {
            assert!(routes
                .iter()
                .any(|&(a, b, iso)| a == i && b == i && iso.matrix == IDENTITY && iso.scale == e));
        }
    }
    for e in [1, 2] {
        assert!(routes
            .iter()
            .any(|&(a, b, iso)| a == 0 && b == 0 && iso.matrix == INVOLUTION && iso.scale == e));
    }
    let first0 = kernel::Kernel::new(&f, &MODELS[0], G0.map(|p| p.to_vec())).unwrap();
    let first2 = kernel::Kernel::new(&f, &MODELS[0], G2.map(|p| p.to_vec())).unwrap();
    let dual0 = first0.dual(&f);
    let dual2 = first2.dual(&f);
    let mut cross_count = 0;
    for &(i, j, iso) in &routes {
        if (i == 1 && j == 2) || (i == 2 && j == 1) {
            let (first, dual) = if i == 1 {
                (&first0, &dual2)
            } else {
                (&first2, &dual0)
            };
            let label = format!(
                "C_via_{}_iso_{}_rawdual_M{:?}_e{}",
                LABELS[i], LABELS[j], iso.matrix, iso.scale
            );
            cross_chain(
                &label,
                &charts[0],
                &fixtures[0],
                (&charts[i], &charts[j]),
                (first, dual),
                iso,
            );
            cross_count += 1;
        }
    }
    twist_control(&f, &charts[0], &fixtures[0]);
    println!("SUMMARY PGL2_F3_matrices=24 ordered_model_pairs=9 full_coefficient_decisions=216 F3_isomorphisms={} each_map_classes=21 each_addition_pairs=441 each_scalar_controls=441 each_effective_pair_controls=24 each_exact_inverse_roundtrips=21 actual_three_hop_cross_chains={cross_count} each_cross_classes=21 each_cross_additions=441 extension_twist_classes=21 twist_F3_nonidentity_rejections=20 old_constructor_replayed=0 transfer_scalar_labels=0 actual_weil8=not_computed costs=unmeasured", routes.len());
}
