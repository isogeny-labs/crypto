//! Exact, opt-in elimination of Im(mu) from the Nagao six-quadric system.
//! The returned roots are lifted and checked against every original equation.
use crate::cryptanalysis::{
    f4_fp::{self, F4Options, Ordering, Poly},
    jv_bitops as work,
    jv_cover::{self as cover, ZeroDimStats},
    residual_walk::{inv_mod, is_prime_u64},
};
use rand::rngs::StdRng;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Clone, Serialize)]
pub struct Audit {
    pub original: Vec<Poly>,
    pub replacement: Poly,
    pub reduced_sha256: String,
    pub lifted_roots: Vec<Vec<u64>>,
    pub stats: ZeroDimStats,
}

const N: usize = 6;
const MAX_TERMS: usize = 4096;

fn mul(a: u64, b: u64, p: u64) -> u64 {
    work::hook_mul(p);
    (a as u128 * b as u128 % p as u128) as u64
}
fn add(a: u64, b: u64, p: u64) -> u64 {
    work::hook_add(p);
    (a + b) % p
}
// Unlike f4_fp::normalise, this preserves the scalar of a polynomial.
fn collect(terms: Poly, p: u64) -> Result<Poly, &'static str> {
    if terms.len() > MAX_TERMS {
        return Err("polynomial expansion cap");
    }
    let mut out = BTreeMap::new();
    for (e, c) in terms {
        work::words(e.len() as u64 + 2);
        work::probe(1);
        let x = out.entry(e).or_insert(0);
        *x = add(*x, c, p);
    }
    Ok(out.into_iter().filter(|(_, c)| *c != 0).collect())
}
fn product(a: &Poly, b: &Poly, p: u64) -> Result<Poly, &'static str> {
    if a.len().saturating_mul(b.len()) > MAX_TERMS {
        return Err("polynomial expansion cap");
    }
    let mut terms = Vec::new();
    for (e, c) in a {
        for (f, d) in b {
            terms.push((
                e.iter().zip(f).map(|(x, y)| x + y).collect(),
                mul(*c, *d, p),
            ));
            work::words(2 * (N as u64 - 1));
        }
    }
    collect(terms, p)
}

/// A checked substitution d = replacement(a0,b0,a1,b1,c0).
/// No varying denominator, sampling, or parameter specialization is used.
pub struct UnitReduction {
    p: u64,
    original: Vec<Poly>,
    reduced: Vec<Poly>,
    replacement: Poly,
}
impl UnitReduction {
    pub fn new(input: &[Poly], p: u64) -> Result<Self, &'static str> {
        if !(5..1u64 << 63).contains(&p) || !is_prime_u64(p) {
            return Err("unsupported field");
        }
        if input.len() != N
            || input.iter().any(|f| {
                f.len() > 28
                    || f.iter().any(|(e, c)| {
                        e.len() != N
                            || e.iter().any(|&x| x > 2)
                            || e.iter().sum::<u32>() > 2
                            || *c == 0
                            || *c >= p
                    })
            })
        {
            return Err("expected six quadrics");
        }
        let pivot = &input[N - 1];
        let mut coefficient = 0;
        for (e, c) in pivot {
            if e[N - 1] != 0 {
                if e[N - 1] != 1 || e[..N - 1].iter().any(|&x| x != 0) {
                    return Err("non-unit pivot");
                }
                coefficient = add(coefficient, *c, p);
            }
        }
        if coefficient == 0 {
            return Err("zero pivot");
        }
        let inverse = inv_mod(coefficient, p);
        let replacement = collect(
            pivot
                .iter()
                .filter(|(e, _)| e[N - 1] == 0)
                .map(|(e, c)| (e[..N - 1].to_vec(), mul(p - *c, inverse, p)))
                .collect(),
            p,
        )?;
        let powers = [
            vec![(vec![0; N - 1], 1)],
            replacement.clone(),
            product(&replacement, &replacement, p)?,
        ];
        let mut reduced = Vec::new();
        for f in &input[..N - 1] {
            let mut terms = Vec::new();
            for (e, c) in f {
                for (r, d) in &powers[e[N - 1] as usize] {
                    terms.push((
                        e[..N - 1].iter().zip(r).map(|(x, y)| x + y).collect(),
                        mul(*c, *d, p),
                    ));
                    work::words(2 * (N as u64 - 1));
                }
            }
            let exact = collect(terms, p)?;
            // A nonzero constant survives and correctly makes the system empty.
            let f = f4_fp::normalise(&exact, p, Ordering::Grevlex);
            if !f.is_empty() {
                reduced.push(f);
            }
        }
        Ok(Self {
            p,
            original: input.to_vec(),
            reduced,
            replacement,
        })
    }
    pub fn polynomials(&self) -> &[Poly] {
        &self.reduced
    }
    pub fn lift(&self, reduced: &[u64]) -> Result<Vec<u64>, &'static str> {
        if reduced.len() != N - 1 || reduced.iter().any(|&x| x >= self.p) {
            return Err("noncanonical reduced root");
        }
        let mut root = reduced.to_vec();
        root.push(f4_fp::eval(&self.replacement, reduced, self.p));
        if self
            .original
            .iter()
            .any(|f| f4_fp::eval(f, &root, self.p) != 0)
        {
            return Err("original-equation replay");
        }
        Ok(root)
    }
}

/// Setup, five-variable solve, lifting and original-equation replay all execute
/// inside this call. Solver caps are returned unchanged; invalid lifts refuse.
pub fn solve_audited(
    input: &[Poly],
    p: u64,
    opts: &F4Options,
    rng: &mut StdRng,
) -> Result<(Vec<Vec<u64>>, ZeroDimStats, Audit), &'static str> {
    let reduction = UnitReduction::new(input, p)?;
    let reduced_sha256 = format!(
        "{:x}",
        Sha256::digest(
            serde_json::to_vec(reduction.polynomials()).map_err(|_| "reduced serialization")?
        )
    );
    let (roots, stats) = cover::solve_zero_dim(reduction.polynomials(), N - 1, p, opts, rng);
    let roots = roots
        .iter()
        .map(|x| reduction.lift(x))
        .collect::<Result<Vec<_>, _>>()?;
    let audit = Audit {
        original: input.to_vec(),
        replacement: reduction.replacement.clone(),
        reduced_sha256,
        lifted_roots: roots.clone(),
        stats: stats.clone(),
    };
    Ok((roots, stats, audit))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn term(i: usize, degree: u32, c: u64) -> (Vec<u32>, u64) {
        let mut e = vec![0; 6];
        e[i] = degree;
        (e, c)
    }
    fn fixture(p: u64) -> Vec<Poly> {
        let mut out: Vec<_> = (0..5)
            .map(|i| vec![term(i, 2, 1), term(i, 1, p - 1)])
            .collect();
        out[0].push(term(5, 2, 1));
        let mut ab = vec![0; 6];
        ab[2] = 1;
        ab[3] = 1;
        out.push(vec![term(5, 1, 2), (ab, p - 2), (vec![0; 6], 3)]);
        out
    }
    // Independent, deliberately simple evaluator for exhaustive algebra checks.
    fn eval(f: &Poly, x: &[u64], p: u64) -> u64 {
        let mut value = 0;
        for (e, c) in f {
            let mut t = *c;
            for (xi, ei) in x.iter().zip(e) {
                for _ in 0..*ei {
                    t = t * xi % p;
                }
            }
            value = (value + t) % p;
        }
        value
    }
    #[test]
    fn exhaustive_bijection_including_empty_system() {
        for p in [5, 7] {
            for empty in [false, true] {
                let mut original = fixture(p);
                if empty {
                    original[1] = vec![(vec![0; 6], 1)];
                }
                let r = UnitReduction::new(&original, p).unwrap();
                let mut full = Vec::new();
                for mut code in 0..p.pow(6) {
                    let x: Vec<_> = (0..6)
                        .map(|_| {
                            let t = code % p;
                            code /= p;
                            t
                        })
                        .collect();
                    if original.iter().all(|f| eval(f, &x, p) == 0) {
                        full.push(x);
                    }
                }
                let mut lifted = Vec::new();
                for mut code in 0..p.pow(5) {
                    let x: Vec<_> = (0..5)
                        .map(|_| {
                            let t = code % p;
                            code /= p;
                            t
                        })
                        .collect();
                    if r.polynomials().iter().all(|f| eval(f, &x, p) == 0) {
                        lifted.push(r.lift(&x).unwrap());
                    }
                }
                full.sort();
                lifted.sort();
                assert_eq!(full, lifted);
                if empty {
                    assert!(full.is_empty());
                }
            }
        }
    }
    #[test]
    fn rejects_bad_pivots_fields_and_roots() {
        let p = 7;
        let original = fixture(p);
        let r = UnitReduction::new(&original, p).unwrap();
        assert!(r.lift(&[0; 4]).is_err());
        assert!(r.lift(&[p; 5]).is_err());
        assert!(r.lift(&[0; 5]).is_err());
        for bad_p in [0, 1, 2, 4, 9, u64::MAX] {
            assert!(UnitReduction::new(&original, bad_p).is_err());
        }
        let mut bad = original.clone();
        bad[5].push(term(5, 2, 1));
        assert!(UnitReduction::new(&bad, p).is_err());
        bad = original.clone();
        bad[5].retain(|(e, _)| e[5] == 0);
        assert!(UnitReduction::new(&bad, p).is_err());
        bad = original;
        bad[0][0].0[0] = u32::MAX;
        assert!(UnitReduction::new(&bad, p).is_err());
    }
}
