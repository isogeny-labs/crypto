//! Dynamic evaluation of gcd_x(F,H) over the split algebra F_p[t]/G.
//!
//! F and G are monic, squarefree and split. No roots of G are supplied to
//! this solver. A nonunit leading coefficient splits G before inversion.
//! Each terminal block carries both Bezout and exact-divisibility witnesses.
//! This is a dense coefficient implementation, with no sublinear cost claim.

use crate::semaev_prime_poly::{
    derivative, poly_gcd, poly_mulmod, poly_powmod, poly_rem, trim, Fp,
};
use serde::{Deserialize, Serialize};

pub type Poly = Vec<u64>;
/// Low-first outer x coefficients, themselves low-first polynomials in t.
pub type Tensor = Vec<Poly>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Leaf {
    pub modulus: Poly,
    pub gcd: Tensor,
    pub f_cofactor: Tensor,
    pub h_cofactor: Tensor,
    pub bezout_f: Tensor,
    pub bezout_h: Tensor,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct QueryStats {
    pub blocks_started: usize,
    pub splits: usize,
    pub coefficient_inversions: usize,
    pub euclidean_steps: usize,
    pub zero_leading_coefficients: usize,
    /// Stored scalar slots in inputs and terminal polynomial witnesses.
    pub output_coefficients: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Certificate {
    pub p: u64,
    pub f: Poly,
    pub g: Poly,
    pub h: Tensor,
    pub leaves: Vec<Leaf>,
    pub verdict_empty: bool,
    pub support_count: usize,
    pub stats: QueryStats,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub enum QueryError {
    UnsupportedField,
    InvalidPolynomial(&'static str),
    Nonmonic(&'static str),
    Nonsquarefree(&'static str),
    Nonsplit(&'static str),
    NoncanonicalTensor,
    DegreeLimit,
    Internal(&'static str),
}

pub fn is_zero(p: &[u64]) -> bool {
    p == [0]
}

pub fn add(f: Fp, a: &[u64], b: &[u64]) -> Poly {
    trim(
        (0..a.len().max(b.len()))
            .map(|i| f.add(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0)))
            .collect(),
    )
}

pub fn sub(f: Fp, a: &[u64], b: &[u64]) -> Poly {
    trim(
        (0..a.len().max(b.len()))
            .map(|i| f.sub(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0)))
            .collect(),
    )
}

pub fn scale(f: Fp, a: &[u64], c: u64) -> Poly {
    trim(a.iter().map(|&v| f.mul(v, c)).collect())
}

pub fn monic(f: Fp, a: &[u64]) -> Poly {
    let a = trim(a.to_vec());
    assert!(!is_zero(&a));
    scale(f, &a, f.inv(*a.last().unwrap()))
}

/// Exact multiplication via the unchanged production modular multiplier.
pub fn multiply(f: Fp, a: &[u64], b: &[u64]) -> Poly {
    let mut m = vec![0; a.len() + b.len()];
    *m.last_mut().unwrap() = 1;
    poly_mulmod(f, a, b, &m)
}

/// Quotient and remainder needed for extended Euclid; field operations are
/// the existing production Fp implementation.
pub fn divide(f: Fp, a: &[u64], b: &[u64]) -> (Poly, Poly) {
    assert!(!is_zero(b));
    let mut r = trim(a.to_vec());
    if r.len() < b.len() {
        return (vec![0], r);
    }
    let mut q = vec![0; r.len() - b.len() + 1];
    let inv = f.inv(*b.last().unwrap());
    while !is_zero(&r) && r.len() >= b.len() {
        let shift = r.len() - b.len();
        let c = f.mul(*r.last().unwrap(), inv);
        q[shift] = c;
        for (i, &v) in b.iter().enumerate() {
            r[shift + i] = f.sub(r[shift + i], f.mul(c, v));
        }
        r = trim(r);
    }
    (trim(q), r)
}

fn xgcd(f: Fp, a: &[u64], b: &[u64]) -> (Poly, Poly, Poly) {
    let (mut r0, mut r1) = (a.to_vec(), b.to_vec());
    let (mut u0, mut u1) = (vec![1], vec![0]);
    let (mut v0, mut v1) = (vec![0], vec![1]);
    while !is_zero(&r1) {
        let (q, r2) = divide(f, &r0, &r1);
        let u2 = sub(f, &u0, &multiply(f, &q, &u1));
        let v2 = sub(f, &v0, &multiply(f, &q, &v1));
        (r0, r1) = (r1, r2);
        (u0, u1) = (u1, u2);
        (v0, v1) = (v1, v2);
    }
    let c = f.inv(*r0.last().unwrap());
    (scale(f, &r0, c), scale(f, &u0, c), scale(f, &v0, c))
}

pub fn tensor_trim(mut a: Tensor) -> Tensor {
    for p in &mut a {
        *p = trim(std::mem::take(p));
    }
    while a.len() > 1 && is_zero(a.last().unwrap()) {
        a.pop();
    }
    if a.is_empty() {
        a.push(vec![0]);
    }
    a
}

fn tensor_zero(a: &Tensor) -> bool {
    a == &[vec![0]]
}

fn tensor_reduce(f: Fp, a: &Tensor, m: &[u64]) -> Tensor {
    tensor_trim(a.iter().map(|c| poly_rem(f, c, m)).collect())
}

fn tensor_sub(f: Fp, a: &Tensor, b: &Tensor) -> Tensor {
    tensor_trim(
        (0..a.len().max(b.len()))
            .map(|i| {
                sub(
                    f,
                    a.get(i).map(Vec::as_slice).unwrap_or(&[0]),
                    b.get(i).map(Vec::as_slice).unwrap_or(&[0]),
                )
            })
            .collect(),
    )
}

fn tensor_scale(f: Fp, a: &Tensor, c: &[u64], m: &[u64]) -> Tensor {
    tensor_trim(a.iter().map(|v| poly_mulmod(f, v, c, m)).collect())
}

pub(crate) fn tensor_mul(f: Fp, a: &Tensor, b: &Tensor, m: &[u64]) -> Tensor {
    let mut out = vec![vec![0]; a.len() + b.len() - 1];
    for (i, ai) in a.iter().enumerate() {
        for (j, bj) in b.iter().enumerate() {
            out[i + j] = add(f, &out[i + j], &poly_mulmod(f, ai, bj, m));
        }
    }
    tensor_trim(out)
}

fn tensor_divide(
    f: Fp,
    a: &Tensor,
    b: &Tensor,
    lead_inv: &[u64],
    m: &[u64],
    stats: &mut QueryStats,
) -> (Tensor, Tensor) {
    let mut r = a.clone();
    if r.len() < b.len() {
        return (vec![vec![0]], r);
    }
    let mut q = vec![vec![0]; r.len() - b.len() + 1];
    while !tensor_zero(&r) && r.len() >= b.len() {
        let shift = r.len() - b.len();
        let c = poly_mulmod(f, r.last().unwrap(), lead_inv, m);
        q[shift] = c.clone();
        for (i, v) in b.iter().enumerate() {
            r[shift + i] = sub(f, &r[shift + i], &poly_mulmod(f, &c, v, m));
        }
        let old_len = r.len();
        r = tensor_trim(r);
        stats.zero_leading_coefficients += old_len.saturating_sub(r.len());
    }
    (tensor_trim(q), r)
}

enum Inversion {
    Unit(Poly),
    Split(Poly, Poly),
}

fn invert_or_split(
    f: Fp,
    c: &[u64],
    m: &[u64],
    stats: &mut QueryStats,
) -> Result<Inversion, QueryError> {
    let (d, u, _) = xgcd(f, c, m);
    if d == [1] {
        stats.coefficient_inversions += 1;
        return Ok(Inversion::Unit(poly_rem(f, &u, m)));
    }
    if d.len() <= 1 || d.len() >= m.len() {
        return Err(QueryError::Internal(
            "zero or improper leading-coefficient split",
        ));
    }
    let (other, residual) = divide(f, m, &d);
    if !is_zero(&residual) || other.last() != Some(&1) {
        return Err(QueryError::Internal("support split does not divide"));
    }
    stats.splits += 1;
    Ok(Inversion::Split(d, other))
}

enum Block {
    Done(Leaf),
    Split(Poly, Poly),
}

fn solve_block(
    f: Fp,
    fpoly: &[u64],
    h: &Tensor,
    m: &[u64],
    stats: &mut QueryStats,
) -> Result<Block, QueryError> {
    stats.blocks_started += 1;
    let original_f: Tensor = fpoly.iter().map(|&v| vec![v]).collect();
    let original_h = tensor_reduce(f, h, m);
    stats.zero_leading_coefficients += h.len().saturating_sub(original_h.len());
    let (mut r0, mut r1) = (original_f.clone(), original_h.clone());
    let one = vec![vec![1]];
    let zero = vec![vec![0]];
    let (mut u0, mut u1) = (one.clone(), zero.clone());
    let (mut v0, mut v1) = (zero.clone(), one.clone());
    while !tensor_zero(&r1) {
        let lead_inverse = match invert_or_split(f, r1.last().unwrap(), m, stats)? {
            Inversion::Unit(i) => i,
            Inversion::Split(a, b) => return Ok(Block::Split(a, b)),
        };
        let (q, r2) = tensor_divide(f, &r0, &r1, &lead_inverse, m, stats);
        let u2 = tensor_sub(f, &u0, &tensor_mul(f, &q, &u1, m));
        let v2 = tensor_sub(f, &v0, &tensor_mul(f, &q, &v1, m));
        (r0, r1) = (r1, r2);
        (u0, u1) = (u1, u2);
        (v0, v1) = (v1, v2);
        stats.euclidean_steps += 1;
    }
    let lead_inverse = match invert_or_split(f, r0.last().unwrap(), m, stats)? {
        Inversion::Unit(i) => i,
        Inversion::Split(a, b) => return Ok(Block::Split(a, b)),
    };
    let gcd = tensor_scale(f, &r0, &lead_inverse, m);
    let bezout_f = tensor_scale(f, &u0, &lead_inverse, m);
    let bezout_h = tensor_scale(f, &v0, &lead_inverse, m);
    let (f_cofactor, f_residual) = tensor_divide(f, &original_f, &gcd, &[1], m, stats);
    let (h_cofactor, h_residual) = tensor_divide(f, &original_h, &gcd, &[1], m, stats);
    if !tensor_zero(&f_residual) || !tensor_zero(&h_residual) {
        return Err(QueryError::Internal(
            "terminal gcd does not divide both inputs",
        ));
    }
    Ok(Block::Done(Leaf {
        modulus: m.to_vec(),
        gcd,
        f_cofactor,
        h_cofactor,
        bezout_f,
        bezout_h,
    }))
}

fn validate_modulus(f: Fp, p: &[u64], name: &'static str) -> Result<(), QueryError> {
    if p.is_empty() || p.iter().any(|&x| x >= f.p) || trim(p.to_vec()) != p {
        return Err(QueryError::InvalidPolynomial(name));
    }
    if p.last() != Some(&1) {
        return Err(QueryError::Nonmonic(name));
    }
    if p.len() > 257 {
        return Err(QueryError::DegreeLimit);
    }
    if p == [1] {
        return Ok(());
    }
    if poly_gcd(f, p, &derivative(f, p)).len() != 1 {
        return Err(QueryError::Nonsquarefree(name));
    }
    if poly_powmod(f, &[0, 1], f.p, p) != poly_rem(f, &[0, 1], p) {
        return Err(QueryError::Nonsplit(name));
    }
    Ok(())
}

/// Exact split-algebra query on the two declared development fields.
/// No curve or signed-relation conclusion follows from this algebraic API.
pub fn solve(f: Fp, fpoly: &[u64], g: &[u64], h: &Tensor) -> Result<Certificate, QueryError> {
    if f.p != 97 && f.p != 2147483647 {
        return Err(QueryError::UnsupportedField);
    }
    validate_modulus(f, fpoly, "F")?;
    validate_modulus(f, g, "G")?;
    if h.is_empty()
        || h.iter()
            .any(|c| c.is_empty() || c.iter().any(|&v| v >= f.p) || trim(c.clone()) != *c)
        || tensor_trim(h.clone()) != *h
    {
        return Err(QueryError::NoncanonicalTensor);
    }
    if (fpoly == [1] || g == [1]) && !tensor_zero(h) {
        return Err(QueryError::NoncanonicalTensor);
    }
    if fpoly != [1] && h.len() >= fpoly.len() || g != [1] && h.iter().any(|c| c.len() >= g.len()) {
        return Err(QueryError::NoncanonicalTensor);
    }
    let mut stats = QueryStats::default();
    let mut leaves = Vec::new();
    let mut pending = if g == [1] {
        Vec::new()
    } else {
        vec![g.to_vec()]
    };
    while let Some(m) = pending.pop() {
        match solve_block(f, fpoly, h, &m, &mut stats)? {
            Block::Done(leaf) => leaves.push(leaf),
            Block::Split(a, b) => {
                pending.push(b);
                pending.push(a);
            }
        }
    }
    leaves.sort_by(|a, b| a.modulus.cmp(&b.modulus));
    let support_count = leaves
        .iter()
        .map(|leaf| (leaf.modulus.len() - 1) * (leaf.gcd.len() - 1))
        .sum();
    stats.output_coefficients = fpoly.len()
        + g.len()
        + h.iter().map(Vec::len).sum::<usize>()
        + leaves
            .iter()
            .map(|l| {
                l.modulus.len()
                    + [
                        &l.gcd,
                        &l.f_cofactor,
                        &l.h_cofactor,
                        &l.bezout_f,
                        &l.bezout_h,
                    ]
                    .iter()
                    .map(|t| t.iter().map(Vec::len).sum::<usize>())
                    .sum::<usize>()
            })
            .sum::<usize>();
    Ok(Certificate {
        p: f.p,
        f: fpoly.to_vec(),
        g: g.to_vec(),
        h: h.clone(),
        leaves,
        verdict_empty: support_count == 0,
        support_count,
        stats,
    })
}

/// Canonical reduction of a tensor modulo monic univariate F(x),G(t).
pub fn reduce_tensor(f: Fp, h: &Tensor, fpoly: &[u64], g: &[u64]) -> Tensor {
    if fpoly == [1] || g == [1] {
        return vec![vec![0]];
    }
    let h = tensor_reduce(f, h, g);
    let divisor: Tensor = fpoly.iter().map(|&c| vec![c]).collect();
    tensor_divide(f, &h, &divisor, &[1], g, &mut QueryStats::default()).1
}
