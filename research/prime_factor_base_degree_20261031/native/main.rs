//! Ordinary bivariate F_p Macaulay certificates; no Boolean quotient/reduction.
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
mod pilot;

type Mono = (u32, u32);
type Poly = BTreeMap<Mono, u64>;
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Term {
    x: u32,
    y: u32,
    c: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct System {
    p: u64,
    max_degree: u32,
    polynomials: Vec<Vec<Term>>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Recipe {
    generator: usize,
    x: u32,
    y: u32,
    c: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Dual {
    degree: u32,
    values: Vec<Term>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Profile {
    degree: u32,
    rows: usize,
    columns: usize,
    rank: usize,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Certificate {
    schema: String,
    input_sha256: String,
    refutation_degree: Option<u32>,
    upper: Vec<Recipe>,
    lower: Option<Dual>,
    profiles: Vec<Profile>,
}
fn add(a: u64, b: u64, p: u64) -> u64 {
    (a + b) % p
}
fn mul(a: u64, b: u64, p: u64) -> u64 {
    a * b % p
}
fn neg(a: u64, p: u64) -> u64 {
    (p - a) % p
}
fn pow(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut out = 1;
    while n > 0 {
        if n & 1 != 0 {
            out = mul(out, a, p);
        }
        a = mul(a, a, p);
        n >>= 1;
    }
    out
}
fn is_prime(p: u64) -> bool {
    p >= 2
        && (2..)
            .take_while(|d| d * d <= p)
            .all(|d| !p.is_multiple_of(d))
}
fn degree(poly: &[Term]) -> u32 {
    poly.iter().map(|t| t.x + t.y).max().unwrap_or(0)
}
fn validate(s: &System) -> Result<(), String> {
    if !(5..=65521).contains(&s.p)
        || !is_prime(s.p)
        || s.max_degree > 24
        || s.polynomials.len() > 64
    {
        return Err("unsupported prime/degree/generator count".into());
    }
    for poly in &s.polynomials {
        if poly
            .iter()
            .any(|t| t.x > 24 || t.y > 24 || t.x + t.y > 24 || t.c == 0 || t.c >= s.p)
            || poly
                .windows(2)
                .any(|w| (w[0].x, w[0].y) >= (w[1].x, w[1].y))
        {
            return Err("noncanonical term list or exponent/coefficient outside cap".into());
        }
    }
    Ok(())
}
fn monos(d: u32) -> Vec<Mono> {
    (0..=d)
        .rev()
        .flat_map(|n| (0..=n).rev().map(move |x| (x, n - x)))
        .collect()
}
fn rows(s: &System, d: u32) -> Result<Vec<Recipe>, String> {
    let mut out = Vec::new();
    for (i, f) in s.polynomials.iter().enumerate() {
        if f.is_empty() || degree(f) > d {
            continue;
        }
        for (x, y) in monos(d - degree(f)) {
            out.push(Recipe {
                generator: i,
                x,
                y,
                c: 1,
            });
            if out.len() > 4096 {
                return Err("UNKNOWN: row cap exceeded".into());
            }
        }
    }
    Ok(out)
}
fn subtract_multiple(a: &mut [u64], b: &[u64], c: u64, p: u64) {
    for (u, v) in a.iter_mut().zip(b) {
        *u = add(*u, neg(mul(c, *v, p), p), p);
    }
}
struct Matrix {
    profile: Profile,
    upper: Option<Vec<Recipe>>,
    dual: Option<Dual>,
}
fn matrix(s: &System, d: u32) -> Result<Matrix, String> {
    let cols = monos(d);
    let positions: BTreeMap<Mono, usize> = cols.iter().enumerate().map(|(i, &m)| (m, i)).collect();
    let recipes = rows(s, d)?;
    type Pivot = Option<(Vec<u64>, Vec<u64>)>;
    let mut pivots: Vec<Pivot> = vec![None; cols.len()];
    let mut rank = 0;
    for (i, r) in recipes.iter().enumerate() {
        let mut row = vec![0; cols.len()];
        for t in &s.polynomials[r.generator] {
            row[positions[&(t.x + r.x, t.y + r.y)]] = t.c;
        }
        let mut history = vec![0; recipes.len()];
        history[i] = 1;
        for (j, pivot) in pivots.iter_mut().enumerate() {
            if row[j] == 0 {
                continue;
            }
            if let Some((base, previous)) = pivot {
                let c = row[j];
                subtract_multiple(&mut row, base, c, s.p);
                subtract_multiple(&mut history, previous, c, s.p);
            } else {
                let inverse = pow(row[j], s.p - 2, s.p);
                for v in &mut row {
                    *v = mul(*v, inverse, s.p);
                }
                for v in &mut history {
                    *v = mul(*v, inverse, s.p);
                }
                *pivot = Some((row, history));
                rank += 1;
                break;
            }
        }
    }
    let constant = cols.len() - 1;
    let upper = pivots[constant].as_ref().map(|(_, history)| {
        recipes
            .iter()
            .zip(history)
            .filter(|(_, c)| **c != 0)
            .map(|(r, &c)| Recipe { c, ..r.clone() })
            .collect()
    });
    let dual = if upper.is_none() {
        let mut values = vec![0; cols.len()];
        values[constant] = 1;
        for j in (0..cols.len()).rev() {
            if let Some((row, _)) = &pivots[j] {
                values[j] = neg(
                    ((j + 1)..cols.len())
                        .fold(0, |sum, k| add(sum, mul(row[k], values[k], s.p), s.p)),
                    s.p,
                );
            }
        }
        Some(Dual {
            degree: d,
            values: cols
                .iter()
                .zip(values)
                .filter(|(_, c)| *c != 0)
                .map(|(&(x, y), c)| Term { x, y, c })
                .collect(),
        })
    } else {
        None
    };
    Ok(Matrix {
        profile: Profile {
            degree: d,
            rows: recipes.len(),
            columns: cols.len(),
            rank,
        },
        upper,
        dual,
    })
}
fn certify(s: &System, hash: String) -> Result<Certificate, String> {
    validate(s)?;
    let mut profiles = Vec::new();
    let mut lower = None;
    for d in 0..=s.max_degree {
        let m = matrix(s, d)?;
        profiles.push(m.profile);
        if let Some(upper) = m.upper {
            return Ok(Certificate {
                schema: "prime-bivariate-macaulay/v1".into(),
                input_sha256: hash,
                refutation_degree: Some(d),
                upper,
                lower,
                profiles,
            });
        }
        lower = m.dual;
    }
    Ok(Certificate {
        schema: "prime-bivariate-macaulay/v1".into(),
        input_sha256: hash,
        refutation_degree: None,
        upper: vec![],
        lower,
        profiles,
    })
}
fn accumulate(poly: &mut Poly, m: Mono, c: u64, p: u64) {
    let value = add(poly.get(&m).copied().unwrap_or(0), c, p);
    if value == 0 {
        poly.remove(&m);
    } else {
        poly.insert(m, value);
    }
}
/// Independent polynomial replay of the upper and dual identities.
fn verify(s: &System, hash: &str, cert: &Certificate) -> Result<(), String> {
    validate(s)?;
    if cert.schema != "prime-bivariate-macaulay/v1" || cert.input_sha256 != hash {
        return Err("schema/input binding mismatch".into());
    }
    if let Some(d) = cert.refutation_degree {
        if d > s.max_degree {
            return Err("degree exceeds input cap".into());
        }
        let mut polynomial = Poly::new();
        for r in &cert.upper {
            let f = s
                .polynomials
                .get(r.generator)
                .ok_or("bad generator index")?;
            if r.x > d || r.y > d || r.x + r.y + degree(f) > d || r.c == 0 || r.c >= s.p {
                return Err("invalid multiplier".into());
            }
            for t in f {
                accumulate(
                    &mut polynomial,
                    (t.x + r.x, t.y + r.y),
                    mul(t.c, r.c, s.p),
                    s.p,
                );
            }
        }
        if polynomial != BTreeMap::from([((0, 0), 1)]) {
            return Err("upper identity is not 1".into());
        }
        if d == 0 && cert.lower.is_some() {
            return Err("unexpected lower witness at zero".into());
        }
        if d > 0 && cert.lower.as_ref().map(|v| v.degree) != Some(d - 1) {
            return Err("missing adjacent lower witness".into());
        }
    } else if !cert.upper.is_empty() || cert.lower.as_ref().map(|v| v.degree) != Some(s.max_degree)
    {
        return Err("invalid capped result".into());
    }
    if let Some(dual) = &cert.lower {
        let mut values = Poly::new();
        for t in &dual.values {
            if t.x > dual.degree
                || t.y > dual.degree
                || t.x + t.y > dual.degree
                || t.c == 0
                || t.c >= s.p
                || values.insert((t.x, t.y), t.c).is_some()
            {
                return Err("invalid dual-vector coordinates".into());
            }
        }
        if values.get(&(0, 0)) != Some(&1) {
            return Err("dual does not separate constant".into());
        }
        for r in rows(s, dual.degree)? {
            let dot = s.polynomials[r.generator].iter().fold(0, |sum, t| {
                add(
                    sum,
                    mul(
                        t.c,
                        values.get(&(t.x + r.x, t.y + r.y)).copied().unwrap_or(0),
                        s.p,
                    ),
                    s.p,
                )
            });
            if dot != 0 {
                return Err("dual fails a lower-degree row".into());
            }
        }
    }
    let last = cert.refutation_degree.unwrap_or(s.max_degree);
    if cert.profiles.len() != last as usize + 1 {
        return Err("incomplete degree trace".into());
    }
    for (d, profile) in cert.profiles.iter().enumerate() {
        if matrix(s, d as u32)?.profile != *profile {
            return Err("profile mismatch".into());
        }
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 4 && args[1] == "prepare" {
        return pilot::prepare(&args[2], &args[3]);
    }
    if args.len() != 4 || !matches!(args[1].as_str(), "certify" | "verify") {
        return Err(
            "usage: prime-degree-certificate certify|verify INPUT.json CERTIFICATE.json".into(),
        );
    }
    let bytes = fs::read(&args[2])?;
    let s: System = serde_json::from_slice(&bytes)?;
    let hash = format!("{:x}", Sha256::digest(&bytes));
    if args[1] == "certify" {
        let c = certify(&s, hash.clone())?;
        verify(&s, &hash, &c)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&args[3])?;
        file.write_all(&serde_json::to_vec_pretty(&c)?)?;
        file.write_all(b"\n")?;
    } else {
        let c: Certificate = serde_json::from_slice(&fs::read(&args[3])?)?;
        verify(&s, &hash, &c)?;
    }
    println!("verified");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(p: u64) -> System {
        System {
            p,
            max_degree: 4,
            polynomials: vec![
                vec![Term { x: 0, y: 0, c: 1 }, Term { x: 2, y: 0, c: 1 }],
                vec![
                    Term { x: 0, y: 0, c: 1 },
                    Term { x: 1, y: 0, c: 1 },
                    Term { x: 2, y: 0, c: 1 },
                ],
            ],
        }
    }
    #[test]
    fn exact_degree_three_over_odd_primes() {
        for p in [5, 7, 17, 97] {
            let s = fixture(p);
            let c = certify(&s, "fixture".into()).unwrap();
            assert_eq!(c.refutation_degree, Some(3));
            verify(&s, "fixture", &c).unwrap();
        }
    }
    #[test]
    fn rejects_corruption_and_wrong_field() {
        let s = fixture(17);
        let mut c = certify(&s, "fixture".into()).unwrap();
        c.upper[0].c = (c.upper[0].c + 1) % 17;
        assert!(verify(&s, "fixture", &c).is_err());
        let mut c = certify(&s, "fixture".into()).unwrap();
        c.lower.as_mut().unwrap().values.clear();
        assert!(verify(&s, "fixture", &c).is_err());
        assert!(certify(&fixture(15), "bad".into()).is_err());
    }
    #[test]
    fn sat_and_capped_unsat_remain_unresolved() {
        let mut s = fixture(7);
        s.max_degree = 2;
        let c = certify(&s, "cap".into()).unwrap();
        assert_eq!(c.refutation_degree, None);
        verify(&s, "cap", &c).unwrap();
        s.polynomials.pop();
        let c = certify(&s, "sat".into()).unwrap();
        assert_eq!(c.refutation_degree, None);
        verify(&s, "sat", &c).unwrap();
    }
    #[test]
    fn checks_all_linear_generator_pairs_over_five() {
        for a in 0..25 {
            for b in 0..25 {
                let poly = |v: u64| {
                    let mut p = vec![];
                    if v % 5 != 0 {
                        p.push(Term {
                            x: 0,
                            y: 0,
                            c: v % 5,
                        });
                    }
                    if v / 5 != 0 {
                        p.push(Term {
                            x: 1,
                            y: 0,
                            c: v / 5,
                        });
                    }
                    p
                };
                let s = System {
                    p: 5,
                    max_degree: 2,
                    polynomials: vec![poly(a), poly(b)],
                };
                let sat = (0..5).any(|x| {
                    s.polynomials.iter().all(|p| {
                        p.iter()
                            .fold(0, |v, t| add(v, mul(t.c, pow(x, t.x as u64, 5), 5), 5))
                            == 0
                    })
                });
                let c = certify(&s, "linear".into()).unwrap();
                assert_eq!(c.refutation_degree.is_none(), sat);
                verify(&s, "linear", &c).unwrap();
            }
        }
    }
    #[test]
    fn ordinary_powers_are_not_boolean() {
        let s = System {
            p: 5,
            max_degree: 3,
            polynomials: vec![vec![Term { x: 2, y: 0, c: 1 }, Term { x: 3, y: 0, c: 4 }]],
        };
        let c = certify(&s, "ordinary".into()).unwrap();
        assert_eq!(c.refutation_degree, None);
        verify(&s, "ordinary", &c).unwrap();
        let mut s = s;
        s.polynomials[0][0].x = 25;
        assert!(validate(&s).is_err());
    }
}
