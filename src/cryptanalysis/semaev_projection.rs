//! Exact projection joins for prime-field Semaev decomposition systems.
//!
//! This module replaces a Gröbner-basis decision for the three-summand
//! chain by a join on the roots of its two quadratic `S_3` links.  A root in
//! `F_p` is keyed by its value.  The conjugate roots of an irreducible
//! quadratic are keyed by the coefficients of their monic minimal
//! polynomial, so the join never has to construct `F_{p^2}`.  For four
//! summands, the same pair projection gives the balanced tree
//!
//! ```text
//! S_3(x1,x2,u) = S_3(x3,x4,v) = S_3(u,v,target) = 0.
//! ```
//!
//! The construction is exact, but it is still a table algorithm: it stores
//! at most `min(p, O(B^2))` finite pair roots for a factor base of size `B`.
//! It is therefore an oracle-stage alternative, not by itself an asymptotic
//! ECDLP claim.

use std::collections::HashMap;

use super::pkm_smooth_tower::{addm, invm, mulm, sqrtm, subm, Curve};

/// A root represented without choosing an extension-field model.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize)]
pub enum QuadraticRootKey {
    /// A root in the base field.
    Base(u64),
    /// Either conjugate root of `X^2 + linear*X + constant`.
    Irreducible { linear: u64, constant: u64 },
}

/// The roots of a polynomial of degree at most two.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub enum QuadraticRoots {
    None,
    Finite(Vec<QuadraticRootKey>),
    All,
}

/// Exact, structural work done while building the pair projection.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct BuildStats {
    pub unordered_pairs: u64,
    pub root_keys: u64,
    pub base_field_root_keys: u64,
    pub irreducible_root_keys: u64,
    pub universal_pairs: u64,
    pub distinct_keys: usize,
    pub distinct_base_roots: usize,
}

/// Exact work done by one online query.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct QueryStats {
    pub quadratics: u64,
    pub root_keys: u64,
    pub lookups: u64,
    pub compatible_keys: u64,
    pub cancellation_checks: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct ThreeSumWitness {
    pub x: [u64; 3],
    /// `None` denotes the point at infinity in the cancellation branch.
    pub auxiliary: Option<QuadraticRootKey>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct FourSumWitness {
    pub x: [u64; 4],
    /// `None` in one slot denotes a cancelling pair and the point at infinity.
    pub auxiliaries: [Option<u64>; 2],
}

/// `S_3(x1,x2,X)`, low coefficient first.
pub fn s3_quadratic(curve: &Curve, x1: u64, x2: u64) -> [u64; 3] {
    let p = curve.p;
    let d = subm(x1, x2, p);
    let s = addm(x1, x2, p);
    let product = mulm(x1, x2, p);
    let c2 = mulm(d, d, p);
    let c1 = subm(
        0,
        mulm(
            2,
            addm(
                mulm(s, addm(product, curve.a, p), p),
                mulm(2, curve.b, p),
                p,
            ),
            p,
        ),
        p,
    );
    let shifted = subm(product, curve.a, p);
    let c0 = subm(
        mulm(shifted, shifted, p),
        mulm(4, mulm(curve.b, s, p), p),
        p,
    );
    [c0, c1, c2]
}

pub fn eval_quadratic(q: [u64; 3], x: u64, p: u64) -> u64 {
    addm(q[0], mulm(x, addm(q[1], mulm(q[2], x, p), p), p), p)
}

/// Classify the roots of `c0 + c1 X + c2 X^2` over the algebraic closure.
pub fn quadratic_root_keys(q: [u64; 3], p: u64) -> QuadraticRoots {
    let [c0, c1, c2] = q.map(|x| x % p);
    if c2 == 0 {
        if c1 == 0 {
            return if c0 == 0 {
                QuadraticRoots::All
            } else {
                QuadraticRoots::None
            };
        }
        return QuadraticRoots::Finite(vec![QuadraticRootKey::Base(mulm(
            subm(0, c0, p),
            invm(c1, p),
            p,
        ))]);
    }

    let inv_lead = invm(c2, p);
    let linear = mulm(c1, inv_lead, p);
    let constant = mulm(c0, inv_lead, p);
    let discriminant = subm(mulm(linear, linear, p), mulm(4 % p, constant, p), p);
    if let Some(root_disc) = sqrtm(discriminant, p) {
        let inv_two = invm(2 % p, p);
        let minus_linear = subm(0, linear, p);
        let r0 = mulm(addm(minus_linear, root_disc, p), inv_two, p);
        let r1 = mulm(subm(minus_linear, root_disc, p), inv_two, p);
        let mut roots = vec![QuadraticRootKey::Base(r0)];
        if r1 != r0 {
            roots.push(QuadraticRootKey::Base(r1));
        }
        QuadraticRoots::Finite(roots)
    } else {
        QuadraticRoots::Finite(vec![QuadraticRootKey::Irreducible { linear, constant }])
    }
}

/// A target-independent projection of all unordered factor-base pairs.
#[derive(Clone, Debug)]
pub struct PairProjection {
    curve: Curve,
    base: Vec<u64>,
    witnesses: HashMap<QuadraticRootKey, (u64, u64)>,
    finite_witnesses: HashMap<u64, (u64, u64)>,
    universal_pair: Option<(u64, u64)>,
    stats: BuildStats,
}

impl PairProjection {
    pub fn build(curve: Curve, base: &[u64]) -> Result<Self, String> {
        if curve.p <= 2 {
            return Err("projection join requires an odd prime field".into());
        }
        let mut clean = base.to_vec();
        clean.sort_unstable();
        clean.dedup();
        if let Some(&bad) = clean
            .iter()
            .find(|&&x| sqrtm(curve.rhs(x), curve.p).is_none())
        {
            return Err(format!("factor-base x={bad} is not an F_p point"));
        }

        let mut witnesses = HashMap::new();
        let mut finite_witnesses = HashMap::new();
        let mut universal_pair = None;
        let mut stats = BuildStats::default();
        for (i, &x1) in clean.iter().enumerate() {
            for &x2 in &clean[i..] {
                stats.unordered_pairs += 1;
                match quadratic_root_keys(s3_quadratic(&curve, x1, x2), curve.p) {
                    QuadraticRoots::None => {}
                    QuadraticRoots::All => {
                        stats.universal_pairs += 1;
                        universal_pair.get_or_insert((x1, x2));
                    }
                    QuadraticRoots::Finite(keys) => {
                        for key in keys {
                            stats.root_keys += 1;
                            match key {
                                QuadraticRootKey::Base(root) => {
                                    stats.base_field_root_keys += 1;
                                    finite_witnesses.entry(root).or_insert((x1, x2));
                                }
                                QuadraticRootKey::Irreducible { .. } => {
                                    stats.irreducible_root_keys += 1;
                                }
                            }
                            witnesses.entry(key).or_insert((x1, x2));
                        }
                    }
                }
            }
        }
        stats.distinct_keys = witnesses.len();
        stats.distinct_base_roots = finite_witnesses.len();
        Ok(Self {
            curve,
            base: clean,
            witnesses,
            finite_witnesses,
            universal_pair,
            stats,
        })
    }

    pub fn curve(&self) -> Curve {
        self.curve
    }

    pub fn factor_base(&self) -> &[u64] {
        &self.base
    }

    pub fn build_stats(&self) -> BuildStats {
        self.stats
    }

    pub fn finite_support(&self) -> usize {
        self.finite_witnesses.len()
    }

    /// Exact three-summand query, including the pair-cancellation component
    /// omitted by a finite affine auxiliary variable.
    pub fn decompose_three(&self, target: u64) -> (Option<ThreeSumWitness>, QueryStats) {
        let p = self.curve.p;
        let target = target % p;
        let mut stats = QueryStats::default();

        stats.cancellation_checks += 1;
        if !self.base.is_empty() && self.base.binary_search(&target).is_ok() {
            let x = self.base[0];
            return (
                Some(ThreeSumWitness {
                    x: [x, x, target],
                    auxiliary: None,
                }),
                stats,
            );
        }

        for &x3 in &self.base {
            stats.quadratics += 1;
            match quadratic_root_keys(s3_quadratic(&self.curve, x3, target), p) {
                QuadraticRoots::None => {}
                QuadraticRoots::All => {
                    if let Some((&key, &(x1, x2))) = self.witnesses.iter().next() {
                        stats.compatible_keys += 1;
                        return (
                            Some(ThreeSumWitness {
                                x: [x1, x2, x3],
                                auxiliary: Some(key),
                            }),
                            stats,
                        );
                    }
                    if let Some((x1, x2)) = self.universal_pair {
                        stats.compatible_keys += 1;
                        return (
                            Some(ThreeSumWitness {
                                x: [x1, x2, x3],
                                auxiliary: None,
                            }),
                            stats,
                        );
                    }
                }
                QuadraticRoots::Finite(keys) => {
                    stats.root_keys += keys.len() as u64;
                    for key in keys {
                        stats.lookups += 1;
                        if let Some(&(x1, x2)) = self.witnesses.get(&key) {
                            stats.compatible_keys += 1;
                            return (
                                Some(ThreeSumWitness {
                                    x: [x1, x2, x3],
                                    auxiliary: Some(key),
                                }),
                                stats,
                            );
                        }
                        if let Some((x1, x2)) = self.universal_pair {
                            stats.compatible_keys += 1;
                            return (
                                Some(ThreeSumWitness {
                                    x: [x1, x2, x3],
                                    auxiliary: Some(key),
                                }),
                                stats,
                            );
                        }
                    }
                }
            }
        }
        (None, stats)
    }

    /// Exact balanced four-summand query.  `compatible_keys` counts all
    /// finite pair-root matches examined, so it is also a target function
    /// suitable for the parametric-compression falsifier.
    pub fn decompose_four(&self, target: u64) -> (Option<FourSumWitness>, QueryStats) {
        let p = self.curve.p;
        let target = target % p;
        let mut stats = QueryStats::default();
        let mut witness = None;

        // If either pair cancels to infinity, the other pair must have target
        // as a finite root. Repetition of a factor-base x is allowed.
        stats.cancellation_checks += 1;
        if let (Some(&cancel), Some(&(x3, x4))) =
            (self.base.first(), self.finite_witnesses.get(&target))
        {
            witness = Some(FourSumWitness {
                x: [cancel, cancel, x3, x4],
                auxiliaries: [None, Some(target)],
            });
        }

        for (&u, &(x1, x2)) in &self.finite_witnesses {
            stats.quadratics += 1;
            match quadratic_root_keys(s3_quadratic(&self.curve, u, target), p) {
                QuadraticRoots::None => {}
                QuadraticRoots::All => {
                    if let Some((&v, &(x3, x4))) = self.finite_witnesses.iter().next() {
                        stats.compatible_keys += 1;
                        witness.get_or_insert(FourSumWitness {
                            x: [x1, x2, x3, x4],
                            auxiliaries: [Some(u), Some(v)],
                        });
                    }
                }
                QuadraticRoots::Finite(keys) => {
                    stats.root_keys += keys.len() as u64;
                    for key in keys {
                        let QuadraticRootKey::Base(v) = key else {
                            continue;
                        };
                        stats.lookups += 1;
                        if let Some(&(x3, x4)) = self.finite_witnesses.get(&v) {
                            stats.compatible_keys += 1;
                            witness.get_or_insert(FourSumWitness {
                                x: [x1, x2, x3, x4],
                                auxiliaries: [Some(u), Some(v)],
                            });
                        }
                    }
                }
            }
        }
        (witness, stats)
    }
}

/// Interval factor base, restricted to x-coordinates of `F_p` points.
pub fn interval_factor_base(curve: &Curve, bound: u64) -> Vec<u64> {
    (0..bound.min(curve.p))
        .filter(|&x| sqrtm(curve.rhs(x), curve.p).is_some())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::pkm_smooth_tower::{s3, Pt};

    fn curve() -> Curve {
        Curve { p: 101, a: 2, b: 3 }
    }

    fn canonical_point(curve: &Curve, x: u64) -> Pt {
        Some((x, sqrtm(curve.rhs(x), curve.p).unwrap()))
    }

    fn signed_points(curve: &Curve, x: u64) -> Vec<Pt> {
        let point = canonical_point(curve, x);
        let neg = curve.neg(point);
        if neg == point {
            vec![point]
        } else {
            vec![point, neg]
        }
    }

    fn direct(curve: &Curve, base: &[u64], target: u64, arity: usize) -> bool {
        fn tuples(
            curve: &Curve,
            base: &[u64],
            target: u64,
            arity: usize,
            depth: usize,
            sum: Pt,
        ) -> bool {
            if depth == arity {
                return sum.is_some_and(|(x, _)| x == target);
            }
            base.iter().any(|&x| {
                signed_points(curve, x).into_iter().any(|point| {
                    tuples(curve, base, target, arity, depth + 1, curve.add(sum, point))
                })
            })
        }
        tuples(curve, base, target, arity, 0, None)
    }

    fn witness_reaches(curve: &Curve, xs: &[u64], target: u64) -> bool {
        fn visit(curve: &Curve, xs: &[u64], target: u64, depth: usize, sum: Pt) -> bool {
            if depth == xs.len() {
                return sum.is_some_and(|(x, _)| x == target);
            }
            signed_points(curve, xs[depth])
                .into_iter()
                .any(|point| visit(curve, xs, target, depth + 1, curve.add(sum, point)))
        }
        visit(curve, xs, target, 0, None)
    }

    #[test]
    fn quadratic_coefficients_match_s3() {
        let e = curve();
        for x1 in 0..e.p {
            for x2 in 0..e.p {
                let q = s3_quadratic(&e, x1, x2);
                for x3 in 0..e.p {
                    assert_eq!(eval_quadratic(q, x3, e.p), s3(&e, x1, x2, x3));
                }
            }
        }
    }

    #[test]
    fn root_keys_are_exact() {
        let p = 101;
        for c0 in 0..17 {
            for c1 in 0..17 {
                for c2 in 0..17 {
                    let q = [c0, c1, c2];
                    let actual: Vec<_> = (0..p).filter(|&x| eval_quadratic(q, x, p) == 0).collect();
                    match quadratic_root_keys(q, p) {
                        QuadraticRoots::None => assert!(actual.is_empty()),
                        QuadraticRoots::All => assert_eq!(actual.len(), p as usize),
                        QuadraticRoots::Finite(keys) => {
                            let mut got: Vec<_> = keys
                                .into_iter()
                                .filter_map(|k| match k {
                                    QuadraticRootKey::Base(x) => Some(x),
                                    QuadraticRootKey::Irreducible { .. } => None,
                                })
                                .collect();
                            got.sort_unstable();
                            assert_eq!(got, actual);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn projection_matches_signed_group_control() {
        let e = curve();
        let base = interval_factor_base(&e, 12);
        let projection = PairProjection::build(e, &base).unwrap();
        for target in 0..e.p {
            if sqrtm(e.rhs(target), e.p).is_none() {
                continue;
            }
            let three = projection.decompose_three(target).0;
            assert_eq!(
                three.is_some(),
                direct(&e, &base, target, 3),
                "three-sum target {target}"
            );
            if let Some(witness) = three {
                assert!(witness_reaches(&e, &witness.x, target));
            }
            let four = projection.decompose_four(target).0;
            assert_eq!(
                four.is_some(),
                direct(&e, &base, target, 4),
                "four-sum target {target}"
            );
            if let Some(witness) = four {
                assert!(witness_reaches(&e, &witness.x, target));
            }
        }
    }

    #[test]
    fn rejects_non_point_factor_base_entries() {
        let e = curve();
        let bad = (0..e.p).find(|&x| sqrtm(e.rhs(x), e.p).is_none()).unwrap();
        assert!(PairProjection::build(e, &[bad]).is_err());
    }
}
