//! A prime-field adapter for the budgeted recursive individual-log
//! scheduler ([`crate::cryptanalysis::recursive_descent`]).
//!
//! [`PrimePairTableDescent`] works in the counted prime-order subgroup of a
//! single-word prime curve.  Its relations are
//!
//! * a two-summand decomposition `node = P_i + P_j` found by one probe of a
//!   negation-folded [`PairTable`] (through [`decompose_mitm`] with `m = 2`):
//!   `1·log(node) = log P_i + log P_j`, constant 0, both children base
//!   points;
//! * one-large-prime relations `node = P_i + L`, `L = node − P_i` outside the
//!   base, for a bounded number of base points `P_i` per attempt
//!   ([`PrimePairTableDescentOptions::max_large_prime_probes`]): the
//!   scheduler recurses on `L`.
//!
//! Canonicalisation is by negation only (the representative with the
//! smaller group key, multiplier `1` or `r − 1`); no other symmetry is
//! claimed.  Leaf logarithms come from a caller-supplied slice of column
//! logarithms and the base's column map (`log P = coef_of[i] · log(column)`);
//! the adapter does not know, and does not need to know, how they were
//! obtained.  Every relation is re-checked in the counted group before the
//! scheduler consumes it, and every group operation the adapter performs is
//! charged to its own [`GroupOps`] ledger.

use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use serde::Serialize;

use crate::cryptanalysis::ic_boundary::{
    decompose_mitm, CountedGroup, FactorBase, GroupOps, OracleCounters, PairTable, PrimeCurve,
    PrimePoint,
};
use crate::cryptanalysis::recursive_descent::{
    CanonicalNode, DescentRelation, DescentStep, RecursiveDescentOracle,
};

#[inline]
fn add_mod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 + b as u128) % m as u128) as u64
}

#[inline]
fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn inverse_mod(a: u64, m: u64) -> Option<u64> {
    let (mut old_r, mut r) = (a as i128, m as i128);
    let (mut old_s, mut s) = (1i128, 0i128);
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
    }
    (old_r == 1).then(|| old_s.rem_euclid(m as i128) as u64)
}

/// A subgroup point as a scheduler node.  [`PrimePoint`] does not derive
/// `Hash`, so the node wraps it and hashes its fields.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrimeNode(pub PrimePoint);

impl Hash for PrimeNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.x.hash(state);
        self.0.y.hash(state);
        self.0.infinity.hash(state);
    }
}

/// Which collector produced a relation, by base-point index.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum PairDescentWitness {
    /// `node = points[i] + points[j]` from the pair table.
    Pair { i: usize, j: usize },
    /// `node = points[i] + L`, `L` the second child.
    LargePrime { i: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct PrimePairTableDescentOptions {
    /// Base points subtracted from a node per scheduler attempt; attempt
    /// `a` uses base indices `[a·K, (a+1)·K)`.
    pub max_large_prime_probes: usize,
}

impl Default for PrimePairTableDescentOptions {
    fn default() -> Self {
        Self {
            max_large_prime_probes: 64,
        }
    }
}

/// The adapter's counters.  `table_build_ops` is the pair table's build;
/// `ops` is everything the adapter did afterwards (probes, subtractions,
/// relation and logarithm checks).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct PrimePairTableDescentReport {
    pub table_entries: u64,
    pub table_build_ops: GroupOps,
    pub ops: GroupOps,
    pub pair_probes: u64,
    pub pair_hits: u64,
    pub table_lookups: u64,
    pub large_prime_probes: u64,
    pub large_prime_children: u64,
    pub relations_verified: u64,
    pub relations_rejected: u64,
    pub logs_verified: u64,
    pub logs_rejected: u64,
}

/// Recursive descent over a negation-folded pair table on a prime curve.
pub struct PrimePairTableDescent<'a> {
    curve: &'a PrimeCurve,
    generator: PrimePoint,
    r: u64,
    fb: &'a FactorBase<PrimePoint>,
    column_logs: &'a [u64],
    table: PairTable,
    options: PrimePairTableDescentOptions,
    report: RefCell<PrimePairTableDescentReport>,
}

impl<'a> PrimePairTableDescent<'a> {
    /// Build the negation-folded pair table over `fb` (charged to
    /// `table_build_ops`).  `column_logs[c]` is the certified logarithm,
    /// base `generator`, of column `c`'s unknown.
    pub fn new(
        curve: &'a PrimeCurve,
        generator: PrimePoint,
        r: u64,
        fb: &'a FactorBase<PrimePoint>,
        column_logs: &'a [u64],
        options: PrimePairTableDescentOptions,
    ) -> Result<Self, String> {
        if column_logs.len() != fb.columns {
            return Err(format!(
                "{} column logarithms for {} columns",
                column_logs.len(),
                fb.columns
            ));
        }
        let table = PairTable::build_negation_folded(curve, fb);
        let report = PrimePairTableDescentReport {
            table_entries: table.entries,
            table_build_ops: table.build_ops,
            ..Default::default()
        };
        Ok(Self {
            curve,
            generator,
            r,
            fb,
            column_logs,
            table,
            options,
            report: RefCell::new(report),
        })
    }

    pub fn report(&self) -> PrimePairTableDescentReport {
        self.report.borrow().clone()
    }

    fn base_index(&self, p: &PrimePoint) -> Option<usize> {
        let i = self.fb.index_of_key(self.curve.key(p))?;
        (self.fb.points[i] == *p).then_some(i)
    }

    /// `[k]P`, charged.
    fn mul(&self, p: PrimePoint, k: u64) -> PrimePoint {
        let mut report = self.report.borrow_mut();
        self.curve.mul(&mut report.ops, p, k % self.r)
    }

    fn add(&self, p: PrimePoint, q: PrimePoint) -> PrimePoint {
        let mut report = self.report.borrow_mut();
        self.curve.add(&mut report.ops, p, q)
    }

    fn check_relation(
        &self,
        node: &PrimeNode,
        relation: &DescentRelation<PrimeNode, PairDescentWitness>,
    ) -> bool {
        // The witness must name the base points the children claim to be.
        let n = self.fb.points.len();
        match (relation.witness, relation.children.as_slice()) {
            (PairDescentWitness::Pair { i, j }, [(a, _), (b, _)]) => {
                if i >= n || j >= n || a.0 != self.fb.points[i] || b.0 != self.fb.points[j] {
                    return false;
                }
            }
            (PairDescentWitness::LargePrime { i }, [(a, _), _]) => {
                if i >= n || a.0 != self.fb.points[i] {
                    return false;
                }
            }
            _ => return false,
        }
        // [a]node == [c]G + Σ [b]child, in the counted group.
        let r = self.r;
        let lhs = if relation.target_coefficient % r == 1 {
            node.0
        } else {
            self.mul(node.0, relation.target_coefficient)
        };
        let mut rhs = if relation.constant.is_multiple_of(r) {
            self.curve.identity()
        } else {
            self.mul(self.generator, relation.constant)
        };
        for (child, b) in &relation.children {
            if !self.curve.is_on_curve(child.0) {
                return false;
            }
            let term = match b % r {
                0 => continue,
                1 => child.0,
                b => self.mul(child.0, b),
            };
            rhs = self.add(rhs, term);
        }
        lhs == rhs
    }
}

impl RecursiveDescentOracle for PrimePairTableDescent<'_> {
    type Node = PrimeNode;
    type Witness = PairDescentWitness;

    fn modulus(&self) -> u64 {
        self.r
    }

    fn canonicalise(&self, node: &PrimeNode) -> CanonicalNode<PrimeNode> {
        let neg = self.curve.neg(node.0);
        if self.curve.key(&neg) < self.curve.key(&node.0) {
            CanonicalNode {
                node: PrimeNode(neg),
                multiplier: self.r - 1,
            }
        } else {
            CanonicalNode {
                node: *node,
                multiplier: 1,
            }
        }
    }

    fn base_log(&self, node: &PrimeNode) -> Option<u64> {
        let i = self.base_index(&node.0)?;
        let column = *self.column_logs.get(self.fb.col_of[i])?;
        Some(mul_mod(
            self.fb.coef_of[i] % self.r,
            column % self.r,
            self.r,
        ))
    }

    fn relations(
        &mut self,
        node: &PrimeNode,
        attempt: usize,
    ) -> Vec<DescentRelation<PrimeNode, PairDescentWitness>> {
        let mut out = Vec::new();
        let report = self.report.get_mut();
        if attempt == 0 {
            report.pair_probes += 1;
            let mut ctr = OracleCounters::default();
            let hit = decompose_mitm(
                self.curve,
                self.fb,
                &self.table,
                2,
                &mut report.ops,
                &mut ctr,
                node.0,
            );
            report.table_lookups += ctr.lookups;
            if let Some(ij) = hit {
                report.pair_hits += 1;
                out.push(DescentRelation {
                    target_coefficient: 1,
                    constant: 0,
                    children: vec![
                        (PrimeNode(self.fb.points[ij[0]]), 1),
                        (PrimeNode(self.fb.points[ij[1]]), 1),
                    ],
                    witness: PairDescentWitness::Pair { i: ij[0], j: ij[1] },
                });
            }
        }
        let k = self.options.max_large_prime_probes;
        let start = attempt.saturating_mul(k).min(self.fb.points.len());
        let end = start.saturating_add(k).min(self.fb.points.len());
        for i in start..end {
            let report = self.report.get_mut();
            report.large_prime_probes += 1;
            let p = self.fb.points[i];
            let l = self.curve.add(&mut report.ops, node.0, self.curve.neg(p));
            if self.curve.is_identity(&l) || self.base_index(&l).is_some() {
                // A base remainder is a pair relation the probe owns.
                continue;
            }
            self.report.get_mut().large_prime_children += 1;
            out.push(DescentRelation {
                target_coefficient: 1,
                constant: 0,
                children: vec![(PrimeNode(p), 1), (PrimeNode(l), 1)],
                witness: PairDescentWitness::LargePrime { i },
            });
        }
        out
    }

    fn verify_relation(
        &self,
        node: &PrimeNode,
        relation: &DescentRelation<PrimeNode, PairDescentWitness>,
    ) -> bool {
        let ok = self.check_relation(node, relation);
        let mut report = self.report.borrow_mut();
        if ok {
            report.relations_verified += 1;
        } else {
            report.relations_rejected += 1;
        }
        ok
    }

    fn verify_log(&self, node: &PrimeNode, logarithm: u64) -> bool {
        let ok = logarithm < self.r && self.mul(self.generator, logarithm) == node.0;
        let mut report = self.report.borrow_mut();
        if ok {
            report.logs_verified += 1;
        } else {
            report.logs_rejected += 1;
        }
        ok
    }
}

/// Re-verify a descent transcript without re-running the search.
///
/// Each step's relation is re-checked by the oracle, each child logarithm
/// must be either a base leaf (through the canonical multiplier) or the
/// result of an earlier step, each step's result must follow from its
/// relation and be confirmed by `verify_log`, and `logarithm` must be the
/// target's logarithm and pass `verify_log`.  Any mismatch returns `false`.
pub fn replay_transcript<O: RecursiveDescentOracle>(
    oracle: &O,
    target: &O::Node,
    logarithm: u64,
    transcript: &[DescentStep<O::Node, O::Witness>],
) -> bool {
    let r = oracle.modulus();
    if r < 2 {
        return false;
    }
    let known_log = |solved: &HashMap<O::Node, u64>, node: &O::Node| -> Option<u64> {
        let canonical = oracle.canonicalise(node);
        let m = canonical.multiplier % r;
        let log = match oracle.base_log(&canonical.node) {
            Some(l) => l % r,
            None => *solved.get(&canonical.node)?,
        };
        Some(mul_mod(m, log, r))
    };
    let mut solved: HashMap<O::Node, u64> = HashMap::new();
    for step in transcript {
        let relation = &step.relation;
        if relation.children.len() != step.child_logs.len()
            || !oracle.verify_relation(&step.target, relation)
        {
            return false;
        }
        let Some(inv) = inverse_mod(relation.target_coefficient % r, r) else {
            return false;
        };
        let mut rhs = relation.constant % r;
        for ((child, b), &claimed) in relation.children.iter().zip(&step.child_logs) {
            if known_log(&solved, child) != Some(claimed % r) {
                return false;
            }
            rhs = add_mod(rhs, mul_mod(b % r, claimed % r, r), r);
        }
        let result = mul_mod(rhs, inv, r);
        if result != step.result % r || !oracle.verify_log(&step.target, result) {
            return false;
        }
        solved.insert(step.target.clone(), result);
    }
    known_log(&solved, target) == Some(logarithm % r) && oracle.verify_log(target, logarithm % r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_boundary::prime_instance_for;
    use crate::cryptanalysis::ic_framework::plugins::KnownLogBase;
    use crate::cryptanalysis::ic_framework::stages::{FactorBaseBuilder, InstanceCtx, Params};
    use crate::cryptanalysis::recursive_descent::{
        solve_recursive_log, RecursiveDescentFailure, RecursiveDescentOptions,
    };

    const BITS: u32 = 20;
    const BASE_SIZE: u64 = 200;

    struct Setup {
        inst: crate::cryptanalysis::ic_boundary::PrimeInstance,
        fb: FactorBase<PrimePoint>,
        logs: Vec<u64>,
    }

    /// A 20-bit prime-order curve and a `known-log` base with seeded
    /// uniform scalars: the column logarithms are known by construction
    /// (`FactorBase::known_logs`), not computed by any descent.
    fn setup() -> Setup {
        let inst = prime_instance_for(BITS, 5);
        let (fb, logs) = {
            let ctx = InstanceCtx {
                group: &inst.curve,
                generator: inst.generator_point(),
                target: inst.generator_point(),
                r: inst.r,
                cofactor: inst.cofactor,
                group_order: inst.group_order,
                name: inst.name.clone(),
                field_degree: None,
            };
            let mut params = Params::default();
            params.set("size", BASE_SIZE.to_string());
            params.set("scalars", "random");
            let mut ops = GroupOps::default();
            let fb = KnownLogBase.build(&ctx, &params, &mut ops).unwrap();
            let logs = fb.known_logs.clone().unwrap();
            (fb, logs)
        };
        Setup { inst, fb, logs }
    }

    fn point(s: &Setup, k: u64) -> PrimePoint {
        s.inst
            .curve
            .mul(&mut GroupOps::default(), s.inst.generator_point(), k)
    }

    fn adapter(s: &Setup, k: usize) -> PrimePairTableDescent<'_> {
        PrimePairTableDescent::new(
            &s.inst.curve,
            s.inst.generator_point(),
            s.inst.r,
            &s.fb,
            &s.logs,
            PrimePairTableDescentOptions {
                max_large_prime_probes: k,
            },
        )
        .unwrap()
    }

    /// The first planted log, scanning a fixed stream, whose point is not a
    /// pair sum of the base (so the descent must go through a large prime).
    fn planted_needing_large_prime(s: &Setup) -> u64 {
        let probe = adapter(s, 0);
        let mut ctr = OracleCounters::default();
        (0u64..)
            .map(|t| (123_457 + 7_919 * t) % s.inst.r)
            .find(|&k| {
                k != 0
                    && probe.base_index(&point(s, k)).is_none()
                    && decompose_mitm(
                        &s.inst.curve,
                        &s.fb,
                        &probe.table,
                        2,
                        &mut GroupOps::default(),
                        &mut ctr,
                        point(s, k),
                    )
                    .is_none()
            })
            .unwrap()
    }

    fn options(max_depth: usize) -> RecursiveDescentOptions {
        RecursiveDescentOptions {
            max_depth,
            attempts_per_node: 2,
            ..Default::default()
        }
    }

    #[test]
    fn recursive_adapter_descends_through_large_prime_and_replays() {
        let s = setup();
        assert!(s.inst.cofactor == 1, "prime-order curve");
        let planted = planted_needing_large_prime(&s);
        let target = PrimeNode(point(&s, planted));
        let mut oracle = adapter(&s, 32);
        let solution = solve_recursive_log(&mut oracle, target, &options(2)).unwrap();
        assert_eq!(solution.logarithm, planted);
        assert!(solution.report.final_verified);
        assert!(oracle.verify_log(&target, solution.logarithm));
        let large_prime_steps = solution
            .transcript
            .iter()
            .filter(|step| matches!(step.relation.witness, PairDescentWitness::LargePrime { .. }))
            .count();
        assert!(large_prime_steps >= 1, "went through a large-prime child");
        assert!(solution.transcript.len() >= 2, "child step and parent step");
        let report = oracle.report();
        eprintln!(
            "recursive_adapter e2e: r={} planted={} steps={} scheduler={:?} adapter={:?}",
            s.inst.r,
            planted,
            solution.transcript.len(),
            solution.report,
            report
        );
        assert!(report.large_prime_children >= 1);
        assert!(report.relations_verified >= 2);
        assert!(report.ops.adds > 0);

        // The transcript replays without the search.
        assert!(replay_transcript(
            &oracle,
            &target,
            solution.logarithm,
            &solution.transcript
        ));
        // A wrong final logarithm is refused.
        assert!(!replay_transcript(
            &oracle,
            &target,
            (solution.logarithm + 1) % s.inst.r,
            &solution.transcript
        ));
        let last = solution.transcript.len() - 1;
        // Tampered coefficient.
        let mut bad = solution.transcript.clone();
        bad[last].relation.children[1].1 = 2;
        assert!(!replay_transcript(
            &oracle,
            &target,
            solution.logarithm,
            &bad
        ));
        // Tampered child: another point in place of the large prime.
        let mut bad = solution.transcript.clone();
        let other = s.fb.points[3];
        bad[last].relation.children[1].0 = PrimeNode(other);
        assert!(!replay_transcript(
            &oracle,
            &target,
            solution.logarithm,
            &bad
        ));
        // Tampered child logarithm.
        let mut bad = solution.transcript.clone();
        bad[last].child_logs[1] = (bad[last].child_logs[1] + 1) % s.inst.r;
        assert!(!replay_transcript(
            &oracle,
            &target,
            solution.logarithm,
            &bad
        ));
        // A dropped child step leaves the parent's large prime unjustified.
        let mut bad = solution.transcript.clone();
        bad.remove(0);
        assert!(!replay_transcript(
            &oracle,
            &target,
            solution.logarithm,
            &bad
        ));
    }

    /// The scheduler checks the depth before the base lookup, so a
    /// relation's children sit at depth 1: `max_depth = 0` refuses every
    /// relation (only a base-point target would solve), `max_depth = 1`
    /// admits a direct pair but refuses to decompose a large prime (its
    /// children would sit at depth 2).
    #[test]
    fn recursive_adapter_depth_limits_large_prime_recursion() {
        let s = setup();
        let planted = planted_needing_large_prime(&s);
        let target = PrimeNode(point(&s, planted));
        for depth in [0, 1] {
            let mut oracle = adapter(&s, 32);
            let err = solve_recursive_log(&mut oracle, target, &options(depth)).unwrap_err();
            assert_eq!(
                err,
                RecursiveDescentFailure::DepthBudget,
                "max_depth {depth}"
            );
            assert!(oracle.report().large_prime_children >= 1);
        }

        // A direct pair sum of two base points.
        let mut oracle = adapter(&s, 32);
        let (i, j) = (2usize, 9usize);
        let direct = s
            .inst
            .curve
            .add(&mut GroupOps::default(), s.fb.points[i], s.fb.points[j]);
        let expect = add_mod(
            oracle.base_log(&PrimeNode(s.fb.points[i])).unwrap(),
            oracle.base_log(&PrimeNode(s.fb.points[j])).unwrap(),
            s.inst.r,
        );
        assert!(
            oracle.base_index(&direct).is_none(),
            "not itself a base point"
        );
        let target = PrimeNode(direct);
        assert_eq!(
            solve_recursive_log(&mut oracle, target, &options(0)).unwrap_err(),
            RecursiveDescentFailure::DepthBudget,
            "max_depth 0 refuses even base-point children"
        );
        let solution = solve_recursive_log(&mut oracle, target, &options(1)).unwrap();
        assert_eq!(solution.logarithm, expect);
        assert_eq!(solution.transcript.len(), 1);
        assert!(matches!(
            solution.transcript[0].relation.witness,
            PairDescentWitness::Pair { .. }
        ));
        assert!(replay_transcript(
            &oracle,
            &target,
            solution.logarithm,
            &solution.transcript
        ));
    }

    #[test]
    fn recursive_adapter_budget_exhaustion_fails_closed() {
        let s = setup();
        let planted = planted_needing_large_prime(&s);
        let target = PrimeNode(point(&s, planted));
        let budgets = [
            RecursiveDescentOptions {
                max_nodes: 1,
                ..options(2)
            },
            RecursiveDescentOptions {
                max_relations: 1,
                ..options(2)
            },
            RecursiveDescentOptions {
                attempts_per_node: 1,
                max_nodes: 3,
                ..options(2)
            },
        ];
        let mut failures = 0;
        for budget in budgets {
            let mut oracle = adapter(&s, 32);
            match solve_recursive_log(&mut oracle, target, &budget) {
                Ok(solution) => assert_eq!(solution.logarithm, planted, "never a wrong log"),
                Err(e) => {
                    failures += 1;
                    assert!(matches!(
                        e,
                        RecursiveDescentFailure::NodeBudget
                            | RecursiveDescentFailure::RelationBudget
                            | RecursiveDescentFailure::DepthBudget
                            | RecursiveDescentFailure::NoDecomposition
                    ));
                }
            }
        }
        // The first two budgets cannot fit a large-prime descent.
        assert!(failures >= 2);
        let mut oracle = adapter(&s, 32);
        assert_eq!(
            solve_recursive_log(
                &mut oracle,
                target,
                &RecursiveDescentOptions {
                    max_nodes: 1,
                    ..options(2)
                }
            )
            .unwrap_err(),
            RecursiveDescentFailure::NodeBudget
        );
    }
}
