//! Equivalent finite-base Semaev systems for public small-prime experiments.
//! Symmetrization handles the off-diagonal first pair. Its diagonal and identity
//! prefix branches must be supplied and charged by the caller.
use super::f4_fp::{Ordering, Poly};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Case {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub target_x: u64,
    pub xs: Vec<u64>,
    pub arity: usize,
    pub encoding: String,
    pub names: Vec<String>,
    pub equations: Vec<Poly>,
}
#[derive(Clone, Copy)]
struct R {
    p: u64,
    n: usize,
}
impl R {
    fn norm(self, a: Poly) -> Poly {
        // Solver normalization also makes a polynomial monic. Algebraic
        // expressions must preserve their coefficients until equations exist.
        let mut coefficients = std::collections::BTreeMap::new();
        for (powers, c) in a {
            let value = coefficients.entry(powers).or_insert(0u64);
            *value = (*value + c) % self.p;
        }
        let mut out: Poly = coefficients.into_iter().filter(|(_, c)| *c != 0).collect();
        out.sort_by(|a, b| super::groebner_f4::cmp_monomial(&b.0, &a.0, Ordering::Grevlex));
        out
    }
    fn c(self, c: u64) -> Poly {
        if c % self.p == 0 {
            vec![]
        } else {
            vec![(vec![0; self.n], c % self.p)]
        }
    }
    fn v(self, i: usize) -> Poly {
        let mut e = vec![0; self.n];
        e[i] = 1;
        vec![(e, 1)]
    }
    fn add(self, a: &Poly, b: &Poly) -> Poly {
        self.norm(a.iter().chain(b).cloned().collect())
    }
    fn neg(self, a: &Poly) -> Poly {
        a.iter()
            .map(|(e, c)| (e.clone(), (self.p - c) % self.p))
            .collect()
    }
    fn sub(self, a: &Poly, b: &Poly) -> Poly {
        self.add(a, &self.neg(b))
    }
    fn mul(self, a: &Poly, b: &Poly) -> Poly {
        let mut c = vec![];
        for (e, x) in a {
            for (f, y) in b {
                c.push((
                    e.iter().zip(f).map(|(x, y)| x + y).collect(),
                    x * y % self.p,
                ));
            }
        }
        self.norm(c)
    }
    fn sc(self, a: &Poly, c: u64) -> Poly {
        self.mul(a, &self.c(c))
    }
    fn sq(self, a: &Poly) -> Poly {
        self.mul(a, a)
    }
    fn s3_sym(self, u: &Poly, v: &Poly, z: &Poly, a: u64, b: u64) -> Poly {
        let lead = self.mul(&self.sub(&self.sq(u), &self.sc(v, 4)), &self.sq(z));
        let mid = self.sc(
            &self.mul(
                &self.add(
                    &self.mul(u, &self.add(v, &self.c(a))),
                    &self.c(2 * b % self.p),
                ),
                z,
            ),
            2,
        );
        let tail = self.sub(
            &self.sq(&self.sub(v, &self.c(a))),
            &self.sc(u, 4 * b % self.p),
        );
        self.add(&self.sub(&lead, &mid), &tail)
    }
    fn s3(self, x: &Poly, y: &Poly, z: &Poly, a: u64, b: u64) -> Poly {
        self.s3_sym(&self.add(x, y), &self.mul(x, y), z, a, b)
    }
    fn membership(self, x: &Poly, xs: &[u64]) -> Poly {
        xs.iter()
            .fold(self.c(1), |p, a| self.mul(&p, &self.sub(x, &self.c(*a))))
    }
}
fn pow(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut r = 1;
    while n > 0 {
        if n & 1 != 0 {
            r = r * a % p;
        }
        a = a * a % p;
        n >>= 1;
    }
    r
}
fn interpolation(xs: &[u64], p: u64) -> Vec<u64> {
    let mut out = vec![0; xs.len()];
    for (i, &x) in xs.iter().enumerate() {
        let mut q = vec![1];
        let mut d = 1;
        for j in 0..xs.len() {
            if i == j {
                continue;
            }
            let mut n = vec![0; q.len() + 1];
            for (k, &c) in q.iter().enumerate() {
                n[k] = (n[k] + p - c * j as u64 % p) % p;
                n[k + 1] = (n[k + 1] + c) % p;
            }
            q = n;
            d = d * (i as u64 + p - j as u64) % p;
        }
        let s = x * pow(d, p - 2, p) % p;
        for k in 0..q.len() {
            out[k] = (out[k] + s * q[k]) % p;
        }
    }
    out
}
pub fn build(p: u64, a: u64, b: u64, target_x: u64, xs: &[u64], m: usize, encoding: &str) -> Case {
    assert!((2..=4).contains(&m) && p > xs.len() as u64 && xs.len() >= 2);
    assert!(xs.windows(2).all(|x| x[0] < x[1]));
    let bits = (usize::BITS - (xs.len() - 1).leading_zeros()) as usize;
    let mut names: Vec<String> = if encoding == "symmetric" {
        vec!["u".into(), "v".into()]
    } else {
        vec!["x1".into(), "x2".into()]
    };
    names.extend((2..m).map(|i| format!("x{}", i + 1)));
    names.extend((0..m - 2).map(|i| format!("t{}", i + 1)));
    let core = names.len();
    if encoding == "chain" {
        for i in 0..m {
            for j in 0..xs.len() - 1 {
                names.push(format!("c{i}_{j}"));
            }
        }
    }
    if encoding == "bits" {
        for i in 0..m {
            names.push(format!("s{i}"));
            for j in 0..bits {
                names.push(format!("b{i}_{j}"));
            }
        }
    }
    let r = R { p, n: names.len() };
    let mut equations = vec![];
    if encoding == "symmetric" {
        // Remainder of the squarefree membership polynomial modulo T²-uT+v.
        let mut f = vec![1u64];
        for &x in xs {
            let mut next = vec![0; f.len() + 1];
            for (i, &c) in f.iter().enumerate() {
                next[i] = (next[i] + p - c * x % p) % p;
                next[i + 1] = (next[i + 1] + c) % p;
            }
            f = next;
        }
        let (mut c0, mut c1) = (r.c(0), r.c(0));
        for c in f.into_iter().rev() {
            let n0 = r.sub(&r.c(c), &r.mul(&r.v(1), &c1));
            let n1 = r.add(&c0, &r.mul(&r.v(0), &c1));
            c0 = n0;
            c1 = n1;
        }
        equations.push(c0);
        equations.push(c1);
    }
    for i in 0..m {
        if encoding == "symmetric" && i < 2 {
            continue;
        }
        let x = r.v(i);
        match encoding {
            "chain" => {
                let mut prior = r.c(1);
                for (j, &v) in xs.iter().enumerate() {
                    let product = r.mul(&prior, &r.sub(&x, &r.c(v)));
                    if j + 1 == xs.len() {
                        equations.push(product);
                    } else {
                        let z = r.v(core + i * (xs.len() - 1) + j);
                        equations.push(r.sub(&z, &product));
                        prior = z;
                    }
                }
            }
            "bits" => {
                let offset = core + i * (bits + 1);
                let s = r.v(offset);
                let mut bit_sum = r.c(0);
                for j in 0..bits {
                    let bit = r.v(offset + j + 1);
                    equations.push(r.mul(&bit, &r.sub(&bit, &r.c(1))));
                    bit_sum = r.add(&bit_sum, &r.sc(&bit, 1u64 << j));
                }
                equations.push(r.sub(&s, &bit_sum));
                if !xs.len().is_power_of_two() {
                    equations.push(r.membership(&s, &(0..xs.len() as u64).collect::<Vec<_>>()));
                }
                let coefficients = interpolation(xs, p);
                let mut expr = r.c(0);
                for c in coefficients.into_iter().rev() {
                    expr = r.add(&r.mul(&expr, &s), &r.c(c));
                }
                equations.push(r.sub(&x, &expr));
            }
            _ => equations.push(r.membership(&x, xs)),
        }
    }
    let first_z = if m == 2 { r.c(target_x) } else { r.v(m) };
    equations.push(if encoding == "symmetric" {
        r.s3_sym(&r.v(0), &r.v(1), &first_z, a, b)
    } else {
        r.s3(&r.v(0), &r.v(1), &first_z, a, b)
    });
    for i in 2..m {
        let left = r.v(m + i - 2);
        let right = r.v(i);
        let z = if i + 1 == m {
            r.c(target_x)
        } else {
            r.v(m + i - 1)
        };
        equations.push(r.s3(&left, &right, &z, a, b));
    }
    Case {
        p,
        a,
        b,
        target_x,
        xs: xs.to_vec(),
        arity: m,
        encoding: encoding.into(),
        names,
        equations,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pair_membership_is_exact_in_both_orders() {
        let xs = [1, 3, 5, 8];
        let c = build(59, 1, 13, 7, &xs, 2, "symmetric");
        for u in 0..59 {
            for v in 0..59 {
                let admitted = c.equations[..2]
                    .iter()
                    .all(|f| super::super::f4_fp::eval(f, &[u, v], 59) == 0);
                let expected = (0..xs.len()).any(|i| {
                    (i + 1..xs.len()).any(|j| (xs[i] + xs[j]) % 59 == u && xs[i] * xs[j] % 59 == v)
                });
                assert_eq!(admitted, expected);
            }
        }
    }
}
