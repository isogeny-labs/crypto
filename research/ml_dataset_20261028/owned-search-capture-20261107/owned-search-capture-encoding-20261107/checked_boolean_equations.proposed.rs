//! Child of binary_semaev_chain_provenance. Only an existing CheckedMap enters.
//! Auxiliary gate wires are explicit variables: DIMACS wire w -> index w-1.
//! This module does not mint an original owner, reset a deadline, or search proofs.
use super::{Admission, CheckedMap, Error, Quote};
#[path = "sparse_equations.rs"]
mod sparse;
pub use sparse::{Charge, Failure, Limits, Monomial, Polynomial, Stage, Unknown};
/// Borrowing retains exact original identity, field basis, target, sorted codes,
/// twist and pattern. Neither raw Map nor caller-supplied equations enter here.
pub struct Expansion<'a> {
    source: &'a CheckedMap,
    attempt: sparse::Attempt,
    owner_failure: Option<Error>,
}
impl Expansion<'_> {
    pub fn schema(&self) -> &'static str {
        "checked-chain-auxiliary-boolean-equations/v1"
    }
    pub fn source(&self) -> &CheckedMap {
        self.source
    }
    pub fn variables(&self) -> u32 {
        self.source.variables()
    }
    /// Ordered AND, XOR, unit, optional impossible, domain generators; zeros retained.
    pub fn equations(&self) -> &[Polynomial] {
        &self.attempt.equations
    }
    pub fn status(&self) -> Result<(), Failure> {
        self.attempt.result
    }
    pub fn owner_failure(&self) -> Option<Error> {
        self.owner_failure
    }
    pub fn charged(&self) -> Charge {
        self.attempt.charge
    }
    /// Partial rows are diagnostics, never a complete system or admitted generator.
    pub fn retained(&self) -> (&Polynomial, &Polynomial, Stage, usize, usize) {
        (
            &self.attempt.partial,
            &self.attempt.scratch,
            self.attempt.stage,
            self.attempt.row,
            self.attempt.literal,
        )
    }
}
struct Bridge<'a, T: Admission> {
    owner: &'a mut T,
    last: Option<Error>,
}
impl<T: Admission> sparse::Owner for Bridge<'_, T> {
    fn reserve(&mut self, q: Charge) -> Result<(), ()> {
        self.owner
            .reserve(Quote {
                work: q.work,
                bytes: q.bytes,
            })
            .map_err(|e| {
                self.last = Some(e);
            })
    }
    fn check(&mut self) -> Result<(), ()> {
        self.owner.check().map_err(|e| {
            self.last = Some(e);
        })
    }
}
/// Same borrowed original Admission owner supplied to provenance construction.
/// Owner identity remains enforced by the owner integration, not a public bool.
pub fn expand<'a>(
    source: &'a CheckedMap,
    limits: Limits,
    owner: &mut impl Admission,
) -> Expansion<'a> {
    let map = &source.0;
    let mut bridge = Bridge { owner, last: None };
    let attempt = sparse::expand(
        sparse::Source {
            variables: map.variables,
            ands: &map.ands,
            xors: &map.xors,
            units: &map.units,
            impossible: map.impossible,
            domain: &map.domain,
        },
        limits,
        &mut bridge,
    );
    Expansion {
        source,
        attempt,
        owner_failure: bridge.last,
    }
}

#[cfg(test)]
mod integration_tests {
    use super::super::{build_original, Original};
    use super::*;
    use crate::binary_ecc::{F2mElement, IrreduciblePoly};
    use num_bigint::BigUint;
    #[derive(Default)]
    struct Ledger {
        work: u64,
        bytes: u64,
    }
    impl Admission for Ledger {
        fn reserve(&mut self, q: Quote) -> Result<(), Error> {
            self.work = self.work.checked_add(q.work).ok_or(Error::Admission)?;
            self.bytes = self.bytes.checked_add(q.bytes).ok_or(Error::Admission)?;
            if self.work > 64 * 1024 * 1024 || self.bytes > 32 * 1024 * 1024 {
                return Err(Error::Admission);
            }
            Ok(())
        }
        fn check(&mut self) -> Result<(), Error> {
            Ok(())
        }
    }
    #[test]
    fn genuine_checked_maps_m2_through_m15_keep_original_owner_and_order() {
        let irr = IrreduciblePoly {
            degree: 2,
            low_terms: vec![0, 1],
        };
        let a = F2mElement::one(2);
        let b = F2mElement::from_bit_positions(&[1], 2);
        let target = F2mElement::one(2);
        let codes = vec![BigUint::from(0u8), BigUint::from(1u8), BigUint::from(3u8)];
        for m in 2..=15 {
            for twist in [false, true] {
                for pattern in [0, (1u32 << (m - 2)) - 1] {
                    let original = Original {
                        binding: [17; 32],
                        a: &a,
                        b: &b,
                        irr: &irr,
                        twist,
                        m,
                        target: Some(&target),
                        pattern,
                        codes: &codes,
                    };
                    let mut owner = Ledger::default();
                    let checked = build_original(
                        &original,
                        super::super::Limits {
                            variables: 10000,
                            codes: 16,
                            domain_clauses: 10000,
                            work: 32 * 1024 * 1024,
                            bytes: 16 * 1024 * 1024,
                        },
                        &mut owner,
                    )
                    .unwrap();
                    let before = owner.work;
                    let expansion = expand(
                        &checked,
                        Limits {
                            variables: 10000,
                            generators: 10000,
                            terms: 64,
                            degree: 64,
                            work: 32 * 1024 * 1024,
                            bytes: 16 * 1024 * 1024,
                        },
                        &mut owner,
                    );
                    assert_eq!(expansion.status(), Ok(()));
                    assert!(std::ptr::eq(expansion.source(), &checked));
                    assert!(owner.work > before);
                    let map = &checked.0;
                    assert_eq!(
                        expansion.equations().len(),
                        map.ands.len()
                            + map.xors.len()
                            + map.units.len()
                            + usize::from(map.impossible)
                            + map.domain.len()
                    );
                    // Evaluate the exact ordered source constraints against four deterministic
                    // full auxiliary-wire assignments. This is not a SAT witness claim.
                    for salt in 0..4u32 {
                        let bit = |w: u32| {
                            ((w.wrapping_mul(1664525).wrapping_add(salt * 1013904223)) & 0x80000000)
                                != 0
                        };
                        let eval = |p: &Polynomial| {
                            p.iter()
                                .fold(false, |x, t| x ^ t.iter().all(|v| bit(v + 1)))
                        };
                        let mut equations = expansion.equations().iter();
                        for &(g, l, r) in &map.ands {
                            assert_eq!(eval(equations.next().unwrap()), bit(g) ^ (bit(l) & bit(r)));
                        }
                        for (wires, rhs) in &map.xors {
                            assert_eq!(
                                eval(equations.next().unwrap()),
                                wires.iter().fold(*rhs, |x, w| x ^ bit(*w))
                            );
                        }
                        for &(w, rhs) in &map.units {
                            assert_eq!(eval(equations.next().unwrap()), bit(w) ^ rhs);
                        }
                        if map.impossible {
                            assert!(eval(equations.next().unwrap()));
                        }
                        for clause in &map.domain {
                            assert_eq!(
                                eval(equations.next().unwrap()),
                                !clause.iter().any(|l| bit(l.unsigned_abs()) == (*l > 0))
                            );
                        }
                        assert!(equations.next().is_none());
                    }
                }
            }
        }
    }
}

/// Bounded derived rows and independent arithmetic certificates; not dataset activation.
#[path="checked_matrix_rows.rs"]
pub mod matrix_rows;

/// Owning counterpart: moves the checked source together with every paid prefix.
/// Construction still requires CheckedMap; no raw-polynomial constructor exists.
pub struct OwnedExpansion {
    source: CheckedMap,
    attempt: sparse::Attempt,
    owner_failure: Option<Error>,
}
pub fn expand_owned(source: CheckedMap, limits: Limits, owner: &mut impl Admission) -> OwnedExpansion {
    let map = &source.0;
    let mut bridge = Bridge { owner, last: None };
    let attempt = sparse::expand(sparse::Source {
        variables: map.variables, ands: &map.ands, xors: &map.xors,
        units: &map.units, impossible: map.impossible, domain: &map.domain,
    }, limits, &mut bridge);
    OwnedExpansion { source, attempt, owner_failure: bridge.last }
}
impl OwnedExpansion {
    pub fn source(&self) -> &CheckedMap { &self.source }
    pub fn variables(&self) -> u32 { self.source.variables() }
    pub fn equations(&self) -> &[Polynomial] { &self.attempt.equations }
    pub fn status(&self) -> Result<(), Failure> { self.attempt.result }
    pub fn owner_failure(&self) -> Option<Error> { self.owner_failure }
    pub fn charged(&self) -> Charge { self.attempt.charge }
    /// Partial rows are diagnostics, never a complete admitted generator.
    pub fn retained(&self) -> (&Polynomial, &Polynomial, Stage, usize, usize) {
        (&self.attempt.partial, &self.attempt.scratch, self.attempt.stage, self.attempt.row, self.attempt.literal)
    }
}

/// Distinct checked-domain encoding; never substitutes raw-domain generators.
#[path = "checked_affine_domain.rs"]
pub mod affine_domain;
