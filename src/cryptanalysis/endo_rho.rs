//! # Pollard rho on the orbits of an efficient endomorphism
//!
//! A curve that carries an endomorphism `φ` acting on the order-`r`
//! subgroup as `[λ]` lets rho walk on the classes `{±φ^i(P)}` instead
//! of on points.  When `λ` has multiplicative order `m` modulo `r`, a
//! class has `m` points, or `2m` when `−1` is not a power of `λ`, and
//! the expected walk shrinks from `√(πr/4)` (negation) to
//! `√(πr/2A)` with `A` the class size.  Two odd-characteristic families
//! carry such a map:
//!
//! - **Subfield curves.**  `E / F_p` seen on `E(F_{p^k})`, `k` prime:
//!   the `p`-power Frobenius acts on the subgroup outside `E(F_p)` as a
//!   primitive `k`-th root of unity, so `A = 2k`
//!   ([`crate::cryptanalysis::subfield_fp3`], `k = 3`, `A = 6`).
//! - **GLS curves.**  The quadratic twist over `F_{p²}` of a curve over
//!   `F_p` carries `ψ` with `ψ² = −1`, so `A = 4`
//!   ([`crate::cryptanalysis::gls_fp2`]).
//!
//! [`EndomorphismClasses`] is the [`RhoClasses`] for any such map: the
//! representative of a class is its member with the smallest group key,
//! and the multiplier is the matching power of `λ` (negated when the
//! member is a negation).  It runs under [`rho_walk_with`] with the
//! negation walk's look-ahead and cycle escape, exactly as the cubic and
//! quartic automorphism classes do, so every rho arm on one curve is
//! the same walk in the same counted unit.
//!
//! **What a canonicalisation costs.**  `m − 1` applications of `φ` per
//! step (two Frobenius maps on `F_{p³}` are 8 `F_p` multiplications; one
//! `ψ` on `F_{p²}` is 8), plus `A − 1` key comparisons.  None of it is a
//! group operation, so the walk reports it as
//! `canonicalisations_uncharged` (one per step, as for negation), and
//! [`EndomorphismClasses::applications`] counts the maps applied so a
//! comparison can price them (AGENTS.md §5a).
//!
//! [`rho_walk_with`]: crate::cryptanalysis::ic_boundary::rho_walk_with

use std::cell::Cell;
use std::marker::PhantomData;

use crate::cryptanalysis::glv_invariant_base::Endomorphism;
use crate::cryptanalysis::ic_boundary::{mulmod, CountedGroup, GroupOps, RhoClasses};

/// The classes `{±φ^i(P)}` of an endomorphism with eigenvalue `λ`.
pub struct EndomorphismClasses<G: CountedGroup, E: Endomorphism<G>> {
    endo: E,
    r: u64,
    /// `λ^i mod r`, `0 ≤ i < m`.
    powers: Vec<u64>,
    /// `−1` is not a power of `λ`, so negations are added to the class.
    negate: bool,
    method: &'static str,
    applications: Cell<u64>,
    _group: PhantomData<G>,
}

impl<G: CountedGroup, E: Endomorphism<G>> EndomorphismClasses<G, E> {
    /// The classes of `endo` on `⟨generator⟩`, or why not.  The
    /// eigenvalue is checked as `φ(G) = [λ]G` with the scalar
    /// multiplication counted on `ops` (it is set-up, like the cubic
    /// classes' check), and `λ` must have order between 2 and 64.
    pub fn new(
        g: &G,
        endo: E,
        generator: G::Elt,
        r: u64,
        ops: &mut GroupOps,
        method: &'static str,
    ) -> Result<Self, String> {
        let lambda = endo.eigenvalue() % r;
        if lambda <= 1 {
            return Err(format!("`{}` acts trivially on the subgroup", endo.name()));
        }
        if endo.apply(g, generator) != g.mul(ops, generator, lambda) {
            return Err(format!(
                "`{}` does not act as [{lambda}] on the generator",
                endo.name()
            ));
        }
        let mut powers = vec![1u64];
        let mut x = lambda;
        while x != 1 {
            if powers.len() >= 64 {
                return Err(format!(
                    "the eigenvalue of `{}` has order above 64 mod r",
                    endo.name()
                ));
            }
            powers.push(x);
            x = mulmod(x, lambda, r);
        }
        let negate = !powers.contains(&(r - 1));
        Ok(Self {
            endo,
            r,
            powers,
            negate,
            method,
            applications: Cell::new(0),
            _group: PhantomData,
        })
    }

    /// The multiplicative order `m` of `λ` modulo `r`.
    pub fn order(&self) -> usize {
        self.powers.len()
    }

    /// Endomorphism applications performed by canonicalisations so far.
    pub fn applications(&self) -> u64 {
        self.applications.get()
    }
}

impl<G: CountedGroup, E: Endomorphism<G>> RhoClasses<G> for EndomorphismClasses<G, E> {
    fn automorphisms(&self) -> u32 {
        (self.powers.len() * if self.negate { 2 } else { 1 }) as u32
    }

    fn canon(&self, g: &G, _ops: &mut GroupOps, p: G::Elt) -> (G::Elt, u64) {
        if g.is_identity(&p) {
            return (p, 1);
        }
        let mut best = p;
        let mut best_key = g.key(&p);
        let mut best_mu = 1u64;
        let mut consider = |pt: G::Elt, mu: u64| {
            let k = g.key(&pt);
            if k < best_key {
                best = pt;
                best_key = k;
                best_mu = mu;
            }
        };
        let mut cur = p;
        for (i, &mu) in self.powers.iter().enumerate() {
            if i > 0 {
                cur = self.endo.apply(g, cur);
                consider(cur, mu);
            }
            if self.negate {
                consider(g.neg(cur), (self.r - mu) % self.r);
            }
        }
        self.applications
            .set(self.applications.get() + self.powers.len() as u64 - 1);
        (best, best_mu)
    }

    fn method(&self) -> &'static str {
        self.method
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::gls_fp2::generate_gls_instance;
    use crate::cryptanalysis::ic_boundary::{rho_walk_with, NegationClasses, RhoWalk};
    use crate::cryptanalysis::subfield_fp3::generate_subfield_instance;

    #[test]
    fn subfield_frobenius_classes_have_six_members_and_a_correct_multiplier() {
        let inst = generate_subfield_instance(10, 3, false, 64).unwrap();
        let mut ops = GroupOps::default();
        let c = EndomorphismClasses::new(
            &inst.curve,
            inst.frobenius,
            inst.generator,
            inst.r,
            &mut ops,
            "test",
        )
        .unwrap();
        assert_eq!(c.order(), 3);
        assert_eq!(RhoClasses::automorphisms(&c), 6);
        for k in [1u64, 7, 12345, inst.r - 2] {
            let p = inst.curve.mul(&mut ops, inst.generator, k % inst.r);
            let (rep, mu) = c.canon(&inst.curve, &mut ops, p);
            assert_eq!(rep, inst.curve.mul(&mut ops, p, mu));
            // Every member of the class has the same representative.
            let q = inst.curve.neg(c.endo.apply(&inst.curve, p));
            assert_eq!(c.canon(&inst.curve, &mut ops, q).0, rep);
        }
    }

    #[test]
    fn gls_classes_have_four_members_without_explicit_negation() {
        let inst = generate_gls_instance(10, 5, 64).unwrap();
        let mut ops = GroupOps::default();
        let c = EndomorphismClasses::new(
            &inst.curve,
            inst.psi,
            inst.generator,
            inst.r,
            &mut ops,
            "test",
        )
        .unwrap();
        assert_eq!(c.order(), 4);
        assert_eq!(RhoClasses::automorphisms(&c), 4);
        let p = inst.curve.mul(&mut ops, inst.generator, 99);
        let (rep, mu) = c.canon(&inst.curve, &mut ops, p);
        assert_eq!(rep, inst.curve.mul(&mut ops, p, mu));
    }

    #[test]
    fn endomorphism_rho_solves_and_beats_negation_rho_on_average() {
        // Positive control: on the same subfield curves and targets, the
        // Frobenius-orbit walk takes fewer steps than the negation walk.
        let (mut neg_steps, mut frob_steps) = (0u64, 0u64);
        for seed in 0..12u64 {
            let inst = generate_subfield_instance(9, seed, false, 64).unwrap();
            let mut ops = GroupOps::default();
            let k = (seed * 7919 + 13) % inst.r;
            let q = inst.curve.mul(&mut ops, inst.generator, k);
            let frob = EndomorphismClasses::new(
                &inst.curve,
                inst.frobenius,
                inst.generator,
                inst.r,
                &mut ops,
                "frobenius",
            )
            .unwrap();
            let cap = 64 * (inst.r as f64).sqrt() as u64;
            let a = rho_walk_with(
                &inst.curve,
                &NegationClasses { r: inst.r },
                inst.generator,
                q,
                inst.r,
                seed,
                cap,
                RhoWalk::negation(),
            );
            let b = rho_walk_with(
                &inst.curve,
                &frob,
                inst.generator,
                q,
                inst.r,
                seed,
                cap,
                RhoWalk::negation(),
            );
            assert_eq!(a.recovered, Some(k));
            assert_eq!(b.recovered, Some(k));
            assert_eq!(b.automorphisms, 6);
            neg_steps += a.steps;
            frob_steps += b.steps;
        }
        assert!(
            frob_steps < neg_steps,
            "frobenius {frob_steps} vs negation {neg_steps}"
        );
    }
}
