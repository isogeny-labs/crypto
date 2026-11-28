//! Exact Fp101[T][X] pencil arithmetic, not a coefficient solver.
//! Hard bounds are fixed by PROTOCOL.md, not inferred from favorable outputs.
use crate::Result;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
pub type T = Vec<u64>;
pub type XT = Vec<T>;
fn trim(a: &mut T) {
    while a.last() == Some(&0) {
        a.pop();
    }
}
fn xt_trim(a: &mut XT) {
    while a.last().is_some_and(Vec::is_empty) {
        a.pop();
    }
}
pub fn scalar(a: u64) -> T {
    if a.is_multiple_of(101) {
        vec![]
    } else {
        vec![a % 101]
    }
}
pub fn add(a: &T, b: &T) -> T {
    let mut r = vec![0; a.len().max(b.len())];
    for (i, v) in r.iter_mut().enumerate() {
        *v = (a.get(i).copied().unwrap_or(0) + b.get(i).copied().unwrap_or(0)) % 101;
    }
    trim(&mut r);
    r
}
pub fn neg(a: &T) -> T {
    a.iter().map(|x| (101 - x) % 101).collect()
}
pub fn mul(a: &T, b: &T) -> Result<T> {
    if a.is_empty() || b.is_empty() {
        return Ok(vec![]);
    }
    if a.len() + b.len() - 2 > 96 {
        return Err("formal T-degree cap".into());
    }
    let mut r = vec![0; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            r[i + j] = (r[i + j] + x * y) % 101;
        }
    }
    trim(&mut r);
    Ok(r)
}
fn xpadd(a: &XT, b: &XT) -> XT {
    let mut r = vec![vec![]; a.len().max(b.len())];
    for (i, v) in r.iter_mut().enumerate() {
        *v = add(a.get(i).unwrap_or(&vec![]), b.get(i).unwrap_or(&vec![]));
    }
    xt_trim(&mut r);
    r
}
fn xpneg(a: &XT) -> XT {
    a.iter().map(neg).collect()
}
fn xpmul(a: &XT, b: &XT) -> Result<XT> {
    if a.is_empty() || b.is_empty() {
        return Ok(vec![]);
    }
    if a.len() + b.len() - 2 > 48 {
        return Err("formal X-degree cap".into());
    }
    let mut r = vec![vec![]; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            r[i + j] = add(&r[i + j], &mul(x, y)?);
        }
    }
    xt_trim(&mut r);
    Ok(r)
}

/// c4=1, c3=T; the remaining coefficients and finite R are fixed.
pub fn norm_pencil(a: u64, r: u64, s: u64, cs: &[u64]) -> Result<XT> {
    if cs.len() != 6 || cs[4] != 1 || s == 0 {
        return Err("finite normalized pencil contract".into());
    }
    let line = vec![scalar((101 - r) % 101), scalar(1)];
    let u = xpadd(
        &xpmul(&line, &vec![scalar(cs[0]), scalar(cs[1]), vec![0, 1]])?,
        &vec![scalar(cs[5] * s)],
    );
    let v = xpadd(
        &xpmul(&line, &vec![scalar(cs[2]), scalar(1)])?,
        &vec![scalar(cs[5])],
    );
    let n = xpadd(
        &xpmul(&u, &u)?,
        &xpneg(&xpmul(
            &vec![vec![], scalar(1), scalar(a), scalar(1)],
            &xpmul(&v, &v)?,
        )?),
    );
    let mut rem = n;
    let mut quotient = vec![vec![]; rem.len() - 1];
    for i in (1..rem.len()).rev() {
        let lead = rem[i].clone();
        quotient[i - 1] = lead.clone();
        rem[i] = vec![];
        rem[i - 1] = add(&rem[i - 1], &mul(&scalar(r), &lead)?);
    }
    if !rem[0].is_empty() {
        return Err("symbolic forced factor does not divide".into());
    }
    let result = xpneg(&quotient);
    if result.len() != 7 || result[6] != scalar(1) || result[5].len() != 3 || result[5][2] != 100 {
        return Err("normalized norm pencil leading contract".into());
    }
    Ok(result)
}

pub fn audit(dense_e: &[u64], norm: &XT) -> Result<Value> {
    if dense_e.len() < 7 || dense_e.len() > 49 || norm.len() != 7 || norm[6] != scalar(1) {
        return Err("degree audit cap/monicity".into());
    }
    let mut rem: XT = dense_e.iter().map(|v| scalar(*v)).collect();
    for i in (6..rem.len()).rev() {
        let lead = rem[i].clone();
        rem[i] = vec![];
        for (j, s) in norm.iter().take(6).enumerate() {
            rem[i - 6 + j] = add(&rem[i - 6 + j], &neg(&mul(&lead, s)?));
        }
    }
    rem.truncate(6);
    let degree = rem[5].len().checked_sub(1).ok_or("zero X5 coefficient")?;
    let expected = 2 * (dense_e.len() - 1 - 5);
    if degree != expected {
        return Err(format!("degree mismatch {degree} versus {expected}"));
    }
    let bytes = serde_json::to_vec(&rem).map_err(|e| e.to_string())?;
    Ok(
        json!({"E_degree":dense_e.len()-1,"X5_T_degree":degree,"expected":expected,
        "remainder":rem,"remainder_sha256":format!("{:x}",Sha256::digest(bytes)),"S_pencil":norm,
        "function_reduction":false}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn formal_is_not_function_reduction() {
        assert!(add(&vec![100, 1], &vec![1, 100]).is_empty());
        assert_eq!(mul(&vec![0, 1], &vec![0, 1]).unwrap(), vec![0, 0, 1]);
        assert!(mul(&vec![1; 50], &vec![1; 50]).is_err());
    }
    #[test]
    fn norm_and_degree_leading_path() {
        let n = norm_pencil(95, 21, 77, &[2, 3, 4, 9, 1, 5]).unwrap();
        for d in [6, 12, 18, 24, 30, 36, 42, 48] {
            let mut e = vec![0; d + 1];
            e[d] = 1;
            assert_eq!(audit(&e, &n).unwrap()["X5_T_degree"], 2 * (d - 5));
        }
    }
}
