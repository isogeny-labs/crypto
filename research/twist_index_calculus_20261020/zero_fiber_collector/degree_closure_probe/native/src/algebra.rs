//! Exact sufficient rejection certificates. No F4, group or scalar recovery.
#![allow(dead_code)]
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::time::{Duration, Instant};

type M = [u8; 3];
type Poly = BTreeMap<M, u64>;
type R<T> = Result<T, String>;
const PRIMES: [u64; 7] = [37, 73, 97, 157, 229, 241, 337];
const WORKLOAD_SHA: &str = "47604fa481c5c26ef17e9e9a41494461556c1b9356e20e9652a499926a499b77";

fn need(ok: bool, why: &str) -> R<()> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn num(v: &Value) -> R<u64> {
    v.as_u64()
        .ok_or_else(|| format!("missing unsigned integer: {v}"))
}
fn arr(v: &Value) -> R<&Vec<Value>> {
    v.as_array().ok_or_else(|| "missing array".into())
}
fn txt(v: &Value) -> R<&str> {
    v.as_str().ok_or_else(|| "missing string".into())
}
fn read(p: &Path) -> R<Value> {
    serde_json::from_slice(&std::fs::read(p).map_err(|e| format!("{}: {e}", p.display()))?)
        .map_err(|e| format!("{}: {e}", p.display()))
}
fn sha(p: &Path) -> R<String> {
    Ok(hex::encode(Sha256::digest(
        std::fs::read(p).map_err(|e| e.to_string())?,
    )))
}
fn write_new(p: &Path, v: &Value) -> R<()> {
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(p)
        .map_err(|e| e.to_string())?;
    f.write_all(&serde_json::to_vec(v).unwrap())
        .map_err(|e| e.to_string())?;
    f.write_all(b"\n").map_err(|e| e.to_string())?;
    f.sync_all().map_err(|e| e.to_string())
}
fn parse_poly(v: &Value, p: u64) -> R<Poly> {
    let terms: Vec<(M, u64)> = serde_json::from_value(v.clone()).map_err(|e| e.to_string())?;
    let mut out = Poly::new();
    for (m, c) in terms {
        need(c > 0 && c < p, "noncanonical coefficient")?;
        need(out.insert(m, c).is_none(), "duplicate monomial")?;
    }
    Ok(out)
}
fn poly_json(p: &Poly) -> Value {
    json!(p.iter().map(|(&m, &c)| (m, c)).collect::<Vec<_>>())
}
fn degree(m: &M) -> u16 {
    m.iter().map(|&x| x as u16).sum()
}
fn poly_degree(p: &Poly) -> Option<u16> {
    p.keys().map(degree).max()
}
fn monomials(d: u8) -> Vec<M> {
    let mut out = Vec::new();
    for i in 0..=d {
        for j in 0..=d - i {
            for k in 0..=d - i - j {
                out.push([i, j, k]);
            }
        }
    }
    out.sort_by(|a, b| degree(b).cmp(&degree(a)).then_with(|| b.cmp(a)));
    out
}

#[derive(Clone, Copy, Default)]
struct Work {
    muls: u64,
    adds: u64,
    invs: u64,
}
impl Work {
    fn json(self) -> Value {
        json!({"modular_multiplications":self.muls,"modular_additions":self.adds,"inversion_calls":self.invs})
    }
    fn minus(self, b: Self) -> Self {
        Self {
            muls: self.muls - b.muls,
            adds: self.adds - b.adds,
            invs: self.invs - b.invs,
        }
    }
}
struct Fp {
    p: u64,
    w: Work,
}
impl Fp {
    fn new(p: u64) -> Self {
        assert!(p >= 2 && p <= u32::MAX as u64);
        Self {
            p,
            w: Work::default(),
        }
    }
    fn add(&mut self, a: u64, b: u64) -> u64 {
        self.w.adds += 1;
        (a + b) % self.p
    }
    fn sub(&mut self, a: u64, b: u64) -> u64 {
        self.w.adds += 1;
        (a + self.p - b) % self.p
    }
    fn mul(&mut self, a: u64, b: u64) -> u64 {
        self.w.muls += 1;
        a * b % self.p
    }
    fn inv(&mut self, a: u64) -> u64 {
        assert!(a > 0);
        self.w.invs += 1;
        let (mut x, mut n, mut y) = (a, self.p - 2, 1);
        while n > 0 {
            if n & 1 != 0 {
                y = self.mul(y, x);
            }
            n >>= 1;
            if n > 0 {
                x = self.mul(x, x);
            }
        }
        y
    }
}
fn insert(p: &mut Poly, m: M, c: u64, f: &mut Fp) {
    let v = f.add(p.get(&m).copied().unwrap_or(0), c);
    if v == 0 {
        p.remove(&m);
    } else {
        p.insert(m, v);
    }
}
fn product(a: &Poly, b: &Poly, f: &mut Fp) -> Poly {
    let mut out = Poly::new();
    for (&x, &c) in a {
        for (&y, &d) in b {
            let m = std::array::from_fn(|i| x[i].checked_add(y[i]).unwrap());
            let v = f.mul(c, d);
            insert(&mut out, m, v, f);
        }
    }
    out
}
fn q(a: u64, f: &mut Fp) -> Poly {
    let a2 = f.mul(a, a);
    let a3 = f.mul(a2, a);
    let na2 = f.sub(0, a2);
    let neg1 = f.sub(0, 1);
    [
        ([0, 0, 0], a3),
        ([1, 0, 0], na2),
        ([0, 1, 0], a),
        ([0, 0, 1], neg1),
    ]
    .into_iter()
    .filter(|(_, c)| *c != 0)
    .collect()
}
fn powers(a: u64, f: &mut Fp) -> Vec<Poly> {
    let v = q(a, f);
    let v2 = product(&v, &v, f);
    let v3 = product(&v2, &v, f);
    vec![v, v2, v3]
}

#[derive(Clone)]
struct Certificate {
    d: u8,
    k: u8,
    multipliers: Vec<Poly>,
}
impl Certificate {
    fn json(&self) -> Value {
        json!({"degree_cap":self.d,"power":self.k,
        "multipliers":self.multipliers.iter().map(poly_json).collect::<Vec<_>>()})
    }
    fn parse(v: &Value, p: u64) -> R<Self> {
        let d = num(&v["degree_cap"])?;
        let k = num(&v["power"])?;
        need(
            (1..=16).contains(&d) && (1..=3).contains(&k),
            "certificate limit",
        )?;
        let multipliers = arr(&v["multipliers"])?
            .iter()
            .map(|v| parse_poly(v, p))
            .collect::<R<Vec<_>>>()?;
        need(multipliers.len() == 4, "certificate arity")?;
        Ok(Self {
            d: d as u8,
            k: k as u8,
            multipliers,
        })
    }
}
// This verifier expands sparse polynomials; it never uses the dense basis or
// the representation vectors that constructed the certificate.
fn verify(polys: &[Poly], a: u64, c: &Certificate, f: &mut Fp) -> R<()> {
    need(
        polys.len() == 4 && c.multipliers.len() == 4,
        "certificate arity",
    )?;
    let mut out = Poly::new();
    for (h, g) in c.multipliers.iter().zip(polys) {
        if let (Some(hd), Some(gd)) = (poly_degree(h), poly_degree(g)) {
            need(hd + gd <= c.d as u16, "certificate degree exceeds cap")?;
        }
        for (m, v) in product(h, g, f) {
            insert(&mut out, m, v, f);
        }
    }
    let expected = powers(a, f)[c.k as usize - 1].clone();
    need(out == expected, "false certificate identity")
}

// Find an exact row-span representation. No original roots or F4 results are
// available here. The high-degree columns are eliminated first.
fn search(
    polys: &[Poly],
    a: u64,
    f: &mut Fp,
    deadline: Instant,
    maximum_degree: u8,
    note: &mut impl FnMut(Value) -> R<()>,
) -> R<(Option<Certificate>, Vec<Value>)> {
    let targets = powers(a, f);
    let mut attempts = Vec::new();
    for d in 4..=maximum_degree {
        need(Instant::now() < deadline, "cooperative deadline")?;
        let before = f.w;
        let mons = monomials(d);
        let index: BTreeMap<_, _> = mons.iter().enumerate().map(|(i, &m)| (m, i)).collect();
        let mut generators = Vec::new();
        for (pi, poly) in polys.iter().enumerate() {
            if let Some(gd) = poly_degree(poly) {
                if gd <= d as u16 {
                    for m in monomials(d - gd as u8) {
                        generators.push((pi, m));
                    }
                }
            }
        }
        let rows = generators.len();
        let cols = mons.len();
        note(
            json!({"phase":"degree-start","degree":d,"rows":rows,"columns":cols,"work_so_far":f.w.json()}),
        )?;
        let mut basis: Vec<Option<(Vec<u64>, Vec<u64>)>> = vec![None; cols];
        let mut rank = 0;
        for (ri, &(pi, shift)) in generators.iter().enumerate() {
            if Instant::now() >= deadline {
                note(
                    json!({"phase":"degree-timeout","degree":d,"rows":rows,"columns":cols,"rank":rank,"rows_processed":ri,"work_so_far":f.w.json()}),
                )?;
                return Err("cooperative deadline".into());
            }
            let mut row = vec![0; cols];
            let mut representation = vec![0; rows];
            representation[ri] = 1;
            for (&m, &c) in &polys[pi] {
                let shifted = std::array::from_fn(|i| m[i] + shift[i]);
                row[index[&shifted]] = c;
            }
            for col in 0..cols {
                let scalar = row[col];
                if scalar == 0 {
                    continue;
                }
                if let Some((b, br)) = &basis[col] {
                    for j in col..cols {
                        if b[j] != 0 {
                            let v = f.mul(scalar, b[j]);
                            row[j] = f.sub(row[j], v);
                        }
                    }
                    for j in 0..rows {
                        if br[j] != 0 {
                            let v = f.mul(scalar, br[j]);
                            representation[j] = f.sub(representation[j], v);
                        }
                    }
                } else {
                    let inverse = f.inv(scalar);
                    for v in &mut row[col..] {
                        if *v != 0 {
                            *v = f.mul(*v, inverse);
                        }
                    }
                    for v in &mut representation {
                        if *v != 0 {
                            *v = f.mul(*v, inverse);
                        }
                    }
                    basis[col] = Some((row, representation));
                    rank += 1;
                    break;
                }
            }
        }
        for (ki, target) in targets.iter().enumerate() {
            let mut row = vec![0; cols];
            let mut weights = vec![0; rows];
            for (&m, &v) in target {
                row[index[&m]] = v;
            }
            for col in 0..cols {
                let scalar = row[col];
                if scalar == 0 {
                    continue;
                }
                if let Some((b, br)) = &basis[col] {
                    for j in col..cols {
                        if b[j] != 0 {
                            let v = f.mul(scalar, b[j]);
                            row[j] = f.sub(row[j], v);
                        }
                    }
                    for j in 0..rows {
                        if br[j] != 0 {
                            let v = f.mul(scalar, br[j]);
                            weights[j] = f.add(weights[j], v);
                        }
                    }
                }
            }
            if row.iter().all(|&v| v == 0) {
                let mut multipliers = vec![Poly::new(); 4];
                for (&(pi, m), &v) in generators.iter().zip(&weights) {
                    if v != 0 {
                        insert(&mut multipliers[pi], m, v, f);
                    }
                }
                attempts.push(json!({"degree":d,"rows":rows,"columns":cols,"rank":rank,"found_power":ki+1,"work":f.w.minus(before).json()}));
                note(json!({"phase":"degree-completed","attempt":attempts.last().unwrap()}))?;
                return Ok((
                    Some(Certificate {
                        d,
                        k: ki as u8 + 1,
                        multipliers,
                    }),
                    attempts,
                ));
            }
        }
        attempts.push(json!({"degree":d,"rows":rows,"columns":cols,"rank":rank,"found_power":Value::Null,"work":f.w.minus(before).json()}));
        note(json!({"phase":"degree-completed","attempt":attempts.last().unwrap()}))?;
    }
    Ok((None, attempts))
}

fn polynomial_value(p: &Poly, e: [u64; 3], modulus: u64) -> u64 {
    // Audit-only direct evaluation, with u128 modular products.
    let mut out = 0u128;
    for (m, &c) in p {
        let mut v = c as u128;
        for i in 0..3 {
            for _ in 0..m[i] {
                v = v * e[i] as u128 % modulus as u128;
            }
        }
        out = (out + v) % modulus as u128;
    }
    out as u64
}

#[path = "probe.rs"]
mod probe;
pub fn entry() {
    probe::entry();
}
