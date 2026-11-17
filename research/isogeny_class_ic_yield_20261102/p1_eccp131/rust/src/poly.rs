//! Sparse multivariate polynomials over Fp, monomials packed as 8-bit exponents in a u128
//! (at most 16 variables, exponent <= 255).  The position of a variable in the packing is
//! its place in the monomial order (degrevlex: the variable at the highest position is the
//! smallest).

use crate::fp::Fp;
use std::collections::HashMap;

pub type Mono = u128;

pub fn mono_deg(m: Mono) -> u32 {
    m.to_le_bytes().iter().map(|&b| b as u32).sum()
}

pub fn mono_exp(m: Mono, pos: usize) -> u32 {
    ((m >> (8 * pos)) & 0xff) as u32
}

/// Ascending degrevlex comparison of two monomials of the same degree over `n` positions.
pub fn cmp_degrevlex(a: Mono, b: Mono, n: usize) -> std::cmp::Ordering {
    use std::cmp::Ordering::*;
    for i in (0..n).rev() {
        let (x, y) = (mono_exp(a, i), mono_exp(b, i));
        if x != y {
            // the monomial with the larger exponent in the smallest variable is smaller
            return if x > y { Less } else { Greater };
        }
    }
    Equal
}

#[derive(Clone, Debug, Default)]
pub struct Poly {
    pub t: HashMap<Mono, Fp>,
}

impl Poly {
    pub fn zero() -> Poly {
        Poly { t: HashMap::new() }
    }
    pub fn constant(c: Fp) -> Poly {
        let mut p = Poly::zero();
        if !c.is_zero() {
            p.t.insert(0, c);
        }
        p
    }
    pub fn var(pos: usize) -> Poly {
        let mut p = Poly::zero();
        p.t.insert(1u128 << (8 * pos), Fp::one());
        p
    }
    pub fn add(&self, o: &Poly) -> Poly {
        let mut r = self.clone();
        for (m, c) in &o.t {
            let e = r.t.entry(*m).or_insert(Fp::ZERO);
            *e = e.add(*c);
        }
        r.t.retain(|_, c| !c.is_zero());
        r
    }
    pub fn neg(&self) -> Poly {
        Poly { t: self.t.iter().map(|(m, c)| (*m, c.neg())).collect() }
    }
    pub fn sub(&self, o: &Poly) -> Poly {
        self.add(&o.neg())
    }
    pub fn scale(&self, k: Fp) -> Poly {
        let mut r = Poly::zero();
        if k.is_zero() {
            return r;
        }
        for (m, c) in &self.t {
            r.t.insert(*m, c.mul(k));
        }
        r
    }
    pub fn mul(&self, o: &Poly) -> Poly {
        let mut r = Poly::zero();
        for (m1, c1) in &self.t {
            for (m2, c2) in &o.t {
                let e = r.t.entry(m1 + m2).or_insert(Fp::ZERO);
                *e = e.add(c1.mul(*c2));
            }
        }
        r.t.retain(|_, c| !c.is_zero());
        r
    }
    pub fn degree(&self) -> u32 {
        self.t.keys().map(|&m| mono_deg(m)).max().unwrap_or(0)
    }
    /// Evaluate at `vals[pos]`.
    pub fn eval(&self, vals: &[Fp]) -> Fp {
        let mut s = Fp::ZERO;
        for (m, c) in &self.t {
            let mut term = *c;
            for (pos, v) in vals.iter().enumerate() {
                for _ in 0..mono_exp(*m, pos) {
                    term = term.mul(*v);
                }
            }
            s = s.add(term);
        }
        s
    }
    /// Terms sorted ascending by (degree, degrevlex) over `n` positions.
    pub fn sorted_terms(&self, n: usize) -> Vec<(Mono, Fp)> {
        let mut v: Vec<(Mono, Fp)> = self.t.iter().map(|(m, c)| (*m, *c)).collect();
        v.sort_by(|a, b| {
            mono_deg(a.0).cmp(&mono_deg(b.0)).then(cmp_degrevlex(a.0, b.0, n))
        });
        v
    }
}
