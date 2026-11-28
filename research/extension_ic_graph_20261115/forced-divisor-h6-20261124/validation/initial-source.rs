//! Opt-in norm family containing a chosen quadratic divisor. This constructs
//! equations and normalized candidate functions, not verified graph relations.
//! Map/word/allocation work is not completely instrumented; no native total.
use crate::cryptanalysis::{
    f4_fp::{self, Poly},
    jv_cover::{Fld, Fq, E2},
    residual_walk::{is_prime_u64, pow_mod},
};
use std::collections::BTreeMap;

const N: usize = 8;
type Numeric = Vec<E2>;
type Symbolic = BTreeMap<Vec<u32>, E2>;
type Outer = Vec<Symbolic>;

fn trim(mut a: Numeric) -> Numeric {
    while a.last() == Some(&E2::ZERO) {
        a.pop();
    }
    a
}
fn add(f: &Fq, a: &[E2], b: &[E2]) -> Numeric {
    trim(
        (0..a.len().max(b.len()))
            .map(|i| f.add(a.get(i).unwrap_or(&E2::ZERO), b.get(i).unwrap_or(&E2::ZERO)))
            .collect(),
    )
}
fn sub(f: &Fq, a: &[E2], b: &[E2]) -> Numeric {
    trim(
        (0..a.len().max(b.len()))
            .map(|i| f.sub(a.get(i).unwrap_or(&E2::ZERO), b.get(i).unwrap_or(&E2::ZERO)))
            .collect(),
    )
}
fn mul(f: &Fq, a: &[E2], b: &[E2]) -> Numeric {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut c = vec![E2::ZERO; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            c[i + j] = f.add(&c[i + j], &f.mul(x, y));
        }
    }
    trim(c)
}
fn divide(f: &Fq, a: &[E2], u: &[E2]) -> (Numeric, Numeric) {
    let mut r = trim(a.to_vec());
    let mut q = vec![E2::ZERO; r.len().saturating_sub(u.len()) + 1];
    while r.len() >= u.len() {
        let k = r.len() - u.len();
        let c = *r.last().unwrap();
        q[k] = c;
        for (j, b) in u.iter().enumerate() {
            r[j + k] = f.sub(&r[j + k], &f.mul(&c, b));
        }
        r = trim(r);
    }
    (trim(q), r)
}
fn constant(c: E2) -> Symbolic {
    if c == E2::ZERO {
        BTreeMap::new()
    } else {
        BTreeMap::from([(vec![0; N], c)])
    }
}
fn variable(real: usize, imag: usize) -> Symbolic {
    let mut a = vec![0; N];
    a[real] = 1;
    let mut b = vec![0; N];
    b[imag] = 1;
    BTreeMap::from([(a, E2::ONE), (b, E2([0, 1]))])
}
fn accumulate(f: &Fq, out: &mut Symbolic, m: Vec<u32>, c: E2) {
    let value = f.add(out.get(&m).unwrap_or(&E2::ZERO), &c);
    if value == E2::ZERO {
        out.remove(&m);
    } else {
        out.insert(m, value);
    }
}
fn sym_mul(f: &Fq, a: &Symbolic, b: &Symbolic) -> Symbolic {
    let mut out = BTreeMap::new();
    for (m, c) in a {
        for (n, d) in b {
            let e = m.iter().zip(n).map(|(x, y)| x + y).collect();
            accumulate(f, &mut out, e, f.mul(c, d));
        }
    }
    out
}
fn outer_mul(f: &Fq, a: &Outer, b: &Outer) -> Outer {
    let mut out = vec![BTreeMap::new(); a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            for (m, c) in sym_mul(f, x, y) {
                accumulate(f, &mut out[i + j], m, c);
            }
        }
    }
    out
}
fn outer_sub(f: &Fq, a: &Outer, b: &Outer) -> Outer {
    let mut out = a.clone();
    out.resize(a.len().max(b.len()), BTreeMap::new());
    for (i, row) in b.iter().enumerate() {
        for (m, c) in row {
            accumulate(f, &mut out[i], m.clone(), f.neg(c));
        }
    }
    out
}
fn fixed(a: &[E2]) -> Outer {
    a.iter().copied().map(constant).collect()
}

#[derive(Clone, Debug)]
pub struct Family {
    p: u64,
    h: Numeric,
    u: Numeric,
    v: Numeric,
    equations: Vec<Poly>,
}
/// A norm-valid candidate still needs complete support, sign, group and graph
/// checks at the existing principal relation boundary.
#[derive(Clone, Debug)]
pub struct CandidateFunction {
    pub a: Numeric,
    pub c: Numeric,
    pub norm: Numeric,
    pub quotient: Numeric,
}
impl Family {
    pub fn new(f: &Fq, h: &[E2], u: &[E2], v: &[E2]) -> Result<Self, &'static str> {
        if !(5..=1511).contains(&f.p)
            || !is_prime_u64(f.p)
            || h.len() != 8
            || h.last() != Some(&E2::ONE)
            || u.len() != 3
            || u.last() != Some(&E2::ONE)
            || u.iter().any(|e| !e.in_fp())
            || v.len() > 2
            || h.iter()
                .chain(u)
                .chain(v)
                .any(|e| e.0.iter().any(|&x| x >= f.p))
        {
            return Err("canonical monic degree7/degree2 odd-prime domain required");
        }
        let disc = (u[1].0[0] as u128 * u[1].0[0] as u128 + 4 * f.p as u128
            - (4 * u[0].0[0] as u128) % f.p as u128)
            % f.p as u128;
        if disc == 0 || pow_mod(disc as u64, (f.p - 1) / 2, f.p) != f.p - 1 {
            return Err("irreducible Fp quadratic required");
        }
        let (w, r) = divide(f, &sub(f, h, &mul(f, v, v)), u);
        if !r.is_empty() || w.len() != 6 || w.last() != Some(&E2::ONE) {
            return Err("original Mumford identity/exact division required");
        }
        let c = vec![variable(0, 1), constant(E2::ONE)];
        let t = vec![variable(2, 3), variable(4, 5), variable(6, 7)];
        let ut2 = outer_mul(f, &fixed(u), &outer_mul(f, &t, &t));
        let c2w = outer_mul(f, &outer_mul(f, &c, &c), &fixed(&w));
        let tcv = outer_mul(f, &outer_mul(f, &t, &c), &fixed(v));
        let twice = outer_mul(f, &tcv, &vec![constant(f.from_fp(2))]);
        let s = outer_sub(f, &outer_sub(f, &ut2, &twice), &c2w);
        if s.len() != 8 || s[7] != constant(f.neg(&E2::ONE)) {
            return Err("internal leading coefficient invariant");
        }
        let equations: Vec<Poly> = s[..7]
            .iter()
            .map(|row| {
                row.iter()
                    .filter_map(|(m, c)| (c.0[1] != 0).then(|| (m.clone(), c.0[1])))
                    .collect()
            })
            .collect();
        if equations
            .iter()
            .flatten()
            .any(|(m, _)| m.len() != N || m.iter().sum::<u32>() > 2)
        {
            return Err("internal quadratic invariant");
        }
        Ok(Self {
            p: f.p,
            h: h.to_vec(),
            u: u.to_vec(),
            v: trim(v.to_vec()),
            equations,
        })
    }
    pub fn polynomials(&self) -> &[Poly] {
        &self.equations
    }
    fn coordinates(&self, z: &[u64]) -> Result<(), &'static str> {
        if z.len() != N || z.iter().any(|&x| x >= self.p) {
            Err("eight canonical Fp coordinates required")
        } else {
            Ok(())
        }
    }
    pub fn evaluate(&self, z: &[u64]) -> Result<Vec<u64>, &'static str> {
        self.coordinates(z)?;
        Ok(self
            .equations
            .iter()
            .map(|p| f4_fp::eval(p, z, self.p))
            .collect())
    }
    pub fn jacobian(&self, z: &[u64]) -> Result<Vec<Vec<u64>>, &'static str> {
        self.coordinates(z)?;
        let mut rows = vec![vec![0; N]; 7];
        for (i, equation) in self.equations.iter().enumerate() {
            for (m, c) in equation {
                for (j, &exponent) in m.iter().enumerate() {
                    if exponent == 0 {
                        continue;
                    }
                    let mut v = (*c as u128 * exponent as u128 % self.p as u128) as u64;
                    for (k, &e) in m.iter().enumerate() {
                        let e = e - u32::from(k == j);
                        v = (v as u128 * pow_mod(z[k], e as u64, self.p) as u128 % self.p as u128)
                            as u64;
                    }
                    rows[i][j] = ((rows[i][j] as u128 + v as u128) % self.p as u128) as u64;
                }
            }
        }
        Ok(rows)
    }
    /// Fix one development fiber coordinate; retain every remaining equation.
    pub fn specialize(&self, coordinate: usize, value: u64) -> Result<Vec<Poly>, &'static str> {
        if coordinate >= N || value >= self.p {
            return Err("fiber coordinate/value");
        }
        Ok(self
            .equations
            .iter()
            .map(|p| {
                let mut out = BTreeMap::<Vec<u32>, u64>::new();
                for (m, c) in p {
                    let mut n = m.clone();
                    let exponent = n.remove(coordinate);
                    let v = (*c as u128 * pow_mod(value, exponent as u64, self.p) as u128
                        % self.p as u128) as u64;
                    let entry = out.entry(n).or_default();
                    *entry = ((*entry as u128 + v as u128) % self.p as u128) as u64;
                }
                out.into_iter().filter(|(_, c)| *c != 0).collect()
            })
            .collect())
    }
    /// Independent numeric construction uses the direct norm and exact
    /// division, rather than the symbolic quotient formula used to compile.
    pub fn evaluate_function(&self, f: &Fq, z: &[u64]) -> Result<CandidateFunction, &'static str> {
        self.coordinates(z)?;
        if f.p != self.p {
            return Err("field mismatch");
        }
        let c = vec![E2([z[0], z[1]]), E2::ONE];
        let t = vec![E2([z[2], z[3]]), E2([z[4], z[5]]), E2([z[6], z[7]])];
        let a = sub(f, &mul(f, &self.u, &t), &mul(f, &c, &self.v));
        let norm = sub(f, &mul(f, &a, &a), &mul(f, &mul(f, &c, &c), &self.h));
        let (quotient, r) = divide(f, &norm, &self.u);
        if !r.is_empty() || quotient.len() != 8 || quotient[7] != f.neg(&E2::ONE) {
            return Err("numeric norm division invariant");
        }
        Ok(CandidateFunction {
            a,
            c,
            norm,
            quotient,
        })
    }
    pub fn instantiate(&self, f: &Fq, z: &[u64]) -> Result<CandidateFunction, &'static str> {
        let value = self.evaluate_function(f, z)?;
        let root = f.neg(&value.c[0]);
        let at_root = self
            .u
            .iter()
            .rev()
            .fold(E2::ZERO, |a, c| f.add(&f.mul(&a, &root), c));
        if at_root == E2::ZERO {
            return Err("C is not invertible on forced divisor");
        }
        if self.evaluate(z)?.iter().any(|&v| v != 0)
            || value.quotient.iter().any(|v| !v.in_fp())
            || value.norm.iter().any(|v| !v.in_fp())
        {
            return Err("norm is not in Fp[x]");
        }
        Ok(value)
    }
    /// Normalize a retained positive function, preserving its divisor and
    /// its Fp norm. This is development input, not an unknown target answer.
    pub fn recover_parameters(&self, f: &Fq, a: &[E2], c: &[E2]) -> Result<Vec<u64>, &'static str> {
        if f.p != self.p
            || a.len() > 5
            || c.len() != 2
            || c[1] == E2::ZERO
            || a.iter().chain(c).any(|v| v.0.iter().any(|&x| x >= self.p))
            || !f.sq(&c[1]).in_fp()
        {
            return Err("retained function/gauge domain");
        }
        let inv = f.inv(&c[1]);
        let a: Vec<_> = a.iter().map(|v| f.mul(v, &inv)).collect();
        let c: Vec<_> = c.iter().map(|v| f.mul(v, &inv)).collect();
        let (mut t, r) = divide(f, &add(f, &a, &mul(f, &c, &self.v)), &self.u);
        if !r.is_empty() || t.len() > 3 {
            return Err("retained function does not contain signed divisor");
        }
        t.resize(3, E2::ZERO);
        let z = vec![
            c[0].0[0], c[0].0[1], t[0].0[0], t[0].0[1], t[1].0[0], t[1].0[1], t[2].0[0], t[2].0[1],
        ];
        let check = self.instantiate(f, &z)?;
        if trim(a) != check.a || c != check.c {
            return Err("normalized function replay");
        }
        Ok(z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (Fq, Family) {
        let f = Fq::new(11);
        let u = vec![E2::ONE, E2::ZERO, E2::ONE];
        let v = vec![E2::ONE, E2::ONE];
        let w = [3, 2, 0, 0, 0, 1].map(|x| f.from_fp(x));
        let h = add(&f, &mul(&f, &v, &v), &mul(&f, &u, &w));
        let family = Family::new(&f, &h, &u, &v).unwrap();
        (f, family)
    }
    #[test]
    fn compiled_quadrics_match_direct_norm_division() {
        let (f, g) = fixture();
        assert_eq!(g.polynomials().len(), 7);
        for seed in 0..16 {
            let z: Vec<_> = (0..8).map(|j| (seed * 7 + j * 3) % 11).collect();
            let value = g.evaluate_function(&f, &z).unwrap();
            assert_eq!(
                g.evaluate(&z).unwrap(),
                value.quotient[..7]
                    .iter()
                    .map(|v| v.0[1])
                    .collect::<Vec<_>>()
            );
            assert_eq!(value.norm, mul(&f, &g.u, &value.quotient));
        }
    }
    #[test]
    fn real_and_imaginary_gauge_replay_the_same_positive_function() {
        let (f, g) = fixture();
        let z = vec![2, 0, 3, 0, 1, 0, 4, 0];
        let value = g.instantiate(&f, &z).unwrap();
        for gamma in [f.from_fp(2), E2([0, 1])] {
            let a: Vec<_> = value.a.iter().map(|v| f.mul(v, &gamma)).collect();
            let c: Vec<_> = value.c.iter().map(|v| f.mul(v, &gamma)).collect();
            assert_eq!(g.recover_parameters(&f, &a, &c).unwrap(), z);
        }
    }
    #[test]
    fn analytic_derivatives_and_specialization_match_numeric_values() {
        let (_, g) = fixture();
        let z = vec![2, 3, 4, 5, 6, 7, 8, 9];
        let j = g.jacobian(&z).unwrap();
        for k in 0..8 {
            let mut plus = z.clone();
            let mut minus = z.clone();
            plus[k] = (plus[k] + 1) % 11;
            minus[k] = (minus[k] + 10) % 11;
            let hi = g.evaluate(&plus).unwrap();
            let lo = g.evaluate(&minus).unwrap();
            for i in 0..7 {
                assert_eq!(j[i][k], ((hi[i] + 11 - lo[i]) * 6) % 11);
            }
            let specialized = g.specialize(k, z[k]).unwrap();
            let mut reduced = z.clone();
            reduced.remove(k);
            for (a, b) in specialized.iter().zip(g.evaluate(&z).unwrap()) {
                assert_eq!(f4_fp::eval(a, &reduced, 11), b);
            }
        }
    }
    #[test]
    fn invalid_mumford_gauge_domain_and_nonunit_c_are_refused() {
        let (f, g) = fixture();
        assert!(Family::new(&f, &g.h, &g.u, &[]).is_err());
        assert!(Family::new(&f, &g.h, &[E2::ZERO, E2::ZERO, E2::ONE], &g.v).is_err());
        let mut bad = g.h.clone();
        bad[7] = f.from_fp(2);
        assert!(Family::new(&f, &bad, &g.u, &g.v).is_err());
        assert!(g.evaluate(&[0; 7]).is_err());
        assert!(g.evaluate(&[11; 8]).is_err());
        let value = g.instantiate(&f, &[2, 0, 3, 0, 1, 0, 4, 0]).unwrap();
        let gamma = E2([1, 1]);
        let c: Vec<_> = value.c.iter().map(|x| f.mul(x, &gamma)).collect();
        assert!(g.recover_parameters(&f, &value.a, &c).is_err());
        let mut nonunit = None;
        for a in 0..11 {
            for b in 0..11 {
                let c = E2([a, b]);
                if f.add(&f.sq(&c), &E2::ONE) == E2::ZERO {
                    nonunit = Some(c);
                }
            }
        }
        let c = nonunit.unwrap();
        let mut z = vec![2, 0, 3, 0, 1, 0, 4, 0];
        z[0] = c.0[0];
        z[1] = c.0[1];
        assert_eq!(
            g.instantiate(&f, &z).unwrap_err(),
            "C is not invertible on forced divisor"
        );
    }
}
