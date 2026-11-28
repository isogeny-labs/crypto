//! Existing fixed-p31 mature-CAS parsers, shared without semantic changes.
use certified_relations::Poly;
use std::{collections::BTreeSet, error::Error};
pub type Basis = BTreeSet<Poly>;
type Result<T> = std::result::Result<T, Box<dyn Error>>;
const P: u64 = 2147483647;
pub fn singular_basis(text: &str, n: usize) -> Result<Basis> {
    let mut basis = Basis::new();
    let mut current = None;
    let mut done = false;
    for line in text.lines().map(str::trim).filter(|s| !s.is_empty()) {
        if line == "POLY" {
            if current.is_some() || done {
                return Err("bad POLY order".into());
            }
            current = Some(Poly::new());
        } else if let Some(term) = line.strip_prefix("TERM:") {
            let (c, m) = term.split_once(':').ok_or("bad TERM")?;
            let c = (c.parse::<i128>()?).rem_euclid(P as i128) as u64;
            let m: Vec<usize> = m
                .split(',')
                .map(|x| x.trim().parse())
                .collect::<std::result::Result<_, _>>()?;
            if m.len() != n
                || m.iter()
                    .try_fold(0usize, |s, e| s.checked_add(*e))
                    .is_none_or(|d| d > 64)
                || c == 0
            {
                return Err("bad term shape/residue".into());
            }
            if current
                .as_mut()
                .ok_or("TERM outside POLY")?
                .insert(m, c)
                .is_some()
            {
                return Err("duplicate monomial".into());
            }
        } else if line == "ENDPOLY" {
            let f = current.take().ok_or("END outside POLY")?;
            if f.is_empty() || !basis.insert(f) {
                return Err("empty/duplicate polynomial".into());
            }
        } else if line == "DONE" {
            if done || current.is_some() {
                return Err("bad completion".into());
            }
            done = true;
        } else if !line.starts_with("//") {
            return Err("unexpected Singular output".into());
        }
    }
    if !done || current.is_some() || basis.is_empty() {
        return Err("missing complete basis".into());
    }
    Ok(basis)
}
pub fn msolve_basis(text: &str, names: &[String]) -> Result<Basis> {
    let text = text
        .lines()
        .filter(|s| !s.trim_start().starts_with('#'))
        .collect::<String>();
    let body = text
        .trim()
        .strip_prefix('[')
        .and_then(|s| s.strip_suffix("]:"))
        .ok_or("bad msolve framing")?;
    let mut basis = Basis::new();
    for polynomial in body.split(',') {
        let mut f = Poly::new();
        for term in polynomial.trim().split('+') {
            let mut factors = term.trim().split('*');
            let c = factors
                .next()
                .ok_or("missing coefficient")?
                .trim()
                .parse::<i128>()?
                .rem_euclid(P as i128) as u64;
            let mut m = vec![0; names.len()];
            for factor in factors {
                let (v, e) = factor.trim().split_once('^').ok_or("missing power")?;
                let axis = names
                    .iter()
                    .position(|n| n == v.trim())
                    .ok_or("unknown variable")?;
                if m[axis] != 0 {
                    return Err("repeated variable".into());
                }
                m[axis] = e.trim().parse()?;
            }
            if m.iter()
                .try_fold(0usize, |s, e| s.checked_add(*e))
                .is_none_or(|d| d > 64)
                || c == 0
                || f.insert(m, c).is_some()
            {
                return Err("malformed/duplicate msolve term".into());
            }
        }
        if f.is_empty() || !basis.insert(f) {
            return Err("empty/duplicate msolve polynomial".into());
        }
    }
    if basis.is_empty() {
        return Err("missing msolve basis".into());
    }
    Ok(basis)
}
