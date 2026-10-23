//! Independent fixture oracle: solve quadratic links by GF(2) linear algebra.
//! It counts Boolean chain roots, including repeated x coordinates. It is
//! solely a correctness oracle; enumeration costs are not an IC claim.
use super::{span, Gf2};
use std::collections::BTreeMap;

pub struct Quadratic<'a> {
    gf: &'a Gf2,
    pivots: Vec<(u64, u64)>,
}

impl<'a> Quadratic<'a> {
    pub fn new(gf: &'a Gf2) -> Self {
        let mut pivots: Vec<(u64, u64)> = Vec::new();
        for j in 1..gf.n {
            let mut image = gf.sqr(1 << j) ^ (1 << j);
            let mut preimage = 1 << j;
            for &(v, c) in &pivots {
                if image >> (63 - v.leading_zeros()) & 1 != 0 {
                    image ^= v;
                    preimage ^= c;
                }
            }
            assert_ne!(image, 0, "kernel of y^2+y is exactly {{0,1}}");
            pivots.push((image, preimage));
            pivots.sort_unstable_by_key(|&(v, _)| std::cmp::Reverse(63 - v.leading_zeros()));
        }
        Self { gf, pivots }
    }

    fn artin(&self, mut v: u64) -> Option<u64> {
        let mut result = 0;
        for &(p, c) in &self.pivots {
            if v >> (63 - p.leading_zeros()) & 1 != 0 {
                v ^= p;
                result ^= c;
            }
        }
        (v == 0).then_some(result)
    }

    /// None denotes the identically zero polynomial, whose root set is the
    /// whole field. Such a link cannot occur for S3 with b=1.
    pub fn roots(&self, a: u64, b: u64, c: u64) -> Option<Vec<u64>> {
        let gf = self.gf;
        if a == 0 {
            return if b != 0 {
                Some(vec![gf.mul(c, gf.inv(b))])
            } else if c != 0 {
                Some(vec![])
            } else {
                None
            };
        }
        if b == 0 {
            let mut root = gf.mul(c, gf.inv(a));
            for _ in 1..gf.n {
                root = gf.sqr(root);
            }
            return Some(vec![root]);
        }
        let inv_b = gf.inv(b);
        let rhs = gf.mul(gf.mul(a, c), gf.sqr(inv_b));
        let Some(y) = self.artin(rhs) else {
            return Some(vec![]);
        };
        let scale = gf.mul(b, gf.inv(a));
        let mut roots = vec![gf.mul(scale, y), gf.mul(scale, y ^ 1)];
        roots.sort_unstable();
        Some(roots)
    }

    fn link(&self, x: u64, y: u64) -> Vec<u64> {
        let xy = self.gf.mul(x, y);
        self.roots(self.gf.sqr(x ^ y), xy, self.gf.sqr(xy) ^ 1)
            .expect("nonzero S3 polynomial")
    }
}

/// Meet in the middle on the chain, not on group points. For m=3 and m=4
/// this uses O(|V|^2) quadratic solves, and exactly counts intermediate roots.
pub fn count(gf: &Gf2, bs: &[u64], xr: u64, m: usize) -> u64 {
    assert!(m == 3 || m == 4);
    let domain = span(bs);
    let quad = Quadratic::new(gf);
    let mut right = BTreeMap::<u64, u64>::new();
    for &last in &domain {
        for u in quad.link(last, xr) {
            if m == 3 {
                *right.entry(u).or_default() += 1;
            } else {
                for &x in &domain {
                    for v in quad.link(x, u) {
                        *right.entry(v).or_default() += 1;
                    }
                }
            }
        }
    }
    let mut total = 0u64;
    for &x in &domain {
        for &y in &domain {
            for u in quad.link(x, y) {
                total = total
                    .checked_add(*right.get(&u).unwrap_or(&0))
                    .expect("count overflow");
            }
        }
    }
    total
}

/// One exact chain witness, returned as (summand x values, intermediates).
pub fn witness(gf: &Gf2, bs: &[u64], xr: u64, m: usize) -> Option<(Vec<u64>, Vec<u64>)> {
    assert!(m == 3 || m == 4);
    let domain = span(bs);
    let quad = Quadratic::new(gf);
    let mut right = BTreeMap::new();
    for &last in &domain {
        for u in quad.link(last, xr) {
            if m == 3 {
                right.entry(u).or_insert((vec![last], vec![u]));
            } else {
                for &x in &domain {
                    for v in quad.link(x, u) {
                        right.entry(v).or_insert((vec![x, last], vec![v, u]));
                    }
                }
            }
        }
    }
    for &x in &domain {
        for &y in &domain {
            for u in quad.link(x, y) {
                if let Some((suffix, intermediates)) = right.get(&u) {
                    let mut xs = vec![x, y];
                    xs.extend(suffix);
                    return Some((xs, intermediates.clone()));
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{element, find_irreducible_sparse};
    #[test]
    fn quadratic_roots_equal_full_field_enumeration() {
        for n in [5, 6, 7] {
            let gf = Gf2::new(&find_irreducible_sparse(n).unwrap());
            let q = Quadratic::new(&gf);
            for a in [0, 1, 2, 3, 7] {
                for b in [0, 1, 2, 3, 7] {
                    for c in 0..1 << n {
                        let expected: Vec<_> = (0..1 << n)
                            .filter(|&x| gf.mul(a, gf.sqr(x)) ^ gf.mul(b, x) ^ c == 0)
                            .collect();
                        match q.roots(a, b, c) {
                            Some(roots) => assert_eq!(roots, expected),
                            None => assert_eq!(expected.len(), 1 << n),
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn chain_count_equals_legacy_exhaustive_oracle() {
        use crypto_lib::cryptanalysis::koblitz_bench::chained_solution_count;
        let irr = find_irreducible_sparse(5).unwrap();
        let gf = Gf2::new(&irr);
        let bs = [1, 2];
        let base: Vec<_> = bs.iter().map(|&x| element(x, 5)).collect();
        for m in [3, 4] {
            for xr in 0..32 {
                assert_eq!(
                    count(&gf, &bs, xr, m),
                    chained_solution_count(m, &base, &element(xr, 5), &element(1, 5), &irr)
                );
            }
        }
    }
}
