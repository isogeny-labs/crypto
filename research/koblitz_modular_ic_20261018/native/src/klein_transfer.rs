use crate::gf2k::SmallField;
use crate::round02::{Elliptic, Point};
use serde_json::{json, Value};
type P = [u16; 3];
type Series = Vec<u16>;
fn normalize(f: &SmallField, mut p: P) -> P {
    let scale = f.inv(*p.iter().find(|x| **x != 0).unwrap());
    for x in &mut p {
        *x = f.mul(*x, scale);
    }
    p
}
fn on(f: &SmallField, p: P) -> bool {
    let [x, y, z] = p;
    f.mul(f.pow(x, 3), y) ^ f.mul(f.pow(y, 3), z) ^ f.mul(f.pow(z, 3), x) == 0
}
fn dot(f: &SmallField, a: P, b: P) -> u16 {
    f.mul(a[0], b[0]) ^ f.mul(a[1], b[1]) ^ f.mul(a[2], b[2])
}
fn mv(f: &SmallField, m: &[P; 3], p: P) -> P {
    normalize(f, [dot(f, m[0], p), dot(f, m[1], p), dot(f, m[2], p)])
}
fn det(f: &SmallField, m: &[P; 3]) -> u16 {
    f.mul(m[0][0], f.mul(m[1][1], m[2][2]) ^ f.mul(m[1][2], m[2][1]))
        ^ f.mul(m[0][1], f.mul(m[1][0], m[2][2]) ^ f.mul(m[1][2], m[2][0]))
        ^ f.mul(m[0][2], f.mul(m[1][0], m[2][1]) ^ f.mul(m[1][1], m[2][0]))
}
fn quotient(f: &SmallField, p: P) -> Point {
    let [x, y, z] = p;
    let a = x ^ y ^ z;
    let b = f.mul(x, y) ^ f.mul(y, z) ^ f.mul(z, x);
    let c = f.mul(f.mul(x, y), z);
    if b == 0 {
        assert!(a != 0);
        if c == 0 {
            Point::Aff(0, 1)
        } else {
            Point::O
        }
    } else {
        Point::Aff(
            f.mul(f.mul(a, c), f.inv(f.sqr(b))),
            1 ^ f.mul(f.mul(f.pow(a, 3), c), f.inv(f.pow(b, 3))),
        )
    }
}
fn smul(f: &SmallField, a: &[u16], b: &[u16], n: usize) -> Series {
    let mut c = vec![0; n];
    for (i, x) in a.iter().enumerate().take(n) {
        for (j, y) in b.iter().enumerate().take(n - i) {
            c[i + j] ^= f.mul(*x, *y);
        }
    }
    c
}
fn spow(f: &SmallField, a: &[u16], k: usize, n: usize) -> Series {
    let mut p = vec![0; n];
    p[0] = 1;
    for _ in 0..k {
        p = smul(f, &p, a, n);
    }
    p
}
fn curve_series(f: &SmallField, v: &[Series; 3], n: usize) -> Series {
    let mut out = vec![0; n];
    for (i, j) in [(0, 1), (1, 2), (2, 0)] {
        let term = smul(f, &spow(f, &v[i], 3, n), &v[j], n);
        for k in 0..n {
            out[k] ^= term[k];
        }
    }
    out
}
fn jets(f: &SmallField, p: P, n: usize) -> [Series; 3] {
    let fixed = p.iter().position(|x| *x != 0).unwrap();
    let other: Vec<_> = (0..3).filter(|i| *i != fixed).collect();
    let grad = [
        f.mul(f.sqr(p[0]), p[1]) ^ f.pow(p[2], 3),
        f.pow(p[0], 3) ^ f.mul(f.sqr(p[1]), p[2]),
        f.pow(p[1], 3) ^ f.mul(f.sqr(p[2]), p[0]),
    ];
    let (u, v) = if grad[other[1]] != 0 {
        (other[0], other[1])
    } else {
        assert!(grad[other[0]] != 0);
        (other[1], other[0])
    };
    let mut coords: [Series; 3] = std::array::from_fn(|i| {
        let mut s = vec![0; n];
        s[0] = p[i];
        s
    });
    if n > 1 {
        coords[u][1] = 1;
    }
    for k in 1..n {
        let residual = curve_series(f, &coords, k + 1)[k];
        coords[v][k] = f.mul(residual, f.inv(grad[v]));
    }
    assert!(curve_series(f, &coords, n).iter().all(|x| *x == 0));
    coords
}
fn line_divisor(f: &SmallField, pts: &[P], line: P) -> Option<Vec<i32>> {
    let mut d = vec![0; pts.len()];
    for (i, p) in pts.iter().enumerate() {
        if dot(f, line, *p) != 0 {
            continue;
        }
        let j = jets(f, *p, 5);
        let val: Vec<_> = (0..5)
            .map(|k| f.mul(line[0], j[0][k]) ^ f.mul(line[1], j[1][k]) ^ f.mul(line[2], j[2][k]))
            .collect();
        let order = val.iter().position(|v| *v != 0)?;
        d[i] = order as i32;
    }
    if d.iter().sum::<i32>() == 4 {
        Some(d)
    } else {
        None
    }
}
fn null_vector(f: &SmallField, mut rows: Vec<Vec<u16>>, cols: usize) -> Option<Vec<u16>> {
    let mut pivots = Vec::new();
    for c in 0..cols {
        let Some(p) = (pivots.len()..rows.len()).find(|i| rows[*i][c] != 0) else {
            continue;
        };
        let r = pivots.len();
        rows.swap(p, r);
        let inv = f.inv(rows[r][c]);
        for j in c..cols {
            rows[r][j] = f.mul(rows[r][j], inv);
        }
        for i in 0..rows.len() {
            if i == r || rows[i][c] == 0 {
                continue;
            }
            let factor = rows[i][c];
            for j in c..cols {
                rows[i][j] ^= f.mul(factor, rows[r][j]);
            }
        }
        pivots.push(c);
    }
    if pivots.len() == cols {
        return None;
    }
    let free = (0..cols).find(|c| !pivots.contains(c)).unwrap();
    let mut out = vec![0; cols];
    out[free] = 1;
    for (i, c) in pivots.iter().enumerate() {
        out[*c] = rows[i][free];
    }
    Some(out)
}
fn ensure_line(f: &SmallField, pts: &[P], lines: &mut Vec<(P, Vec<i32>)>, i: usize) {
    if lines.iter().any(|(_, v)| v[i] > 0) {
        return;
    }
    let p = pts[i];
    for q in pts {
        if *q == p {
            continue;
        }
        let line = normalize(
            f,
            [
                f.mul(p[1], q[2]) ^ f.mul(p[2], q[1]),
                f.mul(p[0], q[2]) ^ f.mul(p[2], q[0]),
                f.mul(p[0], q[1]) ^ f.mul(p[1], q[0]),
            ],
        );
        if let Some(div) = line_divisor(f, pts, line) {
            lines.push((line, div));
            return;
        }
    }
    panic!("no split denominator line through support point")
}
fn principal(
    f: &SmallField,
    pts: &[P],
    lines: &mut Vec<(P, Vec<i32>)>,
    divisor: &[i32],
) -> Option<Value> {
    assert_eq!(divisor.iter().sum::<i32>(), 0);
    let mut denominator = vec![0i32; pts.len()];
    let mut factors = Vec::new();
    for (i, d) in divisor.iter().enumerate() {
        while denominator[i] + d < 0 {
            ensure_line(f, pts, lines, i);
            let (line, ld) = lines
                .iter()
                .filter(|(_, v)| v[i] > 0)
                .max_by_key(|(_, v)| v[i])
                .expect("split line through negative support");
            for (j, v) in ld.iter().enumerate() {
                denominator[j] += v;
            }
            factors.push(*line);
        }
    }
    let degree = factors.len();
    let monomials: Vec<_> = (0..=degree)
        .flat_map(|i| (0..=degree - i).map(move |j| [i, j, degree - i - j]))
        .filter(|m| !(m[0] >= 3 && m[1] >= 1))
        .collect();
    let mut matrix = Vec::new();
    for (p_idx, p) in pts.iter().enumerate() {
        let order = (denominator[p_idx] + divisor[p_idx]) as usize;
        if order == 0 {
            continue;
        }
        let coords = jets(f, *p, order.max(2));
        let terms: Vec<_> = monomials
            .iter()
            .map(|m| {
                smul(
                    f,
                    &smul(
                        f,
                        &spow(f, &coords[0], m[0], order),
                        &spow(f, &coords[1], m[1], order),
                        order,
                    ),
                    &spow(f, &coords[2], m[2], order),
                    order,
                )
            })
            .collect();
        for k in 0..order {
            matrix.push(terms.iter().map(|a| a[k]).collect::<Vec<_>>());
        }
    }
    let numerator = null_vector(f, matrix.clone(), monomials.len())?;
    assert!(numerator.iter().any(|x| *x != 0));
    for row in &matrix {
        let value = row
            .iter()
            .zip(&numerator)
            .fold(0, |sum, (a, b)| sum ^ f.mul(*a, *b));
        assert_eq!(value, 0);
    }
    Some(
        json!({"degree":degree,"monomials":monomials,"numerator_coefficients_F8":numerator,"denominator_lines_F8":factors,"divisor":divisor,"interpolation_rows":matrix.len(),"coefficient_nullspace_verified":true}),
    )
}
fn automorphisms(f: &SmallField, pts: &[P]) -> Vec<[P; 3]> {
    let mut out = Vec::new();
    for a in pts {
        for b in pts {
            if a == b {
                continue;
            }
            for c in pts {
                if c == a || c == b {
                    continue;
                }
                for sb in 1..8 {
                    for sc in 1..8 {
                        let m = std::array::from_fn(|i| [a[i], f.mul(sb, b[i]), f.mul(sc, c[i])]);
                        if det(f, &m) == 0 {
                            continue;
                        }
                        if pts.iter().all(|p| on(f, mv(f, &m, *p))) {
                            out.push(m);
                        }
                    }
                }
            }
        }
    }
    assert_eq!(out.len(), 168);
    out
}
pub fn run(path: &std::path::Path) {
    run_field(path, 3);
}
pub fn run_wide(path: &std::path::Path) {
    run_field(path, 9);
}
fn run_field(path: &std::path::Path, k: u32) {
    assert!(!path.exists());
    let f = SmallField::new(k);
    let e = Elliptic::models()[1];
    let mut pts = Vec::new();
    for x in 0..f.q as u16 {
        for y in 0..f.q as u16 {
            let p = [1, x, y];
            if on(&f, p) {
                pts.push(p);
            }
        }
    }
    for y in 0..f.q as u16 {
        let p = [0, 1, y];
        if on(&f, p) {
            pts.push(p);
        }
    }
    pts.push([0, 0, 1]);
    if k == 3 {
        assert_eq!(pts.len(), 24);
    }
    let all_lines: Vec<_> = if k == 3 {
        (0..8)
            .flat_map(|x| (0..8).map(move |y| [1, x, y]))
            .chain((0..8).map(|y| [0, 1, y]))
            .chain(std::iter::once([0, 0, 1]))
            .collect()
    } else {
        vec![[1, 0, 0], [0, 1, 0], [0, 0, 1]]
    };
    let mut split_lines: Vec<_> = all_lines
        .into_iter()
        .filter_map(|l| line_divisor(&f, &pts, l).map(|d| (l, d)))
        .collect();
    let p0 = pts.iter().position(|p| *p == [1, 0, 0]).unwrap();
    let p1 = pts.iter().position(|p| *p == [0, 1, 0]).unwrap();
    let mut d1 = vec![0i32; pts.len()];
    d1[p1] = 1;
    d1[p0] = -1;
    assert!(principal(&f, &pts, &mut split_lines, &d1).is_none());
    let d7: Vec<_> = d1.iter().map(|d| 7 * d).collect();
    let d7cert =
        principal(&f, &pts, &mut split_lines, &d7).expect("7 times flex difference is principal");
    let frob: Vec<_> = pts
        .iter()
        .map(|p| {
            pts.iter()
                .position(|q| *q == normalize(&f, p.map(|x| f.sqr(x))))
                .unwrap()
        })
        .collect();
    let autos = if k == 3 {
        automorphisms(&f, &pts)
    } else {
        let f8 = SmallField::new(3);
        let small: Vec<_> = pts
            .iter()
            .filter(|p| p.iter().all(|v| f.pow(*v, 8) == *v))
            .copied()
            .collect();
        assert_eq!(small.len(), 24);
        let root = (0..f.q as u16)
            .find(|v| f.pow(*v, 3) ^ *v ^ 1 == 0)
            .unwrap();
        let embedding: Vec<_> = (0..8)
            .map(|v| {
                let mut x = 0;
                for i in 0..3 {
                    if v >> i & 1 == 1 {
                        x ^= f.pow(root, i);
                    }
                }
                x
            })
            .collect();
        let original: Vec<_> = small
            .iter()
            .map(|p| p.map(|v| embedding.iter().position(|a| *a == v).unwrap() as u16))
            .collect();
        automorphisms(&f8, &original)
            .into_iter()
            .map(|m| m.map(|p| p.map(|v| embedding[v as usize])))
            .collect()
    };
    println!("KT0: all 168 automorphisms enumerated; flex divisor has exact order 7");
    let epts = e.points(&f);
    let subgroup: Vec<_> = epts
        .iter()
        .copied()
        .filter(|p| e.mul(&f, *p, 7) == Point::O)
        .collect();
    assert_eq!(subgroup.len(), 7);
    let g = *subgroup.iter().find(|p| **p != Point::O).unwrap();
    if let Point::Aff(x, y) = g {
        assert_eq!((f.pow(x, 8), f.pow(y, 8)), (x, y));
    }
    let q = e.mul(&f, g, 2);
    let mut maps = Vec::new();
    let mut seen = Vec::new();
    let mut first_nonzero = None;
    for m in &autos {
        let images: Vec<_> = pts
            .iter()
            .map(|p| {
                let target = quotient(&f, mv(&f, m, *p));
                assert!(e.on(&f, target));
                epts.iter().position(|q| *q == target).unwrap()
            })
            .collect();
        if seen.contains(&images) {
            continue;
        }
        seen.push(images.clone());
        let mut records = Vec::new();
        for scalar in [1u64, 2, 0, 3, 4, 5, 6] {
            let target = e.mul(&f, g, scalar);
            let pull: Vec<_> = images
                .iter()
                .enumerate()
                .filter(|(_, j)| epts[**j] == target)
                .map(|(i, _)| i)
                .collect();
            let zero: Vec<_> = images
                .iter()
                .enumerate()
                .filter(|(_, j)| epts[**j] == Point::O)
                .map(|(i, _)| i)
                .collect();
            if pull.len() != 3 || zero.len() != 3 {
                records.push(json!({"target":target,"supported_split_fiber":false,"pull_points":pull,"origin_points":zero}));
                continue;
            }
            let mut push = Point::O;
            for _ in &pull {
                push = e.add(&f, push, target);
            }
            assert_eq!(push, e.mul(&f, target, 3));
            let mut traced = vec![0i32; pts.len()];
            for (i, sign) in pull
                .iter()
                .map(|i| (*i, 1))
                .chain(zero.iter().map(|i| (*i, -1)))
            {
                let mut j = i;
                for _ in 0..3 {
                    traced[j] += sign;
                    j = frob[j];
                }
            }
            assert!((0..pts.len()).all(|i| traced[i] == traced[frob[i]]));
            let mut found = Vec::new();
            for k in 0..7 {
                let diff: Vec<_> = traced.iter().zip(&d1).map(|(a, b)| a - k * b).collect();
                if let Some(cert) = principal(&f, &pts, &mut split_lines, &diff) {
                    found.push((k, cert));
                }
            }
            assert_eq!(found.len(), 1, "one and only one class in F2 7-subgroup");
            let (k, cert) = found.pop().unwrap();
            records.push(json!({"target":target,"source_scalar":scalar,"supported_split_fiber":true,"trace_divisor":traced,"class_multiple_of_D":k,"principal_certificate":cert}));
        }
        if records.iter().all(|r| r["supported_split_fiber"] == true) {
            let k = records[0]["class_multiple_of_D"].as_i64().unwrap();
            let k2 = records[1]["class_multiple_of_D"].as_i64().unwrap();
            assert_eq!(k2, 2 * k % 7);
            for record in &records {
                assert_eq!(
                    record["class_multiple_of_D"].as_i64().unwrap(),
                    record["source_scalar"].as_i64().unwrap() * k % 7
                );
            }
            if k != 0 && first_nonzero.is_none() {
                first_nonzero = Some(maps.len());
            }
        }
        maps.push(json!({"matrix":m,"target_images":images,"records":records}));
    }
    let result = json!({"schema":"koblitz.experiment/kt0-v3","coordinate_field_degree":k,"field_modulus":f.poly,"curve_points":pts,"split_lines":split_lines,"automorphisms":autos.len(),"distinct_quotient_maps":maps.len(),"flex_divisor_order":7,"flex_seven_principal_certificate":d7cert,"source_generator_E1_F8":g,"second_point_is_twice_generator":q,"maps":maps,"first_nonzero_transfer_map":first_nonzero,"source_scalars_checked":[0,1,2,3,4,5,6],"scope":"degree-three quotients, F8 source subgroup and Frobenius traces, with principal-divisor certificates; coordinate extension recorded explicitly"});
    std::fs::write(path, serde_json::to_string_pretty(&result).unwrap() + "\n").unwrap();
    println!(
        "KT0 complete: {} distinct quotient maps; first nonzero transfer {:?}",
        seen.len(),
        first_nonzero
    );
}

fn verify_principal(f: &SmallField, pts: &[P], cert: &Value, expected: &[i32]) {
    let degree = cert["degree"].as_u64().unwrap() as usize;
    let monos: Vec<[usize; 3]> = serde_json::from_value(cert["monomials"].clone()).unwrap();
    let coefficients: Vec<u16> =
        serde_json::from_value(cert["numerator_coefficients_F8"].clone()).unwrap();
    let lines: Vec<P> = serde_json::from_value(cert["denominator_lines_F8"].clone()).unwrap();
    let divisor: Vec<i32> = serde_json::from_value(cert["divisor"].clone()).unwrap();
    assert_eq!(divisor, expected);
    assert_eq!(divisor.iter().sum::<i32>(), 0);
    let standard: Vec<_> = (0..=degree)
        .flat_map(|i| (0..=degree - i).map(move |j| [i, j, degree - i - j]))
        .filter(|m| !(m[0] >= 3 && m[1] >= 1))
        .collect();
    assert_eq!(monos, standard);
    assert_eq!(coefficients.len(), monos.len());
    assert!(coefficients.iter().any(|c| *c != 0));
    assert_eq!(lines.len(), degree);
    let mut poles = vec![0i32; pts.len()];
    for line in lines {
        let ld = line_divisor(f, pts, line).expect("fully split denominator line");
        for (i, v) in ld.iter().enumerate() {
            poles[i] += v;
        }
    }
    for (i, p) in pts.iter().enumerate() {
        let order = poles[i] + divisor[i];
        assert!(order >= 0);
        if order == 0 {
            continue;
        }
        let n = order as usize;
        let coords = jets(f, *p, n.max(2));
        let mut expansion = vec![0; n];
        for (m, c) in monos.iter().zip(&coefficients) {
            let term = smul(
                f,
                &smul(
                    f,
                    &spow(f, &coords[0], m[0], n),
                    &spow(f, &coords[1], m[1], n),
                    n,
                ),
                &spow(f, &coords[2], m[2], n),
                n,
            );
            for j in 0..n {
                expansion[j] ^= f.mul(*c, term[j]);
            }
        }
        assert!(expansion.iter().all(|c| *c == 0));
    }
}
fn verify_matrix_polynomial(f: &SmallField, m: &[P; 3]) {
    use std::collections::BTreeMap;
    let mut coefficients = BTreeMap::new();
    for (r, s) in [(0, 1), (1, 2), (2, 0)] {
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    for l in 0..3 {
                        let mut exponents = [0usize; 3];
                        for index in [i, j, k, l] {
                            exponents[index] += 1;
                        }
                        let value = f.mul(f.mul(f.mul(m[r][i], m[r][j]), m[r][k]), m[s][l]);
                        *coefficients.entry(exponents).or_insert(0u16) ^= value;
                    }
                }
            }
        }
    }
    coefficients.retain(|_, c| *c != 0);
    let scalar = *coefficients.get(&[3, 1, 0]).unwrap();
    assert!(scalar != 0);
    let expected: BTreeMap<_, _> = [
        ([3, 1, 0], scalar),
        ([0, 3, 1], scalar),
        ([1, 0, 3], scalar),
    ]
    .into_iter()
    .collect();
    assert_eq!(coefficients, expected);
    assert!(det(f, m) != 0);
}
pub fn verify(path: &std::path::Path) {
    let data: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    let field_degree = data["coordinate_field_degree"].as_u64().unwrap() as u32;
    let f = SmallField::new(field_degree);
    assert_eq!(data["field_modulus"], f.poly);
    let pts: Vec<P> = serde_json::from_value(data["curve_points"].clone()).unwrap();
    assert!(pts.iter().all(|p| on(&f, *p)));
    let e = Elliptic::models()[1];
    let g: Point = serde_json::from_value(data["source_generator_E1_F8"].clone()).unwrap();
    assert!(g != Point::O && e.on(&f, g));
    assert_eq!(e.mul(&f, g, 7), Point::O);
    let p0 = pts.iter().position(|p| *p == [1, 0, 0]).unwrap();
    let p1 = pts.iter().position(|p| *p == [0, 1, 0]).unwrap();
    let mut d = vec![0i32; pts.len()];
    d[p0] = -1;
    d[p1] = 1;
    verify_principal(
        &f,
        &pts,
        &data["flex_seven_principal_certificate"],
        &d.iter().map(|v| 7 * v).collect::<Vec<_>>(),
    );
    let frob: Vec<_> = pts
        .iter()
        .map(|p| {
            pts.iter()
                .position(|q| *q == normalize(&f, p.map(|x| f.sqr(x))))
                .unwrap()
        })
        .collect();
    let mut checks = 0;
    let mut nonzero = 0;
    let mut zero = 0;
    for map in data["maps"].as_array().unwrap() {
        let m: [P; 3] = serde_json::from_value(map["matrix"].clone()).unwrap();
        verify_matrix_polynomial(&f, &m);
        let records = map["records"].as_array().unwrap();
        assert_eq!(records.len(), 7);
        let gain = records[0]["class_multiple_of_D"].as_i64().unwrap();
        if gain == 0 {
            zero += 1
        } else {
            nonzero += 1;
        }
        for record in records {
            assert_eq!(record["supported_split_fiber"], true);
            let scalar = record["source_scalar"].as_u64().unwrap();
            let target = e.mul(&f, g, scalar);
            assert_eq!(
                serde_json::from_value::<Point>(record["target"].clone()).unwrap(),
                target
            );
            let mut div = vec![0i32; pts.len()];
            let mut fiber = 0;
            let mut origin = 0;
            for (i, p) in pts.iter().enumerate() {
                let value = quotient(&f, mv(&f, &m, *p));
                let sign = if value == target {
                    fiber += 1;
                    1
                } else {
                    0
                } - if value == Point::O {
                    origin += 1;
                    1
                } else {
                    0
                };
                let mut j = i;
                for _ in 0..3 {
                    div[j] += sign;
                    j = frob[j];
                }
            }
            assert_eq!((fiber, origin), (3, 3));
            assert_eq!(
                serde_json::from_value::<Vec<i32>>(record["trace_divisor"].clone()).unwrap(),
                div
            );
            let multiple = record["class_multiple_of_D"].as_i64().unwrap();
            assert_eq!(multiple, scalar as i64 * gain % 7);
            let difference: Vec<_> = div
                .iter()
                .zip(&d)
                .map(|(a, b)| a - multiple as i32 * b)
                .collect();
            verify_principal(&f, &pts, &record["principal_certificate"], &difference);
            checks += 1;
        }
    }
    assert_eq!(checks, 392);
    assert_eq!((nonzero, zero), (42, 14));
    println!(
        "{}",
        json!({"schema":"koblitz.klein-certificate-replay/v1","coordinate_field_degree":field_degree,"principal_divisor_checks":checks+1,"faithful_maps":nonzero,"zero_maps":zero,"polynomial_automorphism_checks":56,"uses_gaussian_elimination":false,"status":"PASS"})
    );
}

pub fn line_relations(dir: &std::path::Path, wide: bool) {
    let input: Value = serde_json::from_str(
        &std::fs::read_to_string(dir.join(if wide {
            "kt1-klein-transfer-wide.json"
        } else {
            "kt0-klein-transfer.json"
        }))
        .unwrap(),
    )
    .unwrap();
    let k = if wide { 9 } else { 3 };
    let f = SmallField::new(k);
    let pts: Vec<P> = serde_json::from_value(input["curve_points"].clone()).unwrap();
    let mut lines: Vec<(P, Vec<i32>)> = if wide {
        let all_lines = (0..f.q as u16)
            .flat_map(|x| (0..f.q as u16).map(move |y| [1, x, y]))
            .chain((0..f.q as u16).map(|y| [0, 1, y]))
            .chain(std::iter::once([0, 0, 1]));
        all_lines
            .filter_map(|l| line_divisor(&f, &pts, l).map(|d| (l, d)))
            .collect()
    } else {
        serde_json::from_value(input["split_lines"].clone()).unwrap()
    };
    let frob: Vec<_> = pts
        .iter()
        .map(|p| {
            pts.iter()
                .position(|q| *q == normalize(&f, p.map(|v| f.sqr(v))))
                .unwrap()
        })
        .collect();
    let p0 = pts.iter().position(|p| *p == [1, 0, 0]).unwrap();
    let p1 = pts.iter().position(|p| *p == [0, 1, 0]).unwrap();
    let mut seen = vec![false; pts.len()];
    let mut orbits = Vec::new();
    for i in 0..pts.len() {
        if seen[i] {
            continue;
        }
        let mut orbit = Vec::new();
        let mut j = i;
        while !seen[j] {
            seen[j] = true;
            orbit.push(j);
            j = frob[j];
        }
        if !orbit.contains(&p0) {
            orbits.push(orbit);
        }
    }
    let anchor = orbits.iter().position(|o| o.contains(&p1)).unwrap();
    let hz = line_divisor(&f, &pts, [0, 0, 1]).unwrap();
    let mut rows = Vec::new();
    let mut row_sources = Vec::new();
    for (line, ld) in &lines {
        let mut traced = vec![0i32; pts.len()];
        for i in 0..pts.len() {
            let mut j = i;
            for _ in 0..k {
                traced[j] += ld[i] - hz[i];
                j = frob[j];
            }
        }
        let row: Vec<_> = orbits
            .iter()
            .map(|o| {
                assert!(o.iter().all(|i| traced[*i] == traced[o[0]]));
                traced[o[0]].rem_euclid(7) as u64
            })
            .collect();
        rows.push(row);
        row_sources.push(json!({"line":line,"norm_principal_divisor":traced}));
    }
    let (reduced, pivots) = crate::gluing::rref(rows.clone(), 7);
    let columns = orbits.len();
    let mut solved = None;
    let mut certificates = Vec::new();
    if pivots.len() == columns - 1 {
        let free = (0..columns).find(|i| !pivots.contains(i)).unwrap();
        let mut logs = vec![0u64; columns];
        logs[free] = 1;
        for (i, c) in pivots.iter().enumerate() {
            logs[*c] = (7 - reduced[i][free]) % 7;
        }
        assert!(logs[anchor] != 0);
        let inv = (1..7).find(|i| logs[anchor] * i % 7 == 1).unwrap();
        for l in &mut logs {
            *l = *l * inv % 7;
        }
        for row in &rows {
            assert_eq!(
                row.iter().zip(&logs).map(|(a, b)| a * b).sum::<u64>() % 7,
                0
            );
        }
        for (column, orbit) in orbits.iter().enumerate() {
            let mut div = vec![0i32; pts.len()];
            for p in orbit {
                div[*p] += 2;
            }
            div[p0] -= 2 * orbit.len() as i32;
            div[p1] -= 2 * logs[column] as i32;
            div[p0] += 2 * logs[column] as i32;
            let cert = principal(&f, &pts, &mut lines, &div)
                .expect("line-solved class must match independent interpolation");
            verify_principal(&f, &pts, &cert, &div);
            certificates.push(cert);
        }
        solved = Some(logs);
    }
    // Only geometric point/fiber data enter this solver. Previously computed
    // target classes and principal certificates are deliberately not consulted.
    let e = Elliptic::models()[1];
    let g: Point = serde_json::from_value(input["source_generator_E1_F8"].clone()).unwrap();
    let mut recovery = Vec::new();
    let mut selected_map = None;
    if let Some(logs) = &solved {
        let target_log = |m: &[P; 3], target: Point| -> Option<u64> {
            let mut traced = vec![0i32; pts.len()];
            let mut fiber = 0;
            let mut origin = 0;
            for (i, p) in pts.iter().enumerate() {
                let value = quotient(&f, mv(&f, m, *p));
                let sign = if value == target {
                    fiber += 1;
                    1
                } else {
                    0
                } - if value == Point::O {
                    origin += 1;
                    1
                } else {
                    0
                };
                let mut j = i;
                for _ in 0..3 {
                    traced[j] += sign;
                    j = frob[j];
                }
            }
            if (fiber, origin) != (3, 3) {
                return None;
            }
            let mut total = 0i64;
            for (orbit, log) in orbits.iter().zip(logs) {
                assert!(orbit.iter().all(|i| traced[*i] == traced[orbit[0]]));
                total += traced[orbit[0]] as i64 * *log as i64;
            }
            Some(total.rem_euclid(7) as u64)
        };
        for (index, map) in input["maps"].as_array().unwrap().iter().enumerate() {
            let m: [P; 3] = serde_json::from_value(map["matrix"].clone()).unwrap();
            let Some(gain) = target_log(&m, g).filter(|v| *v != 0) else {
                continue;
            };
            selected_map = Some(index);
            let inverse = (1..7).find(|v| v * gain % 7 == 1).unwrap();
            for fixture_scalar in 0..7 {
                let target = e.mul(&f, g, fixture_scalar);
                let recovered = target_log(&m, target).map(|v| v * inverse % 7);
                if let Some(d) = recovered {
                    assert_eq!(e.mul(&f, g, d), target);
                }
                recovery.push(json!({"target":target,"fixture_scalar":fixture_scalar,
                    "recovered_scalar":recovered,"verified_by_multiplication":recovered.is_some(),
                    "failure":if recovered.is_none(){Some("unsupported fiber in coordinate field")}else{None}}));
            }
            break;
        }
    }
    let result = json!({"schema":"koblitz.experiment/mr2-v2","base_field_degree":1,"line_field_degree":k,"closed_point_orbits":orbits,"columns":columns,"line_relations":rows.len(),"rank_mod_7":pivots.len(),"nullity":columns-pivots.len(),"anchor_column":anchor,"relation_matrix":rows,"relation_sources":row_sources,"factor_base_logs_mod_7":solved,"class_verification_certificates":certificates,"selected_transfer_map":selected_map,"target_recovery":recovery,"source":"geometric line functions only; no target logs and no KT class certificates used to construct rows or recover scalars","scope":"direct principal-divisor relations on the Klein modular-curve control"});
    crate::round02::save(
        dir,
        if wide {
            "mr2b-klein-wide-relations.json"
        } else {
            "mr2-klein-line-relations-v2.json"
        },
        &result,
    );
    println!(
        "MR2 complete: {} relations, {} columns, rank {}",
        rows.len(),
        columns,
        pivots.len()
    );
}

pub fn verify_relations(path: &std::path::Path, geometry: &std::path::Path) {
    let data: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    let input: Value = serde_json::from_str(&std::fs::read_to_string(geometry).unwrap()).unwrap();
    let k = data["line_field_degree"].as_u64().unwrap() as u32;
    let f = SmallField::new(k);
    let pts: Vec<P> = serde_json::from_value(input["curve_points"].clone()).unwrap();
    assert!(pts.iter().all(|p| on(&f, *p)));
    let orbits: Vec<Vec<usize>> =
        serde_json::from_value(data["closed_point_orbits"].clone()).unwrap();
    let rows: Vec<Vec<u64>> = serde_json::from_value(data["relation_matrix"].clone()).unwrap();
    let logs: Vec<u64> = serde_json::from_value(data["factor_base_logs_mod_7"].clone()).unwrap();
    assert_eq!(
        rows.len(),
        data["relation_sources"].as_array().unwrap().len()
    );
    assert_eq!(
        rows.len(),
        data["line_relations"].as_u64().unwrap() as usize
    );
    assert_eq!(logs.len(), orbits.len());
    assert!(rows
        .iter()
        .all(|row| row.len() == logs.len() && row.iter().all(|v| *v < 7)));
    assert!(logs.iter().all(|v| *v < 7));
    assert_eq!(
        data["class_verification_certificates"]
            .as_array()
            .unwrap()
            .len(),
        logs.len()
    );
    let frob: Vec<_> = pts
        .iter()
        .map(|p| {
            pts.iter()
                .position(|q| *q == normalize(&f, p.map(|v| f.sqr(v))))
                .unwrap()
        })
        .collect();
    for (row, source) in rows
        .iter()
        .zip(data["relation_sources"].as_array().unwrap())
    {
        let line: P = serde_json::from_value(source["line"].clone()).unwrap();
        let ld = line_divisor(&f, &pts, line).unwrap();
        let hz = line_divisor(&f, &pts, [0, 0, 1]).unwrap();
        let mut norm = vec![0i32; pts.len()];
        for (i, value) in ld.iter().zip(&hz).map(|(a, b)| a - b).enumerate() {
            let mut j = i;
            for _ in 0..k {
                norm[j] += value;
                j = frob[j];
            }
        }
        assert_eq!(
            serde_json::from_value::<Vec<i32>>(source["norm_principal_divisor"].clone()).unwrap(),
            norm
        );
        for (column, orbit) in orbits.iter().enumerate() {
            assert!(orbit.iter().all(|i| norm[*i] == norm[orbit[0]]));
            assert_eq!(row[column], norm[orbit[0]].rem_euclid(7) as u64);
        }
        assert_eq!(
            row.iter().zip(&logs).map(|(a, b)| a * b).sum::<u64>() % 7,
            0
        );
    }
    let big_rows = rows
        .iter()
        .map(|r| r.iter().map(|v| num_bigint::BigInt::from(*v)).collect())
        .collect();
    let rank = crate::round02::rank(big_rows, &num_bigint::BigInt::from(7));
    assert_eq!(rank, data["rank_mod_7"].as_u64().unwrap() as usize);
    assert_eq!(rank + 1, logs.len());
    let p0 = pts.iter().position(|p| *p == [1, 0, 0]).unwrap();
    let p1 = pts.iter().position(|p| *p == [0, 1, 0]).unwrap();
    for (column, orbit) in orbits.iter().enumerate() {
        let mut div = vec![0i32; pts.len()];
        for p in orbit {
            div[*p] += 2;
        }
        div[p0] -= 2 * orbit.len() as i32;
        div[p1] -= 2 * logs[column] as i32;
        div[p0] += 2 * logs[column] as i32;
        verify_principal(
            &f,
            &pts,
            &data["class_verification_certificates"][column],
            &div,
        );
    }
    println!(
        "{}",
        json!({"status":"PASS","line_field_degree":k,"relations_reconstructed":rows.len(),"independent_bigint_rank":rank,"class_certificates_checked":logs.len(),"uses_interpolation_solver":false})
    );
}
