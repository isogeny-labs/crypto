//! n=5 augmentation and same-field cubic-cover relation experiments.
use crate::{
    binary_ecc::{
        BinaryPoint, F2mElement, F2mPoly, IrreduciblePoly, MumfordDivisor, OrdinaryBinaryCover,
    },
    gf2k::SmallField,
    gluing, lpoly,
    round02::{self, Elliptic, Point},
};
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde_json::{json, Value};
use std::path::Path;
type Poly = Vec<u16>;
fn trim(mut p: Poly) -> Poly {
    while p.last() == Some(&0) {
        p.pop();
    }
    p
}
fn add(a: &[u16], b: &[u16]) -> Poly {
    trim(
        (0..a.len().max(b.len()))
            .map(|i| a.get(i).copied().unwrap_or(0) ^ b.get(i).copied().unwrap_or(0))
            .collect(),
    )
}
fn mul(f: &SmallField, a: &[u16], b: &[u16]) -> Poly {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut c = vec![0; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            c[i + j] ^= f.mul(*x, *y);
        }
    }
    trim(c)
}
fn divrem(f: &SmallField, a: &[u16], b: &[u16]) -> (Poly, Poly) {
    assert!(!b.is_empty());
    let mut r = trim(a.to_vec());
    let mut q = vec![0; r.len().saturating_sub(b.len()) + 1];
    while r.len() >= b.len() {
        let d = r.len() - b.len();
        let c = f.mul(*r.last().unwrap(), f.inv(*b.last().unwrap()));
        q[d] ^= c;
        for (i, v) in b.iter().enumerate() {
            r[d + i] ^= f.mul(c, *v);
        }
        r = trim(r);
    }
    (trim(q), r)
}
fn eval(f: &SmallField, p: &[u16], x: u16) -> u16 {
    p.iter().rev().fold(0, |a, c| f.mul(a, x) ^ c)
}
fn small(p: &F2mPoly) -> Poly {
    trim(
        p.coeffs
            .iter()
            .map(|c| c.to_biguint().to_u16().unwrap())
            .collect(),
    )
}
fn element(v: u16) -> F2mElement {
    F2mElement::from_biguint(&BigUint::from(v), 5)
}
fn elliptic_point(p: Point) -> BinaryPoint {
    match p {
        Point::O => BinaryPoint::Infinity,
        Point::Aff(x, y) => BinaryPoint::Affine {
            x: element(x),
            y: element(y),
        },
    }
}
fn norm(f: &SmallField, v: &[u16], a: u16) -> Poly {
    let fp = [0, 1, 0, 0, a, 0, 0, 1];
    add(&add(&mul(f, v, v), &mul(f, v, &[0, 0, 1])), &fp)
}
fn split(f: &SmallField, polynomial: &[u16], v: &[u16], points: &[(u16, u16)]) -> Option<Vec<u64>> {
    let mut p = trim(polynomial.to_vec());
    assert!(!p.is_empty());
    let mut row = vec![0; points.len()];
    for x in 0..32u16 {
        if eval(f, &p, x) != 0 {
            continue;
        }
        let y = eval(f, v, x);
        let index = points.iter().position(|q| *q == (x, y))?;
        loop {
            let (q, r) = divrem(f, &p, &[x, 1]);
            if !r.is_empty() {
                break;
            }
            row[index] += 1;
            p = q;
            if p.len() <= 1 {
                break;
            }
        }
        if p.len() <= 1 {
            break;
        }
    }
    if p.len() == 1 {
        Some(row)
    } else {
        None
    }
}
fn solve(rows: &[Vec<u64>], p: u64) -> Option<(Vec<u64>, usize)> {
    let (r, piv) = gluing::rref(rows.to_vec(), p);
    let n = rows[0].len();
    if piv.len() + 1 != n {
        return None;
    }
    let free = (0..n).find(|c| !piv.contains(c)).unwrap();
    let mut v = vec![0; n];
    v[free] = 1;
    for (i, c) in piv.iter().enumerate() {
        v[*c] = (p - r[i][free]) % p;
    }
    let anchor = v.iter().position(|x| *x != 0).unwrap();
    let inv = (1..p).find(|i| v[anchor] * i % p == 1).unwrap();
    for l in &mut v {
        *l = *l * inv % p;
    }
    Some((v, anchor))
}
fn point_count(a: u16, k: u32) -> i64 {
    let f = SmallField::new(k);
    let mut n = 2; // infinity and u=v=0
    for x in 1..f.q as u16 {
        let x3 = f.pow(x, 3);
        let z = x3 ^ a ^ f.inv(x3);
        if f.trace(z) == 0 {
            n += 2;
        }
    }
    n
}
fn scalar(d: &MumfordDivisor, k: u64, cover: &OrdinaryBinaryCover) -> MumfordDivisor {
    d.scalar_mul(&BigUint::from(k), cover.source_curve())
}
fn div_json(d: &MumfordDivisor) -> Value {
    json!({"u":small(&d.u),"v":small(&d.v)})
}
fn row_class(row: &[u64], divs: &[MumfordDivisor], cover: &OrdinaryBinaryCover) -> MumfordDivisor {
    row.iter()
        .zip(divs)
        .fold(MumfordDivisor::identity(5), |sum, (n, d)| {
            sum.add(&scalar(d, *n, cover), cover.source_curve())
        })
}
fn descend(
    d: &MumfordDivisor,
    offsets: &[MumfordDivisor],
    logs: &[u64],
    points: &[(u16, u16)],
    cover: &OrdinaryBinaryCover,
    f: &SmallField,
    a: u16,
) -> (Option<u64>, Vec<Value>) {
    let mut attempts = Vec::new();
    for offset in 0..=offsets.len().min(8) {
        let shifted = if offset == 0 {
            d.clone()
        } else {
            d.add(&offsets[offset - 1], cover.source_curve())
        };
        if shifted.is_identity() {
            let value = if offset == 0 {
                0
            } else {
                (11 - logs[offset - 1]) % 11
            };
            attempts
                .push(json!({"offset":offset,"trials":0,"identity_after_offset":true,"log":value}));
            return (Some(value), attempts);
        }
        let u = small(&shifted.u);
        let vv = small(&shifted.v);
        for c in 0..32u16 {
            let v = add(&vv, &mul(f, &[c], &u));
            let n = norm(f, &v, a);
            let (residual, rem) = divrem(f, &n, &u);
            assert!(rem.is_empty());
            if let Some(row) = split(f, &residual, &v, points) {
                assert!(shifted
                    .add(&row_class(&row, offsets, cover), cover.source_curve())
                    .is_identity());
                let s = row.iter().zip(logs).map(|(a, b)| a * b).sum::<u64>() % 11;
                let offset_log = if offset == 0 { 0 } else { logs[offset - 1] };
                let result = (22 - s - offset_log) % 11;
                attempts.push(json!({"offset":offset,"trials":c+1,"function_v":v,"input_divisor":div_json(&shifted),"residual_polynomial":residual,"residual_row":row,"log":result,"class_identity_verified":true}));
                return (Some(result), attempts);
            }
        }
        attempts.push(json!({"offset":offset,"trials":32,"status":"no fully split residual"}));
    }
    (None, attempts)
}
fn augmentation(e: Elliptic, f: &SmallField, g: Point) -> Value {
    let mut images = Vec::new();
    for k in 0..11 {
        let p = e.mul(f, g, k);
        let mut tuple = Vec::new();
        let mut q = p;
        for _ in 0..5 {
            tuple.push(q);
            q = match q {
                Point::O => q,
                Point::Aff(x, y) => Point::Aff(f.sqr(x), f.sqr(y)),
            };
        }
        assert_eq!(q, p);
        assert_eq!(
            tuple.iter().fold(Point::O, |s, p| e.add(f, s, *p)),
            Point::O
        );
        for i in 0..5 {
            let previous = tuple[(i + 4) % 5];
            let shifted = match previous {
                Point::O => previous,
                Point::Aff(x, y) => Point::Aff(f.sqr(x), f.sqr(y)),
            };
            assert_eq!(shifted, tuple[i]);
        }
        images.push(tuple);
    }
    for i in 0..11 {
        for j in 0..11 {
            for col in 0..5 {
                assert_eq!(
                    e.add(f, images[i][col], images[j][col]),
                    images[(i + j) % 11][col]
                );
            }
        }
    }
    json!({"images":images,"order":11,"frobenius_fixed_and_trace_zero":true,"addition_checks":121,"quotient_coordinates_evaluated":false,"scope":"actual augmentation-model points; no unevaluated ppav quotient mislabeled as a coordinate map"})
}
pub fn run(dir: &Path) {
    run_collection(dir, false, false, false);
}
pub fn run_vertical(dir: &Path) {
    run_collection(dir, true, false, false);
}
pub fn run_quartic(dir: &Path) {
    run_collection(dir, true, true, false);
}
pub fn run_hermite(dir: &Path) {
    run_collection(dir, true, true, true);
}
fn run_collection(dir: &Path, vertical: bool, quartic: bool, hermite: bool) {
    let f = SmallField::new(5);
    let irr = IrreduciblePoly {
        degree: 5,
        low_terms: (0..5).filter(|i| f.poly >> i & 1 == 1).collect(),
    };
    for x in 0..32u16 {
        for y in 0..32u16 {
            assert_eq!(
                f.mul(x, y),
                element(x)
                    .mul(&element(y), &irr)
                    .to_biguint()
                    .to_u16()
                    .unwrap()
            );
        }
    }
    let mut records = Vec::new();
    for a in [0u16, 1] {
        let e = Elliptic::models()[a as usize];
        let epts = e.points(&f);
        let g = epts
            .iter()
            .map(|p| e.mul(&f, *p, epts.len() as u64 / 11))
            .find(|p| *p != Point::O)
            .unwrap();
        assert_eq!(e.mul(&f, g, 11), Point::O);
        let augmentation = augmentation(e, &f, g);
        let cover = OrdinaryBinaryCover::new(5, irr.clone(), element(a), element(1)).unwrap();
        cover.verify_certificate().unwrap();
        let curve = cover.source_curve();
        let mut points = Vec::new();
        let mut divs = Vec::new();
        for x in 0..32u16 {
            for y in 0..32u16 {
                let xx = element(x);
                let yy = element(y);
                if curve.is_on_curve(&xx, &yy) {
                    points.push((x, y));
                    divs.push(MumfordDivisor::try_from_point(curve, &xx, &yy).unwrap());
                }
            }
        }
        let counts: Vec<_> = [5, 10, 15].into_iter().map(|k| point_count(a, k)).collect();
        assert_eq!(counts[0], points.len() as i64 + 1);
        let polynomial = lpoly::char_poly_from_counts(&counts, 32).unwrap();
        let jac_order = polynomial.iter().sum::<i128>() as u64;
        assert_eq!(jac_order % 11, 0);
        let cofactor = jac_order / 11;
        assert_ne!(cofactor % 11, 0);
        let mut rows = Vec::new();
        let mut functions = Vec::new();
        if vertical {
            for x in 0..32u16 {
                let fiber: Vec<_> = points
                    .iter()
                    .enumerate()
                    .filter(|(_, p)| p.0 == x)
                    .map(|(i, _)| i)
                    .collect();
                if fiber.is_empty() {
                    continue;
                }
                let mut row = vec![0u64; points.len()];
                if fiber.len() == 1 {
                    row[fiber[0]] = 2;
                } else {
                    assert_eq!(fiber.len(), 2);
                    for i in fiber {
                        row[i] = 1;
                    }
                }
                assert!(row_class(&row, &divs, &cover).is_identity());
                rows.push(row.clone());
                functions.push(json!({"function":"u-c","c":x,"multiplicities":row}));
            }
        }
        for leading in 0..if quartic { 2 } else { 1 } {
            for code in 0..1u32 << 20 {
                let mut v: Vec<_> = (0..4).map(|i| ((code >> (5 * i)) & 31) as u16).collect();
                if leading == 1 {
                    v.push(1);
                }
                if let Some(row) = split(&f, &norm(&f, &v, a), &v, &points) {
                    assert_eq!(row.iter().sum::<u64>(), if leading == 1 { 8 } else { 7 });
                    assert!(row_class(&row, &divs, &cover).is_identity());
                    rows.push(row.iter().map(|c| c % 11).collect::<Vec<_>>());
                    functions.push(json!({"v":v,"multiplicities":row}));
                }
            }
        }
        let mut interpolation = Value::Null;
        if hermite && a == 1 {
            let (polys, divisor_count) = hermite_quartics(&f, &points, a);
            let all: std::collections::HashSet<_> = polys.iter().cloned().collect();
            let mut seen = std::collections::HashSet::new();
            for record in &functions {
                if let Some(v) = record.get("v") {
                    let mut v: Poly = serde_json::from_value(v.clone()).unwrap();
                    v.resize(5, 0);
                    assert!(all.contains(&v));
                    seen.insert(v);
                }
            }
            let old = rows.len();
            for v in &polys {
                if !seen.insert(v.clone()) {
                    continue;
                }
                if let Some(row) = split(&f, &norm(&f, v, a), v, &points) {
                    assert!(row_class(&row, &divs, &cover).is_identity());
                    rows.push(row.iter().map(|v| v % 11).collect());
                    functions.push(json!({"v":v,"multiplicities":row,"source":"Hermite degree-five subdivision"}));
                }
            }
            interpolation = json!({"effective_divisors":divisor_count,"unique_polynomials":polys.len(),"new_relations":rows.len()-old,"prior_y_plus_v_coverage_verified":true});
        }
        let rank = gluing::rref(rows.clone(), 11).1.len();
        let solved = solve(&rows, 11);
        let mut log_records = Vec::new();
        let mut recovery = Vec::new();
        if let Some((logs, anchor)) = &solved {
            let projected_base = scalar(&divs[*anchor], cofactor, &cover);
            assert!(!projected_base.is_identity());
            assert!(scalar(&projected_base, 11, &cover).is_identity());
            for (i, d) in divs.iter().enumerate() {
                assert_eq!(
                    scalar(d, cofactor, &cover),
                    scalar(&projected_base, logs[i], &cover)
                );
                log_records
                    .push(json!({"point":points[i],"log":logs[i],"projected_class_verified":true}));
            }
            let image_g = cover.pullback_point_class(&elliptic_point(g)).unwrap();
            assert!(!image_g.is_identity());
            assert!(scalar(&image_g, 11, &cover).is_identity());
            let (gain, base_attempts) = descend(&image_g, &divs, logs, &points, &cover, &f, a);
            if let Some(gain) = gain {
                assert_ne!(gain, 0);
                let inv = (1..11).find(|i| gain * i % 11 == 1).unwrap();
                for k in 0..11 {
                    let q = e.mul(&f, g, k);
                    let image = cover.pullback_point_class(&elliptic_point(q)).unwrap();
                    assert_eq!(image, scalar(&image_g, k, &cover));
                    let (log, attempts) = descend(&image, &divs, logs, &points, &cover, &f, a);
                    let recovered = log.map(|v| v * inv % 11);
                    if let Some(d) = recovered {
                        assert_eq!(e.mul(&f, g, d), q);
                    }
                    recovery.push(json!({"fixture_scalar":k,"point":q,"pullback":div_json(&image),"recovered_scalar":recovered,"verified":recovered.is_some(),"attempts":attempts}));
                }
                log_records.push(json!({"base_transfer_gain":gain,"base_descent_attempts":base_attempts,"source_generator":g}));
            } else {
                log_records.push(json!({"base_transfer_gain":null,"base_descent_attempts":base_attempts,"source_generator":g,"failure":"base descent exhausted frozen bound"}));
            }
        }
        let done = recovery.iter().filter(|r| r["verified"] == true).count();
        println!(
            "N5 a={a}: {} points, {} relations, rank {rank}; {done}/11 recovered",
            points.len(),
            rows.len()
        );
        records.push(json!({"a":a,"trace":e.trace,"field_modulus":f.poly,"curve":"v^2+u^2*v=u^7+a*u^4+u","field_degree":5,"genus":3,"cover_degree":3,"curve_point_counts_k5_10_15":counts,"jacobian_order_over_F32":jac_order,"eleven_cofactor":cofactor,"augmentation":augmentation,"factor_base":points,"polynomials_tested":(1u32<<20)*if quartic{2}else{1},"hermite_interpolation":interpolation,"relations":functions,"relation_matrix_mod11":rows,"rank":rank,"columns":points.len(),"nullity":points.len()-rank,"factor_base_logs":log_records,"target_recovery":recovery,"recovered":done,"scope":"same-field genus3 relation control over F32; not an F2 carrier or modular-Jacobian map"}));
    }
    round02::save(
        dir,
        if hermite {
            "n5d-geometric-relations.json"
        } else if quartic {
            "n5c-geometric-relations.json"
        } else if vertical {
            "n5b-geometric-relations.json"
        } else {
            "n5-geometric-relations.json"
        },
        &json!({"schema":"koblitz.round03/n5-v1","records":records,"fixture_labels_used_in_relations":false,"source_map":"explicit cubic pullback plus augmentation embedding","modular_jacobian_evaluator":false}),
    );
}

fn hermite_quartics(f: &SmallField, points: &[(u16, u16)], a: u16) -> (Vec<Poly>, usize) {
    use std::collections::BTreeSet;
    let mut polynomials = BTreeSet::new();
    let mut divisors = 0;
    fn visit(
        f: &SmallField,
        points: &[(u16, u16)],
        a: u16,
        indices: &mut Vec<usize>,
        start: usize,
        polys: &mut BTreeSet<u32>,
        divisors: &mut usize,
    ) {
        if indices.len() < 5 {
            for i in start..points.len() {
                if indices
                    .iter()
                    .any(|j| points[*j].0 == points[i].0 && (*j != i || points[i].0 == 0))
                {
                    continue;
                }
                indices.push(i);
                visit(f, points, a, indices, i, polys, divisors);
                indices.pop();
            }
            return;
        }
        *divisors += 1;
        let mut rows = Vec::new();
        for (position, index) in indices.iter().enumerate() {
            if position > 0 && indices[position - 1] == *index {
                continue;
            }
            let m = indices.iter().filter(|i| **i == *index).count();
            let (x, y) = points[*index];
            let mut ys = vec![y];
            for n in 1..m {
                assert_ne!(x, 0);
                let fnth = if n <= 7 { f.pow(x, (7 - n) as u64) } else { 0 }
                    ^ if n == 1 { 1 } else { 0 }
                    ^ if n == 4 { a } else { 0 };
                let sq = if n % 2 == 0 { f.sqr(ys[n / 2]) } else { 0 };
                let shifted = if n >= 2 { ys[n - 2] } else { 0 };
                ys.push(f.mul(fnth ^ sq ^ shifted, f.inv(f.sqr(x))));
            }
            for n in 0..m {
                let mut row: Vec<_> = (0..5)
                    .map(|j| {
                        if n <= j && n & j == n {
                            f.pow(x, (j - n) as u64)
                        } else {
                            0
                        }
                    })
                    .collect();
                row.push(ys[n]);
                rows.push(row);
            }
        }
        assert_eq!(rows.len(), 5);
        for c in 0..5 {
            let pivot = (c..5).find(|i| rows[*i][c] != 0).unwrap();
            rows.swap(c, pivot);
            let inv = f.inv(rows[c][c]);
            for j in c..6 {
                rows[c][j] = f.mul(rows[c][j], inv);
            }
            for i in 0..5 {
                if i == c {
                    continue;
                }
                let factor = rows[i][c];
                for j in c..6 {
                    rows[i][j] ^= f.mul(factor, rows[c][j]);
                }
            }
        }
        let v: Vec<_> = (0..5).map(|i| rows[i][5]).collect();
        let code = v
            .iter()
            .enumerate()
            .fold(0u32, |mask, (i, c)| mask | (*c as u32) << (5 * i));
        polys.insert(code);
    }
    visit(
        f,
        points,
        a,
        &mut Vec::new(),
        0,
        &mut polynomials,
        &mut divisors,
    );
    (
        polynomials
            .into_iter()
            .map(|code| (0..5).map(|i| ((code >> (5 * i)) & 31) as u16).collect())
            .collect(),
        divisors,
    )
}

pub fn verify(path: &Path) {
    let data: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    let f = SmallField::new(5);
    let irr = IrreduciblePoly {
        degree: 5,
        low_terms: (0..5).filter(|i| f.poly >> i & 1 == 1).collect(),
    };
    let mut relation_checks = 0;
    let mut class_checks = 0;
    let mut targets = 0;
    let mut ranks = Vec::new();
    for record in data["records"].as_array().unwrap() {
        let a = record["a"].as_u64().unwrap() as u16;
        let cover = OrdinaryBinaryCover::new(5, irr.clone(), element(a), element(1)).unwrap();
        let curve = cover.source_curve();
        let points: Vec<(u16, u16)> =
            serde_json::from_value(record["factor_base"].clone()).unwrap();
        let divs: Vec<_> = points
            .iter()
            .map(|(x, y)| {
                MumfordDivisor::try_from_point(curve, &element(*x), &element(*y)).unwrap()
            })
            .collect();
        let rows: Vec<Vec<u64>> =
            serde_json::from_value(record["relation_matrix_mod11"].clone()).unwrap();
        assert_eq!(rows.len(), record["relations"].as_array().unwrap().len());
        for (stored, source) in rows.iter().zip(record["relations"].as_array().unwrap()) {
            let row: Vec<u64> = if source.get("v").is_some() {
                let v: Poly = serde_json::from_value(source["v"].clone()).unwrap();
                split(&f, &norm(&f, &v, a), &v, &points).unwrap()
            } else {
                assert_eq!(source["function"], "u-c");
                let x = source["c"].as_u64().unwrap() as u16;
                points
                    .iter()
                    .map(|p| {
                        if p.0 != x {
                            0
                        } else if x == 0 {
                            2
                        } else {
                            1
                        }
                    })
                    .collect()
            };
            assert_eq!(&row, stored);
            assert!(row_class(&row, &divs, &cover).is_identity());
            relation_checks += 1;
        }
        let big = rows
            .iter()
            .map(|r| r.iter().map(|v| num_bigint::BigInt::from(*v)).collect())
            .collect();
        let rank = round02::rank(big, &num_bigint::BigInt::from(11));
        assert_eq!(rank + 1, points.len());
        ranks.push(rank);
        let log_records = record["factor_base_logs"].as_array().unwrap();
        let logs: Vec<_> = log_records[..points.len()]
            .iter()
            .map(|r| r["log"].as_u64().unwrap())
            .collect();
        let anchor = logs.iter().position(|v| *v == 1).unwrap();
        let cofactor = record["eleven_cofactor"].as_u64().unwrap();
        let base = scalar(&divs[anchor], cofactor, &cover);
        assert!(!base.is_identity());
        assert!(scalar(&base, 11, &cover).is_identity());
        for (i, d) in divs.iter().enumerate() {
            assert_eq!(scalar(d, cofactor, &cover), scalar(&base, logs[i], &cover));
            class_checks += 1;
        }
        let e = Elliptic::models()[a as usize];
        let last = log_records.last().unwrap();
        let g: Point = serde_json::from_value(last["source_generator"].clone()).unwrap();
        let gain = last["base_transfer_gain"].as_u64().unwrap();
        let gain_inv = (1..11).find(|v| gain * v % 11 == 1).unwrap();
        for target in record["target_recovery"].as_array().unwrap() {
            let q: Point = serde_json::from_value(target["point"].clone()).unwrap();
            let d = cover.pullback_point_class(&elliptic_point(q)).unwrap();
            assert_eq!(target["pullback"], div_json(&d));
            let success = target["attempts"].as_array().unwrap().last().unwrap();
            let offset = success["offset"].as_u64().unwrap() as usize;
            let shifted = if offset == 0 {
                d.clone()
            } else {
                d.add(&divs[offset - 1], curve)
            };
            let log = if success["identity_after_offset"] == true {
                assert!(shifted.is_identity());
                if offset == 0 {
                    0
                } else {
                    (11 - logs[offset - 1]) % 11
                }
            } else {
                let v: Poly = serde_json::from_value(success["function_v"].clone()).unwrap();
                let (residual, r) = divrem(&f, &norm(&f, &v, a), &small(&shifted.u));
                assert!(r.is_empty());
                // The interpolated polynomial must take the divisor's y-values.
                assert!(divrem(&f, &add(&v, &small(&shifted.v)), &small(&shifted.u))
                    .1
                    .is_empty());
                let row = split(&f, &residual, &v, &points).unwrap();
                assert!(shifted
                    .add(&row_class(&row, &divs, &cover), curve)
                    .is_identity());
                (22 - row.iter().zip(&logs).map(|(a, b)| a * b).sum::<u64>() % 11
                    - if offset == 0 { 0 } else { logs[offset - 1] })
                    % 11
            };
            let recovered = log * gain_inv % 11;
            assert_eq!(target["recovered_scalar"], recovered);
            assert_eq!(e.mul(&f, g, recovered), q);
            targets += 1;
        }
    }
    assert_eq!(targets, 22);
    println!(
        "{}",
        json!({"status":"PASS","principal_relation_checks":relation_checks,"independent_bigint_ranks":ranks,"projected_factor_base_class_checks":class_checks,"target_descent_and_multiplication_checks":targets,"uses_target_labels_in_relations":false,"scope":"same-field F32 controls, no modular evaluator"})
    );
}
