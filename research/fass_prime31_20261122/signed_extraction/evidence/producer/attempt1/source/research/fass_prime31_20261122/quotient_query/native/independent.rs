//! Independent algebraic certificate checker.
//!
//! This module deliberately does not call the producer's field, polynomial,
//! gcd, or tensor routines. It uses explicit u128 residue arithmetic and
//! elementary polynomial identities. It verifies the supplied certificate;
//! it is not an independent implementation of the producer search algorithm.
use crate::kernel::{Certificate, Poly, Tensor};
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Debug, Serialize)]
pub struct Verification {
    pub branches: usize,
    pub algebraic_support: usize,
    pub identity_checks: usize,
}

#[derive(Debug, Serialize)]
pub struct GridVerification {
    pub cells: usize,
    pub zero_cells: usize,
}

#[derive(Debug, Serialize)]
pub struct InputVerification {
    pub raw_right_degree: usize,
    pub direct_product_cells: usize,
}

fn add(p: u64, a: u64, b: u64) -> u64 {
    ((a as u128 + b as u128) % p as u128) as u64
}
fn sub(p: u64, a: u64, b: u64) -> u64 {
    ((a as u128 + p as u128 - b as u128) % p as u128) as u64
}
fn mul(p: u64, a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn pow(p: u64, mut a: u64, mut n: u64) -> u64 {
    let mut z = 1;
    while n != 0 {
        if n & 1 != 0 {
            z = mul(p, z, a);
        }
        a = mul(p, a, a);
        n >>= 1;
    }
    z
}
fn prime(p: u64) -> bool {
    if p < 3 || p >= (1u64 << 32) || p % 2 == 0 {
        return false;
    }
    let mut d = 3;
    while d * d <= p {
        if p % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}
fn zero(a: &[u64]) -> bool {
    a.iter().all(|&c| c == 0)
}
fn trim(mut a: Poly) -> Poly {
    while a.len() > 1 && a.last() == Some(&0) {
        a.pop();
    }
    if a.is_empty() {
        a.push(0);
    }
    a
}
fn padd(p: u64, a: &[u64], b: &[u64]) -> Poly {
    let mut c = vec![0; a.len().max(b.len())];
    for (i, v) in c.iter_mut().enumerate() {
        *v = add(p, *a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0));
    }
    trim(c)
}
fn psub(p: u64, a: &[u64], b: &[u64]) -> Poly {
    let mut c = vec![0; a.len().max(b.len())];
    for (i, v) in c.iter_mut().enumerate() {
        *v = sub(p, *a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0));
    }
    trim(c)
}
fn pmul(p: u64, a: &[u64], b: &[u64]) -> Poly {
    let mut c = vec![0; a.len() + b.len() - 1];
    for (i, &u) in a.iter().enumerate() {
        for (j, &v) in b.iter().enumerate() {
            c[i + j] = add(p, c[i + j], mul(p, u, v));
        }
    }
    trim(c)
}
fn prem(p: u64, a: &[u64], b: &[u64]) -> Poly {
    assert!(!zero(b));
    let mut r = trim(a.to_vec());
    let lead_inv = pow(p, *b.last().unwrap(), p - 2);
    while !zero(&r) && r.len() >= b.len() {
        let k = r.len() - b.len();
        let scale = mul(p, *r.last().unwrap(), lead_inv);
        for (j, &v) in b.iter().enumerate() {
            r[k + j] = sub(p, r[k + j], mul(p, scale, v));
        }
        r = trim(r);
    }
    r
}
fn pgcd(p: u64, a: &[u64], b: &[u64]) -> Poly {
    let (mut a, mut b) = (trim(a.to_vec()), trim(b.to_vec()));
    while !zero(&b) {
        let r = prem(p, &a, &b);
        a = b;
        b = r;
    }
    if zero(&a) {
        return a;
    }
    let inv = pow(p, *a.last().unwrap(), p - 2);
    a.iter().map(|&v| mul(p, v, inv)).collect()
}
fn ppow_mod(p: u64, a: &[u64], mut n: u64, m: &[u64]) -> Poly {
    let mut z = prem(p, &[1], m);
    let mut a = prem(p, a, m);
    while n != 0 {
        if n & 1 != 0 {
            z = prem(p, &pmul(p, &z, &a), m);
        }
        a = prem(p, &pmul(p, &a, &a), m);
        n >>= 1;
    }
    z
}
fn canonical_poly(p: u64, a: &[u64], name: &str) -> Result<(), String> {
    if a.is_empty() || a.iter().any(|&v| v >= p) || (a.len() > 1 && a.last() == Some(&0)) {
        return Err(format!("{name}: noncanonical polynomial"));
    }
    Ok(())
}
fn split_squarefree(p: u64, a: &[u64], name: &str) -> Result<(), String> {
    canonical_poly(p, a, name)?;
    if a.last() != Some(&1) {
        return Err(format!("{name}: not monic"));
    }
    // A monic polynomial divides Z^p-Z iff its roots are distinct and in Fp.
    let q = ppow_mod(p, &[0, 1], p, a);
    if !zero(&prem(p, &psub(p, &q, &[0, 1]), a)) {
        return Err(format!("{name}: not squarefree and split"));
    }
    Ok(())
}
fn ttrim(mut a: Tensor) -> Tensor {
    while a.len() > 1 && zero(a.last().unwrap()) {
        a.pop();
    }
    if a.is_empty() {
        a.push(vec![0]);
    }
    a
}
fn canonical_tensor(p: u64, a: &Tensor, m: &[u64], name: &str) -> Result<(), String> {
    if a.is_empty() || (a.len() > 1 && zero(a.last().unwrap())) {
        return Err(format!("{name}: noncanonical tensor"));
    }
    for row in a {
        canonical_poly(p, row, name)?;
        if !zero(row) && row.len() >= m.len() {
            return Err(format!("{name}: coefficient exceeds quotient degree"));
        }
    }
    Ok(())
}
fn treduce(p: u64, a: &Tensor, m: &[u64]) -> Tensor {
    ttrim(a.iter().map(|row| prem(p, row, m)).collect())
}
fn tadd(p: u64, a: &Tensor, b: &Tensor) -> Tensor {
    let mut c = vec![vec![0]; a.len().max(b.len())];
    for (i, row) in a.iter().enumerate() {
        c[i] = padd(p, &c[i], row);
    }
    for (i, row) in b.iter().enumerate() {
        c[i] = padd(p, &c[i], row);
    }
    ttrim(c)
}
fn tmul(p: u64, a: &Tensor, b: &Tensor, m: &[u64]) -> Tensor {
    let mut c = vec![vec![0]; a.len() + b.len() - 1];
    for (i, u) in a.iter().enumerate() {
        for (j, v) in b.iter().enumerate() {
            c[i + j] = padd(p, &c[i + j], &pmul(p, u, v));
        }
    }
    treduce(p, &c, m)
}
fn tensor_of(a: &[u64]) -> Tensor {
    a.iter().map(|&v| vec![v]).collect()
}
fn eval(p: u64, a: &[u64], z: u64) -> u64 {
    a.iter().rev().fold(0, |r, &v| add(p, mul(p, r, z), v))
}
fn teval(p: u64, a: &Tensor, x: u64, t: u64) -> u64 {
    a.iter()
        .rev()
        .fold(0, |r, row| add(p, mul(p, r, x), eval(p, row, t)))
}

pub fn verify(c: &Certificate) -> Result<Verification, String> {
    if !prime(c.p) {
        return Err("unsupported or composite field".into());
    }
    if c.f.len() > 4097 || c.g.len() > 8193 {
        return Err("verification dimension cap".into());
    }
    split_squarefree(c.p, &c.f, "F")?;
    split_squarefree(c.p, &c.g, "G")?;
    canonical_tensor(c.p, &c.h, &c.g, "H")?;
    if c.f.len() == 1 {
        if c.h != vec![vec![0]] {
            return Err("H must be zero in the empty left quotient".into());
        }
    } else if c.h.len() >= c.f.len() {
        return Err("H exceeds left remainder degree".into());
    }
    if c.g.len() == 1 {
        if !c.leaves.is_empty() || c.support_count != 0 || !c.verdict_empty {
            return Err(
                "empty right quotient requires an empty partition and empty verdict".into(),
            );
        }
        return Ok(Verification {
            branches: 0,
            algebraic_support: 0,
            identity_checks: 0,
        });
    }
    let mut product = vec![1];
    let mut support = 0usize;
    let f = tensor_of(&c.f);
    for (i, leaf) in c.leaves.iter().enumerate() {
        let q = &leaf.modulus;
        canonical_poly(c.p, q, "branch modulus")?;
        if q.len() < 2 || q.len() > c.g.len() || q.last() != Some(&1) {
            return Err("branch modulus must be nonconstant monic".into());
        }
        for prev in &c.leaves[..i] {
            if pgcd(c.p, q, &prev.modulus) != vec![1] {
                return Err("branch moduli are not coprime".into());
            }
        }
        product = pmul(c.p, &product, q);
        if product.len() > c.g.len() {
            return Err("partition has excess degree".into());
        }
        for (name, a) in [
            ("D", &leaf.gcd),
            ("F cofactor", &leaf.f_cofactor),
            ("H cofactor", &leaf.h_cofactor),
            ("Bezout F", &leaf.bezout_f),
            ("Bezout H", &leaf.bezout_h),
        ] {
            if a.len() > 2 * c.f.len() + 1 {
                return Err(format!("{name}: certificate size cap"));
            }
            canonical_tensor(c.p, a, q, name)?;
        }
        if leaf.gcd.last() != Some(&vec![1]) || leaf.gcd.len() > c.f.len() {
            return Err("D must be monic of degree at most deg F".into());
        }
        let h = treduce(c.p, &c.h, q);
        if tmul(c.p, &leaf.gcd, &leaf.f_cofactor, q) != f {
            return Err("F cofactor identity fails".into());
        }
        if tmul(c.p, &leaf.gcd, &leaf.h_cofactor, q) != h {
            return Err("H cofactor identity fails".into());
        }
        let bezout = tadd(
            c.p,
            &tmul(c.p, &leaf.bezout_f, &f, q),
            &tmul(c.p, &leaf.bezout_h, &h, q),
        );
        if bezout != leaf.gcd {
            return Err("Bezout identity fails".into());
        }
        support = support
            .checked_add((q.len() - 1) * (leaf.gcd.len() - 1))
            .ok_or("support count overflow")?;
    }
    if product != c.g {
        return Err("branch product is not G".into());
    }
    if c.support_count != support || c.verdict_empty != (support == 0) {
        return Err("support count or empty verdict disagrees with certificate".into());
    }
    Ok(Verification {
        branches: c.leaves.len(),
        algebraic_support: support,
        identity_checks: 3 * c.leaves.len(),
    })
}

fn check_roots(p: u64, a: &[u64], roots: &[u64], name: &str) -> Result<(), String> {
    let distinct: BTreeSet<_> = roots.iter().copied().collect();
    if roots.len() + 1 != a.len()
        || distinct.len() != roots.len()
        || roots.iter().any(|&z| z >= p || eval(p, a, z) != 0)
    {
        return Err(format!("{name}: root inventory is incomplete or invalid"));
    }
    Ok(())
}

pub fn verify_grid(c: &Certificate, xs: &[u64], ts: &[u64]) -> Result<GridVerification, String> {
    let checked = verify(c)?;
    check_roots(c.p, &c.f, xs, "F")?;
    check_roots(c.p, &c.g, ts, "G")?;
    let mut zeros = 0;
    for &t in ts {
        let branches: Vec<_> = c
            .leaves
            .iter()
            .filter(|leaf| eval(c.p, &leaf.modulus, t) == 0)
            .collect();
        if branches.len() != 1 {
            return Err("right root not routed to exactly one branch".into());
        }
        let leaf = branches[0];
        let mut fiber_zeros = 0;
        for &x in xs {
            let actual = teval(c.p, &c.h, x, t) == 0;
            let claimed = teval(c.p, &leaf.gcd, x, t) == 0;
            if actual != claimed {
                return Err("branch gcd does not match grid support".into());
            }
            if actual {
                fiber_zeros += 1;
            }
        }
        if fiber_zeros + 1 != leaf.gcd.len() {
            return Err("fiber support disagrees with monic gcd degree".into());
        }
        zeros += fiber_zeros;
    }
    if zeros != checked.algebraic_support {
        return Err("grid and certificate support counts differ".into());
    }
    Ok(GridVerification {
        cells: xs.len() * ts.len(),
        zero_cells: zeros,
    })
}

// Symmetric expanded formula, deliberately different from the producer's
// difference-square representation of the same S3 polynomial.
fn s3(p: u64, a: u64, b: u64, x: u64, z: u64, t: u64) -> u64 {
    let xz = mul(p, x, z);
    let xt = mul(p, x, t);
    let zt = mul(p, z, t);
    let sum = add(p, add(p, x, z), t);
    let pairs = add(p, add(p, xz, xt), zt);
    let squares = add(p, add(p, mul(p, xz, xz), mul(p, xt, xt)), mul(p, zt, zt));
    let triple = mul(p, 2, mul(p, mul(p, xz, t), sum));
    let a_part = mul(p, 2, mul(p, a, pairs));
    let b_part = mul(p, 4 % p, mul(p, b, sum));
    add(
        p,
        sub(p, sub(p, sub(p, squares, triple), a_part), b_part),
        mul(p, a, a),
    )
}

/// Bind a checked certificate to its original S3 inputs. The independent
/// path interpolates each quadratic at 0,+1,-1 using the symmetric formula,
/// builds the raw right product, extracts its rational squarefree part by
/// an independent gcd, and checks H at every point of the complete split
/// quotient grid. The grid covers the canonical remainder, not just samples.
pub fn verify_s3_input(
    c: &Certificate,
    a: u64,
    b: u64,
    d: usize,
    target_x: u64,
    selector_roots: &[u64],
    xs: &[u64],
    ts: &[u64],
    restrict_liftable: bool,
) -> Result<InputVerification, String> {
    verify(c)?;
    if c.p <= 3 || a >= c.p || b >= c.p || target_x >= c.p || d == 0 || (c.p - 1) % d as u64 != 0 {
        return Err("S3 input outside canonical split-selector domain".into());
    }
    let disc = add(
        c.p,
        mul(c.p, 4, mul(c.p, mul(c.p, a, a), a)),
        mul(c.p, 27 % c.p, mul(c.p, b, b)),
    );
    if disc == 0 {
        return Err("singular curve input".into());
    }
    let selector_set: BTreeSet<_> = selector_roots.iter().copied().collect();
    if selector_roots.len() != d
        || selector_set.len() != d
        || selector_roots
            .iter()
            .any(|&z| z >= c.p || pow(c.p, z, d as u64) != 1)
    {
        return Err("selector roots do not exhaust mu_d".into());
    }
    check_roots(c.p, &c.f, xs, "F")?;
    check_roots(c.p, &c.g, ts, "G")?;
    let wanted: BTreeSet<_> = selector_roots
        .iter()
        .copied()
        .filter(|&z| {
            let rhs = add(
                c.p,
                add(c.p, mul(c.p, mul(c.p, z, z), z), mul(c.p, a, z)),
                b,
            );
            !restrict_liftable || rhs == 0 || pow(c.p, rhs, (c.p - 1) / 2) == 1
        })
        .collect();
    if wanted != xs.iter().copied().collect() {
        return Err("left roots do not match the declared liftability restriction".into());
    }
    let half = pow(c.p, 2, c.p - 2);
    let mut raw = vec![1];
    for &z in selector_roots {
        let at0 = s3(c.p, a, b, z, target_x, 0);
        let at1 = s3(c.p, a, b, z, target_x, 1);
        let atm1 = s3(c.p, a, b, z, target_x, c.p - 1);
        let linear = mul(c.p, sub(c.p, at1, atm1), half);
        let quadratic = mul(c.p, sub(c.p, add(c.p, at1, atm1), mul(c.p, 2, at0)), half);
        let q = trim(vec![at0, linear, quadratic]);
        if zero(&q) {
            return Err("zero S3 factor outside nonsingular domain".into());
        }
        raw = pmul(c.p, &raw, &q);
    }
    let field_eq_mod_raw = psub(c.p, &ppow_mod(c.p, &[0, 1], c.p, &raw), &[0, 1]);
    if pgcd(c.p, &raw, &field_eq_mod_raw) != c.g {
        return Err("G does not equal the rational part of the original right product".into());
    }
    for &x in xs {
        for &t in ts {
            let direct = selector_roots
                .iter()
                .fold(1, |v, &z| mul(c.p, v, s3(c.p, a, b, x, z, t)));
            if teval(c.p, &c.h, x, t) != direct {
                return Err("H does not equal the original S3 norm on the quotient grid".into());
            }
        }
    }
    Ok(InputVerification {
        raw_right_degree: raw.len() - 1,
        direct_product_cells: xs.len() * ts.len(),
    })
}

fn bump_tensor(p: u64, a: &mut Tensor) {
    a[0][0] = add(p, a[0][0], 1);
    a[0] = trim(a[0].clone());
    *a = ttrim(a.clone());
}

/// Deliberate mutations of a supplied passing certificate; no producer logic
/// is used to decide whether the mutated identities are acceptable.
pub fn verify_rejects_mutations(c: &Certificate) -> Result<usize, String> {
    verify(c)?;
    let mut cases: Vec<(&str, Certificate)> = Vec::new();
    let mut z = c.clone();
    z.verdict_empty = !z.verdict_empty;
    cases.push(("false verdict", z));
    let mut z = c.clone();
    z.support_count += 1;
    cases.push(("false count", z));
    let mut z = c.clone();
    *z.f.last_mut().unwrap() = 2;
    cases.push(("nonmonic F", z));
    let mut z = c.clone();
    z.h[0][0] = c.p;
    cases.push(("noncanonical residue", z));
    let mut z = c.clone();
    z.g = vec![1, c.p - 2, 1];
    cases.push(("repeated G", z));
    let nonsquare = (2..c.p)
        .find(|&a| pow(c.p, a, (c.p - 1) / 2) == c.p - 1)
        .ok_or("no nonsquare found")?;
    let mut z = c.clone();
    z.g = vec![c.p - nonsquare, 0, 1];
    cases.push(("nonsplit G", z));
    if !c.leaves.is_empty() {
        let mut z = c.clone();
        z.leaves.remove(0);
        cases.push(("missing branch", z));
        let mut z = c.clone();
        z.leaves.push(z.leaves[0].clone());
        cases.push(("duplicated noncoprime branch", z));
        let mut z = c.clone();
        bump_tensor(c.p, &mut z.leaves[0].bezout_f);
        cases.push(("wrong Bezout", z));
        let mut z = c.clone();
        bump_tensor(c.p, &mut z.leaves[0].f_cofactor);
        cases.push(("wrong cofactor", z));
        let mut z = c.clone();
        bump_tensor(c.p, &mut z.leaves[0].gcd);
        cases.push(("wrong gcd", z));
        let mut z = c.clone();
        *z.leaves[0].modulus.last_mut().unwrap() = 2;
        cases.push(("nonmonic branch", z));
    }
    for (name, altered) in &cases {
        if verify(altered).is_ok() {
            return Err(format!("mutation accepted: {name}"));
        }
    }
    Ok(cases.len())
}
