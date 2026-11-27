mod field_e;
mod poly_e;
use field_e::*;
use poly_e::*;
use std::collections::BTreeSet;
const A_INPUT: &str = include_str!("first-A-native/actual-fibres.stdout");
const C_INPUT: &str = include_str!("first-C-native/weighted-fibres.stdout");
const R_MATRIX: [[u8; 3]; 3] = [[1, 5, 2], [0, 5, 0], [2, 1, 1]];
const T: EPoint = Some((1, 3));
fn field<'a>(line: &'a str, prefix: &str) -> &'a str {
    line.split('\t')
        .find_map(|x| x.strip_prefix(prefix))
        .unwrap()
}
fn numbers(s: &str) -> Vec<usize> {
    s.split(|c: char| !c.is_ascii_digit())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect()
}
fn parse_e(s: &str) -> EPoint {
    if s == "None" {
        return None;
    }
    assert!(s.starts_with("Some((") && s.ends_with("))"));
    let v = numbers(s);
    assert_eq!(v.len(), 2);
    Some((u8::try_from(v[0]).unwrap(), u8::try_from(v[1]).unwrap()))
}
fn parse_covers(mut s: &str) -> [EPoint; 3] {
    s = s.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
    let mut r = Vec::new();
    while !s.is_empty() {
        let n = if s.starts_with("None") {
            4
        } else {
            s.find("))").unwrap() + 2
        };
        r.push(parse_e(&s[..n]));
        s = &s[n..];
        if !s.is_empty() {
            s = s.strip_prefix(", ").unwrap();
        }
    }
    r.try_into().unwrap()
}
#[derive(Debug)]
struct Row {
    point: Point,
    covers: [EPoint; 3],
    image: EPoint,
    jet: [Jet; 3],
    omega: u8,
}
fn point_jet(p: Point, f: &Poly) -> [Jet; 3] {
    assert_eq!(p[0], 1);
    let [_, y, z] = p;
    let fy = f.jet([y, 1, 0], jc(z))[1];
    let fz = f.jet(jc(y), [z, 1, 0])[1];
    let jet = if fy != 0 {
        let mut yy = [y, mul(sub(0, fz), inv(fy)), 0];
        let zz = [z, 1, 0];
        yy[2] = mul(sub(0, f.jet(yy, zz)[2]), inv(fy));
        [jc(1), yy, zz]
    } else {
        assert_ne!(fz, 0);
        let yy = [y, 1, 0];
        let mut zz = [z, mul(sub(0, fy), inv(fz)), 0];
        zz[2] = mul(sub(0, f.jet(yy, zz)[2]), inv(fz));
        [jc(1), yy, zz]
    };
    assert_eq!(f.jet(jet[1], jet[2]), [0; 3]);
    jet
}
fn transform(p: [Jet; 3]) -> [Jet; 3] {
    R_MATRIX.map(|row| {
        row.into_iter()
            .zip(p)
            .map(|(n, j)| js(j, n))
            .fold(jc(0), ja)
    })
}
fn f0_jet(p: [Jet; 3]) -> (EPoint, u8) {
    let [x, y, z] = p;
    let g = [jp(z, 2), js(jp(x, 2), 6), js(jm(x, y), 3), jp(y, 2)]
        .into_iter()
        .fold(jc(0), ja);
    if y[0] == 0 {
        let xy = jm(x, y);
        assert_eq!([xy[0], xy[1], g[0]], [0; 3]);
        assert_ne!(g[1], 0);
        return (None, mul(sub(0, xy[2]), inv(g[1])));
    }
    let xe = jd(x, y);
    let ye = jd(g, jp(y, 2));
    let value = Some((xe[0], ye[0]));
    assert!(valid(value));
    assert_ne!(ye[0], 0);
    assert_eq!(
        sub(
            mul(mul(2, ye[0]), ye[1]),
            mul(add(mul(3, pow(xe[0], 2)), 2), xe[1])
        ),
        0
    );
    (value, mul(xe[1], inv(mul(2, ye[0]))))
}
fn f3_jet(p: [Jet; 3]) -> (EPoint, u8) {
    let [a, b, c] = p;
    let x = js(b, 3);
    let y = js(ja(ja(a, js(b, 4)), c), 4);
    let z = js(ja(ja(a, js(b, 4)), js(c, 6)), 4);
    if x[0] == 0 {
        assert!(y[0] == 0 || z[0] == 0);
        let (v, df0) = f0_jet(p);
        let (vv, df1) = f0_jet(transform(p));
        assert_eq!(v, None);
        assert_eq!(vv, None);
        let tangent = mul(df1, inv(df0));
        assert!([2, 4].contains(&tangent));
        return (if y[0] == 0 { None } else { T }, 0);
    }
    let u = jd(jm(y, z), jp(x, 2));
    let v = jd(jp(y, 3), jp(x, 3));
    let w = [js(v, 4), jc(1), js(u, 3), js(jp(u, 2), 2)]
        .into_iter()
        .fold(jc(0), ja);
    let delta = [w, js(jp(u, 2), 2), js(u, 6), jc(3)]
        .into_iter()
        .fold(jc(0), ja);
    let xe = ja(delta, jc(1));
    let ye = ja(jm(ja(js(u, 5), jc(4)), delta), jc(4));
    let value = Some((xe[0], ye[0]));
    assert!(valid(value));
    assert_ne!(ye[0], 0);
    assert_eq!(
        mul(mul(2, ye[0]), ye[1]),
        mul(add(mul(3, pow(xe[0], 2)), 2), xe[1])
    );
    (value, mul(xe[1], inv(mul(2, ye[0]))))
}
fn read_rows(f: &Poly) -> Vec<Row> {
    let rows:Vec<_>=A_INPUT.lines().filter(|s|s.starts_with("ACTUAL_POINT\t")).map(|s|{
        let n=numbers(field(s,"C="));assert_eq!(n.len(),3);let point=n.into_iter().map(|x|u8::try_from(x).unwrap()).collect::<Vec<_>>().try_into().unwrap();let jet=point_jet(point,f);let covers=parse_covers(field(s,"f0-f1-f3="));let image=parse_e(field(s,"f4="));
        let (v0,o0)=f0_jet(jet);let (v1,o1)=f0_jet(transform(jet));let (v3,o3)=f3_jet(jet);assert_eq!([v0,v1,v3],covers);let omega=sub(o1,mul(5,o3));
        println!("ACTUAL_CURVE_AND_COVER_JET\tC={point:?}\tjet={jet:?}\tcover-values={covers:?}\tomega0={o0}\tomega1={o1}\tomega3={o3}\ttrue-omega4={omega}");Row{point,covers,image,jet,omega}
    }).collect();
    assert_eq!(rows.len(), 17);
    assert_eq!(
        rows.iter().map(|r| r.point).collect::<BTreeSet<_>>().len(),
        17
    );
    rows
}
fn full_pole_divisor(rows: &[Row]) -> (EPoint, Vec<(usize, usize)>) {
    assert!(C_INPUT
        .lines()
        .any(|s| s == "COMPLETE_NINE_PAIR_RESULT\tcases=9\tmatching-pairs=1"));
    let line = C_INPUT
        .lines()
        .find(|s| s.starts_with("ACTUAL_FULL_WEIGHTED_FIBRE\tscalar-T=4\t"))
        .unwrap();
    let target = parse_e(field(line, "R="));
    assert_eq!(target, emul(4, T));
    let v = numbers(field(line, "D="));
    assert_eq!(v.len(), 15);
    let d: Vec<_> = v
        .as_chunks::<5>()
        .0
        .iter()
        .map(|a| {
            let i = a[0];
            let point: Point = [
                u8::try_from(a[1]).unwrap(),
                u8::try_from(a[2]).unwrap(),
                u8::try_from(a[3]).unwrap(),
            ];
            assert_eq!(rows[i].point, point);
            assert_eq!(rows[i].image, target);
            (i, a[4])
        })
        .collect();
    assert_eq!(d, [(8, 1), (9, 1), (16, 2)]);
    assert_eq!(d.iter().map(|x| x.1).sum::<usize>(), 4);
    for &(i, m) in &d {
        if m == 2 {
            assert_eq!(rows[i].omega, 0);
        } else {
            assert_ne!(rows[i].omega, 0);
        }
    }
    println!("TRUE_WEIGHTED_POLE_DIVISOR\tR={target:?}\tD={d:?}\tdouble-derivative=0\tsimple-derivatives-nonzero\tglobal-completeness=qualified-C-plus-written-proof");
    (target, d)
}
fn vector_jet(basis: &[(usize, usize)], p: &Row, order: usize) -> Vec<u8> {
    basis
        .iter()
        .map(|&(y, z)| jm(jp(p.jet[1], y), jp(p.jet[2], z))[order])
        .collect()
}
fn print_linear(label: &str, input: &[Vec<u8>], r: &Linear) {
    println!("EXACT_LINEAR_CERTIFICATE\t{label}\tinput={input:?}\trank={}\tpivots={:?}\trref={:?}\tsolution={:?}\tkernel={:?}",r.rank,r.pivots,r.reduced,r.solution,r.kernel);
}
fn conic(rows: &[Row], d: &[(usize, usize)]) -> Poly {
    let basis = basis(2);
    let mut input = Vec::new();
    for &(i, m) in d {
        for order in 0..m {
            let mut r = vector_jet(&basis, &rows[i], order);
            r.push(0);
            input.push(r);
        }
    }
    let r = solve(&input, basis.len());
    assert_eq!(r.rank, 4);
    assert_eq!(r.kernel.len(), 2);
    print_linear("weighted-conic", &input, &r);
    let q = Poly::from_coefficients(&basis, &r.kernel[0]);
    assert!(!q.0.is_empty());
    for &(i, m) in d {
        let jet = q.jet(rows[i].jet[1], rows[i].jet[2]);
        assert!(jet[..m].iter().all(|x| *x == 0));
        println!("CONIC_POLE_JET\tindex={i}\tweight={m}\tq={jet:?}");
    }
    println!("ACTUAL_DENOMINATOR_CONIC\tdegree=2\tterms-Y-Z={:?}\tX-exponent=degree-minus-Y-minus-Z\textra-zeros-permitted",q.0);
    q
}
fn numerator(
    rows: &[Row],
    d: &[(usize, usize)],
    target: EPoint,
    q: &Poly,
    n: usize,
    f: &Poly,
) -> Poly {
    let degree = 2 * n;
    let basis = basis(degree);
    let mut input = Vec::new();
    let mut finite = 0;
    for (i, p) in rows
        .iter()
        .enumerate()
        .filter(|(i, _)| !d.iter().any(|(j, _)| i == j))
    {
        finite += 1;
        let translated = eadd(p.image, eneg(target));
        let (x, y) = translated.unwrap();
        assert_ne!(y, 0);
        let dx = mul(mul(2, y), p.omega);
        let dy = mul(add(mul(3, pow(x, 2)), 2), p.omega);
        let (value, derivative) = if n == 2 {
            (x, dx)
        } else {
            assert_eq!(n, 3);
            (y, dy)
        };
        let qj = q.jet(p.jet[1], p.jet[2]);
        let rhs = jm([value, derivative, 0], jp(qj, n));
        for (order, &b) in rhs.iter().enumerate().take(2) {
            let mut row = vector_jet(&basis, p, order);
            row.push(b);
            input.push(row);
        }
        println!("TRUE_TRANSLATED_VALUE_JET\tn={n}\tindex={i}\tC={:?}\tg={translated:?}\tomega={}\tdx={dx}\tdy={dy}\tq={qj:?}\tmultiplied-rhs={rhs:?}",p.point,p.omega);
    }
    assert_eq!(finite, 14);
    assert_eq!(input.len(), 28);
    let r = solve(&input, basis.len());
    let rank = if n == 2 { 14 } else { 22 };
    assert_eq!(r.rank, rank);
    print_linear(&format!("numerator-degree-{degree}"), &input, &r);
    for k in &r.kernel {
        let poly = Poly::from_coefficients(&basis, k);
        let (quotient, remainder) = poly.divide_curve(f);
        assert!(remainder.0.is_empty());
        println!(
            "FULL_KERNEL_CURVE_MULTIPLE\tdegree={degree}\tkernel={:?}\tquotient={:?}",
            poly.0, quotient.0
        );
    }
    let result = Poly::from_coefficients(&basis, &r.solution);
    println!(
        "ACTUAL_NUMERATOR\tdegree={degree}\tterms-Y-Z={:?}\tX-exponent=degree-minus-Y-minus-Z",
        result.0
    );
    result
}
fn main() {
    println!("PROFILE\tactual-CM19-conic-Hermite-functions-E\tfixed-proof-construction\tall-scientific-standing-unsigned\tno-performance-claim");
    let f = actual_curve();
    let rows = read_rows(&f);
    let (target, d) = full_pole_divisor(&rows);
    let q = conic(&rows, &d);
    let x = numerator(&rows, &d, target, &q, 2, &f);
    let y = numerator(&rows, &d, target, &q, 3, &f);
    let identity = y
        .power(2)
        .minus(&x.power(3))
        .minus(&x.times(&q.power(4)).scaled(2))
        .minus(&q.power(6).scaled(6));
    let (quotient, remainder) = identity.divide_curve(&f);
    assert!(remainder.0.is_empty());
    println!("COMPLETE_ELLIPTIC_POLYNOMIAL_IDENTITY\tleft={:?}\tcurve={:?}\tquotient-degree-at-most=8\tquotient={:?}\tremainder={:?}",identity.0,f.0,quotient.0,remainder.0);
    let mut bad_x = x.clone();
    bad_x = bad_x.plus(&Poly::term(0, 0, 1));
    let bad = y
        .power(2)
        .minus(&bad_x.power(3))
        .minus(&bad_x.times(&q.power(4)).scaled(2))
        .minus(&q.power(6).scaled(6));
    assert!(!bad.divide_curve(&f).1 .0.is_empty());
    let mut changed_rhs = Vec::new();
    let b = basis(4);
    for (i, p) in rows
        .iter()
        .enumerate()
        .filter(|(i, _)| !d.iter().any(|(j, _)| i == j))
    {
        let mut r = vector_jet(&b, p, 0);
        let val = eadd(p.image, eneg(target)).unwrap().0;
        let rhs = mul(val, pow(q.jet(p.jet[1], p.jet[2])[0], 2));
        r.push(if i == 0 { add(rhs, 1) } else { rhs });
        changed_rhs.push(r);
    }
    let changed_count = changed_rhs
        .iter()
        .filter(|r| {
            let actual = b
                .iter()
                .map(|&(a, b)| x.0.get(&(a, b)).copied().unwrap_or(0))
                .zip(&r[..b.len()])
                .map(|(a, b)| mul(a, *b))
                .fold(0, add);
            actual != r[b.len()]
        })
        .count();
    assert_eq!(changed_count, 1);
    assert!(identity.divide_curve(&f).1 .0.is_empty());
    assert!(rows.iter().all(|p| p.covers.iter().all(|x| valid(*x))));
    println!("ACTUAL_FUNCTION_CONTROLS\tchanged-real-x-numerator-rejected-full-identity\tchanged-real-value-row-rejected\tpristine-restored");
    println!("SUMMARY\tfull-weighted-pole-divisor=4\t14-first-jets\tconic-rank=4\tnumerator-ranks=14,22\tcomplete-symbolic-E-identity\ttrue-map-equality=written-section-existence-and28>16,24\tg=f4-minus-R\tf4=g-plus-R\texceptional-evaluation=OPEN\tECDLP-speedup=UNMEASURED");
}
