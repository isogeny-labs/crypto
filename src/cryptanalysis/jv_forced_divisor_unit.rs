//! Exact global H6 substitution c0.im=t2.re*t2.im+constant/2.
//! The constant coefficient -2 has no exceptional branch in the odd-prime
//! domain. Coordinate/map/allocation work is not fully instrumented.
use super::{f4_fp::Poly, jv_forced_divisor::Family};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct CoefficientWork {
    pub products: u64,
    pub additions: u64,
}
impl CoefficientWork {
    fn mul(&mut self, a: u64, b: u64, p: u64) -> u64 {
        self.products += 1;
        a * b % p
    }
    fn add(&mut self, a: u64, b: u64, p: u64) -> u64 {
        self.additions += 1;
        (a + b) % p
    }
    fn collect(&mut self, terms: Poly, p: u64) -> Poly {
        let mut out = BTreeMap::new();
        for (e, c) in terms {
            let entry = out.entry(e).or_insert(0);
            *entry = self.add(*entry, c, p);
        }
        out.into_iter().filter(|(_, c)| *c != 0).collect()
    }
}
#[derive(Clone, Debug)]
pub struct UnitSystem {
    p: u64,
    constant: u64,
    // Coordinates: c0.re, t0.re, t0.im, t1.re, t1.im, t2.re, t2.im.
    equations: Vec<Poly>,
}
impl UnitSystem {
    pub fn new(family: &Family) -> Result<(Self, CoefficientWork), &'static str> {
        Self::compile(family.polynomials(), family.modulus())
    }
    fn compile(input: &[Poly], p: u64) -> Result<(Self, CoefficientWork), &'static str> {
        if !(5..=1511).contains(&p)
            || !super::residual_walk::is_prime_u64(p)
            || input.len() != 7
            || input.iter().any(|row| {
                row.iter()
                    .any(|(e, c)| e.len() != 8 || e.iter().sum::<u32>() > 2 || *c == 0 || *c >= p)
            })
        {
            return Err("canonical seven quadrics in eight variables required");
        }
        let mut work = CoefficientWork::default();
        let pivot = work.collect(input[6].clone(), p);
        let mut b = vec![0; 8];
        b[1] = 1;
        let mut rs = vec![0; 8];
        rs[6] = 1;
        rs[7] = 1;
        let constant = pivot
            .iter()
            .find(|(e, _)| e.iter().all(|&x| x == 0))
            .map_or(0, |(_, c)| *c);
        let mut expected = vec![(b, p - 2), (rs, 2)];
        if constant != 0 {
            expected.push((vec![0; 8], constant));
        }
        if pivot != work.collect(expected, p) {
            return Err("exact constant unit pivot required");
        }
        let constant = work.mul(constant, p.div_ceil(2), p);
        let mut rs = vec![0; 7];
        rs[5] = 1;
        rs[6] = 1;
        let mut q = vec![(rs, 1)];
        if constant != 0 {
            q.push((vec![0; 7], constant));
        }
        let mut square = vec![];
        for (e, c) in &q {
            for (f, d) in &q {
                square.push((
                    e.iter().zip(f).map(|(x, y)| x + y).collect(),
                    work.mul(*c, *d, p),
                ));
            }
        }
        let powers = [vec![(vec![0; 7], 1)], q, work.collect(square, p)];
        let mut equations = vec![];
        for row in &input[..6] {
            let mut terms = vec![];
            for (e, c) in row {
                let mut base = e.clone();
                let exponent = base.remove(1);
                for (r, d) in &powers[exponent as usize] {
                    terms.push((
                        base.iter().zip(r).map(|(x, y)| x + y).collect(),
                        work.mul(*c, *d, p),
                    ));
                }
            }
            equations.push(work.collect(terms, p));
        }
        Ok((
            Self {
                p,
                constant,
                equations,
            },
            work,
        ))
    }
    /// Fix c0.re. Remaining order is t0.re,t0.im,t1.re,t1.im,t2.re,t2.im.
    pub fn specialize(&self, value: u64) -> Result<(Vec<Poly>, CoefficientWork), &'static str> {
        if value >= self.p {
            return Err("canonical fiber value required");
        }
        let mut work = CoefficientWork::default();
        let mut powers = [1; 5];
        for i in 1..5 {
            powers[i] = work.mul(powers[i - 1], value, self.p);
        }
        let mut equations = vec![];
        for row in &self.equations {
            let terms = row
                .iter()
                .map(|(e, c)| {
                    let mut n = e.clone();
                    let exponent = n.remove(0);
                    (n, work.mul(*c, powers[exponent as usize], self.p))
                })
                .collect();
            equations.push(work.collect(terms, self.p));
        }
        Ok((equations, work))
    }
    pub fn lift(&self, value: u64, z: &[u64]) -> Result<(Vec<u64>, CoefficientWork), &'static str> {
        if value >= self.p || z.len() != 6 || z.iter().any(|&x| x >= self.p) {
            return Err("canonical six-coordinate root and fiber required");
        }
        let mut work = CoefficientWork::default();
        let product = work.mul(z[4], z[5], self.p);
        let imag = work.add(product, self.constant, self.p);
        let mut out = vec![value, imag];
        out.extend_from_slice(z);
        Ok((out, work))
    }
}

#[cfg(test)]
mod controls {
    use super::*;
    use crate::cryptanalysis::{
        f4_fp,
        jv_cover::{Fld, Fq, E2},
    };
    fn family(im: u64) -> Family {
        let f = Fq::new(11);
        let u = [E2::ONE, E2::ZERO, E2::ONE];
        let v = [E2::ZERO];
        // H=U*W, W=x^5+i*im*x^4+3; nonzero H6 imaginary part.
        let w = [
            f.from_fp(3),
            E2::ZERO,
            E2::ZERO,
            E2::ZERO,
            E2([0, im]),
            E2::ONE,
        ];
        let mut h = vec![E2::ZERO; 8];
        for (i, a) in u.iter().enumerate() {
            for (j, b) in w.iter().enumerate() {
                h[i + j] = f.add(&h[i + j], &f.mul(a, b));
            }
        }
        Family::new(&f, &h, &u, &v).unwrap()
    }
    #[test]
    fn exact_substitution_matches_original_off_variety_with_nonzero_constant() {
        for im in [0, 1, 7, 10] {
            let g = family(im);
            let (u, build) = UnitSystem::new(&g).unwrap();
            assert!(build.products > 0 && build.additions > 0);
            for a in 0..11 {
                let (input, work) = u.specialize(a).unwrap();
                assert!(work.products > 0);
                for seed in 0..32 {
                    let z: Vec<_> = (0..6).map(|j| (seed * 7 + j * 3) % 11).collect();
                    let (point, lift) = u.lift(a, &z).unwrap();
                    assert_eq!(lift.products, 1);
                    let original = g.evaluate(&point).unwrap();
                    assert_eq!(original[6], 0);
                    let reduced: Vec<_> = input.iter().map(|q| f4_fp::eval(q, &z, 11)).collect();
                    assert_eq!(&original[..6], reduced);
                    assert!(input
                        .iter()
                        .flatten()
                        .all(|(e, _)| e.len() == 6 && e.iter().sum::<u32>() <= 4));
                }
            }
        }
    }
    #[test]
    fn malformed_pivots_and_coordinates_are_refused() {
        let g = family(1);
        let (u, _) = UnitSystem::new(&g).unwrap();
        let mut bad = g.polynomials().to_vec();
        bad[6].push((vec![0; 8], 1));
        // Constant changes are allowed; nonlinear pivots are not.
        let mut term = vec![0; 8];
        term[1] = 2;
        bad[6].push((term, 1));
        assert!(UnitSystem::compile(&bad, 11).is_err());
        assert!(UnitSystem::compile(g.polynomials(), 9).is_err());
        assert!(u.specialize(11).is_err());
        assert!(u.lift(0, &[0; 5]).is_err());
        assert!(u.lift(0, &[11; 6]).is_err());
    }
    #[test]
    fn retained_positive_projection_lifts_to_the_original_function() {
        let raw:serde_json::Value=serde_json::from_str(include_str!("../../research/extension_ic_graph_20261115/forced-divisor-h6-20261124/fixture/public.json")).unwrap();
        let decode = |v: &serde_json::Value| {
            serde_json::from_value::<Vec<[u64; 2]>>(v.clone())
                .unwrap()
                .into_iter()
                .map(E2)
                .collect::<Vec<_>>()
        };
        let f = Fq::new(1009);
        let r = &raw["remainders"][0];
        let key: Vec<u64> = serde_json::from_value(r["key"].clone()).unwrap();
        let mut v = vec![E2([key[2], key[3]]), E2([key[4], key[5]])];
        if r["coefficient"] == -1 {
            for x in &mut v {
                *x = f.neg(x);
            }
        }
        let g = Family::new(
            &f,
            &decode(&raw["h"]),
            &[f.from_fp(key[0]), f.from_fp(key[1]), E2::ONE],
            &v,
        )
        .unwrap();
        let point = g
            .recover_parameters(&f, &decode(&raw["a"]), &decode(&raw["c"]))
            .unwrap();
        let (u, _) = UnitSystem::new(&g).unwrap();
        let (z, _) = u.lift(point[0], &point[2..]).unwrap();
        assert_eq!(z, point);
        let (input, _) = u.specialize(point[0]).unwrap();
        assert!(input.iter().all(|q| f4_fp::eval(q, &point[2..], 1009) == 0));
        let a = g.instantiate(&f, &point).unwrap();
        let b = g.instantiate(&f, &z).unwrap();
        assert_eq!(a.a, b.a);
        assert_eq!(a.c, b.c);
        assert_eq!(a.norm, b.norm);
    }
}
