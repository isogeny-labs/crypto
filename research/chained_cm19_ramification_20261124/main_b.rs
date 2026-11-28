use std::collections::BTreeMap;
const E_INPUT: &str =
    include_str!("../chained_cm19_fibres_20261124/first-E-native/actual-functions.stdout");
const F_INPUT: &str =
    include_str!("../chained_cm19_fibres_20261124/first-F-native/polynomial-evaluation.stdout");
type Point = [u8; 3];
type Poly = BTreeMap<(usize, usize), u8>;
fn m(n: i32) -> u8 {
    u8::try_from(n.rem_euclid(7)).unwrap()
}
fn add(a: u8, b: u8) -> u8 {
    m(i32::from(a) + i32::from(b))
}
fn sub(a: u8, b: u8) -> u8 {
    m(i32::from(a) - i32::from(b))
}
fn mul(a: u8, b: u8) -> u8 {
    m(i32::from(a) * i32::from(b))
}
fn pow(a: u8, n: usize) -> u8 {
    (0..n).fold(1, |v, _| mul(v, a))
}
fn inv(a: u8) -> u8 {
    (1..7).find(|b| mul(a, *b) == 1).unwrap()
}
fn numbers(s: &str) -> Vec<usize> {
    s.split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}
fn field<'a>(s: &'a str, prefix: &str) -> &'a str {
    s.split('\t').find_map(|s| s.strip_prefix(prefix)).unwrap()
}
fn actual_curve() -> Poly {
    let line = E_INPUT
        .lines()
        .find(|s| s.starts_with("COMPLETE_ELLIPTIC_POLYNOMIAL_IDENTITY\t"))
        .unwrap();
    assert_eq!(field(line, "remainder="), "{}");
    let n = numbers(field(line, "curve="));
    assert_eq!(n.len() % 3, 0);
    let mut p = Poly::new();
    for a in n.as_chunks::<3>().0 {
        assert!(a[0] + a[1] <= 4 && a[2] > 0 && a[2] < 7);
        assert!(p
            .insert((a[0], a[1]), u8::try_from(a[2]).unwrap())
            .is_none());
    }
    assert_eq!(p.len(), 7);
    p
}
fn eval(p: &Poly, y: u8, z: u8) -> u8 {
    p.iter()
        .map(|(&(a, b), &c)| mul(c, mul(pow(y, a), pow(z, b))))
        .fold(0, add)
}
fn partial(p: &Poly, y: u8, z: u8, variable: usize) -> u8 {
    p.iter()
        .filter_map(|(&(a, b), &c)| {
            let exponent = if variable == 0 { a } else { b };
            if exponent == 0 {
                return None;
            }
            let aa = if variable == 0 { a - 1 } else { a };
            let bb = if variable == 1 { b - 1 } else { b };
            Some(mul(
                mul(c, u8::try_from(exponent).unwrap()),
                mul(pow(y, aa), pow(z, bb)),
            ))
        })
        .fold(0, add)
}
#[derive(Debug)]
struct Datum {
    point: Point,
    theta: u8,
    omega: u8,
    value: u8,
}
fn actual_values(curve: &Poly) -> Vec<Datum> {
    let rows:Vec<_>=E_INPUT.lines().filter(|s|s.starts_with("ACTUAL_CURVE_AND_COVER_JET\t")).map(|s|{
  let n=numbers(field(s,"C="));let point:Point=n.into_iter().map(|n|u8::try_from(n).unwrap()).collect::<Vec<_>>().try_into().unwrap();assert_eq!(point[0],1);assert_eq!(eval(curve,point[1],point[2]),0);
  let j=numbers(field(s,"jet="));assert_eq!(j.len(),9);for (i,&coordinate) in point.iter().enumerate() {assert_eq!(j[3*i],usize::from(coordinate));}
  let fy=partial(curve,point[1],point[2],0);let fz=partial(curve,point[1],point[2],1);let dy=u8::try_from(j[4]).unwrap();let dz=u8::try_from(j[7]).unwrap();assert_eq!(add(mul(fy,dy),mul(fz,dz)),0);
  let theta=if fz!=0 {mul(dy,inv(fz))}else{assert_ne!(fy,0);mul(sub(0,dz),inv(fy))};assert_ne!(theta,0);
  let omega:u8=field(s,"true-omega4=").parse().unwrap();assert!(omega<7);let value=mul(omega,inv(theta));
  println!("ACTUAL_CANONICAL_GENERATOR_VALUE\tC={point:?}\tFy={fy}\tFz={fz}\tdy={dy}\tdz={dz}\ttheta={theta}\tomega4={omega}\tlinear-value={value}");Datum{point,theta,omega,value}
 }).collect();
    assert_eq!(rows.len(), 17);
    rows
}
fn recover_line(rows: &[Datum]) -> [u8; 3] {
    let input: Vec<Vec<u8>> = rows
        .iter()
        .map(|r| vec![r.point[0], r.point[1], r.point[2], r.value])
        .collect();
    let mut a = input.clone();
    let mut pivots = Vec::new();
    let mut next = 0;
    for column in 0..3 {
        let Some(found) = (next..a.len()).find(|i| a[*i][column] != 0) else {
            continue;
        };
        a.swap(next, found);
        let factor = inv(a[next][column]);
        for x in &mut a[next] {
            *x = mul(*x, factor);
        }
        let pivot = a[next].clone();
        for (i, r) in a.iter_mut().enumerate() {
            if i != next {
                let c = r[column];
                for (x, &p) in r.iter_mut().zip(&pivot) {
                    *x = sub(*x, mul(c, p));
                }
            }
        }
        pivots.push(column);
        next += 1;
    }
    assert_eq!(pivots, [0, 1, 2]);
    assert!(a
        .iter()
        .all(|r| r[..3].iter().any(|x| *x != 0) || r[3] == 0));
    let line = std::array::from_fn(|i| a[i][3]);
    for r in rows {
        let actual = r
            .point
            .into_iter()
            .zip(line)
            .map(|(a, b)| mul(a, b))
            .fold(0, add);
        assert_eq!(actual, r.value);
        assert_eq!(mul(actual, r.theta), r.omega);
    }
    println!("COMPLETE_CANONICAL_LINEAR_CERTIFICATE\tinput={input:?}\trank=3\tpivots={pivots:?}\trref={a:?}\tsolution={line:?}\tall17-residuals=0");
    line
}
fn trim(mut p: Vec<u8>) -> Vec<u8> {
    while p.len() > 1 && p.last() == Some(&0) {
        p.pop();
    }
    p
}
fn p_add(a: &[u8], b: &[u8]) -> Vec<u8> {
    trim(
        (0..a.len().max(b.len()))
            .map(|i| {
                add(
                    a.get(i).copied().unwrap_or(0),
                    b.get(i).copied().unwrap_or(0),
                )
            })
            .collect(),
    )
}
fn p_scale(a: &[u8], c: u8) -> Vec<u8> {
    trim(a.iter().map(|a| mul(*a, c)).collect())
}
fn p_mul(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut r = vec![0; a.len() + b.len() - 1];
    for (i, &a) in a.iter().enumerate() {
        for (j, &b) in b.iter().enumerate() {
            r[i + j] = add(r[i + j], mul(a, b));
        }
    }
    trim(r)
}
fn p_pow(a: &[u8], n: usize) -> Vec<u8> {
    (0..n).fold(vec![1], |p, _| p_mul(&p, a))
}
fn p_eval(a: &[u8], x: u8) -> u8 {
    a.iter().rev().fold(0, |v, c| add(mul(v, x), *c))
}
fn divide_root(a: &[u8], root: u8) -> Vec<u8> {
    assert!(a.len() > 1);
    let mut q = vec![0; a.len() - 1];
    q[a.len() - 2] = *a.last().unwrap();
    for i in (1..a.len() - 1).rev() {
        q[i - 1] = add(a[i], mul(root, q[i]));
    }
    assert_eq!(add(a[0], mul(root, q[0])), 0);
    assert_eq!(p_mul(&q, &[sub(0, root), 1]), a);
    trim(q)
}
fn line_parameter(line: [u8; 3]) -> [[u8; 2]; 3] {
    let pivot = (0..3).rfind(|i| line[*i] != 0).unwrap();
    let free: Vec<_> = (0..3).filter(|i| *i != pivot).collect();
    let mut p = [[0; 2]; 3];
    p[free[0]] = [1, 0];
    p[free[1]] = [0, 1];
    p[pivot] = [
        mul(sub(0, line[free[0]]), inv(line[pivot])),
        mul(sub(0, line[free[1]]), inv(line[pivot])),
    ];
    let annihilation = p.iter().zip(line).fold([0u8; 2], |sum, (coordinate, c)| {
        std::array::from_fn(|j| add(sum[j], mul(c, coordinate[j])))
    });
    assert_eq!(annihilation, [0; 2]);
    println!("FULL_HOMOGENEOUS_LINE_PARAMETER\tline={line:?}\tpivot={pivot}\tfree={free:?}\tcoordinates-U-V={p:?}");
    p
}
fn section(curve: &Poly, p: [[u8; 2]; 3]) -> Vec<u8> {
    let mut q = vec![0];
    for (&(y, z), &c) in curve {
        let term = p_mul(
            &p_mul(&p_pow(&p[0], 4 - y - z), &p_pow(&p[1], y)),
            &p_pow(&p[2], z),
        );
        q = p_add(&q, &p_scale(&term, c));
    }
    assert!(q.len() <= 5 && q != [0]);
    let infinity_order = 5 - q.len();
    let mut full = q.clone();
    full.resize(5, 0);
    println!("COMPLETE_BINARY_QUARTIC_SECTION\tcoeff-U4-U3V-U2V2-UV3-V4={full:?}\taffine-U1={q:?}\tinfinity-U0-order={infinity_order}");
    q
}
fn normalize(p: Point) -> Point {
    let first = p.iter().copied().find(|x| *x != 0).unwrap();
    p.map(|x| mul(x, inv(first)))
}
fn rational_branch(p: Point, multiplicity: usize, rows: &[Datum]) {
    let p = normalize(p);
    let row = rows.iter().find(|r| r.point == p).unwrap();
    assert_eq!(row.omega, 0);
    let line = F_INPUT
        .lines()
        .filter(|s| s.starts_with("ACTUAL_FUNCTION_EVALUATION\t"))
        .find(|s| {
            numbers(field(s, "C="))
                .into_iter()
                .map(|n| u8::try_from(n).unwrap())
                .collect::<Vec<_>>()
                == p
        })
        .unwrap();
    assert_eq!(field(line, "oracle-agrees="), "true");
    println!("ACTUAL_RATIONAL_RAMIFICATION_BRANCH\tC={p:?}\tcanonical-zero-order={multiplicity}\ttame-e={}\tf4={}\timage-source=inherited-independent-polynomial-F",multiplicity+1,field(line,"f4="));
}
fn factor_section(
    poly: &[u8],
    parameter: [[u8; 2]; 3],
    rows: &[Datum],
) -> (Vec<u8>, Vec<(u8, usize)>) {
    let infinity = 5 - poly.len();
    if infinity > 0 {
        rational_branch(parameter.map(|a| a[1]), infinity, rows);
    }
    let mut rest = poly.to_vec();
    let mut factors = Vec::new();
    for r in 0..7u8 {
        let mut count = 0;
        while rest.len() > 1 && p_eval(&rest, r) == 0 {
            let old = rest.clone();
            rest = divide_root(&rest, r);
            count += 1;
            println!("FULL_ACTUAL_ROOT_DIVISION\troot={r}\tinput={old:?}\tquotient={rest:?}\texact-product=true");
        }
        if count > 0 {
            factors.push((r, count));
            rational_branch(parameter.map(|a| add(a[0], mul(a[1], r))), count, rows);
        }
    }
    let mut reconstructed = rest.clone();
    for &(r, n) in &factors {
        reconstructed = p_mul(&reconstructed, &p_pow(&[sub(0, r), 1], n));
    }
    assert_eq!(reconstructed, poly);
    assert_eq!(
        infinity + factors.iter().map(|(_, n)| n).sum::<usize>() + rest.len() - 1,
        4
    );
    let scale = *rest.last().unwrap();
    let monic = p_scale(&rest, inv(scale));
    let evaluations: Vec<_> = (0..7u8).map(|r| (r, p_eval(&monic, r))).collect();
    if monic.len() > 1 {
        assert!(evaluations.iter().all(|(_, v)| *v != 0));
    }
    println!("COMPLETE_ACTUAL_RAMIFICATION_FACTOR_CERTIFICATE\taffine={poly:?}\tinfinity-order={infinity}\trational-factors={factors:?}\tresidual-leading={scale}\tresidual-monic={monic:?}\tall7-residual-evaluations={evaluations:?}\ttotal-degree=4\tfull-product-equal=true\tquadratic-irreducible={}",monic.len()==3);
    (rest, factors)
}
fn main() {
    println!("PROFILE\tactual-CM19-canonical-ramification-B\tfixed-exact-correctness\tunsigned-actual-geometry-bridges-explicit");
    let curve = actual_curve();
    let rows = actual_values(&curve);
    let line = recover_line(&rows);
    println!("ACTUAL_GLOBAL_CANONICAL_RAMIFICATION_LINE\tcoeff-X-Y-Z={line:?}\tglobal-equality=canonical-basis-plus17>4\tzero-divisor-degree=4");
    let parameter = line_parameter(line);
    let quartic = section(&curve, parameter);
    let (rest, factors) = factor_section(&quartic, parameter, &rows);
    let mut bad = line;
    bad[0] = add(bad[0], 1);
    let changed = rows
        .iter()
        .filter(|r| {
            r.point
                .into_iter()
                .zip(bad)
                .map(|(a, b)| mul(a, b))
                .fold(0, add)
                != r.value
        })
        .count();
    assert!(changed > 0);
    let mut product = rest;
    for (root, count) in factors {
        product = p_mul(&product, &p_pow(&[sub(0, root), 1], count));
    }
    let mut changed_section = quartic.clone();
    changed_section[0] = add(changed_section[0], 1);
    assert_ne!(changed_section, product);
    changed_section[0] = sub(changed_section[0], 1);
    assert_eq!(changed_section, product);
    println!("ACTUAL_RAMIFICATION_CONTROLS\tchanged-real-linear-coefficient-rejected-rows={changed}\tchanged-real-binary-coefficient-rejected-full-product\tpristine-restored");
    println!("SUMMARY\tall17-actual-canonical-values\trank3-complete-line-certificate\tfull-homogeneous-line-quartic-with-infinity\tcomplete-rational-deflation-residual-algebra\ttotal-ramification-degree4\tresidual-extension-images=NEXT-SEPARATE-FROZEN-SOURCE\tfull-geometric-formalization=OPEN");
}
