//! Exact fixed-fixture exporter and correctness checks. No timing benchmark.
use serde::Serialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const PRIME: i64 = 17;
fn f(x: i64) -> i64 {
    x.rem_euclid(PRIME)
}
fn inv(x: i64) -> i64 {
    assert_ne!(f(x), 0);
    (1..PRIME).find(|y| f(x * y) == 1).unwrap()
}
#[derive(Clone, Debug, Serialize)]
struct Poly {
    n: usize,
    terms: BTreeMap<Vec<u8>, i64>,
}
impl Poly {
    fn c(n: usize, c: i64) -> Self {
        let mut p = Self {
            n,
            terms: BTreeMap::new(),
        };
        if f(c) != 0 {
            p.terms.insert(vec![0; n], f(c));
        }
        p
    }
    fn x(n: usize, i: usize) -> Self {
        let mut p = Self::c(n, 0);
        let mut e = vec![0; n];
        e[i] = 1;
        p.terms.insert(e, 1);
        p
    }
    fn add(&self, q: &Self) -> Self {
        assert_eq!(self.n, q.n);
        let mut p = self.clone();
        for (e, c) in &q.terms {
            let v = f(p.terms.get(e).unwrap_or(&0) + c);
            if v == 0 {
                p.terms.remove(e);
            } else {
                p.terms.insert(e.clone(), v);
            }
        }
        p
    }
    fn scale(&self, c: i64) -> Self {
        let mut p = Self::c(self.n, 0);
        for (e, v) in &self.terms {
            if f(c * v) != 0 {
                p.terms.insert(e.clone(), f(c * v));
            }
        }
        p
    }
    fn sub(&self, q: &Self) -> Self {
        self.add(&q.scale(-1))
    }
    fn mul(&self, q: &Self) -> Self {
        assert_eq!(self.n, q.n);
        let mut p = Self::c(self.n, 0);
        for (e, c) in &self.terms {
            for (d, b) in &q.terms {
                let ex: Vec<_> = e
                    .iter()
                    .zip(d)
                    .map(|(a, b)| a.checked_add(*b).unwrap())
                    .collect();
                let v = f(p.terms.get(&ex).unwrap_or(&0) + c * b);
                if v == 0 {
                    p.terms.remove(&ex);
                } else {
                    p.terms.insert(ex, v);
                }
            }
        }
        p
    }
    fn pow(&self, k: u32) -> Self {
        (0..k).fold(Self::c(self.n, 1), |p, _| p.mul(self))
    }
    #[cfg(test)]
    fn eval(&self, v: &[i64]) -> i64 {
        assert_eq!(v.len(), self.n);
        f(self
            .terms
            .iter()
            .map(|(e, c)| {
                e.iter()
                    .zip(v)
                    .fold(*c, |a, (k, x)| (0..*k).fold(a, |v, _| f(v * x)))
            })
            .sum())
    }
    fn text(&self) -> String {
        if self.terms.is_empty() {
            return "0".into();
        }
        self.terms
            .iter()
            .map(|(e, c)| {
                let mut s = c.to_string();
                for (i, k) in e.iter().enumerate() {
                    if *k > 0 {
                        s.push_str(&format!("*x{i}^{k}"));
                    }
                }
                s
            })
            .collect::<Vec<_>>()
            .join("+")
    }
    fn value(&self) -> Value {
        json!({"variables":self.n,"terms":self.terms.iter().map(|(e,c)|json!({"e":e,"c":c})).collect::<Vec<_>>()})
    }
}
#[derive(Clone)]
struct System {
    mode: &'static str,
    e: i64,
    g: i64,
    r: i64,
    s: i64,
    n: usize,
    polys: Vec<Poly>,
    condition: &'static str,
}
impl System {
    fn value(&self) -> Value {
        json!({"mode":self.mode,"signs":[self.e,self.g],"target":[self.r,self.s],"variables":self.n,"polynomials":self.polys.iter().map(Poly::value).collect::<Vec<_>>(),"chart":self.condition,"solutions":"field-valued; exact group replay required"})
    }
    fn msolve(&self) -> String {
        format!(
            "{}\n17\n{}\n",
            (0..self.n)
                .map(|i| format!("x{i}"))
                .collect::<Vec<_>>()
                .join(","),
            self.polys
                .iter()
                .map(Poly::text)
                .collect::<Vec<_>>()
                .join(",\n")
        )
    }
}
fn offset(v: &[Poly], recoded: bool) -> Poly {
    let a = v[0].scale(2);
    if recoded {
        a.add(&v[1].scale(12)).sub(&v[0].mul(&v[1]).scale(4))
    } else {
        a.add(&v[1].scale(10))
    }
}
fn graph(x: &Poly) -> Poly {
    x.pow(2).scale(4).add(&x.scale(9)).add(&Poly::c(x.n, 9))
}
fn booleans(p: &mut Vec<Poly>, v: &[Poly]) {
    p.extend(v.iter().map(|x| x.pow(2).sub(x)));
}
fn basic(mode: &'static str, r: i64, s: i64, e: i64, g: i64) -> System {
    let n = if mode == "one_hot" { 9 } else { 7 };
    let v: Vec<_> = (0..n).map(|i| Poly::x(n, i)).collect();
    let (x, z, m) = (&v[0], &v[1], &v[2]);
    let c = |k| Poly::c(n, k);
    let (mut p, tx, tz) = if mode == "one_hot" {
        let tx = v[3].scale(2).add(&v[4].scale(10)).add(&v[5].scale(12));
        let tz = v[6].scale(2).add(&v[7].scale(10)).add(&v[8].scale(12));
        (vec![], tx, tz)
    } else {
        (
            vec![],
            offset(&v[3..5], mode == "xor_bits"),
            offset(&v[5..7], mode == "xor_bits"),
        )
    };
    p.extend([
        m.pow(2).sub(x).sub(z).sub(&c(r)),
        x.mul(z)
            .sub(&c(r * r))
            .sub(&m.pow(2).scale(r))
            .sub(&c(2))
            .sub(&m.scale(2 * s)),
        graph(x)
            .add(&tx)
            .scale(e)
            .sub(&m.mul(&x.sub(&c(r))))
            .add(&c(s)),
        graph(z)
            .add(&tz)
            .scale(g)
            .sub(&m.mul(&z.sub(&c(r))))
            .add(&c(s)),
    ]);
    booleans(&mut p, &v[3..]);
    if mode == "one_hot" {
        for start in [3, 6] {
            for i in start..start + 3 {
                for j in i + 1..start + 3 {
                    p.push(v[i].mul(&v[j]));
                }
            }
        }
    }
    System {
        mode,
        e,
        g,
        r,
        s,
        n,
        polys: p,
        condition: "none",
    }
}
fn chord_coeff(n: usize, m: &Poly, tx: &Poly, tz: &Poly, target: [i64; 4]) -> [Poly; 5] {
    let [r, s, e, g] = target;
    let c = |k| Poly::c(n, k);
    let sig = m.pow(2).sub(&c(r));
    let pi = m.pow(2).scale(r).add(&m.scale(2 * s)).add(&c(r * r + 2));
    let d = sig.pow(2).sub(&pi.scale(4));
    let h = sig
        .pow(2)
        .sub(&pi.scale(2))
        .scale(4)
        .add(&sig.scale(9))
        .add(&c(18));
    let ab = sig.scale(4).add(&c(9));
    let a1 = ab.scale(e - g);
    let a2 = ab.scale(e + g).sub(&m.scale(2));
    let b1 = h
        .scale(e + g)
        .add(&tx.scale(2 * e))
        .add(&tz.scale(2 * g))
        .sub(&m.mul(&sig.sub(&c(2 * r))).scale(2))
        .add(&c(4 * s));
    let b2 = h.scale(e - g).add(&tx.scale(2 * e)).sub(&tz.scale(2 * g));
    [d, a1, a2, b1, b2]
}
fn symmetric(mode: &'static str, r: i64, s: i64, e: i64, g: i64) -> System {
    let n = 6;
    let v: Vec<_> = (0..n).map(|i| Poly::x(n, i)).collect();
    let (m, d) = (&v[0], &v[1]);
    let tx = offset(&v[2..4], false);
    let tz = offset(&v[4..6], false);
    let [dd, a1, a2, b1, b2] = chord_coeff(n, m, &tx, &tz, [r, s, e, g]);
    let mut p = vec![d.pow(2).sub(&dd), a1.mul(d).add(&b1), a2.mul(d).add(&b2)];
    booleans(&mut p, &v[2..]);
    System {
        mode,
        e,
        g,
        r,
        s,
        n,
        polys: p,
        condition: "all pivots retained; d is explicit",
    }
}
fn determinant(r: i64, s: i64, e: i64, g: i64) -> System {
    let n = 13;
    let v: Vec<_> = (0..n).map(|i| Poly::x(n, i)).collect();
    let c = |k| Poly::c(n, k);
    let (x, z, m) = (&v[0], &v[1], &v[2]);
    let mut p = vec![
        m.pow(2).sub(x).sub(z).sub(&c(r)),
        x.mul(z)
            .sub(&m.pow(2).scale(r))
            .sub(&m.scale(2 * s))
            .sub(&c(r * r + 2)),
        v[7].sub(&m.mul(&x.sub(&c(r)))).add(&c(s)),
        v[8].sub(&m.mul(&z.sub(&c(r)))).add(&c(s)),
    ];
    for (ix, iy, iu, iv, iw, sign) in [(0, 7, 3, 4, 9, e), (1, 8, 5, 6, 11, g)] {
        let x = &v[ix];
        let y = &v[iy];
        let l0 = v[iu].scale(2 * sign).add(&v[iv].scale(12 * sign));
        let l1 = v[iu].scale(-4 * sign);
        p.extend([
            y.sub(&x.scale(9 * sign))
                .sub(&c(9 * sign))
                .sub(&l0)
                .add(&x.mul(&v[iw]))
                .add(&l1.mul(&v[iw + 1])),
            x.scale(4 * sign).add(&v[iw]),
            v[iv].add(&v[iw + 1]),
        ]);
    }
    booleans(&mut p, &v[3..7]);
    System {
        mode: "determinantal",
        e,
        g,
        r,
        s,
        n,
        polys: p,
        condition: "kernel first coordinate fixed1; identity pivot makes chart complete",
    }
}
fn dense(r: i64, s: i64) -> System {
    let n = 2;
    let x = Poly::x(n, 0);
    let z = Poly::x(n, 1);
    let c = |k| Poly::c(n, k);
    let xz = x.mul(&z);
    let sum = x.add(&z);
    let sm = x
        .sub(&z)
        .pow(2)
        .scale(r * r)
        .sub(&sum.mul(&xz.add(&c(2))).add(&c(4)).scale(2 * r))
        .add(&xz.sub(&c(2)).pow(2))
        .sub(&sum.scale(8));
    let xs = [0, 3, 5, 6, 7, 9, 10, 16];
    let member = |v: &Poly| xs.iter().fold(c(1), |a, t| a.mul(&v.sub(&c(*t))));
    System {
        mode: "dense",
        e: 0,
        g: 0,
        r,
        s,
        n,
        polys: vec![member(&x), member(&z), sm],
        condition: "ordinate signs recovered and replayed",
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize)]
struct Point {
    x: i64,
    y: i64,
}
fn curve(x: i64) -> i64 {
    f(x * x * x + 2 * x + 2)
}
fn points() -> Vec<Point> {
    (0..17)
        .flat_map(|x| (0..17).filter_map(move |y| (f(y * y) == curve(x)).then_some(Point { x, y })))
        .collect()
}
fn add(p: Point, q: Point) -> Option<Point> {
    if p.x == q.x && f(p.y + q.y) == 0 {
        return None;
    }
    let m = if p.x == q.x {
        f((3 * p.x * p.x + 2) * inv(2 * p.y))
    } else {
        f((q.y - p.y) * inv(q.x - p.x))
    };
    let x = f(m * m - p.x - q.x);
    Some(Point {
        x,
        y: f(m * (p.x - x) - p.y),
    })
}
fn base() -> Vec<Point> {
    points()
        .into_iter()
        .filter(|p| [0, 3, 5, 6, 7, 9, 10, 16].contains(&p.x))
        .collect()
}
fn samples() -> Vec<Point> {
    let mut p = points();
    p.sort_by_key(|p| {
        Sha256::digest(format!(
            "fourtracks-v1|icv1-fp5-tm1-a77cc6af|{}|{}",
            p.x, p.y
        ))
        .to_vec()
    });
    p
}
fn systems(r: i64, s: i64) -> Vec<System> {
    let mut out = vec![dense(r, s)];
    for e in [1, -1] {
        for g in [1, -1] {
            for mode in ["linear_bits", "xor_bits", "one_hot"] {
                out.push(basic(mode, r, s, e, g));
            }
            out.push(symmetric("symmetric", r, s, e, g));
            out.push(determinant(r, s, e, g));
        }
    }
    out
}
fn digest(x: &[u8]) -> String {
    hex::encode(Sha256::digest(x))
}
fn emit(path: &Path, panel: bool) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir(path)?;
    let old: Value = serde_json::from_str(include_str!(
        "../../pencil_search_20261101/fixture/fixture.json"
    ))?;
    let targets = if panel {
        samples()
    } else {
        vec![Point { x: 13, y: 10 }]
    };
    let mut rows = vec![];
    for target in targets {
        for (i, system) in systems(target.x, target.y).iter().enumerate() {
            let text = system.msolve();
            let name = format!("r{}-s{}-{}-{i}.ms", target.x, target.y, system.mode);
            fs::write(path.join(&name), &text)?;
            let refs: Vec<_> = base()
                .iter()
                .flat_map(|p| {
                    base()
                        .into_iter()
                        .filter_map(move |q| (add(*p, q) == Some(target)).then_some([*p, q]))
                })
                .collect();
            rows.push(json!({"system":system.value(),"msolve":name,"msolve_sha256":digest(text.as_bytes()),"ordered_group_reference":refs}));
        }
    }
    let data = json!({"schema":"four-selector-systems/v1","curve":old["curve"],"factor_base":old["factor_base"],"selection":if panel{"all finite toy targets in fixed hash order"}else{"one known-answer development fixture"},"systems":rows,"benchmark_run":false,"f4_degree":null,"runtime":null,"field_ops":null,"group_ops":null,"ram":null,"disk_traffic":null,"independent_rows":null});
    fs::write(path.join("systems.json"), serde_json::to_vec_pretty(&data)?)?;
    println!(
        "{}",
        json!({"systems":rows.len(),"metadata_sha256":digest(&serde_json::to_vec_pretty(&data)?),"benchmark_run":false})
    );
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 || !["emit-fixture", "emit-panel"].contains(&args[1].as_str()) {
        return Err("usage: four-selector-fixtures emit-fixture|emit-panel NEW_DIRECTORY".into());
    }
    emit(Path::new(&args[2]), args[1] == "emit-panel")
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    #[test]
    fn field_and_poly_arithmetic() {
        for x in 1..17 {
            assert_eq!(f(x * inv(x)), 1);
        }
        let x = Poly::x(2, 0);
        let y = Poly::x(2, 1);
        for a in 0..17 {
            for b in 0..17 {
                assert_eq!(x.add(&y).pow(2).eval(&[a, b]), f((a + b) * (a + b)));
            }
        }
    }
    #[test]
    fn identical_offset_and_point_sets() {
        let mut a = BTreeSet::new();
        let mut b = BTreeSet::new();
        for u in 0..2 {
            for v in 0..2 {
                a.insert(f(2 * u + 10 * v));
                b.insert(f(2 * u + 12 * v - 4 * u * v));
            }
        }
        assert_eq!(a, b);
        assert_eq!(a, BTreeSet::from([0, 2, 10, 12]));
        assert_eq!(base().len(), 16);
        let onehot = BTreeSet::from([0, 2, 10, 12]);
        assert_eq!(a, onehot);
    }
    #[test]
    fn exact_complete_scalar_reference() {
        let pts = points();
        assert_eq!(pts.len(), 18);
        let mut roots = 0usize;
        for r in pts {
            for e in [1, -1] {
                for g in [1, -1] {
                    for x in 0..17 {
                        for z in 0..17 {
                            for m in 0..17 {
                                for bits in 0..16 {
                                    let u = [
                                        bits & 1,
                                        (bits >> 1) & 1,
                                        (bits >> 2) & 1,
                                        (bits >> 3) & 1,
                                    ];
                                    let t = 2 * u[0] + 10 * u[1];
                                    let w = 2 * u[2] + 10 * u[3];
                                    let y = f(e * (4 * x * x + 9 * x + 9 + t));
                                    let v = f(g * (4 * z * z + 9 * z + 9 + w));
                                    let scalar = f(m * m - x - z - r.x) == 0
                                        && f(x * z - r.x * r.x - r.x * m * m - 2 - 2 * m * r.y)
                                            == 0
                                        && f(y - m * (x - r.x) + r.y) == 0
                                        && f(v - m * (z - r.x) + r.y) == 0;
                                    let p = Point { x, y };
                                    let q = Point { x: z, y: v };
                                    let expected = f(y * y) == curve(x)
                                        && f(v * v) == curve(z)
                                        && add(p, q) == Some(r)
                                        && m == if x == z {
                                            f((3 * x * x + 2) * inv(2 * y))
                                        } else {
                                            f((v - y) * inv(z - x))
                                        };
                                    assert_eq!(scalar, expected);
                                    if scalar {
                                        roots += 1;
                                        assert_eq!(f(y * y), curve(x));
                                        assert_eq!(f(v * v), curve(z));
                                        assert_eq!(
                                            add(Point { x, y }, Point { x: z, y: v }),
                                            Some(r)
                                        );
                                    }
                                    let nu = [
                                        u[0] + u[1] - 2 * u[0] * u[1],
                                        u[1],
                                        u[2] + u[3] - 2 * u[2] * u[3],
                                        u[3],
                                    ];
                                    let yt = f(e
                                        * (4 * x * x + 9 * x + 9 + 2 * nu[0] + 12 * nu[1]
                                            - 4 * nu[0] * nu[1]));
                                    let vt = f(g
                                        * (4 * z * z + 9 * z + 9 + 2 * nu[2] + 12 * nu[3]
                                            - 4 * nu[2] * nu[3]));
                                    assert_eq!((y, v), (yt, vt));
                                }
                            }
                        }
                    }
                }
            }
        }
        assert!(roots > 0);
    }
    #[test]
    fn complete_linear_square_charts() {
        for d in 0..17 {
            for a in 0..17 {
                for b in 0..17 {
                    for c in 0..17 {
                        for e in 0..17 {
                            let direct = (0..17)
                                .any(|z| f(z * z) == d && f(a * z + b) == 0 && f(c * z + e) == 0);
                            let chart = if a != 0 {
                                f(b * b - a * a * d) == 0 && f(a * e - c * b) == 0
                            } else if c != 0 {
                                f(e * e - c * c * d) == 0 && f(c * b - a * e) == 0
                            } else {
                                b == 0 && e == 0 && (0..17).any(|z| f(z * z) == d)
                            };
                            assert_eq!(direct, chart);
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn one_hot_polynomials_cover_all_group_pairs() {
        let labels = [[0, 0, 0], [1, 0, 0], [0, 1, 0], [0, 0, 1]];
        for target in points() {
            for e in [1, -1] {
                for g in [1, -1] {
                    let system = basic("one_hot", target.x, target.y, e, g);
                    for x in 0..17 {
                        for z in 0..17 {
                            for u in labels {
                                for v in labels {
                                    let y = f(e
                                        * (4 * x * x
                                            + 9 * x
                                            + 9
                                            + 2 * u[0]
                                            + 10 * u[1]
                                            + 12 * u[2]));
                                    let w = f(g
                                        * (4 * z * z
                                            + 9 * z
                                            + 9
                                            + 2 * v[0]
                                            + 10 * v[1]
                                            + 12 * v[2]));
                                    if f(y * y) != curve(x)
                                        || f(w * w) != curve(z)
                                        || add(Point { x, y }, Point { x: z, y: w }) != Some(target)
                                    {
                                        continue;
                                    }
                                    let m = if x == z {
                                        f((3 * x * x + 2) * inv(2 * y))
                                    } else {
                                        f((w - y) * inv(z - x))
                                    };
                                    assert!(system.polys.iter().all(|p| p
                                        .eval(&[x, z, m, u[0], u[1], u[2], v[0], v[1], v[2]])
                                        == 0));
                                }
                            }
                        }
                    }
                    for mask in 0..64 {
                        let bits: Vec<_> = (0..6).map(|i| (mask >> i) & 1).collect();
                        let valid = bits[..3].iter().sum::<i64>() <= 1
                            && bits[3..].iter().sum::<i64>() <= 1;
                        let mut assignment = vec![0, 0, 0];
                        assignment.extend(bits);
                        assert_eq!(
                            system.polys[4..].iter().all(|p| p.eval(&assignment) == 0),
                            valid
                        );
                    }
                }
            }
        }
    }
    #[test]
    fn symmetric_and_determinant_polynomials_match() {
        for target in points() {
            for e in [1, -1] {
                for g in [1, -1] {
                    let original = basic("linear_bits", target.x, target.y, e, g);
                    let sym = symmetric("symmetric", target.x, target.y, e, g);
                    let det = determinant(target.x, target.y, e, g);
                    let xor = basic("xor_bits", target.x, target.y, e, g);
                    for m in 0..17 {
                        for d in 0..17 {
                            for bits in 0..16 {
                                let b =
                                    [bits & 1, (bits >> 1) & 1, (bits >> 2) & 1, (bits >> 3) & 1];
                                let sig = f(m * m - target.x);
                                let x = f((sig + d) * inv(2));
                                let z = f((sig - d) * inv(2));
                                let v = [x, z, m, b[0], b[1], b[2], b[3]];
                                let sv = [m, d, b[0], b[1], b[2], b[3]];
                                assert_eq!(
                                    original.polys.iter().all(|p| p.eval(&v) == 0),
                                    sym.polys.iter().all(|p| p.eval(&sv) == 0)
                                );
                                let y = f(e
                                    * (4 * x * x + 9 * x + 9 + 2 * b[0] + 12 * b[1]
                                        - 4 * b[0] * b[1]));
                                let q = f(g
                                    * (4 * z * z + 9 * z + 9 + 2 * b[2] + 12 * b[3]
                                        - 4 * b[2] * b[3]));
                                let dv = [
                                    x,
                                    z,
                                    m,
                                    b[0],
                                    b[1],
                                    b[2],
                                    b[3],
                                    y,
                                    q,
                                    f(-4 * e * x),
                                    f(-b[1]),
                                    f(-4 * g * z),
                                    f(-b[3]),
                                ];
                                assert_eq!(
                                    xor.polys.iter().all(|p| p.eval(&v) == 0),
                                    det.polys.iter().all(|p| p.eval(&dv) == 0)
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn dense_matches_independent_group() {
        for target in points() {
            let sys = dense(target.x, target.y);
            for x in 0..17 {
                for z in 0..17 {
                    let exact = base().into_iter().filter(|p| p.x == x).any(|p| {
                        base()
                            .into_iter()
                            .filter(|q| q.x == z)
                            .any(|q| add(p, q) == Some(target))
                    });
                    assert_eq!(sys.polys.iter().all(|p| p.eval(&[x, z]) == 0), exact);
                }
            }
        }
    }
}
