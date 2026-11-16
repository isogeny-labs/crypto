//! Adapter from Gaudry's inhomogeneous relations to the existing symbolic
//! large-variable forest. The target column stays symbolic; a final protected
//! column represents the constant one. No point logarithms are supplied.

use pencil_symbolic_graph::{meter, Outcome, SymbolicGraph};

use super::SparseRel;

pub(super) struct GraphEliminator {
    graph: SymbolicGraph,
    small: usize,
    large: usize,
    n: u64,
    pub work: meter::Snapshot,
}

impl GraphEliminator {
    pub fn new(small: usize, large: usize, n: u64) -> Result<Self, String> {
        let before = meter::snapshot();
        let graph = SymbolicGraph::new(n, large, small + 2)
            .map_err(|e| format!("Gaudry symbolic graph: {e:?}"))?;
        Ok(Self {
            graph,
            small,
            large,
            n,
            work: meter::snapshot() - before,
        })
    }

    pub fn feed(&mut self, rel: SparseRel, ops: &mut u64) -> Result<Option<SparseRel>, String> {
        // Validate before changing the graph; target is column `small`, and
        // original large columns are `small+1 .. small+large+1`.
        if rel
            .cols
            .iter()
            .any(|&(column, _)| column > self.small + self.large)
        {
            return Err("Gaudry graph relation column is out of range".into());
        }
        let before = meter::snapshot();
        let mut large = Vec::new();
        let mut small = Vec::new();
        for &(column, coefficient) in &rel.cols {
            meter::collection(1, 0, 1, 1);
            if column <= self.small {
                small.push((column, coefficient));
            } else {
                large.push((column - self.small - 1, coefficient));
            }
        }
        let rhs = rel.rhs % self.n;
        small.push((self.small + 1, if rhs == 0 { 0 } else { self.n - rhs }));
        meter::collection(0, 0, 1, 0);
        let outcome = self.graph.insert(&large, &small);
        let delta = meter::snapshot() - before;
        self.work += delta;
        *ops = ops
            .checked_add(
                u64::try_from(delta.scalar_muls)
                    .map_err(|_| "Gaudry graph multiplication counter overflow")?,
            )
            .ok_or("Gaudry graph multiplication counter overflow")?;
        match outcome.map_err(|e| format!("Gaudry symbolic graph: {e:?}"))? {
            Outcome::Joined | Outcome::Anchored => Ok(None),
            Outcome::Constraint(expression) => {
                let mut cols = Vec::new();
                let mut rhs = 0;
                for (column, coefficient) in expression {
                    if column == self.small + 1 {
                        rhs = if coefficient == 0 {
                            0
                        } else {
                            self.n - coefficient
                        };
                    } else {
                        cols.push((column, coefficient));
                    }
                }
                Ok(Some(SparseRel { cols, rhs }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::gaudry_cubic::LargePrimeEliminator;
    use crate::cryptanalysis::residual_walk::RelationSystem;

    fn insert(system: &mut RelationSystem, row: SparseRel, width: usize, n: u64) {
        let mut dense = vec![0; width];
        for (c, v) in row.cols {
            dense[c] = ((dense[c] as u128 + v as u128) % n as u128) as u64;
        }
        system.insert(dense, row.rhs);
    }

    #[test]
    fn graph_projection_matches_pivots_after_every_prefix() {
        // A deterministic algebra regression, not a timing or curve cohort.
        // Every protected variable is checked against the full input system.
        for n in [3, 7, 101] {
            let (small, large, width) = (3, 9, 13);
            let mut graph = GraphEliminator::new(small, large, n).unwrap();
            let mut pivot = LargePrimeEliminator::new(small + 1, n);
            let mut g = RelationSystem::new(n, small + 1);
            let mut p = RelationSystem::new(n, small + 1);
            let mut full = RelationSystem::new(n, width);
            let mut ops = 0;
            let values: Vec<_> = (0..width).map(|i| (3 * i as u64 + 1) % n).collect();
            let mut state = 0xa1c3_2026_u64;
            for step in 0..240 {
                let mut next = || {
                    state ^= state << 13;
                    state ^= state >> 7;
                    state ^= state << 17;
                    state
                };
                let u = small + 1 + next() as usize % large;
                let v = small + 1 + next() as usize % large;
                let mut cols = vec![
                    (next() as usize % (small + 1), next() % n),
                    (u, 1),
                    (v, if step % 2 == 0 { 1 } else { n - 1 }),
                ];
                cols.sort_unstable();
                let rhs = cols.iter().fold(0u64, |acc, &(c, x)| {
                    ((acc as u128 + x as u128 * values[c] as u128) % n as u128) as u64
                });
                // Baseline requires canonical, distinct column indices.
                let mut canonical = Vec::<(usize, u64)>::new();
                for (c, x) in &cols {
                    if let Some((_, value)) = canonical.last_mut().filter(|(last, _)| last == c) {
                        *value = (*value + x) % n;
                    } else {
                        canonical.push((*c, *x));
                    }
                }
                canonical.retain(|&(_, v)| v != 0);
                let row = SparseRel {
                    cols: canonical,
                    rhs,
                };
                insert(&mut full, row.clone(), width, n);
                if let Some(r) = graph.feed(row.clone(), &mut ops).unwrap() {
                    assert!(r.cols.iter().all(|&(c, _)| c <= small));
                    let lhs = r.cols.iter().fold(0u64, |a, &(c, x)| {
                        ((a as u128 + x as u128 * values[c] as u128) % n as u128) as u64
                    });
                    assert_eq!(lhs, r.rhs);
                    insert(&mut g, r, small + 1, n);
                }
                if let Some(r) = pivot.feed(row, &mut ops) {
                    insert(&mut p, r, small + 1, n);
                }
                assert_eq!(g.rank(), p.rank());
                for c in 0..=small {
                    assert_eq!(g.solved(c), full.solved(c));
                    assert_eq!(p.solved(c), full.solved(c));
                }
            }
        }
    }

    #[test]
    fn duplicate_large_terms_and_nonzero_rhs_preserve_target() {
        let mut graph = GraphEliminator::new(0, 1, 101).unwrap();
        let mut ops = 0;
        assert!(graph
            .feed(
                SparseRel {
                    cols: vec![(1, 1), (1, 1), (0, 1)],
                    rhs: 17
                },
                &mut ops
            )
            .unwrap()
            .is_none());
        let relation = graph
            .feed(
                SparseRel {
                    cols: vec![(1, 2)],
                    rhs: 10,
                },
                &mut ops,
            )
            .unwrap()
            .unwrap();
        let mut system = RelationSystem::new(101, 1);
        insert(&mut system, relation, 1, 101);
        assert_eq!(system.solved(0), Some(7));
        assert!(graph.work.scalar_muls > 0);
    }

    #[test]
    fn bad_columns_are_rejected() {
        let mut graph = GraphEliminator::new(0, 1, 101).unwrap();
        assert!(graph
            .feed(
                SparseRel {
                    cols: vec![(2, 1)],
                    rhs: 0
                },
                &mut 0
            )
            .is_err());
    }

    #[test]
    fn comparison_pivot_boundary_keeps_the_target_symbolic() {
        let rows = [
            SparseRel {
                cols: vec![(1, 1), (2, 1)],
                rhs: 17,
            },
            SparseRel {
                cols: vec![(2, 1)],
                rhs: 10,
            },
        ];
        let mut legacy = LargePrimeEliminator::new(1, 101);
        let mut protected = LargePrimeEliminator::new(2, 101);
        let mut small_system = RelationSystem::new(101, 2);
        for row in rows {
            assert!(legacy.feed(row.clone(), &mut 0).is_none());
            if let Some(full) = protected.feed(row, &mut 0) {
                insert(&mut small_system, full, 2, 101);
            }
        }
        assert_eq!(small_system.solved(1), Some(7));
    }

    #[test]
    fn actual_cubic_oracles_keep_the_same_relation_prefix_and_scalar() {
        use crate::cryptanalysis::gaudry_cubic::{
            generate_instance3, run_gaudry_opts, run_gaudry_with_eliminator, GaudryOptions,
            LargePrimeBackend, Solver, SubspaceBase,
        };
        use rand::{rngs::StdRng, SeedableRng};
        for solver in [
            Solver::MeetInTheMiddle,
            Solver::Groebner,
            Solver::PairOnly,
            Solver::SymmetricPairOnly,
            Solver::SymmetricMeetInTheMiddle,
        ] {
            let inst = generate_instance3(7, 3);
            let base = SubspaceBase::build(&inst, &mut StdRng::seed_from_u64(3));
            let opts = GaudryOptions {
                max_residuals: 20_000,
                solver,
                small_base: 1,
                max_large_primes: 2,
                ..GaudryOptions::default()
            };
            let p = run_gaudry_with_eliminator(&inst, &base, 5, &opts, LargePrimeBackend::Pivot);
            let g = run_gaudry_with_eliminator(
                &inst,
                &base,
                5,
                &opts,
                LargePrimeBackend::SymbolicGraph,
            );
            assert_eq!(p.correct, Some(true), "{p:?}");
            assert_eq!(g.recovered_scalar, p.recovered_scalar);
            assert_eq!(g.residuals, p.residuals);
            assert_eq!(g.relations_verified, p.relations_verified);
            assert_eq!(g.oracle_fp_muls, p.oracle_fp_muls);
            assert_eq!(g.group_ops, p.group_ops);
            assert_eq!(g.lp_histogram, p.lp_histogram);
            assert_eq!(g.lp_input_digest, p.lp_input_digest);
            assert_eq!(g.lp_input_hash_bytes, p.lp_input_hash_bytes);
            assert!(g.lp_input_digest.is_some());
            assert!(g.lp_graph_work.is_some());
            // Legacy metadata stays absent, so old report keys are stable.
            let legacy = run_gaudry_opts(&inst, &base, 5, &opts);
            let json = serde_json::to_value(legacy).unwrap();
            assert!(json.get("lp_backend").is_none());
            assert!(json.get("lp_graph_work").is_none());
            assert!(json.get("lp_input_digest").is_none());
            assert!(json.get("lp_input_hash_bytes").is_none());
        }
    }

    #[test]
    #[should_panic(expected = "at most two")]
    fn graph_refuses_unsupported_budget_before_sampling() {
        use crate::cryptanalysis::gaudry_cubic::*;
        use rand::{rngs::StdRng, SeedableRng};
        let inst = generate_instance3(7, 3);
        let base = SubspaceBase::build(&inst, &mut StdRng::seed_from_u64(3));
        let opts = GaudryOptions {
            max_large_primes: 3,
            ..GaudryOptions::default()
        };
        run_gaudry_with_eliminator(&inst, &base, 5, &opts, LargePrimeBackend::SymbolicGraph);
    }
}
