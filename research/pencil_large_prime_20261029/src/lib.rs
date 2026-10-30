#![recursion_limit = "256"]
//! Exact elimination of at most two large variables per homogeneous row.
//!
//! Small columns are arbitrary symbolic variables, not known scalar values.
//! This module has no curve, target-scalar, or discrete-log interface.
//! Input modulus must be prime. All input coefficients are canonicalized,
//! including duplicate columns, before changing graph state.

pub type Sparse = sparse::TrackedMap<u64>;

pub mod allocation;
pub mod collector;
pub mod meter;
pub mod outer_primality;
pub mod residual;
pub mod scalar_ledger;
pub mod sparse;
pub mod tracked_set;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    NonPrimeModulus,
    ColumnOutOfRange,
    MoreThanTwoLargeVariables,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Joined,
    Anchored,
    /// The returned symbolic expression must equal zero. An empty map is
    /// a dependent row, not an additional independent constraint.
    Constraint(Sparse),
}

fn add(a: u64, b: u64, p: u64) -> u64 {
    meter::charge(|c| c.scalar_adds += 1);
    ((a as u128 + b as u128) % p as u128) as u64
}
fn mul(a: u64, b: u64, p: u64) -> u64 {
    meter::charge(|c| c.scalar_muls += 1);
    ((a as u128 * b as u128) % p as u128) as u64
}
fn neg(a: u64, p: u64) -> u64 {
    meter::charge(|c| c.scalar_negations += 1);
    if a == 0 {
        0
    } else {
        p - a
    }
}
fn pow(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut v = 1;
    while n != 0 {
        if n & 1 != 0 {
            v = mul(v, a, p);
        }
        a = mul(a, a, p);
        n >>= 1;
    }
    v
}

fn reduce(a: u64, p: u64) -> u64 {
    meter::charge(|c| c.modular_reductions += 1);
    a % p
}
fn inverse(a: u64, p: u64) -> u64 {
    meter::charge(|c| c.scalar_inversions += 1);
    pow(a, p - 2, p)
}

// Deterministic Miller--Rabin base set for the complete u64 range.
fn is_prime(p: u64) -> bool {
    meter::charge(|c| c.primality_checks += 1);
    if p < 2 {
        return false;
    }
    for d in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if reduce(p, d) == 0 {
            return p == d;
        }
    }
    let s = (p - 1).trailing_zeros();
    let d = (p - 1) >> s;
    for a in [2, 325, 9375, 28178, 450775, 9780504, 1795265022] {
        if reduce(a, p) == 0 {
            continue;
        }
        let mut x = pow(reduce(a, p), d, p);
        if x == 1 || x == p - 1 {
            continue;
        }
        let mut witness = true;
        for _ in 1..s {
            x = mul(x, x, p);
            if x == p - 1 {
                witness = false;
                break;
            }
        }
        if witness {
            return false;
        }
    }
    true
}

fn accumulate(out: &mut Sparse, column: usize, value: u64, p: u64) {
    meter::charge(|c| c.sparse_accumulations += 1);
    let v = add(
        out.get(&column)
            .map(|&v| {
                meter::collection(0, 1, 0, 0);
                v
            })
            .unwrap_or(0),
        value,
        p,
    );
    if v == 0 {
        out.remove(&column);
    } else {
        out.insert(column, v);
    }
}
fn axpy(out: &mut Sparse, a: u64, input: &Sparse, p: u64) {
    for (&column, &v) in input {
        meter::charge(|c| c.payload_terms += 1);
        accumulate(out, column, mul(a, v, p), p);
    }
}
fn scaled(input: &Sparse, a: u64, p: u64) -> Sparse {
    let mut out = Sparse::new();
    axpy(&mut out, a, input, p);
    out
}

/// A forest of substitutions X_i = scale_i X_parent + offset_i(S).
/// Root anchors, when present, express a root entirely in small variables.
/// Union by component size bounds forest depth; there is no path compression
/// and no assumption that a graph cycle is a full symbolic relation.
#[derive(Clone)]
pub struct SymbolicGraph {
    p: u64,
    small_count: usize,
    parent: Vec<usize>,
    size: Vec<usize>,
    scale: Vec<u64>,
    offset: Vec<Sparse>,
    anchor: Vec<Option<Sparse>>,
}

impl SymbolicGraph {
    pub fn new(p: u64, large_count: usize, small_count: usize) -> Result<Self, Error> {
        if !is_prime(p) {
            return Err(Error::NonPrimeModulus);
        }
        meter::collection(
            0,
            0,
            large_count.checked_mul(5).expect("vector size overflow"),
            0,
        );
        Ok(Self {
            p,
            small_count,
            parent: (0..large_count).collect(),
            size: vec![1; large_count],
            scale: vec![1; large_count],
            offset: vec![Sparse::new(); large_count],
            anchor: vec![None; large_count],
        })
    }

    fn expression(&self, mut node: usize) -> (usize, u64, Sparse) {
        let mut a = 1;
        let mut b = Sparse::new();
        while {
            meter::collection(1, 0, 0, 1);
            self.parent[node] != node
        } {
            meter::collection(3, 0, 0, 0); // offset, scale, next parent reads
            meter::charge(|c| c.parent_hops += 1);
            axpy(&mut b, a, &self.offset[node], self.p);
            a = mul(a, self.scale[node], self.p);
            node = self.parent[node];
        }
        meter::collection(1, 0, 0, 0);
        if let Some(value) = &self.anchor[node] {
            axpy(&mut b, a, value, self.p);
            a = 0;
        }
        (node, a, b)
    }

    /// Insert sum(large coefficients * X) + sum(small coefficients * S) = 0.
    /// Errors leave the graph unchanged. Three surviving large variables are
    /// rejected explicitly; a future collector must record that admission cost.
    /// The caller must retain every nonempty returned constraint: they are not
    /// stored inside this forest, whose substitutions alone are insufficient.
    pub fn insert(
        &mut self,
        large: &[(usize, u64)],
        small: &[(usize, u64)],
    ) -> Result<Outcome, Error> {
        if large.iter().any(|&(i, _)| {
            meter::collection(1, 0, 0, 1);
            i >= self.parent.len()
        }) || small.iter().any(|&(i, _)| {
            meter::collection(1, 0, 0, 1);
            i >= self.small_count
        }) {
            return Err(Error::ColumnOutOfRange);
        }
        let mut canonical = Sparse::new();
        for &(i, v) in large {
            meter::collection(1, 0, 0, 0);
            accumulate(&mut canonical, i, reduce(v, self.p), self.p);
        }
        meter::collection(0, 0, 0, 1);
        if canonical.len() > 2 {
            return Err(Error::MoreThanTwoLargeVariables);
        }
        let mut payload = Sparse::new();
        for &(i, v) in small {
            meter::collection(1, 0, 0, 0);
            accumulate(&mut payload, i, reduce(v, self.p), self.p);
        }
        let mut roots = Sparse::new();
        for (i, v) in canonical {
            let (root, a, b) = self.expression(i);
            accumulate(&mut roots, root, mul(v, a, self.p), self.p);
            axpy(&mut payload, v, &b, self.p);
        }
        match roots.len() {
            0 => Ok(Outcome::Constraint(payload)),
            1 => {
                let (&root, &a) = roots.first_key_value().unwrap();
                meter::collection(0, 1, 0, 0);
                let factor = neg(inverse(a, self.p), self.p);
                self.anchor[root] = Some(scaled(&payload, factor, self.p));
                Ok(Outcome::Anchored)
            }
            2 => {
                let mut iter = roots.into_iter();
                let (mut u, mut a) = iter.next().unwrap();
                let (mut v, mut b) = iter.next().unwrap();
                meter::collection(2, 0, 0, 1 + usize::from(self.size[u] == self.size[v]));
                if (self.size[u], u) > (self.size[v], v) {
                    std::mem::swap(&mut u, &mut v);
                    std::mem::swap(&mut a, &mut b);
                }
                let factor = neg(inverse(a, self.p), self.p);
                self.parent[u] = v;
                self.scale[u] = mul(factor, b, self.p);
                self.offset[u] = scaled(&payload, factor, self.p);
                meter::collection(2, 0, 0, 0);
                self.size[v] += self.size[u];
                Ok(Outcome::Joined)
            }
            _ => unreachable!("two input variables give at most two roots"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn anchoring_copies_one_borrowed_lookup_entry() {
        let mut graph = SymbolicGraph::new(7, 1, 0).unwrap();
        meter::reset();
        assert_eq!(graph.insert(&[(0, 1)], &[]), Ok(Outcome::Anchored));
        assert_eq!(meter::snapshot().collection_elements_copied, 1);
    }
    #[test]
    fn accumulation_copies_only_existing_coefficients() {
        let mut out = Sparse::new();
        meter::reset();
        accumulate(&mut out, 0, 1, 7);
        assert_eq!(meter::snapshot().collection_elements_copied, 0);
        accumulate(&mut out, 0, 2, 7);
        assert_eq!(meter::snapshot().collection_elements_copied, 1);
        assert_eq!(meter::snapshot().collection_elements_initialized, 1);
    }

    #[test]
    fn forest_initialization_and_slice_passes_are_disjoint() {
        meter::reset();
        let mut graph = SymbolicGraph::new(7, 2, 2).unwrap();
        assert_eq!(meter::snapshot().collection_elements_initialized, 10);
        meter::reset();
        assert_eq!(
            graph.insert(&[(0, 0)], &[(1, 0)]),
            Ok(Outcome::Constraint(Sparse::new()))
        );
        let c = meter::snapshot();
        assert_eq!(c.collection_entry_visits, 4); // two validation + two canonicalization entries
        assert_eq!(c.collection_comparisons, 3); // bounds twice, canonical cardinality once
        assert_eq!(c.collection_elements_initialized, 0);
        meter::reset();
        assert_eq!(
            graph.insert(&[(2, 1)], &[(0, 1)]),
            Err(Error::ColumnOutOfRange)
        );
        assert_eq!(meter::snapshot().collection_entry_visits, 1);
        assert_eq!(meter::snapshot().collection_comparisons, 1);
    }

    #[test]
    fn semantic_counters_separate_primality_and_inverse_work() {
        meter::reset();
        assert_eq!(inverse(3, 7), 5);
        let c = meter::snapshot();
        // Exponent 5 has three loop squares and two selected multiplies.
        assert_eq!(c.scalar_muls, 5);
        assert_eq!(c.scalar_inversions, 1);
        meter::reset();
        assert!(is_prime(101));
        let c = meter::snapshot();
        assert_eq!(c.primality_checks, 1);
        assert_eq!(c.scalar_inversions, 0);
        assert!(c.scalar_muls > 0);
        assert!(c.modular_reductions > 0);
        let mut g = SymbolicGraph::new(7, 3, 1).unwrap();
        meter::reset();
        assert_eq!(
            g.insert(&[(0, 1), (1, 1), (2, 1)], &[]),
            Err(Error::MoreThanTwoLargeVariables)
        );
        let c = meter::snapshot();
        assert_eq!(c.modular_reductions, 3);
        assert_eq!(c.sparse_accumulations, 3);
        assert_eq!(c.scalar_adds, 3);
        assert_eq!(c.scalar_inversions, 0);
    }

    // Independent small-field reference: dense rows, exhaustive inverse
    // search, and ordinary RREF. No graph arithmetic helper is used.
    fn rref(mut a: Vec<Vec<u64>>, p: u64, columns: usize) -> Vec<Vec<u64>> {
        let mut rank = 0;
        for col in 0..columns {
            let Some(i) = (rank..a.len()).find(|&i| a[i][col] != 0) else {
                continue;
            };
            a.swap(rank, i);
            let inverse = (1..p).find(|&x| x * a[rank][col] % p == 1).unwrap();
            for x in &mut a[rank] {
                *x = *x * inverse % p;
            }
            for i in 0..a.len() {
                if i == rank {
                    continue;
                }
                let factor = a[i][col];
                let pivot = a[rank].clone();
                for (value, basis) in a[i].iter_mut().zip(pivot) {
                    *value = (*value + p - factor * basis % p) % p;
                }
            }
            rank += 1;
        }
        a.retain(|r| r.iter().any(|&v| v != 0));
        a
    }

    #[test]
    fn signed_cycle_anchors_before_it_produces_a_constraint() {
        let mut g = SymbolicGraph::new(7, 2, 1).unwrap();
        assert_eq!(g.insert(&[(0, 1), (1, 1)], &[]), Ok(Outcome::Joined));
        // x0+x1=0 and x0-x1+s=0 imply an anchored root, not s=0.
        assert_eq!(
            g.insert(&[(0, 1), (1, 6)], &[(0, 1)]),
            Ok(Outcome::Anchored)
        );
        assert_eq!(
            g.insert(&[(0, 1)], &[]),
            Ok(Outcome::Constraint([(0, 3)].into()))
        );
    }

    #[test]
    fn every_prefix_matches_independent_elimination_space() {
        let mut prefixes = 0;
        for p in [2, 3, 5, 7, 101] {
            for seed in 1..=128u64 {
                let (nl, ns) = (6, 4);
                let mut state = seed;
                let mut next = || {
                    state ^= state << 13;
                    state ^= state >> 7;
                    state ^= state << 17;
                    state
                };
                let mut g = SymbolicGraph::new(p, nl, ns).unwrap();
                let mut composed = collector::Collector::new(
                    p,
                    &[
                        false, false, false, false, false, false, true, true, true, true,
                    ],
                    residual::Limits {
                        retained_coefficients: ns * ns,
                        working_coefficients: ns,
                    },
                )
                .unwrap();
                let mut basis = residual::Basis::new(
                    p,
                    ns,
                    residual::Limits {
                        retained_coefficients: ns * ns,
                        working_coefficients: ns,
                    },
                )
                .unwrap();
                let mut original = Vec::new();
                let mut emitted = Vec::new();
                for _ in 0..40 {
                    let large: Vec<_> = (0..next() % 3)
                        .map(|_| (next() as usize % nl, next() % p))
                        .collect();
                    let small: Vec<_> = (0..next() % 5)
                        .map(|_| (next() as usize % ns, next() % p))
                        .collect();
                    let mut dense = vec![0; nl + ns];
                    for &(i, v) in &large {
                        dense[i] = (dense[i] + v) % p;
                    }
                    for &(i, v) in &small {
                        dense[nl + i] = (dense[nl + i] + v) % p;
                    }
                    original.push(dense);
                    let mut global = large.clone();
                    global.extend(small.iter().map(|&(i, v)| (nl + i, v)));
                    let admission = composed.insert(&global).unwrap();
                    assert!(matches!(
                        admission,
                        collector::Admission::Joined
                            | collector::Admission::Anchored
                            | collector::Admission::Independent
                            | collector::Admission::Dependent
                    ));
                    if let Outcome::Constraint(row) = g.insert(&large, &small).unwrap() {
                        let terms: Vec<_> = row.iter().map(|(&i, &v)| (i, v)).collect();
                        assert!(matches!(
                            basis.insert(&terms).unwrap(),
                            residual::Insert::Independent | residual::Insert::Dependent
                        ));
                        let mut dense = vec![0; ns];
                        for (i, v) in row {
                            dense[i] = v;
                        }
                        emitted.push(dense);
                    }
                    let expected: Vec<_> = rref(original.clone(), p, nl + ns)
                        .into_iter()
                        .filter(|r| r[..nl].iter().all(|&x| x == 0))
                        .map(|r| r[nl..].to_vec())
                        .collect();
                    assert_eq!(
                        rref(emitted.clone(), p, ns),
                        expected,
                        "p={p} seed={seed} prefix={}",
                        original.len()
                    );
                    let retained: Vec<_> = basis
                        .rows()
                        .map(|row| {
                            let mut dense = vec![0; ns];
                            for (&i, &v) in row {
                                dense[i] = v;
                            }
                            dense
                        })
                        .collect();
                    assert_eq!(rref(retained, p, ns), expected);
                    let retained: Vec<_> = composed
                        .basis()
                        .rows()
                        .map(|row| {
                            let mut dense = vec![0; ns];
                            for (&i, &v) in row {
                                dense[i] = v;
                            }
                            dense
                        })
                        .collect();
                    assert_eq!(rref(retained, p, ns), expected);
                    prefixes += 1;
                }
            }
        }
        assert_eq!(prefixes, 25600);
    }

    #[test]
    fn malformed_input_is_atomic_and_duplicates_are_combined() {
        let mut g = SymbolicGraph::new(7, 3, 1).unwrap();
        assert_eq!(
            g.insert(&[(0, 1), (1, 1), (2, 1)], &[]),
            Err(Error::MoreThanTwoLargeVariables)
        );
        assert_eq!(g.insert(&[(3, 1)], &[]), Err(Error::ColumnOutOfRange));
        assert_eq!(g.insert(&[], &[(1, 1)]), Err(Error::ColumnOutOfRange));
        assert_eq!(
            g.insert(&[(0, 1), (0, 6)], &[(0, 2), (0, 5)]),
            Ok(Outcome::Constraint(Sparse::new()))
        );
        assert_eq!(g.insert(&[(0, 1), (1, 1)], &[]), Ok(Outcome::Joined));
    }

    #[test]
    fn nonsaturated_long_components_keep_independent_small_variables() {
        for p in [2, 3, 5, 7, 101, 18446744073709551557] {
            let (nl, ns) = (32, 4);
            let mut g = SymbolicGraph::new(p, nl, ns).unwrap();
            // Prescribe X_i=S_(i mod 4) symbolically, without assigning S.
            // Pair joins then hierarchical joins exercise size-balanced paths.
            for stride in [1, 2, 4, 8, 16] {
                for i in (0..nl).step_by(2 * stride) {
                    let j = i + stride;
                    let a = 1 + (i as u64 % (p - 1));
                    let b = 1 + (j as u64 % (p - 1));
                    assert_eq!(
                        g.insert(&[(i, a), (j, b)], &[(i % ns, p - a), (j % ns, p - b)]),
                        Ok(Outcome::Joined)
                    );
                }
            }
            assert_eq!(g.insert(&[(0, 1)], &[(0, p - 1)]), Ok(Outcome::Anchored));
            for i in 0..nl {
                assert_eq!(
                    g.insert(&[(i, 1)], &[(i % ns, p - 1)]),
                    Ok(Outcome::Constraint(Sparse::new()))
                );
            }
            // Rank of emitted small constraints stays zero until this row,
            // then one, rather than saturating all four small dimensions.
            assert_eq!(
                g.insert(&[(0, 1)], &[(1, 1)]),
                Ok(Outcome::Constraint([(0, 1), (1, 1)].into()))
            );
        }
    }

    #[test]
    fn wide_moduli_and_composites() {
        for n in [0, 1, 4, 9, 341, 561, 3215031751, u64::MAX] {
            assert!(matches!(
                SymbolicGraph::new(n, 1, 1),
                Err(Error::NonPrimeModulus)
            ));
        }
        let p = 18446744073709551557;
        let mut g = SymbolicGraph::new(p, 1, 1).unwrap();
        assert_eq!(
            g.insert(&[(0, p - 1)], &[(0, p - 1)]),
            Ok(Outcome::Anchored)
        );
        assert_eq!(
            g.insert(&[(0, 1)], &[(0, 1)]),
            Ok(Outcome::Constraint(Sparse::new()))
        );
    }
}

#[cfg(feature = "producer")]
pub use crate as kernel;
#[cfg(feature = "producer")]
pub use five_leads_study::three_leads as engine;
#[cfg(feature = "producer")]
#[path = "../../../src/cryptanalysis/ic_measurement.rs"]
// The root-only external Callgrind dump entry is unused by this standalone crate.
#[expect(
    dead_code,
    reason = "shared clock includes a root-only Callgrind entry"
)]
pub mod measurement;
#[cfg(feature = "producer")]
pub mod native_solver;
