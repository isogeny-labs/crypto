//! Separate coefficient arithmetic: does not call Field or Poly arithmetic/eval.
use crate::{Artifact, Terms};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use torus_s4::{Poly, System};
pub type Result<T> = std::result::Result<T, String>;
type Polynomial = BTreeMap<Vec<usize>, BigUint>;
const MAX_TERMS: usize = 131072;
struct Ring {
    modulus: BigUint,
    remaining: usize,
}
impl Ring {
    fn step(&mut self) -> Result<()> {
        if self.remaining == 0 {
            return Err("checker resource refusal, not invalidity/absence".into());
        }
        self.remaining -= 1;
        Ok(())
    }
    fn add(&mut self, a: &mut Polynomial, b: &Polynomial, scale: &BigUint) -> Result<()> {
        for (m, c) in b {
            self.step()?;
            let value = (a.get(m).cloned().unwrap_or_default() + c * scale) % &self.modulus;
            if value.is_zero() {
                a.remove(m);
            } else {
                if !a.contains_key(m) && a.len() >= MAX_TERMS {
                    return Err("checker term refusal".into());
                }
                a.insert(m.clone(), value);
            }
        }
        Ok(())
    }
    fn multiply(&mut self, a: &Polynomial, b: &Polynomial) -> Result<Polynomial> {
        let mut out = Polynomial::new();
        for (ma, ca) in a {
            for (mb, cb) in b {
                self.step()?;
                let mut m = ma.clone();
                m.extend(mb);
                m.sort_unstable();
                if m.len() > 2 {
                    return Err("certificate multiplication exceeds degree2".into());
                }
                let value = (out.get(&m).cloned().unwrap_or_default() + ca * cb) % &self.modulus;
                if value.is_zero() {
                    out.remove(&m);
                } else {
                    if !out.contains_key(&m) && out.len() >= MAX_TERMS {
                        return Err("checker term refusal".into());
                    }
                    out.insert(m, value);
                }
            }
        }
        Ok(out)
    }
}
pub fn decimal(text: &str) -> Result<BigUint> {
    if text.is_empty()
        || text.len() > 155
        || !text.bytes().all(|b| b.is_ascii_digit())
        || (text.len() > 1 && text.starts_with('0'))
    {
        return Err("noncanonical bounded decimal".into());
    }
    text.parse::<BigUint>().map_err(|e| e.to_string())
}
fn decode(t: &Terms, n: usize, p: &BigUint, degree: usize) -> Result<Polynomial> {
    if t.len() > MAX_TERMS {
        return Err("certificate term limit".into());
    }
    let mut out = Polynomial::new();
    let mut previous: Option<&Vec<usize>> = None;
    for term in t {
        let m = &term.monomial;
        if m.len() > degree
            || !m.windows(2).all(|w| w[0] <= w[1])
            || m.iter().any(|v| *v >= n)
            || previous.is_some_and(|old| old >= m)
        {
            return Err("noncanonical monomial/order/index".into());
        }
        let c = decimal(&term.coefficient)?;
        if c.is_zero() || &c >= p {
            return Err("noncanonical coefficient".into());
        }
        out.insert(m.clone(), c);
        previous = Some(m);
    }
    Ok(out)
}
fn to_poly(p: &BigUint, t: Polynomial) -> Poly {
    let mut x = Poly::small(p, 0);
    x.terms = t;
    x
}

/// Strict request parser. The original/private Poly modulus is reconstructed
/// from this one public modulus, rather than trusting arbitrary in-memory Polys.
pub fn parse_request(bytes: &[u8]) -> Result<(System, Vec<usize>)> {
    if bytes.len() > 2 << 20 {
        return Err("request byte cap".into());
    }
    let v: Value = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    let allowed = BTreeSet::from([
        "schema",
        "id",
        "system",
        "protected",
        "preprocess_limits",
        "solver_limits",
        "source_identity_sha256",
    ]);
    if v.as_object()
        .ok_or("request object")?
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>()
        != allowed
    {
        return Err("request fields".into());
    }
    if v["schema"] != "torus.affine-request/v1" {
        return Err("request schema".into());
    }
    if v["id"]
        .as_str()
        .is_none_or(|s| s.is_empty() || s.len() > 256)
    {
        return Err("request id".into());
    }
    if v["source_identity_sha256"]
        .as_str()
        .is_none_or(|s| s.len() != 64 || !s.bytes().all(|b| b.is_ascii_hexdigit()))
    {
        return Err("source identity".into());
    }
    let _: crate::Limits =
        serde_json::from_value(v["preprocess_limits"].clone()).map_err(|e| e.to_string())?;
    if v["preprocess_limits"]
        != serde_json::to_value(crate::Limits::default()).map_err(|e| e.to_string())?
    {
        return Err("preprocess limits differ from frozen default".into());
    }
    if v["solver_limits"]
        != serde_json::to_value(torus_worklist_solver::Limits::default())
            .map_err(|e| e.to_string())?
    {
        return Err("v1 solver limits differ from frozen default".into());
    }
    let s = &v["system"];
    if s.as_object()
        .ok_or("system object")?
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>()
        != BTreeSet::from(["schema", "p", "variables", "equations"])
    {
        return Err("system fields".into());
    }
    if s["schema"] != "torus-s4.quadratic/v1" {
        return Err("system schema".into());
    }
    let p = decimal(s["p"].as_str().ok_or("modulus string")?)?;
    if !(5..=512).contains(&p.bits()) || !p.bit(0) {
        return Err("odd5..512bit modulus; primality external".into());
    }
    let names = s["variables"].as_array().ok_or("variable names")?;
    if names.is_empty() || names.len() > 4096 {
        return Err("variable admission".into());
    }
    let mut variables = vec![];
    let mut distinct = BTreeSet::new();
    for name in names {
        let name = name.as_str().ok_or("variable name type")?;
        if name.is_empty() || name.len() > 256 || !distinct.insert(name) {
            return Err("variable name admission/duplicate".into());
        }
        variables.push(name.to_owned());
    }
    let es = s["equations"].as_array().ok_or("equation array")?;
    if es.len() > 8192 {
        return Err("equation admission".into());
    }
    let mut equations = vec![];
    let mut total = 0;
    for e in es {
        let parsed: Terms = serde_json::from_value(e.clone()).map_err(|e| e.to_string())?;
        total += parsed.len();
        if total > 32768 {
            return Err("input term admission".into());
        }
        equations.push(to_poly(&p, decode(&parsed, variables.len(), &p, 2)?));
    }
    let roles = v["protected"].as_array().ok_or("protected array")?;
    let mut protected = vec![];
    let mut seen = BTreeSet::new();
    for role in roles {
        let i = role
            .as_u64()
            .and_then(|x| usize::try_from(x).ok())
            .ok_or("role index type")?;
        if i >= variables.len() || !seen.insert(i) {
            return Err("role duplicate/index".into());
        }
        protected.push(i);
    }
    Ok((
        System {
            p,
            variables,
            equations,
        },
        protected,
    ))
}

/// Validate all coefficient identities, then construct the admitted append-only
/// System. The producer's polynomial routines are not used for these identities.
pub fn verify(original: &System, protected: &[usize], a: &Artifact) -> Result<System> {
    if a.schema != "torus.affine-certificate/v1"
        || a.original_system_sha256 != crate::system_hash(original)
        || a.protected != protected
    {
        return Err("certificate binding".into());
    }
    let n = original.variables.len();
    let p = &original.p;
    if a.basis.len() > n + original.equations.len() || a.rewrites.len() != original.equations.len()
    {
        return Err("certificate shape".into());
    }
    let protected: BTreeSet<_> = protected.iter().copied().collect();
    let mut pivots = BTreeSet::new();
    for row in &a.basis {
        if let Some(v) = row.pivot {
            if v >= n || protected.contains(&v) || !pivots.insert(v) {
                return Err("pivot/role duplicate".into());
            }
        }
    }
    let mut ring = Ring {
        modulus: p.clone(),
        remaining: 1 << 24,
    };
    let mut polys = vec![];
    let mut retained = 0;
    for row in &a.basis {
        let polynomial = decode(&row.polynomial, n, p, 1)?;
        retained += polynomial.len() + row.origins.len();
        if retained > MAX_TERMS {
            return Err("certificate retained-term refusal".into());
        }
        if polynomial.is_empty() {
            return Err("zero basis row".into());
        }
        if let Some(v) = row.pivot {
            if polynomial.get(&vec![v]) != Some(&BigUint::one())
                || polynomial
                    .keys()
                    .flatten()
                    .any(|x| *x != v && pivots.contains(x))
            {
                return Err("nonreduced/unit pivot".into());
            }
        } else if polynomial.keys().flatten().any(|v| !protected.contains(v)) {
            return Err("side row contains an unprotected variable".into());
        }
        let mut combined = Polynomial::new();
        let mut seen = BTreeSet::new();
        for w in &row.origins {
            if w.equation >= original.equations.len() || !seen.insert(w.equation) {
                return Err("lineage index/duplicate".into());
            }
            let f = &original.equations[w.equation];
            if f.degree() > 1 {
                return Err("lineage is not an original linear equation".into());
            }
            let c = decimal(&w.coefficient)?;
            if c.is_zero() || &c >= p {
                return Err("lineage coefficient".into());
            }
            ring.add(&mut combined, &f.terms, &c)?;
        }
        if combined != polynomial {
            return Err("basis lineage identity failed".into());
        }
        polys.push(polynomial);
    }
    let mut output = original.clone();
    output
        .equations
        .extend(polys.iter().cloned().map(|t| to_poly(p, t)));
    for (i, r) in a.rewrites.iter().enumerate() {
        if r.source_equation != i {
            return Err("rewrite source/order".into());
        }
        let residual = decode(&r.residual, n, p, 2)?;
        retained += residual.len()
            + r.quotients
                .iter()
                .map(|q| q.polynomial.len())
                .sum::<usize>();
        if retained > MAX_TERMS {
            return Err("certificate retained-term refusal".into());
        }
        let mut recomposed = residual.clone();
        let mut seen = BTreeSet::new();
        for q in &r.quotients {
            if q.basis >= polys.len() || a.basis[q.basis].pivot.is_none() || !seen.insert(q.basis) {
                return Err("quotient basis index/duplicate".into());
            }
            let coefficient = decode(&q.polynomial, n, p, 1)?;
            let product = ring.multiply(&polys[q.basis], &coefficient)?;
            ring.add(&mut recomposed, &product, &BigUint::one())?;
        }
        if recomposed != original.equations[i].terms {
            return Err("rewrite coefficient identity failed".into());
        }
        if !residual.is_empty() && residual != original.equations[i].terms {
            output.equations.push(to_poly(p, residual));
        }
    }
    if output.variables != original.variables
        || output.equations[..original.equations.len()] != original.equations
    {
        return Err("original prefix lost".into());
    }
    Ok(output)
}

pub fn assignment(original: &System, values: &[BigUint]) -> Result<()> {
    if values.len() != original.variables.len() || values.iter().any(|v| v >= &original.p) {
        return Err("assignment canonical shape".into());
    }
    for p in &original.equations {
        let mut sum = BigUint::zero();
        for (m, c) in &p.terms {
            let mut term = c.clone();
            for &v in m {
                term = term * values.get(v).ok_or("assignment index")? % &original.p;
            }
            sum = (sum + term) % &original.p;
        }
        if !sum.is_zero() {
            return Err("independent original equation rejected".into());
        }
    }
    Ok(())
}
