//! Exact S4 certificate and independent finite-field structural profiler.
#![allow(dead_code)]
extern crate self as crypto_lib;

#[path = "../../../../../src/cryptanalysis/identity_certificate.rs"]
pub mod identity_certificate;
pub mod cryptanalysis {
    pub use crate::identity_certificate;
}
pub mod hash {
    pub mod sha256 {
        use sha2::{Digest, Sha256};
        pub fn sha256(bytes: &[u8]) -> [u8; 32] {
            Sha256::digest(bytes).into()
        }
    }
}
#[path = "../../../../../examples/quartic_twist_s4_structure.rs"]
mod certificate_probe;
#[path = "../../../../../examples/quartic_twist_s4_integer.rs"]
mod integer;

use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::time::Duration;
mod image;
mod image_identity;
mod meter;
mod stream;

#[path = "../../../../../src/cryptanalysis/f4_fp.rs"]
mod f4_fp;
#[path = "../../../../../src/cryptanalysis/fx_hash.rs"]
mod fx_hash;
// Exact ordering definitions from groebner_f4.rs; the rest of that module
// depends on the unrelated BigUint cryptography layer.
mod groebner_f4 {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ordering {
        Lex,
        Grlex,
        Grevlex,
    }
    pub fn cmp_monomial(a: &[u32], b: &[u32], ord: Ordering) -> std::cmp::Ordering {
        use std::cmp::Ordering as O;
        let da: u32 = a.iter().sum();
        let db: u32 = b.iter().sum();
        match ord {
            Ordering::Lex => a.cmp(b),
            Ordering::Grlex => da.cmp(&db).then_with(|| a.cmp(b)),
            Ordering::Grevlex => match da.cmp(&db) {
                O::Equal => {
                    for i in (0..a.len()).rev() {
                        match a[i].cmp(&b[i]) {
                            O::Equal => continue,
                            O::Less => return O::Greater,
                            O::Greater => return O::Less,
                        }
                    }
                    O::Equal
                }
                o => o,
            },
        }
    }
}

type E = [u64; 4];
type Poly = BTreeMap<[u8; 3], u64>;

#[derive(Clone, Copy)]
struct Field {
    p: u64,
    d: u64,
}
impl Field {
    fn new(p: u64) -> Self {
        assert_eq!(p % 4, 1);
        let d = (2..p).find(|&v| fpow(v, (p - 1) / 2, p) == p - 1).unwrap();
        Self { p, d }
    }
    fn scalar(&self, x: u64) -> E {
        [x % self.p, 0, 0, 0]
    }
    fn add(&self, a: E, b: E) -> E {
        std::array::from_fn(|i| (a[i] + b[i]) % self.p)
    }
    fn neg(&self, a: E) -> E {
        std::array::from_fn(|i| (self.p - a[i]) % self.p)
    }
    fn sub(&self, a: E, b: E) -> E {
        self.add(a, self.neg(b))
    }
    fn mul(&self, a: E, b: E) -> E {
        meter::fp(19);
        let mut c = [0; 7];
        for i in 0..4 {
            for j in 0..4 {
                c[i + j] = (c[i + j] + a[i] * b[j]) % self.p;
            }
        }
        for i in (4..7).rev() {
            c[i - 4] = (c[i - 4] + self.d * c[i]) % self.p;
        }
        [c[0], c[1], c[2], c[3]]
    }
    fn pow(&self, mut a: E, mut n: u64) -> E {
        let mut out = self.scalar(1);
        while n > 0 {
            if n & 1 != 0 {
                out = self.mul(out, a);
            }
            a = self.mul(a, a);
            n >>= 1;
        }
        out
    }
    fn inv(&self, a: E) -> E {
        assert_ne!(a, [0; 4]);
        self.pow(a, self.p.pow(4) - 2)
    }
}
fn fpow(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut out = 1;
    while n > 0 {
        if n & 1 != 0 {
            meter::modular(p, 1);
            out = if p <= u32::MAX as u64 {
                out * a % p
            } else {
                ((out as u128 * a as u128) % p as u128) as u64
            };
        }
        meter::modular(p, 1);
        a = if p <= u32::MAX as u64 {
            a * a % p
        } else {
            ((a as u128 * a as u128) % p as u128) as u64
        };
        n >>= 1;
    }
    out
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: E,
    y: E,
    inf: bool,
}
const INF: Point = Point {
    x: [0; 4],
    y: [0; 4],
    inf: true,
};
struct Curve {
    f: Field,
    b: E,
}
impl Curve {
    fn neg(&self, a: Point) -> Point {
        Point {
            y: self.f.neg(a.y),
            ..a
        }
    }
    fn on_curve(&self, a: Point) -> bool {
        a.inf || self.f.mul(a.y, a.y) == self.f.add(self.f.mul(self.f.mul(a.x, a.x), a.x), self.b)
    }
    fn add(&self, a: Point, b: Point) -> Point {
        meter::group_call();
        if a.inf {
            return b;
        }
        if b.inf {
            return a;
        }
        let f = self.f;
        let slope = if a.x == b.x {
            if a.y != b.y || a.y == [0; 4] {
                return INF;
            }
            f.mul(
                f.mul(f.scalar(3), f.mul(a.x, a.x)),
                f.inv(f.mul(f.scalar(2), a.y)),
            )
        } else {
            f.mul(f.sub(b.y, a.y), f.inv(f.sub(b.x, a.x)))
        };
        let x = f.sub(f.sub(f.mul(slope, slope), a.x), b.x);
        Point {
            x,
            y: f.sub(f.mul(slope, f.sub(a.x, x)), a.y),
            inf: false,
        }
    }
    fn mul(&self, mut a: Point, mut n: u64) -> Point {
        let mut out = INF;
        while n > 0 {
            if n & 1 != 0 {
                out = self.add(out, a);
            }
            a = self.add(a, a);
            n >>= 1;
        }
        out
    }
}
fn element(v: &Value) -> E {
    std::array::from_fn(|i| v[i].as_u64().unwrap())
}
fn point(v: &Value) -> Point {
    Point {
        x: element(&v[0]),
        y: element(&v[1]),
        inf: false,
    }
}

fn s3(f: Field, b: E, x: E, y: E) -> [E; 3] {
    let sum = f.add(x, y);
    let product = f.mul(x, y);
    [
        f.sub(f.mul(product, product), f.mul(f.scalar(4), f.mul(b, sum))),
        f.mul(
            f.scalar(f.p - 2),
            f.add(f.mul(sum, product), f.mul(f.scalar(2), b)),
        ),
        f.mul(f.sub(x, y), f.sub(x, y)),
    ]
}
fn direct(f: Field, b: E, x: [E; 4]) -> E {
    let a = s3(f, b, x[0], x[1]);
    let c = s3(f, b, x[2], x[3]);
    let u = f.sub(f.mul(a[2], c[0]), f.mul(a[0], c[2]));
    let v = f.sub(f.mul(a[2], c[1]), f.mul(a[1], c[2]));
    let w = f.sub(f.mul(a[1], c[0]), f.mul(a[0], c[1]));
    f.sub(f.mul(u, u), f.mul(v, w))
}
fn symmetric(f: Field, x: [E; 3]) -> [E; 3] {
    [
        f.add(f.add(x[0], x[1]), x[2]),
        f.add(
            f.add(f.mul(x[0], x[1]), f.mul(x[0], x[2])),
            f.mul(x[1], x[2]),
        ),
        f.mul(f.mul(x[0], x[1]), x[2]),
    ]
}
fn monos() -> Vec<[u8; 3]> {
    let mut out = Vec::new();
    for i in 0..=4 {
        for j in 0..=4 - i {
            for k in 0..=4 - i - j {
                out.push([i, j, k]);
            }
        }
    }
    out
}
fn monomial(f: Field, e: [E; 3], m: [u8; 3]) -> E {
    (0..3).fold(f.scalar(1), |v, i| f.mul(v, f.pow(e[i], m[i] as u64)))
}
fn next(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e3779b97f4a7c15);
    let mut x = *state;
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}
fn interpolate(f: Field, b: E) -> Vec<([u8; 3], [E; 5])> {
    let ms = monos();
    let n = ms.len();
    let mut rng = 0x53345f494e544552;
    for _ in 0..100 {
        let mut rows = Vec::new();
        // Solve all 35 symmetric coefficients at each of five last-variable values.
        for _ in 0..n {
            let x: [E; 3] = std::array::from_fn(|_| std::array::from_fn(|_| next(&mut rng) % f.p));
            let e = symmetric(f, x);
            let mut row: Vec<E> = ms.iter().map(|&m| monomial(f, e, m)).collect();
            row.extend((0..5).map(|j| direct(f, b, [x[0], x[1], x[2], f.scalar(j)])));
            rows.push(row);
        }
        let mut singular = false;
        for col in 0..n {
            let Some(pivot) = (col..n).find(|&r| rows[r][col] != [0; 4]) else {
                singular = true;
                break;
            };
            rows.swap(pivot, col);
            let inv = f.inv(rows[col][col]);
            for j in col..n + 5 {
                rows[col][j] = f.mul(rows[col][j], inv);
            }
            for r in 0..n {
                if r != col {
                    let factor = rows[r][col];
                    for j in col..n + 5 {
                        rows[r][j] = f.sub(rows[r][j], f.mul(factor, rows[col][j]));
                    }
                }
            }
        }
        if singular {
            continue;
        }
        // Invert the base-field 5x5 Vandermonde once, without the formula source.
        let mut vand = vec![vec![0; 10]; 5];
        for (i, row) in vand.iter_mut().enumerate() {
            for (j, v) in row.iter_mut().take(5).enumerate() {
                *v = fpow(i as u64, j as u64, f.p);
            }
            row[5 + i] = 1;
        }
        for col in 0..5 {
            let pivot = (col..5).find(|&r| vand[r][col] != 0).unwrap();
            vand.swap(col, pivot);
            let inv = fpow(vand[col][col], f.p - 2, f.p);
            for j in col..10 {
                vand[col][j] = vand[col][j] * inv % f.p;
            }
            for r in 0..5 {
                if r != col {
                    let k = vand[r][col];
                    for j in col..10 {
                        vand[r][j] = (vand[r][j] + f.p - k * vand[col][j] % f.p) % f.p;
                    }
                }
            }
        }
        return ms
            .into_iter()
            .enumerate()
            .map(|(r, m)| {
                let coeff = std::array::from_fn(|j| {
                    (0..5).fold([0; 4], |v, k| {
                        f.add(v, f.mul(f.scalar(vand[j][5 + k]), rows[r][n + k]))
                    })
                });
                (m, coeff)
            })
            .collect();
    }
    panic!("interpolation singular at 100 attempts");
}
fn restrict_reference(f: Field, pre: &[([u8; 3], [E; 5])], x: E) -> [Poly; 4] {
    let mut out: [Poly; 4] = std::array::from_fn(|_| BTreeMap::new());
    for (m, coeff) in pre {
        let value = (0..5).fold([0; 4], |sum, j| {
            f.add(sum, f.mul(coeff[j], f.pow(x, j as u64)))
        });
        for i in 0..4 {
            if value[i] != 0 {
                out[i].insert(*m, value[i]);
            }
        }
    }
    out
}
fn restrict_exact(f: Field, b: E, h: &integer::ZPoly, x: E) -> [Poly; 4] {
    let mut out: [Poly; 4] = std::array::from_fn(|_| BTreeMap::new());
    for (&e, &c) in &h.0 {
        let scalar = c.rem_euclid(f.p as i128) as u64;
        let value = f.mul(
            f.scalar(scalar),
            f.mul(f.pow(x, e[3] as u64), f.pow(b, e[4] as u64)),
        );
        for i in 0..4 {
            let m = [e[0], e[1], e[2]];
            let v = (out[i].get(&m).copied().unwrap_or(0) + value[i]) % f.p;
            if v == 0 {
                out[i].remove(&m);
            } else {
                out[i].insert(m, v);
            }
        }
    }
    out
}
fn degrees(polys: &[Poly]) -> Vec<u8> {
    polys
        .iter()
        .map(|p| p.keys().map(|m| m.iter().sum()).max().unwrap_or(0))
        .collect()
}
fn triangularise(polys: &[Poly; 4], p: u64) -> [Poly; 4] {
    let mut ms: Vec<_> = polys.iter().flat_map(|v| v.keys().copied()).collect();
    ms.sort();
    ms.dedup();
    ms.sort_by_key(|m| {
        (
            std::cmp::Reverse(m.iter().sum::<u8>()),
            std::cmp::Reverse(*m),
        )
    });
    let mut rows: Vec<Vec<u64>> = polys
        .iter()
        .map(|v| ms.iter().map(|m| v.get(m).copied().unwrap_or(0)).collect())
        .collect();
    let mut rank = 0;
    for col in 0..ms.len() {
        let Some(pivot) = (rank..4).find(|&i| rows[i][col] != 0) else {
            continue;
        };
        rows.swap(rank, pivot);
        let inv = fpow(rows[rank][col], p - 2, p);
        for j in col..ms.len() {
            rows[rank][j] = rows[rank][j] * inv % p;
        }
        for i in 0..4 {
            if i != rank {
                let k = rows[i][col];
                for j in col..ms.len() {
                    rows[i][j] = (rows[i][j] + p - k * rows[rank][j] % p) % p;
                }
            }
        }
        rank += 1;
        if rank == 4 {
            break;
        }
    }
    std::array::from_fn(|i| {
        ms.iter()
            .zip(&rows[i])
            .filter_map(|(&m, &v)| (v != 0).then_some((m, v)))
            .collect()
    })
}
fn profile(path: &Path, h: &integer::ZPoly) -> Value {
    let bytes = std::fs::read(path).unwrap();
    let raw: Value = serde_json::from_slice(&bytes).unwrap();
    let p = raw["p"].as_u64().unwrap();
    let seed = raw["seed"].as_u64().unwrap();
    let f = Field::new(p);
    let c = Curve {
        f,
        b: [0, 0, 0, raw["b0"].as_u64().unwrap()],
    };
    let g = Point {
        x: element(&raw["generator_x"]),
        y: element(&raw["generator_y"]),
        inf: false,
    };
    let q = Point {
        x: element(&raw["target_x"]),
        y: element(&raw["target_y"]),
        inf: false,
    };
    let base: Vec<_> = raw["base_coordinates"]
        .as_array()
        .unwrap()
        .iter()
        .map(point)
        .collect();
    assert!(base.iter().chain([&g, &q]).all(|&pt| c.on_curve(pt)));
    let reference = interpolate(f, c.b);
    let mut counts: BTreeMap<String, u64> = BTreeMap::new();
    let mut attempted = 0u64;
    let mut infinity = 0u64;
    let mut representative = Vec::new();
    for (query, item) in raw["trace"].as_array().unwrap().iter().enumerate() {
        let r = c.add(
            c.mul(g, item["a"].as_u64().unwrap()),
            c.mul(q, item["b"].as_u64().unwrap()),
        );
        for (id, &factor) in base.iter().enumerate().filter(|(_, pt)| pt.x[2] != 0) {
            for sign in [-1, 1] {
                let peel = if sign == 1 { factor } else { c.neg(factor) };
                let u = c.add(r, c.neg(peel));
                if u.inf {
                    infinity += 1;
                    continue;
                }
                let exact = restrict_exact(f, c.b, h, u.x);
                let reference = restrict_reference(f, &reference, u.x);
                assert_eq!(
                    exact, reference,
                    "coefficient mismatch p={p}, seed={seed}, q={query}, peel={id}/{sign}"
                );
                let reduced = triangularise(&exact, p);
                let dg = degrees(&exact);
                let rd = degrees(&reduced);
                let key = format!("original={dg:?}; triangular={rd:?}");
                *counts.entry(key).or_default() += 1;
                attempted += 1;
                if representative.is_empty()
                    || (p == 73 && seed == 1 && query == 18 && id == 54 && sign == 1)
                {
                    let polys: Vec<_> = exact
                        .iter()
                        .map(|p| {
                            p.iter()
                                .map(|(e, v)| json!({"e":e,"c":v}))
                                .collect::<Vec<_>>()
                        })
                        .collect();
                    representative.push(json!({"query":query,"peel_id":id,"sign":sign,"peeled_x":u.x,"degrees":dg,"triangular_degrees":rd,"equations":polys}));
                }
            }
        }
    }
    json!({"status":"pass","p":p,"seed":seed,"queries":raw["trace"].as_array().unwrap().len(),
        "finite_peels":attempted,"infinity_peels":infinity,"coefficient_matches":attempted,
        "degree_profiles":counts,"representatives":representative,
        "input_sha256":hex::encode(hash::sha256::sha256(&bytes))})
}

fn scan_roots(polys: &[Poly; 4], p: u64) -> (Vec<Vec<u64>>, u64) {
    let mut muls = 0;
    let powers: Vec<[u64; 5]> = (0..p)
        .map(|x| {
            let mut v = [1; 5];
            for i in 1..5 {
                v[i] = v[i - 1] * x % p;
                muls += 1;
            }
            v
        })
        .collect();
    let mut roots = Vec::new();
    for a in 0..p {
        for b in 0..p {
            let coeffs: [[u64; 5]; 4] = std::array::from_fn(|i| {
                let mut out = [0; 5];
                for (e, &c) in &polys[i] {
                    let v = c * powers[a as usize][e[0] as usize] % p
                        * powers[b as usize][e[1] as usize]
                        % p;
                    muls += 2;
                    out[e[2] as usize] = (out[e[2] as usize] + v) % p;
                }
                out
            });
            for z in 0..p {
                let mut valid = true;
                for row in coeffs {
                    let mut value = row[4];
                    for j in (0..4).rev() {
                        value = (value * z + row[j]) % p;
                        muls += 1;
                    }
                    if value != 0 {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    roots.push(vec![a, b, z]);
                }
            }
        }
    }
    (roots, muls)
}

fn relation_set(
    roots: &[Vec<u64>],
    p: u64,
    raw: &Value,
    fixture: &Value,
) -> BTreeSet<Vec<(usize, i8)>> {
    let f = Field::new(p);
    let c = Curve {
        f,
        b: [0, 0, 0, raw["b0"].as_u64().unwrap()],
    };
    let g = Point {
        x: element(&raw["generator_x"]),
        y: element(&raw["generator_y"]),
        inf: false,
    };
    let q = Point {
        x: element(&raw["target_x"]),
        y: element(&raw["target_y"]),
        inf: false,
    };
    let base: Vec<_> = raw["base_coordinates"]
        .as_array()
        .unwrap()
        .iter()
        .map(point)
        .collect();
    let qi = fixture["query"].as_u64().unwrap() as usize;
    let item = &raw["trace"][qi];
    let residual = c.add(
        c.mul(g, item["a"].as_u64().unwrap()),
        c.mul(q, item["b"].as_u64().unwrap()),
    );
    let peel = fixture["peel_id"].as_u64().unwrap() as usize;
    let sign = fixture["sign"].as_i64().unwrap() as i8;
    let mut out = BTreeSet::new();
    for e in roots {
        let xs: Vec<_> = (0..p)
            .filter(|&x| (((x + p - e[0]) * x + e[1]) % p * x + p - e[2]) % p == 0)
            .collect();
        if xs.len() != 3 {
            continue;
        }
        let ids: Option<Vec<_>> = xs
            .iter()
            .map(|&x| base.iter().position(|pt| pt.x == f.scalar(x)))
            .collect();
        let Some(ids) = ids else {
            continue;
        };
        for mask in 0..8 {
            let mut sum = if sign == 1 {
                base[peel]
            } else {
                c.neg(base[peel])
            };
            let mut terms = vec![(peel, sign)];
            for (j, &id) in ids.iter().enumerate() {
                let s = if mask & (1 << j) == 0 { 1 } else { -1 };
                terms.push((id, s));
                sum = c.add(sum, if s == 1 { base[id] } else { c.neg(base[id]) });
            }
            if sum == residual {
                terms.sort();
                out.insert(terms);
            }
        }
    }
    out
}

fn expected_relations(
    raw: &Value,
    fixture: &Value,
    diagnosis: &Value,
) -> BTreeSet<Vec<(usize, i8)>> {
    let qi = fixture["query"].as_u64().unwrap() as usize;
    let id = fixture["peel_id"].as_u64().unwrap() as usize;
    let sign = fixture["sign"].as_i64().unwrap() as i8;
    let mut rows = raw["trace"][qi]["patterns"][0]["signed_relations"]
        .as_array()
        .unwrap()
        .clone();
    if raw["p"] == 73 && raw["seed"] == 1 && qi == 18 && id == 54 && sign == 1 {
        let r = diagnosis["runs"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["p"] == 73 && r["seed"] == 1)
            .unwrap();
        rows.extend(
            r["differences"][0]["missing_terms"]
                .as_array()
                .unwrap()
                .clone(),
        );
    }
    rows.iter()
        .map(|v| {
            v.as_array()
                .unwrap()
                .iter()
                .map(|t| {
                    (
                        t[0].as_u64().unwrap() as usize,
                        t[1].as_i64().unwrap() as i8,
                    )
                })
                .collect::<Vec<_>>()
        })
        .filter(|v| v.contains(&(id, sign)))
        .collect()
}

fn solve_fixtures(fixture_path: &Path, expected_sha: &str, root: &Path, out: &Path) {
    let bytes = std::fs::read(fixture_path).unwrap();
    assert_eq!(
        hex::encode(hash::sha256::sha256(&bytes)),
        expected_sha,
        "fixture SHA-256 mismatch before solving"
    );
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let profile: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(profile["status"], "pass");
    let diagnosis: Value = serde_json::from_slice(
        &std::fs::read(root.join("quartic_twist_k1_missing_diagnostic_v1.json")).unwrap(),
    )
    .unwrap();
    let mut results = Vec::new();
    for run in profile["runs"].as_array().unwrap() {
        let p = run["p"].as_u64().unwrap();
        let seed = run["seed"].as_u64().unwrap();
        let raw: Value = serde_json::from_slice(
            &std::fs::read(root.join(format!("quartic_twist_k1_scale_p{p}_seed{seed}_v1.json")))
                .unwrap(),
        )
        .unwrap();
        for fixture in run["representatives"].as_array().unwrap() {
            let equations: [Poly; 4] = std::array::from_fn(|i| {
                fixture["equations"][i]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|term| {
                        let exps = std::array::from_fn(|j| term["e"][j].as_u64().unwrap() as u8);
                        (exps, term["c"].as_u64().unwrap())
                    })
                    .collect()
            });
            let (reference, scan_muls) = scan_roots(&equations, p);
            let input: Vec<_> = equations
                .iter()
                .map(|poly| {
                    f4_fp::normalise(
                        &poly
                            .iter()
                            .map(|(e, &c)| (e.iter().map(|&x| x as u32).collect(), c))
                            .collect::<Vec<_>>(),
                        p,
                        f4_fp::Ordering::Grevlex,
                    )
                })
                .collect();
            let opts = f4_fp::F4Options::new(f4_fp::Ordering::Grevlex, 12)
                .with_budget(Duration::from_secs(1200));
            let solved = f4_fp::solve(&input, 3, p, &opts);
            let (mut roots, verdict) = match solved.verdict {
                f4_fp::Verdict::Solutions(s) => (s, "solutions"),
                f4_fp::Verdict::Inconsistent => (Vec::new(), "inconsistent"),
                f4_fp::Verdict::Undetermined => (Vec::new(), "undetermined"),
            };
            roots.sort();
            roots.dedup();
            let equal = verdict != "undetermined" && roots == reference;
            let relations = relation_set(&roots, p, &raw, fixture);
            let expected = expected_relations(&raw, fixture, &diagnosis);
            let result = json!({"p":p,"seed":seed,"query":fixture["query"],"peel_id":fixture["peel_id"],"sign":fixture["sign"],
                "status":if equal&&relations==expected{"pass"}else{"partial"},"verdict":verdict,"full_field_assignments":p.pow(3),
                "reference_roots":reference,"f4_roots":roots,"root_sets_equal":equal,"f4_row_reduction_fp_muls":solved.field_ops,
                "reference_scan_fp_muls":scan_muls,"solving_degree":solved.solving_degree,"max_rows":solved.max_rows,"max_cols":solved.max_cols,
                "signed_relations":relations,"expected_signed_relations":expected,"group_sets_equal":relations==expected});
            let name = format!(
                "p{p}-seed{seed}-q{}-peel{}-s{}.json",
                fixture["query"], fixture["peel_id"], fixture["sign"]
            );
            std::fs::write(out.join(name), serde_json::to_vec_pretty(&result).unwrap()).unwrap();
            results.push(result);
        }
    }
    assert_eq!(results.len(), 8);
    let pass = results.iter().all(|r| r["status"] == "pass");
    let output = json!({"schema":"quartic-twist-s4-f4-completeness/v1","status":if pass{"pass"}else{"partial"},"fixtures_sha256":expected_sha,"cases":results});
    std::fs::write(
        out.join("summary.json"),
        serde_json::to_vec_pretty(&output).unwrap(),
    )
    .unwrap();
    assert!(pass, "F4 completeness discrepancy retained in summary.json");
    println!("all eight exact field-root and signed-group sets match");
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 6 && args[1] == "--joint-five-point-replay" {
        let pass = image::joint::five_point_replay(
            Path::new(&args[2]),
            &args[3],
            Path::new(&args[4]),
            Path::new(&args[5]),
        );
        std::process::exit(if pass { 0 } else { 2 });
    }
    if args.len() == 4 && args[1] == "--joint-five-point-summary" {
        let pass = image::joint::five_point_summary(Path::new(&args[2]), Path::new(&args[3]));
        std::process::exit(if pass { 0 } else { 2 });
    }
    if args.len() == 6 && args[1] == "--joint-five-zero" {
        let pass = image::joint::five_zero(
            Path::new(&args[2]),
            &args[3],
            Path::new(&args[4]),
            Path::new(&args[5]),
        );
        std::process::exit(if pass { 0 } else { 2 });
    }
    if args.len() == 4 && args[1] == "--joint-five-zero-summary" {
        let pass = image::joint::five_zero_summary(Path::new(&args[2]), Path::new(&args[3]));
        std::process::exit(if pass { 0 } else { 2 });
    }
    if args.len() == 8 && args[1] == "--joint-fp-pruning-audit" {
        let pass = image::joint::fp_pruning_audit(
            Path::new(&args[2]),
            &args[3],
            Path::new(&args[4]),
            Path::new(&args[5]),
            Path::new(&args[6]),
            Path::new(&args[7]),
        );
        std::process::exit(if pass { 0 } else { 2 });
    }
    if args.len() == 4 && args[1] == "--joint-fp-pruning-summary" {
        let pass = image::joint::fp_pruning_summary(Path::new(&args[2]), Path::new(&args[3]));
        std::process::exit(if pass { 0 } else { 2 });
    }
    if args.len() == 4 && args[1] == "--joint-variant-report" {
        let pass =
            image::joint::variant_construction_report(Path::new(&args[2]), Path::new(&args[3]));
        std::process::exit(if pass { 0 } else { 2 });
    }
    if args.len() == 10 && args[1] == "--joint-external-audit" {
        let pass = image::joint::external_audit(
            Path::new(&args[2]),
            &args[3],
            Path::new(&args[4]),
            &args[5],
            Path::new(&args[6]),
            &args[7],
            Path::new(&args[8]),
            Path::new(&args[9]),
        );
        std::process::exit(if pass { 0 } else { 2 });
    }
    if args.len() == 5 && args[1] == "--joint-zero-identity-audit" {
        let pass = image::joint::zero_identity_audit(
            Path::new(&args[2]),
            Path::new(&args[3]),
            Path::new(&args[4]),
        );
        std::process::exit(if pass { 0 } else { 2 });
    }
    if args.len() == 5 && args[1] == "--joint-zero-oracle" {
        image::joint::zero_oracle(Path::new(&args[2]), &args[3], Path::new(&args[4]));
        return;
    }
    if args.len() == 5 && args[1] == "--joint-population-oracle" {
        image::joint::population_oracle(Path::new(&args[2]), &args[3], Path::new(&args[4]));
        return;
    }
    if args.len() == 4 && args[1] == "--joint-population-summary" {
        let pass = image::joint::population_summary(Path::new(&args[2]), Path::new(&args[3]));
        std::process::exit(if pass { 0 } else { 2 });
    }
    if args.len() == 7 && args[1] == "--joint-variant-export" {
        image::joint::export_variant(
            Path::new(&args[2]),
            &args[3],
            &args[4],
            args[5].parse().unwrap(),
            Path::new(&args[6]),
        );
        return;
    }
    if args.len() == 4 && args[1] == "--joint-construction-report" {
        let pass = image::joint::construction_report(Path::new(&args[2]), Path::new(&args[3]));
        std::process::exit(if pass { 0 } else { 2 });
    }
    if args.len() == 3 && args[1] == "--joint-orbit-proofs" {
        image::joint::orbit_proofs(Path::new(&args[2]));
        return;
    }
    if args.len() == 7 && args[1] == "--joint-orbit-solve" {
        let pass = image::joint::solve_orbit_query(
            Path::new(&args[2]),
            &args[3],
            Path::new(&args[4]),
            &args[5],
            Path::new(&args[6]),
        );
        std::process::exit(if pass { 0 } else { 2 });
    }
    if args.len() == 7 && args[1] == "--joint-orbit-export" {
        image::joint::export_orbits(
            Path::new(&args[2]),
            &args[3],
            &args[4],
            args[5].parse().unwrap(),
            Path::new(&args[6]),
        );
        return;
    }
    if args.len() == 7 && args[1] == "--joint-solve" {
        let pass = image::joint::solve_query(
            Path::new(&args[2]),
            &args[3],
            Path::new(&args[4]),
            &args[5],
            Path::new(&args[6]),
        );
        std::process::exit(if pass { 0 } else { 2 });
    }
    if args.len() == 7 && args[1] == "--joint-check" {
        let pass = image::joint::check_query(
            Path::new(&args[2]),
            &args[3],
            Path::new(&args[4]),
            Path::new(&args[5]),
            Path::new(&args[6]),
        );
        std::process::exit(if pass { 0 } else { 2 });
    }
    if args.len() == 4 && args[1] == "--joint-prepare" {
        image::joint::prepare(Path::new(&args[2]), Path::new(&args[3]));
        return;
    }
    if args.len() == 6 && args[1] == "--joint-workload" {
        image::joint::workload(
            Path::new(&args[2]),
            Path::new(&args[3]),
            Path::new(&args[4]),
            Path::new(&args[5]),
        );
        return;
    }
    if args.len() == 7 && args[1] == "--joint-export" {
        image::joint::export(
            Path::new(&args[2]),
            &args[3],
            &args[4],
            args[5].parse().unwrap(),
            Path::new(&args[6]),
        );
        return;
    }
    if args.len() == 6 && args[1] == "--joint-audit" {
        image::joint::audit(
            Path::new(&args[2]),
            Path::new(&args[3]),
            Path::new(&args[4]),
            Path::new(&args[5]),
        );
        return;
    }
    if args.len() == 4 && args[1] == "--image-cold-assemble" {
        image::cold_catalog::assemble(Path::new(&args[2]), Path::new(&args[3]));
        return;
    }
    if args.len() == 5 && args[1] == "--image-external-report" {
        image::external::summarize(
            Path::new(&args[2]),
            Path::new(&args[3]),
            Path::new(&args[4]),
        );
        return;
    }
    if args.len() == 4 && args[1] == "--image-job-reports" {
        image::report::job_reports(Path::new(&args[2]), Path::new(&args[3]));
        return;
    }
    if args.len() == 5 && args[1] == "--image-external-benchmark" {
        image::external::benchmark(
            Path::new(&args[2]),
            Path::new(&args[3]),
            Path::new(&args[4]),
        );
        return;
    }
    if args.len() == 7 && args[1] == "--image-audit-mutations" {
        image::audit_mutations(
            Path::new(&args[2]),
            &args[3],
            Path::new(&args[4]),
            Path::new(&args[5]),
            Path::new(&args[6]),
        );
        return;
    }
    if args.len() == 5 && args[1] == "--image-baseline-parity" {
        image::baseline_parity(
            Path::new(&args[2]),
            Path::new(&args[3]),
            Path::new(&args[4]),
        );
        return;
    }
    if args.len() == 5 && args[1] == "--image-external-export" {
        image::external::export(Path::new(&args[2]), &args[3], Path::new(&args[4]));
        return;
    }
    if args.len() == 4 && args[1] == "--image-native-gb" {
        image::external::native(Path::new(&args[2]), Path::new(&args[3]));
        return;
    }
    if args.len() == 11 && args[1] == "--image-report" {
        image::report::summarize(&args[2..]);
        return;
    }
    if args.len() == 8 && args[1] == "--image-cold" {
        image::cold::execute(
            Path::new(&args[2]),
            &args[3],
            args[4].parse().unwrap(),
            args[5].parse().unwrap(),
            &args[6],
            Path::new(&args[7]),
        );
        return;
    }
    if args.len() == 6 && args[1] == "--image-cold-check" {
        image::cold::check(
            Path::new(&args[2]),
            Path::new(&args[3]),
            Path::new(&args[4]),
            Path::new(&args[5]),
        );
        return;
    }
    if args.len() == 3 && args[1] == "--image-identity" {
        image_identity::write(Path::new(&args[2]));
        return;
    }
    if args.len() == 7 && (args[1] == "--image-census" || args[1] == "--image-collect") {
        image::run(
            &args[1],
            Path::new(&args[2]),
            &args[3],
            args[4].parse().unwrap(),
            args[5].parse().unwrap(),
            Path::new(&args[6]),
        );
        return;
    }
    if args.len() == 8 && args[1] == "--image-collect-arm" {
        image::run_arm(
            Path::new(&args[2]),
            &args[3],
            args[4].parse().unwrap(),
            args[5].parse().unwrap(),
            &args[6],
            Path::new(&args[7]),
        );
        return;
    }
    if args.len() == 8 && args[1] == "--image-reference-arm" {
        image::reference_arm(
            Path::new(&args[2]),
            &args[3],
            args[4].parse().unwrap(),
            args[5].parse().unwrap(),
            &args[6],
            Path::new(&args[7]),
        );
        return;
    }
    if args.len() == 6 && args[1] == "--image-collect-shard" {
        image::run_shard(
            Path::new(&args[2]),
            &args[3],
            args[4].parse().unwrap(),
            Path::new(&args[5]),
        );
        return;
    }
    if args.len() == 7 && args[1] == "--image-audit" {
        image::audit(
            Path::new(&args[2]),
            &args[3],
            Path::new(&args[4]),
            Path::new(&args[5]),
            Path::new(&args[6]),
        );
        return;
    }
    if args.len() == 3 && args[1] == "--certificate" {
        certificate_probe::write_record(Path::new(&args[2]));
        return;
    }
    if args.len() == 6 && args[1] == "--prefixes" {
        stream::run(
            Path::new(&args[2]),
            &args[3],
            Path::new(&args[4]),
            Path::new(&args[5]),
        );
        return;
    }
    if args.len() == 6 && args[1] == "--fixtures" {
        solve_fixtures(
            Path::new(&args[2]),
            &args[3],
            Path::new(&args[4]),
            Path::new(&args[5]),
        );
        return;
    }
    if args.len() == 4 && args[1] == "--corrupt-fixture" {
        let mut v: Value = serde_json::from_slice(&std::fs::read(&args[2]).unwrap()).unwrap();
        let c = v["runs"][0]["representatives"][0]["equations"][0][0]["c"]
            .as_u64()
            .unwrap();
        v["runs"][0]["representatives"][0]["equations"][0][0]["c"] = json!((c + 1) % 37);
        std::fs::write(&args[3], serde_json::to_vec_pretty(&v).unwrap()).unwrap();
        return;
    }
    assert_eq!(args.len(), 3, "probe STUDY_ROOT OUTPUT_DIR");
    let root = Path::new(&args[1]);
    let out = Path::new(&args[2]);
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    certificate_probe::write_record(&out.join("identity"));
    let (_, h) = integer::derive();
    let mut runs = Vec::new();
    for (p, seeds) in [(37, 1), (73, 3), (97, 3)] {
        for seed in 0..seeds {
            runs.push(profile(
                &root.join(format!("quartic_twist_k1_scale_p{p}_seed{seed}_v1.json")),
                &h,
            ));
        }
    }
    let total: u64 = runs
        .iter()
        .map(|v| v["finite_peels"].as_u64().unwrap())
        .sum();
    let result = json!({"schema":"quartic-twist-s4-native-profile/v1","status":"pass","runs":runs,"finite_peels":total,"all_interpolated_coefficients_equal":true});
    std::fs::write(
        out.join("profile.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    println!("profiled {total} peeled S4 systems with exact interpolated coefficient equality");
}
