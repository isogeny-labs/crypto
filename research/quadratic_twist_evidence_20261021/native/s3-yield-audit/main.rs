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
fn main_degree_witness() -> Value {
    let c = Curve { p: 43, a: 1, b: 0 };
    let eqs = [
        s3_poly(c, 31),
        base_poly(&[4, 13], 0, 43),
        base_poly(&[4, 13], 1, 43),
    ];
    let mut witnesses = vec![];
    for d in [4, 5] {
        let ms = monomials(d);
        let indices: BTreeMap<_, _> = ms.iter().enumerate().map(|(i, &m)| (m, i)).collect();
        let mut rows = vec![];
        let mut labels = vec![];
        for (i, f) in eqs.iter().enumerate() {
            let deg = f.keys().map(|(i, j)| i + j).max().unwrap();
            for (sx, sy) in monomials(d - deg) {
                let mut row = vec![0; ms.len()];
                for (&(x, y), &v) in f {
                    row[indices[&(x + sx, y + sy)]] = v;
                }
                rows.push(row);
                labels.push((i, sx, sy));
            }
        }
        if d == 4 {
            let mut constraints: Vec<_> = rows
                .iter()
                .map(|r| {
                    let mut r = r.clone();
                    r.push(0);
                    r
                })
                .collect();
            let mut last = vec![0; ms.len() + 1];
            last[indices[&(0, 0)]] = 1;
            last[ms.len()] = 1;
            constraints.push(last);
            let dual = solve_linear(constraints, ms.len(), 43);
            for row in &rows {
                assert_eq!(red(row.iter().zip(&dual).map(|(a, b)| a * b).sum(), 43), 0);
            }
            assert_eq!(dual[indices[&(0, 0)]], 1);
            let sparse: Vec<_> = ms
                .iter()
                .zip(&dual)
                .filter(|(_, v)| **v != 0)
                .map(|(&(i, j), &v)| json!([i, j, v]))
                .collect();
            witnesses.push(json!({"degree":4,"dual_functional":sparse}));
        } else {
            let constraints: Vec<_> = (0..ms.len())
                .map(|j| {
                    let mut row: Vec<_> = rows.iter().map(|r| r[j]).collect();
                    row.push(I::from(j == indices[&(0, 0)]));
                    row
                })
                .collect();
            let weights = solve_linear(constraints, rows.len(), 43);
            let mut multipliers = [Poly::new(), Poly::new(), Poly::new()];
            for (&w, &(i, sx, sy)) in weights.iter().zip(&labels) {
                add_term(&mut multipliers[i], (sx, sy), w, 43);
            }
            let sparse: Vec<Vec<Value>> = multipliers
                .iter()
                .map(|f| f.iter().map(|(&(i, j), &c)| json!([i, j, c])).collect())
                .collect();
            for j in 0..ms.len() {
                assert_eq!(
                    red(rows.iter().zip(&weights).map(|(r, w)| r[j] * w).sum(), 43),
                    I::from(j == indices[&(0, 0)])
                );
            }
            witnesses.push(json!({"degree":5,"unit_multipliers":sparse}));
        }
    }
    json!(witnesses)
}
fn degree_profile(eqs: &[Poly], roots: &[(I, I)], p: I, b: usize) -> Value {
    let (mut closure, mut refutation) = (Value::Null, Value::Null);
    let mut journal = vec![];
    for d in 4usize.max(b)..=12 {
        let ms = monomials(d);
        let index: BTreeMap<_, _> = ms.iter().enumerate().map(|(i, &m)| (m, i)).collect();
        let mut rows = vec![];
        for f in eqs {
            let deg = f.keys().map(|(i, j)| i + j).max().unwrap();
            for (sx, sy) in monomials(d - deg) {
                let mut row = vec![0; ms.len()];
                for (&(i, j), &co) in f {
                    row[index[&(i + sx, j + sy)]] = co;
                }
                rows.push(row);
            }
        }
        let (basis, pivots) = rref(rows, ms.len(), p);
        let eval: Vec<_> = roots
            .iter()
            .map(|&(x, y)| {
                ms.iter()
                    .map(|&(i, j)| red(pow(x, i, p) * pow(y, j, p), p))
                    .collect()
            })
            .collect();
        let erank = rref(eval, ms.len(), p).1.len();
        let mut one = vec![0; ms.len()];
        one[index[&(0, 0)]] = 1;
        for (row, &j) in basis.iter().zip(&pivots) {
            let t = one[j];
            for i in j..ms.len() {
                one[i] = red(one[i] - t * row[i], p);
            }
        }
        let unit = one.iter().all(|&x| x == 0);
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
fn replay_arm(c: Curve, p: Point, r: I, xs: &[I], transported: Option<Vec<Point>>) -> Value {
    let bs = transported.unwrap_or_else(|| c.reps(xs));
    let labels: Vec<_> = bs
        .iter()
        .map(|&q| (0..r).find(|&i| c.mul(i, p) == q).unwrap())
        .collect();
    let eqx = base_poly(xs, 0, c.p);
    let eqy = base_poly(xs, 1, c.p);
    let (mut matrix, mut targets) = (vec![], vec![]);
    let (mut raw, mut signed, mut useful) = (0, 0, 0);
    let (mut pr, mut pu) = (0, 0);
    let mut full = Value::Null;
    for n in 1..r {
        let target = c.mul(n, p).unwrap();
        let eq = s3_poly(c, target.0);
        let before = rref(matrix.clone(), xs.len(), r).1.len();
        let (mut roots, mut count, mut coefficient_rows) = (vec![], 0, BTreeSet::new());
        for (i, &x) in xs.iter().enumerate() {
            for (j, &y) in xs.iter().enumerate() {
                let mut lifted = false;
                for si in [-1, 1] {
                    for sj in [-1, 1] {
                        if c.add(c.mul(si, bs[i]), c.mul(sj, bs[j])) == Some(target) {
                            count += 1;
                            lifted = true;
                            let mut row = vec![0; xs.len()];
                            row[i] = red(row[i] + si, r);
                            row[j] = red(row[j] + sj, r);
                            assert_eq!(
                                red(row.iter().zip(&labels).map(|(a, b)| a * b).sum(), r),
                                n,
                                "false coefficient relation"
                            );
                            coefficient_rows.insert(row.clone());
                            matrix.push(row);
                        }
                    }
                }
                assert_eq!(
                    s3_value(c, x, y, target.0) == 0,
                    lifted,
                    "S3 geometric lifting mismatch"
                );
                if lifted {
                    roots.push((x, y));
                }
            }
        }
        let after = rref(matrix.clone(), xs.len(), r).1.len();
        raw += roots.len();
        signed += count;
        useful += usize::from(after > before);
        if n <= 3 {
            pr = after;
            pu += usize::from(after > before);
        }
        if after == xs.len() && full.is_null() {
            full = json!(n);
        }
        targets.push(json!({"n":n,"root_pairs":roots.len(),"signed_rows":count,"before":before,"after":after,
            "coefficient_rows":coefficient_rows,"degree":degree_profile(&[eq,eqx.clone(),eqy.clone()],&roots,c.p,xs.len())}));
    }
    json!({"generator":point_json(p),"base_x":xs,"base_points":bs.iter().map(|&p|point_json(p)).collect::<Vec<_>>(),
        "labels_for_audit_only":labels,"targets":targets,"summary":{"root_pairs":raw,"signed_rows":signed,
        "rank":rref(matrix,xs.len(),r).1.len(),"useful_targets":useful,"prefix_rank":pr,
        "prefix_useful_targets":pu,"first_full_rank":full}})
}
fn same_arm(got: &Value, want: &Value) {
    assert_eq!(
        got["targets"].as_array().unwrap().len(),
        want["targets"].as_array().unwrap().len(),
        "missing targets"
    );
    for (g, w) in got["targets"]
        .as_array()
        .unwrap()
        .iter()
        .zip(want["targets"].as_array().unwrap())
    {
        assert_eq!(g["after"], w["after"], "false useful rank");
        assert_eq!(g["degree"], w["degree"], "false degree profile");
    }
    assert_eq!(got, want, "arm exact replay mismatch");
}
fn model_names(reg: &Value) -> BTreeMap<(I, I, I), String> {
    fn small(v: &Value) -> Option<I> {
        v.as_i64().or_else(|| {
            v.as_str()
                .filter(|s| s.len() < 4)
                .and_then(|s| s.parse().ok())
        })
    }
    reg["curves"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|m| {
            if m["family"] != "prime" {
                return None;
            }
            let p = &m["params"];
            Some((
                (small(&p["p"])?, small(&p["a"])?, small(&p["b"])?),
                m["slug"].as_str()?.to_string(),
            ))
        })
        .collect()
}
fn recover(c: Curve, p: Point, target: Point, r: I, xs: &[I]) -> Value {
    // The solver never reads scalar labels. Rows are constructed with actual
    // points; right sides are known public multiples used in precomputation.
    let bs = c.reps(xs);
    let mut equations = vec![];
    let mut attempts = 0;
    let mut logs = vec![];
    for n in 1..r {
        attempts += 1;
        let q = c.mul(n, p);
        for i in 0..bs.len() {
            for j in 0..bs.len() {
                for si in [-1, 1] {
                    for sj in [-1, 1] {
                        if c.add(c.mul(si, bs[i]), c.mul(sj, bs[j])) == q {
                            let mut row = vec![0; bs.len() + 1];
                            row[i] = red(row[i] + si, r);
                            row[j] = red(row[j] + sj, r);
                            row[bs.len()] = n;
                            equations.push(row);
                        }
                    }
                }
            }
        }
        let (echelon, pivots) = rref(equations.clone(), bs.len() + 1, r);
        if pivots.len() == bs.len() {
            logs = (0..bs.len()).map(|i| echelon[i][bs.len()]).collect();
            break;
        }
    }
    assert_eq!(logs.len(), bs.len(), "base logarithms not determined");
    for (i, &l) in logs.iter().enumerate() {
        assert_eq!(c.mul(l, p), bs[i]);
    }
    for shift in 0..r {
        let q = c.add(target, c.mul(shift, p));
        if q.is_none() {
            return json!({"recovered":red(-shift,r),"precomputation_targets":attempts,"shift":shift,"method":"identity","base_logs":logs});
        }
        for (i, &b) in bs.iter().enumerate() {
            for sign in [-1, 1] {
                if c.mul(sign, b) == q {
                    let recovered = red(sign * logs[i] - shift, r);
                    assert_eq!(c.mul(recovered, p), target);
                    return json!({"recovered":recovered,"precomputation_targets":attempts,"shift":shift,"method":"factor-base lookup","base_logs":logs});
                }
            }
        }
        for i in 0..bs.len() {
            for j in 0..bs.len() {
                for si in [-1, 1] {
                    for sj in [-1, 1] {
                        if c.add(c.mul(si, bs[i]), c.mul(sj, bs[j])) == q {
                            let recovered = red(si * logs[i] + sj * logs[j] - shift, r);
                            assert_eq!(c.mul(recovered, p), target);
                            return json!({"recovered":recovered,"precomputation_targets":attempts,"shift":shift,"method":"signed pair","base_logs":logs});
                        }
                    }
                }
            }
        }
    }
    panic!("target descent failed");
}
fn audit(dir: &Path, reg_path: &Path, out: &Path) {
    assert!(!out.exists(), "refusing receipt overwrite");
    let raw = fs::read(reg_path).unwrap();
    let ids = model_names(&serde_json::from_slice(&raw).unwrap());
    let index: Value = serde_json::from_slice(&fs::read(dir.join("index.json")).unwrap()).unwrap();
    assert_eq!(index["registry_sha256"], sha(&raw));
    let mut stored = BTreeMap::new();
    for entry in index["cases"].as_array().unwrap() {
        let name = entry["path"].as_str().unwrap();
        assert!(!name.contains('/'));
        let bytes = fs::read(dir.join(name)).unwrap();
        assert_eq!(entry["sha256"], sha(&bytes));
        let v: Value = serde_json::from_slice(&bytes).unwrap();
        assert!(stored.insert(name.to_string(), v).is_none());
    }
    let (mut models, mut shared, mut small, mut scaling_cases, mut systems) = (0, 0, 0, 0, 0);
    let (mut compared, mut reg_lower, mut prefix_better, mut joint_first, mut mapped_joint) =
        (0, 0, 0, 0, 0);
    let mut first_fixture = Value::Null;
    let mut subset_table = vec![];
    let mut joint_cases = vec![];
    let mut prefix_rank_cases = vec![];
    let mut populations = vec![];
    for p in [5, 7, 11, 13, 17, 19] {
        let d = nonsquare(p);
        for a in 0..p {
            for b in 0..p {
                if red(4 * a * a * a + 27 * b * b, p) == 0 {
                    continue;
                }
                models += 1;
                let e = Curve { p, a, b };
                let t = Curve {
                    p,
                    a: red(d * d * a, p),
                    b: red(d * d * d * b, p),
                };
                for r in primes(gcd(e.points().len() as I, t.points().len() as I)) {
                    if r == 2 {
                        continue;
                    }
                    shared += 1;
                    if r < 5 {
                        small += 1;
                        continue;
                    }
                    populations.push((e, d, r, false));
                }
            }
        }
    }
    populations.push((Curve { p: 43, a: 1, b: 0 }, 2, 11, true));
    for (e, d, r, fixture) in populations {
        let t = Curve {
            p: e.p,
            a: red(d * d * e.a, e.p),
            b: red(d * d * d * e.b, e.p),
        };
        let s = (1..e.p).find(|&s| {
            red(pow(s, 4, e.p) * e.a - t.a, e.p) == 0 && red(pow(s, 6, e.p) * e.b - t.b, e.p) == 0
        });
        let p = if fixture {
            Some((31, 18))
        } else {
            e.generator(r)
        };
        let u = s.map(|s| scale(e, p, s)).unwrap_or_else(|| t.generator(r));
        if let Some(s) = s {
            scaling_cases += 1;
            for n in 0..r {
                for m in 0..r {
                    let q = scale(e, e.mul(n, p), s);
                    assert!(t.contains(q));
                    assert_eq!(q, t.mul(n, u));
                    assert_eq!(
                        scale(e, e.add(e.mul(n, p), e.mul(m, p)), s),
                        t.add(t.mul(n, u), t.mul(m, u))
                    );
                }
            }
        }
        let ex = e.xs(p, r);
        let tx = t.xs(u, r);
        for bsize in [2, 3] {
            if ex.len() < bsize {
                continue;
            }
            let eb = if fixture {
                subs(&ex, bsize)
            } else {
                vec![ex[..bsize].to_vec()]
            };
            let tb = if fixture {
                subs(&tx, bsize)
            } else {
                vec![tx[..bsize].to_vec()]
            };
            for (policy, (bx, by)) in eb.iter().zip(&tb).enumerate() {
                let name = format!(
                    "p{}-a{}-b{}-r{r}-size{bsize}-policy{policy}.json",
                    e.p, e.a, e.b
                );
                let got = stored.remove(&name).expect("missing case coverage");
                assert_eq!(got["scaling"], json!(s), "false scaling");
                let main = replay_arm(e, p, r, bx, None);
                let twist = replay_arm(t, u, r, by, None);
                same_arm(&got["main"], &main);
                same_arm(&got["twist"], &twist);
                let matched = if let Some(s) = s {
                    let mb: Vec<_> = e.reps(bx).into_iter().map(|q| scale(e, q, s)).collect();
                    let mx: Vec<_> = mb.iter().map(|q| q.unwrap().0).collect();
                    let v = replay_arm(t, u, r, &mx, Some(mb));
                    assert_eq!(
                        main["targets"], v["targets"],
                        "matched coefficient/rank/degree invariance"
                    );
                    v
                } else {
                    Value::Null
                };
                same_arm_optional(&got["matched"], &matched);
                let pullback = if let Some(s) = s {
                    let pb: Vec<_> = t
                        .reps(by)
                        .into_iter()
                        .map(|q| scale(e, q, inv(s, e.p)))
                        .collect();
                    let px: Vec<_> = pb.iter().map(|q| q.unwrap().0).collect();
                    let v = replay_arm(e, p, r, &px, Some(pb));
                    assert_eq!(
                        twist["targets"], v["targets"],
                        "twist advantage not reproduced by main-curve pullback"
                    );
                    v
                } else {
                    Value::Null
                };
                same_arm_optional(&got["pulled_back_twist_base"], &pullback);
                let hilbert: Vec<_> = (0..=bsize + 2)
                    .map(|dd| {
                        (0..=dd)
                            .filter(|&i| i < bsize && dd - i < bsize && (i < 2 || dd - i < 2))
                            .count()
                    })
                    .collect();
                let want = json!({"p":e.p,"a":e.a,"b":e.b,"d":d,"r":r,"size":bsize,"policy":policy,
                    "additional_fixture":fixture,"icv1":ids[&(e.p,e.a,e.b)],"twist_icv1":ids[&(t.p,t.a,t.b)],
                    "scaling":s,"homogeneous_hilbert":hilbert,"homogeneous_regularity":bsize+1,
                    "main":main,"twist":twist,"matched":matched,"pulled_back_twist_base":pullback});
                assert_eq!(got, want, "case replay mismatch");
                compared += 1;
                systems += 2 * (r - 1) + if s.is_some() { 2 * (r - 1) } else { 0 };
                reg_lower += usize::from(n(&want["homogeneous_regularity"]) < (bsize + 1) as I);
                prefix_better += usize::from(
                    n(&twist["summary"]["prefix_rank"]) > n(&main["summary"]["prefix_rank"]),
                );
                if n(&twist["summary"]["prefix_rank"]) > n(&main["summary"]["prefix_rank"]) {
                    prefix_rank_cases.push(name.clone());
                }
                let mt = &main["targets"][0];
                let tt = &twist["targets"][0];
                let joint = n(&tt["after"]) > n(&mt["after"])
                    && n(&tt["degree"]["closure"]) < n(&mt["degree"]["closure"]);
                joint_first += usize::from(joint);
                mapped_joint += usize::from(joint && s.is_some());
                if joint {
                    joint_cases.push(json!({"case":name,"scaling":s,
                        "main_first_rank":mt["after"],"twist_first_rank":tt["after"],
                        "main_first_closure":mt["degree"]["closure"],"twist_first_closure":tt["degree"]["closure"]}));
                }
                if fixture {
                    subset_table.push(json!({"size":bsize,"policy":policy,"main_x":bx,"twist_x":by,
                    "main_first_rank":mt["after"],"twist_first_rank":tt["after"],
                    "main_first_closure":mt["degree"]["closure"],"twist_first_closure":tt["degree"]["closure"],
                    "main_full_at":main["summary"]["first_full_rank"],"twist_full_at":twist["summary"]["first_full_rank"]}));
                    if bsize == 2 && policy == 0 {
                        let mut recoveries = vec![];
                        for z in 0..11 {
                            let target = e.mul(z, p);
                            let image = scale(e, target, s.unwrap());
                            let mut a = recover(e, p, target, 11, bx);
                            let mut b = recover(t, u, image, 11, by);
                            a["precomputation_signed_candidates"] = json!(
                                n(&a["precomputation_targets"]) * 4 * (bx.len() * bx.len()) as I
                            );
                            b["precomputation_signed_candidates"] = json!(
                                n(&b["precomputation_targets"]) * 4 * (by.len() * by.len()) as I
                            );
                            a["independent_rows"] = json!(bx.len());
                            b["independent_rows"] = json!(by.len());
                            assert_eq!(n(&a["recovered"]), z);
                            assert_eq!(n(&b["recovered"]), z);
                            recoveries.push(json!({"target":point_json(target),"twist_target":point_json(image),"main":a,"twist":b}));
                        }
                        let profile:Vec<_>=main["targets"].as_array().unwrap().iter().zip(twist["targets"].as_array().unwrap())
                            .map(|(m,t)|json!({"n":m["n"],"main_closure":m["degree"]["closure"],"twist_closure":t["degree"]["closure"],
                                "main_rank":m["after"],"twist_rank":t["after"],"main_root_pairs":m["root_pairs"],"twist_root_pairs":t["root_pairs"]})).collect();
                        let msum: I = profile.iter().map(|v| n(&v["main_closure"])).sum();
                        let tsum: I = profile.iter().map(|v| n(&v["twist_closure"])).sum();
                        first_fixture = json!({"main_first_target":mt,"twist_first_target":tt,"recoveries":recoveries,
                            "profile":profile,"closure_sums":{"main":msum,"twist":tsum,"targets":r-1},
                            "pulled_back_twist_x":pullback["base_x"]});
                    }
                }
            }
        }
    }
    assert!(stored.is_empty(), "unexpected case coverage");
    let summary = json!({"models":models,"shared_cases":shared,"too_small_for_size2":small,"eligible_cases":shared-small,
        "comparisons":compared,"scaling_cases":scaling_cases,"target_systems_including_controls":systems});
    assert_eq!(index["summary"], summary, "false declared summary");
    assert_eq!(
        (models, shared, small, compared, systems),
        (942, 274, 186, 124, 1904)
    );
    let receipt = json!({"status":"passed","summary":summary,"twist_lower_homogeneous_regularity":reg_lower,
        "twist_better_prefix_rank":prefix_better,"first_target_joint_rank_and_closure":joint_first,
        "first_target_joint_with_scaling":mapped_joint,"joint_cases":joint_cases,"prefix_rank_cases":prefix_rank_cases,
        "F43_prefix_size2":first_fixture,"F43_degree_witnesses":main_degree_witness(),"F43_all_subsets":subset_table,
        "scope":"finite useful rank, exact Macaulay row spaces and base-field maps; no runtime or uniform regularity gain"});
    fs::write(out, serde_json::to_string_pretty(&receipt).unwrap() + "\n").unwrap();
    println!("PASS independent actual S3 replay: {compared} cases, {systems} target systems; {joint_first} first-target joint gains, {mapped_joint} with scaling");
}
fn same_arm_optional(g: &Value, w: &Value) {
    if w.is_null() {
        assert!(g.is_null());
    } else {
        same_arm(g, w);
    }
}

const PROOF: &str = "research/quadratic_twist_evidence_20261021/lean/S3System.lean";
fn var(s: &str) -> Ex {
    Ex::var(s)
}
fn add(a: Ex, b: Ex) -> Ex {
    Ex::plus(a, b)
}
fn mul(a: Ex, b: Ex) -> Ex {
    Ex::product(vec![a, b])
}
fn sub(a: Ex, b: Ex) -> Ex {
    add(a, Ex::minus(b))
}
fn power(a: Ex, n: usize) -> Ex {
    if n == 0 {
        Ex::Const(1)
    } else {
        Ex::product(vec![a; n])
    }
}
fn expr_s3(a: Ex, b: Ex, x: Ex, y: Ex, z: Ex) -> Ex {
    sub(
        add(
            sub(
                mul(power(sub(x.clone(), y.clone()), 2), power(z.clone(), 2)),
                mul(
                    Ex::Const(2),
                    mul(
                        add(
                            mul(
                                add(x.clone(), y.clone()),
                                add(mul(x.clone(), y.clone()), a.clone()),
                            ),
                            mul(Ex::Const(2), b.clone()),
                        ),
                        z,
                    ),
                ),
            ),
            power(sub(mul(x.clone(), y.clone()), a), 2),
        ),
        mul(Ex::Const(4), mul(b, add(x, y))),
    )
}
fn statement() -> IdentityStatement {
    IdentityStatement {
        name: "actual-S3-twist-scaling".into(),
        source: format!("{PROOF}:S3System.twist_scaling_identity; polynomial only"),
        variables: ["a", "b", "d", "x", "y", "z"].map(str::to_string).into(),
        degree_bound: 8,
        lhs: expr_s3(
            mul(power(var("d"), 2), var("a")),
            mul(power(var("d"), 3), var("b")),
            mul(var("d"), var("x")),
            mul(var("d"), var("y")),
            mul(var("d"), var("z")),
        ),
        rhs: mul(
            power(var("d"), 4),
            expr_s3(var("a"), var("b"), var("x"), var("y"), var("z")),
        ),
    }
}
fn poly_expr(terms: &[(i64, usize, usize)]) -> Ex {
    terms.iter().fold(Ex::Const(0), |sum, &(c, i, j)| {
        add(
            sum,
            mul(Ex::Const(c), mul(power(var("x"), i), power(var("y"), j))),
        )
    })
}
const UNIT_PROOF: &str = "research/quadratic_twist_evidence_20261021/lean/S3DegreeFixture.lean";
fn unit_statement() -> IdentityStatement {
    let f = poly_expr(&[
        (1, 2, 2),
        (24, 2, 1),
        (24, 1, 2),
        (15, 2, 0),
        (11, 1, 1),
        (15, 0, 2),
        (24, 1, 0),
        (24, 0, 1),
        (1, 0, 0),
    ]);
    let gx = poly_expr(&[(1, 2, 0), (26, 1, 0), (9, 0, 0)]);
    let gy = poly_expr(&[(1, 0, 2), (26, 0, 1), (9, 0, 0)]);
    let w0 = poly_expr(&[(37, 0, 0), (16, 1, 0), (16, 0, 1)]);
    let w1 = poly_expr(&[
        (12, 0, 0),
        (37, 0, 1),
        (41, 0, 2),
        (27, 0, 3),
        (18, 1, 0),
        (3, 1, 1),
        (27, 1, 2),
    ]);
    let w2 = poly_expr(&[(27, 0, 0), (33, 0, 1), (38, 1, 0), (32, 1, 1)]);
    let k = poly_expr(&[
        (9, 0, 0),
        (40, 1, 0),
        (80, 1, 1),
        (81, 1, 2),
        (26, 1, 3),
        (33, 2, 0),
        (33, 2, 1),
        (36, 2, 2),
        (1, 2, 3),
        (6, 3, 0),
        (9, 3, 1),
        (1, 3, 2),
        (52, 0, 1),
        (51, 0, 2),
        (12, 0, 3),
    ]);
    IdentityStatement{name:"F43-S3-degree5-cleared-unit".into(),source:format!("{UNIT_PROOF}:S3DegreeFixture.cleared_unit_identity; 43=0 and the degree4 obstruction are checked separately in Lean"),
        variables:["x","y"].map(str::to_string).into(),degree_bound:5,
        lhs:sub(add(add(mul(w0,f),mul(w1,gx)),mul(w2,gy)),Ex::Const(1)),rhs:mul(Ex::Const(43),k)}
}
fn verify_cert(
    root: &Path,
    c: &IdentityCertificate,
    expected: &IdentityStatement,
    proof: &str,
) -> Result<(), String> {
    if serde_json::to_value(&c.statement).unwrap() != serde_json::to_value(expected).unwrap() {
        return Err("wrong statement".into());
    }
    let p = c.proof.as_ref().ok_or("missing proof")?;
    if p.kind != "lean4" || p.path != proof || p.sha256 != sha(&fs::read(root.join(proof)).unwrap())
    {
        return Err("false source hash".into());
    }
    match identity_certificate::check_with_extra(c, Some((2381, 16))).map_err(|e| e.to_string())? {
        Verdict::Accept {
            points: 32,
            extra_points: 16,
            ..
        } => Ok(()),
        v => Err(format!("certificate rejected: {v:?}")),
    }
}
fn certificate(mode: &str, root: &Path, out: &Path) {
    if mode == "issue" {
        assert!(!out.exists());
        fs::create_dir_all(out).unwrap();
    }
    for (stem, expected, proof) in [
        ("identity", statement(), PROOF),
        ("degree5-unit", unit_statement(), UNIT_PROOF),
    ] {
        let c = if mode == "issue" {
            identity_certificate::issue(
                expected.clone(),
                20261024,
                32,
                "S3 scaling polynomial only, not a yield/regularity/runtime assertion",
                Some(ProofRef {
                    kind: "lean4".into(),
                    path: proof.into(),
                    sha256: sha(&fs::read(root.join(proof)).unwrap()),
                }),
            )
            .unwrap()
        } else {
            serde_json::from_slice(&fs::read(out.join(format!("{stem}.json"))).unwrap()).unwrap()
        };
        verify_cert(root, &c, &expected, proof).unwrap();
        let mut wrong = c.clone();
        wrong.proof.as_mut().unwrap().sha256 = "0".repeat(64);
        let rejected = verify_cert(root, &wrong, &expected, proof).unwrap_err();
        let mut bad = expected;
        bad.rhs = add(bad.rhs, Ex::Const(1));
        let witness =
            match identity_certificate::issue(bad.clone(), 20261024, 32, "false plus one", None)
                .unwrap_err()
            {
                identity_certificate::IssueError::False(w) => w,
                e => panic!("wrong rejection: {e:?}"),
            };
        for (name, v) in [
            (format!("{stem}.json"), serde_json::to_value(&c).unwrap()),
            (
                format!("{stem}-controls.json"),
                json!({"false_source":wrong,"rejection":rejected,
        "false_identity":bad,"counterexample":{"index":witness.index,"assignment":witness.assignment,"lhs":witness.lhs,"rhs":witness.rhs}}),
            ),
            (
                format!("{stem}-receipt.json"),
                json!({"status":"passed","id":c.id,"source":c.proof,"issue_points":32,"extra_points":16,"false_identity":"rejected","false_source":"rejected"}),
            ),
        ] {
            // Canonical compact JSON is ordinary text, not a compressed blob.
            let bytes = serde_json::to_string(&v).unwrap() + "\n";
            if mode == "issue" {
                fs::write(out.join(name), bytes).unwrap();
            } else {
                assert_eq!(fs::read_to_string(out.join(name)).unwrap(), bytes);
            }
        }
    }
    println!("PASS S3 polynomial source binding, 32+16 points and controls");
}
fn corrupt(kind: &str, src: &Path, out: &Path) {
    assert!(!out.exists());
    fs::create_dir_all(out).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let p = entry.unwrap().path();
        fs::copy(&p, out.join(p.file_name().unwrap())).unwrap();
    }
    let mut index: Value =
        serde_json::from_slice(&fs::read(out.join("index.json")).unwrap()).unwrap();
    if kind == "missing-case" {
        index["cases"].as_array_mut().unwrap().pop();
    } else {
        let name = index["cases"][0]["path"].as_str().unwrap().to_string();
        let mut v: Value = serde_json::from_slice(&fs::read(out.join(&name)).unwrap()).unwrap();
        match kind {
            "false-rank" => v["main"]["targets"][0]["after"] = json!(99),
            "false-degree" => v["main"]["targets"][0]["degree"]["closure"] = json!(0),
            "false-scaling" => v["scaling"] = json!(1),
            _ => panic!("unknown control"),
        }
        let bytes = serde_json::to_vec(&v).unwrap();
        fs::write(out.join(name), &bytes).unwrap();
        index["cases"][0]["sha256"] = json!(sha(&bytes));
    }
    fs::write(out.join("index.json"), serde_json::to_vec(&index).unwrap()).unwrap();
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a.get(1).map(String::as_str){
    Some("audit")=>{assert_eq!(a.len(),5);audit(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4]));},
    Some("certificate")=>{assert_eq!(a.len(),5);certificate(&a[2],Path::new(&a[3]),Path::new(&a[4]));},
    Some("corrupt")=>{assert_eq!(a.len(),5);corrupt(&a[2],Path::new(&a[3]),Path::new(&a[4]));},
    _=>panic!("s3-yield-audit audit RESULTS REGISTRY RECEIPT | certificate issue|check ROOT OUT | corrupt KIND RESULTS OUT")}
}
