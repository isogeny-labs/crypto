//! Independent point, polynomial, exact Macaulay and rank replay. No timings.
#![allow(dead_code)]
#[path = "../../../../src/hash/sha256.rs"]
pub mod sha_backend;
pub mod hash {
    pub use crate::sha_backend as sha256;
}
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity_certificate;
use identity_certificate::{Expr as Ex, IdentityCertificate, IdentityStatement, ProofRef, Verdict};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
type I = i64;
type Point = Option<(I, I)>;
type Poly = BTreeMap<(usize, usize), I>;
fn red(x: I, p: I) -> I {
    x.rem_euclid(p)
}
fn pow(mut a: I, mut n: usize, p: I) -> I {
    let mut r = 1;
    while n > 0 {
        if n & 1 == 1 {
            r = red(r * a, p);
        }
        a = red(a * a, p);
        n >>= 1;
    }
    r
}
fn inv(a: I, p: I) -> I {
    let (mut r, mut rr, mut s, mut ss) = (p, red(a, p), 0, 1);
    while rr != 0 {
        let q = r / rr;
        (r, rr) = (rr, r - q * rr);
        (s, ss) = (ss, s - q * ss);
    }
    assert_eq!(r, 1, "noninvertible denominator");
    red(s, p)
}
fn sha(bytes: &[u8]) -> String {
    hex::encode(sha_backend::sha256(bytes))
}
fn n(v: &Value) -> I {
    v.as_i64().unwrap()
}
fn ints(v: &Value) -> Vec<I> {
    v.as_array().unwrap().iter().map(n).collect()
}
fn point_json(q: Point) -> Value {
    match q {
        None => json!([-1, -1]),
        Some((x, y)) => json!([x, y]),
    }
}
#[derive(Clone, Copy)]
struct Curve {
    p: I,
    a: I,
    b: I,
}
impl Curve {
    fn contains(self, q: Point) -> bool {
        q.is_none_or(|(x, y)| red(y * y - x * x * x - self.a * x - self.b, self.p) == 0)
    }
    fn points(self) -> Vec<Point> {
        let mut out = vec![None];
        for x in 0..self.p {
            for y in 0..self.p {
                let q = Some((x, y));
                if self.contains(q) {
                    out.push(q);
                }
            }
        }
        out
    }
    fn neg(self, q: Point) -> Point {
        q.map(|(x, y)| (x, red(-y, self.p)))
    }
    fn add(self, q: Point, r: Point) -> Point {
        let (Some((x, y)), Some((u, v))) = (q, r) else {
            return q.or(r);
        };
        if x == u && red(y + v, self.p) == 0 {
            return None;
        }
        let m = if x == u {
            red((3 * x * x + self.a) * inv(2 * y, self.p), self.p)
        } else {
            red((v - y) * inv(u - x, self.p), self.p)
        };
        let xx = red(m * m - x - u, self.p);
        Some((xx, red(m * (x - xx) - y, self.p)))
    }
    fn mul(self, z: I, q: Point) -> Point {
        // Linear chain deliberately differs from the producer's binary chain.
        let q = if z < 0 { self.neg(q) } else { q };
        let mut out = None;
        for _ in 0..z.abs() {
            out = self.add(out, q);
        }
        out
    }
    fn generator(self, r: I) -> Point {
        self.points()
            .into_iter()
            .skip(1)
            .find(|&q| self.mul(r, q).is_none())
            .unwrap()
    }
    fn xs(self, q: Point, r: I) -> Vec<I> {
        (1..r)
            .map(|i| self.mul(i, q).unwrap().0)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }
    fn reps(self, xs: &[I]) -> Vec<Point> {
        xs.iter()
            .map(|&x| {
                (0..self.p)
                    .map(|y| Some((x, y)))
                    .find(|&q| self.contains(q))
                    .unwrap()
            })
            .collect()
    }
}
fn scale(c: Curve, q: Point, s: I) -> Point {
    q.map(|(x, y)| (red(s * s * x, c.p), red(s * s * s * y, c.p)))
}
fn primes(mut x: I) -> Vec<I> {
    let mut ps = vec![];
    let mut p = 2;
    while p * p <= x {
        if x % p == 0 {
            ps.push(p);
            while x % p == 0 {
                x /= p;
            }
        }
        p += 1;
    }
    if x > 1 {
        ps.push(x);
    }
    ps
}
fn nonsquare(p: I) -> I {
    (1..p)
        .find(|&d| !(0..p).any(|s| red(s * s, p) == d))
        .unwrap()
}
fn gcd(mut a: I, mut b: I) -> I {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
fn subs(xs: &[I], b: usize) -> Vec<Vec<I>> {
    let mut out = vec![];
    for mask in 0..1usize << xs.len() {
        if mask.count_ones() as usize == b {
            out.push(
                xs.iter()
                    .enumerate()
                    .filter(|(i, _)| mask & (1 << i) != 0)
                    .map(|(_, x)| *x)
                    .collect(),
            );
        }
    }
    out.sort();
    out
}
fn add_term(f: &mut Poly, m: (usize, usize), c: I, p: I) {
    let c = red(f.get(&m).copied().unwrap_or(0) + c, p);
    if c == 0 {
        f.remove(&m);
    } else {
        f.insert(m, c);
    }
}
fn s3_poly(c: Curve, z: I) -> Poly {
    // Explicit coefficient table, independent of the producer's convolutions.
    let mut f = Poly::new();
    for (m, v) in [
        ((2, 2), 1),
        ((2, 1), -2 * z),
        ((1, 2), -2 * z),
        ((2, 0), z * z),
        ((1, 1), -2 * z * z - 2 * c.a),
        ((0, 2), z * z),
        ((1, 0), -2 * c.a * z - 4 * c.b),
        ((0, 1), -2 * c.a * z - 4 * c.b),
        ((0, 0), c.a * c.a - 4 * c.b * z),
    ] {
        add_term(&mut f, m, v, c.p);
    }
    f
}
fn s3_value(c: Curve, x: I, y: I, z: I) -> I {
    let t = x * y + x * z + y * z - c.a;
    red(t * t - 4 * (x + y + z) * (x * y * z + c.b), c.p)
}
fn base_poly(xs: &[I], variable: usize, p: I) -> Poly {
    let mut coeff = vec![1];
    for &x in xs {
        let mut next = vec![0; coeff.len() + 1];
        for (i, &a) in coeff.iter().enumerate() {
            next[i] = red(next[i] - x * a, p);
            next[i + 1] = red(next[i + 1] + a, p);
        }
        coeff = next;
    }
    coeff
        .into_iter()
        .enumerate()
        .filter(|(_, a)| *a != 0)
        .map(|(i, a)| (if variable == 0 { (i, 0) } else { (0, i) }, a))
        .collect()
}
fn monomials(d: usize) -> Vec<(usize, usize)> {
    (0..=d)
        .flat_map(|k| (0..=k).map(move |i| (i, k - i)))
        .collect()
}
fn rref(mut rows: Vec<Vec<I>>, cols: usize, p: I) -> (Vec<Vec<I>>, Vec<usize>) {
    // Batch RREF with full elimination and ascending monomial degrees; neither
    // order nor reduction strategy is shared with the incremental C++ basis.
    let mut pivots = vec![];
    for j in 0..cols {
        let k = pivots.len();
        let Some(i) = (k..rows.len()).find(|&i| rows[i][j] != 0) else {
            continue;
        };
        rows.swap(k, i);
        let t = inv(rows[k][j], p);
        for x in &mut rows[k] {
            *x = red(*x * t, p);
        }
        for i in 0..rows.len() {
            if i != k {
                let t = rows[i][j];
                if t != 0 {
                    for l in j..cols {
                        rows[i][l] = red(rows[i][l] - t * rows[k][l], p);
                    }
                }
            }
        }
        pivots.push(j);
    }
    rows.truncate(pivots.len());
    (rows, pivots)
}
fn solve_linear(equations: Vec<Vec<I>>, vars: usize, p: I) -> Vec<I> {
    let (rows, pivots) = rref(equations, vars + 1, p);
    assert!(
        !pivots.contains(&vars),
        "linear certificate system inconsistent"
    );
    let mut solution = vec![0; vars];
    for (row, &j) in rows.iter().zip(&pivots) {
        solution[j] = row[vars];
    }
    solution
}

// The above arithmetic is inherited verbatim from the independently written
// prior Rust auditor. No C++ producer field/group/polynomial code is shared.
fn degree(f: &Poly) -> usize {
    f.keys().map(|(i, j)| i + j).max().unwrap_or(0)
}
fn ev(f: &Poly, x: I, y: I, p: I) -> I {
    f.iter().fold(0, |s, (&(i, j), &c)| {
        red(s + c * pow(x, i, p) * pow(y, j, p), p)
    })
}
fn divide_base(mut f: Poly, xs: &[I], p: I) -> Poly {
    let (s, t) = (red(xs[0] + xs[1], p), red(xs[0] * xs[1], p));
    for variable in 0..2 {
        loop {
            let found = f
                .iter()
                .rev()
                .find(|((i, j), _)| if variable == 0 { *i >= 2 } else { *j >= 2 })
                .map(|(&m, &c)| (m, c));
            let Some(((i, j), c)) = found else { break };
            f.remove(&(i, j));
            let (u, v) = if variable == 0 {
                ((i - 1, j), (i - 2, j))
            } else {
                ((i, j - 1), (i, j - 2))
            };
            add_term(&mut f, u, s * c, p);
            add_term(&mut f, v, -t * c, p);
        }
    }
    f
}
fn poly_json(f: &Poly) -> Value {
    json!(f
        .iter()
        .map(|(&(i, j), &c)| vec![i as I, j as I, c])
        .collect::<Vec<_>>())
}
fn homogeneous(eqs: &[Poly], p: I) -> Value {
    let (mut hs, mut cutoff) = (vec![], Value::Null);
    for d in 0..=8 {
        let ms: Vec<_> = (0..=d).map(|i| (i, d - i)).collect();
        let index: BTreeMap<_, _> = ms.iter().enumerate().map(|(i, &m)| (m, i)).collect();
        let mut rows = vec![];
        for f in eqs.iter().filter(|f| !f.is_empty() && degree(f) <= d) {
            let df = degree(f);
            for sx in 0..=d - df {
                let sy = d - df - sx;
                let mut row = vec![0; ms.len()];
                for (&(i, j), &c) in f {
                    if i + j == df {
                        row[index[&(i + sx, j + sy)]] = c;
                    }
                }
                rows.push(row);
            }
        }
        let h = ms.len() - rref(rows, ms.len(), p).1.len();
        hs.push(h);
        if h == 0 && cutoff.is_null() {
            cutoff = json!(d);
        }
    }
    json!({"hilbert":hs,"cutoff":cutoff})
}
fn affine(eqs: &[Poly], roots: &[(I, I)], p: I, start: usize, cap: usize) -> Value {
    let (mut closure, mut refutation, mut journal) = (Value::Null, Value::Null, vec![]);
    for d in start..=cap {
        let ms = monomials(d);
        let index: BTreeMap<_, _> = ms.iter().enumerate().map(|(i, &m)| (m, i)).collect();
        let mut rows = vec![];
        for f in eqs.iter().filter(|f| !f.is_empty() && degree(f) <= d) {
            for (sx, sy) in monomials(d - degree(f)) {
                let mut row = vec![0; ms.len()];
                for (&(i, j), &c) in f {
                    row[index[&(i + sx, j + sy)]] = c;
                }
                rows.push(row);
            }
        }
        let (basis, pivots) = rref(rows, ms.len(), p);
        let evaluation = roots
            .iter()
            .map(|&(x, y)| {
                ms.iter()
                    .map(|&(i, j)| red(pow(x, i, p) * pow(y, j, p), p))
                    .collect()
            })
            .collect();
        let erank = rref(evaluation, ms.len(), p).1.len();
        let mut one = vec![0; ms.len()];
        one[index[&(0, 0)]] = 1;
        for (row, &j) in basis.iter().zip(&pivots) {
            let t = one[j];
            for i in j..ms.len() {
                one[i] = red(one[i] - t * row[i], p);
            }
        }
        let unit = one.iter().all(|&c| c == 0);
        if unit && refutation.is_null() {
            refutation = json!(d);
        }
        journal.push(json!([d, pivots.len(), ms.len(), erank, usize::from(unit)]));
        if pivots.len() + erank == ms.len() {
            assert_eq!(erank, roots.len());
            closure = json!(d);
            break;
        }
    }
    assert!(roots.is_empty() || refutation.is_null());
    json!({"closure":closure,"refutation":refutation,"journal":journal})
}
fn as_point(v: &Value) -> Point {
    let a = ints(v);
    assert_eq!(a.len(), 2);
    Some((a[0], a[1]))
}
fn fresh_arm(c: Curve, src: &Value, r: I) -> Value {
    let xs = ints(&src["base_x"]);
    assert_eq!(xs.len(), 2);
    assert_ne!(xs[0], xs[1]);
    let generator = as_point(&src["generator"]);
    assert!(
        generator.is_some() && c.contains(generator) && c.mul(r, generator).is_none(),
        "valid subgroup generator"
    );
    let base: Vec<_> = src["base_points"]
        .as_array()
        .unwrap()
        .iter()
        .map(as_point)
        .collect();
    assert_eq!(base.len(), 2);
    for (i, q) in base.iter().enumerate() {
        assert_eq!(q.unwrap().0, xs[i], "base x binding");
    }
    let logs: Vec<_> = base
        .iter()
        .map(|&q| {
            assert!(c.contains(q) && c.mul(r, q).is_none());
            (0..r).find(|&n| c.mul(n, generator) == q).unwrap()
        })
        .collect();
    let gx = base_poly(&xs, 0, c.p);
    let gy = base_poly(&xs, 1, c.p);
    let mut all_rows = vec![];
    let mut targets = vec![];
    for scalar in 1..r {
        let target = c.mul(scalar, generator).unwrap();
        let f = s3_poly(c, target.0);
        let h = divide_base(f.clone(), &xs, c.p);
        let before = rref(all_rows.clone(), 2, r).1.len();
        let mut relations = BTreeSet::new();
        let mut roots = vec![];
        for i in 0..2 {
            for j in 0..2 {
                let mut lift = false;
                for si in [-1, 1] {
                    for sj in [-1, 1] {
                        if c.add(c.mul(si, base[i]), c.mul(sj, base[j])) == Some(target) {
                            lift = true;
                            let mut row = vec![0, 0];
                            row[i] = red(row[i] + si, r);
                            row[j] = red(row[j] + sj, r);
                            assert_eq!(red(row[0] * logs[0] + row[1] * logs[1], r), scalar);
                            relations.insert(row.clone());
                            all_rows.push(row);
                        }
                    }
                }
                assert_eq!(
                    s3_value(c, xs[i], xs[j], target.0) == 0,
                    lift,
                    "actual S3 lift"
                );
                assert_eq!(ev(&h, xs[i], xs[j], c.p) == 0, lift, "reduced lift");
                if lift {
                    roots.push((xs[i], xs[j]));
                }
            }
        }
        let eqs = [h.clone(), gx.clone(), gy.clone()];
        let start = eqs.iter().map(degree).max().unwrap();
        targets.push(json!({"n":scalar,"target":[target.0,target.1],"remainder":poly_json(&h),"remainder_degree":if h.is_empty(){-1}else{degree(&h) as I},
            "rows":relations,"before":before,"after":rref(all_rows.clone(),2,r).1.len(),"root_pairs":roots.len(),
            "homogeneous":homogeneous(&eqs,c.p),"affine":affine(&eqs,&roots,c.p,start,8),"unreduced":affine(&[f,gx.clone(),gy.clone()],&roots,c.p,4,12)}));
    }
    json!({"generator":src["generator"],"base_x":src["base_x"],"base_points":src["base_points"],"targets":targets})
}
fn compare(left: &str, right: &str, l: &Value, r: &Value, out: &mut Vec<Value>) {
    let lt = l["targets"].as_array().unwrap();
    let rt = r["targets"].as_array().unwrap();
    assert_eq!(lt.len(), rt.len());
    let lower = |a: &Value, b: &Value| !a.is_null() && !b.is_null() && n(a) < n(b);
    for (a, b) in lt.iter().zip(rt) {
        let da = n(&a["after"]) - n(&a["before"]);
        let db = n(&b["after"]) - n(&b["before"]);
        let (ha, hb) = (&a["homogeneous"]["cutoff"], &b["homogeneous"]["cutoff"]);
        out.push(json!({"main":left,"twist":right,"n":a["n"],"main_useful":da,"twist_useful":db,"main_cutoff":ha,"twist_cutoff":hb,
            "joint_homogeneous":db>da&&lower(hb,ha),"joint_closure":db>da&&lower(&b["affine"]["closure"],&a["affine"]["closure"])}));
    }
}
fn load(p: &Path) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn write(p: &Path, v: &Value) {
    fs::write(p, format!("{}\n", serde_json::to_string(v).unwrap())).unwrap()
}
fn fresh_audit(input: &Path, result: &Path, receipt: &Path) {
    assert!(!receipt.exists(), "fresh receipt required");
    let mut paths: Vec<_> = fs::read_dir(input)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| {
            p.extension().is_some_and(|x| x == "json") && p.file_name().unwrap() != "index.json"
        })
        .collect();
    paths.sort();
    let actual_cases = fs::read_dir(result)
        .unwrap()
        .filter(|e| {
            e.as_ref()
                .unwrap()
                .path()
                .extension()
                .is_some_and(|x| x == "json")
        })
        .count();
    assert_eq!(actual_cases, 99, "case coverage");
    let (mut cases, mut population, mut fixture, mut pins) =
        (BTreeMap::new(), vec![], vec![], vec![]);
    for path in paths {
        let bytes = fs::read(&path).unwrap();
        let src: Value = serde_json::from_slice(&bytes).unwrap();
        if n(&src["size"]) != 2 {
            continue;
        }
        let (p, a, b, d, r) = (
            n(&src["p"]),
            n(&src["a"]),
            n(&src["b"]),
            n(&src["d"]),
            n(&src["r"]),
        );
        let e = Curve { p, a, b };
        let t = Curve {
            p,
            a: red(d * d * a, p),
            b: red(d * d * d * b, p),
        };
        let name = path.file_name().unwrap().to_str().unwrap().to_owned();
        let mut expected = json!({"p":p,"a":a,"b":b,"d":d,"r":r,"source_sha256":sha(&bytes),"source_case":name,"scaling":src["scaling"],
            "main":fresh_arm(e,&src["main"],r),"twist":fresh_arm(t,&src["twist"],r)});
        for role in ["matched", "pulled_back_twist_base"] {
            if !src[role].is_null() {
                expected[role] = fresh_arm(if role == "matched" { t } else { e }, &src[role], r);
            }
        }
        if !src["scaling"].is_null() {
            let s = n(&src["scaling"]);
            assert_eq!(red(pow(s, 4, p) * a, p), t.a);
            assert_eq!(red(pow(s, 6, p) * b, p), t.b);
            for (left, right) in [("main", "matched"), ("pulled_back_twist_base", "twist")] {
                assert_eq!(
                    scale(e, as_point(&src[left]["generator"]), s),
                    as_point(&src[right]["generator"])
                );
                for (q, u) in src[left]["base_points"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .zip(src[right]["base_points"].as_array().unwrap())
                {
                    assert_eq!(scale(e, as_point(q), s), as_point(u));
                }
            }
        }
        let actual = load(&result.join(&name));
        assert_eq!(actual, expected, "case semantic mismatch: {name}");
        pins.push(json!({"case":name,"sha256":sha(format!("{}\n",serde_json::to_string(&expected).unwrap()).as_bytes())}));
        (if p == 43 {
            &mut fixture
        } else {
            &mut population
        })
        .push(name.clone());
        cases.insert(name, expected);
    }
    assert_eq!(
        (population.len(), fixture.len()),
        (88, 10),
        "source coverage"
    );
    let mut comparisons = vec![];
    for s in &population {
        compare(
            s,
            s,
            &cases[s]["main"],
            &cases[s]["twist"],
            &mut comparisons,
        );
    }
    for l in &fixture {
        for r in &fixture {
            compare(
                l,
                r,
                &cases[l]["main"],
                &cases[r]["twist"],
                &mut comparisons,
            );
        }
    }
    let joint = comparisons
        .iter()
        .filter(|v| v["joint_homogeneous"] == true)
        .count();
    let closure = comparisons
        .iter()
        .filter(|v| v["joint_closure"] == true)
        .count();
    let expected = json!({"schema":"quadratic-twist-reduced-s3/v1","cases":pins,"comparisons":comparisons,"comparison_count":188,"target_comparisons":comparisons.len(),"joint_homogeneous_gains":joint,"joint_closure_gains":closure});
    assert_eq!(
        load(&result.join("index.json")),
        expected,
        "index semantic mismatch"
    );
    write(
        receipt,
        &json!({"schema":"quadratic-twist-reduced-s3-audit/v1","cases":98,"comparisons":188,"target_comparisons":comparisons.len(),"joint_homogeneous_gains":joint,"joint_closure_gains":closure,"index_sha256":sha(&fs::read(result.join("index.json")).unwrap()),"arithmetic":"independent Rust division, point arithmetic and batch RREF","problems":0}),
    );
    println!("98 cases / 188 comparisons independently replayed; {joint} joint homogeneous gains / {closure} joint closure gains");
}
fn corrupt(kind: &str, dir: &Path) {
    let index_path = dir.join("index.json");
    let mut index = load(&index_path);
    let name = index["cases"][0]["case"].as_str().unwrap().to_owned();
    let path = dir.join(&name);
    if kind == "missing-case" {
        fs::remove_file(path).unwrap();
        return;
    }
    let mut v = load(&path);
    match kind {
        "false-remainder" => {
            v["main"]["targets"][0]["remainder"][0][2] =
                json!(n(&v["main"]["targets"][0]["remainder"][0][2]) + 1)
        }
        "false-degree" => v["main"]["targets"][0]["homogeneous"]["cutoff"] = json!(99),
        "false-rank" => v["main"]["targets"][0]["after"] = json!(3),
        _ => panic!("unknown control"),
    }
    write(&path, &v);
    index["cases"][0]["sha256"] = json!(sha(&fs::read(&path).unwrap()));
    write(&index_path, &index);
}
const PROOF: &str = "research/quadratic_twist_evidence_20261021/lean-reduced/ReducedS3.lean";
fn v(s: &str) -> Ex {
    Ex::var(s)
}
fn sum(xs: Vec<Ex>) -> Ex {
    xs.into_iter().fold(Ex::Const(0), Ex::plus)
}
fn prod(xs: Vec<Ex>) -> Ex {
    Ex::product(xs)
}
fn neg(x: Ex) -> Ex {
    Ex::minus(x)
}
fn sq(x: Ex) -> Ex {
    prod(vec![x.clone(), x])
}
fn statement() -> IdentityStatement {
    let a = v("a");
    let b = v("b");
    let z = v("z");
    let s = v("s");
    let t = v("t");
    let x = v("x");
    let y = v("y");
    let s3 = sum(vec![
        prod(vec![
            sq(sum(vec![x.clone(), neg(y.clone())])),
            sq(z.clone()),
        ]),
        neg(prod(vec![
            Ex::Const(2),
            sum(vec![
                prod(vec![
                    sum(vec![x.clone(), y.clone()]),
                    sum(vec![prod(vec![x.clone(), y.clone()]), a.clone()]),
                ]),
                prod(vec![Ex::Const(2), b.clone()]),
            ]),
            z.clone(),
        ])),
        sq(sum(vec![prod(vec![x.clone(), y.clone()]), neg(a.clone())])),
        neg(prod(vec![
            Ex::Const(4),
            b.clone(),
            sum(vec![x.clone(), y.clone()]),
        ])),
    ]);
    let aa = sum(vec![
        sq(s.clone()),
        neg(prod(vec![Ex::Const(4), s.clone(), z.clone()])),
        neg(prod(vec![Ex::Const(2), sq(z.clone())])),
        neg(prod(vec![Ex::Const(2), a.clone()])),
    ]);
    let bb = sum(vec![
        neg(prod(vec![s.clone(), t.clone()])),
        prod(vec![Ex::Const(2), z.clone(), t.clone()]),
        prod(vec![s.clone(), sq(z.clone())]),
        neg(prod(vec![Ex::Const(2), a.clone(), z.clone()])),
        neg(prod(vec![Ex::Const(4), b.clone()])),
    ]);
    let cc = sum(vec![
        sq(t.clone()),
        neg(prod(vec![Ex::Const(2), t.clone(), sq(z.clone())])),
        sq(a),
        neg(prod(vec![Ex::Const(4), b, z.clone()])),
    ]);
    let h = sum(vec![
        prod(vec![aa, x.clone(), y.clone()]),
        prod(vec![bb, sum(vec![x.clone(), y.clone()])]),
        cc,
    ]);
    let gx = sum(vec![
        sq(x.clone()),
        neg(prod(vec![s.clone(), x.clone()])),
        t.clone(),
    ]);
    let gy = sum(vec![
        sq(y.clone()),
        neg(prod(vec![s.clone(), y.clone()])),
        t.clone(),
    ]);
    let qx = sum(vec![
        sq(y.clone()),
        neg(prod(vec![Ex::Const(2), z.clone(), y])),
        sq(z.clone()),
    ]);
    let qy = sum(vec![
        prod(vec![
            sum(vec![s, neg(prod(vec![Ex::Const(2), z.clone()]))]),
            x,
        ]),
        sq(z),
        neg(t),
    ]);
    IdentityStatement {
        name: "factor-base-reduced-S3-division".into(),
        source: format!("{PROOF}:ReducedS3.division_identity; polynomial only"),
        variables: ["a", "b", "s", "t", "x", "y", "z"]
            .map(str::to_string)
            .into(),
        degree_bound: 4,
        lhs: s3,
        rhs: sum(vec![prod(vec![qx, gx]), prod(vec![qy, gy]), h]),
    }
}
fn verify_certificate(root: &Path, c: &IdentityCertificate) -> Result<(), String> {
    if serde_json::to_value(&c.statement).unwrap() != serde_json::to_value(statement()).unwrap() {
        return Err("wrong statement".into());
    }
    let p = c.proof.as_ref().ok_or("missing source")?;
    if p.kind != "lean4" || p.path != PROOF || p.sha256 != sha(&fs::read(root.join(PROOF)).unwrap())
    {
        return Err("false source hash".into());
    }
    match identity_certificate::check_with_extra(c, Some((2381, 16))).map_err(|e| e.to_string())? {
        Verdict::Accept {
            points: 32,
            extra_points: 16,
            ..
        } => Ok(()),
        other => Err(format!("rejected: {other:?}")),
    }
}
fn certificate(mode: &str, root: &Path, out: &Path) {
    if mode == "issue" {
        assert!(!out.exists());
        fs::create_dir_all(out).unwrap();
    }
    let c: IdentityCertificate = if mode == "issue" {
        identity_certificate::issue(
            statement(),
            20261025,
            32,
            "Exact division polynomial only; not a runtime/yield theorem",
            Some(ProofRef {
                kind: "lean4".into(),
                path: PROOF.into(),
                sha256: sha(&fs::read(root.join(PROOF)).unwrap()),
            }),
        )
        .unwrap()
    } else {
        serde_json::from_slice(&fs::read(out.join("identity.json")).unwrap()).unwrap()
    };
    verify_certificate(root, &c).unwrap();
    let mut wrong = c.clone();
    wrong.proof.as_mut().unwrap().sha256 = "0".repeat(64);
    let rejection = verify_certificate(root, &wrong).unwrap_err();
    let mut bad = statement();
    bad.rhs = Ex::plus(bad.rhs, Ex::Const(1));
    let witness =
        match identity_certificate::issue(bad.clone(), 20261025, 32, "false plus one", None)
            .unwrap_err()
        {
            identity_certificate::IssueError::False(w) => w,
            e => panic!("wrong failure {e:?}"),
        };
    let controls = json!({"false_source":wrong,"source_rejection":rejection,"false_identity":bad,"counterexample":{"index":witness.index,"assignment":witness.assignment,"lhs":witness.lhs,"rhs":witness.rhs}});
    let receipt = json!({"id":c.id,"status":"passed","issue_points":32,"extra_points":16,"false_identity":"rejected","false_source":"rejected","source":c.proof});
    for (name, value) in [
        ("identity.json", serde_json::to_value(&c).unwrap()),
        ("controls.json", controls),
        ("receipt.json", receipt),
    ] {
        let bytes = format!("{}\n", serde_json::to_string(&value).unwrap());
        if mode == "issue" {
            fs::write(out.join(name), bytes).unwrap();
        } else {
            assert_eq!(fs::read_to_string(out.join(name)).unwrap(), bytes);
        }
    }
    println!(
        "Source-bound division certificate {}, 32+16 points and controls pass",
        c.id
    );
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("audit") => {
            assert_eq!(a.len(), 5);
            fresh_audit(Path::new(&a[2]), Path::new(&a[3]), Path::new(&a[4]));
        }
        Some("corrupt") => {
            assert_eq!(a.len(), 4);
            corrupt(&a[2], Path::new(&a[3]));
        }
        Some("certificate") => {
            assert_eq!(a.len(), 5);
            certificate(&a[2], Path::new(&a[3]), Path::new(&a[4]));
        }
        _ => panic!("audit ORIGINAL_CASES RESULTS FRESH_RECEIPT | corrupt KIND COPY_OF_RESULTS"),
    }
}
