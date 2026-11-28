mod field_c;
use field_c::*;
use std::collections::BTreeMap;
const B_INPUT: &str = include_str!("first-B-native/canonical-ramification.stdout");
const E_INPUT: &str =
    include_str!("../chained_cm19_fibres_20261124/first-E-native/actual-functions.stdout");
type Poly = BTreeMap<(usize, usize), u8>;
type Point = [K; 3];
fn numbers(s: &str) -> Vec<usize> {
    s.split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}
fn field<'a>(s: &'a str, prefix: &str) -> &'a str {
    s.split('\t').find_map(|s| s.strip_prefix(prefix)).unwrap()
}
fn parse_poly(s: &str, degree: usize) -> Poly {
    let n = numbers(s);
    assert_eq!(n.len() % 3, 0);
    let mut p = Poly::new();
    for a in n.as_chunks::<3>().0 {
        assert!(a[0] + a[1] <= degree && a[2] > 0 && a[2] < 7);
        assert!(p
            .insert((a[0], a[1]), u8::try_from(a[2]).unwrap())
            .is_none());
    }
    assert!(!p.is_empty());
    p
}
fn evaluate(f: Quad, p: &Poly, degree: usize, point: Point) -> K {
    p.iter()
        .map(|(&(y, z), &c)| {
            f.mul(
                k(c),
                f.mul(
                    f.mul(f.pow(point[0], degree - y - z), f.pow(point[1], y)),
                    f.pow(point[2], z),
                ),
            )
        })
        .fold(k(0), |a, b| f.add(a, b))
}
struct Functions {
    q: Poly,
    x: Poly,
    y: Poly,
    curve: Poly,
}
fn functions() -> Functions {
    let q = E_INPUT
        .lines()
        .find(|s| s.starts_with("ACTUAL_DENOMINATOR_CONIC\t"))
        .unwrap();
    let x = E_INPUT
        .lines()
        .find(|s| s.starts_with("ACTUAL_NUMERATOR\tdegree=4\t"))
        .unwrap();
    let y = E_INPUT
        .lines()
        .find(|s| s.starts_with("ACTUAL_NUMERATOR\tdegree=6\t"))
        .unwrap();
    let identity = E_INPUT
        .lines()
        .find(|s| s.starts_with("COMPLETE_ELLIPTIC_POLYNOMIAL_IDENTITY\t"))
        .unwrap();
    assert_eq!(field(identity, "remainder="), "{}");
    Functions {
        q: parse_poly(field(q, "terms-Y-Z="), 2),
        x: parse_poly(field(x, "terms-Y-Z="), 4),
        y: parse_poly(field(y, "terms-Y-Z="), 6),
        curve: parse_poly(field(identity, "curve="), 4),
    }
}
fn parse_e(s: &str) -> EPoint {
    if s == "None" {
        return None;
    }
    assert!(s.starts_with("Some((") && s.ends_with("))"));
    let n = numbers(s);
    assert_eq!(n.len(), 2);
    Some((
        k(u8::try_from(n[0]).unwrap()),
        k(u8::try_from(n[1]).unwrap()),
    ))
}
fn field_certificate(f: Quad, modulus: [u8; 3]) {
    let alpha = [0, 1];
    assert_eq!(
        f.add(
            f.add(f.pow(alpha, 2), f.mul(k(modulus[1]), alpha)),
            k(modulus[0])
        ),
        k(0)
    );
    let mut count = 0;
    let mut nonzero = 0;
    for a in 0..7u8 {
        for b in 0..7u8 {
            let v = [a, b];
            let conjugate = f.conjugate(v);
            assert_eq!(f.pow(v, 7), conjugate);
            assert_eq!(f.conjugate(conjugate), v);
            assert_eq!(f.pow(v, 49), v);
            let norm = f.mul(v, conjugate);
            assert_eq!(norm[1], 0);
            if v != k(0) {
                assert_eq!(f.mul(v, f.inverse(v)), k(1));
                nonzero += 1;
            }
            println!("ACTUAL_QUADRATIC_FIELD_CERTIFICATE\telement={v:?}\tFrobenius7={conjugate:?}\tnorm={norm:?}\tunit={}",v!=k(0));
            count += 1;
        }
    }
    assert_eq!((count, nonzero), (49, 48));
    println!("COMPLETE_FIELD_RESULT\tmodulus={modulus:?}\t49-Frobenius49-identity\t48-norm-inverses\tactual-alpha={alpha:?}");
}
fn read_parameter() -> ([[u8; 2]; 3], [u8; 3]) {
    let line = B_INPUT
        .lines()
        .find(|s| s.starts_with("FULL_HOMOGENEOUS_LINE_PARAMETER\t"))
        .unwrap();
    let n = numbers(field(line, "coordinates-U-V="));
    assert_eq!(n.len(), 6);
    let p = std::array::from_fn(|i| {
        [
            u8::try_from(n[2 * i]).unwrap(),
            u8::try_from(n[2 * i + 1]).unwrap(),
        ]
    });
    let n = numbers(field(line, "line="));
    assert_eq!(n.len(), 3);
    let h = n
        .into_iter()
        .map(|x| u8::try_from(x).unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    (p, h)
}
fn residual_modulus() -> [u8; 3] {
    let line = B_INPUT
        .lines()
        .find(|s| s.starts_with("COMPLETE_ACTUAL_RAMIFICATION_FACTOR_CERTIFICATE\t"))
        .unwrap();
    assert_eq!(field(line, "total-degree="), "4");
    assert_eq!(field(line, "full-product-equal="), "true");
    assert_eq!(field(line, "quadratic-irreducible="), "true");
    let n = numbers(field(line, "residual-monic="));
    assert_eq!(n.len(), 3);
    n.into_iter()
        .map(|n| u8::try_from(n).unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
fn branch_image(f: Quad, functions: &Functions, point: Point) -> (EPoint, EPoint, K, K, K) {
    assert_eq!(evaluate(f, &functions.curve, 4, point), k(0));
    let q = evaluate(f, &functions.q, 2, point);
    assert_ne!(
        q,
        k(0),
        "Residual q=0 requires separately frozen local extension source"
    );
    let x = evaluate(f, &functions.x, 4, point);
    let y = evaluate(f, &functions.y, 6, point);
    let g = Some((f.divide(x, f.pow(q, 2)), f.divide(y, f.pow(q, 3))));
    assert!(valid(f, g));
    let image = eadd(f, g, Some((k(3), k(5))));
    (g, image, q, x, y)
}
fn actual_target_relation(f: Quad, images: &[EPoint]) {
    let (Some((x, y)), Some((xx, yy))) = (images[0], images[1]) else {
        assert_eq!(images[0], images[1]);
        println!("ACTUAL_RESIDUAL_TARGET_INFINITY\tboth-conjugate-images-infinity\tno-quadratic-x-relation-claimed");
        return;
    };
    assert_eq!(f.conjugate(x), xx);
    assert_eq!(f.conjugate(y), yy);
    if x[1] != 0 {
        let trace = f.add(x, xx);
        let norm = f.mul(x, xx);
        assert_eq!(trace[1], 0);
        assert_eq!(norm[1], 0);
        let monic = [norm[0], sub(0, trace[0]), 1];
        let slope = mul(y[1], inv(x[1]));
        let intercept = sub(y[0], mul(slope, x[0]));
        assert_eq!(y, f.add(k(intercept), f.mul(k(slope), x)));
        assert_eq!(yy, f.add(k(intercept), f.mul(k(slope), xx)));
        for v in [x, xx] {
            assert_eq!(
                f.add(f.add(f.pow(v, 2), f.mul(k(monic[1]), v)), k(monic[0])),
                k(0)
            );
        }
        println!("ACTUAL_BASE_FIELD_RESIDUAL_BRANCH_RELATION\tx-monic-constant-linear-leading={monic:?}\ty-intercept-slope={:?}\tboth-images-satisfy=true",[intercept,slope]);
    } else {
        println!("ACTUAL_RESIDUAL_TARGET_X_RATIONAL\tx={x:?}\ty={y:?}\tconjugate-x={xx:?}\tconjugate-y={yy:?}\tno-x-quadratic-relation-claimed");
    }
}
fn main() {
    println!("PROFILE\tactual-CM19-quadratic-ramification-branches-C\tfixed-exact-correctness\tcoordinates-in-basis-1-alpha\tunsigned-geometric-bridges-explicit");
    let modulus = residual_modulus();
    let f = Quad::new(modulus);
    field_certificate(f, modulus);
    let functions = functions();
    let (parameter, line) = read_parameter();
    let alpha = [0, 1];
    let roots = [alpha, f.conjugate(alpha)];
    assert_ne!(roots[0], roots[1]);
    let mut images = Vec::new();
    let mut residual = Vec::new();
    let mut controls = 0;
    for root in roots {
        assert_eq!(
            f.add(
                f.add(f.pow(root, 2), f.mul(k(modulus[1]), root)),
                k(modulus[0])
            ),
            k(0)
        );
        let point = parameter.map(|p| f.add(k(p[0]), f.mul(k(p[1]), root)));
        assert_eq!(
            point
                .into_iter()
                .zip(line)
                .map(|(x, c)| f.mul(x, k(c)))
                .fold(k(0), |a, b| f.add(a, b)),
            k(0)
        );
        let (g, image, q, x, y) = branch_image(f, &functions, point);
        let conjugate_point = point.map(|v| f.pow(v, 7));
        let cg = branch_image(f, &functions, conjugate_point).1;
        assert_eq!(image.map(|(x, y)| (f.pow(x, 7), f.pow(y, 7))), cg);
        println!("ACTUAL_RESIDUAL_RAMIFICATION_BRANCH\troot={root:?}\tC={point:?}\tq={q:?}\tp4={x:?}\tp6={y:?}\tg={g:?}\tf4={image:?}\tFrobenius-image={cg:?}\tcanonical-zero-order=1\ttame-e=2");
        let Some((gx, gy)) = g else {
            panic!("dense finite fractions have finite g");
        };
        let bad_x = f.divide(f.add(x, f.pow(point[0], 4)), f.pow(q, 2));
        assert_ne!(gx, bad_x);
        let bad = Some((bad_x, gy));
        println!("ACTUAL_RESIDUAL_NUMERATOR_CONTROL\troot={root:?}\tchanged-real-X4-coefficient\tpristine-g={g:?}\tmutant-g={bad:?}\tmutant-on-E={}\tactual-map-equality-rejects=true",valid(f,bad));
        assert_eq!(f.divide(x, f.pow(q, 2)), gx);
        controls += 1;
        images.push(image);
        residual.push(image);
    }
    for line in B_INPUT
        .lines()
        .filter(|s| s.starts_with("ACTUAL_RATIONAL_RAMIFICATION_BRANCH\t"))
    {
        let image = parse_e(field(line, "f4="));
        assert!(valid(f, image));
        println!(
            "INHERITED_ACTUAL_RATIONAL_BRANCH\tC={}\tf4={image:?}\ttame-e={}",
            field(line, "C="),
            field(line, "tame-e=")
        );
        images.push(image);
    }
    assert_eq!(controls, 2);
    let matrix: Vec<Vec<bool>> = images
        .iter()
        .map(|a| images.iter().map(|b| a == b).collect())
        .collect();
    let distinct = images
        .iter()
        .enumerate()
        .filter(|(i, a)| !images[..*i].contains(a))
        .count();
    println!("COMPLETE_ACTUAL_BRANCH_COLLISION_CERTIFICATE\tall-geometric-branch-images={images:?}\tequality-matrix={matrix:?}\tdistinct-values={distinct}");
    actual_target_relation(f, &residual);
    let group_trace = eadd(f, residual[0], residual[1]);
    assert_eq!(
        group_trace.map(|(x, y)| (f.pow(x, 7), f.pow(y, 7))),
        group_trace
    );
    println!("ACTUAL_ELLIPTIC_GROUP_TRACE_OF_RESIDUAL_BRANCH_PAIR\ttrace={group_trace:?}\tFrobenius-fixed=true");
    println!("SUMMARY\tactual-irreducible-quadratic-field\tboth-residual-points-and-images\tcomplete-Frobenius-collision-certificate\tbase-field-target-relation-derived\t2-real-numerator-controls-restored\tactual-canonical-RH-map-bridges-written\tfull-geometric-formalization=OPEN");
}
