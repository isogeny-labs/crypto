//! Explicit modular base coordinates and transported 11-isogenies.
use crate::{round02, round04::*};
use serde_json::json;
use std::path::Path;
fn a(f: Field, t: u32) -> u32 {
    f.pow(t, 8) ^ f.pow(t, 4) ^ 1
}
fn j_pair(f: Field, t: u32) -> Option<(u32, u32)> {
    if t == 0 {
        return None;
    }
    Some((f.mul(a(f, t), f.inv(f.pow(t, 7))), f.mul(a(f, t), f.inv(t))))
}
fn eta(f: Field, p: Pt) -> Option<(u32, u32)> {
    let Pt::A(x, y) = p else {
        return None;
    };
    Some((x, f.mul(f.sq(x) ^ x ^ 1, y) ^ x ^ 1))
}
pub(crate) fn triple(f: Field, p: Pt) -> Option<[u32; 3]> {
    let (x, y) = eta(f, p)?;
    if y == 0 {
        return None;
    }
    let t = f.mul(f.pow(x, 4), f.inv(y));
    let (j0, j7) = j_pair(f, t)?;
    let (j7b, j49) = j_pair(f, y)?;
    assert_eq!(j7, j7b);
    Some([j0, j7, j49])
}
fn phi(dir: &Path) -> Vec<(u32, u32)> {
    let text = std::fs::read_to_string(dir.join("phi_j_11.txt")).unwrap();
    let mut out = vec![];
    let mut rows = 0;
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let (end, coeff) = line.split_once(']').unwrap();
        let nums: Vec<u32> = end
            .trim_start_matches('[')
            .split(',')
            .map(|v| v.trim().parse().unwrap())
            .collect();
        let c = coeff.trim().as_bytes();
        assert!(c.iter().all(|c| c.is_ascii_digit() || *c == b'-'));
        if (c.last().unwrap() - b'0') % 2 == 1 {
            out.push((nums[0], nums[1]));
            if nums[0] != nums[1] {
                out.push((nums[1], nums[0]));
            }
        }
        rows += 1;
    }
    assert_eq!(rows, 79);
    out
}
fn phi_eval(f: Field, terms: &[(u32, u32)], x: u32, y: u32) -> u32 {
    terms.iter().fold(0, |s, (i, j)| {
        s ^ f.mul(f.pow(x, *i as u64), f.pow(y, *j as u64))
    })
}
pub fn run(dir: &Path) {
    let terms = phi(dir);
    let mut records = vec![];
    for m in 1..=10 {
        let f = Field::find(m);
        let c = Curve { f, a: 1, b: 1 };
        let points = c.points();
        assert_eq!(points.len() as u64, order(1, m));
        let mut rows = vec![];
        let mut involutions = 0;
        for p in &points {
            if let Some((x, y)) = eta(f, *p) {
                let a = f.sq(x) ^ x ^ 1;
                let b = (0..=6).fold(0, |v, i| v ^ f.pow(x, i));
                assert_eq!(f.sq(y) ^ f.mul(f.mul(x, y), a) ^ f.mul(x, b), 0);
                if x != 0 {
                    let w = c.add(Pt::A(0, 1), c.neg(*p));
                    if let Some((u, v)) = eta(f, w) {
                        assert_eq!(u, f.inv(x));
                        assert_eq!(v, f.mul(y, f.inv(f.pow(x, 4))));
                        involutions += 1;
                    }
                }
            }
            rows.push(json!({"point":p.json(),"eta":eta(f,*p),"j_triple":triple(f,*p)}));
        }
        let finite: Vec<_> = rows
            .iter()
            .filter_map(|r| serde_json::from_value::<[u32; 3]>(r["j_triple"].clone()).ok())
            .collect();
        let mut pairs = 0;
        let mut simultaneous = 0;
        for p in &finite {
            for q in &finite {
                if phi_eval(f, &terms, p[0], q[0]) == 0 {
                    pairs += 1;
                    if phi_eval(f, &terms, p[1], q[1]) == 0 && phi_eval(f, &terms, p[2], q[2]) == 0
                    {
                        simultaneous += 1;
                    }
                }
            }
        }
        println!("M1/M2 F2^{m}: {} points, {} finite triples, {pairs} j-neighbor pairs, {simultaneous} triple-compatible pairs",points.len(),finite.len());
        records.push(json!({"degree":m,"modulus":f.modulus,"points":points.len(),"finite_triples":finite.len(),"cusps":points.len()-finite.len(),"involution_checks":involutions,"j_neighbor_pairs":pairs,"triple_compatible_pairs":simultaneous,"records":rows}));
    }
    round02::save(
        dir,
        "m1-modular-base.json",
        &json!({"schema":"koblitz.round04/m1-v1","records":records,"mod2_phi11_terms":terms,"source":"Kilford--McMurdy arXiv1110.6801 equations1,3,5,6; Sutherland classical phi_j_11.txt","scope":"actual modular base coordinates; j-neighbor data alone do not transport level49 structure or evaluate the twisted factor"}),
    );
}
fn power(f: Field, p: &[u32], n: usize) -> Poly {
    (0..n).fold(vec![1], |v, _| mul(f, &v, p))
}
fn division(f: Field, b: u32, n: usize) -> Poly {
    let mut ds = vec![vec![]; n + 1];
    ds[1] = vec![1];
    ds[2] = vec![0, 1];
    ds[3] = vec![b, 0, 0, 1, 1];
    ds[4] = vec![0, 0, b, 0, 0, 0, 1];
    for i in 5..=n {
        let m = i / 2;
        ds[i] = if i % 2 == 1 {
            add(
                &mul(f, &ds[m + 2], &power(f, &ds[m], 3)),
                &mul(f, &ds[m - 1], &power(f, &ds[m + 1], 3)),
            )
        } else {
            let z = mul(
                f,
                &ds[m],
                &add(
                    &mul(f, &ds[m + 2], &power(f, &ds[m - 1], 2)),
                    &mul(f, &ds[m - 2], &power(f, &ds[m + 1], 2)),
                ),
            );
            let (q, r) = div(f, &z, &[0, 1]);
            assert!(r.is_empty());
            q
        };
    }
    ds[n].clone()
}
fn velu_x(f: Field, k: &[u32], modulus: &[u32]) -> Poly {
    let z = modmul(f, &derivative(k), &invmod(f, k, modulus), modulus);
    let x = vec![0, 1];
    add(
        &add(&x, &modmul(f, &x, &z, modulus)),
        &modmul(f, &[0, 0, 1], &modmul(f, &z, &z, modulus), modulus),
    )
}
fn norm_poly(f: Field, r: &[u32], k: &[u32]) -> Poly {
    let n = k.len() - 1;
    let mut matrix = vec![vec![0; n]; n];
    let mut pow = vec![1];
    for j in 0..n {
        let v = modmul(f, r, &pow, k);
        for (i, x) in v.iter().enumerate() {
            matrix[i][j] = *x;
        }
        pow = modmul(f, &pow, &[0, 1], k);
    }
    fn rec(f: Field, m: &Vec<Vec<u32>>, i: usize, used: u32, p: Poly, result: &mut Poly) {
        let n = m.len();
        if i == n {
            *result = add(result, &p);
            return;
        }
        for j in 0..n {
            if used & (1 << j) != 0 {
                continue;
            }
            let q = if i == j {
                vec![m[i][j], 1]
            } else {
                vec![m[i][j]]
            };
            if q.iter().all(|v| *v == 0) {
                continue;
            }
            rec(f, m, i + 1, used | (1 << j), mul(f, &p, &q), result);
        }
    }
    let mut out = vec![];
    rec(f, &matrix, 0, 0, vec![1], &mut out);
    assert_eq!(out.len(), n + 1);
    out
}
fn quotient_b(f: Field, b: u32, k: &[u32]) -> u32 {
    let t = k[k.len() - 2];
    b ^ t ^ f.sq(t)
}
pub(crate) fn kernels7(f: Field, b: u32, target: u32) -> Vec<Poly> {
    let delta = b ^ f.inv(target);
    let Some(t) = f.quadratic(delta) else {
        return vec![];
    };
    let d = division(f, b, 7);
    let mut out = vec![];
    for t in [t, t ^ 1] {
        for c in 0..1 << f.m {
            for v in 1..1 << f.m {
                let k = vec![v, c, t, 1];
                if !div(f, &d, &k).1.is_empty() {
                    continue;
                }
                if gcd(f, &k, &derivative(&k)).len() != 1 {
                    continue;
                }
                let x2 = add(&[0, 0, 1], &scale(f, &invmod(f, &[0, 0, 1], &k), b));
                if composemod(f, &k, &x2, &k).is_empty() {
                    out.push(k);
                }
            }
        }
    }
    out
}
fn reconstruct(f: Field, j: [u32; 3]) -> Result<Pt, String> {
    let parameter = |a: u32, b: u32| -> Result<u32, String> {
        let mut p = vec![1, 0, 0, 0, 1, 0, 0, a, 1];
        p = trim(p);
        let q = vec![1, b, 0, 0, 1, 0, 0, 0, 1];
        let g = gcd(f, &p, &q);
        if g.len() != 2 {
            return Err(format!(
                "7-edge parameter gcd degree {}",
                g.len().saturating_sub(1)
            ));
        }
        Ok(f.mul(g[0], f.inv(g[1])))
    };
    let (t, y) = (parameter(j[0], j[1])?, parameter(j[1], j[2])?);
    let x = f.frob(f.mul(t, y), f.m - 2);
    let d = f.sq(x) ^ x ^ 1;
    if d == 0 {
        return Err("exceptional eta denominator".into());
    }
    let yy = f.mul(y ^ x ^ 1, f.inv(d));
    let p = Pt::A(x, yy);
    if !(Curve { f, a: 1, b: 1 }).valid(p) {
        return Err("reconstructed point not on X0(49)".into());
    }
    if triple(f, p) != Some(j) {
        return Err("triple reconstruction mismatch".into());
    }
    Ok(p)
}
fn embed_point(map: &[u32], p: Pt) -> Pt {
    match p {
        Pt::O => p,
        Pt::A(x, y) => Pt::A(map[x as usize], map[y as usize]),
    }
}
fn subgroup(c: Curve, p: Pt, n: u64) -> Vec<Pt> {
    (0..n).map(|i| c.times(p, i)).collect()
}
pub fn transport(dir: &Path) {
    transport_inner(dir, false);
}
pub fn transport_paired(dir: &Path) {
    transport_inner(dir, true);
}
fn transport_inner(dir: &Path, paired: bool) {
    let small = Field::find(6);
    let f = Field::find(30);
    let map = f.embed(small);
    let base = Curve {
        f: small,
        a: 1,
        b: 1,
    };
    let modular = Curve { f, a: 1, b: 1 };
    let terms = phi(dir);
    let mut ap = 11i64;
    for x in 0..11i64 {
        for y in 0..11i64 {
            if (y * y + x * y - (x * x * x - x * x - 2 * x - 1)).rem_euclid(11) == 0 {
                ap -= 1;
            }
        }
    }
    let mut records = vec![];
    for source in base.points() {
        let Some(js) = triple(small, source) else {
            continue;
        };
        if js.iter().any(|v| *v == 0) {
            continue;
        }
        let mut original = Curve {
            f: small,
            a: 0,
            b: small.inv(js[0]),
        };
        let n = original.points().len();
        if n != 66 && n != 64 {
            records.push(json!({"source":source.json(),"status":"unsupported","reason":"underlying trace not +/-1","order":n}));
            continue;
        }
        if n != 66 {
            original.a = (1..64).find(|v| small.trace(*v) == 1).unwrap();
            assert_eq!(original.points().len(), 66);
        }
        let k1 = kernels7(small, original.b, js[1]);
        let b7 = small.inv(js[1]);
        let k2 = kernels7(small, b7, js[2]);
        if k1.len() != 1 || k2.len() != 1 {
            records.push(json!({"source":source.json(),"status":"unsupported","reason":"ambiguous rational 7-kernel","counts":[k1.len(),k2.len()]}));
            continue;
        }
        let k1: Poly = k1[0].iter().map(|v| map[*v as usize]).collect();
        let k2: Poly = k2[0].iter().map(|v| map[*v as usize]).collect();
        let c = Curve {
            f,
            a: map[original.a as usize],
            b: map[original.b as usize],
        };
        let p = original
            .points()
            .into_iter()
            .map(|p| original.times(p, 6))
            .find(|p| *p != Pt::O)
            .unwrap();
        let p = embed_point(&map, p);
        assert_eq!(c.times(p, 11), Pt::O);
        // Trace over F64 is -1; use the extension-degree-five recurrence.
        let (mut s0, mut s1) = (2i128, -1i128);
        for _ in 2..=5 {
            let v = -s1 - 64 * s0;
            s0 = s1;
            s1 = v;
        }
        let order = ((1i128 << 30) + 1 - s1) as u64;
        let mut cofactor = order;
        while cofactor % 11 == 0 {
            cofactor /= 11;
        }
        let mut q = None;
        for x in 1..4096 {
            let Some(z) = c.point(x) else {
                continue;
            };
            let mut z = c.times(z, cofactor);
            if z == Pt::O {
                continue;
            }
            while c.times(z, 11) != Pt::O {
                z = c.times(z, 11);
            }
            let z = c.times(c.add(c.frob(z, 6), c.neg(z)), 7);
            if z != Pt::O {
                assert_eq!(c.frob(z, 6), c.times(z, 9));
                assert!(!subgroup(c, p, 11).contains(&z));
                q = Some(z);
                break;
            }
        }
        let q = q.expect("full11 torsion search budget");
        let zeta = (2..4096)
            .map(|a| f.pow(a, ((1u64 << 30) - 1) / 11))
            .find(|v| *v != 1)
            .unwrap();
        let (q, original_pairing) = if paired {
            paired_basis(c, p, q, zeta)
        } else {
            (q, 0)
        };
        let mut kernels = vec![];
        for a in 0..=11 {
            let r = if a == 11 { q } else { c.add(p, c.times(q, a)) };
            let mut k = vec![1];
            for i in 1..=5 {
                let Pt::A(x, _) = c.times(r, i) else {
                    panic!("torsion kernel contains origin")
                };
                k = mul(f, &k, &[x, 1]);
            }
            assert!(div(f, &division(f, c.b, 11), &k).1.is_empty());
            kernels.push(k);
        }
        let mut targets = vec![];
        let mut failure = None;
        for k in &kernels {
            let b0 = quotient_b(f, c.b, k);
            let transported7 = norm_poly(f, &velu_x(f, k, &k1), &k1);
            let transported11 = norm_poly(f, &velu_x(f, &k1, k), k);
            let b70 = quotient_b(f, map[b7 as usize], &transported11);
            let b71 = quotient_b(f, b0, &transported7);
            assert_eq!(b70, b71, "commuting square");
            let transported_second = norm_poly(f, &velu_x(f, &transported11, &k2), &k2);
            let b49 = quotient_b(f, b70, &transported_second);
            let j = [f.inv(b0), f.inv(b70), f.inv(b49)];
            assert_eq!(phi_eval(f, &terms, map[js[0] as usize], j[0]), 0);
            match reconstruct(f, j) {
                Ok(point) => {
                    targets.push(point);
                }
                Err(e) => {
                    failure = Some(e);
                    break;
                }
            }
        }
        if let Some(reason) = failure {
            records.push(json!({"source":source.json(),"status":"unsupported","reason":reason,"targets_before_failure":targets.len()}));
            continue;
        }
        assert_eq!(targets.len(), 12);
        let total = targets.iter().fold(Pt::O, |v, p| modular.add(v, *p));
        let embedded = embed_point(&map, source);
        let expected = if ap >= 0 {
            modular.times(embedded, ap as u64)
        } else {
            modular.neg(modular.times(embedded, (-ap) as u64))
        };
        assert_eq!(total, expected, "Hecke T11 sum");
        let mut outputs = vec![];
        for sign in [1i64, -1] {
            let mut ds = vec![];
            for i in 0..5 {
                let a = 2u64.pow(i as u32) % 11;
                let p = targets[a as usize];
                let q = targets[(11 - a) as usize];
                let mut d = modular.add(p, if sign == 1 { q } else { modular.neg(q) });
                if sign == -1 && i % 2 == 1 {
                    d = modular.neg(d);
                }
                ds.push(d);
            }
            let total = ds.iter().fold(Pt::O, |v, p| modular.add(v, *p));
            for d in &mut ds {
                *d = modular.add(modular.times(*d, 5), modular.neg(total));
            }
            for i in 0..5 {
                assert_eq!(modular.frob(ds[i], 6), ds[(i + 1) % 5]);
            }
            let image: Vec<_> = (0..5)
                .map(|i| {
                    (0..6).fold(Pt::O, |v, j| {
                        let mut p = modular.frob(ds[(i + 5 - j % 5) % 5], j as u32);
                        if sign == -1 && j % 2 == 1 {
                            p = modular.neg(p);
                        }
                        modular.add(v, p)
                    })
                })
                .collect();
            for i in 0..5 {
                let mut p = modular.frob(image[(i + 4) % 5], 1);
                if sign == -1 {
                    p = modular.neg(p);
                }
                assert_eq!(p, image[i]);
            }
            assert_eq!(image.iter().fold(Pt::O, |v, p| modular.add(v, *p)), Pt::O);
            assert_eq!(modular.times(image[0], 11), Pt::O);
            let descended = if sign == 1 {
                image[0]
            } else {
                let s = f.quadratic(1).unwrap();
                match image[0] {
                    Pt::O => Pt::O,
                    Pt::A(x, y) => Pt::A(x, y ^ f.mul(s, x)),
                }
            };
            let target = Curve {
                f,
                a: if sign == 1 { 1 } else { 0 },
                b: 1,
            };
            assert!(target.valid(descended));
            assert_eq!(target.frob(descended, 5), descended);
            outputs.push(json!({"target_a":if sign==1{1}else{0},"twisted_trace_tuple":image.iter().map(|p|p.json()).collect::<Vec<_>>(),"descended_F32_point":descended.json(),"nonzero":descended!=Pt::O}));
        }
        println!(
            "M3 source {source:?}: 12 actual Hecke targets, T11={ap}, projected images {:?}",
            outputs
                .iter()
                .map(|v| v["nonzero"].clone())
                .collect::<Vec<_>>()
        );
        records.push(json!({"source":source.json(),"status":"PASS","underlying_a":original.a,"underlying_b":original.b,"basis11":[p.json(),q.json()],"fixed_weil_pairing":if paired{Some(zeta)}else{None},"original_weil_pairing":original_pairing,"kernels11":kernels,"targets":targets.iter().map(|p|p.json()).collect::<Vec<_>>(),"hecke_ap":ap,"projections":outputs}));
    }
    if !paired {
        for row in &mut records {
            if let Some(obj) = row.as_object_mut() {
                obj.remove("fixed_weil_pairing");
                obj.remove("original_weil_pairing");
            }
        }
    }
    round02::save(
        dir,
        if paired {
            "m3b-paired-transport.json"
        } else {
            "m3-modular-transport.json"
        },
        &json!({"schema":"koblitz.round04/m3-v1","source_field":{"degree":6,"modulus":small.modulus},"evaluation_field":{"degree":30,"modulus":f.modulus},"embedding_F64":map,"a11":ap,"records":records,"scope":"explicit level49 transport and candidate twisting-correspondence images; reverse divisor transfer, optimal saturation and relation generation require separate certificates"}),
    );
}

fn miller(c: Curve, p: Pt, r: Pt) -> Option<u32> {
    fn line(c: Curve, p: Pt, q: Pt, r: Pt) -> Option<u32> {
        let (Pt::A(x, y), Pt::A(u, v), Pt::A(rx, ry)) = (p, q, r) else {
            return None;
        };
        if x == u && y != v {
            return Some(rx ^ x).filter(|v| *v != 0);
        }
        if x == 0 && p == q {
            return Some(rx).filter(|v| *v != 0);
        }
        let f = c.f;
        let slope = if p == q {
            x ^ f.mul(y, f.inv(x))
        } else {
            f.mul(y ^ v, f.inv(x ^ u))
        };
        let value = ry ^ f.mul(slope, rx ^ x) ^ y;
        if value == 0 {
            return None;
        }
        let sum = c.add(p, q);
        match sum {
            Pt::O => Some(value),
            Pt::A(s, _) => {
                if rx == s {
                    None
                } else {
                    Some(f.mul(value, f.inv(rx ^ s)))
                }
            }
        }
    }
    let mut v = p;
    let mut value = 1;
    for bit in [0, 1, 1] {
        value = c.f.mul(c.f.sq(value), line(c, v, v, r)?);
        v = c.add(v, v);
        if bit == 1 {
            value = c.f.mul(value, line(c, v, p, r)?);
            v = c.add(v, p);
        }
    }
    assert_eq!(v, Pt::O);
    Some(value)
}
fn weil(c: Curve, p: Pt, q: Pt) -> u32 {
    for x in 1..1024 {
        let Some(s) = c.point(x) else {
            continue;
        };
        let ns = c.neg(s);
        let a = c.add(q, s);
        let b = c.add(p, ns);
        if let (Some(a), Some(b), Some(d), Some(e)) = (
            miller(c, p, a),
            miller(c, p, s),
            miller(c, q, ns),
            miller(c, q, b),
        ) {
            let v = c.f.mul(c.f.mul(a, d), c.f.inv(c.f.mul(b, e)));
            assert_eq!(c.f.pow(v, 11), 1);
            return v;
        }
    }
    panic!("Miller auxiliary budget")
}
pub(crate) fn paired_basis(c: Curve, p: Pt, q: Pt, zeta: u32) -> (Pt, u32) {
    let value = weil(c, p, q);
    assert_ne!(value, 1);
    assert_eq!(weil(c, q, p), c.f.inv(value));
    assert_eq!(weil(c, p, p), 1);
    assert_eq!(weil(c, c.times(p, 2), c.times(q, 3)), c.f.pow(value, 6));
    let exponent = (1..11).find(|i| c.f.pow(zeta, *i) == value).unwrap();
    let inv = (1..11).find(|i| i * exponent % 11 == 1).unwrap();
    let q = c.times(q, inv);
    assert_eq!(weil(c, p, q), zeta);
    (q, value)
}

fn parse_point(v: &serde_json::Value) -> Pt {
    if v.is_array() {
        Pt::A(v[0].as_u64().unwrap() as u32, v[1].as_u64().unwrap() as u32)
    } else {
        assert_eq!(v["infinity"], true);
        Pt::O
    }
}
fn canon(g: [u64; 4]) -> [u64; 4] {
    g.min(g.map(|v| (11 - v) % 11))
}
fn coefficients(c: Curve, p: Pt, q: Pt) -> std::collections::BTreeMap<Pt, (u64, u64)> {
    let mut map = std::collections::BTreeMap::new();
    for a in 0..11 {
        for b in 0..11 {
            assert!(map
                .insert(c.add(c.times(p, a), c.times(q, b)), (a, b))
                .is_none());
        }
    }
    map
}
fn tuple(c: Curve, targets: &[Pt], g: [u64; 4], sign: i64) -> Vec<Pt> {
    let image = |a: u64| {
        let den = (g[0] + a * g[2]) % 11;
        let num = (g[1] + a * g[3]) % 11;
        let i = if den == 0 {
            11
        } else {
            num * (1..11).find(|v| v * den % 11 == 1).unwrap() % 11
        };
        targets[i as usize]
    };
    let mut ds = vec![];
    for i in 0..5 {
        let a = 2u64.pow(i) % 11;
        let p = image(a);
        let q = image(11 - a);
        let mut d = c.add(p, if sign == 1 { q } else { c.neg(q) });
        if sign == -1 && i % 2 == 1 {
            d = c.neg(d);
        }
        ds.push(d);
    }
    let total = ds.iter().fold(Pt::O, |v, p| c.add(v, *p));
    ds.into_iter()
        .map(|d| c.add(c.times(d, 5), c.neg(total)))
        .collect()
}
pub fn relations(dir: &Path) {
    use std::collections::{BTreeMap, BTreeSet};
    let input = read(&dir.join("m3b-paired-transport.json"));
    let f = Field {
        m: 30,
        modulus: input["evaluation_field"]["modulus"].as_u64().unwrap(),
    };
    let s = Field {
        m: 6,
        modulus: input["source_field"]["modulus"].as_u64().unwrap(),
    };
    let embedding: Vec<u32> = serde_json::from_value(input["embedding_F64"].clone()).unwrap();
    let ec = Curve { f, a: 1, b: 1 };
    let base = Curve { f: s, a: 1, b: 1 };
    let rows: Vec<_> = input["records"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["status"] == "PASS")
        .collect();
    assert_eq!(rows.len(), 24);
    let sources: Vec<_> = rows.iter().map(|r| parse_point(&r["source"])).collect();
    let curves: Vec<_> = rows
        .iter()
        .map(|r| Curve {
            f,
            a: embedding[r["underlying_a"].as_u64().unwrap() as usize],
            b: embedding[r["underlying_b"].as_u64().unwrap() as usize],
        })
        .collect();
    let basis: Vec<_> = rows
        .iter()
        .map(|r| [parse_point(&r["basis11"][0]), parse_point(&r["basis11"][1])])
        .collect();
    let targets: Vec<Vec<Pt>> = rows
        .iter()
        .map(|r| {
            r["targets"]
                .as_array()
                .unwrap()
                .iter()
                .map(parse_point)
                .collect()
        })
        .collect();
    let grids: Vec<_> = (0..rows.len())
        .map(|i| coefficients(curves[i], basis[i][0], basis[i][1]))
        .collect();
    let mut gs = BTreeSet::new();
    for a in 0..11 {
        for b in 0..11 {
            for c in 0..11 {
                for d in 0..11 {
                    if (a * d + 121 - b * c) % 11 == 1 {
                        gs.insert(canon([a, b, c, d]));
                    }
                }
            }
        }
    }
    let gs: Vec<_> = gs.into_iter().collect();
    assert_eq!(gs.len(), 660);
    let gindex: BTreeMap<_, _> = gs.iter().enumerate().map(|(i, g)| (*g, i)).collect();
    let mut succ = vec![0; 24 * 660];
    let mut tuples = vec![vec![]; 2];
    for i in 0..24 {
        let next = sources
            .iter()
            .position(|p| *p == base.frob(sources[i], 1))
            .unwrap();
        let a_i = rows[i]["underlying_a"].as_u64().unwrap() as u32;
        let a_j = rows[next]["underlying_a"].as_u64().unwrap() as u32;
        let shift = embedding[s.quadratic(s.sq(a_i) ^ a_j).unwrap() as usize];
        let twist = |p: Pt| match p {
            Pt::O => p,
            Pt::A(x, y) => Pt::A(x, y ^ f.mul(shift, x)),
        };
        for (k, g) in gs.iter().enumerate() {
            let c = curves[i];
            let p = c.add(c.times(basis[i][0], g[0]), c.times(basis[i][1], g[1]));
            let q = c.add(c.times(basis[i][0], g[2]), c.times(basis[i][1], g[3]));
            let p = twist(c.frob(p, 1));
            let q = curves[next].times(twist(c.frob(q, 1)), 6);
            let (a, b) = grids[next][&p];
            let (c, d) = grids[next][&q];
            assert_eq!((a * d + 121 - b * c) % 11, 1);
            let ng = canon([a, b, c, d]);
            succ[i * 660 + k] = next * 660 + gindex[&ng];
            for (j, sign) in [1, -1].iter().enumerate() {
                tuples[j].push(tuple(ec, &targets[i], *g, *sign));
            }
        }
    }
    assert_eq!(
        succ.iter().copied().collect::<BTreeSet<_>>().len(),
        succ.len()
    );
    for (sign, values) in [1, -1].iter().zip(&tuples) {
        for i in 0..succ.len() {
            for k in 0..5 {
                let mut p = ec.frob(values[i][(k + 4) % 5], 1);
                if *sign == -1 {
                    p = ec.neg(p);
                }
                assert_eq!(p, values[succ[i]][k], "modular Frobenius equivariance");
            }
        }
    }
    let mut seen = BTreeSet::new();
    let mut closed = vec![];
    let mut group_images = vec![vec![]; 2];
    for start in 0..succ.len() {
        if seen.contains(&start) {
            continue;
        }
        let mut orbit = vec![];
        let mut i = start;
        loop {
            assert!(seen.insert(i));
            orbit.push(i);
            i = succ[i];
            if i == start {
                break;
            }
        }
        assert!([6, 30].contains(&orbit.len()));
        let mut images = vec![];
        for j in 0..2 {
            let t: Vec<_> = (0..5)
                .map(|k| orbit.iter().fold(Pt::O, |v, i| ec.add(v, tuples[j][*i][k])))
                .collect();
            for k in 0..5 {
                let mut p = ec.frob(t[(k + 4) % 5], 1);
                if j == 1 {
                    p = ec.neg(p);
                }
                assert_eq!(p, t[k]);
            }
            assert_eq!(t.iter().fold(Pt::O, |v, p| ec.add(v, *p)), Pt::O);
            assert_eq!(ec.times(t[0], 11), Pt::O);
            let z = if j == 0 {
                t[0]
            } else {
                let r = f.quadratic(1).unwrap();
                match t[0] {
                    Pt::O => Pt::O,
                    Pt::A(x, y) => Pt::A(x, y ^ f.mul(r, x)),
                }
            };
            let dest = Curve {
                f,
                a: if j == 0 { 1 } else { 0 },
                b: 1,
            };
            assert!(dest.valid(z));
            assert_eq!(dest.frob(z, 5), z);
            group_images[j].push(z);
            images.push(json!({"a":if j==0{1}else{0},"point":z.json(),"nonzero":z!=Pt::O}));
        }
        closed.push(json!({"degree":orbit.len(),"members":orbit,"source":sources[start/660].json(),"basis_matrix":gs[start%660],"images":images}));
    }
    let cusp = |p: Pt| triple(s, p).is_none();
    let supported = |p: Pt| sources.contains(&p) || cusp(p);
    let mut functions = vec![];
    let mut candidates = 0;
    let mut incomplete = 0;
    let mut admit = |kind: &str, params: Vec<u32>, roots: Vec<(Pt, u64)>| {
        candidates += 1;
        if roots.iter().any(|(p, _)| !supported(*p)) {
            incomplete += 1;
            return;
        }
        let mut multiplicity: BTreeMap<Pt, u64> = BTreeMap::new();
        for (p, m) in &roots {
            for j in 0..6 {
                *multiplicity.entry(base.frob(*p, j)).or_default() += m;
            }
        }
        let row: Vec<u64> = closed
            .iter()
            .map(|col| {
                let p = parse_point(&col["source"]);
                multiplicity.get(&p).copied().unwrap_or(0)
            })
            .collect();
        if row.iter().all(|v| *v == 0) {
            return;
        }
        for j in 0..2 {
            let c = Curve {
                f,
                a: if j == 0 { 1 } else { 0 },
                b: 1,
            };
            let sum = row
                .iter()
                .zip(&group_images[j])
                .fold(Pt::O, |s, (m, p)| c.add(s, c.times(*p, *m)));
            assert_eq!(sum, Pt::O, "full modular principal pullback relation");
        }
        functions.push(json!({"function":kind,"parameters":params,"base_zeros":roots.iter().map(|(p,m)|json!({"point":p.json(),"multiplicity":m})).collect::<Vec<_>>(),"norm_degree":6,"row_mod11":row.iter().map(|v|v%11).collect::<Vec<_>>(),"geometric_level_fiber_degree":660,"cusps_omitted_as_zero_images":"images have order dividing14 and11, hence vanish"}));
    };
    for l in 0..64 {
        for v in 0..64 {
            let mut p = vec![s.sq(v) ^ 1, v, s.sq(l) ^ l ^ 1, 1];
            let mut roots = vec![];
            for x in 0..64 {
                let mut m = 0;
                while s.eval(&p, x) == 0 {
                    let (q, r) = div(s, &p, &[x, 1]);
                    assert!(r.is_empty());
                    p = q;
                    m += 1;
                    if p.len() == 1 {
                        break;
                    }
                }
                if m > 0 {
                    roots.push((Pt::A(x, s.mul(l, x) ^ v), m));
                }
                if p.len() == 1 {
                    break;
                }
            }
            if p.len() == 1 {
                admit("Norm_F64/F2(Y+lX+v)", vec![l, v], roots);
            }
        }
    }
    for x in 0..64 {
        let ps: Vec<_> = base
            .points()
            .into_iter()
            .filter(|p| matches!(p,Pt::A(u,_)if *u==x))
            .collect();
        if ps.len() == 2 {
            admit(
                "Norm_F64/F2(X+x)",
                vec![x],
                ps.iter().map(|p| (*p, 1)).collect(),
            );
        } else if ps.len() == 1 {
            admit("Norm_F64/F2(X+x)", vec![x], vec![(ps[0], 2)]);
        }
    }
    let matrix: Vec<Vec<u64>> = functions
        .iter()
        .map(|r| serde_json::from_value(r["row_mod11"].clone()).unwrap())
        .collect();
    let (_, pivots) = crate::gluing::rref(matrix, 11);
    let degrees: BTreeMap<usize, usize> = closed.iter().fold(BTreeMap::new(), |mut m, c| {
        *m.entry(c["degree"].as_u64().unwrap() as usize).or_default() += 1;
        m
    });
    let nonzero: Vec<_> = group_images
        .iter()
        .map(|v| v.iter().filter(|p| **p != Pt::O).count())
        .collect();
    println!("M4: {} geometric points, {} closed columns {:?}, {} genuine principal rows, rank {}; nonzero images {:?}",succ.len(),closed.len(),degrees,functions.len(),pivots.len(),nonzero);
    round02::save(
        dir,
        "m4-direct-modular-relations.json",
        &json!({"schema":"koblitz.round04/m4-v1","geometric_points":succ.len(),"SL2_mod_sign":gs,"frobenius_successor":succ,"closed_points":closed,"closed_degrees":degrees,"nonzero_images_by_sign":nonzero,"functions_tested_with_split_base_zeros":candidates,"functions_rejected_missing_base_support":incomplete,"principal_relations":functions,"rank_mod11":pivots.len(),"columns":closed.len(),"missing_rank_to_unique_scale":closed.len()-1-pivots.len(),"source_curve":"X_H(5929), H d=+/-1 mod11, conjugated full11 level on X0(49)","curve_genus":2841,"target_group_order":11,"scope":"full level fibers of modular principal functions, with cusp images removed by prime-to11 torsion; no individual descent or subgroup-to-Jacobian injection claimed"}),
    );
}

pub fn interior_test(dir: &Path) {
    let input = read(&dir.join("m3b-paired-transport.json"));
    let f = Field {
        m: 30,
        modulus: input["evaluation_field"]["modulus"].as_u64().unwrap(),
    };
    let s = Field {
        m: 6,
        modulus: input["source_field"]["modulus"].as_u64().unwrap(),
    };
    let embedding: Vec<u32> = serde_json::from_value(input["embedding_F64"].clone()).unwrap();
    let mut ds = vec![vec![]; 12];
    ds[1] = vec![1];
    ds[2] = vec![1];
    ds[3] = vec![0, 1];
    ds[4] = vec![1, 1];
    for i in 5..12 {
        let m = i / 2;
        ds[i] = if i % 2 == 1 {
            add(
                &mul(s, &ds[m + 2], &power(s, &ds[m], 3)),
                &mul(s, &ds[m - 1], &power(s, &ds[m + 1], 3)),
            )
        } else {
            add(
                &mul(s, &ds[m], &mul(s, &ds[m + 2], &power(s, &ds[m - 1], 2))),
                &mul(s, &ds[m], &mul(s, &ds[m - 2], &power(s, &ds[m + 1], 2))),
            )
        };
    }
    let psi = ds[11].clone();
    assert_eq!(psi.len(), 16);
    assert_ne!(psi[0], 0);
    let mut remaining = psi.clone();
    let mut rational_roots = vec![];
    for b in 1..64 {
        let mut n = 0;
        while s.eval(&remaining, b) == 0 {
            let (q, r) = div(s, &remaining, &[b, 1]);
            assert!(r.is_empty());
            remaining = q;
            n += 1;
            if remaining.len() == 1 {
                break;
            }
        }
        if n > 0 {
            rational_roots.push((b, n));
        }
    }
    let points = read(&dir.join("m4-direct-modular-relations.json"));
    let gs: Vec<[u64; 4]> = serde_json::from_value(points["SL2_mod_sign"].clone()).unwrap();
    let mut represented = 0;
    let mut by_base = vec![];
    for row in input["records"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["status"] == "PASS")
    {
        let c = Curve {
            f,
            a: embedding[row["underlying_a"].as_u64().unwrap() as usize],
            b: embedding[row["underlying_b"].as_u64().unwrap() as usize],
        };
        let p = parse_point(&row["basis11"][0]);
        let q = parse_point(&row["basis11"][1]);
        let count = gs
            .iter()
            .filter(|g| matches!(c.add(c.times(p, g[0]), c.times(q, g[1])), Pt::A(1, _)))
            .count();
        represented += count;
        by_base.push(json!({"source":row["source"],"zeros":count}));
    }
    let complete = 15 * 56 * 11;
    assert!(represented < complete);
    println!("M5: x(P)+1 has complete ordinary zero degree{complete}, only{represented} represented; row REJECTED");
    round02::save(
        dir,
        "m5-interior-function.json",
        &json!({"schema":"koblitz.round04/m5-v1","function":"x(P)+1","psi11_at_x1_coefficients_in_b":psi,"rational_b_roots_with_multiplicities":rational_roots,"remaining_b_polynomial":remaining,"remaining_b_degree":remaining.len()-1,"complete_ordinary_zero_degree":complete,"represented_zeros":represented,"missing_ordinary_zero_degree":complete-represented,"known_zeros_by_base":by_base,"relation_admitted":false,"failure":"incomplete zero divisor; pole divisor also not resolved","rank_change":0}),
    );
}
